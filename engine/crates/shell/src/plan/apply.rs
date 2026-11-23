use super::*;
use crate::{Result, store, user_config};
use rmcp::{
    RoleClient,
    model::CallToolRequestParams,
    service::{RunningService, ServiceError},
};
use std::fs;
use std::io::{ErrorKind, Write as _};
use std::os::unix::fs::DirBuilderExt;
use std::time::Duration;

const REQUEST_TIMEOUT_SECONDS: u64 = 150;
const META: &str = "io.github.shrav-k.conduit/";

#[derive(Clone, Debug, Default)]
pub(super) struct Action {
    pub(super) binding: Option<Binding>,
    pub(super) check: Option<ObservedObject>,
    pub(super) effect: Effect,
    pub(super) panel: bool,
    pub(super) approvals: BTreeSet<Approval>,
    pub(super) remakes: BTreeSet<Name>,
    pub(super) replacement_slots: BTreeMap<Name, ReplacementSlot>,
    pub(super) opens: Option<(Name, Value)>,
}

#[derive(Clone, Debug)]
pub(super) enum Binding {
    Object {
        from: usize,
        field: &'static str,
    },
    Tombstone {
        closed: usize,
        listed: usize,
        object: String,
        prior_cause: Option<String>,
    },
}

#[derive(Clone, Debug, Default)]
pub(super) enum Effect {
    #[default]
    Unspecified,
    None,
    Register {
        objects: BTreeMap<Name, ObservedObject>,
        defaults: Vec<(String, ObservedObject)>,
        nested: Vec<NestedRegistration>,
    },
    Replace(Box<ObservedObject>),
    RegisterReplacement {
        wanted: Box<ObservedObject>,
        parent: Name,
        field: &'static str,
        slot: String,
        previous: Name,
    },
    SetLifecycle {
        state: &'static str,
        opened_by: Option<(String, String)>,
    },
    Remove,
    Retire {
        closes_on_dispatch: BTreeSet<Name>,
        closes_on_removal: BTreeSet<Name>,
    },
    Tombstones,
    Lift,
}

impl Action {
    fn tombstone_candidates(&self) -> Vec<(&Name, &Value)> {
        match &self.effect {
            Effect::Register {
                objects, nested, ..
            } => objects
                .iter()
                .filter(|(_, object)| !object.is_template())
                .map(|(name, object)| (name, &object.definition))
                .chain(
                    nested
                        .iter()
                        .map(|child| (&child.name, &child.resolved.definition)),
                )
                .collect(),
            Effect::RegisterReplacement {
                previous, wanted, ..
            } => {
                vec![(previous, &wanted.definition)]
            }
            _ => self
                .opens
                .iter()
                .map(|(name, definition)| (name, definition))
                .collect(),
        }
    }
}

pub(super) async fn send(
    client: &RunningService<RoleClient, ()>,
    tool: &str,
    args: Value,
    key: &str,
) -> Result<Value> {
    let mut request = CallToolRequestParams::new(tool.to_owned()).with_arguments(
        args.as_object()
            .ok_or("step arguments are not a map")?
            .clone(),
    );
    request.meta = Some(
        serde_json::from_value(json!({
            format!("{META}idempotencyKey"):key,
            "io.modelcontextprotocol/clientCapabilities":{"extensions":{
                format!("{META}implementation-interface"):{"revision":"v0"}
            }}
        }))
        .map_err(|_| "invalid call metadata")?,
    );
    match tokio::time::timeout(
        Duration::from_secs(REQUEST_TIMEOUT_SECONDS),
        client.call_tool(request),
    )
    .await
    {
        Ok(Ok(result)) => serde_json::to_value(result).map_err(|_| "invalid response".into()),
        Ok(Err(ServiceError::McpError(error))) => {
            let raw = serde_json::to_value(error).map_err(|_| "invalid protocol error")?;
            if raw["data"].get(format!("{META}reason")).is_none() {
                return Err("engine returned an unclassified protocol error".into());
            }
            Ok(
                json!({"_meta":raw["data"],"structuredContent":{"diagnostics":raw["data"]["diagnostics"]}}),
            )
        }
        Ok(Err(_)) => Err("transport failed".into()),
        Err(_) => Err(format!(
            "request timed out after {REQUEST_TIMEOUT_SECONDS} s"
        )),
    }
}

fn projection(raw: &Value) -> Value {
    let mut view = raw["structuredContent"]
        .as_object()
        .cloned()
        .unwrap_or_default();
    for key in ["records", "recordGaps", "recordsComplete"] {
        view.remove(key);
    }
    for key in ["outcome", "reason", "invocation", "pendingAction"] {
        if let Some(value) = raw["_meta"].get(format!("{META}{key}")) {
            view.insert(key.into(), value.clone());
        }
    }
    safe(&Value::Object(view))
}

fn outcome(result: &Value) -> &str {
    result["outcome"]
        .as_str()
        .or_else(|| result["reason"].as_str())
        .unwrap_or("unknown")
}

pub(super) fn dispatched_unsuccessful_retirement(result: &Value) -> bool {
    matches!(result["outcome"].as_str(), Some("failed" | "unknown"))
        && result["invocation"].as_str().is_some()
}

#[derive(Debug)]
enum BindingError {
    Invalid(&'static str),
    TombstoneMismatch { listed: usize, cause: String },
}

impl From<&'static str> for BindingError {
    fn from(message: &'static str) -> Self {
        Self::Invalid(message)
    }
}

fn step_arguments(
    step: &Step,
    action: &Action,
    results: &[Value],
) -> std::result::Result<Value, BindingError> {
    let mut args = step.arguments.clone();
    match &action.binding {
        Some(Binding::Object { from, field }) => {
            let result = from
                .checked_sub(1)
                .and_then(|index| results.get(index))
                .ok_or("missing earlier result")?;
            let item = array(&result["objects"])
                .iter()
                .find(|item| Name::object(item).as_ref() == Some(&step.subject))
                .or_else(|| result.get("object"))
                .ok_or("earlier registration did not return the object")?;
            args["expectedId"] = json!(
                item["id"]
                    .as_str()
                    .ok_or("earlier registration omitted the ID")?
            );
            args[*field] = json!({"kind":item["kind"],"name":item["name"]});
        }
        Some(Binding::Tombstone {
            closed,
            listed,
            object,
            prior_cause,
        }) => {
            let invocation = results
                .get(closed.checked_sub(1).ok_or("invalid close binding")?)
                .and_then(|result| result["invocation"].as_str())
                .ok_or("missing closing invocation")?;
            let result = results
                .get(listed.checked_sub(1).ok_or("invalid list binding")?)
                .ok_or("missing tombstone result")?;
            let stones = result["tombstones"]
                .as_array()
                .ok_or("tombstone list result omitted its tombstones")?;
            let matches: Vec<_> = stones
                .iter()
                .filter(|stone| stone["object"]["id"] == *object)
                .collect();
            let mismatch = |cause| BindingError::TombstoneMismatch {
                listed: *listed,
                cause,
            };
            let [stone] = matches.as_slice() else {
                return Err(mismatch(format!(
                    "expected one tombstone for the removed object, found {}",
                    matches.len()
                )));
            };
            let cause = text(&stone["cause"]["invocation"]);
            if cause != invocation && prior_cause.as_deref() != Some(cause) {
                return Err(mismatch(
                    "the tombstone was not caused by the planned removal".into(),
                ));
            }
            let id = stone["id"]
                .as_str()
                .ok_or("matched tombstone omitted its ID")?;
            args["id"] = json!(id);
        }
        None => {}
    }
    Ok(args)
}

