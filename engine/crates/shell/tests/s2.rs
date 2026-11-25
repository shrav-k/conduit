//! S2's dry run on the engine, with a fresh loopback stub and store per test.
mod package_target;

use axum::{
    Router,
    body::{Body, to_bytes},
    extract::Request,
    http::{StatusCode, header},
    response::Response,
    routing::any,
};
use chrono::{DateTime, Utc};
use conduit_shell::{Config, Shell, boundary};
use futures_util::StreamExt;
use package_target::package_target_dir;
use rmcp::{
    ClientHandler, RoleClient, ServiceExt,
    model::{CallToolRequestParams, ClientConfig, ProtocolVersion},
    service::{NotificationContext, RunningService, ServiceError},
    transport::{
        StreamableHttpClientTransport, streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use serde_json::{Value, json};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    net::TcpListener,
    ops::Deref,
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    process::{Output, Stdio},
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};
use tokio::{
    process::{Child, Command},
    time::{sleep, timeout},
};
use uuid::Uuid;

const API: &str = "github.com/shrav-k/conduit/v0";
const CORE: &str = "github.com/shrav-k/conduit/core";
const FILE: &str = "example.org/conduit/provider-file";
const RULES: &str = "example.org/conduit/scenario-s2-linear-restrictions";
const LINEAR: &str = "example.org/conduit/scenario-s2-linear";
const META: &str = "io.github.shrav-k.conduit/";
const READ: &str = "read.read-issue.v1";
const UPDATE: &str = "write.update-issue.v1";
const WAIT: Duration = Duration::from_secs(20);
// Match the response timeout in packages/scenario-s2-linear/README.md.
const AWARE_REQUEST_DEADLINE: Duration = Duration::from_secs(2);
static SCENARIO: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn cargo(directory: &Path, args: &[&str]) {
    let log = tempfile::tempfile().unwrap();
    let mut child = std::process::Command::new("cargo")
        .current_dir(directory)
        .envs(package_target_dir(directory).map(|dir| ("CARGO_TARGET_DIR", dir)))
        .env("CARGO_NET_OFFLINE", "true")
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(log.try_clone().unwrap())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(480);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("offline build timed out in {}", directory.display());
        }
        std::thread::sleep(Duration::from_millis(25));
    };
    if !status.success() {
        use std::io::{Read, Seek};
        let mut log = log;
        let length = log.metadata().unwrap().len();
        log.seek(std::io::SeekFrom::Start(length.saturating_sub(4096)))
            .unwrap();
        let mut tail = String::new();
        log.read_to_string(&mut tail).unwrap();
        panic!("offline build failed in {}:\n{tail}", directory.display());
    }
}

struct Artifacts {
    stub: PathBuf,
    packages: Vec<(&'static str, PathBuf, String)>,
}
fn artifacts() -> &'static Artifacts {
    static CACHE: OnceLock<Artifacts> = OnceLock::new();
    CACHE.get_or_init(|| {
        let repo = repo();
        let packages = [
            (FILE, "provider-file"),
            (RULES, "scenario-s2-linear-restrictions"),
            (LINEAR, "scenario-s2-linear"),
        ]
        .into_iter()
        .map(|(address, name)| {
            let directory = repo.join("packages").join(name);
            if address == LINEAR {
                cargo(
                    &directory,
                    &["build", "--offline", "--locked", "--release", "--bins"],
                );
            }
            cargo(
                &directory,
                &[
                    "run",
                    "--offline",
                    "--locked",
                    "--release",
                    "--bin",
                    "package",
                ],
            );
            let archive = directory.join("dist").join(format!("{name}.tar.gz"));
            let digest = conduit_core::packages::bytes_digest(&fs::read(&archive).unwrap());
            (address, archive, digest)
        })
        .collect();
        let directory = repo.join("experiments/013-s2-shared-login/linear-stub");
        cargo(
            &directory,
            &["build", "--offline", "--locked", "--release", "--bins"],
        );
        let target = package_target_dir(&directory).unwrap_or_else(|| {
            std::env::var_os("CARGO_TARGET_DIR")
                .map(PathBuf::from)
                .map(|path| directory.join(path))
                .unwrap_or_else(|| directory.join("target"))
        });
        Artifacts {
            stub: target.join("release/s2-linear-stub"),
            packages,
        }
    })
}

#[derive(Default)]
struct Audit {
    markers: Mutex<Vec<String>>,
    repeat_markers: Mutex<Vec<String>>,
    inbound_leak: AtomicBool,
}
impl Audit {
    fn add(&self, marker: String) {
        assert!(!marker.is_empty());
        self.markers.lock().unwrap().push(marker);
    }
    fn bytes(&self, bytes: &[u8], source: &str) {
        assert!(!self.contains_marker(bytes), "synthetic marker in {source}");
    }
    fn contains_marker(&self, bytes: &[u8]) -> bool {
        contains_marker(bytes, &self.markers.lock().unwrap())
    }
    fn inbound(&self, payload: &[u8]) {
        let forbidden = |bytes: &[u8]| {
            self.contains_marker(bytes)
                || contains_marker(bytes, &self.repeat_markers.lock().unwrap())
        };
        let decoded = serde_json::from_slice::<Value>(payload).ok();
        if forbidden(payload)
            || decoded.is_some_and(|value| forbidden(value.to_string().as_bytes()))
        {
            // A panic in a transport task alone would not fail its scenario.
            self.inbound_leak.store(true, Ordering::SeqCst);
        }
    }
    fn value(&self, value: &impl serde::Serialize, source: &str) {
        self.bytes(&serde_json::to_vec(value).unwrap(), source);
    }
    fn scan(&self, directory: &Path) {
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let kind = entry.file_type().unwrap();
            if kind.is_dir() {
                self.scan(&entry.path());
            } else if kind.is_file() {
                self.bytes(&fs::read(entry.path()).unwrap(), "store file");
            } else if kind.is_symlink() {
                self.bytes(
                    fs::read_link(entry.path())
                        .unwrap()
                        .as_os_str()
                        .as_encoded_bytes(),
                    "store symlink",
                );
            }
        }
    }
}

fn contains_marker(bytes: &[u8], markers: &[String]) -> bool {
    markers.iter().any(|marker| {
        bytes
            .windows(marker.len())
            .any(|part| part == marker.as_bytes())
    })
}

#[derive(Default)]
struct SseAudit {
    line: Vec<u8>,
    data: Vec<u8>,
    after_cr: bool,
}

impl SseAudit {
    fn receive(&mut self, chunk: &[u8], audit: &Audit) {
        for &byte in chunk {
            if std::mem::replace(&mut self.after_cr, byte == b'\r') && byte == b'\n' {
                continue;
            }
            if !matches!(byte, b'\r' | b'\n') {
                self.line.push(byte);
                continue;
            }
            let line = self
                .line
                .strip_prefix(b"\xef\xbb\xbf")
                .unwrap_or(&self.line);
            if line.is_empty() {
                audit.inbound(&self.data);
                self.data.clear();
            } else if line == b"data" {
                self.data.push(b'\n');
            } else if let Some(data) = line.strip_prefix(b"data:") {
                self.data
                    .extend_from_slice(data.strip_prefix(b" ").unwrap_or(data));
                self.data.push(b'\n');
            }
            self.line.clear();
        }
    }
}

