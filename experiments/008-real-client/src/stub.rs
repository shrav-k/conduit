use crate::Names;
use anyhow::{Context, Result, anyhow};
use axum::{
    Router,
    body::{Body, to_bytes},
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use futures_util::StreamExt;
use ring::{
    rand::SystemRandom,
    signature::{self, KeyPair},
};
use rmcp::{
    ErrorData, Peer, RoleServer, ServerHandler,
    model::*,
    service::{RequestContext, SubscriptionContext},
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    borrow::Cow,
    fs::{self, File, OpenOptions},
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::Path,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{net::TcpListener, sync::broadcast, task::JoinHandle};
use tokio_util::sync::CancellationToken;

pub const REVISIONS: [&str; 3] = ["2025-06-18", "2025-11-25", "2026-07-28"];

#[derive(Clone, Copy, Debug, Serialize, Deserialize, clap::ValueEnum)]
pub enum Case {
    C1,
    C2,
    C3,
    C4,
    C5,
}
impl Case {
    pub fn name(self) -> &'static str {
        match self {
            Self::C1 => "C1",
            Self::C2 => "C2",
            Self::C3 => "C3",
            Self::C4 => "C4",
            Self::C5 => "C5",
        }
    }
}

#[derive(Clone)]
pub struct Log(Arc<Mutex<File>>);
impl Log {
    pub fn new(path: &Path) -> Result<Self> {
        Ok(Self(Arc::new(Mutex::new(
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .mode(0o600)
                .open(path)?,
        ))))
    }
    pub fn event(&self, value: Value) {
        let mut value = value;
        value["unix_ms"] = json!(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );
        let mut file = self.0.lock().unwrap();
        writeln!(file, "{value}").expect("write evidence");
        file.flush().expect("flush evidence");
    }
}

/// ES256 JWT-SVID. Only the freshly minted experiment credential is ever read.
pub struct Identity {
    token: String,
    public_key: Vec<u8>,
    audience: String,
}
impl Identity {
    fn mint(port: u16) -> Result<Self> {
        let rng = SystemRandom::new();
        let pkcs8 = signature::EcdsaKeyPair::generate_pkcs8(
            &signature::ECDSA_P256_SHA256_FIXED_SIGNING,
            &rng,
        )
        .map_err(|_| anyhow!("generate signing key"))?;
        let key = signature::EcdsaKeyPair::from_pkcs8(
            &signature::ECDSA_P256_SHA256_FIXED_SIGNING,
            pkcs8.as_ref(),
            &rng,
        )
        .map_err(|_| anyhow!("load signing key"))?;
        let audience = format!("https://localhost:{port}/");
        let exp = now() + 24 * 3600;
        let header = B64.encode(serde_json::to_vec(
            &json!({"alg":"ES256","typ":"JWT","kid":"lab"}),
        )?);
        let claims = B64.encode(serde_json::to_vec(&json!({"sub":"spiffe://experiment.test/conduit/instance/consumer-1","aud":[audience],"exp":exp}))?);
        let input = format!("{header}.{claims}");
        let sig = key
            .sign(&rng, input.as_bytes())
            .map_err(|_| anyhow!("sign token"))?;
        Ok(Self {
            token: format!("{input}.{}", B64.encode(sig.as_ref())),
            public_key: key.public_key().as_ref().to_vec(),
            audience,
        })
    }
    fn valid(&self, token: &str) -> bool {
        let Some((input, sig)) = token.rsplit_once('.') else {
            return false;
        };
        let Ok(sig) = B64.decode(sig) else {
            return false;
        };
        if signature::UnparsedPublicKey::new(&signature::ECDSA_P256_SHA256_FIXED, &self.public_key)
            .verify(input.as_bytes(), &sig)
            .is_err()
        {
            return false;
        }
        let Some((header, claims)) = input.split_once('.') else {
            return false;
        };
        let decode = |s: &str| {
            B64.decode(s)
                .ok()
                .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
        };
        let (Some(header), Some(claims)) = (decode(header), decode(claims)) else {
            return false;
        };
        header["alg"] == "ES256"
            && claims["sub"] == "spiffe://experiment.test/conduit/instance/consumer-1"
            && claims["aud"] == json!([self.audience])
            && claims["exp"].as_u64().is_some_and(|exp| exp > now())
    }
    fn write(&self, dir: &Path, url: &str, names: &Names) -> Result<()> {
        fs::create_dir(dir)?;
        let jwks = json!({"keys":[{"kty":"EC","crv":"P-256","alg":"ES256","use":"sig","kid":"lab","x":B64.encode(&self.public_key[1..33]),"y":B64.encode(&self.public_key[33..65])}]});
        for (name, bytes) in [
            (names.get("tokenFile"), self.token.as_bytes().to_vec()),
            (names.get("jwtBundleFile"), serde_json::to_vec(&jwks)?),
            (
                names.get("engineIdFile"),
                b"spiffe://experiment.test/conduit/engine".to_vec(),
            ),
        ] {
            let mut f = OpenOptions::new()
                .create_new(true)
                .write(true)
                .mode(0o600)
                .open(dir.join(name))?;
            f.write_all(&bytes)?;
        }
        // Loopback-only probe: no TLS certificate/root is minted or claimed.
        fs::write(
            dir.join("environment.json"),
            serde_json::to_vec_pretty(
                &json!({names.get("engineUrlVariable"):url,names.get("identityDirVariable"):dir}),
            )?,
        )?;
        Ok(())
    }
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Clone)]
struct Stub(Arc<Shared>);
struct Shared {
    names: Names,
    case: Case,
    ttl: u64,
    advanced: AtomicBool,
    invocation: AtomicU64,
    peers: Mutex<Vec<Peer<RoleServer>>>,
    changes: broadcast::Sender<()>,
    identity: Identity,
    origin: String,
    first_401: AtomicBool,
    log: Log,
    request_id: AtomicU64,
    streams: AtomicUsize,
}
impl Stub {
    fn presented(&self) -> Vec<&'static str> {
        let advanced = self.0.advanced.load(Ordering::SeqCst);
        let mut names = vec!["control.next.v1"];
        if !(matches!(self.0.case, Case::C1) && advanced) {
            names.extend(["files.read.v1", "files.write.v1"]);
        }
        if matches!(self.0.case, Case::C5) && advanced {
            names.push("notes.read.v1");
        }
        names
    }
    fn result(
        &self,
        outcome: Option<&str>,
        reason: Option<&str>,
        outputs: Value,
    ) -> CallToolResult {
        let mut meta = serde_json::Map::new();
        if let Some(outcome) = outcome {
            meta.insert(self.0.names.meta("outcome"), json!(outcome));
            meta.insert(
                self.0.names.meta("invocation"),
                json!(format!(
                    "inv-{}",
                    self.0.invocation.fetch_add(1, Ordering::SeqCst)
                )),
            );
        }
        if let Some(reason) = reason {
            meta.insert(self.0.names.meta("reason"), json!(reason));
        }
        let error = reason.is_some() || outcome != Some("succeeded");
        let text = if error {
            reason.or(outcome).unwrap().to_string()
        } else {
            outputs.to_string()
        };
        let value = json!({"resultType":"complete","content":[{"type":"text","text":text}],"structuredContent":if error {Value::Null} else {outputs},"isError":error,"_meta":meta});
        let mut value = value;
        if error {
            value.as_object_mut().unwrap().remove("structuredContent");
        }
        serde_json::from_value::<CallToolResult>(value).unwrap()
    }
}
impl ServerHandler for Stub {
    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Owned(
            REVISIONS
                .iter()
                .map(|v| serde_json::from_value(json!(v)).unwrap())
                .collect(),
        )
    }
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2026-07-28","capabilities":{"tools":{"listChanged":true},"extensions":{self.0.names.meta("implementation-interface"):{"revision":"v0"}}},"serverInfo":{"name":"experiment-008","version":"0.1.0"}})).unwrap()
    }
    async fn initialize(
        &self,
        request: InitializeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, ErrorData> {
        context.peer.set_peer_info(request.clone());
        let result = self.negotiate_initialize(&request)?;
        self.0.log.event(json!({"kind":"negotiated","offered":request.protocol_version,"revision":result.protocol_version}));
        self.0.peers.lock().unwrap().push(context.peer);
        Ok(result)
    }
    async fn discover(
        &self,
        context: RequestContext<RoleServer>,
    ) -> Result<DiscoverResult, ErrorData> {
        self.0.log.event(
            json!({"kind":"negotiated","revision":context.protocol_version(),"mode":"per-request"}),
        );
        Ok(DiscoverResult::from_server_info(
            self.supported_protocol_versions().into_owned(),
            self.get_info(),
        )
        .with_ttl_ms(self.0.ttl)
        .with_cache_scope(CacheScope::Private))
    }
    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let tools = self.presented().iter().map(|name| {
            let (input, output, description, read_only) = match *name {
                "control.next.v1" => (json!({"type":"object","properties":{},"additionalProperties":false}), json!({"type":"object","properties":{"state":{"type":"integer"}},"required":["state"]}), "Advance this case once; later calls keep the final state.", false),
                "files.write.v1" => (json!({"type":"object","properties":{"path":{"type":"string"},"text":{"type":"string"}},"required":["path","text"],"additionalProperties":false}), json!({"type":"object","properties":{"written":{"type":"boolean"}},"required":["written"]}), "Write a dummy file in the in-memory directory.", false),
                _ => (json!({"type":"object","properties":{"path":{"type":"string"}},"required":["path"],"additionalProperties":false}),json!({"type":"object","properties":{"text":{"type":"string"}},"required":["text"]}), "Read a dummy file from the in-memory directory.", true),
            };
            let mut annotations = json!({"openWorldHint":false});
            if read_only {
                annotations["readOnlyHint"] = json!(true);
            }
            serde_json::from_value::<Tool>(json!({"name":name,"description":description,"inputSchema":input,"outputSchema":output,"annotations":annotations})).unwrap()
        }).collect();
        Ok(ListToolsResult::with_all_items(tools)
            .with_ttl_ms(self.0.ttl)
            .with_cache_scope(CacheScope::Private))
    }
    fn accepted_subscription_filter(
        &self,
        requested: &SubscriptionFilter,
    ) -> Option<SubscriptionFilter> {
        Some(requested.clone())
    }
    async fn listen(&self, context: SubscriptionContext) -> Result<(), ErrorData> {
        let mut rx = self.0.changes.subscribe();
        loop {
            tokio::select! {
                _ = context.cancelled() => return Ok(()),
                event = rx.recv() => {
                    if event.is_err() { return Ok(()) }
                    let result = context.sink().notify_tool_list_changed().await;
                    self.0.log.event(json!({"kind":"notification_delivery","transport":"subscription","sent":result.is_ok(),"streams_open":self.0.streams.load(Ordering::SeqCst)}));
                }
            }
        }
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let name = request.name.as_ref();
        if !self.presented().contains(&name) {
            // IFC-012 / IFC-013 / ACC-011 / OPS-005: no invocation or outcome.
            return Err(ErrorData::invalid_params(
                format!("No tool of name {name} is presented to the instance"),
                Some(json!({self.0.names.meta("reason"):"refused"})),
            ));
        }
        let args = request.arguments.unwrap_or_default();
        let path = args.get("path").and_then(Value::as_str);
        // OPS-033: validate even the control before dispatch or any state change.
        let invalid = if name == "control.next.v1" {
            !args.is_empty()
        } else {
            path.is_none()
                || (name == "files.write.v1" && !args.get("text").is_some_and(Value::is_string))
                || args
                    .keys()
                    .any(|k| k != "path" && !(name == "files.write.v1" && k == "text"))
        };
        if invalid {
            let mut result: Value =
                serde_json::to_value(self.result(None, Some("invalid"), json!({}))).unwrap();
            result["structuredContent"] = json!({"diagnostics":[{"level":"error","ruleId":"OPS-033","message":"Arguments do not match the input schema"}]});
            return Ok(serde_json::from_value::<CallToolResult>(result)
                .unwrap()
                .into());
        }
        if name == "control.next.v1" {
            let changed = !self.0.advanced.swap(true, Ordering::SeqCst)
                && matches!(self.0.case, Case::C1 | Case::C5);
            if changed {
                let peers = self.0.peers.lock().unwrap().clone();
                self.0.log.event(json!({"kind":"list_changed","tools":self.presented(),"streams_open":self.0.streams.load(Ordering::SeqCst)}));
                for peer in peers {
                    let sent = peer.notify_tool_list_changed().await.is_ok();
                    self.0.log.event(json!({"kind":"notification_delivery","transport":"legacy","sent":sent,"streams_open":self.0.streams.load(Ordering::SeqCst)}));
                }
                let _ = self.0.changes.send(());
            }
            return Ok(self
                .result(Some("succeeded"), None, json!({"state":1}))
                .into());
        }
        if name == "files.write.v1" {
            return Ok(self.result(Some("unknown"), None, json!({})).into());
        }
        let text = match (name, path.unwrap()) {
            ("notes.read.v1", "hello.txt") => Some("A newly opened note."),
            (_, "hello.txt") => Some("Hello from memory."),
            (_, "other.txt") => Some("Another dummy file."),
            _ => None,
        };
        Ok(match text {
            Some(text) => self.result(Some("succeeded"), None, json!({"text":text})),
            None => self.result(Some("failed"), None, json!({})),
        }
        .into())
    }
}

