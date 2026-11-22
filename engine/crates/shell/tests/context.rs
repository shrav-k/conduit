use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    io::{BufRead, BufReader},
    net::TcpListener,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    sync::mpsc,
    time::Duration,
};

struct Lab {
    directory: tempfile::TempDir,
    store: PathBuf,
    config: PathBuf,
    project: PathBuf,
    url: String,
}

impl Lab {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let inside = |name| directory.path().join(name);
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let url = format!("http://{}/mcp", listener.local_addr().unwrap());
        let project = inside("project");
        fs::create_dir(&project).unwrap();
        Self {
            store: inside("store"),
            config: inside("config"),
            directory,
            project,
            url,
        }
    }

    fn command(&self, verb: &str) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_conduit"));
        command
            .arg(verb)
            .current_dir(&self.project)
            .env("CONDUIT_CONFIG_DIR", &self.config)
            .env("HOME", self.directory.path())
            .stdin(Stdio::null());
        command
    }

    fn at_store(&self, verb: &str) -> Command {
        let mut command = self.command(verb);
        command.arg("--store").arg(&self.store);
        command
    }

    fn init_store(&self) {
        let result = self
            .at_store("init")
            .args(["--url", &self.url])
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(0),
            "explicit store initialization"
        );
    }

    fn start(&self, config: &Path) -> Server {
        let stderr = self.directory.path().join("serve.stderr");
        let mut server = Server(
            self.at_store("serve")
                .env("CONDUIT_CONFIG_DIR", config)
                .stdout(Stdio::piped())
                .stderr(fs::File::create(stderr).unwrap())
                .spawn()
                .unwrap(),
        );
        let out = server.0.stdout.take().unwrap();
        let (send, receive) = mpsc::channel();
        std::thread::spawn(move || {
            let mut out = BufReader::new(out);
            let mut line = String::new();
            let result = out.read_line(&mut line).map(|_| line);
            let _ = send.send(result);
            let _ = std::io::copy(&mut out, &mut std::io::sink());
        });
        let ready = receive
            .recv_timeout(Duration::from_secs(90))
            .expect("serve readiness timeout")
            .expect("cannot read ready line");
        assert_eq!(
            serde_json::from_str::<Value>(&ready).expect("serve must print its JSON ready line"),
            json!({"ready":{"store":self.store.canonicalize().unwrap(),"url":self.url}})
        );
        server
    }

    fn context_file(&self) -> PathBuf {
        self.config.join("context.json")
    }

    fn state_bytes(&self) -> Vec<u8> {
        fs::read(self.store.join("state.json")).unwrap()
    }
}

struct Server(Child);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn mode(file: &Path) -> u32 {
    fs::metadata(file).unwrap().permissions().mode() & 0o777
}

fn stderr(result: &Output) -> &str {
    std::str::from_utf8(&result.stderr).unwrap()
}

fn archive(name: &str) -> (String, String, Vec<u8>) {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../conformance/fixtures/install-archives")
        .join(name);
    let bytes = fs::read(&file).unwrap();
    let digest = conduit_core::packages::bytes_digest(&bytes);
    let hex = digest.strip_prefix("sha256:").unwrap();
    (format!("{}#{hex}", file.display()), hex.into(), bytes)
}

fn one_result(result: &Output) -> Value {
    let output = std::str::from_utf8(&result.stdout).unwrap();
    assert_eq!(output.lines().count(), 1, "one engine result line");
    serde_json::from_str(output).expect("the engine result is JSON")
}

fn skeleton() -> String {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let spec = conduit_loader::Specification::read(&repo.join("spec")).unwrap();
    format!("apiVersion: {}\npackages: {{}}\n", spec.names().api_version)
}

