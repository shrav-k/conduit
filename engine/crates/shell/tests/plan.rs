mod package_target;
mod plan_apply_support;

use std::{fs, process::Output};

use conduit_shell::boundary;
use plan_apply_support::cli::*;
use plan_apply_support::{Engine, cases, installs, replay_visit, repository, sha256};
use serde_json::{Value, json};
use tempfile::TempDir;

fn assert_plan(output: &Output, expected: &str, code: i32) {
    assert_eq!(
        output.status.code(),
        Some(code),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let actual = stdin_free(output);
    if actual != expected {
        let difference = actual
            .lines()
            .zip(expected.lines())
            .enumerate()
            .find(|(_, (actual, expected))| actual != expected);
        if let Some((n, (got, want))) = difference {
            panic!("plan line {}: expected {want}, observed {got}", n + 1);
        }
        panic!(
            "plan length: expected {}, observed {}",
            expected.lines().count(),
            actual.lines().count()
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_case_01_text_and_digest() {
    let (engine, config, dir, case) = first_case().await;
    let out = tokio::task::block_in_place(|| tool(&engine, dir.path(), config.path(), &["plan"]));
    assert_plan(&out, case["recorded"]["text"].as_str().unwrap(), 0);
    assert_eq!(
        json!(format!("sha256:{}", sha256(stdin_free(&out)))),
        case["recorded"]["digest"]
    );
    assert!(!dir.path().join(".conduit").exists());
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_cases_02_03_07_09_on_replayed_state() {
    let report = replay_visit(&cases(), |engine, case, symbols| {
        let id = case["id"].as_str().unwrap();
        if !["02", "03", "07", "09"].contains(&&id[..2]) {
            return;
        }
        let config = tempfile::tempdir().unwrap();
        let dir = tempfile::tempdir().unwrap();
        project_fixture(dir.path(), case, Some(symbols));
        cache_fixture(config.path(), id[..2].parse().unwrap());
        let out =
            tokio::task::block_in_place(|| tool(engine, dir.path(), config.path(), &["plan"]));
        assert_plan(
            &out,
            &symbols.plan_text(case["recorded"]["text"].as_str().unwrap()),
            if id.starts_with("09") { 2 } else { 0 },
        );
    })
    .await;
    assert!(report.lines.is_empty(), "replay differs");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_reads_only() {
    let fixture = cases();
    let report = replay_visit(&fixture[..7], |engine, case, symbols| {
        if !case["id"].as_str().unwrap().starts_with("07") {
            return;
        }
        let config = tempfile::tempdir().unwrap();
        let dir = tempfile::tempdir().unwrap();
        project_fixture(dir.path(), case, Some(symbols));
        cache_fixture(config.path(), 7);
        let state = engine.directory().join("state.json");
        let before = boundary::value(&state).unwrap()["state"].take();
        let source = files(dir.path());
        let cached = files(config.path());
        let out =
            tokio::task::block_in_place(|| tool(engine, dir.path(), config.path(), &["plan"]));
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(source == files(dir.path()));
        assert!(cached == files(config.path()));
        let after = boundary::value(&state).unwrap()["state"].take();
        assert!(before["objects"] == after["objects"]);
        assert!(before["tombstones"] == after["tombstones"]);
        let root = before["objects"]
            .as_array()
            .unwrap()
            .iter()
            .find(|o| o["kind"] == "Interface" && o["name"] == "root")
            .unwrap();
        let records = after["records"].as_array().unwrap();
        assert!(records.len() > before["records"].as_array().unwrap().len());
        for event in &records[before["records"].as_array().unwrap().len()..] {
            assert!(matches!(
                event["type"].as_str().unwrap(),
                "access-decision" | "invocation-dispatched" | "invocation-ended" | "request-ended"
            ));
            let operation = event["data"]["operation"].as_str().unwrap();
            assert!(operation.ends_with("#list.v1") || operation.ends_with("#inspect.v1"));
            assert_eq!(event["data"]["caller"], root["id"]);
        }
    })
    .await;
    assert!(report.lines.is_empty(), "replay differs");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_stops_on_a_cache_miss() {
    let (engine, config, dir, _) = first_case().await;
    let fixture = installs();
    let archive = &fixture[0];
    let hash = archive.args["digest"]
        .as_str()
        .unwrap()
        .trim_start_matches("sha256:");
    let entry = config.path().join("packages").join(hash);
    fs::remove_file(&entry).unwrap();
    tokio::task::block_in_place(|| {
        let absent = tool(&engine, dir.path(), config.path(), &["plan"]);
        assert_eq!(absent.status.code(), Some(2));
        assert!(stdin_free(&absent).contains("archive=absent requires=conduit-install PKG-004"));
        fs::write(&entry, b"not the archive").unwrap();
        let mismatch = tool(&engine, dir.path(), config.path(), &["plan"]);
        assert_eq!(mismatch.status.code(), Some(2));
        assert!(stdin_free(&mismatch).contains("archive=mismatch PKG-004"));
        let installer = tempfile::tempdir().unwrap();
        assert!(
            tool(
                &engine,
                installer.path(),
                config.path(),
                &["install", &archive_arg(&archive.args)]
            )
            .status
            .success()
        );
        assert!(
            tool(&engine, dir.path(), config.path(), &["plan"])
                .status
                .success()
        );
    });
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_stops_on_another_engine_version() {
    let engine = Engine::start().await;
    let config = tempfile::tempdir().unwrap();
    let dir = tempfile::tempdir().unwrap();
    project_fixture(dir.path(), &cases()[0], None);
    check_foreign_engine(&engine, dir.path(), config.path(), "0.0.0+other", "plan").await;
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_secret_literal_leaves_no_trace() {
    let engine = Engine::start().await;
    let config = tempfile::tempdir().unwrap();
    let dir = tempfile::tempdir().unwrap();
    const SAMPLE: &str = "invalid-reference-example";
    let ctx = boundary::context(&repository()).unwrap();
    let api = ctx.builtin.as_ref().unwrap().manifest.as_ref().unwrap()["apiVersion"].clone();
    let docs = json!([
        {"apiVersion":api,"kind":"Type","metadata":{"name":"config.v1"},"spec":{"family":"Resource","schema":{"type":"object","properties":{"value":{"$ref":format!("{}#secret-reference.v1",ctx.core_package)}}}}},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"example--t"},"spec":{"type":"#config.v1","config":{"value":SAMPLE}}}
    ]);
    fs::write(
        dir.path().join("conduit.json"),
        json!({"apiVersion":api,"address":"example.org/demo","packages":{}}).to_string(),
    )
    .unwrap();
    fs::write(dir.path().join("documents.json"), docs.to_string()).unwrap();
    let state = engine.directory().join("state.json");
    let records_before = boundary::value(&state).unwrap()["state"]["records"].take();
    let out = tokio::task::block_in_place(|| tool(&engine, dir.path(), config.path(), &["plan"]));
    assert!(
        !out.stdout
            .windows(SAMPLE.len())
            .any(|w| w == SAMPLE.as_bytes())
    );
    assert!(
        !out.stderr
            .windows(SAMPLE.len())
            .any(|w| w == SAMPLE.as_bytes())
    );
    assert_plan(
        &out,
        "plan\nstop Resource/example--t place=/spec/config/value SEC-001\nflags []\nstopped\n",
        2,
    );
    let records_after = boundary::value(&state).unwrap()["state"]["records"].take();
    assert!(records_before == records_after, "plan sent engine requests");
    let without_context = binary(dir.path(), config.path())
        .arg("plan")
        .output()
        .unwrap();
    assert_plan(&without_context, stdin_free(&out), 2);
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_b_02_protected_first_start_name_prints_row_six() {
    let engine = Engine::start().await;
    let config = tempfile::tempdir().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let context = boundary::context(&repository()).unwrap();
    let api = &context.builtin.as_ref().unwrap().manifest.as_ref().unwrap()["apiVersion"];
    fs::write(
        dir.path().join("documents.json"),
        json!([{"apiVersion":api,"kind":"Interface","metadata":{"name":"root"},
            "from":format!("{}#root", context.core_package)}])
        .to_string(),
    )
    .unwrap();
    let out = tokio::task::block_in_place(|| tool(&engine, dir.path(), config.path(), &["plan"]));
    assert_plan(
        &out,
        "plan\nstop Interface/root made-by=engine OBJ-002\nflags []\nstopped\n",
        2,
    );
    engine.close().await;
}

async fn ready_reader() -> (Engine, TempDir, TempDir, Value) {
    use plan_apply_support::result;

    let dir = tempfile::tempdir().unwrap();
    let config = tempfile::tempdir().unwrap();
    let context = boundary::context(&repository()).unwrap();
    let core = &context.core_package;
    let api = &context.builtin.as_ref().unwrap().manifest.as_ref().unwrap()["apiVersion"];
    let manifest = json!({"apiVersion":api,"address":"example.org/control","implementations":{
        "reader-implementation":{"command":["fixture-reader"],"implements":["#set-limit.v1"],"plain":true,"tools":{"#set-limit.v1":"set_limit"}}
    }});
    let docs = json!([
        {"apiVersion":api,"kind":"Operation","metadata":{"name":"set-limit.v1"},"spec":{
            "inputSchema":{"type":"object","properties":{"revision":{"$ref":format!("{core}#revision.v1")},"limit":{"type":"integer"}},"required":["revision"]},
            "effect":"Sets the limit.","changes":{"limit":"/limit"}
        }},
        {"apiVersion":api,"kind":"Capability","metadata":{"name":"reader.v1"},"spec":{"operations":["#set-limit.v1"]}},
        {"apiVersion":api,"kind":"Type","metadata":{"name":"reader-type.v1"},"spec":{
            "family":"Resource","offers":["#reader.v1"],"controls":["#set-limit.v1"],
            "schema":{"type":"object","properties":{"limit":{"type":"integer"}}}
        }},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"reader--t","description":"Original description"},"spec":{
            "type":"#reader-type.v1","config":{"limit":1},"implementation":{"#set-limit.v1":"#reader-implementation"}
        }},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"reader","annotations":{"purpose":"original"}},"from":"reader--t","owner":"system"}
    ]);
    let engine =
        Engine::start_with(|shell| {
            let root = shell
                .state()
                .unwrap()
                .objects
                .iter()
                .find(|object| object["kind"] == "Interface" && object["name"] == "root")
                .unwrap()["id"]
                .clone();
            for (tool, arguments) in [
                (
                    "management.register.v1",
                    json!({"manifest":manifest,"documents":docs}),
                ),
                (
                    "system.set-up.v1",
                    json!({"instance":{"kind":"Resource","name":"reader"}}),
                ),
            ] {
                let reply = shell.submit(json!({"kind":"invoke","request":{
                "identity":{"kind":"instance","id":root},"tool":tool,"arguments":arguments
            }}), &ControlFixture).unwrap();
                let view = result(&reply);
                assert_eq!(
                    view["outcome"], "succeeded",
                    "{tool}: {}",
                    view["diagnostics"]
                );
            }
        })
        .await;

    let listed = result(&engine.call("management.list.v1", json!({}), None).await);
    assert!(
        listed["objects"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["kind"] == "Resource"
                && item["name"] == "reader"
                && item["state"] == "ready")
    );

    let mut objects = Vec::new();
    for (name, tool, state) in [
        ("reader--t", "management.inspect.v1", "template"),
        ("reader", "system.inspect.v1", "ready"),
    ] {
        let view = result(
            &engine
                .call(
                    tool,
                    json!({"object":{"kind":"Resource","name":name}}),
                    None,
                )
                .await,
        );
        assert_eq!(view["outcome"], "succeeded");
        let definition = if state == "template" {
            let mut spec = view["declared"]["spec"].clone();
            spec["description"] = json!("Original description");
            spec
        } else {
            json!({"from":"reader--t","owner":"system","annotations":{"purpose":"original"},"resolved":{"config":{"limit":1}}})
        };
        objects.push(json!([
            "Resource",
            name,
            view["id"],
            view["revision"],
            state,
            definition,
            view["packages"]
        ]));
    }
    fs::write(dir.path().join("conduit.json"), manifest.to_string()).unwrap();
    fs::create_dir(dir.path().join(".conduit")).unwrap();
    fs::write(
        dir.path().join(".conduit/state.yaml"),
        json!({"format":1,"files":{},"objects":objects,"tombstones":[],"plans":[],"journals":{}})
            .to_string(),
    )
    .unwrap();
    (engine, config, dir, docs)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_b_03_description_only_template_edit() {
    let (engine, config, dir, mut docs) = ready_reader().await;
    docs[3]["metadata"]["description"] = json!("Updated description");
    fs::write(dir.path().join("documents.json"), docs.to_string()).unwrap();
    let out = tokio::task::block_in_place(|| tool(&engine, dir.path(), config.path(), &["plan"]));
    assert!(out.status.success());
    assert!(
        stdin_free(&out)
            .lines()
            .any(|line| line.starts_with("1 management.replace.v1 Resource/reader--t ")),
        "description edit omitted the template replacement"
    );
    assert!(!stdin_free(&out).contains("stop Resource/reader "));
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_b_03_annotation_only_instance_edit() {
    let (engine, config, dir, mut docs) = ready_reader().await;
    docs[4]["metadata"]["annotations"]["purpose"] = json!("updated");
    fs::write(dir.path().join("documents.json"), docs.to_string()).unwrap();
    let out = tokio::task::block_in_place(|| tool(&engine, dir.path(), config.path(), &["plan"]));
    assert_plan(
        &out,
        "plan\nstop Resource/reader place=/annotations/purpose requires=--replace reader OBJ-011\nflags []\nstopped\n",
        2,
    );
    engine.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_b_04_ready_instance_names_control_for_changed_setting() {
    let (engine, config, dir, mut docs) = ready_reader().await;
    docs[3]["spec"]["config"]["limit"] = json!(2);
    fs::write(dir.path().join("documents.json"), docs.to_string()).unwrap();
    let out = tokio::task::block_in_place(|| tool(&engine, dir.path(), config.path(), &["plan"]));
    assert_eq!(out.status.code(), Some(2));
    assert!(stdin_free(&out).lines().any(|line| line == "stop Resource/reader setting=/limit control=example.org/control#set-limit.v1 requires=--replace reader OBJ-011"), "row 25 did not name the ready instance's control");
    engine.close().await;
}

#[test]
fn registration_batches_follow_reference_order() {
    let repository = repository();
    let spec = conduit_loader::Specification::read(&repository.join("spec")).unwrap();
    for case in cases() {
        let dir = tempfile::tempdir().unwrap();
        project_fixture(dir.path(), &case, None);
        let project = boundary::project(dir.path(), &spec).unwrap();
        for request in case["requests"].as_array().unwrap() {
            if request["tool"] != "management.register.v1" {
                continue;
            }
            let want = request["arguments"]["documents"].as_array().unwrap();
            let mut actual: Vec<_> = project
                .documents
                .iter()
                .filter(|d| want.contains(d))
                .collect();
            conduit_shell::plan::order_registration_documents(&mut actual);
            assert!(
                actual.into_iter().eq(want),
                "{}: batch differs from the loader's translated documents",
                case["id"]
            );
        }
    }
}

struct ControlFixture;
impl conduit_shell::Executor for ControlFixture {
    fn builtin_runner(
        &self,
        _: &Value,
        _: Option<&conduit_shell::RunIdentity>,
    ) -> conduit_shell::Result<conduit_shell::Execution> {
        Ok(conduit_shell::Execution::SetupReport(
            json!({"kind":"inspect-run","state":"running"}),
        ))
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_a_03_live_inspection_cannot_confirm_a_reconciled_control_setting() {
    use conduit_shell::Shell;
    use plan_apply_support::result;

    let dir = tempfile::tempdir().unwrap();
    let config = tempfile::tempdir().unwrap();
    let context = boundary::context(&repository()).unwrap();
    let core = &context.core_package;
    let api = &context.builtin.as_ref().unwrap().manifest.as_ref().unwrap()["apiVersion"];
    let manifest = json!({"apiVersion":api,"address":"example.org/control","implementations":{
        "reader-implementation":{"command":["fixture-reader"],"implements":["#set-limit.v1"],"plain":true,"tools":{"#set-limit.v1":"set_limit"}}
    }});
    let mut documents = json!([
        {"apiVersion":api,"kind":"Operation","metadata":{"name":"set-limit.v1"},"spec":{
            "inputSchema":{"type":"object","properties":{"revision":{"$ref":format!("{core}#revision.v1")},"limit":{"type":"integer"}},"required":["revision"]},
            "effect":"Sets the limit.","changes":{"limit":"/limit"}
        }},
        {"apiVersion":api,"kind":"Capability","metadata":{"name":"reader.v1"},"spec":{"operations":["#set-limit.v1"]}},
        {"apiVersion":api,"kind":"Type","metadata":{"name":"reader-type.v1"},"spec":{
            "family":"Resource","offers":["#reader.v1"],"controls":["#set-limit.v1"],
            "schema":{"type":"object","properties":{"limit":{"type":"integer"}}}
        }},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"reader--t"},"spec":{"type":"#reader-type.v1","config":{"limit":1},"implementation":{"#set-limit.v1":"#reader-implementation"}}},
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"reader"},"from":"reader--t","owner":"system"},
        {"apiVersion":api,"kind":"Conduit","metadata":{"name":"control--t"},"spec":{"capability":"#reader.v1","operations":["#set-limit.v1"]}},
        {"apiVersion":api,"kind":"Conduit","metadata":{"name":"control"},"from":"control--t",
            "consumer":{"kind":"Interface","name":"root"},"target":{"kind":"Resource","name":"reader"}}
    ]);

    let mut control_id = String::new();
    let engine = Engine::start_with(|shell: &Shell| {
        let root = shell
            .state()
            .unwrap()
            .objects
            .iter()
            .find(|o| o["kind"] == "Interface" && o["name"] == "root")
            .unwrap()["id"]
            .clone();
        let invoke =
            |tool: &str, arguments: Value| {
                let reply = shell.submit(json!({"kind":"invoke","request":{
                "identity":{"kind":"instance","id":root},"tool":tool,"arguments":arguments,
            }}),&ControlFixture).unwrap();
                result(&reply)
            };
        let registered = invoke(
            "management.register.v1",
            json!({"manifest":manifest,"documents":documents}),
        );
        assert_eq!(
            registered["outcome"], "succeeded",
            "register: {}",
            registered["diagnostics"]
        );
        assert_eq!(
            invoke(
                "system.set-up.v1",
                json!({"instance":{"kind":"Resource","name":"reader"}})
            )["outcome"],
            "succeeded"
        );
        assert_eq!(
            invoke(
                "access.open.v1",
                json!({"conduit":{"kind":"Conduit","name":"control"}})
            )["outcome"],
            "succeeded"
        );
        let inspected = invoke(
            "system.inspect.v1",
            json!({"object":{"kind":"Resource","name":"reader"}}),
        );
        let control = invoke(
            "control.set-limit.v1",
            json!({"revision":inspected["revision"],"limit":2}),
        );
        assert_eq!(
            control["outcome"], "unknown",
            "the scripted control must reach an unknown outcome"
        );
        control_id = control["invocation"].as_str().unwrap().to_owned();
    })
    .await;

    let inspect = json!({"object":{"kind":"Resource","name":"reader"}});
    let before = result(
        &engine
            .call("system.inspect.v1", inspect.clone(), None)
            .await,
    );
    assert_eq!(before["outcome"], "succeeded");
    assert_eq!(before["status"]["unreconciled"], json!([control_id]));
    let unknown_settings = json!({"/limit":{
        "state":"unknown","control":"example.org/control#set-limit.v1","invocation":control_id
    }});
    assert_eq!(before["status"]["settings"], unknown_settings);
    let reconciled = result(
        &engine
            .call(
                "system.reconcile.v1",
                json!({
                    "invocation":control_id,"occurred":true,"checked":"fixture observed the effect"
                }),
                None,
            )
            .await,
    );
    assert_eq!(reconciled["outcome"], "succeeded");
    let after = result(&engine.call("system.inspect.v1", inspect, None).await);
    assert_eq!(after["outcome"], "succeeded");
    assert_eq!(after["status"]["unreconciled"], json!([]));
    assert_eq!(after["status"]["settings"], unknown_settings);

    fs::write(dir.path().join("conduit.json"), manifest.to_string()).unwrap();
    documents[3]["spec"]["config"]["limit"] = json!(3);
    fs::write(dir.path().join("documents.json"), documents.to_string()).unwrap();
    let list = result(&engine.call("management.list.v1", json!({}), None).await);
    let mut objects = Vec::new();
    for (kind, name, view) in [
        ("Resource", "reader", Some(&after)),
        ("Resource", "reader--t", None),
        ("Conduit", "control", None),
        ("Conduit", "control--t", None),
    ] {
        let item = list["objects"]
            .as_array()
            .unwrap()
            .iter()
            .find(|o| o["kind"] == kind && o["name"] == name)
            .unwrap();
        let fetched;
        let view = if let Some(view) = view {
            view
        } else {
            fetched = result(
                &engine
                    .call(
                        "management.inspect.v1",
                        json!({"object":{"kind":kind,"name":name}}),
                        None,
                    )
                    .await,
            );
            &fetched
        };
        assert_eq!(view["outcome"], "succeeded");
        let kept_definition = if item["template"] == true {
            view["declared"]["spec"].clone()
        } else if name == "reader" {
            json!({"from":"reader--t","owner":"system","resolved":{"type":"example.org/control#reader-type.v1","config":{"limit":1}}})
        } else {
            json!({"from":"control--t"})
        };
        objects.push(json!([
            kind,
            name,
            item["id"],
            view["revision"],
            if item["template"] == true {
                json!("template")
            } else {
                item["state"].clone()
            },
            kept_definition,
            view["packages"]
        ]));
    }
    fs::create_dir(dir.path().join(".conduit")).unwrap();
    fs::write(
        dir.path().join(".conduit/state.yaml"),
        json!({"format":1,"files":{},"objects":objects,"tombstones":[],"plans":[],"journals":{}})
            .to_string(),
    )
    .unwrap();
    let out = tokio::task::block_in_place(|| tool(&engine, dir.path(), config.path(), &["plan"]));
    assert_eq!(
        out.status.code(),
        Some(2),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    // Reconciliation settled the invocation but not the setting: the engine's status still
    // holds `/limit` as unknown (REV-009), and plan stops on it.
    let wanted = format!(
        "stop Resource/reader setting=/limit state=unknown control=example.org/control#set-limit.v1 invocation={control_id} REV-009"
    );
    assert!(
        stdin_free(&out).lines().any(|line| line == wanted),
        "missing REV-009 stop for the live control history"
    );
    assert!(!stdin_free(&out).contains("inspection="));
    assert!(!stdin_free(&out).contains("unreconciled="));
    engine.close().await;
}
