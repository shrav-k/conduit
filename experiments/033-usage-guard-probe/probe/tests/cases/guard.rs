use super::*;
use usage_guard_probe::failure::Step;
use usage_guard_probe::process::{Client, Launch, ScratchRoot};

/// A Claude client that stays alive after its result and then shows a sign of model work, or
/// prints a line that is not JSON: the probe keeps reading while it waits for exit, stops the
/// group at that line, and says so. A long hold shows the stop did not wait for the hold's end.
#[test]
fn claude_sign_after_the_result_stops_the_client() {
    let result = json!({"type":"result","duration_api_ms":0,"usage":{},"result":""});
    for (name, events, raw_lines, raw_bytes, delay, hold, key) in [
        (
            "delayed stream_event",
            json!([result, {"type":"stream_event"}]),
            json!(null),
            json!(null),
            300,
            "20",
            "claude.read.stopped_on_model_work: true",
        ),
        (
            "queued stream_event",
            json!([result, {"type":"stream_event"}]),
            json!(null),
            json!(null),
            0,
            "20",
            "claude.read.stopped_on_model_work: true",
        ),
        (
            "delayed api_retry",
            json!([result, {"type":"system","subtype":"api_retry"}]),
            json!(null),
            json!(null),
            300,
            "20",
            "claude.read.stopped_on_model_work: true",
        ),
        (
            "delayed assistant tokens",
            json!([result, {"type":"assistant","message":{"usage":{"output_tokens":3}}}]),
            json!(null),
            json!(null),
            300,
            "20",
            "claude.read.stopped_on_model_work: true",
        ),
        (
            "stream_event with no hold",
            json!([result, {"type":"stream_event"}]),
            json!(null),
            json!(null),
            0,
            "0",
            "claude.read.stopped_on_model_work: true",
        ),
        (
            "line not JSON",
            json!([result]),
            json!(["not-json"]),
            json!(null),
            300,
            "20",
            "claude.read.stopped_on_unreadable_line: true",
        ),
        (
            "line not UTF-8",
            json!([result]),
            json!(null),
            json!([255, 254, 10]),
            300,
            "20",
            "claude.read.stopped_on_unreadable_line: true",
        ),
    ] {
        let c = Case::new(json!({
            "events": events,
            "raw_lines": raw_lines,
            "raw_bytes": raw_bytes,
            "event_delay_ms": delay,
            "never": true,
            "descendant": true,
        }));
        let start = Instant::now();
        let out = c
            .command_with(false, "--hold-seconds", Some(hold))
            .output()
            .unwrap();
        assert!(
            start.elapsed() < Duration::from_secs(8),
            "{name}: the stop waited for the hold's end"
        );
        assert_eq!(out.status.code(), Some(1), "{name}");
        has(&out, "claude.read.answered: true");
        has(&out, key);
        if key.contains("model_work") {
            has(&out, "claude.read.model_work: true");
        }
        has(&out, "claude.read.group_stopped: true");
        assert!(!text(&out).contains("not-json"), "{name}");
        c.clean_scratch();
        descendants_gone(&c);
    }
}

/// A client that exits after its result is not held for the whole hold, and nothing it did
/// is taken for a sign.
#[test]
fn claude_quiet_exit_after_the_result_is_clean() {
    let c = Case::new(json!({}));
    let start = Instant::now();
    let out = c
        .command_with(false, "--hold-seconds", Some("20"))
        .output()
        .unwrap();
    assert!(start.elapsed() < Duration::from_secs(8));
    success(&out);
    has(&out, "claude.read.model_work: false");
    assert!(!text(&out).contains("stopped_on"));
}

fn step(error: &std::io::Error) -> &'static str {
    error
        .get_ref()
        .and_then(|e| e.downcast_ref::<Step>())
        .map_or("none", |s| s.name())
}

