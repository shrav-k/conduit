//! Loopback MCP adapter. Access, records and outcomes stay in the core.
use crate::{Execution, Executor, Result, RunIdentity, Shell, Ticket};
use axum::{
    Router,
    body::{Body, to_bytes},
    extract::{Request, State},
    http::{StatusCode, request::Parts},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use conduit_core::access::Identity;
use futures_util::StreamExt;
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    model::*,
    service::{RequestContext, SubscriptionContext},
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService,
        session::{SessionId, SessionManager, local::LocalSessionManager},
    },
};
use serde_json::{Value, json};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    path::Path,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU8, Ordering},
    },
};
use tokio::{
    net::TcpListener,
    sync::{broadcast, watch},
};
use tokio_util::sync::CancellationToken;

pub const REVISIONS: [&str; 3] = ["2025-06-18", "2025-11-25", "2026-07-28"];
// The list names the core package's operations that the shell passes to the core.
// It includes temporary stubs that answer unavailable and name the order that
// builds each operation (OPS-005).
pub(crate) const EXECUTABLE_ENGINE_OPERATIONS: [&str; 18] = [
    "install.v1",
    "retire-with-resources.v1",
    "retire-with-agents.v1",
    "retire.v1",
    "unregister.v1",
    "set-up.v1",
    "open.v1",
    "close.v1",
    "register-replacement.v1",
    "register.v1",
    "replace.v1",
    "inspect.v1",
    "purge.v1",
    "reconcile.v1",
    "list.v1",
    "lift.v1",
    "answer.v1",
    "sign-in.v1",
];
// An order that builds one of these moves it to EXECUTABLE_ENGINE_OPERATIONS.
// A test in tests.rs checks both lists against the core package (MAN-012).
#[cfg(test)]
pub(crate) const UNFINISHED_ENGINE_OPERATIONS: [(&str, &str); 1] =
    [("declare-lost.v1", "none yet: ROADMAP.md S3, AUDIT.md A35")];
fn instance(id: &str) -> Value {
    json!({"kind":"instance","id":id})
}
fn reference(value: &Value) -> &Value {
    value.get("id").unwrap_or(value)
}
async fn instance_exists(shell: Arc<Shell>, identity: String) -> Result<bool> {
    tokio::task::spawn_blocking(move || {
        shell
            .state()
            .map(|s| s.objects.iter().any(|o| o["id"] == identity))
    })
    .await
    .map_err(|_| "instance observation task failed".to_string())?
}
fn internal(message: impl Into<String>) -> ErrorData {
    ErrorData::internal_error(message.into(), None)
}

// The publisher's decision names the exact held scope it resolved. Inspect that
// contract, rather than a current package or the implementation's source.
fn held_control(target: &Value, operation: &str, scope: &str, shell: &Shell) -> Result<bool> {
    let environment = target["held"]
        .get("environment")
        .or_else(|| target.get("schema_environment"));
    let env: conduit_core::schemas::Environment = match environment {
        Some(value) => serde_json::from_value(value.clone()).map_err(|e| e.to_string())?,
        None => shell.context.core_environment.clone(),
    };
    let (_, name) = operation
        .split_once('#')
        .ok_or("invalid operation reference")?;
    let held = env.scopes.get(scope).ok_or("missing held scope")?;
    let contract = held.contracts.get(name).ok_or("missing held operation")?;
    Ok(contract["spec"].get("changes").is_some())
}

