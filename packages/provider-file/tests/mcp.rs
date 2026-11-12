#![cfg(unix)]
use provider_file::{EXTENSION, MAX_VALUE_BYTES, OPERATION, OUTCOME, PREFIX, REASON, RESTRICTION};
use rmcp::{
    RoleClient, ServiceExt,
    model::*,
    service::{RunningService, ServiceError},
    transport::TokioChildProcess,
};
use serde_json::{Value, json};
use std::{
    fs::{self, File, FileTimes, Permissions},
    os::unix::fs::{PermissionsExt, symlink},
    path::Path,
    process::Stdio,
    time::{Duration, SystemTime},
};
use tokio::{io::AsyncReadExt, task::JoinHandle};

struct Client {
    service: RunningService<RoleClient, ()>,
    stderr: JoinHandle<Vec<u8>>,
}

impl Client {
    async fn start(marker: &str) -> Self {
        let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_provider-file"));
        command
            .env_clear()
            .env("PROVIDER_FILE_SYNTHETIC_FALLBACK", marker);
        let (transport, stderr) = TokioChildProcess::builder(command)
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn provider");
        let mut stderr = stderr.expect("capture stderr");
        let stderr = tokio::spawn(async move {
            let mut bytes = Vec::new();
            stderr
                .read_to_end(&mut bytes)
                .await
                .expect("capture provider stderr");
            bytes
        });
        let service = tokio::time::timeout(Duration::from_secs(15), ().serve(transport))
            .await
            .expect("provider startup timed out")
            .unwrap_or_else(|_| panic!("provider startup failed"));
        Self { service, stderr }
    }

    async fn call(&self, request: CallToolRequestParams) -> Value {
        let result = tokio::time::timeout(Duration::from_secs(5), self.service.call_tool(request))
            .await
            .expect("call timed out")
            .unwrap_or_else(|_| panic!("MCP call failed"));
        serde_json::to_value(result).expect("serialize result")
    }

    async fn stop(self) {
        self.service
            .cancel()
            .await
            .unwrap_or_else(|_| panic!("provider shutdown failed"));
        let bytes = tokio::time::timeout(Duration::from_secs(20), self.stderr)
            .await
            .expect("stderr capture timed out")
            .expect("stderr task failed");
        assert!(bytes.is_empty(), "provider wrote a stderr line");
    }
}

fn marker() -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    format!(
        "CONDUIT SYNTHETIC VALUE {} {}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    )
}

fn request(config: Value) -> CallToolRequestParams {
    serde_json::from_value(json!({"name":"get-secret.v1","arguments":{},"_meta":{
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}},
        "io.github.shrav-k.conduit/instance":"secret-instance",
        "io.github.shrav-k.conduit/operation":OPERATION,
        "io.github.shrav-k.conduit/invocation":"test-invocation",
        "io.github.shrav-k.conduit/caller":"setup-caller",
        "io.github.shrav-k.conduit/filters":[],
        "io.github.shrav-k.conduit/config":config,
        "io.github.shrav-k.conduit/revision":"r1",
        "io.github.shrav-k.conduit/conduit":"instance-secret-conduit"
    }}))
    .expect("build request")
}

fn with_meta(request: CallToolRequestParams, key: &str, value: Value) -> CallToolRequestParams {
    let mut wire = serde_json::to_value(request).expect("serialize request");
    wire["_meta"][key] = value;
    serde_json::from_value(wire).expect("build request")
}

fn private_file(path: &Path, bytes: &[u8]) {
    fs::write(path, bytes).expect("write fixture");
    fs::set_permissions(path, Permissions::from_mode(0o600)).expect("set fixture permissions");
}

fn failed(wire: &Value, marker: &str, reason: &str) {
    assert!(wire["_meta"][OUTCOME] == "failed");
    assert!(wire["_meta"].get(REASON).is_none());
    assert!(wire["isError"] == true);
    assert!(wire.get("structuredContent").is_none());
    assert!(wire.get("value").is_none());
    assert!(
        !wire.to_string().contains(marker),
        "answer leaked the synthetic marker"
    );
    assert!(
        wire["content"][0]["text"]
            .as_str()
            .is_some_and(|message| message.contains(reason)),
        "missing explicit failure reason"
    );
}

fn succeeded(wire: &Value, expected: &str) {
    assert!(wire["_meta"][OUTCOME] == "succeeded");
    assert!(wire["_meta"].get(REASON).is_none());
    assert!(wire["isError"] == false);
    assert!(wire["content"].as_array().is_some_and(Vec::is_empty));
    assert!(
        wire["structuredContent"]["value"] == expected,
        "incorrect secret value"
    );
    let schema = provider_file::output_schema();
    assert!(
        jsonschema::validator_for(&schema)
            .expect("output schema")
            .is_valid(&wire["structuredContent"])
    );
}

