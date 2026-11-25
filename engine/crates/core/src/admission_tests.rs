use super::*;
use crate::login_resource_tests::{context, invoke, project, run_case, subject};

const CHECKS: &str =
    include_str!("../../../../conformance/language/registration/admission-checks/case.yaml");
const REPORT: &str =
    include_str!("../../../../conformance/conventions/requests/admission-report-kept/case.yaml");

fn guarded() -> packages::Project {
    project(
        include_str!("../../../../conformance/language/contracts/admission-shape/conduit.json"),
        include_str!(
            "../../../../conformance/language/contracts/admission-shape/valid-documents.json"
        ),
    )
}

fn variant(prefix: &str) -> packages::Project {
    macro_rules! fixtures {
        ($($prefix:literal),+ $(,)?) => {
            match prefix {
                $($prefix => project(
                    include_str!(concat!("../../../../conformance/language/registration/admission-checks/", $prefix, "-manifest.json")),
                    include_str!(concat!("../../../../conformance/language/registration/admission-checks/", $prefix, "-documents.json")),
                ),)+
                _ => panic!("unknown admission fixture"),
            }
        };
    }
    fixtures!(
        "default",
        "optional-unknown",
        "unconfined-paid",
        "renamed-control",
        "overlapping-control",
        "unlisted-control",
        "agent-login-type",
        "missing-agent-admission",
        "invalid-pair",
        "valid",
    )
}

fn evidence(serial: u32) -> Evidence {
    Evidence {
        time: "2026-11-21T13:00:00Z".into(),
        ids: (1..100)
            .map(|n| format!("00000000-0000-4000-a317-{serial:06}{n:06}"))
            .collect(),
        revisions: (0..50).map(|n| format!("admission-{serial}-{n}")).collect(),
        suffixes: (0..20).map(|n| format!("{serial:03}{n:02}")).collect(),
    }
}

fn started(c: &Context) -> State {
    let case: Value = serde_json::from_str(CHECKS).unwrap();
    step(
        &State::default(),
        serde_json::from_value(case["steps"][7]["input"].clone()).unwrap(),
        c,
    )
    .unwrap()
    .state
}

fn register(state: &State, p: packages::Project, serial: u32, c: &Context) -> Transition {
    let case: Value = serde_json::from_str(CHECKS).unwrap();
    let mut request = case["steps"][10]["input"]["request"].clone();
    request["operation"] = json!(c.qualified("register.v1"));
    step(
        state,
        Input::RegisterProject {
            project: p,
            snapshots: vec![],
            builtin: c.builtin.clone().unwrap(),
            evidence: evidence(serial),
            request: serde_json::from_value(request).unwrap(),
        },
        c,
    )
    .unwrap()
}

fn rejected(reply: &Value, words: &[&str]) {
    let content = reply.get("structuredContent").unwrap_or(reply);
    assert_eq!(reply.get("isError").unwrap_or(&json!(true)), true);
    let reason = reply
        .get("reason")
        .unwrap_or(&reply["_meta"]["io.github.shrav-k.conduit/reason"]);
    assert_eq!(reason, "invalid");
    let ds = content["diagnostics"].as_array().unwrap();
    assert_eq!(ds.len(), 1, "one authored place: {ds:?}");
    assert_eq!(ds[0]["ruleId"], "CTR-015");
    assert_eq!(ds[0]["level"], "error");
    let text = ds[0]["message"]["text"].as_str().unwrap();
    for word in words {
        assert!(text.contains(word), "missing {word}: {text}");
    }
}

fn archive(mut p: packages::Project) -> archives::ArchiveInput {
    let manifest = p.manifest.as_mut().unwrap();
    manifest["version"] = json!("0.1.0");
    let address = manifest["address"].as_str().unwrap().to_owned();
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut tar = tar::Builder::new(encoder);
    let contracts: Vec<_> = p.documents.into_iter().filter(packages::contract).collect();
    for (name, value) in [
        ("conduit.json", manifest.clone()),
        ("contracts.json", json!(contracts)),
    ] {
        let bytes = serde_json::to_vec(&value).unwrap();
        let mut header = tar::Header::new_ustar();
        header.set_size(bytes.len() as u64);
        header.set_mode(0o644);
        tar.append_data(&mut header, name, bytes.as_slice())
            .unwrap();
    }
    let bytes = tar.into_inner().unwrap().finish().unwrap();
    archives::ArchiveInput {
        name: "admission.tar.gz".into(),
        address,
        version: "0.1.0".into(),
        digest: packages::bytes_digest(&bytes),
        directory: None,
        completion: archives::FetchCompletion::Complete { bytes },
    }
}

#[test]
fn admission_types_checked_at_install_and_submission() {
    let c = context();
    for (prefix, cause) in [
        ("default", "default"),
        ("optional-unknown", "whenUnknown"),
        ("unconfined-paid", "paidUsage"),
        ("renamed-control", "levers"),
        ("overlapping-control", "overlap"),
        ("unlisted-control", "listed"),
        ("agent-login-type", "signIn and admission"),
    ] {
        let t = step(
            &State::default(),
            Input::ResolveProject {
                snapshots: vec![],
                builtin: c.builtin.clone().unwrap(),
                project: variant(prefix),
            },
            &c,
        )
        .unwrap();
        assert_eq!(t.reply["accepted"], false, "{prefix} accepted");
        rejected(
            &t.reply,
            &[
                if prefix == "agent-login-type" {
                    "shared-agent.v1"
                } else {
                    "login-holder.v1"
                },
                cause,
            ],
        );
    }
    let valid = step(
        &State::default(),
        Input::ResolveProject {
            snapshots: vec![],
            builtin: c.builtin.clone().unwrap(),
            project: variant("valid"),
        },
        &c,
    )
    .unwrap();
    assert_eq!(valid.reply["accepted"], true);
    let state = started(&c);
    for (serial, prefix) in [(1, "default"), (2, "valid")] {
        let archive = archive(variant(prefix));
        let t = step(&state, Input::Invoke {
            evidence: evidence(serial), key: None,
            request: serde_json::from_value(json!({
                "identity":{"kind":"instance","id":subject(&state,"Interface","root")["id"]},
                "tool":"management.install.v1",
                "arguments":{"address":archive.address,"version":archive.version,"digest":archive.digest,
                             "fetch":"file:///fixture/admission.tar.gz"}
            })).unwrap(),
            archive: Some(archive),
        }, &c).unwrap();
        if prefix == "default" {
            rejected(&t.reply, &["login-holder.v1", "default"]);
            assert!(t.state.installed_packages == state.installed_packages);
        } else {
            assert_eq!(
                t.reply["isError"], false,
                "valid archive: {}",
                t.reply["structuredContent"]["diagnostics"]
            );
            assert_eq!(
                t.state.installed_packages.len(),
                state.installed_packages.len() + 1
            );
        }
    }
}

fn root_call(state: &State, c: &Context, serial: u32, tool: &str, args: Value) -> Transition {
    invoke(
        state,
        c,
        serial,
        &subject(state, "Interface", "root")["id"],
        tool,
        args,
    )
}

fn document_mut<'a>(p: &'a mut packages::Project, name: &str) -> &'a mut Value {
    p.documents
        .iter_mut()
        .find(|d| d["metadata"]["name"] == name)
        .unwrap()
}

#[test]
fn admission_pair_faults_follow_individual_instance_arguments() {
    let c = context();
    let state = started(&c);
    for (lever, initial, supplied) in [
        ("whenUnknown", "refuse", "admit"),
        ("paidUsage", "allow", "refuse"),
    ] {
        let mut p = guarded();
        let template = document_mut(&mut p, "login-holder");
        template["spec"]["config"]["admission"]["whenUnknown"] = json!("admit");
        template["spec"]["config"]["admission"][lever] = json!(initial);
        template["spec"]["inputs"] = json!({
            "choice":{"schema":{"type":"string"},"to":[format!("/config/admission/{lever}")]}
        });
        for name in ["login", "gone"] {
            document_mut(&mut p, name)["arguments"] = json!({"choice":supplied});
        }
        let t = register(&state, p, 70, &c);
        let ds = t.reply["diagnostics"].as_array().unwrap();
        assert_eq!(
            ds.len(),
            2,
            "separate instance arguments must stay separate: {ds:?}"
        );
        for name in ["login", "gone"] {
            let d = ds
                .iter()
                .find(|d| {
                    d["message"]["text"]
                        .as_str()
                        .unwrap()
                        .starts_with(&format!("{name}:"))
                })
                .expect("diagnostic names the instance that supplied the lever");
            assert_eq!(d["ruleId"], "CTR-015");
            assert_eq!(
                d["locations"][0]["logicalLocations"][0]["fullyQualifiedName"],
                format!("/objects/{name}/arguments/choice")
            );
        }
    }
    // A single template-authored pair still produces one diagnostic for both instances.
    let t = register(&state, variant("invalid-pair"), 71, &c);
    rejected(
        &t.reply,
        &[
            "login-holder:",
            "/admission/whenUnknown",
            "/admission/paidUsage",
        ],
    );
}

#[test]
fn admission_pair_preserves_independent_conditional_schema_faults() {
    let c = context();
    let state = started(&c);
    let mut p = guarded();
    let schema = &mut document_mut(&mut p, "login-holder.v1")["spec"]["schema"];
    schema["properties"]["locked"] = json!({"type":"boolean"});
    schema["allOf"] = json!([{
        "if":{"properties":{"locked":{"const":true}},"required":["locked"]},
        "then":{"properties":{"admission":{"properties":{"whenUnknown":{"const":"refuse"}}}}}
    }]);
    let config = &mut document_mut(&mut p, "login-holder")["spec"]["config"];
    config["locked"] = json!(true);
    config["admission"]["whenUnknown"] = json!("admit");
    for paid in ["refuse", "allow"] {
        document_mut(&mut p, "login-holder")["spec"]["config"]["admission"]["paidUsage"] =
            json!(paid);
        let t = register(&state, p.clone(), 72, &c);
        let ds = t.reply["diagnostics"].as_array().unwrap();
        assert!(
            ds.iter().any(|d| d["ruleId"] == "INS-014"
                && d["message"]["text"]
                    .as_str()
                    .unwrap()
                    .contains("/allOf/0/then/properties/admission/properties/whenUnknown/const")),
            "the independent locked restriction must remain INS-014: {ds:?}"
        );
        assert_eq!(
            ds.iter().filter(|d| d["ruleId"] == "CTR-015").count(),
            usize::from(paid == "refuse")
        );
    }
}

#[test]
fn admission_levers_diagnostics_explain_the_schema_fix() {
    let c = context();
    for (prefix, cause) in [
        (
            "unconfined-paid",
            "/admission/paidUsage must be confined by enum to some of: allow, refuse, refuse-if-possible",
        ),
        (
            "optional-unknown",
            "the levers schema must require /admission/whenUnknown",
        ),
        ("default", "a default stands at"),
    ] {
        let t = step(
            &State::default(),
            Input::ResolveProject {
                snapshots: vec![],
                builtin: c.builtin.clone().unwrap(),
                project: variant(prefix),
            },
            &c,
        )
        .unwrap();
        rejected(&t.reply, &[cause]);
        let text = t.reply["diagnostics"][0]["message"]["text"]
            .as_str()
            .unwrap();
        assert!(!text.contains("rule (a)"));
        if prefix == "default" {
            assert!(
                text.contains("/properties/admission/properties/whenUnknown/default"),
                "default's schema pointer: {text}"
            );
            assert!(text.contains("levers place /admission"));
        }
    }
    // Following a local reference must name the default's actual schema location.
    let mut p = guarded();
    let schema = &mut document_mut(&mut p, "login-holder.v1")["spec"]["schema"];
    let mut levers = schema["properties"]["admission"].take();
    levers["properties"]["whenUnknown"]["default"] = json!("refuse");
    schema["$defs"] = json!({"levers":levers});
    schema["properties"]["admission"] = json!({"$ref":"#/$defs/levers"});
    let t = step(
        &State::default(),
        Input::ResolveProject {
            snapshots: vec![],
            builtin: c.builtin.clone().unwrap(),
            project: p,
        },
        &c,
    )
    .unwrap();
    rejected(
        &t.reply,
        &[
            "a default stands at /$defs/levers/properties/whenUnknown/default",
            "levers place /admission",
        ],
    );
}

#[test]
fn admission_checked_at_registration_and_replacement() {
    let c = context();
    let state = started(&c);
    for (serial, prefix, words) in [
        (
            10,
            "missing-agent-admission",
            ["shared-agent.v1", "login-holder.v1"],
        ),
        (
            11,
            "invalid-pair",
            ["/admission/whenUnknown", "/admission/paidUsage"],
        ),
    ] {
        let t = register(&state, variant(prefix), serial, &c);
        rejected(&t.reply, &words);
        assert!(t.state.objects == state.objects);
    }
    assert_eq!(
        register(&state, variant("valid"), 12, &c).reply["accepted"],
        true
    );
    let mut p = guarded();
    p.documents
        .retain(|d| d["metadata"]["name"] != "login" && d["metadata"]["name"] != "gone");
    let pool = p
        .documents
        .iter_mut()
        .find(|d| d["metadata"]["name"] == "login-pool")
        .unwrap();
    pool["spec"]["entries"] = json!({"seat":{"from":"login-holder"}});
    let worker = p
        .documents
        .iter_mut()
        .find(|d| d["metadata"]["name"] == "worker")
        .unwrap();
    worker["spec"]["config"]["login"] = json!({"kind":"Pool","name":"logins","entry":"seat"});
    let t = register(&state, p.clone(), 13, &c);
    assert_eq!(
        t.reply["accepted"], true,
        "nested login: {}",
        t.reply["diagnostics"]
    );
    let mut state = t.state;
    let pool_id = subject(&state, "Pool", "logins")["id"].clone();
    let seat = state
        .objects
        .iter_mut()
        .find(|o| o["nested"]["parent"] == pool_id && o["nested"]["name"] == "seat")
        .unwrap();
    seat["state"] = json!("retired");
    let retired_id = seat["id"].clone();
    let mut ty = p
        .documents
        .iter()
        .find(|d| d["metadata"]["name"] == "login-holder.v1")
        .unwrap()
        .clone();
    ty["metadata"]["name"] = json!("other-login.v1");
    p.documents.push(ty);
    for (serial, type_name, unknown, words) in [
        (
            20,
            "#other-login.v1",
            "refuse",
            Some(["shared-agent.v1", "other-login.v1"]),
        ),
        (
            22,
            "#login-holder.v1",
            "admit",
            Some(["/admission/whenUnknown", "/admission/paidUsage"]),
        ),
        (24, "#login-holder.v1", "refuse", None),
    ] {
        let mut template = p
            .documents
            .iter()
            .find(|d| d["metadata"]["name"] == "login-holder")
            .unwrap()
            .clone();
        template["spec"]["type"] = json!(type_name);
        template["spec"]["config"]["admission"]["whenUnknown"] = json!(unknown);
        let updated = root_call(
            &state,
            &c,
            serial,
            "management.replace.v1",
            json!({
                "template":template,"contracts":p.documents.iter().filter(|d| packages::contract(d)).collect::<Vec<_>>(),
                "manifest":p.manifest,"revision":subject(&state,"Resource","login-holder")["revision"]
            }),
        );
        assert_eq!(
            updated.reply["isError"], false,
            "replace template: {}",
            updated.reply["structuredContent"]["diagnostics"]
        );
        state = updated.state;
        let t = root_call(
            &state,
            &c,
            serial + 1,
            "management.register-replacement.v1",
            json!({"instance":{"kind":"Pool","name":"logins"},"nested":"seat"}),
        );
        if let Some(words) = words {
            rejected(&t.reply, &words);
            assert!(state.objects == t.state.objects);
            assert_eq!(state.invocations, t.state.invocations);
            assert!(t.external_intents.is_empty());
            assert!(
                t.state
                    .objects
                    .iter()
                    .any(|o| o["id"] == retired_id && o["state"] == "retired")
            );
        } else {
            assert_eq!(
                t.reply["isError"], false,
                "valid replacement: {}",
                t.reply["structuredContent"]["diagnostics"]
            );
            assert_ne!(t.reply["structuredContent"]["object"]["id"], retired_id);
        }
        state = t.state;
    }
}

fn control_project() -> packages::Project {
    let mut p = guarded();
    let ty = p
        .documents
        .iter_mut()
        .find(|d| d["metadata"]["name"] == "login-holder.v1")
        .unwrap();
    let schema = ty["spec"]["schema"]["properties"]["admission"]
        .as_object_mut()
        .unwrap();
    schema.remove("if");
    schema.remove("then");
    ty["spec"]["offers"] = json!(["#tuning.v1"]);
    let api = p.manifest.as_ref().unwrap()["apiVersion"].clone();
    p.documents.extend([
        json!({"apiVersion":api,"kind":"Capability","metadata":{"name":"tuning.v1"},"spec":{"operations":["#set-admission.v1"]}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"tuning"},"spec":{"capability":"#tuning.v1","operations":["#set-admission.v1"]}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"tune-login"},"from":"#tuning",
            "consumer":{"kind":"Interface","name":"root"},"target":{"kind":"Resource","name":"login"}}),
    ]);
    p
}

