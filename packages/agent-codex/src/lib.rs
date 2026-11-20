//! Client adapter. This source is copied into each independently buildable package.
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    net::{Ipv4Addr, TcpListener},
    path::{Path, PathBuf},
    process::Stdio,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    process::{Child, ChildStderr, ChildStdin, ChildStdout},
    time::{Instant, sleep_until, timeout},
};

pub const CLAUDE: bool = false;
pub const WRAPPER: &str = if CLAUDE {
    "agent-claude-code"
} else {
    "agent-codex"
};
const BASE: [&str; 6] = ["HOME", "LOGNAME", "PATH", "SHELL", "TERM", "USER"];
/// The client's home variable (claude-code.v2 and codex.v2, item 4).
pub const HOME_VARIABLE: &str = if CLAUDE {
    "CLAUDE_CONFIG_DIR"
} else {
    "CODEX_HOME"
};
/// The home's name inside the instance's state directory.
pub const HOME_NAME: &str = "client-home";
/// Codex keeps its login in a file in its home (codex.v2 item 4).
pub const CREDENTIAL_STORE: &str = "cli_auth_credentials_store=\"file\"";
/// A .v2 stop sends TERM, then KILL after this grace (the full review's order 16).
pub const STOP_GRACE: Duration = Duration::from_secs(2);
/// How long a .v2 stop waits after KILL before it names what outlived it.
pub const KILL_WAIT: Duration = Duration::from_secs(1);
/// The note of a stop that could not read the process table.
pub const TREE_UNCONFIRMED: &str =
    "client tree: the process table could not be read, so the stop is unconfirmed";
/// How long the retire hook waits for the client's logout (D53).
pub const LOGOUT_DEADLINE: Duration = Duration::from_secs(60);
/// The retire hook's answer once the client is signed out and its home emptied.
pub const RETIRED: &str = "client signed out and its home emptied";
/// Every turn once the retire hook has begun.
pub const RETIRED_REFUSAL: &str = "session retired; implicit restart refused";
/// How long the account hook waits, in all, for Codex's answers (ruling 82, point 1).
pub const ACCOUNT_DEADLINE: Duration = Duration::from_secs(30);
/// The account hook's text where Codex reported a ChatGPT account; the account is not in it.
pub const ACCOUNT_REPORTED: &str = "account hook (account/read): Codex reported a ChatGPT account";
/// The sign-out hook's answer once the client has signed out of the Agent's own home.
pub const SIGNED_OUT: &str = "client signed out of this Agent's own home";

/// The login names `codex.v2` states.
pub const BROWSER_LOGIN: &str = "browser";
pub const DEVICE_LOGIN: &str = "device";

/// One login `codex.v2` states, read from this package's own `contracts.yaml`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatedLogin {
    pub name: String,
    pub kind: String,
    pub authorization: String,
    pub client_id: String,
    pub host: Option<String>,
    pub ports: Vec<u16>,
    pub path: Option<String>,
    pub carried: Option<bool>,
    pub issuer: Option<String>,
}

pub fn stated_logins() -> Vec<StatedLogin> {
    parse_stated_logins(include_str!("../contracts.yaml")).unwrap_or_default()
}

#[doc(hidden)]
pub fn parse_stated_logins(contracts: &str) -> Option<Vec<StatedLogin>> {
    let doc = contracts
        .split("\n---\n")
        .find(|doc| doc.lines().any(|line| line == "  name: codex.v2"))?;
    let mut lines = doc.lines().skip_while(|line| *line != "  signIn:").skip(1);
    if lines.next()? != "    account: /account" || lines.next()? != "    logins:" {
        return None;
    }

    fn quoted(value: &str) -> Option<String> {
        let value = value.strip_prefix('\'')?.strip_suffix('\'')?;
        (!value.is_empty() && !value.contains('\'')).then(|| value.to_owned())
    }

    let mut logins = Vec::new();
    while let Some(line) = lines.next() {
        if line.starts_with("  ") && !line.starts_with("   ") {
            break;
        }
        let name = line.strip_prefix("    - name: ")?;
        let kind = lines.next()?.strip_prefix("      return: ")?;
        let authorization = quoted(lines.next()?.strip_prefix("      authorization: ")?)?;
        let client_id = quoted(lines.next()?.strip_prefix("      clientId: ")?)?;
        let mut login = StatedLogin {
            name: name.to_owned(),
            kind: kind.to_owned(),
            authorization,
            client_id,
            host: None,
            ports: Vec::new(),
            path: None,
            carried: None,
            issuer: None,
        };
        match (name, kind) {
            (BROWSER_LOGIN, "redirect") => {
                let redirect = lines.next()?.strip_prefix("      redirect: {host: ")?;
                let (host, rest) = redirect.split_once(", ports: [")?;
                if host.is_empty() || host.contains([',', ' ', ':', '{', '}']) {
                    return None;
                }
                let (ports, path) = rest.split_once("], path: ")?;
                login.host = Some(host.to_owned());
                let (first, second) = ports.split_once(", ")?;
                login.ports = vec![first.parse().ok()?, second.parse().ok()?];
                login.path = Some(quoted(path.strip_suffix('}')?)?);
                login.carried = Some(
                    lines
                        .next()?
                        .strip_prefix("      carried: ")?
                        .parse()
                        .ok()?,
                );
                login.issuer = Some(quoted(lines.next()?.strip_prefix("      issuer: ")?)?);
            }
            (DEVICE_LOGIN, "device") => {}
            _ => return None,
        }
        logins.push(login);
    }
    logins
        .iter()
        .map(|login| login.name.as_str())
        .eq([BROWSER_LOGIN, DEVICE_LOGIN])
        .then_some(logins)
}

fn parse_login_url(text: &str) -> Option<reqwest::Url> {
    // URL parsing can normalize away an empty user-info field. Check the authority as written.
    if text
        .bytes()
        .any(|byte| byte == b'\\' || byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return None;
    }
    let (_, after_scheme) = text.split_once("://")?;
    let authority = after_scheme.split(['/', '?', '#']).next()?;
    if authority.is_empty() || authority.contains(char::from(0x40)) {
        return None;
    }
    let url = reqwest::Url::parse(text).ok()?;
    let raw_path = after_scheme[authority.len()..].split('?').next()?;
    (url.username().is_empty()
        && url.password().is_none()
        && url.fragment().is_none()
        && raw_path == url.path())
    .then_some(url)
}

pub fn redirect_url_matches(login: &StatedLogin, text: &str) -> bool {
    if login.kind != "redirect" {
        return false;
    }
    let (Some(url), Some(authorization)) =
        (parse_login_url(text), parse_login_url(&login.authorization))
    else {
        return false;
    };
    if url.scheme() != "https" {
        return false;
    }
    let mut params = BTreeMap::new();
    for (name, value) in url.query_pairs() {
        // The pinned CLI adds only these fixed fields to its standard browser URL.
        // A configured workspace restriction or login hint is outside this flow.
        let allowed = match name.as_ref() {
            "client_id"
            | "response_type"
            | "code_challenge_method"
            | "code_challenge"
            | "state"
            | "redirect_uri" => true,
            "scope" => {
                value
                    == "openid profile email offline_access api.connectors.read api.connectors.invoke"
            }
            "id_token_add_organizations" | "codex_cli_simplified_flow" => value == "true",
            "originator" => value == "codex_cli_rs",
            _ => false,
        };
        if !allowed || params.insert(name, value).is_some() {
            return false;
        }
    }
    let param = |name| params.get(name).map(|value| value.as_ref());
    let base64url = |byte: u8| byte.is_ascii_alphanumeric() || b"-_".contains(&byte);
    if url.origin() != authorization.origin()
        || url.path() != authorization.path()
        || authorization.query().is_some()
        || param("client_id") != Some(login.client_id.as_str())
        || param("response_type") != Some("code")
        || param("code_challenge_method") != Some("S256")
        || !param("code_challenge")
            .is_some_and(|value| value.len() == 43 && value.bytes().all(base64url))
        || !param("state").is_some_and(|value| {
            value.len() >= 22
                && value
                    .bytes()
                    .all(|byte| base64url(byte) || b".~".contains(&byte))
        })
    {
        return false;
    }
    let Some(redirect) = param("redirect_uri").and_then(parse_login_url) else {
        return false;
    };
    redirect.scheme() == "http"
        && redirect.query().is_none()
        && redirect.host_str() == login.host.as_deref()
        && redirect
            .port()
            .is_some_and(|port| login.ports.contains(&port))
        && Some(redirect.path()) == login.path.as_deref()
}

