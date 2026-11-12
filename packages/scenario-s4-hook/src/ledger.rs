//! v1 durable, package-owned evidence. This module never reads engine state.
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, Write},
    os::unix::fs::OpenOptionsExt,
    path::Path,
    sync::{
        LazyLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

static START: LazyLock<u128> = LazyLock::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
});
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
pub fn new_id() -> String {
    format!(
        "{}-{}-{}",
        std::process::id(),
        *START,
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    )
}
/// Call identity in v1 ledger and barrier files. `attempt` is trace evidence, not an engine number.
#[derive(Clone, Serialize, Deserialize)]
pub struct Call {
    pub id: String,
    pub instance: String,
    pub invocation: String,
    pub caller: String,
    pub key: Option<String>,
    pub revision: String,
    pub attempt: u64,
    #[serde(skip)]
    pub configuration: Value,
}
impl Call {
    pub fn line(&self, package: &str, kind: &str, fields: Value) -> Value {
        let mut line = serde_json::to_value(self).unwrap();
        let map = line.as_object_mut().unwrap();
        map.extend(
            json!({"version":1,"package":package,"kind":kind})
                .as_object()
                .unwrap()
                .clone(),
        );
        map.extend(fields.as_object().unwrap().clone());
        line
    }
}
fn open(path: &Path) -> io::Result<File> {
    match OpenOptions::new()
        .read(true)
        .append(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
    {
        Ok(file) => {
            File::open(path.parent().unwrap())?.sync_all()?;
            Ok(file)
        }
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            OpenOptions::new().read(true).append(true).open(path)
        }
        Err(e) => Err(e),
    }
}
fn commit(file: &mut File, lines: &[Value]) -> io::Result<()> {
    let mut bytes = Vec::new();
    // Preserve a torn line as a separate invalid line when appending later evidence.
    let length = file.metadata()?.len();
    if length > 0 {
        file.seek(io::SeekFrom::End(-1))?;
        let mut last = [0];
        file.read_exact(&mut last)?;
        if last != *b"\n" {
            bytes.push(b'\n');
        }
    }
    for line in lines {
        serde_json::to_writer(&mut bytes, line)?;
        bytes.push(b'\n');
    }
    if file.write(&bytes)? != bytes.len() {
        return Err(io::ErrorKind::WriteZero.into());
    }
    file.sync_all()
}
/// Append one batch under an exclusive flock; a successful return is the commit boundary.
pub fn append(path: &Path, lines: &[Value]) -> io::Result<()> {
    let mut file = open(path)?;
    file.lock_exclusive()?;
    commit(&mut file, lines)
}
/// Count and commit receipt in the same lock, across processes and instances sharing a ledger.
pub fn receive(path: &Path, package: &str, call: &mut Call) -> io::Result<()> {
    let mut file = open(path)?;
    file.lock_exclusive()?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    call.attempt = text
        .lines()
        .filter_map(|s| serde_json::from_str::<Value>(s).ok())
        .filter(|v| {
            v["kind"] == "received" && v["instance"] == call.instance && v["key"] == json!(call.key)
        })
        .count() as u64
        + 1;
    commit(&mut file, &[call.line(package, "received", json!({}))])
}
/// Read only the ledger under a shared flock. Missing files are errors, never evidence of no effect.
pub fn readback(path: &Path, instance: Option<&str>) -> io::Result<Value> {
    let mut file = File::open(path)?;
    FileExt::lock_shared(&file)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let mut attempts: Vec<Value> = Vec::new();
    let mut invalid = Vec::new();
    for (index, raw) in bytes.split_inclusive(|b| *b == b'\n').enumerate() {
        let parsed = serde_json::from_slice::<Value>(raw).ok().filter(|v| {
            v["version"] == 1
                && v["package"].is_string()
                && serde_json::from_value::<Call>(v.clone()).is_ok()
                && matches!(
                    v["kind"].as_str(),
                    Some(
                        "received"
                            | "effect"
                            | "ended"
                            | "reply"
                            | "released"
                            | "dropped"
                            | "cancelled"
                            | "control-lost"
                    )
                )
        });
        let Some(v) = parsed.filter(|_| raw.ends_with(b"\n")) else {
            invalid.push(index + 1);
            continue;
        };
        if instance.is_some_and(|i| v["instance"] != i) {
            continue;
        }
        if v["kind"] == "received" {
            let mut a = v.clone();
            let m = a.as_object_mut().unwrap();
            m.remove("kind");
            m.insert("work".into(), json!("open"));
            m.insert("effect".into(), json!({"state":"not-known"}));
            m.insert("reply".into(), json!("none"));
            attempts.push(a);
        } else if let Some(a) = attempts.iter_mut().find(|a| a["id"] == v["id"]) {
            match v["kind"].as_str().unwrap() {
                "effect" => a["effect"] = json!({"state":"applied","id":v["effect"]}),
                "ended" => {
                    a["work"] = json!("ended");
                    if v["effect"].is_string() {
                        a["effect"] = json!({"state":"applied","id":v["effect"]});
                    } else if v["effect"].is_null()
                        && v.get("effect").is_some()
                        && a["effect"]["state"] != "applied"
                    {
                        a["effect"] = json!({"state":"none","cause":v["cause"]});
                    }
                }
                "reply" => a["reply"] = v["outcome"].clone(),
                _ => {}
            }
        }
    }
    Ok(json!({"version":1,"attempts":attempts,"invalidLines":invalid}))
}