#[test]
fn admission_control_pair_refused_before_dispatch() {
    let (state, c, _) = run_case(REPORT, control_project(), 15);
    let opened = root_call(
        &state,
        &c,
        50,
        "access.open.v1",
        json!({"conduit":"tune-login"}),
    );
    assert_eq!(opened.reply["isError"], false);
    let state = opened.state;
    let bad = json!({"revision":subject(&state,"Resource","login")["revision"],
        "levers":{"billing":"included","paidUsage":"refuse","whenUnknown":"admit"}});
    let call = |state: &State, serial, args: Value, key: Option<String>| {
        step(
            state,
            Input::Invoke {
                evidence: evidence(serial),
                key,
                archive: None,
                request: serde_json::from_value(json!({
                    "identity":{"kind":"instance","id":subject(state,"Interface","root")["id"]},
                    "tool":"tune-login.set-admission.v1","arguments":args
                }))
                .unwrap(),
            },
            &c,
        )
        .unwrap()
    };
    let t = call(&state, 51, bad.clone(), None);
    rejected(
        &t.reply,
        &["/admission/whenUnknown", "/admission/paidUsage"],
    );
    let text = t.reply["structuredContent"]["diagnostics"][0]["message"]["text"]
        .as_str()
        .unwrap();
    assert!(
        text.contains("Type example.org/conformance/shared-login#login-holder.v1:"),
        "unquoted Type: {text}"
    );
    assert!(t.external_intents.is_empty());
    assert_eq!(t.state.invocations, state.invocations);
    let mut good = bad.clone();
    good["levers"]["paidUsage"] = json!("allow");
    let sent = call(&state, 52, good, Some("admission-key".into()));
    assert!(!sent.external_intents.is_empty());
    let repeated = call(&sent.state, 53, bad, Some("admission-key".into()));
    rejected(
        &repeated.reply,
        &["/admission/whenUnknown", "/admission/paidUsage"],
    );
    assert!(repeated.external_intents.is_empty());
    assert_eq!(repeated.state.invocations, sent.state.invocations);
}

fn report_one() -> Value {
    let case: Value = serde_json::from_str(REPORT).unwrap();
    let mut report =
        case["steps"][17]["input"]["completion"]["result"]["structuredContent"]["admission"]
            .clone();
    report["observedAt"] = json!("2026-11-21T12:00:00Z");
    report
}

fn limits_member() -> Value {
    let case: Value = serde_json::from_str(REPORT).unwrap();
    case["steps"][20]["input"]["completion"]["result"]["_meta"]["io.github.shrav-k.conduit/limits"]
        .clone()
}

fn result(outcome: &str, output: Value) -> Value {
    json!({"isError":outcome != "succeeded","structuredContent":output,
        "_meta":{"io.github.shrav-k.conduit/outcome":outcome}})
}

fn report_map(state: &State) -> Value {
    serde_json::to_value(state).unwrap()["admission_reports"].clone()
}

struct GuardScenario {
    state: State,
    c: Context,
    serial: u32,
}

impl GuardScenario {
    fn new(project: packages::Project) -> Self {
        let (state, c, _) = run_case(REPORT, project, 15);
        Self {
            state,
            c,
            serial: 100,
        }
    }

    fn evidence(&mut self, time: &str) -> Evidence {
        self.serial += 1;
        let mut evidence = evidence(self.serial);
        evidence.time = time.into();
        evidence
    }

    // Put only the ask's future invocation in place; no production transition asks yet.
    fn ask(&mut self, time: &str) -> String {
        let evidence = self.evidence(time);
        let id = evidence.ids[0].clone();
        let target =
            registration::publication_object(subject(&self.state, "Resource", "login"), &self.c)
                .unwrap();
        let op = self.c.qualified("admission-hook.v1");
        let (_, _, contract) = schemas::operation(&target, &op, &self.c.core_package).unwrap();
        let implementation = target["resolved_implementations"][&op].clone();
        assert!(!implementation.is_null());
        assert!(self.state.allocated_ids.insert(id.clone()));
        self.state.invocations.push(json!({
            "id":id,"key":null,"hook":true,"sender":null,"caller":null,
            "target":target["id"],"operation":op,"arguments":{},"filters":[],
            "state":"in-progress","plain":false,"contract":contract,"held_target":target,
            "implementation":implementation,"needs_result":false,"attempt":1,
            "config":target["config"],"revision":target["revision"],"discovery":"settled"
        }));
        id
    }

    fn finish(&mut self, invocation: &str, time: &str, result: Value) -> Transition {
        let evidence = self.evidence(time);
        let t = step(
            &self.state,
            Input::Complete {
                evidence,
                invocation: invocation.into(),
                attempt: 1,
                completion: requests::Completion::Result { result },
            },
            &self.c,
        )
        .unwrap();
        self.state = t.state.clone();
        t
    }

    fn keep(&mut self, time: &str, report: &Value, limits: Option<Value>) -> (String, Transition) {
        let id = self.ask(time);
        let mut output = result("succeeded", json!({"admission":report}));
        if let Some(limits) = limits {
            output["_meta"]["io.github.shrav-k.conduit/limits"] = limits;
        }
        let t = self.finish(&id, time, output);
        assert_eq!(
            t.reply["_meta"]["io.github.shrav-k.conduit/outcome"],
            "succeeded"
        );
        (id, t)
    }

    fn call(&mut self, time: &str, tool: &str, arguments: Value) -> Transition {
        let evidence = self.evidence(time);
        let t = step(&self.state, Input::Invoke {
            evidence, key: None, archive: None,
            request: serde_json::from_value(json!({
                "identity":{"kind":"instance","id":subject(&self.state,"Interface","root")["id"]},
                "tool":tool,"arguments":arguments
            })).unwrap(),
        }, &self.c).unwrap();
        self.state = t.state.clone();
        t
    }

    fn dispatch(&mut self, time: &str, tool: &str, arguments: Value) -> String {
        let t = self.call(time, tool, arguments);
        assert!(
            !t.external_intents.is_empty(),
            "scripted call must dispatch"
        );
        let id = t.reply["invocation"]
            .as_str()
            .expect("pending invocation")
            .to_owned();
        if self
            .state
            .invocations
            .iter()
            .find(|v| v["id"] == id)
            .unwrap()["discovery"]
            == "pending"
        {
            let requests::Completion::Result { result } = requests::advertised_extension(&self.c)
            else {
                unreachable!()
            };
            let t = self.finish(&id, time, result);
            assert!(
                !t.external_intents.is_empty(),
                "discovery dispatches the scripted call"
            );
        }
        id
    }

    fn inspect(&mut self, time: &str, actor: &str, route: &str, kind: &str, name: &str) -> Value {
        let evidence = self.evidence(time);
        let before = report_map(&self.state);
        let t = step(&self.state, Input::Invoke {
            evidence, key: None, archive: None,
            request: serde_json::from_value(json!({
                "identity":{"kind":"instance","id":subject(&self.state,"Interface",actor)["id"]},
                "tool":format!("{route}.inspect.v1"),"arguments":{"object":{"kind":kind,"name":name}}
            })).unwrap(),
        }, &self.c).unwrap();
        assert_eq!(
            t.reply["isError"], false,
            "inspect {actor} {route} {kind}/{name}: {}",
            t.reply["structuredContent"]["diagnostics"]
        );
        assert!(t.external_intents.is_empty());
        assert_eq!(
            report_map(&t.state),
            before,
            "inspection must not change kept reports"
        );
        t.reply["structuredContent"]["status"]["admission"].clone()
    }

    fn login_status(&mut self, time: &str) -> Value {
        self.inspect(time, "root", "logins", "Resource", "login")
    }
}

fn end_record<'a>(t: &'a Transition, invocation: &str) -> &'a Value {
    t.records
        .iter()
        .find(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == invocation)
        .expect("invocation end record")
}

#[test]
fn admission_report_kept_with_its_validity() {
    let mut s = GuardScenario::new(guarded());
    let r1 = report_one();
    let (id, t) = s.keep("2026-11-21T13:01:00Z", &r1, None);
    let record = end_record(&t, &id);
    assert_eq!(
        record["data"]["admission"], r1,
        "hook end record keeps the report"
    );
    assert_eq!(
        t.state
            .records
            .iter()
            .find(|r| r["id"] == record["id"])
            .unwrap()["data"]["admission"],
        r1
    );
    let mut expected = json!({"checked":true,"report":r1,"invocation":id,
        "keptAt":"2026-11-21T13:01:00Z","validUntil":"2026-11-21T14:01:00Z",
        "state":"current","ended":null,"asking":null,"noReport":null});
    assert_eq!(s.login_status("2026-11-21T14:00:59Z"), expected);
    expected["state"] = json!("ended");
    expected["ended"] = json!("valid-for-ended");
    assert_eq!(s.login_status("2026-11-21T14:01:00Z"), expected);
    let mut invalid = r1.clone();
    invalid.as_object_mut().unwrap().remove("singleUse");
    for (time, outcome, output, wanted) in [
        (
            "2026-11-21T14:02:00Z",
            "succeeded",
            json!({"admission":invalid}),
            "unknown",
        ),
        ("2026-11-21T14:03:00Z", "failed", json!({}), "failed"),
    ] {
        let id = s.ask(time);
        let t = s.finish(&id, time, result(outcome, output));
        assert_eq!(
            t.reply["_meta"]["io.github.shrav-k.conduit/outcome"],
            wanted
        );
        assert!(end_record(&t, &id)["data"].get("admission").is_none());
        assert_eq!(s.login_status(time), expected);
    }
    let mut r2 = r1.clone();
    r2["reason"] = json!("cannot-tell");
    r2["cannotTell"] = json!("never-observed");
    r2["lever"] = json!({"pointer":"/admission/whenUnknown","value":"one-trial"});
    r2["validForSeconds"] = json!(0);
    r2["singleUse"] = json!(true);
    let (single, _) = s.keep("2026-11-21T14:05:00Z", &r2, None);
    expected = json!({"checked":true,"report":r2,"invocation":single,
        "keptAt":"2026-11-21T14:05:00Z","validUntil":null,"state":"current",
        "ended":null,"asking":null,"noReport":null});
    assert_eq!(s.login_status("2026-11-22T14:05:00Z"), expected);
    let purged = s.call(
        "2026-11-22T14:06:00Z",
        "management.purge.v1",
        json!({"before":"2026-11-22T14:06:00Z"}),
    );
    assert_eq!(purged.reply["isError"], false);
    assert!(
        s.state
            .records
            .iter()
            .all(|r| r["data"].get("admission").is_none())
    );
    s.state = serde_json::from_value(serde_json::to_value(&s.state).unwrap()).unwrap();
    assert_eq!(s.login_status("2026-11-22T14:07:00Z"), expected);

    // A synthetic call through a Conduit cannot publish an engine-owned report.
    let previous = report_map(&s.state);
    let forwarded = s.ask("2026-11-22T14:08:00Z");
    let conduit = subject(&s.state, "Conduit", "call-a")["id"].clone();
    s.state.invocations.last_mut().unwrap()["conduit"] = conduit;
    let t = s.finish(
        &forwarded,
        "2026-11-22T14:08:00Z",
        result("succeeded", json!({"admission":r1})),
    );
    assert!(
        end_record(&t, &forwarded)["data"]
            .get("admission")
            .is_none()
    );
    assert_eq!(report_map(&s.state), previous);

    // Count absolute instants and preserve fractional precision beyond nanoseconds.
    let mut short = r1.clone();
    short["validForSeconds"] = json!(1.0);
    s.keep("2026-11-23T15:00:00.0000000002+02:00", &short, None);
    let before = s.login_status("2026-11-23T13:00:01.0000000001Z");
    assert_eq!(before["state"], "current");
    assert_eq!(before["validUntil"], "2026-11-23T13:00:01.0000000002Z");
    assert_eq!(
        s.login_status("2026-11-23T13:00:01.000000000200Z")["ended"],
        "valid-for-ended"
    );
}

#[test]
fn admission_report_ended_early() {
    let mut s = GuardScenario::new(control_project());
    let opened = s.call(
        "2026-11-21T13:00:00Z",
        "access.open.v1",
        json!({"conduit":"tune-login"}),
    );
    assert_eq!(opened.reply["isError"], false);
    let r1 = report_one();
    s.keep("2026-11-21T13:01:00Z", &r1, None);
    assert_eq!(s.login_status("2026-11-21T13:01:00Z")["state"], "current");
    let mut malformed = limits_member();
    malformed["paidUsage"] = json!({"possible":null,"inUse":null,"resetsAt":null,"amount":1});
    for (tool, limits) in [
        ("call-a.turn.v1", None),
        ("call-b.turn.v1", Some(malformed)),
    ] {
        let id = s.dispatch("2026-11-21T13:01:10Z", tool, json!({}));
        let mut output = result("succeeded", json!({}));
        if let Some(limits) = limits {
            output["_meta"]["io.github.shrav-k.conduit/limits"] = limits;
        }
        s.finish(&id, "2026-11-21T13:01:10Z", output);
        assert_eq!(s.login_status("2026-11-21T13:01:10Z")["state"], "current");
    }
    let id = s.dispatch("2026-11-21T13:02:00Z", "call-b.turn.v1", json!({}));
    let mut output = result("succeeded", json!({}));
    output["_meta"]["io.github.shrav-k.conduit/limits"] = limits_member();
    s.finish(&id, "2026-11-21T13:02:00Z", output.clone());
    let ended = s.login_status("2026-11-21T13:02:00Z");
    assert_eq!(ended["state"], "ended");
    assert_eq!(ended["ended"], "newer-limits");
    assert_eq!(ended["report"], r1);
    let mut newer = r1.clone();
    newer["observedAt"] = json!("2026-11-21T13:03:00Z");
    s.keep("2026-11-21T13:03:00Z", &newer, Some(limits_member()));
    assert_eq!(s.login_status("2026-11-21T13:03:00Z")["state"], "current");
    for (outcome, supplied) in [("failed", true), ("unknown", true), ("succeeded", false)] {
        let mut args = json!({"revision":subject(&s.state,"Resource","login")["revision"]});
        if supplied {
            args["levers"] =
                json!({"billing":"included","paidUsage":"allow","whenUnknown":"admit"});
        }
        let id = s.dispatch("2026-11-21T13:03:10Z", "tune-login.set-admission.v1", args);
        s.finish(&id, "2026-11-21T13:03:10Z", result(outcome, json!({})));
        assert_eq!(s.login_status("2026-11-21T13:03:10Z")["state"], "current");
        if outcome == "unknown" {
            let reconciled = s.call(
                "2026-11-21T13:03:11Z",
                "logins.reconcile.v1",
                json!({"invocation":id,"occurred":true,"checked":"scripted observation"}),
            );
            assert_eq!(reconciled.reply["isError"], false);
            assert_eq!(s.login_status("2026-11-21T13:03:11Z")["state"], "current");
        }
    }
    let config = subject(&s.state, "Resource", "login")["config"].clone();
    let args = json!({"revision":subject(&s.state,"Resource","login")["revision"],
        "levers":{"billing":"included","paidUsage":"allow","whenUnknown":"admit"}});
    let id = s.dispatch("2026-11-21T13:04:00Z", "tune-login.set-admission.v1", args);
    let t = s.finish(&id, "2026-11-21T13:04:00Z", result("succeeded", json!({})));
    assert!(t.external_intents.is_empty());
    assert_eq!(subject(&s.state, "Resource", "login")["config"], config);
    let ended = s.login_status("2026-11-21T13:04:00Z");
    assert_eq!(ended["state"], "ended");
    assert_eq!(ended["ended"], "levers-changed");
    assert!(t.records.iter().all(|r| r["data"].get("hold").is_none()));
    let mut short = r1.clone();
    short["validForSeconds"] = json!(60);
    s.keep("2026-11-21T13:05:00Z", &short, None);
    let id = s.dispatch("2026-11-21T13:05:30Z", "call-b.turn.v1", json!({}));
    s.finish(&id, "2026-11-21T13:07:00Z", output);
    assert_eq!(
        s.login_status("2026-11-21T13:07:00Z")["ended"],
        "valid-for-ended"
    );

    // REV-007: retirement itself and a success after retirement store no levers.
    s.keep("2026-11-21T13:08:00Z", &r1, None);
    let args = json!({"revision":subject(&s.state,"Resource","login")["revision"],
        "levers":{"billing":"included","paidUsage":"allow","whenUnknown":"admit"}});
    let id = s.dispatch("2026-11-21T13:08:00Z", "tune-login.set-admission.v1", args);
    s.state
        .objects
        .iter_mut()
        .find(|o| o["name"] == "login")
        .unwrap()["state"] = json!("retired");
    let before = report_map(&s.state);
    s.finish(&id, "2026-11-21T13:08:01Z", result("succeeded", json!({})));
    assert_eq!(report_map(&s.state), before);
}

