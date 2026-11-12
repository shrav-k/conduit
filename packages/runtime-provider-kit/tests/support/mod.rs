use async_trait::async_trait;
use rmcp::{RoleClient, ServiceExt, service::RunningService};
use runtime_provider_kit::*;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, DuplexStream},
    sync::Notify,
};
use tokio_util::sync::CancellationToken;
pub const ID: &str = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
pub fn machine() -> Machine {
    Machine {
        hostname: "test-machine".into(),
        labels: BTreeMap::new(),
    }
}
pub fn context() -> CallContext {
    CallContext {
        config: json!({"backend":"fake"}),
        revision: "r1".into(),
        machine: machine(),
    }
}
pub fn record(id: &str) -> RunRecord {
    RunRecord {
        instance: id.into(),
        context: context(),
        state: RunState::Running,
        owed_stop: false,
    }
}
pub fn options() -> Options {
    Options {
        max_lifetime: Duration::from_secs(30),
        drain_interval: Duration::from_secs(30),
    }
}
// Construct markers without ever printing their values, including on assertion failure.
pub fn markers() -> Vec<String> {
    (0..9)
        .map(|i| format!("{}{}{}", "sensitive-", "synthetic-", i))
        .collect()
}
pub fn run_args() -> Value {
    let marks = markers();
    let files: serde_json::Map<String, Value> = schemas::identity_file_names()
        .into_iter()
        .enumerate()
        .map(|(i, name)| (name.into(), json!(marks[i])))
        .collect();
    json!({"instance":ID,"machine":machine(),"item":{"implements":[format!("{}#example.v1",names::CORE_PACKAGE)],"command":["fake-server"]},"environment":{"ORDINARY":"public"},"packageArguments":{"ordinary":"public"},"secrets":{"environment":{"PRIVATE":marks[6]},"packageArguments":{"private":marks[7]}},"identity":{"environment":{names::ENGINE_URL_VARIABLE:format!("https://{}.invalid/mcp",marks[8]),names::IDENTITY_DIR_VARIABLE:format!("/identity/{}",marks[8])},"files":files}})
}
pub fn args() -> Value {
    json!({"instance":ID,"machine":machine()})
}
pub fn call(name: &str, args: Value) -> rmcp::model::CallToolRequestParams {
    serde_json::from_value(json!({"name":name,"arguments":args,"_meta":{names::key("config"):context().config,names::key("revision"):context().revision,"io.modelcontextprotocol/clientCapabilities":{"extensions":{names::extension():{"revision":"v0"}}}}})).unwrap()
}
pub fn outcome(result: &rmcp::model::CallToolResult, expected: &str) {
    let wire = serde_json::to_value(result).unwrap();
    assert!(
        wire["_meta"][names::key("outcome")] == expected,
        "wrong outcome"
    );
    assert!(wire["_meta"].get(names::key("reason")).is_none());
    assert!(result.is_error == Some(expected != "succeeded"));
    for marker in markers() {
        assert!(!wire.to_string().contains(&marker), "sensitive answer");
    }
}
/// This fake store actually commits to a file, then a new object reopens it on restart.
pub struct FileStore {
    pub fail_load: AtomicBool,
    pub loads: AtomicUsize,
    pub fail_save: AtomicBool,
    pub fail_stopped_save: AtomicBool,
    pub fail_running_save: AtomicBool,
    pub failure_reason: Mutex<Option<String>>,
    pub pause_next_load: AtomicBool,
    pub load_entered: Notify,
    pub resume_load: Notify,
    path: PathBuf,
    lock: Mutex<()>,
}
impl FileStore {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            lock: Mutex::new(()),
            fail_load: AtomicBool::new(false),
            loads: AtomicUsize::new(0),
            fail_save: AtomicBool::new(false),
            fail_stopped_save: AtomicBool::new(false),
            fail_running_save: AtomicBool::new(false),
            failure_reason: Mutex::new(None),
            pause_next_load: AtomicBool::new(false),
            load_entered: Notify::new(),
            resume_load: Notify::new(),
        }
    }
    fn read(&self) -> Vec<RunRecord> {
        if self.path.exists() {
            serde_json::from_slice(&std::fs::read(&self.path).unwrap()).unwrap()
        } else {
            Vec::new()
        }
    }
    pub fn bytes(&self) -> Vec<u8> {
        std::fs::read(&self.path).unwrap_or_default()
    }
}
#[async_trait]
impl StateStore for FileStore {
    async fn load(&self) -> Result<Vec<RunRecord>> {
        self.loads.fetch_add(1, Ordering::SeqCst);
        if self.pause_next_load.swap(false, Ordering::SeqCst) {
            self.load_entered.notify_one();
            self.resume_load.notified().await;
        }
        if self.fail_load.load(Ordering::SeqCst) {
            return Err(fake_error(&self.failure_reason));
        }
        let _lock = self.lock.lock().unwrap();
        Ok(self.read())
    }
    async fn save(&self, record: &RunRecord) -> Result<()> {
        if self.fail_save.load(Ordering::SeqCst)
            || (record.state == RunState::Stopped && self.fail_stopped_save.load(Ordering::SeqCst))
            || (record.state == RunState::Running && self.fail_running_save.load(Ordering::SeqCst))
        {
            return Err(fake_error(&self.failure_reason));
        }
        let _lock = self.lock.lock().unwrap();
        let mut records = self.read();
        records.retain(|r| !r.same_run(record));
        records.push(record.clone());
        let temporary = self.path.with_extension("new");
        std::fs::write(&temporary, serde_json::to_vec(&records).unwrap())
            .map_err(|_| Unavailable)?;
        std::fs::File::open(&temporary)
            .unwrap()
            .sync_all()
            .map_err(|_| Unavailable)?;
        std::fs::rename(&temporary, &self.path).map_err(|_| Unavailable)?;
        std::fs::File::open(self.path.parent().unwrap())
            .unwrap()
            .sync_all()
            .map_err(|_| Unavailable)?;
        Ok(())
    }
}
fn fake_error(reason: &Mutex<Option<String>>) -> Unavailable {
    reason
        .lock()
        .unwrap()
        .as_ref()
        .map_or(Unavailable, |reason| Unavailable::because(reason.clone()))
}
#[derive(Default)]
pub struct FakeBackend {
    pub calls: AtomicUsize,
    pub starts: AtomicUsize,
    pub launches: AtomicUsize,
    pub stops: AtomicUsize,
    pub unavailable: AtomicBool,
    pub unknown: AtomicBool,
    pub block: AtomicBool,
    pub fail_stage: AtomicBool,
    pub fail_start: AtomicBool,
    pub stage_reason: Mutex<Option<String>>,
    pub start_reason: Mutex<Option<String>>,
    pub failure_reason: Mutex<Option<String>>,
    pub fail_inventory: AtomicBool,
    pub fail_state: AtomicBool,
    pub cancel_after_stage: AtomicBool,
    pub cancel_after_start: AtomicBool,
    pub pause_next_stage: AtomicBool,
    pub stage_entered: Notify,
    pub resume_stage: Notify,
    pub staged_drops: Arc<AtomicUsize>,
    pub pause_next_start: AtomicBool,
    pub resume_start: Notify,
    pub pause_next_state: AtomicBool,
    pub state_entered: Notify,
    pub resume_state: Notify,
    pub pause_next_stop: AtomicBool,
    pub stop_entered: Notify,
    pub resume_stop: Notify,
    pub entered: Notify,
    pub contexts: Mutex<Vec<CallContext>>,
    pub runs: Mutex<BTreeMap<String, (RunRecord, Option<DuplexStream>)>>,
    pub staged_valid: AtomicBool,
}
pub struct Staged {
    drops: Arc<AtomicUsize>,
}
impl Drop for Staged {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}
impl FakeBackend {
    fn note(&self, context: &CallContext) {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.contexts.lock().unwrap().push(context.clone());
    }
    pub fn end_channel(&self) {
        self.runs.lock().unwrap().get_mut(ID).unwrap().1.take();
    }
    pub fn seed(&self, id: &str) {
        self.runs
            .lock()
            .unwrap()
            .insert(id.into(), (record(id), None));
    }
}
#[async_trait]
impl RuntimeBackend for FakeBackend {
    type Staged = Staged;
    fn declaration(&self) -> Declaration {
        Declaration {
            registry_types: vec!["oci".into()],
            transports: vec!["stdio".into()],
            command: true,
            machines: vec![MachineSelector {
                hostname: Some(machine().hostname),
                labels: BTreeMap::new(),
            }],
        }
    }
    async fn inventory(&self) -> Result<Vec<RunRecord>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail_inventory.load(Ordering::SeqCst) {
            return Err(fake_error(&self.failure_reason));
        }
        Ok(self
            .runs
            .lock()
            .unwrap()
            .values()
            .map(|(r, _)| r.clone())
            .collect())
    }
    async fn stage(
        &self,
        context: &CallContext,
        input: &RunInput,
        cancel: &CancellationToken,
    ) -> Result<Staged> {
        self.note(context);
        let expected = run_args();
        let valid = input
            .identity
            .files
            .iter()
            .all(|(k, v)| expected["identity"]["files"][k] == *v)
            && input.identity.files.len() == 6
            && input
                .identity
                .environment
                .iter()
                .all(|(k, v)| expected["identity"]["environment"][k] == *v)
            && input.secrets.environment["PRIVATE"]
                == expected["secrets"]["environment"]["PRIVATE"]
            && input.secrets.package_arguments["private"]
                == expected["secrets"]["packageArguments"]["private"]
            && input.environment["ORDINARY"] == "public"
            && input.package_arguments["ordinary"] == "public";
        self.staged_valid.store(valid, Ordering::SeqCst);
        if self.fail_stage.load(Ordering::SeqCst) {
            return Err(fake_error(&self.stage_reason));
        }
        let staged = Staged {
            drops: self.staged_drops.clone(),
        };
        if self.pause_next_stage.swap(false, Ordering::SeqCst) {
            self.stage_entered.notify_one();
            tokio::select! {
                _ = self.resume_stage.notified() => {},
                _ = cancel.cancelled() => return Err(Unavailable),
            }
        }
        if self.cancel_after_stage.load(Ordering::SeqCst) {
            cancel.cancel();
        }
        Ok(staged)
    }
    async fn start(
        &self,
        context: &CallContext,
        instance: &str,
        _staged: Staged,
        cancel: &CancellationToken,
        _: Duration,
    ) -> Result<Channel> {
        self.note(context);
        self.starts.fetch_add(1, Ordering::SeqCst);
        self.entered.notify_one();
        if self.pause_next_start.swap(false, Ordering::SeqCst) {
            tokio::select! {
                _ = self.resume_start.notified() => {},
                _ = cancel.cancelled() => return Err(Unavailable),
            }
        }
        if self.block.load(Ordering::SeqCst) {
            cancel.cancelled().await;
            return Err(Unavailable);
        }
        if cancel.is_cancelled() {
            return Err(Unavailable);
        }
        let (channel, peer) = tokio::io::duplex(1024);
        self.runs.lock().unwrap().insert(
            instance.into(),
            (
                RunRecord {
                    context: context.clone(),
                    ..record(instance)
                },
                Some(peer),
            ),
        );
        self.launches.fetch_add(1, Ordering::SeqCst);
        if self.cancel_after_start.load(Ordering::SeqCst) {
            cancel.cancel();
        }
        if self.fail_start.load(Ordering::SeqCst) {
            Err(fake_error(&self.start_reason))
        } else {
            Ok(Box::new(channel))
        }
    }
    async fn stop(&self, context: &CallContext, instance: &str) -> Result<()> {
        self.note(context);
        self.stops.fetch_add(1, Ordering::SeqCst);
        if self.pause_next_stop.swap(false, Ordering::SeqCst) {
            self.stop_entered.notify_one();
            self.resume_stop.notified().await;
        }
        if self.unavailable.load(Ordering::SeqCst) {
            return Err(fake_error(&self.failure_reason));
        }
        self.runs.lock().unwrap().remove(instance);
        Ok(())
    }
    async fn state(&self, context: &CallContext, instance: &str) -> Result<RunState> {
        self.note(context);
        if self.fail_state.load(Ordering::SeqCst) {
            return Err(fake_error(&self.failure_reason));
        }
        if self.pause_next_state.swap(false, Ordering::SeqCst) {
            self.state_entered.notify_one();
            self.resume_state.notified().await;
        }
        if self.unknown.load(Ordering::SeqCst) {
            return Ok(RunState::Unknown);
        }
        Ok(if self.runs.lock().unwrap().contains_key(instance) {
            RunState::Running
        } else {
            RunState::Stopped
        })
    }
}
#[derive(Default)]
pub struct FakeGate {
    pub handles: Mutex<Vec<Arc<Handle>>>,
    pub opens: AtomicUsize,
    pub fail: AtomicBool,
    pub failure_reason: Mutex<Option<String>>,
    pub url_override: Mutex<Option<String>>,
}
pub struct Handle {
    close: CancellationToken,
    done: CancellationToken,
    pub url: Mutex<String>,
}
#[async_trait]
impl GateHandle for Handle {
    fn url(&self) -> String {
        self.url.lock().unwrap().clone()
    }
    async fn close(&self) {
        self.close.cancel();
        self.done.cancelled().await;
    }
    async fn ended(&self) {
        self.done.cancelled().await;
    }
}
#[async_trait]
impl Gate for FakeGate {
    async fn open(
        &self,
        mut channel: Channel,
        identity: &Identity,
        _: &CancellationToken,
    ) -> Result<Arc<dyn GateHandle>> {
        if self.fail.load(Ordering::SeqCst) {
            return Err(fake_error(&self.failure_reason));
        }
        assert!(identity.files.len() == 6);
        let n = self.opens.fetch_add(1, Ordering::SeqCst);
        let handle = Arc::new(Handle {
            close: CancellationToken::new(),
            done: CancellationToken::new(),
            url: Mutex::new(
                self.url_override
                    .lock()
                    .unwrap()
                    .clone()
                    .unwrap_or_else(|| format!("https://127.0.0.1:{}/mcp", 41000 + n)),
            ),
        });
        let task_handle = handle.clone();
        tokio::spawn(async move {
            let mut buf = [0u8; 256];
            loop {
                tokio::select! { _=task_handle.close.cancelled()=>break,result=channel.read(&mut buf)=>match result { Ok(0)|Err(_)=>break,_=>{} } }
            }
            drop(channel);
            task_handle.done.cancel();
        });
        self.handles.lock().unwrap().push(handle.clone());
        Ok(handle)
    }
}
pub type Server = RuntimeServer<FakeBackend, FileStore, FakeGate>;
pub type Client = RunningService<RoleClient, ()>;
pub struct Harness {
    pub server: Server,
    pub backend: Arc<FakeBackend>,
    pub store: Arc<FileStore>,
    pub gate: Arc<FakeGate>,
    pub client: Client,
    task: tokio::task::JoinHandle<Result<()>>,
    pub directory: tempfile::TempDir,
}
impl Harness {
    pub async fn new() -> Self {
        Self::with_options(options()).await
    }
    pub async fn with_options(options: Options) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let store = Arc::new(FileStore::new(directory.path().join("ledger.json")));
        let backend = Arc::new(FakeBackend::default());
        let gate = Arc::new(FakeGate::default());
        let server = Server::new(backend.clone(), store.clone(), gate.clone(), options)
            .await
            .unwrap();
        let (client, transport) = tokio::io::duplex(65536);
        let running = server.clone();
        let task = tokio::spawn(async move { running.serve_transport(transport).await });
        let client = ().serve(client).await.unwrap();
        Self {
            server,
            backend,
            store,
            gate,
            client,
            task,
            directory,
        }
    }
    pub async fn call(&self, name: &str, args: Value) -> rmcp::model::CallToolResult {
        self.client.call_tool(call(name, args)).await.unwrap()
    }
    pub async fn finish(self) -> tempfile::TempDir {
        self.client.cancel().await.unwrap();
        let _ = tokio::time::timeout(Duration::from_secs(3), self.task)
            .await
            .unwrap()
            .unwrap();
        self.directory
    }
    pub async fn close(self) {
        drop(self.finish().await);
    }
}
pub async fn eventually(mut condition: impl FnMut() -> bool) {
    tokio::time::timeout(Duration::from_secs(3), async {
        while !condition() {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("condition not reached");
}
