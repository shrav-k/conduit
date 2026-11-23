use super::apply::{Action, Binding, Effect};
use super::*;

pub(super) fn plan(
    desired: &Desired,
    ledger: &Ledger,
    engine: &Engine,
    approvals: &BTreeSet<Approval>,
) -> Plan {
    let mut result = Plan {
        stops: desired.stops.clone(),
        names: engine
            .objects
            .iter()
            .map(|(name, object)| (object.id.clone(), name.clone()))
            .collect(),
        ..Plan::default()
    };
    if result.stops.iter().any(|stop| stop.ends_with(" SEC-001")) {
        result.stops.retain(|stop| stop.ends_with(" SEC-001"));
        return result;
    }
    if !result.stops.is_empty() {
        return result;
    }
    if ledger.objects.iter().any(|(name, kept)| {
        engine
            .objects
            .get(name)
            .filter(|old| old.id == kept.id)
            .is_some_and(|old| held_address_changed(desired, old))
    }) {
        result
            .stops
            .insert("stop Manifest/- place=/address rule=none -".into());
    }
    let mut changes = Changes {
        desired,
        ledger,
        engine,
        approvals,
        result,
        touched: BTreeSet::new(),
        add: vec![],
        replace_templates: vec![],
        close: BTreeSet::new(),
        narrow: BTreeSet::new(),
        resume: vec![],
        used: BTreeMap::new(),
        replacement_flags: BTreeMap::new(),
        retire: BTreeSet::new(),
        remakes: BTreeMap::new(),
        lifts: BTreeMap::new(),
        lift_permissions: BTreeMap::new(),
    };
    changes.collect();
    changes.expand_replacements();
    changes.check_tombstones();
    changes.steps();
    changes.unknown();
    changes.check_operands();
    changes.result
}

struct Changes<'a> {
    desired: &'a Desired,
    ledger: &'a Ledger,
    engine: &'a Engine,
    approvals: &'a BTreeSet<Approval>,
    result: Plan,
    touched: BTreeSet<Name>,
    add: Vec<Name>,
    replace_templates: Vec<(Name, Option<String>)>,
    close: BTreeSet<Name>,
    narrow: BTreeSet<Name>,
    resume: Vec<Name>,
    used: BTreeMap<Approval, BTreeSet<Name>>,
    replacement_flags: BTreeMap<Name, BTreeSet<Approval>>,
    retire: BTreeSet<Name>,
    remakes: BTreeMap<Name, Name>,
    lifts: BTreeMap<String, PlannedLift>,
    lift_permissions: BTreeMap<Name, BTreeSet<Approval>>,
}

#[derive(Default)]
struct PlannedLift {
    subject: Option<Name>,
    approvals: BTreeSet<Approval>,
    descendants: BTreeSet<Name>,
}