fn verify(
    raw: &Value,
    id: &Value,
    expected: &ObservedObject,
    names: &BTreeMap<String, Name>,
) -> Option<String> {
    if &raw["id"] != id {
        return Some("/id".into());
    }
    let names = names
        .iter()
        .map(|(id, name)| (id.as_str(), name.clone()))
        .collect();
    let actual = read::compact_definition(&read::name_refs(&raw["declared"], &names));
    let mut differences = Vec::new();
    compare::differences(
        &compare::comparable(&expected.definition),
        &compare::comparable(&actual),
        "/declared",
        &mut differences,
    );
    if let Some(first) = differences.into_iter().next() {
        return Some(first);
    }
    let expected_slots = read::name_refs(&expected.definition["slots"], &names);
    for field in ["conduits", "entries"] {
        let wanted = &expected_slots[field];
        let observed = &actual["slots"][field];
        let slots: BTreeSet<_> = [wanted, observed]
            .into_iter()
            .filter_map(Value::as_object)
            .flat_map(|slots| slots.keys())
            .collect();
        for slot in slots {
            if wanted.get(slot) != observed.get(slot) {
                let token = slot.replace('~', "~0").replace('/', "~1");
                return Some(format!("/declared/slots/{field}/{token}"));
            }
        }
    }
    let sorted = |packages: &[Value]| {
        let mut sorted = packages.to_vec();
        sorted.sort_by_key(Value::to_string);
        sorted
    };
    (sorted(array(&raw["packages"])) != sorted(&expected.packages)).then(|| "/packages".into())
}

fn apply_effect(
    ledger: &mut Ledger,
    names: &mut BTreeMap<String, Name>,
    step: &Step,
    effect: &Effect,
    result: &Value,
    args: &Value,
) -> Result<()> {
    match effect {
        Effect::Unspecified => return Err("planner omitted the ledger effect".into()),
        Effect::None => {}
        Effect::Register {
            objects,
            defaults,
            nested,
        } => {
            let mut defaults = defaults.clone();
            let mut updated = BTreeMap::new();
            let mut matched = BTreeMap::new();
            let mut renamed = BTreeMap::new();
            for item in array(&result["objects"]) {
                let name = Name::object(item).ok_or("registered object has no name")?;
                let id = item["id"].as_str().ok_or("register omitted an object ID")?;
                let (planned_name, mut object) = if let Some(parent) = item.get("nested") {
                    let parent_name = matched
                        .get(text(&parent["instance"]))
                        .ok_or("registration omitted the enclosing instance")?;
                    let child = nested
                        .iter()
                        .find(|child| {
                            &child.parent == parent_name
                                && child.name.kind == name.kind
                                && child.local == text(&parent["name"])
                        })
                        .ok_or("registration returned an unplanned nested object")?;
                    renamed.insert(child.name.clone(), name.clone());
                    (child.name.clone(), child.resolved.clone())
                } else if let Some(object) = objects.get(&name) {
                    (name.clone(), object.clone())
                } else {
                    let index = defaults
                        .iter()
                        .position(|(prefix, _)| {
                            name.kind == "Conduit"
                                && name
                                    .name
                                    .strip_prefix(prefix)
                                    .is_some_and(|suffix| suffix.starts_with('-'))
                        })
                        .ok_or("registration returned an object without a planned definition")?;
                    (name.clone(), defaults.remove(index).1)
                };
                object.id = id.into();
                object.revision = item["revision"]
                    .as_str()
                    .ok_or("register omitted an object revision")?
                    .into();
                matched.insert(id, planned_name);
                updated.insert(name, object);
            }
            if !defaults.is_empty() || objects.keys().any(|name| !updated.contains_key(name)) {
                return Err("registration omitted a planned object".into());
            }
            for (name, mut object) in updated {
                let planned_name = &matched[object.id.as_str()];
                names.retain(|_, known| known != planned_name);
                names.insert(object.id.clone(), planned_name.clone());
                if !renamed.is_empty() {
                    object.definition = rename_refs(&object.definition, &renamed);
                }
                ledger.objects.insert(name, object);
            }
        }
        Effect::Replace(wanted) => {
            let mut object = (**wanted).clone();
            object.id = step
                .expected
                .clone()
                .ok_or("replace has no expected object")?;
            object.revision = result["revision"]
                .as_str()
                .ok_or("replace omitted the revision")?
                .into();
            ledger.objects.insert(step.subject.clone(), object);
        }
        Effect::RegisterReplacement {
            wanted,
            parent,
            field,
            slot,
            previous,
        } => {
            let item = &result["object"];
            let name = Name::object(item).ok_or("replacement omitted the object")?;
            let mut object = (**wanted).clone();
            object.id = item["id"]
                .as_str()
                .ok_or("replacement omitted the ID")?
                .into();
            object.revision = item["revision"]
                .as_str()
                .ok_or("replacement omitted the revision")?
                .into();
            object.state = "registered".into();
            names.retain(|_, known| known != previous);
            names.insert(object.id.clone(), previous.clone());
            let reference = name.reference();
            ledger.objects.insert(name, object);
            if let Some(parent) = ledger.objects.get_mut(parent) {
                parent.definition["slots"][*field][slot] = reference;
            }
        }
        Effect::SetLifecycle { state, opened_by } => {
            let subject = ["object", "instance", "conduit"]
                .into_iter()
                .find_map(|field| Name::object(&args[field]));
            if let Some(object) = ledger
                .objects
                .get_mut(subject.as_ref().unwrap_or(&step.subject))
            {
                object.state = (*state).into();
                if let Some((actor, route)) = opened_by {
                    object.definition["provenance"] = json!({"actor":actor,"sentBy":actor,
                        "invocation":result["invocation"],"openedThrough":[route]});
                }
            }
        }
        Effect::Remove => {
            ledger.objects.remove(&step.subject);
            if let Some(id) = &step.expected {
                names.remove(id);
            }
        }
        Effect::Retire {
            closes_on_dispatch,
            closes_on_removal,
        } => {
            let removed = result["outcome"] == "succeeded";
            if removed {
                ledger.objects.remove(&step.subject);
            } else if let Some(object) = ledger.objects.get_mut(&step.subject) {
                object.state = "retired".into();
            }
            let closes = closes_on_dispatch
                .iter()
                .chain(closes_on_removal.iter().filter(|_| removed));
            for closed in closes {
                if let Some(object) = ledger.objects.remove(closed) {
                    names.remove(&object.id);
                }
            }
            if removed && let Some(id) = &step.expected {
                names.remove(id);
            }
        }
        Effect::Tombstones => ledger.tombstones = array(&result["tombstones"]).to_vec(),
        Effect::Lift => {
            let remaining: BTreeSet<_> = array(&result["remaining"])
                .iter()
                .filter_map(|stone| stone["id"].as_str())
                .collect();
            let mut lifted = BTreeSet::from([text(&args["id"]).to_owned()]);
            loop {
                let descendents: Vec<_> = ledger
                    .tombstones
                    .iter()
                    .filter(|stone| {
                        lifted.contains(text(&stone["cause"]["tombstone"]))
                            && !lifted.contains(text(&stone["id"]))
                    })
                    .map(|stone| text(&stone["id"]).to_owned())
                    .collect();
                if descendents.is_empty() {
                    break;
                }
                lifted.extend(descendents);
            }
            ledger.tombstones.retain(|stone| {
                let id = text(&stone["id"]);
                !lifted.contains(id) || remaining.contains(id)
            });
        }
    }
    Ok(())
}

