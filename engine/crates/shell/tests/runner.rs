mod common;
use conduit_shell::{
    Config, Execution, Executor, MissingExecutor, Shell, boundary, runner::Runner,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::{BufRead, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
    time::{Duration, Instant},
};

static TERM_RECEIVED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

extern "C" fn record_term(_: libc::c_int) {
    TERM_RECEIVED.store(true, std::sync::atomic::Ordering::SeqCst);
}

fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn open(parent: &Path, config: Config) -> Arc<Shell> {
    Arc::new(
        Shell::open(
            &parent.join("store"),
            boundary::context(&repository()).unwrap(),
            config,
        )
        .unwrap(),
    )
}
fn new_runner(shell: &Arc<Shell>) -> Runner {
    Runner::new(shell, PathBuf::from(env!("CARGO_BIN_EXE_conduit"))).unwrap()
}
fn id(shell: &Shell, name: &str) -> String {
    shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|v| v["name"] == name && v.get("spec").is_none())
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .into()
}
fn invoke(
    shell: &Shell,
    executor: &dyn Executor,
    actor: &str,
    tool: &str,
    arguments: Value,
) -> Value {
    shell
        .submit(input(shell, actor, tool, arguments), executor)
        .unwrap()
}
fn input(shell: &Shell, actor: &str, tool: &str, arguments: Value) -> Value {
    json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":id(shell,actor)},"tool":tool,"arguments":arguments}})
}
fn outcome(shell: &Shell, result: &Value) -> String {
    result["_meta"][format!("{}outcome", shell.context.meta_prefix)]
        .as_str()
        .unwrap_or("")
        .into()
}
fn register(shell: &Shell, parent: &Path, real: bool, mode: &str) {
    shell.submit(json!({"kind":"first-start","machine":conduit_shell::local_machine(BTreeMap::new()).unwrap()}), &MissingExecutor).unwrap();
    let repo = repository();
    let spec = conduit_loader::Specification::read(&repo.join("spec")).unwrap();
    let mut project =
        boundary::project(&repo.join("conformance/fixtures/s1/project"), &spec).unwrap();
    for name in ["shared", "history"] {
        let path = parent.join(name);
        fs::create_dir_all(&path).unwrap();
        fs::write(path.join("mode"), mode).unwrap();
    }
    for doc in &mut project.documents {
        if doc["kind"] == "Resource" {
            doc["arguments"]["root"] =
                json!(parent.join(doc["metadata"]["name"].as_str().unwrap()));
        }
    }
    let mut snapshots = common::snapshots(&repo).unwrap();
    if !real {
        let implementation = &mut snapshots
            .iter_mut()
            .find(|s| s.address.ends_with("/filesystem"))
            .unwrap()
            .project
            .manifest
            .as_mut()
            .unwrap()["implementations"]["filesystem-server"];
        implementation.as_object_mut().unwrap().remove("package");
        implementation
            .as_object_mut()
            .unwrap()
            .remove("packageArguments");
        implementation["command"] = json!([
            std::env::current_exe().unwrap(),
            "--exact",
            "scripted_server",
            "--ignored",
            "--nocapture"
        ]);
        implementation["environment"] = json!({"TEST_ROOT":"/root"});
        if mode.starts_with("discovery-") {
            implementation["plain"] = json!(false);
            implementation.as_object_mut().unwrap().remove("tools");
        }
    }
    if mode == "unsupported-registry" || mode == "hash-mismatch" {
        let item = &mut snapshots
            .iter_mut()
            .find(|s| s.address.ends_with("/filesystem"))
            .unwrap()
            .project
            .manifest
            .as_mut()
            .unwrap()["implementations"]["filesystem-server"];
        if mode == "unsupported-registry" {
            item["package"]["registryType"] = json!("pypi");
        } else {
            item["package"]["fileSha256"] = json!("0".repeat(64));
        }
    }
    let root = id(shell, "root");
    let state = shell.state().unwrap();
    let route = state
        .objects
        .iter()
        .find(|v| v["local_name"] == "management" && v["consumer"] == root)
        .unwrap();
    common::install(shell, &repo, snapshots, &mut project);
    let reply = shell.submit(json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,"project":project,"request":{"sender":root,"caller":root,"conduit":route["id"],"target":route["target"],"operation":format!("{}#register.v1",shell.context.core_package)}}), &MissingExecutor).unwrap();
    assert_eq!(reply["accepted"], true, "{reply}");
    let reply = invoke(
        shell,
        &MissingExecutor,
        "root",
        "access.open.v1",
        json!({"conduit":"root-workspace"}),
    );
    assert_eq!(outcome(shell, &reply), "succeeded", "{reply}");
}
fn setup(shell: &Shell, runner: &Runner, name: &str) -> Value {
    let kind = if name == "workspace" {
        "Pool"
    } else if name == "s1" {
        "Fleet"
    } else if name.starts_with("agent") {
        "Agent"
    } else {
        "Resource"
    };
    let tool = if kind == "Resource" {
        "root-workspace.set-up.v1"
    } else if kind == "Agent" {
        "s1.set-up.v1"
    } else {
        "management.set-up.v1"
    };
    invoke(
        shell,
        runner,
        "root",
        tool,
        json!({"instance":{"kind":kind,"name":name}}),
    )
}
fn establish(shell: &Shell, runner: &Runner) {
    for name in ["workspace", "s1", "shared", "history", "agent-a", "agent-b"] {
        let reply = setup(shell, runner, name);
        assert_eq!(
            outcome(shell, &reply),
            "succeeded",
            "{name}: {reply}; {:?}",
            runner.diagnostics()
        );
    }
}
fn lines(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|s| serde_json::from_str(s).ok())
        .collect()
}
#[track_caller]
fn until(ready: impl FnMut() -> bool) {
    // This bounds a hang, not the speed of startup, cancellation or cleanup.
    until_for(Duration::from_secs(600), ready);
}
#[track_caller]
fn until_for(timeout: Duration, mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + timeout;
    while !ready() {
        assert!(Instant::now() < deadline, "wait deadline");
        std::thread::sleep(Duration::from_millis(10));
    }
}
fn call_count(parent: &Path) -> usize {
    lines(&parent.join("shared/wire.jsonl"))
        .iter()
        .filter(|v| v["method"] == "tools/call")
        .count()
}

