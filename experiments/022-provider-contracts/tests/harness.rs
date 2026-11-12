use provider_contracts::{
    Options,
    form::Case,
    launch::digest,
    names,
    report::{Report, Verdict},
};
use serde_json::{Value, json};
use std::{fs, path::PathBuf, time::Duration};

struct Lab {
    dir: tempfile::TempDir,
    descriptor: Value,
    case: Value,
}
impl Lab {
    fn new(contract: &str, operation: &str) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let archive = dir.path().join("package.tar");
        let mut tar = tar::Builder::new(fs::File::create(&archive).unwrap());
        tar.append_path_with_name(env!("CARGO_BIN_EXE_scripted-provider"), "bin/provider")
            .unwrap();
        let manifest = json!({"implementations":{"provider":{"command":["bin/provider",contract],"environment":{"FIXTURE_MODE":"/mode","FIXTURE_BINDING":"/binding"}}}});
        let text = serde_yaml::to_string(&manifest).unwrap();
        let mut h = tar::Header::new_gnu();
        h.set_size(text.len() as u64);
        h.set_mode(0o600);
        h.set_cksum();
        tar.append_data(&mut h, "conduit.yaml", text.as_bytes())
            .unwrap();
        tar.finish().unwrap();
        let hash = digest(&fs::read(archive).unwrap());
        let descriptor = json!({"archive":format!("package.tar#{hash}"),"item":"provider","type":"example.invalid/providers#fixture.v1","instance":"00000000-0000-4000-8000-000000000001","revision":"revision-1","config":{"mode":"good","binding":"test-binding"},"parameters":{"machine":"fixture-host"},"features":[],"tool_version_command":[env!("CARGO_BIN_EXE_scripted-provider"),"--version"],"pass_environment":[],"lab_paths":{"record":"fixture-record"}});
        let meta = if operation == "get-secret.v1" {
            json!({"caller":"caller","invocation":"invocation","conduit":"conduit","filters":[{"requirement":"example.invalid/test#filter.v1","arguments":{}}]})
        } else if ["place.v1", "fetch-package.v1"].contains(&operation) {
            json!({"caller":"caller"})
        } else {
            json!({})
        };
        let case = json!({"id":"harness-case","title":"synthetic harness fixture","contract":contract,"promises":["PRV-005"],"features":[],"fixtures":{},"markers":[],"steps":[{"step":"call","operation":operation,"arguments":{},"meta":meta,"assertions":[{"assert":"outcome","value":"succeeded"}]}]});
        Self {
            dir,
            descriptor,
            case,
        }
    }
    fn runtime() -> Self {
        Self::new("runtime-provider.v1", "inspect-run.v1")
    }
    fn mode(&mut self, mode: &str) {
        self.descriptor["config"]["mode"] = json!(mode);
    }
    fn expected(&mut self, outcome: &str) {
        self.case["steps"][0]["assertions"][0]["value"] = json!(outcome);
    }
    fn write(&self) -> (PathBuf, PathBuf) {
        let d = self.dir.path().join("descriptor.yaml");
        let suite = self.dir.path().join("index.yaml");
        fs::write(&d, serde_yaml::to_string(&self.descriptor).unwrap()).unwrap();
        fs::write(
            self.dir.path().join("provider.yaml"),
            serde_yaml::to_string(&self.case).unwrap(),
        )
        .unwrap();
        fs::write(
            &suite,
            serde_yaml::to_string(
                &json!({"contract":self.case["contract"],"cases":["provider.yaml"]}),
            )
            .unwrap(),
        )
        .unwrap();
        (suite, d)
    }
    async fn run(&self) -> Report {
        let (suite, d) = self.write();
        provider_contracts::run_suite(
            &suite,
            &d,
            &self.dir.path().join("lab"),
            &Options {
                timeout: Duration::from_secs(5),
            },
        )
        .await
        .unwrap()
    }
}
fn passed(r: &Report) {
    assert!(
        r.totals.passed == 1 && r.totals.failed == 0 && r.totals.untested == 0,
        "fixed harness diagnostic: {:?}",
        r.cases.iter().map(|c| &c.detail).collect::<Vec<_>>()
    );
}
fn failed(r: &Report) {
    assert!(r.totals.failed == 1 && r.totals.passed == 0);
}

