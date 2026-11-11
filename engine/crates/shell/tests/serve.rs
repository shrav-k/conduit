//! Process coverage for the joined HTTP adapter and unchanged npm filesystem runner.
mod common;
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
    let replies: Vec<Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    for reply in &replies {
        assert_ne!(reply["isError"], true, "{reply}");
    }
    replies.last().unwrap().clone()
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
    common::install(
        &shell,
        &repo,
        common::snapshots(&repo).unwrap(),
        &mut project,
    );
    let registered = shell.submit(json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,"project":project,"request":{"sender":root,"caller":root,"conduit":route["id"],"target":route["target"],"operation":format!("{}#register.v1",shell.context.core_package)}}), &MissingExecutor).unwrap();
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
        // T5/T6: install a new package version while serving, then submit a new
        // Resource rooted elsewhere. Existing instances keep their old package.
        let archive=repo.join("conformance/fixtures/s1/archives/filesystem-0.2.0.tar.gz");
        let digest=conduit_core::packages::bytes_digest(&fs::read(&archive).unwrap());
        cli(&store,"install",&[&format!("{}#{digest}",archive.display())]);
        let upgrade=parent.path().join("upgrade"); fs::create_dir(&upgrade).unwrap();
        let new_root=parent.path().join("another-shared"); fs::create_dir(&new_root).unwrap();
        fs::write(new_root.join("hello.txt"),"read through 2.2.0").unwrap();
        let spec=conduit_loader::Specification::read(&repo.join("spec")).unwrap();
        let base=boundary::project(&repo.join("conformance/fixtures/s1/project"),&spec).unwrap();
        let mut manifest=base.manifest.unwrap();
        manifest["packages"]["example.org/s1/filesystem"]["version"]=json!("0.2.0");
        manifest["packages"]["example.org/s1/filesystem"]["digest"]=json!(digest);
        let mut resource=base.documents.iter().find(|d| d["kind"]=="Resource" && d["metadata"]["name"]=="shared-directory").unwrap().clone();
        resource["metadata"]["name"]=json!("upgrade-directory");
        let mut access=base.documents.iter().find(|d| d["kind"]=="Conduit" && d["metadata"]["name"]=="directory-read-write").unwrap().clone();
        access["metadata"]["name"]=json!("upgrade-access");
        let api=spec.names().api_version.clone();
        let docs=json!([resource,access,
            {"apiVersion":api,"kind":"Resource","metadata":{"name":"shared-upgraded"},"from":"upgrade-directory","owner":"workspace","arguments":{"root":new_root}},
            {"apiVersion":api,"kind":"Conduit","metadata":{"name":"upgraded"},"from":"upgrade-access","consumer":{"kind":"Interface","name":"root"},"target":{"kind":"Resource","name":"shared-upgraded"}}
        ]);
        fs::write(upgrade.join("conduit.json"),serde_json::to_vec(&manifest).unwrap()).unwrap();
        fs::write(upgrade.join("documents.json"),serde_json::to_vec(&docs).unwrap()).unwrap();
        cli(&store,"submit",&[upgrade.to_str().unwrap()]);
        let list=cli(&store,"tools",&[]);
        let names: Vec<_>=list["tools"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(names.iter().filter(|n| n.starts_with("upgraded.")).count(),3);
        assert!(names.iter().all(|n| !n.contains("append_file") && !n.contains("copy_file")));
        let read=cli(&store,"call",&["upgraded.file.read.v1",&json!({"path":new_root.join("hello.txt")}).to_string()]);
        assert_eq!(read["content"][0]["text"],"read through 2.2.0");
        for (name,version) in [("shared","0.1.0"),("shared-upgraded","0.2.0")] {
            assert!(state(&store)["objects"].as_array().unwrap().iter().find(|o| o["name"]==name).unwrap()["packages"].as_array().unwrap().iter().any(|p| p["address"]=="example.org/s1/filesystem" && p["version"]==version));
        }
        let upgraded_group: Vec<_>=fs::read_dir(store.join("run-groups")).unwrap().map(|e| serde_json::from_slice::<Value>(&fs::read(e.unwrap().path()).unwrap()).unwrap()).filter(|v| v["engine"]==server.0.id()).map(|v| v["group"].as_i64().unwrap() as i32).filter(|g| !groups.contains(g)).collect();
        assert_eq!(upgraded_group.len(),1);
        cli(&store,"retire",&["Resource/shared-upgraded"]);
        let start=Instant::now();
        while unsafe { libc::kill(-upgraded_group[0],0) } == 0 { assert!(start.elapsed()<Duration::from_secs(30)); std::thread::sleep(Duration::from_millis(10)); }
        assert!(state(&store)["objects"].as_array().unwrap().iter().all(|o| o["name"]!="shared-upgraded" && o["name"]!="upgraded"));
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
    let output = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args(["inspect", "--store"])
        .arg(store)
        .output()
        .unwrap();
    let mut stopped = 0;
    for line in String::from_utf8(output.stdout).unwrap().lines() {
        let row: Value = serde_json::from_str(line).unwrap();
        let Some(past) = row["pastConditions"].as_array() else {
            continue;
        };
        let Some(ended) = past
            .iter()
            .rev()
            .find(|p| p["condition"]["reason"] == "RunRunning")
        else {
            continue;
        };
        let current = row["result"]["structuredContent"]["status"]["conditions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["type"] == "Running")
            .unwrap();
        assert_eq!(current["reason"], "RunStopped");
        assert_eq!(current["status"], "False");
        assert_eq!(ended["endedAt"], current["lastTransitionTime"]);
        assert!(ended["endedAt"].is_string());
        stopped += 1;
    }
    assert_eq!(stopped, groups.len());
}

#[test]
fn submit_refuses_relative_values_before_sending_registration() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    cli(
        &store,
        "init",
        &["--url", &format!("http://127.0.0.1:{port}/mcp")],
    );
    let mut server = start_server(&store, port);
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let api = conduit_loader::Specification::read(&repo.join("spec"))
        .unwrap()
        .names()
        .api_version
        .clone();
    let core = boundary::context(&repo).unwrap().core_package;
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();
    let doc = json!({"apiVersion":api,"kind":"Pool","metadata":{"name":"relative-template"},"spec":{"type":format!("{core}#pool.v1")}});
    let manifest = json!({"apiVersion":api,"address":"example.org/test/relative-submit"});
    let before = state(&store);
    for location in [
        "manifest-fetch",
        "manifest-program",
        "document-fetch",
        "document-program",
    ] {
        let mut manifest = manifest.clone();
        let mut doc = doc.clone();
        match location {
            "manifest-fetch" => {
                manifest["packages"] = json!({"example.org/test/dependency":{"version":"0.1.0","digest":format!("sha256:{}","0".repeat(64)),"fetch":"../dependency.tar.gz"}})
            }
            "manifest-program" => {
                manifest["implementations"] = json!({"local":{"implements":[format!("{core}#retire-hook.v1")],"command":["./program"]}})
            }
            "document-fetch" => doc["spec"]["config"] = json!({"fetch":"./dependency.tar.gz"}),
            "document-program" => doc["spec"]["config"] = json!({"command":["../program"]}),
            _ => unreachable!(),
        }
        fs::write(
            project.join("conduit.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
        fs::write(
            project.join("documents.json"),
            serde_json::to_vec(&doc).unwrap(),
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_conduit"))
            .args(["submit", "--store"])
            .arg(&store)
            .arg(&project)
            .output()
            .unwrap();
        assert!(!output.status.success(), "{location}");
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains("a submission through the serving engine takes absolute values only"),
            "{location}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stdout.is_empty(), "{location}");
        assert_eq!(state(&store), before, "{location}: request reached engine");
    }
    for (name, program) in [
        ("absolute-program", "/test/program"),
        ("path-program", "program"),
    ] {
        let mut manifest = manifest.clone();
        let mut doc = doc.clone();
        manifest["address"] = json!(format!("example.org/test/{name}"));
        manifest["implementations"] =
            json!({"local":{"implements":[format!("{core}#retire-hook.v1")],"command":[program]}});
        doc["metadata"]["name"] = json!(name);
        fs::write(
            project.join("conduit.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
        fs::write(
            project.join("documents.json"),
            serde_json::to_vec(&doc).unwrap(),
        )
        .unwrap();
        cli(&store, "submit", &[project.to_str().unwrap()]);
        assert!(
            state(&store)["objects"]
                .as_array()
                .unwrap()
                .iter()
                .any(|o| o["name"] == name)
        );
    }
    stop_server(&mut server, &store, &[]);
}

#[test]
fn author_commands_require_serve_and_make_recorded_requests() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    cli(
        &store,
        "init",
        &["--url", &format!("http://127.0.0.1:{port}/mcp")],
    );
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let api = conduit_loader::Specification::read(&repo.join("spec"))
        .unwrap()
        .names()
        .api_version
        .clone();
    let core = boundary::context(&repo).unwrap().core_package;
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();
    let mut documents = json!([
        {"apiVersion":api,"kind":"Pool","metadata":{"name":"change-pool"},"spec":{"type":format!("{core}#pool.v1")}},
        {"apiVersion":api,"kind":"Pool","metadata":{"name":"new-pool"},"from":"change-pool"},
        {"apiVersion":api,"kind":"Conduit","metadata":{"name":"new-admin"},"spec":{"capability":format!("{core}#pool-management.v1"),"operations":[format!("{core}#set-up.v1"),format!("{core}#retire.v1"),format!("{core}#inspect.v1")]}},
        {"apiVersion":api,"kind":"Conduit","metadata":{"name":"new-route"},"from":"new-admin","consumer":{"kind":"Interface","name":"root"},"target":{"kind":"Pool","name":"new-pool"}}
    ]);
    fs::write(
        project.join("documents.json"),
        serde_json::to_vec(&documents).unwrap(),
    )
    .unwrap();
    let archive = repo.join("conformance/fixtures/s1/archives/filesystem-0.2.0.tar.gz");
    let digest = conduit_core::packages::bytes_digest(&fs::read(&archive).unwrap());
    let package = format!("{}#{digest}", archive.display());
    for (verb, args) in [
        ("install", vec![package.as_str()]),
        ("submit", vec![project.to_str().unwrap()]),
        (
            "replace",
            vec![project.to_str().unwrap(), "Pool/change-pool"],
        ),
        ("retire", vec!["Pool/new-pool"]),
        ("unregister", vec!["Pool/change-pool"]),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_conduit"))
            .arg(verb)
            .arg("--store")
            .arg(&store)
            .args(args)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("conduit serve must hold this store")
        );
    }
    let mut server = start_server(&store, port);
    cli(&store, "install", &[&package]);
    cli(&store, "submit", &[project.to_str().unwrap()]);
    assert!(
        state(&store)["objects"]
            .as_array()
            .unwrap()
            .iter()
            .any(|o| o["name"] == "new-pool" && o["state"] == "ready")
    );
    let prior = state(&store)["objects"]
        .as_array()
        .unwrap()
        .iter()
        .find(|o| o["name"] == "change-pool")
        .unwrap()["revision"]
        .clone();
    documents[0]["metadata"]["description"] = json!("changed by author command");
    fs::write(
        project.join("documents.json"),
        serde_json::to_vec(&documents).unwrap(),
    )
    .unwrap();
    let replaced = cli(
        &store,
        "replace",
        &[project.to_str().unwrap(), "Pool/change-pool"],
    );
    assert_ne!(replaced["structuredContent"]["revision"], prior);
    let conflict = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args([
            "unregister",
            "--store",
            store.to_str().unwrap(),
            "Pool/new-pool",
        ])
        .output()
        .unwrap();
    assert!(!conflict.status.success());
    assert!(String::from_utf8_lossy(&conflict.stdout).contains("conflict"));
    cli(&store, "unregister", &["Pool/change-pool"]);
    cli(&store, "retire", &["Pool/new-pool"]);
    assert!(
        state(&store)["objects"]
            .as_array()
            .unwrap()
            .iter()
            .all(|o| o["name"] != "new-pool" && o["name"] != "change-pool")
    );
    let current = state(&store);
    for operation in [
        "install.v1",
        "register.v1",
        "replace.v1",
        "retire.v1",
        "unregister.v1",
    ] {
        assert!(
            current["records"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["type"] == "invocation-dispatched"
                    && r["data"]["operation"] == format!("{core}#{operation}"))
        );
    }
    stop_server(&mut server, &store, &[]);
}
