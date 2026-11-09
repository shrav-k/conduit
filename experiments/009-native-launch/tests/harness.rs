use native_launch_harness::{
    Client,
    client::{Config, arguments, prepare, settings, start_report},
    private_dir, run_environment,
    runner::{Connection, matrix},
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, os::unix::fs::PermissionsExt, path::Path, time::Duration};

const FIXTURE: &str = env!("CARGO_BIN_EXE_scripted-client");
fn wrapper(client: Client) -> &'static str {
    match client {
        Client::Claude => env!("CARGO_BIN_EXE_claude-wrapper"),
        Client::Codex => env!("CARGO_BIN_EXE_codex-wrapper"),
    }
}
async fn start(client: Client, root: &Path, seconds: u64) -> Connection {
    private_dir(&root.join("work")).unwrap();
    private_dir(&root.join("identity")).unwrap();
    let env = run_environment(
        [
            ("PATH".into(), std::env::var("PATH").unwrap()),
            (
                "ANTHROPIC_API_KEY".into(),
                "should-never-reach-client".into(),
            ),
            ("OPENAI_API_KEY".into(), "should-never-reach-client".into()),
            ("OWNER_ONLY".into(), "filtered".into()),
        ],
        "http://127.0.0.1:1/",
        &root.join("identity"),
    );
    let args = vec![
        "--evidence".into(),
        root.display().to_string(),
        "--client-program".into(),
        FIXTURE.into(),
        "--timeout-secs".into(),
        seconds.to_string(),
    ];
    let mut connection = Connection::start(
        Path::new(wrapper(client)),
        &args,
        &env,
        &root.join("work"),
        root,
        seconds,
    )
    .unwrap();
    connection.initialize().await.unwrap();
    connection
}
fn fixture(response: &Value) -> &Value {
    response["result"]["structuredContent"]["events"]
        .as_array()
        .unwrap()
        .iter()
        .find(|value| value["type"] == "fixture")
        .unwrap()
}
#[tokio::test]
async fn wrappers_keep_session_and_expose_only_one_tool() {
    for client in [Client::Claude, Client::Codex] {
        let root = tempfile::tempdir().unwrap();
        let mut connection = start(client, root.path(), 3).await;
        let invalid = connection
            .request(
                "tools/call",
                json!({"name":"turn","arguments":{"message":false}}),
            )
            .await
            .unwrap();
        assert_eq!(invalid["error"]["code"], -32602);
        let invalid = connection
            .request("tools/call", json!({"name":"missing","arguments":{}}))
            .await
            .unwrap();
        assert_eq!(invalid["error"]["code"], -32602);
        let invalid = connection
            .request(
                "tools/call",
                json!({"name":"turn","arguments":{"message":"x","extra":1}}),
            )
            .await
            .unwrap();
        assert_eq!(invalid["error"]["code"], -32602);
        assert!(!root.path().join("turn-1-plan.json").exists());
        let first = connection.turn(1).await.unwrap();
        let second = connection.turn(2).await.unwrap();
        assert_eq!(
            first["result"]["structuredContent"]["text"],
            "fixture turn 1"
        );
        assert_eq!(
            second["result"]["structuredContent"]["text"],
            "fixture turn 2"
        );
        let one = fixture(&first);
        let two = fixture(&second);
        assert_eq!(
            one["cwd"],
            root.path()
                .join("work")
                .canonicalize()
                .unwrap()
                .display()
                .to_string()
        );
        assert!(
            fs::read_dir(root.path().join("work"))
                .unwrap()
                .next()
                .is_none()
        );
        for key in one["environment"].as_object().unwrap().keys() {
            assert!(
                ["PATH", "CONDUIT_URL", "CONDUIT_IDENTITY_DIR"].contains(&key.as_str()),
                "unexpected {key}"
            );
        }
        if client == Client::Claude {
            assert_eq!(one["pid"], two["pid"]);
            assert_eq!(
                first["result"]["structuredContent"]["start_report"]["authentication"]["status"],
                "reported"
            );
        } else {
            assert_ne!(one["pid"], two["pid"]);
            let args = two["args"].as_array().unwrap();
            assert!(
                args.contains(&json!("resume")) && args.contains(&json!("fixture-exact-session"))
            );
            assert_eq!(
                second["result"]["structuredContent"]["start_report"]["authentication"]["status"],
                "not_reported"
            );
        }
        connection.stop().await;
        let exchange = fs::read_to_string(root.path().join("exchange.jsonl")).unwrap();
        assert!(exchange.contains("from_wrapper"));
    }
}
#[tokio::test]
async fn incomplete_error_and_timeout_poison_session() {
    for client in [Client::Claude, Client::Codex] {
        for prompt in ["EOF", "ERROR", "HANG"] {
            let root = tempfile::tempdir().unwrap();
            let mut connection = start(client, root.path(), 1).await;
            let failed = connection
                .request(
                    "tools/call",
                    json!({"name":"turn","arguments":{"message":prompt}}),
                )
                .await
                .unwrap();
            assert_eq!(
                failed["result"]["isError"], true,
                "{client:?} {prompt}: {failed}"
            );
            let again = connection
                .request(
                    "tools/call",
                    json!({"name":"turn","arguments":{"message":"next"}}),
                )
                .await
                .unwrap();
            assert_eq!(again["result"]["isError"], true);
            assert!(!root.path().join("turn-2-plan.json").exists());
            assert!(root.path().join("turn-1-events.jsonl").exists());
            connection.stop().await;
        }
    }
}
#[tokio::test]
async fn codex_rejects_changed_session_and_failed_exit() {
    for prompt in ["MISMATCH", "BAD_EXIT"] {
        let root = tempfile::tempdir().unwrap();
        let mut connection = start(Client::Codex, root.path(), 3).await;
        connection.turn(1).await.unwrap();
        let failed = connection
            .request(
                "tools/call",
                json!({"name":"turn","arguments":{"message":prompt}}),
            )
            .await
            .unwrap();
        assert_eq!(failed["result"]["isError"], true);
        connection.stop().await;
    }
}
#[test]
fn environment_flags_and_reports() {
    let source = [
        ("HOME", "/owner"),
        ("LOGNAME", "me"),
        ("PATH", "/bin"),
        ("SHELL", "()bad"),
        ("TERM", "x"),
        ("USER", "me"),
        ("CONDUIT_URL", "evil"),
        ("OPENAI_API_KEY", "secret"),
        ("LANG", "secret"),
    ]
    .map(|(k, v)| (k.into(), v.into()));
    let env = run_environment(source, "http://stub/", Path::new("/private/identity"));
    assert_eq!(env.len(), 7);
    assert!(!env.contains_key("SHELL"));
    assert_eq!(env["CONDUIT_URL"], "http://stub/");
    for client in [Client::Claude, Client::Codex] {
        let config = Config {
            client,
            program: client.name().into(),
            helper: "/helper with 'quote".into(),
            evidence: "/evidence".into(),
            cwd: "/work".into(),
            env: BTreeMap::new(),
            url: "http://stub/".into(),
            timeout: Duration::from_secs(3),
        };
        for session in [None, Some("exact")] {
            let args = arguments(&config, session, "message");
            let expected = if client == Client::Claude {
                vec![
                    "--setting-sources",
                    "--strict-mcp-config",
                    "--mcp-config",
                    "--tools",
                    "",
                    "mcp__stub__*",
                    "--input-format",
                ]
            } else {
                vec![
                    "--ignore-user-config",
                    "--strict-config",
                    "--disable",
                    "apps",
                    "read-only",
                    "mcp_servers.stub.default_tools_approval_mode=\"approve\"",
                ]
            };
            for flag in expected {
                assert!(args.iter().any(|arg| arg == flag), "{flag}");
            }
            assert!(!args.iter().any(|arg| arg == "--ephemeral"));
        }
    }
    assert_eq!(settings()["crossSessionInbound"], "refuse");
    assert_eq!(settings()["disableClaudeAiConnectors"], true);
    assert_eq!(
        settings()["permissions"]["deny"],
        json!(["SendMessage", "ListAgents", "RemoteTrigger"])
    );
    assert_eq!(
        start_report(&[])["authentication"]["status"],
        "not_reported"
    );
}
#[test]
fn header_helpers_use_client_specific_directory_sources() {
    let root = tempfile::tempdir().unwrap();
    let names = real_client_harness::Names::default();
    let identity = root.path().join("identity with 'quote");
    let fallback = root.path().join("environment-identity");
    for (dir, token) in [
        (&identity, "argument-token"),
        (&fallback, "environment-token"),
    ] {
        private_dir(dir).unwrap();
        fs::write(dir.join(names.get("tokenFile")), token).unwrap();
    }
    for client in [Client::Claude, Client::Codex] {
        let evidence = root.path().join(client.name());
        private_dir(&evidence).unwrap();
        let helper = evidence.join("helper with 'quote");
        std::os::unix::fs::symlink(wrapper(client), &helper).unwrap();
        let config = Config {
            client,
            program: FIXTURE.into(),
            helper,
            evidence,
            cwd: root.path().into(),
            env: run_environment([], "http://stub/", &identity),
            url: "http://stub/".into(),
            timeout: Duration::from_secs(3),
        };
        prepare(&config).unwrap();
        for session in [None, Some("exact")] {
            let args = arguments(&config, session, "message");
            let command = if client == Client::Codex {
                assert!(!args.iter().any(|arg| arg.contains("env_vars")));
                let setting = args
                    .iter()
                    .find_map(|arg| arg.strip_prefix("mcp_servers.stub.http_headers_helper="))
                    .unwrap();
                let command: String = serde_json::from_str(setting).unwrap();
                assert!(command.contains("--identity-dir"));
                command
            } else {
                assert!(!args.iter().any(|arg| arg.contains("--identity-dir")));
                let path = &args[args.iter().position(|arg| arg == "--mcp-config").unwrap() + 1];
                let mcp: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
                let command = mcp["mcpServers"]["stub"]["headersHelper"].as_str().unwrap();
                assert!(!command.contains("--identity-dir"));
                command.to_owned()
            };
            // Execute the actual configured command, including shell quoting. Codex must work
            // both without the variable and with a conflicting value; Claude uses the variable.
            for variable_present in [false, true] {
                if client == Client::Claude && !variable_present {
                    continue;
                }
                let mut helper = std::process::Command::new("/bin/sh");
                helper.env_clear().args(["-c", &command]);
                if variable_present {
                    helper.env(names.get("identityDirVariable"), &fallback);
                }
                let output = helper.output().unwrap();
                assert!(output.status.success(), "{client:?}: helper failed");
                let headers: Value = serde_json::from_slice(&output.stdout).unwrap();
                assert_eq!(
                    headers["Authorization"],
                    if client == Client::Codex {
                        "Bearer argument-token"
                    } else {
                        "Bearer environment-token"
                    }
                );
            }
        }
    }
}

