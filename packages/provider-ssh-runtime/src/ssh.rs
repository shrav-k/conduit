//! The only subprocess boundary. Tests replace Executor without starting ssh or sshd.
use crate::{
    codec::{command, option_path},
    config::{Config, MachineConfig},
};
use async_trait::async_trait;
use runtime_provider_kit::{Channel, Result, Unavailable};
use std::{
    fs::File,
    path::{Path, PathBuf},
    pin::Pin,
    process::Stdio,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
    process::{Child, ChildStdin, ChildStdout, Command},
    time::{Instant, sleep, timeout},
};
use tokio_util::sync::CancellationToken;

pub struct Invocation {
    pub program: PathBuf,
    pub args: Vec<String>,
}
pub struct Output {
    pub success: bool,
    pub stdout: Vec<u8>,
}
pub trait Master: Send {
    fn alive(&mut self) -> bool;
}
pub struct Session {
    pub channel: Channel,
    pub ended: CancellationToken,
    /// Ask the owned SSH child to exit; ended acknowledges that it was reaped.
    pub terminate: CancellationToken,
}
#[async_trait]
pub trait Executor: Send + Sync + 'static {
    async fn master(&self, call: Invocation, socket: &Path) -> Result<Box<dyn Master>>;
    async fn execute(&self, call: Invocation, input: Vec<u8>) -> Result<Output>;
    /// stderr is already open privately, never a pipe that could fill and deadlock.
    async fn session(&self, call: Invocation, stderr: File) -> Result<Session>;
}
/// Resolve the executable once at startup. No environment is ever passed to it.
pub fn find_ssh() -> Result<PathBuf> {
    use std::os::unix::fs::PermissionsExt;
    let paths = std::env::var_os("PATH").ok_or(Unavailable)?;
    for dir in std::env::split_paths(&paths) {
        let p = dir.join("ssh");
        if p.metadata()
            .is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
            && let Ok(path) = p.canonicalize()
        {
            return Ok(path);
        }
    }
    Err(Unavailable)
}
pub fn invocation(
    ssh: &Path,
    config: &Config,
    machine: &MachineConfig,
    pinned: &Path,
    socket: &Path,
    words: Option<&[String]>,
) -> Result<Invocation> {
    fn path(p: &Path) -> Result<String> {
        option_path(p.to_str().ok_or(Unavailable)?).map_err(|_| Unavailable)
    }
    let mut args: Vec<String> = ["-F", "none", "-T", "-e", "none"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    for option in [
        "IdentitiesOnly=yes".into(),
        "IdentityAgent=none".into(),
        format!("IdentityFile={}", path(&config.private_key)?),
        format!("User={}", machine.user),
        format!("Port={}", machine.port),
        format!("UserKnownHostsFile={}", path(pinned)?),
        "GlobalKnownHostsFile=/dev/null".into(),
        format!("HostKeyAlias={}", machine.hostname),
        "StrictHostKeyChecking=yes".into(),
        "UpdateHostKeys=no".into(),
        "HostKeyAlgorithms=ssh-ed25519".into(),
        "BatchMode=yes".into(),
        "ConnectTimeout=5".into(),
        "ServerAliveInterval=10".into(),
        "ServerAliveCountMax=3".into(),
        "LogLevel=ERROR".into(),
        format!("ControlPath={}", path(socket)?),
    ] {
        args.extend(["-o".into(), option]);
    }
    if words.is_none() {
        args.extend([
            "-N".into(),
            "-o".into(),
            "ControlMaster=yes".into(),
            "-o".into(),
            "ControlPersist=no".into(),
        ]);
    }
    args.push(machine.host.clone());
    if let Some(words) = words {
        args.push(command(words));
    }
    Ok(Invocation {
        program: ssh.into(),
        args,
    })
}
fn process(call: Invocation) -> Command {
    let mut command = Command::new(call.program);
    command.args(call.args).env_clear().kill_on_drop(true);
    command
}
struct ProcessMaster(Child);
impl Master for ProcessMaster {
    fn alive(&mut self) -> bool {
        matches!(self.0.try_wait(), Ok(None))
    }
}
pub struct ProcessExecutor;
#[async_trait]
impl Executor for ProcessExecutor {
    async fn master(&self, call: Invocation, socket: &Path) -> Result<Box<dyn Master>> {
        let mut child = process(call)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| Unavailable)?;
        let until = Instant::now() + Duration::from_secs(6);
        loop {
            if !matches!(child.try_wait(), Ok(None)) {
                return Err(Unavailable);
            }
            if socket.exists() {
                return Ok(Box::new(ProcessMaster(child)));
            }
            if Instant::now() >= until {
                return Err(Unavailable);
            }
            sleep(Duration::from_millis(20)).await;
        }
    }
    async fn execute(&self, call: Invocation, input: Vec<u8>) -> Result<Output> {
        let mut child = process(call)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| Unavailable)?;
        let mut stdin = child.stdin.take().ok_or(Unavailable)?;
        let mut stdout = child.stdout.take().ok_or(Unavailable)?;
        timeout(Duration::from_secs(45), async {
            let send = async move {
                stdin.write_all(&input).await?;
                // ChildStdin::shutdown does not close the Unix pipe. The remote
                // shell needs EOF before it can finish a script sent with -s.
                drop(stdin);
                Ok::<_, std::io::Error>(())
            };
            let receive = async {
                let mut bytes = Vec::new();
                (&mut stdout)
                    .take(1024 * 1024 + 1)
                    .read_to_end(&mut bytes)
                    .await?;
                Ok::<_, std::io::Error>(bytes)
            };
            let (_, bytes) = tokio::try_join!(send, receive).map_err(|_| Unavailable)?;
            if bytes.len() > 1024 * 1024 {
                return Err(Unavailable);
            }
            let status = child.wait().await.map_err(|_| Unavailable)?;
            if status.code() == Some(255) {
                return Err(Unavailable);
            }
            let success = status.success();
            Ok(Output {
                success,
                stdout: bytes,
            })
        })
        .await
        .map_err(|_| Unavailable)?
    }
    async fn session(&self, call: Invocation, stderr: File) -> Result<Session> {
        let mut child = process(call)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(stderr)
            .spawn()
            .map_err(|_| Unavailable)?;
        let channel = ProcessChannel {
            stdin: Some(child.stdin.take().ok_or(Unavailable)?),
            stdout: child.stdout.take().ok_or(Unavailable)?,
        };
        let ended = CancellationToken::new();
        let signal = ended.clone();
        let terminate = CancellationToken::new();
        let requested = terminate.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = child.wait() => (),
                _ = requested.cancelled() => { let _ = child.kill().await; let _ = child.wait().await; }
            }
            signal.cancel();
        });
        Ok(Session {
            channel: Box::new(channel),
            ended,
            terminate,
        })
    }
}
struct ProcessChannel {
    stdin: Option<ChildStdin>,
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
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        match self.stdin.as_mut() {
            Some(stdin) => Pin::new(stdin).poll_write(cx, buf),
            None => Poll::Ready(Err(std::io::ErrorKind::BrokenPipe.into())),
        }
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.stdin.as_mut() {
            Some(stdin) => Pin::new(stdin).poll_flush(cx),
            None => Poll::Ready(Ok(())),
        }
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        self.stdin.take();
        Poll::Ready(Ok(()))
    }
}
