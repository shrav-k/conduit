use rmcp::{
    RoleClient, ServiceExt,
    model::*,
    service::{PeerRequestOptions, RequestHandle, RunningService, ServiceError},
    transport::TokioChildProcess,
};
use scenario_s4_effect::{
    ADDRESS, EXTENSION, OUTCOME, PREFIX, REASON, RESTRICTION, SCHEMAS, ledger,
};
use serde_json::{Value, json};
use std::{
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    process::Stdio,
    time::{Duration, Instant},
};

struct Fixture {
    dir: tempfile::TempDir,
    ledger: PathBuf,
    control: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let ledger = dir.path().join("ledger.jsonl");
        let control = dir.path().join("control");
        fs::create_dir(&control).unwrap();
        fs::set_permissions(&control, fs::Permissions::from_mode(0o700)).unwrap();
        Self {
            dir,
            ledger,
            control,
        }
    }
    fn read(&self) -> Value {
        ledger::readback(&self.ledger, None).unwrap()
    }
    fn cli(&self) -> Value {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_scenario-s4-effect"))
            .arg("readback")
            .arg(&self.ledger)
            .env_clear()
            .output()
            .unwrap();
        assert!(output.status.success(), "readback command failed");
        assert!(output.stderr.is_empty());
        serde_json::from_slice(&output.stdout).unwrap_or_else(|_| panic!("invalid readback JSON"))
    }
    async fn reached(&self, label: &str, barrier: &str, attempt: u64) -> PathBuf {
        let start = Instant::now();
        loop {
            for entry in fs::read_dir(&self.control).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().is_some_and(|s| s == "reached") {
                    let v: Value = serde_json::from_slice(&fs::read(&path).unwrap())
                        .unwrap_or_else(|_| panic!("invalid reached file"));
                    if v["details"]["label"] == label
                        && v["barrier"] == barrier
                        && v["details"]["attempt"] == attempt
                    {
                        assert!(v["version"] == 1 && v["package"] == ADDRESS);
                        assert!(v["call"] == v["details"]["call"]);
                        assert!(fs::metadata(&path).unwrap().mode() & 0o777 == 0o600);
                        return path;
                    }
                }
            }
            assert!(
                start.elapsed() < Duration::from_secs(20),
                "server barrier was never reached"
            );
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    }
    async fn release(&self, reached: &Path, verdict: &str) {
        let release = reached.with_extension("release");
        let temporary = reached.with_extension("controller-temporary");
        fs::write(&temporary, verdict).unwrap();
        fs::set_permissions(&temporary, fs::Permissions::from_mode(0o600)).unwrap();
        fs::rename(&temporary, &release).unwrap();
        self.acknowledged(reached).await;
    }
    async fn acknowledged(&self, reached: &Path) {
        let ack = reached.with_extension("acknowledged");
        let start = Instant::now();
        while !ack.exists() {
            assert!(
                start.elapsed() < Duration::from_secs(20),
                "server never acknowledged release"
            );
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        assert!(fs::metadata(ack).unwrap().mode() & 0o777 == 0o600);
    }
    async fn finish(&self, process: &Process, request: Value, label: &str, attempt: u64) -> Value {
        let handle = process.begin(request).await;
        let first = self.reached(label, "before-apply", attempt).await;
        self.release(&first, "proceed").await;
        let last = self.reached(label, "after-commit", attempt).await;
        self.release(&last, "proceed").await;
        success(handle, "apply-effect.v1", true).await
    }
}
struct Process {
    client: RunningService<RoleClient, ()>,
    pid: u32,
    stderr: tempfile::NamedTempFile,
}
impl Process {
    async fn start(f: &Fixture, item: &str, mode: &str, barriers: bool) -> Self {
        let stderr = tempfile::NamedTempFile::new().unwrap();
        let mut cmd = tokio::process::Command::new(env!("CARGO_BIN_EXE_scenario-s4-effect"));
        cmd.arg(item)
            .env_clear()
            .env("S4_EFFECT_LEDGER", &f.ledger)
            .env("S4_EFFECT_MODE", mode)
            .stderr(Stdio::from(stderr.reopen().unwrap()));
        if barriers {
            cmd.env("S4_EFFECT_CONTROL", &f.control);
        }
        let transport = TokioChildProcess::new(cmd).unwrap();
        let pid = transport.id().unwrap();
        let client = ().serve(transport).await.unwrap_or_else(|_| panic!("stdio startup failed"));
        Self {
            client,
            pid,
            stderr,
        }
    }
    async fn begin(&self, params: Value) -> RequestHandle<RoleClient> {
        self.client
            .send_request_with_option(
                serde_json::from_value(json!({"method":"tools/call","params":params}))
                    .unwrap_or_else(|_| panic!("invalid test request")),
                PeerRequestOptions::no_options(),
            )
            .await
            .unwrap_or_else(|_| panic!("request send failed"))
    }
    async fn discover(&self) -> Value {
        let response = self
            .client
            .send_request(serde_json::from_value(json!({"method":"server/discover","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}}})).unwrap())
            .await
            .unwrap_or_else(|_| panic!("discovery failed"));
        serde_json::to_value(response).unwrap()
    }
    async fn stop(self) {
        self.client
            .cancel()
            .await
            .unwrap_or_else(|_| panic!("client shutdown failed"));
        assert!(
            fs::read(self.stderr.path()).unwrap().is_empty(),
            "unexpected server diagnostic"
        );
    }
    fn kill(&self) {
        // SAFETY: this is the PID obtained from this fixture's owned child transport.
        assert!(unsafe { libc::kill(self.pid as i32, libc::SIGKILL) } == 0);
    }
}
fn request(label: &str) -> Value {
    json!({"name":"apply-effect.v1","arguments":{"label":label},"_meta":{
        "io.modelcontextprotocol/protocolVersion":"2026-07-28",
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}},
        format!("{PREFIX}instance"):"target-marker",
        format!("{PREFIX}operation"):format!("{ADDRESS}#apply-effect.v1"),
        format!("{PREFIX}invocation"):"invocation-marker",
        format!("{PREFIX}caller"):"caller-marker",
        format!("{PREFIX}idempotencyKey"):"key-marker",
        format!("{PREFIX}conduit"):"conduit-marker",
        format!("{PREFIX}revision"):"revision-marker",
        format!("{PREFIX}filters"):[],format!("{PREFIX}config"):{}
    }})
}
fn plain(label: &str, op: &str) -> Value {
    json!({"name":op,"arguments":{"label":label},"_meta":{"io.modelcontextprotocol/clientCapabilities":{}}})
}
async fn wire(handle: RequestHandle<RoleClient>) -> Value {
    let result = handle
        .await_response()
        .await
        .unwrap_or_else(|_| panic!("MCP request unexpectedly failed"));
    serde_json::to_value(result).unwrap()
}
async fn success(handle: RequestHandle<RoleClient>, op: &str, aware: bool) -> Value {
    let result = wire(handle).await;
    assert!(result["isError"] == false);
    if aware {
        assert!(result["_meta"][OUTCOME] == "succeeded");
    }
    assert!(
        jsonschema::is_valid(&SCHEMAS[op]["outputSchema"], &result["structuredContent"]),
        "output failed contract validation"
    );
    result["structuredContent"].clone()
}
async fn error(handle: RequestHandle<RoleClient>, code: i32) -> ErrorData {
    match handle.await_response().await {
        Err(ServiceError::McpError(error)) => {
            assert!(
                error.code.0 == code,
                "protocol error code was {}, expected {}",
                error.code.0,
                code
            );
            error
        }
        _ => panic!("expected protocol error"),
    }
}

#[tokio::test]
async fn discovery_and_extension_errors_perform_nothing() {
    for item in ["plain", "keyed", "read"] {
        let f = Fixture::new();
        let p = Process::start(&f, item, "complete", false).await;
        let discovery = p.discover().await;
        let text = discovery.to_string();
        assert!(text.contains(EXTENSION) == (item != "plain"));
        assert!(!text.contains("tasks"));
        let tools = p
            .client
            .list_all_tools()
            .await
            .unwrap_or_else(|_| panic!("list tools failed"));
        let names: Vec<_> = tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(
            names
                == match item {
                    "plain" => vec!["apply-effect", "start-model-work"],
                    "keyed" => vec!["apply-effect.v1"],
                    _ => vec!["read-effects.v1"],
                }
        );
        if item != "plain" {
            assert!(text.contains("v0"));
            for revision in [None, Some("v999")] {
                let mut call = request("discovery");
                if item == "read" {
                    call["name"] = json!("read-effects.v1");
                }
                if let Some(revision) = revision {
                    call["_meta"]["io.modelcontextprotocol/clientCapabilities"]["extensions"]
                        [EXTENSION]["revision"] = json!(revision);
                } else {
                    call["_meta"]["io.modelcontextprotocol/clientCapabilities"] = json!({});
                }
                let error = error(p.begin(call).await, -32021).await;
                assert!(
                    error.data.unwrap()["requiredCapabilities"]["extensions"][EXTENSION]["revision"]
                        == "v0"
                );
            }
        }
        let mut unknown = request("unknown");
        unknown["name"] = json!("missing-tool");
        error(p.begin(unknown).await, -32602).await;
        assert!(fs::metadata(&f.ledger).unwrap().len() == 0);
        p.stop().await;
    }
}

#[tokio::test]
async fn e01_both_operations_commit_partial_before_protocol_error() {
    for op in ["apply-effect", "start-model-work"] {
        let f = Fixture::new();
        let p = Process::start(&f, "plain", "partial-error", true).await;
        let mut held = p.begin(plain("partial", op)).await;
        let first = f.reached("partial", "before-apply", 1).await;
        assert!(f.read()["records"].as_array().unwrap().is_empty());
        f.release(&first, "proceed").await;
        let last = f.reached("partial", "after-commit", 1).await;
        let read = f.cli();
        assert!(read["records"][0]["state"] == "partial");
        assert!(read["records"][0]["parts"] == json!([1]));
        assert!(read["calls"][0]["reply"] == "none");
        for field in ["instance", "caller", "key", "invocation"] {
            assert!(read["calls"][0][field].is_null());
        }
        assert!(matches!(
            held.rx.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        f.release(&last, "proceed").await;
        let reply = error(held, -32021).await;
        assert!(reply.data.unwrap()["requiredCapabilities"] == json!({"elicitation":{"form":{}}}));
        assert!(f.read()["calls"][0]["reply"] == "released");
        assert!(fs::metadata(&f.ledger).unwrap().mode() & 0o777 == 0o600);
        p.stop().await;
    }
}

#[tokio::test]
async fn required_metadata_and_filters_are_checked_without_writes() {
    for item in ["keyed", "read"] {
        let f = Fixture::new();
        let p = Process::start(&f, item, "complete", false).await;
        let mut base = request("metadata");
        if item == "read" {
            base["name"] = json!("read-effects.v1");
            base["_meta"][format!("{PREFIX}operation")] =
                json!(format!("{ADDRESS}#read-effects.v1"));
        }
        for key in [
            "instance",
            "operation",
            "caller",
            "invocation",
            "revision",
            "config",
            "filters",
        ] {
            for missing in [true, false] {
                let mut call = base.clone();
                let keyname = format!("{PREFIX}{key}");
                if missing {
                    call["_meta"].as_object_mut().unwrap().remove(&keyname);
                } else {
                    call["_meta"][&keyname] = Value::Null;
                }
                let result = wire(p.begin(call).await).await;
                assert!(result["isError"] == true && result["_meta"][OUTCOME] == "failed");
                assert!(result["content"][0]["text"].as_str().unwrap().contains(key));
            }
        }
        for key in ["idempotencyKey", "conduit"] {
            for value in [json!(""), json!(0), Value::Null] {
                let mut call = base.clone();
                call["_meta"][format!("{PREFIX}{key}")] = value;
                let result = wire(p.begin(call).await).await;
                assert!(result["_meta"][OUTCOME] == "failed" && result["isError"] == true);
                assert!(result["content"][0]["text"].as_str().unwrap().contains(key));
            }
        }
        let filter = json!({"requirement":"example.org/unsupported#filter.v1","arguments":{}});
        let mut call = base.clone();
        call["_meta"][format!("{PREFIX}filters")] = json!([filter]);
        let result = wire(p.begin(call).await).await;
        assert!(result["_meta"][REASON] == "unavailable" && result["isError"] == true);
        assert!(result["_meta"].get(OUTCOME).is_none());
        assert!(result["_meta"][RESTRICTION] == filter);
        for label in [
            json!(""),
            json!("Upper"),
            json!("-bad"),
            json!("a".repeat(64)),
            Value::Null,
        ] {
            let mut call = base.clone();
            call["arguments"]["label"] = label;
            assert!(wire(p.begin(call).await).await["_meta"][OUTCOME] == "failed");
        }
        assert!(fs::metadata(&f.ledger).unwrap().len() == 0);
        p.stop().await;
    }
}

#[tokio::test]
async fn keyed_scope_attempt_counts_and_unkeyed_new_work() {
    let f = Fixture::new();
    let p = Process::start(&f, "keyed", "complete", false).await;
    let base = request("scope");
    let one = success(p.begin(base.clone()).await, "apply-effect.v1", true).await;
    let two = success(p.begin(base.clone()).await, "apply-effect.v1", true).await;
    assert!(one == two);
    let mut another = base.clone();
    another["_meta"][format!("{PREFIX}conduit")] = json!("another-conduit-marker");
    let three = success(p.begin(another).await, "apply-effect.v1", true).await;
    assert!(three == one);
    let snapshot = f.read();
    assert!(snapshot["records"].as_array().unwrap().len() == 1);
    let counts: Vec<_> = snapshot["calls"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["attempt"].as_u64().unwrap())
        .collect();
    assert!(counts == vec![1, 2, 3]);
    assert!(
        snapshot["calls"][1]["deduplicated"] == true
            && snapshot["calls"][2]["deduplicated"] == true
    );
    for field in ["caller", "invocation", "instance", "idempotencyKey"] {
        let mut call = base.clone();
        call["_meta"][format!("{PREFIX}{field}")] = json!("another-marker");
        let result = success(p.begin(call).await, "apply-effect.v1", true).await;
        assert!(result != one);
    }
    let mut unkeyed = base.clone();
    unkeyed["_meta"]
        .as_object_mut()
        .unwrap()
        .remove(&format!("{PREFIX}idempotencyKey"));
    let a = success(p.begin(unkeyed.clone()).await, "apply-effect.v1", true).await;
    let b = success(p.begin(unkeyed).await, "apply-effect.v1", true).await;
    assert!(a != b);
    assert!(f.read()["records"].as_array().unwrap().len() == 7);
    p.stop().await;
}

#[tokio::test]
async fn concurrent_held_attempts_apply_one_effect() {
    let f = Fixture::new();
    // Separate processes prove the lock, rather than an in-process mutex, serializes application.
    let p = Process::start(&f, "keyed", "complete", true).await;
    let q = Process::start(&f, "keyed", "complete", true).await;
    let one = p.begin(request("concurrent")).await;
    let first = f.reached("concurrent", "before-apply", 1).await;
    let two = q.begin(request("concurrent")).await;
    let second = f.reached("concurrent", "before-apply", 2).await;
    assert!(f.read()["records"].as_array().unwrap().is_empty());
    let ((), ()) = tokio::join!(f.release(&first, "proceed"), f.release(&second, "proceed"));
    let (a, b) = tokio::join!(
        f.reached("concurrent", "after-commit", 1),
        f.reached("concurrent", "after-commit", 2)
    );
    assert!(f.read()["records"].as_array().unwrap().len() == 1);
    let ((), ()) = tokio::join!(f.release(&a, "proceed"), f.release(&b, "proceed"));
    let a = success(one, "apply-effect.v1", true).await;
    let b = success(two, "apply-effect.v1", true).await;
    assert!(a == b);
    let calls = f.read()["calls"].as_array().unwrap().clone();
    assert!(calls.iter().filter(|c| c["deduplicated"] == true).count() == 1);
    p.stop().await;
    q.stop().await;
}

#[tokio::test]
async fn partial_record_is_completed_without_repeating_first_part() {
    let f = Fixture::new();
    let p = Process::start(&f, "keyed", "complete", true).await;
    let held = p.begin(request("seeded")).await;
    let first = f.reached("seeded", "before-apply", 1).await;
    // Seed exactly the durable boundary a stopped writer leaves between its two fsyncs.
    let mut line = f.read()["calls"][0].clone();
    for key in ["parts", "deduplicated", "reply", "answer"] {
        line.as_object_mut().unwrap().remove(key);
    }
    line["version"] = json!(1);
    line["kind"] = json!("entry");
    line["record"] = json!("seeded-record");
    line["part"] = json!(1);
    line["of"] = json!(2);
    ledger::Ledger::open(&f.ledger, true)
        .unwrap()
        .append(&line)
        .unwrap();
    f.release(&first, "proceed").await;
    let last = f.reached("seeded", "after-commit", 1).await;
    f.release(&last, "proceed").await;
    let result = success(held, "apply-effect.v1", true).await;
    assert!(result["record"] == "seeded-record");
    let lines = ledger::Ledger::open(&f.ledger, false)
        .unwrap()
        .lines()
        .unwrap()
        .0;
    for part in [1, 2] {
        assert!(
            lines
                .iter()
                .filter(|v| v["kind"] == "entry" && v["part"] == part)
                .count()
                == 1
        );
    }
    assert!(f.read()["records"][0]["state"] == "whole");
    p.stop().await;
}

#[tokio::test]
async fn restart_after_commit_reuses_record_and_preserves_open_receipt() {
    let f = Fixture::new();
    let p = Process::start(&f, "keyed", "complete", true).await;
    let held = p.begin(request("restart")).await;
    let first = f.reached("restart", "before-apply", 1).await;
    f.release(&first, "proceed").await;
    f.reached("restart", "after-commit", 1).await;
    let before = f.read();
    assert!(before["records"][0]["state"] == "whole" && before["calls"][0]["reply"] == "none");
    p.kill();
    p.stop().await;
    drop(held); // The child is already stopped; no held request is timed out or cancelled.
    let p = Process::start(&f, "keyed", "complete", true).await;
    let result = f.finish(&p, request("restart"), "restart", 2).await;
    assert!(result["record"] == before["records"][0]["record"]);
    let read = f.read();
    assert!(read["records"].as_array().unwrap().len() == 1);
    assert!(
        read["calls"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v["reply"] == "none")
    );
    assert!(
        read["calls"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v["deduplicated"] == true)
    );
    p.stop().await;
}

#[tokio::test]
async fn identical_plain_calls_are_new_effects() {
    let f = Fixture::new();
    let p = Process::start(&f, "plain", "complete", false).await;
    let a = success(
        p.begin(plain("plain", "apply-effect")).await,
        "apply-effect.v1",
        false,
    )
    .await;
    let b = success(
        p.begin(plain("plain", "apply-effect")).await,
        "apply-effect.v1",
        false,
    )
    .await;
    assert!(a != b);
    let read = f.read();
    assert!(read["records"].as_array().unwrap().len() == 2);
    assert!(read["calls"][1]["attempt"] == 2);
    p.stop().await;
}

#[tokio::test]
async fn held_and_dropped_calls_do_not_hold_up_later_calls() {
    for barrier in ["before-apply", "after-commit"] {
        let f = Fixture::new();
        let p = Process::start(&f, "keyed", "complete", true).await;
        let mut held = p.begin(request("dropped")).await;
        let first = f.reached("dropped", "before-apply", 1).await;
        let reached = if barrier == "after-commit" {
            f.release(&first, "proceed").await;
            f.reached("dropped", "after-commit", 1).await
        } else {
            first
        };
        // An unrelated call completes while the first still has no release file.
        let mut later = request("later");
        later["_meta"][format!("{PREFIX}invocation")] = json!("later-invocation-marker");
        let output = f.finish(&p, later, "later", 1).await;
        assert!(output["record"].is_string());
        assert!(matches!(
            held.rx.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        assert!(
            f.read()["calls"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v["label"] == "dropped" && v["reply"] == "none")
        );
        f.release(&reached, "drop").await;
        assert!(matches!(
            held.rx.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        let read = f.read();
        assert!(
            read["calls"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v["label"] == "dropped" && v["reply"] == "dropped")
        );
        assert!(
            read["records"].as_array().unwrap().len()
                == if barrier == "before-apply" { 1 } else { 2 }
        );
        let mut after_drop = request("after-drop");
        after_drop["_meta"][format!("{PREFIX}invocation")] = json!("after-drop-invocation-marker");
        f.finish(&p, after_drop, "after-drop", 1).await;
        assert!(matches!(
            held.rx.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        p.stop().await;
        drop(held);
    }
}

#[tokio::test]
async fn invalid_or_unreadable_release_drops_with_control_lost() {
    for unreadable in [true, false] {
        let f = Fixture::new();
        let p = Process::start(&f, "plain", "complete", true).await;
        let mut held = p.begin(plain("control-lost", "apply-effect")).await;
        let reached = f.reached("control-lost", "before-apply", 1).await;
        if unreadable {
            // A directory makes read fail even when tests are run by a privileged user.
            fs::create_dir(reached.with_extension("release")).unwrap();
            f.acknowledged(&reached).await;
        } else {
            f.release(&reached, "invalid-verdict").await;
        }
        let read = f.read();
        assert!(read["records"].as_array().unwrap().is_empty());
        assert!(read["calls"][0]["reply"] == "dropped");
        let lines = ledger::Ledger::open(&f.ledger, false)
            .unwrap()
            .lines()
            .unwrap()
            .0;
        assert!(lines.iter().any(|l| l["kind"] == "control-lost"));
        assert!(matches!(
            held.rx.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        p.stop().await;
        drop(held);
    }
}

#[tokio::test]
async fn cancellation_after_commit_is_not_a_released_receipt() {
    let f = Fixture::new();
    let p = Process::start(&f, "keyed", "complete", true).await;
    let held = p.begin(request("cancelled")).await;
    let first = f.reached("cancelled", "before-apply", 1).await;
    f.release(&first, "proceed").await;
    let last = f.reached("cancelled", "after-commit", 1).await;
    held.cancel(Some("test cancellation".into()))
        .await
        .unwrap_or_else(|_| panic!("cancellation failed"));
    // A later round trip orders notification processing before the controller releases the call.
    p.discover().await;
    f.release(&last, "proceed").await;
    let read = f.read();
    assert!(read["records"][0]["state"] == "whole");
    assert!(read["calls"][0]["reply"] == "cancelled");
    let lines = ledger::Ledger::open(&f.ledger, false)
        .unwrap()
        .lines()
        .unwrap()
        .0;
    assert!(
        !lines
            .iter()
            .any(|l| l["kind"] == "reply" && l["reply"] == "released")
    );
    p.stop().await;
}

#[tokio::test]
async fn reader_and_cli_share_readback_including_torn_and_open_lines() {
    let f = Fixture::new();
    let plain_server = Process::start(&f, "plain", "partial-error", false).await;
    error(
        plain_server
            .begin(plain("partial", "start-model-work"))
            .await,
        -32021,
    )
    .await;
    plain_server.stop().await;
    let keyed = Process::start(&f, "keyed", "complete", true).await;
    let whole = f.finish(&keyed, request("whole"), "whole", 1).await;
    assert!(whole["record"].is_string());
    let open = keyed.begin(request("open")).await;
    f.reached("open", "before-apply", 2).await;
    use std::io::Write;
    let mut file = fs::OpenOptions::new().append(true).open(&f.ledger).unwrap();
    file.write_all(b"{\"torn\":").unwrap();
    file.sync_all().unwrap();
    let reader = Process::start(&f, "read", "complete", false).await;
    let mut call = request("whole");
    call["name"] = json!("read-effects.v1");
    call["_meta"][format!("{PREFIX}operation")] = json!(format!("{ADDRESS}#read-effects.v1"));
    call["arguments"] = json!({});
    let output = success(reader.begin(call.clone()).await, "read-effects.v1", true).await;
    assert!(output == f.cli());
    assert!(output["invalidLines"].as_array().unwrap().len() == 1);
    assert!(
        output["records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["state"] == "partial")
    );
    assert!(
        output["records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["state"] == "whole")
    );
    assert!(
        output["calls"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["reply"] == "none" && r["answer"].is_null())
    );
    call["arguments"] = json!({"label":"absent"});
    let absent = success(reader.begin(call.clone()).await, "read-effects.v1", true).await;
    assert!(absent["label"] == "absent" && absent["records"].as_array().unwrap().is_empty());
    call["arguments"] = json!({"label":"whole"});
    let filtered = success(reader.begin(call).await, "read-effects.v1", true).await;
    assert!(filtered == ledger::readback(&f.ledger, Some("whole")).unwrap());
    reader.stop().await;
    keyed.stop().await;
    drop(open);
    // Subsequent appends preserve the torn line's number rather than silently merging it.
    let keyed = Process::start(&f, "keyed", "complete", false).await;
    success(keyed.begin(request("later")).await, "apply-effect.v1", true).await;
    assert!(f.cli()["invalidLines"] == output["invalidLines"]);
    keyed.stop().await;
}

#[test]
fn startup_diagnostics_name_configuration_variables_without_values() {
    let f = Fixture::new();
    let cases = [
        ("S4_EFFECT_LEDGER", "relative-ledger"),
        ("S4_EFFECT_CONTROL", "missing-control"),
        ("S4_EFFECT_MODE", "invalid-mode"),
    ];
    for (variable, value) in cases {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_scenario-s4-effect"))
            .arg("plain")
            .env_clear()
            .env("S4_EFFECT_LEDGER", &f.ledger)
            .env("S4_EFFECT_MODE", "complete")
            .env(variable, value)
            .output()
            .unwrap();
        assert!(!output.status.success());
        let diagnostic = String::from_utf8(output.stderr).unwrap();
        assert!(diagnostic.contains(variable));
        assert!(!diagnostic.contains(value));
        assert!(output.stdout.is_empty());
    }
    fs::set_permissions(&f.control, fs::Permissions::from_mode(0o770)).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_scenario-s4-effect"))
        .arg("keyed")
        .env_clear()
        .env("S4_EFFECT_LEDGER", &f.ledger)
        .env("S4_EFFECT_CONTROL", &f.control)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("S4_EFFECT_CONTROL")
    );
    assert!(f.dir.path().exists());
}
