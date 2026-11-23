mod package_target;
mod plan_apply_support;

use conduit_shell::boundary;
use plan_apply_support::cli::*;
use plan_apply_support::{Engine, Symbols, cases, check_apply_case, replay_while, result, sha256};
use serde_json::{Value, json};
use std::{fs, path::Path, process::Output};

const API: &str = "github.com/shrav-k/conduit/v0";
const CORE: &str = "github.com/shrav-k/conduit/core";

fn call(engine: &Engine, operation: &str, args: Value) -> Value {
    call_with_key(engine, operation, args, None)
}

fn call_with_key(engine: &Engine, operation: &str, args: Value, key: Option<&str>) -> Value {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { result(&engine.call(operation, args, key).await) })
    })
}

fn succeeded(engine: &Engine, operation: &str, args: Value) -> Value {
    let reply = call(engine, operation, args);
    assert_eq!(
        reply["outcome"], "succeeded",
        "fixture operation {operation}"
    );
    reply
}

fn listed(engine: &Engine) -> Value {
    let reply = succeeded(engine, "management.list.v1", json!({}));
    json!({"objects":reply["objects"],"tombstones":reply["tombstones"]})
}

fn project_id(engine: &Engine, kind: &str, name: &str) -> String {
    listed(engine)["objects"]
        .as_array()
        .unwrap()
        .iter()
        .find(|object| object["kind"] == kind && object["name"] == name)
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .to_owned()
}

fn command(engine: &Engine, dir: &Path, config: &Path, args: &[&str]) -> Output {
    tokio::task::block_in_place(|| tool(engine, dir, config, args))
}

fn stopped(engine: &Engine, dir: &Path, config: &Path, args: &[&str], line: &str) {
    let before = listed(engine);
    let kept = files(&dir.join(".conduit"));
    let result = command(engine, dir, config, args);
    assert_eq!(result.status.code(), Some(2));
    assert!(
        stdin_free(&result).contains(line),
        "expected stopped plan line: {line}"
    );
    assert_eq!(listed(engine), before);
    assert_eq!(files(&dir.join(".conduit")), kept);
}

async fn at_case(id: &str, mut test: impl FnMut(&Engine, &Path, &Path, &Value, &Symbols)) {
    let cases = cases();
    let target = cases.iter().position(|case| case["id"] == id).unwrap();
    let replay = replay_while(&cases[..=target], |engine, case, symbols| {
        if case["id"] != id {
            return true;
        }
        let dir = tempfile::tempdir().unwrap();
        let config = tempfile::tempdir().unwrap();
        cache_fixture(config.path(), id[..2].parse().unwrap());
        project_fixture(dir.path(), case, Some(symbols));
        test(engine, dir.path(), config.path(), case, symbols);
        false
    })
    .await;
    assert!(replay.lines.is_empty());
}

