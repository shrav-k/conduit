use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum Contract {
    #[serde(rename = "runtime-provider.v1")]
    Runtime,
    #[serde(rename = "machine-provider.v1")]
    Machines,
    #[serde(rename = "package-source.v1")]
    Packages,
    #[serde(rename = "secret-provider.v1")]
    Secrets,
}
impl Contract {
    pub fn operations(self) -> &'static [&'static str] {
        match self {
            Self::Runtime => &["run.v1", "stop.v1", "inspect-run.v1"],
            Self::Machines => &["list-machines.v1", "place.v1"],
            Self::Packages => &["fetch-package.v1"],
            Self::Secrets => &["get-secret.v1"],
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Index {
    pub contract: Contract,
    pub cases: Vec<String>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub id: String,
    pub title: String,
    pub contract: Contract,
    pub promises: Vec<String>,
    pub features: Vec<String>,
    pub fixtures: BTreeMap<String, Value>,
    #[serde(default)]
    pub markers: Vec<String>,
    pub steps: Vec<Step>,
}
#[derive(Debug, Deserialize)]
#[serde(tag = "step", rename_all = "snake_case", deny_unknown_fields)]
pub enum Step {
    Call {
        operation: String,
        arguments: Value,
        #[serde(default)]
        meta: BTreeMap<String, Value>,
        #[serde(default)]
        capture: BTreeMap<String, String>,
        assertions: Vec<Assertion>,
    },
    Pause {
        milliseconds: u64,
    },
    Restart {},
}
#[derive(Debug, Deserialize)]
#[serde(tag = "assert", rename_all = "snake_case", deny_unknown_fields)]
pub enum Assertion {
    Outcome { value: Outcome },
    Reason { value: Reason },
    Equal { pointer: String, value: Value },
    Pattern { pointer: String, pattern: String },
    NoMarkerAnywhere {},
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Succeeded,
    Failed,
    Unknown,
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Reason {
    Refused,
    Unavailable,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Descriptor {
    pub archive: String,
    pub item: String,
    #[serde(rename = "type")]
    pub type_ref: String,
    pub instance: String,
    pub revision: String,
    pub config: Value,
    pub parameters: BTreeMap<String, Value>,
    pub features: Vec<String>,
    pub tool_version_command: Vec<String>,
    pub pass_environment: Vec<String>,
    pub lab_paths: BTreeMap<String, String>,
}
pub fn safe_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 100
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
}
pub fn pointer(p: &str) -> bool {
    (p.is_empty() || p.starts_with('/')) && !regex::Regex::new(r"~([^01]|$)").unwrap().is_match(p)
}
impl Case {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !safe_id(&self.id)
            || self.title.is_empty()
            || self.promises.is_empty()
            || !self.steps.iter().any(|s| matches!(s, Step::Call { .. }))
        {
            return Err("case requires id, title, promises and steps");
        }
        if self.markers.iter().any(|s| s.is_empty()) {
            return Err("empty marker");
        }
        for step in &self.steps {
            match step {
                Step::Pause { milliseconds } if *milliseconds > 60_000 => {
                    return Err("pause exceeds 60000 milliseconds");
                }
                Step::Call {
                    operation,
                    arguments,
                    meta,
                    capture,
                    assertions,
                } => {
                    if !self.contract.operations().contains(&operation.as_str())
                        || !arguments.is_object()
                    {
                        return Err("operation or arguments invalid for contract");
                    }
                    if operation == "get-secret.v1"
                        && arguments.as_object().is_none_or(|m| !m.is_empty())
                    {
                        return Err("get-secret has no arguments");
                    }
                    let allowed: &[&str] = if self.contract == Contract::Secrets {
                        &["caller", "invocation", "conduit", "filters"]
                    } else {
                        &["caller", "invocation"]
                    };
                    if meta.keys().any(|k| !allowed.contains(&k.as_str())) {
                        return Err("unknown metadata key");
                    }
                    if matches!(operation.as_str(), "place.v1" | "fetch-package.v1")
                        && (!meta.contains_key("caller") || meta.contains_key("invocation"))
                    {
                        return Err("pre-dispatch call requires caller and no invocation");
                    }
                    if self.contract == Contract::Secrets
                        && allowed.iter().any(|k| !meta.contains_key(*k))
                    {
                        return Err("secret read requires caller, invocation, conduit and filters");
                    }
                    if capture.iter().any(|(n, p)| !safe_id(n) || !pointer(p)) {
                        return Err("invalid capture");
                    }
                    if assertions.is_empty() {
                        return Err("call requires assertions");
                    }
                    for a in assertions {
                        match a {
                            Assertion::Equal { pointer: p, .. }
                            | Assertion::Pattern { pointer: p, .. }
                                if !pointer(p) =>
                            {
                                return Err("invalid assertion pointer");
                            }
                            Assertion::Pattern { pattern, .. }
                                if regex::Regex::new(pattern).is_err() =>
                            {
                                return Err("invalid assertion pattern");
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
}
