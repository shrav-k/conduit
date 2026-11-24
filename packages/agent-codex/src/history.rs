use crate::{Config, Mode, home_label, resolve_home, usage};
use anyhow::{Context, Result, anyhow, ensure};
use serde_json::{Map, Value, json};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, BufWriter, Write},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

const FILE: &str = "history.jsonl";
const TEMPORARY: &str = ".history.jsonl.tmp";
const START_REFUSAL: &str = "history (history.jsonl): cannot keep this turn in CONDUIT_STATE_DIR; no client started, the conversation is unchanged; call again once it is writable";
const READ_REFUSAL: &str = "history (history.jsonl): cannot read the history in CONDUIT_STATE_DIR";
const INVALID_START: &str = "history (history.jsonl): the history in CONDUIT_STATE_DIR is invalid; no client started, the conversation is unchanged; retire the Agent to remove it";
const INVALID_READ: &str = "history (history.jsonl): the history in CONDUIT_STATE_DIR is invalid; retire the Agent to remove it";
const REMOVE_REFUSAL: &str = "client stopped; history (history.jsonl): cannot remove the history in CONDUIT_STATE_DIR; the home was not emptied";

#[derive(Default)]
struct State {
    active: Option<u64>,
    retired: bool,
}

pub struct History {
    directory: std::result::Result<PathBuf, String>,
    state: Mutex<State>,
}

impl History {
    pub(crate) fn new(config: &Config) -> Self {
        let directory = config.state_dir.clone().ok_or_else(|| {
            config.configuration_error.clone().unwrap_or_else(|| {
                format!("{}: CONDUIT_STATE_DIR is unset", home_label(config.mode))
            })
        });
        Self {
            directory,
            state: Mutex::default(),
        }
    }

    fn directory(&self) -> Result<&Path> {
        self.directory
            .as_deref()
            .map_err(|message| anyhow!("{message}"))
    }

    pub(crate) fn start(
        &self,
        message: &str,
        invocation: Option<&str>,
        caller: Option<&str>,
    ) -> Result<Turn<'_>> {
        let mut state = self.state.lock().unwrap();
        let path = self
            .directory()
            .map_err(|_| anyhow!(START_REFUSAL))?
            .join(FILE);
        let mut turns = read(&path).map_err(|error| {
            anyhow!(match error {
                ReadError::Access => START_REFUSAL,
                ReadError::InvalidEntry | ReadError::InvalidTimestamp => INVALID_START,
            })
        })?;
        let index = (|| -> Result<u64> {
            let index = latest(&turns)
                .checked_add(1)
                .context("history index exhausted")?;
            turns.push(json!({
                "index":index, "invocation":invocation, "caller":caller, "message":message,
                "text":null, "state":"running", "startedAt":now()?, "endedAt":null
            }));
            write(&path, &turns)?;
            Ok(index)
        })()
        .map_err(|_| anyhow!(START_REFUSAL))?;
        state.active = Some(index);
        Ok(Turn {
            history: self,
            index,
        })
    }

    pub fn page(&self, since: u64, count: usize) -> Result<Value> {
        let state = self.state.lock().unwrap();
        ensure!(
            !state.retired,
            "history: session retired; its history is no longer served"
        );
        let directory = self.directory()?;
        resolve_home(Some(directory), Mode::SharedLoginAgent, false)?;
        let turns = read(&directory.join(FILE)).map_err(|error| {
            anyhow!(match error {
                ReadError::InvalidEntry => INVALID_READ,
                ReadError::Access | ReadError::InvalidTimestamp => READ_REFUSAL,
            })
        })?;
        let latest = latest(&turns);
        let page: Vec<_> = turns
            .into_iter()
            .filter(|turn| turn["index"].as_u64().unwrap() > since)
            .take(count)
            .map(|mut turn| {
                if turn["state"] == "running" && turn["index"].as_u64() != state.active {
                    turn["state"] = json!("interrupted");
                }
                turn
            })
            .collect();
        Ok(json!({"turns":page,"latest":latest}))
    }

    pub(crate) fn retire(&self) {
        self.state.lock().unwrap().retired = true;
    }

    pub(crate) fn remove(&self) -> Result<()> {
        let directory = self.directory()?;
        remove_if_present(&directory.join(TEMPORARY))
            .and_then(|()| remove_if_present(&directory.join(FILE)))
            .map_err(|_| anyhow!(REMOVE_REFUSAL))
    }
}

pub(crate) struct Turn<'a> {
    history: &'a History,
    index: u64,
}

impl Turn<'_> {
    pub(crate) fn finish(self, text: Option<&str>) {
        let mut state = self.history.state.lock().unwrap();
        // A failed replacement leaves the start record intact. It reads as interrupted.
        let _ = (|| -> Result<()> {
            let path = self.history.directory()?.join(FILE);
            let Ok(mut turns) = read(&path) else {
                return Ok(());
            };
            let entry = turns.last_mut().context("missing turn")?;
            ensure!(entry["index"] == self.index && entry["state"] == "running");
            entry["text"] = json!(text);
            entry["state"] = json!(if text.is_some() {
                "completed"
            } else {
                "failed"
            });
            entry["endedAt"] = json!(now()?);
            write(&path, &turns)
        })();
        state.active = None;
    }
}

impl Drop for Turn<'_> {
    fn drop(&mut self) {
        self.history.state.lock().unwrap().active = None;
    }
}