/// After SIGINT, SIGTERM or SIGHUP no client starts and no message is written to one, even when
/// the signal came while the probe was busy elsewhere, such as listing the home.
#[test]
fn interrupt_withholds_spawn_and_writes() {
    let c = Case::new(json!({}));
    if std::env::var_os("INTERRUPT_CASE_CHILD").is_none() {
        for signal in ["INT", "TERM", "HUP"] {
            let out = Command::new(std::env::current_exe().unwrap())
                .env_clear()
                .env("HOME", c.root.join("base"))
                .env("TMPDIR", c.root.join("tmp"))
                .env("INTERRUPT_CASE_CHILD", signal)
                .args(["--exact", "guard::interrupt_withholds_spawn_and_writes"])
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "{signal}: {} {}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
        }
        return;
    }
    let home = PathBuf::from(std::env::var_os("HOME").unwrap())
        .parent()
        .unwrap()
        .join("home");
    let root = ScratchRoot::validate(&home).unwrap();
    usage_guard_probe::process::install_signals();
    let signal = match std::env::var("INTERRUPT_CASE_CHILD").unwrap().as_str() {
        "INT" => libc::SIGINT,
        "TERM" => libc::SIGTERM,
        _ => libc::SIGHUP,
    };
    assert_eq!(unsafe { libc::raise(signal) }, 0);
    assert!(usage_guard_probe::process::interrupted());
    for codex in [true, false] {
        let spawned = Client::spawn(
            &root,
            Path::new(if codex {
                env!("CARGO_BIN_EXE_fake-codex")
            } else {
                env!("CARGO_BIN_EXE_fake-claude")
            }),
            &home,
            &["app-server".into()],
            Launch {
                codex,
                deadline: Duration::from_secs(2),
                version: false,
                settings: !codex,
            },
        );
        let error = spawned.err().expect("a client started after an interrupt");
        assert_eq!(step(&error), "interrupted");
    }
    let parent = home.parent().unwrap();
    assert!(
        fs::read_dir(parent).unwrap().all(|e| !e
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with("log-")),
        "a client ran after an interrupt"
    );
    assert_eq!(fs::read_dir(parent.join("tmp")).unwrap().count(), 0);
    let mut bytes = Vec::new();
    let error = send::claude(&mut bytes, "/usage").unwrap_err();
    assert_eq!(step(&error), "interrupted");
    for message in [
        send::initialize(),
        send::initialized(),
        send::rate_limits_read(),
    ] {
        let error = send::codex(&mut bytes, &message).unwrap_err();
        assert_eq!(step(&error), "interrupted");
    }
    assert!(bytes.is_empty(), "a message was written after an interrupt");
}

/// The gate passes the three messages only as the probe sends them, whole: no other id,
/// parameter, capability or field passes, for `initialize` and `initialized` as for the read.
#[test]
fn codex_gate_compares_whole_messages() {
    let allowed = [
        json!({"id":1,"method":"initialize","params":{
            "clientInfo":{"name":"usage-guard-probe","version":"0.1.0"},
            "capabilities":{"experimentalApi":false}}}),
        json!({"method":"initialized"}),
        json!({"id":2,"method":"account/rateLimits/read",
            "params":{"excludeResetCreditDetails":true}}),
    ];
    assert_eq!(
        allowed,
        [
            send::initialize(),
            send::initialized(),
            send::rate_limits_read()
        ]
    );
    for message in &allowed {
        let mut bytes = Vec::new();
        send::codex(&mut bytes, message).unwrap();
        assert_eq!(bytes, format!("{message}\n").into_bytes());
    }
    let modified = |index: usize, change: &dyn Fn(&mut Value)| {
        let mut message = allowed[index].clone();
        change(&mut message);
        message
    };
    for message in [
        modified(0, &|m| {
            m["params"]["capabilities"]["experimentalApi"] = json!(true)
        }),
        modified(0, &|m| {
            m["params"]["capabilities"]["optOutNotificationMethods"] = json!([])
        }),
        modified(0, &|m| m["params"]["clientInfo"]["name"] = json!("other")),
        modified(0, &|m| m["params"]["clientInfo"]["title"] = json!("other")),
        modified(0, &|m| m["id"] = json!(3)),
        modified(0, &|m| {
            m.as_object_mut().unwrap().remove("params");
        }),
        modified(0, &|m| {
            m.as_object_mut().unwrap().remove("id");
        }),
        modified(0, &|m| m["jsonrpc"] = json!("2.0")),
        modified(1, &|m| m["params"] = json!({})),
        modified(1, &|m| m["id"] = json!(1)),
        modified(2, &|m| m["id"] = json!(3)),
        modified(2, &|m| {
            m.as_object_mut().unwrap().remove("id");
        }),
        modified(2, &|m| m["extra"] = json!(true)),
    ] {
        let mut bytes = Vec::new();
        assert!(
            send::codex(&mut bytes, &message).is_err(),
            "the gate passed {message}"
        );
        assert!(bytes.is_empty());
    }
}

