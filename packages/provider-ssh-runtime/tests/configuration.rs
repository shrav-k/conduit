use base64::{Engine, engine::general_purpose::STANDARD};
use provider_ssh_runtime::{
    Config, SshBackend,
    config::MachineConfig,
    ssh::{Executor, Invocation, Master, Output, Session},
};
use runtime_provider_kit::{Result, Unavailable};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

fn places() -> BTreeMap<String, String> {
    let mut bytes = b"\0\0\0\x0bssh-ed25519\0\0\0\x20".to_vec();
    bytes.extend([41u8; 32]);
    let machines = vec![MachineConfig {
        hostname: "synthetic-node".into(),
        host: "synthetic.invalid".into(),
        port: 2222,
        user: "conduit-run".into(),
        host_key: format!("ssh-ed25519 {}", STANDARD.encode(bytes)),
    }];
    [
        ("DURABLE_DIR", "/synthetic/durable".into()),
        ("MACHINES", serde_json::to_string(&machines).unwrap()),
        ("PRIVATE_KEY", "/synthetic/key".into()),
        ("BASE", "/synthetic/base".into()),
        ("PATH", "/usr/bin:/bin".into()),
        ("MAX_LIFETIME", "3600".into()),
        ("RUN_LIMIT", "62".into()),
        ("STOP_TIMEOUT", "10".into()),
        ("DRAIN_INTERVAL", "5".into()),
        ("BIND_ADDRESS", "127.0.0.1".into()),
        ("ADVERTISED_HOST", "synthetic.invalid".into()),
        ("KEEPALIVE_IDLE", "30".into()),
        ("KEEPALIVE_INTERVAL", "5".into()),
        ("KEEPALIVE_COUNT", "3".into()),
    ]
    .into_iter()
    .map(|(name, value)| (format!("SSH_RUNTIME_{name}"), value))
    .collect()
}
#[test]
fn required_bindings_and_defaults_and_malformed_places() {
    let values = places();
    let config = Config::from_lookup(|n| values.get(n).cloned()).unwrap();
    assert_eq!(config.systemd_run, "/usr/bin/systemd-run");
    assert_eq!(config.systemctl, "/usr/bin/systemctl");
    assert_eq!(config.loginctl, "/usr/bin/loginctl");
    assert_eq!(config.prefix, "conduit-run-");
    let mut spaced = values.clone();
    spaced.insert(
        "SSH_RUNTIME_SYSTEMCTL".into(),
        "/synthetic/tool with 'quotes' $%".into(),
    );
    assert!(Config::from_lookup(|n| spaced.get(n).cloned()).is_ok());
    for key in values.keys() {
        let mut missing = values.clone();
        missing.remove(key);
        assert!(
            Config::from_lookup(|n| missing.get(n).cloned())
                .err()
                .is_some_and(|n| n == key)
        );
    }
    for (name, value) in [
        ("MACHINES", "[]"),
        ("MACHINES", "malformed"),
        ("PRIVATE_KEY", "~/key"),
        ("DURABLE_DIR", "relative"),
        ("DURABLE_DIR", "/bad\npath"),
        ("DURABLE_DIR", "/bad\"path"),
        ("DURABLE_DIR", "/bad\\path"),
        ("DURABLE_DIR", "/bad\rpath"),
        ("DURABLE_DIR", "/bad\0path"),
        ("BASE", "/with space"),
        ("BASE", "/a/../b"),
        ("PATH", "a\0b"),
        ("RUN_LIMIT", "0"),
        ("RUN_LIMIT", "63"),
        ("RUN_LIMIT", "bad"),
        ("MAX_LIFETIME", "0"),
        ("STOP_TIMEOUT", "-1"),
        ("DRAIN_INTERVAL", "1.5"),
        ("BIND_ADDRESS", "0.0.0.0"),
        ("ADVERTISED_HOST", "https://synthetic.invalid"),
        ("KEEPALIVE_COUNT", "0"),
        ("KEEPALIVE_COUNT", "2147483647"),
        ("SYSTEMD_RUN", "relative"),
        ("SYSTEMCTL", "relative/path"),
        ("LOGINCTL", "/bad\0path"),
        ("UNIT_PREFIX", "../unsafe"),
    ] {
        let mut bad = values.clone();
        let name = format!("SSH_RUNTIME_{name}");
        bad.insert(name.clone(), value.into());
        assert!(
            Config::from_lookup(|n| bad.get(n).cloned())
                .err()
                .is_some_and(|n| n == name)
        );
    }
    for changed in ["other-user", "root"] {
        let mut c = config.clone();
        c.machines[0].user = changed.into();
        assert_eq!(c.validate().err(), Some("SSH_RUNTIME_MACHINES"));
    }
    let mut c = config.clone();
    c.machines.push(c.machines[0].clone());
    assert_eq!(c.validate().err(), Some("SSH_RUNTIME_MACHINES"));
    let mut c = config;
    c.machines[0].host_key = "ssh-ed25519 invalid".into();
    assert_eq!(c.validate().err(), Some("SSH_RUNTIME_MACHINES"));
}
struct Unused(AtomicBool);
#[async_trait::async_trait]
impl Executor for Unused {
    async fn master(&self, _: Invocation, _: &Path) -> Result<Box<dyn Master>> {
        self.0.store(true, Ordering::SeqCst);
        Err(Unavailable)
    }
    async fn execute(&self, _: Invocation, _: Vec<u8>) -> Result<Output> {
        panic!("constructor must not run remote commands")
    }
    async fn session(&self, _: Invocation, _: File) -> Result<Session> {
        panic!("constructor must not start a run")
    }
}
#[test]
fn persistent_part_owner_lock_and_short_socket_fallback_without_network() {
    let root = tempfile::tempdir().unwrap();
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700)).unwrap();
    let mut config = Config::from_lookup(|n| places().get(n).cloned()).unwrap();
    let mut long = root.path().to_path_buf();
    for _ in 0..4 {
        long = long.join("synthetic-long-state-directory");
        fs::create_dir(&long).unwrap();
        fs::set_permissions(&long, fs::Permissions::from_mode(0o700)).unwrap();
    }
    config.durable_dir = long.clone();
    let state = root
        .path()
        .join("instances/00000000-0000-4000-8000-000000000145");
    fs::create_dir_all(&state).unwrap();
    let durable = provider_ssh_runtime::durable::instance_directory(&long, &state);
    fs::create_dir(&durable).unwrap();
    fs::set_permissions(&durable, fs::Permissions::from_mode(0o700)).unwrap();
    config.private_key = durable.join("key");
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&config.private_key)
        .unwrap();
    let executor = Arc::new(Unused(AtomicBool::new(false)));
    let first = SshBackend::with_executor(
        &durable,
        Some(&state),
        config.clone(),
        None,
        "/synthetic/ssh".into(),
        executor.clone(),
    )
    .unwrap();
    let part = first.part().to_owned();
    let socket = first.socket_directory().to_path_buf();
    assert!(!socket.starts_with(&long));
    assert!(socket.as_os_str().len() + 19 < 80);
    assert!(
        SshBackend::with_executor(
            &durable,
            Some(&state),
            config.clone(),
            None,
            "/synthetic/ssh".into(),
            executor.clone()
        )
        .is_err()
    );
    drop(first);
    assert!(!socket.exists());
    let second = SshBackend::with_executor(
        &durable,
        Some(&state),
        config,
        None,
        "/synthetic/ssh".into(),
        executor.clone(),
    )
    .unwrap();
    assert_eq!(part, second.part());
    assert!(!executor.0.load(Ordering::SeqCst));
}

