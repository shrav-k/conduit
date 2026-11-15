use crate::{authorization_urls, status, url_facts, version};
use serde_json::{Value, json};
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::{
    fs::PermissionsExt,
    process::{CommandExt, ExitStatusExt},
};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::{Duration, Instant};

const LIMIT: usize = 64 * 1024;
const COMMAND_LIMIT: Duration = Duration::from_secs(30);
const TICK: Duration = Duration::from_millis(20);

// Set by SIGINT, SIGTERM or SIGHUP. Scratch commands stop immediately; everyday
// commands finish, and the safety check and logout run even after a signal.
static SIGNALLED: AtomicBool = AtomicBool::new(false);
extern "C" fn on_signal(_: libc::c_int) {
    SIGNALLED.store(true, Ordering::SeqCst);
}
fn interrupted() -> io::Result<()> {
    if SIGNALLED.load(Ordering::SeqCst) {
        Err(io::Error::new(io::ErrorKind::Interrupted, "signal"))
    } else {
        Ok(())
    }
}

struct Options {
    clients: [PathBuf; 2],
    out: PathBuf,
    extra_path: Vec<PathBuf>,
    login_args: [Vec<String>; 2],
}
impl Options {
    fn parse() -> Result<Self, ()> {
        let mut clients = [None, None];
        let mut out = None;
        let mut extra_path = Vec::new();
        let mut login_args = [Vec::new(), Vec::new()];
        let mut args = std::env::args_os().skip(1);
        while let Some(key) = args.next() {
            let value = args.next().ok_or(())?;
            match key.to_str().ok_or(())? {
                "--claude" | "--codex" => {
                    let i = usize::from(key == "--codex");
                    if clients[i].is_some() {
                        return Err(());
                    }
                    clients[i] = Some(std::path::absolute(value).map_err(|_| ())?);
                }
                "--out" if out.is_none() => out = Some(PathBuf::from(value)),
                "--extra-path" => extra_path.push(std::path::absolute(value).map_err(|_| ())?),
                "--login-arg" => {
                    let (name, arg) = value.to_str().ok_or(())?.split_once('=').ok_or(())?;
                    let i = match name {
                        "claude" => 0,
                        "codex" => 1,
                        _ => return Err(()),
                    };
                    login_args[i].push(arg.to_owned());
                }
                _ => return Err(()),
            }
        }
        let [a, b] = clients;
        Ok(Self {
            clients: [a.ok_or(())?, b.ok_or(())?],
            out: out.ok_or(())?,
            extra_path,
            login_args,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CommandKind {
    Everyday,
    Scratch,
    Safety,
}
impl CommandKind {
    fn check_running(self) -> io::Result<()> {
        if self == Self::Scratch {
            interrupted()?;
        }
        Ok(())
    }
}

struct Context<'a> {
    executable: &'a Path,
    home_variable: &'static str,
    scratch_home: PathBuf,
    shim: PathBuf,
    shim_log: PathBuf,
    path: OsString,
    base: &'a [(String, OsString)],
}
impl Context<'_> {
    fn start(&self, args: &[&str], kind: CommandKind) -> io::Result<Running> {
        if kind != CommandKind::Safety {
            interrupted()?;
        }
        let mut command = Command::new(self.executable);
        command
            .args(args)
            .env_clear()
            .envs(self.base.iter().cloned())
            .env("PATH", &self.path)
            .env("BROWSER", &self.shim)
            .env("A5_SHIM_LOG", &self.shim_log)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if kind != CommandKind::Everyday {
            command.env(self.home_variable, &self.scratch_home);
        }
        // SAFETY: setsid is async-signal-safe; no Rust allocation or locks in the child.
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() < 0 {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(())
                }
            });
        }
        let child = command.spawn()?;
        let mut running = Running {
            child,
            kind,
            output: [Vec::new(), Vec::new()],
            exit: None,
            clean: false,
        };
        running.nonblocking()?;
        Ok(running)
    }
    fn shim_output(&self) -> io::Result<(bool, String)> {
        match File::open(&self.shim_log) {
            Ok(file) => {
                let mut buf = Vec::new();
                file.take(LIMIT as u64).read_to_end(&mut buf)?;
                Ok((true, String::from_utf8_lossy(&buf).into_owned()))
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok((false, String::new())),
            Err(e) => Err(e),
        }
    }
    fn command(&self, args: &[&str], kind: CommandKind) -> io::Result<(Running, bool)> {
        let mut process = self.start(args, kind)?;
        let start = Instant::now();
        let timed_out = loop {
            kind.check_running()?;
            process.poll()?;
            if process.exit.is_some() {
                break false;
            }
            if start.elapsed() >= COMMAND_LIMIT {
                break true;
            }
            sleep(TICK);
        };
        if timed_out {
            process.signal(libc::SIGKILL)?;
        }
        process.finish()?;
        Ok((process, timed_out))
    }
    fn status(&self, args: &[&str], kind: CommandKind) -> io::Result<Value> {
        let (process, timed_out) = self.command(args, kind)?;
        let mut fact = status(&process.stdout(), &process.stderr());
        fact["skipped"] = json!(false);
        fact["exit"] = exit_value(process.exit);
        fact["timed_out"] = json!(timed_out);
        Ok(fact)
    }
    fn login(&self, args: &[&str], login_args: &[String]) -> io::Result<Value> {
        let mut all_args = args.to_vec();
        all_args.extend(login_args.iter().map(String::as_str));
        // The per-client log is reused by statuses; step 4 measures only this login.
        match fs::remove_file(&self.shim_log) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        let mut process = self.start(&all_args, CommandKind::Scratch)?;
        let start = Instant::now();
        let mut first_url = None;
        loop {
            interrupted()?;
            process.poll()?;
            let urls = authorization_urls(&process.printed(), &self.shim_output()?.1);
            if !urls.is_empty() {
                first_url.get_or_insert_with(Instant::now);
            }
            if first_url.is_some_and(|t: Instant| t.elapsed() >= Duration::from_secs(2))
                || (first_url.is_none() && start.elapsed() >= COMMAND_LIMIT)
                || (first_url.is_none() && process.exit.is_some())
            {
                break;
            }
            sleep(TICK);
        }
        let mut facts = std::collections::BTreeMap::<String, Value>::new();
        loop {
            interrupted()?;
            process.poll()?;
            let mut added = false;
            for (raw, source) in authorization_urls(&process.printed(), &self.shim_output()?.1) {
                let entry = facts.entry(raw);
                if let std::collections::btree_map::Entry::Occupied(mut entry) = entry {
                    entry.get_mut()["source"] = json!(source);
                    continue;
                }
                let (mut fact, target) = url_facts(entry.key(), source);
                if let Some((host, port)) = target
                    && process.exit.is_none()
                {
                    let ips: Vec<IpAddr> = match host.as_str() {
                        "localhost" => vec![Ipv4Addr::LOCALHOST.into(), Ipv6Addr::LOCALHOST.into()],
                        "127.0.0.1" => vec![Ipv4Addr::LOCALHOST.into()],
                        _ => vec![Ipv6Addr::LOCALHOST.into()],
                    };
                    let listening = ips.into_iter().any(|ip| {
                        TcpStream::connect_timeout(
                            &SocketAddr::new(ip, port),
                            Duration::from_secs(2),
                        )
                        .is_ok()
                    });
                    fact["listens"] = json!(if listening { "yes" } else { "no" });
                }
                entry.or_insert(fact);
                added = true;
            }
            if !added {
                break;
            }
            // A listener connection can make the client emit another URL. Drain again
            // before cancellation, allowing the client to react to the connection.
            sleep(TICK);
        }
        let waiting = process.exit.is_none();
        let before = process.exit;
        // The last poll's URLs have all been processed. Do not capture another batch
        // between that check and TERM; output captured after TERM is only redacted.
        let ended = process.finish_polled()?;
        // Capture late output as well; no listener is contacted after cancellation.
        for (raw, source) in authorization_urls(&process.printed(), &self.shim_output()?.1) {
            let fact = facts
                .entry(raw.clone())
                .or_insert_with(|| url_facts(&raw, source).0);
            fact["source"] = json!(source);
        }
        let facts: Vec<_> = facts.into_values().collect();
        Ok(
            json!({"skipped": false, "login_args": login_args, "urls_seen": facts.len(),
            "shim_called": self.shim_output()?.0, "waiting_at_cancel": waiting,
            "exited_before_cancel": exit_value(before), "ended_on": ended,
            "group_gone": process.clean, "logged_out": false, "urls": facts}),
        )
    }
}

