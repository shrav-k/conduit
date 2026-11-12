use super::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use http_body_util::Full;
use p256::ecdsa::{Signature, SigningKey, signature::Signer};
use rcgen::{
    BasicConstraints, CertificateParams, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose, SanType,
};
use rustls::{
    DigitallySignedStruct, Error as TlsError, RootCertStore, SignatureScheme,
    client::{
        danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
        verify_server_cert_signed_by_trust_anchor,
    },
    crypto::{verify_tls12_signature, verify_tls13_signature},
    pki_types::{CertificateDer, ServerName, UnixTime},
    server::ParsedCertificate,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    io,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, DuplexStream},
    task::JoinHandle,
    time::timeout,
};
use tokio_rustls::TlsConnector;
use x509_parser::{extensions::GeneralName, prelude::parse_x509_certificate};
const INSTANCE: &str = "spiffe://gate.test/conduit/instance/aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
const ENGINE: &str = "spiffe://gate.test/conduit/engine";
const WAIT: Duration = Duration::from_secs(5);

struct Fixture {
    identity: Identity,
    root: CertificateDer<'static>,
    signing: SigningKey,
}
impl Fixture {
    fn new(instance: &str) -> Self {
        let mut root_params = CertificateParams::new(Vec::<String>::new()).unwrap();
        root_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        root_params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
        let root_key = KeyPair::generate().unwrap();
        let root = root_params.self_signed(&root_key).unwrap();
        let issuer = Issuer::from_params(&root_params, root_key);
        let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
        params.subject_alt_names = vec![SanType::URI(instance.try_into().unwrap())];
        params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
        params.extended_key_usages = vec![
            ExtendedKeyUsagePurpose::ServerAuth,
            ExtendedKeyUsagePurpose::ClientAuth,
        ];
        let key = KeyPair::generate().unwrap();
        let certificate = params.signed_by(&key, &issuer).unwrap();
        let signing = SigningKey::random(&mut p256::elliptic_curve::rand_core::OsRng);
        let point = signing.verifying_key().to_encoded_point(false);
        let jwks = json!({"keys":[{"kty":"EC","crv":"P-256","alg":"ES256","use":"sig","kid":"store",
            "x":B64.encode(point.x().unwrap()),"y":B64.encode(point.y().unwrap())}]});
        let run_token = sign(
            &signing,
            json!({"alg":"ES256","kid":"store"}),
            json!({"sub":instance}),
        );
        Self {
            identity: Identity {
                environment: BTreeMap::new(),
                files: BTreeMap::from([
                    ("tls.crt".into(), certificate.pem()),
                    ("tls.key".into(), key.serialize_pem()),
                    ("ca.crt".into(), root.pem()),
                    ("token".into(), run_token),
                    ("engine-id".into(), format!(" \n{ENGINE}\n ")),
                    ("jwks.json".into(), jwks.to_string()),
                ]),
            },
            root: root.der().clone(),
            signing,
        }
    }
    fn token(&self) -> String {
        self.custom(json!({"alg":"ES256","kid":"store"}), claims())
    }
    fn custom(&self, header: Value, claims: Value) -> String {
        sign(&self.signing, header, claims)
    }
    async fn open(&self) -> (Arc<dyn GateHandle>, Run) {
        self.open_with(GateConfig::default()).await
    }
    async fn open_with(&self, config: GateConfig) -> (Arc<dyn GateHandle>, Run) {
        let (channel, run) = tokio::io::duplex(1024 * 1024);
        let handle = TlsGate::new(config)
            .open(Box::new(channel), &self.identity, &CancellationToken::new())
            .await
            .unwrap();
        (handle, Run(BufReader::new(run)))
    }
    async fn connect(&self, handle: &dyn GateHandle) -> Client {
        Client::connect(&handle.url(), self.root.clone(), INSTANCE)
            .await
            .unwrap()
    }
}
fn claims() -> Value {
    json!({"sub":ENGINE,"aud":[INSTANCE],"exp":SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()+600})
}
fn sign(key: &SigningKey, header: Value, claims: Value) -> String {
    let message = format!(
        "{}.{}",
        B64.encode(header.to_string()),
        B64.encode(claims.to_string())
    );
    let signature: Signature = key.sign(message.as_bytes());
    format!("{message}.{}", B64.encode(signature.to_bytes()))
}
#[derive(Debug)]
struct Verifier {
    root: RootCertStore,
    expected: String,
}
impl ServerCertVerifier for Verifier {
    fn verify_server_cert(
        &self,
        cert: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        _: &ServerName<'_>,
        _: &[u8],
        now: UnixTime,
    ) -> std::result::Result<ServerCertVerified, TlsError> {
        let provider = rustls::crypto::ring::default_provider();
        let parsed = ParsedCertificate::try_from(cert)?;
        verify_server_cert_signed_by_trust_anchor(
            &parsed,
            &self.root,
            intermediates,
            now,
            provider.signature_verification_algorithms.all,
        )?;
        let (_, x509) = parse_x509_certificate(cert.as_ref())
            .map_err(|_| TlsError::General("certificate refused".into()))?;
        let san = x509
            .subject_alternative_name()
            .map_err(|_| TlsError::General("certificate refused".into()))?
            .ok_or_else(|| TlsError::General("certificate refused".into()))?;
        let valid = !x509.is_ca()
            && matches!(san.value.general_names.as_slice(), [GeneralName::URI(uri)] if *uri == self.expected);
        if !valid {
            return Err(TlsError::General("certificate refused".into()));
        }
        Ok(ServerCertVerified::assertion())
    }
    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, TlsError> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &rustls::crypto::ring::default_provider().signature_verification_algorithms,
        )
    }
    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, TlsError> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &rustls::crypto::ring::default_provider().signature_verification_algorithms,
        )
    }
    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        rustls::crypto::ring::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}
