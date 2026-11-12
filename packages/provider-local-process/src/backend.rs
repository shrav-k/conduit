use crate::{
    guard::{GroupWitness, Launch, Marker},
    instance_key, private_directory,
    storage::{atomic_write, private_file},
};
use async_trait::async_trait;
use runtime_provider_kit::{names, *};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::{
        fd::{AsRawFd, FromRawFd, OwnedFd},
        unix::{fs::OpenOptionsExt, net::UnixStream, process::CommandExt},
    },
    path::{Path, PathBuf},
    process::Stdio,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    process::{ChildStdin, ChildStdout},
};
use tokio_util::sync::CancellationToken;

pub fn hostname() -> Result<String> {
    let mut bytes = [0u8; 1024];
    // SAFETY: writable buffer with its exact length; a truncated name is refused.
    if unsafe { libc::gethostname(bytes.as_mut_ptr().cast(), bytes.len()) } != 0 {
        return Err(Unavailable);
    }
    let end = bytes.iter().position(|b| *b == 0).ok_or(Unavailable)?;
    String::from_utf8(bytes[..end].to_vec()).map_err(|_| Unavailable)
}

/// Injectable only through the library constructor. The shipped binary always uses StopPipe.
pub trait Signaller: Send + Sync + 'static {
    fn stop(&self, pipe: &UnixStream) -> std::io::Result<()>;
}
pub struct StopPipe;
impl Signaller for StopPipe {
    fn stop(&self, mut pipe: &UnixStream) -> std::io::Result<()> {
        pipe.write_all(b"S")
    }
}

pub struct LocalBackend {
    root: PathBuf,
    binary: PathBuf,
    hostname: String,
    pipes: Mutex<BTreeMap<String, UnixStream>>,
    signaller: Arc<dyn Signaller>,
    _owner: File,
}
pub struct Staged {
    directory: PathBuf,
    launch: Option<Launch>,
    armed: bool,
}
impl Drop for Staged {
    fn drop(&mut self) {
        if self.armed {
            let _ = fs::remove_dir_all(&self.directory);
        }
    }
}

