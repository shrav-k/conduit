//! A process lease and one atomic, fsynced state file. Credential bytes live elsewhere.
use crate::Result;
use conduit_core::State;
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub(crate) const VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Stored {
    pub version: u32,
    pub state: State,
    // Kept apart from records so purge cannot change the output retention age.
    pub output_times: BTreeMap<String, String>,
}
impl Default for Stored {
    fn default() -> Self {
        Self {
            version: VERSION,
            state: State::default(),
            output_times: BTreeMap::new(),
        }
    }
}

pub(crate) fn private_directory(path: &Path) -> Result<()> {
    match DirBuilder::new().mode(0o700).create(path) {
        Ok(()) => {
            if let Some(parent) = path.parent() {
                File::open(parent)
                    .and_then(|f| f.sync_all())
                    .map_err(|e| e.to_string())?;
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(e) => return Err(e.to_string()),
    }
    let metadata = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !metadata.is_dir() || metadata.permissions().mode() & 0o777 != 0o700 {
        return Err("store directories must be real directories with mode 0700".into());
    }
    Ok(())
}

pub(crate) fn private_read(path: &Path) -> Result<Vec<u8>> {
    let metadata = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.permissions().mode() & 0o777 != 0o600 {
        return Err("store files must be regular files with mode 0600".into());
    }
    fs::read(path).map_err(|e| e.to_string())
}

pub(crate) fn atomic_write(directory: &Path, name: &str, bytes: &[u8]) -> Result<()> {
    let temporary = directory.join(format!(".write-{}", Uuid::new_v4()));
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temporary)
            .map_err(|e| e.to_string())?;
        file.write_all(bytes)
            .and_then(|()| file.sync_all())
            .map_err(|e| e.to_string())?;
        fs::rename(&temporary, directory.join(name)).map_err(|e| e.to_string())?;
        File::open(directory)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())
    })();
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}

pub(crate) struct Store {
    pub directory: PathBuf,
    _lease: File,
}
impl Store {
    pub fn open(directory: &Path) -> Result<Self> {
        let directory = std::path::absolute(directory).map_err(|e| e.to_string())?;
        private_directory(&directory)?;
        let lease_path = directory.join("writer.lock");
        match fs::symlink_metadata(&lease_path) {
            Ok(_) => {
                private_read(&lease_path)?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.to_string()),
        }
        let lease = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .open(&lease_path)
            .map_err(|e| e.to_string())?;
        lease
            .try_lock_exclusive()
            .map_err(|_| "store already has an active shell".to_string())?;
        lease.sync_all().map_err(|e| e.to_string())?;
        File::open(&directory)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
        let store = Self {
            directory,
            _lease: lease,
        };
        // Reject an unknown/corrupt format before touching keys or recovering.
        store.read()?;
        Ok(store)
    }
    pub fn read(&self) -> Result<Stored> {
        let path = self.directory.join("state.json");
        if !path.try_exists().map_err(|e| e.to_string())? {
            return Ok(Stored::default());
        }
        let bytes = private_read(&path)?;
        // A valid document can already be 128 levels deep before State adds its
        // own wrappers and held contexts. The loader, not serde's default JSON
        // recursion limit, governs authored document depth.
        let mut deserializer = serde_json::Deserializer::from_slice(&bytes);
        deserializer.disable_recursion_limit();
        let header = serde_json::Value::deserialize(&mut deserializer)
            .map_err(|_| "invalid store".to_string())?;
        deserializer
            .end()
            .map_err(|_| "invalid store".to_string())?;
        if header["version"] != VERSION {
            return Err("unsupported store format version".into());
        }
        serde_json::from_value(header).map_err(|_| "invalid store state".into())
    }
    pub fn write(&self, stored: &Stored) -> Result<()> {
        let bytes = serde_json::to_vec(stored).map_err(|e| e.to_string())?;
        atomic_write(&self.directory, "state.json", &bytes)
    }
}