/// A sign printed after the wait for exit has ended, or after the spawn deadline, while the
/// group is being stopped (the client ignores TERM, so the stop waits its grace) is still read
/// once the group is gone. The hold is the cases' default of 0.
#[test]
fn claude_sign_during_the_stop_is_seen() {
    for (name, script, deadline, answered) in [
        (
            "after a zero hold",
            json!({"events":[{"type":"result","duration_api_ms":0},{"type":"stream_event"}],
                "event_delay_ms":300,"never":true,"ignore_term":true}),
            "5",
            true,
        ),
        (
            "after the deadline",
            json!({"events":[{"type":"stream_event"}],"answer_delay_ms":1300,
                "never":true,"ignore_term":true}),
            "1",
            false,
        ),
    ] {
        let c = Case::new(script);
        let out = c
            .command_with(false, "--deadline-seconds", Some(deadline))
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(1), "{name}");
        has(&out, &format!("claude.read.answered: {answered}"));
        has(&out, "claude.read.model_work: true");
        has(&out, "claude.read.stopped_on_model_work: true");
        has(&out, "claude.read.group_stopped: true");
        c.clean_scratch();
    }
}

/// A line that arrives after the spawn deadline is no answer, but `line()` keeps it for the
/// drain rather than discarding it, so a sign in it is still read after the stop.
#[test]
fn late_line_is_kept_for_the_drain() {
    let c = Case::new(json!({"events":[{"type":"system"},{"type":"stream_event"}],
        "event_delay_ms":1500,"never":true}));
    if std::env::var_os("LATE_CASE_CHILD").is_none() {
        let out = Command::new(std::env::current_exe().unwrap())
            .env_clear()
            .env("HOME", c.root.join("base"))
            .env("TMPDIR", c.root.join("tmp"))
            .env("LATE_CASE_CHILD", "1")
            .args(["--exact", "guard::late_line_is_kept_for_the_drain"])
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{} {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        return;
    }
    // Use the parent's marked root and script.
    let home = PathBuf::from(std::env::var_os("HOME").unwrap())
        .parent()
        .unwrap()
        .join("home");
    let root = ScratchRoot::validate(&home).unwrap();
    let mut client = Client::spawn(
        &root,
        Path::new(env!("CARGO_BIN_EXE_fake-claude")),
        &home,
        &[],
        Launch {
            codex: false,
            deadline: Duration::from_millis(1000),
            version: false,
            settings: false,
        },
    )
    .unwrap();
    send::claude(client.input.as_mut().unwrap(), "/usage").unwrap();
    // Both events are queued, the second stamped after the deadline, before either is taken.
    thread::sleep(Duration::from_millis(2500));
    assert!(client.line().is_some(), "the timely event was lost");
    assert!(client.line().is_none(), "a late line was taken as timely");
    assert!(client.stop());
    let mut seen = Vec::new();
    client.drain(|line| {
        seen.push(line.text.trim_end().to_owned());
        false
    });
    assert_eq!(
        seen,
        [r#"{"type":"stream_event"}"#],
        "the late line was lost"
    );
}
