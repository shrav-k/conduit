use crate::STOP_GRACE;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    os::{
        fd::{AsRawFd, FromRawFd},
        unix::process::CommandExt,
    },
    process::Command,
    time::{Duration, Instant},
};

#[derive(Serialize, Deserialize)]
pub(crate) struct Launch {
    pub command: Vec<String>,
    pub environment: BTreeMap<String, String>,
    pub lifetime_ms: u64,
}

/// A saved group ID is only evidence of absence, never authority to send a signal.
/// Older markers without this witness cannot establish that a run has stopped.
#[derive(Serialize, Deserialize)]
pub(crate) struct Marker {
    #[serde(flatten)]
    pub record: runtime_provider_kit::RunRecord,
    #[serde(default)]
    pub witness: Option<GroupWitness>,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum GroupWitness {
    Unstarted,
    Group(i32),
}

/// Internal mode, entered before constructing Tokio. FD 3 is the provider's lifetime pipe;
/// FD 4 owns the already-locked marker. Neither descriptor reaches the run's program.
pub fn guard_main() -> std::io::Result<()> {
    // SAFETY: only our launcher invokes this mode, with these owned descriptors.
    let mut pipe = unsafe { File::from_raw_fd(3) };
    let mut marker = unsafe { File::from_raw_fd(4) };
    // SAFETY: set descriptor flags and a process-local signal disposition before spawning.
    unsafe {
        libc::fcntl(3, libc::F_SETFD, libc::FD_CLOEXEC);
        libc::fcntl(4, libc::F_SETFD, libc::FD_CLOEXEC);
        libc::signal(libc::SIGTERM, libc::SIG_IGN);
    }
    let mut size = [0; 8];
    pipe.read_exact(&mut size)?;
    let size = u64::from_be_bytes(size);
    if size > 16 * 1024 * 1024 {
        return Err(std::io::Error::other("launch too large"));
    }
    let mut bytes = vec![0; size as usize];
    pipe.read_exact(&mut bytes)?;
    let launch: Launch =
        serde_json::from_slice(&bytes).map_err(|_| std::io::Error::other("invalid launch"))?;
    let program = launch
        .command
        .first()
        .ok_or_else(|| std::io::Error::other("missing program"))?;
    // Persist the group before any program can exist. If the guard alone dies, a free
    // lock is insufficient: the backend must also establish that this group is absent.
    // The marker lock excludes observers throughout the in-place durable update.
    let mut record: Marker = serde_json::from_reader(&mut marker)?;
    record.witness = Some(GroupWitness::Group(unsafe { libc::getpgrp() }));
    let bytes = serde_json::to_vec(&record)?;
    marker.seek(SeekFrom::Start(0))?;
    marker.write_all(&bytes)?;
    marker.set_len(bytes.len() as u64)?;
    marker.sync_all()?;
    let mut command = Command::new(program);
    command
        .args(&launch.command[1..])
        .env_clear()
        .envs(launch.environment);
    // SAFETY: restore the normal termination disposition in the child only.
    unsafe {
        command.pre_exec(|| {
            libc::signal(libc::SIGTERM, libc::SIG_DFL);
            Ok(())
        });
    }
    let mut child = command.spawn()?;
    // Only the run retains stdio; EOF must be observable when it exits.
    unsafe {
        libc::close(0);
        libc::close(1);
        libc::close(2);
    }
    // Once a program may exist, every exit from the guard kills its whole group.
    let ready = pipe.write_all(b"R");
    let deadline = Instant::now() + Duration::from_millis(launch.lifetime_ms);
    if ready.is_ok() {
        loop {
            if Instant::now() >= deadline || !matches!(child.try_wait(), Ok(None)) {
                break;
            }
            let mut pfd = libc::pollfd {
                fd: pipe.as_raw_fd(),
                events: libc::POLLIN | libc::POLLHUP,
                revents: 0,
            };
            // SAFETY: pfd points to one initialized poll descriptor.
            let n = unsafe { libc::poll(&mut pfd, 1, 10) };
            if n < 0 {
                if std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                    continue;
                }
                break;
            }
            if n > 0 {
                let mut byte = [0];
                if matches!(pipe.read(&mut byte), Ok(1)) && byte[0] == b'S' {
                    // The guard ignores TERM; the run has its ordinary disposition.
                    unsafe {
                        libc::kill(0, libc::SIGTERM);
                    }
                    std::thread::sleep(STOP_GRACE);
                }
                break;
            }
        }
    }
    // SAFETY: the launcher made this guard the leader of a new process group. Delivery of
    // SIGKILL to every member requests termination. Lock release alone does not certify
    // group death; the backend additionally checks the durable absence witness.
    unsafe {
        libc::kill(0, libc::SIGKILL);
    }
    Err(std::io::Error::other("group termination failed"))
}
