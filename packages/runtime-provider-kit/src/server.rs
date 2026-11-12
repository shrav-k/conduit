use crate::{backend::*, channel::Observed, names, schemas};
use rmcp::{ErrorData, RoleServer, ServerHandler, ServiceExt, model::*, service::RequestContext};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::{Mutex as AsyncMutex, RwLock};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Copy)]
pub struct Options {
    pub max_lifetime: Duration,
    pub drain_interval: Duration,
}
struct Flight {
    epoch: u64,
    serial: u64,
    cancel: CancellationToken,
}
struct Slot {
    operation: AsyncMutex<()>,
    flight: Mutex<Flight>,
    gate: AsyncMutex<Option<Arc<dyn GateHandle>>>,
    // Cleanup must not depend on a readable ledger. Keep only nonsensitive recovery context,
    // including historical machines, before handing any staged work to the backend.
    contexts: Mutex<BTreeMap<String, CallContext>>,
    // Include durable and in-progress intents so an older ledger snapshot cannot miss them.
    held_stops: Mutex<BTreeMap<String, RunRecord>>,
}
impl Default for Slot {
    fn default() -> Self {
        Self {
            operation: AsyncMutex::new(()),
            flight: Mutex::new(Flight {
                epoch: 0,
                serial: 0,
                cancel: CancellationToken::new(),
            }),
            gate: AsyncMutex::new(None),
            contexts: Mutex::new(BTreeMap::new()),
            held_stops: Mutex::new(BTreeMap::new()),
        }
    }
}
impl Slot {
    fn remember(&self, record: &RunRecord) {
        self.contexts.lock().unwrap().insert(
            record.context.machine.hostname.clone(),
            record.context.clone(),
        );
    }
    fn refresh(&self, record: &mut RunRecord, call: Option<&CallContext>) {
        // Saved records must never replace a context refreshed by a live call. The lookup, the
        // update and the insert happen under one hold of the mutex: released in between, a
        // refresh without a call could copy an older context, let a live call install a newer
        // one, and then write the older one back.
        let mut contexts = self.contexts.lock().unwrap();
        if let Some(held) = contexts.get(&record.context.machine.hostname) {
            record.context = held.clone();
        }
        if let Some(call) = call {
            record.context.config = call.config.clone();
            record.context.revision = call.revision.clone();
            if record.context.machine.hostname == call.machine.hostname {
                record.context.machine = call.machine.clone();
            }
        }
        contexts.insert(
            record.context.machine.hostname.clone(),
            record.context.clone(),
        );
    }
    fn pending(&self) -> Vec<RunRecord> {
        self.held_stops.lock().unwrap().values().cloned().collect()
    }
    fn records(&self, instance: &str) -> Vec<RunRecord> {
        let contexts = self
            .contexts
            .lock()
            .unwrap()
            .values()
            .map(|context| candidate(instance, context))
            .collect();
        combine(self.pending(), contexts)
    }
    fn cancel(&self) {
        let mut flight = self.flight.lock().unwrap();
        flight.epoch += 1;
        flight.serial += 1;
        flight.cancel.cancel();
    }
}
struct Inner<B, S, G> {
    backend: Arc<B>,
    store: Arc<S>,
    gate: Arc<G>,
    options: Options,
    slots: Mutex<BTreeMap<String, Arc<Slot>>>,
    admissions: Mutex<BTreeMap<String, Arc<RwLock<()>>>>,
    shutdown: CancellationToken,
}
pub struct RuntimeServer<B, S, G>(Arc<Inner<B, S, G>>);
struct Handler<B, S, G>(RuntimeServer<B, S, G>);
impl<B, S, G> Clone for RuntimeServer<B, S, G> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// Independent of MCP and instance registration. The caller must have exclusive ownership of
/// these backend/store runs (no simultaneous server or second drainer).
pub async fn drain_owed_stops<B: RuntimeBackend, S: StateStore>(
    backend: &B,
    store: &S,
) -> Result<usize> {
    let mut stopped = 0;
    for record in store
        .load()
        .await
        .map_err(|_| Unavailable)?
        .into_iter()
        .filter(|r| r.owed_stop)
    {
        if stop_record(backend, store, record).await.is_ok() {
            stopped += 1;
        }
    }
    Ok(stopped)
}
async fn stop_record<B: RuntimeBackend, S: StateStore>(
    backend: &B,
    store: &S,
    mut record: RunRecord,
) -> Result<()> {
    stop_attempt(backend, store, &mut record, None).await.result
}
struct StopAttempt {
    result: Result<()>,
    storage_failed: bool,
}
async fn stop_attempt<B: RuntimeBackend, S: StateStore>(
    backend: &B,
    store: &S,
    record: &mut RunRecord,
    coordination: Option<(&Slot, &RwLock<()>)>,
) -> StopAttempt {
    // Publish before awaiting storage/backend work. Admission checks under the same
    // machine lock, so either the run is admitted first or it sees this obligation.
    record.owed_stop = true;
    record.state = RunState::Unknown;
    if let Some((slot, admission)) = coordination {
        let _guard = admission.write().await;
        slot.held_stops
            .lock()
            .unwrap()
            .insert(record.context.machine.hostname.clone(), record.clone());
    }
    // Even on a store failure, try to stop the tree.
    let intent = store.save(record).await;
    let stopped = backend.stop(&record.context, &record.instance).await;
    let mut storage_failed = intent.is_err();
    let result = if intent.is_ok() && stopped.is_ok() {
        record.state = RunState::Stopped;
        record.owed_stop = false;
        let saved = store.save(record).await;
        storage_failed |= saved.is_err();
        saved.map_err(|_| Unavailable)
    } else {
        Err(Unavailable)
    };
    if (result.is_ok() || (intent.is_err() && stopped.is_ok()))
        && let Some((slot, _)) = coordination
    {
        slot.held_stops
            .lock()
            .unwrap()
            .remove(&record.context.machine.hostname);
    }
    StopAttempt {
        result,
        storage_failed,
    }
}

