//! Durable boundary for the pure core, with an optional owned stdio runner.
pub mod author;
pub mod boundary;
mod identity;
pub mod runner;
pub mod server;
mod store;
use chrono::{DateTime, SecondsFormat, Utc};
use conduit_core::{Context, Input, State, Transition};
pub use identity::RunIdentity;
use serde_json::{Value, json};
use std::collections::{BTreeMap, VecDeque};
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex, Weak};
use std::time::Duration;
use store::{Store, Stored};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone)]
pub struct Config {
    pub max_output_bytes: usize,
    pub max_output_age: Duration,
    pub engine_url: String,
    pub pass_environment: Vec<String>,
    pub initialize_timeout: Duration,
    pub call_timeout: Duration,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            max_output_bytes: 64 * 1024 * 1024,
            max_output_age: Duration::from_secs(7 * 24 * 60 * 60),
            engine_url: "http://127.0.0.1:3000/mcp".into(),
            pass_environment: Vec::new(),
            initialize_timeout: Duration::from_secs(30),
            call_timeout: Duration::from_secs(120),
        }
    }
}
impl Config {
    /// Clients read only the public endpoint, without opening/recovering a live store.
    pub fn for_store(directory: &Path) -> Result<Self> {
        let path = directory.join("engine-url.json");
        let mut config = Self::default();
        if path.try_exists().map_err(|e| e.to_string())? {
            config.engine_url = serde_json::from_slice(&store::private_read(&path)?)
                .map_err(|_| "invalid stored engine URL")?;
        }
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
}
struct Committed {
    ticket: Ticket,
    intents: Vec<Value>,
    notifications: Vec<Value>,
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
    trace: Mutex<Vec<Transition>>,
}

impl Shell {
    pub fn open(directory: &Path, context: Context, config: Config) -> Result<Self> {
        let url = url::Url::parse(&config.engine_url).map_err(|_| "invalid engine URL")?;
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
        let store = Store::open(directory)?;
        let address = store.directory.join("engine-url.json");
        if address.try_exists().map_err(|e| e.to_string())? {
            let fixed: String = serde_json::from_slice(&store::private_read(&address)?)
                .map_err(|_| "invalid stored engine URL")?;
            if fixed != config.engine_url {
                return Err("engine address differs from the store's fixed token audience".into());
            }
        } else {
            store::atomic_write(
                &store.directory,
                "engine-url.json",
                &serde_json::to_vec(&config.engine_url).map_err(|e| e.to_string())?,
            )?;
        }
        let authority = identity::Authority::open(&store)?;
        let shell = Self {
            directory: store.directory.clone(),
            run_groups: Mutex::new(Vec::new()),
            writer: Mutex::new(Writer {
                store,
                waiters: BTreeMap::new(),
            }),
            authority,
            context,
            config,
            #[cfg(test)]
            evidence: Mutex::new(VecDeque::new()),
            #[cfg(test)]
            trace: Mutex::new(Vec::new()),
        };
        // An open is a restart boundary, even for an empty store. No intent is replayed.
        let recovered = shell.commit(json!({"kind":"recover"}))?;
        if !recovered.intents.is_empty() {
            return Err("recovery unexpectedly requested external work".into());
        }
        Ok(shell)
    }

    pub fn state(&self) -> Result<State> {
        self.writer
            .lock()
            .map_err(|_| "writer lock poisoned")?
            .store
            .read()
            .map(|s| s.state)
    }

    /// PRV-029: stable, private storage, separate from each run's working directory.
    pub(crate) fn state_directory(&self, instance: &str) -> Result<std::path::PathBuf> {
        let writer = self.writer.lock().map_err(|_| "writer lock poisoned")?;
        let state = writer.store.read()?.state;
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
        let committed = self.commit(input)?;
        self.drive(committed.intents, committed.notifications, executor)?;
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
        Ok(self
            .authority
            .verify(token, &self.config.engine_url, &self.state()?))
    }
    pub fn materialize_identity(&self, descriptor: &Value) -> Result<RunIdentity> {
        let writer = self.writer.lock().map_err(|_| "writer lock poisoned")?;
        let identity = self.authority.materialize(
            &writer.store.directory,
            descriptor,
            &self.config.engine_url,
            &writer.store.read()?.state,
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

    fn client_token(&self, writer: &Writer, instance: &str) -> Result<()> {
        let state = writer.store.read()?.state;
        if !state.objects.iter().any(|o| {
            o["id"] == instance
                && o.get("spec").is_none()
                && ["Interface", "Agent", "Resource", "Pool"]
                    .iter()
                    .any(|k| o["kind"] == *k)
        }) {
            return Err("token subject is not an instance".into());
        }
        let directory = writer.store.directory.join("client-tokens");
        store::private_directory(&directory)?;
        let token = self.authority.token(instance, &self.config.engine_url)?;
        store::atomic_write(&directory, instance, token.as_bytes())
    }

    pub fn issue_client_token(&self, instance: &str) -> Result<()> {
        let writer = self.writer.lock().map_err(|_| "writer lock poisoned")?;
        self.client_token(&writer, instance)
    }

    fn transition(&self, state: &State, mut input: Value) -> Result<Transition> {
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
                | "cancel"
                | "setup-report"
                | "recover"
                | "builtin-run-stopped"
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
            match conduit_core::step(state, parsed, &self.context) {
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
                        return conduit_core::step(state, parsed, &self.context)
                            .map_err(|e| format!("embedding error: {e}"));
                    }
                    return Ok(t);
                }
                Err(e) if e.starts_with("exhausted ") && supply == 256 => continue,
                Err(e) => return Err(format!("embedding error: {e}")),
            }
        }
        Err("embedding error: evidence exhausted after retry".into())
    }

