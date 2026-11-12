//! Private stdio/Streamable HTTP byte bridge. No store or MCP model is opened here.
use futures_util::StreamExt;
use reqwest::{Client, Response, StatusCode, header::HeaderValue};
use rustls::{
    DigitallySignedStruct, RootCertStore, SignatureScheme,
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    pki_types::{CertificateDer, PrivateKeyDer, ServerName, UnixTime, pem::PemObject},
};
use serde_json::Value;
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
    sync::mpsc,
    task::JoinSet,
};

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
    client: Client,
    url: reqwest::Url,
    token: HeaderValue,
    session: Option<HeaderValue>,
    revision: Option<HeaderValue>,
    output: mpsc::UnboundedSender<Vec<u8>>,
}
impl Bridge {
    fn request(&self, method: reqwest::Method) -> reqwest::RequestBuilder {
        let mut request = self
            .client
            .request(method, self.url.clone())
            .header("Authorization", self.token.clone())
            .header("Accept", "application/json, text/event-stream");
        if let Some(session) = &self.session {
            request = request.header("Mcp-Session-Id", session.clone());
        }
        if let Some(revision) = &self.revision {
            request = request.header("MCP-Protocol-Version", revision.clone());
        }
        request
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
        response: Response,
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
            let body = response.bytes().await.map_err(|_| ())?;
            return if accepted && body.is_empty() {
                Ok(None)
            } else {
                self.emit(&body)
            };
        }
        let mut stream = response.bytes_stream();
        let mut pending = Vec::new();
        let mut data = Vec::new();
        let mut skip_lf = false;
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
                if line.is_empty() {
                    if !data.is_empty() {
                        data.pop();
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
    async fn post(
        &self,
        line: String,
        initialize: bool,
    ) -> Result<(Option<HeaderValue>, Option<String>), ()> {
        let response = self
            .request(reqwest::Method::POST)
            .header("Content-Type", "application/json")
            .body(line)
            .send()
            .await
            .map_err(|_| ())?;
        let session = response.headers().get("mcp-session-id").cloned();
        Ok((session, self.response(response, initialize, false).await?))
    }
    async fn get_stream(self) -> Result<(), ()> {
        loop {
            let started = tokio::time::Instant::now();
            let response = self
                .request(reqwest::Method::GET)
                .send()
                .await
                .map_err(|_| ())?;
            if response.status() == StatusCode::METHOD_NOT_ALLOWED {
                return std::future::pending().await;
            }
            self.response(response, false, true).await?;
            tokio::time::sleep_until(started + Duration::from_secs(1)).await;
        }
    }
}

fn client(directory: &Path, refused: Arc<AtomicBool>) -> Result<(Client, HeaderValue), ()> {
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
    let client = Client::builder()
        .tls_backend_preconfigured(config)
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .no_proxy()
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_zstd()
        .connect_timeout(Duration::from_secs(10))
        .build()
        .map_err(|_| ())?;
    let mut token = HeaderValue::from_str(&format!("Bearer {token}")).map_err(|_| ())?;
    token.set_sensitive(true);
    Ok((client, token))
}
async fn run(url: reqwest::Url, directory: &Path, refused: Arc<AtomicBool>) -> Result<(), ()> {
    let (client, token) = client(directory, refused)?;
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
        client,
        url,
        token,
        session: None,
        revision: None,
        output,
    };
    let result = async {
        let mut stdin = BufReader::new(tokio::io::stdin()).lines();
        let first = stdin.next_line().await.map_err(|_| ())?.ok_or(())?;
        let (session, revision) = bridge.post(first, true).await?;
        bridge.session = session;
        bridge.revision = Some(HeaderValue::from_str(&revision.ok_or(())?).map_err(|_| ())?);
        let mut tasks = JoinSet::new();
        tasks.spawn(bridge.clone().get_stream());
        loop {
            tokio::select! {
                line = stdin.next_line() => {
                    match line.map_err(|_| ())? {
                        Some(line) => { let b = bridge.clone(); tasks.spawn(async move { b.post(line, false).await.map(|_| ()) }); }
                        None => {
                            tasks.abort_all();
                            // Session deletion has its own bound; TCP keepalive is not a deadline.
                            if bridge.session.is_some() {
                                let _ = tokio::time::timeout(Duration::from_secs(5), bridge.request(reqwest::Method::DELETE).send()).await;
                            }
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
    let refused = Arc::new(AtomicBool::new(false));
    let raw = std::env::args().nth(2).unwrap_or_default();
    let user_info = raw.split_once("://").is_some_and(|(_, tail)| {
        tail.split(['/', '?', '#'])
            .next()
            .is_some_and(|authority| authority.contains('@'))
    });
    let parsed = reqwest::Url::parse(&raw).ok();
    let Some(url) = parsed.filter(|u| {
        u.scheme() == "https"
            && u.has_host()
            && u.username().is_empty()
            && u.password().is_none()
            && u.fragment().is_none()
            && !user_info
    }) else {
        return PROOF_REFUSED;
    };
    let Some(directory) = std::env::var_os("HTTP_BRIDGE_DIR") else {
        return 1;
    };
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
    #[tokio::test]
    async fn response_envelopes_and_sse_endings_preserve_bytes_or_end_the_bridge() {
        crate::install_crypto_provider();
        let (output, mut lines) = mpsc::unbounded_channel();
        let bridge = Bridge {
            client: Client::new(),
            url: reqwest::Url::parse("https://bridge.test/mcp").unwrap(),
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
                bridge.response(response.into(), false, get).await.is_ok(),
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
