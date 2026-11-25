//! The run's locks: an exclusive lock on `state/queue.lock`, held until the run exits, and,
//! where `legacy_lock` is set, the kit's lock directory, made as the kit's lock script makes it.

use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

/// The words an `owner` file of the queue's begins with.
pub const OWNER_PREFIX: &str = "wo queue";

/// What follows, in an `owner` line of the queue's, the path of the queue's lock its run held.
pub const OWNER_LOCK: &str = " lock ";

/// How long a run tries the queue's lock before it says another holds it: `status` and `retry`
/// take it for an instant to see whether a run holds it.
const GRACE: Duration = Duration::from_millis(100);

/// Tries the lock until it is taken or `GRACE` has passed.
fn try_lock_within_grace(file: &File) -> Result<(), TryLockError> {
    let deadline = Instant::now() + GRACE;
    loop {
        match file.try_lock() {
            Err(TryLockError::WouldBlock) if Instant::now() < deadline => {
                thread::sleep(Duration::from_millis(5));
            }
            other => return other,
        }
    }
}

/// Takes the queue's lock; `Ok(None)` when another run holds it and `no_wait` is set.
pub fn queue_lock(path: &Path, no_wait: bool) -> Result<Option<File>, String> {
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
        .map_err(|error| {
            format!(
                "the queue's lock {} cannot be opened: {error}",
                path.display()
            )
        })?;
    match try_lock_within_grace(&file) {
        Ok(()) => Ok(Some(file)),
        Err(TryLockError::WouldBlock) => {
            if no_wait {
                println!(
                    "the queue's lock {} is held by another run; under --no-wait no entry ran",
                    path.display()
                );
                return Ok(None);
            }
            println!(
                "waiting for the queue's lock {}, which another run holds",
                path.display()
            );
            file.lock().map_err(|error| {
                format!(
                    "the queue's lock {} cannot be taken: {error}",
                    path.display()
                )
            })?;
            Ok(Some(file))
        }
        Err(TryLockError::Error(error)) => Err(format!(
            "the queue's lock {} cannot be taken: {error}",
            path.display()
        )),
    }
}

/// Whether a run holds the queue's lock now.
pub fn queue_lock_held(path: &Path) -> bool {
    match File::open(path) {
        Ok(file) => matches!(file.try_lock(), Err(TryLockError::WouldBlock)),
        Err(_) => false,
    }
}

/// The kit's lock directory, while this run holds it; dropping it removes it.
#[derive(Debug)]
pub struct LegacyLock {
    dir: PathBuf,
    owner: String,
}

impl Drop for LegacyLock {
    fn drop(&mut self) {
        if owner_line(&self.dir).as_deref() == Some(self.owner.as_str()) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }
}

/// The first line of the lock directory's `owner` file, when there is one.
pub fn owner_line(dir: &Path) -> Option<String> {
    fs::read_to_string(dir.join("owner"))
        .ok()
        .map(|text| text.lines().next().unwrap_or("").trim_end().to_string())
}

/// Whether an `owner` line of the queue's was left by a run that stopped. Its run held the queue's
/// lock the line names, so that run is gone when the lock is `ours`, which this run holds, or can
/// be taken now. A line that names no lock, as an earlier queue wrote, was left by a run that
/// stopped, since this run holds the queue's lock.
fn left_by_a_stopped_run(line: &str, ours: &Path) -> bool {
    let Some((_, lock)) = line.split_once(OWNER_LOCK) else {
        return true;
    };
    let same = |a: &Path, b: &Path| match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    };
    if same(Path::new(lock), ours) {
        return true;
    }
    match File::open(lock) {
        Ok(file) => !matches!(file.try_lock(), Err(TryLockError::WouldBlock)),
        Err(_) => true,
    }
}

/// Makes the kit's lock directory; `Ok(None)` when another holds it and `no_wait` is set.
///
/// The `owner` line names `wo queue`, the process ID and `queue_lock`, the queue's lock this run
/// holds. A directory whose `owner` names `wo queue` was left by a run that stopped when the
/// queue's lock it names can be taken, and is removed; while a run under another `state` holds
/// that lock, the directory is held as the kit's is.
pub fn legacy_lock(
    dir: &Path,
    queue_lock: &Path,
    no_wait: bool,
) -> Result<Option<LegacyLock>, String> {
    let owner = format!(
        "{OWNER_PREFIX} pid {}{OWNER_LOCK}{}",
        std::process::id(),
        queue_lock.display()
    );
    let mut said = false;
    loop {
        match fs::create_dir(dir) {
            Ok(()) => {
                let lock = LegacyLock {
                    dir: dir.to_path_buf(),
                    owner: owner.clone(),
                };
                fs::write(dir.join("owner"), format!("{owner}\n")).map_err(|error| {
                    format!(
                        "the kit's lock {} has no owner file: {error}",
                        dir.display()
                    )
                })?;
                return Ok(Some(lock));
            }
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                let line = owner_line(dir);
                if let Some(line) = line.as_deref().filter(|line| {
                    line.starts_with(OWNER_PREFIX) && left_by_a_stopped_run(line, queue_lock)
                }) {
                    fs::remove_dir_all(dir).map_err(|error| {
                        format!(
                            "the kit's lock {}, left by a stopped run, cannot be removed: {error}",
                            dir.display()
                        )
                    })?;
                    println!(
                        "removed the kit's lock {}, left by a stopped run; owner: {line}",
                        dir.display()
                    );
                    continue;
                }
                let shown = line.unwrap_or_else(|| "(no owner file)".to_string());
                if no_wait {
                    println!(
                        "the kit's lock {} is held, owner: {shown}; under --no-wait no entry ran",
                        dir.display()
                    );
                    return Ok(None);
                }
                if !said {
                    println!(
                        "waiting for the kit's lock {}, owner: {shown}",
                        dir.display()
                    );
                    said = true;
                }
                thread::sleep(Duration::from_millis(500));
            }
            Err(error) => {
                return Err(format!(
                    "the kit's lock {} cannot be made: {error}",
                    dir.display()
                ));
            }
        }
    }
}