#[test]
fn serve_records_the_engine_context() {
    let first = Lab::new();
    let second = Lab::new();
    first.init_store();
    second.init_store();
    let _first_server = first.start(&first.config);
    let version = first.command("--version").output().unwrap();
    assert_eq!(version.status.code(), Some(0));
    let version = std::str::from_utf8(&version.stdout)
        .unwrap()
        .trim_end_matches('\n');
    let file = first.context_file();
    let check = |lab: &Lab| {
        let record: Value = serde_json::from_slice(
            &fs::read(&file).expect("serve records context.json before readiness"),
        )
        .unwrap();
        assert_eq!(
            record,
            json!({"store":lab.store.canonicalize().unwrap(),"url":lab.url,
            "version":version})
        );
        assert_eq!(mode(&first.config), 0o700);
        assert_eq!(mode(&file), 0o600);
    };
    check(&first);
    let second_server = second.start(&first.config);
    check(&second);
    drop(second_server);
    check(&second);
    let relative_server = second.start(Path::new("relative-config"));
    let warning = fs::read_to_string(second.directory.path().join("serve.stderr")).unwrap();
    assert_eq!(warning.lines().count(), 1);
    assert!(warning.contains("CONDUIT_CONFIG_DIR") && warning.contains("absolute"));
    assert!(warning.contains("context.json"));
    check(&second);
    drop(relative_server);
    let _fallback_server = second.start(Path::new(""));
    let fallback = second.directory.path().join(".config/conduit/context.json");
    assert_eq!(fs::read(fallback).unwrap(), fs::read(&file).unwrap());
}

#[test]
fn bare_commands_use_the_context_and_stop_on_a_stale_one() {
    let lab = Lab::new();
    let override_lab = Lab::new();
    lab.init_store();
    override_lab.init_store();
    let server = lab.start(&lab.config);
    let _override_server = override_lab.start(&override_lab.config);
    let file = lab.context_file();
    for (verb, extra) in [
        ("inspect", &[][..]),
        ("pending", &[][..]),
        ("tools", &[][..]),
        ("call", &["management.list.v1"][..]),
    ] {
        let output = lab.command(verb).args(extra).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "bare {verb} reaches context");
        let matching = stderr(&output)
            .lines()
            .filter(|line| line.contains("context.json"))
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1);
        assert!(matching[0].contains(file.to_str().unwrap()));
        assert!(matching[0].contains(lab.store.canonicalize().unwrap().to_str().unwrap()));
        assert!(matching[0].contains(&lab.url));
    }

    let original = lab.state_bytes();
    let override_before = override_lab.state_bytes();
    let output = lab
        .command("call")
        .arg("--store")
        .arg(&override_lab.store)
        .arg("management.list.v1")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(!stderr(&output).contains("context.json"));
    assert!(lab.state_bytes() == original);
    assert!(override_lab.state_bytes() != override_before);

    let valid = fs::read(&file).unwrap();
    let expect_stop = |mut command: Command, names: &[&str], original: &[u8]| {
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(2), "unusable context");
        assert!(output.stdout.is_empty());
        let error = stderr(&output);
        assert_eq!(error.lines().count(), 1);
        assert!(
            error.contains("context.json")
                && error.contains("conduit serve --store DIR")
                && error.contains("--store DIR")
        );
        for name in names {
            assert!(error.contains(name), "diagnostic names {name}");
        }
        assert!(lab.state_bytes() == original);
    };
    let mut relative = lab.command("inspect");
    relative.env("CONDUIT_CONFIG_DIR", "relative-config");
    expect_stop(relative, &["CONDUIT_CONFIG_DIR"], &original);
    for home in [None, Some(""), Some("relative-home")] {
        let mut fallback = lab.command("inspect");
        fallback.env("CONDUIT_CONFIG_DIR", "").env_remove("HOME");
        if let Some(home) = home {
            fallback.env("HOME", home);
        }
        expect_stop(fallback, &["HOME"], &original);
    }
    fs::remove_file(&file).unwrap();
    expect_stop(lab.command("inspect"), &[file.to_str().unwrap()], &original);
    for wrong in [
        json!([]),
        json!([lab.store, lab.url, "fixture-version"]),
        json!({"store":lab.store,"url":lab.url,"version":1}),
        json!({"store":lab.store,"url":lab.url,"version":"fixture-version","extra":false}),
        json!({"store":"relative-store","url":lab.url,"version":"fixture-version"}),
    ] {
        fs::write(&file, serde_json::to_vec(&wrong).unwrap()).unwrap();
        expect_stop(lab.command("inspect"), &[file.to_str().unwrap()], &original);
    }
    let missing = lab.directory.path().join("not-a-store");
    for (store, url) in [
        (&missing, lab.url.as_str()),
        (&lab.store, override_lab.url.as_str()),
    ] {
        fs::write(
            &file,
            serde_json::to_vec(&json!({"store":store,"url":url,"version":"fixture-version"}))
                .unwrap(),
        )
        .unwrap();
        expect_stop(
            lab.command("inspect"),
            &[store.to_str().unwrap()],
            &original,
        );
    }
    let mut foreign = Lab::new();
    foreign.url = lab.url.clone();
    foreign.init_store();
    let foreign_before = foreign.state_bytes();
    fs::write(
        &file,
        serde_json::to_vec(&json!({
            "store":foreign.store,"url":foreign.url,"version":"fixture-version"
        }))
        .unwrap(),
    )
    .unwrap();
    expect_stop(
        lab.command("inspect"),
        &[
            foreign.store.to_str().unwrap(),
            &foreign.url,
            "Auth required",
        ],
        &original,
    );
    assert!(foreign.state_bytes() == foreign_before);
    fs::write(&file, valid).unwrap();
    drop(server);
    let after_stop = lab.state_bytes();
    for verb in ["inspect", "records"] {
        expect_stop(
            lab.command(verb),
            &[
                lab.store.canonicalize().unwrap().to_str().unwrap(),
                &lab.url,
            ],
            &after_stop,
        );
    }
}

