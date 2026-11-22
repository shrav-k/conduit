//! Client adapter. This source is copied into each independently buildable package.
mod limit;
mod usage;

use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::{Path, PathBuf},
    process::Stdio,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    process::{Child, ChildStdin, ChildStdout},
    time::{Instant, sleep_until, timeout},
};

pub use usage::MEMBER as USAGE_MEMBER;

pub const CLAUDE: bool = true;
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
const POLL: Duration = Duration::from_millis(20);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Agent,
    SharedLoginAgent,
    LoginResource,
}

const ACCOUNT_HOME: &str = "account-home";
const AUTO_MEMORY_SWITCH: &str = "CLAUDE_CODE_DISABLE_AUTO_MEMORY";

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
    pub mode: Mode,
    pub login_directory: Option<PathBuf>,
    pub instructions: Option<String>,
    pub output_schema: Option<String>,
    pub configuration_error: Option<String>,
}
impl Config {
    pub fn from_env(mode: Mode) -> Result<Self> {
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
            v2: mode != Mode::Agent,
            directory: None,
            effort: None,
            state_dir: None,
            mode,
            login_directory: if mode == Mode::SharedLoginAgent {
                std::env::var_os("AGENT_LOGIN_DIRECTORY").map(PathBuf::from)
            } else {
                None
            },
            instructions: None,
            output_schema: None,
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
            let v2 = c.v2 || std::env::var_os("AGENT_TURN_DEADLINE_SECONDS").is_some();
            c.v2 = v2;
            if mode != Mode::LoginResource {
                c.account_services = boolean("AGENT_ACCOUNT_SERVICES")?;
                c.peer_messaging = if v2 && std::env::var_os("AGENT_PEER_MESSAGING").is_none() {
                    false
                } else {
                    boolean("AGENT_PEER_MESSAGING")?
                };
                c.model = std::env::var("AGENT_MODEL").ok().filter(|s| !s.is_empty());
                c.instructions = role_input("AGENT_INSTRUCTIONS", "instructions")?;
                c.output_schema = role_input("AGENT_OUTPUT_SCHEMA", "outputSchema")?;
                if let Some(schema) = &c.output_schema {
                    ensure!(
                        serde_json::from_str::<Value>(schema)
                            .is_ok_and(|schema| schema.is_object() && schema["type"] == "object"),
                        "{OUTPUT_SCHEMA_ERROR}"
                    );
                }
            }
            if v2 {
                let label = home_label(mode);
                let state_dir = std::env::var_os("CONDUIT_STATE_DIR")
                    .filter(|dir| !dir.is_empty())
                    .map(PathBuf::from)
                    .with_context(|| format!("{label}: CONDUIT_STATE_DIR is unset"))?;
                ensure!(
                    state_dir.is_absolute(),
                    "{label}: CONDUIT_STATE_DIR is not an absolute path"
                );
                c.state_dir = Some(state_dir);
                if mode == Mode::LoginResource {
                    return Ok(());
                }
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

const OUTPUT_SCHEMA_ERROR: &str =
    "outputSchema (AGENT_OUTPUT_SCHEMA): expected the JSON text of a schema whose type is object";

fn role_input(key: &str, place: &str) -> Result<Option<String>> {
    match std::env::var(key) {
        Ok(value) => Ok(Some(value)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) if key == "AGENT_OUTPUT_SCHEMA" => {
            bail!("{OUTPUT_SCHEMA_ERROR}")
        }
        Err(std::env::VarError::NotUnicode(_)) => bail!("{place} ({key}): invalid encoding"),
    }
}

fn home_label(mode: Mode) -> String {
    if mode != Mode::Agent {
        format!("login home ({HOME_VARIABLE})")
    } else {
        format!("client home ({HOME_VARIABLE})")
    }
}

/// Resolve and check the client home before a .v2 launch.
pub fn client_home(c: &Config) -> Result<PathBuf> {
    use std::io::ErrorKind::NotFound;

    let label = home_label(c.mode);
    let state_dir = c
        .state_dir
        .as_ref()
        .with_context(|| format!("{label}: CONDUIT_STATE_DIR is unset"))?;
    let state_dir = state_dir
        .canonicalize()
        .ok()
        .filter(|dir| dir.is_dir())
        .with_context(|| format!("{label}: CONDUIT_STATE_DIR is not an existing directory"))?;
    let home = state_dir.join(if c.mode == Mode::LoginResource {
        ACCOUNT_HOME
    } else {
        HOME_NAME
    });
    if fs::symlink_metadata(&home).is_err_and(|error| error.kind() == NotFound) {
        #[cfg(unix)]
        let created = {
            use std::os::unix::fs::DirBuilderExt;
            fs::DirBuilder::new().mode(0o700).create(&home)
        };
        #[cfg(not(unix))]
        let created = fs::create_dir(&home);
        created
            .map_err(|_| anyhow::anyhow!("{label}: cannot create the home in CONDUIT_STATE_DIR"))?;
    }
    let home = home
        .canonicalize()
        .map_err(|_| anyhow::anyhow!("{label}: the home is not a directory"))?;
    ensure!(
        home.starts_with(&state_dir) && home != state_dir,
        "{label}: the home resolves outside CONDUIT_STATE_DIR"
    );
    ensure!(home.is_dir(), "{label}: the home is not a directory");
    if !CLAUDE {
        match fs::symlink_metadata(home.join(".env")) {
            Ok(_) => bail!(
                "client home ({HOME_VARIABLE}): the home holds a .env, which Codex would read into its environment; the wrapper refuses it, since the Agent holds no API key (ruling 83)"
            ),
            Err(error) if error.kind() == NotFound => {}
            Err(_) => bail!("client home ({HOME_VARIABLE}): cannot check the home for a .env"),
        }
    }
    if !CLAUDE || c.mode == Mode::LoginResource {
        ensure!(
            home.to_str().is_some(),
            "{label}: the canonical home path is not valid Unicode"
        );
    }
    Ok(home)
}

#[derive(Debug, Default)]
pub struct LoginLapsed {
    pub usage: Option<Value>,
}

impl std::fmt::Display for LoginLapsed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("the login Resource's Claude Code login is missing or has lapsed")
    }
}

impl std::error::Error for LoginLapsed {}

fn shared_login_home(c: &Config) -> Result<PathBuf> {
    use io::ErrorKind::NotFound;

    let label = home_label(c.mode);
    let directory = c
        .login_directory
        .as_deref()
        .with_context(|| format!("{label}: AGENT_LOGIN_DIRECTORY is unset"))?;
    ensure!(
        directory.is_absolute(),
        "{label}: AGENT_LOGIN_DIRECTORY is not an absolute path"
    );
    ensure!(
        directory.to_str().is_some(),
        "{label}: AGENT_LOGIN_DIRECTORY is not valid Unicode"
    );
    let directory = match directory.canonicalize() {
        Ok(directory) if directory.is_dir() => directory,
        Err(error) if error.kind() == NotFound => return Err(LoginLapsed::default().into()),
        _ => bail!("{label}: AGENT_LOGIN_DIRECTORY is not an existing directory"),
    };
    let home = directory.join(ACCOUNT_HOME);
    if fs::symlink_metadata(&home).is_err_and(|error| error.kind() == NotFound) {
        return Err(LoginLapsed::default().into());
    }
    let home = home
        .canonicalize()
        .ok()
        .filter(|home| home.is_dir())
        .with_context(|| format!("{label}: account-home is not an existing directory"))?;
    ensure!(
        home.starts_with(&directory) && home != directory,
        "{label}: account-home resolves outside AGENT_LOGIN_DIRECTORY"
    );
    ensure!(
        home.to_str().is_some(),
        "{label}: the canonical home path is not valid Unicode"
    );
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

fn empty_home(home: &Path, mode: Mode) -> Result<()> {
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
    remove_entries(home).map_err(|_| anyhow::anyhow!("{}: cannot empty the home", home_label(mode)))
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
    if c.v2 {
        settings["autoMemoryEnabled"] = json!(false);
    }
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
        write_run_file(
            &c.cwd.join("settings.json"),
            &serde_json::to_vec(&settings(c))?,
        )?;
        let mut mcp = json!({"mcpServers":{"engine":{"type":"http","url":c.url,"headersHelper":helper_command(c)}}});
        if c.v2 {
            mcp["mcpServers"]["engine"]["timeout"] = json!(c.timeout_millis()?);
        }
        write_run_file(&c.cwd.join("mcp.json"), &serde_json::to_vec(&mcp)?)?;
        if let Some(instructions) = &c.instructions {
            write_run_file(&c.cwd.join("instructions.md"), instructions.as_bytes())?;
        }
    }
    Ok(())
}

fn write_run_file(path: &Path, bytes: &[u8]) -> Result<()> {
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)?;
    // Check the opened file before truncating it. A hard link could also reach the shared home.
    let metadata = file.metadata()?;
    ensure!(
        metadata.is_file() && metadata.nlink() == 1,
        "run config must be a regular file with exactly one link"
    );
    file.set_len(0)?;
    io::Write::write_all(&mut file, bytes)?;
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
        if c.instructions.is_some() {
            args.extend([
                "--append-system-prompt-file".into(),
                c.cwd.join("instructions.md").display().to_string(),
            ]);
        }
        if let Some(schema) = &c.output_schema {
            args.extend(["--json-schema".into(), schema.clone()]);
        }
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
#[derive(Debug)]
struct ClientFailure {
    class: &'static str,
    engine_mcp: bool,
    window: Option<&'static str>,
    resets_at: Option<String>,
    usage: Option<Value>,
}

impl ClientFailure {
    fn with_rejection(mut self, resets_at: Option<&Value>, started: SystemTime) -> Self {
        if let Some(resets_at) = resets_at {
            self.class = "usage-limit";
            self.engine_mcp = false;
            self.resets_at = limit::reset_time(resets_at, started);
        }
        self
    }

