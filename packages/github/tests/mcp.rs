#![cfg(unix)]

use conduit_github::{ADDRESS, EXTENSION, OUTCOME, PREFIX, REASON, RESTRICTION, TOKEN_VARIABLE};
use rmcp::{
    RoleClient, ServiceExt,
    service::{RunningService, ServiceError},
    transport::async_rw::AsyncRwTransport,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    pin::Pin,
    process::{ExitStatus, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, ready},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, ReadBuf},
    process::{Child, ChildStdout, Command},
    task::JoinHandle,
    time::timeout,
};

const WAIT: Duration = Duration::from_secs(5);

fn marker() -> String {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    format!(
        "synthetic-github-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    )
}

struct Fixture {
    _temp: tempfile::TempDir,
    home: PathBuf,
    path: PathBuf,
    markers: [String; 4],
}

impl Fixture {
    fn new(stand_in: bool) -> Self {
        let temp = tempfile::tempdir().unwrap();
        let home = temp.path().canonicalize().unwrap();
        let path = home.join("path");
        fs::create_dir(&path).unwrap();
        if stand_in {
            fs::copy(
                env!("CARGO_BIN_EXE_github-mcp-stand-in"),
                path.join("github-mcp-server"),
            )
            .unwrap();
        }
        Self {
            _temp: temp,
            home,
            path,
            markers: std::array::from_fn(|_| marker()),
        }
    }

    fn mode(&self, mode: &str) {
        fs::write(self.home.join("mode"), mode).unwrap();
    }

    fn seen(&self) -> Vec<Value> {
        found(&self.home)
    }

    fn no_leak(&self, bytes: &[u8]) {
        assert!(
            self.markers.iter().all(|marker| !bytes
                .windows(marker.len())
                .any(|window| window == marker.as_bytes())),
            "front disclosed a synthetic marker"
        );
    }

    async fn start(&self, token: Option<&str>) -> Front {
        let mut command = Command::new(env!("CARGO_BIN_EXE_conduit-github"));
        command
            .env_clear()
            .env("PATH", &self.path)
            .env("HOME", &self.home)
            .env("GH_TOKEN", &self.markers[1])
            .env("GITHUB_TOKEN", &self.markers[2])
            .env("EXTRA", &self.markers[3])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .process_group(0)
            .kill_on_drop(true);
        if let Some(token) = token {
            command.env(TOKEN_VARIABLE, token);
        }
        let mut child = command.spawn().unwrap();
        let mut stderr = child.stderr.take().unwrap();
        let stderr = tokio::spawn(async move {
            let mut bytes = Vec::new();
            stderr.read_to_end(&mut bytes).await.unwrap();
            bytes
        });
        let raw = Arc::new(Mutex::new(Vec::new()));
        let service = timeout(
            WAIT,
            ().serve(AsyncRwTransport::new_client(
                Capture {
                    stdout: child.stdout.take().unwrap(),
                    raw: raw.clone(),
                },
                child.stdin.take().unwrap(),
            )),
        )
        .await
        .expect("front startup timed out")
        .unwrap_or_else(|_| panic!("front could not initialize"));
        Front {
            child,
            service,
            stderr,
            raw,
        }
    }

    async fn active(&self) -> Front {
        self.start(Some(&self.markers[0])).await
    }
}

struct Capture {
    stdout: ChildStdout,
    raw: Arc<Mutex<Vec<u8>>>,
}

impl AsyncRead for Capture {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let start = buf.filled().len();
        ready!(Pin::new(&mut self.stdout).poll_read(cx, buf))?;
        self.raw
            .lock()
            .unwrap()
            .extend_from_slice(&buf.filled()[start..]);
        Poll::Ready(Ok(()))
    }
}

fn found(home: &Path) -> Vec<Value> {
    fs::read_to_string(home.join("seen"))
        .unwrap_or_default()
        .lines()
        .map(|line| {
            serde_json::from_str(line).unwrap_or_else(|_| panic!("invalid stand-in evidence"))
        })
        .collect()
}

struct Front {
    child: Child,
    service: RunningService<RoleClient, ()>,
    stderr: JoinHandle<Vec<u8>>,
    raw: Arc<Mutex<Vec<u8>>>,
}

impl Front {
    async fn call(&self, fixture: &Fixture, wire: Value) -> Value {
        let request =
            serde_json::from_value(wire).unwrap_or_else(|_| panic!("invalid test request"));
        let result = timeout(WAIT, self.service.call_tool(request))
            .await
            .expect("front call timed out")
            .unwrap_or_else(|_| panic!("front returned a protocol error"));
        let value = serde_json::to_value(result).unwrap();
        fixture.no_leak(&self.raw.lock().unwrap());
        fixture.no_leak(value.to_string().as_bytes());
        value
    }

