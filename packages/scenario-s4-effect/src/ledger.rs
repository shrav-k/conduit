//! The ledger is the only source of records, receipts and attempt counts.
use fs2::FileExt;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

pub struct Ledger(File);
impl Ledger {
    pub fn open(path: &Path, exclusive: bool) -> io::Result<Self> {
        let file = OpenOptions::new().read(true).append(exclusive).open(path)?;
        if exclusive {
            FileExt::lock_exclusive(&file)?;
        } else {
            FileExt::lock_shared(&file)?;
        }
        Ok(Self(file))
    }
    pub fn create(path: &Path) -> io::Result<()> {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(path)
        {
            Ok(file) => file.sync_all()?,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
        // Sync also when another process created it just before this one opened it.
        File::open(path.parent().ok_or(io::ErrorKind::InvalidInput)?)?.sync_all()
    }
    pub fn lines(&mut self) -> io::Result<(Vec<Value>, Vec<usize>)> {
        use std::io::{Seek, SeekFrom};
        self.0.seek(SeekFrom::Start(0))?;
        let mut bytes = Vec::new();
        self.0.read_to_end(&mut bytes)?;
        let mut lines = Vec::new();
        let mut invalid = Vec::new();
        for (i, bytes) in bytes.split_inclusive(|b| *b == b'\n').enumerate() {
            match serde_json::from_slice::<Value>(bytes) {
                Ok(v) if v["version"] == 1 && v["kind"].is_string() && v["call"].is_string() => {
                    lines.push(v)
                }
                _ => invalid.push(i + 1),
            }
        }
        Ok((lines, invalid))
    }
    pub fn append(&mut self, value: &Value) -> io::Result<()> {
        // Preserve a torn final line as its own invalid line before appending a new event.
        use std::io::{Seek, SeekFrom};
        let mut bytes = Vec::new();
        if self.0.metadata()?.len() > 0 {
            self.0.seek(SeekFrom::End(-1))?;
            let mut last = [0];
            self.0.read_exact(&mut last)?;
            if last[0] != b'\n' {
                bytes.push(b'\n');
            }
        }
        serde_json::to_writer(&mut bytes, value)?;
        bytes.push(b'\n');
        // One append write, never split a line among write_all retries.
        if self.0.write(&bytes)? != bytes.len() {
            return Err(io::ErrorKind::WriteZero.into());
        }
        self.0.sync_all()
    }
}
impl Drop for Ledger {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.0);
    }
}

pub fn readback(path: &Path, label: Option<&str>) -> io::Result<Value> {
    let (lines, invalid) = Ledger::open(path, false)?.lines()?;
    let mut records = BTreeMap::<String, Value>::new();
    let mut calls = BTreeMap::<String, Value>::new();
    for line in lines
        .iter()
        .filter(|v| label.is_none_or(|l| v["label"] == l))
    {
        let id = line["call"].as_str().unwrap();
        match line["kind"].as_str().unwrap() {
            "received" => {
                let mut call = line.clone();
                call.as_object_mut().unwrap().remove("kind");
                call.as_object_mut().unwrap().remove("version");
                call["parts"] = json!([]);
                call["deduplicated"] = json!(false);
                call["reply"] = json!("none");
                call["answer"] = Value::Null;
                calls.insert(id.into(), call);
            }
            "entry" => {
                if let (Some(record), Some(part @ 1..=2)) =
                    (line["record"].as_str(), line["part"].as_u64())
                {
                    let r = records.entry(record.into()).or_insert_with(|| json!({"record":record,"operation":line["operation"],"label":line["label"],"parts":[],"state":"partial"}));
                    let parts = r["parts"].as_array_mut().unwrap();
                    if !parts.contains(&json!(part)) {
                        parts.push(json!(part));
                        parts.sort_by_key(|v| v.as_u64());
                    }
                    if parts.len() == 2 {
                        r["state"] = json!("whole");
                    }
                    if let Some(call) = calls.get_mut(id) {
                        call["parts"]
                            .as_array_mut()
                            .unwrap()
                            .push(json!({"record":record,"part":part}));
                    }
                }
            }
            "deduplicated" => {
                if let Some(c) = calls.get_mut(id) {
                    c["deduplicated"] = json!(true);
                }
            }
            "reply" => {
                if let Some(c) = calls.get_mut(id) {
                    c["reply"] = line["reply"].clone();
                    c["answer"] = line["answer"].clone();
                }
            }
            _ => {}
        }
    }
    Ok(
        json!({"version":1,"label":label,"records":records.into_values().collect::<Vec<_>>(),"calls":calls.into_values().collect::<Vec<_>>(),"invalidLines":invalid}),
    )
}
