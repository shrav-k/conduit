//! claude-code.v2's sign-in hooks (orders 050i and 050j): Claude Code's own status, logout and
//! login, run in this Agent's own home alone.
use super::{
    ClientTree, Config, LOGOUT_DEADLINE, POLL, Session, client_command, client_home, exit_cause,
    logout, process, signal_group,
};
use anyhow::{Result, anyhow, bail, ensure};
use serde_json::Value;
use std::{
    fs,
    io::{self, Read, Write},
    os::unix::fs::{DirBuilderExt, OpenOptionsExt},
    path::Path,
    process::Stdio,
    time::Duration,
};
use tokio::{
    io::AsyncReadExt,
    time::{Instant, sleep, timeout},
};

/// How long the account hook waits for the client's status (A5 stage 0's limit).
pub const STATUS_DEADLINE: Duration = Duration::from_secs(30);
/// The most bytes of the status's standard output that the account hook reads.
pub const STATUS_LIMIT: u64 = 65_536;
pub const STATUS_DID_NOT_START: &str = "client status did not start";
pub const STATUS_UNREADABLE: &str = "client status unreadable";
pub const STATUS_NO_ACCOUNT: &str = "client status names no account";
/// The sign-out hook's answer once the client is signed out.
pub const SIGNED_OUT: &str = "client signed out of this Agent's own home";
/// The cause a sign-out gives a conversation whose client it stopped.
pub const SIGNED_OUT_CAUSE: &str = "client signed out";

/// The client's own status, run with the Agent's home alone (A5 stage 0).
pub fn status_arguments() -> Vec<String> {
    vec!["auth".into(), "status".into()]
}

/// The account the status names, or None where it reports no login. Its errors are fixed
/// texts: no byte of the client's output reaches them.
async fn account_status(
    c: &Config,
    home: &Path,
    tree: &ClientTree,
    deadline: Duration,
) -> Result<Option<String>> {
    let (mut child, pid) = {
        let mut state = tree.0.lock().unwrap();
        ensure!(!state.closed, "{STATUS_DID_NOT_START}");
        let child = client_command(c, Some(home), &status_arguments())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| anyhow!("{STATUS_DID_NOT_START}"))?;
        let pid = child
            .id()
            .ok_or_else(|| anyhow!("{STATUS_DID_NOT_START}"))? as i32;
        let start = process(pid).ok().flatten().map(|p| p.start);
        state.groups.insert(pid, start);
        (child, pid)
    };
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("{STATUS_DID_NOT_START}"))?;
    let finished = timeout(deadline, async {
        let mut output = Vec::new();
        // `take` consumes the pipe, so a longer output cannot block the wait.
        let read = stdout.take(STATUS_LIMIT + 1).read_to_end(&mut output).await;
        (read.map(|_| output), child.wait().await)
    })
    .await;
    let Ok((output, status)) = finished else {
        // The status's own group alone: a conversation's client, if one runs, is not stopped.
        unsafe { libc::kill(-pid, libc::SIGKILL) };
        let _ = child.kill().await;
        bail!(
            "client status failed: no exit within {} seconds",
            deadline.as_secs()
        );
    };
    let output = match output {
        Ok(output) if output.len() as u64 <= STATUS_LIMIT => output,
        _ => bail!("{STATUS_UNREADABLE}"),
    };
    let status =
        status.map_err(|_| anyhow!("client status failed: client exit: status unknown"))?;
    if !matches!(status.code(), Some(0 | 1)) {
        bail!("client status failed: {}", exit_cause(status));
    }
    let reading: Value =
        serde_json::from_slice(&output).map_err(|_| anyhow!("{STATUS_UNREADABLE}"))?;
    match (
        status.code(),
        reading.get("loggedIn").and_then(Value::as_bool),
    ) {
        (Some(0), Some(true)) => match reading.get("email").and_then(Value::as_str) {
            Some(account) if !account.is_empty() => Ok(Some(account.to_owned())),
            _ => bail!("{STATUS_NO_ACCOUNT}"),
        },
        (Some(1), Some(false)) => Ok(None),
        _ => bail!("{STATUS_UNREADABLE}"),
    }
}

impl Session {
    /// The account hook: the account the client's status names in this Agent's own home.
    pub async fn account(&self) -> Result<Option<String>> {
        self.account_within(STATUS_DEADLINE).await
    }
    /// The account hook, with the status's deadline given; `account` gives STATUS_DEADLINE.
    pub async fn account_within(&self, deadline: Duration) -> Result<Option<String>> {
        ensure!(self.config.v2, "account hook: a .v1 Agent has none");
        if self.config.state_dir.is_none()
            && let Some(error) = &self.config.configuration_error
        {
            bail!("{error}");
        }
        let home = client_home(&self.config)?;
        account_status(&self.config, &home, &self.tree, deadline).await
    }
    /// The sign-out hook: the client's own logout in this Agent's own home, the home kept.
    pub async fn sign_out(&mut self) -> Result<String> {
        self.sign_out_within(LOGOUT_DEADLINE).await
    }
    /// The sign-out hook, with the logout's deadline given; `sign_out` gives LOGOUT_DEADLINE.
    pub async fn sign_out_within(&mut self, deadline: Duration) -> Result<String> {
        ensure!(self.config.v2, "sign-out hook: a .v1 Agent has none");
        if self.config.state_dir.is_none()
            && let Some(error) = &self.config.configuration_error
        {
            bail!("{error}");
        }
        if self.stream.is_some() && self.failed.is_none() {
            self.failed = Some((self.turns_started, SIGNED_OUT_CAUSE.into()));
        }
        if let Some(note) = self.shutdown().await.note() {
            bail!("{note}");
        }
        let home = client_home(&self.config)?;
        logout(&self.config, &home, &self.tree, deadline).await?;
        Ok(SIGNED_OUT.into())
    }
}

