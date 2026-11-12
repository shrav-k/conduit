//! Exercise the documented rebase procedure with Git's actual hunk merging.
use conduit_conformance::pieces::{self, Changes, Files, Ordered};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const INPUTS: &str = "/properties/steps/items/properties/input/oneOf";
const EXPECT: &str = "/properties/steps/items/properties/expect/properties";

fn apply(files: &Files, changes: Changes) -> Files {
    let mut result = files.clone();
    for (name, bytes) in changes {
        if let Some(bytes) = bytes {
            result.insert(name, bytes);
        } else {
            result.remove(&name);
        }
    }
    result
}

fn baseline() -> Files {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../conformance");
    let mut files = Files::new();
    files.insert(
        "case.schema.json".into(),
        std::fs::read(root.join("case.schema.json")).unwrap(),
    );
    for directory in ["case-schema/inputs", "case-schema/expect"] {
        for entry in std::fs::read_dir(root.join(directory)).unwrap() {
            let path = entry.unwrap().path();
            let name = path.strip_prefix(&root).unwrap().to_str().unwrap();
            files.insert(name.into(), std::fs::read(&path).unwrap());
        }
    }
    let schema = pieces::assemble(&files).unwrap();
    Files::from([
        (
            "manifest.yaml".into(),
            b"{\"cases\":[],\"s1_coverage\":[]}".to_vec(),
        ),
        (
            "case.schema.json".into(),
            serde_json::to_vec_pretty(&schema).unwrap(),
        ),
    ])
}

struct Stages(PathBuf);
impl Drop for Stages {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

fn rebase(name: &str, edit: impl FnOnce(&mut Value)) {
    let original = baseline();
    let split = apply(
        &original,
        pieces::split(&original, None, &BTreeSet::new()).unwrap(),
    );
    let mut branch: Value = serde_json::from_slice(&original["case.schema.json"]).unwrap();
    edit(&mut branch);
    let directory = std::env::temp_dir().join(format!(
        "conduit-pieces-rebase-{}-{name}",
        std::process::id()
    ));
    std::fs::create_dir(&directory).unwrap();
    let stages = Stages(directory);
    for (name, bytes) in [
        ("stage1", original["case.schema.json"].clone()),
        ("stage2", split["case.schema.json"].clone()),
        ("stage3", serde_json::to_vec_pretty(&branch).unwrap()),
    ] {
        std::fs::write(stages.0.join(name), bytes).unwrap();
    }
    let output = std::process::Command::new("git")
        .args(["merge-file", "--theirs", "stage2", "stage1", "stage3"])
        .current_dir(&stages.0)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let mut incoming = split;
    incoming.insert(
        "case.schema.json".into(),
        std::fs::read(stages.0.join("stage2")).unwrap(),
    );
    let merged_schema = Ordered::parse(&incoming["case.schema.json"])
        .unwrap()
        .value();
    if name == "input-only" {
        assert_eq!(merged_schema.pointer(EXPECT).unwrap(), &json!({}));
    } else {
        assert_eq!(merged_schema.pointer(INPUTS).unwrap(), &json!([false]));
    }
    let merged = apply(
        &incoming,
        pieces::split(&incoming, Some(&original), &BTreeSet::new()).unwrap(),
    );
    let mut assembled = pieces::assemble(&merged).unwrap();
    for schema in [&mut assembled, &mut branch] {
        schema
            .pointer_mut(INPUTS)
            .unwrap()
            .as_array_mut()
            .unwrap()
            .sort_by_key(|value| serde_json::to_vec(value).unwrap());
    }
    assert_eq!(assembled, branch);
    assert!(
        pieces::split(&merged, Some(&original), &BTreeSet::new())
            .unwrap()
            .is_empty()
    );
}

#[test]
fn input_only_rebase_keeps_already_split_expectations() {
    rebase("input-only", |schema| {
        let inputs = schema.pointer_mut(INPUTS).unwrap().as_array_mut().unwrap();
        inputs[0]["description"] = json!("Changed on the rebased branch");
        inputs.push(json!({"properties":{"kind":{"const":"new-input"}}}));
    });
}

#[test]
fn expectation_only_rebase_keeps_already_split_inputs() {
    rebase("expectation-only", |schema| {
        let expect = schema.pointer_mut(EXPECT).unwrap();
        expect["absent"]["description"] = json!("Changed on the rebased branch");
        expect["new-expectation"] = json!({"type":"boolean"});
    });
}