struct Running {
    child: Child,
    kind: CommandKind,
    output: [Vec<u8>; 2],
    exit: Option<ExitStatus>,
    clean: bool,
}
impl Running {
    fn nonblocking(&mut self) -> io::Result<()> {
        for fd in [
            self.child.stdout.as_ref().unwrap().as_raw_fd(),
            self.child.stderr.as_ref().unwrap().as_raw_fd(),
        ] {
            // SAFETY: fd belongs to a live piped stream; preserve all existing flags.
            unsafe {
                let flags = libc::fcntl(fd, libc::F_GETFL);
                if flags < 0 || libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) < 0 {
                    return Err(io::Error::last_os_error());
                }
            }
        }
        Ok(())
    }
    fn drain(fd: RawFd, out: &mut Vec<u8>) -> io::Result<()> {
        let mut buf = [0u8; 4096];
        // Bound each polling round as well, so a noisy child cannot defeat the deadline.
        for _ in 0..32 {
            // SAFETY: buf is writable for the supplied length and fd is a live pipe.
            let size = unsafe { libc::read(fd, buf.as_mut_ptr().cast(), buf.len()) };
            if size == 0 {
                break;
            }
            if size < 0 {
                let error = io::Error::last_os_error();
                if error.kind() == io::ErrorKind::WouldBlock {
                    break;
                }
                if error.kind() == io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(error);
            }
            let count = (size as usize).min(LIMIT - out.len());
            out.extend_from_slice(&buf[..count]);
        }
        Ok(())
    }
    fn poll(&mut self) -> io::Result<()> {
        Self::drain(
            self.child.stdout.as_ref().unwrap().as_raw_fd(),
            &mut self.output[0],
        )?;
        Self::drain(
            self.child.stderr.as_ref().unwrap().as_raw_fd(),
            &mut self.output[1],
        )?;
        if self.exit.is_none() {
            self.exit = self.child.try_wait()?;
        }
        Ok(())
    }
    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.output[0]).into_owned()
    }
    fn stderr(&self) -> String {
        String::from_utf8_lossy(&self.output[1]).into_owned()
    }
    fn printed(&self) -> String {
        format!("{}\n{}", self.stdout(), self.stderr())
    }
    fn signal(&self, signal: i32) -> io::Result<()> {
        // SAFETY: negative PID targets only the fresh session's process group.
        if unsafe { libc::kill(-(self.child.id() as i32), signal) } == 0 {
            return Ok(());
        }
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ESRCH) {
            Ok(())
        } else {
            Err(error)
        }
    }
    fn gone(&self) -> io::Result<bool> {
        // SAFETY: signal 0 only tests group existence.
        if unsafe { libc::kill(-(self.child.id() as i32), 0) } == 0 {
            return Ok(false);
        }
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ESRCH) {
            Ok(true)
        } else {
            Err(error)
        }
    }
    fn finish(&mut self) -> io::Result<&'static str> {
        self.poll()?;
        self.finish_polled()
    }
    fn finish_polled(&mut self) -> io::Result<&'static str> {
        let exited = self.exit.is_some();
        self.signal(libc::SIGTERM)?;
        let mut ended = if exited { "exited" } else { "term" };
        let start = Instant::now();
        loop {
            self.kind.check_running()?;
            self.poll()?;
            if self.exit.is_some() && self.gone()? {
                self.clean = true;
                break;
            }
            if start.elapsed() >= Duration::from_secs(5) {
                break;
            }
            sleep(TICK);
        }
        if !self.clean {
            self.signal(libc::SIGKILL)?;
            if !exited {
                ended = "kill";
            }
            let start = Instant::now();
            loop {
                self.kind.check_running()?;
                self.poll()?;
                if self.exit.is_some() && self.gone()? {
                    self.clean = true;
                    break;
                }
                if start.elapsed() >= Duration::from_secs(2) {
                    return Err(io::Error::other("group"));
                }
                sleep(TICK);
            }
        }
        // Drain the tail after reaping. Both buffers still have their fixed cap.
        self.poll()?;
        Ok(ended)
    }
}
impl Drop for Running {
    fn drop(&mut self) {
        if !self.clean {
            let _ = self.signal(libc::SIGKILL);
            let _ = self.child.wait();
        }
    }
}

