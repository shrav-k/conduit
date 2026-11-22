//! Scripted native client: never reads owner configuration or credentials.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    io::{self, BufRead, Read, Write},
    process::{Command, Stdio},
};
/// The canary URL the scripted login prints beside the URL it hands to its browser.
const URL_CANARY: &str = "https://synthetic-url-canary.invalid/oauth?state=synthetic-url-canary";

fn emit(value: Value) {
    println!("{value}");
    io::stdout().flush().unwrap();
}

// The orphan has no Child handle. Keep its start time for fixture-side cleanup
// if the test's independent observer cannot register it.
#[cfg(target_os = "linux")]
fn orphan_start(pid: i32) -> Option<u64> {
    let stat = std::fs::read(format!("/proc/{pid}/stat")).ok()?;
    String::from_utf8_lossy(&stat)
        .rsplit_once(')')?
        .1
        .split_ascii_whitespace()
        .nth(19)?
        .parse()
        .ok()
}

#[cfg(target_os = "macos")]
fn orphan_start(pid: i32) -> Option<u64> {
    let mut info = std::mem::MaybeUninit::<libc::proc_bsdinfo>::uninit();
    let size = std::mem::size_of::<libc::proc_bsdinfo>() as i32;
    let count = unsafe {
        libc::proc_pidinfo(
            pid,
            libc::PROC_PIDTBSDINFO,
            1,
            info.as_mut_ptr().cast(),
            size,
        )
    };
    if count == size {
        let info = unsafe { info.assume_init() };
        Some(info.pbi_start_tvsec * 1_000_000 + info.pbi_start_tvusec)
    } else {
        None
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn orphan_start(_: i32) -> Option<u64> {
    None
}

async fn plant_tree() {
    use std::{path::Path, time::Duration};
    use tokio::process::Command;

    let sleeper = || {
        let mut command = Command::new("sleep");
        command
            .arg("60")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        command
    };
    let mut background = sleeper().spawn().unwrap();
    let mut session = unsafe {
        sleeper()
            .pre_exec(|| {
                if libc::setsid() == -1
                    || libc::signal(libc::SIGTERM, libc::SIG_IGN) == libc::SIG_ERR
                {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            })
            .spawn()
            .unwrap()
    };
    let shell = Command::new("sh")
        .args(["-c", "sleep 60 </dev/null >/dev/null 2>&1 & echo $!"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await
        .unwrap();
    assert!(shell.status.success());
    let double_fork: i32 = String::from_utf8(shell.stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let Some(double_fork_start) = orphan_start(double_fork) else {
        // The PID came from the sh just waited on, so it cannot have been reused yet.
        unsafe { libc::kill(double_fork, libc::SIGKILL) };
        let _ = background.kill().await;
        let _ = session.kill().await;
        panic!("scripted client could not read the double fork's start time");
    };
    let tree = json!({
        "client": std::process::id(),
        "background": background.id().unwrap(),
        "session": session.id().unwrap(),
        "doubleFork": double_fork,
    });
    std::fs::write("tree.json.new", tree.to_string()).unwrap();
    std::fs::rename("tree.json.new", "tree.json").unwrap();
    while !Path::new("release").exists() {
        if Path::new("tree-cleanup").exists() {
            let _ = background.start_kill();
            let _ = session.start_kill();
            if orphan_start(double_fork) == Some(double_fork_start) {
                unsafe { libc::kill(double_fork, libc::SIGKILL) };
            }
            let mut ended = background.wait().await.is_ok() && session.wait().await.is_ok();
            for _ in 0..50 {
                if orphan_start(double_fork) != Some(double_fork_start) {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            ended &= orphan_start(double_fork) != Some(double_fork_start);
            if !ended {
                std::process::exit(1);
            }
            std::fs::write("tree-cleaned", b"ready").unwrap();
            std::process::exit(0);
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

async fn turn(args: &[String], message: &str, number: usize) {
    if message == "ACCOUNT-HOME" {
        let file = args.iter().position(|arg| arg == "--settings").unwrap();
        let settings: Value =
            serde_json::from_slice(&std::fs::read(&args[file + 1]).unwrap()).unwrap();
        let home = std::path::PathBuf::from(std::env::var_os("CLAUDE_CONFIG_DIR").unwrap());
        let home_settings = std::fs::read(home.join("settings.json"))
            .ok()
            .map(|bytes| serde_json::from_slice::<Value>(&bytes).unwrap());
        let home_memory_enabled =
            home_settings.is_some_and(|value| value["autoMemoryEnabled"] == true);
        let auto_memory_disabled =
            std::env::var_os("CLAUDE_CODE_DISABLE_AUTO_MEMORY").is_some_and(|value| value == "1");
        if home_memory_enabled && !auto_memory_disabled && settings["autoMemoryEnabled"] != false {
            std::fs::create_dir_all(home.join("projects/fixture/memory")).unwrap();
        }
        let evidence = json!({
            "home": home, "settings": settings, "autoMemoryDisabled": auto_memory_disabled,
            "homeMemoryEnabled": home_memory_enabled, "args": args,
            "number": number, "pid": std::process::id()
        });
        emit(json!({"type":"result", "result": evidence.to_string()}));
        return;
    }
    if let Some(cause) = message.strip_prefix("ACCOUNT-HOME-EOF:") {
        emit(json!({"type":"error", "message": cause}));
        std::process::exit(1);
    }
    if let Some(cause) = message.strip_prefix("ACCOUNT-HOME:") {
        emit(json!({"type":"result", "is_error": true, "result": cause}));
        return;
    }
    if message == "STALE_INIT" {
        let recorded: Value = serde_json::from_str(include_str!("stale-init.json")).unwrap();
        for event in recorded["turns"][number - 1].as_array().unwrap() {
            emit(event.clone());
        }
        return;
    }
    let claude = args.iter().any(|s| s == "--input-format");
    let (url, helper) = if claude {
        let i = args.iter().position(|s| s == "--mcp-config").unwrap();
        let config: Value = serde_json::from_slice(&std::fs::read(&args[i + 1]).unwrap()).unwrap();
        (
            config["mcpServers"]["engine"]["url"]
                .as_str()
                .unwrap()
                .to_owned(),
            config["mcpServers"]["engine"]["headersHelper"]
                .as_str()
                .unwrap()
                .to_owned(),
        )
    } else {
        let get = |key: &str| -> String {
            serde_json::from_str(args.iter().find_map(|a| a.strip_prefix(key)).unwrap()).unwrap()
        };
        (
            get("mcp_servers.engine.url="),
            get("mcp_servers.engine.http_headers_helper="),
        )
    };
    let mut command = Command::new("sh");
    command.args(["-c", &helper]).stderr(Stdio::null());
    if !claude {
        command.env_clear();
    }
    let output = command.output().unwrap();
    assert!(output.status.success());
    let header: Value = serde_json::from_slice(&output.stdout).unwrap();
    // The header stays in memory only; it is never returned in diagnostics or evidence.
    let mut tools = Vec::<String>::new();
    let mut reply = format!("fixture turn {number}");
    if message == "ENGINE" {
        use rmcp::{
            ServiceExt,
            model::CallToolRequestParams,
            transport::{
                StreamableHttpClientTransport,
                streamable_http_client::StreamableHttpClientTransportConfig,
            },
        };
        let token = header["Authorization"]
            .as_str()
            .unwrap()
            .strip_prefix("Bearer ")
            .unwrap();
        let service = ()
            .serve(StreamableHttpClientTransport::with_client(
                reqwest::Client::new(),
                StreamableHttpClientTransportConfig::with_uri(url).auth_header(token),
            ))
            .await
            .unwrap();
        tools = service
            .list_all_tools()
            .await
            .unwrap()
            .into_iter()
            .map(|t| format!("mcp__engine__{}", t.name))
            .collect();
        let call: CallToolRequestParams = serde_json::from_value(
            json!({"name":"project.directory.list.v1","arguments":{"path":"."}}),
        )
        .unwrap();
        let result = service.call_tool(call).await.unwrap();
        assert_ne!(result.is_error, Some(true));
        reply = "engine reached".into();
        service.cancel().await.unwrap();
    } else {
        // Synthetic startup evidence for wrapper tests, clearly fixture supplied.
        tools.push(format!("mcp__engine__fixture{number}"));
        let env: std::collections::BTreeMap<_, _> = std::env::vars().collect();
        let settings = if claude {
            let i = args.iter().position(|s| s == "--settings").unwrap();
            serde_json::from_slice(&std::fs::read(&args[i + 1]).unwrap()).unwrap()
        } else {
            Value::Null
        };
        let mcp = if claude {
            let i = args.iter().position(|s| s == "--settings").unwrap();
            let path = std::path::Path::new(&args[i + 1])
                .parent()
                .unwrap()
                .join("mcp.json");
            serde_json::from_slice::<Value>(&std::fs::read(path).unwrap()).unwrap()
        } else {
            Value::Null
        };
        let mut evidence = json!({"mcp":mcp,"args":args,"environment":env,"cwd":std::env::current_dir().unwrap(),"pid":std::process::id(),"settings":settings,"number":number,"message":message});
        if message == "HOME" {
            use std::{fs, io::ErrorKind::NotFound, path::PathBuf};

            let (variable, default) = if claude {
                ("CLAUDE_CONFIG_DIR", ".claude")
            } else {
                ("CODEX_HOME", ".codex")
            };
            let home = std::env::var_os(variable)
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(std::env::var_os("HOME").unwrap()).join(default));
            fs::create_dir_all(&home).unwrap();
            fs::write(home.join("fixture-home-write"), number.to_string()).unwrap();
            let mut dotenv = Vec::new();
            if !claude {
                let aliases = home.join("tmp/arg0-fixture");
                fs::create_dir_all(&aliases).unwrap();
                fs::write(aliases.join("lock"), b"").unwrap();
                match fs::read_to_string(home.join(".env")) {
                    Ok(contents) => {
                        dotenv.extend(contents.lines().filter_map(|line| {
                            line.split_once('=').map(|(name, _)| name.to_owned())
                        }))
                    }
                    Err(error) if error.kind() == NotFound => {}
                    Err(error) => panic!("cannot read fixture .env: {}", error.kind()),
                }
            }
            evidence["home"] = json!(home);
            evidence["dotenv"] = json!(dotenv);
        }
        reply = evidence.to_string();
    }
    if message == "TREE" {
        plant_tree().await;
    } else if message == "BLOCK" {
        std::fs::write("blocked", b"ready").unwrap();
        while !std::path::Path::new("release").exists() {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    if message == "QUEUED" {
        std::fs::write("queued-launched", b"unexpected").unwrap();
    }
    if message == "INVALID_JSON" {
        println!("sentinel-not-json");
        return;
    }
    if message == "MISSING_THREAD" {
        emit(json!({"type":"thread.started"}));
        return;
    }
    if message == "NO_THREAD" {
        emit(json!({"type":"turn.completed"}));
        return;
    }
    if message == "CHANGED_THREAD" {
        emit(json!({"type":"thread.started","thread_id":"changed"}));
        return;
    }
    if message == "NO_CATALOG" {
        emit(if claude {
            json!({"type":"system","subtype":"init"})
        } else {
            json!({"type":"thread.started","thread_id":"fixture-exact-thread"})
        });
    } else if claude {
        emit(json!({"type":"system","subtype":"init","tools":tools}));
    } else {
        emit(json!({"type":"thread.started","thread_id":"fixture-exact-thread","tools":tools}));
    }
    if let Some(status) = message.strip_prefix("EXIT:") {
        std::process::exit(status.parse().unwrap());
    }
    if message == "SIGNAL" {
        Command::new("kill")
            .args(["-KILL", &std::process::id().to_string()])
            .status()
            .unwrap();
        unreachable!();
    }
    if message == "ERROR_COMPLETED" || message == "ERROR_FAILED" || message == "ERROR_EXIT" {
        emit(json!({"type":"error","message":"sentinel-secret quota"}));
        if message == "ERROR_EXIT" {
            std::process::exit(1);
        }
        if message == "ERROR_FAILED" {
            emit(json!({"type":"turn.failed","error":{"message":"sentinel-secret unauthorized"}}));
            return;
        }
    }
    if let Some(cause) = message.strip_prefix("FAIL:") {
        if claude {
            emit(
                json!({"type":"result","is_error":true,"result":format!("sentinel-secret {cause}")}),
            );
        } else {
            emit(
                json!({"type":"turn.failed","error":{"message":format!("sentinel-secret {cause}")}}),
            );
        }
        return;
    }
    if message == "FAIL_NO_TEXT" {
        emit(if claude {
            json!({"type":"result","is_error":true})
        } else {
            json!({"type":"turn.failed"})
        });
        return;
    }
    if message == "COMPLETED_EXIT" {
        emit(json!({"type":"turn.completed"}));
        std::process::exit(4);
    }
    if message == "EOF" {
        return;
    }
    if message == "ERROR" {
        emit(json!({"type":"error"}));
        return;
    }
    if claude {
        emit(json!({"type":"result","result":reply}));
    } else {
        emit(json!({"type":"item.completed","item":{"type":"agent_message","text":reply}}));
        emit(json!({"type":"turn.completed"}));
    }
}
fn logout(args: &[String]) {
    use std::{collections::BTreeMap, fs, path::PathBuf, time::Duration};

    let variable = if args[0] == "auth" {
        "CLAUDE_CONFIG_DIR"
    } else {
        "CODEX_HOME"
    };
    let home = PathBuf::from(std::env::var_os(variable).expect("logout without the client home"));
    let mut entries: Vec<_> = fs::read_dir(&home)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    entries.sort();
    let environment: BTreeMap<_, _> = std::env::vars().collect();
    let evidence = json!({
        "args": args,
        "home": home,
        "environment": environment,
        "entries": entries,
        "cwd": std::env::current_dir().unwrap(),
    });
    let mut log = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("logouts.jsonl")
        .unwrap();
    writeln!(log, "{evidence}").unwrap();

    let login = home.join("fixture-login");
    let text = fs::read_to_string(&login).unwrap_or_default();
    println!("logout output {text}");
    eprintln!("logout error {text}");
    if let Ok(exit) = fs::read_to_string("logout-exit") {
        std::process::exit(exit.trim().parse().unwrap());
    }
    if std::path::Path::new("logout-hang").exists() {
        std::thread::sleep(Duration::from_secs(30));
    }
    if login.exists() {
        fs::remove_file(login).unwrap();
    }
}

fn status() {
    use std::{
        collections::BTreeMap,
        fs,
        path::{Path, PathBuf},
        time::Duration,
    };

    let home = PathBuf::from(
        std::env::var_os("CLAUDE_CONFIG_DIR").expect("status without the client home"),
    );
    let mut entries: Vec<_> = fs::read_dir(&home)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    entries.sort();
    let environment: BTreeMap<_, _> = std::env::vars().collect();
    let evidence = json!({
        "args": ["auth", "status"],
        "home": home,
        "environment": environment,
        "entries": entries,
        "cwd": std::env::current_dir().unwrap(),
    });
    let mut log = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("statuses.jsonl")
        .unwrap();
    writeln!(log, "{evidence}").unwrap();

    if Path::new("status-hang").exists() {
        if Path::new("status-descendant.sock").exists() {
            // Inherit the status's process group. The test observes this child's socket.
            tokio::process::Command::new(std::env::current_exe().unwrap())
                .arg("fixture-status-descendant")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();
        }
        std::thread::sleep(Duration::from_secs(30));
    }
    let login: Option<Value> = match fs::read(home.join("fixture-login")) {
        Ok(bytes) => Some(serde_json::from_slice(&bytes).unwrap()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => panic!("cannot read fixture login: {error}"),
    };
    match fs::read("status-output") {
        Ok(bytes) => io::stdout().write_all(&bytes).unwrap(),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let output = if let Some(login) = &login {
                let mut output = json!({
                    "loggedIn": true,
                    "authMethod": "claude.ai",
                    "configDirectory": home,
                    "orgName": login["claims"]["account"],
                    "accessToken": login["token"],
                });
                if let Some(email) = login.get("email") {
                    output["email"] = email.clone();
                }
                output
            } else {
                json!({"loggedIn": false, "authMethod": "none", "configDirectory": home})
            };
            emit(output);
        }
        Err(error) => panic!("cannot read fixture status output: {error}"),
    }
    io::stdout().flush().unwrap();
    if let Some(login) = &login {
        eprintln!("status error {}", login["token"].as_str().unwrap());
    }
    let exit = match fs::read_to_string("status-exit") {
        Ok(exit) => exit.trim().parse().unwrap(),
        Err(error) if error.kind() == io::ErrorKind::NotFound => i32::from(login.is_none()),
        Err(error) => panic!("cannot read fixture status exit: {error}"),
    };
    std::process::exit(exit);
}

/// 43 characters of the base64url alphabet, as Claude Code's state and challenge have.
fn random43() -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut bytes = [0; 32];
    std::fs::File::open("/dev/urandom")
        .unwrap()
        .read_exact(&mut bytes)
        .unwrap();
    let mut text = String::new();
    for chunk in bytes.chunks(3) {
        let n = chunk.iter().fold(0u32, |n, &b| n << 8 | u32::from(b)) << (8 * (3 - chunk.len()));
        for i in 0..=chunk.len() {
            text.push(char::from(ALPHABET[(n >> (18 - 6 * i) & 63) as usize]));
        }
    }
    text
}

fn login() {
    use std::{
        collections::BTreeMap,
        fs,
        net::TcpListener,
        path::{Path, PathBuf},
        time::{Duration, Instant},
    };

    let home = PathBuf::from(
        std::env::var_os("CLAUDE_CONFIG_DIR").expect("login without the client home"),
    );
    let mut entries: Vec<_> = fs::read_dir(&home)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    entries.sort();
    let environment: BTreeMap<_, _> = std::env::vars().collect();
    let evidence = json!({
        "args": ["auth", "login"],
        "home": home,
        "environment": environment,
        "entries": entries,
        "cwd": std::env::current_dir().unwrap(),
    });
    let mut log = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("logins.jsonl")
        .unwrap();
    writeln!(log, "{evidence}").unwrap();
    fs::write("login.pid", std::process::id().to_string()).unwrap();

    // Client output that the wrapper discards: the canary URL is printed beside the one handed over.
    println!("If the browser didn't open, visit: {URL_CANARY}");
    io::stdout().flush().unwrap();
    eprintln!("login error {URL_CANARY}");
    let leave_descendant = || {
        if Path::new("login-descendant.sock").exists() {
            #[expect(
                clippy::zombie_processes,
                reason = "this fixture deliberately leaves a live descendant when its leader exits"
            )]
            let mut descendant = Command::new(std::env::current_exe().unwrap())
                .arg("fixture-login-descendant")
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();
            // Confirm the descendant connected before leaving it in the login's group.
            descendant
                .stdout
                .take()
                .unwrap()
                .read_exact(&mut [0])
                .unwrap();
        }
    };
    if let Ok(exit) = fs::read_to_string("login-exit") {
        leave_descendant();
        std::process::exit(exit.trim().parse().unwrap());
    }
    if Path::new("login-fifo").exists() {
        // A FIFO where the URL goes, which no browser writes.
        use std::os::unix::ffi::OsStrExt;
        let capture = std::env::var_os("AGENT_SIGN_IN_CAPTURE").expect("login without capture");
        let capture = std::ffi::CString::new(capture.as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(capture.as_ptr(), 0o600) }, 0);
    }
    if Path::new("login-silent").exists() {
        std::thread::sleep(Duration::from_secs(30));
        return;
    }
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let state = random43();
    let url = fs::read_to_string("login-url")
        .unwrap()
        .replace("{port}", &port.to_string())
        .replace("{state}", &state)
        .replace("{challenge}", &random43());
    fs::write("login-url.sha256", Sha256::digest(url.as_bytes())).unwrap();
    let browser = std::env::var_os("BROWSER").expect("login without BROWSER");
    let _ = Command::new(browser)
        .arg(&url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if Path::new("login-exit-after-url").exists() {
        leave_descendant();
        // Exit only once the hook has answered with the URL.
        std::thread::sleep(Duration::from_secs(1));
        std::process::exit(6);
    }
    // Wait at most 30 s for the return at the login's own loopback listener.
    listener.set_nonblocking(true).unwrap();
    let until = Instant::now() + Duration::from_secs(30);
    let mut stream = loop {
        match listener.accept() {
            Ok((stream, _)) => break stream,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                if Instant::now() >= until {
                    std::process::exit(5);
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(error) => panic!("login listener failed: {error}"),
        }
    };
    stream.set_nonblocking(false).unwrap();
    let mut head = Vec::new();
    let mut byte = [0];
    while !head.ends_with(b"\r\n\r\n") && head.len() < 4096 {
        if stream.read(&mut byte).unwrap() == 0 {
            break;
        }
        head.push(byte[0]);
    }
    let head = String::from_utf8_lossy(&head);
    let target = head.split(' ').nth(1).unwrap_or_default();
    if !target.ends_with(&format!("&state={state}")) {
        stream
            .write_all(
                b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            )
            .unwrap();
        std::process::exit(4);
    }
    fs::copy("login-result", home.join("fixture-login")).unwrap();
    stream
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
        .unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["auth", "logout"] || args == ["logout", "-c", "cli_auth_credentials_store=\"file\""]
    {
        logout(&args);
        return;
    }
    if args == ["auth", "status"] {
        status();
        return;
    }
    if args == ["auth", "login"] {
        login();
        return;
    }
    if args == ["fixture-login-descendant"] {
        let _observer = std::os::unix::net::UnixStream::connect("login-descendant.sock").unwrap();
        io::stdout().write_all(&[1]).unwrap();
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(15));
        return;
    }
    if args == ["fixture-status-descendant"] {
        let _observer = std::os::unix::net::UnixStream::connect("status-descendant.sock").unwrap();
        // Bound the descendant's lifetime even if a broken hook leaves it running.
        std::thread::sleep(std::time::Duration::from_secs(15));
        return;
    }
    std::fs::write(
        "client-started",
        std::env::current_exe().unwrap().to_str().unwrap(),
    )
    .unwrap();
    if args
        .windows(2)
        .any(|a| a[0] == "--model" && a[1] == "exit-at-start")
    {
        std::process::exit(2);
    }
    if args.iter().any(|s| s == "--input-format") {
        for (i, line) in io::stdin().lock().lines().enumerate() {
            let v: Value = serde_json::from_str(&line.unwrap()).unwrap();
            turn(&args, v["message"]["content"].as_str().unwrap(), i + 1).await;
            if matches!(
                v["message"]["content"].as_str(),
                Some("EOF" | "ERROR" | "INVALID_JSON")
            ) {
                return;
            }
        }
    } else {
        let resumed = args.iter().any(|s| s == "resume");
        if resumed {
            assert_eq!(args[args.len() - 2], "fixture-exact-thread");
        }
        assert_eq!(args.last().unwrap(), "-");
        let mut message = String::new();
        io::stdin().read_to_string(&mut message).unwrap();
        turn(&args, &message, if resumed { 2 } else { 1 }).await;
    }
}
