use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Method, Request, Response, body::Incoming};
use hyper_util::rt::TokioIo;
use p256::ecdsa::{Signature, SigningKey, signature::Signer};
use rcgen::{
    BasicConstraints, CertificateParams, ExtendedKeyUsagePurpose, IsCa, Issuer, KeyPair,
    KeyUsagePurpose, SanType,
};
use runtime_provider_kit::{Identity, names};
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
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{net::TcpStream, task::JoinHandle, time::timeout};
use tokio_rustls::TlsConnector;
use x509_parser::{extensions::GeneralName, prelude::parse_x509_certificate};
pub const INSTANCE: &str =
    "spiffe://gate.test/conduit/instance/aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
const ENGINE: &str = "spiffe://gate.test/conduit/engine";
const WAIT: Duration = Duration::from_secs(5);
pub struct Fixture {
    pub identity: Identity,
    pub root: CertificateDer<'static>,
    signing: SigningKey,
}
impl Fixture {
    pub fn new(instance: &str) -> Self {
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
                environment: BTreeMap::from([
                    (
                        names::ENGINE_URL_VARIABLE.into(),
                        "https://engine.invalid/mcp".into(),
                    ),
                    (
                        names::IDENTITY_DIR_VARIABLE.into(),
                        "/unmaterialized".into(),
                    ),
                ]),
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
    pub fn token(&self) -> String {
        self.custom(json!({"alg":"ES256","kid":"store"}), claims())
    }
    fn custom(&self, header: Value, claims: Value) -> String {
        sign(&self.signing, header, claims)
    }
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
pub struct Client {
    sender: hyper::client::conn::http1::SendRequest<Full<Bytes>>,
    task: JoinHandle<()>,
}
impl Drop for Client {
    fn drop(&mut self) {
        self.task.abort();
    }
}
impl Client {
    pub async fn connect(
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
    pub async fn send(
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
fn sign(key: &SigningKey, header: Value, claims: Value) -> String {
    let message = format!(
        "{}.{}",
        B64.encode(header.to_string()),
        B64.encode(claims.to_string())
    );
    let signature: Signature = key.sign(message.as_bytes());
    format!("{message}.{}", B64.encode(signature.to_bytes()))
}
fn claims() -> Value {
    json!({"sub":ENGINE,"aud":[INSTANCE],"exp":SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()+600})
}
