use chrono::{DateTime, Timelike, Utc};
use reqwest::{Client, Url, header::HeaderValue};
use rmcp::{ErrorData, RoleServer, ServerHandler, model::*, service::RequestContext};
use serde_json::{Value, json};
use std::{collections::BTreeMap, sync::LazyLock, time::Duration};

pub mod redactor;
pub const BINARY: &str = "scenario-s2-linear-adapter";
pub const PREFIX: &str = "io.github.shrav-k.conduit/";
pub const EXTENSION: &str = "io.github.shrav-k.conduit/implementation-interface";
pub const CONTRACTS: &str = "example.org/conduit/scenario-s2-linear-restrictions";
pub const BASE_VARIABLE: &str = "S2_LINEAR_ADAPTER_BASE_URL";
pub const KEY_VARIABLE: &str = "S2_LINEAR_ADAPTER_API_KEY";
pub const OPERATIONS: [&str; 2] = ["read-issue.v1", "list-issues.v1"];
pub static SCHEMAS: LazyLock<Value> =
    LazyLock::new(|| serde_json::from_str(include_str!("operations.json")).unwrap());
const READ: &str = "query Read($id: String!) { issue(id: $id) { identifier title description state { name } updatedAt } }";
const LIST: &str = "query List($after: String) { issues(first: 100, after: $after) { nodes { identifier title description state { name } updatedAt } pageInfo { hasNextPage endCursor } } }";

pub fn valid(schema: &Value, value: &Value) -> bool {
    jsonschema::options()
        .should_validate_formats(true)
        .build(schema)
        .expect("embedded schema")
        .is_valid(value)
}

// Compare arbitrary fractional precision without rounding through nanoseconds.
// Trim trailing zeroes so equal fractions have one spelling and sort as strings.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    seconds: i64,
    leap_second: bool,
    fraction: String,
}
impl From<DateTime<Utc>> for Instant {
    fn from(time: DateTime<Utc>) -> Self {
        Self {
            seconds: time.timestamp(),
            leap_second: time.nanosecond() >= 1_000_000_000,
            fraction: format!("{:09}", time.nanosecond() % 1_000_000_000)
                .trim_end_matches('0')
                .to_owned(),
        }
    }
}

// Chrono also parses strings outside RFC 3339's grammar. Check the grammar first.
pub fn instant(value: &Value) -> Option<Instant> {
    static GRAMMAR: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r"\A[0-9]{4}-[0-9]{2}-[0-9]{2}[Tt][0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]+)?([Zz]|[+-][0-9]{2}:[0-9]{2})\z").unwrap()
    });
    let text = value.as_str()?;
    let captures = GRAMMAR.captures(text)?;
    if !valid(&json!({"format":"date-time"}), value) {
        return None;
    }
    let mut instant = Instant::from(DateTime::parse_from_rfc3339(text).ok()?.with_timezone(&Utc));
    instant.fraction = captures
        .get(1)
        .map_or("", |m| &m.as_str()[1..])
        .trim_end_matches('0')
        .to_owned();
    Some(instant)
}

fn meta(value: &Value, name: &str) -> Value {
    value[format!("{PREFIX}{name}")].clone()
}
fn nonempty(value: &Value) -> bool {
    value.as_str().is_some_and(|v| !v.is_empty())
}
fn result(outcome: &str, diagnostic: &str, output: Option<Value>) -> CallToolResponse {
    let mut answer: CallToolResult = serde_json::from_value(json!({
        "_meta":{format!("{PREFIX}outcome"):outcome}, "isError":outcome != "succeeded",
        "content":[{"type":"text","text":diagnostic}]
    }))
    .unwrap();
    answer.structured_content = output;
    answer.into()
}
fn failed(cause: &str) -> CallToolResponse {
    result("failed", cause, None)
}
fn denial(reason: &str, filter: &Value) -> CallToolResponse {
    let answer: CallToolResult = serde_json::from_value(json!({
        "_meta":{format!("{PREFIX}reason"):reason, format!("{PREFIX}restriction"):filter},
        "isError":true, "content":[{"type":"text","text":if reason == "refused" {
            "Issue scope or deadline forbids this call"
        } else { "Requirement or input is unsupported" }}]
    }))
    .unwrap();
    answer.into()
}

