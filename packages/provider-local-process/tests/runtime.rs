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

const DURABLE_VARIABLE: &str = "LOCAL_PROCESS_DURABLE_DIR";

struct TestDirectories {
    _temp: tempfile::TempDir,
    state: PathBuf,
    durable: PathBuf,
}
impl TestDirectories {
    fn path(&self) -> &Path {
        &self.state
    }

    fn durable(&self) -> &Path {
        &self.durable
    }

    fn ledger_dir(&self) -> PathBuf {
        self.durable
            .join(instance_key(self.state.to_str().unwrap()))
    }
}
fn temporary_state() -> TestDirectories {
    let temp = tempfile::tempdir().unwrap();
    let instances = temp.path().join("instances");
    fs::create_dir(&instances).unwrap();
    let state = instances.join("provider");
    let durable = temp.path().join("durable");
    for path in [&state, &durable] {
        provider_local_process::private_directory(path).unwrap();
    }
    TestDirectories {
        _temp: temp,
        state,
        durable,
    }
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
/// Waits until the fixture has recorded `expected` starts, then returns the count read.
async fn starts(root: &Path, id: &str, expected: usize) -> usize {
    let path = state(root, id).join("starts");
    let read = || {
        fs::read_to_string(&path)
            .map(|s| s.lines().count())
            .unwrap_or(0)
    };
    let _ = timeout(WAIT, async {
        while read() < expected {
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await;
    read()
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
    ledger_dir: PathBuf,
    log: PathBuf,
}
impl Provider {
    async fn start(directories: &TestDirectories) -> Self {
        Self::start_in(directories.path(), directories.durable()).await
    }

    async fn start_in(root: &Path, durable: &Path) -> Self {
        Self::start_with_nice(root, durable, None).await
    }

    async fn start_with_nice(root: &Path, durable: &Path, nice: Option<i32>) -> Self {
        let log = root.join("provider.log");
        let stderr = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .mode(0o600)
            .open(&log)
            .unwrap();
        let mut command = Command::new(env!("CARGO_BIN_EXE_provider-local-process"));
        command
            .env_clear()
            .envs(synthetic())
            .env(names::STATE_DIR_VARIABLE, root)
            .env(DURABLE_VARIABLE, durable)
            .env("UNREQUESTED_TEST_VALUE", "synthetic-unrequested")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(stderr)
            .kill_on_drop(true);
        if let Some(nice) = nice {
            // SAFETY: setpriority is called only in the child before exec.
            unsafe {
                command.pre_exec(move || {
                    if libc::setpriority(libc::PRIO_PROCESS, 0, nice) != 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    Ok(())
                });
            }
        }
        let mut child = command.spawn().unwrap();
        let transport = (child.stdout.take().unwrap(), child.stdin.take().unwrap());
        let client = timeout(WAIT, ().serve(transport))
            .await
            .unwrap()
            .expect("provider initialization failed");
        Self {
            child,
            client,
            ledger_dir: durable.join(instance_key(root.to_str().unwrap())),
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
        bytes.extend(fs::read(self.ledger_dir.join("ledger.json")).unwrap());
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
    let provider = Provider::start(&root).await;
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
    assert_eq!(
        fs::read_to_string(state(root.path(), ID).join("starts"))
            .unwrap()
            .lines()
            .count(),
        1,
        "the repeated run.v1 returned the same URL, so the fixture must not have started again"
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
    let provider = Provider::start(&root).await;
    let run = invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await;
    outcome(&run, "succeeded");
    let pids = ready(root.path(), ID).await;
    let ledger = fs::read(root.ledger_dir().join("ledger.json")).unwrap();
    assert!(!ledger.is_empty());
    fs::write(state(root.path(), ID).join("persistent"), b"retained").unwrap();
    provider.kill().await;
    gone(&pids).await;
    let provider = Provider::start(&root).await;
    let inspect = invoke(&provider.client, "inspect-run.v1", args(ID)).await;
    outcome(&inspect, "succeeded");
    assert!(inspect.structured_content.as_ref().unwrap()["state"] == "stopped");
    assert!(!root.path().join("runs").join(instance_key(ID)).exists());
    let records: Vec<RunRecord> =
        serde_json::from_slice(&fs::read(root.ledger_dir().join("ledger.json")).unwrap()).unwrap();
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
    assert_eq!(
        starts(root.path(), ID, 2).await,
        2,
        "the second run.v1 succeeded, so the fixture must have started twice"
    );
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "succeeded",
    );
    provider.close().await;
}

struct SurvivingGroup(i32);
impl Drop for SurvivingGroup {
    fn drop(&mut self) {
        if self.0 > 1 {
            unsafe { libc::kill(-self.0, libc::SIGKILL) };
        }
    }
}
impl SurvivingGroup {
    fn end(&mut self) {
        assert!(unsafe { libc::kill(-self.0, libc::SIGKILL) } == 0);
        self.0 = 0;
    }
}
async fn kill_guard(pids: &[i32]) -> SurvivingGroup {
    // The live fixture is in the guard-led group. Kill only that guard, leaving
    // both the program and its child in the original group (neither calls setsid).
    let group = unsafe { libc::getpgid(pids[0]) };
    assert!(group > 1 && group != pids[0]);
    let cleanup = SurvivingGroup(group);
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
    cleanup
}

#[tokio::test]
async fn killed_guard_cannot_confirm_stop_or_admit_a_duplicate_after_restart() {
    let root = temporary_state();
    let fixture = Fixture::new(INSTANCE);
    let provider = Provider::start(&root).await;
    outcome(
        &invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await,
        "succeeded",
    );
    let pids = ready(root.path(), ID).await;
    let mut cleanup = kill_guard(&pids).await;
    let group = cleanup.0;
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
    let provider = Provider::start(&root).await;
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
    assert_eq!(
        fs::read_to_string(state(root.path(), ID).join("starts"))
            .unwrap()
            .lines()
            .count(),
        1,
        "run.v1 was answered unknown, so the fixture must not have started again"
    );
    assert!(pids.iter().all(|pid| alive(*pid)));

    // An external test owner terminates its known group. Only established absence
    // permits confirmation, identity cleanup, and another run of this instance.
    cleanup.end();
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
    assert_eq!(
        starts(root.path(), ID, 2).await,
        2,
        "the second run.v1 succeeded, so the fixture must have started twice"
    );
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "succeeded",
    );
    provider.close().await;
}

#[tokio::test]
async fn owed_stop_outlives_the_state_directory() {
    let root = temporary_state();
    let fixture = Fixture::new(INSTANCE);
    let provider = Provider::start(&root).await;
    outcome(
        &invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await,
        "succeeded",
    );
    let pids = ready(root.path(), ID).await;
    let mut survivors = kill_guard(&pids).await;
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "unknown",
    );
    provider.close().await;
    assert!(pids.iter().all(|pid| alive(*pid)));
    fs::remove_dir_all(root.path()).unwrap();

    let durable = root.ledger_dir();
    assert!(
        FileStore::new(&durable).load().await.is_ok_and(|records| {
            records
                .iter()
                .any(|record| record.instance == ID && record.owed_stop)
        }),
        "owed stop did not outlive CONDUIT_STATE_DIR"
    );
    assert_eq!(
        fs::read(durable.join("state-dir")).unwrap(),
        root.path().as_os_str().as_encoded_bytes()
    );
    assert_eq!(
        fs::metadata(&durable).unwrap().permissions().mode() & 0o7777,
        0o700
    );
    assert!(durable.join("markers").join(instance_key(ID)).is_file());
    let before = fs::read(durable.join("ledger.json")).unwrap();

    let replacement_state = root.path().with_file_name("replacement");
    provider_local_process::private_directory(&replacement_state).unwrap();
    let replacement = Provider::start_in(&replacement_state, root.durable()).await;
    assert_ne!(replacement.ledger_dir, durable);
    assert!(
        FileStore::new(&replacement.ledger_dir)
            .load()
            .await
            .is_ok_and(|records| records.is_empty())
    );
    replacement.close().await;
    assert!(before == fs::read(durable.join("ledger.json")).unwrap());
    survivors.end();
    gone(&pids).await;
}

async fn startup_output(state: &Path, durable: Option<&Path>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_provider-local-process"));
    command
        .env_clear()
        .env(names::STATE_DIR_VARIABLE, state)
        .kill_on_drop(true);
    if let Some(durable) = durable {
        command.env(DURABLE_VARIABLE, durable);
    }
    timeout(WAIT, command.output()).await.unwrap().unwrap()
}

async fn refuses_start(state: &Path, durable: Option<&Path>, expected: &[&str], case: &str) {
    let output = startup_output(state, durable).await;
    assert_startup_refusal(&output, expected, case);
}

fn assert_startup_refusal(output: &std::process::Output, expected: &[&str], case: &str) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success()
            && stderr.lines().count() == 1
            && expected.iter().all(|needle| stderr.contains(needle)),
        "startup did not give the required refusal for {case}"
    );
}

async fn save_stop_intent(store: &FileStore) {
    store
        .save(&RunRecord {
            instance: ID.into(),
            context: context(),
            state: RunState::Unknown,
            owed_stop: true,
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn startup_recovers_a_stop_intent_for_an_instance_that_never_launched() {
    let root = temporary_state();
    Provider::start(&root).await.close().await;
    let store = FileStore::new(&root.ledger_dir());
    // Simulate a crash after stop.v1 saves its intent for an untouched instance.
    save_stop_intent(&store).await;
    let provider = Provider::start(&root).await;
    assert!(store.load().await.unwrap().iter().any(|record| {
        record.instance == ID && !record.owed_stop && record.state == RunState::Stopped
    }));
    provider.close().await;
}

async fn refuses_recovery_without_owed_stop_marker(remove_directory: bool) {
    let root = temporary_state();
    let fixture = Fixture::new(INSTANCE);
    let provider = Provider::start(&root).await;
    outcome(
        &invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await,
        "succeeded",
    );
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "succeeded",
    );
    provider.close().await;
    let durable = root.ledger_dir();
    save_stop_intent(&FileStore::new(&durable)).await;
    let ledger = durable.join("ledger.json");
    let before = fs::read(&ledger).unwrap();
    let markers = durable.join("markers");
    let marker_path = format!("markers/{}", instance_key(ID));
    let marker = durable.join(&marker_path);
    assert!(!root.path().join("runs").join(instance_key(ID)).exists());
    assert!(state(root.path(), ID).is_dir());
    let contents = fs::read(&marker).unwrap();
    if !remove_directory {
        fs::set_permissions(&markers, fs::Permissions::from_mode(0o755)).unwrap();
        refuses_start(
            root.path(),
            Some(root.durable()),
            &[DURABLE_VARIABLE, "markers/ has mode 0755 instead of 0700"],
            "wrong markers directory mode",
        )
        .await;
        fs::set_permissions(&markers, fs::Permissions::from_mode(0o700)).unwrap();
        let mut other: Value = serde_json::from_slice(&contents).unwrap();
        other["instance"] = json!(SECOND);
        fs::write(&marker, serde_json::to_vec(&other).unwrap()).unwrap();
        refuses_start(
            root.path(),
            Some(root.durable()),
            &[DURABLE_VARIABLE, ID, &marker_path, "mismatched"],
            "mismatched marker",
        )
        .await;
    }
    fs::remove_file(&marker).unwrap();
    if remove_directory {
        fs::remove_dir(&markers).unwrap();
    }

    let output = startup_output(root.path(), Some(root.durable())).await;
    assert!(
        fs::read(&ledger).unwrap() == before,
        "startup rewrote the owed-stop ledger without a marker"
    );
    assert_startup_refusal(
        &output,
        &[
            "PRV-021",
            DURABLE_VARIABLE,
            "owed stop",
            ID,
            &marker_path,
            "missing",
        ],
        "owed stop without a matching marker",
    );
    assert_eq!(markers.is_dir(), !remove_directory);
}

#[tokio::test]
async fn startup_refuses_an_owed_stop_without_the_markers_directory() {
    refuses_recovery_without_owed_stop_marker(true).await;
}

#[tokio::test]
async fn startup_refuses_an_owed_stop_without_its_marker_entry() {
    refuses_recovery_without_owed_stop_marker(false).await;
}

#[tokio::test]
async fn startup_preserves_an_owed_stop_migrated_from_the_old_layout() {
    let root = temporary_state();
    let fixture = Fixture::new(INSTANCE);
    let provider = Provider::start(&root).await;
    outcome(
        &invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await,
        "succeeded",
    );
    let pids = ready(root.path(), ID).await;
    let mut survivors = kill_guard(&pids).await;
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "unknown",
    );
    provider.close().await;

    // Recreate the old layout from the interrupted run, then migrate as documented.
    let durable = root.ledger_dir();
    for name in ["ledger.json", "markers"] {
        fs::rename(durable.join(name), root.path().join(name)).unwrap();
    }
    fs::remove_dir_all(&durable).unwrap();
    provider_local_process::private_directory(&durable).unwrap();
    for name in ["ledger.json", "markers"] {
        fs::rename(root.path().join(name), durable.join(name)).unwrap();
    }
    let run = root.path().join("runs").join(instance_key(ID));
    assert!(run.is_dir());
    assert!(!durable.join("state-dir").exists());

    let provider = Provider::start(&root).await;
    assert!(FileStore::new(&durable).load().await.is_ok_and(|records| {
        records
            .iter()
            .any(|record| record.instance == ID && record.owed_stop)
    }));
    assert!(run.is_dir());
    assert!(durable.join("markers").join(instance_key(ID)).is_file());
    assert_eq!(
        fs::read(durable.join("state-dir")).unwrap(),
        root.path().as_os_str().as_encoded_bytes()
    );
    assert!(pids.iter().all(|pid| alive(*pid)));
    provider.close().await;
    survivors.end();
    gone(&pids).await;
}

#[tokio::test]
async fn startup_refuses_a_first_binding_with_existing_runs() {
    let root = temporary_state();
    Provider::start(&root).await.close().await;
    save_stop_intent(&FileStore::new(&root.ledger_dir())).await;
    let run = root.path().join("runs").join(instance_key(ID));
    provider_local_process::private_directory(&run).unwrap();
    let old_ledger = root.ledger_dir().join("ledger.json");
    let before = fs::read(&old_ledger).unwrap();
    let replacement = root.durable().with_file_name("replacement");
    provider_local_process::private_directory(&replacement).unwrap();

    refuses_start(
        root.path(),
        Some(&replacement),
        &[
            DURABLE_VARIABLE,
            "CONDUIT_STATE_DIR/runs/ is non-empty",
            "durableDir may have changed",
            "owed stops may be in the old directory",
        ],
        "new durable directory with existing runs",
    )
    .await;
    let unbound = replacement.join(root.ledger_dir().file_name().unwrap());
    assert!(!unbound.join("state-dir").exists());
    assert!(!unbound.join("ledger.json").exists());
    assert!(run.is_dir());
    assert!(fs::read(&old_ledger).unwrap() == before);
}

#[tokio::test]
async fn startup_refuses_a_misplaced_durable_directory() {
    let root = temporary_state();
    let parent = root.path().parent().unwrap();
    let missing = root.durable().with_file_name("missing");
    let file = root.durable().with_file_name("file");
    fs::write(&file, b"fixture").unwrap();
    let open = root.durable().with_file_name("nonprivate");
    fs::create_dir(&open).unwrap();
    fs::set_permissions(&open, fs::Permissions::from_mode(0o755)).unwrap();
    let inside = root.path().join("inside");
    let sibling = parent.join("sibling");
    for path in [&inside, &sibling] {
        provider_local_process::private_directory(path).unwrap();
    }
    fs::set_permissions(parent, fs::Permissions::from_mode(0o700)).unwrap();
    let alias = root.durable().with_file_name("alias");
    std::os::unix::fs::symlink(parent, &alias).unwrap();
    let through_alias = alias.join("sibling");
    let trailing_slash = alias.join("");
    for (case, path, reason) in [
        ("missing value", None, "needs durableDir"),
        ("empty value", Some(Path::new("")), "needs durableDir"),
        (
            "relative directory",
            Some(Path::new("relative")),
            "must be an absolute directory",
        ),
        (
            "missing directory",
            Some(missing.as_path()),
            "does not exist",
        ),
        ("regular file", Some(file.as_path()), "is not a directory"),
        ("final symlink", Some(alias.as_path()), "is not a directory"),
        (
            "symlink with trailing slash",
            Some(trailing_slash.as_path()),
            "is not a directory",
        ),
        (
            "wrong mode",
            Some(open.as_path()),
            "has mode 0755 instead of 0700",
        ),
        ("inside state", Some(inside.as_path()), "must be outside"),
        (
            "inside state parent",
            Some(sibling.as_path()),
            "must be outside",
        ),
        (
            "inside state parent through an alias",
            Some(through_alias.as_path()),
            "must be outside",
        ),
        ("state parent itself", Some(parent), "must be outside"),
    ] {
        let count = path
            .and_then(|dir| fs::read_dir(dir).ok())
            .map(Iterator::count);
        refuses_start(
            root.path(),
            path,
            &[DURABLE_VARIABLE, "durableDir", reason],
            case,
        )
        .await;
        assert_eq!(
            path.and_then(|dir| fs::read_dir(dir).ok())
                .map(Iterator::count),
            count,
            "refusal created durable entries for {case}"
        );
    }
    assert!(fs::read_dir(root.durable()).unwrap().next().is_none());

    let ledger = root.path().join("ledger.json");
    let markers = root.path().join("markers");
    fs::write(&ledger, b"[]").unwrap();
    refuses_start(
        root.path(),
        Some(root.durable()),
        &[DURABLE_VARIABLE, "ledger.json"],
        "old ledger",
    )
    .await;
    fs::remove_file(&ledger).unwrap();
    fs::create_dir(&markers).unwrap();
    refuses_start(
        root.path(),
        Some(root.durable()),
        &[DURABLE_VARIABLE, "markers/"],
        "old markers",
    )
    .await;
    fs::write(&ledger, b"[]").unwrap();
    refuses_start(
        root.path(),
        Some(root.durable()),
        &[DURABLE_VARIABLE, "ledger.json", "markers/"],
        "both old entries",
    )
    .await;
    assert!(fs::read_dir(root.durable()).unwrap().next().is_none());

    fs::remove_file(&ledger).unwrap();
    fs::remove_dir(&markers).unwrap();
    let instance = root.ledger_dir();
    std::os::unix::fs::symlink(root.durable(), &instance).unwrap();
    refuses_start(
        root.path(),
        Some(root.durable()),
        &[
            DURABLE_VARIABLE,
            "instance subdirectory",
            "is not a directory",
        ],
        "instance subdirectory symlink",
    )
    .await;
    fs::remove_file(&instance).unwrap();
    fs::create_dir(&instance).unwrap();
    fs::set_permissions(&instance, fs::Permissions::from_mode(0o755)).unwrap();
    refuses_start(
        root.path(),
        Some(root.durable()),
        &[
            DURABLE_VARIABLE,
            "instance subdirectory",
            "has mode 0755 instead of 0700",
        ],
        "instance subdirectory wrong mode",
    )
    .await;
    assert!(fs::read_dir(instance).unwrap().next().is_none());
}

#[tokio::test]
async fn startup_binds_an_existing_ledger_and_refuses_missing_or_wrong_state() {
    let root = temporary_state();
    let durable = root.ledger_dir();
    provider_local_process::private_directory(&durable).unwrap();
    let ledger = durable.join("ledger.json");
    let binding = durable.join("state-dir");
    fs::write(&ledger, b"[]\n").unwrap();

    // Resume after the initial ledger write but before the binding was committed.
    Provider::start(&root).await.close().await;
    assert!(
        binding.is_file(),
        "startup did not bind the durable ledger to the state directory"
    );
    assert_eq!(
        fs::read(&binding).unwrap(),
        root.path().as_os_str().as_encoded_bytes()
    );
    assert!(fs::read(&ledger).unwrap() == b"[]\n");

    fs::write(&binding, b"another-state-directory").unwrap();
    refuses_start(
        root.path(),
        Some(root.durable()),
        &[DURABLE_VARIABLE, "state-dir"],
        "different state binding",
    )
    .await;
    fs::write(&binding, root.path().as_os_str().as_encoded_bytes()).unwrap();

    fs::remove_file(&ledger).unwrap();
    assert!(FileStore::new(&durable).load().await.is_err());
    refuses_start(
        root.path(),
        Some(root.durable()),
        &[DURABLE_VARIABLE, "ledger.json"],
        "missing ledger",
    )
    .await;
    assert!(!ledger.exists());
}

#[tokio::test]
async fn declaration_refusals_and_path_safe_instance() {
    let root = temporary_state();
    let fixture = Fixture::new(INSTANCE);
    let provider = Provider::start(&root).await;
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
            root.durable(),
            Path::new(env!("CARGO_BIN_EXE_provider-local-process")),
            signaller.clone(),
        )
        .unwrap(),
    );
    let store = Arc::new(FileStore::new(&root.ledger_dir()));
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
    let _backend = LocalBackend::new(
        root.path(),
        root.durable(),
        Path::new(env!("CARGO_BIN_EXE_provider-local-process")),
    )
    .unwrap();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let record = RunRecord {
            instance: ID.into(),
            context: context(),
            state: RunState::Unknown,
            owed_stop: true,
        };
        FileStore::new(&root.ledger_dir())
            .save(&record)
            .await
            .unwrap();
        let before = fs::read(root.ledger_dir().join("ledger.json")).unwrap();
        let records = FileStore::new(&root.ledger_dir()).load().await.unwrap();
        assert!(records.len() == 1 && records[0].instance == ID && records[0].owed_stop);
        assert!(before == fs::read(root.ledger_dir().join("ledger.json")).unwrap());
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
        root.durable(),
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
        root.durable(),
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

fn priority(pid: i32) -> i32 {
    // SAFETY: the PID is our own or a live process spawned by this fixture.
    unsafe { libc::getpriority(libc::PRIO_PROCESS, pid as _) }
}

async fn invoke_with_config(client: &Client, arguments: Value, config: Value) -> CallToolResult {
    let mut request = serde_json::to_value(call("run.v1", arguments)).unwrap();
    request["_meta"][names::key("config")] = config;
    timeout(
        WAIT,
        client.call_tool(serde_json::from_value(request).unwrap()),
    )
    .await
    .expect("provider call timed out")
    .expect("provider call failed")
}

#[tokio::test]
async fn nice_from_the_call_reaches_every_process_of_the_run() {
    let base = priority(std::process::id() as i32);
    assert!(base < 19, "this test needs its own nice below 19");
    let n = (base + 5).min(19);
    let root = temporary_state();
    let provider = Provider::start(&root).await;
    let fixture = Fixture::new(INSTANCE);
    // A JSON number with a zero fractional part is an integer in the Type schema.
    let run = invoke_with_config(
        &provider.client,
        inputs(ID, &fixture),
        json!({"nice": n as f64}),
    )
    .await;
    outcome(&run, "succeeded");
    let pids = ready(root.path(), ID).await;
    assert_eq!(pids.len(), 2);
    let guard = unsafe { libc::getpgid(pids[0]) };
    assert!(guard > 1);
    let priorities: Vec<_> = pids.iter().copied().chain([guard]).map(priority).collect();
    let server_priority = priority(provider.child.id().unwrap() as i32);
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "succeeded",
    );
    gone(&pids).await;
    fs::remove_file(state(root.path(), ID).join("pids.json")).unwrap();
    outcome(
        &invoke(&provider.client, "run.v1", inputs(ID, &fixture)).await,
        "succeeded",
    );
    let pids = ready(root.path(), ID).await;
    let inherited: Vec<_> = pids.iter().copied().map(priority).collect();
    outcome(
        &invoke(&provider.client, "stop.v1", args(ID)).await,
        "succeeded",
    );
    gone(&pids).await;
    provider.close().await;
    assert_eq!(
        priorities,
        vec![n; 3],
        "guard, program and child must take call nice"
    );
    assert_eq!(server_priority, base);
    assert_eq!(inherited, vec![base; 2]);
}

#[tokio::test]
async fn nice_that_cannot_be_given_runs_nothing_and_names_nice() {
    let base = priority(std::process::id() as i32);
    assert!(base <= 15, "this test needs its own nice at most 15");
    let root = temporary_state();
    let provider = Provider::start_with_nice(root.path(), root.durable(), Some(base + 2)).await;
    assert_eq!(priority(provider.child.id().unwrap() as i32), base + 2);
    let fixture = Fixture::new(INSTANCE);
    for (nice, reason) in [
        (
            json!(base + 1),
            "nice is below the provider's own nice value",
        ),
        (json!(20), "nice must be an integer from 0 to 19"),
        (json!("10"), "nice must be an integer from 0 to 19"),
    ] {
        let result =
            invoke_with_config(&provider.client, inputs(ID, &fixture), json!({"nice":nice})).await;
        // Clean up even on the old implementation, which incorrectly starts a program.
        if result
            .structured_content
            .as_ref()
            .is_some_and(|v| v.get("url").is_some())
        {
            outcome(
                &invoke(&provider.client, "stop.v1", args(ID)).await,
                "succeeded",
            );
        }
        outcome(&result, "failed");
        assert!(
            serde_json::to_value(&result).unwrap()["content"]
                .to_string()
                .contains(reason),
            "refusal must name the nice setting"
        );
        assert!(!state(root.path(), ID).join("starts").exists());
        let inspect = invoke(&provider.client, "inspect-run.v1", args(ID)).await;
        outcome(&inspect, "succeeded");
        assert_eq!(
            inspect.structured_content.as_ref().unwrap()["state"],
            "stopped"
        );
    }
    provider.close().await;
}
