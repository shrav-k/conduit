//! A process lease and one atomic, fsynced state file. Credential bytes live elsewhere.
use crate::Result;
use conduit_core::State;
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::{cell::RefCell, collections::BTreeMap, sync::Arc};
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

// DL2: faults are scoped to one Store's state-file write, never credential writes.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum WriteFault {
    Write,
    FileSync,
    Rename,
    DirectorySync,
}

pub(crate) fn atomic_write(directory: &Path, name: &str, bytes: &[u8]) -> Result<()> {
    // DL2/DL13: credential and endpoint writes never receive a test-store fault.
    atomic_write_inner(
        directory,
        name,
        bytes,
        #[cfg(test)]
        None,
    )
}
fn atomic_write_inner(
    directory: &Path,
    name: &str,
    bytes: &[u8],
    #[cfg(test)] fault: Option<WriteFault>,
) -> Result<()> {
    // DL2: all four persistence stages precede publication of the new memory snapshot.
    #[cfg(test)]
    let check = |stage| {
        if fault == Some(stage) {
            Err(format!("injected state write failure: {stage:?}"))
        } else {
            Ok(())
        }
    };
    let temporary = directory.join(format!(".write-{}", Uuid::new_v4()));
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temporary)
            .map_err(|e| e.to_string())?;
        #[cfg(test)]
        check(WriteFault::Write)?;
        file.write_all(bytes).map_err(|e| e.to_string())?;
        #[cfg(test)]
        check(WriteFault::FileSync)?;
        file.sync_all().map_err(|e| e.to_string())?;
        #[cfg(test)]
        check(WriteFault::Rename)?;
        fs::rename(&temporary, directory.join(name)).map_err(|e| e.to_string())?;
        #[cfg(test)]
        check(WriteFault::DirectorySync)?;
        File::open(directory)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())
    })();
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}

pub(crate) struct Memory {
    pub state: Arc<State>,
    pub output_times: BTreeMap<String, String>,
}
pub(crate) struct Store {
    pub directory: PathBuf,
    _lease: File,
    // DL2: accessed only under Shell's writer mutex while this Store holds its lease.
    cached: RefCell<Option<Memory>>,
    #[cfg(test)]
    pub fault: RefCell<Option<WriteFault>>,
}
impl Store {
    pub fn clean_state_directories(&self, state: &State) -> Result<()> {
        let root = self.directory.join("instances");
        private_directory(&root)?;
        for entry in fs::read_dir(&root).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let name = entry.file_name();
            let Some(id) = name.to_str() else { continue };
            if Uuid::parse_str(id).is_ok()
                && !state
                    .objects
                    .iter()
                    .any(|o| o["id"] == id && o.get("spec").is_none())
            {
                // Never inspect contents. remove_dir_all does not follow symlinks.
                // Failed deletes are retried on every commit and store reopen.
                let _ = fs::remove_dir_all(entry.path());
            }
        }
        Ok(())
    }

    pub fn open(directory: &Path) -> Result<Self> {
        // DL2: parse once at open, before keys/recovery, and hand that state to readers.
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
            cached: RefCell::new(None),
            #[cfg(test)]
            fault: RefCell::new(None),
        };
        // Reject an unknown/corrupt format before touching keys or recovering.
        store.load()?;
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
    fn load(&self) -> Result<()> {
        // DL2: a missing cache means open or a failed/consumed commit; disk is authoritative.
        if self.cached.borrow().is_none() {
            let stored = self.read()?;
            self.remember(Memory {
                state: Arc::new(stored.state),
                output_times: stored.output_times,
            });
        }
        Ok(())
    }
    pub fn snapshot(&self) -> Result<Arc<State>> {
        // DL2/DL13: readers share exactly the last committed State.
        self.load()?;
        Ok(Arc::clone(&self.cached.borrow().as_ref().unwrap().state))
    }
    pub fn take(&self) -> Result<Memory> {
        // DL2/DL6: leave no cached State behind if a step or write returns an error.
        self.load()?;
        Ok(self.cached.borrow_mut().take().unwrap())
    }
    pub fn remember(&self, memory: Memory) {
        // DL2: only open/reload or a fully durable commit may install a snapshot.
        *self.cached.borrow_mut() = Some(memory);
    }
    pub fn write_state(
        &self,
        state: &State,
        output_times: &BTreeMap<String, String>,
    ) -> Result<()> {
        // DL2: identical version-1 field order and bytes, serialized from references.
        self.cached.borrow_mut().take();
        #[derive(Serialize)]
        struct StoredRef<'a> {
            version: u32,
            state: &'a State,
            output_times: &'a BTreeMap<String, String>,
        }
        let bytes = serde_json::to_vec(&StoredRef {
            version: VERSION,
            state,
            output_times,
        })
        .map_err(|e| e.to_string())?;
        atomic_write_inner(
            &self.directory,
            "state.json",
            &bytes,
            #[cfg(test)]
            self.fault.borrow_mut().take(),
        )
    }
    #[cfg(test)]
    pub fn write(&self, stored: &Stored) -> Result<()> {
        // DL2: fixture writes also invalidate memory, so their next reader reloads the file.
        self.write_state(&stored.state, &stored.output_times)
    }
}
