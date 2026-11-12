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
        ("tests/fixtures/project/conduit.yaml", "manifest.v1"),
        ("tests/fixtures/project/templates.yaml", "document.v1"),
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
        for mut document in documents(&root.join(file)) {
            if file.contains("fixtures/project/conduit") {
                document["packages"][scenario_s4_hook::ADDRESS]["digest"] =
                    json!(format!("sha256:{}", "0".repeat(64)));
            }
            assert!(
                validator.is_valid(&document),
                "package document does not validate: {file}"
            );
        }
    }
}

#[test]
fn embedded_schemas_match_contracts_and_package_wiring() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let docs = documents(&root.join("contracts.yaml"));
    assert!(docs.len() == 6);
    for name in ["read-hook-ledger.v1", "set-blocked-by.v1"] {
        let doc = docs.iter().find(|d| d["metadata"]["name"] == name).unwrap();
        let mut expected = doc["spec"].clone();
        if name == "set-blocked-by.v1" {
            expected["inputSchema"]["properties"]["revision"] =
                json!({"type":"string","minLength":1});
        }
        assert!(expected == scenario_s4_hook::SCHEMAS[name]);
    }
    let manifest = documents(&root.join("conduit.yaml")).remove(0);
    for (item, command) in [("hook", "hook"), ("reader", "read")] {
        let item = &manifest["implementations"][item];
        assert!(item["command"] == json!(["bin/scenario-s4-hook", command]));
        for field in ["supports", "environment", "plain"] {
            assert!(item.get(field).is_none());
        }
    }
    let hook = docs
        .iter()
        .find(|d| d["metadata"]["name"] == "s4-hook.v1")
        .unwrap();
    assert!(hook["spec"]["family"] == "Resource");
    assert!(hook["spec"]["requires"] == json!([format!("{CORE}#setup-hook.v1")]));
    assert!(hook["spec"]["controls"] == json!(["#set-blocked-by.v1"]));
    assert!(hook["spec"]["implementation"][format!("{CORE}#setup-hook.v1")] == "#hook");
    assert!(hook["spec"]["implementation"]["#set-blocked-by.v1"] == "#hook");
}
