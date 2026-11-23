//! Runtime-owned mechanisms. None of the sensitive input types implements Debug or Serialize.
use async_trait::async_trait;
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;
use std::{collections::BTreeMap, sync::Arc};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::sync::CancellationToken;

/// A runtime failure, optionally with text the runtime writes itself naming what failed
/// (a setting, variable name, program or machine hostname), never a value. Never pass through
/// operating-system, library or remote-tool error text: it can contain credentials.
/// Only staging and start reasons can reach answers, after the kit checks and bounds them.
/// Compare errors as values; do not use `Unavailable` as a pattern.
#[derive(Clone, PartialEq, Eq)]
pub struct Unavailable {
    reason: Option<String>,
}
/// A failure without a reason, preserving existing value uses of `Unavailable`.
#[allow(non_upper_case_globals)]
pub const Unavailable: Unavailable = Unavailable { reason: None };
impl Unavailable {
    /// Attach a runtime-authored explanation naming what failed, never a supplied value or
    /// operating-system, library or remote-tool error text.
    pub fn because(reason: impl Into<String>) -> Self {
        Self {
            reason: Some(reason.into()),
        }
    }
    pub(crate) fn into_reason(self) -> Option<String> {
        self.reason
    }
}
impl std::fmt::Debug for Unavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unavailable")
    }
}
pub type Result<T> = std::result::Result<T, Unavailable>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Machine {
    pub hostname: String,
    pub labels: BTreeMap<String, String>,
}
/// The configuration and revision are exclusively those of a provider call.
#[derive(Clone, Serialize, Deserialize)]
pub struct CallContext {
    pub config: Value,
    pub revision: String,
    pub machine: Machine,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RunState {
    Running,
    Stopped,
    Unknown,
}
/// Durable recovery data only: never add input items, environment, secrets or identity.
#[derive(Clone, Serialize, Deserialize)]
pub struct RunRecord {
    pub instance: String,
    pub context: CallContext,
    pub state: RunState,
    pub owed_stop: bool,
}
impl RunRecord {
    pub fn same_run(&self, other: &Self) -> bool {
        self.instance == other.instance
            && self.context.machine.hostname == other.context.machine.hostname
    }
}
#[derive(Deserialize)]
pub struct Identity {
    pub environment: BTreeMap<String, String>,
    pub files: BTreeMap<String, String>,
}
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Secrets {
    #[serde(default)]
    pub environment: BTreeMap<String, String>,
    #[serde(default)]
    pub package_arguments: BTreeMap<String, String>,
}
/// Staging must install all identity files, and deliver secrets only at their named bindings.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInput {
    pub instance: String,
    pub item: Value,
    #[serde(default)]
    pub package_arguments: BTreeMap<String, String>,
    #[serde(default)]
    pub environment: BTreeMap<String, String>,
    #[serde(default)]
    pub secrets: Secrets,
    pub identity: Identity,
    pub program: Option<String>,
    pub archive: Option<ArchiveDescriptor>,
}

#[derive(Deserialize)]
pub struct ArchiveDescriptor {
    pub digest: String,
    #[serde(deserialize_with = "archive_size")]
    pub size: u64,
    pub urls: Vec<String>,
}

fn archive_size<'de, D: Deserializer<'de>>(deserializer: D) -> std::result::Result<u64, D::Error> {
    let number = serde_json::Number::deserialize(deserializer)?;
    if let Some(size) = number.as_u64() {
        return Ok(size);
    }
    // JSON Schema accepts integral decimals. The upper bound is exclusive because
    // u64::MAX rounds up to 2^64 as an f64.
    const U64_END: f64 = 18_446_744_073_709_551_616.0;
    number
        .as_f64()
        .filter(|size| (0.0..U64_END).contains(size) && size.fract() == 0.0)
        .map(|size| size as u64)
        .ok_or_else(|| de::Error::custom("archive size is not an unsigned 64-bit integer"))
}

/// An explicit declaration, evaluated before staging. Machine labels are exact selectors.
pub struct Declaration {
    pub registry_types: Vec<String>,
    pub transports: Vec<String>,
    pub command: bool,
    pub packaged_programs: bool,
    pub machines: Vec<MachineSelector>,
}
pub struct MachineSelector {
    /// None means any hostname matching the labels.
    pub hostname: Option<String>,
    pub labels: BTreeMap<String, String>,
}
impl Declaration {
    fn supports_machine(&self, machine: &Machine) -> bool {
        self.machines.iter().any(|s| {
            s.hostname.as_ref().is_none_or(|h| h == &machine.hostname)
                && s.labels
                    .iter()
                    .all(|(k, v)| machine.labels.get(k) == Some(v))
        })
    }