impl Changes<'_> {
    fn stop(&mut self, name: &Name, cause: &str) {
        self.result.stops.insert(format!("stop {name} {cause}"));
    }

    fn permit(&mut self, name: &Name, cause: &str, approval: &Approval, rule: &str) -> bool {
        self.touched.insert(name.clone());
        if self.approvals.contains(approval) {
            let users = self.used.entry(approval.clone()).or_default();
            if name.name == approval.operand {
                users.insert(name.clone());
            } else if users.is_empty() {
                users.extend(
                    self.desired
                        .objects
                        .keys()
                        .filter(|name| name.name == approval.operand)
                        .cloned(),
                );
            }
            true
        } else {
            self.stop(name, &format!("{cause} requires={approval} {rule}"));
            false
        }
    }

    fn flag(&mut self, name: &Name, cause: &str, flag: &str, rule: &str) -> bool {
        self.permit(name, cause, &Approval::new(flag, &name.name), rule)
    }

    fn permit_all(
        &mut self,
        name: &Name,
        cause: &str,
        approvals: &BTreeSet<Approval>,
        rule: &str,
    ) -> bool {
        let mut allowed = true;
        for approval in approvals {
            allowed &= self.permit(name, cause, approval, rule);
        }
        allowed
    }

    fn check_operands(&mut self) {
        for approval in self.approvals {
            let choice = Name::new("Flag", &approval.flag);
            match self.used.get(approval) {
                None => self.stop(&choice, &format!("operand={} use=none -", approval.operand)),
                Some(names) if names.len() > 1 => self.stop(
                    &choice,
                    &format!(
                        "operand={} use=ambiguous objects=[{}] -",
                        approval.operand,
                        names
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(",")
                    ),
                ),
                _ => {}
            }
        }
    }

    fn remake(&mut self, name: &Name, approval: Approval) {
        self.replacement_flags
            .entry(name.clone())
            .or_default()
            .insert(approval);
        self.remakes.insert(name.clone(), name.clone());
        if name.kind == "Conduit" {
            self.close.insert(name.clone());
            self.narrow.insert(name.clone());
        } else {
            self.retire.insert(name.clone());
        }
    }

    fn collect(&mut self) {
        let protected: BTreeSet<_> = self
            .desired
            .project
            .documents
            .iter()
            .filter_map(Name::document)
            .filter(|name| self.engine.protects(name))
            .collect();
        let names: BTreeSet<_> = self
            .desired
            .objects
            .keys()
            .chain(self.ledger.objects.keys())
            .chain(&protected)
            .cloned()
            .collect();
        // Decide parents before their declared children, even when kind order would
        // put a Conduit before its enclosing Interface.
        let mut names: Vec<_> = names.into_iter().collect();
        names.sort_by_cached_key(|name| nested_slot(self.engine, name).is_some());
        for name in &names {
            if nested_slot(self.engine, name)
                .is_some_and(|(parent, _, _)| self.retire.contains(parent))
                || self
                    .ledger
                    .replacement_slot(self.engine, name)
                    .is_some_and(|slot| self.retire.contains(&slot.parent))
            {
                continue;
            }
            let want = self.desired.objects.get(name);
            let old = self.engine.objects.get(name);
            let kept = self.ledger.objects.get(name);
            let owned = old.zip(kept).is_some_and(|(old, kept)| old.id == kept.id);
            if old.is_some_and(|old| old.made_by_engine) {
                if want.is_some() || protected.contains(name) {
                    let rule = if name.kind == "Conduit" {
                        "ACC-024"
                    } else {
                        "OBJ-002"
                    };
                    self.stop(name, &format!("made-by=engine {rule}"));
                }
                continue;
            }
            if (want.is_some() || owned)
                && old.is_some_and(|old| {
                    old.state == "retired" && old.status["shutdown"] != "confirmed"
                })
            {
                let this_copy = owned && kept.is_some_and(|kept| kept.state == "retired");
                let approvals = self
                    .ledger
                    .pending_replacement(name)
                    .map(|proof| proof.approvals.clone());
                if owned
                    && want.is_some()
                    && let Some(approvals) = approvals
                {
                    if self.permit_all(name, "state=retired", &approvals, "LIF-026") {
                        for flag in approvals {
                            self.remake(name, flag);
                        }
                    }
                    continue;
                }
                let cause = if want.is_none() || this_copy {
                    "state=retired rule=none -"
                } else {
                    "state=retired LIF-024"
                };
                self.stop(name, cause);
                continue;
            }
            if self.covered_by_absent_parent(name, old) {
                continue;
            }
            match (want, old, kept) {
                (Some(want), Some(old), kept) => self.existing(name, want, old, kept),
                (Some(want), None, kept) => self.missing(name, want, kept),
                (None, Some(old), Some(_)) if owned => self.dropped(name, old),
                _ => {}
            }
        }
    }

    fn covered_by_absent_parent(&self, name: &Name, old: Option<&ObservedObject>) -> bool {
        if self.desired.objects.contains_key(name) {
            return false;
        }
        if let Some(base) = name.name.strip_suffix("--t") {
            let base = Name::new(&name.kind, base);
            if self.engine.objects.get(&base).is_some_and(|o| {
                !o.is_template()
                    && self
                        .ledger
                        .objects
                        .get(&base)
                        .is_some_and(|kept| kept.id == o.id)
            }) && !self.desired.objects.contains_key(&base)
            {
                return true;
            }
        }
        let owner_kind = match name.kind.as_str() {
            "Resource" => "Pool",
            "Agent" => "Fleet",
            _ => return false,
        };
        let owner = name_reference(&old.map(|o| &o.definition["owner"]), owner_kind);
        owner.is_some_and(|owner| {
            self.ledger
                .objects
                .get(&owner)
                .zip(self.engine.objects.get(&owner))
                .is_some_and(|(kept, current)| kept.id == current.id)
                && !self.desired.objects.contains_key(&owner)
        })
    }

    fn existing(
        &mut self,
        name: &Name,
        want: &DesiredObject,
        old: &ObservedObject,
        kept: Option<&ObservedObject>,
    ) {
        let template = old.is_template();
        if !template
            && old.state == "retired"
            && let Some(stone) = self
                .engine
                .tombstones
                .iter()
                .find(|s| s["object"]["id"] == old.id)
        {
            self.flag(
                name,
                &format!("tombstone={}", text(&stone["id"])),
                if name.kind == "Conduit" {
                    "--reopen"
                } else {
                    "--recreate"
                },
                "LIF-041",
            );
            return;
        }
        let Some(kept) =
            kept.filter(|k| k.id == old.id && (!template || k.revision == old.revision))
        else {
            let show_revision = if template && kept.is_some() {
                format!(" revision={}", old.revision)
            } else {
                String::new()
            };
            self.flag(
                name,
                &format!("id={}{}", old.id, show_revision),
                "--adopt",
                "OBJ-002",
            );
            return;
        };
        let previous = comparable(&old.definition);
        let next = comparable(&want.resolved.definition);
        let redacted = redacted_selection(want, old);
        let contracts_changed = changed_documents(want, old);
        let changed_pins = changed_release_pins(want, old);
        let different_item = !same_items(want, old, self.desired);
        if template {
            if previous != next
                || contracts_changed
                || different_item
                || !changed_pins.is_empty()
                || redacted.is_some()
            {
                self.touched.insert(name.clone());
                self.replace_templates.push((name.clone(), redacted));
            }
            return;
        }
        let mut places = vec![];
        differences(&previous, &next, "", &mut places);
        let changed_execution = contracts_changed
            || different_item
            || redacted.is_some()
            || !changed_pins.is_empty() && !same_contracts(want, old, self.desired);
        if places.is_empty() && !changed_execution {
            if old.state == "registered" {
                if let Some(flags) = self.ledger.registration_flags.get(&kept.id) {
                    if !self.permit_all(name, "state=registered", flags, "LIF-013") {
                        return;
                    }
                    self.replacement_flags.insert(name.clone(), flags.clone());
                }
                self.touched.insert(name.clone());
                self.resume.push(name.clone());
            } else if old.state != "ready" {
                self.stop(name, &format!("state={} rule=none -", old.state));
            }
            return;
        }
        if nested_slot(self.engine, name).is_some()
            && (previous["from"] != next["from"]
                || previous["resolved"] != next["resolved"]
                || changed_execution)
        {
            if name.kind == "Conduit" && is_subset(&previous, &next) && !changed_execution {
                self.close.insert(name.clone());
                self.narrow.insert(name.clone());
                self.touched.insert(name.clone());
            } else if self.flag(name, "place=/from", "--replace", "LIF-009") {
                if previous["from"] != next["from"] {
                    // The core uses the parent's held slot, including its original reference.
                    self.stop(name, "place=/from LIF-009");
                } else {
                    self.remake(name, Approval::new("--replace", &name.name));
                }
            }
            return;
        }
        if name.kind == "Conduit" {
            if is_subset(&previous, &next) && !changed_execution {
                self.close.insert(name.clone());
                self.narrow.insert(name.clone());
                self.touched.insert(name.clone());
            } else {
                if places.is_empty() {
                    places.push("/resolved/operations".into());
                }
                for place in places {
                    if self.flag(name, &format!("place={place}"), "--replace", "OBJ-011") {
                        self.remake(name, Approval::new("--replace", &name.name));
                    }
                }
            }
            return;
        }
        let kept_definition = comparable(&kept.definition);
        let replace = changed_execution
            || places.iter().any(|p| {
                let key = p.split('/').nth(1).unwrap_or("");
                !matches!(key, "placement" | "runtime" | "apiVersion")
                    && (setting_pointer(p).is_none()
                        || kept_definition.pointer(p) == previous.pointer(p))
            });
        let owned = owns_members(self.engine, name);
        for place in places {
            if place.starts_with("/placement")
                || place.starts_with("/runtime")
                || place == "/apiVersion"
            {
                self.stop(name, &format!("place={place} rule=none -"));
            } else if let Some(setting) = setting_pointer(&place)
                && kept_definition.pointer(&place) != previous.pointer(&place)
            {
                let cause = format!("setting={setting} changed-by=control REV-007");
                if replace || kept_definition.pointer(&place) != next.pointer(&place) {
                    self.stop(name, &cause);
                } else {
                    self.result
                        .differences
                        .push(format!("difference {name} {cause}"));
                }
            } else if owned {
                self.stop(name, &format!("place={place} LIF-020"));
            } else if let Some(setting) = setting_pointer(&place) {
                let control = if old.state == "ready" {
                    want.controls
                        .get(setting)
                        .and_then(BTreeSet::first)
                        .map(|op| format!(" control={op}"))
                        .unwrap_or_default()
                } else {
                    String::new()
                };
                self.flag(
                    name,
                    &format!("setting={setting}{control}"),
                    "--replace",
                    "OBJ-011",
                );
            } else {
                self.flag(name, &format!("place={place}"), "--replace", "OBJ-011");
            }
        }
        if changed_execution && owned {
            if let Some(item) = redacted {
                self.stop(name, &format!("item={item} secret=not-shown LIF-020"));
            }
            if changed_pins.is_empty() {
                self.stop(name, "place=/resolved LIF-020");
            } else {
                for pin in changed_pins {
                    self.stop(name, &format!("pin={} LIF-020", pin.label()));
                }
            }
        } else if let Some(item) = redacted {
            self.flag(
                name,
                &format!("item={item} secret=not-shown"),
                "--replace",
                "SEC-021",
            );
        } else if changed_execution {
            if changed_pins.is_empty() {
                self.flag(name, "place=/resolved", "--replace", "OBJ-011");
            } else {
                for pin in changed_pins {
                    self.flag(
                        name,
                        &format!("pin={}", pin.label()),
                        "--replace",
                        "OBJ-011",
                    );
                }
            }
        }
        if replace && !owned {
            let approval = Approval::new("--replace", &name.name);
            if self
                .used
                .get(&approval)
                .is_some_and(|names| names.contains(name))
            {
                self.remake(name, approval);
            }
            for (other, conduit) in &self.engine.objects {
                if other.kind == "Conduit"
                    && !conduit.made_by_engine
                    && [
                        &conduit.definition["consumer"],
                        &conduit.definition["target"],
                    ]
                    .contains(&&name.reference())
                {
                    self.touched.insert(other.clone());
                    if !self.desired.objects.contains_key(other) {
                        let endpoint = if conduit.definition["consumer"] == name.reference() {
                            "consumer"
                        } else {
                            "target"
                        };
                        self.stop(other, &format!("absent=file place=/{endpoint} CON-008"));
                    }
                }
            }
        }
    }

    fn missing(&mut self, name: &Name, want: &DesiredObject, kept: Option<&ObservedObject>) {
        let (flag, kind) = if name.kind == "Conduit" {
            ("--reopen", "Conduit")
        } else {
            ("--recreate", &*name.kind)
        };
        if !want.resolved.is_template() {
            let stones_match = self
                .engine
                .tombstones
                .iter()
                .any(|stone| matches_tombstone(name, &want.resolved.definition, stone));
            if let Some(proof) = self.ledger.pending_replacement(name) {
                if self.permit_all(name, "absent=engine", &proof.approvals, "LIF-040") {
                    self.replacement_flags
                        .insert(name.clone(), proof.step_approvals.clone());
                    self.add.push(name.clone());
                }
                return;
            }
            if stones_match {
                self.add.push(name.clone());
                return;
            }
            if kind == "Conduit"
                && let Some((prior, _)) = self.engine.objects.iter().find(|(prior, old)| {
                    prior.kind == "Conduit"
                        && self
                            .ledger
                            .objects
                            .get(*prior)
                            .is_some_and(|kept| kept.id == old.id)
                        && !self.desired.objects.contains_key(*prior)
                        && same_match(&old.definition, &want.resolved.definition)
                })
            {
                if self.flag(name, &format!("tombstone=({prior})"), flag, "LIF-041") {
                    let approval = Approval::new(flag, &name.name);
                    self.remakes.insert(name.clone(), prior.clone());
                    self.replacement_flags
                        .entry(name.clone())
                        .or_default()
                        .insert(approval.clone());
                    self.lift_permissions
                        .entry(prior.clone())
                        .or_default()
                        .insert(approval);
                }
                return;
            }
        }
        if kept.is_some() {
            if self.flag(name, "absent=engine", flag, "LIF-040") {
                self.replacement_flags
                    .entry(name.clone())
                    .or_default()
                    .insert(Approval::new(flag, &name.name));
                self.add.push(name.clone());
            }
        } else {
            self.add.push(name.clone());
            self.touched.insert(name.clone());
        }
    }

    fn check_tombstones(&mut self) {
        let additions: BTreeSet<_> = self
            .add
            .iter()
            .chain(self.remakes.keys())
            .cloned()
            .collect();
        let implicit: BTreeMap<_, _> = self
            .desired
            .nested
            .iter()
            .filter(|child| self.added_with_parent(&child.parent, &additions))
            .map(|child| (child.name.clone(), &child.resolved.definition))
            .collect();
        let checks: BTreeMap<_, _> = additions
            .iter()
            .chain(self.resume.iter().filter(|name| name.kind == "Conduit"))
            .filter_map(|name| {
                let object = &self.desired.objects[name].resolved;
                (!object.is_template()).then_some((name.clone(), &object.definition))
            })
            .chain(implicit)
            .collect();
        let mut blocked = BTreeSet::new();
        for (name, definition) in checks {
            for stone in self
                .engine
                .tombstones
                .iter()
                .filter(|stone| matches_tombstone(&name, definition, stone))
            {
                let cause = format!("tombstone={}", text(&stone["id"]));
                let root = tombstone_root(&self.engine.tombstones, stone);
                let proof = self
                    .ledger
                    .lift_proof(text(&root["cause"]["invocation"]))
                    .filter(|proof| proof.remade.contains(&name));
                if let Some(proof) = proof {
                    if self.permit_all(&name, &cause, &proof.approvals, "LIF-041") {
                        self.replacement_flags
                            .entry(name.clone())
                            .or_default()
                            .extend(proof.step_approvals.iter().cloned());
                        // A pending retirement of the same object will bind this stone
                        // to its earlier, journaled cause when listing after retry.
                        if self.remakes.get(&name).is_none_or(|removal| {
                            self.engine.objects[removal].id != root["object"]["id"]
                        }) {
                            self.schedule_lift(stone, None);
                        }
                    } else {
                        blocked.insert(name.clone());
                    }
                } else {
                    let flag = if name.kind == "Conduit" {
                        "--reopen"
                    } else {
                        "--recreate"
                    };
                    if self.flag(&name, &cause, flag, "LIF-041") {
                        let approval = Approval::new(flag, &name.name);
                        self.schedule_lift(stone, Some(approval.clone()));
                        self.replacement_flags
                            .entry(name.clone())
                            .or_default()
                            .insert(approval);
                    } else {
                        blocked.insert(name.clone());
                    }
                }
            }
        }
        self.add.retain(|name| !blocked.contains(name));
        self.resume.retain(|name| !blocked.contains(name));
    }

    fn added_with_parent(&self, name: &Name, additions: &BTreeSet<Name>) -> bool {
        additions.contains(name)
            || self
                .desired
                .nested
                .iter()
                .find(|child| &child.name == name)
                .is_some_and(|child| self.added_with_parent(&child.parent, additions))
    }

    fn schedule_lift(&mut self, stone: &Value, approval: Option<Approval>) {
        let lift = self.lifts.entry(text(&stone["id"]).to_owned()).or_default();
        lift.approvals.extend(approval);
    }

    fn combine_lifts(&mut self) -> Vec<(String, PlannedLift)> {
        let selected: BTreeSet<_> = self.lifts.keys().cloned().collect();
        let stones = &self.engine.tombstones;
        let mut groups = BTreeMap::<_, PlannedLift>::new();
        for (id, lift) in std::mem::take(&mut self.lifts) {
            let stone = stones
                .iter()
                .find(|stone| stone["id"] == id)
                .expect("a planned tombstone is in the inspection");
            // Only merge into an ancestor whose own lift was planned.
            let root = tombstone_chain(stones, stone)
                .into_iter()
                .rfind(|parent| selected.contains(text(&parent["id"])))
                .unwrap_or(stone);
            let group = groups.entry(text(&root["id"]).to_owned()).or_default();
            group.subject = Name::object(&root["object"]);
            group.approvals.extend(lift.approvals);
        }
        let mut groups: Vec<_> = groups.into_iter().collect();
        for (id, group) in &mut groups {
            group.descendants.extend(
                stones
                    .iter()
                    .filter(|stone| {
                        tombstone_chain(stones, stone)
                            .iter()
                            .skip(1)
                            .any(|parent| parent["id"] == *id)
                    })
                    .filter_map(|stone| Name::object(&stone["object"])),
            );
        }
        groups.sort_by(|(_, a), (_, b)| a.subject.cmp(&b.subject));
        groups
    }

    fn expand_replacements(&mut self) {
        for name in self.narrow.clone() {
            self.remakes.entry(name.clone()).or_insert(name);
        }
        for endpoint in self.retire.clone() {
            let Some(wanted) = self.desired.objects.get(&endpoint) else {
                continue;
            };
            let old = &self.engine.objects[&endpoint];
            self.mark_bounds(&endpoint, old, &wanted.resolved);
            if endpoint.kind == "Agent" {
                match self.held_sign_in(old) {
                    Some(true) => {
                        self.result.signed_out.insert(endpoint.clone());
                    }
                    Some(false) => {}
                    None => self.stop(&endpoint, "place=/type sign-in=not-shown CTR-013"),
                }
            }
            let approvals = self.replacement_flags[&endpoint].clone();
            for (name, conduit) in &self.engine.objects {
                if name.kind != "Conduit"
                    || !["consumer", "target"]
                        .iter()
                        .any(|field| conduit.definition[*field] == endpoint.reference())
                {
                    continue;
                }
                self.result.closed_conduits.insert(name.name.clone());
                self.close.remove(name);
                self.narrow.remove(name);
                self.touched.insert(name.clone());
                if self.desired.objects.contains_key(name) {
                    if let Some((parent, _, _)) = nested_slot(self.engine, name) {
                        if self.retire.contains(parent) {
                            self.remakes.remove(name);
                            self.add.retain(|child| child != name);
                            self.resume.retain(|child| child != name);
                            continue;
                        }
                        // The surviving parent froze the target reference to the old ID.
                        // register-replacement.v1 would use that same held reference.
                        if conduit.definition["target"] == endpoint.reference() {
                            self.stop(name, "place=/target LIF-009");
                            self.remakes.remove(name);
                            continue;
                        }
                    }
                    self.remakes.insert(name.clone(), endpoint.clone());
                    self.replacement_flags
                        .entry(name.clone())
                        .or_default()
                        .extend(approvals.clone());
                }
            }
        }
    }

    fn incident(&self, endpoint: &Name) -> BTreeSet<Name> {
        if endpoint.kind == "Conduit" {
            return BTreeSet::new();
        }
        self.engine
            .objects
            .iter()
            .filter(|(name, conduit)| {
                name.kind == "Conduit"
                    && !conduit.is_template()
                    && ["consumer", "target"]
                        .iter()
                        .any(|field| conduit.definition[*field] == endpoint.reference())
            })
            .map(|(name, _)| name.clone())
            .collect()
    }

    fn held_sign_in(&self, old: &ObservedObject) -> Option<bool> {
        let ty = text(&old.definition["resolved"]["type"]);
        if let Some((address, name)) = ty.split_once('#') {
            let is_type = |doc: &&Value| doc["kind"] == "Type" && doc["metadata"]["name"] == name;
            for package in &old.packages {
                if package["address"] != address {
                    continue;
                }
                let archived = Pin::from_value(package)
                    .and_then(|pin| self.desired.available_archives.get(&pin))
                    .and_then(|archive| match archive {
                        Archive::Read(archive) => {
                            archive.package.project.documents.iter().find(is_type)
                        }
                        _ => None,
                    });
                let current = || {
                    let held = array(&package["documents"])
                        .iter()
                        .find(|doc| doc["kind"] == "Type" && doc["name"] == name)?;
                    let candidate = self.desired.project.documents.iter().find(is_type)?;
                    (conduit_core::packages::document_digest(candidate)
                        .ok()
                        .as_deref()
                        == Some(held["digest"].as_str()?))
                    .then_some(candidate)
                };
                if let Some(document) = archived.or_else(current) {
                    return Some(document["spec"].get("signIn").is_some());
                }
            }
        }
        if array(&old.status["pending"])
            .iter()
            .any(|action| action["kind"] == "sign-in")
            || array(&old.status["conditions"])
                .iter()
                .any(|condition| condition["type"] == "SignInUndone")
        {
            return Some(true);
        }
        // A Type with signIn must require all three hooks (CTR-013).
        (!["account-hook.v1", "sign-in-hook.v1", "sign-out-hook.v1"]
            .iter()
            .all(|hook| {
                old.implementations
                    .get(format!("{}#{hook}", self.desired.core_address))
                    .is_some()
            }))
        .then_some(false)
    }

    fn mark_bounds(&mut self, name: &Name, old: &ObservedObject, new: &ObservedObject) {
        let previous = &old.definition["owner"];
        let next = &new.definition["owner"];
        for (holder, conduit) in &self.engine.objects {
            if holder.kind != "Conduit" || conduit.state != "ready" {
                continue;
            }
            for requirement in conduit.definition["resolved"]["requirements"]
                .as_object()
                .into_iter()
                .flat_map(|r| r.values())
            {
                if requirement["requirement"] != format!("{}#bound.v1", self.desired.core_address) {
                    continue;
                }
                let args = &requirement["arguments"];
                let target = array(&args["targets"])
                    .iter()
                    .any(|r| r == &name.reference() || r["id"] == old.id);
                let pool = |owner: &Value| {
                    name.kind == "Resource"
                        && name_reference(&Some(owner), "Pool").is_some_and(|pool| {
                            array(&args["pools"]).iter().any(|r| {
                                r == &pool.reference()
                                    || self
                                        .engine
                                        .objects
                                        .get(&pool)
                                        .is_some_and(|held| r["id"] == held.id)
                            })
                        })
                };
                let covered_before = target
                    || pool(previous)
                    || name.kind == "Pool"
                        && array(&args["pools"])
                            .iter()
                            .any(|p| p == &name.reference() || p["id"] == old.id);
                let covered_after = pool(next);
                match (covered_before, covered_after) {
                    (false, true) => {
                        self.result.bounds_added.insert(format!("{holder}:{name}"));
                    }
                    (true, false) => {
                        self.result
                            .affected_bounds
                            .insert(format!("{holder}:{name}"));
                    }
                    _ => {}
                }
            }
        }
    }

    fn dropped(&mut self, name: &Name, old: &ObservedObject) {
        if old.is_template() {
            self.flag(name, "absent=file", "--prune", "INS-020");
        } else if name.kind == "Conduit" {
            self.close.insert(name.clone());
            self.touched.insert(name.clone());
        } else if owns_members(self.engine, name) {
            self.flag(
                name,
                "absent=file",
                if name.kind == "Pool" {
                    "--with-resources"
                } else {
                    "--with-agents"
                },
                "LIF-020",
            );
            for (member, old) in &self.engine.objects {
                if name_reference(&old.definition.get("owner"), &name.kind).as_ref() == Some(name)
                    && member.kind != name.kind
                    && !old.is_template()
                {
                    self.touched.insert(member.clone());
                }
            }
        } else {
            self.flag(name, "absent=file", "--prune", "LIF-035");
        }
    }

    fn steps(&mut self) {
        let mut nested_activations = Vec::new();
        for (name, redacted) in std::mem::take(&mut self.replace_templates) {
            let desired = &self.desired.objects[&name];
            let observed = &self.engine.objects[&name];
            let mut details = vec![format!("document={}", safe(&desired.document))];
            let approval = name
                .name
                .strip_suffix("--t")
                .and_then(|inline| self.replacement_flags.get(&Name::new(&name.kind, inline)))
                .cloned()
                .unwrap_or_default();
            let rule = if approval.is_empty() {
                "SUB-009"
            } else {
                "INS-020"
            };
            if !approval.is_empty() {
                let typed_address = text(&desired.resolved.definition["type"])
                    .split_once('#')
                    .map(|(a, _)| a);
                if let Some(pin) = desired
                    .resolved
                    .pins()
                    .find(|pin| Some(pin.address.as_str()) == typed_address)
                    && let Some((alias, version)) = short_pin(&pin)
                    && let Some(digest) = &pin.digest
                {
                    let root = safe(&desired.resolved.definition["config"]["root"]);
                    let root = root["pathSha256"]
                        .as_str()
                        .map(|hex| format!(" root-sha256={hex}"))
                        .unwrap_or_default();
                    details.push(format!(
                        "{alias}={version} digest={digest}{root} revision={}",
                        observed.revision
                    ));
                }
            } else if let Some(manifest) = self.desired.project.manifest.as_ref() {
                details.push(format!("manifest={}", safe(manifest)));
            }
            if let Some(item) = redacted {
                details.push(format!("item={item} secret=not-shown SEC-021"));
            }
            let mut arguments = json!({"template":desired.document,"revision":observed.revision,
                "manifest":self.desired.project.manifest});
            let contracts: Vec<_> = self
                .desired
                .project
                .documents
                .iter()
                .filter(|document| conduit_core::packages::contract(document))
                .collect();
            if !contracts.is_empty() {
                arguments["contracts"] = json!(contracts);
            }
            let step = Step {
                tool: "management.replace.v1".into(),
                subject: name.clone(),
                expected: Some(observed.id.clone()),
                relative: Expected::None,
                class: "replaces",
                rule,
                arguments,
                details,
                action: Action {
                    effect: Effect::Replace(Box::new(desired.resolved.clone())),
                    approvals: approval,
                    ..Action::default()
                },
            };
            self.result.steps.push(step);
        }
        if !self.close.is_empty() || !self.retire.is_empty() {
            let templates: Vec<_> = self
                .add
                .iter()
                .filter(|n| self.desired.objects[*n].resolved.is_template())
                .cloned()
                .collect();
            if !templates.is_empty() {
                self.registration(&templates, "SCT-003");
                self.add.retain(|n| !templates.contains(n));
            }
        }
        let removals: Vec<_> = self.close.union(&self.retire).cloned().collect();
        let mut remove_steps = BTreeMap::new();
        for name in &removals {
            let previous = &self.engine.objects[name];
            let retire = self.retire.contains(name);
            let (tool, rule, field) = if retire {
                let route = self.owner_route(&previous.definition, &name.kind);
                (format!("{route}.retire.v1"), "LIF-035", "instance")
            } else {
                self.result.closed_conduits.insert(name.name.clone());
                (format!("{}.close.v1", self.access()), "LIF-037", "conduit")
            };
            let remakes: BTreeSet<_> = self
                .remakes
                .iter()
                .filter(|(_, removal)| *removal == name)
                .map(|(remake, _)| remake.clone())
                .collect();
            let replacement_slots = remakes
                .iter()
                .filter_map(|child| {
                    let (parent, field, slot) = nested_slot(self.engine, child)?;
                    Some((
                        child.clone(),
                        ReplacementSlot {
                            parent: parent.clone(),
                            parent_id: self.engine.objects[parent].id.clone(),
                            field,
                            member: slot.into(),
                            child_id: self.engine.objects[child].id.clone(),
                        },
                    ))
                })
                .collect();
            let closes = if retire {
                self.incident(name)
            } else {
                BTreeSet::new()
            };
            let approvals = self
                .replacement_flags
                .get(name)
                .cloned()
                .unwrap_or_default();
            remove_steps.insert(name.clone(), self.result.steps.len() + 1);
            let effect = if retire {
                let (closes_on_dispatch, closes_on_removal) =
                    closes.into_iter().partition(|child| {
                        self.engine.objects[child].definition["target"] == name.reference()
                    });
                Effect::Retire {
                    closes_on_dispatch,
                    closes_on_removal,
                }
            } else {
                Effect::Remove
            };
            let step = self.push(
                &tool,
                name.clone(),
                Some(previous.id.clone()),
                Expected::None,
                "narrows",
                rule,
            );
            step.arguments = json!({field:name.reference(),"expectedId":previous.id});
            step.action = Action {
                effect,
                remakes,
                replacement_slots,
                approvals,
                ..Action::default()
            };
        }
        if !remove_steps.is_empty() {
            let listed = self.result.steps.len() + 1;
            self.push(
                "management.list.v1",
                Name::new("Tombstone", "*"),
                None,
                Expected::None,
                "neither",
                "LIF-040",
            )
            .action
            .effect = Effect::Tombstones;
            let causes: BTreeSet<_> = self.remakes.values().cloned().collect();
            for name in causes {
                let previous = &self.engine.objects[&name];
                if previous.state == "retired"
                    && self.engine.tombstones.iter().any(|stone| {
                        stone["object"]["id"] == previous.id
                            && self.lifts.contains_key(text(&stone["id"]))
                    })
                {
                    // An explicitly approved outside stone is lifted by ID below.
                    continue;
                }
                let nested = nested_slot(self.engine, &name);
                let approvals = self
                    .lift_permissions
                    .get(&name)
                    .cloned()
                    .or_else(|| nested.and_then(|_| self.replacement_flags.get(&name).cloned()))
                    .unwrap_or_default();
                let mut closed = self.incident(&name);
                closed.retain(|child| !self.engine.objects[child].made_by_engine);
                let binding = Binding::Tombstone {
                    closed: remove_steps[&name],
                    listed,
                    object: previous.id.clone(),
                    prior_cause: self
                        .engine
                        .tombstones
                        .iter()
                        .filter(|stone| stone["object"]["id"] == previous.id)
                        .filter_map(|stone| stone["cause"]["invocation"].as_str())
                        .find(|cause| self.ledger.lift_proof(cause).is_some())
                        .map(str::to_owned),
                };
                let access = self.access().to_owned();
                let step = self.push(
                    &format!("{access}.lift.v1"),
                    Name::new("Tombstone", &name.name),
                    None,
                    Expected::Step(listed),
                    "neither",
                    "ACC-053",
                );
                step.action = Action {
                    binding: Some(binding),
                    effect: Effect::Lift,
                    approvals,
                    ..Action::default()
                };
                if !closed.is_empty() {
                    step.details.push(lifts_caused(&closed));
                }
            }
        }
        for (id, lift) in self.combine_lifts() {
            let name = lift.subject.expect("a planned tombstone has an object");
            let access = self.access().to_owned();
            let step = self.push(
                &format!("{access}.lift.v1"),
                Name::new("Tombstone", &name.name),
                Some(id.clone()),
                Expected::None,
                "neither",
                "ACC-053",
            );
            step.arguments = json!({"id":id});
            step.action = Action {
                effect: Effect::Lift,
                approvals: lift.approvals,
                ..Action::default()
            };
            if !lift.descendants.is_empty() {
                step.details.push(lifts_caused(&lift.descendants));
            }
        }
        for (name, _) in std::mem::take(&mut self.remakes) {
            if let Some((parent, field, slot)) = nested_slot(self.engine, &name) {
                nested_activations.push((name, parent.clone(), field, slot.to_owned()));
            } else {
                self.add.push(name);
            }
        }
        self.add.retain(|child| {
            if let Some(slot) = self.ledger.replacement_slot(self.engine, child) {
                nested_activations.push((
                    child.clone(),
                    slot.parent.clone(),
                    slot.field,
                    slot.member.clone(),
                ));
                false
            } else {
                true
            }
        });
        let nested_activations: Vec<_> = nested_activations
            .into_iter()
            .map(|(child, parent, field, slot)| {
                let registration = self.result.steps.len() + 1;
                let id = self.engine.objects[&parent].id.clone();
                self.result.steps.push(Step {
                    tool: "management.register-replacement.v1".into(),
                    subject: parent.clone(),
                    expected: Some(id.clone()),
                    relative: Expected::None,
                    class: "neither",
                    rule: "LIF-009",
                    arguments: json!({"instance":{"kind":parent.kind,"id":id}, "nested":slot}),
                    details: vec![format!("nested={slot}")],
                    action: Action {
                        effect: Effect::RegisterReplacement {
                            wanted: Box::new(self.desired.objects[&child].resolved.clone()),
                            parent,
                            field,
                            slot,
                            previous: child.clone(),
                        },
                        approvals: self
                            .replacement_flags
                            .get(&child)
                            .cloned()
                            .unwrap_or_default(),
                        ..Action::default()
                    },
                });
                (child, registration)
            })
            .collect();
        let mut pending: Vec<_> = nested_activations
            .into_iter()
            .map(|(name, register)| (name, Expected::Step(register), true))
            .collect();
        let adds = std::mem::take(&mut self.add);
        if !adds.is_empty() {
            let register = self.result.steps.len() + 1;
            let rule = if adds.iter().all(|n| {
                n.kind == "Conduit"
                    && (self.narrow.contains(n) || self.replacement_flags.contains_key(n))
            }) {
                "SCT-007"
            } else if adds.iter().any(|n| self.retire.contains(n)) {
                "SCT-004"
            } else {
                "SCT-003"
            };
            self.registration(&adds, rule);
            pending.extend(
                adds.into_iter()
                    .filter(|n| !self.desired.objects[n].resolved.is_template())
                    .map(|name| (name, Expected::Step(register), false)),
            );
        }
        pending.extend(
            std::mem::take(&mut self.resume)
                .into_iter()
                .map(|name| (name, Expected::None, false)),
        );
        self.activate(pending);
        self.retained_differences();
    }

    fn activate(&mut self, mut pending: Vec<(Name, Expected, bool)>) {
        pending.sort_by(|(a, _, _), (b, _, _)| {
            let order = ["Pool", "Resource", "Fleet", "Agent", "Interface", "Conduit"];
            let approved = |name: &Name| {
                name.kind == "Conduit"
                    && self
                        .replacement_flags
                        .get(name)
                        .is_some_and(|flags| !flags.is_empty())
            };
            (rank(&a.kind, &order), approved(a), &a.name).cmp(&(
                rank(&b.kind, &order),
                approved(b),
                &b.name,
            ))
        });
        let mut awaiting: BTreeSet<_> = pending.iter().map(|(name, _, _)| name.clone()).collect();
        while !pending.is_empty() {
            let next = pending.iter().position(|(name, _, _)| {
                let definition = &self.desired.objects[name].resolved.definition;
                activation_dependencies(name, definition)
                    .all(|required| !awaiting.contains(&required))
            });
            let Some(next) = next else {
                for (name, _, _) in pending {
                    self.stop(&name, "place=/runtime rule=none -");
                }
                break;
            };
            let (name, relative, nested) = pending.remove(next);
            awaiting.remove(&name);

            let want = &self.desired.objects[&name];
            let route = self.owner_route(&want.resolved.definition, &name.kind);
            let subject = if nested && let Expected::Step(register) = relative {
                Name::new(&name.kind, &format!("(step {register}).object"))
            } else {
                name.clone()
            };
            let expected =
                (relative == Expected::None).then(|| self.engine.objects[&name].id.clone());
            let check = want.resolved.clone();
            self.push(
                &format!("{route}.inspect.v1"),
                subject.clone(),
                expected.clone(),
                relative,
                "neither",
                "LIF-013",
            )
            .action = Action {
                binding: object_binding(relative, "object"),
                check: Some(check),
                effect: Effect::None,
                ..Action::default()
            };
            if name.kind == "Conduit" {
                self.open_conduit(&name, subject, expected, relative);
            } else {
                let mut approvals = self
                    .replacement_flags
                    .get(&name)
                    .cloned()
                    .unwrap_or_default();
                if !nested && relative != Expected::None && self.retire.contains(&name) {
                    approvals.clear();
                }
                let class = if want.resolved.definition["resolved"]["conduits"]
                    .as_object()
                    .is_some_and(|m| !m.is_empty())
                {
                    "widens"
                } else {
                    "neither"
                };
                self.push(
                    &format!("{route}.set-up.v1"),
                    subject,
                    expected,
                    relative,
                    class,
                    "TPL-012",
                )
                .action = Action {
                    binding: object_binding(relative, "instance"),
                    effect: Effect::SetLifecycle {
                        state: "ready",
                        opened_by: None,
                    },
                    panel: true,
                    approvals,
                    ..Action::default()
                };
            }
        }
    }

    fn open_conduit(
        &mut self,
        name: &Name,
        subject: Name,
        expected: Option<String>,
        relative: Expected,
    ) {
        let want = &self.desired.objects[name];
        let approvals = self
            .replacement_flags
            .get(name)
            .cloned()
            .unwrap_or_default();
        let provenance = self
            .engine
            .objects
            .get(&Name::new("Interface", "root"))
            .zip(self.engine.access_conduit.as_ref())
            .map(|(root, conduit)| (root.id.clone(), conduit.clone()));
        self.push(
            &format!("{}.open.v1", self.access()),
            subject,
            expected,
            relative,
            "widens",
            "LIF-037",
        )
        .action = Action {
            binding: object_binding(relative, "conduit"),
            effect: Effect::SetLifecycle {
                state: "ready",
                opened_by: provenance,
            },
            approvals,
            opens: Some((name.clone(), want.resolved.definition.clone())),
            ..Action::default()
        };
        let detail = self.result.steps.last_mut().unwrap();
        detail.details.push(format!(
            "operations=[{}]",
            array(&want.resolved.definition["resolved"]["operations"])
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ));
        if want.model_work
            && let Some(target) = Name::object(&want.resolved.definition["target"])
            && target.kind == "Agent"
            && let Some(own_tools) = self
                .desired
                .objects
                .get(&target)
                .and_then(|target| target.own_tools.as_ref())
                .or_else(|| self.engine.objects.get(&target)?.inventory.get("ownTools"))
        {
            detail.details.push(format!(
                "targetOwnTools {target} {} CTR-014",
                safe(own_tools)
            ));
        }
    }

    fn registration(&mut self, names: &[Name], rule: &'static str) {
        let mut docs: Vec<_> = names
            .iter()
            .map(|n| &self.desired.objects[n].document)
            .collect();
        order_registration_documents(&mut docs);
        let names: Vec<_> = docs.iter().filter_map(|d| Name::document(d)).collect();
        let effect = self.registration_effect(&names);
        let approvals = names
            .iter()
            .filter_map(|name| self.replacement_flags.get(name))
            .flatten()
            .cloned()
            .collect();
        let primary = names
            .iter()
            .find(|n| n.kind == "Fleet" && !self.desired.objects[*n].resolved.is_template())
            .or_else(|| {
                names
                    .iter()
                    .find(|n| !self.desired.objects[*n].resolved.is_template())
            })
            .unwrap_or(&names[0])
            .clone();
        self.push(
            "management.register.v1",
            primary,
            None,
            Expected::Step(self.result.steps.len() + 1),
            "adds",
            rule,
        )
        .action = Action {
            effect,
            approvals,
            ..Action::default()
        };
        let step = self.result.steps.last_mut().unwrap();
        let manifest = &self.desired.project.manifest;
        let contracts = self
            .desired
            .project
            .documents
            .iter()
            .filter(|d| conduit_core::packages::contract(d));
        step.arguments = json!({"documents":contracts.clone().chain(docs.iter().copied()).collect::<Vec<_>>(),"manifest":manifest});
        for d in contracts.chain(docs.iter().copied()) {
            step.details.push(format!("document={}", safe(d)));
        }
        if let Some(manifest) = manifest {
            step.details.push(format!("manifest={}", safe(manifest)));
        }
        if names.len() > 1 || rule != "SCT-007" {
            step.details.push(format!(
                "objects=[{}]",
                names
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }
        let defaults: Vec<_> = names
            .iter()
            .filter(|n| {
                matches!(n.kind.as_str(), "Pool" | "Fleet")
                    && !self.desired.objects[*n].resolved.is_template()
            })
            .map(|n| format!("root->{n} as={}", n.name))
            .collect();
        if !defaults.is_empty() {
            step.details
                .push(format!("defaults=[{}] FLEET-005", defaults.join(",")));
        }
        for n in names {
            if let Some(own) = self.desired.objects[&n].own_tools.as_ref() {
                step.details
                    .push(format!("ownTools {n} {} CTR-014", safe(own)));
            }
        }
        for n in names_held_details(self.desired, step) {
            step.details.push(n);
        }
    }

    fn registration_effect(&self, names: &[Name]) -> Effect {
        let objects: BTreeMap<_, _> = names
            .iter()
            .map(|name| (name.clone(), self.desired.objects[name].resolved.clone()))
            .collect();
        let mut defaults = Vec::new();
        for (name, object) in &objects {
            if object.is_template() {
                continue;
            }
            let template = match name.kind.as_str() {
                "Pool" => "root-pool",
                "Fleet" => "root-fleet",
                _ => continue,
            };
            let mut view = object.clone();
            view.state = "ready".into();
            view.packages = vec![self.desired.core_pin.clone()];
            view.definition = json!({"from":format!("{}#{template}", self.desired.core_address),
                "consumer":Name::new("Interface","root").reference(),"target":name.reference(),
                "as":name.name,"engine-made":true,"provenance":{"cause":"default"}});
            defaults.push((template.to_owned(), view));
        }
        defaults.sort_by(|(_, a), (_, b)| text(&a.definition["as"]).cmp(text(&b.definition["as"])));
        let roots = names.iter().cloned().collect();
        let nested = self
            .desired
            .nested
            .iter()
            .filter(|child| self.added_with_parent(&child.parent, &roots))
            .cloned()
            .collect();
        Effect::Register {
            objects,
            defaults,
            nested,
        }
    }

    fn push(
        &mut self,
        tool: &str,
        subject: Name,
        expected: Option<String>,
        relative: Expected,
        class: &'static str,
        rule: &'static str,
    ) -> &mut Step {
        let mut args = json!({});
        if subject.kind != "Tombstone" {
            let field = match tool
                .rsplit_once('.')
                .and_then(|(stem, _)| stem.rsplit('.').next())
            {
                Some("open" | "close") => "conduit",
                Some("set-up") => "instance",
                _ => "object",
            };
            args[field] = subject.reference();
            if let Some(id) = &expected {
                args["expectedId"] = json!(id);
            }
        }
        self.result.steps.push(Step {
            tool: tool.into(),
            subject,
            expected,
            relative,
            class,
            rule,
            arguments: args,
            details: vec![],
            action: Action::default(),
        });
        self.result.steps.last_mut().unwrap()
    }

    fn access(&self) -> &str {
        self.engine
            .routes
            .get(&Name::new("Resource", "access"))
            .map(String::as_str)
            .unwrap_or("access")
    }

    fn owner_route(&self, definition: &Value, kind: &str) -> String {
        let owner_kind = match kind {
            "Resource" => "Pool",
            "Agent" => "Fleet",
            _ => return "management".into(),
        };
        let Some(owner) = name_reference(&definition.get("owner"), owner_kind) else {
            return "management".into();
        };
        self.engine
            .routes
            .get(&owner)
            .cloned()
            .unwrap_or(owner.name)
    }

    fn unknown(&mut self) {
        for (name, old) in &self.engine.objects {
            let hold = self.touched.contains(name);
            if !hold
                && !self.desired.objects.contains_key(name)
                && !self.ledger.objects.contains_key(name)
            {
                continue;
            }
            let prefix = if hold { "stop" } else { "difference" };
            let mut lines: Vec<_> = array(&old.status["unreconciled"])
                .iter()
                .filter_map(Value::as_str)
                .map(|id| format!("{prefix} {name} unreconciled={id} OPS-013"))
                .collect();
            if let Some(want) = self.desired.objects.get(name) {
                // Reconciliation settles the invocation's outcome but cannot confirm the
                // setting. Inspection still omits the per-setting state required by REV-009.
                for (id, operation) in &old.unknown_endings {
                    for (setting, controls) in &want.controls {
                        if controls.contains(operation) {
                            lines.push(format!(
                                "{prefix} {name} setting={setting} control={operation} invocation={id} inspection=unknown-settings-not-built REV-009"
                            ));
                        }
                    }
                }
            }
            if hold {
                self.result.stops.extend(lines);
            } else {
                self.result.differences.extend(lines);
            }
        }
    }

    fn retained_differences(&mut self) {
        for closed in &self.close {
            if self.narrow.contains(closed) {
                continue;
            }
            let old = &self.engine.objects[closed];
            for (name, other) in &self.engine.objects {
                if name.kind != "Conduit"
                    || other.made_by_engine
                    || !self.desired.objects.contains_key(name)
                    || self.close.contains(name)
                    || other.state != "ready"
                {
                    continue;
                }
                if other.definition["target"] == old.definition["target"]
                    && other.definition["as"] == old.definition["as"]
                {
                    self.result.differences.push(format!(
                        "difference {} remains ready id={} operations=[{}]",
                        name.name,
                        other.id,
                        array(&other.definition["resolved"]["operations"])
                            .iter()
                            .filter_map(Value::as_str)
                            .collect::<Vec<_>>()
                            .join(",")
                    ));
                }
            }
        }
        let mut grouped = BTreeSet::new();
        for name in self.desired.objects.keys() {
            let Some(old) = self.engine.objects.get(name) else {
                continue;
            };
            if old.is_template()
                || name.kind == "Conduit"
                || self.ledger.objects.get(name).is_none_or(|k| k.id != old.id)
            {
                continue;
            }
            let grants: Vec<_> = self
                .desired
                .objects
                .iter()
                .filter(|(n, w)| {
                    n.kind == "Conduit" && w.resolved.definition["target"] == name.reference()
                })
                .filter_map(|(n, w)| Some((n, self.engine.objects.get(n)?, w)))
                .filter(|(n, o, w)| self.retains(n, w, o))
                .collect();
            for pin in old.pins() {
                let Some((short, held_version)) = short_pin(&pin) else {
                    continue;
                };
                let mut retain = BTreeSet::new();
                for (grant, old, want) in &grants {
                    for held in old.pins().filter(|p| {
                        p.address == pin.address
                            && want
                                .resolved
                                .pins()
                                .any(|n| n.address == p.address && n.digest != p.digest)
                    }) {
                        grouped.insert(((*grant).clone(), held.address.clone()));
                        retain.insert(held);
                    }
                }
                for grant in retain {
                    let Some((_, grant_version)) = short_pin(&grant) else {
                        continue;
                    };
                    self.result.differences.push(format!("difference {} holds {short}={held_version}; {} grants retain {short}={grant_version} NAM-010",name.name,name.name));
                }
            }
        }
        for (name, want) in &self.desired.objects {
            let Some(old) = self
                .engine
                .objects
                .get(name)
                .filter(|old| self.retains(name, want, old))
            else {
                continue;
            };
            for new in changed_release_pins(want, old) {
                if let Some(held) = old.pins().find(|pin| pin.address == new.address)
                    && !grouped.contains(&(name.clone(), held.address.clone()))
                {
                    self.result.differences.push(format!(
                        "difference {name} holds {}; file selects {} NAM-010",
                        held.label(),
                        new.label()
                    ));
                }
            }
            if changed_project(want, old)
                && let Some(held) = project_pin(old)
            {
                self.result.differences.push(format!(
                    "difference {name} holds {} digest={} NAM-010",
                    text(&held["address"]),
                    text(&held["digest"])
                ));
            }
        }
    }

    fn retains(&self, name: &Name, want: &DesiredObject, old: &ObservedObject) -> bool {
        !self.replacement_flags.contains_key(name)
            && !old.is_template()
            && !held_address_changed(self.desired, old)
            && self
                .ledger
                .objects
                .get(name)
                .is_some_and(|kept| kept.id == old.id)
            && comparable(&old.definition) == comparable(&want.resolved.definition)
            && !changed_documents(want, old)
            && redacted_selection(want, old).is_none()
            && same_contracts(want, old, self.desired)
            && same_items(want, old, self.desired)
    }
}

fn short_pin(pin: &Pin) -> Option<(&str, &str)> {
    Some((pin.address.rsplit('/').next()?, pin.version.as_deref()?))
}

fn names_held_details(desired: &Desired, step: &Step) -> Vec<String> {
    let mut out = vec![];
    let manifest = desired.project.manifest.as_ref().unwrap_or(&Value::Null);
    for name in array(&step.arguments["documents"])
        .iter()
        .filter_map(Name::document)
    {
        let Some(object) = desired.objects.get(&name) else {
            continue;
        };
        if object.resolved.is_template() || name.kind == "Conduit" {
            continue;
        }
        for pin in object.resolved.pins() {
            let listed = text(&manifest["packages"][&pin.address]["version"]);
            if let Some((short, version)) = short_pin(&pin)
                && !listed.is_empty()
                && listed != version
            {
                out.push(format!(
                    "held-{}-{short}={version} project-{short}={listed} NAM-010",
                    name.name
                ));
            }
        }
    }
    out
}

fn name_reference(value: &Option<&Value>, kind: &str) -> Option<Name> {
    let value = (*value)?;
    Name::object(value).or_else(|| Some(Name::new(kind, value.as_str()?)))
}

fn activation_dependencies(name: &Name, definition: &Value) -> impl Iterator<Item = Name> {
    let owner_kind = match name.kind.as_str() {
        "Resource" => Some("Pool"),
        "Agent" => Some("Fleet"),
        _ => None,
    };
    owner_kind
        .and_then(|kind| name_reference(&definition.get("owner"), kind))
        .into_iter()
        .chain(name_reference(&definition.get("runtime"), "Resource"))
        .chain(Name::object(&definition["consumer"]))
        .chain(Name::object(&definition["target"]))
}

fn owns_members(engine: &Engine, owner: &Name) -> bool {
    let member_kind = match owner.kind.as_str() {
        "Pool" => "Resource",
        "Fleet" => "Agent",
        _ => return false,
    };
    engine.objects.iter().any(|(name, obj)| {
        name.kind == member_kind
            && !obj.is_template()
            && name_reference(&obj.definition.get("owner"), &owner.kind).as_ref() == Some(owner)
    })
}

fn object_binding(relative: Expected, field: &'static str) -> Option<Binding> {
    match relative {
        Expected::Step(from) => Some(Binding::Object { from, field }),
        Expected::None => None,
    }
}

fn nested_slot<'a>(engine: &'a Engine, name: &Name) -> Option<(&'a Name, &'static str, &'a str)> {
    let id = &engine.objects.get(name)?.id;
    nested_slot_for(engine, name, id)
}

