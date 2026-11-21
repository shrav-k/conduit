//! R-1 qualification against the installed local-process runtime and its real TLS gate.
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use conduit_shell::{Config, MissingExecutor, Shell, boundary, runner::Runner};
use p256::{
    ecdsa::{Signature, SigningKey, signature::Signer},
    pkcs8::DecodePrivateKey,
};
use rustls::{
    CertificateError, DigitallySignedStruct, RootCertStore, SignatureScheme,
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    pki_types::{CertificateDer, ServerName, UnixTime, pem::PemObject},
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, OnceLock},
    thread,
    time::{Duration, Instant},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const PACKAGE: &str = "example.org/conduit/provider-local-process";
const HTTPS: &str = "https://engine.test/mcp";
const DOMAIN: &str = "runtime.test";
const WAIT: Duration = Duration::from_secs(45);

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn archive() -> &'static (String, String) {
    static ARCHIVE: OnceLock<(String, String)> = OnceLock::new();
    ARCHIVE.get_or_init(|| {
        let package = repository().join("packages/provider-local-process");
        let output = Command::new("cargo")
            .current_dir(&package)
            .env("CARGO_NET_OFFLINE", "true")
            .args([
                "run",
                "--offline",
                "--locked",
                "--release",
                "--bin",
                "package",
            ])
            .output()
            .expect("could not run offline provider packaging");
        if !output.status.success() {
            // Compiler diagnostics are public. Keep the failure bounded.
            let tail = output.stderr.len().saturating_sub(3000);
            panic!(
                "offline provider packaging failed: {}",
                String::from_utf8_lossy(&output.stderr[tail..])
            );
        }
        let path = package.join("dist/provider-local-process.tar.gz");
        let digest = conduit_core::packages::bytes_digest(&fs::read(&path).unwrap());
        (format!("{}#{digest}", path.display()), digest)
    })
}

#[track_caller]
fn until(mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + WAIT;
    while !ready() {
        assert!(Instant::now() < deadline, "progress deadline");
        thread::sleep(Duration::from_millis(10));
    }
}

struct Engine {
    runner: Runner,
    shell: Arc<Shell>,
}
fn observing_guard(store: &Path) -> PathBuf {
    // The provider omits outputs from records. Observe the URL when the runner
    // passes it to the bridge, using the same guard and process group as usual.
    let wrapper = store.parent().unwrap().join("guard");
    let binary = env!("CARGO_BIN_EXE_conduit").replace('\'', "'\\''");
    fs::write(
        &wrapper,
        format!(
            "#!/bin/sh\nif [ \"$1\" = __run-guard ] && [ \"${{4-}}\" = __http-bridge ]; then\n  printf '%s\\n' \"$5\" > ../bridge-url\nfi\nexec '{binary}' \"$@\"\n"
        ),
    )
    .unwrap();
    fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o700)).unwrap();
    wrapper
}

impl Engine {
    fn open(store: &Path) -> Self {
        let shell = Arc::new(
            Shell::open(
                store,
                boundary::context(&repository()).unwrap(),
                Config {
                    trust_domain: Some(DOMAIN.into()),
                    engine_https_url: Some(HTTPS.into()),
                    initialize_timeout: Duration::from_secs(10),
                    call_timeout: Duration::from_secs(10),
                    stop_grace: Duration::from_millis(500),
                    ..Config::default()
                },
            )
            .unwrap(),
        );
        let runner = Runner::new(&shell, observing_guard(store)).unwrap();
        Self { runner, shell }
    }

    fn object(&self, name: &str) -> Value {
        self.shell
            .state()
            .unwrap()
            .objects
            .iter()
            .find(|object| object["name"] == name && object.get("spec").is_none())
            .cloned()
            .expect("test instance missing")
    }

    fn id(&self, name: &str) -> String {
        self.object(name)["id"].as_str().unwrap().to_owned()
    }