#[test]
#[ignore = "stdio fixture, spawned by runner tests"]
fn scripted_server() {
    let root = PathBuf::from(std::env::var("TEST_ROOT").unwrap());
    let mode = fs::read_to_string(root.join("mode")).unwrap();
    fs::write(root.join("pid"), std::process::id().to_string()).unwrap();
    if mode == "trap-term" {
        // The handler only sets an atomic flag; ordinary thread code writes the file.
        unsafe {
            libc::signal(
                libc::SIGTERM,
                record_term as *const () as libc::sighandler_t,
            )
        };
        let term = root.join("term");
        std::thread::spawn(move || {
            while !TERM_RECEIVED.load(std::sync::atomic::Ordering::SeqCst) {
                std::thread::sleep(Duration::from_millis(10));
            }
            fs::write(term, b"TERM\n").unwrap();
        });
    }
    if mode == "zombie-in-group" {
        // Deliberately leave an unreaped child in the old group after escaping it.
        #[allow(clippy::zombie_processes)]
        let _zombie = Command::new("/bin/sleep")
            .arg("0")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        assert!(unsafe { libc::setsid() } >= 0);
    }
    // These are synthetic test processes. No token file is read or copied.
    fs::write(
        root.join("environment.json"),
        serde_json::to_vec(&std::env::vars().collect::<BTreeMap<_, _>>()).unwrap(),
    )
    .unwrap();
    let cwd = std::env::current_dir().unwrap();
    fs::write(root.join("cwd.json"), serde_json::to_vec(&json!({"path":cwd,"mode":fs::metadata(&cwd).unwrap().permissions().mode()&0o777,"empty":fs::read_dir(&cwd).unwrap().count()==0})).unwrap()).unwrap();
    let _descendant = if mode == "descendant" {
        // Outlive every progress guard (and CI's job bound): a leaked descendant
        // must not exit on its own and make the kill/cleanup assertions pass.
        let child = Command::new("/bin/sleep")
            .arg("86400")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .unwrap();
        fs::write(root.join("descendant-pid"), child.id().to_string()).unwrap();
        Some(child)
    } else {
        None
    };
    let mut calls = 0;
    let mut held_id = Value::Null;
    for line in std::io::stdin().lock().lines() {
        let value: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let mut wire = OpenOptions::new()
            .create(true)
            .append(true)
            .open(root.join("wire.jsonl"))
            .unwrap();
        writeln!(wire, "{value}").unwrap();
        let result = match value["method"].as_str().unwrap() {
            "initialize" if mode == "no-initialize" => continue,
            "initialize" => {
                json!({"protocolVersion":value["params"]["protocolVersion"],"serverInfo":{"name":"test","version":"1"},"capabilities":{"tools":{}}})
            }
            "server/discover" if mode == "discovery-v0" => {
                json!({"capabilities":{"extensions":{
                    "io.github.shrav-k.conduit/implementation-interface":{"revision":"v0"}
                }}})
            }
            "server/discover" => json!({"capabilities":{"tools":{}}}),
            "tools/list" if mode == "repair-list-hang" => continue,
            "tools/list" if mode == "repair-list-error" => {
                println!(
                    "{}",
                    json!({"jsonrpc":"2.0","id":value["id"],"error":{"code":-32603,"message":"list unavailable"}})
                );
                std::io::stdout().flush().unwrap();
                continue;
            }
            "tools/list" => {
                json!({"tools":[{"name":"read_text_file","inputSchema":{"type":"object"}}]})
            }
            "tools/call" => {
                calls += 1;
                if mode == "exit" {
                    std::process::exit(0);
                }
                if mode == "hang" || mode == "cancel" {
                    held_id = value["id"].clone();
                    continue;
                }
                if mode == "hang-closed-stdin" {
                    // Hold the recorded call and stay alive with standard input closed, so the
                    // runner's next write to this run fails. The runner's stop ends the process.
                    unsafe { libc::close(0) };
                    loop {
                        std::thread::sleep(Duration::from_secs(3600));
                    }
                }
                if mode == "error-write" || mode.starts_with("repair") && calls == 1 {
                    println!(
                        "{}",
                        json!({"jsonrpc":"2.0","id":value["id"],"error":{"code":-32020,"message":"refresh","data":{"marker":"original-data"}}})
                    );
                    std::io::stdout().flush().unwrap();
                    continue;
                }
                if mode == "input-required" && calls == 1 {
                    json!({
                        "resultType": "input_required",
                        "inputRequests": {
                            "sign-in": {
                                "method": "elicitation/create",
                                "params": {
                                    "mode": "url",
                                    "message": "Sign in to continue.",
                                    "url": "https://example.org/authorize"
                                }
                            }
                        },
                        "requestState": "impl-state-1"
                    })
                } else {
                    let mut response =
                        json!({"content":[{"type":"text","text":"test result"}],"isError":false});
                    if mode == "discovery-v0" {
                        response["_meta"]["io.github.shrav-k.conduit/outcome"] = json!("succeeded");
                    }
                    response
                }
            }
            "notifications/cancelled" if mode == "cancel" => {
                println!(
                    "{}",
                    json!({"jsonrpc":"2.0","id":held_id,"result":{"content":[{"type":"text","text":"late"}]}})
                );
                std::io::stdout().flush().unwrap();
                continue;
            }
            _ => continue,
        };
        println!(
            "{}",
            json!({"jsonrpc":"2.0","id":value["id"],"result":result})
        );
        std::io::stdout().flush().unwrap();
    }
}

const DEVICE_LOGIN_URL: &str = "https://example.org/device";

#[tokio::test]
#[ignore = "rmcp stdio fixture, spawned by the sign-in runner test"]
async fn rmcp_sign_in_provider() {
    use rmcp::{
        ErrorData, RoleServer, ServerHandler, ServiceExt, model::*, service::RequestContext,
    };

    struct SignInProvider;

    impl ServerHandler for SignInProvider {
        fn get_info(&self) -> ServerConfig {
            ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
        }

        async fn call_tool(
            &self,
            request: CallToolRequestParams,
            context: RequestContext<RoleServer>,
        ) -> Result<CallToolResponse, ErrorData> {
            match request.name.as_ref() {
                "account_hook" => Ok(CallToolResult::structured(json!({"account":null})).into()),
                "sign_in_hook" => {
                    if context
                        .client_capabilities()
                        .and_then(|c| c.elicitation)
                        .and_then(|e| e.url)
                        .is_none()
                    {
                        return Err(ErrorData::invalid_request(
                            "no URL elicitation declared",
                            None,
                        ));
                    }
                    let request = serde_json::from_value(json!({
                        "method":"elicitation/create",
                        "params":{"mode":"url","message":"Open the device login","url":DEVICE_LOGIN_URL}
                    }))
                    .unwrap();
                    Ok(CallToolResponse::InputRequired(InputRequiredResult::new(
                        Some(BTreeMap::from([("login".into(), request)])),
                        None,
                    )))
                }
                _ => Err(ErrorData::invalid_params("unexpected tool", None)),
            }
        }
    }

    SignInProvider
        .serve(rmcp::transport::stdio())
        .await
        .unwrap()
        .waiting()
        .await
        .unwrap();
}