#[test]
fn dry_run_is_complete_requires_output_and_has_no_side_effects() {
    let root = tempfile::tempdir().unwrap();
    let output = root.path().join("never-created");
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_native-launch-harness"))
        .args(["--dry-run", "--output"])
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let text = String::from_utf8(result.stdout).unwrap();
    assert_eq!(
        text.lines().filter(|line| line.starts_with("RUN ")).count(),
        9
    );
    assert_eq!(
        text.lines()
            .filter(|line| line.contains("\"instance\""))
            .count(),
        12
    );
    for (id, _) in matrix() {
        assert!(text.contains(&format!("RUN {id}\n")));
    }
    assert!(!output.exists());
    let missing = std::process::Command::new(env!("CARGO_BIN_EXE_native-launch-harness"))
        .arg("--dry-run")
        .output()
        .unwrap();
    assert!(!missing.status.success());
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("should-not-exist");
    assert!(native_launch_harness::runner::validate_output(&repository).is_err());
    std::os::unix::fs::symlink(env!("CARGO_MANIFEST_DIR"), root.path().join("alias")).unwrap();
    assert!(native_launch_harness::runner::validate_output(&root.path().join("alias/x")).is_err());
}

#[tokio::test]
async fn shared_stub_attributes_two_credentials_and_changes_access() {
    use real_client_harness::{Names, stub::start_agents};
    let root = tempfile::tempdir().unwrap();
    let a = root.path().join("a");
    let b = root.path().join("b");
    let names = Names::default();
    let stub = start_agents(
        root.path(),
        &[
            ("spiffe://experiment.test/a", &a),
            ("spiffe://experiment.test/b", &b),
        ],
    )
    .await
    .unwrap();
    let http = reqwest::Client::new();
    let mut tokens = Vec::new();
    for identity in [&a, &b] {
        assert_eq!(
            fs::metadata(identity).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            fs::metadata(identity.join(names.get("tokenFile")))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
        assert!(identity.join(names.get("x509BundleFile")).exists());
        let helper = std::process::Command::new(wrapper(Client::Claude))
            .arg("--headers-helper")
            .env(names.get("identityDirVariable"), identity)
            .output()
            .unwrap();
        assert!(helper.status.success());
        let headers: Value = serde_json::from_slice(&helper.stdout).unwrap();
        tokens.push(
            headers["Authorization"]
                .as_str()
                .unwrap()
                .strip_prefix("Bearer ")
                .unwrap()
                .to_string(),
        );
    }
    assert_ne!(tokens[0], tokens[1]);
    async fn request(
        http: &reqwest::Client,
        url: &str,
        token: &str,
        id: &str,
        method: &str,
        mut params: Value,
    ) -> Value {
        params["_meta"] = json!({"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientInfo":{"name":"scripted-test","version":"1"},"io.modelcontextprotocol/clientCapabilities":{}});
        let response = http
            .post(url)
            .bearer_auth(token)
            .header("MCP-Protocol-Version", "2026-07-28")
            .header("Mcp-Method", method)
            .header("Mcp-Name", params["name"].as_str().unwrap_or(""))
            .header("accept", "application/json, text/event-stream")
            .json(&json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))
            .send()
            .await
            .unwrap();
        let status = response.status();
        let text = response.text().await.unwrap();
        assert_eq!(status, 200, "{text}");
        let response: Value = serde_json::from_str(&text).unwrap_or_else(|_| {
            serde_json::from_str(
                text.lines()
                    .find_map(|line| line.strip_prefix("data: "))
                    .unwrap(),
            )
            .unwrap()
        });
        assert_eq!(response["id"], id);
        response
    }
    let mut expected = BTreeMap::new();
    for (agent, subject) in [
        ("a", "spiffe://experiment.test/a"),
        ("b", "spiffe://experiment.test/b"),
    ] {
        for step in ["before", "after", "closed", "open"] {
            expected.insert(format!("{agent}-{step}"), Some(subject));
        }
    }
    expected.insert("invalid".into(), None);
    let (one, two) = tokio::join!(
        request(
            &http,
            &stub.url,
            &tokens[0],
            "a-before",
            "tools/list",
            json!({})
        ),
        request(
            &http,
            &stub.url,
            &tokens[1],
            "b-before",
            "tools/list",
            json!({})
        )
    );
    for response in [one, two] {
        assert!(
            response["result"]["tools"]
                .as_array()
                .unwrap()
                .iter()
                .any(|tool| tool["name"] == "files.read.v1")
        );
    }
    stub.change_access().await;
    for (agent, token) in ["a", "b"].into_iter().zip(&tokens) {
        let response = request(
            &http,
            &stub.url,
            token,
            &format!("{agent}-after"),
            "tools/list",
            json!({}),
        )
        .await;
        let tools = response["result"]["tools"].as_array().unwrap();
        assert!(!tools.iter().any(|tool| tool["name"] == "files.read.v1"));
        assert!(tools.iter().any(|tool| tool["name"] == "notes.read.v1"));
        let refused = request(
            &http,
            &stub.url,
            token,
            &format!("{agent}-closed"),
            "tools/call",
            json!({"name":"files.read.v1","arguments":{"path":"hello.txt"}}),
        )
        .await;
        assert!(refused.get("error").is_some());
        let ok = request(
            &http,
            &stub.url,
            token,
            &format!("{agent}-open"),
            "tools/call",
            json!({"name":"notes.read.v1","arguments":{"path":"hello.txt"}}),
        )
        .await;
        assert_eq!(ok["result"]["isError"], false);
    }
    let rejected = http
        .post(&stub.url)
        .bearer_auth("invalid")
        .header("accept", "application/json, text/event-stream")
        .json(&json!({"jsonrpc":"2.0","id":"invalid","method":"tools/list"}))
        .send()
        .await
        .unwrap();
    assert_eq!(rejected.status(), 401);
    rejected.text().await.unwrap();
    stub.cancel.cancel();
    stub.task.await.unwrap().unwrap();
    let log = fs::read_to_string(root.path().join("stub.jsonl")).unwrap();
    for token in &tokens {
        assert!(!log.contains(token));
    }
    let requests: Vec<Value> = log
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|v| v["kind"] == "request")
        .collect();
    assert_eq!(requests.len(), expected.len());
    for request in requests {
        let body: Value = serde_json::from_str(request["body"].as_str().unwrap()).unwrap();
        let id = body["id"].as_str().unwrap();
        let subject = expected
            .remove(id)
            .expect("unexpected or duplicate request ID");
        assert_eq!(request["instance"], json!(subject), "request {id}");
        assert_eq!(request["token_valid"], subject.is_some(), "request {id}");
    }
    assert!(expected.is_empty(), "missing requests: {expected:?}");
}

