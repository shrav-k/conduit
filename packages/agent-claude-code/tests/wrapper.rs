use agent_adapter::sign_in::{
    CAPTURE_NAME, CAPTURE_VARIABLE, LOGIN_LIMIT, LOGIN_URL_UNUSABLE, SIGN_IN_MESSAGE,
    SIGN_IN_RETURN_REFUSED, SIGNED_OUT, SIGNED_OUT_CAUSE, STATUS_DID_NOT_START, STATUS_NO_ACCOUNT,
    STATUS_UNREADABLE, login_arguments, status_arguments,
};
use agent_adapter::{
    CLAUDE, CREDENTIAL_STORE, Config, FailedTurn, HOME_NAME, HOME_VARIABLE, RETIRED,
    RETIRED_REFUSAL, Session, arguments, header, helper_command, logout_arguments, prepare,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    os::unix::{
        ffi::OsStrExt,
        fs::{DirBuilderExt, PermissionsExt, symlink},
    },
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_agent-claude-code")
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
        mode: agent_adapter::Mode::Agent,
        login_directory: None,
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
async fn recorded_experiment_009_repeated_init_is_not_a_current_turn_catalog() {
    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    prepare(&c).unwrap();
    let mut session = Session::new(c);
    let first = session.turn("STALE_INIT").await.unwrap();
    assert_eq!(first["clientTools"]["status"], "reported");
    let second = session.turn("STALE_INIT").await.unwrap();
    assert_eq!(second["clientTools"]["status"], "unavailable");
    assert_eq!(
        second["clientTools"]["observations"][0]["freshness"],
        "session_init_unverified"
    );
    assert_eq!(
        second["clientTools"]["observations"][0]["reportedNames"],
        first["clientTools"]["observations"][0]["reportedNames"]
    );
    // The recorded server is named stub, not engine. Preserve the actual names and
    // assert the fixture includes the old files catalog and new notes use mid-turn.
    let recorded: Value = serde_json::from_str(include_str!("fixtures/stale-init.json")).unwrap();
    assert_eq!(
        recorded["turns"][0][0]["tools"],
        recorded["turns"][1][0]["tools"]
    );
    assert!(
        recorded["turns"][1][0]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .any(|n| n == "mcp__stub__files_read_v1")
    );
    assert!(recorded["turns"][1].as_array().unwrap().iter().any(|e| {
        e["content"].as_array().is_some_and(|items| {
            items
                .iter()
                .any(|i| i["name"] == "mcp__stub__notes_read_v1")
        })
    }));
    session.shutdown().await;
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
            let mut v2_settings = agent_adapter::settings(&c);
            v2_settings["autoMemoryEnabled"] = json!(false);
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
                assert_eq!(agent_adapter::settings(&c), v2_settings);
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
            eprintln!(
                "{version} turn number {} pid {}",
                evidence["number"], evidence["pid"]
            );
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

        let message = || json!({"message":"TEST"});
        let (first, first_evidence) = turn(call("turn", message()).await.unwrap());
        assert_eq!(first_evidence["number"], 1);

        assert_eq!(first["clientTools"]["status"], "reported");

        let (second, second_evidence) = turn(call("turn", message()).await.unwrap());
        assert_eq!(second_evidence["number"], 2);
        assert_eq!(second_evidence["pid"], first_evidence["pid"]);
        assert_eq!(second["clientTools"]["status"], "unavailable");

        reset(call("new-conversation", json!({})).await.unwrap(), 2, false);

        let (fresh, fresh_evidence) = turn(call("turn", message()).await.unwrap());
        assert_ne!(fresh_evidence["pid"], second_evidence["pid"]);
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
const ACCOUNT: &str = "Synthetic-Account";

fn write_signed_in_login(home: &Path, email: Option<&str>) {
    if let Some(email) = email {
        let login =
            json!({"token": LOGIN_TOKEN, "claims": {"account": LOGIN_CLAIM}, "email": email});
        fs::write(home.join("fixture-login"), login.to_string()).unwrap();
    } else {
        plant_login(home);
    }
}

fn hook_lines(path: &Path) -> Vec<Value> {
    let log = match fs::read_to_string(path) {
        Ok(log) => log,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Vec::new(),
        Err(error) => panic!("cannot read scripted status log: {error}"),
    };
    assert_no_hook_canary(&log);
    log.lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn assert_no_hook_canary(text: &str) {
    assert_no_login_canary(text);
    assert!(!text.contains(ACCOUNT));
}

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
            expected.insert("CLAUDE_CODE_DISABLE_AUTO_MEMORY".into(), "1".into());
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

#[tokio::test]
async fn v2_account_reports_only_the_status_account() {
    let parent = tempfile::tempdir().unwrap();
    let default_home = plant_default_home(parent.path());
    let mut c = config(parent.path(), false, false);
    c.v2 = true;
    c.env.insert("MCP_TOOL_TIMEOUT".into(), "10000".into());
    prepare(&c).unwrap();
    let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
    fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
    let login = home.join("fixture-login");
    let started = c.cwd.join("client-started");
    let mut session = Session::new(c.clone());
    session.turn("HOME").await.unwrap();
    fs::remove_file(&started).unwrap();

    assert_eq!(session.account().await.unwrap(), None);
    write_signed_in_login(&home, Some(ACCOUNT));
    let login_bytes = fs::read(&login).unwrap();
    assert_eq!(session.account().await.unwrap(), Some(ACCOUNT.into()));

    let lines = hook_lines(&c.cwd.join("statuses.jsonl"));
    assert_eq!(lines.len(), 2);
    let mut expected = c.env.clone();
    expected.remove("ANTHROPIC_API_KEY");
    expected.remove("OPENAI_API_KEY");
    expected.insert(
        HOME_VARIABLE.into(),
        home.canonicalize().unwrap().to_str().unwrap().into(),
    );
    for (line, entries) in lines.iter().zip([
        json!(["fixture-home-write"]),
        json!(["fixture-home-write", "fixture-login"]),
    ]) {
        assert_eq!(line["args"], json!(status_arguments()));
        assert_client_environment(line, &expected);
        assert_eq!(line["cwd"], json!(c.cwd));
        assert_eq!(line["entries"], entries);
        eprintln!("status args {} entries {}", line["args"], line["entries"]);
    }
    assert_eq!(entry_names(&home), ["fixture-home-write", "fixture-login"]);
    assert!(fs::read(&login).unwrap() == login_bytes);
    assert_eq!(home.metadata().unwrap().permissions().mode() & 0o777, 0o700);
    session.turn("HOME").await.unwrap();
    assert!(!started.exists());
    assert_default_home(&default_home);
    session.shutdown().await;
}

#[tokio::test]
async fn v2_account_error_paths_hold_no_canary() {
    let unset = "client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is unset";
    let invalid_home = format!("invalid configuration: {unset}");
    let invalid_effort =
        "invalid configuration: effort (AGENT_EFFORT): expected lowercase ASCII letters";
    for (case, expected, count) in [
        ("no-email", Err(STATUS_NO_ACCOUNT), 1),
        ("empty-email", Err(STATUS_NO_ACCOUNT), 1),
        ("not-json", Err(STATUS_UNREADABLE), 1),
        ("no-flag", Err(STATUS_UNREADABLE), 1),
        ("logged-in-exit-1", Err(STATUS_UNREADABLE), 1),
        ("logged-out-exit-0", Err(STATUS_UNREADABLE), 1),
        (
            "exit",
            Err("client status failed: client exit: status 2"),
            1,
        ),
        ("too-long", Err(STATUS_UNREADABLE), 1),
        (
            "deadline",
            Err("client status failed: no exit within 3 seconds"),
            1,
        ),
        ("no-client", Err(STATUS_DID_NOT_START), 0),
        (
            "outside-link",
            Err("client home (CLAUDE_CONFIG_DIR): the home resolves outside CONDUIT_STATE_DIR"),
            0,
        ),
        ("unset", Err(unset), 0),
        ("configuration-unset", Err(invalid_home.as_str()), 0),
        ("configuration", Ok(Some(ACCOUNT)), 1),
    ] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        c.env.insert("MCP_TOOL_TIMEOUT".into(), "10000".into());
        prepare(&c).unwrap();
        let mut home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
        if case == "outside-link" {
            let elsewhere = parent.path().join("elsewhere");
            fs::rename(&home, &elsewhere).unwrap();
            symlink(&elsewhere, &home).unwrap();
            home = elsewhere;
        }
        match case {
            "no-email" => write_signed_in_login(&home, None),
            "empty-email" => write_signed_in_login(&home, Some("")),
            "logged-out-exit-0" => {}
            _ => write_signed_in_login(&home, Some(ACCOUNT)),
        }
        match case {
            "not-json" => fs::write(
                c.cwd.join("status-output"),
                format!("Logged in as {ACCOUNT} token {LOGIN_TOKEN}"),
            ).unwrap(),
            "no-flag" => fs::write(
                c.cwd.join("status-output"),
                json!({"email": ACCOUNT, "accessToken": LOGIN_TOKEN}).to_string(),
            ).unwrap(),
            "logged-in-exit-1" => fs::write(c.cwd.join("status-exit"), b"1").unwrap(),
            "logged-out-exit-0" => fs::write(c.cwd.join("status-exit"), b"0").unwrap(),
            "exit" => fs::write(c.cwd.join("status-exit"), b"2").unwrap(),
            "too-long" => fs::write(
                c.cwd.join("status-output"),
                json!({"loggedIn": true, "email": ACCOUNT, "accessToken": LOGIN_TOKEN, "padding": "x".repeat(70_000)}).to_string(),
            ).unwrap(),
            "deadline" => fs::write(c.cwd.join("status-hang"), b"").unwrap(),
            "no-client" => c.program = parent.path().join("missing-client"),
            "unset" => c.state_dir = None,
            "configuration-unset" => {
                c.state_dir = None;
                c.configuration_error = Some(invalid_home.clone());
            }
            "configuration" => c.configuration_error = Some(invalid_effort.into()),
            _ => {}
        }
        let login = home.join("fixture-login");
        let login_bytes = login.exists().then(|| fs::read(&login).unwrap());
        let entries = entry_names(&home);
        let session = Session::new(c.clone());
        let result = if case == "deadline" {
            let start = std::time::Instant::now();
            let result = session.account_within(Duration::from_secs(3)).await;
            assert!(start.elapsed() < Duration::from_secs(10));
            result
        } else {
            session.account().await
        }
        .map_err(|error| error.to_string());
        if let Err(error) = &result {
            assert_no_hook_canary(error);
        }
        assert_eq!(
            result,
            expected
                .map(|account| account.map(str::to_owned))
                .map_err(str::to_owned),
            "{case}"
        );
        assert_eq!(
            hook_lines(&c.cwd.join("statuses.jsonl")).len(),
            count,
            "{case}"
        );
        assert!(!c.cwd.join("client-started").exists(), "{case}");
        assert_eq!(entry_names(&home), entries, "{case}");
        if let Some(bytes) = login_bytes {
            assert!(fs::read(&login).unwrap() == bytes, "{case}");
        }
        assert_default_home(&default_home);
        eprintln!("account {case}: {result:?}");

        if case == "too-long" {
            let output = c.cwd.join("status-output");
            let mut status = json!({"loggedIn": true, "email": ACCOUNT})
                .to_string()
                .into_bytes();
            // A truncated copy would still be valid JSON.
            status.resize(65_536, b' ');
            fs::write(&output, &status).unwrap();
            assert_eq!(session.account().await.unwrap().as_deref(), Some(ACCOUNT));
            status.push(b' ');
            fs::write(&output, &status).unwrap();
            let error = session.account().await.unwrap_err().to_string();
            assert_no_hook_canary(&error);
            assert_eq!(error, STATUS_UNREADABLE);
        }

        if case == "deadline" {
            use tokio::{io::AsyncReadExt, net::UnixListener, time::timeout};

            let mut session = session;
            session.turn("HOME").await.unwrap();
            let started = c.cwd.join("client-started");
            fs::remove_file(&started).unwrap();
            let listener = UnixListener::bind(c.cwd.join("status-descendant.sock")).unwrap();
            let (result, connection) = tokio::join!(
                session.account_within(Duration::from_secs(3)),
                timeout(Duration::from_secs(5), listener.accept()),
            );
            let (mut descendant, _) = connection.unwrap().unwrap();
            let error = result.unwrap_err().to_string();
            assert_no_hook_canary(&error);
            assert_eq!(error, "client status failed: no exit within 3 seconds");
            let read = timeout(Duration::from_secs(2), descendant.read(&mut [0])).await;
            assert_eq!(read.expect("status descendant kept running").unwrap(), 0);
            session.turn("HOME").await.unwrap();
            assert!(!started.exists(), "status timeout restarted the client");
            assert_default_home(&default_home);
            session.shutdown().await;
        }
    }
}

#[tokio::test]
async fn v2_sign_out_keeps_the_home_and_ends_a_running_conversation() {
    for running in [false, true] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        c.env.insert("MCP_TOOL_TIMEOUT".into(), "10000".into());
        prepare(&c).unwrap();
        let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
        let mut session = Session::new(c.clone());
        let started = c.cwd.join("client-started");
        if running {
            session.turn("HOME").await.unwrap();
            write_signed_in_login(&home, Some(ACCOUNT));
            fs::remove_file(&started).unwrap();
        } else {
            write_signed_in_login(&home, Some(ACCOUNT));
            fs::write(home.join("keep"), b"kept").unwrap();
        }

        assert_eq!(session.sign_out().await.unwrap(), SIGNED_OUT);
        let lines = recorded_logouts(&c.cwd);
        assert_eq!(lines.len(), 1);
        let line = &lines[0];
        assert_eq!(line["args"], json!(logout_arguments()));
        if running {
            assert_eq!(
                line["entries"],
                json!(["fixture-home-write", "fixture-login"])
            );
            assert_eq!(entry_names(&home), ["fixture-home-write"]);
            assert_eq!(
                session.turn("TEST").await.unwrap_err().to_string(),
                format!("session failed at turn 1: {SIGNED_OUT_CAUSE}; implicit restart refused")
            );
            assert!(!started.exists());
            assert_eq!(session.account().await.unwrap(), None);
        } else {
            let mut expected = c.env.clone();
            expected.remove("ANTHROPIC_API_KEY");
            expected.remove("OPENAI_API_KEY");
            expected.insert(
                HOME_VARIABLE.into(),
                home.canonicalize().unwrap().to_str().unwrap().into(),
            );
            assert_client_environment(line, &expected);
            assert_eq!(line["cwd"], json!(c.cwd));
            assert_eq!(line["entries"], json!(["fixture-login", "keep"]));
            assert_eq!(entry_names(&home), ["keep"]);
            assert_eq!(fs::read(home.join("keep")).unwrap(), b"kept");
            assert_eq!(home.metadata().unwrap().permissions().mode() & 0o777, 0o700);
            assert_eq!(session.account().await.unwrap(), None);
            session.turn("HOME").await.unwrap();
        }
        assert_default_home(&default_home);
        eprintln!("logout args {} entries {}", line["args"], line["entries"]);
        session.shutdown().await;
    }
}

#[tokio::test]
async fn v2_new_conversation_after_sign_out_reports_no_failed_turn() {
    let parent = tempfile::tempdir().unwrap();
    let mut c = config(parent.path(), false, false);
    c.v2 = true;
    c.env.insert("MCP_TOOL_TIMEOUT".into(), "10000".into());
    prepare(&c).unwrap();
    let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
    fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
    let mut session = Session::new(c.clone());
    session.turn("HOME").await.unwrap();
    write_signed_in_login(&home, Some(ACCOUNT));
    assert_eq!(session.sign_out().await.unwrap(), SIGNED_OUT);
    assert!(session.turn("TEST").await.is_err());
    let ended = &session.new_conversation().await.unwrap()["ended"];
    assert_eq!(ended["turns"], 1);
    assert_eq!(ended["failed"], false);
    session.shutdown().await;
}

#[tokio::test]
async fn v2_sign_out_error_paths_hold_no_canary() {
    let unset = "client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is unset";
    let invalid_home = format!("invalid configuration: {unset}");
    let invalid_effort =
        "invalid configuration: effort (AGENT_EFFORT): expected lowercase ASCII letters";
    for (case, expected, count) in [
        ("no-client", Err("client logout did not start"), 0),
        (
            "exit",
            Err("client logout failed: client exit: status 3"),
            1,
        ),
        (
            "deadline",
            Err("client logout failed: no exit within 3 seconds"),
            1,
        ),
        (
            "outside-link",
            Err("client home (CLAUDE_CONFIG_DIR): the home resolves outside CONDUIT_STATE_DIR"),
            0,
        ),
        ("unset", Err(unset), 0),
        ("configuration-unset", Err(invalid_home.as_str()), 0),
        ("configuration", Ok(SIGNED_OUT), 1),
    ] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let mut c = config(parent.path(), false, false);
        c.v2 = true;
        c.env.insert("MCP_TOOL_TIMEOUT".into(), "10000".into());
        prepare(&c).unwrap();
        let mut home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
        fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
        match case {
            "no-client" => c.program = parent.path().join("missing-client"),
            "exit" => fs::write(c.cwd.join("logout-exit"), b"3").unwrap(),
            "deadline" => fs::write(c.cwd.join("logout-hang"), b"").unwrap(),
            "outside-link" => {
                let elsewhere = parent.path().join("elsewhere");
                fs::rename(&home, &elsewhere).unwrap();
                symlink(&elsewhere, &home).unwrap();
                home = elsewhere;
            }
            "unset" => c.state_dir = None,
            "configuration-unset" => {
                c.state_dir = None;
                c.configuration_error = Some(invalid_home.clone());
            }
            "configuration" => c.configuration_error = Some(invalid_effort.into()),
            _ => unreachable!(),
        }
        write_signed_in_login(&home, Some(ACCOUNT));
        let login = home.join("fixture-login");
        let bytes = fs::read(&login).unwrap();
        let mut session = Session::new(c.clone());
        let result = if case == "deadline" {
            session.sign_out_within(Duration::from_secs(3)).await
        } else {
            session.sign_out().await
        }
        .map_err(|error| error.to_string());
        if let Err(error) = &result {
            assert_no_hook_canary(error);
        }
        assert_eq!(
            result,
            expected.map(str::to_owned).map_err(str::to_owned),
            "{case}"
        );
        assert_eq!(recorded_logouts(&c.cwd).len(), count, "{case}");
        if expected.is_err() {
            assert!(fs::read(&login).unwrap() == bytes, "{case}");
        } else {
            assert!(!login.exists(), "{case}");
        }
        assert!(!c.cwd.join("client-started").exists(), "{case}");
        assert_default_home(&default_home);
        eprintln!("sign-out {case}: {result:?}");

        if case == "exit" {
            fs::remove_file(c.cwd.join("logout-exit")).unwrap();
            assert_eq!(session.sign_out().await.unwrap(), SIGNED_OUT);
            assert_eq!(recorded_logouts(&c.cwd).len(), count + 1);
            assert!(entry_names(&home).is_empty());
        }
    }
}

