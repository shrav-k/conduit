//! Scripted native client: never reads owner configuration or credentials.
use serde_json::{Value, json};
use std::{
    io::{self, BufRead, Read, Write},
    process::{Command, Stdio},
};
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
    if matches!(
        message,
        "HISTORY-DETAILS" | "HISTORY-QUOTED command-output-canary"
    ) {
        let answer = if message == "HISTORY-DETAILS" {
            "public final answer"
        } else {
            "The final answer quotes command-output-canary."
        };
        emit(json!({"type":"thread.started", "thread_id":"history-thread"}));
        for item in [
            json!({"type":"command_execution","command":"tool-command-canary","aggregated_output":"command-output-canary","exit_code":0}),
            json!({"type":"mcp_tool_call","tool":"mcp-call-canary","arguments":{},"result":{"content":[{"type":"text","text":"mcp-result-canary"}]}}),
            json!({"type":"file_change","changes":[{"path":"file-change-canary","kind":"update"}]}),
            json!({"type":"reasoning","text":"reasoning-canary"}),
            json!({"type":"agent_message","text":answer}),
        ] {
            emit(json!({"type":"item.completed","item":item}));
        }
        emit(
            json!({"type":"turn.completed","usage":{"input_tokens":438291,"output_tokens":764812,"reasoning_output_tokens":349821}}),
        );
        return;
    }
    if let Some(fixture) = message.strip_prefix("ORDER-260:") {
        let fixture: Value = serde_json::from_str(fixture).unwrap();
        emit(json!({"type":"thread.started", "thread_id":"fixture-exact-thread"}));
        if let Some(cause) = fixture["error"].as_str() {
            emit(json!({"type":"turn.failed", "error":{"message":cause}}));
            return;
        }
        let text = if fixture["launch"] == true {
            let schema = args
                .iter()
                .position(|arg| arg == "--output-schema")
                .map(|index| std::fs::read_to_string(&args[index + 1]).unwrap());
            json!({"args":args,"schema":schema}).to_string()
        } else {
            fixture["text"]
                .as_str()
                .unwrap_or("scripted answer")
                .to_owned()
        };
        emit(json!({"type":"item.completed", "item":{"type":"agent_message", "text":text}}));
        let mut completed = json!({"type":"turn.completed"});
        if let Some(usage) = fixture.get("usage") {
            completed["usage"] = usage.clone();
        }
        emit(completed);
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

fn app_server(args: &[String]) {
    use std::{collections::BTreeMap, fs, path::PathBuf, time::Duration};

    let home = PathBuf::from(
        std::env::var_os("CODEX_HOME").expect("account read without the client home"),
    );
    let mut entries: Vec<_> = fs::read_dir(&home)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    entries.sort();
    let aliases = home.join("tmp/arg0-fixture");
    fs::create_dir_all(&aliases).unwrap();
    fs::write(aliases.join("lock"), b"").unwrap();
    let append = |path| {
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap()
    };
    let environment: BTreeMap<_, _> = std::env::vars().collect();
    let evidence = json!({
        "args": args,
        "home": home,
        "environment": environment,
        "entries": entries,
        "cwd": std::env::current_dir().unwrap(),
        "pid": std::process::id(),
    });
    writeln!(append("account-reads.jsonl"), "{evidence}").unwrap();
    let login = fs::read_to_string(home.join("fixture-login")).unwrap_or_default();
    eprintln!("app-server error {login}");

    let script: Value = match fs::read_to_string("account-script.json") {
        Ok(text) => serde_json::from_str(&text).unwrap(),
        Err(error) if error.kind() == io::ErrorKind::NotFound => json!({}),
        Err(error) => panic!("cannot read account script: {error}"),
    };
    assert!(script.is_object());
    if script["detach"].as_bool().unwrap_or(false) {
        use std::os::unix::process::CommandExt;
        let detached = unsafe {
            Command::new("sleep")
                .arg("60")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .pre_exec(|| {
                    if libc::setsid() == -1 {
                        return Err(io::Error::last_os_error());
                    }
                    Ok(())
                })
                .spawn()
                .unwrap()
        };
        fs::write("account-detached", detached.id().to_string()).unwrap();
    }
    let answer = |stage: &str, id: &Value, result: Value| {
        if let Some(delay_ms) = script["delayMs"][stage].as_u64() {
            std::thread::sleep(Duration::from_millis(delay_ms));
        }
        if script["notify"].as_bool().unwrap_or(false) {
            emit(json!({"method":"account/updated","params":{"fixture":login}}));
            emit(
                json!({"id":"fixture-request","method":"fixture/request","params":{"fixture":login}}),
            );
        }
        match script[stage].as_str().unwrap_or("answer") {
            "answer" => emit(json!({"id":id,"result":result})),
            "error" => emit(
                json!({"id":id,"error":{"code":-32600,"message":format!("sentinel-secret {login}")}}),
            ),
            "invalid" => {
                println!("sentinel-not-json {login}");
                io::stdout().flush().unwrap();
            }
            "wrong-id" => emit(json!({"id":99,"result":result})),
            "hang" => {
                std::thread::sleep(Duration::from_secs(60));
                std::process::exit(0);
            }
            "eof" => {
                unsafe { libc::close(1) };
                std::thread::sleep(Duration::from_secs(60));
                std::process::exit(0);
            }
            "eof-exit" => {
                unsafe { libc::close(1) };
                std::thread::sleep(Duration::from_millis(300));
                std::process::exit(0);
            }
            behavior => {
                std::process::exit(behavior.strip_prefix("exit:").unwrap().parse().unwrap())
            }
        }
    };
    let mut log = append("account-requests.jsonl");
    let mut requests = io::stdin().lock().lines().map(|line| {
        let request: Value = serde_json::from_str(&line.unwrap()).unwrap();
        writeln!(log, "{request}").unwrap();
        request
    });
    let Some(initialize) = requests.next() else {
        return;
    };
    answer(
        "initialize",
        &initialize["id"],
        json!({"userAgent":"fixture"}),
    );
    let Some(read) = requests.find(|request| {
        matches!(
            request["method"].as_str(),
            Some("account/read" | "account/rateLimits/read")
        )
    }) else {
        return;
    };
    if read["method"] == "account/rateLimits/read" {
        emit(json!({"id":read["id"],"result":{}}));
        return;
    }
    answer(
        "read",
        &read["id"],
        script
            .get("answer")
            .cloned()
            .unwrap_or_else(|| json!({"account":null,"requiresOpenaiAuth":true})),
    );
    if script["exitOnEof"].as_bool().unwrap_or(false) {
        requests.for_each(drop);
        std::fs::write("account-eof", "").unwrap();
    } else if script["linger"].as_bool().unwrap_or(true) {
        std::thread::sleep(Duration::from_secs(60));
    }
}

fn login(args: &[String]) {
    use std::{
        collections::BTreeMap,
        fs,
        os::unix::process::CommandExt,
        path::{Path, PathBuf},
        time::{Duration, Instant},
    };

    let home =
        PathBuf::from(std::env::var_os("CODEX_HOME").expect("login without the client home"));
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
        "pid": std::process::id(),
    });
    let mut log = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("logins.jsonl")
        .unwrap();
    writeln!(log, "{evidence}").unwrap();

    let login_path = home.join("fixture-login");
    let stored = fs::read_to_string(&login_path).unwrap_or_default();
    eprintln!("login error {stored}");
    let script = fs::read_to_string("login-script.json")
        .map(|text| serde_json::from_str::<Value>(&text).unwrap())
        .unwrap_or_else(|_| json!({}));
    match script["behavior"].as_str().unwrap_or("url") {
        "url" | "canary-only" => {
            eprintln!(
                "canary https://canary.invalid/authorize?client_id=sentinel-canary&state=sentinel-canary-state {stored}"
            );
            if script["behavior"] == "canary-only" {
                std::thread::sleep(Duration::from_secs(60));
                return;
            }
            let browser = agent_adapter::stated_logins()
                .into_iter()
                .find(|login| login.name == agent_adapter::BROWSER_LOGIN)
                .unwrap();
            let mut url = reqwest::Url::parse(&browser.authorization).unwrap();
            let redirect = format!(
                "http://{}:{}{}",
                browser.host.unwrap(),
                browser.ports[0],
                browser.path.unwrap(),
            );
            url.query_pairs_mut()
                .append_pair("response_type", "code")
                .append_pair("client_id", &browser.client_id)
                .append_pair("redirect_uri", &redirect)
                .append_pair(
                    "code_challenge",
                    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQ",
                )
                .append_pair("code_challenge_method", "S256")
                .append_pair("state", "fixture-state-0123456789abcdef");
            eprintln!(
                "If your browser did not open, navigate to this URL to authenticate:\n\n{url}\n"
            );
            io::stderr().flush().unwrap();
            let until = Instant::now() + Duration::from_secs(60);
            let mut detached = false;
            while Instant::now() < until {
                if !detached && Path::new("login-detach").exists() {
                    let descendant = unsafe {
                        Command::new("sleep")
                            .arg("10")
                            .stdin(Stdio::null())
                            .stdout(Stdio::null())
                            .stderr(Stdio::null())
                            .pre_exec(|| {
                                if libc::setsid() == -1 {
                                    return Err(io::Error::last_os_error());
                                }
                                Ok(())
                            })
                            .spawn()
                            .unwrap()
                    };
                    fs::write("login-detached.new", descendant.id().to_string()).unwrap();
                    fs::rename("login-detached.new", "login-detached").unwrap();
                    detached = true;
                }
                if Path::new("login-finish").exists() {
                    fs::write(login_path, "fixture-signed-in").unwrap();
                    let exit = fs::read_to_string("login-exit")
                        .map(|text| text.trim().parse().unwrap())
                        .unwrap_or(0);
                    std::process::exit(exit);
                }
                std::thread::sleep(Duration::from_millis(50));
            }
        }
        "hang" => std::thread::sleep(Duration::from_secs(60)),
        behavior => {
            let exit = behavior.strip_prefix("exit:").unwrap().parse().unwrap();
            std::process::exit(exit);
        }
    }
}

