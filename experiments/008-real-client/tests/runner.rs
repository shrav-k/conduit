use real_client_harness::runner::{self, Client};
use std::{fs, path::Path, process::Command};

#[test]
fn dry_run_produces_amended_24_cells_and_68_runs_without_creating_output() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("does-not-exist");
    let result = Command::new(env!("CARGO_BIN_EXE_real-client-harness"))
        .args(["--dry-run", "--output"])
        .arg(&output)
        .output()
        .unwrap();
    assert!(result.status.success());
    let text = String::from_utf8(result.stdout).unwrap();
    assert_eq!(text.lines().filter(|s| s.starts_with("RUN ")).count(), 68);
    assert_eq!(text.lines().filter(|s| s.contains(" --serve ")).count(), 68);
    assert_eq!(
        text.lines().filter(|s| s.starts_with("CONFIG ")).count(),
        68
    );
    assert_eq!(
        text.lines().filter(|s| s.contains(" < /dev/null")).count(),
        136
    );
    assert!(
        text.ends_with("TOTAL 68 runs (24 cells; 3 repetitions per normal cell, 1 per 401 cell)\n")
    );
    assert!(!output.exists());
    for cell in runner::matrix() {
        for rep in 1..=cell.repetitions() {
            assert!(text.contains(&format!("RUN {}-r{rep}\n", cell.id())));
        }
    }
}
#[test]
fn only_selects_one_cell_with_its_three_repetitions_and_rejects_typos() {
    let cell = runner::matrix().remove(0).id();
    let result = Command::new(env!("CARGO_BIN_EXE_real-client-harness"))
        .args(["--dry-run", "--only", &cell])
        .output()
        .unwrap();
    assert!(result.status.success());
    let text = String::from_utf8(result.stdout).unwrap();
    assert_eq!(text.lines().filter(|s| s.starts_with("RUN ")).count(), 3);
    let result = Command::new(env!("CARGO_BIN_EXE_real-client-harness"))
        .args(["--dry-run", "--only", "typo"])
        .output()
        .unwrap();
    assert!(!result.status.success());
}
#[test]
fn plans_keep_isolation_helpers_and_cache_variants_explicit() {
    let run = Path::new("/tmp/lab with apostrophe's path");
    let exe = Path::new("/tmp/executable with apostrophe's path");
    for cell in runner::matrix() {
        let plan = runner::plan(&cell, run, exe, "http://127.0.0.1:12345/");
        let config = runner::configs(&cell, run, exe, "http://127.0.0.1:12345/");
        assert_eq!(plan.cwd, run.join("lab"));
        for key in ["ANTHROPIC_API_KEY", "OPENAI_API_KEY"] {
            assert!(plan.env_remove.iter().any(|name| name == key));
            assert!(
                plan.command()
                    .as_std()
                    .get_envs()
                    .any(|(name, value)| name == key && value.is_none())
            );
            assert!(plan.display().contains(&format!("-u '{key}'")));
        }
        assert!(!plan.env.contains_key("CLAUDE_CONFIG_DIR"));
        assert!(!plan.env.contains_key("CODEX_HOME"));
        match cell.client {
            Client::Claude => {
                assert!(plan.args.windows(2).any(|w| w == ["--tools", ""]));
                assert!(
                    plan.args
                        .windows(2)
                        .any(|w| w == ["--allowedTools", "mcp__stub__*"])
                );
                assert!(
                    plan.args
                        .windows(2)
                        .any(|w| w == ["--setting-sources", "project"])
                );
                for flag in [
                    "--strict-mcp-config",
                    "--disable-slash-commands",
                    "--no-session-persistence",
                ] {
                    assert!(plan.args.iter().any(|s| s == flag));
                }
                assert_eq!(
                    plan.env["MCP_DISCOVERY_CACHE"],
                    if cell.cache { "1" } else { "0" }
                );
                assert!(
                    config["mcp.json"]["mcpServers"]["stub"]["headersHelper"]
                        .as_str()
                        .unwrap()
                        .contains("--headers-helper")
                );
            }
            Client::Codex => {
                for flag in [
                    "--ignore-user-config",
                    "--strict-config",
                    "--ignore-rules",
                    "--ephemeral",
                ] {
                    assert!(plan.args.iter().any(|s| s == flag));
                }
                assert!(
                    plan.args
                        .windows(2)
                        .any(|w| w == ["--sandbox", "read-only"])
                );
                assert!(plan.args.windows(2).any(|w| w == ["--disable", "apps"]));
                assert!(plan.args.iter().any(|s| s
                    == &format!(
                        "mcp_servers.stub.startup_readiness=\"{}\"",
                        if cell.cache { "catalog" } else { "connection" }
                    )));
                assert!(
                    plan.args
                        .iter()
                        .any(|s| s.starts_with("mcp_servers.stub.http_headers_helper="))
                );
            }
        }
        assert_eq!(
            config["mcp.json"]["mcpServers"].as_object().unwrap().len(),
            1
        );
        // Real shell parsing, with harmless printf, verifies paths can't become shell syntax.
        let command = format!(
            "printf '%s' {}",
            runner::quote(
                config["mcp.json"]["mcpServers"]["stub"]["headersHelper"]
                    .as_str()
                    .unwrap()
            )
        );
        let result = Command::new("sh").arg("-c").arg(command).output().unwrap();
        assert!(result.status.success());
        assert_eq!(
            String::from_utf8(result.stdout).unwrap(),
            config["mcp.json"]["mcpServers"]["stub"]["headersHelper"]
                .as_str()
                .unwrap()
        );
    }
}
#[test]
fn output_rejects_repository_and_symlink_aliases_before_writing() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();
    let forbidden = repo.join("must-not-be-created");
    assert!(runner::prepare_output(&forbidden).is_err());
    assert!(!forbidden.exists());
    let dir = tempfile::tempdir().unwrap();
    let alias = dir.path().join("alias");
    std::os::unix::fs::symlink(&repo, &alias).unwrap();
    assert!(runner::prepare_output(&alias.join("must-not-be-created")).is_err());
    assert!(runner::prepare_output(Path::new("relative")).is_err());
    assert!(runner::prepare_output(&dir.path().join("new/../another")).is_err());
    assert!(!dir.path().join("new").exists());
    let outside = dir.path().join("outside");
    assert_eq!(
        runner::prepare_output(&outside).unwrap(),
        fs::canonicalize(outside).unwrap()
    );
}