#[tokio::test]
async fn v2_account_and_sign_out_tools_through_the_wrapper() {
    use rmcp::{
        ClientLifecycleMode, ClientServiceExt, ServiceExt,
        model::{CallToolRequestParams, ProtocolVersion},
    };
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let call = |name: &str, arguments: Value| -> CallToolRequestParams {
        serde_json::from_value(json!({"name": name, "arguments": arguments})).unwrap()
    };
    let account_answer = |account: Option<&str>| {
        let structured = json!({"account": account});
        json!({
            "isError": false, "resultType": "complete",
            "content": [{"type": "text", "text": structured.to_string()}],
            "structuredContent": structured,
        })
    };
    let parent = tempfile::tempdir().unwrap();
    let default_home = plant_default_home(parent.path());
    let c = config(parent.path(), false, false);
    let state = c.state_dir.as_ref().unwrap();
    let home = state.join(HOME_NAME);
    fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
    let mut child = tokio::process::Command::new(binary())
        .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
        .env_clear()
        .envs(&c.env)
        .env("AGENT_ACCOUNT_SERVICES", "false")
        .env("AGENT_TURN_DEADLINE_SECONDS", "60")
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

    for name in ["account", "sign-out"] {
        let invalid = service
            .call_tool(call(name, json!({"extra": true})))
            .await
            .unwrap_err();
        assert_no_hook_canary(&invalid.to_string());
    }
    let read_account = || async {
        let result = service.call_tool(call("account", json!({}))).await.unwrap();
        let wire = serde_json::to_value(result).unwrap();
        assert_no_login_canary(&wire.to_string());
        wire
    };
    assert_eq!(read_account().await, account_answer(None));
    write_signed_in_login(&home, Some(ACCOUNT));
    let account = read_account().await;
    assert_eq!(account, account_answer(Some(ACCOUNT)));

    let turn = service
        .call_tool(call("turn", json!({"message": "TEST"})))
        .await
        .unwrap();
    assert_eq!(turn.is_error, Some(false));

    let result = service
        .call_tool(call("sign-out", json!({})))
        .await
        .unwrap();
    let sign_out = serde_json::to_value(result).unwrap();
    assert_no_hook_canary(&sign_out.to_string());
    assert_eq!(
        sign_out,
        json!({
            "isError": false, "resultType": "complete",
            "content": [{"type": "text", "text": SIGNED_OUT}],
        })
    );
    let turn = service
        .call_tool(call("turn", json!({"message": "TEST"})))
        .await
        .unwrap();
    let turn = serde_json::to_value(turn).unwrap();
    assert_no_hook_canary(&turn.to_string());
    assert_eq!(
        turn,
        json!({
            "isError": true, "resultType": "complete",
            "content": [{
                "type": "text",
                "text": format!("session failed at turn 1: {SIGNED_OUT_CAUSE}; implicit restart refused"),
            }],
        })
    );
    assert_eq!(read_account().await, account_answer(None));
    let lines = hook_lines(&c.cwd.join("statuses.jsonl"));
    assert_eq!(lines.len(), 3);
    let home = home.canonicalize().unwrap();
    for line in lines {
        assert_eq!(line["args"], json!(status_arguments()));
        assert_eq!(line["environment"][HOME_VARIABLE], json!(home));
    }
    let logouts = recorded_logouts(&c.cwd);
    assert_eq!(logouts.len(), 1);
    assert_eq!(logouts[0]["args"], json!(logout_arguments()));
    assert_default_home(&default_home);

    write_signed_in_login(&home, Some(ACCOUNT));
    fs::write(c.cwd.join("status-exit"), b"2").unwrap();
    fs::write(c.cwd.join("logout-exit"), b"3").unwrap();
    for (name, expected) in [
        ("account", "client status failed: client exit: status 2"),
        ("sign-out", "client logout failed: client exit: status 3"),
    ] {
        let result = service.call_tool(call(name, json!({}))).await.unwrap();
        let wire = serde_json::to_value(result).unwrap();
        assert_no_hook_canary(&wire.to_string());
        assert_eq!(
            wire,
            json!({
                "isError": true, "resultType": "complete",
                "content": [{"type": "text", "text": expected}],
            })
        );
    }
    assert_eq!(hook_lines(&c.cwd.join("statuses.jsonl")).len(), 4);
    assert_eq!(recorded_logouts(&c.cwd).len(), 2);
    assert_default_home(&default_home);
    service.cancel().await.unwrap();
    let output = child.wait_with_output().await.unwrap();
    assert!(output.status.success());
    let stdout = raw_stdout.await.unwrap();
    assert_no_hook_canary(&String::from_utf8_lossy(&output.stderr));
    let stdout_text = String::from_utf8_lossy(&stdout);
    assert_no_login_canary(&stdout_text);
    assert_eq!(stdout_text.matches(ACCOUNT).count(), 2);
    for bytes in [&stdout, &output.stderr] {
        let text = String::from_utf8_lossy(bytes);
        for secret in [
            "synthetic-test-token",
            "synthetic-anthropic-key",
            "synthetic-openai-key",
        ] {
            assert!(!text.contains(secret));
        }
    }
    eprintln!("account tool {account}");
    eprintln!("sign-out tool {sign_out}");

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
    for name in ["account", "sign-out"] {
        let invalid = service.call_tool(call(name, json!({}))).await.unwrap_err();
        assert_no_hook_canary(&invalid.to_string());
    }
    assert!(!c.cwd.join("statuses.jsonl").exists());
    assert!(!c.cwd.join("logouts.jsonl").exists());
    service.cancel().await.unwrap();
    let output = child.wait_with_output().await.unwrap();
    assert!(output.status.success());
    assert_no_hook_canary(&String::from_utf8_lossy(&output.stderr));
}