/// How long the sign-in hook waits for Codex's login to print its URL.
pub const LOGIN_URL_DEADLINE: Duration = Duration::from_secs(30);
/// How long a started login may wait for the person before the wrapper stops it.
pub const LOGIN_LIMIT: Duration = Duration::from_secs(600);
/// The message of the URL request the hook gives; the engine writes its own (IFC-049).
pub const LOGIN_MESSAGE: &str = "Open this page to sign Codex in for this Agent.";
pub const DEVICE_REFUSAL: &str = "sign-in hook (device): PND-010: the device login cannot start yet: its user code has no path to the engine's panel until F83's code path and order 051c's panel exist; ask for the browser login, or for none";
pub const LOGIN_UNKNOWN: &str = "sign-in hook: PND-010: the asked-for login is not one that codex.v2 states; it states browser and device";
pub const RETURN_REFUSAL: &str = "sign-in hook (return): PND-013: this hook takes no return yet, and passed nothing to the login";
pub const LOGIN_WAITING: &str =
    "sign-in hook (browser): the browser login is waiting in this Agent's own home";
pub const LOGIN_ENDED: &str = "sign-in hook (browser): the browser login ended";
pub const STATE_UNKNOWN: &str =
    "sign-in hook (retry): PND-013: no login of this hook's holds that request state";

/// Codex's own login, with the Agent's home and its file store (review item 4).
pub fn login_arguments() -> Vec<String> {
    ["login", "-c", CREDENTIAL_STORE].map(str::to_owned).into()
}

/// What a sign-in call carried: the login asked for, whether a return came, and, on a retry,
/// the hook's own requestState. The return's value is never read.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SignInCall {
    pub login: Option<Value>,
    pub returned: bool,
    pub request_state: Option<String>,
}

/// The sign-in hook's answer: a URL to relay with the hook's requestState, or a text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SignIn {
    Url { url: String, request_state: String },
    Text(String),
}

const POLL: Duration = Duration::from_millis(20);
const LOGIN_TRACK: Duration = Duration::from_millis(250);

#[derive(Clone, Debug)]
pub struct Config {
    pub program: PathBuf,
    pub helper: PathBuf,
    pub cwd: PathBuf,
    pub env: BTreeMap<String, String>,
    pub url: String,
    pub identity: PathBuf,
    pub account_services: bool,
    pub peer_messaging: bool,
    pub model: Option<String>,
    pub timeout: Duration,
    pub v2: bool,
    pub directory: Option<PathBuf>,
    pub effort: Option<String>,
    pub state_dir: Option<PathBuf>,
    pub configuration_error: Option<String>,
}
impl Config {
    pub fn from_env() -> Result<Self> {
        let mut c = Self {
            program: if CLAUDE { "claude" } else { "codex" }.into(),
            helper: std::env::current_exe()?,
            cwd: std::env::current_dir()?,
            env: BASE
                .into_iter()
                .filter_map(|k| {
                    std::env::var(k)
                        .ok()
                        .filter(|v| !v.starts_with("()"))
                        .map(|v| (k.to_owned(), v))
                })
                .collect(),
            url: String::new(),
            identity: PathBuf::new(),
            account_services: false,
            peer_messaging: false,
            model: None,
            timeout: Duration::from_secs(180),
            v2: false,
            directory: None,
            effort: None,
            state_dir: None,
            configuration_error: None,
        };
        let mut validate = || -> Result<()> {
            let required = |key: &str| {
                std::env::var(key)
                    .map_err(|_| anyhow::anyhow!("{key}: missing or invalid encoding"))
            };
            let boolean = |key: &str| -> Result<bool> {
                match required(key)?.as_str() {
                    "true" => Ok(true),
                    "false" => Ok(false),
                    _ => bail!("{key}: expected true or false"),
                }
            };
            c.url = required("CONDUIT_URL")?;
            c.identity = required("CONDUIT_IDENTITY_DIR")?.into();
            c.env.insert("CONDUIT_URL".into(), c.url.clone());
            c.env.insert(
                "CONDUIT_IDENTITY_DIR".into(),
                c.identity.display().to_string(),
            );
            let v2 = std::env::var_os("AGENT_TURN_DEADLINE_SECONDS").is_some();
            c.v2 = v2;
            c.account_services = boolean("AGENT_ACCOUNT_SERVICES")?;
            c.peer_messaging = if v2 && std::env::var_os("AGENT_PEER_MESSAGING").is_none() {
                false
            } else {
                boolean("AGENT_PEER_MESSAGING")?
            };
            c.model = std::env::var("AGENT_MODEL").ok().filter(|s| !s.is_empty());
            if v2 {
                let state_dir = std::env::var_os("CONDUIT_STATE_DIR")
                    .filter(|dir| !dir.is_empty())
                    .map(PathBuf::from)
                    .with_context(|| {
                        format!("client home ({HOME_VARIABLE}): CONDUIT_STATE_DIR is unset")
                    })?;
                ensure!(
                    state_dir.is_absolute(),
                    "client home ({HOME_VARIABLE}): CONDUIT_STATE_DIR is not an absolute path"
                );
                c.state_dir = Some(state_dir);
                let n = required("AGENT_TURN_DEADLINE_SECONDS")?;
                ensure!(
                    !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()),
                    "turnDeadlineSeconds (AGENT_TURN_DEADLINE_SECONDS): expected whole number from 1 to 604800"
                );
                let n: u64 = n.parse().ok().filter(|n| (1..=604800).contains(n))
                    .context("turnDeadlineSeconds (AGENT_TURN_DEADLINE_SECONDS): expected whole number from 1 to 604800")?;
                c.timeout = Duration::from_secs(n);
                if let Some(dir) = std::env::var_os("AGENT_DIRECTORY") {
                    let dir = PathBuf::from(dir);
                    ensure!(
                        dir.is_absolute(),
                        "directory: not an absolute path: {}",
                        dir.display()
                    );
                    ensure!(
                        dir.is_dir(),
                        "directory: not an existing directory: {}",
                        dir.display()
                    );
                    c.directory = Some(dir.canonicalize().with_context(|| {
                        format!("directory: cannot canonicalize: {}", dir.display())
                    })?);
                }
                c.effort = std::env::var("AGENT_EFFORT").ok().filter(|s| !s.is_empty());
                if let Some(e) = &c.effort {
                    ensure!(
                        e.bytes().all(|b| b.is_ascii_lowercase()),
                        "effort (AGENT_EFFORT): expected lowercase ASCII letters"
                    );
                }
                if CLAUDE {
                    c.env
                        .insert("MCP_TOOL_TIMEOUT".into(), c.timeout_millis()?.to_string());
                }
            }
            Ok(())
        };
        if let Err(error) = validate() {
            c.configuration_error = Some(format!("invalid configuration: {error}"));
        }
        Ok(c)
    }
    fn timeout_millis(&self) -> Result<u64> {
        self.timeout
            .as_secs()
            .checked_mul(1000)
            .context("turnDeadlineSeconds: milliseconds overflow")
    }
    fn client_directory(&self) -> &Path {
        self.directory
            .as_deref()
            .filter(|_| self.v2)
            .unwrap_or(&self.cwd)
    }
}
/// Resolve and check the client home before a .v2 launch.
pub fn client_home(c: &Config) -> Result<PathBuf> {
    use std::io::ErrorKind::NotFound;

    let state_dir = c
        .state_dir
        .as_ref()
        .with_context(|| format!("client home ({HOME_VARIABLE}): CONDUIT_STATE_DIR is unset"))?;
    let state_dir = state_dir
        .canonicalize()
        .ok()
        .filter(|dir| dir.is_dir())
        .with_context(|| {
            format!("client home ({HOME_VARIABLE}): CONDUIT_STATE_DIR is not an existing directory")
        })?;
    let home = state_dir.join(HOME_NAME);
    if fs::symlink_metadata(&home).is_err_and(|error| error.kind() == NotFound) {
        #[cfg(unix)]
        let created = {
            use std::os::unix::fs::DirBuilderExt;
            fs::DirBuilder::new().mode(0o700).create(&home)
        };
        #[cfg(not(unix))]
        let created = fs::create_dir(&home);
        created.map_err(|_| {
            anyhow::anyhow!(
                "client home ({HOME_VARIABLE}): cannot create the home in CONDUIT_STATE_DIR"
            )
        })?;
    }
    let home = home.canonicalize().map_err(|_| {
        anyhow::anyhow!("client home ({HOME_VARIABLE}): the home is not a directory")
    })?;
    ensure!(
        home.starts_with(&state_dir) && home != state_dir,
        "client home ({HOME_VARIABLE}): the home resolves outside CONDUIT_STATE_DIR"
    );
    ensure!(
        home.is_dir(),
        "client home ({HOME_VARIABLE}): the home is not a directory"
    );
    if !CLAUDE {
        match fs::symlink_metadata(home.join(".env")) {
            Ok(_) => bail!(
                "client home ({HOME_VARIABLE}): the home holds a .env, which Codex would read into its environment; the wrapper refuses it, since the Agent holds no API key (ruling 83)"
            ),
            Err(error) if error.kind() == NotFound => {}
            Err(_) => bail!("client home ({HOME_VARIABLE}): cannot check the home for a .env"),
        }
        // Codex treats a non-Unicode CODEX_HOME as unset and uses its default home.
        ensure!(
            home.to_str().is_some(),
            "client home ({HOME_VARIABLE}): the canonical home path is not valid Unicode"
        );
    }
    Ok(home)
}
/// The client's own logout, run with the Agent's home alone (order 050g).
pub fn logout_arguments() -> Vec<String> {
    if CLAUDE {
        vec!["auth".into(), "logout".into()]
    } else {
        vec!["logout".into(), "-c".into(), CREDENTIAL_STORE.into()]
    }
}