#[test]
fn context_requires_the_persisted_engine_url() {
    let mut lab = Lab::new();
    lab.url = conduit_shell::Config::default().engine_url;
    let listener =
        TcpListener::bind(("127.0.0.1", 3000)).expect("default engine port is available");
    lab.init_store();
    drop(listener);
    let _server = lab.start(&lab.config);
    let fixed_url = lab.store.join("engine-url.json");
    fs::remove_file(&fixed_url).unwrap();
    let before = lab.state_bytes();

    let output = lab.command("tools").output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "bare tools needs the persisted URL"
    );
    assert!(output.stdout.is_empty());
    let error = stderr(&output);
    assert_eq!(error.lines().count(), 1);
    assert!(error.contains(lab.context_file().to_str().unwrap()));
    assert!(error.contains(fixed_url.file_name().unwrap().to_str().unwrap()));
    assert!(error.contains("start conduit serve --store DIR or give --store DIR"));
    assert!(lab.state_bytes() == before);

    let output = lab.at_store("tools").output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "explicit tools keeps the default URL"
    );
    assert!(!stderr(&output).contains("context.json"));
    assert!(!fixed_url.exists());
}

#[test]
fn context_and_cache_diagnostics_escape_path_line_breaks() {
    let mut lab = Lab::new();
    lab.config = lab.directory.path().join("config\nline\rbreak");
    lab.store = lab.directory.path().join("store\nline");
    lab.init_store();
    let server = lab.start(&lab.config);

    let output = lab.command("tools").output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    let diagnostic = stderr(&output);
    assert_eq!(diagnostic.lines().count(), 1);
    assert!(diagnostic.contains("config\\nline\\rbreak/context.json"));
    assert!(diagnostic.contains("store\\nline"));

    fs::remove_file(lab.context_file()).unwrap();
    let output = lab.command("tools").output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let diagnostic = stderr(&output);
    assert_eq!(diagnostic.lines().count(), 1);
    assert!(diagnostic.contains("config\\nline\\rbreak/context.json"));

    let blocked = lab.directory.path().join("blocked\nconfig");
    fs::write(&blocked, "").unwrap();
    let (arg, hex, _) = archive("filesystem-0.2.1.tar.gz");
    let output = lab
        .at_store("install")
        .env("CONDUIT_CONFIG_DIR", &blocked)
        .arg(arg)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let diagnostic = stderr(&output);
    assert_eq!(diagnostic.lines().count(), 1);
    assert!(diagnostic.contains("cache was not written"));
    assert!(diagnostic.contains(&format!("blocked\\nconfig/packages/{hex}")));

    drop(server);
    let _server = lab.start(&blocked);
    let warning = fs::read_to_string(lab.directory.path().join("serve.stderr")).unwrap();
    assert_eq!(warning.lines().count(), 1);
    assert!(warning.contains("blocked\\nconfig/context.json"));
}

