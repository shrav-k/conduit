use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const OVERRIDE: &str = "CODEX_REFRESH_TOKEN_URL_OVERRIDE";
pub const BLOCK: &str = "https://refresh.invalid/oauth/token";
pub const DISCOVERY: &str = "/backend-api/wham/accounts/check";
pub const ROTATION: &str = "/oauth/token";
pub const ACCOUNT: &str = "probe-208-account";
pub const EMAIL: &str = "work-orders@conduit.invalid";
pub const CLAIMS: &str = "https://api.openai.com/auth";
const INITIAL: &str = "probe-208-refresh-original";
const NEXT: &str = "probe-208-refresh-issued";
const LOCKS: [&str; 2] = ["auth.lock", "auth.json.lock"];
const PATH: &str = "/usr/bin:/bin";
const EXCHANGE: Duration = Duration::from_secs(30);
const TERM_WAIT: Duration = Duration::from_secs(5);

#[derive(Clone, Copy)]
pub enum Case {
    P1,
    P2,
    P3,
}

impl Case {
    pub fn name(self) -> &'static str {
        match self {
            Self::P1 => "P1",
            Self::P2 => "P2",
            Self::P3 => "P3",
        }
    }

    pub fn expires(self, now: i64) -> i64 {
        now + match self {
            Self::P1 => 180,
            Self::P2 => 86_400,
            Self::P3 => -600,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Arm {
    Blocked,
    Control,
}

impl Arm {
    fn name(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Control => "control",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Form {
    Issuer,
    Variable,
}

impl Form {
    pub fn name(self) -> &'static str {
        match self {
            Self::Issuer => "issuer",
            Self::Variable => "variable",
        }
    }
}

pub fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

fn utc(timestamp: i64) -> String {
    let seconds = timestamp as libc::time_t;
    let mut parts = std::mem::MaybeUninit::<libc::tm>::uninit();
    // gmtime_r fills the structure and keeps concurrent callers independent.
    let parts = unsafe {
        assert!(!libc::gmtime_r(&seconds, parts.as_mut_ptr()).is_null());
        parts.assume_init()
    };
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        parts.tm_year + 1900,
        parts.tm_mon + 1,
        parts.tm_mday,
        parts.tm_hour,
        parts.tm_min,
        parts.tm_sec
    )
}

pub fn today() -> String {
    utc(unix_now())[..10].to_owned()
}

fn base64url(bytes: &[u8]) -> String {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut output = String::new();
    let mut buffer = 0_u32;
    let mut bits = 0;
    for &byte in bytes {
        buffer = (buffer << 8) | u32::from(byte);
        bits += 8;
        while bits >= 6 {
            bits -= 6;
            output.push(char::from(alphabet[((buffer >> bits) & 63) as usize]));
        }
    }
    if bits != 0 {
        output.push(char::from(alphabet[((buffer << (6 - bits)) & 63) as usize]));
    }
    output
}

fn jwt(payload: Value, segment: &str) -> String {
    let header = base64url(br#"{"alg":"none","typ":"JWT"}"#);
    let claims = base64url(payload.to_string().as_bytes());
    // Codex's payload decoder requires a nonempty third segment. These bytes are fabricated.
    format!("{header}.{claims}.{segment}")
}

fn identity() -> String {
    jwt(
        json!({
            "email": EMAIL,
            (CLAIMS): {
                "chatgpt_plan_type": "plus",
                "chatgpt_user_id": "probe-208-user",
                "chatgpt_account_id": ACCOUNT
            }
        }),
        "AA",
    )
}

fn original(case: Case, now: i64) -> String {
    jwt(json!({"exp": case.expires(now)}), "AA")
}

pub struct Login {
    case: Case,
    now: i64,
    original: String,
}

impl Login {
    pub fn new(case: Case, now: i64) -> Self {
        Self {
            case,
            now,
            original: original(case, now),
        }
    }

    pub fn write(&self, home: &Path, form: Form, origin: &str) -> io::Result<()> {
        let mut contents = json!({
            "auth_mode": "chatgpt",
            "OPENAI_API_KEY": null,
            "tokens": {
                "id_token": identity(),
                "access_token": self.original,
                "refresh_token": INITIAL,
                "account_id": ACCOUNT
            },
            "last_refresh": utc(self.now)
        });
        if matches!(form, Form::Issuer) {
            contents["tokens"]["oauth_provenance"] = json!({
                "issuer": origin,
                "client_id": "probe-208-client"
            });
        }
        private_dir(home)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(home.join("auth.json"))?;
        writeln!(file, "{contents}")
    }
}

fn private_dir(path: &Path) -> io::Result<()> {
    fs::DirBuilder::new().mode(0o700).create(path)
}

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub struct Scratch(PathBuf);

impl Scratch {
    pub fn new(root: &Path, prefix: &str) -> io::Result<Self> {
        let counter = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = root.join(format!("{prefix}-{}-{nanos}-{counter}", std::process::id()));
        private_dir(&path)?;
        Ok(Self(path))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn remove(self) -> io::Result<()> {
        fs::remove_dir_all(&self.0)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

pub struct LoginState(Option<LoginFile>);

struct LoginFile {
    bytes: Vec<u8>,
    mtime: (i64, i64),
    inode: u64,
}

impl LoginState {
    pub fn read(home: &Path) -> io::Result<Self> {
        match File::open(home.join("auth.json")) {
            Ok(mut file) => {
                let metadata = file.metadata()?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                Ok(Self(Some(LoginFile {
                    bytes,
                    mtime: (metadata.mtime(), metadata.mtime_nsec()),
                    inode: metadata.ino(),
                })))
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Self(None)),
            Err(error) => Err(error),
        }
    }
}

pub fn wrote(before: &LoginState, after: &LoginState) -> bool {
    match (&before.0, &after.0) {
        (Some(before), Some(after)) => {
            before.bytes != after.bytes
                || before.mtime != after.mtime
                || before.inode != after.inode
        }
        (Some(_), None) | (None, Some(_)) => true,
        (None, None) => false,
    }
}

fn login_report(before: &LoginState, after: &LoginState) -> Value {
    let both = before.0.as_ref().zip(after.0.as_ref());
    json!({
        "present_after": after.0.is_some(),
        "bytes_equal": both.is_some_and(|(before, after)| before.bytes == after.bytes),
        "mtime_equal": both.is_some_and(|(before, after)| before.mtime == after.mtime),
        "inode_equal": both.is_some_and(|(before, after)| before.inode == after.inode),
        "size_before": before.0.as_ref().map(|file| file.bytes.len()),
        "size_after": after.0.as_ref().map(|file| file.bytes.len())
    })
}

#[derive(Default)]
struct Requests {
    entries: Vec<Value>,
    issued: BTreeSet<String>,
    refused: bool,
}

pub struct Stub {
    address: String,
    requests: Arc<Mutex<Requests>>,
    stopping: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<io::Result<()>>>,
}

impl Stub {
    pub fn start(login: Login) -> io::Result<Self> {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
        listener.set_nonblocking(true)?;
        let address = listener.local_addr()?.to_string();
        let requests = Arc::new(Mutex::new(Requests::default()));
        let stopping = Arc::new(AtomicBool::new(false));
        let shared_requests = Arc::clone(&requests);
        let stop = Arc::clone(&stopping);
        let login = Arc::new(login);
        let thread = thread::spawn(move || {
            let mut connections = Vec::new();
            let outcome = loop {
                if stop.load(Ordering::Acquire) {
                    break Ok(());
                }
                match listener.accept() {
                    Ok((stream, _)) => {
                        let requests = Arc::clone(&shared_requests);
                        let login = Arc::clone(&login);
                        connections.push(thread::spawn(move || serve(stream, &login, &requests)));
                    }
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(5));
                    }
                    Err(error) => break Err(error),
                }
            };
            for connection in connections {
                connection
                    .join()
                    .map_err(|_| io::Error::other("stub connection"))??;
            }
            outcome
        });
        Ok(Self {
            address,
            requests,
            stopping,
            thread: Some(thread),
        })
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn origin(&self) -> String {
        format!("http://{}", self.address)
    }

    fn stop(&mut self) -> io::Result<()> {
        self.stopping.store(true, Ordering::Release);
        if let Some(thread) = self.thread.take() {
            thread
                .join()
                .map_err(|_| io::Error::other("stub listener"))??;
        }
        Ok(())
    }

    pub fn finish(mut self) -> io::Result<Vec<Value>> {
        self.stop()?;
        let mut requests = self.requests.lock().unwrap();
        if requests.refused {
            return Err(io::Error::other(
                "stub log contains an unsafe method or path",
            ));
        }
        Ok(std::mem::take(&mut requests.entries))
    }
}

impl Drop for Stub {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

fn serve(stream: TcpStream, login: &Login, requests: &Mutex<Requests>) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        return Ok(());
    }
    let target = line.clone();
    let mut parts = target.split_ascii_whitespace();
    let method = parts.next().unwrap_or_default();
    let path = parts
        .next()
        .unwrap_or_default()
        .split('?')
        .next()
        .unwrap_or_default();

    let mut bearer = "none";
    let mut body_len = 0_u64;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 || line == "\r\n" || line == "\n" {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("authorization") {
                bearer = match value.trim().split_once(' ') {
                    Some((scheme, value)) if scheme.eq_ignore_ascii_case("bearer") => {
                        if value == login.original {
                            "original"
                        } else if requests.lock().unwrap().issued.contains(value) {
                            "issued"
                        } else {
                            "other"
                        }
                    }
                    _ => "other",
                };
            } else if name.eq_ignore_ascii_case("content-length") {
                body_len = value.trim().parse().unwrap_or(0);
            }
        }
    }
    io::copy(&mut (&mut reader).take(body_len), &mut io::sink())?;

    let (status, body) = match (method, path) {
        ("GET", DISCOVERY) if matches!(login.case, Case::P2 | Case::P3) && bearer == "original" => {
            (401, json!({}))
        }
        ("GET", DISCOVERY) => (
            200,
            json!({"accounts": [{
                "id": ACCOUNT,
                "workspace_backend_origin": "https://backend.invalid",
                "account_routing_override": "NO_CONSTRAINT"
            }]}),
        ),
        ("POST", ROTATION) => {
            let issued = jwt(json!({"exp": unix_now() + 86_400}), "AQ");
            requests.lock().unwrap().issued.insert(issued.clone());
            (
                200,
                json!({
                    "access_token": issued,
                    "refresh_token": NEXT,
                    "id_token": identity(),
                }),
            )
        }
        _ => (404, json!({})),
    };
    {
        let mut requests = requests.lock().unwrap();
        if safe_label(method) && safe_label(path) && path.starts_with('/') {
            requests
                .entries
                .push(json!({"method":method,"path":path,"status":status,"bearer":bearer}));
        } else {
            requests.refused = true;
        }
    }
    let reason = match status {
        200 => "OK",
        401 => "Unauthorized",
        _ => "Not Found",
    };
    let body = body.to_string();
    write!(
        reader.get_mut(),
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

fn safe_label(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 256
        && !s.contains("probe-208-refresh")
        && !s.contains("eyJ")
        && !s.starts_with("//")
        && s.split('/').all(|part| part != "..")
        && s.bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"/-_.:".contains(&byte))
}

fn home_entries(home: &Path) -> io::Result<BTreeSet<String>> {
    fn visit(root: &Path, dir: &Path, entries: &mut BTreeSet<String>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(root).unwrap();
            let name = relative
                .to_str()
                .filter(|name| safe_label(name))
                .ok_or_else(|| io::Error::other("unsafe home entry"))?;
            entries.insert(name.to_owned());
            if entry.file_type()?.is_dir() {
                visit(root, &path, entries)?;
            }
        }
        Ok(())
    }
    let mut entries = BTreeSet::new();
    visit(home, home, &mut entries)?;
    Ok(entries)
}

#[cfg(target_os = "linux")]
struct LockWatch(File);

#[cfg(target_os = "linux")]
impl LockWatch {
    fn start(home: &Path) -> io::Result<Self> {
        use std::os::fd::FromRawFd;
        let name = std::ffi::CString::new(home.as_os_str().as_encoded_bytes())?;
        // The owned file closes the watcher even if registering the directory fails.
        let fd = unsafe { libc::inotify_init1(libc::IN_CLOEXEC | libc::IN_NONBLOCK) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        let file = unsafe { File::from_raw_fd(fd) };
        let mask = libc::IN_CREATE | libc::IN_MOVED_TO;
        if unsafe { libc::inotify_add_watch(fd, name.as_ptr(), mask) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self(file))
    }

    fn appeared(&mut self, locks: &mut [bool; 2]) -> io::Result<()> {
        let mut buffer = [0_u8; 8192];
        let header_size = std::mem::size_of::<libc::inotify_event>();
        loop {
            let n = match self.0.read(&mut buffer) {
                Ok(0) => return Ok(()),
                Ok(n) => n,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            };
            let mut rest = &buffer[..n];
            while rest.len() >= header_size {
                // Linux emits complete records here, while the byte buffer may be unaligned.
                let event = unsafe { rest.as_ptr().cast::<libc::inotify_event>().read_unaligned() };
                let end = header_size + event.len as usize;
                if end > rest.len() || event.mask & libc::IN_Q_OVERFLOW != 0 {
                    return Err(io::Error::other("lock observation"));
                }
                if let Some(name) = rest[header_size..end].split(|&byte| byte == 0).next() {
                    for (i, lock) in LOCKS.iter().enumerate() {
                        locks[i] |= name == lock.as_bytes();
                    }
                }
                rest = &rest[end..];
            }
            if !rest.is_empty() {
                return Err(io::Error::other("incomplete lock event"));
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
struct LockWatch;

#[cfg(not(target_os = "linux"))]
impl LockWatch {
    // Only the fake runs on this platform, and it leaves any lock it creates in place.
    fn start(_home: &Path) -> io::Result<Self> {
        Ok(Self)
    }

    fn appeared(&mut self, _locks: &mut [bool; 2]) -> io::Result<()> {
        Ok(())
    }
}

pub fn isolated_command(client: &Path, working: &Path, home: &Path, codex_home: &Path) -> Command {
    let mut command = Command::new(client);
    command
        .current_dir(working)
        .env_clear()
        .env("HOME", home)
        .env("CODEX_HOME", codex_home)
        .env("PATH", PATH);
    command
}

struct Group {
    child: Child,
    id: i32,
    stopped: bool,
}

impl Group {
    fn start(command: &mut Command) -> io::Result<Self> {
        let child = command.process_group(0).spawn()?;
        let id = child.id() as i32;
        Ok(Self {
            child,
            id,
            stopped: false,
        })
    }

    fn signal(&self, signal: i32) -> io::Result<bool> {
        // A negative PID addresses the process group created by CommandExt.
        if unsafe { libc::kill(-self.id, signal) } == 0 {
            return Ok(true);
        }
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ESRCH) {
            Ok(false)
        } else {
            Err(error)
        }
    }

    fn wait_until(&mut self, duration: Duration) -> io::Result<Option<ExitStatus>> {
        let deadline = Instant::now() + duration;
        loop {
            let status = self.child.try_wait()?;
            if let Some(status) = status {
                // Reap adopted descendants on Linux when the namespace worker is a subreaper.
                let mut raw = 0;
                while unsafe { libc::waitpid(-self.id, &mut raw, libc::WNOHANG) } > 0 {}
                if !self.signal(0)? {
                    self.stopped = true;
                    return Ok(Some(status));
                }
            }
            if Instant::now() >= deadline {
                return Ok(None);
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn stop(&mut self) -> io::Result<(&'static str, ExitStatus)> {
        if let Some(status) = self.wait_until(Duration::ZERO)? {
            return Ok(("exited", status));
        }
        self.signal(libc::SIGTERM)?;
        if let Some(status) = self.wait_until(TERM_WAIT)? {
            return Ok(("term", status));
        }
        self.signal(libc::SIGKILL)?;
        match self.wait_until(TERM_WAIT)? {
            Some(status) => Ok(("kill", status)),
            None => Err(io::Error::other("process group still present")),
        }
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        if !self.stopped {
            let _ = self.signal(libc::SIGKILL);
            let _ = self.wait_until(TERM_WAIT);
        }
    }
}

type Fault = &'static str;

fn send(input: &mut ChildStdin, message: Value) -> Result<(), Fault> {
    writeln!(input, "{message}")
        .and_then(|()| input.flush())
        .map_err(|_| "send request")
}

fn redact(line: &str, run: &Path, address: &str) -> Result<String, Fault> {
    let run = run.to_str().ok_or("encode run path")?;
    let replaced = line
        .replace(run, "<run>")
        .replace(address, "127.0.0.1:<port>");

    // Inspect every decoded string, including object keys and duplicate fields.
    // Keep the original JSON spelling and whitespace in the recorded line.
    let mut rest = replaced.as_str();
    while let Some(start) = rest.find('"') {
        let mut strings = serde_json::Deserializer::from_str(&rest[start..]).into_iter::<String>();
        let value = strings
            .next()
            .ok_or("decode answer string")?
            .map_err(|_| "decode answer string")?;
        validate_answer_string(&value)?;
        rest = &rest[start + strings.byte_offset()..];
    }
    Ok(replaced)
}

fn validate_answer_string(value: &str) -> Result<(), Fault> {
    let inspected = value
        .replace("<run>/", "<run>")
        .replace("https://backend.invalid", "")
        .replace(BLOCK, "")
        .replace(EMAIL, "")
        .replace("127.0.0.1:<port>", "");
    let mut previous = ' ';
    for ch in inspected.chars() {
        if ch == '\\'
            || (ch == '/' && !previous.is_ascii_alphanumeric() && !"._-".contains(previous))
        {
            return Err("answer contains an unapproved absolute path");
        }
        previous = ch;
    }
    if inspected.contains("eyJ")
        || inspected.contains("probe-208-refresh")
        || inspected.contains('@')
        || inspected.contains("://")
        || inspected.contains("127.0.0.1:")
    {
        return Err("answer contains unapproved data");
    }
    Ok(())
}

enum Reply {
    Line { raw: String, message: Value },
    Eof,
    Timeout,
}

fn read_answer(
    lines: &mpsc::Receiver<io::Result<String>>,
    deadline: Instant,
    id: u64,
    methods: &mut Vec<String>,
) -> Result<Reply, Fault> {
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Ok(Reply::Timeout);
        }
        let line = match lines.recv_timeout(remaining) {
            Ok(Ok(line)) => line,
            Ok(Err(_)) => return Err("read answer"),
            Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(Reply::Eof),
            Err(mpsc::RecvTimeoutError::Timeout) => return Ok(Reply::Timeout),
        };
        let message: Value = serde_json::from_str(&line).map_err(|_| "decode answer")?;
        if let Some(method) = message["method"].as_str() {
            if !safe_label(method) || method.starts_with('/') {
                return Err("invalid skipped method");
            }
            methods.push(method.to_owned());
            continue;
        }
        if message["id"].as_u64() != Some(id) {
            return Err("unexpected answer id");
        }
        return Ok(Reply::Line { raw: line, message });
    }
}

fn account_answer(reply: Reply, run: &Path, address: &str) -> Result<Value, Fault> {
    let raw = match reply {
        Reply::Line { raw, .. } => raw,
        Reply::Eof => return Ok(answer("eof", None, None)),
        Reply::Timeout => return Ok(answer("timeout", None, None)),
    };
    let line = redact(&raw, run, address)?;
    let message: Value = serde_json::from_str(&line).map_err(|_| "decode redacted answer")?;
    if let Some(error) = message.get("error") {
        let error = error["message"].as_str().ok_or("read error message")?;
        return Ok(answer("error", Some(line), Some(error.to_owned())));
    }
    if message.get("result").is_some() {
        return Ok(answer("result", Some(line), None));
    }
    Err("missing answer result")
}

fn answer(kind: &str, line: Option<String>, error: Option<String>) -> Value {
    json!({"kind": kind, "line": line, "error_message": error})
}

fn exchange(group: &mut Group, run: &Path, address: &str, report: &mut Value) -> Result<(), Fault> {
    let deadline = Instant::now() + EXCHANGE;
    let mut input = group.child.stdin.take().ok_or("open client input")?;
    let stdout = group.child.stdout.take().ok_or("open client output")?;
    let (tx, rx) = mpsc::sync_channel(64);
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        loop {
            let mut line = String::new();
            let result = reader.by_ref().take(1024 * 1024).read_line(&mut line);
            match result {
                Ok(0) => break,
                Ok(_) if line.ends_with('\n') => {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                    if tx.send(Ok(line)).is_err() {
                        break;
                    }
                }
                Ok(_) => {
                    let _ = tx.send(Err(io::Error::other("incomplete client line")));
                    break;
                }
                Err(error) => {
                    let _ = tx.send(Err(error));
                    break;
                }
            }
        }
    });

    let mut methods = Vec::new();
    let result = (|| {
        send(
            &mut input,
            json!({
                "id": 1, "method": "initialize",
                "params": {
                    "clientInfo": {"name": "agent-codex", "version": "0.2.0"},
                    "capabilities": {"experimentalApi": false}
                }
            }),
        )?;
        match read_answer(&rx, deadline, 1, &mut methods)? {
            Reply::Line { message, .. }
                if message.get("result").is_some() && message.get("error").is_none() => {}
            Reply::Line { .. } => return Err("initialize rejected"),
            Reply::Eof => return Err("initialize eof"),
            Reply::Timeout => return Err("initialize timeout"),
        }
        send(&mut input, json!({"method": "initialized"}))?;
        send(
            &mut input,
            json!({
                "id": 2, "method": "account/read",
                "params": {"refreshToken": false}
            }),
        )?;
        let reply = read_answer(&rx, deadline, 2, &mut methods)?;
        report["answer"] = account_answer(reply, run, address)?;
        Ok(())
    })();
    // Keep stdin open throughout the hold, and continue draining stdout after the answer.
    group.child.stdin = Some(input);
    thread::spawn(move || while rx.recv().is_ok() {});
    report["skipped_methods"] = json!(methods);
    result
}

fn stderr_lines(path: &Path) -> io::Result<usize> {
    let mut file = File::open(path)?;
    let mut buffer = [0_u8; 8192];
    let mut count = 0;
    let mut terminated = true;
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            return Ok(count + usize::from(!terminated));
        }
        count += buffer[..n].iter().filter(|&&byte| byte == b'\n').count();
        terminated = buffer[n - 1] == b'\n';
    }
}

pub struct Run<'a> {
    pub client: &'a Path,
    pub scratch: &'a Path,
    pub logs: &'a Path,
    pub case: Case,
    pub arm: Arm,
    pub repetition: u8,
    pub form: Form,
    pub hold: Duration,
}

pub type Prepare<'a> = dyn Fn(&Path, &Path, &str) -> io::Result<()> + 'a;

pub struct Hooks<'a> {
    pub prepare: &'a Prepare<'a>,
    pub inspect: &'a dyn Fn(&Path) -> io::Result<()>,
}

impl Run<'_> {
    pub fn execute(&self, hooks: Option<&Hooks<'_>>) -> Value {
        let mut report = json!({
            "case": self.case.name(), "arm": self.arm.name(), "repetition": self.repetition,
            "form": self.form.name(),
            "answer": answer("eof", None, None), "skipped_methods": [],
            "ended_on": null, "exit": null,
            "login": {
                "present_after": null, "bytes_equal": null, "mtime_equal": null,
                "inode_equal": null, "size_before": null, "size_after": null
            },
            "locks_appeared": {"auth.lock": false, "auth.json.lock": false},
            "wrote": false, "began_save": false,
            "home_before": [], "home_after": [], "home_added": [], "home_removed": [],
            "requests": [], "stderr_lines": null, "fault": null
        });
        if let Err(step) = self.observe(hooks, &mut report) {
            report["fault"] = json!(step);
        }
        report
    }

    fn observe(&self, hooks: Option<&Hooks<'_>>, report: &mut Value) -> Result<(), Fault> {
        let run = Scratch::new(self.scratch, "run").map_err(|_| "create run directory")?;
        let home = run.path().join("codex-home");
        let process_home = run.path().join("home");
        private_dir(&process_home).map_err(|_| "create process home")?;
        let login = Login::new(self.case, unix_now());
        let stub = Stub::start(Login::new(self.case, login.now)).map_err(|_| "start stub")?;
        login
            .write(&home, self.form, &stub.origin())
            .map_err(|_| "write fabricated login")?;
        if let Some(hooks) = hooks {
            (hooks.prepare)(run.path(), &process_home, stub.address())
                .map_err(|_| "prepare fake client")?;
        }
        let before = LoginState::read(&home).map_err(|_| "read login before start")?;
        let entries_before = home_entries(&home).map_err(|_| "list home before start")?;
        report["home_before"] = json!(entries_before);
        let mut watch = LockWatch::start(&home).map_err(|_| "start lock observation")?;

        let log = self
            .logs
            .join(run.path().file_name().unwrap())
            .with_extension("stderr");
        let stderr = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&log)
            .map_err(|_| "open stderr log")?;
        let mut command = isolated_command(self.client, run.path(), &process_home, &home);
        command
            .args([
                "app-server",
                "--stdio",
                "-c",
                "cli_auth_credentials_store=\"file\"",
                "-c",
                &format!("chatgpt_base_url=\"{}/backend-api\"", stub.origin()),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(stderr);
        match (self.arm, self.form) {
            (Arm::Blocked, _) => {
                command.env(OVERRIDE, BLOCK);
            }
            (Arm::Control, Form::Variable) => {
                command.env(OVERRIDE, format!("{}{ROTATION}", stub.origin()));
            }
            (Arm::Control, Form::Issuer) => {}
        }
        let mut group = Group::start(&mut command).map_err(|_| "start client")?;
        let exchanged = exchange(&mut group, run.path(), stub.address(), report);
        if report["answer"]["line"].is_string() {
            thread::sleep(self.hold);
        }
        let stopped = group.stop();
        drop(group);
        if let Ok((ended_on, status)) = &stopped {
            report["ended_on"] = json!(ended_on);
            report["exit"] = json!({"code": status.code(), "signal": status.signal()});
        }
        let mut fault = exchanged.err();
        if stopped.is_err() {
            fault.get_or_insert("stop client group");
        }

        match LoginState::read(&home) {
            Ok(after) => {
                report["wrote"] = json!(wrote(&before, &after));
                report["login"] = login_report(&before, &after);
            }
            Err(_) => {
                fault.get_or_insert("read login after stop");
            }
        }
        let mut locks = [false; 2];
        if watch.appeared(&mut locks).is_err() {
            fault.get_or_insert("read lock events");
        }
        match home_entries(&home) {
            Ok(after) => {
                for (i, name) in LOCKS.iter().enumerate() {
                    locks[i] |= after.contains(*name) && !entries_before.contains(*name);
                }
                report["home_after"] = json!(after);
                report["home_added"] = json!(after.difference(&entries_before).collect::<Vec<_>>());
                report["home_removed"] =
                    json!(entries_before.difference(&after).collect::<Vec<_>>());
            }
            Err(_) => {
                fault.get_or_insert("list home after stop");
            }
        }
        report["locks_appeared"] = json!({(LOCKS[0]): locks[0], (LOCKS[1]): locks[1]});
        report["began_save"] = json!(locks.into_iter().any(|present| present));
        match stub.finish() {
            Ok(requests) => report["requests"] = json!(requests),
            Err(_) => {
                fault.get_or_insert("finish stub log");
            }
        }
        match stderr_lines(&log) {
            Ok(lines) => report["stderr_lines"] = json!(lines),
            Err(_) => {
                fault.get_or_insert("count stderr lines");
            }
        }
        if let Some(hooks) = hooks
            && (hooks.inspect)(&process_home).is_err()
        {
            fault.get_or_insert("inspect fake client");
        }
        if run.remove().is_err() {
            fault.get_or_insert("remove run directory");
        }
        fault.map_or(Ok(()), Err)
    }
}

pub fn output_path(path: &Path) -> io::Result<PathBuf> {
    let name = path
        .file_name()
        .ok_or_else(|| io::Error::other("output filename"))?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let path = parent.canonicalize()?.join(name);
    match fs::symlink_metadata(&path) {
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "output exists",
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(path),
        Err(error) => Err(error),
    }
}

pub fn write_report(path: &Path, report: &Value) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    serde_json::to_writer_pretty(&mut file, report)?;
    writeln!(file)
}

pub fn report(form: Form, date: &str) -> Value {
    json!({
        "order": "208", "date": date, "form": form.name(), "codex_version": null,
        "machine": {}, "self_test": {}, "stop": null, "runs": []
    })
}

pub fn sitting(
    report: &mut Value,
    client: &Path,
    scratch: &Path,
    logs: &Path,
    form: Form,
    hold: Duration,
    hooks: Option<&Hooks<'_>>,
) {
    let mut faults = 0;
    for repetition in 1..=3 {
        for case in [Case::P1, Case::P2, Case::P3] {
            for arm in [Arm::Blocked, Arm::Control] {
                let run = Run {
                    client,
                    scratch,
                    logs,
                    case,
                    arm,
                    repetition,
                    form,
                    hold,
                };
                let observed = run.execute(hooks);
                faults += usize::from(!observed["fault"].is_null());
                report["runs"].as_array_mut().unwrap().push(observed);
                if faults == 3 {
                    report["stop"] = json!({"rule": 2, "check": "third run fault"});
                    return;
                }
            }
        }
    }
}
