//! The engine's own runner reads from the installed file store through a Pool entry.
use conduit_shell::{
    Config, Execution, Executor, MissingExecutor, RunIdentity, Shell, boundary,
    runner::Runner,
    server::{self, ServingExecutor},
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{BufRead, Write},
    net::{TcpListener, TcpStream},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

const PACKAGE: &str = "example.org/conduit/provider-file";
const META: &str = "io.github.shrav-k.conduit/";

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn pack(repository: &Path) -> (String, String) {
    let package = repository.join("packages/provider-file");
    let output = Command::new("cargo")
        .current_dir(&package)
        .env("CARGO_NET_OFFLINE", "true")
        .args([
            "run",
            "--offline",
            "--locked",
            "--release",
            "--bin",
            "package",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = String::from_utf8(output.stdout).unwrap();
    let digest = output.trim().rsplit_once("sha256:").unwrap().1.to_owned();
    (
        format!(
            "{}#{digest}",
            package.join("dist/provider-file.tar.gz").display()
        ),
        digest,
    )
}

fn no_marker(bytes: &[u8], markers: &[&str]) {
    assert!(
        markers
            .iter()
            .all(|marker| !bytes.windows(marker.len()).any(|b| b == marker.as_bytes())),
        "a file, command, or record contained a synthetic marker"
    );
}

fn cli(store: &Path, markers: &[&str], args: &[&str], succeeded: bool) -> Output {
    let output = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .arg(args[0])
        .arg("--store")
        .arg(store)
        .args(&args[1..])
        .output()
        .unwrap();
    no_marker(&output.stdout, markers);
    no_marker(&output.stderr, markers);
    assert_eq!(
        output.status.success(),
        succeeded,
        "{args:?}: {} {}",
        String::from_utf8_lossy(&output.stderr),
        if args[0] == "call" {
            String::from_utf8_lossy(&output.stdout)
        } else {
            "".into()
        }
    );
    output
}

fn scan(dir: &Path, markers: &[&str]) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let ty = entry.file_type().unwrap();
        if ty.is_dir() {
            scan(&entry.path(), markers);
        } else if ty.is_file() {
            no_marker(&fs::read(entry.path()).unwrap(), markers);
        } else {
            panic!("unexpected store entry type");
        }
    }
}

fn until(mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(15);
    while !ready() {
        assert!(Instant::now() < deadline, "test condition did not occur");
        thread::sleep(Duration::from_millis(20));
    }
}

fn locate(shell: &Shell, kind: &str, entry: &str) -> Value {
    shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| {
            o["kind"] == kind
                && o.get("spec").is_none()
                && (o["name"] == entry || o["nested"]["name"] == entry)
        })
        .unwrap()
        .clone()
}

struct Engine {
    shell: Arc<Shell>,
    runner: Runner,
    stop: CancellationToken,
    server: Option<thread::JoinHandle<conduit_shell::Result<()>>>,
    executor: Arc<CountingExecutor>,
}

struct CountingExecutor {
    inner: Arc<dyn Executor>,
    runs: Mutex<BTreeMap<String, usize>>,
}

impl Executor for CountingExecutor {
    fn builtin_runner(
        &self,
        intent: &Value,
        identity: Option<&RunIdentity>,
    ) -> conduit_shell::Result<Execution> {
        if intent["operation"]
            .as_str()
            .is_some_and(|op| op.ends_with("#run.v1"))
        {
            let subject = intent["subject"].as_str().unwrap().to_owned();
            *self.runs.lock().unwrap().entry(subject).or_default() += 1;
        }
        self.inner.builtin_runner(intent, identity)
    }

    fn dispatch(&self, intent: &Value) -> conduit_shell::Result<Execution> {
        self.inner.dispatch(intent)
    }

    fn refresh_tools_and_call(&self, intent: &Value) -> conduit_shell::Result<Execution> {
        self.inner.refresh_tools_and_call(intent)
    }

    fn cancel(&self, intent: &Value) -> conduit_shell::Result<Execution> {
        self.inner.cancel(intent)
    }