#[test]
fn matrix_variants_apply_only_to_registered_cases() {
    let cells = runner::matrix();
    assert_eq!(cells.len(), 24);
    let mut run_counts = std::collections::BTreeMap::new();
    let mut ids = std::collections::BTreeSet::new();
    for cell in cells {
        assert!(ids.insert(cell.id()));
        *run_counts.entry(cell.case.name()).or_insert(0) += cell.repetitions();
        if cell.first_401 {
            assert_eq!(cell.case.name(), "C3");
            assert_eq!(cell.repetitions(), 1);
        }
        if !matches!(cell.case.name(), "C1" | "C5") {
            assert!(!cell.cache);
            assert_eq!(cell.ttl, 0);
        }
    }
    assert_eq!(
        run_counts,
        [("C1", 24), ("C2", 6), ("C3", 8), ("C4", 6), ("C5", 24)].into()
    );
    for client in ["claude", "codex"] {
        let cell = format!("{client}-C3-cache-off-ttl-0-auth-401");
        let result = Command::new(env!("CARGO_BIN_EXE_real-client-harness"))
            .args(["--dry-run", "--only", &cell])
            .output()
            .unwrap();
        assert!(result.status.success());
        let text = String::from_utf8(result.stdout).unwrap();
        assert_eq!(
            text.lines().filter(|line| line.starts_with("RUN ")).count(),
            1
        );
        assert!(text.contains(&format!("RUN {cell}-r1\n")));
    }
}

