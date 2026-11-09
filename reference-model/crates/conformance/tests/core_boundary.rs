//! Check the pure boundary against real built-in templates and the portable first-start input.
use conduit_core::{Context, Evidence, Input, Request, State, step};
use conduit_loader::{Specification, read_file};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn fixture() -> (Context, Value) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let specification = Specification::read(&root.join("spec")).unwrap();
    let mut templates = BTreeMap::new();
    for document in read_file(&root.join("spec/core/templates.yaml")).unwrap() {
        let value = document.root.to_json();
        templates.insert(
            value["metadata"]["name"].as_str().unwrap().to_string(),
            value,
        );
    }
    let case =
        read_file(&root.join("conformance/conventions/first-start/initial/case.yaml")).unwrap();
    let input = case[0].root.to_json()["steps"][0]["input"].clone();
    (
        Context {
            core_package: specification.names().core_package.clone(),
            templates,
            record_source: "urn:conduit:first-build".into(),
        },
        input,
    )
}

#[test]
fn same_boundary_values_give_the_same_transition_and_restored_state_keeps_ids() {
    let (context, input) = fixture();
    let initial = State::default();
    let first = step(
        &initial,
        serde_json::from_value(input.clone()).unwrap(),
        &context,
    )
    .unwrap();
    let again = step(
        &initial,
        serde_json::from_value(input.clone()).unwrap(),
        &context,
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(&first).unwrap(),
        serde_json::to_value(again).unwrap()
    );
    let restored: State =
        serde_json::from_slice(&serde_json::to_vec(&first.state).unwrap()).unwrap();
    let mut restart_input = input;
    restart_input["evidence"]["ids"] = serde_json::json!([]);
    let repeat = step(
        &restored,
        serde_json::from_value(restart_input).unwrap(),
        &context,
    )
    .unwrap();
    assert_eq!(repeat.state, restored);
    assert!(repeat.records.is_empty());
    assert!(repeat.external_intents.is_empty());
}

#[test]
fn partial_allocations_and_reused_ids_never_publish_partial_state() {
    let (context, mut input) = fixture();
    // Exhaust only after objects and some records were constructed privately.
    input["evidence"]["ids"]
        .as_array_mut()
        .unwrap()
        .truncate(12);
    let state = State::default();
    let before = state.clone();
    assert!(
        step(
            &state,
            serde_json::from_value(input.clone()).unwrap(),
            &context
        )
        .unwrap_err()
        .contains("exhausted")
    );
    assert_eq!(state, before);
    input["evidence"]["ids"][3] = input["evidence"]["ids"][0].clone();
    assert!(
        step(&state, serde_json::from_value(input).unwrap(), &context)
            .unwrap_err()
            .contains("reused")
    );
    assert_eq!(state, before);
}

#[test]
fn unsupported_input_does_not_become_a_conduit_reason() {
    let (_, mut input) = fixture();
    input["kind"] = serde_json::json!("tick");
    assert!(serde_json::from_value::<Input>(input).is_err());
}

#[test]
fn registration_requires_both_open_conduits_listing_the_operation() {
    let (context, input) = fixture();
    let mut state = step(
        &State::default(),
        serde_json::from_value(input).unwrap(),
        &context,
    )
    .unwrap()
    .state;
    let root = state.objects[4]["id"].as_str().unwrap().to_owned();
    let management = state.objects[2]["id"].as_str().unwrap().to_owned();
    let management_conduit = state.objects[5]["id"].as_str().unwrap().to_owned();
    let operation = format!("{}#register.v1", context.core_package);
    let grant = json!({"id":"00000000-0000-4000-8000-000000000050", "kind":"Conduit",
        "state":"ready", "open":true, "consumer":root, "target":state.objects[0]["id"],
        "capability":format!("{}#pool-management.v1", context.core_package),
        "operations":[operation]});
    state.objects.push(grant.clone());
    let request = Request {
        sender: root.clone(),
        caller: root,
        conduit: management_conduit,
        target: management,
        operation,
    };
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let documents =
        read_file(&repository.join("conformance/language/first-start/protected/project/node.yaml"))
            .unwrap()
            .into_iter()
            .map(|d| d.root.to_json())
            .collect::<Vec<_>>();
    for fault in [
        "none",
        "missing",
        "closed",
        "not-ready",
        "unlisted",
        "other-consumer",
        "other-target",
        "management-closed",
        "management-unlisted",
    ] {
        let mut initial = state.clone();
        match fault {
            "none" => {}
            "missing" => {
                initial.objects.pop();
            }
            "closed" => initial.objects[7]["open"] = json!(false),
            "not-ready" => initial.objects[7]["state"] = json!("registered"),
            "unlisted" => initial.objects[7]["operations"] = json!([]),
            "other-consumer" => initial.objects[7]["consumer"] = initial.objects[3]["id"].clone(),
            "other-target" => initial.objects[7]["target"] = initial.objects[3]["id"].clone(),
            "management-closed" => initial.objects[5]["open"] = json!(false),
            "management-unlisted" => initial.objects[5]["operations"] = json!([]),
            _ => unreachable!(),
        }
        let result = step(
            &initial,
            Input::Submit {
                evidence: Evidence {
                    time: "2026-11-09T12:00:00Z".into(),
                    ids: vec![
                        "00000000-0000-4000-8000-000000000100".into(),
                        "00000000-0000-4000-8000-000000000101".into(),
                    ],
                    revisions: vec![],
                    suffixes: vec![],
                },
                request: request.clone(),
                documents: documents.clone(),
            },
            &context,
        )
        .unwrap();
        assert_eq!(result.state.objects, initial.objects, "{fault}");
        assert!(result.external_intents.is_empty(), "{fault}");
        if fault == "none" {
            assert_eq!(result.reply["reason"], "invalid");
            assert_eq!(result.reply["diagnostics"][0]["ruleId"], "FST-006");
        } else {
            assert_eq!(result.reply, json!({"reason":"refused"}), "{fault}");
            assert!(
                result
                    .decisions
                    .iter()
                    .any(|d| d.promises.iter().any(|p| p == "OPS-023"))
            );
        }
    }
}