    fn invoke(&self, tool: &str, arguments: Value) -> Value {
        let ticket = self.shell.begin(json!({"kind":"invoke","request":{
            "identity":{"kind":"instance","id":self.id("root")},"tool":tool,"arguments":arguments
        }}), &self.runner).unwrap();
        until(|| ticket.try_reply().unwrap().is_some());
        ticket.try_reply().unwrap().unwrap()
    }

    fn setup(&self, name: &str) -> Value {
        self.invoke(
            "s1.set-up.v1",
            json!({"instance":{"kind":"Agent","name":name}}),
        )
    }

    fn outcome(&self, reply: &Value) -> Value {
        reply["_meta"][format!("{}outcome", self.shell.context.meta_prefix)].clone()
    }

    #[track_caller]
    fn succeeds(&self, reply: &Value) {
        assert_eq!(
            self.outcome(reply),
            "succeeded",
            "operation did not succeed: {reply}; {:?}",
            self.runner.diagnostics()
        );
    }

    fn turn(&self, name: &str) {
        let reply = self.invoke(&format!("{name}-turn.ping.v1"), json!({}));
        self.succeeds(&reply);
        assert_eq!(reply["structuredContent"], json!({"reply":"scripted turn"}));
    }

    fn calls(&self, agent: &str, operation: &str, phase: &str) -> Vec<Value> {
        self.calls_for_id(&self.id(agent), operation, phase)
    }

    fn calls_for_id(&self, id: &str, operation: &str, phase: &str) -> Vec<Value> {
        let qualified = format!("{}#{operation}", self.shell.context.core_package);
        self.shell
            .state()
            .unwrap()
            .records
            .iter()
            .filter(|r| {
                r["type"] == "provider-call"
                    && r["subject"] == id
                    && r["data"]["operation"] == qualified
                    && r["data"]["phase"] == phase
            })
            .map(|r| r["data"].clone())
            .collect()
    }

    fn last_inspected_run_state(&self, agent: &str) -> Value {
        let id = self.id(agent);
        let operation = format!("{}#inspect-run.v1", self.shell.context.core_package);
        self.shell
            .state()
            .unwrap()
            .records
            .iter()
            .rev()
            .find(|r| {
                r["type"] == "object-change"
                    && r["data"]["target"] == id
                    && r["data"]["action"] == "setup-part-reported"
                    && r["data"]["partOperation"] == operation
            })
            .map_or(Value::Null, |r| r["data"]["runState"].clone())
    }

