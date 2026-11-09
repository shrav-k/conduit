pub mod client;
pub mod runner;
pub mod wrapper;

use anyhow::Result;
use real_client_harness::Names;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, os::unix::fs::DirBuilderExt, path::Path};
use tokio::process::Command;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
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

pub const BASE_ENV: [&str; 6] = ["HOME", "LOGNAME", "PATH", "SHELL", "TERM", "USER"];

/// Read only the six allowed names, never an ambient credential's value.
pub fn inherited_environment() -> BTreeMap<String, String> {
    BASE_ENV
        .into_iter()
        .filter_map(|key| std::env::var(key).ok().map(|value| (key.to_owned(), value)))
        .collect()
}

/// PRV-023: no inherited variables except these six, and no exported functions.
pub fn run_environment(
    source: impl IntoIterator<Item = (String, String)>,
    url: &str,
    identity: &Path,
) -> BTreeMap<String, String> {
    let mut env: BTreeMap<_, _> = source
        .into_iter()
        .filter(|(key, value)| BASE_ENV.contains(&key.as_str()) && !value.starts_with("()"))
        .collect();
    let names = Names::default();
    env.insert(names.get("engineUrlVariable").into(), url.into());
    env.insert(
        names.get("identityDirVariable").into(),
        identity.display().to_string(),
    );
    env
}

pub fn clean_command(program: &Path, env: &BTreeMap<String, String>, cwd: &Path) -> Command {
    let mut command = Command::new(program);
    command.env_clear().envs(env).current_dir(cwd);
    // Explicit removals also document the required subscription authentication policy.
    command
        .env_remove("ANTHROPIC_API_KEY")
        .env_remove("OPENAI_API_KEY");
    command.kill_on_drop(true);
    command
}

pub fn private_dir(path: &Path) -> Result<()> {
    fs::DirBuilder::new().mode(0o700).create(path)?;
    Ok(())
}

pub fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    fs::write(path, serde_json::to_vec_pretty(value)?)?;
    Ok(())
}

/// Only the token of this throwaway experiment is read, never owner credentials.
pub fn headers_helper() -> Result<()> {
    let names = Names::default();
    // Codex runs its header helper with a filtered environment and cannot forward a variable to
    // it over HTTP, so its wrapper writes the directory into the helper's command line; Claude
    // Code passes the variable through (experiment 009).
    let args: Vec<String> = std::env::args().collect();
    let dir = match args.iter().position(|a| a == "--identity-dir") {
        Some(i) => args
            .get(i + 1)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("--identity-dir needs a directory"))?,
        None => std::env::var(names.get("identityDirVariable"))?,
    };
    real_client_harness::stub::headers_helper(
        Path::new(&dir),
        &Path::new(&dir).join("helper.jsonl"),
    )
}

pub fn prompt(turn: usize) -> &'static str {
    match turn {
        1 => {
            "Use only the stub MCP server. Call files.read.v1 with path hello.txt once. Report the returned text and stop. Do not call control.next.v1 or any other tool."
        }
        _ => {
            "Access changed between turns. Use only the stub MCP server. Call notes.read.v1 with path hello.txt once (discover it if needed). Also read hello.txt through files.read.v1 if it remains available; otherwise report it unavailable. Report what happened and stop. Do not call control.next.v1 or substitute other tools."
        }
    }
}

pub fn user_message(prompt: &str) -> Value {
    json!({"type":"user","message":{"role":"user","content":prompt},"parent_tool_use_id":null,"session_id":""})
}
