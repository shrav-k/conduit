//! The queue's configuration: one JSON object, kept outside the tree, named by `--config`.
//!
//! An unknown key is refused, and so is a missing one that is not optional. Paths are absolute,
//! and nothing expands `~` or a variable.

use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// The identity every commit the queue makes or checks carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub name: String,
    pub email: String,
}

/// The gate's cache: `env` is added only when `address` accepts a TCP connection within 2 s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cache {
    pub address: String,
    pub env: BTreeMap<String, String>,
}

/// The configured gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gate {
    /// The gate's argv; `{base}` in any element becomes the pinned commit.
    pub argv: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub cache: Option<Cache>,
}

/// One `wo queue` configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub clone: PathBuf,
    pub worktrees: PathBuf,
    pub state: PathBuf,
    pub messages: PathBuf,
    pub remote: String,
    pub base: String,
    pub identity: Identity,
    pub python: Vec<String>,
    pub gate: Gate,
    pub gh: Vec<String>,
    pub pr_footer: String,
    pub trailers: Vec<String>,
    pub legacy_lock: Option<PathBuf>,
}

impl Config {
    /// The remote-tracking ref of the base, such as `refs/remotes/origin/main`.
    pub fn base_ref(&self) -> String {
        format!("refs/remotes/{}/{}", self.remote, self.base)
    }

    /// `REMOTE/BASE`, as the output names it.
    pub fn base_name(&self) -> String {
        format!("{}/{}", self.remote, self.base)
    }

    pub fn queue_file(&self) -> PathBuf {
        self.state.join("queue.jsonl")
    }

    pub fn ledger_file(&self) -> PathBuf {
        self.state.join("ledger.jsonl")
    }

    pub fn lock_file(&self) -> PathBuf {
        self.state.join("queue.lock")
    }

    pub fn records_dir(&self) -> PathBuf {
        self.state.join("gate-records")
    }

    pub fn message_file(&self, name: &str) -> PathBuf {
        self.messages.join(format!("{name}.txt"))
    }
}

const TOP: &[&str] = &[
    "clone",
    "worktrees",
    "state",
    "messages",
    "remote",
    "base",
    "identity",
    "python",
    "gate",
    "gh",
    "pr_footer",
    "trailers",
];

/// Reads and checks the configuration file at `path`.
pub fn load(path: &Path) -> Result<Config, String> {
    let text = fs::read_to_string(path).map_err(|error| {
        format!(
            "the configuration {} cannot be read: {error}",
            path.display()
        )
    })?;
    parse(&text).map_err(|error| format!("the configuration {}: {error}", path.display()))
}

/// Checks a configuration's text.
pub fn parse(text: &str) -> Result<Config, String> {
    let value: Value =
        serde_json::from_str(text).map_err(|error| format!("it is not JSON: {error}"))?;
    let top = value
        .as_object()
        .ok_or("it is not a JSON object".to_string())?;
    keys(top, "the configuration", TOP, &["legacy_lock"])?;
    let identity = object(top, "identity")?;
    keys(identity, "identity", &["name", "email"], &[])?;
    let gate = object(top, "gate")?;
    keys(gate, "gate", &["argv", "env"], &["cache"])?;
    let cache = match gate.get("cache") {
        None => None,
        Some(value) => {
            let cache = value
                .as_object()
                .ok_or("gate.cache is not an object".to_string())?;
            keys(cache, "gate.cache", &["address", "env"], &[])?;
            let address = text_value(cache, "address", "gate.cache.address")?;
            check_address(&address)?;
            Some(Cache {
                address,
                env: env_map(cache, "gate.cache.env")?,
            })
        }
    };
    let trailers = strings(top, "trailers", "trailers")?;
    if trailers.iter().any(String::is_empty) {
        return Err("trailers holds an empty prefix".into());
    }
    let pr_footer = match top.get("pr_footer") {
        Some(Value::String(text)) => text.clone(),
        _ => return Err("pr_footer is not a string".into()),
    };
    Ok(Config {
        clone: path_value(top, "clone")?,
        worktrees: path_value(top, "worktrees")?,
        state: path_value(top, "state")?,
        messages: path_value(top, "messages")?,
        remote: ref_part(top, "remote")?,
        base: ref_part(top, "base")?,
        identity: Identity {
            name: line_value(identity, "name", "identity.name")?,
            email: line_value(identity, "email", "identity.email")?,
        },
        python: strings(top, "python", "python")?,
        gate: Gate {
            argv: strings(gate, "argv", "gate.argv")?,
            env: env_map(gate, "gate.env")?,
            cache,
        },
        gh: strings(top, "gh", "gh")?,
        pr_footer,
        trailers,
        legacy_lock: match top.get("legacy_lock") {
            None => None,
            Some(_) => Some(path_value(top, "legacy_lock")?),
        },
    })
}

