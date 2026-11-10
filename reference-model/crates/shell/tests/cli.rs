use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};

fn command(store: &Path, verb: &str, extra: &[&str]) -> Output {
    let output = Command::new(env!("CARGO_BIN_EXE_conduit"))
        .arg(verb)
        .arg("--store")
        .arg(store)
        .args(extra)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn init_register_pause_inspect_and_export() {
    let parent = tempfile::tempdir().unwrap();
    let store = parent.path().join("store");
    let first = command(&store, "init", &[]);
    assert_eq!(
        serde_json::from_slice::<Value>(&first.stdout).unwrap()["started"],
        true
    );
    let state: Value =
        serde_json::from_slice(&std::fs::read(store.join("state.json")).unwrap()).unwrap();
    let ids: Vec<_> = state["state"]["objects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["id"].clone())
        .collect();
    let repeated = command(&store, "init", &[]);
    assert_eq!(
        serde_json::from_slice::<Value>(&repeated.stdout).unwrap()["started"],
        false
    );
    let project =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../conformance/fixtures/s1/project");
    let registration = command(&store, "register", &[project.to_str().unwrap()]);
    let diagnostic = String::from_utf8(registration.stderr).unwrap();
    assert!(
        diagnostic.contains("waiting for runner intent"),
        "{diagnostic}"
    );
    assert!(diagnostic.contains("inspect-run.v1"));
    assert!(
        !String::from_utf8(registration.stdout)
            .unwrap()
            .contains("\"pending\"")
    );
    let inspection = command(&store, "inspect", &[]);
    let inspection: String = String::from_utf8(inspection.stdout).unwrap();
    let lines: Vec<Value> = inspection
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert!(
        lines
            .iter()
            .any(|v| v["object"]["name"] == "root" && v["result"]["isError"] == false)
    );
    let records = command(&store, "records", &[]);
    let exported: Vec<Value> = String::from_utf8(records.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let after: Value =
        serde_json::from_slice(&std::fs::read(store.join("state.json")).unwrap()).unwrap();
    assert_eq!(
        exported,
        after["state"]["records"].as_array().unwrap().clone()
    );
    assert_eq!(
        ids,
        after["state"]["objects"].as_array().unwrap()[..7]
            .iter()
            .map(|v| v["id"].clone())
            .collect::<Vec<_>>()
    );
    assert!(
        exported
            .iter()
            .any(|v| v["type"] == "invocation-ended" && v["data"]["outcome"] == "unknown")
    );
}
