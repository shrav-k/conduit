use crate::guard::Marker;
use async_trait::async_trait;
use runtime_provider_kit::{Result, RunRecord, RunState, StateStore, Unavailable};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    os::unix::{
        ffi::OsStrExt,
        fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt},
    },
    path::{Path, PathBuf},
    sync::Mutex,
};

pub const DURABLE_DIR_VARIABLE: &str = "LOCAL_PROCESS_DURABLE_DIR";
const DURABLE_SETTING: &str = "durableDir (LOCAL_PROCESS_DURABLE_DIR)";

pub fn instance_key(id: &str) -> String {
    format!("{:x}", Sha256::digest(id.as_bytes()))
}

fn validate_durable_owner(
    path: &Path,
    expected_uid: u32,
    label: &str,
) -> std::result::Result<(), String> {
    let meta = match path.symlink_metadata() {
        Ok(meta) => meta,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Err(format!("{label} does not exist"));
        }
        Err(error) if error.kind() == io::ErrorKind::NotADirectory => {
            return Err(format!("{label} is not a directory"));
        }
        Err(_) => return Err(format!("cannot inspect {label}")),
    };
    if !meta.is_dir() {
        return Err(format!("{label} is not a directory"));
    }
    if meta.uid() != expected_uid {
        return Err(format!(
            "{label} is owned by another uid ({} instead of {expected_uid})",
            meta.uid()
        ));
    }
    let mode = meta.mode() & 0o7777;
    if mode != 0o700 {
        return Err(format!("{label} has mode 0{mode:03o} instead of 0700"));
    }
    Ok(())
}

pub(crate) fn prepare_layout(
    state: &Path,
    durable: &Path,
) -> std::result::Result<(PathBuf, PathBuf), String> {
    if !durable.is_absolute() {
        return Err(format!("{DURABLE_SETTING} must be an absolute directory"));
    }
    // SAFETY: geteuid takes no arguments.
    let owner = unsafe { libc::geteuid() };
    // Trailing slashes must not cause the metadata lookup to follow a final symlink.
    let durable: PathBuf = durable.components().collect();
    validate_durable_owner(&durable, owner, DURABLE_SETTING)?;
    let durable = durable
        .canonicalize()
        .map_err(|_| format!("cannot resolve {DURABLE_SETTING}"))?;
    let state_dir = canonical_state_directory(state)
        .map_err(|_| "cannot resolve private CONDUIT_STATE_DIR or its parent")?;
    let parent = state_dir
        .parent()
        .ok_or("CONDUIT_STATE_DIR needs a parent")?;
    let instance = durable.join(format!(
        "{:x}",
        Sha256::digest(state.as_os_str().as_bytes())
    ));
    if durable.starts_with(parent) || instance.starts_with(parent) {
        return Err(format!(
            "{DURABLE_SETTING} must be outside CONDUIT_STATE_DIR and its parent"
        ));
    }
    let mut old = Vec::new();
    for (name, label) in [("ledger.json", "ledger.json"), ("markers", "markers/")] {
        match fs::symlink_metadata(state_dir.join(name)) {
            Ok(_) => old.push(label),
            Err(e) if e.kind() == io::ErrorKind::NotFound => (),
            Err(_) => {
                return Err(format!(
                    "{DURABLE_SETTING}: cannot check CONDUIT_STATE_DIR for {label}"
                ));
            }
        }
    }
    if !old.is_empty() {
        return Err(format!(
            "{DURABLE_SETTING}: CONDUIT_STATE_DIR has old layout entries {}; move them before startup",
            old.join(", ")
        ));
    }
    private_directory(state).map_err(|_| "cannot open private CONDUIT_STATE_DIR")?;
    let label = format!("instance subdirectory in {DURABLE_SETTING}");
    create_private_directory(&instance).map_err(|_| format!("cannot create {label}"))?;
    validate_durable_owner(&instance, owner, &label)?;
    Ok((state_dir, instance))
}

fn canonical_state_directory(state: &Path) -> io::Result<PathBuf> {
    match state.canonicalize() {
        Ok(path) => Ok(path),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            // The engine creates its directory first. Also allow a direct start
            // under an existing parent, as before, after checking placement.
            let name = state.file_name().ok_or(error)?;
            let parent = state
                .parent()
                .filter(|path| !path.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            Ok(parent.canonicalize()?.join(name))
        }
        Err(error) => Err(error),
    }
}

fn read_entry(path: &Path) -> io::Result<Option<Vec<u8>>> {
    let mut file = match OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
    {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e),
    };
    if !file.metadata()?.is_file() {
        return Err(io::Error::other(
            "durable state entry must be a regular file",
        ));
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(Some(bytes))
}

fn is_missing(path: &Path) -> bool {
    path.symlink_metadata()
        .is_err_and(|error| error.kind() == io::ErrorKind::NotFound)
}

fn never_launched(state_dir: &Path, key: &str) -> bool {
    // Launch requires staging both directories. Confirmed stop removes runs/<key>,
    // but instances/<key> survives for the lifetime of CONDUIT_STATE_DIR.
    ["runs", "instances"].into_iter().all(|name| {
        let area = state_dir.join(name);
        area.symlink_metadata()
            .is_ok_and(|meta| meta.is_dir() && meta.mode() & 0o7777 == 0o700)
            && is_missing(&area.join(key))
    })
}