pub fn paging(arguments: &Map<String, Value>) -> std::result::Result<(u64, usize), &'static str> {
    if arguments
        .keys()
        .any(|name| name != "since" && name != "count")
    {
        return Err("history: expected only since and count");
    }
    let since = match arguments.get("since") {
        None => 0,
        Some(value) => whole_number(value).ok_or(
            "history: since must be a whole number from 0, the index of the last turn already read",
        )?,
    };
    let count = match arguments.get("count") {
        None => 20,
        Some(value) => whole_number(value)
            .filter(|count| (1..=50).contains(count))
            .ok_or("history: count must be a whole number from 1 to 50")?
            as usize,
    };
    Ok((since, count))
}

fn whole_number(value: &Value) -> Option<u64> {
    value.as_u64().or_else(|| {
        value
            .as_f64()
            .filter(|n| *n >= 0.0 && n.fract() == 0.0)
            .map(|n| n as u64)
    })
}

fn now() -> Result<String> {
    let seconds = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    usage::utc(seconds.try_into()?).context("history time out of range")
}

fn latest(turns: &[Value]) -> u64 {
    turns
        .last()
        .map_or(0, |turn| turn["index"].as_u64().unwrap())
}

enum ReadError {
    Access,
    InvalidEntry,
    InvalidTimestamp,
}

fn read(path: &Path) -> std::result::Result<Vec<Value>, ReadError> {
    let file = match OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
    {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(_) => return Err(ReadError::Access),
    };
    if !file.metadata().map_err(|_| ReadError::Access)?.is_file() {
        return Err(ReadError::Access);
    }
    let mut turns = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|error| match error.kind() {
            io::ErrorKind::InvalidData => ReadError::InvalidEntry,
            _ => ReadError::Access,
        })?;
        let entry: Value = serde_json::from_str(&line).map_err(|_| ReadError::InvalidEntry)?;
        if !valid_entry(&entry, latest(&turns)) {
            return Err(ReadError::InvalidEntry);
        }
        if !entry["startedAt"].as_str().is_some_and(utc_timestamp)
            || entry["endedAt"]
                .as_str()
                .is_some_and(|time| !utc_timestamp(time))
        {
            return Err(ReadError::InvalidTimestamp);
        }
        turns.push(entry);
    }
    Ok(turns)
}

fn valid_entry(entry: &Value, previous: u64) -> bool {
    let Some(fields) = entry.as_object() else {
        return false;
    };
    // Reject additions to the file that do not belong to the public history shape.
    if fields.len() != 8
        || !entry["index"]
            .as_u64()
            .is_some_and(|index| index > previous)
        || !entry["message"].is_string()
        || !entry["startedAt"].is_string()
        || !["invocation", "caller", "text", "endedAt"]
            .into_iter()
            .all(|key| {
                fields
                    .get(key)
                    .is_some_and(|value| value.is_null() || value.is_string())
            })
    {
        return false;
    }
    match entry["state"].as_str() {
        Some("running" | "interrupted") => entry["text"].is_null() && entry["endedAt"].is_null(),
        Some("completed") => entry["text"].is_string() && entry["endedAt"].is_string(),
        Some("failed") => entry["text"].is_null() && entry["endedAt"].is_string(),
        _ => false,
    }
}

fn utc_timestamp(timestamp: &str) -> bool {
    let bytes = timestamp.as_bytes();
    let [
        _,
        _,
        _,
        _,
        b'-',
        _,
        _,
        b'-',
        _,
        _,
        b'T' | b't',
        _,
        _,
        b':',
        _,
        _,
        b':',
        _,
        _,
        suffix @ ..,
    ] = bytes
    else {
        return false;
    };
    let number = |digits: &[u8]| {
        digits.iter().try_fold(0_u16, |value, &digit| {
            digit
                .is_ascii_digit()
                .then(|| value * 10 + u16::from(digit - b'0'))
        })
    };
    let [
        Some(year),
        Some(month),
        Some(day),
        Some(hour),
        Some(minute),
        Some(second),
    ] = [
        number(&bytes[..4]),
        number(&bytes[5..7]),
        number(&bytes[8..10]),
        number(&bytes[11..13]),
        number(&bytes[14..16]),
        number(&bytes[17..19]),
    ]
    else {
        return false;
    };
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) => {
            29
        }
        2 => 28,
        _ => return false,
    };
    let fraction = match suffix {
        [fraction @ .., b'Z' | b'z']
        | [fraction @ .., b'+' | b'-', b'0', b'0', b':', b'0', b'0'] => fraction,
        _ => return false,
    };
    (1..=days).contains(&day)
        && hour < 24
        && minute < 60
        && (second < 60 || second == 60 && hour == 23 && minute == 59 && day == days)
        && (fraction.is_empty()
            || matches!(fraction, [b'.', digits @ ..] if !digits.is_empty() && digits.iter().all(u8::is_ascii_digit)))
}

fn remove_if_present(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        result => result,
    }
}

fn write(path: &Path, turns: &[Value]) -> Result<()> {
    // There is one run per state directory. The lock serializes this run's writes,
    // and a replacement discards a temporary file left by a stopped run.
    let temporary = path.with_file_name(TEMPORARY);
    remove_if_present(&temporary)?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&temporary)?;
    let result = (|| -> Result<()> {
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
        let mut output = BufWriter::<File>::new(file);
        for turn in turns {
            serde_json::to_writer(&mut output, turn)?;
            output.write_all(b"\n")?;
        }
        output.flush()?;
        output.get_ref().sync_all()?;
        fs::rename(&temporary, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}
