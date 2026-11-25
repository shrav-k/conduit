use super::*;
use crate::login_resource_tests::{complete, invoke, subject};

fn evidence(n: u32) -> Evidence {
    Evidence {
        time: "2026-11-24T12:00:00Z".into(),
        ids: (0..256).map(|i| format!("stop-{n}-{i}")).collect(),
        revisions: (0..64).map(|i| format!("stop-revision-{n}-{i}")).collect(),
        suffixes: vec![],
    }
}

fn input(state: &State, c: &Context, n: u32, mut value: Value) -> Transition {
    let e = evidence(n);
    value["evidence"] =
        json!({"time":e.time,"ids":e.ids,"revisions":e.revisions,"suffixes":e.suffixes});
    let parsed = serde_json::from_value(value).expect("stop input must be supported");
    step(state, parsed, c).unwrap()
}

fn project_files(files: &Value) -> packages::Project {
    let mut project = packages::Project {
        manifest: None,
        documents: vec![],
        directory: None,
    };
    for (name, alias) in files.as_object().unwrap() {
        macro_rules! fixture {
            ($file:literal) => {
                include_bytes!(concat!("../../../../", $file)).as_slice()
            };
        }
        let bytes = match alias.as_str().unwrap() {
            "filesystem-m" => fixture!("conformance/fixtures/s1/packages/filesystem/conduit.yaml"),
            "filesystem-d" => {
                fixture!("conformance/fixtures/s1/packages/filesystem/contracts.yaml")
            }
            "history-m" => fixture!("conformance/fixtures/s1/packages/history/conduit.yaml"),
            "history-d" => fixture!("conformance/fixtures/s1/packages/history/contracts.yaml"),
            "agents-m" => fixture!("conformance/fixtures/s1/packages/agents/conduit.yaml"),
            "agents-d" => fixture!("conformance/fixtures/s1/packages/agents/contracts.yaml"),
            "conduit.yaml" => fixture!("conformance/fixtures/s1/project/conduit.yaml"),
            "templates.yaml" => fixture!("conformance/fixtures/s1/project/templates.yaml"),
            "instances.yaml" => fixture!("conformance/fixtures/s1/project/instances.yaml"),
            _ => panic!("unknown S1 fixture file"),
        };
        for doc in conduit_loader::read_bytes(conduit_loader::Syntax::Yaml, bytes).unwrap() {
            if name == "conduit.yaml" {
                project.manifest = Some(doc.to_json());
            } else {
                project.documents.push(doc.to_json());
            }
        }
    }
    project
}

