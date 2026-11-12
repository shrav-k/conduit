//! Per-run TLS Streamable HTTP gate for an opaque, line-oriented stdio channel.
//! No identity or protocol payload is logged or written to storage.
mod identity;
mod session;
use crate::backend::{Channel, Gate, GateHandle, Identity, Result, Unavailable};
use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::{BodyExt, Limited};
use hyper::{
    Method, Request, Response, StatusCode,
    body::{Body, Frame, Incoming},
    service::service_fn,
};
use hyper_util::rt::TokioIo;
use session::{Command, GateBody};
use socket2::{SockRef, TcpKeepalive};
use std::{
    convert::Infallible,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
    task::JoinSet,
};
use tokio_rustls::TlsAcceptor;
use tokio_util::sync::CancellationToken;

/// Public runtime contract settings; the advertised host is independent of the bind IP.
#[derive(Clone, Debug)]
pub struct GateConfig {
    pub bind_address: IpAddr,
    pub advertised_host: String,
    pub keepalive_idle: Duration,
    pub keepalive_interval: Duration,
    pub keepalive_count: u32,
    /// Maximum queued run-originated messages, including a slow GET consumer's queue.
    pub buffer_limit: usize,
    /// Maximum bytes in one HTTP message or one stdio line (excluding its delimiter).
    pub message_limit: usize,
    /// Observe sockets after the serving path configures them, only in unit tests.
    #[cfg(test)]
    accepted_sockets: Option<mpsc::UnboundedSender<std::io::Result<socket2::Socket>>>,
}
impl Default for GateConfig {
    fn default() -> Self {
        Self {
            bind_address: Ipv4Addr::LOCALHOST.into(),
            advertised_host: "127.0.0.1".into(),
            keepalive_idle: Duration::from_secs(30),
            keepalive_interval: Duration::from_secs(5),
            keepalive_count: 3,
            buffer_limit: 256,
            message_limit: 8 * 1024 * 1024,
            #[cfg(test)]
            accepted_sockets: None,
        }
    }
}
impl GateConfig {
    /// Idle time plus interval times count. macOS lacks TCP_USER_TIMEOUT (see README).
    pub fn silent_cut_bound(&self) -> Duration {
        self.keepalive_idle
            .saturating_add(self.keepalive_interval.saturating_mul(self.keepalive_count))
    }
    fn validate(&self) -> Result<()> {
        if self.bind_address.is_unspecified()
            || self.keepalive_count == 0
            || self.buffer_limit == 0
            || self.message_limit == 0
            || self.keepalive_idle.as_secs() == 0
            || self.keepalive_interval.as_secs() == 0
            || self.keepalive_idle.subsec_nanos() != 0
            || self.keepalive_interval.subsec_nanos() != 0
            || self.keepalive_idle.as_secs() > i32::MAX as u64
            || self.keepalive_interval.as_secs() > i32::MAX as u64
            || self.keepalive_count > i32::MAX as u32
            || self.silent_cut_bound().as_millis() > i32::MAX as u128
        {
            return Err(Unavailable);
        }
        // A host is a bare IP or DNS hostname, never a URL authority carrying user info/path.
        let host = &self.advertised_host;
        if host.is_empty()
            || (host.parse::<IpAddr>().is_err()
                && (!host.is_ascii()
                    || host.split('.').any(|part| {
                        part.is_empty()
                            || part.len() > 63
                            || part.starts_with('-')
                            || part.ends_with('-')
                            || !part.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
                    })))
        {
            return Err(Unavailable);
        }
        Ok(())
    }
    fn configure_socket(&self, socket: &TcpStream) -> std::io::Result<()> {
        let socket = SockRef::from(socket);
        socket.set_tcp_keepalive(
            &TcpKeepalive::new()
                .with_time(self.keepalive_idle)
                .with_interval(self.keepalive_interval)
                .with_retries(self.keepalive_count),
        )?;
        #[cfg(target_os = "linux")]
        socket.set_tcp_user_timeout(Some(self.silent_cut_bound()))?;
        Ok(())
    }
}
/// Shared implementation for local-process, container and SSH runtimes.
#[derive(Clone, Default)]
pub struct TlsGate {
    pub config: GateConfig,
}
impl TlsGate {
    pub fn new(config: GateConfig) -> Self {
        Self { config }
    }
}
struct Handle {
    url: String,
    stop: CancellationToken,
    ended: CancellationToken,
}
#[async_trait]
impl GateHandle for Handle {
    fn url(&self) -> String {
        self.url.clone()
    }
    async fn close(&self) {
        self.stop.cancel();
        self.ended.cancelled().await;
    }
    async fn ended(&self) {
        self.ended.cancelled().await;
    }
}
#[async_trait]
impl Gate for TlsGate {
    async fn open(
        &self,
        channel: Channel,
        identity: &Identity,
        cancel: &CancellationToken,
    ) -> Result<Arc<dyn GateHandle>> {
        self.config.validate()?;
        if cancel.is_cancelled() {
            return Err(Unavailable);
        }
        let (proof, tls) = identity::Proof::load(identity)?;
        let listener = tokio::select! {
            biased;
            _ = cancel.cancelled() => return Err(Unavailable),
            listener = TcpListener::bind(SocketAddr::new(self.config.bind_address, 0)) => listener.map_err(|_| Unavailable)?,
        };
        let port = listener.local_addr().map_err(|_| Unavailable)?.port();
        let host = if self
            .config
            .advertised_host
            .parse::<std::net::Ipv6Addr>()
            .is_ok()
        {
            format!("[{}]", self.config.advertised_host)
        } else {
            self.config.advertised_host.clone()
        };
        let stop = CancellationToken::new();
        let ended = CancellationToken::new();
        if cancel.is_cancelled() {
            return Err(Unavailable);
        }
        // No await between the final cancellation check and handing ownership to the task.
        let handle = Arc::new(Handle {
            url: format!("https://{host}:{port}/mcp"),
            stop: stop.clone(),
            ended: ended.clone(),
        });
        let config = self.config.clone();
        tokio::spawn(serve(
            listener,
            channel,
            config,
            Arc::new(proof),
            TlsAcceptor::from(Arc::new(tls)),
            stop,
            ended,
        ));
        Ok(handle)
    }
}
async fn serve(
    listener: TcpListener,
    channel: Channel,
    config: GateConfig,
    proof: Arc<identity::Proof>,
    tls: TlsAcceptor,
    stop: CancellationToken,
    ended: CancellationToken,
) {
    let (commands, receiver) = mpsc::unbounded_channel();
    let actor = tokio::spawn(session::run(
        channel,
        receiver,
        commands.clone(),
        config.clone(),
        stop.clone(),
    ));
    let connections_stop = CancellationToken::new();
    let mut connections = JoinSet::new();
    loop {
        tokio::select! {
            biased;
            _ = stop.cancelled() => break,
            Some(_) = connections.join_next(), if !connections.is_empty() => {},
            accepted = listener.accept() => {
                let Ok((socket, _)) = accepted else { break; };
                if config.configure_socket(&socket).is_err() { continue; }
                #[cfg(test)]
                if let Some(observer) = &config.accepted_sockets {
                    let _ = observer.send(SockRef::from(&socket).try_clone());
                }
                let tls = tls.clone();
                let proof = proof.clone();
                let commands = commands.clone();
                let connection_stop = connections_stop.clone();
                let limit = config.message_limit;
                connections.spawn(async move {
                    let work = async {
                        let Ok(tls) = tls.accept(socket).await else { return; };
                        let service = service_fn(move |request| http(request, proof.clone(), commands.clone(), limit));
                        // Half-close stays disabled: a disconnected HTTP/1 stream cancels its work.
                        let _ = hyper::server::conn::http1::Builder::new()
                            .serve_connection(TokioIo::new(tls), service).await;
                    };
                    tokio::select! { _ = connection_stop.cancelled() => {}, _ = work => {} }
                });
            }
        }
    }
    drop(listener);
    connections_stop.cancel();
    while connections.join_next().await.is_some() {}
    let _ = commands.send(Command::Shutdown);
    let _ = actor.await;
    ended.cancel();
}
async fn http(
    request: Request<Incoming>,
    proof: Arc<identity::Proof>,
    commands: mpsc::UnboundedSender<Command>,
    limit: usize,
) -> std::result::Result<Response<GateBody>, Infallible> {
    if !proof.admits(request.headers()) {
        let mut response = empty(StatusCode::UNAUTHORIZED);
        response.headers_mut().insert(
            hyper::header::WWW_AUTHENTICATE,
            hyper::header::HeaderValue::from_static("Bearer"),
        );
        return Ok(response);
    }
    if request.uri().path() != "/mcp" || request.uri().query().is_some() {
        return Ok(empty(StatusCode::NOT_FOUND));
    }
    if !matches!(
        *request.method(),
        Method::POST | Method::GET | Method::DELETE
    ) {
        let mut response = empty(StatusCode::METHOD_NOT_ALLOWED);
        response.headers_mut().insert(
            hyper::header::ALLOW,
            hyper::header::HeaderValue::from_static("POST, GET, DELETE"),
        );
        return Ok(response);
    }
    let session = match request.headers().get("mcp-session-id") {
        Some(h) => match h.to_str() {
            Ok(s) => Some(s.to_owned()),
            Err(_) => return Ok(empty(StatusCode::BAD_REQUEST)),
        },
        None => None,
    };
    let method = request.method().clone();
    let sse = accepts_sse(request.headers());
    let data = if method == Method::POST {
        match Limited::new(request.into_body(), limit).collect().await {
            Ok(body) => body.to_bytes(),
            Err(_) => return Ok(empty(StatusCode::PAYLOAD_TOO_LARGE)),
        }
    } else {
        Bytes::new()
    };
    let (send, receive) = tokio::sync::oneshot::channel();
    if commands
        .send(Command::Http {
            method,
            session,
            data,
            sse,
            reply: send,
        })
        .is_err()
    {
        return Ok(empty(StatusCode::SERVICE_UNAVAILABLE));
    }
    Ok(receive
        .await
        .unwrap_or_else(|_| empty(StatusCode::SERVICE_UNAVAILABLE)))
}
fn empty(status: StatusCode) -> Response<GateBody> {
    let mut response = Response::new(GateBody::empty());
    *response.status_mut() = status;
    response
}
fn accepts_sse(headers: &hyper::HeaderMap) -> bool {
    let mut best = None;
    for value in headers
        .get_all(hyper::header::ACCEPT)
        .iter()
        .filter_map(|h| h.to_str().ok())
    {
        for range in value.split(',') {
            let mut parts = range.trim().split(';');
            let media = parts.next().unwrap_or("").trim();
            let specificity = if media.eq_ignore_ascii_case("text/event-stream") {
                2
            } else if media.eq_ignore_ascii_case("text/*") {
                1
            } else if media == "*/*" {
                0
            } else {
                continue;
            };
            let mut quality = 1.0f32;
            for param in parts {
                if let Some((name, value)) = param.trim().split_once('=')
                    && name.eq_ignore_ascii_case("q")
                {
                    quality = value
                        .trim()
                        .parse()
                        .ok()
                        .filter(|q: &f32| q.is_finite() && (0.0..=1.0).contains(q))
                        .unwrap_or(0.0);
                }
            }
            if best.is_none_or(|(s, q)| specificity > s || (specificity == s && quality > q)) {
                best = Some((specificity, quality));
            }
        }
    }
    best.is_some_and(|(_, quality)| quality > 0.0)
}

#[cfg(test)]
mod tests;
