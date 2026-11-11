//! Engine-owned processes and MCP sessions. Only completions reenter the durable shell.
use crate::{Execution, Executor, Result, RunIdentity, Shell, store::private_directory};
use conduit_core::archives::{InstalledPackage, Member};
use rmcp::{
    RoleClient, ServiceExt,
    model::{ClientRequest, GetExtensions, JsonRpcMessage, RequestId},
    service::{Peer, PeerRequestOptions, RunningService, RxJsonRpcMessage, TxJsonRpcMessage},
    transport::Transport,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs,
    io::{Read, Write},
    os::{
        fd::{AsRawFd, FromRawFd, OwnedFd},
        unix::{fs::PermissionsExt, net::UnixStream, process::CommandExt},
    },
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout},
    runtime::{Handle, Runtime},
    sync::Mutex as AsyncMutex,
};
use uuid::Uuid;

type Attempt = (String, u64);
type Responses = Arc<Mutex<HashMap<RequestId, Value>>>;

/// The lifetime pipe is held by the engine only. The guard kills its whole process group
/// on EOF, including when the engine is killed without running destructors.
pub(crate) struct ProcessGroup {
    pid: i32,
    stopped: AtomicBool,
    lifetime: Mutex<Option<UnixStream>>,
}
impl ProcessGroup {
    pub(crate) fn stop(&self) {
        if !self.stopped.swap(true, Ordering::SeqCst) {
            // SAFETY: a negative PID addresses only the group created for this child.
            unsafe {
                libc::kill(-self.pid, libc::SIGKILL);
            }
            self.lifetime.lock().unwrap().take();
        }
    }
}
impl Drop for ProcessGroup {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Internal entry point used by conduit before opening any store or Tokio runtime.
/// FD 3 is a read-only lifetime pipe; no credential is passed through it.
pub fn guard_main() -> Result<()> {
    let mut args = std::env::args_os().skip(2);
    let names: Vec<String> = serde_json::from_str(
        &args
            .next()
            .ok_or("guard needs environment names")?
            .to_string_lossy(),
    )
    .map_err(|_| "invalid guard environment names")?;
    let environment = names
        .into_iter()
        .filter_map(|name| std::env::var_os(&name).map(|v| (name, v)))
        .collect::<BTreeMap<_, _>>();
    let program = args.next().ok_or("guard needs program")?;
    // SAFETY: the runner explicitly installs its inherited lifetime descriptor at FD 3.
    let mut lifetime = unsafe { fs::File::from_raw_fd(3) };
    // The run must not keep the engine's lifetime channel open across exec.
    unsafe {
        libc::fcntl(3, libc::F_SETFD, libc::FD_CLOEXEC);
    }
    let mut status_pipe = lifetime
        .try_clone()
        .map_err(|_| "guard status pipe failed")?;
    let mut child = Command::new(program)
        .args(args)
        .env_clear()
        .envs(environment)
        .spawn()
        .map_err(|_| "run spawn failed")?;
    // Only the run retains the stdio pipes. Its exit must close the MCP session promptly.
    unsafe {
        libc::close(0);
        libc::close(1);
        libc::close(2);
    }
    // Reap the direct child while waiting for the engine's lifetime pipe to close.
    std::thread::spawn(move || {
        let code = child.wait().ok().and_then(|s| s.code()).unwrap_or(-1);
        let _ = status_pipe.write_all(&code.to_be_bytes());
    });
    let mut byte = [0];
    loop {
        match lifetime.read(&mut byte) {
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            _ => break,
        }
    }
    // SAFETY: the guard is the leader of a fresh group and the run inherits that group.
    unsafe {
        libc::kill(0, libc::SIGKILL);
    }
    Err("guard could not terminate its process group".into())
}

struct Run {
    service: RunningService<RoleClient, ()>,
    group: Arc<ProcessGroup>,
    responses: Responses,
    setup: String,
}
struct Inner {
    stopping: AtomicBool,
    shell: Weak<Shell>,
    guard: PathBuf,
    directory: PathBuf,
    handle: Handle,
    runs: Mutex<BTreeMap<String, Arc<Run>>>,
    pending_groups: Mutex<BTreeMap<String, Vec<Arc<ProcessGroup>>>>,
    cancelled_setups: Mutex<BTreeSet<String>>,
    requests: Mutex<BTreeMap<Attempt, (Peer<RoleClient>, RequestId)>>,
    cancelled_attempts: Mutex<BTreeSet<Attempt>>,
    cache_lock: AsyncMutex<()>,
    errors: Mutex<Vec<String>>,
    completion_executor: Mutex<Option<Weak<dyn Executor>>>,
}

/// Own this alongside the shell. Dropping either owner ends every run; reopening starts
/// with an empty run table. Workers retain only a Weak<Shell> while waiting on I/O.
pub struct Runner {
    inner: Arc<Inner>,
    _runtime: Runtime,
}
impl Runner {
    pub fn new(shell: &Arc<Shell>, guard_program: PathBuf) -> Result<Self> {
        let directory = shell
            .writer
            .lock()
            .map_err(|_| "writer lock poisoned")?
            .store
            .directory
            .clone();
        for name in ["npm", "npm-downloads", "work", "programs", "run-groups"] {
            private_directory(&directory.join(name))?;
        }
        crate::store::atomic_write(&directory, "npm-userconfig", b"")?;
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Self {
            inner: Arc::new(Inner {
                stopping: AtomicBool::new(false),
                shell: Arc::downgrade(shell),
                guard: guard_program,
                directory,
                handle: runtime.handle().clone(),
                runs: Mutex::new(BTreeMap::new()),
                pending_groups: Mutex::new(BTreeMap::new()),
                cancelled_setups: Mutex::new(BTreeSet::new()),
                requests: Mutex::new(BTreeMap::new()),
                cancelled_attempts: Mutex::new(BTreeSet::new()),
                cache_lock: AsyncMutex::new(()),
                errors: Mutex::new(Vec::new()),
                completion_executor: Mutex::new(None),
            }),
            _runtime: runtime,
        })
    }
    /// Decorate deferred transitions without prolonging the runner's lifetime.
    pub fn set_completion_executor(&self, executor: Weak<dyn Executor>) {
        *self.inner.completion_executor.lock().unwrap() = Some(executor);
    }
    pub fn executor(&self) -> Arc<dyn Executor> {
        Arc::new(self.inner.clone())
    }
    /// Fixed diagnostics only: never child stderr, environment values, arguments or identity.
    pub fn diagnostics(&self) -> Vec<String> {
        self.inner.errors.lock().unwrap().clone()
    }
    pub fn running_count(&self) -> usize {
        self.inner
            .runs
            .lock()
            .unwrap()
            .values()
            .filter(|r| {
                !r.service.is_closed()
                    && !r.service.peer().is_transport_closed()
                    && !r.group.stopped.load(Ordering::SeqCst)
            })
            .count()
    }
}
impl Drop for Runner {
    fn drop(&mut self) {
        // Serialize shutdown with the spawn-and-register section, so a deferred start
        // cannot register a fresh group just after shutdown's group scan.
        let _spawns = self.inner.cancelled_setups.lock().unwrap();
        self.inner.stopping.store(true, Ordering::SeqCst);
        for (subject, run) in self.inner.runs.lock().unwrap().iter() {
            run.group.stop();
            if let Some(shell) = self.inner.shell.upgrade()
                && shell
                    .submit(
                        json!({"kind":"builtin-run-stopped","object":subject}),
                        &crate::MissingExecutor,
                    )
                    .is_err()
            {
                eprintln!(
                    "runner: stopped run observation could not be persisted; inspect and set up the instance again to refresh its status"
                );
            }
        }
        for groups in self.inner.pending_groups.lock().unwrap().values() {
            for group in groups {
                group.stop();
            }
        }
    }
}
impl Executor for Runner {
    fn builtin_runner(&self, intent: &Value, identity: Option<&RunIdentity>) -> Result<Execution> {
        self.inner.builtin_runner(intent, identity)
    }
    fn dispatch(&self, intent: &Value) -> Result<Execution> {
        self.inner.dispatch(intent)
    }
    fn refresh_tools_and_call(&self, intent: &Value) -> Result<Execution> {
        self.inner.refresh_tools_and_call(intent)
    }
    fn cancel(&self, intent: &Value) -> Result<Execution> {
        self.inner.cancel(intent)
    }
    fn cancel_setup_parts(&self, intent: &Value) -> Result<Execution> {
        self.inner.cancel_setup_parts(intent)
    }
}
// Arc permits deferred work to use the same executor for any intents its completion emits.
impl Executor for Arc<Inner> {
    fn builtin_runner(&self, intent: &Value, identity: Option<&RunIdentity>) -> Result<Execution> {
        let subject = string(intent, "subject")?.to_owned();
        let operation = string(intent, "operation")?;
        if operation.ends_with("#inspect-run.v1") {
            let running = self.runs.lock().unwrap().get(&subject).is_some_and(|r| {
                !r.service.is_closed()
                    && !r.service.peer().is_transport_closed()
                    && !r.group.stopped.load(Ordering::SeqCst)
            });
            return Ok(Execution::SetupReport(
                json!({"kind":"inspect-run","state":if running {"running"} else {"stopped"}}),
            ));
        }
        if operation.ends_with("#stop.v1") {
            if let Some(run) = self.runs.lock().unwrap().remove(&subject) {
                run.group.stop();
            }
            return Ok(Execution::SetupReport(
                json!({"kind":"stop","outcome":"succeeded"}),
            ));
        }
        if !operation.ends_with("#run.v1") {
            return Ok(Execution::Missing);
        }
        let identity = identity
            .ok_or("runner needs materialized identity")?
            .environment
            .clone();
        let intent = intent.clone();
        let this = self.clone();
        self.handle.spawn(async move {
            let invocation = intent["invocation"].as_str().unwrap();
            let outcome = match this.start(&intent, identity).await {
                Ok(()) => "succeeded",
                Err(error) => {
                    this.errors.lock().unwrap().push(error);
                    "failed"
                }
            };
            let executor = this
                .completion_executor
                .lock()
                .unwrap()
                .as_ref()
                .and_then(Weak::upgrade);
            if let Some(shell) = this.shell.upgrade()
                && let Err(e) = shell.setup_report(
                    invocation,
                    json!({"kind":"run","outcome":outcome}),
                    executor.as_deref().unwrap_or(&this),
                )
            {
                this.errors.lock().unwrap().push(e);
            }
        });
        Ok(Execution::Deferred)
    }
    fn dispatch(&self, intent: &Value) -> Result<Execution> {
        self.dispatch_later(intent, false)
    }
    fn refresh_tools_and_call(&self, intent: &Value) -> Result<Execution> {
        self.dispatch_later(intent, true)
    }
    fn cancel(&self, intent: &Value) -> Result<Execution> {
        let key = attempt(intent)?;
        self.cancelled_attempts.lock().unwrap().insert(key.clone());
        let request = self.requests.lock().unwrap().get(&key).cloned();
        if let Some((peer, id)) = request {
            self.handle.spawn(async move {
                let _ = notify_cancel(&peer, id).await;
            });
        }
        Ok(Execution::Done)
    }
    fn cancel_setup_parts(&self, intent: &Value) -> Result<Execution> {
        let setup = string(intent, "invocation")?;
        self.cancelled_setups.lock().unwrap().insert(setup.into());
        for run in self
            .runs
            .lock()
            .unwrap()
            .values()
            .filter(|r| r.setup == setup)
        {
            run.group.stop();
        }
        if let Some(groups) = self.pending_groups.lock().unwrap().get(setup) {
            for group in groups {
                group.stop();
            }
        }
        Ok(Execution::Done)
    }
}
impl Inner {
    fn spawn(
        &self,
        program: &Path,
        args: &[String],
        cwd: &Path,
        env: &BTreeMap<String, String>,
        setup: &str,
    ) -> Result<(Child, Arc<ProcessGroup>)> {
        let shell = self.shell.upgrade().ok_or("shell stopped")?;
        let cancelled = self.cancelled_setups.lock().unwrap();
        if self.stopping.load(Ordering::SeqCst) || cancelled.contains(setup) {
            return Err("setup cancelled".into());
        }
        let (engine_end, guard_end) = UnixStream::pair().map_err(|_| "lifetime pipe failed")?;
        // Keep the source away from stdio and FD 3, which Command sets up before pre_exec.
        let fd = unsafe { libc::fcntl(guard_end.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 10) };
        if fd < 0 {
            return Err("lifetime descriptor failed".into());
        }
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };
        let mut command = Command::new(&self.guard);
        command
            .arg("__run-guard")
            .arg(
                serde_json::to_string(&env.keys().collect::<Vec<_>>())
                    .map_err(|_| "guard environment encoding failed")?,
            )
            .arg(program)
            .args(args)
            .env_clear()
            .envs(env)
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .process_group(0);
        unsafe {
            command.pre_exec(move || {
                if libc::dup2(fd.as_raw_fd(), 3) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let mut command = tokio::process::Command::from(command);
        let child = command
            .kill_on_drop(true)
            .spawn()
            .map_err(|_| "guard spawn failed")?;
        let group = Arc::new(ProcessGroup {
            pid: child.id().ok_or("guard has no PID")? as i32,
            stopped: AtomicBool::new(false),
            lifetime: Mutex::new(Some(engine_end)),
        });
        let mut groups = shell.run_groups.lock().unwrap();
        groups.retain(|g| !g.stopped.load(Ordering::SeqCst));
        groups.push(group.clone());
        drop(groups);
        let mut pending = self.pending_groups.lock().unwrap();
        pending.retain(|_, groups| {
            groups.retain(|g| !g.stopped.load(Ordering::SeqCst));
            !groups.is_empty()
        });
        pending.entry(setup.into()).or_default().push(group.clone());
        drop(cancelled);
        Ok((child, group))
    }
    async fn start(&self, intent: &Value, identity: BTreeMap<String, String>) -> Result<()> {
        let subject = string(intent, "subject")?;
        let setup = string(intent, "invocation")?;
        let inputs = &intent["inputs"];
        let (config, state) = {
            let shell = self.shell.upgrade().ok_or("shell stopped")?;
            (shell.config.clone(), shell.state()?)
        };
        let names = ["HOME", "LOGNAME", "PATH", "SHELL", "TERM", "USER"]
            .into_iter()
            .map(str::to_owned)
            .chain(config.pass_environment.iter().cloned())
            .collect::<BTreeSet<_>>();
        let inherited = names
            .into_iter()
            .filter_map(|name| std::env::var(&name).ok().map(|value| (name, value)));
        let mut helper_env = run_environment(
            inherited,
            &config.pass_environment,
            &Value::Null,
            &json!({}),
            &BTreeMap::new(),
        )?;
        for name in identity.keys() {
            helper_env.remove(name);
        }
        let env = run_environment(
            helper_env.clone().into_iter(),
            &config.pass_environment,
            &inputs["item"],
            inputs,
            &identity,
        )?;
        let work = self.directory.join("work").join(Uuid::new_v4().to_string());
        private_directory(&work)?;
        let item = &inputs["item"];
        let (program, mut args) = if item.get("package").is_some() {
            let package = &item["package"];
            if package["registryType"] != "npm" {
                return Err(format!(
                    "unsupported registryType: {}",
                    package["registryType"]
                ));
            }
            if package["transport"]["type"] != "stdio" {
                return Err("built-in runner requires stdio package transport".into());
            }
            let path = self.npm_package(package, setup, &helper_env).await?;
            let manifest: Value = serde_json::from_slice(
                &fs::read(path.join("package.json")).map_err(|_| "npm package manifest missing")?,
            )
            .map_err(|_| "invalid npm package manifest")?;
            let bin = manifest["bin"]
                .as_str()
                .or_else(|| {
                    manifest["bin"]
                        .as_object()
                        .filter(|m| m.len() == 1)
                        .and_then(|m| m.values().next())
                        .and_then(Value::as_str)
                })
                .ok_or("npm package must have exactly one bin")?;
            let bin = program_path(&path, bin)?;
            (
                PathBuf::from("node"),
                vec![bin.to_str().ok_or("non-UTF8 bin")?.to_owned()],
            )
        } else {
            self.command(item, subject, &state)?
        };
        args.extend(package_arguments(item, inputs)?);
        let (mut child, group) = self.spawn(&program, &args, &work, &env, setup)?;
        // Historical diagnostics only. Never use persisted PIDs to inspect or kill a
        // process after reopening: the OS may have reused them for another process.
        crate::store::atomic_write(
            &self.directory.join("run-groups"),
            &Uuid::new_v4().to_string(),
            &serde_json::to_vec(&json!({"engine":std::process::id(),"group":group.pid}))
                .map_err(|_| "process group diagnostic encoding failed")?,
        )?;
        let responses = Arc::new(Mutex::new(HashMap::new()));
        let transport = Wire {
            reader: BufReader::new(child.stdout.take().ok_or("missing stdout")?).lines(),
            writer: Arc::new(AsyncMutex::new(child.stdin.take())),
            responses: responses.clone(),
        };
        let initialized =
            tokio::time::timeout(config.initialize_timeout, ().serve(transport)).await;
        let service = match initialized {
            Ok(Ok(service)) => service,
            _ => {
                group.stop();
                let _ = child.wait().await;
                return Err("MCP initialize failed or deadline passed".into());
            }
        };
        responses.lock().unwrap().clear();
        let cancelled = self.cancelled_setups.lock().unwrap();
        if self.stopping.load(Ordering::SeqCst) || cancelled.contains(setup) {
            group.stop();
            return Err("setup cancelled".into());
        }
        let run = Arc::new(Run {
            service,
            group,
            responses,
            setup: setup.into(),
        });
        if let Some(previous) = self
            .runs
            .lock()
            .unwrap()
            .insert(subject.into(), run.clone())
        {
            previous.group.stop();
        }
        drop(cancelled);
        // Reap the guard after lifetime termination; keep it owned while the session runs.
        let group = run.group.clone();
        self.handle.spawn(async move {
            let _ = child.wait().await;
            group.stop();
        });
        Ok(())
    }
    fn command(
        &self,
        item: &Value,
        subject: &str,
        state: &conduit_core::State,
    ) -> Result<(PathBuf, Vec<String>)> {
        let command = item["command"]
            .as_array()
            .ok_or("item has no command or package")?;
        let raw = command
            .first()
            .and_then(Value::as_str)
            .ok_or("empty command")?;
        // The resolver expands relative programs. Recover their original path from the
        // selected installed snapshot; never execute an expanded path outside the store.
        let object = state
            .objects
            .iter()
            .find(|o| o["id"] == subject)
            .ok_or("run subject missing")?;
        let selected = object["resolved_implementations"]
            .as_object()
            .and_then(|m| m.values().find(|v| &v["item"] == item));
        let original = selected
            .and_then(|v| v["selection"].as_str())
            .and_then(|q| q.rsplit_once('#'));
        let held = selected.map(|v| &object["held"]["contexts"][v["scope"].as_str().unwrap_or("")]);
        let installed = original.and_then(|(address, name)| {
            state.installed_packages.iter().find_map(|p| {
                let cmd = &p.package.project.manifest.as_ref()?["implementations"][name]["command"];
                (p.package.address == address
                    && held.is_some_and(|h| {
                        h["version"] == p.package.version && h["digest"] == p.package.digest
                    }))
                .then_some((p, cmd))
            })
        });
        let was_relative = selected.is_some_and(|v| {
            object["held"]["contexts"][v["scope"].as_str().unwrap_or("")]["relative_programs"]
                .as_array()
                .is_some_and(|names| {
                    v["selection"]
                        .as_str()
                        .and_then(|q| q.rsplit_once('#'))
                        .is_some_and(|(_, name)| names.contains(&json!(name)))
                })
        });
        let program = if let Some((package, cmd)) = installed.filter(|(_, cmd)| {
            cmd[0]
                .as_str()
                .is_some_and(|s| s.contains('/') && !Path::new(s).is_absolute())
        }) {
            let root = self.materialize_package(package)?;
            program_path(&root, cmd[0].as_str().unwrap())?
        } else if was_relative {
            return Err("relative program has no installed package files".into());
        } else if Path::new(raw).is_absolute() || !raw.contains('/') {
            PathBuf::from(raw)
        } else {
            return Err("relative program has no installed package files".into());
        };
        let args = command[1..]
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| "non-string command argument".into())
            })
            .collect::<Result<_>>()?;
        Ok((program, args))
    }
    fn materialize_package(&self, package: &InstalledPackage) -> Result<PathBuf> {
        let root = self
            .directory
            .join("programs")
            .join(Uuid::new_v4().to_string());
        private_directory(&root)?;
        for (name, member) in &package.files {
            let path = safe_join(&root, name)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|_| "program directory failed")?;
            }
            match member {
                Member::Directory => {
                    fs::create_dir_all(&path).map_err(|_| "program directory failed")?
                }
                Member::File { bytes, mode } => {
                    fs::write(&path, bytes).map_err(|_| "program write failed")?;
                    fs::set_permissions(&path, fs::Permissions::from_mode(*mode & 0o777))
                        .map_err(|_| "program mode failed")?;
                }
                Member::Link { target } => {
                    // Archive admission validated containment, including link expansion.
                    // Targets are package-root-relative and retain `..`: normalizing it
                    // here would change traversal when a preceding component is a link.
                    if Path::new(target).is_absolute() {
                        return Err("absolute package link target".into());
                    }
                    std::os::unix::fs::symlink(root.join(target), &path)
                        .map_err(|_| "program link failed")?;
                }
            }
        }
        Ok(root)
    }
    async fn helper(
        &self,
        args: &[String],
        cwd: &Path,
        env: &BTreeMap<String, String>,
        setup: &str,
    ) -> Result<Vec<u8>> {
        let mut args = args.to_vec();
        args.extend([
            "--cache".into(),
            self.directory
                .join("npm-downloads")
                .to_str()
                .ok_or("non-UTF8 npm cache")?
                .into(),
            format!(
                "--userconfig={}",
                self.directory.join("npm-userconfig").display()
            ),
            "--globalconfig=/dev/null".into(),
            "--no-audit".into(),
            "--no-fund".into(),
            "--registry=https://registry.npmjs.org".into(),
        ]);
        let (mut child, group) = self.spawn(Path::new("npm"), &args, cwd, env, setup)?;
        child.stdin.take();
        // Guard keeps running until its lifetime ends. Exit status uses the lifetime
        // socket in the opposite direction; stdout belongs only to npm.
        let mut stdout = child.stdout.take().ok_or("npm stdout missing")?;
        let mut bytes = Vec::new();
        use tokio::io::AsyncReadExt;
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(120),
            stdout.read_to_end(&mut bytes),
        )
        .await;
        let socket = group
            .lifetime
            .lock()
            .unwrap()
            .as_ref()
            .ok_or("npm helper stopped")?
            .try_clone()
            .map_err(|_| "npm status pipe failed")?;
        socket
            .set_nonblocking(true)
            .map_err(|_| "npm status pipe failed")?;
        let mut socket =
            tokio::net::UnixStream::from_std(socket).map_err(|_| "npm status pipe failed")?;
        let status =
            tokio::time::timeout(std::time::Duration::from_secs(5), socket.read_i32()).await;
        group.stop();
        let _ = child.wait().await;
        match (result, status) {
            (Ok(Ok(_)), Ok(Ok(0))) => Ok(bytes),
            (Ok(Ok(_)), Ok(Ok(code))) => Err(format!("npm helper failed with exit status {code}")),
            _ => Err("npm helper failed or deadline passed".into()),
        }
    }
    async fn npm_package(
        &self,
        package: &Value,
        setup: &str,
        env: &BTreeMap<String, String>,
    ) -> Result<PathBuf> {
        let _cache = self.cache_lock.lock().await;
        let hash = string(package, "fileSha256")?;
        if hash.len() != 64 || !hash.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("invalid npm fileSha256".into());
        }
        let cache = self.directory.join("npm").join(hash);
        if cache.join("ready").is_file() {
            return Ok(cache.join("package"));
        }
        if cache.exists() {
            fs::remove_dir_all(&cache).map_err(|_| "npm cache cleanup failed")?;
        }
        private_directory(&cache)?;
        let identifier = string(package, "identifier")?;
        let version = string(package, "version")?;
        let bytes = self
            .helper(
                &[
                    "pack".into(),
                    format!("{identifier}@{version}"),
                    "--ignore-scripts".into(),
                    "--json".into(),
                ],
                &cache,
                env,
                setup,
            )
            .await?;
        let packed: Value = serde_json::from_slice(&bytes).map_err(|_| "npm pack failed")?;
        let filename = packed[0]["filename"].as_str().ok_or("npm pack failed")?;
        let artifact = fs::read(safe_join(&cache, filename)?).map_err(|_| "npm archive missing")?;
        // Before decompression, manifest reading, dependency installation or execution.
        if format!("{:x}", Sha256::digest(&artifact)) != hash {
            return Err("npm artifact SHA-256 mismatch".into());
        }
        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(artifact.as_slice()));
        archive
            .unpack(&cache)
            .map_err(|_| "npm archive extraction failed")?;
        let path = cache.join("package");
        self.helper(
            &[
                "install".into(),
                "--omit=dev".into(),
                "--ignore-scripts".into(),
                "--legacy-peer-deps".into(),
                "--json".into(),
            ],
            &path,
            env,
            setup,
        )
        .await?;
        // Only the helper's successful exit marks the cache ready. Packages without
        // dependencies need not create node_modules.
        fs::write(
            cache.join("ready"),
            b"verified artifact; dependencies are not pinned\n",
        )
        .map_err(|_| "npm cache marker failed")?;
        Ok(path)
    }
}