#[test]
fn sign_in_relays_url_from_rmcp_stdio_provider() {
    use conduit_shell::server::{ServingExecutor, serve};
    use rmcp::{
        ClientServiceExt,
        model::*,
        service::ClientLifecycleMode,
        transport::{
            StreamableHttpClientTransport,
            streamable_http_client::StreamableHttpClientTransportConfig,
        },
    };
    use tokio_util::sync::CancellationToken;

    let parent = tempfile::tempdir().unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let config = Config {
        engine_url: format!("http://{}/mcp", listener.local_addr().unwrap()),
        ..Config::default()
    };
    drop(listener);
    let shell = open(parent.path(), config);
    shell.submit(
        json!({"kind":"first-start","machine":conduit_shell::local_machine(BTreeMap::new()).unwrap()}),
        &MissingExecutor,
    ).unwrap();

    let repo = repository();
    let fixtures = repo.join("conformance/conventions/requests");
    let manifest_path = fixtures.join("sign-in-login-plain-hook/manifest.json");
    let documents_path = fixtures.join("sign-in-lapse-and-conflict/documents.json");
    let mut manifest = boundary::value(&manifest_path).unwrap();
    manifest["implementations"]["signed-server"]["command"] = json!([
        std::env::current_exe().unwrap(),
        "--exact",
        "rmcp_sign_in_provider",
        "--ignored",
        "--nocapture"
    ]);
    let spec = conduit_loader::Specification::read(&repo.join("spec")).unwrap();
    let project = conduit_loader::read_submitted_files(
        &[
            (
                "conduit.json".into(),
                serde_json::to_vec(&manifest).unwrap(),
            ),
            ("documents.json".into(), fs::read(documents_path).unwrap()),
        ],
        &spec,
    )
    .unwrap();
    let project = conduit_core::packages::Project {
        manifest: project.manifest.map(|doc| doc.root.to_json()),
        documents: project
            .documents
            .iter()
            .map(|doc| doc.root.to_json())
            .collect(),
        directory: None,
    };
    let root = id(&shell, "root");
    let state = shell.state().unwrap();
    let route = state
        .objects
        .iter()
        .find(|v| v["local_name"] == "management" && v["consumer"] == root)
        .unwrap();
    let reply = shell.submit(
        json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,"project":project,
            "request":{"sender":root,"caller":root,"conduit":route["id"],"target":route["target"],
                "operation":format!("{}#register.v1", shell.context.core_package)}}),
        &MissingExecutor,
    ).unwrap();
    assert_eq!(reply["accepted"], true, "{reply}");

    let runner = new_runner(&shell);
    let executor = Arc::new(ServingExecutor::new(runner.executor()));
    runner.set_completion_executor(Arc::downgrade(&(executor.clone() as Arc<dyn Executor>)));
    let instance = json!({"instance":{"kind":"Agent","name":"signed"}});
    let setup = invoke(
        &shell,
        executor.as_ref(),
        "root",
        "group.set-up.v1",
        instance.clone(),
    );
    assert_eq!(outcome(&shell, &setup), "succeeded", "{setup}");
    assert_eq!(runner.running_count(), 1);

    shell.issue_client_token(&root).unwrap();
    let token = fs::read_to_string(parent.path().join("store/client-tokens").join(root)).unwrap();
    let response = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let stop = CancellationToken::new();
            let task = tokio::spawn(serve(shell.clone(), executor, stop.clone()));
            let url = &shell.config.engine_url;
            let http = reqwest::Client::builder().no_proxy().build().unwrap();
            tokio::time::timeout(Duration::from_secs(10), async {
                while http.get(url).send().await.is_err() {
                    assert!(!task.is_finished(), "engine server exited before binding");
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            })
            .await
            .expect("engine server did not bind");
            let mut client = ClientConfig::default();
            client.capabilities.elicitation =
                Some(ElicitationCapability::new().with_url(UrlElicitationCapability::new()));
            let transport = StreamableHttpClientTransport::with_client(
                http,
                StreamableHttpClientTransportConfig::with_uri(url.as_str()).auth_header(token),
            );
            let client = client
                .serve_with_lifecycle(
                    transport,
                    ClientLifecycleMode::Discover {
                        preferred_versions: vec![ProtocolVersion::V_2026_07_28],
                    },
                )
                .await
                .unwrap();
            let request = CallToolRequestParams::new("group.sign-in.v1")
                .with_arguments(instance.as_object().unwrap().clone());
            let response = client.call_tool_once(request).await;
            client.cancel().await.unwrap();
            stop.cancel();
            task.await.unwrap().unwrap();
            response.unwrap()
        });
    if let CallToolResponse::Complete(result) = response {
        let reply = serde_json::to_value(result).unwrap();
        panic!(
            "expected a relayed sign-in URL, got outcome {} under {}",
            outcome(&shell, &reply),
            reply["structuredContent"]["diagnostics"][0]["ruleId"]
                .as_str()
                .unwrap_or("no rule")
        );
    }
    let CallToolResponse::InputRequired(result) = response else {
        panic!("expected an input-required sign-in result");
    };
    let requests = result.input_requests.unwrap();
    assert_eq!(requests.len(), 1);
    let request = serde_json::to_value(&requests["url"]).unwrap();
    assert_eq!(request["method"], "elicitation/create");
    assert_eq!(request["params"]["mode"], "url");
    assert_eq!(request["params"]["url"], DEVICE_LOGIN_URL);
}

// Completion transitions must pass through the serving decorator, even though the
// initial call returned Deferred on a different thread.
struct CompletionObserver(std::sync::atomic::AtomicUsize);
impl Executor for CompletionObserver {
    fn tools_changed(&self, _: &[Value]) -> conduit_shell::Result<()> {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
}
#[test]
fn deferred_setup_and_dispatch_reenter_the_completion_executor() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), false, "normal");
    let runner = new_runner(&shell);
    let observer = Arc::new(CompletionObserver(std::sync::atomic::AtomicUsize::new(0)));
    let executor: Arc<dyn Executor> = observer.clone();
    runner.set_completion_executor(Arc::downgrade(&executor));
    establish(&shell, &runner);
    until(|| observer.0.load(std::sync::atomic::Ordering::SeqCst) == 2);
    let reply = invoke(
        &shell,
        &runner,
        "agent-b",
        "project.file.read.v1",
        json!({"path":"hello.txt"}),
    );
    assert_eq!(outcome(&shell, &reply), "succeeded", "{reply}");
    until(|| observer.0.load(std::sync::atomic::Ordering::SeqCst) == 3);
}

#[test]
fn runner_exit_observes_owned_runs_as_stopped() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), false, "normal");
    let runner = new_runner(&shell);
    establish(&shell, &runner);
    assert_eq!(runner.running_count(), 2);
    drop(runner);
    for name in ["shared", "history"] {
        let reply = invoke(
            &shell,
            &MissingExecutor,
            "root",
            "root-workspace.inspect.v1",
            json!({"object":{"kind":"Resource","name":name}}),
        );
        assert_eq!(outcome(&shell, &reply), "succeeded");
        let current = reply["structuredContent"]["status"]["conditions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["type"] == "Running")
            .unwrap();
        assert_eq!(current["reason"], "RunStopped");
        assert_eq!(current["status"], "False");
        assert_eq!(reply["structuredContent"]["status"]["state"], "ready");
        let state = shell.state().unwrap();
        let object = state
            .objects
            .iter()
            .find(|o| o["name"] == name && o.get("spec").is_none())
            .unwrap();
        let row = conduit_shell::author::inspection_row(&state.objects, object, reply.clone());
        let ended = row["pastConditions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["condition"]["reason"] == "RunRunning")
            .unwrap();
        assert_eq!(ended["endedAt"], current["lastTransitionTime"]);
        assert!(ended["endedAt"].is_string());
    }
}

fn running_condition(shell: &Shell, name: &str) -> Value {
    let reply = invoke(
        shell,
        &MissingExecutor,
        "root",
        "root-workspace.inspect.v1",
        json!({"object":{"kind":"Resource","name":name}}),
    );
    assert_eq!(outcome(shell, &reply), "succeeded");
    reply["structuredContent"]["status"]["conditions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["type"] == "Running")
        .unwrap()
        .clone()
}

#[test]
fn shutdown_reports_a_run_that_traps_term_stopped_after_kill() {
    let parent = tempfile::tempdir().unwrap();
    let grace = Duration::from_secs(2);
    let shell = open(
        parent.path(),
        Config {
            stop_grace: grace,
            ..Config::default()
        },
    );
    register(&shell, parent.path(), false, "trap-term");
    let runner = new_runner(&shell);
    establish(&shell, &runner);
    let before = chrono::Utc::now();
    let started = Instant::now();
    runner.shutdown();
    assert!(started.elapsed() >= grace);
    for name in ["shared", "history"] {
        assert!(parent.path().join(name).join("term").is_file());
        let pid = fs::read_to_string(parent.path().join(name).join("pid"))
            .unwrap()
            .parse::<i32>()
            .unwrap();
        assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ESRCH)
        );
        let current = running_condition(&shell, name);
        assert_eq!(current["status"], "False");
        assert_eq!(current["reason"], "RunStopped");
        let transitioned =
            chrono::DateTime::parse_from_rfc3339(current["lastTransitionTime"].as_str().unwrap())
                .unwrap();
        assert!(transitioned.timestamp_millis() >= (before + grace).timestamp_millis());
    }
    assert!(runner.diagnostics().is_empty());
    runner.shutdown();
    assert!(runner.diagnostics().is_empty());
}

