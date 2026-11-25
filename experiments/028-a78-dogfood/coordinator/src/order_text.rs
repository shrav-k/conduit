//! An entry's order text and the Record rule.
//!
//! For a branch `wo/ID-...` whose ID, up to the first `-`, begins with a digit, the order text
//! is the one file `docs/work-orders/ID-*.md` at the branch's head; another branch has none.
//! The Record rule: that file exists and its last `## Record` section is not exactly
//! `Not yet run.`. Texts written ahead are not checked.

use crate::git::Git;

/// The directory order texts live in.
pub const ORDERS: &str = "docs/work-orders/";

/// The order's ID a branch names, when the branch has an order text.
pub fn order_id(branch: &str) -> Option<&str> {
    let rest = branch.strip_prefix("wo/")?;
    let id = rest.split('-').next().unwrap_or(rest);
    id.starts_with(|c: char| c.is_ascii_digit()).then_some(id)
}

/// The text of the last `## Record` section, trimmed, when the text has one.
pub fn record_section(text: &str) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .rposition(|line| line.trim_end() == "## Record")?;
    let body: Vec<&str> = lines[start + 1..]
        .iter()
        .take_while(|line| !(line.starts_with("# ") || line.starts_with("## ")))
        .copied()
        .collect();
    Some(body.join("\n").trim().to_string())
}

/// Checks the Record rule for `branch` at `revision`.
pub fn check_record(git: &Git, branch: &str, revision: &str) -> Result<String, String> {
    let Some(id) = order_id(branch) else {
        return Ok(format!("{branch} has no order text"));
    };
    let listing = git.text(&["ls-tree", "-z", revision, "--", ORDERS])?;
    let prefix = format!("{ORDERS}{id}-");
    let mut found = Vec::new();
    for item in listing.split('\0').filter(|item| !item.is_empty()) {
        let Some((meta, path)) = item.split_once('\t') else {
            continue;
        };
        if path.starts_with(&prefix) && path.ends_with(".md") && meta.contains(" blob ") {
            found.push(path.to_string());
        }
    }
    let path = match found.as_slice() {
        [] => {
            return Err(format!(
                "{branch} has no order text: no file {prefix}*.md at its head"
            ));
        }
        [path] => path.clone(),
        many => {
            return Err(format!(
                "{branch} has {} order texts, not one: {}",
                many.len(),
                many.join(", ")
            ));
        }
    };
    let text = git.text(&["cat-file", "blob", &format!("{revision}:{path}")])?;
    match record_section(&text) {
        None => Err(format!("{path} has no `## Record` section")),
        Some(record) if record == "Not yet run." => {
            Err(format!("{path}'s Record is `Not yet run.`"))
        }
        Some(_) => Ok(format!("{path} has its Record")),
    }
}

#[cfg(test)]
mod tests {
    use super::{order_id, record_section};

    #[test]
    fn names_the_order_of_a_work_order_branch() {
        assert_eq!(order_id("wo/401-coordinator-merge-queue"), Some("401"));
        assert_eq!(order_id("wo/401b-more"), Some("401b"));
        assert_eq!(order_id("wo/401"), Some("401"));
        assert_eq!(order_id("wo/dec-174"), None);
        assert_eq!(order_id("feature/401-x"), None);
    }

    #[test]
    fn reads_the_last_record_section() {
        let text = "# Order\n\n## Record\n\nNot yet run.\n\n## Other\n\nx\n\n## Record\n\n### Run\n\nDone.\n";
        assert_eq!(record_section(text).unwrap(), "### Run\n\nDone.");
        assert_eq!(
            record_section("# O\n\n## Record\n\nNot yet run.\n").unwrap(),
            "Not yet run."
        );
        assert!(record_section("# O\n").is_none());
    }
}
