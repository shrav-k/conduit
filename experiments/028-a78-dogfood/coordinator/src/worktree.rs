//! A trusted worktree: a directory directly under `worktrees` whose `.git` is a file with a
//! `gitdir:` line naming a directory under the main clone's `.git/worktrees/`, read before any
//! git runs there.

use crate::config::Config;
use crate::git::Git;
use std::fs;
use std::path::{Path, PathBuf};

/// A worktree that passed the file checks.
#[derive(Debug, Clone)]
pub struct Worktree {
    pub dir: PathBuf,
    /// The worktree's own git directory, under the clone's `.git/worktrees/`.
    pub gitdir: PathBuf,
}

/// Whether `name` can name an entry: a directory name, a ref component and a file name.
pub fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.starts_with(|c: char| c.is_ascii_alphanumeric())
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "._-".contains(c))
        && !name.contains("..")
        && !name.ends_with(".lock")
        && !name.ends_with('.')
}

/// Finds the worktree an entry names, reading only files.
pub fn locate(config: &Config, name: &str) -> Result<Worktree, String> {
    if !valid_name(name) {
        return Err(format!("{name:?} cannot name a worktree's directory"));
    }
    let dir = config.worktrees.join(name);
    let shown = dir.display();
    let meta =
        fs::symlink_metadata(&dir).map_err(|error| format!("the worktree {shown}: {error}"))?;
    if meta.file_type().is_symlink() {
        return Err(format!(
            "the worktree {shown} is a symbolic link, not a directory under worktrees"
        ));
    }
    if !meta.is_dir() {
        return Err(format!("the worktree {shown} is not a directory"));
    }
    let dotgit = dir.join(".git");
    let meta = fs::symlink_metadata(&dotgit)
        .map_err(|error| format!("the worktree {shown} has no .git: {error}"))?;
    if meta.is_dir() {
        return Err(format!(
            "the worktree {shown} has a .git directory: it is a clone of its own, not a worktree of the main clone"
        ));
    }
    if !meta.is_file() {
        return Err(format!("the worktree {shown}'s .git is not a file"));
    }
    let text = fs::read_to_string(&dotgit)
        .map_err(|error| format!("the worktree {shown}'s .git cannot be read: {error}"))?;
    let named = text
        .lines()
        .find_map(|line| line.strip_prefix("gitdir:"))
        .map(str::trim)
        .ok_or(format!("the worktree {shown}'s .git has no gitdir: line"))?;
    let target = if Path::new(named).is_absolute() {
        PathBuf::from(named)
    } else {
        dir.join(named)
    };
    let gitdir = fs::canonicalize(&target)
        .map_err(|error| format!("the worktree {shown}'s gitdir: {named}: {error}"))?;
    let worktrees = fs::canonicalize(config.clone.join(".git").join("worktrees")).ok();
    if !gitdir.is_dir() || worktrees.is_none() || gitdir.parent() != worktrees.as_deref() {
        return Err(format!(
            "the worktree {shown}'s gitdir: line names {named}, which is not a directory under the clone's .git/worktrees/"
        ));
    }
    Ok(Worktree { dir, gitdir })
}

/// What git shows of a located worktree.
#[derive(Debug, Clone)]
pub struct Checked {
    pub branch: String,
    pub head: String,
}

/// The in-progress operations a worktree's git directory can hold.
const IN_PROGRESS: &[(&str, &str)] = &[
    ("rebase-merge", "a rebase"),
    ("rebase-apply", "a rebase"),
    ("MERGE_HEAD", "a merge"),
    ("CHERRY_PICK_HEAD", "a cherry-pick"),
    ("REVERT_HEAD", "a revert"),
];

/// The operation in progress in a worktree, when there is one.
pub fn in_progress(worktree: &Worktree) -> Option<&'static str> {
    IN_PROGRESS
        .iter()
        .find(|(file, _)| worktree.gitdir.join(file).exists())
        .map(|(_, what)| *what)
}

/// Checks a located worktree with git: no operation in progress, on a branch, and clean,
/// untracked files included.
pub fn check(worktree: &Worktree, git: &Git) -> Result<Checked, String> {
    let shown = worktree.dir.display();
    if let Some(what) = in_progress(worktree) {
        return Err(format!("the worktree {shown} has {what} in progress"));
    }
    let head_ref = git
        .text(&["symbolic-ref", "-q", "HEAD"])
        .map_err(|_| format!("the worktree {shown} is not on a branch"))?;
    let branch = head_ref
        .strip_prefix("refs/heads/")
        .ok_or(format!(
            "the worktree {shown} is not on a branch: {head_ref}"
        ))?
        .to_string();
    let head = git.commit("HEAD")?;
    let dirty = git.status_lines()?;
    if !dirty.is_empty() {
        return Err(format!(
            "the worktree {shown} is not clean: {} paths, the first {}",
            dirty.len(),
            crate::cut(&dirty[0], 200)
        ));
    }
    Ok(Checked { branch, head })
}
