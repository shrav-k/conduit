use super::*;
use crate::login_resource_tests::{invoke, subject};

const CASE: &str =
    include_str!("../../../../conformance/conventions/access/reserved-package-operation/case.yaml");
const ADDRESS: &str = "example.org/conformance/reserved-operation";

fn fixture() -> (State, Context) {
    let c = crate::login_resource_tests::context();
    let case: Value = serde_json::from_str(CASE).unwrap();
    let started = step(
        &State::default(),
        serde_json::from_value(case["steps"][0]["input"].clone()).unwrap(),
        &c,
    )
    .unwrap();
    let mut project = crate::login_resource_tests::project(
        include_str!(
            "../../../../conformance/conventions/access/reserved-package-operation/initial-manifest.json"
        ),
        include_str!(
            "../../../../conformance/conventions/access/reserved-package-operation/initial-documents.json"
        ),
    );
    let api = project.documents[0]["apiVersion"].clone();
    project.documents.extend([
        json!({"apiVersion":api,"kind":"Operation","metadata":{"name":"purge.v1"},"spec":{"inputSchema":{"type":"object"}}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"repo-purge"},"spec":{"capability":"#repo-access.v1","operations":["#purge.v1"]}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"repo-input"},"spec":{"capability":"#repo-access.v1","inputs":{"ops":{"schema":{"type":"array","items":{"type":"string"}},"to":["/operations"]}}}}),
        json!({"apiVersion":api,"kind":"Type","metadata":{"name":"worker.v1"},"spec":{"family":"Agent","schema":{"type":"object"},"offers":[],"requires":[]}}),
        json!({"apiVersion":api,"kind":"Agent","metadata":{"name":"worker-template"},"spec":{"type":"#worker.v1","config":{},"conduits":{"repo":{"from":"repo-status","target":{"kind":"Resource","name":"repo"}}}}}),
        json!({"apiVersion":api,"kind":"Fleet","metadata":{"name":"workers"},"from":c.qualified("fleet")}),
        json!({"apiVersion":api,"kind":"Agent","metadata":{"name":"worker"},"from":"worker-template","owner":"workers"}),
    ]);
    for doc in &mut project.documents {
        if doc["metadata"]["name"] == "repo-access.v1" {
            doc["spec"]["operations"]
                .as_array_mut()
                .unwrap()
                .push(json!("#purge.v1"));
        }
        if doc["metadata"]["name"] == "repo.v1" {
            doc["spec"]["implementation"]["#purge.v1"] = json!("#scripted-repo");
        }
    }
    let item = &mut project.manifest.as_mut().unwrap()["implementations"]["scripted-repo"];
    item["implements"]
        .as_array_mut()
        .unwrap()
        .push(json!("#purge.v1"));
    item["tools"]["#purge.v1"] = json!("purge.v1");
    let input = &case["steps"][1]["input"];
    let mut evidence: Evidence = serde_json::from_value(input["evidence"].clone()).unwrap();
    evidence
        .ids
        .extend((200..300).map(|n| format!("42100000-0000-4000-8000-{n:012}")));
    evidence
        .revisions
        .extend((30..60).map(|n| format!("initial-{n}")));
    let mut request: Request = serde_json::from_value(input["request"].clone()).unwrap();
    request.operation = c.qualified("register.v1");
    let t = step(
        &started.state,
        Input::RegisterProject {
            snapshots: vec![],
            builtin: c.builtin.clone().unwrap(),
            project,
            evidence,
            request,
        },
        &c,
    )
    .unwrap();
    assert_eq!(t.reply["accepted"], true, "{}", t.reply);
    (t.state, c)
}

fn root_call(s: &State, c: &Context, serial: u32, tool: &str, args: Value) -> Transition {
    invoke(
        s,
        c,
        serial,
        &subject(s, "Interface", "root")["id"],
        tool,
        args,
    )
}

