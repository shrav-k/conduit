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
use std::sync::atomic::AtomicBool;
use tokio_util::sync::CancellationToken;

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
    Consumer { changes }
        .serve(StreamableHttpClientTransport::with_client(
            reqwest::Client::new(),
            StreamableHttpClientTransportConfig::with_uri(url.to_owned()).auth_header(token),
        ))
        .await
        .unwrap()
}
async fn modern(url: &str, token: &str) -> RunningService<RoleClient, ()> {
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
    let shell = established(path);
    drop(shell);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    // The fixture creates no run tokens. Give this independent test store a fixed
    // endpoint before opening the server; production cannot rewrite its endpoint.
    let config = Config {
        engine_url: format!("http://127.0.0.1:{port}/mcp"),
        ..Config::default()
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
    for _ in 0..6000 {
        if reqwest::get(url).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("server did not bind");
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
    tokio::time::timeout(Duration::from_secs(60), async {
        while counter.load(Ordering::SeqCst) < minimum {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn s1_http_consumers_authority_records_and_private_t7_changes() {
    let temp = temporary();
    let shell = Arc::new(fixed_shell(temp.path()));
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
    let conduit = shell
        .state()
        .unwrap()
        .objects
        .into_iter()
        .find(|o| {
            o["kind"] == "Conduit"
                && reference_value(&o["consumer"]) == &json!(actor(&shell, "agent-a"))
                && o["nested"]["name"] == "project"
        })
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
        tokio::time::timeout(Duration::from_secs(60), subscription_a.next())
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
        tokio::time::timeout(Duration::from_secs(60), subscription_a.next())
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
        serde_json::to_value(shell.state().unwrap()).unwrap(),
        serde_json::to_value(before).unwrap()
    );
    drop(subscription_a);
    drop(subscription_b);
    modern_a.cancel().await.unwrap();
    modern_b.cancel().await.unwrap();
    client_a.cancel().await.unwrap();
    client_b.cancel().await.unwrap();
    stop.cancel();
    tokio::time::timeout(Duration::from_secs(60), task)
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
    let invocation = shell
        .state()
        .unwrap()
        .invocations
        .into_iter()
        .find(|i| i["state"] == "in-progress")
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
    tokio::time::timeout(Duration::from_secs(60), async {
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
    tokio::time::timeout(Duration::from_secs(60), task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}

async fn raw_session(url: &str, token: &str, revision: &str) -> Option<String> {
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
    let parent = shell
        .state()
        .unwrap()
        .invocations
        .into_iter()
        .find(|v| v["id"] == invocation)
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
        tokio::time::timeout(Duration::from_secs(60), async {
            while let Some(chunk) = stream.next().await {
                chunk.unwrap();
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
        tokio::time::timeout(Duration::from_secs(60), async {
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
        tokio::time::timeout(Duration::from_secs(60), async {
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
        tokio::time::timeout(Duration::from_secs(30), async {
            while let Some(chunk) = stream.next().await {
                chunk.unwrap();
            }
        })
        .await
        .unwrap();
        tokio::time::timeout(Duration::from_secs(30), async {
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
