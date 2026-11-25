//! Durable boundary for the pure core, with an optional owned stdio runner.
pub mod author;
pub mod boundary;
pub mod diagnostics;
pub mod http_bridge;
mod https;
mod identity;
mod install;
pub mod plan;
pub mod runner;
pub mod server;
pub mod sign_in;
mod store;
pub mod user_config;
pub mod value;
use chrono::{DateTime, SecondsFormat, Utc};
use conduit_core::{Context, Input, RecordFault, State, Transition};
pub use identity::RunIdentity;

/// Install the single TLS crypto provider used by the shell and its tests.
pub fn install_crypto_provider() {
    let _ = rustls::crypto::ring::default_provider().install_default();
}
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex, Weak};
use std::time::{Duration, Instant};
#[cfg(test)]
use store::Stored;
use store::{Memory, StateWriteError, Store};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, String>;

const ROOT_CLIENT_ID_FILE: &str = "root-client-id";

#[derive(Clone)]
pub struct Config {
    pub trust_domain: Option<String>,
    pub max_output_bytes: usize,
    pub max_output_age: Duration,
    pub engine_url: String,
    pub engine_https_url: Option<String>,
    pub https_listen: Option<std::net::SocketAddr>,
    pub https_roots: Option<String>,
    pub pass_environment: Vec<String>,
    pub initialize_timeout: Duration,
    pub call_timeout: Duration,
    pub admission_ask_timeout: Duration,
    pub stop_grace: Duration,
    pub grace_period: Duration,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            trust_domain: None,
            max_output_bytes: 64 * 1024 * 1024,
            max_output_age: Duration::from_secs(7 * 24 * 60 * 60),
            engine_url: "http://127.0.0.1:3000/mcp".into(),
            engine_https_url: None,
            https_listen: None,
            https_roots: None,
            pass_environment: Vec::new(),
            initialize_timeout: Duration::from_secs(30),
            call_timeout: Duration::from_secs(120),
            admission_ask_timeout: Duration::from_secs(30),
            stop_grace: Duration::from_secs(10),
            grace_period: Duration::from_secs(30),
        }
    }
}
impl Config {
    /// Read public endpoints and certificate roots without opening/recovering a live store.
    pub fn for_store(directory: &Path) -> Result<Self> {
        Self::read_store(directory, false)
    }

    fn for_context(directory: &Path) -> Result<Self> {
        Self::read_store(directory, true)
    }

    fn read_store(directory: &Path, require_fixed_url: bool) -> Result<Self> {
        let path = directory.join("engine-url.json");
        let mut config = Self::default();
        if require_fixed_url || path.try_exists().map_err(|e| e.to_string())? {
            let read_url = store::private_read(&path).and_then(|bytes| {
                serde_json::from_slice(&bytes).map_err(|_| "invalid stored engine URL".into())
            });
            config.engine_url = read_url.map_err(|error| {
                if require_fixed_url {
                    format!("cannot read fixed URL file {path:?}: {error}")
                } else {
                    error
                }
            })?;
        }
        config.read_https(directory)?;
        Ok(config)
    }
}

/// Deferred means that an adapter owns the completion and will submit it later.
/// Missing adapters use Missing, which is translated to a core completion/report.
pub enum Execution {
    Complete(Value),
    SetupReport(Value),
    Deferred,
    Missing,
    Done,
}

pub trait Executor: Send + Sync {
    fn dispatch(&self, _intent: &Value) -> Result<Execution> {
        Ok(Execution::Missing)
    }
    fn refresh_tools_and_call(&self, _intent: &Value) -> Result<Execution> {
        Ok(Execution::Missing)
    }
    fn builtin_runner(
        &self,
        _intent: &Value,
        _identity: Option<&RunIdentity>,
    ) -> Result<Execution> {
        Ok(Execution::Missing)
    }
    fn provider_call(&self, _intent: &Value, _identity: Option<&RunIdentity>) -> Result<Execution> {
        Ok(Execution::Missing)
    }
    fn connect(&self, _intent: &Value) -> Result<Execution> {
        Ok(Execution::Missing)
    }
    // Cancellation intents are notifications for already-ended invocations, with no core
    // completion input. The defaults explicitly acknowledge that no transport exists.
    fn cancel(&self, _intent: &Value) -> Result<Execution> {
        Ok(Execution::Done)
    }
    fn cancel_setup_parts(&self, _intent: &Value) -> Result<Execution> {
        Ok(Execution::Done)
    }
    fn admission_asks_started(&self, _asks: &[Value]) -> Result<()> {
        Ok(())
    }
    fn tools_changed(&self, _notifications: &[Value]) -> Result<()> {
        Ok(())
    }
}
pub struct MissingExecutor;
impl Executor for MissingExecutor {}

#[derive(Default)]
struct ReplyCell {
    value: Mutex<Option<Value>>,
    changed: Condvar,
}

/// An internal wait handle, never a successful pending MCP result.
pub struct Ticket {
    invocation: Option<String>,
    cell: Arc<ReplyCell>,
}
impl Ticket {
    pub fn invocation(&self) -> Option<&str> {
        self.invocation.as_deref()
    }
    pub fn try_reply(&self) -> Result<Option<Value>> {
        Ok(self
            .cell
            .value
            .lock()
            .map_err(|_| "reply lock poisoned")?
            .clone())
    }
    pub fn wait(&self) -> Result<Value> {
        let mut value = self.cell.value.lock().map_err(|_| "reply lock poisoned")?;
        while value.is_none() {
            value = self
                .cell
                .changed
                .wait(value)
                .map_err(|_| "reply lock poisoned")?;
        }
        Ok(value.clone().unwrap())
    }
}

struct Writer {
    store: Store,
    waiters: BTreeMap<(String, u64), Vec<Weak<ReplyCell>>>,
    read_values: BTreeMap<String, String>,
    // REC-007: the replied-to steps waiting for the next state write.
    held_records: VecDeque<RecordFault>,
    open_micros: Option<u64>,
    unwritten: u64,
}

