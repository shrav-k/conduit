use super::*;

const CORE: &str = "github.com/shrav-k/conduit/core";
const META: &str = "io.github.shrav-k.conduit/";

fn yaml(bytes: &[u8]) -> Vec<Value> {
    conduit_loader::read_bytes(conduit_loader::Syntax::Yaml, bytes)
        .unwrap()
        .iter()
        .map(|document| document.to_json())
        .collect()
}

pub(super) fn context() -> Context {
    macro_rules! documents {
        ($($file:literal),+ $(,)?) => {
            [$(include_bytes!(concat!("../../../../spec/", $file)).as_slice()),+]
                .into_iter().flat_map(yaml).collect::<Vec<_>>()
        };
    }
    let documents = documents![
        "core/capabilities.yaml",
        "core/operations.yaml",
        "core/references.yaml",
        "core/requirements.yaml",
        "core/results.yaml",
        "core/templates.yaml",
        "core/types.yaml",
        "schemas/contracts.yaml",
        "schemas/document.yaml",
        "schemas/envelope.yaml",
        "schemas/instances.yaml",
        "schemas/manifest.yaml",
        "schemas/schema.yaml",
        "schemas/sectioned.yaml",
        "schemas/templates.yaml",
        "schemas/values.yaml"
    ];
    let named = |doc: &Value| {
        (
            doc["metadata"]["name"].as_str().unwrap().into(),
            doc.clone(),
        )
    };
    let scope = schemas::Scope {
        address: CORE.into(),
        version: None,
        manifest: BTreeMap::new(),
        contracts: documents
            .iter()
            .filter(|doc| packages::contract(doc))
            .map(named)
            .collect(),
        exports: None,
    };
    Context {
        builtin: Some(packages::Project {
            manifest: yaml(include_bytes!("../../../../spec/core/conduit.yaml")).pop(),
            documents: documents.clone(),
            directory: None,
        }),
        core_package: CORE.into(),
        meta_prefix: META.into(),
        templates: documents
            .iter()
            .filter(|doc| !packages::contract(doc))
            .map(named)
            .collect(),
        record_source: "urn:conduit:first-build".into(),
        core_environment: schemas::Environment {
            scopes: BTreeMap::from([("core".into(), scope)]),
        },
    }
}

fn from_json(source: &str) -> Value {
    serde_json::from_str(source).unwrap()
}

fn evidence(number: u32) -> Evidence {
    Evidence {
        time: "2026-11-21T13:00:00Z".into(),
        ids: (1..65)
            .map(|n| format!("00000000-0000-4000-8000-{:012}", 1000 * number + n))
            .collect(),
        revisions: (0..32).map(|n| format!("rev-{number}-{n}")).collect(),
        suffixes: (0..16).map(|n| format!("{number:03}{n:02}")).collect(),
    }
}

pub(super) fn project(manifest: &str, documents: &str) -> packages::Project {
    packages::Project {
        manifest: Some(from_json(manifest)),
        documents: serde_json::from_str(documents).unwrap(),
        directory: None,
    }
}

macro_rules! fixture {
    ($case:literal, $prefix:literal) => {
        project(
            include_str!(concat!(
                "../../../../conformance/",
                $case,
                "/",
                $prefix,
                "conduit.json"
            )),
            include_str!(concat!(
                "../../../../conformance/",
                $case,
                "/",
                $prefix,
                "documents.json"
            )),
        )
    };
}

fn registration_request(c: &Context) -> Request {
    Request {
        sender: "00000000-0000-4000-8000-000000000005".into(),
        caller: "00000000-0000-4000-8000-000000000005".into(),
        conduit: "00000000-0000-4000-8000-000000000006".into(),
        target: "00000000-0000-4000-8000-000000000003".into(),
        operation: c.qualified("register.v1"),
    }
}

fn start(c: &Context) -> State {
    let case = from_json(include_str!(
        "../../../../conformance/conventions/requests/login-resource-lapse/case.yaml"
    ));
    let input = serde_json::from_value(case["steps"][0]["input"].clone()).unwrap();
    step(&State::default(), input, c).unwrap().state
}

fn register(state: &State, project: packages::Project, serial: u32, c: &Context) -> Transition {
    step(
        state,
        Input::RegisterProject {
            snapshots: vec![],
            builtin: c.builtin.clone().unwrap(),
            project,
            evidence: evidence(serial),
            request: registration_request(c),
        },
        c,
    )
    .unwrap()
}