// No Debug implementation: neither transport errors nor credentials are diagnostics.
pub struct Adapter {
    endpoint: String,
    key: String,
    client: Client,
}
impl Adapter {
    pub fn new(endpoint: String, key: String) -> Result<Self, &'static str> {
        let client = Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .connect_timeout(Duration::from_secs(2))
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|_| "HTTP client initialization failed")?;
        Ok(Self {
            endpoint,
            key,
            client,
        })
    }
    fn safe_endpoint(&self) -> Option<Url> {
        let url = Url::parse(&self.endpoint).ok()?;
        let local = url
            .host_str()?
            .trim_matches(['[', ']'])
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback());
        if !(url.scheme() == "https" || (url.scheme() == "http" && local))
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return None;
        }
        Some(url)
    }
    async fn graphql(&self, query: &str, variables: Value) -> Result<Value, &'static str> {
        let url = self.safe_endpoint().ok_or("Unsafe service URL")?;
        let mut auth =
            HeaderValue::from_str(&self.key).map_err(|_| "Invalid service credential")?;
        if self.key.is_empty() {
            return Err("Missing service credential");
        }
        auth.set_sensitive(true);
        let mut response = self
            .client
            .post(url)
            .header("Authorization", auth)
            .json(&json!({"query":query,"variables":variables}))
            .send()
            .await
            .map_err(|_| "Service unreachable or request timed out")?;
        if !response.status().is_success() {
            return Err(if matches!(response.status().as_u16(), 401 | 403) {
                "Service authorization failed"
            } else {
                "Service HTTP failure"
            });
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| "Service response interrupted")?
        {
            if bytes.len() + chunk.len() > 2 * 1024 * 1024 {
                return Err("Service response too large");
            }
            bytes.extend_from_slice(&chunk);
        }
        let body: Value = serde_json::from_slice(&bytes).map_err(|_| "Invalid service response")?;
        if body.get("errors").is_some() {
            return Err("Service GraphQL lookup failed");
        }
        body.get("data").cloned().ok_or("Missing service data")
    }
    async fn read(&self, issue: &str) -> Result<Value, &'static str> {
        let data = self.graphql(READ, json!({"id":issue})).await?;
        let projected = project(&data["issue"]);
        if projected["identifier"] != issue {
            return Err("Service returned a different issue");
        }
        Ok(json!({"issue":projected}))
    }
    async fn list(&self, scopes: &[Vec<String>], limit: f64) -> Result<Value, &'static str> {
        let mut issues = BTreeMap::new();
        let mut after = Value::Null;
        let mut cursors = std::collections::BTreeSet::new();
        loop {
            let data = self.graphql(LIST, json!({"after":after})).await?;
            let page = &data["issues"];
            for issue in page["nodes"].as_array().ok_or("Invalid issue page")? {
                let id = issue["identifier"]
                    .as_str()
                    .ok_or("Invalid issue identifier")?;
                if scopes.iter().all(|scope| scope.iter().any(|s| s == id)) {
                    issues.insert(id.to_owned(), project(issue));
                }
            }
            match page["pageInfo"]["hasNextPage"].as_bool() {
                Some(false) => break,
                Some(true) => {
                    let next = &page["pageInfo"]["endCursor"];
                    if !nonempty(next) || !cursors.insert(next.as_str().unwrap().to_owned()) {
                        return Err("Invalid service pagination");
                    }
                    after = next.clone();
                }
                None => return Err("Invalid service pagination"),
            }
        }
        let output: Vec<_> = issues
            .into_values()
            .enumerate()
            .take_while(|(i, _)| (*i as f64) < limit)
            .map(|(_, issue)| issue)
            .collect();
        Ok(json!({"issues":output}))
    }
    async fn execute(
        &self,
        name: &str,
        args: &Value,
        wire: &Value,
        receipt: DateTime<Utc>,
    ) -> CallToolResponse {
        if !valid(&SCHEMAS[name]["inputSchema"], args) {
            return failed("Arguments do not match the operation schema");
        }
        for key in ["instance", "operation", "invocation", "caller", "revision"] {
            if !nonempty(&meta(wire, key)) {
                return failed("Missing or malformed call metadata");
            }
        }
        for key in ["conduit", "idempotencyKey"] {
            if wire.get(format!("{PREFIX}{key}")).is_some() && !nonempty(&meta(wire, key)) {
                return failed("Malformed optional call metadata");
            }
        }
        if meta(wire, "operation") != format!("{CONTRACTS}#{name}") {
            return failed("Operation metadata does not match tool");
        }
        let config = meta(wire, "config");
        let Some(config_map) = config.as_object() else {
            return failed("Missing configuration");
        };
        let secret = &config["apiKey"];
        if config["baseUrl"] != self.endpoint
            || secret["kind"] != "Resource"
            || !nonempty(&secret["id"])
            || secret.as_object().is_none_or(|s| s.len() != 2)
            || config_map
                .keys()
                .any(|k| !["baseUrl", "apiKey", "testClock"].contains(&k.as_str()))
        {
            return failed("Invalid configuration or endpoint mismatch");
        }
        let now = if let Some(clock) = config.get("testClock") {
            let Some(clock) = instant(clock) else {
                return failed("Malformed test clock");
            };
            clock
        } else {
            Instant::from(receipt)
        };
        let filters = meta(wire, "filters");
        let Some(filters) = filters.as_array() else {
            return failed("Missing or malformed filters");
        };
        let mut scopes = Vec::new();
        for filter in filters {
            let Some(item) = filter.as_object() else {
                return failed("Malformed filter item");
            };
            if item.len() != 2
                || !nonempty(&filter["requirement"])
                || !filter["arguments"].is_object()
            {
                return failed("Malformed filter item");
            }
            if filter["requirement"] != format!("{CONTRACTS}#issue-scope.v1") {
                return denial("unavailable", filter);
            }
            let inputs = filter["arguments"].as_object().unwrap();
            if inputs
                .keys()
                .any(|k| !["issues", "deadline"].contains(&k.as_str()))
            {
                return denial("unavailable", filter);
            }
            if !valid(
                &json!({"type":"array","minItems":1,"uniqueItems":true,"items":{
                    "type":"string","pattern":"^[A-Z]+-[0-9]+$","not":{"pattern":"[^A-Z0-9-]"}
                }}),
                &filter["arguments"]["issues"],
            ) {
                return failed("Malformed issue scope");
            }
            let Some(deadline) = instant(&filter["arguments"]["deadline"]) else {
                return failed("Malformed filter deadline");
            };
            let scope: Vec<String> = filter["arguments"]["issues"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s.as_str().unwrap().to_owned())
                .collect();
            if now >= deadline
                || (name == "read-issue.v1" && !scope.iter().any(|id| args["issue"] == *id))
            {
                return denial("refused", filter);
            }
            scopes.push(scope);
        }
        let answer = if name == "read-issue.v1" {
            self.read(args["issue"].as_str().unwrap()).await
        } else {
            self.list(&scopes, args["limit"].as_f64().unwrap()).await
        };
        match answer {
            Ok(output) if valid(&SCHEMAS[name]["outputSchema"], &output) => {
                let encoded = output.to_string();
                let escaped = serde_json::to_string(&self.key).unwrap();
                if encoded.contains(&self.key) || encoded.contains(&escaped[1..escaped.len() - 1]) {
                    failed("Service output contains a credential")
                } else {
                    result("succeeded", "Issue read completed", Some(output))
                }
            }
            Ok(_) => failed("Service output does not match the operation schema"),
            Err(cause) => failed(cause),
        }
    }
}
fn project(issue: &Value) -> Value {
    json!({"identifier":issue["identifier"],"title":issue["title"],
        "description":issue["description"],"state":issue["state"]["name"],"updatedAt":issue["updatedAt"]})
}
impl ServerHandler for Adapter {
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2026-07-28", "serverInfo":{
            "name":BINARY,"version":env!("CARGO_PKG_VERSION")}, "capabilities":{
            "tools":{},"extensions":{EXTENSION:{"revision":"v0"}}}}))
        .unwrap()
    }
    async fn discover(&self, _: RequestContext<RoleServer>) -> Result<DiscoverResult, ErrorData> {
        Ok(DiscoverResult::from_server_info(
            self.supported_protocol_versions().into_owned(),
            self.get_info(),
        )
        .with_cache_scope(CacheScope::Private))
    }
    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult::with_all_items(OPERATIONS.iter().map(|name| {
            serde_json::from_value(json!({"name":name,"description":"Read issues through S2's small Linear adapter",
                "inputSchema":SCHEMAS[name]["inputSchema"],"outputSchema":SCHEMAS[name]["outputSchema"],
                "annotations":{"readOnlyHint":true,"idempotentHint":true,"openWorldHint":true}})).unwrap()
        }).collect()))
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let receipt = Utc::now();
        let wire = serde_json::to_value(context.meta).unwrap();
        if wire["io.modelcontextprotocol/clientCapabilities"]["extensions"][EXTENSION]["revision"]
            != "v0"
        {
            return Err(serde_json::from_value(
                json!({"code":-32021,"message":"Conduit extension v0 is required",
                "data":{"requiredCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}}}}),
            )
            .unwrap());
        }
        if !OPERATIONS.contains(&request.name.as_ref()) {
            return Err(ErrorData::invalid_params("Unknown adapter operation", None));
        }
        Ok(self
            .execute(
                &request.name,
                &Value::Object(request.arguments.unwrap_or_default()),
                &wire,
                receipt,
            )
            .await)
    }
}