fn document(c: &Context, template: &str, consumer: &str) -> Value {
    json!({"apiVersion":c.builtin.as_ref().unwrap().manifest.as_ref().unwrap()["apiVersion"],
        "kind":"Conduit","metadata":{"name":"candidate"},"from":template,
        "consumer":{"kind":"Interface","name":consumer},"target":{"kind":"Resource","name":"repo"}})
}

fn diagnostic(t: &Transition) -> &Value {
    if t.reply.get("diagnostics").is_some() {
        &t.reply["diagnostics"][0]
    } else if t.reply.get("error").is_some() {
        &t.reply["error"]["data"]["diagnostics"][0]
    } else {
        &t.reply["structuredContent"]["diagnostics"][0]
    }
}

fn refused(s: &State, t: &Transition, location: Option<&str>) {
    assert_eq!(diagnostic(t)["ruleId"], "ACC-032", "{}", t.reply);
    assert_eq!(diagnostic(t)["level"], "error");
    assert!(diagnostic(t)["properties"]["refusal"].is_object());
    let message = diagnostic(t)["message"]["text"].as_str().unwrap();
    assert!(message.contains("no Conduit whose consumer is not root"));
    assert!(message.contains("If your task needs it, say so in your reply."));
    let diagnostics = t
        .reply
        .get("diagnostics")
        .or_else(|| t.reply.pointer("/structuredContent/diagnostics"))
        .or_else(|| t.reply.pointer("/error/data/diagnostics"))
        .unwrap();
    assert_eq!(diagnostics.as_array().unwrap().len(), 1);
    assert!(t.decisions.iter().any(|decision| {
        decision.promises.iter().any(|rule| rule == "ACC-032")
            && decision.promises.iter().any(|rule| rule == "OPS-023")
    }));
    if let Some(location) = location {
        assert_eq!(
            diagnostic(t)["locations"][0]["logicalLocations"][0]["fullyQualifiedName"],
            location
        );
    }
    assert_eq!(s.objects, t.state.objects);
    assert!(t.external_intents.is_empty());
    assert!(
        t.records
            .iter()
            .any(|r| r["type"] == "access-decision" && r["data"]["reason"] == "refused")
    );
    assert!(
        t.records
            .iter()
            .any(|r| r["type"] == "request-ended" && r["data"]["reason"] == "refused")
    );
}

#[test]
fn reserved_sibling_entry_refused_before_duplicate_documents() {
    let (s, c) = fixture();
    let api = c.builtin.as_ref().unwrap().manifest.as_ref().unwrap()["apiVersion"].clone();
    let ty = json!({"apiVersion":api,"kind":"Type","metadata":{"name":"sibling-client.v1"},
        "spec":{"family":"Resource","schema":{"type":"object"},"offers":[],"requires":[]}});
    let manifest = json!({"apiVersion":api,"address":"example.org/conformance/reserved-siblings"});
    let mut client = json!({"apiVersion":api,"kind":"Resource","metadata":{"name":"sibling-client"},
        "spec":{"type":"#sibling-client.v1","config":{},
            "inputs":{"peer":{"schema":{"$ref":c.qualified("resource-reference.v1")},"to":["/conduits/commit/target"]}},
            "conduits":{"commit":{"from":"repo-commit"}}}});
    let pool = json!({"apiVersion":api,"kind":"Pool","metadata":{"name":"sibling-pool"},
        "spec":{"type":c.qualified("pool.v1"),"config":{},"entries":{
            "repo":{"from":"repo-template"},"client":{"from":"sibling-client","arguments":{"peer":{"entry":"repo"}}}}}});
    let instance = json!({"apiVersion":api,"kind":"Pool","metadata":{"name":"siblings"},"from":"sibling-pool"});
    let duplicate = json!({"apiVersion":api,"kind":"Interface","metadata":{"name":"unrelated"},"from":c.qualified("interface")});
    let t = root_call(
        &s,
        &c,
        150,
        "management.register.v1",
        json!({"manifest":manifest,"documents":[ty.clone(),client.clone(),pool.clone(),instance.clone(),duplicate.clone(),duplicate.clone()]}),
    );
    refused(&s, &t, Some("/objects/sibling-client/conduits/commit"));
    client["spec"]["conduits"]["commit"]["from"] = json!("repo-status");
    let t = root_call(
        &s,
        &c,
        151,
        "management.register.v1",
        json!({"manifest":manifest,"documents":[ty.clone(),client.clone(),pool.clone(),instance.clone(),duplicate.clone(),duplicate]}),
    );
    assert_eq!(diagnostic(&t)["ruleId"], "SUB-004");
    let t = root_call(
        &s,
        &c,
        152,
        "management.register.v1",
        json!({"manifest":manifest,"documents":[ty,client,pool,instance]}),
    );
    assert_eq!(
        t.reply["_meta"][format!("{}outcome", c.meta_prefix)],
        "succeeded",
        "{}",
        t.reply
    );
}