#[tokio::test]
async fn all_contracts_use_their_own_metadata() {
    for (contract, op) in [
        ("runtime-provider.v1", "run.v1"),
        ("runtime-provider.v1", "stop.v1"),
        ("runtime-provider.v1", "inspect-run.v1"),
        ("machine-provider.v1", "list-machines.v1"),
        ("package-source.v1", "fetch-package.v1"),
        ("secret-provider.v1", "get-secret.v1"),
    ] {
        passed(&Lab::new(contract, op).run().await);
    }
}
#[tokio::test]
async fn baseline_calls_keep_the_extension_declaration() {
    for (contract, operation) in [
        ("runtime-provider.v1", "run.v1"),
        ("runtime-provider.v1", "inspect-run.v1"),
        ("runtime-provider.v1", "stop.v1"),
        ("machine-provider.v1", "list-machines.v1"),
        ("machine-provider.v1", "place.v1"),
        ("package-source.v1", "fetch-package.v1"),
        ("secret-provider.v1", "get-secret.v1"),
    ] {
        let mut lab = Lab::new(contract, operation);
        lab.mode("baseline");
        if operation == "place.v1" {
            lab.case["steps"][0]["arguments"] = json!({"hostname":{"$param":"machine"}});
        }
        passed(&lab.run().await);
    }
}
#[tokio::test]
async fn incomplete_calls_stay_unknown_without_implicit_continuation() {
    for operation in ["inspect-run.v1", "run.v1"] {
        let mut lab = Lab::new("runtime-provider.v1", operation);
        lab.mode("input-required");
        lab.case["steps"][0]["assertions"] = json!([
            {"assert":"outcome","value":"unknown"},
            {"assert":"equal","pointer":"/resultType","value":"input_required"}
        ]);
        let mut second = lab.case["steps"][0].clone();
        second["operation"] = json!("inspect-run.v1");
        second["arguments"] = json!({"fixture_mode":"call-count"});
        second["assertions"] = json!([
            {"assert":"outcome","value":"succeeded"},
            {"assert":"equal","pointer":"/structuredContent/calls","value":2}
        ]);
        lab.case["steps"].as_array_mut().unwrap().push(second);
        passed(&lab.run().await);
    }
}
#[tokio::test]
async fn launch_failure_is_failed() {
    let mut lab = Lab::runtime();
    lab.mode("exit");
    failed(&lab.run().await);
}
#[tokio::test]
async fn digest_mismatch_is_failed() {
    let mut lab = Lab::runtime();
    lab.descriptor["archive"] = json!(format!("package.tar#{}", "0".repeat(64)));
    failed(&lab.run().await);
}
#[tokio::test]
async fn discovery_requires_extension_and_revision() {
    for mode in ["no-extension", "wrong-revision"] {
        let mut lab = Lab::runtime();
        lab.mode(mode);
        lab.expected("unknown");
        failed(&lab.run().await);
    }
}
#[tokio::test]
async fn malformed_reply_is_unknown() {
    let mut lab = Lab::runtime();
    lab.mode("malformed");
    lab.expected("unknown");
    passed(&lab.run().await);
}
#[tokio::test]
async fn output_schema_violation_is_unknown() {
    let mut lab = Lab::runtime();
    lab.mode("bad-schema");
    lab.expected("unknown");
    passed(&lab.run().await);
}
#[tokio::test]
async fn failed_and_unknown_need_no_outputs() {
    for mode in ["failed", "unknown"] {
        let mut lab = Lab::runtime();
        lab.mode(mode);
        lab.expected(mode);
        passed(&lab.run().await);
    }
}
#[tokio::test]
async fn contradictory_and_absent_outcomes_are_unknown() {
    for mode in [
        "both",
        "disagree",
        "neither",
        "invalid-outcome",
        "unqualified-error",
    ] {
        let mut lab = Lab::runtime();
        lab.mode(mode);
        lab.expected("unknown");
        passed(&lab.run().await);
    }
}
#[tokio::test]
async fn capability_error_is_failed() {
    let mut lab = Lab::runtime();
    lab.mode("protocol-error");
    lab.expected("failed");
    passed(&lab.run().await);
}
#[tokio::test]
async fn reasons_require_the_sent_filter() {
    for reason in ["refused", "unavailable"] {
        let mut lab = Lab::new("secret-provider.v1", "get-secret.v1");
        lab.mode(reason);
        lab.case["steps"][0]["assertions"] = json!([{"assert":"reason","value":reason}]);
        passed(&lab.run().await);
        lab.case["steps"][0]["meta"]["filters"] = json!([]);
        lab.case["steps"][0]["assertions"] = json!([{"assert":"outcome","value":"unknown"}]);
        passed(&lab.run().await);
    }
}
#[tokio::test]
async fn exact_assertions_and_patterns_pass_and_fail_without_values_in_report() {
    let mut lab = Lab::runtime();
    lab.case["steps"][0]["assertions"] = json!([{"assert":"equal","pointer":"/structuredContent/state","value":"running"},{"assert":"pattern","pointer":"/_meta/{meta:outcome}","pattern":"^succeeded$"}]);
    passed(&lab.run().await);
    lab.case["steps"][0]["assertions"][0]["value"] = json!("stopped");
    let r = lab.run().await;
    failed(&r);
    assert!(
        r.cases[0]
            .detail
            .as_ref()
            .unwrap()
            .contains("step 1, assertion 1")
    );
    assert!(!serde_json::to_string(&r).unwrap().contains("stopped"));
}
#[tokio::test]
async fn captures_parameters_and_fixtures_are_typed() {
    let mut lab = Lab::new("machine-provider.v1", "place.v1");
    lab.case["fixtures"] = json!({"machine":{"$param":"machine"}});
    lab.case["steps"][0]["arguments"] = json!({"hostname":{"$fixture":"machine"}});
    lab.case["steps"][0]["capture"] = json!({"chosen":"/structuredContent/hostname"});
    let mut second = lab.case["steps"][0].clone();
    second["arguments"] = json!({"hostname":{"$capture":"chosen"}});
    second.as_object_mut().unwrap().remove("capture");
    second["assertions"] = json!([{"assert":"equal","pointer":"/structuredContent/hostname","value":{"$param":"machine"}}]);
    lab.case["steps"].as_array_mut().unwrap().push(second);
    passed(&lab.run().await);
}
#[tokio::test]
async fn restart_and_relaunch_keep_private_state() {
    let mut lab = Lab::runtime();
    lab.mode("state");
    lab.case["steps"][0]["assertions"] =
        json!([{"assert":"equal","pointer":"/structuredContent/count","value":1}]);
    let mut second = lab.case["steps"][0].clone();
    second["assertions"][0]["value"] = json!(2);
    lab.case["steps"].as_array_mut().unwrap().extend([
        json!({"step":"pause","milliseconds":1}),
        json!({"step":"restart"}),
        second,
    ]);
    passed(&lab.run().await);
    lab.case["steps"][0]["assertions"][0]["value"] = json!(3);
    lab.case["steps"][3]["assertions"][0]["value"] = json!(4);
    passed(&lab.run().await);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let entries = fs::read_dir(lab.dir.path().join("lab/state")).unwrap();
        for e in entries {
            let p = e
                .unwrap()
                .path()
                .join(lab.descriptor["instance"].as_str().unwrap());
            assert!(fs::metadata(p).unwrap().permissions().mode() & 0o777 == 0o700);
        }
    }
}
#[tokio::test]
async fn dot_instance_ids_cannot_collapse_state_namespaces() {
    for instance in [".", ".."] {
        let mut lab = Lab::runtime();
        lab.descriptor["instance"] = json!(instance);
        let report = lab.run().await;
        failed(&report);
        assert!(report.cases[0].detail.as_deref() == Some("unsafe instance id"));
        assert!(!lab.dir.path().join("lab/state").exists());
    }
}
#[tokio::test]
async fn different_descriptors_get_distinct_initially_empty_state() {
    let mut lab = Lab::runtime();
    lab.mode("state");
    lab.case["steps"][0]["assertions"] =
        json!([{"assert":"equal","pointer":"/structuredContent/count","value":1}]);
    passed(&lab.run().await);
    lab.descriptor["config"]["binding"] = json!("different-binding");
    passed(&lab.run().await);
    let state = lab.dir.path().join("lab/state");
    let dirs: Vec<_> = fs::read_dir(&state)
        .unwrap()
        .map(|e| {
            fs::canonicalize(
                e.unwrap()
                    .path()
                    .join(lab.descriptor["instance"].as_str().unwrap()),
            )
            .unwrap()
        })
        .collect();
    assert!(dirs.len() == 2 && dirs[0] != dirs[1]);
    assert!(
        dirs.iter()
            .all(|p| p.starts_with(fs::canonicalize(&state).unwrap()))
    );
}
#[tokio::test]
async fn strict_unknown_case_step_and_assertion_fail() {
    for variant in 0..5 {
        let mut lab = Lab::runtime();
        match variant {
            0 => lab.case["unexpected"] = json!(true),
            1 => lab.case["steps"][0]["step"] = json!("surprise"),
            2 => lab.case["steps"][0]["assertions"][0]["assert"] = json!("surprise"),
            3 => lab.case["steps"]
                .as_array_mut()
                .unwrap()
                .push(json!({"step":"restart","unexpected":true})),
            _ => lab.case["steps"][0]["assertions"]
                .as_array_mut()
                .unwrap()
                .push(json!({"assert":"no_marker_anywhere","unexpected":true})),
        }
        failed(&lab.run().await);
    }
}
#[tokio::test]
async fn missing_feature_is_untested() {
    let mut lab = Lab::runtime();
    lab.case["features"] = json!(["missing-feature"]);
    let r = lab.run().await;
    assert!(r.totals.untested == 1 && r.cases[0].verdict == Verdict::Untested);
}
#[tokio::test]
async fn markers_are_checked_in_answers_and_stderr_and_never_reported() {
    for mode in ["echo-marker", "log-marker"] {
        let mut lab = Lab::runtime();
        lab.mode(mode);
        let marker = "SYNTHETIC-H1-MARKER";
        lab.case["markers"] = json!([marker]);
        lab.case["steps"][0]["arguments"] = json!({"marker":marker});
        lab.case["steps"][0]["assertions"]
            .as_array_mut()
            .unwrap()
            .push(json!({"assert":"no_marker_anywhere"}));
        let r = lab.run().await;
        failed(&r);
        assert!(!serde_json::to_string(&r).unwrap().contains(marker));
        assert!(!r.markdown().contains(marker));
    }
}
#[tokio::test]
async fn marker_assertion_observes_later_steps() {
    let mut lab = Lab::runtime();
    let mut second = lab.case["steps"][0].clone();
    lab.case["markers"] = json!(["SYNTHETIC-LATER-MARKER"]);
    lab.case["steps"][0]["assertions"]
        .as_array_mut()
        .unwrap()
        .push(json!({"assert":"no_marker_anywhere"}));
    second["arguments"] = json!({"fixture_mode":"log-marker","marker":"SYNTHETIC-LATER-MARKER"});
    lab.case["steps"].as_array_mut().unwrap().push(second);
    failed(&lab.run().await);
}
#[tokio::test]
async fn bindings_are_filled_and_report_contains_no_local_values() {
    let mut lab = Lab::runtime();
    lab.mode("environment");
    lab.case["steps"][0]["assertions"] = json!([{"assert":"equal","pointer":"/structuredContent/binding","value":"test-binding"},{"assert":"equal","pointer":"/structuredContent/ambient_absent","value":true}]);
    let r = lab.run().await;
    passed(&r);
    let encoded = serde_json::to_string(&r).unwrap();
    assert!(!encoded.contains(lab.dir.path().to_str().unwrap()));
    assert!(!encoded.contains("fixture-host"));
    assert!(!encoded.contains("test-binding"));
    assert!(r.markdown().contains("Passed: 1; failed: 0; untested: 0"));
}
#[tokio::test]
async fn both_reports_redact_markers_in_tool_name_and_version() {
    let mut lab = Lab::runtime();
    let marker = "SYNTHETIC-TOOL-MARKER";
    lab.case["markers"] = json!([marker]);
    lab.descriptor["tool_version_command"] = json!([
        env!("CARGO_BIN_EXE_scripted-provider"),
        "--version",
        format!("provider-{marker}"),
        format!("1.0.0-{marker}")
    ]);
    let report = lab.run().await;
    passed(&report);
    assert!(!serde_json::to_string(&report).unwrap().contains(marker));
    assert!(!report.markdown().contains(marker));
    let tool = report.tool.unwrap();
    assert!(tool.name == "<redacted>" && tool.version == "<redacted>");
}
#[tokio::test]
async fn both_reports_redact_case_ids_matching_fixture_values() {
    let mut lab = Lab::runtime();
    let hostname = "lab-fixture-hostname-only";
    lab.case["id"] = json!(hostname);
    lab.case["fixtures"] = json!({"machine":{"hostname":hostname}});
    let report = lab.run().await;
    passed(&report);
    assert!(!serde_json::to_string(&report).unwrap().contains(hostname));
    assert!(!report.markdown().contains(hostname));
    assert!(report.cases[0].id == "case-1");
}
#[tokio::test]
async fn later_untested_case_values_redact_earlier_report_fields() {
    let lab = Lab::runtime();
    let (suite, descriptor) = lab.write();
    let mut later = lab.case.clone();
    later["id"] = json!("later-case");
    later["features"] = json!(["missing-feature"]);
    later["markers"] = json!(["scripted-provider"]);
    later["fixtures"] = json!({"hostname":"harness-case"});
    fs::write(
        lab.dir.path().join("later.yaml"),
        serde_yaml::to_string(&later).unwrap(),
    )
    .unwrap();
    fs::write(
        &suite,
        serde_yaml::to_string(&json!({
            "contract":"runtime-provider.v1","cases":["provider.yaml","later.yaml"]
        }))
        .unwrap(),
    )
    .unwrap();
    let report = provider_contracts::run_suite(
        &suite,
        &descriptor,
        &lab.dir.path().join("lab"),
        &Default::default(),
    )
    .await
    .unwrap();
    assert!(report.totals.passed == 1 && report.totals.untested == 1);
    for hidden in ["scripted-provider", "harness-case"] {
        assert!(!serde_json::to_string(&report).unwrap().contains(hidden));
        assert!(!report.markdown().contains(hidden));
    }
}
#[test]
fn specification_names_are_current() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let spec: Value =
        serde_yaml::from_slice(&fs::read(root.join("spec/names.yaml")).unwrap()).unwrap();
    for (key, value) in [
        ("corePackage", names::CORE),
        ("metaPrefix", names::PREFIX),
        ("engineUrlVariable", names::ENGINE_URL),
        ("stateDirVariable", names::STATE_DIR),
    ]
    .into_iter()
    .chain(names::IDENTITY_FILES)
    {
        assert!(
            spec[key] == value,
            "specification-assigned name differs: {key}"
        );
    }
}
#[test]
fn copied_outputs_and_reachable_definitions_match_specification() {
    use serde::Deserialize;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let copied = provider_contracts::schema::copied();
    let text = fs::read_to_string(root.join("spec/core/operations.yaml")).unwrap();
    let mut actual = serde_json::Map::new();
    for d in serde_yaml::Deserializer::from_str(&text) {
        let d = Value::deserialize(d).unwrap();
        if let Some(name) = d["metadata"]["name"]
            .as_str()
            .filter(|n| copied["outputs"].get(n).is_some())
        {
            actual.insert(name.into(), d["spec"]["outputSchema"].clone());
        }
    }
    assert!(
        Value::Object(actual) == copied["outputs"],
        "operation output copy differs"
    );
    let text = fs::read_to_string(root.join("spec/schemas/values.yaml")).unwrap();
    let definitions: Values = serde_yaml::Deserializer::from_str(&text)
        .filter_map(|d| {
            let d = Value::deserialize(d).unwrap();
            let n = d["metadata"]["name"].as_str()?;
            copied["definitions"]
                .get(n)
                .map(|_| (n.to_owned(), d["spec"].clone()))
        })
        .collect();
    assert!(
        serde_json::to_value(definitions).unwrap() == copied["definitions"],
        "reachable definition copy differs"
    );
    for op in [
        "run.v1",
        "stop.v1",
        "inspect-run.v1",
        "list-machines.v1",
        "place.v1",
        "fetch-package.v1",
        "get-secret.v1",
    ] {
        assert!(copied["outputs"].get(op).is_some());
    }
}
type Values = std::collections::BTreeMap<String, Value>;
#[test]
fn validation_rejects_cross_contract_calls_and_metadata() {
    let mut lab = Lab::runtime();
    for operation in ["place.v1", "get-secret.v1"] {
        lab.case["steps"][0]["operation"] = json!(operation);
        let c: Case = serde_json::from_value(lab.case.clone()).unwrap();
        assert!(c.validate().is_err());
    }
    lab.case["steps"][0]["operation"] = json!("inspect-run.v1");
    lab.case["steps"][0]["meta"] = json!({"filters":[]});
    let c: Case = serde_json::from_value(lab.case).unwrap();
    assert!(c.validate().is_err());
}

