use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::{collections::BTreeMap, path::Path};

fn documents(path: &Path) -> Vec<Value> {
    serde_yaml::Deserializer::from_str(&std::fs::read_to_string(path).unwrap())
        .map(|d| Value::deserialize(d).unwrap())
        .collect()
}

fn contracts() -> BTreeMap<String, Value> {
    documents(&Path::new(env!("CARGO_MANIFEST_DIR")).join("contracts.yaml"))
        .into_iter()
        .map(|d| (d["metadata"]["name"].as_str().unwrap().to_owned(), d))
        .collect()
}

fn compile(schema: &Value) -> jsonschema::Validator {
    jsonschema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .should_validate_formats(true)
        .build(schema)
        .unwrap()
}

// Adapted from provider-local-machine/tests/schema_drift.rs: resolve Conduit's
// schema-contract references into local Draft 2020-12 definitions, without fetches.
fn convert(
    value: &Value,
    owner: Option<&str>,
    core: &str,
    source: &BTreeMap<String, Value>,
    defs: &mut Map<String, Value>,
) -> Value {
    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|v| convert(v, owner, core, source, defs))
                .collect(),
        ),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| {
                    if key == "$ref"
                        && let Some(reference) = value.as_str()
                    {
                        if let Some(name) = reference.strip_prefix(&format!("{core}#")) {
                            if !defs.contains_key(name) {
                                defs.insert(name.into(), json!({}));
                                let converted = convert(
                                    source
                                        .get(name)
                                        .unwrap_or_else(|| panic!("missing schema {name}")),
                                    Some(name),
                                    core,
                                    source,
                                    defs,
                                );
                                defs.insert(name.into(), converted);
                            }
                            return (key.clone(), json!(format!("#/$defs/{name}")));
                        }
                        if let (Some(owner), Some(local)) = (owner, reference.strip_prefix('#')) {
                            return (key.clone(), json!(format!("#/$defs/{owner}{local}")));
                        }
                    }
                    (key.clone(), convert(value, owner, core, source, defs))
                })
                .collect(),
        ),
        _ => value.clone(),
    }
}

#[test]
fn package_documents_validate_against_spec_including_schema_profile() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec = root.join("../../spec");
    if !spec.exists() {
        eprintln!("SKIPPED manifest/document schema validation: spec/ absent");
        return;
    }
    let assigned: Value =
        serde_yaml::from_str(&std::fs::read_to_string(spec.join("names.yaml")).unwrap()).unwrap();
    let core = assigned["corePackage"].as_str().unwrap();
    let mut source = BTreeMap::new();
    for directory in ["schemas", "core"] {
        for entry in std::fs::read_dir(spec.join(directory)).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "yaml") {
                for doc in documents(&path) {
                    if doc["kind"] == "Schema" {
                        source.insert(
                            doc["metadata"]["name"].as_str().unwrap().to_owned(),
                            doc["spec"].clone(),
                        );
                    }
                }
            }
        }
    }
    for (file, name) in [
        ("conduit.yaml", "manifest.v1"),
        ("contracts.yaml", "document.v1"),
    ] {
        let mut defs = Map::new();
        let mut schema = convert(
            &json!({"$ref": format!("{core}#{name}")}),
            None,
            core,
            &source,
            &mut defs,
        );
        schema["$defs"] = Value::Object(defs);
        let validator = compile(&schema);
        for document in documents(&root.join(file)) {
            let errors: Vec<_> = validator
                .iter_errors(&document)
                .map(|e| e.to_string())
                .collect();
            assert!(errors.is_empty(), "{file}: {errors:?}");
        }
    }
}

#[test]
fn exports_are_unique_local_contracts_and_capabilities_resolve() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = documents(&root.join("conduit.yaml")).remove(0);
    let docs = documents(&root.join("contracts.yaml"));
    let by_name = contracts();
    assert_eq!(docs.len(), by_name.len(), "duplicate contract name");
    let exports = manifest["exports"].as_array().unwrap();
    let mut seen = std::collections::BTreeSet::new();
    for export in exports {
        let name = export.as_str().unwrap();
        assert!(seen.insert(name), "duplicate export {name}");
        assert!(by_name.contains_key(name), "export has no document: {name}");
    }
    assert_eq!(exports.len(), docs.len());
    assert!(manifest.get("implementations").is_none());
    for (name, expected) in [
        (
            "issue-reading.v1",
            vec!["#read-issue.v1", "#list-issues.v1"],
        ),
        ("issue-writing.v1", vec!["#update-issue.v1"]),
    ] {
        assert_eq!(by_name[name]["kind"], "Capability");
        assert_eq!(by_name[name]["spec"]["operations"], json!(expected));
        for operation in expected {
            assert_eq!(by_name[&operation[1..]]["kind"], "Operation");
        }
    }
}

#[test]
fn every_operation_schema_and_requirement_input_compiles() {
    let docs = contracts();
    for doc in docs.values() {
        match doc["kind"].as_str().unwrap() {
            "Operation" => {
                for field in ["inputSchema", "outputSchema"] {
                    compile(&doc["spec"][field]);
                }
            }
            "Requirement" => {
                for input in doc["spec"]["inputs"].as_object().unwrap().values() {
                    compile(&input["schema"]);
                }
            }
            "Capability" => (),
            other => panic!("unexpected contract kind {other}"),
        }
    }
}

