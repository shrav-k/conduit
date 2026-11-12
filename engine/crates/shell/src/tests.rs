#[path = "fixtures.rs"]
mod fixtures;
use super::*;
use conduit_core::access::Identity;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

fn temporary() -> tempfile::TempDir {
    let temp = tempfile::tempdir().unwrap();
    fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o700)).unwrap();
    temp
}
fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn context() -> Context {
    boundary::context(&repository()).unwrap()
}

#[test]
fn every_engine_operation_is_executable_or_unfinished_with_its_order() {
    // MAN-012: the shell accounts for every operation implemented by the core's engine item.
    let context = context();
    let manifest = context.builtin.as_ref().unwrap().manifest.as_ref().unwrap();
    let prefix = format!("{}#", context.core_package);
    let engine: std::collections::BTreeSet<_> = manifest["implementations"]["engine"]["implements"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry.as_str().unwrap().strip_prefix(&prefix).unwrap())
        .collect();
    let mut listed = BTreeMap::new();
    let mut faults = Vec::new();
    for (operation, list) in server::EXECUTABLE_ENGINE_OPERATIONS
        .iter()
        .map(|operation| (*operation, "EXECUTABLE_ENGINE_OPERATIONS"))
        .chain(
            server::UNFINISHED_ENGINE_OPERATIONS
                .iter()
                .map(|(operation, _)| (*operation, "UNFINISHED_ENGINE_OPERATIONS")),
        )
    {
        if let Some(previous) = listed.insert(operation, list) {
            faults.push(format!("{operation}: duplicate in {previous} and {list}"));
        }
        if !engine.contains(operation) {
            faults.push(format!(
                "{operation}: stale in {list}; absent from core engine"
            ));
        }
    }
    for operation in engine {
        if !listed.contains_key(operation) {
            faults.push(format!(
                "{operation}: missing from EXECUTABLE_ENGINE_OPERATIONS and UNFINISHED_ENGINE_OPERATIONS"
            ));
        }
    }
    for (operation, order) in server::UNFINISHED_ENGINE_OPERATIONS {
        let bytes = order.as_bytes();
        let numbered = (bytes.len() == 3 || bytes.len() == 4)
            && bytes[..3].iter().all(u8::is_ascii_digit)
            && (bytes.len() == 3 || bytes[3].is_ascii_lowercase());
        if !numbered && order != "none yet: ROADMAP.md S3, AUDIT.md A35" {
            faults.push(format!(
                "{operation}: invalid order {order:?} in UNFINISHED_ENGINE_OPERATIONS"
            ));
        }
    }
    assert!(faults.is_empty(), "{}", faults.join("\n"));
}

fn open(path: &Path) -> Shell {
    Shell::open(path, context(), Config::default()).unwrap()
}

#[test]
fn private_state_directory_is_empty_distinct_and_stable_after_restart() {
    // PRV-029: instance IDs, rather than names or run IDs, select the directory.
    let temp = temporary();
    let shell = open(temp.path());
    shell
        .submit(
            json!({"kind":"first-start","machine":local_machine(BTreeMap::new()).unwrap()}),
            &MissingExecutor,
        )
        .unwrap();
    let ids: Vec<String> = shell
        .state()
        .unwrap()
        .objects
        .iter()
        .filter(|o| o.get("spec").is_none())
        .take(2)
        .map(|o| o["id"].as_str().unwrap().into())
        .collect();
    let path = shell.state_directory(&ids[0]).unwrap();
    assert_eq!(path.file_name().unwrap(), ids[0].as_str());
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o700
    );
    assert_eq!(fs::read_dir(&path).unwrap().count(), 0);
    let other = shell.state_directory(&ids[1]).unwrap();
    assert_ne!(path, other);
    fs::write(path.join("sentinel"), "synthetic state").unwrap();
    assert_eq!(shell.state_directory(&ids[0]).unwrap(), path);
    assert_eq!(fs::read_dir(&other).unwrap().count(), 0);
    assert!(shell.state_directory("../escape").is_err());
    drop(shell);
    let shell = open(temp.path());
    assert_eq!(shell.state_directory(&ids[0]).unwrap(), path);
    assert_eq!(
        fs::read_to_string(path.join("sentinel")).unwrap(),
        "synthetic state"
    );
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(shell.state_directory(&ids[0]).is_err());
}
fn case() -> Value {
    boundary::value(&repository().join("conformance/conventions/s1/whole/case.yaml")).unwrap()
}

fn expand(v: &Value, symbols: &BTreeMap<String, Value>, core: &str) -> Value {
    match v {
        Value::String(s) if s.starts_with('$') => symbols.get(&s[1..]).unwrap_or(v).clone(),
        Value::String(s) if s.starts_with("@core#") => json!(format!("{core}#{}", &s[6..])),
        Value::Array(a) => json!(
            a.iter()
                .map(|v| expand(v, symbols, core))
                .collect::<Vec<_>>()
        ),
        Value::Object(m) => Value::Object(
            m.iter()
                .map(|(k, v)| (k.clone(), expand(v, symbols, core)))
                .collect(),
        ),
        _ => v.clone(),
    }
}
fn fixture_input(raw: &Value, shell: &Shell, symbols: &BTreeMap<String, Value>) -> Value {
    let mut input = expand(raw, symbols, &shell.context.core_package);
    input.as_object_mut().unwrap().remove("evidence");
    if input["kind"] == "first-start" {
        input.as_object_mut().unwrap().remove("trustDomain");
    }
    if input["kind"] == "resolve-project" || input["kind"] == "register-project" {
        input.as_object_mut().unwrap().remove("file");
        input.as_object_mut().unwrap().remove("reverse");
        let mut snapshots = fixtures::snapshots(&repository()).unwrap();
        for s in &mut snapshots {
            s.project.directory = None;
        }
        input["snapshots"] = json!(snapshots);
        input["builtin"] = json!(shell.context.builtin);
        let spec = conduit_loader::Specification::read(&repository().join("spec")).unwrap();
        let mut project =
            boundary::project(&repository().join("conformance/fixtures/s1/project"), &spec)
                .unwrap();
        project.directory = None;
        input["project"] = json!(project);
    }
    input
}

