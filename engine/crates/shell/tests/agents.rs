//! Live S1 packages with scripted native clients and a scripted filesystem (no npm).
mod common;
use conduit_shell::{
    Config, MissingExecutor, Shell, boundary,
    runner::Runner,
    server::{self, ServingExecutor},
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    io::{BufRead, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    time::{Duration, Instant},
};
fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}
fn id(shell: &Shell, name: &str) -> String {
    shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| o["name"] == name && o.get("spec").is_none())
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .into()
}
fn invoke(
    shell: &Shell,
    executor: &dyn conduit_shell::Executor,
    tool: &str,
    arguments: Value,
) -> Value {
    let reply=shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":id(shell,"root")},"tool":tool,"arguments":arguments}}),executor).unwrap();
    assert_eq!(reply["isError"], false, "{reply}");
    reply
}
#[test]
#[ignore]
fn scripted_filesystem() {
    for line in std::io::stdin().lock().lines() {
        let v: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let result = match v["method"].as_str().unwrap() {
            "initialize" => {
                json!({"protocolVersion":v["params"]["protocolVersion"],"serverInfo":{"name":"scripted-filesystem","version":"1"},"capabilities":{"tools":{}}})
            }
            "tools/list" => {
                json!({"tools":[{"name":"read_text_file","inputSchema":{"type":"object"}},{"name":"list_directory","inputSchema":{"type":"object"}},{"name":"write_file","inputSchema":{"type":"object"}}]})
            }
            "tools/call" => {
                json!({"content":[{"type":"text","text":"scripted directory"}],"isError":false})
            }
            _ => continue,
        };
        println!("{}", json!({"jsonrpc":"2.0","id":v["id"],"result":result}));
        std::io::stdout().flush().unwrap();
    }
}
fn pack(name: &str) -> String {
    let path = repo().join("packages").join(name);
    assert!(
        Command::new("cargo")
            .current_dir(&path)
            .args(["build", "--locked", "--release", "--bin", "scripted-client"])
            .status()
            .unwrap()
            .success()
    );
    let output = Command::new("cargo")
        .current_dir(&path)
        .args(["run", "--locked", "--release", "--bin", "package"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().into()
}
fn cli(store: &Path, verb: &str, args: &[&str]) -> Value {
    let o = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .arg(verb)
        .arg("--store")
        .arg(store)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    serde_json::from_slice(&o.stdout).unwrap()
}
#[test]
fn live_s1_scripted_agents_call_engine_with_distinct_run_identities() {
    let parent = tempfile::tempdir().unwrap();
    let root = repo();
    let store = parent.path().join("store");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let shell = Arc::new(
        Shell::open(
            &store,
            boundary::context(&root).unwrap(),
            Config {
                engine_url: format!("http://127.0.0.1:{port}/mcp"),
                ..Config::default()
            },
        )
        .unwrap(),
    );
    shell.submit(json!({"kind":"first-start","machine":conduit_shell::local_machine(BTreeMap::new()).unwrap()}),&MissingExecutor).unwrap();
    for name in ["agent-claude-code", "agent-codex"] {
        for file in ["conduit.yaml", "contracts.yaml"] {
            assert_eq!(
                fs::read(root.join("packages").join(name).join(file)).unwrap(),
                fs::read(
                    root.join("conformance/fixtures/agents")
                        .join(name)
                        .join(file)
                )
                .unwrap()
            );
        }
        let package = pack(name);
        let before = shell.state().unwrap();
        let path = package.rsplit_once('#').unwrap().0;
        assert!(
            boundary::install_package(&shell, &root, &format!("{path}#{}", "0".repeat(64)))
                .is_err()
        );
        assert_eq!(shell.state().unwrap(), before);
        // CLI installation of the unmodified package, without executing any client.
        let catalog_store = parent.path().join(format!("catalog-{name}"));
        cli(&catalog_store, "init", &[]);
        let template = parent.path().join(format!("template-{name}"));
        fs::create_dir(&template).unwrap();
        let ty = if name == "agent-codex" {
            "codex.v1"
        } else {
            "claude-code.v1"
        };
        fs::write(template.join("conduit.yaml"),format!("apiVersion: {}\npackages:\n  example.org/conduit/{name}:\n    version: 0.1.1\n    digest: sha256:{}\n",spec_api(&root),package.rsplit_once('#').unwrap().1)).unwrap();
        fs::write(template.join("template.yaml"),format!("apiVersion: {}\nkind: Agent\nmetadata:\n  name: test-template\nspec:\n  type: example.org/conduit/{name}#{ty}\n  config:\n    accountServices: false\n    peerMessaging: false\n",spec_api(&root))).unwrap();
        let out = Command::new(env!("CARGO_BIN_EXE_conduit"))
            .args(["register", "--store"])
            .arg(&catalog_store)
            .arg("--package")
            .arg(&package)
            .arg(&template)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let replies: Vec<Value> = String::from_utf8(out.stdout)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(replies[0]["accepted"], true, "{replies:?}");
        boundary::install_package(
            &shell,
            &root,
            &scripted_package(&package, name, parent.path()),
        )
        .unwrap();
    }
    let spec = conduit_loader::Specification::read(&root.join("spec")).unwrap();
    let mut project = boundary::project(&root.join("engine/examples/s1"), &spec).unwrap();
    for installed in &shell.state().unwrap().installed_packages {
        project.manifest.as_mut().unwrap()["packages"][&installed.package.address]["digest"] =
            json!(installed.package.digest);
    }
    for d in &mut project.documents {
        if d["kind"] == "Resource" {
            let path = parent.path().join(d["metadata"]["name"].as_str().unwrap());
            fs::create_dir(&path).unwrap();
            d["arguments"]["root"] = json!(path);
        }
    }
    let mut snapshots = common::snapshots(&root).unwrap();
    let item = &mut snapshots
        .iter_mut()
        .find(|s| s.address.ends_with("/filesystem"))
        .unwrap()
        .project
        .manifest
        .as_mut()
        .unwrap()["implementations"]["filesystem-server"];
    item.as_object_mut().unwrap().remove("package");
    item.as_object_mut().unwrap().remove("packageArguments");
    item["command"] = json!([
        std::env::current_exe().unwrap(),
        "--exact",
        "scripted_filesystem",
        "--ignored",
        "--nocapture"
    ]);
    let state = shell.state().unwrap();
    let root_id = id(&shell, "root");
    let route = state
        .objects
        .iter()
        .find(|o| o["local_name"] == "management" && o["consumer"] == root_id)
        .unwrap();
    common::install(&shell, &root, snapshots, &mut project);
    let registered=shell.submit(json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,"project":project,"request":{"sender":root_id,"caller":root_id,"conduit":route["id"],"target":route["target"],"operation":format!("{}#register.v1",shell.context.core_package)}}),&MissingExecutor).unwrap();
    assert_eq!(registered["accepted"], true, "{registered}");
    for name in ["root-workspace", "turn-a", "turn-b"] {
        invoke(
            &shell,
            &MissingExecutor,
            "access.open.v1",
            json!({"conduit":name}),
        );
    }
    run_agents(shell, &store);
}

