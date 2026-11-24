use super::*;
use usage_guard_probe::process::{Client, Launch, ScratchRoot};

#[test]
fn scratch_overlap_is_refused() {
    for form in ["direct", "nested", "alias", "reachable", "dangling"] {
        let c = Case::new(json!({}));
        let temporary = match form {
            "direct" => c.home.clone(),
            "nested" => {
                fs::create_dir(c.home.join("temp")).unwrap();
                c.home.join("temp")
            }
            "alias" => {
                symlink(&c.home, c.root.join("alias")).unwrap();
                c.root.join("alias")
            }
            "dangling" => {
                symlink(
                    c.root.join("tmp/future-directory"),
                    c.home.join("temp-link"),
                )
                .unwrap();
                c.root.join("tmp")
            }
            _ => {
                symlink(c.root.join("tmp"), c.home.join("temp-link")).unwrap();
                c.root.join("tmp")
            }
        };
        for codex in [false, true] {
            let out = c.command(codex).env("TMPDIR", &temporary).output().unwrap();
            assert!(c.logs().is_empty(), "scratch overlap started a client");
            assert_eq!(out.status.code(), Some(1));
            assert!(!text(&out).contains(c.root.to_str().unwrap()));
        }
    }
}

#[test]
fn listing_refuses_directory_replacement() {
    for replacement in ["link", "directory"] {
        let c = Case::new(json!({}));
        fs::create_dir(c.home.join("state")).unwrap();
        fs::create_dir(c.root.join("outside")).unwrap();
        fs::write(c.root.join("outside/never-traverse"), b"").unwrap();
        let mut replaced = false;
        let result = Listing::read_observed(&c.home, |path, stage| {
            if path == Path::new("state") && stage == "after_stat" && !replaced {
                fs::rename(c.home.join("state"), c.root.join("old-state")).unwrap();
                if replacement == "link" {
                    symlink(c.root.join("outside"), c.home.join("state")).unwrap();
                } else {
                    fs::create_dir(c.home.join("state")).unwrap();
                }
                replaced = true;
            }
        });
        assert!(result.is_err(), "listing accepted a replaced directory");
    }
}

#[test]
fn whitespace_empty_arrays_do_not_panic() {
    for raw in [
        r#"{"id":2,"result":{"rateLimits":[ ]}}"#,
        r#"{"id":2,"result":{"rateLimits":[ [ ], {"primary":[  ]} ]}}"#,
        "{\"id\":2,\"result\":{\"rateLimits\":[\t\r ]}}",
    ] {
        let c = Case::new(json!({"raw_answer":raw}));
        let out = c.run(true);
        assert_eq!(
            out.status.code(),
            Some(0),
            "valid whitespace-only arrays panicked"
        );
        assert!(out.stderr.is_empty());
        has(&out, "codex.read.1.field rateLimits: array");
    }
}

