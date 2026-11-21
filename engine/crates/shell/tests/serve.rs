//! Process coverage for the joined HTTP adapter and unchanged npm filesystem runner.
mod common;
use conduit_shell::{Config, MissingExecutor, Shell, boundary};
use fs2::FileExt;
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
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
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
    conduit_shell::install_crypto_provider();
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
    let output = cli_output(store, verb, args);
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
fn cli_output(store: &Path, verb: &str, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_conduit"))
        .arg(verb)
        .arg("--store")
        .arg(store)
        .args(args)
        .output()
        .unwrap()
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
// Notification delivery, startup and shutdown waits bound hangs, not speed.
async fn changed(changes: &AtomicUsize, target: usize) {
    tokio::time::timeout(Duration::from_secs(600), async {
        while changes.load(Ordering::SeqCst) < target {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
}

#[test]
fn real_npm_serve_s1_tokens_access_t7_and_shutdown() {
    conduit_shell::install_crypto_provider();
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
        // ACC-053 / LIF-041: restoration requires the root client's explicit lift.
        let closed_state = state(&store);
        let stone = &closed_state["tombstones"].as_object().unwrap().values()
            .find(|stone| stone["object"]["id"] == conduit["id"]).unwrap()["id"];
        cli(&store,"call",&["access.lift.v1",&json!({"id":stone}).to_string()]);
        cli(&store, "call", &["management.register-replacement.v1", r#"{"instance":{"kind":"Agent","name":"agent-a"},"nested":"project"}"#]);
        cli(&store, "call", &["s1.set-up.v1", r#"{"instance":{"kind":"Agent","name":"agent-a"}}"#]);
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
        while unsafe { libc::kill(-upgraded_group[0],0) } == 0 { assert!(start.elapsed()<Duration::from_secs(600)); std::thread::sleep(Duration::from_millis(10)); }
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
    let mut server = spawn_server(store, Stdio::piped());
    let ready = server_output_event(server.0.stdout.take().unwrap(), |_| true);
    let line = ready
        .recv_timeout(Duration::from_secs(600))
        .expect("server startup timed out")
        .expect("cannot read the server ready line")
        .unwrap_or_else(|| {
            panic!(
                "server exited before printing its ready line: {}",
                fs::read_to_string(store.join("serve.stderr")).unwrap()
            )
        });
    assert_eq!(
        serde_json::from_str::<Value>(&line).unwrap(),
        json!({"ready":{"url":format!("http://127.0.0.1:{port}/mcp"),"store":store.canonicalize().unwrap()}})
    );
    server
}

fn server_output_event(
    stream: impl Read + Send + 'static,
    matches: fn(&str) -> bool,
) -> mpsc::Receiver<std::io::Result<Option<String>>> {
    let (send, receive) = mpsc::channel();
    std::thread::spawn(move || {
        let mut stream = BufReader::new(stream);
        let event = stream
            .by_ref()
            .lines()
            .find(|line| line.as_ref().is_err() || line.as_ref().is_ok_and(|line| matches(line)))
            .transpose();
        let _ = send.send(event);
        // Keep the child's output open until it exits.
        let _ = std::io::copy(&mut stream, &mut std::io::sink());
    });
    receive
}

fn spawn_server(store: &Path, stdout: Stdio) -> Server {
    let stderr = fs::File::create(store.join("serve.stderr")).unwrap();
    Server(
        server_command(store)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .unwrap(),
    )
}

fn server_command(store: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_conduit"));
    command.arg("serve").arg("--store").arg(store);
    command
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
fn terminate_server(server: &mut Server) {
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
        assert!(start.elapsed() < Duration::from_secs(600));
        std::thread::sleep(Duration::from_millis(20));
    }
}
fn stop_server(server: &mut Server, store: &Path, groups: &[i32]) {
    terminate_server(server);
    for &group in groups {
        let start = Instant::now();
        while unsafe { libc::kill(-group, 0) } == 0 {
            assert!(
                start.elapsed() < Duration::from_secs(600),
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

fn pack_scenario_s4_effect(package: &Path) -> (PathBuf, String) {
    let cargo = |subcommand, binary| {
        // Use the package's pinned host build even when the engine shares a cache.
        let output = Command::new("cargo")
            .current_dir(package)
            .env_remove("RUSTUP_TOOLCHAIN")
            .env_remove("CARGO_BUILD_TARGET")
            .env_remove("CARGO_BUILD_BUILD_DIR")
            .args([
                subcommand,
                "--offline",
                "--locked",
                "--release",
                "--bin",
                binary,
            ])
            .arg("--target-dir")
            .arg(package.join("target"))
            .output()
            .expect("start Cargo for scenario S4");
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let lines: Vec<_> = stderr.lines().collect();
            panic!(
                "scenario S4 cargo {subcommand} --bin {binary} failed ({}):\n{}",
                output.status,
                lines[lines.len().saturating_sub(20)..].join("\n")
            );
        }
        output.stdout
    };
    cargo("build", "scenario-s4-effect");
    let output = cargo("run", "package");
    let archive = package.join("dist/scenario-s4-effect.tar.gz");
    let sidecar = fs::read_to_string(archive.with_extension("gz.sha256")).unwrap();
    let digest = format!("sha256:{}", sidecar.trim());
    assert_eq!(
        String::from_utf8(output).unwrap().trim(),
        format!("scenario-s4-effect.tar.gz {digest}")
    );
    assert_eq!(
        digest,
        conduit_core::packages::bytes_digest(&fs::read(&archive).unwrap())
    );
    (archive, digest)
}

#[test]
fn scenario_s4_effect_installs_and_registers_templates_only() {
    const ADDRESS: &str = "example.org/conduit/scenario-s4-effect";
    const PLACEHOLDER: &str = "sha256:PLACEHOLDER";
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let package = repo
        .join("packages/scenario-s4-effect")
        .canonicalize()
        .unwrap();
    let (archive, digest) = pack_scenario_s4_effect(&package);
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let fixture = package.join("tests/fixtures/project");
    fs::create_dir(&project).unwrap();
    let mut placeholders = 0;
    for name in ["conduit.yaml", "templates.yaml"] {
        let text = fs::read_to_string(fixture.join(name)).unwrap();
        placeholders += text.matches(PLACEHOLDER).count();
        fs::write(project.join(name), text.replace(PLACEHOLDER, &digest)).unwrap();
    }
    assert_eq!(placeholders, 1, "expected one fixture digest pin");

    let store = temp.path().join("store");
    let (port, _) = records_store(&store);
    let mut server = start_server(&store, port);
    cli(
        &store,
        "install",
        &[&format!("{}#{digest}", archive.display())],
    );
    let before_submit = state(&store)["records"].as_array().unwrap().len();
    cli(&store, "submit", &[project.to_str().unwrap()]);

    let current = state(&store);
    let resources = current["objects"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|object| object["kind"] == "Resource");
    for name in ["effect-plain", "effect-keyed", "effect-ledger"] {
        let kind = format!("{ADDRESS}#{name}.v1");
        assert!(
            resources
                .clone()
                .any(|object| { object["name"] == name && object["spec"]["type"] == kind }),
            "missing Resource template {name} of Type {kind}"
        );
        assert!(
            !resources
                .clone()
                .any(|object| object.get("spec").is_none() && object["type"] == kind),
            "submit created a Resource instance of Type {kind}"
        );
    }

    let core = boundary::context(&repo).unwrap().core_package;
    let register = format!("{core}#register.v1");
    let dispatches: Vec<_> = current["records"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .filter(|(_, record)| record["type"] == "invocation-dispatched")
        .map(|(index, record)| (index, record["data"]["operation"].as_str().unwrap()))
        .collect();
    for operation in [format!("{core}#install.v1"), register.clone()] {
        assert!(
            dispatches
                .iter()
                .any(|(_, dispatched)| *dispatched == operation),
            "missing dispatch record for {operation}"
        );
    }
    let prefix = format!("{ADDRESS}#");
    for (index, operation) in dispatches {
        assert!(
            !operation.starts_with(&prefix),
            "dispatched to S4: {operation}"
        );
        if index >= before_submit {
            assert_eq!(
                operation, register,
                "submit dispatched more than registration"
            );
        }
    }
    terminate_server(&mut server);
}

fn records_store(store: &Path) -> (u16, String) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}/mcp");
    cli(store, "init", &["--url", &address]);
    (port, address)
}

#[test]
fn call_reads_arguments_from_a_file_or_stdin() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let (port, _) = records_store(&store);
    let mut server = start_server(&store, port);
    let root = state(&store)["objects"]
        .as_array()
        .unwrap()
        .iter()
        .find(|object| object["kind"] == "Interface" && object["name"] == "root")
        .unwrap()["id"]
        .clone();
    let tool = "management.inspect.v1";
    let arguments = r#"{"object":{"kind":"Interface","name":"root"}}"#;
    assert_eq!(
        cli(&store, "call", &[tool, arguments])["structuredContent"]["id"],
        root
    );

    let file = temp.path().join("arguments.json");
    fs::write(&file, format!("{}{arguments}", " ".repeat(64 * 1024))).unwrap();
    let path = file.to_str().unwrap();
    assert_eq!(
        cli(&store, "call", &[tool, "--arguments-file", path])["structuredContent"]["id"],
        root
    );
    let call_stdin = |input: &str| {
        let mut child = Command::new(env!("CARGO_BIN_EXE_conduit"))
            .args(["call", "--store"])
            .arg(&store)
            .args([tool, "--arguments-file", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        child.wait_with_output().unwrap()
    };
    let stdin = call_stdin(arguments);
    assert!(
        stdin.status.success(),
        "{}",
        String::from_utf8_lossy(&stdin.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&stdin.stdout).unwrap()["structuredContent"]["id"],
        root
    );

    let decisions = || {
        state(&store)["records"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|record| record["type"] == "access-decision")
            .count()
    };
    let before = decisions();
    let marker = format!("synthetic-{}-{port}", std::process::id());
    let invalid = format!("{{\n  \"case\": {marker}\n}}");
    let refused = |output: Output, source: &str, cause: &str| {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!stderr.contains(&marker), "stderr exposed input content");
        assert!(
            stderr.contains(source) && stderr.contains(cause),
            "{stderr}"
        );
        assert_eq!(decisions(), before);
    };
    refused(
        cli_output(&store, "call", &[tool, arguments, "--arguments-file", path]),
        path,
        "inline",
    );
    let missing = temp.path().join("missing.json");
    let missing = missing.to_str().unwrap();
    refused(
        cli_output(&store, "call", &[tool, "--arguments-file", missing]),
        missing,
        "read",
    );
    fs::write(&file, &invalid).unwrap();
    refused(
        cli_output(&store, "call", &[tool, "--arguments-file", path]),
        path,
        "line 2, column",
    );
    refused(call_stdin(&invalid), "standard input", "line 2, column");
    terminate_server(&mut server);
}

fn serve_failed_setups(parent: &Path, store: &Path) -> Server {
    let (port, _) = records_store(store);
    let server = start_server(store, port);
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let spec = conduit_loader::Specification::read(&repo.join("spec")).unwrap();
    let api = &spec.names().api_version;
    let core = boundary::context(&repo).unwrap().core_package;
    let hook = format!("{core}#setup-hook.v1");
    let hook = hook.as_str();
    let project = parent.join("project");
    fs::create_dir(&project).unwrap();
    let manifest = json!({
        "apiVersion":api,"address":"example.org/test/setup-views",
        "implementations":{"local":{"implements":[hook],"command":[parent.join("missing-program")]}}
    });
    let documents = json!([
        {"apiVersion":api,"kind":"Type","metadata":{"name":"empty.v1"},"spec":{"family":"Resource","schema":{"type":"object"},"offers":[],"requires":[]}},
        {"apiVersion":api,"kind":"Type","metadata":{"name":"worker.v1"},"spec":{"family":"Agent","schema":{"type":"object"},"offers":[],"requires":[]}},
        {"apiVersion":api,"kind":"Pool","metadata":{"name":"test-pool"},"spec":{"type":format!("{core}#pool.v1")}},
        {"apiVersion":api,"kind":"Pool","metadata":{"name":"workspace"},"from":"test-pool"},
        {"apiVersion":api,"kind":"Fleet","metadata":{"name":"crew"},"from":format!("{core}#fleet")},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"failing-template"},"spec":{"type":"#empty.v1","config":{},"implementation":{hook:"#local"}}},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"failing"},"from":"failing-template","owner":"workspace"},
        {"apiVersion":api,"kind":"Agent","metadata":{"name":"worker-template"},"spec":{"type":"#worker.v1","config":{},"implementation":{hook:"#local"}}},
        {"apiVersion":api,"kind":"Agent","metadata":{"name":"worker"},"from":"worker-template","owner":"crew"}
    ]);
    fs::write(project.join("conduit.json"), manifest.to_string()).unwrap();
    fs::write(project.join("documents.json"), documents.to_string()).unwrap();
    let submitted = cli_output(store, "submit", &[project.to_str().unwrap()]);
    let submit_stdout = String::from_utf8_lossy(&submitted.stdout);
    let submit_stderr = String::from_utf8_lossy(&submitted.stderr);
    assert!(
        !submitted.status.success(),
        "submit unexpectedly succeeded\nstdout:\n{submit_stdout}\nstderr:\n{submit_stderr}"
    );
    assert!(
        record_lines(&submitted.stdout).iter().any(|step| {
            step["step"] == "setup Resource/failing"
                && matches!(step["outcome"].as_str(), Some("failed" | "unknown"))
        }),
        "submit omitted the failed Resource setup\nstdout:\n{submit_stdout}\nstderr:\n{submit_stderr}"
    );
    let worker = cli_output(
        store,
        "call",
        &[
            "crew.set-up.v1",
            r#"{"instance":{"kind":"Agent","name":"worker"}}"#,
        ],
    );
    let worker_stdout = String::from_utf8_lossy(&worker.stdout);
    let worker_stderr = String::from_utf8_lossy(&worker.stderr);
    assert!(
        !worker.status.success(),
        "Agent setup unexpectedly succeeded\nstdout:\n{worker_stdout}\nstderr:\n{worker_stderr}"
    );
    assert!(
        !worker.stdout.is_empty(),
        "Agent setup omitted its result\nstdout:\n{worker_stdout}\nstderr:\n{worker_stderr}"
    );
    server
}

#[test]
#[should_panic(expected = "server exited before printing its ready line")]
fn failed_setups_fixture_forced_cause() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let other = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = other.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}/mcp");
    cli(&store, "init", &["--url", &address]);
    let _server = start_server(&store, port);
}

#[test]
fn pending_agent_lists_that_agents_actions_alone() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let mut server = serve_failed_setups(temp.path(), &store);
    let full = cli_output(&store, "pending", &[]);
    assert!(
        full.status.success(),
        "{}",
        String::from_utf8_lossy(&full.stderr)
    );
    let lines: Vec<_> = full.stdout.split_inclusive(|byte| *byte == b'\n').collect();
    assert_eq!(lines.len(), 2);
    let agent_line = lines
        .iter()
        .find(|line| {
            let action: Value = serde_json::from_slice(line).unwrap();
            action["subject"]
                .as_array()
                .unwrap()
                .iter()
                .any(|subject| subject["kind"] == "Agent" && subject["name"] == "worker")
        })
        .unwrap();
    let before = state(&store)["records"].as_array().unwrap().len();
    let agent = cli_output(&store, "pending", &["--agent", "worker"]);
    assert!(
        agent.status.success(),
        "{}",
        String::from_utf8_lossy(&agent.stderr)
    );
    assert_eq!(agent.stdout, *agent_line);
    let after = state(&store);
    let tools: Vec<_> = after["records"]
        .as_array()
        .unwrap()
        .iter()
        .skip(before)
        .filter(|record| record["type"] == "access-decision")
        .map(|record| &record["data"]["tool"])
        .collect();
    assert_eq!(tools, [&json!("management.list.v1")]);
    for name in ["failing", "missing"] {
        let rejected = cli_output(&store, "pending", &["--agent", name]);
        let stderr = String::from_utf8_lossy(&rejected.stderr);
        assert!(!rejected.status.success());
        assert!(rejected.stdout.is_empty());
        assert!(
            stderr.contains(name) && stderr.contains("root sees no Agent"),
            "{stderr}"
        );
    }
    let action: Value = serde_json::from_slice(agent_line).unwrap();
    cli(
        &store,
        "answer",
        &[action["id"].as_str().unwrap(), "dismissed"],
    );
    let empty = cli_output(&store, "pending", &["--agent", "worker"]);
    let stderr = String::from_utf8_lossy(&empty.stderr);
    assert!(empty.status.success(), "{stderr}");
    assert!(empty.stdout.is_empty());
    assert!(
        stderr.contains("worker") && stderr.contains("no pending action"),
        "{stderr}"
    );
    terminate_server(&mut server);
}

#[test]
fn inspect_setup_prints_one_line_per_instance() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let mut server = serve_failed_setups(temp.path(), &store);
    for (tool, kind, name) in [
        ("workspace.inspect.v1", "Resource", "failing"),
        ("crew.inspect.v1", "Agent", "worker"),
    ] {
        let args = json!({"object":{"kind":kind,"name":name}}).to_string();
        let reply = cli(&store, "call", &[tool, &args]);
        assert!(
            reply["structuredContent"]["status"]["setup"].is_object(),
            "{kind}/{name} inspection omitted the completed setup"
        );
    }
    let before = state(&store);
    let expected: Vec<_> = before["objects"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|object| {
            matches!(
                object["kind"].as_str(),
                Some("Interface" | "Pool" | "Fleet" | "Resource" | "Agent")
            ) && object.get("spec").is_none()
        })
        .map(|object| json!({"kind":object["kind"],"name":object["name"]}))
        .collect();
    let online = cli_output(&store, "inspect", &["--setup"]);
    assert!(
        online.status.success(),
        "{}",
        String::from_utf8_lossy(&online.stderr)
    );
    let rows = record_lines(&online.stdout);
    assert_eq!(
        rows.iter()
            .map(|row| row["object"].clone())
            .collect::<Vec<_>>(),
        expected
    );
    let after = state(&store);
    assert_eq!(
        after["records"]
            .as_array()
            .unwrap()
            .iter()
            .skip(before["records"].as_array().unwrap().len())
            .filter(|record| record["type"] == "access-decision")
            .count(),
        rows.len()
    );
    let row = |name: &str| {
        rows.iter()
            .find(|row| row["object"]["name"] == name)
            .unwrap()
    };
    let pool = &row("workspace")["setup"];
    assert_eq!(pool["outcome"], "succeeded");
    assert_eq!(pool["notSucceeded"], json!([]));
    for name in ["failing", "worker"] {
        let inspected = row(name);
        assert_eq!(inspected["state"], "registered");
        let setup = &inspected["setup"];
        assert!(
            matches!(setup["outcome"].as_str(), Some("failed" | "unknown")),
            "{setup}"
        );
        assert!(setup["invocation"].is_string());
        assert!(
            setup["notSucceeded"]
                .as_array()
                .unwrap()
                .iter()
                .any(|part| {
                    part["name"] == "run"
                        && matches!(
                            part["outcome"].as_str(),
                            Some("failed" | "unknown" | "unavailable")
                        )
                }),
            "{setup}"
        );
    }
    for row in rows.iter().filter(|row| {
        !["workspace", "crew", "failing", "worker"]
            .iter()
            .any(|name| row["object"]["name"] == *name)
    }) {
        assert_eq!(row.get("setup"), Some(&Value::Null), "{row}");
    }
    terminate_server(&mut server);
    let stopped = cli_output(&store, "inspect", &["--setup"]);
    assert!(
        stopped.status.success(),
        "{}",
        String::from_utf8_lossy(&stopped.stderr)
    );
    assert_eq!(stopped.stdout, online.stdout);
}

#[test]
fn serve_prints_one_ready_line_once_it_answers() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let (port, address) = records_store(&store);
    let mut server = spawn_server(&store, Stdio::piped());
    let stdout = server.0.stdout.take().unwrap();
    let (line, ready) = mpsc::channel();
    let reader = std::thread::spawn(move || {
        let mut stdout = BufReader::new(stdout);
        let mut first = String::new();
        stdout.read_line(&mut first).unwrap();
        if line.send(first).is_err() {
            return Vec::new();
        }
        let mut rest = Vec::new();
        stdout.read_to_end(&mut rest).unwrap();
        rest
    });
    let line = ready
        .recv_timeout(Duration::from_secs(10))
        .expect("serve did not print its ready line");
    assert!(line.ends_with('\n'));
    assert_eq!(
        serde_json::from_str::<Value>(&line).unwrap(),
        json!({"ready":{"url":address,"store":store.canonicalize().unwrap()}})
    );
    let tools = cli_output(&store, "tools", &[]);
    let stderr = String::from_utf8_lossy(&tools.stderr);
    assert!(tools.status.success(), "{stderr}");
    assert!(!stderr.contains("retrying"), "{stderr}");
    terminate_server(&mut server);
    assert!(reader.join().unwrap().is_empty());

    let (read, write) = std::io::pipe().unwrap();
    drop(read);
    let mut broken = Server(
        server_command(&store)
            .stdout(write)
            .stderr(Stdio::piped())
            .spawn()
            .unwrap(),
    );
    let diagnostic = server_output_event(broken.0.stderr.take().unwrap(), |line| {
        line.contains("cannot write ready line")
    });
    assert!(
        diagnostic
            .recv_timeout(Duration::from_secs(600))
            .expect("server startup timed out")
            .expect("cannot read the server startup diagnostic")
            .is_some(),
        "server exited without reporting that it cannot write its ready line"
    );
    assert!(cli_output(&store, "tools", &[]).status.success());
    terminate_server(&mut broken);

    let _occupied = std::net::TcpListener::bind(("127.0.0.1", port)).unwrap();
    let failed = cli_output(&store, "serve", &[]);
    assert!(!failed.status.success());
    assert!(failed.stdout.is_empty());
}

fn record_lines(stdout: &[u8]) -> Vec<Value> {
    std::str::from_utf8(stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn complete_records(store: &Path) -> Vec<u8> {
    let output = cli_output(store, "records", &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "{}",
        stderr.lines().next().unwrap_or("")
    );
    assert!(output.stderr.is_empty());
    output.stdout
}

#[test]
fn records_while_serving_match_records_after_stop() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let (port, _) = records_store(&store);
    let mut server = start_server(&store, port);
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let spec = conduit_loader::Specification::read(&repo.join("spec")).unwrap();
    let core = boundary::context(&repo).unwrap().core_package;
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();
    fs::write(
        project.join("documents.json"),
        serde_json::to_vec(&json!([
            {"apiVersion":spec.names().api_version,"kind":"Pool","metadata":{"name":"change-pool"},"spec":{"type":format!("{core}#pool.v1")}}
        ]))
        .unwrap(),
    )
    .unwrap();
    cli(&store, "submit", &[project.to_str().unwrap()]);
    let registered = state(&store)["objects"]
        .as_array()
        .unwrap()
        .iter()
        .find(|object| object["name"] == "change-pool")
        .unwrap()["id"]
        .clone();
    cli(&store, "unregister", &["Pool/change-pool"]);
    assert!(
        state(&store)["objects"]
            .as_array()
            .unwrap()
            .iter()
            .all(|object| object["id"] != registered)
    );

    let a = complete_records(&store);
    cli(&store, "inspect", &[]);
    let b = complete_records(&store);
    terminate_server(&mut server);
    let c = complete_records(&store);
    assert!(!a.is_empty());
    assert_eq!(a.last(), Some(&b'\n'));
    assert!(b.starts_with(&a) && b.len() > a.len());
    assert!(b == c, "exports before and after shutdown differ");
    assert!(record_lines(&b[a.len()..]).iter().any(|record| {
        record["type"] == "invocation-dispatched"
            && record["data"]["operation"] == format!("{core}#inspect.v1")
    }));
    assert!(record_lines(&b).iter().any(|record| {
        record["type"] == "object-change"
            && record["data"]["target"] == registered
            && record["data"]["action"] == "removed"
    }));
}

#[test]
fn records_while_serving_report_a_gap_as_after_stop() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let (port, _) = records_store(&store);
    let mut server = start_server(&store, port);
    let arguments = json!({"before":chrono::Utc::now().to_rfc3339()}).to_string();
    cli(&store, "call", &["management.purge.v1", &arguments]);
    let live = cli_output(&store, "records", &[]);
    assert!(!live.status.success());
    let stderr = String::from_utf8_lossy(&live.stderr);
    assert!(
        stderr.starts_with("records: export has gaps:"),
        "{}",
        stderr.lines().next().unwrap_or("")
    );
    assert!(stderr.contains("\"cause\":\"purged\""));
    assert!(!live.stdout.is_empty());
    terminate_server(&mut server);
    let stopped = cli_output(&store, "records", &[]);
    assert_eq!(live.status.code(), stopped.status.code());
    assert!(live.stdout == stopped.stdout, "exports with gaps differ");
    assert!(live.stderr == stopped.stderr, "gap diagnostics differ");
}

#[tokio::test]
async fn records_route_answers_root_alone() {
    use reqwest::{Method, StatusCode};

    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let (port, _) = records_store(&store);
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let shell = Shell::open(
        &store,
        boundary::context(&repo).unwrap(),
        Config::for_store(&store).unwrap(),
    )
    .unwrap();
    let snapshot = shell.state().unwrap();
    let id = |kind, name| {
        snapshot
            .objects
            .iter()
            .find(|object| object["kind"] == kind && object["name"] == name)
            .unwrap()["id"]
            .as_str()
            .unwrap()
    };
    let pool = id("Pool", "system");
    shell.issue_client_token(pool).unwrap();
    drop(shell);

    let _server = start_server(&store, port);
    let token = |subject| fs::read_to_string(store.join("client-tokens").join(subject)).unwrap();
    let root_token = token(id("Interface", "root"));
    let pool_token = token(pool);
    let client = reqwest::Client::builder().no_proxy().build().unwrap();
    let url = format!("http://127.0.0.1:{port}/records");
    let record_count = || state(&store)["records"].as_array().unwrap().len();
    let before = record_count();
    let response = client
        .get(&url)
        .bearer_auth(&root_token)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK, "GET /records with root");
    let reply: Value = response.json().await.unwrap();
    let records = record_lines(&complete_records(&store));
    assert!(!records.is_empty());
    assert!(reply == json!({"records":records,"gaps":[],"complete":true}));

    for (method, token, origin, status) in [
        (Method::GET, None, None, StatusCode::UNAUTHORIZED),
        (
            Method::GET,
            Some("forged-token"),
            None,
            StatusCode::UNAUTHORIZED,
        ),
        (
            Method::GET,
            Some(root_token.as_str()),
            Some("http://127.0.0.1:1"),
            StatusCode::FORBIDDEN,
        ),
        (
            Method::POST,
            Some(root_token.as_str()),
            None,
            StatusCode::METHOD_NOT_ALLOWED,
        ),
        (
            Method::HEAD,
            Some(root_token.as_str()),
            None,
            StatusCode::METHOD_NOT_ALLOWED,
        ),
        (
            Method::GET,
            Some(pool_token.as_str()),
            None,
            StatusCode::FORBIDDEN,
        ),
    ] {
        let mut request = client.request(method.clone(), &url);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        if let Some(origin) = origin {
            request = request.header("origin", origin);
        }
        let response = request.send().await.unwrap();
        assert_eq!(response.status(), status, "{method} /records");
        assert!(!response.text().await.unwrap().contains("\"records\""));
    }
    assert_eq!(record_count(), before);
}

