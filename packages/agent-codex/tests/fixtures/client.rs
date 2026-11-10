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
async fn turn(args: &[String], message: &str, number: usize) {
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
            serde_json::from_slice(&std::fs::read("settings.json").unwrap()).unwrap()
        } else {
            Value::Null
        };
        reply = json!({"args":args,"environment":env,"cwd":std::env::current_dir().unwrap(),"pid":std::process::id(),"settings":settings,"number":number,"message":message}).to_string();
    }
    if message == "BLOCK" {
        std::fs::write("blocked", b"ready").unwrap();
        while !std::path::Path::new("release").exists() {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    if message == "QUEUED" {
        std::fs::write("queued-launched", b"unexpected").unwrap();
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
#[tokio::main]
async fn main() {
    std::fs::write(
        "client-started",
        std::env::current_exe().unwrap().to_str().unwrap(),
    )
    .unwrap();
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.iter().any(|s| s == "--input-format") {
        for (i, line) in io::stdin().lock().lines().enumerate() {
            let v: Value = serde_json::from_str(&line.unwrap()).unwrap();
            turn(&args, v["message"]["content"].as_str().unwrap(), i + 1).await;
            if v["message"]["content"] == "EOF" || v["message"]["content"] == "ERROR" {
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
