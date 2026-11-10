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
fn init_inspect_and_export() {
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
}

#[test]
fn real_npm_register_completes_setup_and_ends_runs_on_exit() {
    if std::env::var_os("CONDUIT_TEST_NPM").is_none() {
        return;
    }
    let parent = tempfile::tempdir().unwrap();
    let store = parent.path().join("store");
    command(&store, "init", &[]);
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../conformance/fixtures/s1/project");
    let project = parent.path().join("project");
    std::fs::create_dir(&project).unwrap();
    for name in ["shared", "history"] {
        std::fs::create_dir(parent.path().join(name)).unwrap();
    }
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let text = std::fs::read_to_string(entry.path())
            .unwrap()
            .replace(
                "/tmp/conduit-s1/shared",
                parent.path().join("shared").to_str().unwrap(),
            )
            .replace(
                "/tmp/conduit-s1/history",
                parent.path().join("history").to_str().unwrap(),
            );
        std::fs::write(project.join(entry.file_name()), text).unwrap();
    }
    let registration = command(&store, "register", &[project.to_str().unwrap()]);
    assert!(
        registration.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&registration.stderr)
    );
    let state: Value =
        serde_json::from_slice(&std::fs::read(store.join("state.json")).unwrap()).unwrap();
    for name in ["workspace", "shared", "history", "agent-a", "agent-b"] {
        assert!(
            state["state"]["objects"]
                .as_array()
                .unwrap()
                .iter()
                .any(|o| o["name"] == name && o["state"] == "ready")
        );
    }
    assert!(
        state["state"]["invocations"]
            .as_array()
            .unwrap()
            .iter()
            .all(|v| v["state"] != "in-progress")
    );
    assert!(
        !String::from_utf8(registration.stdout)
            .unwrap()
            .contains("\"pending\"")
    );
}