#[test]
fn shutdown_reports_a_run_that_exits_on_term_before_the_grace() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(
        parent.path(),
        Config {
            stop_grace: Duration::from_secs(120),
            ..Config::default()
        },
    );
    register(&shell, parent.path(), false, "normal");
    let runner = new_runner(&shell);
    establish(&shell, &runner);
    let started = Instant::now();
    runner.shutdown();
    assert!(started.elapsed() < Duration::from_secs(60));
    for name in ["shared", "history"] {
        let current = running_condition(&shell, name);
        assert_eq!(current["status"], "False");
        assert_eq!(current["reason"], "RunStopped");
    }
    assert!(runner.diagnostics().is_empty());
}

// Construct before starting either fixture, so even a failed setup assertion cleans up
// every escaped process that managed to publish its PID.
struct EscapedFixtures(PathBuf);
impl Drop for EscapedFixtures {
    fn drop(&mut self) {
        for name in ["shared", "history"] {
            if let Ok(pid) = fs::read_to_string(self.0.join(name).join("pid"))
                && let Ok(pid) = pid.parse::<i32>()
            {
                unsafe { libc::kill(pid, libc::SIGKILL) };
            }
        }
    }
}

#[test]
fn shutdown_submits_nothing_for_a_group_not_seen_absent() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(
        parent.path(),
        Config {
            stop_grace: Duration::from_secs(1),
            ..Config::default()
        },
    );
    register(&shell, parent.path(), false, "zombie-in-group");
    let runner = new_runner(&shell);
    let _fixtures = EscapedFixtures(parent.path().into());
    establish(&shell, &runner);
    runner.shutdown();
    let diagnostics = runner.diagnostics();
    assert_eq!(diagnostics.len(), 2);
    for name in ["shared", "history"] {
        let current = running_condition(&shell, name);
        assert_eq!(current["status"], "True");
        assert_eq!(current["reason"], "RunRunning");
        let expected = format!(
            "the run of instance {} was not seen to end within 3 seconds of KILL, so its stop was not recorded",
            id(&shell, name),
        );
        assert_eq!(
            diagnostics.iter().filter(|line| **line == expected).count(),
            1
        );
    }
    runner.shutdown();
    assert_eq!(runner.diagnostics(), diagnostics);
}

