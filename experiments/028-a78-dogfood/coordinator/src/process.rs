//! Starting the commands the queue runs.
//!
//! Every command gets `PYTHONDONTWRITEBYTECODE=1`, `GIT_EDITOR=true` and the identity as
//! `GIT_AUTHOR_*` and `GIT_COMMITTER_*`; the rest of the queue's environment passes on, and the
//! queue never prints it. Standard input is empty unless a step gives it text.

use crate::config::Identity;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// What a finished command left.
#[derive(Debug, Clone)]
pub struct Finished {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl Finished {
    pub fn success(&self) -> bool {
        self.code == Some(0)
    }

    /// The last line that is not blank, from standard error, else from standard output.
    pub fn last_line(&self) -> String {
        let line = crate::last_line(&self.stderr);
        if line.is_empty() {
            crate::last_line(&self.stdout)
        } else {
            line
        }
    }

    /// How the command ended, as words.
    pub fn status(&self) -> String {
        match self.code {
            Some(code) => format!("exit {code}"),
            None => "ended by a signal".to_string(),
        }
    }
}

/// Builds every command the queue starts.
#[derive(Debug, Clone)]
pub struct Runner {
    fixed: Vec<(String, String)>,
}

impl Runner {
    pub fn new(identity: &Identity) -> Self {
        let mut fixed = vec![
            ("PYTHONDONTWRITEBYTECODE".to_string(), "1".to_string()),
            ("GIT_EDITOR".to_string(), "true".to_string()),
        ];
        for who in ["AUTHOR", "COMMITTER"] {
            fixed.push((format!("GIT_{who}_NAME"), identity.name.clone()));
            fixed.push((format!("GIT_{who}_EMAIL"), identity.email.clone()));
        }
        Self { fixed }
    }

    /// A command for `argv` in `cwd`, with the fixed environment and `extra` set.
    pub fn command(
        &self,
        argv: &[String],
        cwd: &Path,
        extra: &BTreeMap<String, String>,
    ) -> Command {
        let mut command = Command::new(&argv[0]);
        command
            .args(&argv[1..])
            .current_dir(cwd)
            .stdin(Stdio::null());
        for (name, value) in extra {
            command.env(name, value);
        }
        for (name, value) in &self.fixed {
            command.env(name, value);
        }
        command
    }

    /// Runs `argv` in `cwd` to its end, with `input` on standard input when given.
    pub fn run(
        &self,
        argv: &[String],
        cwd: &Path,
        extra: &BTreeMap<String, String>,
        input: Option<&str>,
    ) -> Result<Finished, String> {
        let mut command = self.command(argv, cwd, extra);
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
        if input.is_some() {
            command.stdin(Stdio::piped());
        }
        let mut child = command
            .spawn()
            .map_err(|error| format!("{} could not start: {error}", argv[0]))?;
        if let Some(text) = input {
            let mut stdin = child.stdin.take().ok_or("no standard input".to_string())?;
            stdin
                .write_all(text.as_bytes())
                .map_err(|error| format!("{} did not take its input: {error}", argv[0]))?;
        }
        let output = child
            .wait_with_output()
            .map_err(|error| format!("{} could not be waited on: {error}", argv[0]))?;
        Ok(Finished {
            code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}
