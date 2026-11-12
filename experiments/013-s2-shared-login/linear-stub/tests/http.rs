use std::{sync::Arc, time::Duration};

use reqwest::{Client, Response, StatusCode};
use s2_linear_stub::{IssueFixture, Stub, StubConfig, SyntheticToken};
use serde_json::{Value, json};

const FIELDS: &str = "id identifier title description state { name } updatedAt";

struct Harness {
    stub: Stub,
    key: SyntheticToken,
    control: SyntheticToken,
    client: Client,
}

impl Harness {
    async fn new() -> Self {
        let mut first = IssueFixture::new("SYN-1", "Synthetic first");
        first.description = Some("Synthetic mention SYN-2".into());
        let config = StubConfig::new(vec![
            IssueFixture::new("SYN-2", "Synthetic second"),
            first,
            IssueFixture::new("SYN-10", "Synthetic tenth"),
        ]);
        let key = config.api_key.clone();
        let control = config.control_token.clone();
        Self {
            stub: Stub::start(config).await.expect("start loopback stub"),
            key,
            control,
            client: Client::builder()
                .no_proxy()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("HTTP client"),
        }
    }

    async fn send(&self, body: Value) -> Response {
        self.client
            .post(self.stub.graphql_url())
            .header("Authorization", self.key.expose())
            .json(&body)
            .send()
            .await
            .expect("loopback GraphQL response")
    }

    async fn gql(&self, query: &str) -> Value {
        let response = self.send(json!({"query": query})).await;
        assert_eq!(response.status(), StatusCode::OK);
        response.json().await.expect("GraphQL JSON")
    }

    async fn control_response(&self, path: &str, body: Value) -> Response {
        self.client
            .post(format!("{}/{path}", self.stub.control_url()))
            .header("Authorization", self.control.expose())
            .json(&body)
            .send()
            .await
            .expect("control response")
    }

    async fn control(&self, path: &str, body: Value) -> Value {
        let response = self.control_response(path, body).await;
        assert_eq!(response.status(), StatusCode::OK);
        response.json().await.expect("control JSON")
    }

    async fn count(&self) -> u64 {
        self.client
            .get(format!("{}/count", self.stub.control_url()))
            .header("Authorization", self.control.expose())
            .send()
            .await
            .expect("count response")
            .json::<Value>()
            .await
            .expect("count JSON")["api_requests"]
            .as_u64()
            .expect("count")
    }

    async fn read(&self, id: &str) -> Value {
        self.gql(&format!("{{ issue(id: \"{id}\") {{ {FIELDS} }} }}"))
            .await
    }

    async fn arm(&self, field: &str, identifier: Option<&str>, timeout_ms: u64) -> u64 {
        self.control(
            "barrier/arm",
            json!({
                "match": {"field": field, "identifier": identifier}, "timeout_ms": timeout_ms
            }),
        )
        .await["id"]
            .as_u64()
            .expect("barrier id")
    }

    async fn wait(&self, id: u64) {
        let answer = self
            .control("barrier/wait", json!({"id": id, "timeout_ms": 2000}))
            .await;
        assert_eq!(answer["state"], "reached");
    }
}

#[tokio::test]
async fn read_supports_variables_aliases_fragments_and_operation_selection() {
    let h = Harness::new().await;
    let response = h.send(json!({
        "query": "query Unused { issue(id: \"SYN-2\") { title } } query Read($id: String!) { selected: issue(id: $id) { ...Details } } fragment Details on Issue { id identifier title description state { name } updatedAt }",
        "operationName": "Read", "variables": {"id": "SYN-1"}
    })).await;
    assert_eq!(response.status(), StatusCode::OK);
    let answer: Value = response.json().await.expect("JSON");
    assert!(answer.get("errors").is_none());
    assert_eq!(
        answer["data"]["selected"],
        json!({
            "id": "stub:SYN-1", "identifier": "SYN-1", "title": "Synthetic first",
            "description": "Synthetic mention SYN-2", "state": {"name": "Todo"},
            "updatedAt": "2026-01-01T00:00:00+00:00"
        })
    );
    assert_eq!(
        h.read("stub:SYN-1").await["data"]["issue"],
        answer["data"]["selected"]
    );
    assert!(h.read("syn-1").await.get("errors").is_some());
    assert!(h.read("SYN-999").await.get("errors").is_some());
    h.stub.shutdown().await.expect("shutdown");
}