fn micros(duration: Duration) -> u64 {
    duration.as_micros().min(u64::MAX as u128) as u64
}

#[derive(Default)]
struct CommitMeasures {
    state_bytes: usize,
    writes: u64,
    step_time: Duration,
    write_time: Duration,
}
impl CommitMeasures {
    fn written(&mut self, (bytes, elapsed): (usize, Duration)) {
        self.state_bytes = bytes;
        self.writes += 1;
        self.write_time += elapsed;
    }
}
struct Committed {
    ticket: Ticket,
    intents: Vec<Intent>,
    notifications: Vec<Value>,
    fault_endings: Option<BTreeSet<String>>,
    admission_asks: Vec<Value>,
}

struct Intent {
    descriptor: Value,
    secrets: Option<Value>,
    from_record_fault: bool,
}

#[derive(Debug)]
enum PersistenceError {
    Write(StateWriteError),
    Other(String),
}

impl From<&str> for PersistenceError {
    fn from(message: &str) -> Self {
        Self::Other(message.to_owned())
    }
}

impl From<String> for PersistenceError {
    fn from(message: String) -> Self {
        Self::Other(message)
    }
}

impl From<PersistenceError> for String {
    fn from(error: PersistenceError) -> Self {
        match error {
            PersistenceError::Write(error) => error.message,
            PersistenceError::Other(message) => message,
        }
    }
}

pub struct Shell {
    directory: std::path::PathBuf,
    pub(crate) run_groups: Mutex<Vec<Arc<runner::ProcessGroup>>>,
    writer: Mutex<Writer>,
    authority: identity::Authority,
    pub context: Context,
    pub config: Config,
    // Only tests can replace OS evidence; the production entry point always generates it.
    #[cfg(test)]
    evidence: Mutex<VecDeque<Value>>,
    #[cfg(test)]
    trace: Mutex<Option<Vec<Transition>>>,
}

fn local_engine_url(address: &str) -> Result<url::Url> {
    let url = url::Url::parse(address).map_err(|_| "invalid engine URL")?;
    let loopback = matches!(url.host(), Some(url::Host::Domain("localhost")))
        || matches!(url.host(), Some(url::Host::Ipv4(ip)) if ip == std::net::Ipv4Addr::LOCALHOST)
        || matches!(url.host(), Some(url::Host::Ipv6(ip)) if ip == std::net::Ipv6Addr::LOCALHOST);
    if url.scheme() != "http"
        || !loopback
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || url.query().is_some()
        || url.path() != "/mcp"
        || url.port_or_known_default() == Some(0)
    {
        return Err(
            "this shell requires an HTTP engine address at 127.0.0.1, localhost or ::1".into(),
        );
    }
    Ok(url)
}

impl Shell {
    pub fn open(directory: &Path, context: Context, mut config: Config) -> Result<Self> {
        let started = Instant::now();
        // DL2/DL13: Store hands its one parsed committed snapshot to identity and recovery.
        local_engine_url(&config.engine_url)?;
        let store = Store::open(directory)?;
        let address = store.directory.join("engine-url.json");
        let address_exists = address.try_exists().map_err(|e| e.to_string())?;
        if address_exists {
            let fixed: String = serde_json::from_slice(&store::private_read(&address)?)
                .map_err(|_| "invalid stored engine URL")?;
            if fixed != config.engine_url {
                return Err("engine address differs from the store's fixed token audience".into());
            }
        }
        // DL13/IFC-042: resolve and validate the fixed audience under the writer lease.
        config.fix_https(&store.directory)?;
        if !address_exists {
            store::atomic_write(
                &store.directory,
                "engine-url.json",
                &serde_json::to_vec(&config.engine_url).map_err(|e| e.to_string())?,
            )?;
        }
        let authority = identity::Authority::open(&store, config.trust_domain.as_deref())?;
        let mut shell = Self {
            directory: store.directory.clone(),
            run_groups: Mutex::new(Vec::new()),
            writer: Mutex::new(Writer {
                store,
                waiters: BTreeMap::new(),
                read_values: BTreeMap::new(),
                held_records: VecDeque::new(),
                open_micros: None,
                unwritten: 0,
            }),
            authority,
            context,
            config,
            #[cfg(test)]
            evidence: Mutex::new(VecDeque::new()),
            #[cfg(test)]
            trace: Mutex::new(None),
        };
        // An open is a restart boundary, even for an empty store. No intent is replayed.
        let recover = json!({"kind":"recover"});
        shell
            .writer
            .get_mut()
            .map_err(|_| "writer lock poisoned")?
            .open_micros = Some(micros(started.elapsed()));
        let recovered = shell.commit(recover)?;
        if !recovered.intents.is_empty() {
            return Err("recovery unexpectedly requested external work".into());
        }
        Ok(shell)
    }

    pub fn state(&self) -> Result<Arc<State>> {
        // DL2/DL13: each reader shares the last commit, with no parse or deep copy.
        self.writer
            .lock()
            .map_err(|_| "writer lock poisoned")?
            .store
            .snapshot()
    }

    /// PRV-029: stable, private storage, separate from each run's working directory.
    pub(crate) fn state_directory(&self, instance: &str) -> Result<std::path::PathBuf> {
        let writer = self.writer.lock().map_err(|_| "writer lock poisoned")?;
        let state = writer.store.snapshot()?;
        if !state
            .objects
            .iter()
            .any(|o| o["id"] == instance && o.get("spec").is_none())
            || Uuid::parse_str(instance).is_err()
        {
            return Err("state directory needs an existing instance ID".into());
        }
        let root = writer.store.directory.join("instances");
        store::private_directory(&root)?;
        let path = root.join(instance);
        store::private_directory(&path)?;
        Ok(path)
    }

