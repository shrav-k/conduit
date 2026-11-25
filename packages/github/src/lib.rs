//! Repository-scoped read and pull-request write fronts for GitHub's stdio MCP server.
use rmcp::{
    ClientHandler, ErrorData, RoleClient, RoleServer, ServerHandler, ServiceExt,
    model::*,
    service::{RequestContext, RunningService},
    transport::async_rw::AsyncRwTransport,
};
use serde_json::{Map, Value, json};
use std::{
    collections::BTreeSet,
    io,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll, ready},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, ReadBuf},
    net::UnixStream,
    process::{Child, ChildStdin, ChildStdout},
    sync::{Mutex, watch},
    time::timeout,
};

mod upstream_lifetime;
mod writes;
#[doc(hidden)]
pub use upstream_lifetime::{SUPERVISE_UPSTREAM, supervise_upstream};

pub const BINARY: &str = "conduit-github";
pub const ADDRESS: &str = "example.org/conduit/github";
pub const TOKEN_VARIABLE: &str = "GITHUB_PERSONAL_ACCESS_TOKEN";
pub const EXTENSION: &str = "io.github.shrav-k.conduit/implementation-interface";
pub const PREFIX: &str = "io.github.shrav-k.conduit/";
pub const OUTCOME: &str = "io.github.shrav-k.conduit/outcome";
pub const REASON: &str = "io.github.shrav-k.conduit/reason";
pub const RESTRICTION: &str = "io.github.shrav-k.conduit/restriction";
pub const MAX_TEXT: usize = 8 * 1024 * 1024;
const MAX_WIRE: usize = 6 * MAX_TEXT + 1024 * 1024;
const TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, Copy)]
pub struct Operation {
    pub name: &'static str,
    pub tool: &'static str,
    pub argument: &'static str,
    pub property: &'static str,
    pub get: bool,
}

pub const OPERATIONS: [Operation; 4] = [
    Operation {
        name: "issue.read.v1",
        tool: "issue_read",
        argument: "number",
        property: "issue_number",
        get: true,
    },
    Operation {
        name: "issue.list.v1",
        tool: "list_issues",
        argument: "limit",
        property: "perPage",
        get: false,
    },
    Operation {
        name: "pull-request.read.v1",
        tool: "pull_request_read",
        argument: "number",
        property: "pullNumber",
        get: true,
    },
    Operation {
        name: "pull-request.list.v1",
        tool: "list_pull_requests",
        argument: "limit",
        property: "perPage",
        get: false,
    },
];

impl Operation {
    fn forwarded_fields(self) -> impl Iterator<Item = &'static str> {
        ["owner", "repo", self.property]
            .into_iter()
            .chain(self.get.then_some("method"))
    }

    pub fn input_schema(self) -> Value {
        let mut number = json!({"type":"integer","minimum":1});
        if !self.get {
            number["maximum"] = json!(100);
        }
        json!({"type":"object","additionalProperties":false,"required":[self.argument],
            "properties":{self.argument:number}})
    }

    fn arguments(
        self,
        arguments: Option<&Map<String, Value>>,
        config: &Value,
        token: &str,
    ) -> Result<Map<String, Value>, &'static str> {
        let (owner, repo) = repository(config).ok_or(
            "configuration requires repository owner/name and a resolved secret Resource reference",
        )?;
        if owner.contains(token) || repo.contains(token) {
            return Err("configured owner or repository contains the active credential");
        }
        let value = arguments
            .filter(|args| args.len() == 1)
            .and_then(|args| args.get(self.argument))
            .filter(|value| {
                value
                    .as_f64()
                    .is_some_and(|n| n >= 1.0 && n.fract() == 0.0 && (self.get || n <= 100.0))
            })
            .ok_or(if self.get {
                "arguments require only an integer number of at least 1"
            } else {
                "arguments require only an integer limit from 1 to 100"
            })?;
        let mut forwarded = Map::from_iter([
            ("owner".into(), json!(owner)),
            ("repo".into(), json!(repo)),
            (self.property.into(), value.clone()),
        ]);
        if self.get {
            forwarded.insert("method".into(), json!("get"));
        }
        Ok(forwarded)
    }
}

pub fn output_schema() -> Value {
    json!({"type":"object","additionalProperties":false,"required":["data"],
        "properties":{"data":{"type":["object","array"]}}})
}

