//! v1 shared server barrier protocol. No elapsed time releases or abandons a barrier.
use crate::ledger::{Call, append, new_id};
use serde_json::{Value, json};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::Path,
    time::Duration,
};
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Proceed,
    Fail,
    Drop,
}
pub fn validate_directory(path: &Path) -> bool {
    path.is_absolute()
        && fs::metadata(path).is_ok_and(|m| {
            m.is_dir() && m.uid() == unsafe { libc::geteuid() } && m.mode() & 0o022 == 0
        })
}
/// Publish new 0600 evidence atomically. Temporary names never match a handshake suffix.
pub fn publish(path: &Path, value: &Value) -> io::Result<()> {
    if path.symlink_metadata().is_ok() {
        return Err(io::ErrorKind::AlreadyExists.into());
    }
    let temporary = path.with_file_name(format!("{}.tmp", new_id()));
    let result = (|| {
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temporary)?;
        serde_json::to_writer(&mut f, value)?;
        f.write_all(b"\n")?;
        f.sync_all()?;
        fs::rename(&temporary, path)?;
        File::open(path.parent().unwrap())?.sync_all()
    })();
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}
/// Commit the release before acknowledgement. I/O or invalid verdicts fail closed as Drop.
pub async fn wait(
    directory: &Path,
    package: &str,
    call: &Call,
    ledger: &Path,
    barrier: &str,
) -> io::Result<Verdict> {
    let base = format!("{}.{}", call.id, barrier);
    let result = async {
        publish(&directory.join(format!("{base}.reached")), &json!({"version":1,"package":package,"barrier":barrier,"id":call.id,"call":call}))?;
        let release = directory.join(format!("{base}.release"));
        let verdict = loop {
            match fs::read(&release) {
                Ok(bytes) => {
                    let v: Value = serde_json::from_slice(&bytes)?;
                    break match v.as_str() {
                        Some("proceed") => Verdict::Proceed,
                        Some("drop") => Verdict::Drop,
                        Some("fail") if barrier == "before-apply" => Verdict::Fail,
                        _ => return Err(io::Error::new(io::ErrorKind::InvalidData,"release verdict")),
                    };
                }
                Err(e) if e.kind() == io::ErrorKind::NotFound && release.symlink_metadata().is_err() => tokio::time::sleep(Duration::from_millis(5)).await,
                Err(e) => return Err(e),
            }
        };
        append(ledger,&[call.line(package,"released",json!({"barrier":barrier,"outcome":match verdict { Verdict::Proceed=>"proceed", Verdict::Fail=>"fail", Verdict::Drop=>"drop" }}))])?;
        publish(&directory.join(format!("{base}.acknowledged")), &json!({"version":1}))?;
        Ok(verdict)
    }.await;
    match result {
        Ok(v) => Ok(v),
        Err(_) => {
            append(ledger,&[call.line(package,"control-lost",json!({"barrier":barrier,"outcome":"drop","diagnostic":"control release or handshake I/O failed"}))])?;
            Ok(Verdict::Drop)
        }
    }
}