    /// Inputs here are trusted embedding values, never raw consumer wire messages.
    /// Supplied evidence is replaced with shell evidence.
    pub fn begin(&self, input: Value, executor: &dyn Executor) -> Result<Ticket> {
        let committed = self.commit_input(input, true)?;
        self.drive_committed(
            committed.intents,
            committed.notifications,
            committed.admission_asks,
            committed.fault_endings.is_some(),
            executor,
        )?;
        Ok(committed.ticket)
    }
    pub fn submit(&self, input: Value, executor: &dyn Executor) -> Result<Value> {
        self.begin(input, executor)?.wait()
    }
    pub fn complete(
        &self,
        invocation: &str,
        attempt: u64,
        completion: Value,
        executor: &dyn Executor,
    ) -> Result<Ticket> {
        self.begin(json!({"kind":"complete","invocation":invocation,"attempt":attempt,"completion":completion}), executor)
    }
    pub fn setup_report(
        &self,
        invocation: &str,
        report: Value,
        executor: &dyn Executor,
    ) -> Result<Ticket> {
        self.begin(
            json!({"kind":"setup-report","invocation":invocation,"report":report}),
            executor,
        )
    }
    pub fn verify_token(&self, token: Option<&str>) -> Result<conduit_core::access::Identity> {
        // DL13: revocation checks the instance's existence in the last committed State.
        Ok(self
            .authority
            .verify(token, &self.config.engine_url, self.state()?.as_ref()))
    }
    pub fn materialize_identity(&self, descriptor: &Value) -> Result<RunIdentity> {
        // DL2/DL13: identity materialization checks the last committed snapshot.
        let writer = self.writer.lock().map_err(|_| "writer lock poisoned")?;
        let identity = self.authority.materialize(
            &writer.store,
            descriptor,
            &self.config.engine_url,
            writer.store.snapshot()?.as_ref(),
            &self.context,
        )?;
        self.client_token(
            &writer,
            descriptor["materializeFor"]
                .as_str()
                .ok_or("missing identity subject")?,
        )?;
        Ok(identity)
    }

    /// IFC-042: the same proof checks, with only the receiver's audience changed.
    pub fn verify_https_token(
        &self,
        token: Option<&str>,
    ) -> Result<conduit_core::access::Identity> {
        Ok(match self.config.engine_https_url.as_deref() {
            Some(address) => self
                .authority
                .verify(token, address, self.state()?.as_ref()),
            None => conduit_core::access::Identity::None,
        })
    }

    /// DL13/PRV-022: provider runs never receive the own-runner HTTP identity.
    pub fn materialize_provider_identity(&self, descriptor: &Value) -> Result<Option<RunIdentity>> {
        let writer = self.writer.lock().map_err(|_| "writer lock poisoned")?;
        let instance = descriptor["materializeFor"]
            .as_str()
            .ok_or("missing identity subject")?;
        let state = writer.store.snapshot()?;
        // IFC-042: no issuance path gives root the https audience.
        if state
            .objects
            .iter()
            .any(|o| o["id"] == instance && o["kind"] == "Interface" && o["name"] == "root")
        {
            return Err("root cannot receive the https token audience (IFC-042)".into());
        }
        self.client_token(&writer, instance)?;
        let Some(address) = self.config.engine_https_url.as_deref() else {
            return Ok(None);
        };
        let mut identity = self.authority.materialize(
            &writer.store,
            descriptor,
            address,
            &state,
            &self.context,
        )?;
        // PRV-022/DL13: preserve the store root first, with every PEM block on its own lines.
        if let Some(roots) = self.config.https_roots.as_deref() {
            let path = identity.directory.join(
                descriptor["fileNames"][0]
                    .as_str()
                    .ok_or("invalid identity file name")?,
            );
            let root = String::from_utf8(store::private_read(&path)?)
                .map_err(|_| "invalid store root PEM")?;
            let bundle = format!("{}\n{}", root.trim_end(), https::roots(roots)?);
            store::atomic_write(
                &identity.directory,
                path.file_name()
                    .and_then(|s| s.to_str())
                    .ok_or("invalid identity file name")?,
                bundle.as_bytes(),
            )?;
        }
        self.authority
            .add_run_certificate(&mut identity, instance, &self.context)?;
        // Keep the augmented trust bundle in the in-memory provider payload as well.
        let bundle_name = descriptor["fileNames"][0]
            .as_str()
            .ok_or("invalid identity file name")?;
        identity.files.insert(
            bundle_name.into(),
            String::from_utf8(store::private_read(&identity.directory.join(bundle_name))?)
                .map_err(|_| "invalid trust bundle")?,
        );
        Ok(Some(identity))
    }

    fn client_token(&self, writer: &Writer, instance: &str) -> Result<()> {
        // DL13: issue a token only for an instance in the last committed State.
        let state = writer.store.snapshot()?;
        let subject = state
            .objects
            .iter()
            .find(|o| {
                o["id"] == instance
                    && o.get("spec").is_none()
                    && ["Interface", "Agent", "Resource", "Pool"]
                        .iter()
                        .any(|k| o["kind"] == *k)
            })
            .ok_or("token subject is not an instance")?;
        let directory = writer.store.directory.join("client-tokens");
        store::private_directory(&directory)?;
        let token = self.authority.token(instance, &self.config.engine_url)?;
        store::atomic_write(&directory, instance, token.as_bytes())?;
        if subject["kind"] == "Interface" && subject["name"] == "root" {
            store::atomic_write(
                &writer.store.directory,
                ROOT_CLIENT_ID_FILE,
                instance.as_bytes(),
            )?;
        }
        Ok(())
    }

    pub fn issue_client_token(&self, instance: &str) -> Result<()> {
        let writer = self.writer.lock().map_err(|_| "writer lock poisoned")?;
        self.client_token(&writer, instance)
    }

