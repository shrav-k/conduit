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
        v2: false,
        directory: None,
        effort: None,
        configuration_error: None,
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
                    assert_eq!(
                        evidence["mcp"],
                        json!({"mcpServers":{"engine":{"type":"http","url":c.url,"headersHelper":helper_command(&c)}}})
                    );
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
    let failed = tokio::time::timeout(Duration::from_secs(5), service.call_tool(next))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(failed.is_error, Some(true));
    assert!(
        serde_json::to_string(&failed)
            .unwrap()
            .contains("session failed at turn 3: cancelled; implicit restart refused")
    );
    assert!(!c.cwd.join("queued-launched").exists());
    service.cancel().await.unwrap();
    child.wait().await.unwrap();
}

fn expected_v2_args(c: &Config, thread: Option<&str>) -> Vec<String> {
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
                match (c.directory.is_some(), c.account_services, c.peer_messaging) {
                    (true, false, false) => "Bash,Read,Edit,Write,Glob,Grep,WebFetch",
                    (true, true, false) => "Bash,Read,Edit,Write,Glob,Grep,WebFetch,WebSearch",
                    (true, false, true) => "Bash,Read,Edit,Write,Glob,Grep,WebFetch,SendMessage,ListAgents,RemoteTrigger",
                    (true, true, true) => "Bash,Read,Edit,Write,Glob,Grep,WebFetch,WebSearch,SendMessage,ListAgents,RemoteTrigger",
                    (false, _, true) => "SendMessage,ListAgents,RemoteTrigger",
                    _ => "",
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
        if c.directory.is_some() {
            a.extend(
                [
                    "--permission-mode",
                    "bypassPermissions",
                    "--permission-prompts",
                    "none",
                ]
                .map(str::to_owned),
            );
        }
        if let Some(effort) = &c.effort {
            a.extend(["--effort".into(), effort.clone()]);
        }
        a
    } else {
        let mut a: Vec<String> = [
            "exec",
            "--sandbox",
            if c.directory.is_some() {
                "danger-full-access"
            } else {
                "read-only"
            },
            "--color",
            "never",
            "--model",
            "test-model",
        ]
        .map(str::to_owned)
        .into();
        a.extend([
            "--cd".into(),
            c.directory.as_ref().unwrap_or(&c.cwd).display().to_string(),
        ]);
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
        let features: &[&str] = if c.directory.is_some() {
            &[
                "browser_use",
                "browser_use_external",
                "in_app_browser",
                "computer_use",
                "multi_agent",
                "multi_agent_v2",
                "multi_agent_v2_dynamic_tools",
                "hooks",
                "plugins",
                "goals",
                "image_generation",
            ]
        } else {
            &[
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
            ]
        };
        for feature in features {
            a.extend(["--disable".into(), (*feature).into()]);
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
            if c.directory.is_some() && c.account_services {
                "web_search=\"live\""
            } else {
                "web_search=\"disabled\""
            }
            .into(),
            "project_doc_max_bytes=0".into(),
        ] {
            a.extend(["-c".into(), s]);
        }
        a.extend(["-c".into(), "mcp_servers.engine.tool_timeout_sec=10".into()]);
        if let Some(effort) = &c.effort {
            a.extend([
                "-c".into(),
                format!("model_reasoning_effort={}", json!(effort)),
            ]);
        }
        if let Some(t) = thread {
            a.push(t.into());
        }
        a.push("-".into());
        a
    }
}

#[test]
fn v2_without_directory_keeps_v1_tools_and_sandbox() {
    for account in [false, true] {
        for peer in [false, true] {
            let parent = tempfile::tempdir().unwrap();
            let mut c = config(parent.path(), account, peer);
            let v1_settings = agent_adapter::settings(&c);
            for thread in [None, Some("fixture-thread")] {
                c.v2 = false;
                let v1 = arguments(&c, thread);
                c.v2 = true;
                let mut v2 = arguments(&c, thread);
                if !CLAUDE {
                    // These .v2 working-directory and MCP timeout arguments are retained
                    // from 050a; every tool, sandbox and account argument stays .v1's.
                    for flag in ["--cd", "mcp_servers.engine.tool_timeout_sec=10"] {
                        let i = v2.iter().position(|a| a == flag).unwrap();
                        if flag == "--cd" {
                            v2.drain(i..i + 2);
                        } else {
                            assert_eq!(v2[i - 1], "-c");
                            v2.drain(i - 1..=i);
                        }
                    }
                }
                assert_eq!(v2, v1);
                assert_eq!(agent_adapter::settings(&c), v1_settings);
            }
        }
    }
}