#[tokio::test]
async fn list_paginates_in_ascii_order_with_nodes_edges_and_page_info() {
    let h = Harness::new().await;
    let first = h.gql("{ issues(first: 2) { nodes { identifier description } edges { cursor node { identifier } } pageInfo { hasNextPage hasPreviousPage startCursor endCursor } } }").await;
    assert!(first.get("errors").is_none());
    let connection = &first["data"]["issues"];
    assert_eq!(connection["nodes"][0]["identifier"], "SYN-1");
    assert_eq!(connection["nodes"][1]["identifier"], "SYN-10");
    assert_eq!(connection["edges"][1]["cursor"], "SYN-10");
    assert_eq!(connection["edges"][1]["node"]["identifier"], "SYN-10");
    assert_eq!(
        connection["pageInfo"],
        json!({
            "hasNextPage": true, "hasPreviousPage": false,
            "startCursor": "SYN-1", "endCursor": "SYN-10"
        })
    );
    let second = h.gql("{ issues(first: 2, after: \"SYN-10\") { nodes { identifier description } pageInfo { hasNextPage hasPreviousPage endCursor } } }").await;
    assert_eq!(
        second["data"]["issues"]["nodes"],
        json!([{"identifier": "SYN-2", "description": null}])
    );
    assert_eq!(second["data"]["issues"]["pageInfo"]["hasNextPage"], false);
    assert_eq!(
        second["data"]["issues"]["pageInfo"]["hasPreviousPage"],
        true
    );
    let empty = h.gql("{ issues(after: \"SYN-999\") { nodes { identifier } pageInfo { startCursor endCursor hasNextPage } } }").await;
    assert_eq!(empty["data"]["issues"]["nodes"], json!([]));
    assert!(empty["data"]["issues"]["pageInfo"]["endCursor"].is_null());
    assert_eq!(
        h.gql("{ issues { nodes { identifier } } }").await["data"]["issues"]["nodes"]
            .as_array()
            .expect("nodes")
            .len(),
        3
    );
    for query in [
        "{ issues(first: 0) { nodes { title } } }",
        "{ issues(first: 251) { nodes { title } } }",
        "{ issues(after: \"bad\") { nodes { title } } }",
    ] {
        assert!(h.gql(query).await.get("errors").is_some());
    }
}

#[tokio::test]
async fn update_preserves_omitted_fields_clears_description_and_advances_time() {
    let h = Harness::new().await;
    let before = h.read("SYN-1").await;
    let changed = h.gql(&format!("mutation {{ issueUpdate(id: \"SYN-1\", input: {{title: \"\"}}) {{ success issue {{ {FIELDS} }} }} }}")).await;
    assert!(changed.get("errors").is_none());
    assert_eq!(changed["data"]["issueUpdate"]["success"], true);
    let issue = &changed["data"]["issueUpdate"]["issue"];
    assert_eq!(issue["title"], "");
    assert_eq!(issue["description"], before["data"]["issue"]["description"]);
    assert_eq!(issue["state"], before["data"]["issue"]["state"]);
    let parse_time = |v: &Value| {
        chrono::DateTime::parse_from_rfc3339(v.as_str().expect("time")).expect("RFC3339")
    };
    assert!(parse_time(&issue["updatedAt"]) > parse_time(&before["data"]["issue"]["updatedAt"]));
    let cleared = h.gql("mutation { issueUpdate(id: \"stub:SYN-1\", input: {description: null}) { success issue { title description } } }").await;
    assert_eq!(
        cleared["data"]["issueUpdate"]["issue"],
        json!({"title": "", "description": null})
    );
    let both = h.gql("mutation Change($input: IssueUpdateInput!) { issueUpdate(id: \"SYN-1\", input: $input) { issue { title description } } }").await;
    assert!(both.get("errors").is_some()); // Required variable is checked by GraphQL.
    let response = h.send(json!({
        "query": "mutation Change($input: IssueUpdateInput!) { issueUpdate(id: \"SYN-1\", input: $input) { issue { title description } } }",
        "variables": {"input": {"title": "Synthetic edited", "description": "SYN-2 is text"}}
    })).await;
    let both: Value = response.json().await.expect("JSON");
    assert_eq!(
        both["data"]["issueUpdate"]["issue"],
        json!({"title": "Synthetic edited", "description": "SYN-2 is text"})
    );
    let before_no_op = h.read("SYN-1").await;
    let no_op = h.gql(&format!("mutation {{ issueUpdate(id: \"SYN-1\", input: {{title: null}}) {{ issue {{ {FIELDS} }} }} }}")).await;
    assert_eq!(
        before_no_op["data"]["issue"],
        no_op["data"]["issueUpdate"]["issue"]
    );
    let empty_edit = h
        .gql(&format!(
            "mutation {{ issueUpdate(id: \"SYN-1\", input: {{}}) {{ issue {{ {FIELDS} }} }} }}"
        ))
        .await;
    assert_eq!(
        before_no_op["data"]["issue"],
        empty_edit["data"]["issueUpdate"]["issue"]
    );
    assert_eq!(
        h.read("SYN-2").await["data"]["issue"]["title"],
        "Synthetic second"
    );
    assert!(
        h.gql(
            "mutation { issueUpdate(id: \"SYN-999\", input: {title: \"Synthetic\"}) { success } }"
        )
        .await
        .get("errors")
        .is_some()
    );
}