    fn transition(&self, state: State, store: &Store, input: Value) -> Result<Transition> {
        self.transition_input(state, store, input, &VecDeque::new())
            .map(|(t, _)| t)
    }

    fn replay_held(&self, mut state: State, held: &VecDeque<RecordFault>) -> Result<State> {
        for input in held {
            state = input.step(state, &self.context)?.state;
        }
        Ok(state)
    }

    fn transition_input(
        &self,
        state: State,
        store: &Store,
        mut input: Value,
        held: &VecDeque<RecordFault>,
    ) -> Result<(Transition, Value)> {
        if input["kind"] == "first-start"
            && input
                .get("trustDomain")
                .is_some_and(|d| d != self.authority.trust_domain())
        {
            return Err("first-start trustDomain differs from signing authority (FST-007)".into());
        }
        if input["kind"] == "first-start" && input.get("trustDomain").is_none() {
            input["trustDomain"] = json!(self.authority.trust_domain());
        }
        let kind = input["kind"]
            .as_str()
            .ok_or("input needs kind")?
            .to_string();
        let needs_evidence = matches!(
            kind.as_str(),
            "first-start"
                | "register-project"
                | "invoke"
                | "complete"
                | "respond"
                | "sign-in-wait-ended"
                | "admission-ask-deadline"
                | "cancel"
                | "setup-report"
                | "stop"
                | "recover"
                | "builtin-run-stopped"
                | "builtin-secret-provider-exited"
                | "access"
                | "submit"
                | "request"
        );
        let time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        #[cfg(test)]
        let scripted = if needs_evidence {
            self.evidence
                .lock()
                .map_err(|_| "evidence lock poisoned")?
                .pop_front()
        } else {
            None
        };
        // DL6: retry a consumed State only by rereading the last durable commit.
        let mut state = Some(state);
        for supply in [256, 16384] {
            if needs_evidence {
                input["evidence"] = evidence(&kind, &time, supply)?;
                #[cfg(test)]
                if let Some(ref scripted) = scripted {
                    input["evidence"] = scripted.clone();
                }
            }
            let parsed: Input = serde_json::from_value(input.clone())
                .map_err(|e| format!("embedding input: {e}"))?;
            let state = match state.take() {
                Some(state) => state,
                None => self.replay_held(store.read()?.state, held)?,
            };
            match conduit_core::step_owned(state, parsed, &self.context) {
                Ok(t) => {
                    if kind == "invoke"
                        && input.get("archive").is_none()
                        && t.decisions
                            .iter()
                            .any(|d| d.action == "archive-unavailable")
                    {
                        let args = &input["request"]["arguments"];
                        let path = url::Url::parse(args["fetch"].as_str().ok_or("missing fetch")?)
                            .map_err(|e| e.to_string())?
                            .to_file_path()
                            .map_err(|_| "invalid file URL")?;
                        let completion = match std::fs::read(&path) {
                            Ok(bytes) => json!({"status":"complete","bytes":bytes}),
                            Err(_) => json!({"status":"unavailable"}),
                        };
                        let directory = url::Url::from_directory_path(
                            self.directory.join("archive-catalog").join(
                                args["digest"]
                                    .as_str()
                                    .unwrap_or("")
                                    .trim_start_matches("sha256:"),
                            ),
                        )
                        .map_err(|_| "invalid archive directory")?
                        .to_string();
                        input["archive"] = json!({"name":path.to_string_lossy(),"address":args["address"],"version":args["version"],"digest":args["digest"],"directory":directory,"completion":completion});
                        let parsed = serde_json::from_value(input.clone())
                            .map_err(|e| format!("embedding input: {e}"))?;
                        // DL2/IMP-007: archive evidence retries the committed State, not the trial.
                        drop(t);
                        let t = conduit_core::step_owned(
                            self.replay_held(store.read()?.state, held)?,
                            parsed,
                            &self.context,
                        )
                        .map_err(|e| format!("embedding error: {e}"))?;
                        return Ok((t, input));
                    }
                    return Ok((t, input));
                }
                Err(e) if e.starts_with("exhausted ") && supply == 256 => continue,
                Err(e) => return Err(format!("embedding error: {e}")),
            }
        }
        Err("embedding error: evidence exhausted after retry".into())
    }

