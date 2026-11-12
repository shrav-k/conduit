//! Runtime-owned mechanisms. None of the sensitive input types implements Debug or Serialize.
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, sync::Arc};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::sync::CancellationToken;

/// Deliberately carries no backend error text, which can include credentials.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unavailable;
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
}
/// An explicit declaration, evaluated before staging. Machine labels are exact selectors.
pub struct Declaration {
    pub registry_types: Vec<String>,
    pub transports: Vec<String>,
    pub command: bool,
    pub machines: Vec<MachineSelector>,
}
pub struct MachineSelector {
    /// None means any hostname matching the labels.
    pub hostname: Option<String>,
    pub labels: BTreeMap<String, String>,
}
impl Declaration {
    pub fn supports(&self, item: &Value, machine: &Machine) -> bool {
        let machine_ok = self.machines.iter().any(|s| {
            s.hostname.as_ref().is_none_or(|h| h == &machine.hostname)
                && s.labels
                    .iter()
                    .all(|(k, v)| machine.labels.get(k) == Some(v))
        });
        if !machine_ok {
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
