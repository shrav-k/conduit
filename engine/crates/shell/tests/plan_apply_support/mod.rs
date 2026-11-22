#[allow(dead_code)]
pub mod cli;

use crate::package_target::package_target_dir;
use conduit_shell::{
    Config, Execution, Executor, MissingExecutor, RunIdentity, Shell, boundary, runner::Runner,
    server,
};
use rmcp::{
    RoleClient, ServiceExt,
    model::CallToolRequestParams,
    service::{RunningService, ServiceError},
    transport::{
        StreamableHttpClientTransport, streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    net::{Ipv4Addr, TcpListener},
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{sync::oneshot, task::JoinHandle, time::timeout};
use tokio_util::sync::CancellationToken;

const META: &str = "io.github.shrav-k.conduit/";
const WAIT: Duration = Duration::from_secs(30);
const FILESYSTEM: &str = "@modelcontextprotocol/server-filesystem";

pub fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

pub fn reference() -> PathBuf {
    repository().join("docs/implementation/plan-and-apply")
}

pub fn cases() -> Vec<Value> {
    let mut paths: Vec<_> = fs::read_dir(reference().join("cases"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    paths.sort_by_key(|path| {
        let prefix = path.file_name().unwrap().to_str().unwrap()[..2]
            .parse::<u8>()
            .unwrap();
        match prefix {
            8 => 9,
            9 => 8,
            n => n,
        }
    });
    assert_eq!(paths.len(), 9);
    paths
        .iter()
        .map(|path| {
            boundary::value(path).unwrap_or_else(|error| {
                panic!(
                    "case {}: {error}",
                    path.file_name().unwrap().to_string_lossy()
                )
            })
        })
        .collect()
}

pub fn sha256(data: &str) -> String {
    format!("{:x}", Sha256::digest(data.as_bytes()))
}

pub fn safe_projection(value: &Value) -> Value {
    match value {
        Value::Object(fields) => Value::Object(
            fields
                .iter()
                .map(|(key, value)| {
                    let projected = if key == "root" {
                        value
                            .as_str()
                            .filter(|path| path.starts_with('/'))
                            .map(|path| json!({"pathSha256":sha256(path)}))
                            .unwrap_or_else(|| safe_projection(value))
                    } else {
                        safe_projection(value)
                    };
                    (key.clone(), projected)
                })
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.iter().map(safe_projection).collect()),
        _ => value.clone(),
    }
}

pub struct Install {
    pub stage: u8,
    pub args: Value,
    pub pin: Value,
}

pub fn installs() -> Vec<Install> {
    let fixtures = repository().join("conformance/fixtures");
    let digests = boundary::value(&fixtures.join("install-archives/digests.json")).unwrap();
    let s1 = fs::read_to_string(fixtures.join("s1/README.md")).unwrap();
    [
        (1, "install-archives", "filesystem-0.1.1.tar.gz"),
        (1, "s1/archives", "agents-0.2.1.tar.gz"),
        (2, "s1/archives", "filesystem.tar.gz"),
        (2, "s1/archives", "history.tar.gz"),
        (5, "install-archives", "filesystem-0.2.1.tar.gz"),
    ]
    .iter()
    .map(|&(stage, directory, name)| {
        let digest = if directory == "install-archives" {
            digests[name]
                .as_str()
                .unwrap()
                .strip_prefix("sha256:")
                .unwrap()
        } else {
            s1.lines()
                .find(|line| line.starts_with(&format!("| [{name}](archives/")))
                .and_then(|line| line.split('`').nth(1))
                .unwrap()
        };
        let path = fixtures.join(directory).join(name);
        let args = boundary::install_arguments(&format!("{}#{digest}", path.display()))
            .unwrap_or_else(|_| panic!("fixture archive {name} did not match its declared pin"));
        let pin =
            json!({"address":args["address"],"version":args["version"],"digest":args["digest"]});
        Install { stage, args, pin }
    })
    .collect()
}

#[derive(Default)]
struct Filesystem {
    runs: Mutex<BTreeSet<String>>,
    fail_setup: Arc<AtomicBool>,
}

struct AgentPrograms {
    wrapper: PathBuf,
    client: PathBuf,
}

fn scripted_agents() -> BTreeMap<&'static str, AgentPrograms> {
    ["claude-code", "codex"]
        .into_iter()
        .map(|kind| {
            let name = format!("agent-{kind}");
            let directory = repository().join("packages").join(&name);
            let output = Command::new("cargo")
                .current_dir(&directory)
                .envs(package_target_dir(&directory).map(|dir| ("CARGO_TARGET_DIR", dir)))
                .args([
                    "build",
                    "--offline",
                    "--locked",
                    "--release",
                    "--bin",
                    &name,
                    "--bin",
                    "scripted-client",
                    "--message-format=json",
                ])
                .output()
                .expect("could not build scripted Agent");
            assert!(
                output.status.success(),
                "offline scripted Agent build failed for {kind}"
            );
            let artifacts: BTreeMap<_, _> = output
                .stdout
                .split(|byte| *byte == b'\n')
                .filter_map(|line| serde_json::from_slice::<Value>(line).ok())
                .filter_map(|line| {
                    (line["reason"] == "compiler-artifact")
                        .then(|| {
                            Some((
                                line["target"]["name"].as_str()?.to_owned(),
                                PathBuf::from(line["executable"].as_str()?),
                            ))
                        })
                        .flatten()
                })
                .collect();
            (
                kind,
                AgentPrograms {
                    wrapper: artifacts[&name].clone(),
                    client: artifacts["scripted-client"].clone(),
                },
            )
        })
        .collect()
}

struct ReplayExecutor {
    filesystem: Filesystem,
    runner: Arc<dyn Executor>,
    programs: BTreeMap<&'static str, AgentPrograms>,
    agents: Mutex<BTreeSet<String>>,
    shell: Weak<Shell>,
}

impl ReplayExecutor {
    fn agent(&self, id: &Value) -> bool {
        self.agents
            .lock()
            .unwrap()
            .contains(id.as_str().unwrap_or(""))
    }

    fn selected(&self, subject: &Value) -> conduit_shell::Result<(bool, bool)> {
        let shell = self.shell.upgrade().ok_or("replay shell stopped")?;
        let state = shell.state()?;
        let items = state
            .objects
            .iter()
            .find(|object| object["id"] == *subject)
            .and_then(|object| object["resolved_implementations"].as_object());
        let Some(items) = items else {
            return Ok((false, false));
        };
        Ok((
            items
                .values()
                .any(|selection| selection["item"]["command"][0] == "s1-scripted-agent"),
            items
                .values()
                .any(|selection| selection["item"]["package"]["identifier"] == FILESYSTEM),
        ))
    }
}

impl Executor for ReplayExecutor {
    fn builtin_runner(
        &self,
        intent: &Value,
        identity: Option<&RunIdentity>,
    ) -> conduit_shell::Result<Execution> {
        let command = &intent["inputs"]["item"]["command"];
        if command[0] == "s1-scripted-agent" {
            let program = self
                .programs
                .get(command[1].as_str().unwrap_or(""))
                .ok_or("unknown scripted Agent fixture command")?;
            let mut intent = intent.clone();
            intent["inputs"]["item"]["command"] =
                json!([program.wrapper, "--client-program", program.client]);
            self.agents
                .lock()
                .unwrap()
                .insert(intent["subject"].as_str().unwrap().into());
            self.runner.builtin_runner(&intent, identity)
        } else if self.agent(&intent["subject"]) {
            self.runner.builtin_runner(intent, identity)
        } else {
            let (agent, filesystem) = self.selected(&intent["subject"])?;
            if agent {
                self.runner.builtin_runner(intent, identity)
            } else if filesystem {
                self.filesystem.builtin_runner(intent, identity)
            } else {
                Ok(Execution::Missing)
            }
        }
    }

    fn dispatch(&self, intent: &Value) -> conduit_shell::Result<Execution> {
        if self.agent(&intent["target"]) {
            self.runner.dispatch(intent)
        } else {
            self.filesystem.dispatch(intent)
        }
    }

    fn refresh_tools_and_call(&self, intent: &Value) -> conduit_shell::Result<Execution> {
        if self.agent(&intent["target"]) {
            self.runner.refresh_tools_and_call(intent)
        } else {
            self.filesystem.refresh_tools_and_call(intent)
        }
    }
}

struct OwnedRunner(Option<Runner>);

impl Drop for OwnedRunner {
    fn drop(&mut self) {
        let runner = self.0.take();
        std::thread::spawn(move || drop(runner)).join().unwrap();
    }
}

impl Executor for Filesystem {
    fn builtin_runner(
        &self,
        intent: &Value,
        _: Option<&RunIdentity>,
    ) -> conduit_shell::Result<Execution> {
        let subject = intent["subject"].as_str().unwrap_or("");
        let mut runs = self.runs.lock().unwrap();
        let report = match intent["operation"]
            .as_str()
            .and_then(|op| op.rsplit('#').next())
        {
            Some("run.v1") if intent["inputs"]["item"]["package"]["identifier"] == FILESYSTEM => {
                if self.fail_setup.swap(false, Ordering::SeqCst) {
                    return Ok(Execution::SetupReport(
                        json!({"kind":"run","outcome":"failed"}),
                    ));
                }
                runs.insert(subject.into());
                json!({"kind":"run","outcome":"succeeded"})
            }
            Some("inspect-run.v1") => {
                json!({"kind":"inspect-run","state":if runs.contains(subject) {"running"} else {"stopped"}})
            }
            Some("stop.v1") if runs.remove(subject) => json!({"kind":"stop","outcome":"succeeded"}),
            _ => return Ok(Execution::Missing),
        };
        Ok(Execution::SetupReport(report))
    }

    fn dispatch(&self, intent: &Value) -> conduit_shell::Result<Execution> {
        if !self
            .runs
            .lock()
            .unwrap()
            .contains(intent["target"].as_str().unwrap_or(""))
        {
            return Ok(Execution::Missing);
        }
        let result = match intent["call"]["method"].as_str() {
            Some("server/discover") => json!({"capabilities":{"extensions":
                intent["call"]["params"]["_meta"]["io.modelcontextprotocol/clientCapabilities"]["extensions"]}}),
            Some("tools/list") => {
                json!({"tools":(["read_file","write_file","list_directory"].map(|name| {
                    json!({"name":name,"inputSchema":{"type":"object"}})
                }))})
            }
            _ => return Ok(Execution::Missing),
        };
        Ok(Execution::Complete(
            json!({"kind":"result","result":result}),
        ))
    }

    fn refresh_tools_and_call(&self, intent: &Value) -> conduit_shell::Result<Execution> {
        self.dispatch(intent)
    }
}

pub struct Engine {
    root: RunningService<RoleClient, ()>,
    shutdown: CancellationToken,
    server: JoinHandle<conduit_shell::Result<()>>,
    _runner: OwnedRunner,
    _store: tempfile::TempDir,
    #[allow(dead_code)]
    fail_setup: Arc<AtomicBool>,
}

impl Engine {
    #[allow(dead_code)] // Used by the planner integration tests.
    pub fn directory(&self) -> PathBuf {
        self._store.path().join("store")
    }

    #[allow(dead_code)] // Used by the apply integration tests.
    pub fn fail_next_setup(&self) {
        self.fail_setup.store(true, Ordering::SeqCst);
    }

    pub async fn start() -> Self {
        Self::start_with(|_| {}).await
    }

    pub async fn start_with(prepare: impl FnOnce(&Shell)) -> Self {
        conduit_shell::install_crypto_provider();
        let store = tempfile::tempdir().unwrap();
        let directory = store.path().join("store");
        let reserved = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let endpoint = format!("http://{}/mcp", reserved.local_addr().unwrap());
        let shell = Arc::new(
            Shell::open(
                &directory,
                boundary::context(&repository()).unwrap(),
                Config {
                    engine_url: endpoint.clone(),
                    ..Config::default()
                },
            )
            .unwrap(),
        );
        let host = "plan-apply-fixture";
        let start = shell.submit(
            json!({"kind":"first-start","machine":{"hostname":host,"labels":{"kubernetes.io/hostname":host,"kubernetes.io/os":"linux","kubernetes.io/arch":"amd64"},"state":"reachable"}}),
            &MissingExecutor,
        ).unwrap();
        assert_eq!(start["started"], true, "first start refused");
        prepare(&shell);
        let id = shell
            .state()
            .unwrap()
            .objects
            .iter()
            .find(|object| {
                object["kind"] == "Interface"
                    && object["name"] == "root"
                    && object.get("spec").is_none()
            })
            .unwrap()["id"]
            .as_str()
            .unwrap()
            .to_owned();
        shell
            .issue_client_token(&id)
            .expect("could not provision temporary root client");
        let shutdown = CancellationToken::new();
        let runner = OwnedRunner(Some(
            Runner::new(&shell, PathBuf::from(env!("CARGO_BIN_EXE_conduit"))).unwrap(),
        ));
        let programs = tokio::task::spawn_blocking(scripted_agents).await.unwrap();
        let fail_setup = Arc::new(AtomicBool::new(false));
        let executor = Arc::new(server::ServingExecutor::new(Arc::new(ReplayExecutor {
            runner: runner.0.as_ref().unwrap().executor(),
            filesystem: Filesystem {
                fail_setup: fail_setup.clone(),
                ..Filesystem::default()
            },
            agents: Mutex::default(),
            shell: Arc::downgrade(&shell),
            programs,
        })));
        let completion: Arc<dyn Executor> = executor.clone();
        runner
            .0
            .as_ref()
            .unwrap()
            .set_completion_executor(Arc::downgrade(&completion));
        let (ready, started) = oneshot::channel();
        let stop = shutdown.clone();
        drop(reserved);
        let server = tokio::spawn(server::serve_with_ready(shell, executor, stop, move || {
            let _ = ready.send(());
        }));
        timeout(WAIT, started)
            .await
            .expect("server startup timed out")
            .expect("server could not serve");
        let root = timeout(
            WAIT,
            ().serve(StreamableHttpClientTransport::with_client(
                reqwest::Client::builder()
                    .no_proxy()
                    .retry(reqwest::retry::never())
                    .build()
                    .unwrap(),
                StreamableHttpClientTransportConfig::with_uri(endpoint)
                    .auth_header(
                        fs::read_to_string(directory.join("client-tokens").join(id))
                            .expect("temporary root client unavailable"),
                    )
                    .reinit_on_expired_session(false),
            )),
        )
        .await
        .expect("root MCP initialization timed out")
        .unwrap_or_else(|_| panic!("root MCP initialization failed"));
        Self {
            root,
            shutdown,
            server,
            _runner: runner,
            _store: store,
            fail_setup,
        }
    }

    pub async fn call(&self, tool: &str, arguments: Value, key: Option<&str>) -> Value {
        let mut request = CallToolRequestParams::new(tool.to_owned())
            .with_arguments(arguments.as_object().unwrap().clone());
        if let Some(key) = key {
            request.meta = Some(
                serde_json::from_value(json!({
                    format!("{META}idempotencyKey"):key,
                    "io.modelcontextprotocol/clientCapabilities":{"extensions":{
                        format!("{META}implementation-interface"):{"revision":"v0"}
                    }}
                }))
                .unwrap(),
            );
        }
        match timeout(WAIT, self.root.call_tool(request))
            .await
            .unwrap_or_else(|_| panic!("{tool}: request timed out"))
        {
            Ok(result) => serde_json::to_value(result).unwrap(),
            Err(ServiceError::McpError(error)) => json!({"protocolError":error}),
            Err(_) => panic!("{tool}: transport failed"),
        }
    }

    pub async fn close(self) {
        let _ = self.root.cancel().await;
        self.shutdown.cancel();
        timeout(WAIT, self.server)
            .await
            .expect("server shutdown timed out")
            .unwrap()
            .expect("server failed");
    }
}

pub fn result(raw: &Value) -> Value {
    let mut result = raw
        .get("structuredContent")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    for key in ["invocation", "pendingAction"] {
        if let Some(value) = raw["_meta"].get(format!("{META}{key}")) {
            result.insert(key.into(), value.clone());
        }
    }
    let outcome = raw["_meta"]
        .get(format!("{META}outcome"))
        .or_else(|| raw["_meta"].get(format!("{META}reason")))
        .or_else(|| raw["protocolError"]["data"].get(format!("{META}reason")));
    result.insert("outcome".into(), outcome.cloned().unwrap_or(Value::Null));
    if let Some(value) = raw["protocolError"]["data"].get("diagnostics") {
        result.insert("diagnostics".into(), value.clone());
    }
    Value::Object(result)
}

fn list(value: &Value) -> &[Value] {
    value.as_array().map(Vec::as_slice).unwrap_or_default()
}

fn generated(name: &str) -> Option<&str> {
    ["root-pool-", "root-fleet-"]
        .into_iter()
        .find(|prefix| name.starts_with(prefix))
}

fn symbolic_id(name: &str) -> bool {
    name.starts_with("00000000-0000-4000-8000-") && name.len() == 36
}

#[derive(Clone, Default)]
struct Bindings(BTreeMap<String, String>);

impl Bindings {
    fn bind(&mut self, symbol: &str, observed: &str) -> bool {
        if let Some(existing) = self.0.get(symbol) {
            return existing == observed;
        }
        if self.0.values().any(|value| value == observed) {
            return false;
        }
        self.0.insert(symbol.into(), observed.into());
        true
    }

    fn get(&self, symbol: &str) -> Option<&str> {
        self.0.get(symbol).map(String::as_str)
    }
}

#[derive(Clone, Default)]
pub struct Symbols {
    ids: Bindings,
    names: Bindings,
    revisions: BTreeMap<String, Bindings>,
}

impl Symbols {
    #[allow(dead_code)] // Used by the planner integration tests.
    pub fn kept_for_plan(&self, value: &Value) -> Value {
        let mut mapped = self.substitute(value).unwrap();
        if let Some(rows) = mapped["objects"].as_array_mut() {
            for (original, mapped) in list(&value["objects"]).iter().zip(rows) {
                if let Some(revision) = self
                    .revisions
                    .get(original[2].as_str().unwrap())
                    .and_then(|r| r.get(original[3].as_str().unwrap()))
                {
                    mapped[3] = json!(revision);
                }
            }
        }
        mapped
    }

    #[allow(dead_code)] // Used by the planner integration tests.
    pub fn plan_text(&self, text: &str) -> String {
        let mut out = text.to_owned();
        for (expected, observed) in self.ids.0.iter().chain(&self.names.0) {
            out = out.replace(expected, observed);
        }
        out
    }

    fn bind_base(&mut self, rows: &[Value]) {
        for (n, kind, name) in [
            (1, "Pool", "system"),
            (2, "Resource", "local"),
            (3, "Resource", "management"),
            (4, "Resource", "access"),
            (5, "Interface", "root"),
            (6, "Conduit", "root-management"),
            (7, "Conduit", "root-access"),
        ] {
            let row = rows
                .iter()
                .find(|row| {
                    row["kind"] == kind
                        && if kind == "Conduit" {
                            row["name"]
                                .as_str()
                                .is_some_and(|label| label.starts_with(&format!("{name}-")))
                        } else {
                            row["name"] == name
                        }
                })
                .unwrap_or_else(|| panic!("first-start list omitted {kind}/{name}"));
            assert!(self.ids.bind(
                &format!("00000000-0000-4000-8000-{n:012}"),
                row["id"].as_str().unwrap()
            ));
        }
    }

    fn arguments(&self, request: &Value, state: &Value) -> Result<Value, &'static str> {
        let arguments = &request["arguments"];
        let mut translated = self.substitute(arguments)?;
        if let Some(symbol) = arguments["revision"].as_str() {
            let name = arguments["template"]["metadata"]["name"]
                .as_str()
                .ok_or("template name absent")?;
            let template = list(&state["kept"]["objects"])
                .iter()
                .find(|row| {
                    row[0] == arguments["template"]["kind"]
                        && row[1] == name
                        && row[4] == "template"
                })
                .ok_or("template symbol absent")?;
            let id = template[2].as_str().unwrap();
            translated["revision"] = json!(
                self.revisions
                    .get(id)
                    .and_then(|r| r.get(symbol))
                    .ok_or("template revision is unbound")?
            );
        }
        Ok(translated)
    }

    fn substitute(&self, value: &Value) -> Result<Value, &'static str> {
        Ok(match value {
            Value::String(text) if symbolic_id(text) => {
                json!(self.ids.get(text).ok_or("ID is unbound")?)
            }
            Value::String(text) if generated(text).is_some() => {
                json!(self.names.get(text).ok_or("generated name is unbound")?)
            }
            Value::Object(fields) => Value::Object(
                fields
                    .iter()
                    .map(|(key, value)| self.substitute(value).map(|value| (key.clone(), value)))
                    .collect::<Result<_, _>>()?,
            ),
            Value::Array(items) => Value::Array(
                items
                    .iter()
                    .map(|item| self.substitute(item))
                    .collect::<Result<_, _>>()?,
            ),
            _ => value.clone(),
        })
    }

    fn bind_revision(&mut self, object: &Value, expected: &Value, actual: &Value) -> bool {
        let Some(symbol) = expected.as_str() else {
            return false;
        };
        let Some(revision) = actual.as_str() else {
            return false;
        };
        self.revisions
            .entry(object.as_str().unwrap().into())
            .or_default()
            .bind(symbol, revision)
    }

    fn matches(&mut self, expected: &Value, actual: &Value) -> bool {
        match (expected, actual) {
            (Value::String(e), Value::String(a)) if symbolic_id(e) => {
                uuid::Uuid::parse_str(a).is_ok() && self.ids.bind(e, a)
            }
            (Value::String(e), Value::String(a)) if generated(e).is_some() => {
                generated(e) == generated(a) && self.names.bind(e, a)
            }
            (Value::Object(e), Value::Object(a)) => e.iter().fold(true, |ok, (key, expected)| {
                a.get(key)
                    .is_some_and(|actual| self.matches(expected, actual))
                    && ok
            }),
            (Value::Array(e), Value::Array(a)) => e
                .iter()
                .zip(a)
                .fold(e.len() == a.len(), |ok, (e, a)| self.matches(e, a) && ok),
            _ => expected == actual,
        }
    }

    fn named(&self, expected: &Value, actual: &Value) -> bool {
        expected["kind"] == actual["kind"]
            && (expected["name"] == actual["name"]
                || expected["name"]
                    .as_str()
                    .zip(actual["name"].as_str())
                    .is_some_and(|(e, a)| {
                        self.names.get(e).map_or_else(
                            || generated(e).is_some() && generated(e) == generated(a),
                            |name| name == a,
                        )
                    }))
    }
}

fn pin_order(value: &Value) -> Vec<Value> {
    let mut pins = list(value).to_vec();
    pins.sort_by_key(|pin| pin.to_string());
    pins
}

fn references(value: &Value, objects: &[Value]) -> Value {
    match value {
        Value::Object(fields) if fields.contains_key("id") && fields.contains_key("kind") => {
            objects
                .iter()
                .find(|row| row["id"] == value["id"] && row["kind"] == value["kind"])
                .map(|row| json!({"kind":row["kind"],"name":row["name"]}))
                .unwrap_or_else(|| value.clone())
        }
        _ => value.clone(),
    }
}

fn compact(row: &Value, actual: &Value, objects: &[Value]) -> Value {
    let declared = &actual["declared"];
    let definition = if row["template"] == true {
        declared["spec"].clone()
    } else {
        let mut definition = serde_json::Map::new();
        for (field, projected) in [
            ("from", "from"),
            ("owner", "owner"),
            ("consumer", "consumer"),
            ("target", "target"),
            ("local_name", "as"),
            ("engine_made", "engine-made"),
        ] {
            if let Some(value) = declared.get(field) {
                let value = references(value, objects);
                definition.insert(
                    projected.into(),
                    if matches!(field, "owner" | "from") && value["name"].is_string() {
                        value["name"].clone()
                    } else {
                        value
                    },
                );
            }
        }
        let mut resolved = serde_json::Map::new();
        for field in ["type", "config", "capability", "operations"] {
            if let Some(value) = declared.get(field) {
                resolved.insert(field.into(), value.clone());
            }
        }
        if !resolved.is_empty() {
            definition.insert("resolved".into(), Value::Object(resolved));
        }
        if let Some(provenance) = actual.get("provenance") {
            definition.insert("provenance".into(), provenance.clone());
            if provenance["cause"] == "default" {
                definition.insert("engine-made".into(), Value::Bool(true));
            }
        }
        Value::Object(definition)
    };
    json!([
        row["kind"],
        row["name"],
        actual["id"],
        actual["revision"],
        if row["template"] == true {
            json!("template")
        } else {
            actual["status"]["state"].clone()
        },
        definition,
        actual["packages"],
    ])
}

pub struct Replay {
    pub lines: BTreeSet<String>,
    started: i64,
}

impl Replay {
    fn mismatch(
        &mut self,
        case: &str,
        step: &str,
        path: &str,
        promise: &str,
        want: &Value,
        got: &Value,
    ) {
        let line = format!("{case} {step} {path} {promise}");
        if self.lines.insert(line.clone()) {
            println!(
                "Mismatch {line} expected {} observed {}",
                safe_projection(want),
                safe_projection(got)
            );
        }
    }

    fn compare_objects(
        &mut self,
        symbols: &mut Symbols,
        case: &str,
        step: &str,
        path: &str,
        expected: &Value,
        actual: &Value,
    ) {
        let e = list(expected);
        let a = list(actual);
        if e.len() != a.len() || !actual.is_array() {
            self.mismatch(
                case,
                step,
                path,
                "FLEET-005",
                &json!(e.len()),
                &json!(a.len()),
            );
        }
        for (i, want) in e.iter().enumerate() {
            let got = a
                .iter()
                .find(|got| symbols.named(want, got))
                .unwrap_or(&Value::Null);
            if !symbols.matches(want, got) {
                self.mismatch(case, step, &format!("{path}/{i}"), "FLEET-005", want, got);
            }
        }
    }

    fn compare_view(
        &mut self,
        symbols: &mut Symbols,
        case: &str,
        step: &str,
        path: &str,
        expected: &Value,
        actual: &Value,
    ) {
        if actual.is_null() {
            self.mismatch(case, step, path, "LIF-013", expected, actual);
            return;
        }
        for i in 0..7 {
            let matches = match i {
                3 => symbols.bind_revision(&expected[2], &expected[i], &actual[i]),
                6 => symbols.matches(
                    &json!(pin_order(&expected[i])),
                    &json!(pin_order(&actual[i])),
                ),
                _ => symbols.matches(&safe_projection(&expected[i]), &safe_projection(&actual[i])),
            };
            if !matches {
                self.mismatch(
                    case,
                    step,
                    &format!("{path}/{i}"),
                    if i == 6 { "NAM-009" } else { "LIF-013" },
                    &expected[i],
                    &actual[i],
                );
            }
        }
        if expected[5]["engine-made"] == true {
            let mut operations = vec![
                "register.v1",
                "inspect.v1",
                "list.v1",
                "set-up.v1",
                "retire.v1",
                "reconcile.v1",
                "sign-in.v1",
                "answer.v1",
            ];
            if expected[5]["target"]["kind"] == "Pool" {
                operations.insert(1, "register-replacement.v1");
            }
            let operations = json!(
                operations
                    .iter()
                    .map(|name| { format!("github.com/shrav-k/conduit/core#{name}") })
                    .collect::<Vec<_>>()
            );
            let actual = &actual[5]["resolved"]["operations"];
            if pin_order(&operations) != pin_order(actual) {
                self.mismatch(
                    case,
                    step,
                    &format!("{path}/5"),
                    "FLEET-005",
                    &operations,
                    actual,
                );
            }
        }
    }

    fn compare_tombstones(
        &mut self,
        symbols: &mut Symbols,
        case: &str,
        step: &str,
        path: &str,
        expected: &Value,
        actual: &Value,
    ) {
        let e = list(expected);
        let a = list(actual);
        if e.len() != a.len() || !actual.is_array() {
            self.mismatch(
                case,
                step,
                path,
                "LIF-040",
                &json!(e.len()),
                &json!(a.len()),
            );
        }
        for (i, want) in e.iter().enumerate() {
            let got = a
                .iter()
                .find(|got| {
                    symbols
                        .ids
                        .get(want["object"]["id"].as_str().unwrap_or(""))
                        .map_or_else(
                            || symbols.named(&want["object"], &got["object"]),
                            |id| got["object"]["id"] == id,
                        )
                })
                .unwrap_or(&Value::Null);
            if !got.is_null()
                && !got["time"]
                    .as_str()
                    .and_then(|time| chrono::DateTime::parse_from_rfc3339(time).ok())
                    .is_some_and(|time| {
                        (self.started..=chrono::Utc::now().timestamp()).contains(&time.timestamp())
                    })
            {
                self.mismatch(
                    case,
                    step,
                    &format!("{path}/{i}/time"),
                    "LIF-040",
                    &json!("timestamp within replay"),
                    &got["time"],
                );
            }
            if !symbols.matches(want, got) {
                self.mismatch(case, step, &format!("{path}/{i}"), "LIF-040", want, got);
            }
        }
    }

    async fn compare_kept(
        &mut self,
        engine: &Engine,
        symbols: &mut Symbols,
        case: &Value,
        base: &BTreeSet<String>,
        current: &Value,
    ) {
        let id = case["id"].as_str().unwrap();
        let wanted = list(&case["kept"]["objects"]);
        let rows = list(&current["objects"]);
        let actual: Vec<_> = rows
            .iter()
            .filter(|row| {
                matches!(
                    row["kind"].as_str(),
                    Some("Agent" | "Conduit" | "Fleet" | "Interface" | "Pool" | "Resource")
                ) && row["id"].as_str().is_some_and(|id| !base.contains(id))
            })
            .collect();
        if wanted.len() != actual.len() {
            self.mismatch(
                id,
                "-",
                "/kept/objects",
                "LIF-013",
                &json!(wanted.len()),
                &json!(actual.len()),
            );
        }
        for (i, want) in wanted.iter().enumerate() {
            let key = json!({"kind":want[0],"name":want[1]});
            let path = format!("/kept/objects/{i}");
            let Some(row) = actual.iter().find(|row| symbols.named(&key, row)) else {
                self.mismatch(id, "-", &path, "LIF-013", &key, &Value::Null);
                continue;
            };
            let owner = want[5]["owner"].as_str();
            let route = if matches!(want[0].as_str(), Some("Resource" | "Agent"))
                && want[4] != "template"
            {
                owner.unwrap_or("management")
            } else {
                "management"
            };
            let inspected = result(
                &engine
                    .call(
                        &format!("{route}.inspect.v1"),
                        json!({
                            "object":{"kind":row["kind"],"name":row["name"]},"expectedId":row["id"],
                        }),
                        None,
                    )
                    .await,
            );
            let got = if inspected["outcome"] == "succeeded" {
                compact(row, &inspected, rows)
            } else {
                Value::Null
            };
            self.compare_view(symbols, id, "-", &path, want, &got);
        }
        self.compare_tombstones(
            symbols,
            id,
            "-",
            "/kept/tombstones",
            &case["kept"]["tombstones"],
            &current["tombstones"],
        );
    }
}

#[allow(dead_code)] // Used by the consistency and replay target.
pub async fn replay(cases: &[Value]) -> Replay {
    replay_visit(cases, |_, _, _| {}).await
}

pub async fn replay_visit(
    cases: &[Value],
    mut before_case: impl FnMut(&Engine, &Value, &Symbols),
) -> Replay {
    replay_while(cases, |engine, case, symbols| {
        before_case(engine, case, symbols);
        true
    })
    .await
}

pub async fn replay_while(
    cases: &[Value],
    mut before_case: impl FnMut(&Engine, &Value, &Symbols) -> bool,
) -> Replay {
    let started = chrono::Utc::now().timestamp();
    let engine = Engine::start().await;
    let initial = result(&engine.call("management.list.v1", json!({}), None).await);
    assert_eq!(initial["outcome"], "succeeded", "first-start list failed");
    let mut symbols = Symbols::default();
    symbols.bind_base(list(&initial["objects"]));
    let base = list(&initial["objects"])
        .iter()
        .filter_map(|row| row["id"].as_str().map(str::to_owned))
        .collect();
    let archives = installs();
    let mut report = Replay {
        lines: BTreeSet::new(),
        started,
    };

    for case in cases {
        let id = case["id"].as_str().unwrap();
        let stage = id[..2].parse::<u8>().unwrap();
        let mut first = None;
        for archive in archives.iter().filter(|archive| archive.stage == stage) {
            let reply = result(
                &engine
                    .call("management.install.v1", archive.args.clone(), None)
                    .await,
            );
            if reply["outcome"] != "succeeded" {
                let rule = list(&reply["diagnostics"])
                    .first()
                    .and_then(|diagnostic| diagnostic["ruleId"].as_str())
                    .unwrap_or("-");
                let fetch = archive.args["fetch"].as_str().unwrap();
                let mut diagnostics = reply["diagnostics"].to_string().replace(fetch, "<archive>");
                if let Ok(path) = url::Url::parse(fetch).unwrap().to_file_path() {
                    diagnostics = diagnostics.replace(path.to_string_lossy().as_ref(), "<archive>");
                }
                println!(
                    "Mismatch {id} install - {rule} expected {} observed {} diagnostics {diagnostics}",
                    archive.pin, reply["outcome"],
                );
                report.lines.insert(format!("{id} install - {rule}"));
                first.get_or_insert(format!("install - {rule}"));
            }
        }
        let before = result(&engine.call("management.list.v1", json!({}), None).await);
        if pin_order(&before["packages"]) != pin_order(&case["engine"]["packages"]) {
            report.mismatch(
                id,
                "-",
                "/engine/packages",
                "NAM-011",
                &case["engine"]["packages"],
                &before["packages"],
            );
            first.get_or_insert("- /engine/packages NAM-011".into());
        }
        if !before_case(&engine, case, &symbols) {
            break;
        }
        let mut sent = 0;
        let mut matched = 0;
        for (i, request) in list(&case["requests"]).iter().enumerate() {
            let step = (i + 1).to_string();
            let pointer = format!("/requests/{i}");
            let args = match symbols.arguments(request, case) {
                Ok(args) => args,
                Err(cause) => {
                    report.mismatch(
                        id,
                        &step,
                        &format!("{pointer}/arguments"),
                        "-",
                        &json!("bound symbols"),
                        &json!(cause),
                    );
                    first.get_or_insert(format!("{step} {pointer}/arguments -"));
                    continue;
                }
            };
            let tool = request["tool"].as_str().unwrap();
            sent += 1;
            let got = result(
                &engine
                    .call(tool, args.clone(), request["idempotencyKey"].as_str())
                    .await,
            );
            let start = report.lines.len();
            if tool == "s1.set-up.v1" {
                let inspection = result(
                    &engine
                        .call(
                            "s1.inspect.v1",
                            json!({
                                "object":args["instance"], "expectedId":args["expectedId"],
                            }),
                            None,
                        )
                        .await,
                );
                let raised = json!(
                    list(&inspection["status"]["pending"])
                        .iter()
                        .filter(|action| action["kind"] == "sign-in")
                        .map(|action| &action["id"])
                        .collect::<Vec<_>>()
                );
                let expected = request["result"]
                    .get("pendingAction")
                    .cloned()
                    .unwrap_or(json!([]));
                if !symbols.matches(&expected, &raised) {
                    report.mismatch(
                        id,
                        &step,
                        &format!("{pointer}/result/pendingAction"),
                        "PND-005",
                        &expected,
                        &raised,
                    );
                }
            }
            for (name, want) in request["result"].as_object().unwrap() {
                let path = format!("{pointer}/result/{name}");
                match name.as_str() {
                    "objects" => {
                        report.compare_objects(&mut symbols, id, &step, &path, want, &got[name])
                    }
                    "view" => {
                        let rows =
                            result(&engine.call("management.list.v1", json!({}), None).await);
                        let rows = list(&rows["objects"]);
                        let row = rows.iter().find(|row| row["id"] == got["id"]);
                        let view = row
                            .map(|row| compact(row, &got, rows))
                            .unwrap_or(Value::Null);
                        report.compare_view(&mut symbols, id, &step, &path, want, &view);
                    }
                    "tombstones" | "remaining" => {
                        report.compare_tombstones(&mut symbols, id, &step, &path, want, &got[name]);
                    }
                    "packages" => {
                        if pin_order(want) != pin_order(&got[name]) {
                            report.mismatch(id, &step, &path, "NAM-011", want, &got[name]);
                        }
                    }
                    "revision" => {
                        let template = list(&case["kept"]["objects"])
                            .iter()
                            .find(|row| {
                                row[0] == args["template"]["kind"]
                                    && row[1] == args["template"]["metadata"]["name"]
                                    && row[4] == "template"
                            })
                            .unwrap();
                        if !symbols.bind_revision(&template[2], want, &got[name]) {
                            report.mismatch(id, &step, &path, "LIF-037", want, &got[name]);
                        }
                    }
                    _ => {
                        let value = got.get(name);
                        if !value.is_some_and(|got| symbols.matches(want, got)) {
                            let rule = match name.as_str() {
                                "outcome" => list(&got["diagnostics"])
                                    .first()
                                    .and_then(|d| d["ruleId"].as_str())
                                    .unwrap_or("OPS-022"),
                                "pendingAction" => "IFC-025",
                                "diagnostics" => "NAM-008",
                                _ => "IFC-025",
                            };
                            report.mismatch(
                                id,
                                &step,
                                &path,
                                rule,
                                want,
                                value.unwrap_or(&Value::Null),
                            );
                        }
                    }
                }
            }
            if report.lines.len() == start {
                matched += 1;
            } else {
                first.get_or_insert_with(|| {
                    report
                        .lines
                        .iter()
                        .find_map(|line| line.strip_prefix(&format!("{id} {step} ")))
                        .map(|line| format!("{step} {line}"))
                        .unwrap()
                });
            }
        }
        let after = result(&engine.call("management.list.v1", json!({}), None).await);
        let start = report.lines.len();
        report
            .compare_kept(&engine, &mut symbols, case, &base, &after)
            .await;
        if report.lines.len() != start {
            first.get_or_insert_with(|| {
                report
                    .lines
                    .iter()
                    .find_map(|line| line.strip_prefix(&format!("{id} - /kept")))
                    .map(|line| format!("- /kept{line}"))
                    .unwrap()
            });
        }
        println!(
            "Measures {id} {sent} {matched} {}",
            first.as_deref().unwrap_or("-")
        );
    }
    engine.close().await;
    report
}

#[allow(dead_code)] // Used by the apply integration tests.
pub fn check_apply_case(
    engine: &Engine,
    case: &Value,
    seed: &Symbols,
    entries: &[Value],
    ledger: &Value,
    digest: &str,
) {
    let rows = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { result(&engine.call("management.list.v1", json!({}), None).await) })
    });
    let rows = list(&rows["objects"]);
    let mut symbols = seed.clone();
    symbols.bind_base(rows);
    let mut replay = Replay {
        lines: BTreeSet::new(),
        started: 0,
    };
    let case_id = case["id"].as_str().unwrap();
    for (index, (request, entry)) in list(&case["requests"]).iter().zip(entries).enumerate() {
        let step = (index + 1).to_string();
        let arguments = symbols
            .arguments(request, case)
            .expect("earlier request symbols");
        assert_eq!(
            entry["arguments"],
            safe_projection(&arguments),
            "arguments at step {step}"
        );
        assert_eq!(entry["flag"], request["flag"]);
        let actual = &entry["result"];
        for (member, expected) in request["result"].as_object().unwrap() {
            match member.as_str() {
                "objects" => replay.compare_objects(
                    &mut symbols,
                    case_id,
                    &step,
                    "/result/objects",
                    expected,
                    &actual[member],
                ),
                "view" => {
                    let row = json!({"kind":expected[0],"name":expected[1],"template":expected[4]=="template"});
                    let view = compact(&row, actual, rows);
                    replay.compare_view(
                        &mut symbols,
                        case_id,
                        &step,
                        "/result/view",
                        expected,
                        &view,
                    );
                }
                "tombstones" => replay.compare_tombstones(
                    &mut symbols,
                    case_id,
                    &step,
                    "/result/tombstones",
                    expected,
                    &actual[member],
                ),
                "packages" => assert_eq!(
                    pin_order(expected),
                    pin_order(&actual[member]),
                    "pins at step {step}"
                ),
                _ => assert!(
                    symbols.matches(expected, &actual[member]),
                    "{case_id} step {step} result member {member}"
                ),
            }
        }
    }
    for want in list(&case["kept"]["objects"]) {
        let id = symbols
            .ids
            .get(want[2].as_str().unwrap())
            .expect("known kept ID");
        let got = list(&ledger["objects"])
            .iter()
            .find(|row| row[2] == id)
            .expect("ledger object");
        assert!(symbols.matches(&want[0], &got[0]));
        assert!(symbols.matches(&want[1], &got[1]));
        assert!(
            symbols.bind_revision(&want[2], &want[3], &got[3]),
            "kept object's revision"
        );
        assert_eq!(got[4], want[4], "kept object's lifecycle");
        assert!(
            symbols.matches(&safe_projection(&want[5]), &got[5]),
            "kept definition of {}",
            want[1]
        );
        assert_eq!(pin_order(&want[6]), pin_order(&got[6]));
    }
    replay.compare_tombstones(
        &mut symbols,
        case_id,
        "-",
        "/kept/tombstones",
        &case["kept"]["tombstones"],
        &ledger["tombstones"],
    );
    assert!(
        replay.lines.is_empty(),
        "mapped golden reply or ledger mismatch"
    );
    let records = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            result(
                &engine
                    .call(
                        "management.inspect.v1",
                        json!({"object":{"kind":"Interface","name":"root"}}),
                        None,
                    )
                    .await,
            )
        })
    });
    let prefix = format!("plan:{digest}:");
    let sent: Vec<_> = list(&records["records"])
        .iter()
        .filter(|record| {
            record["type"] == "access-decision"
                && record["data"]["idempotencyKey"]
                    .as_str()
                    .is_some_and(|key| key.starts_with(&prefix))
        })
        .map(|record| &record["data"]["tool"])
        .collect();
    assert_eq!(
        sent,
        entries
            .iter()
            .map(|entry| &entry["tool"])
            .collect::<Vec<_>>()
    );
}
