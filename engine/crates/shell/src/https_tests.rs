use super::*;
use crate::server::{self, ServingExecutor};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use tokio_util::sync::CancellationToken;

const ADDRESS: &str = "https://engine.example:8443/mcp";
const ROOTS: &str = "-----BEGIN CERTIFICATE-----\nAQID\n-----END CERTIFICATE-----\n";

fn configured() -> Config {
    Config {
        engine_https_url: Some(ADDRESS.into()),
        ..Config::default()
    }
}

fn claims(token: &str) -> Value {
    serde_json::from_slice(
        &URL_SAFE_NO_PAD
            .decode(token.split('.').nth(1).unwrap())
            .unwrap(),
    )
    .unwrap()
}

fn run_token(identity: &RunIdentity) -> String {
    fs::read_to_string(identity.directory.join("token")).unwrap()
}

#[test]
fn https_address_is_canonical_fixed_and_loaded_without_rewriting() {
    let temp = temporary();
    let shell = Shell::open(temp.path(), context(), configured()).unwrap();
    let path = temp.path().join("engine-https-url.json");
    assert_eq!(
        serde_json::from_slice::<String>(&fs::read(&path).unwrap()).unwrap(),
        ADDRESS
    );
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let modified = fs::metadata(&path).unwrap().modified().unwrap();
    drop(shell);
    drop(Shell::open(temp.path(), context(), configured()).unwrap());
    assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), modified);
    let config = Config::for_store(temp.path()).unwrap();
    assert_eq!(config.engine_https_url.as_deref(), Some(ADDRESS));
    let shell = Shell::open(temp.path(), context(), Config::default()).unwrap();
    assert_eq!(shell.config.engine_https_url.as_deref(), Some(ADDRESS));
    drop(shell);
    let mut other = configured();
    other.engine_https_url = Some("https://other.example/mcp".into());
    let error = Shell::open(temp.path(), context(), other).err().unwrap();
    assert!(
        error.contains(ADDRESS)
            && error.contains("https://other.example/mcp")
            && error.contains("store fixes the https token audience")
    );
    assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), modified);
}

#[test]
fn https_invalid_addresses_write_no_address_or_roots() {
    for value in [
        "http://engine.example/mcp",
        "https://localhost/mcp",
        "https://127.0.0.1/mcp",
        "https://127.2.3.4/mcp",
        "https://[::1]/mcp",
        "https://[::ffff:127.0.0.1]/mcp",
        "https://engine.example/other",
        "https://engine.example/mcp?query",
        "https://engine.example/mcp#fragment",
        "https://user@engine.example/mcp",
        "https://user:password@engine.example/mcp",
        "https://engine.example:0/mcp",
        "https://ENGINE.example/mcp",
        "https://engine.example:443/mcp",
    ] {
        let temp = temporary();
        let config = Config {
            engine_https_url: Some(value.into()),
            https_roots: Some(ROOTS.into()),
            ..Config::default()
        };
        let error = Shell::open(temp.path(), context(), config).err().unwrap();
        assert!(error.contains("--https-url"), "{value}: {error}");
        if value.contains("ENGINE") || value.contains(":443") {
            assert!(error.contains("canonical form is https://engine.example/mcp"));
        }
        assert!(!temp.path().join("engine-https-url.json").exists());
        assert!(!temp.path().join("engine-https-roots.pem").exists());
    }
}

#[test]
fn https_writer_lease_protects_both_new_and_existing_files() {
    for existing in [false, true] {
        let temp = temporary();
        let config = if existing {
            Config {
                https_roots: Some(ROOTS.into()),
                ..configured()
            }
        } else {
            Config::default()
        };
        let shell = Shell::open(temp.path(), context(), config).unwrap();
        let files = ["engine-https-url.json", "engine-https-roots.pem"];
        let before = files.map(|f| fs::read(temp.path().join(f)).ok());
        let attempt = Config {
            https_roots: Some(ROOTS.repeat(2)),
            ..configured()
        };
        assert_eq!(
            Shell::open(temp.path(), context(), attempt).err().unwrap(),
            "store already has an active shell"
        );
        assert_eq!(files.map(|f| fs::read(temp.path().join(f)).ok()), before);
        drop(shell);
    }
}