fn assert_fields(actual: &Value, expected: &Value, step: usize) {
    match expected {
        Value::Object(fields) if !fields.is_empty() => {
            for (name, value) in fields {
                assert_fields(&actual[name], value, step);
            }
        }
        Value::Array(values) => {
            let actual = actual.as_array().expect("expected an array in reply");
            assert_eq!(
                actual.len(),
                values.len(),
                "reply array length at step {step}"
            );
            for (actual, value) in actual.iter().zip(values) {
                assert_fields(actual, value, step);
            }
        }
        _ => assert_eq!(actual, expected, "reply field at step {step}"),
    }
}

fn substitute(value: &mut Value, captures: &BTreeMap<String, Value>) {
    match value {
        Value::String(name) if name.starts_with('$') => *value = captures[&name[1..]].clone(),
        Value::Object(fields) => fields.values_mut().for_each(|v| substitute(v, captures)),
        Value::Array(items) => items.iter_mut().for_each(|v| substitute(v, captures)),
        _ => {}
    }
}

pub(super) fn run_case(
    source: &str,
    project: packages::Project,
    count: usize,
) -> (State, Context, BTreeMap<String, Value>) {
    let c = context();
    let case = from_json(source);
    let mut state = State::default();
    let mut captures = BTreeMap::new();
    for (index, item) in case["steps"]
        .as_array()
        .unwrap()
        .iter()
        .take(count)
        .enumerate()
    {
        let mut input = item["input"].clone();
        substitute(&mut input, &captures);
        let input = if input["kind"] == "register-project" {
            Input::RegisterProject {
                snapshots: vec![],
                builtin: c.builtin.clone().unwrap(),
                project: project.clone(),
                evidence: serde_json::from_value(input["evidence"].clone()).unwrap(),
                request: registration_request(&c),
            }
        } else {
            serde_json::from_value(input).unwrap()
        };
        let t = step(&state, input, &c).unwrap();
        let mut expected = item["expect"]["reply"].clone();
        substitute(&mut expected, &captures);
        assert_fields(&t.reply, &expected, index + 1);
        if item["expect"]["external_intents"] == json!([]) {
            assert!(
                t.external_intents.is_empty(),
                "dispatch at step {}",
                index + 1
            );
        }
        for (name, pointer) in item["capture"].as_object().into_iter().flatten() {
            let reply_pointer = pointer.as_str().unwrap().strip_prefix("/reply").unwrap();
            captures.insert(
                name.clone(),
                t.reply
                    .pointer(reply_pointer)
                    .expect("unavailable reply capture")
                    .clone(),
            );
        }
        state = t.state;
    }
    (state, c, captures)
}

pub(super) fn subject<'a>(state: &'a State, kind: &str, name: &str) -> &'a Value {
    access::object(state, &json!({"kind":kind,"name":name})).unwrap()
}

pub(super) fn invoke(
    state: &State,
    c: &Context,
    serial: u32,
    actor: &Value,
    tool: &str,
    arguments: Value,
) -> Transition {
    step(
        state,
        Input::Invoke {
            evidence: evidence(serial),
            key: None,
            archive: None,
            request: serde_json::from_value(
                json!({"identity":{"kind":"instance","id":actor},"tool":tool,
            "arguments":arguments,"clientCapabilities":{"elicitation":{"url":{}}}}),
            )
            .unwrap(),
        },
        c,
    )
    .unwrap()
}

pub(super) fn complete(
    state: &State,
    c: &Context,
    serial: u32,
    invocation: &Value,
    result: Value,
) -> Transition {
    step(
        state,
        Input::Complete {
            evidence: evidence(serial),
            invocation: invocation.as_str().unwrap().into(),
            attempt: 1,
            completion: requests::Completion::Result { result },
        },
        c,
    )
    .unwrap()
}

fn first_diagnostic(reply: &Value) -> &Value {
    &reply["structuredContent"]["diagnostics"][0]
}

