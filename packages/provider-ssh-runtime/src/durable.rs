//! Local durable ownership, shared by the server and its future external drainer.
use crate::{
    codec,
    storage::{atomic_write, private_directory},
};
use runtime_provider_kit::{Result, Unavailable};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    os::{
        fd::AsRawFd,
        unix::{
            ffi::OsStrExt,
            fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
        },
    },
    path::{Path, PathBuf},
};

pub(crate) fn refusal(text: String) -> Unavailable {
    eprintln!("SSH-R refuses startup: {text}");
    Unavailable::because(text)
}
fn at<T>(result: std::io::Result<T>, place: &Path) -> Result<T> {
    result.map_err(|_| {
        refusal(format!(
            "durable state {} cannot be accessed",
            place.display()
        ))
    })
}

/// Hash the raw OS bytes, without canonicalization or a trailing newline.
pub fn instance_directory(durable: &Path, state: &Path) -> PathBuf {
    durable.join(format!(
        "{:x}",
        Sha256::digest(state.as_os_str().as_bytes())
    ))
}

pub(crate) fn validate_directory(durable: &Path, state: Option<&Path>) -> Result<()> {
    if codec::option_path(durable.to_str().unwrap_or("")).is_err() {
        return Err(refusal(
            "setting SSH_RUNTIME_DURABLE_DIR (durableDir): absolute SSH option path required"
                .into(),
        ));
    }
    // SAFETY: geteuid takes no arguments and has no preconditions.
    validate_owner(durable, unsafe { libc::geteuid() })?;
    if let Some(state) = state {
        let expected = instance_directory(durable, state);
        let canonical = at(durable.canonicalize(), durable)?;
        let state = at(state.canonicalize(), state)?;
        let parent = state
            .parent()
            .ok_or_else(|| refusal("CONDUIT_STATE_DIR parent required".into()))?;
        if canonical.starts_with(parent) {
            return Err(refusal(format!(
                "SSH_RUNTIME_DURABLE_DIR (durableDir) {} must be outside CONDUIT_STATE_DIR and its parent {}",
                durable.display(),
                parent.display()
            )));
        }
        let mut old = Vec::new();
        for name in [
            "ledger.json",
            "markers",
            "generations",
            "part",
            "known_hosts",
        ] {
            match fs::symlink_metadata(state.join(name)) {
                Ok(_) => old.push(name),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                Err(_) => {
                    return Err(refusal(format!(
                        "CONDUIT_STATE_DIR old-layout check {} cannot be read",
                        state.join(name).display()
                    )));
                }
            }
        }
        if !old.is_empty() {
            return Err(refusal(format!(
                "CONDUIT_STATE_DIR {} has old layout: {}; move each into {}",
                state.display(),
                old.join(", "),
                expected.display()
            )));
        }
    }
    Ok(())
}

fn validate_owner(durable: &Path, expected_uid: u32) -> Result<()> {
    let meta = fs::symlink_metadata(durable).map_err(|_| {
        refusal(format!(
            "SSH_RUNTIME_DURABLE_DIR (durableDir) {} must exist and be accessible",
            durable.display()
        ))
    })?;
    if !meta.is_dir() || meta.uid() != expected_uid || meta.permissions().mode() & 0o7777 != 0o700 {
        return Err(refusal(format!(
            "SSH_RUNTIME_DURABLE_DIR (durableDir) {} must be an owned directory at mode 0700",
            durable.display()
        )));
    }
    Ok(())
}

/// An exclusive lock held until drop; cannot be manufactured from an unlocked file.
pub struct OwnerLock {
    root: PathBuf,
    _file: File,
}
impl OwnerLock {
    pub fn acquire(root: &Path) -> Result<Self> {
        at(private_directory(root), root)?;
        let root = at(root.canonicalize(), root)?;
        let path = root.join("owner.lock");
        let file = at(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .mode(0o600)
                .custom_flags(libc::O_NOFOLLOW)
                .open(&path),
            &path,
        )?;
        // SAFETY: a live descriptor, held for the guard's lifetime; no blocking.
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err(refusal(format!("owner.lock {} is held", path.display())));
        }
        Ok(Self { root, _file: file })
    }
    /// Verify this lock covers root, then check its state-dir binding under the lock.
    /// Some(state) permits initialization, writing the ledger first. None requires existing
    /// durable state and never accesses the engine's state directory.
    pub fn check(&self, root: &Path, state: Option<&Path>) -> Result<()> {
        if self.root != root {
            return Err(refusal(format!("owner.lock must cover {}", root.display())));
        }
        let binding = root.join("state-dir");
        let ledger = root.join("ledger.json");
        match fs::read(&binding) {
            Ok(bytes) => {
                if state.is_some_and(|s| s.as_os_str().as_bytes() != bytes) {
                    return Err(refusal(format!(
                        "state-dir {} holds another CONDUIT_STATE_DIR value",
                        binding.display()
                    )));
                }
                at(fs::metadata(&ledger), &ledger)?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let state = state.ok_or_else(|| {
                    refusal(format!(
                        "state-dir {} missing; drainer cannot initialize an instance",
                        binding.display()
                    ))
                })?;
                match fs::read(&ledger) {
                    Ok(bytes) => {
                        // A crash between the two atomic writes or a manual migration must
                        // never erase obligations. Validate and retain an existing ledger.
                        serde_json::from_slice::<Vec<runtime_provider_kit::RunRecord>>(&bytes)
                            .map_err(|_| {
                                refusal(format!("ledger.json {} is invalid", ledger.display()))
                            })?;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        at(atomic_write(root, "ledger.json", b"[]"), &ledger)?;
                    }
                    Err(_) => {
                        return Err(refusal(format!(
                            "ledger.json {} cannot be read",
                            ledger.display()
                        )));
                    }
                }
                at(
                    atomic_write(root, "state-dir", state.as_os_str().as_bytes()),
                    &binding,
                )?;
            }
            Err(_) => {
                return Err(refusal(format!(
                    "state-dir {} cannot be read",
                    binding.display()
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn foreign_owned_durable_directory_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
        let uid = fs::metadata(dir.path()).unwrap().uid();
        assert!(validate_owner(dir.path(), uid).is_ok());
        assert!(validate_owner(dir.path(), uid.wrapping_add(1)).is_err());
    }
}