#[test]
fn reserved_conduit_refused_at_registration() {
    let (s, c) = fixture();
    let reserved = document(&c, "repo-commit", "actor");
    // Resolve both the target and its marked contract in this same submission.
    let case: Value = serde_json::from_str(CASE).unwrap();
    let started = step(
        &State::default(),
        serde_json::from_value(case["steps"][0]["input"].clone()).unwrap(),
        &c,
    )
    .unwrap();
    let mut project = crate::login_resource_tests::project(
        include_str!(
            "../../../../conformance/conventions/access/reserved-package-operation/initial-manifest.json"
        ),
        include_str!(
            "../../../../conformance/conventions/access/reserved-package-operation/initial-documents.json"
        ),
    );
    project.documents.push(reserved.clone());
    let mut request: Request =
        serde_json::from_value(case["steps"][1]["input"]["request"].clone()).unwrap();
    request.operation = c.qualified("register.v1");
    let mut invalid_target = project.clone();
    invalid_target
        .documents
        .iter_mut()
        .find(|doc| doc["metadata"]["name"] == "repo.v1")
        .unwrap()["spec"]["controls"] = json!(["#missing-control.v1"]);
    for project in [project, invalid_target] {
        let t = step(
            &started.state,
            Input::RegisterProject {
                project,
                builtin: c.builtin.clone().unwrap(),
                snapshots: vec![],
                request: request.clone(),
                evidence: serde_json::from_value(case["steps"][1]["input"]["evidence"].clone())
                    .unwrap(),
            },
            &c,
        )
        .unwrap();
        assert_eq!(t.reply["accepted"], false);
        refused(&started.state, &t, Some("/objects/candidate"));
    }
    let mut core = document(&c, "repo-commit", "actor");
    core["from"] = json!(c.qualified("root-management"));
    core["target"]["name"] = json!("management");
    let api = reserved["apiVersion"].clone();
    let template = json!({"apiVersion":api,"kind":"Interface","metadata":{"name":"committer"},
        "spec":{"type":c.qualified("interface.v1"),"config":{},"conduits":{"commit":{"from":"repo-commit","target":{"kind":"Resource","name":"repo"}}}}});
    let instance = json!({"apiVersion":api,"kind":"Interface","metadata":{"name":"committer-1"},"from":"committer"});
    let mut input = document(&c, "repo-input", "actor");
    input["arguments"] = json!({"ops":[format!("{ADDRESS}#commit.v1")]});
    for (i, (documents, location)) in [
        (json!([reserved.clone()]), "/objects/candidate"),
        (json!([core]), "/objects/candidate"),
        (json!([template.clone(), instance]), "/objects/committer/conduits/commit"),
        (json!([input]), "/objects/candidate"),
        (json!([reserved, {"apiVersion":api,"kind":"Resource","metadata":{"name":"invalid"},"from":"absent"}]), "/objects/candidate"),
    ].into_iter().enumerate() {
        let t = root_call(&s, &c, 100 + i as u32, "management.register.v1", json!({"documents":documents}));
        refused(&s, &t, Some(location));
    }
    for (i, docs) in [
        json!([template]),
        json!([document(&c, "repo-status", "actor")]),
        json!([document(&c, "repo-purge", "actor")]),
        json!([document(&c, "repo-commit", "root")]),
    ]
    .into_iter()
    .enumerate()
    {
        let t = root_call(
            &s,
            &c,
            110 + i as u32,
            "management.register.v1",
            json!({"documents":docs}),
        );
        assert_eq!(t.reply["isError"], false, "{}", t.reply);
    }
    // An unresolved local selection must not borrow the target's marked namesake.
    let mut unresolved = document(&c, "foreign-link", "actor");
    unresolved["metadata"]["name"] = json!("unresolved");
    let t = root_call(
        &s,
        &c,
        119,
        "management.register.v1",
        json!({"documents":[
            {"apiVersion":api,"kind":"Conduit","metadata":{"name":"foreign-link"},
             "spec":{"capability":c.qualified("object-management.v1"),"operations":["#commit.v1"]}},
            unresolved
        ]}),
    );
    assert_eq!(
        t.reply["_meta"][format!("{}reason", c.meta_prefix)],
        "invalid"
    );
    assert_ne!(diagnostic(&t)["ruleId"], "ACC-032");
    let mut s = s;
    let worker = subject(&s, "Agent", "worker")["id"].clone();
    for o in &mut s.objects {
        if o["nested"]["parent"] == worker {
            o["state"] = json!("retired");
        }
        if o["kind"] == "Conduit" && o["name"] == "repo-status" && o.get("spec").is_some() {
            o["spec"]["operations"] = json!([format!("{ADDRESS}#commit.v1")]);
        }
    }
    let t = root_call(
        &s,
        &c,
        120,
        "management.register-replacement.v1",
        json!({"instance":{"kind":"Agent","name":"worker"},"nested":"repo"}),
    );
    refused(&s, &t, Some("/objects/worker/conduits/repo"));
}