#[tokio::test]
async fn client_command_strips_api_key_environment_without_reading_keys() {
    for cell in runner::matrix() {
        let mut plan = runner::plan(
            &cell,
            Path::new("/tmp"),
            Path::new("/bin/false"),
            "http://127.0.0.1:1/",
        );
        plan.program = "/bin/sh".into();
        plan.cwd = Path::new("/tmp").into();
        plan.args = vec![
            "-c".into(),
            "test -z \"${ANTHROPIC_API_KEY+x}\" && test -z \"${OPENAI_API_KEY+x}\"".into(),
        ];
        plan.env
            .insert("ANTHROPIC_API_KEY".into(), "synthetic-test-value".into());
        plan.env
            .insert("OPENAI_API_KEY".into(), "synthetic-test-value".into());
        assert!(plan.command().status().await.unwrap().success());
    }
}

#[test]
fn actual_run_and_standalone_stub_require_explicit_output() {
    for args in [vec![], vec!["--serve"]] {
        let result = Command::new(env!("CARGO_BIN_EXE_real-client-harness"))
            .args(args)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(
            String::from_utf8(result.stderr)
                .unwrap()
                .contains("provide --output")
        );
    }
}

#[test]
fn follow_up_dry_run_prints_exactly_twelve_runs_and_both_turns_without_output() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("no-output");
    let result = Command::new(env!("CARGO_BIN_EXE_real-client-harness"))
        .args(["--follow-up", "--dry-run", "--output"])
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let text = String::from_utf8(result.stdout).unwrap();
    let cells = runner::follow_up_matrix();
    assert_eq!(cells.len(), 4);
    let actual: std::collections::BTreeSet<_> = text
        .lines()
        .filter(|line| line.starts_with("RUN "))
        .map(str::to_owned)
        .collect();
    let expected: std::collections::BTreeSet<_> = cells
        .iter()
        .flat_map(|cell| (1..=3).map(move |rep| format!("RUN {}-r{rep}", cell.id())))
        .collect();
    assert_eq!(actual, expected);
    assert_eq!(
        text.lines().filter(|line| line.starts_with("RUN ")).count(),
        12
    );
    for prefix in ["TURN 1 START", "TURN 2 AFTER TURN 1 RESULT", "CONFIG "] {
        assert_eq!(
            text.lines().filter(|line| line.starts_with(prefix)).count(),
            12
        );
    }
    assert_eq!(
        text.lines()
            .filter(|line| line.starts_with("SEND TURN "))
            .count(),
        12
    );
    assert_eq!(
        text.lines()
            .filter(|line| line.contains("'resume'"))
            .count(),
        6
    );
    assert!(!text.contains("--ephemeral"));
    assert!(
        text.ends_with("TOTAL 12 runs (4 cells; 3 repetitions per normal cell, 1 per 401 cell)\n")
    );
    assert!(!output.exists());
    for cell in cells {
        assert!(!cell.cache && cell.ttl == 0 && !cell.first_401 && cell.later);
        let result = Command::new(env!("CARGO_BIN_EXE_real-client-harness"))
            .args(["--follow-up", "--dry-run", "--only", &cell.id()])
            .output()
            .unwrap();
        assert!(result.status.success());
        assert_eq!(
            String::from_utf8(result.stdout)
                .unwrap()
                .lines()
                .filter(|line| line.starts_with("RUN "))
                .count(),
            3
        );
    }
}

