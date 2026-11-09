//! Scripted HTTP client; no Claude/Codex process, model, or outside network.
use futures_util::StreamExt;
use real_client_harness::{
    Names,
    stub::{self, Case},
};
use reqwest::{Client, Response};
use serde_json::{Value, json};
use std::{fs, time::Duration};

struct Lab {
    dir: tempfile::TempDir,
    server: stub::RunningStub,
    client: Client,
    token: String,
    revision: &'static str,
    session: Option<String>,
    id: u64,
}
impl Drop for Lab {
    fn drop(&mut self) {
        self.server.cancel.cancel();
        self.server.task.abort();
    }
}
impl Lab {
    async fn new(case: Case, revision: &'static str, ttl: u64, first_401: bool) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let server = stub::start(case, ttl, first_401, dir.path(), 0)
            .await
            .unwrap();
        let token = fs::read_to_string(
            dir.path()
                .join("identity")
                .join(Names::default().get("tokenFile")),
        )
        .unwrap();
        Self {
            dir,
            server,
            client: Client::builder()
                .no_proxy()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
            token,
            revision,
            session: None,
            id: 0,
        }
    }
    fn request(&mut self, method: &str, params: Value) -> reqwest::RequestBuilder {
        self.id += 1;
        let mut params = params;
        let mut request = self
            .client
            .post(&self.server.url)
            .bearer_auth(&self.token)
            .header("accept", "application/json, text/event-stream");
        if self.revision == "2026-07-28" {
            params["_meta"] = json!({"io.modelcontextprotocol/protocolVersion":self.revision,"io.modelcontextprotocol/clientInfo":{"name":"scripted-test","version":"1"},"io.modelcontextprotocol/clientCapabilities":{}});
            request = request.header("mcp-method", method);
            if method == "tools/call" {
                request = request.header("mcp-name", params["name"].as_str().unwrap_or(""));
            }
        }
        request = request.header("mcp-protocol-version", self.revision);
        if let Some(ref id) = self.session {
            request = request.header("mcp-session-id", id);
        }
        request.json(&json!({"jsonrpc":"2.0","id":self.id,"method":method,"params":params}))
    }
    async fn rpc(&mut self, method: &str, params: Value) -> Value {
        let response = self.request(method, params).send().await.unwrap();
        assert!(response.status().is_success(), "HTTP {}", response.status());
        response_json(response).await
    }
    async fn initialize(&mut self) {
        if self.revision == "2026-07-28" {
            let result = self.rpc("server/discover", json!({})).await;
            assert_eq!(
                result["result"]["supportedVersions"],
                json!(stub::REVISIONS)
            );
            assert_eq!(
                result["result"]["capabilities"]["tools"]["listChanged"],
                true
            );
            assert_eq!(
                result["result"]["capabilities"]["extensions"]
                    [Names::default().meta("implementation-interface")]["revision"],
                "v0"
            );
            return;
        }
        let revision = self.revision;
        let response=self.request("initialize",json!({"protocolVersion":revision,"capabilities":{},"clientInfo":{"name":"scripted","version":"1"}})).send().await.unwrap();
        self.session = response
            .headers()
            .get("mcp-session-id")
            .map(|s| s.to_str().unwrap().into());
        assert_eq!(
            response_json(response).await["result"]["protocolVersion"],
            revision
        );
        let mut req = self
            .client
            .post(&self.server.url)
            .bearer_auth(&self.token)
            .header("accept", "application/json, text/event-stream")
            .header("mcp-protocol-version", revision);
        if let Some(ref session) = self.session {
            req = req.header("mcp-session-id", session);
        }
        assert_eq!(
            req.json(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
                .send()
                .await
                .unwrap()
                .status(),
            202
        );
    }
    async fn call(&mut self, name: &str, args: Value) -> Value {
        self.rpc("tools/call", json!({"name":name,"arguments":args}))
            .await
    }
    async fn list(&mut self, expected: &[&str], ttl: u64) {
        let response = self.rpc("tools/list", json!({})).await;
        let result = &response["result"];
        assert_eq!(
            result["tools"]
                .as_array()
                .unwrap()
                .iter()
                .map(|t| t["name"].as_str().unwrap())
                .collect::<Vec<_>>(),
            expected
        );
        // IFC-038: metadata survives even on legacy clients (an additive hint).
        assert_eq!(result["cacheScope"], "private");
        assert_eq!(result["ttlMs"], ttl);
        for tool in result["tools"].as_array().unwrap() {
            // IFC-033: self-contained contracts.
            assert_eq!(tool["inputSchema"]["type"], "object");
            assert_eq!(tool["outputSchema"]["type"], "object");
            if tool["name"].as_str().unwrap().ends_with(".read.v1") {
                assert_eq!(tool["annotations"]["readOnlyHint"], true);
            } else {
                assert!(tool["annotations"].get("readOnlyHint").is_none());
            }
        }
    }
    async fn stream(&mut self) -> Response {
        let response = if self.revision == "2026-07-28" {
            self.request(
                "subscriptions/listen",
                json!({"notifications":{"toolsListChanged":true}}),
            )
            .send()
            .await
            .unwrap()
        } else {
            self.client
                .get(&self.server.url)
                .bearer_auth(&self.token)
                .header("accept", "text/event-stream")
                .header("mcp-protocol-version", self.revision)
                .header("mcp-session-id", self.session.as_ref().unwrap())
                .send()
                .await
                .unwrap()
        };
        assert_eq!(response.status(), 200);
        response
    }
}
async fn response_json(response: Response) -> Value {
    let content_type = response
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let text = response.text().await.unwrap();
    if content_type.starts_with("application/json") {
        serde_json::from_str(&text).unwrap()
    } else {
        text.lines()
            .filter_map(|line| line.strip_prefix("data: "))
            .filter_map(|s| serde_json::from_str::<Value>(s).ok())
            .find(|v| v.get("result").is_some() || v.get("error").is_some())
            .unwrap_or_else(|| panic!("No response in {text}"))
    }
}
fn outcome(result: &Value, value: &str) {
    // IFC-025 / OPS-001.
    let names = Names::default();
    let result = &result["result"];
    assert_eq!(result["_meta"][names.meta("outcome")], value);
    assert!(
        result["_meta"][names.meta("invocation")]
            .as_str()
            .unwrap()
            .starts_with("inv-")
    );
    assert!(result["_meta"].get(names.meta("reason")).is_none());
    assert_eq!(result["isError"], value != "succeeded");
    if value == "succeeded" {
        assert!(result["structuredContent"].is_object())
    } else {
        assert!(result.get("structuredContent").is_none());
        assert_eq!(result["content"][0]["text"], value);
    }
}
fn refused(result: &Value) {
    // IFC-012 / IFC-013 / ACC-011 / OPS-005.
    assert_eq!(result["error"]["code"], -32602);
    assert_eq!(
        result["error"]["data"][Names::default().meta("reason")],
        "refused"
    );
    assert!(result.get("result").is_none());
    assert!(result.get("isError").is_none());
    assert!(
        result["error"]["data"]
            .get(Names::default().meta("invocation"))
            .is_none()
    );
    assert!(
        result["error"]["data"]
            .get(Names::default().meta("outcome"))
            .is_none()
    );
}

#[tokio::test]
async fn ifc_012_013_025_033_038_acc_011_ops_001_005_all_cases_all_revisions() {
    for revision in stub::REVISIONS {
        for case in [Case::C1, Case::C2, Case::C3, Case::C4, Case::C5] {
            for ttl in [0, 60000] {
                let mut lab = Lab::new(case, revision, ttl, false).await;
                lab.initialize().await;
                lab.list(&["control.next.v1", "files.read.v1", "files.write.v1"], ttl)
                    .await;
                match case {
                    Case::C1 | Case::C5 => {
                        outcome(
                            &lab.call("files.read.v1", json!({"path":"hello.txt"})).await,
                            "succeeded",
                        );
                        let response = lab.stream().await;
                        let mut stream = response.bytes_stream();
                        // New revision acknowledges the subscription before it can deliver a change.
                        if revision == "2026-07-28" {
                            let first = stream.next().await.unwrap().unwrap();
                            assert!(
                                String::from_utf8_lossy(&first)
                                    .contains("notifications/subscriptions/acknowledged")
                            );
                        }
                        outcome(&lab.call("control.next.v1", json!({})).await, "succeeded");
                        let notification = tokio::time::timeout(Duration::from_secs(2), async {
                            loop {
                                let chunk = stream.next().await.unwrap().unwrap();
                                let text = String::from_utf8_lossy(&chunk);
                                if text.contains("notifications/tools/list_changed") {
                                    break text.to_string();
                                }
                            }
                        })
                        .await
                        .unwrap();
                        if revision == "2026-07-28" {
                            assert!(
                                notification.contains("io.modelcontextprotocol/subscriptionId")
                            );
                        }
                        match case {
                            Case::C1 => {
                                lab.list(&["control.next.v1"], ttl).await;
                                refused(
                                    &lab.call("files.read.v1", json!({"path":"hello.txt"})).await,
                                );
                                // IFC-012: a new protocol session sees the same closed list.
                                if revision != "2026-07-28" {
                                    let previous = lab.session.take();
                                    lab.initialize().await;
                                    lab.list(&["control.next.v1"], ttl).await;
                                    lab.session = previous;
                                }
                            }
                            Case::C5 => {
                                lab.list(
                                    &[
                                        "control.next.v1",
                                        "files.read.v1",
                                        "files.write.v1",
                                        "notes.read.v1",
                                    ],
                                    ttl,
                                )
                                .await;
                                outcome(
                                    &lab.call("notes.read.v1", json!({"path":"hello.txt"})).await,
                                    "succeeded",
                                );
                            }
                            _ => unreachable!(),
                        }
                        // A second step is stable, with no duplicate transition.
                        lab.call("control.next.v1", json!({})).await;
                        drop(stream);
                    }
                    Case::C2 => refused(&lab.call("never.presented.v1", json!({})).await),
                    Case::C3 => outcome(
                        &lab.call("files.read.v1", json!({"path":"fail.txt"})).await,
                        "failed",
                    ),
                    Case::C4 => outcome(
                        &lab.call(
                            "files.write.v1",
                            json!({"path":"hello.txt","text":"replacement"}),
                        )
                        .await,
                        "unknown",
                    ),
                }
                if !matches!(case, Case::C1 | Case::C5) {
                    lab.call("control.next.v1", json!({})).await;
                    lab.list(&["control.next.v1", "files.read.v1", "files.write.v1"], ttl)
                        .await;
                }
                let log = fs::read_to_string(lab.dir.path().join("stub.jsonl")).unwrap();
                let changes = log
                    .lines()
                    .filter(|line| line.contains("\"kind\":\"list_changed\""))
                    .count();
                assert_eq!(changes, usize::from(matches!(case, Case::C1 | Case::C5)));
                assert!(!log.contains(&lab.token));
                if matches!(case, Case::C1 | Case::C5) {
                    assert!(log.contains("list_changed"));
                    assert!(log.contains("\"streams_open\":1"));
                }
            }
        }
    }
}

#[tokio::test]
async fn ifc_022_042_prv_022_dl13_admission_and_forced_first_401() {
    for revision in stub::REVISIONS {
        let mut lab = Lab::new(Case::C1, revision, 0, true).await;
        let method = if revision == "2026-07-28" {
            "server/discover"
        } else {
            "initialize"
        };
        assert_eq!(
            lab.request(method, json!({}))
                .send()
                .await
                .unwrap()
                .status(),
            401
        );
        lab.initialize().await;
        assert_eq!(
            lab.client
                .get(&lab.server.url)
                .send()
                .await
                .unwrap()
                .status(),
            401
        );
        assert_eq!(
            lab.client
                .get(&lab.server.url)
                .bearer_auth("not-a-jwt")
                .send()
                .await
                .unwrap()
                .status(),
            401
        );
        let altered = format!("{}x", lab.token);
        assert_eq!(
            lab.client
                .get(&lab.server.url)
                .bearer_auth(altered)
                .send()
                .await
                .unwrap()
                .status(),
            401
        );
        lab.list(&["control.next.v1", "files.read.v1", "files.write.v1"], 0)
            .await;
        let helper_log = lab.dir.path().join("helper.jsonl");
        // Execute only our helper: each invocation reads the file afresh, with no agent.
        let run_helper = || {
            std::process::Command::new(env!("CARGO_BIN_EXE_real-client-harness"))
                .args(["--headers-helper", "--identity-dir"])
                .arg(lab.dir.path().join("identity"))
                .arg("--helper-log")
                .arg(&helper_log)
                .output()
                .unwrap()
        };
        let first = run_helper();
        assert!(first.status.success());
        assert_eq!(
            serde_json::from_slice::<Value>(&first.stdout).unwrap()["Authorization"],
            format!("Bearer {}", lab.token)
        );
        fs::write(
            lab.dir
                .path()
                .join("identity")
                .join(Names::default().get("tokenFile")),
            "replacement-for-read-test",
        )
        .unwrap();
        let second = run_helper();
        assert_eq!(
            serde_json::from_slice::<Value>(&second.stdout).unwrap()["Authorization"],
            "Bearer replacement-for-read-test"
        );
        assert_eq!(fs::read_to_string(helper_log).unwrap().lines().count(), 2);
    }
}
#[tokio::test]
async fn ifc_036_invalid_origin_and_undeclared_extension() {
    let mut lab = Lab::new(Case::C2, "2026-07-28", 0, false).await;
    lab.initialize().await;
    assert_eq!(
        lab.request("tools/list", json!({}))
            .header("origin", "https://untrusted.invalid")
            .send()
            .await
            .unwrap()
            .status(),
        403
    );
    let mut request = lab
        .request(
            "tools/call",
            json!({"name":"files.read.v1","arguments":{"path":"hello.txt"}}),
        )
        .build()
        .unwrap();
    let mut body: Value =
        serde_json::from_slice(request.body().unwrap().as_bytes().unwrap()).unwrap();
    body["params"]["_meta"][Names::default().meta("example")] = json!(true);
    *request.body_mut() = Some(serde_json::to_vec(&body).unwrap().into());
    let response = lab.client.execute(request).await.unwrap();
    assert_eq!(response.status(), 400);
    let response = response_json(response).await;
    assert_eq!(response["error"]["code"], -32021);
    assert_eq!(
        response["error"]["data"][Names::default().meta("reason")],
        "invalid"
    );
}

#[tokio::test]
async fn ifc_036_protocol_errors_keep_their_code_and_gain_invalid_reason() {
    let mut lab = Lab::new(Case::C2, "2026-07-28", 0, false).await;
    lab.initialize().await;
    let mut request = lab
        .request(
            "tools/call",
            json!({"name":"files.read.v1","arguments":{"path":"hello.txt"}}),
        )
        .build()
        .unwrap();
    request
        .headers_mut()
        .insert("mcp-name", "different-name".parse().unwrap());
    let response = lab.client.execute(request).await.unwrap();
    assert_eq!(response.status(), 400);
    let response = response_json(response).await;
    assert_eq!(response["error"]["code"], -32020);
    assert_eq!(
        response["error"]["data"][Names::default().meta("reason")],
        "invalid"
    );
    // IFC-025: schema invalidity is a reason with diagnostics, not an outcome.
    let response = lab.call("files.read.v1", json!({"path":42})).await;
    assert_eq!(
        response["result"]["_meta"][Names::default().meta("reason")],
        "invalid"
    );
    assert_eq!(response["result"]["isError"], true);
    assert_eq!(
        response["result"]["structuredContent"]["diagnostics"][0]["level"],
        "error"
    );
    assert_eq!(
        response["result"]["structuredContent"]["diagnostics"][0]["ruleId"],
        "OPS-033"
    );
    assert!(
        response["result"]["_meta"]
            .get(Names::default().meta("invocation"))
            .is_none()
    );
}

#[tokio::test]
async fn ifc_038_records_a_change_when_no_stream_is_open() {
    let mut lab = Lab::new(Case::C5, "2026-07-28", 60000, false).await;
    lab.initialize().await;
    lab.call("control.next.v1", json!({})).await;
    lab.list(
        &[
            "control.next.v1",
            "files.read.v1",
            "files.write.v1",
            "notes.read.v1",
        ],
        60000,
    )
    .await;
    let log = fs::read_to_string(lab.dir.path().join("stub.jsonl")).unwrap();
    assert!(
        log.lines()
            .any(|line| line.contains("\"kind\":\"list_changed\"")
                && line.contains("\"streams_open\":0"))
    );
}

#[tokio::test]
async fn ops_033_invalid_control_has_no_invocation_state_change_or_notification() {
    for revision in stub::REVISIONS {
        for case in [Case::C1, Case::C5] {
            let mut lab = Lab::new(case, revision, 0, false).await;
            lab.initialize().await;
            let mut stream = lab.stream().await.bytes_stream();
            if revision == "2026-07-28" {
                let ack = stream.next().await.unwrap().unwrap();
                assert!(
                    String::from_utf8_lossy(&ack)
                        .contains("notifications/subscriptions/acknowledged")
                );
            }
            let response = lab
                .call("control.next.v1", json!({"unexpected":true}))
                .await;
            let result = &response["result"];
            let names = Names::default();
            assert_eq!(result["_meta"][names.meta("reason")], "invalid");
            assert_eq!(result["isError"], true);
            assert_eq!(
                result["structuredContent"]["diagnostics"][0]["ruleId"],
                "OPS-033"
            );
            assert!(result["_meta"].get(names.meta("invocation")).is_none());
            assert!(result["_meta"].get(names.meta("outcome")).is_none());
            lab.list(&["control.next.v1", "files.read.v1", "files.write.v1"], 0)
                .await;
            assert!(
                tokio::time::timeout(Duration::from_millis(100), async {
                    while let Some(chunk) = stream.next().await {
                        if String::from_utf8_lossy(&chunk.unwrap())
                            .contains("notifications/tools/list_changed")
                        {
                            return;
                        }
                    }
                    panic!("notification stream closed unexpectedly");
                })
                .await
                .is_err()
            );
            let log = fs::read_to_string(lab.dir.path().join("stub.jsonl")).unwrap();
            assert!(!log.contains("\"kind\":\"list_changed\""));
            assert!(!log.contains("\"kind\":\"notification_delivery\""));
            let response = lab.call("control.next.v1", json!({})).await;
            assert_eq!(
                response["result"]["_meta"][names.meta("invocation")],
                "inv-1"
            );
            match case {
                Case::C1 => lab.list(&["control.next.v1"], 0).await,
                Case::C5 => {
                    lab.list(
                        &[
                            "control.next.v1",
                            "files.read.v1",
                            "files.write.v1",
                            "notes.read.v1",
                        ],
                        0,
                    )
                    .await
                }
                _ => unreachable!(),
            }
        }
    }
}

#[tokio::test]
async fn ifc_036_unsupported_version_negotiation_has_no_conduit_reason() {
    let mut lab = Lab::new(Case::C2, "2026-07-28", 0, false).await;
    let mut request = lab.request("server/discover", json!({})).build().unwrap();
    let mut body: Value =
        serde_json::from_slice(request.body().unwrap().as_bytes().unwrap()).unwrap();
    body["params"]["_meta"]["io.modelcontextprotocol/protocolVersion"] = json!("2099-01-01");
    request
        .headers_mut()
        .insert("mcp-protocol-version", "2099-01-01".parse().unwrap());
    *request.body_mut() = Some(serde_json::to_vec(&body).unwrap().into());
    let response = lab.client.execute(request).await.unwrap();
    assert_eq!(response.status(), 400);
    let response = response_json(response).await;
    assert_eq!(response["error"]["code"], -32022);
    assert_eq!(response["error"]["data"]["requested"], "2099-01-01");
    assert!(
        response["error"]["data"]
            .get(Names::default().meta("reason"))
            .is_none()
    );
    assert!(response.get("result").is_none());
    let log = fs::read_to_string(lab.dir.path().join("stub.jsonl")).unwrap();
    assert!(!log.contains("invalid"));
    // Negotiation can be retried at a supported revision.
    lab.initialize().await;
}

#[tokio::test]
async fn request_log_captures_protocol_header_allowlist_without_authorization() {
    let mut lab = Lab::new(Case::C2, "2026-07-28", 0, false).await;
    lab.initialize().await;
    let response = lab
        .request(
            "tools/call",
            json!({"name":"files.read.v1","arguments":{"path":"hello.txt"}}),
        )
        .header("origin", lab.server.url.trim_end_matches('/'))
        .header("mcp-session-id", "header-evidence-only")
        .header("last-event-id", "last-event")
        .header("x-unlisted", "must-not-be-recorded")
        .send()
        .await
        .unwrap();
    let _ = response.bytes().await.unwrap();
    let log = fs::read_to_string(lab.dir.path().join("stub.jsonl")).unwrap();
    let request = log
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .find(|event| {
            event["kind"] == "request" && event["headers"]["mcp-method"] == json!(["tools/call"])
        })
        .unwrap();
    let headers = &request["headers"];
    for (name, expected) in [
        ("mcp-protocol-version", "2026-07-28"),
        ("mcp-session-id", "header-evidence-only"),
        ("mcp-method", "tools/call"),
        ("mcp-name", "files.read.v1"),
        ("origin", lab.server.url.trim_end_matches('/')),
        ("accept", "application/json, text/event-stream"),
        ("content-type", "application/json"),
        ("last-event-id", "last-event"),
    ] {
        assert_eq!(headers[name], json!([expected]));
    }
    assert_eq!(headers.as_object().unwrap().len(), 8);
    assert!(headers.get("authorization").is_none());
    assert!(!log.contains("must-not-be-recorded"));
    assert!(!log.contains(&lab.token));
}
