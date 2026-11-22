//! Working-copy planning and application through public engine requests.

mod apply;
mod compare;
mod read;
#[cfg(test)]
mod tests;

use conduit_core::packages::{Project, bytes_digest};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::{Path, PathBuf};

pub use read::{apply_command, command};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Name {
    pub kind: String,
    pub name: String,
}

impl Name {
    pub fn new(kind: &str, name: &str) -> Self {
        Self {
            kind: kind.into(),
            name: name.into(),
        }
    }

    fn object(value: &Value) -> Option<Self> {
        Some(Self::new(value["kind"].as_str()?, value["name"].as_str()?))
    }

    fn document(value: &Value) -> Option<Self> {
        Some(Self::new(
            value["kind"].as_str()?,
            value["metadata"]["name"].as_str()?,
        ))
    }

    fn reference(&self) -> Value {
        json!({"kind":self.kind,"name":self.name})
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.kind, self.name)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pin {
    pub address: String,
    pub version: Option<String>,
    pub digest: Option<String>,
}

impl Pin {
    fn from_value(value: &Value) -> Option<Self> {
        Some(Self {
            address: value["address"].as_str()?.to_owned(),
            version: optional_string(value, "version")?,
            digest: optional_string(value, "digest")?,
        })
    }

    fn label(&self) -> String {
        format!(
            "{}@{}",
            self.address,
            self.version.as_deref().unwrap_or("-")
        )
    }

    fn package_stop(&self, cause: &str, rule: &str) -> String {
        format!(
            "stop Package/{} version={} digest={} {cause} {rule}",
            self.address,
            self.version.as_deref().unwrap_or("-"),
            self.digest.as_deref().unwrap_or("-")
        )
    }
}

fn optional_string(value: &Value, member: &str) -> Option<Option<String>> {
    match value.get(member) {
        Some(value) => Some(Some(value.as_str()?.to_owned())),
        None => Some(None),
    }
}

#[derive(Clone, Debug)]
pub struct ObservedObject {
    pub id: String,
    pub revision: String,
    pub state: String,
    /// The framed, resolved definition, with object IDs translated to project names where known.
    pub definition: Value,
    pub packages: Vec<Value>,
    pub status: Value,
    /// Invocation IDs and operations whose inspection records report an unknown outcome.
    pub unknown_endings: BTreeMap<String, String>,
    pub inventory: Value,
    pub implementations: Value,
    pub made_by_engine: bool,
}

impl ObservedObject {
    fn is_template(&self) -> bool {
        self.state == "template"
    }

    fn pins(&self) -> impl Iterator<Item = Pin> {
        self.packages.iter().filter_map(Pin::from_value)
    }
}

#[derive(Default)]
pub struct Engine {
    pub objects: BTreeMap<Name, ObservedObject>,
    pub tombstones: Vec<Value>,
    pub installed: BTreeSet<Pin>,
    pub routes: BTreeMap<Name, String>,
    access_conduit: Option<String>,
}

impl Engine {
    fn protects(&self, name: &Name) -> bool {
        self.objects
            .get(name)
            .is_some_and(|object| object.made_by_engine)
    }
}

#[derive(Debug, Default)]
pub struct Ledger {
    pub objects: BTreeMap<Name, ObservedObject>,
    pub tombstones: Vec<Value>,
    pub files: Value,
    plans: Vec<Value>,
    journals: serde_json::Map<String, Value>,
    journal_update: u64,
    recovered_update: bool,
    registration_flags: BTreeMap<String, Approval>,
}

#[derive(Clone, Debug)]
struct Approval {
    flag: String,
    operand: String,
}

impl std::fmt::Display for Approval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.flag, self.operand)
    }
}

#[derive(Clone, Debug)]
pub struct DesiredObject {
    pub document: Value,
    pub resolved: ObservedObject,
    pub own_tools: Option<Value>,
    pub controls: BTreeMap<String, BTreeSet<String>>,
    pub reached_documents: BTreeMap<(String, String), String>,
    pub reached_packages: BTreeSet<String>,
    pub model_work: bool,
}

pub struct Desired {
    pub project: Project,
    pub objects: BTreeMap<Name, DesiredObject>,
    nested: Vec<NestedRegistration>,
    core_address: String,
    core_pin: Value,
    pub stops: BTreeSet<String>,
    pub available_archives: BTreeMap<Pin, Archive>,
}

#[derive(Clone, Debug)]
struct NestedRegistration {
    parent: Name,
    local: String,
    name: Name,
    resolved: ObservedObject,
}

