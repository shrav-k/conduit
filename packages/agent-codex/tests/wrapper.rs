use agent_adapter::{
    ACCOUNT_REPORTED, AccountReport, CLAUDE, CREDENTIAL_STORE, Config, FailedTurn, HOME_NAME,
    HOME_VARIABLE, RETIRED, RETIRED_REFUSAL, SIGNED_OUT, Session, account_arguments, arguments,
    header, helper_command, logout_arguments, prepare,
};
use agent_adapter::{
    BROWSER_LOGIN, DEVICE_LOGIN, DEVICE_REFUSAL, LOGIN_ENDED, LOGIN_LIMIT, LOGIN_MESSAGE,
    LOGIN_UNKNOWN, LOGIN_URL_DEADLINE, LOGIN_WAITING, REFRESH_BLOCKED, REFRESH_OVERRIDE_VARIABLE,
    RETURN_REFUSAL, REVOKE_OVERRIDE_VARIABLE, STATE_UNKNOWN, SignIn, SignInCall, StatedLogin,
    parse_stated_logins, redirect_url_matches, stated_logins,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    os::unix::{
        ffi::OsStrExt,
        fs::{DirBuilderExt, PermissionsExt, symlink},
        process::ExitStatusExt,
    },
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_agent-codex")
}
fn config(parent: &std::path::Path, account_services: bool, peer_messaging: bool) -> Config {
    let state = parent.join("state");
    fs::DirBuilder::new().mode(0o700).create(&state).unwrap();
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
        state_dir: Some(state.canonicalize().unwrap()),
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
async fn stdio_exposes_turn_and_new_conversation_and_setup_never_launches_client() {
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
    let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
    assert_eq!(names, ["turn", "new-conversation"]);
    assert_eq!(
        serde_json::to_value(tools.last().unwrap()).unwrap()["inputSchema"],
        json!({"type":"object","properties":{},"additionalProperties":false})
    );
    assert!(!c.cwd.join("client-started").exists());
    let invalid =
        serde_json::from_value(json!({"name":"new-conversation","arguments":{"reset":true}}))
            .unwrap();
    assert!(service.call_tool(invalid).await.is_err());
    let call = serde_json::from_value(json!({"name":"new-conversation","arguments":{}})).unwrap();
    let reply = service.call_tool(call).await.unwrap();
    assert_eq!(reply.is_error, Some(false));
    assert_eq!(
        reply.structured_content.unwrap(),
        json!({"conversation":2,"ended":{"turns":0,"failed":false,"survivors":[],"stopConfirmed":true}})
    );
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

fn planted_ps(pids: &[i32], columns: &str) -> std::io::Result<String> {
    let list: Vec<_> = pids.iter().map(i32::to_string).collect();
    let output = Command::new("ps")
        .args(["-o", columns, "-p", &list.join(",")])
        .output()?;
    if !matches!(output.status.code(), Some(0 | 1)) || !output.stderr.is_empty() {
        return Err(std::io::Error::other(
            "ps failed to read the requested fields",
        ));
    }
    String::from_utf8(output.stdout).map_err(std::io::Error::other)
}

struct PlantedTree {
    pids: [i32; 4],
    starts: Vec<(i32, String)>,
}

impl PlantedTree {
    async fn read(path: &Path, case: &str) -> Self {
        let tree: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        let pids = ["client", "background", "session", "doubleFork"].map(|key| {
            let pid = i32::try_from(tree[key].as_i64().unwrap()).unwrap();
            assert!(pid > 1);
            pid
        });
        let [client, background, session, double_fork] = pids;
        eprintln!(
            "{case} tree.json {{\"client\":{client},\"background\":{background},\"session\":{session},\"doubleFork\":{double_fork}}}"
        );
        let mut planted = Self {
            pids,
            starts: Vec::new(),
        };
        let mut missing = Vec::new();
        for pid in pids {
            if let Ok(start) = planted_ps(&[pid], "lstart=")
                && !start.trim().is_empty()
            {
                planted.starts.push((pid, start));
            } else {
                missing.push(pid);
            }
        }
        if !missing.is_empty() {
            let dir = path.parent().unwrap();
            fs::write(dir.join("tree-cleanup"), b"stop").unwrap();
            tokio::time::timeout(Duration::from_secs(1), wait_for(&dir.join("tree-cleaned")))
                .await
                .expect("scripted client did not acknowledge tree cleanup");
            panic!("ps could not register planted PIDs {missing:?}");
        }
        planted
    }

    fn live(&self) -> BTreeMap<i32, (i32, i32)> {
        planted_ps(&self.pids, "pid=,ppid=,pgid=,stat=")
            .unwrap()
            .lines()
            .filter_map(|line| {
                let mut fields = line.split_whitespace();
                let pid = fields.next().unwrap().parse().unwrap();
                let parent = fields.next().unwrap().parse().unwrap();
                let group = fields.next().unwrap().parse().unwrap();
                let state = fields.next().unwrap();
                (!state.starts_with('Z')).then_some((pid, (parent, group)))
            })
            .collect()
    }

    fn assert_shape(&self, v2: bool) {
        let live = self.live();
        assert_eq!(live.len(), self.pids.len());
        let [client, background, session, double_fork] = self.pids;
        let own_group = unsafe { libc::getpgrp() };
        if v2 {
            assert_eq!(live[&client].1, client);
            assert_ne!(live[&client].1, own_group);
            assert_eq!(live[&background], (client, client));
            assert_eq!(live[&session], (client, session));
            assert_eq!(live[&double_fork].1, client);
            assert_ne!(live[&double_fork].0, client);
        } else {
            assert_eq!(live[&client].1, own_group);
        }
    }

    fn check_report(live: BTreeMap<i32, (i32, i32)>, text: &str, prefix: &str, separator: &str) {
        use agent_adapter::survivor_note;

        assert!(text.starts_with(prefix), "unexpected stop prefix");
        let pids: Vec<_> = live.into_keys().collect();
        let suffix = text.strip_prefix(prefix).unwrap();
        let expected = survivor_note(&pids)
            .map(|note| format!("{separator}{note}"))
            .unwrap_or_default();
        assert!(
            suffix == expected,
            "each live planted PID must be ended or named in the stop report. Live PIDs: {pids:?}"
        );
    }
}

impl Drop for PlantedTree {
    fn drop(&mut self) {
        for (pid, start) in &self.starts {
            if planted_ps(&[*pid], "lstart=").is_ok_and(|current| current == *start) {
                unsafe { libc::kill(*pid, libc::SIGKILL) };
            }
        }
    }
}

#[tokio::test]
async fn v2_client_tree_stop_leaves_no_survivor() {
    use agent_adapter::{KILL_WAIT, STOP_GRACE, WRAPPER, survivor_note};
    use rmcp::{ServiceExt, model::ClientRequest, service::PeerRequestOptions};
    use tokio::{
        io::AsyncReadExt,
        time::{Instant, timeout, timeout_at},
    };

    assert_eq!(survivor_note(&[]), None);
    assert_eq!(
        survivor_note(&[43, 42]).as_deref(),
        Some("client tree: PIDs 42, 43 outlived the stop")
    );
    // Check the observer before planting processes that need it for cleanup.
    let self_pid = i32::try_from(std::process::id()).unwrap();
    let own_start = planted_ps(&[self_pid], "lstart=").unwrap_or_else(|error| {
        panic!("ps is required to observe the scripted client tree: {error}")
    });
    assert!(!own_start.trim().is_empty());
    let start = Instant::now();
    for (case, v2, seconds) in [
        ("deadline", true, 2),
        ("cancellation", true, 60),
        ("v1", false, 2),
    ] {
        let parent = tempfile::tempdir().unwrap();
        let mut c = config(parent.path(), false, false);
        c.v2 = v2;
        c.timeout = Duration::from_secs(seconds);
        prepare(&c).unwrap();
        let path = c.cwd.join("tree.json");
        let bound = Instant::now() + c.timeout + STOP_GRACE + KILL_WAIT + Duration::from_secs(3);
        let mut session = Session::new(c);
        let (tree, error) = {
            let mut turn = std::pin::pin!(session.turn("TREE"));
            tokio::select! {
                _ = &mut turn => panic!("turn ended before planting the tree"),
                _ = wait_for(&path) => {}
            }
            let tree = PlantedTree::read(&path, case).await;
            tree.assert_shape(v2);
            let error = if case == "cancellation" {
                None
            } else {
                Some(timeout_at(bound, turn).await.unwrap().unwrap_err())
            };
            (tree, error)
        };
        let error = match error {
            Some(error) => error,
            None => timeout(Duration::from_secs(6), session.cancel())
                .await
                .unwrap(),
        };
        let cause = match case {
            "deadline" => "deadline: no turn boundary within 2 seconds (input turnDeadlineSeconds)",
            "cancellation" => "cancelled",
            _ => "deadline: no turn boundary within 2 seconds",
        };
        let expected = format!("client turn 1 failed: {cause}; session cannot restart implicitly");
        let text = error.to_string();
        eprintln!("{case} result {text}");
        assert_eq!(text, expected);
        let failure = error.downcast_ref::<FailedTurn>().unwrap();
        if v2 {
            let stop = &failure.stop;
            let mut report = expected.clone();
            if let Some(note) = stop.note() {
                report.push_str("; ");
                report.push_str(&note);
            }
            let live_map = tree.live();
            let live: Vec<_> = live_map.keys().copied().collect();
            PlantedTree::check_report(live_map, &report, &expected, "; ");
            let fields = failure.fields().unwrap();
            assert_eq!(fields["failedTurn"]["survivors"], json!(live));
            assert_eq!(fields["failedTurn"]["stopConfirmed"], true);
            eprintln!("{case} planted PIDs {:?}, fields {fields}", tree.pids);
        } else {
            assert_eq!(failure.fields(), None);
        }
        if case == "deadline" {
            assert!(Instant::now() <= bound);
            let refusal = session.turn("TEST").await.unwrap_err().to_string();
            assert_eq!(
                refusal,
                format!("session failed at turn 1: {cause}; implicit restart refused")
            );
            eprintln!("deadline retry {refusal}");
        }
    }

    let parent = tempfile::tempdir().unwrap();
    let mut c = config(parent.path(), false, false);
    c.v2 = true;
    prepare(&c).unwrap();
    let mut wrapper = tokio::process::Command::new(binary())
        .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
        .env_clear()
        .envs(&c.env)
        .env("AGENT_ACCOUNT_SERVICES", "false")
        .env("AGENT_TURN_DEADLINE_SECONDS", "60")
        .env("CONDUIT_STATE_DIR", c.state_dir.as_ref().unwrap())
        .current_dir(&c.cwd)
        .process_group(0)
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let service = ()
        .serve((
            wrapper.stdout.take().unwrap(),
            wrapper.stdin.take().unwrap(),
        ))
        .await
        .unwrap();
    let request: ClientRequest = serde_json::from_value(
        json!({"method":"tools/call","params":{"name":"turn","arguments":{"message":"TREE"}}}),
    )
    .unwrap();
    let _active = service
        .send_request_with_option(request, PeerRequestOptions::no_options())
        .await
        .unwrap();
    let path = c.cwd.join("tree.json");
    wait_for(&path).await;
    let tree = PlantedTree::read(&path, "TERM").await;
    tree.assert_shape(true);
    let pid = i32::try_from(wrapper.id().unwrap()).unwrap();
    assert_eq!(unsafe { libc::kill(-pid, libc::SIGTERM) }, 0);
    let mut stderr = String::new();
    let (status, live) = timeout(Duration::from_secs(10), async {
        let status = wrapper.wait().await.unwrap();
        let live = tree.live();
        wrapper
            .stderr
            .take()
            .unwrap()
            .read_to_string(&mut stderr)
            .await
            .unwrap();
        (status, live)
    })
    .await
    .unwrap();
    assert_eq!(status.code(), Some(143));
    assert!(
        ![
            "synthetic-test-token",
            "synthetic-anthropic-key",
            "synthetic-openai-key",
        ]
        .iter()
        .any(|value| stderr.contains(value)),
        "wrapper stderr contains a fixture secret"
    );
    if live.is_empty() {
        assert!(
            stderr.is_empty(),
            "wrapper emitted an unexpected stderr line"
        );
        eprintln!("TERM exited with status 143, stderr empty");
    } else {
        assert_eq!(stderr.lines().count(), 1);
        let line = stderr.lines().next().unwrap();
        PlantedTree::check_report(live, line, WRAPPER, ": ");
        eprintln!("TERM exited with status 143, stderr {line}");
    }
    eprintln!(
        "v2_client_tree_stop_leaves_no_survivor took {:.2?}",
        start.elapsed()
    );
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
        a.extend(["-c".into(), "cli_auth_credentials_store=\"file\"".into()]);
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
                    // Ignore the .v2 working directory, credential store and MCP timeout.
                    for flag in [
                        "--cd",
                        CREDENTIAL_STORE,
                        "mcp_servers.engine.tool_timeout_sec=10",
                    ] {
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
    let error = session.turn(message).await.unwrap_err();
    let first = error.to_string();
    assert_eq!(
        first,
        format!("client turn {k} failed: {cause}; session cannot restart implicitly")
    );
    let failure = error.downcast_ref::<FailedTurn>().unwrap();
    assert_eq!(failure.turn, k);
    assert_eq!(failure.cause, cause);
    if c.v2 {
        let class = cause.strip_prefix("client failure: ");
        let kind = match cause {
            cause if cause.starts_with("deadline: ") => "deadline",
            "cancelled" => "cancelled",
            _ if class.is_some() => "client-failure",
            cause if cause.starts_with("client exit: ") => "client-exit",
            "EOF before turn boundary" => "eof",
            "no thread" => "no-thread",
            "invalid client output" => "invalid-client-output",
            "resume changed thread" => "resume-changed-thread",
            "client did not start" => "client-did-not-start",
            cause if cause.starts_with("client home (") => "client-home",
            _ => "unclassified",
        };
        let fields = failure.fields().unwrap();
        assert!(!fields.to_string().contains("sentinel-secret"));
        assert_eq!(
            fields,
            json!({"failedTurn": {
                "turn": k,
                "cause": kind,
                "class": class,
                "clientStarted": cause != "client did not start",
                "survivors": [],
                "stopConfirmed": true,
            }})
        );
    } else {
        assert_eq!(failure.fields(), None);
    }
    let marker = c.cwd.join("client-started");
    if marker.exists() {
        fs::remove_file(&marker).unwrap();
    }
    let refusal = session.turn("TEST").await.unwrap_err();
    assert!(refusal.downcast_ref::<FailedTurn>().is_none());
    let next = refusal.to_string();
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
            (
                "FAIL:stream disconnected before completion",
                "client failure: network",
                false,
            ),
            ("FAIL:Connection error.", "client failure: network", false),
            ("FAIL:Request timed out", "client failure: network", false),
            (
                "FAIL:error sending request for url",
                "client failure: network",
                false,
            ),
            (
                "FAIL:401 connection reset",
                "client failure: authentication",
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
    use rmcp::{ClientLifecycleMode, ClientServiceExt, ServiceExt, model::ProtocolVersion};
    for case in [
        "valid",
        "modern",
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
        "state-link",
        "state-unset",
        "state-relative",
        "state-empty",
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
            .env("CONDUIT_STATE_DIR", c.state_dir.as_ref().unwrap())
            .env(HOME_VARIABLE, parent.path().join("elsewhere"))
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
            "state-link" => {
                let link = parent.path().join("state-link");
                symlink(c.state_dir.as_ref().unwrap(), &link).unwrap();
                command.env("CONDUIT_STATE_DIR", link);
                String::new()
            }
            "state-unset" | "state-empty" => {
                if case == "state-unset" {
                    command.env_remove("CONDUIT_STATE_DIR");
                } else {
                    command.env("CONDUIT_STATE_DIR", "");
                }
                format!("client home ({HOME_VARIABLE}): CONDUIT_STATE_DIR is unset")
            }
            "state-relative" => {
                command.env("CONDUIT_STATE_DIR", "state");
                format!("client home ({HOME_VARIABLE}): CONDUIT_STATE_DIR is not an absolute path")
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
        let transport = (child.stdout.take().unwrap(), child.stdin.take().unwrap());
        let modern = case == "modern";
        let service = if modern {
            let lifecycle = ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            };
            ().serve_with_lifecycle(transport, lifecycle).await.unwrap()
        } else {
            ().serve(transport).await.unwrap()
        };
        let tools = service.list_all_tools().await.unwrap();
        let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
        let expected_names: &[_] = if case == "v1" {
            &["turn", "new-conversation"]
        } else {
            &[
                "turn",
                "retire",
                "account",
                "sign-out",
                "sign-in",
                "new-conversation",
            ]
        };
        assert_eq!(names, expected_names);
        for _ in 0..2 {
            let call =
                serde_json::from_value(json!({"name":"turn","arguments":{"message":"TEST"}}))
                    .unwrap();
            let reply = service.call_tool(call).await.unwrap();
            assert_eq!(reply.is_error, Some(!expected.is_empty()));
            let reply = serde_json::to_value(reply).unwrap();
            if modern {
                assert_eq!(reply["resultType"], "complete");
            } else {
                assert!(reply.get("resultType").is_none());
            }
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
                let home = parent.path().join("state/client-home");
                assert_eq!(
                    evidence["environment"][HOME_VARIABLE],
                    json!(home.canonicalize().unwrap())
                );
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
        if !expected.is_empty() {
            let call =
                serde_json::from_value(json!({"name":"new-conversation","arguments":{}})).unwrap();
            let reply = service.call_tool(call).await.unwrap();
            assert_eq!(reply.is_error, Some(true));
            assert_eq!(
                serde_json::to_value(reply).unwrap()["content"],
                json!([{"type":"text","text":format!("invalid configuration: {expected}")}])
            );
        }
        service.cancel().await.unwrap();
        let output = child.wait_with_output().await.unwrap();
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
    }
}

#[tokio::test]
async fn new_conversation_shares_no_context_without_restart() {
    use rmcp::{ServiceExt, model::CallToolResult};

    for version in ["v1", "v2"] {
        let parent = tempfile::tempdir().unwrap();
        let c = config(parent.path(), false, false);
        let mut command = tokio::process::Command::new(binary());
        command
            .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
            .env_clear()
            .envs(&c.env)
            .env("AGENT_ACCOUNT_SERVICES", "false")
            .current_dir(&c.cwd)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if version == "v2" {
            command
                .env("AGENT_TURN_DEADLINE_SECONDS", "60")
                .env("CONDUIT_STATE_DIR", c.state_dir.as_ref().unwrap());
        } else {
            command.env("AGENT_PEER_MESSAGING", "false");
        }
        let mut child = command.spawn().unwrap();
        let pid = child.id().unwrap();
        let transport = (child.stdout.take().unwrap(), child.stdin.take().unwrap());
        let service = ().serve(transport).await.unwrap();
        let mut call = async |name: &str, arguments: Value| {
            let request =
                serde_json::from_value(json!({"name":name,"arguments":arguments})).unwrap();
            let reply = service.call_tool(request).await;
            assert_eq!(child.id(), Some(pid));
            assert!(child.try_wait().unwrap().is_none());
            reply
        };
        let turn = |reply: CallToolResult| {
            assert_eq!(reply.is_error, Some(false));
            let structured = reply.structured_content.unwrap();
            let evidence: Value =
                serde_json::from_str(structured["text"].as_str().unwrap()).unwrap();
            eprintln!("{version} turn number {}", evidence["number"]);
            (structured, evidence)
        };
        let reset = |reply: CallToolResult, number, failed| {
            let reply = serde_json::to_value(reply).unwrap();
            eprintln!("{version} new-conversation: {reply}");
            let expected = json!({
                "conversation":number,
                "ended":{"turns":2,"failed":failed,"survivors":[],"stopConfirmed":true}
            });
            assert_eq!(reply["isError"], false);
            assert_eq!(reply["structuredContent"], expected);
            let content = reply["content"].as_array().unwrap();
            assert_eq!(content.len(), 1);
            assert_eq!(content[0]["type"], "text");
            assert_eq!(
                serde_json::from_str::<Value>(content[0]["text"].as_str().unwrap()).unwrap(),
                expected
            );
        };
        let has_arg = |evidence: &Value, arg: &str| {
            evidence["args"]
                .as_array()
                .unwrap()
                .iter()
                .any(|a| a == arg)
        };

        let message = || json!({"message":"TEST"});
        let (first, first_evidence) = turn(call("turn", message()).await.unwrap());
        assert_eq!(first_evidence["number"], 1);
        assert!(!has_arg(&first_evidence, "resume"));
        assert_eq!(first["clientTools"]["status"], "reported");

        let (second, second_evidence) = turn(call("turn", message()).await.unwrap());
        assert_eq!(second_evidence["number"], 2);
        assert!(has_arg(&second_evidence, "resume"));
        assert!(has_arg(&second_evidence, "fixture-exact-thread"));
        assert_eq!(second["clientTools"]["status"], "unavailable");

        reset(call("new-conversation", json!({})).await.unwrap(), 2, false);

        let (fresh, fresh_evidence) = turn(call("turn", message()).await.unwrap());
        assert!(!has_arg(&fresh_evidence, "resume"));
        assert!(!has_arg(&fresh_evidence, "fixture-exact-thread"));
        assert_eq!(fresh_evidence["number"], 1);
        assert_eq!(fresh["clientTools"]["status"], "reported");
        assert_eq!(
            fresh["clientTools"]["observations"][0]["freshness"],
            "current"
        );

        let failure = call(
            "turn",
            json!({"message":"FAIL:stream disconnected before completion"}),
        )
        .await
        .unwrap();
        assert_eq!(failure.is_error, Some(true));
        assert_eq!(
            serde_json::to_value(failure).unwrap()["content"][0]["text"],
            "client turn 2 failed: client failure: network; session cannot restart implicitly"
        );
        let refused = call("turn", message()).await.unwrap();
        assert_eq!(refused.is_error, Some(true));
        assert_eq!(
            serde_json::to_value(refused).unwrap()["content"],
            json!([{"type":"text","text":"session failed at turn 2: client failure: network; implicit restart refused"}])
        );
        reset(call("new-conversation", json!({})).await.unwrap(), 3, true);
        let (_, fresh_evidence) = turn(call("turn", message()).await.unwrap());
        assert_eq!(fresh_evidence["number"], 1);
        assert!(!has_arg(&fresh_evidence, "resume"));

        let invalid = call("new-conversation", json!({"reset":true}))
            .await
            .unwrap_err();
        eprintln!("{version} invalid new-conversation: {invalid}");
        assert!(invalid.to_string().contains("expected no arguments"));
        let (_, evidence) = turn(call("turn", message()).await.unwrap());
        assert_eq!(evidence["number"], 2);

        service.cancel().await.unwrap();
        let output = child.wait_with_output().await.unwrap();
        assert!(output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        for sentinel in [
            "sentinel-secret",
            "synthetic-test-token",
            "synthetic-anthropic-key",
            "synthetic-openai-key",
        ] {
            assert!(!stderr.contains(sentinel));
        }
    }
}

#[test]
fn v2_new_conversation_reports_survivors() {
    let stop = agent_adapter::StopReport {
        survivors: vec![42, 43],
        unconfirmed: true,
    };
    assert_eq!(
        stop.ended_conversation(2, true),
        json!({"turns":2,"failed":true,"survivors":[42,43],"stopConfirmed":false})
    );
    let survivors = agent_adapter::StopReport {
        survivors: vec![42],
        unconfirmed: false,
    };
    assert_eq!(
        survivors.ended_conversation(1, false),
        json!({"turns":1,"failed":false,"survivors":[42],"stopConfirmed":false})
    );
    let unconfirmed = agent_adapter::StopReport {
        survivors: vec![],
        unconfirmed: true,
    };
    assert_eq!(
        unconfirmed.ended_conversation(0, false),
        json!({"turns":0,"failed":false,"survivors":[],"stopConfirmed":false})
    );
}

#[tokio::test]
async fn v2_failed_turn_fields_reach_the_result() {
    use rmcp::{
        ClientHandler, ClientLifecycleMode, ClientServiceExt, ErrorData, RoleClient, model::*,
        service::RequestContext,
    };
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    struct TurnCaller(Arc<AtomicBool>);
    impl ClientHandler for TurnCaller {
        fn get_info(&self) -> ClientConfig {
            let mut config = ClientConfig::default();
            config.capabilities.elicitation = Some(
                ElicitationCapability::new()
                    .with_form(FormElicitationCapability::new())
                    .with_url(UrlElicitationCapability::new()),
            );
            config
        }

        async fn create_elicitation(
            &self,
            _: ElicitRequestParams,
            _: RequestContext<RoleClient>,
        ) -> Result<ElicitResult, ErrorData> {
            self.0.store(true, Ordering::Relaxed);
            Ok(ElicitResult::new(ElicitationAction::Decline))
        }
    }

    let assert_no_client_text = |text: &str| {
        for sentinel in [
            "sentinel-secret",
            "synthetic-test-token",
            "synthetic-anthropic-key",
            "synthetic-openai-key",
        ] {
            assert!(!text.contains(sentinel));
        }
    };
    for (version, message, class, modern_mcp) in [
        (
            "v2",
            "FAIL:stream disconnected before completion",
            "network",
            true,
        ),
        ("v2", "FAIL:401 unauthorized", "authentication", true),
        (
            "v1",
            "FAIL:stream disconnected before completion",
            "network",
            true,
        ),
        (
            "v2",
            "FAIL:stream disconnected before completion",
            "network",
            false,
        ),
    ] {
        let parent = tempfile::tempdir().unwrap();
        let c = config(parent.path(), false, false);
        let mut command = tokio::process::Command::new(binary());
        command
            .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
            .env_clear()
            .envs(&c.env)
            .env("AGENT_ACCOUNT_SERVICES", "false")
            .current_dir(&c.cwd)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let v2 = version == "v2";
        if v2 {
            command
                .env("AGENT_TURN_DEADLINE_SECONDS", "60")
                .env("CONDUIT_STATE_DIR", c.state_dir.as_ref().unwrap());
        } else {
            command.env("AGENT_PEER_MESSAGING", "false");
        }
        let mut child = command.spawn().unwrap();
        let elicited = Arc::new(AtomicBool::new(false));
        let lifecycle = if modern_mcp {
            ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            }
        } else {
            ClientLifecycleMode::Initialize
        };
        let service = TurnCaller(elicited.clone())
            .serve_with_lifecycle(
                (child.stdout.take().unwrap(), child.stdin.take().unwrap()),
                lifecycle,
            )
            .await
            .unwrap();
        let messages = if v2 {
            &[message, "TEST"][..]
        } else {
            &[message]
        };
        for (index, message) in messages.iter().enumerate() {
            let call =
                serde_json::from_value(json!({"name":"turn","arguments":{"message":message}}))
                    .unwrap();
            let reply = serde_json::to_value(service.call_tool(call).await.unwrap()).unwrap();
            let reply_text = reply.to_string();
            assert_no_client_text(&reply_text);
            assert!(!reply_text.contains("elicitation"));
            assert_eq!(reply["isError"], true);
            assert!(reply.get("structuredContent").is_none());
            assert!(reply.get("_meta").is_none());
            if modern_mcp {
                assert_eq!(reply["resultType"], "complete");
            } else {
                assert!(reply.get("resultType").is_none());
            }
            let content = reply["content"].as_array().unwrap();
            let first = index == 0;
            let with_fields = first && v2;
            assert_eq!(
                content.len(),
                if with_fields { 2 } else { 1 },
                "{version} {class} result block count"
            );
            let expected = if first {
                format!(
                    "client turn 1 failed: client failure: {class}; session cannot restart implicitly"
                )
            } else {
                format!(
                    "session failed at turn 1: client failure: {class}; implicit restart refused"
                )
            };
            assert_eq!(content[0], json!({"type":"text","text":expected}));
            if with_fields {
                assert_eq!(content[1]["type"], "text");
                let fields: Value =
                    serde_json::from_str(content[1]["text"].as_str().unwrap()).unwrap();
                assert_eq!(
                    fields,
                    json!({"failedTurn": {
                        "turn": 1,
                        "cause": "client-failure",
                        "class": class,
                        "clientStarted": true,
                        "survivors": [],
                        "stopConfirmed": true,
                    }})
                );
            }
            for (block, item) in content.iter().enumerate() {
                eprintln!(
                    "{version} {class} (modern MCP: {modern_mcp}) call {} block {}: {}",
                    index + 1,
                    block + 1,
                    item["text"].as_str().unwrap()
                );
            }
        }
        service.cancel().await.unwrap();
        let output = child.wait_with_output().await.unwrap();
        assert!(output.status.success());
        assert_no_client_text(&String::from_utf8_lossy(&output.stderr));
        assert!(!elicited.load(Ordering::Relaxed));
    }
}

const DEFAULT_MARKER: &str = "synthetic-marker";
const DEFAULT_DOTENV: &str = "FIXTURE_DOTENV=synthetic";
const AGENT_DOTENV: &str = "OPENAI_API_KEY=synthetic-dotenv-value";
const DOTENV_REFUSAL: &str = "the home holds a .env, which Codex would read into its environment; the wrapper refuses it, since the Agent holds no API key (ruling 83)";
const LOGIN_TOKEN: &str = "synthetic-login-token-canary";
const LOGIN_CLAIM: &str = "synthetic-claim-canary";

fn plant_login(home: &Path) {
    let login = json!({"token": LOGIN_TOKEN, "claims": {"account": LOGIN_CLAIM}});
    fs::write(home.join("fixture-login"), login.to_string()).unwrap();
}

fn assert_no_login_canary(text: &str) {
    assert!(!text.contains(LOGIN_TOKEN));
    assert!(!text.contains(LOGIN_CLAIM));
}

fn recorded_logouts(work: &Path) -> Vec<Value> {
    let log = match fs::read_to_string(work.join("logouts.jsonl")) {
        Ok(log) => log,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
        Err(error) => panic!("cannot read scripted logout log: {error}"),
    };
    assert_no_login_canary(&log);
    log.lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn plant_default_home(parent: &Path) -> std::path::PathBuf {
    let home = parent.join(if CLAUDE { ".claude" } else { ".codex" });
    fs::create_dir(&home).unwrap();
    fs::write(home.join("marker"), DEFAULT_MARKER).unwrap();
    if !CLAUDE {
        fs::write(home.join(".env"), DEFAULT_DOTENV).unwrap();
    }
    home
}

fn entry_names(dir: &Path) -> Vec<String> {
    let mut names: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

fn assert_default_home(home: &Path) {
    assert_eq!(
        fs::read(home.join("marker")).unwrap(),
        DEFAULT_MARKER.as_bytes()
    );
    if CLAUDE {
        assert_eq!(entry_names(home), ["marker"]);
    } else {
        assert_eq!(entry_names(home), [".env", "marker"]);
        assert_eq!(
            fs::read(home.join(".env")).unwrap(),
            DEFAULT_DOTENV.as_bytes()
        );
    }
}

fn assert_client_environment(evidence: &Value, expected: &BTreeMap<String, String>) {
    let mut actual = evidence["environment"].as_object().unwrap().clone();
    actual.remove("__CF_USER_TEXT_ENCODING");
    assert_eq!(Value::Object(actual), json!(expected));
}

async fn assert_home_refusal(
    session: &mut Session,
    turn: usize,
    cause: &str,
    c: &Config,
    default_home: &Path,
) {
    let error = session.turn("HOME").await.unwrap_err().to_string();
    assert_eq!(
        error,
        format!(
            "client turn {turn} failed: client home ({HOME_VARIABLE}): {cause}; session cannot restart implicitly"
        )
    );
    for absent in [&c.env["HOME"], "OPENAI_API_KEY", "synthetic-dotenv-value"] {
        assert!(!error.contains(absent));
    }
    let work = c.directory.as_ref().unwrap_or(&c.cwd);
    assert!(!work.join("client-started").exists());
    assert_default_home(default_home);
}

#[tokio::test]
async fn v2_client_home_in_state_directory_leaves_default_home_untouched() {
    for directory in [false, true] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        if directory {
            let project = parent.path().join("project");
            fs::create_dir(&project).unwrap();
            c.directory = Some(project.canonicalize().unwrap());
        }
        if CLAUDE {
            c.env.insert("MCP_TOOL_TIMEOUT".into(), "10000".into());
        }
        prepare(&c).unwrap();
        let home = parent.path().join("state/client-home");
        let mut session = Session::new(c.clone());
        for _ in 0..2 {
            let reply = session.turn("HOME").await.unwrap();
            let evidence: Value = serde_json::from_str(reply["text"].as_str().unwrap()).unwrap();
            let canonical_home = home.canonicalize().unwrap();
            assert_eq!(
                evidence["environment"][HOME_VARIABLE],
                json!(canonical_home)
            );
            let mut expected = c.env.clone();
            expected.remove("ANTHROPIC_API_KEY");
            expected.remove("OPENAI_API_KEY");
            expected.insert(
                HOME_VARIABLE.into(),
                canonical_home.to_str().unwrap().into(),
            );
            assert_client_environment(&evidence, &expected);
            assert_eq!(evidence["home"], json!(canonical_home));
            assert_eq!(evidence["dotenv"], json!([]));
            if !CLAUDE {
                assert!(
                    evidence["args"]
                        .as_array()
                        .unwrap()
                        .windows(2)
                        .any(|pair| pair[0] == "-c" && pair[1] == CREDENTIAL_STORE)
                );
            }
        }
        assert_default_home(&default_home);
        assert_eq!(fs::read(home.join("fixture-home-write")).unwrap(), b"2");
        if CLAUDE {
            assert_eq!(entry_names(&home), ["fixture-home-write"]);
        } else {
            assert!(home.join("tmp/arg0-fixture/lock").is_file());
            assert_eq!(entry_names(&home), ["fixture-home-write", "tmp"]);
        }
        #[cfg(unix)]
        assert_eq!(
            fs::metadata(&home).unwrap().permissions().mode() & 0o777,
            0o700
        );

        if !CLAUDE {
            let work = c.directory.as_ref().unwrap_or(&c.cwd);
            fs::remove_file(work.join("client-started")).unwrap();
            let dotenv = home.join(".env");
            fs::write(&dotenv, AGENT_DOTENV).unwrap();
            assert_home_refusal(&mut session, 3, DOTENV_REFUSAL, &c, &default_home).await;
            assert_eq!(fs::read(dotenv).unwrap(), AGENT_DOTENV.as_bytes());
            assert_eq!(entry_names(&home), [".env", "fixture-home-write", "tmp"]);
        }
        session.shutdown().await;
    }
}

#[tokio::test]
async fn v2_retire_signs_out_and_empties_only_its_home() {
    for named_directory in [false, true] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        if named_directory {
            let project = parent.path().join("project");
            fs::create_dir(&project).unwrap();
            c.directory = Some(project.canonicalize().unwrap());
        }
        if CLAUDE {
            c.env.insert("MCP_TOOL_TIMEOUT".into(), "10000".into());
        }
        prepare(&c).unwrap();
        let work = c.directory.as_ref().unwrap_or(&c.cwd);
        let mut session = Session::new(c.clone());
        let reply = session.turn("HOME").await.unwrap();
        assert_no_login_canary(&reply.to_string());
        let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        let canonical_home = home.canonicalize().unwrap();
        plant_login(&home);
        fs::create_dir_all(home.join("nested/deeper")).unwrap();
        fs::write(home.join("nested/deeper/file"), b"fixture").unwrap();
        symlink(&default_home, home.join("outside-link")).unwrap();

        let result = session.retire().await.unwrap();
        assert_no_login_canary(&result);
        assert_eq!(result, RETIRED);
        let logs = recorded_logouts(work);
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log["args"], json!(logout_arguments()));
        assert_eq!(log["cwd"], json!(work));
        assert_eq!(log["home"], json!(canonical_home));
        let mut expected_env = c.env.clone();
        expected_env.remove("ANTHROPIC_API_KEY");
        expected_env.remove("OPENAI_API_KEY");
        expected_env.insert(
            HOME_VARIABLE.into(),
            canonical_home.to_str().unwrap().into(),
        );
        assert_client_environment(log, &expected_env);
        let mut entries = vec![
            "fixture-home-write",
            "fixture-login",
            "nested",
            "outside-link",
        ];
        if !CLAUDE {
            entries.push("tmp");
        }
        assert_eq!(log["entries"], json!(entries));
        assert!(entry_names(&home).is_empty());
        assert_eq!(
            fs::metadata(&home).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_default_home(&default_home);

        let result = session.retire().await.unwrap();
        assert_no_login_canary(&result);
        assert_eq!(result, RETIRED);
        let logs = recorded_logouts(work);
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[1]["args"], json!(logout_arguments()));
        assert_eq!(logs[1]["entries"], json!([]));
        assert!(entry_names(&home).is_empty());
        fs::remove_file(work.join("client-started")).unwrap();
        let refusal = session.turn("TEST").await.unwrap_err().to_string();
        assert_no_login_canary(&refusal);
        assert_eq!(refusal, RETIRED_REFUSAL);
        assert!(!work.join("client-started").exists());
        assert_default_home(&default_home);
        for log in logs {
            eprintln!("logout args={}, entries={}", log["args"], log["entries"]);
        }
    }
}

#[tokio::test]
async fn v2_retire_error_paths_hold_no_canary() {
    for case in [
        "no-client",
        "exit",
        "deadline",
        "empty",
        "outside-link",
        "unset",
        "configuration",
        "configuration-unset",
        "dotenv",
    ] {
        if case == "empty" && unsafe { libc::geteuid() } == 0 {
            eprintln!("empty: skipped under UID 0");
            continue;
        }
        if case == "dotenv" && CLAUDE {
            continue;
        }
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        prepare(&c).unwrap();
        let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        let login_home = if case == "outside-link" {
            parent.path().join("elsewhere")
        } else {
            home.clone()
        };
        fs::DirBuilder::new()
            .mode(0o700)
            .create(&login_home)
            .unwrap();
        plant_login(&login_home);
        if case == "outside-link" {
            symlink(&login_home, &home).unwrap();
        }
        let locked = home.join("locked");
        let logout_exit = c.cwd.join("logout-exit");
        let logout_hang = c.cwd.join("logout-hang");
        let entries_with_login: &[_] = &["fixture-login"];
        let unset = format!("client home ({HOME_VARIABLE}): CONDUIT_STATE_DIR is unset");
        let (expected, count, entries): (String, usize, &[_]) = match case {
            "no-client" => {
                c.program = parent.path().join("missing-client");
                ("client logout did not start".into(), 0, entries_with_login)
            }
            "exit" => {
                fs::write(&logout_exit, b"3").unwrap();
                (
                    "client logout failed: client exit: status 3".into(),
                    1,
                    entries_with_login,
                )
            }
            "deadline" => {
                fs::write(&logout_hang, b"").unwrap();
                (
                    "client logout failed: no exit within 3 seconds".into(),
                    1,
                    entries_with_login,
                )
            }
            "empty" => {
                fs::create_dir(&locked).unwrap();
                fs::write(locked.join("file"), b"fixture").unwrap();
                fs::set_permissions(&locked, fs::Permissions::from_mode(0o500)).unwrap();
                (
                    format!("client home ({HOME_VARIABLE}): cannot empty the home"),
                    1,
                    &["locked"],
                )
            }
            "outside-link" => (
                format!(
                    "client home ({HOME_VARIABLE}): the home resolves outside CONDUIT_STATE_DIR"
                ),
                0,
                entries_with_login,
            ),
            "unset" => {
                c.state_dir = None;
                (unset, 0, entries_with_login)
            }
            "configuration" => {
                c.configuration_error = Some(
                    "invalid configuration: effort (AGENT_EFFORT): expected lowercase ASCII letters"
                        .into(),
                );
                (RETIRED.into(), 1, &[])
            }
            "configuration-unset" => {
                c.state_dir = None;
                let error = format!("invalid configuration: {unset}");
                c.configuration_error = Some(error.clone());
                (error, 0, entries_with_login)
            }
            "dotenv" => {
                fs::write(home.join(".env"), AGENT_DOTENV).unwrap();
                (
                    format!("client home ({HOME_VARIABLE}): {DOTENV_REFUSAL}"),
                    0,
                    &[".env", "fixture-login"],
                )
            }
            _ => unreachable!(),
        };
        let mut session = Session::new(c.clone());
        let result = if case == "deadline" {
            session.retire_within(Duration::from_secs(3)).await
        } else {
            session.retire().await
        };
        let answer = if case == "configuration" {
            result.unwrap()
        } else {
            result.unwrap_err().to_string()
        };
        assert_no_login_canary(&answer);
        assert_eq!(answer, expected);
        assert_eq!(recorded_logouts(&c.cwd).len(), count);
        assert_eq!(entry_names(&login_home), entries);
        let turn = session.turn("TEST").await.unwrap_err().to_string();
        assert_no_login_canary(&turn);
        assert_eq!(
            turn,
            c.configuration_error.as_deref().unwrap_or(RETIRED_REFUSAL)
        );
        assert_default_home(&default_home);
        assert!(!c.cwd.join("client-started").exists());
        eprintln!("{case}: retire={answer:?}, turn={turn:?}");

        match case {
            "exit" => fs::remove_file(logout_exit).unwrap(),
            "deadline" => fs::remove_file(logout_hang).unwrap(),
            "empty" => fs::set_permissions(locked, fs::Permissions::from_mode(0o700)).unwrap(),
            _ => continue,
        }
        let retry = session.retire().await.unwrap();
        assert_no_login_canary(&retry);
        assert_eq!(retry, RETIRED);
        assert_eq!(recorded_logouts(&c.cwd).len(), count + 1);
        assert!(entry_names(&home).is_empty());
        assert_default_home(&default_home);
        assert!(!c.cwd.join("client-started").exists());
        eprintln!("{case}: retry={retry:?}");
    }
}

#[tokio::test]
async fn v2_retire_tool_through_the_wrapper() {
    use rmcp::{
        ClientLifecycleMode, ClientServiceExt, ServiceExt,
        model::{CallToolRequestParams, ProtocolVersion},
    };
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let call = |name: &str, arguments: Value| -> CallToolRequestParams {
        serde_json::from_value(json!({"name": name, "arguments": arguments})).unwrap()
    };
    for (named, logout_fails) in [(false, false), (true, false), (true, true)] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let c = config(parent.path(), false, false);
        let state = c.state_dir.as_ref().unwrap();
        let home = state.join(HOME_NAME);
        let directory = parent.path().join("project");
        fs::create_dir(&directory).unwrap();
        fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
        plant_login(&home);
        let mut command = tokio::process::Command::new(binary());
        command
            .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
            .env_clear()
            .envs(&c.env)
            .env("AGENT_ACCOUNT_SERVICES", "false")
            .env("AGENT_TURN_DEADLINE_SECONDS", "60");
        if named {
            command.env("AGENT_DIRECTORY", &directory);
        }
        let mut child = command
            .env("CONDUIT_STATE_DIR", state)
            .env(HOME_VARIABLE, parent.path().join("elsewhere"))
            .current_dir(&c.cwd)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        // Keep the bytes that the MCP transport may skip while parsing.
        let mut stdout = child.stdout.take().unwrap();
        let (mcp_stdout, mut forward) = tokio::io::duplex(4096);
        let raw_stdout = tokio::spawn(async move {
            let mut bytes = Vec::new();
            let mut chunk = [0; 4096];
            loop {
                let n = stdout.read(&mut chunk).await.unwrap();
                if n == 0 {
                    break;
                }
                bytes.extend_from_slice(&chunk[..n]);
                if let Err(error) = forward.write_all(&chunk[..n]).await {
                    assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
                    stdout.read_to_end(&mut bytes).await.unwrap();
                    break;
                }
            }
            bytes
        });
        let transport = (mcp_stdout, child.stdin.take().unwrap());
        let lifecycle = ClientLifecycleMode::Discover {
            preferred_versions: vec![ProtocolVersion::V_2026_07_28],
        };
        let service = ().serve_with_lifecycle(transport, lifecycle).await.unwrap();
        let tools = service.list_all_tools().await.unwrap();
        let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
        assert_eq!(
            names,
            [
                "turn",
                "retire",
                "account",
                "sign-out",
                "sign-in",
                "new-conversation"
            ]
        );
        let invalid = service
            .call_tool(call("retire", json!({"extra": true})))
            .await
            .unwrap_err();
        assert_no_login_canary(&invalid.to_string());
        let turn = service
            .call_tool(call("turn", json!({"message": "TEST"})))
            .await
            .unwrap();
        let turn = serde_json::to_value(turn).unwrap();
        assert_no_login_canary(&turn.to_string());
        assert_eq!(turn["isError"], false);
        if named {
            assert!(directory.join("client-started").is_file());
            fs::remove_dir_all(&directory).unwrap();
        } else {
            assert!(c.cwd.join("client-started").is_file());
        }
        if logout_fails {
            fs::write(c.cwd.join("logout-exit"), b"3").unwrap();
        }

        let result = service.call_tool(call("retire", json!({}))).await.unwrap();
        let result = serde_json::to_value(result).unwrap();
        assert_no_login_canary(&result.to_string());
        let expected_text = if logout_fails {
            "client logout failed: client exit: status 3"
        } else {
            RETIRED
        };
        assert_eq!(
            result,
            json!({
                "isError": logout_fails, "resultType": "complete",
                "content": [{"type": "text", "text": expected_text}],
            })
        );
        let reset = service
            .call_tool(call("new-conversation", json!({})))
            .await
            .unwrap();
        assert_eq!(reset.is_error, Some(true));
        assert_eq!(
            serde_json::to_value(reset).unwrap()["content"],
            json!([{"type":"text","text":"session retired; no new conversation starts"}])
        );
        let turn = service
            .call_tool(call("turn", json!({"message": "TEST"})))
            .await
            .unwrap();
        let turn = serde_json::to_value(turn).unwrap();
        assert_no_login_canary(&turn.to_string());
        assert_eq!(turn["isError"], true);
        assert_eq!(turn["resultType"], "complete");
        assert_eq!(
            turn["content"],
            json!([{"type": "text", "text": RETIRED_REFUSAL}])
        );
        let logs = recorded_logouts(&c.cwd);
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0]["args"], json!(logout_arguments()));
        assert_eq!(logs[0]["cwd"], json!(c.cwd));
        assert_eq!(
            logs[0]["environment"][HOME_VARIABLE],
            json!(home.canonicalize().unwrap())
        );
        if logout_fails {
            assert!(home.join("fixture-login").is_file());
        } else {
            assert!(entry_names(&home).is_empty());
        }
        assert_default_home(&default_home);
        assert_eq!(c.cwd.join("client-started").exists(), !named);
        service.cancel().await.unwrap();
        let output = child.wait_with_output().await.unwrap();
        assert!(output.status.success());
        let stdout = raw_stdout.await.unwrap();
        for bytes in [&stdout, &output.stderr] {
            let text = String::from_utf8_lossy(bytes);
            assert_no_login_canary(&text);
            for secret in [
                "synthetic-test-token",
                "synthetic-anthropic-key",
                "synthetic-openai-key",
            ] {
                assert!(!text.contains(secret));
            }
        }
        eprintln!("retire result after directory removal: {result}");
    }

    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    let mut child = tokio::process::Command::new(binary())
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
    let transport = (child.stdout.take().unwrap(), child.stdin.take().unwrap());
    let service = ().serve(transport).await.unwrap();
    let tools = service.list_all_tools().await.unwrap();
    let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
    assert_eq!(names, ["turn", "new-conversation"]);
    let invalid = service
        .call_tool(call("retire", json!({})))
        .await
        .unwrap_err();
    assert_no_login_canary(&invalid.to_string());
    assert!(!c.cwd.join("logouts.jsonl").exists());
    service.cancel().await.unwrap();
    let output = child.wait_with_output().await.unwrap();
    assert!(output.status.success());
    assert_no_login_canary(&String::from_utf8_lossy(&output.stderr));
}

const EXPECTED_ACCOUNT: &str = "synthetic-expected-account";
const NO_ACCOUNT: &str = "account hook (account/read): no account: Codex answered with none";

fn chatgpt(email: Value) -> Value {
    json!({
        "account": {"type":"chatgpt", "email":email, "planType":"plus", "workspaceName":null},
        "requiresOpenaiAuth": true,
        "workspaceRouting": null,
    })
}

fn write_account_script(work: &Path, script: Value) {
    fs::write(work.join("account-script.json"), script.to_string()).unwrap();
}

fn account_log(work: &Path, filename: &str) -> Vec<Value> {
    let log = match fs::read_to_string(work.join(filename)) {
        Ok(log) => log,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
        Err(error) => panic!("cannot read scripted account log: {error}"),
    };
    assert_hook_text(&log);
    log.lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn assert_hook_text(text: &str) {
    assert_no_login_canary(text);
    assert!(!text.contains("sentinel"));
}

fn hook_environment(c: &Config, home: &Path) -> BTreeMap<String, String> {
    let mut expected = c.env.clone();
    expected.remove("ANTHROPIC_API_KEY");
    expected.remove("OPENAI_API_KEY");
    expected.insert(
        HOME_VARIABLE.into(),
        home.canonicalize().unwrap().to_str().unwrap().into(),
    );
    expected
}

fn assert_account_client_gone(read: &Value) {
    let pid = i32::try_from(read["pid"].as_i64().unwrap()).unwrap();
    assert!(pid > 0);
    assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
}

/// The scripted app-server's child in a new session is gone; it is reaped by its new parent.
fn assert_detached_child_gone(work: &Path, case: &str) {
    let pid: i32 = fs::read_to_string(work.join("account-detached"))
        .unwrap()
        .parse()
        .unwrap();
    assert!(pid > 0);
    for _ in 0..200 {
        if unsafe { libc::kill(pid, 0) } == -1 {
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    unsafe { libc::kill(pid, libc::SIGKILL) };
    panic!("{case}: the app-server's child in a new session outlived the account hook");
}

async fn assert_resumed_turn(session: &mut Session) {
    let reply = session.turn("TEST").await.unwrap();
    assert_hook_text(&reply.to_string());
    let evidence: Value = serde_json::from_str(reply["text"].as_str().unwrap()).unwrap();
    let args = evidence["args"].as_array().unwrap();
    assert!(args.iter().any(|arg| arg == "resume"));
    assert_eq!(args[args.len() - 2], "fixture-exact-thread");
}

#[tokio::test]
async fn v2_account_hook_reads_codex_in_its_own_home() {
    for named_directory in [false, true] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        if named_directory {
            let project = parent.path().join("project");
            fs::create_dir(&project).unwrap();
            c.directory = Some(project.canonicalize().unwrap());
        }
        prepare(&c).unwrap();
        let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
        plant_login(&home);
        let login = fs::read(home.join("fixture-login")).unwrap();
        let work = c.directory.as_ref().unwrap_or(&c.cwd);
        let script = json!({"notify":true,"answer":chatgpt(json!(EXPECTED_ACCOUNT))});
        let mut session = Session::new(c.clone());
        let expected = AccountReport {
            account: Some(EXPECTED_ACCOUNT.into()),
            text: ACCOUNT_REPORTED.into(),
        };

        write_account_script(work, script.clone());
        let report = session.account().await.unwrap();
        assert_hook_text(&format!("{report:?}"));
        assert_eq!(report, expected);
        eprintln!("{report:?}");
        assert_eq!(
            account_log(work, "account-requests.jsonl"),
            [
                json!({"id":1,"method":"initialize","params":{"clientInfo":{"name":"agent-codex","version":env!("CARGO_PKG_VERSION")},"capabilities":{"experimentalApi":false}}}),
                json!({"method":"initialized"}),
                json!({"id":2,"method":"account/read","params":{"refreshToken":false}}),
            ]
        );
        let reads = account_log(work, "account-reads.jsonl");
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0]["args"], json!(account_arguments()));
        assert_eq!(
            reads[0]["args"],
            json!([
                "app-server",
                "--stdio",
                "-c",
                "cli_auth_credentials_store=\"file\""
            ])
        );
        assert_eq!(reads[0]["home"], json!(home.canonicalize().unwrap()));
        let mut expected_environment = hook_environment(&c, &home);
        expected_environment.insert(REFRESH_OVERRIDE_VARIABLE.into(), REFRESH_BLOCKED.into());
        assert_client_environment(&reads[0], &expected_environment);
        assert_eq!(reads[0]["cwd"], json!(work));
        assert_eq!(reads[0]["entries"], json!(["fixture-login"]));
        assert_account_client_gone(&reads[0]);
        assert_eq!(
            fs::metadata(&home).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(entry_names(&home), ["fixture-login", "tmp"]);
        assert_eq!(fs::read(home.join("fixture-login")).unwrap(), login);
        assert_default_home(&default_home);
        assert!(!work.join("client-started").exists());

        write_account_script(work, script.clone());
        let report = session.account().await.unwrap();
        assert_hook_text(&format!("{report:?}"));
        assert_eq!(report, expected);
        eprintln!("{report:?}");
        let reads = account_log(work, "account-reads.jsonl");
        assert_eq!(reads.len(), 2);
        assert_eq!(reads[1]["entries"], json!(["fixture-login", "tmp"]));
        assert_account_client_gone(&reads[1]);

        let other = parent.path().join("other");
        fs::create_dir(&other).unwrap();
        fs::write(other.join("logout-hang"), b"").unwrap();
        let other_home = parent.path().join("other-home");
        fs::DirBuilder::new()
            .mode(0o700)
            .create(&other_home)
            .unwrap();
        plant_login(&other_home);
        let expected_env = hook_environment(&c, &other_home);
        let mut child = tokio::process::Command::new(&c.program)
            .args(logout_arguments())
            .env_clear()
            .envs(&expected_env)
            .current_dir(&other)
            .process_group(0)
            .kill_on_drop(true)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let pid = child.id().unwrap() as i32;
        assert!(session.tree().record_group(pid));
        tokio::time::timeout(Duration::from_secs(10), async {
            while !fs::read(other.join("logouts.jsonl")).is_ok_and(|log| log.ends_with(b"\n")) {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
        let logouts = recorded_logouts(&other);
        assert_eq!(logouts.len(), 1);
        assert_client_environment(&logouts[0], &expected_env);

        write_account_script(work, script.clone());
        let report = session.account().await.unwrap();
        assert_hook_text(&format!("{report:?}"));
        assert_eq!(report, expected);
        eprintln!("{report:?}");
        let reads = account_log(work, "account-reads.jsonl");
        assert_eq!(reads.len(), 3);
        assert_account_client_gone(&reads[2]);
        assert_eq!(unsafe { libc::kill(pid, 0) }, 0);
        assert!(child.try_wait().unwrap().is_none());
        assert!(other_home.join("fixture-login").is_file());
        assert_eq!(unsafe { libc::kill(-pid, libc::SIGKILL) }, 0);
        assert_eq!(child.wait().await.unwrap().signal(), Some(libc::SIGKILL));
        assert!(!work.join("client-started").exists());
        assert_hook_text(&session.turn("TEST").await.unwrap().to_string());

        write_account_script(work, script);
        let report = session.account().await.unwrap();
        assert_hook_text(&format!("{report:?}"));
        assert_eq!(report, expected);
        eprintln!("between turns: {report:?}");
        assert_resumed_turn(&mut session).await;
        assert_default_home(&default_home);
    }
}

#[tokio::test]
async fn v2_account_hook_reports_no_account_without_a_chatgpt_email() {
    let parent = tempfile::tempdir().unwrap();
    let default_home = plant_default_home(parent.path());
    let mut c = config(parent.path(), false, false);
    c.v2 = true;
    prepare(&c).unwrap();
    let mut session = Session::new(c.clone());
    for (answer, cause) in [
        (
            json!({"account":null,"requiresOpenaiAuth":true}),
            "Codex answered with none",
        ),
        (
            json!({"account":null,"requiresOpenaiAuth":false}),
            "Codex answered with none",
        ),
        (
            json!({"account":{"type":"apiKey"},"requiresOpenaiAuth":true}),
            "the login is not a ChatGPT login",
        ),
        (
            json!({"account":{"type":"amazonBedrock","usesCodexManagedCredentials":false},"requiresOpenaiAuth":true}),
            "the login is not a ChatGPT login",
        ),
        (chatgpt(Value::Null), "the ChatGPT login names no email"),
        (chatgpt(json!("")), "the ChatGPT login names no email"),
    ] {
        write_account_script(&c.cwd, json!({"notify":true,"answer":answer}));
        let report = session.account().await.unwrap();
        assert_hook_text(&report.text);
        assert_eq!(
            report,
            AccountReport {
                account: None,
                text: format!("account hook (account/read): no account: {cause}")
            }
        );
        eprintln!("{}", report.text);
    }
    let reads = account_log(&c.cwd, "account-reads.jsonl");
    assert_eq!(reads.len(), 6);
    assert_eq!(reads[0]["entries"], json!([]));
    let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
    assert_eq!(
        fs::metadata(&home).unwrap().permissions().mode() & 0o777,
        0o700
    );
    assert_default_home(&default_home);
}

#[tokio::test]
async fn v2_account_hook_error_paths_hold_no_canary() {
    let unset = format!("client home ({HOME_VARIABLE}): CONDUIT_STATE_DIR is unset");
    let configuration_unset = format!("invalid configuration: {unset}");
    let dotenv = format!("client home ({HOME_VARIABLE}): {DOTENV_REFUSAL}");
    let outside =
        format!("client home ({HOME_VARIABLE}): the home resolves outside CONDUIT_STATE_DIR");
    for (case, changes, expected, reads) in [
        (
            "no-client",
            json!({}),
            "account hook (start): client did not start",
            0,
        ),
        (
            "no-client-directory",
            json!({}),
            "account hook (start): client did not start: the client directory is gone",
            0,
        ),
        (
            "initialize-exit",
            json!({"initialize":"exit:3"}),
            "account hook (initialize): client exit: status 3",
            1,
        ),
        (
            "initialize-error",
            json!({"initialize":"error"}),
            "account hook (initialize): error answer, code -32600",
            1,
        ),
        (
            "initialize-hang",
            json!({"initialize":"hang"}),
            "account hook (initialize): no answer within 3 seconds",
            1,
        ),
        (
            "read-eof",
            json!({"read":"eof"}),
            "account hook (account/read): EOF before answer",
            1,
        ),
        (
            "read-error",
            json!({"read":"error"}),
            "account hook (account/read): error answer, code -32600",
            1,
        ),
        (
            "read-invalid",
            json!({"read":"invalid"}),
            "account hook (account/read): invalid answer",
            1,
        ),
        (
            "read-wrong-id",
            json!({"read":"wrong-id"}),
            "account hook (account/read): invalid answer",
            1,
        ),
        (
            "read-shape",
            json!({"answer":{"account":{"type":"chatgpt","planType":"plus"},"requiresOpenaiAuth":true}}),
            "account hook (account/read): invalid answer",
            1,
        ),
        (
            "read-hang",
            json!({"read":"hang"}),
            "account hook (account/read): no answer within 3 seconds",
            1,
        ),
        (
            "shared-deadline",
            json!({"delayMs":{"initialize":2000,"read":2000}}),
            "account hook (account/read): no answer within 3 seconds",
            1,
        ),
        (
            "detached-eof",
            json!({"detach":true,"exitOnEof":true}),
            NO_ACCOUNT,
            1,
        ),
        (
            "detached-exit",
            json!({"detach":true,"read":"eof-exit"}),
            "account hook (account/read): client exit: status 0",
            1,
        ),
        ("dotenv", json!({}), &dotenv, 0),
        ("outside-link", json!({}), &outside, 0),
        ("unset", json!({}), &unset, 0),
        ("configuration-unset", json!({}), &configuration_unset, 0),
        ("configuration", json!({}), NO_ACCOUNT, 1),
        ("retired", json!({}), RETIRED_REFUSAL, 0),
        ("v1", json!({}), "account hook: a .v1 Agent has none", 0),
    ] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        prepare(&c).unwrap();
        let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        let login_home = if case == "outside-link" {
            let elsewhere = parent.path().join("elsewhere");
            symlink(&elsewhere, &home).unwrap();
            elsewhere
        } else {
            home.clone()
        };
        fs::DirBuilder::new()
            .mode(0o700)
            .create(&login_home)
            .unwrap();
        plant_login(&login_home);
        let login_path = login_home.join("fixture-login");
        let login = fs::read(&login_path).unwrap();
        let mut script = json!({"notify":true});
        script
            .as_object_mut()
            .unwrap()
            .extend(changes.as_object().unwrap().clone());
        write_account_script(&c.cwd, script);
        match case {
            "no-client" => c.program = parent.path().join("missing-client"),
            "no-client-directory" => c.directory = Some(parent.path().join("missing-directory")),
            "dotenv" => fs::write(home.join(".env"), AGENT_DOTENV).unwrap(),
            "unset" => c.state_dir = None,
            "configuration-unset" => {
                c.state_dir = None;
                c.configuration_error = Some(configuration_unset.clone());
            }
            "configuration" => c.configuration_error = Some(
                "invalid configuration: effort (AGENT_EFFORT): expected lowercase ASCII letters"
                    .into(),
            ),
            "v1" => c.v2 = false,
            _ => {}
        }
        let mut session = Session::new(c.clone());
        if case == "retired" {
            let text = session.retire().await.unwrap();
            assert_hook_text(&text);
            assert_eq!(text, RETIRED);
        }
        let result = if matches!(case, "initialize-hang" | "read-hang" | "shared-deadline") {
            session.account_within(Duration::from_secs(3)).await
        } else {
            session.account().await
        };
        let text = if matches!(case, "configuration" | "detached-eof") {
            let report = result.unwrap();
            assert_eq!(
                report,
                AccountReport {
                    account: None,
                    text: expected.into()
                }
            );
            report.text
        } else {
            result.unwrap_err().to_string()
        };
        assert_hook_text(&text);
        assert_eq!(text, expected, "{case}");
        eprintln!("{case}: {text}");
        let logged = account_log(&c.cwd, "account-reads.jsonl");
        assert_eq!(logged.len(), reads, "{case}");
        for read in &logged {
            assert_account_client_gone(read);
        }
        if case.starts_with("detached") {
            if case == "detached-eof" {
                assert!(
                    !c.cwd.join("account-eof").exists(),
                    "detached-eof: the hook closed the app-server's input before its stop"
                );
            }
            assert_detached_child_gone(&c.cwd, case);
        }
        if case == "retired" {
            assert!(entry_names(&home).is_empty());
        } else {
            assert_eq!(fs::read(login_path).unwrap(), login);
        }
        assert_default_home(&default_home);
        assert!(!c.cwd.join("client-started").exists());
    }
}

#[tokio::test]
async fn v2_sign_out_hook_signs_out_and_keeps_the_home() {
    for case in ["success", "exit", "deadline", "dotenv", "retired", "v1"] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = case != "v1";
        prepare(&c).unwrap();
        let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
        fs::create_dir(home.join("nested")).unwrap();
        fs::write(home.join("nested/file"), b"kept").unwrap();
        plant_login(&home);
        let login_path = home.join("fixture-login");
        let login = fs::read(&login_path).unwrap();
        let mut session = Session::new(c.clone());
        let dotenv = format!("client home ({HOME_VARIABLE}): {DOTENV_REFUSAL}");
        let expected = match case {
            "success" => SIGNED_OUT,
            "exit" => {
                fs::write(c.cwd.join("logout-exit"), b"3").unwrap();
                "client logout failed: client exit: status 3"
            }
            "deadline" => {
                fs::write(c.cwd.join("logout-hang"), b"").unwrap();
                "client logout failed: no exit within 3 seconds"
            }
            "dotenv" => {
                fs::write(home.join(".env"), AGENT_DOTENV).unwrap();
                &dotenv
            }
            "retired" => {
                let text = session.retire().await.unwrap();
                assert_hook_text(&text);
                assert_eq!(text, RETIRED);
                RETIRED_REFUSAL
            }
            "v1" => "sign-out hook: a .v1 Agent has none",
            _ => unreachable!(),
        };
        let result = if case == "deadline" {
            session.sign_out_within(Duration::from_secs(3)).await
        } else {
            session.sign_out().await
        };
        let text = if case == "success" {
            result.unwrap()
        } else {
            result.unwrap_err().to_string()
        };
        assert_hook_text(&text);
        assert_eq!(text, expected, "{case}");
        eprintln!("{case}: {text}");
        let logouts = recorded_logouts(&c.cwd);
        assert_eq!(logouts.len(), usize::from(!matches!(case, "dotenv" | "v1")));
        if let Some(logout) = logouts.first() {
            assert_hook_text(&logout.to_string());
            assert_eq!(logout["args"], json!(logout_arguments()));
            assert_eq!(logout["cwd"], json!(c.cwd));
            assert_eq!(logout["entries"], json!(["fixture-login", "nested"]));
            assert_client_environment(logout, &hook_environment(&c, &home));
        }
        if case == "retired" {
            assert!(entry_names(&home).is_empty());
        } else if case != "success" {
            assert_eq!(fs::read(&login_path).unwrap(), login);
        }
        if case == "exit" {
            fs::remove_file(c.cwd.join("logout-exit")).unwrap();
            let text = session.sign_out().await.unwrap();
            assert_hook_text(&text);
            assert_eq!(text, SIGNED_OUT);
            eprintln!("{case} retry: {text}");
            assert_eq!(recorded_logouts(&c.cwd).len(), 2);
        }
        if matches!(case, "success" | "exit") {
            assert_eq!(entry_names(&home), ["nested"]);
            assert_eq!(fs::read(home.join("nested/file")).unwrap(), b"kept");
            assert_eq!(
                fs::metadata(&home).unwrap().permissions().mode() & 0o777,
                0o700
            );
        }
        assert_default_home(&default_home);
        assert!(!c.cwd.join("client-started").exists());
        if case == "success" {
            assert_hook_text(&session.turn("TEST").await.unwrap().to_string());
            assert!(c.cwd.join("client-started").is_file());

            plant_login(&home);
            let text = session.sign_out().await.unwrap();
            assert_hook_text(&text);
            assert_eq!(text, SIGNED_OUT);
            eprintln!("between turns: {text}");
            assert_eq!(entry_names(&home), ["nested"]);
            assert_eq!(recorded_logouts(&c.cwd).len(), 2);
            assert_resumed_turn(&mut session).await;
            assert_default_home(&default_home);
        }
    }
}

#[tokio::test]
async fn v2_account_and_sign_out_tools_through_the_wrapper() {
    use rmcp::{
        ClientLifecycleMode, ClientServiceExt, ServiceExt,
        model::{CallToolRequestParams, ProtocolVersion},
    };

    let call = |name: &str, arguments: Value| -> CallToolRequestParams {
        serde_json::from_value(json!({"name":name,"arguments":arguments})).unwrap()
    };
    for v2 in [true, false] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let c = config(parent.path(), false, false);
        let mut command = tokio::process::Command::new(binary());
        command
            .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
            .env_clear()
            .envs(&c.env)
            .env("AGENT_ACCOUNT_SERVICES", "false")
            .current_dir(&c.cwd)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let state = c.state_dir.as_ref().unwrap();
        let home = state.join(HOME_NAME);
        if v2 {
            fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
            plant_login(&home);
            command
                .env("AGENT_TURN_DEADLINE_SECONDS", "60")
                .env("CONDUIT_STATE_DIR", state)
                .env(HOME_VARIABLE, parent.path().join("elsewhere"));
        } else {
            command
                .env("AGENT_PEER_MESSAGING", "false")
                .env("AMBIENT_SENTINEL", "synthetic-ambient-secret");
        }
        let mut child = command.spawn().unwrap();
        let transport = (child.stdout.take().unwrap(), child.stdin.take().unwrap());
        let service = if v2 {
            let lifecycle = ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            };
            ().serve_with_lifecycle(transport, lifecycle).await.unwrap()
        } else {
            ().serve(transport).await.unwrap()
        };
        let tools = service.list_all_tools().await.unwrap();
        let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
        let expected_names: &[_] = if v2 {
            &[
                "turn",
                "retire",
                "account",
                "sign-out",
                "sign-in",
                "new-conversation",
            ]
        } else {
            &["turn", "new-conversation"]
        };
        assert_eq!(names, expected_names);
        for name in ["account", "sign-out"] {
            let arguments = if v2 { json!({"extra":true}) } else { json!({}) };
            let error = service.call_tool(call(name, arguments)).await.unwrap_err();
            assert_hook_text(&error.to_string());
        }

        if v2 {
            for (script, text, content) in [
                (
                    json!({"notify":true,"answer":chatgpt(json!(EXPECTED_ACCOUNT))}),
                    ACCOUNT_REPORTED,
                    Some(json!({"account":EXPECTED_ACCOUNT})),
                ),
                (
                    json!({"notify":true}),
                    NO_ACCOUNT,
                    Some(json!({"account":null})),
                ),
                (
                    json!({"notify":true,"read":"error"}),
                    "account hook (account/read): error answer, code -32600",
                    None,
                ),
            ] {
                write_account_script(&c.cwd, script);
                let result = service.call_tool(call("account", json!({}))).await.unwrap();
                let result = serde_json::to_value(result).unwrap();
                assert_hook_text(&result.to_string());
                let mut expected = json!({"isError":content.is_none(),"resultType":"complete","content":[{"type":"text","text":text}]});
                if let Some(content) = content {
                    expected["structuredContent"] = content;
                }
                assert_eq!(result, expected);
                eprintln!("{result}");
            }
            let result = service
                .call_tool(call("sign-out", json!({})))
                .await
                .unwrap();
            let result = serde_json::to_value(result).unwrap();
            assert_hook_text(&result.to_string());
            assert_eq!(
                result,
                json!({"isError":false,"resultType":"complete","content":[{"type":"text","text":SIGNED_OUT}]})
            );
            let reads = account_log(&c.cwd, "account-reads.jsonl");
            let logouts = recorded_logouts(&c.cwd);
            assert_eq!(reads.len(), 3);
            assert_eq!(logouts.len(), 1);
            let canonical_home = json!(home.canonicalize().unwrap());
            for log in reads.iter().chain(&logouts) {
                assert_eq!(log["environment"][HOME_VARIABLE], canonical_home);
            }
            assert!(!home.join("fixture-login").exists());
        } else {
            assert!(!c.cwd.join("account-reads.jsonl").exists());
            assert!(!c.cwd.join("logouts.jsonl").exists());
        }
        assert_default_home(&default_home);
        service.cancel().await.unwrap();
        let output = child.wait_with_output().await.unwrap();
        assert!(output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_hook_text(&stderr);
        for synthetic in [
            "synthetic-test-token",
            "synthetic-anthropic-key",
            "synthetic-openai-key",
        ] {
            assert!(!stderr.contains(synthetic));
        }
    }
}

#[tokio::test]
async fn v2_client_home_refusals_start_no_client() {
    for (case, cause) in [
        ("unset", "CONDUIT_STATE_DIR is unset"),
        ("missing", "CONDUIT_STATE_DIR is not an existing directory"),
        ("file", "the home is not a directory"),
        (
            "default-link",
            "the home resolves outside CONDUIT_STATE_DIR",
        ),
        (
            "outside-link",
            "the home resolves outside CONDUIT_STATE_DIR",
        ),
        ("dangling-link", "the home is not a directory"),
        (
            "non-unicode",
            "the canonical home path is not valid Unicode",
        ),
        ("replaced", "the home resolves outside CONDUIT_STATE_DIR"),
        ("dotenv-file", DOTENV_REFUSAL),
        ("dotenv-link", DOTENV_REFUSAL),
    ] {
        if CLAUDE && matches!(case, "non-unicode" | "replaced" | "dotenv-link") {
            continue;
        }
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        let dotenv = home.join(".env");
        match case {
            "unset" => c.state_dir = None,
            "missing" => c.state_dir = Some(parent.path().join("missing")),
            "file" => fs::write(&home, "regular file").unwrap(),
            "default-link" => symlink(&default_home, &home).unwrap(),
            "outside-link" => {
                let elsewhere = parent.path().join("elsewhere");
                fs::create_dir(&elsewhere).unwrap();
                symlink(elsewhere, &home).unwrap();
            }
            "dangling-link" => symlink(parent.path().join("missing"), &home).unwrap(),
            "non-unicode" => {
                let name = std::ffi::OsStr::from_bytes(b"non-unicode-\xff");
                let target = home.with_file_name(name);
                if let Err(error) = fs::create_dir(&target) {
                    // macOS can reject non-Unicode names with EPERM, EINVAL, or EILSEQ.
                    if cfg!(target_os = "macos")
                        && matches!(error.raw_os_error(), Some(1 | 22 | 92))
                    {
                        eprintln!(
                            "non-Unicode client home case skipped: filesystem rejected the test name"
                        );
                        continue;
                    }
                    panic!("could not create non-Unicode test directory: {error}");
                }
                symlink(target, &home).unwrap();
                assert!(home.canonicalize().unwrap().to_str().is_none());
            }
            "dotenv-file" | "dotenv-link" => {
                fs::create_dir(&home).unwrap();
                if case == "dotenv-file" {
                    fs::write(&dotenv, AGENT_DOTENV).unwrap();
                } else {
                    symlink(home.join("missing"), &dotenv).unwrap();
                }
            }
            "replaced" => {}
            _ => unreachable!(),
        }
        prepare(&c).unwrap();
        let mut session = Session::new(c.clone());
        if CLAUDE && case == "dotenv-file" {
            let reply = session.turn("HOME").await.unwrap();
            let evidence: Value = serde_json::from_str(reply["text"].as_str().unwrap()).unwrap();
            assert_eq!(evidence["home"], json!(home.canonicalize().unwrap()));
            assert_eq!(evidence["dotenv"], json!([]));
            assert_default_home(&default_home);
        } else {
            let turn = if case == "replaced" {
                session.turn("HOME").await.unwrap();
                fs::remove_file(c.cwd.join("client-started")).unwrap();
                fs::rename(&home, home.with_file_name("previous-home")).unwrap();
                symlink(&default_home, &home).unwrap();
                2
            } else {
                1
            };
            assert_home_refusal(&mut session, turn, cause, &c, &default_home).await;
        }
        if case == "dotenv-file" {
            assert_eq!(fs::read(dotenv).unwrap(), AGENT_DOTENV.as_bytes());
        } else if case == "dotenv-link" {
            assert_eq!(fs::read_link(dotenv).unwrap(), home.join("missing"));
        }
        session.shutdown().await;
    }
}

#[tokio::test]
async fn v1_launch_environment_equals_0_1_1() {
    use rmcp::ServiceExt;

    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    let mut expected = c.env.clone();
    expected.remove("ANTHROPIC_API_KEY");
    expected.remove("OPENAI_API_KEY");
    expected.extend(
        [
            ("LOGNAME", "synthetic-user"),
            ("USER", "synthetic-user"),
            ("TERM", "dumb"),
            ("SHELL", "/bin/sh"),
        ]
        .map(|(name, value)| (name.into(), value.into())),
    );
    let state = c.state_dir.as_ref().unwrap();
    let mut child = tokio::process::Command::new(binary())
        .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
        .env_clear()
        .envs(&expected)
        .env("CONDUIT_STATE_DIR", state)
        .env(HOME_VARIABLE, parent.path().join("elsewhere"))
        .env("ANTHROPIC_API_KEY", "synthetic-anthropic-key")
        .env("OPENAI_API_KEY", "synthetic-openai-key")
        .env("AMBIENT_SENTINEL", "synthetic-ambient-secret")
        .env("AGENT_ACCOUNT_SERVICES", "false")
        .env("AGENT_PEER_MESSAGING", "false")
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
    let call =
        serde_json::from_value(json!({"name":"turn","arguments":{"message":"TEST"}})).unwrap();
    let reply = service.call_tool(call).await.unwrap();
    assert_eq!(reply.is_error, Some(false));
    let wire = serde_json::to_value(reply).unwrap();
    let evidence: Value =
        serde_json::from_str(wire["structuredContent"]["text"].as_str().unwrap()).unwrap();
    assert_client_environment(&evidence, &expected);
    assert!(!state.join(HOME_NAME).exists());
    assert!(
        !arguments(&c, None)
            .iter()
            .any(|arg| arg == CREDENTIAL_STORE)
    );
    service.cancel().await.unwrap();
    let output = child.wait_with_output().await.unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}

static LOGIN_PORT: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn browser_login() -> StatedLogin {
    stated_logins()
        .into_iter()
        .find(|login| login.name == BROWSER_LOGIN)
        .unwrap()
}

fn login_redirect(login: &StatedLogin, port: u16) -> String {
    format!(
        "http://{}:{port}{}",
        login.host.as_deref().unwrap(),
        login.path.as_deref().unwrap(),
    )
}

fn login_url(login: &StatedLogin, redirect: &str) -> reqwest::Url {
    let mut url = reqwest::Url::parse(&login.authorization).unwrap();
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", &login.client_id)
        .append_pair("redirect_uri", redirect)
        .append_pair(
            "code_challenge",
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQ",
        )
        .append_pair("code_challenge_method", "S256")
        .append_pair("state", "fixture-state-0123456789abcdef");
    url
}

fn assert_login_text(text: &str) {
    assert_hook_text(text);
    assert!(!text.contains("canary.invalid"));
}

fn write_login_script(work: &Path, script: Value) {
    fs::write(work.join("login-script.json"), script.to_string()).unwrap();
}

fn login_pid(log: &Value) -> i32 {
    let pid = i32::try_from(log["pid"].as_i64().unwrap()).unwrap();
    assert!(pid > 0);
    pid
}

async fn wait_for_login_exit(pid: i32) {
    tokio::time::timeout(Duration::from_secs(2), async {
        while unsafe { libc::kill(pid, 0) } != -1 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the scripted login outlived its stop");
}

fn retry_login(state: &str) -> SignInCall {
    SignInCall {
        request_state: Some(state.into()),
        ..Default::default()
    }
}

async fn poll_login(session: &mut Session, state: &str, work: &Path) -> anyhow::Result<SignIn> {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            write_login_script(work, json!({}));
            let result = session.sign_in(retry_login(state)).await;
            assert_login_text(&format!("{result:?}"));
            if !matches!(&result, Ok(SignIn::Text(text)) if text == LOGIN_WAITING) {
                return result;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("the login retry never reported an exit")
}

async fn assert_browser_start(session: &mut Session, work: &Path, call: SignInCall) -> String {
    write_login_script(work, json!({}));
    let result = session.sign_in(call).await.expect("browser sign-in failed");
    assert_login_text(&format!("{result:?}"));
    let SignIn::Url { url, request_state } = result else {
        panic!("expected a URL request");
    };
    let login = browser_login();
    let expected = login_url(&login, &login_redirect(&login, login.ports[0]));
    assert!(
        url == expected.as_str(),
        "relayed URL differs from the scripted login"
    );
    assert!(request_state.starts_with("agent-codex-login-"));
    eprintln!("browser login returned URL and request state");
    request_state
}

#[tokio::test]
async fn v2_stated_logins_match_the_type() {
    let _port = LOGIN_PORT.lock().await;
    let logins = stated_logins();
    assert_eq!(logins.len(), 2);
    let browser = &logins[0];
    let device = &logins[1];
    assert_eq!(browser.name, BROWSER_LOGIN);
    assert_eq!(browser.kind, "redirect");
    assert_eq!(browser.carried, Some(true));
    assert_eq!(browser.ports.len(), 2);
    assert!(browser.host.is_some() && browser.path.is_some() && browser.issuer.is_some());
    assert_eq!(device.name, DEVICE_LOGIN);
    assert_eq!(device.kind, "device");
    assert!(device.host.is_none() && device.path.is_none() && device.issuer.is_none());
    assert!(device.ports.is_empty() && device.carried.is_none());
    for login in &logins {
        assert!(login.authorization.starts_with("https://") && !login.client_id.is_empty());
    }
    let contracts = include_str!("../contracts.yaml");
    let [first, second] = browser.ports.as_slice() else {
        unreachable!();
    };
    let declaration = format!("ports: [{first}, {second}]");
    assert!(parse_stated_logins(contracts).is_some());
    for malformed in [
        "ports: []".into(),
        format!("ports: [{first}]"),
        format!("ports: [{first}, {second}, {first}]"),
        format!("ports: [{first}, other]"),
    ] {
        assert!(parse_stated_logins(&contracts.replacen(&declaration, &malformed, 1)).is_none());
    }
    let host = browser.host.as_deref().unwrap();
    let stated_host = format!("host: {host}, ports: [");
    assert!(contracts.contains(&stated_host));
    for malformed in [
        "host: , ports: [".into(),
        format!("host: {host}, extra: x, ports: ["),
    ] {
        assert!(parse_stated_logins(&contracts.replacen(&stated_host, &malformed, 1)).is_none());
    }

    let redirect = login_redirect(browser, browser.ports[0]);
    let url = login_url(browser, &redirect);
    assert!(redirect_url_matches(browser, url.as_str()));
    let fallback = login_url(browser, &login_redirect(browser, browser.ports[1]));
    assert!(redirect_url_matches(browser, fallback.as_str()));
    assert!(!redirect_url_matches(device, url.as_str()));

    let mut cli_url = url.clone();
    cli_url
        .query_pairs_mut()
        .append_pair(
            "scope",
            "openid profile email offline_access api.connectors.read api.connectors.invoke",
        )
        .append_pair("id_token_add_organizations", "true")
        .append_pair("codex_cli_simplified_flow", "true")
        .append_pair("originator", "codex_cli_rs");
    assert!(redirect_url_matches(browser, cli_url.as_str()));

    let reject = |text: &str| assert!(!redirect_url_matches(browser, text));
    reject(&url.as_str().replacen("https://", "http://", 1));
    reject(&format!("{url}#x"));
    reject(
        &url.as_str()
            .replacen("https://", &format!("https://user{}", char::from(0x40)), 1),
    );
    reject(
        &url.as_str()
            .replacen("https://", &format!("https://{}", char::from(0x40)), 1),
    );
    for separators in ["/\\", "\\/", "//\\", "/\\/"] {
        let prefix = format!("https:{separators}{}", char::from(0x40));
        reject(&url.as_str().replacen("https://", &prefix, 1));
    }
    // A normalized path can also conceal text that would survive in the relayed original.
    reject(&format!(
        "{}/sentinel/..{}?{}",
        url.origin().ascii_serialization(),
        url.path(),
        url.query().unwrap()
    ));
    let mut other_path = url.clone();
    other_path.set_path(&format!("{}x", url.path()));
    reject(other_path.as_str());

    let changed = |name: &str, value: &str| {
        let mut changed = url.clone();
        changed
            .query_pairs_mut()
            .clear()
            .extend_pairs(url.query_pairs().map(|(key, old)| {
                let value = if key == name { value.into() } else { old };
                (key, value)
            }));
        reject(changed.as_str());
    };
    for (name, value) in [
        ("client_id", "other-client"),
        ("response_type", "token"),
        ("code_challenge_method", "plain"),
        (
            "code_challenge",
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOP",
        ),
        (
            "code_challenge",
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOP+",
        ),
        ("state", "123456789012345678901"),
        ("state", "fixture-state-0123456789abcde/"),
    ] {
        changed(name, value);
    }
    reject(&format!("{url}&state=another-fixture-state-abcdef"));
    for (name, value) in [
        ("access_token", "sentinel-token"),
        ("login_hint", "sentinel-person"),
        ("scope", "openid sentinel-claim"),
        ("originator", "sentinel-caller"),
        ("id_token_add_organizations", "sentinel-value"),
        ("codex_cli_simplified_flow", "sentinel-value"),
    ] {
        let mut with_extra = url.clone();
        with_extra.query_pairs_mut().append_pair(name, value);
        reject(with_extra.as_str());
    }
    for changed_redirect in [
        redirect.replacen(browser.host.as_deref().unwrap(), "elsewhere.invalid", 1),
        login_redirect(browser, browser.ports[0] + 1),
        format!("{redirect}x"),
        format!("{redirect}?x=1"),
        format!("{redirect}#x"),
        redirect.replacen("http://", "https://", 1),
        redirect.replacen("http://", &format!("http://user{}", char::from(0x40)), 1),
    ] {
        changed("redirect_uri", &changed_redirect);
    }
    for separators in ["//", "/\\", "\\/", "//\\", "/\\/"] {
        let prefix = format!("http:{separators}{}", char::from(0x40));
        changed("redirect_uri", &redirect.replacen("http://", &prefix, 1));
    }
    reject(
        "https://canary.invalid/authorize?client_id=sentinel-canary&state=sentinel-canary-state",
    );
    eprintln!("both stated ports passed the URL check and every altered URL was refused");
}

#[tokio::test]
async fn v2_new_conversation_keeps_a_sign_in_in_progress() {
    let _port = LOGIN_PORT.lock().await;
    let parent = tempfile::tempdir().unwrap();
    let mut c = config(parent.path(), false, false);
    c.v2 = true;
    prepare(&c).unwrap();
    let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
    fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
    let mut session = Session::new(c.clone());
    let state = assert_browser_start(&mut session, &c.cwd, SignInCall::default()).await;
    let pid = login_pid(&account_log(&c.cwd, "logins.jsonl")[0]);

    let refused = session.new_conversation().await.unwrap_err();
    assert_eq!(
        refused.to_string(),
        "new conversation refused: a sign-in is in progress in this Agent's own home; nothing was stopped; call again once the sign-in has ended"
    );
    assert_eq!(unsafe { libc::kill(pid, 0) }, 0);
    assert_eq!(
        session.sign_in(retry_login(&state)).await.unwrap(),
        SignIn::Text(LOGIN_WAITING.into())
    );

    fs::write(c.cwd.join("login-finish"), "").unwrap();
    wait_for_login_exit(pid).await;
    let result = session.new_conversation().await.unwrap();
    assert_eq!(result["conversation"], 2);
    assert_eq!(result["ended"]["turns"], 0);
    session.shutdown().await;
}

#[tokio::test]
async fn v2_sign_in_hook_starts_the_browser_login() {
    let _port = LOGIN_PORT.lock().await;
    for named_directory in [false, true] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        if named_directory {
            let project = parent.path().join("project");
            fs::create_dir(&project).unwrap();
            c.directory = Some(project.canonicalize().unwrap());
        }
        prepare(&c).unwrap();
        let work = c.directory.as_ref().unwrap_or(&c.cwd);
        let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
        plant_login(&home);
        let mut session = Session::new(c.clone());

        let first = assert_browser_start(&mut session, work, SignInCall::default()).await;
        let log = account_log(work, "logins.jsonl");
        assert_eq!(log.len(), 1);
        assert_eq!(
            log[0]["args"],
            json!(["login", "-c", "cli_auth_credentials_store=\"file\""])
        );
        assert_client_environment(&log[0], &hook_environment(&c, &home));
        assert_eq!(log[0]["home"], json!(home.canonicalize().unwrap()));
        assert_eq!(log[0]["cwd"], json!(work));
        assert_eq!(log[0]["entries"], json!(["fixture-login"]));
        let pid = login_pid(&log[0]);
        assert_eq!(unsafe { libc::kill(pid, 0) }, 0);

        write_login_script(work, json!({}));
        let SignIn::Text(waiting) = session.sign_in(retry_login(&first)).await.unwrap() else {
            panic!("expected a waiting text");
        };
        assert_login_text(&waiting);
        assert_eq!(waiting, LOGIN_WAITING);
        eprintln!("{waiting}");

        let second = assert_browser_start(&mut session, work, SignInCall::default()).await;
        assert_ne!(first, second);
        wait_for_login_exit(pid).await;
        write_login_script(work, json!({}));
        let stale = session
            .sign_in(retry_login(&first))
            .await
            .unwrap_err()
            .to_string();
        assert_login_text(&stale);
        assert_eq!(stale, STATE_UNKNOWN);
        eprintln!("{stale}");

        let asked = SignInCall {
            login: Some(json!({"name":"browser"})),
            ..Default::default()
        };
        let third = assert_browser_start(&mut session, work, asked).await;
        assert_ne!(second, third);
        let log = account_log(work, "logins.jsonl");
        assert_eq!(log.len(), 3);
        wait_for_login_exit(login_pid(&log[1])).await;
        let detached = if named_directory {
            fs::write(work.join("login-detach"), "").unwrap();
            let marker = work.join("login-detached");
            wait_for(&marker).await;
            let pid: i32 = fs::read_to_string(marker).unwrap().parse().unwrap();
            assert!(pid > 1);
            assert_eq!(unsafe { libc::getpgid(pid) }, pid);
            // Allow the tracker to sample the descendant while its parent is still alive.
            tokio::time::sleep(Duration::from_millis(600)).await;
            Some(pid)
        } else {
            None
        };
        fs::write(work.join("login-finish"), "").unwrap();
        wait_for_login_exit(login_pid(&log[2])).await;
        let ended = poll_login(&mut session, &third, work).await.unwrap();
        assert_eq!(ended, SignIn::Text(LOGIN_ENDED.into()));
        eprintln!("{LOGIN_ENDED}");
        if let Some(pid) = detached {
            wait_for_login_exit(pid).await;
            eprintln!("login cleanup stopped the descendant started after URL relay");
            fs::remove_file(work.join("login-detach")).unwrap();
        }
        // A sign-in refused before its client starts keeps the ended login's outcome.
        let listener =
            std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, browser_login().ports[0]))
                .unwrap();
        let refused = session
            .sign_in(SignInCall::default())
            .await
            .unwrap_err()
            .to_string();
        drop(listener);
        assert!(refused.contains("PND-010"), "{refused}");
        let kept = poll_login(&mut session, &third, work).await.unwrap();
        assert_eq!(kept, SignIn::Text(LOGIN_ENDED.into()));
        eprintln!("an ended login kept its outcome through a refused sign-in");
        assert_eq!(
            fs::read(home.join("fixture-login")).unwrap(),
            b"fixture-signed-in"
        );
        assert_default_home(&default_home);
        assert!(!work.join("client-started").exists());

        fs::remove_file(work.join("login-finish")).unwrap();
        fs::write(work.join("login-exit"), "3").unwrap();
        let fourth = assert_browser_start(&mut session, work, SignInCall::default()).await;
        fs::write(work.join("login-finish"), "").unwrap();
        let error = poll_login(&mut session, &fourth, work)
            .await
            .unwrap_err()
            .to_string();
        assert_login_text(&error);
        assert_eq!(
            error,
            "sign-in hook (browser): the browser login ended: client exit: status 3"
        );
        eprintln!("{error}");

        let reply = session.turn("TEST").await.unwrap();
        assert_login_text(&reply.to_string());
        eprintln!("turn succeeded after the login exited");
        session.shutdown().await;
    }
}

#[tokio::test]
async fn v2_sign_in_hook_refusals_hold_no_canary() {
    let _port = LOGIN_PORT.lock().await;
    let port = browser_login().ports[0];
    let port_error = format!(
        "sign-in hook (browser): PND-010: the client cannot bind its login port {port} on this machine's loopback: it is in use or not available"
    );
    let dotenv = format!("client home (CODEX_HOME): {DOTENV_REFUSAL}");
    let deadline_text = "sign-in hook (browser): no login URL within 3 seconds";
    for (case, expected, count) in [
        ("device", DEVICE_REFUSAL, 0),
        ("unknown", LOGIN_UNKNOWN, 0),
        ("malformed", LOGIN_UNKNOWN, 0),
        ("return", RETURN_REFUSAL, 0),
        ("stale-state", STATE_UNKNOWN, 0),
        ("port", &port_error, 0),
        (
            "exit",
            "sign-in hook (browser): no login URL: client exit: status 3",
            1,
        ),
        ("hang", deadline_text, 1),
        ("canary-only", deadline_text, 1),
        ("no-client", "sign-in hook (start): client did not start", 0),
        (
            "no-client-directory",
            "sign-in hook (start): client did not start: the client directory is gone",
            0,
        ),
        ("dotenv", &dotenv, 0),
        (
            "unset",
            "client home (CODEX_HOME): CONDUIT_STATE_DIR is unset",
            0,
        ),
        ("retired", RETIRED_REFUSAL, 0),
        ("v1", "sign-in hook: a .v1 Agent has none", 0),
    ] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        prepare(&c).unwrap();
        let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
        plant_login(&home);
        let login_path = home.join("fixture-login");
        let original = fs::read(&login_path).unwrap();

        let mut call = SignInCall::default();
        let mut script = json!({});
        let mut listener = None;
        match case {
            "device" => call.login = Some(json!({"name":"device"})),
            "unknown" => call.login = Some(json!({"name":"other"})),
            "malformed" => call.login = Some(json!("browser")),
            "return" => call.returned = true,
            "stale-state" => call = retry_login("agent-codex-login-1-1"),
            "port" => {
                listener = Some(
                    std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).unwrap(),
                )
            }
            "exit" => script = json!({"behavior":"exit:3"}),
            "hang" | "canary-only" => script = json!({"behavior":case}),
            "no-client" => c.program = parent.path().join("missing-client"),
            "no-client-directory" => c.directory = Some(parent.path().join("missing-directory")),
            "dotenv" => fs::write(home.join(".env"), AGENT_DOTENV).unwrap(),
            "unset" => c.state_dir = None,
            "v1" => c.v2 = false,
            _ => {}
        }
        write_login_script(&c.cwd, script);
        let mut session = Session::new(c.clone());
        if case == "retired" {
            let retired = session.retire().await.unwrap();
            assert_login_text(&retired);
            assert_eq!(retired, RETIRED);
        }
        let result = if matches!(case, "hang" | "canary-only") {
            session.sign_in_within(call, Duration::from_secs(3)).await
        } else {
            session.sign_in(call).await
        };
        drop(listener);
        let error = result.expect_err("expected a sign-in refusal").to_string();
        assert_login_text(&error);
        if matches!(case, "return" | "stale-state") {
            assert_eq!(error, expected, "{case}");
        } else {
            assert!(error.contains("PND-010"), "{case}");
            if expected.contains("PND-010") {
                assert_eq!(error, expected, "{case}");
            } else {
                assert_eq!(error, format!("{expected} (PND-010)"), "{case}");
            }
        }
        eprintln!("{case}: {error}");

        let log = account_log(&c.cwd, "logins.jsonl");
        assert_eq!(log.len(), count, "{case}");
        for entry in &log {
            assert_account_client_gone(entry);
        }
        if case == "retired" {
            assert!(entry_names(&home).is_empty());
        } else {
            assert!(fs::read(login_path).unwrap() == original);
        }
        assert_default_home(&default_home);
        assert!(!c.cwd.join("client-started").exists());
    }
}

