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
fn output_schema_copy_matches_spec() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec");
    if !root.exists() {
        eprintln!("SKIPPED output schema drift comparison: spec/ absent");
        return;
    }
    let operation = documents(&root.join("core/operations.yaml"))
        .into_iter()
        .find(|doc| doc["kind"] == "Operation" && doc["metadata"]["name"] == "get-secret.v1")
        .expect("get-secret.v1 is present");
    assert!(operation["spec"]["outputSchema"] == provider_file::output_schema());
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