#[test]
fn shipped_binary_names_a_missing_configuration_binding_without_contact() {
    let state = tempfile::tempdir().unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_provider-ssh-runtime"))
        .env_clear()
        .env(
            runtime_provider_kit::names::STATE_DIR_VARIABLE,
            state.path(),
        )
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("SSH_RUNTIME_MACHINES")
    );
}

struct Layout {
    _temp: tempfile::TempDir,
    state: std::path::PathBuf,
    root: std::path::PathBuf,
    config: Config,
}
impl Layout {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let state = temp
            .path()
            .join("instances/00000000-0000-4000-8000-000000000145");
        fs::create_dir_all(&state).unwrap();
        fs::set_permissions(&state, fs::Permissions::from_mode(0o700)).unwrap();
        fs::set_permissions(state.parent().unwrap(), fs::Permissions::from_mode(0o700)).unwrap();
        let durable = temp.path().join("durable space %");
        fs::create_dir(&durable).unwrap();
        fs::set_permissions(&durable, fs::Permissions::from_mode(0o700)).unwrap();
        let root = provider_ssh_runtime::durable::instance_directory(&durable, &state);
        let mut config = Config::from_lookup(|n| places().get(n).cloned()).unwrap();
        config.durable_dir = durable;
        config.private_key = root.join("key");
        Self {
            _temp: temp,
            state,
            root,
            config,
        }
    }
    fn key(&self) {
        fs::create_dir_all(&self.root).unwrap();
        fs::set_permissions(&self.root, fs::Permissions::from_mode(0o700)).unwrap();
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&self.config.private_key)
            .unwrap();
    }
    fn start(&self) -> Result<SshBackend> {
        self.open(Some(&self.state), None)
    }
    fn diagnostic(&self, config: &Config) -> String {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_provider-ssh-runtime"))
            .env_clear()
            .envs(places())
            .env("PATH", "/usr/bin:/bin")
            .env(runtime_provider_kit::names::STATE_DIR_VARIABLE, &self.state)
            .env("SSH_RUNTIME_DURABLE_DIR", &config.durable_dir)
            .env("SSH_RUNTIME_PRIVATE_KEY", &config.private_key)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        String::from_utf8(output.stderr).unwrap()
    }
    fn open(
        &self,
        state: Option<&Path>,
        owner: Option<provider_ssh_runtime::durable::OwnerLock>,
    ) -> Result<SshBackend> {
        SshBackend::with_executor(
            &self.root,
            state,
            self.config.clone(),
            owner,
            "/synthetic/ssh".into(),
            Arc::new(Unused(AtomicBool::new(false))),
        )
    }
}

