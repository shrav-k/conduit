//! The final safety read's four musts: a remount of the home, a detached descendant, Codex's
//! model-work notifications, and cleanup through a replaced scratch directory.
use super::*;
use usage_guard_probe::directory::Identity;
use usage_guard_probe::failure::Step;
use usage_guard_probe::process::{Scratch, ScratchRoot, Seen};

fn step(error: &std::io::Error) -> &'static str {
    error
        .get_ref()
        .and_then(|e| e.downcast_ref::<Step>())
        .map_or("none", |s| s.name())
}
/// Runs this test again in a child whose temporary root is `tmp`, with `key` set to `form`; the
/// child's panic message where it failed.
fn child(c: &Case, test: &str, key: &str, form: &str, tmp: &Path) -> Option<String> {
    let out = Command::new(std::env::current_exe().unwrap())
        .env_clear()
        .env("HOME", c.root.join("base"))
        .env("TMPDIR", tmp)
        .env(key, form)
        .args(["--exact", test, "--nocapture"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    (!out.status.success()).then(|| {
        stderr
            .lines()
            .skip_while(|l| !l.contains("panicked at"))
            .take(4)
            .collect::<Vec<_>>()
            .join(" ")
    })
}
/// Fails once with every form that failed, so one run shows them all.
fn all_passed(failed: Vec<String>) {
    assert!(failed.is_empty(), "failed forms:\n{}", failed.join("\n"));
}
/// The child's login home: the parent's marked root's `home`.
fn child_home() -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").unwrap())
        .parent()
        .unwrap()
        .join("home")
}
/// Every name under `dir`, at any depth, never following a link.
fn names(dir: &Path) -> Vec<String> {
    let mut found = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        found.push(entry.file_name().to_string_lossy().into_owned());
        if entry.file_type().unwrap().is_dir() {
            found.extend(names(&entry.path()));
        }
    }
    found
}
fn made_inside(home: &Path) -> bool {
    names(home)
        .iter()
        .any(|n| n.starts_with("usage-probe-") || n == "settings.json")
}

const OTHER_DEVICE: u128 = 0xfeed_0000_0000_0001;
const OTHER_INODE: u128 = 1 << 100;

/// Must 1. A remount of the login home, or of a directory in it (nullfs, bindfs or another FUSE
/// mount), reports its own device, or inodes of its own, so identity alone cannot tell it from
/// another directory. Mounting needs privileges a test lacks, so validation's reads are rewritten
/// as such a mount reports them, while the temporary root really is the home's `temp`: no
/// scratch directory and no settings file may be made inside the home, and the refusal names the
/// scratch step. A home that is itself a mount point is refused, and so is a temporary root
/// beside the home, before any client starts.
#[test]
fn scratch_remount_is_refused() {
    const TEST: &str = "safety::scratch_remount_is_refused";
    if let Some(form) = std::env::var_os("REMOUNT_CASE_CHILD") {
        let form = form.into_string().unwrap();
        let home = child_home();
        let rewrite = form.clone();
        let observe = move |seen: Seen, id: &mut Identity| match (rewrite.as_str(), seen) {
            ("device", Seen::Scratch(_) | Seen::Fresh) => id.0 = OTHER_DEVICE,
            ("inode", Seen::Scratch(_) | Seen::Fresh) => id.1 += OTHER_INODE,
            ("home_mount_point", Seen::Home(1)) => id.0 = OTHER_DEVICE,
            _ => {}
        };
        let made = ScratchRoot::validate_observed(&home, observe).and_then(|root| {
            let scratch = Scratch::new(&root)?;
            let _ = scratch.write_settings();
            Ok(made_inside(&home))
        });
        match made {
            Ok(inside) => panic!("{form}: the scratch root was accepted; made inside: {inside}"),
            Err(error) => assert_eq!(step(&error), "scratch", "{form}"),
        }
        // A different device is refused before any write; other inodes on the home's device
        // only once the fresh directory's name shows up in the home's listing.
        let reason = match form.as_str() {
            "device" => "scratch_other_device",
            "inode" => "scratch_alias",
            _ => "home_mount_point",
        };
        assert_eq!(usage_guard_probe::failure::reason(), Some(reason), "{form}");
        assert!(!made_inside(&home), "{form}: left inside the home");
        return;
    }
    let mut failed = Vec::new();
    for form in ["device", "inode", "home_mount_point"] {
        let c = Case::new(json!({}));
        fs::create_dir(c.home.join("temp")).unwrap();
        let tmp = if form == "home_mount_point" {
            c.root.join("tmp")
        } else {
            c.home.join("temp")
        };
        failed.extend(child(&c, TEST, "REMOUNT_CASE_CHILD", form, &tmp));
        assert!(!made_inside(&c.home), "{form}: left inside the home");
        c.clean_scratch();
    }
    // A temporary root beside the home (its parent) would put the scratch directory beside it.
    for codex in [true, false] {
        let c = Case::new(json!({}));
        let out = c.command(codex).env("TMPDIR", &c.root).output().unwrap();
        let beside = fs::read_dir(&c.root).unwrap().any(|e| {
            e.unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with("usage-probe-")
        });
        if out.status.code() != Some(1)
            || !text(&out)
                .lines()
                .any(|l| l == "probe_failed.scratch: true")
            || !text(&out)
                .lines()
                .any(|l| l == "probe_failed.reason: scratch_overlap")
            || !c.logs().is_empty()
            || beside
        {
            failed.push(format!(
                "beside (codex={codex}): exit {:?}, {} clients started, not refused",
                out.status.code(),
                c.logs().len()
            ));
        }
    }
    all_passed(failed);
}

