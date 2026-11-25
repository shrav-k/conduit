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
    let id = s.dispatch("2026-11-21T13:07:00Z", "call-b.turn.v1", json!({}));
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