/// Compare actual publication with the dispatcher inventory from the I3 handoff.
/// This bounds implementation; it never filters the tool list or decides access.
pub fn unfinished(shell: &Shell, consumer: &str) -> Result<BTreeMap<String, String>> {
    // IFC-012/IFC-033: share one lazy object view for this publication inventory.
    let state = shell.state()?;
    let view = conduit_core::access::PublicationView::new(&state, &shell.context);
    let mut found = BTreeMap::new();
    for path in state.objects.iter().filter(|o| {
        o["kind"] == "Conduit" && reference(&o["consumer"]) == consumer && o["open"] == true
    }) {
        let target = state
            .objects
            .iter()
            .find(|o| &o["id"] == reference(&path["target"]))
            .ok_or("missing publication target")?;
        for operation in path["operations"]
            .as_array()
            .ok_or("missing Conduit operations")?
        {
            let op = operation.as_str().ok_or("invalid operation")?;
            let (definition, publication) = conduit_core::schemas::publish(
                &view,
                consumer,
                path["id"].as_str().ok_or("missing path ID")?,
                op,
                &shell.context,
            )?;
            if let Some(name) = definition["tool"]["name"].as_str() {
                let scope = publication.facts["scope"]
                    .as_str()
                    .ok_or("missing publication scope")?;
                let control = held_control(target, op, scope, shell)?;
                let engine = target["resolved_implementations"][op]["source"] == "engine"
                    || op.starts_with(&format!("{}#", shell.context.core_package));
                let implemented = EXECUTABLE_ENGINE_OPERATIONS
                    .iter()
                    .any(|name| op == format!("{}#{name}", shell.context.core_package));
                if control || (engine && !implemented) {
                    found.insert(name.into(), op.into());
                }
            }
        }
    }
    Ok(found)
}
pub fn preflight(shell: &Shell) -> Result<()> {
    for object in shell
        .state()?
        .objects
        .iter()
        .filter(|o| o.get("spec").is_none() && o["kind"] != "Conduit")
    {
        let Some(id) = object["id"].as_str() else {
            continue;
        };
        let limits = unfinished(shell, id)?;
        if !(object["kind"] == "Interface" && object["name"] == "root") && !limits.is_empty() {
            return Err(format!(
                "consumer {} would receive unfinished operations: {}",
                object["name"],
                limits.values().cloned().collect::<Vec<_>>().join(", ")
            ));
        }
    }
    Ok(())
}