#[tokio::test]
async fn v2_exact_launch_directories_and_private_files() {
    for account in [false, true] {
        for peer in [false, true] {
            for directory in [false, true] {
                for effort in [false, true] {
                    let parent = tempfile::tempdir().unwrap();
                    let mut c = config(parent.path(), account, peer);
                    c.v2 = true;
                    c.effort = effort.then(|| "high".into());
                    let work = parent.path().join("project");
                    fs::create_dir(&work).unwrap();
                    if directory {
                        c.directory = Some(work.canonicalize().unwrap());
                    }
                    if CLAUDE {
                        c.env.insert("MCP_TOOL_TIMEOUT".into(), "10000".into());
                    }
                    prepare(&c).unwrap();
                    let mut session = Session::new(c.clone());
                    for turn in 1..=2 {
                        let thread = (turn == 2 && !CLAUDE).then_some("fixture-exact-thread");
                        let expected = expected_v2_args(&c, thread);
                        assert_eq!(arguments(&c, thread), expected);
                        if let Some(i) = expected.iter().position(|s| s == "resume") {
                            for flag in ["--sandbox", "--model", "--cd"] {
                                assert!(expected.iter().position(|s| s == flag).unwrap() < i);
                            }
                        }
                        let reply = session.turn("TEST").await.unwrap();
                        let evidence: Value =
                            serde_json::from_str(reply["text"].as_str().unwrap()).unwrap();
                        assert_eq!(evidence["args"], json!(expected));
                        assert_eq!(
                            evidence["cwd"],
                            json!(c.directory.as_ref().unwrap_or(&c.cwd))
                        );
                        if CLAUDE {
                            assert_eq!(
                                evidence["settings"]["permissions"]["defaultMode"],
                                if directory {
                                    "bypassPermissions"
                                } else {
                                    "dontAsk"
                                }
                            );
                            assert_eq!(evidence["mcp"]["mcpServers"]["engine"]["timeout"], 10000);
                            assert_eq!(evidence["environment"]["MCP_TOOL_TIMEOUT"], "10000");
                            assert!(c.cwd.join("settings.json").exists());
                            assert!(c.cwd.join("mcp.json").exists());
                        } else {
                            assert!(evidence["environment"].get("MCP_TOOL_TIMEOUT").is_none());
                        }
                    }
                    session.shutdown().await;
                    let names: Vec<_> = fs::read_dir(&work)
                        .unwrap()
                        .map(|e| e.unwrap().file_name())
                        .collect();
                    assert_eq!(
                        names,
                        if directory {
                            vec![std::ffi::OsString::from("client-started")]
                        } else {
                            vec![]
                        }
                    );
                }
            }
        }
    }
}

async fn assert_failure(c: Config, message: &str, cause: &str, warmup: bool) {
    prepare(&c).unwrap();
    let mut session = Session::new(c.clone());
    if warmup {
        session.turn("TEST").await.unwrap();
    }
    let k = if warmup { 2 } else { 1 };
    let first = session.turn(message).await.unwrap_err().to_string();
    assert_eq!(
        first,
        format!("client turn {k} failed: {cause}; session cannot restart implicitly")
    );
    let marker = c.cwd.join("client-started");
    if marker.exists() {
        fs::remove_file(&marker).unwrap();
    }
    let next = session.turn("TEST").await.unwrap_err().to_string();
    assert_eq!(
        next,
        format!("session failed at turn {k}: {cause}; implicit restart refused")
    );
    assert!(!marker.exists());
    assert!(!first.contains("sentinel-secret"));
    assert!(!next.contains("sentinel-secret"));
}

