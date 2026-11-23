#[path = "fixtures.rs"]
mod fixtures;
#[path = "limits_tests.rs"]
mod limits_tests;
#[path = "usage_tests.rs"]
mod usage_tests;
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
fn commit_measures_one_line_per_commit() {
    let temp = temporary();
    let shell = open(temp.path());
    let path = temp.path().join("commit-measures.jsonl");
    assert!(path.exists());
    let read = || -> Vec<Value> {
        fs::read_to_string(&path)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    };
    let mode = || fs::metadata(&path).unwrap().permissions().mode() & 0o777;
    let check_keys = |line: &Value, extra: Option<&str>| {
        let mut keys = vec![
            "at",
            "input",
            "stateBytes",
            "invocations",
            "writes",
            "stepMicros",
            "writeMicros",
            "commitMicros",
        ];
        keys.extend(extra);
        keys.sort();
        assert_eq!(
            line.as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            keys
        );
    };
    assert_eq!(mode(), 0o600);
    let lines = read();
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0]["input"], "recover");
    assert!(lines[0]["openMicros"].as_u64().is_some());
    assert!(lines[0].get("unwritten").is_none());
    shell
        .submit(
            json!({"kind":"first-start","machine":local_machine(BTreeMap::new()).unwrap()}),
            &MissingExecutor,
        )
        .unwrap();
    let input =
        json!({"kind":"list-tools","identity":{"kind":"instance","id":actor(&shell,"root")}});
    shell.submit(input.clone(), &MissingExecutor).unwrap();
    let lines = read();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[1]["input"], "first-start");
    assert_eq!(lines[2]["input"], "list-tools");
    for (index, line) in lines.iter().enumerate() {
        check_keys(line, (index == 0).then_some("openMicros"));
        assert_eq!(line["writes"], 1);
        let at = line["at"].as_str().unwrap();
        DateTime::parse_from_rfc3339(at).unwrap();
        assert!(at.ends_with('Z'));
        assert!(
            line["stepMicros"].as_u64().unwrap() + line["writeMicros"].as_u64().unwrap()
                <= line["commitMicros"].as_u64().unwrap()
        );
    }
    assert_eq!(
        lines[2]["stateBytes"],
        fs::metadata(temp.path().join("state.json")).unwrap().len()
    );
    assert_eq!(
        lines[2]["invocations"],
        shell.state().unwrap().invocations.len()
    );
    let original = fs::read(&path).unwrap();
    *shell.writer.lock().unwrap().store.fault.borrow_mut() = Some(store::WriteFault::Write);
    assert!(shell.submit(input.clone(), &MissingExecutor).is_err());
    assert_eq!(fs::read(&path).unwrap(), original);
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    shell.submit(input.clone(), &MissingExecutor).unwrap();
    assert_eq!(fs::read(&path).unwrap(), original);
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    fs::rename(&path, temp.path().join("commit-measures.old")).unwrap();
    std::os::unix::fs::symlink(temp.path().join("missing-target"), &path).unwrap();
    shell.submit(input.clone(), &MissingExecutor).unwrap();
    assert!(!temp.path().join("missing-target").exists());
    fs::remove_file(&path).unwrap();
    shell.submit(input, &MissingExecutor).unwrap();
    assert!(path.exists());
    assert_eq!(mode(), 0o600);
    let lines = read();
    assert_eq!(lines.len(), 1);
    check_keys(&lines[0], Some("unwritten"));
    assert_eq!(lines[0]["unwritten"], 2);
    drop(shell);
    let _shell = open(temp.path());
    let lines = read();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[1]["input"], "recover");
    check_keys(&lines[1], Some("openMicros"));
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
    // DL2: only tests that inspect transitions retain the test trace.
    *shell.trace.lock().unwrap() = Some(Vec::new());
    let case = case();
    let steps = case["steps"].as_array().unwrap();
    let mut symbols = BTreeMap::new();
    for step in &steps[..13] {
        let input = fixture_input(&step["input"], &shell, &symbols);
        if let Some(evidence) = step["input"].get("evidence") {
            shell.evidence.lock().unwrap().push_back(evidence.clone());
        }
        let committed = shell.commit(input).unwrap();
        let t = shell
            .trace
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .last()
            .unwrap()
            .clone();
        if let Some(capture) = step["capture"].as_object() {
            let value = capture_value(&t);
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
    *shell.trace.lock().unwrap() = None;
    shell
}
// LIF-040 / ACC-053: expose the same opaque tombstone captures as the portable runner.
fn capture_value(transition: &conduit_core::Transition) -> Value {
    let mut value = serde_json::to_value(transition).unwrap();
    value["tombstones"] = json!(transition.state.tombstones.values().collect::<Vec<_>>());
    value
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
    // DL2: the full-loop test explicitly opts into transition tracing.
    *shell.trace.lock().unwrap() = Some(Vec::new());
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
        let trace = std::mem::take(shell.trace.lock().unwrap().as_mut().unwrap());
        assert!(!trace.is_empty());
        for transition in trace {
            let step = &steps[index];
            let actual = capture_value(&transition);
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

fn scripted_discovery(intent: &Value) -> Option<Execution> {
    (intent["call"]["method"] == "server/discover").then(|| {
        Execution::Complete(json!({"kind":"result","result":{"capabilities":{
            "extensions":intent["call"]["params"]["_meta"]
                ["io.modelcontextprotocol/clientCapabilities"]["extensions"]
        }}}))
    })
}

struct Deferred;
impl Executor for Deferred {
    fn dispatch(&self, intent: &Value) -> Result<Execution> {
        Ok(scripted_discovery(intent).unwrap_or(Execution::Deferred))
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
    let measures_path = temp.path().join("commit-measures.jsonl");
    let read_measures = || -> Vec<Value> {
        fs::read_to_string(&measures_path)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    };
    let count_outputs = || {
        shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .filter(|v| retained_results(v).next().is_some())
            .count()
    };
    let before_measures = read_measures();
    let before_outputs = count_outputs();
    shell
        .submit(
            invoke(&shell, "agent-b", "project.file.write.v1", Some("newer")),
            &executor,
        )
        .unwrap();
    let after_measures = read_measures();
    let after_outputs = count_outputs();
    assert!(before_measures.iter().all(|line| line["writes"] == 1));
    assert_eq!(after_measures.len(), before_measures.len() + 2);
    let appended = &after_measures[before_measures.len()..];
    assert_eq!(appended[0]["input"], "invoke");
    assert_eq!(appended[1]["input"], "complete");
    let invoke_writes = appended[0]["writes"].as_u64().unwrap();
    let complete_writes = appended[1]["writes"].as_u64().unwrap();
    assert!(complete_writes >= 2);
    assert_eq!(
        invoke_writes + complete_writes,
        (2 + before_outputs + 1 - after_outputs) as u64
    );
    println!(
        "retention measures: writes {invoke_writes}, {complete_writes}; outputs before {before_outputs}, after {after_outputs}"
    );
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
pub(crate) fn filtered_shell(path: &Path) -> Shell {
    let shell = established(path);
    let writer = shell.writer.lock().unwrap();
    // DL2: adapt the fixture's state access to the shared committed snapshot.
    let mut stored = writer.store.take().unwrap();
    let mut filtered_paths = 0;
    for object in &mut Arc::make_mut(&mut stored.state).objects {
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
                Arc::unwrap_or_clone(std::mem::take(&mut stored.state)),
                &writer.store,
                json!({"kind":"expire-outputs","invocation":id}),
            )
            .unwrap();
        shell.persist(&writer, &mut stored, &transition).unwrap();
        stored.state = Arc::new(transition.state);
    }
    writer.store.remember(stored);
    drop(writer);
    shell
}
fn filter() -> Value {
    json!({"requirement":"example.org/requirements#deadline.v1","arguments":{"until":"2026-11-09T00:00:00Z"}})
}
pub(crate) fn filtered_read(shell: &Shell, key: &str) -> Value {
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
    assert_eq!(
        actual["structuredContent"]["diagnostics"][0]["ruleId"],
        "IFC-026"
    );
    assert_eq!(
        actual["structuredContent"]["diagnostics"][0]["properties"]["refusal"]["tier"],
        1
    );
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
    Arc::make_mut(&mut removed)
        .objects
        .retain(|v| v["id"] != id);
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
#[path = "https_tests.rs"]
mod https_tests;

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
    // ACC-053 / OPS-029: lift is built; an unbounded unknown ID is invalid before dispatch.
    let lift = shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":"access.lift.v1","arguments":{"id":"00000000-0000-4000-8000-000000009999"}}}), &MissingExecutor).unwrap();
    assert_eq!(
        lift["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "invalid"
    );
    assert_eq!(
        lift["structuredContent"]["diagnostics"][0]["ruleId"],
        "OPS-029"
    );
    let reply = shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":"system.sign-in.v1","arguments":{"instance":{"kind":"Resource","name":"management"}}}}), &MissingExecutor).unwrap();
    assert_eq!(
        reply["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "invalid"
    );
    assert_eq!(
        reply["structuredContent"]["diagnostics"][0]["ruleId"],
        "PND-010"
    );
    assert!(
        reply["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("the instance's Type states no signIn")
    );
    let answer = shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":"management.answer.v1","arguments":{"action":"00000000-0000-4000-8000-000000009999","answer":"dismissed"}}}), &MissingExecutor).unwrap();
    assert_eq!(
        answer["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "refused"
    );
    assert_eq!(
        answer["structuredContent"]["diagnostics"][0]["ruleId"],
        "PND-009"
    );
    assert!(
        !shell
            .state()
            .unwrap()
            .records
            .iter()
            .any(|r| r["type"] == "invocation-dispatched")
    );
}

// DL2/DL6: compare memory with a fresh parse without exposing store credentials.
fn assert_memory_equals_disk(shell: &Shell) {
    let snapshot = shell.state().unwrap();
    let writer = shell.writer.lock().unwrap();
    let disk = writer.store.read().unwrap();
    let memory = writer.store.take().unwrap();
    assert!(*snapshot == disk.state, "reader differs from state.json");
    assert!(
        *memory.state == disk.state,
        "writer differs from state.json"
    );
    assert_eq!(memory.output_times, disk.output_times);
    // DL2: reference serialization must retain version 1's exact field order and bytes.
    assert!(
        serde_json::to_vec(&disk).unwrap()
            == store::private_read(&writer.store.directory.join("state.json")).unwrap(),
        "version-1 serialization differs from state.json"
    );
    writer.store.remember(memory);
}

// DL2/IMP-007/REC-003/OPS-013: a failed persistence boundary discards memory;
// after rename, the committed file rather than the failed proposed state wins.
fn memory_equals_disk_after_fault(fault: store::WriteFault) {
    let temp = temporary();
    let shell = established(temp.path());
    let pending = shell
        .begin(
            invoke(&shell, "agent-b", "project.file.write.v1", None),
            &Deferred,
        )
        .unwrap();
    let input = json!({"kind":"complete","invocation":pending.invocation.as_ref().unwrap(),"attempt":1,
        "completion":{"kind":"result","result":{"content":[{"type":"text","text":"retained fault-test output"}]}}});
    let held = shell.state().unwrap();
    let before = store::private_read(&shell.directory.join("state.json")).unwrap();
    assert!(Arc::ptr_eq(&held, &shell.state().unwrap()));
    *shell.writer.lock().unwrap().store.fault.borrow_mut() = Some(fault);
    let error = match shell.commit(input.clone()) {
        Err(error) => error,
        Ok(_) => panic!("fault did not fail the commit"),
    };
    assert!(error.contains("injected state write failure"));
    let after = store::private_read(&shell.directory.join("state.json")).unwrap();
    assert_eq!(
        before == after,
        fault != store::WriteFault::DirectorySync,
        "only the directory-fsync fault must leave the renamed commit on disk"
    );
    assert_memory_equals_disk(&shell);
    shell.commit(input).unwrap();
    assert_memory_equals_disk(&shell);
    // DL2: a held reader retains its immutable previous commit across the next write.
    assert!(!Arc::ptr_eq(&held, &shell.state().unwrap()));
}

#[test]
fn memory_equals_disk_after_temporary_write_failure() {
    // DL2: the per-store fault must leave memory equal to the authoritative file.
    memory_equals_disk_after_fault(store::WriteFault::Write);
}
#[test]
fn memory_equals_disk_after_file_fsync_failure() {
    // DL2: the per-store fault must leave memory equal to the authoritative file.
    memory_equals_disk_after_fault(store::WriteFault::FileSync);
}
#[test]
fn memory_equals_disk_after_rename_failure() {
    // DL2: the per-store fault must leave memory equal to the authoritative file.
    memory_equals_disk_after_fault(store::WriteFault::Rename);
}
#[test]
fn memory_equals_disk_after_directory_fsync_failure() {
    // DL2: the per-store fault must leave memory equal to the authoritative file.
    memory_equals_disk_after_fault(store::WriteFault::DirectorySync);
}

#[test]
fn memory_equals_disk_after_step_failure_and_evidence_retry() {
    // DL6: both exhausted-ID attempts consume their State, then reload the last commit.
    let temp = temporary();
    let shell = established(temp.path());
    let input = invoke(&shell, "agent-b", "project.file.write.v1", None);
    shell
        .evidence
        .lock()
        .unwrap()
        .push_back(json!({"time":"2026-11-11T00:00:00Z","ids":[],"revisions":[],"suffixes":[]}));
    let error = match shell.commit(input.clone()) {
        Err(error) => error,
        Ok(_) => panic!("empty evidence unexpectedly succeeded"),
    };
    assert!(error.starts_with("embedding error: exhausted "));
    assert_memory_equals_disk(&shell);
    shell.commit(input).unwrap();
    assert_memory_equals_disk(&shell);
}

#[test]
fn state_write_fault_is_private_to_its_store_and_state_file() {
    // DL2/DL13: token writes and another store cannot consume this store's state fault.
    let first = temporary();
    let shell = open(first.path());
    let bootstrap = json!({"kind":"first-start","machine":local_machine(BTreeMap::new()).unwrap()});
    shell.submit(bootstrap.clone(), &MissingExecutor).unwrap();
    let root = actor(&shell, "root");
    *shell.writer.lock().unwrap().store.fault.borrow_mut() = Some(store::WriteFault::Write);
    shell.issue_client_token(&root).unwrap();

    let second = temporary();
    let other = open(second.path());
    other.submit(bootstrap, &MissingExecutor).unwrap();
    assert_memory_equals_disk(&other);

    let input = json!({"kind":"list-tools","identity":{"kind":"instance","id":root}});
    let error = shell.submit(input.clone(), &MissingExecutor).unwrap_err();
    assert!(error.contains("injected state write failure"));
    assert_memory_equals_disk(&shell);
    shell.submit(input, &MissingExecutor).unwrap();
    assert_memory_equals_disk(&shell);
}

fn fail_writes(shell: &Shell, fault: store::WriteFault, count: usize) {
    shell
        .writer
        .lock()
        .unwrap()
        .store
        .faults
        .borrow_mut()
        .extend(std::iter::repeat_n(fault, count));
}

fn list_tools_after_fault(shell: &Shell) {
    shell
        .submit(
            json!({"kind":"list-tools","identity":{"kind":"instance","id":actor(shell, "root")}}),
            &MissingExecutor,
        )
        .unwrap();
    assert_memory_equals_disk(shell);
}

fn assert_record_fault_reply(shell: &Shell, reply: &Value, field: &str, expected: &str) {
    assert_eq!(reply["isError"], true);
    assert_eq!(
        reply["_meta"][format!("{}{field}", shell.context.meta_prefix)],
        expected
    );
    let diagnostics = reply["structuredContent"]["diagnostics"]
        .as_array()
        .unwrap();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0]["ruleId"], "REC-007");
    assert_eq!(diagnostics[0]["level"], "error");
}

#[test]
fn a_request_whose_record_cannot_be_kept_reports_unavailable() {
    struct NoDispatch;
    impl Executor for NoDispatch {
        fn dispatch(&self, _: &Value) -> Result<Execution> {
            panic!("an unrecorded request reached the executor");
        }
    }

    for tool in ["project.file.write.v1", "project.file.absent.v1"] {
        for fault in [
            store::WriteFault::Write,
            store::WriteFault::FileSync,
            store::WriteFault::Rename,
        ] {
            let temp = temporary();
            let shell = established(temp.path());
            let before = shell.state().unwrap().records.len();
            fail_writes(&shell, fault, 2);
            let reply = shell
                .submit(invoke(&shell, "agent-b", tool, None), &NoDispatch)
                .expect("an unrecorded request must return a result");
            assert_record_fault_reply(&shell, &reply, "reason", "unavailable");
            assert_eq!(shell.state().unwrap().records.len(), before);
            list_tools_after_fault(&shell);
            let state = shell.state().unwrap();
            let decision = &state.records[before];
            let end = &state.records[before + 1];
            assert_eq!(decision["type"], "access-decision");
            assert_eq!(decision["data"]["dispatched"], false);
            assert_eq!(end["type"], "request-ended");
            for record in [decision, end] {
                assert_eq!(record["data"]["reason"], "unavailable");
                assert_eq!(record["data"]["ruleId"], "REC-007");
            }
        }
    }
    let temp = temporary();
    let shell = established(temp.path());
    let before = shell.state().unwrap().records.len();
    fail_writes(&shell, store::WriteFault::Write, 4);
    for tool in ["project.file.write.v1", "project.file.absent.v1"] {
        let reply = shell
            .submit(invoke(&shell, "agent-b", tool, None), &NoDispatch)
            .unwrap();
        assert_record_fault_reply(&shell, &reply, "reason", "unavailable");
        assert_eq!(shell.state().unwrap().records.len(), before);
    }
    list_tools_after_fault(&shell);
    let state = shell.state().unwrap();
    assert_eq!(state.records.len(), before + 4);
    for (records, tool) in state.records[before..]
        .chunks_exact(2)
        .zip(["project.file.write.v1", "project.file.absent.v1"])
    {
        assert_eq!(records[0]["type"], "access-decision");
        assert_eq!(records[0]["data"]["tool"], tool);
        assert_eq!(records[1]["type"], "request-ended");
        assert_eq!(records[1]["data"]["ruleId"], "REC-007");
    }
}

#[test]
fn an_invocation_whose_end_cannot_be_recorded_ends_unknown() {
    let temp = temporary();
    let shell = established(temp.path());
    let pending = shell
        .begin(
            invoke(&shell, "agent-b", "project.file.write.v1", None),
            &Deferred,
        )
        .unwrap();
    let id = pending.invocation().unwrap();
    let before = shell.state().unwrap().records.len();
    fail_writes(&shell, store::WriteFault::Write, 2);
    shell.complete(id, 1, json!({"kind":"result","result":{"content":[{"type":"text","text":"uncommitted implementation output"}]}}), &MissingExecutor)
        .expect("a lost end record must settle the invocation");
    let reply = pending
        .try_reply()
        .unwrap()
        .expect("completion must release the caller");
    assert_record_fault_reply(&shell, &reply, "outcome", "unknown");
    assert!(
        !reply
            .to_string()
            .contains("uncommitted implementation output")
    );
    assert_eq!(shell.state().unwrap().records.len(), before);
    list_tools_after_fault(&shell);
    let state = shell.state().unwrap();
    for (record, kind) in state.records[before..]
        .iter()
        .zip(["invocation-ended", "request-ended"])
    {
        assert_eq!(record["type"], kind);
        assert_eq!(record["data"]["invocation"], id);
        assert_eq!(record["data"]["outcome"], "unknown");
    }
    assert!(state.records.len() >= before + 2);
}

#[test]
fn a_completion_read_back_after_rename_keeps_its_result_and_end_records() {
    let temp = temporary();
    let shell = established(temp.path());
    let pending = shell
        .begin(
            invoke(&shell, "agent-b", "project.file.write.v1", None),
            &Deferred,
        )
        .unwrap();
    let id = pending.invocation().unwrap();
    let before = shell.state().unwrap().records.len();
    fail_writes(&shell, store::WriteFault::DirectorySync, 1);
    let completed = shell.complete(
        id, 1,
        json!({"kind":"result","result":{"isError":false,"content":[{"type":"text","text":"completion output"}]}}),
        &MissingExecutor,
    ).unwrap();
    let reply = pending.try_reply().unwrap().unwrap();
    assert_eq!(Some(reply.clone()), completed.try_reply().unwrap());
    assert_eq!(
        reply["_meta"][format!("{}outcome", shell.context.meta_prefix)],
        "succeeded"
    );
    assert_eq!(reply["content"][0]["text"], "completion output");
    assert_memory_equals_disk(&shell);
    let state = shell.state().unwrap();
    assert_eq!(state.records.len(), before + 2);
    for (record, kind) in state.records[before..]
        .iter()
        .zip(["invocation-ended", "request-ended"])
    {
        assert_eq!(record["type"], kind);
        assert_eq!(record["data"]["invocation"], id);
        assert_eq!(record["data"]["outcome"], "succeeded");
    }
    list_tools_after_fault(&shell);
    assert_eq!(shell.state().unwrap().records.len(), before + 2);
}

#[test]
fn an_ordinary_request_waiting_for_discovery_ends_unavailable_at_recovery() {
    struct PendingDiscovery;
    impl Executor for PendingDiscovery {
        fn dispatch(&self, intent: &Value) -> Result<Execution> {
            assert_eq!(intent["call"]["method"], "server/discover");
            Ok(Execution::Deferred)
        }
    }

    let temp = temporary();
    let shell = filtered_shell(temp.path());
    let pending = shell
        .begin(
            filtered_read(&shell, "discovery-restart"),
            &PendingDiscovery,
        )
        .unwrap();
    let id = pending.invocation().unwrap();
    assert!(pending.try_reply().unwrap().is_none());
    let before = shell.state().unwrap();
    let access = before
        .records
        .iter()
        .find(|record| record["data"]["invocation"] == id && record["type"] == "access-decision")
        .expect("access must be recorded before discovery");
    assert_eq!(access["data"]["permitted"], true);
    assert!(!before.records.iter().any(|record| {
        record["data"]["invocation"] == id && record["type"] == "invocation-dispatched"
    }));
    let context = shell.context.clone();
    drop(shell);

    let reopened = Shell::open(temp.path(), context, Config::default()).unwrap();
    let state = reopened.state().unwrap();
    let end = state.records[before.records.len()..]
        .iter()
        .find(|record| record["data"]["invocation"] == id && record["type"] == "request-ended")
        .expect("recovery must append a request end");
    assert_eq!(end["data"]["reason"], "unavailable");
    assert_eq!(end["data"]["ruleId"], "REC-007");
    let records: Vec<_> = state
        .records
        .iter()
        .filter(|record| record["data"]["invocation"] == id)
        .collect();
    assert_eq!(records.len(), 2);
    assert!(records[0] == access, "the recorded access decision changed");
    assert!(
        records[1] == end,
        "recovery appended a different request end"
    );
    assert_memory_equals_disk(&reopened);
}

#[test]
fn an_invocation_whose_end_cannot_be_recorded_keeps_the_implementation_reason() {
    let temp = temporary();
    let shell = filtered_shell(temp.path());
    let pending = shell
        .begin(filtered_read(&shell, "filter-end"), &Deferred)
        .unwrap();
    let id = pending.invocation().unwrap();
    let before = shell.state().unwrap().records.len();
    fail_writes(&shell, store::WriteFault::Write, 2);
    let completed = shell
        .complete(id, 1, filter_refusal(&shell), &MissingExecutor)
        .expect("the implementation's reason must settle the invocation");
    assert_filter_reply(&shell, &pending, &completed);
    assert_eq!(shell.state().unwrap().records.len(), before);
    list_tools_after_fault(&shell);
    let state = shell.state().unwrap();
    for (record, kind) in state.records[before..]
        .iter()
        .zip(["invocation-ended", "request-ended"])
    {
        assert_eq!(record["type"], kind);
        assert_eq!(record["data"]["invocation"], id);
        assert_eq!(record["data"]["reason"], "refused");
        assert!(record["data"].get("outcome").is_none());
    }
    assert!(state.records.len() >= before + 2);
}

#[test]
fn a_write_that_fails_after_its_rename_ends_the_undispatched_request_unavailable() {
    struct NoDispatch;
    impl Executor for NoDispatch {
        fn dispatch(&self, _: &Value) -> Result<Execution> {
            panic!("a recorded but unsent dispatch reached the executor");
        }
    }
    let temp = temporary();
    let shell = established(temp.path());
    let before = shell.state().unwrap().records.len();
    fail_writes(&shell, store::WriteFault::DirectorySync, 1);
    let reply = shell
        .submit(
            invoke(&shell, "agent-b", "project.file.write.v1", None),
            &NoDispatch,
        )
        .expect("a recorded but unsent dispatch must return a result");
    assert_record_fault_reply(&shell, &reply, "reason", "unavailable");
    assert!(
        reply["structuredContent"]["diagnostics"][0]["message"]["text"]
            .as_str()
            .is_some_and(|message| message.contains("the record could be read back"))
    );
    let state = shell.state().unwrap();
    assert_eq!(state.records[before]["type"], "access-decision");
    assert_eq!(state.records[before + 1]["type"], "invocation-dispatched");
    let end = &state.records[before + 2];
    assert_eq!(end["type"], "request-ended");
    assert_eq!(end["data"]["reason"], "unavailable");
    assert_eq!(end["data"]["ruleId"], "REC-007");
    assert_memory_equals_disk(&shell);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SecretReadFault {
    Inspect,
    Discovery,
    Restart,
    NullRestriction,
    DirectorySync,
}

// SEC-016: inject the fault as the secret provider answers discovery, or
// inject it on the first run inspection to check that setup stops before discovery.
fn secret_read_record_fault(fault: SecretReadFault) {
    struct StoreServer<'a> {
        shell: &'a Shell,
        store_id: Value,
        discoveries: AtomicUsize,
        inspections: AtomicUsize,
        reads: AtomicUsize,
        consumer_runs: AtomicUsize,
        withheld: String,
        fault: SecretReadFault,
    }
    impl Executor for StoreServer<'_> {
        fn dispatch(&self, intent: &Value) -> Result<Execution> {
            assert_eq!(intent["target"], self.store_id);
            match intent["call"]["method"].as_str().unwrap() {
                "server/discover" => {
                    self.discoveries.fetch_add(1, Ordering::SeqCst);
                    let stage = if self.fault == SecretReadFault::DirectorySync {
                        store::WriteFault::DirectorySync
                    } else {
                        store::WriteFault::Write
                    };
                    fail_writes(
                        self.shell,
                        stage,
                        if self.fault == SecretReadFault::Restart {
                            2
                        } else {
                            1
                        },
                    );
                    if self.fault == SecretReadFault::NullRestriction {
                        let prefix = &self.shell.context.meta_prefix;
                        return Ok(Execution::Complete(json!({"kind":"result","result":{
                            "isError":true,"_meta":{format!("{prefix}reason"):"unavailable",format!("{prefix}restriction"):null}
                        }})));
                    }
                    let extension =
                        format!("{}implementation-interface", self.shell.context.meta_prefix);
                    Ok(Execution::Complete(json!({"kind":"result","result":{
                        "capabilities":{"extensions":{extension:{"revision":"v0"}}}
                    }})))
                }
                "tools/call" => {
                    assert_eq!(intent["call"]["params"]["name"], "get-secret.v1");
                    self.reads.fetch_add(1, Ordering::SeqCst);
                    let outcome = format!("{}outcome", self.shell.context.meta_prefix);
                    Ok(Execution::Complete(json!({"kind":"result","result":{
                        "structuredContent":{"value":self.withheld},"_meta":{outcome:"succeeded"}
                    }})))
                }
                _ => panic!("unexpected store server request"),
            }
        }

        fn builtin_runner(&self, intent: &Value, _: Option<&RunIdentity>) -> Result<Execution> {
            let core = &self.shell.context.core_package;
            let inspect = format!("{core}#inspect-run.v1");
            let run = format!("{core}#run.v1");
            if intent["operation"] == inspect {
                if self.inspections.fetch_add(1, Ordering::SeqCst) == 0
                    && self.fault == SecretReadFault::Inspect
                {
                    fail_writes(self.shell, store::WriteFault::Write, 1);
                }
                return Ok(Execution::SetupReport(
                    json!({"kind":"inspect-run","state":"stopped"}),
                ));
            }
            assert_eq!(intent["operation"], run);
            if intent["subject"] != self.store_id {
                self.consumer_runs.fetch_add(1, Ordering::SeqCst);
            }
            Ok(Execution::SetupReport(
                json!({"kind":"run","outcome":"succeeded"}),
            ))
        }
    }

    let temp = temporary();
    let shell = open(temp.path());
    shell
        .submit(
            json!({"kind":"first-start","machine":local_machine(BTreeMap::new()).unwrap()}),
            &MissingExecutor,
        )
        .unwrap();
    let root = actor(&shell, "root");
    let spec = conduit_loader::Specification::read(&repository().join("spec")).unwrap();
    let project = boundary::project(
        &repository().join("conformance/conventions/secret-reads/project"),
        &spec,
    )
    .unwrap();
    let manifest = project.manifest.unwrap();
    let registered = shell
        .submit(
            json!({"kind":"invoke","request":{
                "identity":{"kind":"instance","id":root},"tool":"management.register.v1",
                "arguments":{"documents":project.documents,"manifest":manifest}
            }}),
            &MissingExecutor,
        )
        .unwrap();
    assert_eq!(registered["isError"], false);
    let locator = |entry| {
        shell
            .state()
            .unwrap()
            .objects
            .iter()
            .find(|object| object["kind"] == "Resource" && object["nested"]["name"] == entry)
            .unwrap()["id"]
            .clone()
    };
    let probe = StoreServer {
        shell: &shell,
        store_id: locator("store"),
        discoveries: AtomicUsize::new(0),
        inspections: AtomicUsize::new(0),
        reads: AtomicUsize::new(0),
        consumer_runs: AtomicUsize::new(0),
        withheld: format!("withheld-{}", Uuid::new_v4()),
        fault,
    };
    let result = shell.submit(
        json!({"kind":"invoke","request":{
            "identity":{"kind":"instance","id":root},"tool":"management.set-up.v1",
            "arguments":{"instance":{"kind":"Pool","name":"demo"}}
        }}),
        &probe,
    );
    let inspect_fault = fault == SecretReadFault::Inspect;
    let recover = fault == SecretReadFault::Restart;
    assert_eq!(
        probe.discoveries.load(Ordering::SeqCst),
        usize::from(!inspect_fault)
    );
    assert_eq!(probe.reads.load(Ordering::SeqCst), 0);
    assert_eq!(probe.consumer_runs.load(Ordering::SeqCst), 0);
    assert!(shell.writer.lock().unwrap().read_values.is_empty());
    let context = shell.context.clone();
    let withheld = probe.withheld.clone();
    let shell = if recover {
        drop(shell);
        Shell::open(temp.path(), context, Config::default()).unwrap()
    } else {
        let result = result.expect("setup must return a final result when a record write fails");
        assert!(!result.to_string().contains(&withheld));
        if inspect_fault {
            assert_record_fault_reply(&shell, &result, "outcome", "unknown");
        }
        if fault == SecretReadFault::DirectorySync {
            assert!(
                result["structuredContent"]["diagnostics"]
                    .as_array()
                    .is_some_and(|diagnostics| {
                        diagnostics.iter().any(|diagnostic| {
                            diagnostic["ruleId"] == "SEC-016"
                                && diagnostic["message"]["text"]
                                    .as_str()
                                    .is_some_and(|message| {
                                        message.contains("the record could be read back")
                                            && message.contains("the write reported a failure")
                                    })
                        })
                    })
            );
        }
        shell
    };
    let state = shell.state().unwrap();
    assert!(
        !serde_json::to_string(state.as_ref())
            .unwrap()
            .contains(&withheld)
    );
    assert_memory_equals_disk(&shell);
    if inspect_fault {
        assert!(
            state
                .records
                .iter()
                .any(|record| record["type"] == "request-ended"
                    && record["data"]["operation"]
                        == format!("{}#set-up.v1", shell.context.core_package)
                    && record["data"]["outcome"] == "unknown")
        );
        return;
    }
    let read = state
        .invocations
        .iter()
        .find(|invocation| {
            invocation["operation"] == format!("{}#get-secret.v1", shell.context.core_package)
        })
        .expect("discovery did not commit");
    assert_eq!(read["state"], "unavailable");
    let read_records: Vec<_> = state
        .records
        .iter()
        .filter(|record| record["data"]["invocation"] == read["id"])
        .collect();
    let read_back = fault == SecretReadFault::DirectorySync;
    assert_eq!(read_records.len(), if read_back { 3 } else { 2 });
    assert_eq!(read_records[0]["type"], "access-decision");
    if read_back {
        assert_eq!(read_records[0]["data"]["permitted"], true);
        assert_eq!(read_records[1]["type"], "invocation-dispatched");
    } else {
        assert_eq!(read_records[0]["data"]["dispatched"], false);
        assert_eq!(read_records[0]["data"]["reason"], "unavailable");
        assert_eq!(read_records[0]["data"]["ruleId"], "SEC-016");
    }
    let end = read_records.last().unwrap();
    assert_eq!(end["type"], "request-ended");
    assert_eq!(end["data"]["reason"], "unavailable");
    assert_eq!(end["data"]["ruleId"], "SEC-016");
    assert!(state.invocations.iter().any(|invocation| invocation["id"]
        == read["secret_read"]["run"]
        && if recover {
            invocation["state"] == "unknown"
        } else {
            invocation["setup"]["parts"]
                .as_array()
                .unwrap()
                .iter()
                .any(|part| part["name"] == "run" && part["outcome"] == "unavailable")
        }));
}

#[test]
fn a_setup_whose_records_cannot_be_kept_asks_for_no_run() {
    secret_read_record_fault(SecretReadFault::Inspect);
}

#[test]
fn a_read_whose_record_cannot_be_kept_ends_unavailable_at_once() {
    secret_read_record_fault(SecretReadFault::Discovery);
}

#[test]
fn an_undispatched_secret_read_ends_unavailable_at_recovery() {
    secret_read_record_fault(SecretReadFault::Restart);
}

#[test]
fn a_failed_secret_discovery_record_does_not_preserve_a_null_restriction() {
    secret_read_record_fault(SecretReadFault::NullRestriction);
}

#[test]
fn a_secret_dispatch_read_back_after_rename_ends_unavailable_under_sec_016() {
    secret_read_record_fault(SecretReadFault::DirectorySync);
}
