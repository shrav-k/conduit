//! Tiny real stdio MCP process; built only as test support, never packaged.
#[cfg(unix)]
fn main() {
    use runtime_provider_kit::names;
    use serde_json::{Value, json};
    use std::{
        fs,
        io::{self, BufRead, Write},
        path::Path,
        process::{Command, Stdio},
    };
    if std::env::args().any(|a| a == "--child") {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
    }
    let state = std::env::var_os(names::STATE_DIR_VARIABLE).unwrap();
    let state = Path::new(&state);
    let mut starts = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(state.join("starts"))
        .unwrap();
    writeln!(starts, "started").unwrap();
    let child = Command::new(std::env::current_exe().unwrap())
        .arg("--child")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    fs::write(
        state.join("pids.json"),
        json!([std::process::id(), child.id()]).to_string(),
    )
    .unwrap();
    // Only names and explicitly synthetic ordinary values are recorded. No credentials.
    let names: Vec<String> = std::env::vars_os()
        .map(|(k, _)| k.to_string_lossy().into_owned())
        .collect();
    let synthetic: std::collections::BTreeMap<String, String> = [
        "HOME", "LOGNAME", "PATH", "SHELL", "TERM", "USER", "ORDINARY",
    ]
    .into_iter()
    .filter_map(|name| std::env::var(name).ok().map(|v| (name.into(), v)))
    .collect();
    fs::write(
        state.join("environment.json"),
        json!({"names":names,"synthetic":synthetic}).to_string(),
    )
    .unwrap();
    for line in io::stdin().lock().lines() {
        let Ok(line) = line else { break };
        let Ok(input) = serde_json::from_str::<Value>(&line) else {
            break;
        };
        if let Some(id) = input.get("id") {
            let result = match input["method"].as_str().unwrap_or("") {
                "initialize" => {
                    json!({"protocolVersion":"2025-03-26","capabilities":{"tools":{}},"serverInfo":{"name":"test-run","version":"1"}})
                }
                "tools/list" => json!({"tools":[]}),
                _ => {
                    json!({"content":[{"type":"text","text":"reached real process"}],"isError":false})
                }
            };
            println!("{}", json!({"jsonrpc":"2.0","id":id,"result":result}));
            io::stdout().flush().unwrap();
        }
    }
    // Keep the tree alive on stdin loss so stop tests prove the guard's signal, not EOF.
    let _ = child;
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
#[cfg(not(unix))]
fn main() {}