fn queue_child(test_name: &str, pair: bool) {
    let c = Case::new(json!({}));
    if std::env::var_os("QUEUE_CASE_CHILD").is_none() {
        let out = Command::new(std::env::current_exe().unwrap())
            .env_clear()
            .env("HOME", c.root.join("base"))
            .env("TMPDIR", c.root.join("tmp"))
            .env("QUEUE_CASE_CHILD", "1")
            .args(["--exact", test_name, "--nocapture"])
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "timely queued reply was discarded: {} {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        return;
    }
    // Use the parent's marked root: both client homes must share its marker.
    let home = PathBuf::from(std::env::var_os("HOME").unwrap())
        .parent()
        .unwrap()
        .join("home");
    let args = vec!["app-server".into()];
    let root = ScratchRoot::validate(&home).unwrap();
    let mut a = Client::spawn(
        &root,
        Path::new(env!("CARGO_BIN_EXE_fake-codex")),
        &home,
        &args,
        Launch {
            codex: true,
            deadline: Duration::from_millis(2000),
            version: false,
            settings: false,
        },
    )
    .unwrap();
    if pair {
        let mut b = Client::spawn(
            &root,
            Path::new(env!("CARGO_BIN_EXE_fake-codex")),
            &home,
            &args,
            Launch {
                codex: true,
                deadline: Duration::from_millis(1500),
                version: false,
                settings: false,
            },
        )
        .unwrap();
        send::codex(b.input.as_mut().unwrap(), &json!({"id":2,"method":"account/rateLimits/read","params":{"excludeResetCreditDetails":true}})).unwrap();
        // A stays silent until its longer deadline. B's reader queues its timely answer.
        let (aa, ba) = usage_guard_probe::run::pair_answers(&a, &b);
        assert!(aa.is_none());
        assert!(ba.is_some(), "pair discarded B's timely queued reply");
        assert!(b.stop());
    } else {
        send::codex(a.input.as_mut().unwrap(), &json!({"id":2,"method":"account/rateLimits/read","params":{"excludeResetCreditDetails":true}})).unwrap();
        thread::sleep(Duration::from_millis(2300));
        assert!(
            a.line().is_some(),
            "consumer discarded a timely queued reply"
        );
    }
    assert!(a.stop());
}
#[test]
fn queued_reply_survives_delayed_consumption() {
    queue_child("fixes::queued_reply_survives_delayed_consumption", false);
}
#[test]
fn paired_reply_survives_delayed_consumption() {
    queue_child("fixes::paired_reply_survives_delayed_consumption", true);
}

#[test]
fn private_filenames_are_masked() {
    let account = ["acct", "-private-id"].concat();
    let token = ["sk", "-live-", "QxYzMnOpRtUvWxYz"].concat();
    let c = Case::new(json!({"create_names":[account,token]}));
    let out = c.run(false);
    success(&out);
    assert!(
        !text(&out).contains(&account),
        "account ID escaped through a filename"
    );
    assert!(
        !text(&out).contains(&token),
        "token escaped through a filename"
    );
    has(&out, "home.changed ?: added");
}

#[test]
fn no_answer_stop_is_explicit() {
    for script in [
        json!({"never":true}),
        json!({"initialize_delay_ms":1500}),
        json!({"raw_answer":"not-json"}),
    ] {
        let c = Case::new(script);
        let out = c
            .command_with(true, "--deadline-seconds", Some("1"))
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(1));
        has(&out, "codex.stopped_on_no_answer: true");
        assert_eq!(c.logs().len(), 2);
    }
}
#[test]
fn absent_note_has_no_location() {
    let c = Case::new(json!({}));
    let out = c.run(false);
    success(&out);
    has(&out, "claude.read.last_known_note: false");
    has(&out, "claude.read.last_known_where: none");
}
#[test]
fn failures_name_the_step_and_version() {
    let c = Case::new(json!({"version":"9.8.7-private-suffix"}));
    let out = c.run(true);
    has(&out, "codex.version_found: 9.8.7");
    assert!(!text(&out).contains("private-suffix"));
    let out = c
        .command_with(
            true,
            "--codex",
            Some(c.root.join("missing-client").to_str().unwrap()),
        )
        .output()
        .unwrap();
    has(&out, "probe_failed.spawn: true");
    let out = c
        .command(true)
        .env("TMPDIR", c.root.join("absent"))
        .output()
        .unwrap();
    has(&out, "probe_failed.scratch: true");
    symlink(c.root.join("tmp"), c.home.join("directory-link")).unwrap();
    let out = c.run(true);
    has(&out, "probe_failed.listing: true");
    let c = Case::new(json!({"version":"not-a-version"}));
    let out = c.run(true);
    has(&out, "codex.version_found: none");
}
#[test]
fn vanished_listing_entry_is_absent() {
    let c = Case::new(json!({}));
    fs::write(c.home.join("vanishing"), b"").unwrap();
    let result = Listing::read_observed(&c.home, |path, stage| {
        if path == Path::new("vanishing") && stage == "before_stat" {
            fs::remove_file(c.home.join(path)).unwrap();
        }
    });
    assert!(result.is_ok(), "a vanished entry failed the listing");
}