    fn running(&self, name: &str) -> Value {
        self.object(name)["status"]["conditions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["type"] == "Running")
            .unwrap()
            .clone()
    }
}

struct Fixture {
    engine: Option<Engine>,
    temp: tempfile::TempDir,
}
impl Fixture {
    fn new(builtin: bool) -> Self {
        let (argument, digest) = archive();
        let temporary_root = std::env::temp_dir().canonicalize().unwrap();
        let temp = tempfile::tempdir_in(temporary_root).unwrap();
        let engine = Engine::open(&temp.path().join("store"));
        let shell = &engine.shell;
        shell.submit(json!({"kind":"first-start","machine":conduit_shell::local_machine(BTreeMap::new()).unwrap()}), &MissingExecutor).unwrap();
        boundary::install_package(shell, &repository(), argument).unwrap();
        let base = repository().join("conformance/conventions/setup/review-relative-provider");
        let documents: Value =
            serde_json::from_slice(&fs::read(base.join("documents.json")).unwrap()).unwrap();
        let mut documents = documents.as_array().unwrap().clone();
        documents.retain(|doc| doc["metadata"]["name"] != "provider.v1");
        let api = documents[0]["apiVersion"].clone();
        for doc in &mut documents {
            match doc["metadata"]["name"].as_str().unwrap() {
                "test.v1" => {
                    doc["spec"]["schema"] = json!({"type":"object","required":["observations"],
                        "properties":{"observations":{"type":"string"}}});
                }
                "agent" => doc["spec"]["config"] = json!({"observations":temp.path().join("a")}),
                "provider-template" => {
                    doc["spec"]["type"] = json!(format!("{PACKAGE}#local-process.v1"));
                    let durable = temp.path().join("durable");
                    fs::create_dir(&durable).unwrap();
                    fs::set_permissions(&durable, fs::Permissions::from_mode(0o700)).unwrap();
                    doc["spec"]["config"]["durableDir"] = json!(durable);
                }
                _ => {}
            }
        }
        documents.push(
            json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"turn-template"},
            "spec":{"capability":"#test-cap.v1","operations":["#ping.v1"]}}),
        );
        if builtin {
            documents.extend([
                json!({"apiVersion":api,"kind":"Agent","metadata":{"name":"builtin-template"},
                    "spec":{"type":"#test.v1","config":{"observations":temp.path().join("builtin")}}}),
                json!({"apiVersion":api,"kind":"Agent","metadata":{"name":"builtin"},
                    "from":"builtin-template","owner":"s1"}),
            ]);
        }
        let agents = if builtin {
            &["a", "builtin"][..]
        } else {
            &["a"][..]
        };
        for name in agents {
            fs::create_dir(temp.path().join(name)).unwrap();
            documents.push(
                json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":format!("{name}-turn")},
                "from":"turn-template","consumer":{"kind":"Interface","name":"root"},
                "target":{"kind":"Agent","name":name}}),
            );
        }
        let mut manifest = boundary::value(&base.join("manifest.json")).unwrap();
        manifest["packages"] = json!({PACKAGE:{"version":"0.1.0","digest":digest}});
        manifest["implementations"]
            .as_object_mut()
            .unwrap()
            .remove("provider-server");
        manifest["implementations"]["relative"] = json!({
            "implements":["#ping.v1"], "plain":true,
            "tools":{"#ping.v1":"ping.v1"},
            "command":[build_scripted_server(temp.path())],
            "environment":{"TEST_OBSERVATIONS":"/observations"}
        });
        let root = engine.id("root");
        let state = shell.state().unwrap();
        let route = state
            .objects
            .iter()
            .find(|o| o["local_name"] == "management" && o["consumer"] == root)
            .unwrap();
        let reply = shell
            .submit(
                json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,
            "project":{"manifest":manifest,"documents":documents,"directory":null},
            "request":{"sender":root,"caller":root,"conduit":route["id"],"target":route["target"],
                "operation":format!("{}#register.v1",shell.context.core_package)}}),
                &MissingExecutor,
            )
            .unwrap();
        assert_eq!(
            reply["accepted"], true,
            "local-process fixture was not admitted: {reply}"
        );
        let provider = engine.object("provider");
        assert_eq!(provider["placement"]["id"], engine.id("local"));
        engine.succeeds(&engine.invoke(
            "system.set-up.v1",
            json!({"instance":{"kind":"Resource","name":"provider"}}),
        ));
        engine.succeeds(&engine.invoke(
            "management.set-up.v1",
            json!({"instance":{"kind":"Fleet","name":"s1"}}),
        ));
        for name in agents {
            let conduit = format!("{name}-turn");
            engine.succeeds(&engine.invoke("access.open.v1", json!({"conduit":conduit})));
        }
        Self {
            engine: Some(engine),
            temp,
        }
    }

    fn engine(&self) -> &Engine {
        self.engine.as_ref().unwrap()
    }

    fn provider_dir(&self) -> PathBuf {
        self.temp
            .path()
            .join("store/instances")
            .join(self.engine().id("provider"))
    }

    fn hash(&self) -> String {
        format!("{:x}", Sha256::digest(self.engine().id("a")))
    }

    fn observation(&self, name: &str) -> Value {
        let report = self.temp.path().join(name);
        let line = |name| fs::read_to_string(report.join(name)).unwrap();
        json!({"pid":line("pid").parse::<i32>().unwrap(), "url":line("url"), "state_dir":line("state-dir"),
            "identity":{"tls.crt":report.join("has-cert").is_file(),"tls.key":report.join("has-key").is_file()}})
    }

    fn events(&self, name: &str) -> Vec<String> {
        fs::read_to_string(self.temp.path().join(name).join("methods"))
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect()
    }

    fn assert_provider_location(&self, observation: &Value) {
        let state = self.provider_dir().join("instances").join(self.hash());
        assert!(
            observation["state_dir"] == json!(state),
            "run state directory is outside the provider layout"
        );
        assert!(
            !self
                .temp
                .path()
                .join("store/instances")
                .join(self.engine().id("a"))
                .exists()
        );
        assert!(
            observation["url"] == HTTPS,
            "run did not receive the configured HTTPS address"
        );
        assert_eq!(observation["identity"]["tls.crt"], true);
        assert_eq!(observation["identity"]["tls.key"], true);
    }
}

