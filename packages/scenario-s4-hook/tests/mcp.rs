use rmcp::{
    RoleClient, ServiceExt,
    service::{PeerRequestOptions, RequestHandle, RunningService},
};
use scenario_s4_hook::{
    ADDRESS, BINARY, CORE, EXTENSION, GateVerdict, HookServer, OUTCOME, PREFIX, SCHEMAS, ledger,
};
use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    process::Stdio,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

struct Process {
    client: RunningService<RoleClient, ()>,
    child: tokio::process::Child,
    stdout: Arc<Mutex<Vec<u8>>>,
    stderr: tempfile::NamedTempFile,
    pump: tokio::task::JoinHandle<()>,
}
impl Process {
    async fn start(mode: &str) -> Self {
        let stderr = tempfile::NamedTempFile::new().unwrap();
        let mut child = tokio::process::Command::new(env!("CARGO_BIN_EXE_scenario-s4-hook"))
            .arg(mode)
            .env_clear()
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::from(stderr.reopen().unwrap()))
            .spawn()
            .unwrap();
        let input = child.stdin.take().unwrap();
        let output = child.stdout.take().unwrap();
        let stdout = Arc::new(Mutex::new(Vec::new()));
        let capture = stdout.clone();
        let (mut relay, observed) = tokio::io::duplex(65536);
        let pump = tokio::spawn(async move {
            let mut reader = BufReader::new(output);
            loop {
                let mut bytes = Vec::new();
                if reader.read_until(b'\n', &mut bytes).await.unwrap_or(0) == 0 {
                    break;
                }
                capture.lock().unwrap().extend_from_slice(&bytes);
                if relay.write_all(&bytes).await.is_err() {
                    break;
                }
            }
        });
        let client = ().serve((observed, input)).await.unwrap();
        Self {
            client,
            child,
            stdout,
            stderr,
            pump,
        }
    }
    async fn send(&self, params: Value) -> RequestHandle<RoleClient> {
        send(&self.client, params).await
    }
    async fn call(&self, params: Value) -> Value {
        reply(self.send(params).await).await
    }
    async fn close(mut self, marker: Option<&str>) {
        self.client.cancel().await.unwrap();
        let _ = self.child.kill().await;
        let _ = self.child.wait().await;
        let _ = self.pump.await;
        if let Some(marker) = marker {
            assert!(!String::from_utf8_lossy(&self.stdout.lock().unwrap()).contains(marker));
            assert!(
                !fs::read_to_string(self.stderr.path())
                    .unwrap()
                    .contains(marker)
            );
        }
    }
}
async fn send(client: &RunningService<RoleClient, ()>, params: Value) -> RequestHandle<RoleClient> {
    let mut params = params;
    let meta = params.as_object_mut().unwrap().remove("_meta").unwrap();
    let request = serde_json::from_value(json!({"method":"tools/call","params":params})).unwrap();
    client
        .send_request_with_option(
            request,
            PeerRequestOptions::no_options().with_meta(serde_json::from_value(meta).unwrap()),
        )
        .await
        .unwrap()
}
async fn reply(handle: RequestHandle<RoleClient>) -> Value {
    let wire = serde_json::to_value(handle.await_response().await.unwrap()).unwrap();
    // Untagged ServerResult serializes to its tool-result body.
    let state = wire["_meta"][OUTCOME].as_str();
    assert!(wire["isError"] == (state != Some("succeeded")));
    if state == Some("succeeded") && wire.get("structuredContent").is_some() {
        let validator =
            jsonschema::validator_for(&SCHEMAS["read-hook-ledger.v1"]["outputSchema"]).unwrap();
        assert!(validator.is_valid(&wire["structuredContent"]));
    }
    wire
}
struct Lab {
    dir: tempfile::TempDir,
    ledger: PathBuf,
    control: PathBuf,
}
impl Lab {
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
    fn request(&self, controlled: bool) -> Value {
        let mut config = json!({"ledger":self.ledger,"blockedBy":null});
        if controlled {
            config["control"] = json!(self.control);
        }
        request("setup-hook.v1", config, json!({}))
    }
    fn read(&self) -> Value {
        let v = ledger::readback(&self.ledger, None).unwrap();
        assert!(
            jsonschema::validator_for(&SCHEMAS["read-hook-ledger.v1"]["outputSchema"])
                .unwrap()
                .is_valid(&v)
        );
        v
    }
    fn lines(&self) -> Vec<Value> {
        fs::read_to_string(&self.ledger)
            .unwrap()
            .lines()
            .filter_map(|s| serde_json::from_str(s).ok())
            .collect()
    }
    async fn reached(&self, barrier: &str, attempt: u64) -> PathBuf {
        eventually(|| {
            fs::read_dir(&self.control)
                .unwrap()
                .filter_map(Result::ok)
                .find_map(|entry| {
                    if !entry.file_name().to_string_lossy().ends_with(".reached") {
                        return None;
                    }
                    let v: Value =
                        serde_json::from_slice(&fs::read(entry.path()).unwrap()).unwrap();
                    if v["barrier"] == barrier && v["call"]["attempt"] == attempt {
                        assert!(v["version"] == 1 && v["package"] == BINARY);
                        assert!(v["call"]["instance"] == "instance-a");
                        assert!(
                            v["id"]
                                .as_str()
                                .unwrap()
                                .bytes()
                                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
                        );
                        assert!(fs::metadata(entry.path()).unwrap().mode() & 0o777 == 0o600);
                        Some(entry.path().with_extension("release"))
                    } else {
                        None
                    }
                })
        })
        .await
    }
    async fn release(&self, path: &Path, verdict: &str) {
        let temp = path.with_extension("controller-tmp");
        fs::write(&temp, serde_json::to_vec(&json!(verdict)).unwrap()).unwrap();
        fs::rename(temp, path).unwrap();
        eventually(|| path.with_extension("acknowledged").exists().then_some(())).await;
        let v: Value =
            serde_json::from_slice(&fs::read(path.with_extension("acknowledged")).unwrap())
                .unwrap();
        assert!(v["version"] == 1);
        assert!(
            self.lines()
                .iter()
                .any(|v| v["kind"] == "released" && v["outcome"] == verdict)
        );
    }
    async fn complete(&self, attempt: u64) {
        let before = self.reached("before-apply", attempt).await;
        self.release(&before, "proceed").await;
        let reply = self.reached("before-reply", attempt).await;
        self.release(&reply, "proceed").await;
    }
}
fn request(name: &str, config: Value, args: Value) -> Value {
    json!({"name":name,"arguments":args,"_meta":{
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}},
        format!("{PREFIX}instance"):"instance-a",format!("{PREFIX}operation"):format!("{}#{name}",if name=="setup-hook.v1"{CORE}else{ADDRESS}),
        format!("{PREFIX}invocation"):"invocation-a",format!("{PREFIX}caller"):"caller-a",format!("{PREFIX}idempotencyKey"):"hook-key",
        format!("{PREFIX}revision"):"revision-a",format!("{PREFIX}filters"):[],format!("{PREFIX}config"):config}})
}
// Poll evidence, with a deadline only to fail broken tests. Never order events by elapsed time.
async fn eventually<T>(mut probe: impl FnMut() -> Option<T>) -> T {
    tokio::time::timeout(Duration::from_secs(15), async {
        loop {
            if let Some(v) = probe() {
                return v;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("evidence did not arrive")
}
#[tokio::test]
async fn discovery_and_invalid_calls_do_nothing() {
    let l = Lab::new();
    let p = Process::start("hook").await;
    let discover=serde_json::to_value(p.client.discover(serde_json::from_value(json!({"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}})).unwrap()).await.unwrap()).unwrap();
    assert!(discover["capabilities"]["extensions"][EXTENSION]["revision"] == "v0");
    let tools = p.client.list_all_tools().await.unwrap();
    assert!(tools.len() == 2);
    assert!(
        tools
            .iter()
            .all(|t| ["setup-hook.v1", "set-blocked-by.v1"].contains(&t.name.as_ref()))
    );
    for revision in [None, Some("v-next")] {
        let mut r = l.request(false);
        if let Some(revision) = revision {
            r["_meta"]["io.modelcontextprotocol/clientCapabilities"]["extensions"][EXTENSION]["revision"] =
                json!(revision);
        } else {
            r["_meta"]
                .as_object_mut()
                .unwrap()
                .remove("io.modelcontextprotocol/clientCapabilities");
        }
        let error = p.send(r).await.await_response().await.err().unwrap();
        let rmcp::service::ServiceError::McpError(error) = error else {
            panic!("expected capability error")
        };
        assert!(error.code.0 == -32021);
        assert!(error.message.contains("v0"));
        assert!(
            error.data.unwrap()["requiredCapabilities"]["extensions"][EXTENSION]["revision"]
                == "v0"
        );
    }
    let unsafe_dir = l.dir.path().join("unsafe");
    fs::create_dir(&unsafe_dir).unwrap();
    fs::set_permissions(&unsafe_dir, fs::Permissions::from_mode(0o777)).unwrap();
    let mut missing = l.request(false);
    missing["_meta"]
        .as_object_mut()
        .unwrap()
        .remove(&format!("{PREFIX}idempotencyKey"));
    let mut invalid = vec![(missing, "idempotencyKey")];
    for (place, value) in [
        ("ledger", json!("relative")),
        ("blockedBy", json!("relative")),
        ("control", json!(unsafe_dir)),
    ] {
        let mut r = l.request(false);
        r["_meta"][format!("{PREFIX}config")][place] = value;
        invalid.push((r, place));
    }
    let mut wrong = l.request(false);
    wrong["_meta"][format!("{PREFIX}operation")] = json!("wrong#operation.v1");
    invalid.push((wrong, "operation"));
    for (r, place) in invalid {
        let v = p.call(r).await;
        assert!(v["_meta"][OUTCOME] == "failed");
        assert!(v["content"][0]["text"].as_str().unwrap().contains(place));
        assert!(!l.ledger.exists());
    }
    let mut r = l.request(false);
    let filter = json!({"requirement":"example.org/filter#unsupported.v1","arguments":{}});
    r["_meta"][format!("{PREFIX}filters")] = json!([filter]);
    let v = p.call(r).await;
    assert!(v["_meta"][format!("{PREFIX}reason")] == "unavailable");
    assert!(v["_meta"][format!("{PREFIX}restriction")] == filter);
    assert!(v["_meta"].get(OUTCOME).is_none());
    assert!(!l.ledger.exists());
    p.close(None).await;
}
#[tokio::test]
async fn repeat_is_another_effect_and_keeps_its_caller() {
    let l = Lab::new();
    let p = Process::start("hook").await;
    let v = p.call(l.request(false)).await;
    assert!(v["_meta"][OUTCOME] == "succeeded");
    assert!(v.get("structuredContent").is_none());
    let mut r = l.request(false);
    r["_meta"][format!("{PREFIX}caller")] = json!("caller-b");
    p.call(r).await;
    let v = l.read();
    let a = v["attempts"].as_array().unwrap();
    assert!(a.len() == 2);
    assert!(a[0]["caller"] == "caller-a" && a[1]["caller"] == "caller-b");
    assert!(a[1]["attempt"] == 2);
    assert!(a[0]["effect"]["id"] != a[1]["effect"]["id"]);
    assert!(a.iter().all(|a| a["work"] == "ended"
        && a["reply"] == "released"
        && a["effect"]["state"] == "applied"));
    assert!(l.lines().iter().filter(|v| v["kind"] == "effect").count() == 2);
    assert!(fs::metadata(&l.ledger).unwrap().mode() & 0o777 == 0o600);
    p.close(None).await;
}
#[tokio::test]
async fn blocked_file_is_never_opened_and_control_only_accepts_a_change() {
    let l = Lab::new();
    let p = Process::start("hook").await;
    let block = l.dir.path().join("blocked");
    let marker = "private-file-content-marker";
    fs::write(&block, marker).unwrap();
    let mut r = l.request(false);
    r["_meta"][format!("{PREFIX}config")]["blockedBy"] = json!(block);
    let v = p.call(r.clone()).await;
    assert!(v["_meta"][OUTCOME] == "failed");
    let diagnostic = v["content"][0]["text"].as_str().unwrap();
    assert!(diagnostic.contains("blockedBy") && diagnostic.contains(block.to_str().unwrap()));
    assert!(!v.to_string().contains(marker));
    assert!(l.read()["attempts"][0]["effect"] == json!({"state":"none","cause":"blockedBy"}));
    let before = fs::read(&l.ledger).unwrap();
    let control = request(
        "set-blocked-by.v1",
        r["_meta"][format!("{PREFIX}config")].clone(),
        json!({"revision":"revision-a","blockedBy":null}),
    );
    let v = p.call(control).await;
    assert!(v["_meta"][OUTCOME] == "succeeded" && v["isError"] == false);
    assert!(fs::read(&l.ledger).unwrap() == before);
    assert!(p.call(l.request(false)).await["_meta"][OUTCOME] == "succeeded");
    assert!(block.exists());
    fs::remove_file(&block).unwrap();
    assert!(p.call(r.clone()).await["_meta"][OUTCOME] == "succeeded");
    fs::create_dir(&block).unwrap();
    assert!(p.call(r.clone()).await["_meta"][OUTCOME] == "succeeded");
    fs::remove_dir(&block).unwrap();
    let target = l.dir.path().join("target");
    fs::write(&target, marker).unwrap();
    std::os::unix::fs::symlink(&target, &block).unwrap();
    assert!(p.call(r).await["_meta"][OUTCOME] == "failed");
    assert!(!fs::read_to_string(&l.ledger).unwrap().contains(marker));
    p.close(Some(marker)).await;
}
#[tokio::test]
async fn lost_failed_reply_and_new_caller_share_one_process() {
    let l = Lab::new();
    let p = Process::start("hook").await;
    let first = p.send(l.request(true)).await;
    let before = l.reached("before-apply", 1).await;
    l.release(&before, "fail").await;
    let held = l.reached("before-reply", 1).await;
    let v = l.read();
    assert!(
        v["attempts"][0]["work"] == "ended"
            && v["attempts"][0]["effect"] == json!({"state":"none","cause":"controller"})
            && v["attempts"][0]["reply"] == "none"
    );
    l.release(&held, "proceed").await;
    assert!(reply(first).await["_meta"][OUTCOME] == "failed");
    assert!(l.read()["attempts"][0]["reply"] == "released");
    let mut r = l.request(true);
    r["_meta"][format!("{PREFIX}caller")] = json!("caller-b");
    let next = p.send(r).await;
    l.complete(2).await;
    assert!(reply(next).await["_meta"][OUTCOME] == "succeeded");
    let v = l.read();
    assert!(
        v["attempts"][1]["caller"] == "caller-b"
            && v["attempts"][1]["effect"]["state"] == "applied"
    );
    assert!(
        v["attempts"][0]["invocation"] == v["attempts"][1]["invocation"]
            && v["attempts"][0]["key"] == v["attempts"][1]["key"]
    );
    p.close(None).await;
}
#[tokio::test]
async fn dropped_calls_do_not_hold_up_later_calls() {
    for drop_at in ["before-apply", "before-reply"] {
        let l = Lab::new();
        let p = Process::start("hook").await;
        let mut held = p.send(l.request(true)).await;
        let before = l.reached("before-apply", 1).await;
        if drop_at == "before-apply" {
            l.release(&before, "drop").await;
        } else {
            l.release(&before, "proceed").await;
            let before = l.reached("before-reply", 1).await;
            l.release(&before, "drop").await;
        }
        eventually(|| (l.read()["attempts"][0]["reply"] == "dropped").then_some(())).await;
        let next = p.send(l.request(true)).await;
        l.complete(2).await;
        assert!(reply(next).await["_meta"][OUTCOME] == "succeeded");
        assert!(matches!(
            held.rx.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        let v = l.read();
        assert!(v["attempts"][0]["work"] == "ended");
        assert!(
            v["attempts"][0]["effect"]["state"]
                == if drop_at == "before-apply" {
                    "none"
                } else {
                    "applied"
                }
        );
        // Keep the handle alive through session shutdown; no request-level cancellation.
        p.close(None).await;
        drop(held);
    }
}
#[tokio::test]
async fn unreadable_or_invalid_release_records_control_lost() {
    for directory in [false, true] {
        let l = Lab::new();
        let p = Process::start("hook").await;
        let mut held = p.send(l.request(true)).await;
        let before = l.reached("before-apply", 1).await;
        if directory {
            fs::create_dir(&before).unwrap();
        } else {
            let temp = before.with_extension("bad-tmp");
            fs::write(&temp, b"\"invalid-verdict\"").unwrap();
            fs::rename(temp, &before).unwrap();
        }
        eventually(|| {
            l.lines()
                .iter()
                .any(|v| v["kind"] == "control-lost")
                .then_some(())
        })
        .await;
        eventually(|| (l.read()["attempts"][0]["reply"] == "dropped").then_some(())).await;
        assert!(!before.with_extension("acknowledged").exists());
        assert!(l.read()["attempts"][0]["effect"]["state"] == "none");
        let next = p.send(l.request(true)).await;
        l.complete(2).await;
        reply(next).await;
        assert!(matches!(
            held.rx.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        p.close(None).await;
        drop(held);
    }
}
#[tokio::test]
async fn peer_cancellation_before_release_is_recorded() {
    let l = Lab::new();
    let p = Process::start("hook").await;
    let held = p.send(l.request(true)).await;
    let before = l.reached("before-apply", 1).await;
    l.release(&before, "fail").await;
    let before = l.reached("before-reply", 1).await;
    let id = held.id.clone();
    held.cancel(Some("cancellation qualification".into()))
        .await
        .unwrap();
    // A subsequent round trip orders receipt of the cancellation before the controller release.
    p.client
        .send_request_with_option(
            serde_json::from_value(json!({"method":"ping"})).unwrap(),
            PeerRequestOptions::no_options(),
        )
        .await
        .unwrap()
        .await_response()
        .await
        .unwrap();
    l.release(&before, "proceed").await;
    eventually(|| (l.read()["attempts"][0]["reply"] == "cancelled").then_some(())).await;
    let next = p.send(l.request(true)).await;
    l.complete(2).await;
    reply(next).await;
    assert!(
        !l.lines()
            .iter()
            .any(|v| v["kind"] == "reply" && v["attempt"] == 1 && v["outcome"] == "released")
    );
    for line in p
        .stdout
        .lock()
        .unwrap()
        .split(|c| *c == b'\n')
        .filter(|l| !l.is_empty())
    {
        let v: Value = serde_json::from_slice(line).unwrap();
        assert!(v["id"] != json!(id));
    }
    p.close(None).await;
}
#[tokio::test]
async fn library_gate_can_fail_without_effect_and_names_its_package() {
    let l = Lab::new();
    let (client_io, server_io) = tokio::io::duplex(65536);
    let gate = Arc::new(|call: &ledger::Call| {
        assert!(
            call.instance == "instance-a"
                && call.invocation == "invocation-a"
                && call.caller == "caller-a"
                && call.attempt == 1
        );
        assert!(call.configuration["blockedBy"].is_null());
        GateVerdict::Fail("backing precondition absent".into())
    });
    let server = tokio::spawn(async move {
        HookServer::new("scenario-s9-test", false, Some(gate))
            .serve(server_io)
            .await
            .unwrap()
            .waiting()
            .await
            .unwrap();
    });
    let client = ().serve(client_io).await.unwrap();
    let v = reply(send(&client, l.request(false)).await).await;
    assert!(v["_meta"][OUTCOME] == "failed");
    let v = l.read();
    assert!(
        v["attempts"][0]["effect"] == json!({"state":"none","cause":"gate"})
            && v["attempts"][0]["package"] == "scenario-s9-test"
    );
    client.cancel().await.unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn readback_is_independent_and_never_infers_absence_for_open_work() {
    let l = Lab::new();
    let p = Process::start("hook").await;
    let held = p.send(l.request(true)).await;
    let before = l.reached("before-apply", 1).await;
    let v = l.read();
    assert!(
        v["attempts"][0]["work"] == "open"
            && v["attempts"][0]["effect"]["state"] == "not-known"
            && v["attempts"][0]["reply"] == "none"
    );
    let mut file = fs::OpenOptions::new().append(true).open(&l.ledger).unwrap();
    file.write_all(b"{\"torn\":").unwrap();
    file.sync_all().unwrap();
    let line_count = fs::read_to_string(&l.ledger).unwrap().lines().count();
    let v = l.read();
    assert!(v["invalidLines"] == json!([line_count]));
    let reader = Process::start("read").await;
    let config = json!({"ledger":l.ledger});
    let wire = reader
        .call(request("read-hook-ledger.v1", config.clone(), json!({})))
        .await;
    let output = tokio::process::Command::new(env!("CARGO_BIN_EXE_scenario-s4-hook"))
        .arg("readback")
        .arg(&l.ledger)
        .output()
        .await
        .unwrap();
    assert!(output.status.success());
    let cli: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(wire["structuredContent"] == cli && cli == v);
    let filtered = reader
        .call(request(
            "read-hook-ledger.v1",
            config,
            json!({"instance":"missing"}),
        ))
        .await;
    assert!(filtered["structuredContent"]["attempts"] == json!([]));
    assert!(filtered["structuredContent"]["invalidLines"] == json!([line_count]));
    l.release(&before, "proceed").await;
    let before = l.reached("before-reply", 1).await;
    l.release(&before, "proceed").await;
    reply(held).await;
    assert!(l.read()["invalidLines"] == json!([line_count]));
    reader.close(None).await;
    p.close(None).await;
}
#[tokio::test]
async fn shared_ledger_serializes_process_receipts_and_keeps_instances_apart() {
    let l = Lab::new();
    let a = Process::start("hook").await;
    let b = Process::start("hook").await;
    let first = a.send(l.request(true)).await;
    let before = l.reached("before-apply", 1).await;
    let second = b.send(l.request(true)).await;
    let second_before = l.reached("before-apply", 2).await;
    l.release(&second_before, "proceed").await;
    let end = l.reached("before-reply", 2).await;
    l.release(&end, "proceed").await;
    reply(second).await;
    l.release(&before, "proceed").await;
    let end = l.reached("before-reply", 1).await;
    l.release(&end, "proceed").await;
    reply(first).await;
    let mut another = l.request(false);
    another["_meta"][format!("{PREFIX}instance")] = json!("instance-b");
    b.call(another).await;
    let v = l.read();
    assert!(v["attempts"].as_array().unwrap().len() == 3);
    assert!(v["attempts"][2]["attempt"] == 1);
    assert!(
        ledger::readback(&l.ledger, Some("instance-a")).unwrap()["attempts"]
            .as_array()
            .unwrap()
            .len()
            == 2
    );
    a.close(None).await;
    b.close(None).await;
}