    fn cancel_setup_parts(&self, intent: &Value) -> conduit_shell::Result<Execution> {
        self.inner.cancel_setup_parts(intent)
    }
}

impl Engine {
    fn new(store: &Path, repository: &Path, port: u16) -> Self {
        let shell = Arc::new(
            Shell::open(
                store,
                boundary::context(repository).unwrap(),
                Config {
                    engine_url: format!("http://127.0.0.1:{port}/mcp"),
                    stop_grace: Duration::from_millis(100),
                    ..Config::default()
                },
            )
            .unwrap(),
        );
        let runner = Runner::new(&shell, PathBuf::from(env!("CARGO_BIN_EXE_conduit"))).unwrap();
        let executor = Arc::new(CountingExecutor {
            inner: runner.executor(),
            runs: Mutex::new(BTreeMap::new()),
        });
        Self {
            shell,
            runner,
            stop: CancellationToken::new(),
            server: None,
            executor,
        }
    }

    fn serve(&mut self, port: u16) {
        let root = locate(&self.shell, "Interface", "root");
        self.shell
            .issue_client_token(root["id"].as_str().unwrap())
            .unwrap();
        let shell = self.shell.clone();
        let executor = Arc::new(ServingExecutor::new(self.executor.clone()));
        let completion: Arc<dyn Executor> = executor.clone();
        self.runner
            .set_completion_executor(Arc::downgrade(&completion));
        let stop = self.stop.clone();
        self.server = Some(thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(server::serve(shell, executor, stop))
        }));
        until(|| TcpStream::connect(("127.0.0.1", port)).is_ok());
    }

    fn run_intents(&self, subject: &Value) -> usize {
        self.executor
            .runs
            .lock()
            .unwrap()
            .get(subject.as_str().unwrap())
            .copied()
            .unwrap_or(0)
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.stop.cancel();
        if let Some(server) = self.server.take() {
            let result = server.join().unwrap();
            assert!(result.is_ok(), "test server failed");
        }
        self.runner.shutdown();
    }
}

fn setup(store: &Path, markers: &[&str], kind: &str, name: &str, succeeds: bool) -> Value {
    let tool = if kind == "Pool" {
        "management.set-up.v1"
    } else {
        "demo.set-up.v1"
    };
    let arguments = json!({"instance":{"kind":kind,"name":name}}).to_string();
    serde_json::from_slice(&cli(store, markers, &["call", tool, &arguments], succeeds).stdout)
        .unwrap()
}

fn ended_read(shell: &Shell, status: &str, value: &str) -> Value {
    let record = shell
        .state()
        .unwrap()
        .records
        .iter()
        .rev()
        .find(|r| is_read_ending(r))
        .unwrap()
        .clone();
    assert_eq!(record["data"][status], value);
    record
}

fn is_read_ending(record: &Value) -> bool {
    record["type"] == "request-ended"
        && record["data"]["operation"]
            .as_str()
            .is_some_and(|op| op.ends_with("#get-secret.v1"))
}

fn one_read_for_setup(shell: &Shell, since: usize, setup: &Value) -> Value {
    let id = &setup["_meta"][format!("{META}invocation")];
    assert!(id.is_string());
    let state = shell.state().unwrap();
    let mut reads = state.records[since..]
        .iter()
        .filter(|record| is_read_ending(record));
    let record = reads.next().expect("setup has no new read ending");
    assert!(
        reads.next().is_none(),
        "setup has multiple new read endings"
    );
    assert_eq!(record["data"]["serving"], *id);
    record.clone()
}

fn test_endpoint(store: &Path, markers: &[&str]) -> u16 {
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let url = format!("http://127.0.0.1:{port}/mcp");
    cli(store, markers, &["init", "--url", &url], true);
    port
}

fn project(repository: &Path) -> conduit_core::packages::Project {
    let spec = conduit_loader::Specification::read(&repository.join("spec")).unwrap();
    boundary::project(
        &repository.join("conformance/conventions/secret-reads/project"),
        &spec,
    )
    .unwrap()
}

fn script_command(name: &str) -> Value {
    json!([
        std::env::current_exe().unwrap(),
        "--exact",
        name,
        "--ignored",
        "--nocapture"
    ])
}

