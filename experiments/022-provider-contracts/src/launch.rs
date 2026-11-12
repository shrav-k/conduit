use crate::{
    form::Descriptor,
    names,
    substitute::{Context, Values},
};
use flate2::read::GzDecoder;
use rmcp::{RoleClient, ServiceExt, model::ClientConfig, service::RunningService};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Cursor, Read},
    path::{Component, Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::{
    io::AsyncReadExt,
    process::{Child, Command},
    task::JoinHandle,
};

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn private_dir(p: &Path) -> Result<(), &'static str> {
    fs::create_dir_all(p).map_err(|_| "cannot create private directory")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(p, fs::Permissions::from_mode(0o700))
            .map_err(|_| "cannot protect directory")?;
    }
    Ok(())
}
pub struct Prepared {
    pub root: PathBuf,
    pub item: Value,
    pub digest: String,
    pub state: PathBuf,
}
pub fn prepare(
    d: &Descriptor,
    base: &Path,
    lab: &Path,
    descriptor_digest: &str,
) -> Result<Prepared, &'static str> {
    let (archive, hash) = d
        .archive
        .rsplit_once('#')
        .ok_or("archive must be ARCHIVE#SHA256")?;
    if hash.len() != 64
        || !hash
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err("invalid archive digest");
    }
    let bytes = fs::read(base.join(archive)).map_err(|_| "cannot read package archive")?;
    if digest(&bytes) != hash {
        return Err("package archive digest mismatch");
    }
    private_dir(lab)?;
    // A new extraction per attempt prevents reuse of files mutated by an earlier run.
    let root = lab
        .join("packages")
        .join(format!("{descriptor_digest}-{hash}"));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|_| "cannot refresh package extraction")?;
    }
    private_dir(&root)?;
    let reader: Box<dyn Read> = if bytes.starts_with(&[0x1f, 0x8b]) {
        Box::new(GzDecoder::new(Cursor::new(bytes)))
    } else {
        Box::new(Cursor::new(bytes))
    };
    for entry in tar::Archive::new(reader)
        .entries()
        .map_err(|_| "invalid package archive")?
    {
        let mut entry = entry.map_err(|_| "invalid archive entry")?;
        let path = entry.path().map_err(|_| "invalid archive path")?;
        if !package_relative(&path)
            || !(entry.header().entry_type().is_file() || entry.header().entry_type().is_dir())
        {
            return Err("archive requires contained regular files and directories");
        }
        if !entry
            .unpack_in(&root)
            .map_err(|_| "archive extraction failed")?
        {
            return Err("archive path escapes package");
        }
    }
    let manifest: Value = serde_yaml::from_slice(
        &fs::read(root.join("conduit.yaml")).map_err(|_| "package manifest missing")?,
    )
    .map_err(|_| "invalid package manifest")?;
    let item = manifest["implementations"]
        .get(&d.item)
        .cloned()
        .ok_or("implementation item missing")?;
    if !crate::form::safe_id(&d.instance) || matches!(d.instance.as_str(), "." | "..") {
        return Err("unsafe instance id");
    }
    let state = lab.join("state").join(descriptor_digest).join(&d.instance);
    private_dir(&state)?;
    Ok(Prepared {
        root,
        item,
        digest: hash.into(),
        state,
    })
}
pub fn environment(d: &Descriptor, p: &Prepared, lab: &Path) -> Result<Values, &'static str> {
    let mut env = Values::new();
    for name in ["HOME", "LOGNAME", "PATH", "SHELL", "TERM", "USER"]
        .into_iter()
        .chain(d.pass_environment.iter().map(String::as_str))
    {
        if let Ok(value) = std::env::var(name)
            && !value.starts_with("()")
        {
            env.insert(name.into(), Value::String(value));
        }
    }
    let empty = Values::new();
    let context = Context {
        descriptor: d,
        fixtures: &empty,
        captures: &empty,
        lab,
    };
    let config = context.expand(&d.config)?;
    if let Some(bindings) = p.item.get("environment") {
        for (name, pointer) in bindings
            .as_object()
            .ok_or("environment bindings must be a map")?
        {
            if [names::ENGINE_URL, names::IDENTITY_DIR, names::STATE_DIR].contains(&name.as_str()) {
                return Err("binding uses a reserved variable");
            }
            let pointer = pointer.as_str().ok_or("binding requires JSON pointer")?;
            if !crate::form::pointer(pointer) {
                return Err("invalid binding pointer");
            }
            let value = config.pointer(pointer).ok_or("binding pointer missing")?;
            let value = match value {
                Value::Null => continue,
                Value::String(s) => s.clone(),
                Value::Bool(_) | Value::Number(_) => serde_json_canonicalizer::to_string(value)
                    .map_err(|_| "binding canonicalization failed")?,
                _ => return Err("binding must select a scalar"),
            };
            env.insert(name.clone(), Value::String(value));
        }
    }
    env.insert(
        names::STATE_DIR.into(),
        Value::String(p.state.to_string_lossy().into_owned()),
    );
    // H1 deliberately makes no identity fixture.
    env.remove(names::ENGINE_URL);
    env.remove(names::IDENTITY_DIR);
    Ok(env)
}
pub struct Run {
    pub service: RunningService<RoleClient, ClientConfig>,
    pub(crate) supported_versions: Value,
    child: Child,
    pub observations: crate::observe::Observations,
    drain: JoinHandle<()>,
    stdout_drain: JoinHandle<()>,
}
impl Run {
    pub async fn start(
        d: &Descriptor,
        p: &Prepared,
        lab: &Path,
        limit: Duration,
    ) -> Result<Self, &'static str> {
        let args = p.item["command"].as_array().ok_or("item has no command")?;
        let args = args
            .iter()
            .map(|v| v.as_str().ok_or("command arguments must be strings"))
            .collect::<Result<Vec<_>, _>>()?;
        let raw = args
            .first()
            .filter(|s| !s.is_empty())
            .ok_or("empty command")?;
        let program = if raw.contains('/') && !Path::new(raw).is_absolute() {
            if !package_relative(Path::new(raw)) {
                return Err("relative command escapes package");
            }
            let program =
                fs::canonicalize(p.root.join(raw)).map_err(|_| "package program missing")?;
            if !program.starts_with(fs::canonicalize(&p.root).map_err(|_| "package root missing")?)
            {
                return Err("program escapes package");
            }
            program
        } else {
            PathBuf::from(raw)
        };
        let env = environment(d, p, lab)?;
        let mut cmd = Command::new(program);
        cmd.args(&args[1..])
            .env_clear()
            .envs(env.iter().map(|(k, v)| (k, v.as_str().unwrap())))
            .current_dir(&p.root)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        #[cfg(unix)]
        cmd.process_group(0);
        let mut child = cmd.spawn().map_err(|_| "provider launch failed")?;
        let mut stderr = child.stderr.take().ok_or("stderr unavailable")?;
        let observations = crate::observe::Observations::default();
        let collected = observations.clone();
        let drain = tokio::spawn(async move {
            let mut buf = [0; 4096];
            while let Ok(n) = stderr.read(&mut buf).await {
                if n == 0 {
                    break;
                }
                collected.add(&buf[..n]);
            }
        });
        let (sender, reader) = crate::observe::WireReader::channel();
        let stdout_drain = tokio::spawn(crate::observe::pump(
            child.stdout.take().unwrap(),
            observations.clone(),
            sender,
        ));
        // Baseline MCP adds these capabilities to every request automatically, replacing
        // any per-call declaration. Configure the client as well as the call metadata.
        let client = ClientConfig::new(
            serde_json::from_value(serde_json::json!({
                "extensions": {names::extension(): {"revision": "v0"}}
            }))
            .unwrap(),
            Default::default(),
        );
        let started =
            tokio::time::timeout(limit, client.serve((reader, child.stdin.take().unwrap()))).await;
        let service = match started {
            Ok(Ok(s)) => s,
            _ => {
                stop_child(&mut child).await;
                drain.abort();
                stdout_drain.abort();
                return Err("provider initialization failed or timed out");
            }
        };
        let mut run = Self {
            service,
            supported_versions: Value::Null,
            child,
            observations,
            drain,
            stdout_drain,
        };
        let request =
            serde_json::from_value(serde_json::json!({"method":"server/discover","params":{}}))
                .unwrap();
        let discover = tokio::time::timeout(limit, run.service.send_request(request)).await;
        let discovered = discover
            .ok()
            .and_then(Result::ok)
            .and_then(|d| serde_json::to_value(d).ok());
        let valid = discovered.as_ref().is_some_and(|v| {
            v["capabilities"]["extensions"][names::extension()]["revision"] == "v0"
        });
        if let Some(v) = discovered {
            run.supported_versions = v["supportedVersions"].clone();
        }
        if !valid {
            run.stop().await;
            return Err("discovery lacks Conduit extension v0");
        }
        Ok(run)
    }
    pub async fn stop(&mut self) {
        stop_child(&mut self.child).await;
        let _ = tokio::time::timeout(Duration::from_secs(1), &mut self.stdout_drain).await;
        self.stdout_drain.abort();
        let _ = tokio::time::timeout(Duration::from_secs(1), self.service.close()).await;
        let _ = tokio::time::timeout(Duration::from_secs(1), &mut self.drain).await;
        self.drain.abort();
    }
}
async fn stop_child(child: &mut Child) {
    #[cfg(unix)]
    if let Some(pid) = child.id() {
        unsafe {
            libc::kill(-(pid as i32), libc::SIGKILL);
        }
    }
    let _ = child.kill().await;
    let _ = child.wait().await;
}
impl Drop for Run {
    fn drop(&mut self) {
        #[cfg(unix)]
        if let Some(pid) = self.child.id() {
            unsafe {
                libc::kill(-(pid as i32), libc::SIGKILL);
            }
        }
        self.drain.abort();
        self.stdout_drain.abort();
    }
}

fn package_relative(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && path
            .components()
            .all(|c| matches!(c, Component::Normal(_) | Component::CurDir))
}