#[test]
fn serve_signal_during_setup_ends_run_and_starts_no_later_part() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), false, "no-initialize");
    drop(shell);
    struct Server(std::process::Child);
    impl Drop for Server {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let mut server = Server(
        Command::new(env!("CARGO_BIN_EXE_conduit"))
            .env("CONDUIT_CONFIG_DIR", parent.path().join("config"))
            .arg("serve")
            .arg("--store")
            .arg(parent.path().join("store"))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(fs::File::create(parent.path().join("serve.stderr")).unwrap())
            .spawn()
            .unwrap(),
    );
    let startup = Instant::now();
    while !["shared", "history"].iter().any(|name| {
        fs::read_to_string(parent.path().join(name).join("pid"))
            .is_ok_and(|s| s.parse::<i32>().is_ok())
    }) {
        assert!(
            server.0.try_wait().unwrap().is_none(),
            "serve exited during setup: {}",
            fs::read_to_string(parent.path().join("serve.stderr")).unwrap()
        );
        assert!(
            startup.elapsed() < Duration::from_secs(600),
            "serve never started the first setup run"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    let first = ["shared", "history"]
        .into_iter()
        .find(|name| {
            fs::read_to_string(parent.path().join(name).join("pid"))
                .is_ok_and(|s| s.parse::<i32>().is_ok())
        })
        .unwrap();
    let pid: i32 = fs::read_to_string(parent.path().join(first).join("pid"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        unsafe { libc::kill(server.0.id() as i32, libc::SIGTERM) },
        0
    );
    let shutdown_started = Instant::now();
    let mut status = None;
    until_for(Duration::from_secs(180), || {
        status = server.0.try_wait().unwrap();
        status.is_some()
    });
    eprintln!(
        "SIGTERM shutdown completed in {:?}",
        shutdown_started.elapsed()
    );
    assert!(status.unwrap().success());
    until(|| unsafe { libc::kill(pid, 0) } != 0);
    for name in ["shared", "history"] {
        if name != first {
            assert!(!parent.path().join(name).join("pid").exists());
        }
    }
    let shell = open(parent.path(), Config::default());
    assert!(
        shell
            .state()
            .unwrap()
            .invocations
            .iter()
            .all(|v| v["state"] != "in-progress")
    );
}

#[test]
fn keyed_commands_read_discovery_once_per_run_and_withhold_undeclared_calls() {
    let extension = "io.github.shrav-k.conduit/implementation-interface";
    for (mode, advertised) in [("discovery-v0", true), ("discovery-missing", false)] {
        let parent = tempfile::tempdir().unwrap();
        let shell = open(parent.path(), Config::default());
        register(&shell, parent.path(), false, mode);
        let runner = new_runner(&shell);
        establish(&shell, &runner);

        for _ in 0..2 {
            let reply = invoke(
                &shell,
                &runner,
                "agent-b",
                "project.file.read.v1",
                json!({"path":"hello.txt"}),
            );
            if advertised {
                assert_eq!(outcome(&shell, &reply), "succeeded", "{reply}");
            } else {
                assert_eq!(
                    reply["_meta"]["io.github.shrav-k.conduit/reason"],
                    "unavailable"
                );
            }
        }
        let wire = lines(&parent.path().join("shared/wire.jsonl"));
        let traffic: Vec<_> = wire
            .iter()
            .filter(|v| matches!(v["method"].as_str(), Some("server/discover" | "tools/call")))
            .collect();
        assert_eq!(traffic[0]["method"], "server/discover");
        assert_eq!(traffic.len(), if advertised { 3 } else { 1 });
        for call in &traffic[1..] {
            assert_eq!(call["method"], "tools/call");
            assert_eq!(
                call["params"]["_meta"]["io.modelcontextprotocol/clientCapabilities"]["extensions"]
                    [extension]["revision"],
                "v0"
            );
        }
        if !advertised {
            let state = shell.state().unwrap();
            let retained: Vec<_> = state
                .records
                .iter()
                .filter(|record| {
                    record["data"]["operation"]
                        .as_str()
                        .is_some_and(|op| op.ends_with("#file.read.v1"))
                })
                .collect();
            assert!(
                !retained
                    .iter()
                    .any(|record| record["type"] == "invocation-dispatched")
            );
            assert_eq!(
                retained
                    .iter()
                    .filter(|record| {
                        record["type"] == "request-ended"
                            && record["data"]["missingExtension"] == extension
                    })
                    .count(),
                2
            );
        }
    }
}

#[test]
fn plain_wire_environment_workdir_records_and_restart() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), false, "normal");
    let runner = new_runner(&shell);
    establish(&shell, &runner);
    assert_eq!(runner.running_count(), 2);
    let inspection = invoke(
        &shell,
        &runner,
        "root",
        "root-workspace.inspect.v1",
        json!({"object":{"kind":"Resource","name":"shared"}}),
    );
    assert_eq!(outcome(&shell, &inspection), "succeeded");
    assert!(
        inspection["structuredContent"]["declared"]
            .get("past_conditions")
            .is_none()
    );
    assert_eq!(
        inspection["structuredContent"]["status"]["conditions"][0]["reason"],
        "RunRunning"
    );
    let reply = invoke(
        &shell,
        &runner,
        "agent-b",
        "project.file.read.v1",
        json!({"path":"hello.txt"}),
    );
    assert_eq!(outcome(&shell, &reply), "succeeded", "{reply}");
    let wire = lines(&parent.path().join("shared/wire.jsonl"));
    let init = wire.iter().find(|v| v["method"] == "initialize").unwrap();
    assert!(init["params"]["capabilities"].get("roots").is_none());
    let call = wire.iter().find(|v| v["method"] == "tools/call").unwrap();
    assert_eq!(
        call["params"],
        json!({"name":"read_text_file","arguments":{"path":"hello.txt"}})
    );
    let environment: Value =
        serde_json::from_slice(&fs::read(parent.path().join("shared/environment.json")).unwrap())
            .unwrap();
    for key in environment.as_object().unwrap().keys() {
        // Apple's test harness adds this in-process. The exec-boundary test below
        // independently checks the environment delivered by the production spawn path.
        if cfg!(target_os = "macos") && key == "__CF_USER_TEXT_ENCODING" {
            continue;
        }
        assert!(
            [
                "HOME",
                "LOGNAME",
                "PATH",
                "SHELL",
                "TERM",
                "USER",
                "TEST_ROOT",
                "CONDUIT_URL",
                "CONDUIT_IDENTITY_DIR",
                &conduit_core::run_variables()["stateDirVariable"]
            ]
            .contains(&key.as_str()),
            "unexpected environment name: {key}"
        );
    }
    assert!(environment.get("GITHUB_TOKEN").is_none());
    assert_eq!(environment["CONDUIT_URL"], shell.config.engine_url);
    let state_variable = &conduit_core::run_variables()["stateDirVariable"];
    let state_dir = PathBuf::from(environment[state_variable].as_str().unwrap());
    assert_eq!(
        state_dir,
        parent
            .path()
            .join("store/instances")
            .join(id(&shell, "shared"))
    );
    assert_eq!(
        fs::metadata(&state_dir).unwrap().permissions().mode() & 0o777,
        0o700
    );
    assert_eq!(fs::read_dir(&state_dir).unwrap().count(), 0);
    let history_env: Value =
        serde_json::from_slice(&fs::read(parent.path().join("history/environment.json")).unwrap())
            .unwrap();
    assert_ne!(environment[state_variable], history_env[state_variable]);
    fs::write(state_dir.join("synthetic-state"), "persists").unwrap();
    let cwd: Value =
        serde_json::from_slice(&fs::read(parent.path().join("shared/cwd.json")).unwrap()).unwrap();
    assert_eq!(cwd["mode"], 0o700);
    assert_eq!(cwd["empty"], true);
    assert!(
        Path::new(cwd["path"].as_str().unwrap())
            .starts_with(parent.path().join("store/work").canonicalize().unwrap())
    );
    let state = shell.state().unwrap();
    let invocation = state
        .invocations
        .iter()
        .find(|v| {
            v["target"] == id(&shell, "shared")
                && v["operation"] == "example.org/s1/filesystem#file.read.v1"
        })
        .unwrap();
    assert_eq!(invocation["attempt"], 1);
    assert_eq!(
        state
            .records
            .iter()
            .filter(
                |r| r["type"] == "invocation-ended" && r["data"]["invocation"] == invocation["id"]
            )
            .count(),
        1
    );
    let pid = fs::read_to_string(parent.path().join("shared/pid"))
        .unwrap()
        .parse::<i32>()
        .unwrap();
    drop(shell);
    until(|| unsafe { libc::kill(pid, 0) } != 0); // Runner may still be held.
    drop(runner);
    let shell = open(parent.path(), Config::default());
    let runner = new_runner(&shell);
    let inspect = json!({"kind":"builtin-runner","subject":id(&shell,"shared"),"operation":format!("{}#inspect-run.v1",shell.context.core_package)});
    assert!(
        matches!(runner.builtin_runner(&inspect,None).unwrap(),Execution::SetupReport(v) if v["state"]=="stopped")
    );
    assert_eq!(
        outcome(&shell, &setup(&shell, &runner, "shared")),
        "succeeded"
    );
    let restarted_env: Value =
        serde_json::from_slice(&fs::read(parent.path().join("shared/environment.json")).unwrap())
            .unwrap();
    assert_eq!(restarted_env[state_variable], environment[state_variable]);
    assert_eq!(
        fs::read_to_string(state_dir.join("synthetic-state")).unwrap(),
        "persists"
    );
    let retired = invoke(
        &shell,
        &runner,
        "root",
        "root-workspace.retire.v1",
        json!({"instance":{"kind":"Resource","name":"shared"}}),
    );
    assert_eq!(outcome(&shell, &retired), "succeeded", "{retired}");
    assert!(!state_dir.exists());
}

// The wire shows the deadline's cancellation sent, for the held request and with its reason, and
// before the next call on the run. That it comes before the missing report rests on the code
// order in `Inner::dispatch_later` (src/runner.rs:1151-1187), which awaits the send before it
// calls `shell.complete`.
#[test]
fn dispatch_deadline_cancels_the_held_request_before_the_next_call() {
    for mode in ["hang", "repair-list-hang"] {
        let parent = tempfile::tempdir().unwrap();
        let shell = open(
            parent.path(),
            Config {
                call_timeout: Duration::from_secs(1),
                ..Config::default()
            },
        );
        register(&shell, parent.path(), false, mode);
        let runner = new_runner(&shell);
        assert_eq!(
            outcome(&shell, &setup(&shell, &runner, "workspace")),
            "succeeded"
        );
        assert_eq!(
            outcome(&shell, &setup(&shell, &runner, "shared")),
            "succeeded"
        );
        setup(&shell, &runner, "agent-b");
        let (tool, args, held_method) = if mode == "hang" {
            (
                "project.file.write.v1",
                json!({"path":"x","content":"y"}),
                "tools/call",
            )
        } else {
            ("project.file.read.v1", json!({"path":"x"}), "tools/list")
        };
        let reply = shell
            .begin(input(&shell, "agent-b", tool, args), &runner)
            .unwrap()
            .wait()
            .unwrap();
        assert_eq!(outcome(&shell, &reply), "unknown", "{mode}: {reply}");

        let second = shell
            .begin(
                input(
                    &shell,
                    "agent-b",
                    "project.file.read.v1",
                    json!({"path":"y"}),
                ),
                &runner,
            )
            .unwrap();
        until(|| call_count(parent.path()) == 2);
        let wire = lines(&parent.path().join("shared/wire.jsonl"));
        let cancellation = wire
            .iter()
            .position(|v| v["method"] == "notifications/cancelled")
            .expect("deadline must send notifications/cancelled");
        let held = wire.iter().find(|v| v["method"] == held_method).unwrap();
        assert_eq!(wire[cancellation]["params"]["requestId"], held["id"]);
        assert_eq!(
            wire[cancellation]["params"]["reason"],
            "call deadline exceeded"
        );
        let calls = wire
            .iter()
            .enumerate()
            .filter(|(_, v)| v["method"] == "tools/call")
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        assert_eq!(calls.len(), 2, "{mode}");
        assert!(cancellation < calls[1], "{mode}");
        assert_eq!(
            wire.iter().filter(|v| v["method"] == "tools/list").count(),
            usize::from(mode == "repair-list-hang"),
            "{mode}"
        );
        second.wait().unwrap();
    }
}

