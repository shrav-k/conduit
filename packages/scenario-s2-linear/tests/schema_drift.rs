const CORE: &str = "github.com/shrav-k/conduit/core";
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::{collections::BTreeMap, path::Path};

fn documents(path: &Path) -> Vec<Value> {
    serde_yaml::Deserializer::from_str(&std::fs::read_to_string(path).unwrap())
        .map(|d| Value::deserialize(d).unwrap())
        .collect()
}
fn convert(
    value: &Value,
    owner: Option<&str>,
    source: &BTreeMap<String, Value>,
    defs: &mut Map<String, Value>,
) -> Value {
    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|v| convert(v, owner, source, defs))
                .collect(),
        ),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| {
                    if key == "$ref"
                        && let Some(reference) = value.as_str()
                    {
                        if let Some(name) = reference.strip_prefix(&format!("{}#", CORE)) {
                            if !defs.contains_key(name) {
                                defs.insert(name.into(), json!({}));
                                let converted = convert(
                                    source
                                        .get(name)
                                        .unwrap_or_else(|| panic!("missing schema {name}")),
                                    Some(name),
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
                    (key.clone(), convert(value, owner, source, defs))
                })
                .collect(),
        ),
        _ => value.clone(),
    }
}
#[test]
fn package_documents_validate_against_spec() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec = root.join("../../spec/schemas");
    if !spec.exists() {
        eprintln!("SKIPPED manifest/document schema validation: spec/ absent");
        return;
    }
    let mut source = BTreeMap::new();
    for entry in std::fs::read_dir(&spec)
        .unwrap()
        .chain(std::fs::read_dir(spec.parent().unwrap().join("core")).unwrap())
    {
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
    for (file, name) in [
        ("conduit.yaml", "manifest.v1"),
        ("contracts.yaml", "document.v1"),
    ] {
        let mut defs = Map::new();
        let mut schema = convert(
            &json!({"$ref": format!("{CORE}#{name}")}),
            None,
            &source,
            &mut defs,
        );
        schema["$defs"] = Value::Object(defs);
        let validator = jsonschema::options()
            .with_draft(jsonschema::Draft::Draft202012)
            .should_validate_formats(true)
            .build(&schema)
            .unwrap();
        for document in documents(&root.join(file)) {
            assert!(
                validator.is_valid(&document),
                "package document does not validate: {file}"
            );
        }
    }
}

#[test]
fn bundled_operation_and_filter_schemas_match_pinned_contracts_when_present() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = root.join("../scenario-s2-linear-restrictions/contracts.yaml");
    if !source.exists() {
        eprintln!("SKIPPED contract schema comparison: pinned contracts source absent");
        return;
    }
    let mut expected = Map::new();
    for document in documents(&source) {
        if document["kind"] == "Operation" || document["kind"] == "Requirement" {
            expected.insert(
                document["metadata"]["name"].as_str().unwrap().into(),
                document["spec"].clone(),
            );
        }
    }
    assert!(
        Value::Object(expected) == *scenario_s2_linear::SCHEMAS,
        "bundled schemas differ from pinned contracts"
    );
    let manifest = documents(&root.join("conduit.yaml")).remove(0);
    let address = scenario_s2_linear::CONTRACTS;
    assert!(manifest["packages"][address]["version"] == "0.1.0");
    assert!(
        manifest["packages"][address]["digest"]
            == "sha256:6380ce0e6a7db56712f457fd3a56aa7a6802ce3e019c56a3433eda440feac43e"
    );
    let item = &manifest["implementations"]["linear"];
    assert!(item["command"] == json!(["bin/scenario-s2-linear"]));
    assert!(item.get("plain").is_none());
    assert!(item["supports"] == json!([scenario_s2_linear::REQUIREMENT]));
    let contract = documents(&root.join("contracts.yaml")).remove(0);
    assert!(contract["metadata"]["name"] == "linear-aware.v1");
    assert!(contract["spec"]["family"] == "Resource");
    assert!(contract["spec"]["schema"]["required"] == json!(["baseUrl", "apiKey"]));
    assert!(
        contract["spec"]["schema"]["properties"]["apiKey"]["$ref"]
            == format!("{CORE}#secret-reference.v1")
    );
    for operation in scenario_s2_linear::OPERATIONS {
        let reference = format!("{address}#{operation}");
        assert!(contract["spec"]["implementation"][&reference] == "#linear");
        assert!(
            item["implements"]
                .as_array()
                .unwrap()
                .contains(&json!(reference))
        );
    }
}
