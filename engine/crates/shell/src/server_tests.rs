use super::*;
use crate::server::{self, ServingExecutor};
use rmcp::{
    ClientHandler, RoleClient, ServiceExt,
    model::*,
    service::{NotificationContext, RunningService},
    transport::{
        StreamableHttpClientTransport, streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use std::{collections::VecDeque, sync::atomic::AtomicBool};
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn sign_in_wait_end_is_submitted_at_expiry() {
    let temp = temporary();
    let shell = Arc::new(established(temp.path()));
    *shell.trace.lock().unwrap() = Some(Vec::new());
    let executor = Arc::new(ServingExecutor::new(Arc::new(crate::MissingExecutor)));
    let is_expiry = |decision: &conduit_core::Decision| decision.action == "sign-in-wait-ended";
    let count = || {
        shell
            .trace
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .iter()
            .filter(|t| t.decisions.iter().any(is_expiry))
            .count()
    };
    let shutdown = CancellationToken::new();
    let started = Instant::now();
    let task = server::end_sign_in_wait_at(
        shell.clone(),
        executor.clone(),
        shutdown,
        "absent-invocation".into(),
        1,
        Utc::now() + chrono::TimeDelta::milliseconds(300),
    );
    assert_eq!(count(), 0);
    tokio::time::timeout(Duration::from_secs(10), task)
        .await
        .unwrap()
        .unwrap();
    assert!(started.elapsed() >= Duration::from_millis(250));
    assert_eq!(count(), 1);
    {
        let trace = shell.trace.lock().unwrap();
        let expired = trace
            .as_ref()
            .unwrap()
            .iter()
            .find(|t| t.decisions.iter().any(is_expiry))
            .unwrap();
        let decision = expired.decisions.iter().find(|d| is_expiry(d)).unwrap();
        assert_eq!(
            decision.facts,
            json!({"invocation":"absent-invocation","attempt":1,"ended":false})
        );
        assert_eq!(expired.records, Vec::<Value>::new());
        assert_eq!(expired.external_intents, Vec::<Value>::new());
    }

    let shutdown = CancellationToken::new();
    let task = server::end_sign_in_wait_at(
        shell.clone(),
        executor,
        shutdown.clone(),
        "absent-invocation".into(),
        1,
        Utc::now() + chrono::TimeDelta::seconds(10),
    );
    shutdown.cancel();
    tokio::time::timeout(Duration::from_secs(5), task)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(count(), 1);
}

#[test]
fn sign_in_members_reach_the_core() {
    let prefix = context().meta_prefix;
    let capabilities = json!({"elicitation":{"url":{}},"extensions":{}});
    let login = json!({"name":"browser"});
    let meta = json!({
        "io.modelcontextprotocol/clientCapabilities":capabilities,
        format!("{prefix}signInLogin"):login,
        format!("{prefix}signInReturn"):{"value":"x","via":"paste"}
    });
    assert_eq!(
        Value::Object(server::sign_in_members(&meta, &prefix)),
        json!({"clientCapabilities":capabilities,"signInLogin":login})
    );
    for meta in [
        json!({"io.modelcontextprotocol/clientCapabilities":null,format!("{prefix}signInLogin"):null}),
        json!({}),
    ] {
        assert_eq!(
            server::sign_in_members(&meta, &prefix),
            serde_json::Map::new()
        );
    }
}

#[derive(Clone)]
struct Consumer {
    changes: Arc<AtomicUsize>,
}
impl ClientHandler for Consumer {
    fn get_info(&self) -> ClientConfig {
        let mut config = ClientConfig::default();
        config.protocol_version = ProtocolVersion::V_2025_06_18;
        config
    }
    async fn on_tool_list_changed(&self, _: NotificationContext<RoleClient>) {
        self.changes.fetch_add(1, Ordering::SeqCst);
    }
}
async fn connect(
    url: &str,
    token: &str,
    changes: Arc<AtomicUsize>,
) -> RunningService<RoleClient, Consumer> {
    install_crypto_provider();
    Consumer { changes }
        .serve(StreamableHttpClientTransport::with_client(
            reqwest::Client::new(),
            StreamableHttpClientTransportConfig::with_uri(url.to_owned()).auth_header(token),
        ))
        .await
        .unwrap()
}
async fn modern(url: &str, token: &str) -> RunningService<RoleClient, ()> {
    install_crypto_provider();
    use rmcp::{ClientServiceExt, service::ClientLifecycleMode};
    ().serve_with_lifecycle(
        StreamableHttpClientTransport::with_client(
            reqwest::Client::new(),
            StreamableHttpClientTransportConfig::with_uri(url.to_owned()).auth_header(token),
        ),
        ClientLifecycleMode::Discover {
            preferred_versions: vec![ProtocolVersion::V_2026_07_28],
        },
    )
    .await
    .unwrap()
}

async fn url_client(
    url: &str,
    token: &str,
    prefix: &str,
) -> RunningService<RoleClient, ClientConfig> {
    use rmcp::{ClientServiceExt, service::ClientLifecycleMode};
    let mut config = ClientConfig::default();
    config.capabilities.elicitation =
        Some(ElicitationCapability::new().with_url(UrlElicitationCapability::new()));
    config.capabilities.extensions = Some(BTreeMap::from([(
        format!("{prefix}implementation-interface"),
        json!({"revision":"v0"}).as_object().unwrap().clone(),
    )]));
    config
        .serve_with_lifecycle(
            StreamableHttpClientTransport::with_client(
                reqwest::Client::new(),
                StreamableHttpClientTransportConfig::with_uri(url.to_owned()).auth_header(token),
            ),
            ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            },
        )
        .await
        .unwrap()
}

struct Memory {
    content: Mutex<String>,
    shell: Weak<Shell>,
    dispatches: AtomicUsize,
    deferred: AtomicBool,
    cancellations: Mutex<Vec<Value>>,
}
impl Executor for Memory {
    fn cancel(&self, intent: &Value) -> Result<Execution> {
        self.cancellations.lock().unwrap().push(intent.clone());
        Ok(Execution::Done)
    }
    fn dispatch(&self, intent: &Value) -> Result<Execution> {
        let shell = self.shell.upgrade().unwrap();
        let stored = shell.writer.lock().unwrap().store.read()?;
        assert!(
            stored
                .state
                .records
                .iter()
                .any(|r| r["type"] == "invocation-dispatched"
                    && r["data"]["invocation"] == intent["invocation"])
        );
        assert!(intent["call"]["params"].get("_meta").is_none());
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        if self.deferred.load(Ordering::SeqCst) {
            return Ok(Execution::Deferred);
        }
        let args = &intent["call"]["params"]["arguments"];
        let text = if intent["call"]["params"]["name"] == "write_file" {
            *self.content.lock().unwrap() = args["content"].as_str().unwrap().into();
            "written".into()
        } else {
            self.content.lock().unwrap().clone()
        };
        Ok(Execution::Complete(
            json!({"kind":"result","result":{"content":[{"type":"text","text":text}]}}),
        ))
    }
}
fn fixed_shell(path: &Path) -> Shell {
    drop(established(path));
    fixed_endpoint_shell(path)
}

fn established_case(path: &Path, case: &Path, through: usize) -> Shell {
    use conduit_core::packages::{Project, Snapshot, SnapshotSource};

    #[derive(serde::Deserialize)]
    struct Recipe {
        files: BTreeMap<String, String>,
        directory: Option<String>,
    }
    #[derive(serde::Deserialize)]
    struct SnapshotRecipe {
        source: SnapshotSource,
        address: String,
        version: String,
        digest: String,
        archive: Option<String>,
        project: Recipe,
    }
    #[derive(serde::Deserialize)]
    struct Artifact {
        address: String,
        implementation: String,
        sha256: String,
    }
    #[derive(serde::Deserialize)]
    struct Fixture {
        snapshots: Vec<SnapshotRecipe>,
        project: Recipe,
        #[serde(default)]
        artifacts: Vec<Artifact>,
    }

    let shell = open(path);
    let vector = boundary::value(&case.join("case.yaml")).unwrap();
    let files = vector["files"].as_object().unwrap();
    let fixture_path = |alias: &str| case.join(files[alias].as_str().unwrap());
    let spec = conduit_loader::Specification::read(&repository().join("spec")).unwrap();
    let load = |recipe: Recipe| {
        let bytes: Vec<_> = recipe
            .files
            .into_iter()
            .map(|(name, alias)| (name, fs::read(fixture_path(&alias)).unwrap()))
            .collect();
        let project = conduit_loader::read_submitted_files(&bytes, &spec).unwrap();
        Project {
            manifest: project.manifest.map(|file| file.root.to_json()),
            documents: project
                .documents
                .iter()
                .map(|file| file.root.to_json())
                .collect(),
            directory: recipe.directory,
        }
    };

    *shell.trace.lock().unwrap() = Some(Vec::new());
    let mut symbols = BTreeMap::new();
    for step in &vector["steps"].as_array().unwrap()[..through] {
        let mut input = expand(&step["input"], &symbols, &shell.context.core_package);
        let fields = input.as_object_mut().unwrap();
        fields.remove("evidence");
        if let Some(evidence) = step["input"].get("evidence") {
            shell.evidence.lock().unwrap().push_back(evidence.clone());
        }
        match fields["kind"].as_str().unwrap() {
            "first-start" => {
                fields.remove("trustDomain");
            }
            "register-project" | "resolve-project" => {
                let file = fields.remove("file").unwrap();
                let reverse = fields.remove("reverse").is_some_and(|v| v == true);
                let fixture: Fixture = serde_json::from_value(
                    boundary::value(&fixture_path(file.as_str().unwrap())).unwrap(),
                )
                .unwrap();
                let snapshots: Vec<_> = fixture
                    .snapshots
                    .into_iter()
                    .map(|recipe| {
                        assert!(matches!(
                            recipe.source,
                            SnapshotSource::PreinstalledSnapshot
                        ));
                        if let Some(archive) = recipe.archive {
                            let bytes = fs::read(fixture_path(&archive)).unwrap();
                            assert_eq!(conduit_core::packages::bytes_digest(&bytes), recipe.digest);
                        }
                        Snapshot {
                            source: recipe.source,
                            address: recipe.address,
                            version: recipe.version,
                            digest: recipe.digest,
                            project: load(recipe.project),
                        }
                    })
                    .collect();
                for artifact in fixture.artifacts {
                    let snapshot = snapshots
                        .iter()
                        .find(|s| s.address == artifact.address)
                        .unwrap();
                    let item = &snapshot.project.manifest.as_ref().unwrap()["implementations"]
                        [&artifact.implementation];
                    assert_eq!(item["package"]["fileSha256"], artifact.sha256);
                }
                let mut project = load(fixture.project);
                if reverse {
                    project.documents.reverse();
                }
                input["snapshots"] = json!(snapshots);
                input["builtin"] = json!(shell.context.builtin);
                input["project"] = json!(project);
            }
            _ => {}
        }
        shell.commit(input).unwrap();
        if let Some(captures) = step["capture"].as_object() {
            let trace = shell.trace.lock().unwrap();
            let value = capture_value(trace.as_ref().unwrap().last().unwrap());
            for (name, pointer) in captures {
                symbols.insert(
                    name.clone(),
                    value.pointer(pointer.as_str().unwrap()).unwrap().clone(),
                );
            }
        }
    }
    shell.evidence.lock().unwrap().clear();
    *shell.trace.lock().unwrap() = None;
    shell
}

fn fixed_case_shell(path: &Path, case: &Path, through: usize) -> Shell {
    drop(established_case(path, case, through));
    fixed_endpoint_shell(path)
}

fn fixed_endpoint_shell(path: &Path) -> Shell {
    fixed_endpoint_shell_with_config(path, Config::default())
}

fn fixed_endpoint_shell_with_config(path: &Path, config: Config) -> Shell {
    install_crypto_provider();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    // The fixture creates no run tokens. Give this independent test store a fixed
    // endpoint before opening the server; production cannot rewrite its endpoint.
    let config = Config {
        engine_url: format!("http://127.0.0.1:{port}/mcp"),
        ..config
    };
    store::atomic_write(
        path,
        "engine-url.json",
        &serde_json::to_vec(&config.engine_url).unwrap(),
    )
    .unwrap();
    Shell::open(path, context(), config).unwrap()
}
async fn ready(url: &str) {
    install_crypto_provider();
    // Binding is a progress condition; the test makes no startup-speed promise.
    tokio::time::timeout(Duration::from_secs(600), async {
        loop {
            if reqwest::get(url).await.is_ok() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("server did not bind");
}
fn token(shell: &Shell, name: &str) -> String {
    let id = actor(shell, name);
    let descriptor = descriptor(shell, &id);
    let identity = shell.materialize_identity(&descriptor).unwrap();
    String::from_utf8(
        store::private_read(
            &identity
                .directory
                .join(descriptor["fileNames"][3].as_str().unwrap()),
        )
        .unwrap(),
    )
    .unwrap()
}
fn call(name: &str, arguments: Value) -> CallToolRequestParams {
    serde_json::from_value(json!({"name":name,"arguments":arguments})).unwrap()
}
fn durable_ending(shell: &Shell, reply: &CallToolResult) {
    let value = serde_json::to_value(reply).unwrap();
    let id = &value["_meta"][format!("{}invocation", shell.context.meta_prefix)];
    let state = shell.writer.lock().unwrap().store.read().unwrap().state;
    assert!(
        state
            .records
            .iter()
            .any(|r| r["type"] == "request-ended" && r["data"]["invocation"] == *id)
    );
    assert!(
        state
            .records
            .iter()
            .any(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == *id)
    );
}
async fn wait_changes(counter: &AtomicUsize, minimum: usize) {
    tokio::time::timeout(Duration::from_secs(600), async {
        while counter.load(Ordering::SeqCst) < minimum {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn root_reaches_list_and_lift_unknown_id_over_mcp() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    server::preflight(&shell).unwrap();
    let root = actor(&shell, "root");
    shell.issue_client_token(&root).unwrap();
    let executor = Arc::new(ServingExecutor::new(Arc::new(crate::MissingExecutor)));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
    ready(&shell.config.engine_url).await;

    // OPS-036 and OPS-019: both listings reach the core and leave durable endings.
    for arguments in [json!({}), json!({"kind":"Interface"})] {
        let reply = server::client(
            temp.path(),
            None,
            Some("management.list.v1"),
            arguments.clone(),
        )
        .await
        .unwrap();
        assert_eq!(reply["isError"], false);
        let outputs = reply["structuredContent"].as_object().unwrap();
        for key in ["objects", "packages", "tombstones"] {
            assert!(outputs[key].is_array(), "{key} must be an array");
        }
        assert!(
            outputs
                .keys()
                .all(|key| ["objects", "packages", "tombstones", "pending"].contains(&key.as_str()))
        );
        let objects = outputs["objects"].as_array().unwrap();
        assert!(objects.iter().any(|item| item["id"] == root
            && item["kind"] == "Interface"
            && item["name"] == "root"
            && item["template"] == false
            && item.get("state").is_some()));
        if arguments.get("kind").is_some() {
            assert!(objects.iter().all(|item| item["kind"] == "Interface"));
        }
        assert_eq!(
            reply["_meta"][format!("{}outcome", shell.context.meta_prefix)],
            "succeeded"
        );
        assert!(reply["_meta"][format!("{}invocation", shell.context.meta_prefix)].is_string());
        let reply: CallToolResult = serde_json::from_value(reply).unwrap();
        durable_ending(&shell, &reply);
    }

    // ACC-053 / OPS-029: an unbounded unknown ID is invalid, with one durable ending.
    let before = shell.state().unwrap().records.len();
    let reply = server::client(
        temp.path(),
        None,
        Some("access.lift.v1"),
        json!({"id":"00000000-0000-4000-8000-000000009999"}),
    )
    .await
    .unwrap();
    assert_eq!(reply["isError"], true);
    assert_eq!(
        reply["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "invalid"
    );
    assert_eq!(
        reply["structuredContent"]["diagnostics"][0]["ruleId"],
        "OPS-029"
    );
    let state = shell.state().unwrap();
    let endings: Vec<_> = state.records[before..]
        .iter()
        .filter(|record| record["type"] == "request-ended")
        .collect();
    assert_eq!(endings.len(), 1);
    assert_eq!(endings[0]["data"]["reason"], "invalid");

    stop.cancel();
    tokio::time::timeout(Duration::from_secs(60), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn s1_http_consumers_authority_records_and_private_t7_changes() {
    let temp = temporary();
    let mut shell = fixed_shell(temp.path());
    // Legacy consumers stay idle while other sessions exercise the protocol.
    // Keep them for this long scenario rather than testing idle expiry here.
    shell.config.call_timeout = Duration::from_secs(4 * 3600);
    let shell = Arc::new(shell);
    server::preflight(&shell).unwrap();
    let a = token(&shell, "agent-a");
    let b = token(&shell, "agent-b");
    shell.issue_client_token(&actor(&shell, "root")).unwrap();
    let memory = Arc::new(Memory {
        content: Mutex::new("hello".into()),
        shell: Arc::downgrade(&shell),
        dispatches: AtomicUsize::new(0),
        deferred: AtomicBool::new(false),
        cancellations: Mutex::default(),
    });
    let executor = Arc::new(ServingExecutor::new(memory.clone()));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor.clone(), stop.clone()));
    let url = &shell.config.engine_url;
    ready(url).await;
    let changes_a = Arc::new(AtomicUsize::new(0));
    let changes_b = Arc::new(AtomicUsize::new(0));
    let client_a = connect(url, &a, changes_a.clone()).await;
    let client_b = connect(url, &b, changes_b.clone()).await;
    // Legacy sessions expire after five idle minutes. Work on other identities
    // must not turn a slow run into a test of idle expiry instead of notification
    // delivery. Protocol pings keep these fixtures live without invoking tools.
    let keepalive = CancellationToken::new();
    let keepalive_tasks = [&client_a, &client_b].map(|client| {
        let peer = client.peer().clone();
        let stop = keepalive.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = stop.cancelled() => break,
                    _ = tokio::time::sleep(Duration::from_secs(30)) => {}
                }
                tokio::select! {
                    _ = stop.cancelled() => break,
                    _ = peer.send_request(ClientRequest::PingRequest(PingRequest {
                        method: Default::default(),
                        extensions: Default::default(),
                    })) => {}
                }
            }
        })
    });
    for (name, client) in [("agent-a", &client_a), ("agent-b", &client_b)] {
        let list = client.list_tools(None).await.unwrap();
        let expected = shell
            .submit(
                json!({"kind":"list-tools","identity":instance_value(&actor(&shell,name))}),
                executor.as_ref(),
            )
            .unwrap();
        assert_eq!(
            serde_json::to_value(&list).unwrap()["tools"],
            expected["tools"]
        );
        assert_eq!(list.cache_scope, Some(CacheScope::Private));
        assert!(list.ttl_ms.is_none());
    }
    let list_a = client_a.list_tools(None).await.unwrap();
    let list_b = client_b.list_tools(None).await.unwrap();
    assert_eq!(list_a.tools.len(), 5);
    assert_eq!(list_b.tools.len(), 6);
    let refused = client_a
        .call_tool(call(
            "project.file.write.v1",
            json!({"path":"hello.txt","content":"forbidden"}),
        ))
        .await
        .unwrap_err();
    assert!(refused.to_string().contains("refused"), "{refused}");
    assert_eq!(memory.dispatches.load(Ordering::SeqCst), 0);
    assert!(
        shell
            .writer
            .lock()
            .unwrap()
            .store
            .read()
            .unwrap()
            .state
            .records
            .iter()
            .any(|r| r["type"] == "request-ended" && r["data"]["reason"] == "refused")
    );
    let written = client_b
        .call_tool(call(
            "project.file.write.v1",
            json!({"path":"hello.txt","content":"B wrote"}),
        ))
        .await
        .unwrap();
    assert_eq!(written.is_error, Some(false));
    durable_ending(&shell, &written);
    let read = client_b
        .call_tool(call("project.file.read.v1", json!({"path":"hello.txt"})))
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(&read).unwrap()["content"][0]["text"],
        "B wrote"
    );
    durable_ending(&shell, &read);
    let http = reqwest::Client::new();
    for token in [None, Some("forged")] {
        let mut request=http.post(url).json(&json!({"jsonrpc":"2.0","id":90,"method":"tools/call","params":{"name":"project.file.write.v1","arguments":{"path":"x","content":"bad"}}}));
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        let response = request.send().await.unwrap();
        assert_eq!(response.status(), 401);
        assert!(response.headers().contains_key("www-authenticate"));
        assert_eq!(memory.dispatches.load(Ordering::SeqCst), 2);
        assert_eq!(
            shell
                .writer
                .lock()
                .unwrap()
                .store
                .read()
                .unwrap()
                .state
                .records
                .last()
                .unwrap()["data"]["reason"],
            "refused"
        );
    }
    let response = http
        .post(url)
        .bearer_auth(&a)
        .header("origin", "http://attacker.invalid")
        .json(&json!({"jsonrpc":"2.0","id":91,"method":"tools/list"}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 403);
    assert_eq!(
        shell
            .writer
            .lock()
            .unwrap()
            .store
            .read()
            .unwrap()
            .state
            .records
            .last()
            .unwrap()["data"]["reason"],
        "invalid"
    );
    let foreign = shell
        .authority
        .token(&actor(&shell, "agent-a"), "http://127.0.0.1:1/mcp")
        .unwrap();
    let removed = shell
        .authority
        .token("aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa", url)
        .unwrap();
    for token in [&foreign, &removed] {
        let response=http.post(url).bearer_auth(token).json(&json!({"jsonrpc":"2.0","id":92,"method":"tools/call","params":{"name":"project.file.read.v1","arguments":{}}})).send().await.unwrap();
        assert_eq!(response.status(), 401);
        assert!(response.headers().contains_key("www-authenticate"));
    }
    let initialized=http.post(url).bearer_auth(&a).header("accept","application/json, text/event-stream").json(&json!({"jsonrpc":"2.0","id":900,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"malformed-test","version":"1"}}})).send().await.unwrap();
    let session = initialized
        .headers()
        .get("mcp-session-id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let _ = initialized.text().await.unwrap();
    let stolen = http
        .post(url)
        .bearer_auth(&b)
        .header("mcp-session-id", &session)
        .header("accept", "application/json, text/event-stream")
        .json(&json!({"jsonrpc":"2.0","id":901,"method":"tools/list"}))
        .send()
        .await
        .unwrap();
    assert_eq!(stolen.status(), 401);
    let records_before = shell.state().unwrap().records.len();
    let dispatches_before = memory.dispatches.load(Ordering::SeqCst);
    let stolen_call = http.post(url).bearer_auth(&b)
        .header("mcp-session-id", &session)
        .json(&json!({"jsonrpc":"2.0","id":902,"method":"tools/call","params":{"name":"project.file.write.v1","arguments":{"path":"hello.txt","content":"stolen session"}}}))
        .send().await.unwrap();
    assert_eq!(stolen_call.status(), 401);
    assert!(stolen_call.headers().contains_key("www-authenticate"));
    let state = shell.writer.lock().unwrap().store.read().unwrap().state;
    let endings: Vec<_> = state.records[records_before..]
        .iter()
        .filter(|r| r["type"] == "request-ended")
        .collect();
    assert_eq!(endings.len(), 1);
    assert_eq!(endings[0]["data"]["consumer"], actor(&shell, "agent-b"));
    assert_eq!(endings[0]["data"]["caller"], actor(&shell, "agent-b"));
    assert_eq!(endings[0]["data"]["reason"], "invalid");
    assert_eq!(memory.dispatches.load(Ordering::SeqCst), dispatches_before);
    let response = http
        .post(url)
        .bearer_auth(&a)
        .header("accept", "application/json, text/event-stream")
        .header("mcp-protocol-version", "2025-06-18")
        .header("mcp-session-id", &session)
        .json(&json!({"jsonrpc":"2.0","id":93,"method":"tools/call","params":{"arguments":{}}}))
        .send()
        .await
        .unwrap();
    let error = response.text().await.unwrap();
    assert!(
        error.contains(&format!("{}reason", shell.context.meta_prefix)),
        "{error}"
    );
    assert!(error.contains("invalid"));
    assert_eq!(
        shell
            .writer
            .lock()
            .unwrap()
            .store
            .read()
            .unwrap()
            .state
            .records
            .last()
            .unwrap()["data"]["reason"],
        "invalid"
    );
    let prefix = &shell.context.meta_prefix;
    let mut keyed = call(
        "project.file.write.v1",
        json!({"path":"hello.txt","content":"keyed"}),
    );
    keyed.meta=Some(serde_json::from_value(json!({format!("{prefix}idempotencyKey"):"http-key","io.modelcontextprotocol/clientCapabilities":{"extensions":{format!("{prefix}implementation-interface"):{"revision":"v0"}}}})).unwrap());
    let dispatches = memory.dispatches.load(Ordering::SeqCst);
    let first = client_b.call_tool(keyed.clone()).await.unwrap();
    let repeated = client_b.call_tool(keyed).await.unwrap();
    assert_eq!(first, repeated);
    assert_eq!(memory.dispatches.load(Ordering::SeqCst), dispatches + 1);
    durable_ending(&shell, &repeated);
    let modern_a = modern(url, &a).await;
    let modern_b = modern(url, &b).await;
    let filter = SubscriptionFilterBuilder::default()
        .tools_list_changed()
        .build();
    let mut subscription_a = modern_a.listen(filter.clone()).await.unwrap();
    let mut subscription_b = modern_b.listen(filter).await.unwrap();
    // A normal request round trip also lets listen's post-ack handler start.
    let modern_list = modern_a.list_tools(None).await.unwrap();
    assert_eq!(modern_list.tools, list_a.tools);
    assert_eq!(modern_list.cache_scope, Some(CacheScope::Private));
    assert_eq!(modern_list.ttl_ms, Some(0)); // rmcp's required modern protocol default
    // DL2: copy the selected value from the shared last-commit snapshot.
    let conduit = shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| {
            o["kind"] == "Conduit"
                && reference_value(&o["consumer"]) == &json!(actor(&shell, "agent-a"))
                && o["nested"]["name"] == "project"
        })
        .cloned()
        .unwrap();
    let closed = server::client(
        temp.path(),
        None,
        Some("access.close.v1"),
        json!({"conduit":conduit["name"]}),
    )
    .await
    .unwrap();
    assert_eq!(closed["isError"], false);
    wait_changes(&changes_a, 1).await;
    assert!(matches!(
        tokio::time::timeout(Duration::from_secs(600), subscription_a.next())
            .await
            .unwrap()
            .unwrap(),
        Some(ServerNotification::ToolListChangedNotification(_))
    ));
    assert!(
        tokio::time::timeout(Duration::from_millis(100), subscription_b.next())
            .await
            .is_err()
    );
    assert_eq!(client_a.list_tools(None).await.unwrap().tools.len(), 3);
    assert_eq!(client_b.list_tools(None).await.unwrap().tools, list_b.tools);
    assert_eq!(changes_b.load(Ordering::SeqCst), 0);
    let refused = client_a
        .call_tool(call("project.file.read.v1", json!({"path":"hello.txt"})))
        .await
        .unwrap_err();
    assert!(refused.to_string().contains("refused"));
    let result = client_b
        .call_tool(call(
            "project.file.write.v1",
            json!({"path":"hello.txt","content":"B still works"}),
        ))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(false));
    durable_ending(&shell, &result);
    // ACC-053 / LIF-041: the root client explicitly lifts before restoring the slot.
    let stone = shell
        .state()
        .unwrap()
        .tombstones
        .values()
        .find(|stone| stone["object"]["id"] == conduit["id"])
        .unwrap()["id"]
        .clone();
    let lifted = server::client(
        temp.path(),
        None,
        Some("access.lift.v1"),
        json!({"id":stone}),
    )
    .await
    .unwrap();
    assert_eq!(lifted["isError"], false);
    let restored = server::client(
        temp.path(),
        None,
        Some("management.register-replacement.v1"),
        json!({"instance":{"kind":"Agent","name":"agent-a"},"nested":"project"}),
    )
    .await
    .unwrap();
    assert_eq!(restored["isError"], false);
    let setup = server::client(
        temp.path(),
        None,
        Some("s1.set-up.v1"),
        json!({"instance":{"kind":"Agent","name":"agent-a"}}),
    )
    .await
    .unwrap();
    assert_eq!(setup["isError"], false);
    wait_changes(&changes_a, 2).await;
    assert!(
        tokio::time::timeout(Duration::from_secs(600), subscription_a.next())
            .await
            .unwrap()
            .unwrap()
            .is_some()
    );
    assert!(
        tokio::time::timeout(Duration::from_millis(100), subscription_b.next())
            .await
            .is_err()
    );
    assert_eq!(client_a.list_tools(None).await.unwrap().tools, list_a.tools);
    assert_eq!(changes_b.load(Ordering::SeqCst), 0);
    let management_limits = server::unfinished(&shell, &actor(&shell, "root")).unwrap();
    assert!(!management_limits.is_empty());
    let before = shell.state().unwrap();
    let unfinished = management_limits.keys().next().unwrap();
    let result = server::client(temp.path(), None, Some(unfinished), json!({}))
        .await
        .unwrap_err();
    assert!(result.contains("unfinished"), "{result}");
    assert_eq!(
        // DL2: serialize the shared committed snapshot by reference.
        serde_json::to_value(shell.state().unwrap().as_ref()).unwrap(),
        serde_json::to_value(before).unwrap()
    );
    keepalive.cancel();
    for task in keepalive_tasks {
        task.await.unwrap();
    }
    drop(subscription_a);
    drop(subscription_b);
    modern_a.cancel().await.unwrap();
    modern_b.cancel().await.unwrap();
    client_a.cancel().await.unwrap();
    client_b.cancel().await.unwrap();
    stop.cancel();
    tokio::time::timeout(Duration::from_secs(600), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}
fn instance_value(id: &str) -> Value {
    json!({"kind":"instance","id":id})
}
fn reference_value(value: &Value) -> &Value {
    value.get("id").unwrap_or(value)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn pending_http_completion_and_client_cancellation() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    let b = token(&shell, "agent-b");
    let memory = Arc::new(Memory {
        content: Mutex::new(String::new()),
        shell: Arc::downgrade(&shell),
        dispatches: AtomicUsize::new(0),
        deferred: AtomicBool::new(true),
        cancellations: Mutex::default(),
    });
    let executor = Arc::new(ServingExecutor::new(memory.clone()));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor.clone(), stop.clone()));
    ready(&shell.config.engine_url).await;
    let client = connect(&shell.config.engine_url, &b, Arc::default()).await;
    let peer = client.peer().clone();
    let request = tokio::spawn(async move {
        peer.call_tool(call("project.file.read.v1", json!({"path":"hello.txt"})))
            .await
    });
    wait_changes(&memory.dispatches, 1).await;
    assert!(!request.is_finished());
    // DL2: copy the selected value from the shared last-commit snapshot.
    let invocation = shell
        .state()
        .unwrap()
        .invocations
        .iter()
        .find(|i| i["state"] == "in-progress")
        .cloned()
        .unwrap();
    shell
        .complete(
            invocation["id"].as_str().unwrap(),
            1,
            json!({"kind":"result","result":{"content":[{"type":"text","text":"later"}]}}),
            executor.as_ref(),
        )
        .unwrap();
    let result = request.await.unwrap().unwrap();
    durable_ending(&shell, &result);
    let handle = client
        .send_cancellable_request(
            ClientRequest::CallToolRequest(CallToolRequest::new(call(
                "project.file.write.v1",
                json!({"path":"hello.txt","content":"cancel"}),
            ))),
            rmcp::service::PeerRequestOptions::no_options(),
        )
        .await
        .unwrap();
    wait_changes(&memory.dispatches, 2).await;
    handle
        .cancel(Some("test cancellation".into()))
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(600), async {
        loop {
            let state = shell.state().unwrap();
            if state
                .invocations
                .iter()
                .all(|i| i["state"] != "in-progress")
            {
                assert!(state.invocations.iter().any(|i| i["state"] == "unknown"));
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    client.cancel().await.unwrap();
    stop.cancel();
    tokio::time::timeout(Duration::from_secs(600), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

const SIGN_IN_URL: &str = concat!(
    "https://example.org/authorize?client_id=placeholder-signed-agent&response_type=code",
    "&redirect_uri=http%3A%2F%2F127.0.0.1%3A8400%2Fcallback&code_challenge_method=S256",
    "&code_challenge=E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM&state=join-state-0123456789abcdef",
);
const HOOK_STATE: &str = "hook-state-1";
const HOOK_MESSAGE: &str = "Open the page to sign in.";
const CLIENT_CAPABILITIES: &str = "io.modelcontextprotocol/clientCapabilities";

struct SignInScript {
    plain: bool,
    prefix: String,
    dispatches: Mutex<Vec<Value>>,
    cancellations: Mutex<Vec<Value>>,
    account: Mutex<Value>,
}

impl SignInScript {
    fn new(shell: &Shell, plain: bool) -> Self {
        Self {
            plain,
            prefix: shell.context.meta_prefix.clone(),
            dispatches: Mutex::default(),
            cancellations: Mutex::default(),
            account: Mutex::new(Value::Null),
        }
    }

    async fn serve(
        self: &Arc<Self>,
        shell: &Arc<Shell>,
        stop: &CancellationToken,
    ) -> tokio::task::JoinHandle<Result<()>> {
        let executor = Arc::new(ServingExecutor::new(self.clone()));
        let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
        ready(&shell.config.engine_url).await;
        task
    }

    fn expect_dispatches(&self, names: &[&str]) -> Vec<Value> {
        let dispatches = self.dispatches.lock().unwrap().clone();
        let actual: Vec<_> = dispatches
            .iter()
            .map(|intent| intent["call"]["params"]["name"].as_str().unwrap())
            .collect();
        assert_eq!(actual, names);
        dispatches
    }

    fn report_cancellations(&self, shell: &Shell) {
        let state = shell.state().unwrap();
        let cancellations = self.cancellations.lock().unwrap();
        eprintln!(
            "Sign-in script received {} cancel intents.",
            cancellations.len()
        );
        for intent in cancellations.iter() {
            let operation = state
                .invocations
                .iter()
                .find(|invocation| invocation["id"] == intent["invocation"])
                .and_then(|invocation| invocation["operation"].as_str());
            eprintln!("Cancel operation: {}", operation.unwrap_or("unresolved"));
        }
    }
}

impl Executor for SignInScript {
    fn dispatch(&self, intent: &Value) -> Result<Execution> {
        if let Some(reply) = scripted_discovery(intent) {
            return Ok(reply);
        }
        self.dispatches.lock().unwrap().push(intent.clone());
        let (account_hook, sign_in_hook) = if self.plain {
            ("account_hook", "sign_in_hook")
        } else {
            ("account-hook.v1", "sign-in-hook.v1")
        };
        let name = intent["call"]["params"]["name"].as_str().unwrap();
        let mut result = if name == sign_in_hook {
            json!({
                "resultType": "input_required",
                "inputRequests": {"login": {"method": "elicitation/create", "params": {
                    "mode": "url", "message": HOOK_MESSAGE, "url": SIGN_IN_URL
                }}},
                "requestState": HOOK_STATE
            })
        } else {
            let mut result = json!({"content": []});
            if name == account_hook {
                result["structuredContent"] = json!({"account": *self.account.lock().unwrap()});
            }
            result
        };
        if name != sign_in_hook && !self.plain {
            result["_meta"] = json!({format!("{}outcome", self.prefix): "succeeded"});
        }
        Ok(Execution::Complete(
            json!({"kind": "result", "result": result}),
        ))
    }

    fn cancel(&self, intent: &Value) -> Result<Execution> {
        self.cancellations.lock().unwrap().push(intent.clone());
        Ok(Execution::Done)
    }
}

fn sign_in_root_token(shell: &Shell, path: &Path) -> String {
    let root = actor(shell, "root");
    shell.issue_client_token(&root).unwrap();
    String::from_utf8(store::private_read(&path.join("client-tokens").join(root)).unwrap()).unwrap()
}

fn sign_in_request() -> CallToolRequestParams {
    call(
        "group.sign-in.v1",
        json!({"instance": {"kind": "Agent", "name": "signed"}}),
    )
}

fn sign_in_retry(prompt: &Value, action: &str) -> CallToolRequestParams {
    sign_in_request()
        .with_request_state(prompt["requestState"].as_str().unwrap())
        .with_input_responses(BTreeMap::from([("url".into(), json!({"action": action}))]))
}

fn sign_in_reply(
    response: std::result::Result<CallToolResponse, rmcp::ServiceError>,
    step: &str,
) -> CallToolResponse {
    match response {
        Ok(reply) => reply,
        Err(rmcp::ServiceError::McpError(error)) => {
            let rule = error
                .data
                .as_ref()
                .and_then(|data| data["diagnostics"][0]["ruleId"].as_str());
            let rule =
                rule.filter(|rule| matches!(*rule, "IFC-036" | "IFC-049" | "PND-010" | "PND-013"));
            panic!("{step}: MCP error {}, rule {rule:?}", error.code.0);
        }
        Err(_) => panic!("{step}: MCP transport error"),
    }
}

fn sign_in_prompt(
    response: std::result::Result<CallToolResponse, rmcp::ServiceError>,
    step: &str,
) -> Value {
    let reply = match sign_in_reply(response, step) {
        CallToolResponse::InputRequired(reply) => reply,
        CallToolResponse::Complete(reply) => {
            panic!(
                "{step}: expected input-required reply, received complete (is_error {:?})",
                reply.is_error
            )
        }
        _ => panic!("{step}: expected input-required reply"),
    };
    let reply = serde_json::to_value(reply).unwrap();
    assert!(
        reply["inputRequests"]
            .as_object()
            .is_some_and(|requests| requests.len() == 1),
        "{step}: expected one input request"
    );
    let request = &reply["inputRequests"]["url"];
    assert!(
        request["method"] == "elicitation/create",
        "{step}: expected elicitation/create under url"
    );
    assert!(
        request["params"]["mode"] == "url",
        "{step}: expected URL mode"
    );
    assert!(
        request["params"]["url"] == SIGN_IN_URL,
        "{step}: relayed URL differs"
    );
    assert!(
        reply["requestState"]
            .as_str()
            .is_some_and(|state| state.chars().count() == 43),
        "{step}: expected a 43-character requestState"
    );
    reply
}

fn assert_keyed_sign_in_prompt(reply: &Value, login_key: &str) {
    let message = reply["inputRequests"]["url"]["params"]["message"]
        .as_str()
        .unwrap();
    assert!(message.contains("example.org") && message.contains("primary-account"));
    assert!(!message.contains(HOOK_STATE) && !message.contains(HOOK_MESSAGE));
    let login = &reply["_meta"][login_key];
    assert!(login["name"] == "browser");
    assert!(login["return"] == "redirect");
    assert!(login.get("node").is_some());
    assert!(login["acceptsReturn"] == true);
    assert!(login.get("returned").is_none());
}

fn assert_sign_in_store_excludes(root: &Path, markers: &[&str]) {
    scan_sign_in_store(&root.join("authority.json"), root, markers);
}

fn scan_sign_in_store(authority: &Path, directory: &Path, markers: &[&str]) {
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let kind = entry.file_type().unwrap();
        if kind.is_dir() {
            scan_sign_in_store(authority, &entry.path(), markers);
        } else if entry.path() != authority {
            // The root signing authority file contains private keys and is not an output of
            // sign-in; any other file of that name, at any depth, is scanned.
            assert!(kind.is_file(), "unexpected file type in the test store");
            let bytes = fs::read(entry.path()).unwrap();
            for marker in markers {
                assert!(
                    !bytes
                        .windows(marker.len())
                        .any(|window| window == marker.as_bytes()),
                    "a persisted file contained a value that should not be kept"
                );
            }
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sign_in_relays_through_the_interface_and_polls_to_signed_in() {
    let temp = temporary();
    let case = repository().join("conformance/conventions/requests/sign-in-hold");
    let shell = Arc::new(fixed_case_shell(temp.path(), &case, 11));
    let signed = actor(&shell, "signed");
    assert!(shell.state().unwrap().sign_in_holds.contains_key(&signed));

    let token = sign_in_root_token(&shell, temp.path());
    let script = Arc::new(SignInScript::new(&shell, false));
    let stop = CancellationToken::new();
    let task = script.serve(&shell, &stop).await;
    let url = &shell.config.engine_url;
    let prefix = &shell.context.meta_prefix;
    let client = url_client(url, &token, prefix).await;

    let login_key = format!("{prefix}signInLogin");
    let mut request = sign_in_request();
    request.meta = Some(serde_json::from_value(json!({&login_key: {"name": "browser"}})).unwrap());
    let first = sign_in_prompt(
        client.call_tool_once(request).await,
        "step 2, first request",
    );
    assert_keyed_sign_in_prompt(&first, &login_key);

    let mut names = vec!["account-hook.v1", "sign-in-hook.v1"];
    let dispatches = script.expect_dispatches(&names);
    // IFC-006 and IFC-045 govern the metadata of the hook's call.
    let hook_meta = &dispatches[1]["call"]["params"]["_meta"];
    assert!(hook_meta[&login_key] == json!({"name": "browser"}));
    assert!(hook_meta[CLIENT_CAPABILITIES]["elicitation"] == json!({"url": {}}));

    let second = sign_in_prompt(
        client.call_tool_once(sign_in_retry(&first, "accept")).await,
        "step 4, first poll",
    );
    assert!(first["requestState"] != second["requestState"]);
    names.push("account-hook.v1");
    script.expect_dispatches(&names);
    assert!(shell.state().unwrap().sign_in_holds.contains_key(&signed));

    *script.account.lock().unwrap() = json!("primary-account");
    *shell.trace.lock().unwrap() = Some(Vec::new());
    client.cancel().await.unwrap();
    let close_transitions = shell.trace.lock().unwrap().take().unwrap().len();
    eprintln!("Closing the first client committed {close_transitions} core inputs.");
    let client = url_client(url, &token, prefix).await;
    let retry = sign_in_retry(&second, "accept");

    let step = "step 5, poll from second connection";
    let CallToolResponse::Complete(result) =
        sign_in_reply(client.call_tool_once(retry.clone()).await, step)
    else {
        panic!("{step}: expected a complete reply");
    };
    assert_ne!(result.is_error, Some(true), "{step}");
    let final_reply = serde_json::to_value(result).unwrap();
    assert!(
        final_reply["structuredContent"]["account"] == "primary-account",
        "{step}"
    );
    assert!(
        final_reply["_meta"][format!("{prefix}outcome")] == "succeeded",
        "{step}"
    );
    let login = &final_reply["_meta"][&login_key];
    assert!(
        login["name"] == "browser" && login.get("returned").is_none(),
        "{step}"
    );

    names.push("account-hook.v1");
    for dispatch in script.expect_dispatches(&names) {
        let params = &dispatch["call"]["params"];
        assert!(params.get("requestState").is_none());
        assert!(params.get("inputResponses").is_none());
    }
    // PND-005 lifts the hold after the expected account is verified under PND-011.
    assert!(!shell.state().unwrap().sign_in_holds.contains_key(&signed));

    let Err(rmcp::ServiceError::McpError(error)) = client.call_tool_once(retry).await else {
        panic!("step 8: expected an MCP error for the consumed requestState");
    };
    assert_eq!(error.code, ErrorCode::INVALID_PARAMS, "step 8");
    assert!(
        error.message.starts_with("requestState was already used"),
        "step 8"
    );
    script.expect_dispatches(&names);

    let error = serde_json::to_value(error).unwrap();
    for reply in [&first, &second, &final_reply, &error] {
        assert!(!reply.to_string().contains(HOOK_STATE));
    }
    let state = shell.state().unwrap();
    for record in &state.records {
        let record = record.to_string();
        assert!(!record.contains(HOOK_STATE));
        assert!(!record.contains(SIGN_IN_URL));
    }
    assert!(!final_reply.to_string().contains(SIGN_IN_URL));

    assert_sign_in_store_excludes(temp.path(), &[HOOK_STATE]);
    client.cancel().await.unwrap();
    stop.cancel();
    task.await.unwrap().unwrap();
    script.report_cancellations(&shell);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn relayed_sign_in_starts_the_expiry_timer() {
    let temp = temporary();
    let case = repository().join("conformance/conventions/requests/sign-in-hold");
    let shell = Arc::new(fixed_case_shell(temp.path(), &case, 11));
    let token = sign_in_root_token(&shell, temp.path());
    let script = Arc::new(SignInScript::new(&shell, false));
    let mut executor = ServingExecutor::new(script.clone());
    executor.sign_in_wait_limit = Some(chrono::TimeDelta::milliseconds(300));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(
        shell.clone(),
        Arc::new(executor),
        stop.clone(),
    ));
    let url = &shell.config.engine_url;
    ready(url).await;
    let prefix = &shell.context.meta_prefix;
    let client = url_client(url, &token, prefix).await;
    *shell.trace.lock().unwrap() = Some(Vec::new());

    let login_key = format!("{prefix}signInLogin");
    let mut request = sign_in_request();
    request.meta = Some(serde_json::from_value(json!({&login_key: {"name": "browser"}})).unwrap());
    let prompt = sign_in_prompt(client.call_tool_once(request).await, "first request");
    assert_keyed_sign_in_prompt(&prompt, &login_key);

    let invocation = tokio::time::timeout(Duration::from_secs(4), async {
        loop {
            let ended = {
                let trace = shell.trace.lock().unwrap();
                trace
                    .as_ref()
                    .unwrap()
                    .iter()
                    .flat_map(|t| &t.decisions)
                    .find(|d| d.action == "sign-in-wait-ended" && d.facts["ended"] == true)
                    .map(|d| d.facts["invocation"].clone())
            };
            if let Some(invocation) = ended {
                break invocation;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the relay did not end sign-in at the timer's expiry");
    let state = shell.state().unwrap();
    let ended = state
        .invocations
        .iter()
        .find(|v| v["id"] == invocation)
        .unwrap();
    assert_eq!(
        ended["operation"],
        format!("{}#sign-in.v1", shell.context.core_package)
    );
    assert_eq!(ended["state"], "unknown");
    script.expect_dispatches(&["account-hook.v1", "sign-in-hook.v1"]);
    client.cancel().await.unwrap();
    stop.cancel();
    task.await.unwrap().unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sign_in_decline_through_the_interface_ends_failed() {
    const RETURN: &str =
        "http://127.0.0.1:8400/callback?code=join-code-0001&state=join-state-0123456789abcdef";
    let temp = temporary();
    let case = repository().join("conformance/conventions/requests/sign-in-hold");
    let shell = Arc::new(fixed_case_shell(temp.path(), &case, 11));
    let signed = actor(&shell, "signed");
    assert!(shell.state().unwrap().sign_in_holds.contains_key(&signed));

    let token = sign_in_root_token(&shell, temp.path());
    let script = Arc::new(SignInScript::new(&shell, false));
    let stop = CancellationToken::new();
    let task = script.serve(&shell, &stop).await;
    let prefix = &shell.context.meta_prefix;
    let client = url_client(&shell.config.engine_url, &token, prefix).await;

    let first = sign_in_prompt(
        client.call_tool_once(sign_in_request()).await,
        "step 2, first request",
    );
    let login_key = format!("{prefix}signInLogin");
    assert_keyed_sign_in_prompt(&first, &login_key);
    let names = ["account-hook.v1", "sign-in-hook.v1"];
    let dispatches = script.expect_dispatches(&names);
    let hook_meta = &dispatches[1]["call"]["params"]["_meta"];
    assert!(hook_meta.get(&login_key).is_none());
    assert!(hook_meta[CLIENT_CAPABILITIES]["elicitation"] == json!({"url": {}}));

    let mut retry = sign_in_retry(&first, "decline");
    retry.meta = Some(
        serde_json::from_value(json!({
            format!("{prefix}signInReturn"): {"value": RETURN, "via": "paste"}
        }))
        .unwrap(),
    );
    let step = "declined retry";
    let CallToolResponse::Complete(result) =
        sign_in_reply(client.call_tool_once(retry).await, step)
    else {
        panic!("{step}: expected a complete reply");
    };
    assert_eq!(result.is_error, Some(true), "{step}");
    let reply = serde_json::to_value(result).unwrap();
    assert!(
        reply["_meta"][format!("{prefix}outcome")] == "failed",
        "{step}"
    );
    assert!(
        reply["structuredContent"]["diagnostics"]
            .as_array()
            .is_some_and(|diagnostics| diagnostics.iter().any(|d| d["ruleId"] == "PND-010")),
        "{step}: expected a PND-010 diagnostic"
    );
    script.expect_dispatches(&names);
    let state = shell.state().unwrap();
    assert!(state.sign_in_holds.contains_key(&signed));

    // Under PND-013 a decline passes on no return, and PND-014 forbids keeping it.
    let markers = [RETURN, "join-code-0001"];
    for value in [&first, &reply].into_iter().chain(&state.records) {
        let serialized = value.to_string();
        assert!(markers.iter().all(|marker| !serialized.contains(*marker)));
    }
    assert_sign_in_store_excludes(temp.path(), &markers);
    client.cancel().await.unwrap();
    stop.cancel();
    task.await.unwrap().unwrap();
    script.report_cancellations(&shell);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sign_in_plain_hook_through_the_interface_signs_in() {
    let temp = temporary();
    let case = repository().join("conformance/conventions/requests/sign-in-plain-hook-part");
    let shell = Arc::new(fixed_case_shell(temp.path(), &case, 2));
    let token = sign_in_root_token(&shell, temp.path());
    let script = Arc::new(SignInScript::new(&shell, true));
    let stop = CancellationToken::new();
    let task = script.serve(&shell, &stop).await;
    let prefix = &shell.context.meta_prefix;
    let client = url_client(&shell.config.engine_url, &token, prefix).await;

    let first = sign_in_prompt(
        client.call_tool_once(sign_in_request()).await,
        "step 1, first request",
    );
    let login = &first["_meta"][format!("{prefix}signInLogin")];
    assert!(login["name"] == "browser");
    assert!(login["acceptsReturn"] == false);

    // IFC-007 strips Conduit's keys while IFC-045 still declares URL elicitation.
    let dispatches = script.expect_dispatches(&["account_hook", "sign_in_hook"]);
    for dispatch in &dispatches {
        let meta = dispatch["call"]["params"]["_meta"].as_object().unwrap();
        assert!(meta.keys().all(|key| !key.starts_with(prefix)));
    }
    let hook_meta = &dispatches[1]["call"]["params"]["_meta"];
    assert!(hook_meta[CLIENT_CAPABILITIES]["elicitation"] == json!({"url": {}}));

    *script.account.lock().unwrap() = json!("primary-account");
    let step = "step 3, poll after browser login";
    let CallToolResponse::Complete(result) = sign_in_reply(
        client.call_tool_once(sign_in_retry(&first, "accept")).await,
        step,
    ) else {
        panic!("{step}: expected a complete reply");
    };
    assert_ne!(result.is_error, Some(true), "{step}");
    let reply = serde_json::to_value(result).unwrap();
    assert!(
        reply["structuredContent"]["account"] == "primary-account",
        "{step}"
    );
    assert!(
        reply["_meta"][format!("{prefix}outcome")] == "succeeded",
        "{step}"
    );
    script.expect_dispatches(&["account_hook", "sign_in_hook", "account_hook"]);

    for reply in [&first, &reply] {
        assert!(!reply.to_string().contains(HOOK_STATE));
    }
    for record in &shell.state().unwrap().records {
        assert!(!record.to_string().contains(HOOK_STATE));
    }
    assert_sign_in_store_excludes(temp.path(), &[HOOK_STATE]);
    client.cancel().await.unwrap();
    stop.cancel();
    task.await.unwrap().unwrap();
    script.report_cancellations(&shell);
}

const CONSENT_PROMPT: &str = "Open this URL in your browser? Type y to open it, or n to decline: ";
const ENTER_PROMPT: &str = "Press Enter once you have signed in, or type n to stop: ";
const RETURN_NOTICE: &str = "conduit sign-in: the engine reports that this login takes a return";

#[derive(Default)]
struct Scripted {
    answers: VecDeque<Option<String>>,
    prompts: Vec<String>,
    told: Vec<String>,
    opened: Vec<String>,
    events: Vec<String>,
}

impl Scripted {
    fn new(answers: &[Option<&str>]) -> Self {
        Self {
            answers: answers
                .iter()
                .map(|answer| answer.map(str::to_owned))
                .collect(),
            ..Self::default()
        }
    }
}

impl crate::sign_in::Person for Scripted {
    fn ask(&mut self, prompt: &str) -> Result<Option<String>> {
        self.prompts.push(prompt.into());
        self.events.push(prompt.into());
        Ok(self.answers.pop_front().flatten())
    }

    fn tell(&mut self, line: &str) {
        self.told.push(line.into());
    }

    fn open(&mut self, url: &str) -> Result<()> {
        self.opened.push(url.into());
        self.events.push(format!("open:{url}"));
        Ok(())
    }
}

struct PollScript {
    plain: bool,
    prefix: String,
    accounts: Mutex<VecDeque<Value>>,
    dispatches: Mutex<Vec<Value>>,
    cancellations: Mutex<Vec<Value>>,
}

impl PollScript {
    async fn serve(
        shell: &Arc<Shell>,
        stop: &CancellationToken,
        plain: bool,
        accounts: impl IntoIterator<Item = Value>,
    ) -> (Arc<Self>, tokio::task::JoinHandle<Result<()>>) {
        shell.issue_client_token(&actor(shell, "root")).unwrap();
        let script = Arc::new(Self {
            plain,
            prefix: shell.context.meta_prefix.clone(),
            accounts: Mutex::new(accounts.into_iter().collect()),
            dispatches: Mutex::default(),
            cancellations: Mutex::default(),
        });
        let executor = Arc::new(ServingExecutor::new(script.clone()));
        let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
        ready(&shell.config.engine_url).await;
        (script, task)
    }

    fn expect_dispatches(&self, names: &[&str]) {
        let dispatches = self.dispatches.lock().unwrap();
        let actual: Vec<_> = dispatches
            .iter()
            .map(|intent| intent["call"]["params"]["name"].as_str().unwrap())
            .collect();
        assert_eq!(actual, names);
    }
}

impl Executor for PollScript {
    fn dispatch(&self, intent: &Value) -> Result<Execution> {
        if let Some(reply) = scripted_discovery(intent) {
            return Ok(reply);
        }
        self.dispatches.lock().unwrap().push(intent.clone());
        let (account_hook, sign_in_hook) = if self.plain {
            ("account_hook", "sign_in_hook")
        } else {
            ("account-hook.v1", "sign-in-hook.v1")
        };
        let name = intent["call"]["params"]["name"].as_str().unwrap();
        let mut result = if name == sign_in_hook {
            json!({
                "resultType": "input_required",
                "inputRequests": {"login": {"method": "elicitation/create", "params": {
                    "mode": "url", "message": HOOK_MESSAGE, "url": SIGN_IN_URL
                }}},
                "requestState": HOOK_STATE
            })
        } else {
            let mut result = json!({"content": []});
            if name == account_hook {
                let account = self
                    .accounts
                    .lock()
                    .unwrap()
                    .pop_front()
                    .unwrap_or(Value::Null);
                result["structuredContent"] = json!({"account": account});
            }
            result
        };
        if name != sign_in_hook && !self.plain {
            result["_meta"] = json!({format!("{}outcome", self.prefix): "succeeded"});
        }
        Ok(Execution::Complete(
            json!({"kind": "result", "result": result}),
        ))
    }

    fn cancel(&self, intent: &Value) -> Result<Execution> {
        self.cancellations.lock().unwrap().push(intent.clone());
        Ok(Execution::Done)
    }
}

fn assert_invalid_sign_in_login(result: &Value, prefix: &str, text: &str) {
    assert!(result["isError"] == true);
    assert!(result["_meta"][format!("{prefix}reason")] == "invalid");
    let diagnostics = result["structuredContent"]["diagnostics"]
        .as_array()
        .unwrap();
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0]["ruleId"] == "PND-010");
    assert!(
        diagnostics[0]["message"]["text"]
            .as_str()
            .unwrap()
            .contains(text)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sign_in_command_opens_the_url_and_polls_to_signed_in() {
    let temp = temporary();
    let case = repository().join("conformance/conventions/requests/sign-in-hold");
    let shell = Arc::new(fixed_case_shell(temp.path(), &case, 11));
    let prefix = &shell.context.meta_prefix;
    let signed = actor(&shell, "signed");
    let stop = CancellationToken::new();
    let (script, task) = PollScript::serve(
        &shell,
        &stop,
        false,
        [Value::Null, Value::Null, json!("primary-account")],
    )
    .await;
    let object = json!({"kind": "Agent", "name": "signed"});
    let mut person = Scripted::new(&[Some("y"), Some(""), Some("")]);
    assert_eq!(
        crate::author::route(temp.path(), &object, "sign-in.v1").unwrap(),
        "group.sign-in.v1"
    );
    let result = crate::sign_in::sign_in(temp.path(), &object, None, prefix, &mut person)
        .await
        .unwrap();
    assert!(result["isError"] != true);
    assert!(result["structuredContent"]["account"] == "primary-account");
    assert!(result["_meta"][format!("{prefix}outcome")] == "succeeded");
    assert_eq!(person.opened, [SIGN_IN_URL]);
    assert_eq!(person.prompts, [CONSENT_PROMPT, ENTER_PROMPT, ENTER_PROMPT]);
    assert_eq!(
        person.events[..3],
        [
            CONSENT_PROMPT.to_string(),
            format!("open:{SIGN_IN_URL}"),
            ENTER_PROMPT.to_string()
        ]
    );
    assert_eq!(person.told.len(), 10);
    assert_eq!(
        person.told[0],
        "conduit sign-in: the engine asks you to open a URL to sign in Agent/signed."
    );
    assert_eq!(
        person.told[5],
        "conduit sign-in: the engine has not yet found the expected account signed in on Agent/signed; the URL below is the one already opened."
    );
    for lines in person.told.chunks_exact(5) {
        assert!(lines[1].starts_with("Message: ") && lines[1].contains("example.org"));
        assert_eq!(lines[2], format!("URL: {SIGN_IN_URL}"));
        assert_eq!(lines[3], "Login: browser");
        assert!(lines[4].starts_with(RETURN_NOTICE));
        assert!(
            lines[4].contains("this command carries none (order 051b-3, with the keyed release)")
        );
    }
    script.expect_dispatches(&[
        "account-hook.v1",
        "sign-in-hook.v1",
        "account-hook.v1",
        "account-hook.v1",
    ]);
    assert!(!shell.state().unwrap().sign_in_holds.contains_key(&signed));
    stop.cancel();
    task.await.unwrap().unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sign_in_command_declines_cancels_and_asks_for_a_login() {
    let temp = temporary();
    let case = repository().join("conformance/conventions/requests/sign-in-hold");
    let shell = Arc::new(fixed_case_shell(temp.path(), &case, 11));
    let prefix = &shell.context.meta_prefix;
    let signed = actor(&shell, "signed");
    let stop = CancellationToken::new();
    let (script, task) = PollScript::serve(&shell, &stop, false, []).await;
    let object = json!({"kind": "Agent", "name": "signed"});

    let mut person = Scripted::new(&[Some("maybe"), Some("n")]);
    let declined = crate::sign_in::sign_in(temp.path(), &object, None, prefix, &mut person)
        .await
        .unwrap();
    assert!(declined["isError"] == true);
    assert!(declined["_meta"][format!("{prefix}outcome")] == "failed");
    assert!(
        declined["structuredContent"]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|diagnostic| diagnostic["message"]["text"]
                .as_str()
                .is_some_and(|text| text.contains("declined")))
    );
    assert_eq!(person.prompts, [CONSENT_PROMPT, CONSENT_PROMPT]);
    assert!(person.opened.is_empty());

    let mut person = Scripted::default();
    let invalid = crate::sign_in::sign_in(temp.path(), &object, Some("phone"), prefix, &mut person)
        .await
        .unwrap();
    assert_invalid_sign_in_login(&invalid, prefix, "browser");
    assert!(person.prompts.is_empty() && person.told.is_empty() && person.opened.is_empty());

    let mut person = Scripted::default();
    let error = crate::sign_in::run(
        temp.path(),
        &["Agent/signed".into()],
        None,
        &repository(),
        &mut person,
    )
    .await
    .unwrap_err();
    assert!(error.contains("did not succeed"));
    assert_eq!(person.prompts, [CONSENT_PROMPT]);
    assert!(person.opened.is_empty());
    script.expect_dispatches(&[
        "account-hook.v1",
        "sign-in-hook.v1",
        "account-hook.v1",
        "sign-in-hook.v1",
    ]);
    assert!(shell.state().unwrap().sign_in_holds.contains_key(&signed));
    stop.cancel();
    task.await.unwrap().unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sign_in_command_signs_in_through_a_plain_hook() {
    let temp = temporary();
    let case = repository().join("conformance/conventions/requests/sign-in-plain-hook-part");
    let shell = Arc::new(fixed_case_shell(temp.path(), &case, 2));
    let prefix = &shell.context.meta_prefix;
    let stop = CancellationToken::new();
    let (script, task) =
        PollScript::serve(&shell, &stop, true, [Value::Null, json!("primary-account")]).await;
    let object = json!({"kind": "Agent", "name": "signed"});

    let mut person = Scripted::default();
    let invalid =
        crate::sign_in::sign_in(temp.path(), &object, Some("browser"), prefix, &mut person)
            .await
            .unwrap();
    assert_invalid_sign_in_login(&invalid, prefix, "is a plain hook");
    assert!(person.prompts.is_empty() && person.told.is_empty() && person.opened.is_empty());
    script.expect_dispatches(&[]);

    let mut person = Scripted::new(&[Some("y"), Some("")]);
    let result = crate::sign_in::sign_in(temp.path(), &object, None, prefix, &mut person)
        .await
        .unwrap();
    assert!(result["isError"] != true);
    assert!(result["structuredContent"]["account"] == "primary-account");
    assert!(result["_meta"][format!("{prefix}outcome")] == "succeeded");
    assert_eq!(person.opened, [SIGN_IN_URL]);
    assert_eq!(person.prompts, [CONSENT_PROMPT, ENTER_PROMPT]);
    assert_eq!(
        person
            .told
            .iter()
            .filter(|line| line.as_str() == "Login: browser")
            .count(),
        1
    );
    assert!(
        !person
            .told
            .iter()
            .any(|line| line.starts_with(RETURN_NOTICE))
    );
    script.expect_dispatches(&["account_hook", "sign_in_hook", "account_hook"]);
    stop.cancel();
    task.await.unwrap().unwrap();
}

async fn raw_session(url: &str, token: &str, revision: &str) -> Option<String> {
    install_crypto_provider();
    if revision == "2026-07-28" {
        return None;
    }
    let http = reqwest::Client::new();
    let response = http.post(url).bearer_auth(token)
        .header("accept", "application/json, text/event-stream")
        .json(&json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":revision,"capabilities":{},"clientInfo":{"name":"raw-test","version":"1"}}}))
        .send().await.unwrap();
    assert_eq!(response.status(), 200);
    let session = response.headers()["mcp-session-id"]
        .to_str()
        .unwrap()
        .to_owned();
    let _ = response.text().await.unwrap();
    let response = http
        .post(url)
        .bearer_auth(token)
        .header("accept", "application/json, text/event-stream")
        .header("mcp-protocol-version", revision)
        .header("mcp-session-id", &session)
        .json(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 202);
    Some(session)
}

#[test]
fn endpoint_hosts_are_checked_before_fixing_the_audience() {
    for host in ["127.0.0.2", "127.1.2.3", "192.0.2.1", "[::ffff:127.0.0.1]"] {
        let temp = temporary();
        let config = Config {
            engine_url: format!("http://{host}:3000/mcp"),
            ..Config::default()
        };
        let error = Shell::open(temp.path(), context(), config).err().unwrap();
        assert!(error.contains("127.0.0.1, localhost or ::1"), "{error}");
        assert!(!temp.path().join("engine-url.json").exists());
        // A refused first start must leave the supported endpoint available.
        drop(open(temp.path()));
    }
    for host in ["127.0.0.1", "localhost", "[::1]"] {
        let temp = temporary();
        let config = Config {
            engine_url: format!("http://{host}:3000/mcp"),
            ..Config::default()
        };
        drop(Shell::open(temp.path(), context(), config).unwrap());
        assert!(temp.path().join("engine-url.json").exists());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn rejected_calls_keep_valid_keys_and_onward_attribution() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    let b = token(&shell, "agent-b");
    let memory = Arc::new(Memory {
        content: Mutex::default(),
        shell: Arc::downgrade(&shell),
        dispatches: AtomicUsize::new(0),
        deferred: AtomicBool::new(true),
        cancellations: Mutex::default(),
    });
    let executor = Arc::new(ServingExecutor::new(memory.clone()));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor.clone(), stop.clone()));
    let url = &shell.config.engine_url;
    ready(url).await;
    let session = raw_session(url, &b, "2025-06-18").await.unwrap();
    let prefix = &shell.context.meta_prefix;
    let http = reqwest::Client::new();
    // Origin rejection, extension-capability rejection, and handler metadata validation.
    for (origin, declares, serving, key) in [
        (true, true, Value::Null, json!("origin-key")),
        (false, false, Value::Null, json!("capability-key")),
        (false, true, json!(42), json!("valid-key")),
    ] {
        let mut meta = json!({format!("{prefix}idempotencyKey"):key});
        if !serving.is_null() {
            meta[format!("{prefix}serving")] = serving;
        }
        if declares {
            meta["io.modelcontextprotocol/clientCapabilities"] = json!({"extensions":{format!("{prefix}implementation-interface"):{"revision":"v0"}}});
        }
        let before = shell.state().unwrap().records.len();
        let mut request = http.post(url).bearer_auth(&b)
            .header("accept", "application/json, text/event-stream")
            .header("mcp-session-id", &session)
            .header("mcp-protocol-version", "2025-06-18")
            .json(&json!({"jsonrpc":"2.0","id":71,"method":"tools/call","params":{"name":"project.file.write.v1","arguments":{"path":"x","content":"never"},"_meta":meta}}));
        if origin {
            request = request.header("origin", "http://attacker.invalid");
        }
        let response = request.send().await.unwrap();
        if origin {
            assert_eq!(response.status(), 403);
        } else if !declares {
            assert_eq!(response.status(), 400);
        } else {
            assert!(response.text().await.unwrap().contains("nonempty string"));
        }
        let state = shell.writer.lock().unwrap().store.read().unwrap().state;
        let records = &state.records[before..];
        for kind in ["access-decision", "request-ended"] {
            let record = records.iter().find(|r| r["type"] == kind).unwrap();
            assert_eq!(record["data"]["idempotencyKey"], key);
            assert_eq!(record["data"]["caller"], actor(&shell, "agent-b"));
            assert_eq!(record["data"]["reason"], "invalid");
            assert!(record["data"].get("serving").is_none());
        }
        assert_eq!(memory.dispatches.load(Ordering::SeqCst), 0);
    }
    // A live invocation establishes an original caller different from its target.
    let pending = shell.begin(json!({"kind":"invoke","request":{"identity":instance_value(&actor(&shell,"agent-b")),"tool":"project.file.read.v1","arguments":{"path":"hello.txt"}}}), executor.as_ref()).unwrap();
    let invocation = pending.invocation().unwrap();
    // DL2: copy the selected value from the shared last-commit snapshot.
    let parent = shell
        .state()
        .unwrap()
        .invocations
        .iter()
        .find(|v| v["id"] == invocation)
        .cloned()
        .unwrap();
    let target = parent["target"].as_str().unwrap();
    let onward_token = shell.authority.token(target, url).unwrap();
    for (origin, key) in [(true, json!("onward-key")), (false, json!(42))] {
        let onward_session = raw_session(url, &onward_token, "2025-06-18").await.unwrap();
        let before = shell.state().unwrap().records.len();
        let mut request = http.post(url).bearer_auth(&onward_token)
            .header("accept", "application/json, text/event-stream")
            .header("mcp-session-id", onward_session)
            .header("mcp-protocol-version", "2025-06-18")
            .json(&json!({"jsonrpc":"2.0","id":72,"method":"tools/call","params":{"name":"project.file.read.v1","arguments":{"path":"x"},"_meta":{format!("{prefix}serving"):invocation,format!("{prefix}idempotencyKey"):key,"io.modelcontextprotocol/clientCapabilities":{"extensions":{format!("{prefix}implementation-interface"):{"revision":"v0"}}}}}}));
        if origin {
            request = request.header("origin", "http://attacker.invalid");
        }
        let response = request.send().await.unwrap();
        if origin {
            assert_eq!(response.status(), 403);
        } else {
            assert!(response.text().await.unwrap().contains("nonempty string"));
        }
        let state = shell.writer.lock().unwrap().store.read().unwrap().state;
        for kind in ["access-decision", "request-ended"] {
            let record = state.records[before..]
                .iter()
                .find(|r| r["type"] == kind)
                .unwrap();
            assert_eq!(record["data"]["serving"], invocation);
            assert_eq!(record["data"]["consumer"], target);
            assert_eq!(record["data"]["caller"], parent["caller"]);
            assert_ne!(record["data"]["caller"], target);
            if origin {
                assert_eq!(record["data"]["idempotencyKey"], key);
            } else {
                assert!(record["data"].get("idempotencyKey").is_none());
            }
        }
        assert_eq!(memory.dispatches.load(Ordering::SeqCst), 1);
    }
    stop.cancel();
    task.await.unwrap().unwrap();
}

// A single-thread runtime makes the overflow deterministic: all sends happen
// before the forwarding task can poll its receiver again.
#[tokio::test]
async fn notification_overflow_closes_legacy_sessions_and_reconnect_delivers() {
    use futures_util::StreamExt;
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    let a = token(&shell, "agent-a");
    let executor = Arc::new(ServingExecutor::new(Arc::new(Memory {
        content: Mutex::default(),
        shell: Arc::downgrade(&shell),
        dispatches: AtomicUsize::new(0),
        deferred: AtomicBool::new(false),
        cancellations: Mutex::default(),
    })));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor.clone(), stop.clone()));
    let url = &shell.config.engine_url;
    ready(url).await;
    for revision in ["2025-06-18", "2025-11-25"] {
        let session = raw_session(url, &a, revision).await.unwrap();
        let http = reqwest::Client::new();
        let response = http
            .get(url)
            .bearer_auth(&a)
            .header("accept", "text/event-stream")
            .header("mcp-session-id", &session)
            .header("mcp-protocol-version", revision)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let mut stream = response.bytes_stream();
        let _ = stream.next().await.unwrap().unwrap();
        let changes = vec![json!({"consumer":actor(&shell,"agent-a")}); 1025];
        executor.tools_changed(&changes).unwrap();
        tokio::time::timeout(Duration::from_secs(600), async {
            // Idle expiry must not stand in for overflow cleanup. Keep the
            // session active while independently draining its notification stream.
            let keepalive = async {
                let mut interval = tokio::time::interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    let response = http
                        .post(url)
                        .bearer_auth(&a)
                        .header("accept", "application/json, text/event-stream")
                        .header("mcp-session-id", &session)
                        .header("mcp-protocol-version", revision)
                        .json(&json!({"jsonrpc":"2.0","id":74,"method":"ping"}))
                        .send()
                        .await
                        .unwrap();
                    // Closure may race the ping; neither response ends the wait.
                    assert!(matches!(response.status().as_u16(), 200 | 404));
                    response.bytes().await.unwrap();
                }
            };
            tokio::select! {
                _ = keepalive => unreachable!(),
                _ = async {
                    while let Some(chunk) = stream.next().await {
                        chunk.unwrap();
                    }
                } => {}
            }
        })
        .await
        .unwrap();
        let response = http
            .post(url)
            .bearer_auth(&a)
            .header("accept", "application/json, text/event-stream")
            .header("mcp-session-id", &session)
            .header("mcp-protocol-version", revision)
            .json(&json!({"jsonrpc":"2.0","id":73,"method":"tools/list"}))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 404);
        let changes = Arc::new(AtomicUsize::new(0));
        let client = connect(url, &a, changes.clone()).await;
        executor
            .tools_changed(&[json!({"consumer":actor(&shell,"agent-a")})])
            .unwrap();
        wait_changes(&changes, 1).await;
        assert_eq!(client.list_tools(None).await.unwrap().tools.len(), 5);
        client.cancel().await.unwrap();
    }
    stop.cancel();
    task.await.unwrap().unwrap();
}

async fn raw_pending_call(
    url: &str,
    token: &str,
    revision: &str,
    session: Option<&str>,
    id: u64,
    tool: &str,
) -> tokio::net::TcpStream {
    use tokio::io::AsyncWriteExt;
    let url = url::Url::parse(url).unwrap();
    let address = format!("{}:{}", url.host_str().unwrap(), url.port().unwrap());
    let mut socket = tokio::net::TcpStream::connect(&address).await.unwrap();
    let mut params = json!({"name":tool,"arguments":{"path":"hello.txt"}});
    if tool == "project.file.write.v1" {
        params["arguments"]["content"] = json!("pending");
    }
    if revision == "2026-07-28" {
        params["_meta"] = json!({"io.modelcontextprotocol/protocolVersion":revision,"io.modelcontextprotocol/clientInfo":{"name":"raw-test","version":"1"},"io.modelcontextprotocol/clientCapabilities":{}});
    }
    let body = json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":params}).to_string();
    let session = session
        .map(|s| format!("Mcp-Session-Id: {s}\r\n"))
        .unwrap_or_default();
    let request = format!(
        "POST /mcp HTTP/1.1\r\nHost: {address}\r\nAuthorization: Bearer {token}\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMcp-Method: tools/call\r\nMcp-Name: {tool}\r\nMcp-Protocol-Version: {revision}\r\n{session}Content-Length: {}\r\n\r\n{body}",
        body.len()
    );
    socket.write_all(request.as_bytes()).await.unwrap();
    socket
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn dropped_http_stream_cancels_only_its_invocation_for_every_revision() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    let b = token(&shell, "agent-b");
    let memory = Arc::new(Memory {
        content: Mutex::default(),
        shell: Arc::downgrade(&shell),
        dispatches: AtomicUsize::new(0),
        deferred: AtomicBool::new(true),
        cancellations: Mutex::default(),
    });
    let executor = Arc::new(ServingExecutor::new(memory.clone()));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
    ready(&shell.config.engine_url).await;
    for (index, revision) in server::REVISIONS.iter().enumerate() {
        let session = raw_session(&shell.config.engine_url, &b, revision).await;
        let read = raw_pending_call(
            &shell.config.engine_url,
            &b,
            revision,
            session.as_deref(),
            2,
            "project.file.read.v1",
        )
        .await;
        wait_changes(&memory.dispatches, index * 2 + 1).await;
        let read_id = shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .find(|i| i["state"] == "in-progress")
            .unwrap()["id"]
            .clone();
        let write = raw_pending_call(
            &shell.config.engine_url,
            &b,
            revision,
            session.as_deref(),
            3,
            "project.file.write.v1",
        )
        .await;
        wait_changes(&memory.dispatches, index * 2 + 2).await;
        let write_id = shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .find(|i| i["state"] == "in-progress" && i["id"] != read_id)
            .unwrap()["id"]
            .clone();
        // No notifications/cancelled or session DELETE is sent: close only this response.
        drop(read);
        tokio::time::timeout(Duration::from_secs(600), async {
            loop {
                if memory.cancellations.lock().unwrap().len() == index * 2 + 1 {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
        let state = shell.writer.lock().unwrap().store.read().unwrap().state;
        assert_eq!(
            state
                .invocations
                .iter()
                .find(|i| i["id"] == read_id)
                .unwrap()["state"],
            "unknown"
        );
        assert_eq!(
            memory.cancellations.lock().unwrap().last().unwrap()["invocation"],
            read_id
        );
        assert!(state.records.iter().any(|r| r["type"] == "invocation-ended"
            && r["data"]["invocation"] == read_id
            && r["data"]["outcome"] == "unknown"));
        assert!(
            state
                .records
                .iter()
                .any(|r| r["type"] == "request-ended" && r["data"]["invocation"] == read_id)
        );
        assert_eq!(
            state
                .invocations
                .iter()
                .filter(|i| i["state"] == "in-progress")
                .count(),
            1
        );
        drop(write);
        tokio::time::timeout(Duration::from_secs(600), async {
            loop {
                if memory.cancellations.lock().unwrap().len() == index * 2 + 2 {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
        let state = shell.writer.lock().unwrap().store.read().unwrap().state;
        assert_eq!(
            state
                .invocations
                .iter()
                .find(|i| i["id"] == write_id)
                .unwrap()["state"],
            "unknown"
        );
        assert!(
            state
                .invocations
                .iter()
                .all(|i| i["state"] != "in-progress")
        );
    }
    stop.cancel();
    task.await.unwrap().unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn malformed_unidentified_calls_keep_protocol_error_without_records() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    let executor = Arc::new(ServingExecutor::new(Arc::new(MissingExecutor)));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
    ready(&shell.config.engine_url).await;
    let b = token(&shell, "agent-b");
    let http = reqwest::Client::new();
    for revision in server::REVISIONS {
        let session = raw_session(&shell.config.engine_url, &b, revision).await;
        for token in [None, Some("forged")] {
            for mut params in [
                json!({"arguments":{}}),
                json!([]),
                json!({"name":"project.file.read.v1","arguments":[]}),
            ] {
                if revision == "2026-07-28" && params.is_object() {
                    params["_meta"] = json!({"io.modelcontextprotocol/protocolVersion":revision,"io.modelcontextprotocol/clientInfo":{"name":"raw-test","version":"1"},"io.modelcontextprotocol/clientCapabilities":{}});
                }
                let before = store::private_read(&temp.path().join("state.json")).unwrap();
                let mut request = http
                    .post(&shell.config.engine_url)
                    .header("accept", "application/json, text/event-stream")
                    .header("mcp-method", "tools/call")
                    .header("mcp-protocol-version", revision)
                    .json(&json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":params}));
                if let Some(name) = params["name"].as_str() {
                    request = request.header("mcp-name", name);
                }
                if let Some(token) = token {
                    request = request.bearer_auth(token);
                }
                if let Some(session) = &session {
                    request = request.header("mcp-session-id", session);
                }
                let response = request.send().await.unwrap();
                // rmcp falls back to CustomRequest for malformed object parameters,
                // answering method-not-found; a nonobject params fails decoding.
                let status = response.status();
                let text = response.text().await.unwrap();
                if params.is_array() {
                    assert_eq!(status, 415, "{text}");
                    assert!(text.starts_with("fail to deserialize request body"));
                } else {
                    assert_eq!(
                        status,
                        if revision == "2026-07-28" { 404 } else { 200 },
                        "{text}"
                    );
                    let data = text
                        .lines()
                        .filter_map(|l| l.strip_prefix("data:"))
                        .find(|data| data.trim_start().starts_with('{'))
                        .unwrap_or(&text);
                    let error: Value = serde_json::from_str(data).unwrap();
                    assert_eq!(error["error"]["code"], -32601);
                    assert!(error["error"].get("data").is_none());
                }
                assert_eq!(
                    store::private_read(&temp.path().join("state.json")).unwrap(),
                    before
                );
            }
        }
    }
    stop.cancel();
    task.await.unwrap().unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn custom_controls_are_unfinished_for_consumers_and_management() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    let fixture = boundary::value(
        &repository().join("conformance/interface/schemas/e05-control/fixture.json"),
    )
    .unwrap();
    let operation = fixture["operation"].as_str().unwrap();
    let root = actor(&shell, "root");
    let a = actor(&shell, "agent-a");
    for source in ["type", "template"] {
        let writer = shell.writer.lock().unwrap();
        let mut stored = writer.store.read().unwrap();
        stored
            .state
            .objects
            .retain(|o| o["id"] != "target" && o["id"] != "connection");
        let mut target = fixture["state"]["objects"][0].clone();
        target["name"] = json!("target"); // IFC-033 includes the actual target's name.
        let mut environment = fixture["environment"].clone();
        environment["scopes"]["main"]["contracts"]["controlled.v1"] = json!({"kind":"Type","metadata":{"name":"controlled.v1"},"spec":{"controls":["#test.v1"]}});
        // A registered instance keeps its environment in held, rather than on the
        // publication view. A conflicting newer scope must not determine this limit.
        environment["scopes"]["newer"] = environment["scopes"]["main"].clone();
        environment["scopes"]["newer"]["contracts"]["test.v1"]["spec"]
            .as_object_mut()
            .unwrap()
            .remove("changes");
        target["type"] = json!("example.org/e05/main#controlled.v1");
        target["held"] = json!({"environment":environment,"indexes":{"main":environment["scopes"]["main"]["contracts"]},"contexts":{},"scope":"main"});
        target["resolved_implementations"][operation] = json!({"source":source});
        let mut path = fixture["state"]["objects"][1].clone();
        path["consumer"] = json!(a);
        stored.state.objects.extend([target, path]);
        writer.store.write(&stored).unwrap();
        drop(writer);
        assert_eq!(
            server::unfinished(&shell, &a).unwrap()["use.test.v1"],
            operation
        );
        let error = server::preflight(&shell).unwrap_err();
        assert!(
            error.contains(operation) && error.contains("unfinished"),
            "{error}"
        );
    }
    {
        let writer = shell.writer.lock().unwrap();
        let mut stored = writer.store.read().unwrap();
        stored
            .state
            .objects
            .iter_mut()
            .find(|o| o["id"] == "connection")
            .unwrap()["consumer"] = json!(root);
        writer.store.write(&stored).unwrap();
    }
    server::preflight(&shell).unwrap();
    shell.issue_client_token(&root).unwrap();
    let executor = Arc::new(ServingExecutor::new(Arc::new(MissingExecutor)));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
    ready(&shell.config.engine_url).await;
    let token = String::from_utf8(
        store::private_read(&temp.path().join("client-tokens").join(&root)).unwrap(),
    )
    .unwrap();
    let client = connect(&shell.config.engine_url, &token, Arc::default()).await;
    assert!(
        client
            .list_tools(None)
            .await
            .unwrap()
            .tools
            .iter()
            .any(|t| t.name == "use.test.v1")
    );
    for source in ["type", "template"] {
        {
            let writer = shell.writer.lock().unwrap();
            let mut stored = writer.store.read().unwrap();
            stored
                .state
                .objects
                .iter_mut()
                .find(|o| o["id"] == "target")
                .unwrap()["resolved_implementations"][operation]["source"] = json!(source);
            writer.store.write(&stored).unwrap();
        }
        let before = store::private_read(&temp.path().join("state.json")).unwrap();
        let error = client
            .call_tool(call("use.test.v1", json!({"revision":"r","limit":50})))
            .await
            .unwrap_err();
        let rmcp::ServiceError::McpError(error) = error else {
            panic!("expected protocol error");
        };
        assert_eq!(error.code, ErrorCode::INTERNAL_ERROR);
        assert!(error.message.contains(operation) && error.message.contains("unfinished"));
        assert_eq!(
            store::private_read(&temp.path().join("state.json")).unwrap(),
            before
        );
    }
    client.cancel().await.unwrap();
    stop.cancel();
    task.await.unwrap().unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn retired_agent_consumer_stream_ends_and_other_consumer_remains() {
    use futures_util::StreamExt;
    for revision in ["2025-06-18", "2025-11-25"] {
        let temp = temporary();
        let shell = Arc::new(fixed_shell(temp.path()));
        let a = token(&shell, "agent-a");
        let b = token(&shell, "agent-b");
        let executor = Arc::new(ServingExecutor::new(Arc::new(MissingExecutor)));
        let stop = CancellationToken::new();
        let task = tokio::spawn(server::serve(shell.clone(), executor.clone(), stop.clone()));
        ready(&shell.config.engine_url).await;
        let session = raw_session(&shell.config.engine_url, &a, revision)
            .await
            .unwrap();
        let http = reqwest::Client::new();
        let response = http
            .get(&shell.config.engine_url)
            .bearer_auth(&a)
            .header("accept", "text/event-stream")
            .header("mcp-session-id", &session)
            .header("mcp-protocol-version", revision)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let mut stream = response.bytes_stream();
        stream.next().await.unwrap().unwrap();
        let modern_client = modern(&shell.config.engine_url, &a).await;
        let mut subscription = modern_client
            .listen(
                SubscriptionFilterBuilder::default()
                    .tools_list_changed()
                    .build(),
            )
            .await
            .unwrap();
        modern_client.list_tools(None).await.unwrap();
        let other = connect(&shell.config.engine_url, &b, Arc::default()).await;
        let list = other.list_tools(None).await.unwrap().tools;
        let root = actor(&shell, "root");
        let reply=shell.submit(json!({"kind":"invoke","request":{"identity":instance_value(&root),"tool":"s1.retire.v1","arguments":{"instance":{"kind":"Agent","name":"agent-a"}}}}),executor.as_ref()).unwrap();
        assert_eq!(reply["isError"], false, "{reply}");
        // The retired consumer's streams must end because retirement ends its sessions, not
        // because the session idled out. So both waits stay below the session idle limit: rmcp's
        // 300 s default, and call_timeout + 60 s (180 s by default) once order 074 sets it.
        tokio::time::timeout(Duration::from_secs(150), async {
            while let Some(chunk) = stream.next().await {
                chunk.unwrap();
            }
        })
        .await
        .unwrap();
        tokio::time::timeout(Duration::from_secs(150), async {
            while subscription.next().await.unwrap().is_some() {}
        })
        .await
        .unwrap();
        modern_client.cancel().await.unwrap();
        assert_eq!(other.list_tools(None).await.unwrap().tools, list);
        assert!(matches!(
            shell.verify_token(Some(&a)).unwrap(),
            conduit_core::access::Identity::Unreadable
        ));
        other.cancel().await.unwrap();
        stop.cancel();
        task.await.unwrap().unwrap();
    }
}

#[tokio::test]
async fn initialization_failures_retry_only_handshake_and_name_step_and_wait() {
    install_crypto_provider();
    use axum::{Json, Router, http::StatusCode, routing::post};
    let temp = temporary();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/mcp", listener.local_addr().unwrap());
    let shell = Shell::open(
        temp.path(),
        context(),
        Config {
            engine_url: url,
            ..Config::default()
        },
    )
    .unwrap();
    shell
        .submit(
            json!({"kind":"first-start","machine":crate::local_machine(BTreeMap::new()).unwrap()}),
            &MissingExecutor,
        )
        .unwrap();
    shell.issue_client_token(&actor(&shell, "root")).unwrap();
    let methods = Arc::new(Mutex::new(Vec::new()));
    let seen = methods.clone();
    let router = Router::new().route(
        "/mcp",
        post(move |Json(value): Json<Value>| {
            let seen = seen.clone();
            async move {
                seen.lock().unwrap().push(value["method"].clone());
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "initialization unavailable",
                )
            }
        }),
    );
    let server = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    let error = server::client(temp.path(), None, Some("management.retire.v1"), json!({}))
        .await
        .unwrap_err();
    assert!(error.contains("session initialization"), "{error}");
    assert!(error.contains("seconds (attempt 3/3)"), "{error}");
    assert_eq!(*methods.lock().unwrap(), vec![json!("initialize"); 3]);
    server.abort();
}

#[tokio::test]
async fn request_failure_after_initialization_keeps_tool_identity_cause_and_remedy() {
    install_crypto_provider();
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    shell.issue_client_token(&actor(&shell, "agent-a")).unwrap();
    let stop = CancellationToken::new();
    let serving = tokio::spawn(server::serve(
        shell.clone(),
        Arc::new(ServingExecutor::new(Arc::new(MissingExecutor))),
        stop.clone(),
    ));
    ready(&shell.config.engine_url).await;
    let error = server::client(
        temp.path(),
        Some("Agent/agent-a"),
        Some("project.file.write.v1"),
        json!({"path":"hello.txt","content":"refused"}),
    )
    .await
    .unwrap_err();
    for expected in [
        "project.file.write.v1",
        "Agent/agent-a",
        "refused",
        "check conduit tools",
        "serving engine's diagnostics",
        "request was not retried",
    ] {
        assert!(error.contains(expected), "missing {expected}: {error}");
    }
    stop.cancel();
    serving.await.unwrap().unwrap();
}

#[tokio::test]
async fn transport_failure_after_initialization_keeps_context_without_retry() {
    install_crypto_provider();
    use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};
    for tool in [Some("management.retire.v1"), None] {
        let temp = temporary();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let shell = Shell::open(
            temp.path(),
            context(),
            Config {
                engine_url: format!("http://{}/mcp", listener.local_addr().unwrap()),
                ..Config::default()
            },
        )
        .unwrap();
        shell.submit(json!({"kind":"first-start","machine":crate::local_machine(BTreeMap::new()).unwrap()}), &MissingExecutor).unwrap();
        shell.issue_client_token(&actor(&shell, "root")).unwrap();
        let methods = Arc::new(Mutex::new(Vec::new()));
        let seen = methods.clone();
        let router = Router::new().route("/mcp", post(move |Json(value): Json<Value>| {
            let seen = seen.clone();
            async move {
                seen.lock().unwrap().push(value["method"].clone());
                match value["method"].as_str() {
                    Some("initialize") => Json(json!({"jsonrpc":"2.0","id":value["id"],"result":{"protocolVersion":"2025-06-18","capabilities":{"tools":{}},"serverInfo":{"name":"request-failure","version":"1"}}})).into_response(),
                    Some("notifications/initialized") => StatusCode::ACCEPTED.into_response(),
                    _ => StatusCode::SERVICE_UNAVAILABLE.into_response(),
                }
            }
        }));
        let serving = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        let error = server::client(temp.path(), Some("Interface/root"), tool, json!({}))
            .await
            .unwrap_err();
        for expected in [
            tool.unwrap_or("tools/list"),
            "Interface/root",
            "503",
            "check conduit tools",
            "serving engine's diagnostics",
            "request was not retried",
        ] {
            assert!(error.contains(expected), "missing {expected}: {error}");
        }
        let methods = methods.lock().unwrap();
        for method in [
            "initialize",
            if tool.is_some() {
                "tools/call"
            } else {
                "tools/list"
            },
        ] {
            assert_eq!(methods.iter().filter(|m| **m == method).count(), 1);
        }
        serving.abort();
    }
}

#[tokio::test]
async fn authentication_waiting_for_commit_does_not_block_async_worker() {
    install_crypto_provider();
    let temp = temporary();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/mcp", listener.local_addr().unwrap());
    drop(listener);
    let shell = Arc::new(
        Shell::open(
            temp.path(),
            context(),
            Config {
                engine_url: url.clone(),
                ..Config::default()
            },
        )
        .unwrap(),
    );
    shell
        .submit(
            json!({"kind":"first-start","machine":crate::local_machine(BTreeMap::new()).unwrap()}),
            &MissingExecutor,
        )
        .unwrap();
    let stop = CancellationToken::new();
    let serving = tokio::spawn(server::serve(
        shell.clone(),
        Arc::new(ServingExecutor::new(Arc::new(MissingExecutor))),
        stop.clone(),
    ));
    ready(&url).await;
    let (locked, acquired) = tokio::sync::oneshot::channel();
    let (release, released) = std::sync::mpsc::channel();
    let locked_shell = shell.clone();
    let holder = std::thread::spawn(move || {
        let _guard = locked_shell.writer.lock().unwrap();
        locked.send(()).unwrap();
        let _ = released.recv_timeout(Duration::from_secs(4));
    });
    acquired.await.unwrap();
    let started = Instant::now();
    let request = tokio::spawn(async move {
        reqwest::Client::new()
            .post(url)
            .json(&json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}))
            .send()
            .await
            .unwrap()
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    let elapsed = started.elapsed();
    let _ = release.send(());
    holder.join().unwrap();
    assert!(
        elapsed < Duration::from_secs(2),
        "authentication blocked async worker for {elapsed:?}"
    );
    assert_eq!(
        request.await.unwrap().status(),
        reqwest::StatusCode::UNAUTHORIZED
    );
    stop.cancel();
    serving.await.unwrap().unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn root_refusal_carries_tier_two() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    server::preflight(&shell).unwrap();
    shell.issue_client_token(&actor(&shell, "root")).unwrap();
    let executor = Arc::new(ServingExecutor::new(Arc::new(crate::MissingExecutor)));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
    ready(&shell.config.engine_url).await;
    let reply = server::client(
        temp.path(),
        None,
        Some("management.retire.v1"),
        json!({"instance":{"kind":"Pool","name":"system"}}),
    )
    .await
    .unwrap();
    let refusal = &reply["structuredContent"]["diagnostics"][0]["properties"]["refusal"];
    assert_eq!(refusal["tier"], 2);
    assert_eq!(refusal["conduits"].as_array().unwrap().len(), 1);
    assert_eq!(refusal["conduits"][0]["as"], "management");
    stop.cancel();
    tokio::time::timeout(Duration::from_secs(60), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn shell_rejection_carries_diagnostic() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    server::preflight(&shell).unwrap();
    let root = actor(&shell, "root");
    shell.issue_client_token(&root).unwrap();
    let token = token(&shell, "root");
    let executor = Arc::new(ServingExecutor::new(Arc::new(crate::MissingExecutor)));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
    ready(&shell.config.engine_url).await;
    let session = raw_session(&shell.config.engine_url, &token, "2025-06-18")
        .await
        .unwrap();
    let prefix = &shell.context.meta_prefix;
    let response = reqwest::Client::new().post(&shell.config.engine_url).bearer_auth(&token)
        .header("accept", "application/json, text/event-stream")
        .header("mcp-session-id", session)
        .header("mcp-protocol-version", "2025-06-18")
        .json(&json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"management.list.v1","arguments":{},"_meta":{format!("{prefix}serving"):"","io.modelcontextprotocol/clientCapabilities":{"extensions":{format!("{prefix}implementation-interface"): {"revision":"v0"}}}}}}))
        .send().await.unwrap();
    let text = response.text().await.unwrap();
    let value: Value = text
        .lines()
        .filter_map(|line| line.strip_prefix("data:"))
        .filter_map(|body| serde_json::from_str::<Value>(body.trim()).ok())
        .find(|value| value["id"] == 2)
        .or_else(|| serde_json::from_str(&text).ok())
        .expect("protocol rejection must contain a JSON reply");
    assert_eq!(value["error"]["code"], -32602);
    assert_eq!(
        value["error"]["data"]["diagnostics"][0]["ruleId"],
        "IFC-036"
    );
    stop.cancel();
    tokio::time::timeout(Duration::from_secs(60), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sign_in_return_not_taken_reports_invalid_over_mcp() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    server::preflight(&shell).unwrap();
    let root = actor(&shell, "root");
    shell.issue_client_token(&root).unwrap();
    let token = token(&shell, "root");
    let executor = Arc::new(ServingExecutor::new(Arc::new(crate::MissingExecutor)));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
    ready(&shell.config.engine_url).await;
    let session = raw_session(&shell.config.engine_url, &token, "2025-06-18")
        .await
        .unwrap();
    let prefix = &shell.context.meta_prefix;
    let key = format!("{prefix}signInReturn");
    let value = "http://127.0.0.1:49152/callback?code=not-a-code&state=not-a-state";
    let response = reqwest::Client::new()
        .post(&shell.config.engine_url)
        .bearer_auth(&token)
        .header("accept", "application/json, text/event-stream")
        .header("mcp-session-id", session)
        .header("mcp-protocol-version", "2025-06-18")
        .json(&json!({
            "jsonrpc":"2.0","id":2,"method":"tools/call",
            "params":{
                "name":"management.inspect.v1",
                "arguments":{"object":{"kind":"Interface","name":"root"}},
                "_meta":{
                    "io.modelcontextprotocol/clientCapabilities":{"extensions":{format!("{prefix}implementation-interface"):{"revision":"v0"}}},
                    &key:{"value":value,"via":"paste"}
                }
            }
        }))
        .send()
        .await
        .unwrap();
    let text = response.text().await.unwrap();
    let reply: Value = text
        .lines()
        .filter_map(|line| line.strip_prefix("data:"))
        .filter_map(|body| serde_json::from_str::<Value>(body.trim()).ok())
        .find(|value| value["id"] == 2)
        .or_else(|| serde_json::from_str(&text).ok())
        .expect("tool call must contain a JSON reply");
    let result = &reply["result"];
    assert_eq!(result["isError"], true);
    assert_eq!(result["_meta"][format!("{prefix}reason")], "invalid");
    let diagnostics = result["structuredContent"]["diagnostics"]
        .as_array()
        .unwrap();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0]["ruleId"], "PND-013");
    assert!(
        diagnostics[0]["message"]["text"]
            .as_str()
            .unwrap()
            .contains(&key)
    );
    let state = shell.writer.lock().unwrap().store.read().unwrap().state;
    let records = serde_json::to_string(&state.records).unwrap();
    for withheld in [value, "not-a-code", "not-a-state"] {
        assert!(!text.contains(withheld));
        assert!(!records.contains(withheld));
    }
    stop.cancel();
    tokio::time::timeout(Duration::from_secs(60), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn forged_retry_state_is_refused_and_recorded() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
    server::preflight(&shell).unwrap();
    let root = actor(&shell, "root");
    shell.issue_client_token(&root).unwrap();
    let token = token(&shell, "root");
    let executor = Arc::new(ServingExecutor::new(Arc::new(crate::MissingExecutor)));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
    ready(&shell.config.engine_url).await;
    let client = modern(&shell.config.engine_url, &token).await;
    let prefix = &shell.context.meta_prefix;
    let ending_reasons = || {
        let state = shell.writer.lock().unwrap().store.read().unwrap().state;
        state
            .records
            .iter()
            .filter(|record| record["type"] == "request-ended")
            .map(|record| record["data"]["reason"].clone())
            .collect::<Vec<_>>()
    };
    let before = ending_reasons().len();
    for (with_responses, text, added) in [
        (
            true,
            "requestState is not one this engine issued to this Interface: it was altered, issued to another Interface, or forgotten at a restart",
            1,
        ),
        (
            false,
            "a retry carries both requestState and inputResponses",
            2,
        ),
    ] {
        let mut request = serde_json::to_value(call("management.list.v1", json!({}))).unwrap();
        request["requestState"] = json!("forged");
        if with_responses {
            request["inputResponses"] = json!({"url":{"action":"accept"}});
        }
        let error = match client
            .call_tool_once(serde_json::from_value(request).unwrap())
            .await
        {
            Err(rmcp::ServiceError::McpError(error)) => error,
            Ok(_) => panic!("retry succeeded as a new request; expected an IFC-049 error"),
            Err(_) => panic!("expected an IFC-049 protocol error"),
        };
        assert_eq!(error.code, ErrorCode::INVALID_PARAMS);
        assert_eq!(error.message, text);
        let data = error.data.unwrap();
        assert_eq!(data[format!("{prefix}reason")], "invalid");
        assert_eq!(data["diagnostics"][0]["ruleId"], "IFC-049");
        assert_eq!(data["diagnostics"][0]["message"]["text"], text);
        let reasons = ending_reasons();
        assert_eq!(reasons.len(), before + added);
        assert_eq!(reasons.last().unwrap(), "invalid");
    }
    let result = client
        .call_tool(call("management.list.v1", json!({})))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(false));
    client.cancel().await.unwrap();
    stop.cancel();
    tokio::time::timeout(Duration::from_secs(60), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

#[test]
fn admission_asks_kept_across_a_reopen() {
    let temp = temporary();
    let shell = established(temp.path());
    let read = |path: &Path| -> Value {
        serde_json::from_slice(&fs::read(path.join("state.json")).unwrap()).unwrap()
    };
    assert!(read(temp.path())["state"].get("admission_asks").is_none());
    shell.submit(json!({"kind":"invoke","request":{
        "identity":{"kind":"instance","id":actor(&shell,"root")},
        "tool":"management.inspect.v1","arguments":{"object":{"kind":"Interface","name":"root"}}
    }}), &MissingExecutor).unwrap();
    assert!(read(temp.path())["state"].get("admission_asks").is_none());
    assert_memory_equals_disk(&shell);
    drop(shell);

    let temp = temporary();
    let case = repository().join("conformance/conventions/requests/admission-report-kept");
    let shell = established_case(temp.path(), &case, 15);
    let ticket = shell
        .begin(
            json!({"kind":"invoke","request":{
                "identity":{"kind":"instance","id":actor(&shell,"root")},
                "tool":"call-a.turn.v1","arguments":{}
            }}),
            &Deferred,
        )
        .unwrap();
    assert!(ticket.try_reply().unwrap().is_none());
    let disk = read(temp.path());
    assert_eq!(disk["version"], 1);
    let login = shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| o["kind"] == "Resource" && o["name"] == "login")
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .to_owned();
    assert!(
        disk["state"]["admission_asks"][&login].is_object(),
        "store must keep the login's admission ask"
    );
    let ask = disk["state"]["admission_asks"][&login]["invocation"].clone();
    assert!(ask.is_string());
    assert!(
        disk["state"]["invocations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v["id"] == ask && v["state"] == "in-progress")
    );
    assert_memory_equals_disk(&shell);
    drop(ticket);
    drop(shell);
    let shell = open(temp.path());
    let first = read(temp.path())["state"]["admission_asks"].clone();
    assert!(
        first
            .as_object()
            .is_none_or(|asks| asks.values().all(|v| v["invocation"].is_null()))
    );
    assert!(
        shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .all(|v| v["id"] != ask || v["state"] != "in-progress")
    );
    assert_memory_equals_disk(&shell);
    drop(shell);
    let shell = open(temp.path());
    assert_eq!(read(temp.path())["state"]["admission_asks"], first);
    assert_memory_equals_disk(&shell);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn admission_ask_ends_at_its_deadline() {
    assert_eq!(
        Config::default().admission_ask_timeout,
        Duration::from_secs(30)
    );
    let temp = temporary();
    let case = repository().join("conformance/conventions/requests/admission-report-kept");
    drop(established_case(temp.path(), &case, 15));
    let shell = Arc::new(fixed_endpoint_shell_with_config(
        temp.path(),
        Config {
            admission_ask_timeout: Duration::from_secs(1),
            ..Config::default()
        },
    ));
    let token = sign_in_root_token(&shell, temp.path());
    let executor = Arc::new(ServingExecutor::new(Arc::new(Deferred)));
    let stop = CancellationToken::new();
    let task = tokio::spawn(server::serve(shell.clone(), executor, stop.clone()));
    ready(&shell.config.engine_url).await;
    let client = modern(&shell.config.engine_url, &token).await;
    *shell.trace.lock().unwrap() = Some(Vec::new());
    let peer = client.peer().clone();
    let call_task =
        tokio::spawn(async move { peer.call_tool(call("call-a.turn.v1", json!({}))).await });
    let ask = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let ended = shell
                .trace
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .iter()
                .flat_map(|t| &t.decisions)
                .find(|d| d.action == "admission-ask-deadline" && d.facts["ended"] == true)
                .map(|d| d.facts["invocation"].clone());
            if let Some(id) = ended {
                break id;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the serving timer must end the admission ask");
    assert!(
        shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .any(|v| v["id"] == ask && v["state"] == "unknown")
    );
    let inspected = client
        .call_tool(call(
            "logins.inspect.v1",
            json!({"object":{"kind":"Resource","name":"login"}}),
        ))
        .await
        .unwrap();
    let inspected = serde_json::to_value(inspected).unwrap();
    assert_eq!(
        inspected["structuredContent"]["status"]["admission"]["noReport"]["cause"],
        "deadline"
    );
    call_task.abort();
    let _ = call_task.await;
    client.cancel().await.unwrap();
    stop.cancel();
    task.await.unwrap().unwrap();

    struct ObservedAsks(Mutex<Vec<Value>>);
    impl Executor for ObservedAsks {
        fn dispatch(&self, _: &Value) -> Result<Execution> {
            Ok(Execution::Deferred)
        }
        fn refresh_tools_and_call(&self, _: &Value) -> Result<Execution> {
            Ok(Execution::Deferred)
        }
        fn admission_asks_started(&self, asks: &[Value]) -> Result<()> {
            self.0.lock().unwrap().extend_from_slice(asks);
            Ok(())
        }
    }
    let temp = temporary();
    let case = repository().join("conformance/conventions/requests/admission-single-use");
    let shell = established_case(temp.path(), &case, 32);
    let state = shell.state().unwrap();
    let login = state.objects.iter().find(|v| v["name"] == "login").unwrap()["id"]
        .as_str()
        .unwrap();
    let ask = state.admission_asks[login].invocation.clone();
    let vector = boundary::value(&case.join("case.yaml")).unwrap();
    let executor = ObservedAsks(Mutex::new(Vec::new()));
    shell
        .complete(
            &ask,
            1,
            vector["steps"][32]["input"]["completion"].clone(),
            &executor,
        )
        .unwrap();
    let observed = executor.0.lock().unwrap();
    assert_eq!(observed.len(), 1);
    assert_eq!(
        observed[0]["invocation"],
        shell.state().unwrap().admission_asks[login].invocation
    );
    assert_eq!(observed[0]["attempt"], 1);
    assert_eq!(
        observed[0]["startedAt"],
        shell.state().unwrap().admission_asks[login].started_at
    );
}