/// Codex's app-server on stdio, with the Agent's home and its file store (ruling 82, point 1).
pub fn account_arguments() -> Vec<String> {
    ["app-server", "--stdio", "-c", CREDENTIAL_STORE]
        .map(str::to_owned)
        .into()
}

/// Codex's refresh endpoint override, set on the account check's child alone and stripped
/// from every other child's environment (ruling 86, point 5).
pub const REFRESH_OVERRIDE_VARIABLE: &str = "CODEX_REFRESH_TOKEN_URL_OVERRIDE";
/// An endpoint that never resolves (RFC 6761), so that the check completes no refresh.
pub const REFRESH_BLOCKED: &str = "https://refresh.invalid/oauth/token";
/// Codex's revoke endpoint override, stripped from every child's environment, the account
/// check's included, so that no template can send the stored tokens where it chooses.
pub const REVOKE_OVERRIDE_VARIABLE: &str = "CODEX_REVOKE_TOKEN_URL_OVERRIDE";

/// What the account hook reports: the account Codex gave, or none, and a text that names the
/// step and the cause and never a value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountReport {
    pub account: Option<String>,
    pub text: String,
}

fn empty_home(home: &Path) -> Result<()> {
    fn remove_entries(home: &Path) -> io::Result<()> {
        for entry in fs::read_dir(home)? {
            let path = entry?.path();
            if fs::symlink_metadata(&path)?.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }
    remove_entries(home)
        .map_err(|_| anyhow::anyhow!("client home ({HOME_VARIABLE}): cannot empty the home"))
}

enum Reply {
    Result(Value),
    Failed(String),
    Eof,
}

async fn ask(
    input: &mut ChildStdin,
    lines: &mut Lines<BufReader<ChildStdout>>,
    id: u64,
    method: &str,
    params: Value,
) -> Reply {
    let request = json!({"id": id, "method": method, "params": params});
    if input
        .write_all(format!("{request}\n").as_bytes())
        .await
        .is_err()
        || input.flush().await.is_err()
    {
        return Reply::Eof;
    }
    while let Ok(Some(line)) = lines.next_line().await {
        let Ok(mut message) = serde_json::from_str::<Value>(&line) else {
            return Reply::Failed("invalid answer".into());
        };
        if message.get("method").is_some() {
            continue;
        }
        if message["id"] != json!(id) {
            return Reply::Failed("invalid answer".into());
        }
        if let Some(error) = message.get("error") {
            let cause = match error["code"].as_i64() {
                Some(code) => format!("error answer, code {code}"),
                None => "error answer".into(),
            };
            return Reply::Failed(cause);
        }
        return match message.get_mut("result") {
            Some(result) => Reply::Result(result.take()),
            None => Reply::Failed("invalid answer".into()),
        };
    }
    Reply::Eof
}

fn account_answer(result: &Value) -> Option<AccountReport> {
    let none = |cause: &str| AccountReport {
        account: None,
        text: format!("account hook (account/read): no account: {cause}"),
    };
    let account = result.as_object()?.get("account")?;
    if account.is_null() {
        return Some(none("Codex answered with none"));
    }
    let account = account.as_object()?;
    if account.get("type")?.as_str()? != "chatgpt" {
        return Some(none("the login is not a ChatGPT login"));
    }
    match account.get("email")? {
        Value::String(email) if !email.is_empty() => Some(AccountReport {
            account: Some(email.clone()),
            text: ACCOUNT_REPORTED.into(),
        }),
        Value::String(_) | Value::Null => Some(none("the ChatGPT login names no email")),
        _ => None,
    }
}

async fn read_account(
    c: &Config,
    home: &Path,
    tree: &ClientTree,
    deadline: Duration,
) -> Result<AccountReport> {
    let until = Instant::now() + deadline;
    let start_error = "account hook (start): client did not start";
    ensure!(
        c.client_directory().is_dir(),
        "{start_error}: the client directory is gone"
    );
    let (mut child, mut input, mut lines, pid, start) = {
        let mut state = tree.0.lock().unwrap();
        ensure!(!state.closed, "{start_error}");
        let mut child = client_command(c, Some(home), &account_arguments())
            .env(REFRESH_OVERRIDE_VARIABLE, REFRESH_BLOCKED)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| anyhow::anyhow!(start_error))?;
        let pid = child.id().context(start_error)? as i32;
        let start = process(pid).ok().flatten().map(|p| p.start);
        state.groups.insert(pid, start);
        let input = child.stdin.take().context(start_error)?;
        let lines = BufReader::new(child.stdout.take().context(start_error)?).lines();
        (child, input, lines, pid, start)
    };

    let mut step = "initialize";
    let reply = tokio::time::timeout_at(until, async {
        match ask(
            &mut input,
            &mut lines,
            1,
            step,
            json!({
                "clientInfo": {"name": WRAPPER, "version": env!("CARGO_PKG_VERSION")},
                "capabilities": {"experimentalApi": false},
            }),
        )
        .await
        {
            Reply::Result(_) => {}
            other => return other,
        }
        if input
            .write_all(b"{\"method\":\"initialized\"}\n")
            .await
            .is_err()
            || input.flush().await.is_err()
        {
            return Reply::Eof;
        }
        step = "account/read";
        ask(
            &mut input,
            &mut lines,
            2,
            step,
            json!({"refreshToken": false}),
        )
        .await
    })
    .await;
    let mut seen = BTreeMap::new();
    let mut scan_failed = false;
    let answer = match reply {
        Err(_) => Err(format!("no answer within {} seconds", deadline.as_secs())),
        Ok(Reply::Eof) => {
            // Read the tree before the exit wait, so a descendant reparented by the exit is found.
            match scan_group(pid, start) {
                Some(found) => seen = found,
                None => scan_failed = true,
            }
            Err(match timeout(EXIT_WAIT, child.wait()).await {
                Ok(Ok(status)) => exit_cause(status),
                Ok(Err(_)) => "client exit: status unknown".into(),
                Err(_) => "EOF before answer".into(),
            })
        }
        Ok(Reply::Failed(cause)) => Err(cause),
        Ok(Reply::Result(result)) => account_answer(&result).ok_or_else(|| "invalid answer".into()),
    };
    let stop = tree.stop_group(pid, start, seen).await;
    let stop = if scan_failed {
        StopReport::UNCONFIRMED
    } else {
        stop
    };
    drop(input);
    let _ = child.kill().await;
    let _ = child.wait().await;
    match (answer, stop.note()) {
        (Ok(report), None) => Ok(report),
        (Ok(_), Some(note)) => bail!("account hook (stop): {note}"),
        (Err(cause), note) => {
            let text = format!("account hook ({step}): {cause}");
            bail!(match note {
                Some(note) => append_stop_note(Some(text), &note),
                None => text,
            });
        }
    }
}