#[test]
fn admission_status_shows_the_login_or_not_checked() {
    let mut s = GuardScenario::new(guarded());
    let none = json!({"checked":true,"state":"none","report":null,"invocation":null,
        "keptAt":null,"validUntil":null,"ended":null,"asking":null,"noReport":null});
    assert_eq!(s.login_status("2026-11-21T13:00:00Z"), none);
    assert_eq!(
        s.inspect("2026-11-21T13:00:00Z", "root", "workers", "Agent", "a"),
        none
    );
    let report = report_one();
    let (id, _) = s.keep("2026-11-21T13:01:00Z", &report, None);
    let login = s.login_status("2026-11-21T13:01:00Z");
    assert_eq!(login["report"], report);
    assert_eq!(login["invocation"], id);
    assert_eq!(
        s.inspect("2026-11-21T13:01:00Z", "root", "workers", "Agent", "a"),
        login
    );
    let opened = s.call(
        "2026-11-21T13:01:00Z",
        "access.open.v1",
        json!({"conduit":"observer-scope"}),
    );
    assert_eq!(opened.reply["isError"], false);
    let mut observer = login;
    observer["invocation"] = Value::Null;
    assert_eq!(
        s.inspect(
            "2026-11-21T13:01:00Z",
            "observer",
            "observer-scope",
            "Agent",
            "a"
        ),
        observer
    );
    // A non-null lever also reaches an actor allowed to inspect only the Agent.
    let mut report = report;
    report["reason"] = json!("cannot-tell");
    report["cannotTell"] = json!("never-observed");
    report["lever"] = json!({"pointer":"/admission/whenUnknown","value":"admit"});
    s.keep("2026-11-21T13:01:01Z", &report, None);
    let inspected = s.inspect(
        "2026-11-21T13:01:01Z",
        "observer",
        "observer-scope",
        "Agent",
        "a",
    );
    assert_eq!(inspected["report"], report);
    assert!(inspected["invocation"].is_null());
    assert!(
        s.inspect(
            "2026-11-21T13:01:00Z",
            "root",
            "system",
            "Resource",
            "local"
        )
        .is_null()
    );
    assert!(
        s.inspect(
            "2026-11-21T13:01:00Z",
            "root",
            "management",
            "Resource",
            "login-holder"
        )
        .is_null()
    );

    let old = project(
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/conduit.json"
        ),
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/documents.json"
        ),
    );
    let (state, c, _) = run_case(
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/case.yaml"
        ),
        old,
        10,
    );
    let mut old = GuardScenario {
        state,
        c,
        serial: 500,
    };
    assert_eq!(
        old.login_status("2026-11-21T13:00:00Z"),
        json!({"checked":false})
    );
    assert_eq!(
        old.inspect("2026-11-21T13:00:00Z", "root", "workers", "Agent", "a"),
        json!({"checked":false})
    );
    let mut no_login = guarded();
    let spec = no_login
        .documents
        .iter_mut()
        .find(|d| d["metadata"]["name"] == "shared-agent.v1")
        .unwrap()["spec"]
        .as_object_mut()
        .unwrap();
    spec.remove("admission");
    spec.remove("loginResource");
    let (state, c, _) = run_case(REPORT, no_login, 2);
    let mut no_login = GuardScenario {
        state,
        c,
        serial: 600,
    };
    assert!(
        no_login
            .inspect("2026-11-21T13:00:00Z", "root", "workers", "Agent", "a")
            .is_null()
    );
}

// Order 319a: every added input is parsed through the public JSON boundary.
impl GuardScenario {
    fn admission_call(&mut self, time: &str, tool: &str) -> Transition {
        let evidence = self.evidence(time);
        let input = json!({"kind":"invoke","evidence":{"time":evidence.time,"ids":evidence.ids,"revisions":evidence.revisions,"suffixes":evidence.suffixes},"request":{
            "identity":{"kind":"instance","id":subject(&self.state,"Interface","root")["id"]},
            "tool":tool,"arguments":{}
        }});
        let t = step(&self.state, serde_json::from_value(input).unwrap(), &self.c).unwrap();
        self.state = t.state.clone();
        t
    }

    fn discover_ask(&mut self, ask: &str, time: &str) -> Transition {
        let requests::Completion::Result { result } = requests::advertised_extension(&self.c)
        else {
            unreachable!()
        };
        self.finish(ask, time, result)
    }
}

fn waiting(t: &Transition) -> (String, String) {
    assert_eq!(t.reply["pending"], true);
    let invocation = t.reply["invocation"].as_str().unwrap().to_owned();
    assert!(
        t.reply["hook"].is_string(),
        "waiting reply must name its admission ask"
    );
    let ask = t.reply["hook"].as_str().unwrap().to_owned();
    assert_ne!(invocation, ask);
    assert!(
        t.records
            .iter()
            .all(|r| r["type"] != "invocation-dispatched" || r["data"]["invocation"] != invocation)
    );
    (invocation, ask)
}

fn dispatched_ids(t: &Transition) -> Vec<Value> {
    t.external_intents
        .iter()
        .map(|v| v["invocation"].clone())
        .collect()
}

fn plain_guarded(keyed_sign_in: bool) -> packages::Project {
    let mut p = guarded();
    let item = &mut p.manifest.as_mut().unwrap()["implementations"]["resource"];
    item["plain"] = json!(true);
    item["tools"] = json!({
        "github.com/shrav-k/conduit/core#account-hook.v1":"account",
        "github.com/shrav-k/conduit/core#sign-in-hook.v1":"sign-in",
        "github.com/shrav-k/conduit/core#sign-out-hook.v1":"sign-out",
        "github.com/shrav-k/conduit/core#admission-hook.v1":"admission",
        "#set-admission.v1":"set-admission"
    });
    if keyed_sign_in {
        item["keyed"] = json!(["github.com/shrav-k/conduit/core#sign-in-hook.v1"]);
    }
    p
}

fn plain_report_state(keyed_sign_in: bool) -> GuardScenario {
    let (state, c, captures) = run_case(REPORT, plain_guarded(keyed_sign_in), 13);
    let account = captures["accountcheck"].as_str().unwrap();
    let check = state
        .invocations
        .iter()
        .find(|v| v["id"] == account)
        .unwrap();
    assert_eq!(check["plain"], true);
    assert!(check.get("discovery").is_none());
    let mut s = GuardScenario {
        state,
        c,
        serial: 100,
    };
    let t = s.finish(
        account,
        "2026-11-21T13:00:00Z",
        json!({
            "isError":false,"content":[{"type":"text","text":"example-account"}],
            "structuredContent":{"account":"example-account"}
        }),
    );
    assert_eq!(ask_end(&t, account)["outcome"], "succeeded");
    assert!(t.external_intents.is_empty());
    s
}

fn plain_admission_result() -> Value {
    json!({"isError":false,"content":[{"type":"text","text":"admitted"}],
        "structuredContent":{"admission":report_one()},
        "_meta":{"io.github.shrav-k.conduit/limits":limits_member()}})
}

fn ask_end<'a>(t: &'a Transition, ask: &str) -> &'a Value {
    let ends: Vec<_> = t
        .records
        .iter()
        .filter(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == ask)
        .collect();
    assert_eq!(ends.len(), 1);
    &ends[0]["data"]
}

fn no_plain_ask_discovery(t: &Transition, ask: &str) {
    // IFC-037 still requires discovery for the admitted Agent's keyed turn.
    assert!(
        t.external_intents
            .iter()
            .all(|intent| intent["call"]["method"] != "server/discover"
                || intent["invocation"] != ask)
    );
}

#[test]
fn admission_ask_sent_plain_to_a_plain_item() {
    let time = "2026-11-21T13:01:00Z";
    for keyed_sign_in in [false, true] {
        let mut s = plain_report_state(keyed_sign_in);
        let t = s.admission_call(time, "call-a.turn.v1");
        let (a, k) = waiting(&t);
        assert_eq!(t.external_intents.len(), 1);
        let intent = &t.external_intents[0];
        assert_eq!(intent["kind"], "dispatch");
        assert_eq!(intent["invocation"], k);
        assert_eq!(
            intent["target"],
            subject(&s.state, "Resource", "login")["id"]
        );
        assert_eq!(intent["call"]["method"], "tools/call");
        let params = &intent["call"]["params"];
        assert_eq!(params["name"], "admission");
        assert_eq!(
            params["arguments"],
            json!({
                "levers":{"billing":"included","paidUsage":"refuse","whenUnknown":"refuse"},
                "limits":{"state":"unknown","windows":[]},"previous":null
            })
        );
        let meta = &params["_meta"];
        assert!(
            meta.as_object()
                .into_iter()
                .flatten()
                .all(|(key, _)| !key.starts_with(&s.c.meta_prefix))
        );
        assert!(
            meta["io.modelcontextprotocol/clientCapabilities"]["extensions"]
                .get(format!("{}implementation-interface", s.c.meta_prefix))
                .is_none()
        );
        let ask = s.state.invocations.iter().find(|v| v["id"] == k).unwrap();
        assert_eq!(ask["plain"], true);
        assert!(ask.get("discovery").is_none());
        let records: Vec<_> = t
            .records
            .iter()
            .filter(|r| r["type"] == "invocation-dispatched" && r["data"]["invocation"] == k)
            .collect();
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0]["data"]["caller"],
            subject(&s.state, "Interface", "root")["id"]
        );
        assert_eq!(records[0]["data"]["causedBy"], a);
        assert!(records[0]["data"].get("serving").is_none());
        let starts: Vec<_> = t
            .decisions
            .iter()
            .filter(|d| d.action == "start-admission-ask")
            .collect();
        assert_eq!(starts.len(), 1);
        assert_eq!(starts[0].facts["invocation"], k);
        assert_eq!(starts[0].facts["causedBy"], a);
        assert!(
            t.decisions
                .iter()
                .all(|d| d.action != "read-implementation-discovery")
        );
        no_plain_ask_discovery(&t, &k);
        let t = s.admission_call(time, "call-b.turn.v1");
        let (b, shared) = waiting(&t);
        assert_eq!(shared, k);
        assert!(t.external_intents.is_empty());
        let t = s.finish(&k, time, plain_admission_result());
        let ended = ask_end(&t, &k);
        assert_eq!(ended["outcome"], "succeeded");
        assert_eq!(ended["admission"], report_one());
        assert_eq!(ended["limits"], limits_member());
        assert_eq!(ended["causedBy"], a);
        assert_eq!(dispatched_ids(&t), vec![json!(a), json!(b)]);
        no_plain_ask_discovery(&t, &k);
        let status = s.login_status(time);
        assert_eq!(status["state"], "current");
        assert_eq!(status["report"], report_one());
    }
}

#[test]
fn admission_plain_ask_answers_read_as_plain() {
    let time = "2026-11-21T13:01:00Z";
    let mut invalid = report_one();
    invalid.as_object_mut().unwrap().remove("singleUse");
    for (completion, outcome) in [
        (
            json!({"kind":"result","result":{"isError":true,"content":[{"type":"text","text":"read failed"}]}}),
            "failed",
        ),
        (
            json!({"kind":"protocol-error","code":-32603,"data":{}}),
            "failed",
        ),
        (
            json!({"kind":"protocol-error","code":-32020,"data":{}}),
            "repair",
        ),
        (
            json!({"kind":"result","result":{"isError":false,"structuredContent":{"admission":invalid}}}),
            "unknown",
        ),
        (
            json!({"kind":"result","result":{"resultType":"unrecognized"}}),
            "unknown",
        ),
        (
            json!({"kind":"result","result":{"resultType":"input_required"}}),
            "unknown",
        ),
        (json!({"kind":"missing"}), "unknown"),
        (Value::Null, "deadline"),
    ] {
        let mut s = plain_report_state(false);
        let started = s.admission_call(time, "call-a.turn.v1");
        let (a, k) = waiting(&started);
        let (end_time, input) = if outcome == "deadline" {
            (
                "2026-11-21T13:01:30Z",
                json!({"kind":"admission-ask-deadline","invocation":k,"attempt":1}),
            )
        } else {
            (
                time,
                json!({"kind":"complete","invocation":k,"attempt":1,"completion":completion}),
            )
        };
        let t = s.admission_input(end_time, input);
        if outcome == "repair" {
            let ask = s.state.invocations.iter().find(|v| v["id"] == k).unwrap();
            assert_eq!(ask["state"], "in-progress");
            let waiter = s.state.invocations.iter().find(|v| v["id"] == a).unwrap();
            assert_eq!(waiter["admission_wait"], k);
            assert!(
                t.records
                    .iter()
                    .all(|r| r["type"] != "invocation-ended" || r["data"]["invocation"] != k)
            );
            assert_eq!(dispatched_ids(&t), vec![json!(k)]);
            assert_eq!(t.external_intents[0]["kind"], "refresh-tools-and-call");
            assert_eq!(
                t.external_intents[0]["call"],
                started.external_intents[0]["call"]
            );
            assert_eq!(
                t.decisions
                    .iter()
                    .filter(|d| d.action == "repair-parameter-headers-once")
                    .count(),
                1
            );
            no_plain_ask_discovery(&t, &k);
            let t = s.finish(&k, time, plain_admission_result());
            assert_eq!(ask_end(&t, &k)["outcome"], "succeeded");
            assert_eq!(dispatched_ids(&t), vec![json!(a)]);
            no_plain_ask_discovery(&t, &k);
        } else {
            let ended = ask_end(&t, &k);
            assert_eq!(
                ended["outcome"],
                if outcome == "deadline" {
                    "unknown"
                } else {
                    outcome
                }
            );
            assert!(ended.get("admission").is_none());
            assert_admission_hold(
                &s,
                &t,
                hold_reply(&t, &a),
                no_report_fields(
                    json!(k),
                    outcome,
                    if outcome == "deadline" {
                        "2026-11-21T13:02:30Z"
                    } else {
                        "2026-11-21T13:02:00Z"
                    },
                ),
            );
            if outcome == "deadline" {
                assert_eq!(dispatched_ids(&t), vec![json!(k)]);
                assert_eq!(t.external_intents[0]["kind"], "cancel");
            } else {
                assert!(t.external_intents.is_empty());
            }
            no_plain_ask_discovery(&t, &k);
        }
        no_plain_ask_discovery(&started, &k);
    }
}

