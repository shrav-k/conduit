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
    until_for(Duration::from_secs(30), ready);
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
    // These are synthetic test processes. No token file is read or copied.
    fs::write(
        root.join("environment.json"),
        serde_json::to_vec(&std::env::vars().collect::<BTreeMap<_, _>>()).unwrap(),
    )
    .unwrap();
    let cwd = std::env::current_dir().unwrap();
    fs::write(root.join("cwd.json"), serde_json::to_vec(&json!({"path":cwd,"mode":fs::metadata(&cwd).unwrap().permissions().mode()&0o777,"empty":fs::read_dir(&cwd).unwrap().count()==0})).unwrap()).unwrap();
    let _descendant = if mode == "descendant" {
        let child = Command::new("/bin/sleep")
            .arg("600")
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
                if mode == "error-write" || mode.starts_with("repair") && calls == 1 {
                    println!(
                        "{}",
                        json!({"jsonrpc":"2.0","id":value["id"],"error":{"code":-32020,"message":"refresh","data":{"marker":"original-data"}}})
                    );
                    std::io::stdout().flush().unwrap();
                    continue;
                }
                json!({"content":[{"type":"text","text":"test result"}],"isError":false})
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
            startup.elapsed() < Duration::from_secs(180),
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
    // Allow startup under parallel suite load; post-kill waits still use 30 seconds.
    until_for(Duration::from_secs(120), || {
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
