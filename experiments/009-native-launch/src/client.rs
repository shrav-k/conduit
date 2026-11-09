use crate::{Client, clean_command, user_message, write_json};
use anyhow::{Context, Result, bail, ensure};
use real_client_harness::runner::quote;
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

#[derive(Clone)]
pub struct Config {
    pub client: Client,
    pub program: PathBuf,
    pub helper: PathBuf,
    pub evidence: PathBuf,
    pub cwd: PathBuf,
    pub env: BTreeMap<String, String>,
    pub url: String,
    pub timeout: Duration,
}

pub fn settings() -> Value {
    json!({"permissions":{"allow":["mcp__stub__*"],"deny":["SendMessage","ListAgents","RemoteTrigger"],"defaultMode":"dontAsk"},"disableAllHooks":true,"enabledPlugins":{},"disableClaudeAiConnectors":true,"crossSessionInbound":"refuse"})
}

pub fn helper_command(exe: &Path) -> String {
    format!("{} --headers-helper", quote(&exe.display().to_string()))
}

/// Codex's helper gets no variable of the run's, so the directory goes on its command line.
pub fn helper_command_with_dir(exe: &Path, dir: &str) -> String {
    format!("{} --identity-dir {}", helper_command(exe), quote(dir))
}

pub fn arguments(config: &Config, session: Option<&str>, prompt: &str) -> Vec<String> {
    let mut args: Vec<String> = match config.client {
        Client::Claude => [
            "--print",
            "--model",
            "sonnet",
            "--setting-sources",
            "project",
            "--settings",
        ]
        .map(str::to_owned)
        .into(),
        Client::Codex => ["exec", "--sandbox", "read-only", "--color", "never"]
            .map(str::to_owned)
            .into(),
    };
    match config.client {
        Client::Claude => {
            args.push(config.evidence.join("settings.json").display().to_string());
            args.push("--strict-mcp-config".into());
            args.push("--mcp-config".into());
            args.push(config.evidence.join("mcp.json").display().to_string());
            args.extend(
                [
                    "--tools",
                    "",
                    "--allowedTools",
                    "mcp__stub__*",
                    "--disallowedTools",
                    "SendMessage,ListAgents,RemoteTrigger",
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
        }
        Client::Codex => {
            if session.is_some() {
                args.push("resume".into());
            }
            args.extend(
                [
                    "--ignore-user-config",
                    "--strict-config",
                    "--ignore-rules",
                    "--disable",
                    "apps",
                    "--skip-git-repo-check",
                    "--json",
                ]
                .map(str::to_owned),
            );
            for setting in [
                format!("mcp_servers.stub.url={}", json!(config.url)),
                format!(
                    "mcp_servers.stub.http_headers_helper={}",
                    json!(helper_command_with_dir(
                        &config.helper,
                        config
                            .env
                            .get("CONDUIT_IDENTITY_DIR")
                            .map(String::as_str)
                            .unwrap_or_default()
                    ))
                ),
                "mcp_servers.stub.startup_readiness=\"connection\"".into(),
                "mcp_servers.stub.required=true".into(),
                "mcp_servers.stub.default_tools_approval_mode=\"approve\"".into(),
                "model_reasoning_effort=\"low\"".into(),
                "web_search=\"disabled\"".into(),
            ] {
                args.extend(["-c".into(), setting]);
            }
            if let Some(session) = session {
                args.push(session.into());
            }
            args.push(prompt.into());
        }
    }
    args
}

pub fn prepare(config: &Config) -> Result<()> {
    write_json(&config.evidence.join("settings.json"), &settings())?;
    write_json(
        &config.evidence.join("mcp.json"),
        &json!({"mcpServers":{"stub":{"type":"http","url":config.url,"headersHelper":helper_command(&config.helper)}}}),
    )?;
    Ok(())
}

struct Stream {
    child: Child,
    input: Option<ChildStdin>,
    lines: Lines<BufReader<ChildStdout>>,
}
impl Stream {
    fn spawn(config: &Config, args: &[String], turn: usize) -> Result<Self> {
        write_json(
            &config.evidence.join(format!("turn-{turn}-plan.json")),
            &json!({"program":config.program,"args":args,"environment":config.env,"cwd":config.cwd,"api_keys_removed":["ANTHROPIC_API_KEY","OPENAI_API_KEY"]}),
        )?;
        let stderr = fs::File::create(config.evidence.join(format!("process-{turn}-stderr.txt")))?;
        let mut child = clean_command(&config.program, &config.env, &config.cwd)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(stderr)
            .spawn()
            .context("start client")?;
        let input = child.stdin.take();
        let lines = BufReader::new(child.stdout.take().context("client stdout")?).lines();
        Ok(Self {
            child,
            input,
            lines,
        })
    }
    async fn stop(&mut self) {
        self.input.take();
        let _ = self.child.kill().await;
        let _ = self.child.wait().await;
    }
}

pub struct Session {
    config: Config,
    stream: Option<Stream>,
    thread: Option<String>,
    turn: usize,
    failed: bool,
}
impl Session {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            stream: None,
            thread: None,
            turn: 0,
            failed: false,
        }
    }
    pub async fn turn(&mut self, prompt: &str) -> Result<Value> {
        ensure!(
            !self.failed,
            "session has failed; refusing an implicit restart"
        );
        self.turn += 1;
        let config = self.config.clone();
        fs::write(
            config
                .evidence
                .join(format!("turn-{}-prompt.txt", self.turn)),
            prompt,
        )?;
        fs::write(
            config
                .evidence
                .join(format!("turn-{}-events.jsonl", self.turn)),
            "",
        )?;
        fs::write(
            config
                .evidence
                .join(format!("turn-{}-final.txt", self.turn)),
            "",
        )?;
        write_json(
            &config
                .evidence
                .join(format!("turn-{}-start-report.json", self.turn)),
            &start_report(&[]),
        )?;
        let result = timeout(config.timeout, self.run_turn(prompt)).await;
        let result = match result {
            Ok(result) => result,
            Err(_) => Err(anyhow::anyhow!("client turn timed out")),
        };
        if result.is_err() {
            self.failed = true;
            self.shutdown().await;
        }
        write_json(
            &config
                .evidence
                .join(format!("turn-{}-status.json", self.turn)),
            &json!({"completed":result.is_ok(),"error":result.as_ref().err().map(|e| format!("{e:#}"))}),
        )?;
        result
    }
    async fn run_turn(&mut self, prompt: &str) -> Result<Value> {
        let config = &self.config;
        if self.stream.is_none() {
            let args = arguments(config, self.thread.as_deref(), prompt);
            self.stream = Some(Stream::spawn(config, &args, self.turn)?);
        }
        let stream = self.stream.as_mut().unwrap();
        if config.client == Client::Claude {
            let input = user_message(prompt);
            fs::write(
                config
                    .evidence
                    .join(format!("turn-{}-input.jsonl", self.turn)),
                format!("{input}\n"),
            )?;
            stream
                .input
                .as_mut()
                .context("client input")?
                .write_all(format!("{input}\n").as_bytes())
                .await?;
            stream.input.as_mut().unwrap().flush().await?;
        } else {
            stream.input.take();
        }
        let mut events = Vec::new();
        let mut text = String::new();
        let mut complete = false;
        let mut reported_thread = false;
        let path = config
            .evidence
            .join(format!("turn-{}-events.jsonl", self.turn));
        let mut raw = fs::File::create(path)?;
        use std::io::Write;
        while let Some(line) = stream.lines.next_line().await? {
            writeln!(raw, "{line}")?;
            raw.flush()?;
            let event: Value = serde_json::from_str(&line)
                .context("client emitted non-JSON event; retained verbatim")?;
            if event["type"] == "thread.started" {
                reported_thread = true;
                let id = event["thread_id"].as_str().context("missing thread ID")?;
                ensure!(
                    self.thread.as_deref().is_none_or(|previous| previous == id),
                    "resume changed thread ID"
                );
                self.thread = Some(id.into());
            }
            if event["type"] == "item.completed"
                && event["item"]["type"] == "agent_message"
                && let Some(value) = event["item"]["text"].as_str()
            {
                text = value.into();
            }
            let boundary = if config.client == Client::Claude {
                event["type"] == "result"
            } else {
                event["type"] == "turn.completed"
            };
            let error = event["is_error"] == true
                || event["type"] == "turn.failed"
                || event["type"] == "error";
            if config.client == Client::Claude && boundary {
                text = event["result"].as_str().unwrap_or("").into();
            }
            events.push(event);
            // Evidence is available even if a later boundary, EOF or process exit fails.
            write_json(
                &config
                    .evidence
                    .join(format!("turn-{}-start-report.json", self.turn)),
                &start_report(&events),
            )?;
            fs::write(
                config
                    .evidence
                    .join(format!("turn-{}-final.txt", self.turn)),
                &text,
            )?;
            if error {
                bail!("client reported an error; see events");
            }
            if boundary {
                complete = true;
                if config.client == Client::Claude {
                    break;
                }
            }
        }
        ensure!(complete, "client EOF before result boundary");
        if config.client == Client::Codex {
            ensure!(
                stream.child.wait().await?.success(),
                "Codex exited unsuccessfully"
            );
            ensure!(
                reported_thread && self.thread.is_some(),
                "Codex did not report its session ID in this process"
            );
            write_json(
                &config.evidence.join("session.json"),
                &json!({"thread_id":self.thread,"source":"thread.started"}),
            )?;
            self.stream = None;
        }
        Ok(json!({"text":text,"events":events,"start_report":start_report(&events)}))
    }
    pub async fn shutdown(&mut self) {
        if let Some(stream) = &mut self.stream {
            stream.stop().await;
        }
        self.stream = None;
    }
}