fn quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
pub fn helper_command(config: &Config) -> String {
    let cmd = format!(
        "{} --headers-helper",
        quote(&config.helper.display().to_string())
    );
    if CLAUDE {
        cmd
    } else {
        format!(
            "{cmd} --identity-dir {}",
            quote(&config.identity.display().to_string())
        )
    }
}
/// The sole credential-reading path. Errors never include the token or file contents.
pub fn header(dir: &Path) -> Result<Value> {
    let token = fs::read_to_string(dir.join("token")).context("cannot read run token")?;
    let token = token.trim();
    ensure!(
        !token.is_empty() && !token.contains(['\r', '\n']),
        "invalid run token"
    );
    Ok(json!({"Authorization": format!("Bearer {token}")}))
}
pub fn settings(c: &Config) -> Value {
    let mut allow = vec!["mcp__engine__*"];
    if c.peer_messaging {
        allow.extend(["SendMessage", "ListAgents", "RemoteTrigger"]);
    }
    let mut settings = json!({"permissions":{"allow":allow,"deny":if c.peer_messaging {vec![]} else {vec!["SendMessage","ListAgents","RemoteTrigger"]},"defaultMode":"dontAsk"},"disableAllHooks":true,"enabledPlugins":{},"disableClaudeAiConnectors":!c.account_services});
    if c.v2 && c.directory.is_some() {
        settings["permissions"]["defaultMode"] = json!("bypassPermissions");
    }
    if !c.peer_messaging {
        settings["crossSessionInbound"] = json!("refuse");
    }
    settings
}
pub fn prepare(c: &Config) -> Result<()> {
    if CLAUDE {
        fs::write(
            c.cwd.join("settings.json"),
            serde_json::to_vec(&settings(c))?,
        )?;
        let mut mcp = json!({"mcpServers":{"engine":{"type":"http","url":c.url,"headersHelper":helper_command(c)}}});
        if c.v2 {
            mcp["mcpServers"]["engine"]["timeout"] = json!(c.timeout_millis()?);
        }
        fs::write(c.cwd.join("mcp.json"), serde_json::to_vec(&mcp)?)?;
    }
    Ok(())
}
pub fn arguments(c: &Config, thread: Option<&str>) -> Vec<String> {
    let mut args: Vec<String> = if CLAUDE {
        vec![
            "--print".into(),
            "--setting-sources".into(),
            "".into(),
            "--settings".into(),
            c.cwd.join("settings.json").display().to_string(),
            "--strict-mcp-config".into(),
            "--mcp-config".into(),
            c.cwd.join("mcp.json").display().to_string(),
        ]
    } else {
        [
            "exec",
            "--sandbox",
            if c.v2 && c.directory.is_some() {
                "danger-full-access"
            } else {
                "read-only"
            },
            "--color",
            "never",
        ]
        .map(str::to_owned)
        .into()
    };
    if let Some(model) = &c.model {
        args.extend(["--model".into(), model.clone()]);
    }
    if !CLAUDE && c.v2 {
        args.extend(["--cd".into(), c.client_directory().display().to_string()]);
    }
    if CLAUDE {
        args.extend(
            [
                "--tools",
                if c.peer_messaging {
                    "SendMessage,ListAgents,RemoteTrigger"
                } else {
                    ""
                },
                "--allowedTools",
                if c.peer_messaging {
                    "mcp__engine__*,SendMessage,ListAgents,RemoteTrigger"
                } else {
                    "mcp__engine__*"
                },
                "--no-session-persistence",
                "--output-format",
                "stream-json",
                "--input-format",
                "stream-json",
                "--verbose",
                "--include-partial-messages",
                "--disable-slash-commands",
            ]
            .map(str::to_owned),
        );
        if !c.peer_messaging {
            args.extend([
                "--disallowedTools".into(),
                "SendMessage,ListAgents,RemoteTrigger".into(),
            ]);
        }
        if c.v2 && c.directory.is_some() {
            let i = args.iter().position(|a| a == "--tools").unwrap();
            let mut tools = "Bash,Read,Edit,Write,Glob,Grep,WebFetch".to_owned();
            if c.account_services {
                tools.push_str(",WebSearch");
            }
            if c.peer_messaging {
                tools.push_str(",SendMessage,ListAgents,RemoteTrigger");
            }
            args[i + 1] = tools;
            args.extend(
                [
                    "--permission-mode",
                    "bypassPermissions",
                    "--permission-prompts",
                    "none",
                ]
                .map(str::to_owned),
            );
        }
        if c.v2
            && let Some(effort) = &c.effort
        {
            args.extend(["--effort".into(), effort.clone()]);
        }
    } else {
        if thread.is_some() {
            args.push("resume".into());
        }
        args.extend(
            [
                "--ignore-user-config",
                "--strict-config",
                "--ignore-rules",
                "--skip-git-repo-check",
                "--json",
            ]
            .map(str::to_owned),
        );
        args.extend([
            if c.account_services {
                "--enable"
            } else {
                "--disable"
            }
            .into(),
            "apps".into(),
        ]);
        for feature in [
            "shell_tool",
            "unified_exec",
            "browser_use",
            "browser_use_external",
            "in_app_browser",
            "computer_use",
            "multi_agent",
            "multi_agent_v2",
            "multi_agent_v2_dynamic_tools",
            "hooks",
            "plugins",
            "view_image",
            "goals",
            "image_generation",
        ] {
            if c.v2
                && c.directory.is_some()
                && matches!(feature, "shell_tool" | "unified_exec" | "view_image")
            {
                continue;
            }
            args.extend(["--disable".into(), feature.into()]);
        }
        for setting in [
            format!("mcp_servers.engine.url={}", json!(c.url)),
            format!(
                "mcp_servers.engine.http_headers_helper={}",
                json!(helper_command(c))
            ),
            "mcp_servers.engine.startup_readiness=\"connection\"".into(),
            "mcp_servers.engine.required=true".into(),
            "mcp_servers.engine.default_tools_approval_mode=\"approve\"".into(),
            if c.v2 && c.directory.is_some() && c.account_services {
                "web_search=\"live\""
            } else {
                "web_search=\"disabled\""
            }
            .into(),
            "project_doc_max_bytes=0".into(),
        ] {
            args.extend(["-c".into(), setting]);
        }
        if c.v2 {
            args.extend(["-c".into(), CREDENTIAL_STORE.into()]);
            args.extend([
                "-c".into(),
                format!(
                    "mcp_servers.engine.tool_timeout_sec={}",
                    c.timeout.as_secs()
                ),
            ]);
            if let Some(effort) = &c.effort {
                args.extend([
                    "-c".into(),
                    format!("model_reasoning_effort={}", json!(effort)),
                ]);
            }
        }
        if let Some(thread) = thread {
            args.push(thread.into());
        }
        args.push("-".into());
    }
    args
}
const EXIT_WAIT: Duration = Duration::from_secs(2);
const ERROR_CLASSES: &[(&str, &[&str])] = &[
    ("usage-limit", &["usage limit", "usage_limit", "quota"]),
    (
        "rate-limit",
        &["rate limit", "rate_limit", "too many requests", "429"],
    ),
    (
        "authentication",
        &[
            "authentication",
            "unauthorized",
            "401",
            "login",
            "log in",
            "api key",
            "credential",
        ],
    ),
    (
        "context-window",
        &[
            "context window",
            "context_window",
            "context length",
            "context_length",
            "prompt is too long",
        ],
    ),
    (
        "network",
        &[
            "network",
            "connection",
            "disconnected",
            "error sending request",
            "timed out",
            "timeout",
            "dns",
            "getaddrinfo",
            "econnreset",
            "econnrefused",
            "enotfound",
            "unreachable",
            "fetch failed",
        ],
    ),
];
fn classify(text: Option<&str>) -> &'static str {
    let text = text.unwrap_or_default().to_ascii_lowercase();
    ERROR_CLASSES
        .iter()
        .find(|(_, needles)| needles.iter().any(|needle| text.contains(needle)))
        .map_or("other", |(class, _)| *class)
}
fn exit_cause(status: std::process::ExitStatus) -> String {
    if let Some(code) = status.code() {
        return format!("client exit: status {code}");
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return format!("client exit: signal {signal}");
        }
    }
    "client exit: status unknown".into()
}
struct Process {
    pid: i32,
    parent: i32,
    group: i32,
    start: u64,
    zombie: bool,
}

#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn parse_stat(pid: i32, text: &str) -> Option<Process> {
    let mut fields = text.rsplit_once(')')?.1.split_ascii_whitespace();
    let zombie = matches!(fields.next()?, "Z" | "X");
    let parent = fields.next()?.parse().ok()?;
    let group = fields.next()?.parse().ok()?;
    let start = fields.nth(16)?.parse().ok()?;
    Some(Process {
        pid,
        parent,
        group,
        start,
        zombie,
    })
}

#[cfg(target_os = "linux")]
fn processes() -> io::Result<Vec<Process>> {
    let mut table = Vec::new();
    for entry in fs::read_dir("/proc")? {
        let name = entry?.file_name();
        if let Some(name) = name.to_str()
            && name.bytes().all(|byte| byte.is_ascii_digit())
            && let Ok(pid) = name.parse()
            && let Some(p) = process(pid)?
        {
            table.push(p);
        }
    }
    // A working procfs always shows the wrapper itself; without it the table is not read.
    if !table.iter().any(|p| p.pid == std::process::id() as i32) {
        return Err(io::ErrorKind::NotFound.into());
    }
    Ok(table)
}

#[cfg(target_os = "linux")]
fn process(pid: i32) -> io::Result<Option<Process>> {
    Ok(fs::read(format!("/proc/{pid}/stat"))
        .ok()
        .and_then(|line| parse_stat(pid, &String::from_utf8_lossy(&line))))
}

#[cfg(target_os = "macos")]
fn processes() -> io::Result<Vec<Process>> {
    // A null buffer asks for an estimate. Leave room for new PIDs and retry if filled.
    let count = unsafe { libc::proc_listallpids(std::ptr::null_mut(), 0) };
    if count <= 0 {
        return Err(io::Error::last_os_error());
    }
    let mut capacity = count as usize * 2;
    loop {
        let bytes = i32::try_from(capacity * std::mem::size_of::<libc::pid_t>())
            .map_err(io::Error::other)?;
        let mut pids = vec![0; capacity];
        let count = unsafe { libc::proc_listallpids(pids.as_mut_ptr().cast(), bytes) };
        if count <= 0 {
            return Err(io::Error::last_os_error());
        }
        if (count as usize) < capacity {
            pids.truncate(count as usize);
            return Ok(pids
                .into_iter()
                .filter(|&pid| pid > 0)
                .filter_map(|pid| process(pid).ok().flatten())
                .collect());
        }
        capacity *= 2;
    }
}