/// What the scripted login prints beside the URL it hands over, and what no answer may hold.
const URL_CANARY: &str = "synthetic-url-canary";

/// claude-code.v2's stated logins, read from `contracts.yaml` (one JSON line each under `logins`).
fn stated_logins() -> Vec<Value> {
    let contracts =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("contracts.yaml")).unwrap();
    let document = contracts
        .split("\n---\n")
        .find(|document| document.contains("\n  name: claude-code.v2\n"))
        .unwrap();
    let logins: Vec<Value> = document
        .lines()
        .skip_while(|line| *line != "    logins:")
        .skip(1)
        .take_while(|line| line.starts_with("    - "))
        .map(|line| serde_json::from_str(&line["    - ".len()..]).unwrap())
        .collect();
    assert_eq!(logins.len(), 2);
    logins
}

/// The browser login, the one the sign-in hook starts when no login is asked for by name.
fn stated_login() -> Value {
    stated_logins()
        .into_iter()
        .find(|login| login["name"] == "browser")
        .unwrap()
}

fn encode(text: &str) -> String {
    text.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                char::from(b).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

fn decode(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = std::str::from_utf8(bytes.get(i + 1..i + 3)?).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// A stated login's URL in the form Claude Code 2.1.318's builder gives (the browser login's is
/// the one the scripted login hands over), built from the stated login; the login fills `{port}`,
/// `{state}` and `{challenge}`.
fn login_template(login: &Value) -> String {
    let redirect = &login["redirect"];
    let redirect_uri = match login["return"].as_str().unwrap() {
        "page" => encode(login["page"].as_str().unwrap()),
        _ => format!(
            "{}{{port}}{}",
            encode(&format!("http://{}:", redirect["host"].as_str().unwrap())),
            encode(redirect["path"].as_str().unwrap()),
        ),
    };
    format!(
        "{}?code=true&client_id={}&response_type=code&redirect_uri={}&scope={}&code_challenge={{challenge}}&code_challenge_method=S256&state={{state}}",
        login["authorization"].as_str().unwrap(),
        encode(login["clientId"].as_str().unwrap()),
        redirect_uri,
        encode("user:profile"),
    )
}

/// The query's parameters, each decoded; None where one does not decode.
fn parameters(url: &str) -> Option<Vec<(String, String)>> {
    let query = url.split_once('?').map_or("", |(_, query)| query);
    query
        .split('&')
        .map(|pair| {
            let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
            Some((decode(name)?, decode(value)?))
        })
        .collect()
}

/// IFC-049's checks of a URL for a redirect or page login, each failing check named.
fn ifc_049_failures(url: &str, login: &Value) -> Vec<&'static str> {
    let mut failures = Vec::new();
    let base = url.split_once('?').map_or(url, |(base, _)| base);
    if base != login["authorization"].as_str().unwrap() {
        failures.push("origin and path");
    }
    if base["https://".len().min(base.len())..]
        .split('/')
        .next()
        .is_some_and(|authority| authority.contains('\u{40}'))
    {
        failures.push("user information");
    }
    if url.contains('#') {
        failures.push("fragment");
    }
    let Some(pairs) = parameters(url) else {
        failures.push("query");
        return failures;
    };
    let names: std::collections::BTreeSet<_> = pairs.iter().map(|(name, _)| name).collect();
    if names.len() != pairs.len() {
        failures.push("repeated parameter");
    }
    let get = |name: &str| {
        pairs
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, value)| value.as_str())
    };
    if get("client_id") != login["clientId"].as_str() {
        failures.push("client_id");
    }
    if get("response_type") != Some("code") {
        failures.push("response_type");
    }
    let redirect = &login["redirect"];
    let port_ok = |port: u16| match &redirect["ports"] {
        Value::String(any) if any == "any" => port >= 1024,
        Value::Array(ports) => ports.contains(&json!(port)),
        _ => false,
    };
    // For page, redirect_uri is the stated page itself; for redirect, the stated loopback form.
    let redirect_ok = match login["return"].as_str() {
        Some("page") => get("redirect_uri").is_some_and(|uri| Some(uri) == login["page"].as_str()),
        _ => redirect["host"].as_str().is_some_and(|host| {
            get("redirect_uri")
                .and_then(|uri| uri.strip_prefix(&format!("http://{host}:")))
                .and_then(|rest| {
                    let (port, path) = rest.split_at(rest.find('/')?);
                    Some(port.parse::<u16>().is_ok_and(port_ok) && path == redirect["path"])
                })
                == Some(true)
        }),
    };
    if !redirect_ok {
        failures.push("redirect_uri");
    }
    if get("code_challenge_method") != Some("S256") {
        failures.push("code_challenge_method");
    }
    let base64url = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
    if !get("code_challenge").is_some_and(|v| v.len() == 43 && v.chars().all(base64url)) {
        failures.push("code_challenge");
    }
    let unreserved = |c: char| c.is_ascii_alphanumeric() || "-._~".contains(c);
    if !get("state").is_some_and(|v| v.len() >= 22 && v.chars().all(unreserved)) {
        failures.push("state");
    }
    failures
}