fn rename_refs(value: &Value, renamed: &BTreeMap<Name, Name>) -> Value {
    match value {
        Value::Object(fields) => {
            if fields.len() == 2
                && let Some(name) = Name::object(value)
                && let Some(actual) = renamed.get(&name)
            {
                return actual.reference();
            }
            Value::Object(
                fields
                    .iter()
                    .map(|(key, child)| (key.clone(), rename_refs(child, renamed)))
                    .collect(),
            )
        }
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|child| rename_refs(child, renamed))
                .collect(),
        ),
        _ => value.clone(),
    }
}

fn timestamp_free_state(ledger: &Ledger) -> Value {
    let mut objects: Vec<_> = ledger.objects.iter().collect();
    objects.sort_by_key(|(name, _)| {
        (
            rank(
                &name.kind,
                &["Pool", "Resource", "Agent", "Fleet", "Interface", "Conduit"],
            ),
            &name.name,
        )
    });
    json!({
        "format":1,"files":ledger.files,"plans":ledger.plans,"journals":ledger.journals,
        "journalUpdate":ledger.journal_update,
        "tombstones":safe(&json!(ledger.tombstones)),
        "objects":objects.into_iter().map(|(name, object)| json!([
            name.kind,name.name,object.id,object.revision,object.state,
            safe(&object.definition),safe(&json!(object.packages))
        ])).collect::<Vec<_>>()
    })
}

pub(super) fn project_hashes(project: &Path) -> Result<Value> {
    let mut files = serde_json::Map::new();
    for entry in fs::read_dir(project).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if conduit_loader::FileRole::of(&name) != conduit_loader::FileRole::NotRead
            && entry.path().is_file()
        {
            let bytes = fs::read(entry.path()).map_err(|e| format!("cannot hash {name}: {e}"))?;
            files.insert(
                name,
                json!(bytes_digest(&bytes).trim_start_matches("sha256:")),
            );
        }
    }
    Ok(Value::Object(files))
}

fn directory(path: &Path, display: &str) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.is_dir() => Ok(()),
        Err(e) if e.kind() == ErrorKind::NotFound => fs::DirBuilder::new()
            .mode(0o700)
            .create(path)
            .and_then(|()| fs::File::open(path.parent().unwrap())?.sync_all())
            .map_err(|e| format!("{display}: {e}")),
        Ok(_) => Err(format!("{display}: expected a directory")),
        Err(e) => Err(format!("{display}: {e}")),
    }
}

fn write(directory: &Path, name: &str, bytes: &[u8], display: &str) -> Result<()> {
    store::atomic_write(directory, name, bytes)
        .map_err(|_| format!("{display}/{name}: atomic write failed"))
}

fn next_attempt(journal: &Value) -> Result<u64> {
    let attempt = journal["attempt"]
        .as_u64()
        .filter(|n| *n != 0)
        .ok_or("invalid attempt")?;
    let entries = journal["entries"].as_array().ok_or("invalid entries")?;
    if entries.last().is_some_and(|entry| {
        entry["verificationFailed"].is_string()
            || entry
                .get("result")
                .is_some_and(|result| outcome(result) != "succeeded")
    }) {
        attempt
            .checked_add(1)
            .ok_or("attempt counter exhausted".into())
    } else {
        Ok(attempt)
    }
}

fn not_sent(from: usize, count: usize) {
    if from <= count {
        println!("not sent {from}..{count}");
    }
}

fn local_failure(error: &str, from: usize, count: usize) -> i32 {
    eprintln!("conduit apply: {}", user_config::escape_line_breaks(error));
    not_sent(from, count);
    2
}