#[test]
fn records_fallback_uses_serving_snapshot_when_disk_is_unreadable() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let (port, _) = records_store(&store);
    // Start as an existing store that predates publication of the root ID.
    let metadata = store.join("root-client-id");
    if metadata.exists() {
        fs::remove_file(&metadata).unwrap();
    }
    let mut server = start_server(&store, port);
    let expected = complete_records(&store);
    assert!(!expected.is_empty());
    fs::write(store.join("state.json"), b"invalid JSON").unwrap();
    assert!(
        complete_records(&store) == expected,
        "live export differs from the serving snapshot"
    );
    // Simulate a serving build that has not published the root ID.
    fs::remove_file(metadata).unwrap();
    let output = cli_output(&store, "records", &[]);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "conduit records: root client identity is unavailable. Restart conduit serve with this build.\n"
    );
    terminate_server(&mut server);
}

#[test]
fn records_when_held_without_serve_names_the_address() {
    assert_records_names_unanswered_address(false);
}

#[test]
fn records_fallback_names_address_when_root_token_is_unavailable() {
    assert_records_names_unanswered_address(true);
}

fn assert_records_names_unanswered_address(remove_token: bool) {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let socket = tokio::net::TcpSocket::new_v4().unwrap();
    socket.bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let address = format!("http://{}/mcp", socket.local_addr().unwrap());
    cli(&store, "init", &["--url", &address]);
    if remove_token {
        fs::remove_dir_all(store.join("client-tokens")).unwrap();
    }
    let lease = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(store.join("writer.lock"))
        .unwrap();
    FileExt::try_lock_exclusive(&lease).unwrap();
    let output = cli_output(&store, "records", &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        stderr.contains(&address) && stderr.contains("serve does not answer"),
        "{}",
        stderr.lines().next().unwrap_or("")
    );
    assert!(stderr.contains("another process holds the store"));
}