#[test]
fn settings_and_write_failures_are_classified() {
    use usage_guard_probe::{failure::Step, process::Scratch};
    let c = Case::new(json!({}));
    if std::env::var_os("SETTINGS_CASE_CHILD").is_none() {
        let out = Command::new(std::env::current_exe().unwrap())
            .env_clear()
            .env("HOME", c.root.join("base"))
            .env("TMPDIR", c.root.join("tmp"))
            .env("SETTINGS_CASE_CHILD", "1")
            .args([
                "--exact",
                "fixes::settings_and_write_failures_are_classified",
            ])
            .output()
            .unwrap();
        assert!(out.status.success());
        return;
    }
    let home = PathBuf::from(std::env::var_os("HOME").unwrap())
        .parent()
        .unwrap()
        .join("home");
    let root = ScratchRoot::validate(&home).unwrap();
    let scratch = Scratch::new(&root).unwrap();
    fs::create_dir(scratch.0.join("settings.json")).unwrap();
    let error = scratch.write_settings().unwrap_err();
    assert_eq!(
        error
            .get_ref()
            .unwrap()
            .downcast_ref::<Step>()
            .unwrap()
            .name(),
        "settings"
    );
    struct Broken;
    impl std::io::Write for Broken {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::ErrorKind::BrokenPipe.into())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let error = send::claude(&mut Broken, "/usage").unwrap_err();
    assert_eq!(
        error
            .get_ref()
            .unwrap()
            .downcast_ref::<Step>()
            .unwrap()
            .name(),
        "write"
    );
}

fn identity(path: &Path) -> Option<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path).ok().map(|m| (m.dev(), m.ino()))
}
fn other_case(name: &std::ffi::OsStr) -> std::ffi::OsString {
    name.to_string_lossy().to_uppercase().into()
}