#[cfg(target_os = "macos")]
fn process(pid: i32) -> io::Result<Option<Process>> {
    let mut info = std::mem::MaybeUninit::<libc::proc_bsdinfo>::uninit();
    let size = std::mem::size_of::<libc::proc_bsdinfo>() as i32;
    // Argument 1 includes zombies. Only tree and lifetime fields leave this read.
    let count = unsafe {
        libc::proc_pidinfo(
            pid,
            libc::PROC_PIDTBSDINFO,
            1,
            info.as_mut_ptr().cast(),
            size,
        )
    };
    if count != size {
        return Ok(None);
    }
    let info = unsafe { info.assume_init() };
    Ok(Some(Process {
        pid,
        parent: info.pbi_ppid as i32,
        group: info.pbi_pgid as i32,
        start: info.pbi_start_tvsec * 1_000_000 + info.pbi_start_tvusec,
        zombie: info.pbi_status == libc::SZOMB,
    }))
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn processes() -> io::Result<Vec<Process>> {
    Err(io::ErrorKind::Unsupported.into())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn process(_: i32) -> io::Result<Option<Process>> {
    Err(io::ErrorKind::Unsupported.into())
}

/// The client's tree, and the lock that lets one stop of it run at a time.
#[derive(Clone, Default)]
pub struct ClientTree(Arc<Mutex<TreeState>>, Arc<tokio::sync::Mutex<()>>);

#[derive(Default)]
struct TreeState {
    groups: BTreeMap<i32, Option<u64>>,
    seen: BTreeMap<i32, u64>,
    closed: bool,
}

struct TreeReading {
    pids: Vec<i32>,
    unconfirmed: bool,
}

#[derive(Default)]
struct SignalDelivery {
    direct: BTreeSet<(i32, u64)>,
    via_group: BTreeMap<(i32, u64), i32>,
}

impl SignalDelivery {
    fn signal(&mut self, p: &Process, signal: i32, via_group: bool) {
        let identity = (p.pid, p.start);
        if via_group {
            self.via_group.insert(identity, p.group);
        }
        // A process can leave a group between sampling it and signalling the group.
        if self.via_group.get(&identity) == Some(&p.group) {
            return;
        }
        self.via_group.remove(&identity);
        if !self.direct.contains(&identity)
            && process(p.pid).is_ok_and(|entry| {
                entry.is_some_and(|current| !current.zombie && current.start == p.start)
            })
            && unsafe { libc::kill(p.pid, signal) } == 0
        {
            self.direct.insert(identity);
        }
    }
}

impl TreeReading {
    fn into_report(mut self) -> StopReport {
        self.pids.sort_unstable();
        StopReport {
            survivors: self.pids,
            unconfirmed: self.unconfirmed,
        }
    }
}

impl TreeState {
    fn for_group(group: i32, start: u64, seen: BTreeMap<i32, u64>) -> Self {
        Self {
            groups: BTreeMap::from([(group, Some(start))]),
            seen,
            ..Self::default()
        }
    }

    fn find(&mut self, table: Vec<Process>) -> Vec<Process> {
        let starts: BTreeMap<_, _> = table.iter().map(|p| (p.pid, p.start)).collect();
        let live_groups: BTreeSet<_> = table
            .iter()
            .filter(|p| !p.zombie)
            .map(|p| p.group)
            .collect();
        self.groups.retain(|group, start| {
            live_groups.contains(group)
                && !matches!((start, starts.get(group)), (Some(old), Some(new)) if old != new)
        });

        let own_pid = std::process::id() as i32;
        let mut found = BTreeMap::new();
        let mut remaining: Vec<_> = table
            .iter()
            .filter(|p| !p.zombie && p.pid != own_pid)
            .collect();
        remaining.retain(|p| {
            if matches!(self.groups.get(&p.group), Some(Some(_)))
                || self.seen.get(&p.pid) == Some(&p.start)
            {
                found.insert(p.pid, p.start);
                false
            } else {
                true
            }
        });
        loop {
            let before = found.len();
            remaining.retain(|p| {
                if found.contains_key(&p.parent) {
                    found.insert(p.pid, p.start);
                    false
                } else {
                    true
                }
            });
            if found.len() == before {
                break;
            }
        }
        self.seen = found;
        table
            .into_iter()
            .filter(|p| self.seen.contains_key(&p.pid))
            .collect()
    }
}

impl ClientTree {
    /// Two stops of one tree never run at once. A stop waits for the one running; once the
    /// wrapper's TERM stop (`close`) has begun, any other stop waits for good, since TERM's
    /// stop ends in the wrapper's exit.
    pub async fn stop(&self, close: bool) -> StopReport {
        if close {
            self.0.lock().unwrap().closed = true;
        } else if self.0.lock().unwrap().closed {
            return std::future::pending().await;
        }
        let running = self.1.lock().await;
        let (superseded, empty) = {
            let state = self.0.lock().unwrap();
            (
                state.closed && !close,
                state.groups.is_empty() && state.seen.is_empty(),
            )
        };
        if superseded {
            drop(running);
            return std::future::pending().await;
        }
        if empty {
            return StopReport::default();
        }

        let mut term = SignalDelivery::default();
        match self.read_and_signal(libc::SIGTERM, true, &mut term) {
            Ok(reading) if reading.pids.is_empty() => return reading.into_report(),
            Ok(_) => {}
            Err(_) => return self.unconfirmed_stop(None).await,
        }
        let term_until = Instant::now() + STOP_GRACE;
        match self.poll(libc::SIGTERM, &mut term, term_until).await {
            Ok(reading) if reading.pids.is_empty() => return reading.into_report(),
            Ok(_) => {}
            Err(_) => return self.unconfirmed_stop(Some(term_until)).await,
        }

        let mut kill = SignalDelivery::default();
        match self.read_and_signal(libc::SIGKILL, true, &mut kill) {
            Ok(reading) if reading.pids.is_empty() => return reading.into_report(),
            Ok(_) => {}
            Err(_) => return self.unconfirmed_stop(Some(term_until)).await,
        }
        let kill_until = Instant::now() + KILL_WAIT;
        match self.poll(libc::SIGKILL, &mut kill, kill_until).await {
            Ok(reading) => reading.into_report(),
            Err(_) => {
                sleep_until(kill_until).await;
                StopReport::UNCONFIRMED
            }
        }
    }

    /// Stop one group's tree, keeping the other groups and their discovery state intact. `seen`
    /// holds identities that an earlier `scan_group` of this group found.
    pub async fn stop_group(
        &self,
        group: i32,
        start: Option<u64>,
        seen: BTreeMap<i32, u64>,
    ) -> StopReport {
        if self.0.lock().unwrap().closed {
            return std::future::pending().await;
        }
        let running = self.1.lock().await;
        if self.0.lock().unwrap().closed {
            drop(running);
            return std::future::pending().await;
        }
        let Some(start) = start else {
            return StopReport::UNCONFIRMED;
        };
        let mut state = TreeState::for_group(group, start, seen);
        let mut survivors = Vec::new();
        for (signal, grace) in [(libc::SIGTERM, STOP_GRACE), (libc::SIGKILL, KILL_WAIT)] {
            let mut delivery = SignalDelivery::default();
            let mut first = true;
            let mut until: Option<Instant> = None;
            loop {
                let Ok(table) = processes() else {
                    return StopReport::UNCONFIRMED;
                };
                let found = state.find(table);
                if found.is_empty() {
                    let mut tree = self.0.lock().unwrap();
                    if tree.groups.get(&group) == Some(&Some(start)) {
                        tree.groups.remove(&group);
                    }
                    return StopReport::default();
                }
                let group_signalled = first
                    && state.groups.contains_key(&group)
                    && signal_group(group, start, signal, false);
                for p in &found {
                    delivery.signal(p, signal, group_signalled && p.group == group);
                }
                let until = *until.get_or_insert_with(|| Instant::now() + grace);
                if Instant::now() >= until {
                    survivors = found.into_iter().map(|p| p.pid).collect();
                    break;
                }
                first = false;
                sleep_until(until.min(Instant::now() + POLL)).await;
            }
        }
        TreeReading {
            pids: survivors,
            unconfirmed: false,
        }
        .into_report()
    }

    /// Record a group that this tree may need to stop.
    pub fn record_group(&self, pid: i32) -> bool {
        let mut state = self.0.lock().unwrap();
        if state.closed {
            return false;
        }
        state
            .groups
            .insert(pid, process(pid).ok().flatten().map(|p| p.start));
        true
    }

    async fn poll(
        &self,
        signal: i32,
        delivery: &mut SignalDelivery,
        until: Instant,
    ) -> io::Result<TreeReading> {
        loop {
            sleep_until(until.min(Instant::now() + POLL)).await;
            let reading = self.read_and_signal(signal, false, delivery)?;
            if reading.pids.is_empty() || Instant::now() >= until {
                return Ok(reading);
            }
        }
    }

    fn read_and_signal(
        &self,
        signal: i32,
        signal_groups: bool,
        delivery: &mut SignalDelivery,
    ) -> io::Result<TreeReading> {
        let mut state = self.0.lock().unwrap();
        let found = state.find(processes()?);
        let mut groups_signalled = BTreeSet::new();
        if signal_groups && !found.is_empty() {
            for (&group, &start) in &state.groups {
                if let Some(start) = start
                    && signal_group(group, start, signal, false)
                {
                    groups_signalled.insert(group);
                }
            }
        }
        for p in &found {
            delivery.signal(
                p,
                signal,
                signal == libc::SIGTERM && groups_signalled.contains(&p.group),
            );
        }
        Ok(TreeReading {
            pids: found.into_iter().map(|p| p.pid).collect(),
            unconfirmed: state.groups.values().any(Option::is_none),
        })
    }

    async fn unconfirmed_stop(&self, term_until: Option<Instant>) -> StopReport {
        let term_until = match term_until {
            Some(until) => until,
            None if self.signal_confirmed_groups(libc::SIGTERM) => Instant::now() + STOP_GRACE,
            None => return StopReport::UNCONFIRMED,
        };
        sleep_until(term_until).await;
        self.signal_confirmed_groups(libc::SIGKILL);
        tokio::time::sleep(KILL_WAIT).await;
        StopReport::UNCONFIRMED
    }

    fn signal_confirmed_groups(&self, signal: i32) -> bool {
        let state = self.0.lock().unwrap();
        let mut sent = false;
        for (&group, &start) in &state.groups {
            if let Some(start) = start {
                sent |= signal_group(group, start, signal, true);
            }
        }
        sent
    }
}

/// One reading of a group's tree, seeded from that group alone, as `stop_group` reads it.
/// Returns `None` without a known start time or if the process table cannot be read.
fn scan_group(group: i32, start: Option<u64>) -> Option<BTreeMap<i32, u64>> {
    let start = start?;
    let table = processes().ok()?;
    let mut state = TreeState::for_group(group, start, BTreeMap::new());
    state.find(table);
    Some(state.seen)
}

fn signal_group(group: i32, start: u64, signal: i32, require_leader: bool) -> bool {
    group > 0
        && group != unsafe { libc::getpgrp() }
        && process(group).is_ok_and(|entry| entry.map_or(!require_leader, |p| p.start == start))
        && unsafe { libc::kill(-group, signal) } == 0
}

pub fn survivor_note(pids: &[i32]) -> Option<String> {
    if pids.is_empty() {
        return None;
    }
    let mut pids = pids.to_vec();
    pids.sort_unstable();
    let list: Vec<_> = pids.iter().map(i32::to_string).collect();
    Some(format!(
        "client tree: PIDs {} outlived the stop",
        list.join(", ")
    ))
}

/// What a .v2 stop left: the PIDs that outlived it, ascending, and whether it is unconfirmed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StopReport {
    pub survivors: Vec<i32>,
    pub unconfirmed: bool,
}

impl StopReport {
    const UNCONFIRMED: Self = Self {
        survivors: Vec::new(),
        unconfirmed: true,
    };

    pub fn note(&self) -> Option<String> {
        let survivors = survivor_note(&self.survivors);
        if self.unconfirmed {
            Some(append_stop_note(survivors, TREE_UNCONFIRMED))
        } else {
            survivors
        }
    }
}

/// A failed turn (the full review's order 17): fixed words and numbers, never client text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FailedTurn {
    pub turn: usize,
    pub cause: String,
    pub client_started: bool,
    pub stop: StopReport,
    v2: bool,
}

