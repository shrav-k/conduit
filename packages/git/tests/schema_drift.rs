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
fn contracts_match_the_capability_and_advertised_tools() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = documents(&root.join("conduit.yaml")).remove(0);
    let contracts = documents(&root.join("contracts.yaml"));
    let names: Vec<_> = contracts
        .iter()
        .map(|d| d["metadata"]["name"].as_str().unwrap())
        .collect();
    assert_eq!(manifest["exports"], json!(names));
    assert_eq!(manifest["address"], conduit_git::ADDRESS);
    assert_eq!(manifest["version"], env!("CARGO_PKG_VERSION"));
    let implementation = &manifest["implementations"]["server"];
    assert_eq!(implementation["command"], json!(["bin/conduit-git"]));
    assert!(implementation.get("plain").is_none());
    let of = |name: &str| {
        contracts
            .iter()
            .find(|d| d["metadata"]["name"] == name)
            .unwrap()
    };
    let kind = &of("working-copy.v1")["spec"];
    assert_eq!(kind["offers"], json!(["#copy.v1"]));
    assert_eq!(kind["records"], json!({"inputs":true,"outputs":false}));
    let hooks = [
        format!("{CORE}#setup-hook.v1"),
        format!("{CORE}#retire-hook.v1"),
    ];
    assert_eq!(kind["requires"], json!(hooks));
    let ops = [
        "#file.read.v1",
        "#directory.list.v1",
        "#file.write.v1",
        "#changes.v1",
        "#move.v1",
    ];
    assert_eq!(of("copy.v1")["spec"]["operations"], json!(ops));
    assert_eq!(
        implementation["implements"],
        json!(
            hooks
                .iter()
                .map(String::as_str)
                .chain(ops)
                .collect::<Vec<_>>()
        )
    );
    let mapping = kind["implementation"].as_object().unwrap();
    assert_eq!(mapping.len(), 7);
    for name in hooks.iter().map(String::as_str).chain(ops) {
        assert_eq!(mapping[name], "#server");
    }
    for (name, operations) in [
        (
            "read",
            json!(["#file.read.v1", "#directory.list.v1", "#changes.v1"]),
        ),
        (
            "read-write",
            json!([
                "#file.read.v1",
                "#directory.list.v1",
                "#changes.v1",
                "#file.write.v1"
            ]),
        ),
        (
            "manage",
            json!([
                "#file.read.v1",
                "#directory.list.v1",
                "#changes.v1",
                "#file.write.v1",
                "#move.v1"
            ]),
        ),
    ] {
        assert_eq!(of(name)["kind"], "Conduit");
        assert_eq!(
            of(name)["spec"],
            json!({"capability":"#copy.v1","operations":operations})
        );
    }
    for op in ops {
        let name = op.trim_start_matches('#');
        let operation = &of(name)["spec"];
        let (input, output) = conduit_git::schemas(name).unwrap();
        assert_eq!(operation["inputSchema"], input);
        assert_eq!(operation.get("outputSchema"), output.as_ref());
        let validator = jsonschema::validator_for(&input).unwrap();
        if let Some(path) = &input["properties"].get("path") {
            let path = jsonschema::validator_for(path).unwrap();
            for value in [".", "dir/file", "name space", ".gitignore", "a\nb/#file"] {
                assert!(path.is_valid(&json!(value)), "{value:?}");
            }
            for value in [
                "",
                "..",
                "../file",
                "/abs",
                "a//b",
                "a/",
                "a/.",
                "a/../b",
                ".git",
                "a/.GiT/file",
            ] {
                assert!(!path.is_valid(&json!(value)), "{value:?}");
            }
        } else if name == "move.v1" {
            let valid = json!({"commit":"a".repeat(40),"expectedBase":"b".repeat(40),
                "expectedPatchSha256":"c".repeat(64),"patch":""});
            assert!(validator.is_valid(&valid));
            assert!(!validator.is_valid(&json!({})));
            let mut invalid = valid;
            invalid["extra"] = json!(true);
            assert!(!validator.is_valid(&invalid));
        } else {
            assert!(validator.is_valid(&json!({})));
            assert!(!validator.is_valid(&json!({"extra":true})));
        }
    }
}