/// How long the sign-in hook waits for the client to hand over its login's URL (A5 stage 0's wait).
pub const LOGIN_URL_DEADLINE: Duration = Duration::from_secs(30);
/// How long a started login may wait for its return before the wrapper stops it.
pub const LOGIN_LIMIT: Duration = Duration::from_secs(600);
/// The most bytes of a handed-over URL that the sign-in hook reads.
pub const LOGIN_URL_LIMIT: u64 = 8_192;
/// Set on the login alone: the wrapper's own program, run as the login's browser, keeps the URL.
pub const CAPTURE_VARIABLE: &str = "AGENT_SIGN_IN_CAPTURE";
/// The capture directory's name inside the instance's state directory, beside the home.
pub const CAPTURE_NAME: &str = "sign-in-capture";
pub const LOGIN_URL_UNUSABLE: &str = "client login gave no usable URL (PND-010)";
/// The sign-in hook's answer to a call carrying a return: its login carries none.
pub const SIGN_IN_RETURN_REFUSED: &str = "sign-in hook: this login carries no return (PND-013)";
/// The fixed message of the hook's URL request; the engine writes its own (IFC-049).
pub const SIGN_IN_MESSAGE: &str = "Sign the client in to this Agent's own home.";

/// The client's own browser login, the one that claude-code.v2 states as `browser`.
pub fn login_arguments() -> Vec<String> {
    vec!["auth".into(), "login".into()]
}

/// A login the sign-in hook started that still waits for its return.
pub struct Login {
    group: i32,
    start: Option<u64>,
    watch: tokio::task::JoinHandle<()>,
}

impl Login {
    pub(crate) fn is_running(&self) -> bool {
        if self.watch.is_finished() {
            return false;
        }
        match login_exited(self.group) {
            Ok(exited) => !exited,
            Err(error) => error.raw_os_error() != Some(libc::ECHILD),
        }
    }

    fn stop(self) {
        if let Some(start) = self.start {
            signal_group(self.group, start, libc::SIGKILL, true);
        }
        self.watch.abort();
    }
}

/// The wrapper's own program run as the login's `BROWSER`: keep the first URL handed over.
pub fn capture(file: &Path, url: &str) -> Result<()> {
    let own = file.with_extension(std::process::id().to_string());
    let mut written = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&own)?;
    written.write_all(url.as_bytes())?;
    drop(written);
    // A second URL never replaces the first.
    let _ = fs::hard_link(&own, file);
    fs::remove_file(&own)?;
    Ok(())
}

fn read_url(file: &Path) -> Result<String> {
    let mut bytes = Vec::new();
    // Neither a link nor a FIFO: the login, handed the capture path, may make either.
    fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(file)
        .and_then(|f| {
            if !f.metadata()?.file_type().is_file() {
                return Err(io::ErrorKind::InvalidInput.into());
            }
            f.take(LOGIN_URL_LIMIT + 1).read_to_end(&mut bytes)
        })
        .map_err(|_| anyhow!("{LOGIN_URL_UNUSABLE}"))?;
    match String::from_utf8(bytes) {
        Ok(url)
            if url.len() as u64 <= LOGIN_URL_LIMIT
                && url.starts_with("https://")
                && url.bytes().all(|b| b.is_ascii_graphic()) =>
        {
            Ok(url)
        }
        _ => bail!("{LOGIN_URL_UNUSABLE}"),
    }
}

fn spawn_login(
    c: &Config,
    home: &Path,
    tree: &ClientTree,
    file: &Path,
) -> Result<(tokio::process::Child, i32, Option<u64>)> {
    let mut state = tree.0.lock().unwrap();
    ensure!(
        !state.closed,
        "client login did not start: client process tree is closed (PND-010)"
    );
    let child = client_command(c, Some(home), &login_arguments())
        .env("BROWSER", &c.helper)
        .env(CAPTURE_VARIABLE, file)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| anyhow!("client login did not start: could not spawn client (PND-010)"))?;
    let pid = child.id().ok_or_else(|| {
        anyhow!("client login did not start: client process ID unavailable (PND-010)")
    })? as i32;
    let start = process(pid).ok().flatten().map(|p| p.start);
    state.groups.insert(pid, start);
    Ok((child, pid, start))
}