/// The port that a URL's redirect_uri names.
fn redirect_port(url: &str) -> u16 {
    let pairs = parameters(url).unwrap();
    let uri = &pairs.iter().find(|(n, _)| n == "redirect_uri").unwrap().1;
    let rest = uri.strip_prefix("http://").unwrap();
    let authority = rest.split('/').next().unwrap();
    authority.rsplit_once(':').unwrap().1.parse().unwrap()
}

fn assert_no_url_canary(text: &str) {
    assert_no_hook_canary(text);
    assert!(!text.contains(URL_CANARY));
}

fn assert_handed_over_url(directory: &Path, url: &str) {
    let handed_over = fs::read(directory.join("login-url.sha256")).unwrap();
    let returned = Sha256::digest(url.as_bytes());
    assert!(
        handed_over == returned[..],
        "sign-in changed the URL handed to its browser"
    );
}

fn sign_in_setup(parent: &Path) -> (Config, std::path::PathBuf, Value) {
    let login = stated_login();
    let mut c = config(parent, false, false);
    c.v2 = true;
    c.env.insert("MCP_TOOL_TIMEOUT".into(), "10000".into());
    prepare(&c).unwrap();
    let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
    fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
    fs::write(c.cwd.join("login-url"), login_template(&login)).unwrap();
    let result =
        json!({"token": LOGIN_TOKEN, "claims": {"account": LOGIN_CLAIM}, "email": ACCOUNT});
    fs::write(c.cwd.join("login-result"), result.to_string()).unwrap();
    (c, home, login)
}

fn process_gone(pid: i32) -> bool {
    (0..100).any(|_| {
        let gone = unsafe { libc::kill(pid, 0) } != 0;
        if !gone {
            std::thread::sleep(Duration::from_millis(20));
        }
        gone
    })
}