async fn golden(id: &str, flag: &str, operand: &str) {
    at_case(id, |engine, dir, config, case, symbols| {
        let mut expected = symbols.plan_text(case["recorded"]["text"].as_str().unwrap());
        let mapped = symbols.kept_for_plan(&case["applied"]);
        for (original, live) in case["applied"]["objects"]
            .as_array()
            .unwrap()
            .iter()
            .zip(mapped["objects"].as_array().unwrap())
        {
            if original[1] == "shared--t" {
                expected = expected.replace(
                    &format!(" revision={}\n", original[3].as_str().unwrap()),
                    &format!(" revision={}\n", live[3].as_str().unwrap()),
                );
            }
        }
        let before = files(&dir.join(".conduit"));
        let before_engine = listed(engine);
        let plan = command(engine, dir, config, &["plan", flag, operand]);
        assert_eq!(
            plan.status.code(),
            Some(0),
            "{id}: approved plan should succeed: {:?}",
            stdin_free(&plan)
                .lines()
                .filter(|line| line.starts_with("stop "))
                .collect::<Vec<_>>()
        );
        if stdin_free(&plan) != expected {
            let pair = stdin_free(&plan)
                .lines()
                .zip(expected.lines())
                .find(|(a, b)| a != b);
            panic!("{id}: first plan difference: {pair:?}");
        }
        assert_eq!(files(&dir.join(".conduit")), before);
        assert_eq!(listed(engine), before_engine);

        let output = command(engine, dir, config, &["apply", flag, operand]);
        assert_eq!(output.status.code().map(i64::from), case["exit"].as_i64());
        let tail = stdin_free(&output)
            .strip_prefix(&expected)
            .expect("apply prints the golden plan");
        let hex = sha256(&expected);
        let kept = dir.join(".conduit");
        assert!(fs::read_to_string(kept.join(format!("plans/{hex}.txt"))).unwrap() == expected);
        let journal = boundary::value(&kept.join(format!("results/{hex}.yaml"))).unwrap();
        let entries = journal["entries"].as_array().unwrap();
        let lines: Vec<_> = tail
            .lines()
            .filter(|line| line.starts_with("result "))
            .collect();
        assert_eq!(entries.len(), case["requests"].as_array().unwrap().len());
        assert_eq!(lines.len(), entries.len());
        for (i, (line, entry)) in lines.iter().zip(entries).enumerate() {
            let prefix = format!(
                "result {} {} ",
                i + 1,
                entry["result"]["outcome"].as_str().unwrap()
            );
            let printed: Value = serde_json::from_str(line.strip_prefix(&prefix).unwrap()).unwrap();
            assert_eq!(printed, entry["result"]);
            assert_eq!(entry["key"], format!("plan:{hex}:1:{}", i + 1));
        }
        let ledger = boundary::value(&kept.join("state.yaml")).unwrap();
        assert_eq!(ledger["files"], case["kept"]["files"]);
        assert_eq!(
            ledger["objects"].as_array().unwrap().len(),
            case["kept"]["objects"].as_array().unwrap().len()
        );
        // Template revisions have their own symbols, distinct from object IDs.
        let mut expected_case = case.clone();
        for (request, entry) in expected_case["requests"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .zip(entries)
        {
            if request["result"].get("revision").is_some() {
                let template = &request["arguments"]["template"];
                let live = succeeded(
                    engine,
                    "management.inspect.v1",
                    json!({
                        "object":{"kind":template["kind"],"name":template["metadata"]["name"]}
                    }),
                );
                assert_eq!(entry["result"]["revision"], live["revision"]);
                assert_ne!(entry["arguments"]["revision"], live["revision"]);
                request["result"]["revision"] = live["revision"].clone();
            }
        }
        check_apply_case(engine, &expected_case, symbols, entries, &ledger, &hex);
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn golden_case_04_t4() {
    golden("04-t4", "--replace", "agent-a--project").await;
}
#[tokio::test(flavor = "multi_thread")]
async fn golden_case_05_t5() {
    golden("05-t5", "--replace", "shared").await;
}
#[tokio::test(flavor = "multi_thread")]
async fn golden_case_06_t6() {
    golden("06-t6", "--replace", "shared").await;
}
#[tokio::test(flavor = "multi_thread")]
async fn golden_case_08_t7_restore() {
    golden("08-t7-restore", "--reopen", "agent-a--project").await;
}

#[tokio::test(flavor = "multi_thread")]
async fn unflagged_replacements_stop_and_send_nothing() {
    let cases = cases();
    let report = replay_while(&cases[..6], |engine, case, symbols| {
        let place = match case["id"].as_str().unwrap() {
            "04-t4" => "place=/from requires=--replace agent-a--project",
            "05-t5" => "pin=example.org/s1/filesystem@0.2.1 requires=--replace shared",
            // K3's setting pointers start at the configuration, so this is /root.
            "06-t6" => "setting=/root requires=--replace shared",
            _ => return true,
        };
        let dir = tempfile::tempdir().unwrap();
        let config = tempfile::tempdir().unwrap();
        project_fixture(dir.path(), case, Some(symbols));
        cache_fixture(
            config.path(),
            case["id"].as_str().unwrap()[..2].parse().unwrap(),
        );
        stopped(engine, dir.path(), config.path(), &["apply"], place);
        case["id"] != "06-t6"
    })
    .await;
    assert!(report.lines.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn operands_name_one_object_or_stop() {
    at_case("04-t4", |engine, dir, config, _, _| {
        for (args, line) in [
            (
                vec!["apply", "--reopen", "shared"],
                "stop Flag/--reopen operand=shared use=none -",
            ),
            (
                vec![
                    "apply",
                    "--replace",
                    "agent-a--project",
                    "--reopen",
                    "agent-a--project",
                ],
                "stop Flag/--reopen operand=agent-a--project use=none -",
            ),
        ] {
            stopped(engine, dir, config, &args, line);
        }
        let missing = command(engine, dir, config, &["apply", "--replace"]);
        assert_eq!(missing.status.code(), Some(2));
        assert!(missing.stdout.is_empty());
        assert!(String::from_utf8_lossy(&missing.stderr).contains("--replace needs OBJECT"));
        let twice = command(
            engine,
            dir,
            config,
            &[
                "plan",
                "--replace",
                "agent-a--project",
                "--replace",
                "agent-a--project",
            ],
        );
        assert_eq!(twice.status.code(), Some(0));
        assert_eq!(
            stdin_free(&twice)
                .matches("flags [--replace agent-a--project]")
                .count(),
            1
        );
        let steady = cases().into_iter().find(|case| case["id"] == "03-t3-noop").unwrap();
        fs::write(dir.join("s1.yaml"), steady["files"]["s1.yaml"].as_str().unwrap()).unwrap();
        let grant_file = dir.join("ambiguous-grant.yaml");
        let mut grant = framed("Conduit", "ambiguous-grant", json!({
            "capability":"example.org/s1/filesystem#directory.v1",
            "operations":["example.org/s1/filesystem#file.read.v1"]
        }));
        let parent = framed("Interface", "ambiguous-parent--t", json!({
            "type":format!("{CORE}#interface.v1"),
            "conduits":{"project":{"from":"ambiguous-grant","target":{"kind":"Resource","name":"shared"}}}
        }));
        let interface = json!({"apiVersion":API,"kind":"Interface","metadata":{"name":"ambiguous-parent"},"from":"ambiguous-parent--t"});
        let write_grant = |grant: &Value| fs::write(&grant_file, format!("{}\n---\n{}\n---\n{}\n",grant,parent,interface)).unwrap();
        write_grant(&grant);
        assert_eq!(command(engine, dir, config, &["apply"]).status.code(), Some(0), "seed the generated Conduit");
        let info = succeeded(engine, "management.inspect.v1",
            json!({"object":{"kind":"Interface","name":"ambiguous-parent"}}));
        let child = &info["declared"]["slots"]["conduits"]["project"]["id"];
        let objects = listed(engine);
        let name = objects["objects"].as_array().unwrap().iter()
            .find(|object| object["id"] == *child).unwrap()["name"].as_str().unwrap();
        let resource_file = dir.join("ambiguous-resource.yaml");
        let mut resource = framed("Resource", "ambiguous-resource--t", json!({
            "type":"example.org/s1/filesystem#local-directory.v1","config":{"root":"/lab/shared"}
        }));
        let instance = json!({"apiVersion":API,"kind":"Resource","metadata":{"name":name},
            "from":"ambiguous-resource--t","owner":"team"});
        let write_resource = |resource: &Value| fs::write(&resource_file, format!("{}\n---\n{}\n",resource,instance)).unwrap();
        write_resource(&resource);
        let seed = command(engine, dir, config, &["apply"]);
        assert_eq!(seed.status.code(), Some(0), "seed Resource with the Conduit's generated name. Stops: {:?}. Error: {:?}",
            stdin_free(&seed).lines().filter(|line| line.starts_with("stop ")).collect::<Vec<_>>(),
            String::from_utf8_lossy(&seed.stderr).lines().last());
        grant["spec"]["operations"].as_array_mut().unwrap().push(json!("example.org/s1/filesystem#file.write.v1"));
        write_grant(&grant);
        resource["spec"]["config"]["root"] = json!("/lab/ambiguous");
        write_resource(&resource);
        stopped(engine, dir, config, &["apply","--replace",name],
            &format!("stop Flag/--replace operand={name} use=ambiguous objects=[Conduit/{name},Resource/{name}] -"));
    })
    .await;
}

fn retire_shared(engine: &Engine, dir: &Path, config: &Path) -> String {
    let previous = project_id(engine, "Resource", "shared");
    assert_eq!(
        command(engine, dir, config, &["retire", "Resource/shared"])
            .status
            .code(),
        Some(0)
    );
    previous
}

#[tokio::test(flavor = "multi_thread")]
async fn recreate_and_reopen_after_an_outside_retirement() {
    at_case("07-t7-revoke", |engine, dir, config, _, _| {
        let case_06 = cases()
            .into_iter()
            .find(|case| case["id"] == "06-t6")
            .unwrap();
        fs::write(
            dir.join("s1.yaml"),
            case_06["files"]["s1.yaml"].as_str().unwrap(),
        )
        .unwrap();
        for lifted in [false, true] {
            let previous = retire_shared(engine, dir, config);
            if lifted {
                let stones = listed(engine)["tombstones"].as_array().unwrap().clone();
                let root = stones
                    .iter()
                    .find(|stone| stone["object"]["id"] == previous)
                    .unwrap();
                succeeded(engine, "access.lift.v1", json!({"id":root["id"]}));
            }
            let place = if lifted {
                "absent=engine requires=--recreate shared"
            } else {
                "requires=--recreate shared LIF-041"
            };
            stopped(engine, dir, config, &["apply"], place);
            stopped(
                engine,
                dir,
                config,
                &["apply", "--recreate", "shared"],
                "requires=--reopen agent-a--project",
            );
            let restored = command(
                engine,
                dir,
                config,
                &[
                    "apply",
                    "--recreate",
                    "shared",
                    "--reopen",
                    "agent-a--project",
                    "--reopen",
                    "agent-b--project",
                ],
            );
            assert_eq!(
                restored.status.code(),
                Some(0),
                "all three approvals must restore the objects"
            );
            assert_ne!(project_id(engine, "Resource", "shared"), previous);
            project_id(engine, "Conduit", "agent-a--project");
            project_id(engine, "Conduit", "agent-b--project");
        }
        // Reopening one grant at a new target must leave the retired target's
        // tombstone and the other grant's tombstone alone.
        let previous = retire_shared(engine, dir, config);
        let source = dir.join("s1.yaml");
        let mut project = boundary::value(&source).unwrap();
        let shared = project["resources"]
            .as_object_mut()
            .unwrap()
            .remove("shared")
            .unwrap();
        let template = framed(
            "Resource",
            "shared--t",
            json!({"type":shared["type"],"config":shared["config"]}),
        );
        fs::write(dir.join("shared-template.yaml"), template.to_string()).unwrap();
        project["agents"]["agent-a"]["consumes"]["project"]["to"] =
            json!({"kind":"Resource","name":"history"});
        project["agents"]["agent-b"]["consumes"]
            .as_object_mut()
            .unwrap()
            .remove("project");
        fs::write(source, project.to_string()).unwrap();
        let reopened = command(
            engine,
            dir,
            config,
            &["apply", "--reopen", "agent-a--project"],
        );
        assert_eq!(reopened.status.code(), Some(0));
        let after = listed(engine);
        let stones = after["tombstones"].as_array().unwrap();
        assert!(stones.iter().any(|stone| stone["object"]["id"] == previous));
        assert!(
            stones
                .iter()
                .any(|stone| stone["object"]["name"] == "agent-b--project")
        );
    })
    .await;
}

fn framed(kind: &str, name: &str, spec: Value) -> Value {
    json!({"apiVersion":API,"kind":kind,"metadata":{"name":name},"spec":spec})
}

fn register(engine: &Engine, documents: Vec<Value>) {
    succeeded(
        engine,
        "management.register.v1",
        json!({
            "documents":documents,"manifest":{"apiVersion":API,"address":"example.org/replacement"}
        }),
    );
}

fn open(engine: &Engine, name: &str) {
    let id = project_id(engine, "Conduit", name);
    succeeded(
        engine,
        "access.open.v1",
        json!({"conduit":{"kind":"Conduit","name":name},"expectedId":id}),
    );
}

fn add_bound(engine: &Engine, name: &str, pools: Value, targets: Value) {
    let template = format!("{name}--t");
    register(
        engine,
        vec![
            framed(
                "Conduit",
                &template,
                json!({
                    "capability":format!("{CORE}#access-control.v1"),"operations":[format!("{CORE}#open.v1")],
                    "requirements":{"bound":{"requirement":format!("{CORE}#bound.v1"),"arguments":{
                        "pools":pools,"targets":targets,"operations":["example.org/s1/filesystem#file.read.v1"]}}}
                }),
            ),
            json!({"apiVersion":API,"kind":"Conduit","metadata":{"name":name},"from":template,
            "consumer":{"kind":"Interface","name":"root"},"target":{"kind":"Resource","name":"access"}}),
        ],
    );
    open(engine, name);
}

#[tokio::test(flavor = "multi_thread")]
async fn replacement_prints_bounds_and_an_owner_change() {
    at_case("06-t6", |engine, dir, config, _, _| {
        let mut project: Value = boundary::value(&dir.join("s1.yaml")).unwrap();
        project["pools"]["other"] = json!({"type":format!("{CORE}#pool.v1")});
        // Create the second Pool under this working copy before naming it in a bound.
        let case_05 = cases()
            .into_iter()
            .find(|case| case["id"] == "05-t5")
            .unwrap();
        let old_root = case_05["files"]["s1.yaml"].as_str().unwrap();
        let setup = old_root.replace(
            "pools: {team:",
            "pools: {other: {type: github.com/shrav-k/conduit/core#pool.v1}, team:",
        );
        fs::write(dir.join("s1.yaml"), setup).unwrap();
        assert_eq!(
            command(engine, dir, config, &["apply"]).status.code(),
            Some(0),
            "second Pool setup"
        );
        fs::write(dir.join("s1.yaml"), project.to_string()).unwrap();
        add_bound(
            engine,
            "target-bound",
            json!([]),
            json!([{"kind":"Resource","name":"shared"}]),
        );
        add_bound(
            engine,
            "old-pool-bound",
            json!([{"kind":"Pool","name":"team"}]),
            json!([]),
        );
        add_bound(
            engine,
            "new-pool-bound",
            json!([{"kind":"Pool","name":"other"}]),
            json!([]),
        );
        let plan = command(engine, dir, config, &["plan", "--replace", "shared"]);
        assert_eq!(
            plan.status.code(),
            Some(0),
            "replace should show lost target coverage"
        );
        assert!(stdin_free(&plan).contains("bounds=[Conduit/target-bound:Resource/shared]"));
        project["resources"]["shared"]["owner"] = json!("other");
        fs::write(dir.join("s1.yaml"), project.to_string()).unwrap();
        stopped(
            engine,
            dir,
            config,
            &["apply"],
            "place=/owner requires=--replace shared OBJ-011",
        );
        let applied = command(engine, dir, config, &["apply", "--replace", "shared"]);
        assert_eq!(applied.status.code(), Some(0));
        let text = stdin_free(&applied);
        assert!(text.contains("team.retire.v1 Resource/shared"));
        assert!(text.contains("other.set-up.v1 Resource/shared"));
        assert!(text.contains(
            "bounds=[Conduit/old-pool-bound:Resource/shared,Conduit/target-bound:Resource/shared]"
        ));
        assert!(text.contains("bounds-added=[Conduit/new-pool-bound:Resource/shared]"));
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn replacement_stops_on_a_conduit_outside_the_file() {
    at_case("06-t6", |engine, dir, config, case, _| {
        let manifest = boundary::value(&dir.join("conduit.yaml")).unwrap();
        succeeded(engine, "management.register.v1", json!({"manifest":manifest,"documents":[{
            "apiVersion":API,"kind":"Conduit","metadata":{"name":"outside"},"as":"outside",
            "from":"example.org/s1/filesystem#read","consumer":{"kind":"Interface","name":"root"},
            "target":{"kind":"Resource","name":"shared"}
        }]}));
        assert_eq!(case["id"], "06-t6");
        open(engine, "outside");
        stopped(engine, dir, config, &["apply", "--replace", "shared"],
            "stop Conduit/outside absent=file place=/target CON-008");
    }).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn agent_replacement_prints_the_sign_in_loss() {
    at_case("07-t7-revoke", |engine, dir, config, _, _| {
        let old = cases()
            .into_iter()
            .find(|case| case["id"] == "06-t6")
            .unwrap();
        fs::write(
            dir.join("s1.yaml"),
            old["files"]["s1.yaml"].as_str().unwrap(),
        )
        .unwrap();
        let mut project = boundary::value(&dir.join("s1.yaml")).unwrap();
        project["agents"]["agent-a"]["annotations"] = json!({"purpose":"review"});
        fs::write(dir.join("s1.yaml"), project.to_string()).unwrap();
        let applied = command(engine, dir, config, &["apply", "--replace", "agent-a"]);
        assert_eq!(
            applied.status.code(),
            Some(0),
            "approved Agent replacement should succeed"
        );
        let text = stdin_free(&applied);
        assert!(text.contains("s1.retire.v1 Agent/agent-a"));
        assert!(text.contains("signed-out=[Agent/agent-a]"));
        assert!(text.contains("detail ownTools Agent/agent-a "));
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn interrupted_replacement_resumes_only_under_its_flag() {
    interrupted_replacement(false).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn review_248_commented_replacement_still_requires_approval() {
    interrupted_replacement(true).await;
}

async fn interrupted_replacement(after_lift: bool) {
    at_case("04-t4", |engine, dir, config, case, symbols| {
        let original = command(
            engine,
            dir,
            config,
            &["plan", "--replace", "agent-a--project"],
        );
        assert_eq!(
            original.status.code(),
            Some(0),
            "original replacement should be ready"
        );
        let text = stdin_free(&original);
        let digest = sha256(text);
        let local = dir.join(".conduit");
        let file = local.join("state.yaml");
        let mut ledger = boundary::value(&file).unwrap();
        let mut entries = vec![];
        for (i, name) in ["agent-a--project", "agent-b--project"].iter().enumerate() {
            let id = project_id(engine, "Conduit", name);
            let arguments = json!({"conduit":{"kind":"Conduit","name":name},"expectedId":id});
            let key = format!("plan:{digest}:1:{}", i + 1);
            let reply = call_with_key(engine, "access.close.v1", arguments.clone(), Some(&key));
            assert_eq!(reply["outcome"], "succeeded");
            entries.push(
                json!({"step":i+1,"tool":"access.close.v1","arguments":arguments,
                "key":key,"flag":case["requests"][i]["flag"],"result":reply}),
            );
            ledger["objects"]
                .as_array_mut()
                .unwrap()
                .retain(|row| row[2] != id);
        }
        if after_lift {
            let stones = listed(engine);
            for stone in stones["tombstones"].as_array().unwrap() {
                succeeded(engine, "access.lift.v1", json!({"id":stone["id"]}));
            }
        }
        keep_interruption(dir, text, ledger, entries);
        if after_lift {
            use std::io::Write;
            writeln!(
                fs::OpenOptions::new()
                    .append(true)
                    .open(dir.join("s1.yaml"))
                    .unwrap(),
                "\n# An unrelated comment."
            )
            .unwrap();
        }
        stopped(
            engine,
            dir,
            config,
            &["apply"],
            "requires=--replace agent-a--project",
        );
        let resumed = command(
            engine,
            dir,
            config,
            &["apply", "--replace", "agent-a--project"],
        );
        assert_eq!(
            resumed.status.code(),
            Some(0),
            "the original approval must resume the replacement"
        );
        assert!(!stdin_free(&resumed).contains("access.close.v1"));
        let rows = listed(engine);
        assert_eq!(rows["tombstones"].as_array().unwrap().len(), 0);
        assert!(
            symbols
                .plan_text(case["recorded"]["text"].as_str().unwrap())
                .contains("flag=--replace agent-a--project")
        );
    })
    .await;
}

fn keep_interruption(dir: &Path, text: &str, mut ledger: Value, entries: Vec<Value>) {
    let local = dir.join(".conduit");
    let file = local.join("state.yaml");
    let digest = sha256(text);
    fs::create_dir_all(local.join("plans")).unwrap();
    fs::create_dir_all(local.join("results")).unwrap();
    fs::write(local.join(format!("plans/{digest}.txt")), text).unwrap();
    fs::write(
        local.join(format!("results/{digest}.yaml")),
        json!({"attempt":1,"entries":entries}).to_string(),
    )
    .unwrap();
    ledger["plans"]
        .as_array_mut()
        .unwrap()
        .push(json!(format!("sha256:{digest}")));
    for name in ["conduit.yaml", "s1.yaml", "nested.yaml"]
        .into_iter()
        .filter(|name| dir.join(name).is_file())
    {
        ledger["files"][name] = json!(sha256(&fs::read_to_string(dir.join(name)).unwrap()));
    }
    ledger["journals"][format!("sha256:{digest}")] =
        json!(format!(".conduit/results/{digest}.yaml"));
    fs::write(&file, ledger.to_string()).unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn nested_instance_replaced_under_a_new_name() {
    nested_replacement(false).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn review_248_nested_replacement_resumes_after_close() {
    nested_replacement(true).await;
}

async fn nested_replacement(interrupt: bool) {
    at_case("07-t7-revoke", |engine, dir, config, _, _| {
        let old = cases().into_iter().find(|case| case["id"] == "06-t6").unwrap();
        fs::write(dir.join("s1.yaml"), old["files"]["s1.yaml"].as_str().unwrap()).unwrap();
        let file = dir.join("nested.yaml");
        let mut template = framed("Conduit", "nested-grant", json!({
            "capability":"example.org/s1/filesystem#directory.v1",
            "operations":["example.org/s1/filesystem#file.read.v1"]
        }));
        let parent = framed("Interface", "operator--t", json!({
            "type":format!("{CORE}#interface.v1"),"conduits":{"project":{
                "from":"nested-grant","target":{"kind":"Resource","name":"shared"}}}
        }));
        let instance = json!({"apiVersion":API,"kind":"Interface","metadata":{"name":"operator"},"from":"operator--t"});
        let write = |template: &Value| {
            fs::write(&file, format!("{}\n---\n{}\n---\n{}\n",template,parent,instance)).unwrap();
        };
        write(&template);
        assert_eq!(command(engine, dir, config, &["apply"]).status.code(), Some(0), "seed declared Conduit");
        let view = succeeded(engine, "management.inspect.v1", json!({"object":{"kind":"Interface","name":"operator"}}));
        let old_child = view["declared"]["slots"]["conduits"]["project"]["id"].as_str().unwrap();
        let old_name = listed(engine)["objects"].as_array().unwrap()
            .iter().find(|o| o["id"] == old_child).unwrap()["name"].as_str().unwrap().to_owned();
        template["spec"]["operations"].as_array_mut().unwrap().push(json!("example.org/s1/filesystem#file.write.v1"));
        write(&template);
        if interrupt {
            let approved = command(engine, dir, config, &["plan", "--replace", &old_name]);
            assert_eq!(approved.status.code(), Some(0));
            let text = stdin_free(&approved);
            let before = succeeded(engine, "management.inspect.v1", json!({"object":{"kind":"Conduit","name":"nested-grant"}}));
            let replace_args = json!({
                "manifest":boundary::value(&dir.join("conduit.yaml")).unwrap(),
                "revision":before["revision"],"template":template
            });
            let replaced = succeeded(engine, "management.replace.v1", replace_args.clone());
            let args = json!({"conduit":{"kind":"Conduit","name":old_name},"expectedId":old_child});
            let reply = succeeded(engine, "access.close.v1", args.clone());
            let mut ledger = boundary::value(&dir.join(".conduit/state.yaml")).unwrap();
            let rows = ledger["objects"].as_array_mut().unwrap();
            rows.retain(|row| row[2] != old_child);
            let kept_template = rows.iter_mut().find(|row| row[2] == before["id"]).unwrap();
            kept_template[3] = replaced["revision"].clone();
            kept_template[5] = template["spec"].clone();
            keep_interruption(dir, text, ledger, vec![
                json!({"step":1,"tool":"management.replace.v1","arguments":replace_args,
                    "flag":null,"result":replaced}),
                json!({"step":2,"tool":"access.close.v1","arguments":args,
                    "flag":format!("--replace {old_name}"),"result":reply,
                    "remakes":[{"kind":"Conduit","name":old_name}]}),
            ]);
        }
        let output = command(engine, dir, config, &["apply", "--replace", &old_name]);
        let stops: Vec<_> = stdin_free(&output).lines()
            .filter(|line| line.starts_with("stop ") || line.starts_with("verify "))
            .collect();
        assert_eq!(output.status.code(), Some(0), "approved nested replacement should succeed: {stops:?}");
        assert!(stdin_free(&output).contains("management.register-replacement.v1 Interface/operator"));
        let updated = succeeded(engine, "management.inspect.v1", json!({"object":{"kind":"Interface","name":"operator"}}));
        assert_ne!(updated["declared"]["slots"]["conduits"]["project"]["id"], old_child);
    }).await;
}
