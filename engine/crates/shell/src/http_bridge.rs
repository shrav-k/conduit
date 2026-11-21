//! Private stdio/Streamable HTTP byte bridge. No store or MCP model is opened here.
use axum::{
    body::{Body, to_bytes},
    http::{Method, Request, Response, StatusCode, header::HeaderValue},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use futures_util::StreamExt;
use hyper_util::rt::TokioIo;
use rustls::{
    DigitallySignedStruct, RootCertStore, SignatureScheme,
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    pki_types::{CertificateDer, PrivateKeyDer, ServerName, UnixTime, pem::PemObject},
};
use serde_json::{Value, value::RawValue};
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc,
    task::JoinSet,
};
use tokio_rustls::TlsConnector;
use url::Url;

/// Reserved for rejection of the server's proof, including a non-TLS URL.
pub const PROOF_REFUSED: i32 = 78;

#[derive(Debug)]
struct InstanceVerifier {
    roots: RootCertStore,
    expected: String,
    refused: Arc<AtomicBool>,
    provider: rustls::crypto::CryptoProvider,
}
impl ServerCertVerifier for InstanceVerifier {
    fn verify_server_cert(
        &self,
        end: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        _: &ServerName<'_>,
        _: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let checked = (|| {
            let parsed = rustls::server::ParsedCertificate::try_from(end)?;
            rustls::client::verify_server_cert_signed_by_trust_anchor(
                &parsed,
                &self.roots,
                intermediates,
                now,
                self.provider.signature_verification_algorithms.all,
            )?;
            let (_, cert) = x509_parser::parse_x509_certificate(end.as_ref())
                .map_err(|_| rustls::Error::General("invalid instance certificate".into()))?;
            let san = cert
                .subject_alternative_name()
                .map_err(|_| rustls::Error::General("invalid instance URI".into()))?;
            let names = san
                .as_ref()
                .map(|s| s.value.general_names.as_slice())
                .unwrap_or_default();
            if cert.is_ca()
                || !matches!(names, [x509_parser::extensions::GeneralName::URI(uri)] if *uri == self.expected)
            {
                return Err(rustls::Error::General(
                    "certificate did not prove the instance".into(),
                ));
            }
            Ok(ServerCertVerified::assertion())
        })();
        if checked.is_err() {
            self.refused.store(true, Ordering::SeqCst);
        }
        checked
    }
    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        signature: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        let result = rustls::crypto::verify_tls12_signature(
            message,
            cert,
            signature,
            &self.provider.signature_verification_algorithms,
        );
        if result.is_err() {
            self.refused.store(true, Ordering::SeqCst);
        }
        result
    }
    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        signature: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        let result = rustls::crypto::verify_tls13_signature(
            message,
            cert,
            signature,
            &self.provider.signature_verification_algorithms,
        );
        if result.is_err() {
            self.refused.store(true, Ordering::SeqCst);
        }
        result
    }
    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}

#[derive(Clone)]
struct Bridge {
    tls: TlsConnector,
    url: Url,
    token: HeaderValue,
    session: Option<HeaderValue>,
    revision: Option<HeaderValue>,
    output: mpsc::UnboundedSender<Vec<u8>>,
}
#[derive(serde::Deserialize)]
struct RequestMetadata<'a> {
    method: Option<String>,
    #[serde(borrow)]
    params: Option<&'a RawValue>,
}

#[derive(serde::Deserialize)]
struct ToolCallParams {
    name: String,
}