    fn persist(&self, writer: &Writer, stored: &mut Stored, transition: &Transition) -> Result<()> {
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
        stored.state = transition.state.clone();
        writer.store.write(stored)
    }

    fn retention(&self, writer: &Writer, stored: &mut Stored) -> Result<()> {
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
                let t = self.transition(
                    &stored.state,
                    json!({"kind":"expire-outputs","invocation":id}),
                )?;
                self.persist(writer, stored, &t)?;
                total -= size;
            }
        }
        Ok(())
    }

    fn commit(&self, input: Value) -> Result<Committed> {
        let mut writer = self.writer.lock().map_err(|_| "writer lock poisoned")?;
        let mut stored = writer.store.read()?;
        if input["kind"] != "recover" {
            self.retention(&writer, &mut stored)?;
        }
        let before = stored.state.clone();
        let t = self.transition(&before, input)?;
        self.persist(&writer, &mut stored, &t)?;
        // PRV-029/LIF-035: reconcile only after removal is durable. A directory
        // whose delete fails remains the retry marker, including across restart.
        writer.store.clean_state_directories(&stored.state)?;
        #[cfg(test)]
        self.trace
            .lock()
            .map_err(|_| "trace lock poisoned")?
            .push(t.clone());
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
        self.retention(&writer, &mut stored)?;
        let mut notifications =
            conduit_core::access::tool_list_changes(&before, &t.state, &self.context)?;
        for o in before.objects.iter().filter(|o| {
            o.get("spec").is_none() && (o["kind"] == "Agent" || o["kind"] == "Interface")
        }) {
            if !t.state.objects.iter().any(|n| n["id"] == o["id"]) {
                notifications.push(json!({"consumer":o["id"],"method":"consumer-ended"}));
            }
        }
        Ok(Committed {
            ticket: Ticket { invocation, cell },
            intents: t.external_intents,
            notifications,
        })
    }

    fn drive(
        &self,
        intents: Vec<Value>,
        notifications: Vec<Value>,
        executor: &dyn Executor,
    ) -> Result<()> {
        executor.tools_changed(&notifications)?;
        let mut queue = VecDeque::from(intents);
        while let Some(intent) = queue.pop_front() {
            let kind = intent["kind"].as_str().ok_or("intent has no kind")?;
            let run = matches!(kind, "builtin-runner" | "provider-call")
                && intent["operation"] == format!("{}#run.v1", self.context.core_package);
            let identity = if run {
                Some(self.materialize_identity(&intent["inputs"]["identity"])?)
            } else {
                None
            };
            let execution = match kind {
                "dispatch" => executor.dispatch(&intent)?,
                "refresh-tools-and-call" => executor.refresh_tools_and_call(&intent)?,
                "builtin-runner" => executor.builtin_runner(&intent, identity.as_ref())?,
                "provider-call" => executor.provider_call(&intent, identity.as_ref())?,
                "connect" => executor.connect(&intent)?,
                "cancel" => executor.cancel(&intent)?,
                "cancel-setup-parts" => executor.cancel_setup_parts(&intent)?,
                _ => return Err("embedding error: unsupported intent kind".into()),
            };
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
            let next = self.commit(input)?;
            executor.tools_changed(&next.notifications)?;
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
            if intent["operation"] == format!("{}#run.v1", context.core_package) =>
        {
            Execution::SetupReport(json!({"kind":"run","outcome":"failed"}))
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
    let revisions = if kind == "first-start" { 7 } else { supply };
    let suffixes = if kind == "first-start" { 2 } else { supply };
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