struct Client {
    sender: hyper::client::conn::http1::SendRequest<Full<Bytes>>,
    task: JoinHandle<()>,
}
impl Drop for Client {
    fn drop(&mut self) {
        self.task.abort();
    }
}
impl Client {
    async fn connect(
        url: &str,
        root: CertificateDer<'static>,
        expected: &str,
    ) -> std::result::Result<Self, ()> {
        let address = url
            .strip_prefix("https://")
            .unwrap()
            .strip_suffix("/mcp")
            .unwrap();
        let mut roots = RootCertStore::empty();
        roots.add(root).unwrap();
        let mut config = rustls::ClientConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(Verifier {
            root: roots,
            expected: expected.into(),
        }))
        .with_no_client_auth();
        config.alpn_protocols = vec![b"http/1.1".to_vec()];
        let socket = TcpStream::connect(address).await.map_err(|_| ())?;
        SockRef::from(&socket)
            .set_linger(Some(Duration::ZERO))
            .unwrap();
        let tls = TlsConnector::from(Arc::new(config))
            .connect(ServerName::try_from("unused.test").unwrap(), socket)
            .await
            .map_err(|_| ())?;
        assert!(tls.get_ref().1.alpn_protocol() == Some(b"http/1.1"));
        assert!(tls.get_ref().1.protocol_version() == Some(rustls::ProtocolVersion::TLSv1_3));
        let (sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(tls))
            .await
            .map_err(|_| ())?;
        let task = tokio::spawn(async move {
            let _ = connection.await;
        });
        Ok(Self { sender, task })
    }
    async fn send(
        &mut self,
        method: Method,
        body: &str,
        token: Option<&str>,
        session: Option<&str>,
        accept: Option<&str>,
    ) -> Response<Incoming> {
        let mut request = Request::builder().method(method).uri("/mcp");
        if let Some(token) = token {
            request = request.header("authorization", format!("Bearer {token}"));
        }
        if let Some(session) = session {
            request = request.header("mcp-session-id", session);
        }
        if let Some(accept) = accept {
            request = request.header("accept", accept);
        }
        // Deliberately omit Host, Origin and protocol version: the bridge need not supply them.
        timeout(
            WAIT,
            self.sender.send_request(
                request
                    .body(Full::new(Bytes::copy_from_slice(body.as_bytes())))
                    .unwrap(),
            ),
        )
        .await
        .unwrap()
        .unwrap()
    }
}
struct Run(BufReader<DuplexStream>);
impl Run {
    async fn line(&mut self) -> Vec<u8> {
        let mut line = Vec::new();
        assert!(
            timeout(WAIT, self.0.read_until(b'\n', &mut line))
                .await
                .unwrap()
                .unwrap()
                > 0
        );
        line.pop();
        line
    }
    async fn json(&mut self) -> Value {
        serde_json::from_slice(&self.line().await).unwrap()
    }
    async fn write(&mut self, line: &str) {
        self.0
            .get_mut()
            .write_all(format!("{line}\n").as_bytes())
            .await
            .unwrap();
    }
    async fn no_bytes(&mut self) {
        let mut byte = [0];
        assert!(
            timeout(Duration::from_millis(80), self.0.read(&mut byte))
                .await
                .is_err(),
            "unexpected channel byte"
        );
    }
    async fn eof(&mut self) {
        let mut byte = [0];
        assert!(
            timeout(WAIT, self.0.read(&mut byte))
                .await
                .unwrap()
                .unwrap()
                == 0
        );
    }
}
async fn body(response: Response<Incoming>) -> Bytes {
    timeout(WAIT, response.into_body().collect())
        .await
        .unwrap()
        .unwrap()
        .to_bytes()
}
async fn frame(body: &mut Incoming) -> Bytes {
    timeout(WAIT, body.frame())
        .await
        .unwrap()
        .unwrap()
        .unwrap()
        .into_data()
        .unwrap()
}
async fn initialize(client: &mut Client, run: &mut Run, token: &str, id: i64) -> String {
    let data =
        format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"method\":\"initialize\",\"params\":{{}}}}");
    let response = client
        .send(
            Method::POST,
            &data,
            Some(token),
            None,
            Some("application/json, text/event-stream"),
        )
        .await;
    assert!(response.status() == StatusCode::OK);
    let session = response.headers()["mcp-session-id"]
        .to_str()
        .unwrap()
        .to_owned();
    assert!(run.line().await == data.as_bytes());
    run.write(&format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{{}}}}"
    ))
    .await;
    assert!(body(response).await.starts_with(b"data: "));
    session
}