#[test]
fn v2_sign_in_login_limit_fits_the_engine_wait() {
    assert!(LOGIN_LIMIT <= Duration::from_secs(600));
}

#[tokio::test]
async fn v2_sign_in_retry_after_retirement_keeps_its_text() {
    let parent = tempfile::tempdir().unwrap();
    let mut c = config(parent.path(), false, false);
    c.v2 = true;
    prepare(&c).unwrap();
    let mut session = Session::new(c);
    assert_eq!(session.retire().await.unwrap(), RETIRED);
    let retry = session
        .sign_in(retry_login("agent-codex-login-1-1"))
        .await
        .unwrap_err()
        .to_string();
    assert_eq!(retry, RETIRED_REFUSAL);
    let returned = session
        .sign_in(SignInCall {
            returned: true,
            ..Default::default()
        })
        .await
        .unwrap_err()
        .to_string();
    assert_eq!(returned, RETIRED_REFUSAL);
}

#[tokio::test]
async fn v2_sign_in_alone_adds_pnd_010_to_shared_refusals() {
    for refused_configuration in [true, false] {
        let parent = tempfile::tempdir().unwrap();
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        let expected = if refused_configuration {
            let error = format!(
                "invalid configuration: client home ({HOME_VARIABLE}): CONDUIT_STATE_DIR is unset"
            );
            c.state_dir = None;
            c.configuration_error = Some(error.clone());
            error
        } else {
            let outside = parent.path().join("outside");
            fs::create_dir(&outside).unwrap();
            symlink(&outside, c.state_dir.as_ref().unwrap().join(HOME_NAME)).unwrap();
            format!("client home ({HOME_VARIABLE}): the home resolves outside CONDUIT_STATE_DIR")
        };
        let mut session = Session::new(c);
        let sign_in = session
            .sign_in(SignInCall::default())
            .await
            .unwrap_err()
            .to_string();
        assert_eq!(sign_in, format!("{expected} (PND-010)"));
        assert_eq!(session.account().await.unwrap_err().to_string(), expected);

        let error = session.turn("HOME").await.unwrap_err();
        if refused_configuration {
            assert_eq!(error.to_string(), expected);
        } else {
            let failure = error.downcast_ref::<FailedTurn>().unwrap();
            assert_eq!(failure.cause, expected);
            assert!(!error.to_string().contains("PND-010"));
        }
    }
}

