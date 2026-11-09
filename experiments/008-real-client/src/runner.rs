use crate::{
    Names,
    stub::{self, Case},
};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
};

#[derive(Clone, Copy, Debug, Serialize)]
pub enum Client {
    Claude,
    Codex,
}
impl Client {
    pub fn name(self) -> &'static str {
        match self {
            Self::Claude => "claude",
            Self::Codex => "codex",
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct Cell {
    pub client: Client,
    pub case: Case,
    pub cache: bool,
    pub ttl: u64,
    pub first_401: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub later: bool,
}
impl Cell {
    pub fn repetitions(&self) -> usize {
        if self.first_401 { 1 } else { 3 }
    }
    pub fn id(&self) -> String {
        format!(
            "{}-{}{}-cache-{}-ttl-{}-auth-{}",
            self.client.name(),
            self.case.name(),
            if self.later { "-later" } else { "" },
            if self.cache { "on" } else { "off" },
            self.ttl,
            if self.first_401 { "401" } else { "valid" }
        )
    }
    pub fn prompt(&self) -> String {
        if self.later {
            return self.turn_prompt(1);
        }
        let task = match self.case {
            Case::C1 => {
                "Call files.read.v1 with path hello.txt, then control.next.v1 with {}, then files.read.v1 with path hello.txt again, in that order. Attempt the final call even if the tool list changed."
            }
            Case::C2 => {
                "Call never.presented.v1 with {}. This tool has never appeared in the server's list. Attempt the call by that exact name if your client permits it."
            }
            Case::C3 => "Call files.read.v1 with path fail.txt.",
            Case::C4 => "Call files.write.v1 with path hello.txt and text replacement.",
            Case::C5 => {
                "Call control.next.v1 with {}, then use the newly opened tool notes.read.v1 with path hello.txt. Discover the new tool if your client permits it."
            }
        };
        format!(
            "Use only the experiment stub's MCP tools. {task} Report what happened. Do not substitute shell commands or other tools for the requested calls."
        )
    }
    pub fn turn_prompt(&self, turn: usize) -> String {
        assert!(self.later && matches!(turn, 1 | 2));
        let task = match (self.case, turn) {
            (Case::C1, 1) => {
                "Call files.read.v1 with path hello.txt, then control.next.v1 with {}, in that order. Stop after these two calls."
            }
            (Case::C1, 2) => "Read hello.txt again using files.read.v1 if it is available.",
            (Case::C5, 1) => "Call control.next.v1 with {}. Stop after that call.",
            (Case::C5, 2) => {
                "Call notes.read.v1 with path hello.txt. Discover the new tool if your client permits it."
            }
            _ => unreachable!("only C1 and C5 have later-turn cases"),
        };
        format!(
            "Use only the experiment stub's MCP tools. {task} Report what happened. Do not substitute shell commands or other tools for the requested calls."
        )
    }
}
pub fn matrix() -> Vec<Cell> {
    let mut cells = Vec::new();
    for client in [Client::Claude, Client::Codex] {
        for case in [Case::C1, Case::C2, Case::C3, Case::C4, Case::C5] {
            let changes = matches!(case, Case::C1 | Case::C5);
            for &cache in if changes {
                &[false, true][..]
            } else {
                &[false][..]
            } {
                for &ttl in if changes { &[0, 60000][..] } else { &[0][..] } {
                    cells.push(Cell {
                        client,
                        case,
                        cache,
                        ttl,
                        first_401: false,
                        later: false,
                    });
                }
            }
        }
        cells.push(Cell {
            client,
            case: Case::C3,
            cache: false,
            ttl: 0,
            first_401: true,
            later: false,
        });
    }
    cells
}

pub fn follow_up_matrix() -> Vec<Cell> {
    [Client::Claude, Client::Codex]
        .into_iter()
        .flat_map(|client| {
            [Case::C1, Case::C5].map(|case| Cell {
                client,
                case,
                cache: false,
                ttl: 0,
                first_401: false,
                later: true,
            })
        })
        .collect()
}

#[derive(Serialize)]
pub struct Plan {
    pub program: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub env_remove: Vec<String>,
    pub cwd: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_message: Option<Value>,
}
impl Plan {
    pub fn command(&self) -> Command {
        let mut command = Command::new(&self.program);
        command
            .args(&self.args)
            .envs(&self.env)
            .current_dir(&self.cwd)
            .stdin(if self.stdin_message.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            });
        for key in &self.env_remove {
            command.env_remove(key);
        }
        command
    }
    pub fn display(&self) -> String {
        format!(
            "cd {} && env {} {} {} {} {}",
            quote(&self.cwd.display().to_string()),
            self.env_remove
                .iter()
                .map(|k| format!("-u {}", quote(k)))
                .collect::<Vec<_>>()
                .join(" "),
            self.env
                .iter()
                .map(|(k, v)| format!("{k}={}", quote(v)))
                .collect::<Vec<_>>()
                .join(" "),
            quote(&self.program),
            self.args
                .iter()
                .map(|a| quote(a))
                .collect::<Vec<_>>()
                .join(" "),
            if self.stdin_message.is_some() {
                "< <streamed-input>"
            } else {
                "< /dev/null"
            }
        )
    }
}
pub fn quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
fn helper_command(exe: &Path, run: &Path) -> String {
    [
        exe.to_string_lossy().to_string(),
        "--headers-helper".into(),
        "--identity-dir".into(),
        run.join("identity").display().to_string(),
        "--helper-log".into(),
        run.join("helper.jsonl").display().to_string(),
    ]
    .iter()
    .map(|s| quote(s))
    .collect::<Vec<_>>()
    .join(" ")
}
/// Codex's helper is a command string (run by its shell), not an argv array.
pub fn plan(cell: &Cell, run: &Path, exe: &Path, url: &str) -> Plan {
    let mut args: Vec<String> = Vec::new();
    let mut env = BTreeMap::new();
    let names = Names::default();
    env.insert(names.get("engineUrlVariable").into(), url.into());
    env.insert(
        names.get("identityDirVariable").into(),
        run.join("identity").display().to_string(),
    );
    env.insert("TMPDIR".into(), run.join("tmp").display().to_string());
    env.insert(
        "npm_config_cache".into(),
        run.join("npm-cache").display().to_string(),
    );
    match cell.client {
        Client::Claude => {
            env.insert(
                "MCP_DISCOVERY_CACHE".into(),
                if cell.cache { "1" } else { "0" }.into(),
            );
            args.extend(
                [
                    "--print",
                    "--model",
                    "sonnet",
                    "--setting-sources",
                    "project",
                    "--settings",
                ]
                .map(str::to_string),
            );
            args.push(run.join("settings.json").display().to_string());
            args.extend(["--strict-mcp-config", "--mcp-config"].map(str::to_string));
            args.push(run.join("mcp.json").display().to_string());
            args.extend(
                [
                    "--tools",
                    "",
                    "--allowedTools",
                    "mcp__stub__*",
                    "--no-session-persistence",
                    "--output-format",
                    "stream-json",
                    "--verbose",
                    "--include-partial-messages",
                    "--disable-slash-commands",
                ]
                .map(str::to_string),
            );
            args.push(cell.prompt());
        }
        Client::Codex => {
            args.extend(
                [
                    "exec",
                    "--ignore-user-config",
                    "--strict-config",
                    "--ignore-rules",
                    "--disable",
                    "apps",
                    "--sandbox",
                    "read-only",
                    "--ephemeral",
                    "--skip-git-repo-check",
                    "--json",
                    "--color",
                    "never",
                    "--output-last-message",
                ]
                .map(str::to_string),
            );
            args.push(run.join("final.txt").display().to_string());
            for setting in [
                format!("mcp_servers.stub.url={}", json!(url)),
                format!(
                    "mcp_servers.stub.http_headers_helper={}",
                    json!(helper_command(exe, run))
                ),
                format!(
                    "mcp_servers.stub.startup_readiness={}",
                    json!(if cell.cache { "catalog" } else { "connection" })
                ),
                "mcp_servers.stub.required=true".into(),
                // Codex asks before an MCP call that is not read-only, and refuses it headless;
                // one approval of the stub's server, as the first build's increment 3 needs,
                // leaves the stub's answer the only decision (A70, mistake 3).
                "mcp_servers.stub.default_tools_approval_mode=\"approve\"".into(),
                "model_reasoning_effort=\"low\"".into(),
                "web_search=\"disabled\"".into(),
            ] {
                args.extend(["-c".into(), setting]);
            }
            args.push(cell.prompt());
        }
    }
    Plan {
        program: cell.client.name().into(),
        args,
        env,
        // These are names only: no credential value is read by the runner.
        env_remove: [
            "ANTHROPIC_API_KEY",
            "OPENAI_API_KEY",
            "CLAUDECODE",
            "MCP_CONFIG",
        ]
        .map(str::to_string)
        .into(),
        cwd: run.join("lab"),
        stdin_message: None,
    }
}

/// Claude's second plan describes input to the first process, not a second launch.
/// Codex puts exec-only flags before the resume subcommand (installed CLI grammar).
pub fn turn_plan(
    cell: &Cell,
    run: &Path,
    exe: &Path,
    url: &str,
    turn: usize,
    session: Option<&str>,
) -> Plan {
    assert!(cell.later && matches!(turn, 1 | 2));
    let mut plan = plan(cell, run, exe, url);
    plan.args.pop();
    let prompt = cell.turn_prompt(turn);
    match cell.client {
        Client::Claude => {
            plan.args
                .extend(["--input-format".into(), "stream-json".into()]);
            plan.stdin_message = Some(
                json!({"type":"user", "message":{"role":"user", "content":prompt}, "parent_tool_use_id":null, "session_id":""}),
            );
        }
        Client::Codex => {
            plan.args.retain(|arg| arg != "--ephemeral");
            let output_index = plan
                .args
                .iter()
                .position(|arg| arg == "--output-last-message")
                .unwrap()
                + 1;
            plan.args[output_index] = run
                .join(format!("turn-{turn}/final.txt"))
                .display()
                .to_string();
            if turn == 2 {
                // --sandbox and --color are defined on exec, not exec resume.
                let mut parent = vec!["exec".to_string()];
                for flag in ["--sandbox", "--color"] {
                    let index = plan.args.iter().position(|arg| arg == flag).unwrap();
                    parent.extend(plan.args.drain(index..index + 2));
                }
                parent.push("resume".into());
                parent.extend(plan.args.drain(1..));
                plan.args = parent;
                plan.args.push(
                    session
                        .expect("resume requires the exact first thread ID")
                        .into(),
                );
            }
            plan.args.push(prompt);
        }
    }
    plan
}

pub fn configs(cell: &Cell, run: &Path, exe: &Path, url: &str) -> Value {
    json!({"mcp.json":{"mcpServers":{"stub":{"type":"http","url":url,"headersHelper":helper_command(exe,run)}}},"settings.json":{"permissions":{"allow":["mcp__stub__*"],"defaultMode":"dontAsk"},"disableAllHooks":true,"enabledPlugins":{}},"cell":cell})
}

pub async fn run(
    output: &Path,
    dry_run: bool,
    only: Option<&str>,
    timeout_secs: u64,
    follow_up: bool,
) -> Result<()> {
    let selected: Vec<_> = if follow_up {
        follow_up_matrix()
    } else {
        matrix()
    }
    .into_iter()
    .filter(|cell| only.is_none_or(|id| cell.id() == id))
    .collect();
    if selected.is_empty() {
        bail!("unknown --only cell; use --dry-run to list cell IDs")
    }
    let exe = std::env::current_exe()?;
    if dry_run {
        for cell in &selected {
            for rep in 1..=cell.repetitions() {
                let dir = output.join(format!("{}-r{rep}", cell.id()));
                println!("RUN {}-r{rep}", cell.id());
                println!(
                    "{} --serve --case {} --ttl-ms {} {} --output {} --port 0",
                    quote(&exe.display().to_string()),
                    cell.case.name().to_lowercase(),
                    cell.ttl,
                    if cell.first_401 { "--first-401" } else { "" },
                    quote(&dir.display().to_string())
                );
                println!(
                    "CONFIG {}",
                    configs(cell, &dir, &exe, "http://127.0.0.1:<free-port>/")
                );
                let mut version_plan = plan(cell, &dir, &exe, "http://127.0.0.1:<free-port>/");
                version_plan.args = vec!["--version".into()];
                println!("{}", version_plan.display());
                if cell.later {
                    for turn in 1..=2 {
                        let plan = turn_plan(
                            cell,
                            &dir,
                            &exe,
                            "http://127.0.0.1:<free-port>/",
                            turn,
                            Some("<turn-1-thread-id>"),
                        );
                        println!(
                            "TURN {turn} {}",
                            if turn == 1 {
                                "START"
                            } else {
                                "AFTER TURN 1 RESULT"
                            }
                        );
                        if turn == 1 || matches!(cell.client, Client::Codex) {
                            println!("{}", plan.display());
                        }
                        if let Some(message) = plan.stdin_message {
                            println!("SEND TURN {turn} {message}");
                        }
                    }
                } else {
                    println!(
                        "{}",
                        plan(cell, &dir, &exe, "http://127.0.0.1:<free-port>/").display()
                    );
                }
            }
        }
        println!(
            "TOTAL {} runs ({} cells; 3 repetitions per normal cell, 1 per 401 cell)",
            selected.iter().map(Cell::repetitions).sum::<usize>(),
            selected.len()
        );
        return Ok(());
    }
    let output = prepare_output(output)?;
    for cell in &selected {
        for rep in 1..=cell.repetitions() {
            let dir = output.join(format!("{}-r{rep}", cell.id()));
            fs::create_dir(&dir)
                .with_context(|| format!("fresh run directory {}", dir.display()))?;
            for name in ["lab", "tmp", "npm-cache"] {
                fs::create_dir(dir.join(name))?;
            }
            let server = stub::start(cell.case, cell.ttl, cell.first_401, &dir, 0).await?;
            let config = configs(cell, &dir, &exe, &server.url);
            for name in ["mcp.json", "settings.json"] {
                fs::write(dir.join(name), serde_json::to_vec_pretty(&config[name])?)?;
            }
            let plan = plan(cell, &dir, &exe, &server.url);
            fs::write(dir.join("plan.json"), serde_json::to_vec_pretty(&plan)?)?;
            fs::write(dir.join("prompt.txt"), cell.prompt())?;
            fs::write(
                dir.join("cell.json"),
                serde_json::to_vec_pretty(
                    &json!({"cell":cell,"repetition":rep,"timeout_secs":timeout_secs,"model_visibility":"Only directly exposed client events are evidence; wire is recorded separately."}),
                )?,
            )?;
            println!("RUN {}-r{rep}", cell.id());
            // Version commands are also captured; they neither start a model nor read credentials here.
            let mut version_plan = self::plan(cell, &dir, &exe, &server.url);
            version_plan.args = vec!["--version".into()];
            let version = version_plan.command().output().await;
            match version {
                Ok(v) => {
                    fs::write(dir.join("version.txt"), v.stdout)?;
                    fs::write(dir.join("version-stderr.txt"), v.stderr)?;
                }
                Err(e) => {
                    fs::write(dir.join("version-stderr.txt"), e.to_string())?;
                }
            }
            let result = if cell.later {
                execute_later(cell, &dir, &exe, &server, timeout_secs).await
            } else {
                execute(&plan, &dir, timeout_secs).await
            };
            server.cancel.cancel();
            server.task.abort();
            // Keep errors as observations; a missing client does not destroy earlier runs.
            fs::write(
                dir.join("status.json"),
                serde_json::to_vec_pretty(&match result {
                    Ok(v) => v,
                    Err(e) => json!({"runner_error":e.to_string()}),
                })?,
            )?;
            if !cell.later {
                extract_final(cell.client, &dir)?;
            }
        }
    }
    Ok(())
}
async fn execute(plan: &Plan, dir: &Path, timeout_secs: u64) -> Result<Value> {
    let mut command = plan.command();
    command
        .stdout(File::create(dir.join("events.jsonl"))?)
        .stderr(File::create(dir.join("stderr.txt"))?)
        .kill_on_drop(true);
    let mut child = command.spawn()?;
    let result = tokio::time::timeout(Duration::from_secs(timeout_secs), child.wait()).await;
    match result {
        Ok(status) => Ok(json!({"exit_code":status?.code(),"timed_out":false})),
        Err(_) => {
            child.kill().await?;
            child.wait().await?;
            Ok(json!({"exit_code":null,"timed_out":true}))
        }
    }
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    fs::write(path, serde_json::to_vec_pretty(value)?)?;
    Ok(())
}

async fn execute_later(
    cell: &Cell,
    dir: &Path,
    exe: &Path,
    server: &stub::RunningStub,
    timeout_secs: u64,
) -> Result<Value> {
    for turn in 1..=2 {
        let turn_dir = dir.join(format!("turn-{turn}"));
        fs::create_dir(&turn_dir)?;
        fs::write(turn_dir.join("prompt.txt"), cell.turn_prompt(turn))?;
    }
    let first = turn_plan(cell, dir, exe, &server.url, 1, None);
    write_json(&dir.join("plan.json"), &first)?;
    write_json(&dir.join("turn-1/plan.json"), &first)?;
    let result = match cell.client {
        Client::Claude => {
            let second = turn_plan(cell, dir, exe, &server.url, 2, None);
            write_json(&dir.join("turn-2/plan.json"), &second)?;
            execute_streamed_turns(&first, &second, dir, &server.log, timeout_secs).await
        }
        Client::Codex => execute_resumed_turns(cell, &first, dir, exe, server, timeout_secs).await,
    };
    // Extraction also runs after a timeout, failed launch or incomplete stream.
    let log = fs::read_to_string(dir.join("stub.jsonl"))?;
    let mut turns = Vec::new();
    for turn in 1..=2 {
        let turn_dir = dir.join(format!("turn-{turn}"));
        let status_path = turn_dir.join("status.json");
        if !status_path.exists() {
            write_json(
                &status_path,
                &json!({"not_completed":true,"runner_error":result.as_ref().err().map(ToString::to_string),"note":"See run status and events; no successful turn is inferred."}),
            )?;
        }
        turns.push(serde_json::from_slice::<Value>(&fs::read(status_path)?)?);
        extract_final(cell.client, &turn_dir)?;
        let mut file = File::create(turn_dir.join("stub.jsonl"))?;
        use std::io::Write;
        for line in log.lines() {
            if serde_json::from_str::<Value>(line).is_ok_and(|event| event["turn"] == turn) {
                writeln!(file, "{line}")?;
            }
        }
    }
    Ok(
        json!({"turns":turns,"process":result.as_ref().ok(),"runner_error":result.as_ref().err().map(ToString::to_string)}),
    )
}

/// Require both a successful exit and a completed first turn before resuming.
pub fn completed_session(events: &str) -> Result<String> {
    let mut session = None;
    let mut completed = false;
    for line in events.lines() {
        if let Ok(event) = serde_json::from_str::<Value>(line) {
            match event["type"].as_str() {
                Some("thread.started") => {
                    let id = event["thread_id"]
                        .as_str()
                        .context("thread.started has no thread_id")?;
                    if session.as_deref().is_some_and(|previous| previous != id) {
                        bail!("multiple session IDs in first turn")
                    }
                    session = Some(id.to_string());
                }
                Some("turn.completed") => completed = true,
                Some("turn.failed" | "error") => bail!("first Codex turn reported failure"),
                _ => {}
            }
        }
    }
    if !completed {
        bail!("first Codex turn has no turn.completed event")
    }
    session.context("first Codex turn has no exact thread ID; refusing to resume another session")
}

async fn execute_resumed_turns(
    cell: &Cell,
    first: &Plan,
    dir: &Path,
    exe: &Path,
    server: &stub::RunningStub,
    timeout_secs: u64,
) -> Result<Value> {
    server.log.begin_turn(1);
    let status = execute(first, &dir.join("turn-1"), timeout_secs).await?;
    write_json(&dir.join("turn-1/status.json"), &status)?;
    if status["exit_code"] != 0 || status["timed_out"] != false {
        bail!("first Codex process did not succeed; second turn not sent")
    }
    let events = fs::read_to_string(dir.join("turn-1/events.jsonl"))?;
    let session = completed_session(&events)?;
    write_json(
        &dir.join("session.json"),
        &json!({"thread_id":session,"source":"turn-1/events.jsonl"}),
    )?;
    let second = turn_plan(cell, dir, exe, &server.url, 2, Some(&session));
    write_json(&dir.join("turn-2/plan.json"), &second)?;
    server.log.begin_turn(2);
    let status = execute(&second, &dir.join("turn-2"), timeout_secs).await?;
    write_json(&dir.join("turn-2/status.json"), &status)?;
    // Preserve events and flag any unexpected session rather than treating it as a result.
    let events = fs::read_to_string(dir.join("turn-2/events.jsonl"))?;
    for line in events.lines() {
        if let Ok(event) = serde_json::from_str::<Value>(line)
            && event["type"] == "thread.started"
            && event["thread_id"] != session
        {
            bail!("Codex resume reported a different session ID")
        }
    }
    Ok(status)
}

/// One Claude process, with the next JSONL user message gated by a result event.
async fn execute_streamed_turns(
    first: &Plan,
    second: &Plan,
    dir: &Path,
    log: &stub::Log,
    timeout_secs: u64,
) -> Result<Value> {
    use std::io::Write;
    let mut command = first.command();
    command
        .stdout(Stdio::piped())
        .stderr(File::create(dir.join("stderr.txt"))?)
        .kill_on_drop(true);
    log.begin_turn(1);
    let mut child = command.spawn()?;
    let mut input = child.stdin.take().context("streamed stdin")?;
    let mut output = BufReader::new(child.stdout.take().context("streamed stdout")?);
    let mut raw = File::create(dir.join("events.jsonl"))?;
    for (index, plan) in [first, second].into_iter().enumerate() {
        let turn = index + 1;
        let turn_dir = dir.join(format!("turn-{turn}"));
        let mut events = File::create(turn_dir.join("events.jsonl"))?;
        let message = format!(
            "{}\n",
            plan.stdin_message
                .as_ref()
                .context("streamed user message")?
        );
        fs::write(turn_dir.join("input.jsonl"), &message)?;
        if turn == 2 {
            log.begin_turn(turn);
        }
        let result = tokio::time::timeout(Duration::from_secs(timeout_secs), async {
            input.write_all(message.as_bytes()).await?;
            input.flush().await?;
            loop {
                let mut bytes = Vec::new();
                if output.read_until(b'\n', &mut bytes).await? == 0 {
                    bail!("Claude closed stdout before turn {turn}'s result")
                }
                raw.write_all(&bytes)?;
                events.write_all(&bytes)?;
                if let Ok(event) = serde_json::from_slice::<Value>(&bytes)
                    && event["type"] == "result"
                {
                    return Ok(event);
                }
            }
        })
        .await;
        let event = match result {
            Ok(Ok(event)) => event,
            other => {
                let status = match other {
                    Err(_) => json!({"timed_out":true,"completed":false}),
                    Ok(Err(error)) => json!({"runner_error":error.to_string(),"completed":false}),
                    _ => unreachable!(),
                };
                write_json(&turn_dir.join("status.json"), &status)?;
                child.kill().await?;
                child.wait().await?;
                return Ok(status);
            }
        };
        write_json(
            &turn_dir.join("status.json"),
            &json!({"completed":true,"timed_out":false,"result_is_error":event["is_error"] == true,"result_subtype":event["subtype"]}),
        )?;
        if event["is_error"] == true {
            child.kill().await?;
            child.wait().await?;
            bail!("Claude turn {turn} returned an error result; no further input sent")
        }
    }
    // EOF is sent only after the second result; drain trailing events before waiting.
    drop(input);
    let mut trailing = File::options()
        .append(true)
        .open(dir.join("turn-2/events.jsonl"))?;
    let finish = tokio::time::timeout(Duration::from_secs(timeout_secs), async {
        loop {
            let mut bytes = Vec::new();
            if output.read_until(b'\n', &mut bytes).await? == 0 {
                break;
            }
            raw.write_all(&bytes)?;
            trailing.write_all(&bytes)?;
        }
        Ok::<_, anyhow::Error>(child.wait().await?)
    })
    .await;
    match finish {
        Ok(status) => Ok(json!({"exit_code":status?.code(),"timed_out":false})),
        Err(_) => {
            child.kill().await?;
            child.wait().await?;
            Ok(json!({"exit_code":null,"timed_out":true,"phase":"shutdown after second result"}))
        }
    }
}

pub fn prepare_output(path: &Path) -> Result<PathBuf> {
    if path
        .components()
        .any(|part| part == std::path::Component::ParentDir)
    {
        bail!("--output must not contain parent-directory components")
    }
    if !path.is_absolute() {
        bail!("--output must be an absolute directory outside the repository")
    }
    // Check the existing ancestor before creating anything, including symlink aliases.
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    let mut ancestor = path;
    while !ancestor.exists() {
        ancestor = ancestor.parent().context("output ancestor")?;
    }
    if ancestor.canonicalize()?.starts_with(&repo) {
        bail!("output must be outside the repository")
    }
    fs::create_dir_all(path)?;
    let path = path.canonicalize()?;
    if path.starts_with(&repo) {
        bail!("output must be outside the repository")
    }
    Ok(path)
}
fn extract_final(client: Client, dir: &Path) -> Result<()> {
    let events = fs::read_to_string(dir.join("events.jsonl")).unwrap_or_default();
    let mut final_text = String::new();
    let mut visible = Vec::new();
    let mut authentication = Vec::new();
    for (line_number, line) in events.lines().enumerate() {
        if let Ok(event) = serde_json::from_str::<Value>(line) {
            // Authentication evidence comes from client metadata, not model/tool content.
            let metadata = match client {
                Client::Claude => event["type"] == "system" && event["subtype"] == "init",
                Client::Codex => matches!(
                    event["type"].as_str(),
                    Some("thread.started" | "session.started" | "auth.status")
                ),
            };
            if metadata {
                collect_authentication(&event, "", line_number + 1, &mut authentication);
            }
            // These are client-exposed events, NOT an inferred model prompt.
            if event["type"] == "user"
                || event["type"] == "assistant"
                || event["type"] == "item.completed"
            {
                visible.push(event.clone());
            }
            match client {
                Client::Claude if event["type"] == "result" => {
                    final_text = event["result"].as_str().unwrap_or("").into();
                }
                Client::Codex
                    if event["type"] == "item.completed"
                        && event["item"]["type"] == "agent_message" =>
                {
                    final_text = event["item"]["text"].as_str().unwrap_or("").into();
                }
                _ => {}
            }
        }
    }
    fs::write(
        dir.join("authentication.json"),
        serde_json::to_vec_pretty(&json!({
            "requested":"owner subscription login",
            "client":client.name(),
            "source":"events.jsonl",
            "status":if authentication.is_empty() { "not reported in client events" } else { "reported; inspect observations" },
            "observations":authentication,
            "note":"Only client-reported authentication fields are copied. The requested login and removed API key variables do not prove which authentication the client used."
        }))?,
    )?;
    if !dir.join("final.txt").exists() {
        fs::write(dir.join("final.txt"), final_text)?;
    }
    let mut file = File::create(dir.join("client-exposed.jsonl"))?;
    use std::io::Write;
    for event in visible {
        writeln!(file, "{event}")?;
    }
    fs::write(
        dir.join("visibility.txt"),
        "stub.jsonl records wire facts. client-exposed.jsonl is a subset of the unmodified event stream, not proof of the exact model input. No visibility conclusion is inferred from _meta on the wire. If the client does not expose the model input, mark that observation unavailable.\n",
    )?;
    Ok(())
}

// Copy only mode/source fields, never credential contents or entire auth objects.
fn collect_authentication(value: &Value, pointer: &str, line: usize, out: &mut Vec<Value>) {
    match value {
        Value::Object(fields) => {
            for (key, value) in fields {
                let pointer = format!("{pointer}/{}", key.replace('~', "~0").replace('/', "~1"));
                if matches!(
                    key.as_str(),
                    "apiKeySource" | "authMode" | "auth_mode" | "authenticationMode"
                ) && value.is_string()
                {
                    out.push(json!({"line":line,"pointer":pointer,"value":value}));
                } else {
                    collect_authentication(value, &pointer, line, out);
                }
            }
        }
        Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                collect_authentication(value, &format!("{pointer}/{index}"), line, out);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn streamed_turns_wait_for_results_keep_one_process_and_close_input_after_turn_two() {
        let dir = tempfile::tempdir().unwrap();
        for turn in 1..=2 {
            fs::create_dir(dir.path().join(format!("turn-{turn}"))).unwrap();
        }
        let log = stub::Log::new(&dir.path().join("stub.jsonl")).unwrap();
        // A local shell simulates the CLI protocol; it never launches an agent.
        // Bash 3 on macOS needs an integer timeout. Elapsed SECONDS also
        // distinguishes an open, idle pipe from premature EOF.
        let script = r#"
read -r first || exit 11
[[ "$first" == *first* ]] || exit 12
before=$SECONDS
read -r -t 1 premature && exit 13
(( SECONDS - before >= 1 )) || exit 13
printf '{"type":"assistant","message":{"content":"first"},"pid":%s}\n' "$$"
printf '%s\n' '{"type":"result","subtype":"success","is_error":false,"result":"first final"}'
read -r second || exit 14
[[ "$second" == *second* ]] || exit 15
before=$SECONDS
read -r -t 1 premature && exit 16
(( SECONDS - before >= 1 )) || exit 16
printf '{"type":"assistant","message":{"content":"second"},"pid":%s}\n' "$$"
printf '%s\n' '{"type":"result","subtype":"success","is_error":false,"result":"second final"}'
read -r extra && exit 17
exit 0
"#;
        let mut first = Plan {
            program: "/bin/bash".into(),
            args: vec!["-c".into(), script.into()],
            env: BTreeMap::new(),
            env_remove: vec![],
            cwd: dir.path().into(),
            stdin_message: Some(json!({"type":"user","message":{"role":"user","content":"first"}})),
        };
        let second = Plan {
            program: "must-never-be-launched".into(),
            args: vec![],
            env: BTreeMap::new(),
            env_remove: vec![],
            cwd: dir.path().into(),
            stdin_message: Some(
                json!({"type":"user","message":{"role":"user","content":"second"}}),
            ),
        };
        let status = execute_streamed_turns(&first, &second, dir.path(), &log, 5)
            .await
            .unwrap();
        assert_eq!(status["exit_code"], 0, "{status}");
        let mut pids = Vec::new();
        for (turn, expected) in [(1, "first final"), (2, "second final")] {
            let path = dir.path().join(format!("turn-{turn}"));
            extract_final(Client::Claude, &path).unwrap();
            assert_eq!(
                fs::read_to_string(path.join("final.txt")).unwrap(),
                expected
            );
            let events = fs::read_to_string(path.join("events.jsonl")).unwrap();
            assert_eq!(events.lines().count(), 2);
            pids.push(
                serde_json::from_str::<Value>(events.lines().next().unwrap()).unwrap()["pid"]
                    .clone(),
            );
        }
        assert_eq!(pids[0], pids[1]);
        let markers: Vec<Value> = fs::read_to_string(dir.path().join("stub.jsonl"))
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(
            markers
                .iter()
                .map(|event| event["turn"].as_u64().unwrap())
                .collect::<Vec<_>>(),
            [1, 2]
        );

        // A closed stdout before a result must leave the second input unsent.
        let failed = tempfile::tempdir().unwrap();
        for turn in 1..=2 {
            fs::create_dir(failed.path().join(format!("turn-{turn}"))).unwrap();
        }
        first.args = vec![
            "-c".into(),
            "read -r first; printf '%s\\n' '{\"type\":\"assistant\"}'".into(),
        ];
        let log = stub::Log::new(&failed.path().join("stub.jsonl")).unwrap();
        let status = execute_streamed_turns(&first, &second, failed.path(), &log, 5)
            .await
            .unwrap();
        assert_eq!(status["completed"], false);
        assert!(!failed.path().join("turn-2/input.jsonl").exists());
    }

    #[test]
    fn authentication_records_client_report_with_event_location_or_unavailable() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("events.jsonl"),
            concat!(
                "{\"type\":\"system\",\"subtype\":\"init\",\"apiKeySource\":\"none\"}\n",
                "{\"type\":\"assistant\",\"apiKeySource\":\"invented-by-model\"}\n"
            ),
        )
        .unwrap();
        extract_final(Client::Claude, dir.path()).unwrap();
        let report: Value =
            serde_json::from_slice(&fs::read(dir.path().join("authentication.json")).unwrap())
                .unwrap();
        assert_eq!(
            report["observations"],
            json!([{"line":1,"pointer":"/apiKeySource","value":"none"}])
        );
        fs::write(
            dir.path().join("events.jsonl"),
            "{\"type\":\"thread.started\",\"thread_id\":\"test\"}\n",
        )
        .unwrap();
        extract_final(Client::Codex, dir.path()).unwrap();
        let report: Value =
            serde_json::from_slice(&fs::read(dir.path().join("authentication.json")).unwrap())
                .unwrap();
        assert_eq!(report["observations"], json!([]));
        assert_eq!(report["status"], "not reported in client events");
    }
}
