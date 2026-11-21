#![allow(dead_code)]
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use p256::ecdsa::{Signature, VerifyingKey, signature::Verifier};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

pub struct TestAuthority {
    pub root: String,
    pub key: rcgen::KeyPair,
    pub params: rcgen::CertificateParams,
}
impl TestAuthority {
    pub fn new() -> Self {
        let key = rcgen::KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256).unwrap();
        let mut params = rcgen::CertificateParams::default();
        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        params.key_usages = vec![rcgen::KeyUsagePurpose::KeyCertSign];
        let root = params.self_signed(&key).unwrap().pem();
        Self { root, key, params }
    }
    pub fn issue(&self, uri: &str) -> (String, String) {
        let key = rcgen::KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256).unwrap();
        let mut params = rcgen::CertificateParams::default();
        params.subject_alt_names = vec![rcgen::SanType::URI(uri.try_into().unwrap())];
        params.is_ca = rcgen::IsCa::ExplicitNoCa;
        params.key_usages = vec![rcgen::KeyUsagePurpose::DigitalSignature];
        params.extended_key_usages = vec![
            rcgen::ExtendedKeyUsagePurpose::ClientAuth,
            rcgen::ExtendedKeyUsagePurpose::ServerAuth,
        ];
        let issuer = rcgen::Issuer::new(self.params.clone(), &self.key);
        (
            params.signed_by(&key, &issuer).unwrap().pem(),
            key.serialize_pem(),
        )
    }
    pub fn self_signed(uri: &str) -> (String, String) {
        let key = rcgen::KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256).unwrap();
        let mut params = rcgen::CertificateParams::default();
        params.subject_alt_names = vec![rcgen::SanType::URI(uri.try_into().unwrap())];
        (params.self_signed(&key).unwrap().pem(), key.serialize_pem())
    }
}
pub struct Captured {
    pub method: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
    pub client_uri: Option<String>,
    pub at: tokio::time::Instant,
}
pub struct Gate {
    pub url: String,
    pub received: Arc<Mutex<Vec<Captured>>>,
    pub ended: Arc<AtomicBool>,
    pub session: Arc<Mutex<String>>,
    pub calls: Arc<std::sync::atomic::AtomicUsize>,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Gate {
    fn drop(&mut self) {
        self.task.abort();
    }
}
pub struct GateOptions {
    pub cert: String,
    pub key: String,
    pub client_root: Option<String>,
    pub auth: Option<(String, String, Value)>,
    pub redirect: Option<String>,
    pub sse: bool,
    pub count_file: Option<std::path::PathBuf>,
    pub get_events: bool,
    pub delay_first: bool,
    pub block_initialize: bool,
    pub prime_events: bool,
    pub hold_initialize_body: bool,
}
impl Gate {
    pub async fn start(options: GateOptions) -> Self {
        conduit_shell::install_crypto_provider();
        let cert = CertificateDer::pem_slice_iter(options.cert.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let key = PrivateKeyDer::from_pem_slice(options.key.as_bytes()).unwrap();
        let config = rustls::ServerConfig::builder();
        let config = if let Some(root) = &options.client_root {
            let mut roots = rustls::RootCertStore::empty();
            roots
                .add(CertificateDer::from_pem_slice(root.as_bytes()).unwrap())
                .unwrap();
            config.with_client_cert_verifier(
                rustls::server::WebPkiClientVerifier::builder(Arc::new(roots))
                    .build()
                    .unwrap(),
            )
        } else {
            config.with_no_client_auth()
        };
        let acceptor =
            tokio_rustls::TlsAcceptor::from(Arc::new(config.with_single_cert(cert, key).unwrap()));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("https://{}/mcp", listener.local_addr().unwrap());
        let received = Arc::new(Mutex::new(Vec::new()));
        let ended = Arc::new(AtomicBool::new(false));
        let session = Arc::new(Mutex::new(String::new()));
        let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let (log, ended2, session2, calls2) = (
            received.clone(),
            ended.clone(),
            session.clone(),
            calls.clone(),
        );
        let options = Arc::new(options);
        let session_requires_headers = Arc::new(AtomicBool::new(false));
        let release = Arc::new(tokio::sync::Notify::new());
        let second_seen = Arc::new(AtomicBool::new(false));
        let task = tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let (acceptor, log, options, ended, session, calls) = (
                    acceptor.clone(),
                    log.clone(),
                    options.clone(),
                    ended2.clone(),
                    session2.clone(),
                    calls2.clone(),
                );
                let (release, second_seen) = (release.clone(), second_seen.clone());
                let session_requires_headers = session_requires_headers.clone();
                tokio::spawn(async move {
                    let Ok(tls) = acceptor.accept(stream).await else {
                        return;
                    };
                    let client_uri = tls
                        .get_ref()
                        .1
                        .peer_certificates()
                        .and_then(|c| c.first())
                        .and_then(|c| {
                            let (_, parsed) =
                                x509_parser::parse_x509_certificate(c.as_ref()).ok()?;
                            let san = parsed.subject_alternative_name().ok()??;
                            match san.value.general_names.first()? {
                                x509_parser::extensions::GeneralName::URI(s) => Some(s.to_string()),
                                _ => None,
                            }
                        });
                    let mut reader = BufReader::new(tls);
                    loop {
                        let mut first = String::new();
                        if reader.read_line(&mut first).await.unwrap_or(0) == 0 {
                            return;
                        }
                        let method = first.split_whitespace().next().unwrap_or("").to_owned();
                        let mut headers = BTreeMap::new();
                        loop {
                            let mut line = String::new();
                            if reader.read_line(&mut line).await.unwrap_or(0) == 0 {
                                return;
                            }
                            if line == "\r\n" {
                                break;
                            }
                            if let Some((k, v)) = line.split_once(':') {
                                headers.insert(k.to_lowercase(), v.trim().to_owned());
                            }
                        }
                        let length = headers
                            .get("content-length")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0);
                        let mut body = vec![0; length];
                        if reader.read_exact(&mut body).await.is_err() {
                            return;
                        }
                        log.lock().unwrap().push(Captured {
                            method: method.clone(),
                            headers: headers.clone(),
                            body: body.clone(),
                            client_uri: client_uri.clone(),
                            at: tokio::time::Instant::now(),
                        });
                        if let Some(path) = &options.count_file {
                            std::fs::write(path, log.lock().unwrap().len().to_string()).unwrap();
                        }
                        let valid_auth =
                            options
                                .auth
                                .as_ref()
                                .is_none_or(|(engine, instance, jwks)| {
                                    headers.get("authorization").is_some_and(|v| {
                                        verify_token(
                                            v.strip_prefix("Bearer ").unwrap_or(""),
                                            engine,
                                            instance,
                                            jwks,
                                        )
                                    })
                                });
                        // The gate checks the headers of deeply nested test requests too.
                        let mut decoder = serde_json::Deserializer::from_slice(&body);
                        decoder.disable_recursion_limit();
                        let value = serde::Deserialize::deserialize(&mut decoder)
                            .and_then(|value: Value| decoder.end().map(|()| value))
                            .unwrap_or(Value::Null);
                        let initialize = value["method"] == "initialize";
                        let requires_headers = if initialize {
                            value["params"]["protocolVersion"] == "2026-07-28"
                        } else {
                            session_requires_headers.load(Ordering::SeqCst)
                        };
                        let valid_headers =
                            !requires_headers || valid_mcp_headers(&headers, &value);
                        if initialize && options.block_initialize {
                            std::future::pending::<()>().await;
                        }
                        if options.delay_first {
                            if value["id"] == 3 {
                                second_seen.store(true, Ordering::SeqCst);
                                release.notify_waiters();
                            }
                            if value["id"] == 2 {
                                loop {
                                    let notified = release.notified();
                                    tokio::pin!(notified);
                                    notified.as_mut().enable();
                                    if second_seen.load(Ordering::SeqCst) {
                                        break;
                                    }
                                    notified.await;
                                }
                            }
                        }
                        let mut extra = String::new();
                        let (status, content_type, response) = if !valid_auth {
                            ("401 Unauthorized", "application/json", String::new())
                        } else if !valid_headers {
                            ("400 Bad Request", "application/json", String::new())
                        } else if let Some(target) = &options.redirect {
                            extra = format!("Location: {target}\r\n");
                            ("307 Temporary Redirect", "application/json", String::new())
                        } else if initialize {
                            let fresh = uuid::Uuid::new_v4().to_string();
                            *session.lock().unwrap() = fresh.clone();
                            session_requires_headers.store(requires_headers, Ordering::SeqCst);
                            ended.store(false, Ordering::SeqCst);
                            extra = format!("Mcp-Session-Id: {fresh}\r\n");
                            if options.hold_initialize_body {
                                // An observable event proves the bridge has the
                                // response headers while initialize is pending.
                                let event = b"data: {\"jsonrpc\":\"2.0\",\"method\":\"notifications/test\"}\n\n";
                                let head = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\n{extra}\r\n",
                                    event.len() + 1
                                );
                                if reader.get_mut().write_all(head.as_bytes()).await.is_ok()
                                    && reader.get_mut().write_all(event).await.is_ok()
                                {
                                    let mut ignored = [0u8];
                                    let _ = reader.read(&mut ignored).await;
                                }
                                return;
                            }
                            let reply = format!(
                                "{{ \"jsonrpc\" : \"2.0\", \"id\" : {}, \"result\" : {{\"protocolVersion\":{},\"capabilities\":{{\"tools\":{{}}}},\"serverInfo\":{{\"name\":\"gate\",\"version\":\"1\"}}}} }}",
                                value["id"], value["params"]["protocolVersion"]
                            );
                            if options.sse {
                                ("200 OK", "text/event-stream", format!("data: {reply}\n\n"))
                            } else {
                                ("200 OK", "application/json", reply)
                            }
                        } else if !headers.contains_key("mcp-session-id") {
                            ("400 Bad Request", "application/json", String::new())
                        } else if ended.load(Ordering::SeqCst)
                            || headers.get("mcp-session-id") != Some(&*session.lock().unwrap())
                        {
                            ("404 Not Found", "application/json", String::new())
                        } else if method == "GET" && options.get_events {
                            ("200 OK", "text/event-stream", concat!(r#"data: { "jsonrpc":"2.0","#, "\r\n", r#"data: "method":"notifications/test", "params":{"s":"x\n"} }"#, "\r\n\r\n").into())
                        } else if method == "GET" {
                            ("405 Method Not Allowed", "application/json", String::new())
                        } else if method == "DELETE" {
                            ended.store(true, Ordering::SeqCst);
                            ("204 No Content", "application/json", String::new())
                        } else if value.get("id").is_none() {
                            ("202 Accepted", "application/json", String::new())
                        } else {
                            if value["method"] == "tools/call" {
                                calls.fetch_add(1, Ordering::SeqCst);
                            }
                            let reply = if value["method"] == "server/discover" {
                                json!({"jsonrpc":"2.0","id":value["id"],"result":{
                                    "supportedVersions":["2026-07-28"],
                                    "serverInfo":{"name":"gate","version":"1"},
                                    "capabilities":{"extensions":{
                                        "io.github.shrav-k.conduit/implementation-interface":{"revision":"v0"}
                                    }}
                                }}).to_string()
                            } else {
                                format!(
                                    "{{ \"jsonrpc\": \"2.0\", \"id\": {}, \"result\": {{\"content\":[],\"structuredContent\":{{}},\"_meta\":{{\"io.github.shrav-k.conduit/outcome\":\"succeeded\"}}}} }}",
                                    value["id"]
                                )
                            };
                            if options.sse && options.prime_events {
                                (
                                    "200 OK",
                                    "text/event-stream",
                                    format!("data: {{\r\ndata\r\ndata:{}\r\n\r\n", &reply[1..]),
                                )
                            } else if options.sse {
                                (
                                    "200 OK",
                                    "text/event-stream",
                                    format!("data: {reply}\r\n\r\n"),
                                )
                            } else {
                                ("200 OK", "application/json", reply)
                            }
                        };
                        let response =
                            if options.prime_events && content_type == "text/event-stream" {
                                format!("id: 0\ndata:\n\n{response}")
                            } else {
                                response
                            };
                        let reply = format!(
                            "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n{extra}\r\n{response}",
                            response.len()
                        );
                        if reader.get_mut().write_all(reply.as_bytes()).await.is_err() {
                            return;
                        }
                    }
                });
            }
        });
        Self {
            url,
            received,
            ended,
            session,
            calls,
            task,
        }
    }
}

