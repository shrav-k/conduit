#![cfg(unix)]
mod support {
    pub mod tls;
}
use http_body_util::BodyExt;
use provider_local_process::{
    DRAIN_INTERVAL, FileStore, LocalBackend, Signaller, StopPipe, hostname, instance_key,
};
use rmcp::{
    RoleClient, ServiceExt,
    model::{CallToolRequestParams, CallToolResult},
    service::RunningService,
};
use runtime_provider_kit::{
    gate::{GateConfig, TlsGate},
    names, *,
};
use serde_json::{Value, json};
use std::{
    fs,
    os::unix::{
        fs::{OpenOptionsExt, PermissionsExt},
        net::UnixStream,
    },
    path::{Path, PathBuf},
    process::Stdio,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use support::tls::{Client as GateClient, Fixture, INSTANCE};
use tokio::{
    process::{Child, Command},
    time::{sleep, timeout},
};

const ID: &str = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
const SECOND: &str = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
const WAIT: Duration = Duration::from_secs(8);
type Client = RunningService<RoleClient, ()>;

fn temporary_state() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
    dir
}
fn synthetic() -> [(&'static str, &'static str); 6] {
    [
        ("HOME", "/synthetic/home"),
        ("LOGNAME", "synthetic-login"),
        ("PATH", "/synthetic/bin"),
        ("SHELL", "/synthetic/shell"),
        ("TERM", "synthetic-term"),
        ("USER", "synthetic-user"),
    ]
}
fn machine() -> Machine {
    Machine {
        hostname: hostname().unwrap(),
        labels: Default::default(),
    }
}
fn context() -> CallContext {
    CallContext {
        config: json!({}),
        revision: "r1".into(),
        machine: machine(),
    }
}
fn args(id: &str) -> Value {
    json!({"instance":id,"machine":machine()})
}
fn inputs(id: &str, fixture: &Fixture) -> Value {
    let mut value = args(id);
    value["item"] = json!({"implements":["#example.v1"],"command":[env!("CARGO_BIN_EXE_test-run")],"environment":{"ORDINARY":"/ordinary"}});
    value["environment"] = json!({"ORDINARY":"synthetic-ordinary"});
    // These are supplied too, for tests embedding the backend in this test process.
    for (name, value_) in synthetic() {
        value["environment"][name] = json!(value_);
    }
    value["secrets"] = json!({"environment":{"PRIVATE_TEST":secret()}});
    value["identity"] =
        json!({"environment":fixture.identity.environment,"files":fixture.identity.files});
    value
}
fn secret() -> String {
    ["synthetic", "private", "binding"].join("-")
}
fn call(name: &str, arguments: Value) -> CallToolRequestParams {
    serde_json::from_value(json!({"name":name,"arguments":arguments,"_meta":{
        names::key("config"): {}, names::key("revision"):"r1",
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{names::extension():{"revision":"v0"}}}
    }})).unwrap()
}
async fn invoke(client: &Client, name: &str, args: Value) -> CallToolResult {
    timeout(WAIT, client.call_tool(call(name, args)))
        .await
        .expect("provider call timed out")
        .expect("provider call failed")
}
#[track_caller]
fn outcome(result: &CallToolResult, expected: &str) {
    let value = serde_json::to_value(result).unwrap();
    assert!(
        value["_meta"][names::key("outcome")] == expected,
        "unexpected outcome (expected {expected})"
    );
    assert!(!value.to_string().contains(&secret()), "sensitive answer");
}
fn url(result: &CallToolResult) -> String {
    result.structured_content.as_ref().unwrap()["url"]
        .as_str()
        .unwrap()
        .into()
}
fn state(root: &Path, id: &str) -> PathBuf {
    root.join("instances").join(instance_key(id))
}
async fn ready(root: &Path, id: &str) -> Vec<i32> {
    let path = state(root, id).join("pids.json");
    timeout(WAIT, async {
        loop {
            if let Ok(bytes) = fs::read(&path)
                && let Ok(pids) = serde_json::from_slice::<Vec<i32>>(&bytes)
            {
                return pids;
            }
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("test run did not start")
}
fn alive(pid: i32) -> bool {
    // Zombies cannot execute; Linux test containers do not necessarily reap orphans promptly.
    #[cfg(target_os = "linux")]
    if fs::read_to_string(format!("/proc/{pid}/stat"))
        .ok()
        .is_some_and(|s| {
            s.rsplit_once(')')
                .is_some_and(|(_, rest)| rest.starts_with(" Z"))
        })
    {
        return false;
    }
    unsafe { libc::kill(pid, 0) == 0 }
}
async fn gone(pids: &[i32]) {
    timeout(WAIT, async {
        while pids.iter().any(|p| alive(*p)) {
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("a process survived group stop");
}
struct Provider {
    child: Child,
    client: Client,
    root: PathBuf,
    log: PathBuf,
}
impl Provider {
    async fn start(root: &Path) -> Self {
        let log = root.join("provider.log");
        let stderr = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .mode(0o600)
            .open(&log)
            .unwrap();
        let mut child = Command::new(env!("CARGO_BIN_EXE_provider-local-process"))
            .env_clear()
            .envs(synthetic())
            .env(names::STATE_DIR_VARIABLE, root)
            .env("UNREQUESTED_TEST_VALUE", "synthetic-unrequested")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(stderr)
            .kill_on_drop(true)
            .spawn()
            .unwrap();
        let transport = (child.stdout.take().unwrap(), child.stdin.take().unwrap());
        let client = timeout(WAIT, ().serve(transport))
            .await
            .unwrap()
            .expect("provider initialization failed");
        Self {
            child,
            client,
            root: root.into(),
            log,
        }
    }
    async fn kill(mut self) {
        self.child.kill().await.unwrap();
        self.child.wait().await.unwrap();
        let _ = self.client.cancel().await;
    }
    async fn close(mut self) {
        let _ = self.client.cancel().await;
        timeout(WAIT, self.child.wait()).await.unwrap().unwrap();
    }
    fn safe(&self, fixture: &Fixture, results: &[&CallToolResult]) {
        let mut bytes = fs::read(&self.log).unwrap();
        bytes.extend(fs::read(self.root.join("ledger.json")).unwrap());
        for result in results {
            bytes.extend(serde_json::to_vec(result).unwrap());
        }
        let text = String::from_utf8_lossy(&bytes);
        for value in fixture
            .identity
            .files
            .values()
            .chain(fixture.identity.environment.values())
            .chain(std::iter::once(&secret()))
        {
            assert!(
                !text.contains(value),
                "sensitive value outside private run material"
            );
        }
    }
}

#[tokio::test]
async fn real_run_gate_repeat_inspect_environment_and_group_stop() {
    let root = temporary_state();
    let provider = Provider::start(root.path()).await;
    let fixture = Fixture::new(INSTANCE);
    // Here the six permitted values must come from provider inheritance, not call inputs.
    let mut input = inputs(ID, &fixture);
    for (name, _) in synthetic() {
        input["environment"].as_object_mut().unwrap().remove(name);
    }
    let run = invoke(&provider.client, "run.v1", input.clone()).await;
    outcome(&run, "succeeded");
    let pids = ready(root.path(), ID).await;
    assert!(pids.iter().all(|p| alive(*p)));
    let again = invoke(&provider.client, "run.v1", input).await;
    outcome(&again, "succeeded");
    assert!(url(&again) == url(&run));
    assert!(
        fs::read_to_string(state(root.path(), ID).join("starts"))
            .unwrap()
            .lines()
            .count()
            == 1
    );
    let inspect = invoke(&provider.client, "inspect-run.v1", args(ID)).await;
    outcome(&inspect, "succeeded");
    assert!(inspect.structured_content.as_ref().unwrap()["state"] == "running");
    assert!(
        GateClient::connect(&url(&run), fixture.root.clone(), "spiffe://wrong/instance")
            .await
            .is_err()
    );
    let foreign = Fixture::new(INSTANCE);
    assert!(
        GateClient::connect(&url(&run), foreign.root, INSTANCE)
            .await
            .is_err()
    );
    let mut gate = GateClient::connect(&url(&run), fixture.root.clone(), INSTANCE)
        .await
        .unwrap();
    let refused = gate.send(hyper::Method::POST, "{}", None, None, None).await;
    assert!(refused.status() == 401);
    let response = gate.send(hyper::Method::POST,
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"test","version":"1"}}}"#,
        Some(&fixture.token()),None,None).await;
    assert!(response.status() == 200);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(String::from_utf8_lossy(&body).contains("test-run"));
    let env: Value =
        serde_json::from_slice(&fs::read(state(root.path(), ID).join("environment.json")).unwrap())
            .unwrap();
    let mut expected: Vec<&str> = synthetic()
        .into_iter()
        .map(|(name, _)| name)
        .chain([
            "ORDINARY",
            "PRIVATE_TEST",
            names::ENGINE_URL_VARIABLE,
            names::IDENTITY_DIR_VARIABLE,
            names::STATE_DIR_VARIABLE,
        ])
        .collect();
    let mut actual: Vec<&str> = env["names"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    expected.sort();
    actual.sort();
    assert_eq!(actual, expected);
    for (name, value) in synthetic() {
        assert!(
            env["synthetic"][name] == value,
            "inherited synthetic value differs"
        );
    }
    assert!(env["synthetic"]["ORDINARY"] == "synthetic-ordinary");
    let private = root.path().join("runs").join(instance_key(ID));
    assert!(fs::metadata(&private).unwrap().permissions().mode() & 0o777 == 0o700);
    assert!(
        fs::metadata(private.join("identity"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777
            == 0o700
    );
    for name in fixture.identity.files.keys() {
        let path = private.join("identity").join(name);
        assert!(fs::metadata(&path).unwrap().permissions().mode() & 0o777 == 0o600);
        assert!(
            fs::read_to_string(path).unwrap() == fixture.identity.files[name],
            "identity content differs"
        );
    }
    let stop = invoke(&provider.client, "stop.v1", args(ID)).await;
    outcome(&stop, "succeeded");
    gone(&pids).await;
    assert!(!private.exists());
    assert!(
        fs::metadata(state(root.path(), ID))
            .unwrap()
            .permissions()
            .mode()
            & 0o777
            == 0o700
    );
    provider.safe(&fixture, &[&run, &again, &inspect, &stop]);
    provider.close().await;
}

#[tokio::test]
async fn killed_provider_fences_descendants_and_restart_keeps_state() {
    let root = temporary_state();
    let fixture = Fixture::new(INSTANCE);
    let provider = Provider::start(root.path()).await;
    let run = invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await;
    outcome(&run, "succeeded");
    let pids = ready(root.path(), ID).await;
    let ledger = fs::read(root.path().join("ledger.json")).unwrap();
    assert!(!ledger.is_empty());
    fs::write(state(root.path(), ID).join("persistent"), b"retained").unwrap();
    provider.kill().await;
    gone(&pids).await;
    let provider = Provider::start(root.path()).await;
    let inspect = invoke(&provider.client, "inspect-run.v1", args(ID)).await;
    outcome(&inspect, "succeeded");
    assert!(inspect.structured_content.as_ref().unwrap()["state"] == "stopped");
    assert!(!root.path().join("runs").join(instance_key(ID)).exists());
    let records: Vec<RunRecord> =
        serde_json::from_slice(&fs::read(root.path().join("ledger.json")).unwrap()).unwrap();
    assert!(
        records.iter().any(|r| r.instance == ID
            && r.context.revision == "r1"
            && r.state == RunState::Stopped)
    );
    assert!(fs::read(state(root.path(), ID).join("persistent")).unwrap() == b"retained");
    outcome(
        &invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await,
        "succeeded",
    );
    assert!(
        fs::read_to_string(state(root.path(), ID).join("starts"))
            .unwrap()
            .lines()
            .count()
            == 2
    );
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "succeeded",
    );
    provider.close().await;
}

#[tokio::test]
async fn killed_guard_cannot_confirm_stop_or_admit_a_duplicate_after_restart() {
    let root = temporary_state();
    let fixture = Fixture::new(INSTANCE);
    let provider = Provider::start(root.path()).await;
    outcome(
        &invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await,
        "succeeded",
    );
    let pids = ready(root.path(), ID).await;
    // The live fixture is in the guard-led group. Kill only that guard, leaving
    // both the program and its child in the original group (neither calls setsid).
    let group = unsafe { libc::getpgid(pids[0]) };
    assert!(group > 1 && group != pids[0]);
    struct Cleanup(i32);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            if self.0 > 1 {
                unsafe { libc::kill(-self.0, libc::SIGKILL) };
            }
        }
    }
    let mut cleanup = Cleanup(group);
    assert!(unsafe { libc::kill(group, libc::SIGKILL) } == 0);
    // Wait for the provider to reap its guard and the flock to become available.
    timeout(WAIT, async {
        while alive(group) {
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert!(pids.iter().all(|pid| alive(*pid)));
    let inspect = invoke(&provider.client, "inspect-run.v1", args(ID)).await;
    outcome(&inspect, "succeeded");
    assert!(inspect.structured_content.as_ref().unwrap()["state"] == "unknown");
    outcome(
        &invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await,
        "unknown",
    );
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "unknown",
    );
    assert!(pids.iter().all(|pid| alive(*pid)));
    assert!(root.path().join("runs").join(instance_key(ID)).exists());
    provider.kill().await;

    // Recovery cannot turn guard death into a confirmed stop or trust a saved ID
    // to signal a group. The owed stop continues to block admission on this machine.
    let provider = Provider::start(root.path()).await;
    let inspect = invoke(&provider.client, "inspect-run.v1", args(ID)).await;
    outcome(&inspect, "succeeded");
    assert!(inspect.structured_content.as_ref().unwrap()["state"] == "unknown");
    outcome(
        &invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await,
        "unknown",
    );
    outcome(
        &invoke(&provider.client, "run.v1", inputs(SECOND, &fixture)).await,
        "failed",
    );
    assert!(!state(root.path(), SECOND).exists());
    assert!(
        fs::read_to_string(state(root.path(), ID).join("starts"))
            .unwrap()
            .lines()
            .count()
            == 1
    );
    assert!(pids.iter().all(|pid| alive(*pid)));

    // An external test owner terminates its known group. Only established absence
    // permits confirmation, identity cleanup, and another run of this instance.
    assert!(unsafe { libc::kill(-group, libc::SIGKILL) } == 0);
    cleanup.0 = 0;
    gone(&pids).await;
    timeout(WAIT, async {
        while unsafe { libc::kill(-group, 0) } == 0 {
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("terminated group did not disappear");
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "succeeded",
    );
    let inspect = invoke(&provider.client, "inspect-run.v1", args(ID)).await;
    assert!(inspect.structured_content.as_ref().unwrap()["state"] == "stopped");
    outcome(
        &invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await,
        "succeeded",
    );
    assert!(
        fs::read_to_string(state(root.path(), ID).join("starts"))
            .unwrap()
            .lines()
            .count()
            == 2
    );
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "succeeded",
    );
    provider.close().await;
}

#[tokio::test]
async fn declaration_refusals_and_path_safe_instance() {
    let root = temporary_state();
    let fixture = Fixture::new(INSTANCE);
    let provider = Provider::start(root.path()).await;
    for command in [json!(["relative/program"]), json!(["bare-program"])] {
        let mut input = inputs(ID, &fixture);
        input["item"]["command"] = command;
        outcome(&invoke(&provider.client, "run.v1", input).await, "failed");
    }
    let mut package = inputs(ID, &fixture);
    package["item"] = json!({"implements":["#example.v1"],"package":{"registryType":"npm","identifier":"unused","version":"1.0.0","fileSha256":"0".repeat(64),"transport":{"type":"stdio"}}});
    outcome(&invoke(&provider.client, "run.v1", package).await, "failed");
    let mut other = inputs(ID, &fixture);
    other["machine"]["hostname"] = json!(format!("{}-other", hostname().unwrap()));
    outcome(&invoke(&provider.client, "run.v1", other).await, "failed");
    assert!(!state(root.path(), ID).join("starts").exists());
    let run = invoke(&provider.client, "run.v1", inputs("../x", &fixture)).await;
    outcome(&run, "succeeded");
    let pids = ready(root.path(), "../x").await;
    let entries: Vec<_> = fs::read_dir(root.path().join("runs"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    assert!(entries == [std::ffi::OsString::from(instance_key("../x"))]);
    outcome(
        &invoke(&provider.client, "stop.v1", args("../x")).await,
        "succeeded",
    );
    gone(&pids).await;
    provider.close().await;
}

struct SwitchSignaller(AtomicBool);
impl Signaller for SwitchSignaller {
    fn stop(&self, pipe: &UnixStream) -> std::io::Result<()> {
        if self.0.load(Ordering::SeqCst) {
            Err(std::io::Error::other("injected signal failure"))
        } else {
            StopPipe.stop(pipe)
        }
    }
}
#[tokio::test]
async fn real_held_stop_blocks_every_new_run_until_carried_out() {
    let root = temporary_state();
    let signaller = Arc::new(SwitchSignaller(AtomicBool::new(false)));
    let backend = Arc::new(
        LocalBackend::with_signaller(
            root.path(),
            Path::new(env!("CARGO_BIN_EXE_provider-local-process")),
            signaller.clone(),
        )
        .unwrap(),
    );
    let store = Arc::new(FileStore::new(root.path()));
    let server = RuntimeServer::new(
        backend.clone(),
        store.clone(),
        Arc::new(TlsGate::new(GateConfig::default())),
        Options {
            max_lifetime: Duration::from_secs(30),
            drain_interval: DRAIN_INTERVAL,
        },
    )
    .await
    .unwrap();
    let (client, wire) = tokio::io::duplex(1024 * 1024);
    let serving = tokio::spawn(async move { server.serve_transport(wire).await });
    let client = ().serve(client).await.unwrap();
    let fixture = Fixture::new(INSTANCE);
    outcome(
        &invoke(&client, "run.v1", inputs(ID, &fixture)).await,
        "succeeded",
    );
    let pids = ready(root.path(), ID).await;
    signaller.0.store(true, Ordering::SeqCst);
    let stop = invoke(&client, "stop.v1", args(ID)).await;
    outcome(&stop, "unknown");
    assert!(backend.state(&context(), ID).await.unwrap() == RunState::Running);
    assert!(pids.iter().all(|p| alive(*p)));
    assert!(
        store
            .load()
            .await
            .unwrap()
            .iter()
            .any(|r| r.instance == ID && r.owed_stop)
    );
    let refused = invoke(&client, "run.v1", inputs(SECOND, &fixture)).await;
    outcome(&refused, "failed");
    let wire = serde_json::to_value(&refused).unwrap();
    let message = wire["content"][0]["text"].as_str().unwrap();
    assert!(message.contains(ID) && message.contains(&hostname().unwrap()));
    assert!(!state(root.path(), SECOND).exists());
    signaller.0.store(false, Ordering::SeqCst);
    outcome(
        &invoke(&client, "run.v1", inputs(SECOND, &fixture)).await,
        "succeeded",
    );
    gone(&pids).await;
    assert!(store.load().await.unwrap().iter().all(|r| !r.owed_stop));
    outcome(&invoke(&client, "stop.v1", args(SECOND)).await, "succeeded");
    client.cancel().await.unwrap();
    assert!(serving.await.unwrap().is_ok());
}

#[test]
fn state_store_reopens_identical_contents_and_missing_state_directory_is_explicit() {
    let root = temporary_state();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let record = RunRecord {
            instance: ID.into(),
            context: context(),
            state: RunState::Unknown,
            owed_stop: true,
        };
        FileStore::new(root.path()).save(&record).await.unwrap();
        let before = fs::read(root.path().join("ledger.json")).unwrap();
        let records = FileStore::new(root.path()).load().await.unwrap();
        assert!(records.len() == 1 && records[0].instance == ID && records[0].owed_stop);
        assert!(before == fs::read(root.path().join("ledger.json")).unwrap());
    });
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_provider-local-process"))
        .env_clear()
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains(names::STATE_DIR_VARIABLE));
}

#[tokio::test]
async fn backend_guard_enforces_lifetime_and_inventory_finds_orphans() {
    use tokio_util::sync::CancellationToken;
    let root = temporary_state();
    let fixture = Fixture::new(INSTANCE);
    let backend = LocalBackend::new(
        root.path(),
        Path::new(env!("CARGO_BIN_EXE_provider-local-process")),
    )
    .unwrap();
    let input: RunInput = serde_json::from_value(inputs(ID, &fixture)).unwrap();
    let cancel = CancellationToken::new();
    let staged = backend.stage(&context(), &input, &cancel).await.unwrap();
    let channel = backend
        .start(&context(), ID, staged, &cancel, Duration::from_secs(1))
        .await
        .unwrap();
    let pids = ready(root.path(), ID).await;
    let inventory = backend.inventory().await.unwrap();
    assert!(
        inventory.len() == 1
            && inventory[0].instance == ID
            && inventory[0].state == RunState::Running
    );
    // No server, gate, cancellation, or ledger exists in this test: the guard is the bound.
    gone(&pids).await;
    // The backend proves the group gone only once its last member is reaped, and a task of the
    // backend collects the guard. Until then it answers running or unknown, so wait for stopped.
    timeout(WAIT, async {
        while backend.state(&context(), ID).await.unwrap() != RunState::Stopped {
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the backend never proved the run's group gone");
    backend.stop(&context(), ID).await.unwrap();
    drop(channel);
    assert!(!root.path().join("runs").join(instance_key(ID)).exists());
}

#[tokio::test]
async fn cancelled_staging_and_failed_spawn_leave_no_private_identity() {
    use tokio_util::sync::CancellationToken;
    let root = temporary_state();
    let fixture = Fixture::new(INSTANCE);
    let backend = LocalBackend::new(
        root.path(),
        Path::new(env!("CARGO_BIN_EXE_provider-local-process")),
    )
    .unwrap();
    let mut value = inputs(ID, &fixture);
    value["item"]["command"] = json!(["/nonexistent-test-program"]);
    let input: RunInput = serde_json::from_value(value).unwrap();
    let cancel = CancellationToken::new();
    cancel.cancel();
    assert!(backend.stage(&context(), &input, &cancel).await.is_err());
    let cancel = CancellationToken::new();
    let staged = backend.stage(&context(), &input, &cancel).await.unwrap();
    drop(staged);
    let private = root.path().join("runs").join(instance_key(ID));
    assert!(!private.exists());
    let staged = backend.stage(&context(), &input, &cancel).await.unwrap();
    assert!(
        backend
            .start(&context(), ID, staged, &cancel, Duration::from_secs(5))
            .await
            .is_err()
    );
    backend.stop(&context(), ID).await.unwrap();
    assert!(!private.exists());
    assert!(backend.state(&context(), ID).await.unwrap() == RunState::Stopped);
}