#[tokio::test]
async fn v2_sign_in_tool_through_the_wrapper() {
    use rmcp::{
        ClientLifecycleMode, ClientServiceExt, ServiceExt,
        model::{CallToolRequestParams, CallToolResponse, ProtocolVersion},
    };
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let _port = LOGIN_PORT.lock().await;
    let call = |arguments: Value| -> CallToolRequestParams {
        serde_json::from_value(json!({"name":"sign-in","arguments":arguments})).unwrap()
    };
    for (v2, failure_probes) in [(true, false), (false, false), (true, true)] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = v2;
        prepare(&c).unwrap();
        let mut command = tokio::process::Command::new(binary());
        command
            .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
            .env_clear()
            .envs(&c.env)
            .env("AGENT_ACCOUNT_SERVICES", "false")
            .current_dir(&c.cwd)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let state = c.state_dir.as_ref().unwrap();
        let home = state.join(HOME_NAME);
        if v2 {
            fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
            plant_login(&home);
            command
                .env("AGENT_TURN_DEADLINE_SECONDS", "60")
                .env("CONDUIT_STATE_DIR", state)
                .env(HOME_VARIABLE, parent.path().join("elsewhere"));
        } else {
            command.env("AGENT_PEER_MESSAGING", "false");
        }
        let mut child = command.spawn().unwrap();
        let mut stdout = BufReader::new(child.stdout.take().unwrap());
        let (reader, mut relay) = tokio::io::duplex(8192);
        let capture = tokio::spawn(async move {
            let mut wire = Vec::new();
            loop {
                let offset = wire.len();
                if stdout.read_until(b'\n', &mut wire).await.unwrap() == 0 {
                    return wire;
                }
                // Keep draining stdout after the rmcp client disconnects.
                let _ = relay.write_all(&wire[offset..]).await;
            }
        });
        let transport = (reader, child.stdin.take().unwrap());
        let service = if v2 {
            let lifecycle = ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            };
            ().serve_with_lifecycle(transport, lifecycle).await.unwrap()
        } else {
            ().serve(transport).await.unwrap()
        };
        let tools = service.list_all_tools().await.unwrap();
        let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
        let expected: &[_] = if v2 {
            &[
                "turn",
                "retire",
                "account",
                "sign-out",
                "sign-in",
                "new-conversation",
            ]
        } else {
            &["turn", "new-conversation"]
        };
        assert_eq!(names, expected);

        write_login_script(&c.cwd, json!({}));
        let invalid = if v2 { json!({"extra":true}) } else { json!({}) };
        let error = service
            .call_tool_once(call(invalid))
            .await
            .unwrap_err()
            .to_string();
        assert_login_text(&error);
        eprintln!("{error}");
        let mut pid = None;
        let mut expected_input = None;
        if failure_probes {
            for (behavior, text) in [
                (
                    "exit:3",
                    "sign-in hook (browser): no login URL: client exit: status 3 (PND-010)"
                        .to_owned(),
                ),
                (
                    "canary-only",
                    format!(
                        "sign-in hook (browser): no login URL within {} seconds (PND-010)",
                        LOGIN_URL_DEADLINE.as_secs()
                    ),
                ),
            ] {
                write_login_script(&c.cwd, json!({"behavior":behavior}));
                let response = tokio::time::timeout(
                    LOGIN_URL_DEADLINE + Duration::from_secs(10),
                    service.call_tool_once(call(json!({}))),
                )
                .await
                .unwrap()
                .unwrap();
                let CallToolResponse::Complete(result) = response else {
                    panic!("expected a complete sign-in failure");
                };
                let result = serde_json::to_value(result).unwrap();
                assert_login_text(&result.to_string());
                assert_eq!(
                    result,
                    json!({"isError":true,"resultType":"complete","content":[{"type":"text","text":text}]})
                );
                eprintln!("{text}");
            }
            let log = account_log(&c.cwd, "logins.jsonl");
            assert_eq!(log.len(), 2);
            for login in &log {
                wait_for_login_exit(login_pid(login)).await;
            }
        } else if v2 {
            write_login_script(&c.cwd, json!({}));
            let CallToolResponse::InputRequired(result) =
                service.call_tool_once(call(json!({}))).await.unwrap()
            else {
                panic!("sign-in did not require URL input");
            };
            let result = serde_json::to_value(result).unwrap();
            let wire = result.to_string();
            assert_login_text(&wire);
            let state = result["requestState"].as_str().unwrap();
            assert!(state.starts_with("agent-codex-login-"));
            let login = browser_login();
            let url = login_url(&login, &login_redirect(&login, login.ports[0]));
            expected_input = Some(json!({
                "resultType":"input_required",
                "inputRequests":{"sign-in":{"method":"elicitation/create","params":{
                    "mode":"url","message":LOGIN_MESSAGE,"url":url.as_str(),"elicitationId":"sign-in"
                }}},
                "requestState":state,
            }));
            eprintln!("sign-in returned input_required with URL and request state");
            let calls = [
                (
                    call(json!({})).with_request_state(state),
                    false,
                    LOGIN_WAITING,
                ),
                (
                    serde_json::from_value(json!({
                        "name":"sign-in","arguments":{},"_meta":{
                            "io.github.shrav-k.conduit/signInLogin":{"name":"device"}
                        }
                    }))
                    .unwrap(),
                    true,
                    DEVICE_REFUSAL,
                ),
                (
                    serde_json::from_value(json!({
                        "name":"sign-in","arguments":{},"_meta":{
                            "io.github.shrav-k.conduit/signInReturn":{
                                "value":"http://sentinel-return.invalid/?code=sentinel-code"
                            }
                        }
                    }))
                    .unwrap(),
                    true,
                    RETURN_REFUSAL,
                ),
            ];
            for (request, is_error, text) in calls {
                write_login_script(&c.cwd, json!({}));
                let CallToolResponse::Complete(result) =
                    service.call_tool_once(request).await.unwrap()
                else {
                    panic!("expected a complete sign-in response");
                };
                let result = serde_json::to_value(result).unwrap();
                assert_login_text(&result.to_string());
                assert_eq!(
                    result,
                    json!({"isError":is_error,"resultType":"complete","content":[{"type":"text","text":text}]})
                );
                eprintln!("{result}");
            }
            let log = account_log(&c.cwd, "logins.jsonl");
            assert_eq!(log.len(), 1);
            pid = Some(login_pid(&log[0]));
        } else {
            assert!(!c.cwd.join("logins.jsonl").exists());
        }

        service.cancel().await.unwrap();
        let output = child.wait_with_output().await.unwrap();
        assert!(output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_login_text(&stderr);
        let wire = String::from_utf8(capture.await.unwrap()).unwrap();
        assert_login_text(&wire);
        let results: Vec<Value> = wire
            .lines()
            .map(|line| {
                serde_json::from_str::<Value>(line)
                    .unwrap_or_else(|_| panic!("invalid JSON-RPC output"))
            })
            .map(|mut message| message["result"].take())
            .collect();
        let mut inputs = results
            .iter()
            .filter(|result| result["resultType"] == "input_required");
        assert!(
            inputs.next() == expected_input.as_ref(),
            "unexpected raw input-required result"
        );
        assert!(inputs.next().is_none(), "extra input-required result");
        // The complete sign-in replies, as written, hold no member but these three.
        let completes: Vec<_> = results
            .iter()
            .filter(|result| {
                result["resultType"] == "complete"
                    && result["content"][0]["text"]
                        .as_str()
                        .is_some_and(|text| text.starts_with("sign-in hook"))
            })
            .collect();
        let expected_completes = match (v2, failure_probes) {
            (true, true) => 2,
            (true, false) => 3,
            (false, _) => 0,
        };
        assert_eq!(completes.len(), expected_completes);
        for result in completes {
            let keys: Vec<_> = result
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(keys, ["content", "isError", "resultType"], "{result}");
        }
        for canary in [
            "synthetic-test-token",
            "synthetic-anthropic-key",
            "synthetic-openai-key",
        ] {
            assert!(!stderr.contains(canary));
            assert!(!wire.contains(canary));
        }
        if let Some(pid) = pid {
            wait_for_login_exit(pid).await;
        }
        assert_default_home(&default_home);
        assert_no_return_canary_in_files(parent.path());
    }
}

