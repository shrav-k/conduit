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

#[tokio::test]
async fn pull_request_exports_and_tools_match_the_contracts() {
    use conduit_github::{ADDRESS, TOKEN_VARIABLE};
    use rmcp::{ServiceExt, transport::async_rw::AsyncRwTransport};
    use std::{process::Stdio, time::Duration};
    use tokio::{process::Command, time::timeout};
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = documents(&here.join("conduit.yaml")).remove(0);
    let contracts = documents(&here.join("contracts.yaml"));
    let named = |name| {
        contracts
            .iter()
            .find(|d| d["metadata"]["name"] == name)
            .unwrap_or_else(|| panic!("missing export {name}"))
    };
    assert_eq!(manifest["version"], "0.2.0");
    assert_eq!(manifest["address"], ADDRESS);
    let names = json!(["#pull-request.create.v1", "#pull-request.merge.v1"]);
    assert_eq!(
        manifest["implementations"]["writer"],
        json!({
            "implements":names,"command":["bin/conduit-github","--write"],
            "environment":{TOKEN_VARIABLE:"/token"}
        })
    );
    let ty = &named("pull-requests.v1")["spec"];
    assert_eq!(ty["family"], "Resource");
    assert_eq!(ty["offers"], json!(["#pull-request-writing.v1"]));
    assert_eq!(ty["requires"], json!([]));
    assert!(ty.get("controls").is_none());
    assert_eq!(ty["records"], json!({"inputs":true,"outputs":true}));
    assert_eq!(
        ty["schema"]["required"],
        json!(["repository", "base", "token"])
    );
    assert_eq!(ty["schema"]["additionalProperties"], false);
    assert_eq!(
        ty["schema"]["properties"]["repository"],
        named("repository.v1")["spec"]["schema"]["properties"]["repository"]
    );
    assert_eq!(
        ty["schema"]["properties"]["token"],
        json!({"$ref":format!("{CORE}#secret-reference.v1")})
    );
    assert_eq!(
        named("pull-request-writing.v1")["spec"]["operations"],
        names
    );
    assert_eq!(named("write")["kind"], "Conduit");
    assert_eq!(
        named("write")["spec"],
        json!({"capability":"#pull-request-writing.v1","operations":names})
    );
    let branch = &ty["schema"]["properties"]["base"];
    let validator = jsonschema::validator_for(branch).unwrap();
    for good in ["main", "topic/change", "a_b-1.2", "a.locked"] {
        assert!(validator.is_valid(&json!(good)));
    }
    for bad in [
        "", "-a", "/a", ".a", "a/", "a.", "a.lock", "a..b", "a//b", "a/.b", "a.lock/b", "é",
    ] {
        assert!(!validator.is_valid(&json!(bad)), "invalid branch accepted");
    }
    assert_eq!(
        named("pull-request.create.v1")["spec"]["inputSchema"]["properties"]["head"],
        *branch
    );
    let create = &named("pull-request.create.v1")["spec"];
    let merge = &named("pull-request.merge.v1")["spec"];
    assert_eq!(
        create["inputSchema"]["required"],
        json!(["head", "title", "body"])
    );
    assert_eq!(
        merge["inputSchema"]["required"],
        json!(["number", "expectedHead", "title", "message"])
    );
    assert_eq!(create["outputSchema"]["required"], json!(["number", "url"]));
    assert_eq!(merge["outputSchema"]["required"], json!(["sha"]));
    assert_eq!(
        create["outputSchema"]["properties"]["number"],
        json!({"type":"integer","minimum":1})
    );
    assert_eq!(
        create["outputSchema"]["properties"]["url"],
        json!({"type":"string","pattern":"^https://github[.]com/"})
    );
    assert_eq!(
        merge["inputSchema"]["properties"]["expectedHead"],
        json!({"type":"string","pattern":"^[0-9a-f]{40}$"})
    );
    assert_eq!(
        merge["outputSchema"]["properties"]["sha"],
        merge["inputSchema"]["properties"]["expectedHead"]
    );
    for (operation, limit, text) in [(create, 256, "body"), (merge, 1024, "message")] {
        assert_eq!(
            operation["inputSchema"]["properties"]["title"]["minLength"],
            1
        );
        assert_eq!(
            operation["inputSchema"]["properties"]["title"]["maxLength"],
            limit
        );
        assert_eq!(
            operation["inputSchema"]["properties"][text],
            json!({"type":"string","maxLength":65536})
        );
        let title =
            jsonschema::validator_for(&operation["inputSchema"]["properties"]["title"]).unwrap();
        assert!(!title.is_valid(&json!("bad\ntitle")));
        assert!(!title.is_valid(&json!("bad\rtitle")));
        assert!(!title.is_valid(&json!("")));
        assert!(!title.is_valid(&json!("x".repeat(limit as usize + 1))));
        assert!(title.is_valid(&json!("é".repeat(limit as usize))));
    }
    let temp = tempfile::tempdir().unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_conduit-github"))
        .arg("--write")
        .env_clear()
        .env("HOME", temp.path())
        .env("PATH", temp.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let wait = Duration::from_secs(5);
    let service = timeout(
        wait,
        ().serve(AsyncRwTransport::new_client(
            child.stdout.take().unwrap(),
            child.stdin.take().unwrap(),
        )),
    )
    .await
    .unwrap()
    .unwrap();
    let tools = timeout(wait, service.list_all_tools())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(tools.len(), 2);
    for (index, tool) in tools.iter().enumerate() {
        let spec = &named(tool.name.as_ref())["spec"];
        assert_eq!(ty["implementation"][format!("#{}", tool.name)], "#writer");
        assert_eq!(
            serde_json::to_value(&tool.input_schema).unwrap(),
            spec["inputSchema"]
        );
        assert_eq!(
            serde_json::to_value(&tool.output_schema).unwrap(),
            spec["outputSchema"]
        );
        assert_eq!(spec["idempotentHint"], index == 1);
        assert_eq!(spec["destructiveHint"], false);
        assert_eq!(spec["openWorldHint"], true);
        assert!(spec["effect"].as_str().is_some_and(|s| !s.is_empty()));
        assert!(spec.get("deduplicates").is_none());
        assert!(spec.get("reservedToRoot").is_none());
        assert_eq!(spec["inputSchema"]["additionalProperties"], false);
        assert_eq!(spec["outputSchema"]["additionalProperties"], false);
    }
    timeout(wait, service.cancel()).await.unwrap().unwrap();
    timeout(wait, child.wait()).await.unwrap().unwrap();
}
