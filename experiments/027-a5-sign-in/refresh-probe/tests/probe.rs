use std::cell::RefCell;
use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use refresh_probe::{
    ACCOUNT, Arm, BLOCK, CLAIMS, Case, DISCOVERY, EMAIL, Form, Hooks, Login, ROTATION, Run,
    Scratch, Stub, output_path, report, sitting, today, unix_now, write_report,
};
use serde_json::{Value, json};

fn scratch() -> Scratch {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/test-scratch");
    fs::create_dir_all(&root).unwrap();
    Scratch::new(&root, "test").unwrap()
}

fn fake() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_fake-app-server"))
}

fn payload(jwt: &Value) -> Value {
    let Some(jwt) = jwt.as_str() else {
        panic!("JWT string missing")
    };
    let parts: Vec<&str> = jwt.split('.').collect();
    assert_eq!(parts.len(), 3);
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut bytes = Vec::new();
    let mut shift = 0;
    let mut buffer = 0_u32;
    for byte in parts[1].bytes() {
        let index = alphabet.iter().position(|&symbol| symbol == byte).unwrap();
        buffer = (buffer << 6) | index as u32;
        shift += 6;
        if shift >= 8 {
            shift -= 8;
            bytes.push((buffer >> shift) as u8);
        }
    }
    assert!(
        serde_json::from_slice::<Value>(&bytes).is_ok(),
        "JWT payload shape"
    );
    serde_json::from_slice(&bytes).unwrap()
}

fn login_data(home: &Path) -> Value {
    serde_json::from_slice(&fs::read(home.join("auth.json")).unwrap()).unwrap()
}

