//! Independent owed-stop worker. Never constructs a server, opens a gate, or starts a run.
use crate::{
    Config, FileStore, SshBackend,
    durable::{OwnerLock, try_lock},
};
use runtime_provider_kit::{Result, StateStore, drain_owed_stops};
use std::{fs, future::Future, os::unix::ffi::OsStrExt, path::Path, time::Duration};
use tokio::time::{Instant, sleep, sleep_until};

const POLL: Duration = Duration::from_secs(1);
const WATCH: Duration = Duration::from_secs(60);
const RESUME: Duration = Duration::from_secs(11);
const MAX_BACKOFF: Duration = Duration::from_secs(300);

// Fields drop in declaration order, including on cancellation: backend/attempts
// live inside run's loop and drop first, then drainer.lock, then owner.lock.
struct Locks<L: FnMut(&'static str)> {
    drainer: Option<fs::File>,
    owner: Option<OwnerLock>,
    log: L,
}
impl<L: FnMut(&'static str)> Drop for Locks<L> {
    fn drop(&mut self) {
        // This also runs if the task is cancelled while draining or yielding.
        self.drainer = None;
        (self.log)("drainer.lock released before owner.lock");
    }
}

fn missing(root: &Path, log: &mut impl FnMut(&'static str)) -> bool {
    for (path, message) in [
        (root.to_path_buf(), "durable subdirectory gone; exiting"),
        (root.join("ledger.json"), "ledger.json gone; exiting"),
    ] {
        if fs::symlink_metadata(path).is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound) {
            log(message);
            return true;
        }
    }
    false
}

enum Event<T> {
    Done(T),
    Yield,
    Gone,
    Bound,
}

/// The attempt remains in this stack frame, so cancellation drops it before the backend.
async fn interrupt<T>(
    root: &Path,
    future: impl Future<Output = T>,
    bound: Option<Instant>,
    log: &mut impl FnMut(&'static str),
) -> Event<T> {
    let signals = async {
        loop {
            if missing(root, log) {
                return Event::Gone;
            }
            if root.join("yield").exists() {
                return Event::Yield;
            }
            sleep(POLL).await;
        }
    };
    let deadline = async {
        match bound {
            Some(until) => sleep_until(until).await,
            None => std::future::pending().await,
        }
    };
    tokio::select! {
        biased;
        event = signals => event,
        _ = deadline => Event::Bound,
        result = future => Event::Done(result),
    }
}

fn retired(state: &Path) -> bool {
    fs::symlink_metadata(state).is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound)
        && state
            .parent()
            .is_some_and(|p| fs::metadata(p).is_ok_and(|m| m.is_dir()))
}

