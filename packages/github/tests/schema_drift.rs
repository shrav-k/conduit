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
fn repository_exports_and_tools_match_the_contracts() {
    use conduit_github::{ADDRESS, BINARY, OPERATIONS, TOKEN_VARIABLE, output_schema};
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = documents(&here.join("conduit.yaml")).remove(0);
    let contracts = documents(&here.join("contracts.yaml"));
    let named = |name| {
        contracts
            .iter()
            .find(|doc| doc["metadata"]["name"] == name)
            .unwrap()
    };
    let operations = OPERATIONS.map(|op| format!("#{}", op.name));
    let exports: Vec<_> = contracts
        .iter()
        .map(|doc| doc["metadata"]["name"].as_str().unwrap())
        .collect();
    assert_eq!(manifest["address"], ADDRESS);
    assert_eq!(manifest["exports"], json!(exports));
    let front = &manifest["implementations"]["front"];
    assert_eq!(front["command"], json!([format!("bin/{BINARY}")]));
    assert_eq!(front["implements"], json!(operations));
    assert_eq!(front["environment"], json!({TOKEN_VARIABLE:"/token"}));
    assert!(front.get("plain").is_none() && front.get("supports").is_none());
    let repository = &named("repository.v1")["spec"];
    assert_eq!(repository["family"], "Resource");
    assert_eq!(repository["offers"], json!(["#repository-reading.v1"]));
    assert_eq!(
        repository["records"],
        json!({"inputs":true,"outputs":false})
    );
    assert!(repository.get("controls").is_none());
    assert_eq!(
        repository["schema"]["required"],
        json!(["repository", "token"])
    );
    assert_eq!(repository["schema"]["additionalProperties"], false);
    assert_eq!(
        repository["schema"]["properties"]["token"]["$ref"],
        format!("{CORE}#secret-reference.v1")
    );
    let names = json!(operations);
    assert_eq!(named("repository-reading.v1")["spec"]["operations"], names);
    let read = named("read");
    assert_eq!(read["kind"], "Conduit");
    assert_eq!(
        read["spec"],
        json!({"capability":"#repository-reading.v1","operations":names})
    );
    for op in OPERATIONS {
        let contract = &named(op.name)["spec"];
        assert_eq!(
            repository["implementation"][format!("#{}", op.name)],
            "#front"
        );
        assert_eq!(contract["inputSchema"], op.input_schema());
        assert_eq!(contract["outputSchema"], output_schema());
        assert_eq!(contract["idempotentHint"], true);
        assert_eq!(contract["openWorldHint"], true);
        assert!(contract.get("effect").is_none());
    }
}