#[test]
fn admission_ask_started_once_for_waiting_calls() {
    let time = "2026-11-21T13:01:00Z";
    let mut s = GuardScenario::new(guarded());
    let t = s.admission_call(time, "call-a.turn.v1");
    let (a, k) = waiting(&t);
    assert_eq!(t.external_intents.len(), 1);
    assert_eq!(t.external_intents[0]["invocation"], k);
    assert_eq!(
        t.external_intents[0]["target"],
        subject(&s.state, "Resource", "login")["id"]
    );
    assert_eq!(t.external_intents[0]["call"]["method"], "server/discover");
    assert_eq!(
        t.records
            .iter()
            .filter(|r| r["type"] == "access-decision" && r["data"]["invocation"] == a)
            .count(),
        1
    );
    let t = s.admission_call(time, "call-b.turn.v1");
    let (b, shared) = waiting(&t);
    assert_eq!(shared, k);
    assert!(t.external_intents.is_empty());
    for status in [
        s.login_status(time),
        s.inspect(time, "root", "workers", "Agent", "a"),
    ] {
        assert_eq!(status["asking"], k);
        assert_eq!(status["state"], "none");
    }
    s.call(time, "access.open.v1", json!({"conduit":"observer-scope"}));
    assert!(s.inspect(time, "observer", "observer-scope", "Agent", "a")["asking"].is_null());
    let t = s.discover_ask(&k, time);
    assert_eq!(dispatched_ids(&t), vec![json!(k)]);
    let call = &t.external_intents[0]["call"]["params"];
    assert_eq!(
        call["arguments"],
        json!({"levers":{"billing":"included","paidUsage":"refuse","whenUnknown":"refuse"},"limits":{"state":"unknown","windows":[]},"previous":null})
    );
    let root = subject(&s.state, "Interface", "root")["id"].clone();
    assert_eq!(call["_meta"]["io.github.shrav-k.conduit/caller"], root);
    assert_eq!(
        call["_meta"]["io.github.shrav-k.conduit/filters"],
        json!([])
    );
    assert!(
        call["_meta"]
            .get("io.github.shrav-k.conduit/conduit")
            .is_none()
    );
    let record = t
        .records
        .iter()
        .find(|r| r["type"] == "invocation-dispatched")
        .unwrap();
    assert_eq!(record["data"]["caller"], root);
    assert_eq!(record["data"]["causedBy"], a);
    for field in ["serving", "conduit", "consumer"] {
        assert!(record["data"].get(field).is_none());
    }
    let r1 = report_one();
    let t = s.finish(&k, time, result("succeeded", json!({"admission":r1})));
    let ended = t
        .records
        .iter()
        .find(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == k)
        .unwrap();
    assert_eq!(ended["data"]["admission"], r1);
    assert_eq!(ended["data"]["causedBy"], a);
    assert_eq!(dispatched_ids(&t), vec![json!(a), json!(b)]);
    assert!(t.records.iter().all(|r| r["type"] != "access-decision"));
    for intent in &t.external_intents {
        let v = t
            .state
            .invocations
            .iter()
            .find(|v| v["id"] == intent["invocation"])
            .unwrap();
        let expected = if v["discovery"] == "pending" {
            requests::discovery_intent(v, &s.c)
        } else {
            requests::dispatch_intent(v, false, &s.c)
        };
        assert_eq!(*intent, expected);
    }
    let status = s.login_status(time);
    assert!(status["asking"].is_null());
    assert_eq!(status["state"], "current");
    let t = s.admission_call("2026-11-21T13:02:00Z", "call-a.turn.v1");
    assert!(t.reply["hook"].is_null());
    assert_eq!(dispatched_ids(&t), vec![t.reply["invocation"].clone()]);

    // REC-007 ends an ask through RecordFault rather than requests::complete.
    // Decision 2 requires every end to remove the active ask, and decision 3
    // requires its waiters to be decided in that transition.
    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    s.discover_ask(&k, time);
    let evidence = s.evidence(time);
    let input = json!({"kind":"complete", "invocation":k, "attempt":1,
        "evidence":{"time":evidence.time,"ids":evidence.ids,
            "revisions":evidence.revisions,"suffixes":evidence.suffixes},
        "completion":{"kind":"missing"}});
    let trial = step(
        &s.state,
        serde_json::from_value(input.clone()).unwrap(),
        &s.c,
    )
    .unwrap();
    let fault =
        RecordFault::from_trial(serde_json::from_value(input).unwrap(), &trial, false).unwrap();
    let ended = fault.step(s.state, &s.c).unwrap();
    assert!(
        ended
            .state
            .invocations
            .iter()
            .any(|v| v["id"] == k && v["state"] == "unknown")
    );
    assert!(
        serde_json::to_value(&ended.state)
            .unwrap()
            .get("admission_asks")
            .is_none(),
        "an ask ended by REC-007 must leave admission_asks"
    );
    assert!(ended.external_intents.is_empty());
    assert!(ended.records.iter().all(|r| r["type"] != "access-decision"));
    assert!(
        ended
            .state
            .invocations
            .iter()
            .all(|v| v["id"] != a || v["state"] != "in-progress")
    );
    let reply = &ended
        .request_replies
        .iter()
        .find(|r| r.invocation == a)
        .unwrap()
        .reply;
    assert_eq!(
        reply["_meta"]["io.github.shrav-k.conduit/reason"],
        "unavailable"
    );
    assert_eq!(
        reply["structuredContent"]["diagnostics"][0]["ruleId"],
        "REC-007"
    );
    s.state = ended.state;
    assert_eq!(
        s.login_status(time)["noReport"],
        json!({"invocation":k,"cause":"unknown","until":"2026-11-21T13:02:00Z"})
    );
    // The period survives serialization, ends at exactly 60 seconds, and starts no ask.
    s.state = serde_json::from_value(serde_json::to_value(&s.state).unwrap()).unwrap();
    let t = s.admission_call("2026-11-21T13:01:59Z", "call-b.turn.v1");
    assert!(t.external_intents.is_empty() && t.state.admission_asks.is_empty());
    assert_admission_hold(
        &s,
        &t,
        &t.reply,
        no_report_fields(json!(k), "unknown", "2026-11-21T13:02:00Z"),
    );
    assert!(s.login_status("2026-11-21T13:02:00Z")["noReport"].is_null());
    waiting(&s.admission_call("2026-11-21T13:02:00Z", "call-a.turn.v1"));

    // Failure before dispatch, including a failed write read back after rename:
    // suppressing the ask's discovery must also settle the original request.
    for renamed in [false, true] {
        let mut s = GuardScenario::new(guarded());
        let evidence = s.evidence(time);
        let invoke = json!({"kind":"invoke",
            "evidence":{"time":evidence.time,"ids":evidence.ids,"revisions":evidence.revisions,"suffixes":evidence.suffixes},
            "request":{"identity":{"kind":"instance","id":subject(&s.state,"Interface","root")["id"]},
                "tool":"call-a.turn.v1","arguments":{}}});
        let first = step(
            &s.state,
            serde_json::from_value(invoke.clone()).unwrap(),
            &s.c,
        )
        .unwrap();
        let (a, k) = waiting(&first);
        let (input, trial, before) = if renamed {
            let before = first.state.clone();
            (invoke, first, before)
        } else {
            let evidence = s.evidence(time);
            let input = json!({"kind":"complete","invocation":k,"attempt":1,
                "evidence":{"time":evidence.time,"ids":evidence.ids,"revisions":evidence.revisions,"suffixes":evidence.suffixes},
                "completion":{"kind":"missing"}});
            let trial = step(
                &first.state,
                serde_json::from_value(input.clone()).unwrap(),
                &s.c,
            )
            .unwrap();
            (input, trial, first.state)
        };
        let fault =
            RecordFault::from_trial(serde_json::from_value(input).unwrap(), &trial, renamed)
                .unwrap();
        let ended = fault.step(before, &s.c).unwrap();
        assert!(ended.external_intents.is_empty());
        assert!(
            serde_json::to_value(&ended.state)
                .unwrap()
                .get("admission_asks")
                .is_none()
        );
        assert!(ended.request_replies.iter().any(|r| r.invocation == a
            && r.reply["_meta"]["io.github.shrav-k.conduit/reason"] == "unavailable"));
        s.state = ended.state;
        assert_eq!(
            s.login_status(time)["noReport"],
            json!({"invocation":null,"cause":"not-dispatched","until":"2026-11-21T13:02:00Z"})
        );
    }
}

#[test]
fn admission_single_use_report_spent_once() {
    let time = "2026-11-21T13:01:00Z";
    let case: Value = serde_json::from_str(include_str!(
        "../../../../conformance/conventions/requests/admission-single-use/case.yaml"
    ))
    .unwrap();
    let r3 = case["steps"][18]["input"]["completion"]["result"]["structuredContent"]["admission"]
        .clone();
    assert_eq!(r3["singleUse"], true);
    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    let (b, _) = waiting(&s.admission_call(time, "call-b.turn.v1"));
    assert_eq!(
        s.call(time, "access.close.v1", json!({"conduit":"call-a"}))
            .reply["isError"],
        false
    );
    s.discover_ask(&k, time);
    let t = s.finish(&k, time, result("succeeded", json!({"admission":r3})));
    assert_eq!(dispatched_ids(&t), vec![json!(b)]);
    let rejected = t
        .request_replies
        .iter()
        .find(|r| r.invocation == a)
        .unwrap();
    assert_eq!(
        rejected.reply["structuredContent"]["diagnostics"][0]["ruleId"],
        "PND-016"
    );
    assert_eq!(s.login_status(time)["ended"], "single-use");
    s.keep("2026-11-21T13:02:00Z", &r3, None);
    let t = s.admission_call("2026-11-21T13:02:10Z", "call-b.turn.v1");
    assert!(t.reply["hook"].is_null());
    assert_eq!(dispatched_ids(&t), vec![t.reply["invocation"].clone()]);
    assert_eq!(
        s.login_status("2026-11-21T13:02:10Z")["ended"],
        "single-use"
    );
    let (_, k2) = waiting(&s.admission_call("2026-11-21T13:02:20Z", "call-b.turn.v1"));
    let t = s.discover_ask(&k2, "2026-11-21T13:02:20Z");
    assert_eq!(
        t.external_intents[0]["call"]["params"]["arguments"]["previous"],
        r3
    );
    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    let (b, _) = waiting(&s.admission_call(time, "call-b.turn.v1"));
    s.discover_ask(&k, time);
    let t = s.finish(&k, time, result("succeeded", json!({"admission":r3})));
    assert_eq!(dispatched_ids(&t), vec![json!(a)]);
    assert_admission_hold(
        &s,
        &t,
        hold_reply(&t, &b),
        json!({"report":null,"reportInvocation":k,"noReport":null,"singleUse":a,"lifts":"after-call","retryAt":"2026-11-21T13:02:00Z"}),
    );
    assert_eq!(s.login_status(time)["ended"], "single-use");
}

#[test]
fn admission_wait_checks_dispatch_once() {
    let time = "2026-11-21T13:01:00Z";
    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    let (b, _) = waiting(&s.admission_call(time, "call-b.turn.v1"));
    assert_eq!(
        s.call(
            "2026-11-21T13:01:05Z",
            "access.close.v1",
            json!({"conduit":"call-b"})
        )
        .reply["isError"],
        false
    );
    s.discover_ask(&k, time);
    let t = s.finish(
        &k,
        "2026-11-21T13:01:10Z",
        result("succeeded", json!({"admission":report_one()})),
    );
    assert_eq!(dispatched_ids(&t), vec![json!(a)]);
    let reply = &t
        .request_replies
        .iter()
        .find(|r| r.invocation == b)
        .unwrap()
        .reply;
    assert_eq!(
        reply["_meta"]["io.github.shrav-k.conduit/reason"],
        "unavailable"
    );
    let ds = reply["structuredContent"]["diagnostics"]
        .as_array()
        .unwrap();
    assert_eq!(ds.len(), 1);
    assert_eq!(ds[0]["ruleId"], "PND-016");
    let text = ds[0]["message"]["text"].as_str().unwrap();
    for word in [
        "call-b",
        "was closed while the call waited for its login's admission report.",
        "not dispatched",
        "started no model work",
        "not queued",
        "decided afresh",
    ] {
        assert!(text.contains(word), "{text}");
    }
    assert!(ds[0]["properties"].get("admission").is_none());
    let end = t
        .records
        .iter()
        .find(|r| r["type"] == "request-ended" && r["data"]["invocation"] == b)
        .unwrap();
    assert_eq!(end["data"]["ruleId"], "PND-016");
    assert_eq!(end["data"]["reason"], "unavailable");
    assert_eq!(end["data"]["dispatched"], false);
    assert!(t.records.iter().all(|r| r["type"] != "access-decision"));
    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    let login = subject(&s.state, "Resource", "login")["id"]
        .as_str()
        .unwrap()
        .to_owned();
    s.state.sign_in_holds.insert(login, json!({"since":time}));
    s.discover_ask(&k, time);
    let t = s.finish(
        &k,
        time,
        result("succeeded", json!({"admission":report_one()})),
    );
    assert!(t.external_intents.is_empty());
    let reply = &t
        .request_replies
        .iter()
        .find(|r| r.invocation == a)
        .unwrap()
        .reply;
    assert_eq!(
        reply["structuredContent"]["diagnostics"][0]["ruleId"],
        "PND-005"
    );
}

// Order 319b inputs cross the public JSON boundary, including the new deadline.
impl GuardScenario {
    fn admission_input(&mut self, time: &str, mut input: Value) -> Transition {
        let e = self.evidence(time);
        input["evidence"] =
            json!({"time":e.time,"ids":e.ids,"revisions":e.revisions,"suffixes":e.suffixes});
        let t = step(&self.state, serde_json::from_value(input).unwrap(), &self.c).unwrap();
        self.state = t.state.clone();
        t
    }

    fn keyed_admission_call(&mut self, time: &str) -> Transition {
        self.admission_input(
            time,
            json!({"kind":"invoke","key":"waiting-turn","request":{
            "identity":{"kind":"instance","id":subject(&self.state,"Interface","root")["id"]},
            "tool":"call-a.turn.v1","arguments":{}}}),
        )
    }
}

#[test]
fn admission_ask_without_a_report_starts_none_for_60_seconds() {
    let time = "2026-11-21T13:01:00Z";
    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    s.discover_ask(&k, time);
    let t = s.finish(&k, time, result("failed", json!({})));
    assert!(t.external_intents.is_empty());
    assert_admission_hold(
        &s,
        &t,
        hold_reply(&t, &a),
        no_report_fields(json!(k), "failed", "2026-11-21T13:02:00Z"),
    );
    assert!(
        t.records
            .iter()
            .filter(|r| r["type"] == "invocation-ended")
            .all(|r| r["data"].get("admission").is_none())
    );
    let period = json!({"invocation":k,"cause":"failed","until":"2026-11-21T13:02:00Z"});
    assert_eq!(s.login_status(time)["noReport"], period);
    let t = s.admission_call("2026-11-21T13:01:30Z", "call-b.turn.v1");
    assert!(t.external_intents.is_empty() && t.state.admission_asks.is_empty());
    assert_admission_hold(
        &s,
        &t,
        &t.reply,
        no_report_fields(json!(k), "failed", "2026-11-21T13:02:00Z"),
    );
    assert_eq!(s.login_status("2026-11-21T13:01:30Z")["noReport"], period);
    assert!(s.login_status("2026-11-21T13:02:00Z")["noReport"].is_null());
    let (_, k2) = waiting(&s.admission_call("2026-11-21T13:02:00Z", "call-a.turn.v1"));
    s.discover_ask(&k2, "2026-11-21T13:02:00Z");
    let mut bad = report_one();
    bad.as_object_mut().unwrap().remove("singleUse");
    s.finish(
        &k2,
        "2026-11-21T13:02:00Z",
        result("succeeded", json!({"admission":bad})),
    );
    assert_eq!(
        s.login_status("2026-11-21T13:02:00Z")["noReport"],
        json!({"invocation":k2,"cause":"unknown","until":"2026-11-21T13:03:00Z"})
    );

    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    let t = s.finish(&k, time, json!({"capabilities":{}}));
    assert!(t.external_intents.is_empty());
    assert_admission_hold(
        &s,
        &t,
        hold_reply(&t, &a),
        no_report_fields(Value::Null, "not-dispatched", "2026-11-21T13:02:00Z"),
    );
    assert!(s.state.admission_asks.is_empty());
    assert_eq!(
        s.login_status(time)["noReport"],
        json!({"invocation":null,"cause":"not-dispatched","until":"2026-11-21T13:02:00Z"})
    );
    assert!(t.records.iter().any(|r| r["data"]["causedBy"] == a));
}

