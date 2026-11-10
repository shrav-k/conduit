//! Process coverage for the joined HTTP adapter and unchanged npm filesystem runner.
use conduit_shell::{Config, MissingExecutor, Shell, boundary};
use rmcp::{
    ClientHandler, RoleClient, ServiceExt,
    model::{CallToolRequestParams, CallToolResult, ClientConfig, ProtocolVersion},
    service::{NotificationContext, RunningService},
    transport::{
        StreamableHttpClientTransport, streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
#[derive(Clone)]
struct Consumer(Arc<AtomicUsize>);
impl ClientHandler for Consumer {
    fn get_info(&self) -> ClientConfig {
        let mut config = ClientConfig::default();
        config.protocol_version = ProtocolVersion::V_2025_06_18;
        config
    }
    async fn on_tool_list_changed(&self, _: NotificationContext<RoleClient>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}
async fn connect(
    url: &str,
    token: &str,
    changes: Arc<AtomicUsize>,
) -> RunningService<RoleClient, Consumer> {
    Consumer(changes)
        .serve(StreamableHttpClientTransport::with_client(
            reqwest::Client::new(),
            StreamableHttpClientTransportConfig::with_uri(url.to_owned()).auth_header(token),
        ))
        .await
        .unwrap()
}
fn state(store: &Path) -> Value {
    serde_json::from_slice::<Value>(&fs::read(store.join("state.json")).unwrap()).unwrap()["state"]
        .clone()
}
fn cli(store: &Path, verb: &str, args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .arg(verb)
        .arg("--store")
        .arg(store)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reply: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_ne!(reply["isError"], true, "{reply}");
    reply
}
fn call(name: &str, arguments: Value) -> CallToolRequestParams {
    serde_json::from_value(json!({"name":name,"arguments":arguments})).unwrap()
}
fn durable(store: &Path, reply: &CallToolResult, prefix: &str) {
    assert_eq!(reply.is_error, Some(false));
    let reply = serde_json::to_value(reply).unwrap();
    let id = &reply["_meta"][format!("{prefix}invocation")];
    assert!(id.is_string());
    let records = state(store)["records"].as_array().unwrap().clone();
    for kind in ["request-ended", "invocation-ended"] {
        assert_eq!(
            records
                .iter()
                .filter(|r| r["type"] == kind && r["data"]["invocation"] == *id)
                .count(),
            1
        );
    }
    let position = |kind: &str| {
        records
            .iter()
            .position(|r| r["type"] == kind && r["data"]["invocation"] == *id)
            .unwrap()
    };
    assert!(position("access-decision") < position("invocation-dispatched"));
    assert!(position("invocation-dispatched") < position("invocation-ended"));
}
async fn changed(changes: &AtomicUsize, target: usize) {
    tokio::time::timeout(Duration::from_secs(60), async {
        while changes.load(Ordering::SeqCst) < target {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
}

#[test]
fn real_npm_serve_s1_tokens_access_t7_and_shutdown() {
    if std::env::var_os("CONDUIT_TEST_NPM").is_none() {
        return;
    }
    let parent = tempfile::tempdir().unwrap();
    let store = parent.path().join("store");
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let url = format!("http://127.0.0.1:{port}/mcp");
    let shell = Shell::open(
        &store,
        boundary::context(&repo).unwrap(),
        Config {
            engine_url: url.clone(),
            ..Config::default()
        },
    )
    .unwrap();
    shell.submit(json!({"kind":"first-start","machine":conduit_shell::local_machine(BTreeMap::new()).unwrap()}), &MissingExecutor).unwrap();
    let root = shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| o["name"] == "root")
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let spec = conduit_loader::Specification::read(&repo.join("spec")).unwrap();
    let mut project =
        boundary::project(&repo.join("conformance/fixtures/s1/project"), &spec).unwrap();
    for name in ["shared", "history"] {
        fs::create_dir(parent.path().join(name)).unwrap();
    }
    for doc in &mut project.documents {
        if doc["kind"] == "Resource" {
            doc["arguments"]["root"] = json!(
                parent
                    .path()
                    .join(doc["metadata"]["name"].as_str().unwrap())
            );
        }
    }
    let current = shell.state().unwrap();
    let route = current
        .objects
        .iter()
        .find(|o| o["consumer"] == root && o["local_name"] == "management")
        .unwrap();
    let registered = shell.submit(json!({"kind":"register-project","snapshots":boundary::s1_snapshots(&repo).unwrap(),"builtin":shell.context.builtin,"project":project,"request":{"sender":root,"caller":root,"conduit":route["id"],"target":route["target"],"operation":format!("{}#register.v1",shell.context.core_package)}}), &MissingExecutor).unwrap();
    assert_eq!(registered["accepted"], true, "{registered}");
    let opened = shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":"access.open.v1","arguments":{"conduit":"root-workspace"}}}), &MissingExecutor).unwrap();
    assert_eq!(opened["isError"], false, "{opened}");
    // Base S1 Agents have no executable (order 023 supplies those). Materialize their
    // run identities with the same contract/API the runner uses, without starting models.
    let mut tokens = Vec::new();
    for name in ["agent-a", "agent-b"] {
        let current = shell.state().unwrap();
        let id = current
            .objects
            .iter()
            .find(|o| o["name"] == name && o.get("spec").is_none())
            .unwrap()["id"]
            .as_str()
            .unwrap();
        let identity_schema = &shell.context.core_environment.scopes["core"].contracts["run.v1"]["spec"]
            ["inputSchema"]["properties"]["identity"]["properties"];
        let descriptor = json!({"materializeFor":id,"environmentNames":identity_schema["environment"]["required"],"fileNames":identity_schema["files"]["required"]});
        let identity = shell.materialize_identity(&descriptor).unwrap();
        let token_name = identity_schema["files"]["required"][3].as_str().unwrap();
        tokens.push(fs::read_to_string(identity.directory.join(token_name)).unwrap());
    }
    let prefix = shell.context.meta_prefix.clone();
    drop(shell);
    let mut server = start_server(&store, port);
    for name in ["shared", "history", "agent-a", "agent-b"] {
        assert!(
            state(&store)["objects"]
                .as_array()
                .unwrap()
                .iter()
                .any(|o| o["name"] == name && o["state"] == "ready")
        );
    }
    let groups = run_groups(&store, &server);
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let changes_a = Arc::new(AtomicUsize::new(0));
        let changes_b = Arc::new(AtomicUsize::new(0));
        let a = connect(&url, &tokens[0], changes_a.clone()).await;
        let b = connect(&url, &tokens[1], changes_b.clone()).await;
        let list_a = a.list_tools(None).await.unwrap().tools;
        let list_b = b.list_tools(None).await.unwrap().tools;
        let mut names: Vec<_> = list_a.iter().map(|t| t.name.as_ref()).collect(); names.sort();
        assert_eq!(names, ["history.directory.list.v1", "history.file.read.v1", "history.file.write.v1", "project.directory.list.v1", "project.file.read.v1"]);
        assert_eq!(list_b.len(), 6);
        for name in ["agent-a", "agent-b"] {
            let helpers = cli(&store, "tools", &["--as", name]);
            assert_eq!(helpers["tools"], serde_json::to_value(if name == "agent-a" { &list_a } else { &list_b }).unwrap());
        }
        let path = parent.path().join("shared/hello.txt");
        let refused = a.call_tool(call("project.file.write.v1", json!({"path":path,"content":"forbidden"}))).await.unwrap_err();
        assert!(refused.to_string().contains("refused")); assert!(!path.exists());
        assert_eq!(state(&store)["records"].as_array().unwrap().last().unwrap()["data"]["reason"], "refused");
        let written = b.call_tool(call("project.file.write.v1", json!({"path":path,"content":"B wrote"}))).await.unwrap();
        durable(&store, &written, &prefix);
        assert_eq!(fs::read_to_string(&path).unwrap(), "B wrote");
        let read = b.call_tool(call("project.file.read.v1", json!({"path":path}))).await.unwrap();
        durable(&store, &read, &prefix);
        assert_eq!(serde_json::to_value(read).unwrap()["content"][0]["text"], "B wrote");
        for token in [None, Some("forged")] {
            let mut request = reqwest::Client::new().post(&url).json(&json!({"jsonrpc":"2.0","id":90,"method":"tools/call","params":{"name":"project.file.write.v1","arguments":{"path":path,"content":"intruder"}}}));
            if let Some(token) = token { request = request.bearer_auth(token); }
            let response = request.send().await.unwrap();
            assert_eq!(response.status(), 401);
            assert!(response.headers().contains_key("www-authenticate"));
            assert_eq!(state(&store)["records"].as_array().unwrap().last().unwrap()["data"]["reason"], "refused");
            assert_eq!(fs::read_to_string(&path).unwrap(), "B wrote");
        }
        let current = state(&store);
        let a_id = &current["objects"].as_array().unwrap().iter().find(|o| o["name"] == "agent-a" && o.get("spec").is_none()).unwrap()["id"];
        let conduit = current["objects"].as_array().unwrap().iter().find(|o| o["kind"] == "Conduit" && o["consumer"].get("id").unwrap_or(&o["consumer"]) == a_id && o["nested"]["name"] == "project").unwrap();
        cli(&store, "call", &["access.close.v1", &json!({"conduit":conduit["name"]}).to_string()]);
        changed(&changes_a, 1).await;
        assert_eq!(a.list_tools(None).await.unwrap().tools.len(), 3);
        assert_eq!(b.list_tools(None).await.unwrap().tools, list_b);
        assert!(a.call_tool(call("project.file.read.v1", json!({"path":path}))).await.unwrap_err().to_string().contains("refused"));
        let written = b.call_tool(call("project.file.write.v1", json!({"path":path,"content":"B works while A is closed"}))).await.unwrap();
        durable(&store, &written, &prefix);
        assert_eq!(fs::read_to_string(&path).unwrap(), "B works while A is closed");
        cli(&store, "call", &["management.register-replacement.v1", r#"{"instance":{"kind":"Agent","name":"agent-a"},"nested":"project"}"#]);
        cli(&store, "call", &["management.set-up.v1", r#"{"instance":{"kind":"Agent","name":"agent-a"}}"#]);
        changed(&changes_a, 2).await;
        assert_eq!(a.list_tools(None).await.unwrap().tools, list_a);
        let read = a.call_tool(call("project.file.read.v1", json!({"path":path}))).await.unwrap(); durable(&store, &read, &prefix);
        let written = b.call_tool(call("project.file.write.v1", json!({"path":path,"content":"B unaffected"}))).await.unwrap(); durable(&store, &written, &prefix);
        assert_eq!(fs::read_to_string(&path).unwrap(), "B unaffected");
        assert_eq!(b.list_tools(None).await.unwrap().tools, list_b);
        tokio::time::sleep(Duration::from_millis(300)).await;
        assert_eq!(changes_b.load(Ordering::SeqCst), 0);
        a.cancel().await.unwrap(); b.cancel().await.unwrap();
    });
    stop_server(&mut server, &store, &groups);
    // A persisted ready state has no live stdio sessions after shutdown. Serving
    // the same store again must inspect and restart both runs, as after register.
    let mut server = start_server(&store, port);
    let groups = run_groups(&store, &server);
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let b = connect(&url, &tokens[1], Arc::default()).await;
        let read = b
            .call_tool(call(
                "project.file.read.v1",
                json!({"path":parent.path().join("shared/hello.txt")}),
            ))
            .await
            .unwrap();
        durable(&store, &read, &prefix);
        assert_eq!(
            serde_json::to_value(read).unwrap()["content"][0]["text"],
            "B unaffected"
        );
        b.cancel().await.unwrap();
    });
    stop_server(&mut server, &store, &groups);
}
fn start_server(store: &Path, port: u16) -> Server {
    let stderr = fs::File::create(store.join("serve.stderr")).unwrap();
    let mut server = Server(
        Command::new(env!("CARGO_BIN_EXE_conduit"))
            .arg("serve")
            .arg("--store")
            .arg(store)
            .stdout(Stdio::null())
            .stderr(stderr)
            .spawn()
            .unwrap(),
    );
    let start = Instant::now();
    while std::net::TcpStream::connect(("127.0.0.1", port)).is_err() {
        assert!(
            server.0.try_wait().unwrap().is_none(),
            "server exited: {}",
            fs::read_to_string(store.join("serve.stderr")).unwrap()
        );
        assert!(
            start.elapsed() < Duration::from_secs(180),
            "server startup timed out"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    server
}
fn run_groups(store: &Path, server: &Server) -> Vec<i32> {
    // Read the runner's private historical group diagnostics, avoiding ps (which
    // the worker sandbox disallows). Check the OS, not just persisted core status.
    let groups: Vec<i32> = fs::read_dir(store.join("run-groups"))
        .unwrap()
        .filter_map(|entry| {
            let value: Value =
                serde_json::from_slice(&fs::read(entry.unwrap().path()).unwrap()).unwrap();
            if value["engine"] != server.0.id() {
                return None;
            }
            let group = value["group"].as_i64().unwrap() as i32;
            assert_eq!(
                unsafe { libc::kill(-group, 0) },
                0,
                "run group must be live"
            );
            Some(group)
        })
        .collect();
    assert_eq!(groups.len(), 2, "two real filesystem groups must be live");
    groups
}
fn stop_server(server: &mut Server, store: &Path, groups: &[i32]) {
    assert!(
        Command::new("kill")
            .args(["-TERM", &server.0.id().to_string()])
            .status()
            .unwrap()
            .success()
    );
    let start = Instant::now();
    loop {
        if let Some(status) = server.0.try_wait().unwrap() {
            assert!(status.success());
            break;
        }
        assert!(start.elapsed() < Duration::from_secs(60));
        std::thread::sleep(Duration::from_millis(20));
    }
    for &group in groups {
        let start = Instant::now();
        while unsafe { libc::kill(-group, 0) } == 0 {
            assert!(
                start.elapsed() < Duration::from_secs(30),
                "run group survived serve"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ESRCH)
        );
    }
    assert!(
        state(store)["invocations"]
            .as_array()
            .unwrap()
            .iter()
            .all(|i| i["state"] != "in-progress")
    );
}