#[test]
fn registration_allocation_failures_leave_objects_records_and_allocations_untouched() {
    use conduit_core::packages::Project;
    let (context, input) = fixture();
    let initial = step(
        &State::default(),
        serde_json::from_value(input).unwrap(),
        &context,
    )
    .unwrap()
    .state;
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let spec = Specification::read(&repository.join("spec")).unwrap();
    let core = conduit_loader::read_project(&repository.join("spec/core"), &spec).unwrap();
    let mut documents = core
        .documents
        .iter()
        .map(|d| d.root.to_json())
        .collect::<Vec<_>>();
    for entry in std::fs::read_dir(repository.join("spec/schemas")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|s| s == "yaml") {
            documents.extend(read_file(&path).unwrap().iter().map(|d| d.root.to_json()));
        }
    }
    let evidence = Evidence {
        time: "2026-11-09T12:00:00Z".into(),
        ids: (100..104)
            .map(|i| format!("00000000-0000-4000-8000-{i:012}"))
            .collect(),
        revisions: vec!["t".into(), "i".into()],
        suffixes: vec![],
    };
    let request = Request {
        sender: initial.objects[4]["id"].as_str().unwrap().into(),
        caller: initial.objects[4]["id"].as_str().unwrap().into(),
        conduit: initial.objects[5]["id"].as_str().unwrap().into(),
        target: initial.objects[2]["id"].as_str().unwrap().into(),
        operation: format!("{}#register.v1", context.core_package),
    };
    let project = Project {
        manifest: None,
        directory: None,
        documents: vec![
            json!({"apiVersion":spec.names().api_version,"kind":"Pool","from":"pool","metadata":{"name":"p"}}),
            json!({"apiVersion":spec.names().api_version,"kind":"Pool","metadata":{"name":"pool"},"spec":{"type":format!("{}#pool.v1",context.core_package)}}),
        ],
    };
    let builtin = Project {
        manifest: core.manifest.map(|d| d.root.to_json()),
        documents,
        directory: None,
    };
    let original = initial.clone();
    for fault in ["exhausted", "reused"] {
        let mut allocations = evidence.clone();
        if fault == "exhausted" {
            allocations.ids.truncate(3);
        } else {
            allocations.ids[1] = allocations.ids[0].clone();
        }
        let result = step(
            &initial,
            Input::RegisterProject {
                snapshots: vec![],
                builtin: builtin.clone(),
                project: project.clone(),
                evidence: allocations,
                request: request.clone(),
            },
            &context,
        );
        assert!(result.unwrap_err().contains(fault));
        assert_eq!(initial, original);
    }
    let accepted = step(
        &initial,
        Input::RegisterProject {
            snapshots: vec![],
            builtin,
            project,
            evidence,
            request,
        },
        &context,
    )
    .unwrap();
    assert_eq!(accepted.reply["accepted"], true);
    assert_eq!(accepted.state.objects.len(), initial.objects.len() + 2);
    assert_eq!(accepted.records.len(), 2);
    assert!(accepted.external_intents.is_empty());
}