fn worker_path(s: &State) -> Value {
    let worker = &subject(s, "Agent", "worker")["id"];
    s.objects
        .iter()
        .find(|o| o["kind"] == "Conduit" && access::id(&o["consumer"]) == worker)
        .unwrap()
        .clone()
}

#[test]
fn reserved_package_operation_refused_at_open() {
    let (initial, c) = fixture();
    for (i, op) in ["commit", "status", "purge"].into_iter().enumerate() {
        let mut s = initial.clone();
        let path = worker_path(&s);
        s.objects
            .iter_mut()
            .find(|o| o["id"] == path["id"])
            .unwrap()["operations"] = json!([format!("{ADDRESS}#{op}.v1")]);
        let t = root_call(
            &s,
            &c,
            200 + i as u32,
            "access.open.v1",
            json!({"conduit":path["name"]}),
        );
        if op == "commit" {
            refused(&s, &t, None);
        } else {
            assert_eq!(t.reply["isError"], false, "{}", t.reply);
        }
    }
    let opened = root_call(
        &initial,
        &c,
        210,
        "access.open.v1",
        json!({"conduit":"actor-access"}),
    );
    assert_eq!(opened.reply["isError"], false, "{}", opened.reply);
    let mut s = opened.state;
    s.objects
        .iter_mut()
        .find(|o| o["name"] == "actor-status")
        .unwrap()["operations"] = json!([format!("{ADDRESS}#commit.v1")]);
    let t = invoke(
        &s,
        &c,
        211,
        &subject(&s, "Interface", "actor")["id"],
        "actor-access.open.v1",
        json!({"conduit":"actor-status"}),
    );
    refused(&s, &t, None);
    let t = root_call(
        &s,
        &c,
        212,
        "access.open.v1",
        json!({"conduit":"root-repo"}),
    );
    assert_eq!(t.reply["isError"], false, "{}", t.reply);
    // The sixth tier must not depend on which restricted operation is listed first.
    s.objects
        .iter_mut()
        .find(|o| o["name"] == "actor-status")
        .unwrap()["operations"] =
        json!([format!("{ADDRESS}#commit.v1"), c.qualified("get-secret.v1")]);
    let t = invoke(
        &s,
        &c,
        213,
        &subject(&s, "Interface", "actor")["id"],
        "actor-access.open.v1",
        json!({"conduit":"actor-status"}),
    );
    assert_eq!(diagnostic(&t)["ruleId"], "SEC-008");
}