#[test]
fn capability_errors_require_well_formed_named_capabilities() {
    use provider_contracts::{
        answer::{Decision, read},
        form::Outcome,
    };
    for caps in [
        Value::Null,
        json!({}),
        json!({"sampling":null}),
        json!({"extensions":{}}),
        json!({"extensions":{"other":null}}),
    ] {
        assert!(
            read(
                "inspect-run.v1",
                &json!({"error":{"code":-32021,"data":{"requiredCapabilities":caps}}}),
                &json!([])
            ) == Decision::Outcome(Outcome::Unknown)
        );
    }
    for caps in [
        json!({"sampling":{}}),
        json!({"extensions":{"example.invalid/other":{}}}),
    ] {
        assert!(
            read(
                "inspect-run.v1",
                &json!({"error":{"code":-32021,"data":{"requiredCapabilities":caps}}}),
                &json!([])
            ) == Decision::Outcome(Outcome::Failed)
        );
    }
}
#[test]
fn markers_are_detected_across_chunks_and_in_decoded_json() {
    let observed = provider_contracts::observe::Observations::default();
    observed.add_wire(br#"{"result":{"value":"SYNTHETIC\nMARKER"}}"#);
    assert!(observed.contains_marker(&["SYNTHETIC\nMARKER".into()]));
    let observed = provider_contracts::observe::Observations::default();
    observed.add(b"SYNTHETIC-");
    observed.add(b"MARKER");
    assert!(observed.contains_marker(&["SYNTHETIC-MARKER".into()]));
}
#[tokio::test]
async fn scalar_bindings_use_rfc8785_and_null_is_omitted() {
    for (value, expected) in [
        (json!(1.0), json!("1")),
        (json!(true), json!("true")),
        (Value::Null, Value::Null),
    ] {
        let mut lab = Lab::runtime();
        lab.mode("environment");
        lab.descriptor["config"]["binding"] = value;
        lab.case["steps"][0]["assertions"] =
            json!([{"assert":"equal","pointer":"/structuredContent/binding","value":expected}]);
        passed(&lab.run().await);
    }
}
fn replace_archive(lab: &mut Lab, item: Value) {
    let archive = lab.dir.path().join("package.tar");
    let mut tar = tar::Builder::new(fs::File::create(&archive).unwrap());
    let manifest = serde_yaml::to_string(&json!({"implementations":{"provider":item}})).unwrap();
    let mut h = tar::Header::new_gnu();
    h.set_size(manifest.len() as u64);
    h.set_mode(0o600);
    h.set_cksum();
    tar.append_data(&mut h, "conduit.yaml", manifest.as_bytes())
        .unwrap();
    tar.finish().unwrap();
    lab.descriptor["archive"] = json!(format!(
        "package.tar#{}",
        digest(&fs::read(archive).unwrap())
    ));
}
#[tokio::test]
async fn missing_executable_is_a_failed_launch() {
    let mut lab = Lab::runtime();
    replace_archive(&mut lab, json!({"command":["bin/missing"]}));
    lab.expected("unknown");
    failed(&lab.run().await);
}
#[tokio::test]
async fn registry_items_are_untested() {
    let mut lab = Lab::runtime();
    replace_archive(
        &mut lab,
        json!({"package":{"registryType":"npm","identifier":"@example/fixture","version":"1.0.0","transport":{"type":"stdio"}}}),
    );
    let r = lab.run().await;
    assert!(r.totals.untested == 1 && r.totals.failed == 0);
}
#[test]
fn lab_path_substitution_rejects_symlink_escape() {
    #[cfg(unix)]
    {
        use provider_contracts::{form::Descriptor, substitute::Context};
        let lab = Lab::runtime();
        let root = lab.dir.path().join("lab");
        fs::create_dir(&root).unwrap();
        std::os::unix::fs::symlink(lab.dir.path(), root.join("escape")).unwrap();
        let mut d = lab.descriptor;
        d["lab_paths"]["record"] = json!("escape/anything");
        let d: Descriptor = serde_json::from_value(d).unwrap();
        let empty = Values::new();
        let ctx = Context {
            descriptor: &d,
            fixtures: &empty,
            captures: &empty,
            lab: &root,
        };
        assert!(ctx.expand(&json!({"$lab":"record"})).is_err());
    }
}
#[test]
fn schema_validator_checks_referenced_labels_and_successful_uri() {
    use provider_contracts::schema::output_valid;
    assert!(output_valid(
        "list-machines.v1",
        &json!({"machines":[{"hostname":"machine","labels":{"valid/key":"value"},"state":"reachable"}]})
    ));
    assert!(!output_valid(
        "list-machines.v1",
        &json!({"machines":[{"hostname":"machine","labels":{"invalid key":"value"},"state":"reachable"}]})
    ));
    assert!(!output_valid("run.v1", &json!({"url":"not a URI"})));
}
#[tokio::test]
async fn binary_writes_both_reports_and_nonzero_failure_exit() {
    let mut lab = Lab::runtime();
    lab.mode("environment");
    lab.descriptor["pass_environment"] = json!(["FIXTURE_ALLOWLIST"]);
    lab.case["steps"][0]["assertions"] = json!([
        {"assert":"equal","pointer":"/structuredContent/ambient_absent","value":true},
        {"assert":"equal","pointer":"/structuredContent/binding","value":"test-binding"},
        {"assert":"equal","pointer":"/structuredContent/allowed","value":"test-pass"}
    ]);
    let (suite, descriptor) = lab.write();
    let status = std::process::Command::new(env!("CARGO_BIN_EXE_provider-contracts"))
        .env("AMBIENT_SENTINEL", "synthetic-ambient")
        .env("FIXTURE_ALLOWLIST", "test-pass")
        .env("FIXTURE_BINDING", "synthetic-ambient-binding")
        .args([
            suite,
            descriptor,
            lab.dir.path().join("lab"),
            lab.dir.path().join("report.json"),
            lab.dir.path().join("report.md"),
        ])
        .output()
        .unwrap();
    assert!(status.status.success());
    let report: Value =
        serde_json::from_slice(&fs::read(lab.dir.path().join("report.json")).unwrap()).unwrap();
    assert!(
        report["totals"]["passed"] == 1 && report["archive_digest"].as_str().unwrap().len() == 64
    );
    let markdown = fs::read_to_string(lab.dir.path().join("report.md")).unwrap();
    assert!(markdown.contains("Archive SHA-256") && markdown.contains("scripted-provider"));
    let mut bad = Lab::runtime();
    bad.expected("failed");
    let (suite, descriptor) = bad.write();
    let status = std::process::Command::new(env!("CARGO_BIN_EXE_provider-contracts"))
        .args([
            suite,
            descriptor,
            bad.dir.path().join("lab"),
            bad.dir.path().join("report.json"),
            bad.dir.path().join("report.md"),
        ])
        .output()
        .unwrap();
    assert!(status.status.code() == Some(1));
}

#[tokio::test]
async fn marker_check_keeps_observing_after_protocol_parser_stops() {
    use provider_contracts::observe::{Observations, WireReader, pump};
    let observed = Observations::default();
    let (sender, reader) = WireReader::channel();
    drop(reader);
    let source = b"malformed reply\nSYNTHETIC-AFTER-PARSER-MARKER\n";
    pump(&source[..], observed.clone(), sender).await;
    assert!(observed.contains_marker(&["SYNTHETIC-AFTER-PARSER-MARKER".into()]));
}
#[tokio::test]
async fn duplicate_yaml_keys_fail_the_case() {
    let lab = Lab::runtime();
    let (suite, descriptor) = lab.write();
    let path = lab.dir.path().join("provider.yaml");
    let mut bytes = fs::read(&path).unwrap();
    bytes.extend_from_slice(b"title: duplicate title\n");
    fs::write(path, bytes).unwrap();
    let report = provider_contracts::run_suite(
        &suite,
        &descriptor,
        &lab.dir.path().join("lab"),
        &Default::default(),
    )
    .await
    .unwrap();
    failed(&report);
}
#[test]
fn copied_schema_bundle_contains_every_transitive_reference() {
    let copied = provider_contracts::schema::copied();
    fn visit(v: &Value, copied: &Value, visited: &mut std::collections::BTreeSet<String>) {
        match v {
            Value::Object(m) => {
                if let Some(reference) = m.get("$ref").and_then(Value::as_str) {
                    let prefix = format!("{}#", names::CORE);
                    let name = reference
                        .strip_prefix(&prefix)
                        .expect("unexpected schema reference");
                    if visited.insert(name.to_owned()) {
                        let definition = copied["definitions"]
                            .get(name)
                            .expect("reachable definition missing");
                        visit(definition, copied, visited);
                    }
                }
                for v in m.values() {
                    visit(v, copied, visited);
                }
            }
            Value::Array(a) => {
                for v in a {
                    visit(v, copied, visited);
                }
            }
            _ => (),
        }
    }
    let mut visited = std::collections::BTreeSet::new();
    visit(&copied["outputs"], &copied, &mut visited);
    assert!(visited.len() == copied["definitions"].as_object().unwrap().len());
}
#[tokio::test]
async fn archive_links_are_rejected_before_launch() {
    let mut lab = Lab::runtime();
    let path = lab.dir.path().join("package.tar");
    let mut archive = tar::Builder::new(fs::File::create(&path).unwrap());
    let mut h = tar::Header::new_gnu();
    h.set_entry_type(tar::EntryType::Symlink);
    h.set_size(0);
    h.set_mode(0o777);
    h.set_link_name("../../outside").unwrap();
    h.set_cksum();
    archive
        .append_data(&mut h, "escape", std::io::empty())
        .unwrap();
    archive.finish().unwrap();
    lab.descriptor["archive"] = json!(format!("package.tar#{}", digest(&fs::read(path).unwrap())));
    failed(&lab.run().await);
}

#[tokio::test]
async fn parameter_header_repair_is_once_and_only_for_read_only_operations() {
    let mut lab = Lab::runtime();
    lab.mode("repair");
    passed(&lab.run().await);
    lab.mode("repair-always");
    lab.expected("unknown");
    passed(&lab.run().await);
    let mut lab = Lab::new("runtime-provider.v1", "run.v1");
    lab.mode("repair");
    lab.expected("unknown");
    passed(&lab.run().await);
}

#[tokio::test]
async fn regression_125_literal_identity_and_secrets_are_redacted_in_both_reports() {
    for field in ["identity", "secrets"] {
        let mut lab = Lab::runtime();
        let literal = digest(format!("direct argument {field}").as_bytes());
        lab.case["steps"][0]["arguments"][field] = json!({"token": literal, "nested": [literal]});
        lab.case["id"] = json!(format!("case-{literal}"));
        let report = lab.run().await;
        passed(&report);
        assert!(!serde_json::to_string(&report).unwrap().contains(&literal));
        assert!(!report.markdown().contains(&literal));
    }
}

#[tokio::test]
async fn regression_125_secret_equal_literal_is_redacted() {
    let mut lab = Lab::new("secret-provider.v1", "get-secret.v1");
    let literal = digest(b"equal assertion literal");
    lab.case["id"] = json!(format!("case-{literal}"));
    lab.case["steps"][0]["assertions"] = json!([
        {"assert":"equal", "pointer":"/structuredContent/value", "value":literal}
    ]);
    let report = lab.run().await;
    failed(&report);
    assert!(!serde_json::to_string(&report).unwrap().contains(&literal));
    assert!(!report.markdown().contains(&literal));
}

#[tokio::test]
async fn regression_125_placeholders_do_not_redact_their_names() {
    let mut lab = Lab::runtime();
    lab.case["id"] = json!("hostname-machine-captured");
    lab.case["fixtures"] = json!({"hostname":"fixture-value"});
    lab.case["steps"][0]["capture"] = json!({"captured":"/structuredContent/state"});
    let mut second = lab.case["steps"][0].clone();
    second["arguments"] = json!({"identity": {"one":{"$param":"machine"}, "two":{"$fixture":"hostname"}}, "secrets":[{"$capture":"captured"}]});
    lab.case["steps"].as_array_mut().unwrap().push(second);
    let report = lab.run().await;
    passed(&report);
    assert!(report.cases[0].id == "hostname-machine-captured");
}

#[tokio::test]
async fn regression_125_empty_label_keys_are_redacted_without_erasing_machine() {
    for source in ["config", "parameters", "fixtures"] {
        let mut lab = Lab::runtime();
        let label = "lab-label-key-only";
        let labels = json!({"labels":{label:""}});
        match source {
            "config" => lab.descriptor["config"]["machine"] = labels,
            "parameters" => lab.descriptor["parameters"]["labels"] = labels["labels"].clone(),
            _ => lab.case["fixtures"]["machine"] = labels,
        }
        lab.case["id"] = json!("machine-hostname-case");
        lab.descriptor["tool_version_command"] = json!([
            env!("CARGO_BIN_EXE_scripted-provider"),
            "--version",
            format!("provider-{label}")
        ]);
        let report = lab.run().await;
        passed(&report);
        assert!(!serde_json::to_string(&report).unwrap().contains(label));
        assert!(!report.markdown().contains(label));
        assert!(report.cases[0].id == "machine-hostname-case");
    }
}

async fn revision_case(mode: &str, expected_calls: usize, outcome: &str, operation: &str) {
    let mut lab = Lab::new("runtime-provider.v1", operation);
    lab.mode(mode);
    lab.expected(outcome);
    let mut count = lab.case["steps"][0].clone();
    count["arguments"] = json!({"fixture_mode":"call-count"});
    count["assertions"] = json!([
        {"assert":"outcome","value":"succeeded"},
        {"assert":"equal","pointer":"/structuredContent/calls","value":expected_calls + 1}
    ]);
    lab.case["steps"].as_array_mut().unwrap().push(count);
    passed(&lab.run().await);
}

#[tokio::test]
async fn regression_125_revision_fallback_uses_newest_shared_revision() {
    revision_case("revision-success", 2, "succeeded", "inspect-run.v1").await;
    revision_case("revision-success", 2, "succeeded", "run.v1").await;
    // Modern sessions overwrite params._meta; only request options win after that.
    revision_case("revision-modern", 2, "succeeded", "run.v1").await;
}

#[tokio::test]
async fn regression_125_revision_fallback_uses_discovery_when_supported_absent() {
    revision_case("revision-discovery", 2, "succeeded", "inspect-run.v1").await;
}

#[tokio::test]
async fn regression_125_revision_no_shared_alternative_sends_nothing_more() {
    revision_case("revision-no-shared", 1, "unknown", "inspect-run.v1").await;
}

#[tokio::test]
async fn regression_125_revision_retry_rejected_stops_after_two_sends() {
    revision_case("revision-rejected", 2, "unknown", "inspect-run.v1").await;
}

#[tokio::test]
async fn regression_125_header_retry_cannot_retry_again_for_revision() {
    revision_case("repair-then-revision", 2, "unknown", "inspect-run.v1").await;
}

#[tokio::test]
async fn regression_125_revision_retry_cannot_retry_again_for_headers() {
    revision_case("revision-then-repair", 2, "unknown", "inspect-run.v1").await;
}

#[tokio::test]
async fn regression_125_verifier_revision_retry_rejects_malformed_content() {
    // Otherwise valid success fields must not bypass MCP response validation.
    revision_case("malformed-content", 1, "unknown", "inspect-run.v1").await;
    revision_case("revision-malformed-content", 2, "unknown", "inspect-run.v1").await;
}

#[tokio::test]
async fn regression_125_verifier_mixed_supported_list_sends_no_retry() {
    revision_case("revision-mixed-supported", 1, "unknown", "inspect-run.v1").await;
}
