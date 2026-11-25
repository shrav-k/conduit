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
use std::ffi::{CStr, CString};
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::Write;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
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

#[derive(Clone, Copy)]
struct EntryIdentity {
    device: libc::dev_t,
    inode: libc::ino_t,
    mode: libc::mode_t,
}
impl EntryIdentity {
    fn from_stat(stat: libc::stat) -> Self {
        Self {
            device: stat.st_dev,
            inode: stat.st_ino,
            mode: stat.st_mode,
        }
    }
    fn same_inode(self, other: Self) -> bool {
        self.device == other.device && self.inode == other.inode
    }
    fn is_directory(self) -> bool {
        self.mode & libc::S_IFMT == libc::S_IFDIR
    }
    fn is_file_or_link(self) -> bool {
        matches!(self.mode & libc::S_IFMT, libc::S_IFREG | libc::S_IFLNK)
    }
}

fn changed_entry() -> std::io::Error {
    std::io::Error::other("entry changed during cleanup")
}

// Descriptors pin the opened store and its directories. Parent/name identities
// let cleanup refuse moved or replaced entries, including another run's entry.
// Every descent uses openat(O_NOFOLLOW); every deletion uses unlinkat. No cleanup
// pathname is resolved again, and no file contents are read.
pub(crate) struct CleanupDirectory {
    file: File,
    identity: EntryIdentity,
    parent: Option<(Arc<CleanupDirectory>, CString)>,
}
impl CleanupDirectory {
    fn open(path: &Path) -> std::io::Result<Arc<Self>> {
        let file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(path)?;
        let identity = Self::file_identity(&file)?;
        Ok(Arc::new(Self {
            file,
            identity,
            parent: None,
        }))
    }
    fn file_identity(file: &File) -> std::io::Result<EntryIdentity> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
        // SAFETY: fstat initializes stat on success and borrows a live descriptor.
        if unsafe { libc::fstat(file.as_raw_fd(), stat.as_mut_ptr()) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(EntryIdentity::from_stat(unsafe { stat.assume_init() }))
    }
    fn stat(&self, name: &CStr) -> std::io::Result<EntryIdentity> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
        // SAFETY: name is one NUL-terminated directory entry, never a path.
        if unsafe {
            libc::fstatat(
                self.file.as_raw_fd(),
                name.as_ptr(),
                stat.as_mut_ptr(),
                libc::AT_SYMLINK_NOFOLLOW,
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error());
        }
        Ok(EntryIdentity::from_stat(unsafe { stat.assume_init() }))
    }
    fn open_at(&self, name: &CStr, flags: libc::c_int) -> std::io::Result<File> {
        // SAFETY: name is a single component, and ownership of a successful fd
        // passes to File. O_NOFOLLOW applies even if a name changes during open.
        let fd = unsafe {
            libc::openat(
                self.file.as_raw_fd(),
                name.as_ptr(),
                flags | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600,
            )
        };
        if fd < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(unsafe { File::from_raw_fd(fd) })
    }
    fn verify(&self) -> std::io::Result<()> {
        if let Some((parent, name)) = &self.parent {
            parent.verify()?;
            if !self.identity.same_inode(parent.stat(name)?) {
                return Err(changed_entry());
            }
        }
        Ok(())
    }
    fn child(self: &Arc<Self>, name: &CStr, expected: EntryIdentity) -> std::io::Result<Arc<Self>> {
        self.verify()?;
        let file = self.open_at(name, libc::O_RDONLY | libc::O_DIRECTORY)?;
        let identity = Self::file_identity(&file)?;
        if !identity.same_inode(expected) {
            return Err(changed_entry());
        }
        Ok(Arc::new(Self {
            file,
            identity,
            parent: Some((self.clone(), name.to_owned())),
        }))
    }
    fn names(&self) -> std::io::Result<Vec<CString>> {
        self.verify()?;
        // Open a fresh description so concurrent enumerations do not share an offset.
        let file = self.open_at(c".", libc::O_RDONLY | libc::O_DIRECTORY)?;
        let fd = file.into_raw_fd();
        // SAFETY: fdopendir takes ownership only on success.
        let stream = unsafe { libc::fdopendir(fd) };
        if stream.is_null() {
            let error = std::io::Error::last_os_error();
            unsafe {
                libc::close(fd);
            }
            return Err(error);
        }
        struct DirectoryStream(*mut libc::DIR);
        impl Drop for DirectoryStream {
            fn drop(&mut self) {
                unsafe {
                    libc::closedir(self.0);
                }
            }
        }
        let stream = DirectoryStream(stream);
        let mut names = Vec::new();
        loop {
            // readdir reports both EOF and failure with null; errno distinguishes them.
            #[cfg(target_vendor = "apple")]
            let errno = unsafe { libc::__error() };
            #[cfg(not(target_vendor = "apple"))]
            let errno = unsafe { libc::__errno_location() };
            unsafe {
                *errno = 0;
            }
            let entry = unsafe { libc::readdir(stream.0) };
            if entry.is_null() {
                let error = unsafe { *errno };
                if error != 0 {
                    return Err(std::io::Error::from_raw_os_error(error));
                }
                return Ok(names);
            }
            // SAFETY: readdir's name remains valid until the next call; copy it now.
            let name = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) };
            if name != c"." && name != c".." {
                names.push(name.to_owned());
            }
        }
    }
    pub(crate) fn entry(
        self: &Arc<Self>,
        category: &str,
        name: &std::ffi::OsStr,
    ) -> std::io::Result<Option<CleanupEntry>> {
        use std::os::unix::ffi::OsStrExt;
        let name = CString::new(name.as_bytes()).map_err(|_| changed_entry())?;
        if name.to_bytes().contains(&b'/') || name == c"." || name == c".." {
            return Err(changed_entry());
        }
        let category = CString::new(category).map_err(|_| changed_entry())?;
        if category.to_bytes().contains(&b'/') || category == c"." || category == c".." {
            return Err(changed_entry());
        }
        let parent = self.child(&category, self.stat(&category)?)?;
        CleanupEntry::capture(parent, &name)
    }
}