#[test]
fn admission_levers_changed_while_asking() {
    for cancelled in [false, true] {
        let mut s = GuardScenario::new(control_project());
        s.call(
            "2026-11-21T13:00:00Z",
            "access.open.v1",
            json!({"conduit":"tune-login"}),
        );
        let (a, k) = waiting(&s.admission_call("2026-11-21T13:01:00Z", "call-a.turn.v1"));
        s.discover_ask(&k, "2026-11-21T13:01:00Z");
        if cancelled {
            s.admission_input(
                "2026-11-21T13:01:01Z",
                json!({"kind":"cancel","invocation":a}),
            );
        }
        let revision = subject(&s.state, "Resource", "login")["revision"].clone();
        let control = s.dispatch("2026-11-21T13:01:02Z", "tune-login.set-admission.v1",
            json!({"revision":revision,"levers":{"billing":"included","paidUsage":"allow","whenUnknown":"admit"}}));
        s.finish(
            &control,
            "2026-11-21T13:01:02Z",
            result("succeeded", json!({})),
        );
        let r1 = report_one();
        let t = s.finish(
            &k,
            "2026-11-21T13:01:04Z",
            result("succeeded", json!({"admission":r1})),
        );
        let status = s.login_status("2026-11-21T13:01:04Z");
        assert_eq!(status["state"], "ended");
        assert_eq!(status["ended"], "levers-changed");
        assert_eq!(status["keptAt"], "2026-11-21T13:01:04Z");
        assert_eq!(status["invocation"], k);
        assert_eq!(status["report"], r1);
        assert!(status["noReport"].is_null());
        assert!(
            t.records
                .iter()
                .all(|r| r["type"] != "request-ended" || r["data"]["invocation"] != a),
            "waiting call ended: {:?}",
            t.records
                .iter()
                .filter(|r| r["type"] == "request-ended")
                .map(|r| &r["data"])
                .collect::<Vec<_>>()
        );
        if cancelled {
            assert!(t.external_intents.is_empty());
            assert!(status["asking"].is_null());
            continue;
        }
        let k2 = status["asking"].as_str().unwrap();
        assert_eq!(dispatched_ids(&t), vec![json!(k2)]);
        let t = s.discover_ask(k2, "2026-11-21T13:01:04Z");
        assert_eq!(
            t.external_intents[0]["call"]["params"]["arguments"]["previous"],
            r1
        );
        assert_eq!(
            t.external_intents[0]["call"]["params"]["arguments"]["levers"],
            json!({"billing":"included","paidUsage":"refuse","whenUnknown":"refuse"})
        );
        let record = t
            .records
            .iter()
            .find(|r| r["type"] == "invocation-dispatched")
            .unwrap();
        assert_eq!(record["data"]["causedBy"], a);
        let t = s.finish(
            k2,
            "2026-11-21T13:01:06Z",
            result("succeeded", json!({"admission":r1})),
        );
        assert_eq!(dispatched_ids(&t), vec![json!(a)]);
        let mut output = result("succeeded", json!({}));
        output["_meta"]["io.github.shrav-k.conduit/limits"] = limits_member();
        s.finish(&a, "2026-11-21T13:02:00Z", output);
        let (b, k3) = waiting(&s.admission_call("2026-11-21T13:02:10Z", "call-b.turn.v1"));
        s.discover_ask(&k3, "2026-11-21T13:02:10Z");
        let revision = subject(&s.state, "Resource", "login")["revision"].clone();
        let control = s.dispatch(
            "2026-11-21T13:02:11Z",
            "tune-login.set-admission.v1",
            json!({"revision":revision}),
        );
        s.finish(
            &control,
            "2026-11-21T13:02:11Z",
            result("succeeded", json!({})),
        );
        let t = s.finish(
            &k3,
            "2026-11-21T13:02:12Z",
            result("succeeded", json!({"admission":r1})),
        );
        assert_eq!(dispatched_ids(&t), vec![json!(b)]);
        assert_eq!(s.login_status("2026-11-21T13:02:12Z")["state"], "current");
    }
}

#[test]
fn admission_wait_cancelled_restarted_or_repeated() {
    let mut s = GuardScenario::new(guarded());
    let time = "2026-11-21T13:01:00Z";
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    let (b, _) = waiting(&s.admission_call(time, "call-b.turn.v1"));
    let t = s.admission_input(time, json!({"kind":"cancel","invocation":a}));
    assert!(
        t.external_intents.is_empty(),
        "waiting cancellation reaches no implementation"
    );
    assert_eq!(
        t.reply["_meta"]["io.github.shrav-k.conduit/reason"],
        "unavailable"
    );
    let diagnostics = &t.reply["structuredContent"]["diagnostics"];
    assert_eq!(diagnostics.as_array().unwrap().len(), 1);
    assert_eq!(diagnostics[0]["ruleId"], "PND-016");
    assert!(
        diagnostics[0]["message"]["text"]
            .as_str()
            .unwrap()
            .contains("cancelled while")
    );
    assert!(diagnostics[0]["properties"]["admission"].is_null());
    let ended = t
        .records
        .iter()
        .find(|r| r["type"] == "request-ended")
        .unwrap();
    assert_eq!(ended["data"]["cancelledWhileWaiting"], true);
    assert_eq!(ended["data"]["dispatched"], false);
    assert!(
        t.decisions
            .iter()
            .any(|d| d.facts["invocation"] == a && d.promises.iter().any(|r| r == "PND-016"))
    );
    s.discover_ask(&k, time);
    let t = s.finish(
        &k,
        time,
        result("succeeded", json!({"admission":report_one()})),
    );
    assert_eq!(dispatched_ids(&t), vec![json!(b)]);
    assert!(
        t.records
            .iter()
            .any(|r| r["type"] == "invocation-ended" && r["data"]["causedBy"] == a)
    );
    let mut output = result("succeeded", json!({}));
    output["_meta"]["io.github.shrav-k.conduit/limits"] = limits_member();
    s.finish(&b, "2026-11-21T13:02:00Z", output);
    let (a2, k2) = waiting(&s.keyed_admission_call("2026-11-21T13:03:00Z"));
    let t = s.keyed_admission_call("2026-11-21T13:03:00Z");
    assert!(t.external_intents.is_empty());
    assert_eq!(t.reply["_meta"]["io.github.shrav-k.conduit/invocation"], a2);
    assert_eq!(
        t.reply["structuredContent"]["diagnostics"][0]["ruleId"],
        "PND-016"
    );
    let message = t.reply["structuredContent"]["diagnostics"][0]["message"]["text"]
        .as_str()
        .unwrap();
    assert!(message.contains("nothing was dispatched for this request"));
    assert!(message.contains("the earlier call's reply gives its result"));
    let t = s.admission_input("2026-11-21T13:03:30Z", json!({"kind":"recover"}));
    assert!(t.external_intents.is_empty());
    let a_end = t
        .records
        .iter()
        .position(|r| r["type"] == "request-ended" && r["data"]["invocation"] == a2)
        .unwrap();
    let k_end = t
        .records
        .iter()
        .position(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == k2)
        .unwrap();
    assert!(a_end < k_end);
    assert_eq!(
        s.login_status("2026-11-21T13:03:30Z")["noReport"],
        json!({"invocation":k2,"cause":"unknown","until":"2026-11-21T13:04:30Z"})
    );
}

#[test]
fn admission_ask_deadline_ends_it_unknown() {
    for discovery_pending in [false, true] {
        let mut s = GuardScenario::new(guarded());
        let (a, k) = waiting(&s.admission_call("2026-11-21T13:01:00Z", "call-a.turn.v1"));
        if !discovery_pending {
            s.discover_ask(&k, "2026-11-21T13:01:00Z");
        }
        let input = json!({"kind":"admission-ask-deadline","invocation":k,"attempt":1});
        let t = s.admission_input("2026-11-21T13:01:30Z", input.clone());
        assert_eq!(dispatched_ids(&t), vec![json!(k)]);
        assert_admission_hold(
            &s,
            &t,
            hold_reply(&t, &a),
            no_report_fields(json!(k), "deadline", "2026-11-21T13:02:30Z"),
        );
        assert_eq!(t.external_intents[0]["kind"], "cancel");
        let ended = t.state.invocations.iter().find(|v| v["id"] == k).unwrap();
        let diagnostic = ended["cause"].to_string();
        assert!(
            diagnostic.contains("The login's admission ask"),
            "{diagnostic}"
        );
        assert!(
            diagnostic.contains("DL21 deadline (30 seconds after it started) and ended unknown"),
            "{diagnostic}"
        );
        assert!(
            t.decisions
                .iter()
                .any(|d| d.action == "admission-ask-deadline" && d.facts["ended"] == true)
        );
        assert_eq!(
            s.login_status("2026-11-21T13:01:30Z")["noReport"],
            json!({"invocation":k,"cause":"deadline","until":"2026-11-21T13:02:30Z"})
        );
        if discovery_pending {
            s.discover_ask(&k, "2026-11-21T13:01:31Z");
        } else {
            s.finish(
                &k,
                "2026-11-21T13:01:31Z",
                result("succeeded", json!({"admission":report_one()})),
            );
        }
        assert!(s.state.admission_reports.is_empty());
        let before = s.state.clone();
        let t = s.admission_input("2026-11-21T13:01:32Z", input);
        assert_eq!(t.reply, json!({"askEnded":false}));
        assert_eq!(
            serde_json::to_value(&t.state).unwrap(),
            serde_json::to_value(&before).unwrap()
        );
        assert!(
            t.decisions
                .iter()
                .any(|d| d.action == "admission-ask-deadline" && d.facts["ended"] == false)
        );
    }
}

#[test]
fn admission_waiter_cannot_authorize_onward_request() {
    let time = "2026-11-21T13:01:00Z";
    let mut p = guarded();
    document_mut(&mut p, "call-b")["consumer"] = json!({"kind":"Agent","name":"a"});
    let mut s = GuardScenario::new(p);
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    let request = json!({"identity":{"kind":"instance","id":subject(&s.state,"Agent","a")["id"]},
        "serving":a,"tool":"call-b.status.v1","arguments":{}});
    let before = s.state.clone();
    let t = s.admission_input(time, json!({"kind":"invoke","request":request}));
    assert_eq!(
        t.reply["structuredContent"]["diagnostics"][0]["ruleId"],
        "IFC-028"
    );
    assert_eq!(
        t.reply["_meta"]["io.github.shrav-k.conduit/reason"],
        "refused"
    );
    assert!(t.external_intents.is_empty());
    assert_eq!(t.state.admission_asks, before.admission_asks);
    assert_eq!(t.state.invocations, before.invocations);
    // The same onward status operation is permitted once the parent is dispatched.
    s.discover_ask(&k, time);
    s.finish(
        &k,
        time,
        result("succeeded", json!({"admission":report_one()})),
    );
    let t = s.admission_input(time, json!({"kind":"invoke","request":request}));
    assert_eq!(t.external_intents.len(), 1);
    assert!(t.reply["hook"].is_null());
}

#[test]
fn admission_wait_decisions_name_each_call_and_fresh_ask() {
    let mut s = GuardScenario::new(control_project());
    let time = "2026-11-21T13:01:00Z";
    s.call(time, "access.open.v1", json!({"conduit":"tune-login"}));
    let login = subject(&s.state, "Resource", "login")["id"].clone();
    let mut calls = vec![];
    let mut ask = String::new();
    for tool in ["call-a.turn.v1", "call-b.turn.v1"] {
        let t = s.admission_call(time, tool);
        let (call, k) = waiting(&t);
        assert!(t.decisions.iter().any(|d| d.action == "admission-wait"
            && d.facts == json!({"invocation":call,"ask":k,"login":login})
            && d.promises.iter().any(|p| p == "PND-016")));
        calls.push(call);
        ask = k;
    }
    s.discover_ask(&ask, time);
    let revision = subject(&s.state, "Resource", "login")["revision"].clone();
    let control = s.dispatch(time, "tune-login.set-admission.v1",
        json!({"revision":revision,"levers":{"billing":"included","paidUsage":"allow","whenUnknown":"admit"}}));
    s.finish(&control, time, result("succeeded", json!({})));
    let t = s.finish(
        &ask,
        time,
        result("succeeded", json!({"admission":report_one()})),
    );
    let fresh = &t.state.admission_asks[login.as_str().unwrap()].invocation;
    assert_ne!(fresh, &ask);
    for call in calls {
        assert!(t.decisions.iter().any(|d| d.action == "admission-wait"
            && d.facts == json!({"invocation":call,"ask":fresh,"login":login})));
    }
}

#[test]
fn admission_dispatch_diagnostic_distinguishes_failed_and_unreadable_conditions() {
    for (field, value, words) in [
        ("open", json!(false), "was closed"),
        ("open", Value::Null, "open state could not be checked"),
        ("operations", json!([]), "no longer listed the operation"),
        ("operations", Value::Null, "operations could not be checked"),
        ("consumer-state", json!("retired"), "had a retired consumer"),
        (
            "consumer-state",
            Value::Null,
            "consumer's state could not be checked",
        ),
    ] {
        let time = "2026-11-21T13:01:00Z";
        let mut s = GuardScenario::new(guarded());
        let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
        s.discover_ask(&k, time);
        let conduit = subject(&s.state, "Conduit", "call-a")["id"].clone();
        let consumer = access::object_by_id(
            &s.state,
            &subject(&s.state, "Conduit", "call-a")["consumer"],
        )
        .unwrap()["id"]
            .clone();
        let (id, field) = if field == "consumer-state" {
            (consumer, "state")
        } else {
            (conduit, field)
        };
        s.state.objects.iter_mut().find(|o| o["id"] == id).unwrap()[field] = value;
        let t = s.finish(
            &k,
            time,
            result("succeeded", json!({"admission":report_one()})),
        );
        assert!(t.external_intents.is_empty());
        let reply = &t
            .request_replies
            .iter()
            .find(|r| r.invocation == a)
            .unwrap()
            .reply;
        let text = reply["structuredContent"]["diagnostics"][0]["message"]["text"]
            .as_str()
            .unwrap();
        assert!(text.contains(words), "{text}");
        assert!(
            text.contains("while the call waited for its login's admission report"),
            "{text}"
        );
        assert!(!text.contains(" or "), "{text}");
    }
}

fn refusing_report() -> Value {
    let case: Value = serde_json::from_str(include_str!(
        "../../../../conformance/conventions/requests/admission-hold/case.yaml"
    ))
    .unwrap();
    case["steps"][17]["input"]["completion"]["result"]["structuredContent"]["admission"].clone()
}

fn single_use_report() -> Value {
    let case: Value = serde_json::from_str(include_str!(
        "../../../../conformance/conventions/requests/admission-single-use/case.yaml"
    ))
    .unwrap();
    case["steps"][18]["input"]["completion"]["result"]["structuredContent"]["admission"].clone()
}

#[track_caller]
fn hold_reply<'a>(t: &'a Transition, id: &str) -> &'a Value {
    &t.request_replies
        .iter()
        .find(|r| r.invocation == id)
        .expect("held reply")
        .reply
}

fn assert_admission_hold(
    s: &GuardScenario,
    t: &Transition,
    reply: &Value,
    fields: Value,
) -> String {
    assert_eq!(reply["isError"], true);
    assert_eq!(
        reply["_meta"]["io.github.shrav-k.conduit/reason"],
        "unavailable"
    );
    assert!(
        reply["_meta"]
            .get("io.github.shrav-k.conduit/pendingAction")
            .is_none()
    );
    let id = reply["_meta"]["io.github.shrav-k.conduit/invocation"]
        .as_str()
        .unwrap();
    let diagnostics = reply["structuredContent"]["diagnostics"]
        .as_array()
        .unwrap();
    assert_eq!(diagnostics.len(), 1);
    let d = &diagnostics[0];
    assert_eq!(d["ruleId"], "PND-016");
    assert_eq!(d["level"], "error");
    let mut expected = json!({"login":inventory::object(subject(&s.state,"Resource","login")),
        "dispatched":false,"levers":{"type":"example.org/conformance/shared-login#login-holder.v1",
        "pointer":"/admission","control":"example.org/conformance/shared-login#set-admission.v1"}});
    expected
        .as_object_mut()
        .unwrap()
        .extend(fields.as_object().unwrap().clone());
    assert_eq!(d["properties"]["admission"], expected);
    assert_eq!(
        s.c.core_environment
            .first_core_failure(&s.c.core_package, "admission-hold.v1", &expected)
            .unwrap(),
        None
    );
    let record = t
        .records
        .iter()
        .find(|r| r["type"] == "request-ended" && r["data"]["invocation"] == id)
        .unwrap();
    assert_eq!(record["data"]["ruleId"], "PND-016");
    assert_eq!(record["data"]["reason"], "unavailable");
    assert_eq!(record["data"]["dispatched"], false);
    assert!(
        !t.records
            .iter()
            .any(|r| r["type"] == "invocation-dispatched" && r["data"]["invocation"] == id)
    );
    assert!(!t.external_intents.iter().any(|v| v["invocation"] == id));
    assert!(
        !t.state
            .invocations
            .iter()
            .any(|v| v["id"] == id && v["state"] == "in-progress")
    );
    let decisions: Vec<_> = t
        .decisions
        .iter()
        .filter(|d| d.action == "hold-login-admission" && d.facts["invocation"] == id)
        .collect();
    assert_eq!(decisions.len(), 1);
    assert_eq!(decisions[0].facts["instance"], expected["login"]["id"]);
    let text = d["message"]["text"].as_str().unwrap();
    for word in ["amount", "balance", "credit", "currency"] {
        assert!(!text.contains(word));
    }
    assert!(text.contains("reports unavailable under PND-016: "));
    assert!(text.contains("Once dispatched, a call runs to its end; the guard never stops it."));
    if expected["report"].is_null() {
        assert!(!text.contains("also ends the report"));
        if expected["noReport"].is_string() {
            assert!(text.contains("A lever change does not shorten the 60 seconds."));
        }
    } else {
        assert!(text.contains(": its admission report says"));
        assert!(text.contains("window resets at") && text.contains("paid usage resets at"));
        if expected["lifts"] == "newer-report" {
            assert!(text.contains("is when its validity ends"));
            assert!(text.contains("held at once until then and no ask is made"));
            assert!(text.contains("the first call at or after retryAt asks the login afresh"));
        }
    }
    assert!(
        text.contains("not dispatched")
            && text.contains("queued")
            && text.contains("decided afresh")
    );
    text.into()
}