async fn audit(State(stub): State<Stub>, request: Request, next: Next) -> Response {
    let id = stub.0.request_id.fetch_add(1, Ordering::SeqCst);
    let (parts, body) = request.into_parts();
    let bytes = match to_bytes(body, 4 * 1024 * 1024).await {
        Ok(b) => b,
        Err(_) => return StatusCode::PAYLOAD_TOO_LARGE.into_response(),
    };
    let body: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    let token = parts
        .headers
        .get("authorization")
        .and_then(|s| s.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));
    let valid = token.is_some_and(|t| stub.0.identity.valid(t));
    let request_text =
        String::from_utf8_lossy(&bytes).replace(&stub.0.identity.token, "[REDACTED]");
    let headers: std::collections::BTreeMap<_, _> = [
        "mcp-session-id",
        "mcp-protocol-version",
        "mcp-method",
        "mcp-name",
        "origin",
        "accept",
        "content-type",
        "last-event-id",
    ]
    .into_iter()
    .filter_map(|name| {
        let values: Vec<_> = parts
            .headers
            .get_all(name)
            .iter()
            .map(|value| {
                value
                    .to_str()
                    .unwrap_or("[non-UTF8]")
                    .replace(&stub.0.identity.token, "[REDACTED]")
            })
            .collect();
        (!values.is_empty()).then_some((name, values))
    })
    .collect();
    stub.0.log.event(json!({"kind":"request","id":id,"http_method":parts.method.as_str(),"body":request_text,"session":parts.headers.get("mcp-session-id").and_then(|s|s.to_str().ok()),"revision":parts.headers.get("mcp-protocol-version").and_then(|s|s.to_str().ok()),"headers":headers,"token_valid":valid}));
    let forced = stub.0.first_401.swap(false, Ordering::SeqCst);
    let origin_invalid = parts
        .headers
        .get("origin")
        .is_some_and(|value| value != stub.0.origin.as_str());
    let response = if origin_invalid {
        StatusCode::FORBIDDEN.into_response()
    } else if !valid || forced {
        (
            StatusCode::UNAUTHORIZED,
            [("www-authenticate", "Bearer error=\"invalid_token\"")],
        )
            .into_response()
    } else {
        let carries_keys = body["params"]["_meta"].as_object().is_some_and(|m| {
            m.keys()
                .any(|k| k.starts_with(stub.0.names.get("metaPrefix")))
        });
        let declares = body["params"]["_meta"]["io.modelcontextprotocol/clientCapabilities"]["extensions"]
            [stub.0.names.meta("implementation-interface")]["revision"]
            == "v0";
        if carries_keys && !declares {
            (StatusCode::BAD_REQUEST,axum::Json(json!({"jsonrpc":"2.0","id":body["id"],"error":{"code":-32021,"message":"Missing required client capability","data":{"requiredCapabilities":{"extensions":{stub.0.names.meta("implementation-interface"):{"revision":"v0"}}},stub.0.names.meta("reason"):"invalid"}}}))).into_response()
        } else {
            next.run(Request::from_parts(parts, Body::from(bytes)))
                .await
        }
    };
    let status = response.status().as_u16();
    let notification_stream = response
        .headers()
        .get("content-type")
        .is_some_and(|s| s.to_str().unwrap_or("").starts_with("text/event-stream"))
        && (body["method"] == "subscriptions/listen" || body.is_null());
    stub.0.log.event(json!({"kind":"response","id":id,"status":status,"notification_stream":notification_stream,"reason":if valid && origin_invalid {Some("invalid")} else {None},"headers":response.headers().iter().map(|(k,v)|(k.as_str(),v.to_str().unwrap_or(""))).collect::<std::collections::BTreeMap<_,_>>()}));
    let (mut parts, body) = response.into_parts();
    // Error data gains Conduit's reason below; its serialized length can change.
    parts.headers.remove("content-length");
    let stream = body.into_data_stream();
    if notification_stream {
        stub.0.streams.fetch_add(1, Ordering::SeqCst);
    }
    let guard = StreamGuard {
        stub: stub.clone(),
        id,
        counted: notification_stream,
    };
    let output = futures_util::stream::unfold(
        (stream, guard),
        move |(mut stream, guard)| async move {
            let chunk = stream.next().await?.map(|bytes| {
                if valid && !forced {
                    annotate_protocol_error(&bytes, &guard.stub.0.names).into()
                } else {
                    bytes
                }
            });
            if let Ok(ref chunk) = chunk {
                guard.stub.0.log.event(json!({"kind":"wire","id":guard.id,"data":String::from_utf8_lossy(chunk).replace(&guard.stub.0.identity.token,"[REDACTED]")}));
            }
            Some((chunk, (stream, guard)))
        },
    );
    Response::from_parts(parts, Body::from_stream(output))
}
// IFC-036: retain the protocol error/status and add the authenticated record's reason.
// rmcp emits each JSON body or SSE event in one data frame.
fn annotate_protocol_error(bytes: &[u8], names: &Names) -> Vec<u8> {
    fn annotate(value: &mut Value, names: &Names) -> bool {
        let Some(error) = value.get_mut("error").and_then(Value::as_object_mut) else {
            return false;
        };
        // IFC-036: unsupported-version negotiation is not a request ending invalid.
        if error.get("code") == Some(&json!(ErrorCode::UNSUPPORTED_PROTOCOL_VERSION)) {
            return false;
        }
        let data = error.entry("data").or_insert_with(|| json!({}));
        if !data.is_object() {
            *data = json!({"detail":data.clone()});
        }
        let data = data.as_object_mut().unwrap();
        if data.contains_key(&names.meta("reason")) {
            return false;
        }
        data.insert(names.meta("reason"), json!("invalid"));
        true
    }
    if let Ok(mut value) = serde_json::from_slice::<Value>(bytes) {
        return if annotate(&mut value, names) {
            serde_json::to_vec(&value).unwrap()
        } else {
            bytes.to_vec()
        };
    }
    let Ok(text) = std::str::from_utf8(bytes) else {
        return bytes.to_vec();
    };
    text.split_inclusive('\n')
        .map(|line| {
            if let Some(data) = line.trim_end().strip_prefix("data: ")
                && let Ok(mut value) = serde_json::from_str::<Value>(data)
                && annotate(&mut value, names)
            {
                return format!("data: {value}\n");
            }
            line.to_string()
        })
        .collect::<String>()
        .into_bytes()
}