fn build_scripted_server(directory: &Path) -> PathBuf {
    // The kit expects only JSON-RPC frames on stdout. A libtest child writes its own lines.
    let source = directory.join("scripted.rs");
    let program = directory.join("scripted");
    fs::write(&source, r####"
use std::{env, fs, io::{self, BufRead, Write}, path::PathBuf, thread, time::Duration};

// The clients send these fields as simple, unescaped strings and integer request IDs.
fn field<'a>(line: &'a str, name: &str) -> &'a str {
    line.split_once(&format!("\"{name}\"")).unwrap().1
        .trim_start().strip_prefix(':').unwrap().trim_start()
}
fn string<'a>(line: &'a str, name: &str) -> &'a str {
    field(line, name).strip_prefix('"').unwrap().split_once('"').unwrap().0
}
fn main() {
    let report = PathBuf::from(std::env::var_os("TEST_OBSERVATIONS").unwrap());
    let identity = PathBuf::from(std::env::var_os("CONDUIT_IDENTITY_DIR").unwrap());
    fs::write(report.join("pid"), std::process::id().to_string()).unwrap();
    fs::write(report.join("url"), env::var("CONDUIT_URL").unwrap()).unwrap();
    fs::write(report.join("state-dir"), env::var("CONDUIT_STATE_DIR").unwrap()).unwrap();
    for (source, marker) in [("tls.crt", "has-cert"), ("tls.key", "has-key")] {
        if identity.join(source).is_file() { fs::write(report.join(marker), []).unwrap(); }
    }
    let exit = report.join("exit");
    thread::spawn(move || loop {
        if fs::remove_file(&exit).is_ok() { std::process::exit(0); }
        thread::sleep(Duration::from_millis(10));
    });
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let method = string(&line, "method");
        writeln!(fs::OpenOptions::new().create(true).append(true).open(report.join("methods")).unwrap(), "{method}").unwrap();
        let result = match method {
            "initialize" => format!(r#"{{"protocolVersion":"{}","serverInfo":{{"name":"scripted local process","version":"1"}},"capabilities":{{"tools":{{}}}}}}"#, string(&line, "protocolVersion")),
            "tools/list" => r#"{"tools":[{"name":"ping.v1","inputSchema":{"type":"object"}}]}"#.to_owned(),
            "tools/call" => {
                assert!(string(&line, "name") == "ping.v1");
                r#"{"content":[],"structuredContent":{"reply":"scripted turn"}}"#.to_owned()
            }
            _ => continue,
        };
        let id = field(&line, "id").split([',', '}']).next().unwrap().trim();
        assert!(id.parse::<u64>().is_ok());
        println!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#);
        io::stdout().flush().unwrap();
    }
}
"####).unwrap();
    let output = Command::new("rustc")
        .arg(&source)
        .arg("-o")
        .arg(&program)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "could not build the scripted server: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    program
}

