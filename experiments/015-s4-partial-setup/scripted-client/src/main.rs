mod names;
mod record;
mod script;
use names::{EXTENSION, key};
use record::{Redactor, Trace, emit};
use rmcp::{
    model::*,
    service::{ClientLifecycleMode, ClientServiceExt},
    service::{PeerRequestOptions, ServiceError},
    transport::{
        StreamableHttpClientTransport,
        common::client_side_sse::NeverRetry,
        streamable_http_client::{StreamableHttpClientTransportConfig, StreamableHttpError},
    },
};
use script::{Script, Step};
use serde_json::{Value, json};
use std::{
    collections::HashSet,
    io,
    process::Stdio,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    io::AsyncReadExt,
    process::Command,
    time::{Instant as Clock, timeout_at},
};
use uuid::Uuid;

struct Launch {
    url: String,
    helper: String,
    thread: String,
}
fn launch(args: &[String]) -> Option<Launch> {
    if args.first()?.as_str() != "exec" || args.last()?.as_str() != "-" {
        return None;
    }
    // Consume option values before interpreting positional subcommands or settings.
    let mut settings = Vec::new();
    let mut positional = Vec::new();
    let mut rest = args[1..args.len() - 1].iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "-c" => settings.push(rest.next()?),
            "--sandbox" | "--color" | "--model" | "--disable" | "--enable" => {
                rest.next()?;
            }
            "--ignore-user-config"
            | "--strict-config"
            | "--ignore-rules"
            | "--skip-git-repo-check"
            | "--json" => (),
            s if s.starts_with('-') => return None,
            _ => positional.push(arg.as_str()),
        }
    }
    let setting = |prefix: &str| -> Option<String> {
        let mut values = Vec::new();
        for a in &settings {
            if let Some(v) = a.strip_prefix(prefix) {
                values.push(serde_json::from_str::<String>(v).ok()?);
            }
        }
        if values.len() == 1 {
            values.pop()
        } else {
            None
        }
    };
    let thread = match positional.as_slice() {
        [] => format!("scr-{}", Uuid::new_v4()),
        ["resume", id] if !id.is_empty() && !id.contains('=') && args[args.len() - 2] == *id => {
            (*id).to_owned()
        }
        _ => return None,
    };
    Some(Launch {
        url: setting("mcp_servers.engine.url=")?,
        helper: setting("mcp_servers.engine.http_headers_helper=")?,
        thread,
    })
}
fn allowed_url(s: &str) -> bool {
    let Ok(url) = reqwest::Url::parse(s) else {
        return false;
    };
    // Literal addresses avoid DNS and host-file lookups. Credentials/fragments are refused too.
    url.scheme() == "http"
        && url.username().is_empty()
        && url.password().is_none()
        && url.fragment().is_none()
        && url.host_str().is_some_and(|host| {
            host.trim_matches(['[', ']'])
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
        })
}
async fn token(helper: &str, deadline: Clock) -> Result<String, &'static str> {
    let mut cmd = Command::new("sh");
    cmd.args(["-c", helper])
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true);
    let output = timeout_at(deadline, cmd.output())
        .await
        .map_err(|_| "script-deadline")?
        .map_err(|_| "token-unavailable")?;
    if !output.status.success() {
        return Err("token-unavailable");
    }
    let v: Value = serde_json::from_slice(&output.stdout).map_err(|_| "token-unavailable")?;
    if v.as_object().is_none_or(|o| o.len() != 1) {
        return Err("token-unavailable");
    }
    let t = v["Authorization"]
        .as_str()
        .and_then(|h| h.strip_prefix("Bearer "))
        .filter(|t| !t.is_empty())
        .ok_or("token-unavailable")?;
    // Reject anything the HTTP header encoder cannot represent before opening a transport.
    reqwest::header::HeaderValue::from_str(&format!("Bearer {t}"))
        .map_err(|_| "token-unavailable")?;
    Ok(t.to_owned())
}
fn summary(s: Option<&Script>) -> Value {
    json!({"scr":"v1","id":s.map(|s| &s.id),"status":"completed","trace":s.map(|s| &s.trace),"stoppedAt":null,"cause":null,"problem":null,"steps":[]})
}
fn stop(s: &mut Value, at: Option<usize>, cause: &str) {
    s["status"] = json!("stopped");
    s["stoppedAt"] = json!(at);
    s["cause"] = json!(cause);
}
fn client_info() -> ClientConfig {
    serde_json::from_value(json!({"protocolVersion":"2026-07-28","clientInfo":{"name":"scr-client","version":env!("CARGO_PKG_VERSION")},"capabilities":{"experimental":{"scr-client":{"script":"v1"}}}})).unwrap()
}
fn reply_error(error: &ServiceError) -> Value {
    if let ServiceError::McpError(e) = error {
        return json!({"reply":"error","error":{"code":e.code,"message":e.message,"data":e.data}});
    }
    let mut status = None;
    let kind = match error {
        ServiceError::TransportSend(e) => {
            if let Some(e) = e
                .error
                .downcast_ref::<StreamableHttpError<reqwest::Error>>()
            {
                match e {
                    StreamableHttpError::SessionExpired => {
                        status = Some(404);
                        "session-expired"
                    }
                    StreamableHttpError::Client(e) => {
                        status = e.status().map(|s| s.as_u16());
                        "http-client"
                    }
                    StreamableHttpError::HttpStatus(e) => {
                        status = Some(e.status);
                        "http-status"
                    }
                    StreamableHttpError::UnexpectedEndOfStream => "stream-closed",
                    _ => "http-transport",
                }
            } else {
                "transport-send"
            }
        }
        ServiceError::TransportClosed => "transport-closed",
        ServiceError::UnexpectedResponse => "unexpected-response",
        _ => "service",
    };
    json!({"reply":"transport","transport":{"kind":kind,"httpStatus":status}})
}
fn startup_reply(error: rmcp::service::ClientInitializeError) -> Value {
    use rmcp::service::ClientInitializeError as E;
    match error {
        E::TransportError { error, .. } => reply_error(&ServiceError::TransportSend(error)),
        E::JsonRpcError(error) => reply_error(&ServiceError::McpError(error)),
        E::LegacyFallbackFailed { fallback, .. } => startup_reply(*fallback),
        other => {
            let kind = if matches!(other, E::NoCompatibleProtocolVersion { .. }) {
                "protocol-version"
            } else {
                "startup"
            };
            json!({"reply":"transport","transport":{"kind":kind,"httpStatus":null}})
        }
    }
}
fn copy_reply(step: &mut Value, r: &Value) {
    step["reply"] = r["reply"].clone();
    let source = if r["reply"] == "result" {
        &r["result"]["_meta"]
    } else {
        &r["error"]["data"]
    };
    step["isError"] = r["result"]["isError"].clone();
    for name in ["outcome", "reason", "restriction", "invocation"] {
        step[name] = source[key(name)].clone();
    }
    step["ruleIds"] = json!(
        r["result"]["structuredContent"]["diagnostics"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|d| d.get("ruleId"))
            .collect::<Vec<_>>()
    );
    step["code"] = r["error"]["code"].clone();
}
async fn request(
    peer: &rmcp::Peer<rmcp::RoleClient>,
    req: ClientRequest,
    meta: Value,
    deadline: Clock,
) -> Result<Value, ()> {
    let options = PeerRequestOptions::no_options().with_meta(serde_json::from_value(meta).unwrap());
    // No convenience API, timeout option, or RequestHandle::cancel: dropping the wait cancels nothing.
    timeout_at(deadline, async {
        match peer.send_request_with_option(req, options).await {
            Ok(handle) => match handle.await_response().await {
                Ok(result) => json!({"reply":"result","result":result}),
                Err(e) => reply_error(&e),
            },
            Err(e) => reply_error(&e),
        }
    })
    .await
    .map_err(|_| ())
}
async fn barrier(
    s: &Script,
    name: &str,
    index: usize,
    trace: &mut Trace,
    deadline: Clock,
) -> Result<(), &'static str> {
    let dir = s.barrier_dir.as_ref().unwrap();
    trace.event("barrier-reached", Some(index), json!({"name":name}))?;
    record::create(&dir.join(format!("{name}.reached"))).map_err(|_| "trace-failed")?;
    let limit = deadline.min(Clock::now() + Duration::from_millis(s.barrier_ms));
    loop {
        // Check time before release: a late release never passes a barrier.
        if Clock::now() >= limit {
            return Err(if limit == deadline {
                "script-deadline"
            } else {
                "barrier-timeout"
            });
        }
        if dir
            .join(format!("{name}.release"))
            .try_exists()
            .map_err(|_| "trace-failed")?
        {
            break;
        }
        tokio::time::sleep_until(limit.min(Clock::now() + Duration::from_millis(5))).await;
    }
    trace.event("barrier-released", Some(index), json!({"name":name}))?;
    record::create(&dir.join(format!("{name}.acknowledged"))).map_err(|_| "trace-failed")?;
    Ok(())
}
async fn drive(
    s: &Script,
    url: &str,
    bearer: &str,
    trace: &mut Trace,
    sum: &mut Value,
    deadline: Clock,
) -> Result<(), &'static str> {
    let mut session = None;
    let result = drive_steps(s, url, bearer, trace, sum, deadline, &mut session).await;
    if let Some(service) = session {
        // Closing the service shuts down transport; it sends no request cancellation.
        let _ = timeout_at(deadline, service.cancel()).await;
    }
    if result.is_ok() && Clock::now() >= deadline {
        return Err("script-deadline");
    }
    result
}
async fn drive_steps(
    s: &Script,
    url: &str,
    bearer: &str,
    trace: &mut Trace,
    sum: &mut Value,
    deadline: Clock,
    session: &mut Option<rmcp::service::RunningService<rmcp::RoleClient, ClientConfig>>,
) -> Result<(), &'static str> {
    let mut tools = HashSet::<String>::new();
    for (index, step) in s.steps.iter().enumerate() {
        sum["stoppedAt"] = json!(index);
        if Clock::now() >= deadline {
            return Err("script-deadline");
        }
        if !matches!(step, Step::Barrier { .. }) && session.is_none() {
            let client = reqwest::Client::builder()
                .retry(reqwest::retry::never())
                .redirect(reqwest::redirect::Policy::none())
                .no_proxy()
                .build()
                .map_err(|_| "open-failed")?;
            let mut config =
                StreamableHttpClientTransportConfig::with_uri(url.to_owned()).auth_header(bearer);
            config.reinit_on_expired_session = false;
            config.retry_config = Arc::new(NeverRetry::default());
            let limit = deadline.min(Clock::now() + Duration::from_millis(s.call_ms));
            let opened = timeout_at(
                limit,
                client_info().serve_with_lifecycle(
                    StreamableHttpClientTransport::with_client(client, config),
                    ClientLifecycleMode::Auto {
                        preferred_versions: vec![ProtocolVersion::V_2026_07_28],
                        legacy_version: Some(ProtocolVersion::V_2025_11_25),
                    },
                ),
            )
            .await;
            let service = match opened {
                Ok(Ok(service)) => service,
                Ok(Err(error)) => {
                    trace.event("reply", None, startup_reply(error))?;
                    return Err("open-failed");
                }
                _ => {
                    return Err(if Clock::now() >= deadline {
                        "script-deadline"
                    } else {
                        "open-failed"
                    });
                }
            };
            trace.event("open", None, json!({"server":service.peer_info()}))?;
            *session = Some(service);
        }
        match step {
            Step::Barrier { name } => {
                let mut entry =
                    json!({"index":index,"kind":"barrier","name":name,"acknowledged":false});
                let result = barrier(s, name, index, trace, deadline).await;
                entry["acknowledged"] = json!(result.is_ok());
                sum["steps"].as_array_mut().unwrap().push(entry);
                result?;
            }
            Step::Discover => {
                tools.clear();
                let mut cursor = None::<String>;
                let mut entry =
                    json!({"index":index,"kind":"discover","tools":[],"reply":"pending"});
                for page in 0..16 {
                    let params = cursor
                        .as_ref()
                        .map(|c| json!({"cursor":c}))
                        .unwrap_or_else(|| json!({}));
                    trace.event(
                        "request",
                        Some(index),
                        json!({"method":"tools/list","params":params,"page":page}),
                    )?;
                    let req: ClientRequest =
                        serde_json::from_value(json!({"method":"tools/list","params":params}))
                            .unwrap();
                    let limit = deadline.min(Clock::now() + Duration::from_millis(s.call_ms));
                    let result =
                        request(session.as_ref().unwrap().peer(), req, json!({}), limit).await;
                    let r = match result {
                        Ok(r) => r,
                        Err(()) => json!({"reply":"timeout"}),
                    };
                    trace.event(
                        if r["reply"] == "timeout" {
                            "timeout"
                        } else {
                            "reply"
                        },
                        Some(index),
                        r.clone(),
                    )?;
                    copy_reply(&mut entry, &r);
                    if r["reply"] != "result" || !r["result"]["tools"].is_array() {
                        sum["steps"].as_array_mut().unwrap().push(entry);
                        return Err(if Clock::now() >= deadline {
                            "script-deadline"
                        } else {
                            "discover-failed"
                        });
                    }
                    for tool in r["result"]["tools"].as_array().unwrap() {
                        if let Some(name) = tool["name"].as_str() {
                            tools.insert(name.into());
                        }
                    }
                    let mut sorted: Vec<_> = tools.iter().cloned().collect();
                    sorted.sort();
                    entry["tools"] = json!(sorted);
                    cursor = r["result"]["nextCursor"].as_str().map(str::to_owned);
                    if cursor.is_none() {
                        break;
                    }
                    if page == 15 {
                        sum["steps"].as_array_mut().unwrap().push(entry);
                        return Err("discover-failed");
                    }
                }
                sum["steps"].as_array_mut().unwrap().push(entry);
            }
            Step::Call {
                tool,
                arguments,
                mode,
                key: given,
            } => {
                let call_key = if mode == "fresh" {
                    Some(format!("scr-{}", Uuid::new_v4()))
                } else {
                    given.clone()
                };
                let mut entry = json!({"index":index,"kind":"call","tool":tool,"mode":mode,"key":call_key,"label":format!("call-{index}"),"reply":"pending","isError":null,"outcome":null,"reason":null,"restriction":null,"invocation":null,"ruleIds":[],"code":null});
                if !tools.contains(tool) {
                    sum["steps"].as_array_mut().unwrap().push(entry);
                    return Err("undiscovered-tool");
                }
                let mut meta = json!({});
                if let Some(k) = call_key {
                    let mut capabilities =
                        serde_json::to_value(client_info().capabilities).unwrap();
                    capabilities["extensions"] = json!({EXTENSION:{"revision":"v0"}});
                    meta[key("idempotencyKey")] = json!(k);
                    meta["io.modelcontextprotocol/clientCapabilities"] = capabilities;
                }
                let mut params = json!({"name":tool,"arguments":arguments});
                if mode != "unkeyed" {
                    params["_meta"] = meta.clone();
                }
                trace.event(
                    "request",
                    Some(index),
                    json!({"method":"tools/call","params":params}),
                )?;
                let req: ClientRequest =
                    serde_json::from_value(json!({"method":"tools/call","params":params})).unwrap();
                let limit = deadline.min(Clock::now() + Duration::from_millis(s.call_ms));
                let result = request(session.as_ref().unwrap().peer(), req, meta, limit).await;
                let r = match result {
                    Ok(r) => r,
                    Err(()) => json!({"reply":"timeout"}),
                };
                copy_reply(&mut entry, &r);
                trace.event(
                    if r["reply"] == "timeout" {
                        "timeout"
                    } else {
                        "reply"
                    },
                    Some(index),
                    r.clone(),
                )?;
                sum["steps"].as_array_mut().unwrap().push(entry);
                if r["reply"] == "timeout" {
                    return Err(if Clock::now() >= deadline {
                        "script-deadline"
                    } else {
                        "call-timeout"
                    });
                }
            }
        }
    }
    sum["stoppedAt"] = Value::Null;
    Ok(())
}
async fn execute(
    s: &Script,
    launch: &Launch,
    started: Instant,
    budget: Duration,
    announce: bool,
) -> (Value, Redactor) {
    let mut sum = summary(Some(s));
    let redact = Redactor::new(&s.redact);
    let file = match record::create(std::path::Path::new(&s.trace)) {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == io::ErrorKind::AlreadyExists {
                sum["status"] = json!("invalid");
                sum["problem"] = json!("trace.exists");
            } else {
                stop(&mut sum, None, "trace-failed");
            }
            if announce {
                let _ = emit(json!({"type":"thread.started","thread_id":launch.thread}));
            }
            return (sum, redact);
        }
    };
    let mut trace = Trace::new(file, redact, started);
    let deadline = Clock::from_std(started + budget);
    let prepared = if allowed_url(&launch.url) {
        token(&launch.helper, deadline).await
    } else {
        Err("url-refused")
    };
    if let Ok(bearer) = &prepared {
        trace.redact.add(bearer);
    }
    if announce {
        let _ = emit(json!({"type":"thread.started","thread_id":launch.thread}));
    }
    let result = match prepared {
        Ok(bearer) => drive(s, &launch.url, &bearer, &mut trace, &mut sum, deadline).await,
        Err(cause) => Err(cause),
    };
    if let Err(cause) = result {
        let at = sum["stoppedAt"].as_u64().map(|n| n as usize);
        stop(&mut sum, at, cause);
    }
    if trace
        .event(
            "end",
            None,
            json!({"status":sum["status"],"cause":sum["cause"]}),
        )
        .is_err()
    {
        let at = sum["stoppedAt"].as_u64().map(|n| n as usize);
        stop(&mut sum, at, "trace-failed");
    }
    (sum, trace.redact)
}
fn completion(sum: Value, redact: &Redactor) -> [Value; 2] {
    let text = redact.value(sum).to_string();
    [
        json!({"type":"item.completed","item":{"type":"agent_message","text":text}}),
        json!({"type":"turn.completed"}),
    ]
}
async fn main_result() -> io::Result<()> {
    let started = Instant::now();
    let Some(launch) = launch(&std::env::args().skip(1).collect::<Vec<_>>()) else {
        std::process::exit(2);
    };
    let mut input = String::new();
    if let Ok(read) = timeout_at(
        Clock::from_std(started + Duration::from_secs(170)),
        tokio::io::stdin().read_to_string(&mut input),
    )
    .await
    {
        read?;
    } else {
        let mut sum = summary(None);
        stop(&mut sum, None, "script-deadline");
        let redact = Redactor::default();
        emit(json!({"type":"thread.started","thread_id":launch.thread}))?;
        for event in completion(sum, &redact) {
            emit(event)?;
        }
        return Ok(());
    }
    let (sum, redact) = match Script::parse(&input) {
        Ok(s) => execute(&s, &launch, started, Duration::from_secs(170), true).await,
        Err(problem) => {
            let mut sum = summary(None);
            sum["status"] = json!("invalid");
            sum["problem"] = json!(problem.field);
            {
                let redact = Redactor::new(&problem.redact);
                emit(json!({"type":"thread.started","thread_id":launch.thread}))?;
                (sum, redact)
            }
        }
    };
    for event in completion(sum, &redact) {
        emit(event)?;
    }
    Ok(())
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    if main_result().await.is_err() {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn shortened_script_deadline_completes_turn() {
        let dir = tempfile::tempdir().unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        // Build directly rather than changing process cwd or exposing a production test override.
        let s = Script {
            id: "deadline".into(),
            trace: dir.path().join("deadline.jsonl").to_str().unwrap().into(),
            barrier_dir: Some(dir.path().into()),
            redact: vec![],
            call_ms: 60000,
            barrier_ms: 60000,
            steps: vec![Step::Barrier {
                name: "held".into(),
            }],
        };
        let launch = Launch {
            url: "http://127.0.0.1:1/mcp".into(),
            helper: "printf '%s' '{\"Authorization\":\"Bearer synthetic-deadline-marker\"}'".into(),
            thread: "test".into(),
        };
        let (sum, redact) = execute(
            &s,
            &launch,
            Instant::now(),
            Duration::from_millis(50),
            false,
        )
        .await;
        assert!(sum["status"] == "stopped" && sum["cause"] == "script-deadline");
        assert!(!dir.path().join("held.acknowledged").exists());
        let events = completion(sum, &redact);
        assert!(events[1]["type"] == "turn.completed");
        let message: Value =
            serde_json::from_str(events[0]["item"]["text"].as_str().unwrap()).unwrap();
        assert!(message["status"] == "stopped" && message["cause"] == "script-deadline");
    }
}
