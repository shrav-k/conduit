use agent_adapter::{CLAUDE, Config, Session, arguments, header, helper_command, prepare};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    process::{Command, Stdio},
    time::Duration,
};
fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_agent-codex")
}
fn config(parent: &std::path::Path, account_services: bool, peer_messaging: bool) -> Config {
    let cwd = parent.join("work");
    fs::create_dir(&cwd).unwrap();
    let cwd = cwd.canonicalize().unwrap();
    let identity = parent.join("identity");
    fs::create_dir(&identity).unwrap();
    fs::write(identity.join("token"), "synthetic-test-token\n").unwrap();
    let mut env = BTreeMap::from([
        ("PATH".into(), std::env::var("PATH").unwrap()),
        ("HOME".into(), parent.display().to_string()),
        ("CONDUIT_URL".into(), "http://127.0.0.1:1/mcp".into()),
        (
            "CONDUIT_IDENTITY_DIR".into(),
            identity.display().to_string(),
        ),
    ]);
    env.insert("ANTHROPIC_API_KEY".into(), "synthetic-anthropic-key".into());
    env.insert("OPENAI_API_KEY".into(), "synthetic-openai-key".into());
    Config {
        program: env!("CARGO_BIN_EXE_scripted-client").into(),
        helper: binary().into(),
        cwd,
        identity,
        env,
        url: "http://127.0.0.1:1/mcp".into(),
        account_services,
        peer_messaging,
        model: Some("test-model".into()),
        timeout: Duration::from_secs(10),
    }
}
fn expected_args(c: &Config, thread: Option<&str>) -> Vec<String> {
    if CLAUDE {
        let mut a: Vec<String> = ["--print", "--setting-sources", "", "--settings"]
            .map(str::to_owned)
            .into();
        a.push(c.cwd.join("settings.json").display().to_string());
        a.extend([
            "--strict-mcp-config".into(),
            "--mcp-config".into(),
            c.cwd.join("mcp.json").display().to_string(),
        ]);
        a.extend(
            [
                "--model",
                "test-model",
                "--tools",
                if c.peer_messaging {
                    "SendMessage,ListAgents,RemoteTrigger"
                } else {
                    ""
                },
                "--allowedTools",
                if c.peer_messaging {
                    "mcp__engine__*,SendMessage,ListAgents,RemoteTrigger"
                } else {
                    "mcp__engine__*"
                },
                "--no-session-persistence",
                "--output-format",
                "stream-json",
                "--input-format",
                "stream-json",
                "--verbose",
                "--include-partial-messages",
                "--disable-slash-commands",
            ]
            .map(str::to_owned),
        );
        if !c.peer_messaging {
            a.extend(
                ["--disallowedTools", "SendMessage,ListAgents,RemoteTrigger"].map(str::to_owned),
            );
        }
        a
    } else {
        let mut a: Vec<String> = [
            "exec",
            "--sandbox",
            "read-only",
            "--color",
            "never",
            "--model",
            "test-model",
        ]
        .map(str::to_owned)
        .into();
        if thread.is_some() {
            a.push("resume".into());
        }
        a.extend(
            [
                "--ignore-user-config",
                "--strict-config",
                "--ignore-rules",
                "--skip-git-repo-check",
                "--json",
            ]
            .map(str::to_owned),
        );
        a.extend([
            if c.account_services {
                "--enable"
            } else {
                "--disable"
            }
            .into(),
            "apps".into(),
        ]);
        for feature in [
            "shell_tool",
            "unified_exec",
            "browser_use",
            "browser_use_external",
            "in_app_browser",
            "computer_use",
            "multi_agent",
            "multi_agent_v2",
            "multi_agent_v2_dynamic_tools",
            "hooks",
            "plugins",
            "view_image",
            "goals",
            "image_generation",
        ] {
            a.extend(["--disable".into(), feature.into()]);
        }
        let expected_helper = format!(
            "'{}' --headers-helper --identity-dir '{}'",
            binary(),
            c.identity.display()
        );
        for s in [
            format!("mcp_servers.engine.url={}", json!(c.url)),
            format!(
                "mcp_servers.engine.http_headers_helper={}",
                json!(expected_helper)
            ),
            "mcp_servers.engine.startup_readiness=\"connection\"".into(),
            "mcp_servers.engine.required=true".into(),
            "mcp_servers.engine.default_tools_approval_mode=\"approve\"".into(),
            "web_search=\"disabled\"".into(),
            "project_doc_max_bytes=0".into(),
        ] {
            a.extend(["-c".into(), s]);
        }
        if let Some(t) = thread {
            a.push(t.into());
        }
        a.push("-".into());
        a
    }
}
#[tokio::test]
async fn four_reach_combinations_exact_launch_and_same_conversation() {
    for account in [false, true] {
        for peer in [false, true] {
            let parent = tempfile::tempdir().unwrap();
            let c = config(parent.path(), account, peer);
            prepare(&c).unwrap();
            let mut session = Session::new(c.clone());
            assert!(!c.cwd.join("client-started").exists());
            let mut previous_pid = None;
            for i in 1..=2 {
                let value = session.turn("TEST").await.unwrap();
                let evidence: Value =
                    serde_json::from_str(value["text"].as_str().unwrap()).unwrap();
                assert_eq!(
                    evidence["args"],
                    json!(expected_args(
                        &c,
                        if i == 2 && !CLAUDE {
                            Some("fixture-exact-thread")
                        } else {
                            None
                        }
                    ))
                );
                let mut env = c.env.clone();
                env.remove("ANTHROPIC_API_KEY");
                env.remove("OPENAI_API_KEY");
                // macOS may add its own process text encoding; no ambient user values allowed.
                let actual = evidence["environment"].as_object().unwrap();
                for (k, v) in &env {
                    assert_eq!(actual.get(k), Some(&json!(v)));
                }
                assert!(
                    actual
                        .keys()
                        .all(|k| env.contains_key(k) || k == "__CF_USER_TEXT_ENCODING")
                );
                assert_eq!(evidence["cwd"], json!(c.cwd));
                assert_eq!(evidence["number"], i);
                if let Some(pid) = previous_pid {
                    if CLAUDE {
                        assert_eq!(evidence["pid"], pid);
                    } else {
                        assert_ne!(evidence["pid"], pid);
                    }
                }
                previous_pid = Some(evidence["pid"].clone());
                if CLAUDE {
                    let mut expected_settings = json!({"permissions":{"allow": if peer {vec!["mcp__engine__*","SendMessage","ListAgents","RemoteTrigger"]} else {vec!["mcp__engine__*"]},"deny":if peer {vec![]} else {vec!["SendMessage","ListAgents","RemoteTrigger"]},"defaultMode":"dontAsk"},"disableAllHooks":true,"enabledPlugins":{},"disableClaudeAiConnectors":!account});
                    if !peer {
                        expected_settings["crossSessionInbound"] = json!("refuse");
                    }
                    assert_eq!(evidence["settings"], expected_settings);
                }
                assert_eq!(
                    value["clientTools"]["status"],
                    if i > 1 { "unavailable" } else { "reported" }
                );
                assert_eq!(
                    value["clientTools"]["observations"][0]["names"],
                    json!([format!("mcp__engine__fixture{i}")])
                );
                for secret in [
                    "synthetic-test-token",
                    "synthetic-anthropic-key",
                    "synthetic-openai-key",
                ] {
                    assert!(!value.to_string().contains(secret));
                }
            }
            session.shutdown().await;
            assert_eq!(
                header(&c.identity).unwrap(),
                json!({"Authorization":"Bearer synthetic-test-token"})
            );
            let output = Command::new(binary())
                .arg("--headers-helper")
                .arg("--identity-dir")
                .arg(&c.identity)
                .env_clear()
                .output()
                .unwrap();
            assert!(output.status.success());
            assert_eq!(
                output.stdout,
                b"{\"Authorization\":\"Bearer synthetic-test-token\"}\n"
            );
            assert!(output.stderr.is_empty());
            let env_helper = Command::new(binary())
                .arg("--headers-helper")
                .env_clear()
                .env("CONDUIT_IDENTITY_DIR", &c.identity)
                .output()
                .unwrap();
            assert_eq!(env_helper.stdout, output.stdout);
            assert!(env_helper.stderr.is_empty());
            fs::write(c.identity.join("token"), "rotated-synthetic-token").unwrap();
            assert_eq!(
                header(&c.identity).unwrap(),
                json!({"Authorization":"Bearer rotated-synthetic-token"})
            );
            fs::write(c.identity.join("token"), "bad\nsynthetic-token").unwrap();
            let invalid = Command::new(binary())
                .args(["--headers-helper", "--identity-dir"])
                .arg(&c.identity)
                .output()
                .unwrap();
            assert!(!invalid.status.success());
            assert!(invalid.stdout.is_empty());
            assert!(!String::from_utf8_lossy(&invalid.stderr).contains("synthetic-token"));
            if CLAUDE {
                assert!(!helper_command(&c).contains("--identity-dir"));
            }
            let mut no_model = c.clone();
            no_model.model = None;
            assert!(!arguments(&no_model, None).contains(&"--model".into()));
        }
    }
}
#[tokio::test]
async fn stdio_exposes_only_turn_and_setup_never_launches_client() {
    use rmcp::{ServiceExt, model::CallToolRequestParams};
    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    let mut command = tokio::process::Command::new(binary());
    let mut child = command
        .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
        .env_clear()
        .envs(&c.env)
        .env("AGENT_ACCOUNT_SERVICES", "false")
        .env("AGENT_PEER_MESSAGING", "false")
        .env("AMBIENT_SENTINEL", "synthetic-ambient-secret")
        .current_dir(&c.cwd)
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let service =
        ().serve((child.stdout.take().unwrap(), child.stdin.take().unwrap()))
            .await
            .unwrap();
    let tools = service.list_all_tools().await.unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "turn");
    assert!(!c.cwd.join("client-started").exists());
    let invalid: CallToolRequestParams =
        serde_json::from_value(json!({"name":"other","arguments":{}})).unwrap();
    assert!(service.call_tool(invalid).await.is_err());
    for message in [json!({"message":1}), json!({"message":"TEST","extra":true})] {
        let call = serde_json::from_value(json!({"name":"turn","arguments":message})).unwrap();
        assert!(service.call_tool(call).await.is_err());
    }
    let normal =
        serde_json::from_value(json!({"name":"turn","arguments":{"message":"TEST"}})).unwrap();
    let observed = service.call_tool(normal).await.unwrap();
    assert_eq!(observed.is_error, Some(false));
    let wire = serde_json::to_value(observed).unwrap();
    let launch: Value =
        serde_json::from_str(wire["structuredContent"]["text"].as_str().unwrap()).unwrap();
    for key in [
        "ANTHROPIC_API_KEY",
        "OPENAI_API_KEY",
        "AMBIENT_SENTINEL",
        "AGENT_ACCOUNT_SERVICES",
        "AGENT_PEER_MESSAGING",
    ] {
        assert!(launch["environment"].get(key).is_none());
    }
    let call =
        serde_json::from_value(json!({"name":"turn","arguments":{"message":"ERROR"}})).unwrap();
    assert_eq!(service.call_tool(call).await.unwrap().is_error, Some(true));
    let call =
        serde_json::from_value(json!({"name":"turn","arguments":{"message":"TEST"}})).unwrap();
    assert_eq!(service.call_tool(call).await.unwrap().is_error, Some(true));
    service.cancel().await.unwrap();
    let out = child.wait_with_output().await.unwrap();
    for secret in [
        "synthetic-test-token",
        "synthetic-anthropic-key",
        "synthetic-openai-key",
    ] {
        assert!(!String::from_utf8_lossy(&out.stderr).contains(secret));
    }
}