#[test]
fn later_turn_plans_preserve_flags_and_use_streamed_input_or_exact_session_resume() {
    let run = Path::new("/tmp/run");
    for cell in runner::follow_up_matrix() {
        let original = runner::plan(&cell, run, Path::new("/tmp/harness"), "http://127.0.0.1:1/");
        let first = runner::turn_plan(
            &cell,
            run,
            Path::new("/tmp/harness"),
            "http://127.0.0.1:1/",
            1,
            None,
        );
        let second = runner::turn_plan(
            &cell,
            run,
            Path::new("/tmp/harness"),
            "http://127.0.0.1:1/",
            2,
            Some("exact-session-id"),
        );
        assert_ne!(cell.turn_prompt(1), cell.turn_prompt(2));
        assert!(cell.turn_prompt(1).contains("control.next.v1"));
        assert!(!cell.turn_prompt(2).contains("control.next.v1"));
        if cell.case.name() == "C1" {
            assert!(cell.turn_prompt(1).contains("files.read.v1"));
            assert!(cell.turn_prompt(2).contains("files.read.v1"));
        } else {
            assert!(!cell.turn_prompt(1).contains("notes.read.v1"));
            assert!(cell.turn_prompt(2).contains("notes.read.v1"));
        }
        for (index, plan) in [&first, &second].into_iter().enumerate() {
            assert_eq!(plan.env, original.env);
            assert_eq!(plan.env_remove, original.env_remove);
            assert_eq!(plan.cwd, original.cwd);
            // Every original flag is retained apart from Codex's deliberate persistence change.
            for arg in &original.args[..original.args.len() - 1] {
                if arg == "--ephemeral" || arg.ends_with("/final.txt") {
                    continue;
                }
                assert!(plan.args.contains(arg), "missing original argument {arg}");
            }
            match cell.client {
                Client::Claude => {
                    assert_eq!(first.args, second.args);
                    assert!(
                        plan.args
                            .windows(2)
                            .any(|pair| pair == ["--input-format", "stream-json"])
                    );
                    let message = plan.stdin_message.as_ref().unwrap();
                    assert_eq!(message["type"], "user");
                    assert_eq!(message["message"]["role"], "user");
                    assert_eq!(message["message"]["content"], cell.turn_prompt(index + 1));
                    assert!(!plan.args.contains(&cell.turn_prompt(index + 1)));
                }
                Client::Codex => {
                    assert!(plan.stdin_message.is_none());
                    assert!(
                        !plan
                            .args
                            .iter()
                            .any(|arg| arg == "--ephemeral" || arg == "--last")
                    );
                    assert!(plan.args.iter().any(
                        |arg| arg == "mcp_servers.stub.default_tools_approval_mode=\"approve\""
                    ));
                    assert!(
                        plan.args
                            .contains(&format!("/tmp/run/turn-{}/final.txt", index + 1))
                    );
                    assert_eq!(plan.args.last().unwrap(), &cell.turn_prompt(index + 1));
                    if index == 1 {
                        assert_eq!(
                            &plan.args[..6],
                            [
                                "exec",
                                "--sandbox",
                                "read-only",
                                "--color",
                                "never",
                                "resume"
                            ]
                        );
                        assert_eq!(plan.args[plan.args.len() - 2], "exact-session-id");
                    } else {
                        assert!(!plan.args.iter().any(|arg| arg == "resume"));
                    }
                }
            }
        }
    }
}

#[test]
fn resume_rejects_missing_failed_or_ambiguous_first_turns() {
    let started = "{\"type\":\"thread.started\",\"thread_id\":\"exact-id\"}\n";
    let completed = "{\"type\":\"turn.completed\"}\n";
    assert_eq!(
        runner::completed_session(&format!("{started}{completed}")).unwrap(),
        "exact-id"
    );
    for events in [
        String::new(),
        started.into(),
        completed.into(),
        format!("{started}{{\"type\":\"turn.failed\"}}\n{completed}"),
        format!("{started}{{\"type\":\"thread.started\",\"thread_id\":\"other-id\"}}\n{completed}"),
    ] {
        assert!(runner::completed_session(&events).is_err());
    }
}