pub(super) async fn execute(
    project: &Path,
    plan: &Plan,
    mut ledger: Ledger,
    mut send: impl AsyncFnMut(&str, Value, &str) -> Result<Value>,
) -> i32 {
    let plan_text = plan.text();
    print!("{plan_text}");
    let count = plan.steps.len();
    if !plan.stops.is_empty() {
        not_sent(1, count);
        return 2;
    }
    if count == 0 {
        return 0;
    }
    if let Some(index) = plan
        .steps
        .iter()
        .position(|step| matches!(step.action.effect, Effect::Unspecified))
    {
        return local_failure(
            &format!("step {}: planner omitted the ledger effect", index + 1),
            1,
            count,
        );
    }
    if let Err(error) = std::io::stdout().flush() {
        return local_failure(&format!("stdout: {error}"), 1, count);
    }
    let base = project.join(".conduit");
    let plans = base.join("plans");
    let reports = base.join("results");
    let digest = bytes_digest(plan_text.as_bytes());
    let hex = digest.trim_start_matches("sha256:");
    let report_name = format!("{hex}.yaml");
    let journal = reports.join(&report_name);
    let files = match project_hashes(project) {
        Ok(files) => files,
        Err(error) => return local_failure(&error, 1, count),
    };
    let save_ledger = |state: &Value| {
        write(
            &base,
            "state.yaml",
            state.to_string().as_bytes(),
            ".conduit",
        )
    };
    let prepare = || -> Result<(u64, Vec<Value>)> {
        directory(&base, ".conduit")?;
        directory(&plans, ".conduit/plans")?;
        directory(&reports, ".conduit/results")?;
        let (attempt, prior_removals) = match fs::symlink_metadata(&journal) {
            Err(e) if e.kind() == ErrorKind::NotFound => (1, vec![]),
            Ok(meta) if meta.is_file() => {
                let prior =
                    crate::boundary::value(&journal).map_err(|_| "cannot read kept journal")?;
                let pending = |entry: &Value| {
                    ledger
                        .replacements
                        .contains_key(text(&entry["result"]["invocation"]))
                };
                let old_files = prior.get("files").unwrap_or(&ledger.files);
                let removals = array(&prior["priorRemovals"])
                    .iter()
                    .filter(|saved| pending(&saved["entry"]))
                    .cloned()
                    .chain(
                        array(&prior["entries"])
                            .iter()
                            .filter(|entry| pending(entry))
                            .map(|entry| json!({"entry":entry,"files":old_files})),
                    )
                    .collect();
                (next_attempt(&prior)?, removals)
            }
            _ => return Err("journal is not a readable regular file".into()),
        };
        write(
            &plans,
            &format!("{hex}.txt"),
            plan_text.as_bytes(),
            ".conduit/plans",
        )?;
        Ok((attempt, prior_removals))
    };
    let (attempt, prior_removals) = match prepare() {
        Ok(prepared) => prepared,
        Err(error) => return local_failure(&error, 1, count),
    };
    let save = |attempt, entries: &[Value], update: Option<&Value>| {
        let mut report = json!({"attempt":attempt,"entries":entries,"files":files});
        if !prior_removals.is_empty() {
            report["priorRemovals"] = json!(prior_removals);
        }
        if let Some(update) = update {
            report["ledgerUpdate"] = update.clone();
        }
        write(
            &reports,
            &report_name,
            report.to_string().as_bytes(),
            ".conduit/results",
        )
    };
    // The old journal may be this attempt's only durable proof of a prior update.
    if ledger.recovered_update
        && let Err(error) = save_ledger(&timestamp_free_state(&ledger))
    {
        return local_failure(&error, 1, count);
    }
    if !ledger.plans.contains(&json!(digest)) {
        ledger.plans.push(json!(digest));
    }
    ledger.journals.insert(
        digest.clone(),
        json!(format!(".conduit/results/{report_name}")),
    );
    ledger.files = files.clone();
    let mut kept_state = timestamp_free_state(&ledger);
    if let Err(error) = save(attempt, &[], None).and_then(|()| save_ledger(&kept_state)) {
        return local_failure(&error, 1, count);
    }
    let mut entries: Vec<Value> = Vec::new();
    let mut results = Vec::new();
    let mut shown = BTreeSet::new();
    let mut names = plan.names.clone();
    let mut listed_tombstones = None;
    for (index, step) in plan.steps.iter().enumerate() {
        let action = &step.action;
        let number = index + 1;
        let key = format!("plan:{hex}:{attempt}:{number}");
        let args = match step_arguments(step, action, &results) {
            Ok(args) => args,
            Err(BindingError::TombstoneMismatch { listed, cause }) => {
                if let Some(entry) = entries.last_mut() {
                    entry["verificationFailed"] = json!("/tombstones");
                }
                if let Err(error) = save(attempt, &entries, None) {
                    return local_failure(&error, number, count);
                }
                println!("verify {number} failed {} place=/tombstones", step.subject);
                eprintln!("conduit apply: lift step {number} read list step {listed}: {cause}");
                not_sent(number, count);
                return 1;
            }
            Err(BindingError::Invalid(error)) => {
                return local_failure(&format!("step {number}: {error}"), number, count);
            }
        };
        if let Some(listed) = listed_tombstones
            && let Some(stone) = ledger.tombstones.iter().find(|stone| {
                action
                    .tombstone_candidates()
                    .into_iter()
                    .any(|(name, definition)| compare::matches_tombstone(name, definition, stone))
            })
        {
            if let Some(entry) = entries.last_mut() {
                entry["verificationFailed"] = json!("/tombstones");
            }
            if let Err(error) = save(attempt, &entries, None) {
                return local_failure(&error, number, count);
            }
            println!("verify {number} failed {} place=/tombstones", step.subject);
            eprintln!(
                "conduit apply: tombstone {} from list step {listed} still matches step {number}",
                user_config::escape_line_breaks(text(&stone["id"]))
            );
            not_sent(number, count);
            return 1;
        }
        let mut entry = json!({"step":number,"tool":step.tool,"arguments":safe(&args),"key":key,
            "flag":approval_text(&action.approvals)});
        entry["remakes"] = json!(
            action
                .remakes
                .iter()
                .map(Name::reference)
                .collect::<Vec<_>>()
        );
        if !action.replacement_slots.is_empty() {
            entry["replacementSlots"] = json!(
                action
                    .replacement_slots
                    .iter()
                    .map(|(name, slot)| slot.kept(name))
                    .collect::<Vec<_>>()
            );
        }
        if let Effect::RegisterReplacement { previous, .. } = &action.effect {
            entry["replaces"] = previous.reference();
        }
        entries.push(entry);
        if let Err(error) = save(attempt, &entries, None) {
            return local_failure(&error, number, count);
        }
        let raw = match send(&step.tool, args.clone(), &key).await {
            Ok(raw) => raw,
            Err(error) => {
                println!("result {number} unknown {{}}");
                eprintln!(
                    "conduit apply: {} at step {number} with key {key}",
                    user_config::escape_line_breaks(&error)
                );
                not_sent(number + 1, count);
                return 1;
            }
        };
        let result = projection(&raw);
        let outcome = outcome(&result).to_owned();
        println!("result {number} {outcome} {result}");
        if outcome == "succeeded" && action.panel {
            for id in array(&result["pendingAction"])
                .iter()
                .filter_map(Value::as_str)
            {
                if shown.insert(id.to_owned()) {
                    println!("panel action {id}: sign-in required");
                }
            }
        }
        let failed_check = action
            .check
            .as_ref()
            .and_then(|check| {
                if outcome == "succeeded" {
                    verify(
                        &raw["structuredContent"],
                        &args["expectedId"],
                        check,
                        &names,
                    )
                } else if outcome == "conflict"
                    && array(&result["diagnostics"]).iter().any(|diagnostic| {
                        diagnostic["ruleId"] == "LIF-037"
                            && diagnostic["properties"]["expectedId"] == args["expectedId"]
                            && diagnostic["properties"]["actualId"].as_str().is_some()
                            && diagnostic["properties"]["actualId"] != args["expectedId"]
                    })
                {
                    Some("/id".into())
                } else {
                    None
                }
            })
            .or_else(|| {
                (matches!(action.effect, Effect::Lift)
                    && plan.steps[index + 1..]
                        .iter()
                        .flat_map(|later| later.action.tombstone_candidates())
                        .any(|(name, definition)| {
                            array(&result["remaining"])
                                .iter()
                                .any(|stone| compare::matches_tombstone(name, definition, stone))
                        }))
                .then(|| "/remaining".into())
            });
        entries[index]["result"] = result;
        if let Some(place) = &failed_check {
            entries[index]["verificationFailed"] = json!(place);
        }
        let partial_retirement = matches!(action.effect, Effect::Retire { .. })
            && dispatched_unsuccessful_retirement(&entries[index]["result"]);
        if partial_retirement {
            entries[index]["retirementDispatched"] = json!(true);
        }
        let update = if (outcome == "succeeded" || partial_retirement)
            && (failed_check.is_none() || matches!(action.effect, Effect::Lift))
            && !matches!(action.effect, Effect::None)
        {
            let changed = apply_effect(
                &mut ledger,
                &mut names,
                step,
                &action.effect,
                &entries[index]["result"],
                &args,
            )
            .and_then(|()| {
                ledger.journal_update = ledger
                    .journal_update
                    .checked_add(1)
                    .ok_or("ledger update counter exhausted")?;
                Ok(
                    json!({"from":bytes_digest(kept_state.to_string().as_bytes()),
                    "state":timestamp_free_state(&ledger)}),
                )
            });
            match changed {
                Ok(update) => Some(update),
                Err(error) => {
                    let saved = save(attempt, &entries, None);
                    return local_failure(&saved.err().unwrap_or(error), number + 1, count);
                }
            }
        } else {
            None
        };
        if let Err(error) = save(attempt, &entries, update.as_ref()) {
            return local_failure(&error, number + 1, count);
        }
        if let Some(mut update) = update {
            kept_state = update["state"].take();
            if let Err(error) = save_ledger(&kept_state) {
                return local_failure(&error, number + 1, count);
            }
        }
        if let Some(place) = failed_check {
            println!("verify {number} failed {} place={place}", step.subject);
            not_sent(number + 1, count);
            return 1;
        }
        if outcome != "succeeded" {
            not_sent(number + 1, count);
            return 1;
        }
        if matches!(action.effect, Effect::Tombstones) {
            listed_tombstones = Some(number);
        }
        results.push(entries[index]["result"].clone());
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct Schedule {
        actions: Vec<Action>,
        names: BTreeMap<String, Name>,
    }

    fn no_effect() -> Action {
        Action {
            effect: Effect::None,
            ..Action::default()
        }
    }

    async fn execute_fixture(
        project: &Path,
        plan: &Plan,
        schedule: &Schedule,
        ledger: Ledger,
        send: impl AsyncFnMut(&str, Value, &str) -> Result<Value>,
    ) -> i32 {
        let mut plan = plan.clone();
        assert_eq!(plan.steps.len(), schedule.actions.len());
        for (step, action) in plan.steps.iter_mut().zip(&schedule.actions) {
            step.action = action.clone();
        }
        plan.names.clone_from(&schedule.names);
        execute(project, &plan, ledger, send).await
    }

    fn step(tool: &str) -> Step {
        Step {
            tool: tool.into(),
            subject: Name::new("Resource", "sample"),
            expected: None,
            relative: Expected::None,
            class: "neither",
            rule: "LIF-013",
            arguments: json!({"object":{"kind":"Resource","name":"sample"}}),
            details: vec![],
            action: no_effect(),
        }
    }

    fn response(outcome: &str, outputs: Value) -> Value {
        json!({"_meta":{format!("{META}outcome"):outcome},"structuredContent":outputs})
    }

    fn journal(directory: &Path, plan: &Plan) -> Value {
        let digest = plan.digest();
        let hex = digest.trim_start_matches("sha256:");
        crate::boundary::value(&directory.join(format!(".conduit/results/{hex}.yaml"))).unwrap()
    }

    #[tokio::test]
    async fn review_248_lift_remaining_and_fresh_list_use_grant_matching() {
        for remaining in [false, true] {
            let directory = tempfile::tempdir().unwrap();
            let name = Name::new("Conduit", "new-grant");
            let definition = json!({
                "consumer":{"kind":"Agent","name":"a"},"target":{"kind":"Resource","name":"new"},
                "as":"files","resolved":{"capability":"example.org/files#files.v1"}
            });
            let blocker = json!({
                "id":"blocking-stone","object":{"kind":"Conduit","name":"different","id":"different-id"},
                "consumer":{"kind":"Agent","name":"a"},"target":{"kind":"Resource","name":"old"},
                "localName":"files","capability":"example.org/files#files.v1"
            });
            let mut lift = step("fixture.lift");
            lift.arguments = json!({"id":"planned-stone"});
            let plan = Plan {
                steps: vec![step("fixture.list"), lift, step("fixture.open")],
                ..Plan::default()
            };
            let schedule = Schedule {
                actions: vec![
                    Action {
                        effect: Effect::Tombstones,
                        ..no_effect()
                    },
                    Action {
                        effect: Effect::Lift,
                        ..no_effect()
                    },
                    Action {
                        opens: Some((name, definition)),
                        ..no_effect()
                    },
                ],
                names: BTreeMap::new(),
            };
            let code = execute_fixture(
                directory.path(),
                &plan,
                &schedule,
                Ledger::default(),
                async |tool, _, _| match tool {
                    "fixture.list" => Ok(response(
                        "succeeded",
                        json!({"tombstones":[blocker.clone()]}),
                    )),
                    "fixture.lift" => Ok(response(
                        "succeeded",
                        json!({"remaining":if remaining { vec![blocker.clone()] } else { vec![] }}),
                    )),
                    _ => panic!("a known tombstone must stop the open"),
                },
            )
            .await;
            assert_eq!(code, 1);
            let journal = journal(directory.path(), &plan);
            assert_eq!(array(&journal["entries"]).len(), 2);
            assert_eq!(
                journal["entries"][1]["verificationFailed"],
                if remaining {
                    "/remaining"
                } else {
                    "/tombstones"
                }
            );
        }
    }

    #[tokio::test]
    async fn review_248_unconfirmed_retirement_keeps_its_partial_effect() {
        let directory = tempfile::tempdir().unwrap();
        let mut retire = step("fixture.retire.v1");
        let id = "00000000-0000-4000-8000-000000000001";
        retire.expected = Some(id.into());
        let target = Name::new("Conduit", "target");
        let consumer = Name::new("Conduit", "consumer");
        let observation = |id: &str, definition| ObservedObject {
            id: id.into(),
            revision: "r1".into(),
            state: "ready".into(),
            definition,
            packages: vec![],
            status: Value::Null,
            unknown_endings: BTreeMap::new(),
            inventory: Value::Null,
            implementations: Value::Null,
            made_by_engine: false,
        };
        let ledger = Ledger {
            objects: BTreeMap::from([
                (retire.subject.clone(), observation(id, json!({}))),
                (
                    target.clone(),
                    observation(
                        "00000000-0000-4000-8000-000000000002",
                        json!({"target":retire.subject.reference()}),
                    ),
                ),
                (
                    consumer.clone(),
                    observation(
                        "00000000-0000-4000-8000-000000000003",
                        json!({"consumer":retire.subject.reference()}),
                    ),
                ),
            ]),
            ..Ledger::default()
        };
        let plan = Plan {
            steps: vec![retire, step("fixture.must_not_send")],
            ..Plan::default()
        };
        let schedule = Schedule {
            actions: vec![
                Action {
                    effect: Effect::Retire {
                        closes_on_dispatch: BTreeSet::from([target.clone()]),
                        closes_on_removal: BTreeSet::from([consumer.clone()]),
                    },
                    approvals: BTreeSet::from([Approval::new("--replace", "sample")]),
                    remakes: BTreeSet::from([plan.steps[0].subject.clone()]),
                    ..no_effect()
                },
                no_effect(),
            ],
            names: BTreeMap::new(),
        };
        let state = directory.path().join(".conduit/state.yaml");
        let before_reply = RefCell::new(None);
        let outcome = execute_fixture(
            directory.path(),
            &plan,
            &schedule,
            ledger,
            async |tool, _, _| {
                assert_eq!(tool, "fixture.retire.v1");
                before_reply.replace(Some(fs::read(&state).unwrap()));
                let mut reply = response("unknown", json!({}));
                reply["_meta"][format!("{META}invocation")] = json!("retirement-invocation");
                Ok(reply)
            },
        )
        .await;
        assert_eq!(outcome, 1);
        let assert_partial = |kept: &Ledger| {
            assert_eq!(kept.objects[&plan.steps[0].subject].state, "retired");
            assert!(!kept.objects.contains_key(&target));
            assert!(kept.objects.contains_key(&consumer));
        };
        assert_partial(&Ledger::read(&state).unwrap());
        fs::write(&state, before_reply.into_inner().unwrap()).unwrap();
        let recovered = Ledger::read(&state).unwrap();
        assert!(recovered.recovered_update);
        assert_partial(&recovered);
        let code = execute_fixture(
            directory.path(),
            &plan,
            &schedule,
            recovered,
            async |tool, _, _| {
                assert_eq!(tool, "fixture.retire.v1");
                Err("scripted reply loss".into())
            },
        )
        .await;
        assert_eq!(code, 1);
        let after_retry = Ledger::read(&state).unwrap();
        assert_partial(&after_retry);
        assert!(
            after_retry
                .pending_replacement(&plan.steps[0].subject)
                .is_some()
        );
    }

    #[tokio::test]
    async fn apply_journals_each_step_before_sending_it() {
        let directory = tempfile::tempdir().unwrap();
        let resource = Name::new("Resource", "sample");
        let declared_child = Name::new("Conduit", "sample--declared");
        let object = ObservedObject {
            id: "draft".into(),
            revision: "draft".into(),
            state: "registered".into(),
            definition: json!({"slots":{"conduits":{"child":declared_child.reference()}}}),
            packages: vec![],
            status: Value::Null,
            unknown_endings: BTreeMap::new(),
            inventory: Value::Null,
            implementations: Value::Null,
            made_by_engine: false,
        };
        let plan = Plan {
            steps: vec![step("future.publish"), step("future.observe")],
            ..Plan::default()
        };
        let schedule = Schedule {
            actions: vec![
                Action {
                    effect: Effect::Register {
                        objects: BTreeMap::from([(resource.clone(), object.clone())]),
                        defaults: vec![],
                        nested: vec![NestedRegistration {
                            parent: resource,
                            local: "child".into(),
                            name: declared_child,
                            resolved: ObservedObject {
                                definition: json!({}),
                                ..object
                            },
                        }],
                    },
                    ..no_effect()
                },
                Action {
                    binding: Some(Binding::Object {
                        from: 1,
                        field: "object",
                    }),
                    ..no_effect()
                },
            ],
            names: BTreeMap::new(),
        };
        let seen = RefCell::new(Vec::new());
        let id = "00000000-0000-4000-8000-000000000001";
        let result = execute_fixture(
            directory.path(),
            &plan,
            &schedule,
            Ledger::default(),
            async |tool, args, key| {
                let mut seen = seen.borrow_mut();
                let index = seen.len();
                let saved = journal(directory.path(), &plan);
                assert_eq!(saved["entries"][index]["tool"], tool);
                assert_eq!(saved["entries"][index]["key"], key);
                assert!(saved["entries"][index].get("result").is_none());
                seen.push(tool.to_owned());
                if index == 0 {
                    Ok(response(
                        "succeeded",
                        json!({"objects":[
                            {"kind":"Resource","name":"sample","id":id,"revision":"revision-1"},
                            {"kind":"Conduit","name":"sample--actual","id":"child-id",
                                "revision":"revision-2","nested":{"instance":id,"name":"child"}}
                        ]}),
                    ))
                } else {
                    assert_eq!(saved["entries"][0]["result"]["outcome"], "succeeded");
                    assert_eq!(args["expectedId"], id);
                    Ok(response("succeeded", json!({})))
                }
            },
        )
        .await;
        assert_eq!(result, 0);
        assert_eq!(seen.into_inner(), ["future.publish", "future.observe"]);
        assert_eq!(
            journal(directory.path(), &plan)["entries"][1]["result"]["outcome"],
            "succeeded"
        );
        let kept = crate::boundary::value(&directory.path().join(".conduit/state.yaml")).unwrap();
        assert_eq!(
            kept["objects"][0][5]["slots"]["conduits"]["child"]["name"],
            "sample--actual"
        );
        assert_eq!(kept["objects"][1][2], "child-id");
        assert_eq!(kept["objects"][1][3], "revision-2");
    }

    // K3 safety cases 6 and 7: an ID or definition changed before verification.
    #[tokio::test]
    async fn apply_stops_on_a_failed_read_check() {
        let directory = tempfile::tempdir().unwrap();
        let expected = ObservedObject {
            id: "expected".into(),
            revision: "revision".into(),
            state: "registered".into(),
            definition: json!({"resolved":{"config":{"setting":1}}}),
            packages: vec![],
            status: Value::Null,
            unknown_endings: BTreeMap::new(),
            inventory: Value::Null,
            implementations: Value::Null,
            made_by_engine: false,
        };
        let mut read = step("future.observe");
        read.arguments["expectedId"] = json!("expected");
        let plan = Plan {
            steps: vec![read, step("future.activate")],
            ..Plan::default()
        };
        let schedule = Schedule {
            actions: vec![
                Action {
                    check: Some(expected.clone()),
                    ..no_effect()
                },
                no_effect(),
            ],
            names: BTreeMap::new(),
        };
        for (raw, pointer) in [
            (json!({"id":"other"}), "/id"),
            (
                json!({"id":"expected","declared":{"config":{"setting":2}}}),
                "/declared/resolved/config/setting",
            ),
            (
                json!({"id":"expected","declared":{"config":{"setting":1}},"packages":[{"address":"example.org/other"}]}),
                "/packages",
            ),
        ] {
            assert_eq!(
                verify(&raw, &json!("expected"), &expected, &schedule.names).as_deref(),
                Some(pointer)
            );
            let code = execute_fixture(
                directory.path(),
                &plan,
                &schedule,
                Ledger::default(),
                async |tool, _, _| {
                    assert_eq!(tool, "future.observe");
                    Ok(response("succeeded", raw.clone()))
                },
            )
            .await;
            assert_eq!(code, 1);
        }
    }

    #[tokio::test]
    async fn apply_attempt_rises_only_after_an_unsuccessful_step() {
        let directory = tempfile::tempdir().unwrap();
        let plan = Plan {
            steps: vec![step("future.mutate")],
            ..Plan::default()
        };
        let schedule = Schedule {
            actions: vec![no_effect()],
            names: BTreeMap::new(),
        };
        let keys = RefCell::new(Vec::new());
        for (returned, code) in [
            (None, 1),
            (Some("succeeded"), 0),
            (Some("refused"), 1),
            (Some("succeeded"), 0),
        ] {
            let actual = execute_fixture(
                directory.path(),
                &plan,
                &schedule,
                Ledger::default(),
                async |_, _, key| {
                    keys.borrow_mut().push(key.to_owned());
                    match returned {
                        None => Err("no response".into()),
                        Some("refused") => Ok(json!({"_meta":{format!("{META}reason"):"refused"}})),
                        Some(outcome) => Ok(response(outcome, json!({}))),
                    }
                },
            )
            .await;
            assert_eq!(actual, code);
        }
        let keys = keys.into_inner();
        assert!(keys[..3].iter().all(|key| key == &keys[0]));
        assert!(keys[3].ends_with(":2:1"));
    }

    fn registered(definition: Value) -> ObservedObject {
        ObservedObject {
            id: "00000000-0000-4000-8000-000000000001".into(),
            revision: "revision-1".into(),
            state: "registered".into(),
            definition,
            packages: vec![],
            status: Value::Null,
            unknown_endings: BTreeMap::new(),
            inventory: Value::Null,
            implementations: Value::Null,
            made_by_engine: false,
        }
    }

    #[tokio::test]
    async fn fix_247_recovers_a_registration_with_a_kept_reply() {
        let directory = tempfile::tempdir().unwrap();
        let file = ledger_at(directory.path());
        let mut publish = step("fixture.publish");
        let object = registered(json!({}));
        publish.action = Action {
            approvals: BTreeSet::from([Approval::new("--replace", "original-child")]),
            effect: Effect::Register {
                objects: BTreeMap::from([(publish.subject.clone(), object.clone())]),
                defaults: vec![],
                nested: vec![],
            },
            ..no_effect()
        };
        let plan = Plan {
            steps: vec![publish],
            ..Plan::default()
        };
        let before = RefCell::new(Vec::new());
        let code = execute(directory.path(), &plan, Ledger::default(), async |_, _, _| {
            let entry = journal(directory.path(), &plan);
            assert!(entry["entries"][0].get("result").is_none());
            assert_eq!(entry["entries"][0]["flag"], "--replace original-child");
            *before.borrow_mut() = fs::read(&file).unwrap();
            fs::remove_file(&file).unwrap();
            fs::create_dir(&file).unwrap();
            Ok(response("succeeded", json!({"objects":[{"kind":"Resource","name":"sample","id":object.id,"revision":"live-revision"}]})))
        }).await;
        assert_eq!(code, 2);
        assert_eq!(
            journal(directory.path(), &plan)["entries"][0]["result"]["outcome"],
            "succeeded"
        );
        fs::remove_dir(&file).unwrap();
        fs::write(&file, before.into_inner()).unwrap();
        let recovered = Ledger::read(&file).unwrap();
        assert_eq!(
            recovered
                .objects
                .get(&plan.steps[0].subject)
                .map(|o| (&*o.id, &*o.revision)),
            Some((object.id.as_str(), "live-revision")),
            "a durable registration reply must recover ownership"
        );
        assert_eq!(
            approval_text(&recovered.registration_flags[&object.id]).as_deref(),
            Some("--replace original-child")
        );
        assert!(recovered.recovered_update);
        fs::write(&file, timestamp_free_state(&recovered).to_string()).unwrap();
        assert!(!Ledger::read(&file).unwrap().recovered_update);
    }

    #[tokio::test]
    async fn fix_247_missing_effect_stops_before_the_first_step() {
        let directory = tempfile::tempdir().unwrap();
        let mut unplanned = step("fixture.unspecified");
        unplanned.action = Action::default();
        let plan = Plan {
            steps: vec![step("fixture.valid"), unplanned],
            ..Plan::default()
        };
        let code = execute(
            directory.path(),
            &plan,
            Ledger::default(),
            async |_, _, _| {
                panic!("a plan without all ledger effects must not dispatch");
            },
        )
        .await;
        assert_eq!(code, 2);
        assert!(!directory.path().join(".conduit").exists());
    }

    #[test]
    fn fix_247_verifies_nested_slot_identity() {
        let child = Name::new("Conduit", "sample--child");
        let object = registered(json!({"slots":{"conduits":{"child":child.reference()}}}));
        let old_id = json!(object.id);
        let names = BTreeMap::from([("known-child".into(), child)]);
        let mut raw = json!({"id":old_id,"declared":{"slots":{"conduits":{"child":{"kind":"Conduit","id":"known-child"}}}}});
        assert_eq!(verify(&raw, &old_id, &object, &names), None);
        raw["declared"]["slots"]["conduits"]["child"]["id"] = json!("other-child");
        assert_eq!(
            verify(&raw, &old_id, &object, &names).as_deref(),
            Some("/declared/slots/conduits/child"),
            "changed nested slot must fail verification"
        );
    }

    #[tokio::test]
    async fn fix_247_verifies_live_id_conflict() {
        let directory = tempfile::tempdir().unwrap();
        let mut read = step("fixture.observe");
        read.arguments["expectedId"] = json!("expected");
        let plan = Plan {
            steps: vec![read, step("fixture.activate")],
            ..Plan::default()
        };
        let schedule = Schedule {
            actions: vec![
                Action {
                    check: Some(registered(json!({}))),
                    ..no_effect()
                },
                no_effect(),
            ],
            names: BTreeMap::new(),
        };
        let code = execute_fixture(
            directory.path(),
            &plan,
            &schedule,
            Ledger::default(),
            async |tool, _, _| {
                assert_eq!(tool, "fixture.observe");
                Ok(json!({
                    "_meta":{format!("{META}reason"):"conflict"},
                    "structuredContent":{"diagnostics":[{"ruleId":"LIF-037",
                        "properties":{"expectedId":"expected","actualId":"changed"}}]}
                }))
            },
        )
        .await;
        assert_eq!(code, 1);
        let entry = &journal(directory.path(), &plan)["entries"][0];
        assert_eq!(entry["result"]["reason"], "conflict");
        assert_eq!(
            entry["verificationFailed"], "/id",
            "live LIF-037 conflict must use the verification failure path"
        );
    }

    #[test]
    fn fix_247_checks_tombstone_cause() {
        let action = Action {
            binding: Some(Binding::Tombstone {
                closed: 1,
                listed: 2,
                object: "previous".into(),
                prior_cause: None,
            }),
            ..no_effect()
        };
        let close = json!({"invocation":"this-close"});
        let mut listed = json!({"tombstones":[{"id":"stone","object":{"id":"previous"},"cause":{"invocation":"another-close"}}]});
        assert!(
            step_arguments(
                &step("fixture.lift"),
                &action,
                &[close.clone(), listed.clone()]
            )
            .is_err(),
            "another close's tombstone must not be lifted"
        );
        listed["tombstones"][0]["cause"]["invocation"] = json!("this-close");
        let args = step_arguments(&step("fixture.lift"), &action, &[close, listed]).unwrap();
        assert_eq!(args["id"], "stone");

        let binding = &step("fixture.lift");
        let close = json!({"invocation":"this-close"});
        assert!(matches!(
            step_arguments(binding, &action, std::slice::from_ref(&close)),
            Err(BindingError::Invalid("missing tombstone result"))
        ));
        let empty = json!({"tombstones":[]});
        assert!(matches!(
            step_arguments(binding, &action, &[close, empty]),
            Err(BindingError::TombstoneMismatch { listed: 2, cause }) if cause.ends_with("found 0")
        ));
    }

    #[tokio::test]
    async fn fix_247_advances_attempt_after_verification_failure() {
        let directory = tempfile::tempdir().unwrap();
        let mut read = step("fixture.observe");
        read.arguments["expectedId"] = json!("expected");
        let plan = Plan {
            steps: vec![read],
            ..Plan::default()
        };
        let schedule = Schedule {
            actions: vec![Action {
                check: Some(registered(json!({}))),
                ..no_effect()
            }],
            names: BTreeMap::new(),
        };
        for (id, expected_code) in [("changed", 1), ("expected", 0)] {
            let code = execute_fixture(
                directory.path(),
                &plan,
                &schedule,
                Ledger::default(),
                async |_, _, _| Ok(response("succeeded", json!({"id":id}))),
            )
            .await;
            assert_eq!(code, expected_code);
        }
        let report = journal(directory.path(), &plan);
        assert_eq!(
            report["attempt"], 2,
            "the same plan must use a fresh inspection key after failed verification"
        );
    }

    #[test]
    fn fix_247_reads_a_flag_with_its_operand() {
        let directory = tempfile::tempdir().unwrap();
        let base = directory.path().join(".conduit");
        fs::create_dir_all(base.join("results")).unwrap();
        let hex = "a".repeat(64);
        let digest = format!("sha256:{hex}");
        let object = registered(json!({}));
        let file = ledger_at(directory.path());
        let ledger = Ledger {
            plans: vec![json!(digest)],
            journals: serde_json::Map::from_iter([(
                digest,
                json!(format!(".conduit/results/{hex}.yaml")),
            )]),
            files: json!({}),
            ..Ledger::default()
        };
        fs::write(&file, timestamp_free_state(&ledger).to_string()).unwrap();
        fs::write(base.join(format!("results/{hex}.yaml")), json!({
            "attempt":1,"entries":[{"flag":"--replace original-child","result":{"object":{"id":object.id}}}]
        }).to_string()).unwrap();
        let actual = Ledger::read(&file).expect("a kept approval includes its operand");
        assert_eq!(
            approval_text(&actual.registration_flags[&object.id]).as_deref(),
            Some("--replace original-child")
        );
    }

    #[test]
    fn fix_247_updates_the_parent_slot_from_replacement_reply() {
        let parent = Name::new("Resource", "sample");
        let old_child = Name::new("Conduit", "sample--old");
        let new_child = Name::new("Conduit", "sample--new");
        let mut ledger = Ledger::default();
        ledger.objects.insert(
            parent.clone(),
            registered(json!({"slots":{"conduits":{"child":old_child.reference()}}})),
        );
        let mut replacement = step("fixture.replace_child");
        replacement.arguments = json!({"instance":parent.reference(),"nested":"child"});
        apply_effect(&mut ledger, &mut BTreeMap::new(), &replacement,
            &Effect::RegisterReplacement {
                wanted: Box::new(registered(json!({}))), parent: parent.clone(),
                field: "conduits", slot: "child".into(), previous: old_child,
            },
            &json!({"object":{"kind":new_child.kind,"name":new_child.name,"id":"actual-child","revision":"new-revision"}}),
            &replacement.arguments,
        ).unwrap();
        assert_eq!(ledger.objects[&new_child].id, "actual-child");
        assert_eq!(ledger.objects[&new_child].revision, "new-revision");
        assert_eq!(
            ledger.objects[&parent].definition["slots"]["conduits"]["child"],
            new_child.reference(),
            "the enclosing instance must keep the returned child identity"
        );
    }
}