/// The scratch root is refused by directory identity, never by spelling. On a case-insensitive
/// filesystem (the Mac's default APFS) each alias below is a case alias: the home's own name,
/// or an ancestor's, in another case. On a case-sensitive one no case alias can be formed, so the
/// same forms are built from a symlink spelled in the other case, which still reaches the home by
/// identity alone; the test says which it ran. Those forms alone cannot tell identity from
/// spelling, since the probe canonicalizes both paths and macOS returns the on-disk case. The
/// firmlink forms can: macOS leaves the data volume's spelling of a directory under `/Users` as
/// given, so the two paths differ in spelling and match only by identity. On macOS they must run.
#[test]
fn scratch_identity_alias_is_refused() {
    let probe = Case::new(json!({}));
    let upper = probe.root.join(other_case(std::ffi::OsStr::new("home")));
    let insensitive = identity(&upper).is_some() && identity(&upper) == identity(&probe.home);
    eprintln!(
        "scratch_identity_alias_is_refused: {}",
        if insensitive {
            "case-insensitive filesystem, case aliases"
        } else {
            "case-sensitive filesystem, symlink aliases spelled in the other case"
        }
    );
    let mut ran = Vec::new();
    for form in [
        "direct",
        "nested",
        "home_direct",
        "home_nested",
        "ancestor",
        "firmlink_direct",
        "firmlink_nested",
        "firmlink_home",
    ] {
        let c = Case::new(json!({}));
        fs::create_dir(c.home.join("temp")).unwrap();
        let home_alias = c.root.join(other_case(c.home.file_name().unwrap()));
        let root_alias = c
            .root
            .parent()
            .unwrap()
            .join(other_case(c.root.file_name().unwrap()));
        if !insensitive {
            symlink(&c.home, &home_alias).unwrap();
            symlink(&c.root, &root_alias).unwrap();
        }
        // The other way in: a directory under the home is matched by identity, not by name.
        let open = |path: &Path| {
            usage_guard_probe::directory::identity(
                &usage_guard_probe::directory::Directory::open(&path.canonicalize().unwrap())
                    .unwrap()
                    .own_metadata()
                    .unwrap(),
            )
        };
        let listing = Listing::read(&c.home.canonicalize().unwrap()).unwrap();
        assert!(listing.holds_any_directory(&[open(&home_alias.join("temp"))]));
        assert!(!listing.holds_any_directory(&[open(&c.root.join("tmp"))]));
        let (home, temporary) = match form {
            "direct" => (c.home.clone(), home_alias.clone()),
            "nested" => (c.home.clone(), home_alias.join("temp")),
            "home_direct" => (home_alias.clone(), c.home.clone()),
            "home_nested" => (home_alias.clone(), c.home.join("temp")),
            "ancestor" => (c.home.clone(), root_alias.join("home").join("temp")),
            _ => {
                let data = Path::new("/System/Volumes/Data")
                    .join(c.home.canonicalize().unwrap().strip_prefix("/").unwrap());
                let firmlink = identity(&data).is_some() && identity(&data) == identity(&c.home);
                if !firmlink {
                    // A regression to spelling comparison must fail where the sitting runs.
                    if cfg!(target_os = "macos") {
                        panic!("{form}: no data-volume spelling of the home on macOS");
                    }
                    continue;
                }
                assert_ne!(
                    data.canonicalize().unwrap(),
                    c.home.canonicalize().unwrap(),
                    "{form}: the spellings do not differ after canonicalizing"
                );
                match form {
                    "firmlink_direct" => (c.home.clone(), data.clone()),
                    "firmlink_nested" => (c.home.clone(), data.join("temp")),
                    _ => (data.clone(), c.home.join("temp")),
                }
            }
        };
        assert_eq!(
            identity(&temporary),
            identity(
                &c.home
                    .join(if form.ends_with("direct") { "" } else { "temp" })
            )
        );
        for codex in [false, true] {
            let out = c
                .command_with(codex, "--home", Some(home.to_str().unwrap()))
                .env("TMPDIR", &temporary)
                .output()
                .unwrap();
            assert!(
                c.logs().is_empty(),
                "{form}: a scratch root inside the home started a client"
            );
            assert_eq!(out.status.code(), Some(1), "{form}");
            has(&out, "probe_failed.scratch: true");
            assert!(!text(&out).contains(c.root.to_str().unwrap()));
            for dir in [&c.home, &c.home.join("temp")] {
                assert!(
                    fs::read_dir(dir).unwrap().all(|e| !e
                        .unwrap()
                        .file_name()
                        .to_string_lossy()
                        .starts_with("usage-probe-")),
                    "{form}: a scratch directory was made inside the home"
                );
            }
        }
        ran.push(form);
    }
    eprintln!("scratch_identity_alias_is_refused: ran {}", ran.join(", "));
    if cfg!(target_os = "macos") {
        assert_eq!(ran.len(), 8, "a form was skipped on macOS: {ran:?}");
    }
}

/// A sink every write to which fails: the write end of a pipe whose read end is already closed
/// (EPIPE), or a file opened for reading only (EBADF).
fn failing_sink(c: &Case, sink: &str) -> Stdio {
    if sink == "closed_pipe" {
        let (reader, writer) = std::io::pipe().unwrap();
        drop(reader);
        Stdio::from(writer)
    } else {
        let path = c.root.join("read-only-sink");
        fs::write(&path, b"").unwrap();
        Stdio::from(fs::File::open(path).unwrap())
    }
}

#[test]
fn output_failure_is_controlled() {
    for sink in ["closed_pipe", "read_only_file"] {
        for both in [false, true] {
            for codex in [true, false] {
                let c = Case::new(json!({}));
                let mut cmd = c.command(codex);
                cmd.stdout(failing_sink(&c, sink));
                if both {
                    cmd.stderr(failing_sink(&c, sink));
                } else {
                    cmd.stderr(Stdio::piped());
                }
                let out = cmd.spawn().unwrap().wait_with_output().unwrap();
                drop(cmd);
                assert_eq!(
                    out.status.code(),
                    Some(1),
                    "{sink} both={both} codex={codex}: not a controlled exit 1"
                );
                if !both {
                    assert_eq!(
                        String::from_utf8(out.stderr).unwrap(),
                        "probe_failed.output: true\n"
                    );
                }
                c.clean_scratch();
                // The first line fails, so no client starts after the version check's.
                assert_eq!(
                    c.logs().len(),
                    1,
                    "{sink}: a client started after the failure"
                );
            }
        }
    }
}