#[test]
fn requirement_arguments_require_scope_and_offset_deadline() {
    let docs = contracts();
    let inputs = docs["issue-scope.v1"]["spec"]["inputs"]
        .as_object()
        .unwrap();
    assert_eq!(inputs.len(), 2);
    let mut properties = Map::new();
    let mut required = Vec::new();
    for (name, input) in inputs {
        properties.insert(name.clone(), input["schema"].clone());
        assert!(input.get("default").is_none());
        assert!(input.get("optional").is_none());
        required.push(name.clone());
    }
    let validator = compile(&json!({
        "type": "object", "properties": properties, "required": required,
        "additionalProperties": false
    }));
    for deadline in ["2030-01-02T03:04:05Z", "2030-01-02T08:34:05+05:30"] {
        assert!(validator.is_valid(&json!({"issues": ["SYN-123"], "deadline": deadline})));
    }
    let valid = json!({"issues": ["SYN-123"], "deadline": "2030-01-02T03:04:05Z"});
    for issues in [json!([]), json!(["SYN-123", "SYN-123"])] {
        let mut invalid = valid.clone();
        invalid["issues"] = issues;
        assert!(!validator.is_valid(&invalid));
    }
    for issue in [
        "syn-123",
        "SYN_123",
        "123",
        " SYN-123",
        "SYN-123\n",
        "SYN-123x",
    ] {
        let mut invalid = valid.clone();
        invalid["issues"] = json!([issue]);
        assert!(!validator.is_valid(&invalid), "accepted {issue:?}");
    }
    for deadline in ["2030-01-02", "2030-01-02T03:04:05", "2030-13-02T03:04:05Z"] {
        let mut invalid = valid.clone();
        invalid["deadline"] = json!(deadline);
        assert!(!validator.is_valid(&invalid), "accepted {deadline}");
    }
    for field in ["issues", "deadline"] {
        let mut invalid = valid.clone();
        invalid.as_object_mut().unwrap().remove(field);
        assert!(!validator.is_valid(&invalid));
    }
}

#[test]
fn operation_arguments_outputs_and_repeat_contracts() {
    let docs = contracts();
    let identifier = &docs["issue-scope.v1"]["spec"]["inputs"]["issues"]["schema"]["items"];
    let issue = json!({"identifier": "SYN-123", "title": "Synthetic issue",
        "description": null, "state": "Todo", "updatedAt": "2030-01-02T03:04:05Z"});
    let read = &docs["read-issue.v1"]["spec"];
    let list = &docs["list-issues.v1"]["spec"];
    let update = &docs["update-issue.v1"]["spec"];
    for operation in [read, list, update] {
        assert_ne!(operation["startsModelWork"], true);
        assert!(operation.get("deduplicates").is_none());
        assert_eq!(operation["openWorldHint"], true);
        let output = if operation == list {
            &operation["outputSchema"]["properties"]["issues"]["items"]
        } else {
            &operation["outputSchema"]["properties"]["issue"]
        };
        assert_eq!(&output["properties"]["identifier"], identifier);
        let validator = compile(output);
        assert!(validator.is_valid(&issue));
        for field in ["relations", "parent", "subIssues", "comments"] {
            let mut invalid = issue.clone();
            invalid[field] = json!([]);
            assert!(!validator.is_valid(&invalid));
        }
        for field in ["identifier", "title", "description", "state", "updatedAt"] {
            let mut invalid = issue.clone();
            invalid.as_object_mut().unwrap().remove(field);
            assert!(!validator.is_valid(&invalid));
        }
        if operation != list {
            assert_eq!(&operation["inputSchema"]["properties"]["issue"], identifier);
            assert!(compile(&operation["outputSchema"]).is_valid(&json!({"issue": issue})));
        }
    }
    for operation in [read, list] {
        assert!(operation.get("effect").is_none());
        assert_eq!(operation["idempotentHint"], true);
    }
    assert!(update["effect"].as_str().is_some_and(|s| !s.is_empty()));
    assert_eq!(update["idempotentHint"], false);
    assert_eq!(update["outputSchema"]["required"], json!(["issue"]));
    assert!(compile(&read["inputSchema"]).is_valid(&json!({"issue": "SYN-123"})));
    assert!(!compile(&read["inputSchema"]).is_valid(&json!({"issue": "syn-123"})));
    let list_input = compile(&list["inputSchema"]);
    assert!(list_input.is_valid(&json!({"limit": 2})));
    for invalid in [
        json!({}),
        json!({"limit": 0}),
        json!({"limit": 1, "cursor": "x"}),
    ] {
        assert!(!list_input.is_valid(&invalid));
    }
    let list_output = compile(&list["outputSchema"]);
    assert!(list_output.is_valid(&json!({"issues": []})));
    assert!(list_output.is_valid(&json!({"issues": [issue]})));
    for extra in ["count", "cursor"] {
        let mut invalid = json!({"issues": []});
        invalid[extra] = json!(0);
        assert!(!list_output.is_valid(&invalid));
    }
    let update_input = compile(&update["inputSchema"]);
    for valid in [
        json!({"issue": "SYN-123", "title": ""}),
        json!({"issue": "SYN-123", "description": null}),
        json!({"issue": "SYN-123", "title": "Edit", "description": "SYN-456 is text"}),
    ] {
        assert!(update_input.is_valid(&valid));
    }
    for invalid in [
        json!({"issue": "SYN-123"}),
        json!({"issue": "SYN-123", "title": "Edit", "team": "OTHER"}),
        json!({"issue": "SYN-123", "title": "Edit", "parent": "SYN-456"}),
    ] {
        assert!(!update_input.is_valid(&invalid));
    }
}
