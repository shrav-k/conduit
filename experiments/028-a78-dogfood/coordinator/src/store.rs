//! The queue file and the ledger: JSON lines under `state`, each appended by one write.
//!
//! A reader skips a last line with no newline. A writer holds the file's lock, and first cuts
//! such a torn line, which no reader saw, so that the line it appends starts a line of its own.
//! Neither file stores a state: an entry's state is computed from its current attempt's lines.

use serde_json::{Map, Value, json};
use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;

/// One line of `queue.jsonl`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueEntry {
    pub name: String,
    pub branch: String,
    pub onto: Option<String>,
    pub after: Vec<String>,
    pub added: String,
}

impl QueueEntry {
    pub fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "branch": self.branch,
            "onto": self.onto,
            "after": self.after,
            "added": self.added,
        })
    }

    fn from_json(value: &Value) -> Option<Self> {
        Some(Self {
            name: value.get("name")?.as_str()?.to_string(),
            branch: value.get("branch")?.as_str()?.to_string(),
            onto: match value.get("onto")? {
                Value::Null => None,
                other => Some(other.as_str()?.to_string()),
            },
            after: value
                .get("after")?
                .as_array()?
                .iter()
                .map(|item| item.as_str().map(str::to_string))
                .collect::<Option<Vec<_>>>()?,
            added: value.get("added")?.as_str()?.to_string(),
        })
    }
}

/// The ledger's `step` of the line that ends an attempt.
pub const END: &str = "end";
/// The ledger's `step` of the line that opens the next attempt.
pub const RETRY: &str = "retry";

/// One line of `ledger.jsonl`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerLine {
    pub name: String,
    pub attempt: u64,
    pub step: String,
    pub outcome: String,
    pub started: String,
    pub ended: String,
    pub why: String,
    pub pinned: Option<String>,
    pub head: Option<String>,
    pub backup: Option<String>,
    pub gate_record: Option<String>,
    /// Only on a merged `end` line.
    pub pr: Option<u64>,
    /// Only on a merged `end` line.
    pub commit: Option<String>,
}

impl LedgerLine {
    pub fn to_json(&self) -> Value {
        let mut map = Map::new();
        map.insert("name".into(), json!(self.name));
        map.insert("attempt".into(), json!(self.attempt));
        map.insert("step".into(), json!(self.step));
        map.insert("outcome".into(), json!(self.outcome));
        map.insert("started".into(), json!(self.started));
        map.insert("ended".into(), json!(self.ended));
        map.insert("why".into(), json!(self.why));
        map.insert("pinned".into(), json!(self.pinned));
        map.insert("head".into(), json!(self.head));
        map.insert("backup".into(), json!(self.backup));
        map.insert("gate_record".into(), json!(self.gate_record));
        if self.step == END && self.outcome == "merged" {
            map.insert("pr".into(), json!(self.pr));
            map.insert("commit".into(), json!(self.commit));
        }
        Value::Object(map)
    }

    fn from_json(value: &Value) -> Option<Self> {
        let text = |key: &str| value.get(key).and_then(Value::as_str).map(str::to_string);
        let optional = |key: &str| match value.get(key) {
            None | Some(Value::Null) => Some(None),
            Some(Value::String(text)) => Some(Some(text.clone())),
            Some(_) => None,
        };
        Some(Self {
            name: text("name")?,
            attempt: value.get("attempt")?.as_u64()?,
            step: text("step")?,
            outcome: text("outcome")?,
            started: text("started")?,
            ended: text("ended")?,
            why: text("why")?,
            pinned: optional("pinned")?,
            head: optional("head")?,
            backup: optional("backup")?,
            gate_record: optional("gate_record")?,
            pr: value.get("pr").and_then(Value::as_u64),
            commit: optional("commit")?,
        })
    }

    /// Whether this line records a step, not an attempt's end or opening.
    pub fn is_step(&self) -> bool {
        self.step != END && self.step != RETRY
    }
}

/// Reads a JSON-lines file; a missing file has no lines, and a last line with no newline is
/// skipped.
pub fn read_values(path: &Path) -> Result<Vec<Value>, String> {
    let text = match fs::read(path) {
        Ok(bytes) => {
            String::from_utf8(bytes).map_err(|_| format!("{} is not UTF-8", path.display()))?
        }
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(format!("{} cannot be read: {error}", path.display())),
    };
    let mut values = Vec::new();
    let mut lines: Vec<&str> = text.split('\n').collect();
    lines.pop();
    for (number, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(line).map_err(|error| {
            format!("{} line {}: not JSON: {error}", path.display(), number + 1)
        })?;
        values.push(value);
    }
    Ok(values)
}