/// Copy observations with JSON pointers. Missing fields remain explicitly unavailable.
pub fn start_report(events: &[Value]) -> Value {
    fn visit(value: &Value, pointer: String, line: usize, found: &mut BTreeMap<&str, Vec<Value>>) {
        match value {
            Value::Object(map) => {
                for (key, value) in map {
                    let pointer =
                        format!("{pointer}/{}", key.replace('~', "~0").replace('/', "~1"));
                    let category = match key.as_str() {
                        "tools" => Some("tools"),
                        "mcp_servers" | "mcpServers" => Some("mcp_servers"),
                        "authMode" | "auth_mode" | "authenticationMode" | "apiKeySource" => {
                            Some("authentication")
                        }
                        _ => None,
                    };
                    if let Some(category) = category {
                        found
                            .entry(category)
                            .or_default()
                            .push(json!({"line":line,"pointer":pointer,"value":value}));
                    }
                    visit(value, pointer, line, found);
                }
            }
            Value::Array(array) => {
                for (index, value) in array.iter().enumerate() {
                    visit(value, format!("{pointer}/{index}"), line, found);
                }
            }
            _ => {}
        }
    }
    let mut found = BTreeMap::new();
    for (index, event) in events.iter().enumerate() {
        let startup = (event["type"] == "system" && event["subtype"] == "init")
            || matches!(
                event["type"].as_str(),
                Some("thread.started" | "session.started" | "startup" | "auth" | "authentication")
            );
        if startup {
            visit(event, String::new(), index + 1, &mut found);
        }
    }
    let mut result = json!({});
    for category in ["tools", "mcp_servers", "authentication"] {
        let observations = found.remove(category).unwrap_or_default();
        result[category] = json!({"status":if observations.is_empty() {"not_reported"} else {"reported"},"observations":observations});
    }
    result
}