fn reset_access_time(path: &Path) -> SystemTime {
    File::open(path)
        .expect("open fixture")
        .set_times(FileTimes::new().set_accessed(SystemTime::UNIX_EPOCH))
        .expect("set access time");
    fs::metadata(path)
        .expect("fixture stat")
        .accessed()
        .expect("access time")
}

#[tokio::test]
async fn discovery_and_extension_refusal_read_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("secret");
    let marker = marker();
    private_file(&path, marker.as_bytes());
    let client = Client::start(&marker).await;
    let discovery = client
        .service
        .discover(
            serde_json::from_value(json!({
                "io.modelcontextprotocol/protocolVersion":"2026-07-28",
                "io.modelcontextprotocol/clientCapabilities":{}
            }))
            .unwrap(),
        )
        .await
        .unwrap_or_else(|_| panic!("discovery failed"));
    let wire = serde_json::to_value(discovery).unwrap();
    assert!(wire["capabilities"]["extensions"][EXTENSION]["revision"] == "v0");
    let tools = client
        .service
        .list_all_tools()
        .await
        .unwrap_or_else(|_| panic!("tool discovery failed"));
    assert!(tools.len() == 1 && tools[0].name == "get-secret.v1");
    let tool = serde_json::to_value(&tools[0]).unwrap();
    assert!(tool["outputSchema"] == provider_file::output_schema());
    for revision in [None, Some("v1")] {
        let before = reset_access_time(&path);
        let capabilities = revision.map_or(
            json!({}),
            |revision| json!({"extensions":{EXTENSION:{"revision":revision}}}),
        );
        let call = with_meta(
            request(json!({"path":path})),
            "io.modelcontextprotocol/clientCapabilities",
            capabilities,
        );
        let result = tokio::time::timeout(Duration::from_secs(5), client.service.call_tool(call))
            .await
            .expect("call timed out");
        let Err(ServiceError::McpError(error)) = result else {
            panic!("expected capability error")
        };
        assert!(error.code.0 == -32021);
        assert!(
            error.data.as_ref().unwrap()["requiredCapabilities"]["extensions"][EXTENSION]["revision"]
                == "v0"
        );
        assert!(!error.message.contains(&marker));
        assert!(!error.data.unwrap().to_string().contains(&marker));
        assert!(fs::metadata(&path).unwrap().accessed().unwrap() == before);
    }
    client.stop().await;
}

#[tokio::test]
async fn filters_return_reason_and_restriction_without_reading() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("secret");
    let marker = marker();
    private_file(&path, marker.as_bytes());
    let client = Client::start(&marker).await;
    let restriction = json!({"requirement":"example.org/test-requirements#deny.v1","arguments":{"scope":"nothing"}});
    for config in [json!({"path":path}), json!({})] {
        let before = reset_access_time(&path);
        let call = with_meta(
            request(config),
            &format!("{PREFIX}filters"),
            json!([restriction, {"requirement":"example.org/test-requirements#second.v1","arguments":{}}]),
        );
        let wire = client.call(call).await;
        assert!(wire["isError"] == true);
        assert!(wire["_meta"][REASON] == "unavailable");
        assert!(wire["_meta"][RESTRICTION] == restriction);
        assert!(wire["_meta"].get(OUTCOME).is_none());
        assert!(wire.get("structuredContent").is_none());
        assert!(!wire.to_string().contains(&marker));
        assert!(fs::metadata(&path).unwrap().accessed().unwrap() == before);
    }
    client.stop().await;
}

#[tokio::test]
async fn utf8_newline_boundaries_and_no_cache() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("secret");
    let marker = marker();
    let client = Client::start(&marker).await;
    for (content, expected) in [
        (marker.clone(), marker.clone()),
        (format!("{marker}\n"), marker.clone()),
        (format!("{marker}\n\n"), format!("{marker}\n")),
        (format!("{marker}\r\n"), format!("{marker}\r")),
        (format!("{marker} λ雪"), format!("{marker} λ雪")),
        (String::new(), String::new()),
        ("\n".into(), String::new()),
        ("x".repeat(MAX_VALUE_BYTES), "x".repeat(MAX_VALUE_BYTES)),
    ] {
        private_file(&path, content.as_bytes());
        succeeded(&client.call(request(json!({"path":path}))).await, &expected);
    }
    // Replacing and removing the file affects the very next request; no cache or copy survives.
    let replacement = dir.path().join("replacement");
    private_file(&replacement, marker.as_bytes());
    fs::rename(&replacement, &path).unwrap();
    succeeded(&client.call(request(json!({"path":path}))).await, &marker);
    fs::remove_file(&path).unwrap();
    failed(
        &client.call(request(json!({"path":path}))).await,
        &marker,
        "missing",
    );
    assert!(fs::read_dir(dir.path()).unwrap().count() == 0);
    client.stop().await;
}