#[test]
fn admission_held_on_a_refusing_report() {
    let time = "2026-11-21T13:01:00Z";
    let mut s = GuardScenario::new(guarded());
    let r2 = refusing_report();
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    s.discover_ask(&k, time);
    let t = s.finish(&k, time, result("succeeded", json!({"admission":r2})));
    assert!(t.external_intents.is_empty());
    let fields = json!({"report":r2,"reportInvocation":k,"noReport":null,"singleUse":null,"lifts":"newer-report","retryAt":"2026-11-21T13:11:00Z"});
    let text = assert_admission_hold(&s, &t, hold_reply(&t, &a), fields.clone());
    assert!(!t.records.iter().any(|r| r["type"] == "access-decision"));
    for word in [
        login_resource::label(subject(&s.state, "Resource", "login")).as_str(),
        "/admission/paidUsage",
        "refuse",
        "not reported",
        "2026-11-21T13:11:00Z",
        &k,
        "Interface",
    ] {
        assert!(text.contains(word), "missing {word}");
    }
    let t = s.admission_call("2026-11-21T13:02:00Z", "call-b.turn.v1");
    assert_admission_hold(&s, &t, &t.reply, fields);
    assert!(t.external_intents.is_empty() && t.state.admission_asks.is_empty());
    let (_, t) = s.keep("2026-11-21T13:03:00Z", &report_one(), None);
    assert!(t.external_intents.is_empty());
    let t = s.admission_call("2026-11-21T13:03:10Z", "call-a.turn.v1");
    assert_eq!(dispatched_ids(&t), vec![t.reply["invocation"].clone()]);

    for lever_change in [true, false] {
        let mut s = GuardScenario::new(guarded());
        let mut report = r2.clone();
        if lever_change {
            report["lifts"] = json!("lever-change");
        } else {
            report = json!({"admit":false,"reason":"threshold","lever":{"pointer":"/admission/refuseAt","value":95},"window":{"id":"example/all/300m","usedPercent":97,"resetsAt":"2026-11-21T14:00:00Z"},"paidUsage":{"possible":false,"inUse":false,"resetsAt":null},"cannotTell":null,"lifts":"at-time","observedAt":"2026-11-21T13:00:30Z","source":{"client":null,"method":"example-read"},"validForSeconds":600,"singleUse":false});
        }
        let (k, _) = s.keep(time, &report, None);
        let t = s.admission_call("2026-11-21T13:01:30Z", "call-a.turn.v1");
        let text = assert_admission_hold(
            &s,
            &t,
            &t.reply,
            json!({"report":report,"reportInvocation":k,"noReport":null,"singleUse":null,"lifts":report["lifts"],"retryAt":if lever_change {Value::Null} else {json!("2026-11-21T13:11:00Z")}}),
        );
        if lever_change {
            assert!(text.contains(&format!("Waiting alone does not lift this; a change of `/admission` by the control `example.org/conformance/shared-login#set-admission.v1` on `{}`, sent by an Interface, does.",login_resource::label(subject(&s.state,"Resource","login")))));
        } else {
            for word in [
                "/admission/refuseAt",
                "95",
                "example/all/300m",
                "97",
                "2026-11-21T13:00:30Z",
                "2026-11-21T14:00:00Z",
            ] {
                assert!(text.contains(word));
            }
        }
    }
}

#[test]
fn admission_single_use_holds_the_others_after_call() {
    let time = "2026-11-21T13:01:00Z";
    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    let (b, _) = waiting(&s.admission_call(time, "call-b.turn.v1"));
    let (b2, _) = waiting(&s.admission_call(time, "call-b.turn.v1"));
    assert_eq!(
        s.call(time, "access.close.v1", json!({"conduit":"call-a"}))
            .reply["isError"],
        false
    );
    s.discover_ask(&k, time);
    let r3 = single_use_report();
    let t = s.finish(&k, time, result("succeeded", json!({"admission":r3})));
    assert_eq!(dispatched_ids(&t), vec![json!(b)]);
    assert!(
        hold_reply(&t, &a)["structuredContent"]["diagnostics"][0]["properties"]
            .get("admission")
            .is_none()
    );
    let text = assert_admission_hold(
        &s,
        &t,
        hold_reply(&t, &b2),
        json!({"report":null,"reportInvocation":k,"noReport":null,"singleUse":b,"lifts":"after-call","retryAt":"2026-11-21T13:02:00Z"}),
    );
    assert!(text.contains(&b) && text.contains("need not wait"));
    assert!(!t.records.iter().any(|r| r["type"] == "access-decision"));
    let (_, k2) = waiting(&s.admission_call("2026-11-21T13:01:10Z", "call-b.turn.v1"));
    assert_ne!(k2, k);
    let t = s.discover_ask(&k2, "2026-11-21T13:01:10Z");
    assert_eq!(
        t.external_intents[0]["call"]["params"]["arguments"]["previous"],
        r3
    );
}

#[test]
fn admission_hold_names_only_what_the_caller_may_inspect() {
    let time = "2026-11-21T13:01:00Z";
    let mut p = guarded();
    let mut path = document_mut(&mut p, "call-b").clone();
    path["metadata"]["name"] = json!("observer-turn");
    path["consumer"]["name"] = json!("observer");
    p.documents.push(path);
    for single in [true, false] {
        let mut s = GuardScenario::new(p.clone());
        assert_eq!(
            s.call(time, "access.open.v1", json!({"conduit":"observer-turn"}))
                .reply["isError"],
            false
        );
        let observer = subject(&s.state, "Interface", "observer")["id"].clone();
        for (kind, name) in [("Resource", "login"), ("Agent", "a")] {
            assert!(
                !access::may_inspect(&s.state, &observer, subject(&s.state, kind, name), &s.c)
                    .unwrap()
            );
        }
        let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
        let t = s.admission_input(time,json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":observer},"tool":"observer-turn.turn.v1","arguments":{}}}));
        let (b, _) = waiting(&t);
        s.discover_ask(&k, time);
        let report = if single {
            single_use_report()
        } else {
            refusing_report()
        };
        let t = s.finish(&k, time, result("succeeded", json!({"admission":report})));
        assert_eq!(
            dispatched_ids(&t),
            if single { vec![json!(a)] } else { vec![] }
        );
        let text = assert_admission_hold(
            &s,
            &t,
            hold_reply(&t, &b),
            json!({"report":if single {Value::Null} else {report},"reportInvocation":null,"noReport":null,"singleUse":null,"lifts":if single {"after-call"} else {"newer-report"},"retryAt":if single {"2026-11-21T13:02:00Z"} else {"2026-11-21T13:11:00Z"}}),
        );
        assert!(!text.contains(&k) && !text.contains(&a));
        if single {
            assert!(text.contains("a call of an Agent of the login"));
        }
    }
}

#[test]
fn admission_hold_never_stops_a_dispatched_call() {
    let time = "2026-11-21T13:01:00Z";
    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    let (b, _) = waiting(&s.admission_call(time, "call-b.turn.v1"));
    s.discover_ask(&k, time);
    let t = s.finish(
        &k,
        time,
        result("succeeded", json!({"admission":report_one()})),
    );
    assert_eq!(dispatched_ids(&t), vec![json!(a), json!(b)]);
    let mut output = result("succeeded", json!({}));
    output["_meta"]["io.github.shrav-k.conduit/limits"] = limits_member();
    let t = s.finish(&b, "2026-11-21T13:02:00Z", output);
    assert!(!dispatched_ids(&t).contains(&json!(a)));
    assert_eq!(
        s.login_status("2026-11-21T13:02:00Z")["ended"],
        "newer-limits"
    );
    let t = s.admission_call("2026-11-21T13:03:00Z", "call-b.turn.v1");
    assert!(!dispatched_ids(&t).contains(&json!(a)));
    let (b2, k2) = waiting(&t);
    let t = s.discover_ask(&k2, "2026-11-21T13:03:00Z");
    assert!(!dispatched_ids(&t).contains(&json!(a)));
    let t = s.finish(
        &k2,
        "2026-11-21T13:03:00Z",
        result("succeeded", json!({"admission":refusing_report()})),
    );
    assert!(t.external_intents.is_empty());
    assert_admission_hold(
        &s,
        &t,
        hold_reply(&t, &b2),
        json!({"report":refusing_report(),"reportInvocation":k2,"noReport":null,"singleUse":null,"lifts":"newer-report","retryAt":"2026-11-21T13:13:00Z"}),
    );
    assert!(
        s.state
            .invocations
            .iter()
            .any(|v| v["id"] == a && v["state"] == "in-progress")
    );
    let t = s.finish(&a, "2026-11-21T13:04:00Z", result("succeeded", json!({})));
    assert_eq!(
        t.reply["_meta"]["io.github.shrav-k.conduit/outcome"],
        "succeeded"
    );
    assert!(
        t.reply["_meta"]
            .get("io.github.shrav-k.conduit/admission")
            .is_none()
    );
    assert!(
        !t.reply["structuredContent"]["diagnostics"]
            .as_array()
            .is_some_and(|ds| ds.iter().any(|d| d["ruleId"] == "PND-016"))
    );
    assert!(!dispatched_ids(&t).contains(&json!(a)));
}

fn no_report_fields(ask: Value, cause: &str, until: &str) -> Value {
    json!({"report":null,"reportInvocation":ask,"noReport":cause,"singleUse":null,
        "lifts":"at-time","retryAt":until})
}

#[test]
fn admission_held_for_want_of_a_report() {
    let time = "2026-11-21T13:01:00Z";
    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    s.discover_ask(&k, time);
    let t = s.finish(&k, time, result("failed", json!({})));
    assert!(t.external_intents.is_empty());
    let fields = no_report_fields(json!(k), "failed", "2026-11-21T13:02:00Z");
    let text = assert_admission_hold(&s, &t, hold_reply(&t, &a), fields.clone());
    assert!(text.contains(&k) && text.contains("failed") && text.contains("2026-11-21T13:02:00Z"));
    assert!(t.records.iter().all(|r| r["type"] != "access-decision"));
    let t = s.admission_call("2026-11-21T13:01:30Z", "call-b.turn.v1");
    assert_admission_hold(&s, &t, &t.reply, fields.clone());
    assert!(t.external_intents.is_empty() && t.state.admission_asks.is_empty());
    let (_, t) = s.keep("2026-11-21T13:01:40Z", &report_one(), None);
    assert!(t.external_intents.is_empty());
    let t = s.admission_call("2026-11-21T13:01:45Z", "call-a.turn.v1");
    assert_admission_hold(&s, &t, &t.reply, fields);
    assert!(t.external_intents.is_empty() && t.state.admission_asks.is_empty());
    let t = s.admission_call("2026-11-21T13:02:00Z", "call-a.turn.v1");
    assert_eq!(dispatched_ids(&t), vec![t.reply["invocation"].clone()]);
    assert!(t.state.admission_asks.is_empty());
    assert_eq!(
        s.login_status("2026-11-21T13:02:00Z")["validUntil"],
        "2026-11-21T14:01:40Z"
    );

    let mut s = GuardScenario::new(guarded());
    let (_, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    s.discover_ask(&k, time);
    s.finish(&k, time, result("failed", json!({})));
    let (a, k2) = waiting(&s.admission_call("2026-11-21T13:02:00Z", "call-a.turn.v1"));
    assert_ne!(k2, k);
    s.discover_ask(&k2, "2026-11-21T13:02:00Z");
    let mut bad = report_one();
    bad.as_object_mut().unwrap().remove("singleUse");
    let t = s.finish(
        &k2,
        "2026-11-21T13:02:00Z",
        result("succeeded", json!({"admission":bad})),
    );
    let text = assert_admission_hold(
        &s,
        &t,
        hold_reply(&t, &a),
        no_report_fields(json!(k2), "unknown", "2026-11-21T13:03:00Z"),
    );
    assert!(t.external_intents.is_empty());
    assert!(
        text.contains(&k2) && text.contains("unknown") && text.contains("2026-11-21T13:03:00Z")
    );

    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    s.discover_ask(&k, time);
    let t = s.admission_input(
        "2026-11-21T13:01:30Z",
        json!({"kind":"admission-ask-deadline","invocation":k,"attempt":1}),
    );
    assert_eq!(
        t.external_intents,
        vec![json!({"kind":"cancel","invocation":k,"attempt":1})]
    );
    let text = assert_admission_hold(
        &s,
        &t,
        hold_reply(&t, &a),
        no_report_fields(json!(k), "deadline", "2026-11-21T13:02:30Z"),
    );
    assert!(text.contains(&k) && text.contains("DL21") && text.contains("2026-11-21T13:02:30Z"));

    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    let t = s.finish(&k, time, json!({"capabilities":{}}));
    assert!(t.external_intents.is_empty());
    let text = assert_admission_hold(
        &s,
        &t,
        hold_reply(&t, &a),
        no_report_fields(Value::Null, "not-dispatched", "2026-11-21T13:02:00Z"),
    );
    assert!(text.contains("could not be dispatched") && text.contains("2026-11-21T13:02:00Z"));
    assert!(!text.contains(&k));

    let mut s = GuardScenario::new(guarded());
    let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
    s.discover_ask(&k, time);
    let t = s.admission_input("2026-11-21T13:01:30Z", json!({"kind":"recover"}));
    assert!(t.external_intents.is_empty());
    // Recovery records the ending; the previous process's caller is gone.
    let end = t
        .records
        .iter()
        .find(|r| r["type"] == "request-ended" && r["data"]["invocation"] == a)
        .unwrap();
    assert_eq!(end["data"]["ruleId"], "REC-007");
    assert_eq!(end["data"]["reason"], "unavailable");
    assert_eq!(end["data"]["dispatched"], false);
    assert!(end["data"].get("admission").is_none());
    assert!(!t.state.invocations.iter().any(|v| v["id"] == a));
    let t = s.admission_call("2026-11-21T13:02:00Z", "call-b.turn.v1");
    let text = assert_admission_hold(
        &s,
        &t,
        &t.reply,
        no_report_fields(json!(k), "unknown", "2026-11-21T13:02:30Z"),
    );
    assert!(t.external_intents.is_empty());
    assert!(text.contains(&k) && text.contains("unknown") && text.contains("2026-11-21T13:02:30Z"));

    // REC-007 precedes the guard even when the proposed completion would hold
    // the call, its Conduit closes, or its login acquires a PND-005 hold.
    for condition in ["open", "closed", "login-held"] {
        let mut s = GuardScenario::new(guarded());
        let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
        s.discover_ask(&k, time);
        if condition == "closed" {
            s.call(time, "access.close.v1", json!({"conduit":"call-a"}));
        } else if condition == "login-held" {
            let login = subject(&s.state, "Resource", "login")["id"]
                .as_str()
                .unwrap()
                .to_owned();
            s.state.sign_in_holds.insert(login, json!({"since":time}));
        }
        let e = s.evidence(time);
        let input: Input = serde_json::from_value(json!({"kind":"complete","invocation":k,"attempt":1,
            "evidence":{"time":e.time,"ids":e.ids,"revisions":e.revisions,"suffixes":e.suffixes},
            "completion":{"kind":"result","result":result("succeeded",json!({"admission":refusing_report()}))}})).unwrap();
        let trial = step(&s.state, input.clone(), &s.c).unwrap();
        let fault = RecordFault::from_trial(input, &trial, false).unwrap();
        let t = fault.step(s.state, &s.c).unwrap();
        assert!(t.external_intents.is_empty());
        let reply = hold_reply(&t, &a);
        assert_eq!(
            reply["structuredContent"]["diagnostics"][0]["ruleId"],
            "REC-007"
        );
        assert!(
            reply["structuredContent"]["diagnostics"][0]["properties"]
                .get("admission")
                .is_none()
        );
        assert!(
            !t.decisions
                .iter()
                .any(|d| d.action == "hold-login-admission")
        );
        assert!(t.records.iter().all(|r| r["type"] != "access-decision"));
    }
}

#[test]
fn admission_status_shows_what_holds() {
    let time = "2026-11-21T13:01:00Z";
    for refused in [true, false] {
        let mut s = GuardScenario::new(guarded());
        let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
        s.discover_ask(&k, time);
        let t = s.finish(
            &k,
            time,
            if refused {
                result("succeeded", json!({"admission":refusing_report()}))
            } else {
                result("failed", json!({}))
            },
        );
        assert!(t.external_intents.is_empty());
        let fields = if refused {
            json!({"report":refusing_report(),"reportInvocation":k,"noReport":null,"singleUse":null,"lifts":"newer-report","retryAt":"2026-11-21T13:11:00Z"})
        } else {
            no_report_fields(json!(k), "failed", "2026-11-21T13:02:00Z")
        };
        assert_admission_hold(&s, &t, hold_reply(&t, &a), fields.clone());
        assert_eq!(
            s.call(time, "access.open.v1", json!({"conduit":"observer-scope"}))
                .reply["isError"],
            false
        );
        let expected = if refused {
            json!({"checked":true,"report":fields["report"],"invocation":k,"keptAt":time,"validUntil":fields["retryAt"],"state":"current","ended":null,"asking":null,"noReport":null})
        } else {
            json!({"checked":true,"report":null,"invocation":null,"keptAt":null,"validUntil":null,"state":"none","ended":null,"asking":null,"noReport":{"invocation":k,"cause":fields["noReport"],"until":fields["retryAt"]}})
        };
        for (actor, route, kind, name) in [
            ("root", "logins", "Resource", "login"),
            ("root", "workers", "Agent", "a"),
            ("observer", "observer-scope", "Agent", "a"),
        ] {
            let e = s.evidence(time);
            let before = s.state.clone();
            let input: Input = serde_json::from_value(json!({"kind":"invoke",
                "evidence":{"time":e.time,"ids":e.ids,"revisions":e.revisions,"suffixes":e.suffixes},
                "request":{"identity":{"kind":"instance","id":subject(&s.state,"Interface",actor)["id"]},
                "tool":format!("{route}.inspect.v1"),"arguments":{"object":{"kind":kind,"name":name}}}})).unwrap();
            let t = step(&s.state, input, &s.c).unwrap();
            assert_eq!(t.reply["isError"], false);
            let mut expected = expected.clone();
            if actor == "observer" {
                assert!(
                    !access::may_inspect(
                        &before,
                        &subject(&before, "Interface", actor)["id"],
                        subject(&before, "Resource", "login"),
                        &s.c
                    )
                    .unwrap()
                );
                expected["invocation"] = Value::Null;
                if !expected["noReport"].is_null() {
                    expected["noReport"]["invocation"] = Value::Null;
                }
            }
            let status = &t.reply["structuredContent"]["status"]["admission"];
            assert_eq!(*status, expected);
            assert_eq!(
                s.c.core_environment
                    .first_core_failure(&s.c.core_package, "admission-status.v1", status)
                    .unwrap(),
                None
            );
            assert!(t.external_intents.is_empty());
            assert!(
                !t.decisions
                    .iter()
                    .any(|d| d.action == "hold-login-admission")
            );
            assert_eq!(t.state.admission_reports, before.admission_reports);
            assert_eq!(t.state.admission_asks, before.admission_asks);
            assert_eq!(t.state.admission_no_reports, before.admission_no_reports);
            // OBS-006 protects the observed state. REC-001 and OPS-009 still
            // record this inspection and retain its own successful result.
            assert_eq!(t.state.invocations.len(), before.invocations.len() + 1);
            assert!(
                t.state.invocations[..before.invocations.len()] == before.invocations,
                "inspection changed a pre-existing invocation"
            );
            let inspection = t.state.invocations.last().unwrap();
            assert_eq!(
                inspection["id"],
                t.reply["_meta"]["io.github.shrav-k.conduit/invocation"]
            );
            assert_eq!(inspection["operation"], s.c.qualified("inspect.v1"));
            assert_eq!(inspection["state"], "succeeded");
            for (kind, name) in [("Resource", "login"), ("Agent", "a")] {
                assert_eq!(subject(&t.state, kind, name), subject(&before, kind, name));
            }
            assert_eq!(
                t.state
                    .objects
                    .iter()
                    .filter(|o| o["kind"] == "Conduit")
                    .collect::<Vec<_>>(),
                before
                    .objects
                    .iter()
                    .filter(|o| o["kind"] == "Conduit")
                    .collect::<Vec<_>>()
            );
            s.state = t.state;
        }
    }
}

#[test]
fn admission_hold_restores_keyed_attempt_without_wait_fields() {
    for waits in [false, true] {
        let mut project = guarded();
        // An extension-aware implementation can report a definite failed turn,
        // allowing another attempt with the same key.
        project
            .documents
            .iter_mut()
            .find(|d| d["metadata"]["name"] == "shared-agent.v1")
            .unwrap()["spec"]["implementation"] =
            json!({"#turn.v1":"#agent","#status.v1":"#agent"});
        let mut s = GuardScenario::new(project);
        s.keep("2026-11-21T13:00:00Z", &report_one(), None);
        let t = s.keyed_admission_call("2026-11-21T13:01:00Z");
        let id = t.reply["invocation"].as_str().unwrap().to_owned();
        assert_eq!(dispatched_ids(&t), vec![json!(id)]);
        s.discover_ask(&id, "2026-11-21T13:01:00Z");
        s.finish(&id, "2026-11-21T13:01:01Z", result("failed", json!({})));
        let previous = s
            .state
            .invocations
            .iter()
            .find(|v| v["id"] == id)
            .unwrap()
            .clone();
        assert_eq!(previous["state"], "failed");
        let time = "2026-11-21T14:01:00Z";
        if !waits {
            s.keep(time, &refusing_report(), None);
        }
        let mut t = s.keyed_admission_call(time);
        let reply = if waits {
            let (call, ask) = waiting(&t);
            assert_eq!(call, id);
            s.discover_ask(&ask, time);
            t = s.finish(
                &ask,
                time,
                result("succeeded", json!({"admission":refusing_report()})),
            );
            hold_reply(&t, &id)
        } else {
            &t.reply
        };
        assert_eq!(
            reply["structuredContent"]["diagnostics"][0]["ruleId"],
            "PND-016"
        );
        assert!(t.external_intents.is_empty());
        let restored = t.state.invocations.iter().find(|v| v["id"] == id).unwrap();
        assert_eq!(restored["state"], previous["state"]);
        assert_eq!(restored["retained"], previous["retained"]);
        for field in ["admission_tool", "admission_wait", "previous"] {
            assert!(
                restored.get(field).is_none(),
                "restored keyed attempt retains {field}; waits={waits}"
            );
        }
        s.keep("2026-11-21T14:02:00Z", &report_one(), None);
        let t = s.keyed_admission_call("2026-11-21T14:02:01Z");
        assert_eq!(dispatched_ids(&t), vec![json!(id)]);
    }
}

#[test]
fn admission_hold_explains_cannot_tell_in_schema_words() {
    for (cause, words) in [
        ("never-observed", "`limits` is `unknown`"),
        ("not-reported", "the client gave no signal needed to decide"),
        ("stale", "the needed observation is no longer current"),
        ("read-failed", "the client's read failed"),
        ("read-timed-out", "the client's read timed out"),
    ] {
        let mut s = GuardScenario::new(guarded());
        let mut report = refusing_report();
        report["reason"] = json!("cannot-tell");
        report["cannotTell"] = json!(cause);
        report["lever"] = json!({"pointer":"/admission/whenUnknown","value":"refuse"});
        let (ask, _) = s.keep("2026-11-21T13:01:00Z", &report, None);
        let t = s.admission_call("2026-11-21T13:01:01Z", "call-a.turn.v1");
        let text = assert_admission_hold(
            &s,
            &t,
            &t.reply,
            json!({"report":report,"reportInvocation":ask,"noReport":null,"singleUse":null,
                "lifts":"newer-report","retryAt":"2026-11-21T13:11:00Z"}),
        );
        assert!(
            text.contains(&format!("It cannot tell: {words}.")),
            "{cause}: {text}"
        );
    }
}

// Order 321a: a warning belongs to the dispatched attempt, never its wire request.
const ADMISSION_KEY: &str = "io.github.shrav-k.conduit/admission";

fn warning_report() -> Value {
    let case: Value = serde_json::from_str(include_str!(
        "../../../../conformance/conventions/requests/admission-warning/case.yaml"
    ))
    .unwrap();
    case["steps"][21]["input"]["completion"]["result"]["structuredContent"]["admission"].clone()
}

fn discovered_guarded() -> packages::Project {
    let mut p = guarded();
    document_mut(&mut p, "shared-agent.v1")["spec"]["implementation"] =
        json!({"#turn.v1":"#agent","#status.v1":"#agent"});
    p
}

fn observer_guarded() -> packages::Project {
    let mut p = guarded();
    let mut path = document_mut(&mut p, "call-b").clone();
    path["metadata"]["name"] = json!("observer-turn");
    path["consumer"]["name"] = json!("observer");
    p.documents.push(path);
    p
}

#[track_caller]
fn dispatch_warning<'a>(t: &'a Transition, id: &str) -> &'a Value {
    &t.records
        .iter()
        .find(|r| r["type"] == "invocation-dispatched" && r["data"]["invocation"] == id)
        .expect("dispatch record")["data"]["admissionWarning"]
}