// The run closed its standard input while holding the call, so the deadline's cancellation
// cannot be written. The call is still reported missing, and one fixed line names the invocation.
#[test]
fn dispatch_deadline_reports_a_cancellation_that_could_not_be_sent() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(
        parent.path(),
        Config {
            call_timeout: Duration::from_secs(1),
            ..Config::default()
        },
    );
    register(&shell, parent.path(), false, "hang-closed-stdin");
    let runner = new_runner(&shell);
    assert_eq!(
        outcome(&shell, &setup(&shell, &runner, "workspace")),
        "succeeded"
    );
    assert_eq!(
        outcome(&shell, &setup(&shell, &runner, "shared")),
        "succeeded"
    );
    setup(&shell, &runner, "agent-b");
    let ticket = shell
        .begin(
            input(
                &shell,
                "agent-b",
                "project.file.write.v1",
                json!({"path":"x","content":"y"}),
            ),
            &runner,
        )
        .unwrap();
    let invocation = ticket.invocation().unwrap().to_owned();
    let reply = ticket.wait().unwrap();
    assert_eq!(outcome(&shell, &reply), "unknown", "{reply}");
    assert_eq!(
        runner.diagnostics(),
        [format!(
            "the call of invocation {invocation} passed its deadline and its cancellation could not be sent"
        )]
    );
    assert_eq!(call_count(parent.path()), 1);
}

#[test]
fn implementation_input_request_reaches_the_core_and_its_retry_is_sent_as_written() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), false, "input-required");
    let runner = new_runner(&shell);
    for name in ["workspace", "shared"] {
        let reply = setup(&shell, &runner, name);
        assert_eq!(outcome(&shell, &reply), "succeeded", "{name}: {reply}");
    }
    setup(&shell, &runner, "agent-b");

    let ticket = shell
        .begin(
            input(
                &shell,
                "agent-b",
                "project.file.read.v1",
                json!({"path":"x"}),
            ),
            &runner,
        )
        .unwrap();
    let invocation = ticket.invocation().unwrap().to_owned();
    let reply = ticket.wait().unwrap();
    assert_eq!(outcome(&shell, &reply), "unknown", "{reply}");
    assert!(
        reply["structuredContent"]["diagnostics"]
            .as_array()
            .is_some_and(|diagnostics| diagnostics.iter().any(|diagnostic| {
                diagnostic["ruleId"] == "IFC-016"
                    && diagnostic["message"]["text"]
                        .as_str()
                        .is_some_and(|text| text.contains("the instance's Type states no signIn"))
            })),
        "{reply}"
    );
    let reply_text = reply.to_string();
    assert!(!reply_text.contains("impl-state-1"));
    assert!(!reply_text.contains("example.org/authorize"));

    let wire_path = parent.path().join("shared/wire.jsonl");
    let wire = lines(&wire_path);
    let first = wire.iter().find(|v| v["method"] == "tools/call").unwrap();
    assert_eq!(
        first["params"],
        json!({"name":"read_text_file","arguments":{"path":"x"}})
    );
    let protocol_version = wire
        .iter()
        .find(|v| v["method"] == "initialize")
        .map_or(&Value::Null, |v| &v["params"]["protocolVersion"]);
    eprintln!("initialize protocolVersion: {protocol_version}");

    let mut params = first["params"].clone();
    params["requestState"] = json!("impl-state-1");
    params["inputResponses"] = json!({"sign-in":{"action":"accept"}});
    let retry = json!({
        "kind": "dispatch",
        "invocation": invocation,
        "attempt": 1,
        "target": id(&shell, "shared"),
        "call": {"method":"tools/call","params":params}
    });
    assert!(matches!(
        runner.dispatch(&retry).unwrap(),
        Execution::Deferred
    ));
    until(|| call_count(parent.path()) == 2);
    let wire = lines(&wire_path);
    let second = wire
        .iter()
        .filter(|v| v["method"] == "tools/call")
        .nth(1)
        .unwrap();
    assert_eq!(second["params"], params);
}

#[test]
fn deadlines_exit_repair_write_and_cancel() {
    for mode in [
        "no-initialize",
        "hang",
        "exit",
        "repair",
        "repair-list-error",
        "error-write",
        "cancel",
    ] {
        let parent = tempfile::tempdir().unwrap();
        let shell = open(
            parent.path(),
            Config {
                initialize_timeout: Duration::from_secs(1),
                call_timeout: if mode == "cancel" {
                    Duration::from_secs(30)
                } else {
                    Duration::from_millis(400)
                },
                ..Config::default()
            },
        );
        register(&shell, parent.path(), false, mode);
        let runner = new_runner(&shell);
        assert_eq!(
            outcome(&shell, &setup(&shell, &runner, "workspace")),
            "succeeded"
        );
        let reply = setup(&shell, &runner, "shared");
        if mode == "no-initialize" {
            assert_eq!(outcome(&shell, &reply), "failed");
            assert_eq!(runner.running_count(), 0);
            continue;
        }
        assert_eq!(outcome(&shell, &reply), "succeeded", "{reply}");
        setup(&shell, &runner, "agent-b");
        let write = mode == "error-write";
        let tool = if write {
            "project.file.write.v1"
        } else {
            "project.file.read.v1"
        };
        let args = if write {
            json!({"path":"x","content":"y"})
        } else {
            json!({"path":"x"})
        };
        let ticket = shell
            .begin(input(&shell, "agent-b", tool, args), &runner)
            .unwrap();
        if mode == "cancel" {
            until(|| call_count(parent.path()) == 1);
            shell
                .submit(
                    json!({"kind":"cancel","invocation":ticket.invocation().unwrap()}),
                    &runner,
                )
                .unwrap();
            until(|| {
                lines(&parent.path().join("shared/wire.jsonl"))
                    .iter()
                    .any(|v| v["method"] == "notifications/cancelled")
            });
            let wire = lines(&parent.path().join("shared/wire.jsonl"));
            assert_eq!(
                wire.iter().find(|v| v["method"] == "tools/call").unwrap()["id"],
                wire.iter()
                    .find(|v| v["method"] == "notifications/cancelled")
                    .unwrap()["params"]["requestId"]
            );
        }
        let reply = ticket.wait().unwrap();
        assert_eq!(
            outcome(&shell, &reply),
            if mode == "repair" {
                "succeeded"
            } else {
                "unknown"
            },
            "{mode}: {reply}"
        );
        let wire = lines(&parent.path().join("shared/wire.jsonl"));
        assert_eq!(
            wire.iter().filter(|v| v["method"] == "tools/call").count(),
            if mode == "repair" { 2 } else { 1 }
        );
        assert_eq!(
            wire.iter().filter(|v| v["method"] == "tools/list").count(),
            usize::from(mode.starts_with("repair"))
        );
        if mode == "exit" {
            let intent = json!({"subject":id(&shell,"shared"),"operation":format!("{}#inspect-run.v1",shell.context.core_package)});
            assert!(
                matches!(runner.builtin_runner(&intent,None).unwrap(),Execution::SetupReport(v) if v["state"]=="stopped")
            );
        }
    }
}

