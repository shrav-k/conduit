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
    files.extend(pieces::read_schema_pieces(&root).unwrap());
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

#[test]
fn one_walk_reads_pieces_and_refuses_other_files() {
    let directory =
        std::env::temp_dir().join(format!("conduit-pieces-walk-{}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    let stages = Stages(directory);
    assert!(pieces::read_schema_pieces(&stages.0).unwrap().is_empty());
    for folder in [
        "case-schema/inputs",
        "case-schema/.hidden",
        "case-schema/rules/sub",
    ] {
        std::fs::create_dir_all(stages.0.join(folder)).unwrap();
    }
    let piece = "case-schema/inputs/load.json";
    let bytes = br#"{"properties":{"kind":{"const":"load"}}}"#;
    std::fs::write(stages.0.join(piece), bytes).unwrap();
    std::fs::write(stages.0.join("case-schema/inputs/.DS_Store"), b"hidden").unwrap();
    std::fs::write(stages.0.join("case-schema/.hidden/notes.txt"), b"hidden").unwrap();
    assert_eq!(
        pieces::read_schema_pieces(&stages.0).unwrap(),
        Files::from([(piece.into(), bytes.to_vec())])
    );
    for bad in [
        "case-schema/inputs/load.json.orig",
        "case-schema/rules/sub/a.json",
        "case-schema/README.md",
    ] {
        let path = stages.0.join(bad);
        std::fs::write(&path, b"{}").unwrap();
        let error = pieces::read_schema_pieces(&stages.0).unwrap_err();
        assert!(error.contains(bad), "{error}");
        assert!(error.contains("rename or move it"), "{error}");
        std::fs::remove_file(path).unwrap();
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
    if name == "remove-all-expectations" {
        assert_eq!(merged_schema.pointer(EXPECT).unwrap(), &json!({}));
        let before = incoming.clone();
        let error = pieces::split(&incoming, Some(&original), &BTreeSet::new()).unwrap_err();
        assert!(error.contains("schema element removed inline"), "{error}");
        assert_eq!(incoming, before);
        return;
    }
    if name == "input-only" {
        assert_eq!(
            merged_schema.pointer(EXPECT).unwrap(),
            &json!({"$pieces":false})
        );
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

#[test]
fn rebase_rejects_removing_all_expectations_while_changing_input() {
    rebase("remove-all-expectations", |schema| {
        schema.pointer_mut(INPUTS).unwrap()[0]["description"] =
            json!("Changed on the rebased branch");
        schema
            .pointer_mut(EXPECT)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .clear();
    });
}