#[test]
fn login_resource_type_rules() {
    let c = context();
    let cases = [
        (
            fixture!("language/contracts/login-resource-misplaced", "optional-"),
            false,
        ),
        (
            fixture!("language/contracts/login-resource-misplaced", "string-"),
            false,
        ),
        (
            fixture!("language/contracts/login-resource-misplaced", "control-"),
            false,
        ),
        (
            fixture!("language/contracts/login-resource-misplaced", "valid-"),
            true,
        ),
    ];
    for (project, accepted) in cases {
        let t = step(
            &State::default(),
            Input::ResolveProject {
                snapshots: vec![],
                builtin: c.builtin.clone().unwrap(),
                project,
            },
            &c,
        )
        .unwrap();
        assert_eq!(t.reply["accepted"], accepted, "login-place Type validation");
        if !accepted {
            assert_eq!(t.reply["diagnostics"][0]["ruleId"], "CTR-013");
        }
    }
    let base = register(
        &start(&c),
        fixture!("language/registration/login-place", ""),
        40,
        &c,
    );
    assert_eq!(
        base.reply["accepted"], true,
        "login-place environment binding"
    );
    for (index, project) in [
        fixture!("language/registration/login-place", "no-sign-in-"),
        fixture!("language/registration/login-place", "null-"),
    ]
    .into_iter()
    .enumerate()
    {
        let t = register(&base.state, project, 41 + index as u32, &c);
        assert_eq!(t.reply["diagnostics"][0]["ruleId"], "CTR-013");
    }
}

#[test]
fn login_resource_pool_authority_refuses_registration_without_the_grant() {
    let c = context();
    let project = fixture!("conventions/requests/login-resource-lapse", "");
    let base = register(&start(&c), project.clone(), 50, &c);
    assert_eq!(
        base.reply["accepted"], true,
        "register the login and its Pool"
    );
    let mut state = base.state.clone();
    let pool =
        access::object(&state, &json!({"kind":"Pool","name":"logins"})).unwrap()["id"].clone();
    let actor = registration_request(&c).sender;
    let authority = state
        .objects
        .iter_mut()
        .find(|object| {
            object["kind"] == "Conduit" && object["target"] == pool && object["consumer"] == actor
        })
        .unwrap();
    authority["operations"]
        .as_array_mut()
        .unwrap()
        .retain(|operation| operation != &c.qualified("register.v1"));

    let mut candidate = project;
    candidate.documents.retain(packages::contract);
    candidate.documents.push(json!({
        "apiVersion":"github.com/shrav-k/conduit/v0","kind":"Agent",
        "metadata":{"name":"candidate"},"from":"worker","owner":"workers"
    }));
    let refused = register(&state, candidate.clone(), 51, &c);
    assert_eq!(
        refused.reply["reason"], "refused",
        "the login Pool must authorize registration"
    );
    assert_eq!(refused.reply["diagnostics"][0]["ruleId"], "ACC-033");
    assert!(
        refused.reply["diagnostics"][0]["message"]["text"]
            .as_str()
            .unwrap()
            .contains("logins")
    );
    assert_eq!(
        refused.state.objects, state.objects,
        "the refusal registers nothing"
    );
    assert_eq!(
        register(&base.state, candidate, 51, &c).reply["accepted"],
        true
    );
}

#[test]
fn login_resource_run_binding() {
    let c = context();
    let base = register(
        &start(&c),
        fixture!("conventions/requests/login-resource-lapse", ""),
        60,
        &c,
    );
    assert_eq!(
        base.reply["accepted"], true,
        "register the shared login project"
    );
    let resource = access::object(&base.state, &json!({"kind":"Resource","name":"login"})).unwrap();
    let agent = access::object(&base.state, &json!({"kind":"Agent","name":"a"})).unwrap();
    let invocation = json!({"object":agent["id"],"setup":{"item":{"item":{"environment":{"LOGIN_DIRECTORY":"/login"}}}}});
    let (inputs, secrets) = lifecycle::run_inputs(agent, &invocation, &c).unwrap();
    assert_eq!(
        inputs["loginDirectories"]["LOGIN_DIRECTORY"], resource["id"],
        "login is a directory binding"
    );
    assert!(secrets["environment"].get("LOGIN_DIRECTORY").is_none());
    let actor = &subject(&base.state, "Interface", "root")["id"];
    for (mode, condition) in [
        ("agent-runtime", "the Agent names a runtime"),
        ("resource-runtime", "the Resource names a runtime"),
        (
            "different-node",
            "the Agent and the Resource have different placements",
        ),
    ] {
        let (id, field, value) = match mode {
            "agent-runtime" => (
                &agent["id"],
                "runtime",
                json!({"kind":"Resource","id":resource["id"]}),
            ),
            "resource-runtime" => (
                &resource["id"],
                "runtime",
                json!({"kind":"Resource","id":resource["id"]}),
            ),
            _ => (
                &agent["id"],
                "placement",
                json!({"kind":"Resource","id":"other-node"}),
            ),
        };
        let mut state = base.state.clone();
        state.objects.iter_mut().find(|o| &o["id"] == id).unwrap()[field] = value;
        let setup = invoke(
            &state,
            &c,
            160,
            actor,
            "workers.set-up.v1",
            json!({"instance":{"kind":"Agent","name":"a"}}),
        );
        assert!(
            setup.external_intents.is_empty(),
            "{mode} must fail before reaching a runner"
        );
        assert_eq!(setup.reply["_meta"][format!("{META}outcome")], "failed");
        assert!(
            setup.reply["structuredContent"]["diagnostics"]
                .to_string()
                .contains("PRV-029")
        );
        assert!(
            setup.reply["structuredContent"]["diagnostics"]
                .to_string()
                .contains("Resource/login")
        );
        assert!(
            setup.reply["structuredContent"]["diagnostics"]
                .to_string()
                .contains(condition)
        );
    }
}