#[test]
fn quiet_root_probe_reports_the_serving_error_on_one_line() {
    use axum::{Json, Router, response::IntoResponse, routing::post};

    let runtime = tokio::runtime::Runtime::new().unwrap();
    for (json_rpc, expected) in [
        (false, "Unexpected content type"),
        (true, "fixture declined\\r\\nsecond line"),
    ] {
        let mut lab = Lab::new();
        let listener = runtime
            .block_on(tokio::net::TcpListener::bind(("127.0.0.1", 0)))
            .unwrap();
        lab.url = format!("http://{}/mcp", listener.local_addr().unwrap());
        lab.init_store();
        fs::create_dir(&lab.config).unwrap();
        fs::write(
            lab.context_file(),
            serde_json::to_vec(&json!({"store":lab.store,"url":lab.url,"version":"fixture"}))
                .unwrap(),
        )
        .unwrap();
        let app = Router::new().route(
            "/mcp",
            post(move |Json(request): Json<Value>| async move {
                if json_rpc {
                    Json(json!({"jsonrpc":"2.0","id":request["id"],"error":{
                        "code":-32603,"message":"fixture declined\r\nsecond line"
                    }}))
                    .into_response()
                } else {
                    "plain HTTP answer".into_response()
                }
            }),
        );
        let server = runtime.spawn(async move { axum::serve(listener, app).await.unwrap() });
        let before = lab.state_bytes();
        let output = lab.command("tools").output().unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        let diagnostic = stderr(&output);
        assert_eq!(diagnostic.lines().count(), 1);
        assert!(
            diagnostic.contains(expected),
            "probe should report {expected}"
        );
        assert!(!diagnostic.contains("retrying initialization"));
        assert!(lab.state_bytes() == before);
        server.abort();
    }
}

#[test]
fn init_writes_the_flow_skeleton_unless_a_manifest_is_there() {
    let lab = Lab::new();
    let output = lab.command("init").output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "bare init writes the skeleton"
    );
    assert_eq!(fs::read_dir(&lab.project).unwrap().count(), 1);
    assert_eq!(
        fs::read(lab.project.join("conduit.yaml")).unwrap(),
        skeleton().as_bytes()
    );
    assert!(!lab.config.exists() && !lab.store.exists());
    fs::remove_file(lab.project.join("conduit.yaml")).unwrap();

    for name in conduit_loader::MANIFEST_NAMES {
        let existing = lab.project.join(name);
        fs::write(&existing, "existing manifest bytes\n").unwrap();
        let output = lab.command("init").output().unwrap();
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(fs::read_dir(&lab.project).unwrap().count(), 1);
        assert_eq!(fs::read(&existing).unwrap(), b"existing manifest bytes\n");
        assert!(stderr(&output).contains(name));
        fs::remove_file(existing).unwrap();
    }
    for (option, argument) in [
        ("--label", "purpose=fixture"),
        ("--trust-domain", "fixture.test"),
        ("--url", &lab.url),
        ("--output-cap-bytes", "100"),
        ("--output-age-seconds", "2"),
        ("--initialize-timeout-seconds", "2"),
        ("--call-timeout-seconds", "2"),
        ("--pass-environment", "FIXTURE_SETTING"),
    ] {
        let output = lab
            .command("init")
            .args([option, argument])
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(stderr(&output).contains(option) && stderr(&output).contains("--store"));
        assert_eq!(fs::read_dir(&lab.project).unwrap().count(), 0);
    }
    lab.init_store();
    assert_eq!(fs::read_dir(&lab.project).unwrap().count(), 0);
    assert!(!lab.config.exists());
}