pub struct ServingExecutor {
    inner: Arc<dyn Executor>,
    changes: broadcast::Sender<String>,
}
impl ServingExecutor {
    pub fn new(inner: Arc<dyn Executor>) -> Self {
        Self {
            inner,
            changes: broadcast::channel(1024).0,
        }
    }
}
impl Executor for ServingExecutor {
    fn dispatch(&self, v: &Value) -> Result<Execution> {
        self.inner.dispatch(v)
    }
    fn refresh_tools_and_call(&self, v: &Value) -> Result<Execution> {
        self.inner.refresh_tools_and_call(v)
    }
    fn builtin_runner(&self, v: &Value, i: Option<&RunIdentity>) -> Result<Execution> {
        self.inner.builtin_runner(v, i)
    }
    fn provider_call(&self, v: &Value, i: Option<&RunIdentity>) -> Result<Execution> {
        self.inner.provider_call(v, i)
    }
    fn connect(&self, v: &Value) -> Result<Execution> {
        self.inner.connect(v)
    }
    fn cancel(&self, v: &Value) -> Result<Execution> {
        self.inner.cancel(v)
    }
    fn cancel_setup_parts(&self, v: &Value) -> Result<Execution> {
        self.inner.cancel_setup_parts(v)
    }
    fn tools_changed(&self, notifications: &[Value]) -> Result<()> {
        self.inner.tools_changed(notifications)?;
        for change in notifications {
            if let Some(id) = change["consumer"].as_str() {
                let _ = self.changes.send(id.into());
            }
        }
        Ok(())
    }
}
#[derive(Clone)]
struct Authenticated(String);
#[derive(Clone, Default)]
struct Audit(Arc<AtomicU8>);
#[derive(Clone)]
struct ResponseLifetime(CancellationToken);
#[derive(Clone)]
struct SessionBinding(watch::Receiver<Option<SessionId>>);
#[derive(Clone)]
struct Handler {
    shell: Arc<Shell>,
    executor: Arc<ServingExecutor>,
    shutdown: CancellationToken,
    sessions: Arc<Mutex<BTreeMap<String, String>>>,
    session_manager: Arc<LocalSessionManager>,
}
impl Handler {
    fn identity(
        &self,
        context: &RequestContext<RoleServer>,
    ) -> std::result::Result<String, ErrorData> {
        context
            .extensions
            .get::<Parts>()
            .and_then(|p| p.extensions.get::<Authenticated>())
            .map(|a| a.0.clone())
            .ok_or_else(|| internal("missing authenticated request context"))
    }
    fn handled(&self, context: &RequestContext<RoleServer>) {
        if let Some(audit) = context
            .extensions
            .get::<Parts>()
            .and_then(|p| p.extensions.get::<Audit>())
        {
            audit.0.store(1, Ordering::SeqCst);
        }
    }
}
// Cancels if the handler is dropped, including while its blocking begin finishes.
struct Pending {
    shell: Arc<Shell>,
    executor: Arc<ServingExecutor>,
    ticket: Ticket,
}
impl Drop for Pending {
    fn drop(&mut self) {
        if let Some(invocation) = self.ticket.invocation().map(str::to_owned)
            && self.ticket.try_reply().ok().flatten().is_none()
        {
            let shell = self.shell.clone();
            let executor = self.executor.clone();
            tokio::task::spawn_blocking(move || {
                shell.begin(
                    json!({"kind":"cancel","invocation":invocation}),
                    executor.as_ref(),
                )
            });
        }
    }
}
impl ServerHandler for Handler {
    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Owned(
            REVISIONS
                .iter()
                .map(|v| serde_json::from_value(json!(v)).unwrap())
                .collect(),
        )
    }
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2026-07-28","capabilities":{"tools":{"listChanged":true},"extensions":{format!("{}implementation-interface",self.shell.context.meta_prefix):{"revision":"v0"}}},"serverInfo":{"name":"conduit","version":env!("CARGO_PKG_VERSION")}})).unwrap()
    }
    async fn initialize(
        &self,
        request: InitializeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> std::result::Result<InitializeResult, ErrorData> {
        let identity = self.identity(&context)?;
        context.peer.set_peer_info(request.clone());
        let result = self.negotiate_initialize(&request)?;
        if result.protocol_version != ProtocolVersion::V_2026_07_28 {
            // The HTTP adapter learns the new session ID only after initialize
            // returns. Share that binding with this session's notification task.
            let mut binding = context
                .extensions
                .get::<Parts>()
                .and_then(|p| p.extensions.get::<SessionBinding>())
                .ok_or_else(|| internal("missing session binding"))?
                .0
                .clone();
            let manager = self.session_manager.clone();
            let sessions = self.sessions.clone();
            let mut changes = self.executor.changes.subscribe();
            let peer = context.peer;
            let shutdown = self.shutdown.clone();
            let shell = self.shell.clone();
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        _=shutdown.cancelled()=>break,
                        event=changes.recv()=>match event {
                            Ok(id) if id == identity => {
                                let exists = instance_exists(shell.clone(), identity.clone()).await.unwrap_or(false);
                                if !exists {
                                    let session = binding.wait_for(Option::is_some).await.ok().and_then(|value| value.clone());
                                    if let Some(session) = session { let _ = manager.close_session(&session).await; sessions.lock().unwrap().remove(session.as_ref()); }
                                    break;
                                }
                                if peer.notify_tool_list_changed().await.is_err(){break;}
                            }
                            Err(broadcast::error::RecvError::Closed)=>break,
                            Err(broadcast::error::RecvError::Lagged(_))=>{
                                let session = binding.wait_for(Option::is_some).await
                                    .ok().and_then(|value| value.clone());
                                if let Some(session) = session {
                                    let _ = manager.close_session(&session).await;
                                    sessions.lock().unwrap().remove(session.as_ref());
                                }
                                break;
                            },
                            _=>{if peer.is_transport_closed(){break;}}
                        }
                    }
                }
            });
        }
        Ok(result)
    }
    async fn discover(
        &self,
        _: RequestContext<RoleServer>,
    ) -> std::result::Result<DiscoverResult, ErrorData> {
        Ok(DiscoverResult::from_server_info(
            self.supported_protocol_versions().into_owned(),
            self.get_info(),
        )
        .with_cache_scope(CacheScope::Private))
    }
    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> std::result::Result<ListToolsResult, ErrorData> {
        let identity = self.identity(&context)?;
        let shell = self.shell.clone();
        let executor = self.executor.clone();
        let reply = tokio::task::spawn_blocking(move || {
            shell.submit(
                json!({"kind":"list-tools","identity":instance(&identity)}),
                executor.as_ref(),
            )
        })
        .await
        .map_err(|_| internal("list task failed"))?
        .map_err(internal)?;
        serde_json::from_value(reply).map_err(|_| internal("invalid core tool list"))
    }
    fn accepted_subscription_filter(
        &self,
        requested: &SubscriptionFilter,
    ) -> Option<SubscriptionFilter> {
        Some(requested.clone())
    }
    async fn listen(&self, context: SubscriptionContext) -> std::result::Result<(), ErrorData> {
        let identity = self.identity(context.request_context())?;
        let mut changes = self.executor.changes.subscribe();
        loop {
            tokio::select! {
                _=context.cancelled()=>return Ok(()),
                _=self.shutdown.cancelled()=>return Ok(()),
                event=changes.recv()=>match event {
                    Ok(id) if id == identity=> {
                        if !instance_exists(self.shell.clone(), identity.clone()).await.map_err(internal)? { return Ok(()); }
                        context.sink().notify_tool_list_changed().await.map_err(|_|internal("subscription closed"))?;
                    },
                    Err(_)=>return Ok(()),
                    _=>{}
                }
            }
        }
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> std::result::Result<CallToolResponse, ErrorData> {
        let identity = self.identity(&context)?;
        let response_lifetime = context
            .extensions
            .get::<Parts>()
            .and_then(|p| p.extensions.get::<ResponseLifetime>())
            .ok_or_else(|| internal("missing response lifetime"))?
            .0
            .clone();
        let shell = self.shell.clone();
        let consumer = identity.clone();
        let inventory = tokio::task::spawn_blocking(move || unfinished(&shell, &consumer))
            .await
            .map_err(|_| internal("tool inventory task failed"))?
            .map_err(internal)?;
        if let Some(op) = inventory.get(request.name.as_ref()) {
            self.handled(&context);
            return Err(internal(format!("operation {op} is unfinished")));
        }
        let meta = serde_json::to_value(&context.meta).map_err(|_| internal("invalid metadata"))?;
        let prefix = &self.shell.context.meta_prefix;
        let serving = meta
            .get(format!("{prefix}serving"))
            .cloned()
            .unwrap_or(Value::Null);
        let key = meta
            .get(format!("{prefix}idempotencyKey"))
            .cloned()
            .unwrap_or(Value::Null);
        for field in ["serving", "idempotencyKey"] {
            if let Some(value) = meta.get(format!("{prefix}{field}"))
                && value.as_str().is_none_or(str::is_empty)
            {
                record_rejection(
                    self,
                    Some(&identity),
                    &json!({"params":{"name":request.name,"arguments":request.arguments,"_meta":meta}}),
                    false,
                )
                .await
                .map_err(internal)?;
                self.handled(&context);
                return Err(ErrorData::invalid_params(
                    format!("{field} must be a nonempty string"),
                    Some(json!({format!("{prefix}reason"):"invalid"})),
                ));
            }
        }
        let input = json!({"kind":"invoke","request":{"identity":instance(&identity),"tool":request.name,"arguments":request.arguments.unwrap_or_default(),"serving":serving},"key":key});
        let shell = self.shell.clone();
        let executor = self.executor.clone();
        let pending = tokio::task::spawn_blocking(move || {
            let ticket = shell.begin(input, executor.as_ref())?;
            Ok::<_, String>(Pending {
                shell,
                executor,
                ticket,
            })
        })
        .await
        .map_err(|_| internal("invoke task failed"))?
        .map_err(internal)?;
        self.handled(&context);
        loop {
            if let Some(reply) = pending.ticket.try_reply().map_err(internal)? {
                if let Some(error) = reply.get("error") {
                    return Err(serde_json::from_value(error.clone())
                        .map_err(|_| internal("invalid core protocol error"))?);
                }
                return serde_json::from_value::<CallToolResult>(reply)
                    .map(Into::into)
                    .map_err(|_| internal("invalid core MCP result"));
            }
            tokio::select! {
                _=async { tokio::select! { _=context.ct.cancelled()=>{}, _=response_lifetime.cancelled()=>{}, _=self.shutdown.cancelled()=>{} } }=> {
                    if let Some(invocation)=pending.ticket.invocation() {
                        let shell=self.shell.clone(); let executor=self.executor.clone();let invocation=invocation.to_owned();
                        tokio::task::spawn_blocking(move ||shell.begin(json!({"kind":"cancel","invocation":invocation}),executor.as_ref())).await.map_err(|_|internal("cancel task failed"))?.map_err(internal)?;
                    }
                    return Err(internal("request cancelled"));
                }
                _=tokio::time::sleep(std::time::Duration::from_millis(10))=>{}
            }
        }
    }
}
async fn record_rejection(
    handler: &Handler,
    identity: Option<&str>,
    body: &Value,
    readable: bool,
) -> Result<()> {
    let shell = handler.shell.clone();
    let executor = handler.executor.clone();
    let identity = identity.map(instance).unwrap_or(json!({"kind":"none"}));
    let meta = &body["params"]["_meta"];
    let valid_metadata = |field| {
        meta.get(format!("{}{field}", shell.context.meta_prefix))
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
    };
    let input = json!({"kind":"invoke","request":{"identity":identity,"tool":body["params"]["name"].as_str().unwrap_or(""),"arguments":body["params"].get("arguments").cloned().unwrap_or(json!({})),"readable":readable,"serving":valid_metadata("serving")},"key":valid_metadata("idempotencyKey")});
    tokio::task::spawn_blocking(move || shell.submit(input, executor.as_ref()))
        .await
        .map_err(|_| "rejection task failed")??;
    Ok(())
}
fn unauthorized() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        [("www-authenticate", "Bearer error=\"invalid_token\"")],
    )
        .into_response()
}
async fn authenticate(State(handler): State<Handler>, request: Request, next: Next) -> Response {
    let (mut parts, body) = request.into_parts();
    let token = parts
        .headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));
    // Verification reads the committed state under the writer mutex. Waiting for
    // an in-flight commit must not block an async worker (including initialize).
    let token = token.map(str::to_owned);
    let shell = handler.shell.clone();
    let verified = tokio::task::spawn_blocking(move || shell.verify_token(token.as_deref())).await;
    let identity = match verified {
        Ok(Ok(Identity::Instance { id })) => Some(id),
        Ok(Ok(_)) => None,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let bytes = match to_bytes(body, 4 * 1024 * 1024).await {
        Ok(b) => b,
        Err(_) => {
            if record_rejection(&handler, identity.as_deref(), &Value::Null, false)
                .await
                .is_err()
            {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            return StatusCode::PAYLOAD_TOO_LARGE.into_response();
        }
    };
    let value: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    let origin = url::Url::parse(&handler.shell.config.engine_url)
        .unwrap()
        .origin()
        .ascii_serialization();
    if parts
        .headers
        .get_all("origin")
        .iter()
        .any(|v| v != origin.as_str())
    {
        if record_rejection(&handler, identity.as_deref(), &value, false)
            .await
            .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        return StatusCode::FORBIDDEN.into_response();
    }
    let Some(identity) = identity else {
        if value["method"] == "tools/call"
            && !matches!(
                serde_json::from_slice::<ClientJsonRpcMessage>(&bytes),
                Ok(ClientJsonRpcMessage::Request(JsonRpcRequest {
                    request: ClientRequest::CallToolRequest(_),
                    ..
                }))
            )
        {
            // IFC-036: rmcp owns malformed-protocol responses. With no established
            // identity there is no request record, and no tool handler can be reached.
            return next
                .run(Request::from_parts(parts, Body::from(bytes)))
                .await;
        }
        if value["method"] == "tools/call"
            && record_rejection(&handler, None, &value, true)
                .await
                .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        return unauthorized();
    };
    if let Some(session) = parts
        .headers
        .get("mcp-session-id")
        .and_then(|v| v.to_str().ok())
    {
        let matches = handler
            .sessions
            .lock()
            .unwrap()
            .get(session)
            .is_none_or(|owner| owner == &identity);
        if !matches {
            if value["method"] == "tools/call"
                && record_rejection(&handler, Some(&identity), &value, false)
                    .await
                    .is_err()
            {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            return unauthorized();
        }
    }
    let audit = Audit::default();
    let response_lifetime = CancellationToken::new();
    let response_guard = response_lifetime.clone().drop_guard();
    let (session_binding, session_receiver) = watch::channel(None);
    parts.extensions.insert(SessionBinding(session_receiver));
    parts.extensions.insert(ResponseLifetime(response_lifetime));
    parts.extensions.insert(Authenticated(identity.clone()));
    parts.extensions.insert(audit.clone());
    let prefix = &handler.shell.context.meta_prefix;
    let meta = &value["params"]["_meta"];
    let carries = meta
        .as_object()
        .is_some_and(|m| m.keys().any(|k| k.starts_with(prefix)));
    let declares = meta["io.modelcontextprotocol/clientCapabilities"]["extensions"]
        [format!("{prefix}implementation-interface")]["revision"]
        == "v0";
    if carries && !declares {
        if record_rejection(&handler, Some(&identity), &value, false)
            .await
            .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        return (StatusCode::BAD_REQUEST,axum::Json(json!({"jsonrpc":"2.0","id":value["id"],"error":{"code":-32021,"message":"Missing required client capability","data":{"requiredCapabilities":{"extensions":{format!("{prefix}implementation-interface"): {"revision":"v0"}}},format!("{prefix}reason"):"invalid"}}}))).into_response();
    }
    let response = next
        .run(Request::from_parts(parts, Body::from(bytes)))
        .await;
    if let Some(session) = response
        .headers()
        .get("mcp-session-id")
        .and_then(|v| v.to_str().ok())
    {
        handler
            .sessions
            .lock()
            .unwrap()
            .insert(session.into(), identity.clone());
        let _ = session_binding.send(Some(session.to_owned().into()));
    }
    let response = if !response.status().is_success() {
        let (parts, body) = response.into_parts();
        let bytes = match to_bytes(body, 4 * 1024 * 1024).await {
            Ok(b) => b,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
        let error: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        if error["error"]["code"] != json!(ErrorCode::UNSUPPORTED_PROTOCOL_VERSION)
            && audit.0.swap(2, Ordering::SeqCst) == 0
            && record_rejection(&handler, Some(&identity), &value, false)
                .await
                .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        Response::from_parts(parts, Body::from(bytes))
    } else {
        response
    };
    let (mut parts, body) = response.into_parts();
    parts.headers.remove("content-length");
    let stream = body.into_data_stream();
    // rmcp emits each JSON reply or SSE event as a frame (as in experiment 008).
    let output = futures_util::stream::unfold(
        (stream, handler, identity, audit, value, response_guard),
        |(mut stream, handler, identity, audit, request, response_guard)| async move {
            let chunk = stream.next().await?;
            let chunk = match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    let data_line = text.lines().find(|line| line.starts_with("data:"));
                    let data = data_line
                        .and_then(|line| line.strip_prefix("data:"))
                        .map(str::trim)
                        .unwrap_or(text.as_ref());
                    let mut value: Value = serde_json::from_str(data).unwrap_or(Value::Null);
                    if value.get("error").is_some()
                        && value["error"]["code"] != json!(ErrorCode::UNSUPPORTED_PROTOCOL_VERSION)
                        && audit.0.load(Ordering::SeqCst) != 1
                    {
                        if audit.0.swap(2, Ordering::SeqCst) == 0
                            && record_rejection(&handler, Some(&identity), &request, false)
                                .await
                                .is_err()
                        {
                            return None;
                        }
                        if !value["error"]["data"].is_object() {
                            let previous = value["error"]["data"].clone();
                            value["error"]["data"] = if previous.is_null() {
                                json!({})
                            } else {
                                json!({"detail":previous})
                            };
                        }
                        value["error"]["data"]
                            [format!("{}reason", handler.shell.context.meta_prefix)] =
                            json!("invalid");
                        if data_line.is_some() {
                            Ok(text
                                .split_inclusive('\n')
                                .map(|line| {
                                    if line.starts_with("data:") {
                                        format!("data: {value}\n")
                                    } else {
                                        line.into()
                                    }
                                })
                                .collect::<String>()
                                .into())
                        } else {
                            Ok(value.to_string().into())
                        }
                    } else {
                        Ok(bytes)
                    }
                }
                Err(e) => Err(e),
            };
            Some((
                chunk,
                (stream, handler, identity, audit, request, response_guard),
            ))
        },
    );
    Response::from_parts(parts, Body::from_stream(output))
}

fn session_manager(config: &crate::Config) -> LocalSessionManager {
    let mut manager = LocalSessionManager::default();
    // rmcp counts messages and HTTP session events as activity, not a running
    // handler. Leave time for the call's deadline and its reply before expiry.
    // If no longer Duration is representable, disable the idle limit.
    manager.session_config.keep_alive = config
        .call_timeout
        .checked_add(std::time::Duration::from_secs(60));
    manager
}

/// Binds only the fixed loopback endpoint. The caller owns executor lifetime/setup.
pub async fn serve(
    shell: Arc<Shell>,
    executor: Arc<ServingExecutor>,
    shutdown: CancellationToken,
) -> Result<()> {
    preflight(&shell)?;
    let url = url::Url::parse(&shell.config.engine_url).map_err(|_| "invalid engine address")?;
    if url.path() != "/mcp" || url.query().is_some() {
        return Err("engine address must have path /mcp and no query".into());
    }
    let ip = match url.host() {
        Some(url::Host::Ipv4(ip)) if ip.is_loopback() => std::net::IpAddr::V4(ip),
        Some(url::Host::Ipv6(ip)) if ip.is_loopback() => std::net::IpAddr::V6(ip),
        Some(url::Host::Domain("localhost")) => std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
        _ => return Err("engine must bind loopback".into()),
    };
    let port = url.port_or_known_default().ok_or("engine needs port")?;
    if port == 0 {
        return Err("engine needs a fixed nonzero port".into());
    }
    let listener = TcpListener::bind((ip, port))
        .await
        .map_err(|e| e.to_string())?;
    let handler = Handler {
        shell: shell.clone(),
        executor: executor.clone(),
        shutdown: shutdown.clone(),
        sessions: Arc::default(),
        session_manager: Arc::new(session_manager(&shell.config)),
    };
    let mut config = StreamableHttpServerConfig::default();
    config.cancellation_token = shutdown.clone();
    let service = StreamableHttpService::new(
        {
            let handler = handler.clone();
            move || Ok(handler.clone())
        },
        handler.session_manager.clone(),
        config,
    );
    let router = Router::new()
        .route_service("/mcp", service)
        .layer(middleware::from_fn_with_state(handler, authenticate));
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown.cancelled_owned())
        .await
        .map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || shell.submit(json!({"kind":"recover"}), executor.as_ref()))
        .await
        .map_err(|_| "shutdown recovery failed")??;
    Ok(())
}

