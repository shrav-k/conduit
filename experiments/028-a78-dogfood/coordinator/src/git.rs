//! Git, as the queue runs it: always with hooks, signing, `rebase.updateRefs` and rerere off.

use crate::process::{Finished, Runner};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The configuration every git command the queue runs carries. Rerere is off whatever the
/// caller's configuration says: a recorded resolution replays a person's choice of a side, which
/// the queue never makes (decision 6).
pub const FIXED: &[&str] = &[
    "-c",
    "core.hooksPath=/dev/null",
    "-c",
    "commit.gpgsign=false",
    "-c",
    "rebase.updateRefs=false",
    "-c",
    "rerere.enabled=false",
    "-c",
    "rerere.autoUpdate=false",
];

/// Git in one directory.
#[derive(Debug, Clone)]
pub struct Git<'a> {
    runner: &'a Runner,
    dir: PathBuf,
}

impl<'a> Git<'a> {
    pub fn new(runner: &'a Runner, dir: &Path) -> Self {
        Self {
            runner,
            dir: dir.to_path_buf(),
        }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn argv(args: &[&str]) -> Vec<String> {
        let mut argv = vec!["git".to_string()];
        argv.extend(FIXED.iter().map(|s| s.to_string()));
        argv.extend(args.iter().map(|s| s.to_string()));
        argv
    }

    /// Runs git to its end, whatever its status.
    pub fn run(&self, args: &[&str]) -> Result<Finished, String> {
        self.runner
            .run(&Self::argv(args), &self.dir, &BTreeMap::new(), None)
    }

    /// Runs git with `input` on standard input.
    pub fn run_input(&self, args: &[&str], input: &str) -> Result<Finished, String> {
        self.runner
            .run(&Self::argv(args), &self.dir, &BTreeMap::new(), Some(input))
    }

    /// Runs git and gives its standard output, trimmed, or why it failed.
    pub fn text(&self, args: &[&str]) -> Result<String, String> {
        let finished = self.run(args)?;
        if finished.success() {
            Ok(finished.stdout.trim().to_string())
        } else {
            Err(format!(
                "git {} ended {}: {}",
                args.join(" "),
                finished.status(),
                crate::cut(&finished.last_line(), 200)
            ))
        }
    }

    /// The commit a revision names, or why it names none.
    pub fn commit(&self, revision: &str) -> Result<String, String> {
        self.text(&[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{revision}^{{commit}}"),
        ])
        .map_err(|_| format!("{revision} names no commit"))
    }

    /// The tree a revision names.
    pub fn tree(&self, revision: &str) -> Result<String, String> {
        self.text(&["rev-parse", "--verify", &format!("{revision}^{{tree}}")])
    }

    /// Whether `ancestor` is an ancestor of `descendant`; `None` when git cannot say.
    pub fn is_ancestor(&self, ancestor: &str, descendant: &str) -> Option<bool> {
        match self.run(&["merge-base", "--is-ancestor", ancestor, descendant]) {
            Ok(finished) if finished.code == Some(0) => Some(true),
            Ok(finished) if finished.code == Some(1) => Some(false),
            _ => None,
        }
    }

    /// The paths `git status` reports, untracked ones included, as `XY path` lines.
    pub fn status_lines(&self) -> Result<Vec<String>, String> {
        let finished = self.run(&[
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--ignore-submodules=none",
        ])?;
        if !finished.success() {
            return Err(format!(
                "git status ended {}: {}",
                finished.status(),
                crate::cut(&finished.last_line(), 200)
            ));
        }
        let mut lines = Vec::new();
        let mut parts = finished.stdout.split('\0').filter(|part| !part.is_empty());
        while let Some(part) = parts.next() {
            if part.starts_with('R') || part.starts_with('C') {
                parts.next();
            }
            lines.push(part.to_string());
        }
        Ok(lines)
    }

    /// The paths that differ between two trees.
    pub fn changed_paths(&self, from: &str, to: &str) -> Result<Vec<String>, String> {
        let text = self.text(&["diff", "--name-only", "-z", "--no-renames", from, to])?;
        Ok(text
            .split('\0')
            .filter(|path| !path.is_empty())
            .map(str::to_string)
            .collect())
    }
}