    async fn stop(self, fixture: &Fixture) {
        assert!(self.finish(fixture, None).await.success());
    }

    async fn finish(
        self,
        fixture: &Fixture,
        signal: Option<rustix::process::Signal>,
    ) -> ExitStatus {
        let Self {
            mut child,
            service,
            stderr,
            raw,
        } = self;
        if let Some(signal) = signal {
            use rustix::process::{Signal, kill_process, kill_process_group};

            let pid = rustix::process::Pid::from_raw(child.id().unwrap() as i32).unwrap();
            if signal == Signal::KILL {
                kill_process_group(pid, signal).unwrap();
            } else {
                kill_process(pid, signal).unwrap();
            }
        } else {
            timeout(WAIT, service.cancel())
                .await
                .expect("front stdin close timed out")
                .expect("front transport did not close");
        }
        let status = timeout(WAIT, child.wait())
            .await
            .expect("front did not exit")
            .unwrap();
        let stderr = timeout(WAIT, stderr)
            .await
            .expect("stderr did not close")
            .unwrap();
        fixture.no_leak(&stderr);
        fixture.no_leak(&raw.lock().unwrap());
        assert!(stderr.is_empty(), "front emitted stderr");
        status
    }
}

fn call(operation: &str, arguments: Value) -> Value {
    json!({"name":operation,"arguments":arguments,"_meta":{
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}},
        format!("{PREFIX}instance"):"resource", format!("{PREFIX}operation"):format!("{ADDRESS}#{operation}"),
        format!("{PREFIX}invocation"):"invocation",format!("{PREFIX}caller"):"caller",
        format!("{PREFIX}filters"):[], format!("{PREFIX}revision"):"r1",format!("{PREFIX}conduit"):"read",
        format!("{PREFIX}config"):{"repository":"octo/allowed","token":{"kind":"Resource","id":"00000000-0000-4000-8000-000000000001"}}
    }})
}

fn failed(result: &Value, reason: &str) {
    assert_eq!(result["_meta"][OUTCOME], "failed");
    assert_eq!(result["isError"], true);
    assert!(result.get("structuredContent").is_none());
    assert!(
        result["content"][0]["text"]
            .as_str()
            .is_some_and(|message| message.contains(reason)),
        "expected fixed failure diagnosis"
    );
}

fn pid_exists(pid: u64) -> bool {
    rustix::process::test_kill_process(
        rustix::process::Pid::from_raw(i32::try_from(pid).unwrap()).unwrap(),
    )
    .is_ok()
}