#[tokio::test]
async fn stub_redacts_both_tokens_in_request_and_response_logs() {
    use real_client_harness::{Names, stub::start_agents};
    let root = tempfile::tempdir().unwrap();
    let a = root.path().join("a");
    let b = root.path().join("b");
    let stub = start_agents(
        root.path(),
        &[
            ("spiffe://experiment.test/a", &a),
            ("spiffe://experiment.test/b", &b),
        ],
    )
    .await
    .unwrap();
    let names = Names::default();
    let tokens = [&a, &b].map(|dir| {
        fs::read_to_string(dir.join(names.get("tokenFile")))
            .unwrap()
            .trim()
            .to_owned()
    });
    assert_ne!(tokens[0], tokens[1]);
    // Both throwaway tokens occur in captured data, beyond the excluded Authorization header.
    // JSON-RPC echoes this string ID, exercising request-body and response-wire redaction.
    let id = tokens.join("|");
    let response = reqwest::Client::new()
        .post(&stub.url)
        .bearer_auth(&tokens[0])
        .header("accept", "application/json, text/event-stream")
        .header("MCP-Protocol-Version", "2026-07-28")
        .header("Mcp-Method", "tools/list")
        .json(
            &json!({"jsonrpc":"2.0","id":id,"method":"tools/list","params":{
                "_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28",
                    "io.modelcontextprotocol/clientInfo":{"name":"redaction-test","version":"1"},
                    "io.modelcontextprotocol/clientCapabilities":{}}
            }}),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let text = response.text().await.unwrap();
    assert!(text.contains(&id), "response must echo the unredacted ID");
    stub.cancel.cancel();
    stub.task.await.unwrap().unwrap();
    let log = fs::read_to_string(root.path().join("stub.jsonl")).unwrap();
    for token in &tokens {
        assert!(!log.contains(token), "throwaway token leaked into stub log");
    }
    let entries: Vec<Value> = log
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let requests: Vec<_> = entries.iter().filter(|e| e["kind"] == "request").collect();
    assert_eq!(requests.len(), 1);
    let body: Value = serde_json::from_str(requests[0]["body"].as_str().unwrap()).unwrap();
    assert_eq!(body["id"], "[REDACTED]|[REDACTED]");
    let wire = entries
        .iter()
        .filter(|e| e["kind"] == "wire" && e["id"] == requests[0]["id"])
        .map(|e| e["data"].as_str().unwrap())
        .collect::<String>();
    assert!(wire.contains("[REDACTED]|[REDACTED]"));
}