impl std::fmt::Display for FailedTurn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "client turn {} failed: {}; session cannot restart implicitly",
            self.turn, self.cause
        )
    }
}

impl std::error::Error for FailedTurn {}

impl FailedTurn {
    pub fn kind(&self) -> &'static str {
        match self.cause.as_str() {
            cause if cause.starts_with("deadline: ") => "deadline",
            "cancelled" => "cancelled",
            cause if cause.starts_with("client failure: ") => "client-failure",
            cause if cause.starts_with("client exit: ") => "client-exit",
            "EOF before turn boundary" => "eof",
            "no thread" => "no-thread",
            "invalid client output" => "invalid-client-output",
            "resume changed thread" => "resume-changed-thread",
            "client did not start" => "client-did-not-start",
            cause if cause.starts_with("client home (") => "client-home",
            _ => "unclassified",
        }
    }

    pub fn class(&self) -> Option<&str> {
        self.cause.strip_prefix("client failure: ")
    }

    pub fn fields(&self) -> Option<Value> {
        self.v2.then(|| {
            json!({"failedTurn": {
                "turn": self.turn,
                "cause": self.kind(),
                "class": self.class(),
                "clientStarted": self.client_started,
                "survivors": self.stop.survivors,
                "stopConfirmed": !self.stop.unconfirmed,
            }})
        })
    }
}

fn append_stop_note(text: Option<String>, note: &str) -> String {
    text.map_or_else(|| note.to_owned(), |text| format!("{text}; {note}"))
}

fn client_command(c: &Config, home: Option<&Path>, args: &[String]) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new(&c.program);
    if c.v2 {
        cmd.process_group(0);
    }
    cmd.args(args)
        .env_clear()
        .envs(&c.env)
        .envs(home.map(|home| (HOME_VARIABLE, home)))
        .env_remove("ANTHROPIC_API_KEY")
        .env_remove("OPENAI_API_KEY")
        .env_remove(REFRESH_OVERRIDE_VARIABLE)
        .env_remove(REVOKE_OVERRIDE_VARIABLE)
        .current_dir(c.client_directory())
        .kill_on_drop(true);
    cmd
}

async fn logout(c: &Config, home: &Path, tree: &ClientTree, deadline: Duration) -> Result<()> {
    let mut child = {
        let mut state = tree.0.lock().unwrap();
        ensure!(!state.closed, "client logout did not start");
        let mut command = client_command(c, Some(home), &logout_arguments());
        if !c.client_directory().is_dir() {
            // Retirement can outlast the project directory.
            command.current_dir(if c.cwd.is_dir() { &c.cwd } else { home });
        }
        let child = command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| anyhow::anyhow!("client logout did not start"))?;
        let pid = child.id().context("client logout did not start")? as i32;
        let start = process(pid).ok().flatten().map(|p| p.start);
        state.groups.insert(pid, start);
        child
    };
    match timeout(deadline, child.wait()).await {
        Ok(Ok(status)) if status.success() => Ok(()),
        Ok(Ok(status)) => bail!("client logout failed: {}", exit_cause(status)),
        Ok(Err(_)) => bail!("client logout failed: client exit: status unknown"),
        Err(_) => {
            let stop = tree.stop(false).await;
            let _ = child.kill().await;
            let _ = child.wait().await;
            let cause = format!(
                "client logout failed: no exit within {} seconds",
                deadline.as_secs()
            );
            match stop.note() {
                Some(note) => bail!("{}", append_stop_note(Some(cause), &note)),
                None => bail!("{cause}"),
            }
        }
    }
}

struct Stream {
    child: Child,
    input: Option<ChildStdin>,
    lines: Lines<BufReader<ChildStdout>>,
}
impl Stream {
    async fn boundary_failure(&mut self, class: Option<&str>) -> String {
        let status = timeout(EXIT_WAIT, self.child.wait()).await;
        if let Some(class) = class {
            return format!("client failure: {class}");
        }
        match status {
            Ok(Ok(status)) if !status.success() => exit_cause(status),
            Ok(Err(_)) => "client exit: status unknown".into(),
            _ => "EOF before turn boundary".into(),
        }
    }
    fn spawn(c: &Config, args: &[String], tree: &ClientTree) -> Result<Self> {
        let mut state = c.v2.then(|| tree.0.lock().unwrap());
        ensure!(
            state.as_ref().is_none_or(|state| !state.closed),
            "client did not start"
        );
        let home = if c.v2 { Some(client_home(c)?) } else { None };
        let mut child = client_command(c, home.as_deref(), args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| anyhow::anyhow!("client did not start"))?;
        if let Some(state) = &mut state {
            let pid = child.id().context("client did not start")? as i32;
            let start = process(pid).ok().flatten().map(|p| p.start);
            state.groups.insert(pid, start);
        }
        Ok(Self {
            input: child.stdin.take(),
            lines: BufReader::new(child.stdout.take().context("client did not start")?).lines(),
            child,
        })
    }
}

struct Login {
    group: LoginGroup,
    request_state: String,
    outcome: Arc<Mutex<Option<Result<(), String>>>>,
}

impl Login {
    fn is_running(&self) -> bool {
        self.outcome.lock().unwrap().is_none()
    }
}

#[derive(Clone)]
struct LoginGroup {
    tree: ClientTree,
    pid: i32,
    start: Option<u64>,
    discovery: Arc<Mutex<LoginDiscovery>>,
}

#[derive(Default)]
struct LoginDiscovery {
    seen: BTreeMap<i32, u64>,
    unconfirmed: bool,
}