/// Every `detached-PID` the fake's detached descendant wrote is gone within three seconds.
fn detached_gone(c: &Case) -> usize {
    let mut count = 0;
    for item in fs::read_dir(&c.root).unwrap() {
        let item = item.unwrap();
        if !item.file_name().to_string_lossy().starts_with("detached-") {
            continue;
        }
        count += 1;
        let pid: i32 = fs::read_to_string(item.path()).unwrap().parse().unwrap();
        let until = Instant::now() + Duration::from_secs(3);
        let gone = loop {
            let alive = unsafe { libc::kill(pid, 0) } == 0
                || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH);
            if !alive {
                break true;
            }
            if Instant::now() >= until {
                break false;
            }
            thread::sleep(Duration::from_millis(20));
        };
        assert!(gone, "a detached descendant survived the probe");
    }
    count
}

/// Must 2. A client's descendant that left its process group and session (`setsid`) survives
/// the group's stop. The probe finds it among the client's descendants it saw, kills it, prints
/// `stray_processes: 1` and exits 1, starting no further client.
#[test]
fn detached_descendant_is_stopped() {
    for codex in [true, false] {
        let c = Case::new(json!({"detached": true}));
        let out = c.run(codex);
        let client = if codex { "codex" } else { "claude" };
        assert_eq!(
            out.status.code(),
            Some(1),
            "{client}: the stray went unseen"
        );
        has(&out, "stray_processes: 1");
        has(&out, &format!("{client}.stopped_on_survivor: true"));
        has(
            &out,
            if codex {
                "codex.read.1.group_stopped: true"
            } else {
                "claude.read.group_stopped: true"
            },
        );
        assert_eq!(
            c.logs().len(),
            2,
            "{client}: a client started after the stray"
        );
        c.clean_scratch();
        assert_eq!(detached_gone(&c), 1, "{client}: no detached descendant ran");
    }
    // A run without one prints a count of none.
    let c = Case::new(json!({}));
    let out = c.run(true);
    success(&out);
    has(&out, "stray_processes: 0");
}

