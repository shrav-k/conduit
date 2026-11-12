mod runtime_support;
use conduit_shell::{Config, Executor, MissingExecutor, Shell, boundary, runner::Runner};
use runtime_support::*;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    io::{BufRead, Write},
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};
fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn id(shell: &Shell, name: &str) -> String {
    shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| o["name"] == name && o.get("spec").is_none())
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .into()
}
fn invoke(shell: &Shell, executor: &dyn Executor, tool: &str, arguments: Value) -> Value {
    let ticket = shell.begin(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":id(shell,"root")},"tool":tool,"arguments":arguments}}), executor).unwrap();
    deadline(|| ticket.try_reply().unwrap().is_some());
    ticket.try_reply().unwrap().unwrap()
}
fn outcome(shell: &Shell, result: &Value) -> String {
    result["_meta"][format!("{}outcome", shell.context.meta_prefix)]
        .as_str()
        .unwrap_or("")
        .into()
}
#[track_caller]
fn deadline(mut ready: impl FnMut() -> bool) {
    let end = Instant::now() + Duration::from_secs(45);
    while !ready() {
        assert!(Instant::now() < end, "progress deadline");
        std::thread::sleep(Duration::from_millis(10));
    }
}
fn log(path: &Path) -> Vec<Value> {
    fs::read_to_string(path.join("messages"))
        .unwrap_or_default()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
struct Fixture {
    temp: tempfile::TempDir,
    shell: Arc<Shell>,
    runner: Runner,
}
impl Fixture {
    fn new(mode: &str) -> Self {
        Self::with_initialize_timeout(mode, Duration::from_secs(5))
    }
    fn with_initialize_timeout(mode: &str, initialize_timeout: Duration) -> Self {
        assert!(std::env::var_os("CARGO_MANIFEST_DIR").is_some());
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("mode"), mode).unwrap();
        let shell = Arc::new(
            Shell::open(
                &temp.path().join("store"),
                boundary::context(&repository()).unwrap(),
                Config {
                    engine_https_url: if mode == "no-https" {
                        None
                    } else {
                        Some("https://engine.test/mcp".into())
                    },
                    pass_environment: vec!["CARGO_MANIFEST_DIR".into()],
                    call_timeout: Duration::from_secs(5),
                    initialize_timeout,
                    ..Config::default()
                },
            )
            .unwrap(),
        );
        shell.submit(json!({"kind":"first-start","machine":conduit_shell::local_machine(BTreeMap::new()).unwrap()}), &MissingExecutor).unwrap();
        let base = repository().join("conformance/conventions/setup/review-relative-provider");
        let mut documents: Value =
            serde_json::from_slice(&fs::read(base.join("documents.json")).unwrap()).unwrap();
        for doc in documents.as_array_mut().unwrap() {
            if doc["metadata"]["name"] == "provider.v1" {
                doc["spec"]["schema"] = json!({"type":"object","properties":{"root":{"type":"string"}},"required":["root"]});
            }
            if doc["metadata"]["name"] == "test.v1" {
                doc["spec"]["schema"] = json!({"type":"object","properties":{"binding":{"type":"string"}},"required":["binding"]});
            }
            if doc["metadata"]["name"] == "provider-template" {
                doc["spec"]["config"] = json!({"root":temp.path().to_str().unwrap()});
            }
            if doc["metadata"]["name"] == "agent" {
                doc["spec"]["config"] = json!({"binding":"own-only"});
            }
        }
        let api = documents[0]["apiVersion"].clone();
        documents.as_array_mut().unwrap().extend([
            json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"test-call"},"spec":{"capability":"#test-cap.v1","operations":["#ping.v1"]}}),
            json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"runtime-test"},"from":"test-call","consumer":{"kind":"Interface","name":"root"},"target":{"kind":"Agent","name":"a"}}),
        ]);
        let mut manifest = boundary::value(&base.join("manifest.json")).unwrap();
        manifest["implementations"]["provider-server"]["command"] = json!([
            std::env::current_exe().unwrap().to_str().unwrap(),
            "--exact",
            "scripted_runtime_provider",
            "--ignored",
            "--nocapture"
        ]);
        manifest["implementations"]["provider-server"]["environment"] =
            json!({"TEST_PROVIDER_DIR":"/root"});
        manifest["implementations"]["relative"]["command"] = json!(["/unused-test-program"]);
        manifest["implementations"]["relative"]["environment"] = json!({"VISIBLE":"/binding"});
        let state = shell.state().unwrap();
        let root = id(&shell, "root");
        let route = state
            .objects
            .iter()
            .find(|o| o["local_name"] == "management" && o["consumer"] == root)
            .unwrap();
        let reply = shell.submit(json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,
            "project":{"manifest":manifest,"documents":documents,"directory":null},
            "request":{"sender":root,"caller":root,"conduit":route["id"],"target":route["target"],"operation":format!("{}#register.v1",shell.context.core_package)}}), &MissingExecutor).unwrap();
        assert_eq!(reply["accepted"], true, "{reply}");
        let runner = Runner::new(&shell, PathBuf::from(env!("CARGO_BIN_EXE_conduit"))).unwrap();
        let reply = invoke(
            &shell,
            &runner,
            "system.set-up.v1",
            json!({"instance":{"kind":"Resource","name":"provider"}}),
        );
        assert_eq!(
            outcome(&shell, &reply),
            "succeeded",
            "{reply}; {:?}",
            runner.diagnostics()
        );
        let reply = invoke(
            &shell,
            &runner,
            "management.set-up.v1",
            json!({"instance":{"kind":"Fleet","name":"s1"}}),
        );
        assert_eq!(outcome(&shell, &reply), "succeeded", "{reply}");
        let reply = invoke(
            &shell,
            &runner,
            "access.open.v1",
            json!({"conduit":"runtime-test"}),
        );
        assert_eq!(outcome(&shell, &reply), "succeeded", "{reply}");
        Self {
            temp,
            shell,
            runner,
        }
    }
    fn setup(&self) -> Value {
        invoke(
            &self.shell,
            &self.runner,
            "s1.set-up.v1",
            json!({"instance":{"kind":"Agent","name":"a"}}),
        )
    }
    fn call(&self) -> Value {
        invoke(&self.shell, &self.runner, "runtime-test.ping.v1", json!({}))
    }
}
#[test]
#[ignore = "stdio runtime provider fixture; spawned by provider tests"]
fn scripted_runtime_provider() {
    conduit_shell::install_crypto_provider();
    let directory = PathBuf::from(std::env::var_os("TEST_PROVIDER_DIR").unwrap());
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let prefix = "io.github.shrav-k.conduit/";
    let mut gate: Option<Gate> = None;
    let mut last_auth: Option<(String, String, Value)> = None;
    for line in std::io::stdin().lock().lines() {
        let Ok(value) = serde_json::from_str::<Value>(&line.unwrap()) else {
            continue;
        };
        let mode = fs::read_to_string(directory.join("mode")).unwrap();
        if mode == "ended-session"
            && let Some(gate) = &gate
        {
            gate.ended.store(true, std::sync::atomic::Ordering::SeqCst);
        }

        let method = value["method"].as_str().unwrap_or("");
        let mut recorded = value.clone();
        if method == "tools/call" && value["params"]["name"] == "run.v1" {
            let identity = &value["params"]["arguments"]["identity"];
            let files = identity["files"].as_object().unwrap();
            let expected = [
                "ca.crt",
                "jwks.json",
                "engine-id",
                "token",
                "tls.crt",
                "tls.key",
            ];
            assert!(files.len() == 6 && expected.iter().all(|n| files.contains_key(*n)));
            assert!(identity["environment"]["CONDUIT_URL"] == "https://engine.test/mcp");
            assert!(identity["environment"].as_object().unwrap().len() == 2);
            let token = files["token"].as_str().unwrap();
            use base64::Engine;
            let claims: Value = serde_json::from_slice(
                &base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .decode(token.split('.').nth(1).unwrap())
                    .unwrap(),
            )
            .unwrap();
            let instance = claims["sub"].as_str().unwrap().to_owned();
            let (cert, key) = if mode == "self-signed" {
                TestAuthority::self_signed(&instance)
            } else {
                (
                    files["tls.crt"].as_str().unwrap().into(),
                    files["tls.key"].as_str().unwrap().into(),
                )
            };
            last_auth = Some((
                files["engine-id"].as_str().unwrap().into(),
                instance,
                serde_json::from_str(files["jwks.json"].as_str().unwrap()).unwrap(),
            ));
            let options = GateOptions {
                cert,
                key,
                client_root: None,
                auth: last_auth.clone(),
                redirect: None,
                sse: true,
                count_file: Some(directory.join("gate-count")),
                get_events: false,
                delay_first: false,
                block_initialize: mode == "blocked-initialize",
            };
            gate = Some(runtime.block_on(Gate::start(options)));
            recorded["params"]["arguments"]["identity"]["environment"] = json!(
                identity["environment"]
                    .as_object()
                    .unwrap()
                    .keys()
                    .map(|name| (name.clone(), "redacted"))
                    .collect::<BTreeMap<_, _>>()
            );
            recorded["params"]["arguments"]["identity"]["files"] = json!(
                expected
                    .into_iter()
                    .map(|n| (n, "redacted"))
                    .collect::<BTreeMap<_, _>>()
            );
        }
        let mut log = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(directory.join("messages"))
            .unwrap();
        writeln!(log, "{recorded}").unwrap();
        let result = match method {
            "initialize" => {
                json!({"protocolVersion":value["params"]["protocolVersion"],"serverInfo":{"name":"provider-test","version":"1"},"capabilities":{"tools":{}}})
            }
            "server/discover" => {
                json!({"supportedVersions":["2026-07-28"],"serverInfo":{"name":"provider-test","version":"1"},"capabilities":{"extensions":if mode == "no-extension" { json!({}) } else { json!({format!("{prefix}implementation-interface"):{"revision":"v0"}}) }}})
            }
            "tools/call" => {
                let name = value["params"]["name"].as_str().unwrap();
                let mut outputs = json!({});
                let mut stated = "succeeded";
                let mut error = false;
                match name {
                    "inspect-run.v1" => {
                        if mode == "bad-inspect" {
                            let (cert, key) =
                                TestAuthority::self_signed(&last_auth.as_ref().unwrap().1);
                            gate = Some(runtime.block_on(Gate::start(GateOptions {
                                cert,
                                key,
                                client_root: None,
                                auth: last_auth.clone(),
                                redirect: None,
                                sse: true,
                                count_file: Some(directory.join("bad-gate-count")),
                                get_events: false,
                                delay_first: false,
                                block_initialize: false,
                            })));
                        }
                        if let Some(gate) = &gate {
                            outputs = json!({"state":"running","url":gate.url});
                        } else {
                            outputs = json!({"state":"stopped"});
                        }
                    }
                    "run.v1" => {
                        outputs = json!({"url":gate.as_ref().unwrap().url});
                        if mode == "failed" || mode == "unknown" {
                            stated = if mode == "failed" {
                                "failed"
                            } else {
                                "unknown"
                            };
                            error = true;
                        }
                        if mode == "contradiction" {
                            error = true;
                        }
                        if mode == "no-url" {
                            outputs = json!({});
                        }
                    }
                    "stop.v1" => {
                        gate.take();
                        if mode == "stop-failed" {
                            stated = "failed";
                            error = true;
                        }
                    }
                    _ => {}
                }
                json!({"content":[],"structuredContent":outputs,"isError":error,"_meta":{format!("{prefix}outcome"):stated}})
            }
            _ => continue,
        };
        if value.get("id").is_some() {
            println!(
                "{}",
                json!({"jsonrpc":"2.0","id":value["id"],"result":result})
            );
            std::io::stdout().flush().unwrap();
        }
        if let Some(gate) = &gate {
            fs::write(
                directory.join("gate-count"),
                gate.received.lock().unwrap().len().to_string(),
            )
            .unwrap();
        }
    }
}
#[test]
fn runtime_provider_setup_discovers_materializes_bridges_and_drop_ends_bridge() {
    let fixture = Fixture::new("success");
    let reply = fixture.setup();
    assert_eq!(
        outcome(&fixture.shell, &reply),
        "succeeded",
        "{reply}; {:?}",
        fixture.runner.diagnostics()
    );
    let messages = log(fixture.temp.path());
    let discovered = messages
        .iter()
        .position(|m| m["method"] == "server/discover")
        .unwrap();
    let calls = messages
        .iter()
        .enumerate()
        .filter(|(_, m)| m["method"] == "tools/call")
        .collect::<Vec<_>>();
    assert!(discovered < calls[0].0);
    assert_eq!(calls[0].1["params"]["name"], "inspect-run.v1");
    assert_eq!(calls[1].1["params"]["name"], "run.v1");
    let prefix = &fixture.shell.context.meta_prefix;
    for (_, call) in &calls {
        let keys = call["params"]["_meta"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        let mut expected = [
            "instance",
            "operation",
            "config",
            "revision",
            "invocation",
            "caller",
        ]
        .map(|n| format!("{prefix}{n}"))
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
        expected.insert("io.modelcontextprotocol/clientCapabilities".into());
        assert_eq!(keys, expected);
        assert_eq!(
            call["params"]["_meta"]["io.modelcontextprotocol/clientCapabilities"]["extensions"]
                [format!("{prefix}implementation-interface")]["revision"],
            "v0"
        );
    }
    assert_eq!(
        calls[1].1["params"]["arguments"]["environment"],
        json!({"VISIBLE":"own-only"})
    );
    assert!(fixture.runner.running_count() >= 2);
    assert_private(&fixture);
    let encoded = serde_json::to_string(&fixture.shell.state().unwrap()).unwrap();
    assert!(
        !encoded.contains("BEGIN CERTIFICATE")
            && !encoded.contains("PRIVATE KEY")
            && !encoded.contains("eyJhbGci")
    );
    let reply = fixture.call();
    assert_eq!(
        outcome(&fixture.shell, &reply),
        "succeeded",
        "{reply}; {:?}",
        fixture.runner.diagnostics()
    );
    let groups = groups(&fixture);
    drop(fixture.runner);
    deadline(|| groups.iter().all(|pid| unsafe { libc::kill(*pid, 0) } != 0));
}
#[test]
fn runtime_provider_failures_discovery_and_tls_refusal_map_reports() {
    for (mode, expected) in [
        ("failed", "failed"),
        ("unknown", "unknown"),
        ("contradiction", "unknown"),
        ("no-url", "unknown"),
        ("self-signed", "unknown"),
        ("no-extension", "failed"),
        ("no-https", "failed"),
    ] {
        let fixture = Fixture::new(mode);
        let reply = fixture.setup();
        assert_eq!(
            outcome(&fixture.shell, &reply),
            expected,
            "{mode}: {reply}; {:?}",
            fixture.runner.diagnostics()
        );
        let messages = log(fixture.temp.path());
        if mode == "no-extension" {
            assert!(!messages.iter().any(|m| m["method"] == "tools/call"));
        }
        if mode == "no-https" {
            assert!(!messages.iter().any(|m| m["params"]["name"] == "run.v1"));
        }
        if mode == "self-signed" {
            assert!(
                fixture
                    .runner
                    .diagnostics()
                    .iter()
                    .any(|s| s.contains("certificate did not prove the instance"))
            );
            assert_eq!(
                fs::read_to_string(fixture.temp.path().join("gate-count")).unwrap(),
                "0"
            );
        }
    }
}

fn groups(fixture: &Fixture) -> Vec<i32> {
    fs::read_dir(fixture.temp.path().join("store/run-groups"))
        .unwrap()
        .map(|e| {
            let value: Value =
                serde_json::from_slice(&fs::read(e.unwrap().path()).unwrap()).unwrap();
            value["group"].as_i64().unwrap() as i32
        })
        .collect()
}
#[test]
fn runtime_provider_retirement_stops_bridge_and_keeps_provider_stop_outcome() {
    for mode in ["success", "stop-failed"] {
        let fixture = Fixture::new("success");
        assert_eq!(outcome(&fixture.shell, &fixture.setup()), "succeeded");
        let subject = id(&fixture.shell, "a");
        let groups = groups(&fixture);
        assert_eq!(groups.len(), 2);
        fs::write(fixture.temp.path().join("mode"), mode).unwrap();
        let reply = invoke(
            &fixture.shell,
            &fixture.runner,
            "s1.retire.v1",
            json!({"instance":{"kind":"Agent","name":"a"}}),
        );
        assert_eq!(
            outcome(&fixture.shell, &reply),
            if mode == "success" {
                "succeeded"
            } else {
                "failed"
            },
            "{reply}; {:?}",
            fixture.runner.diagnostics()
        );
        let messages = log(fixture.temp.path());
        let stop = messages
            .iter()
            .find(|m| m["params"]["name"] == "stop.v1")
            .unwrap();
        let run = messages
            .iter()
            .find(|m| m["params"]["name"] == "run.v1")
            .unwrap();
        assert_eq!(
            stop["params"]["arguments"]["machine"],
            run["params"]["arguments"]["machine"]
        );
        assert_eq!(stop["params"]["arguments"]["instance"], subject);
        deadline(|| fixture.runner.running_count() == 1);
        deadline(|| groups.iter().any(|pid| unsafe { libc::kill(*pid, 0) } != 0));
    }
}
#[test]
fn runtime_provider_inspection_repairs_ended_session_and_refuses_changed_unproved_url() {
    let fixture = Fixture::new("success");
    assert_eq!(outcome(&fixture.shell, &fixture.setup()), "succeeded");
    let reply = fixture.call();
    assert_eq!(outcome(&fixture.shell, &reply), "succeeded", "{reply}");
    // A provider-side session close is observed as a 404 by the next bridge POST.
    fs::write(fixture.temp.path().join("mode"), "ended-session").unwrap();
    let reply = fixture.setup();
    assert_eq!(outcome(&fixture.shell, &reply), "succeeded", "{reply}");
    let reply = fixture.call();
    assert_eq!(outcome(&fixture.shell, &reply), "unknown", "{reply}");
    deadline(|| fixture.runner.running_count() == 1);
    fs::write(fixture.temp.path().join("mode"), "success").unwrap();
    let reply = fixture.setup();
    assert_eq!(
        outcome(&fixture.shell, &reply),
        "succeeded",
        "{reply}; {:?}",
        fixture.runner.diagnostics()
    );
    let reply = fixture.call();
    assert_eq!(outcome(&fixture.shell, &reply), "succeeded", "{reply}");
    assert_eq!(
        log(fixture.temp.path())
            .iter()
            .filter(|m| m["params"]["name"] == "run.v1")
            .count(),
        1
    );
    fs::write(fixture.temp.path().join("mode"), "bad-inspect").unwrap();
    let reply = fixture.setup();
    assert!(
        ["failed", "unknown"].contains(&outcome(&fixture.shell, &reply).as_str()),
        "{reply}"
    );
    assert!(
        fixture
            .runner
            .diagnostics()
            .iter()
            .any(|s| s.contains("certificate did not prove the instance"))
    );
    assert!(!fixture.temp.path().join("bad-gate-count").exists());
    deadline(|| fixture.runner.running_count() == 1);
    let state = fixture.shell.state().unwrap();
    let object = state
        .objects
        .iter()
        .find(|o| o["id"] == id(&fixture.shell, "a"))
        .unwrap();
    assert!(
        object["status"]["conditions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["type"] == "Running" && c["status"] == "Unknown")
    );
    assert_private(&fixture);
    let records = serde_json::to_string(&fixture.shell.state().unwrap()).unwrap();
    assert!(
        !records.contains("BEGIN CERTIFICATE")
            && !records.contains("PRIVATE KEY")
            && !records.contains("eyJhbGci")
    );
}
#[test]
fn runtime_provider_cancelled_setup_sends_nothing_and_starts_no_bridge() {
    let fixture = Fixture::new("success");
    let invocation = "cancelled-provider-setup";
    let state = fixture.shell.state().unwrap();
    let subject = id(&fixture.shell, "a");
    let provider = id(&fixture.shell, "provider");
    let runtime = state.objects.iter().find(|o| o["id"] == provider).unwrap();
    let machine = &state
        .objects
        .iter()
        .find(|o| o["kind"] == "Resource" && o["name"] == "local" && o.get("spec").is_none())
        .unwrap()["status"];
    let schema = &fixture.shell.context.core_environment.scopes["core"].contracts["run.v1"]["spec"]
        ["inputSchema"]["properties"]["identity"]["properties"];
    let identity = fixture
        .shell
        .materialize_provider_identity(&json!({
            "materializeFor":subject,
            "environmentNames":schema["environment"]["required"],
            "fileNames":schema["files"]["required"]
        }))
        .unwrap()
        .unwrap();
    let intent = json!({
        "kind":"provider-call", "subject":subject, "provider":provider,
        "operation":format!("{}#run.v1",fixture.shell.context.core_package),
        "invocation":invocation, "caller":id(&fixture.shell,"root"),
        "config":runtime["config"], "revision":runtime["revision"],
        "inputs":{
            "instance":subject, "machine":machine,
            "item":{"command":[std::env::current_exe().unwrap(),"--exact","scripted_runtime_provider","--ignored","--nocapture"]},
            "environment":{"VISIBLE":"own-only"}, "packageArguments":{}
        }
    });
    fixture
        .runner
        .cancel_setup_parts(&json!({"invocation":invocation}))
        .unwrap();
    let before = log(fixture.temp.path()).len();
    fixture
        .runner
        .provider_call(&intent, Some(&identity))
        .unwrap();
    // Queue a normal inspection/setup after cancellation, proving the runner remains usable.
    assert_eq!(outcome(&fixture.shell, &fixture.setup()), "succeeded");
    let messages = log(fixture.temp.path());
    assert!(!messages[before..].iter().any(|m| m["params"]["_meta"]
        [format!("{}invocation", fixture.shell.context.meta_prefix)]
        == invocation));
    assert_eq!(
        messages
            .iter()
            .filter(|m| m["params"]["name"] == "run.v1")
            .count(),
        1
    );
    assert_eq!(groups(&fixture).len(), 2);
    assert!(
        !fixture
            .runner
            .diagnostics()
            .iter()
            .any(|d| d.contains("no https"))
    );
}

#[test]
fn runtime_provider_cancel_during_bridge_initialize_ends_group_without_stopping_provider_run() {
    // Keep the initialization deadline longer than the cancellation assertion's bound.
    let fixture = Fixture::with_initialize_timeout("blocked-initialize", Duration::from_secs(30));
    let before_groups = groups(&fixture);
    let ticket = fixture
        .shell
        .begin(
            json!({
                "kind":"invoke", "request":{
                    "identity":{"kind":"instance","id":id(&fixture.shell,"root")},
                    "tool":"s1.set-up.v1",
                    "arguments":{"instance":{"kind":"Agent","name":"a"}}
                }
            }),
            &fixture.runner,
        )
        .unwrap();
    deadline(|| {
        fs::read_to_string(fixture.temp.path().join("gate-count"))
            .ok()
            .is_some_and(|count| count == "1")
    });
    let bridge_group = *groups(&fixture)
        .iter()
        .find(|group| !before_groups.contains(group))
        .unwrap();
    assert_eq!(unsafe { libc::kill(-bridge_group, 0) }, 0);
    fixture
        .runner
        .cancel_setup_parts(&json!({"invocation":ticket.invocation().unwrap()}))
        .unwrap();
    let end = Instant::now() + Duration::from_secs(10);
    while unsafe { libc::kill(-bridge_group, 0) } == 0 {
        assert!(Instant::now() < end, "cancelled bridge group did not end");
        std::thread::sleep(Duration::from_millis(10));
    }
    deadline(|| {
        fs::read_dir(fixture.temp.path().join("store/http-bridges"))
            .unwrap()
            .next()
            .is_none()
    });
    assert_eq!(fixture.runner.running_count(), 1);
    let messages = log(fixture.temp.path());
    assert_eq!(
        messages
            .iter()
            .filter(|m| m["params"]["name"] == "run.v1")
            .count(),
        1
    );
    assert!(!messages.iter().any(|m| m["params"]["name"] == "stop.v1"));
    assert_eq!(
        fs::read_to_string(fixture.temp.path().join("gate-count")).unwrap(),
        "1"
    );
}

fn assert_private(fixture: &Fixture) {
    let export = fixture
        .shell
        .submit(json!({"kind":"export-records"}), &MissingExecutor)
        .unwrap();
    let inspection = invoke(
        &fixture.shell,
        &fixture.runner,
        "s1.inspect.v1",
        json!({"object":{"kind":"Agent","name":"a"}}),
    );
    assert_eq!(outcome(&fixture.shell, &inspection), "succeeded");
    for value in [export, inspection] {
        let text = serde_json::to_string(&value).unwrap();
        assert!(
            !text.contains("BEGIN CERTIFICATE")
                && !text.contains("PRIVATE KEY")
                && !text.contains("eyJhbGci")
        );
    }
}