// Execute the actual stop case prefix, including its S1 package snapshots.
fn case_state(through: usize) -> (State, Context) {
    let c = crate::login_resource_tests::context();
    let case: Value = serde_json::from_str(include_str!(
        "../../../../conformance/conventions/requests/stop-takes-no-new-request/case.yaml"
    ))
    .unwrap();
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../conformance/language/registration/s1/fixture.json"
    ))
    .unwrap();
    let project = project_files(&fixture["project"]["files"]);
    let snapshots: Vec<packages::Snapshot> = fixture["snapshots"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| {
            let mut snapshot = s.clone();
            snapshot.as_object_mut().unwrap().remove("archive");
            snapshot["project"] =
                serde_json::to_value(project_files(&s["project"]["files"])).unwrap();
            serde_json::from_value(snapshot).unwrap()
        })
        .collect();
    fn substitute(v: &mut Value, captures: &BTreeMap<String, Value>, c: &Context) {
        match v {
            Value::String(s) if s.starts_with('$') => *v = captures[&s[1..]].clone(),
            Value::String(s) if s.starts_with("@core#") => {
                *s = s.replacen("@core", &c.core_package, 1)
            }
            Value::Object(m) => m.values_mut().for_each(|v| substitute(v, captures, c)),
            Value::Array(a) => a.iter_mut().for_each(|v| substitute(v, captures, c)),
            _ => {}
        }
    }
    let mut state = State::default();
    let mut captures = BTreeMap::new();
    for (n, s) in case["steps"]
        .as_array()
        .unwrap()
        .iter()
        .take(through)
        .enumerate()
    {
        let mut v = s["input"].clone();
        substitute(&mut v, &captures, &c);
        let i = match v["kind"].as_str().unwrap() {
            "resolve-project" => Input::ResolveProject {
                snapshots: snapshots.clone(),
                builtin: c.builtin.clone().unwrap(),
                project: project.clone(),
            },
            "register-project" => Input::RegisterProject {
                snapshots: snapshots.clone(),
                builtin: c.builtin.clone().unwrap(),
                project: project.clone(),
                evidence: serde_json::from_value(v["evidence"].clone()).unwrap(),
                request: serde_json::from_value(v["request"].clone()).unwrap(),
            },
            _ => serde_json::from_value(v).unwrap(),
        };
        let t = step(&state, i, &c).unwrap_or_else(|e| panic!("case step {}: {e}", n + 1));
        assert_ne!(t.reply["isError"], true, "case step {}", n + 1);
        for (name, pointer) in s["capture"].as_object().into_iter().flatten() {
            captures.insert(
                name.clone(),
                t.reply
                    .pointer(pointer.as_str().unwrap().strip_prefix("/reply").unwrap())
                    .unwrap()
                    .clone(),
            );
        }
        state = t.state;
    }
    (state, c)
}
fn ask(state: &State, c: &Context) -> Transition {
    input(state, c, 800, json!({"kind":"stop"}))
}
fn cut(state: &State, c: &Context) -> Transition {
    input(state, c, 801, json!({"kind":"stop","graceEnded":true}))
}
fn running(state: &State) -> &Value {
    state
        .invocations
        .iter()
        .find(|i| i["state"] == "in-progress")
        .unwrap()
}
fn diagnostic(reply: &Value) -> &str {
    reply["structuredContent"]["diagnostics"][0]["message"]["text"]
        .as_str()
        .unwrap()
}
fn unavailable(t: &Transition, c: &Context) {
    assert_eq!(t.reply["isError"], true);
    assert_eq!(
        t.reply["_meta"][format!("{}reason", c.meta_prefix)],
        "unavailable"
    );
    assert!(
        t.reply["_meta"]
            .get(format!("{}outcome", c.meta_prefix))
            .is_none()
    );
    assert_eq!(
        t.reply["structuredContent"]["diagnostics"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        t.reply["structuredContent"]["diagnostics"][0]["ruleId"],
        "OPS-037"
    );
    assert!(diagnostic(&t.reply).contains("started again"));
    assert_eq!(t.records.len(), 1);
    assert_eq!(t.records[0]["type"], "request-ended");
    assert_eq!(t.records[0]["data"]["ruleId"], "OPS-037");
    assert!(t.external_intents.is_empty());
}

#[test]
fn stop_turns_away_new_requests_unavailable_first() {
    let (state, c) = case_state(14);
    let root = subject(&state, "Interface", "root")["id"].clone();
    let agent = subject(&state, "Agent", "agent-a")["id"].clone();
    let stopped = ask(&state, &c);
    assert_eq!(stopped.reply, json!({"stopping":true}));
    assert_eq!(stopped.records.len(), 1);
    assert_eq!(stopped.records[0]["data"]["phase"], "asked");
    assert!(stopped.records[0]["data"].get("means").is_none());
    assert!(stopped.records[0]["data"].get("caller").is_none());
    for (who, tool, args) in [
        (
            &root,
            "management.inspect.v1",
            json!({"object":{"kind":"Pool","name":"system"}}),
        ),
        (
            &root,
            "management.inspect.v1",
            json!({"object":{"kind":"Resource","name":"absent"}}),
        ),
        (
            &agent,
            "project.file.write.v1",
            json!({"path":"a","content":"b"}),
        ),
    ] {
        unavailable(&invoke(&stopped.state, &c, 900, who, tool, args), &c);
    }
    for who in [&root, &json!("unestablished")] {
        let request = json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":who},"tool":"unknown","arguments":{},"readable":false}});
        let before = input(&state, &c, 901, request.clone());
        let after = input(&stopped.state, &c, 901, request);
        assert_eq!(before.reply, after.reply);
        assert_eq!(before.records, after.records);
        assert_eq!(
            serde_json::to_value(before.decisions).unwrap(),
            serde_json::to_value(after.decisions).unwrap()
        );
    }
    let request = json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":"unknown","arguments":{},"authority_readable":false}});
    unavailable(&input(&stopped.state, &c, 902, request), &c);
    let raw = json!({"kind":"access","request":{"identity":{"kind":"instance","id":root},"tool":"management.inspect.v1","arguments":{}}});
    let denied = input(&stopped.state, &c, 903, raw);
    unavailable(&denied, &c);
    assert_eq!(denied.records[0]["data"]["caller"], root);
    let request = json!({"sender":root,"caller":root,"conduit":"absent","target":"absent","operation":c.qualified("inspect.v1")});
    for raw in [
        json!({"kind":"request","request":request,"object":"absent"}),
        json!({"kind":"submit","request":request,"documents":[]}),
        json!({"kind":"register-project","request":request,"snapshots":[],"builtin":c.builtin,"project":{"manifest":null,"documents":[],"directory":null}}),
        json!({"kind":"invoke","key":"","request":{"identity":{"kind":"instance","id":root},"tool":"unknown","arguments":{}}}),
    ] {
        unavailable(&input(&stopped.state, &c, 904, raw), &c);
    }
    let list = json!({"kind":"list-tools","identity":{"kind":"instance","id":root}});
    let list_before = step(&state, serde_json::from_value(list.clone()).unwrap(), &c).unwrap();
    let list_after = step(&stopped.state, serde_json::from_value(list).unwrap(), &c).unwrap();
    assert_eq!(list_before.reply, list_after.reply);
    let again = input(&stopped.state, &c, 803, json!({"kind":"stop"}));
    assert!(again.records.is_empty());
    assert_eq!(again.state, stopped.state);
}

#[test]
fn stop_takes_the_work_in_progress_and_its_repeats() {
    let mut project = crate::login_resource_tests::project(
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/conduit.json"
        ),
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/documents.json"
        ),
    );
    // The fixture's Agent has no outward Conduit. Give a the existing status operation on b.
    let api = project.manifest.as_ref().unwrap()["apiVersion"].clone();
    project.documents.push(json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"agent-read"},"from":"turns","consumer":{"kind":"Agent","name":"a"},"target":{"kind":"Agent","name":"b"}}));
    let case = include_str!("../../../../conformance/conventions/requests/limits-latest/case.yaml");
    let (state, c, _) = crate::login_resource_tests::run_case(case, project, 10);
    let root = subject(&state, "Interface", "root")["id"].clone();
    let a = subject(&state, "Agent", "a")["id"].clone();
    let b = subject(&state, "Agent", "b")["id"].clone();
    let opened = invoke(
        &state,
        &c,
        910,
        &root,
        "access.open.v1",
        json!({"conduit":"agent-read"}),
    );
    assert_eq!(opened.reply["isError"], false);
    let kept_request = json!({"kind":"invoke","key":"kept-key","request":{"identity":{"kind":"instance","id":root},"tool":"call-b.status.v1","arguments":{}}});
    let keyed = input(&opened.state, &c, 911, kept_request.clone());
    let done = complete(
        &keyed.state,
        &c,
        912,
        &keyed.reply["invocation"],
        json!({"isError":false,"structuredContent":{}}),
    );
    assert_eq!(
        done.reply["_meta"][format!("{}outcome", c.meta_prefix)],
        "succeeded"
    );
    let turn = invoke(&done.state, &c, 913, &root, "call-a.turn.v1", json!({}));
    assert_eq!(turn.external_intents.len(), 1);
    let stopped = ask(&turn.state, &c);
    let call = invoke(
        &stopped.state,
        &c,
        914,
        &a,
        "agent-read.status.v1",
        json!({}),
    );
    assert_eq!(call.external_intents.len(), 1);
    let ordinary = invoke(&turn.state, &c, 914, &a, "agent-read.status.v1", json!({}));
    assert_eq!(call.external_intents, ordinary.external_intents);
    unavailable(
        &invoke(
            &stopped.state,
            &c,
            915,
            &b,
            "agent-read.status.v1",
            json!({}),
        ),
        &c,
    );
    let before = input(&turn.state, &c, 916, kept_request.clone());
    let after = input(&stopped.state, &c, 916, kept_request);
    assert_eq!(before.reply, after.reply);
    assert!(after.external_intents.is_empty());
    let respond = json!({"kind":"respond","invocation":turn.reply["invocation"],"attempt":1,"identity":{"kind":"instance","id":root},"responses":{}});
    assert_eq!(
        input(&turn.state, &c, 917, respond.clone()).reply,
        input(&stopped.state, &c, 917, respond).reply
    );
}