fn project_assert(actual: &Value, expected: &Value) {
    match expected {
        Value::Object(map) if !map.is_empty() => {
            for (k, v) in map {
                project_assert(
                    actual
                        .get(k)
                        .unwrap_or_else(|| panic!("missing {k} in {actual}")),
                    v,
                );
            }
        }
        Value::Array(a) => {
            let b = actual.as_array().unwrap();
            assert_eq!(a.len(), b.len(), "array lengths");
            for (b, a) in b.iter().zip(a) {
                project_assert(b, a);
            }
        }
        _ => assert_eq!(actual, expected),
    }
}

fn projection_matches(actual: &Value, expected: &Value) -> bool {
    match expected {
        Value::Object(map) if !map.is_empty() => map
            .iter()
            .all(|(k, v)| actual.get(k).is_some_and(|a| projection_matches(a, v))),
        Value::Array(values) => actual.as_array().is_some_and(|a| {
            a.len() == values.len() && a.iter().zip(values).all(|(a, e)| projection_matches(a, e))
        }),
        _ => actual == expected,
    }
}

struct Scripted {
    replies: Mutex<VecDeque<Value>>,
    directory: PathBuf,
    dispatched: AtomicUsize,
}
impl Scripted {
    fn answer(&self, intent: &Value) -> Result<Execution> {
        // Every executor sees the already-fsynced invocation and dispatch record.
        let stored: Stored =
            serde_json::from_slice(&fs::read(self.directory.join("state.json")).unwrap()).unwrap();
        assert!(
            stored
                .state
                .invocations
                .iter()
                .any(|v| v["id"] == intent["invocation"] && v["state"] == "in-progress")
        );
        self.dispatched.fetch_add(1, Ordering::SeqCst);
        let next = self
            .replies
            .lock()
            .unwrap()
            .pop_front()
            .expect("script exhausted");
        Ok(if next["kind"] == "complete" {
            Execution::Complete(next["completion"].clone())
        } else {
            Execution::SetupReport(next["report"].clone())
        })
    }
}
impl Executor for Scripted {
    fn dispatch(&self, intent: &Value) -> Result<Execution> {
        self.answer(intent)
    }
    fn refresh_tools_and_call(&self, intent: &Value) -> Result<Execution> {
        self.answer(intent)
    }
    fn builtin_runner(&self, intent: &Value, identity: Option<&RunIdentity>) -> Result<Execution> {
        assert_eq!(
            identity.is_some(),
            intent["operation"].as_str().unwrap().ends_with("#run.v1")
        );
        self.answer(intent)
    }
    fn provider_call(&self, intent: &Value, _: Option<&RunIdentity>) -> Result<Execution> {
        self.answer(intent)
    }
    fn connect(&self, intent: &Value) -> Result<Execution> {
        self.answer(intent)
    }
}

/// Establish S1 through setup using the same fixture evidence, then use OS evidence.
fn established(path: &Path) -> Shell {
    let shell = open(path);
    let case = case();
    let steps = case["steps"].as_array().unwrap();
    let mut symbols = BTreeMap::new();
    for step in &steps[..13] {
        let input = fixture_input(&step["input"], &shell, &symbols);
        if let Some(evidence) = step["input"].get("evidence") {
            shell.evidence.lock().unwrap().push_back(evidence.clone());
        }
        let committed = shell.commit(input).unwrap();
        let t = shell.trace.lock().unwrap().last().unwrap().clone();
        if let Some(capture) = step["capture"].as_object() {
            let value = serde_json::to_value(&t).unwrap();
            for (k, p) in capture {
                symbols.insert(
                    k.clone(),
                    value.pointer(p.as_str().unwrap()).unwrap().clone(),
                );
            }
        }
        // This fixture helper deliberately supplies the next report itself, as the vector
        // does. The full-loop test below sends every intent through its scripted executor.
        drop(committed);
    }
    shell.evidence.lock().unwrap().clear();
    shell.trace.lock().unwrap().clear();
    shell
}
fn actor(shell: &Shell, name: &str) -> String {
    shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|v| v["name"] == name && v.get("spec").is_none())
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .into()
}
fn invoke(shell: &Shell, name: &str, tool: &str, key: Option<&str>) -> Value {
    json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":actor(shell,name)},"tool":tool,"arguments":{"path":"hello.txt","content":"test write"}},"key":key})
}