fn exit_value(status: Option<ExitStatus>) -> Value {
    status
        .map(|s| json!(s.code().unwrap_or_else(|| -s.signal().unwrap_or(0))))
        .unwrap_or(Value::Null)
}
// A skipped step observed nothing, so every observed field is null.
fn skipped_status() -> Value {
    json!({"skipped": true, "exit": null, "timed_out": null, "login": null, "account_named": null, "account_field": null})
}
fn skipped_login(args: &[String]) -> Value {
    json!({"skipped": true, "login_args": args, "urls_seen": null, "shim_called": null,
        "waiting_at_cancel": null, "exited_before_cancel": null, "ended_on": null,
        "group_gone": null, "logged_out": false, "urls": []})
}

fn client(
    context: &Context<'_>,
    index: usize,
    login_args: &[String],
    logouts: &mut Vec<String>,
) -> Result<Value, String> {
    let name = ["claude-code", "codex"][index];
    let fault = |step: usize| format!("step {step} {name}: fault");
    let stopped = |step: usize, error: io::Error| {
        if error.kind() == io::ErrorKind::Interrupted {
            format!("step {step} {name}: signal")
        } else {
            fault(step)
        }
    };
    let status_args: &[&str] = if index == 0 {
        &["auth", "status"]
    } else {
        &["login", "status"]
    };
    let login_args_base: &[&str] = if index == 0 {
        &["auth", "login"]
    } else {
        &["login"]
    };
    let logout_args: &[&str] = if index == 0 {
        &["auth", "logout"]
    } else {
        &["logout"]
    };
    let (v, timed_out) = context
        .command(&["--version"], CommandKind::Everyday)
        .map_err(|e| stopped(1, e))?;
    interrupted().map_err(|e| stopped(1, e))?;
    let version = version(&v.stdout(), &v.stderr());
    let step2 = context
        .status(status_args, CommandKind::Everyday)
        .map_err(|e| stopped(2, e))?;
    interrupted().map_err(|e| stopped(2, e))?;
    let step3 = context
        .status(status_args, CommandKind::Scratch)
        .map_err(|e| stopped(3, e))?;
    let mut step4 = skipped_login(login_args);
    let mut step5 = skipped_status();
    if step3["login"] == "no" {
        // A signal in step 4 still owes the empty scratch home its safety check.
        let login_signal = match context.login(login_args_base, login_args) {
            Ok(facts) => {
                step4 = facts;
                None
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => Some(stopped(4, e)),
            Err(_) => return Err(fault(4)),
        };
        step5 = context
            .status(status_args, CommandKind::Safety)
            .map_err(|_| fault(5))?;
        if step5["login"] == "yes" {
            // Only the positively empty scratch home may ever receive a logout.
            // Name the login before even starting its scratch-only logout.
            logouts.push(format!("step 5 {name}: scratch login found"));
            context
                .command(logout_args, CommandKind::Safety)
                .map_err(|_| fault(5))?;
            step4["logged_out"] = json!(true);
            logouts.push(format!("step 5 {name}: logout"));
        }
        if let Some(signal) = login_signal {
            return Err(signal);
        }
        interrupted().map_err(|e| stopped(5, e))?;
    }
    let step6 = context
        .status(status_args, CommandKind::Everyday)
        .map_err(|e| stopped(6, e))?;
    interrupted().map_err(|e| stopped(6, e))?;
    Ok(json!({"client": name, "version": version, "steps": {
    "1": {"exit": exit_value(v.exit), "timed_out": timed_out},
    "2": step2, "3": step3, "4": step4, "5": step5, "6": step6}}))
}

fn flatten(value: &Value, prefix: &str, rows: &mut std::collections::BTreeMap<String, String>) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                flatten(value, &format!("{prefix}.{key}"), rows);
            }
        }
        Value::Array(items) if !items.is_empty() => {
            for (i, value) in items.iter().enumerate() {
                flatten(value, &format!("{prefix}.{i}"), rows);
            }
        }
        _ => {
            rows.insert(
                prefix.into(),
                value.to_string().replace('|', "\\|").replace('\n', "\\n"),
            );
        }
    }
}
fn table(clients: &[Value]) {
    println!(
        "| Fact | {} {} | {} {} |",
        clients[0]["client"].as_str().unwrap(),
        clients[0]["version"].as_str().unwrap(),
        clients[1]["client"].as_str().unwrap(),
        clients[1]["version"].as_str().unwrap()
    );
    println!("| --- | --- | --- |");
    let mut columns = [
        std::collections::BTreeMap::new(),
        std::collections::BTreeMap::new(),
    ];
    for i in 0..2 {
        flatten(&clients[i]["steps"], "step", &mut columns[i]);
    }
    let keys: std::collections::BTreeSet<_> = columns.iter().flat_map(|c| c.keys()).collect();
    for key in keys {
        println!(
            "| {key} | {} | {} |",
            columns[0].get(key).map(String::as_str).unwrap_or("null"),
            columns[1].get(key).map(String::as_str).unwrap_or("null")
        );
    }
}