fn login_exited(pid: i32) -> io::Result<bool> {
    loop {
        // Keep the child waitable so its PID cannot be reused before we signal the group.
        let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
        let result = unsafe {
            libc::waitid(
                libc::P_PID,
                pid as libc::id_t,
                &mut info,
                libc::WEXITED | libc::WNOHANG | libc::WNOWAIT,
            )
        };
        if result == 0 {
            return Ok(unsafe { info.si_pid() } == pid);
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
}

async fn stop_starting_login(
    child: &mut tokio::process::Child,
    group: i32,
) -> io::Result<std::process::ExitStatus> {
    if child.id().is_some() && login_exited(group).is_ok() {
        // client_command created this child's group. The waitable leader pins its group ID.
        unsafe { libc::kill(-group, libc::SIGKILL) };
    }
    let _ = child.start_kill();
    child.wait().await
}

/// Start the client's login and wait for the URL it hands to its browser. Its errors are fixed
/// texts: no byte of the client's output reaches them.
async fn start_login(
    c: &Config,
    home: &Path,
    tree: &ClientTree,
    deadline: Duration,
) -> Result<(String, Login)> {
    let capture = c
        .state_dir
        .as_ref()
        .and_then(|dir| dir.canonicalize().ok())
        .ok_or_else(|| {
            anyhow!("client login did not start: could not resolve state directory (PND-010)")
        })?
        .join(CAPTURE_NAME);
    ensure!(
        !home.starts_with(&capture) && !capture.starts_with(home),
        "client login did not start: capture directory overlaps client home (PND-010)"
    );
    // A directory an interrupted call left holds no home: the check above rules that out.
    let _ = fs::remove_dir_all(&capture);
    fs::DirBuilder::new()
        .mode(0o700)
        .create(&capture)
        .map_err(|_| {
            anyhow!("client login did not start: could not create capture directory (PND-010)")
        })?;
    let file = capture.join("url");
    let (mut child, group, start) = match spawn_login(c, home, tree, &file) {
        Ok(spawned) => spawned,
        Err(error) => {
            let _ = fs::remove_dir_all(&capture);
            return Err(error);
        }
    };
    let until = Instant::now() + deadline;
    let outcome = loop {
        let url = fs::symlink_metadata(&file).is_ok().then(|| read_url(&file));
        match login_exited(group) {
            Ok(true) => {
                let cause = stop_starting_login(&mut child, group)
                    .await
                    .map_or_else(|_| "client exit: status unknown".into(), exit_cause);
                break Err(anyhow!("client login failed: {cause} (PND-010)"));
            }
            Ok(false) => {}
            Err(_) => {
                break Err(anyhow!(
                    "client login failed: client exit: status unknown (PND-010)"
                ));
            }
        }
        if let Some(url) = url {
            break url;
        }
        if Instant::now() >= until {
            break Err(anyhow!(
                "client login failed: no URL within {} seconds (PND-010)",
                deadline.as_secs()
            ));
        }
        sleep(POLL).await;
    };
    let _ = fs::remove_dir_all(&capture);
    let url = match outcome {
        Ok(url) => url,
        Err(error) => {
            // The login's own group alone: a conversation's client, if one runs, is not stopped.
            let _ = stop_starting_login(&mut child, group).await;
            return Err(error);
        }
    };
    let watch = tokio::spawn(async move {
        // The leader stays unreaped until its group is stopped, whether it exits or the limit comes.
        let until = Instant::now() + LOGIN_LIMIT;
        while matches!(login_exited(group), Ok(false)) && Instant::now() < until {
            sleep(POLL).await;
        }
        let _ = stop_starting_login(&mut child, group).await;
    });
    Ok((
        url,
        Login {
            group,
            start,
            watch,
        },
    ))
}

impl Session {
    /// The sign-in hook: start the client's login in this Agent's own home and give its URL.
    pub async fn sign_in(&mut self) -> Result<String> {
        self.sign_in_within(LOGIN_URL_DEADLINE).await
    }
    /// The sign-in hook, with the URL's deadline given; `sign_in` gives LOGIN_URL_DEADLINE.
    pub async fn sign_in_within(&mut self, deadline: Duration) -> Result<String> {
        self.start_sign_in(deadline).await.map_err(|error| {
            if error.to_string().contains("PND-010") {
                error
            } else {
                anyhow!("{error} (PND-010)")
            }
        })
    }

    async fn start_sign_in(&mut self, deadline: Duration) -> Result<String> {
        ensure!(self.config.v2, "sign-in hook: a .v1 Agent has none");
        if self.config.state_dir.is_none()
            && let Some(error) = &self.config.configuration_error
        {
            bail!("{error}");
        }
        if let Some(login) = self.login.take() {
            login.stop();
        }
        let home = client_home(&self.config)?;
        let (url, login) = start_login(&self.config, &home, &self.tree, deadline).await?;
        self.login = Some(login);
        Ok(url)
    }
}