#[tokio::test]
async fn fixed_failure_causes_in_both_versions() {
    for v2 in [false, true] {
        let mut cases = vec![
            ("EOF", "EOF before turn boundary", false),
            ("ERROR", "client failure: other", false),
            ("EXIT:7", "client exit: status 7", false),
            ("SIGNAL", "client exit: signal 9", false),
            ("INVALID_JSON", "invalid client output", false),
            ("FAIL:QuOtA", "client failure: usage-limit", false),
            ("FAIL:429", "client failure: rate-limit", false),
            ("FAIL:API KEY", "client failure: authentication", false),
            (
                "FAIL:Prompt is too long",
                "client failure: context-window",
                false,
            ),
            ("FAIL:unrecognized", "client failure: other", false),
            ("FAIL_NO_TEXT", "client failure: other", false),
            (
                "FAIL:quota 429 unauthorized context window",
                "client failure: usage-limit",
                false,
            ),
            (
                "FAIL:rate_limit credential",
                "client failure: rate-limit",
                false,
            ),
            (
                "FAIL:log in context_length",
                "client failure: authentication",
                false,
            ),
        ];
        if !CLAUDE {
            cases.extend([
                ("NO_THREAD", "no thread", false),
                ("MISSING_THREAD", "invalid client output", false),
                ("CHANGED_THREAD", "resume changed thread", true),
                ("ERROR_FAILED", "client failure: authentication", false),
                ("ERROR_EXIT", "client failure: usage-limit", false),
                ("COMPLETED_EXIT", "client exit: status 4", false),
            ]);
        }
        for (message, cause, warmup) in cases {
            let parent = tempfile::tempdir().unwrap();
            let mut c = config(parent.path(), false, false);
            if v2 {
                c.v2 = true;
            }
            assert_failure(c, message, cause, warmup).await;
        }
        let parent = tempfile::tempdir().unwrap();
        let mut c = config(parent.path(), false, false);
        if v2 {
            c.v2 = true;
        }
        c.model = Some("exit-at-start".into());
        assert_failure(c, &"X".repeat(1024 * 1024), "client exit: status 2", false).await;
        let parent = tempfile::tempdir().unwrap();
        let mut c = config(parent.path(), false, false);
        if v2 {
            c.v2 = true;
        }
        c.program = parent.path().join("absent-client");
        assert_failure(c, "TEST", "client did not start", false).await;
    }
}

#[tokio::test]
async fn deadline_and_retry_events() {
    for v2 in [false, true] {
        let parent = tempfile::tempdir().unwrap();
        let mut c = config(parent.path(), false, false);
        c.timeout = Duration::from_secs(1);
        if v2 {
            c.v2 = true;
        }
        let cause = if v2 {
            "deadline: no turn boundary within 1 seconds (input turnDeadlineSeconds)"
        } else {
            "deadline: no turn boundary within 1 seconds"
        };
        assert_failure(c, "BLOCK", cause, false).await;
        {
            let parent = tempfile::tempdir().unwrap();
            let mut c = config(parent.path(), false, false);
            if v2 {
                c.v2 = true;
            }
            prepare(&c).unwrap();
            let mut session = Session::new(c);
            assert!(session.turn("ERROR_COMPLETED").await.is_ok());
            assert!(session.turn("TEST").await.is_ok());
            session.shutdown().await;
        }
    }
}

