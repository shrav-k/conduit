use axum::{
    body::{Body, to_bytes},
    extract::{Request, State},
    middleware::{Next, from_fn_with_state},
    response::Response,
};
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    model::*,
    service::RequestContext,
    service::{ClientLifecycleMode, ClientServiceExt},
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
    },
};
use serde_json::{Value, json};
use std::{
    borrow::Cow,
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Output, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tempfile::TempDir;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{Child, Command},
    sync::Notify,
    time::timeout,
};
#[path = "../src/names.rs"]
mod names;
const BEARER: &str = "synthetic-scr-bearer-only";
const SENTINEL: &str = "synthetic-parent-sentinel";
const MARKER: &str = "synthetic-output-marker";
const ESCAPED: &str = "synthetic-quote\"and\\slash";
fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_scr-client")
}
#[derive(Default)]
struct Wire {
    posts: Mutex<Vec<Value>>,
    authorized: Mutex<Vec<bool>>,
    notify: Notify,
    lists: AtomicUsize,
    fail_list_at: usize,
    pages: bool,
    endless: bool,
    legacy: bool,
    ack: Option<PathBuf>,
    ack_observed: Mutex<Vec<bool>>,
    release: Notify,
}
impl Wire {
    fn posts(&self) -> Vec<Value> {
        self.posts.lock().unwrap().clone()
    }
    fn count(&self, method: &str) -> usize {
        self.posts()
            .iter()
            .filter(|p| p["method"] == method)
            .count()
    }
    async fn wait_count(&self, method: &str, n: usize) {
        timeout(Duration::from_secs(10), async {
            loop {
                let notified = self.notify.notified();
                if self.count(method) >= n {
                    break;
                }
                notified.await;
            }
        })
        .await
        .unwrap();
    }
}
#[derive(Clone)]
struct Server(Arc<Wire>);
impl ServerHandler for Server {
    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        if self.0.legacy {
            Cow::Borrowed(ProtocolVersion::KNOWN_VERSIONS)
        } else {
            Cow::Owned(vec![ProtocolVersion::V_2026_07_28])
        }
    }
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":if self.0.legacy{"2025-11-25"}else{"2026-07-28"},"serverInfo":{"name":"scr-test","version":"1"},"capabilities":{"tools":{}}})).unwrap()
    }
    async fn discover(&self, _: RequestContext<RoleServer>) -> Result<DiscoverResult, ErrorData> {
        if self.0.legacy {
            return Err(ErrorData::method_not_found::<DiscoverRequestMethod>());
        }
        Ok(DiscoverResult::from_server_info(
            self.supported_protocol_versions().into_owned(),
            self.get_info(),
        ))
    }
    async fn list_tools(
        &self,
        r: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let n = self.0.lists.fetch_add(1, Ordering::SeqCst) + 1;
        if n == self.0.fail_list_at {
            return Err(ErrorData::internal_error("crafted discovery failure", None));
        }
        let second = r.and_then(|r| r.cursor).is_some();
        Ok(serde_json::from_value(json!({"tools":[{"name":if second{"other.v1"}else{"effect.write.v1"},"inputSchema":{"type":"object"}}],"nextCursor":if self.0.endless || self.0.pages&&!second{Some("page-two")}else{None}})).unwrap())
    }
    async fn call_tool(
        &self,
        r: CallToolRequestParams,
        _: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let args = r.arguments.unwrap_or_default();
        if args.get("action") == Some(&json!("error")) {
            return Err(ErrorData::new(
                ErrorCode(-32021),
                format!("{MARKER} {ESCAPED} {BEARER}"),
                Some(json!({names::key("reason"):"invalid"})),
            ));
        }
        if args.get("action") == Some(&json!("input")) {
            let input: InputRequiredResult = serde_json::from_value(
                json!({"resultType":"input_required","requestState":"script-must-repeat"}),
            )
            .unwrap();
            return Ok(input.into());
        }
        let restriction = args
            .get("restriction")
            .cloned()
            .unwrap_or_else(|| json!({"requirement":"test#filter.v1"}));
        let result:CallToolResult=serde_json::from_value(json!({"content":[{"type":"text","text":format!("{MARKER} {ESCAPED} {BEARER}")}],"structuredContent":{"echo":args,"marker":MARKER,"escaped":serde_json::to_string(ESCAPED).unwrap(),"diagnostics":[{"ruleId":"OPS-032"}]},"isError":true,"_meta":{names::key("outcome"):"unknown",names::key("reason"):"conflict",names::key("restriction"):restriction,names::key("invocation"):"invocation-from-server"}})).unwrap();
        Ok(result.into())
    }
}
async fn layer(State(w): State<Arc<Wire>>, req: Request, next: Next) -> Response {
    let authorized = req
        .headers()
        .get("authorization")
        .is_some_and(|h| h.as_bytes() == format!("Bearer {BEARER}").as_bytes());
    if req.method() != "POST" {
        return next.run(req).await;
    }
    w.authorized.lock().unwrap().push(authorized);
    if !authorized {
        return Response::builder().status(401).body(Body::empty()).unwrap();
    }
    let (parts, body) = req.into_parts();
    let body = to_bytes(body, 1024 * 1024).await.unwrap();
    let v: Value = serde_json::from_slice(&body).unwrap();
    w.posts.lock().unwrap().push(v.clone());
    if v["method"] == "tools/call"
        && let Some(ack) = &w.ack
    {
        w.ack_observed.lock().unwrap().push(ack.exists());
    }
    w.notify.notify_waiters();
    if v["method"] == "tools/call" {
        match v["params"]["arguments"]["action"].as_str() {
            Some("404") => return Response::builder().status(404).body(Body::empty()).unwrap(),
            Some("500") => return Response::builder().status(500).body(Body::empty()).unwrap(),
            Some("closed") => {
                return Response::builder()
                    .header("content-type", "text/event-stream")
                    .body(Body::empty())
                    .unwrap();
            }
            Some("hold") => w.release.notified().await,
            _ => (),
        }
    }
    next.run(Request::from_parts(parts, Body::from(body))).await
}
struct Running {
    url: String,
    wire: Arc<Wire>,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Running {
    fn drop(&mut self) {
        self.task.abort();
    }
}
async fn server(w: Wire) -> Running {
    let wire = Arc::new(w);
    let cloned = wire.clone();
    let service = StreamableHttpService::new(
        move || Ok(Server(cloned.clone())),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default()
            .with_legacy_session_mode(wire.legacy)
            .with_json_response(true),
    );
    let app = axum::Router::new()
        .nest_service("/mcp", service)
        .layer(from_fn_with_state(wire.clone(), layer));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/mcp", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    Running { url, wire, task }
}
struct Run {
    dir: TempDir,
    helper: String,
}
fn quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
impl Run {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
        let header = dir.path().join("header.json");
        fs::write(
            &header,
            json!({"Authorization":format!("Bearer {BEARER}")}).to_string(),
        )
        .unwrap();
        let helper = format!("/bin/cat {}", quote(header.to_str().unwrap()));
        Self { dir, helper }
    }
    fn script(&self, steps: Value) -> Value {
        json!({"scr":"v1","id":"cell-01","trace":"trace.jsonl","barrierDir":self.dir.path(),"redact":[MARKER,ESCAPED],"callTimeoutMs":1500,"barrierTimeoutMs":1500,"steps":steps})
    }
    fn args(&self, url: &str, thread: Option<&str>) -> Vec<String> {
        // Copied from agent-codex::arguments; the wrapper launch test guards this seam.
        let mut a: Vec<String> = ["exec", "--sandbox", "read-only", "--color", "never"]
            .map(str::to_owned)
            .into();
        if thread.is_some() {
            a.push("resume".into());
        }
        a.extend(
            [
                "--ignore-user-config",
                "--strict-config",
                "--ignore-rules",
                "--skip-git-repo-check",
                "--json",
                "--disable",
                "apps",
            ]
            .map(str::to_owned),
        );
        for feature in [
            "shell_tool",
            "unified_exec",
            "browser_use",
            "browser_use_external",
            "in_app_browser",
            "computer_use",
            "multi_agent",
            "multi_agent_v2",
            "multi_agent_v2_dynamic_tools",
            "hooks",
            "plugins",
            "view_image",
            "goals",
            "image_generation",
        ] {
            a.extend(["--disable".into(), feature.into()]);
        }
        for s in [
            format!("mcp_servers.engine.url={}", json!(url)),
            format!(
                "mcp_servers.engine.http_headers_helper={}",
                json!(self.helper)
            ),
            "mcp_servers.engine.startup_readiness=\"connection\"".into(),
            "mcp_servers.engine.required=true".into(),
            "mcp_servers.engine.default_tools_approval_mode=\"approve\"".into(),
            "web_search=\"disabled\"".into(),
            "project_doc_max_bytes=0".into(),
        ] {
            a.extend(["-c".into(), s]);
        }
        if let Some(t) = thread {
            a.push(t.into());
        }
        a.push("-".into());
        a
    }
    async fn spawn(&self, args: Vec<String>, script: &Value) -> Child {
        let mut child = Command::new(bin())
            .args(args)
            .env_clear()
            .env("HOME", self.dir.path())
            .env("SCR_SENTINEL", SENTINEL)
            .current_dir(self.dir.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .unwrap();
        let mut input = child.stdin.take().unwrap();
        input
            .write_all(script.to_string().as_bytes())
            .await
            .unwrap();
        input.shutdown().await.unwrap();
        child
    }
    async fn run(&self, url: &str, script: &Value) -> Output {
        let output = finish(self.spawn(self.args(url, None), script).await).await;
        for entry in fs::read_dir(self.dir.path()).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|s| s == "jsonl") {
                clean(&fs::read_to_string(path).unwrap());
            }
        }
        output
    }
    fn trace(&self) -> Vec<Value> {
        fs::read_to_string(self.dir.path().join("trace.jsonl"))
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect()
    }
}
async fn finish(child: Child) -> Output {
    timeout(Duration::from_secs(15), child.wait_with_output())
        .await
        .unwrap()
        .unwrap()
}
fn clean(text: &str) {
    for secret in [BEARER, MARKER, ESCAPED, SENTINEL] {
        assert!(!text.contains(secret));
        let escaped = serde_json::to_string(secret).unwrap();
        assert!(!text.contains(&escaped[1..escaped.len() - 1]));
    }
}
fn lines(out: &Output) -> Vec<Value> {
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    clean(&stdout);
    clean(&stderr);
    assert!(stderr.is_empty());
    let v: Vec<Value> = stdout
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert!(
        v.len() == 3
            && v[0]["type"] == "thread.started"
            && v[0].get("tools").is_none()
            && v[1]["type"] == "item.completed"
            && v[2]["type"] == "turn.completed"
    );
    v
}
fn sum(out: &Output) -> Value {
    let s: Value = serde_json::from_str(lines(out)[1]["item"]["text"].as_str().unwrap()).unwrap();

    s
}
fn discover() -> Value {
    json!({"step":"discover"})
}
fn call(mode: &str, action: &str) -> Value {
    let mut c = json!({"step":"call","tool":"effect.write.v1","mode":mode,"arguments":{"action":action,"entry":1,"marker":ESCAPED}});
    if mode == "keyed" {
        c["key"] = json!("repeat-key");
    }
    c
}
async fn wait_file(path: &Path) {
    timeout(Duration::from_secs(10), async {
        while !path.exists() {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
}
#[tokio::test]
async fn launch_first_resumed_and_rejected_forms() {
    let server = server(Wire::default()).await;
    let run = Run::new();
    let first = run.run(&server.url, &run.script(json!([discover()]))).await;
    let first_lines = lines(&first);
    let id = first_lines[0]["thread_id"].as_str().unwrap();
    assert!(id.starts_with("scr-"));
    let mut script = run.script(json!([discover()]));
    script["trace"] = json!("resumed.jsonl");
    let resumed = finish(run.spawn(run.args(&server.url, Some(id)), &script).await).await;
    assert!(lines(&resumed)[0]["thread_id"] == id);
    let before = server.wire.posts().len();
    let mut no_end = run.args(&server.url, None);
    no_end.pop();
    let mut bad_url = run.args(&server.url, None);
    let i = bad_url
        .iter()
        .position(|a| a.starts_with("mcp_servers.engine.url="))
        .unwrap();
    bad_url.drain(i - 1..=i);
    let mut bad_helper = run.args(&server.url, None);
    let i = bad_helper
        .iter()
        .position(|a| a.starts_with("mcp_servers.engine.http_headers_helper="))
        .unwrap();
    bad_helper.drain(i - 1..=i);
    for args in [
        vec!["--input-format".into(), "stream-json".into()],
        no_end,
        bad_url,
        bad_helper,
    ] {
        let out = finish(run.spawn(args, &script).await).await;
        assert!(out.status.code() == Some(2) && out.stdout.is_empty() && out.stderr.is_empty());
    }
    assert!(server.wire.posts().len() == before);
}
#[tokio::test]
async fn launch_option_values_are_not_resume_subcommands() {
    let server = server(Wire::default()).await;
    for model in ["harmless-model", "resume"] {
        let run = Run::new();
        let mut script = run.script(json!([discover()]));
        let mut args = run.args(&server.url, None);
        args.splice(5..5, ["--model".into(), model.into()]);
        let first = finish(run.spawn(args, &script).await).await;
        assert!(sum(&first)["status"] == "completed");
        let first_lines = lines(&first);
        let id = first_lines[0]["thread_id"].as_str().unwrap();
        assert!(id.starts_with("scr-"));
        script["trace"] = json!("resumed.jsonl");
        let mut args = run.args(&server.url, Some(id));
        args.splice(5..5, ["--model".into(), model.into()]);
        let resumed = finish(run.spawn(args, &script).await).await;
        assert!(sum(&resumed)["status"] == "completed");
        assert!(lines(&resumed)[0]["thread_id"] == id);
    }
    assert!(server.wire.count("tools/list") == 4);
}
#[tokio::test]
async fn authentication_refusals_run_no_request_or_forbidden_helper() {
    let server = server(Wire::default()).await;
    for helper in [
        "exit 7",
        "printf '%s' '{}'",
        "printf '%s' '{\"Authorization\":\"Bearer wrong\",\"extra\":1}'",
        "printf '%s' '{\"Authorization\":\"Bearer \"}'",
    ] {
        let mut run = Run::new();
        run.helper = helper.into();
        let out = run.run(&server.url, &run.script(json!([discover()]))).await;
        assert!(sum(&out)["cause"] == "token-unavailable");
        assert!(server.wire.posts().is_empty());
    }
    for url in [
        "http://192.0.2.1/mcp",
        "https://127.0.0.1/mcp",
        "http://localhost/mcp",
    ] {
        let mut run = Run::new();
        run.helper = format!(
            "/usr/bin/touch {}",
            quote(run.dir.path().join("helper-ran").to_str().unwrap())
        );
        let out = run.run(url, &run.script(json!([discover()]))).await;
        assert!(sum(&out)["cause"] == "url-refused");
        assert!(!run.dir.path().join("helper-ran").exists());
    }
}
#[tokio::test]
async fn metadata_discovery_summary_and_redaction_are_wire_checked() {
    let server = server(Wire {
        pages: true,
        ..Default::default()
    })
    .await;
    let run = Run::new();
    let out = run
        .run(
            &server.url,
            &run.script(json!([
                discover(),
                discover(),
                call("keyed", "ok"),
                call("fresh", "ok"),
                call("fresh", "ok"),
                call("unkeyed", "ok"),
                call("keyed", "error")
            ])),
        )
        .await;
    let summary = sum(&out);

    assert!(summary["status"] == "completed");
    assert!(server.wire.count("tools/list") == 4 && server.wire.count("tools/call") == 5);
    assert!(server.wire.authorized.lock().unwrap().iter().all(|b| *b));
    let calls: Vec<_> = server
        .wire
        .posts()
        .into_iter()
        .filter(|p| p["method"] == "tools/call")
        .collect();
    for (i, p) in calls.iter().enumerate() {
        let m = &p["params"]["_meta"];
        assert!(p["params"]["arguments"] == call("unkeyed", "ok")["arguments"] || i == 4);
        if i == 3 {
            assert!(m.get(names::key("idempotencyKey")).is_none());
            assert!(
                m["io.modelcontextprotocol/clientCapabilities"]["extensions"]
                    .get(names::EXTENSION)
                    .is_none()
            );
        } else {
            assert!(m[names::key("idempotencyKey")] == summary["steps"][i + 2]["key"]);
            let mut capabilities = m["io.modelcontextprotocol/clientCapabilities"].clone();
            assert!(capabilities["extensions"][names::EXTENSION]["revision"] == "v0");
            capabilities.as_object_mut().unwrap().remove("extensions");
            let base = server
                .wire
                .posts()
                .into_iter()
                .find(|p| p["method"] == "server/discover")
                .unwrap()["params"]["_meta"]["io.modelcontextprotocol/clientCapabilities"]
                .clone();
            assert!(base["experimental"]["scr-client"]["script"] == "v1");
            assert!(capabilities == base);
        }
    }
    let fresh1 = summary["steps"][3]["key"].as_str().unwrap();
    let fresh2 = summary["steps"][4]["key"].as_str().unwrap();
    assert!(fresh1 != fresh2 && fresh1.starts_with("scr-") && fresh2.starts_with("scr-"));
    assert!(
        uuid::Uuid::parse_str(&fresh1[4..])
            .unwrap()
            .get_version_num()
            == 4
    );
    let reply = &summary["steps"][2];
    assert!(
        reply["outcome"] == "unknown"
            && reply["reason"] == "conflict"
            && reply["invocation"] == "invocation-from-server"
            && reply["isError"] == true
            && reply["ruleIds"] == json!(["OPS-032"])
            && reply["restriction"] == json!({"requirement":"test#filter.v1"})
    );
    assert!(summary["steps"][6]["code"] == -32021 && summary["steps"][6]["reason"] == "invalid");
    let path = run.dir.path().join("trace.jsonl");
    assert!(fs::metadata(&path).unwrap().permissions().mode() & 0o777 == 0o600);
    let raw = fs::read_to_string(path).unwrap();
    clean(&raw);
    let trace = run.trace();
    for (i, line) in trace.iter().enumerate() {
        assert!(line["seq"] == i + 1);
        assert!(chrono::DateTime::parse_from_rfc3339(line["at"].as_str().unwrap()).is_ok());
        assert!(line["elapsedMs"].is_number());
    }
    assert!(
        trace
            .iter()
            .filter(|p| p["event"] == "request" && p["method"] == "tools/list")
            .count()
            == 4
    );
    assert!(
        trace[0]["event"] == "open"
            && trace[0]["server"]["protocolVersion"] == "2026-07-28"
            && trace.last().unwrap()["event"] == "end"
    );
}
#[tokio::test]
async fn discovery_failure_and_undiscovered_names_stop() {
    let server = server(Wire {
        fail_list_at: 2,
        ..Default::default()
    })
    .await;
    let run = Run::new();
    let out = run
        .run(
            &server.url,
            &run.script(json!([discover(), discover(), call("unkeyed", "ok")])),
        )
        .await;
    assert!(
        sum(&out)["cause"] == "discover-failed"
            && server.wire.count("tools/list") == 2
            && server.wire.count("tools/call") == 0
    );
    let run = Run::new();
    let mut c = call("unkeyed", "ok");
    c["tool"] = json!("not-listed.v1");
    let out = run
        .run(&server.url, &run.script(json!([discover(), c])))
        .await;
    assert!(sum(&out)["cause"] == "undiscovered-tool" && server.wire.count("tools/call") == 0);
}
#[tokio::test]
async fn exactly_once_across_http_and_rpc_failures() {
    for legacy in [false, true] {
        let server = server(Wire {
            legacy,
            ..Default::default()
        })
        .await;
        let run = Run::new();
        let out = run
            .run(
                &server.url,
                &run.script(json!([
                    discover(),
                    call("keyed", "500"),
                    call("keyed", "error"),
                    call("keyed", "404"),
                    call("unkeyed", "ok")
                ])),
            )
            .await;
        let summary = sum(&out);
        assert!(
            summary["status"] == "completed" && server.wire.count("tools/call") == 4,
            "legacy={legacy} status={} cause={} calls={} replies={:?}",
            summary["status"],
            summary["cause"],
            server.wire.count("tools/call"),
            summary["steps"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s["reply"].as_str())
                .collect::<Vec<_>>()
        );
        assert!(
            summary["steps"][1]["reply"] == "transport"
                && summary["steps"][2]["reply"] == "error"
                && summary["steps"][3]["reply"] == "transport"
        );
        assert!(
            run.trace()[0]["server"]["protocolVersion"]
                == if legacy { "2025-11-25" } else { "2026-07-28" }
        );
        let posts = server.wire.posts();
        let failed = posts
            .iter()
            .position(|p| p["params"]["arguments"]["action"] == "404")
            .unwrap();
        assert!(
            !posts[failed + 1..]
                .iter()
                .any(|p| p["method"] == "initialize" || p["method"] == "server/discover")
        );
        assert!(
            !posts
                .iter()
                .any(|p| p["method"] == "notifications/cancelled")
        );
        assert!(
            run.trace()
                .iter()
                .any(|e| e["transport"]["httpStatus"] == 500)
        );
    }
}
#[tokio::test]
async fn held_call_times_out_without_cancellation_or_later_call() {
    let server = server(Wire::default()).await;
    let run = Run::new();
    let mut script = run.script(json!([
        discover(),
        call("keyed", "hold"),
        call("unkeyed", "ok")
    ]));
    script["callTimeoutMs"] = json!(150);
    let out = run.run(&server.url, &script).await;
    let summary = sum(&out);
    assert!(summary["status"] == "stopped" && summary["cause"] == "call-timeout");
    assert!(
        summary["steps"][1]["reply"] == "timeout"
            && summary["steps"].as_array().unwrap().len() == 2
    );
    assert!(server.wire.count("tools/call") == 1);
    assert!(server.wire.count("notifications/cancelled") == 0);
    server.wire.release.notify_waiters();
}
#[tokio::test]
async fn closed_response_is_not_retried_or_cancelled() {
    let server = server(Wire::default()).await;
    let run = Run::new();
    let mut script = run.script(json!([
        discover(),
        call("keyed", "closed"),
        call("unkeyed", "ok")
    ]));
    script["callTimeoutMs"] = json!(150);
    let out = run.run(&server.url, &script).await;
    let summary = sum(&out);
    let expected = if summary["cause"] == "call-timeout" {
        assert!(summary["status"] == "stopped" && summary["steps"][1]["reply"] == "timeout");
        1
    } else {
        assert!(summary["status"] == "completed" && summary["steps"][1]["reply"] == "transport");
        assert!(summary["steps"][2]["reply"] == "result");
        2
    };
    assert!(server.wire.count("tools/call") == expected);
    assert!(server.wire.count("notifications/cancelled") == 0);
}
#[tokio::test]
async fn acknowledged_barrier_orders_repeat_and_unreleased_barrier_stops() {
    let run = Run::new();
    let server = server(Wire {
        ack: Some(run.dir.path().join("first-held.acknowledged")),
        ..Default::default()
    })
    .await;
    let child=run.spawn(run.args(&server.url,None),&run.script(json!([discover(),call("keyed","ok"),{"step":"barrier","name":"first-held"},call("keyed","ok")]))).await;
    wait_file(&run.dir.path().join("first-held.reached")).await;
    assert!(server.wire.count("tools/call") == 1);
    fs::write(run.dir.path().join("first-held.release"), b"").unwrap();
    server.wire.wait_count("tools/call", 2).await;
    let out = finish(child).await;
    let summary = sum(&out);
    assert!(
        summary["status"] == "completed"
            && summary["steps"][2]["acknowledged"] == true
            && summary["steps"][1]["reply"] == "result"
            && summary["steps"][3]["reply"] == "result"
    );
    assert!(*server.wire.ack_observed.lock().unwrap() == vec![false, true]);
    for suffix in ["reached", "acknowledged"] {
        assert!(
            fs::metadata(run.dir.path().join(format!("first-held.{suffix}")))
                .unwrap()
                .permissions()
                .mode()
                & 0o777
                == 0o600
        );
    }
    let trace = run.trace();
    let reached = trace
        .iter()
        .position(|e| e["event"] == "barrier-reached")
        .unwrap();
    let released = trace
        .iter()
        .position(|e| e["event"] == "barrier-released")
        .unwrap();
    assert!(reached < released && trace[released + 1]["event"] == "request");
    let run = Run::new();
    let mut script =
        run.script(json!([discover(),{"step":"barrier","name":"held"},call("keyed","ok")]));
    script["barrierTimeoutMs"] = json!(50);
    let out = run.run(&server.url, &script).await;
    assert!(
        sum(&out)["cause"] == "barrier-timeout"
            && !run.dir.path().join("held.acknowledged").exists()
    );
    assert!(server.wire.count("tools/call") == 2);
}
#[tokio::test]
async fn invalid_script_has_no_files_helper_or_posts() {
    let server = server(Wire::default()).await;
    for (case, problem) in [
        ("unknown", "script.misspelledTimeout"),
        ("unknown-discover", "steps[0].unexpected"),
        ("unknown-call", "steps[0].unexpected"),
        ("unknown-barrier", "steps[0].unexpected"),
        ("unknown-marker", "script.[redacted]"),
        ("unknown-escaped-marker", "script.[redacted]"),
        ("unknown-json-escaped-marker", "steps[0].[redacted]"),
        ("version", "scr"),
        ("steps", "steps"),
        ("step", "steps.step"),
        ("trace-existing", "trace.exists"),
        ("trace-slash", "trace"),
        ("trace-dot", "trace"),
        ("trace-reached", "trace.barrier-collision"),
        ("trace-release", "trace.barrier-collision"),
        ("trace-acknowledged", "trace.barrier-collision"),
        ("trace-alias", "trace.barrier-collision"),
        ("trace-mixed-reached", "trace.barrier-collision"),
        ("trace-mixed-release", "trace.barrier-collision"),
        ("trace-mixed-acknowledged", "trace.barrier-collision"),
        ("barrier-existing", "steps.barrier.files"),
        ("barrier-mode", "barrierDir"),
        ("barrier-duplicate", "steps.barrier.name"),
        ("barrier-name", "steps.barrier.name"),
        ("timeout", "callTimeoutMs"),
        ("key", "steps.call.key"),
        ("arguments", "steps.call.arguments"),
        ("marker", "redact"),
    ] {
        let mut run = Run::new();
        run.helper = format!(
            "/usr/bin/touch {}",
            quote(run.dir.path().join("helper-ran").to_str().unwrap())
        );
        let mut s = run.script(json!([discover()]));
        match case {
            "unknown" => s["misspelledTimeout"] = json!("unreported-value"),
            "unknown-discover" => s["steps"][0]["unexpected"] = json!("unreported-value"),
            "unknown-call" => {
                s["steps"] = json!([call("unkeyed", "ok")]);
                s["steps"][0]["unexpected"] = json!("unreported-value");
            }
            "unknown-barrier" => {
                s["steps"] =
                    json!([{"step":"barrier","name":"held","unexpected":"unreported-value"}]);
            }
            "unknown-marker" => s[MARKER] = json!("unreported-value"),
            "unknown-escaped-marker" => s[ESCAPED] = json!("unreported-value"),
            "unknown-json-escaped-marker" => {
                let escaped = serde_json::to_string(ESCAPED).unwrap();
                s["steps"][0][&escaped[1..escaped.len() - 1]] = json!("unreported-value");
            }
            "version" => s["scr"] = json!("v2"),
            "steps" => s["steps"] = json!([]),
            "step" => s["steps"] = json!([{"step":"other"}]),
            "trace-existing" => fs::write(run.dir.path().join("trace.jsonl"), b"original").unwrap(),
            "trace-slash" => s["trace"] = json!("nested/trace"),
            "trace-dot" => s["trace"] = json!(".trace"),
            "trace-reached" | "trace-release" | "trace-acknowledged" | "trace-alias" => {
                s["steps"] = json!([discover(),{"step":"barrier","name":"held"}]);
                let suffix = case.strip_prefix("trace-").unwrap();
                s["trace"] = json!(format!(
                    "held.{}",
                    if suffix == "alias" { "release" } else { suffix }
                ));
                if suffix == "alias" {
                    fs::create_dir(run.dir.path().join("alias")).unwrap();
                    s["barrierDir"] = json!(run.dir.path().join("alias/.."));
                }
            }
            "trace-mixed-reached" | "trace-mixed-release" | "trace-mixed-acknowledged" => {
                s["steps"] = json!([discover(),{"step":"barrier","name":"held"}]);
                let suffix = case.strip_prefix("trace-mixed-").unwrap();
                s["trace"] = json!(format!("Held.{}", suffix.to_ascii_uppercase()));
                // A different directory must not hide a filename collision, even on Linux.
                let barriers = run.dir.path().join("barriers");
                fs::create_dir(&barriers).unwrap();
                fs::set_permissions(&barriers, fs::Permissions::from_mode(0o700)).unwrap();
                s["barrierDir"] = json!(barriers);
            }
            "barrier-existing" => {
                s["steps"] = json!([{"step":"barrier","name":"held"}]);
                fs::write(run.dir.path().join("held.release"), b"").unwrap();
            }
            "barrier-mode" => {
                s["steps"] = json!([{"step":"barrier","name":"held"}]);
                fs::set_permissions(run.dir.path(), fs::Permissions::from_mode(0o720)).unwrap();
            }
            "barrier-duplicate" => {
                s["steps"] =
                    json!([{"step":"barrier","name":"held"},{"step":"barrier","name":"held"}])
            }
            "barrier-name" => s["steps"] = json!([{"step":"barrier","name":"-held"}]),
            "timeout" => s["callTimeoutMs"] = json!(0),
            "key" => {
                s["steps"] = json!([call("fresh", "ok")]);
                s["steps"][0]["key"] = json!("extra");
            }
            "arguments" => {
                s["steps"] = json!([call("keyed", "ok")]);
                s["steps"][0]["arguments"] = json!([]);
            }
            "marker" => s["redact"] = json!(["short"]),
            _ => unreachable!(),
        }
        let files = || {
            fs::read_dir(run.dir.path())
                .unwrap()
                .map(|e| e.unwrap().path())
                .collect::<std::collections::BTreeSet<_>>()
        };
        let before = files();
        let out = run.run(&server.url, &s).await;
        let summary = sum(&out);
        assert!(summary["status"] == "invalid" && summary["problem"] == problem);
        assert!(before == files());
        assert!(!run.dir.path().join("helper-ran").exists());
        assert!(server.wire.posts().is_empty());
        if case == "trace-existing" {
            assert!(fs::read(run.dir.path().join("trace.jsonl")).unwrap() == b"original");
        } else {
            assert!(!run.dir.path().join("trace.jsonl").exists());
        }
    }
}
fn redaction_report_scripts(run: &Run) -> Vec<(Value, &'static str)> {
    let mut cases = Vec::new();
    for marker in [
        "type", "item", "text", "scr", "id", "trace", "status", "invalid", "cause", "problem",
        "steps", "redact",
    ] {
        let mut script = run.script(json!([discover()]));
        script["redact"] = json!([MARKER, ESCAPED, marker]);
        // Short markers invalidate the script but must not corrupt its report.
        cases.push((script, "invalid"));
    }
    for marker in [
        "thread.started",
        "thread_id",
        "item.completed",
        "agent_message",
        "turn.completed",
        "null,\"cause",
        // This marker also spans adjacent fields in a serialized summary.
        "null,\"id",
    ] {
        for valid in [false, true] {
            let mut script = run.script(json!([discover(), call("unkeyed", "ok")]));
            script["redact"] = json!([MARKER, ESCAPED, marker]);
            if !valid {
                script["scr"] = json!("v2");
            }
            cases.push((script, if valid { "completed" } else { "invalid" }));
        }
    }
    for (i, (script, _)) in cases.iter_mut().enumerate() {
        script["trace"] = json!(format!("report-{i}.jsonl"));
    }
    cases
}

#[tokio::test]
async fn redaction_markers_preserve_protocol_and_parseable_reports() {
    let server = server(Wire::default()).await;
    let run = Run::new();
    for (script, status) in redaction_report_scripts(&run) {
        let before = server.wire.posts().len();
        let out = finish(
            run.spawn(run.args(&server.url, Some("turn.completed")), &script)
                .await,
        )
        .await;
        let events = lines(&out);
        assert!(events[0]["thread_id"] == "turn.completed");
        let summary = sum(&out);
        assert!(summary["status"] == status);
        let trace = run.dir.path().join(script["trace"].as_str().unwrap());
        if status == "invalid" {
            assert!(!trace.exists());
            assert!(server.wire.posts().len() == before);
        } else {
            let raw = fs::read_to_string(trace).unwrap();
            clean(&raw);
            for line in raw.lines() {
                assert!(serde_json::from_str::<Value>(line).is_ok());
            }
        }
    }
}

#[tokio::test]
async fn numeric_markers_are_redacted_in_trace_and_copied_summary_values() {
    let server = server(Wire::default()).await;
    for restriction in [
        json!(9123456789_u64),
        json!({"nested":[9123456789_u64,17],"9123456789":"kept"}),
    ] {
        let run = Run::new();
        let mut step = call("unkeyed", "ok");
        step["arguments"]["restriction"] = restriction.clone();
        let mut script = run.script(json!([discover(), step]));
        script["redact"] = json!([MARKER, ESCAPED, "12345678"]);
        let out = run.run(&server.url, &script).await;
        assert!(!String::from_utf8_lossy(&out.stdout).contains("12345678"));
        let summary = sum(&out);
        let expected = if restriction.is_number() {
            json!("[redacted]")
        } else {
            json!({"nested":["[redacted]",17],"9[redacted]9":"kept"})
        };
        assert!(summary["status"] == "completed");
        assert!(summary["steps"][1]["restriction"] == expected);
        let raw = fs::read_to_string(run.dir.path().join("trace.jsonl")).unwrap();
        assert!(!raw.contains("12345678"));
        let trace = run.trace();
        let request = trace.iter().find(|v| v["method"] == "tools/call").unwrap();
        assert!(request["params"]["arguments"]["restriction"] == expected);
        let reply = trace
            .iter()
            .find(|v| v["event"] == "reply" && v["step"] == 1)
            .unwrap();
        assert!(reply["result"]["_meta"][names::key("restriction")] == expected);
        // Redaction affects records only: the server receives the original numeric value.
        let posts = server.wire.posts();
        let call = posts
            .iter()
            .rev()
            .find(|v| v["method"] == "tools/call")
            .unwrap();
        assert!(call["params"]["arguments"]["restriction"] == restriction);
    }
}

#[tokio::test]
async fn wrapper_launch_first_and_resumed_turns() {
    let Some(wrapper) = std::env::var_os("SCR_AGENT_CODEX") else {
        println!(
            "skipped wrapper launch: SCR_AGENT_CODEX is unset; build agent-codex and supply its absolute binary path"
        );
        return;
    };
    assert!(
        Path::new(&wrapper).is_file(),
        "SCR_AGENT_CODEX must name a built binary"
    );
    let server = server(Wire::default()).await;
    let run = Run::new();
    let identity = run.dir.path().join("identity");
    fs::create_dir(&identity).unwrap();
    fs::write(identity.join("token"), BEARER).unwrap();
    let work = run.dir.path().join("work");
    fs::create_dir(&work).unwrap();
    let mut child = Command::new(wrapper)
        .args(["--client-program", bin()])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("HOME", run.dir.path())
        .env("CONDUIT_URL", &server.url)
        .env("CONDUIT_IDENTITY_DIR", &identity)
        .env("AGENT_ACCOUNT_SERVICES", "false")
        .env("AGENT_PEER_MESSAGING", "false")
        .env("SCR_SENTINEL", SENTINEL)
        .current_dir(&work)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let stderr = child.stderr.take().unwrap();
    let err_task = tokio::spawn(async move {
        let mut stderr = stderr;
        let mut text = String::new();
        stderr.read_to_string(&mut text).await.unwrap();
        text
    });
    let service = ()
        .serve_with_lifecycle(
            (child.stdout.take().unwrap(), child.stdin.take().unwrap()),
            ClientLifecycleMode::Auto {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
                legacy_version: Some(ProtocolVersion::V_2025_11_25),
            },
        )
        .await
        .unwrap();
    for i in 0..2 {
        let name = format!("turn-{i}.jsonl");
        let barrier = format!("held-{i}");
        let mut s =
            run.script(json!([discover(),{"step":"barrier","name":barrier},call("keyed","ok")]));
        s["trace"] = json!(name);
        let request:ClientRequest=serde_json::from_value(json!({"method":"tools/call","params":{"name":"turn","arguments":{"message":s.to_string()}}})).unwrap();
        let handle = service
            .send_request_with_option(request, rmcp::service::PeerRequestOptions::no_options())
            .await
            .unwrap();
        wait_file(&run.dir.path().join(format!("{barrier}.reached"))).await;
        fs::write(run.dir.path().join(format!("{barrier}.release")), b"").unwrap();
        let response = timeout(Duration::from_secs(15), handle.await_response())
            .await
            .unwrap()
            .unwrap();
        let value = serde_json::to_value(response).unwrap();
        clean(&value.to_string());
        assert!(
            value["isError"] == false
                && value["structuredContent"]["clientTools"]["status"] == "not_reported"
        );
        let summary: Value =
            serde_json::from_str(value["content"][0]["text"].as_str().unwrap()).unwrap();
        assert!(
            summary["status"] == "completed"
                && value["structuredContent"]["text"] == value["content"][0]["text"]
        );
        assert!(fs::metadata(work.join(name)).unwrap().permissions().mode() & 0o777 == 0o600);
        assert!(
            run.dir
                .path()
                .join(format!("{barrier}.acknowledged"))
                .exists()
        );
    }
    assert!(server.wire.count("tools/call") == 2);
    for (script, status) in redaction_report_scripts(&run) {
        let before = server.wire.posts().len();
        let request: ClientRequest = serde_json::from_value(json!({"method":"tools/call","params":{"name":"turn","arguments":{"message":script.to_string()}}})).unwrap();
        let handle = service
            .send_request_with_option(request, rmcp::service::PeerRequestOptions::no_options())
            .await
            .unwrap();
        let response = timeout(Duration::from_secs(15), handle.await_response())
            .await
            .unwrap()
            .unwrap();
        let value = serde_json::to_value(response).unwrap();
        clean(&value.to_string());
        assert!(value["isError"] == false);
        assert!(value["structuredContent"]["clientTools"]["status"] == "not_reported");
        assert!(value["structuredContent"]["text"] == value["content"][0]["text"]);
        let summary: Value =
            serde_json::from_str(value["content"][0]["text"].as_str().unwrap()).unwrap();
        assert!(summary["status"] == status);
        if status == "invalid" {
            assert!(server.wire.posts().len() == before);
            assert!(!work.join(script["trace"].as_str().unwrap()).exists());
        }
    }
    service.cancel().await.unwrap();
    timeout(Duration::from_secs(10), child.wait())
        .await
        .unwrap()
        .unwrap();
    clean(&err_task.await.unwrap());
    assert!(server.wire.authorized.lock().unwrap().iter().all(|b| *b));
    println!("wrapper launch ran: first, resumed and redaction regression turns completed");
}

#[tokio::test]
async fn input_required_is_recorded_without_automatic_repeat() {
    let server = server(Wire::default()).await;
    let run = Run::new();
    let out = run
        .run(
            &server.url,
            &run.script(json!([
                discover(),
                call("keyed", "input"),
                call("unkeyed", "ok")
            ])),
        )
        .await;
    let summary = sum(&out);
    assert!(summary["status"] == "completed" && server.wire.count("tools/call") == 2);
    assert!(
        run.trace()
            .iter()
            .any(|r| r["result"]["resultType"] == "input_required")
    );
}

#[tokio::test]
async fn discovery_is_bounded_to_sixteen_pages() {
    let server = server(Wire {
        endless: true,
        ..Default::default()
    })
    .await;
    let run = Run::new();
    let out = run
        .run(
            &server.url,
            &run.script(json!([discover(), call("keyed", "ok")])),
        )
        .await;
    assert!(
        sum(&out)["cause"] == "discover-failed"
            && server.wire.count("tools/list") == 16
            && server.wire.count("tools/call") == 0
    );
}