#[tokio::test]
async fn certificate_chain_uri_tls13_and_identity_mismatch() {
    let mut fixture = Fixture::new(INSTANCE);
    let (handle, mut run) = fixture.open().await;
    let _client = fixture.connect(handle.as_ref()).await;
    assert!(
        Client::connect(
            &handle.url(),
            fixture.root.clone(),
            "spiffe://gate.test/conduit/instance/other"
        )
        .await
        .is_err()
    );
    let stranger = Fixture::new(INSTANCE);
    assert!(
        Client::connect(&handle.url(), stranger.root, INSTANCE)
            .await
            .is_err()
    );
    run.no_bytes().await;
    handle.close().await;
    fixture.identity.files.insert(
        "token".into(),
        sign(
            &fixture.signing,
            json!({"alg":"ES256","kid":"store"}),
            json!({"sub":"spiffe://gate.test/conduit/instance/other"}),
        ),
    );
    let (channel, mut run) = tokio::io::duplex(256);
    assert!(
        TlsGate::default()
            .open(
                Box::new(channel),
                &fixture.identity,
                &CancellationToken::new()
            )
            .await
            .is_err()
    );
    let mut byte = [0];
    assert!(run.read(&mut byte).await.unwrap() == 0);
}

#[tokio::test]
async fn authorization_refusals_are_empty_and_never_touch_channel() {
    let fixture = Fixture::new(INSTANCE);
    let (handle, mut run) = fixture.open().await;
    let header = json!({"alg":"ES256","kid":"store"});
    let mut bad = vec![None, Some("not.a.jwt".into())];
    for (key, value) in [
        ("aud", json!(["spiffe://gate.test/other"])),
        ("aud", json!([INSTANCE, INSTANCE])),
        ("sub", json!("spiffe://gate.test/conduit/instance/other")),
        ("exp", json!(1)),
        ("exp", json!(1.5)),
        ("exp", Value::Null),
    ] {
        let mut c = claims();
        c[key] = value;
        bad.push(Some(fixture.custom(header.clone(), c)));
    }
    for h in [
        json!({"alg":"HS256","kid":"store"}),
        json!({"alg":"ES256","kid":"unknown"}),
        json!({"alg":"ES256","kid":"store","crit":[]}),
        json!({"alg":"ES256","kid":"store","crit":null}),
    ] {
        bad.push(Some(fixture.custom(h, claims())));
    }
    let other = Fixture::new(INSTANCE);
    bad.push(Some(other.custom(header, claims())));
    // Every method authenticates, including GET with no Accept.
    for token in bad {
        for method in [Method::POST, Method::GET, Method::DELETE] {
            let mut client = fixture.connect(handle.as_ref()).await;
            let response = client
                .send(
                    method,
                    r#"{"jsonrpc":"2.0","id":0,"method":"initialize"}"#,
                    token.as_deref(),
                    None,
                    None,
                )
                .await;
            assert!(response.status() == StatusCode::UNAUTHORIZED);
            assert!(response.headers()["www-authenticate"] == "Bearer");
            assert!(body(response).await.is_empty());
        }
    }
    run.no_bytes().await;
    let mut client = fixture.connect(handle.as_ref()).await;
    let mut c = claims();
    c["aud"] = json!(INSTANCE);
    let token = fixture.custom(json!({"alg":"ES256","kid":"store"}), c);
    initialize(&mut client, &mut run, &token, 0).await;
    handle.close().await;
}