// Let rmcp handle the protocol while the test has access to the raw response
// bodies, including any notification fields the typed transport discards.
struct AgentProxy(tokio::task::JoinHandle<()>);

impl Drop for AgentProxy {
    fn drop(&mut self) {
        self.0.abort();
    }
}

impl AgentProxy {
    async fn start(http: reqwest::Client, upstream: String, audit: Arc<Audit>) -> (Self, String) {
        let source = url::Url::parse(&upstream).unwrap();
        assert!(source.scheme() == "http" && source.host_str() == Some("127.0.0.1"));
        let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
            .await
            .unwrap();
        let address = listener.local_addr().unwrap();
        let router = Router::new().route(
            "/mcp",
            any(move |request: Request| {
                let (http, upstream, audit) = (http.clone(), upstream.clone(), audit.clone());
                async move {
                    relay(http, upstream, audit, request)
                        .await
                        .unwrap_or_else(|| {
                            Response::builder()
                                .status(StatusCode::BAD_GATEWAY)
                                .body(Body::empty())
                                .unwrap()
                        })
                }
            }),
        );
        let server = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        (Self(server), format!("http://{address}/mcp"))
    }
}

async fn relay(
    http: reqwest::Client,
    upstream: String,
    audit: Arc<Audit>,
    request: Request,
) -> Option<Response> {
    let (mut parts, body) = request.into_parts();
    for key in [header::HOST, header::CONNECTION, header::TRANSFER_ENCODING] {
        parts.headers.remove(key);
    }
    let body = to_bytes(body, 1 << 20).await.ok()?;
    let answer = http
        .request(parts.method, upstream)
        .headers(parts.headers)
        .body(body)
        .send()
        .await
        .ok()?;
    let status = answer.status();
    let mut headers = answer.headers().clone();
    for key in [header::CONNECTION, header::TRANSFER_ENCODING] {
        headers.remove(key);
    }
    let body = if headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.split(';').next() == Some("text/event-stream"))
    {
        let mut events = SseAudit::default();
        Body::from_stream(answer.bytes_stream().map(move |chunk| {
            if let Ok(bytes) = &chunk {
                events.receive(bytes, &audit);
            }
            chunk
        }))
    } else {
        let bytes = answer.bytes().await.ok()?;
        audit.inbound(&bytes);
        Body::from(bytes)
    };
    let mut response = Response::new(body);
    *response.status_mut() = status;
    *response.headers_mut() = headers;
    Some(response)
}

#[tokio::test]
async fn s2_wire_audit_detects_a_marker_in_an_unknown_notification_field() {
    let audit = Arc::new(Audit::default());
    let marker = Uuid::new_v4().to_string();
    audit.add(marker.clone());
    let message = json!({"jsonrpc":"2.0","method":"notifications/message",
        "params":{"level":"info","data":"ordinary"},"extra":marker});
    let body = format!("data: {message}\n\n");
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let upstream = AgentProxy(tokio::spawn(async move {
        let router = Router::new().route(
            "/mcp",
            any(move || {
                let body = body.clone();
                async { ([(header::CONTENT_TYPE, "text/event-stream")], body) }
            }),
        );
        axum::serve(listener, router).await.unwrap();
    }));
    let (_proxy, url) =
        AgentProxy::start(http(), format!("http://{address}/mcp"), audit.clone()).await;
    let response = http().get(url).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    response.bytes().await.unwrap();
    assert!(
        audit.inbound_leak.load(Ordering::SeqCst),
        "the wire audit missed a notification marker"
    );
    drop(upstream);
}