#[tokio::test]
async fn durable_initialization_is_ordered_and_missing_ledger_is_an_error() {
    use provider_ssh_runtime::FileStore;
    use runtime_provider_kit::StateStore;
    let l = Layout::new();
    // The constructor creates the subdirectory and ledger before it checks the key.
    assert!(l.start().is_err());
    assert_eq!(fs::read(l.root.join("ledger.json")).unwrap(), b"[]");
    assert_eq!(
        fs::read(l.root.join("state-dir")).unwrap(),
        l.state.as_os_str().as_encoded_bytes()
    );
    assert_eq!(
        fs::metadata(&l.root).unwrap().permissions().mode() & 0o777,
        0o700
    );
    let store = FileStore::new(&l.root);
    assert!(store.load().await.unwrap().is_empty());
    fs::remove_file(l.root.join("ledger.json")).unwrap();
    assert!(store.load().await.is_err());
    assert!(l.start().is_err());
    assert!(!l.root.join("ledger.json").exists());

    let l = Layout::new();
    l.key();
    // Force the second atomic write to fail: an empty durable ledger must already exist.
    fs::create_dir(l.root.join(".state-dir.new")).unwrap();
    assert!(l.start().is_err());
    assert_eq!(fs::read(l.root.join("ledger.json")).unwrap(), b"[]");
    assert!(!l.root.join("state-dir").exists());
    fs::remove_dir(l.root.join(".state-dir.new")).unwrap();
    let backend = l.start().unwrap();
    assert_eq!(
        fs::read(l.root.join("state-dir")).unwrap(),
        l.state.as_os_str().as_encoded_bytes()
    );
    drop(backend);
}