#[track_caller]
fn assert_warning(s: &GuardScenario, warning: &Value, report: &Value, ask: Value) {
    assert_eq!(warning["level"], "warning");
    assert_eq!(warning["ruleId"], "PND-016");
    let form = json!({"login":inventory::object(subject(&s.state,"Resource","login")),
        "dispatched":true,"report":report,"reportInvocation":ask,
        "noReport":null,"singleUse":null,"lifts":null,"retryAt":null,
        "levers":{"type":"example.org/conformance/shared-login#login-holder.v1",
        "pointer":"/admission","control":"example.org/conformance/shared-login#set-admission.v1"}});
    assert_eq!(warning["properties"]["admission"], form);
    for (schema, value) in [("admission-hold.v1", &form), ("diagnostic.v1", warning)] {
        assert_eq!(
            s.c.core_environment
                .first_core_failure(&s.c.core_package, schema, value)
                .unwrap(),
            None
        );
    }
    let text = warning["message"]["text"].as_str().unwrap();
    assert!(text.contains(" was dispatched under PND-016 on a report that warns: the login "));
    assert_eq!(text.matches("was dispatched").count(), 1);
    assert!(text.contains("paid usage resets at "));
    if report["reason"] == "warning" {
        assert!(text.contains("used percentage 85 against this warning threshold"));
    }
    assert!(text.contains(&login_resource::label(&form["login"])));
    assert!(text.contains("The call started model work") && text.contains("runs to its end"));
    for word in ["amount", "balance", "credit", "currency", "unavailable"] {
        assert!(!text.contains(word));
    }
}

fn normalize_invocation(value: &Value, id: &str) -> Value {
    serde_json::from_str(&value.to_string().replace(id, "INVOCATION")).unwrap()
}

#[track_caller]
fn assert_warning_end(t: &Transition, id: &str, warning: &Value) {
    assert_eq!(&hold_reply(t, id)["_meta"][ADMISSION_KEY], warning);
    assert!(
        t.records
            .iter()
            .all(|r| r["data"].get("admissionWarning").is_none())
    );
}

#[test]
fn admission_warning_on_a_report_that_warns() {
    let mut s = GuardScenario::new(guarded());
    let r4 = warning_report();
    let (k, _) = s.keep("2026-11-21T13:02:00Z", &r4, None);
    let t = s.admission_call("2026-11-21T13:02:10Z", "call-a.turn.v1");
    let id = t.reply["invocation"].as_str().unwrap().to_owned();
    let w = dispatch_warning(&t, &id).clone();
    assert_warning(&s, &w, &r4, json!(k));
    assert!(
        !json!(t.external_intents)
            .to_string()
            .contains(ADMISSION_KEY)
    );
    assert!(t.reply["_meta"].get(ADMISSION_KEY).is_none());
    let text = w["message"]["text"].as_str().unwrap();
    for word in [
        "call-a.turn.v1",
        "/admission/warnAt",
        "80",
        "example/all/300m",
        "85",
        "2026-11-21T14:00:00Z",
        "not reported",
        &k,
        "may be held",
    ] {
        assert!(text.contains(word), "missing {word}");
    }
    let output = json!({"isError":false,"content":[{"type":"text","text":"plain answer"}]});
    let t = s.finish(&id, "2026-11-21T13:03:00Z", output.clone());
    assert_warning_end(&t, &id, &w);
    assert_eq!(t.reply, *hold_reply(&t, &id));
    let mut warned = t.reply.clone();
    warned["_meta"]
        .as_object_mut()
        .unwrap()
        .remove(ADMISSION_KEY);
    let mut un = GuardScenario::new(guarded());
    un.keep("2026-11-21T13:02:00Z", &report_one(), None);
    let un_id = un.dispatch("2026-11-21T13:02:10Z", "call-a.turn.v1", json!({}));
    let plain = un.finish(&un_id, "2026-11-21T13:03:00Z", output);
    assert_eq!(
        normalize_invocation(&warned, &id),
        normalize_invocation(&plain.reply, &un_id)
    );
    // OPS-007: a plain error cannot confirm that model work did not happen.
    let id2 = s.dispatch("2026-11-21T13:03:10Z", "call-a.turn.v1", json!({}));
    let t = s.finish(
        &id2,
        "2026-11-21T13:03:30Z",
        json!({"isError":true,"content":[{"type":"text","text":"plain error"}]}),
    );
    assert_eq!(t.reply["isError"], true);
    assert_warning_end(&t, &id2, &w);
    let mut un = GuardScenario::new(guarded());
    un.keep("2026-11-21T13:02:00Z", &report_one(), None);
    let un_id = un.dispatch("2026-11-21T13:03:10Z", "call-a.turn.v1", json!({}));
    let plain = un.finish(
        &un_id,
        "2026-11-21T13:03:30Z",
        json!({"isError":true,"content":[{"type":"text","text":"plain error"}]}),
    );
    assert_eq!(t.reply["content"], plain.reply["content"]);
    let mut s = GuardScenario::new(discovered_guarded());
    s.keep("2026-11-21T13:02:00Z", &r4, None);
    let id = s.dispatch("2026-11-21T13:03:10Z", "call-a.turn.v1", json!({}));
    let t = s.finish(&id, "2026-11-21T13:03:30Z", result("failed", json!({})));
    assert_eq!(
        t.reply["_meta"]["io.github.shrav-k.conduit/outcome"],
        "failed"
    );
    assert_eq!(t.reply["_meta"][ADMISSION_KEY]["level"], "warning");
}

#[test]
fn admission_warning_on_a_single_use_report() {
    for discover in [false, true] {
        let mut s = GuardScenario::new(if discover {
            discovered_guarded()
        } else {
            guarded()
        });
        let time = "2026-11-21T13:01:00Z";
        let (a, k) = waiting(&s.admission_call(time, "call-a.turn.v1"));
        let (b, _) = waiting(&s.admission_call(time, "call-b.turn.v1"));
        s.discover_ask(&k, time);
        let r3 = single_use_report();
        let t = s.finish(&k, time, result("succeeded", json!({"admission":r3})));
        assert_eq!(
            hold_reply(&t, &b)["structuredContent"]["diagnostics"][0]["properties"]["admission"]["lifts"],
            "after-call"
        );
        assert!(hold_reply(&t, &b)["_meta"].get(ADMISSION_KEY).is_none());
        assert!(
            t.records
                .iter()
                .filter(|r| r["data"]["invocation"] == k)
                .all(|r| r["data"].get("admissionWarning").is_none())
        );
        let t = if discover {
            assert!(
                !t.records
                    .iter()
                    .any(|r| r["type"] == "invocation-dispatched" && r["data"]["invocation"] == a)
            );
            s.discover_ask(&a, "2026-11-21T13:01:10Z")
        } else {
            t
        };
        let w = dispatch_warning(&t, &a).clone();
        assert_warning(&s, &w, &r3, json!(k));
        let text = w["message"]["text"].as_str().unwrap();
        for word in [
            "never-observed",
            "`limits` is `unknown`",
            "/admission/whenUnknown",
            "one-trial",
            "admitted this call alone",
            "is spent",
            "new ask",
        ] {
            assert!(text.contains(word), "missing {word}");
        }
        let t = s.finish(&a, "2026-11-21T13:01:20Z", result("succeeded", json!({})));
        assert_warning_end(&t, &a, &w);
    }
}

fn warning_reason_end(
    s: &mut GuardScenario,
    id: &str,
    attempt: u64,
    expected: &Value,
    previous: &Value,
) {
    let restriction = json!({"requirement":"example.org/filter#limit.v1","arguments":{}});
    s.state
        .invocations
        .iter_mut()
        .find(|v| v["id"] == id)
        .unwrap()["filters"] = json!([restriction]);
    let input = Input::Complete {
        evidence: s.evidence("2026-11-21T13:03:15Z"),
        invocation: id.into(),
        attempt,
        completion: requests::Completion::Result {
            result: json!({"isError":true,
            "_meta":{"io.github.shrav-k.conduit/reason":"unavailable",
            "io.github.shrav-k.conduit/restriction":restriction}}),
        },
    };
    let t = step(&s.state, input.clone(), &s.c).unwrap();
    let fault = crate::record_fault::RecordFault::from_trial(input, &t, false)
        .unwrap()
        .step(s.state.clone(), &s.c)
        .unwrap();
    for ended in [&t, &fault] {
        assert_eq!(
            ended.reply["_meta"]["io.github.shrav-k.conduit/reason"],
            "unavailable"
        );
        assert_eq!(&ended.reply["_meta"][ADMISSION_KEY], expected);
        assert_warning_end(ended, id, expected);
        let restored = ended
            .state
            .invocations
            .iter()
            .find(|v| v["id"] == id)
            .unwrap();
        let reply = requests::ended_reply(&s.c, restored, &restored["contract"], "failed");
        assert_eq!(&reply["_meta"][ADMISSION_KEY], previous);
    }
}