fn run_agents(shell: Arc<Shell>, store: &Path) {
    let runner = Runner::new(&shell, env!("CARGO_BIN_EXE_conduit").into()).unwrap();
    let executor = Arc::new(ServingExecutor::new(runner.executor()));
    let completion: Arc<dyn conduit_shell::Executor> = executor.clone();
    runner.set_completion_executor(Arc::downgrade(&completion));
    for (kind, name) in [
        ("Pool", "workspace"),
        ("Fleet", "s1"),
        ("Resource", "shared"),
        ("Resource", "history"),
        ("Agent", "agent-a"),
        ("Agent", "agent-b"),
    ] {
        invoke(
            &shell,
            executor.as_ref(),
            if kind == "Resource" {
                "root-workspace.set-up.v1"
            } else if kind == "Agent" {
                "s1.set-up.v1"
            } else {
                "management.set-up.v1"
            },
            json!({"instance":{"kind":kind,"name":name}}),
        );
    }
    // Starting wrappers and listing their tools must not start either scripted client.
    assert_eq!(runner.running_count(), 4);
    for entry in fs::read_dir(store.join("work")).unwrap() {
        assert!(!entry.unwrap().path().join("client-started").exists());
    }
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let shutdown = tokio_util::sync::CancellationToken::new();
    let task = runtime.spawn(server::serve(
        shell.clone(),
        executor.clone(),
        shutdown.clone(),
    ));
    let port = url::Url::parse(&shell.config.engine_url)
        .unwrap()
        .port()
        .unwrap();
    let started = Instant::now();
    while std::net::TcpStream::connect(("127.0.0.1", port)).is_err() {
        assert!(started.elapsed() < Duration::from_secs(30));
        std::thread::sleep(Duration::from_millis(10));
    }
    shell.issue_client_token(&id(&shell, "root")).unwrap();
    for (name, turn, count) in [
        ("agent-a", "turn-a.turn.v1", 5),
        ("agent-b", "turn-b.turn.v1", 6),
    ] {
        let reply = cli(store, "call", &[turn, r#"{"message":"ENGINE"}"#]);
        assert_eq!(reply["isError"], false, "{reply}");
        assert_eq!(reply["structuredContent"]["text"], "engine reached");
        assert_eq!(
            reply["structuredContent"]["clientTools"]["observations"][0]["names"]
                .as_array()
                .unwrap()
                .len(),
            count
        );
        let state = shell.state().unwrap();
        let agent = id(&shell, name);
        let ends: Vec<_> = state
            .records
            .iter()
            .filter(|r| {
                r["type"] == "request-ended"
                    && r["data"]["consumer"] == agent
                    && r["data"]["operation"] == "example.org/s1/filesystem#directory.list.v1"
            })
            .collect();
        assert_eq!(ends.len(), 1);
        assert_eq!(ends[0]["data"]["outcome"], "succeeded");
        assert_eq!(ends[0]["data"]["caller"], agent);
    }
    let native_workdirs: Vec<_> = fs::read_dir(store.join("work"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.join("client-started").exists())
        .collect();
    assert_eq!(native_workdirs.len(), 2);
    assert_ne!(native_workdirs[0], native_workdirs[1]);
    for dir in &native_workdirs {
        assert!(!dir.starts_with(store.parent().unwrap().join("shared")));
    }
    let actual_clients: std::collections::BTreeSet<_> = native_workdirs
        .iter()
        .map(|p| {
            PathBuf::from(fs::read_to_string(p.join("client-started")).unwrap())
                .canonicalize()
                .unwrap()
        })
        .collect();
    let expected_clients = ["agent-claude-code", "agent-codex"]
        .into_iter()
        .map(|n| scripted_client(n).canonicalize().unwrap())
        .collect();
    assert_eq!(actual_clients, expected_clients);
    let records = shell.state().unwrap().records;
    assert!(!records.iter().any(|r| {
        r["data"].get("inputs").is_some()
            && r["data"]["operation"]
                .as_str()
                .is_some_and(|s| s.ends_with("#turn.v1"))
    }));
    assert!(!records.iter().any(|r| {
        r["data"].get("outputs").is_some()
            && r["data"]["operation"]
                .as_str()
                .is_some_and(|s| s.ends_with("#turn.v1"))
    }));
    // Public retirement ends the actual scripted Agent wrapper, leaving the
    // other Agent and the two filesystem runs available.
    let retired = id(&shell, "agent-a");
    let reply = cli(
        store,
        "call",
        &[
            "s1.retire.v1",
            r#"{"instance":{"kind":"Agent","name":"agent-a"}}"#,
        ],
    );
    assert_eq!(reply["isError"], false, "{reply}");
    assert_eq!(runner.running_count(), 3);
    assert!(
        !shell
            .state()
            .unwrap()
            .objects
            .iter()
            .any(|o| o["id"] == retired)
    );
    assert!(
        shell
            .state()
            .unwrap()
            .records
            .iter()
            .any(|r| r["type"] == "object-change"
                && r["data"]["target"] == retired
                && r["data"]["action"] == "removed")
    );
    shutdown.cancel();
    runtime.block_on(task).unwrap().unwrap();
    drop(completion);
    drop(runner);
}

fn spec_api(root: &Path) -> String {
    boundary::value(&root.join("spec/names.yaml")).unwrap()["apiVersion"]
        .as_str()
        .unwrap()
        .into()
}
fn scripted_package(package: &str, name: &str, parent: &Path) -> String {
    use std::io::Read;
    let bytes = fs::read(package.rsplit_once('#').unwrap().0).unwrap();
    let mut original = tar::Archive::new(flate2::read::GzDecoder::new(bytes.as_slice()));
    let mut packed = tar::Builder::new(
        flate2::GzBuilder::new()
            .mtime(0)
            .write(Vec::new(), flate2::Compression::default()),
    );
    for entry in original.entries().unwrap() {
        let mut entry = entry.unwrap();
        let path = entry.path().unwrap().into_owned();
        let mode = entry.header().mode().unwrap();
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).unwrap();
        if path == Path::new("conduit.yaml") {
            let source = String::from_utf8(bytes).unwrap();
            let client = scripted_client(name);
            bytes = source
                .replace(
                    &format!("command: [bin/{name}]"),
                    &format!(
                        "command: [bin/{name}, --client-program, {}]",
                        serde_json::to_string(&client).unwrap()
                    ),
                )
                .into_bytes();
        }
        let mut h = tar::Header::new_ustar();
        h.set_size(bytes.len() as u64);
        h.set_mode(mode);
        h.set_uid(0);
        h.set_gid(0);
        h.set_mtime(0);
        h.set_cksum();
        packed.append_data(&mut h, path, bytes.as_slice()).unwrap();
    }
    let bytes = packed.into_inner().unwrap().finish().unwrap();
    let digest = conduit_core::packages::bytes_digest(&bytes);
    let path = parent.join(format!("{name}-scripted.tar.gz"));
    fs::write(&path, bytes).unwrap();
    format!("{}#{digest}", path.display())
}

fn scripted_client(name: &str) -> PathBuf {
    let output = Command::new("cargo")
        .current_dir(repo().join("packages").join(name))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let metadata: Value = serde_json::from_slice(&output.stdout).unwrap();
    Path::new(metadata["target_directory"].as_str().unwrap())
        .join("release")
        .join(format!("scripted-client{}", std::env::consts::EXE_SUFFIX))
}

#[test]
fn cli_admits_alternate_and_linked_package_manifests() {
    use std::io::Read;
    for (filename, link) in [
        ("conduit.yml", None),
        ("conduit.json", None),
        ("conduit.yaml", Some(tar::EntryType::Link)),
        ("conduit.yaml", Some(tar::EntryType::Symlink)),
    ] {
        let parent = tempfile::tempdir().unwrap();
        let root = repo();
        let source =
            fs::read(root.join("conformance/fixtures/install-archives/hardlink.tar.gz")).unwrap();
        let mut original = tar::Archive::new(flate2::read::GzDecoder::new(source.as_slice()));
        let mut packed = tar::Builder::new(flate2::write::GzEncoder::new(
            Vec::new(),
            flate2::Compression::default(),
        ));
        for entry in original.entries().unwrap() {
            let mut entry = entry.unwrap();
            let path = entry.path().unwrap().into_owned();
            if path == Path::new("conduit.yaml") {
                continue;
            }
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes).unwrap();
            let manifest = path == Path::new("a-manifest");
            if manifest {
                bytes = String::from_utf8(bytes)
                    .unwrap()
                    .replace("example.org/s1/filesystem", "example.org/test/manifest")
                    .into_bytes();
            }
            let path = if manifest && link.is_none() {
                Path::new(filename)
            } else {
                &path
            };
            if manifest && filename.ends_with(".json") {
                let value = conduit_loader::read_bytes(conduit_loader::Syntax::Yaml, &bytes)
                    .unwrap()[0]
                    .to_json();
                bytes = serde_json::to_vec(&value).unwrap();
            }
            let mut h = tar::Header::new_ustar();
            h.set_size(bytes.len() as u64);
            h.set_mode(0o644);
            h.set_cksum();
            packed.append_data(&mut h, path, bytes.as_slice()).unwrap();
        }
        if let Some(kind) = link {
            let mut h = tar::Header::new_ustar();
            h.set_entry_type(kind);
            h.set_size(0);
            h.set_mode(0o644);
            h.set_link_name("a-manifest").unwrap();
            h.set_cksum();
            packed.append_data(&mut h, filename, &[][..]).unwrap();
        }
        let bytes = packed.into_inner().unwrap().finish().unwrap();
        let archive = parent.path().join("package.tar.gz");
        fs::write(&archive, &bytes).unwrap();
        let argument = format!(
            "{}#{}",
            archive.display(),
            conduit_core::packages::bytes_digest(&bytes)
        );
        let project = parent.path().join("project");
        fs::create_dir(&project).unwrap();
        fs::write(
            project.join("conduit.yaml"),
            format!("apiVersion: {}\n", spec_api(&root)),
        )
        .unwrap();
        let store = parent.path().join("store");
        cli(&store, "init", &[]);
        let output = cli(
            &store,
            "register",
            &["--package", &argument, project.to_str().unwrap()],
        );
        assert_eq!(output["accepted"], true, "{filename} {link:?}: {output}");
        let shell =
            Shell::open(&store, boundary::context(&root).unwrap(), Config::default()).unwrap();
        assert_eq!(shell.state().unwrap().installed_packages.len(), 1);
    }
}
