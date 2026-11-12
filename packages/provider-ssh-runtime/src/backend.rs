use crate::{
    codec::{self, quote},
    config::Config,
    durable::{self, OwnerLock},
    instance_key,
    ssh::{self, Executor, Master, Output},
    storage::{atomic_write, private_directory, private_file},
};
use async_trait::async_trait;
use rand::RngCore;
use runtime_provider_kit::{names, *};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::Write,
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::AsyncWriteExt,
    sync::Mutex as AsyncMutex,
    time::{Instant, sleep},
};
use tokio_util::sync::CancellationToken;

const START_BOUND: Duration = Duration::from_secs(10);
const POLL: Duration = Duration::from_millis(25);
#[derive(Clone, Serialize, Deserialize)]
struct Marker {
    record: RunRecord,
    generation: String,
    started: bool,
    active: bool,
    acknowledged: bool,
}
struct Connection {
    master: Option<Box<dyn Master>>,
    uid: Option<u32>,
    retry: Instant,
    failures: u32,
}
struct Inner {
    config: Config,
    root: PathBuf,
    state: Option<PathBuf>,
    ssh: PathBuf,
    part: String,
    sockets: PathBuf,
    connections: BTreeMap<String, AsyncMutex<Connection>>,
    diagnostics: Mutex<File>,
    marker_lock: Mutex<()>,
    _owner: OwnerLock,
    _temporary: tempfile::TempDir,
    executor: Arc<dyn Executor>,
}
#[derive(Clone)]
pub struct SshBackend(Arc<Inner>);
pub struct Staged {
    backend: SshBackend,
    context: CallContext,
    instance: String,
    generation: String,
    command: Vec<String>,
    armed: bool,
}
impl Drop for Staged {
    fn drop(&mut self) {
        if self.armed {
            let backend = self.backend.clone();
            let context = self.context.clone();
            let instance = self.instance.clone();
            let generation = self.generation.clone();
            if let Ok(handle) = tokio::runtime::Handle::try_current() {
                handle.spawn(async move {
                    backend
                        .cleanup_generation(&context, &instance, &generation)
                        .await;
                });
            }
            // Without a live runtime the durable unstarted marker is cleaned at recovery.
        }
    }
}
fn random() -> String {
    let mut bytes = [0u8; 8];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn io<T>(r: std::io::Result<T>) -> Result<T> {
    r.map_err(|_| Unavailable)
}
impl SshBackend {
    pub fn new(
        root: &Path,
        state: Option<&Path>,
        config: Config,
        owner: Option<OwnerLock>,
    ) -> Result<Self> {
        let ssh = ssh::find_ssh().inspect_err(|_| {
            eprintln!("SSH-R refuses startup: program ssh on PATH");
        })?;
        Self::with_executor(
            root,
            state,
            config,
            owner,
            ssh,
            Arc::new(ssh::ProcessExecutor),
        )
    }
    pub fn with_executor(
        root: &Path,
        state: Option<&Path>,
        config: Config,
        owner: Option<OwnerLock>,
        ssh: PathBuf,
        executor: Arc<dyn Executor>,
    ) -> Result<Self> {
        if let Err(name) = config.validate() {
            let message = if name == "SSH_RUNTIME_PRIVATE_KEY" {
                format!("setting {name} must lie in {}", root.display())
            } else {
                format!("setting {name}")
            };
            return Err(durable::refusal(message));
        }
        durable::validate_directory(&config.durable_dir, state)?;
        let owner = match owner {
            Some(owner) => owner,
            None => OwnerLock::acquire(root)?,
        };
        let root = io(root.canonicalize())?;
        let durable = io(config.durable_dir.canonicalize())?;
        if root.parent() != Some(durable.as_path())
            || state.is_some_and(|s| {
                root.file_name() != durable::instance_directory(&durable, s).file_name()
            })
        {
            return Err(durable::refusal(format!(
                "durable instance subdirectory {}",
                root.display()
            )));
        }
        owner.check(&root, state)?;
        let key_ok = fs::symlink_metadata(&config.private_key)
            .is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o7777 == 0o600)
            && config
                .private_key
                .canonicalize()
                .is_ok_and(|p| p.starts_with(&root));
        if !key_ok {
            return Err(durable::refusal(format!(
                "setting SSH_RUNTIME_PRIVATE_KEY must be a regular mode 0600 file in {}",
                root.display()
            )));
        }
        io(private_directory(&root.join("markers")))?;
        if let Some(state) = state {
            io(private_directory(&state.join("stderr")))?;
        }
        io(private_directory(&root.join("generations")))?;
        let part = match fs::read_to_string(root.join("part")) {
            Ok(part)
                if part.len() == 16
                    && part
                        .bytes()
                        .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()) =>
            {
                part
            }
            Ok(_) => return Err(Unavailable),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let part = random();
                io(atomic_write(&root, "part", part.as_bytes()))?;
                part
            }
            Err(_) => return Err(Unavailable),
        };
        let mut known = String::new();
        for m in &config.machines {
            known.push_str(&format!("{} {}\n", m.hostname, m.host_key));
        }
        io(atomic_write(&root, "known_hosts", known.as_bytes()))?;
        let temporary = io(tempfile::Builder::new()
            .prefix("sshr-")
            .permissions(fs::Permissions::from_mode(0o700))
            .tempdir())?;
        io(private_directory(temporary.path()))?;
        let sockets = temporary.path().to_path_buf();
        if sockets.as_os_str().len() + 19 >= 80 {
            return Err(Unavailable);
        }
        // Validate all paths before starting any child (OpenSSH parser, not a shell).
        for path in [&sockets, &root.join("known_hosts")] {
            codec::option_path(path.to_str().ok_or(Unavailable)?).map_err(|_| Unavailable)?;
        }
        let diagnostics = io(OpenOptions::new()
            .append(true)
            .create(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(root.join("diagnostics.log")))?;
        io(diagnostics.set_permissions(fs::Permissions::from_mode(0o600)))?;
        let connections = config
            .machines
            .iter()
            .map(|m| {
                (
                    m.hostname.clone(),
                    AsyncMutex::new(Connection {
                        master: None,
                        uid: None,
                        retry: Instant::now(),
                        failures: 0,
                    }),
                )
            })
            .collect();
        Ok(Self(Arc::new(Inner {
            config,
            root,
            state: state.map(Path::to_path_buf),
            ssh,
            part,
            sockets,
            connections,
            diagnostics: Mutex::new(diagnostics),
            marker_lock: Mutex::new(()),
            _owner: owner,
            _temporary: temporary,
            executor,
        })))
    }
    pub fn part(&self) -> &str {
        &self.0.part
    }
    pub fn socket_directory(&self) -> &Path {
        &self.0.sockets
    }
    fn report(&self, text: &str) {
        // Callers supply fixed setting/program names, or validated variable names only.
        let text: String = text
            .chars()
            .map(|c| if c.is_control() { '?' } else { c })
            .collect();
        eprintln!("SSH-R: {text}");
        if let Ok(mut file) = self.0.diagnostics.lock() {
            let _ = writeln!(file, "SSH-R: {text}");
            let _ = file.sync_data();
        }
    }
    fn refusal<T>(&self, text: &str) -> Result<T> {
        self.report(text);
        Err(Unavailable::because(text))
    }
    fn error(&self, text: &str) -> Unavailable {
        self.report(text);
        Unavailable::because(text)
    }
    fn local_io<T>(&self, result: std::io::Result<T>, place: &str) -> Result<T> {
        result.map_err(|_| self.error(place))
    }
    fn marker_name(&self, context: &CallContext, id: &str) -> String {
        instance_key(&format!("{}\0{id}", context.machine.hostname))
    }
    fn marker(&self, context: &CallContext, id: &str) -> Result<Option<Marker>> {
        match fs::read(
            self.0
                .root
                .join("markers")
                .join(self.marker_name(context, id)),
        ) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map(Some)
                .map_err(|_| self.error("marker JSON invalid")),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(_) => self.refusal("marker file cannot be read"),
        }
    }
    fn save_marker(&self, marker: &Marker) -> Result<()> {
        let _guard = self
            .0
            .marker_lock
            .lock()
            .map_err(|_| self.error("marker mutex unavailable"))?;
        self.local_io(
            atomic_write(
                &self.0.root.join("markers"),
                &self.marker_name(&marker.record.context, &marker.record.instance),
                &serde_json::to_vec(marker)
                    .map_err(|_| self.error("marker record cannot be encoded"))?,
            ),
            "marker file durable write failed",
        )
    }
    fn remove_marker(&self, context: &CallContext, id: &str) -> Result<()> {
        let _guard = self
            .0
            .marker_lock
            .lock()
            .map_err(|_| self.error("marker mutex unavailable"))?;
        let dir = self.0.root.join("markers");
        match fs::remove_file(dir.join(self.marker_name(context, id))) {
            Ok(()) => self.local_io(
                self.local_io(File::open(dir), "marker directory cannot be opened")?
                    .sync_all(),
                "marker removal cannot be persisted",
            ),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(_) => self.refusal("marker file cannot be removed"),
        }
    }
    fn directory(&self, id: &str) -> String {
        format!(
            "{}/{}/runs/{}",
            self.0.config.base,
            self.0.part,
            instance_key(id)
        )
    }
    fn generation(&self) -> Result<String> {
        let directory = self.0.root.join("generations");
        loop {
            let generation = random();
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(directory.join(&generation))
            {
                Ok(file) => {
                    self.local_io(file.sync_all(), "generation reservation failed")?;
                    self.local_io(
                        File::open(&directory).and_then(|d| d.sync_all()),
                        "generation reservation cannot be persisted",
                    )?;
                    return Ok(generation);
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(_) => return self.refusal("generation reservation failed"),
            }
        }
    }
    fn unit(&self, id: &str) -> String {
        format!(
            "{}{}-{}.service",
            self.0.config.prefix,
            self.0.part,
            instance_key(id)
        )
    }
    fn socket(&self, hostname: &str) -> PathBuf {
        self.0.sockets.join(&instance_key(hostname)[..16])
    }
    fn tool(&self, uid: u32, program: &str, args: &[&str]) -> Vec<String> {
        [
            "/usr/bin/env".into(),
            format!("XDG_RUNTIME_DIR=/run/user/{uid}"),
            program.into(),
        ]
        .into_iter()
        .chain(args.iter().map(|s| (*s).into()))
        .collect()
    }
    fn invocation(&self, hostname: &str, words: Option<&[String]>) -> Result<ssh::Invocation> {
        let machine = self
            .0
            .config
            .machines
            .iter()
            .find(|m| m.hostname == hostname)
            .ok_or(Unavailable)?;
        ssh::invocation(
            &self.0.ssh,
            &self.0.config,
            machine,
            &self.0.root.join("known_hosts"),
            &self.socket(hostname),
            words,
        )
    }
    fn down(&self, conn: &mut Connection) {
        conn.master = None;
        conn.uid = None;
        conn.failures = conn.failures.saturating_add(1);
        conn.retry = Instant::now() + Duration::from_secs(1u64 << conn.failures.min(5));
    }
    async fn raw(
        &self,
        hostname: &str,
        conn: &mut Connection,
        words: &[String],
        input: Vec<u8>,
    ) -> Result<Output> {
        if !conn.master.as_mut().is_some_and(|m| m.alive()) {
            if conn.master.is_some() {
                self.down(conn);
            }
            return self.refusal("ssh master down; command not sent");
        }
        match self
            .0
            .executor
            .execute(self.invocation(hostname, Some(words))?, input)
            .await
        {
            Ok(output) => Ok(output),
            Err(_) => {
                self.down(conn);
                self.refusal("ssh command unavailable")
            }
        }
    }
    async fn checked(
        &self,
        hostname: &str,
        conn: &mut Connection,
        words: &[String],
        input: Vec<u8>,
        reason: &str,
    ) -> Result<Vec<u8>> {
        let output = self.raw(hostname, conn, words, input).await?;
        if !output.success {
            return self.refusal(reason);
        }
        Ok(output.stdout)
    }
    async fn connect(&self, hostname: &str, conn: &mut Connection) -> Result<u32> {
        if conn.master.as_mut().is_some_and(|m| m.alive()) {
            return conn.uid.ok_or(Unavailable);
        }
        if conn.master.is_some() {
            self.down(conn);
        }
        if Instant::now() < conn.retry {
            return self.refusal("ssh master down; retry backoff");
        }
        let socket = self.socket(hostname);
        match fs::remove_file(&socket) {
            Ok(()) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(_) => return self.refusal("ssh stale ControlPath removal failed"),
        }
        conn.master = Some(
            match self
                .0
                .executor
                .master(self.invocation(hostname, None)?, &socket)
                .await
            {
                Ok(master) => master,
                Err(_) => {
                    self.down(conn);
                    return self.refusal("ssh master connection failed");
                }
            },
        );
        let result = self.contact(hostname, conn).await;
        match result {
            Ok(uid) => {
                conn.uid = Some(uid);
                conn.failures = 0;
                Ok(uid)
            }
            Err(error) => {
                self.down(conn);
                Err(error)
            }
        }
    }
    async fn contact(&self, hostname: &str, conn: &mut Connection) -> Result<u32> {
        let out = self
            .checked(
                hostname,
                conn,
                &["/usr/bin/id".into(), "-u".into()],
                vec![],
                "id -u failed",
            )
            .await?;
        let uid = std::str::from_utf8(&out)
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .filter(|u| *u > 0)
            .ok_or_else(|| self.error("id -u invalid uid"))?;
        // is-system-running exits nonzero for degraded; both named states are supported.
        let out = self
            .raw(
                hostname,
                conn,
                &self.tool(
                    uid,
                    &self.0.config.systemctl,
                    &["--user", "is-system-running"],
                ),
                vec![],
            )
            .await?;
        if !matches!(
            std::str::from_utf8(&out.stdout).map(str::trim),
            Ok("running" | "degraded")
        ) {
            return self.refusal("systemctl --user is-system-running failed");
        }
        let out = self
            .checked(
                hostname,
                conn,
                &self.tool(
                    uid,
                    &self.0.config.loginctl,
                    &["show-user", &uid.to_string(), "-p", "Linger"],
                ),
                vec![],
                "loginctl show-user Linger failed",
            )
            .await?;
        if out != b"Linger=yes\n" && out != b"Linger=yes" {
            return self.refusal("loginctl show-user Linger not enabled");
        }
        let out = self
            .checked(
                hostname,
                conn,
                &self.tool(uid, &self.0.config.systemd_run, &["--version"]),
                vec![],
                "systemd-run --version failed",
            )
            .await?;
        let version = std::str::from_utf8(&out).ok().and_then(|s| {
            let mut words = s.split_whitespace();
            (words.next() == Some("systemd"))
                .then(|| words.next()?.parse::<u32>().ok())
                .flatten()
        });
        if version.is_none_or(|n| n < 254) {
            return self.refusal("systemd-run --version needs systemd 254 or later");
        }
        self.sweep(hostname, conn, uid).await?;
        Ok(uid)
    }
    async fn sweep(&self, hostname: &str, conn: &mut Connection, uid: u32) -> Result<()> {
        let prefix = format!("{}{}-", self.0.config.prefix, self.0.part);
        let units = self
            .checked(
                hostname,
                conn,
                &self.tool(
                    uid,
                    &self.0.config.systemctl,
                    &[
                        "--user",
                        "list-units",
                        "--all",
                        "--plain",
                        "--no-legend",
                        "--",
                        &format!("{prefix}*.service"),
                    ],
                ),
                vec![],
                "systemctl sweep list-units failed",
            )
            .await?;
        let markers = self.inventory().await?;
        for line in std::str::from_utf8(&units)
            .map_err(|_| self.error("systemctl sweep list-units response invalid"))?
            .lines()
        {
            let name = line
                .split_whitespace()
                .next()
                .ok_or_else(|| self.error("systemctl sweep unit name missing"))?;
            if let Some(hash) = name
                .strip_prefix(&prefix)
                .and_then(|s| s.strip_suffix(".service"))
                && valid_hash(hash)
                && !markers.iter().any(|r| {
                    r.context.machine.hostname == hostname && instance_key(&r.instance) == hash
                })
            {
                self.checked(
                    hostname,
                    conn,
                    &self.tool(
                        uid,
                        &self.0.config.systemctl,
                        &["--user", "stop", "--", name],
                    ),
                    vec![],
                    "systemctl sweep stop failed",
                )
                .await?;
                self.report(&format!("sweep stopped orphan unit {name}"));
            }
        }
        let parent = format!("{}/{}/runs", self.0.config.base, self.0.part);
        // POSIX only: the remote is GNU, but the tests run these scripts on macOS too, whose
        // chmod and find lack GNU's `MODE --` order and `-printf`.
        let p = quote(&parent);
        let script = format!(
            "if [ -d {p} ] && [ ! -h {p} ]; then [ -r {p} ] && [ -x {p} ] || exit 1; \
             for e in {p}/* {p}/.[!.]* {p}/..?*; do \
             if [ -d \"$e\" ] && [ ! -h \"$e\" ]; then printf '%s\\n' \"${{e##*/}}\"; fi; done; fi\n"
        );
        let dirs = self
            .checked(
                hostname,
                conn,
                &["/bin/sh".into(), "-s".into()],
                script.into_bytes(),
                "sweep run-directory inventory failed",
            )
            .await?;
        for hash in std::str::from_utf8(&dirs)
            .map_err(|_| self.error("sweep run-directory inventory response invalid"))?
            .lines()
        {
            if !valid_hash(hash) {
                return self.refusal("sweep invalid run-directory name");
            }
            let marker = markers.iter().find(|r| {
                r.context.machine.hostname == hostname && instance_key(&r.instance) == hash
            });
            let keep = match marker {
                Some(r) => self
                    .marker(&r.context, &r.instance)?
                    .filter(|m| m.started)
                    .map(|m| m.generation),
                None => None,
            };
            if let Some(generation) = keep {
                // find visits every immediate name, including files and symlinks.
                let directory = format!("{parent}/{hash}");
                let script = format!(
                    "set -eu\n/usr/bin/find {} -mindepth 1 -maxdepth 1 \\( ! -type d -o ! -name {} \\) -exec /bin/rm -rf -- {{}} +\n",
                    quote(&directory),
                    quote(&generation)
                );
                self.checked(
                    hostname,
                    conn,
                    &["/bin/sh".into(), "-s".into()],
                    script.into_bytes(),
                    "sweep stale generations failed",
                )
                .await?;
                self.report(&format!("sweep removed stale generation names for {hash}"));
            } else {
                self.remove_directory(hostname, conn, &format!("{parent}/{hash}"))
                    .await?;
                self.report(&format!(
                    "sweep removed unstarted or orphan run directory {hash}"
                ));
            }
        }
        Ok(())
    }
    async fn remove_directory(
        &self,
        hostname: &str,
        conn: &mut Connection,
        dir: &str,
    ) -> Result<()> {
        self.checked(
            hostname,
            conn,
            &["/bin/rm".into(), "-rf".into(), "--".into(), dir.into()],
            vec![],
            "remove run directory failed",
        )
        .await
        .map(|_| ())
    }
    async fn observation(
        &self,
        hostname: &str,
        conn: &mut Connection,
        uid: u32,
        unit: &str,
    ) -> Result<(RunState, bool)> {
        let out = self
            .raw(
                hostname,
                conn,
                &self.tool(
                    uid,
                    &self.0.config.systemctl,
                    &[
                        "--user",
                        "show",
                        "--property=LoadState,ActiveState,SubState",
                        "--",
                        unit,
                    ],
                ),
                vec![],
            )
            .await?;
        // An absent unit can produce a nonzero status with LoadState=not-found.
        let state = parse_state(&out.stdout);
        if !out.success && state != RunState::Stopped {
            return self.refusal("systemctl --user show failed");
        }
        let loaded = std::str::from_utf8(&out.stdout)
            .is_ok_and(|s| s.lines().any(|l| l == "LoadState=loaded"));
        Ok((state, loaded))
    }
    async fn show(
        &self,
        hostname: &str,
        conn: &mut Connection,
        uid: u32,
        unit: &str,
    ) -> Result<RunState> {
        self.observation(hostname, conn, uid, unit)
            .await
            .map(|s| s.0)
    }
    async fn stop_locked(
        &self,
        context: &CallContext,
        id: &str,
        conn: &mut Connection,
        uid: u32,
    ) -> Result<()> {
        let hostname = &context.machine.hostname;
        let unit = self.unit(id);
        let output = self
            .raw(
                hostname,
                conn,
                &self.tool(
                    uid,
                    &self.0.config.systemctl,
                    &["--user", "stop", "--", &unit],
                ),
                vec![],
            )
            .await?;
        // Also remove on a nonzero stop (not-found). Show below must prove absence.
        self.remove_directory(hostname, conn, &self.directory(id))
            .await?;
        let state = self.show(hostname, conn, uid, &unit).await?;
        if state != RunState::Stopped {
            return self.refusal("systemctl stop not confirmed inactive or absent");
        }
        if !output.success {
            self.report("systemctl stop returned nonzero; show confirms absent or inactive");
        }
        self.remove_marker(context, id)
    }
    async fn cleanup_generation(&self, context: &CallContext, id: &str, generation: &str) {
        let Some(lock) = self.0.connections.get(&context.machine.hostname) else {
            return;
        };
        let mut conn = lock.lock().await;
        if !self
            .marker(context, id)
            .ok()
            .flatten()
            .is_some_and(|m| m.generation == generation)
        {
            return;
        }
        if let Ok(uid) = self.connect(&context.machine.hostname, &mut conn).await {
            let _ = self.stop_locked(context, id, &mut conn, uid).await;
        }
    }
}
fn valid_hash(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn parse_state(output: &[u8]) -> RunState {
    let Ok(s) = std::str::from_utf8(output) else {
        return RunState::Unknown;
    };
    let values: BTreeMap<_, _> = s.lines().filter_map(|line| line.split_once('=')).collect();
    match (
        values.get("LoadState").copied(),
        values.get("ActiveState").copied(),
    ) {
        (Some("not-found"), _) => RunState::Stopped,
        (Some("loaded"), Some("active")) => RunState::Running,
        (Some("loaded"), Some("inactive" | "failed")) => RunState::Stopped,
        _ => RunState::Unknown,
    }
}
#[async_trait]
impl RuntimeBackend for SshBackend {
    type Staged = Staged;
    fn declaration(&self) -> Declaration {
        Declaration {
            registry_types: vec![],
            transports: vec![],
            command: true,
            machines: self
                .0
                .config
                .machines
                .iter()
                .map(|m| MachineSelector {
                    hostname: Some(m.hostname.clone()),
                    labels: BTreeMap::new(),
                })
                .collect(),
        }
    }
    async fn inventory(&self) -> Result<Vec<RunRecord>> {
        let _guard = self
            .0
            .marker_lock
            .lock()
            .map_err(|_| self.error("marker mutex unavailable"))?;
        let mut records = Vec::new();
        for entry in self.local_io(
            fs::read_dir(self.0.root.join("markers")),
            "inventory marker directory cannot be read",
        )? {
            let entry = self.local_io(entry, "inventory marker entry cannot be read")?;
            let name = entry.file_name();
            let name = name
                .to_str()
                .ok_or_else(|| self.error("inventory marker filename invalid"))?;
            if name.starts_with('.') && name.ends_with(".new") {
                continue;
            }
            if !valid_hash(name) {
                return self.refusal("inventory marker filename invalid");
            }
            let marker: Marker = serde_json::from_slice(&self.local_io(
                fs::read(entry.path()),
                "inventory marker file cannot be read",
            )?)
            .map_err(|_| self.error("inventory marker JSON invalid"))?;
            if self.marker_name(&marker.record.context, &marker.record.instance) != name {
                return self.refusal("inventory marker record mismatch");
            }
            records.push(marker.record);
        }
        Ok(records)
    }
    async fn stage(
        &self,
        context: &CallContext,
        input: &RunInput,
        cancel: &CancellationToken,
    ) -> Result<Staged> {
        if !self.declaration().supports(&input.item, &context.machine) {
            return self.refusal("unsupported machine or command item");
        }
        if !input.package_arguments.is_empty() || !input.secrets.package_arguments.is_empty() {
            return self.refusal("package_arguments unsupported");
        }
        let mut command: Vec<String> = serde_json::from_value(input.item["command"].clone())
            .map_err(|_| self.error("command arguments invalid"))?;
        if command.is_empty() || command.iter().any(|s| s.contains('\0')) {
            return self.refusal("command program or argument invalid");
        }
        for name in input.identity.files.keys() {
            if !schemas::identity_file_names().contains(&name.as_str()) {
                return self.refusal("identity file name unsupported");
            }
        }
        let generation = self.generation()?;
        let parent = self.directory(&input.instance);
        let directory = format!("{parent}/{generation}");
        let identity = format!("{directory}/identity");
        let state = format!(
            "{}/{}/state/{}",
            self.0.config.base,
            self.0.part,
            instance_key(&input.instance)
        );
        let mut values = BTreeMap::from([("PATH".into(), self.0.config.run_path.clone())]);
        values.extend(input.environment.clone());
        values.extend(input.secrets.environment.clone());
        values.extend(input.identity.environment.clone());
        values.insert(names::IDENTITY_DIR_VARIABLE.into(), identity.clone());
        values.insert(names::STATE_DIR_VARIABLE.into(), state.clone());
        let environment = codec::environment(&values).map_err(|name| {
            if codec::variable(&name) {
                self.error(&format!("environment variable {name} cannot be encoded"))
            } else {
                self.error(&format!(
                    "environment variable name {} invalid",
                    name.escape_default()
                ))
            }
        })?;
        if cancel.is_cancelled() {
            return self.refusal("stage cancelled");
        }
        let mut conn = self.0.connections[&context.machine.hostname].lock().await;
        let count = self
            .inventory()
            .await?
            .iter()
            .filter(|r| r.context.machine.hostname == context.machine.hostname)
            .count();
        if count >= self.0.config.run_limit {
            return self.refusal(&format!(
                "machine {} runLimit {} reached",
                context.machine.hostname, self.0.config.run_limit
            ));
        }
        self.connect(&context.machine.hostname, &mut conn).await?;
        if self.marker(context, &input.instance)?.is_some() {
            return self.refusal("marker already exists; stop required before stage");
        }
        self.save_marker(&Marker {
            record: RunRecord {
                instance: input.instance.clone(),
                context: context.clone(),
                state: RunState::Unknown,
                owed_stop: false,
            },
            generation: generation.clone(),
            started: false,
            active: false,
            acknowledged: false,
        })?;
        let mut staged = Staged {
            backend: self.clone(),
            context: context.clone(),
            instance: input.instance.clone(),
            generation,
            command: vec![],
            armed: true,
        };
        let mut script = format!(
            "set -eu\numask 077\n/bin/rm -rf -- {}\n/bin/mkdir -p -- {} {}\n/bin/chmod -- 700 {} {} {}\n",
            quote(&parent),
            quote(&identity),
            quote(&state),
            quote(&directory),
            quote(&identity),
            quote(&state)
        );
        for (name, value) in &input.identity.files {
            codec::write_file(&mut script, &format!("{identity}/{name}"), value.as_bytes());
        }
        codec::write_file(
            &mut script,
            &format!("{directory}/environment"),
            &environment,
        );
        // Resolve against precisely the effective run PATH, delivered only through stdin.
        script.push_str(&format!(
            "PATH=$(printf '%b' '{}x'); PATH=${{PATH%x}}; export PATH\n",
            codec::octal(values["PATH"].as_bytes())
        ));
        if command[0].starts_with('/') {
            script.push_str(&format!(
                "test -x {} && printf '%s\\n' {}\n",
                quote(&command[0]),
                quote(&command[0])
            ));
        } else {
            // Search filesystem PATH entries, even when a shell builtin has this name.
            script.push_str(&format!(
                "cd -- {}\nprogram={}\n",
                quote(&directory),
                quote(&command[0])
            ));
            script.push_str(
                r#"remaining=$PATH
while :; do
    entry=${remaining%%:*}
    case $remaining in *:*) remaining=${remaining#*:}; last=no;; *) last=yes;; esac
    [ -n "$entry" ] || entry=.
    candidate=$entry/$program
    if [ -f "$candidate" ] && [ -x "$candidate" ]; then
        case $candidate in /*) ;; *) candidate=$(/bin/pwd -P)/$candidate;; esac
        printf '%s\n' "$candidate"
        exit 0
    fi
    [ "$last" = no ] || exit 1
done
"#,
            );
        }
        // Never hold the master mutex while staging: an unrelated stop must not wait
        // for this transfer. The kit serializes this instance's own stage and stop.
        let call = self.invocation(
            &context.machine.hostname,
            Some(&["/bin/sh".into(), "-s".into()]),
        )?;
        drop(conn);
        let output = self
            .0
            .executor
            .execute(call, script.into_bytes())
            .await
            .map_err(|_| self.error("ssh staging transfer unavailable"))?;
        if !output.success {
            return self.refusal(&format!(
                "command pre-check or staging failed for program {}",
                command[0].escape_default()
            ));
        }
        let found = output.stdout;
        let found = String::from_utf8(found)
            .map_err(|_| self.error("command pre-check response is not UTF-8"))?;
        // Only one absolute executable path may be returned by the PATH search.
        let found = found.strip_suffix('\n').unwrap_or(&found);
        if !found.starts_with('/') || found.contains(['\0', '\n', '\r']) {
            return self.refusal("command pre-check did not resolve an absolute program");
        }
        command[0] = found.into();
        if cancel.is_cancelled() {
            self.cleanup_generation(context, &input.instance, &staged.generation)
                .await;
            return self.refusal("stage cancelled");
        }
        staged.command = command;
        Ok(staged)
    }
    async fn start(
        &self,
        context: &CallContext,
        instance: &str,
        mut staged: Staged,
        cancel: &CancellationToken,
        lifetime: Duration,
    ) -> Result<Channel> {
        if context.machine.hostname != staged.context.machine.hostname
            || instance != staged.instance
        {
            return self.refusal("staged context mismatch");
        }
        if cancel.is_cancelled() {
            return self.refusal("start cancelled before send");
        }
        if lifetime.is_zero() || lifetime > self.0.config.max_lifetime {
            return self.refusal("RuntimeMaxSec invalid");
        }
        let hostname = &context.machine.hostname;
        let mut conn = self
            .0
            .connections
            .get(hostname)
            .ok_or_else(|| self.error("start machine not configured"))?
            .lock()
            .await;
        let uid = self.connect(hostname, &mut conn).await?;
        let mut marker = self
            .marker(context, instance)?
            .filter(|m| m.generation == staged.generation && !m.started)
            .ok_or_else(|| self.error("start marker generation mismatch or already started"))?;
        let directory = format!("{}/{}", self.directory(instance), staged.generation);
        let unit = self.unit(instance);
        let mut words = self.tool(
            uid,
            &self.0.config.systemd_run,
            &[
                "--user",
                "--pipe",
                "--wait",
                "--collect",
                "--quiet",
                "--service-type=exec",
                "--expand-environment=no",
            ],
        );
        words.push(format!("--unit={unit}"));
        for property in [
            // systemd reloads this file for ExecStopPost too, after activation
            // deleted it. Require it only at start, retaining the launch fence.
            format!("EnvironmentFile=-{directory}/environment"),
            format!("ExecStartPre=:/bin/test -f {directory}/environment"),
            format!("WorkingDirectory={directory}"),
            format!("RuntimeMaxSec={:.9}s", lifetime.as_secs_f64()),
            format!("TimeoutStopSec={}s", self.0.config.stop_timeout.as_secs()),
            format!("ExecStopPost=:/bin/rm -rf -- {directory}"),
        ] {
            words.extend(["-p".into(), property]);
        }
        words.push("--".into());
        words.extend(staged.command.clone());
        let log = self
            .0
            .state
            .as_ref()
            .ok_or_else(|| self.error("run requires CONDUIT_STATE_DIR"))?
            .join("stderr")
            .join(format!(
                "{}-{}.log",
                self.marker_name(context, instance),
                staged.generation
            ));
        let stderr = self.local_io(private_file(&log), "run stderr file cannot be created")?;
        if cancel.is_cancelled() {
            return self.refusal("start cancelled before send");
        }
        marker.started = true;
        self.save_marker(&marker)?;
        staged.armed = false;
        let session = self
            .0
            .executor
            .session(self.invocation(hostname, Some(&words))?, stderr)
            .await
            .map_err(|_| self.error("ssh systemd-run session failed"))?;
        let mut channel = session.channel;
        let ended = session.ended;
        let terminate = session.terminate;
        let backend = self.clone();
        let ctx = context.clone();
        let id = instance.to_owned();
        let generation = staged.generation.clone();
        let observed = ended.clone();
        tokio::spawn(async move {
            observed.cancelled().await;
            backend.cleanup_generation(&ctx, &id, &generation).await;
        });
        drop(conn);
        let until = Instant::now() + START_BOUND;
        let mut cancelled = false;
        loop {
            if cancel.is_cancelled() && !cancelled {
                cancelled = true;
                let _ = tokio::time::timeout(Duration::from_secs(1), channel.shutdown()).await;
            }
            if ended.is_cancelled() {
                return self.refusal("systemd-run session ended before active confirmation");
            }
            // Both waiting for the shared connection and using it consume the
            // confirmation budget. Cancellation must close stdin even while an
            // unrelated inspection owns the connection.
            let observation = tokio::time::timeout_at(until, async {
                let mut conn = self.0.connections[hostname].lock().await;
                let observed = self.observation(hostname, &mut conn, uid, &unit).await;
                (conn, observed)
            });
            let result = tokio::select! {
                result = observation => result,
                _ = cancel.cancelled(), if !cancelled => {
                    cancelled = true;
                    let _ = tokio::time::timeout(Duration::from_secs(1), channel.shutdown()).await;
                    continue;
                }
            };
            let (mut conn, observed) = match result {
                Ok(result) => result,
                Err(_) => {
                    self.report("systemctl show active confirmation deadline");
                    break;
                }
            };
            if observed.as_ref().is_ok_and(|(_, loaded)| *loaded) && !marker.acknowledged {
                marker.acknowledged = true;
                self.save_marker(&marker)?;
            }
            // A loaded unit shortens the cancellation wait. Its name alone cannot
            // identify a generation; the missing generation directory fences late launches.
            if cancelled {
                if observed.as_ref().is_ok_and(|(_, loaded)| *loaded) {
                    return self.refusal("start cancelled after systemd-run send");
                }
            } else if let Ok((RunState::Running, _)) = observed {
                marker.active = true;
                self.save_marker(&marker)?;
                let remove = tokio::time::timeout_at(until, async {
                    self.checked(
                        hostname,
                        &mut conn,
                        &[
                            "/bin/rm".into(),
                            "-f".into(),
                            "--".into(),
                            format!("{directory}/environment"),
                        ],
                        vec![],
                        "remove active EnvironmentFile failed",
                    )
                    .await
                });
                tokio::select! {
                    result = remove => match result {
                        Ok(result) => { result?; }
                        Err(_) => break,
                    },
                    _ = cancel.cancelled() => {
                        let _ = tokio::time::timeout(Duration::from_secs(1), channel.shutdown()).await;
                        return self.refusal("start cancelled after active confirmation");
                    }
                }
                if cancel.is_cancelled() {
                    let _ = tokio::time::timeout(Duration::from_secs(1), channel.shutdown()).await;
                    return self.refusal("start cancelled after active confirmation");
                }
                return Ok(channel);
            } else if matches!(observed, Ok((RunState::Stopped, _))) && !cancelled {
                // not-found while the session is queued is normal. A failed loaded unit
                // is handled by the bounded wait/session exit, then the kit stops it.
            }
            if Instant::now() >= until {
                break;
            }
            drop(conn);
            tokio::select! { _=sleep(POLL)=>(), _=cancel.cancelled(), if !cancelled => () }
        }
        let _ = tokio::time::timeout(Duration::from_secs(1), channel.shutdown()).await;
        // The cleanup budget also includes mutex acquisition. Always terminate the
        // owned session when it expires, even if another command still owns the lock.
        let _ = tokio::time::timeout(Duration::from_secs(5), async {
            let mut conn = self.0.connections[hostname].lock().await;
            self.remove_directory(hostname, &mut conn, &directory).await
        })
        .await;
        terminate.cancel();
        let _ = tokio::time::timeout(Duration::from_secs(1), ended.cancelled()).await;
        self.refusal("systemd-run active confirmation timed out")
    }
    async fn stop(&self, context: &CallContext, instance: &str) -> Result<()> {
        let Some(lock) = self.0.connections.get(&context.machine.hostname) else {
            return self.refusal("stop machine not configured");
        };
        let mut conn = lock.lock().await;
        let uid = self.connect(&context.machine.hostname, &mut conn).await?;
        self.stop_locked(context, instance, &mut conn, uid).await
    }
    async fn state(&self, context: &CallContext, instance: &str) -> Result<RunState> {
        let Some(lock) = self.0.connections.get(&context.machine.hostname) else {
            return self.refusal("state machine not configured");
        };
        let mut conn = lock.lock().await;
        let uid = match self.connect(&context.machine.hostname, &mut conn).await {
            Ok(uid) => uid,
            Err(_) => return Ok(RunState::Unknown),
        };
        let state = self
            .show(
                &context.machine.hostname,
                &mut conn,
                uid,
                &self.unit(instance),
            )
            .await
            .unwrap_or(RunState::Unknown);
        if state == RunState::Stopped
            && self
                .marker(context, instance)?
                .is_some_and(|m| m.started && !m.active)
        {
            return Ok(RunState::Unknown);
        }
        Ok(state)
    }
}