fn valid_mcp_headers(headers: &BTreeMap<String, String>, body: &Value) -> bool {
    let Some(method) = body["method"].as_str() else {
        return true;
    };
    if headers.get("mcp-method").map(String::as_str) != Some(method) {
        return false;
    }
    if method != "tools/call" {
        return true;
    }
    let Some(name) = headers.get("mcp-name") else {
        return false;
    };
    if let Some(encoded) = name
        .strip_prefix("=?base64?")
        .and_then(|name| name.strip_suffix("?="))
    {
        return base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .is_ok_and(|bytes| {
                body["params"]["name"]
                    .as_str()
                    .is_some_and(|name| name.as_bytes() == bytes)
            });
    }
    body["params"]["name"].as_str() == Some(name)
}
pub fn verify_token(token: &str, engine: &str, instance: &str, jwks: &Value) -> bool {
    let mut parts = token.split('.');
    let (Some(h), Some(p), Some(s), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    let checked = || -> Option<()> {
        let header: Value = serde_json::from_slice(&B64.decode(h).ok()?).ok()?;
        if header["alg"] != "ES256" {
            return None;
        }
        let claims: Value = serde_json::from_slice(&B64.decode(p).ok()?).ok()?;
        if claims["sub"] != engine
            || !(claims["aud"] == instance || claims["aud"] == json!([instance]))
        {
            return None;
        }
        let key = &jwks["keys"][0];
        let mut point = vec![4];
        point.extend(B64.decode(key["x"].as_str()?).ok()?);
        point.extend(B64.decode(key["y"].as_str()?).ok()?);
        let key = VerifyingKey::from_sec1_bytes(&point).ok()?;
        key.verify(
            format!("{h}.{p}").as_bytes(),
            &Signature::from_slice(&B64.decode(s).ok()?).ok()?,
        )
        .ok()
    };
    checked().is_some()
}
pub async fn wait(mut ready: impl FnMut() -> bool) {
    tokio::time::timeout(Duration::from_secs(15), async {
        while !ready() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("progress deadline");
}