#[test]
fn local_process_runs_an_agent_turn() {
    let fixture = Fixture::new(false);
    let engine = fixture.engine();
    let setup = engine.setup("a");
    assert_eq!(
        engine.last_inspected_run_state("a"),
        "stopped",
        "the provider must find no run before first setup"
    );
    engine.succeeds(&setup);
    engine.turn("a");
    fixture.assert_provider_location(&fixture.observation("a"));
    let export = engine
        .shell
        .submit(json!({"kind":"export-records"}), &MissingExecutor)
        .unwrap();
    let inspect = engine.invoke(
        "s1.inspect.v1",
        json!({"object":{"kind":"Agent","name":"a"}}),
    );
    engine.succeeds(&inspect);
    for value in [export, inspect, json!(engine.runner.diagnostics())] {
        let text = value.to_string();
        assert!(
            !text.contains("BEGIN CERTIFICATE")
                && !text.contains("PRIVATE KEY")
                && !text.contains("eyJhbGci"),
            "a record, inspection or diagnostic contains identity material"
        );
        assert!(!text.contains("\"identity\":"));
    }
}

#[derive(Debug)]
struct RunCertificate {
    roots: RootCertStore,
    subject: String,
    crypto: rustls::crypto::CryptoProvider,
}
impl ServerCertVerifier for RunCertificate {
    fn verify_server_cert(
        &self,
        end: &CertificateDer<'_>,
        chain: &[CertificateDer<'_>],
        _: &ServerName<'_>,
        _: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let cert = rustls::server::ParsedCertificate::try_from(end)?;
        rustls::client::verify_server_cert_signed_by_trust_anchor(
            &cert,
            &self.roots,
            chain,
            now,
            self.crypto.signature_verification_algorithms.all,
        )?;
        let (_, parsed) = x509_parser::parse_x509_certificate(end.as_ref())
            .map_err(|_| CertificateError::BadEncoding)?;
        let san = parsed
            .subject_alternative_name()
            .map_err(|_| CertificateError::BadEncoding)?;
        if !parsed.is_ca()
            && san.is_some_and(|san| {
                matches!(san.value.general_names.as_slice(),
            [x509_parser::extensions::GeneralName::URI(uri)] if *uri == self.subject)
            })
        {
            Ok(ServerCertVerified::assertion())
        } else {
            Err(CertificateError::NotValidForName.into())
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        signature: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            signature,
            &self.crypto.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        signature: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            signature,
            &self.crypto.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.crypto
            .signature_verification_algorithms
            .supported_schemes()
    }
}

fn own_test_proof(subject: &str) -> String {
    let generated = rcgen::KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256).unwrap();
    let signer = SigningKey::from_pkcs8_der(&generated.serialize_der()).unwrap();
    let header = B64.encode(br#"{"alg":"ES256","typ":"JWT","kid":"store"}"#);
    let claims = B64.encode(
        serde_json::to_vec(&json!({"sub":format!("spiffe://{DOMAIN}/conduit/engine"),
        "aud":[subject],"exp":chrono::Utc::now().timestamp() + 300}))
        .unwrap(),
    );
    let body = format!("{header}.{claims}");
    let signature: Signature = signer.sign(body.as_bytes());
    format!("{body}.{}", B64.encode(signature.to_bytes()))
}

async fn refused_initialize(
    connector: &tokio_rustls::TlsConnector,
    port: u16,
    proof: Option<&str>,
) {
    const REQUEST: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","clientInfo":{"name":"gate test","version":"1"},"capabilities":{}}}"#;
    let socket = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .unwrap();
    let mut stream = connector
        .connect(ServerName::try_from("127.0.0.1").unwrap(), socket)
        .await
        .expect("gate failed to prove the run with the store root");
    let auth = proof
        .map(|signed| format!("Authorization: Bearer {signed}\r\n"))
        .unwrap_or_default();
    let post = format!(
        "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\n{auth}Content-Length: {}\r\n\r\n{REQUEST}",
        REQUEST.len()
    );
    stream.write_all(post.as_bytes()).await.unwrap();
    let mut response = [0; 12];
    stream.read_exact(&mut response).await.unwrap();
    assert!(
        &response == b"HTTP/1.1 401",
        "gate did not refuse unauthorized initialization"
    );
}

#[test]
fn local_process_gate_admits_only_the_engine() {
    let fixture = Fixture::new(false);
    let engine = fixture.engine();
    engine.succeeds(&engine.setup("a"));
    let methods = fixture.events("a");
    assert!(methods.iter().any(|m| m == "initialize"));

    let url = fs::read_to_string(fixture.temp.path().join("bridge-url")).unwrap();
    let endpoint = url::Url::parse(url.trim()).unwrap();
    assert_eq!(
        (endpoint.scheme(), endpoint.host_str(), endpoint.path()),
        ("https", Some("127.0.0.1"), "/mcp")
    );
    let ca = fixture
        .provider_dir()
        .join("runs")
        .join(fixture.hash())
        .join("identity/ca.crt");
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from_pem_file(ca).unwrap())
        .unwrap();
    let subject = format!("spiffe://{DOMAIN}/conduit/instance/{}", engine.id("a"));
    let crypto = rustls::crypto::ring::default_provider();
    let config = rustls::ClientConfig::builder_with_provider(Arc::new(crypto.clone()))
        .with_safe_default_protocol_versions()
        .unwrap()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(RunCertificate {
            roots,
            subject: subject.clone(),
            crypto,
        }))
        .with_no_client_auth();
    let connector = tokio_rustls::TlsConnector::from(Arc::new(config));
    let proof = own_test_proof(&subject);
    let port = endpoint.port().unwrap();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            tokio::time::timeout(Duration::from_secs(5), async {
                refused_initialize(&connector, port, None).await;
                refused_initialize(&connector, port, Some(&proof)).await;
            })
            .await
            .expect("gate requests did not finish");
        });
    engine.turn("a");
    assert_eq!(
        fixture
            .events("a")
            .iter()
            .filter(|m| *m == "initialize")
            .count(),
        methods.iter().filter(|m| *m == "initialize").count()
    );
}

