//! S2-only Conduit-aware implementation. No onward calls or persistent state.
use chrono::{DateTime, Utc};
use rmcp::{ErrorData, RoleServer, ServerHandler, model::*, service::RequestContext};
use serde_json::{Value, json};
use std::{collections::BTreeMap, net::IpAddr, sync::LazyLock, time::Duration};

pub const BINARY: &str = "scenario-s2-linear";
pub const CONTRACTS: &str = "example.org/conduit/scenario-s2-linear-restrictions";
pub const REQUIREMENT: &str = "example.org/conduit/scenario-s2-linear-restrictions#issue-scope.v1";
pub const EXTENSION: &str = "io.github.shrav-k.conduit/implementation-interface";
pub const PREFIX: &str = "io.github.shrav-k.conduit/";
pub const OUTCOME: &str = "io.github.shrav-k.conduit/outcome";
pub const REASON: &str = "io.github.shrav-k.conduit/reason";
pub const RESTRICTION: &str = "io.github.shrav-k.conduit/restriction";
pub const OPERATIONS: [&str; 3] = ["read-issue.v1", "list-issues.v1", "update-issue.v1"];
pub static SCHEMAS: LazyLock<Value> =
    LazyLock::new(|| serde_json::from_str(include_str!("schemas.json")).unwrap());
static VALIDATORS: LazyLock<BTreeMap<String, jsonschema::Validator>> = LazyLock::new(|| {
    let mut map = BTreeMap::new();
    for name in OPERATIONS {
        for kind in ["inputSchema", "outputSchema"] {
            map.insert(format!("{name}/{kind}"), compile(&SCHEMAS[name][kind]));
        }
    }
    let inputs = &SCHEMAS["issue-scope.v1"]["inputs"];
    for key in ["issues", "deadline"] {
        map.insert(key.into(), compile(&inputs[key]["schema"]));
    }
    // The test clock is judged by the same format checker as the deadline, not by chrono alone.
    map.insert(
        "testClock".into(),
        compile(&serde_json::json!({"type":"string","format":"date-time"})),
    );
    map
});
fn compile(schema: &Value) -> jsonschema::Validator {
    jsonschema::options()
        .should_validate_formats(true)
        .build(schema)
        .expect("bundled schema must compile")
}
pub fn valid(name: &str, kind: &str, value: &Value) -> bool {
    VALIDATORS[&format!("{name}/{kind}")].is_valid(value)
}
fn outcome(state: &str, diagnostic: &str, output: Option<Value>) -> Value {
    let mut value = json!({"content":[{"type":"text","text":diagnostic}],"isError":state != "succeeded","_meta":{OUTCOME:state}});
    if let Some(output) = output {
        value["structuredContent"] = output;
    }
    value
}
fn failed(diagnostic: &str) -> Value {
    outcome("failed", diagnostic, None)
}
fn denied(reason: &str, filter: &Value, diagnostic: &str) -> Value {
    json!({"content":[{"type":"text","text":diagnostic}],"isError":true,"_meta":{REASON:reason,RESTRICTION:filter}})
}
/// A `date-time` as the declared schema format asserts it (RFC 3339 with a `T` or `t` between the
/// date and the time); chrono alone also accepts a space there.
fn date_time(s: &str) -> Option<DateTime<chrono::FixedOffset>> {
    (VALIDATORS["testClock"].is_valid(&Value::String(s.to_owned()))
        && matches!(s.as_bytes().get(10), Some(b'T' | b't')))
    .then(|| DateTime::parse_from_rfc3339(s).ok())
    .flatten()
}

fn contains_secret(value: &Value, secret: &str) -> bool {
    match value {
        Value::String(text) => text.contains(secret),
        Value::Array(items) => items.iter().any(|v| contains_secret(v, secret)),
        Value::Object(map) => map
            .iter()
            .any(|(key, v)| key.contains(secret) || contains_secret(v, secret)),
        _ => false,
    }
}
fn nonempty(value: &Value) -> bool {
    value.as_str().is_some_and(|s| !s.is_empty())
}
fn mark<'a>(meta: &'a Value, key: &str) -> &'a Value {
    &meta[format!("{PREFIX}{key}")]
}