impl LoginGroup {
    fn discover(&self) {
        let mut discovery = self.discovery.lock().unwrap();
        // Hold the tree's lock from the reading to the publication, so that a whole-tree stop's
        // reading, which holds it too, comes before or after both.
        let mut tree = self.tree.0.lock().unwrap();
        let (Some(start), Ok(table)) = (self.start, processes()) else {
            discovery.unconfirmed = true;
            return;
        };
        let mut group = TreeState::for_group(self.pid, start, std::mem::take(&mut discovery.seen));
        group.find(table);
        discovery.seen = group.seen;
        // A whole-tree stop must also know about a login descendant that left its group.
        tree.seen.extend(&discovery.seen);
    }

    async fn while_tracking<F: Future>(&self, operation: F) -> F::Output {
        self.discover();
        let mut interval = tokio::time::interval_at(Instant::now() + LOGIN_TRACK, LOGIN_TRACK);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        tokio::pin!(operation);
        loop {
            tokio::select! {
                result = &mut operation => return result,
                _ = interval.tick() => self.discover(),
            }
        }
    }

    async fn stop(&self) -> StopReport {
        let seen = self.discovery.lock().unwrap().seen.clone();
        let mut report = self.tree.stop_group(self.pid, self.start, seen).await;
        report.unconfirmed |= self.discovery.lock().unwrap().unconfirmed;
        report
    }
}

fn check_login_port(port: u16) -> Result<()> {
    TcpListener::bind((Ipv4Addr::LOCALHOST, port)).map_err(|_| {
        anyhow::anyhow!(
            "sign-in hook (browser): PND-010: the client cannot bind its login port {port} on this machine's loopback: it is in use or not available"
        )
    })?;
    Ok(())
}

fn sign_in_start_failure(error: anyhow::Error) -> anyhow::Error {
    if error.to_string().contains("PND-010") {
        error
    } else {
        anyhow::anyhow!("{error} (PND-010)")
    }
}

async fn read_login_url(
    stderr: &mut BufReader<ChildStderr>,
    login: &StatedLogin,
) -> Option<String> {
    let mut line = Vec::new();
    while stderr.read_until(b'\n', &mut line).await.ok()? != 0 {
        for piece in line.split(|byte| byte.is_ascii_whitespace()) {
            if let Ok(piece) = std::str::from_utf8(piece)
                && redirect_url_matches(login, piece)
            {
                return Some(piece.to_owned());
            }
        }
        line.clear();
    }
    None
}

async fn finish_login(
    mut child: Child,
    mut stderr: BufReader<ChildStderr>,
    group: LoginGroup,
) -> Result<(), String> {
    let status = group
        .while_tracking(async {
            let exit = timeout(LOGIN_LIMIT, child.wait());
            tokio::pin!(exit);
            let mut discard = tokio::io::sink();
            tokio::select! {
                status = &mut exit => status,
                _ = tokio::io::copy(&mut stderr, &mut discard) => exit.await,
            }
        })
        .await;
    let stop = group.stop().await;
    let _ = child.kill().await;
    let _ = child.wait().await;
    let note = stop.note();
    let cause = match status {
        Ok(Ok(status)) if status.success() && note.is_none() => return Ok(()),
        Ok(Ok(status)) => exit_cause(status),
        Ok(Err(_)) => "client exit: status unknown".into(),
        Err(_) => format!("no exit within {} seconds", LOGIN_LIMIT.as_secs()),
    };
    Err(match note {
        Some(note) => append_stop_note(Some(cause), &note),
        None => cause,
    })
}

