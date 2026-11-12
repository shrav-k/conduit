use chrono::{Duration as ChronoDuration, Utc};
use rmcp::{
    RoleClient, ServiceExt,
    service::{RunningService, ServiceError},
};
use s2_linear_stub::{IssueFixture, Stub, StubConfig, SyntheticToken};
use scenario_s2_linear_adapter::*;
use serde_json::{Value, json};
use std::{
    pin::Pin,
    process::Stdio,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt, ReadBuf},
    process::{Child, Command},
};

fn clean(bytes: &[u8], secrets: &[String]) {
    let text = String::from_utf8_lossy(bytes);
    for secret in secrets {
        let escaped = serde_json::to_string(secret).unwrap();
        assert!(!text.contains(secret), "credential in captured output");
        assert!(
            !text.contains(&escaped[1..escaped.len() - 1]),
            "escaped credential in captured output"
        );
    }
}
struct Tap<R>(R, Arc<Mutex<Vec<u8>>>);
impl<R: AsyncRead + Unpin> AsyncRead for Tap<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        let before = buf.filled().len();
        let result = Pin::new(&mut this.0).poll_read(cx, buf);
        this.1
            .lock()
            .unwrap()
            .extend_from_slice(&buf.filled()[before..]);
        result
    }
}
struct Session {
    client: RunningService<RoleClient, ()>,
    child: Child,
    stdout: Arc<Mutex<Vec<u8>>>,
    stderr: tokio::task::JoinHandle<Vec<u8>>,
    secrets: Vec<String>,
}
fn command(url: &str, key: &str) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_scenario-s2-linear-adapter"));
    command
        .env_clear()
        .env(BASE_VARIABLE, url)
        .env(KEY_VARIABLE, key)
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}
impl Session {
    async fn start(url: &str, key: &str, secrets: Vec<String>) -> Self {
        let mut child = command(url, key).spawn().unwrap();
        let stdout = Arc::new(Mutex::new(Vec::new()));
        let transport = (
            Tap(child.stdout.take().unwrap(), stdout.clone()),
            child.stdin.take().unwrap(),
        );
        let mut errors = child.stderr.take().unwrap();
        let stderr = tokio::spawn(async move {
            let mut bytes = Vec::new();
            errors.read_to_end(&mut bytes).await.unwrap();
            bytes
        });
        let client = tokio::time::timeout(Duration::from_secs(10), ().serve(transport))
            .await
            .unwrap()
            .unwrap();
        Self {
            client,
            child,
            stdout,
            stderr,
            secrets,
        }
    }
    async fn call(&self, request: Value) -> Value {
        let params = serde_json::from_value(request).unwrap();
        let answer = tokio::time::timeout(Duration::from_secs(10), self.client.call_tool(params))
            .await
            .unwrap()
            .unwrap();
        let answer = serde_json::to_value(answer).unwrap();
        clean(answer.to_string().as_bytes(), &self.secrets);
        answer
    }
    async fn finish(mut self) {
        self.client.cancel().await.unwrap();
        let status = tokio::time::timeout(Duration::from_secs(5), self.child.wait())
            .await
            .unwrap()
            .unwrap();
        assert!(status.success());
        clean(&self.stdout.lock().unwrap(), &self.secrets);
        clean(&self.stderr.await.unwrap(), &self.secrets);
    }
}
struct Fixture {
    stub: Stub,
    key: SyntheticToken,
    control: SyntheticToken,
    http: reqwest::Client,
}
impl Fixture {
    async fn new(issues: Vec<IssueFixture>) -> Self {
        let config = StubConfig::new(issues);
        let key = config.api_key.clone();
        let control = config.control_token.clone();
        let stub = Stub::start(config).await.unwrap();
        let http = reqwest::Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(8))
            .build()
            .unwrap();
        Self {
            stub,
            key,
            control,
            http,
        }
    }
    async fn simple() -> Self {
        let mut one = IssueFixture::new("SYN-1", "Mention SYN-9 stays as content");
        one.description = Some("Text mentions SYN-9 unchanged".into());
        Self::new(vec![
            one,
            IssueFixture::new("SYN-2", "Second issue"),
            IssueFixture::new("SYN-9", "Hidden issue"),
        ])
        .await
    }
    fn secrets(&self) -> Vec<String> {
        vec![
            self.key.expose().to_owned(),
            self.control.expose().to_owned(),
        ]
    }
    async fn session(&self) -> Session {
        Session::start(&self.stub.graphql_url(), self.key.expose(), self.secrets()).await
    }
    async fn control(&self, route: &str, body: Value) -> Value {
        let response = self
            .http
            .post(format!("{}/{}", self.stub.control_url(), route))
            .header("Authorization", self.control.expose())
            .json(&body)
            .send()
            .await
            .unwrap();
        assert!(response.status().is_success(), "stub control failed");
        response.json().await.unwrap()
    }
    async fn count(&self) -> u64 {
        self.http
            .get(format!("{}/count", self.stub.control_url()))
            .header("Authorization", self.control.expose())
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap()["api_requests"]
            .as_u64()
            .unwrap()
    }
    fn request(&self, operation: &str, args: Value, filters: Value) -> Value {
        request(&self.stub.graphql_url(), operation, args, filters)
    }
}
fn key(name: &str) -> String {
    format!("{PREFIX}{name}")
}
fn scope(issues: &[&str], deadline: &str) -> Value {
    json!({"requirement":format!("{CONTRACTS}#issue-scope.v1"),"arguments":{"issues":issues,"deadline":deadline}})
}
fn active(issues: &[&str]) -> Value {
    scope(issues, "2099-01-01T00:00:00Z")
}
fn request(url: &str, name: &str, args: Value, filters: Value) -> Value {
    json!({"name":name,"arguments":args,"_meta":{
        key("instance"):"resource-synthetic",key("operation"):format!("{CONTRACTS}#{name}"),
        key("caller"):"caller-synthetic",key("invocation"):"invocation-synthetic",
        key("revision"):"r1",key("conduit"):"conduit-synthetic",key("idempotencyKey"):"replay-synthetic",
        key("filters"):filters,key("config"):{"baseUrl":url,"apiKey":{"kind":"Resource","id":"secret-synthetic"}},
        "io.modelcontextprotocol/protocolVersion":"2026-07-28",
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}}
    }})
}
fn ending(answer: &Value, outcome: &str) {
    assert!(answer["_meta"][key("outcome")] == outcome);
    assert!(answer["_meta"].get(key("reason")).is_none());
    assert!(answer["isError"] == (outcome != "succeeded"));
    if outcome != "succeeded" {
        assert!(answer.get("structuredContent").is_none());
    }
}
fn success(answer: &Value, op: &str) -> Value {
    ending(answer, "succeeded");
    assert!(valid(
        &SCHEMAS[op]["outputSchema"],
        &answer["structuredContent"]
    ));
    answer["structuredContent"].clone()
}
fn denied(answer: &Value, reason: &str, filter: &Value) {
    assert!(answer["isError"] == true);
    assert!(answer["_meta"][key("reason")] == reason);
    assert!(answer["_meta"][key("restriction")] == *filter);
    assert!(answer["_meta"].get(key("outcome")).is_none());
    assert!(answer.get("structuredContent").is_none());
    assert!(answer.get("issue").is_none());
}