#[tokio::test]
async fn stdio_v2_defaults_and_invalid_configuration_remain_callable() {
    use rmcp::ServiceExt;
    for case in [
        "valid",
        "directory",
        "symlink",
        "file",
        "empty-deadline",
        "relative",
        "missing",
        "zero",
        "too-large",
        "fraction",
        "effort",
        "account",
        "v1",
    ] {
        let parent = tempfile::tempdir().unwrap();
        let c = config(parent.path(), false, false);
        let mut command = tokio::process::Command::new(binary());
        command
            .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
            .env_clear()
            .envs(&c.env)
            .env("AGENT_ACCOUNT_SERVICES", "false")
            .env("AGENT_TURN_DEADLINE_SECONDS", "604800")
            .current_dir(&c.cwd)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let expected = match case {
            "directory" | "symlink" => {
                let work = parent.path().join("project");
                fs::create_dir(&work).unwrap();
                let path = if case == "symlink" {
                    let link = parent.path().join("project-link");
                    std::os::unix::fs::symlink(&work, &link).unwrap();
                    link
                } else {
                    work
                };
                command.env("AGENT_DIRECTORY", path);
                String::new()
            }
            "file" => {
                let path = parent.path().join("file");
                fs::write(&path, "not a directory").unwrap();
                command.env("AGENT_DIRECTORY", &path);
                format!("directory: not an existing directory: {}", path.display())
            }
            "relative" => {
                command.env("AGENT_DIRECTORY", "relative");
                "directory: not an absolute path: relative".into()
            }
            "missing" => {
                let path = parent.path().join("missing");
                command.env("AGENT_DIRECTORY", &path);
                format!("directory: not an existing directory: {}", path.display())
            }
            "zero" | "too-large" | "fraction" | "empty-deadline" => {
                command.env(
                    "AGENT_TURN_DEADLINE_SECONDS",
                    match case {
                        "empty-deadline" => "",
                        "zero" => "0",
                        "fraction" => "1.5",
                        _ => "604801",
                    },
                );
                "turnDeadlineSeconds (AGENT_TURN_DEADLINE_SECONDS): expected whole number from 1 to 604800".into()
            }
            "effort" => {
                command.env("AGENT_EFFORT", "HIGH");
                "effort (AGENT_EFFORT): expected lowercase ASCII letters".into()
            }
            "account" => {
                command.env("AGENT_ACCOUNT_SERVICES", "yes");
                "AGENT_ACCOUNT_SERVICES: expected true or false".into()
            }
            "v1" => {
                command
                    .env_remove("AGENT_TURN_DEADLINE_SECONDS")
                    .env("AGENT_PEER_MESSAGING", "yes");
                "AGENT_PEER_MESSAGING: expected true or false".into()
            }
            _ => String::new(),
        };
        let mut child = command.spawn().unwrap();
        let service =
            ().serve((child.stdout.take().unwrap(), child.stdin.take().unwrap()))
                .await
                .unwrap();
        let tools = service.list_all_tools().await.unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "turn");
        for _ in 0..2 {
            let call =
                serde_json::from_value(json!({"name":"turn","arguments":{"message":"TEST"}}))
                    .unwrap();
            let reply = service.call_tool(call).await.unwrap();
            assert_eq!(reply.is_error, Some(!expected.is_empty()));
            let reply = serde_json::to_value(reply).unwrap();
            if !expected.is_empty() {
                assert_eq!(
                    reply["content"][0]["text"],
                    format!("invalid configuration: {expected}")
                );
                assert!(!c.cwd.join("client-started").exists());
            } else {
                let evidence: Value =
                    serde_json::from_str(reply["structuredContent"]["text"].as_str().unwrap())
                        .unwrap();
                let has_directory = matches!(case, "directory" | "symlink");
                let expected_cwd = if has_directory {
                    parent.path().join("project").canonicalize().unwrap()
                } else {
                    c.cwd.clone()
                };
                assert_eq!(evidence["cwd"], json!(expected_cwd));
                if CLAUDE {
                    assert_eq!(
                        evidence["settings"]["permissions"]["defaultMode"],
                        if has_directory {
                            "bypassPermissions"
                        } else {
                            "dontAsk"
                        }
                    );
                } else {
                    let args = evidence["args"].as_array().unwrap();
                    let cd = args.iter().position(|a| a == "--cd").unwrap();
                    assert_eq!(args[cd + 1], json!(expected_cwd));
                    assert_eq!(
                        args[2],
                        if has_directory {
                            "danger-full-access"
                        } else {
                            "read-only"
                        }
                    );
                }
                assert!(
                    !evidence["args"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|s| s == "--model" || s == "--effort")
                );
                if CLAUDE {
                    assert_eq!(evidence["environment"]["MCP_TOOL_TIMEOUT"], "604800000");
                    assert_eq!(
                        evidence["mcp"]["mcpServers"]["engine"]["timeout"],
                        604800000
                    );
                }
            }
        }
        service.cancel().await.unwrap();
        let output = child.wait_with_output().await.unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
    }
}