pub struct Session {
    config: Config,
    tree: ClientTree,
    stream: Option<Stream>,
    thread: Option<String>,
    failed: Option<(usize, String)>,
    retired: bool,
    turns_started: usize,
    login: Option<Login>,
}
impl Session {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tree: ClientTree::default(),
            stream: None,
            thread: None,
            failed: None,
            retired: false,
            turns_started: 0,
            login: None,
        }
    }
    pub fn tree(&self) -> ClientTree {
        self.tree.clone()
    }
    pub async fn turn(&mut self, message: &str) -> Result<Value> {
        if let Some(error) = &self.config.configuration_error {
            bail!("{error}");
        }
        if self.retired {
            bail!("{RETIRED_REFUSAL}");
        }
        if let Some((k, cause)) = &self.failed {
            bail!("session failed at turn {k}: {cause}; implicit restart refused");
        }
        let duration = self.config.timeout;
        let result = match timeout(duration, self.run_turn(message)).await {
            Ok(result) => result,
            Err(_) => Err(anyhow::anyhow!(
                "deadline: no turn boundary within {} seconds{}",
                duration.as_secs(),
                if self.config.v2 {
                    " (input turnDeadlineSeconds)"
                } else {
                    ""
                }
            )),
        };
        if let Err(error) = result {
            let cause = error.to_string();
            self.failed = Some((self.turns_started, cause.clone()));
            return Err(self.end_failed_turn(&cause).await);
        }
        result
    }
    async fn run_turn(&mut self, message: &str) -> Result<Value> {
        self.turns_started += 1;
        if self.stream.is_none() {
            self.stream = Some(Stream::spawn(
                &self.config,
                &arguments(&self.config, self.thread.as_deref()),
                &self.tree,
            )?);
        }
        let stream = self.stream.as_mut().unwrap();
        let send = async {
            if CLAUDE {
                let input = stream.input.as_mut().context("missing stdin")?;
                input.write_all(format!("{}\n",json!({"type":"user","message":{"role":"user","content":message},"parent_tool_use_id":null,"session_id":""})).as_bytes()).await?;
                input.flush().await?;
            } else {
                let mut input = stream.input.take().context("missing stdin")?;
                input.write_all(message.as_bytes()).await?;
                input.shutdown().await?;
            }
            Ok::<_, anyhow::Error>(())
        }.await;
        if send.is_err() {
            bail!("{}", stream.boundary_failure(None).await);
        }
        let mut text = String::new();
        let mut tools = json!({"status":"not_reported","observations":[]});
        let mut complete = false;
        let mut reported_thread = false;
        let mut at_start = true;
        let mut index = 0;
        let mut latest_error = None;
        loop {
            let line = match stream.lines.next_line().await {
                Ok(Some(line)) => line,
                Ok(None) if complete => break,
                _ => bail!("{}", stream.boundary_failure(latest_error).await),
            };
            index += 1;
            // Never persist raw client streams: they may contain authentication material.
            let event: Value = serde_json::from_str(&line)
                .map_err(|_| anyhow::anyhow!("invalid client output"))?;
            if event["type"] == "thread.started" {
                let id = event["thread_id"]
                    .as_str()
                    .context("invalid client output")?;
                ensure!(
                    self.thread.as_deref().is_none_or(|old| old == id),
                    "resume changed thread"
                );
                self.thread = Some(id.into());
                reported_thread = true;
            }
            if at_start
                && ((event["type"] == "system" && event["subtype"] == "init")
                    || matches!(
                        event["type"].as_str(),
                        Some("thread.started" | "session.started" | "startup")
                    ))
            {
                if let Some(names) = event["tools"].as_array() {
                    let names: Vec<_> = names
                        .iter()
                        .filter_map(Value::as_str)
                        .filter(|s| s.starts_with("mcp__engine__"))
                        .collect();
                    // Claude repeats session initialization on later turns, even after
                    // a catalog change (experiment 009). That is historical evidence.
                    let current = self.turns_started == 1;
                    tools = json!({"status":if current {"reported"} else {"unavailable"},"observations":[{"event":index,"pointer":"/tools","names":names,"reportedNames":event["tools"],"freshness":if current {"current"} else {"session_init_unverified"}}]});
                }
            } else {
                at_start = false;
            }
            if event["type"] == "error" {
                latest_error = Some(classify(event["message"].as_str()));
            }
            if !CLAUDE && event["type"] == "turn.failed" {
                bail!(
                    "client failure: {}",
                    classify(event["error"]["message"].as_str())
                );
            }
            if CLAUDE && event["type"] == "result" && event["is_error"] == true {
                bail!("client failure: {}", classify(event["result"].as_str()));
            }
            if event["type"] == "item.completed" && event["item"]["type"] == "agent_message" {
                text = event["item"]["text"].as_str().unwrap_or_default().into();
            }
            if CLAUDE && event["type"] == "result" {
                text = event["result"].as_str().unwrap_or_default().into();
                break;
            }
            if !CLAUDE && event["type"] == "turn.completed" {
                complete = true;
            }
        }
        if !CLAUDE {
            let status = stream.child.wait().await.map_err(|_| {
                anyhow::anyhow!(latest_error.map_or_else(
                    || "client exit: status unknown".to_owned(),
                    |class| format!("client failure: {class}")
                ))
            })?;
            ensure!(status.success(), "{}", exit_cause(status));
            ensure!(reported_thread, "no thread");
            self.stream = None;
        }
        Ok(json!({"text":text,"clientTools":tools}))
    }
    pub async fn cancel(&mut self) -> anyhow::Error {
        self.failed = Some((self.turns_started, "cancelled".into()));
        self.end_failed_turn("cancelled").await
    }
    async fn end_failed_turn(&mut self, cause: &str) -> anyhow::Error {
        let client_started = self.stream.is_some();
        let stop = self.shutdown().await;
        anyhow::Error::new(FailedTurn {
            turn: self.turns_started,
            cause: cause.into(),
            client_started,
            stop,
            v2: self.config.v2,
        })
    }
    pub async fn retire(&mut self) -> Result<String> {
        self.retire_within(LOGOUT_DEADLINE).await
    }
    /// The retire hook, with the logout's deadline given; `retire` gives LOGOUT_DEADLINE.
    pub async fn retire_within(&mut self, deadline: Duration) -> Result<String> {
        ensure!(self.config.v2, "retire hook: a .v1 Agent has none");
        if self.config.state_dir.is_none()
            && let Some(error) = &self.config.configuration_error
        {
            bail!("{error}");
        }
        self.retired = true;
        if let Some(note) = self.shutdown().await.note() {
            bail!("{note}");
        }
        let home = client_home(&self.config)?;
        logout(&self.config, &home, &self.tree, deadline).await?;
        ensure!(
            client_home(&self.config)? == home,
            "client home ({HOME_VARIABLE}): cannot empty the home"
        );
        empty_home(&home)?;
        Ok(RETIRED.into())
    }
    /// The checks the account and sign-out hooks make first, as the retire hook does.
    fn hook_ready(&self, hook: &str) -> Result<()> {
        ensure!(self.config.v2, "{hook}: a .v1 Agent has none");
        if self.config.state_dir.is_none()
            && let Some(error) = &self.config.configuration_error
        {
            bail!("{error}");
        }
        ensure!(!self.retired, "{RETIRED_REFUSAL}");
        Ok(())
    }
    pub async fn account(&mut self) -> Result<AccountReport> {
        self.account_within(ACCOUNT_DEADLINE).await
    }
    /// The account hook, with its deadline given; `account` gives ACCOUNT_DEADLINE.
    pub async fn account_within(&mut self, deadline: Duration) -> Result<AccountReport> {
        self.hook_ready("account hook")?;
        let home = client_home(&self.config)?;
        read_account(&self.config, &home, &self.tree, deadline).await
    }
    pub async fn sign_out(&mut self) -> Result<String> {
        self.sign_out_within(LOGOUT_DEADLINE).await
    }
    /// The sign-out hook, with the logout's deadline given; `sign_out` gives LOGOUT_DEADLINE.
    pub async fn sign_out_within(&mut self, deadline: Duration) -> Result<String> {
        self.hook_ready("sign-out hook")?;
        if let Some(note) = self.shutdown().await.note() {
            bail!("{note}");
        }
        let home = client_home(&self.config)?;
        logout(&self.config, &home, &self.tree, deadline).await?;
        Ok(SIGNED_OUT.into())
    }
    pub async fn sign_in(&mut self, call: SignInCall) -> Result<SignIn> {
        self.sign_in_within(call, LOGIN_URL_DEADLINE).await
    }

    pub async fn sign_in_within(&mut self, call: SignInCall, deadline: Duration) -> Result<SignIn> {
        let starts = call.request_state.is_none() && !call.returned;
        self.hook_ready("sign-in hook")
            .map_err(|e| if starts { sign_in_start_failure(e) } else { e })?;
        ensure!(!call.returned, "{RETURN_REFUSAL}");
        if let Some(request_state) = call.request_state {
            let login = self
                .login
                .as_ref()
                .filter(|login| login.request_state == request_state)
                .context(STATE_UNKNOWN)?;
            return match &*login.outcome.lock().unwrap() {
                None => Ok(SignIn::Text(LOGIN_WAITING.into())),
                Some(Ok(())) => Ok(SignIn::Text(LOGIN_ENDED.into())),
                Some(Err(cause)) => bail!("{LOGIN_ENDED}: {cause}"),
            };
        }
        self.start_sign_in(call.login, deadline)
            .await
            .map_err(sign_in_start_failure)
    }

    async fn start_sign_in(&mut self, asked: Option<Value>, deadline: Duration) -> Result<SignIn> {
        if let Some(asked) = asked {
            match asked.get("name").and_then(Value::as_str) {
                Some(BROWSER_LOGIN) => {}
                Some(DEVICE_LOGIN) => bail!("{DEVICE_REFUSAL}"),
                _ => bail!("{LOGIN_UNKNOWN}"),
            }
        }
        let c = &self.config;
        let home = client_home(c)?;
        let unread = "sign-in hook (start): the stated logins cannot be read";
        let login = stated_logins()
            .into_iter()
            .find(|login| login.name == BROWSER_LOGIN)
            .context(unread)?;
        let port = *login.ports.first().context(unread)?;
        if self.login.as_ref().is_some_and(Login::is_running) {
            let earlier = self.login.take().unwrap();
            if let Some(note) = earlier.group.stop().await.note() {
                bail!("sign-in hook (stop): {note}");
            }
        }
        let start_error = "sign-in hook (start): client did not start";
        ensure!(
            c.client_directory().is_dir(),
            "{start_error}: the client directory is gone"
        );
        check_login_port(port)?;
        let (mut child, mut stderr, pid, start) = {
            let mut state = self.tree.0.lock().unwrap();
            ensure!(!state.closed, "{start_error}");
            let mut child = client_command(c, Some(&home), &login_arguments())
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|_| anyhow::anyhow!(start_error))?;
            let pid = child.id().context(start_error)? as i32;
            let stderr = BufReader::new(child.stderr.take().context(start_error)?);
            let start = process(pid).ok().flatten().map(|p| p.start);
            state.groups.insert(pid, start);
            (child, stderr, pid, start)
        };

        let group = LoginGroup {
            tree: self.tree.clone(),
            pid,
            start,
            discovery: Arc::default(),
        };
        let cause = match group
            .while_tracking(timeout(deadline, read_login_url(&mut stderr, &login)))
            .await
        {
            Ok(Some(url)) => {
                let request_state = format!("agent-codex-login-{pid}-{}", start.unwrap_or(0));
                let outcome = Arc::new(Mutex::new(None));
                self.login = Some(Login {
                    group: group.clone(),
                    request_state: request_state.clone(),
                    outcome: outcome.clone(),
                });
                tokio::spawn(async move {
                    let finished = finish_login(child, stderr, group).await;
                    *outcome.lock().unwrap() = Some(finished);
                });
                return Ok(SignIn::Url { url, request_state });
            }
            Ok(None) => match group.while_tracking(timeout(EXIT_WAIT, child.wait())).await {
                Ok(Ok(status)) => format!("no login URL: {}", exit_cause(status)),
                Ok(Err(_)) => "no login URL: client exit: status unknown".into(),
                Err(_) => "no login URL: EOF before URL".into(),
            },
            Err(_) => format!("no login URL within {} seconds", deadline.as_secs()),
        };
        let stop = group.stop().await;
        let _ = child.kill().await;
        let _ = child.wait().await;
        check_login_port(port)?;
        let text = format!("sign-in hook (browser): {cause}");
        bail!(match stop.note() {
            Some(note) => append_stop_note(Some(text), &note),
            None => text,
        });
    }

    pub async fn shutdown(&mut self) -> StopReport {
        if let Some(s) = &mut self.stream {
            s.input.take();
        }
        let stop = if self.config.v2 {
            self.tree.stop(false).await
        } else {
            StopReport::default()
        };
        if let Some(s) = &mut self.stream {
            let _ = s.child.kill().await;
            let _ = s.child.wait().await;
        }
        self.stream = None;
        stop
    }
}

#[cfg(test)]
mod tests {
    use super::{FailedTurn, TreeReading, parse_stat};
    use serde_json::json;

    #[test]
    fn unconfirmed_stop_report_sorts_survivors() {
        let stop = TreeReading {
            pids: vec![43, 41, 42],
            unconfirmed: true,
        }
        .into_report();
        assert_eq!(
            stop.note().as_deref(),
            Some(
                "client tree: PIDs 41, 42, 43 outlived the stop; client tree: the process table could not be read, so the stop is unconfirmed"
            )
        );
        let failure = FailedTurn {
            turn: 1,
            cause: "cancelled".into(),
            client_started: true,
            stop,
            v2: true,
        };
        let fields = failure.fields().unwrap();
        assert_eq!(fields["failedTurn"]["survivors"], json!([41, 42, 43]));
        assert_eq!(fields["failedTurn"]["stopConfirmed"], false);
    }

    #[test]
    fn stat_reads_tree_fields_after_the_last_parenthesis() {
        let live = parse_stat(
            42,
            "42 (name) with a space) R 7 33 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 987654321 23",
        )
        .unwrap();
        assert_eq!((live.pid, live.parent, live.group), (42, 7, 33));
        assert_eq!(live.start, 987654321);
        assert!(!live.zombie);
        for state in ["Z", "X"] {
            let line = format!(
                "43 (exited) {state} 7 33 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 987654322 23"
            );
            let exited = parse_stat(43, &line).unwrap();
            assert_eq!(exited.start, 987654322);
            assert!(exited.zombie);
        }
    }
}