fn rate_limit_server(script: &Value) {
    for line in io::stdin().lock().lines() {
        let request: Value = serde_json::from_str(&line.unwrap()).unwrap();
        match request["method"].as_str() {
            Some("initialize") => emit(json!({"id":request["id"], "result":{}})),
            Some("account/rateLimits/read") => {
                let blocked = std::env::var_os(agent_adapter::REFRESH_OVERRIDE_VARIABLE)
                    .is_some_and(|value| value == agent_adapter::REFRESH_BLOCKED);
                std::fs::write(
                    "rate-limit-read",
                    json!({"refreshBlocked":blocked,"pid":std::process::id()}).to_string(),
                )
                .unwrap();
                if script["delayExit"] == true {
                    unsafe { libc::signal(libc::SIGTERM, libc::SIG_IGN) };
                    if script["eof"] == true {
                        unsafe { libc::close(libc::STDOUT_FILENO) };
                    } else {
                        emit(json!({"id":request["id"], "result":script["answer"]}));
                    }
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    return;
                }
                if let Some(delay) = script["delayMillis"].as_u64() {
                    std::thread::sleep(std::time::Duration::from_millis(delay));
                }
                if script["error"] == true {
                    emit(
                        json!({"id":request["id"], "error":{"code":-32603,"message":"scripted failure"}}),
                    );
                } else {
                    emit(json!({"id":request["id"], "result":script["answer"]}));
                }
                return;
            }
            Some("initialized") => {}
            _ => return,
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if let [command, output] = args.as_slice()
        && command == "version-fixture-child"
    {
        let address = std::fs::read_to_string("version-listener").unwrap();
        let mut stream = std::net::TcpStream::connect(address.trim()).unwrap();
        writeln!(stream, "{}", std::process::id()).unwrap();
        if output == "@launch-oversize" {
            print!("{}", "x".repeat(257));
            std::io::stdout().flush().unwrap();
        }
        std::thread::sleep(std::time::Duration::from_secs(30));
        return;
    }
    if args == ["--version"] {
        let mut log = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("version-invocations")
            .unwrap();
        writeln!(log, "version").unwrap();
        let version = std::fs::read_to_string("exec-version")
            .unwrap_or_else(|_| "codex-cli 0.167.1\n".into());
        match version.as_str() {
            "@home" => {
                let expected = std::path::PathBuf::from(
                    std::fs::read_to_string("expected-version-home").unwrap(),
                );
                if std::env::var_os("CODEX_HOME").as_deref() != Some(expected.as_os_str()) {
                    std::process::exit(1);
                }
                if std::fs::symlink_metadata(expected.join("auth.json")).is_ok() {
                    std::process::exit(1);
                }
                println!("codex-cli 0.167.1");
            }
            "@launch-timeout" | "@launch-oversize" => {
                std::fs::write("version-launcher-pid", std::process::id().to_string()).unwrap();
                std::process::Command::new(std::env::current_exe().unwrap())
                    .args(["version-fixture-child", version.as_str()])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
            "@exit" => {
                println!("codex-cli 0.167.1");
                std::process::exit(1);
            }
            "@silent" => std::thread::sleep(std::time::Duration::from_secs(30)),
            "@oversize" => print!("codex-cli 0.167.1 {}", "x".repeat(256)),
            _ => print!("{version}"),
        }
        return;
    }
    if args == ["auth", "logout"] || args == ["logout", "-c", "cli_auth_credentials_store=\"file\""]
    {
        logout(&args);
        return;
    }
    if args.first().map(String::as_str) == Some("app-server") {
        if let Ok(script) = std::fs::read("usage-limits.json") {
            rate_limit_server(&serde_json::from_slice(&script).unwrap());
        } else {
            app_server(&args);
        }
        return;
    }
    if args == agent_adapter::login_arguments() {
        login(&args);
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