#[test]
fn pending_and_answer_list_and_dismiss_a_failed_setup() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("store");
    let run = |verb: &str, args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_conduit"))
            .arg(verb)
            .arg("--store")
            .arg(&store)
            .args(args)
            .output()
            .unwrap()
    };
    let replies = |stdout: &[u8], stderr: &str| -> Vec<Value> {
        let stdout = std::str::from_utf8(stdout)
            .unwrap_or_else(|error| panic!("{error}\n{stderr}\n{}", stdout.escape_ascii()));
        stdout
            .lines()
            .map(|line| {
                serde_json::from_str(line)
                    .unwrap_or_else(|error| panic!("{error}\n{stderr}\n{stdout}"))
            })
            .collect()
    };
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let output = run("init", &["--url", &format!("http://127.0.0.1:{port}/mcp")]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut server = start_server(&store, port);

    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let spec = conduit_loader::Specification::read(&repo.join("spec")).unwrap();
    let api = &spec.names().api_version;
    let core = boundary::context(&repo).unwrap().core_package;
    let hook = format!("{core}#setup-hook.v1");
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();
    let manifest = json!({
        "apiVersion":api,"address":"example.org/test/pending-answer",
        "implementations":{"local":{"implements":[hook],"command":["/test/program"]}}
    });
    let documents = json!([
        {"apiVersion":api,"kind":"Type","metadata":{"name":"empty.v1"},"spec":{"family":"Resource","schema":{"type":"object"},"offers":[],"requires":[]}},
        {"apiVersion":api,"kind":"Pool","metadata":{"name":"test-pool"},"spec":{"type":format!("{core}#pool.v1")}},
        {"apiVersion":api,"kind":"Pool","metadata":{"name":"workspace"},"from":"test-pool"},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"failing-template"},"spec":{"type":"#empty.v1","config":{},"implementation":{hook:"#local"}}},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"failing"},"from":"failing-template","owner":"workspace"}
    ]);
    fs::write(project.join("conduit.json"), manifest.to_string()).unwrap();
    fs::write(project.join("documents.json"), documents.to_string()).unwrap();
    let output = run("submit", &[project.to_str().unwrap()]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    assert!(
        replies(&output.stdout, &stderr).iter().any(|step| {
            step["step"] == "setup Resource/failing"
                && matches!(step["outcome"].as_str(), Some("failed" | "unknown"))
        }),
        "{stderr}"
    );

    let output = run("pending", &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    let actions = replies(&output.stdout, &stderr);
    assert_eq!(actions.len(), 1, "{stderr}");
    assert_eq!(actions[0]["kind"], "setup", "{stderr}");
    assert_eq!(actions[0]["subject"][0]["kind"], "Resource", "{stderr}");
    assert_eq!(actions[0]["subject"][0]["name"], "failing", "{stderr}");
    let id = actions[0]["id"]
        .as_str()
        .unwrap_or_else(|| panic!("action ID must be a string\n{stderr}"));
    assert!(!stderr.contains("unanswered"), "{stderr}");

    let inspect_root = [
        "management.inspect.v1",
        r#"{"object":{"kind":"Interface","name":"root"}}"#,
    ];
    let output = run("call", &inspect_root);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains("conduit: 1 pending action(s) unanswered at root"),
        "{stderr}"
    );

    let assert_unlisted_answer = |id: &str| {
        let before = state(&store)["records"].as_array().unwrap().len();
        let output = run("answer", &[id, "dismissed"]);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{stderr}");
        assert!(output.stdout.is_empty(), "{stderr}");
        assert!(stderr.contains("nothing was sent"), "{stderr}");
        assert!(
            stderr.contains(&format!("no action {id} is raised at root now")),
            "{stderr}"
        );
        let current = state(&store);
        let tools: Vec<_> = current["records"]
            .as_array()
            .unwrap_or_else(|| panic!("records are missing\n{stderr}"))
            .iter()
            .skip(before)
            .filter(|record| record["type"] == "access-decision")
            .map(|record| &record["data"]["tool"])
            .collect();
        assert_eq!(tools, [&json!("management.list.v1")], "{stderr}");
        stderr.into_owned()
    };
    let stderr = assert_unlisted_answer("00000000-0000-4000-8000-0000000000ff");
    assert!(
        stderr.contains("conduit: 1 pending action(s) unanswered at root"),
        "{stderr}"
    );

    for answer in ["retried", "-dismissed", "--help", "--store"] {
        let before = state(&store)["records"].as_array().unwrap().len();
        let output = run("answer", &[id, answer]);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !output.status.success(),
            "answer {answer:?}\n{stderr}\n{stdout}"
        );
        let result = replies(&output.stdout, &stderr);
        assert_eq!(result.len(), 1, "answer {answer:?}\n{stderr}\n{stdout}");
        assert_eq!(
            result[0]["isError"], true,
            "answer {answer:?}\n{stderr}\n{stdout}"
        );
        assert!(
            result[0]["content"][0]["text"]
                .as_str()
                .is_some_and(|text| text.contains("reports invalid under PND-009")),
            "answer {answer:?}\n{stderr}\n{stdout}"
        );
        let current = state(&store);
        let sent: Vec<_> = current["records"]
            .as_array()
            .unwrap_or_else(|| panic!("records are missing\n{stderr}\n{stdout}"))
            .iter()
            .skip(before)
            .filter(|record| {
                record["type"] == "access-decision"
                    && record["data"]["tool"] != "management.list.v1"
            })
            .map(|record| &record["data"]["tool"])
            .collect();
        assert_eq!(
            sent,
            [&json!("workspace.answer.v1")],
            "answer {answer:?}\n{stderr}\n{stdout}"
        );
    }

    let output = run("answer", &[id, "dismissed"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(!stderr.contains("unanswered"), "{stderr}");
    let result = replies(&output.stdout, &stderr);
    assert_eq!(result.len(), 1, "{stderr}");
    assert_eq!(result[0]["isError"], false, "{stderr}");
    let current = state(&store);
    let pool = current["objects"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|object| object["kind"] == "Pool" && object["name"] == "workspace")
        .unwrap_or_else(|| panic!("workspace Pool is missing\n{stderr}"));
    let records = current["records"]
        .as_array()
        .unwrap_or_else(|| panic!("records are missing\n{stderr}"));
    let answer_operation = format!("{core}#answer.v1");
    let dispatch = records
        .iter()
        .rev()
        .find(|r| {
            r["type"] == "invocation-dispatched" && r["data"]["operation"] == answer_operation
        })
        .unwrap_or_else(|| panic!("answer dispatch is missing\n{stderr}"));
    assert_eq!(dispatch["data"]["target"], pool["id"], "{stderr}");
    assert!(
        records
            .iter()
            .any(|r| r["type"] == "pending-action-answered"
                && r["data"]["pendingAction"] == json!([id])
                && r["data"]["answer"]["answer"]["value"] == "dismissed"),
        "{stderr}"
    );

    let output = run("pending", &[]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(output.stdout.is_empty(), "{stderr}");
    assert!(
        stderr.contains("no pending action is raised at root"),
        "{stderr}"
    );

    let output = run("call", &inspect_root);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(!stderr.contains("unanswered"), "{stderr}");

    let stderr = assert_unlisted_answer(id);
    assert!(!stderr.contains("unanswered"), "{stderr}");
    stop_server(&mut server, &store, &[]);
}