fn candidate(instance: &str, context: &CallContext) -> RunRecord {
    RunRecord {
        instance: instance.into(),
        context: context.clone(),
        state: RunState::Unknown,
        owed_stop: false,
    }
}
fn combine(mut records: Vec<RunRecord>, more: Vec<RunRecord>) -> Vec<RunRecord> {
    for record in more {
        if !records.iter().any(|r| r.same_run(&record)) {
            records.push(record);
        }
    }
    records
}
// Order 081 settles the repeated-run answer. Unknown starts nothing; running reuses the gate.
fn repeated_run_policy(state: RunState, url: Option<String>) -> Option<CallToolResult> {
    match state {
        RunState::Stopped => None,
        RunState::Running => Some(url.map_or_else(
            || answer("unknown", None, "running run has no held gate"),
            |url| answer("succeeded", Some(json!({"url":url})), "run already running"),
        )),
        RunState::Unknown => Some(answer("unknown", None, "run may still be running")),
    }
}
// Order 081 settles successful-stop permanence. Cancel first, then wait for stage/start to
// quiesce under the operation lock, then stop. No backend restart is permitted after Ok.
fn successful_stop_policy(slot: &Slot) {
    slot.cancel();
}
// Order 081 settles this kit's channel lifetime choice: no implicit restarts.
async fn lifetime_policy(
    handle: &dyn GateHandle,
    channel_ended: &CancellationToken,
    cancel: &CancellationToken,
) {
    tokio::select! { _=handle.ended()=>{}, _=channel_ended.cancelled()=>{}, _=cancel.cancelled()=>{} }
}
// PRV-010 leaves restart policy to each provider. This kit never restarts implicitly.
fn implicit_restart_policy() -> bool {
    false
}
// No schema fixes a result for malformed provider metadata or unknown tool names. Keep the
// choice here: a failed tool result with a static diagnostic, and no backend call for it.
// spec/ leaves the outcome of a partially failed start open. Confirmed cleanup proves
// failure; otherwise retain uncertainty and the stop intent. Only a checked runtime-authored
// staging/start reason may enter an answer, never passed-through backend error text.
fn start_failure_policy(stopped: bool, reason: Option<&str>) -> CallToolResult {
    let mut message = "start did not complete".to_owned();
    if let Some(reason) = reason {
        message.push_str(": ");
        message.push_str(reason);
    }
    if stopped {
        message.push_str("; tree stopped");
        answer("failed", None, &message)
    } else {
        message.push_str("; stop could not be confirmed and persisted");
        answer("unknown", None, &message)
    }
}
fn invalid_call_policy(message: &'static str) -> CallToolResult {
    answer("failed", None, message)
}
fn answer(outcome: &str, outputs: Option<Value>, message: &str) -> CallToolResult {
    let mut wire = json!({"content":[{"type":"text","text":message}],"isError":outcome!="succeeded","_meta":{names::key("outcome"):outcome}});
    if let Some(outputs) = outputs {
        wire["structuredContent"] = outputs;
    }
    serde_json::from_value(wire).unwrap()
}
fn context_from(meta: &Value, machine: &Value) -> Option<CallContext> {
    let config = meta.get(names::key("config"))?.as_object()?.clone();
    let revision = meta.get(names::key("revision"))?.as_str()?.to_string();
    if revision.is_empty() {
        return None;
    }
    Some(CallContext {
        config: Value::Object(config),
        revision,
        machine: serde_json::from_value(machine.clone()).ok()?,
    })
}
fn bindings_valid(input: &RunInput) -> bool {
    input
        .environment
        .keys()
        .chain(input.secrets.environment.keys())
        .all(|k| {
            !k.eq_ignore_ascii_case(names::ENGINE_URL_VARIABLE)
                && !k.eq_ignore_ascii_case(names::IDENTITY_DIR_VARIABLE)
        })
        && !input
            .environment
            .keys()
            .any(|k| input.secrets.environment.contains_key(k))
}
impl<B: RuntimeBackend, S: StateStore, G: Gate> RuntimeServer<B, S, G> {
    pub async fn new(
        backend: Arc<B>,
        store: Arc<S>,
        gate: Arc<G>,
        options: Options,
    ) -> Result<Self> {
        if options.max_lifetime.is_zero() || options.drain_interval.is_zero() {
            return Err(Unavailable);
        }
        let server = Self(Arc::new(Inner {
            backend,
            store,
            gate,
            options,
            slots: Mutex::new(BTreeMap::new()),
            admissions: Mutex::new(BTreeMap::new()),
            shutdown: CancellationToken::new(),
        }));
        server.startup_policy().await?;
        if server.drain(None).await? {
            return Err(Unavailable);
        }
        let weak = Arc::downgrade(&server.0);
        let shutdown = server.0.shutdown.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! { _=shutdown.cancelled()=>break, _=tokio::time::sleep(options.drain_interval)=>{} }
                let Some(inner) = weak.upgrade() else {
                    break;
                };
                let _ = RuntimeServer(inner).drain(None).await;
            }
        });
        Ok(server)
    }
    fn slot(&self, instance: &str) -> Arc<Slot> {
        self.0
            .slots
            .lock()
            .unwrap()
            .entry(instance.into())
            .or_default()
            .clone()
    }
    fn admission(&self, hostname: &str) -> Arc<RwLock<()>> {
        self.0
            .admissions
            .lock()
            .unwrap()
            .entry(hostname.into())
            .or_default()
            .clone()
    }
    // Order 081 settles startup recovery: no channel survives provider restart, even orphans.
    async fn startup_policy(&self) -> Result<()> {
        let loaded = self.0.store.load().await;
        let inventory = self.0.backend.inventory().await;
        let mut failed = loaded.is_err() || inventory.is_err();
        for mut record in combine(loaded.unwrap_or_default(), inventory.unwrap_or_default()) {
            let slot = self.slot(&record.instance);
            slot.refresh(&mut record, None);
            let admission = self.admission(&record.context.machine.hostname);
            let attempt = stop_attempt(
                self.0.backend.as_ref(),
                self.0.store.as_ref(),
                &mut record,
                Some((&slot, &admission)),
            )
            .await;
            failed |= attempt.storage_failed;
        }
        // Construction discards reasons: only staging/start diagnostics enter answers.
        if failed { Err(Unavailable) } else { Ok(()) }
    }
    async fn stop_record(&self, mut record: RunRecord, slot: &Slot) -> Result<()> {
        let admission = self.admission(&record.context.machine.hostname);
        stop_attempt(
            self.0.backend.as_ref(),
            self.0.store.as_ref(),
            &mut record,
            Some((slot, &admission)),
        )
        .await
        .result
    }
    fn drain_records(&self, records: Vec<RunRecord>) -> Vec<RunRecord> {
        let pending = self
            .0
            .slots
            .lock()
            .unwrap()
            .values()
            .flat_map(|s| s.pending())
            .collect();
        combine(pending, records)
    }
    // Caller holds this instance's operation lock, then the machine's read lock.
    // Include live intents as well as durable ones without locking another instance.
    fn owed_stop_refusal(
        &self,
        instance: &str,
        context: &CallContext,
        ledger: Vec<RunRecord>,
    ) -> Option<CallToolResult> {
        let mut owed: Vec<_> = self
            .drain_records(ledger)
            .into_iter()
            .filter(|r| {
                r.owed_stop
                    && r.instance != instance
                    && r.context.machine.hostname == context.machine.hostname
            })
            .map(|r| r.instance)
            .collect();
        owed.sort();
        owed.dedup();
        if owed.is_empty() {
            None
        } else {
            Some(answer(
                "failed",
                None,
                &format!(
                    "machine {} still owes stops for instances {}; the start may be tried again once those stops are carried out",
                    context.machine.hostname,
                    owed.join(", ")
                ),
            ))
        }
    }
    fn owed_stop_uncertainty(
        &self,
        instance: &str,
        context: &CallContext,
        records: Vec<RunRecord>,
        ledger: Vec<RunRecord>,
    ) -> Option<CallToolResult> {
        let mut owed: Vec<_> = self
            .drain_records(combine(records, ledger))
            .into_iter()
            .filter(|r| {
                r.owed_stop
                    && (r.instance == instance
                        || r.context.machine.hostname == context.machine.hostname)
            })
            .map(|r| {
                format!(
                    "instance {} on machine {}",
                    r.instance, r.context.machine.hostname
                )
            })
            .collect();
        owed.sort();
        owed.dedup();
        if owed.is_empty() {
            None
        } else {
            Some(answer(
                "unknown",
                None,
                &format!(
                    "run may still be running; stops still owed for {}; the start may be tried again once those stops are carried out",
                    owed.join(", ")
                ),
            ))
        }
    }
    // Return write failures separately: only construction refuses service for them.
    async fn drain(&self, call: Option<&CallContext>) -> Result<bool> {
        // Same operation locks as tools/call; re-check memory as well as the ledger after waiting.
        let loaded = self.0.store.load().await;
        let mut failed = loaded.is_err();
        let mut storage_failed = false;
        for record in self.drain_records(loaded.unwrap_or_default()) {
            let slot = self.slot(&record.instance);
            let mut known = record.clone();
            // Remember every context without waiting for an unrelated run's operation.
            slot.refresh(&mut known, call);
            if !record.owed_stop {
                continue;
            }
            let _guard = slot.operation.lock().await;
            let loaded = self.0.store.load().await;
            failed |= loaded.is_err();
            if let Some(mut current) = combine(slot.pending(), loaded.unwrap_or_default())
                .into_iter()
                .find(|r| r.same_run(&record) && r.owed_stop)
            {
                slot.refresh(&mut current, call);
                // A drain retires the old flight, but cannot cancel a run.v1 waiting to
                // take this lock. Only stop.v1 and shutdown advance that call's epoch.
                {
                    let mut flight = slot.flight.lock().unwrap();
                    flight.serial += 1;
                    flight.cancel.cancel();
                }
                if let Some(gate) = slot.gate.lock().await.take() {
                    gate.close().await;
                }
                let admission = self.admission(&current.context.machine.hostname);
                let attempt = stop_attempt(
                    self.0.backend.as_ref(),
                    self.0.store.as_ref(),
                    &mut current,
                    Some((&slot, &admission)),
                )
                .await;
                storage_failed |= attempt.storage_failed;
            }
        }
        if failed {
            Err(Unavailable)
        } else {
            Ok(storage_failed)
        }
    }
    /// Explicitly close all gates and stop trees before retiring this server.
    pub async fn shutdown(&self) -> Result<()> {
        self.0.shutdown.cancel();
        for slot in self.0.slots.lock().unwrap().values() {
            successful_stop_policy(slot);
        }
        let loaded = self.0.store.load().await;
        let mut failed = loaded.is_err();
        for record in loaded.unwrap_or_default() {
            let slot = self.slot(&record.instance);
            let mut record = record;
            slot.refresh(&mut record, None);
        }
        let slots: Vec<_> = self
            .0
            .slots
            .lock()
            .unwrap()
            .iter()
            .map(|(id, slot)| (id.clone(), slot.clone()))
            .collect();
        for (instance, slot) in slots {
            // Cancellation transfers cleanup ownership from the lifetime task to us. Always
            // close the gate and attempt every known tree, even when the ledger read failed.
            let _guard = slot.operation.lock().await;
            if let Some(gate) = slot.gate.lock().await.take() {
                gate.close().await;
            }
            for record in slot.records(&instance) {
                if self.stop_record(record, &slot).await.is_err() {
                    failed = true;
                }
            }
        }
        if failed { Err(Unavailable) } else { Ok(()) }
    }
    /// Serves a byte transport (stdio, a socket, or a duplex test stream), then shuts down
    /// when the provider's own MCP channel ends. No input or backend error is logged.
    pub async fn serve_transport<T>(&self, transport: T) -> Result<()>
    where
        T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send + Unpin + 'static,
    {
        let service = Handler(self.clone())
            .serve({
                let (read, write) = tokio::io::split(transport);
                crate::transport::PrivateTransport::new(read, write)
            })
            .await;
        let result = match service {
            Ok(service) => service.waiting().await.map(|_| ()).map_err(|_| Unavailable),
            Err(_) => Err(Unavailable),
        };
        let cleanup = self.shutdown().await;
        result.and(cleanup)
    }
    pub async fn serve_stdio(&self) -> Result<()> {
        let service = Handler(self.clone())
            .serve(crate::transport::PrivateTransport::new(
                tokio::io::stdin(),
                tokio::io::stdout(),
            ))
            .await;
        let result = match service {
            Ok(service) => service.waiting().await.map(|_| ()).map_err(|_| Unavailable),
            Err(_) => Err(Unavailable),
        };
        let cleanup = self.shutdown().await;
        result.and(cleanup)
    }
    async fn execute(&self, name: String, args: Value, context: CallContext) -> CallToolResult {
        // Register before the first await: even a run stalled in its initial drain is in
        // flight and must observe a later successful stop (order 081).
        let instance = args["instance"].as_str().unwrap().to_owned();
        let slot = self.slot(&instance);
        let epoch = slot.flight.lock().unwrap().epoch;
        if name == "stop.v1" {
            successful_stop_policy(&slot);
        }
        if self.0.shutdown.is_cancelled() {
            return answer("unknown", None, "provider is shutting down");
        }
        let drained = self.drain(Some(&context)).await.is_ok();
        if !drained && name == "run.v1" {
            return answer("unknown", None, "cannot read durable stop obligations");
        }
        let _guard = slot.operation.lock().await;
        if name == "stop.v1" {
            return self
                .stop_instance(&instance, &context, &slot, drained)
                .await;
        }
        if name == "inspect-run.v1" {
            return self.inspect(&instance, &context, &slot).await;
        }
        let input: RunInput = match serde_json::from_value(args) {
            Ok(input) => input,
            Err(_) => return invalid_call_policy("invalid run inputs"),
        };
        if !bindings_valid(&input) {
            return invalid_call_policy("environment bindings overlap or bind identity variables");
        }
        if !self
            .0
            .backend
            .declaration()
            .supports(&input.item, &context.machine)
        {
            return invalid_call_policy("runtime does not run this item on this machine");
        }
        let ledger = match self.0.store.load().await {
            Ok(r) => r,
            Err(_) => return answer("unknown", None, "cannot read run ledger"),
        };
        let records = self.call_records(
            &instance,
            &context,
            ledger
                .iter()
                .filter(|r| r.instance == instance)
                .cloned()
                .collect(),
        );
        let state = self.state_of(&records).await;
        if state == RunState::Unknown
            && let Some(unknown) =
                self.owed_stop_uncertainty(&instance, &context, records, ledger.clone())
        {
            return unknown;
        }
        let url = slot.gate.lock().await.as_ref().map(|g| g.url());
        if let Some(answer) = repeated_run_policy(state, url) {
            return answer;
        }
        // The drain waits for known stops before taking this instance's lock. Stops can
        // publish during state_of: check their live intents under the machine lock.
        // Release it before staging so staging never delays a stop.
        let machine_admission = self.admission(&context.machine.hostname);
        let admission = machine_admission.read().await;
        if let Some(refused) = self.owed_stop_refusal(&instance, &context, ledger) {
            return refused;
        }
        drop(admission);
        if let Some(old_gate) = slot.gate.lock().await.take() {
            old_gate.close().await;
        }
        let (cancel, serial) = {
            let mut flight = slot.flight.lock().unwrap();
            if flight.epoch != epoch || self.0.shutdown.is_cancelled() {
                return answer("failed", None, "start cancelled by stop");
            }
            flight.cancel.cancel();
            flight.serial += 1;
            flight.cancel = CancellationToken::new();
            (flight.cancel.clone(), flight.serial)
        };
        let lifetime = self.0.options.max_lifetime;
        let deadline = cancel.clone();
        tokio::spawn(async move {
            tokio::select! { _=deadline.cancelled()=>{}, _=tokio::time::sleep(lifetime)=>deadline.cancel() }
        });
        let mut record = candidate(&instance, &context);
        if self.0.store.save(&record).await.is_err() {
            cancel.cancel();
            return answer("failed", None, "cannot persist run intent");
        }
        slot.remember(&record);
        let staged = match self.0.backend.stage(&context, &input, &cancel).await {
            Ok(staged) => staged,
            Err(error) => {
                let reason = checked_reason(error, &input);
                cancel.cancel();
                record.state = RunState::Stopped;
                let _ = self.0.store.save(&record).await;
                let message = reason.map_or_else(
                    || "staging failed".to_owned(),
                    |reason| format!("staging failed: {reason}"),
                );
                return answer("failed", None, &message);
            }
        };
        // A stop may have been published during staging. Re-read under admission and
        // hold through start only; failed_start itself needs the machine's write lock.
        let admission = machine_admission.read().await;
        let refused = match self.0.store.load().await {
            Ok(ledger) => self.owed_stop_refusal(&instance, &context, ledger),
            Err(_) => Some(answer("unknown", None, "cannot read run ledger")),
        };
        if let Some(refused) = refused {
            cancel.cancel();
            drop(admission);
            drop(staged);
            record.state = RunState::Stopped;
            let _ = self.0.store.save(&record).await;
            return refused;
        }
        if cancel.is_cancelled() {
            drop(admission);
            drop(staged);
            return self.failed_start(record, &slot, None).await;
        }
        let started = self
            .0
            .backend
            .start(&context, &instance, staged, &cancel, lifetime)
            .await;
        drop(admission);
        let channel = match started {
            Ok(channel) => channel,
            Err(error) => {
                let reason = checked_reason(error, &input);
                return self.failed_start(record, &slot, reason).await;
            }
        };
        if cancel.is_cancelled() {
            drop(channel);
            return self.failed_start(record, &slot, None).await;
        }
        let ended = CancellationToken::new();
        let observed = Box::new(Observed {
            channel,
            ended: ended.clone(),
        });
        let gate = match self.0.gate.open(observed, &input.identity, &cancel).await {
            Ok(gate) => gate,
            Err(_) => return self.failed_start(record, &slot, None).await,
        };
        let url = gate.url();
        if cancel.is_cancelled()
            || !schemas::valid("run.v1", "outputSchema", &json!({"url":url}))
            || sensitive_url(&url, &input)
        {
            gate.close().await;
            return self.failed_start(record, &slot, None).await;
        }
        record.state = RunState::Running;
        if self.0.store.save(&record).await.is_err() {
            gate.close().await;
            return self.failed_start(record, &slot, None).await;
        }
        *slot.gate.lock().await = Some(gate.clone());
        let server = self.clone();
        let slot_for_task = slot.clone();
        tokio::spawn(async move {
            lifetime_policy(gate.as_ref(), &ended, &cancel).await;
            let _guard = slot_for_task.operation.lock().await;
            if slot_for_task.flight.lock().unwrap().serial != serial {
                return;
            }
            cancel.cancel();
            gate.close().await;
            slot_for_task.gate.lock().await.take();
            // Stop with the newest context a call has given since the run started, never the
            // one captured when it started (order 124, decision 1).
            let mut record = record;
            slot_for_task.refresh(&mut record, None);
            let _ = server.stop_record(record, &slot_for_task).await;
            debug_assert!(!implicit_restart_policy());
        });
        answer("succeeded", Some(json!({"url":url})), "run started")
    }
    async fn failed_start(
        &self,
        record: RunRecord,
        slot: &Slot,
        reason: Option<String>,
    ) -> CallToolResult {
        slot.flight.lock().unwrap().cancel.cancel();
        start_failure_policy(
            self.stop_record(record, slot).await.is_ok(),
            reason.as_deref(),
        )
    }
    async fn records(&self, instance: &str, context: &CallContext) -> Result<Vec<RunRecord>> {
        let records: Vec<_> = self
            .0
            .store
            .load()
            .await?
            .into_iter()
            .filter(|r| r.instance == instance)
            .collect();
        Ok(self.call_records(instance, context, records))
    }
    fn call_records(
        &self,
        instance: &str,
        context: &CallContext,
        records: Vec<RunRecord>,
    ) -> Vec<RunRecord> {
        let slot = self.slot(instance);
        let records = combine(slot.pending(), records);
        let mut records = combine(records, slot.records(instance));
        for record in &mut records {
            slot.refresh(record, Some(context));
        }
        // Each refresh above already saved its record's context under the mutex; writing them
        // all again here could restore a context another call installed meanwhile. Only the
        // candidate for this call's own machine is new.
        let candidate = candidate(instance, context);
        slot.remember(&candidate);
        combine(records, vec![candidate])
    }
    async fn state_of(&self, records: &[RunRecord]) -> RunState {
        let mut state = RunState::Stopped;
        for record in records {
            let observed = self
                .0
                .backend
                .state(&record.context, &record.instance)
                .await
                .unwrap_or(RunState::Unknown);
            // An owed stop blocks a new start even if today's observation says stopped.
            if observed == RunState::Unknown || record.owed_stop {
                return RunState::Unknown;
            }
            if observed == RunState::Running {
                state = RunState::Running;
            }
        }
        state
    }
    // Order 081 settles inspection of all held machines: stopped requires every state stopped.
    async fn inspect(&self, instance: &str, context: &CallContext, slot: &Slot) -> CallToolResult {
        let state = match self.records(instance, context).await {
            Ok(records) => self.state_of(&records).await,
            Err(_) => RunState::Unknown,
        };
        let mut outputs = json!({"state":state});
        if let Some(gate) = slot.gate.lock().await.as_ref() {
            outputs["url"] = json!(gate.url());
        }
        answer("succeeded", Some(outputs), "run state inspected")
    }
    async fn stop_instance(
        &self,
        instance: &str,
        context: &CallContext,
        slot: &Slot,
        drained: bool,
    ) -> CallToolResult {
        if let Some(gate) = slot.gate.lock().await.take() {
            gate.close().await;
        }
        let (records, readable) = match self.records(instance, context).await {
            Ok(records) => (records, true),
            Err(_) => (self.call_records(instance, context, Vec::new()), false),
        };
        // A read failure leaves uncertainty about other durable runs, but must not prevent
        // cleanup or durable stop intent for the call's machine and every held machine.
        let mut stopped = drained && readable;
        for record in records {
            if self.stop_record(record, slot).await.is_err() {
                stopped = false;
            }
        }
        if stopped {
            answer("succeeded", Some(json!({})), "run tree stopped")
        } else {
            answer(
                "unknown",
                None,
                "run stop could not be confirmed and persisted",
            )
        }
    }
}
fn sensitive_values(input: &RunInput) -> impl Iterator<Item = &str> {
    input
        .identity
        .environment
        .values()
        .chain(input.identity.files.values())
        .chain(input.secrets.environment.values())
        .chain(input.secrets.package_arguments.values())
        .map(String::as_str)
        .filter(|value| !value.is_empty())
}
fn sensitive_url(url: &str, input: &RunInput) -> bool {
    sensitive_values(input).any(|value| url.contains(value))
}
fn one_line(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_control() || matches!(c, '\u{2028}' | '\u{2029}') {
                ' '
            } else {
                c
            }
        })
        .collect()
}
// Check the entire uncut reason in both forms before exposing any of it in an answer.
fn checked_reason(error: Unavailable, input: &RunInput) -> Option<String> {
    let reason = error.into_reason()?;
    let cleaned = one_line(&reason);
    if sensitive_values(input)
        .any(|value| reason.contains(value) || cleaned.contains(&one_line(value)))
    {
        return Some(
            "runtime's reason was withheld because it contained a value supplied to the run"
                .to_owned(),
        );
    }
    let bounded: String = cleaned.chars().take(512).collect();
    if bounded.trim().is_empty() {
        None
    } else {
        Some(bounded)
    }
}
impl<B: RuntimeBackend, S: StateStore, G: Gate> ServerHandler for Handler<B, S, G> {
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2026-07-28","serverInfo":{"name":"runtime-provider-kit","version":env!("CARGO_PKG_VERSION")},"capabilities":{"tools":{},"extensions":{names::extension():{"revision":"v0"}}}})).unwrap()
    }
    async fn discover(
        &self,
        _: RequestContext<RoleServer>,
    ) -> std::result::Result<DiscoverResult, ErrorData> {
        Ok(DiscoverResult::from_server_info(
            self.supported_protocol_versions().into_owned(),
            self.get_info(),
        )
        .with_cache_scope(CacheScope::Private))
    }
    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> std::result::Result<ListToolsResult, ErrorData> {
        let tools=schemas::OPERATIONS.iter().map(|name| {
            let mut tool=json!({"name":name,"description":format!("Runtime provider {name}"),"inputSchema":schemas::schema(name,"inputSchema").unwrap()});
            if let Some(output)=schemas::schema(name,"outputSchema") { tool["outputSchema"]=output; }
            serde_json::from_value(tool).unwrap()
        }).collect();
        Ok(ListToolsResult::with_all_items(tools))
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        request_context: RequestContext<RoleServer>,
    ) -> std::result::Result<CallToolResponse, ErrorData> {
        let (request, meta) = if let Some(private) = request_context
            .extensions
            .get::<crate::transport::PrivateCall>()
        {
            (
                private.params.clone(),
                serde_json::to_value(&private.meta).unwrap_or(Value::Null),
            )
        } else {
            (
                request,
                serde_json::to_value(request_context.meta).unwrap_or(Value::Null),
            )
        };
        if meta["io.modelcontextprotocol/clientCapabilities"]["extensions"][names::extension()]["revision"]
            != "v0"
        {
            return Err(serde_json::from_value(json!({"code":-32021,"message":"Missing required client capability","data":{"requiredCapabilities":{"extensions":{names::extension():{"revision":"v0"}}}}})).unwrap());
        }
        let name = request.name.to_string();
        let args = Value::Object(request.arguments.unwrap_or_default());
        if !schemas::OPERATIONS.contains(&name.as_str()) {
            return Ok(invalid_call_policy("unknown runtime operation").into());
        }
        if !schemas::valid(&name, "inputSchema", &args) {
            return Ok(invalid_call_policy("arguments do not match operation schema").into());
        }
        let Some(context) = context_from(&meta, &args["machine"]) else {
            return Ok(
                invalid_call_policy("provider config or revision missing or malformed").into(),
            );
        };
        // An MCP caller dropping its wait cannot drop an in-flight start future. The owned
        // task finishes or cooperatively cancels, and stop waits for it to quiesce.
        let server = self.0.clone();
        let task = tokio::spawn(async move { server.execute(name, args, context).await });
        Ok(task
            .await
            .unwrap_or_else(|_| answer("unknown", None, "runtime task ended unexpectedly"))
            .into())
    }
}