#[test]
fn login_resource_lapse_and_hold() {
    let (mut state, c, _) = run_case(
        include_str!("../../../../conformance/conventions/requests/login-resource-lapse/case.yaml"),
        fixture!("conventions/requests/login-resource-lapse", ""),
        usize::MAX,
    );
    let root = subject(&state, "Interface", "root")["id"].clone();
    let resource = subject(&state, "Resource", "login")["id"].clone();
    let agents: Vec<_> = ["a", "b"]
        .iter()
        .map(|name| subject(&state, "Agent", name)["id"].clone())
        .collect();
    assert!(!state.invocations.iter().any(|v| {
        agents.contains(&v["target"])
            && ["account-hook.v1", "sign-in-hook.v1", "sign-out-hook.v1"]
                .iter()
                .any(|op| v["operation"] == c.qualified(op))
    }));
    state
        .objects
        .iter_mut()
        .find(|o| o["id"] == resource)
        .unwrap()["status"]["conditions"] = json!([{"type":"SignInUndone","status":"False","reason":"SignOutFailed","message":"The scripted sign-out failed.","lastTransitionTime":"2026-11-21T13:00:00Z","source":"urn:conduit:first-build"}]);
    for tool in ["call-a.turn.v1", "call-b.turn.v1"] {
        let held = invoke(&state, &c, 161, &root, tool, json!({}));
        assert_eq!(first_diagnostic(&held.reply)["ruleId"], "PND-011");
        assert!(
            first_diagnostic(&held.reply)["message"]["text"]
                .as_str()
                .unwrap()
                .contains("Resource/login")
        );
        assert!(held.external_intents.is_empty());
    }
    let sign_in = invoke(
        &state,
        &c,
        162,
        &root,
        "logins.sign-in.v1",
        json!({"instance":{"kind":"Resource","name":"login"}}),
    );
    let hook = sign_in.reply["hook"].clone();
    let discovered = complete(
        &sign_in.state,
        &c,
        163,
        &hook,
        json!({"capabilities":{"extensions":{format!("{META}implementation-interface"):{"revision":"v0"}}}}),
    );
    let checked = complete(
        &discovered.state,
        &c,
        164,
        &hook,
        json!({"isError":false,"_meta":{format!("{META}outcome"):"succeeded"},"structuredContent":{"account":"example-account"}}),
    );
    assert_eq!(
        checked.reply["_meta"][format!("{META}outcome")],
        "succeeded"
    );
    assert!(!sign_in::request_is_held_for_sign_in_undone(
        &checked.state,
        &resource
    ));
    for tool in ["call-a.turn.v1", "call-b.turn.v1"] {
        assert_eq!(
            invoke(&checked.state, &c, 165, &root, tool, json!({})).reply["pending"],
            true
        );
    }
}

#[test]
fn login_resource_plain_url_lapse() {
    let (state, _, captures) = run_case(
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/case.yaml"
        ),
        fixture!("conventions/requests/login-resource-one-at-a-time", ""),
        12,
    );
    let ending = state
        .records
        .iter()
        .find(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == captures["callA"])
        .unwrap();
    assert_eq!(
        ending["data"]["pendingAction"],
        json!([captures["action"], captures["notice"]])
    );
    assert!(state.sign_in_holds.is_empty());
}