#[derive(Clone)]
struct Consumer {
    notices: Arc<Mutex<usize>>,
}
impl ClientHandler for Consumer {
    fn get_info(&self) -> ClientConfig {
        let mut config = ClientConfig::default();
        config.protocol_version = ProtocolVersion::V_2025_06_18;
        config
    }
    async fn on_tool_list_changed(&self, _: NotificationContext<RoleClient>) {
        *self.notices.lock().unwrap() += 1;
    }
}
struct Session {
    service: RunningService<RoleClient, Consumer>,
    _proxy: AgentProxy,
}
impl Deref for Session {
    type Target = RunningService<RoleClient, Consumer>;

    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

fn meta(value: &Value, key: &str) -> Value {
    value["_meta"][format!("{META}{key}")].clone()
}
fn named(kind: &str, name: &str) -> Value {
    json!({"kind":kind,"name":name})
}
fn doc(kind: &str, name: &str, body: Value) -> Value {
    let mut result = json!({"apiVersion":API,"kind":kind,"metadata":{"name":name}});
    result
        .as_object_mut()
        .unwrap()
        .extend(body.as_object().unwrap().clone());
    result
}
fn conduit(name: &str, capability: &str, operations: &[&str], requirement: Option<Value>) -> Value {
    let mut spec = json!({"capability":capability,"operations":operations});
    if let Some(requirement) = requirement {
        spec["requirements"] = json!({"limit":requirement});
    }
    doc("Conduit", name, json!({"spec":spec}))
}
fn filter(issues: &[&str], deadline: DateTime<Utc>) -> Value {
    json!({"requirement":format!("{RULES}#issue-scope.v1"),
        "arguments":{"issues":issues,"deadline":deadline.to_rfc3339()}})
}
fn reading(name: &str, restriction: Option<Value>) -> Value {
    conduit(
        name,
        &format!("{RULES}#issue-reading.v1"),
        &[
            &format!("{RULES}#read-issue.v1"),
            &format!("{RULES}#list-issues.v1"),
        ],
        restriction,
    )
}
fn writing(name: &str, restriction: Option<Value>) -> Value {
    conduit(
        name,
        &format!("{RULES}#issue-writing.v1"),
        &[&format!("{RULES}#update-issue.v1")],
        restriction,
    )
}
fn conduit_instance(name: &str, template: &str, consumer: &str, target: &str) -> Value {
    doc(
        "Conduit",
        name,
        json!({"from":template,"consumer":named("Agent",consumer),"target":named("Resource",target)}),
    )
}
fn private_file(path: &Path, value: &str) {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .unwrap()
        .write_all(value.as_bytes())
        .unwrap();
}
async fn until(mut ready: impl FnMut() -> bool) {
    timeout(WAIT, async {
        while !ready() {
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("progress deadline");
}
fn http() -> reqwest::Client {
    reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(WAIT)
        .build()
        .unwrap()
}

#[derive(Default)]
struct Scenario {
    bad_login: bool,
    bound: bool,
    defer_a_read: bool,
    read_titles: Option<[String; 2]>,
}

struct S2 {
    temp: tempfile::TempDir,
    store: PathBuf,
    login: PathBuf,
    old: String,
    decoy: String,
    control_token: String,
    graphql: String,
    control_url: String,
    http: reqwest::Client,
    port: u16,
    engine: Option<Child>,
    _stub: Child,
    audit: Arc<Audit>,
    linear_ids: Vec<Value>,
    serve_logs: Vec<(PathBuf, PathBuf)>,
    started: Instant,
    a_deadline: DateTime<Utc>,
    _scenario: tokio::sync::MutexGuard<'static, ()>,
}

impl S2 {
    async fn new(scenario: Scenario) -> Self {
        let Scenario {
            bad_login,
            bound,
            defer_a_read,
            read_titles,
        } = scenario;
        // Each scenario owns fresh processes. Run them one at a time so another
        // startup cannot compete with the two-second in-flight-close window.
        let scenario = timeout(Duration::from_secs(900), SCENARIO.lock())
            .await
            .expect("another S2 scenario did not finish");
        let started = Instant::now();
        let artifacts = artifacts();
        let temp = tempfile::tempdir().unwrap();
        let store = temp.path().join("store");
        let login = temp.path().join("login");
        let issues = temp.path().join("issues.json");
        let connection = temp.path().join("stub.json");
        let fixture: Vec<_> = (1..=4)
            .map(|n| {
                json!({
                    "identifier":format!("SYN-{n}"),
                    "title":read_titles.as_ref().and_then(|titles| titles.get(n - 1)).cloned()
                        .unwrap_or_else(|| format!("Synthetic {n}")),
                    "description":null,"state":"Todo","updatedAt":"2026-01-01T00:00:00Z"
                })
            })
            .collect();
        fs::write(&issues, serde_json::to_vec(&fixture).unwrap()).unwrap();
        let mut stub = Command::new(&artifacts.stub)
            .env_clear()
            .args([&issues, &connection])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .unwrap();
        let info: Value = timeout(WAIT, async {
            loop {
                if let Ok(bytes) = fs::read(&connection)
                    && let Ok(info) = serde_json::from_slice(&bytes)
                {
                    break info;
                }
                assert!(
                    stub.try_wait().unwrap().is_none(),
                    "stub exited before readiness"
                );
                sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("stub did not start");
        let field = |name: &str| info[name].as_str().unwrap().to_owned();
        let audit = Arc::new(Audit::default());
        let old = field("api_key");
        let control_token = field("control_token");
        let decoy = format!("decoy-{}", Uuid::new_v4());
        for marker in [&old, &control_token, &decoy] {
            audit.add(marker.clone());
        }
        let graphql = field("graphql_url");
        let control_url = field("control_url");
        for (raw, path) in [(&graphql, "/graphql"), (&control_url, "/control/v1")] {
            let url = url::Url::parse(raw).expect("invalid stub URL");
            assert!(
                url.scheme() == "http"
                    && url.host_str() == Some("127.0.0.1")
                    && url.port().is_some()
                    && url.path() == path
                    && url.username().is_empty()
                    && url.password().is_none()
                    && url.query().is_none()
                    && url.fragment().is_none()
            );
        }
        private_file(&login, &old);
        if bad_login {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&login, fs::Permissions::from_mode(0o644)).unwrap();
            private_file(&temp.path().join("unused"), &old);
        }
        let port = TcpListener::bind(("127.0.0.1", 0))
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let mut h = Self {
            temp,
            store,
            login,
            old,
            decoy,
            control_token,
            graphql,
            control_url,
            http: http(),
            port,
            engine: None,
            _stub: stub,
            audit,
            linear_ids: vec![],
            serve_logs: vec![],
            started,
            a_deadline: Utc::now(),
            _scenario: scenario,
        };
        h.success("init", &["--url", &h.url()]).await;
        h.start().await;
        for (_, archive, digest) in &artifacts.packages {
            h.success("install", &[&format!("{}#{digest}", archive.display())])
                .await;
        }
        let file_type = format!("{FILE}#file-secret.v1");
        let mut entries = json!({"login":{"from":"login-template"},
            "linear":{"from":"linear-template","arguments":{"source":{"entry":"login"}}}});
        let mut documents = vec![
            doc(
                "Resource",
                "login-template",
                json!({"spec":{"type":file_type,"config":{"path":h.login}}}),
            ),
            conduit(
                "secret-read",
                &format!("{CORE}#secret-provider.v1"),
                &[&format!("{CORE}#get-secret.v1")],
                None,
            ),
            doc(
                "Resource",
                "linear-template",
                json!({"spec":{
                    "type":format!("{LINEAR}#linear-aware.v1"),"config":{"baseUrl":h.graphql},
                    "inputs":{"source":{"schema":{"$ref":format!("{CORE}#secret-reference.v1")},
                        "to":["/config/apiKey","/conduits/login/target"]}},
                    "conduits":{"login":{"from":"secret-read","optional":false}}
                }}),
            ),
            doc(
                "Pool",
                "s2-pool-template",
                json!({"spec":{"type":format!("{CORE}#pool.v1")}}),
            ),
            doc("Pool", "s2-pool", json!({"from":"s2-pool-template"})),
        ];
        if bad_login {
            entries["unused"] = json!({"from":"unused-template"});
            documents.push(doc(
                "Resource",
                "unused-template",
                json!({"spec":{
                    "type":file_type,"config":{"path":h.temp.path().join("unused")}
                }}),
            ));
        }
        documents[3]["spec"]["entries"] = entries;
        h.register("pool", documents).await;
        let linear = h.entry("linear");
        h.linear_ids.push(linear["id"].clone());
        let target = linear["name"].as_str().unwrap();
        let end = Utc::now() + chrono::Duration::hours(1);
        h.a_deadline = end;
        let mut a_routes = json!({});
        let b_routes = json!({"read":{"from":"read-b","target":named("Resource",target)},
            "write":{"from":"write-b","target":named("Resource",target)}});
        let mut documents = vec![
            doc(
                "Type",
                "scripted.v1",
                json!({"spec":{"family":"Agent","schema":{"type":"object","additionalProperties":false},
                "offers":[],"requires":[],"records":{"inputs":false,"outputs":false}}}),
            ),
            reading("read-b", Some(filter(&["SYN-2", "SYN-3"], end))),
            writing("write-b", Some(filter(&["SYN-3"], end))),
        ];
        if !defer_a_read {
            documents.push(reading(
                "read-a",
                Some(filter(&["SYN-1", "SYN-2"], h.a_deadline)),
            ));
            a_routes["read"] = json!({"from":"read-a","target":named("Resource",target)});
        }
        if bound {
            documents.push(conduit("bounded-open", &format!("{CORE}#access-control.v1"), &[&format!("{CORE}#open.v1")],
                Some(json!({"requirement":format!("{CORE}#bound.v1"),"arguments":{
                    "targets":[named("Resource",target)],
                    "operations":[format!("{RULES}#read-issue.v1"),format!("{RULES}#list-issues.v1")]
                }}))));
            a_routes["access"] = json!({"from":"bounded-open","target":named("Resource","access")});
        }
        for (agent, routes) in [("agent-a", a_routes), ("agent-b", b_routes)] {
            let template = format!("{agent}-template");
            documents.push(doc(
                "Agent",
                &template,
                json!({"spec":{"type":"#scripted.v1","conduits":routes}}),
            ));
            documents.push(doc(
                "Agent",
                agent,
                json!({"from":template,"owner":"s2-fleet"}),
            ));
        }
        documents.push(doc(
            "Fleet",
            "s2-fleet",
            json!({"from":format!("{CORE}#fleet")}),
        ));
        if bound {
            documents.extend([
                reading("unfiltered-read-template", None),
                writing("unfiltered-write-template", None),
                conduit_instance(
                    "unfiltered-read",
                    "unfiltered-read-template",
                    "agent-a",
                    target,
                ),
                conduit_instance(
                    "unfiltered-write",
                    "unfiltered-write-template",
                    "agent-a",
                    target,
                ),
            ]);
        }
        h.register("agents", documents).await;
        h.stop(true).await;
        {
            let shell = Shell::open(
                &h.store,
                boundary::context(&repo()).unwrap(),
                Config::for_store(&h.store).unwrap(),
            )
            .unwrap();
            for name in ["agent-a", "agent-b"] {
                let object = h.object("Agent", name);
                shell
                    .issue_client_token(object["id"].as_str().unwrap())
                    .unwrap();
            }
        }
        h.start().await;
        h
    }

    fn url(&self) -> String {
        format!("http://127.0.0.1:{}/mcp", self.port)
    }
    async fn cli(&self, verb: &str, args: &[&str]) -> Output {
        let output = timeout(
            WAIT,
            Command::new(env!("CARGO_BIN_EXE_conduit"))
                .env_clear()
                .env("CONDUIT_CONFIG_DIR", self.temp.path().join("config"))
                .arg(verb)
                .arg("--store")
                .arg(&self.store)
                .args(args)
                .stdin(Stdio::null())
                .kill_on_drop(true)
                .output(),
        )
        .await
        .unwrap_or_else(|_| {
            panic!(
                "conduit {verb} {} timed out",
                args.first().copied().unwrap_or("")
            )
        })
        .unwrap();
        self.audit.bytes(&output.stdout, "conduit stdout");
        self.audit.bytes(&output.stderr, "conduit stderr");
        output
    }
    async fn success(&self, verb: &str, args: &[&str]) -> Output {
        let output = self.cli(verb, args).await;
        assert!(
            output.status.success(),
            "conduit {verb} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }
    async fn root(&self, tool: &str, arguments: Value) -> Value {
        let output = self.cli("call", &[tool, &arguments.to_string()]).await;
        serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
            panic!(
                "root tool {tool} did not return a result: {}",
                String::from_utf8_lossy(&output.stderr)
            )
        })
    }
    async fn register(&self, project: &str, documents: Vec<Value>) {
        let packages = artifacts()
            .packages
            .iter()
            .map(|(address, _, digest)| {
                (
                    address.to_string(),
                    json!({"version":"0.1.0","digest":digest}),
                )
            })
            .collect::<serde_json::Map<_, _>>();
        let result = self.root("management.register.v1",json!({
            "manifest":{"apiVersion":API,"address":format!("example.org/tests/s2/{project}"),"packages":packages},
            "documents":documents
        })).await;
        assert!(
            conduit_shell::author::succeeded(&result),
            "registration {project} failed: {}",
            result["structuredContent"]["diagnostics"]
        );
    }
    async fn start(&mut self) {
        let index = self.serve_logs.len();
        let stdout = self.temp.path().join(format!("serve-{index}.out"));
        let stderr = self.temp.path().join(format!("serve-{index}.err"));
        let mut child = Command::new(env!("CARGO_BIN_EXE_conduit"))
            .env_clear()
            .env("S2_LINEAR_API_KEY", &self.decoy)
            .env("CONDUIT_CONFIG_DIR", self.temp.path().join("config"))
            .args(["serve", "--store"])
            .arg(&self.store)
            .stdout(fs::File::create(&stdout).unwrap())
            .stderr(fs::File::create(&stderr).unwrap())
            .stdin(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .unwrap();
        timeout(WAIT, async {
            while tokio::net::TcpStream::connect(("127.0.0.1", self.port))
                .await
                .is_err()
            {
                assert!(
                    child.try_wait().unwrap().is_none(),
                    "serve exited before readiness"
                );
                sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("serve did not listen");
        self.serve_logs.push((stdout, stderr));
        self.engine = Some(child);
    }
    async fn stop(&mut self, startup_succeeded: bool) {
        let mut child = self.engine.take().unwrap();
        let pid = child.id().unwrap() as i32;
        assert_eq!(unsafe { libc::kill(pid, libc::SIGTERM) }, 0);
        let status = timeout(WAIT, child.wait())
            .await
            .expect("serve did not stop")
            .unwrap();
        assert_eq!(
            status.success(),
            startup_succeeded,
            "graceful SIGTERM stop exits successfully under OPS-037: {status}"
        );
        for (stdout, stderr) in &self.serve_logs {
            self.audit.bytes(&fs::read(stdout).unwrap(), "serve stdout");
            self.audit.bytes(&fs::read(stderr).unwrap(), "serve stderr");
        }
    }
    fn state(&self) -> Value {
        let bytes = fs::read(self.store.join("state.json")).unwrap();
        self.audit.bytes(&bytes, "committed state");
        serde_json::from_slice::<Value>(&bytes).unwrap()["state"].clone()
    }
    fn find(&self, matches: impl Fn(&Value) -> bool) -> Value {
        self.state()["objects"]
            .as_array()
            .unwrap()
            .iter()
            .find(|o| o.get("spec").is_none() && matches(o))
            .cloned()
            .expect("instance not found")
    }
    fn object(&self, kind: &str, name: &str) -> Value {
        self.find(|o| o["kind"] == kind && o["name"] == name)
    }
    fn entry(&self, entry: &str) -> Value {
        self.find(|o| {
            o["kind"] == "Resource" && o["nested"]["name"] == entry && o["state"] != "retired"
        })
    }
    fn route(&self, owner: &str, local: &str) -> Value {
        let agent = self.object("Agent", owner);
        self.find(|o| {
            o["kind"] == "Conduit"
                && o["nested"]["parent"] == agent["id"]
                && o["local_name"] == local
        })
    }
    async fn agent(&self, name: &str) -> (Session, Arc<Mutex<usize>>) {
        let id = self.object("Agent", name)["id"]
            .as_str()
            .unwrap()
            .to_owned();
        let token = fs::read_to_string(self.store.join("client-tokens").join(id)).unwrap();
        let notices = Arc::new(Mutex::new(0));
        let (proxy, url) =
            AgentProxy::start(self.http.clone(), self.url(), self.audit.clone()).await;
        let service = timeout(
            WAIT,
            Consumer {
                notices: notices.clone(),
            }
            .serve(StreamableHttpClientTransport::with_client(
                self.http.clone(),
                StreamableHttpClientTransportConfig::with_uri(url).auth_header(&token),
            )),
        )
        .await
        .expect("Agent connection timed out")
        .expect("Agent initialization failed");
        self.audit
            .value(&service.peer_info(), "Agent initialization result");
        (
            Session {
                service,
                _proxy: proxy,
            },
            notices,
        )
    }
    async fn tools(&self, agent: &Session) -> Vec<String> {
        let list = timeout(WAIT, agent.list_tools(None))
            .await
            .expect("tools/list timed out")
            .unwrap();
        self.audit.value(&list, "Agent tool list");
        assert!(
            list.tools
                .iter()
                .all(|tool| !tool.name.contains("get-secret"))
        );
        list.tools
            .into_iter()
            .map(|t| t.name.into_owned())
            .collect()
    }
    async fn call(
        &self,
        agent: &Session,
        tool: &str,
        args: Value,
        key: Option<&str>,
    ) -> Result<Value, Value> {
        let mut request = CallToolRequestParams::new(tool.to_owned())
            .with_arguments(args.as_object().unwrap().clone());
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
        match timeout(WAIT, agent.call_tool(request))
            .await
            .expect("Agent call timed out")
        {
            Ok(result) => {
                self.audit.value(&result, "Agent MCP result");
                Ok(serde_json::to_value(result).unwrap())
            }
            Err(ServiceError::McpError(error)) => {
                self.audit.value(&error, "Agent MCP error");
                Err(serde_json::to_value(error).unwrap())
            }
            Err(_) => panic!("Agent call failed at the transport"),
        }
    }
    async fn read(&self, agent: &Session, issue: &str) -> Value {
        self.call(agent, READ, json!({"issue":issue}), None)
            .await
            .unwrap()
    }
    async fn control(&self, path: &str, body: Option<Value>) -> Value {
        let url = format!("{}{path}", self.control_url);
        let request = match body {
            Some(body) => self.http.post(url).json(&body),
            None => self.http.get(url),
        };
        let reply = request
            .header("Authorization", &self.control_token)
            .send()
            .await
            .expect("stub request failed");
        assert!(
            reply.status().is_success(),
            "stub control {path} returned {}",
            reply.status()
        );
        reply
            .json()
            .await
            .expect("stub control did not return JSON")
    }
    async fn count(&self) -> u64 {
        self.control("/count", None).await["api_requests"]
            .as_u64()
            .unwrap()
    }
    fn records(&self) -> Vec<Value> {
        self.state()["records"].as_array().unwrap().clone()
    }
    async fn finish(
        mut self,
        name: &str,
        reads: usize,
        successful: usize,
        stub_count: u64,
        startup_succeeded: bool,
    ) {
        let records = self.success("records", &[]).await;
        let exports = String::from_utf8(records.stdout).unwrap();
        let exported = exports
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect::<Vec<Value>>();
        assert_eq!(exported.len(), self.records().len());
        for (tool, object) in [
            (
                "s2-pool.inspect.v1",
                named("Resource", self.entry("linear")["name"].as_str().unwrap()),
            ),
            ("s2-fleet.inspect.v1", named("Agent", "agent-a")),
            ("s2-fleet.inspect.v1", named("Agent", "agent-b")),
        ] {
            let inspected = self.root(tool, json!({"object":object})).await;
            assert_eq!(meta(&inspected, "outcome"), "succeeded");
        }
        let records = self.records();
        let root = self.object("Interface", "root");
        let login = self.entry("login");
        let mut ends = Vec::new();
        let mut runs = 0;
        for record in &records {
            let data = &record["data"];
            assert!(
                !self.linear_ids.contains(&data["caller"]),
                "linear issued a request"
            );
            if record["type"] == "request-ended"
                && data["operation"] == format!("{CORE}#get-secret.v1")
            {
                assert_eq!(data["caller"], root["id"]);
                assert_eq!(data["target"], login["id"]);
                assert!(self.linear_ids.contains(&data["consumer"]));
                assert_eq!((data.get("inputs"), data.get("outputs")), (None, None));
                assert!(records.iter().any(|r| r["type"] == "invocation-dispatched"
                    && r["data"]["invocation"] == data["invocation"]));
                assert!(
                    !ends
                        .iter()
                        .any(|read: &&Value| read["serving"] == data["serving"]),
                    "linear setup read login more than once"
                );
                ends.push(data);
            }
            if record["type"] == "object-change"
                && data["action"] == "setup-part-reported"
                && data["partOperation"] == format!("{CORE}#run.v1")
                && self.linear_ids.contains(&data["target"])
            {
                assert_eq!(data["outcome"], "succeeded");
                assert!(
                    ends.iter().any(|read| read["serving"] == data["invocation"]
                        && read["consumer"] == data["target"]
                        && read["outcome"] == "succeeded"),
                    "linear ran before the matching successful secret read ended"
                );
                runs += 1;
            }
        }
        assert_eq!(ends.len(), reads);
        assert_eq!(
            ends.iter()
                .filter(|data| data["outcome"] == "succeeded")
                .count(),
            successful
        );
        assert_eq!(runs, successful);
        let final_count = self.count().await;
        assert_eq!(final_count, stub_count);
        self.stop(startup_succeeded).await;
        self.audit.scan(&self.store);
        assert!(
            !self.audit.inbound_leak.load(Ordering::SeqCst),
            "synthetic marker in inbound Agent message"
        );
        eprintln!(
            "{name}: {:.2}s, final stub count {final_count}",
            self.started.elapsed().as_secs_f64()
        );
    }
}

fn issue(value: &Value, id: &str) -> Value {
    assert_eq!(
        meta(value, "outcome"),
        "succeeded",
        "issue call result: {value}"
    );
    let issue = value["structuredContent"]["issue"].clone();
    assert_eq!(issue["identifier"], id);
    issue
}

#[tokio::test]
async fn s2_one_login_two_restrictions() {
    let h = S2::new(Scenario::default()).await;
    let (a, _) = h.agent("agent-a").await;
    let (b, _) = h.agent("agent-b").await;
    assert_eq!(
        issue(&h.read(&a, "SYN-2").await, "SYN-2"),
        issue(&h.read(&b, "SYN-2").await, "SYN-2")
    );
    let edit = json!({"issue":"SYN-3","title":"changed by B"});
    let updated = h.call(&b, UPDATE, edit.clone(), None).await.unwrap();
    assert_eq!(issue(&updated, "SYN-3")["title"], "changed by B");
    assert_eq!(
        issue(&h.read(&b, "SYN-3").await, "SYN-3")["title"],
        "changed by B"
    );
    assert!(
        !h.tools(&a)
            .await
            .iter()
            .any(|name| name.ends_with("update-issue.v1"))
    );
    assert!(h.tools(&b).await.iter().any(|name| name == UPDATE));
    let count = h.count().await;
    check_refused(&h.call(&a, UPDATE, edit, None).await.unwrap_err(), UPDATE);
    let refusal = h.read(&a, "SYN-3").await;
    assert_eq!(meta(&refusal, "reason"), "refused");
    assert_eq!(
        meta(&refusal, "restriction"),
        filter(&["SYN-1", "SYN-2"], h.a_deadline)
    );
    assert_eq!(h.count().await, count);
    drop((a, b));
    h.finish("s2_one_login_two_restrictions", 1, 1, 4, true)
        .await;
}

fn check_refused(error: &Value, tool: &str) {
    assert_eq!(error["code"], -32602);
    assert_eq!(error["data"][format!("{META}reason")], "refused");
    assert!(error["message"].as_str().unwrap().contains(tool));
    assert!(error.get("result").is_none());
}

fn no_issue(result: &Value) {
    assert!(result["structuredContent"].get("issue").is_none());
}

fn no_cached_issue(result: &Value, marker: &str) {
    no_issue(result);
    assert!(
        !result.to_string().contains(marker),
        "cached issue in repeat reply"
    );
}

#[test]
#[should_panic(expected = "cached issue in repeat reply")]
fn s2_repeat_assertion_detects_cached_text_content() {
    let marker = Uuid::new_v4().to_string();
    no_cached_issue(
        &json!({"content":[{"type":"text","text":format!("previous issue: {marker}")}]}),
        &marker,
    );
}

fn assert_attempt(records: &[Value], id: &Value, attempt: u64) {
    assert!(id.is_string());
    assert!(records.iter().any(|r| r["type"] == "invocation-dispatched"
        && r["data"]["invocation"] == *id
        && r["data"]["attempt"] == attempt));
}

#[tokio::test]
async fn s2_filter_current_read() {
    let [title_a, title_b, title_edit] =
        std::array::from_fn(|_| format!("s2-output-{}", Uuid::new_v4()));
    let h = S2::new(Scenario {
        defer_a_read: true,
        read_titles: Some([title_a.clone(), title_b.clone()]),
        ..Scenario::default()
    })
    .await;
    let (a, _) = h.agent("agent-a").await;
    let (b, _) = h.agent("agent-b").await;
    // Start the short deadline after engine recovery and Agent initialization.
    let linear = h.entry("linear");
    let deadline = Utc::now() + chrono::Duration::seconds(25);
    let restriction = filter(&["SYN-1", "SYN-2"], deadline);
    h.register(
        "timed-read",
        vec![
            reading("timed-read-template", Some(restriction.clone())),
            conduit_instance(
                "timed-read",
                "timed-read-template",
                "agent-a",
                linear["name"].as_str().unwrap(),
            ),
        ],
    )
    .await;
    open(&h, "timed-read").await;
    const TIMED_READ: &str = "timed-read.read-issue.v1";
    let read_a = json!({"issue":"SYN-1"});
    let read_b = json!({"issue":"SYN-2"});
    let edit = json!({"issue":"SYN-3","title":title_edit});
    let first_a = h
        .call(&a, TIMED_READ, read_a.clone(), Some("read-a"))
        .await
        .unwrap();
    assert!(issue(&first_a, "SYN-1")["title"] == title_a);
    let first_b = h
        .call(&b, READ, read_b.clone(), Some("read-b"))
        .await
        .unwrap();
    assert!(issue(&first_b, "SYN-2")["title"] == title_b);
    let first_edit = h
        .call(&b, UPDATE, edit.clone(), Some("edit-b"))
        .await
        .unwrap();
    assert!(issue(&first_edit, "SYN-3")["title"] == title_edit);
    h.audit.repeat_markers.lock().unwrap().extend([
        title_a.clone(),
        title_b.clone(),
        title_edit.clone(),
    ]);

    timeout(Duration::from_secs(30), async {
        while Utc::now() < deadline {
            sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("filter deadline did not pass");
    let before = h.count().await;
    let later_a = h
        .call(&a, TIMED_READ, read_a, Some("read-a"))
        .await
        .unwrap();
    assert_eq!(meta(&later_a, "reason"), "refused");
    assert_eq!(meta(&later_a, "restriction"), restriction);
    no_cached_issue(&later_a, &title_a);
    assert_eq!(h.count().await, before);

    h.control(
        "/service-authorization",
        Some(json!({"key":h.old,"issues":["SYN-1","SYN-3","SYN-4"]})),
    )
    .await;
    let later_b = h.call(&b, READ, read_b, Some("read-b")).await.unwrap();
    assert_eq!(meta(&later_b, "outcome"), "unknown");
    assert_eq!(later_b["isError"], true);
    no_cached_issue(&later_b, &title_b);
    assert_eq!(h.count().await, before + 1);
    let later_edit = h.call(&b, UPDATE, edit, Some("edit-b")).await.unwrap();
    assert_eq!(meta(&later_edit, "reason"), "conflict");
    no_cached_issue(&later_edit, &title_edit);
    assert_eq!(h.count().await, before + 1);

    let records = h.records();
    for (first, later) in [(&first_a, &later_a), (&first_b, &later_b)] {
        let invocation = meta(first, "invocation");
        if meta(later, "reason").is_null() {
            assert_eq!(meta(later, "invocation"), invocation);
        }
        assert_attempt(&records, &invocation, 1);
        assert_attempt(&records, &invocation, 2);
    }
    let invocations = h.state()["invocations"].as_array().unwrap().clone();
    for (reply, expected) in [
        (&first_a, "succeeded"),
        (&first_b, "unknown"),
        (&first_edit, "succeeded"),
    ] {
        let invocation = meta(reply, "invocation");
        assert!(
            invocations
                .iter()
                .any(|v| v["id"] == invocation && v["state"] == expected),
            "invocation did not keep the specified outcome {expected}"
        );
    }
    drop((a, b));
    h.finish("s2_filter_current_read", 1, 1, 4, true).await;
}

async fn open(h: &S2, name: &str) {
    let result = h.root("access.open.v1", json!({"conduit":name})).await;
    assert_eq!(meta(&result, "outcome"), "succeeded");
}
async fn close(h: &S2, conduit: &Value) {
    let result = h
        .root("access.close.v1", json!({"conduit":conduit["name"]}))
        .await;
    assert_eq!(meta(&result, "outcome"), "succeeded");
}

#[tokio::test]
async fn s2_bound_allows_unfiltered() {
    let h = S2::new(Scenario {
        bound: true,
        ..Scenario::default()
    })
    .await;
    let (a, _) = h.agent("agent-a").await;
    for name in ["unfiltered-read", "unfiltered-write"] {
        assert_eq!(h.object("Conduit", name)["state"], "registered");
    }
    let allowed = h
        .call(
            &a,
            "access.open.v1",
            json!({"conduit":"unfiltered-read"}),
            None,
        )
        .await
        .unwrap();
    assert_eq!(meta(&allowed, "outcome"), "succeeded");
    let read = h
        .call(
            &a,
            "unfiltered-read.read-issue.v1",
            json!({"issue":"SYN-4"}),
            None,
        )
        .await
        .unwrap();
    issue(&read, "SYN-4");
    let refused = h
        .call(
            &a,
            "access.open.v1",
            json!({"conduit":"unfiltered-write"}),
            None,
        )
        .await
        .unwrap();
    assert_eq!(meta(&refused, "reason"), "refused");
    assert_eq!(
        meta(&refused, "restriction")["requirement"],
        format!("{CORE}#bound.v1")
    );
    assert_eq!(
        h.object("Conduit", "unfiltered-write")["state"],
        "registered"
    );
    assert_ne!(h.object("Conduit", "unfiltered-write")["open"], true);
    for (route, filtered) in [
        (h.route("agent-a", "read"), true),
        (h.object("Conduit", "unfiltered-read"), false),
    ] {
        let inspected = h
            .root(
                "management.inspect.v1",
                json!({"object":named("Conduit",route["name"].as_str().unwrap())}),
            )
            .await;
        assert_eq!(meta(&inspected, "outcome"), "succeeded");
        assert_eq!(inspected["structuredContent"]["id"], route["id"]);
        assert_eq!(inspected["structuredContent"]["status"]["state"], "ready");
        assert_eq!(route["open"], true);
        assert_eq!(
            inspected["structuredContent"]["declared"]["target"]["id"],
            h.entry("linear")["id"]
        );
        assert_eq!(
            inspected["structuredContent"]["declared"]["requirements"]
                .as_object()
                .is_some_and(|r| !r.is_empty()),
            filtered
        );
    }
    drop(a);
    h.finish("s2_bound_allows_unfiltered", 1, 1, 1, true).await;
}

async fn setup_linear(h: &S2, resource: &Value) -> Value {
    h.root(
        "s2-pool.set-up.v1",
        json!({"instance":named("Resource",resource["name"].as_str().unwrap())}),
    )
    .await
}

async fn restore_to_replacement(h: &S2, agent: &str, target: &str, issues: &[&str]) -> Vec<Value> {
    let name = format!("restore-{agent}");
    let template = format!("{name}-read");
    let deadline = Utc::now() + chrono::Duration::hours(1);
    let mut docs = vec![
        reading(&template, Some(filter(issues, deadline))),
        conduit_instance(&name, &template, agent, target),
    ];
    let mut names = vec![name.clone()];
    if agent == "agent-b" {
        let write = format!("{name}-write");
        let template = format!("{write}-template");
        docs.push(writing(&template, Some(filter(&["SYN-3"], deadline))));
        docs.push(conduit_instance(&write, &template, agent, target));
        names.push(write);
    }
    h.register(&name, docs).await;
    let mut restored = Vec::new();
    for name in names {
        open(h, &name).await;
        restored.push(h.object("Conduit", &name));
    }
    restored
}

fn assert_object_change(records: &[Value], object: &Value, action: &str) {
    assert!(
        records.iter().any(|r| r["type"] == "object-change"
            && r["data"]["target"] == object["id"]
            && r["data"]["action"] == action),
        "missing object-change record for {action}"
    );
}

#[tokio::test]
async fn s2_rotate_replace_repair() {
    let mut h = S2::new(Scenario::default()).await;
    let (a, _) = h.agent("agent-a").await;
    let (b, _) = h.agent("agent-b").await;
    issue(&h.read(&a, "SYN-1").await, "SYN-1");
    issue(&h.read(&b, "SYN-3").await, "SYN-3");
    let old = h.entry("linear");
    let old_routes = [
        h.route("agent-a", "read"),
        h.route("agent-b", "read"),
        h.route("agent-b", "write"),
    ];
    let new = format!("s2-stub-{}", Uuid::new_v4());
    h.audit.add(new.clone());
    h.control("/rotate-key", Some(json!({"key":new}))).await;
    fs::write(&h.login, new.as_bytes()).unwrap();
    for (agent, id) in [(&a, "SYN-1"), (&b, "SYN-3")] {
        let result = h.read(agent, id).await;
        assert_eq!(meta(&result, "outcome"), "failed");
        no_issue(&result);
    }
    let retired = h
        .root(
            "s2-pool.retire.v1",
            json!({"instance":named("Resource",old["name"].as_str().unwrap())}),
        )
        .await;
    assert_eq!(meta(&retired, "outcome"), "succeeded");
    for route in &old_routes {
        assert!(
            !h.state()["objects"]
                .as_array()
                .unwrap()
                .iter()
                .any(|o| o["id"] == route["id"])
        );
    }
    let replacement = h
        .root(
            "management.register-replacement.v1",
            json!({"instance":named("Pool","s2-pool"),"nested":"linear"}),
        )
        .await;
    assert_eq!(meta(&replacement, "outcome"), "succeeded");
    let replacement = &replacement["structuredContent"]["object"];
    assert_ne!(replacement["id"], old["id"]);
    h.linear_ids.push(replacement["id"].clone());
    let setup = setup_linear(&h, replacement).await;
    assert_eq!(meta(&setup, "outcome"), "succeeded");

    let target = replacement["name"].as_str().unwrap();
    let mut restored = restore_to_replacement(&h, "agent-a", target, &["SYN-1", "SYN-2"]).await;
    drop((a, b));
    h.stop(true).await;
    h.start().await;
    let (a, _) = h.agent("agent-a").await;
    let (b, _) = h.agent("agent-b").await;
    let inspected = h
        .root(
            "s2-fleet.inspect.v1",
            json!({"object":named("Agent","agent-b")}),
        )
        .await;
    assert_eq!(meta(&inspected, "outcome"), "succeeded");
    assert!(
        !inspected["structuredContent"]["inventory"]["consumes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|route| {
                route["target"]["id"] == replacement["id"] || route["target"]["id"] == old["id"]
            })
    );
    assert!(
        !h.tools(&b)
            .await
            .iter()
            .any(|tool| tool.ends_with("read-issue.v1"))
    );
    assert_eq!(h.count().await, 4);
    check_refused(
        &h.call(&b, READ, json!({"issue":"SYN-3"}), None)
            .await
            .unwrap_err(),
        READ,
    );
    assert_eq!(h.count().await, 4);
    let a_tool = "restore-agent-a.read-issue.v1";
    let b_tool = "restore-agent-b.read-issue.v1";
    issue(
        &h.call(&a, a_tool, json!({"issue":"SYN-1"}), None)
            .await
            .unwrap(),
        "SYN-1",
    );
    restored.extend(restore_to_replacement(&h, "agent-b", target, &["SYN-2", "SYN-3"]).await);
    issue(
        &h.call(&a, a_tool, json!({"issue":"SYN-2"}), None)
            .await
            .unwrap(),
        "SYN-2",
    );
    issue(
        &h.call(&b, b_tool, json!({"issue":"SYN-3"}), None)
            .await
            .unwrap(),
        "SYN-3",
    );

    let records = h.records();
    for route in &old_routes {
        assert_object_change(&records, route, "closed");
    }
    assert_object_change(&records, replacement, "registered");
    for route in &restored {
        assert_object_change(&records, route, "opened");
    }
    drop((a, b));
    h.finish("s2_rotate_replace_repair", 3, 3, 7, true).await;
}

async fn assert_unavailable(h: &S2, linear: &Value, a: &Session) {
    let result = h
        .root(
            "s2-pool.inspect.v1",
            json!({"object":named("Resource",linear["name"].as_str().unwrap())}),
        )
        .await;
    let setup = &result["structuredContent"]["status"]["setup"];
    assert!(
        setup["parts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["name"] == "run" && p["outcome"] == "unavailable")
    );
    assert!(
        !h.store
            .join("instances")
            .join(linear["id"].as_str().unwrap())
            .exists(),
        "runner made a process state directory for linear"
    );
    let refused = h
        .call(a, READ, json!({"issue":"SYN-1"}), None)
        .await
        .unwrap();
    assert_eq!(meta(&refused, "reason"), "unavailable");
    no_issue(&refused);
    assert_eq!(h.count().await, 0);
}

#[tokio::test]
async fn s2_secret_read_fails_closed() {
    let h = S2::new(Scenario {
        bad_login: true,
        ..Scenario::default()
    })
    .await;
    let (a, _) = h.agent("agent-a").await;
    let linear = h.entry("linear");
    assert_unavailable(&h, &linear, &a).await;
    fs::remove_file(&h.login).unwrap();
    let missing = setup_linear(&h, &linear).await;
    assert_ne!(meta(&missing, "outcome"), "succeeded");
    assert_unavailable(&h, &linear, &a).await;
    private_file(&h.login, &h.old);
    assert_eq!(
        meta(&setup_linear(&h, &linear).await, "outcome"),
        "succeeded"
    );
    assert!(
        h.store
            .join("instances")
            .join(linear["id"].as_str().unwrap())
            .is_dir()
    );
    issue(&h.read(&a, "SYN-1").await, "SYN-1");
    drop(a);
    // Startup attempts the Pool entry through the Pool and then attempts the
    // still-registered Resource itself. Both attempts see the unsafe mode.
    h.finish("s2_secret_read_fails_closed", 4, 1, 1, true).await;
}

#[tokio::test]
async fn s2_dependent_call_after_close() {
    let h = S2::new(Scenario::default()).await;
    let (a, _) = h.agent("agent-a").await;
    let route = h.route("agent-a", "read");
    assert!(h.tools(&a).await.iter().any(|name| name == READ));
    let barrier = h
        .control(
            "/barrier/arm",
            Some(json!({
                "match":{"field":"issue","identifier":"SYN-1"},
                "timeout_ms":WAIT.as_millis()
            })),
        )
        .await;
    let (answer, ()) = tokio::join!(h.read(&a, "SYN-1"), async {
        let control = json!({"id":barrier["id"],"timeout_ms":(WAIT / 2).as_millis()});
        let reached = h.control("/barrier/wait", Some(control.clone())).await;
        assert_eq!(reached["state"], "reached");
        timeout(AWARE_REQUEST_DEADLINE, async {
            close(&h, &route).await;
            let released = h.control("/barrier/release", Some(control)).await;
            assert_eq!(released["state"], "released");
        })
        .await
        .expect("did not close and release within the aware request deadline");
    });
    issue(&answer, "SYN-1");
    let invocation = meta(&answer, "invocation");
    assert!(h.records().iter().any(|r| r["type"] == "invocation-ended"
        && r["data"]["invocation"] == invocation
        && r["data"]["outcome"] == "succeeded"));
    check_refused(
        &h.call(&a, READ, json!({"issue":"SYN-1"}), None)
            .await
            .unwrap_err(),
        READ,
    );
    assert_eq!(h.count().await, 1);
    drop(a);
    h.finish("s2_dependent_call_after_close", 1, 1, 1, true)
        .await;
}

#[tokio::test]
async fn s2_cached_tool_refused() {
    let h = S2::new(Scenario::default()).await;
    let (a, notices) = h.agent("agent-a").await;
    let (b, _) = h.agent("agent-b").await;
    issue(&h.read(&b, "SYN-3").await, "SYN-3");
    assert!(h.tools(&a).await.iter().any(|name| name == READ));
    let route = h.route("agent-a", "read");
    close(&h, &route).await;
    until(|| *notices.lock().unwrap() >= 1).await;
    assert!(!h.tools(&a).await.iter().any(|name| name == READ));
    let a_id = h.object("Agent", "agent-a")["id"].clone();
    let never = "absent.read-issue.v1";
    for tool in [READ, never] {
        let refused = h
            .call(&a, tool, json!({"issue":"SYN-1"}), None)
            .await
            .unwrap_err();
        check_refused(&refused, tool);
        assert!(h.records().iter().any(|r| r["type"] == "request-ended"
            && r["data"]["caller"] == a_id
            && r["data"]["reason"] == "refused"
            && r["data"]["tool"] == tool));
    }
    assert_eq!(h.count().await, 1);
    issue(&h.read(&b, "SYN-3").await, "SYN-3");
    h.control(
        "/service-authorization",
        Some(json!({"key":h.old,"issues":["SYN-1","SYN-2","SYN-4"]})),
    )
    .await;
    let failed = h.read(&b, "SYN-3").await;
    assert_eq!(meta(&failed, "outcome"), "failed");
    assert!(meta(&failed, "reason").is_null());
    no_issue(&failed);
    issue(&h.read(&b, "SYN-2").await, "SYN-2");
    let closed = h.state();
    let stone = closed["tombstones"]
        .as_object()
        .unwrap()
        .values()
        .find(|stone| stone["object"]["id"] == route["id"])
        .expect("closed Conduit has no tombstone");
    let lifted = h.root("access.lift.v1", json!({"id":stone["id"]})).await;
    assert_eq!(meta(&lifted, "outcome"), "succeeded");
    let restored = h
        .root(
            "management.register-replacement.v1",
            json!({"instance":named("Agent","agent-a"),"nested":"read"}),
        )
        .await;
    assert_eq!(meta(&restored, "outcome"), "succeeded");
    let setup = h
        .root(
            "s2-fleet.set-up.v1",
            json!({"instance":named("Agent","agent-a")}),
        )
        .await;
    assert_eq!(meta(&setup, "outcome"), "succeeded");
    until(|| *notices.lock().unwrap() >= 2).await;
    issue(&h.read(&a, "SYN-1").await, "SYN-1");
    issue(&h.read(&b, "SYN-2").await, "SYN-2");
    drop((a, b));
    h.finish("s2_cached_tool_refused", 1, 1, 6, true).await;
}