pub(crate) struct CleanupEntry {
    parent: Arc<CleanupDirectory>,
    name: CString,
    identity: EntryIdentity,
    directory: Option<Arc<CleanupDirectory>>,
}
impl CleanupEntry {
    fn capture(parent: Arc<CleanupDirectory>, name: &CStr) -> std::io::Result<Option<Self>> {
        parent.verify()?;
        let identity = match parent.stat(name) {
            Ok(identity) => identity,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error),
        };
        let directory = if identity.is_directory() {
            Some(parent.child(name, identity)?)
        } else {
            None
        };
        Ok(Some(Self {
            parent,
            name: name.to_owned(),
            identity,
            directory,
        }))
    }
    pub(crate) fn remove(&self) -> std::io::Result<()> {
        self.remove_after_open(|| {})
    }
    #[cfg(test)]
    pub(crate) fn remove_after_check_for_test(&self, hook: impl FnOnce()) -> std::io::Result<()> {
        self.remove_after_open(hook)
    }
    // The hook also lets tests deterministically replace ancestors after capture.
    fn remove_after_open(&self, after_open: impl FnOnce()) -> std::io::Result<()> {
        let remove = || {
            self.parent.verify()?;
            if !self.identity.same_inode(self.parent.stat(&self.name)?) {
                return Err(changed_entry());
            }
            after_open();
            if let Some(directory) = &self.directory {
                directory.verify()?;
                for name in directory.names()? {
                    if let Some(entry) = Self::capture(directory.clone(), &name)? {
                        entry.remove()?;
                    }
                }
            }
            self.parent.verify()?;
            if !self.identity.same_inode(self.parent.stat(&self.name)?) {
                return Err(changed_entry());
            }
            let flags = if self.directory.is_some() {
                libc::AT_REMOVEDIR
            } else {
                0
            };
            // SAFETY: deletion is relative to the held parent, never through a path
            // or a symlink. Recheck device/inode before unlinking the original name.
            if unsafe { libc::unlinkat(self.parent.file.as_raw_fd(), self.name.as_ptr(), flags) }
                != 0
            {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        };
        match remove() {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            result => result,
        }
    }
}

