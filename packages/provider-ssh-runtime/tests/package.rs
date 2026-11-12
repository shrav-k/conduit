#![cfg(unix)]
use runtime_provider_kit::names;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::{collections::BTreeMap, fs, path::Path};

#[test]
fn archive_contains_both_executables_and_is_byte_identical_twice() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let package = || {
        let result = std::process::Command::new(env!("CARGO_BIN_EXE_package"))
            .output()
            .unwrap();
        assert!(result.status.success(), "packaging failed");
        fs::read(root.join("dist/provider-ssh-runtime.tar.gz")).unwrap()
    };
    let first = package();
    assert!(
        first == package(),
        "archive bytes changed between packaging runs"
    );
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(first.as_slice()));
    let mut found = Vec::new();
    for entry in archive.entries().unwrap() {
        use std::io::Read;
        let mut entry = entry.unwrap();
        let name = entry.path().unwrap().to_string_lossy().into_owned();
        for (expected, binary) in [
            (
                "bin/provider-ssh-runtime",
                env!("CARGO_BIN_EXE_provider-ssh-runtime"),
            ),
            (
                "bin/provider-ssh-runtime-drain",
                env!("CARGO_BIN_EXE_provider-ssh-runtime-drain"),
            ),
        ] {
            if name == expected {
                assert_eq!(entry.header().mode().unwrap(), 0o755);
                let mut bytes = Vec::new();
                entry.read_to_end(&mut bytes).unwrap();
                assert!(
                    bytes == fs::read(binary).unwrap(),
                    "archive executable differs from built binary"
                );
                found.push(name.clone());
            }
        }
    }
    assert_eq!(found.len(), 2);
}

fn documents(path: &Path) -> Vec<Value> {
    serde_yaml::Deserializer::from_str(&fs::read_to_string(path).unwrap())
        .map(|doc| Value::deserialize(doc).unwrap())
        .collect()
}
fn localize(
    value: &Value,
    owner: &str,
    source: &BTreeMap<String, Value>,
    defs: &mut Map<String, Value>,
) -> Value {
    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|v| localize(v, owner, source, defs))
                .collect(),
        ),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| {
                    if key == "$ref"
                        && let Some(reference) = value.as_str()
                    {
                        if let Some(name) =
                            reference.strip_prefix(&format!("{}#", names::CORE_PACKAGE))
                        {
                            if !defs.contains_key(name) {
                                defs.insert(name.into(), json!({}));
                                let converted = localize(&source[name], name, source, defs);
                                defs.insert(name.into(), converted);
                            }
                            return (key.clone(), json!(format!("#/$defs/{name}")));
                        }
                        if let Some(local) = reference.strip_prefix('#') {
                            return (key.clone(), json!(format!("#/$defs/{owner}{local}")));
                        }
                    }
                    (key.clone(), localize(value, owner, source, defs))
                })
                .collect(),
        ),
        _ => value.clone(),
    }
}
#[test]
fn package_documents_validate_against_repository_schemas() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec = root.join("../../spec/schemas");
    assert!(spec.exists(), "repository schemas required");
    let mut source = BTreeMap::new();
    for entry in fs::read_dir(&spec)
        .unwrap()
        .chain(fs::read_dir(spec.with_file_name("core")).unwrap())
    {
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
    for (file, schema) in [
        ("conduit.yaml", "manifest.v1"),
        ("contracts.yaml", "document.v1"),
    ] {
        let mut defs = Map::new();
        defs.insert(schema.into(), json!({}));
        let converted = localize(&source[schema], schema, &source, &mut defs);
        defs.insert(schema.into(), converted);
        let schema = json!({"$ref":format!("#/$defs/{schema}"),"$defs":defs});
        let validator = jsonschema::options()
            .with_draft(jsonschema::Draft::Draft202012)
            .should_validate_formats(true)
            .build(&schema)
            .unwrap();
        for document in documents(&root.join(file)) {
            assert!(
                validator.is_valid(&document),
                "package document fails schema: {file}"
            );
        }
    }
    let manifest = &documents(&root.join("conduit.yaml"))[0];
    let contract = &documents(&root.join("contracts.yaml"))[0];
    assert_eq!(
        manifest["address"],
        "example.org/conduit/provider-ssh-runtime"
    );
    assert_eq!(manifest["exports"], json!(["ssh-runtime.v1"]));
    assert_eq!(manifest["implementations"]["provider"]["plain"], false);
    assert_eq!(
        contract["spec"]["offers"],
        json!([format!("{}#runtime-provider.v1", names::CORE_PACKAGE)])
    );
    assert!(contract["spec"].get("tools").is_none());
}

#[test]
fn every_item_environment_place_is_required_and_scalar_man_017() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = &documents(&root.join("conduit.yaml"))[0];
    let contract = &documents(&root.join("contracts.yaml"))[0];
    let schema = &contract["spec"]["schema"];
    for pointer in manifest["implementations"]["provider"]["environment"]
        .as_object()
        .unwrap()
        .values()
    {
        let pointer = pointer.as_str().unwrap();
        let name = pointer.strip_prefix('/').unwrap();
        assert!(!name.contains('/'), "this item binds root places only");
        assert!(
            schema["required"]
                .as_array()
                .unwrap()
                .contains(&json!(name)),
            "MAN-017: {name} required"
        );
        assert!(
            matches!(
                schema["properties"][name]["type"].as_str(),
                Some("string" | "integer" | "number" | "boolean")
            ),
            "MAN-017: {name} scalar"
        );
    }
    for name in [
        "systemdRun",
        "systemctl",
        "loginctl",
        "unitPrefix",
        "runLimit",
    ] {
        assert!(schema["properties"][name].get("default").is_some());
    }
    assert_eq!(schema["properties"]["runLimit"]["default"], 62);
    assert_eq!(
        manifest["implementations"]["provider"]["environment"]["SSH_RUNTIME_DURABLE_DIR"],
        "/durableDir"
    );
    assert_eq!(schema["properties"]["durableDir"]["pattern"], "^/");
    assert!(schema["properties"]["durableDir"].get("default").is_none());
}
