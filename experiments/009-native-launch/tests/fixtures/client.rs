//! Offline native-client stand-in. It cannot launch an agent or read owner credentials.
use serde_json::{Value, json};
use std::{
    io::{self, BufRead, Write},
    time::Duration,
};
fn emit(value: Value) {
    println!("{value}");
    io::stdout().flush().unwrap();
}
fn fixture_event(args: &[String], turn: usize) {
    emit(
        json!({"type":"fixture","pid":std::process::id(),"turn":turn,"args":args,"environment":std::env::vars().collect::<std::collections::BTreeMap<_,_>>(),"cwd":std::env::current_dir().unwrap()}),
    );
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.contains(&"--version".to_string()) {
        println!("offline-fixture-1");
        return;
    }
    if args.contains(&"--input-format".to_string()) {
        for (index, line) in io::stdin().lock().lines().enumerate() {
            let value: Value = serde_json::from_str(&line.unwrap()).unwrap();
            let prompt = value["message"]["content"].as_str().unwrap();
            fixture_event(&args, index + 1);
            emit(
                json!({"type":"system","subtype":"init","tools":["mcp__stub__files_read_v1"],"mcp_servers":[{"name":"stub","status":"connected"}],"apiKeySource":"fixture-subscription"}),
            );
            if prompt == "EOF" {
                return;
            }
            if prompt == "HANG" {
                std::thread::sleep(Duration::from_secs(5));
            }
            emit(
                json!({"type":"result","is_error":prompt=="ERROR","result":format!("fixture turn {}",index+1)}),
            );
        }
    } else {
        let resumed = args.contains(&"resume".to_string());
        if resumed {
            assert!(args.iter().any(|arg| arg == "fixture-exact-session"));
            assert!(!args.contains(&"--last".to_string()));
        }
        fixture_event(&args, if resumed { 2 } else { 1 });
        emit(
            json!({"type":"thread.started","thread_id":if args.last().is_some_and(|arg|arg=="MISMATCH") {"wrong-session"} else {"fixture-exact-session"}}),
        );
        if args.last().is_some_and(|arg| arg == "EOF") {
            return;
        }
        if args.last().is_some_and(|arg| arg == "HANG") {
            std::thread::sleep(Duration::from_secs(5));
        }
        emit(
            json!({"type":"item.completed","item":{"type":"agent_message","text":if resumed {"fixture turn 2"} else {"fixture turn 1"}}}),
        );
        emit(
            json!({"type":if args.last().is_some_and(|arg|arg=="ERROR") {"turn.failed"} else {"turn.completed"}}),
        );
        if args.last().is_some_and(|arg| arg == "BAD_EXIT") {
            std::process::exit(7);
        }
    }
}