#[test]
fn cancel_setup_initializing_starts_no_later_part() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), false, "no-initialize");
    let runner = new_runner(&shell);
    setup(&shell, &runner, "workspace");
    let ticket = shell
        .begin(
            input(
                &shell,
                "root",
                "root-workspace.set-up.v1",
                json!({"instance":{"kind":"Resource","name":"shared"}}),
            ),
            &runner,
        )
        .unwrap();
    until(|| {
        fs::read_to_string(parent.path().join("shared/pid")).is_ok_and(|s| s.parse::<i32>().is_ok())
    });
    let pid = fs::read_to_string(parent.path().join("shared/pid"))
        .unwrap()
        .parse::<i32>()
        .unwrap();
    shell
        .submit(
            json!({"kind":"cancel","invocation":ticket.invocation().unwrap()}),
            &runner,
        )
        .unwrap();
    assert_eq!(outcome(&shell, &ticket.wait().unwrap()), "unknown");
    until(|| unsafe { libc::kill(pid, 0) } != 0);
    assert_eq!(runner.running_count(), 0);
    assert!(!parent.path().join("history/pid").exists());
}

#[test]
fn real_npm_s1() {
    if std::env::var_os("CONDUIT_TEST_NPM").is_none() {
        return;
    }
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), true, "normal");
    let runner = new_runner(&shell);
    establish(&shell, &runner);
    assert_eq!(runner.running_count(), 2);
    let file = parent.path().join("shared/hello.txt");
    fs::write(&file, "before").unwrap();
    let reply = invoke(
        &shell,
        &runner,
        "agent-b",
        "project.file.read.v1",
        json!({"path":file}),
    );
    assert_eq!(
        outcome(&shell, &reply),
        "succeeded",
        "{reply}; {:?}",
        runner.diagnostics()
    );
    assert!(reply["content"].to_string().contains("before"));
    let reply = invoke(
        &shell,
        &runner,
        "agent-b",
        "project.file.write.v1",
        json!({"path":file,"content":"after"}),
    );
    assert_eq!(outcome(&shell, &reply), "succeeded", "{reply}");
    assert_eq!(fs::read_to_string(&file).unwrap(), "after");
    let outside = parent.path().join("outside.txt");
    fs::write(&outside, "outside").unwrap();
    let reply = invoke(
        &shell,
        &runner,
        "agent-b",
        "project.file.read.v1",
        json!({"path":outside}),
    );
    assert!(
        reply["content"].to_string().contains("Access denied"),
        "{reply}"
    );
    let history = parent.path().join("history/post.txt");
    let reply = invoke(
        &shell,
        &runner,
        "agent-b",
        "history.file.write.v1",
        json!({"path":history,"content":"history bytes"}),
    );
    assert_eq!(outcome(&shell, &reply), "succeeded");
    assert_eq!(fs::read_to_string(history).unwrap(), "history bytes");
    let state = shell.state().unwrap();
    for tool in ["file.read.v1", "file.write.v1"] {
        for invocation in state
            .invocations
            .iter()
            .filter(|v| v["operation"] == format!("example.org/s1/filesystem#{tool}"))
        {
            let records: Vec<_> = state
                .records
                .iter()
                .filter(|r| r["data"]["invocation"] == invocation["id"])
                .collect();
            let decision = records
                .iter()
                .position(|r| r["type"] == "access-decision")
                .unwrap();
            let dispatch = records
                .iter()
                .position(|r| r["type"] == "invocation-dispatched")
                .unwrap();
            let end = records
                .iter()
                .position(|r| r["type"] == "invocation-ended")
                .unwrap();
            assert!(decision < dispatch && dispatch < end);
            assert_eq!(
                records
                    .iter()
                    .filter(|r| r["type"] == "invocation-ended")
                    .count(),
                1
            );
        }
    }
    println!(
        "real npm S1: two initialized runs, read/write bytes, history write, outside-root server refusal, ordered records"
    );
}

#[test]
#[ignore = "engine process fixture, spawned to test SIGKILL"]
fn engine_child() {
    let parent = PathBuf::from(std::env::var("WO021_ENGINE_TEST_PARENT").unwrap());
    let shell = open(&parent, Config::default());
    register(&shell, &parent, false, "descendant");
    let runner = new_runner(&shell);
    setup(&shell, &runner, "workspace");
    assert_eq!(
        outcome(&shell, &setup(&shell, &runner, "shared")),
        "succeeded"
    );
    fs::write(parent.join("ready"), b"ready").unwrap();
    loop {
        std::thread::park();
    }
}
fn entry_names(path: &Path) -> std::collections::BTreeSet<String> {
    fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect()
}
fn canonical_uuid(name: &str) -> bool {
    uuid::Uuid::parse_str(name).is_ok_and(|id| id.to_string() == name)
}

#[test]
fn a_killed_run_leaves_nothing_after_the_next_open() {
    let parent = tempfile::tempdir().unwrap();
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "engine_child", "--ignored", "--nocapture"])
        .env("WO021_ENGINE_TEST_PARENT", parent.path())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    until_for(Duration::from_secs(60), || {
        parent.path().join("ready").is_file()
    });
    let store = parent.path().join("store");
    for name in ["runs", "work", "run-groups"] {
        assert!(!entry_names(&store.join(name)).is_empty());
    }
    let pids: Vec<i32> = ["pid", "descendant-pid"]
        .into_iter()
        .map(|name| {
            fs::read_to_string(parent.path().join("shared").join(name))
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    child.kill().unwrap();
    child.wait().unwrap();
    for pid in pids {
        until_for(
            Duration::from_secs(60),
            || unsafe { libc::kill(pid, 0) } != 0,
        );
    }
    let old_program = store
        .join("programs")
        .join(uuid::Uuid::new_v4().to_string());
    fs::create_dir_all(old_program.join("bin")).unwrap();
    fs::write(old_program.join("bin/worker"), b"old program").unwrap();
    fs::create_dir_all(store.join("programs/kept")).unwrap();
    fs::create_dir_all(
        store
            .join("http-bridges")
            .join(uuid::Uuid::new_v4().to_string()),
    )
    .unwrap();
    let outside = parent.path().join("outside");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("kept"), b"outside").unwrap();
    std::os::unix::fs::symlink(
        &outside,
        store.join("work").join(uuid::Uuid::new_v4().to_string()),
    )
    .unwrap();
    let old_identity = store.join("runs").join(uuid::Uuid::new_v4().to_string());
    fs::create_dir(&old_identity).unwrap();
    let outside_file = parent.path().join("outside-file");
    fs::write(&outside_file, b"file").unwrap();
    std::os::unix::fs::symlink(&outside_file, old_identity.join("link")).unwrap();
    let id = uuid::Uuid::parse_str("abcdefab-cdef-4abc-8def-abcdefabcdef").unwrap();
    let other_names = [id.to_string().to_uppercase(), id.simple().to_string()];
    for name in &other_names {
        fs::create_dir(store.join("work").join(name)).unwrap();
    }
    let instances = entry_names(&store.join("instances"));
    let _shell = open(parent.path(), Config::default());
    for name in ["runs", "work", "run-groups", "http-bridges"] {
        assert!(
            !entry_names(&store.join(name))
                .iter()
                .any(|name| canonical_uuid(name)),
            "leftover in {name}"
        );
    }
    assert_eq!(
        entry_names(&store.join("work")),
        other_names.into_iter().collect()
    );
    assert_eq!(
        entry_names(&store.join("programs")),
        ["kept".to_owned()].into_iter().collect()
    );
    assert_eq!(fs::read(outside.join("kept")).unwrap(), b"outside");
    assert_eq!(fs::read(outside_file).unwrap(), b"file");
    assert_eq!(entry_names(&store.join("instances")), instances);
}