async fn watch(
    root: &Path,
    state: &Path,
    config: &Config,
    store: &FileStore,
    log: &mut impl FnMut(&'static str),
) -> Event<()> {
    let until = Instant::now() + WATCH;
    loop {
        if missing(root, log) {
            return Event::Gone;
        }
        if root.join("yield").exists() {
            return Event::Yield;
        }
        if retired(state) {
            let empty = store
                .load()
                .await
                .is_ok_and(|r| r.iter().all(|r| !r.owed_stop));
            let bound =
                fs::read(root.join("state-dir")).is_ok_and(|b| b == state.as_os_str().as_bytes());
            if empty && bound {
                // Refuse to remove a configured key outside this durable instance.
                if fs::symlink_metadata(&config.private_key).is_ok_and(|m| m.is_file())
                    && config
                        .private_key
                        .canonicalize()
                        .is_ok_and(|p| p.starts_with(root))
                    && fs::remove_file(&config.private_key).is_err()
                {
                    log("privateKey could not be removed");
                }
                match fs::remove_file(root.join("known_hosts")) {
                    Ok(()) => (),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                    Err(_) => log("known_hosts could not be removed"),
                }
                log("instance retired with no owed stops; key cleanup finished");
            } else {
                log("instance retired; owed stops or changed state-dir retain key material");
            }
            return Event::Done(());
        }
        if Instant::now() >= until {
            log("retirement watch ended; retaining key material");
            return Event::Done(());
        }
        sleep_until((Instant::now() + POLL).min(until)).await;
    }
}

/// Factory seam preserves real ownership and kit storage with a fake remote in tests.
/// The factory receives a clone of the same lease: the outer guard outlives its backend.
pub async fn run(
    root: &Path,
    state: &Path,
    mut build: impl FnMut(&Config, OwnerLock) -> Result<SshBackend>,
    mut log: impl FnMut(&'static str),
) {
    if missing(root, &mut log) {
        return;
    }
    let root = match root.canonicalize() {
        Ok(root) => root,
        Err(_) => {
            log("cannot access durable subdirectory; exiting");
            return;
        }
    };
    let root = root.as_path();
    let drainer = match try_lock(root, "drainer.lock") {
        Ok(Some(lock)) => lock,
        Ok(None) => {
            log("drainer.lock held; exiting");
            return;
        }
        Err(_) => {
            log("cannot open drainer.lock; exiting");
            return;
        }
    };
    let mut locks = Locks {
        drainer: Some(drainer),
        owner: None,
        log,
    };
    let log = &mut locks.log;
    loop {
        locks.owner = Some(loop {
            if missing(root, log) {
                return;
            }
            match OwnerLock::try_existing(root) {
                Ok(Some(owner)) => break owner,
                Ok(None) => log("waiting for owner.lock"),
                Err(_) => log("cannot open owner.lock; retrying"),
            }
            sleep(POLL).await;
        });
        let config = match Config::read_drainer(root) {
            Ok(config) => config,
            Err(_) => {
                log("drainer-config missing or invalid; exiting");
                return;
            }
        };
        let until = Instant::now() + config.max_lifetime + config.stop_timeout;
        let _ = fs::remove_file(root.join("yield"));
        log("owner.lock acquired");
        let store = FileStore::new(root);
        let mut backend = build(&config, locks.owner.as_ref().unwrap().clone()).ok();
        if backend.is_none() {
            log("cannot build backend from drainer-config; retrying owed stops");
        }
        let mut backoff = config.drain_interval.min(MAX_BACKOFF);
        let mut yielded = false;
        loop {
            if missing(root, log) {
                return;
            }
            let ledger = store.load().await;
            if ledger
                .as_ref()
                .is_ok_and(|r| r.iter().all(|r| !r.owed_stop))
            {
                log("nothing owed");
                break;
            }
            let last = Instant::now() >= until;
            log(if last {
                "last drain attempt at lifetime bound"
            } else {
                "drain attempt"
            });
            if backend.is_none() && ledger.is_ok() {
                backend = build(&config, locks.owner.as_ref().unwrap().clone()).ok();
            }
            let attempt = async {
                if let Some(backend) = backend.as_ref() {
                    if drain_owed_stops(backend, &store).await.is_err() {
                        return false;
                    }
                } else {
                    return false;
                }
                store
                    .load()
                    .await
                    .is_ok_and(|r| r.iter().all(|r| !r.owed_stop))
            };
            match interrupt(root, attempt, if last { None } else { Some(until) }, log).await {
                Event::Gone => return,
                Event::Yield => {
                    yielded = true;
                    break;
                }
                Event::Bound => {
                    // Contact may have been cancelled before recording its UID. Rebuild
                    // under the same lease, with no masters from the cancelled attempt.
                    backend = None;
                    continue;
                }
                Event::Done(true) => break,
                Event::Done(false) => log("drain attempt failed; ledger remains authoritative"),
            }
            if last {
                log("lifetime bound reached; retaining any owed stops");
                break;
            }
            match interrupt(
                root,
                sleep_until((Instant::now() + backoff).min(until)),
                None,
                log,
            )
            .await
            {
                Event::Gone => return,
                Event::Yield => {
                    yielded = true;
                    break;
                }
                _ => (),
            }
            backoff = (backoff * 2).min(MAX_BACKOFF);
        }
        // SSH masters die before either key removal or release to the next owner.
        drop(backend);
        if !yielded {
            match watch(root, state, &config, &store, log).await {
                Event::Yield => yielded = true,
                _ => return,
            }
        }
        locks.owner = None;
        if yielded {
            log("yielding owner.lock to provider");
            sleep(RESUME).await;
            // A held lock may be the sweep's brief try. Always return to waiting.
        }
    }
}