fn register(shell: &Shell, project: conduit_core::packages::Project, markers: &[&str]) -> Value {
    let root = locate(shell, "Interface", "root");
    let management = shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| {
            o["kind"] == "Conduit" && o["consumer"] == root["id"] && o["local_name"] == "management"
        })
        .unwrap()
        .clone();
    let registration = shell.submit(json!({
        "kind":"register-project","snapshots":[],"builtin":shell.context.builtin,"project":project,
        "request":{"sender":root["id"],"caller":root["id"],"conduit":management["id"],
          "target":management["target"],"operation":format!("{}#register.v1",shell.context.core_package)}
    }), &MissingExecutor).unwrap();
    no_marker(registration.to_string().as_bytes(), markers);
    assert_eq!(registration["accepted"], true, "{registration}");
    root
}

fn stopped(engine: &Engine, consumer: &Value, dir: &Path) {
    fs::write(dir.join("end-run"), []).unwrap();
    let inspect = json!({"subject":consumer["id"], "operation":format!("{}#inspect-run.v1", engine.shell.context.core_package)});
    until(|| {
        matches!(engine.runner.builtin_runner(&inspect, None).unwrap(),
        Execution::SetupReport(report) if report["state"] == "stopped")
    });
    fs::remove_file(dir.join("digest")).unwrap();
}

