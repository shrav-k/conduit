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

pub fn apply(engine: &Engine, directory: &Path, config: &Path) -> Output {
    tokio::task::block_in_place(|| tool(engine, directory, config, &["apply"]))
}

pub fn clean_local_state(engine: &Engine, directory: &Path) {
    let kept = directory.join(".conduit");
    if kept.exists() {
        for bytes in files(&kept).values() {
            let text = String::from_utf8_lossy(bytes);
            assert!(!text.contains(directory.to_str().unwrap()));
            assert!(!text.contains(engine.directory().to_str().unwrap()));
        }
    }
}

pub fn non_read_requests(engine: &Engine) -> Vec<(Value, String)> {
    let root = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            result(
                &engine
                    .call(
                        "management.inspect.v1",
                        json!({"object":{"kind":"Interface","name":"root"}}),
                        None,
                    )
                    .await,
            )
        })
    });
    root["records"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|record| {
            let operation = record["data"]["operation"].as_str()?;
            (record["type"] == "invocation-dispatched"
                && !operation.ends_with("#list.v1")
                && !operation.ends_with("#inspect.v1"))
            .then(|| (record["id"].clone(), operation.to_owned()))
        })
        .collect()
}

pub fn applied_case(
    engine: &Engine,
    directory: &Path,
    config: &Path,
    case: &Value,
    symbols: &Symbols,
    extra: &[&str],
) {
    let kept = directory.join(".conduit");
    let before = kept.exists().then(|| files(&kept));
    let before_requests = case["recorded"]["path"]
        .is_null()
        .then(|| non_read_requests(engine));
    let mut args = vec!["apply"];
    args.extend_from_slice(extra);
    let output = tokio::task::block_in_place(|| tool(engine, directory, config, &args));
    assert_eq!(
        output.status.code().map(i64::from),
        case["exit"].as_i64(),
        "apply status for {}",
        case["id"]
    );
    let expected_plan = symbols.plan_text(case["recorded"]["text"].as_str().unwrap());
    let Some(tail) = stdin_free(&output).strip_prefix(&expected_plan) else {
        panic!("printed plan differs for {}", case["id"]);
    };
    if case["recorded"]["path"].is_null() {
        let expected = case["printed"]
            .as_str()
            .unwrap()
            .strip_prefix(case["recorded"]["text"].as_str().unwrap())
            .unwrap();
        assert_eq!(tail, expected);
        assert_eq!(kept.exists().then(|| files(&kept)), before);
        assert_eq!(Some(non_read_requests(engine)), before_requests);
        clean_local_state(engine, directory);
        return;
    }
    let digest = sha256(&expected_plan);
    assert_eq!(
        fs::read_to_string(kept.join(format!("plans/{digest}.txt"))).unwrap(),
        expected_plan
    );
    let requests = case["requests"].as_array().unwrap();
    let results: Vec<_> = tail
        .lines()
        .filter(|line| line.starts_with("result "))
        .collect();
    assert_eq!(results.len(), requests.len());
    for (index, (line, request)) in results.iter().zip(requests).enumerate() {
        let number = index + 1;
        let outcome = request["result"]["outcome"].as_str().unwrap();
        let prefix = format!("result {number} {outcome} ");
        let json: Value =
            serde_json::from_str(line.strip_prefix(&prefix).expect("ordered result")).unwrap();
        assert_eq!(json["outcome"], outcome);
        for member in ["diagnostics", "pendingAction", "objects", "tombstones"] {
            if let Some(expected) = request["result"].get(member) {
                assert_eq!(
                    json[member].as_array().map(Vec::len),
                    expected.as_array().map(Vec::len)
                );
            }
        }
    }
    assert_eq!(tail.lines().last(), results.last().copied());
    let journal = boundary::value(&kept.join(format!("results/{digest}.yaml"))).unwrap();
    let entries = journal["entries"].as_array().unwrap();
    assert_eq!(entries.len(), requests.len());
    for (index, (entry, request)) in entries.iter().zip(requests).enumerate() {
        assert_eq!(entry["tool"], request["tool"]);
        assert_eq!(entry["key"], format!("plan:{digest}:1:{}", index + 1));
        let printed: Value =
            serde_json::from_str(results[index].splitn(4, ' ').nth(3).unwrap()).unwrap();
        assert_eq!(entry["result"], printed);
    }
    let state = boundary::value(&kept.join("state.yaml")).unwrap();
    assert_eq!(state["files"], case["kept"]["files"]);
    assert_eq!(
        state["objects"].as_array().unwrap().len(),
        case["kept"]["objects"].as_array().unwrap().len()
    );
    assert_eq!(
        state["plans"].as_array().unwrap().last().unwrap(),
        &format!("sha256:{digest}")
    );
    check_apply_case(engine, case, symbols, entries, &state, &digest);
    clean_local_state(engine, directory);
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
