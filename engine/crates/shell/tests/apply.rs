mod package_target;
mod plan_apply_support;

use conduit_shell::boundary;
use plan_apply_support::cli::*;
use plan_apply_support::{Engine, Symbols, cases, replay_while, result, sha256};
use serde_json::{Value, json};
use std::{fs, path::Path, process::Output};

fn apply(engine: &Engine, directory: &Path, config: &Path) -> Output {
    tokio::task::block_in_place(|| tool(engine, directory, config, &["apply"]))
}

fn clean_local_state(engine: &Engine, directory: &Path) {
    let kept = directory.join(".conduit");
    if kept.exists() {
        for bytes in files(&kept).values() {
            let text = String::from_utf8_lossy(bytes);
            assert!(!text.contains(directory.to_str().unwrap()));
            assert!(!text.contains(engine.directory().to_str().unwrap()));
        }
    }
}

fn non_read_requests(engine: &Engine) -> Vec<(Value, String)> {
    let root = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            result(
                &engine
                    .call(
                        "management.inspect.v1",
                        json!({"object":{"kind":"Interface","name":"root"}}),
                        None,
                    )
                    .await,
            )
        })
    });
    root["records"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|record| {
            let operation = record["data"]["operation"].as_str()?;
            (record["type"] == "invocation-dispatched"
                && !operation.ends_with("#list.v1")
                && !operation.ends_with("#inspect.v1"))
            .then(|| (record["id"].clone(), operation.to_owned()))
        })
        .collect()
}

fn applied_case(engine: &Engine, directory: &Path, config: &Path, case: &Value, symbols: &Symbols) {
    let kept = directory.join(".conduit");
    let before = kept.exists().then(|| files(&kept));
    let before_requests = case["recorded"]["path"]
        .is_null()
        .then(|| non_read_requests(engine));
    let output = apply(engine, directory, config);
    assert_eq!(
        output.status.code().map(i64::from),
        case["exit"].as_i64(),
        "apply status for {}",
        case["id"]
    );
    let expected_plan = symbols.plan_text(case["recorded"]["text"].as_str().unwrap());
    let Some(tail) = stdin_free(&output).strip_prefix(&expected_plan) else {
        panic!("printed plan differs for {}", case["id"]);
    };
    if case["recorded"]["path"].is_null() {
        let expected = case["printed"]
            .as_str()
            .unwrap()
            .strip_prefix(case["recorded"]["text"].as_str().unwrap())
            .unwrap();
        assert_eq!(tail, expected);
        assert_eq!(kept.exists().then(|| files(&kept)), before);
        assert_eq!(Some(non_read_requests(engine)), before_requests);
        clean_local_state(engine, directory);
        return;
    }
    let digest = sha256(&expected_plan);
    assert_eq!(
        fs::read_to_string(kept.join(format!("plans/{digest}.txt"))).unwrap(),
        expected_plan
    );
    let requests = case["requests"].as_array().unwrap();
    let results: Vec<_> = tail
        .lines()
        .filter(|line| line.starts_with("result "))
        .collect();
    assert_eq!(results.len(), requests.len());
    for (index, (line, request)) in results.iter().zip(requests).enumerate() {
        let number = index + 1;
        let outcome = request["result"]["outcome"].as_str().unwrap();
        let prefix = format!("result {number} {outcome} ");
        let json: Value =
            serde_json::from_str(line.strip_prefix(&prefix).expect("ordered result")).unwrap();
        assert_eq!(json["outcome"], outcome);
        for member in ["diagnostics", "pendingAction", "objects", "tombstones"] {
            if let Some(expected) = request["result"].get(member) {
                assert_eq!(
                    json[member].as_array().map(Vec::len),
                    expected.as_array().map(Vec::len)
                );
            }
        }
    }
    assert_eq!(tail.lines().last(), results.last().copied());
    let journal = boundary::value(&kept.join(format!("results/{digest}.yaml"))).unwrap();
    let entries = journal["entries"].as_array().unwrap();
    assert_eq!(entries.len(), requests.len());
    for (index, (entry, request)) in entries.iter().zip(requests).enumerate() {
        assert_eq!(entry["tool"], request["tool"]);
        assert_eq!(entry["key"], format!("plan:{digest}:1:{}", index + 1));
        let printed: Value =
            serde_json::from_str(results[index].splitn(4, ' ').nth(3).unwrap()).unwrap();
        assert_eq!(entry["result"], printed);
    }
    let state = boundary::value(&kept.join("state.yaml")).unwrap();
    assert_eq!(state["files"], case["kept"]["files"]);
    assert_eq!(
        state["objects"].as_array().unwrap().len(),
        case["kept"]["objects"].as_array().unwrap().len()
    );
    assert_eq!(
        state["plans"].as_array().unwrap().last().unwrap(),
        &format!("sha256:{digest}")
    );
    plan_apply_support::check_apply_case(engine, case, symbols, entries, &state, &digest);
    clean_local_state(engine, directory);
}