#[test]
fn admission_warning_on_every_end_and_repeat() {
    let mut s = GuardScenario::new(guarded());
    s.keep("2026-11-21T13:02:00Z", &warning_report(), None);
    let t = s.keyed_admission_call("2026-11-21T13:02:10Z");
    let a = t.reply["invocation"].as_str().unwrap().to_owned();
    let w = dispatch_warning(&t, &a).clone();
    assert_eq!(w["level"], "warning");
    let t = s.finish(&a, "2026-11-21T13:02:20Z", result("succeeded", json!({})));
    assert_warning_end(&t, &a, &w);
    let t = s.keyed_admission_call("2026-11-21T13:02:30Z");
    assert!(t.external_intents.is_empty());
    assert_eq!(t.reply["_meta"][ADMISSION_KEY], w);
    let t = s.admission_call("2026-11-21T13:03:00Z", "call-b.turn.v1");
    let b = t.reply["invocation"].as_str().unwrap().to_owned();
    let b_warning = dispatch_warning(&t, &b).clone();
    let t = s.admission_input(
        "2026-11-21T13:03:10Z",
        json!({"kind":"cancel","invocation":b}),
    );
    assert_warning_end(&t, &b, &b_warning);
    let t = s.admission_call("2026-11-21T13:04:00Z", "call-a.turn.v1");
    let a2 = t.reply["invocation"].as_str().unwrap().to_owned();
    let a2_warning = dispatch_warning(&t, &a2).clone();
    let t = s.admission_input("2026-11-21T13:04:10Z", json!({"kind":"recover"}));
    assert_warning_end(&t, &a2, &a2_warning);

    let mut s = GuardScenario::new(discovered_guarded());
    s.keep("2026-11-21T13:02:00Z", &warning_report(), None);
    let t = s.keyed_admission_call("2026-11-21T13:02:10Z");
    let a = t.reply["invocation"].as_str().unwrap().to_owned();
    let t = s.discover_ask(&a, "2026-11-21T13:02:11Z");
    let w = dispatch_warning(&t, &a).clone();
    let t = s.finish(&a, "2026-11-21T13:02:20Z", result("failed", json!({})));
    assert_warning_end(&t, &a, &w);
    s.keep("2026-11-21T13:03:00Z", &report_one(), None);
    s.keyed_admission_call("2026-11-21T13:03:10Z");
    let t = s.admission_input("2026-11-21T13:03:11Z", json!({"kind":"complete","invocation":a,"attempt":2,"completion":{"kind":"result","result":{"capabilities":{"extensions":{"io.github.shrav-k.conduit/implementation-interface":{"revision":"v0"}}}}}}));
    assert!(dispatch_warning(&t, &a).is_null());
    warning_reason_end(&mut s, &a, 2, &Value::Null, &w);
    let t = s.admission_input("2026-11-21T13:03:20Z", json!({"kind":"complete","invocation":a,"attempt":2,"completion":{"kind":"result","result":result("succeeded",json!({}))}}));
    assert!(t.reply["_meta"].get(ADMISSION_KEY).is_none());

    // Restore an earlier un-warned failure after a warned IFC-026 end too.
    let mut s = GuardScenario::new(discovered_guarded());
    s.keep("2026-11-21T13:02:00Z", &report_one(), None);
    let t = s.keyed_admission_call("2026-11-21T13:02:10Z");
    let a = t.reply["invocation"].as_str().unwrap().to_owned();
    s.discover_ask(&a, "2026-11-21T13:02:11Z");
    s.finish(&a, "2026-11-21T13:02:20Z", result("failed", json!({})));
    s.keep("2026-11-21T13:03:00Z", &warning_report(), None);
    s.keyed_admission_call("2026-11-21T13:03:10Z");
    let requests::Completion::Result { result: discovered } = requests::advertised_extension(&s.c)
    else {
        unreachable!()
    };
    let t = s.admission_input("2026-11-21T13:03:11Z", json!({"kind":"complete","invocation":a,"attempt":2,"completion":{"kind":"result","result":discovered}}));
    let w = dispatch_warning(&t, &a).clone();
    assert_eq!(w["level"], "warning");
    warning_reason_end(&mut s, &a, 2, &w, &Value::Null);

    for cancel in [true, false] {
        let mut s = GuardScenario::new(discovered_guarded());
        s.keep("2026-11-21T13:02:00Z", &warning_report(), None);
        let t = s.admission_call("2026-11-21T13:02:10Z", "call-a.turn.v1");
        let a = t.reply["invocation"].as_str().unwrap().to_owned();
        let t = if cancel {
            s.admission_input(
                "2026-11-21T13:02:20Z",
                json!({"kind":"cancel","invocation":a}),
            )
        } else {
            s.finish(&a, "2026-11-21T13:02:20Z", json!({"capabilities":{}}))
        };
        assert!(t.reply["_meta"].get(ADMISSION_KEY).is_none());
        assert!(hold_reply(&t, &a)["_meta"].get(ADMISSION_KEY).is_none());
        assert!(
            !t.state
                .records
                .iter()
                .any(|r| r["type"] == "invocation-dispatched" && r["data"]["invocation"] == a)
        );
    }
}

#[test]
fn admission_warning_names_the_ask_only_to_who_may_inspect_it() {
    let mut s = GuardScenario::new(observer_guarded());
    s.call(
        "2026-11-21T13:01:00Z",
        "access.open.v1",
        json!({"conduit":"observer-turn"}),
    );
    let r4 = warning_report();
    let (k, _) = s.keep("2026-11-21T13:02:00Z", &r4, None);
    let observer = subject(&s.state, "Interface", "observer")["id"].clone();
    let t = s.admission_input("2026-11-21T13:02:10Z",json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":observer},"tool":"observer-turn.turn.v1","arguments":{}}}));
    let b = t.reply["invocation"].as_str().unwrap();
    let w = dispatch_warning(&t, b);
    assert_warning(&s, w, &r4, Value::Null);
    let text = w["message"]["text"].as_str().unwrap();
    assert!(text.contains("the login's ask") && !text.contains(&k));
    let t = s.admission_call("2026-11-21T13:02:11Z", "call-a.turn.v1");
    assert_warning(
        &s,
        dispatch_warning(&t, t.reply["invocation"].as_str().unwrap()),
        &r4,
        json!(k),
    );
}

#[test]
fn admission_warned_call_is_sent_as_without_the_guard() {
    for discover in [false, true] {
        let mut expected = None;
        for report in [
            Some(warning_report()),
            Some(single_use_report()),
            Some(report_one()),
            None,
        ] {
            let mut p = if discover {
                discovered_guarded()
            } else {
                guarded()
            };
            if report.is_none() {
                for name in ["shared-agent.v1", "login-holder.v1"] {
                    document_mut(&mut p, name)["spec"]
                        .as_object_mut()
                        .unwrap()
                        .remove("admission");
                }
            }
            let mut s = GuardScenario::new(p);
            if let Some(report) = report {
                s.keep("2026-11-21T13:02:00Z", &report, None);
            }
            let t = s.admission_call("2026-11-21T13:02:10Z", "call-a.turn.v1");
            let a = t.reply["invocation"].as_str().unwrap().to_owned();
            let mut intents = t.external_intents;
            if discover {
                intents.extend(s.discover_ask(&a, "2026-11-21T13:02:11Z").external_intents);
            }
            let actual = normalize_invocation(&json!(intents), &a);
            for word in [
                ADMISSION_KEY,
                "warnAt",
                "whenUnknown",
                "paidUsage",
                "admissionWarning",
            ] {
                assert!(!actual.to_string().contains(word));
            }
            if let Some(expected) = &expected {
                assert_eq!(&actual, expected);
            } else {
                expected = Some(actual);
            }
        }
    }
}

fn admission_open_project() -> packages::Project {
    let mut p = control_project();
    p.manifest.as_mut().unwrap()["implementations"]["resource"]["implements"]
        .as_array_mut()
        .unwrap()
        .push(json!("#read-admission.v1"));
    let api = p.manifest.as_ref().unwrap()["apiVersion"].clone();
    document_mut(&mut p, "tuning.v1")["spec"]["operations"] =
        json!(["#set-admission.v1", "#read-admission.v1"]);
    document_mut(&mut p, "login-holder.v1")["spec"]["implementation"]["#read-admission.v1"] =
        json!("#resource");
    let mut resource = document_mut(&mut p, "login").clone();
    resource["metadata"]["name"] = json!("tuner");
    p.documents.extend([
        resource,
        json!({"apiVersion":api,"kind":"Operation","metadata":{"name":"read-admission.v1"},"spec":{"inputSchema":{"type":"object"}}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"other-login-operation"},"spec":{"capability":"#tuning.v1","operations":["#read-admission.v1"]}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"agent-tune"},"from":"#tuning","consumer":{"kind":"Agent","name":"a"},"target":{"kind":"Resource","name":"login"}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"resource-tune"},"from":"#tuning","consumer":{"kind":"Resource","name":"tuner"},"target":{"kind":"Resource","name":"login"}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"agent-other"},"from":"#other-login-operation","consumer":{"kind":"Agent","name":"a"},"target":{"kind":"Resource","name":"login"}}),
        json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":"agent-b"},"from":"#turns","consumer":{"kind":"Agent","name":"a"},"target":{"kind":"Agent","name":"b"}}),
    ]);
    p
}

#[test]
fn admission_control_open_refused_for_an_agent_consumer() {
    let time = "2026-11-21T13:01:00Z";
    let mut s = GuardScenario::new(admission_open_project());
    assert_eq!(
        subject(&s.state, "Conduit", "agent-tune")["state"],
        "registered"
    );
    let t = s.call(time, "access.open.v1", json!({"conduit":"agent-tune"}));
    assert_eq!(t.reply["isError"], true);
    assert_eq!(
        t.reply["_meta"]["io.github.shrav-k.conduit/reason"],
        "refused"
    );
    let diagnostics = t.reply["structuredContent"]["diagnostics"]
        .as_array()
        .unwrap();
    assert_eq!(diagnostics.len(), 1);
    let d = &diagnostics[0];
    assert_eq!(d["ruleId"], "CTR-015");
    assert_eq!(d["properties"]["refusal"]["tier"], 2);
    let text = d["message"]["text"].as_str().unwrap();
    for word in [
        "agent-tune",
        "example.org/conformance/shared-login#set-admission.v1",
        "example.org/conformance/shared-login#login-holder.v1",
        "Nothing was opened",
        "Interface",
    ] {
        assert!(text.contains(word), "missing {word}");
    }
    assert!(t.external_intents.is_empty());
    for kind in ["access-decision", "request-ended"] {
        let record = t.records.iter().find(|r| r["type"] == kind).unwrap();
        assert_eq!(record["data"]["dispatched"], false);
        assert_eq!(record["data"]["reason"], "refused");
        assert_eq!(record["data"]["ruleId"], "CTR-015");
    }
    let inspected = s.call(
        time,
        "management.inspect.v1",
        json!({"object":{"kind":"Conduit","name":"agent-tune"}}),
    );
    assert_eq!(
        inspected.reply["structuredContent"]["status"]["state"],
        "registered"
    );
    for name in ["tune-login", "resource-tune", "agent-other", "agent-b"] {
        let t = s.call(time, "access.open.v1", json!({"conduit":name}));
        assert_eq!(t.reply["isError"], false, "{name}: {}", t.reply);
    }
    let mut unguarded = admission_open_project();
    for name in ["shared-agent.v1", "login-holder.v1"] {
        document_mut(&mut unguarded, name)["spec"]
            .as_object_mut()
            .unwrap()
            .remove("admission");
    }
    let mut unguarded = GuardScenario::new(unguarded);
    assert_eq!(
        unguarded
            .call(time, "access.open.v1", json!({"conduit":"agent-tune"}))
            .reply["isError"],
        false
    );

    let mut p = admission_open_project();
    document_mut(&mut p, "worker")["spec"]["conduits"] = json!({"tune":{
        "from":"#tuning","target":{"kind":"Resource","name":"login"},"optional":false}});
    let (state, c, _) = run_case(REPORT, p, 2);
    let mut s = GuardScenario {
        state,
        c,
        serial: 100,
    };
    let t = s.call(
        time,
        "workers.set-up.v1",
        json!({"instance":{"kind":"Agent","name":"a"}}),
    );
    assert_ne!(
        t.reply["_meta"]["io.github.shrav-k.conduit/outcome"],
        "succeeded"
    );
    assert!(t.records.iter().any(|r| r["type"] == "request-ended"
        && r["data"]["operation"] == s.c.qualified("open.v1")
        && r["data"]["reason"] == "refused"
        && r["data"]["ruleId"] == "CTR-015"));
    assert!(
        t.external_intents.is_empty(),
        "a refused required Conduit starts no run"
    );
    let agent = subject(&s.state, "Agent", "a");
    let child = access::object_by_id(&s.state, &agent["slots"]["conduits"]["tune"]["id"]).unwrap();
    assert_eq!(child["state"], "registered");
    assert_eq!(child["open"], false);
}

#[test]
fn admission_control_decision_retains_refusal_facts() {
    let mut s = GuardScenario::new(admission_open_project());
    let t = s.admission_input(
        "2026-11-21T13:01:00Z",
        json!({"kind":"access","request":{
        "identity":{"kind":"instance","id":subject(&s.state,"Interface","root")["id"]},
        "tool":"access.open.v1","arguments":{"conduit":"agent-tune"}}}),
    );
    assert_eq!(t.reply["reason"], "refused");
    let d = &t.reply["diagnostics"][0];
    assert_eq!(d["ruleId"], "CTR-015");
    assert_eq!(d["properties"]["refusal"]["tier"], 2);
    let text = d["message"]["text"].as_str().unwrap();
    for fact in [
        "agent-tune",
        "example.org/conformance/shared-login#set-admission.v1",
        "example.org/conformance/shared-login#login-holder.v1",
        "Nothing was opened",
    ] {
        assert!(text.contains(fact), "missing {fact}");
    }
    assert!(t.external_intents.is_empty());
    assert_eq!(
        subject(&s.state, "Conduit", "agent-tune")["state"],
        "registered"
    );
}

#[test]
fn admission_control_records_omit_explanation_fields() {
    for kind in ["invoke", "access"] {
        let mut s = GuardScenario::new(admission_open_project());
        let t = s.admission_input(
            "2026-11-21T13:01:00Z",
            json!({"kind":kind,"request":{
            "identity":{"kind":"instance","id":subject(&s.state,"Interface","root")["id"]},
            "tool":"access.open.v1","arguments":{"conduit":"agent-tune"}}}),
        );
        assert!(t.records.iter().any(|r| r["type"] == "access-decision"));
        for record in &t.records {
            for field in ["admissionControl", "rootLoginNotice"] {
                assert!(
                    record["data"].get(field).is_none(),
                    "{kind} {} contains {field}",
                    record["type"]
                );
            }
        }
    }
}

#[test]
fn admission_refusal_fixes_preserve_root_login_notice_facts() {
    let source = include_str!(
        "../../../../conformance/conventions/requests/login-resource-one-at-a-time/case.yaml"
    );
    let p = project(
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/conduit.json"
        ),
        include_str!(
            "../../../../conformance/conventions/requests/login-resource-one-at-a-time/documents.json"
        ),
    );
    let (state, c, captures) = run_case(source, p, 20);
    for kind in ["invoke", "access"] {
        let mut s = GuardScenario {
            state: state.clone(),
            c: c.clone(),
            serial: 100,
        };
        let t = s.admission_input("2026-12-01T13:00:00Z", json!({"kind":kind,"request":{
            "identity":{"kind":"instance","id":captures["observer"]},
            "tool":"observer-scope.answer.v1","arguments":{"action":captures["newNotice"],"answer":"acknowledged"}}}));
        let d = if kind == "invoke" {
            &t.reply["structuredContent"]["diagnostics"][0]
        } else {
            &t.reply["diagnostics"][0]
        };
        assert_eq!(d["ruleId"], "PND-009");
        assert!(
            d["message"]["text"]
                .as_str()
                .unwrap()
                .contains("Only root answers a login Resource's notice.")
        );
        assert!(t.external_intents.is_empty());
        for record in &t.records {
            for field in ["admissionControl", "rootLoginNotice"] {
                assert!(
                    record["data"].get(field).is_none(),
                    "{kind} {} contains {field}",
                    record["type"]
                );
            }
        }
    }
}