/// Reads `queue.jsonl`.
pub fn read_queue(path: &Path) -> Result<Vec<QueueEntry>, String> {
    read_values(path)?
        .iter()
        .enumerate()
        .map(|(number, value)| {
            QueueEntry::from_json(value).ok_or(format!(
                "{} entry {} is malformed",
                path.display(),
                number + 1
            ))
        })
        .collect()
}

/// Reads `ledger.jsonl`.
pub fn read_ledger(path: &Path) -> Result<Vec<LedgerLine>, String> {
    read_values(path)?
        .iter()
        .enumerate()
        .map(|(number, value)| {
            LedgerLine::from_json(value).ok_or(format!(
                "{} line {} is malformed",
                path.display(),
                number + 1
            ))
        })
        .collect()
}

/// Opens a JSON-lines file for appending, making it when missing.
pub fn open_append(path: &Path) -> Result<File, String> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(path)
        .map_err(|error| format!("{} cannot be opened: {error}", path.display()))
}

/// Cuts a last line with no newline, which a reader skips, so that the next line appended
/// starts a line of its own; the caller holds the file's lock.
fn cut_torn_line(file: &mut File, path: &Path) -> Result<(), String> {
    let mut bytes = Vec::new();
    file.seek(SeekFrom::Start(0))
        .and_then(|_| file.read_to_end(&mut bytes))
        .map_err(|error| format!("{} cannot be read: {error}", path.display()))?;
    if bytes.last().is_none_or(|last| *last == b'\n') {
        return Ok(());
    }
    let keep = bytes
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(0, |index| index + 1);
    file.set_len(keep as u64)
        .map_err(|error| format!("{} cannot be cut: {error}", path.display()))?;
    eprintln!(
        "wo queue: {}: cut a torn last line of {} bytes with no newline, which no reader saw",
        path.display(),
        bytes.len() - keep
    );
    Ok(())
}

/// Appends one line, in one write, after cutting a torn last line; the caller holds the file's
/// lock.
pub fn append_to(file: &mut File, path: &Path, value: &Value) -> Result<(), String> {
    cut_torn_line(file, path)?;
    let mut line = value.to_string();
    line.push('\n');
    let written = file
        .write(line.as_bytes())
        .map_err(|error| format!("{} cannot be written: {error}", path.display()))?;
    if written != line.len() {
        return Err(format!(
            "{} took {written} of the line's {} bytes",
            path.display(),
            line.len()
        ));
    }
    Ok(())
}

/// Appends one line to the file at `path`, holding its lock, as `add` and `retry` do.
pub fn append(path: &Path, value: &Value) -> Result<(), String> {
    let mut file = open_append(path)?;
    file.lock()
        .map_err(|error| format!("{} cannot be locked: {error}", path.display()))?;
    append_to(&mut file, path, value)
}

/// An entry's state, computed from its current attempt's lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Pending,
    Running,
    Stopped,
    Merged,
    Failed,
    Skipped,
}

impl State {
    pub fn word(self) -> &'static str {
        match self {
            State::Pending => "pending",
            State::Running => "running",
            State::Stopped => "stopped",
            State::Merged => "merged",
            State::Failed => "failed",
            State::Skipped => "skipped",
        }
    }
}

/// What the ledger says of one entry's current attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct View {
    pub attempt: u64,
    pub steps: Vec<LedgerLine>,
    pub end: Option<LedgerLine>,
}

impl View {
    /// The current attempt of `name`: the highest attempt any line names, or 1.
    pub fn of(name: &str, ledger: &[LedgerLine]) -> Self {
        let attempt = ledger
            .iter()
            .filter(|line| line.name == name)
            .map(|line| line.attempt)
            .max()
            .unwrap_or(1);
        let current = || {
            ledger
                .iter()
                .filter(move |line| line.name == name && line.attempt == attempt)
        };
        Self {
            attempt,
            steps: current().filter(|line| line.is_step()).cloned().collect(),
            end: current().rfind(|line| line.step == END).cloned(),
        }
    }

    /// The state, given whether a run holds the queue's lock.
    pub fn state(&self, run_holds_lock: bool) -> State {
        match &self.end {
            Some(end) => match end.outcome.as_str() {
                "merged" => State::Merged,
                "skipped" => State::Skipped,
                _ => State::Failed,
            },
            None if self.steps.is_empty() => State::Pending,
            None if run_holds_lock => State::Running,
            None => State::Stopped,
        }
    }

    pub fn last_step(&self) -> Option<&LedgerLine> {
        self.steps.last()
    }

    /// The copied gate record the attempt's lines last name.
    pub fn gate_record(&self) -> Option<String> {
        self.end
            .iter()
            .chain(self.steps.iter().rev())
            .find_map(|line| line.gate_record.clone())
    }
}