async fn later_case(index: usize) {
    let cases = cases();
    let target = &cases[index];
    let replay = replay_while(&cases[..=index], |engine, case, symbols| {
        if case["id"] != target["id"] {
            return true;
        }
        let config = tempfile::tempdir().unwrap();
        let directory = tempfile::tempdir().unwrap();
        project_fixture(directory.path(), case, Some(symbols));
        cache_fixture(
            config.path(),
            case["id"].as_str().unwrap()[..2].parse().unwrap(),
        );
        applied_case(engine, directory.path(), config.path(), case, symbols);
        false
    })
    .await;
    assert!(replay.lines.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn apply_case_01_t1() {
    let (engine, config, directory, case) = first_case().await;
    applied_case(
        &engine,
        directory.path(),
        config.path(),
        &case,
        &Symbols::default(),
    );
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn apply_case_02_t2() {
    later_case(1).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn apply_case_03_t3_noop() {
    later_case(2).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn apply_case_07_t7_revoke() {
    later_case(6).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn apply_case_09_t7_restore_unflagged() {
    later_case(7).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn apply_resumes_after_a_stopped_step() {
    let (engine, config, directory, case) = first_case().await;
    // The fixture fails the first setup run on request.
    engine.fail_next_setup();
    let first = apply(&engine, directory.path(), config.path());
    assert_eq!(first.status.code(), Some(1));
    assert!(stdin_free(&first).contains("result 5 failed"));
    assert!(stdin_free(&first).contains("not sent 6..15"));
    let resumed = apply(&engine, directory.path(), config.path());
    assert_eq!(resumed.status.code(), Some(0));
    assert!(!stdin_free(&resumed).contains("management.register.v1"));
    let kept = boundary::value(&directory.path().join(".conduit/state.yaml")).unwrap();
    assert_eq!(kept["plans"].as_array().unwrap().len(), 2);
    assert_eq!(kept["files"], case["kept"]["files"]);
    assert_eq!(
        kept["objects"].as_array().unwrap().len(),
        case["kept"]["objects"].as_array().unwrap().len()
    );
    assert!(
        kept["objects"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row[4] == "template" || row[4] == "ready")
    );
    let settled = apply(&engine, directory.path(), config.path());
    assert_eq!(settled.status.code(), Some(0));
    assert_eq!(stdin_free(&settled), "plan\nno changes\n");
    clean_local_state(&engine, directory.path());
    engine.close().await;
}

// K3 safety case 17: sign-in is left to the panel.
#[tokio::test(flavor = "multi_thread")]
async fn apply_prints_pending_sign_ins_and_answers_none() {
    let (engine, config, directory, _) = first_case().await;
    let applied = apply(&engine, directory.path(), config.path());
    assert_eq!(applied.status.code(), Some(0));
    let lines: Vec<_> = stdin_free(&applied).lines().collect();
    let actions: Vec<_> = lines
        .windows(2)
        .filter(|pair| pair[1].starts_with("panel action "))
        .collect();
    assert_eq!(actions.len(), 2);
    let pending = tokio::task::block_in_place(|| {
        tool(&engine, directory.path(), config.path(), &["pending"])
    });
    assert!(pending.status.success());
    let mut shown = std::collections::BTreeSet::new();
    for pair in actions {
        assert!(pair[0].starts_with("result ") && pair[0].contains("pendingAction"));
        assert!(pair[1].ends_with(": sign-in required"));
        let id = pair[1]
            .strip_prefix("panel action ")
            .unwrap()
            .split(':')
            .next()
            .unwrap();
        assert!(shown.insert(id));
        assert!(stdin_free(&pending).contains(id));
    }
    let inspected = result(
        &engine
            .call(
                "s1.inspect.v1",
                json!({"object":{"kind":"Agent","name":"agent-a"}}),
                None,
            )
            .await,
    );
    assert_eq!(inspected["status"]["pending"][0]["kind"], "sign-in");
    assert!(!non_read_requests(&engine).iter().any(|(_, operation)| {
        operation.ends_with("#sign-in.v1") || operation.ends_with("#answer.v1")
    }));
    let again = apply(&engine, directory.path(), config.path());
    assert_eq!(again.status.code(), Some(0));
    assert_eq!(stdin_free(&again), "plan\nno changes\n");
    clean_local_state(&engine, directory.path());
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn apply_and_plan_stop_on_another_engine_version() {
    let (engine, config, directory, _) = first_case().await;
    for version in [env!("CARGO_PKG_VERSION"), "0.0.0+other"] {
        for command in ["apply", "plan"] {
            check_foreign_engine(&engine, directory.path(), config.path(), version, command).await;
        }
    }
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn apply_sends_nothing_when_its_state_cannot_be_kept() {
    let (engine, config, directory, case) = first_case().await;
    let kept = directory.path().join(".conduit");
    fs::create_dir(&kept).unwrap();
    fs::write(kept.join("plans"), "").unwrap();
    let before_requests = non_read_requests(&engine);
    let output = apply(&engine, directory.path(), config.path());
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        stdin_free(&output),
        format!(
            "{}not sent 1..15\n",
            case["recorded"]["text"].as_str().unwrap()
        )
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains(".conduit/plans"));
    assert_eq!(non_read_requests(&engine), before_requests);
    let listed = result(&engine.call("management.list.v1", json!({}), None).await);
    assert!(
        !listed["objects"]
            .as_array()
            .unwrap()
            .iter()
            .any(|row| row["name"] == "s1")
    );
    clean_local_state(&engine, directory.path());
    engine.close().await;
}