    fn persist(
        &self,
        writer: &Writer,
        stored: &mut Memory,
        transition: &Transition,
    ) -> std::result::Result<(usize, Duration), PersistenceError> {
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true);
        stored.output_times.retain(|id, _| {
            transition
                .state
                .invocations
                .iter()
                .any(|v| v["id"] == *id && retained_results(v).next().is_some())
        });
        for v in &transition.state.invocations {
            let id = v["id"].as_str().ok_or("invocation has no ID")?;
            if retained_results(v).next().is_some() {
                // A new successful completion starts an age. Moving an existing result
                // into previous, or restoring it after a refusal, preserves that age.
                let completed = transition.request_replies.iter().any(|r| {
                    r.invocation == id
                        && r.reply["_meta"][format!("{}outcome", self.context.meta_prefix)]
                            == "succeeded"
                });
                if completed || !stored.output_times.contains_key(id) {
                    stored.output_times.insert(id.into(), now.clone());
                }
            }
        }
        // DL2/REC-003/OPS-013: persist records and outcomes before advancing memory.
        let started = Instant::now();
        let bytes = writer
            .store
            .write_state(&transition.state, &stored.output_times)
            .map_err(PersistenceError::Write)?;
        Ok((bytes, started.elapsed()))
    }

    fn retention(
        &self,
        writer: &Writer,
        stored: &mut Memory,
        measures: &mut CommitMeasures,
    ) -> std::result::Result<(), PersistenceError> {
        let now = Utc::now();
        let mut outputs = Vec::new();
        for v in &stored.state.invocations {
            if retained_results(v).next().is_some() {
                let id = v["id"].as_str().ok_or("invocation has no ID")?;
                let time = stored
                    .output_times
                    .get(id)
                    .ok_or("retained output has no retention timestamp")?;
                let instant = DateTime::parse_from_rfc3339(time)
                    .map_err(|_| "invalid output retention timestamp")?;
                outputs.push((
                    instant,
                    id.to_string(),
                    retained_results(v).try_fold(0, |size, retained| {
                        serde_json::to_vec(retained)
                            .map(|bytes| size + bytes.len())
                            .map_err(|e| e.to_string())
                    })?,
                ));
            }
        }
        outputs.sort();
        let mut total: usize = outputs.iter().map(|v| v.2).sum();
        for (time, id, size) in outputs {
            if total > self.config.max_output_bytes
                || now
                    .signed_duration_since(time)
                    .to_std()
                    .is_ok_and(|age| age > self.config.max_output_age)
            {
                // DL10: only expiry writes; both retention checks use the in-memory commit.
                let state = Arc::unwrap_or_clone(std::mem::take(&mut stored.state));
                let input = json!({"kind":"expire-outputs","invocation":id});
                let started = Instant::now();
                let t = self.transition(state, &writer.store, input)?;
                measures.step_time += started.elapsed();
                measures.written(self.persist(writer, stored, &t)?);
                stored.state = Arc::new(t.state);
                total -= size;
            }
        }
        Ok(())
    }

    fn commit(&self, input: Value) -> Result<Committed> {
        // The runner commits discovery under its session lock, before driving the
        // result. Other low-level commits preserve raw embedding errors.
        self.commit_input(input, false)
    }

    fn retry_held(
        &self,
        writer: &mut Writer,
        stored: &mut Memory,
        measures: &mut CommitMeasures,
    ) -> std::result::Result<(), PersistenceError> {
        while let Some(input) = writer.held_records.front() {
            let state = Arc::unwrap_or_clone(std::mem::take(&mut stored.state));
            let started = Instant::now();
            let t = input.step(state, &self.context)?;
            measures.step_time += started.elapsed();
            match self.persist(writer, stored, &t) {
                Ok(written) => {
                    measures.written(written);
                    writer.held_records.pop_front();
                    stored.state = Arc::new(t.state);
                }
                Err(PersistenceError::Write(error)) => {
                    if error.renamed {
                        writer.held_records.pop_front();
                    }
                    return Err(PersistenceError::Write(error));
                }
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }

    fn commit_input(&self, input: Value, fail_closed: bool) -> Result<Committed> {
        let mut writer = self.writer.lock().map_err(|_| "writer lock poisoned")?;
        let started = Instant::now();
        let kind = input["kind"].as_str().unwrap_or("").to_string();
        let mut read_value = (kind == "complete")
            .then(|| {
                Some((
                    input["invocation"].as_str()?.to_owned(),
                    input["completion"]["result"]["structuredContent"]["value"]
                        .as_str()?
                        .to_owned(),
                ))
            })
            .flatten();
        let mut measures = CommitMeasures::default();
        // DL2: taking memory leaves the cache empty on every failed step/write.
        let mut stored = writer.store.take()?;
        let previous_asks: BTreeSet<_> = stored
            .state
            .admission_asks
            .values()
            .map(|ask| ask.invocation.clone())
            .collect();
        let discovery = !fail_closed
            && kind == "complete"
            && stored.state.invocations.iter().any(|v| {
                v["id"] == input["invocation"]
                    && v["attempt"] == input["attempt"]
                    && (v["discovery"] == "pending" || v["secret_read"]["phase"] == "discover")
            });
        let covered = (fail_closed || discovery) && RecordFault::covers(&kind);
        // IFC-038: notices read objects only. The State is copied only for a held reader.
        let before = State {
            objects: stored.state.objects.clone(),
            ..State::default()
        };
        let earlier = self
            .retry_held(&mut writer, &mut stored, &mut measures)
            .and_then(|()| {
                if kind == "recover" {
                    Ok(())
                } else {
                    self.retention(&writer, &mut stored, &mut measures)
                }
            });
        let (state, prewrite_failed) = match earlier {
            Ok(()) => (
                Arc::unwrap_or_clone(std::mem::take(&mut stored.state)),
                false,
            ),
            Err(PersistenceError::Write(_)) if covered => {
                stored = writer.store.take()?;
                (
                    self.replay_held(
                        Arc::unwrap_or_clone(std::mem::take(&mut stored.state)),
                        &writer.held_records,
                    )?,
                    true,
                )
            }
            Err(error) => return Err(error.into()),
        };
        let step_started = Instant::now();
        let (mut t, input) =
            self.transition_input(state, &writer.store, input, &writer.held_records)?;
        measures.step_time += step_started.elapsed();
        let failed = if prewrite_failed {
            Some(false)
        } else {
            let persisted = (|| {
                if let Some(installed) = t.decisions.iter().find(|d| d.action == "install-archive")
                {
                    let digest = installed.facts["digest"]
                        .as_str()
                        .ok_or("installed archive has no digest")?;
                    let bytes: Vec<u8> =
                        serde::Deserialize::deserialize(&input["archive"]["completion"]["bytes"])
                            .map_err(|e| format!("installed archive evidence: {e}"))?;
                    writer
                        .store
                        .keep_archive(digest, &bytes)
                        .map_err(|message| {
                            PersistenceError::Write(StateWriteError {
                                renamed: false,
                                message,
                            })
                        })?;
                }
                self.persist(&writer, &mut stored, &t)
            })();
            match persisted {
                Ok(written) => {
                    measures.written(written);
                    None
                }
                Err(PersistenceError::Write(error)) if covered => Some(error.renamed),
                Err(error) => return Err(error.into()),
            }
        };
        let mut kept_fault = false;
        if let Some(renamed) = failed {
            let input =
                serde_json::from_value(input).map_err(|e| format!("embedding input: {e}"))?;
            let fault = RecordFault::from_trial(input, &t, renamed)?;
            drop(t);
            stored = writer.store.take()?;
            let state = self.replay_held(
                Arc::unwrap_or_clone(std::mem::take(&mut stored.state)),
                &writer.held_records,
            )?;
            let step_started = Instant::now();
            t = fault.step(state, &self.context)?;
            measures.step_time += step_started.elapsed();
            match self.persist(&writer, &mut stored, &t) {
                Ok(written) => {
                    measures.written(written);
                    writer.held_records.clear();
                    kept_fault = true;
                }
                Err(PersistenceError::Write(error)) => {
                    if error.renamed {
                        writer.held_records.clear();
                    } else {
                        writer.held_records.push_back(fault);
                    }
                    stored = writer.store.take()?;
                }
                Err(error) => return Err(error.into()),
            }
            read_value = None;
        }
        // PRV-029/LIF-035: reconcile only after removal is durable. A directory
        // whose delete fails remains the retry marker, including across restart.
        if failed.is_none() {
            writer.store.clean_state_directories(&t.state)?;
        }
        #[cfg(test)]
        if let Some(trace) = self
            .trace
            .lock()
            .map_err(|_| "trace lock poisoned")?
            .as_mut()
        {
            // DL2: retain a transition only when its test explicitly requests a trace.
            trace.push(t.clone());
        }
        if let Some((id, value)) = read_value
            && t.records.iter().any(|r| {
                r["type"] == "request-ended"
                    && r["data"]["invocation"] == id
                    && r["data"]["operation"]
                        == format!("{}#get-secret.v1", self.context.core_package)
                    && r["data"]["outcome"] == "succeeded"
            })
        {
            writer.read_values.insert(id, value);
        }
        let intents = std::mem::take(&mut t.external_intents)
            .into_iter()
            .map(|descriptor| {
                let secrets = if let Some(bindings) = descriptor.get("secretBindings") {
                    let mut secrets = bindings.clone();
                    let mut used = BTreeSet::new();
                    for map in secrets.as_object_mut().unwrap().values_mut() {
                        for value in map.as_object_mut().unwrap().values_mut() {
                            let id = value.as_str().ok_or("missing secret read ID")?;
                            used.insert(id.to_owned());
                            *value = json!(
                                writer
                                    .read_values
                                    .get(id)
                                    .ok_or("secret read value unavailable")?
                            );
                        }
                    }
                    for id in used {
                        writer.read_values.remove(&id);
                    }
                    Some(secrets)
                } else {
                    None
                };
                Ok(Intent {
                    descriptor,
                    secrets,
                    from_record_fault: failed.is_some(),
                })
            })
            .collect::<Result<_>>()?;
        writer.read_values.retain(|id, _| {
            t.state.invocations.iter().any(|v| {
                v["state"] == "in-progress"
                    && v["secret_reads"]["reads"]
                        .as_object()
                        .is_some_and(|reads| reads.values().any(|read| read == id.as_str()))
            })
        });
        // Publish replies only after both records and invocation state are durable.
        for ended in &t.request_replies {
            if let Some(cells) = writer
                .waiters
                .remove(&(ended.invocation.clone(), ended.attempt))
            {
                for cell in cells.iter().filter_map(Weak::upgrade) {
                    let mut value = cell.value.lock().map_err(|_| "reply lock poisoned")?;
                    *value = Some(ended.reply.clone());
                    cell.changed.notify_all();
                }
            }
        }
        writer.waiters.retain(|_, cells| {
            cells.retain(|cell| cell.strong_count() > 0);
            !cells.is_empty()
        });
        let cell = Arc::new(ReplyCell::default());
        let invocation = if t.reply["pending"] == true {
            let id = t.reply["invocation"]
                .as_str()
                .ok_or("pending reply has no invocation")?
                .to_string();
            let attempt = t
                .state
                .invocations
                .iter()
                .find(|v| v["id"] == id)
                .and_then(|v| v["attempt"].as_u64())
                .ok_or("pending reply has no request attempt")?;
            writer
                .waiters
                .entry((id.clone(), attempt))
                .or_default()
                .push(Arc::downgrade(&cell));
            Some(id)
        } else {
            *cell.value.lock().map_err(|_| "reply lock poisoned")? = Some(t.reply.clone());
            None
        };
        for input in std::mem::take(&mut t.input_required) {
            let attempt = input.attempt;
            if let Some(cells) = writer.waiters.remove(&(input.invocation.clone(), attempt)) {
                let value = serde_json::to_value(input).map_err(|e| e.to_string())?;
                let reply = json!({"inputRequired": value});
                for cell in cells.iter().filter_map(Weak::upgrade) {
                    let mut value = cell.value.lock().map_err(|_| "reply lock poisoned")?;
                    *value = Some(reply.clone());
                    cell.changed.notify_all();
                }
            }
        }
        // DL10/IFC-038: keep the second retention check before notices. Expiry
        // changes only retained outputs, so the committed objects are still t's.
        if failed.is_none() || kept_fault {
            stored.state = Arc::new(std::mem::take(&mut t.state));
        }
        if failed.is_none() {
            self.retention(&writer, &mut stored, &mut measures)?;
        }
        let mut notifications =
            conduit_core::access::tool_list_changes(&before, &stored.state, &self.context)?;
        for o in before.objects.iter().filter(|o| {
            o.get("spec").is_none() && (o["kind"] == "Agent" || o["kind"] == "Interface")
        }) {
            if !stored.state.objects.iter().any(|n| n["id"] == o["id"]) {
                notifications.push(json!({"consumer":o["id"],"method":"consumer-ended"}));
            }
        }
        // Only committed state starts timers, including asks born in completions.
        let admission_asks = stored.state.admission_asks.values()
            .filter(|ask| !previous_asks.contains(&ask.invocation))
            .filter_map(|ask| {
                let invocation = stored.state.invocations.iter().find(|v| v["id"] == ask.invocation && v["state"] == "in-progress")?;
                Some(json!({"invocation":ask.invocation,"attempt":invocation["attempt"],"startedAt":ask.started_at}))
            }).collect();
        let invocations = stored.state.invocations.len();
        let fault_endings = failed.map(|_| {
            t.records
                .iter()
                .filter(|r| r["type"] == "request-ended")
                .filter_map(|r| r["data"]["invocation"].as_str().map(str::to_owned))
                .chain(t.request_replies.iter().map(|r| r.invocation.clone()))
                .collect()
        });
        writer.store.remember(stored);
        let commit_micros = micros(started.elapsed());
        // These optional measures cannot turn a durable commit into a failure.
        let mut line = json!({
            "at": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            "input": kind,
            "stateBytes": measures.state_bytes,
            "invocations": invocations,
            "writes": measures.writes,
            "stepMicros": micros(measures.step_time),
            "writeMicros": micros(measures.write_time),
            "commitMicros": commit_micros,
        });
        if let Some(open_micros) = writer.open_micros.take() {
            line["openMicros"] = json!(open_micros);
        }
        if writer.unwritten > 0 {
            line["unwritten"] = json!(writer.unwritten);
        }
        let mut bytes = line.to_string().into_bytes();
        bytes.push(b'\n');
        match store::append_commit_measures(&writer.store.directory, &bytes) {
            Ok(()) => writer.unwritten = 0,
            Err(error) => {
                let _ = writeln!(
                    std::io::stderr(),
                    "conduit: the measures line of a commit of kind {kind} was not appended: {error}"
                );
                writer.unwritten = writer.unwritten.saturating_add(1);
            }
        }
        Ok(Committed {
            ticket: Ticket { invocation, cell },
            intents,
            notifications,
            fault_endings,
            admission_asks,
        })
    }

    fn drive(
        &self,
        intents: Vec<Intent>,
        notifications: Vec<Value>,
        executor: &dyn Executor,
    ) -> Result<()> {
        let from_record_fault = intents.iter().any(|intent| intent.from_record_fault);
        // A discovery commit never starts an admission ask, so this path hands off
        // no asks. A future discovery change that starts asks must carry them here
        // so admission_asks_started installs their DL21 timers.
        self.drive_committed(intents, notifications, vec![], from_record_fault, executor)
    }

    fn complete_run_archive(&self, intent: &mut Value) -> Result<Option<Execution>> {
        let writer = self.writer.lock().map_err(|_| "writer lock poisoned")?;
        let state = writer.store.snapshot()?;
        let Some(invocation) = state.invocations.iter().find(|invocation| {
            invocation["id"] == intent["invocation"]
                && invocation["state"] == "in-progress"
                && invocation["cancelled"] != true
                && invocation["provider_call"]["operation"] == intent["operation"]
                && invocation["provider_call"]["made"] != true
        }) else {
            return Ok(Some(Execution::Deferred));
        };
        let digest = intent["inputs"]["archive"]["digest"]
            .as_str()
            .ok_or("run.v2 archive has no digest")?;
        match writer.store.archive_descriptor(digest) {
            Ok(archive) => {
                intent["inputs"]["archive"] = archive;
                Ok(None)
            }
            Err(error) => {
                let package = state
                    .installed_packages
                    .iter()
                    .find(|entry| entry.package.digest == digest)
                    .ok_or("run.v2 package is not installed")?;
                let item = invocation["setup"]["item"]["selection"]
                    .as_str()
                    .and_then(|selection| selection.rsplit_once('#'))
                    .map(|(_, name)| name)
                    .ok_or("run.v2 item missing")?;
                let provider = state
                    .objects
                    .iter()
                    .find(|object| object["id"] == intent["provider"])
                    .and_then(|object| object["name"].as_str())
                    .ok_or("run.v2 provider missing")?;
                let message = format!(
                    "{item} on provider {provider}: run.v2: package {} {}: {error}",
                    package.package.address, package.package.version
                );
                Ok(Some(Execution::SetupReport(
                    json!({"kind":"provider-run-refused","operation":intent["operation"],"message":message}),
                )))
            }
        }
    }

    fn drive_committed(
        &self,
        intents: Vec<Intent>,
        notifications: Vec<Value>,
        admission_asks: Vec<Value>,
        from_record_fault: bool,
        executor: &dyn Executor,
    ) -> Result<()> {
        let notices = executor.tools_changed(&notifications);
        let timers = executor.admission_asks_started(&admission_asks);
        if !from_record_fault {
            notices?;
            timers?;
        }
        let mut queue = VecDeque::from(intents);
        while let Some(Intent {
            descriptor: mut intent,
            secrets,
            from_record_fault,
        }) = queue.pop_front()
        {
            let preflight = if intent["kind"] == "provider-call"
                && intent["operation"] == format!("{}#run.v2", self.context.core_package)
            {
                self.complete_run_archive(&mut intent)?
            } else {
                None
            };
            let kind = intent["kind"].as_str().ok_or("intent has no kind")?;
            let run = preflight.is_none()
                && matches!(kind, "builtin-runner" | "provider-call")
                && intent["operation"]
                    .as_str()
                    .is_some_and(|op| self.context.is_run_operation(op));
            let identity = if run && kind == "provider-call" {
                self.materialize_provider_identity(&intent["inputs"]["identity"])?
            } else if run {
                Some(self.materialize_identity(&intent["inputs"]["identity"])?)
            } else {
                None
            };
            let execution = match (preflight, kind) {
                (Some(execution), _) => execution,
                (None, "dispatch") => executor.dispatch(&intent)?,
                (None, "refresh-tools-and-call") => executor.refresh_tools_and_call(&intent)?,
                (None, "builtin-runner") => {
                    if let Some(secrets) = secrets {
                        let mut run = intent.clone();
                        run["inputs"]["secrets"] = secrets;
                        executor.builtin_runner(&run, identity.as_ref())?
                    } else {
                        executor.builtin_runner(&intent, identity.as_ref())?
                    }
                }
                (None, "provider-call") => executor.provider_call(&intent, identity.as_ref())?,
                (None, "connect") => executor.connect(&intent)?,
                (None, "cancel" | "cancel-setup-parts") => {
                    let result = if kind == "cancel" {
                        executor.cancel(&intent)
                    } else {
                        executor.cancel_setup_parts(&intent)
                    };
                    if from_record_fault {
                        // The result was already decided by REC-007. Cancellation
                        // is a notification and cannot replace that result.
                        continue;
                    }
                    result?
                }
                _ => return Err("embedding error: unsupported intent kind".into()),
            };
            // A built-in run takes the directory with the identity. Otherwise the
            // call owns it only until returning, including error/missing paths.
            drop(identity);
            let execution = if matches!(execution, Execution::Missing) {
                missing(&intent, &self.context)?
            } else {
                execution
            };
            let input = match execution {
                Execution::Complete(completion)
                    if matches!(kind, "dispatch" | "refresh-tools-and-call") =>
                {
                    json!({"kind":"complete","invocation":intent["invocation"],"attempt":intent["attempt"],"completion":completion})
                }
                Execution::SetupReport(report)
                    if matches!(kind, "builtin-runner" | "provider-call" | "connect") =>
                {
                    json!({"kind":"setup-report","invocation":intent["invocation"],"report":report})
                }
                Execution::Deferred => continue,
                Execution::Done if matches!(kind, "cancel" | "cancel-setup-parts") => continue,
                _ => return Err("embedding error: executor response does not match intent".into()),
            };
            let next = self.commit_input(input, true)?;
            let notices = executor.tools_changed(&next.notifications);
            let timers = executor.admission_asks_started(&next.admission_asks);
            if let Some(ended) = &next.fault_endings {
                queue.retain(|queued| {
                    matches!(
                        queued.descriptor["kind"].as_str(),
                        Some("cancel" | "cancel-setup-parts")
                    ) || !queued.descriptor["invocation"]
                        .as_str()
                        .is_some_and(|id| ended.contains(id))
                });
            } else {
                notices?;
                timers?;
            }
            queue.extend(next.intents);
        }
        Ok(())
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        if let Ok(groups) = self.run_groups.lock() {
            for group in groups.iter() {
                group.stop();
            }
        }
    }
}

// The core may hold an earlier completed invocation inside a pending repeat.
fn retained_results(invocation: &Value) -> impl Iterator<Item = &Value> {
    std::iter::successors(Some(invocation), |v| v.get("previous")).filter_map(|v| v.get("retained"))
}

fn missing(intent: &Value, context: &Context) -> Result<Execution> {
    Ok(match intent["kind"].as_str() {
        Some("dispatch" | "refresh-tools-and-call") => {
            Execution::Complete(json!({"kind":"missing"}))
        }
        Some("connect") => Execution::SetupReport(
            json!({"kind":"connected","part":intent["part"],"outcome":"failed"}),
        ),
        Some("builtin-runner" | "provider-call")
            if intent["operation"] == format!("{}#inspect-run.v1", context.core_package) =>
        {
            Execution::SetupReport(json!({"kind":"inspect-run","state":"unknown"}))
        }
        Some("builtin-runner" | "provider-call")
            if intent["operation"]
                .as_str()
                .is_some_and(|op| context.is_run_operation(op)) =>
        {
            Execution::SetupReport(
                json!({"kind":"run","operation":intent["operation"],"outcome":"failed"}),
            )
        }
        Some("builtin-runner" | "provider-call")
            if intent["operation"] == format!("{}#stop.v1", context.core_package) =>
        {
            Execution::SetupReport(json!({"kind":"stop","outcome":"unknown"}))
        }
        Some("cancel" | "cancel-setup-parts") => Execution::Done,
        _ => return Err("embedding error: no missing-executor report for intent".into()),
    })
}

fn evidence(kind: &str, time: &str, supply: usize) -> Result<Value> {
    let revisions = if kind == "first-start" { 8 } else { supply };
    let suffixes = if kind == "first-start" { 3 } else { supply };
    Ok(
        json!({"time":time,"ids":(0..supply).map(|_|Uuid::new_v4().to_string()).collect::<Vec<_>>(),
        "revisions":(0..revisions).map(|_|Uuid::new_v4().to_string()).collect::<Vec<_>>(),
        "suffixes":(0..suffixes).map(|_|identity::random_text(5)).collect::<Result<Vec<_>>>()?}),
    )
}

pub fn local_machine(mut labels: BTreeMap<String, String>) -> Result<Value> {
    let hostname = hostname::get()
        .map_err(|e| e.to_string())?
        .to_str()
        .ok_or("hostname is not UTF-8")?
        .trim()
        .to_lowercase();
    labels.insert("kubernetes.io/hostname".into(), hostname.clone());
    let os = match std::env::consts::OS {
        "macos" => "darwin",
        os => os,
    };
    let arch = match std::env::consts::ARCH {
        "aarch64" => "arm64",
        "x86_64" => "amd64",
        "x86" => "386",
        "powerpc" => "ppc",
        "powerpc64" if cfg!(target_endian = "little") => "ppc64le",
        "powerpc64" => "ppc64",
        arch => arch,
    };
    labels.insert("kubernetes.io/os".into(), os.into());
    labels.insert("kubernetes.io/arch".into(), arch.into());
    Ok(json!({"hostname":hostname,"labels":labels,"state":"reachable"}))
}

#[cfg(test)]
mod tests;