#[tokio::test]
async fn service_authorization_is_separate_applied_before_pagination_and_preserved_on_rotation() {
    let h = Harness::new().await;
    h.control(
        "service-authorization",
        json!({"key": h.key.expose(), "issues": ["SYN-2", "SYN-999"]}),
    )
    .await;
    let missing = h.read("SYN-999").await;
    let hidden = h.read("SYN-1").await;
    assert_eq!(
        missing["errors"][0]["message"],
        hidden["errors"][0]["message"]
    );
    let list = h
        .gql("{ issues(first: 1) { nodes { identifier } pageInfo { hasNextPage } } }")
        .await;
    assert_eq!(
        list["data"]["issues"]["nodes"],
        json!([{"identifier": "SYN-2"}])
    );
    assert_eq!(list["data"]["issues"]["pageInfo"]["hasNextPage"], false);
    assert!(h.gql("mutation { issueUpdate(id: \"SYN-1\", input: {title: \"Synthetic forbidden\"}) { success } }").await.get("errors").is_some());
    h.control(
        "service-authorization",
        json!({"key": h.key.expose(), "issues": ["SYN-1"]}),
    )
    .await;
    assert_eq!(
        h.read("SYN-1").await["data"]["issue"]["title"],
        "Synthetic first"
    );
    let new_key = SyntheticToken::generate();
    h.control("rotate-key", json!({"key": new_key.expose()}))
        .await;
    let response = h
        .client
        .post(h.stub.graphql_url())
        .header("Authorization", new_key.expose())
        .json(&json!({"query": "{ issues { nodes { identifier } } }"}))
        .send()
        .await
        .expect("response");
    assert_eq!(
        response.json::<Value>().await.expect("JSON")["data"]["issues"]["nodes"],
        json!([{"identifier": "SYN-1"}])
    );
}

#[tokio::test]
async fn authentication_rotation_and_counts_cover_rejected_and_malformed_requests() {
    let h = Harness::new().await;
    assert_eq!(h.count().await, 0);
    let query = json!({"query": "{ issue(id: \"SYN-1\") { title } }"});
    let missing = h
        .client
        .post(h.stub.graphql_url())
        .json(&query)
        .send()
        .await
        .expect("response");
    assert_eq!(missing.status(), StatusCode::UNAUTHORIZED);
    for token in [SyntheticToken::generate(), h.control.clone()] {
        let response = h
            .client
            .post(h.stub.graphql_url())
            .header("Authorization", token.expose())
            .json(&query)
            .send()
            .await
            .expect("response");
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let text = response.text().await.expect("body");
        assert!(!text.contains(token.expose()));
    }
    let response = h
        .client
        .get(format!("{}/count", h.stub.control_url()))
        .header("Authorization", h.key.expose())
        .send()
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(h.count().await, 3);
    let malformed = h
        .client
        .post(h.stub.graphql_url())
        .header("Authorization", h.key.expose())
        .header("Content-Type", "application/json")
        .body("{")
        .send()
        .await
        .expect("response");
    assert_eq!(malformed.status(), StatusCode::BAD_REQUEST);
    let new_key = SyntheticToken::generate();
    h.control("rotate-key", json!({"key": new_key.expose()}))
        .await;
    assert_eq!(
        h.send(query.clone()).await.status(),
        StatusCode::UNAUTHORIZED
    );
    let response = h
        .client
        .post(h.stub.graphql_url())
        .header("Authorization", new_key.expose())
        .json(&query)
        .send()
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(h.count().await, 6);
    assert_eq!(h.count().await, 6);
}

#[tokio::test]
async fn unsupported_schema_fields_inputs_and_invalid_syntax_are_graphql_errors() {
    let h = Harness::new().await;
    for query in [
        "{ viewer { id } }",
        "{ issue(id: \"SYN-1\") { comments { nodes { body } } } }",
        "mutation { issueUpdate(id: \"SYN-1\", input: {stateId: \"synthetic\"}) { success } }",
        "{ issues(filter: {}) { nodes { title } } }",
        "not GraphQL",
    ] {
        let answer = h.gql(query).await;
        assert!(answer.get("errors").is_some());
    }
    assert_eq!(h.count().await, 5);
}