pub(super) fn nested_slot_for<'a>(
    engine: &'a Engine,
    name: &Name,
    id: &str,
) -> Option<(&'a Name, &'static str, &'a str)> {
    for (parent, object) in &engine.objects {
        for field in ["entries", "conduits"] {
            if let Some((slot, _)) = object.definition["slots"][field]
                .as_object()
                .into_iter()
                .flatten()
                .find(|(_, child)| child["id"] == id || Name::object(child).as_ref() == Some(name))
            {
                return Some((parent, field, slot));
            }
        }
    }
    None
}

fn same_match(left: &Value, right: &Value) -> bool {
    left["consumer"] == right["consumer"]
        && (left["as"] == right["as"]
            || left["target"] == right["target"]
                && left["resolved"]["capability"] == right["resolved"]["capability"])
}

fn lifts_caused(names: &BTreeSet<Name>) -> String {
    format!(
        "lifts-caused=[{}]",
        names
            .iter()
            .map(|name| name.name.as_str())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn tombstone_root<'a>(stones: &'a [Value], stone: &'a Value) -> &'a Value {
    tombstone_chain(stones, stone)
        .last()
        .copied()
        .unwrap_or(stone)
}

fn tombstone_chain<'a>(stones: &'a [Value], mut stone: &'a Value) -> Vec<&'a Value> {
    let mut chain = vec![stone];
    let mut seen = BTreeSet::new();
    while let Some(parent) = stone["cause"]["tombstone"].as_str() {
        if !seen.insert(parent) {
            break;
        }
        let Some(ancestor) = stones.iter().find(|candidate| candidate["id"] == parent) else {
            break;
        };
        stone = ancestor;
        chain.push(stone);
    }
    chain
}

pub(super) fn matches_tombstone(name: &Name, definition: &Value, stone: &Value) -> bool {
    if stone["object"]["kind"] != name.kind {
        return false;
    }
    if name.kind != "Conduit" {
        return stone["object"]["name"] == name.name;
    }
    let same_end = |field: &str| {
        let (wanted, held) = (&definition[field], &stone[field]);
        wanted["kind"].is_string()
            && wanted["kind"] == held["kind"]
            && wanted["name"].is_string()
            && wanted["name"] == held["name"]
    };
    let local = definition["as"].as_str().unwrap_or(&name.name);
    same_end("consumer")
        && (stone["localName"] == local
            || same_end("target") && stone["capability"] == definition["resolved"]["capability"])
}

pub(super) fn comparable(value: &Value) -> Value {
    let mut result = safe(value);
    if let Some(map) = result.as_object_mut() {
        for key in ["provenance", "slots", "engine-made"] {
            map.remove(key);
        }
    }
    if let Some(requirements) = result
        .get_mut("resolved")
        .and_then(|spec| spec.get_mut("requirements"))
        .and_then(Value::as_object_mut)
    {
        for req in requirements.values_mut().filter_map(Value::as_object_mut) {
            for key in ["mode", "implementation", "implementations"] {
                req.remove(key);
            }
        }
    }
    result
}

pub(super) fn differences(left: &Value, right: &Value, pointer: &str, out: &mut Vec<String>) {
    if left == right {
        return;
    }
    if left.get("pathSha256").is_some() && right.get("pathSha256").is_some() {
        out.push(pointer.to_owned());
        return;
    }
    if let (Some(left), Some(right)) = (left.as_object(), right.as_object()) {
        for key in left.keys().chain(right.keys()).collect::<BTreeSet<_>>() {
            let child = format!("{pointer}/{}", key.replace('~', "~0").replace('/', "~1"));
            match (left.get(key), right.get(key)) {
                (Some(left), Some(right)) => differences(left, right, &child, out),
                _ => out.push(child),
            }
        }
    } else {
        out.push(pointer.to_owned());
    }
}

fn setting_pointer(place: &str) -> Option<&str> {
    let setting = place.strip_prefix("/resolved/config")?;
    (setting.is_empty() || setting.starts_with('/')).then_some(setting)
}

fn is_subset(left: &Value, right: &Value) -> bool {
    if left["consumer"] != right["consumer"]
        || left["target"] != right["target"]
        || !array(&right["resolved"]["operations"])
            .iter()
            .all(|op| array(&left["resolved"]["operations"]).contains(op))
    {
        return false;
    }
    for (name, old) in left["resolved"]["requirements"]
        .as_object()
        .into_iter()
        .flatten()
    {
        let new = &right["resolved"]["requirements"][name];
        if old == new {
            continue;
        }
        if old["requirement"] != new["requirement"]
            || old["requirement"] != "github.com/shrav-k/conduit/core#bound.v1"
        {
            return false;
        }
        let mut old_args = old["arguments"].clone();
        let mut new_args = new["arguments"].clone();
        for key in ["pools", "targets", "operations"] {
            if new_args.get(key).is_some_and(|new| !new.is_array())
                || old_args.get(key).is_some_and(|old| {
                    !old.as_array()
                        .zip(new_args[key].as_array())
                        .is_some_and(|(old, new)| new.iter().all(|child| old.contains(child)))
                })
            {
                return false;
            }
            if let Some(obj) = old_args.as_object_mut() {
                obj.remove(key);
            }
            if let Some(obj) = new_args.as_object_mut() {
                obj.remove(key);
            }
        }
        if old_args != new_args {
            return false;
        }
    }
    true
}

fn project_pin(object: &ObservedObject) -> Option<&Value> {
    object
        .packages
        .iter()
        .find(|p| p.get("documents").is_some())
}

fn held_address_changed(desired: &Desired, old: &ObservedObject) -> bool {
    let address = desired
        .project
        .manifest
        .as_ref()
        .and_then(|manifest| manifest["address"].as_str());
    project_pin(old).is_some_and(|package| package["address"].as_str() != address)
}

fn changed_project(want: &DesiredObject, old: &ObservedObject) -> bool {
    match (project_pin(&want.resolved), project_pin(old)) {
        (Some(a), Some(b)) => a["digest"] != b["digest"],
        (None, None) => false,
        _ => true,
    }
}

fn changed_documents(want: &DesiredObject, old: &ObservedObject) -> bool {
    if !changed_project(want, old) {
        return false;
    }
    let held = project_pin(old);
    want.reached_documents.iter().any(|((kind, name), digest)| {
        !held.is_some_and(|p| {
            array(&p["documents"])
                .iter()
                .any(|d| d["kind"] == *kind && d["name"] == *name && d["digest"] == *digest)
        })
    }) || want
        .reached_packages
        .iter()
        .any(|address| !old.pins().any(|p| p.address == *address))
}

fn changed_release_pins(want: &DesiredObject, old: &ObservedObject) -> Vec<Pin> {
    want.resolved
        .packages
        .iter()
        .filter(|p| p.get("documents").is_none())
        .filter_map(Pin::from_value)
        .filter(|pin| !old.pins().any(|held| held == *pin))
        .collect()
}

fn marked(value: &Value) -> bool {
    match value {
        Value::String(s) => s == "(secret: not shown)",
        Value::Array(items) => items.iter().any(marked),
        Value::Object(fields) => fields.values().any(marked),
        _ => false,
    }
}

fn redacted_selection(want: &DesiredObject, old: &ObservedObject) -> Option<String> {
    if !changed_project(want, old) || changed_documents(want, old) {
        return None;
    }
    let address = project_pin(old)?["address"].as_str()?;
    old.implementations
        .as_object()?
        .iter()
        .find_map(|(operation, item)| {
            let next = want.resolved.implementations.get(operation)?;
            (item["package"]["address"] == address
                && next["package"]["address"] == address
                && item["package"]["version"] == next["package"]["version"]
                && item["selection"] == next["selection"]
                && item["item"] == next["item"]
                && marked(&item["item"]))
            .then(|| item["selection"].as_str().map(str::to_owned))
            .flatten()
        })
}

fn same_items(want: &DesiredObject, old: &ObservedObject, desired: &Desired) -> bool {
    let (Some(next), Some(previous)) = (
        want.resolved.implementations.as_object(),
        old.implementations.as_object(),
    ) else {
        return want.resolved.implementations == old.implementations
            || old.implementations.is_null()
                && want
                    .resolved
                    .implementations
                    .as_object()
                    .is_some_and(|m| m.is_empty());
    };
    if next.len() != previous.len() {
        return false;
    }
    for (operation, item) in next {
        let Some(prior) = previous.get(operation) else {
            return false;
        };
        if item["selection"] != prior["selection"] {
            return false;
        }
        if item["package"] == prior["package"] {
            if project_pin(&want.resolved)
                .is_some_and(|p| p["address"] == item["package"]["address"])
                && item["item"] != prior["item"]
            {
                return false;
            }
            continue;
        }
        if project_pin(&want.resolved).is_some_and(|p| p["address"] == item["package"]["address"]) {
            if item["item"] != prior["item"] {
                return false;
            }
            continue;
        }
        let Some(a) = Pin::from_value(&prior["package"]).and_then(|p| package(desired, &p)) else {
            return false;
        };
        let Some(b) = Pin::from_value(&item["package"]).and_then(|p| package(desired, &p)) else {
            return false;
        };
        let key = text(&item["selection"])
            .split_once('#')
            .map(|(_, name)| name)
            .unwrap_or("");
        let a_item =
            &a.package.project.manifest.as_ref().unwrap_or(&Value::Null)["implementations"][key];
        let b_item =
            &b.package.project.manifest.as_ref().unwrap_or(&Value::Null)["implementations"][key];
        if a_item != b_item {
            return false;
        }
        if let Some(program) = a_item["command"][0]
            .as_str()
            .filter(|s| s.contains('/') && !s.starts_with('/'))
        {
            match (
                conduit_core::archives::resolve(&a.files, program),
                conduit_core::archives::resolve(&b.files, program),
            ) {
                (
                    Ok(Some(conduit_core::archives::Member::File { bytes: a, .. })),
                    Ok(Some(conduit_core::archives::Member::File { bytes: b, .. })),
                ) if a == b => {}
                _ => return false,
            }
        }
    }
    true
}

fn package<'a>(
    desired: &'a Desired,
    pin: &Pin,
) -> Option<&'a conduit_core::archives::InstalledPackage> {
    match desired.available_archives.get(pin)? {
        Archive::Read(package) => Some(package),
        _ => None,
    }
}

