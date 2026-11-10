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
}
impl Config {
    pub fn from_env() -> Result<Self> {
        let boolean = |key| -> Result<bool> {
            match std::env::var(key)?.as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => bail!("invalid boolean input"),
            }
        };
        let url = std::env::var("CONDUIT_URL")?;
        let identity = PathBuf::from(std::env::var("CONDUIT_IDENTITY_DIR")?);
        let mut env: BTreeMap<_, _> = BASE
            .into_iter()
            .filter_map(|k| {
                std::env::var(k)
                    .ok()
                    .filter(|v| !v.starts_with("()"))
                    .map(|v| (k.to_owned(), v))
            })
            .collect();
        env.insert("CONDUIT_URL".into(), url.clone());
        env.insert(
            "CONDUIT_IDENTITY_DIR".into(),
            identity.display().to_string(),
        );
        Ok(Self {
            program: if CLAUDE { "claude" } else { "codex" }.into(),
            helper: std::env::current_exe()?,
            cwd: std::env::current_dir()?,
            env,
            url,
            identity,
            account_services: boolean("AGENT_ACCOUNT_SERVICES")?,
            peer_messaging: boolean("AGENT_PEER_MESSAGING")?,
            model: std::env::var("AGENT_MODEL").ok().filter(|s| !s.is_empty()),
            timeout: Duration::from_secs(180),
        })
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
        fs::write(
            c.cwd.join("mcp.json"),
            serde_json::to_vec(
                &json!({"mcpServers":{"engine":{"type":"http","url":c.url,"headersHelper":helper_command(c)}}}),
            )?,
        )?;
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
        ["exec", "--sandbox", "read-only", "--color", "never"]
            .map(str::to_owned)
            .into()
    };
    if let Some(model) = &c.model {
        args.extend(["--model".into(), model.clone()]);
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
            "hooks",
            "plugins",
            "view_image",
            "goals",
            "image_generation",
        ] {
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
            "web_search=\"disabled\"".into(),
            "project_doc_max_bytes=0".into(),
        ] {
            args.extend(["-c".into(), setting]);
        }
        if let Some(thread) = thread {
            args.push(thread.into());
        }
        args.push("-".into());
    }
    args
}
struct Stream {
    child: Child,
    input: Option<ChildStdin>,
    lines: Lines<BufReader<ChildStdout>>,
}
impl Stream {
    fn spawn(c: &Config, args: &[String]) -> Result<Self> {
        let mut cmd = tokio::process::Command::new(&c.program);
        let mut child = cmd
            .args(args)
            .env_clear()
            .envs(&c.env)
            .env_remove("ANTHROPIC_API_KEY")
            .env_remove("OPENAI_API_KEY")
            .current_dir(&c.cwd)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("cannot start client")?;
        Ok(Self {
            input: child.stdin.take(),
            lines: BufReader::new(child.stdout.take().context("missing stdout")?).lines(),
            child,
        })
    }
}
pub struct Session {
    config: Config,
    stream: Option<Stream>,
    thread: Option<String>,
    failed: bool,
    turns_started: usize,
}
impl Session {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            stream: None,
            thread: None,
            failed: false,
            turns_started: 0,
        }
    }
    pub async fn turn(&mut self, message: &str) -> Result<Value> {
        ensure!(!self.failed, "session failed; implicit restart refused");
        let result = timeout(self.config.timeout, self.run_turn(message))
            .await
            .unwrap_or_else(|_| Err(anyhow::anyhow!("client deadline exceeded")));
        if result.is_err() {
            self.failed = true;
            self.shutdown().await;
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
        if CLAUDE {
            let input = stream.input.as_mut().context("missing client stdin")?;
            input.write_all(format!("{}\n",json!({"type":"user","message":{"role":"user","content":message},"parent_tool_use_id":null,"session_id":""})).as_bytes()).await?;
            input.flush().await?;
        } else {
            let mut input = stream.input.take().context("missing client stdin")?;
            input.write_all(message.as_bytes()).await?;
            input.shutdown().await?;
        }
        let mut text = String::new();
        let mut tools = json!({"status":"not_reported","observations":[]});
        let mut complete = false;
        let mut reported_thread = false;
        let mut at_start = true;
        let mut index = 0;
        while let Some(line) = stream.lines.next_line().await? {
            index += 1;
            // Never persist raw client streams: they may contain authentication material.
            let event: Value =
                serde_json::from_str(&line).map_err(|_| anyhow::anyhow!("invalid client JSON"))?;
            if event["type"] == "thread.started" {
                let id = event["thread_id"].as_str().context("missing thread ID")?;
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
            if event["is_error"] == true
                || matches!(event["type"].as_str(), Some("error" | "turn.failed"))
            {
                bail!("client reported failure");
            }
            if event["type"] == "item.completed" && event["item"]["type"] == "agent_message" {
                text = event["item"]["text"].as_str().unwrap_or_default().into();
            }
            if CLAUDE && event["type"] == "result" {
                text = event["result"].as_str().unwrap_or_default().into();
                complete = true;
                break;
            }
            if !CLAUDE && event["type"] == "turn.completed" {
                complete = true;
            }
        }
        ensure!(complete, "client EOF before turn boundary");
        if !CLAUDE {
            ensure!(
                stream.child.wait().await?.success(),
                "client exited unsuccessfully"
            );
            ensure!(reported_thread, "client did not report thread");
            self.stream = None;
        }
        Ok(json!({"text":text,"clientTools":tools}))
    }
    pub async fn cancel(&mut self) {
        self.failed = true;
        self.shutdown().await;
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