#[tokio::test]
async fn discovery_extension_and_unknown_tools() {
    let f = Fixture::simple().await;
    let s = f.session().await;
    let discovery = s
        .client
        .discover(
            serde_json::from_value(json!({
                "io.modelcontextprotocol/protocolVersion":"2026-07-28",
                "io.modelcontextprotocol/clientCapabilities":{}
            }))
            .unwrap(),
        )
        .await
        .unwrap();
    let discovery = serde_json::to_value(discovery).unwrap();
    assert!(discovery["capabilities"]["extensions"][EXTENSION]["revision"] == "v0");
    let tools = s.client.list_all_tools().await.unwrap();
    let mut names: Vec<_> = tools.iter().map(|t| t.name.as_ref()).collect();
    names.sort();
    assert!(names == ["list-issues.v1", "read-issue.v1"]);
    for tool in tools {
        let wire = serde_json::to_value(tool).unwrap();
        let name = wire["name"].as_str().unwrap();
        assert!(wire["inputSchema"] == SCHEMAS[name]["inputSchema"]);
        assert!(wire["outputSchema"] == SCHEMAS[name]["outputSchema"]);
    }
    for op in OPERATIONS {
        for extension in [
            json!({}),
            json!({"extensions":{EXTENSION:{"revision":"v9"}}}),
        ] {
            let mut req = f.request(op, json!({}), json!([]));
            req["_meta"]["io.modelcontextprotocol/clientCapabilities"] = extension;
            let error = s
                .client
                .call_tool(serde_json::from_value(req).unwrap())
                .await
                .err()
                .unwrap();
            let ServiceError::McpError(error) = error else {
                panic!("expected capability error")
            };
            assert!(error.code.0 == -32021);
            assert!(
                error.data.unwrap()["requiredCapabilities"]["extensions"][EXTENSION]["revision"]
                    == "v0"
            );
        }
    }
    for op in ["update-issue.v1", "unknown-tool"] {
        let error = s
            .client
            .call_tool(serde_json::from_value(f.request(op, json!({}), json!([]))).unwrap())
            .await
            .err()
            .unwrap();
        let ServiceError::McpError(error) = error else {
            panic!("expected protocol error")
        };
        assert!(error.code.0 == -32602);
    }
    assert!(f.count().await == 0);
    s.finish().await;
    f.stub.shutdown().await.unwrap();
}
#[tokio::test]
async fn read_projection_content_scope_and_missing() {
    let f = Fixture::simple().await;
    let s = f.session().await;
    let filter = active(&["SYN-1", "SYN-404"]);
    let mut req = f.request("read-issue.v1", json!({"issue":"SYN-1"}), json!([filter]));
    let answer = s.call(req.clone()).await;
    let output = success(&answer, "read-issue.v1");
    assert!(
        output["issue"]
            == json!({"identifier":"SYN-1","title":"Mention SYN-9 stays as content",
        "description":"Text mentions SYN-9 unchanged","state":"Todo","updatedAt":"2026-01-01T00:00:00+00:00"})
    );
    assert!(f.count().await == 1);
    for id in ["SYN-9", "SYN-405"] {
        req["arguments"]["issue"] = json!(id);
        denied(&s.call(req.clone()).await, "refused", &filter);
        assert!(f.count().await == 1);
    }
    req["arguments"]["issue"] = json!("SYN-404");
    ending(&s.call(req).await, "failed");
    assert!(f.count().await == 2);
    s.finish().await;
    f.stub.shutdown().await.unwrap();
}
#[tokio::test]
async fn list_pages_scope_intersection_order_missing_and_integral_float() {
    let f = Fixture::new(
        (1..=115)
            .map(|i| IssueFixture::new(format!("SYN-{i}"), "Synthetic page item"))
            .collect(),
    )
    .await;
    let s = f.session().await;
    let a = active(&["SYN-99", "SYN-98", "SYN-404"]);
    let b = active(&["SYN-99", "SYN-404"]);
    for limit in [json!(1), json!(1.0), serde_json::from_str("1e0").unwrap()] {
        let req = f.request("list-issues.v1", json!({"limit":limit}), json!([a, a, b]));
        let output = success(&s.call(req).await, "list-issues.v1");
        assert!(output["issues"].as_array().unwrap().len() == 1);
        assert!(output["issues"][0]["identifier"] == "SYN-99");
    }
    assert!(f.count().await == 6);
    let output = success(
        &s.call(f.request(
            "list-issues.v1",
            json!({"limit":10}),
            json!([active(&["SYN-99", "SYN-98", "SYN-404"])]),
        ))
        .await,
        "list-issues.v1",
    );
    assert!(output["issues"].as_array().unwrap().len() == 2);
    assert!(output["issues"][0]["identifier"] == "SYN-98");
    let output = success(
        &s.call(f.request(
            "list-issues.v1",
            json!({"limit":1}),
            json!([active(&["SYN-404"])]),
        ))
        .await,
        "list-issues.v1",
    );
    assert!(output == json!({"issues":[]}));
    let output = success(
        &s.call(f.request("list-issues.v1", json!({"limit":200}), json!([])))
            .await,
        "list-issues.v1",
    );
    assert!(output["issues"].as_array().unwrap().len() == 115);
    s.finish().await;
    f.stub.shutdown().await.unwrap();
}
#[tokio::test]
async fn test_clock_expiry_repeats_offsets_and_changed_filters() {
    let f = Fixture::simple().await;
    let s = f.session().await;
    let filter = scope(&["SYN-1"], "2030-01-01T01:00:00+01:00");
    let mut read = f.request("read-issue.v1", json!({"issue":"SYN-1"}), json!([filter]));
    read["_meta"][key("config")]["testClock"] = json!("2029-12-31T23:59:59Z");
    success(&s.call(read.clone()).await, "read-issue.v1");
    let changed = active(&["SYN-2"]);
    let mut second = read.clone();
    second["_meta"][key("filters")] = json!([changed]);
    denied(&s.call(second).await, "refused", &changed);
    read["_meta"][key("config")]["testClock"] = json!("2030-01-01T00:00:00Z");
    denied(&s.call(read.clone()).await, "refused", &filter);
    read["_meta"][key("config")]["testClock"] = json!("2030-01-01T00:00:01Z");
    denied(&s.call(read).await, "refused", &filter);
    let mut list = f.request("list-issues.v1", json!({"limit":2}), json!([filter]));
    list["_meta"][key("config")]["testClock"] = json!("2030-01-01T00:00:00Z");
    denied(&s.call(list).await, "refused", &filter);
    assert!(f.count().await == 1);
    s.finish().await;
    f.stub.shutdown().await.unwrap();
}
#[tokio::test]
async fn fractional_deadlines_preserve_precision_beyond_nanoseconds() {
    let f = Fixture::simple().await;
    let s = f.session().await;
    let mut requests = 0;
    for (clock, deadline, allowed) in [
        (
            "2030-01-01T00:00:00.0000000000Z",
            "2030-01-01T00:00:00.0000000001Z",
            true,
        ),
        (
            "2030-01-01T00:00:00.00000000019Z",
            "2030-01-01T00:00:00.0000000002Z",
            true,
        ),
        (
            "2030-01-01T01:00:00.1+01:00",
            "2030-01-01T00:00:00.1000000000000000000000000000000000000001Z",
            true,
        ),
        (
            "2030-01-01T00:00:00.0000000002Z",
            "2030-01-01T00:00:00.0000000001Z",
            false,
        ),
        (
            "2030-01-01T01:00:00.0000000001+01:00",
            "2030-01-01T00:00:00.000000000100Z",
            false,
        ),
        (
            "2030-01-01T00:00:00Z",
            "2030-01-01T00:00:00.0000000000Z",
            false,
        ),
    ] {
        let filter = scope(&["SYN-1"], deadline);
        for op in OPERATIONS {
            let args = if op == "read-issue.v1" {
                json!({"issue":"SYN-1"})
            } else {
                json!({"limit":1})
            };
            let mut req = f.request(op, args, json!([filter]));
            req["_meta"][key("config")]["testClock"] = json!(clock);
            let answer = s.call(req).await;
            if allowed {
                success(&answer, op);
                requests += 1;
            } else {
                denied(&answer, "refused", &filter);
            }
            assert!(f.count().await == requests);
        }
    }
    s.finish().await;
    f.stub.shutdown().await.unwrap();
}
#[tokio::test]
async fn service_authorization_is_distinct_and_rotation_requires_replacement() {
    let f = Fixture::simple().await;
    let old = f.session().await;
    let req = f.request(
        "read-issue.v1",
        json!({"issue":"SYN-1"}),
        json!([active(&["SYN-1"])]),
    );
    success(&old.call(req.clone()).await, "read-issue.v1");
    f.control(
        "service-authorization",
        json!({"key":f.key.expose(),"issues":["SYN-2"]}),
    )
    .await;
    ending(&old.call(req.clone()).await, "failed");
    f.control(
        "service-authorization",
        json!({"key":f.key.expose(),"issues":["SYN-1"]}),
    )
    .await;
    let new_key = SyntheticToken::generate();
    f.control("rotate-key", json!({"key":new_key.expose()}))
        .await;
    let mut secrets = f.secrets();
    secrets.push(new_key.expose().to_owned());
    let new = Session::start(&f.stub.graphql_url(), new_key.expose(), secrets).await;
    success(&new.call(req.clone()).await, "read-issue.v1");
    ending(&old.call(req).await, "failed");
    ending(
        &old.call(f.request("list-issues.v1", json!({"limit":1}), json!([])))
            .await,
        "failed",
    );
    old.finish().await;
    new.finish().await;
    f.stub.shutdown().await.unwrap();
}
#[tokio::test]
async fn real_clock_judged_once_barrier_passes_deadline_then_repeat_refuses() {
    let f = Fixture::simple().await;
    let s = f.session().await;
    let barrier = f
        .control(
            "barrier/arm",
            json!({"match":{"field":"issue","identifier":"SYN-1"},"timeout_ms":4500}),
        )
        .await;
    let deadline = Utc::now() + ChronoDuration::milliseconds(1200);
    let filter = scope(&["SYN-1"], &deadline.to_rfc3339());
    let req = f.request("read-issue.v1", json!({"issue":"SYN-1"}), json!([filter]));
    let task = s.call(req.clone());
    let control = async {
        f.control(
            "barrier/wait",
            json!({"id":barrier["id"],"timeout_ms":2500}),
        )
        .await;
        let wait =
            (deadline - Utc::now()).to_std().unwrap_or_default() + Duration::from_millis(120);
        tokio::time::sleep(wait).await;
        f.control(
            "barrier/release",
            json!({"id":barrier["id"],"timeout_ms":1000}),
        )
        .await;
    };
    let (answer, ()) = tokio::join!(task, control);
    success(&answer, "read-issue.v1");
    assert!(Utc::now() > deadline);
    denied(&s.call(req).await, "refused", &filter);
    assert!(f.count().await == 1);
    s.finish().await;
    f.stub.shutdown().await.unwrap();
}
#[tokio::test]
async fn malformed_metadata_inputs_filters_and_unsupported_requirement_do_nothing() {
    let f = Fixture::simple().await;
    let s = f.session().await;
    let base = f.request(
        "read-issue.v1",
        json!({"issue":"SYN-1"}),
        json!([active(&["SYN-1"])]),
    );
    for field in [
        "filters",
        "instance",
        "invocation",
        "caller",
        "config",
        "operation",
        "revision",
    ] {
        let mut req = base.clone();
        req["_meta"].as_object_mut().unwrap().remove(&key(field));
        ending(&s.call(req).await, "failed");
        let mut req = base.clone();
        req["_meta"][key(field)] = json!(7);
        ending(&s.call(req).await, "failed");
    }
    for filters in [
        json!({}),
        json!([null]),
        json!([{"requirement":2,"arguments":{}}]),
        json!([{"requirement":format!("{CONTRACTS}#issue-scope.v1"),"arguments":{"issues":[],"deadline":"2099-01-01T00:00:00Z"}}]),
    ] {
        let mut req = base.clone();
        req["_meta"][key("filters")] = filters;
        ending(&s.call(req).await, "failed");
    }
    for filter in [
        json!({"requirement":"example.org/unsupported#other.v1","arguments":{}}),
        {
            let mut filter = active(&["SYN-1"]);
            filter["arguments"]["futureInput"] = json!(true);
            filter
        },
    ] {
        let mut req = base.clone();
        req["_meta"][key("filters")] = json!([filter]);
        denied(&s.call(req).await, "unavailable", &filter);
    }
    for args in [
        json!({"issue":"syn-1"}),
        json!({"issue":"SYN-1\n"}),
        json!({"issue":"SYN-1","extra":true}),
    ] {
        let mut req = base.clone();
        req["arguments"] = args;
        ending(&s.call(req).await, "failed");
    }
    for limit in [json!(0), json!(-1), json!(1.1), json!("1"), Value::Null] {
        ending(
            &s.call(f.request("list-issues.v1", json!({"limit":limit}), json!([])))
                .await,
            "failed",
        );
    }
    assert!(f.count().await == 0);
    s.finish().await;
    f.stub.shutdown().await.unwrap();
}
#[tokio::test]
async fn strict_test_clock_never_sends_malformed_instants() {
    let f = Fixture::simple().await;
    let s = f.session().await;
    for clock in [
        json!("not an instant"),
        json!(true),
        Value::Null,
        json!("2030-01-01 00:00:00Z"),
        json!("2030-1-01T00:00:00Z"),
        json!("2030-01-1T00:00:00Z"),
        json!("2030-01-01T0:00:00Z"),
        json!("2030-01-01T00:0:00Z"),
        json!("2030-01-01T00:00:0Z"),
        json!("2030-01-01T00:00:00+0:00"),
        json!("2030-01-01T00:00:00+00:0"),
        json!("2030-01-01T00:00:00+0000"),
        json!("2030-01-01T00:00:00Z "),
        json!("2030-02-30T00:00:00Z"),
        json!("2030-01-01T00:00:00+24:00"),
    ] {
        let mut req = f.request("read-issue.v1", json!({"issue":"SYN-1"}), json!([]));
        req["_meta"][key("config")]["testClock"] = clock;
        ending(&s.call(req).await, "failed");
    }
    assert!(f.count().await == 0);
    s.finish().await;
    f.stub.shutdown().await.unwrap();
}
#[tokio::test]
async fn unsafe_http_base_url_and_unreachable_service_fail() {
    // No DNS or non-loopback request is made: this must be rejected locally.
    let url = "http://192.0.2.1/graphql";
    let marker = "synthetic-adapter-credential";
    let s = Session::start(url, marker, vec![marker.into()]).await;
    for op in OPERATIONS {
        let args = if op == "read-issue.v1" {
            json!({"issue":"SYN-1"})
        } else {
            json!({"limit":1})
        };
        let answer = s.call(request(url, op, args, json!([]))).await;
        ending(&answer, "failed");
        assert!(answer["content"][0]["text"] == "Unsafe service URL");
    }
    s.finish().await;
    let f = Fixture::simple().await;
    let url = f.stub.graphql_url();
    let secrets = f.secrets();
    let key_value = f.key.expose().to_owned();
    f.stub.shutdown().await.unwrap();
    let s = Session::start(&url, &key_value, secrets).await;
    for op in OPERATIONS {
        let args = if op == "read-issue.v1" {
            json!({"issue":"SYN-1"})
        } else {
            json!({"limit":1})
        };
        ending(&s.call(request(&url, op, args, json!([]))).await, "failed");
    }
    s.finish().await;
}
#[tokio::test]
async fn service_credential_in_content_is_suppressed() {
    let mut config = StubConfig::new(vec![]);
    let key_value = config.api_key.expose().to_owned();
    config.issues = vec![IssueFixture::new("SYN-1", key_value.clone())];
    let stub = Stub::start(config).await.unwrap();
    let s = Session::start(&stub.graphql_url(), &key_value, vec![key_value.clone()]).await;
    ending(
        &s.call(request(
            &stub.graphql_url(),
            "read-issue.v1",
            json!({"issue":"SYN-1"}),
            json!([]),
        ))
        .await,
        "failed",
    );
    s.finish().await;
    stub.shutdown().await.unwrap();
}
#[tokio::test]
async fn redacts_library_protocol_error_raw_and_escaped_and_eof_delivers_last_answer_100_times() {
    for i in 0..100 {
        let marker = if i % 2 == 0 {
            "synthetic-protocol-marker"
        } else {
            "synthetic-\"escaped\\marker"
        };
        let mut child = command("http://127.0.0.1:1/graphql", marker)
            .spawn()
            .unwrap();
        let req = json!({"jsonrpc":"2.0","id":i,"method":"server/discover","params":{"_meta":{
            "io.modelcontextprotocol/protocolVersion":marker,"io.modelcontextprotocol/clientCapabilities":{}
        }}});
        let last = json!({"jsonrpc":"2.0","id":1000+i,"method":"server/discover","params":{"_meta":{
            "io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}
        }}});
        let initialize = json!({"jsonrpc":"2.0","id":"initialize","method":"initialize","params":{
            "protocolVersion":"2026-07-28","capabilities":{},"clientInfo":{"name":"synthetic-eof-client","version":"0"}
        }});
        let initialized = json!({"jsonrpc":"2.0","method":"notifications/initialized"});
        let mut input = child.stdin.take().unwrap();
        input
            .write_all(format!("{initialize}\n{initialized}\n{req}\n{last}\n").as_bytes())
            .await
            .unwrap();
        input.shutdown().await.unwrap();
        drop(input);
        let output = tokio::time::timeout(Duration::from_secs(8), child.wait_with_output())
            .await
            .unwrap()
            .unwrap();
        clean(&output.stdout, &[marker.into()]);
        clean(&output.stderr, &[marker.into()]);
        assert!(output.status.success());
        let replies: Vec<Value> = output
            .stdout
            .split(|b| *b == b'\n')
            .filter(|s| !s.is_empty())
            .map(|s| serde_json::from_slice(s).unwrap())
            .collect();
        assert!(replies.len() == 3);
        assert!(replies[0]["id"] == "initialize");
        let reply = replies.iter().find(|reply| reply["id"] == i).unwrap();
        assert!(reply["error"]["code"] == -32022);
        assert!(reply.to_string().contains("[redacted]"));
        let last = replies
            .iter()
            .find(|reply| reply["id"] == 1000 + i)
            .unwrap();
        assert!(last["result"]["capabilities"]["extensions"][EXTENSION]["revision"] == "v0");
    }
}