fn pid(observation: &Value) -> i32 {
    observation["pid"].as_i64().unwrap() as i32
}

fn absent(pid: i32) -> bool {
    (unsafe { libc::kill(pid, 0) }) == -1
        && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
}

#[test]
fn local_process_run_state_follows_the_provider() {
    let fixture = Fixture::new(false);
    let engine = fixture.engine();
    engine.succeeds(&engine.setup("a"));
    let condition = engine.running("a");
    assert_eq!(condition["status"], "True");
    assert_eq!(
        condition["source"],
        format!("urn:uuid:{}", engine.id("provider"))
    );
    let first = pid(&fixture.observation("a"));
    fs::write(fixture.temp.path().join("a/exit"), []).unwrap();
    until(|| absent(first));
    assert_eq!(pid(&fixture.observation("a")), first);
    assert_eq!(engine.calls("a", "run.v1", "made").len(), 1);

    engine.succeeds(&engine.setup("a"));
    let second = pid(&fixture.observation("a"));
    assert_ne!(first, second);
    assert_eq!(engine.calls("a", "run.v1", "made").len(), 2);
    assert_eq!(engine.last_inspected_run_state("a"), "stopped");
    engine.turn("a");
    let id = engine.id("a");
    let private = fixture.provider_dir().join("runs").join(fixture.hash());
    engine.succeeds(&engine.invoke(
        "s1.retire.v1",
        json!({"instance":{"kind":"Agent","name":"a"}}),
    ));
    assert_eq!(
        engine.calls_for_id(&id, "stop.v1", "ended")[0]["outcome"],
        "succeeded"
    );
    assert!(
        !engine
            .shell
            .state()
            .unwrap()
            .objects
            .iter()
            .any(|o| o["id"] == id)
    );
    until(|| absent(first) && absent(second));
    assert!(!private.exists());
}