fn check_owed_stop_markers(
    root: &Path,
    state_dir: &Path,
    records: &[RunRecord],
) -> std::result::Result<(), String> {
    let markers = root.join("markers");
    let have_markers = match markers.symlink_metadata() {
        Ok(meta) if !meta.is_dir() => {
            return Err(format!("{DURABLE_SETTING}: markers/ is not a directory"));
        }
        Ok(meta) => {
            let mode = meta.mode() & 0o7777;
            if mode != 0o700 {
                return Err(format!(
                    "{DURABLE_SETTING}: markers/ has mode 0{mode:03o} instead of 0700"
                ));
            }
            true
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => false,
        Err(_) => return Err(format!("{DURABLE_SETTING}: cannot inspect markers/")),
    };
    for record in records.iter().filter(|record| record.owed_stop) {
        let key = instance_key(&record.instance);
        let problem = match read_entry(&markers.join(&key)) {
            Ok(Some(bytes)) => {
                let matching = serde_json::from_slice::<Marker>(&bytes)
                    .is_ok_and(|marker| record.same_run(&marker.record));
                if matching {
                    continue;
                }
                "is mismatched or invalid"
            }
            Ok(None) => {
                if have_markers
                    && record.state == RunState::Unknown
                    && never_launched(state_dir, &key)
                {
                    // The kit can finish the interrupted stop and persist Stopped.
                    continue;
                }
                "is missing"
            }
            Err(_) => "cannot be read",
        };
        return Err(format!(
            "PRV-021: {DURABLE_SETTING}: owed stop for instance {:?} cannot be recovered because markers/{key} {problem}",
            record.instance
        ));
    }
    Ok(())
}

/// The backend calls this with the instance's exclusive owner lock held.
pub(crate) fn bind_state(
    root: &Path,
    state: &Path,
    state_dir: &Path,
) -> std::result::Result<(), String> {
    let raw_state = state.as_os_str().as_bytes();
    let binding = read_entry(&root.join("state-dir"))
        .map_err(|_| format!("{DURABLE_SETTING}: state-dir cannot be read"))?;
    if binding.as_deref().is_some_and(|saved| saved != raw_state) {
        return Err(format!(
            "{DURABLE_SETTING}: state-dir holds another CONDUIT_STATE_DIR value"
        ));
    }
    match read_entry(&root.join("ledger.json"))
        .map_err(|_| format!("{DURABLE_SETTING}: ledger.json cannot be read"))?
    {
        Some(bytes) => {
            // A crash between the two writes or a manual migration can leave a
            // valid ledger alone. Preserve it rather than erasing its obligations.
            let records = serde_json::from_slice::<Vec<RunRecord>>(&bytes)
                .map_err(|_| format!("{DURABLE_SETTING}: ledger.json is invalid"))?;
            check_owed_stop_markers(root, state_dir, &records)?;
        }
        None => {
            if binding.is_some() || !is_missing(&root.join("markers")) {
                return Err(format!("{DURABLE_SETTING}: ledger.json is missing"));
            }
            let runs = state_dir.join("runs");
            match fs::read_dir(&runs).and_then(|mut entries| entries.next().transpose()) {
                Ok(Some(_)) => {
                    return Err(format!(
                        "{DURABLE_SETTING}: cannot bind state-dir because CONDUIT_STATE_DIR/runs/ is non-empty. \
                         durableDir may have changed and owed stops may be in the old directory"
                    ));
                }
                Ok(None) => (),
                Err(error) if error.kind() == io::ErrorKind::NotFound && is_missing(&runs) => (),
                Err(_) => {
                    return Err(format!(
                        "{DURABLE_SETTING}: cannot read CONDUIT_STATE_DIR/runs/"
                    ));
                }
            }
            atomic_write(root, "ledger.json", b"[]")
                .map_err(|_| format!("{DURABLE_SETTING}: cannot initialize ledger.json"))?;
        }
    }
    if binding.is_none() {
        atomic_write(root, "state-dir", raw_state)
            .map_err(|_| format!("{DURABLE_SETTING}: cannot write state-dir"))?;
    }
    Ok(())
}

fn create_private_directory(path: &Path) -> io::Result<()> {
    match fs::DirBuilder::new().mode(0o700).create(path) {
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(()),
        result => result,
    }
}

pub fn private_directory(path: &Path) -> std::io::Result<()> {
    create_private_directory(path)?;
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

/// One writer per durable instance directory; the backend holds its exclusive owner lock.
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
        let bytes = read_entry(&self.root.join("ledger.json"))
            .map_err(|_| Unavailable)?
            .ok_or(Unavailable)?;
        serde_json::from_slice(&bytes).map_err(|_| Unavailable)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foreign_owned_durable_directory_is_refused() {
        let root = tempfile::tempdir().unwrap();
        let directory = root.path().join("durable");
        private_directory(&directory).unwrap();
        // SAFETY: geteuid takes no arguments.
        let owner = unsafe { libc::geteuid() };
        for label in [
            DURABLE_SETTING,
            &format!("instance subdirectory in {DURABLE_SETTING}"),
        ] {
            assert!(validate_durable_owner(&directory, owner, label).is_ok());
            assert!(
                validate_durable_owner(&directory, owner.wrapping_add(1), label).is_err_and(
                    |error| error.starts_with(&format!("{label} is owned by another uid"))
                )
            );
        }
    }
}