// Once per open, under the lease, after accepting state and before recovery.
// No run of this store lives then: the lease admits one shell, and lifetime guards
// kill its process groups when the engine dies. Runner::new never sweeps.
// Only canonical hyphenated lower-case UUID names in the five run directories,
// plus .write-<uuid> files/links in the store itself, client-tokens and run-groups,
// are eligible. "store" below denotes the held store directory itself. Descriptor
// traversal never follows links or reads file contents.
fn sweep_run_leftovers(directory: &Arc<CleanupDirectory>) {
    let uuid_name = |name: &str| Uuid::parse_str(name).is_ok_and(|id| id.to_string() == name);
    for category in [
        "store",
        "runs",
        "work",
        "run-groups",
        "http-bridges",
        "programs",
        "client-tokens",
    ] {
        let sweep = || -> std::io::Result<()> {
            let root = if category == "store" {
                directory.clone()
            } else {
                let name = CString::new(category).unwrap();
                directory.child(&name, directory.stat(&name)?)?
            };
            for name in root.names()? {
                let Ok(filename) = name.to_str() else {
                    continue;
                };
                let run = matches!(
                    category,
                    "runs" | "work" | "run-groups" | "http-bridges" | "programs"
                ) && uuid_name(filename);
                let temporary = matches!(category, "store" | "client-tokens" | "run-groups")
                    && filename.strip_prefix(".write-").is_some_and(uuid_name);
                if !run && !temporary {
                    continue;
                }
                let result = CleanupEntry::capture(root.clone(), &name).and_then(|entry| {
                    if let Some(entry) = entry
                        && (run || entry.identity.is_file_or_link())
                    {
                        entry.remove()?;
                    }
                    Ok(())
                });
                if let Err(error) = result {
                    eprintln!("cleanup {category}: {error}");
                }
            }
            Ok(())
        };
        if let Err(error) = sweep()
            && error.kind() != std::io::ErrorKind::NotFound
        {
            eprintln!("cleanup {category}: {error}");
        }
    }
}

// DL2: faults are scoped to one Store's state-file write, never credential writes.
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum WriteFault {
    Write,
    FileSync,
    Rename,
    DirectorySync,
    PauseBeforeRename,
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
        if fault == Some(WriteFault::PauseBeforeRename) {
            fs::write(directory.join("write-ready"), b"ready").unwrap();
            loop {
                std::thread::park();
            }
        }
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
    pub(crate) cleanup: Arc<CleanupDirectory>,
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
    #[cfg(test)]
    pub(crate) fn sweep_for_test(&self) {
        sweep_run_leftovers(&self.cleanup);
    }

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
        let cleanup = CleanupDirectory::open(&directory).map_err(|e| e.to_string())?;
        let lease = cleanup
            .open_at(c"writer.lock", libc::O_RDWR | libc::O_CREAT)
            .map_err(|e| e.to_string())?;
        let metadata = lease.metadata().map_err(|e| e.to_string())?;
        if !metadata.is_file() || metadata.permissions().mode() & 0o777 != 0o600 {
            return Err("store files must be regular files with mode 0600".into());
        }
        lease
            .try_lock_exclusive()
            .map_err(|_| "store already has an active shell".to_string())?;
        lease.sync_all().map_err(|e| e.to_string())?;
        cleanup.file.sync_all().map_err(|e| e.to_string())?;
        let store = Self {
            directory,
            cleanup,
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
        sweep_run_leftovers(&store.cleanup);
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