fn keys(
    object: &Map<String, Value>,
    what: &str,
    required: &[&str],
    optional: &[&str],
) -> Result<(), String> {
    let unknown: Vec<&str> = object
        .keys()
        .map(String::as_str)
        .filter(|key| !required.contains(key) && !optional.contains(key))
        .collect();
    if !unknown.is_empty() {
        return Err(format!(
            "{what} has unknown keys: {}; the keys it takes are {}",
            unknown.join(", "),
            required
                .iter()
                .chain(optional)
                .copied()
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    let missing: Vec<&str> = required
        .iter()
        .copied()
        .filter(|key| !object.contains_key(*key))
        .collect();
    if !missing.is_empty() {
        return Err(format!("{what} lacks the keys {}", missing.join(", ")));
    }
    Ok(())
}

fn object<'a>(top: &'a Map<String, Value>, key: &str) -> Result<&'a Map<String, Value>, String> {
    top.get(key)
        .and_then(Value::as_object)
        .ok_or(format!("{key} is not an object"))
}

fn text_value(object: &Map<String, Value>, key: &str, what: &str) -> Result<String, String> {
    match object.get(key) {
        Some(Value::String(text)) if !text.is_empty() => Ok(text.clone()),
        _ => Err(format!("{what} is not a non-empty string")),
    }
}

fn line_value(object: &Map<String, Value>, key: &str, what: &str) -> Result<String, String> {
    let text = text_value(object, key, what)?;
    if text.contains(['\n', '\r', '<', '>']) {
        return Err(format!(
            "{what} holds a line break or an angle bracket, which an identity cannot"
        ));
    }
    Ok(text)
}

fn path_value(object: &Map<String, Value>, key: &str) -> Result<PathBuf, String> {
    let text = text_value(object, key, key)?;
    let path = PathBuf::from(&text);
    if !path.is_absolute() {
        return Err(format!(
            "{key} is not an absolute path; nothing expands `~` or a variable"
        ));
    }
    Ok(path)
}

fn ref_part(object: &Map<String, Value>, key: &str) -> Result<String, String> {
    let text = text_value(object, key, key)?;
    let bad = text.starts_with(['-', '/', '.'])
        || text.ends_with(['/', '.'])
        || text.contains("..")
        || text
            .chars()
            .any(|c| c.is_whitespace() || c.is_control() || "~^:?*[\\".contains(c));
    if bad {
        return Err(format!("{key} is not a plain ref name: {text:?}"));
    }
    Ok(text)
}

fn strings(object: &Map<String, Value>, key: &str, what: &str) -> Result<Vec<String>, String> {
    let list = object
        .get(key)
        .and_then(Value::as_array)
        .ok_or(format!("{what} is not a list of strings"))?;
    let mut out = Vec::new();
    for item in list {
        out.push(
            item.as_str()
                .ok_or(format!("{what} holds an item that is not a string"))?
                .to_string(),
        );
    }
    if out.is_empty() {
        return Err(format!("{what} is empty"));
    }
    Ok(out)
}

fn env_map(object: &Map<String, Value>, what: &str) -> Result<BTreeMap<String, String>, String> {
    let key = what.rsplit('.').next().unwrap_or(what);
    let map = object
        .get(key)
        .and_then(Value::as_object)
        .ok_or(format!("{what} is not an object of names and values"))?;
    let mut out = BTreeMap::new();
    for (name, value) in map {
        if name.is_empty() || name.contains(['=', '\0']) {
            return Err(format!("{what} holds a name that cannot be a variable's"));
        }
        let value = value
            .as_str()
            .ok_or(format!("{what}.{name} is not a string"))?;
        if value.contains('\0') {
            return Err(format!("{what}.{name} holds a NUL"));
        }
        out.insert(name.clone(), value.to_string());
    }
    Ok(out)
}

fn check_address(address: &str) -> Result<(), String> {
    match address.rsplit_once(':') {
        Some((host, port)) if !host.is_empty() && port.parse::<u16>().is_ok() => Ok(()),
        _ => Err(format!("gate.cache.address is not host:port: {address:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use serde_json::{Value, json};

    fn sample() -> Value {
        json!({
            "clone": "/c", "worktrees": "/w", "state": "/s", "messages": "/m",
            "remote": "origin", "base": "main",
            "identity": {"name": "N", "email": "n@example.invalid"},
            "python": ["python3"], "gate": {"argv": ["g", "{base}"], "env": {}},
            "gh": ["gh"], "pr_footer": "", "trailers": ["Co-Authored-By: "]
        })
    }

    #[test]
    fn takes_a_whole_configuration() {
        let config = parse(&sample().to_string()).unwrap();
        assert_eq!(config.base_ref(), "refs/remotes/origin/main");
        assert!(config.legacy_lock.is_none() && config.gate.cache.is_none());
        let mut with = sample();
        with["legacy_lock"] = json!("/l");
        with["gate"]["cache"] = json!({"address": "127.0.0.1:9", "env": {"A": "b"}});
        let config = parse(&with.to_string()).unwrap();
        assert_eq!(config.gate.cache.unwrap().env["A"], "b");
    }

    #[test]
    fn refuses_unknown_missing_and_relative() {
        let mut unknown = sample();
        unknown["extra"] = json!(1);
        assert!(
            parse(&unknown.to_string())
                .unwrap_err()
                .contains("unknown keys: extra")
        );
        let mut missing = sample();
        missing.as_object_mut().unwrap().remove("gh");
        assert!(
            parse(&missing.to_string())
                .unwrap_err()
                .contains("lacks the keys gh")
        );
        let mut relative = sample();
        relative["clone"] = json!("~/clone");
        assert!(
            parse(&relative.to_string())
                .unwrap_err()
                .contains("absolute")
        );
        let mut nested = sample();
        nested["gate"]["extra"] = json!(1);
        assert!(
            parse(&nested.to_string())
                .unwrap_err()
                .contains("gate has unknown")
        );
        let mut address = sample();
        address["gate"]["cache"] = json!({"address": "nowhere", "env": {}});
        assert!(
            parse(&address.to_string())
                .unwrap_err()
                .contains("host:port")
        );
    }
}
