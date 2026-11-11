use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};

fn command(store: &Path, verb: &str, extra: &[&str]) -> Output {
    let output = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .arg(verb)
        .arg("--store")
        .arg(store)
        .args(extra)
        .output()
        .unwrap();
    if verb == "inspect" {
        let rows: Vec<Value> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert!(!rows.is_empty());
        assert_eq!(
            output.status.success(),
            rows.iter()
                .all(|r| conduit_shell::author::succeeded(&r["result"]))
        );
        return output;
    }
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn init_inspect_and_export() {
    let parent = tempfile::tempdir().unwrap();
    let store = parent.path().join("store");
    let first = command(&store, "init", &[]);
    assert_eq!(
        serde_json::from_slice::<Value>(&first.stdout).unwrap()["started"],
        true
    );
    let state: Value =
        serde_json::from_slice(&std::fs::read(store.join("state.json")).unwrap()).unwrap();
    let ids: Vec<_> = state["state"]["objects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["id"].clone())
        .collect();
    let repeated = command(&store, "init", &[]);
    assert_eq!(
        serde_json::from_slice::<Value>(&repeated.stdout).unwrap()["started"],
        false
    );
    let inspection = command(&store, "inspect", &[]);
    let inspection: String = String::from_utf8(inspection.stdout).unwrap();
    let lines: Vec<Value> = inspection
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert!(
        lines
            .iter()
            .any(|v| v["object"]["name"] == "root" && v["result"]["isError"] == false)
    );
    let records = command(&store, "records", &[]);
    let exported: Vec<Value> = String::from_utf8(records.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let after: Value =
        serde_json::from_slice(&std::fs::read(store.join("state.json")).unwrap()).unwrap();
    assert_eq!(
        exported,
        after["state"]["records"].as_array().unwrap().clone()
    );
    assert_eq!(
        ids,
        after["state"]["objects"].as_array().unwrap()[..8]
            .iter()
            .map(|v| v["id"].clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn real_npm_register_completes_setup_and_ends_runs_on_exit() {
    if std::env::var_os("CONDUIT_TEST_NPM").is_none() {
        return;
    }
    let parent = tempfile::tempdir().unwrap();
    let store = parent.path().join("store");
    command(&store, "init", &[]);
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../conformance/fixtures/s1/project");
    let project = parent.path().join("project");
    std::fs::create_dir(&project).unwrap();
    for name in ["shared", "history"] {
        std::fs::create_dir(parent.path().join(name)).unwrap();
    }
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let text = std::fs::read_to_string(entry.path())
            .unwrap()
            .replace(
                "/tmp/conduit-s1/shared",
                parent.path().join("shared").to_str().unwrap(),
            )
            .replace(
                "/tmp/conduit-s1/history",
                parent.path().join("history").to_str().unwrap(),
            );
        std::fs::write(project.join(entry.file_name()), text).unwrap();
    }
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let archives: Vec<_> = ["filesystem", "history", "agents"]
        .iter()
        .map(|name| {
            let path = repo.join(format!("conformance/fixtures/s1/archives/{name}.tar.gz"));
            format!(
                "{}#{}",
                path.display(),
                conduit_core::packages::bytes_digest(&std::fs::read(&path).unwrap())
            )
        })
        .collect();
    let mut extra = vec![project.to_str().unwrap()];
    for archive in &archives {
        extra.extend(["--package", archive]);
    }
    let registration = command(&store, "register", &extra);
    assert!(
        registration.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&registration.stderr)
    );
    let inspected = command(&store, "inspect", &[]);
    let rows: Vec<Value> = String::from_utf8(inspected.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    for name in ["shared", "history"] {
        let row = rows.iter().find(|r| r["object"]["name"] == name).unwrap();
        let current = &row["result"]["structuredContent"]["status"]["conditions"];
        let running = current
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["type"] == "Running")
            .unwrap();
        assert_eq!(running["reason"], "RunStopped");
        assert_eq!(running["status"], "False");
        let past = row["pastConditions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["condition"]["reason"] == "RunRunning")
            .unwrap();
        assert_eq!(past["endedAt"], running["lastTransitionTime"]);
        assert!(past["endedAt"].is_string());
    }
    let state: Value =
        serde_json::from_slice(&std::fs::read(store.join("state.json")).unwrap()).unwrap();
    for name in ["workspace", "shared", "history", "agent-a", "agent-b"] {
        assert!(
            state["state"]["objects"]
                .as_array()
                .unwrap()
                .iter()
                .any(|o| o["name"] == name && o["state"] == "ready")
        );
    }
    assert!(
        state["state"]["invocations"]
            .as_array()
            .unwrap()
            .iter()
            .all(|v| v["state"] != "in-progress")
    );
    assert!(
        !String::from_utf8(registration.stdout)
            .unwrap()
            .contains("\"pending\"")
    );
}

#[test]
fn serve_management_commands_and_fixed_audience() {
    use std::time::{Duration, Instant};
    let parent = tempfile::tempdir().unwrap();
    let store = parent.path().join("store");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let url = format!("http://127.0.0.1:{port}/mcp");
    command(&store, "init", &["--url", &url]);
    let different = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args([
            "serve",
            "--store",
            store.to_str().unwrap(),
            "--url",
            "http://127.0.0.1:1/mcp",
        ])
        .output()
        .unwrap();
    assert!(!different.status.success());
    assert!(String::from_utf8_lossy(&different.stderr).contains("fixed token audience"));
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
            .arg(&store)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap(),
    );
    let start = Instant::now();
    loop {
        if std::net::TcpStream::connect(("127.0.0.1", port)).is_ok() {
            break;
        }
        assert!(server.0.try_wait().unwrap().is_none(), "server exited");
        assert!(start.elapsed() < Duration::from_secs(30));
        std::thread::sleep(Duration::from_millis(20));
    }
    let tools = command(&store, "tools", &[]);
    let tools: Value = serde_json::from_slice(&tools.stdout).unwrap();
    assert_eq!(tools["cacheScope"], "private");
    assert!(
        tools["tools"]
            .as_array()
            .unwrap()
            .iter()
            .any(|t| t["name"] == "access.close.v1")
    );
    let result = command(
        &store,
        "call",
        &[
            "management.inspect.v1",
            r#"{"object":{"kind":"Interface","name":"root"}}"#,
        ],
    );
    let result: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(result["isError"], false);
    // Several separate author processes initialize and send requests concurrently.
    // Serial repeats also cover the intermittent sequential failure in the run.
    for _ in 0..2 {
        std::thread::scope(|scope| {
            let threads: Vec<_> = (0..8)
                .map(|i| {
                    let store = &store;
                    scope.spawn(move || {
                        if i % 2 == 0 {
                            command(store, "tools", &[]);
                        } else {
                            command(
                                store,
                                "call",
                                &[
                                    "management.inspect.v1",
                                    r#"{"object":{"kind":"Interface","name":"root"}}"#,
                                ],
                            );
                        }
                    })
                })
                .collect();
            for thread in threads {
                thread.join().unwrap();
            }
        });
    }
    let failed = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args(["call", "--store"])
        .arg(&store)
        .args([
            "management.inspect.v1",
            r#"{"object":{"kind":"Resource","name":"absent"}}"#,
        ])
        .output()
        .unwrap();
    assert!(!failed.status.success());
    assert!(serde_json::from_slice::<Value>(&failed.stdout).is_ok());
    let tools_by_kind = command(&store, "tools", &["--as", "Interface/root"]);
    assert_eq!(
        serde_json::from_slice::<Value>(&tools_by_kind.stdout).unwrap()["tools"],
        tools["tools"]
    );
    let absent = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args(["tools", "--store"])
        .arg(&store)
        .args(["--as", "Agent/absent"])
        .output()
        .unwrap();
    assert!(!absent.status.success());
    assert!(String::from_utf8_lossy(&absent.stderr).contains("does not exist"));

    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let context = conduit_shell::boundary::context(&repo).unwrap();
    let api = conduit_loader::Specification::read(&repo.join("spec"))
        .unwrap()
        .names()
        .api_version
        .clone();
    let core = context.core_package;
    let project = parent.path().join("partial");
    std::fs::create_dir(&project).unwrap();
    std::fs::write(
        project.join("conduit.json"),
        serde_json::json!({"apiVersion":api,"address":"example.org/test/partial"}).to_string(),
    )
    .unwrap();
    let docs = serde_json::json!([
        {"apiVersion":api,"kind":"Type","metadata":{"name":"empty.v1"},"spec":{"family":"Resource","schema":{"type":"object"},"offers":[],"requires":[]}},
        {"apiVersion":api,"kind":"Pool","metadata":{"name":"team-template"},"spec":{"type":format!("{core}#pool.v1")}},
        {"apiVersion":api,"kind":"Pool","metadata":{"name":"team"},"from":"team-template"},
        {"apiVersion":api,"kind":"Conduit","metadata":{"name":"team-inspection"},"spec":{"capability":format!("{core}#pool-management.v1"),"operations":[format!("{core}#inspect.v1")]}},
        {"apiVersion":api,"kind":"Conduit","metadata":{"name":"root-team-inspection"},"from":"team-inspection","consumer":{"kind":"Interface","name":"root"},"target":{"kind":"Pool","name":"team"}},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"empty-template"},"spec":{"type":"#empty.v1"}},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"empty"},"from":"empty-template","owner":"team"}
    ]);
    std::fs::write(project.join("documents.json"), docs.to_string()).unwrap();
    let partial = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args(["submit", "--store"])
        .arg(&store)
        .arg(&project)
        .output()
        .unwrap();
    assert!(!partial.status.success());
    let rows: Vec<Value> = String::from_utf8(partial.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let steps: Vec<_> = rows.iter().filter(|r| r.get("step").is_some()).collect();
    assert_eq!(
        steps.len(),
        4,
        "{rows:?}: {}",
        String::from_utf8_lossy(&partial.stderr)
    );
    assert_eq!(steps[0]["step"], "registration");
    assert_eq!(steps[0]["outcome"], "succeeded");
    assert_eq!(steps[1]["step"], "open Conduit/root-team-inspection");
    assert_eq!(steps[1]["outcome"], "succeeded");
    assert_eq!(steps[2]["step"], "setup Pool/team");
    assert_eq!(steps[2]["outcome"], "succeeded");
    assert_eq!(steps[3]["step"], "setup Resource/empty");
    assert_eq!(steps[3]["outcome"], "failed");
    let first_step = rows.iter().position(|r| r.get("step").is_some()).unwrap();
    assert!(rows[first_step..].iter().all(|r| r.get("step").is_some()));
    let unready = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args(["tools", "--store"])
        .arg(&store)
        .args(["--as", "Resource/empty"])
        .output()
        .unwrap();
    assert!(!unready.status.success());
    assert!(String::from_utf8_lossy(&unready.stderr).contains("instance exists"));
    let permission = std::fs::metadata(store.join("client-tokens")).unwrap();
    use std::os::unix::fs::PermissionsExt;
    assert_eq!(permission.permissions().mode() & 0o777, 0o700);
    for file in std::fs::read_dir(store.join("client-tokens")).unwrap() {
        assert_eq!(
            file.unwrap().metadata().unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
    let pid = server.0.id().to_string();
    let signal = Command::new("kill").args(["-TERM", &pid]).status().unwrap();
    assert!(signal.success());
    let start = Instant::now();
    loop {
        if let Some(status) = server.0.try_wait().unwrap() {
            assert!(status.success());
            break;
        }
        assert!(start.elapsed() < Duration::from_secs(10));
        std::thread::sleep(Duration::from_millis(20));
    }
    let records = command(&store, "records", &[]);
    assert!(
        String::from_utf8(records.stdout)
            .unwrap()
            .contains("request-ended")
    );
}

#[test]
fn help_describes_every_command_without_store_access() {
    let verbs = [
        "init",
        "register",
        "submit",
        "replace",
        "install",
        "retire",
        "unregister",
        "inspect",
        "records",
        "serve",
        "tools",
        "call",
    ];
    let all = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(all.status.success());
    for verb in verbs {
        assert!(String::from_utf8_lossy(&all.stdout).contains(&format!("conduit {verb}")));
        let help = Command::new(env!("CARGO_BIN_EXE_conduit"))
            .args([verb, "--help"])
            .output()
            .unwrap();
        assert!(help.status.success());
        assert!(String::from_utf8_lossy(&help.stdout).contains("Example:"));
    }
}

#[test]
fn registration_does_not_install_s1_packages() {
    let parent = tempfile::tempdir().unwrap();
    let store = parent.path().join("store");
    command(&store, "init", &[]);
    let project =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../conformance/fixtures/s1/project");
    let output = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args(["register", "--store"])
        .arg(&store)
        .arg(project)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let state: Value =
        serde_json::from_slice(&std::fs::read(store.join("state.json")).unwrap()).unwrap();
    for key in ["installed_packages", "preinstalled_snapshots"] {
        assert!(
            state["state"]
                .get(key)
                .is_none_or(|v| v.as_array().unwrap().is_empty())
        );
    }
}

#[test]
fn failed_startup_setup_can_be_repaired_and_inspected_while_serving() {
    use serde_json::json;
    use std::{
        fs,
        time::{Duration, Instant},
    };
    let parent = tempfile::tempdir().unwrap();
    let store = parent.path().join("store");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    command(
        &store,
        "init",
        &["--url", &format!("http://127.0.0.1:{port}/mcp")],
    );
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let api = conduit_loader::Specification::read(&repo.join("spec"))
        .unwrap()
        .names()
        .api_version
        .clone();
    let core = conduit_shell::boundary::context(&repo)
        .unwrap()
        .core_package;
    let project = parent.path().join("project");
    fs::create_dir(&project).unwrap();
    fs::write(
        project.join("conduit.json"),
        json!({"apiVersion":api,"address":"example.org/test/repair"}).to_string(),
    )
    .unwrap();
    let docs = [
        json!({"apiVersion":api,"kind":"Type","metadata":{"name":"empty.v1"},"spec":{"family":"Resource","schema":{"type":"object"},"offers":[],"requires":[]}}),
        json!({"apiVersion":api,"kind":"Pool","metadata":{"name":"test-pool"},"spec":{"type":format!("{core}#pool.v1")}}),
        json!({"apiVersion":api,"kind":"Pool","metadata":{"name":"workspace"},"from":"test-pool"}),
        json!({"apiVersion":api,"kind":"Resource","metadata":{"name":"empty-resource"},"spec":{"type":"#empty.v1","config":{}}}),
        json!({"apiVersion":api,"kind":"Resource","metadata":{"name":"repair-me"},"from":"empty-resource","owner":"workspace"}),
    ];
    for (i, doc) in docs.iter().enumerate() {
        fs::write(project.join(format!("{i}.json")), doc.to_string()).unwrap();
    }
    let registration = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .arg("register")
        .arg("--store")
        .arg(&store)
        .arg(&project)
        .output()
        .unwrap();
    assert!(!registration.status.success());
    assert!(String::from_utf8_lossy(&registration.stderr).contains("setup"));
    struct Server(std::process::Child);
    impl Drop for Server {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let stderr = parent.path().join("serve.stderr");
    let mut server = Server(
        Command::new(env!("CARGO_BIN_EXE_conduit"))
            .arg("serve")
            .arg("--store")
            .arg(&store)
            .stdout(std::process::Stdio::null())
            .stderr(fs::File::create(&stderr).unwrap())
            .spawn()
            .unwrap(),
    );
    let start = Instant::now();
    while std::net::TcpStream::connect(("127.0.0.1", port)).is_err() {
        assert!(
            server.0.try_wait().unwrap().is_none(),
            "{}",
            fs::read_to_string(&stderr).unwrap()
        );
        assert!(start.elapsed() < Duration::from_secs(30));
        std::thread::sleep(Duration::from_millis(10));
    }
    let before: Value =
        serde_json::from_slice(&fs::read(store.join("state.json")).unwrap()).unwrap();
    assert!(
        before["state"]["records"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["type"] == "request-ended" && r["data"]["reason"] == "refused")
    );
    let read_lines = |output: Output| -> Vec<Value> {
        String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect()
    };
    let unavailable = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args(["inspect", "--store"])
        .arg(&store)
        .output()
        .unwrap();
    assert!(!unavailable.status.success());
    let unavailable = read_lines(unavailable);
    let refused = unavailable
        .iter()
        .find(|r| r["object"]["name"] == "repair-me")
        .unwrap();
    assert_eq!(refused["result"]["isError"], true);
    assert!(refused.get("consumedConduits").is_none());
    let repair = parent.path().join("repair");
    fs::create_dir(&repair).unwrap();
    fs::write(repair.join("admin.yaml"), format!("apiVersion: {api}\nkind: Conduit\nmetadata:\n  name: pool-admin\nspec:\n  capability: {core}#pool-management.v1\n  operations: [{core}#inspect.v1, {core}#set-up.v1]\n---\napiVersion: {api}\nkind: Conduit\nmetadata:\n  name: root-workspace\nfrom: pool-admin\nconsumer: {{kind: Interface, name: root}}\ntarget: {{kind: Pool, name: workspace}}\n")).unwrap();
    command(&store, "submit", &[repair.to_str().unwrap()]);
    let setup = command(
        &store,
        "call",
        &[
            "root-workspace.set-up.v1",
            &json!({"instance":{"kind":"Resource","name":"repair-me"}}).to_string(),
        ],
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&setup.stdout).unwrap()["isError"],
        false
    );
    // Registration through call leaves this standalone Conduit registered, not open.
    let closed = json!({"documents":[{"apiVersion":api,"kind":"Conduit","metadata":{"name":"closed-route"},"from":"pool-admin","consumer":{"kind":"Interface","name":"root"},"target":{"kind":"Pool","name":"workspace"}}]});
    let registered = command(
        &store,
        "call",
        &["management.register.v1", &closed.to_string()],
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&registered.stdout).unwrap()["isError"],
        false
    );
    let assert_inspection = |rows: &[Value]| {
        let resource = rows
            .iter()
            .find(|r| r["object"]["name"] == "repair-me")
            .unwrap();
        assert_eq!(
            resource["result"]["structuredContent"]["status"]["state"],
            "ready"
        );
        let root = rows.iter().find(|r| r["object"]["name"] == "root").unwrap();
        let paths = root["consumedConduits"].as_array().unwrap();
        assert_eq!(paths.len(), 5);
        for target in ["management", "access"] {
            assert!(paths.iter().any(|p| {
                p["target"]["name"] == target
                    && p["open"] == true
                    && p["name"]
                        .as_str()
                        .is_some_and(|n| n.starts_with(&format!("root-{target}-")))
            }));
        }
        for (name, open) in [("root-workspace", true), ("closed-route", false)] {
            assert!(
                paths.iter().any(|p| p["name"] == name
                    && p["open"] == open
                    && p["target"]["id"].is_string()),
                "{paths:?}"
            );
        }
    };
    assert_inspection(&read_lines(command(&store, "inspect", &[])));
    let after: Value =
        serde_json::from_slice(&fs::read(store.join("state.json")).unwrap()).unwrap();
    let root_id = &before["state"]["objects"]
        .as_array()
        .unwrap()
        .iter()
        .find(|o| o["name"] == "root")
        .unwrap()["id"];
    let inspections: Vec<_> = after["state"]["records"]
        .as_array()
        .unwrap()
        .iter()
        .skip(before["state"]["records"].as_array().unwrap().len())
        .filter(|r| {
            r["type"] == "invocation-dispatched"
                && r["data"]["operation"] == format!("{core}#inspect.v1")
        })
        .collect();
    assert!(!inspections.is_empty());
    assert!(
        inspections
            .iter()
            .all(|r| r["data"]["caller"] == *root_id && r["data"]["consumer"] == *root_id)
    );
    unsafe {
        libc::kill(server.0.id() as i32, libc::SIGTERM);
    }
    assert!(!server.0.wait().unwrap().success());
    assert_inspection(&read_lines(command(&store, "inspect", &[])));
}

#[test]
fn init_accepts_and_preserves_the_trust_domain() {
    let temp = tempfile::tempdir().unwrap();
    let store = temp.path().join("engine");
    command(&store, "init", &["--trust-domain", "cli.example"]);
    assert_eq!(
        std::fs::read_to_string(store.join("engine-id")).unwrap(),
        "spiffe://cli.example/conduit/engine"
    );
    let state: Value =
        serde_json::from_slice(&std::fs::read(store.join("state.json")).unwrap()).unwrap();
    assert_eq!(
        state["state"]["objects"][0]["status"]["trustDomain"],
        "cli.example"
    );
    command(&store, "init", &[]);
    let rejected = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args(["init", "--store"])
        .arg(&store)
        .args(["--trust-domain", "changed.example"])
        .output()
        .unwrap();
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("FST-007"));
    let invalid = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .args(["init", "--store"])
        .arg(temp.path().join("invalid"))
        .args(["--trust-domain", "Invalid"])
        .output()
        .unwrap();
    assert!(!invalid.status.success());
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("FST-007"));
}