/// Must 3. A Codex notification that shows model work, before its answer, during its hold or
/// while it is stopped, ends the run at once: no further read starts, `model_work: true` prints
/// for that read and the command exits 1. Other notifications are still skipped.
#[test]
fn codex_model_work_notification_stops_the_reads() {
    let sign = |method: &str| json!({"method": method, "params": {}});
    for method in [
        "turn/started",
        "turn/completed",
        "item/started",
        "item/agentMessage/delta",
        "thread/tokenUsage/updated",
        "codex/event/task_started",
        "codex/event/agent_message_delta",
        "codex/event/token_count",
    ] {
        let c = Case::new(json!({"before_answer": [sign(method)]}));
        let out = c.run(true);
        assert_eq!(out.status.code(), Some(1), "{method} before the answer");
        has(&out, "codex.read.1.model_work: true");
        has(&out, "codex.stopped_on_model_work: true");
        has(&out, "codex.read.1.group_stopped: true");
        assert_eq!(c.logs().len(), 2, "{method}: a read started after the sign");
        c.clean_scratch();
    }
    for (name, script, hold) in [
        (
            "during the hold",
            json!({"after_answer": [sign("turn/started")], "after_delay_ms": 300}),
            "20",
        ),
        (
            "during the stop",
            json!({"after_answer": [sign("item/agentMessage/delta")], "after_delay_ms": 300,
                "ignore_term": true}),
            "0",
        ),
        (
            "with the answer",
            json!({"after_answer": [sign("thread/tokenUsage/updated")]}),
            "0",
        ),
    ] {
        let c = Case::new(script);
        let start = Instant::now();
        let out = c
            .command_with(true, "--hold-seconds", Some(hold))
            .output()
            .unwrap();
        assert!(
            start.elapsed() < Duration::from_secs(8),
            "{name}: the stop waited for the hold's end"
        );
        assert_eq!(out.status.code(), Some(1), "{name}");
        has(&out, "codex.read.1.answered: true");
        has(&out, "codex.read.1.model_work: true");
        has(&out, "codex.stopped_on_model_work: true");
        assert_eq!(c.logs().len(), 2, "{name}: a read started after the sign");
        c.clean_scratch();
    }
    // A sign in read 2 alone: read 1 was clean, read 3 and the pair never start.
    let c = Case::new(json!({"before_answer": [sign("turn/started")], "notify_calls": [2]}));
    let out = c.run(true);
    assert_eq!(out.status.code(), Some(1));
    has(&out, "codex.read.1.model_work: false");
    has(&out, "codex.read.2.model_work: true");
    assert_eq!(c.logs().len(), 3);
    // A sign in the pair, in whichever app-server claimed the fourth place.
    let c = Case::new(json!({"before_answer": [sign("item/started")], "notify_calls": [4]}));
    let out = c.run(true);
    assert_eq!(out.status.code(), Some(1));
    has(&out, "codex.read.3.model_work: false");
    assert!(
        text(&out)
            .lines()
            .any(|l| l == "codex.pair.1.model_work: true")
            || text(&out)
                .lines()
                .any(|l| l == "codex.pair.2.model_work: true"),
        "the pair's sign went unseen"
    );
    has(&out, "codex.stopped_on_model_work: true");
    c.clean_scratch();
    // Notifications without a sign are skipped, as before.
    let c = Case::new(
        json!({"before_answer": [sign("account/rateLimits/updated")],
        "after_answer": [sign("notice")]}),
    );
    let out = c.run(true);
    success(&out);
    has(&out, "codex.read.1.model_work: false");
    has(&out, "codex.pair.2.model_work: false");
}

/// Must 4. Cleanup removes only what lies in the probe's own scratch directory, through the
/// descriptors it kept: a scratch directory replaced by a symlink to the login home, or by the
/// login home itself (as a rename or a mount over it would), and the same one level down, leave
/// every file in the home as it was.
#[test]
fn cleanup_never_follows_a_replacement() {
    const TEST: &str = "safety::cleanup_never_follows_a_replacement";
    let forms = [
        "symlink",
        "directory",
        "nested_symlink",
        "nested_directory",
        "nested_home_entry",
    ];
    if let Some(form) = std::env::var_os("CLEANUP_CASE_CHILD") {
        let form = form.into_string().unwrap();
        let home = child_home();
        fs::create_dir(home.join("state")).unwrap();
        fs::write(home.join("state/entry"), b"kept").unwrap();
        let root = ScratchRoot::validate(&home).unwrap();
        let scratch = Scratch::new(&root).unwrap();
        let path = scratch.0.clone();
        let moved = home.parent().unwrap().join("moved-scratch");
        // Where each form leaves the login home's files once it has run.
        let (files, state) = match form.as_str() {
            "symlink" => {
                fs::rename(&path, &moved).unwrap();
                symlink(&home, &path).unwrap();
                (home.clone(), home.join("state"))
            }
            "directory" => {
                fs::rename(&path, &moved).unwrap();
                fs::rename(&home, &path).unwrap();
                (path.clone(), path.join("state"))
            }
            "nested_symlink" => {
                symlink(&home, path.join("d")).unwrap();
                (home.clone(), home.join("state"))
            }
            "nested_directory" => {
                fs::rename(&home, path.join("d")).unwrap();
                (path.join("d"), path.join("d/state"))
            }
            _ => {
                fs::rename(home.join("state"), path.join("d")).unwrap();
                (home.clone(), path.join("d"))
            }
        };
        drop(scratch);
        drop(root);
        // Whatever cleanup could not remove is reported; only the nested link went entirely.
        assert_eq!(
            usage_guard_probe::process::scratch_left(),
            form != "nested_symlink",
            "{form}: what cleanup left was not reported"
        );
        assert_eq!(
            fs::read(files.join("auth.json")).ok().as_deref(),
            Some(&b"fabricated-login"[..]),
            "{form}: cleanup removed the login file"
        );
        assert_eq!(
            fs::read(state.join("entry")).ok().as_deref(),
            Some(&b"kept"[..]),
            "{form}: cleanup removed a file in the login home"
        );
        return;
    }
    let mut failed = Vec::new();
    for form in forms {
        let c = Case::new(json!({}));
        failed.extend(child(
            &c,
            TEST,
            "CLEANUP_CASE_CHILD",
            form,
            &c.root.join("tmp"),
        ));
    }
    all_passed(failed);
}