#[test]
fn install_keeps_the_archive_in_the_package_cache() {
    let lab = Lab::new();
    lab.init_store();
    let _server = lab.start(&lab.config);
    let cache = lab.config.join("packages");
    let (arg, hex, bytes) = archive("filesystem-0.2.1.tar.gz");
    let (file, _) = arg.rsplit_once('#').unwrap();
    let bad = lab
        .at_store("install")
        .arg(format!("{file}#{}", "0".repeat(64)))
        .output()
        .unwrap();
    assert!(!bad.status.success() && bad.stdout.is_empty());
    assert!(stderr(&bad).contains("digest mismatch"));
    assert!(!cache.exists());
    let output = lab.at_store("install").arg(&arg).output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(conduit_shell::author::succeeded(&one_result(&output)));
    let entry = cache.join(hex);
    assert!(fs::read(&entry).expect("install keeps the verified archive in the cache") == bytes);
    assert_eq!(mode(&entry), 0o600);
    fs::write(&entry, "corrupted cache entry").unwrap();
    let repeated = lab.at_store("install").arg(&arg).output().unwrap();
    assert_eq!(repeated.status.code(), Some(0));
    assert!(fs::read(&entry).unwrap() == bytes);

    let (arg, hex, bytes) = archive("missing-dependency.tar.gz");
    let refused = lab.at_store("install").arg(&arg).output().unwrap();
    assert_eq!(refused.status.code(), Some(1));
    assert!(!conduit_shell::author::succeeded(&one_result(&refused)));
    assert!(fs::read(cache.join(hex)).unwrap() == bytes);

    let blocked = lab.directory.path().join("config-file");
    fs::write(&blocked, "").unwrap();
    let output = lab
        .at_store("install")
        .env("CONDUIT_CONFIG_DIR", &blocked)
        .arg(&arg)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(!conduit_shell::author::succeeded(&one_result(&output)));
    let warning = stderr(&output)
        .lines()
        .filter(|line| line.contains("cache was not written"))
        .collect::<Vec<_>>();
    assert_eq!(warning.len(), 1);
    assert!(warning[0].contains(blocked.to_str().unwrap()));
}

#[test]
fn install_changes_no_manifest() {
    let lab = Lab::new();
    lab.init_store();
    let _server = lab.start(&lab.config);
    let (arg, _, _) = archive("filesystem-0.2.1.tar.gz");
    let (refused_arg, _, _) = archive("missing-dependency.tar.gz");
    let skeleton = skeleton();
    let api = skeleton.lines().next().unwrap();
    let pinned = format!(
        "{api}\npackages: {{example.org/s1/filesystem: {{version: '0.1.1', digest: 'sha256:{}'}}}}\n",
        "0".repeat(64)
    );
    let snapshot = || {
        fs::read_dir(&lab.project)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                (entry.file_name(), fs::read(entry.path()).unwrap())
            })
            .collect::<BTreeMap<_, _>>()
    };
    let manifest = lab.project.join("conduit.yaml");
    for content in [None, Some(skeleton.as_str()), Some(pinned.as_str())] {
        if let Some(content) = content {
            fs::write(&manifest, content).unwrap();
        }
        let before = snapshot();
        for explicit in [false, true] {
            for (archive, expected_status) in [(&arg, 0), (&refused_arg, 1)] {
                let mut command = if explicit {
                    lab.at_store("install")
                } else {
                    lab.command("install")
                };
                let output = command.arg(archive).output().unwrap();
                assert_eq!(
                    output.status.code(),
                    Some(expected_status),
                    "install result with explicit store {explicit}"
                );
                assert_eq!(
                    conduit_shell::author::succeeded(&one_result(&output)),
                    expected_status == 0
                );
                assert!(!stderr(&output).contains(" pin "));
                assert!(snapshot() == before, "install changes no project file");
            }
        }
    }
}
