use super::{Config, client_command_at};
use anyhow::{Result, bail};
use std::{
    cmp::Ordering,
    ffi::CString,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::{
    io::AsyncReadExt,
    process::Command,
    time::{Instant, timeout_at},
};

const NAME: &str = "claude";
const MINIMUM: &str = "2.1.318";

pub(super) async fn check(config: &Config, home: &Path) -> Result<String> {
    let Some(path) = find(config) else {
        bail!(
            "client: {NAME} not found on PATH; install {MINIMUM} or later by the vendor's own route"
        );
    };
    let display = path.display();
    let command = client_command_at(config, &path, Some(home), &["--version".into()]);
    if let Some(bytes) = probe(command, Instant::now() + Duration::from_secs(5)).await
        && let Ok(text) = std::str::from_utf8(&bytes)
        && let Some(version) = text.split_ascii_whitespace().next()
        && let Some(order) = compare_to_minimum(version)
    {
        if order == Ordering::Less {
            bail!(
                "client: {NAME} {version} at {display} is older than {MINIMUM}, the lowest version this release was tested with; update it by the vendor's own route"
            );
        }
        return Ok(format!(
            "client: {NAME} {version} at {display}; lowest tested {MINIMUM}"
        ));
    }
    bail!(
        "client: {NAME} at {display} gave no version this release can read from --version; the lowest version it was tested with is {MINIMUM}"
    );
}

fn find(config: &Config) -> Option<PathBuf> {
    let directory = config.client_directory();
    if config.program != Path::new(NAME) {
        return Some(directory.join(&config.program));
    }
    std::env::split_paths(config.env.get("PATH")?)
        .map(|entry| directory.join(entry).join(NAME))
        .find(|path| {
            if !path.is_file() {
                return false;
            }
            let Ok(path) = CString::new(path.as_os_str().as_bytes()) else {
                return false;
            };
            // Keep the path as found while asking the OS whether it is executable.
            unsafe { libc::access(path.as_ptr(), libc::X_OK) == 0 }
        })
}

pub(super) async fn probe(mut command: Command, deadline: Instant) -> Option<Vec<u8>> {
    let mut child = command
        .process_group(0)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let group = child.id()? as libc::pid_t;
    let output = timeout_at(deadline, async {
        let mut bytes = Vec::new();
        child
            .stdout
            .take()?
            .take(257)
            .read_to_end(&mut bytes)
            .await
            .ok()?;
        (bytes.len() <= 256).then_some(bytes)
    })
    .await;
    if let Ok(Some(bytes)) = output
        && let Ok(Ok(status)) = timeout_at(deadline, child.wait()).await
    {
        return status.success().then_some(bytes);
    }

    // Signal the new group before reaping its leader, so launchers cannot leave
    // a client behind on a timeout, an oversized reply, or an unreadable pipe.
    unsafe { libc::kill(-group, libc::SIGKILL) };
    let _ = child.start_kill();
    let _ = child.wait().await;
    None
}

fn identifiers(text: &str, prerelease: bool) -> bool {
    text.split('.').all(|id| {
        !id.is_empty()
            && id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            && (!prerelease || !id.bytes().all(|byte| byte.is_ascii_digit()) || number(id))
    })
}

fn number(text: &str) -> bool {
    !text.is_empty()
        && text.bytes().all(|byte| byte.is_ascii_digit())
        && (text.len() == 1 || !text.starts_with('0'))
}

fn compare_to_minimum(version: &str) -> Option<Ordering> {
    let version = match version.split_once('+') {
        Some((version, build)) if identifiers(build, false) => version,
        Some(_) => return None,
        None => version,
    };
    let (core, prerelease) = match version.split_once('-') {
        Some((core, prerelease)) if identifiers(prerelease, true) => (core, true),
        Some(_) => return None,
        None => (version, false),
    };
    let mut parts = core.split('.');
    let core = [parts.next()?, parts.next()?, parts.next()?];
    if parts.next().is_some() || !core.iter().all(|part| number(part)) {
        return None;
    }
    // The bound is a release, so only an equal numeric core needs a prerelease comparison.
    let order = core
        .into_iter()
        .zip(MINIMUM.split('.'))
        .map(|(found, minimum)| {
            found
                .len()
                .cmp(&minimum.len())
                .then_with(|| found.cmp(minimum))
        })
        .find(|order| *order != Ordering::Equal)
        .unwrap_or(Ordering::Equal);
    Some(if order == Ordering::Equal && prerelease {
        Ordering::Less
    } else {
        order
    })
}
