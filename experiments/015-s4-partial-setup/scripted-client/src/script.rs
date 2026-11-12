use serde_json::{Map, Value};
use std::{collections::HashSet, fs, os::unix::fs::MetadataExt, path::PathBuf};

pub struct Script {
    pub id: String,
    pub trace: String,
    pub barrier_dir: Option<PathBuf>,
    pub redact: Vec<String>,
    pub call_ms: u64,
    pub barrier_ms: u64,
    pub steps: Vec<Step>,
}
pub enum Step {
    Discover,
    Call {
        tool: String,
        arguments: Map<String, Value>,
        mode: String,
        key: Option<String>,
    },
    Barrier {
        name: String,
    },
}
pub struct Problem {
    pub field: String,
    pub redact: Vec<String>,
}
type Checked<T> = Result<T, String>;
fn fields(v: &Value, allowed: &[&str], path: &str) -> Checked<()> {
    let o = v.as_object().ok_or_else(|| path.to_owned())?;
    if let Some(field) = o.keys().find(|k| !allowed.contains(&k.as_str())) {
        return Err(format!("{path}.{field}"));
    }
    Ok(())
}
fn string(v: &Value, problem: &'static str) -> Checked<String> {
    v.as_str()
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| problem.to_owned())
}
fn filename(v: &Value, max: usize, problem: &'static str) -> Checked<String> {
    let s = string(v, problem)?;
    if s.len() > max
        || !s.as_bytes()[0].is_ascii_alphanumeric()
        || !s
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
    {
        return Err(problem.into());
    }
    Ok(s)
}
fn timeout(v: &Value, field: &'static str) -> Checked<u64> {
    match v.get(field) {
        None => Ok(60000),
        Some(n) => n
            .as_u64()
            .filter(|n| (1..=170000).contains(n))
            .ok_or_else(|| field.to_owned()),
    }
}
fn absent(path: &std::path::Path) -> bool {
    matches!(fs::symlink_metadata(path), Err(e) if e.kind() == std::io::ErrorKind::NotFound)
}
impl Script {
    pub fn parse(input: &str) -> Result<Self, Problem> {
        let v: Value = serde_json::from_str(input).map_err(|_| Problem {
            field: "script.json".into(),
            redact: vec![],
        })?;
        // Even invalid scripts may put a synthetic marker in an unknown field's name.
        let redact = v["redact"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .filter(|s| s.chars().count() >= 8)
            .map(str::to_owned)
            .collect();
        Self::validate(&v).map_err(|field| Problem { field, redact })
    }
    fn validate(v: &Value) -> Checked<Self> {
        fields(
            v,
            &[
                "scr",
                "id",
                "trace",
                "barrierDir",
                "redact",
                "callTimeoutMs",
                "barrierTimeoutMs",
                "steps",
            ],
            "script",
        )?;
        if v["scr"] != "v1" {
            return Err("scr".into());
        }
        let id = filename(&v["id"], 64, "id")?;
        let trace = filename(&v["trace"], 128, "trace")?;
        if !absent(std::path::Path::new(&trace)) {
            return Err("trace.exists".into());
        }
        let call_ms = timeout(v, "callTimeoutMs")?;
        let barrier_ms = timeout(v, "barrierTimeoutMs")?;
        let redact = match v.get("redact") {
            None => vec![],
            Some(r) => r
                .as_array()
                .ok_or("redact")?
                .iter()
                .map(|r| {
                    let s = string(r, "redact")?;
                    if s.chars().count() < 8 {
                        return Err("redact".into());
                    }
                    Ok(s)
                })
                .collect::<Checked<Vec<_>>>()?,
        };
        let barrier_dir = v
            .get("barrierDir")
            .map(|p| string(p, "barrierDir").map(PathBuf::from))
            .transpose()?;
        if let Some(dir) = &barrier_dir {
            let m = fs::symlink_metadata(dir).map_err(|_| "barrierDir")?;
            // The client's effective user owns the handshake directory; reject symlink directories.
            if !dir.is_absolute()
                || !m.is_dir()
                || m.uid() != unsafe { libc::geteuid() }
                || m.mode() & 0o022 != 0
            {
                return Err("barrierDir".into());
            }
        }
        let raw_steps = v["steps"]
            .as_array()
            .filter(|s| (1..=256).contains(&s.len()))
            .ok_or("steps")?;
        let mut steps = Vec::new();
        let mut names = HashSet::new();
        for (index, s) in raw_steps.iter().enumerate() {
            let path = format!("steps[{index}]");
            steps.push(match s["step"].as_str() {
                Some("discover") => {
                    fields(s, &["step"], &path)?;
                    Step::Discover
                }
                Some("call") => {
                    fields(s, &["step", "tool", "arguments", "mode", "key"], &path)?;
                    let tool = string(&s["tool"], "steps.call.tool")?;
                    let mode = string(&s["mode"], "steps.call.mode")?;
                    if !["keyed", "fresh", "unkeyed"].contains(&mode.as_str()) {
                        return Err("steps.call.mode".into());
                    }
                    let key = s
                        .get("key")
                        .map(|k| string(k, "steps.call.key"))
                        .transpose()?;
                    if (mode == "keyed") != key.is_some() {
                        return Err("steps.call.key".into());
                    }
                    let arguments = match s.get("arguments") {
                        None => Map::new(),
                        Some(a) => a.as_object().ok_or("steps.call.arguments")?.clone(),
                    };
                    Step::Call {
                        tool,
                        arguments,
                        mode,
                        key,
                    }
                }
                Some("barrier") => {
                    fields(s, &["step", "name"], &path)?;
                    let name = string(&s["name"], "steps.barrier.name")?;
                    if name.len() > 64
                        || !name.as_bytes()[0].is_ascii_lowercase()
                            && !name.as_bytes()[0].is_ascii_digit()
                        || !name
                            .bytes()
                            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
                        || !names.insert(name.clone())
                    {
                        return Err("steps.barrier.name".into());
                    }
                    let dir = barrier_dir.as_ref().ok_or("barrierDir")?;
                    for suffix in ["reached", "release", "acknowledged"] {
                        let filename = format!("{name}.{suffix}");
                        // Compare names alone: directory aliases can survive canonicalization,
                        // and ASCII case may not distinguish files on the host filesystem.
                        if trace.eq_ignore_ascii_case(&filename) {
                            return Err("trace.barrier-collision".into());
                        }
                        if !absent(&dir.join(filename)) {
                            return Err("steps.barrier.files".into());
                        }
                    }
                    Step::Barrier { name }
                }
                _ => return Err("steps.step".into()),
            });
        }
        Ok(Self {
            id,
            trace,
            barrier_dir,
            redact,
            call_ms,
            barrier_ms,
            steps,
        })
    }
}
