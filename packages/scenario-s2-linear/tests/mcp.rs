use rmcp::{
    RoleClient, ServiceExt,
    service::{RunningService, ServiceError},
    transport::TokioChildProcess,
};
use s2_linear_stub::{IssueFixture, Stub, StubConfig, SyntheticToken};
use scenario_s2_linear::{
    CONTRACTS, EXTENSION, OPERATIONS, OUTCOME, PREFIX, REASON, REQUIREMENT, RESTRICTION, valid,
};
use serde_json::{Value, json};
use std::{process::Stdio, time::Duration};

struct Process {
    client: RunningService<RoleClient, ()>,
    stderr: tempfile::NamedTempFile,
    forbidden: Vec<String>,
}
impl Process {
    async fn start(endpoint: &str, key: &SyntheticToken, control: &SyntheticToken) -> Self {
        let stderr = tempfile::NamedTempFile::new().unwrap();
        let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_scenario-s2-linear"));
        command
            .env_clear()
            .env("S2_LINEAR_BASE_URL", endpoint)
            .env("S2_LINEAR_API_KEY", key.expose())
            .stderr(Stdio::from(stderr.reopen().unwrap()));
        let client = tokio::time::timeout(
            Duration::from_secs(10),
            ().serve(TokioChildProcess::new(command).unwrap()),
        )
        .await
        .expect("stdio startup timed out")
        .unwrap_or_else(|_| panic!("stdio startup failed"));
        Self {
            client,
            stderr,
            forbidden: vec![key.expose().to_owned(), control.expose().to_owned()],
        }
    }
    fn clean(&self, value: &Value) {
        let text = value.to_string();
        assert!(
            self.forbidden.iter().all(|token| !text.contains(token)),
            "credential leak in MCP traffic"
        );
    }
    async fn wire(&self, request: Value) -> Value {
        let result = self
            .client
            .call_tool(serde_json::from_value(request).unwrap())
            .await
            .unwrap_or_else(|_| panic!("MCP call unexpectedly failed"));
        let wire = serde_json::to_value(result).unwrap();
        self.clean(&wire);
        let state = wire["_meta"][OUTCOME].as_str();
        assert!(
            wire["isError"] == (state != Some("succeeded")),
            "isError disagrees with outcome"
        );
        if state == Some("succeeded") {
            assert!(wire.get("structuredContent").is_some());
        } else {
            assert!(
                wire.get("structuredContent").is_none(),
                "failure/refusal delivered issue output"
            );
        }
        if wire["_meta"].get(REASON).is_some() {
            assert!(wire["_meta"].get(OUTCOME).is_none());
        }
        wire
    }
    async fn success(&self, request: Value) -> Value {
        let name = request["name"].as_str().unwrap().to_owned();
        let wire = self.wire(request).await;
        assert!(
            wire["_meta"][OUTCOME] == "succeeded",
            "expected successful issue operation"
        );
        assert!(valid(&name, "outputSchema", &wire["structuredContent"]));
        wire["structuredContent"].clone()
    }
    async fn stop(self) {
        self.client
            .cancel()
            .await
            .unwrap_or_else(|_| panic!("MCP stop failed"));
        let stderr = std::fs::read_to_string(self.stderr.path()).unwrap();
        assert!(
            self.forbidden.iter().all(|token| !stderr.contains(token)),
            "credential leak in stderr"
        );
        assert!(stderr.is_empty(), "unexpected server stderr or log output");
    }
}
struct Harness {
    stub: Stub,
    key: SyntheticToken,
    control: SyntheticToken,
    http: reqwest::Client,
}
impl Harness {
    async fn new() -> Self {
        let mut issues: Vec<_> = (1..=105)
            .map(|n| IssueFixture::new(format!("SYN-{n}"), "Synthetic issue"))
            .collect();
        issues[0].description = Some("Mention SYN-2 remains content".into());
        let config = StubConfig::new(issues);
        let key = config.api_key.clone();
        let control = config.control_token.clone();
        Self {
            stub: Stub::start(config).await.expect("loopback stub start"),
            key,
            control,
            http: reqwest::Client::builder()
                .no_proxy()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }
    async fn start(&self) -> Process {
        Process::start(&self.stub.graphql_url(), &self.key, &self.control).await
    }
    async fn control(&self, path: &str, body: Value) -> Value {
        let response = self
            .http
            .post(format!("{}/{path}", self.stub.control_url()))
            .header("Authorization", self.control.expose())
            .json(&body)
            .send()
            .await
            .expect("loopback control response");
        assert!(response.status().is_success(), "control request failed");
        response.json().await.expect("control JSON")
    }
    async fn count(&self) -> u64 {
        self.http
            .get(format!("{}/count", self.stub.control_url()))
            .header("Authorization", self.control.expose())
            .send()
            .await
            .expect("loopback count")
            .json::<Value>()
            .await
            .expect("count JSON")["api_requests"]
            .as_u64()
            .unwrap()
    }
    fn call(&self, name: &str, args: Value, filters: Value) -> Value {
        json!({"name":name,"arguments":args,"_meta":{
            "io.modelcontextprotocol/clientCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}},
            format!("{PREFIX}instance"):"s2-instance",
            format!("{PREFIX}operation"):format!("{CONTRACTS}#{name}"),
            format!("{PREFIX}invocation"):"s2-invocation",
            format!("{PREFIX}caller"):"s2-caller",
            format!("{PREFIX}revision"):"s2-revision",
            format!("{PREFIX}conduit"):"s2-conduit",
            format!("{PREFIX}idempotencyKey"):"s2-repeat",
            format!("{PREFIX}filters"):filters,
            format!("{PREFIX}config"):{"baseUrl":self.stub.graphql_url(),"apiKey":{"kind":"Resource","id":"s2-secret"},"testClock":"2026-01-01T00:00:00Z"}
        }})
    }
}
fn scope(issues: &[&str], deadline: &str) -> Value {
    json!({"requirement":REQUIREMENT,"arguments":{"issues":issues,"deadline":deadline}})
}
fn filters(issues: &[&str]) -> Value {
    json!([scope(issues, "2026-01-01T00:00:01Z")])
}
fn arguments(name: &str, issue: &str) -> Value {
    match name {
        "list-issues.v1" => json!({"limit":1}),
        "update-issue.v1" => json!({"issue":issue,"title":"Synthetic updated title"}),
        _ => json!({"issue":issue}),
    }
}
fn assert_denied(wire: &Value, reason: &str, filter: &Value) {
    assert!(wire["_meta"][REASON] == reason);
    assert!(
        wire["_meta"][RESTRICTION] == *filter,
        "received restriction must be preserved"
    );
    assert!(wire["_meta"].get(OUTCOME).is_none());
    assert!(wire["isError"] == true);
    assert!(wire.get("structuredContent").is_none());
}

#[tokio::test]
async fn discovery_and_required_capabilities_do_no_service_work() {
    let h = Harness::new().await;
    let p = h.start().await;
    let discovery = p.client.discover(serde_json::from_value(json!({"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}})).unwrap()).await.unwrap_or_else(|_| panic!("discovery failed"));
    let wire = serde_json::to_value(discovery).unwrap();
    p.clean(&wire);
    assert!(wire["capabilities"]["extensions"][EXTENSION]["revision"] == "v0");
    let tools = p
        .client
        .list_all_tools()
        .await
        .unwrap_or_else(|_| panic!("tool listing failed"));
    assert!(tools.len() == 3);
    for tool in tools {
        assert!(OPERATIONS.contains(&tool.name.as_ref()));
        let wire = serde_json::to_value(&tool).unwrap();
        p.clean(&wire);
        assert!(
            wire["inputSchema"] == scenario_s2_linear::SCHEMAS[tool.name.as_ref()]["inputSchema"]
        );
        assert!(
            wire["outputSchema"] == scenario_s2_linear::SCHEMAS[tool.name.as_ref()]["outputSchema"]
        );
    }
    for name in OPERATIONS {
        for capability in [
            json!({}),
            json!({"extensions":{EXTENSION:{"revision":"v1"}}}),
        ] {
            let mut request = h.call(name, arguments(name, "SYN-1"), filters(&["SYN-1"]));
            request["_meta"]["io.modelcontextprotocol/clientCapabilities"] = capability;
            let result = p
                .client
                .call_tool(serde_json::from_value(request).unwrap())
                .await;
            let Err(ServiceError::McpError(error)) = result else {
                panic!("expected MCP capability error");
            };
            let wire = serde_json::to_value(error).unwrap();
            p.clean(&wire);
            assert!(wire["code"] == -32021);
            assert!(
                wire["data"]["requiredCapabilities"]["extensions"][EXTENSION]["revision"] == "v0"
            );
        }
    }
    assert!(h.count().await == 0);
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn unknown_tool_is_a_protocol_error_after_required_capabilities() {
    let h = Harness::new().await;
    let p = h.start().await;
    for (capability, code) in [
        (json!({}), -32021),
        (json!({"extensions":{EXTENSION:{"revision":"v1"}}}), -32021),
        (json!({"extensions":{EXTENSION:{"revision":"v0"}}}), -32602),
    ] {
        let mut request = h.call("unknown-issue.v1", json!({}), json!([]));
        request["_meta"]["io.modelcontextprotocol/clientCapabilities"] = capability;
        let result = p
            .client
            .call_tool(serde_json::from_value(request).unwrap())
            .await;
        let Err(ServiceError::McpError(error)) = result else {
            panic!("expected unknown-tool protocol error");
        };
        let wire = serde_json::to_value(error).unwrap();
        p.clean(&wire);
        assert!(wire["code"] == code);
        if code == -32021 {
            assert!(
                wire["data"]["requiredCapabilities"]["extensions"][EXTENSION]["revision"] == "v0"
            );
        }
        assert!(h.count().await == 0);
    }
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn list_limits_accept_integral_decimal_and_exponent_representations() {
    let h = Harness::new().await;
    let p = h.start().await;
    let expected = ["SYN-1", "SYN-105", "SYN-99"];
    for (limit, count) in [
        ("1", 1),
        ("1.0", 1),
        ("1e0", 1),
        ("2.0", 2),
        ("2e0", 2),
        ("9007199254740991.0", 3),
    ] {
        let args = json!({"limit":serde_json::from_str::<Value>(limit).unwrap()});
        assert!(valid("list-issues.v1", "inputSchema", &args));
        let list = p
            .success(h.call(
                "list-issues.v1",
                args,
                filters(&["SYN-99", "SYN-105", "SYN-1", "SYN-999"]),
            ))
            .await;
        let identifiers: Vec<_> = list["issues"]
            .as_array()
            .unwrap()
            .iter()
            .map(|issue| issue["identifier"].as_str().unwrap())
            .collect();
        assert!(
            identifiers == expected[..count],
            "list limit was not applied"
        );
    }
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn in_scope_projection_mentions_and_edit_semantics() {
    let h = Harness::new().await;
    let p = h.start().await;
    let read = p
        .success(h.call(
            "read-issue.v1",
            arguments("read-issue.v1", "SYN-1"),
            filters(&["SYN-1"]),
        ))
        .await;
    assert!(read["issue"].as_object().unwrap().len() == 5);
    assert!(read["issue"]["description"] == "Mention SYN-2 remains content");
    assert!(read["issue"]["state"] == "Todo");
    let update = p
        .success(h.call(
            "update-issue.v1",
            json!({"issue":"SYN-1","title":""}),
            filters(&["SYN-1"]),
        ))
        .await;
    assert!(update["issue"]["title"] == "");
    assert!(update["issue"]["description"] == read["issue"]["description"]);
    let cleared = p
        .success(h.call(
            "update-issue.v1",
            json!({"issue":"SYN-1","description":null}),
            filters(&["SYN-1"]),
        ))
        .await;
    assert!(cleared["issue"]["description"].is_null());
    assert!(cleared["issue"]["title"] == "");
    // An explicit empty filter array permits access, but never adds projection fields.
    p.success(h.call(
        "read-issue.v1",
        arguments("read-issue.v1", "SYN-2"),
        json!([]),
    ))
    .await;
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn scope_before_lookup_and_limit_including_duplicate_and_intersecting_filters() {
    let h = Harness::new().await;
    let p = h.start().await;
    for name in ["read-issue.v1", "update-issue.v1"] {
        for issue in ["SYN-2", "SYN-999"] {
            let f = filters(&["SYN-1"]);
            let wire = p
                .wire(h.call(name, arguments(name, issue), f.clone()))
                .await;
            assert_denied(&wire, "refused", &f[0]);
        }
    }
    assert!(h.count().await == 0);
    // SYN-99 is past the stub's first 100-node page in ASCII order.
    let f = filters(&["SYN-99", "SYN-105", "SYN-999"]);
    let list = p
        .success(h.call("list-issues.v1", json!({"limit":1}), f))
        .await;
    assert!(list["issues"].as_array().unwrap().len() == 1);
    assert!(list["issues"][0]["identifier"] == "SYN-105");
    let sorted = p
        .success(h.call(
            "list-issues.v1",
            json!({"limit":10}),
            filters(&["SYN-99", "SYN-105", "SYN-1"]),
        ))
        .await;
    let identifiers: Vec<_> = sorted["issues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| issue["identifier"].as_str().unwrap())
        .collect();
    assert!(identifiers == ["SYN-1", "SYN-105", "SYN-99"]);
    let a = scope(&["SYN-99", "SYN-105"], "2026-01-01T00:00:01Z");
    let b = scope(&["SYN-99"], "2026-01-01T00:00:01Z");
    let list = p
        .success(h.call("list-issues.v1", json!({"limit":5}), json!([a, a, b])))
        .await;
    assert!(list["issues"].as_array().unwrap().len() == 1);
    assert!(list["issues"][0]["identifier"] == "SYN-99");
    let list = p
        .success(h.call("list-issues.v1", json!({"limit":20}), filters(&["SYN-999"])))
        .await;
    assert!(list["issues"] == json!([]));
    let missing = p
        .wire(h.call(
            "read-issue.v1",
            arguments("read-issue.v1", "SYN-999"),
            filters(&["SYN-999"]),
        ))
        .await;
    assert!(missing["_meta"][OUTCOME] == "failed");
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn deadlines_compare_offsets_repeat_reads_and_judge_each_call() {
    let h = Harness::new().await;
    let p = h.start().await;
    let mut repeated = h.call(
        "read-issue.v1",
        arguments("read-issue.v1", "SYN-1"),
        filters(&["SYN-1"]),
    );
    p.success(repeated.clone()).await;
    repeated["_meta"][format!("{PREFIX}config")]["testClock"] = json!("2026-01-01T00:00:01Z");
    let before = h.count().await;
    let wire = p.wire(repeated.clone()).await;
    assert_denied(
        &wire,
        "refused",
        &repeated["_meta"][format!("{PREFIX}filters")][0],
    );
    for name in OPERATIONS {
        for deadline in ["2025-12-31T23:00:00Z", "2025-12-31T19:00:00-05:00"] {
            let f = json!([scope(&["SYN-1"], deadline)]);
            let wire = p
                .wire(h.call(name, arguments(name, "SYN-1"), f.clone()))
                .await;
            assert_denied(&wire, "refused", &f[0]);
        }
    }
    assert!(h.count().await == before);
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn changed_filter_data_and_service_authorization_are_distinct() {
    let h = Harness::new().await;
    let p = h.start().await;
    let mut request = h.call(
        "read-issue.v1",
        arguments("read-issue.v1", "SYN-1"),
        filters(&["SYN-1"]),
    );
    p.success(request.clone()).await;
    let before = h.count().await;
    request["_meta"][format!("{PREFIX}filters")] = filters(&["SYN-2"]);
    let wire = p.wire(request).await;
    assert_denied(&wire, "refused", &filters(&["SYN-2"])[0]);
    assert!(h.count().await == before);
    // The service's own authorization changes. It receives no Conduit metadata.
    h.control(
        "service-authorization",
        json!({"key":h.key.expose(),"issues":["SYN-2"]}),
    )
    .await;
    let wire = p
        .wire(h.call(
            "read-issue.v1",
            arguments("read-issue.v1", "SYN-1"),
            filters(&["SYN-1"]),
        ))
        .await;
    assert!(wire["_meta"][OUTCOME] == "failed");
    assert!(h.count().await == before + 1);
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn fail_closed_metadata_inputs_and_unsupported_requirements_send_nothing() {
    let h = Harness::new().await;
    let p = h.start().await;
    let base = h.call(
        "read-issue.v1",
        arguments("read-issue.v1", "SYN-1"),
        filters(&["SYN-1"]),
    );
    for key in [
        "instance",
        "operation",
        "invocation",
        "caller",
        "filters",
        "config",
        "revision",
    ] {
        for bad in [None, Some(json!(null)), Some(json!(7))] {
            let mut request = base.clone();
            if let Some(value) = bad {
                request["_meta"][format!("{PREFIX}{key}")] = value;
            } else {
                request["_meta"]
                    .as_object_mut()
                    .unwrap()
                    .remove(&format!("{PREFIX}{key}"));
            }
            let wire = p.wire(request).await;
            assert!(wire["_meta"][OUTCOME] == "failed");
        }
    }
    for f in [
        json!({}),
        json!("bad"),
        json!([{}]),
        json!([{"requirement":REQUIREMENT,"arguments":{"issues":[],"deadline":"bad"}}]),
    ] {
        let mut request = base.clone();
        request["_meta"][format!("{PREFIX}filters")] = f;
        assert!(p.wire(request).await["_meta"][OUTCOME] == "failed");
    }
    for f in [
        json!({"requirement":"example.org/unsupported#other.v1","arguments":{}}),
        json!({"requirement":REQUIREMENT,"arguments":{"issues":["SYN-1"],"deadline":"2026-01-01T00:00:01Z","futureInput":true}}),
    ] {
        let wire = p
            .wire(h.call(
                "read-issue.v1",
                arguments("read-issue.v1", "SYN-1"),
                json!([f]),
            ))
            .await;
        assert_denied(&wire, "unavailable", &f);
    }
    for bad in [
        json!(null),
        json!(7),
        json!("not-a-clock"),
        // chrono accepts a space here; the declared date-time format does not.
        json!("2026-01-01 00:00:00Z"),
        // The format checker rejects these too; chrono alone accepts them.
        json!("2026-01-01T00:00:00\u{2212}05:00"),
        json!("2026-01-01T12:00:60Z"),
        json!("1998-12-31T23:59:60+01:00"),
    ] {
        let mut request = base.clone();
        request["_meta"][format!("{PREFIX}config")]["testClock"] = bad;
        assert!(p.wire(request).await["_meta"][OUTCOME] == "failed");
    }
    for name in OPERATIONS {
        for args in [
            json!({}),
            json!({"issue":"syn-1"}),
            json!({"issue":"SYN-1","team":"move"}),
        ] {
            assert!(
                p.wire(h.call(name, args, filters(&["SYN-1"]))).await["_meta"][OUTCOME] == "failed"
            );
        }
    }
    assert!(h.count().await == 0);
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn allowed_receipt_completes_after_deadline_at_stub_barrier() {
    let h = Harness::new().await;
    let p = h.start().await;
    let barrier = h
        .control(
            "barrier/arm",
            json!({"match":{"field":"issue","identifier":"SYN-1"},"timeout_ms":5000}),
        )
        .await;
    let id = barrier["id"].clone();
    let deadline = chrono::Utc::now() + chrono::Duration::milliseconds(500);
    let mut request = h.call(
        "read-issue.v1",
        arguments("read-issue.v1", "SYN-1"),
        json!([scope(&["SYN-1"], &deadline.to_rfc3339())]),
    );
    request["_meta"][format!("{PREFIX}config")]
        .as_object_mut()
        .unwrap()
        .remove("testClock");
    let held = p.success(request.clone());
    let release = async {
        h.control("barrier/wait", json!({"id":id,"timeout_ms":2000}))
            .await;
        tokio::time::sleep(Duration::from_millis(600)).await;
        assert!(chrono::Utc::now() >= deadline);
        h.control("barrier/release", json!({"id":id,"timeout_ms":2000}))
            .await;
    };
    let (output, ()) = tokio::join!(held, release);
    assert!(output["issue"]["identifier"] == "SYN-1");
    let before = h.count().await;
    let wire = p.wire(request.clone()).await;
    assert_denied(
        &wire,
        "refused",
        &request["_meta"][format!("{PREFIX}filters")][0],
    );
    assert!(h.count().await == before);
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn rotation_requires_new_process_original_key_remains_fixed() {
    let h = Harness::new().await;
    let mut old = h.start().await;
    old.success(h.call(
        "read-issue.v1",
        arguments("read-issue.v1", "SYN-1"),
        filters(&["SYN-1"]),
    ))
    .await;
    let new_key = SyntheticToken::generate();
    old.forbidden.push(new_key.expose().to_owned());
    h.control("rotate-key", json!({"key":new_key.expose()}))
        .await;
    let mut new = Process::start(&h.stub.graphql_url(), &new_key, &h.control).await;
    new.forbidden.push(h.key.expose().to_owned());
    for name in OPERATIONS {
        let request = h.call(name, arguments(name, "SYN-1"), filters(&["SYN-1"]));
        assert!(old.wire(request.clone()).await["_meta"][OUTCOME] == "failed");
        new.success(request).await;
    }
    old.stop().await;
    new.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn unreachable_service_outcomes_preserve_effect_uncertainty() {
    let h = Harness::new().await;
    let p = h.start().await;
    let requests: Vec<_> = OPERATIONS
        .iter()
        .map(|name| {
            (
                *name,
                h.call(name, arguments(name, "SYN-1"), filters(&["SYN-1"])),
            )
        })
        .collect();
    h.stub.shutdown().await.unwrap();
    for (name, request) in requests {
        let wire = p.wire(request).await;
        assert!(
            wire["_meta"][OUTCOME]
                == if name == "update-issue.v1" {
                    "unknown"
                } else {
                    "failed"
                }
        );
    }
    p.stop().await;
}

#[test]
fn insecure_or_credential_bearing_endpoints_are_rejected() {
    for endpoint in [
        "http://192.0.2.1/graphql",
        "http://localhost/graphql",
        "file:///graphql",
        "https://user@example.invalid/graphql",
        "https://example.invalid/graphql?query=x",
        "https://example.invalid/graphql#fragment",
    ] {
        assert!(
            scenario_s2_linear::LinearServer::new(endpoint.into(), "synthetic-test-header".into())
                .is_err()
        );
    }
    for endpoint in [
        "http://127.0.0.1:1/graphql",
        "http://[::1]:1/graphql",
        "https://example.invalid/graphql",
    ] {
        assert!(
            scenario_s2_linear::LinearServer::new(endpoint.into(), "synthetic-test-header".into())
                .is_ok()
        );
    }
}

#[tokio::test]
async fn held_call_keeps_its_test_clock_receipt_when_later_call_crosses_deadline() {
    let h = Harness::new().await;
    let p = h.start().await;
    let barrier = h
        .control(
            "barrier/arm",
            json!({"match":{"field":"issue","identifier":"SYN-1"},"timeout_ms":5000}),
        )
        .await;
    let id = barrier["id"].clone();
    let request = h.call(
        "read-issue.v1",
        arguments("read-issue.v1", "SYN-1"),
        filters(&["SYN-1"]),
    );
    let held = p.success(request.clone());
    let later = async {
        h.control("barrier/wait", json!({"id":id,"timeout_ms":2000}))
            .await;
        let before = h.count().await;
        let mut expired = request.clone();
        expired["_meta"][format!("{PREFIX}config")]["testClock"] = json!("2026-01-01T00:00:01Z");
        let wire = p.wire(expired).await;
        assert_denied(&wire, "refused", &filters(&["SYN-1"])[0]);
        assert!(h.count().await == before);
        h.control("barrier/release", json!({"id":id,"timeout_ms":2000}))
            .await;
    };
    let (output, ()) = tokio::join!(held, later);
    assert!(output["issue"]["identifier"] == "SYN-1");
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn service_timeouts_split_reads_from_updates() {
    let h = Harness::new().await;
    let p = h.start().await;
    for (name, field) in [
        ("read-issue.v1", "issue"),
        ("list-issues.v1", "issues"),
        ("update-issue.v1", "issueUpdate"),
    ] {
        let barrier = h
            .control(
                "barrier/arm",
                json!({"match":{"field":field},"timeout_ms":5000}),
            )
            .await;
        let wire = p
            .wire(h.call(name, arguments(name, "SYN-1"), filters(&["SYN-1"])))
            .await;
        assert!(
            wire["_meta"][OUTCOME]
                == if name == "update-issue.v1" {
                    "unknown"
                } else {
                    "failed"
                }
        );
        h.control(
            "barrier/release",
            json!({"id":barrier["id"],"timeout_ms":2000}),
        )
        .await;
    }
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn credential_in_service_content_is_suppressed_and_never_logged() {
    let mut config = StubConfig::new(vec![IssueFixture::new("SYN-1", "Synthetic issue")]);
    config.issues[0].description = Some(config.api_key.expose().to_owned());
    let key = config.api_key.clone();
    let control = config.control_token.clone();
    let stub = Stub::start(config).await.unwrap();
    let h = Harness {
        stub,
        key,
        control,
        http: reqwest::Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap(),
    };
    let p = h.start().await;
    for name in OPERATIONS {
        let wire = p
            .wire(h.call(name, arguments(name, "SYN-1"), filters(&["SYN-1"])))
            .await;
        assert!(
            wire["_meta"][OUTCOME]
                == if name == "update-issue.v1" {
                    "unknown"
                } else {
                    "failed"
                }
        );
    }
    // An undeclared filter input must not make the server echo its held key.
    let f = json!([{ "requirement":REQUIREMENT,"arguments":{"issues":["SYN-1"],"deadline":"2026-01-01T00:00:01Z","future":h.key.expose()}}]);
    let before = h.count().await;
    assert!(
        p.wire(h.call("read-issue.v1", arguments("read-issue.v1", "SYN-1"), f))
            .await["_meta"][OUTCOME]
            == "failed"
    );
    assert!(h.count().await == before);
    p.stop().await;
    h.stub.shutdown().await.unwrap();
}

#[tokio::test]
async fn forbidden_http_response_never_echoes_service_body_or_marker() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let h = Harness::new().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/graphql", listener.local_addr().unwrap());
    let marker = h.control.expose().to_owned();
    let server = tokio::spawn(async move {
        for _ in 0..3 {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = vec![0; 8192];
            let _ = socket.read(&mut request).await.unwrap();
            let reply = format!(
                "HTTP/1.1 403 Forbidden\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                marker.len(),
                marker
            );
            socket.write_all(reply.as_bytes()).await.unwrap();
        }
    });
    let p = Process::start(&endpoint, &h.key, &h.control).await;
    for name in OPERATIONS {
        let mut request = h.call(name, arguments(name, "SYN-1"), filters(&["SYN-1"]));
        request["_meta"][format!("{PREFIX}config")]["baseUrl"] = json!(endpoint);
        assert!(p.wire(request).await["_meta"][OUTCOME] == "failed");
    }
    p.stop().await;
    server.await.unwrap();
    h.stub.shutdown().await.unwrap();
}

// A protocol error that the SDK builds before any tool code runs, echoing a field the caller
// sent, must not carry the key: the binary redacts every line it writes.
#[tokio::test]
async fn sdk_errors_never_echo_the_key() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    let key = "SYNTHETIC-REDACTION-PROBE-0001";
    let mut child = tokio::process::Command::new(env!("CARGO_BIN_EXE_scenario-s2-linear"))
        .env_clear()
        .env("S2_LINEAR_BASE_URL", "http://127.0.0.1:9/graphql")
        .env("S2_LINEAR_API_KEY", key)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let discover = json!({"jsonrpc":"2.0","id":1,"method":"server/discover","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}}});
    let call = json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"read-issue.v1","arguments":{"issue":"SYN-1"},"_meta":{"io.modelcontextprotocol/protocolVersion":key,"io.modelcontextprotocol/clientCapabilities":{}}}});
    for message in [discover, call] {
        stdin
            .write_all(format!("{message}\n").as_bytes())
            .await
            .unwrap();
    }
    stdin.flush().await.unwrap();
    let mut answers = Vec::new();
    for _ in 0..2 {
        let line = tokio::time::timeout(Duration::from_secs(10), lines.next_line())
            .await
            .expect("no answer")
            .unwrap()
            .expect("stdout closed");
        assert!(!line.contains(key), "an answer carried the key");
        answers.push(serde_json::from_str::<Value>(&line).unwrap());
    }
    // The redactor fired on the SDK's own error, which echoes the requested version.
    let error = answers.iter().find(|a| a["id"] == 2).unwrap();
    assert!(error["error"]["code"] == -32022);
    assert!(error["error"]["data"]["requested"] == "[redacted]");
    drop(stdin);
    let _ = child.kill().await;
}

// The redactor never drops an answer: a discover followed at once by the end of stdin still
// gets its answer, every time.
#[tokio::test]
async fn early_end_of_input_loses_no_answer() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let discover = json!({"jsonrpc":"2.0","id":1,"method":"server/discover","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}}});
    for _ in 0..100 {
        let mut child = tokio::process::Command::new(env!("CARGO_BIN_EXE_scenario-s2-linear"))
            .env_clear()
            .env("S2_LINEAR_BASE_URL", "http://127.0.0.1:9/graphql")
            .env("S2_LINEAR_API_KEY", "SYNTHETIC-EOF-PROBE-0001")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let mut stdin = child.stdin.take().unwrap();
        stdin
            .write_all(format!("{discover}\n").as_bytes())
            .await
            .unwrap();
        drop(stdin);
        let mut out = String::new();
        tokio::time::timeout(
            Duration::from_secs(10),
            child.stdout.take().unwrap().read_to_string(&mut out),
        )
        .await
        .expect("stdout did not close")
        .unwrap();
        let _ = child.wait().await;
        assert!(out.contains("\"id\":1"), "the discover answer was lost");
    }
}