fn assert_no_return_canary_in_files(dir: &Path) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let kind = entry.file_type().unwrap();
        if kind.is_dir() {
            assert_no_return_canary_in_files(&entry.path());
        } else if kind.is_file() {
            let bytes = fs::read(entry.path()).unwrap();
            for canary in [b"sentinel-return".as_slice(), b"sentinel-code"] {
                assert!(!bytes.windows(canary.len()).any(|window| window == canary));
            }
        }
    }
}

#[tokio::test]
async fn v2_refresh_override_reaches_only_the_account_check() {
    let _port = LOGIN_PORT.lock().await;
    let parent = tempfile::tempdir().unwrap();
    let default_home = plant_default_home(parent.path());
    let mut c = config(parent.path(), false, false);
    c.v2 = true;
    c.env.insert(
        REFRESH_OVERRIDE_VARIABLE.into(),
        "http://template-refresh.invalid/oauth/token".into(),
    );
    c.env.insert(
        REVOKE_OVERRIDE_VARIABLE.into(),
        "http://template-revoke.invalid/oauth/revoke".into(),
    );
    prepare(&c).unwrap();
    let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
    fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
    plant_login(&home);
    let mut session = Session::new(c.clone());

    let launched = session.turn("HOME").await.unwrap();
    let turn: Value = serde_json::from_str(launched["text"].as_str().unwrap()).unwrap();
    write_account_script(&c.cwd, json!({"answer":chatgpt(json!(EXPECTED_ACCOUNT))}));
    let account = session.account().await.unwrap();
    assert_eq!(account.account.as_deref(), Some(EXPECTED_ACCOUNT));
    assert_login_text(&account.text);
    let request_state = assert_browser_start(&mut session, &c.cwd, SignInCall::default()).await;
    let sign_out = session.sign_out().await.unwrap();
    assert_eq!(sign_out, SIGNED_OUT);
    assert_login_text(&sign_out);
    let retired = session.retire().await.unwrap();
    assert_eq!(retired, RETIRED);
    assert_login_text(&retired);
    assert_default_home(&default_home);

    let logins = account_log(&c.cwd, "logins.jsonl");
    let logouts = recorded_logouts(&c.cwd);
    let reads = account_log(&c.cwd, "account-reads.jsonl");
    assert_eq!(logins.len(), 1);
    assert_eq!(logouts.len(), 2);
    assert_eq!(reads.len(), 1);
    let mut expected = hook_environment(&c, &home);
    expected.remove(REFRESH_OVERRIDE_VARIABLE);
    expected.remove(REVOKE_OVERRIDE_VARIABLE);
    let mut account_environment = expected.clone();
    account_environment.insert(REFRESH_OVERRIDE_VARIABLE.into(), REFRESH_BLOCKED.into());
    for (name, evidence, expected) in [
        ("turn", &turn, &expected),
        ("sign-in", &logins[0], &expected),
        ("sign-out", &logouts[0], &expected),
        ("retire", &logouts[1], &expected),
        ("account", &reads[0], &account_environment),
    ] {
        let wire = evidence.to_string();
        assert_login_text(&wire);
        assert!(
            !wire.contains("template-refresh"),
            "{name}: template refresh override reached the child environment"
        );
        assert!(
            !wire.contains("template-revoke"),
            "{name}: template revoke override reached the child environment"
        );
        assert_client_environment(evidence, expected);
        let refresh = evidence["environment"]
            .get(REFRESH_OVERRIDE_VARIABLE)
            .is_some();
        let revoke = evidence["environment"]
            .get(REVOKE_OVERRIDE_VARIABLE)
            .is_some();
        eprintln!("{name} held refresh override {refresh} and revoke override {revoke}");
    }
    assert!(request_state.starts_with("agent-codex-login-"));
    wait_for_login_exit(login_pid(&logins[0])).await;
}
