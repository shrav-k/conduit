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
    config.private_key = long.join("key");
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&config.private_key)
        .unwrap();
    let executor = Arc::new(Unused(AtomicBool::new(false)));
    let first = SshBackend::with_executor(
        &long,
        config.clone(),
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
            &long,
            config.clone(),
            "/synthetic/ssh".into(),
            executor.clone()
        )
        .is_err()
    );
    drop(first);
    assert!(!socket.exists());
    let second =
        SshBackend::with_executor(&long, config, "/synthetic/ssh".into(), executor.clone())
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