struct StreamGuard {
    stub: Stub,
    id: u64,
    counted: bool,
}
impl Drop for StreamGuard {
    fn drop(&mut self) {
        if self.counted {
            self.stub.0.streams.fetch_sub(1, Ordering::SeqCst);
            self.stub
                .0
                .log
                .event(json!({"kind":"stream_closed","id":self.id}));
        }
    }
}

pub struct RunningStub {
    pub url: String,
    pub cancel: CancellationToken,
    pub task: JoinHandle<std::io::Result<()>>,
}
pub async fn start(
    case: Case,
    ttl: u64,
    first_401: bool,
    output: &Path,
    port: u16,
) -> Result<RunningStub> {
    let listener = TcpListener::bind(("127.0.0.1", port)).await?;
    let port = listener.local_addr()?.port();
    let url = format!("http://127.0.0.1:{port}/");
    let names = Names::default();
    let identity = Identity::mint(port)?;
    identity.write(&output.join("identity"), &url, &names)?;
    let log = Log::new(&output.join("stub.jsonl"))?;
    log.event(json!({"kind":"start","case":case.name(),"ttl_ms":ttl,"first_401":first_401,"url":url,"canonical_audience":identity.audience}));
    let stub = Stub(Arc::new(Shared {
        names,
        case,
        ttl,
        advanced: AtomicBool::new(false),
        invocation: AtomicU64::new(1),
        peers: Mutex::new(Vec::new()),
        changes: broadcast::channel(16).0,
        identity,
        origin: url.trim_end_matches('/').to_string(),
        first_401: AtomicBool::new(first_401),
        log,
        request_id: AtomicU64::new(1),
        streams: AtomicUsize::new(0),
    }));
    let cancel = CancellationToken::new();
    let mut config = StreamableHttpServerConfig::default();
    config.cancellation_token = cancel.clone();
    let service = StreamableHttpService::new(
        {
            let stub = stub.clone();
            move || Ok(stub.clone())
        },
        Arc::new(LocalSessionManager::default()),
        config,
    );
    let router = Router::new()
        .fallback_service(service)
        .layer(middleware::from_fn_with_state(stub, audit));
    let shutdown = cancel.clone();
    let task = tokio::spawn(async move {
        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown.cancelled_owned())
            .await
    });
    Ok(RunningStub { url, cancel, task })
}

pub fn headers_helper(identity_dir: &Path, log_path: &Path) -> Result<()> {
    let names = Names::default();
    let token =
        fs::read_to_string(identity_dir.join(names.get("tokenFile"))).context("read lab token")?;
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .open(log_path)?;
    writeln!(
        log,
        "{}",
        json!({"kind":"helper_read","unix_ms":SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()})
    )?;
    println!(
        "{}",
        json!({"Authorization":format!("Bearer {}",token.trim())})
    );
    Ok(())
}
