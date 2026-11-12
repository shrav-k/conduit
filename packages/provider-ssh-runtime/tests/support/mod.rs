use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::STANDARD};
use provider_ssh_runtime::{Config, SshBackend, config::MachineConfig, ssh::*};
use runtime_provider_kit::{gate::GateConfig, *};
use serde_json::json;
use std::{
    collections::{BTreeMap, VecDeque},
    fs::{self, File},
    io::Write,
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    process::Stdio,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::io::AsyncWriteExt;
use tokio_util::sync::CancellationToken;

pub const ID: &str = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
pub const OTHER: &str = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
pub fn context() -> CallContext {
    CallContext {
        config: json!({}),
        revision: "r1".into(),
        machine: Machine {
            hostname: "synthetic-node".into(),
            labels: BTreeMap::new(),
        },
    }
}
pub fn input(id: &str) -> RunInput {
    serde_json::from_value(json!({"instance":id,"item":{"command":["/bin/cat","arg space","quote'\"","line\n$%"]},"environment":{"ORDINARY":"synthetic-ordinary","OVERRIDE":"ordinary"},"secrets":{"environment":{"SECRET_TEST":"synthetic-secret","OVERRIDE":"secret"}},"identity":{"environment":{"OVERRIDE":"identity"},"files":{"token":"synthetic-identity"}}})).unwrap()
}
pub fn config(root: &Path, remote: &Path) -> Config {
    let key = root.join("synthetic key % file");
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&key)
        .unwrap();
    let mut blob = b"\0\0\0\x0bssh-ed25519\0\0\0\x20".to_vec();
    blob.extend([17u8; 32]);
    Config {
        machines: vec![MachineConfig {
            hostname: "synthetic-node".into(),
            host: "synthetic.invalid".into(),
            port: 2222,
            user: "conduit-run".into(),
            host_key: format!("ssh-ed25519 {}", STANDARD.encode(blob)),
        }],
        private_key: key,
        durable_dir: root.parent().unwrap().to_path_buf(),
        base: remote.to_str().unwrap().into(),
        run_path: "/usr/bin:/bin".into(),
        max_lifetime: Duration::from_secs(60),
        run_limit: 62,
        stop_timeout: Duration::from_secs(2),
        drain_interval: Duration::from_secs(1),
        systemd_run: "/usr/bin/systemd-run".into(),
        systemctl: "/usr/bin/systemctl".into(),
        loginctl: "/usr/bin/loginctl".into(),
        prefix: "conduit-run-".into(),
        gate: GateConfig::default(),
    }
}
/// Independent decoder for the generated POSIX single-quoted word grammar.
pub fn words(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut word = String::new();
    let mut quoted = false;
    let mut started = false;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                quoted = !quoted;
                started = true;
            }
            '\\' if !quoted => {
                word.push(chars.next().unwrap());
                started = true;
            }
            ' ' if !quoted => {
                if started {
                    result.push(std::mem::take(&mut word));
                    started = false;
                }
            }
            c => {
                word.push(c);
                started = true;
            }
        }
    }
    assert!(!quoted);
    if started {
        result.push(word);
    }
    result
}
pub struct Recorded {
    pub args: Vec<String>,
    pub input: Vec<u8>,
}
struct StopPost {
    directory: PathBuf,
    environment: PathBuf,
    optional_environment: bool,
}
pub struct Fake {
    pub reachable: Arc<AtomicBool>,
    pub master_alive: Arc<AtomicBool>,
    pub masters: AtomicUsize,
    pub calls: Mutex<Vec<Recorded>>,
    pub units: Mutex<BTreeMap<String, String>>,
    pub checks: Mutex<BTreeMap<String, Output>>,
    pub shows: Mutex<VecDeque<String>>,
    pub sessions: Mutex<Vec<(CancellationToken, tokio::io::DuplexStream)>>,
    pub delayed: AtomicBool,
    pub queued: AtomicBool,
    pub session_failure: AtomicBool,
    pub programs: AtomicUsize,
    pub sent: tokio::sync::Notify,
    pub stage_hold: AtomicBool,
    pub staged: tokio::sync::Notify,
    pub stage_release: tokio::sync::Notify,
    pub show_hold: Mutex<Option<String>>,
    pub showing: tokio::sync::Notify,
    pub show_release: tokio::sync::Notify,
    stop_posts: Mutex<BTreeMap<String, StopPost>>,
}
impl Default for Fake {
    fn default() -> Self {
        Self {
            reachable: Arc::new(AtomicBool::new(true)),
            master_alive: Arc::new(AtomicBool::new(true)),
            masters: AtomicUsize::new(0),
            calls: Mutex::new(vec![]),
            units: Mutex::new(BTreeMap::new()),
            checks: Mutex::new(BTreeMap::new()),
            shows: Mutex::new(VecDeque::new()),
            sessions: Mutex::new(vec![]),
            delayed: AtomicBool::new(false),
            queued: AtomicBool::new(false),
            session_failure: AtomicBool::new(false),
            programs: AtomicUsize::new(0),
            sent: tokio::sync::Notify::new(),
            stage_hold: AtomicBool::new(false),
            staged: tokio::sync::Notify::new(),
            stage_release: tokio::sync::Notify::new(),
            show_hold: Mutex::new(None),
            showing: tokio::sync::Notify::new(),
            show_release: tokio::sync::Notify::new(),
            stop_posts: Mutex::new(BTreeMap::new()),
        }
    }
}
struct FakeMaster {
    reachable: Arc<AtomicBool>,
    alive: Arc<AtomicBool>,
}
impl Master for FakeMaster {
    fn alive(&mut self) -> bool {
        self.reachable.load(Ordering::SeqCst) && self.alive.load(Ordering::SeqCst)
    }
}
impl Fake {
    fn record(&self, call: Invocation, input: Vec<u8>) -> Vec<String> {
        assert!(call.program.is_absolute());
        let remote = call.args.last().unwrap().clone();
        self.calls.lock().unwrap().push(Recorded {
            args: call.args,
            input,
        });
        words(&remote)
    }
    pub fn response(&self, check: &str, text: &str, success: bool) {
        self.checks.lock().unwrap().insert(
            check.into(),
            Output {
                stdout: text.as_bytes().to_vec(),
                success,
            },
        );
    }
    pub fn end_sessions(&self) {
        for (ended, _) in self.sessions.lock().unwrap().drain(..) {
            ended.cancel();
        }
    }
    /// Model systemd spawning ExecStopPost, including its EnvironmentFile reload.
    /// This needs no provider session: e.g. RuntimeMaxSec expired after retirement.
    pub fn finish_unit(&self, unit: &str) -> bool {
        self.units.lock().unwrap().remove(unit);
        if let Some(post) = self.stop_posts.lock().unwrap().remove(unit) {
            if !post.optional_environment && !post.environment.is_file() {
                return false;
            }
            if post.directory.exists() {
                fs::remove_dir_all(post.directory).unwrap();
            }
        }
        true
    }
}
#[async_trait]
impl Executor for Fake {
    async fn master(&self, call: Invocation, socket: &Path) -> Result<Box<dyn Master>> {
        self.masters.fetch_add(1, Ordering::SeqCst);
        assert!(socket.as_os_str().len() < 80);
        assert!(!socket.exists());
        assert_eq!(
            fs::metadata(socket.parent().unwrap())
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o700
        );
        self.calls.lock().unwrap().push(Recorded {
            args: call.args,
            input: vec![],
        });
        if !self.reachable.load(Ordering::SeqCst) {
            return Err(Unavailable);
        }
        self.master_alive.store(true, Ordering::SeqCst);
        Ok(Box::new(FakeMaster {
            reachable: self.reachable.clone(),
            alive: self.master_alive.clone(),
        }))
    }
    async fn execute(&self, call: Invocation, input: Vec<u8>) -> Result<Output> {
        let words = self.record(call, input.clone());
        if !self.reachable.load(Ordering::SeqCst) {
            return Err(Unavailable);
        }
        let check = if words[0] == "/usr/bin/id" {
            "id"
        } else if words.iter().any(|s| s == "is-system-running") {
            "manager"
        } else if words.iter().any(|s| s == "show-user") {
            "linger"
        } else if words.iter().any(|s| s == "--version") {
            "version"
        } else {
            ""
        };
        if !check.is_empty() {
            if let Some(out) = self.checks.lock().unwrap().get(check) {
                return Ok(Output {
                    success: out.success,
                    stdout: out.stdout.clone(),
                });
            }
            let text = match check {
                "id" => "1001\n",
                "manager" => "running\n",
                "linger" => "Linger=yes\n",
                _ => "systemd 255 (synthetic)\n",
            };
            return Ok(Output {
                success: true,
                stdout: text.as_bytes().to_vec(),
            });
        }
        let operation = if words.iter().any(|s| s == "list-units") {
            "sweep"
        } else if words[0] == "/bin/sh" && input.starts_with(b"if [ -d") {
            "sweep-directories"
        } else {
            ""
        };
        if let Some(out) = self.checks.lock().unwrap().get(operation) {
            return Ok(Output {
                success: out.success,
                stdout: out.stdout.clone(),
            });
        }
        if words[0] == "/bin/sh" {
            let staging = input.windows(9).any(|w| w == b"umask 077");
            let mut child = tokio::process::Command::new("/bin/sh")
                .arg("-s")
                .env_clear()
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();
            let mut stdin = child.stdin.take().unwrap();
            stdin.write_all(&input).await.unwrap();
            drop(stdin);
            let out = child.wait_with_output().await.unwrap();
            if staging && self.stage_hold.load(Ordering::SeqCst) {
                self.staged.notify_one();
                self.stage_release.notified().await;
            }
            return Ok(Output {
                success: out.status.success(),
                stdout: out.stdout,
            });
        }
        if words[0] == "/bin/rm" {
            let path = Path::new(words.last().unwrap());
            if path.is_dir() {
                fs::remove_dir_all(path).unwrap();
            } else if path.exists() {
                fs::remove_file(path).unwrap();
            }
            return Ok(Output {
                success: true,
                stdout: vec![],
            });
        }
        if words.iter().any(|s| s == "list-units") {
            let end = words.iter().position(|s| s == "--").unwrap();
            assert_eq!(words.len(), end + 2);
            let text = self
                .units
                .lock()
                .unwrap()
                .keys()
                .map(|s| format!("{s} loaded active running synthetic\n"))
                .collect::<String>();
            return Ok(Output {
                success: true,
                stdout: text.into_bytes(),
            });
        }
        if words.iter().any(|s| s == "stop") {
            let end = words.iter().position(|s| s == "--").unwrap();
            assert_eq!(words.len(), end + 2);
            self.finish_unit(&words[end + 1]);
            return Ok(Output {
                success: true,
                stdout: vec![],
            });
        }
        if words.iter().any(|s| s == "show") {
            let end = words.iter().position(|s| s == "--").unwrap();
            assert_eq!(words.len(), end + 2);
            assert!(words[..end].iter().any(|s| s.starts_with("--property=")));
            let unit = &words[end + 1];
            let hold = self.show_hold.lock().unwrap().as_ref() == Some(unit);
            if hold {
                self.showing.notify_one();
                self.show_release.notified().await;
            }
            let queued = self.shows.lock().unwrap().pop_front();
            let text = queued.unwrap_or_else(|| {
                self.units
                    .lock()
                    .unwrap()
                    .get(unit)
                    .cloned()
                    .unwrap_or_else(|| "LoadState=not-found\nActiveState=inactive\n".into())
            });
            return Ok(Output {
                success: true,
                stdout: text.into_bytes(),
            });
        }
        panic!("unrecognized fake remote operation");
    }
    async fn session(&self, call: Invocation, mut stderr: File) -> Result<Session> {
        let words = self.record(call, vec![]);
        if !self.reachable.load(Ordering::SeqCst) || self.session_failure.load(Ordering::SeqCst) {
            return Err(Unavailable);
        }
        stderr.write_all(b"synthetic run stderr\n").unwrap();
        let unit = words
            .iter()
            .find_map(|s| s.strip_prefix("--unit="))
            .unwrap()
            .to_owned();
        let (channel, peer) = tokio::io::duplex(8192);
        let ended = CancellationToken::new();
        let directory = words
            .iter()
            .find_map(|s| s.strip_prefix("WorkingDirectory="))
            .unwrap();
        let environment = words
            .iter()
            .find_map(|s| s.strip_prefix("EnvironmentFile="))
            .unwrap();
        let optional_environment = environment.starts_with('-');
        let environment = environment.strip_prefix('-').unwrap_or(environment);
        let precheck = words
            .iter()
            .find_map(|s| s.strip_prefix("ExecStartPre=:/bin/test -f "));
        if !Path::new(directory).is_dir()
            || (!optional_environment && !Path::new(environment).is_file())
            || precheck.is_some_and(|path| !Path::new(path).is_file())
        {
            // WorkingDirectory and the start-only file check fence delayed launches.
            ended.cancel();
            drop(peer);
            return Ok(Session {
                channel: Box::new(channel),
                ended,
                terminate: CancellationToken::new(),
            });
        }
        assert!(
            words
                .iter()
                .any(|s| s == &format!("ExecStopPost=:/bin/rm -rf -- {directory}"))
        );
        self.sessions.lock().unwrap().push((ended.clone(), peer));
        if !self.queued.load(Ordering::SeqCst) {
            self.stop_posts.lock().unwrap().insert(
                unit.clone(),
                StopPost {
                    directory: directory.into(),
                    environment: environment.into(),
                    optional_environment,
                },
            );
            if self.delayed.load(Ordering::SeqCst) {
                self.units
                    .lock()
                    .unwrap()
                    .insert(unit, "LoadState=loaded\nActiveState=activating\n".into());
            } else {
                self.programs.fetch_add(1, Ordering::SeqCst);
                self.units
                    .lock()
                    .unwrap()
                    .insert(unit, "LoadState=loaded\nActiveState=active\n".into());
            }
        }
        self.sent.notify_one();
        let terminate = CancellationToken::new();
        let requested = terminate.clone();
        let signal = ended.clone();
        tokio::spawn(async move {
            requested.cancelled().await;
            signal.cancel();
        });
        Ok(Session {
            channel: Box::new(channel),
            ended,
            terminate,
        })
    }
}
pub struct Fixture {
    pub _root: tempfile::TempDir,
    pub durable: PathBuf,
    pub state: PathBuf,
    pub remote: tempfile::TempDir,
    pub backend: SshBackend,
    pub fake: Arc<Fake>,
    pub config: Config,
}
impl Fixture {
    pub fn new() -> Self {
        Self::with_prefix("conduit-run-")
    }
    pub fn with_prefix(prefix: &str) -> Self {
        Self::configured(prefix, 62)
    }
    pub fn with_limit(limit: usize) -> Self {
        Self::configured("conduit-run-", limit)
    }
    fn configured(prefix: &str, limit: usize) -> Self {
        let root = tempfile::Builder::new()
            .prefix("ssh state % ")
            .tempdir()
            .unwrap();
        fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700)).unwrap();
        let remote = tempfile::tempdir().unwrap();
        let state = root
            .path()
            .join("instances/00000000-0000-4000-8000-000000000145");
        fs::create_dir_all(&state).unwrap();
        fs::set_permissions(&state, fs::Permissions::from_mode(0o700)).unwrap();
        let durable_dir = root.path().join("durable space %");
        fs::create_dir(&durable_dir).unwrap();
        fs::set_permissions(&durable_dir, fs::Permissions::from_mode(0o700)).unwrap();
        let durable = provider_ssh_runtime::durable::instance_directory(&durable_dir, &state);
        fs::create_dir(&durable).unwrap();
        fs::set_permissions(&durable, fs::Permissions::from_mode(0o700)).unwrap();
        let mut config = config(&durable, remote.path());
        config.prefix = prefix.into();
        config.run_limit = limit;
        let fake = Arc::new(Fake::default());
        let backend = SshBackend::with_executor(
            &durable,
            Some(&state),
            config.clone(),
            None,
            PathBuf::from("/synthetic/ssh"),
            fake.clone(),
        )
        .unwrap();
        Self {
            _root: root,
            durable,
            state,
            remote,
            backend,
            fake,
            config,
        }
    }
    pub fn parent(&self, id: &str) -> PathBuf {
        self.remote
            .path()
            .join(self.backend.part())
            .join("runs")
            .join(provider_ssh_runtime::instance_key(id))
    }
    pub fn directory(&self, id: &str) -> PathBuf {
        let name =
            provider_ssh_runtime::instance_key(&format!("{}\0{id}", context().machine.hostname));
        let marker = fs::read(self.durable.as_path().join("markers").join(name))
            .ok()
            .and_then(|b| serde_json::from_slice::<serde_json::Value>(&b).ok());
        // Once the marker is gone, cleanup assertions inspect the entire parent.
        match marker.as_ref().and_then(|m| m["generation"].as_str()) {
            Some(generation) => self.parent(id).join(generation),
            None => self.parent(id),
        }
    }
    pub fn unit(&self, id: &str) -> String {
        format!(
            "{}{}-{}.service",
            self.config.prefix,
            self.backend.part(),
            provider_ssh_runtime::instance_key(id)
        )
    }
    pub fn diagnostics(&self) -> String {
        fs::read_to_string(self.durable.as_path().join("diagnostics.log")).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.fake.end_sessions();
    }
}