#[test]
fn s1_whole_through_store_loop_and_scripted_executor() {
    let temp = temporary();
    let shell = open(temp.path());
    shell.trace.lock().unwrap().clear();
    let case = case();
    let steps = case["steps"].as_array().unwrap();
    let replies = steps
        .iter()
        .filter(|s| {
            matches!(
                s["input"]["kind"].as_str(),
                Some("complete" | "setup-report")
            )
        })
        .map(|s| s["input"].clone())
        .collect();
    let executor = Scripted {
        replies: Mutex::new(replies),
        directory: temp.path().into(),
        dispatched: AtomicUsize::new(0),
    };
    *shell.evidence.lock().unwrap() = steps
        .iter()
        .filter_map(|s| s["input"].get("evidence").cloned())
        .collect();
    let mut symbols = BTreeMap::new();
    let mut expected_records = Vec::new();
    let mut index = 0;
    while index < steps.len() {
        let input = fixture_input(&steps[index]["input"], &shell, &symbols);
        let ticket = shell
            .begin(input, &executor)
            .unwrap_or_else(|e| panic!("S1 step {index}: {e}"));
        assert!(
            ticket.try_reply().unwrap().is_some(),
            "pending must not escape the full loop"
        );
        let trace = std::mem::take(&mut *shell.trace.lock().unwrap());
        assert!(!trace.is_empty());
        for transition in trace {
            let step = &steps[index];
            let actual = serde_json::to_value(&transition).unwrap();
            if let Some(captures) = step["capture"].as_object() {
                for (name, pointer) in captures {
                    symbols.insert(
                        name.clone(),
                        actual.pointer(pointer.as_str().unwrap()).unwrap().clone(),
                    );
                }
            }
            let expected = expand(&step["expect"], &symbols, &shell.context.core_package);
            if let Some(records) = expected.get("records") {
                project_assert(&json!(transition.records), records);
            }
            if let Some(facts) = expected["record_facts"].as_array() {
                let mut cursor = 0;
                for fact in facts {
                    let found = transition.records[cursor..]
                        .iter()
                        .position(|r| projection_matches(r, fact))
                        .expect("missing ordered S1 record fact");
                    cursor += found + 1;
                }
            }
            if let Some(reply) = expected.get("reply") {
                project_assert(&transition.reply, reply);
            }
            expected_records.extend(transition.records);
            index += 1;
        }
    }
    assert!(executor.replies.lock().unwrap().is_empty());
    assert!(executor.dispatched.load(Ordering::SeqCst) > 10);
    let exported = shell
        .submit(json!({"kind":"export-records"}), &MissingExecutor)
        .unwrap();
    assert_eq!(exported["records"], json!(expected_records));
    assert!(
        shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .all(|v| v["state"] != "in-progress")
    );
}

struct Deferred;
impl Executor for Deferred {
    fn dispatch(&self, _: &Value) -> Result<Execution> {
        Ok(Execution::Deferred)
    }
}

#[test]
fn pending_waiters_concurrent_repeats_and_stale_completions() {
    let temp = temporary();
    let shell = Arc::new(established(temp.path()));
    let input = invoke(&shell, "agent-b", "project.file.write.v1", Some("same-key"));
    let first = shell.begin(input.clone(), &Deferred).unwrap();
    assert!(first.try_reply().unwrap().is_none());
    let mut threads = vec![];
    for _ in 0..8 {
        let shell = shell.clone();
        let input = input.clone();
        threads.push(std::thread::spawn(move || {
            shell.begin(input, &Deferred).unwrap()
        }));
    }
    let tickets: Vec<_> = threads.into_iter().map(|t| t.join().unwrap()).collect();
    assert!(
        tickets
            .iter()
            .all(|t| t.try_reply().unwrap().unwrap()["_meta"]
                [format!("{}reason", shell.context.meta_prefix)]
                == "conflict")
    );
    let id = first.invocation().unwrap().to_string();
    assert_eq!(
        shell
            .state()
            .unwrap()
            .records
            .iter()
            .filter(|r| r["type"] == "invocation-dispatched" && r["data"]["invocation"] == id)
            .count(),
        1
    );
    let waiter = std::thread::spawn(move || first.wait().unwrap());
    shell
        .complete(
            &id,
            1,
            json!({"kind":"result","result":{"content":[{"type":"text","text":"written"}]}}),
            &MissingExecutor,
        )
        .unwrap();
    let reply = waiter.join().unwrap();
    assert_eq!(reply["isError"], false);
    shell
        .complete(&id, 1, json!({"kind":"missing"}), &MissingExecutor)
        .unwrap();
    assert_eq!(
        shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .find(|v| v["id"] == id)
            .unwrap()["state"],
        "succeeded"
    );
}

#[test]
fn missing_executor_is_reported_to_core() {
    let temp = temporary();
    let shell = established(temp.path());
    let reply = shell
        .submit(
            invoke(&shell, "agent-b", "project.file.write.v1", None),
            &MissingExecutor,
        )
        .unwrap();
    assert_eq!(
        reply["_meta"][format!("{}outcome", shell.context.meta_prefix)],
        "unknown"
    );
    let run = json!({"kind":"builtin-runner","operation":format!("{}#inspect-run.v1",shell.context.core_package)});
    assert!(
        matches!(missing(&run,&shell.context).unwrap(),Execution::SetupReport(v) if v["state"] == "unknown")
    );
    assert!(missing(&json!({"kind":"unknown-intent"}), &shell.context).is_err());
}