#[tokio::test]
async fn v2_new_conversation_keeps_a_sign_in_in_progress() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use tokio::time::{sleep, timeout};

    let parent = tempfile::tempdir().unwrap();
    let (c, _, login) = sign_in_setup(parent.path());
    let mut session = Session::new(c.clone());
    let url = session.sign_in().await.unwrap();
    let pid: i32 = fs::read_to_string(c.cwd.join("login.pid"))
        .unwrap()
        .parse()
        .unwrap();

    let refused = session.new_conversation().await.unwrap_err();
    assert_eq!(
        refused.to_string(),
        "new conversation refused: a sign-in is in progress in this Agent's own home; nothing was stopped; call again once the sign-in has ended"
    );
    assert_eq!(unsafe { libc::kill(pid, 0) }, 0);

    let port = redirect_port(&url);
    let mut browser = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    let pairs = parameters(&url).unwrap();
    let state = &pairs.iter().find(|(name, _)| name == "state").unwrap().1;
    let path = login["redirect"]["path"].as_str().unwrap();
    let host = login["redirect"]["host"].as_str().unwrap();
    let request = format!(
        "GET {path}?code=synthetic-code&state={state} HTTP/1.1\r\nHost: {host}:{port}\r\nConnection: close\r\n\r\n"
    );
    browser.write_all(request.as_bytes()).await.unwrap();
    let mut reply = Vec::new();
    timeout(Duration::from_secs(10), browser.read_to_end(&mut reply))
        .await
        .unwrap()
        .unwrap();
    assert!(reply.starts_with(b"HTTP/1.1 200 "));
    timeout(Duration::from_secs(10), async {
        while unsafe { libc::kill(pid, 0) } == 0 {
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("the scripted login did not exit");
    let result = session.new_conversation().await.unwrap();
    assert_eq!(result["conversation"], 2);
    assert_eq!(result["ended"]["turns"], 0);
    session.shutdown().await;
}

#[tokio::test]
async fn v2_sign_in_relays_the_handed_over_url_and_the_login_completes() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use tokio::time::{sleep, timeout};

    let parent = tempfile::tempdir().unwrap();
    let default_home = plant_default_home(parent.path());
    let (c, home, login) = sign_in_setup(parent.path());
    assert!(login["name"] == "browser");
    assert!(login["return"] == "redirect");
    assert!(login["redirect"]["ports"] == "any");
    assert_eq!(login["carried"].as_bool(), Some(false));

    let mut session = Session::new(c.clone());
    let url = session.sign_in().await.unwrap();
    let failures = ifc_049_failures(&url, &login);
    eprintln!("sign-in IFC-049 failures {failures:?}");
    assert!(failures.is_empty());
    let code_page = stated_logins()
        .into_iter()
        .find(|login| login["name"] == "code-page")
        .unwrap();
    let other = ifc_049_failures(&url, &code_page);
    eprintln!("sign-in IFC-049 failures against code-page {other:?}");
    assert_eq!(other, ["redirect_uri"]);
    assert_no_url_canary(&url);
    assert_handed_over_url(&c.cwd, &url);

    let lines = hook_lines(&c.cwd.join("logins.jsonl"));
    assert_eq!(lines.len(), 1);
    let line = &lines[0];
    assert_no_url_canary(&line.to_string());
    let capture = c.state_dir.as_ref().unwrap().join(CAPTURE_NAME);
    let mut expected = c.env.clone();
    expected.remove("ANTHROPIC_API_KEY");
    expected.remove("OPENAI_API_KEY");
    expected.insert(HOME_VARIABLE.into(), home.to_str().unwrap().into());
    expected.insert("BROWSER".into(), c.helper.to_str().unwrap().into());
    expected.insert(
        CAPTURE_VARIABLE.into(),
        capture.join("url").to_str().unwrap().into(),
    );
    assert_eq!(line["args"], json!(login_arguments()));
    assert_client_environment(line, &expected);
    assert_eq!(line["cwd"], json!(c.cwd));
    assert_eq!(line["entries"], json!([]));
    eprintln!("sign-in args {} entries {}", line["args"], line["entries"]);
    assert!(!capture.exists());
    assert!(!c.cwd.join("client-started").exists());
    assert_eq!(session.account().await.unwrap(), None);

    let port = redirect_port(&url);
    let mut browser = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    let pairs = parameters(&url).unwrap();
    let state = &pairs.iter().find(|(name, _)| name == "state").unwrap().1;
    let path = login["redirect"]["path"].as_str().unwrap();
    let host = login["redirect"]["host"].as_str().unwrap();
    let request = format!(
        "GET {path}?code=synthetic-code&state={state} HTTP/1.1\r\nHost: {host}:{port}\r\nConnection: close\r\n\r\n"
    );
    browser.write_all(request.as_bytes()).await.unwrap();
    let mut reply = Vec::new();
    timeout(Duration::from_secs(10), browser.read_to_end(&mut reply))
        .await
        .unwrap()
        .unwrap();
    assert!(reply.starts_with(b"HTTP/1.1 200 "));
    timeout(Duration::from_secs(10), async {
        loop {
            if session.account().await.unwrap().as_deref() == Some(ACCOUNT) {
                break;
            }
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("account did not report the scripted login");
    assert_eq!(entry_names(&home), ["fixture-login"]);
    assert_eq!(home.metadata().unwrap().permissions().mode() & 0o777, 0o700);
    assert!(!c.cwd.join("client-started").exists());
    assert_default_home(&default_home);
    session.shutdown().await;
}

#[tokio::test]
async fn v2_sign_in_error_paths_hold_no_canary() {
    use tokio::{io::AsyncReadExt, net::UnixListener, time::timeout};

    let unset = "client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is unset";
    let invalid_home = format!("invalid configuration: {unset}");
    let unset_failure = format!("{unset} (PND-010)");
    let invalid_home_failure = format!("{invalid_home} (PND-010)");
    let invalid_effort =
        "invalid configuration: effort (AGENT_EFFORT): expected lowercase ASCII letters";
    for (case, failure, count) in [
        (
            "no-client",
            Some("client login did not start: could not spawn client (PND-010)"),
            0,
        ),
        (
            "exit",
            Some("client login failed: client exit: status 3 (PND-010)"),
            1,
        ),
        (
            "exit-with-descendant",
            Some("client login failed: client exit: status 3 (PND-010)"),
            1,
        ),
        (
            "deadline",
            Some("client login failed: no URL within 3 seconds (PND-010)"),
            1,
        ),
        ("unusable", Some(LOGIN_URL_UNUSABLE), 1),
        ("fifo", Some(LOGIN_URL_UNUSABLE), 1),
        (
            "outside-link",
            Some(
                "client home (CLAUDE_CONFIG_DIR): the home resolves outside CONDUIT_STATE_DIR (PND-010)",
            ),
            0,
        ),
        (
            "capture-is-home",
            Some("client login did not start: capture directory overlaps client home (PND-010)"),
            0,
        ),
        (
            "capture-blocked",
            Some("client login did not start: could not create capture directory (PND-010)"),
            0,
        ),
        ("capture-stale", None, 1),
        ("unset", Some(unset_failure.as_str()), 0),
        (
            "configuration-unset",
            Some(invalid_home_failure.as_str()),
            0,
        ),
        ("configuration", None, 1),
        ("exit-after-url-with-descendant", None, 1),
    ] {
        let parent = tempfile::tempdir().unwrap();
        let default_home = plant_default_home(parent.path());
        let (mut c, mut home, login) = sign_in_setup(parent.path());
        let capture = c.state_dir.as_ref().unwrap().join(CAPTURE_NAME);
        match case {
            "no-client" => c.program = parent.path().join("missing-client"),
            "exit" | "exit-with-descendant" => fs::write(c.cwd.join("login-exit"), b"3").unwrap(),
            "deadline" => fs::write(c.cwd.join("login-silent"), b"").unwrap(),
            "fifo" => {
                fs::write(c.cwd.join("login-silent"), b"").unwrap();
                fs::write(c.cwd.join("login-fifo"), b"").unwrap();
            }
            "exit-after-url-with-descendant" => {
                fs::write(c.cwd.join("login-exit-after-url"), b"").unwrap()
            }
            "unusable" => {
                fs::write(c.cwd.join("login-url"), b"http://insecure.invalid/{port}").unwrap()
            }
            "outside-link" => {
                let elsewhere = parent.path().join("elsewhere");
                fs::rename(&home, &elsewhere).unwrap();
                symlink(&elsewhere, &home).unwrap();
                home = elsewhere;
            }
            "capture-is-home" => {
                fs::rename(&home, &capture).unwrap();
                symlink(&capture, &home).unwrap();
                home = capture.clone();
            }
            "capture-blocked" => fs::write(&capture, b"kept").unwrap(),
            "capture-stale" => {
                fs::create_dir(&capture).unwrap();
                fs::write(capture.join("keep"), b"kept").unwrap();
            }
            "unset" => c.state_dir = None,
            "configuration-unset" => {
                c.state_dir = None;
                c.configuration_error = Some(invalid_home.clone());
            }
            "configuration" => c.configuration_error = Some(invalid_effort.into()),
            _ => unreachable!(),
        }
        write_signed_in_login(&home, Some(ACCOUNT));
        let stored = home.join("fixture-login");
        let bytes = fs::read(&stored).unwrap();
        let mut session = Session::new(c.clone());
        let with_descendant = case.ends_with("-with-descendant");
        let observer = if with_descendant {
            session.turn("HOME").await.unwrap();
            fs::remove_file(c.cwd.join("client-started")).unwrap();
            Some(UnixListener::bind(c.cwd.join("login-descendant.sock")).unwrap())
        } else {
            None
        };
        let started = std::time::Instant::now();
        let answer = if case == "deadline" {
            session.sign_in_within(Duration::from_secs(3)).await
        } else {
            session.sign_in().await
        };
        assert!(started.elapsed() < Duration::from_secs(10), "{case}");
        match (answer, failure) {
            (Err(error), Some(expected)) => {
                let text = error.to_string();
                assert_no_url_canary(&text);
                assert!(text.contains("PND-010"), "{case}");
                assert!(text == expected, "{case}: unexpected sign-in error");
                eprintln!("sign-in {case}: {text}");
            }
            (Ok(url), None) => {
                assert_no_url_canary(&url);
                assert!(ifc_049_failures(&url, &login).is_empty());
                eprintln!("sign-in {case}: URL passed IFC-049");
            }
            _ => panic!("{case}: unexpected sign-in result"),
        }
        if let Some(observer) = observer {
            let (mut descendant, _) = timeout(Duration::from_secs(2), observer.accept())
                .await
                .expect("login descendant never connected")
                .unwrap();
            let read = timeout(Duration::from_secs(5), descendant.read(&mut [0])).await;
            assert_eq!(read.expect("login descendant kept running").unwrap(), 0);
            session.turn("HOME").await.unwrap();
            session.shutdown().await;
        }
        let lines = hook_lines(&c.cwd.join("logins.jsonl"));
        assert_eq!(lines.len(), count, "{case}");
        for line in lines {
            assert_no_url_canary(&line.to_string());
        }
        if count > 0 && failure.is_some() {
            let pid = fs::read_to_string(c.cwd.join("login.pid")).unwrap();
            assert!(process_gone(pid.parse().unwrap()), "{case}");
        }
        match case {
            "capture-is-home" => assert_eq!(entry_names(&capture), ["fixture-login"]),
            "capture-blocked" => assert_eq!(fs::read(&capture).unwrap(), b"kept"),
            _ => assert!(!capture.exists(), "{case}"),
        }
        assert!(!c.cwd.join("client-started").exists(), "{case}");
        assert!(fs::read(&stored).unwrap() == bytes, "{case}");
        assert_default_home(&default_home);
        if failure.is_none() && !with_descendant {
            session.shutdown().await;
        }
    }
}

#[test]
fn v2_sign_in_login_limit_fits_the_engine_wait() {
    assert!(LOGIN_LIMIT <= Duration::from_secs(600));
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
        let sign_in = session.sign_in().await.unwrap_err().to_string();
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
        ClientHandler, ClientLifecycleMode, ClientServiceExt, ServiceError, ServiceExt,
        model::{
            CallToolRequest, CallToolRequestParams, CallToolResponse, ClientCapabilities,
            ClientConfig, ClientRequest, Implementation, ProtocolVersion, RequestMetaObject,
        },
        service::PeerRequestOptions,
    };
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpStream,
    };

    let call = |arguments: Value| -> CallToolRequestParams {
        serde_json::from_value(json!({"name": "sign-in", "arguments": arguments})).unwrap()
    };
    struct UrlClient;
    impl ClientHandler for UrlClient {
        fn get_info(&self) -> ClientConfig {
            let capabilities = serde_json::from_value(json!({"elicitation": {"url": {}}})).unwrap();
            ClientConfig::new(capabilities, Implementation::from_build_env())
        }
    }
    let parent = tempfile::tempdir().unwrap();
    let default_home = plant_default_home(parent.path());
    let (c, home, login) = sign_in_setup(parent.path());
    let mut child = tokio::process::Command::new(binary())
        .args(["--client-program", env!("CARGO_BIN_EXE_scripted-client")])
        .env_clear()
        .envs(&c.env)
        .env("AGENT_ACCOUNT_SERVICES", "false")
        .env("AGENT_TURN_DEADLINE_SECONDS", "60")
        .env("CONDUIT_STATE_DIR", c.state_dir.as_ref().unwrap())
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
    let service = UrlClient
        .serve_with_lifecycle(transport, lifecycle)
        .await
        .unwrap();
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

    let log = c.cwd.join("logins.jsonl");
    let Err(error) = service.call_tool_once(call(json!({"extra": true}))).await else {
        panic!("sign-in accepted an argument");
    };
    assert_no_url_canary(&error.to_string());
    assert!(!log.exists());

    let mut meta = RequestMetaObject::new();
    meta.set_client_capabilities(ClientCapabilities::default());
    let incapable = service
        .send_request_with_option(
            ClientRequest::CallToolRequest(CallToolRequest::new(call(json!({})))),
            PeerRequestOptions::no_options().with_meta(meta),
        )
        .await
        .unwrap();
    let Err(ServiceError::McpError(error)) = incapable.await_response().await else {
        panic!("sign-in accepted a request without URL elicitation");
    };
    assert_no_url_canary(&error.to_string());
    assert_eq!(error.code.0, -32021);
    assert_eq!(
        error.data,
        Some(json!({"requiredCapabilities": {"elicitation": {"url": {}}}}))
    );
    assert!(!log.exists());

    let Ok(CallToolResponse::InputRequired(response)) =
        service.call_tool_once(call(json!({}))).await
    else {
        panic!("sign-in did not request URL input");
    };
    let wire = serde_json::to_value(response).unwrap();
    assert_no_url_canary(&wire.to_string());
    assert!(wire["resultType"] == "input_required");
    assert!(!wire.as_object().unwrap().contains_key("requestState"));
    assert!(!wire.as_object().unwrap().contains_key("_meta"));
    let requests = wire["inputRequests"].as_object().unwrap();
    assert_eq!(requests.len(), 1);
    let (key, request) = requests.iter().next().unwrap();
    let params = &request["params"];
    assert!(key == "login");
    assert!(request["method"] == "elicitation/create");
    assert!(params["mode"] == "url");
    assert!(params["message"] == SIGN_IN_MESSAGE);
    let url = params["url"].as_str().unwrap();
    assert!(ifc_049_failures(url, &login).is_empty());
    assert_handed_over_url(&c.cwd, url);
    let port = redirect_port(url);
    let waiting = TcpStream::connect(("127.0.0.1", port)).await.unwrap();

    let mut retry = serde_json::to_value(call(json!({}))).unwrap();
    retry["requestState"] = json!("synthetic-request-state");
    let Ok(CallToolResponse::Complete(response)) = service
        .call_tool_once(serde_json::from_value(retry).unwrap())
        .await
    else {
        panic!("sign-in did not return a complete refusal");
    };
    let refusal = serde_json::to_value(response).unwrap();
    assert_no_url_canary(&refusal.to_string());
    assert!(
        refusal
            == json!({
                "isError": true, "resultType": "complete",
                "content": [{"type": "text", "text": SIGN_IN_RETURN_REFUSED}],
            })
    );
    let lines = hook_lines(&log);
    assert_eq!(lines.len(), 1);
    let line = &lines[0];
    assert_no_url_canary(&line.to_string());
    assert_eq!(line["args"], json!(login_arguments()));
    assert!(line["environment"]["BROWSER"] == json!(Path::new(binary()).canonicalize().unwrap()));
    assert!(line["environment"][HOME_VARIABLE] == json!(home.canonicalize().unwrap()));
    assert_default_home(&default_home);

    service.cancel().await.unwrap();
    let output = child.wait_with_output().await.unwrap();
    assert!(output.status.success());
    let error = TcpStream::connect(("127.0.0.1", port)).await.unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::ConnectionRefused);
    drop(waiting);
    let stdout = raw_stdout.await.unwrap();
    let stdout_text = String::from_utf8_lossy(&stdout);
    let stderr_text = String::from_utf8_lossy(&output.stderr);
    assert!(!stdout_text.contains(URL_CANARY));
    assert_no_hook_canary(&stdout_text);
    assert_no_url_canary(&stderr_text);
    for text in [&stdout_text, &stderr_text] {
        for secret in [
            "synthetic-test-token",
            "synthetic-anthropic-key",
            "synthetic-openai-key",
        ] {
            assert!(!text.contains(secret));
        }
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
    let Err(error) = service.call_tool_once(call(json!({}))).await else {
        panic!("the .v1 wrapper served the sign-in tool");
    };
    assert_no_url_canary(&error.to_string());
    assert!(!c.cwd.join("logins.jsonl").exists());
    service.cancel().await.unwrap();
    let output = child.wait_with_output().await.unwrap();
    assert!(output.status.success());
    assert_no_url_canary(&String::from_utf8_lossy(&output.stderr));

    eprintln!("sign-in tool {key} elicitation/create url {SIGN_IN_MESSAGE}");
}

#[test]
fn v2_stated_logins_each_match_one_url() {
    let logins = stated_logins();
    let names: Vec<_> = logins.iter().map(|login| login["name"].clone()).collect();
    assert_eq!(names, [json!("browser"), json!("code-page")]);
    let (browser, code_page) = (&logins[0], &logins[1]);
    assert!(browser["return"] == "redirect");
    assert!(code_page["return"] == "page");
    assert_eq!(code_page["carried"].as_bool(), Some(false));
    assert!(code_page.get("redirect").is_none() && code_page.get("issuer").is_none());
    assert!(
        code_page["authorization"] == browser["authorization"],
        "authorization differs between the stated logins"
    );
    assert!(
        code_page["clientId"] == browser["clientId"],
        "clientId differs between the stated logins"
    );
    assert!(code_page["page"].as_str().unwrap().starts_with("https://"));
    // Claude Code 2.1.318 builds both URLs from one state and one challenge.
    let state = "synthetic-state-0123456789abcdef";
    let challenge = "synthetic-challenge-0123456789abcdefghijklm";
    assert_eq!(challenge.len(), 43);
    let urls: Vec<String> = logins
        .iter()
        .map(|login| {
            login_template(login)
                .replace("{port}", "49152")
                .replace("{state}", state)
                .replace("{challenge}", challenge)
        })
        .collect();
    for (i, url) in urls.iter().enumerate() {
        for (j, login) in logins.iter().enumerate() {
            let failures = ifc_049_failures(url, login);
            eprintln!(
                "URL of {} against {}: {failures:?}",
                logins[i]["name"], login["name"]
            );
            if i == j {
                assert!(failures.is_empty());
            } else {
                assert_eq!(failures, ["redirect_uri"]);
            }
        }
    }
}

struct LoginTestWrapper {
    child: tokio::process::Child,
    input: tokio::process::ChildStdin,
    output: tokio::io::Lines<tokio::io::BufReader<tokio::process::ChildStdout>>,
    next_id: u64,
}

impl LoginTestWrapper {
    async fn start(c: &Config, mode: Option<&str>, directory: Option<&Path>) -> Self {
        use tokio::io::AsyncBufReadExt;
        let mut child = tokio::process::Command::new(binary())
            .args(["--client-program"])
            .arg(&c.program)
            .args(mode)
            .env_clear()
            .envs(&c.env)
            .env("AGENT_ACCOUNT_SERVICES", "false")
            .env("AGENT_TURN_DEADLINE_SECONDS", "10")
            .env("CONDUIT_STATE_DIR", c.state_dir.as_ref().unwrap())
            .envs(directory.map(|dir| ("AGENT_LOGIN_DIRECTORY", dir)))
            .current_dir(&c.cwd)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let input = child.stdin.take().unwrap();
        let output = tokio::io::BufReader::new(child.stdout.take().unwrap()).lines();
        let mut wrapper = Self {
            child,
            input,
            output,
            next_id: 1,
        };
        let reply = wrapper.request("server/discover", json!({})).await;
        assert!(
            reply["result"]["supportedVersions"]
                .as_array()
                .is_some_and(|versions| versions.iter().any(|version| version == "2026-07-28")),
            "wrapper discovery did not offer 2026-07-28"
        );
        wrapper
    }

    async fn send(&mut self, message: Value) {
        use tokio::io::AsyncWriteExt;
        self.input
            .write_all(format!("{message}\n").as_bytes())
            .await
            .unwrap();
    }

    async fn request(&mut self, method: &str, mut params: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        params["_meta"] = json!({
            "io.modelcontextprotocol/protocolVersion": "2026-07-28",
            "io.modelcontextprotocol/clientCapabilities": {"elicitation": {"url": {}}}
        });
        self.send(json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}))
            .await;
        tokio::time::timeout(Duration::from_secs(30), async {
            loop {
                let line = self
                    .output
                    .next_line()
                    .await
                    .unwrap()
                    .expect("wrapper closed without a reply");
                let reply: Value = serde_json::from_str(&line).unwrap();
                if reply["id"] == id {
                    return reply;
                }
            }
        })
        .await
        .expect("wrapper reply timed out")
    }

    async fn call(&mut self, name: &str, arguments: Value) -> Value {
        let mut reply = self
            .request("tools/call", json!({"name": name, "arguments": arguments}))
            .await;
        assert!(
            reply.get("result").is_some(),
            "{name} returned no result (error code {})",
            reply["error"]["code"]
        );
        reply["result"].take()
    }

    async fn turn(&mut self, message: &str) -> Value {
        self.call("turn", json!({"message": message})).await
    }

    async fn close(self) {
        let Self {
            mut child, input, ..
        } = self;
        drop(input);
        assert!(child.wait().await.unwrap().success());
    }
}