#[test]
fn https_roots_check_labels_base64_and_whitespace_without_certificate_parsing() {
    assert!(https::roots(&format!(" \n{ROOTS}\n{ROOTS}\t")).is_ok());
    for (value, expected) in [
        ("", "at least one"),
        (" \n", "at least one"),
        (
            "-----BEGIN PRIVATE KEY-----\nAQID\n-----END PRIVATE KEY-----",
            "PRIVATE KEY",
        ),
        (
            "-----BEGIN PUBLIC KEY-----\nAQID\n-----END PUBLIC KEY-----",
            "PUBLIC KEY",
        ),
        ("stray text", "only CERTIFICATE"),
        (
            "-----BEGIN CERTIFICATE-----\n!bad\n-----END CERTIFICATE-----",
            "base64",
        ),
        (
            "-----BEGIN CERTIFICATE-----\n\n-----END CERTIFICATE-----",
            "base64",
        ),
        ("-----BEGIN CERTIFICATE-----\nAQID", "incomplete"),
    ] {
        assert!(https::roots(value).unwrap_err().contains(expected));
    }
    assert!(https::roots(&format!("{ROOTS}stray text")).is_err());
    let malformed = "-----BEGIN PRIVATE KEY\nAQID\n-----END PRIVATE KEY-----";
    let error = https::roots(malformed).unwrap_err();
    assert!(error.contains("incomplete PEM label"));
    assert!(!error.contains("AQID"));
    let temp = temporary();
    let content = format!(" \n{ROOTS}\n{ROOTS}\t");
    drop(
        Shell::open(
            temp.path(),
            context(),
            Config {
                https_roots: Some(content.clone()),
                ..configured()
            },
        )
        .unwrap(),
    );
    assert_eq!(
        fs::read_to_string(temp.path().join("engine-https-roots.pem")).unwrap(),
        content
    );
}

#[test]
fn https_listen_validation_requires_distinct_exact_loopback_addresses() {
    for listen in ["127.0.0.1:3443", "[::1]:3443"] {
        let config = Config {
            https_listen: Some(listen.parse().unwrap()),
            ..configured()
        };
        config.validate_serve().unwrap();
    }
    for listen in [
        "127.0.0.2:3443",
        "0.0.0.0:3443",
        "[::]:3443",
        "[::ffff:127.0.0.1]:3443",
        "127.0.0.1:0",
        "127.0.0.1:3000",
    ] {
        let config = Config {
            https_listen: Some(listen.parse().unwrap()),
            ..configured()
        };
        assert!(config.validate_serve().is_err());
    }
    let config = Config {
        engine_url: "http://localhost:3443/mcp".into(),
        https_listen: Some("127.0.0.1:3443".parse().unwrap()),
        ..configured()
    };
    assert!(
        config
            .validate_serve()
            .unwrap_err()
            .contains("first listener")
    );
    let temp = temporary();
    let config = Config {
        engine_https_url: Some("https://[2001:db8::1]:8443/mcp".into()),
        ..Config::default()
    };
    drop(Shell::open(temp.path(), context(), config).unwrap());
}