#[test]
fn hash_uses_exact_state_directory_bytes() {
    use provider_ssh_runtime::durable::instance_directory;
    use std::os::unix::ffi::OsStrExt;
    let durable = Path::new("/synthetic/durable");
    assert_eq!(
        instance_directory(durable, Path::new("abc"))
            .file_name()
            .unwrap(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_ne!(
        instance_directory(durable, Path::new("/a/b")),
        instance_directory(durable, Path::new("/a//b/"))
    );
    assert_ne!(
        instance_directory(durable, Path::new("/a/b")),
        instance_directory(durable, Path::new("/a/b\n"))
    );
    let raw = Path::new(std::ffi::OsStr::from_bytes(b"/synthetic/\xff"));
    assert_ne!(
        instance_directory(durable, raw),
        instance_directory(durable, Path::new("/synthetic/\u{fffd}"))
    );
}

#[test]
fn drainer_constructor_uses_held_lock_after_state_removal_and_creates_nothing_there() {
    use provider_ssh_runtime::durable::OwnerLock;
    let l = Layout::new();
    l.key();
    drop(l.start().unwrap());
    fs::remove_dir_all(l.state.parent().unwrap()).unwrap();
    let owner = OwnerLock::acquire(&l.root).unwrap();
    assert!(l.open(None, None).is_err());
    let backend = l.open(None, Some(owner)).unwrap();
    assert!(!l.state.parent().unwrap().exists());
    assert!(!l.root.join("stderr").exists());
    drop(backend);
    assert!(!l.state.parent().unwrap().exists());
    assert!(OwnerLock::acquire(&l.root).is_ok());
}

#[test]
fn instances_under_one_durable_directory_have_independent_locks_and_ledgers() {
    let l = Layout::new();
    l.key();
    let first = l.start().unwrap();
    let state = l
        .state
        .parent()
        .unwrap()
        .join("00000000-0000-4000-8000-000000000146");
    fs::create_dir(&state).unwrap();
    let root = provider_ssh_runtime::durable::instance_directory(&l.config.durable_dir, &state);
    assert_ne!(l.root, root);
    fs::create_dir(&root).unwrap();
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
    let mut config = l.config.clone();
    config.private_key = root.join("key");
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&config.private_key)
        .unwrap();
    let second = SshBackend::with_executor(
        &root,
        Some(&state),
        config,
        None,
        "/synthetic/ssh".into(),
        Arc::new(Unused(AtomicBool::new(false))),
    )
    .unwrap();
    assert!(l.start().is_err());
    assert_ne!(first.part(), second.part());
    assert!(root.join("ledger.json").exists());
    fs::write(root.join("ledger.json"), b"second ledger").unwrap();
    assert_eq!(fs::read(l.root.join("ledger.json")).unwrap(), b"[]");
}

#[test]
fn durable_refusals_and_old_layout_name_the_places() {
    let l = Layout::new();
    l.key();
    for path in [
        "",
        "relative",
        "/bad\npath",
        "/bad\rpath",
        "/bad\0path",
        "/bad\"path",
        "/bad\\path",
    ] {
        let mut config = l.config.clone();
        config.durable_dir = path.into();
        let error = SshBackend::with_executor(
            &l.root,
            Some(&l.state),
            config,
            None,
            "/synthetic/ssh".into(),
            Arc::new(Unused(AtomicBool::new(false))),
        )
        .err()
        .unwrap();
        assert_eq!(
            error,
            Unavailable::because("setting SSH_RUNTIME_DURABLE_DIR")
        );
    }
    for kind in [
        "missing",
        "file",
        "mode",
        "inside-state",
        "inside-parent",
        "parent-itself",
        "alias",
        "ancestor-alias",
    ] {
        let mut config = l.config.clone();
        let path = match kind {
            "inside-state" => l.state.join("nested"),
            "inside-parent" => l.state.parent().unwrap().join("another-instance"),
            "parent-itself" => l.state.parent().unwrap().to_path_buf(),
            "ancestor-alias" => {
                let alias = l.config.durable_dir.with_file_name("instances-alias");
                std::os::unix::fs::symlink(l.state.parent().unwrap(), &alias).unwrap();
                alias.join("nested-durable")
            }
            _ => l.config.durable_dir.with_file_name(kind),
        };
        match kind {
            "missing" | "parent-itself" => (),
            "file" => fs::write(&path, b"").unwrap(),
            "alias" => std::os::unix::fs::symlink(&l.state, &path).unwrap(),
            _ => {
                fs::create_dir(&path).unwrap();
                fs::set_permissions(
                    &path,
                    fs::Permissions::from_mode(if kind == "mode" { 0o755 } else { 0o700 }),
                )
                .unwrap();
            }
        }
        config.durable_dir = path;
        let error = l.diagnostic(&config);
        assert!(error.contains("SSH_RUNTIME_DURABLE_DIR"), "{kind}");
    }
    let names = [
        "ledger.json",
        "markers",
        "generations",
        "part",
        "known_hosts",
    ];
    for name in names {
        fs::write(l.state.join(name), b"retained").unwrap();
        let error = l.diagnostic(&l.config);
        assert!(error.contains(name));
        assert!(error.contains(l.root.to_str().unwrap()));
        fs::remove_file(l.state.join(name)).unwrap();
    }
    for name in names {
        fs::write(l.state.join(name), b"retained").unwrap();
    }
    let error = l.diagnostic(&l.config);
    for name in names {
        assert!(error.contains(name));
    }
    assert!(!l.root.join("ledger.json").exists());
}

#[test]
fn state_binding_mismatch_is_refused_and_private_key_refusal_names_expected_directory() {
    let mut l = Layout::new();
    l.key();
    drop(l.start().unwrap());
    fs::write(l.root.join("state-dir"), b"another value").unwrap();
    assert!(l.diagnostic(&l.config).contains("state-dir"));
    fs::write(
        l.root.join("state-dir"),
        l.state.as_os_str().as_encoded_bytes(),
    )
    .unwrap();
    let outside = l.config.durable_dir.join("outside-key");
    fs::rename(&l.config.private_key, &outside).unwrap();
    l.config.private_key = outside;
    let error = l.diagnostic(&l.config);
    assert!(error.contains("SSH_RUNTIME_PRIVATE_KEY"));
    assert!(error.contains(l.root.to_str().unwrap()));
    l.config.private_key = "relative-key".into();
    let error = l.diagnostic(&l.config);
    assert!(error.contains("SSH_RUNTIME_PRIVATE_KEY"));
    assert!(error.contains(l.root.to_str().unwrap()));
}

#[test]
fn only_run_stderr_is_kept_in_engine_state_directory() {
    let l = Layout::new();
    l.key();
    let backend = l.start().unwrap();
    let entries: Vec<_> = fs::read_dir(&l.state)
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    assert_eq!(entries, ["stderr"]);
    assert!(!l.root.join("stderr").exists());
    assert!(!backend.socket_directory().starts_with(&l.root));
    assert!(!backend.socket_directory().starts_with(&l.state));
}

#[tokio::test]
async fn explicit_migration_preserves_an_existing_owed_stop_without_state_binding() {
    use provider_ssh_runtime::FileStore;
    use runtime_provider_kit::StateStore;
    let l = Layout::new();
    l.key();
    let bytes = serde_json::to_vec(&serde_json::json!([{
        "instance": "00000000-0000-4000-8000-000000000999",
        "context": {"config": {}, "revision": "1", "machine": {"hostname": "synthetic-node", "labels": {}}},
        "state": "unknown", "owed_stop": true
    }])).unwrap();
    fs::write(l.root.join("ledger.json"), &bytes).unwrap();
    let backend = l.start().unwrap();
    assert_eq!(fs::read(l.root.join("ledger.json")).unwrap(), bytes);
    assert!(FileStore::new(&l.root).load().await.unwrap()[0].owed_stop);
    drop(backend);
}