fn same_contracts(want: &DesiredObject, old: &ObservedObject, desired: &Desired) -> bool {
    let mut selected = Vec::new();
    let description =
        json!({"kind":want.document["kind"],"spec":want.resolved.definition["resolved"]});
    if conduit_core::packages::selections_document(&description, true, &mut selected).is_err() {
        return false;
    }
    for operation in want
        .resolved
        .implementations
        .as_object()
        .into_iter()
        .flat_map(|m| m.keys())
    {
        selected.push((operation.clone(), "Operation".into()));
    }

    for pin in changed_release_pins(want, old) {
        if pin.digest.is_none() {
            continue;
        }
        let Some(prior) = old.pins().find(|prior| prior.address == pin.address) else {
            return false;
        };
        let (Some(left), Some(right)) = (package(desired, &prior), package(desired, &pin)) else {
            return false;
        };
        let mut pending = Vec::new();
        for (qualified, kind) in &selected {
            if let Some((address, name)) = qualified.split_once('#')
                && address == pin.address
            {
                pending.push((left, right, name.to_owned(), kind.clone()));
            }
        }
        let mut checked = BTreeSet::new();
        while let Some((left, right, name, kind)) = pending.pop() {
            if !checked.insert((
                &left.package.digest,
                &right.package.digest,
                name.clone(),
                kind.clone(),
            )) {
                continue;
            }
            let lookup = |package: &'_ conduit_core::archives::InstalledPackage| {
                package
                    .package
                    .project
                    .documents
                    .iter()
                    .position(|doc| doc["kind"] == kind && doc["metadata"]["name"] == name)
            };
            let (Some(i), Some(j)) = (lookup(left), lookup(right)) else {
                return false;
            };
            let before = &left.package.project.documents[i];
            let after = &right.package.project.documents[j];
            if before != after {
                return false;
            }
            let mut references = Vec::new();
            if conduit_core::packages::selections_document(after, true, &mut references).is_err() {
                return false;
            }
            for (reference, kind) in references {
                let Some((address, name)) = reference.split_once('#') else {
                    continue;
                };
                if address.is_empty() || address == left.package.address {
                    pending.push((left, right, name.into(), kind));
                    continue;
                }
                let (previous, next) = (dependency(left, address), dependency(right, address));
                if previous == next {
                    continue;
                }
                let (Some(previous), Some(next)) = (previous, next) else {
                    return false;
                };
                let (Some(previous), Some(next)) =
                    (package(desired, &previous), package(desired, &next))
                else {
                    return false;
                };
                pending.push((previous, next, name.into(), kind));
            }
        }
    }
    true
}

fn dependency(archive: &conduit_core::archives::InstalledPackage, address: &str) -> Option<Pin> {
    let manifest = archive.package.project.manifest.as_ref()?;
    let identity = &manifest["packages"][address];
    Some(Pin {
        address: address.into(),
        version: Some(identity["version"].as_str()?.into()),
        digest: Some(identity["digest"].as_str()?.into()),
    })
}
