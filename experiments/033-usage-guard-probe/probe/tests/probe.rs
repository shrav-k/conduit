#![cfg(unix)]
use serde_json::{Value, json};
use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use usage_guard_probe::{fields, listing::Listing, send};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    root: PathBuf,
    home: PathBuf,
}
impl Case {
    fn new(script: Value) -> Self {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!(
                "test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(root.join("tmp")).unwrap();
        for dir in ["base", "home"] {
            fs::create_dir(root.join(dir)).unwrap();
        }
        fs::write(root.join("fake-root"), b"").unwrap();
        fs::write(root.join("fake-script.json"), script.to_string()).unwrap();
        let home = root.join("home");
        fs::write(home.join("auth.json"), b"fabricated-login").unwrap();
        Self { root, home }
    }
    fn command(&self, codex: bool) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_usage-probe"));
        cmd.env_clear()
            .env("HOME", self.root.join("base"))
            .env("TMPDIR", self.root.join("tmp"));
        cmd.arg(if codex { "codex" } else { "claude" })
            .arg("--home")
            .arg(&self.home)
            .arg(if codex { "--codex" } else { "--claude" })
            .arg(if codex {
                env!("CARGO_BIN_EXE_fake-codex")
            } else {
                env!("CARGO_BIN_EXE_fake-claude")
            })
            .args([
                "--expect-version",
                "1.2.3",
                "--hold-seconds",
                "0",
                "--deadline-seconds",
                "5",
            ]);
        if !codex {
            cmd.args(["--model", "included-test-model"]);
        }
        cmd
    }
    fn run(&self, codex: bool) -> Output {
        self.command(codex).output().unwrap()
    }
    fn logs(&self) -> Vec<Vec<Value>> {
        let mut paths: Vec<_> = fs::read_dir(&self.root)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.file_name().unwrap().to_string_lossy().starts_with("log-"))
            .collect();
        paths.sort();
        paths
            .into_iter()
            .map(|p| {
                fs::read_to_string(p)
                    .unwrap()
                    .lines()
                    .filter_map(|l| serde_json::from_str(l).ok())
                    .collect()
            })
            .filter(|v: &Vec<Value>| !v.is_empty())
            .collect()
    }
    fn command_with(&self, codex: bool, flag: &str, value: Option<&str>) -> Command {
        let original = self.command(codex);
        let args: Vec<_> = original.get_args().map(|s| s.to_os_string()).collect();
        let mut modified = Vec::new();
        let mut i = 0;
        while i < args.len() {
            if args[i] == flag {
                if let Some(v) = value {
                    modified.push(args[i].clone());
                    modified.push(v.into());
                }
                i += 2;
            } else {
                modified.push(args[i].clone());
                i += 1;
            }
        }
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_usage-probe"));
        cmd.env_clear()
            .env("HOME", self.root.join("base"))
            .env("TMPDIR", self.root.join("tmp"))
            .args(modified);
        cmd
    }
    fn clean_scratch(&self) {
        assert_eq!(fs::read_dir(self.root.join("tmp")).unwrap().count(), 0);
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
fn text(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).unwrap()
}
fn has(out: &Output, line: &str) {
    assert!(
        text(out).lines().any(|l| l == line),
        "missing expected line: {line}"
    );
}
fn success(out: &Output) {
    assert_eq!(out.status.code(), Some(0), "probe failed");
    assert!(out.stderr.is_empty());
}
fn ms(out: &Output, key: &str) -> u64 {
    text(out)
        .lines()
        .find_map(|l| l.strip_prefix(&format!("{key}: ")))
        .unwrap()
        .parse()
        .unwrap()
}