fn string<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v[key].as_str().ok_or_else(|| format!("runner needs {key}"))
}
fn attempt(intent: &Value) -> Result<Attempt> {
    Ok((
        string(intent, "invocation")?.into(),
        intent["attempt"].as_u64().ok_or("runner needs attempt")?,
    ))
}
fn safe_join(root: &Path, name: &str) -> Result<PathBuf> {
    let path = Path::new(name);
    if path.is_absolute()
        || path
            .components()
            .any(|c| !matches!(c, Component::Normal(_) | Component::CurDir))
    {
        return Err("unsafe package file path".into());
    }
    Ok(root.join(path))
}

fn program_path(root: &Path, name: &str) -> Result<PathBuf> {
    if Path::new(name).is_absolute() {
        return Err("package program must be relative".into());
    }
    let path = root.join(name);
    // Preserve filesystem traversal through symlinks and parent components, while
    // requiring the final program to come from the installed package's own files.
    let resolved = path.canonicalize().map_err(|_| "package program missing")?;
    let root = root
        .canonicalize()
        .map_err(|_| "package directory missing")?;
    if !resolved.starts_with(root) || !resolved.is_file() {
        return Err("package program is outside installed files".into());
    }
    Ok(path)
}

fn run_environment(
    inherited: impl Iterator<Item = (String, String)>,
    pass: &[String],
    item: &Value,
    inputs: &Value,
    identity: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>> {
    let mut env = inherited
        .filter(|(k, v)| {
            (["HOME", "LOGNAME", "PATH", "SHELL", "TERM", "USER"].contains(&k.as_str())
                || pass.contains(k))
                && !v.starts_with("()")
        })
        .collect::<BTreeMap<_, _>>();
    for declaration in item["package"]["environmentVariables"]
        .as_array()
        .into_iter()
        .flatten()
    {
        let name = string(declaration, "name")?;
        let supplied = inputs["secrets"]["environment"]
            .get(name)
            .or_else(|| inputs["environment"].get(name));
        if let Some(value) = package_input(declaration, supplied)? {
            env.insert(name.into(), value);
        }
    }
    for values in [&inputs["environment"], &inputs["secrets"]["environment"]] {
        for (name, value) in values.as_object().into_iter().flatten() {
            env.insert(
                name.clone(),
                value
                    .as_str()
                    .ok_or("run environment must be strings")?
                    .into(),
            );
        }
    }
    env.extend(identity.clone());
    Ok(env)
}

// server.json InputWithVariables: substitute declared identifiers once, leaving
// unresolved braces intact. Supplied bindings are whole values, not variable bindings.
fn package_input(input: &Value, supplied: Option<&Value>) -> Result<Option<String>> {
    let value = supplied
        .or_else(|| input.get("value"))
        .or_else(|| input.get("default"));
    let Some(value) = value else {
        if input["isRequired"] == true {
            return Err("required package input missing".into());
        }
        return Ok(None);
    };
    let value = value.as_str().ok_or("package input must be string")?;
    if supplied.is_some() {
        return Ok(Some(value.into()));
    }
    let mut resolved = String::new();
    for part in value.split_inclusive('}') {
        if let Some(body) = part.strip_suffix('}')
            && let Some((prefix, name)) = body.rsplit_once('{')
            && let Some(variable) = input["variables"].get(name)
            && let Some(replacement) = package_input(variable, None)?
        {
            resolved.push_str(prefix);
            resolved.push_str(&replacement);
        } else {
            resolved.push_str(part);
        }
    }
    Ok(Some(resolved))
}

fn package_arguments(item: &Value, inputs: &Value) -> Result<Vec<String>> {
    let mut bound = inputs["packageArguments"]
        .as_object()
        .cloned()
        .unwrap_or_default();
    bound.extend(
        inputs["secrets"]["packageArguments"]
            .as_object()
            .cloned()
            .unwrap_or_default(),
    );
    let mut args = Vec::new();
    for argument in item["package"]["packageArguments"]
        .as_array()
        .into_iter()
        .flatten()
    {
        let positional = argument["type"] == "positional";
        let key = if positional {
            argument["valueHint"].as_str()
        } else {
            Some(string(argument, "name")?)
        };
        if let Some(value) = package_input(argument, key.and_then(|key| bound.get(key)))? {
            if !positional {
                args.push(key.unwrap().into());
            }
            args.push(value);
        }
    }
    Ok(args)
}

// rmcp adds progress/client metadata to requests. An extension restores the core's exact
// supplied method/params at the byte boundary; SDK bookkeeping must never reach plain MCP.
#[derive(Clone)]
struct ExactCall(Value);
struct Wire {
    reader: tokio::io::Lines<BufReader<ChildStdout>>,
    writer: Arc<AsyncMutex<Option<ChildStdin>>>,
    responses: Responses,
}
impl Transport<RoleClient> for Wire {
    type Error = std::io::Error;
    fn send(
        &mut self,
        item: TxJsonRpcMessage<RoleClient>,
    ) -> impl Future<Output = std::io::Result<()>> + Send + 'static {
        let writer = self.writer.clone();
        let mut value = serde_json::to_value(&item).expect("MCP serialization");
        if let JsonRpcMessage::Request(request) = &item
            && let Some(original) = request.request.extensions().get::<ExactCall>()
        {
            self.responses
                .lock()
                .unwrap()
                .insert(request.id.clone(), Value::Null);
            value["method"] = original.0["method"].clone();
            if let Some(params) = original.0.get("params") {
                value["params"] = params.clone();
            } else {
                value.as_object_mut().unwrap().remove("params");
            }
        }
        async move {
            let mut bytes = serde_json::to_vec(&value)?;
            bytes.push(b'\n');
            let mut writer = writer.lock().await;
            let writer = writer.as_mut().ok_or(std::io::ErrorKind::BrokenPipe)?;
            writer.write_all(&bytes).await?;
            writer.flush().await
        }
    }
    async fn receive(&mut self) -> Option<RxJsonRpcMessage<RoleClient>> {
        loop {
            let line = self.reader.next_line().await.ok()??;
            let Ok(value) = serde_json::from_str::<Value>(&line) else {
                continue;
            };
            if value.get("method").is_none()
                && let Ok(id) = serde_json::from_value::<RequestId>(value["id"].clone())
                && let Some(slot) = self.responses.lock().unwrap().get_mut(&id)
            {
                *slot = value.clone();
            }
            if let Ok(message) = serde_json::from_value(value) {
                return Some(message);
            }
        }
    }
    async fn close(&mut self) -> std::io::Result<()> {
        self.writer.lock().await.take();
        Ok(())
    }
}

