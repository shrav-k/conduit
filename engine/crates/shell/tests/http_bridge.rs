mod runtime_support;
use runtime_support::*;
use serde_json::json;
use std::{path::Path, sync::atomic::Ordering, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
};
const INSTANCE: &str = "spiffe://bridge.test/conduit/instance/1234";
const ENGINE: &str = "spiffe://bridge.test/conduit/engine";
fn files(path: &Path, authority: &TestAuthority) {
    let (cert, key) = authority.issue(ENGINE);
    for (name, value) in [
        ("root.pem", authority.root.clone()),
        ("peer-id", INSTANCE.into()),
        ("bearer", "synthetic-test-bearer".into()),
        ("client.pem", cert),
        ("client.key", key),
    ] {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(path.join(name))
            .unwrap();
        file.write_all(value.as_bytes()).unwrap();
    }
}
fn spawn(path: &Path, url: &str) -> Child {
    Command::new(env!("CARGO_BIN_EXE_conduit"))
        .arg("__http-bridge")
        .arg(url)
        .env_clear()
        .env("HTTP_BRIDGE_DIR", path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .unwrap()
}
fn options(cert: String, key: String, sse: bool) -> GateOptions {
    GateOptions {
        cert,
        key,
        sse,
        client_root: None,
        auth: None,
        redirect: None,
        count_file: None,
        get_events: false,
        delay_first: false,
    }
}
async fn line(lines: &mut tokio::io::Lines<BufReader<tokio::process::ChildStdout>>) -> String {
    tokio::time::timeout(Duration::from_secs(15), lines.next_line())
        .await
        .unwrap()
        .unwrap()
        .unwrap()
}
const INIT: &str = r#"{ "jsonrpc" : "2.0", "id": 1, "method":"initialize", "params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"test","version":"1"}} }"#;
#[tokio::test]
async fn bridge_preserves_bytes_headers_credentials_after_directory_removal_and_deletes() {
    for sse in [false, true] {
        let authority = TestAuthority::new();
        let (cert, key) = authority.issue(INSTANCE);
        let gate = Gate::start(options(cert, key, sse)).await;
        let temp = tempfile::tempdir().unwrap();
        files(temp.path(), &authority);
        let mut child = spawn(temp.path(), &gate.url);
        let mut stdin = child.stdin.take().unwrap();
        let mut stdout = BufReader::new(child.stdout.take().unwrap()).lines();
        stdin
            .write_all(format!("{INIT}\n").as_bytes())
            .await
            .unwrap();
        let reply = line(&mut stdout).await;
        assert!(reply.starts_with("{ \"jsonrpc\" :"));
        let parsed: serde_json::Value = serde_json::from_str(&reply).unwrap();
        assert_eq!(parsed["result"]["protocolVersion"], "2025-11-25");
        std::fs::remove_dir_all(temp.path()).unwrap();
        let body = r#"{  "jsonrpc":"2.0", "id" : 2, "method":"tools/call", "params": {"name":"odd", "arguments":{"s":"a\\b\t\u1234\""}} }"#;
        stdin
            .write_all(format!("{body}\r\n").as_bytes())
            .await
            .unwrap();
        let reply = line(&mut stdout).await;
        assert_eq!(
            reply,
            r#"{ "jsonrpc": "2.0", "id": 2, "result": {"content":[],"structuredContent":{},"_meta":{"io.github.shrav-k.conduit/outcome":"succeeded"}} }"#
        );
        stdin
            .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
            .await
            .unwrap();
        wait(|| {
            gate.received.lock().unwrap().iter().any(|r| {
                r.body == b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}"
            })
        })
        .await;
        drop(stdin);
        assert!(
            tokio::time::timeout(Duration::from_secs(15), child.wait())
                .await
                .unwrap()
                .unwrap()
                .success()
        );
        wait(|| gate.ended.load(Ordering::SeqCst)).await;
        let records = gate.received.lock().unwrap();
        assert!(records.iter().any(|r| r.body == INIT.as_bytes()));
        assert!(records.iter().any(|r| r.body == body.as_bytes()));
        assert!(records.iter().all(|r| {
            r.headers
                .get("authorization")
                .is_some_and(|v| v == "Bearer synthetic-test-bearer")
        }));
        assert!(
            records
                .iter()
                .filter(|r| r.body != INIT.as_bytes())
                .all(|r| r.headers.contains_key("mcp-session-id")
                    && r.headers
                        .get("mcp-protocol-version")
                        .is_some_and(|v| v == "2025-11-25"))
        );
        assert!(records.iter().any(|r| r.method == "GET"));
        assert!(records.iter().any(|r| r.method == "DELETE"));
    }
}
#[tokio::test]
async fn bridge_refuses_wrong_uri_self_signed_other_root_and_http_without_http_bytes() {
    for kind in [
        "uri",
        "self-signed",
        "other-root",
        "http",
        "userinfo",
        "empty-userinfo",
        "fragment",
    ] {
        let authority = TestAuthority::new();
        let (cert, key) = match kind {
            "uri" => authority.issue("spiffe://bridge.test/conduit/instance/other"),
            "self-signed" => TestAuthority::self_signed(INSTANCE),
            "other-root" => TestAuthority::new().issue(INSTANCE),
            _ => authority.issue(INSTANCE),
        };
        let gate = Gate::start(options(cert, key, false)).await;
        let temp = tempfile::tempdir().unwrap();
        files(temp.path(), &authority);
        let url = match kind {
            "http" => gate.url.replacen("https:", "http:", 1),
            "userinfo" => gate.url.replacen("://", "://name@", 1),
            "empty-userinfo" => gate.url.replacen("://", "://@", 1),
            "fragment" => format!("{}#fragment", gate.url),
            _ => gate.url.clone(),
        };
        let mut child = spawn(temp.path(), &url);
        let _ = child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(format!("{INIT}\n").as_bytes())
            .await;
        let status = tokio::time::timeout(Duration::from_secs(15), child.wait())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            status.code(),
            Some(conduit_shell::http_bridge::PROOF_REFUSED),
            "{kind}"
        );
        assert!(gate.received.lock().unwrap().is_empty());
    }
}
#[tokio::test]
async fn bridge_presents_engine_client_certificate_and_does_not_follow_redirect() {
    let authority = TestAuthority::new();
    let (cert, key) = authority.issue(INSTANCE);
    let mut config = options(cert, key, false);
    config.client_root = Some(authority.root.clone());
    let gate = Gate::start(config).await;
    let temp = tempfile::tempdir().unwrap();
    files(temp.path(), &authority);
    let mut child = spawn(temp.path(), &gate.url);
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap()).lines();
    stdin
        .write_all(format!("{INIT}\n").as_bytes())
        .await
        .unwrap();
    let _ = line(&mut stdout).await;
    assert!(
        gate.received
            .lock()
            .unwrap()
            .iter()
            .all(|r| r.client_uri.as_deref() == Some(ENGINE))
    );
    drop(stdin);
    assert!(child.wait().await.unwrap().success());
    let (cert, key) = authority.issue(INSTANCE);
    let mut config = options(cert, key, false);
    config.redirect = Some(gate.url.clone());
    let redirect = Gate::start(config).await;
    let before = gate.received.lock().unwrap().len();
    let mut child = spawn(temp.path(), &redirect.url);
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(format!("{INIT}\n").as_bytes())
        .await
        .unwrap();
    let status = tokio::time::timeout(Duration::from_secs(15), child.wait())
        .await
        .unwrap()
        .unwrap();
    assert!(!status.success());
    assert_ne!(
        status.code(),
        Some(conduit_shell::http_bridge::PROOF_REFUSED)
    );
    assert_eq!(gate.received.lock().unwrap().len(), before);
    assert_eq!(redirect.received.lock().unwrap().len(), 1);
}
#[tokio::test]
async fn bridge_restarts_with_fresh_initialize_and_never_reuses_session() {
    let authority = TestAuthority::new();
    let (cert, key) = authority.issue(INSTANCE);
    let gate = Gate::start(options(cert, key, true)).await;
    let temp = tempfile::tempdir().unwrap();
    files(temp.path(), &authority);
    let mut old_session = String::new();
    for _ in 0..2 {
        let mut child = spawn(temp.path(), &gate.url);
        let mut stdin = child.stdin.take().unwrap();
        let mut stdout = BufReader::new(child.stdout.take().unwrap()).lines();
        stdin
            .write_all(format!("{INIT}\n").as_bytes())
            .await
            .unwrap();
        let _ = line(&mut stdout).await;
        let session = gate.session.lock().unwrap().clone();
        assert!(session != old_session);
        old_session = session;
        drop(stdin);
        assert!(child.wait().await.unwrap().success());
    }
    let records = gate.received.lock().unwrap();
    assert_eq!(
        records.iter().filter(|r| r.body == INIT.as_bytes()).count(),
        2
    );
    assert!(
        records
            .iter()
            .filter(|r| r.body == INIT.as_bytes())
            .all(|r| !r.headers.contains_key("mcp-session-id"))
    );
    assert_eq!(
        json!(records.iter().filter(|r| r.method == "DELETE").count()),
        json!(2)
    );
}