#[test]
fn reserved_package_operation_refused_at_a_setups_open() {
    let (mut s, c) = fixture();
    let path = worker_path(&s);
    for o in &mut s.objects {
        if o["id"] == path["id"] {
            o["operations"] = json!([format!("{ADDRESS}#commit.v1")]);
        }
        if o["name"] == "workers" {
            o["state"] = json!("ready");
        }
    }
    let t = root_call(
        &s,
        &c,
        300,
        "workers.set-up.v1",
        json!({"instance":{"kind":"Agent","name":"worker"}}),
    );
    assert!(
        t.records
            .iter()
            .any(|r| r["type"] == "access-decision" && r["data"]["ruleId"] == "ACC-032"),
        "{}",
        t.reply
    );
    let kept = t
        .state
        .objects
        .iter()
        .find(|o| o["id"] == path["id"])
        .unwrap();
    assert_eq!(kept["state"], "registered");
    assert_ne!(kept["open"], true);
}

#[test]
fn reserved_package_operation_refused_but_to_roots_own_requests() {
    let (mut s, c) = crate::explain::tests::fixture();
    let mut env = c.core_environment.clone();
    let mut project = env.scopes["core"].clone();
    project.address = ADDRESS.into();
    project.contracts.retain(|name, _| name == "type.v1");
    for name in ["commit.v1", "status.v1", "purge.v1"] {
        project.contracts.insert(name.into(),json!({"kind":"Operation","metadata":{"name":name},"spec":{"inputSchema":{"type":"object"},"reservedToRoot":name == "commit.v1"}}));
    }
    project.contracts.insert("repo-access.v1".into(), json!({"kind":"Capability","metadata":{"name":"repo-access.v1"},"spec":{"operations":["#commit.v1","#status.v1","#purge.v1"]}}));
    project.contracts.get_mut("type.v1").unwrap()["spec"]["offers"] = json!(["#repo-access.v1"]);
    env.scopes.insert("project".into(), project);
    s.objects.push(json!({"kind":"Resource","id":"repo","name":"repo","type":format!("{ADDRESS}#type.v1"),"state":"ready","config":{},"schema_environment":env,"contract_scope":"project"}));
    for sender in ["agent", "root"] {
        s.objects.push(json!({"kind":"Conduit","id":format!("{sender}-repo"),"name":format!("{sender}-repo"),"consumer":sender,"target":"repo","local_name":format!("{sender}-repo"),"state":"ready","open":true,"capability":format!("{ADDRESS}#repo-access.v1"),"operations":[format!("{ADDRESS}#commit.v1")]}));
        s.invocations.push(json!({"id":format!("{sender}-call"),"target":sender,"caller":"root","state":"in-progress","conduit":"root-management","filters":[]}));
    }
    for (sender, onward, reserved) in [
        ("agent", false, true),
        ("agent", true, true),
        ("root", false, false),
        ("root", true, true),
    ] {
        let request: access::AccessRequest = serde_json::from_value(json!({"identity":{"kind":"instance","id":sender},"tool":format!("{sender}-repo.commit.v1"),"arguments":{},"serving":onward.then(||format!("{sender}-call"))})).unwrap();
        let mut t = crate::explain::tests::transition(s.clone());
        access::decide(&mut t, &crate::explain::tests::evidence(), &request, &c).unwrap();
        if reserved {
            assert_eq!(
                t.reply["diagnostics"][0]["ruleId"], "ACC-032",
                "{}",
                t.reply
            );
        } else {
            assert_eq!(t.reply["permitted"], true, "{}", t.reply);
        }
    }
}