#[tokio::test]
async fn service_timeout_is_bounded_and_fails_without_cancelling_by_deadline() {
    let f = Fixture::simple().await;
    let s = f.session().await;
    let barrier = f
        .control(
            "barrier/arm",
            json!({"match":{"field":"issue","identifier":"SYN-1"},"timeout_ms":9000}),
        )
        .await;
    let call = s.call(f.request(
        "read-issue.v1",
        json!({"issue":"SYN-1"}),
        json!([active(&["SYN-1"])]),
    ));
    let wait = async {
        f.control(
            "barrier/wait",
            json!({"id":barrier["id"],"timeout_ms":2500}),
        )
        .await;
    };
    let started = tokio::time::Instant::now();
    let (answer, ()) = tokio::join!(call, wait);
    ending(&answer, "failed");
    assert!(answer["content"][0]["text"] == "Service unreachable or request timed out");
    assert!(started.elapsed() < Duration::from_secs(8));
    assert!(f.count().await == 1);
    f.control(
        "barrier/release",
        json!({"id":barrier["id"],"timeout_ms":1000}),
    )
    .await;
    s.finish().await;
    f.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn http_403_fails_reads_without_echoing_service_error_body() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/graphql", listener.local_addr().unwrap());
    let marker = "synthetic-authorization-marker";
    let server = tokio::spawn(async move {
        for _ in 0..2 {
            let (mut connection, _) = listener.accept().await.unwrap();
            let mut request = [0u8; 8192];
            assert!(connection.read(&mut request).await.unwrap() > 0);
            let response = format!(
                "HTTP/1.1 403 Forbidden\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                marker.len(),
                marker
            );
            connection.write_all(response.as_bytes()).await.unwrap();
            connection.shutdown().await.unwrap();
        }
    });
    let s = Session::start(&url, marker, vec![marker.into()]).await;
    for op in OPERATIONS {
        let args = if op == "read-issue.v1" {
            json!({"issue":"SYN-1"})
        } else {
            json!({"limit":1})
        };
        let answer = s.call(request(&url, op, args, json!([]))).await;
        ending(&answer, "failed");
        assert!(answer["content"][0]["text"] == "Service authorization failed");
    }
    s.finish().await;
    tokio::time::timeout(Duration::from_secs(8), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn line_redactor_handles_split_escaped_secrets_under_backpressure() {
    use scenario_s2_linear_adapter::redactor::Redactor;
    let marker = "synthetic-\"split\\marker";
    let escaped = serde_json::to_string(marker).unwrap();
    let (writer, mut reader) = tokio::io::duplex(3);
    let mut writer = Redactor::new(writer, marker);
    let input = format!("raw:{marker}\nescaped:{escaped}\n");
    let write = async {
        for byte in input.as_bytes() {
            writer.write_all(&[*byte]).await.unwrap();
        }
        writer.flush().await.unwrap();
        writer.shutdown().await.unwrap();
    };
    let read = async {
        let mut output = Vec::new();
        reader.read_to_end(&mut output).await.unwrap();
        clean(&output, &[marker.into()]);
        assert!(output == b"raw:[redacted]\nescaped:\"[redacted]\"\n");
    };
    tokio::time::timeout(Duration::from_secs(5), async {
        tokio::join!(write, read);
    })
    .await
    .unwrap();
}