#[tokio::test]
async fn unsafe_files_fail_without_echoing_or_logging() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("secret");
    let marker = marker();
    let client = Client::start(&marker).await;
    failed(
        &client.call(request(json!({"path":path}))).await,
        &marker,
        "missing",
    );
    private_file(&path, marker.as_bytes());
    // Independent checks for group/other read, write and execute, including Docker mode.
    for mode in [0o604, 0o640, 0o620, 0o602, 0o610, 0o601, 0o444, 0o644] {
        fs::set_permissions(&path, Permissions::from_mode(mode)).unwrap();
        failed(
            &client.call(request(json!({"path":path}))).await,
            &marker,
            "permissions",
        );
    }
    fs::set_permissions(&path, Permissions::from_mode(0o600)).unwrap();
    let link = dir.path().join("link");
    symlink(&path, &link).unwrap();
    failed(
        &client.call(request(json!({"path":link}))).await,
        &marker,
        "symbolic link",
    );
    let fifo = dir.path().join("fifo");
    // rustix has no mkfifoat on macOS; mkfifo(1) is on every Unix.
    let made = std::process::Command::new("mkfifo")
        .arg("-m")
        .arg("600")
        .arg(&fifo)
        .status()
        .unwrap();
    assert!(made.success());
    failed(
        &client.call(request(json!({"path":fifo}))).await,
        &marker,
        "not a regular file",
    );
    failed(
        &client.call(request(json!({"path":dir.path()}))).await,
        &marker,
        "not a regular file",
    );
    let mut too_large = marker.as_bytes().to_vec();
    too_large.resize(MAX_VALUE_BYTES + 1, b'X');
    private_file(&path, &too_large);
    failed(
        &client.call(request(json!({"path":path}))).await,
        &marker,
        "size bound",
    );
    let mut invalid = marker.as_bytes().to_vec();
    invalid.push(0xff);
    private_file(&path, &invalid);
    failed(
        &client.call(request(json!({"path":path}))).await,
        &marker,
        "not valid UTF-8",
    );
    client.stop().await;
}

#[tokio::test]
async fn unreadable_file_fails_unless_running_as_root() {
    if rustix::process::geteuid().is_root() {
        eprintln!("SKIPPED unreadable-file case: tests run as root");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("secret");
    let marker = marker();
    private_file(&path, marker.as_bytes());
    fs::set_permissions(&path, Permissions::from_mode(0o000)).unwrap();
    let client = Client::start(&marker).await;
    failed(
        &client.call(request(json!({"path":path}))).await,
        &marker,
        "unreadable",
    );
    client.stop().await;
}

#[tokio::test]
async fn extra_or_missing_locator_and_malformed_requests_fail() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("secret");
    let marker = marker();
    private_file(&path, marker.as_bytes());
    let client = Client::start(&marker).await;
    for config in [
        json!({"path":path,"value":marker}),
        json!({}),
        json!({"value":marker}),
        json!({"path":marker}),
        json!({"path":null}),
        json!({"path":"relative"}),
        json!({"path":"/invalid\u{0000}path"}),
        json!(marker),
    ] {
        let before = reset_access_time(&path);
        failed(
            &client.call(request(config)).await,
            &marker,
            "configuration",
        );
        assert!(fs::metadata(&path).unwrap().accessed().unwrap() == before);
    }
    for field in [
        "instance",
        "invocation",
        "caller",
        "operation",
        "filters",
        "revision",
        "conduit",
    ] {
        let before = reset_access_time(&path);
        let wire = client
            .call(with_meta(
                request(json!({"path":path})),
                &format!("{PREFIX}{field}"),
                Value::Null,
            ))
            .await;
        failed(
            &wire,
            &marker,
            if field == "operation" {
                "operation"
            } else {
                "metadata"
            },
        );
        assert!(fs::metadata(&path).unwrap().accessed().unwrap() == before);
    }
    client.stop().await;
}

#[tokio::test]
async fn parent_directory_symlinks_are_allowed() {
    let dir = tempfile::tempdir().unwrap();
    let real = dir.path().join("real");
    fs::create_dir(&real).unwrap();
    let marker = marker();
    private_file(&real.join("secret"), marker.as_bytes());
    let parent_link = dir.path().join("alias");
    symlink(&real, &parent_link).unwrap();
    let client = Client::start(&marker).await;
    succeeded(
        &client
            .call(request(json!({"path":parent_link.join("secret")})))
            .await,
        &marker,
    );
    client.stop().await;
}
