use provider_local_directory::schemas;
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
fn all_copies_match_spec_and_every_reference_is_local() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec");
    if !root.exists() {
        eprintln!("SKIPPED schema drift comparison: spec/ absent");
        return;
    }
    let mut source = BTreeMap::new();
    for entry in std::fs::read_dir(root.join("schemas")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "yaml") {
            for doc in documents(&path) {
                if doc["kind"] == "Schema" {
                    source.insert(
                        doc["metadata"]["name"].as_str().unwrap().into(),
                        doc["spec"].clone(),
                    );
                }
            }
        }
    }
    let mut operations = Map::new();
    for doc in documents(&root.join("core/operations.yaml")) {
        if let Some(name) = doc["metadata"]["name"]
            .as_str()
            .filter(|n| schemas::OPERATIONS.contains(n))
        {
            let schemas = doc["spec"]
                .as_object()
                .unwrap()
                .iter()
                .filter(|(k, _)| k.as_str() == "inputSchema" || k.as_str() == "outputSchema")
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            operations.insert(name.into(), Value::Object(schemas));
        }
    }
    let mut defs = Map::new();
    let operations = convert(&Value::Object(operations), None, &source, &mut defs);
    let expected = json!({"operations":operations,"definitions":defs});
    assert!(
        expected == *schemas::COPIES,
        "schema copies drifted from spec"
    );
    for name in schemas::OPERATIONS {
        for kind in ["inputSchema", "outputSchema"] {
            if let Some(schema) = schemas::schema(name, kind) {
                assert!(
                    jsonschema::options()
                        .with_draft(jsonschema::Draft::Draft202012)
                        .build(&schema)
                        .is_ok()
                );
            }
        }
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
fn package_contract_has_the_required_source_shape() {
    // These assertions run even in a standalone crate without spec/; the general
    // document schema alone does not enforce PRV-016's required schemes shape.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = documents(&root.join("conduit.yaml"));
    assert_eq!(manifest.len(), 1);
    let manifest = &manifest[0];
    assert_eq!(
        manifest["address"],
        "example.org/conduit/provider-local-directory"
    );
    assert_eq!(manifest["version"], "0.1.0");
    assert_eq!(manifest["exports"], json!(["local-directory.v1"]));
    assert_eq!(
        manifest["implementations"],
        json!({"provider":{
            "implements":[format!("{CORE}#fetch-package.v1")],
            "command":["bin/provider-local-directory"]
        }})
    );
    let contracts = documents(&root.join("contracts.yaml"));
    assert_eq!(contracts.len(), 1);
    let contract = &contracts[0];
    assert_eq!(contract["kind"], "Type");
    assert_eq!(contract["metadata"]["name"], "local-directory.v1");
    let spec = &contract["spec"];
    assert_eq!(spec["family"], "Resource");
    assert_eq!(spec["offers"], json!([format!("{CORE}#package-source.v1")]));
    assert_eq!(
        spec["implementation"],
        json!({format!("{CORE}#fetch-package.v1"):"#provider"})
    );
    assert_eq!(spec["records"], json!({"inputs":false,"outputs":false}));
    assert!(spec.get("controls").is_none());
    assert_eq!(
        spec["schema"],
        provider_local_directory::configuration_schema()
    );
    assert_eq!(spec["schema"]["required"], json!(["schemes", "root"]));
    assert_eq!(
        spec["schema"]["properties"]["schemes"],
        json!({"type":"array","minItems":1,"items":{"const":"file"}})
    );
    let validator = jsonschema::validator_for(&spec["schema"]).unwrap();
    assert!(validator.is_valid(&json!({"schemes":["file"],"root":"/archives"})));
    for invalid in [
        json!({"root":"/archives"}),
        json!({"schemes":[],"root":"/archives"}),
        json!({"schemes":["https"],"root":"/archives"}),
        json!({"schemes":["file"],"root":"relative"}),
        json!({"schemes":["file"],"root":"/archives","extra":true}),
    ] {
        assert!(!validator.is_valid(&invalid));
    }
}
