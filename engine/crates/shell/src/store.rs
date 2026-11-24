//! A process lease and one atomic, fsynced state file. Credential bytes live elsewhere.
use crate::Result;
use crate::boundary::archive_metadata::unpack;
use conduit_core::{
    State,
    archives::Member,
    packages::{Snapshot, bytes_digest},
};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{DirBuilderExt, FileTypeExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
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

fn archive_hex(digest: &str) -> Result<&str> {
    digest
        .strip_prefix("sha256:")
        .filter(|hex| {
            hex.len() == 64
                && hex
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
        .ok_or_else(|| "invalid kept archive digest".into())
}

fn is_archive_directory(metadata: &fs::Metadata) -> bool {
    metadata.is_dir() && metadata.permissions().mode() & 0o7777 == 0o700
}

fn archive_file_kind(ty: fs::FileType) -> &'static str {
    if ty.is_dir() {
        "directory"
    } else if ty.is_symlink() {
        "symbolic link"
    } else if ty.is_fifo() {
        "FIFO"
    } else if ty.is_socket() {
        "socket"
    } else if ty.is_block_device() {
        "block device"
    } else if ty.is_char_device() {
        "character device"
    } else if ty.is_file() {
        "regular file"
    } else {
        "file of another kind"
    }
}

fn read_archive(directory: &Path, package: &Snapshot) -> Result<BTreeMap<String, Member>> {
    let archives = directory.join("packages");
    let path = archives.join(archive_hex(&package.digest)?);
    let error = |message: &str| {
        format!(
            "package {} {} archive {}: {message}",
            package.address,
            package.version,
            path.display()
        )
    };
    let metadata = |path: &Path| {
        fs::symlink_metadata(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                error("absent")
            } else {
                error(&e.to_string())
            }
        })
    };
    if !is_archive_directory(&metadata(&archives)?) {
        return Err(error("packages must be a real directory with mode 0700"));
    }
    let meta = metadata(&path)?;
    if !meta.is_file() {
        let kind = archive_file_kind(meta.file_type());
        return Err(error(&format!("found {kind}, expected a regular file")));
    }
    let mode = meta.permissions().mode() & 0o7777;
    if mode != 0o600 {
        return Err(error(&format!("found mode {mode:04o}, expected 0600")));
    }
    let bytes = fs::read(&path).map_err(|e| error(&e.to_string()))?;
    let found = bytes_digest(&bytes);
    if found != package.digest {
        return Err(error(&format!(
            "found digest {found}, expected {}",
            package.digest
        )));
    }
    unpack(&bytes).map_err(|(rule, message)| error(&format!("{rule}: {message}")))
}

fn read_disk(directory: &Path) -> Result<(Stored, BTreeSet<String>)> {
    let path = directory.join("state.json");
    if !path.try_exists().map_err(|e| e.to_string())? {
        return Ok((Stored::default(), BTreeSet::new()));
    }
    let bytes = private_read(&path)?;
    // A valid document can already be 128 levels deep before State adds its
    // own wrappers and held contexts. The loader governs authored document depth.
    let mut deserializer = serde_json::Deserializer::from_slice(&bytes);
    deserializer.disable_recursion_limit();
    let mut header = serde_json::Value::deserialize(&mut deserializer)
        .map_err(|_| "invalid store".to_string())?;
    deserializer
        .end()
        .map_err(|_| "invalid store".to_string())?;
    if header["version"] != VERSION {
        return Err("unsupported store format version".into());
    }
    // Version 1 kept inline files even when empty. Omission denotes a kept copy;
    // an unreferenced copy left by a failed commit does not migrate an old package.
    let mut external = Vec::new();
    if let Some(packages) = header
        .pointer_mut("/state/installed_packages")
        .and_then(serde_json::Value::as_array_mut)
    {
        for (index, entry) in packages.iter_mut().enumerate() {
            if let Some(fields) = entry.as_object_mut()
                && !fields.contains_key("files")
            {
                fields.insert("files".into(), serde_json::json!({}));
                external.push(index);
            }
        }
    }
    let mut stored: Stored =
        serde_json::from_value(header).map_err(|_| "invalid store state".to_string())?;
    let mut kept = BTreeSet::new();
    for index in external {
        let entry = &mut stored.state.installed_packages[index];
        entry.files = read_archive(directory, &entry.package)?;
        kept.insert(entry.package.digest.clone());
    }
    Ok((stored, kept))
}

pub(crate) fn read_stored(directory: &Path) -> Result<Stored> {
    read_disk(directory).map(|(stored, _)| stored)
}

fn create_store_directory(path: &Path) -> Result<fs::Metadata> {
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
    fs::symlink_metadata(path).map_err(|e| e.to_string())
}