#[cfg(test)]
mod session_tests;

/// Store owner helper: no lease, recovery, direct invoke or cached permit.
pub async fn client(
    directory: &Path,
    as_instance: Option<&str>,
    tool: Option<&str>,
    arguments: Value,
) -> Result<Value> {
    use rmcp::{
        ServiceExt,
        transport::{
            StreamableHttpClientTransport,
            streamable_http_client::StreamableHttpClientTransportConfig,
        },
    };
    let config = crate::Config::for_store(directory)?;
    let bytes = crate::store::private_read(&directory.join("state.json"))?;
    let mut decoder = serde_json::Deserializer::from_slice(&bytes);
    decoder.disable_recursion_limit();
    let state: Value =
        serde::Deserialize::deserialize(&mut decoder).map_err(|_| "invalid store")?;
    let asked = as_instance.unwrap_or("root");
    let (kind, name) = asked
        .split_once('/')
        .map_or((None, asked), |(k, n)| (Some(k), n));
    let id = state["state"]["objects"]
        .as_array()
        .ok_or("missing objects")?
        .iter()
        .find(|o| o.get("spec").is_none()
            && ["Agent", "Interface", "Resource", "Pool"].iter().any(|k| o["kind"] == *k)
            && kind.is_none_or(|k| o["kind"] == k)
            && (o["name"] == name || o["id"] == name))
        .and_then(|o| o["id"].as_str())
        .ok_or_else(|| format!("tools for {asked}: instance does not exist; submit its instance document first (use conduit inspect to see registered objects)"))?;
    let token = String::from_utf8(crate::store::private_read(
        &directory.join("client-tokens").join(id),
    ).map_err(|_| format!("tools for {asked}: instance exists but has no presented tools yet (client identity is unavailable); set it up with conduit call through its management Conduit, then open its access Conduits"))?)
    .map_err(|_| "invalid client token file")?;
    let mut initialized = None;
    for attempt in 1..=3 {
        let transport = StreamableHttpClientTransport::with_client(
            reqwest::Client::builder()
                .retry(reqwest::retry::never())
                .build()
                .map_err(|e| e.to_string())?,
            StreamableHttpClientTransportConfig::with_uri(config.engine_url.clone())
                .auth_header(token.clone())
                .reinit_on_expired_session(false),
        );
        let started = std::time::Instant::now();
        let error =
            match tokio::time::timeout(std::time::Duration::from_secs(10), ().serve(transport))
                .await
            {
                Ok(Ok(service)) => {
                    initialized = Some(service);
                    break;
                }
                Ok(Err(error)) => format!("{error}"),
                Err(_) => "timed out".into(),
            };
        let message = format!(
            "engine session initialization at {} failed after {:.3} seconds (attempt {attempt}/3): {error}; run conduit serve --store {} and check its diagnostics",
            config.engine_url,
            started.elapsed().as_secs_f64(),
            directory.display()
        );
        if attempt == 3 {
            return Err(message);
        }
        eprintln!("{message}; retrying initialization only");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    let service = initialized.ok_or("engine initialization did not complete")?;
    let response: Result<Value> = async {
        if let Some(tool) = tool {
            let request = serde_json::from_value(json!({"name":tool,"arguments":arguments}))
                .map_err(|_| "arguments must be a JSON object")?;
            serde_json::to_value(
                service
                    .call_tool(request)
                    .await
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())
        } else {
            serde_json::to_value(service.list_tools(None).await.map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())
        }
    }
    .await;
    let shutdown = service
        .cancel()
        .await
        .map_err(|_| "client shutdown failed".to_string());
    // Preserve the request failure if cleanup also fails, and describe cleanup
    // failures through the same context when the request succeeded.
    let response = response.and_then(|value| shutdown.map(|_| value));
    if tool.is_none()
        && response
            .as_ref()
            .is_ok_and(|r| r["tools"].as_array().is_some_and(Vec::is_empty))
    {
        eprintln!(
            "tools for {asked}: instance exists but has no presented tools yet; set up its targets and open its Conduits, then run tools again"
        );
    }
    response.map_err(|error| format!("{} for {asked} failed: {error}; check conduit tools and the serving engine's diagnostics; this request was not retried", tool.unwrap_or("tools/list")))
}