#[test]
fn local_process_provider_runs_again_after_restart() {
    let mut fixture = Fixture::new(false);
    let engine = fixture.engine();
    engine.succeeds(&engine.setup("a"));
    engine.turn("a");
    let before = pid(&fixture.observation("a"));
    drop(fixture.engine.take());
    until(|| absent(before));

    fixture.engine = Some(Engine::open(&fixture.temp.path().join("store")));
    assert_eq!(fixture.engine().runner.running_count(), 0);
    let engine = fixture.engine();
    let reply = engine.setup("a");
    assert_eq!(
        engine.outcome(&reply),
        "succeeded",
        "agent setup after engine restart: {reply}; {:?}",
        engine.runner.diagnostics()
    );
    assert_eq!(engine.last_inspected_run_state("a"), "stopped");
    assert_ne!(pid(&fixture.observation("a")), before);
    assert_eq!(engine.calls("a", "run.v1", "made").len(), 2);
    engine.turn("a");
}

#[test]
fn local_process_upgrade_with_ambiguous_history_requires_explicit_provider_setup() {
    let mut fixture = Fixture::new(false);
    let engine = fixture.engine();
    let provider = engine.id("provider");
    engine.succeeds(&engine.setup("a"));
    let made_before = engine.calls("a", "inspect-run.v1", "made").len();
    let before = pid(&fixture.observation("a"));
    drop(fixture.engine.take());
    until(|| absent(before));

    // Order 221 stored the setup and ordinary shutdown observation, but
    // did not track the launch or end cause of a non-secret runtime provider.
    let store = fixture.temp.path().join("store");
    let path = store.join("state.json");
    let mut stored: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let mut removed_ledger = false;
    for invocation in stored["state"]["invocations"].as_array_mut().unwrap() {
        if invocation["object"] == provider {
            let fields = invocation.as_object_mut().unwrap();
            removed_ledger |= fields.remove("secret_provider_run").is_some();
            fields.remove("secret_provider_launch");
            fields.remove("secret_provider_end");
        }
    }
    assert!(removed_ledger);
    fs::write(&path, serde_json::to_vec(&stored).unwrap()).unwrap();

    fixture.engine = Some(Engine::open(&store));
    let engine = fixture.engine();
    assert_eq!(engine.runner.running_count(), 0);
    engine.setup("a");
    let agent = engine.object("a");
    let part = agent["status"]["setup"]["parts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|part| part["name"] == "run")
        .unwrap();
    assert_eq!(part["outcome"], "unavailable");
    assert_eq!(part["diagnostics"][0]["ruleId"], "PRV-026");
    let message = part["diagnostics"][0]["message"]["text"].as_str().unwrap();
    assert!(message.contains("needs explicit setup"), "{message}");
    assert!(message.contains("ended at engine shutdown"));
    assert!(message.contains(&provider));
    assert_eq!(
        engine.calls("a", "inspect-run.v1", "made").len(),
        made_before
    );
    assert_eq!(engine.runner.running_count(), 0);

    engine.succeeds(&engine.invoke(
        "system.set-up.v1",
        json!({"instance":{"kind":"Resource","name":"provider"}}),
    ));
    engine.succeeds(&engine.setup("a"));
    engine.turn("a");
}

#[test]
fn builtin_and_local_process_runs_side_by_side() {
    let fixture = Fixture::new(true);
    let engine = fixture.engine();
    for name in ["a", "builtin"] {
        engine.succeeds(&engine.setup(name));
        engine.turn(name);
    }
    fixture.assert_provider_location(&fixture.observation("a"));
    let builtin = fixture.observation("builtin");
    assert!(
        builtin["url"] == Config::default().engine_url,
        "built-in run did not receive the loopback address"
    );
    assert!(
        builtin["state_dir"]
            == json!(
                fixture
                    .temp
                    .path()
                    .join("store/instances")
                    .join(engine.id("builtin"))
            ),
        "built-in run state directory is outside the engine layout"
    );
    assert_eq!(builtin["identity"]["tls.crt"], false);
    assert_eq!(builtin["identity"]["tls.key"], false);
    assert!(engine.calls("builtin", "run.v1", "made").is_empty());
}
