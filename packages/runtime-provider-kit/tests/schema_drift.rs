use runtime_provider_kit::{names, schemas};
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
                    if key == "$ref" {
                        let reference = value.as_str().unwrap();
                        if let Some(name) =
                            reference.strip_prefix(&format!("{}#", names::CORE_PACKAGE))
                        {
                            if !defs.contains_key(name) {
                                defs.insert(name.into(), json!({}));
                                let converted = convert(&source[name], Some(name), source, defs);
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
        // The library also builds/tests as an independently copied crate. Repository CI runs
        // this assertion with spec present; standalone consumers have nothing to compare.
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
    let assigned: Value =
        serde_yaml::from_str(&std::fs::read_to_string(root.join("names.yaml")).unwrap()).unwrap();
    for (key, value) in [
        ("corePackage", names::CORE_PACKAGE),
        ("metaPrefix", names::META_PREFIX),
        ("engineUrlVariable", names::ENGINE_URL_VARIABLE),
        ("identityDirVariable", names::IDENTITY_DIR_VARIABLE),
        ("stateDirVariable", names::STATE_DIR_VARIABLE),
    ] {
        assert!(assigned[key] == value, "assigned name drifted");
    }
    let files = schemas::identity_file_names();
    for key in [
        "x509BundleFile",
        "jwtBundleFile",
        "engineIdFile",
        "tokenFile",
        "certificateFile",
        "keyFile",
    ] {
        assert!(files.contains(&assigned[key].as_str().unwrap()));
    }
    assert!(files.len() == 6);
}