#[test]
fn https_materialization_follows_runner_and_helpers_keep_loopback_audience() {
    let temp = temporary();
    drop(established(temp.path()));
    let config = Config {
        https_roots: Some(ROOTS.into()),
        ..configured()
    };
    let shell = Shell::open(temp.path(), context(), config).unwrap();
    let id = actor(&shell, "agent-a");
    let desc = descriptor(&shell, &id);
    let own = shell.materialize_identity(&desc).unwrap();
    let state_before = fs::read(temp.path().join("state.json")).unwrap();
    let helper = temp.path().join("client-tokens").join(&id);
    assert_eq!(
        claims(&fs::read_to_string(&helper).unwrap())["aud"],
        json!([shell.config.engine_url])
    );
    let own_root = fs::read_to_string(own.directory.join("ca.crt")).unwrap();
    assert_eq!(
        own.environment
            .values()
            .filter(|v| *v == &shell.config.engine_url)
            .count(),
        1
    );
    assert_eq!(
        claims(&run_token(&own))["aud"],
        json!([shell.config.engine_url])
    );
    let provider = shell.materialize_provider_identity(&desc).unwrap().unwrap();
    assert!(provider.environment.values().any(|v| v == ADDRESS));
    assert_eq!(claims(&run_token(&provider))["aud"], json!([ADDRESS]));
    assert_eq!(
        fs::read_to_string(provider.directory.join("ca.crt")).unwrap(),
        format!("{own_root}{ROOTS}")
    );
    assert_eq!(
        claims(&fs::read_to_string(helper).unwrap())["aud"],
        json!([shell.config.engine_url])
    );
    assert_eq!(
        fs::read(temp.path().join("state.json")).unwrap(),
        state_before
    );
    let root = actor(&shell, "root");
    shell.issue_client_token(&root).unwrap();
    assert_eq!(
        claims(&fs::read_to_string(temp.path().join("client-tokens").join(&root)).unwrap())["aud"],
        json!([shell.config.engine_url])
    );
    assert!(
        shell
            .materialize_provider_identity(&descriptor(&shell, &root))
            .err()
            .unwrap()
            .contains("root cannot receive")
    );
    assert_eq!(
        fs::read_to_string(temp.path().join("engine-https-roots.pem")).unwrap(),
        ROOTS
    );
    assert_eq!(
        fs::metadata(temp.path().join("engine-https-roots.pem"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    drop(shell);
    let shell = Shell::open(
        temp.path(),
        context(),
        Config {
            https_roots: Some(ROOTS.repeat(2)),
            ..Config::default()
        },
    )
    .unwrap();
    let later = shell.materialize_provider_identity(&desc).unwrap().unwrap();
    assert_eq!(
        fs::read_to_string(later.directory.join("ca.crt")).unwrap(),
        format!("{own_root}{}", ROOTS.repeat(2))
    );
    assert_eq!(
        fs::read_to_string(provider.directory.join("ca.crt")).unwrap(),
        format!("{own_root}{ROOTS}")
    );
    drop(shell);
    assert_eq!(
        Config::for_store(temp.path())
            .unwrap()
            .https_roots
            .as_deref(),
        Some(ROOTS.repeat(2).as_str())
    );
}

#[test]
fn https_provider_without_address_has_no_identity_but_has_helper() {
    let temp = temporary();
    let shell = established(temp.path());
    let id = actor(&shell, "agent-a");
    assert!(
        shell
            .materialize_provider_identity(&descriptor(&shell, &id))
            .unwrap()
            .is_none()
    );
    assert!(!temp.path().join("runs").exists());
    assert_eq!(
        claims(&fs::read_to_string(temp.path().join("client-tokens").join(&id)).unwrap())["aud"],
        json!([shell.config.engine_url])
    );
    drop(shell);
    let shell = Shell::open(temp.path(), context(), configured()).unwrap();
    let desc = descriptor(&shell, &id);
    let own = shell.materialize_identity(&desc).unwrap();
    let provider = shell.materialize_provider_identity(&desc).unwrap().unwrap();
    assert_eq!(
        fs::read(own.directory.join("ca.crt")).unwrap(),
        fs::read(provider.directory.join("ca.crt")).unwrap()
    );
}

struct Listeners {
    _temp: tempfile::TempDir,
    shell: Arc<Shell>,
    stop: CancellationToken,
    task: tokio::task::JoinHandle<Result<()>>,
    first: String,
    second: String,
    host: String,
}
impl Listeners {
    async fn start(address: &str) -> Self {
        let temp = temporary();
        drop(established(temp.path()));
        let first = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let second = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let engine_url = format!("http://{}/mcp", first.local_addr().unwrap());
        let listen = second.local_addr().unwrap();
        store::atomic_write(
            temp.path(),
            "engine-url.json",
            &serde_json::to_vec(&engine_url).unwrap(),
        )
        .unwrap();
        let shell = Arc::new(
            Shell::open(
                temp.path(),
                context(),
                Config {
                    engine_url: engine_url.clone(),
                    engine_https_url: Some(address.into()),
                    https_listen: Some(listen),
                    ..Config::default()
                },
            )
            .unwrap(),
        );
        drop((first, second));
        let stop = CancellationToken::new();
        let task = tokio::spawn(server::serve(
            shell.clone(),
            Arc::new(ServingExecutor::new(Arc::new(MissingExecutor))),
            stop.clone(),
        ));
        let second = format!("http://{listen}/mcp");
        for url in [&engine_url, &second] {
            tokio::time::timeout(Duration::from_secs(30), async {
                loop {
                    if reqwest::get(url).await.is_ok() {
                        break;
                    }
                    assert!(!task.is_finished(), "serve exited before binding");
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            })
            .await
            .unwrap();
        }
        let url = url::Url::parse(address).unwrap();
        let host = url[url::Position::BeforeHost..url::Position::AfterPort].to_owned();
        Self {
            _temp: temp,
            shell,
            stop,
            task,
            first: engine_url,
            second,
            host,
        }
    }
    async fn finish(self) {
        self.stop.cancel();
        tokio::time::timeout(Duration::from_secs(30), self.task)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
    }
    fn request(&self, second: bool, token: Option<&str>, body: &Value) -> reqwest::RequestBuilder {
        let url = if second { &self.second } else { &self.first };
        let mut request = reqwest::Client::new()
            .post(url)
            .header("accept", "application/json, text/event-stream")
            .header("mcp-protocol-version", "2025-06-18")
            .json(body);
        if second {
            request = request.header("host", &self.host);
        }
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        request
    }
    async fn initialize(&self, second: bool, token: &str) -> String {
        let response = self.request(second, Some(token), &json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"https-test","version":"1"}}})).send().await.unwrap();
        assert_eq!(response.status(), 200);
        let session = response.headers()["mcp-session-id"]
            .to_str()
            .unwrap()
            .to_owned();
        let _ = response.text().await.unwrap();
        let response = self
            .request(
                second,
                Some(token),
                &json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
            )
            .header("mcp-session-id", &session)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 202);
        session
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn https_tokens_admit_only_their_listener_and_sessions_are_separate() {
    let test = Listeners::start(ADDRESS).await;
    let id = actor(&test.shell, "agent-a"); // S1 places the Agent on local.
    let desc = descriptor(&test.shell, &id);
    let own = run_token(&test.shell.materialize_identity(&desc).unwrap());
    let provider = run_token(
        &test
            .shell
            .materialize_provider_identity(&desc)
            .unwrap()
            .unwrap(),
    );
    let list = json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}});
    let mut sessions = Vec::new();
    for (second, token) in [(false, &own), (true, &provider)] {
        let session = test.initialize(second, token).await;
        let response = test
            .request(second, Some(token), &list)
            .header("mcp-session-id", &session)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        assert!(response.text().await.unwrap().contains("tools"));
        sessions.push(session);
        let wrong = test
            .request(!second, Some(token), &list)
            .send()
            .await
            .unwrap();
        assert_eq!(wrong.status(), 401);
        assert!(wrong.headers().contains_key("www-authenticate"));
    }
    let response = test
        .request(true, Some(&provider), &list)
        .header("mcp-session-id", &sessions[0])
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
    test.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn https_refuses_root_own_runner_and_other_audience_with_no_established_identity() {
    let test = Listeners::start(ADDRESS).await;
    let id = actor(&test.shell, "agent-a");
    let own = run_token(
        &test
            .shell
            .materialize_identity(&descriptor(&test.shell, &id))
            .unwrap(),
    );
    let root = actor(&test.shell, "root");
    test.shell.issue_client_token(&root).unwrap();
    let root = fs::read_to_string(test.shell.directory.join("client-tokens").join(&root)).unwrap();
    let other = test
        .shell
        .authority
        .token(&id, "https://other.example/mcp")
        .unwrap();
    let call = json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"project.file.read.v1","arguments":{"path":"hello.txt"}}});
    for token in [&root, &own, &other] {
        let before = test.shell.state().unwrap().records.len();
        let response = test.request(true, Some(token), &call).send().await.unwrap();
        assert_eq!(response.status(), 401);
        assert!(response.headers().contains_key("www-authenticate"));
        let state = test.shell.state().unwrap();
        let ended: Vec<_> = state.records[before..]
            .iter()
            .filter(|r| r["type"] == "request-ended")
            .collect();
        assert_eq!(ended.len(), 1);
        assert_eq!(ended[0]["data"]["reason"], "refused");
        assert!(ended[0]["data"]["caller"].is_null());
    }
    test.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn https_host_origin_and_authenticated_fallback_rules() {
    let test = Listeners::start(ADDRESS).await;
    let desc = descriptor(&test.shell, &actor(&test.shell, "agent-a"));
    let token = run_token(
        &test
            .shell
            .materialize_provider_identity(&desc)
            .unwrap()
            .unwrap(),
    );
    let own = run_token(&test.shell.materialize_identity(&desc).unwrap());
    let session = test.initialize(true, &token).await;
    let list = json!({"jsonrpc":"2.0","id":3,"method":"tools/list","params":{}});
    for host in [
        url::Url::parse(&test.second).unwrap().authority(),
        "localhost",
        "other.example",
        "engine.example",
        "engine.example:9443",
    ]
    .map(str::to_owned)
    {
        let response = reqwest::Client::new()
            .post(&test.second)
            .bearer_auth(&token)
            .header("accept", "application/json, text/event-stream")
            .header("mcp-protocol-version", "2025-06-18")
            .json(&list)
            .header("mcp-session-id", &session)
            .header("host", host)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 403);
    }
    let response = test
        .request(false, Some(&own), &list)
        .header("host", &test.host)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 403);
    for (origin, status) in [
        ("https://engine.example:8443", 200),
        ("https://other.example", 403),
    ] {
        let response = test
            .request(true, Some(&token), &list)
            .header("mcp-session-id", &session)
            .header("origin", origin)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), status);
    }
    for path in ["/", "/.well-known/oauth-protected-resource/mcp"] {
        let url = format!("{}{}", test.second.strip_suffix("/mcp").unwrap(), path);
        for valid in [false, true] {
            let before = test.shell.state().unwrap().records.len();
            let mut request = reqwest::Client::new().get(&url).header("host", &test.host);
            if valid {
                request = request.bearer_auth(&token);
            }
            let response = request.send().await.unwrap();
            assert_eq!(response.status(), if valid { 404 } else { 401 });
            let state = test.shell.state().unwrap();
            assert_eq!(
                state.records[before..]
                    .iter()
                    .filter(|r| r["type"] == "request-ended")
                    .count(),
                usize::from(valid)
            );
        }
    }
    test.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn https_default_port_accepts_bare_host() {
    let test = Listeners::start("https://engine.example/mcp").await;
    let desc = descriptor(&test.shell, &actor(&test.shell, "agent-a"));
    let token = run_token(
        &test
            .shell
            .materialize_provider_identity(&desc)
            .unwrap()
            .unwrap(),
    );
    let _ = test.initialize(true, &token).await;
    test.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn https_failed_second_bind_names_address_and_releases_first_listener() {
    let temp = temporary();
    drop(established(temp.path()));
    let first = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let occupied = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let first_address = first.local_addr().unwrap();
    let second_address = occupied.local_addr().unwrap();
    let config = Config {
        engine_url: format!("http://{first_address}/mcp"),
        https_listen: Some(second_address),
        ..configured()
    };
    store::atomic_write(
        temp.path(),
        "engine-url.json",
        &serde_json::to_vec(&config.engine_url).unwrap(),
    )
    .unwrap();
    let shell = Arc::new(Shell::open(temp.path(), context(), config).unwrap());
    drop(first);
    let error = server::serve(
        shell,
        Arc::new(ServingExecutor::new(Arc::new(MissingExecutor))),
        CancellationToken::new(),
    )
    .await
    .unwrap_err();
    assert!(error.contains(&second_address.to_string()));
    let _released = std::net::TcpListener::bind(first_address).unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn https_malformed_fallback_calls_require_credentials_on_both_listeners() {
    let test = Listeners::start(ADDRESS).await;
    let malformed = json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{}});
    for second in [false, true] {
        let base = if second { &test.second } else { &test.first };
        for path in ["/", "/.well-known/oauth-protected-resource/mcp"] {
            let url = format!("{}{path}", base.strip_suffix("/mcp").unwrap());
            for token in [None, Some("invalid-token")] {
                let before = test.shell.state().unwrap().records.len();
                let mut request = reqwest::Client::new()
                    .post(&url)
                    .header("accept", "application/json, text/event-stream")
                    .json(&malformed);
                if second {
                    request = request.header("host", &test.host);
                }
                if let Some(token) = token {
                    request = request.bearer_auth(token);
                }
                let response = request.send().await.unwrap();
                assert_eq!(response.status(), 401, "{second}: {path}");
                assert_eq!(
                    response.headers()["www-authenticate"],
                    "Bearer error=\"invalid_token\""
                );
                assert_eq!(test.shell.state().unwrap().records.len(), before);
            }
        }
    }
    test.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn https_authenticated_fallback_records_one_invalid_request_on_both_listeners() {
    let test = Listeners::start(ADDRESS).await;
    let id = actor(&test.shell, "agent-a");
    let desc = descriptor(&test.shell, &id);
    let own = run_token(&test.shell.materialize_identity(&desc).unwrap());
    let provider = run_token(
        &test
            .shell
            .materialize_provider_identity(&desc)
            .unwrap()
            .unwrap(),
    );
    let call = json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"project.file.read.v1","arguments":{"path":"hello.txt"}}});
    let malformed = json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{}});
    for (second, token) in [(false, &own), (true, &provider)] {
        let base = if second { &test.second } else { &test.first };
        for path in ["/", "/.well-known/oauth-protected-resource/mcp"] {
            let url = format!("{}{path}", base.strip_suffix("/mcp").unwrap());
            for body in [None, Some(&call), Some(&malformed)] {
                let before = test.shell.state().unwrap().records.len();
                let client = reqwest::Client::new();
                let request = if let Some(body) = body {
                    client.post(&url).json(body)
                } else {
                    client.get(&url)
                };
                let mut request = request.bearer_auth(token);
                if second {
                    request = request.header("host", &test.host);
                }
                let response = request.send().await.unwrap();
                assert_eq!(response.status(), 404, "{second}: {path}");
                let state = test.shell.state().unwrap();
                for kind in ["access-decision", "request-ended"] {
                    let records: Vec<_> = state.records[before..]
                        .iter()
                        .filter(|r| r["type"] == kind)
                        .collect();
                    assert_eq!(records.len(), 1);
                    // IFC-036/OPS-023: a transport rejection precedes tool access.
                    assert_eq!(records[0]["data"]["reason"], "invalid");
                    assert_eq!(records[0]["data"]["caller"], id);
                }
            }
        }
    }
    test.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn https_malformed_mcp_calls_keep_protocol_errors_without_records() {
    let test = Listeners::start(ADDRESS).await;
    let desc = descriptor(&test.shell, &actor(&test.shell, "agent-a"));
    let token = run_token(
        &test
            .shell
            .materialize_provider_identity(&desc)
            .unwrap()
            .unwrap(),
    );
    let session = test.initialize(true, &token).await;
    let malformed = json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{}});
    for token in [None, Some("invalid-token")] {
        let before = test.shell.state().unwrap().records.len();
        let response = test
            .request(true, token, &malformed)
            .header("mcp-session-id", &session)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let text = response.text().await.unwrap();
        let data = text
            .lines()
            .filter_map(|line| line.strip_prefix("data:"))
            .find(|data| data.trim_start().starts_with('{'))
            .unwrap_or(&text);
        let error: Value = serde_json::from_str(data).unwrap();
        assert_eq!(error["error"]["code"], -32601);
        assert!(error["error"].get("data").is_none());
        assert_eq!(test.shell.state().unwrap().records.len(), before);
    }
    test.finish().await;
}