impl Bridge {
    async fn request(&self, method: Method, line: Option<String>) -> Result<Response<Body>, ()> {
        let host = match self.url.host().ok_or(())? {
            url::Host::Ipv6(ip) => ip.to_string(),
            host => host.to_string(),
        };
        let server_name = ServerName::try_from(host.clone()).map_err(|_| ())?;
        let port = self.url.port_or_known_default().ok_or(())?;
        let mut request = Request::builder()
            .method(method)
            .uri(&self.url[url::Position::BeforePath..url::Position::AfterQuery])
            .header(
                "Host",
                &self.url[url::Position::BeforeHost..url::Position::AfterPort],
            )
            .header("Authorization", self.token.clone())
            .header("Accept", "application/json, text/event-stream");
        if let Some(session) = &self.session {
            request = request.header("Mcp-Session-Id", session.clone());
        }
        if let Some(revision) = &self.revision {
            request = request.header("MCP-Protocol-Version", revision.clone());
        }
        if let Some(line) = &line {
            request = request.header("Content-Type", "application/json");
            // MCP request headers describe the unchanged JSON-RPC body. A
            // response and the session transport's GET/DELETE have no method.
            // Borrow params as raw JSON and skip other fields without
            // materializing nested values.
            let metadata: RequestMetadata<'_> = serde_json::from_str(line).map_err(|_| ())?;
            if let Some(method) = metadata.method {
                if method == "tools/call" {
                    let params: ToolCallParams =
                        serde_json::from_str(metadata.params.ok_or(())?.get()).map_err(|_| ())?;
                    request = request.header("Mcp-Name", mcp_name(&params.name));
                }
                request = request.header("Mcp-Method", method);
            }
        }
        let body = line.map_or_else(Body::empty, Body::from);
        let request = request.body(body).map_err(|_| ())?;
        // Establish the authenticated TLS connection before handing any HTTP
        // request bytes to Hyper. Each stream drives its own connection.
        let (mut sender, connection) = tokio::time::timeout(Duration::from_secs(10), async {
            let socket = TcpStream::connect((host.as_str(), port))
                .await
                .map_err(|_| ())?;
            let stream = self
                .tls
                .connect(server_name, socket)
                .await
                .map_err(|_| ())?;
            hyper::client::conn::http1::handshake(TokioIo::new(stream))
                .await
                .map_err(|_| ())
        })
        .await
        .map_err(|_| ())??;
        tokio::spawn(async move {
            let _ = connection.await;
        });
        sender
            .send_request(request)
            .await
            .map(|r| r.map(Body::new))
            .map_err(|_| ())
    }
    fn emit(&self, bytes: &[u8]) -> Result<Option<String>, ()> {
        let message: Value = serde_json::from_slice(bytes).map_err(|_| ())?;
        let id_valid = message
            .get("id")
            .is_none_or(|id| id.is_null() || id.is_string() || id.is_number());
        let request = message["method"].is_string()
            && message.get("result").is_none()
            && message.get("error").is_none()
            && message
                .get("params")
                .is_none_or(|p| p.is_object() || p.is_array());
        let response = message.get("method").is_none()
            && message.get("id").is_some()
            && (message.get("result").is_some() != message.get("error").is_some())
            && message
                .get("error")
                .is_none_or(|e| e["code"].as_i64().is_some() && e["message"].is_string());
        if message["jsonrpc"] != "2.0" || !id_valid || !(request || response) {
            return Err(());
        }
        let revision = message["result"]["protocolVersion"]
            .as_str()
            .map(str::to_owned);
        let mut line = bytes
            .iter()
            .map(|&b| if b == b'\r' || b == b'\n' { b' ' } else { b })
            .collect::<Vec<_>>();
        line.push(b'\n');
        self.output.send(line).map_err(|_| ())?;
        Ok(revision)
    }
    async fn response(
        &self,
        response: Response<Body>,
        initialize: bool,
        get_stream: bool,
    ) -> Result<Option<String>, ()> {
        let status = response.status();
        if status.is_redirection() {
            return Err(());
        }
        let sse = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| {
                v.split(';')
                    .next()
                    .is_some_and(|mime| mime.trim().eq_ignore_ascii_case("text/event-stream"))
            });
        if !sse {
            let accepted = response.status() == StatusCode::ACCEPTED;
            let body = to_bytes(response.into_body(), usize::MAX)
                .await
                .map_err(|_| ())?;
            return if accepted && body.is_empty() {
                Ok(None)
            } else {
                self.emit(&body)
            };
        }
        let mut stream = response.into_body().into_data_stream();
        let mut pending = Vec::new();
        let mut data = Vec::new();
        let mut skip_lf = false;
        let mut first_line = true;
        let mut emitted = false;
        while let Some(chunk) = stream.next().await {
            for byte in chunk.map_err(|_| ())? {
                if skip_lf && byte == b'\n' {
                    skip_lf = false;
                    continue;
                }
                skip_lf = false;
                if byte != b'\n' && byte != b'\r' {
                    pending.push(byte);
                    continue;
                }
                skip_lf = byte == b'\r';
                let line = std::mem::take(&mut pending);
                // The first line accumulates across chunks, including a split UTF-8 BOM.
                let line = if first_line {
                    first_line = false;
                    line.strip_prefix(b"\xef\xbb\xbf").unwrap_or(&line)
                } else {
                    &line
                };
                if line.is_empty() {
                    if !data.is_empty() {
                        data.pop();
                        if data.is_empty() {
                            continue;
                        }
                        let revision = self.emit(&data)?;
                        emitted = true;
                        data.clear();
                        if initialize && revision.is_some() {
                            return Ok(revision);
                        }
                    }
                } else if let Some(value) = line.strip_prefix(b"data:") {
                    data.extend_from_slice(value.strip_prefix(b" ").unwrap_or(value));
                    data.push(b'\n');
                } else if line == b"data" {
                    // A colonless field has an empty value, including its data newline.
                    data.push(b'\n');
                }
            }
        }
        if initialize
            || (!emitted && status != StatusCode::ACCEPTED && !(get_stream && status.is_success()))
        {
            Err(())
        } else {
            Ok(None)
        }
    }
    async fn post(&self, line: String) -> Result<(), ()> {
        let response = self.request(Method::POST, Some(line)).await?;
        self.response(response, false, false).await.map(|_| ())
    }
    async fn delete_session(&self) {
        if self.session.is_some() {
            let _ =
                tokio::time::timeout(Duration::from_secs(5), self.request(Method::DELETE, None))
                    .await;
        }
    }
    async fn get_stream(self) -> Result<(), ()> {
        loop {
            let started = tokio::time::Instant::now();
            let response = self.request(Method::GET, None).await?;
            if response.status() == StatusCode::METHOD_NOT_ALLOWED {
                return std::future::pending().await;
            }
            self.response(response, false, true).await?;
            tokio::time::sleep_until(started + Duration::from_secs(1)).await;
        }
    }
}

