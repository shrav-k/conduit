use std::{
    ffi::OsStr,
    io,
    os::fd::{AsFd, OwnedFd},
    os::unix::{ffi::OsStrExt, net::UnixStream as StdUnixStream},
    process::Stdio,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
    process::{Child, Command},
};

use crate::TOKEN_VARIABLE;

const SERVER: &str = "github-mcp-server";
const STARTED: u8 = 0;
const NOT_FOUND: u8 = 1;
const START_FAILED: u8 = 2;

#[doc(hidden)]
pub const SUPERVISE_UPSTREAM: &str = "--supervise-upstream";

pub(crate) fn spawn() -> Result<(Child, UnixStream), &'static str> {
    let path = std::env::var_os("PATH")
        .filter(|path| !path.is_empty())
        .ok_or("github-mcp-server was not found on PATH")?;
    let home = std::env::var_os("HOME").ok_or("front HOME is not set")?;
    let failed = "github-mcp-server could not be started";
    let (front, supervisor) = StdUnixStream::pair().map_err(|_| failed)?;
    let front = nonblocking(front).map_err(|_| failed)?;
    let child = Command::new(std::env::current_exe().map_err(|_| failed)?)
        .arg(SUPERVISE_UPSTREAM)
        .env_clear()
        .env("PATH", path)
        .env("HOME", home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        // A signal sent to the front's whole process group must leave the
        // supervisor alive long enough to reap the server.
        .process_group(0)
        // The supervisor uses fd 2 as a private control socket. The server gets
        // separate stderr, and the socket never enters its descriptor table.
        .stderr(OwnedFd::from(supervisor))
        .spawn()
        .map_err(|_| failed)?;
    Ok((child, front))
}

pub(crate) async fn start(control: &mut UnixStream, token: &str) -> Result<(), &'static str> {
    let failed = "github-mcp-server could not be started";
    let len = u32::try_from(token.len()).map_err(|_| failed)?;
    control.write_u32(len).await.map_err(|_| failed)?;
    control
        .write_all(token.as_bytes())
        .await
        .map_err(|_| failed)?;
    match control.read_u8().await {
        Ok(STARTED) => Ok(()),
        Ok(NOT_FOUND) => Err("github-mcp-server was not found on PATH"),
        _ => Err(failed),
    }
}

#[doc(hidden)]
pub async fn supervise_upstream() -> io::Result<()> {
    let descriptor = std::io::stderr().as_fd().try_clone_to_owned()?;
    let mut control = nonblocking(StdUnixStream::from(descriptor))?;
    let length = u64::from(control.read_u32().await?);
    let mut token = Vec::new();
    let received = (&mut control).take(length).read_to_end(&mut token).await?;
    if received as u64 != length {
        return Err(io::ErrorKind::UnexpectedEof.into());
    }

    // Keep the credential off the supervisor's command line and environment.
    // Only the server's exec receives it as an environment variable.
    let start = Command::new(SERVER)
        .args(["stdio", "--read-only", "--toolsets", "issues,pull_requests"])
        .env(TOKEN_VARIABLE, OsStr::from_bytes(&token))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn();
    drop(token);
    let mut server = match start {
        Ok(server) => server,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return control.write_u8(NOT_FOUND).await;
        }
        Err(_) => return control.write_u8(START_FAILED).await,
    };
    if control.write_u8(STARTED).await.is_ok() {
        tokio::select! {
            result = server.wait() => return result.map(|_| ()),
            _ = control.read_u8() => {}
        }
    }
    // The front holds the only peer. EOF also arrives on abrupt exit, when
    // running the front's cleanup is impossible.
    let _ = server.start_kill();
    server.wait().await.map(|_| ())
}

fn nonblocking(stream: StdUnixStream) -> io::Result<UnixStream> {
    stream.set_nonblocking(true)?;
    UnixStream::from_std(stream)
}
