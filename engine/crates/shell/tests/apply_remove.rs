mod package_target;
mod plan_apply_support;

use conduit_shell::boundary;
use plan_apply_support::cli::{cache_fixture, files, project_fixture, stdin_free, tool};
use plan_apply_support::{Engine, cases, replay_while, result};
use serde_json::{Value, json};
use std::{fs, path::Path, process::Output};

const API: &str = "github.com/shrav-k/conduit/v0";
const CORE: &str = "github.com/shrav-k/conduit/core";

async fn after_case_two(mut test: impl FnMut(&Engine, &Path, &Path)) {
    let cases = cases();
    let report = replay_while(&cases[..3], |engine, case, symbols| {
        if case["id"] != "03-t3-noop" {
            return true;
        }
        let dir = tempfile::tempdir().unwrap();
        let config = tempfile::tempdir().unwrap();
        cache_fixture(config.path(), 2);
        project_fixture(dir.path(), case, Some(symbols));
        test(engine, dir.path(), config.path());
        false
    })
    .await;
    assert!(report.lines.is_empty());
}

fn call(engine: &Engine, operation: &str, args: Value) -> Value {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { result(&engine.call(operation, args, None).await) })
    })
}

fn rules(reply: &Value) -> Vec<String> {
    reply["diagnostics"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|diagnostic| diagnostic["ruleId"].as_str().map(str::to_owned))
        .collect()
}

fn listed(engine: &Engine) -> Value {
    let reply = call(engine, "management.list.v1", json!({}));
    assert_eq!(reply["outcome"], "succeeded");
    json!({"objects":reply["objects"],"tombstones":reply["tombstones"]})
}

fn command(engine: &Engine, dir: &Path, config: &Path, args: &[&str]) -> Output {
    tokio::task::block_in_place(|| tool(engine, dir, config, args))
}

fn stopped(engine: &Engine, dir: &Path, config: &Path, args: &[&str], line: &str) {
    let before = listed(engine);
    let kept = files(&dir.join(".conduit"));
    let result = command(engine, dir, config, args);
    assert_eq!(result.status.code(), Some(2));
    assert!(stdin_free(&result).contains(line), "expected stop: {line}");
    assert_eq!(listed(engine), before);
    assert_eq!(files(&dir.join(".conduit")), kept);
}

fn edit(dir: &Path, transform: impl FnOnce(&mut Value)) -> String {
    let file = dir.join("s1.yaml");
    let previous = fs::read_to_string(&file).unwrap();
    let mut doc = boundary::value(&file).unwrap();
    transform(&mut doc);
    fs::write(file, doc.to_string()).unwrap();
    previous
}

fn remove(doc: &mut Value, collection: &str, name: &str) {
    doc[collection].as_object_mut().unwrap().remove(name);
}

fn remove_member_grants(doc: &mut Value, local: &str) {
    for agent in doc["agents"].as_object_mut().unwrap().values_mut() {
        remove(agent, "consumes", local);
    }
}

fn has(row_set: &Value, name: &str) -> bool {
    row_set
        .as_array()
        .unwrap()
        .iter()
        .any(|row| row["name"] == name)
}

fn has_tombstone(rows: &Value, name: &str) -> bool {
    rows["tombstones"]
        .as_array()
        .unwrap()
        .iter()
        .any(|row| row["object"]["name"] == name)
}

