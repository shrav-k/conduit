//! The coordinator's Rust tooling (DEC-174): the binary `wo` and its merge queue, `wo queue`.
//!
//! The queue merges each work order's branch in turn: it rebases the branch on the commit
//! `REMOTE/BASE` names when the entry starts, settles only the conflicts it is allowed to,
//! squashes, runs the local gate, pushes, merges the pull request and checks what landed.
//! Its state lives in two JSON-lines files under the configured `state` directory.

pub mod cli;
pub mod config;
pub mod entry;
pub mod git;
pub mod lock;
pub mod message;
pub mod order_text;
pub mod process;
pub mod rebase;
pub mod run;
pub mod store;
pub mod time;
pub mod worktree;

/// Exit status when every entry run merged or none was pending.
pub const EXIT_OK: i32 = 0;
/// Exit status when an entry failed or was skipped.
pub const EXIT_FAILED: i32 = 1;
/// Exit status for a configuration or usage error, or a refusal.
pub const EXIT_USAGE: i32 = 2;
/// Exit status when a lock is held elsewhere under `--no-wait`.
pub const EXIT_LOCKED: i32 = 3;

/// Cuts `text` to at most `limit` characters, on a character boundary.
pub fn cut(text: &str, limit: usize) -> String {
    match text.char_indices().nth(limit) {
        Some((index, _)) => text[..index].to_string(),
        None => text.to_string(),
    }
}

/// The last line of `text` that is not blank, or an empty string.
pub fn last_line(text: &str) -> String {
    text.lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .trim_end()
        .to_string()
}