#[test]
fn stop_ends_when_the_last_call_ends() {
    let (state, c) = case_state(14);
    let a = subject(&state, "Agent", "agent-a")["id"].clone();
    let first = running(&state)["id"].clone();
    let second = invoke(
        &state,
        &c,
        920,
        &a,
        "project.file.read.v1",
        json!({"path":"second.txt"}),
    );
    let second_id = second.reply["invocation"].clone();
    let stopped = ask(&second.state, &c);
    let first = complete(&stopped.state, &c, 921, &first, json!({"isError":false}));
    assert!(first.records.iter().all(|r| r["type"] != "engine-stop"));
    let last = complete(&first.state, &c, 922, &second_id, json!({"isError":false}));
    assert_eq!(last.records.last().unwrap()["data"]["phase"], "stopped");
    assert_eq!(last.records.last().unwrap()["data"]["cut"], false);
    assert_eq!(cut(&last.state, &c).reply, json!({"stopped":true,"cut":[]}));
    let (idle, c) = case_state(13);
    let stopped = ask(&idle, &c);
    assert_eq!(
        stopped
            .records
            .iter()
            .map(|r| r["data"]["phase"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["asked", "stopped"]
    );
}

#[test]
fn stop_grace_end_cuts_calls_as_cancel_ends_them() {
    for through in [6, 14] {
        let (state, c) = case_state(through); // S1's shared filesystem setup at step 6.
        let id = running(&state)["id"].clone();
        let cancelled = input(&state, &c, 930, json!({"kind":"cancel","invocation":id}));
        let stopped = cut(&ask(&state, &c).state, &c);
        assert_eq!(cancelled.external_intents, stopped.external_intents);
        assert_eq!(
            cancelled.reply["_meta"][format!("{}outcome", c.meta_prefix)],
            stopped.reply["cut"][0]["_meta"][format!("{}outcome", c.meta_prefix)]
        );
        assert_eq!(
            stopped.reply["cut"][0]["structuredContent"]["diagnostics"][0]["ruleId"],
            "OPS-037"
        );
    }
    let (mut state, c) = case_state(14);
    let a = subject(&state, "Agent", "agent-a")["id"].clone();
    state
        .invocations
        .iter_mut()
        .find(|i| i["state"] == "in-progress")
        .unwrap()["key"] = json!("private-repeat-value");
    let second = invoke(
        &state,
        &c,
        931,
        &a,
        "project.file.read.v1",
        json!({"path":"second.txt"}),
    );
    let stopped = cut(&second.state, &c);
    let results = stopped.reply["cut"].as_array().unwrap();
    assert_eq!(results.len(), 2);
    for r in results {
        assert_eq!(r["isError"], true);
        assert_eq!(r["_meta"][format!("{}outcome", c.meta_prefix)], "unknown");
        assert!(r["_meta"].get(format!("{}reason", c.meta_prefix)).is_none());
        assert!(diagnostic(r).contains("engine stopped before the call ended"));
        assert!(diagnostic(r).contains("effect may have occurred"));
        assert!(!diagnostic(r).contains("private-repeat-value"));
    }
    assert!(diagnostic(&results[0]).contains("same idempotency key"));
    assert!(diagnostic(&results[0]).contains("started again"));
    assert!(diagnostic(&results[1]).contains("reconcile"));
    assert!(diagnostic(&results[1]).contains("rather than make a new request"));
    assert_eq!(stopped.request_replies.len(), 2);
    assert_eq!(stopped.records[0]["data"]["phase"], "asked");
    for record in stopped.records.iter().filter(|r| {
        matches!(
            r["type"].as_str(),
            Some("invocation-ended" | "request-ended")
        )
    }) {
        assert_eq!(record["data"]["ruleId"], "OPS-037");
    }
    let (mut state, c) = case_state(13);
    let provider = subject(&state, "Resource", "shared")["id"].clone();
    let caller = subject(&state, "Interface", "root")["id"].clone();
    state.invocations.push(json!({"id":"restart","attempt":1,"state":"in-progress","object":provider,"caller":caller,"setup":{"item":{}},"provider_restart":{"setup":"earlier-setup","phase":"run"}}));
    let asked = ask(&state, &c);
    for identity in ["none", "unreadable"] {
        let request = json!({"kind":"invoke","request":{"identity":{"kind":identity},"tool":"management.inspect.v1","arguments":{}}});
        unavailable(&input(&asked.state, &c, 939, request), &c);
    }
    let stopped = cut(&asked.state, &c);
    assert!(
        stopped
            .state
            .invocations
            .iter()
            .all(|v| v["state"] != "in-progress")
    );
    assert_eq!(
        stopped.external_intents,
        json!([{"kind":"cancel-setup-parts","invocation":"restart","subject":provider}])
            .as_array()
            .unwrap()
            .clone()
    );
    assert!(
        stopped.request_replies.is_empty(),
        "a provider restart is not a tool invocation"
    );
    assert_eq!(stopped.records.last().unwrap()["data"]["cut"], true);
}

#[test]
fn stop_grace_end_settles_retirements_without_a_report() {
    for phase in ["stop", "hook", "container", "waiting"] {
        let (mut state, c) = case_state(13);
        let object = subject(&state, "Agent", "agent-a")["id"].clone();
        let root = subject(&state, "Interface", "root")["id"].clone();
        let target = subject(&state, "Fleet", "s1")["id"].clone();
        let subject = state
            .objects
            .iter_mut()
            .find(|v| v["id"] == object)
            .unwrap();
        subject["state"] = json!("retired");
        subject["resolved_implementations"] = json!({"test":{"item":{"command":["scripted"]}}});
        let mut retirement = json!({"id":"retirement","attempt":1,"object":object,"target":target,"sender":root,"caller":root,"state":"in-progress","operation":c.qualified("retire.v1"),"contract":{"spec":{"effect":"retire"}},"retirement":{"phase":if matches!(phase,"container" | "waiting") {"stop"} else {phase},"parts":[]}});
        if phase == "waiting" {
            retirement["provider_wait"] = json!({"restart":"restart","intent":{"kind":"provider-call","operation":c.qualified("stop.v1"),"invocation":"retirement","provider":"runtime","subject":object,"inputs":{}}});
        }
        if phase == "container" {
            retirement["cascade_parent"] = json!("container");
            state.invocations.push(json!({"id":"container","attempt":1,"object":target,"target":target,"sender":root,"caller":root,"state":"in-progress","operation":c.qualified("retire-with-agents.v1"),"cascade":{"phase":"members","parts":[{"id":object,"kind":"Agent","name":"a","status":"in-progress","invocation":"retirement"}]}}));
        }
        state.invocations.push(retirement);
        let stopped = cut(&ask(&state, &c).state, &c);
        assert!(
            stopped
                .state
                .invocations
                .iter()
                .all(|v| v["state"] != "in-progress")
        );
        let stops = stopped
            .external_intents
            .iter()
            .filter(|v| v["operation"] == c.qualified("stop.v1"))
            .count();
        assert_eq!(stops, usize::from(matches!(phase, "hook" | "waiting")));
        for reply in &stopped.request_replies {
            assert_eq!(
                reply.reply["_meta"][format!("{}outcome", c.meta_prefix)],
                if reply.invocation == "container" {
                    "failed"
                } else {
                    "unknown"
                }
            );
            assert_eq!(
                reply.reply["structuredContent"]["diagnostics"][0]["ruleId"],
                "OPS-037"
            );
        }
        assert_eq!(
            stopped.reply["cut"].as_array().unwrap().len(),
            if phase == "container" { 2 } else { 1 }
        );
        assert_eq!(stopped.records.last().unwrap()["data"]["phase"], "stopped");
        let late = input(
            &stopped.state,
            &c,
            940,
            json!({"kind":"setup-report","invocation":"retirement","report":{"kind":"stop","outcome":"succeeded"}}),
        );
        assert_eq!(late.state, stopped.state);
        assert!(late.records.is_empty());
    }
}

#[test]
fn stop_is_forgotten_at_restart() {
    for through in [13, 14] {
        let (state, c) = case_state(through);
        let asked = ask(&state, &c);
        let recovered = input(&asked.state, &c, 950, json!({"kind":"recover"}));
        let ordinary = input(&state, &c, 950, json!({"kind":"recover"}));
        assert!(
            serde_json::to_value(&recovered.state)
                .unwrap()
                .get("stop")
                .is_none()
        );
        assert_eq!(recovered.records, ordinary.records);
        let mut normalized = recovered.state.clone();
        normalized.records.retain(|r| r["type"] != "engine-stop");
        for record in &asked.records {
            normalized
                .allocated_ids
                .remove(record["id"].as_str().unwrap());
        }
        assert_eq!(
            serde_json::to_vec(&normalized).unwrap(),
            serde_json::to_vec(&ordinary.state).unwrap()
        );
        let root = subject(&state, "Interface", "root")["id"].clone();
        let args = json!({"object":{"kind":"Pool","name":"system"}});
        assert_eq!(
            invoke(
                &ordinary.state,
                &c,
                951,
                &root,
                "management.inspect.v1",
                args.clone()
            )
            .reply["isError"],
            invoke(
                &recovered.state,
                &c,
                951,
                &root,
                "management.inspect.v1",
                args
            )
            .reply["isError"]
        );
    }
}

fn provider_stop_fixture(unregister: bool, made: bool) -> (State, Context) {
    let (mut t, c, _) = crate::retirement::tests::fixture(false, None);
    if unregister {
        t.state.objects[0]["state"] = json!("ready");
    }
    t.state
        .objects
        .push(json!({"id":"system","from":c.qualified("system")}));
    let v = &mut t.state.invocations[1];
    v["operation"] = json!(c.qualified(if unregister {
        "unregister.v1"
    } else {
        "retire.v1"
    }));
    v["retirement"] = json!({"phase":"stop","parts":[],"unregister":unregister});
    v["provider_call"] = json!({"invocation":"parent","target":"subject","provider":"runtime","operation":c.qualified("stop.v1"),"inputs":{},"made":made});
    if !made {
        v["provider_wait"] = json!({"restart":"restart","intent":{"kind":"provider-call","operation":c.qualified("stop.v1"),"invocation":"parent","provider":"runtime","subject":"subject","inputs":{}}});
    }
    (t.state, c)
}

fn check_cut_provider_stop(unregister: bool) {
    for made in [true, false] {
        let (state, c) = provider_stop_fixture(unregister, made);
        let stopped = cut(&ask(&state, &c).state, &c);
        let reply = &stopped
            .request_replies
            .iter()
            .find(|r| r.invocation == "parent")
            .unwrap()
            .reply;
        assert_eq!(
            reply["_meta"][format!("{}outcome", c.meta_prefix)],
            "unknown"
        );
        assert_eq!(reply["isError"], true);
        assert_eq!(
            reply["structuredContent"]["diagnostics"][0]["ruleId"],
            "OPS-037"
        );
        assert_eq!(stopped.reply["cut"][0], *reply);
        assert_eq!(
            stopped
                .state
                .objects
                .iter()
                .find(|o| o["id"] == "subject")
                .unwrap()["shutdown"],
            "unknown"
        );
        let records: Vec<_> = stopped
            .records
            .iter()
            .filter(|r| r["type"] == "provider-call")
            .collect();
        assert_eq!(records.len(), if made { 1 } else { 2 });
        if !made {
            assert_eq!(records[0]["data"]["phase"], "made");
        }
        let ended = records.last().unwrap();
        assert_eq!(ended["data"]["phase"], "ended");
        assert_eq!(ended["data"]["outcome"], "unknown");
        assert_eq!(ended["data"]["ruleId"], "OPS-037");
        assert_eq!(
            stopped
                .external_intents
                .iter()
                .filter(|v| v["operation"] == c.qualified("stop.v1"))
                .count(),
            usize::from(!made)
        );
        assert_eq!(stopped.records.last().unwrap()["type"], "engine-stop");
        let late = input(
            &stopped.state,
            &c,
            980,
            json!({"kind":"setup-report","invocation":"parent","report":{"kind":"stop","outcome":"succeeded"}}),
        );
        assert_eq!(late.state, stopped.state);
        assert!(late.records.is_empty());
    }
}

#[test]
fn cut_unregistration_keeps_unknown_and_ignores_late_stop_report() {
    check_cut_provider_stop(true);
}

#[test]
fn cut_retirement_keeps_provider_records_and_unknown_shutdown() {
    check_cut_provider_stop(false);
}

#[test]
fn stop_ask_ends_undispatched_discovery_unavailable() {
    let (mut state, c) = case_state(14);
    let i = state
        .invocations
        .iter()
        .position(|v| v["state"] == "in-progress")
        .unwrap();
    let id = state.invocations[i]["id"].clone();
    state.invocations[i]["discovery"] = json!("pending");
    let asked = ask(&state, &c);
    let reply = &asked
        .request_replies
        .iter()
        .find(|r| r.invocation == id.as_str().unwrap())
        .expect("undispatched discovery must end at the ask")
        .reply;
    assert_eq!(
        reply["_meta"][format!("{}reason", c.meta_prefix)],
        "unavailable"
    );
    assert!(
        reply["_meta"]
            .get(format!("{}outcome", c.meta_prefix))
            .is_none()
    );
    assert_eq!(
        reply["structuredContent"]["diagnostics"][0]["ruleId"],
        "OPS-037"
    );
    assert!(asked.external_intents.is_empty());
    assert!(!asked.records.iter().any(|r| r["type"] == "access-decision"));
    assert_eq!(asked.state.stop, Some(crate::stop::StopState::Stopped));
    assert!(!asked.state.invocations.iter().any(|v| v["id"] == id));
}