#[test]
fn a_running_runs_directories_are_never_removed() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), false, "success");
    let runner = new_runner(&shell);
    establish(&shell, &runner);
    let paths = |name: &str| {
        let root = parent.path().join(name);
        let cwd: Value = serde_json::from_slice(&fs::read(root.join("cwd.json")).unwrap()).unwrap();
        let environment: Value =
            serde_json::from_slice(&fs::read(root.join("environment.json")).unwrap()).unwrap();
        [
            PathBuf::from(cwd["path"].as_str().unwrap()),
            PathBuf::from(environment["CONDUIT_IDENTITY_DIR"].as_str().unwrap()),
        ]
    };
    let shared = paths("shared");
    let history = paths("history");
    let reply = invoke(
        &shell,
        &runner,
        "root",
        "root-workspace.inspect.v1",
        json!({"object":{"kind":"Resource","name":"shared"}}),
    );
    assert_eq!(outcome(&shell, &reply), "succeeded");
    let reply = invoke(
        &shell,
        &runner,
        "agent-b",
        "project.file.read.v1",
        json!({"path":"hello.txt"}),
    );
    assert_eq!(outcome(&shell, &reply), "succeeded");
    let second = new_runner(&shell);
    assert!(shared.iter().chain(&history).all(|path| path.exists()));
    drop(second);
    let groups = parent.path().join("store/run-groups");
    let count = entry_names(&groups).len();
    let stop = json!({"subject":id(&shell,"history"),"operation":format!("{}#stop.v1",shell.context.core_package)});
    runner.builtin_runner(&stop, None).unwrap();
    until_for(Duration::from_secs(60), || {
        history.iter().all(|path| !path.exists()) && entry_names(&groups).len() == count - 1
    });
    assert!(shared.iter().all(|path| path.exists()));
    let inspect = json!({"subject":id(&shell,"shared"),"operation":format!("{}#inspect-run.v1",shell.context.core_package)});
    assert!(
        matches!(runner.builtin_runner(&inspect,None).unwrap(),Execution::SetupReport(v) if v["state"]=="running")
    );
    drop(runner);
    for name in ["runs", "work", "run-groups"] {
        assert!(
            entry_names(&parent.path().join("store").join(name)).is_empty(),
            "leftover in {name}"
        );
    }
}

#[test]
fn replaced_run_entry_never_removes_a_live_runs_work() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), false, "normal");
    let runner = new_runner(&shell);
    for name in ["workspace", "shared", "history"] {
        assert_eq!(outcome(&shell, &setup(&shell, &runner, name)), "succeeded");
    }
    let work = |name: &str| {
        let cwd: Value =
            serde_json::from_slice(&fs::read(parent.path().join(name).join("cwd.json")).unwrap())
                .unwrap();
        PathBuf::from(cwd["path"].as_str().unwrap())
    };
    let ended = work("shared");
    let live = work("history");
    fs::write(live.join("marker"), b"live run").unwrap();
    fs::rename(&ended, ended.parent().unwrap().join("kept-ended")).unwrap();
    fs::rename(&live, &ended).unwrap();
    let stop = json!({"subject":id(&shell,"shared"),"operation":format!("{}#stop.v1",shell.context.core_package)});
    runner.builtin_runner(&stop, None).unwrap();
    until_for(Duration::from_secs(60), || {
        entry_names(&parent.path().join("store/run-groups")).len() == 1
    });
    assert!(
        ended.join("marker").exists(),
        "stopping one run removed a live run's replacement work directory"
    );
    let inspect = json!({"subject":id(&shell,"history"),"operation":format!("{}#inspect-run.v1",shell.context.core_package)});
    assert!(
        matches!(runner.builtin_runner(&inspect,None).unwrap(),Execution::SetupReport(v) if v["state"]=="running")
    );
    assert_eq!(fs::read(ended.join("marker")).unwrap(), b"live run");
    fs::rename(&ended, &live).unwrap();
}

#[test]
fn killed_engine_ends_run_and_descendant_and_does_not_pass_token() {
    let parent = tempfile::tempdir().unwrap();
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "engine_child", "--ignored", "--nocapture"])
        .env("WO021_ENGINE_TEST_PARENT", parent.path())
        .env("GITHUB_TOKEN", "synthetic-engine-token-sentinel")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    // FST-002's expanded first-start publication and setup happen in the child.
    // Startup and post-kill waits guard progress, without asserting its speed.
    until_for(Duration::from_secs(600), || {
        parent.path().join("ready").is_file()
    });
    let environment: Value =
        serde_json::from_slice(&fs::read(parent.path().join("shared/environment.json")).unwrap())
            .unwrap();
    assert!(environment.get("GITHUB_TOKEN").is_none());
    let pids: Vec<i32> = ["pid", "descendant-pid"]
        .into_iter()
        .map(|name| {
            fs::read_to_string(parent.path().join("shared").join(name))
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    child.kill().unwrap();
    child.wait().unwrap();
    for pid in pids {
        until(|| unsafe { libc::kill(pid, 0) } != 0);
    }
    let shell = open(parent.path(), Config::default());
    let runner = new_runner(&shell);
    let inspect = json!({"subject":id(&shell,"shared"),"operation":format!("{}#inspect-run.v1",shell.context.core_package)});
    assert!(
        matches!(runner.builtin_runner(&inspect,None).unwrap(),Execution::SetupReport(v) if v["state"]=="stopped")
    );
}

#[test]
fn unsupported_registry_reports_its_name_without_starting() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), true, "unsupported-registry");
    let runner = new_runner(&shell);
    setup(&shell, &runner, "workspace");
    let reply = setup(&shell, &runner, "shared");
    assert_eq!(outcome(&shell, &reply), "failed");
    assert!(
        runner
            .diagnostics()
            .iter()
            .any(|d| d.contains("unsupported registryType") && d.contains("pypi"))
    );
    assert_eq!(runner.running_count(), 0);
}
#[test]
fn real_npm_hash_mismatch_refuses_before_extraction() {
    if std::env::var_os("CONDUIT_TEST_NPM").is_none() {
        return;
    }
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), true, "hash-mismatch");
    let runner = new_runner(&shell);
    setup(&shell, &runner, "workspace");
    let reply = setup(&shell, &runner, "shared");
    assert_eq!(
        outcome(&shell, &reply),
        "failed",
        "{reply}; {:?}",
        runner.diagnostics()
    );
    assert!(
        runner
            .diagnostics()
            .iter()
            .any(|d| d.contains("SHA-256 mismatch")),
        "{:?}",
        runner.diagnostics()
    );
    assert_eq!(runner.running_count(), 0);
    assert!(
        !parent
            .path()
            .join("store/npm")
            .join("0".repeat(64))
            .join("package")
            .exists()
    );
}

#[test]
fn dropping_runner_during_initialize_ends_group_with_shell_still_open() {
    let parent = tempfile::tempdir().unwrap();
    let shell = open(parent.path(), Config::default());
    register(&shell, parent.path(), false, "no-initialize");
    let runner = new_runner(&shell);
    setup(&shell, &runner, "workspace");
    let _ticket = shell
        .begin(
            input(
                &shell,
                "root",
                "root-workspace.set-up.v1",
                json!({"instance":{"kind":"Resource","name":"shared"}}),
            ),
            &runner,
        )
        .unwrap();
    until(|| {
        fs::read_to_string(parent.path().join("shared/pid")).is_ok_and(|s| s.parse::<i32>().is_ok())
    });
    let pid = fs::read_to_string(parent.path().join("shared/pid"))
        .unwrap()
        .parse::<i32>()
        .unwrap();
    drop(runner);
    until(|| unsafe { libc::kill(pid, 0) } != 0);
    assert!(shell.state().unwrap().started);
}