#[tokio::test]
async fn byte_bridge_sessions_and_two_reply_formats() {
    let fixture = Fixture::new(INSTANCE);
    let (handle, mut run) = fixture.open().await;
    let token = fixture.token();
    let mut post = fixture.connect(handle.as_ref()).await;
    let session = initialize(&mut post, &mut run, &token, 0).await;
    // Engine response/notification POSTs are accepted without a response from stdio.
    for data in [
        r#"{ "jsonrpc" : "2.0", "method":"notifications/initialized" }"#,
        r#"{"jsonrpc":"2.0","id":"server","result":{"escaped":"a\\b"}}"#,
    ] {
        let response = post
            .send(Method::POST, data, Some(&token), Some(&session), None)
            .await;
        assert!(response.status() == StatusCode::ACCEPTED);
        assert!(body(response).await.is_empty());
        assert!(run.line().await == data.as_bytes());
    }
    let buffered =
        r#"{ "jsonrpc":"2.0", "method":"notifications/buffered", "params":{"x":"a\\b"} }"#;
    run.write(buffered).await;
    let mut get = fixture.connect(handle.as_ref()).await;
    let mut stream = get
        .send(Method::GET, "", Some(&token), Some(&session), None)
        .await
        .into_body();
    assert!(frame(&mut stream).await == format!("data: {buffered}\n\n"));
    for (id, accept, sse) in [
        (1, "application/json, text/event-stream", true),
        (2, "application/json", false),
    ] {
        let data = format!(
            "{{ \"jsonrpc\":\"2.0\", \"id\":{id}, \"method\":\"tools/call\", \"params\":{{}} }}"
        );
        let response = post
            .send(
                Method::POST,
                &data,
                Some(&token),
                Some(&session),
                Some(accept),
            )
            .await;
        assert!(
            response.headers()["content-type"]
                == if sse {
                    "text/event-stream"
                } else {
                    "application/json"
                }
        );
        assert!(run.line().await == data.as_bytes());
        // Equal request IDs in the reverse direction are not responses.
        let server_request = format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"method\":\"sampling/createMessage\",\"params\":{{}}}}"
        );
        run.write(&server_request).await;
        assert!(frame(&mut stream).await == format!("data: {server_request}\n\n"));
        let reply =
            format!("{{ \"jsonrpc\":\"2.0\", \"id\":{id}, \"result\":{{\"text\":\"a\\\\b\"}} }}");
        run.write(&reply).await;
        assert!(
            body(response).await
                == if sse {
                    format!("data: {reply}\n\n").into_bytes()
                } else {
                    reply.into_bytes()
                }
        );
    }
    for (session, status) in [
        (None, StatusCode::BAD_REQUEST),
        (Some("unknown"), StatusCode::NOT_FOUND),
    ] {
        let response = post
            .send(
                Method::POST,
                r#"{"jsonrpc":"2.0","method":"notifications/test"}"#,
                Some(&token),
                session,
                None,
            )
            .await;
        assert!(response.status() == status);
        assert!(body(response).await.is_empty());
    }
    let mut get2 = fixture.connect(handle.as_ref()).await;
    let response = get2
        .send(Method::GET, "", Some(&token), Some(&session), None)
        .await;
    assert!(timeout(WAIT, stream.frame()).await.unwrap().is_none());
    let mut stream = response.into_body();
    let note = r#"{"jsonrpc":"2.0","method":"notifications/newGet"}"#;
    run.write(note).await;
    assert!(frame(&mut stream).await == format!("data: {note}\n\n"));
    let response = post
        .send(Method::DELETE, "", Some(&token), Some(&session), None)
        .await;
    assert!(response.status() == StatusCode::OK);
    assert!(body(response).await.is_empty());
    assert!(timeout(WAIT, stream.frame()).await.unwrap().is_none());
    let response = post
        .send(Method::GET, "", Some(&token), Some(&session), None)
        .await;
    assert!(response.status() == StatusCode::NOT_FOUND);
    assert!(body(response).await.is_empty());
    run.no_bytes().await;
    // DELETE leaves stdio alive and another initialize reaches it.
    initialize(&mut post, &mut run, &token, 10).await;
    handle.close().await;
}

