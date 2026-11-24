//! Scripted test clients. The marker is checked before any write or client behavior.
use serde_json::{Value, json};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, Write};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

fn root_for(path: &Path) -> Option<PathBuf> {
    path.canonicalize()
        .ok()?
        .ancestors()
        .find(|p| p.join("fake-root").is_file())
        .map(Path::to_path_buf)
}
fn log(file: &mut File, v: Value) {
    writeln!(file, "{v}").unwrap();
    file.flush().unwrap();
}
fn emit(v: &Value) {
    println!("{v}");
    std::io::stdout().flush().unwrap();
}
fn delay(script: &Value, key: &str) {
    thread::sleep(Duration::from_millis(script[key].as_u64().unwrap_or(0)));
}
fn silence() -> ! {
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
fn change(home: &Path, action: &str) {
    let login = home.join("auth.json");
    match action {
        "rewrite" => fs::write(login, b"changed-fabricated-login").unwrap(),
        "rename" | "restore_time" => {
            let old = fs::metadata(&login).unwrap();
            let replacement = home.join("replacement");
            fs::write(&replacement, b"fabricated-login").unwrap();
            fs::rename(replacement, &login).unwrap();
            if action == "restore_time" {
                times(
                    &login,
                    old.atime(),
                    old.atime_nsec(),
                    old.mtime(),
                    old.mtime_nsec(),
                );
            }
        }
        "time" => {
            let m = fs::metadata(&login).unwrap();
            times(
                &login,
                m.atime(),
                m.atime_nsec(),
                m.mtime() + 2,
                m.mtime_nsec(),
            );
        }
        "lock" => fs::write(home.join("auth.lock"), b"").unwrap(),
        "remove" => {
            let _ = fs::remove_file(home.join("remove-me"));
        }
        "add" => fs::write(home.join("added"), b"").unwrap(),
        "deep" => {
            fs::create_dir_all(home.join("state/deeper")).unwrap();
            fs::write(home.join("state/deeper/entry"), b"").unwrap();
        }
        "hex" => fs::write(home.join(["abcd", "1234", "-entry"].concat()), b"").unwrap(),
        _ => {}
    }
}
fn times(path: &Path, at: i64, an: i64, mt: i64, mn: i64) {
    use std::os::unix::ffi::OsStrExt;
    let path = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap();
    let times = [
        libc::timespec {
            tv_sec: at as _,
            tv_nsec: an as _,
        },
        libc::timespec {
            tv_sec: mt as _,
            tv_nsec: mn as _,
        },
    ];
    assert_eq!(
        unsafe { libc::utimensat(libc::AT_FDCWD, path.as_ptr(), times.as_ptr(), 0) },
        0
    );
}
extern "C" fn reap(_: libc::c_int) {
    unsafe { while libc::waitpid(-1, std::ptr::null_mut(), libc::WNOHANG) > 0 {} }
}
fn descendant(root: &Path) {
    // The descendant ignores TERM; the probe must KILL its group and observe it gone.
    let pid = unsafe { libc::fork() };
    assert!(pid >= 0);
    if pid == 0 {
        unsafe {
            libc::signal(libc::SIGTERM, libc::SIG_IGN);
            libc::signal(libc::SIGCHLD, reap as *const () as libc::sighandler_t);
        }
        fs::write(
            root.join(format!("descendant-{}", std::process::id())),
            std::process::id().to_string(),
        )
        .unwrap();
        silence();
    }
}
/// A descendant that leaves the client's process group and session (`setsid`), as a detached
/// helper might, and ignores TERM. It leaves by itself after 30 seconds, should a probe miss it.
fn detached(root: &Path) {
    let pid = unsafe { libc::fork() };
    assert!(pid >= 0);
    if pid == 0 {
        unsafe {
            libc::setsid();
            libc::signal(libc::SIGTERM, libc::SIG_IGN);
        }
        let _ = fs::write(
            root.join(format!("detached-{}", std::process::id())),
            std::process::id().to_string(),
        );
        thread::sleep(Duration::from_secs(30));
        unsafe { libc::_exit(0) }
    }
    // The client lives on a moment, as a real one would, before it goes on.
    thread::sleep(Duration::from_millis(100));
}
/// This app-server's place among the run's app-servers, from 1, claimed atomically in the root.
fn claim(root: &Path) -> u64 {
    for n in 1..1000 {
        match OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(root.join(format!("call-{n}")))
        {
            Ok(_) => return n,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => return 0,
        }
    }
    0
}
pub fn main_entry(codex: bool) -> i32 {
    let home = match std::env::var_os(if codex {
        "CODEX_HOME"
    } else {
        "CLAUDE_CONFIG_DIR"
    }) {
        Some(v) => PathBuf::from(v),
        None => return 1,
    };
    let base = match std::env::var_os("HOME") {
        Some(v) => PathBuf::from(v),
        None => return 1,
    };
    let Some(root) = root_for(&home) else {
        return 1;
    };
    if root_for(&base).as_ref() != Some(&root) {
        return 1;
    }
    let script: Value = match fs::read(root.join("fake-script.json"))
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
    {
        Some(v) => v,
        None => return 1,
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let version = args == ["--version"];
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(root.join(format!("log-{}.jsonl", std::process::id())))
        .unwrap();
    let mut names: Vec<_> = std::env::vars_os()
        .map(|(k, _)| k.to_string_lossy().into_owned())
        .collect();
    names.sort();
    let sanitized: Vec<_> = args
        .iter()
        .map(|s| {
            if Path::new(s).is_absolute() || s == "settings.json" {
                "scratch/settings.json".to_owned()
            } else {
                s.clone()
            }
        })
        .collect();
    let settings = if let Some(i) = args.iter().position(|s| s == "--settings") {
        fs::read(&args[i + 1])
            .ok()
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
    } else {
        None
    };
    log(
        &mut file,
        json!({"args":sanitized,"variables":names,"blocked":std::env::var_os("CODEX_REFRESH_TOKEN_URL_OVERRIDE").is_some_and(|v|v==crate::process::BLOCK),"settings":settings,"version":version,"empty_home":fs::read_dir(&home).unwrap().next().is_none(),"home_is_base":home==base}),
    );
    if let Some(stderr) = script["stderr"].as_str() {
        eprintln!("{stderr}");
    }
    if version {
        delay(&script, "version_delay_ms");
        println!(
            "fake-client {}",
            script["version"].as_str().unwrap_or("1.2.3")
        );
        return 0;
    }
    if let Some(names) = script["create_names"].as_array() {
        for name in names {
            fs::write(home.join(name.as_str().unwrap()), b"").unwrap();
        }
    }
    if script["descendant"] == true {
        descendant(&root);
    }
    if script["detached"] == true {
        detached(&root);
    }
    // Scripted notifications go to every app-server, or only to those `notify_calls` lists.
    let call = claim(&root);
    let notify = script["notify_calls"]
        .as_array()
        .is_none_or(|calls| calls.contains(&json!(call)));
    if script["ignore_term"] == true {
        // The client itself outlives TERM, so it can still print during the stop's grace.
        unsafe {
            libc::signal(libc::SIGTERM, libc::SIG_IGN);
        }
    }
    for line in std::io::stdin().lock().lines() {
        let Ok(line) = line else {
            break;
        };
        let Ok(v) = serde_json::from_str::<Value>(&line) else {
            return 1;
        };
        log(&mut file, json!({"message":v}));
        if codex {
            match v["method"].as_str().unwrap_or("") {
                "initialize" => {
                    delay(&script, "initialize_delay_ms");
                    emit(&json!({"id":1,"result":{}}));
                }
                "initialized" => {}
                "account/rateLimits/read" => {
                    if script["never"] == true {
                        silence();
                    }
                    delay(&script, "answer_delay_ms");
                    if script["notification"] == true {
                        emit(&json!({"method":"notice","params":{}}));
                    }
                    if notify {
                        for note in script["before_answer"].as_array().into_iter().flatten() {
                            emit(note);
                        }
                    }
                    change(&home, script["change"].as_str().unwrap_or("none"));
                    if let Some(raw) = script["raw_answer"].as_str() {
                        println!("{raw}");
                        std::io::stdout().flush().unwrap();
                    } else if let Some(error) = script.get("error") {
                        emit(&json!({"id":2,"error":error}));
                    } else {
                        emit(
                            &json!({"id":2,"result":script.get("answer").cloned().unwrap_or(json!({"ordinaryUsageAllowed":true,"rateLimits":{"primary":{"usedPercent":25,"windowDurationMins":300,"resetsAt":1900000000}}}))}),
                        );
                    }
                    if notify && let Some(notes) = script["after_answer"].as_array() {
                        delay(&script, "after_delay_ms");
                        for note in notes {
                            emit(note);
                        }
                    }
                    if script["late_write"] == true {
                        thread::sleep(Duration::from_secs(3));
                        fs::write(home.join("late-state"), b"late").unwrap();
                    }
                    if script["exit_after_answer"] == true {
                        return 0;
                    }
                }
                _ => return 1,
            }
        } else {
            if v["message"]["content"] != "/usage" {
                return 1;
            }
            delay(&script, "answer_delay_ms");
            change(&home, script["change"].as_str().unwrap_or("none"));
            let default = json!([{"type":"system","usage_report":{"rate_limits":{"limits":[{"kind":"session","percent":25,"resets_at":"2030-01-01T00:00:00Z"}]}}},{"type":"result","duration_api_ms":0,"usage":{},"result":""}]);
            let events = script.get("events").unwrap_or(&default);
            for event in events.as_array().unwrap() {
                emit(event);
                delay(&script, "event_delay_ms");
            }
            // Lines that are not JSON, printed as given after the events.
            for raw in script["raw_lines"].as_array().into_iter().flatten() {
                println!("{}", raw.as_str().unwrap());
            }
            if let Some(bytes) = script["raw_bytes"].as_array() {
                let bytes: Vec<u8> = bytes.iter().map(|b| b.as_u64().unwrap() as u8).collect();
                std::io::stdout().write_all(&bytes).unwrap();
            }
            std::io::stdout().flush().unwrap();
            if script["never"] == true {
                silence();
            }
            return 0;
        }
    }
    0
}