#[tokio::test]
async fn absent_native_catalog_is_unreported_and_eof_poisons_session() {
    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    prepare(&c).unwrap();
    let mut session = Session::new(c);
    let reply = session.turn("NO_CATALOG").await.unwrap();
    assert_eq!(
        reply["clientTools"],
        json!({"status":"not_reported","observations":[]})
    );
    assert!(session.turn("EOF").await.is_err());
    assert!(session.turn("TEST").await.is_err());
    session.shutdown().await;
}

#[tokio::test]
async fn stdin_delivers_literal_dash_and_option_like_messages_on_initial_and_resumed_turns() {
    for initial in ["-", "--model unexpected", "--help\nsecond line"] {
        let parent = tempfile::tempdir().unwrap();
        let c = config(parent.path(), false, false);
        let mut session = Session::new(c);
        for message in [initial, "-", "--help", "-c key=value"] {
            let reply = session.turn(message).await.unwrap();
            let evidence: Value = serde_json::from_str(reply["text"].as_str().unwrap()).unwrap();
            assert_eq!(evidence["message"], message);
            assert_eq!(evidence["args"].as_array().unwrap().last().unwrap(), "-");
            assert!(
                !evidence["args"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|a| a == message && message != "-")
            );
        }
        session.shutdown().await;
    }
}