fn digest(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

#[test]
#[ignore = "engine-owned stdio server fixture"]
fn scripted_server() {
    let value = std::env::var("TEST_VALUE").expect("run variable missing");
    assert!(std::env::var_os("TEST_UNUSED").is_none());
    let dir = PathBuf::from(std::env::var_os("TEST_DIR").unwrap());
    fs::write(dir.join("digest"), digest(&value)).unwrap();
    drop(value);
    thread::spawn(move || {
        loop {
            if fs::remove_file(dir.join("end-run")).is_ok() {
                std::process::exit(0);
            }
            if fs::remove_file(dir.join("verify")).is_ok() {
                let value = std::env::var("TEST_VALUE").expect("run variable missing");
                fs::write(dir.join("confirmed"), digest(&value)).unwrap();
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
    for line in std::io::stdin().lock().lines() {
        let message: Value = serde_json::from_str(&line.unwrap()).unwrap();
        if message["method"] == "initialize" {
            let reply = json!({"jsonrpc":"2.0","id":message["id"],"result":{
                "protocolVersion":message["params"]["protocolVersion"],
                "serverInfo":{"name":"secret-consumer","version":"1"},"capabilities":{"tools":{}}
            }});
            println!("{reply}");
            std::io::stdout().flush().unwrap();
        }
    }
}

#[test]
fn file_store_read_is_run_local_and_decided_again_after_restart() {
    let repository = repository();
    let (archive, package_digest) = pack(&repository);
    let parent = tempfile::tempdir().unwrap();
    let store = parent.path().join("store");
    let marker_file = parent.path().join("marker");
    let first = format!("synthetic-{}", Uuid::new_v4());
    let second = format!("synthetic-{}", Uuid::new_v4());
    let markers = [first.as_str(), second.as_str()];
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&marker_file)
        .unwrap()
        .write_all(first.as_bytes())
        .unwrap();
    let port = test_endpoint(&store, &markers);

    let mut engine = Engine::new(&store, &repository, port);
    let shell = &engine.shell;
    boundary::install_package(shell, &repository, &archive).unwrap();
    let mut project = project(&repository);
    let manifest = project.manifest.as_mut().unwrap();
    manifest["packages"][PACKAGE] =
        json!({"version":"0.1.0","digest":format!("sha256:{package_digest}")});
    manifest["implementations"]
        .as_object_mut()
        .unwrap()
        .remove("store-exec");
    manifest["implementations"]["consumer-exec"]["command"] = script_command("scripted_server");
    project.documents.retain(|doc| {
        !["scripted-secret.v1", "read-filter.v1"]
            .contains(&doc["metadata"]["name"].as_str().unwrap())
    });
    for doc in &mut project.documents {
        if doc["kind"] == "Resource" && doc["metadata"]["name"] == "store" {
            doc["spec"] =
                json!({"type":format!("{PACKAGE}#file-secret.v1"),"config":{"path":marker_file}});
        } else if doc["kind"] == "Resource" && doc["metadata"]["name"] == "consumer" {
            doc["spec"]["config"]["directory"] = json!(parent.path());
        } else if doc["kind"] == "Conduit" {
            doc["spec"].as_object_mut().unwrap().remove("requirements");
        }
    }
    let root = register(shell, project, &markers);
    engine.serve(port);
    let pool_setup = setup(&store, &markers, "Pool", "demo", true);
    assert_eq!(pool_setup["_meta"][format!("{META}outcome")], "succeeded");
    let consumer = locate(&engine.shell, "Resource", "consumer");
    let provider = locate(&engine.shell, "Resource", "store");
    let conduit = locate(&engine.shell, "Conduit", "source");
    let read1 = ended_read(&engine.shell, "outcome", "succeeded");
    assert_eq!(read1["data"]["caller"], root["id"]);
    assert_eq!(read1["data"]["consumer"], consumer["id"]);
    assert_eq!(read1["data"]["target"], provider["id"]);
    assert_eq!(read1["data"]["conduit"], conduit["id"]);
    let path = parent.path().join("digest");
    assert_eq!(fs::read_to_string(&path).unwrap(), digest(&first));
    cli(&store, &markers, &["inspect"], true);
    scan(&store, &markers);

    drop(engine);
    fs::remove_file(&path).unwrap();
    fs::write(&marker_file, &second).unwrap();
    let mut engine = Engine::new(&store, &repository, port);
    engine.serve(port);
    let name = consumer["name"].as_str().unwrap();
    setup(&store, &markers, "Resource", name, true);
    let read2 = ended_read(&engine.shell, "outcome", "succeeded");
    assert_eq!(engine.run_intents(&provider["id"]), 1);
    assert_ne!(read1["data"]["invocation"], read2["data"]["invocation"]);
    assert_ne!(read1["data"]["serving"], read2["data"]["serving"]);
    assert_eq!(
        locate(&engine.shell, "Resource", "store")["revision"],
        provider["revision"]
    );
    let rotated = fs::read_to_string(&path).unwrap();
    assert_eq!(rotated, digest(&second));
    assert_ne!(rotated, digest(&first));

    for mode in [Some(0o644), None] {
        stopped(&engine, &consumer, parent.path());
        if let Some(mode) = mode {
            fs::set_permissions(&marker_file, fs::Permissions::from_mode(mode)).unwrap();
        } else {
            fs::remove_file(&marker_file).unwrap();
        }
        let since = engine.shell.state().unwrap().records.len();
        let starts = engine.run_intents(&consumer["id"]);
        let failed = setup(&store, &markers, "Resource", name, false);
        let record = one_read_for_setup(&engine.shell, since, &failed);
        assert_eq!(record["data"]["outcome"], "failed");
        assert_eq!(engine.run_intents(&consumer["id"]), starts);
        no_marker(record.to_string().as_bytes(), &markers);
        assert!(!path.exists());
        if mode.is_some() {
            fs::set_permissions(&marker_file, fs::Permissions::from_mode(0o600)).unwrap();
            setup(&store, &markers, "Resource", name, true);
        }
    }

    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&marker_file)
        .unwrap()
        .write_all(second.as_bytes())
        .unwrap();
    setup(&store, &markers, "Resource", name, true);
    let before_close = ended_read(&engine.shell, "outcome", "succeeded");
    let close = json!({"conduit":conduit["name"]}).to_string();
    cli(&store, &markers, &["call", "access.close.v1", &close], true);
    assert_eq!(
        locate(&engine.shell, "Resource", "consumer")["state"],
        "ready"
    );
    assert_eq!(
        ended_read(&engine.shell, "outcome", "succeeded"),
        before_close
    );
    fs::write(parent.path().join("verify"), []).unwrap();
    let confirmed = parent.path().join("confirmed");
    until(|| fs::read_to_string(&confirmed).is_ok_and(|file| file == digest(&second)));
    stopped(&engine, &consumer, parent.path());
    let refused = setup(&store, &markers, "Resource", name, false);
    let record = ended_read(&engine.shell, "reason", "refused");
    assert_eq!(record["data"]["ruleId"], "ACC-001");
    assert!(!path.exists());
    let arguments = json!({"object":{"kind":"Resource","name":name}}).to_string();
    let inspection = cli(
        &store,
        &markers,
        &["call", "demo.inspect.v1", &arguments],
        true,
    );
    let inspected: Value = serde_json::from_slice(&inspection.stdout).unwrap();
    let status = &inspected["structuredContent"]["status"]["setup"];
    assert_eq!(
        status["invocation"],
        refused["_meta"][format!("{META}invocation")]
    );
    let run = status["parts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|part| part["name"] == "run")
        .expect("consumer has a run part");
    assert_eq!(run["outcome"], "unavailable");
    scan(&store, &markers);
    drop(engine);
    cli(&store, &markers, &["records"], true);
    scan(&store, &markers);
}

#[test]
#[ignore = "stdio fixture that loses its transport while serving a secret read"]
fn provider_closes_transport_on_read() {
    for line in std::io::stdin().lock().lines() {
        let request: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let capabilities = json!({"tools":{}, "extensions":{
            format!("{META}implementation-interface"):{"revision":"v0"}
        }});
        let result = match request["method"].as_str() {
            Some("initialize") => json!({"protocolVersion":request["params"]["protocolVersion"],
                "serverInfo":{"name":"exiting-secret-provider","version":"1"},"capabilities":capabilities}),
            Some("server/discover") => {
                json!({"resultType":"complete","supportedVersions":["2026-07-28"],
                "capabilities":capabilities,"ttlMs":0,"cacheScope":"private"})
            }
            Some("tools/call") => {
                assert_eq!(request["params"]["name"], "get-secret.v1");
                // Close transport before exiting so the runner's cleanup can win
                // the race with the guard's exit observation.
                unsafe { libc::close(libc::STDOUT_FILENO) };
                loop {
                    thread::park();
                }
            }
            _ => continue,
        };
        println!(
            "{}",
            json!({"jsonrpc":"2.0","id":request["id"],"result":result})
        );
        std::io::stdout().flush().unwrap();
    }
}

#[test]
fn a_provider_that_ends_during_a_read_is_not_relaunched_after_shutdown() {
    let repository = repository();
    let parent = tempfile::tempdir().unwrap();
    let store = parent.path().join("store");
    let port = test_endpoint(&store, &[]);
    let mut engine = Engine::new(&store, &repository, port);
    let mut project = project(&repository);
    project.manifest.as_mut().unwrap()["implementations"]["store-exec"]["command"] =
        script_command("provider_closes_transport_on_read");
    register(&engine.shell, project, &[]);
    engine.serve(port);
    let provider = locate(&engine.shell, "Resource", "store");
    let consumer = locate(&engine.shell, "Resource", "consumer");
    setup(&store, &[], "Pool", "demo", false);
    assert_eq!(engine.run_intents(&provider["id"]), 1);
    assert_eq!(engine.run_intents(&consumer["id"]), 0);
    ended_read(&engine.shell, "outcome", "unknown");
    assert!(
        engine
            .shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .any(|invocation| {
                invocation["object"] == provider["id"]
                    && invocation["secret_provider_end"]["launch"] == invocation["id"]
                    && invocation["secret_provider_end"]["cause"] == "transport-failure"
            })
    );
    drop(engine);

    let mut engine = Engine::new(&store, &repository, port);
    engine.serve(port);
    let since = engine.shell.state().unwrap().records.len();
    let failed = setup(
        &store,
        &[],
        "Resource",
        consumer["name"].as_str().unwrap(),
        false,
    );
    let read = one_read_for_setup(&engine.shell, since, &failed);
    assert_eq!(read["data"]["reason"], "unavailable");
    assert_eq!(engine.run_intents(&provider["id"]), 0);
    assert_eq!(engine.run_intents(&consumer["id"]), 0);
}
