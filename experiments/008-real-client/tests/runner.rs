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
