//! Client adapter. This source is copied into each independently buildable package.
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    process::{Child, ChildStdin, ChildStdout},
    time::timeout,
};

pub const CLAUDE: bool = false;
pub const WRAPPER: &str = if CLAUDE {
    "agent-claude-code"
} else {
    "agent-codex"
};
const BASE: [&str; 6] = ["HOME", "LOGNAME", "PATH", "SHELL", "TERM", "USER"];
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
    pub own_tools: Option<bool>,
    pub directory: Option<PathBuf>,
    pub effort: Option<String>,
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
            own_tools: None,
            directory: None,
            effort: None,
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
            let v2 = std::env::var_os("AGENT_OWN_TOOLS").is_some();
            if v2 {
                c.own_tools = Some(boolean("AGENT_OWN_TOOLS")?);
            }
            c.account_services = boolean("AGENT_ACCOUNT_SERVICES")?;
            c.peer_messaging = if v2 && std::env::var_os("AGENT_PEER_MESSAGING").is_none() {
                false
            } else {
                boolean("AGENT_PEER_MESSAGING")?
            };
            c.model = std::env::var("AGENT_MODEL").ok().filter(|s| !s.is_empty());
            if v2 {
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
                    c.directory = Some(dir);
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
            .filter(|_| self.own_tools.is_some())
            .unwrap_or(&self.cwd)
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
    if c.own_tools == Some(true) {
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
        if c.own_tools.is_some() {
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
            if c.own_tools == Some(true) {
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
    if !CLAUDE && c.own_tools.is_some() {
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
        if c.own_tools == Some(true) {
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
        if c.own_tools.is_some()
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
            if c.own_tools == Some(true)
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
            if c.own_tools == Some(true) && c.account_services {
                "web_search=\"live\""
            } else {
                "web_search=\"disabled\""
            }
            .into(),
            "project_doc_max_bytes=0".into(),
        ] {
            args.extend(["-c".into(), setting]);
        }
        if c.own_tools.is_some() {
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
    fn spawn(c: &Config, args: &[String]) -> Result<Self> {
        let mut cmd = tokio::process::Command::new(&c.program);
        let mut child = cmd
            .args(args)
            .env_clear()
            .envs(&c.env)
            .env_remove("ANTHROPIC_API_KEY")
            .env_remove("OPENAI_API_KEY")
            .current_dir(c.client_directory())
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| anyhow::anyhow!("client did not start"))?;
        Ok(Self {
            input: child.stdin.take(),
            lines: BufReader::new(child.stdout.take().context("client did not start")?).lines(),
            child,
        })
    }
}
pub struct Session {
    config: Config,
    stream: Option<Stream>,
    thread: Option<String>,
    failed: Option<(usize, String)>,
    turns_started: usize,
}
impl Session {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            stream: None,
            thread: None,
            failed: None,
            turns_started: 0,
        }
    }
    pub async fn turn(&mut self, message: &str) -> Result<Value> {
        if let Some(error) = &self.config.configuration_error {
            bail!("{error}");
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
                if self.config.own_tools.is_some() {
                    " (input turnDeadlineSeconds)"
                } else {
                    ""
                }
            )),
        };
        if let Err(error) = result {
            let cause = error.to_string();
            self.failed = Some((self.turns_started, cause.clone()));
            self.shutdown().await;
            bail!(
                "client turn {} failed: {cause}; session cannot restart implicitly",
                self.turns_started
            );
        }
        result
    }
    async fn run_turn(&mut self, message: &str) -> Result<Value> {
        self.turns_started += 1;
        if self.stream.is_none() {
            self.stream = Some(Stream::spawn(
                &self.config,
                &arguments(&self.config, self.thread.as_deref()),
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
        self.shutdown().await;
        anyhow::anyhow!(
            "client turn {} failed: cancelled; session cannot restart implicitly",
            self.turns_started
        )
    }
    pub async fn shutdown(&mut self) {
        if let Some(s) = &mut self.stream {
            s.input.take();
            let _ = s.child.kill().await;
            let _ = s.child.wait().await;
        }
        self.stream = None;
    }
}