fn mcp_name(name: &str) -> String {
    if name.starts_with(' ')
        || name.ends_with(' ')
        || name.bytes().any(|byte| !(0x20..=0x7e).contains(&byte))
        || (name.starts_with("=?base64?") && name.ends_with("?="))
    {
        format!("=?base64?{}?=", STANDARD.encode(name))
    } else {
        name.to_owned()
    }
}

fn client(directory: &Path, refused: Arc<AtomicBool>) -> Result<(TlsConnector, HeaderValue), ()> {
    // Read exactly once. The runner removes this directory after initialization.
    let root = std::fs::read(directory.join("root.pem")).map_err(|_| ())?;
    let expected = std::fs::read_to_string(directory.join("peer-id")).map_err(|_| ())?;
    let token = std::fs::read_to_string(directory.join("bearer")).map_err(|_| ())?;
    let cert = std::fs::read(directory.join("client.pem")).map_err(|_| ())?;
    let key = std::fs::read(directory.join("client.key")).map_err(|_| ())?;
    let mut roots = RootCertStore::empty();
    let root = CertificateDer::from_pem_slice(&root).map_err(|_| ())?;
    roots.add(root).map_err(|_| ())?;
    let certs = CertificateDer::pem_slice_iter(&cert)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    let key = PrivateKeyDer::from_pem_slice(&key).map_err(|_| ())?;
    let provider = rustls::crypto::ring::default_provider();
    let config = rustls::ClientConfig::builder_with_provider(Arc::new(provider.clone()))
        .with_safe_default_protocol_versions()
        .map_err(|_| ())?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(InstanceVerifier {
            roots,
            expected,
            refused,
            provider,
        }))
        .with_client_auth_cert(certs, key)
        .map_err(|_| ())?;
    let client = TlsConnector::from(Arc::new(config));
    let mut token = HeaderValue::from_str(&format!("Bearer {token}")).map_err(|_| ())?;
    token.set_sensitive(true);
    Ok((client, token))
}
async fn run(url: Url, directory: &Path, refused: Arc<AtomicBool>) -> Result<(), ()> {
    let (tls, token) = client(directory, refused)?;
    let (output, mut lines) = mpsc::unbounded_channel::<Vec<u8>>();
    let writer = tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();
        while let Some(line) = lines.recv().await {
            stdout.write_all(&line).await.map_err(|_| ())?;
            stdout.flush().await.map_err(|_| ())?;
        }
        Ok::<_, ()>(())
    });
    let mut bridge = Bridge {
        tls,
        url,
        token,
        session: None,
        revision: None,
        output,
    };
    let result = async {
        let mut stdin = BufReader::new(tokio::io::stdin()).lines();
        let Some(first) = stdin.next_line().await.map_err(|_| ())? else {
            return Ok(());
        };
        let mut session = None;
        let mut pending = Vec::new();
        let initialized = {
            let response = async {
                let response = bridge.request(Method::POST, Some(first)).await?;
                session = response.headers().get("mcp-session-id").cloned();
                bridge.response(response, true, false).await
            };
            tokio::pin!(response);
            loop {
                tokio::select! {
                    biased;
                    line = stdin.next_line() => match line.map_err(|_| ())? {
                        Some(line) => pending.push(line),
                        None => break None,
                    },
                    revision = &mut response => break Some(revision),
                }
            }
        };
        bridge.session = session;
        let Some(revision) = initialized else {
            bridge.delete_session().await;
            return Ok(());
        };
        bridge.revision = Some(HeaderValue::from_str(&revision?.ok_or(())?).map_err(|_| ())?);
        let mut tasks = JoinSet::new();
        tasks.spawn(bridge.clone().get_stream());
        for line in pending {
            let bridge = bridge.clone();
            tasks.spawn(async move { bridge.post(line).await });
        }
        loop {
            tokio::select! {
                line = stdin.next_line() => {
                    match line.map_err(|_| ())? {
                        Some(line) => { let b = bridge.clone(); tasks.spawn(async move { b.post(line).await }); }
                        None => {
                            tasks.abort_all();
                            bridge.delete_session().await;
                            return Ok(());
                        }
                    }
                }
                task = tasks.join_next() => { if !matches!(task, Some(Ok(Ok(())))) { return Err(()); } }
            }
        }
    }.await;
    drop(bridge);
    let _ = writer.await;
    result
}