#[derive(Clone, Debug)]
pub enum Archive {
    Absent,
    Mismatch,
    Read(Box<conduit_core::archives::InstalledPackage>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Expected {
    None,
    Step(usize),
}

#[derive(Clone, Debug)]
pub struct Step {
    pub tool: String,
    pub subject: Name,
    pub expected: Option<String>,
    pub relative: Expected,
    pub class: &'static str,
    pub rule: &'static str,
    pub arguments: Value,
    pub details: Vec<String>,
    action: apply::Action,
}

#[derive(Clone, Default, Debug)]
pub struct Plan {
    pub steps: Vec<Step>,
    names: BTreeMap<String, Name>,
    pub stops: BTreeSet<String>,
    pub differences: Vec<String>,
    pub closed_conduits: BTreeSet<String>,
    pub affected_bounds: BTreeSet<String>,
}

impl Plan {
    pub fn text(&self) -> String {
        let mut text = String::from("plan\n");
        for (index, step) in self.steps.iter().enumerate() {
            let expected = match step.relative {
                Expected::None => step.expected.as_deref().unwrap_or("-").to_owned(),
                Expected::Step(number) => format!("(step {number})"),
            };
            writeln!(
                text,
                "{} {} {} expected={} {} flag={} {}",
                index + 1,
                step.tool,
                step.subject,
                expected,
                step.class,
                step.action
                    .approval
                    .as_ref()
                    .map_or_else(|| "-".into(), ToString::to_string),
                step.rule
            )
            .unwrap();
            for detail in &step.details {
                writeln!(text, "detail {detail}").unwrap();
            }
        }
        for stop in &self.stops {
            writeln!(text, "{stop}").unwrap();
        }
        if !self.steps.is_empty() {
            writeln!(
                text,
                "affects bounds=[{}] closed-conduits=[{}]",
                self.affected_bounds
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(","),
                self.closed_conduits
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(",")
            )
            .unwrap();
        }
        for difference in &self.differences {
            writeln!(text, "{difference}").unwrap();
        }
        if self.steps.is_empty() && self.stops.is_empty() {
            text.push_str("no changes\n");
        } else {
            let flags: BTreeSet<_> = self
                .steps
                .iter()
                .filter_map(|step| step.action.approval.as_ref().map(ToString::to_string))
                .collect();
            writeln!(
                text,
                "flags [{}]",
                flags.into_iter().collect::<Vec<_>>().join(",")
            )
            .unwrap();
            text.push_str(if self.stops.is_empty() {
                "ready\n"
            } else {
                "stopped\n"
            });
        }
        text
    }

    pub fn digest(&self) -> String {
        bytes_digest(self.text().as_bytes())
    }
}

pub fn compare(desired: &Desired, ledger: &Ledger, engine: &Engine) -> Plan {
    compare::plan(desired, ledger, engine)
}

fn safe(value: &Value) -> Value {
    match value {
        Value::String(text)
            if Path::new(text).is_absolute()
                || url::Url::parse(text).is_ok_and(|url| url.scheme() == "file") =>
        {
            json!({"pathSha256":bytes_digest(text.as_bytes()).trim_start_matches("sha256:")})
        }
        Value::Object(fields) => Value::Object(
            fields
                .iter()
                .map(|(name, child)| {
                    let projection = if value["isSecret"] == true
                        && matches!(name.as_str(), "value" | "default")
                    {
                        json!("(secret: not shown)")
                    } else {
                        safe(child)
                    };
                    (name.clone(), projection)
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(safe).collect()),
        _ => value.clone(),
    }
}

fn batch_key(doc: &Value) -> (usize, &str, bool, &str) {
    let kind = doc["kind"].as_str().unwrap_or("");
    let name = doc["metadata"]["name"].as_str().unwrap_or("");
    let inline = doc.get("spec").is_some() && name.ends_with("--t");
    (
        rank(
            kind,
            &["Pool", "Resource", "Agent", "Fleet", "Interface", "Conduit"],
        ),
        if inline {
            name.trim_end_matches("--t")
        } else {
            name
        },
        !inline,
        name,
    )
}

pub fn order_registration_documents(documents: &mut [&Value]) {
    documents.sort_by(|a, b| batch_key(a).cmp(&batch_key(b)));
}

fn rank(kind: &str, order: &[&str]) -> usize {
    order
        .iter()
        .position(|item| *item == kind)
        .unwrap_or(order.len())
}

fn array(value: &Value) -> &[Value] {
    value.as_array().map(Vec::as_slice).unwrap_or_default()
}

fn text(value: &Value) -> &str {
    value.as_str().unwrap_or("")
}

fn same_version(a: &Pin, b: &Pin) -> bool {
    a.address == b.address
        && a.version.as_deref().map(|v| v.split('+').next())
            == b.version.as_deref().map(|v| v.split('+').next())
}

fn ledger_at(project: &Path) -> PathBuf {
    project.join(".conduit").join("state.yaml")
}