#[cfg(test)]
mod refresh_tests {
    use super::*;
    use std::sync::{Arc, Barrier};

    fn context(revision: &str) -> CallContext {
        CallContext {
            config: serde_json::json!({"revision": revision}),
            revision: revision.to_string(),
            machine: Machine {
                hostname: "lab-host".to_string(),
                labels: BTreeMap::new(),
            },
        }
    }

    fn record(revision: &str) -> RunRecord {
        RunRecord {
            instance: "instance-a".to_string(),
            context: context(revision),
            state: RunState::Running,
            owed_stop: true,
        }
    }

    // A refresh without a call (an interval drain) that has copied r1 must never write r1 back
    // over the r2 that a live call installed meanwhile. Whatever the interleaving of the two
    // refreshes, the held context ends at r2, and so does every record refreshed after it.
    #[test]
    fn interval_refresh_never_writes_back_an_older_context() {
        for _ in 0..2000 {
            let slot = Arc::new(Slot::default());
            slot.remember(&record("r1"));
            let barrier = Arc::new(Barrier::new(2));
            let interval = {
                let (slot, barrier) = (slot.clone(), barrier.clone());
                std::thread::spawn(move || {
                    let mut stale = record("r1");
                    barrier.wait();
                    slot.refresh(&mut stale, None);
                })
            };
            let call = {
                let (slot, barrier) = (slot.clone(), barrier.clone());
                std::thread::spawn(move || {
                    let mut live = record("r1");
                    barrier.wait();
                    slot.refresh(&mut live, Some(&context("r2")));
                })
            };
            interval.join().unwrap();
            call.join().unwrap();
            let held = slot
                .contexts
                .lock()
                .unwrap()
                .get("lab-host")
                .cloned()
                .unwrap();
            assert_eq!(held.revision, "r2");
            let mut later = record("r1");
            slot.refresh(&mut later, None);
            assert_eq!(later.context.revision, "r2");
        }
    }

    #[test]
    fn contended_interval_refreshes_never_regress_a_live_context() {
        let slot = Arc::new(Slot::default());
        slot.remember(&record("0"));
        let done = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let intervals: Vec<_> = (0..3)
            .map(|_| {
                let (slot, done) = (slot.clone(), done.clone());
                std::thread::spawn(move || {
                    while !done.load(std::sync::atomic::Ordering::Relaxed) {
                        slot.refresh(&mut record("0"), None);
                    }
                })
            })
            .collect();
        let mut regressed = None;
        for n in 1..20000u32 {
            let held = slot
                .contexts
                .lock()
                .unwrap()
                .get("lab-host")
                .cloned()
                .unwrap();
            if held.revision != (n - 1).to_string() {
                regressed = Some((n - 1, held.revision));
                break;
            }
            slot.refresh(&mut record("0"), Some(&context(&n.to_string())));
        }
        done.store(true, std::sync::atomic::Ordering::Relaxed);
        for t in intervals {
            t.join().unwrap();
        }
        assert_eq!(regressed, None);
    }
}
