//! Every rule the loader's rejections rest on is a promise that the specification defines,
//! written as a diagnostic's `ruleId` is: `diagnostic.v1` holds it to the pattern
//! `^[A-Z]+-[0-9]{3}$`, so an item of the implementation-defined list, such as DL10, is not one.

mod common;

use std::collections::BTreeSet;

use common::spec_dir;
use conduit_loader::rules;

/// The IDs written as `- id: X` in the YAML files directly under `spec/` and one level below.
fn defined() -> BTreeSet<String> {
    let mut files = vec![spec_dir().join("implementation-defined.yaml")];
    for entry in std::fs::read_dir(spec_dir()).expect("spec/") {
        let path = entry.expect("an entry").path();
        if path.is_dir() {
            for file in std::fs::read_dir(&path).expect("a directory") {
                files.push(file.expect("an entry").path());
            }
        }
    }
    files
        .iter()
        .filter(|path| path.extension().is_some_and(|e| e == "yaml"))
        .flat_map(|path| {
            let text = std::fs::read_to_string(path).expect("a file");
            text.lines()
                .filter_map(|line| line.strip_prefix("- id: "))
                .map(|id| id.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn every_rule_cited_is_defined() {
    let defined = defined();
    assert!(
        defined.contains("SCH-001") && defined.contains("DL1"),
        "the IDs were read"
    );
    for rule in rules::ALL {
        assert!(
            defined.contains(*rule),
            "{rule} is not a promise or an item of spec/"
        );
        let (prefix, number) = rule.split_once('-').unwrap_or_default();
        assert!(
            !prefix.is_empty()
                && prefix.bytes().all(|b| b.is_ascii_uppercase())
                && number.len() == 3
                && number.bytes().all(|b| b.is_ascii_digit()),
            "{rule} is not in the form of a promise's ID, which a `ruleId` takes"
        );
    }
}