async fn wait_for(path: &std::path::Path) {
    tokio::time::timeout(Duration::from_secs(10), async {
        while !path.exists() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
}
#[tokio::test]
async fn cancellation_skips_queued_work_and_stops_active_work() {
    use rmcp::{ServiceExt, model::*, service::PeerRequestOptions};
    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    let mut child = tokio::process::Command::new(binary())
        .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
        .env_clear()
        .envs(&c.env)
        .env("AGENT_ACCOUNT_SERVICES", "false")
        .env("AGENT_PEER_MESSAGING", "false")
        .current_dir(&c.cwd)
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let service =
        ().serve((child.stdout.take().unwrap(), child.stdin.take().unwrap()))
            .await
            .unwrap();
    let request = |message: &str| -> ClientRequest {
        serde_json::from_value(
            json!({"method":"tools/call","params":{"name":"turn","arguments":{"message":message}}}),
        )
        .unwrap()
    };
    let active = service
        .send_request_with_option(request("BLOCK"), PeerRequestOptions::no_options())
        .await
        .unwrap();
    wait_for(&c.cwd.join("blocked")).await;
    let queued = service
        .send_request_with_option(request("QUEUED"), PeerRequestOptions::no_options())
        .await
        .unwrap();
    // Give the server's concurrently dispatched request time to wait on the held lock.
    tokio::time::sleep(Duration::from_millis(100)).await;
    queued
        .cancel(Some("cancel queued turn".into()))
        .await
        .unwrap();
    // A subsequent request is an ordering barrier for receipt of the notification.
    service.list_all_tools().await.unwrap();
    fs::write(c.cwd.join("release"), b"release").unwrap();
    active.await_response().await.unwrap();
    let next: CallToolRequestParams =
        serde_json::from_value(json!({"name":"turn","arguments":{"message":"TEST"}})).unwrap();
    assert_eq!(service.call_tool(next).await.unwrap().is_error, Some(false));
    assert!(!c.cwd.join("queued-launched").exists());

    fs::remove_file(c.cwd.join("blocked")).unwrap();
    fs::remove_file(c.cwd.join("release")).unwrap();
    let active = service
        .send_request_with_option(request("BLOCK"), PeerRequestOptions::no_options())
        .await
        .unwrap();
    wait_for(&c.cwd.join("blocked")).await;
    active
        .cancel(Some("cancel active turn".into()))
        .await
        .unwrap();
    let next: CallToolRequestParams =
        serde_json::from_value(json!({"name":"turn","arguments":{"message":"QUEUED"}})).unwrap();
    assert_eq!(
        tokio::time::timeout(Duration::from_secs(5), service.call_tool(next))
            .await
            .unwrap()
            .unwrap()
            .is_error,
        Some(true)
    );
    assert!(!c.cwd.join("queued-launched").exists());
    service.cancel().await.unwrap();
    child.wait().await.unwrap();
}