fn kept(dir: &Path, name: &str) -> bool {
    boundary::value(&dir.join(".conduit/state.yaml")).unwrap()["objects"]
        .as_array()
        .unwrap()
        .iter()
        .any(|row| row[1] == name)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_apply_prune_retires_and_unregisters() {
    after_case_two(|engine, dir, config| {
        let previous = edit(dir, |doc| {
            remove(doc, "resources", "history");
            remove_member_grants(doc, "history");
        });
        stopped(
            engine,
            dir,
            config,
            &["apply"],
            "requires=--prune history LIF-035",
        );
        let applied = command(engine, dir, config, &["apply", "--prune", "history"]);
        assert_eq!(
            applied.status.code(),
            Some(0),
            "prune history should succeed"
        );
        let rows = listed(engine);
        for name in ["history", "history--t"] {
            assert!(!has(&rows["objects"], name));
            assert!(!kept(dir, name));
        }
        assert!(has_tombstone(&rows, "history"));
        let state = boundary::value(&dir.join(".conduit/state.yaml")).unwrap();
        for name in ["history", "agent-a--history", "agent-b--history"] {
            assert!(
                state["tombstones"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|row| row["object"]["name"] == name),
                "prune must keep the tombstone for {name}"
            );
        }
        fs::write(dir.join("s1.yaml"), previous).unwrap();
        stopped(
            engine,
            dir,
            config,
            &["apply"],
            "requires=--recreate history LIF-041",
        );
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_249_prune_dropped_standalone_template() {
    after_case_two(|engine, dir, config| {
        let previous = edit(dir, |doc| {
            doc["templates"] = json!({"conduits":{"standalone":{
                "capability":format!("{CORE}#access-control.v1"),
                "operations":[format!("{CORE}#open.v1")]
            }}});
        });
        let added = command(engine, dir, config, &["apply"]);
        assert_eq!(added.status.code(), Some(0), "standalone template setup");
        assert!(kept(dir, "standalone"));
        let before = call(
            engine,
            "management.inspect.v1",
            json!({"object":{"kind":"Conduit","name":"standalone"}}),
        );
        assert_eq!(before["outcome"], "succeeded");
        fs::write(dir.join("s1.yaml"), previous).unwrap();
        stopped(
            engine,
            dir,
            config,
            &["apply"],
            "stop Conduit/standalone absent=file requires=--prune standalone INS-020",
        );
        let applied = command(engine, dir, config, &["apply", "--prune", "standalone"]);
        assert_eq!(applied.status.code(), Some(0));
        assert!(stdin_free(&applied).contains("removes flag=--prune standalone INS-020"));
        let state = boundary::value(&dir.join(".conduit/state.yaml")).unwrap();
        let digest = state["plans"]
            .as_array()
            .unwrap()
            .last()
            .unwrap()
            .as_str()
            .unwrap();
        let journal =
            boundary::value(&dir.join(state["journals"][digest].as_str().unwrap())).unwrap();
        assert_eq!(
            journal["entries"][0]["arguments"]["revision"],
            before["revision"]
        );
        assert!(!kept(dir, "standalone"));
        assert!(!has(&listed(engine)["objects"], "standalone"));
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_apply_prune_refuses_unused_operands() {
    after_case_two(|engine, dir, config| {
        let template = json!({"apiVersion":API,"kind":"Pool","metadata":{"name":"outside--t"},
            "spec":{"type":format!("{CORE}#pool.v1")}});
        let outside = json!({"apiVersion":API,"kind":"Pool","metadata":{"name":"outside"},
            "from":"outside--t"});
        let reply = call(
            engine,
            "management.register.v1",
            json!({"documents":[template,outside]}),
        );
        assert_eq!(
            reply["outcome"],
            "succeeded",
            "fixture registration rules: {:?}",
            rules(&reply)
        );
        for flag in ["--prune", "--adopt", "--with-resources", "--with-agents"] {
            for operand in ["no-such-object", "shared", "outside"] {
                stopped(
                    engine,
                    dir,
                    config,
                    &["apply", flag, operand],
                    &format!("stop Flag/{flag} operand={operand} use=none -"),
                );
            }
        }
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_apply_adopt_takes_over_after_verifying() {
    after_case_two(|engine, dir, config| {
        let resource = json!({"type":"example.org/s1/history#file-history.v1","owner":"team",
                               "config":{"root":"/lab/adopted"}});
        edit(dir, |doc| doc["resources"]["adopted"] = resource.clone());
        let submission = tempfile::tempdir().unwrap();
        let spec =
            conduit_loader::Specification::read(&plan_apply_support::repository().join("spec"))
                .unwrap();
        let project = boundary::project(dir, &spec).unwrap();
        for (index, doc) in project
            .documents
            .iter()
            .filter(|doc| {
                doc["kind"] == "Resource"
                    && matches!(
                        doc["metadata"]["name"].as_str(),
                        Some("adopted" | "adopted--t")
                    )
            })
            .enumerate()
        {
            fs::write(
                submission.path().join(format!("resource-{index}.yaml")),
                doc.to_string(),
            )
            .unwrap();
        }
        fs::copy(
            dir.join("conduit.yaml"),
            submission.path().join("conduit.yaml"),
        )
        .unwrap();
        let submitted = command(engine, submission.path(), config, &["submit", "."]);
        let diagnostics: Vec<_> = stdin_free(&submitted)
            .lines()
            .filter_map(|line| serde_json::from_str::<Value>(line).ok())
            .map(|reply| (rules(&reply), rules(&reply["structuredContent"])))
            .collect();
        assert_eq!(
            submitted.status.code(),
            Some(0),
            "fixture submit should succeed: {diagnostics:?}"
        );
        stopped(
            engine,
            dir,
            config,
            &["apply"],
            "requires=--adopt adopted OBJ-002",
        );
        let before = listed(engine);
        let applied = command(
            engine,
            dir,
            config,
            &["apply", "--adopt", "adopted", "--adopt", "adopted--t"],
        );
        assert_eq!(applied.status.code(), Some(0), "adoption should succeed");
        let text = stdin_free(&applied);
        assert_eq!(
            text.lines()
                .filter(|line| line.starts_with("result "))
                .count(),
            2
        );
        assert_eq!(
            text.lines()
                .filter(|line| line.starts_with("detail adopt Resource/"))
                .count(),
            2
        );
        assert_eq!(listed(engine), before);
        assert!(kept(dir, "adopted") && kept(dir, "adopted--t"));
        assert_eq!(
            stdin_free(&command(engine, dir, config, &["plan"])),
            "plan\nno changes\n"
        );
        edit(dir, |doc| {
            doc["resources"]["adopted"]["config"]["root"] = json!("/lab/changed")
        });
        stopped(
            engine,
            dir,
            config,
            &["apply"],
            "requires=--replace adopted",
        );
    })
    .await;
}

fn cascade(
    engine: &Engine,
    dir: &Path,
    config: &Path,
    flag: &str,
    owner: &str,
    members: &[&str],
    retained: &[&str],
) {
    stopped(
        engine,
        dir,
        config,
        &["apply"],
        &format!("requires={flag} {owner} LIF-020"),
    );
    let applied = command(engine, dir, config, &["apply", flag, owner]);
    let text = stdin_free(&applied);
    let result_steps: Vec<_> = text
        .lines()
        .filter_map(|line| {
            if line.starts_with("verify ") || line.starts_with("stop ") {
                return Some(line.to_owned());
            }
            let rest = line.strip_prefix("result ")?;
            let mut fields = rest.splitn(3, ' ');
            let number = fields.next()?;
            let outcome = fields.next()?;
            let reply: Value = serde_json::from_str(fields.next()?).ok()?;
            Some(format!("{number} {outcome} {:?}", rules(&reply)))
        })
        .collect();
    assert_eq!(
        applied.status.code(),
        Some(0),
        "cascade should succeed: {result_steps:?}"
    );
    let plan = text.split_once("ready\n").unwrap().0;
    println!("{plan}ready");
    let operation = if flag == "--with-resources" {
        "retire-with-resources.v1"
    } else {
        "retire-with-agents.v1"
    };
    let operation = format!("cascade-{owner}.{operation}");
    let lines = [
        "management.register.v1",
        "management.inspect.v1",
        "access.open.v1",
        operation.as_str(),
        "management.list.v1",
        "access.lift.v1",
    ];
    let steps: Vec<_> = plan
        .lines()
        .filter_map(|line| {
            let (number, rest) = line.split_once(' ')?;
            number.parse::<usize>().ok()?;
            rest.split_whitespace().next()
        })
        .collect();
    assert!(steps.windows(6).any(|steps| steps == lines));
    let default = if flag == "--with-resources" {
        "root-pool-"
    } else {
        "root-fleet-"
    };
    assert!(plan.contains(&format!(
        "closes=[Conduit/cascade-{owner},Conduit/{default}"
    )));
    let rows = listed(engine);
    for name in std::iter::once(owner).chain(members.iter().copied()) {
        assert!(!has(&rows["objects"], name) && !kept(dir, name));
        assert!(!has(&rows["objects"], &format!("{name}--t")));
        assert!(has_tombstone(&rows, name));
    }
    let temp = format!("cascade-{owner}");
    assert!(!has(&rows["objects"], &temp) && !kept(dir, &temp));
    assert!(!has_tombstone(&rows, &temp));
    for name in retained {
        assert!(
            rows["objects"]
                .as_array()
                .unwrap()
                .iter()
                .any(|row| row["name"] == *name && row["state"] == "ready")
        );
    }
    for grant in [
        "agent-a--history",
        "agent-b--history",
        "agent-a--project",
        "agent-b--project",
    ] {
        assert!(has_tombstone(&rows, grant), "{grant} tombstone stays");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_apply_with_resources_cascades_a_dropped_pool() {
    after_case_two(|engine, dir, config| {
        edit(dir, |doc| {
            remove(doc, "pools", "team");
            doc.as_object_mut().unwrap().remove("resources");
            remove_member_grants(doc, "project");
            remove_member_grants(doc, "history");
        });
        cascade(
            engine,
            dir,
            config,
            "--with-resources",
            "team",
            &["history", "shared"],
            &["agent-a", "agent-b"],
        );
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn plan_apply_with_agents_cascades_a_dropped_fleet() {
    after_case_two(|engine, dir, config| {
        edit(dir, |doc| {
            doc.as_object_mut().unwrap().remove("agents");
            doc.as_object_mut().unwrap().remove("fleet");
        });
        cascade(
            engine,
            dir,
            config,
            "--with-agents",
            "s1",
            &["agent-a", "agent-b"],
            &["team", "history", "shared"],
        );
    })
    .await;
}
