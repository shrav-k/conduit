use super::apply::{Action, Binding, Effect};
use super::*;

pub(super) fn plan(desired: &Desired, ledger: &Ledger, engine: &Engine) -> Plan {
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
        result,
        touched: BTreeSet::new(),
        add: vec![],
        replace_templates: vec![],
        close: BTreeSet::new(),
        narrow: BTreeSet::new(),
        resume: vec![],
    };
    changes.collect();
    changes.steps();
    changes.unknown();
    changes.result
}

struct Changes<'a> {
    desired: &'a Desired,
    ledger: &'a Ledger,
    engine: &'a Engine,
    result: Plan,
    touched: BTreeSet<Name>,
    add: Vec<Name>,
    replace_templates: Vec<(Name, Option<String>)>,
    close: BTreeSet<Name>,
    narrow: BTreeSet<Name>,
    resume: Vec<Name>,
}

impl Changes<'_> {
    fn stop(&mut self, name: &Name, cause: &str) {
        self.result.stops.insert(format!("stop {name} {cause}"));
    }

    fn flag(&mut self, name: &Name, cause: &str, flag: &str, rule: &str) {
        self.touched.insert(name.clone());
        self.stop(
            name,
            &format!("{cause} requires={flag} {} {rule}", name.name),
        );
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
        for name in &names {
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
                if let Some(flag) = self.ledger.registration_flags.get(&kept.id) {
                    self.stop(name, &format!("state=registered requires={flag} LIF-013"));
                    return;
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
            } else {
                self.flag(name, "place=/from", "--replace", "LIF-009");
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
                    self.flag(name, &format!("place={place}"), "--replace", "OBJ-011");
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
                    if !self.desired.objects.contains_key(other)
                        && !self.ledger.objects.contains_key(other)
                    {
                        self.flag(other, "absent=file", "--replace", "OBJ-011");
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
            if let Some(stone) = self
                .engine
                .tombstones
                .iter()
                .find(|s| matches_tombstone(name, &want.resolved.definition, s))
            {
                self.flag(
                    name,
                    &format!("tombstone={}", text(&stone["id"])),
                    flag,
                    "LIF-041",
                );
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
                self.flag(name, &format!("tombstone=({prior})"), flag, "LIF-041");
                return;
            }
        }
        if kept.is_some() {
            self.flag(name, "absent=engine", flag, "LIF-040");
        } else {
            self.add.push(name.clone());
            self.touched.insert(name.clone());
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
            if let Some(manifest) = self.desired.project.manifest.as_ref() {
                details.push(format!("manifest={}", safe(manifest)));
            }
            if let Some(item) = redacted {
                details.push(format!("item={item} secret=not-shown SEC-021"));
            }
            let step = Step {
                tool: "management.replace.v1".into(),
                subject: name.clone(),
                expected: Some(observed.id.clone()),
                relative: Expected::None,
                class: "replaces",
                rule: "SUB-009",
                arguments: json!({"template":desired.document,"revision":observed.revision,"manifest":self.desired.project.manifest,
                    "contracts":self.desired.project.documents.iter().filter(|d|conduit_core::packages::contract(d)).collect::<Vec<_>>()}),
                details,
                action: Action {
                    effect: Effect::Replace(Box::new(desired.resolved.clone())),
                    ..Action::default()
                },
            };
            self.result.steps.push(step);
        }
        if !self.close.is_empty() {
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
        let closes: Vec<_> = self.close.iter().cloned().collect();
        let mut close_steps = BTreeMap::new();
        for name in closes {
            self.result.closed_conduits.insert(name.name.clone());
            close_steps.insert(name.clone(), self.result.steps.len() + 1);
            self.push(
                &format!("{}.close.v1", self.access()),
                name.clone(),
                Some(self.engine.objects[&name].id.clone()),
                Expected::None,
                "narrows",
                "LIF-037",
            )
            .action
            .effect = Effect::Remove;
        }
        if !self.close.is_empty() {
            let list_at = self.result.steps.len() + 1;
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
            let narrow: Vec<_> = self.narrow.iter().cloned().collect();
            for name in narrow {
                let binding = Binding::Tombstone {
                    closed: close_steps[&name],
                    listed: list_at,
                    object: self.engine.objects[&name].id.clone(),
                };
                self.push(
                    &format!("{}.lift.v1", self.access()),
                    Name::new("Tombstone", &name.name),
                    None,
                    Expected::Step(list_at),
                    "neither",
                    "ACC-053",
                )
                .action = Action {
                    binding: Some(binding),
                    effect: Effect::Lift,
                    ..Action::default()
                };
                if let Some((parent, field, slot)) = nested_slot(self.engine, &name) {
                    nested_activations.push((name, parent.clone(), field, slot.to_owned()));
                } else {
                    self.add.push(name);
                }
            }
        }
        let nested_activations: Vec<_> = nested_activations
            .into_iter()
            .map(|(child, parent, field, slot)| {
                let registration = self.result.steps.len() + 1;
                let id = self.engine.objects[&parent].id.clone();
                self.result.steps.push(Step {
                    tool: "management.register-replacement.v1".into(),
                    subject: parent.clone(),
                    expected: Some(id),
                    relative: Expected::None,
                    class: "neither",
                    rule: "LIF-009",
                    arguments: json!({"instance": parent.reference(), "nested": slot}),
                    details: vec![format!("nested={slot}")],
                    action: Action {
                        effect: Effect::RegisterReplacement {
                            wanted: Box::new(self.desired.objects[&child].resolved.clone()),
                            parent,
                            field,
                            slot,
                            previous: child.clone(),
                        },
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
            let rule = if adds.iter().all(|n| self.narrow.contains(n)) {
                "SCT-007"
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
            (rank(&a.kind, &order), &a.name).cmp(&(rank(&b.kind, &order), &b.name))
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
        .action
        .effect = effect;
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
        step.details.push(format!(
            "objects=[{}]",
            names
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ));
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
        Effect::Register {
            objects,
            defaults,
            nested: self.desired.nested.clone(),
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
        !old.is_template()
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
    for (parent, object) in &engine.objects {
        for field in ["entries", "conduits"] {
            if let Some((slot, _)) = object.definition["slots"][field]
                .as_object()
                .into_iter()
                .flatten()
                .find(|(_, child)| child["id"] == *id || Name::object(child).as_ref() == Some(name))
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

fn matches_tombstone(name: &Name, definition: &Value, stone: &Value) -> bool {
    if name.kind == "Conduit" {
        stone["object"]["kind"] == "Conduit"
            && same_match(
                definition,
                &json!({
                    "consumer":{"kind":stone["consumer"]["kind"],"name":stone["consumer"]["name"]},
                    "target":{"kind":stone["target"]["kind"],"name":stone["target"]["name"]},
                    "as":stone["localName"],"resolved":{"capability":stone["capability"]}
                }),
            )
    } else {
        stone["object"]["kind"] == name.kind && stone["object"]["name"] == name.name
    }
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
