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
use tokio::process::Command;

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
}
impl Cell {
    pub fn repetitions(&self) -> usize {
        if self.first_401 { 1 } else { 3 }
    }
    pub fn id(&self) -> String {
        format!(
            "{}-{}-cache-{}-ttl-{}-auth-{}",
            self.client.name(),
            self.case.name(),
            if self.cache { "on" } else { "off" },
            self.ttl,
            if self.first_401 { "401" } else { "valid" }
        )
    }
    pub fn prompt(&self) -> String {
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
        });
    }
    cells
}

#[derive(Serialize)]
pub struct Plan {
    pub program: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub env_remove: Vec<String>,
    pub cwd: PathBuf,
}
impl Plan {
    pub fn command(&self) -> Command {
        let mut command = Command::new(&self.program);
        command
            .args(&self.args)
            .envs(&self.env)
            .current_dir(&self.cwd)
            .stdin(Stdio::null());
        for key in &self.env_remove {
            command.env_remove(key);
        }
        command
    }
    pub fn display(&self) -> String {
        format!(
            "cd {} && env {} {} {} {} < /dev/null",
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
                .join(" ")
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
    }
}

pub fn configs(cell: &Cell, run: &Path, exe: &Path, url: &str) -> Value {
    json!({"mcp.json":{"mcpServers":{"stub":{"type":"http","url":url,"headersHelper":helper_command(exe,run)}}},"settings.json":{"permissions":{"allow":["mcp__stub__*"],"defaultMode":"dontAsk"},"disableAllHooks":true,"enabledPlugins":{}},"cell":cell})
}

pub async fn run(
    output: &Path,
    dry_run: bool,
    only: Option<&str>,
    timeout_secs: u64,
) -> Result<()> {
    let selected: Vec<_> = matrix()
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
                println!(
                    "{}",
                    plan(cell, &dir, &exe, "http://127.0.0.1:<free-port>/").display()
                );
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
            let result = execute(&plan, &dir, timeout_secs).await;
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
            extract_final(cell.client, &dir)?;
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