async fn gone(pid: u64) {
    timeout(WAIT, async {
        while pid_exists(pid) {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("stand-in still running");
}

#[tokio::test]
async fn reads_go_to_the_configured_repository_alone() {
    let f = Fixture::new(true);
    let front = f.active().await;
    for arguments in [
        json!({"number":1,"repository":"octo/other"}),
        json!({"number":1,"owner":"other"}),
        json!({"number":1,"repo":"other"}),
        json!({"number":0}),
        json!({"number":1.4}),
        json!({}),
    ] {
        failed(
            &front.call(&f, call("issue.read.v1", arguments)).await,
            "arguments require only",
        );
    }
    for config in [
        json!("invalid"),
        json!({"repository":"octo/allowed"}),
        json!({"repository":"octo/allowed","token":{"kind":"Resource","id":"id"},"extra":true}),
        json!({"repository":"octo/..","token":{"kind":"Resource","id":"id"}}),
        json!({"repository":"octo/allowed","token":"literal"}),
    ] {
        let mut request = call("issue.read.v1", json!({"number":1}));
        request["_meta"][format!("{PREFIX}config")] = config;
        failed(&front.call(&f, request).await, "configuration requires");
    }
    failed(
        &front
            .call(&f, call("issue.list.v1", json!({"limit":101})))
            .await,
        "integer limit",
    );
    assert!(f.seen().is_empty());
    let operations = [
        (
            "issue.read.v1",
            json!({"number":1}),
            "issue_read",
            json!({"owner":"octo","repo":"allowed","method":"get","issue_number":1}),
        ),
        (
            "issue.list.v1",
            json!({"limit":4}),
            "list_issues",
            json!({"owner":"octo","repo":"allowed","perPage":4}),
        ),
        (
            "pull-request.read.v1",
            json!({"number":9}),
            "pull_request_read",
            json!({"owner":"octo","repo":"allowed","method":"get","pullNumber":9}),
        ),
        (
            "pull-request.list.v1",
            json!({"limit":5}),
            "list_pull_requests",
            json!({"owner":"octo","repo":"allowed","perPage":5}),
        ),
    ];
    for (index, (operation, arguments, tool, expected)) in operations.into_iter().enumerate() {
        let number = arguments[if operation.contains(".list.") {
            "limit"
        } else {
            "number"
        }]
        .clone();
        let result = front.call(&f, call(operation, arguments)).await;
        assert_eq!(result["_meta"][OUTCOME], "succeeded");
        assert!(
            result["structuredContent"]
                == json!({"data":{"owner":"octo","repo":"allowed","number":number}})
        );
        let log = f.seen();
        assert_eq!(log.len(), index + 2);
        assert!(log[index + 1] == json!({"tool":tool,"arguments":expected}));
    }
    let mut request = call("issue.read.v1", json!({"number":2}));
    request["_meta"][format!("{PREFIX}config")]["repository"] = json!("Else/Project_2");
    assert!(front.call(&f, request).await["structuredContent"]["data"]["repo"] == "Project_2");
    assert!(f.seen().last().unwrap()["arguments"]["owner"] == "Else");
    front.stop(&f).await;
}

#[tokio::test]
async fn the_token_reaches_the_server_and_nothing_else() {
    let f = Fixture::new(true);
    for token in [None, Some("")] {
        let front = f.start(token).await;
        let reply = front
            .call(&f, call("issue.read.v1", json!({"number":1})))
            .await;
        failed(&reply, "is absent or empty");
        front.stop(&f).await;
        assert!(f.seen().is_empty());
    }
    let front = f.active().await;
    f.mode("error");
    let reply = front
        .call(&f, call("issue.read.v1", json!({"number":1})))
        .await;
    failed(&reply, "before [token] after [token]");
    assert!(reply["content"][0]["text"].as_str().unwrap().len() <= 2048);
    let seen = f.seen();
    assert_eq!(
        seen[0]["environment"],
        json!([TOKEN_VARIABLE, "HOME", "PATH"])
    );
    assert!(seen[0]["digest"] == format!("{:x}", Sha256::digest(f.markers[0].as_bytes())));
    assert!(
        seen[0]["arguments"]
            == json!(["stdio", "--read-only", "--toolsets", "issues,pull_requests"])
    );
    f.no_leak(serde_json::to_string(&seen).unwrap().as_bytes());
    f.mode("token");
    let reply = front
        .call(&f, call("issue.read.v1", json!({"number":1})))
        .await;
    assert_eq!(reply["_meta"][OUTCOME], "succeeded");
    assert!(
        reply["structuredContent"]["data"]
            == json!({"first":"[token]","nested":["prefix [token] suffix",{"[token]":"key"}]})
    );
    front.stop(&f).await;
}

#[tokio::test]
async fn server_failures_fail_explicitly() {
    let f = Fixture::new(false);
    let request = || call("issue.read.v1", json!({"number":1}));
    let front = f.active().await;
    failed(&front.call(&f, request()).await, "was not found on PATH");
    front.stop(&f).await;
    fs::write(f.path.join("github-mcp-server"), "non-executable fixture").unwrap();
    let front = f.active().await;
    failed(&front.call(&f, request()).await, "could not be started");
    front.stop(&f).await;
    for (mode, diagnosis) in [
        (
            "missing-tool",
            "stand-in-232 is missing tool pull_request_read",
        ),
        (
            "missing-argument",
            "stand-in-232 tool list_issues is missing input property perPage",
        ),
    ] {
        let f = Fixture::new(true);
        f.mode(mode);
        let front = f.active().await;
        for _ in 0..2 {
            failed(&front.call(&f, request()).await, diagnosis);
        }
        assert_eq!(f.seen().len(), 1);
        front.stop(&f).await;
    }
    let f = Fixture::new(true);
    let front = f.active().await;
    for (mode, diagnosis) in [
        ("not-json", "not a JSON object or array"),
        ("scalar", "not a JSON object or array"),
        ("too-big", "exceeds 8 MiB"),
        ("exit", "exited during the call"),
    ] {
        f.mode(mode);
        failed(&front.call(&f, request()).await, diagnosis);
    }
    let old = f.seen()[0]["process"].as_u64().unwrap();
    gone(old).await;
    f.mode("");
    assert_eq!(
        front.call(&f, request()).await["_meta"][OUTCOME],
        "succeeded"
    );
    let new = f
        .seen()
        .into_iter()
        .filter_map(|event| event["process"].as_u64())
        .next_back()
        .unwrap();
    assert_ne!(old, new);
    front.stop(&f).await;
    gone(new).await;
}

#[tokio::test]
async fn the_server_stops_with_the_front() {
    for signal in [None, Some(rustix::process::Signal::TERM)] {
        let f = Fixture::new(true);
        let front = f.active().await;
        assert_eq!(
            front
                .call(&f, call("issue.read.v1", json!({"number":1})))
                .await["_meta"][OUTCOME],
            "succeeded"
        );
        let pid = f.seen()[0]["process"].as_u64().unwrap();
        assert!(pid_exists(pid));
        assert!(front.finish(&f, signal).await.success());
        gone(pid).await;
    }
}

#[tokio::test]
async fn repository_names_containing_the_token_fail_before_server_start() {
    for owner in [true, false] {
        let f = Fixture::new(true);
        let front = f.active().await;
        let mut request = call("issue.read.v1", json!({"number":1}));
        request["_meta"][format!("{PREFIX}config")]["repository"] = if owner {
            json!(format!("{}-owner/repo", f.markers[0]))
        } else {
            json!(format!("owner/repo-{}", f.markers[0]))
        };
        let result = front.call(&f, request).await;
        front.stop(&f).await;
        let seen = f.home.join("seen");
        if seen.exists() {
            f.no_leak(&fs::read(&seen).unwrap());
        }
        assert!(
            !seen.exists(),
            "an invalid configuration started the stand-in"
        );
        failed(
            &result,
            "configured owner or repository contains the active credential",
        );
    }
}

#[tokio::test]
async fn credential_bearing_filters_are_omitted_from_the_raw_refusal() {
    for in_key in [true, false] {
        let f = Fixture::new(true);
        let front = f.active().await;
        let mut request = call("issue.read.v1", json!({"number":1}));
        let arguments = if in_key {
            json!({"outer":[{format!("prefix-{}", f.markers[0]):"value"}]})
        } else {
            json!({"outer":[{"value":format!("prefix-{}-suffix", f.markers[0])}]})
        };
        request["_meta"][format!("{PREFIX}filters")] =
            json!([{"requirement":"example.org/unknown#limited.v1","arguments":arguments}]);
        let result = front.call(&f, request).await;
        front.stop(&f).await;
        assert!(f.seen().is_empty());
        assert!(result["_meta"].get(RESTRICTION).is_none());
        failed(&result, "requested filter contains the active credential");
    }
}

#[tokio::test]
async fn a_stubborn_server_stops_on_front_signals_and_abrupt_exit() {
    use rustix::process::{Pid, Signal, kill_process};
    use std::os::unix::process::ExitStatusExt;

    struct KillOnFailure(Option<Pid>);
    impl Drop for KillOnFailure {
        fn drop(&mut self) {
            if let Some(pid) = self.0 {
                let _ = kill_process(pid, Signal::KILL);
            }
        }
    }

    for signal in [Signal::HUP, Signal::QUIT, Signal::KILL] {
        let f = Fixture::new(true);
        f.mode("stay-alive-after-eof");
        let front = f.active().await;
        let result = front
            .call(&f, call("issue.read.v1", json!({"number":1})))
            .await;
        assert_eq!(result["_meta"][OUTCOME], "succeeded");
        let pid = f.seen()[0]["process"].as_u64().unwrap();
        assert!(pid_exists(pid));
        let mut cleanup = KillOnFailure(Pid::from_raw(i32::try_from(pid).unwrap()));
        let status = front.finish(&f, Some(signal)).await;
        gone(pid).await;
        cleanup.0 = None;
        if signal == Signal::KILL {
            assert_eq!(status.signal(), Some(signal.as_raw()));
        } else {
            assert!(status.success());
        }
    }
}

#[tokio::test]
async fn discovery_advertises_extension_and_refuses_unsupported_requests() {
    let f = Fixture::new(true);
    let front = f.active().await;
    let discovery = timeout(
        WAIT,
        front.service.discover(
            serde_json::from_value(
                json!({"io.modelcontextprotocol/protocolVersion":"2026-07-28",
            "io.modelcontextprotocol/clientCapabilities":{}}),
            )
            .unwrap(),
        ),
    )
    .await
    .expect("discover timed out")
    .expect("discover failed");
    let wire = serde_json::to_value(discovery).unwrap();
    assert_eq!(
        wire["capabilities"]["extensions"][EXTENSION]["revision"],
        "v0"
    );
    let mut missing = call("issue.read.v1", json!({"number":1}));
    missing["_meta"]
        .as_object_mut()
        .unwrap()
        .remove("io.modelcontextprotocol/clientCapabilities");
    let result = timeout(
        WAIT,
        front
            .service
            .call_tool(serde_json::from_value(missing).unwrap()),
    )
    .await
    .unwrap();
    assert!(matches!(result, Err(ServiceError::McpError(error)) if error.code.0 == -32021));
    let mut filtered = call("issue.read.v1", json!({"number":1}));
    let restriction = json!({"requirement":"example.org/unknown#limited.v1","arguments":{}});
    filtered["_meta"][format!("{PREFIX}filters")] = json!([restriction]);
    let result = front.call(&f, filtered).await;
    assert_eq!(result["_meta"][REASON], "unavailable");
    assert!(result["_meta"].get(OUTCOME).is_none());
    assert_eq!(result["_meta"][RESTRICTION], restriction);
    assert!(f.seen().is_empty());
    front.stop(&f).await;
}