#[tokio::test]
async fn barrier_matches_resolved_fields_holds_only_one_request_and_releases_effects() {
    let h = Arc::new(Harness::new().await);
    let id = h.arm("issueUpdate", Some("SYN-1"), 4000).await;
    assert_eq!(
        h.control_response("barrier/wait", json!({"id": id, "timeout_ms": 20}))
            .await
            .status(),
        StatusCode::REQUEST_TIMEOUT
    );
    assert_eq!(
        h.control_response("barrier/release", json!({"id": id, "timeout_ms": 20}))
            .await
            .status(),
        StatusCode::REQUEST_TIMEOUT
    );
    assert_eq!(h.count().await, 0);
    let running = h.clone();
    let task = tokio::spawn(async move {
        running.send(json!({
            "query": "mutation Edit($id: String!) { renamed: issueUpdate(id: $id, input: {title: \"Synthetic after barrier\"}) { success issue { title } } }",
            "variables": {"id": "stub:SYN-1"}
        })).await.json::<Value>().await.expect("JSON")
    });
    h.wait(id).await;
    assert!(!task.is_finished());
    assert_eq!(h.count().await, 1);
    assert_eq!(
        h.read("SYN-1").await["data"]["issue"]["title"],
        "Synthetic first"
    );
    // A second matching request is not captured by the one-shot barrier.
    assert_eq!(h.gql("mutation { issueUpdate(id: \"SYN-1\", input: {description: \"Synthetic parallel\"}) { success } }").await["data"]["issueUpdate"]["success"], true);
    assert!(!task.is_finished());
    assert_eq!(
        h.control("barrier/release", json!({"id": id, "timeout_ms": 1000}))
            .await["state"],
        "released"
    );
    assert_eq!(
        task.await.expect("request task")["data"]["renamed"]["issue"]["title"],
        "Synthetic after barrier"
    );
    assert_eq!(
        h.control("barrier/release", json!({"id": id, "timeout_ms": 1000}))
            .await["state"],
        "released"
    );
    assert_eq!(
        h.read("SYN-1").await["data"]["issue"]["description"],
        "Synthetic parallel"
    );
}

#[tokio::test]
async fn unreleased_barrier_times_out_without_a_mutation_effect_and_can_be_rearmed() {
    let h = Arc::new(Harness::new().await);
    let id = h.arm("issueUpdate", None, 300).await;
    let running = h.clone();
    let task = tokio::spawn(async move {
        running.gql("mutation { issueUpdate(id: \"SYN-1\", input: {title: \"Synthetic never applied\"}) { success } }").await
    });
    h.wait(id).await;
    let answer = task.await.expect("request task");
    assert_eq!(
        answer["errors"][0]["extensions"]["code"],
        "STUB_BARRIER_TIMEOUT"
    );
    assert_eq!(
        h.read("SYN-1").await["data"]["issue"]["title"],
        "Synthetic first"
    );
    assert_eq!(
        h.control_response("barrier/release", json!({"id": id, "timeout_ms": 20}))
            .await
            .status(),
        StatusCode::CONFLICT
    );
    let next = h.arm("any", None, 2000).await;
    let running = h.clone();
    let task = tokio::spawn(async move { running.read("SYN-2").await });
    h.wait(next).await;
    h.control("barrier/release", json!({"id": next, "timeout_ms": 1000}))
        .await;
    assert!(task.await.expect("task").get("errors").is_none());
}

#[tokio::test]
async fn barrier_rejects_invalid_timeouts_with_bad_request() {
    let h = Harness::new().await;
    let id = h.arm("any", None, 60000).await;
    for timeout_ms in [-1, 0, 60001] {
        for endpoint in ["barrier/arm", "barrier/wait", "barrier/release"] {
            let body = if endpoint == "barrier/arm" {
                json!({"match": {"field": "any"}, "timeout_ms": timeout_ms})
            } else {
                json!({"id": id, "timeout_ms": timeout_ms})
            };
            assert_eq!(
                h.control_response(endpoint, body).await.status(),
                StatusCode::BAD_REQUEST,
                "{endpoint} with timeout_ms {timeout_ms}"
            );
        }
    }
    assert_eq!(h.count().await, 0);
}

