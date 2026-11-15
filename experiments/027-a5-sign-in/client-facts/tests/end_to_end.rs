use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

/// The probe against the fake client, with `everyday/` as HOME and `temporary/` as TMPDIR.
fn probe(root: &Path, mode: Option<&str>) -> Command {
    probe_program(root, mode, Path::new(env!("CARGO_BIN_EXE_client-facts")))
}

fn probe_program(root: &Path, mode: Option<&str>, program: &Path) -> Command {
    let home = root.join("everyday");
    let temporary = root.join("temporary");
    fs::create_dir(&home).unwrap();
    fs::create_dir(&temporary).unwrap();
    let output_file = root.join("facts.json");
    let mut command = Command::new(program);
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
    assert_eq!(
        stderr,
        "step 5 claude-code: scratch login found; step 5 claude-code: logout; step 5 codex: scratch login found; step 5 codex: logout\n"
    );
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

fn lines(path: &Path) -> usize {
    fs::read_to_string(path).unwrap_or_default().lines().count()
}

/// Keep this alive through every assertion: even a failed mutation must leave no login.
struct ProbeRun {
    child: Option<Child>,
    home: PathBuf,
    gone: HashSet<i32>,
}
impl ProbeRun {
    fn start(command: &mut Command, root: &Path) -> Self {
        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        Self {
            child: Some(child),
            home: root.join("everyday"),
            gone: HashSet::new(),
        }
    }
    fn wait_lines(&self, name: &str, count: usize) {
        let start = Instant::now();
        while lines(&self.home.join(name)) < count {
            assert!(
                start.elapsed() < Duration::from_secs(60),
                "waiting for {name}: {count} lines"
            );
            sleep(Duration::from_millis(20));
        }
    }
    fn signal(&self, signal: i32) {
        // SAFETY: this PID is the live probe owned by this guard.
        assert_eq!(
            unsafe { libc::kill(self.child.as_ref().unwrap().id() as i32, signal) },
            0
        );
    }
    fn wait(&mut self) -> Output {
        let start = Instant::now();
        while self.child.as_mut().unwrap().try_wait().unwrap().is_none() {
            assert!(
                start.elapsed() < Duration::from_secs(60),
                "probe did not finish"
            );
            sleep(Duration::from_millis(20));
        }
        self.child.take().unwrap().wait_with_output().unwrap()
    }
    fn login_pids(&self) -> Vec<i32> {
        fs::read_to_string(self.home.join("login-started"))
            .unwrap_or_default()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect()
    }
    fn assert_stopped(&mut self, root: &Path, output: &Output, stderr: &str) {
        assert_eq!(output.status.code(), Some(1));
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            format!("{stderr}\n")
        );
        assert!(output.stdout.is_empty());
        assert!(!root.join("facts.json").exists());
        assert_eq!(fs::read_dir(root.join("temporary")).unwrap().count(), 0);
        for pid in self.login_pids() {
            // SAFETY: signal 0 only tests the synthetic login's process group.
            assert_eq!(unsafe { libc::kill(-pid, 0) }, -1);
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::ESRCH)
            );
            self.gone.insert(pid);
        }
    }
}
impl Drop for ProbeRun {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        for pid in self.login_pids() {
            if !self.gone.contains(&pid) {
                // SAFETY: only synthetic logins in this test's private root are recorded.
                unsafe {
                    libc::kill(-pid, libc::SIGKILL);
                }
                let start = Instant::now();
                // Give the killed group time to disappear before the test root is removed.
                while start.elapsed() < Duration::from_secs(2) {
                    // SAFETY: signal 0 only tests this synthetic login's group.
                    if unsafe { libc::kill(-pid, 0) } == -1
                        && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
                    {
                        break;
                    }
                    sleep(Duration::from_millis(20));
                }
            }
        }
    }
}

