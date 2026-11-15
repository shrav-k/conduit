use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

/// The probe against the fake client, with `everyday/` as HOME and `temporary/` as TMPDIR.
fn probe(root: &Path, mode: Option<&str>) -> Command {
    let home = root.join("everyday");
    let temporary = root.join("temporary");
    fs::create_dir(&home).unwrap();
    fs::create_dir(&temporary).unwrap();
    let output_file = root.join("facts.json");
    let mut command = Command::new(env!("CARGO_BIN_EXE_client-facts"));
    command
        .env_clear()
        .env("HOME", &home)
        .env("TMPDIR", &temporary)
        .env("A5_MUST_NOT_LEAK", "synthetic")
        .args([
            "--claude",
            env!("CARGO_BIN_EXE_fake-client"),
            "--codex",
            env!("CARGO_BIN_EXE_fake-client"),
            "--out",
        ])
        .arg(&output_file);
    if let Some(mode) = mode {
        for client in ["claude", "codex"] {
            command
                .arg("--login-arg")
                .arg(format!("{client}=--mode={mode}"));
        }
    }
    command
}

fn run(mode: Option<&str>, shared: bool) {
    let root = tempfile::tempdir().unwrap();
    let mut command = probe(root.path(), mode);
    let home = root.path().join("everyday");
    let temporary = root.path().join("temporary");
    let output_file = root.path().join("facts.json");
    if shared {
        fs::write(home.join("shared-home"), "yes").unwrap();
    }
    let result = command.output().unwrap();
    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    let encoded = fs::read_to_string(&output_file).unwrap();
    for forbidden in [
        "@".into(),
        "://".into(),
        "a".repeat(8),
        "c".repeat(16),
        "p".repeat(43),
        "s".repeat(32),
    ] {
        assert!(!encoded.contains(&forbidden));
        assert!(!String::from_utf8_lossy(&result.stdout).contains(&forbidden));
    }
    let report: Value = serde_json::from_str(&encoded).unwrap();
    let table = String::from_utf8(result.stdout).unwrap();
    assert!(table.starts_with("| Fact | claude-code 1.2.3 | codex 1.2.3 |"));
    for client in report["clients"].as_array().unwrap() {
        assert_eq!(client["version"], "1.2.3");
        let steps = &client["steps"];
        assert_eq!(steps["2"]["login"], "yes");
        assert_eq!(steps["6"]["login"], "yes");
        if shared {
            assert_eq!(steps["3"]["login"], "yes");
            assert_eq!(steps["4"]["skipped"], true);
            assert_eq!(steps["4"]["group_gone"], Value::Null);
            assert_eq!(steps["5"]["skipped"], true);
            assert_eq!(steps["5"]["login"], Value::Null);
            continue;
        }
        assert_eq!(steps["3"]["login"], "no");
        assert_eq!(steps["5"]["login"], "no");
        assert_eq!(
            steps["4"]["urls_seen"],
            if mode == Some("late-redirect") { 2 } else { 1 }
        );
        for url in steps["4"]["urls"].as_array().unwrap() {
            assert_eq!(url["return"], "redirect");
            assert_eq!(url["ifc_049"], serde_json::json!([]));
            assert_eq!(url["listens"], "yes");
            assert_eq!(
                url["source"],
                if mode == Some("browser") {
                    "shim"
                } else {
                    "printed"
                }
            );
        }
        assert_eq!(steps["4"]["waiting_at_cancel"], true);
        assert_eq!(steps["4"]["exited_before_cancel"], Value::Null);
        assert_eq!(
            steps["4"]["ended_on"],
            if mode == Some("ignore-term") {
                "kill"
            } else {
                "term"
            }
        );
        assert_eq!(steps["4"]["group_gone"], true);
        assert_eq!(steps["4"]["logged_out"], false);
        assert_eq!(steps["4"]["shim_called"], mode == Some("browser"));
    }
    assert_eq!(fs::read_dir(&temporary).unwrap().count(), 0);
    assert!(!home.join("logout-started").exists());
    if shared {
        assert!(!home.join("login-started").exists());
    } else {
        assert_eq!(
            fs::read_to_string(home.join("login-started"))
                .unwrap()
                .lines()
                .count(),
            2
        );
    }
    // create_new must refuse overwriting an earlier run, before starting any client.
    let second = command.output().unwrap();
    assert!(!second.status.success());
    assert_eq!(fs::read_to_string(&output_file).unwrap(), encoded);
}

#[test]
fn end_to_end_print() {
    run(None, false);
}
#[test]
fn end_to_end_browser() {
    run(Some("browser"), false);
}
#[test]
fn end_to_end_ignore_term() {
    run(Some("ignore-term"), false);
}
#[test]
fn end_to_end_shared_home() {
    run(None, true);
}

#[test]
fn redirect_emitted_during_listener_check() {
    run(Some("late-redirect"), false);
}

/// A login that signs the scratch home in gets the scratch-only logout, which is reported
/// for both clients, keeps the report and leaves the everyday status signed in.
#[test]
fn safety_logout_scratch_only() {
    let root = tempfile::tempdir().unwrap();
    let result = probe(root.path(), Some("complete")).output().unwrap();
    assert!(!result.status.success());
    let stderr = String::from_utf8(result.stderr).unwrap();
    assert!(stderr.contains("step 5 claude-code: logout"));
    assert!(stderr.contains("step 5 codex: logout"));
    let encoded = fs::read_to_string(root.path().join("facts.json")).unwrap();
    let report: Value = serde_json::from_str(&encoded).unwrap();
    for client in report["clients"].as_array().unwrap() {
        let steps = &client["steps"];
        assert_eq!(steps["4"]["logged_out"], true);
        assert_eq!(steps["5"]["login"], "yes");
        assert_eq!(steps["6"]["login"], "yes");
    }
    let logouts = fs::read_to_string(root.path().join("everyday/logout-started")).unwrap();
    assert_eq!(logouts.lines().count(), 2);
    let temporary = root.path().join("temporary");
    assert_eq!(fs::read_dir(temporary).unwrap().count(), 0);
}
