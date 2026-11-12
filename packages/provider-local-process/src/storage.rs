use async_trait::async_trait;
use runtime_provider_kit::{Result, RunRecord, StateStore, Unavailable};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    sync::Mutex,
};

pub fn instance_key(id: &str) -> String {
    format!("{:x}", Sha256::digest(id.as_bytes()))
}

pub fn private_directory(path: &Path) -> std::io::Result<()> {
    match fs::DirBuilder::new().mode(0o700).create(path) {
        Ok(()) => (),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
        Err(e) => return Err(e),
    }
    let m = fs::symlink_metadata(path)?;
    if !m.is_dir() || m.permissions().mode() & 0o777 != 0o700 {
        return Err(std::io::Error::other("state directory must be private"));
    }
    Ok(())
}

pub(crate) fn private_file(path: &Path) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
}

/// One writer per provider state directory; the backend holds its exclusive owner lock.
pub struct FileStore {
    root: PathBuf,
    lock: Mutex<()>,
}
impl FileStore {
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.into(),
            lock: Mutex::new(()),
        }
    }
    fn read(&self) -> Result<Vec<RunRecord>> {
        match fs::read(self.root.join("ledger.json")) {
            Ok(bytes) => serde_json::from_slice(&bytes).map_err(|_| Unavailable),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(_) => Err(Unavailable),
        }
    }
}
pub(crate) fn atomic_write(root: &Path, name: &str, bytes: &[u8]) -> std::io::Result<()> {
    let temp = root.join(format!(".{name}.new"));
    if temp.exists() {
        fs::remove_file(&temp)?;
    }
    let mut file = private_file(&temp)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(&temp, root.join(name))?;
    File::open(root)?.sync_all()
}
#[async_trait]
impl StateStore for FileStore {
    async fn load(&self) -> Result<Vec<RunRecord>> {
        let _lock = self.lock.lock().map_err(|_| Unavailable)?;
        self.read()
    }
    async fn save(&self, record: &RunRecord) -> Result<()> {
        let _lock = self.lock.lock().map_err(|_| Unavailable)?;
        let mut records = self.read()?;
        records.retain(|r| !r.same_run(record));
        records.push(record.clone());
        records.sort_by(|a, b| {
            (&a.instance, &a.context.machine.hostname)
                .cmp(&(&b.instance, &b.context.machine.hostname))
        });
        let bytes = serde_json::to_vec(&records).map_err(|_| Unavailable)?;
        atomic_write(&self.root, "ledger.json", &bytes).map_err(|_| Unavailable)
    }
}
