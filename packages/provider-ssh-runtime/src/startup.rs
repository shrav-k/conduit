//! Acquire ownership, bind the state path, and launch the independent stop-drainer.
use crate::{
    Config, SshBackend,
    durable::{self, OwnerLock},
};
use runtime_provider_kit::{Result, Unavailable};
use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io::Write,
    os::unix::{
        fs::{OpenOptionsExt, PermissionsExt},
        process::CommandExt,
    },
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub const DRAINER: &str = "provider-ssh-runtime-drain";

/// Complete child description, also used by tests without starting a process.
pub struct Launch {
    pub program: PathBuf,
    pub arguments: [OsString; 2],
    pub environment: Vec<(OsString, OsString)>,
    pub log: PathBuf,
}
impl Launch {
    pub fn new(
        executable: &Path,
        root: &Path,
        state: &Path,
        mut get: impl FnMut(&str) -> Option<OsString>,
    ) -> Self {
        Self {
            program: executable.with_file_name(DRAINER),
            arguments: [root.as_os_str().into(), state.as_os_str().into()],
            environment: get("PATH")
                .map(|value| ("PATH".into(), value))
                .into_iter()
                .collect(),
            log: root.join("drainer.log"),
        }
    }
    pub fn spawn(&self) -> std::io::Result<()> {
        let log = OpenOptions::new()
            .append(true)
            .create(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(&self.log)?;
        log.set_permissions(std::fs::Permissions::from_mode(0o600))?;
        let mut command = Command::new(&self.program);
        command
            .args(&self.arguments)
            .env_clear()
            .envs(self.environment.iter().cloned())
            .stdin(Stdio::null())
            .stdout(log.try_clone()?)
            .stderr(log);
        // SAFETY: setsid is async-signal-safe and touches no Rust state after fork.
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let mut child = command.spawn()?;
        // Reap an early exit without tying the drainer's lifetime to this process.
        std::thread::spawn(move || {
            let _ = child.wait();
        });
        Ok(())
    }
}

/// A held lock means a drainer is alive or about to be. Never wait for it.
pub fn poll_drainer(
    root: &Path,
    start: impl FnOnce() -> std::io::Result<()>,
) -> std::io::Result<bool> {
    match durable::try_lock(root, "drainer.lock")? {
        Some(lock) => {
            drop(lock);
            start()?;
            Ok(true)
        }
        None => Ok(false),
    }
}

/// Run alongside initialization, service and shutdown, until the provider ends.
pub async fn keep_alive(root: &Path, mut start: impl FnMut() -> std::io::Result<()>) {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        if poll_drainer(root, &mut start).is_err() {
            let message =
                "SSH-R: cannot keep provider-ssh-runtime-drain alive: drainer.lock or start failed";
            eprintln!("{message}");
            if let Ok(mut file) = OpenOptions::new()
                .append(true)
                .create(true)
                .mode(0o600)
                .custom_flags(libc::O_NOFOLLOW)
                .open(root.join("diagnostics.log"))
            {
                let _ = file.set_permissions(fs::Permissions::from_mode(0o600));
                let _ = writeln!(file, "{message}");
                let _ = file.sync_data();
            }
        }
    }
}

/// Publish only after the backend has accepted the configuration, before kit recovery.
pub fn configured_backend(
    root: &Path,
    config: &Config,
    build: impl FnOnce() -> Result<SshBackend>,
) -> Result<SshBackend> {
    let backend = build()?;
    config.write_drainer(root).map_err(|_| {
        durable::refusal(format!(
            "cannot write {}",
            root.join("drainer-config").display()
        ))
    })?;
    Ok(backend)
}

/// Must precede backend construction (key, part, known_hosts) and kit recovery.
pub async fn prepare(
    config: &Config,
    state: &Path,
    root: &Path,
    start: impl FnOnce() -> std::io::Result<()>,
) -> Result<OwnerLock> {
    durable::validate_directory(&config.durable_dir, Some(state))?;
    let owner = OwnerLock::acquire_provider(root).await?;
    let canonical = root.canonicalize().map_err(|_| Unavailable)?;
    owner.check(&canonical, Some(state))?;
    poll_drainer(&canonical, start).map_err(|_| {
        durable::refusal(format!(
            "cannot start sibling {DRAINER} (drainer.lock or launch failed)"
        ))
    })?;
    Ok(owner)
}
