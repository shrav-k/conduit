//! The fragment step (order 401b, decision 2; DEC-177).
//!
//! The items that the branch's `spec/CHANGES.md` holds under `## [Unreleased]` and PINNED's
//! lacks move, word for word and in order, to the end of `spec/changes/FRAGMENT`. An item is a
//! line that begins `- ` and its continuation lines, indented two spaces. A `#` or `##` heading
//! begins a section; a deeper one, such as `### Added`, stays within its section. FRAGMENT is
//! the file name of the entry's order text, or for an entry with none, the branch's name with
//! `-` for `/`, then `.md`, as `spec/changes/README.md` names fragments. `spec/CHANGES.md` loses
//! only the moved lines.

use std::collections::BTreeMap;

/// The heading the moved items sit under.
pub const UNRELEASED: &str = "## [Unreleased]";
/// The subject of the commit the fragment step makes.
pub const SUBJECT: &str = "wip: changelog item moved to its fragment (DEC-177)";

/// One item: the index of its first line and its lines, each with its line end.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    first: usize,
    lines: Vec<String>,
    under_unreleased: bool,
}

impl Item {
    /// The item's words, by which PINNED's lacking it is judged.
    fn words(&self) -> String {
        self.lines
            .iter()
            .flat_map(|line| line.split_whitespace())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn items(text: &str) -> Vec<Item> {
    let mut found: Vec<Item> = Vec::new();
    let mut section = String::new();
    let mut open = false;
    for (index, raw) in text.split_inclusive('\n').enumerate() {
        let line = raw.trim_end_matches(['\n', '\r']);
        if line.starts_with("# ") || line.starts_with("## ") {
            section = line.trim_end().to_string();
            open = false;
        } else if line.starts_with("- ") {
            found.push(Item {
                first: index,
                lines: vec![raw.to_string()],
                under_unreleased: section == UNRELEASED,
            });
            open = true;
        } else if open && line.starts_with("  ") && !line.trim().is_empty() {
            found
                .last_mut()
                .expect("an item is open")
                .lines
                .push(raw.to_string());
        } else {
            open = false;
        }
    }
    found
}

/// What the fragment step writes: `spec/CHANGES.md` less the moved lines, and those lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Moved {
    pub changes: String,
    pub fragment: String,
    pub count: usize,
}

/// The items `head`'s `## [Unreleased]` holds and `pinned` lacks, anywhere in it; `None` when
/// there are none.
pub fn split(pinned: &str, head: &str) -> Option<Moved> {
    let mut held: BTreeMap<String, usize> = BTreeMap::new();
    for item in items(pinned) {
        *held.entry(item.words()).or_default() += 1;
    }
    let mut moved = Vec::new();
    for item in items(head).into_iter().filter(|item| item.under_unreleased) {
        match held.get_mut(&item.words()) {
            Some(count) if *count > 0 => *count -= 1,
            _ => moved.push(item),
        }
    }
    if moved.is_empty() {
        return None;
    }
    let mut taken = vec![false; head.split_inclusive('\n').count()];
    let mut fragment = String::new();
    for item in &moved {
        for (offset, line) in item.lines.iter().enumerate() {
            taken[item.first + offset] = true;
            fragment.push_str(line);
            if !line.ends_with('\n') {
                fragment.push('\n');
            }
        }
    }
    let changes = head
        .split_inclusive('\n')
        .zip(&taken)
        .filter(|(_, taken)| !**taken)
        .map(|(line, _)| line)
        .collect();
    Some(Moved {
        changes,
        fragment,
        count: moved.len(),
    })
}

/// Whether `name` is a fragment's file name: a lowercase letter or digit, then lowercase
/// letters, digits, `.` and `-`, then `.md`.
pub fn is_fragment_name(name: &str) -> bool {
    let Some(stem) = name.strip_suffix(".md") else {
        return false;
    };
    let lower_or_digit = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit();
    stem.starts_with(lower_or_digit) && stem.chars().all(|c| lower_or_digit(c) || ".-".contains(c))
}

/// FRAGMENT: the file name of the entry's order text, or for an entry with none, the branch's
/// name with `-` for `/`, then `.md`. A name that is not a fragment's fails.
pub fn fragment_name(branch: &str, order_text: Option<&str>) -> Result<String, String> {
    let (name, from) = match order_text {
        Some(path) => (
            path.rsplit('/').next().unwrap_or(path).to_string(),
            format!("the order text {path}"),
        ),
        None => (
            format!("{}.md", branch.replace('/', "-")),
            format!("the branch {branch}"),
        ),
    };
    if is_fragment_name(&name) {
        Ok(name)
    } else {
        Err(format!(
            "{name}, named after {from}, is not a fragment's name: a lowercase letter or digit, then lowercase letters, digits, . and -, then .md"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{fragment_name, is_fragment_name, split};

    const PINNED: &str = "# Changes\n\n## [Unreleased]\n\n- Added: old (DEC-1).\n- Changed: main's,\n  wrapped (DEC-2).\n\n## [0.1.0]\n\n- Added: released (DEC-0).\n";

    #[test]
    fn moves_only_the_items_pinned_lacks() {
        assert_eq!(split(PINNED, PINNED), None);
        let head = PINNED.replace(
            "  wrapped (DEC-2).\n",
            "  wrapped (DEC-2).\n- Fixed: the branch's,\n  in two lines (DEC-3).\n- Added: released (DEC-0).\n",
        );
        let moved = split(PINNED, &head).unwrap();
        assert_eq!(moved.count, 1, "an item PINNED holds in a release stays");
        assert_eq!(
            moved.fragment,
            "- Fixed: the branch's,\n  in two lines (DEC-3).\n"
        );
        assert_eq!(
            moved.changes,
            PINNED.replace(
                "  wrapped (DEC-2).\n",
                "  wrapped (DEC-2).\n- Added: released (DEC-0).\n"
            )
        );
        let released = PINNED.replace(
            "- Added: released (DEC-0).\n",
            "- Added: released (DEC-0).\n- Added: in a release.\n",
        );
        assert_eq!(split(PINNED, &released), None, "only Unreleased items move");
        let sub = "## [Unreleased]\n\n### Added\n\n- Added: old (DEC-1).\n";
        let sub_head = format!("{sub}- Added: under a subheading (DEC-3).\n\n## [0.1.0]\n");
        assert_eq!(
            split(&format!("{sub}\n## [0.1.0]\n"), &sub_head).map(|moved| moved.fragment),
            Some("- Added: under a subheading (DEC-3).\n".to_string()),
            "a ### heading stays within its ## section"
        );
    }

    #[test]
    fn names_a_fragment() {
        assert_eq!(
            fragment_name("wo/602-branch", Some("docs/work-orders/602-held-text.md")),
            Ok("602-held-text.md".to_string())
        );
        assert_eq!(
            fragment_name("frag/other", None),
            Ok("frag-other.md".to_string())
        );
        assert!(
            fragment_name("Frag/Upper", None)
                .unwrap_err()
                .contains("is not a fragment's name")
        );
        assert!(!is_fragment_name("README.md"));
        assert!(!is_fragment_name(".md"));
        assert!(!is_fragment_name("-x.md"));
        assert!(is_fragment_name("401b-queue.v2.md"));
    }
}