/// Binary-only private entry; errors are fixed text, never remote bodies or credentials.
pub fn main_entry() -> i32 {
    let raw = std::env::args().nth(2).unwrap_or_default();
    // Check the authority before URL parsing can discard empty user information
    // or normalize a malformed separator.
    let Some((scheme, tail)) = raw.split_once("://") else {
        return PROOF_REFUSED;
    };
    let authority = tail.split(['/', '?', '#']).next().unwrap_or_default();
    if !scheme.eq_ignore_ascii_case("https")
        || authority.is_empty()
        || authority
            .bytes()
            .any(|byte| byte.is_ascii_control() || matches!(byte, b' ' | b'@' | b'\\'))
    {
        return PROOF_REFUSED;
    }
    let parsed = Url::parse(&raw).ok();
    let Some(url) = parsed.filter(|u| {
        u.has_host() && u.username().is_empty() && u.password().is_none() && u.fragment().is_none()
    }) else {
        return PROOF_REFUSED;
    };
    let Some(directory) = std::env::var_os("HTTP_BRIDGE_DIR") else {
        return 1;
    };
    let refused = Arc::new(AtomicBool::new(false));
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build();
    let result = match runtime {
        Ok(runtime) => {
            let result = runtime.block_on(run(url, Path::new(&directory), refused.clone()));
            // Tokio's stdin read uses a blocking worker. A fatal HTTP response must
            // exit even while the engine still holds stdin open; runtime Drop would
            // otherwise wait for that read forever. main immediately exits the process.
            runtime.shutdown_background();
            result
        }
        Err(_) => Err(()),
    };
    if refused.load(Ordering::SeqCst) {
        PROOF_REFUSED
    } else if result.is_ok() {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod response_tests {
    use super::*;

    fn tls() -> TlsConnector {
        Arc::new(
            rustls::ClientConfig::builder()
                .with_root_certificates(RootCertStore::empty())
                .with_no_client_auth(),
        )
        .into()
    }
    #[tokio::test]
    async fn sse_bom_prefixed_initialization_survives_every_chunk_boundary() {
        crate::install_crypto_provider();
        let (output, mut lines) = mpsc::unbounded_channel();
        let bridge = Bridge {
            tls: tls(),
            url: Url::parse("https://bridge.test/mcp").unwrap(),
            token: HeaderValue::from_static("Bearer test-marker"),
            session: None,
            revision: None,
            output,
        };
        let reply = r#"{ "jsonrpc":"2.0", "id":1, "result":{"protocolVersion":"2025-11-25"} }"#;
        let event = format!("\u{feff}data: {reply}\r\n\r\n");
        for split in 0..=event.len() {
            let chunks = [
                Ok::<_, std::io::Error>(event.as_bytes()[..split].to_vec()),
                Ok(event.as_bytes()[split..].to_vec()),
            ];
            let response = axum::http::Response::builder()
                .header("content-type", "text/event-stream")
                .body(Body::from_stream(futures_util::stream::iter(chunks)))
                .unwrap();
            assert_eq!(
                bridge.response(response, true, false).await.unwrap(),
                Some("2025-11-25".into()),
                "chunk boundary {split}"
            );
            assert_eq!(lines.try_recv().unwrap(), format!("{reply}\n").as_bytes());
        }
    }

    #[tokio::test]
    async fn sse_bare_data_field_preserves_its_empty_line() {
        crate::install_crypto_provider();
        let (output, mut lines) = mpsc::unbounded_channel();
        let bridge = Bridge {
            tls: tls(),
            url: Url::parse("https://bridge.test/mcp").unwrap(),
            token: HeaderValue::from_static("Bearer test-marker"),
            session: None,
            revision: None,
            output,
        };
        let response = axum::http::Response::builder()
            .header("content-type", "text/event-stream")
            .body("data: {\ndata\ndata: \"jsonrpc\":\"2.0\",\"method\":\"notifications/test\"}\n\n")
            .unwrap();
        bridge
            .response(response.map(Body::from), false, false)
            .await
            .unwrap();
        assert_eq!(
            lines.try_recv().unwrap(),
            b"{  \"jsonrpc\":\"2.0\",\"method\":\"notifications/test\"}\n"
        );
        assert!(lines.try_recv().is_err());
    }

    #[tokio::test]
    async fn response_envelopes_and_sse_endings_preserve_bytes_or_end_the_bridge() {
        crate::install_crypto_provider();
        let (output, mut lines) = mpsc::unbounded_channel();
        let bridge = Bridge {
            tls: tls(),
            url: Url::parse("https://bridge.test/mcp").unwrap(),
            token: HeaderValue::from_static("Bearer test-marker"),
            session: None,
            revision: None,
            output,
        };
        let notification = r#"{ "jsonrpc":"2.0", "method":"notifications/test" }"#;
        for (status, mime, body, get, passes) in [
            (
                200,
                "application/json",
                notification.to_owned(),
                false,
                true,
            ),
            (
                500,
                "application/json",
                notification.to_owned(),
                false,
                true,
            ),
            (202, "application/json", String::new(), false, true),
            (401, "application/json", String::new(), false, false),
            (200, "text/event-stream", String::new(), false, false),
            (200, "text/event-stream", String::new(), true, true),
            (403, "text/event-stream", String::new(), true, false),
            (
                200,
                "text/event-stream",
                format!("data: {notification}\r\r"),
                false,
                true,
            ),
            (
                200,
                "application/json",
                r#"{"jsonrpc":"2.0","id":1,"result":{},"error":{"code":-1,"message":"bad"}}"#
                    .into(),
                false,
                false,
            ),
        ] {
            let response = axum::http::Response::builder()
                .status(status)
                .header("content-type", mime)
                .body(body.clone())
                .unwrap();
            assert_eq!(
                bridge
                    .response(response.map(Body::from), false, get)
                    .await
                    .is_ok(),
                passes,
                "status {status}, {mime}, GET={get}"
            );
            if passes && !body.is_empty() {
                assert_eq!(
                    lines.try_recv().unwrap(),
                    format!("{notification}\n").as_bytes()
                );
            } else {
                assert!(lines.try_recv().is_err());
            }
        }
    }
}