#[tokio::test]
async fn reset_cancels_both_reply_formats_and_discards_late_responses() {
    let fixture = Fixture::new(INSTANCE);
    let (handle, mut run) = fixture.open().await;
    let token = fixture.token();
    let mut client = fixture.connect(handle.as_ref()).await;
    let session = initialize(&mut client, &mut run, &token, 0).await;
    for (id, accept) in [(4, "text/event-stream"), (5, "application/json")] {
        let mut cut = fixture.connect(handle.as_ref()).await;
        let request = format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"method\":\"tools/call\"}}");
        let response = cut
            .send(
                Method::POST,
                &request,
                Some(&token),
                Some(&session),
                Some(accept),
            )
            .await;
        assert!(run.line().await == request.as_bytes());
        drop(cut);
        drop(response);
        let cancelled = run.json().await;
        assert!(
            cancelled["method"] == "notifications/cancelled"
                && cancelled["params"]["requestId"] == id
        );
        run.write(&format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{{}}}}"
        ))
        .await;
        let response = client
            .send(Method::POST, &request, Some(&token), Some(&session), None)
            .await;
        assert!(response.status() == StatusCode::CONFLICT);
        assert!(body(response).await.is_empty());
    }
    let mut get = fixture.connect(handle.as_ref()).await;
    let mut stream = get
        .send(Method::GET, "", Some(&token), Some(&session), None)
        .await
        .into_body();
    let note = r#"{"jsonrpc":"2.0","method":"notifications/afterLateReplies"}"#;
    run.write(note).await;
    assert!(frame(&mut stream).await == format!("data: {note}\n\n"));
    assert!(
        timeout(Duration::from_millis(80), stream.frame())
            .await
            .is_err()
    );
    handle.close().await;
}

#[tokio::test]
async fn replacement_delete_and_close_cancel_requests() {
    let fixture = Fixture::new(INSTANCE);
    let (handle, mut run) = fixture.open().await;
    let token = fixture.token();
    let mut client = fixture.connect(handle.as_ref()).await;
    let mut session = initialize(&mut client, &mut run, &token, 0).await;
    for id in [11, 12, 13] {
        let mut request_client = fixture.connect(handle.as_ref()).await;
        let request = format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"method\":\"tools/call\"}}");
        let response = request_client
            .send(
                Method::POST,
                &request,
                Some(&token),
                Some(&session),
                Some("text/event-stream"),
            )
            .await;
        assert!(run.line().await == request.as_bytes());
        let mut get_client = fixture.connect(handle.as_ref()).await;
        let mut get = get_client
            .send(Method::GET, "", Some(&token), Some(&session), None)
            .await
            .into_body();
        if id == 11 {
            let request = r#"{"jsonrpc":"2.0","id":10,"method":"initialize"}"#;
            let init = client
                .send(
                    Method::POST,
                    request,
                    Some(&token),
                    Some("ignored-on-initialize"),
                    None,
                )
                .await;
            let cancel = run.json().await;
            assert!(
                cancel["method"] == "notifications/cancelled"
                    && cancel["params"]["requestId"] == id
            );
            assert!(run.line().await == request.as_bytes());
            let old_session = session;
            session = init.headers()["mcp-session-id"]
                .to_str()
                .unwrap()
                .to_owned();
            assert!(session != old_session);
            run.write(r#"{"jsonrpc":"2.0","id":10,"result":{}}"#).await;
            body(init).await;
            let old = client
                .send(Method::GET, "", Some(&token), Some(&old_session), None)
                .await;
            assert!(old.status() == StatusCode::NOT_FOUND);
            assert!(body(old).await.is_empty());
            assert!(timeout(WAIT, get.frame()).await.unwrap().is_none());
            assert!(body(response).await.is_empty());
        } else if id == 12 {
            let delete = client
                .send(Method::DELETE, "", Some(&token), Some(&session), None)
                .await;
            assert!(delete.status() == StatusCode::OK);
            body(delete).await;
            let cancel = run.json().await;
            assert!(
                cancel["method"] == "notifications/cancelled"
                    && cancel["params"]["requestId"] == id
            );
            assert!(timeout(WAIT, get.frame()).await.unwrap().is_none());
            assert!(body(response).await.is_empty());
            session = initialize(&mut client, &mut run, &token, 20).await;
        } else {
            let ((), cancel) = tokio::join!(handle.close(), run.json());
            assert!(
                cancel["method"] == "notifications/cancelled"
                    && cancel["params"]["requestId"] == id
            );
            run.eof().await;
            drop(response);
            drop(get);
        }
    }
    timeout(WAIT, handle.close()).await.unwrap();
    timeout(WAIT, handle.ended()).await.unwrap();
}