pub struct LinearServer {
    endpoint: String,
    // Kept once per process. Never format this value or errors from HTTP/header parsing.
    key: String,
    client: reqwest::Client,
}
impl LinearServer {
    pub fn new(endpoint: String, key: String) -> Result<Self, &'static str> {
        let url = reqwest::Url::parse(&endpoint).map_err(|_| "invalid service URL")?;
        let loopback = url.host_str().is_some_and(|host| {
            host.trim_matches(['[', ']'])
                .parse::<IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
        });
        if !(url.scheme() == "https" || url.scheme() == "http" && loopback)
            || !url.username().is_empty()
            || url.password().is_some()
            || url.fragment().is_some()
            || url.query().is_some()
            || url.host().is_none()
        {
            return Err(
                "service URL requires HTTPS or literal loopback HTTP, without credentials, query or fragment",
            );
        }
        if key.is_empty() || reqwest::header::HeaderValue::from_str(&key).is_err() {
            return Err("invalid service credential configuration");
        }
        let client = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(2))
            .connect_timeout(Duration::from_secs(2))
            .build()
            .map_err(|_| "HTTP client initialization failed")?;
        Ok(Self {
            endpoint,
            key,
            client,
        })
    }
    fn validate_meta<'a>(&self, name: &str, meta: &'a Value) -> Result<&'a Vec<Value>, Value> {
        for key in ["instance", "operation", "invocation", "caller", "revision"] {
            if !nonempty(mark(meta, key)) {
                return Err(failed("missing or malformed required Conduit metadata"));
            }
        }
        if *mark(meta, "operation") != format!("{CONTRACTS}#{name}") {
            return Err(failed("operation metadata does not match tool"));
        }
        for key in ["conduit", "idempotencyKey"] {
            if meta.get(format!("{PREFIX}{key}")).is_some() && !nonempty(mark(meta, key)) {
                return Err(failed("malformed optional Conduit metadata"));
            }
        }
        let config = mark(meta, "config");
        if !config.is_object()
            || config["baseUrl"] != self.endpoint
            || config["apiKey"]["kind"] != "Resource"
            || !nonempty(&config["apiKey"]["id"])
            || config["apiKey"].as_object().is_none_or(|m| m.len() != 2)
            || config
                .as_object()
                .unwrap()
                .keys()
                .any(|k| !["baseUrl", "apiKey", "testClock"].contains(&k.as_str()))
        {
            return Err(failed("missing or malformed instance configuration"));
        }
        mark(meta, "filters")
            .as_array()
            .ok_or_else(|| failed("missing or malformed filters metadata"))
    }
    async fn execute(
        &self,
        name: &str,
        args: &Value,
        meta: &Value,
        receipt: DateTime<Utc>,
    ) -> Value {
        let filters = match self.validate_meta(name, meta) {
            Ok(filters) => filters,
            Err(error) => return error,
        };
        let clock = match mark(meta, "config").get("testClock") {
            None => receipt,
            Some(value) => match value.as_str().and_then(date_time) {
                Some(instant) => instant.with_timezone(&Utc),
                None => return failed("malformed test clock"),
            },
        };
        if !valid(name, "inputSchema", args) {
            return failed("arguments do not match operation input schema");
        }
        let mut scopes = Vec::new();
        // Do not coalesce equal items. Each received item is independently applied.
        for filter in filters {
            if filter.as_object().is_none_or(|m| m.len() != 2)
                || !nonempty(&filter["requirement"])
                || !filter["arguments"].is_object()
            {
                return failed("malformed filter item");
            }
            if filter["requirement"] != REQUIREMENT {
                return denied("unavailable", filter, "unsupported requirement");
            }
            let inputs = filter["arguments"].as_object().unwrap();
            if inputs
                .keys()
                .any(|k| !["issues", "deadline"].contains(&k.as_str()))
            {
                return denied("unavailable", filter, "unsupported requirement input");
            }
            if !VALIDATORS["issues"].is_valid(&filter["arguments"]["issues"])
                || !VALIDATORS["deadline"].is_valid(&filter["arguments"]["deadline"])
            {
                return failed("malformed issue-scope arguments");
            }
            let Ok(deadline) =
                DateTime::parse_from_rfc3339(filter["arguments"]["deadline"].as_str().unwrap())
            else {
                return failed("malformed issue-scope deadline");
            };
            if clock >= deadline {
                return denied("refused", filter, "issue-scope deadline reached at receipt");
            }
            let issues = filter["arguments"]["issues"].as_array().unwrap();
            if name != "list-issues.v1" && !issues.contains(&args["issue"]) {
                return denied("refused", filter, "issue is outside issue-scope");
            }
            scopes.push(issues);
        }
        let ambiguous = if name == "update-issue.v1" {
            "unknown"
        } else {
            "failed"
        };
        let output = self.service(name, args, &scopes, ambiguous).await;
        match output {
            Ok(output) if valid(name, "outputSchema", &output) => {
                outcome("succeeded", "issue operation completed", Some(output))
            }
            Ok(_) => outcome(
                ambiguous,
                "service response does not match issue output schema",
                None,
            ),
            Err((state, cause)) => outcome(state, cause, None),
        }
    }
    async fn graphql(
        &self,
        query: String,
        variables: Value,
        ambiguous: &'static str,
    ) -> Result<Value, (&'static str, &'static str)> {
        let mut header = reqwest::header::HeaderValue::from_str(&self.key)
            .map_err(|_| ("failed", "invalid service credential configuration"))?;
        header.set_sensitive(true);
        let mut response = self
            .client
            .post(&self.endpoint)
            .header(reqwest::header::AUTHORIZATION, header)
            .json(&json!({"query":query,"variables":variables}))
            .send()
            .await
            .map_err(|_| (ambiguous, "service unreachable or request timed out"))?;
        let status = response.status();
        if status.as_u16() == 401 || status.as_u16() == 403 {
            return Err(("failed", "service rejected authentication or authorization"));
        }
        if !status.is_success() {
            return Err((ambiguous, "service returned unsuccessful HTTP status"));
        }
        let mut body = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| (ambiguous, "service response interrupted or timed out"))?
        {
            if body.len() + chunk.len() > 2 * 1024 * 1024 {
                return Err((ambiguous, "service response exceeds size limit"));
            }
            body.extend_from_slice(&chunk);
        }
        let wire: Value = serde_json::from_slice(&body)
            .map_err(|_| (ambiguous, "service returned malformed JSON"))?;
        if wire
            .get("errors")
            .is_some_and(|e| e.as_array().is_none_or(|a| !a.is_empty()))
        {
            let missing = wire["errors"].as_array().is_some_and(|a| {
                !a.is_empty()
                    && a.iter()
                        .all(|e| e["extensions"]["code"] == "ENTITY_NOT_FOUND")
            });
            return Err((
                if missing { "failed" } else { ambiguous },
                if missing {
                    "issue missing or inaccessible at service"
                } else {
                    "service returned GraphQL errors"
                },
            ));
        }
        Ok(wire["data"].clone())
    }
    async fn service(
        &self,
        name: &str,
        args: &Value,
        scopes: &[&Vec<Value>],
        ambiguous: &'static str,
    ) -> Result<Value, (&'static str, &'static str)> {
        const FIELDS: &str = "identifier title description state { name } updatedAt";
        if name == "list-issues.v1" {
            // Schema validation guarantees a positive integer, including 1.0 and 1e0.
            // Saturate limits larger than the platform can hold; keep native integers exact.
            let limit = args["limit"].as_u64().map_or_else(
                || args["limit"].as_f64().unwrap() as usize,
                |limit| usize::try_from(limit).unwrap_or(usize::MAX),
            );
            let mut issues = BTreeMap::new();
            let mut cursor = Value::Null;
            loop {
                let data = self.graphql(format!("query List($after: String) {{ issues(first: 100, after: $after) {{ nodes {{ {FIELDS} }} pageInfo {{ hasNextPage endCursor }} }} }}"), json!({"after":cursor}), ambiguous).await?;
                let connection = &data["issues"];
                let nodes = connection["nodes"]
                    .as_array()
                    .ok_or((ambiguous, "malformed service issue page"))?;
                for node in nodes {
                    let issue = projection(node).ok_or((ambiguous, "malformed service issue"))?;
                    if scopes
                        .iter()
                        .all(|scope| scope.contains(&issue["identifier"]))
                    {
                        issues.insert(issue["identifier"].as_str().unwrap().to_owned(), issue);
                    }
                }
                let more = connection["pageInfo"]["hasNextPage"]
                    .as_bool()
                    .ok_or((ambiguous, "malformed service pagination"))?;
                if !more {
                    break;
                }
                let next = &connection["pageInfo"]["endCursor"];
                if !nonempty(next) || next == &cursor || nodes.is_empty() {
                    return Err((ambiguous, "service pagination made no progress"));
                }
                cursor = next.clone();
            }
            Ok(json!({"issues":issues.into_values().take(limit).collect::<Vec<_>>()}))
        } else {
            let (query, variables, field) = if name == "read-issue.v1" {
                (
                    format!("query Read($id: String!) {{ issue(id: $id) {{ {FIELDS} }} }}"),
                    json!({"id":args["issue"]}),
                    "issue",
                )
            } else {
                let mut edit = args.clone();
                edit.as_object_mut().unwrap().remove("issue");
                (
                    format!(
                        "mutation Update($id: String!, $input: IssueUpdateInput!) {{ issueUpdate(id: $id, input: $input) {{ success issue {{ {FIELDS} }} }} }}"
                    ),
                    json!({"id":args["issue"],"input":edit}),
                    "issueUpdate",
                )
            };
            let data = self.graphql(query, variables, ambiguous).await?;
            let raw = if field == "issue" {
                &data[field]
            } else {
                if data[field]["success"] != true {
                    return Err((ambiguous, "service did not confirm update"));
                }
                &data[field]["issue"]
            };
            let issue = projection(raw).ok_or((ambiguous, "malformed service issue"))?;
            if issue["identifier"] != args["issue"] {
                return Err((ambiguous, "service returned a different issue"));
            }
            Ok(json!({"issue":issue}))
        }
    }
}
fn projection(raw: &Value) -> Option<Value> {
    for field in ["identifier", "title", "updatedAt"] {
        raw.get(field)?.as_str()?;
    }
    if !raw.get("description")?.is_null() && !raw["description"].is_string() {
        return None;
    }
    let state = raw["state"]["name"].as_str()?;
    Some(
        json!({"identifier":raw["identifier"],"title":raw["title"],"description":raw["description"],"state":state,"updatedAt":raw["updatedAt"]}),
    )
}
impl ServerHandler for LinearServer {
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2026-07-28","serverInfo":{"name":BINARY,"version":env!("CARGO_PKG_VERSION")},"capabilities":{"tools":{},"extensions":{EXTENSION:{"revision":"v0"}}}})).unwrap()
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
        Ok(ListToolsResult::with_all_items(OPERATIONS.iter().map(|name| serde_json::from_value(json!({"name":name,"inputSchema":SCHEMAS[name]["inputSchema"],"outputSchema":SCHEMAS[name]["outputSchema"],"annotations":{"readOnlyHint":*name != "update-issue.v1","idempotentHint":SCHEMAS[name]["idempotentHint"],"openWorldHint":true}})).unwrap()).collect()))
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let receipt = Utc::now();
        let meta = serde_json::to_value(context.meta).unwrap_or(Value::Null);
        if meta["io.modelcontextprotocol/clientCapabilities"]["extensions"][EXTENSION]["revision"]
            != "v0"
        {
            return Err(serde_json::from_value(json!({"code":-32021,"message":"Conduit implementation extension revision v0 is required","data":{"requiredCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}}}})).unwrap());
        }
        if !OPERATIONS.contains(&request.name.as_ref()) {
            return Err(ErrorData::invalid_params("unknown issue operation", None));
        }
        let args = Value::Object(request.arguments.unwrap_or_default());
        let mut wire = if contains_secret(&meta, &self.key) || contains_secret(&args, &self.key) {
            failed("credential detected in call data")
        } else {
            self.execute(&request.name, &args, &meta, receipt).await
        };
        if contains_secret(&wire, &self.key) {
            wire = outcome(
                if request.name == "update-issue.v1" {
                    "unknown"
                } else {
                    "failed"
                },
                "credential detected in service output",
                None,
            );
        }
        Ok(serde_json::from_value::<CallToolResult>(wire)
            .unwrap()
            .into())
    }
}