async fn notify_cancel(peer: &Peer<RoleClient>, id: RequestId) -> Result<()> {
    let notification = serde_json::from_value(
        json!({"method":"notifications/cancelled","params":{"requestId":id}}),
    )
    .map_err(|_| "cancellation encoding failed")?;
    peer.send_notification(notification)
        .await
        .map_err(|_| "cancellation send failed".into())
}
impl Inner {
    fn dispatch_later(self: &Arc<Self>, intent: &Value, refresh: bool) -> Result<Execution> {
        let key = attempt(intent)?;
        let run = self
            .runs
            .lock()
            .unwrap()
            .get(string(intent, "target")?)
            .cloned();
        let Some(run) = run else {
            return Ok(Execution::Missing);
        };
        let timeout = self
            .shell
            .upgrade()
            .ok_or("shell stopped")?
            .config
            .call_timeout;
        let intent = intent.clone();
        let this = self.clone();
        self.handle.spawn(async move {
            let completion = tokio::time::timeout(timeout, async {
                if refresh {
                    let refreshed = this
                        .call(&run, &key, json!({"method":"tools/list","params":{}}))
                        .await?;
                    if refreshed["kind"] != "result" {
                        return Err("tool refresh failed".into());
                    }
                }
                this.call(&run, &key, intent["call"].clone()).await
            })
            .await
            .ok()
            .and_then(std::result::Result::ok)
            .unwrap_or_else(|| json!({"kind":"missing"}));
            if let Some((_, id)) = this.requests.lock().unwrap().remove(&key) {
                run.responses.lock().unwrap().remove(&id);
            }
            let executor = this
                .completion_executor
                .lock()
                .unwrap()
                .as_ref()
                .and_then(Weak::upgrade);
            if let Some(shell) = this.shell.upgrade()
                && let Err(e) = shell.complete(
                    &key.0,
                    key.1,
                    completion,
                    executor.as_deref().unwrap_or(&this),
                )
            {
                this.errors.lock().unwrap().push(e);
            }
        });
        Ok(Execution::Deferred)
    }
    async fn call(&self, run: &Run, key: &Attempt, call: Value) -> Result<Value> {
        if run.group.stopped.load(Ordering::SeqCst) || run.service.is_closed() {
            return Err("run stopped".into());
        }
        if self.cancelled_attempts.lock().unwrap().contains(key) {
            return Err("attempt cancelled".into());
        }
        let mut request: ClientRequest =
            serde_json::from_value(call.clone()).map_err(|_| "invalid intent call")?;
        request.extensions_mut().insert(ExactCall(call));
        let peer = run.service.peer().clone();
        let handle = peer
            .send_cancellable_request(request, PeerRequestOptions::no_options())
            .await
            .map_err(|_| "run missing")?;
        let id = handle.id.clone();
        self.requests
            .lock()
            .unwrap()
            .insert(key.clone(), (peer.clone(), id.clone()));
        if self.cancelled_attempts.lock().unwrap().contains(key) {
            notify_cancel(&peer, id.clone()).await?;
        }
        let response = handle.await_response().await;
        self.requests.lock().unwrap().remove(key);
        if let Some(raw) = run.responses.lock().unwrap().remove(&id) {
            if raw.get("result").is_some() {
                return Ok(json!({"kind":"result","result":raw["result"]}));
            }
            if let Some(code) = raw["error"]["code"].as_i64() {
                let mut completion = json!({"kind":"protocol-error","code":code});
                if let Some(data) = raw["error"].get("data") {
                    completion["data"] = data.clone();
                }
                return Ok(completion);
            }
        }
        if matches!(
            response,
            Err(rmcp::service::ServiceError::TransportClosed
                | rmcp::service::ServiceError::TransportSend(_))
        ) {
            run.group.stop();
        }
        Err("run missing or malformed reply".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_runner(parent: &Path) -> (Arc<Shell>, Runner) {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let shell = Arc::new(
            Shell::open(
                &parent.join("store"),
                crate::boundary::context(&repo).unwrap(),
                crate::Config::default(),
            )
            .unwrap(),
        );
        let executable = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("conduit");
        let runner = Runner::new(&shell, executable).unwrap();
        (shell, runner)
    }

    fn exec_output(
        runner: &Runner,
        program: &Path,
        args: &[String],
        cwd: &Path,
        env: &BTreeMap<String, String>,
    ) -> String {
        runner._runtime.block_on(async {
            let (mut child, group) = runner
                .inner
                .spawn(program, args, cwd, env, "test-exec")
                .unwrap();
            child.stdin.take();
            use tokio::io::AsyncReadExt;
            let mut bytes = String::new();
            tokio::time::timeout(
                std::time::Duration::from_secs(5),
                child.stdout.take().unwrap().read_to_string(&mut bytes),
            )
            .await
            .unwrap()
            .unwrap();
            group.stop();
            let _ = child.wait().await;
            bytes
        })
    }

    fn installed_command_state(
        item: &Value,
        original: &str,
        files: BTreeMap<String, Member>,
    ) -> conduit_core::State {
        use conduit_core::packages::{Project, Snapshot, SnapshotSource};
        let mut state = conduit_core::State::default();
        let relative = original.contains('/') && !Path::new(original).is_absolute();
        state.objects.push(json!({"id":"subject","resolved_implementations":{"op":{"item":item,"selection":"example.test/provider#worker","scope":"snapshot-0"}},"held":{"contexts":{"snapshot-0":{"relative_programs":if relative {vec!["worker"]} else {vec![]},"version":"1.0.0","digest":"sha256:test"}}}}));
        state.installed_packages.push(InstalledPackage {
            package: Snapshot {
                source: SnapshotSource::PreinstalledSnapshot,
                address: "example.test/provider".into(),
                version: "1.0.0".into(),
                digest: "sha256:test".into(),
                project: Project {
                    manifest: Some(json!({"implementations":{"worker":{"command":[original]}}})),
                    documents: vec![],
                    directory: None,
                },
            },
            files,
        });
        state
    }

    #[test]
    fn installed_bare_command_uses_permitted_path_not_package_files() {
        let parent = tempfile::tempdir().unwrap();
        let (_shell, runner) = test_runner(parent.path());
        let bin = parent.path().join("permitted-bin");
        fs::create_dir(&bin).unwrap();
        std::os::unix::fs::symlink("/bin/sh", bin.join("sh")).unwrap();
        let item = json!({"command":["sh","-c","printf permitted-path"]});
        let state = installed_command_state(
            &item,
            "sh",
            BTreeMap::from([(
                "sh".into(),
                Member::File {
                    bytes: b"#!/bin/sh\nprintf package-local".to_vec(),
                    mode: 0o755,
                },
            )]),
        );
        let (program, args) = runner.inner.command(&item, "subject", &state).unwrap();
        assert_eq!(program, Path::new("sh"));
        assert_eq!(
            exec_output(
                &runner,
                &program,
                &args,
                parent.path(),
                &BTreeMap::from([("PATH".into(), bin.to_str().unwrap().into()),])
            ),
            "permitted-path"
        );
        assert_eq!(
            fs::read_dir(parent.path().join("store/programs"))
                .unwrap()
                .count(),
            0
        );
    }

    #[test]
    fn installed_relative_command_executes_through_parent_symlink_target() {
        let parent = tempfile::tempdir().unwrap();
        let (_shell, runner) = test_runner(parent.path());
        let item = json!({"command":["/old-install/bin/worker"]});
        let mut state = installed_command_state(
            &item,
            "./bin/worker",
            BTreeMap::from([
                ("bin".into(), Member::Directory),
                (
                    "bin/worker".into(),
                    Member::Link {
                        target: "bin/../lib/worker".into(),
                    },
                ),
                ("lib".into(), Member::Directory),
                (
                    "lib/worker".into(),
                    Member::File {
                        bytes: b"#!/bin/sh\nprintf installed-link".to_vec(),
                        mode: 0o755,
                    },
                ),
            ]),
        );
        let (program, args) = runner.inner.command(&item, "subject", &state).unwrap();
        assert_eq!(
            exec_output(&runner, &program, &args, parent.path(), &BTreeMap::new()),
            "installed-link"
        );
        // A parent component after another link follows that link before traversal.
        state.installed_packages[0].files.insert(
            "bin/through".into(),
            Member::Link {
                target: "lib/inner".into(),
            },
        );
        state.installed_packages[0]
            .files
            .insert("lib/inner".into(), Member::Directory);
        state.installed_packages[0].files.insert(
            "bin/worker".into(),
            Member::Link {
                target: "bin/through/../worker".into(),
            },
        );
        let (program, args) = runner.inner.command(&item, "subject", &state).unwrap();
        assert_eq!(
            exec_output(&runner, &program, &args, parent.path(), &BTreeMap::new()),
            "installed-link"
        );
        // The final executable must still be inside the installed package.
        fs::write(parent.path().join("store/outside-worker"), b"outside").unwrap();
        state.installed_packages[0].files.insert(
            "bin/worker".into(),
            Member::Link {
                target: "../../outside-worker".into(),
            },
        );
        assert!(
            runner
                .inner
                .command(&item, "subject", &state)
                .unwrap_err()
                .contains("outside installed files")
        );
    }

    fn exec_package_arguments(item: &Value) -> String {
        let parent = tempfile::tempdir().unwrap();
        let (_shell, runner) = test_runner(parent.path());
        let mut args = vec![
            "-c".into(),
            "printf '%s\\n' \"$@\"".into(),
            "test-argv".into(),
        ];
        args.extend(package_arguments(item, &json!({})).unwrap());
        exec_output(
            &runner,
            Path::new("/bin/sh"),
            &args,
            parent.path(),
            &BTreeMap::new(),
        )
    }

    #[test]
    fn fixed_positional_argument_needs_no_value_hint() {
        assert_eq!(
            exec_package_arguments(&json!({"package":{"packageArguments":[
                {"type":"positional","value":"serve","isRequired":true},
                {"type":"positional","value":""}
            ]}})),
            "serve\n\n"
        );
    }

    #[test]
    fn fixed_argument_substitutes_declared_variables_once_and_preserves_unresolved() {
        assert_eq!(
            exec_package_arguments(&json!({"package":{"packageArguments":[
                {"type":"named","name":"--endpoint","value":"https://{tenant}.example.com/{unknown}/{tenant}/{region}/{unset}","variables":{
                    "tenant":{"value":"acme"},"region":{"default":"west"},"unset":{}
                }},
                {"type":"positional","valueHint":"once","value":"{a}/{b}","variables":{"a":{"value":"{b}"},"b":{"value":"literal"}}},
                {"type":"positional","valueHint":"unresolved","value":"{untouched}"}
            ]}})),
            "--endpoint\nhttps://acme.example.com/{unknown}/acme/west/{unset}\n{b}/literal\n{untouched}\n"
        );
    }

    #[test]
    fn package_environment_values_defaults_and_precedence_at_exec() {
        let parent = tempfile::tempdir().unwrap();
        let (_shell, runner) = test_runner(parent.path());
        let item = json!({"package":{"environmentVariables":[
            {"name":"LOG_LEVEL","value":"warn"},
            {"name":"MODE","default":"quiet"},
            {"name":"BOUND","default":"unbound"},
            {"name":"SECRET","default":"unbound","isRequired":true},
            {"name":"HOME","value":"/package-home"},
            {"name":"CONDUIT_URL","value":"package-url"},
            {"name":"ENDPOINT","value":"https://{tenant}.example.com/{unknown}","variables":{"tenant":{"value":"acme"}}},
            {"name":"OMITTED"}
        ]}});
        let inputs = json!({"environment":{"BOUND":"configured","SECRET":"public","CONDUIT_URL":"configured-url"},"secrets":{"environment":{"SECRET":"synthetic-secret"}}});
        let identity = BTreeMap::from([("CONDUIT_URL".into(), "identity-url".into())]);
        let env = run_environment(
            BTreeMap::from([
                ("HOME".into(), "/inherited-home".into()),
                ("GITHUB_TOKEN".into(), "synthetic-test-sentinel".into()),
            ])
            .into_iter(),
            &[],
            &item,
            &inputs,
            &identity,
        )
        .unwrap();
        let output = exec_output(&runner, Path::new("/usr/bin/env"), &[], parent.path(), &env);
        let actual = output
            .lines()
            .map(|line| {
                let (name, value) = line.split_once('=').unwrap();
                (name.to_owned(), value.to_owned())
            })
            .collect::<BTreeMap<_, _>>();
        assert_eq!(
            actual,
            BTreeMap::from([
                ("LOG_LEVEL".into(), "warn".into()),
                ("MODE".into(), "quiet".into()),
                ("BOUND".into(), "configured".into()),
                ("SECRET".into(), "synthetic-secret".into()),
                ("HOME".into(), "/package-home".into()),
                ("CONDUIT_URL".into(), "identity-url".into()),
                (
                    "ENDPOINT".into(),
                    "https://acme.example.com/{unknown}".into()
                ),
            ])
        );
    }

    #[test]
    fn exact_environment_precedence_and_argument_binding() {
        let inherited = BTreeMap::from([
            ("HOME".into(), "/home/test".into()),
            ("PATH".into(), "() shell function".into()),
            ("GITHUB_TOKEN".into(), "synthetic-test-sentinel".into()),
            ("EXPLICIT".into(), "allowed".into()),
            ("CONDUIT_URL".into(), "wrong".into()),
        ]);
        let identity = BTreeMap::from([
            ("CONDUIT_URL".into(), "http://127.0.0.1:3000/mcp".into()),
            ("CONDUIT_IDENTITY_DIR".into(), "/private/test".into()),
        ]);
        let env=run_environment(inherited.into_iter(), &["EXPLICIT".into(),"CONDUIT_URL".into()], &Value::Null, &json!({"environment":{"HOME":"bound-home","CONDUIT_URL":"also-wrong"},"secrets":{"environment":{"SECRET":"synthetic-secret"}}}), &identity).unwrap();
        assert_eq!(
            env,
            BTreeMap::from([
                ("HOME".into(), "bound-home".into()),
                ("EXPLICIT".into(), "allowed".into()),
                ("SECRET".into(), "synthetic-secret".into()),
                ("CONDUIT_URL".into(), "http://127.0.0.1:3000/mcp".into()),
                ("CONDUIT_IDENTITY_DIR".into(), "/private/test".into())
            ])
        );
        let item = json!({"package":{"packageArguments":[{"type":"positional","valueHint":"root","isRequired":true},{"type":"named","name":"--label","default":"default"}],"runtimeArguments":[{"type":"named","name":"--must-not-pass","value":"x"}]}});
        assert_eq!(
            package_arguments(&item, &json!({"packageArguments":{"root":"/held-root"}})).unwrap(),
            vec!["/held-root", "--label", "default"]
        );
    }
    #[test]
    fn exec_boundary_passes_exact_environment() {
        let parent = tempfile::tempdir().unwrap();
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let shell = Arc::new(
            Shell::open(
                &parent.path().join("store"),
                crate::boundary::context(&repo).unwrap(),
                crate::Config::default(),
            )
            .unwrap(),
        );
        let executable = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("conduit");
        let runner = Runner::new(&shell, executable).unwrap();
        let env = BTreeMap::from([
            ("HOME".into(), "/test-home".into()),
            ("SYNTHETIC_BOUND".into(), "test-value".into()),
        ]);
        let output = runner._runtime.block_on(async {
            let (mut child, group) = runner
                .inner
                .spawn(
                    Path::new("/usr/bin/env"),
                    &[],
                    parent.path(),
                    &env,
                    "test-environment",
                )
                .unwrap();
            child.stdin.take();
            use tokio::io::AsyncReadExt;
            let mut bytes = String::new();
            child
                .stdout
                .take()
                .unwrap()
                .read_to_string(&mut bytes)
                .await
                .unwrap();
            group.stop();
            let _ = child.wait().await;
            bytes
        });
        let actual = output
            .lines()
            .map(|s| {
                let (k, v) = s.split_once('=').unwrap();
                (k.to_owned(), v.to_owned())
            })
            .collect::<BTreeMap<_, _>>();
        assert_eq!(actual, env);
    }
    #[test]
    fn relative_command_uses_installed_bytes_and_keeps_their_mode() {
        use conduit_core::packages::{Project, Snapshot, SnapshotSource};
        let parent = tempfile::tempdir().unwrap();
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let shell = Arc::new(
            Shell::open(
                &parent.path().join("store"),
                crate::boundary::context(&repo).unwrap(),
                crate::Config::default(),
            )
            .unwrap(),
        );
        let runner = Runner::new(&shell, PathBuf::from("unused-no-process-started")).unwrap();
        let item =
            json!({"command":["/must-not-execute-from-the-old-install/bin/worker","literal"]});
        let mut state = conduit_core::State::default();
        state.objects.push(json!({"id":"subject","resolved_implementations":{"op":{"item":item,"selection":"example.test/provider#worker","scope":"snapshot-0"}},"held":{"contexts":{"snapshot-0":{"relative_programs":["worker"],"version":"1.0.0","digest":"sha256:test"}}}}));
        state.installed_packages.push(InstalledPackage {
            package: Snapshot { source:SnapshotSource::PreinstalledSnapshot,address:"example.test/provider".into(),version:"1.0.0".into(),digest:"sha256:test".into(),project:Project { manifest:Some(json!({"implementations":{"worker":{"command":["./bin/worker","literal"]}}})),documents:vec![],directory:None } },
            files:BTreeMap::from([("bin".into(),Member::Directory),("bin/worker".into(),Member::File {bytes:b"installed test bytes".to_vec(),mode:0o755})]),
        });
        let mut other_release = state.installed_packages[0].clone();
        other_release.package.version = "2.0.0".into();
        other_release.package.digest = "sha256:other".into();
        other_release.files.insert(
            "bin/worker".into(),
            Member::File {
                bytes: b"wrong release".to_vec(),
                mode: 0o755,
            },
        );
        state.installed_packages.insert(0, other_release);
        let (program, args) = runner.inner.command(&item, "subject", &state).unwrap();
        assert!(program.starts_with(parent.path().join("store/programs")));
        assert_eq!(fs::read(&program).unwrap(), b"installed test bytes");
        assert_eq!(
            fs::metadata(&program).unwrap().permissions().mode() & 0o777,
            0o755
        );
        assert_eq!(args, vec!["literal"]);
        let root = program.parent().unwrap().parent().unwrap();
        assert_eq!(
            fs::read(program_path(root, "bin/../bin/worker").unwrap()).unwrap(),
            b"installed test bytes"
        );
        assert!(program_path(root, "../outside").is_err());
        state.installed_packages.clear();
        assert!(
            runner
                .inner
                .command(&item, "subject", &state)
                .unwrap_err()
                .contains("no installed package files")
        );
    }
}
