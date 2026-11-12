use serde_json::Value;
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::Path,
    time::Duration,
};

pub fn validate(path: &Path) -> bool {
    // SAFETY: geteuid takes no arguments and cannot invalidate memory.
    fs::metadata(path)
        .is_ok_and(|m| m.is_dir() && m.uid() == unsafe { libc::geteuid() } && m.mode() & 0o022 == 0)
}
pub fn atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let temporary = path.with_extension(format!(
        "{}-temporary",
        path.extension().unwrap().to_string_lossy()
    ));
    let result = (|| {
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temporary)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        fs::rename(&temporary, path)?;
        fs::File::open(path.parent().unwrap())?.sync_all()
    })();
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}
pub fn file(base: &Path, suffix: &str) -> std::path::PathBuf {
    base.with_file_name(format!(
        "{}.{}",
        base.file_name().unwrap().to_string_lossy(),
        suffix
    ))
}
pub async fn wait(base: &Path, details: &Value) -> io::Result<bool> {
    atomic(&file(base, "reached"), &serde_json::to_vec(details)?)?;
    loop {
        if !validate(base.parent().unwrap()) {
            return Err(io::ErrorKind::PermissionDenied.into());
        }
        match fs::read(file(base, "release")) {
            Ok(bytes) => {
                return match bytes.as_slice() {
                    b"proceed" | b"proceed\n" => Ok(true),
                    b"drop" | b"drop\n" => Ok(false),
                    _ => Err(io::ErrorKind::InvalidData.into()),
                };
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
}