#[test]
fn login_shape() {
    let test = scratch();
    let now = unix_now();
    for (case, offset) in [(Case::P1, 180), (Case::P2, 86_400), (Case::P3, -600)] {
        let home = test.path().join(case.name());
        Login::new(case, now)
            .write(&home, Form::Issuer, "http://127.0.0.1")
            .unwrap();
        assert_eq!(
            fs::metadata(&home).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            fs::metadata(home.join("auth.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
        let data = login_data(&home);
        let tokens = &data["tokens"];
        assert_eq!(data["auth_mode"], "chatgpt");
        assert!(data["OPENAI_API_KEY"].is_null());
        assert!(
            data["last_refresh"]
                .as_str()
                .is_some_and(|value| value.ends_with('Z'))
        );
        assert_eq!(tokens["account_id"], ACCOUNT);
        assert!(tokens["refresh_token"].is_string());
        assert!(tokens["oauth_provenance"]["issuer"].is_string());
        assert!(tokens["oauth_provenance"]["client_id"].is_string());
        assert_eq!(
            payload(&tokens["id_token"]),
            json!({
                "email": EMAIL,
                (CLAIMS): {
                    "chatgpt_plan_type": "plus",
                    "chatgpt_user_id": "probe-208-user",
                    "chatgpt_account_id": ACCOUNT
                }
            })
        );
        assert_eq!(payload(&tokens["access_token"])["exp"], now + offset);
    }
    let home = test.path().join("variable");
    Login::new(Case::P1, now)
        .write(&home, Form::Variable, "http://127.0.0.1")
        .unwrap();
    assert!(
        login_data(&home)["tokens"]
            .get("oauth_provenance")
            .is_none()
    );
}

fn http(stub: &Stub, method: &str, path: &str, bearer: Option<&str>) -> (u16, Value) {
    let mut connection = TcpStream::connect(stub.address()).unwrap();
    write!(
        connection,
        "{method} {path} HTTP/1.1\r\nHost: localhost\r\n"
    )
    .unwrap();
    if let Some(bearer) = bearer {
        assert!(write!(connection, "Authorization: Bearer {bearer}\r\n").is_ok());
    }
    connection.write_all(b"Content-Length: 0\r\n\r\n").unwrap();
    let mut response = String::new();
    connection.read_to_string(&mut response).unwrap();
    let (header, body) = response.split_once("\r\n\r\n").unwrap();
    let status = header
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    assert!(header.lines().any(|line| line == "Connection: close"));
    (status, serde_json::from_str(body).unwrap())
}

#[test]
fn stub_answers() {
    let test = scratch();
    for (case, expected) in [(Case::P1, 200), (Case::P2, 401), (Case::P3, 401)] {
        let now = unix_now();
        let stub = Stub::start(Login::new(case, now)).unwrap();
        let home = test.path().join(case.name());
        Login::new(case, now)
            .write(&home, Form::Issuer, &stub.origin())
            .unwrap();
        let data = login_data(&home);
        let original = data["tokens"]["access_token"].as_str().unwrap();
        assert_eq!(http(&stub, "GET", DISCOVERY, Some(original)).0, expected);
        let (status, issued) = http(&stub, "POST", ROTATION, None);
        assert_eq!(status, 200);
        assert!(issued["refresh_token"].is_string());
        assert!(issued["id_token"] == data["tokens"]["id_token"]);
        let bearer = issued["access_token"].as_str().unwrap();
        let exp = payload(&issued["access_token"])["exp"].as_i64().unwrap();
        assert!((now + 86_400..=unix_now() + 86_400).contains(&exp));
        assert_eq!(http(&stub, "GET", DISCOVERY, Some(bearer)).0, 200);
        assert_eq!(http(&stub, "GET", DISCOVERY, Some("unrecognized")).0, 200);
        assert_eq!(http(&stub, "GET", "/something?query=omitted", None).0, 404);
        let log = stub.finish().unwrap();
        for (entry, (method, path, status, bearer)) in log.iter().zip([
            ("GET", DISCOVERY, expected, "original"),
            ("POST", ROTATION, 200, "none"),
            ("GET", DISCOVERY, 200, "issued"),
            ("GET", DISCOVERY, 200, "other"),
            ("GET", "/something", 404, "none"),
        ]) {
            assert_eq!(
                *entry,
                json!({"method":method, "path":path, "status":status, "bearer":bearer})
            );
        }
        assert_eq!(log.len(), 5);
        let log = serde_json::to_string(&log).unwrap();
        for field in ["access_token", "id_token", "refresh_token"] {
            assert!(!log.contains(data["tokens"][field].as_str().unwrap()));
            assert!(!log.contains(issued[field].as_str().unwrap()));
        }
    }
}

struct Observed {
    report: Value,
    record: Vec<Value>,
    pids: Vec<i32>,
}

fn observe(script: Value, hold: Duration) -> Observed {
    let seen = attempt(script, hold);
    assert!(
        seen.report["fault"].is_null(),
        "fake run fault: {}",
        seen.report["fault"]
    );
    seen
}

fn attempt(script: Value, hold: Duration) -> Observed {
    let scratch = scratch();
    let logs = scratch.path().join("logs");
    fs::create_dir(&logs).unwrap();
    let record = RefCell::new(Vec::new());
    let pids = RefCell::new(Vec::new());
    let hooks = Hooks {
        prepare: &|_, home, _| fs::write(home.join("fake-script.json"), script.to_string()),
        inspect: &|home| {
            *record.borrow_mut() = fs::read_to_string(home.join("fake-record.jsonl"))?
                .lines()
                .map(serde_json::from_str)
                .collect::<Result<_, _>>()?;
            *pids.borrow_mut() = fs::read_to_string(home.join("fake-pid"))?
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .map_err(io::Error::other)?;
            Ok(())
        },
    };
    let report = Run {
        client: fake(),
        scratch: scratch.path(),
        logs: &logs,
        case: Case::P1,
        arm: Arm::Blocked,
        repetition: 1,
        form: Form::Issuer,
        hold,
    }
    .execute(Some(&hooks));
    Observed {
        report,
        record: record.into_inner(),
        pids: pids.into_inner(),
    }
}

#[test]
fn exchange_as_the_hook_sends_it() {
    let seen = observe(json!({"notification":"account/updated"}), Duration::ZERO);
    assert_eq!(
        seen.record[1..],
        [
            json!({"id":1,"method":"initialize","params":{
                "clientInfo":{"name":"agent-codex","version":"0.2.0"},
                "capabilities":{"experimentalApi":false}
            }}),
            json!({"method":"initialized"}),
            json!({"id":2,"method":"account/read","params":{"refreshToken":false}}),
        ]
    );
    assert_eq!(seen.report["skipped_methods"], json!(["account/updated"]));
}

#[test]
fn detects_rewrite_in_place() {
    let seen = observe(json!({"action":"rewrite"}), Duration::ZERO).report;
    assert_eq!(seen["wrote"], true);
    assert_eq!(seen["login"]["bytes_equal"], false);
    assert_eq!(seen["login"]["inode_equal"], true);
}

#[test]
fn detects_rename_with_same_bytes() {
    let seen = observe(json!({"action":"rename"}), Duration::ZERO).report;
    assert_eq!(seen["wrote"], true);
    assert_eq!(seen["login"]["bytes_equal"], true);
    assert_eq!(seen["login"]["mtime_equal"], true);
    assert_eq!(seen["login"]["inode_equal"], false);
}

#[test]
fn detects_mtime_only() {
    let seen = observe(json!({"action":"mtime"}), Duration::ZERO).report;
    assert_eq!(seen["wrote"], true);
    assert_eq!(seen["login"]["bytes_equal"], true);
    assert_eq!(seen["login"]["mtime_equal"], false);
    assert_eq!(seen["login"]["inode_equal"], true);
}

#[test]
fn detects_removal() {
    let seen = observe(json!({"action":"remove"}), Duration::ZERO).report;
    assert_eq!(seen["wrote"], true);
    assert_eq!(seen["login"]["present_after"], false);
}

#[test]
fn detects_lock_alone() {
    for name in ["auth.lock", "auth.json.lock"] {
        let seen = observe(json!({"action":name}), Duration::ZERO).report;
        assert_eq!(seen["began_save"], true);
        assert_eq!(seen["locks_appeared"][name], true);
        assert_eq!(seen["wrote"], false);
    }
}

#[test]
fn no_change() {
    let seen = observe(json!({"action":"directory"}), Duration::ZERO).report;
    assert_eq!(seen["wrote"], false);
    assert_eq!(seen["began_save"], false);
    assert_eq!(seen["home_added"], json!(["tmp", "tmp/x"]));
}

#[test]
fn late_write_inside_the_hold() {
    let seen = observe(
        json!({"action":"rewrite","delay_ms":1000}),
        Duration::from_secs(3),
    )
    .report;
    assert_eq!(seen["wrote"], true);
}

#[test]
fn term_ignored_then_killed() {
    // The production namespace reaps orphans. On Linux the test needs to do
    // the same, since the test runner's PID 1 may leave zombies behind.
    #[cfg(target_os = "linux")]
    assert_eq!(
        unsafe { libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0) },
        0
    );
    let seen = attempt(json!({"ignore_term":true}), Duration::ZERO);
    let [leader, descendant] = seen.pids[..] else {
        panic!("fake server did not confirm two processes in the client group");
    };
    assert_ne!(leader, descendant);
    let gone = |pid| {
        let status = unsafe { libc::kill(pid, 0) };
        status == -1 && io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
    };
    let descendant_gone = gone(descendant);
    if !descendant_gone {
        // Clean up even when a regression leaves the fake descendant running.
        unsafe {
            libc::kill(descendant, libc::SIGKILL);
            libc::waitpid(descendant, std::ptr::null_mut(), 0);
        }
    }
    #[cfg(target_os = "linux")]
    assert_eq!(
        unsafe { libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 0, 0, 0, 0) },
        0
    );
    assert!(gone(leader));
    assert!(descendant_gone);
    assert!(gone(-leader));
    assert!(seen.report["fault"].is_null());
    assert_eq!(seen.report["ended_on"], "kill");
}

#[test]
fn answer_recorded_exactly() {
    let test = scratch();
    let hooks = Hooks {
        prepare: &|run, home, addr| {
            let message =
                serde_json::to_string(&format!("look in {}/tmp at {addr}, please", run.display()))?;
            let line = format!(r#" {{ "id":2, "error":{{ "code":-1, "message":{message} }} }} "#);
            fs::write(
                home.join("fake-script.json"),
                json!({"account_line":line}).to_string(),
            )
        },
        inspect: &|_| Ok(()),
    };
    let run = Run {
        client: fake(),
        scratch: test.path(),
        logs: test.path(),
        case: Case::P1,
        arm: Arm::Blocked,
        repetition: 1,
        form: Form::Issuer,
        hold: Duration::ZERO,
    };
    let report = run.execute(Some(&hooks));
    assert_eq!(report["answer"]["kind"], "error");
    assert_eq!(
        report["answer"]["line"],
        r#" { "id":2, "error":{ "code":-1, "message":"look in <run>/tmp at 127.0.0.1:<port>, please" } } "#
    );
    assert_eq!(
        report["answer"]["error_message"],
        "look in <run>/tmp at 127.0.0.1:<port>, please"
    );
}

#[test]
fn rejects_absolute_paths_in_decoded_answers() {
    for line in [
        r#"{"id":2,"error":{"message":"file [/var/lib/example]"}}"#,
        r#"{"id":2,"error":{"message":"file [\u002fvar\u002flib\u002fexample]"}}"#,
        r#"{"id":2,"result":{"diagnostic":["file [/var/lib/example]"]}}"#,
        r#"{"id":2,"error":{"message":"file [/var/lib/example]","message":"ok"}}"#,
    ] {
        let seen = attempt(json!({"account_line": line}), Duration::ZERO);
        assert_eq!(
            seen.report["fault"],
            "answer contains an unapproved absolute path"
        );
        assert!(seen.report["answer"]["line"].is_null());
        assert!(seen.report["answer"]["error_message"].is_null());
    }
}

#[test]
fn account_reply_fault_does_not_record_initialize() {
    let seen = attempt(
        json!({
            "initialize_line": r#"{"id":1,"result":{"codexHome":"/var/lib/example"}}"#,
            "account_line": "{malformed",
        }),
        Duration::ZERO,
    );
    assert_eq!(seen.record[3]["method"], "account/read");
    assert_eq!(seen.report["fault"], "decode answer");
    assert_eq!(
        seen.report["answer"],
        json!({"kind":"eof","line":null,"error_message":null})
    );
}

#[test]
fn refuses_existing_output() {
    let test = scratch();
    let out = test.path().join("existing.json");
    fs::write(&out, b"existing").unwrap();
    assert_eq!(
        output_path(&out).unwrap_err().kind(),
        io::ErrorKind::AlreadyExists
    );
    assert_eq!(
        write_report(&out, &json!({})).unwrap_err().kind(),
        io::ErrorKind::AlreadyExists
    );
    let status = Command::new(env!("CARGO_BIN_EXE_refresh-probe"))
        .args(["run", "--codex"])
        .arg(fake())
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert!(!status.status.success());
    assert_eq!(fs::read(out).unwrap(), b"existing");
}

#[test]
fn report_holds_no_secret_or_path() {
    let test = scratch();
    let dirs = RefCell::new(Vec::<PathBuf>::new());
    let fabricated = RefCell::new(Vec::<String>::new());
    let hooks = Hooks {
        prepare: &|run, home, addr| {
            dirs.borrow_mut().push(run.to_path_buf());
            let data = login_data(&run.join("codex-home"));
            for field in ["id_token", "access_token", "refresh_token"] {
                fabricated
                    .borrow_mut()
                    .push(data["tokens"][field].as_str().unwrap().to_owned());
            }
            let message = format!("check {} at {addr}", run.display());
            let line = json!({"id":2,"error":{"message":message}}).to_string();
            fs::write(
                home.join("fake-script.json"),
                json!({"account_line":line}).to_string(),
            )
        },
        inspect: &|_| Ok(()),
    };
    let mut observed = report(Form::Issuer, &today());
    sitting(
        &mut observed,
        fake(),
        test.path(),
        test.path(),
        Form::Issuer,
        Duration::ZERO,
        Some(&hooks),
    );
    assert_eq!(observed["runs"].as_array().unwrap().len(), 18);
    assert!(
        observed["runs"]
            .as_array()
            .unwrap()
            .iter()
            .all(|run| run["fault"].is_null())
    );
    let path = test.path().join("report.json");
    write_report(&path, &observed).unwrap();
    let text = fs::read_to_string(path).unwrap();
    assert!(!text.contains("eyJ") && !text.contains("probe-208-refresh"));
    assert!(!text.contains(BLOCK));
    assert!(!text.contains(test.path().to_str().unwrap()));
    for run in dirs.into_inner() {
        assert!(!text.contains(run.to_str().unwrap()));
    }
    for value in fabricated.into_inner() {
        assert!(!text.contains(&value));
    }
}