#[test]
fn login_resource_one_call_at_a_time() {
    let source = include_str!(
        "../../../../conformance/conventions/requests/login-resource-one-at-a-time/case.yaml"
    );
    let project = fixture!("conventions/requests/login-resource-one-at-a-time", "");
    run_case(source, project.clone(), usize::MAX);
    let (mut active, c, captures) = run_case(source, project.clone(), 13);
    let login = subject(&active, "Resource", "login")["id"]
        .as_str()
        .unwrap()
        .to_owned();
    active
        .sign_in_holds
        .insert(login, json!({"since":"2026-11-21T13:00:00Z"}));
    let root = &subject(&active, "Interface", "root")["id"];
    let held = invoke(&active, &c, 166, root, "call-a.turn.v1", json!({}));
    assert_eq!(
        held.reply["_meta"][format!("{META}pendingAction")],
        json!([captures["action"], captures["notice"]])
    );
    let text = first_diagnostic(&held.reply)["message"]["text"]
        .as_str()
        .unwrap();
    assert!(text.find("awaits sign-in").unwrap() < text.find("one at a time").unwrap());
    assert!(text.contains(&format!(
        "invocation {} of Agent/b",
        captures["runningB"].as_str().unwrap()
    )));

    let observer = subject(&active, "Interface", "observer")["id"].clone();
    let mut route = subject(&active, "Conduit", "call-a").clone();
    route["consumer"] = observer.clone();
    route["id"] = json!("00000000-0000-4000-8000-000000166999");
    active.objects.push(route);
    let held = invoke(&active, &c, 167, &observer, "call-a.turn.v1", json!({}));
    let text = first_diagnostic(&held.reply)["message"]["text"]
        .as_str()
        .unwrap();
    assert!(text.contains("a call of an Agent of it"));
    assert!(!text.contains(captures["runningB"].as_str().unwrap()));

    let (ready, c, _) = run_case(source, project, 10);
    let root = &subject(&ready, "Interface", "root")["id"];
    let request = invoke(&ready, &c, 168, root, "call-a.status.v1", json!({}));
    let lapse = complete(
        &request.state,
        &c,
        169,
        &request.reply["invocation"],
        json!({"resultType":"input_required","inputRequests":{"sign-in":{"method":"elicitation/create","params":{"mode":"url","message":"Scripted lapse","url":"https://example.invalid/help"}}}}),
    );
    assert_eq!(
        lapse.reply["_meta"][format!("{META}pendingAction")]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(
        !lapse
            .state
            .pending_actions
            .values()
            .any(|entry| entry["action"]["kind"] == "notice")
    );
}

#[test]
fn login_resource_retirement_conflict() {
    let source = include_str!(
        "../../../../conformance/conventions/fleets/e3b/login-resource-retire/case.yaml"
    );
    let project = fixture!("conventions/fleets/e3b/login-resource-retire", "");
    run_case(source, project.clone(), usize::MAX);
    let (state, c, _) = run_case(source, project, 3);
    let root = &subject(&state, "Interface", "root")["id"];
    for (tool, args) in [
        (
            "logins.retire.v1",
            json!({"instance":{"kind":"Resource","name":"login"}}),
        ),
        ("retire-logins.retire-with-resources.v1", json!({})),
    ] {
        let refused = invoke(&state, &c, 170, root, tool, args);
        let diag = first_diagnostic(&refused.reply);
        assert_eq!(diag["ruleId"], "PRV-020");
        assert!(
            diag["message"]["text"]
                .as_str()
                .unwrap()
                .contains("Agent/a")
        );
    }
}

#[test]
fn login_resource_retirement_never_runs_with_an_ineligible_directory() {
    let c = context();
    let base = register(
        &start(&c),
        fixture!("conventions/requests/login-resource-lapse", ""),
        60,
        &c,
    );
    let mut state = base.state;
    let root = subject(&state, "Interface", "root")["id"].clone();
    let resource = subject(&state, "Resource", "login")["id"].clone();
    let agent = subject(&state, "Agent", "a")["id"].clone();
    state
        .objects
        .iter_mut()
        .find(|o| o["id"] == resource)
        .unwrap()["runtime"] = json!({"kind":"Resource","id":resource});
    let turn = c.qualified("retire-hook.v1");
    state.objects.iter_mut().find(|o| o["id"] == agent).unwrap()["resolved_implementations"]
        [turn] =
        json!({"item":{"command":["fixture-retire"],"environment":{"LOGIN_DIRECTORY":"/login"}}});

    let subject = json!({"instance":{"kind":"Agent","name":"a"}});
    let setup = invoke(&state, &c, 180, &root, "workers.set-up.v1", subject.clone());
    assert_eq!(setup.reply["_meta"][format!("{META}outcome")], "failed");
    assert!(
        setup.reply["structuredContent"]["diagnostics"]
            .to_string()
            .contains("PRV-029")
    );
    let retire = invoke(&setup.state, &c, 181, &root, "workers.retire.v1", subject);
    let pending = if retire
        .external_intents
        .iter()
        .any(|intent| intent["operation"] == c.qualified("inspect-run.v1"))
    {
        step(
            &retire.state,
            Input::SetupReport {
                evidence: evidence(182),
                invocation: retire.reply["invocation"].as_str().unwrap().into(),
                report: lifecycle::Report::InspectRun {
                    state: "stopped".into(),
                },
            },
            &c,
        )
        .unwrap()
    } else {
        retire
    };
    assert!(
        !pending
            .external_intents
            .iter()
            .any(|intent| intent["operation"] == c.qualified("run.v1")
                || intent["inputs"].get("loginDirectories").is_some()),
        "retirement must not run a hook with the ineligible login directory"
    );
    assert!(
        pending
            .external_intents
            .iter()
            .any(|intent| intent["operation"] == c.qualified("stop.v1"))
    );
}

#[test]
fn login_resource_lapse_evidence_obeys_agent_inspection_scope() {
    let (mut state, c, captures) = run_case(
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/case.yaml"
        ),
        fixture!("conventions/requests/login-resource-one-at-a-time", ""),
        12,
    );
    let root = subject(&state, "Interface", "root")["id"].clone();
    let observer = subject(&state, "Interface", "observer")["id"].clone();
    let pool = subject(&state, "Pool", "logins")["id"].clone();
    let mut route = state
        .objects
        .iter()
        .find(|o| o["kind"] == "Conduit" && o["consumer"] == root && o["target"] == pool)
        .unwrap()
        .clone();
    route["id"] = json!("00000000-0000-4000-8000-000000180999");
    route["consumer"] = observer.clone();
    route["operations"] = json!([c.qualified("inspect.v1")]);
    state.objects.push(route);
    let login = subject(&state, "Resource", "login");
    let agent = subject(&state, "Agent", "a");
    assert!(access::may_inspect(&state, &observer, login, &c).unwrap());
    assert!(!access::may_inspect(&state, &observer, agent, &c).unwrap());
    let action = &state.pending_actions[captures["action"].as_str().unwrap()]["action"];
    let holder = access::object_by_id(&state, &observer).unwrap();
    let view = pending::raised_view(action, holder, &state, &evidence(180).time, &c)
        .unwrap()
        .unwrap();
    assert_eq!(view["subject"], action["subject"]);
    assert_eq!(
        view["evidence"],
        json!([action["evidence"][0]]),
        "Pool-only inspection must hide the Agent's lapse evidence"
    );
    let holder = access::object_by_id(&state, &root).unwrap();
    let root_view = pending::raised_view(action, holder, &state, &evidence(180).time, &c)
        .unwrap()
        .unwrap();
    assert_eq!(root_view["evidence"], action["evidence"]);
}

#[test]
fn login_resource_notice_cites_the_triggering_request_record() {
    let source = include_str!(
        "../../../../conformance/conventions/requests/login-resource-one-at-a-time/case.yaml"
    );
    let (mut state, c, captures) = run_case(
        source,
        fixture!("conventions/requests/login-resource-one-at-a-time", ""),
        11,
    );
    let request = state
        .records
        .iter()
        .find(|r| r["type"] == "access-decision" && r["data"]["invocation"] == captures["callA"])
        .unwrap()["id"]
        .clone();
    let mut finish = from_json(source)["steps"][11]["input"].clone();
    substitute(&mut finish, &captures);
    for purge in [false, true] {
        if purge {
            state.records.retain(|record| record["id"] != request);
        }
        let ended = step(&state, serde_json::from_value(finish.clone()).unwrap(), &c).unwrap();
        let notice = ended.reply["_meta"][format!("{META}pendingAction")][1]
            .as_str()
            .unwrap();
        let evidence = &ended.state.pending_actions[notice]["action"]["evidence"];
        assert_eq!(
            evidence[1]["value"], request,
            "the notice must cite the request's record ID"
        );
    }
}