pub fn main() -> Result<(), String> {
    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        let handler = on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
        // SAFETY: the handler only stores into an atomic, which is async-signal-safe.
        if unsafe { libc::signal(signal, handler) } == libc::SIG_ERR {
            return Err("step 0 claude-code: signal fault".to_owned());
        }
    }
    let options = Options::parse().map_err(|_| "step 0 claude-code: arguments fault".to_owned())?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&options.out)
        .map_err(|_| "step 0 claude-code: output fault".to_owned())?;
    let mut logouts = Vec::new();
    let mut written = false;
    let result = execute(&options, &mut output, &mut logouts, &mut written);
    // A partial document must never masquerade as a completed run.
    if result.is_err() && !written {
        let _ = fs::remove_file(&options.out);
    }
    // Every safety logout is named, whatever else failed after it.
    match (result, logouts.is_empty()) {
        (result, true) => result,
        (Ok(()), false) => Err(logouts.join("; ")),
        (Err(e), false) => Err(format!("{}; {e}", logouts.join("; "))),
    }
}
fn execute(
    options: &Options,
    output: &mut File,
    logouts: &mut Vec<String>,
    written: &mut bool,
) -> Result<(), String> {
    use std::os::unix::fs::symlink;
    let fault = || "step 0 claude-code: scratch fault".to_owned();
    let temp = std::env::var_os("TMPDIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let scratch = tempfile::Builder::new()
        .prefix("a5-")
        .tempdir_in(temp)
        .map_err(|_| fault())?;
    fs::set_permissions(scratch.path(), fs::Permissions::from_mode(0o700)).map_err(|_| fault())?;
    let bin = scratch.path().join("bin");
    fs::create_dir(&bin).map_err(|_| fault())?;
    let shim = bin.join("browser-shim");
    let executable = std::env::current_exe().map_err(|_| fault())?;
    let target = executable.with_file_name("browser-shim");
    // Without the shim, a client's search for open would reach the real /usr/bin/open.
    if !fs::metadata(&target).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0) {
        return Err("step 0 claude-code: shim fault".to_owned());
    }
    symlink(&target, &shim).map_err(|_| fault())?;
    for name in ["open", "xdg-open"] {
        symlink(&shim, bin.join(name)).map_err(|_| fault())?;
    }
    let base: Vec<_> = ["HOME", "USER", "LOGNAME"]
        .into_iter()
        .filter_map(|key| std::env::var_os(key).map(|v| (key.into(), v)))
        .collect();
    let mut clients = Vec::new();
    for index in 0..2 {
        let home = scratch.path().join(format!("home-{index}"));
        fs::create_dir(&home).map_err(|_| fault())?;
        fs::set_permissions(&home, fs::Permissions::from_mode(0o700)).map_err(|_| fault())?;
        let mut paths = vec![
            bin.clone(),
            options.clients[index]
                .parent()
                .ok_or_else(fault)?
                .to_owned(),
        ];
        paths.extend(options.extra_path.iter().cloned());
        paths.extend(["/usr/bin", "/bin", "/usr/sbin", "/sbin"].map(PathBuf::from));
        let context = Context {
            executable: &options.clients[index],
            home_variable: ["CLAUDE_CONFIG_DIR", "CODEX_HOME"][index],
            scratch_home: home,
            shim: shim.clone(),
            shim_log: scratch.path().join(format!("shim-{index}")),
            path: std::env::join_paths(paths).map_err(|_| fault())?,
            base: &base,
        };
        clients.push(client(
            &context,
            index,
            &options.login_args[index],
            logouts,
        )?);
    }
    scratch.close().map_err(|_| fault())?;
    interrupted().map_err(|_| "step 6 codex: signal".to_owned())?;
    serde_json::to_writer_pretty(&mut *output, &json!({"clients": clients}))
        .map_err(|_| "step 6 codex: output fault".to_owned())?;
    output
        .write_all(b"\n")
        .map_err(|_| "step 6 codex: output fault".to_owned())?;
    *written = true;
    table(&clients);
    Ok(())
}

use std::os::unix::fs::OpenOptionsExt;