    pub fn supports_packaged_programs(&self, machine: &Machine) -> bool {
        self.packaged_programs && self.command && self.supports_machine(machine)
    }

    pub fn supports(&self, item: &Value, machine: &Machine) -> bool {
        if !self.supports_machine(machine) {
            return false;
        }
        if let Some(command) = item.get("command") {
            let Some(program) = command[0].as_str() else {
                return false;
            };
            // MAN-010 defines paths by `/`, independently of the provider host's OS.
            return self.command && (!program.contains('/') || program.starts_with('/'));
        }
        let package = &item["package"];
        self.registry_types
            .iter()
            .any(|s| package["registryType"] == *s)
            && self
                .transports
                .iter()
                .any(|s| package["transport"]["type"] == *s)
    }
}

/// Semantic checks shared by the protocol handler and packaged-program backends.
pub fn check_packaged_program(
    item: &Value,
    program: &str,
) -> std::result::Result<(), &'static str> {
    let original = item["command"][0].as_str();
    if !original.is_some_and(|path| path.contains('/') && !path.starts_with('/'))
        || !item["platforms"]
            .as_array()
            .is_some_and(|pairs| !pairs.is_empty())
        || item.get("package").is_some()
        || item.get("remote").is_some()
    {
        return Err(
            "run.v2 item must be a command item with a relative program that states platforms",
        );
    }
    if program.is_empty()
        || program.starts_with('/')
        || program.contains('\0')
        || program.split('/').any(|part| part == "..")
    {
        return Err("run.v2 program must be a relative path inside the archive");
    }
    Ok(())
}
pub trait ByteChannel: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> ByteChannel for T {}
pub type Channel = Box<dyn ByteChannel>;

#[async_trait]
pub trait RuntimeBackend: Send + Sync + 'static {
    type Staged: Send;
    fn declaration(&self) -> Declaration;
    /// Administrative recovery, not a provider call. Return every run, including orphans,
    /// with its original provider-call context. An incomplete inventory must return Err.
    async fn inventory(&self) -> Result<Vec<RunRecord>>;
    /// On cancellation, clean staged material before returning. Err starts no run.
    async fn stage(
        &self,
        context: &CallContext,
        input: &RunInput,
        cancel: &CancellationToken,
    ) -> Result<Self::Staged>;
    /// A cancelled start must finish without any later launch. Err may have started a tree;
    /// the kit therefore stops it. Enforce the supplied lifetime on the run's machine too.
    async fn start(
        &self,
        context: &CallContext,
        instance: &str,
        staged: Self::Staged,
        cancel: &CancellationToken,
        lifetime: std::time::Duration,
    ) -> Result<Channel>;
    /// Ok proves that the whole tree is gone and that no backend restart can follow.
    async fn stop(&self, context: &CallContext, instance: &str) -> Result<()>;
    /// Stopped proves that no run this provider started for this ID on this machine can run.
    async fn state(&self, context: &CallContext, instance: &str) -> Result<RunState>;
}
#[async_trait]
pub trait StateStore: Send + Sync + 'static {
    async fn load(&self) -> Result<Vec<RunRecord>>;
    /// Atomic durable upsert by (instance, machine hostname), acknowledged only after commit.
    /// Storage must survive provider retirement and be usable by the external stop drainer.
    async fn save(&self, record: &RunRecord) -> Result<()>;
}
#[async_trait]
pub trait Gate: Send + Sync + 'static {
    /// The gate alone receives identity beside staging. Failed opens close their channel.
    /// Opening must quiesce on cancellation, leaving no listener after it returns Err.
    async fn open(
        &self,
        channel: Channel,
        identity: &Identity,
        cancel: &CancellationToken,
    ) -> Result<Arc<dyn GateHandle>>;
}
#[async_trait]
pub trait GateHandle: Send + Sync {
    /// A current Streamable HTTP URL, containing no secret or identity material.
    fn url(&self) -> String;
    /// Idempotent; closes the listener and channel before returning.
    async fn close(&self);
    /// Resolves when the gate or its channel ends, even without further incoming calls.
    async fn ended(&self);
}