#[test]
fn signal_during_login_cleans_up() {
    for (mode, signal, delay) in [
        (None, libc::SIGINT, 0),
        (None, libc::SIGTERM, 0),
        (None, libc::SIGHUP, 0),
        (Some("ignore-term"), libc::SIGTERM, 3500),
    ] {
        let root = tempfile::tempdir().unwrap();
        let mut run = ProbeRun::start(&mut probe(root.path(), mode), root.path());
        run.wait_lines("login-started", 1);
        sleep(Duration::from_millis(delay));
        run.signal(signal);
        let output = run.wait();
        run.assert_stopped(root.path(), &output, "step 4 claude-code: signal");
        assert_eq!(lines(&run.home.join("login-started")), 1);
        assert!(!run.home.join("logout-started").exists());
    }
}

#[test]
fn signal_during_login_still_checks_the_scratch_home() {
    let root = tempfile::tempdir().unwrap();
    let mut run = ProbeRun::start(
        &mut probe(root.path(), Some("complete-and-wait")),
        root.path(),
    );
    run.wait_lines("login-started", 1);
    run.signal(libc::SIGTERM);
    let output = run.wait();
    run.assert_stopped(root.path(), &output, "step 5 claude-code: scratch login found; step 5 claude-code: logout; step 4 claude-code: signal");
    assert_eq!(lines(&run.home.join("logout-done")), 1);
    assert_eq!(lines(&run.home.join("login-started")), 1);
}

#[test]
fn signal_never_cuts_the_safety_logout() {
    let root = tempfile::tempdir().unwrap();
    let mut command = probe(root.path(), Some("complete"));
    fs::write(root.path().join("everyday/slow-logout"), "yes").unwrap();
    let mut run = ProbeRun::start(&mut command, root.path());
    run.wait_lines("logout-started", 1);
    run.signal(libc::SIGTERM);
    let output = run.wait();
    run.assert_stopped(root.path(), &output, "step 5 claude-code: scratch login found; step 5 claude-code: logout; step 5 claude-code: signal");
    assert_eq!(lines(&run.home.join("logout-done")), 1);
    assert_eq!(lines(&run.home.join("login-started")), 1);
}

#[test]
fn signal_lets_an_everyday_status_finish() {
    let root = tempfile::tempdir().unwrap();
    let mut command = probe(root.path(), None);
    fs::write(root.path().join("everyday/slow-status"), "yes").unwrap();
    let mut run = ProbeRun::start(&mut command, root.path());
    run.wait_lines("status-started", 1);
    run.signal(libc::SIGTERM);
    let output = run.wait();
    run.assert_stopped(root.path(), &output, "step 2 claude-code: signal");
    assert_eq!(lines(&run.home.join("status-done")), 1);
    assert!(!run.home.join("login-started").exists());
}

#[test]
fn signal_during_the_last_step_removes_the_report() {
    let root = tempfile::tempdir().unwrap();
    let mut command = probe(root.path(), None);
    fs::write(root.path().join("everyday/slow-status"), "yes").unwrap();
    let mut run = ProbeRun::start(&mut command, root.path());
    run.wait_lines("status-started", 4);
    run.signal(libc::SIGTERM);
    let output = run.wait();
    run.assert_stopped(root.path(), &output, "step 6 codex: signal");
    assert_eq!(lines(&run.home.join("status-done")), 4);
    assert_eq!(lines(&run.home.join("login-started")), 2);
}

#[test]
fn shim_missing_or_not_executable_faults() {
    let binaries = tempfile::tempdir().unwrap();
    let program = binaries.path().join("client-facts");
    fs::copy(env!("CARGO_BIN_EXE_client-facts"), &program).unwrap();
    for present in [false, true] {
        if present {
            let shim = binaries.path().join("browser-shim");
            fs::write(&shim, "synthetic non-executable shim").unwrap();
            fs::set_permissions(shim, fs::Permissions::from_mode(0o600)).unwrap();
        }
        let root = tempfile::tempdir().unwrap();
        let mut run = ProbeRun::start(&mut probe_program(root.path(), None, &program), root.path());
        let output = run.wait();
        run.assert_stopped(root.path(), &output, "step 0 claude-code: shim fault");
        assert!(!run.home.join("login-started").exists());
    }
}
