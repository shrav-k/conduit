use scenario_s2_linear_adapter::{BINARY, CONTRACTS, OPERATIONS, SCHEMAS};
use serde::Deserialize;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path, process::Command};

fn documents(path: &Path) -> Vec<Value> {
    serde_yaml::Deserializer::from_str(&fs::read_to_string(path).unwrap())
        .map(|doc| Value::deserialize(doc).unwrap())
        .collect()
}
fn local_refs(value: &mut Value, owner: &str) {
    match value {
        Value::Object(map) => {
            if let Some(reference) = map.get_mut("$ref").filter(|value| value.is_string()) {
                let raw = reference.as_str().unwrap();
                let rewritten =
                    if let Some(name) = raw.strip_prefix("github.com/shrav-k/conduit/core#") {
                        format!("#/$defs/{name}")
                    } else if let Some(pointer) = raw.strip_prefix("#/") {
                        format!("#/$defs/{owner}/{pointer}")
                    } else if raw == "#" {
                        format!("#/$defs/{owner}")
                    } else if let Some(name) = raw.strip_prefix('#') {
                        format!("#/$defs/{name}")
                    } else {
                        panic!("unexpected nonlocal specification reference")
                    };
                *reference = json!(rewritten);
            }
            for value in map.values_mut() {
                local_refs(value, owner);
            }
        }
        Value::Array(items) => {
            for value in items {
                local_refs(value, owner);
            }
        }
        _ => {}
    }
}
#[test]
fn package_documents_validate_and_pin_the_reading_contract() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec = root.join("../../spec");
    if !spec.exists() {
        eprintln!("SKIPPED package document validation: spec/ absent");
        return;
    }
    let mut defs = Map::new();
    for directory in [spec.join("schemas"), spec.join("core")] {
        for file in fs::read_dir(directory).unwrap() {
            let path = file.unwrap().path();
            if path.extension().is_none_or(|ext| ext != "yaml") {
                continue;
            }
            for doc in documents(&path) {
                if doc["kind"] == "Schema" {
                    let name = doc["metadata"]["name"].as_str().unwrap();
                    let mut schema = doc["spec"].clone();
                    local_refs(&mut schema, name);
                    defs.insert(name.into(), schema);
                }
            }
        }
    }
    for (file, schema) in [
        ("conduit.yaml", "manifest.v1"),
        ("contracts.yaml", "document.v1"),
    ] {
        let validator = jsonschema::options()
            .should_validate_formats(true)
            .build(&json!({"$ref":format!("#/$defs/{schema}"),"$defs":defs}))
            .unwrap();
        for document in documents(&root.join(file)) {
            assert!(
                validator.is_valid(&document),
                "package document violates specification schema"
            );
        }
    }
    let manifest = documents(&root.join("conduit.yaml")).remove(0);
    assert!(manifest["packages"][CONTRACTS]["version"] == "0.1.0");
    assert!(
        manifest["packages"][CONTRACTS]["digest"]
            == "sha256:6380ce0e6a7db56712f457fd3a56aa7a6802ce3e019c56a3433eda440feac43e"
    );
    assert!(
        manifest["implementations"]["adapter"]["implements"]
            .as_array()
            .unwrap()
            .len()
            == 2
    );
    assert!(
        manifest["implementations"]["adapter"]
            .get("plain")
            .is_none()
    );
    assert!(
        manifest["implementations"]["adapter"]["environment"]
            .as_object()
            .unwrap()
            .len()
            == 2
    );
}
#[test]
fn embedded_contracts_match_pinned_source() {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../scenario-s2-linear-restrictions/contracts.yaml");
    if !source.exists() {
        eprintln!("SKIPPED embedded contract comparison: contracts source absent");
        return;
    }
    for document in documents(&source) {
        if let Some(name) = document["metadata"]["name"]
            .as_str()
            .filter(|n| OPERATIONS.contains(n))
        {
            assert!(
                SCHEMAS[name] == document["spec"],
                "embedded operation differs from pinned contract"
            );
        }
    }
}
#[test]
fn archive_is_byte_identical_twice_with_fixed_metadata_and_only_runtime_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let archive_path = root.join(format!("dist/{BINARY}.tar.gz"));
    let pack = || {
        let output = Command::new(env!("CARGO_BIN_EXE_package"))
            .output()
            .unwrap();
        assert!(output.status.success(), "package builder failed");
        assert!(output.stderr.is_empty());
        output.stdout
    };
    let first = pack();
    let bytes = fs::read(&archive_path).unwrap();
    assert!(pack() == first);
    assert!(fs::read(&archive_path).unwrap() == bytes);
    let digest = format!("{:x}", Sha256::digest(&bytes));
    assert!(String::from_utf8(first).unwrap().trim() == format!("{BINARY}.tar.gz sha256:{digest}"));
    assert!(
        fs::read_to_string(archive_path.with_extension("gz.sha256")).unwrap()
            == format!("{digest}\n")
    );
    assert!(bytes[4..8] == [0; 4]);
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(bytes.as_slice()));
    let mut names = Vec::new();
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let name = entry.path().unwrap().to_str().unwrap().to_owned();
        let header = entry.header();
        assert!(header.as_ustar().is_some());
        assert!(header.mtime().unwrap() == 0);
        assert!(header.uid().unwrap() == 0 && header.gid().unwrap() == 0);
        assert!(
            header.mode().unwrap()
                == if name.starts_with("bin/") {
                    0o755
                } else {
                    0o644
                }
        );
        let mut data = Vec::new();
        entry.read_to_end(&mut data).unwrap();
        let expected = if name.starts_with("bin/") {
            fs::read(env!("CARGO_BIN_EXE_scenario-s2-linear-adapter")).unwrap()
        } else {
            fs::read(root.join(&name)).unwrap()
        };
        assert!(data == expected);
        names.push(name);
    }
    assert!(
        names
            == [
                format!("bin/{BINARY}"),
                "conduit.yaml".into(),
                "contracts.yaml".into(),
                "README.md".into()
            ]
    );
}
