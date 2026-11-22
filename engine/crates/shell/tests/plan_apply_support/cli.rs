use super::*;
use std::process::Output;
use tempfile::TempDir;

pub fn binary(directory: &Path, config: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_conduit"));
    command
        .current_dir(directory)
        .env("CONDUIT_CONFIG_DIR", config);
    command
}

pub fn tool(engine: &Engine, directory: &Path, config: &Path, args: &[&str]) -> Output {
    binary(directory, config)
        .args(args)
        .arg("--store")
        .arg(engine.directory())
        .output()
        .unwrap()
}

pub fn archive_arg(args: &Value) -> String {
    let path = url::Url::parse(args["fetch"].as_str().unwrap())
        .unwrap()
        .to_file_path()
        .unwrap();
    format!(
        "{}#{}",
        path.display(),
        args["digest"]
            .as_str()
            .unwrap()
            .trim_start_matches("sha256:")
    )
}

pub fn cache_fixture(config: &Path, stage: u8) {
    fs::create_dir_all(config.join("packages")).unwrap();
    for archive in installs().iter().filter(|archive| archive.stage <= stage) {
        let (path, digest) = archive_arg(&archive.args)
            .rsplit_once('#')
            .map(|(p, d)| (p.to_owned(), d.to_owned()))
            .unwrap();
        fs::copy(path, config.join("packages").join(digest)).unwrap();
    }
}

pub fn project_fixture(dir: &Path, case: &Value, symbols: Option<&Symbols>) {
    for (name, bytes) in case["files"].as_object().unwrap() {
        fs::write(dir.join(name), bytes.as_str().unwrap()).unwrap();
    }
    if let Some(symbols) = symbols
        && !case["applied"].is_null()
    {
        fs::create_dir(dir.join(".conduit")).unwrap();
        fs::write(
            dir.join(".conduit/state.yaml"),
            symbols.kept_for_plan(&case["applied"]).to_string(),
        )
        .unwrap();
    }
}

pub fn stdin_free(output: &Output) -> &str {
    std::str::from_utf8(&output.stdout).unwrap()
}

pub async fn first_case() -> (Engine, TempDir, TempDir, Value) {
    let engine = Engine::start().await;
    let config = tempfile::tempdir().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let installers = tempfile::tempdir().unwrap();
    tokio::task::block_in_place(|| {
        for archive in installs().iter().filter(|archive| archive.stage == 1) {
            let out = tool(
                &engine,
                installers.path(),
                config.path(),
                &["install", &archive_arg(&archive.args)],
            );
            assert!(out.status.success(), "fixture archive install failed");
        }
    });
    let case = cases().remove(0);
    project_fixture(dir.path(), &case, None);
    (engine, config, dir, case)
}

pub fn files(root: &Path) -> BTreeMap<std::path::PathBuf, Vec<u8>> {
    fn visit(dir: &Path, dest: &mut BTreeMap<std::path::PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                visit(&path, dest);
            } else {
                dest.insert(path.clone(), fs::read(path).unwrap());
            }
        }
    }
    let mut result = BTreeMap::new();
    visit(root, &mut result);
    result
}

pub async fn check_foreign_engine(
    engine: &Engine,
    directory: &Path,
    config: &Path,
    version: &str,
    command: &str,
) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .await
        .unwrap();
    let endpoint = format!("http://{}/mcp", listener.local_addr().unwrap());
    let calls = Arc::new(AtomicUsize::new(0));
    let observed = calls.clone();
    let version = Arc::new(version.to_owned());
    let advertised = version.clone();
    let app = axum::Router::new().route("/mcp", axum::routing::post(move |axum::Json(value): axum::Json<Value>| {
        let version = advertised.clone();
        let observed = observed.clone();
        async move {
            use axum::response::IntoResponse;
            if value["method"] == "initialize" {
                axum::Json(json!({"jsonrpc":"2.0","id":value["id"],"result":{
                    "protocolVersion":value["params"]["protocolVersion"],"capabilities":{"tools":{}},
                    "serverInfo":{"name":"conduit","version":version.as_ref()}
                }})).into_response()
            } else {
                if value["method"] == "tools/call" {
                    observed.fetch_add(1, Ordering::SeqCst);
                }
                axum::http::StatusCode::ACCEPTED.into_response()
            }
        }
    }));
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let file = engine.directory().join("engine-url.json");
    let previous = fs::read(&file).unwrap();
    fs::write(&file, serde_json::to_vec(&endpoint).unwrap()).unwrap();
    let output = tokio::task::block_in_place(|| tool(engine, directory, config, &[command]));
    fs::write(file, previous).unwrap();
    server.abort();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains(version.as_str()));
    assert!(error.contains("nothing was sent"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(!directory.join(".conduit").exists());
}