#[test]
fn retention_cap_oldest_first_and_age_survive_restart() {
    let temp = temporary();
    let mut shell = established(temp.path());
    let executor = Scripted {
        replies: Mutex::new(VecDeque::from([
            json!({"kind":"complete","completion":{"kind":"result","result":{"content":[{"type":"text","text":"a".repeat(1000)}]}}}),
            json!({"kind":"complete","completion":{"kind":"result","result":{"content":[{"type":"text","text":"b".repeat(1000)}]}}}),
        ])),
        directory: temp.path().into(),
        dispatched: AtomicUsize::new(0),
    };
    let first = shell
        .submit(
            invoke(&shell, "agent-b", "project.file.write.v1", Some("older")),
            &executor,
        )
        .unwrap();
    let id = first["_meta"][format!("{}invocation", shell.context.meta_prefix)]
        .as_str()
        .unwrap()
        .to_string();
    let original = shell.state().unwrap();
    let retained_size = serde_json::to_vec(
        &original.invocations.iter().find(|v| v["id"] == id).unwrap()["retained"],
    )
    .unwrap()
    .len();
    shell.config.max_output_bytes = retained_size + 128;
    shell
        .submit(
            invoke(&shell, "agent-b", "project.file.write.v1", Some("newer")),
            &executor,
        )
        .unwrap();
    let state = shell.state().unwrap();
    assert_eq!(state.invocations.len(), original.invocations.len() + 1);
    assert!(
        state
            .invocations
            .iter()
            .find(|v| v["id"] == id)
            .unwrap()
            .get("retained")
            .is_none()
    );
    assert!(state.invocations.last().unwrap().get("retained").is_some());
    let reply = shell
        .submit(
            invoke(&shell, "agent-b", "project.file.write.v1", Some("older")),
            &MissingExecutor,
        )
        .unwrap();
    assert_eq!(
        reply["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "conflict"
    );
    let record_count = shell.state().unwrap().records.len();
    drop(shell);
    let config = Config {
        max_output_age: Duration::ZERO,
        ..Config::default()
    };
    let shell = Shell::open(temp.path(), context(), config).unwrap();
    assert!(
        shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .all(|v| v.get("retained").is_none())
    );
    assert_eq!(shell.state().unwrap().records.len(), record_count);
}

// Test-only observed implementation, like the conformance runner's aware fixtures.
// Requests, completions and expiry under test still go through the shell/core loop.
fn filtered_shell(path: &Path) -> Shell {
    let shell = established(path);
    let writer = shell.writer.lock().unwrap();
    let mut stored = writer.store.read().unwrap();
    let mut filtered_paths = 0;
    for object in &mut stored.state.objects {
        if object["kind"] == "Conduit" && object["nested"]["name"] == "project" {
            object["requirements"]["deadline"] = filter();
            filtered_paths += 1;
        }
        if let Some(implementations) = object
            .get_mut("resolved_implementations")
            .and_then(Value::as_object_mut)
        {
            for implementation in implementations.values_mut() {
                if implementation["item"]["plain"] == true {
                    implementation["item"]["plain"] = json!(false);
                }
            }
        }
        for pointer in ["/schema_environment/scopes", "/held/environment/scopes"] {
            if let Some(scopes) = object.pointer_mut(pointer).and_then(Value::as_object_mut) {
                for scope in scopes.values_mut() {
                    if let Some(contract) = scope["contracts"].get_mut("file.read.v1") {
                        contract["spec"]["outputSchema"] = json!({"type":"object"});
                    }
                }
            }
        }
    }
    assert_eq!(filtered_paths, 2);
    let ids: Vec<_> = stored
        .state
        .invocations
        .iter()
        .map(|v| v["id"].clone())
        .collect();
    // Clear the fixture's setup outputs through the core, before testing retention.
    for id in ids {
        let transition = shell
            .transition(
                &stored.state,
                json!({"kind":"expire-outputs","invocation":id}),
            )
            .unwrap();
        shell.persist(&writer, &mut stored, &transition).unwrap();
    }
    drop(writer);
    shell
}
fn filter() -> Value {
    json!({"requirement":"example.org/requirements#deadline.v1","arguments":{"until":"2026-11-09T00:00:00Z"}})
}
fn filtered_read(shell: &Shell, key: &str) -> Value {
    let mut request = invoke(shell, "agent-b", "project.file.read.v1", Some(key));
    request["request"]["arguments"] = json!({"path":"hello.txt"});
    request
}
fn filter_refusal(shell: &Shell) -> Value {
    json!({"kind":"result","result":{"isError":true,"_meta":{
        format!("{}reason",shell.context.meta_prefix):"refused",
        format!("{}restriction",shell.context.meta_prefix):filter()
    }}})
}
fn complete_read(shell: &Shell, ticket: &Ticket, attempt: u64, text: &str) {
    shell
        .complete(
            ticket.invocation().unwrap(),
            attempt,
            json!({"kind":"result","result":{
                "isError":false,"structuredContent":{"text":text},
                "_meta":{format!("{}outcome",shell.context.meta_prefix):"succeeded"}
            }}),
            &MissingExecutor,
        )
        .unwrap();
    assert_eq!(
        ticket.try_reply().unwrap().unwrap()["structuredContent"]["text"],
        text
    );
}
fn assert_filter_reply(shell: &Shell, ticket: &Ticket, completion: &Ticket) {
    let actual = ticket
        .try_reply()
        .unwrap()
        .expect("filtered caller must be released");
    assert_eq!(actual, completion.try_reply().unwrap().unwrap());
    assert_eq!(actual["isError"], true);
    assert_eq!(
        actual["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "refused"
    );
    assert_eq!(
        actual["_meta"][format!("{}restriction", shell.context.meta_prefix)],
        filter()
    );
    assert!(
        actual["_meta"]
            .get(format!("{}outcome", shell.context.meta_prefix))
            .is_none()
    );
    assert!(actual.get("structuredContent").is_none());
}

#[test]
fn first_filter_rejection_delivers_core_reply_to_ticket() {
    let temp = temporary();
    let shell = filtered_shell(temp.path());
    let ticket = shell
        .begin(filtered_read(&shell, "first"), &Deferred)
        .unwrap();
    let id = ticket.invocation().unwrap();
    assert_eq!(
        shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .find(|v| v["id"] == id)
            .unwrap()["filters"],
        json!([filter()])
    );
    let completed = shell
        .complete(id, 1, filter_refusal(&shell), &MissingExecutor)
        .unwrap();
    assert_filter_reply(&shell, &ticket, &completed);
    assert!(
        !shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .any(|v| v["id"] == id)
    );
}

#[test]
fn rejected_repeat_delivers_refusal_instead_of_previous_success() {
    let temp = temporary();
    let shell = filtered_shell(temp.path());
    let request = filtered_read(&shell, "repeat");
    let first = shell.begin(request.clone(), &Deferred).unwrap();
    complete_read(&shell, &first, 1, "previous success");
    let repeated = shell.begin(request, &Deferred).unwrap();
    assert!(repeated.try_reply().unwrap().is_none());
    let completed = shell
        .complete(
            repeated.invocation().unwrap(),
            2,
            filter_refusal(&shell),
            &MissingExecutor,
        )
        .unwrap();
    assert_filter_reply(&shell, &repeated, &completed);
    assert_eq!(first.try_reply().unwrap().unwrap()["isError"], false);
    let state = shell.state().unwrap();
    let restored = state
        .invocations
        .iter()
        .find(|v| v["id"] == repeated.invocation().unwrap())
        .unwrap();
    assert_eq!(restored["state"], "succeeded");
    assert_eq!(restored["retained"]["outputs"]["text"], "previous success");
}

#[test]
fn retention_cap_includes_previous_outputs_during_pending_repeat() {
    let temp = temporary();
    let mut shell = filtered_shell(temp.path());
    let request = filtered_read(&shell, "older");
    let first = shell.begin(request.clone(), &Deferred).unwrap();
    complete_read(&shell, &first, 1, &"a".repeat(1000));
    let state = shell.state().unwrap();
    let retained = &state
        .invocations
        .iter()
        .find(|v| v["id"] == first.invocation().unwrap())
        .unwrap()["retained"];
    shell.config.max_output_bytes = serde_json::to_vec(retained).unwrap().len() + 128;
    let repeated = shell.begin(request, &Deferred).unwrap();
    let other = shell
        .begin(filtered_read(&shell, "newer"), &Deferred)
        .unwrap();
    complete_read(&shell, &other, 1, &"b".repeat(1000));
    let state = shell.state().unwrap();
    let pending = state
        .invocations
        .iter()
        .find(|v| v["id"] == repeated.invocation().unwrap())
        .unwrap();
    assert_eq!(pending["state"], "in-progress");
    assert_eq!(pending["previous"]["state"], "succeeded");
    assert!(pending["previous"].get("retained").is_none());
    assert!(repeated.try_reply().unwrap().is_none());
    assert!(
        state
            .invocations
            .iter()
            .find(|v| v["id"] == other.invocation().unwrap())
            .unwrap()
            .get("retained")
            .is_some()
    );
    let completed = shell
        .complete(
            repeated.invocation().unwrap(),
            2,
            filter_refusal(&shell),
            &MissingExecutor,
        )
        .unwrap();
    assert_filter_reply(&shell, &repeated, &completed);
    let state = shell.state().unwrap();
    let restored = state
        .invocations
        .iter()
        .find(|v| v["id"] == repeated.invocation().unwrap())
        .unwrap();
    assert_eq!(restored["state"], "succeeded");
    assert!(restored.get("retained").is_none());
}

#[test]
fn retention_preserves_age_on_rejection_and_expires_pending_previous_outputs() {
    let temp = temporary();
    let mut shell = filtered_shell(temp.path());
    let request = filtered_read(&shell, "aged");
    let first = shell.begin(request.clone(), &Deferred).unwrap();
    complete_read(&shell, &first, 1, "old result");
    let id = first.invocation().unwrap();
    // Adjust only the shell's clock metadata, never core invocation state.
    let original = (Utc::now() - chrono::Duration::days(6)).to_rfc3339();
    {
        let writer = shell.writer.lock().unwrap();
        let mut stored = writer.store.read().unwrap();
        stored.output_times.insert(id.into(), original.clone());
        writer.store.write(&stored).unwrap();
    }
    let repeated = shell.begin(request.clone(), &Deferred).unwrap();
    let completed = shell
        .complete(id, 2, filter_refusal(&shell), &MissingExecutor)
        .unwrap();
    assert_filter_reply(&shell, &repeated, &completed);
    assert_eq!(
        shell
            .writer
            .lock()
            .unwrap()
            .store
            .read()
            .unwrap()
            .output_times[id],
        original
    );
    let repeated = shell.begin(request, &Deferred).unwrap();
    shell.config.max_output_age = Duration::from_secs(5 * 24 * 60 * 60);
    shell
        .submit(json!({"kind":"export-records"}), &MissingExecutor)
        .unwrap();
    let state = shell.state().unwrap();
    let pending = state.invocations.iter().find(|v| v["id"] == id).unwrap();
    assert_eq!(pending["state"], "in-progress");
    assert!(pending["previous"].get("retained").is_none());
    let completed = shell
        .complete(id, 3, filter_refusal(&shell), &MissingExecutor)
        .unwrap();
    assert_filter_reply(&shell, &repeated, &completed);
    let state = shell.state().unwrap();
    let restored = state.invocations.iter().find(|v| v["id"] == id).unwrap();
    assert_eq!(restored["state"], "succeeded");
    assert!(restored.get("retained").is_none());
    assert!(
        !shell
            .writer
            .lock()
            .unwrap()
            .store
            .read()
            .unwrap()
            .output_times
            .contains_key(id)
    );
}

fn descriptor(shell: &Shell, instance: &str) -> Value {
    let run = &shell.context.core_environment.scopes["core"].contracts["run.v1"]["spec"]["inputSchema"]
        ["properties"]["identity"]["properties"];
    json!({"materializeFor":instance,"environmentNames":run["environment"]["required"],"fileNames":run["files"]["required"]})
}

#[test]
fn identities_verify_revoke_and_never_enter_state_records_or_output() {
    use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
    use p256::{
        ecdsa::{
            Signature, SigningKey, VerifyingKey,
            signature::{Signer, Verifier},
        },
        pkcs8::DecodePrivateKey,
    };
    use x509_parser::prelude::{FromDer, X509Certificate};
    let temp = temporary();
    let shell = established(temp.path());
    let id = actor(&shell, "agent-a");
    let desc = descriptor(&shell, &id);
    let identity = shell.materialize_identity(&desc).unwrap();
    let files = desc["fileNames"].as_array().unwrap();
    let token = fs::read_to_string(identity.directory.join(files[3].as_str().unwrap())).unwrap();
    assert!(
        matches!(shell.verify_token(Some(&token)).unwrap(),Identity::Instance { id: got } if got == id)
    );
    assert!(matches!(shell.verify_token(None).unwrap(), Identity::None));
    assert!(matches!(
        shell.verify_token(Some("broken")).unwrap(),
        Identity::Unreadable
    ));
    assert!(matches!(
        shell.authority.verify(
            Some(&token),
            "http://127.0.0.1:9999/other",
            &shell.state().unwrap()
        ),
        Identity::Unreadable
    ));
    let parts: Vec<_> = token.split('.').collect();
    let jwks: Value = serde_json::from_slice(
        &fs::read(identity.directory.join(files[1].as_str().unwrap())).unwrap(),
    )
    .unwrap();
    let mut public = vec![4];
    public.extend(B64.decode(jwks["keys"][0]["x"].as_str().unwrap()).unwrap());
    public.extend(B64.decode(jwks["keys"][0]["y"].as_str().unwrap()).unwrap());
    let verifying = VerifyingKey::from_sec1_bytes(&public).unwrap();
    verifying
        .verify(
            format!("{}.{}", parts[0], parts[1]).as_bytes(),
            &Signature::from_slice(&B64.decode(parts[2]).unwrap()).unwrap(),
        )
        .unwrap();
    let certificate = fs::read(identity.directory.join(files[0].as_str().unwrap())).unwrap();
    let (_, pem) = x509_parser::pem::parse_x509_pem(&certificate).unwrap();
    let (_, certificate) = X509Certificate::from_der(&pem.contents).unwrap();
    assert_eq!(
        certificate.public_key().subject_public_key.data.as_ref(),
        public
    );
    assert_eq!(certificate.issuer(), certificate.subject());
    assert!(certificate.basic_constraints().unwrap().unwrap().value.ca);
    verifying
        .verify(
            certificate.tbs_certificate.as_ref(),
            &Signature::from_der(certificate.signature_value.data.as_ref()).unwrap(),
        )
        .unwrap();
    let claims: Value = serde_json::from_slice(&B64.decode(parts[1]).unwrap()).unwrap();
    let issued = DateTime::from_timestamp(claims["iat"].as_i64().unwrap(), 0).unwrap();
    assert_eq!(
        claims["exp"].as_i64().unwrap(),
        issued
            .checked_add_months(chrono::Months::new(1200))
            .unwrap()
            .timestamp()
    );
    let engine_id =
        fs::read_to_string(identity.directory.join(files[2].as_str().unwrap())).unwrap();
    let random_domain = engine_id
        .strip_prefix("spiffe://")
        .unwrap()
        .strip_suffix("/conduit/engine")
        .unwrap();
    assert_eq!(random_domain.len(), 64);
    assert!(
        random_domain
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
    );
    let authority: Value =
        serde_json::from_slice(&fs::read(temp.path().join("authority.json")).unwrap()).unwrap();
    let key = SigningKey::from_pkcs8_pem(authority["key"].as_str().unwrap()).unwrap();
    // Signed but expired, missing or malformed expiration claims must all fail.
    for expiration in [
        Some(json!(Utc::now().timestamp() - 1)),
        None,
        Some(json!("later")),
    ] {
        let mut invalid = claims.clone();
        if let Some(exp) = expiration {
            invalid["exp"] = exp;
        } else {
            invalid.as_object_mut().unwrap().remove("exp");
        }
        let signed = format!(
            "{}.{}",
            parts[0],
            B64.encode(serde_json::to_vec(&invalid).unwrap())
        );
        let signature: Signature = key.sign(signed.as_bytes());
        let token = format!("{signed}.{}", B64.encode(signature.to_bytes()));
        assert!(matches!(
            shell.verify_token(Some(&token)).unwrap(),
            Identity::Unreadable
        ));
    }
    let mut other_domain = claims.clone();
    other_domain["sub"] = json!(format!("spiffe://other.conduit.local/instance/{id}"));
    let signing_input = format!(
        "{}.{}",
        parts[0],
        B64.encode(serde_json::to_vec(&other_domain).unwrap())
    );
    let signature: Signature = key.sign(signing_input.as_bytes());
    let other_domain = format!("{signing_input}.{}", B64.encode(signature.to_bytes()));
    assert!(matches!(
        shell.verify_token(Some(&other_domain)).unwrap(),
        Identity::Unreadable
    ));
    assert!(
        claims["sub"]
            .as_str()
            .unwrap()
            .ends_with(&format!("/instance/{id}"))
    );
    let forged = format!(
        "{}.{}.{}",
        B64.encode(br#"{"alg":"none"}"#),
        parts[1],
        parts[2]
    );
    assert!(matches!(
        shell.verify_token(Some(&forged)).unwrap(),
        Identity::Unreadable
    ));
    let mut removed = shell.state().unwrap();
    removed.objects.retain(|v| v["id"] != id);
    assert!(matches!(
        shell
            .authority
            .verify(Some(&token), &shell.config.engine_url, &removed),
        Identity::Unreadable
    ));
    for suffix in ["/extra", "?query", "#fragment"] {
        let wrong = shell
            .authority
            .token(&format!("{id}{suffix}"), &shell.config.engine_url)
            .unwrap();
        assert!(matches!(
            shell.verify_token(Some(&wrong)).unwrap(),
            Identity::Unreadable
        ));
    }
    let state = fs::read_to_string(temp.path().join("state.json")).unwrap();
    let output = shell
        .submit(json!({"kind":"export-records"}), &MissingExecutor)
        .unwrap()
        .to_string();
    for file in files {
        let path = identity.directory.join(file.as_str().unwrap());
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
        let secret = fs::read_to_string(path).unwrap();
        assert!(!state.contains(&secret));
        assert!(!output.contains(&secret));
        // JSON escaping must not hide a leaked PEM/JWK value.
        let escaped = serde_json::to_string(&secret).unwrap();
        assert!(!state.contains(&escaped));
        assert!(!output.contains(&escaped));
    }
    for value in identity.environment.values() {
        assert!(!state.contains(value));
        assert!(!output.contains(value));
    }
    assert!(!state.contains(claims["jti"].as_str().unwrap()));
    assert!(!output.contains(claims["jti"].as_str().unwrap()));
    assert_eq!(
        fs::metadata(&identity.directory)
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    assert_eq!(
        fs::metadata(temp.path().join("authority.json"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    let authority = fs::read(temp.path().join("authority.json")).unwrap();
    drop(shell);
    let reopened = open(temp.path());
    assert!(matches!(
        reopened.verify_token(Some(&token)).unwrap(),
        Identity::Instance { .. }
    ));
    assert_eq!(
        authority,
        fs::read(temp.path().join("authority.json")).unwrap()
    );
}

#[test]
fn store_rejects_versions_permissions_and_competing_owners() {
    let temp = temporary();
    let shell = open(temp.path());
    assert!(Shell::open(temp.path(), context(), Config::default()).is_err());
    assert_eq!(
        fs::metadata(temp.path().join("state.json"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    drop(shell);
    fs::remove_file(temp.path().join("writer.lock")).unwrap();
    std::os::unix::fs::symlink(
        temp.path().join("missing-target"),
        temp.path().join("writer.lock"),
    )
    .unwrap();
    assert!(Shell::open(temp.path(), context(), Config::default()).is_err());
    assert!(!temp.path().join("missing-target").exists());
    fs::remove_file(temp.path().join("writer.lock")).unwrap();
    store::atomic_write(temp.path(), "state.json", br#"{"version":999}"#).unwrap();
    assert!(
        Shell::open(temp.path(), context(), Config::default())
            .err()
            .unwrap()
            .contains("version")
    );
    fs::set_permissions(temp.path(), fs::Permissions::from_mode(0o755)).unwrap();
    assert!(Shell::open(temp.path(), context(), Config::default()).is_err());
}

#[test]
fn store_roundtrips_document_depth_plus_state_wrappers() {
    let temp = temporary();
    let store = Store::open(temp.path()).unwrap();
    let mut stored = Stored::default();
    let mut value = json!("deep document value");
    for _ in 0..140 {
        value = json!({"child":value});
    }
    stored.state.objects.push(value);
    store.write(&stored).unwrap();
    assert_eq!(stored.state, store.read().unwrap().state);
}

#[test]
fn evidence_retry_allocates_large_registration_without_partial_commit() {
    let temp = temporary();
    let shell = open(temp.path());
    shell
        .submit(
            json!({"kind":"first-start","machine":local_machine(BTreeMap::new()).unwrap()}),
            &MissingExecutor,
        )
        .unwrap();
    let before = shell.state().unwrap();
    let documents:Vec<_> = (0..150).map(|n|json!({"apiVersion":shell.context.templates["system"]["apiVersion"],"kind":"Pool","metadata":{"name":format!("pool-template-{n}")},"spec":{"type":format!("{}#pool.v1",shell.context.core_package)}})).collect();
    let input = json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,"project":{"manifest":null,"documents":documents,"directory":null},"request":{"sender":before.objects[4]["id"],"caller":before.objects[4]["id"],"conduit":before.objects[5]["id"],"target":before.objects[2]["id"],"operation":format!("{}#register.v1",shell.context.core_package)}});
    let reply = shell.submit(input, &MissingExecutor).unwrap();
    assert_eq!(reply["accepted"], true);
    let after = shell.state().unwrap();
    assert_eq!(after.objects.len(), 158);
    assert!(after.allocated_ids.len() - before.allocated_ids.len() > 256);
    assert_eq!(
        after
            .records
            .iter()
            .filter(|r| r["data"]["action"] == "registered")
            .count(),
        158
    );
    assert!(
        after
            .allocated_ids
            .iter()
            .all(|id| Uuid::parse_str(id).unwrap().get_version_num() == 4)
    );
}

#[test]
fn failed_evidence_retry_keeps_committed_state() {
    let temp = temporary();
    let shell = established(temp.path());
    let pending = shell
        .begin(
            invoke(&shell, "agent-b", "project.file.write.v1", Some("retry")),
            &Deferred,
        )
        .unwrap();
    let before = fs::read(temp.path().join("state.json")).unwrap();
    shell
        .evidence
        .lock()
        .unwrap()
        .push_back(json!({"time":"2026-11-09T12:00:00Z","ids":[]}));
    let error = shell
        .complete(
            pending.invocation().unwrap(),
            1,
            json!({"kind":"missing"}),
            &MissingExecutor,
        )
        .err()
        .unwrap();
    assert!(error.contains("exhausted"));
    assert_eq!(before, fs::read(temp.path().join("state.json")).unwrap());
    assert!(pending.try_reply().unwrap().is_none());
    shell
        .complete(
            pending.invocation().unwrap(),
            1,
            json!({"kind":"missing"}),
            &MissingExecutor,
        )
        .unwrap();
    assert!(pending.try_reply().unwrap().is_some());
}

struct CrashExecutor {
    directory: PathBuf,
    after_dispatch: bool,
}
impl Executor for CrashExecutor {
    fn dispatch(&self, intent: &Value) -> Result<Execution> {
        let stored: Stored =
            serde_json::from_slice(&fs::read(self.directory.join("state.json")).unwrap()).unwrap();
        assert!(
            stored
                .state
                .records
                .iter()
                .any(|r| r["type"] == "invocation-dispatched"
                    && r["data"]["invocation"] == intent["invocation"])
        );
        if self.after_dispatch {
            // This fsynced marker stands for the transport's externally visible send.
            store::atomic_write(&self.directory, "sent", b"one dispatch").unwrap();
        }
        store::atomic_write(
            &self.directory,
            "ready",
            intent["invocation"].as_str().unwrap().as_bytes(),
        )
        .unwrap();
        loop {
            std::thread::park();
        }
    }
}

#[test]
fn crash_child() {
    let Some(path) = std::env::var_os("SHELL_TEST_STORE") else {
        return;
    };
    let directory = PathBuf::from(path);
    let shell = open(&directory);
    shell
        .submit(
            invoke(
                &shell,
                "agent-b",
                "project.file.write.v1",
                Some("crash-write"),
            ),
            &CrashExecutor {
                directory,
                after_dispatch: std::env::var_os("SHELL_TEST_AFTER_SEND").is_some(),
            },
        )
        .unwrap();
}
fn crash_test(after_dispatch: bool) {
    let temp = temporary();
    drop(established(temp.path()));
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", "tests::crash_child", "--nocapture"])
        .env("SHELL_TEST_STORE", temp.path())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    if after_dispatch {
        command.env("SHELL_TEST_AFTER_SEND", "1");
    } else {
        command.env_remove("SHELL_TEST_AFTER_SEND");
    }
    let mut child = command.spawn().unwrap();
    // Reaching the crash barrier checks progress, not startup speed.
    let deadline = Instant::now() + Duration::from_secs(600);
    while !temp.path().join("ready").exists() {
        if let Some(status) = child.try_wait().unwrap() {
            panic!("child exited {status}");
        }
        if Instant::now() > deadline {
            child.kill().unwrap();
            panic!("crash child did not reach barrier");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    child.kill().unwrap();
    child.wait().unwrap();
    let id = fs::read_to_string(temp.path().join("ready")).unwrap();
    assert_eq!(temp.path().join("sent").exists(), after_dispatch);
    let shell = open(temp.path());
    let state = shell.state().unwrap();
    assert_eq!(
        state.invocations.iter().find(|v| v["id"] == id).unwrap()["state"],
        "unknown"
    );
    assert_eq!(
        state
            .records
            .iter()
            .filter(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == id)
            .count(),
        1
    );
    // Late completion and another restart neither settle unknown nor replay the send.
    shell
        .complete(
            &id,
            1,
            json!({"kind":"result","result":{"content":[]}}),
            &MissingExecutor,
        )
        .unwrap();
    drop(shell);
    let shell = open(temp.path());
    assert_eq!(
        shell
            .state()
            .unwrap()
            .records
            .iter()
            .filter(|r| r["type"] == "invocation-dispatched" && r["data"]["invocation"] == id)
            .count(),
        1
    );
    assert_eq!(temp.path().join("sent").exists(), after_dispatch);
}
#[test]
fn killed_after_commit_before_send() {
    crash_test(false);
}
#[test]
fn killed_after_send_before_completion_commit() {
    crash_test(true);
}

#[path = "server_tests.rs"]
mod http;

#[test]
fn configured_trust_domain_survives_restart_and_cannot_be_replaced() {
    let temp = temporary();
    let config = Config {
        trust_domain: Some("tenant_1.example".into()),
        ..Config::default()
    };
    {
        let shell = Shell::open(temp.path(), context(), config.clone()).unwrap();
        shell
            .submit(
                json!({"kind":"first-start","machine":local_machine(BTreeMap::new()).unwrap()}),
                &MissingExecutor,
            )
            .unwrap();
        assert_eq!(
            shell.state().unwrap().objects[0]["status"]["trustDomain"],
            "tenant_1.example"
        );
        assert_eq!(
            fs::read_to_string(temp.path().join("engine-id")).unwrap(),
            "spiffe://tenant_1.example/conduit/engine"
        );
        assert!(
            shell
                .issue_client_token(shell.state().unwrap().objects[7]["id"].as_str().unwrap())
                .is_err()
        );
    }
    {
        let shell = Shell::open(temp.path(), context(), Config::default()).unwrap();
        assert_eq!(
            shell.state().unwrap().objects[0]["status"]["trustDomain"],
            "tenant_1.example"
        );
    }
    let changed = Config {
        trust_domain: Some("other.example".into()),
        ..Config::default()
    };
    assert!(
        Shell::open(temp.path(), context(), changed)
            .err()
            .unwrap()
            .contains("never changes")
    );
    for domain in ["", "Invalid", "a/b"] {
        let fresh = temporary();
        let bad = Config {
            trust_domain: Some(domain.into()),
            ..Config::default()
        };
        assert!(
            Shell::open(fresh.path(), context(), bad)
                .err()
                .unwrap()
                .contains("FST-007")
        );
    }
}

#[test]
fn first_start_presents_final_lists_and_unbuilt_operations_name_their_orders() {
    let temp = temporary();
    let shell = open(temp.path());
    shell
        .submit(
            json!({"kind":"first-start","machine":local_machine(BTreeMap::new()).unwrap()}),
            &MissingExecutor,
        )
        .unwrap();
    let root = shell.state().unwrap().objects[4]["id"].clone();
    let tools = shell
        .submit(
            json!({"kind":"list-tools","identity":{"kind":"instance","id":root}}),
            &MissingExecutor,
        )
        .unwrap();
    let mut expected = vec![];
    for (prefix, operations) in [
        (
            "management",
            vec![
                "register",
                "replace",
                "unregister",
                "inspect",
                "set-up",
                "retire",
                "reconcile",
                "register-replacement",
                "install",
                "declare-lost",
                "purge",
                "list",
                "answer",
            ],
        ),
        ("access", vec!["open", "close", "reconcile", "lift"]),
        (
            "system",
            vec![
                "register",
                "register-replacement",
                "inspect",
                "list",
                "set-up",
                "retire",
                "reconcile",
                "sign-in",
                "answer",
            ],
        ),
    ] {
        expected.extend(operations.into_iter().map(|op| format!("{prefix}.{op}.v1")));
    }
    expected.sort();
    let names: Vec<_> = tools["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, expected);
    for (tool, arguments, order) in [
        (
            "access.lift.v1",
            json!({"id":"00000000-0000-4000-8000-000000009999"}),
            "042",
        ),
        (
            "management.answer.v1",
            json!({"action":"00000000-0000-4000-8000-000000009999","answer":"dismissed"}),
            "047",
        ),
        (
            "system.sign-in.v1",
            json!({"instance":{"kind":"Resource","name":"management"}}),
            "048a",
        ),
    ] {
        let reply = shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":tool,"arguments":arguments}}), &MissingExecutor).unwrap();
        assert_eq!(
            reply["_meta"][format!("{}reason", shell.context.meta_prefix)],
            "unavailable"
        );
        assert!(
            reply["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains(&format!("not built yet, order {order}"))
        );
    }
    assert!(
        !shell
            .state()
            .unwrap()
            .records
            .iter()
            .any(|r| r["type"] == "invocation-dispatched")
    );
}