pub(crate) fn private_directory(path: &Path) -> Result<()> {
    let metadata = create_store_directory(path)?;
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

#[derive(Debug)]
pub(crate) struct StateWriteError {
    pub renamed: bool,
    pub message: String,
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
    .map_err(|error| error.message)
}

pub(crate) fn append_commit_measures(directory: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(directory.join("commit-measures.jsonl"))?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.permissions().mode() & 0o777 != 0o600 {
        return Err(std::io::Error::other(
            "commit-measures.jsonl is not a regular file with mode 0600",
        ));
    }
    file.write_all(bytes)
}

fn atomic_write_inner(
    directory: &Path,
    name: &str,
    bytes: &[u8],
    #[cfg(test)] fault: Option<WriteFault>,
) -> std::result::Result<(), StateWriteError> {
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
    let mut renamed = false;
    let result: Result<()> = (|| {
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
        renamed = true;
        #[cfg(test)]
        check(WriteFault::DirectorySync)?;
        File::open(directory)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())
    })();
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result.map_err(|message| StateWriteError { renamed, message })
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
    kept_archives: RefCell<BTreeSet<String>>,
    #[cfg(test)]
    pub fault: RefCell<Option<WriteFault>>,
    #[cfg(test)]
    pub faults: RefCell<std::collections::VecDeque<WriteFault>>,
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
            kept_archives: RefCell::new(BTreeSet::new()),
            #[cfg(test)]
            fault: RefCell::new(None),
            #[cfg(test)]
            faults: RefCell::new(std::collections::VecDeque::new()),
        };
        // Reject an unknown/corrupt format before touching keys or recovering.
        store.load()?;
        Ok(store)
    }
    pub fn read(&self) -> Result<Stored> {
        let (stored, kept) = read_disk(&self.directory)?;
        self.kept_archives.replace(kept);
        Ok(stored)
    }

    pub fn keep_archive(&self, digest: &str, bytes: &[u8]) -> Result<()> {
        let archives = self.directory.join("packages");
        let hex = archive_hex(digest)?;
        if !is_archive_directory(&create_store_directory(&archives)?) {
            return Err("packages must be a real directory with mode 0700".into());
        }
        atomic_write(&archives, hex, bytes)?;
        self.kept_archives.borrow_mut().insert(digest.into());
        Ok(())
    }

    pub(crate) fn archive_descriptor(&self, digest: &str) -> Result<Value> {
        let directory = self.directory.join("packages");
        let path = directory.join(archive_hex(digest)?);
        let error = |cause: &str| format!("archive {}: {cause}", path.display());
        if !self.kept_archives.borrow().contains(digest) {
            return Err(error("no kept archive"));
        }
        let metadata = |path: &Path| {
            fs::symlink_metadata(path).map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    error("absent")
                } else {
                    error(&e.to_string())
                }
            })
        };
        let parent = metadata(&directory)?;
        if !parent.is_dir() {
            return Err(error(&format!(
                "packages is a {}",
                archive_file_kind(parent.file_type())
            )));
        }
        let file = metadata(&path)?;
        if !file.is_file() {
            return Err(error(&format!(
                "found {}, expected a regular file",
                archive_file_kind(file.file_type())
            )));
        }
        if file.len() == 0 {
            return Err(error("empty regular file"));
        }
        let url = url::Url::from_file_path(&path).map_err(|_| error("cannot form file URL"))?;
        Ok(json!({"digest":digest,"size":file.len(),"urls":[url.as_str()]}))
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
    ) -> std::result::Result<usize, StateWriteError> {
        // DL2: retain version-1 layout and inline files for legacy packages.
        self.cached.borrow_mut().take();
        #[derive(Serialize)]
        struct StoredRef<'a, S> {
            version: u32,
            state: S,
            output_times: &'a BTreeMap<String, String>,
        }
        #[derive(Serialize)]
        struct PackageRef<'a> {
            package: &'a Snapshot,
            #[serde(skip_serializing_if = "Option::is_none")]
            files: Option<&'a BTreeMap<String, Member>>,
        }
        let kept = self.kept_archives.borrow();
        let packages: Vec<_> = state
            .installed_packages
            .iter()
            .map(|entry| PackageRef {
                package: &entry.package,
                files: (!kept.contains(&entry.package.digest)).then_some(&entry.files),
            })
            .collect();
        let bytes = serde_json::to_vec(&StoredRef {
            version: VERSION,
            state: state.with_installed_packages(&packages),
            output_times,
        })
        .map_err(|e| StateWriteError {
            renamed: false,
            message: e.to_string(),
        })?;
        atomic_write_inner(
            &self.directory,
            "state.json",
            &bytes,
            #[cfg(test)]
            self.fault
                .borrow_mut()
                .take()
                .or_else(|| self.faults.borrow_mut().pop_front()),
        )?;
        Ok(bytes.len())
    }
    #[cfg(test)]
    pub fn write(&self, stored: &Stored) -> Result<()> {
        // DL2: fixture writes also invalidate memory, so their next reader reloads the file.
        self.write_state(&stored.state, &stored.output_times)
            .map(|_| ())
            .map_err(|error| error.message)
    }
}