#[test]
fn runner_executes_entire_matrix_with_only_scripted_clients() {
    let root = tempfile::tempdir().unwrap();
    let tools = root.path().join("tools");
    private_dir(&tools).unwrap();
    for client in ["claude", "codex"] {
        std::os::unix::fs::symlink(FIXTURE, tools.join(client)).unwrap();
    }
    let output = root.path().join("evidence");
    let result = std::process::Command::new(env!("CARGO_BIN_EXE_native-launch-harness"))
        .args(["--output"])
        .arg(&output)
        .args(["--timeout-secs", "3"])
        .env("PATH", &tools)
        .env("ANTHROPIC_API_KEY", "not-a-credential-fixture")
        .env("OPENAI_API_KEY", "not-a-credential-fixture")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "stdout: {} stderr: {}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    for (id, clients) in matrix() {
        let run = output.join(&id);
        let status: Value =
            serde_json::from_slice(&fs::read(run.join("status.json")).unwrap()).unwrap();
        assert_eq!(status["completed"], true);
        let log = fs::read_to_string(run.join("stub.jsonl")).unwrap();
        let changes: Vec<Value> = log
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .filter(|v| v["kind"] == "list_changed")
            .collect();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0]["turn"], 2);
        assert_eq!(
            changes[0]["tools"],
            json!(["control.next.v1", "notes.read.v1"])
        );
        for client in clients {
            let agent = run.join(client.name());
            assert!(fs::read_dir(agent.join("work")).unwrap().next().is_none());
            let launch: Value =
                serde_json::from_slice(&fs::read(agent.join("launch.json")).unwrap()).unwrap();
            for key in launch["environment"].as_object().unwrap().keys() {
                assert!(
                    [
                        "HOME",
                        "LOGNAME",
                        "PATH",
                        "SHELL",
                        "TERM",
                        "USER",
                        "CONDUIT_URL",
                        "CONDUIT_IDENTITY_DIR"
                    ]
                    .contains(&key.as_str())
                );
            }
            for turn in 1..=2 {
                let response: Value = serde_json::from_slice(
                    &fs::read(agent.join(format!("turn-{turn}-response.json"))).unwrap(),
                )
                .unwrap();
                assert_eq!(
                    fixture(&response)["environment"]["CONDUIT_IDENTITY_DIR"],
                    agent.join("identity").display().to_string()
                );
                assert!(
                    fixture(&response)["environment"]
                        .get("ANTHROPIC_API_KEY")
                        .is_none()
                );
                assert!(
                    fixture(&response)["environment"]
                        .get("OPENAI_API_KEY")
                        .is_none()
                );
                assert!(
                    agent
                        .join(format!("turn-{turn}-start-report.json"))
                        .exists()
                );
            }
        }
    }
}