    fn limit(&self) -> Option<Value> {
        (self.class == "usage-limit")
            .then(|| json!({"window":self.window,"windowMinutes":null,"resetsAt":self.resets_at}))
    }
}

impl std::fmt::Display for ClientFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "client failure: {}", self.class)
    }
}

impl std::error::Error for ClientFailure {}

fn engine_mcp_failure(text: &str) -> bool {
    let words: Vec<_> = text
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'))
        .filter(|word| !word.is_empty())
        .collect();
    words.windows(3).any(|phrase| {
        matches!(
            phrase,
            ["engine", "mcp", "server"] | ["mcp", "server", "engine"]
        )
    }) || words.windows(4).any(|phrase| {
        matches!(
            phrase,
            ["mcp", "client", "for", "engine"] | ["mcp", "server", "for" | "named", "engine"]
        )
    })
}

fn classify<'a>(messages: impl IntoIterator<Item = &'a str>) -> ClientFailure {
    let messages: Vec<_> = messages.into_iter().map(str::to_ascii_lowercase).collect();
    let window = messages.iter().find_map(|text| limit::window(text));
    let class = if window.is_some() {
        "usage-limit"
    } else {
        ERROR_CLASSES
            .iter()
            .find(|(_, needles)| {
                messages
                    .iter()
                    .any(|text| needles.iter().any(|needle| text.contains(needle)))
            })
            .map_or("other", |(class, _)| *class)
    };
    ClientFailure {
        class,
        engine_mcp: class == "authentication"
            && messages.iter().any(|text| engine_mcp_failure(text)),
        window,
        resets_at: None,
        usage: None,
    }
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
    term_group: BTreeMap<(i32, u64), i32>,
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
            let identity = (p.pid, p.start);
            if signal == libc::SIGTERM && groups_signalled.contains(&p.group) {
                delivery.term_group.insert(identity, p.group);
            }
            // A process can leave a group between sampling it and signalling the group.
            if delivery.term_group.get(&identity) == Some(&p.group) {
                continue;
            }
            delivery.term_group.remove(&identity);
            if !delivery.direct.contains(&identity)
                && process(p.pid).is_ok_and(|entry| {
                    entry.is_some_and(|current| !current.zombie && current.start == p.start)
                })
                && unsafe { libc::kill(p.pid, signal) } == 0
            {
                delivery.direct.insert(identity);
            }
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

    /// The ended conversation's summary, including whether its processes stopped.
    pub fn ended_conversation(&self, turns: usize, failed: bool) -> Value {
        json!({
            "turns": turns,
            "failed": failed,
            "survivors": self.survivors,
            "stopConfirmed": !self.unconfirmed && self.survivors.is_empty(),
        })
    }

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
    pub usage: Option<Value>,
    limit: Option<Value>,
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
            cause if cause.starts_with("login home (") => "login-home",
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
                "limit": self.limit,
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
    usage: usage::Usage,
}
impl Stream {
    async fn boundary_failure(&mut self, failure: Option<ClientFailure>) -> anyhow::Error {
        let status = timeout(EXIT_WAIT, self.child.wait()).await;
        if let Some(failure) = failure {
            return failure.into();
        }
        anyhow::anyhow!(match status {
            Ok(Ok(status)) if !status.success() => exit_cause(status),
            Ok(Err(_)) => "client exit: status unknown".into(),
            _ => "EOF before turn boundary".into(),
        })
    }
    fn spawn(
        c: &Config,
        shared_home: Option<&Path>,
        args: &[String],
        tree: &ClientTree,
    ) -> Result<Self> {
        let mut state = c.v2.then(|| tree.0.lock().unwrap());
        ensure!(
            state.as_ref().is_none_or(|state| !state.closed),
            "client did not start"
        );
        let private_home = (c.v2 && c.mode == Mode::Agent)
            .then(|| client_home(c))
            .transpose()?;
        let mut command = client_command(c, shared_home.or(private_home.as_deref()), args);
        if c.v2 {
            prepare(c)
                .map_err(|_| anyhow::anyhow!("invalid configuration: cannot prepare run files"))?;
            command.env(AUTO_MEMORY_SWITCH, "1");
        }
        let mut child = command
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
            usage: usage::Usage::default(),
        })
    }
}
pub struct Session {
    config: Config,
    tree: ClientTree,
    stream: Option<Stream>,
    thread: Option<String>,
    failed: Option<(usize, String)>,
    retired: bool,
    conversation: usize,
    turns_started: usize,
    login: Option<sign_in::Login>,
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
            conversation: 1,
            turns_started: 0,
            login: None,
        }
    }
    pub fn tree(&self) -> ClientTree {
        self.tree.clone()
    }
    pub async fn new_conversation(&mut self) -> Result<Value> {
        if let Some(error) = &self.config.configuration_error {
            bail!("{error}");
        }
        ensure!(!self.retired, "session retired; no new conversation starts");
        ensure!(
            !self.login.as_ref().is_some_and(sign_in::Login::is_running),
            "new conversation refused: a sign-in is in progress in this Agent's own home; nothing was stopped; call again once the sign-in has ended"
        );

        let stop = self.shutdown().await;
        let ended = stop.ended_conversation(
            self.turns_started,
            self.failed
                .as_ref()
                .is_some_and(|(_, cause)| cause != sign_in::SIGNED_OUT_CAUSE),
        );
        self.thread = None;
        self.failed = None;
        self.turns_started = 0;
        self.conversation += 1;
        Ok(json!({"conversation": self.conversation, "ended": ended}))
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
            if error.is::<LoginLapsed>() {
                return Err(error);
            }
            let cause = error.to_string();
            let failure = error.downcast_ref::<ClientFailure>();
            let lapse = self.config.mode == Mode::SharedLoginAgent
                && self.stream.is_some()
                && failure.is_some_and(|failure| {
                    failure.class == "authentication" && !failure.engine_mcp
                });
            self.failed = Some((self.turns_started, cause.clone()));
            let failed = self.end_failed_turn(&cause, failure).await;
            return Err(if lapse {
                LoginLapsed {
                    usage: failure.and_then(|failure| failure.usage.clone()),
                }
                .into()
            } else {
                failed
            });
        }
        result
    }
    async fn run_turn(&mut self, message: &str) -> Result<Value> {
        let started = SystemTime::now();
        let home = if self.config.mode == Mode::SharedLoginAgent {
            shared_login_home(&self.config).map(Some)
        } else {
            Ok(None)
        };
        if home.as_ref().is_err_and(|error| error.is::<LoginLapsed>()) {
            return Err(LoginLapsed::default().into());
        }
        self.turns_started += 1;
        let home = home?;
        if self.stream.is_none() {
            self.stream = Some(Stream::spawn(
                &self.config,
                home.as_deref(),
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
            return Err(stream.boundary_failure(None).await);
        }
        let mut text = String::new();
        let mut tools = json!({"status":"not_reported","observations":[]});
        let mut complete = false;
        let mut reported_thread = false;
        let mut at_start = true;
        let mut index = 0;
        let mut latest_error: Option<ClientFailure> = None;
        let mut rejected = None;
        loop {
            let line = match stream.lines.next_line().await {
                Ok(Some(line)) => line,
                Ok(None) if complete => break,
                _ => {
                    let failure =
                        latest_error.map(|error| error.with_rejection(rejected.as_ref(), started));
                    return Err(stream.boundary_failure(failure).await);
                }
            };
            index += 1;
            // Never persist raw client streams: they may contain authentication material.
            let event: Value = serde_json::from_str(&line)
                .map_err(|_| anyhow::anyhow!("invalid client output"))?;
            if event["type"] == "system" && event["subtype"] == "init" {
                stream.usage.init(&event);
            }
            if event["type"] == "rate_limit_event"
                && event["rate_limit_info"]["status"] == "rejected"
            {
                rejected = Some(event["rate_limit_info"]["resetsAt"].clone());
            }
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
                return Err(classify(event["error"]["message"].as_str()).into());
            }
            if event["type"] == "item.completed" && event["item"]["type"] == "agent_message" {
                text = event["item"]["text"].as_str().unwrap_or_default().into();
            }
            if CLAUDE && event["type"] == "result" {
                let usage = stream.usage.result(&event);
                let structured_failure = event["subtype"] == "error_max_structured_output_retries";
                if structured_failure || event["is_error"] == true {
                    let errors = event["errors"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_str);
                    let mut failure = classify(event["result"].as_str().into_iter().chain(errors));
                    if structured_failure {
                        failure.class = "structured-output";
                        failure.engine_mcp = false;
                    }
                    failure.usage = usage;
                    return Err(failure.with_rejection(rejected.as_ref(), started).into());
                }
                let mut result = json!({
                    "text":event["result"].as_str().unwrap_or_default(),
                    "clientTools":tools
                });
                if event["structured_output"].is_object() {
                    result["structured"] = event["structured_output"].clone();
                }
                if let Some(usage) = usage {
                    result["_meta"][USAGE_MEMBER] = usage;
                }
                return Ok(result);
            }
            if !CLAUDE && event["type"] == "turn.completed" {
                complete = true;
            }
        }
        if !CLAUDE {
            let status = stream.child.wait().await.map_err(|_| {
                latest_error.map_or_else(
                    || anyhow::anyhow!("client exit: status unknown"),
                    Into::into,
                )
            })?;
            ensure!(status.success(), "{}", exit_cause(status));
            ensure!(reported_thread, "no thread");
            self.stream = None;
        }
        Ok(json!({"text":text,"clientTools":tools}))
    }
    pub async fn cancel(&mut self) -> anyhow::Error {
        self.failed = Some((self.turns_started, "cancelled".into()));
        self.end_failed_turn("cancelled", None).await
    }
    async fn end_failed_turn(
        &mut self,
        cause: &str,
        failure: Option<&ClientFailure>,
    ) -> anyhow::Error {
        let client_started = self.stream.is_some();
        let stop = self.shutdown().await;
        anyhow::Error::new(FailedTurn {
            turn: self.turns_started,
            cause: cause.into(),
            client_started,
            stop,
            usage: failure.and_then(|failure| failure.usage.clone()),
            limit: failure.and_then(ClientFailure::limit),
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
        if self.config.mode == Mode::SharedLoginAgent {
            return Ok("client stopped; its login and configuration directory were left to its login Resource".into());
        }
        let home = client_home(&self.config)?;
        logout(&self.config, &home, &self.tree, deadline).await?;
        ensure!(
            client_home(&self.config)? == home,
            "{}: cannot empty the home",
            home_label(self.config.mode)
        );
        empty_home(&home, self.config.mode)?;
        Ok(RETIRED.into())
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

pub mod sign_in;
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
            usage: None,
            limit: None,
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