fn repository(config: &Value) -> Option<(&str, &str)> {
    let fields = config.as_object().filter(|fields| fields.len() == 2)?;
    let (owner, repo) = fields.get("repository")?.as_str()?.split_once('/')?;
    let reference = fields
        .get("token")?
        .as_object()
        .filter(|map| map.len() == 2)?;
    if reference.get("kind")? != "Resource"
        || reference.get("id")?.as_str()?.is_empty()
        || !(1..=39).contains(&owner.len())
        || !owner
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        || !(1..=100).contains(&repo.len())
        || matches!(repo, "." | "..")
        || !repo
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
    {
        return None;
    }
    Some((owner, repo))
}

fn redacted(text: &str, token: &str) -> String {
    text.replace(token, "[token]")
}

fn contains_token(value: &Value, token: &str) -> bool {
    match value {
        Value::String(text) => text.contains(token),
        Value::Array(items) => items.iter().any(|item| contains_token(item, token)),
        Value::Object(fields) => fields
            .iter()
            .any(|(key, value)| key.contains(token) || contains_token(value, token)),
        _ => false,
    }
}

fn redact_data(data: &mut Value, token: &str) -> Result<(), String> {
    match data {
        Value::String(text) => *text = redacted(text, token),
        Value::Array(items) => {
            for item in items {
                redact_data(item, token)?;
            }
        }
        Value::Object(fields) => {
            for (key, mut value) in std::mem::take(fields) {
                redact_data(&mut value, token)?;
                if fields.insert(redacted(&key, token), value).is_some() {
                    return Err("server answer has colliding keys after token replacement".into());
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn data(result: CallToolResult, token: &str) -> Result<Value, String> {
    let mut texts = result
        .content
        .iter()
        .filter_map(|item| item.as_text().map(|text| text.text.as_str()));
    if result.is_error == Some(true) {
        let mut message = String::from("server tool reported an error");
        if let Some(text) = texts.next() {
            message.push_str(": ");
            let mut text = redacted(text, token);
            text.truncate(text.floor_char_boundary(2048 - message.len()));
            message.push_str(&text);
        }
        return Err(message);
    }
    if result
        .result_type
        .as_ref()
        .is_some_and(|kind| kind != &ResultType::COMPLETE)
    {
        return Err("server answer is not one text item containing JSON".into());
    }
    let text = texts
        .next()
        .filter(|_| result.content.len() == 1)
        .ok_or("server answer is not one text item containing JSON")?;
    if text.len() > MAX_TEXT {
        return Err("server answer exceeds 8 MiB".into());
    }
    let mut data: Value =
        serde_json::from_str(text).map_err(|_| "server text is not a JSON object or array")?;
    if !(data.is_object() || data.is_array()) {
        return Err("server text is not a JSON object or array".into());
    }
    redact_data(&mut data, token)?;
    Ok(data)
}

fn reply(result: Result<Value, String>) -> CallToolResponse {
    let result = match result {
        Ok(data) => {
            json!({"content":[],"structuredContent":{"data":data},"isError":false,"_meta":{OUTCOME:"succeeded"}})
        }
        Err(message) => {
            json!({"content":[{"type":"text","text":message}],"isError":true,"_meta":{OUTCOME:"failed"}})
        }
    };
    serde_json::from_value::<CallToolResult>(result)
        .unwrap()
        .into()
}

// Bound the wire envelope separately from the decoded text. The flag also
// makes a framing-limit refusal distinguishable from a child that just exited.
struct BoundedOutput {
    pipe: ChildStdout,
    line_len: usize,
    oversized: Arc<AtomicBool>,
}
impl AsyncRead for BoundedOutput {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let start = buf.filled().len();
        ready!(Pin::new(&mut self.pipe).poll_read(cx, buf))?;
        for &byte in &buf.filled()[start..] {
            if byte == b'\n' {
                self.line_len = 0;
            } else {
                self.line_len += 1;
                if self.line_len > MAX_WIRE {
                    self.oversized.store(true, Ordering::Relaxed);
                    return Poll::Ready(Err(io::Error::other("server frame exceeds size bound")));
                }
            }
        }
        Poll::Ready(Ok(()))
    }
}

struct UpstreamClient;
impl ClientHandler for UpstreamClient {}

type UpstreamTransport = AsyncRwTransport<RoleClient, BoundedOutput, ChildStdin>;

struct Upstream {
    supervisor: Child,
    control: UnixStream,
    service: Option<RunningService<RoleClient, UpstreamClient>>,
    oversized: Arc<AtomicBool>,
    compatibility: Result<(), String>,
}

impl Upstream {
    fn spawn(write: bool) -> Result<(Self, UpstreamTransport), &'static str> {
        let (mut supervisor, control) = upstream_lifetime::spawn(write)?;
        let oversized = Arc::new(AtomicBool::new(false));
        let transport = AsyncRwTransport::new_client(
            BoundedOutput {
                pipe: supervisor.stdout.take().unwrap(),
                line_len: 0,
                oversized: oversized.clone(),
            },
            supervisor.stdin.take().unwrap(),
        )
        .with_max_line_length(MAX_WIRE);
        Ok((
            Self {
                supervisor,
                control,
                service: None,
                oversized,
                compatibility: Ok(()),
            },
            transport,
        ))
    }

    fn closed(&mut self) -> bool {
        !matches!(self.supervisor.try_wait(), Ok(None))
            || self
                .service
                .as_ref()
                .is_none_or(|service| service.is_closed() || service.peer().is_transport_closed())
    }

    async fn stop(self) {
        let Self {
            mut supervisor,
            control,
            service,
            ..
        } = self;
        if let Some(service) = service {
            let _ = timeout(Duration::from_millis(200), service.cancel()).await;
        }
        drop(control);
        let _ = supervisor.wait().await;
    }
}

async fn tools_match(
    service: &RunningService<RoleClient, UpstreamClient>,
    token: &str,
    write: bool,
) -> Result<(), String> {
    let info = service.peer_info();
    let version = info
        .as_ref()
        .and_then(|info| info.server_info.as_ref())
        .map_or_else(
            || "unknown".into(),
            |server| redacted(&server.version, token),
        );
    let mut tools = Vec::new();
    let mut seen = BTreeSet::new();
    let mut cursor = None;
    loop {
        let page = service
            .list_tools(
                cursor.map(|cursor| PaginatedRequestParams::default().with_cursor(Some(cursor))),
            )
            .await
            .map_err(|_| "github-mcp-server could not list tools")?;
        tools.extend(page.tools);
        let Some(next) = page.next_cursor else { break };
        if !seen.insert(next.clone()) {
            return Err("github-mcp-server repeated a tools/list cursor".into());
        }
        cursor = Some(next);
    }
    let table: Vec<(&str, Vec<&str>)> = if write {
        writes::TOOLS
            .iter()
            .map(|(name, fields)| (*name, fields.to_vec()))
            .collect()
    } else {
        OPERATIONS
            .iter()
            .map(|op| (op.tool, op.forwarded_fields().collect()))
            .collect()
    };
    for (name, fields) in table {
        let tool = tools
            .iter()
            .find(|tool| tool.name == name)
            .ok_or_else(|| format!("github-mcp-server {version} is missing tool {name}"))?;
        let properties = tool
            .input_schema
            .get("properties")
            .and_then(Value::as_object);
        for property in fields {
            if properties.is_none_or(|properties| !properties.contains_key(property)) {
                return Err(format!(
                    "github-mcp-server {version} tool {name} is missing input property {property}"
                ));
            }
        }
    }
    Ok(())
}

#[derive(Clone)]
pub struct Front {
    write: bool,
    token: Option<Arc<str>>,
    upstream: Arc<Mutex<Option<Upstream>>>,
    stopping: watch::Sender<bool>,
}

impl Default for Front {
    fn default() -> Self {
        Self {
            write: false,
            token: std::env::var(TOKEN_VARIABLE)
                .ok()
                .filter(|value| !value.is_empty())
                .map(Into::into),
            upstream: Arc::default(),
            stopping: watch::Sender::new(false),
        }
    }
}

impl Front {
    pub fn writer() -> Self {
        Self {
            write: true,
            ..Self::default()
        }
    }

    pub async fn shutdown(&self) {
        self.stopping.send_replace(true);
        if let Some(upstream) = self.upstream.lock().await.take() {
            upstream.stop().await;
        }
    }

    async fn dispatch_raw(
        &self,
        token: &str,
        tool: &str,
        arguments: Map<String, Value>,
        sent: Option<&AtomicBool>,
    ) -> Result<CallToolResult, String> {
        let mut slot = self.upstream.lock().await;
        if slot.as_mut().is_some_and(Upstream::closed) {
            slot.take().unwrap().stop().await;
        }
        if slot.is_none() {
            let (upstream, transport) = Upstream::spawn(self.write)?;
            *slot = Some(upstream);
            let started = timeout(
                TIMEOUT,
                upstream_lifetime::start(&mut slot.as_mut().unwrap().control, token),
            )
            .await
            .unwrap_or(Err("github-mcp-server could not be started"));
            if let Err(diagnostic) = started {
                slot.take().unwrap().stop().await;
                return Err(diagnostic.into());
            }
            let Ok(Ok(service)) = timeout(TIMEOUT, UpstreamClient.serve(transport)).await else {
                slot.take().unwrap().stop().await;
                return Err("github-mcp-server could not initialize".into());
            };
            let upstream = slot.as_mut().unwrap();
            upstream.service = Some(service);
            let compatible = timeout(
                TIMEOUT,
                tools_match(upstream.service.as_ref().unwrap(), token, self.write),
            )
            .await;
            if upstream.closed() {
                slot.take().unwrap().stop().await;
                return Err("github-mcp-server exited while listing tools".into());
            }
            upstream.compatibility = compatible
                .unwrap_or_else(|_| Err("github-mcp-server timed out listing tools".into()));
        }
        let upstream = slot.as_mut().unwrap();
        upstream.compatibility.clone()?;
        let request = CallToolRequestParams::new(tool.to_owned()).with_arguments(arguments);
        if let Some(sent) = sent {
            sent.store(true, Ordering::Relaxed);
        }
        match timeout(
            TIMEOUT,
            upstream.service.as_ref().unwrap().call_tool(request),
        )
        .await
        {
            Ok(Ok(result)) => Ok(result),
            _ => {
                let message = if upstream.oversized.load(Ordering::Relaxed) {
                    "server answer exceeds 8 MiB"
                } else if upstream.closed() {
                    "github-mcp-server exited during the call"
                } else {
                    "github-mcp-server did not answer the call"
                };
                slot.take().unwrap().stop().await;
                Err(message.into())
            }
        }
    }

    async fn call(&self, request: CallToolRequestParams, meta: &Value) -> Result<Value, String> {
        let token = self
            .token
            .as_deref()
            .ok_or("GITHUB_PERSONAL_ACCESS_TOKEN is absent or empty")?;
        let operation = OPERATIONS
            .into_iter()
            .find(|op| {
                request.name == op.name
                    && meta[format!("{PREFIX}operation")] == format!("{ADDRESS}#{}", op.name)
            })
            .ok_or("unknown repository operation")?;
        for key in ["instance", "invocation", "caller", "revision", "conduit"] {
            if meta[format!("{PREFIX}{key}")]
                .as_str()
                .is_none_or(str::is_empty)
            {
                return Err("request metadata requires non-empty instance, operation, invocation, caller, revision and conduit".into());
            }
        }
        let arguments = operation.arguments(
            request.arguments.as_ref(),
            &meta[format!("{PREFIX}config")],
            token,
        )?;
        let mut stopping = self.stopping.subscribe();
        if *stopping.borrow() {
            return Err("repository front is stopping".into());
        }
        tokio::select! {
            biased;
            _ = stopping.changed() => Err("repository front is stopping".into()),
            result = self.dispatch_raw(token, operation.tool, arguments, None) => result.and_then(|result| data(result, token)),
        }
    }
}

impl ServerHandler for Front {
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
        if self.write {
            return Ok(ListToolsResult::with_all_items(writes::tools()));
        }
        Ok(ListToolsResult::with_all_items(OPERATIONS.into_iter()
            .map(|op| serde_json::from_value(json!({"name":op.name,"inputSchema":op.input_schema(),"outputSchema":output_schema()})).unwrap())
            .collect()))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let meta = serde_json::to_value(context.meta).unwrap_or(Value::Null);
        if meta["io.modelcontextprotocol/clientCapabilities"]["extensions"][EXTENSION]["revision"]
            != "v0"
        {
            return Err(serde_json::from_value(json!({"code":-32021,"message":"Missing required client capability; performed nothing","data":{"requiredCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}}}})).unwrap());
        }
        let Some(filters) = meta[format!("{PREFIX}filters")].as_array() else {
            if self.write {
                return Ok(serde_json::from_value::<CallToolResult>(self.write_failure(
                    &request,
                    false,
                    "request metadata filters must be a list",
                ))
                .unwrap()
                .into());
            }
            return Ok(reply(Err("request metadata filters must be a list".into())));
        };
        if let Some(restriction) = filters.first() {
            if self
                .token
                .as_deref()
                .is_some_and(|token| filters.iter().any(|filter| contains_token(filter, token)))
            {
                if self.write {
                    return Ok(serde_json::from_value::<CallToolResult>(self.write_failure(
                        &request,
                        false,
                        "requested filter contains the active credential",
                    ))
                    .unwrap()
                    .into());
                }
                return Ok(reply(Err(
                    "requested filter contains the active credential".into(),
                )));
            }
            let message = if self.write {
                "pull-request writer cannot apply the requested filter; performed nothing"
            } else {
                "repository front cannot apply the requested filter; performed nothing"
            };
            return Ok(serde_json::from_value::<CallToolResult>(json!({"content":[{"type":"text","text":message}],"isError":true,"_meta":{REASON:"unavailable",RESTRICTION:restriction}})).unwrap().into());
        }
        if self.write {
            return Ok(self.write_call(request, &meta).await);
        }
        Ok(reply(self.call(request, &meta).await))
    }
}