fn account_home(root: &Path) -> std::path::PathBuf {
    let home = root.join("account-home");
    fs::create_dir_all(&home).unwrap();
    home.canonicalize().unwrap()
}

#[tokio::test]
async fn login_resource_hooks_keep_the_account_home() {
    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    fs::write(c.cwd.join("login-url"), login_template(&stated_login())).unwrap();
    let home = c.state_dir.as_ref().unwrap().join("account-home");
    let mut wrapper = LoginTestWrapper::start(&c, Some("--login-resource"), None).await;
    let reply = wrapper.request("tools/list", json!({})).await;
    let mut tools: Vec<_> = reply["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect();
    tools.sort_unstable();
    assert_eq!(tools, ["account", "retire", "sign-in", "sign-out"]);
    let status = wrapper.call("account", json!({})).await;
    assert_eq!(status["isError"], false);
    assert!(status["structuredContent"]["account"].is_null());
    assert_eq!(home.metadata().unwrap().permissions().mode() & 0o777, 0o700);
    let sign_in = wrapper.call("sign-in", json!({})).await;
    assert_eq!(sign_in["resultType"], "input_required");
    assert_eq!(
        wrapper.call("sign-out", json!({})).await["content"][0]["text"],
        SIGNED_OUT
    );
    fs::write(home.join("marker"), []).unwrap();
    assert_eq!(
        wrapper.call("retire", json!({})).await["content"][0]["text"],
        RETIRED
    );
    assert!(entry_names(&home).is_empty());
    for (log, operation, count) in [
        ("statuses.jsonl", "status", 1),
        ("logins.jsonl", "login", 1),
        ("logouts.jsonl", "logout", 2),
    ] {
        let rows = hook_lines(&c.cwd.join(log));
        assert_eq!(rows.len(), count);
        for row in rows {
            assert_eq!(row["args"], json!(["auth", operation]));
            assert!(row["environment"][HOME_VARIABLE] == json!(home));
        }
    }
    assert!(!c.state_dir.as_ref().unwrap().join(HOME_NAME).exists());
    wrapper.close().await;
}

fn account_home_evidence(reply: &Value) -> Value {
    assert_eq!(reply["isError"], false);
    serde_json::from_str(reply["structuredContent"]["text"].as_str().unwrap()).unwrap()
}

#[tokio::test]
async fn v3_runs_in_the_account_home() {
    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    let login = parent.path().join("login");
    let home = account_home(&login);
    let mut wrapper = LoginTestWrapper::start(&c, Some("--shared-login"), Some(&login)).await;
    for turn in 1..=2 {
        let evidence = account_home_evidence(&wrapper.turn("ACCOUNT-HOME").await);
        assert_eq!(evidence["home"], json!(home));
        assert_eq!(evidence["number"], turn);
        let args = evidence["args"].as_array().unwrap();
        for (flag, argument) in [
            ("--setting-sources", json!("")),
            ("--settings", json!(c.cwd.join("settings.json"))),
            ("--mcp-config", json!(c.cwd.join("mcp.json"))),
        ] {
            let at = args.iter().position(|arg| arg == flag).unwrap();
            assert_eq!(args[at + 1], argument);
        }
        assert!(args.iter().any(|arg| arg == "--no-session-persistence"));
        assert!(args.iter().any(|arg| arg == "--strict-mcp-config"));
        assert!(entry_names(&home).is_empty());
    }
    wrapper.close().await;

    let started = c.cwd.join("client-started");
    fs::remove_file(&started).unwrap();
    fs::remove_dir(&home).unwrap();
    let outside = parent.path().join("outside");
    fs::create_dir(&outside).unwrap();
    let non_unicode = login.join(std::ffi::OsStr::from_bytes(b"non-unicode-\xff"));
    for (directory, destination) in [
        (Path::new("relative"), None),
        (login.as_path(), Some(outside.as_path())),
        (non_unicode.as_path(), None),
    ] {
        if let Some(destination) = destination {
            symlink(destination, &home).unwrap();
        }
        let mut wrapper =
            LoginTestWrapper::start(&c, Some("--shared-login"), Some(directory)).await;
        let reply = wrapper.turn("ACCOUNT-HOME").await;
        assert_eq!(reply["isError"], true);
        assert!(
            reply["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains("login home (CLAUDE_CONFIG_DIR):")
        );
        assert!(!started.exists());
        wrapper.close().await;
        if destination.is_some() {
            fs::remove_file(&home).unwrap();
        }
    }
}

#[tokio::test]
async fn auto_memory_stays_off() {
    for shared in [false, true] {
        let parent = tempfile::tempdir().unwrap();
        let c = config(parent.path(), false, false);
        let login = parent.path().join("login");
        let home = if shared {
            account_home(&login)
        } else {
            let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
            fs::create_dir(&home).unwrap();
            home.canonicalize().unwrap()
        };
        let settings = home.join("settings.json");
        fs::write(&settings, r#"{"autoMemoryEnabled": true}"#).unwrap();
        let mut wrapper = LoginTestWrapper::start(
            &c,
            shared.then_some("--shared-login"),
            shared.then_some(login.as_path()),
        )
        .await;
        for number in [1, 2, 1] {
            let evidence = account_home_evidence(&wrapper.turn("ACCOUNT-HOME").await);
            assert_eq!(evidence["number"], number);
            assert_eq!(
                evidence["settings"]["autoMemoryEnabled"], false,
                "missing auto memory setting"
            );
            assert_eq!(
                evidence["autoMemoryDisabled"], true,
                "missing auto memory environment switch"
            );
            assert!(evidence["homeMemoryEnabled"] == true);
            assert!(!home.join("projects").exists());
            if number == 2 {
                assert_eq!(
                    wrapper.call("new-conversation", json!({})).await["isError"],
                    false
                );
                fs::write(
                    c.cwd.join("settings.json"),
                    r#"{"autoMemoryEnabled": true}"#,
                )
                .unwrap();
            }
        }
        wrapper.close().await;
    }
}

#[tokio::test]
async fn v3_retire_never_logs_out_its_login_resource() {
    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    let login = parent.path().join("login");
    let home = account_home(&login);
    fs::write(home.join("marker"), []).unwrap();
    let mut wrapper = LoginTestWrapper::start(&c, Some("--shared-login"), Some(&login)).await;
    let evidence = account_home_evidence(&wrapper.turn("ACCOUNT-HOME").await);
    let pid = i32::try_from(evidence["pid"].as_i64().unwrap()).unwrap();
    let retired = wrapper.call("retire", json!({})).await;
    assert_eq!(
        retired["content"][0]["text"],
        "client stopped; its login and configuration directory were left to its login Resource"
    );
    assert!(process_gone(pid));
    assert!(!c.cwd.join("logouts.jsonl").exists());
    assert_eq!(entry_names(&home), ["marker"]);
    let next = wrapper.turn("ACCOUNT-HOME").await;
    assert_eq!(next["isError"], true);
    assert_eq!(next["content"][0]["text"], RETIRED_REFUSAL);
    let tools = wrapper.request("tools/list", json!({})).await;
    let mut names: Vec<_> = tools["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect();
    names.sort_unstable();
    assert_eq!(names, ["new-conversation", "retire", "turn"]);
    wrapper.close().await;
}

#[tokio::test]
async fn v3_lapsed_login_is_reported_as_the_resources() {
    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    let login = parent.path().join("login");
    fs::create_dir(&login).unwrap();
    let mut wrapper = LoginTestWrapper::start(&c, Some("--shared-login"), Some(&login)).await;
    let lapse = account_home_lapse();
    assert_eq!(wrapper.turn("ACCOUNT-HOME").await, lapse);
    assert!(!c.cwd.join("client-started").exists());
    account_home(&login);
    let evidence = account_home_evidence(&wrapper.turn("ACCOUNT-HOME").await);
    assert_eq!(evidence["number"], 1);
    assert_eq!(wrapper.turn("ACCOUNT-HOME:authentication").await, lapse);
    let refused = wrapper.turn("ACCOUNT-HOME").await;
    assert!(
        refused["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("implicit restart refused")
    );
    let conversation = wrapper.call("new-conversation", json!({})).await;
    assert_eq!(conversation["structuredContent"]["ended"]["turns"], 2);
    let usage_limit = wrapper.turn("ACCOUNT-HOME:usage limit").await;
    assert_eq!(usage_limit["isError"], true);
    assert!(usage_limit.get("inputRequests").is_none());
    wrapper.close().await;

    let mut v2 = LoginTestWrapper::start(&c, None, None).await;
    let reply = v2.turn("ACCOUNT-HOME:authentication").await;
    assert_eq!(reply["isError"], true);
    assert!(reply.get("inputRequests").is_none());
    let failed: Value =
        serde_json::from_str(reply["content"][1]["text"].as_str().unwrap()).unwrap();
    assert_eq!(failed["failedTurn"]["class"], "authentication");
    v2.close().await;
}

fn account_home_lapse() -> Value {
    json!({"resultType":"input_required","inputRequests":{"sign-in":{
        "method":"elicitation/create","params":{
            "mode":"url","message":"The login Resource's Claude Code login is missing or has lapsed.",
            "url":"https://code.claude.com/docs/en/authentication"
        }
    }}})
}

#[tokio::test]
async fn v3_missing_home_between_turns_leaves_the_conversation() {
    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    let login = parent.path().join("login");
    let home = account_home(&login);
    let mut wrapper = LoginTestWrapper::start(&c, Some("--shared-login"), Some(&login)).await;
    let first = account_home_evidence(&wrapper.turn("ACCOUNT-HOME").await);
    assert_eq!(first["number"], 1);
    fs::remove_dir(&home).unwrap();
    assert!(
        wrapper.turn("ACCOUNT-HOME").await == account_home_lapse(),
        "the resumed turn reached the client after the shared home was removed"
    );
    account_home(&login);
    let next = account_home_evidence(&wrapper.turn("ACCOUNT-HOME").await);
    assert_eq!(next["number"], 2);
    assert_eq!(next["pid"], first["pid"]);
    let conversation = wrapper.call("new-conversation", json!({})).await;
    assert_eq!(conversation["structuredContent"]["ended"]["turns"], 2);
    wrapper.close().await;
}

#[tokio::test]
async fn v3_relaunch_does_not_follow_run_config_links() {
    for name in ["settings.json", "mcp.json"] {
        let parent = tempfile::tempdir().unwrap();
        let c = config(parent.path(), false, false);
        let login = parent.path().join("login");
        let home = account_home(&login);
        let target = home.join("benign.txt");
        let marker = b"shared fixture contents";
        fs::write(&target, marker).unwrap();
        let mut wrapper = LoginTestWrapper::start(&c, Some("--shared-login"), Some(&login)).await;
        account_home_evidence(&wrapper.turn("ACCOUNT-HOME").await);
        assert_eq!(
            wrapper.call("new-conversation", json!({})).await["isError"],
            false
        );
        let started = c.cwd.join("client-started");
        fs::remove_file(&started).unwrap();
        let run_file = c.cwd.join(name);
        fs::remove_file(&run_file).unwrap();
        symlink(&target, &run_file).unwrap();

        let reply = wrapper.turn("ACCOUNT-HOME").await;
        assert!(
            fs::read(&target).unwrap() == marker,
            "{name} overwrote a shared file"
        );
        assert_eq!(reply["isError"], true);
        assert!(
            reply["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains("cannot prepare run files")
        );
        assert!(!started.exists());
        assert_eq!(fs::read_link(&run_file).unwrap(), target);
        wrapper.close().await;
    }
}

#[tokio::test]
async fn v3_engine_mcp_authentication_does_not_report_login_lapse() {
    let parent = tempfile::tempdir().unwrap();
    let c = config(parent.path(), false, false);
    let login = parent.path().join("login");
    account_home(&login);
    let mut wrapper = LoginTestWrapper::start(&c, Some("--shared-login"), Some(&login)).await;
    for message in [
        "ACCOUNT-HOME:engine MCP server returned 401: missing credentials",
        "ACCOUNT-HOME-EOF:MCP client for 'engine' failed to start: 401 missing login credentials",
    ] {
        let reply = wrapper.turn(message).await;
        assert!(
            reply.get("inputRequests").is_none(),
            "engine MCP 401 raised a shared login action"
        );
        assert_eq!(reply["isError"], true);
        let failed: Value =
            serde_json::from_str(reply["content"][1]["text"].as_str().unwrap()).unwrap();
        assert_eq!(failed["failedTurn"]["class"], "authentication");
        assert_eq!(
            wrapper.call("new-conversation", json!({})).await["isError"],
            false
        );
    }
    wrapper.close().await;
}