#[tokio::test]
async fn buffer_overflow_ends_session_and_keeps_channel() {
    let fixture = Fixture::new(INSTANCE);
    let (handle, mut run) = fixture
        .open_with(GateConfig {
            buffer_limit: 2,
            ..GateConfig::default()
        })
        .await;
    let token = fixture.token();
    let mut client = fixture.connect(handle.as_ref()).await;
    let session = initialize(&mut client, &mut run, &token, 0).await;
    let mut pending = fixture.connect(handle.as_ref()).await;
    let response = pending
        .send(
            Method::POST,
            r#"{"jsonrpc":"2.0","id":1,"method":"tools/call"}"#,
            Some(&token),
            Some(&session),
            None,
        )
        .await;
    run.line().await;
    for _ in 0..3 {
        run.write(r#"{"jsonrpc":"2.0","method":"notifications/buffer"}"#)
            .await;
    }
    assert!(run.json().await["params"]["requestId"] == 1);
    assert!(body(response).await.is_empty());
    let response = client
        .send(Method::GET, "", Some(&token), Some(&session), None)
        .await;
    assert!(response.status() == StatusCode::NOT_FOUND);
    body(response).await;
    initialize(&mut client, &mut run, &token, 2).await;
    handle.close().await;
}

#[tokio::test]
async fn two_listeners_serve_distinct_certificates_and_ipv6_url_is_bracketed() {
    let first = Fixture::new(INSTANCE);
    let other_id = "spiffe://gate.test/conduit/instance/bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
    let second = Fixture::new(other_id);
    let (one, _run_one) = first.open().await;
    let (two, _run_two) = second.open().await;
    assert!(one.url() != two.url());
    let _one = first.connect(one.as_ref()).await;
    let _two = Client::connect(&two.url(), second.root.clone(), other_id)
        .await
        .unwrap();
    assert!(
        Client::connect(&two.url(), first.root.clone(), INSTANCE)
            .await
            .is_err()
    );
    assert!(
        Client::connect(&two.url(), second.root.clone(), INSTANCE)
            .await
            .is_err()
    );
    let (v6, _run_v6) = first
        .open_with(GateConfig {
            advertised_host: "::1".into(),
            ..GateConfig::default()
        })
        .await;
    assert!(v6.url().starts_with("https://[::1]:"));
    v6.close().await;
    one.close().await;
    two.close().await;
}

#[tokio::test]
async fn keepalive_settings_read_back_from_accepted_socket() {
    let (accepted_sockets, mut accepted) = mpsc::unbounded_channel();
    let config = GateConfig {
        keepalive_idle: Duration::from_secs(7),
        keepalive_interval: Duration::from_secs(2),
        keepalive_count: 4,
        accepted_sockets: Some(accepted_sockets),
        ..GateConfig::default()
    };
    assert!(config.silent_cut_bound() == Duration::from_secs(15));
    let fixture = Fixture::new(INSTANCE);
    let (handle, _run) = fixture.open_with(config.clone()).await;
    for _ in 0..2 {
        let _client = fixture.connect(handle.as_ref()).await;
        let socket = timeout(WAIT, accepted.recv())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        assert!(socket.keepalive().unwrap());
        assert!(socket.tcp_keepalive_time().unwrap() == config.keepalive_idle);
        assert!(socket.tcp_keepalive_interval().unwrap() == config.keepalive_interval);
        assert!(socket.tcp_keepalive_retries().unwrap() == config.keepalive_count);
        #[cfg(target_os = "linux")]
        assert!(socket.tcp_user_timeout().unwrap() == Some(config.silent_cut_bound()));
    }
    handle.close().await;
}

#[tokio::test]
async fn close_channel_eof_and_cancelled_or_invalid_opens() {
    let fixture = Fixture::new(INSTANCE);
    let (handle, mut run) = fixture.open().await;
    let address = handle
        .url()
        .trim_start_matches("https://")
        .trim_end_matches("/mcp")
        .to_owned();
    let mut handshake = TcpStream::connect(&address).await.unwrap();
    timeout(WAIT, handle.close()).await.unwrap();
    timeout(WAIT, handle.close()).await.unwrap();
    run.eof().await;
    let mut byte = [0];
    assert!(matches!(
        timeout(WAIT, handshake.read(&mut byte)).await.unwrap(),
        Ok(0) | Err(_)
    ));
    assert!(TcpStream::connect(&address).await.is_err());
    let (handle, run) = fixture.open().await;
    drop(run);
    timeout(WAIT, handle.ended()).await.unwrap();
    assert!(
        TcpStream::connect(
            handle
                .url()
                .trim_start_matches("https://")
                .trim_end_matches("/mcp")
        )
        .await
        .is_err()
    );
    let cancel = CancellationToken::new();
    cancel.cancel();
    let (channel, mut run) = tokio::io::duplex(64);
    assert!(
        TlsGate::default()
            .open(Box::new(channel), &fixture.identity, &cancel)
            .await
            .is_err()
    );
    assert!(run.read(&mut byte).await.unwrap() == 0);
    for ip in ["0.0.0.0", "::"] {
        let (channel, mut run) = tokio::io::duplex(64);
        let gate = TlsGate::new(GateConfig {
            bind_address: ip.parse().unwrap(),
            ..GateConfig::default()
        });
        assert!(
            gate.open(
                Box::new(channel),
                &fixture.identity,
                &CancellationToken::new()
            )
            .await
            .is_err()
        );
        assert!(run.read(&mut byte).await.unwrap() == 0);
    }
    for file in ["tls.crt", "tls.key"] {
        let mut missing = Fixture::new(INSTANCE);
        missing.identity.files.remove(file);
        let (channel, mut run) = tokio::io::duplex(64);
        assert!(
            TlsGate::default()
                .open(
                    Box::new(channel),
                    &missing.identity,
                    &CancellationToken::new()
                )
                .await
                .is_err()
        );
        assert!(run.read(&mut byte).await.unwrap() == 0);
    }
}

#[derive(Clone)]
struct Logs(Arc<Mutex<Vec<u8>>>);
impl io::Write for Logs {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Logs {
    type Writer = Self;
    fn make_writer(&'a self) -> Self {
        self.clone()
    }
}
#[tokio::test]
async fn sensitive_markers_are_absent_from_logs_and_answers() {
    let logs = Logs(Arc::new(Mutex::new(Vec::new())));
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_ansi(false)
        .with_max_level(tracing::Level::TRACE)
        .with_writer(logs.clone())
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let markers = async {
        let mut fixture = Fixture::new(INSTANCE);
        let marker = ["sensitive-", "gate-", "synthetic"].concat();
        fixture
            .identity
            .environment
            .insert("private".into(), marker.clone());
        fixture
            .identity
            .files
            .insert("private".into(), marker.clone());
        let token = fixture.token();
        let (handle, mut run) = fixture.open().await;
        let mut client = fixture.connect(handle.as_ref()).await;
        let response = client
            .send(
                Method::POST,
                r#"{"jsonrpc":"2.0","id":0,"method":"initialize"}"#,
                Some(&marker),
                None,
                None,
            )
            .await;
        assert!(response.status() == StatusCode::UNAUTHORIZED);
        let refused_headers = format!("{:?}", response.headers());
        assert!(body(response).await.is_empty());
        run.no_bytes().await;
        let initialize = r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#;
        let response = client
            .send(
                Method::POST,
                initialize,
                Some(&token),
                None,
                Some("application/json"),
            )
            .await;
        assert!(response.status() == StatusCode::OK);
        let admitted_headers = format!("{:?}", response.headers());
        assert!(run.line().await == initialize.as_bytes());
        let reply = r#"{"jsonrpc":"2.0","id":1,"result":{}}"#;
        run.write(reply).await;
        let admitted_body = body(response).await;
        assert!(admitted_body == reply.as_bytes());
        let values = fixture
            .identity
            .files
            .values()
            .chain(fixture.identity.environment.values())
            .cloned()
            .chain([token, marker])
            .collect::<Vec<_>>();
        for value in &values {
            assert!(
                !refused_headers.contains(value)
                    && !admitted_headers.contains(value)
                    && !admitted_body
                        .windows(value.len())
                        .any(|w| w == value.as_bytes())
                    && !handle.url().contains(value),
                "sensitive answer"
            );
        }
        handle.close().await;
        values
    }
    .await;
    let logs = logs.0.lock().unwrap();
    for marker in markers {
        assert!(
            !logs.windows(marker.len()).any(|w| w == marker.as_bytes()),
            "sensitive log"
        );
    }
}

#[test]
fn accepts_sse_and_validates_configuration() {
    for (value, expected) in [
        ("application/json, text/event-stream", true),
        ("text/event-stream;q=0", false),
        ("text/event-stream;q=0, */*;q=1", false),
        ("text/*;q=0, text/event-stream;q=1", true),
        ("TEXT/EVENT-STREAM", true),
        ("application/json", false),
        ("*/*", true),
    ] {
        let mut headers = hyper::HeaderMap::new();
        headers.insert("accept", value.parse().unwrap());
        assert!(accepts_sse(&headers) == expected);
    }
    for host in [
        "user@host",
        "host/path",
        "host:1234",
        "host?x",
        "host#x",
        "",
    ] {
        assert!(
            GateConfig {
                advertised_host: host.into(),
                ..GateConfig::default()
            }
            .validate()
            .is_err()
        );
    }
    assert!(
        GateConfig {
            keepalive_count: 0,
            ..GateConfig::default()
        }
        .validate()
        .is_err()
    );
}

#[tokio::test]
async fn protocol_refusals_limits_and_unmatched_responses() {
    let fixture = Fixture::new(INSTANCE);
    let (handle, mut run) = fixture
        .open_with(GateConfig {
            message_limit: 128,
            ..GateConfig::default()
        })
        .await;
    let token = fixture.token();
    let mut client = fixture.connect(handle.as_ref()).await;
    let session = initialize(&mut client, &mut run, &token, 0).await;
    let unsupported = client
        .send(Method::PUT, "", Some(&token), Some(&session), None)
        .await;
    assert!(unsupported.status() == StatusCode::METHOD_NOT_ALLOWED);
    assert!(unsupported.headers()["allow"] == "POST, GET, DELETE");
    assert!(body(unsupported).await.is_empty());
    for data in [
        "invalid",
        "[]",
        r#"{"jsonrpc":"2.0","id":null,"method":"tools/call"}"#,
        "{\n\"jsonrpc\":\"2.0\",\"method\":\"notifications/pretty\"}",
    ] {
        let response = client
            .send(Method::POST, data, Some(&token), Some(&session), None)
            .await;
        assert!(response.status() == StatusCode::BAD_REQUEST);
        assert!(body(response).await.is_empty());
    }
    let response = client
        .send(
            Method::POST,
            &"x".repeat(129),
            Some(&token),
            Some(&session),
            None,
        )
        .await;
    assert!(response.status() == StatusCode::PAYLOAD_TOO_LARGE);
    assert!(body(response).await.is_empty());
    run.no_bytes().await;
    // A response cannot become a GET message even if no such request ever existed.
    run.write(r#"{"jsonrpc":"2.0","id":"unmatched","result":{}}"#)
        .await;
    let mut get_client = fixture.connect(handle.as_ref()).await;
    let mut get = get_client
        .send(Method::GET, "", Some(&token), Some(&session), None)
        .await
        .into_body();
    let note = r#"{"jsonrpc":"2.0","method":"notifications/valid"}"#;
    run.write(note).await;
    assert!(frame(&mut get).await == format!("data: {note}\n\n"));
    run.0.get_mut().write_all(&[b'x'; 130]).await.unwrap();
    timeout(WAIT, handle.ended()).await.unwrap();
}

#[tokio::test]
async fn close_finishes_when_run_stops_reading_its_channel() {
    let fixture = Fixture::new(INSTANCE);
    let (handle, mut run) = fixture.open().await;
    let token = fixture.token();
    let mut client = fixture.connect(handle.as_ref()).await;
    let session = initialize(&mut client, &mut run, &token, 0).await;
    // Exceed the fake pipe's capacity so shutdown cannot rely on a cooperative reader.
    let message = json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"x":"x".repeat(2*1024*1024)}}).to_string();
    let _response = client
        .send(Method::POST, &message, Some(&token), Some(&session), None)
        .await;
    timeout(WAIT, handle.close()).await.unwrap();
    let mut remainder = Vec::new();
    timeout(WAIT, run.0.read_to_end(&mut remainder))
        .await
        .unwrap()
        .unwrap();
    timeout(WAIT, handle.close()).await.unwrap();
}