fn lock(file: &File) -> std::io::Result<bool> {
    // SAFETY: flock operates on a live file, and is nonblocking.
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
        return Ok(true);
    }
    let e = std::io::Error::last_os_error();
    if e.kind() == std::io::ErrorKind::WouldBlock {
        Ok(false)
    } else {
        Err(e)
    }
}
fn open_marker(path: &Path) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
}
fn duplicate(file: &impl AsRawFd) -> std::io::Result<OwnedFd> {
    // Avoid fd 3/4 source collisions when installing guard descriptors in pre_exec.
    let fd = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 10) };
    if fd < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(unsafe { OwnedFd::from_raw_fd(fd) })
    }
}
impl LocalBackend {
    pub fn new(root: &Path, binary: &Path) -> Result<Self> {
        Self::with_signaller(root, binary, Arc::new(StopPipe))
    }
    pub fn with_signaller(
        root: &Path,
        binary: &Path,
        signaller: Arc<dyn Signaller>,
    ) -> Result<Self> {
        private_directory(root).map_err(|_| Unavailable)?;
        let root = root.canonicalize().map_err(|_| Unavailable)?;
        let owner = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(root.join("owner.lock"))
            .map_err(|_| Unavailable)?;
        if !lock(&owner).map_err(|_| Unavailable)? {
            return Err(Unavailable);
        }
        for name in ["markers", "runs", "instances"] {
            private_directory(&root.join(name)).map_err(|_| Unavailable)?;
        }
        Ok(Self {
            root,
            binary: binary.into(),
            hostname: hostname()?,
            pipes: Mutex::new(BTreeMap::new()),
            signaller,
            _owner: owner,
        })
    }
    fn path(&self, area: &str, instance: &str) -> PathBuf {
        self.root.join(area).join(instance_key(instance))
    }
    fn local(&self, context: &CallContext) -> Result<()> {
        if context.machine.hostname == self.hostname {
            Ok(())
        } else {
            Err(Unavailable)
        }
    }
    fn observed(&self, instance: &str) -> Result<RunState> {
        let mut file = match open_marker(&self.path("markers", instance)) {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(RunState::Stopped),
            Err(_) => return Err(Unavailable),
        };
        if !lock(&file).map_err(|_| Unavailable)? {
            return Ok(RunState::Running);
        }
        let marker: Marker = serde_json::from_reader(&mut file).map_err(|_| Unavailable)?;
        Ok(match marker.witness {
            Some(GroupWitness::Unstarted) => RunState::Stopped,
            Some(GroupWitness::Group(group)) if group > 1 => {
                // Never signal a saved PID/PGID. ESRCH proves the original group gone;
                // existence may mean survivors, zombies, or reuse, all conservatively
                // unknown. Permission errors also cannot prove absence.
                let absent = unsafe { libc::kill(-group, 0) } == -1
                    && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH);
                if absent {
                    RunState::Stopped
                } else {
                    RunState::Unknown
                }
            }
            _ => RunState::Unknown,
        })
    }
}
#[async_trait]
impl RuntimeBackend for LocalBackend {
    type Staged = Staged;
    fn declaration(&self) -> Declaration {
        Declaration {
            registry_types: vec![],
            transports: vec![],
            command: true,
            machines: vec![MachineSelector {
                hostname: Some(self.hostname.clone()),
                labels: BTreeMap::new(),
            }],
        }
    }
    async fn inventory(&self) -> Result<Vec<RunRecord>> {
        let mut records = Vec::new();
        for entry in fs::read_dir(self.root.join("markers")).map_err(|_| Unavailable)? {
            let entry = entry.map_err(|_| Unavailable)?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|n| n.starts_with('.') && n.ends_with(".new"))
            {
                // A crash before rename cannot have launched this uncommitted marker.
                fs::remove_file(entry.path()).map_err(|_| Unavailable)?;
                continue;
            }
            let mut file = open_marker(&entry.path()).map_err(|_| Unavailable)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).map_err(|_| Unavailable)?;
            let mut record: RunRecord = serde_json::from_slice(&bytes).map_err(|_| Unavailable)?;
            if entry.file_name() != instance_key(&record.instance).as_str() {
                return Err(Unavailable);
            }
            record.state = self.observed(&record.instance)?;
            records.push(record);
        }
        Ok(records)
    }
    async fn stage(
        &self,
        context: &CallContext,
        input: &RunInput,
        cancel: &CancellationToken,
    ) -> Result<Staged> {
        self.local(context)?;
        if cancel.is_cancelled() || self.observed(&input.instance)? != RunState::Stopped {
            return Err(Unavailable);
        }
        let command: Vec<String> =
            serde_json::from_value(input.item["command"].clone()).map_err(|_| Unavailable)?;
        if command.first().is_none_or(|p| !Path::new(p).is_absolute())
            || !input.package_arguments.is_empty()
            || !input.secrets.package_arguments.is_empty()
        {
            return Err(Unavailable);
        }
        let directory = self.path("runs", &input.instance);
        // A previous confirmed-dead run may have left private files after a crash.
        if directory.exists() {
            fs::remove_dir_all(&directory).map_err(|_| Unavailable)?;
        }
        private_directory(&directory).map_err(|_| Unavailable)?;
        let mut staged = Staged {
            directory,
            launch: None,
            armed: true,
        };
        let identity_dir = staged.directory.join("identity");
        private_directory(&identity_dir).map_err(|_| Unavailable)?;
        for (name, value) in &input.identity.files {
            if !schemas::identity_file_names().contains(&name.as_str()) {
                return Err(Unavailable);
            }
            let mut file = private_file(&identity_dir.join(name)).map_err(|_| Unavailable)?;
            file.write_all(value.as_bytes()).map_err(|_| Unavailable)?;
        }
        let state = self.path("instances", &input.instance);
        private_directory(&state).map_err(|_| Unavailable)?;
        let mut environment = BTreeMap::new();
        for name in ["HOME", "LOGNAME", "PATH", "SHELL", "TERM", "USER"] {
            if let Ok(value) = std::env::var(name)
                && !value.starts_with("()")
            {
                environment.insert(name.into(), value);
            }
        }
        environment.extend(input.environment.clone());
        environment.extend(input.secrets.environment.clone());
        environment.extend(input.identity.environment.clone());
        environment.insert(
            names::IDENTITY_DIR_VARIABLE.into(),
            identity_dir.to_string_lossy().into_owned(),
        );
        environment.insert(
            names::STATE_DIR_VARIABLE.into(),
            state.to_string_lossy().into_owned(),
        );
        staged.launch = Some(Launch {
            command,
            environment,
            lifetime_ms: 0,
        });
        if cancel.is_cancelled() {
            return Err(Unavailable);
        }
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
        self.local(context)?;
        if cancel.is_cancelled() || self.observed(instance)? != RunState::Stopped {
            return Err(Unavailable);
        }
        let record = RunRecord {
            instance: instance.into(),
            context: context.clone(),
            state: RunState::Unknown,
            owed_stop: false,
        };
        let markers = self.root.join("markers");
        atomic_write(
            &markers,
            &instance_key(instance),
            &serde_json::to_vec(&Marker {
                record,
                witness: Some(GroupWitness::Unstarted),
            })
            .map_err(|_| Unavailable)?,
        )
        .map_err(|_| Unavailable)?;
        let marker = open_marker(&self.path("markers", instance)).map_err(|_| Unavailable)?;
        if !lock(&marker).map_err(|_| Unavailable)? {
            return Err(Unavailable);
        }
        let (mut provider, guard) = UnixStream::pair().map_err(|_| Unavailable)?;
        provider
            .set_read_timeout(Some(Duration::from_secs(3)))
            .map_err(|_| Unavailable)?;
        provider
            .set_write_timeout(Some(Duration::from_secs(3)))
            .map_err(|_| Unavailable)?;
        let guard_fd = duplicate(&guard).map_err(|_| Unavailable)?;
        let marker_fd = duplicate(&marker).map_err(|_| Unavailable)?;
        let stderr = private_file(&staged.directory.join("stderr.log")).map_err(|_| Unavailable)?;
        let mut command = tokio::process::Command::new(&self.binary);
        command
            .arg("--guard")
            .env_clear()
            .current_dir(&staged.directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(stderr);
        command.as_std_mut().process_group(0);
        // SAFETY: dup2/fcntl are async-signal-safe; descriptors are duplicated above 4.
        unsafe {
            command.pre_exec(move || {
                if libc::dup2(guard_fd.as_raw_fd(), 3) < 0
                    || libc::dup2(marker_fd.as_raw_fd(), 4) < 0
                {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let mut child = command.spawn().map_err(|_| Unavailable)?;
        let stdin = child.stdin.take().ok_or(Unavailable)?;
        let stdout = child.stdout.take().ok_or(Unavailable)?;
        drop(command);
        drop(guard);
        // After launch, only confirmed stop may remove private files.
        staged.armed = false;
        let mut launch = staged.launch.take().ok_or(Unavailable)?;
        launch.lifetime_ms = u64::try_from(lifetime.as_millis()).map_err(|_| Unavailable)?;
        let bytes = serde_json::to_vec(&launch).map_err(|_| Unavailable)?;
        let ready = provider
            .write_all(&(bytes.len() as u64).to_be_bytes())
            .and_then(|_| provider.write_all(&bytes))
            .and_then(|_| {
                let mut byte = [0];
                provider.read_exact(&mut byte)?;
                if byte == *b"R" {
                    Ok(())
                } else {
                    Err(std::io::Error::other("guard not ready"))
                }
            });
        if ready.is_err() || cancel.is_cancelled() {
            drop(provider);
            // This is the unreaped child just spawned, never a saved/recovered PID.
            // Fence its guard before returning, so cancellation cannot launch later.
            if let Some(pid) = child.id() {
                unsafe {
                    libc::kill(-(pid as i32), libc::SIGKILL);
                }
            }
            let _ = child.wait().await;
            return Err(Unavailable);
        }
        self.pipes
            .lock()
            .map_err(|_| Unavailable)?
            .insert(instance.into(), provider);
        tokio::spawn(async move {
            let _ = child.wait().await;
        });
        Ok(Box::new(ProcessChannel { stdin, stdout }))
    }
    async fn stop(&self, context: &CallContext, instance: &str) -> Result<()> {
        self.local(context)?;
        if self.observed(instance)? != RunState::Stopped {
            let pipes = self.pipes.lock().map_err(|_| Unavailable)?;
            if let Some(pipe) = pipes.get(instance)
                && self.signaller.stop(pipe).is_err()
                && self.observed(instance)? != RunState::Stopped
            {
                return Err(Unavailable);
            }
        }
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            if self.observed(instance)? == RunState::Stopped {
                break;
            }
            if Instant::now() >= deadline {
                return Err(Unavailable);
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        self.pipes.lock().map_err(|_| Unavailable)?.remove(instance);
        let directory = self.path("runs", instance);
        match fs::remove_dir_all(directory) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(_) => Err(Unavailable),
        }
    }
    async fn state(&self, context: &CallContext, instance: &str) -> Result<RunState> {
        self.local(context)?;
        self.observed(instance)
    }
}
struct ProcessChannel {
    stdin: ChildStdin,
    stdout: ChildStdout,
}
impl AsyncRead for ProcessChannel {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stdout).poll_read(cx, buf)
    }
}
impl AsyncWrite for ProcessChannel {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.stdin).poll_write(cx, bytes)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stdin).poll_flush(cx)
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stdin).poll_shutdown(cx)
    }
}