#[test]
fn codex_sends_only_the_three_methods() {
    let c = Case::new(json!({"notification":true}));
    let out = c.run(true);
    success(&out);
    let logs = c.logs();
    assert_eq!(logs.len(), 6);
    for log in logs.iter().filter(|l| l[0]["version"] == false) {
        assert_eq!(
            log[0]["args"],
            json!([
                "app-server",
                "--stdio",
                "-c",
                "cli_auth_credentials_store=\"file\""
            ])
        );
        let messages: Vec<_> = log.iter().skip(1).map(|v| v["message"].clone()).collect();
        assert_eq!(messages,json!([
            {"id":1,"method":"initialize","params":{"clientInfo":{"name":"usage-guard-probe","version":"0.1.0"},"capabilities":{"experimentalApi":false}}},
            {"method":"initialized"},{"id":2,"method":"account/rateLimits/read","params":{"excludeResetCreditDetails":true}}
        ]).as_array().unwrap().clone());
    }
    has(&out, "codex.read.2.same_fields: true");
    has(&out, "codex.read.3.same_fields: true");
    c.clean_scratch();
    let error = Case::new(json!({"error":{"code":-32001,"message":"never print this"}}));
    let out = error.run(true);
    success(&out);
    has(&out, "codex.read.1.error_answer: true");
    has(&out, "codex.read.1.error_code: -32001");
    assert!(!text(&out).contains("never print this"));
}
#[test]
fn refuses_any_other_method() {
    let c = Case::new(json!({}));
    success(&c.run(true));
    let mut child = Command::new(env!("CARGO_BIN_EXE_fake-codex"))
        .env_clear()
        .env("HOME", c.root.join("base"))
        .env("CODEX_HOME", &c.home)
        .arg("app-server")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    for method in [
        "account/read",
        "thread/start",
        "thread/resume",
        "turn/start",
        "turn/interrupt",
        "account/rateLimitResetCredit/consume",
        "account/sendAddCreditsNudgeEmail",
        "account/login/start",
        "account/login/cancel",
        "account/logout",
        "supportsLunaReserve",
        "unknown",
        "",
    ] {
        assert!(send::codex(&mut input, &json!({"id":2,"method":method,"params":{}})).is_err());
    }
    for params in [
        json!({}),
        json!(null),
        json!({"excludeResetCreditDetails":false}),
        json!({"excludeResetCreditDetails":true,"supportsLunaReserve":true}),
        json!({"supportsLunaReserve":true}),
    ] {
        assert!(
            send::codex(
                &mut input,
                &json!({"id":2,"method":"account/rateLimits/read","params":params})
            )
            .is_err()
        );
    }
    drop(input);
    assert!(child.wait().unwrap().success());
    assert_eq!(
        c.logs()
            .iter()
            .find(|l| l[0]["args"] == json!(["app-server"]))
            .unwrap()
            .len(),
        1
    );
    let mut bytes = Vec::new();
    assert!(send::claude(&mut bytes, "hello").is_err());
    assert!(bytes.is_empty());
}
#[test]
fn codex_environment_exact() {
    let c = Case::new(json!({}));
    let mut cmd = c.command(true);
    for key in ["LOGNAME", "PATH", "SHELL", "TERM", "USER"] {
        cmd.env(key, "test-base");
    }
    for key in [
        "OPENAI_API_KEY",
        "ANTHROPIC_API_KEY",
        "CODEX_REVOKE_TOKEN_URL_OVERRIDE",
    ] {
        cmd.env(key, ["fake", "-barred"].concat());
    }
    success(&cmd.output().unwrap());
    for log in c.logs() {
        assert_eq!(
            log[0]["variables"],
            json!([
                "CODEX_HOME",
                "CODEX_REFRESH_TOKEN_URL_OVERRIDE",
                "HOME",
                "LOGNAME",
                "PATH",
                "SHELL",
                "TERM",
                "USER"
            ])
        );
        assert_eq!(log[0]["blocked"], true);
        if log[0]["version"] == true {
            assert_eq!(log[0]["empty_home"], true);
            assert_eq!(log[0]["home_is_base"], true);
        }
    }
}
#[test]
fn codex_stops_after_login_change() {
    let c = Case::new(json!({"change":"rewrite"}));
    let out = c.run(true);
    assert_eq!(out.status.code(), Some(1));
    has(&out, "codex.stopped_on_login_change: true");
    has(&out, "home.login_file_unchanged: false");
    assert_eq!(c.logs().len(), 2);
    c.clean_scratch();
}
#[test]
fn fields_printed_by_rule() {
    let mut answer = serde_json::Map::new();
    for name in fields::CODEX_NAMES.split_whitespace() {
        answer.insert(name.into(), json!("unprinted"));
    }
    for (name, value) in [
        ("usedPercent", json!(12.5)),
        ("remainingPercent", json!(87.5)),
        ("resetsAt", json!("2030-01-01T01:02:03+00:00")),
        ("windowDurationMins", json!(300)),
        ("ordinaryUsageAllowed", json!(true)),
        ("rateLimitReachedType", json!("rate_limit_reached")),
        ("unexpected", json!(true)),
    ] {
        answer.insert(name.into(), value);
    }
    let raw = json!({"id":2,"result":answer}).to_string().replace(
        "\"rateLimitsByLimitId\":\"unprinted\"",
        "\"rateLimitsByLimitId\":{\"z-data\":{\"usedPercent\":11},\"a-data\":{\"usedPercent\":22}}",
    );
    let c = Case::new(json!({"raw_answer":raw}));
    let out = c.run(true);
    success(&out);
    for name in [
        "rateLimits",
        "rateLimitResetCredits",
        "accountId",
        "rateLimitUpsell",
        "limitId",
        "limitName",
        "normalModelSlug",
        "primary",
        "secondary",
        "credits",
        "hasCredits",
        "unlimited",
        "balance",
        "individualLimit",
        "limit",
        "used",
        "spendControlReached",
        "planType",
    ] {
        has(&out, &format!("codex.read.1.field {name}: string"));
    }
    for line in [
        "usedPercent: number 12.5",
        "remainingPercent: number 87.5",
        "resetsAt: string 2030-01-01T01:02:03+00:00",
        "windowDurationMins: number 300",
        "ordinaryUsageAllowed: boolean true",
        "rateLimitReachedType: string word:true",
        "?: boolean true",
        "rateLimitsByLimitId[1].usedPercent: number 11",
        "rateLimitsByLimitId[2].usedPercent: number 22",
    ] {
        has(&out, &format!("codex.read.1.field {line}"));
    }
    assert!(!text(&out).contains("z-data"));
    has(&out, "codex.read.1.field rateLimitsByLimitId: object");
    assert!(!text(&out).contains("a-data"));
    let mut report = serde_json::Map::new();
    for name in fields::CLAUDE_NAMES.split_whitespace() {
        report.insert(name.into(), json!(false));
    }
    for (key, val) in [
        ("kind", json!("weekly_scoped")),
        ("severity", json!("warning")),
        ("status", json!("allowed_warning")),
        ("rateLimitType", json!("overage")),
        ("resets_at", json!("2030-01-01T00:00:00.125Z")),
        ("percent", json!(33)),
        ("utilization", json!(44)),
    ] {
        report.insert(key.into(), val);
    }
    let c =
        Case::new(json!({"events":[{"type":"system","usage_report":report},{"type":"result"}]}));
    let out = c.run(false);
    success(&out);
    for name in [
        "message",
        "usage_report",
        "rate_limits",
        "limits",
        "group",
        "scope",
        "model",
        "surface",
        "is_active",
        "extra_usage",
        "is_enabled",
        "monthly_limit",
        "used_credits",
        "currency",
        "rate_limit_info",
        "resetsAt",
        "errorCode",
        "canUserPurchaseCredits",
        "hasChargeableSavedPaymentMethod",
        "overageEnabled",
        "overageStatus",
        "overageResetsAt",
        "overageDisabledReason",
        "isUsingOverage",
        "unifiedWindows",
        "five_hour",
        "seven_day",
    ] {
        has(
            &out,
            &format!("claude.read.usage_report.field {name}: boolean false"),
        );
    }
    for line in [
        "kind: string word:true",
        "severity: string word:true",
        "status: string word:true",
        "rateLimitType: string word:true",
        "percent: number 33",
        "utilization: number 44",
        "resets_at: string 2030-01-01T00:00:00.125Z",
    ] {
        has(&out, &format!("claude.read.usage_report.field {line}"));
    }
}
#[test]
fn barred_values_never_printed() {
    let barred = [
        ["probe", "@example.invalid"].concat(),
        ["acct", "-private-id"].concat(),
        "987654.321".into(),
        "876543.219".into(),
        [
            "eyJ",
            "hbGciOiJub25lIn0",
            ".",
            "eyJ",
            "zdWIiOiJmYWtlIn0",
            ".",
            "c2ln",
        ]
        .concat(),
        ["Cookie", ": fake=session"].concat(),
        ["Authorization", ": Bearer fabricated"].concat(),
    ];
    let c = Case::new(
        json!({"answer":{"accountId":barred[1],"balance":987654.321,"limit":876543.219,"limitName":barred[0],"planType":barred[4],"normalModelSlug":barred[5],"unknown":barred[6]},"stderr":barred.join("\n")}),
    );
    let out = c.run(true);
    success(&out);
    for value in &barred {
        assert!(!text(&out).contains(value));
    }
    assert!(!text(&out).contains(c.root.to_str().unwrap()));
    let c = Case::new(
        json!({"events":[{"type":"system","usage_report":{"monthly_limit":987654.321,"used_credits":876543.219,"model":barred[4],"currency":barred[0],"message":barred[6]}},{"type":"result","result":barred[5]}],"stderr":barred.join("\n")}),
    );
    let out = c.run(false);
    success(&out);
    for value in &barred {
        assert!(!text(&out).contains(value));
    }
}
#[test]
fn listing_detects_each_change() {
    for (action, line, code) in [
        ("rewrite", "home.changed auth.json: changed", 1),
        ("rename", "home.changed auth.json: changed", 1),
        ("time", "home.changed auth.json: changed", 1),
        ("restore_time", "home.changed auth.json: changed", 1),
        ("lock", "home.changed auth.lock: added", 1),
        ("remove", "home.changed ?: removed", 0),
        ("add", "home.changed ?: added", 0),
        ("deep", "home.changed state: changed", 0),
        ("hex", "home.changed ?: added", 0),
        ("none", "home.unchanged: true", 0),
    ] {
        let c = Case::new(json!({"change":action}));
        fs::write(c.home.join("remove-me"), b"").unwrap();
        fs::create_dir(c.home.join("state")).unwrap();
        fs::write(c.home.join("unreadable"), b"not opened").unwrap();
        fs::set_permissions(c.home.join("unreadable"), fs::Permissions::from_mode(0o000)).unwrap();
        let out = c.run(true);
        assert_eq!(out.status.code(), Some(code), "action {action}");
        has(&out, line);
        fs::set_permissions(c.home.join("unreadable"), fs::Permissions::from_mode(0o600)).unwrap();
    }
    let c = Case::new(json!({}));
    success(&c.run(true));
    fs::create_dir(c.root.join("outside")).unwrap();
    symlink(c.root.join("outside"), c.home.join("link")).unwrap();
    assert!(Listing::read(&c.home).is_err());
    fs::write(c.root.join("outside/entry"), b"outside").unwrap();
    assert!(Listing::read(&c.home).is_err());
}
#[test]
fn late_write_in_the_hold_detected() {
    let c = Case::new(json!({"late_write":true}));
    let out = c
        .command_with(true, "--hold-seconds", Some("4"))
        .output()
        .unwrap();
    success(&out);
    has(&out, "home.changed ?: added");
    has(&out, "home.login_file_unchanged: true");
    c.clean_scratch();
}
#[test]
fn durations_measured() {
    let c = Case::new(json!({"initialize_delay_ms":300,"answer_delay_ms":500}));
    let out = c.run(true);
    success(&out);
    let init = ms(&out, "codex.read.1.ms_to_initialized");
    let answer = ms(&out, "codex.read.1.ms_to_answer");
    let request = ms(&out, "codex.read.1.ms_request_to_answer");
    assert!((290..2500).contains(&init));
    assert!((790..4000).contains(&answer));
    assert!((490..2500).contains(&request));
    assert!(answer >= init + request - 5);
}
fn descendants_gone(c: &Case) {
    for item in fs::read_dir(&c.root).unwrap() {
        let item = item.unwrap();
        if item
            .file_name()
            .to_string_lossy()
            .starts_with("descendant-")
        {
            let pid: i32 = fs::read_to_string(item.path()).unwrap().parse().unwrap();
            assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::ESRCH)
            );
        }
    }
}
#[test]
fn deadline_stops_the_group() {
    let c = Case::new(json!({"never":true,"descendant":true}));
    let start = Instant::now();
    let out = c
        .command_with(true, "--deadline-seconds", Some("2"))
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(start.elapsed() < Duration::from_secs(7));
    has(&out, "codex.read.1.answered: false");
    has(&out, "codex.read.1.group_stopped: true");
    assert_eq!(c.logs().len(), 2);
    c.clean_scratch();
    descendants_gone(&c);
    for signal in [libc::SIGTERM, libc::SIGINT, libc::SIGHUP] {
        let c = Case::new(json!({"descendant":true}));
        let mut cmd = c.command_with(true, "--hold-seconds", Some("8"));
        let child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let until = Instant::now() + Duration::from_secs(4);
        while Instant::now() < until
            && !c.logs().iter().any(|l| {
                l.iter()
                    .any(|v| v["message"]["method"] == "account/rateLimits/read")
            })
        {
            thread::sleep(Duration::from_millis(20));
        }
        thread::sleep(Duration::from_millis(100));
        assert_eq!(unsafe { libc::kill(child.id() as i32, signal) }, 0);
        let out = child.wait_with_output().unwrap();
        assert_eq!(out.status.code(), Some(1));
        has(&out, "interrupted: true");
        has(&out, "codex.read.1.group_stopped: true");
        c.clean_scratch();
        descendants_gone(&c);
    }
}
#[test]
fn pair_overlaps() {
    let c = Case::new(json!({"answer_delay_ms":100}));
    let out = c.run(true);
    success(&out);
    for line in [
        "codex.pair.both_answered: true",
        "codex.pair.overlapped: true",
        "codex.pair.1.group_stopped: true",
        "codex.pair.2.group_stopped: true",
    ] {
        has(&out, line);
    }
    assert!(ms(&out, "codex.pair.2.ms_to_answer") >= 90);
    assert_eq!(c.logs().len(), 6);
}
#[test]
fn version_mismatch_runs_nothing() {
    for codex in [true, false] {
        let c = Case::new(json!({"version":"9.8.7-beta"}));
        let out = c.run(codex);
        assert_eq!(out.status.code(), Some(1));
        has(
            &out,
            if codex {
                "codex.version_matches: false"
            } else {
                "claude.version_matches: false"
            },
        );
        assert_eq!(c.logs().len(), 1);
        assert_eq!(c.logs()[0][0]["version"], true);
        c.clean_scratch();
    }
    let c = Case::new(json!({"version_delay_ms":3000}));
    let out = c
        .command_with(true, "--deadline-seconds", Some("1"))
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    has(&out, "codex.version_matches: false");
    has(&out, "codex.version_found: none");
    assert_eq!(c.logs().len(), 1);
}
#[test]
fn home_without_login_file_refused() {
    for (flag, value) in [
        ("--home", None),
        ("--codex", None),
        ("--expect-version", None),
        ("--home", Some("")),
        ("--codex", Some("")),
        ("--expect-version", Some("")),
        ("--codex", Some("relative-client")),
        ("--home", Some("missing-home")),
        ("--deadline-seconds", Some("0")),
        ("--deadline-seconds", Some("601")),
        ("--hold-seconds", Some("61")),
    ] {
        let c = Case::new(json!({}));
        let out = c.command_with(true, flag, value).output().unwrap();
        assert_eq!(out.status.code(), Some(1));
        assert_eq!(text(&out).lines().count(), 1);
        assert!(text(&out).starts_with("refused."));
        assert!(c.logs().is_empty());
    }
    let c = Case::new(json!({}));
    fs::remove_file(c.home.join("auth.json")).unwrap();
    let out = c.run(true);
    assert_eq!(out.status.code(), Some(1));
    has(&out, "refused.login_file: true");
    assert!(c.logs().is_empty());
    fs::remove_file(c.root.join("fake-root")).unwrap();
    let out = c.run(false);
    assert_eq!(out.status.code(), Some(1));
    assert!(c.logs().is_empty());
}
#[test]
fn claude_arguments_input_and_environment() {
    let c = Case::new(json!({}));
    let mut cmd = c.command(false);
    for name in [
        "OPENAI_API_KEY",
        "ANTHROPIC_API_KEY",
        "CODEX_REVOKE_TOKEN_URL_OVERRIDE",
        "CODEX_REFRESH_TOKEN_URL_OVERRIDE",
    ] {
        cmd.env(name, ["fake", "-barred"].concat());
    }
    let out = cmd.output().unwrap();
    success(&out);
    let logs = c.logs();
    assert_eq!(logs.len(), 2);
    for log in logs {
        assert_eq!(log[0]["variables"], json!(["CLAUDE_CONFIG_DIR", "HOME"]));
        assert_eq!(log[0]["blocked"], false);
        if log[0]["version"] == true {
            assert_eq!(log[0]["empty_home"], true);
            assert_eq!(log[0]["home_is_base"], true);
        } else {
            assert_eq!(
                log[0]["args"],
                json!([
                    "--print",
                    "--input-format",
                    "stream-json",
                    "--output-format",
                    "stream-json",
                    "--verbose",
                    "--include-partial-messages",
                    "--no-session-persistence",
                    "--setting-sources",
                    "",
                    "--strict-mcp-config",
                    "--tools",
                    "",
                    "--model",
                    "included-test-model",
                    "--settings",
                    "scratch/settings.json"
                ])
            );
            assert_eq!(log[0]["settings"], json!({"disableAllHooks":true}));
            assert_eq!(log.len(), 2);
            assert_eq!(
                log[1]["message"],
                json!({"type":"user","message":{"role":"user","content":"/usage"},"parent_tool_use_id":null,"session_id":""})
            );
        }
    }
    c.clean_scratch();
}
#[test]
fn claude_usage_report_read() {
    let c = Case::new(
        json!({"events":[{"type":"system","nested":{"usage_report":{"rate_limits":null,"extra_usage":{"is_enabled":false}}}},{"type":"rate_limit_event","rate_limit_info":{"status":"allowed","resetsAt":1900000000}},{"type":"result","duration_api_ms":0}]}),
    );
    let out = c.run(false);
    success(&out);
    for line in [
        "claude.read.answered: true",
        "claude.read.model_work: false",
        "claude.read.usage_report_present: true",
        "claude.read.rate_limits_null: true",
        "claude.read.terminal_reason_absent: true",
        "claude.read.ms_api: 0",
        "claude.read.group_stopped: true",
        "claude.read.usage_report.field rate_limits: null",
        "claude.read.rate_limit_info.field status: string word:true",
    ] {
        has(&out, line);
    }
    assert!(!text(&out).contains("home.login_file_unchanged"));
    assert!(ms(&out, "claude.read.ms_to_result") >= ms(&out, "claude.read.ms_to_report"));
    c.clean_scratch();
    let c = Case::new(
        json!({"events":[{"type":"system","usage_report":{"limits":[{"kind":"not-documented","percent":7},{"kind":"session","percent":8}]}},{"type":"result","terminal_reason":"complete"}]}),
    );
    let out = c.run(false);
    success(&out);
    has(&out, "claude.read.usage_report.field limits: array");
    has(
        &out,
        "claude.read.usage_report.field limits[1].kind: string word:false",
    );
    has(
        &out,
        "claude.read.usage_report.field limits[2].percent: number 8",
    );
    has(&out, "claude.read.terminal_reason_absent: false");
}
#[test]
fn claude_model_work_stops_the_client() {
    for event in [
        json!({"type":"stream_event"}),
        json!({"type":"system","subtype":"api_retry"}),
        json!({"type":"assistant","message":{"usage":{"input_tokens":1}}}),
        json!({"type":"result","total_cost_usd":0.01}),
        json!({"type":"result","duration_api_ms":1}),
        json!({"type":"result","modelUsage":{"test-model":{}}}),
        json!({"type":"result","usage":{"output_tokens":1}}),
    ] {
        let c = Case::new(json!({"events":[event],"never":true}));
        let start = Instant::now();
        let out = c.run(false);
        assert!(start.elapsed() < Duration::from_secs(3));
        assert_eq!(out.status.code(), Some(1));
        has(&out, "claude.read.model_work: true");
        has(&out, "claude.read.stopped_on_model_work: true");
        has(&out, "claude.read.group_stopped: true");
        c.clean_scratch();
    }
}
#[test]
fn claude_last_known_note() {
    for (events, expected) in [
        (
            json!([{"type":"assistant","message":{"content":[{"text":"Showing LAST-KNOWN usage"}]}},{"type":"result"}]),
            "claude.read.last_known_where: assistant",
        ),
        (
            json!([{"type":"result","result":"Showing last-known usage"}]),
            "claude.read.last_known_where: result",
        ),
    ] {
        let c = Case::new(json!({"events":events}));
        let out = c.run(false);
        success(&out);
        has(&out, "claude.read.last_known_note: true");
        has(&out, expected);
        assert!(!text(&out).contains("Showing"));
    }
    let c = Case::new(json!({}));
    let out = c.run(false);
    success(&out);
    has(&out, "claude.read.last_known_note: false");
    has(&out, "claude.read.last_known_where: none");
    let c = Case::new(
        json!({"events":[{"type":"notice","message":"Showing last-known usage"},{"type":"result"}]}),
    );
    let out = c.run(false);
    success(&out);
    has(&out, "claude.read.last_known_note: true");
    has(&out, "claude.read.last_known_where: other");
}
#[test]
fn claude_requires_model() {
    for value in [None, Some("")] {
        let c = Case::new(json!({}));
        let out = c.command_with(false, "--model", value).output().unwrap();
        assert_eq!(out.status.code(), Some(1));
        assert!(text(&out).starts_with("refused."));
        assert!(c.logs().is_empty());
    }
}
#[test]
fn output_lines_checked() {
    let c = Case::new(json!({}));
    let out = c.run(true);
    success(&out);
    assert!(text(&out).lines().all(usage_guard_probe::valid_line));
    if std::env::var_os("PROBE_OUTPUT_TEST_CHILD").is_some() {
        let mut out = usage_guard_probe::Output::default();
        for line in ["bad@line".to_owned(), "bad/line".into(), "x".repeat(201)] {
            out.write(&line, false);
            out.write(&line, true);
        }
        assert!(!out.finish());
        return;
    }
    let child = Command::new(std::env::current_exe().unwrap())
        .env_clear()
        .env("HOME", c.root.join("base"))
        .env("TMPDIR", c.root.join("tmp"))
        .env("PROBE_OUTPUT_TEST_CHILD", "1")
        .args(["--exact", "output_lines_checked", "--nocapture"])
        .output()
        .unwrap();
    assert!(child.status.success());
    let stdout = String::from_utf8(child.stdout).unwrap();
    let stderr = String::from_utf8(child.stderr).unwrap();
    assert!(stdout.contains("unprintable_lines: true"));
    for bad in ["bad@line".to_owned(), "bad/line".into(), "x".repeat(201)] {
        assert!(!stdout.contains(&bad));
        assert!(!stderr.contains(&bad));
    }
    let mut nested = json!({"usedPercent": 1});
    for _ in 0..24 {
        nested = json!({"rateLimits": nested});
    }
    let long = Case::new(json!({"answer": nested}));
    let out = long.run(true);
    assert_eq!(out.status.code(), Some(1));
    has(&out, "unprintable_lines: true");
    assert!(text(&out).lines().all(usage_guard_probe::valid_line));
    assert!(out.stderr.is_empty());
    assert!(!usage_guard_probe::valid_line(""));
    assert!(usage_guard_probe::valid_line(
        "prefix.field limits[1].percent: number 1.25"
    ));
}

#[path = "cases/fixes.rs"]
mod fixes;

#[path = "cases/guard.rs"]
mod guard;

#[path = "cases/safety.rs"]
mod safety;
