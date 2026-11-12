#![cfg(unix)]
use runtime_provider_kit::names;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};

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
    if !spec.exists() {
        eprintln!("SKIPPED package schema validation: spec/ is absent");
        return;
    }
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
    let item = &manifest["implementations"]["provider"];
    assert!(item.get("tools").is_none() && item["plain"] == false);
    assert!(item["implements"].as_array().unwrap().len() == 3);
    assert!(
        contract["spec"]["offers"]
            == json!([format!("{}#runtime-provider.v1", names::CORE_PACKAGE)])
    );
    for pointer in item["environment"].as_object().unwrap().values() {
        let field = pointer.as_str().unwrap().strip_prefix('/').unwrap();
        assert!(
            contract["spec"]["schema"]["properties"][field]
                .get("default")
                .is_some()
        );
    }
}
#[test]
fn deterministic_archive_bytes_modes_and_digest() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // The package binary's build/metadata subprocesses run sequentially. No other test
    // invokes Cargo; the provider binary is already built by this test invocation.
    let pack = || {
        let result = Command::new(env!("CARGO_BIN_EXE_package"))
            .output()
            .unwrap();
        assert!(result.status.success(), "package command failed");
        fs::read(root.join("dist/provider-local-process.tar.gz")).unwrap()
    };
    let first = pack();
    let second = pack();
    assert!(first == second, "archive bytes differ");
    let hash = format!("{:x}", Sha256::digest(&first));
    assert!(
        fs::read_to_string(root.join("dist/provider-local-process.tar.gz.sha256")).unwrap()
            == format!("{hash}\n")
    );
    assert!(first[4..8] == [0; 4]);
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(first.as_slice()));
    let mut names = Vec::new();
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let name = entry.path().unwrap().display().to_string();
        let header = entry.header();
        assert!(
            header.as_ustar().is_some()
                && header.uid().unwrap() == 0
                && header.gid().unwrap() == 0
                && header.mtime().unwrap() == 0
        );
        assert!(
            header.mode().unwrap()
                == if name.starts_with("bin/") {
                    0o755
                } else {
                    0o644
                }
        );
        let mut content = Vec::new();
        entry.read_to_end(&mut content).unwrap();
        if name.ends_with(".yaml") {
            assert!(content == fs::read(root.join(&name)).unwrap());
        }
        names.push(name);
    }
    assert!(
        names
            == [
                "bin/provider-local-process",
                "conduit.yaml",
                "contracts.yaml"
            ]
    );
}

#[test]
fn exported_type_template_passes_semantic_registration_rules() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo = root.join("../..");
    if !repo.join("spec").exists() {
        eprintln!("SKIPPED template semantic validation: spec/ is absent");
        return;
    }
    // Reuse the repository's semantic resolver: document schemas alone cannot catch
    // MAN-017. A script file also preserves the checker's interpreter fallback.
    let script = tempfile::NamedTempFile::new().unwrap();
    fs::write(script.path(), r#"
import copy, pathlib, sys
sys.path.insert(0, str(pathlib.Path(sys.argv[1]) / 'tools'))
import check_schemas as checker
root = pathlib.Path(sys.argv[2])
manifest = checker.yaml.safe_load((root / 'conduit.yaml').read_text())
contract = checker.yaml.safe_load((root / 'contracts.yaml').read_text())
address = manifest['address']
core = checker.yaml.safe_load((checker.ROOT / 'spec/names.yaml').read_text())['corePackage']
core_docs = {}
for path in (checker.ROOT / 'spec/core').glob('*.yaml'):
    for doc in checker.yaml.safe_load_all(path.read_text()):
        if isinstance(doc, dict) and 'kind' in doc:
            core_docs[doc['metadata']['name']] = doc
template = {'apiVersion': manifest['apiVersion'], 'kind': 'Resource',
            'metadata': {'name': 'local-runtime'},
            'spec': {'type': address + '#local-process.v1'},
            'config': {k: v['default'] for k, v in contract['spec']['schema']['properties'].items()}}
def rules(type_doc):
    return checker.apply_rules([template], False, core,
        {core: core_docs, address: {'local-process.v1': type_doc}},
        items={address: manifest['implementations']})
assert not rules(contract), 'exported Type template rejected'
for pointer in manifest['implementations']['provider']['environment'].values():
    field = pointer[1:]
    broken = copy.deepcopy(contract)
    broken['spec']['schema']['required'].remove(field)
    assert any(rule == 'item-place-required' and pointer in message
               for rule, message in rules(broken)), 'MAN-017 mutation was not rejected'
print('template semantic validation passed')
"#).unwrap();
    let result = Command::new("python3")
        .arg(script.path())
        .arg(repo)
        .arg(root)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "template semantic validation failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}