#[test]
fn output_closed_mid_run_stops_cleanly() {
    use std::io::BufRead;
    let c = Case::new(json!({"descendant":true}));
    let mut cmd = c.command_with(true, "--hold-seconds", Some("1"));
    let (reader, writer) = std::io::pipe().unwrap();
    let child = cmd.stdout(writer).stderr(Stdio::piped()).spawn().unwrap();
    drop(cmd);
    let mut lines = std::io::BufReader::new(reader).lines();
    // Read through the version check's lines, then close the pipe while read 1 is held.
    for line in lines.by_ref() {
        if line.unwrap() == "codex.version.group_stopped: true" {
            break;
        }
    }
    drop(lines);
    let out = child.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(1), "not a controlled exit 1");
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "probe_failed.output: true\n"
    );
    // Read 1 ran and was stopped and reaped with its TERM-ignoring descendant; nothing after it.
    assert_eq!(c.logs().len(), 2);
    c.clean_scratch();
    descendants_gone(&c);
}

#[test]
fn output_sink_failure_is_recorded() {
    use std::cell::{Cell, RefCell};
    use std::io::Write;
    use std::rc::Rc;
    struct Broken(Rc<Cell<usize>>);
    impl Write for Broken {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            self.0.set(self.0.get() + 1);
            Err(std::io::ErrorKind::BrokenPipe.into())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    struct Kept(Rc<RefCell<Vec<u8>>>);
    impl Write for Kept {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.borrow_mut().extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    // Standard output fails; standard error still takes the one fixed line at the end.
    let tries = Rc::new(Cell::new(0));
    let kept = Rc::new(RefCell::new(Vec::new()));
    let mut out = usage_guard_probe::Output::to(Broken(tries.clone()), Kept(kept.clone()));
    out.value("first", true);
    assert!(out.failed);
    out.value("second", true);
    out.write("third: true", true);
    assert_eq!(tries.get(), 1, "a write was tried after a failed one");
    assert!(kept.borrow().is_empty());
    assert!(!out.finish());
    assert_eq!(kept.borrow().as_slice(), b"probe_failed.output: true\n");
    // Both fail: nothing panics, and the end is still false.
    let tries = Rc::new(Cell::new(0));
    let mut out = usage_guard_probe::Output::to(Broken(tries.clone()), Broken(tries.clone()));
    out.write("first: true", true);
    out.value("second", true);
    assert!(!out.finish());
    assert_eq!(tries.get(), 2);
}

#[test]
fn panic_hook_never_panics() {
    if std::env::var_os("PANIC_HOOK_CHILD").is_some() {
        std::panic::set_hook(Box::new(|_| usage_guard_probe::panicked()));
        let caught = std::panic::catch_unwind(|| panic!("fixed"));
        let _ = std::panic::take_hook();
        assert!(caught.is_err());
        return;
    }
    // A hook that panicked would turn this caught panic into an abort of the child. The child
    // runs with --nocapture, so a hook writing through std's stderr reaches the real descriptor.
    for sink in ["closed_pipe", "read_only_file", "captured"] {
        let c = Case::new(json!({}));
        let stderr = if sink == "captured" {
            Stdio::piped()
        } else {
            failing_sink(&c, sink)
        };
        let out = Command::new(std::env::current_exe().unwrap())
            .env_clear()
            .env("HOME", c.root.join("base"))
            .env("TMPDIR", c.root.join("tmp"))
            .env("PANIC_HOOK_CHILD", "1")
            .args(["--exact", "fixes::panic_hook_never_panics", "--nocapture"])
            .stdout(Stdio::piped())
            .stderr(stderr)
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(0),
            "{sink}: the hook's write panicked"
        );
        if sink == "captured" {
            assert_eq!(String::from_utf8(out.stderr).unwrap(), "panicked: true\n");
        }
    }
}