#[tokio::test]
async fn barrier_rejects_conflicts_bad_matchers_and_unknown_ids_and_expires_unreached() {
    let h = Harness::new().await;
    for body in [
        json!({"match": {"field": "viewer"}, "timeout_ms": 100}),
        json!({"match": {"field": "issues", "identifier": "SYN-1"}, "timeout_ms": 100}),
        json!({"match": {"field": "any"}, "timeout_ms": 0}),
        json!({"match": {"field": "any"}, "timeout_ms": 60001}),
    ] {
        assert_eq!(
            h.control_response("barrier/arm", body).await.status(),
            StatusCode::BAD_REQUEST
        );
    }
    let id = h.arm("issue", Some("SYN-1"), 80).await;
    assert_eq!(
        h.control_response(
            "barrier/arm",
            json!({"match": {"field": "any"}, "timeout_ms": 100})
        )
        .await
        .status(),
        StatusCode::CONFLICT
    );
    assert!(h.read("SYN-2").await.get("errors").is_none());
    assert_eq!(
        h.control_response("barrier/wait", json!({"id": id + 1, "timeout_ms": 20}))
            .await
            .status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        h.control_response("barrier/wait", json!({"id": id, "timeout_ms": 1000}))
            .await
            .status(),
        StatusCode::CONFLICT
    );
    assert!(h.read("SYN-1").await.get("errors").is_none());
    let next = h.arm("any", None, 1000).await;
    assert!(next > id);
}

#[tokio::test]
async fn startup_rejects_duplicate_non_synthetic_data_and_non_loopback_binding() {
    let mut config = StubConfig::new(vec![]);
    config.bind = ([0, 0, 0, 0], 0).into();
    assert!(Stub::start(config).await.is_err());
    assert!(
        Stub::start(StubConfig::new(vec![IssueFixture::new(
            "REAL-1",
            "Synthetic"
        )]))
        .await
        .is_err()
    );
    assert!(
        Stub::start(StubConfig::new(vec![
            IssueFixture::new("SYN-1", "Synthetic"),
            IssueFixture::new("SYN-1", "Synthetic")
        ]))
        .await
        .is_err()
    );
    let mut config = StubConfig::new(vec![]);
    config.control_token = config.api_key.clone();
    assert!(Stub::start(config).await.is_err());
}

#[tokio::test]
async fn held_request_uses_current_service_authorization_but_keeps_admitted_key() {
    let h = Arc::new(Harness::new().await);
    let id = h.arm("issue", Some("SYN-1"), 4000).await;
    let running = h.clone();
    let task = tokio::spawn(async move { running.read("SYN-1").await });
    h.wait(id).await;
    h.control(
        "service-authorization",
        json!({"key": h.key.expose(), "issues": []}),
    )
    .await;
    h.control("barrier/release", json!({"id": id, "timeout_ms": 1000}))
        .await;
    assert_eq!(
        task.await.expect("request task")["errors"][0]["extensions"]["code"],
        "ENTITY_NOT_FOUND"
    );

    h.control(
        "service-authorization",
        json!({"key": h.key.expose(), "issues": ["SYN-1"]}),
    )
    .await;
    let id = h.arm("issue", Some("SYN-1"), 4000).await;
    let running = h.clone();
    let task = tokio::spawn(async move { running.read("SYN-1").await });
    h.wait(id).await;
    let next_key = SyntheticToken::generate();
    h.control("rotate-key", json!({"key": next_key.expose()}))
        .await;
    assert_eq!(
        h.send(json!({"query": "{ issue(id: \"SYN-1\") { title } }"}))
            .await
            .status(),
        StatusCode::UNAUTHORIZED
    );
    h.control("barrier/release", json!({"id": id, "timeout_ms": 1000}))
        .await;
    assert_eq!(
        task.await.expect("request task")["data"]["issue"]["title"],
        "Synthetic first"
    );
}

#[tokio::test]
async fn counts_are_per_post_document_including_body_limit_failures() {
    let h = Harness::new().await;
    let answer = h
        .gql("{ a: issue(id: \"SYN-1\") { title } b: issue(id: \"SYN-2\") { title } }")
        .await;
    assert!(answer.get("errors").is_none());
    assert_eq!(h.count().await, 1);
    let response = h
        .client
        .get(h.stub.graphql_url())
        .send()
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(h.count().await, 1);
    let response = h
        .client
        .post(h.stub.graphql_url())
        .header("Authorization", h.key.expose())
        .header("Content-Type", "application/json")
        .body(" ".repeat(2 * 1024 * 1024 + 1))
        .send()
        .await
        .expect("response");
    // The handler deliberately normalizes all JSON extraction errors to HTTP 400.
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(h.count().await, 2);
}