#[tokio::test]
async fn bridge_get_events_reconnect_with_a_bound_and_posts_complete_concurrently() {
    let authority = TestAuthority::new();
    let (cert, key) = authority.issue(INSTANCE);
    let mut config = options(cert, key, true);
    config.get_events = true;
    config.delay_first = true;
    let gate = Gate::start(config).await;
    let temp = tempfile::tempdir().unwrap();
    files(temp.path(), &authority);
    let mut child = spawn(temp.path(), &gate.url);
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap()).lines();
    stdin
        .write_all(format!("{INIT}\n").as_bytes())
        .await
        .unwrap();
    let _ = line(&mut stdout).await;
    let event = line(&mut stdout).await;
    assert_eq!(
        event,
        r#"{ "jsonrpc":"2.0", "method":"notifications/test", "params":{"s":"x\n"} }"#
    );
    for id in [2, 3] {
        let call = json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{"name":"ping","arguments":{}}});
        stdin
            .write_all(format!("{call}\n").as_bytes())
            .await
            .unwrap();
    }
    let mut replies = Vec::new();
    while replies.len() < 2 {
        let parsed: serde_json::Value = serde_json::from_str(&line(&mut stdout).await).unwrap();
        if let Some(id) = parsed["id"].as_i64() {
            replies.push(id);
        }
    }
    replies.sort();
    assert_eq!(replies, vec![2, 3]);
    wait(|| {
        gate.received
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.method == "GET")
            .count()
            >= 2
    })
    .await;
    {
        let records = gate.received.lock().unwrap();
        let gets = records
            .iter()
            .filter(|r| r.method == "GET")
            .collect::<Vec<_>>();
        assert!(gets[1].at.duration_since(gets[0].at) >= Duration::from_millis(950));
    }
    drop(stdin);
    assert!(child.wait().await.unwrap().success());
}

#[tokio::test]
async fn bridge_session_404_exits_while_stdin_stays_open() {
    let authority = TestAuthority::new();
    let (cert, key) = authority.issue(INSTANCE);
    let gate = Gate::start(options(cert, key, true)).await;
    let temp = tempfile::tempdir().unwrap();
    files(temp.path(), &authority);
    let mut child = spawn(temp.path(), &gate.url);
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap()).lines();
    stdin
        .write_all(format!("{INIT}\n").as_bytes())
        .await
        .unwrap();
    let _ = line(&mut stdout).await;
    gate.ended.store(true, Ordering::SeqCst);
    stdin
        .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\",\"params\":{}}\n")
        .await
        .unwrap();
    let status = tokio::time::timeout(Duration::from_secs(15), child.wait())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(status.code(), Some(1));
    drop(stdin);
}
