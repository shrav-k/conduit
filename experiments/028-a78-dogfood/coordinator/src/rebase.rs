//! A stopped rebase (decision 6).
//!
//! Ours is the side rebased onto; theirs is the replayed commit (`--theirs` in a rebase). A
//! conflicted path without all three stages stops, naming the path and the kind.
//! `engine/STANDING.md` takes theirs and, once every other path is settled, the repository's
//! `promise_standing.py --write` regenerates it and it is added. Every other conflicted path
//! stops, naming it and its first hunk's first line. No hunk keeps one side over a changed other.

use crate::git::Git;
use crate::process::Runner;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// The one path the queue settles.
pub const STANDING: &str = "engine/STANDING.md";

/// The lines that mark a conflict; a path holding one is never added.
pub const MARKERS: &[&str] = &["<<<<<<< ", "||||||| ", ">>>>>>> "];

/// The conflicted paths and the stages `git ls-files -u` gives each.
pub fn unmerged(git: &Git) -> Result<BTreeMap<String, BTreeSet<u8>>, String> {
    let finished = git.run(&["ls-files", "-u", "-z"])?;
    if !finished.success() {
        return Err(format!(
            "git ls-files -u ended {}: {}",
            finished.status(),
            crate::cut(&finished.last_line(), 200)
        ));
    }
    let mut paths: BTreeMap<String, BTreeSet<u8>> = BTreeMap::new();
    for item in finished.stdout.split('\0').filter(|item| !item.is_empty()) {
        let Some((meta, path)) = item.split_once('\t') else {
            continue;
        };
        let stage = meta
            .rsplit(' ')
            .next()
            .and_then(|stage| stage.parse::<u8>().ok())
            .unwrap_or(0);
        paths.entry(path.to_string()).or_default().insert(stage);
    }
    Ok(paths)
}

/// The kind of a conflict that lacks one of the three stages.
pub fn kind(stages: &BTreeSet<u8>) -> String {
    let has = |stage| stages.contains(&stage);
    match (has(1), has(2), has(3)) {
        (true, true, false) => "changed by ours, deleted by theirs".into(),
        (true, false, true) => "deleted by ours, changed by theirs".into(),
        (false, true, true) => "added by both".into(),
        (false, true, false) => "added by ours alone".into(),
        (false, false, true) => "added by theirs alone".into(),
        (true, false, false) => "deleted by both".into(),
        _ => format!(
            "stages {}",
            stages
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// Whether a line of `text` begins a conflict marker.
pub fn has_marker(text: &str) -> bool {
    text.lines()
        .any(|line| MARKERS.iter().any(|marker| line.starts_with(marker)))
}

/// Where a conflicted file's first hunk starts, and that hunk's first line.
pub fn first_hunk(path: &Path) -> String {
    let Ok(bytes) = fs::read(path) else {
        return "the file cannot be read".into();
    };
    let text = String::from_utf8_lossy(&bytes);
    let lines: Vec<&str> = text.lines().collect();
    let Some(start) = lines.iter().position(|line| line.starts_with("<<<<<<< ")) else {
        return "no conflict marker in the file".into();
    };
    let first = lines[start + 1..]
        .iter()
        .find(|line| {
            !MARKERS.iter().any(|marker| line.starts_with(marker)) && !line.starts_with("=======")
        })
        .copied()
        .unwrap_or("(an empty hunk)");
    format!(
        "its first hunk at line {}: {}",
        start + 1,
        crate::cut(first, 200)
    )
}

/// Settles one stop of a rebase, or names each path it cannot settle and why.
pub fn settle(git: &Git, runner: &Runner, python: &[String]) -> Result<String, Vec<String>> {
    let conflicts = unmerged(git).map_err(|error| vec![error])?;
    if conflicts.is_empty() {
        return Err(vec!["the rebase stopped with no conflicted path".into()]);
    }
    let mut problems = Vec::new();
    let mut standing = false;
    for (path, stages) in &conflicts {
        if stages.len() != 3 || !stages.contains(&1) {
            problems.push(format!("{path}: {}", kind(stages)));
        } else if path == STANDING {
            standing = true;
        } else {
            problems.push(format!(
                "{path}: a conflict, {}",
                first_hunk(&git.dir().join(path))
            ));
        }
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    if !standing {
        return Err(vec!["the rebase stopped with no path it can settle".into()]);
    }
    git.text(&["checkout", "--theirs", "--", STANDING])
        .map_err(|error| vec![format!("{STANDING}: {error}")])?;
    let mut argv = python.to_vec();
    argv.extend([
        "tools/promise_standing.py".to_string(),
        "--write".to_string(),
    ]);
    let finished = runner
        .run(&argv, git.dir(), &BTreeMap::new(), None)
        .map_err(|error| vec![format!("{STANDING}: {error}")])?;
    if !finished.success() {
        return Err(vec![format!(
            "{STANDING}: tools/promise_standing.py --write ended {}: {}",
            finished.status(),
            crate::cut(&finished.last_line(), 200)
        )]);
    }
    let changed = git
        .text(&["diff", "--name-only", "-z"])
        .map_err(|error| vec![error])?;
    let others: Vec<&str> = changed
        .split('\0')
        .filter(|path| !path.is_empty() && *path != STANDING)
        .collect();
    if !others.is_empty() {
        return Err(vec![format!(
            "{STANDING}: tools/promise_standing.py --write also changed {}",
            others.join(", ")
        )]);
    }
    let text = fs::read(git.dir().join(STANDING))
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .map_err(|error| vec![format!("{STANDING} cannot be read: {error}")])?;
    if has_marker(&text) {
        return Err(vec![format!(
            "{STANDING}: still holds a conflict marker once regenerated"
        )]);
    }
    git.text(&["add", "--", STANDING])
        .map_err(|error| vec![format!("{STANDING}: {error}")])?;
    Ok(format!("{STANDING} took theirs and was regenerated"))
}

#[cfg(test)]
mod tests {
    use super::{has_marker, kind};
    use std::collections::BTreeSet;

    #[test]
    fn names_each_kind() {
        let set = |stages: &[u8]| stages.iter().copied().collect::<BTreeSet<u8>>();
        assert_eq!(kind(&set(&[1, 3])), "deleted by ours, changed by theirs");
        assert_eq!(kind(&set(&[1, 2])), "changed by ours, deleted by theirs");
        assert_eq!(kind(&set(&[2, 3])), "added by both");
    }

    #[test]
    fn finds_markers_only_at_a_line_start() {
        assert!(has_marker("a\n<<<<<<< HEAD\nb\n"));
        assert!(has_marker("||||||| base\n"));
        assert!(!has_marker("a <<<<<<< b\n=======\n"));
    }
}
