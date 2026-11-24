use super::*;

fn list(shell: &Shell, key: &str) -> Value {
    let mut input = invoke(shell, "root", "management.list.v1", Some(key));
    input["request"]["arguments"] = json!({});
    shell.submit(input, &MissingExecutor).unwrap()
}

fn first_started(path: &Path) -> Shell {
    let shell = open(path);
    shell
        .submit(
            json!({"kind":"first-start","machine":local_machine(BTreeMap::new()).unwrap()}),
            &MissingExecutor,
        )
        .unwrap();
    shell
}

fn last_measure(path: &Path) -> Value {
    serde_json::from_str(
        fs::read_to_string(path.join("commit-measures.jsonl"))
            .unwrap()
            .lines()
            .last()
            .unwrap(),
    )
    .unwrap()
}

fn publication_target(shell: &Shell, object: &Value) -> Value {
    let mut target = object.clone();
    if object.get("held").is_some() {
        target["schema_environment"] = object["held"]["environment"].clone();
        target["contract_scope"] = object["held"]["scope"].clone();
    } else if target.get("schema_environment").is_none() {
        target["schema_environment"] = json!(shell.context.core_environment);
        target["contract_scope"] = json!("core");
    }
    if object.get("held").is_none() {
        let name = target["type"]
            .as_str()
            .unwrap()
            .split_once('#')
            .unwrap()
            .1
            .to_owned();
        let required =
            target["schema_environment"]["scopes"]["core"]["contracts"][name]["spec"]["requires"]
                .as_array()
                .cloned()
                .unwrap_or_default();
        for operation in required {
            let operation = operation.as_str().unwrap();
            if target["resolved_implementations"].get(operation).is_none() {
                target["resolved_implementations"][operation] = json!({"source":"engine"});
            }
        }
    }
    target
}

fn check_ended(shell: &Shell) {
    let state = shell.state().unwrap();
    for v in &state.invocations {
        if v["state"] == "in-progress" || v["state"] == "unknown" {
            continue;
        }
        assert!(
            v.get("implementation").is_none(),
            "ended implementation remains"
        );
        let target = v.get("held_target").cloned().unwrap_or_else(|| {
            publication_target(
                shell,
                state
                    .objects
                    .iter()
                    .find(|o| o["id"] == v["target"])
                    .unwrap(),
            )
        });
        let keeps = conduit_core::requests::held_record_content(
            &target,
            v["operation"].as_str().unwrap(),
            &shell.context,
        )
        .is_none_or(|flags| flags.into_iter().any(|flag| flag));
        assert_eq!(
            v.get("held_target").is_some(),
            keeps,
            "ended held_target policy"
        );
    }
}

#[test]
fn held_copy_store_measure() {
    // DL2: only counts and byte lengths leave this temporary store.
    let temp = temporary();
    let shell = first_started(temp.path());
    for i in 0..20 {
        assert_eq!(list(&shell, &format!("list-{i}"))["isError"], false);
    }
    let line = last_measure(temp.path());
    eprintln!(
        "held-copy measure: invocations={} stateBytes={}",
        line["invocations"], line["stateBytes"]
    );
    check_ended(&shell);
}

#[test]
fn held_copy_compacted_when_a_store_opens() {
    // DL2, IMP-007: emulate version 1 from an earlier engine, in this test's store alone.
    let temp = temporary();
    let shell = first_started(temp.path());
    assert_eq!(list(&shell, "key")["isError"], false);
    let repeated = list(&shell, "key");
    let before_data = shell
        .state()
        .unwrap()
        .records
        .iter()
        .rev()
        .find(|r| r["type"] == "request-ended")
        .unwrap()["data"]
        .clone();
    let id = repeated["_meta"][format!("{}invocation", shell.context.meta_prefix)].clone();
    let held = {
        let state = shell.state().unwrap();
        let v = state.invocations.iter().find(|v| v["id"] == id).unwrap();
        publication_target(
            &shell,
            state
                .objects
                .iter()
                .find(|o| o["id"] == v["target"])
                .unwrap(),
        )
    };
    drop(shell);
    let path = temp.path().join("state.json");
    let mut disk: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let v = disk["state"]["invocations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|v| v["id"] == id)
        .unwrap();
    v["held_target"] = held;
    v["implementation"] = json!({"source":"engine"});
    fs::write(&path, serde_json::to_vec(&disk).unwrap()).unwrap();
    let size = fs::metadata(&path).unwrap().len();
    let shell = open(temp.path());
    assert_memory_equals_disk(&shell);
    let entries = shell.state().unwrap().invocations.clone();
    let v = entries.iter().find(|v| v["id"] == id).unwrap();
    assert!(
        v.get("implementation").is_none(),
        "open kept implementation"
    );
    assert!(v.get("held_target").is_none());
    let measure = last_measure(temp.path());
    assert_eq!(measure["input"], "recover");
    assert!(measure["stateBytes"].as_u64().unwrap() < size);
    drop(shell);
    let shell = open(temp.path());
    assert_eq!(shell.state().unwrap().invocations, entries);
    assert_eq!(list(&shell, "key"), repeated);
    let after_data = shell
        .state()
        .unwrap()
        .records
        .iter()
        .rev()
        .find(|r| r["type"] == "request-ended")
        .unwrap()["data"]
        .clone();
    assert_eq!(after_data, before_data);
    assert_memory_equals_disk(&shell);
}

#[test]
fn held_copy_dropped_on_every_end_path() {
    // DL2, REC-002: register-project, open.v1 and set-up.v1 all occur in S1.
    let temp = temporary();
    let shell = established(temp.path());
    let state = shell.state().unwrap();
    assert!(
        state
            .invocations
            .iter()
            .any(|v| v["boundary"] == "register-project")
    );
    for name in ["open.v1", "set-up.v1"] {
        assert!(state.invocations.iter().any(|v| v["operation"]
            == format!("{}#{name}", shell.context.core_package)
            && v["state"] != "in-progress"
            && v["state"] != "unknown"));
    }
    check_ended(&shell);
}
