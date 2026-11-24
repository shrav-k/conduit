//! Reproduce repository-authored installation fixtures. Never fetches or runs a provider.
use flate2::{Compression, GzBuilder};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, io::Write, path::Path};

const NEW_CONVERSATION: &str = "new-conversation.v1";
const NEW_CONVERSATION_REF: &str = "#new-conversation.v1";

fn shared_login_type(name: &Value) -> bool {
    matches!(
        name.as_str(),
        Some(
            "codex-login.v1"
                | "codex.v3"
                | "codex.v4"
                | "codex.v5"
                | "account-home.v1"
                | "claude-code.v3"
                | "claude-code.v4"
                | "claude-code.v5"
        )
    )
}

fn history_contract(name: &Value) -> bool {
    matches!(
        name.as_str(),
        Some("history-reading.v1" | "history.v1" | "history")
    )
}

fn write_fixture(path: &Path, bytes: &[u8], check: bool) {
    if check {
        assert_eq!(fs::read(path).unwrap(), bytes, "{}", path.display());
    } else {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }
}
fn documents(path: &Path) -> Vec<Value> {
    conduit_loader::read_file(path)
        .unwrap()
        .into_iter()
        .map(|d| d.root.to_json())
        .collect()
}
fn json_bytes(value: &Value) -> Vec<u8> {
    let mut bytes = serde_json::to_vec_pretty(value).unwrap();
    bytes.push(b'\n');
    bytes
}

// Keep the historical release's conversation surface when reading the current copies.
fn agent_contracts_v020(path: &Path) -> Vec<Value> {
    let mut docs = documents(path);
    docs.retain(|d| {
        let name = &d["metadata"]["name"];
        d["kind"] != "Conduit"
            && (d["kind"] != "Operation" || name != NEW_CONVERSATION)
            && (d["kind"] != "Type" || !shared_login_type(name))
            && !history_contract(name)
    });
    for doc in &mut docs {
        match doc["kind"].as_str() {
            Some("Operation") if doc["metadata"]["name"] == "turn.v1" => {
                doc["spec"]["outputSchema"]["properties"]
                    .as_object_mut()
                    .unwrap()
                    .remove("structured");
            }
            Some("Capability") if doc["metadata"]["name"] == "conversation.v1" => {
                doc["spec"]["operations"]
                    .as_array_mut()
                    .unwrap()
                    .retain(|op| op != NEW_CONVERSATION_REF);
            }
            Some("Type") => {
                doc["spec"]["implementation"]
                    .as_object_mut()
                    .unwrap()
                    .remove(NEW_CONVERSATION_REF);
            }
            _ => {}
        }
    }
    docs
}

fn agent_fixtures(root: &Path, check: bool, generated: &mut BTreeMap<String, Vec<u8>>) {
    let fixtures = root.join("conformance/fixtures/agents");
    for name in ["agent-claude-code", "agent-codex"] {
        let package = fixtures.join(name);
        let entries = ["conduit.yaml", "contracts.yaml"]
            .into_iter()
            .map(|file| {
                (
                    file.to_string(),
                    (fs::read(package.join(file)).unwrap(), b'0', String::new()),
                )
            })
            .collect();
        write_fixture(&package.join("contracts.tar.gz"), &archive(entries), check);
    }

    // Registration selects an item that omits the account operation.
    let codex = fixtures.join("agent-codex");
    let mut manifest = documents(&codex.join("conduit.yaml")).remove(0);
    manifest["address"] = json!("example.org/conformance/codex-no-account-hook");
    manifest["version"] = json!("0.2.0");
    manifest["exports"].as_array_mut().unwrap().retain(|op| {
        op != NEW_CONVERSATION
            && op != "turn"
            && op != "conversation"
            && !shared_login_type(op)
            && !history_contract(op)
    });
    let mut wrapper = manifest["implementations"]["wrapper-v2"].take();
    let account = wrapper["implements"]
        .as_array()
        .unwrap()
        .iter()
        .find(|op| op.as_str().unwrap().ends_with("#account-hook.v1"))
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    wrapper["implements"]
        .as_array_mut()
        .unwrap()
        .retain(|op| op != &account && op != NEW_CONVERSATION_REF);
    let tools = wrapper["tools"].as_object_mut().unwrap();
    tools.remove(&account);
    tools.remove(NEW_CONVERSATION_REF);
    manifest["implementations"] = json!({"wrapper-v2": wrapper});
    let mut contracts = agent_contracts_v020(&codex.join("contracts.yaml"));
    let v1 = contracts
        .iter_mut()
        .find(|d| d["kind"] == "Type" && d["metadata"]["name"] == "codex.v1")
        .unwrap();
    // The synthetic manifest has no v1 wrapper for this unused default.
    v1["spec"].as_object_mut().unwrap().remove("implementation");
    let entries = BTreeMap::from([
        (
            "conduit.json".into(),
            (json_bytes(&manifest), b'0', String::new()),
        ),
        (
            "contracts.json".into(),
            (json_bytes(&json!(contracts)), b'0', String::new()),
        ),
    ]);
    generated.insert("codex-no-account-hook".into(), archive(entries));
}

fn release_fixtures(root: &Path, check: bool, generated: &mut BTreeMap<String, Vec<u8>>) {
    let packages = root.join("conformance/fixtures/s1/packages");
    let mut releases = vec![];
    for version in ["0.1.1", "0.2.1"] {
        let source = if version == "0.1.1" {
            "filesystem"
        } else {
            "filesystem-0.2.0"
        };
        let mut manifest = documents(&packages.join(source).join("conduit.yaml")).remove(0);
        manifest["version"] = json!(version);
        let mut docs = documents(&packages.join(source).join("contracts.yaml"));
        for (name, operations) in [
            ("read", vec!["#file.read.v1", "#directory.list.v1"]),
            (
                "read-write",
                vec!["#file.read.v1", "#directory.list.v1", "#file.write.v1"],
            ),
        ] {
            docs.push(json!({"apiVersion":manifest["apiVersion"],"kind":"Conduit","metadata":{"name":name},"spec":{"capability":"#directory.v1","operations":operations}}));
            manifest["exports"]
                .as_array_mut()
                .unwrap()
                .push(json!(name));
        }
        releases.push((format!("filesystem-{version}"), manifest, docs));
    }
    let mut manifest = documents(&packages.join("agents/conduit.yaml")).remove(0);
    manifest["version"] = json!("0.2.0");
    let mut docs = documents(&packages.join("agents/contracts.yaml"));
    // These fixtures declare the released schema and hooks for installation checks.
    // They have no wrapper items and cannot run those hooks.
    for name in ["claude-code.v2", "codex.v2"] {
        let package = name.strip_suffix(".v2").unwrap();
        let source = root.join(format!(
            "conformance/fixtures/agents/agent-{package}/contracts.yaml"
        ));
        let ty = documents(&source)
            .into_iter()
            .find(|d| d["kind"] == "Type" && d["metadata"]["name"] == name)
            .unwrap();
        docs.push(json!({"apiVersion":manifest["apiVersion"],"kind":"Type","metadata":{"name":name},"spec":{
            "family":"Agent","schema":ty["spec"]["schema"],"signIn":ty["spec"]["signIn"],
            "offers":["#conversation.v1"],"requires":ty["spec"]["requires"],"records":{"inputs":false,"outputs":false}
        }}));
        manifest["exports"]
            .as_array_mut()
            .unwrap()
            .push(json!(name));
    }
    // Reuse the native fixture's conversation contracts; no implementation is run.
    docs.extend(
        agent_contracts_v020(
            &root.join("conformance/fixtures/agents/agent-claude-code/contracts.yaml"),
        )
        .into_iter()
        .filter(|d| d["kind"] != "Type"),
    );
    docs.push(json!({"apiVersion":manifest["apiVersion"],"kind":"Conduit","metadata":{"name":"turn"},"spec":{"capability":"#conversation.v1","operations":["#turn.v1"]}}));
    manifest["exports"].as_array_mut().unwrap().extend([
        json!("conversation.v1"),
        json!("turn.v1"),
        json!("turn"),
    ]);
    releases.push(("agents-0.2.0".into(), manifest, docs));
    for ty in
        releases.last().unwrap().2.iter().filter(|d| {
            d["kind"] == "Type" && d["metadata"]["name"].as_str().unwrap().ends_with(".v2")
        })
    {
        let name = ty["metadata"]["name"].as_str().unwrap();
        let mut values = vec![
            json!({"account":"agent@example.org"}),
            json!({"account":null,"apiKey":{"kind":"Resource","name":"api-key"}}),
            json!({}),
            json!({"account":null}),
            json!({"account":"agent@example.org","apiKey":{"kind":"Resource","name":"api-key"}}),
            json!({"account":"agent@example.org","accountServices":true,"model":"chosen"}),
        ];
        values.push(json!({"account":"agent@example.org","peerMessaging":true}));
        let fixture = json!({"environment":{"scopes":{"main":{"address":"example.org/s1/agents","manifest":{},"contracts":{}}}},"scope":"main","schema":ty["spec"]["schema"],"samples":values.into_iter().map(|value|json!({"mode":"supplied","value":value})).collect::<Vec<_>>()});
        write_fixture(
            &root
                .join("conformance/language/packages/agents-v2-defaults")
                .join(format!("{name}-values.json")),
            &json_bytes(&fixture),
            check,
        );
    }

    // NAM-024 admission pairs, isolated from the published S1 fixtures and T5.
    let (_, manifest, docs) = releases[0].clone();
    for (label, default, segment) in [
        ("default", true, false),
        ("optional", false, false),
        ("segment", true, true),
    ] {
        let mut manifest = manifest.clone();
        manifest["version"] = json!("0.1.2");
        let mut docs = docs.clone();
        let ty = docs.iter_mut().find(|d| d["kind"] == "Type").unwrap();
        ty["spec"]["schema"]["properties"]["depth"] = if default {
            json!({"type":"integer","default":1})
        } else {
            json!({"type":"integer"})
        };
        if segment {
            ty["metadata"]["name"] = json!("local-directory.v2");
            // A new name supplements rather than overwrites the published Type.
            docs.push(
                releases[0]
                    .2
                    .iter()
                    .find(|d| d["kind"] == "Type")
                    .unwrap()
                    .clone(),
            );
            manifest["exports"]
                .as_array_mut()
                .unwrap()
                .push(json!("local-directory.v2"));
        }
        releases.push((format!("release-adds-{label}"), manifest, docs));
    }
    // Round 2: defaults are compared at reference use sites, with recursive
    // local and Schema-contract refs, and retained values use SCH-007 equality.
    let (_, mut base_manifest, mut base_docs) = releases[0].clone();
    base_manifest["exports"]
        .as_array_mut()
        .unwrap()
        .push(json!("settings.v1"));
    let schema = &mut base_docs.iter_mut().find(|d| d["kind"] == "Type").unwrap()["spec"]["schema"];
    schema["properties"]["enabled"] = json!({"type":"boolean","default":false});
    schema["properties"]["values"] = json!({"type":"object","default":{"count":1,"steps":[1,2]}});
    schema["properties"]["settings"] = json!({"$ref":"#settings.v1"});
    schema["$defs"] = json!({"settings":{"type":"object","properties":{
        "enabled":{"type":"boolean","default":false},
        "next":{"$ref":"#/$defs/settings"}
    }}});
    base_docs.push(json!({"apiVersion":base_manifest["apiVersion"],"kind":"Schema","metadata":{"name":"settings.v1"},"spec":{
        "type":"object","properties":{
            "enabled":{"type":"boolean","default":false},
            "nested":{"$ref":"#/$defs/subsettings"},
            "next":{"$ref":"#"}
        },
        "$defs":{"subsettings":{"type":"object","properties":{"depth":{"type":"integer","default":1}}}}
    }}));
    releases.push((
        "release-defaults-base".into(),
        base_manifest.clone(),
        base_docs.clone(),
    ));
    for label in [
        "referenced",
        "schema-contract",
        "recursive",
        "changed",
        "removed",
        "equal",
        "reordered",
    ] {
        let mut manifest = base_manifest.clone();
        manifest["version"] = json!("0.1.2");
        let mut docs = base_docs.clone();
        let schema = &mut docs.iter_mut().find(|d| d["kind"] == "Type").unwrap()["spec"]["schema"];
        match label {
            "referenced" => schema["properties"]["options"] = json!({"$ref":"#/$defs/settings"}),
            "schema-contract" => schema["properties"]["options"] = json!({"$ref":"#settings.v1"}),
            "recursive" => {
                let settings = docs
                    .iter_mut()
                    .find(|d| d["metadata"]["name"] == "settings.v1")
                    .unwrap();
                settings["spec"]["properties"]["alias"] = json!({"$ref":"#"});
            }
            "changed" => schema["properties"]["enabled"]["default"] = json!(true),
            "removed" => {
                schema["properties"]["enabled"]
                    .as_object_mut()
                    .unwrap()
                    .remove("default");
            }
            "equal" => {
                schema["properties"]["values"]["default"] = json!({"steps":[1.0,2.0],"count":1.0})
            }
            "reordered" => {
                schema["properties"]["values"]["default"] = json!({"count":1,"steps":[2,1]})
            }
            _ => unreachable!(),
        }
        releases.push((format!("release-defaults-{label}"), manifest, docs));
    }
    for (name, manifest, docs) in releases {
        let entries: BTreeMap<_, _> = [("conduit.json", manifest), ("contracts.json", json!(docs))]
            .into_iter()
            .map(|(file, value)| {
                let bytes = json_bytes(&value);
                write_fixture(&packages.join(&name).join(file), &bytes, check);
                (file.to_string(), (bytes, b'0', String::new()))
            })
            .collect();
        generated.insert(name, archive(entries));
    }
}

fn member(name: &str, data: &[u8], kind: u8, target: &str) -> Vec<u8> {
    member_with_mode(name, data, kind, target, 0o644)
}
fn member_with_mode(name: &str, data: &[u8], kind: u8, target: &str, mode: u32) -> Vec<u8> {
    let mut h = tar::Header::new_ustar();
    h.set_mode(mode);
    h.set_uid(0);
    h.set_gid(0);
    h.set_mtime(0);
    h.set_size(data.len() as u64);
    h.set_entry_type(tar::EntryType::new(kind));
    // Deliberately bypass set_path's traversal guard for rejection fixtures.
    h.as_mut_bytes()[..100].fill(0);
    h.as_mut_bytes()[..name.len()].copy_from_slice(name.as_bytes());
    if !target.is_empty() {
        h.set_link_name(target).unwrap();
    }
    h.set_cksum();
    let mut bytes = h.as_bytes().to_vec();
    bytes.extend(data);
    bytes.resize(bytes.len().div_ceil(512) * 512, 0);
    bytes
}
fn archive(mut members: BTreeMap<String, (Vec<u8>, u8, String)>) -> Vec<u8> {
    let mut tar = vec![];
    for (name, (data, kind, target)) in &mut members {
        tar.extend(member(name, data, *kind, target));
    }
    tar.extend([0; 1024]);
    gzip(&tar)
}
fn gzip(tar: &[u8]) -> Vec<u8> {
    let mut gz = GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(vec![], Compression::best());
    gz.write_all(tar).unwrap();
    gz.finish().unwrap()
}
fn secret_archives(root: &Path, check: bool) {
    for form in ["framed", "sectioned", "long-chain"] {
        let fixture = root.join(format!("conformance/language/sectioned/secret-{form}"));
        let mut manifest = documents(&fixture.join("conduit.json")).remove(0);
        manifest["version"] = json!("0.1.0");
        let files = BTreeMap::from([
            (
                "conduit.json".into(),
                (json_bytes(&manifest), b'0', String::new()),
            ),
            (
                "package.json".into(),
                (
                    fs::read(fixture.join("package.json")).unwrap(),
                    b'0',
                    String::new(),
                ),
            ),
        ]);
        let bytes = archive(files);
        write_fixture(&fixture.join("package.tar.gz"), &bytes, check);
        let recipe = json!({
            "name": "package.tar.gz",
            "address": manifest["address"],
            "version": manifest["version"],
            "digest": conduit_core::packages::bytes_digest(&bytes),
            "completion": {"status": "complete", "file": "package.tar.gz"},
        });
        write_fixture(&fixture.join("archive.json"), &json_bytes(&recipe), check);
    }
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let out = root.join("conformance/fixtures/install-archives");
    let check = std::env::args().any(|a| a == "--check");
    secret_archives(&root, check);
    if !check {
        fs::create_dir_all(&out).unwrap();
    }
    let package = root.join("conformance/fixtures/s1/packages/filesystem");
    let manifest = fs::read(package.join("conduit.yaml")).unwrap();
    let docs = fs::read(package.join("contracts.yaml")).unwrap();
    let base = BTreeMap::from([
        (
            "conduit.yaml".into(),
            (manifest.clone(), b'0', String::new()),
        ),
        ("contracts.yaml".into(), (docs.clone(), b'0', String::new())),
    ]);
    let mut generated = BTreeMap::new();
    agent_fixtures(&root, check, &mut generated);
    for (name, changed) in [
        (
            "wrong-address",
            String::from_utf8(manifest.clone())
                .unwrap()
                .replace("example.org/s1/filesystem", "example.org/s1/another"),
        ),
        (
            "wrong-version",
            String::from_utf8(manifest.clone())
                .unwrap()
                .replace("version: 0.1.0", "version: 0.2.0"),
        ),
        (
            "missing-dependency",
            String::from_utf8(manifest.clone()).unwrap()
                + "\npackages:\n  example.org/missing:\n    version: 1.0.0\n    digest: sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n",
        ),
    ] {
        let mut entries = base.clone();
        entries.get_mut("conduit.yaml").unwrap().0 = changed.into_bytes();
        generated.insert(name.to_string(), archive(entries));
    }
    // Reuse the actual document's apiVersion rather than writing an assigned name here.
    let api = String::from_utf8(docs)
        .unwrap()
        .lines()
        .find(|l| {
            l.split_once(':')
                .is_some_and(|(key, _)| key == "apiVersion")
        })
        .unwrap()
        .to_owned();
    for (suffix, sections, exports) in [
        (
            "templates",
            "templates:\n  pools:\n    catalog: {type: 'github.com/shrav-k/conduit/core#pool.v1'}\n",
            "exports: [catalog]\n",
        ),
        ("fleet", "fleet: maintainers\n", ""),
        ("agents", "agents: {}\n", ""),
    ] {
        let name = format!("sectioned-{suffix}");
        let manifest =
            format!("{api}\naddress: example.org/conformance/{name}\nversion: 0.1.0\n{exports}");
        generated.insert(
            name,
            archive(BTreeMap::from([
                (
                    "conduit.yaml".into(),
                    (manifest.into_bytes(), b'0', String::new()),
                ),
                (
                    "sections.yaml".into(),
                    (sections.as_bytes().to_vec(), b'0', String::new()),
                ),
            ])),
        );
    }
    let instance =
        format!("{api}\nkind: Pool\nmetadata:\n  name: unwanted\nfrom: unused\n").into_bytes();
    for (name, path, data, kind, target) in [
        ("instance", "instance.yaml", instance, b'0', ""),
        (
            "dot-entry",
            "._contracts.yaml",
            b"not a document\0\xff".to_vec(),
            b'0',
            "",
        ),
        ("traversal", "../escape", b"outside".to_vec(), b'0', ""),
        ("absolute", "/escape", b"outside".to_vec(), b'0', ""),
        ("link-escape", "escape", vec![], b'2', "../outside"),
        ("link-absolute", "escape", vec![], b'2', "/outside"),
        ("hardlink-escape", "escape", vec![], b'1', "../outside"),
        ("symlink", "linked.yaml", vec![], b'2', "contracts.yaml"),
        ("dangling", "missing.yaml", vec![], b'2', "absent"),
        ("link-cycle", "cycle", vec![], b'2', "cycle/next"),
        (
            "invalid-document",
            "invalid.yaml",
            b"kind: invented\n".to_vec(),
            b'0',
            "",
        ),
        (
            "nested-document",
            "nested/invalid.yaml",
            b"not a document\0\xff".to_vec(),
            b'0',
            "",
        ),
        (
            "program",
            "bin/server",
            b"repository-authored inert program bytes\n".to_vec(),
            b'0',
            "",
        ),
    ] {
        let mut entries = base.clone();
        entries.insert(path.into(), (data, kind, target.into()));
        // A root link to an existing document replaces it; avoid duplicate document names.
        if name == "symlink" {
            let contracts = entries.remove("contracts.yaml").unwrap();
            entries.insert("data/contracts".into(), contracts);
            entries.get_mut("linked.yaml").unwrap().2 = "data/contracts".into();
        }
        generated.insert(name.to_string(), archive(entries));
    }
    let nested = base
        .iter()
        .map(|(k, v)| (format!("package/{k}"), v.clone()))
        .collect();
    generated.insert("nested-manifest".into(), archive(nested));
    let mut indirect = base.clone();
    indirect.insert("a".into(), (vec![], b'2', ".".into()));
    indirect.insert("escape".into(), (vec![], b'2', "a/../outside".into()));
    generated.insert("indirect-link-escape".into(), archive(indirect));
    // Lookup must visit every directory, even when `..` later removes its name.
    for (name, prefix, kind) in [
        ("manifest-file-parent", "payload", Some(b'0')),
        ("manifest-missing-parent", "absent", None),
        ("manifest-directory-parent", "payload", Some(b'5')),
    ] {
        let mut entries = base.clone();
        let manifest = entries.remove("conduit.yaml").unwrap();
        entries.insert("manifest".into(), manifest);
        if let Some(kind) = kind {
            entries.insert(prefix.into(), (vec![], kind, String::new()));
        }
        entries.insert(
            "conduit.yaml".into(),
            (vec![], b'2', format!("{prefix}/../manifest")),
        );
        generated.insert(name.into(), archive(entries));
    }
    for expansions in [39, 40, 41] {
        let mut entries = base.clone();
        let manifest = entries.remove("conduit.yaml").unwrap();
        entries.insert("manifest".into(), manifest);
        entries.insert("conduit.yaml".into(), (vec![], b'2', "link-01".into()));
        for i in 1..expansions {
            let target = if i + 1 == expansions {
                "manifest".into()
            } else {
                format!("link-{:02}", i + 1)
            };
            entries.insert(format!("link-{i:02}"), (vec![], b'2', target));
        }
        generated.insert(format!("link-expansions-{expansions}"), archive(entries));
    }
    // A hard link to an earlier regular file supplies the manifest at the root.
    let hard = BTreeMap::from([
        ("a-manifest".into(), (manifest, b'0', String::new())),
        ("conduit.yaml".into(), (vec![], b'1', "a-manifest".into())),
        ("contracts.yaml".into(), base["contracts.yaml"].clone()),
    ]);
    generated.insert("hardlink".into(), archive(hard));
    generated.insert("not-gzip".into(), b"not a gzip archive".to_vec());
    generated.insert("not-tar".into(), gzip(b"not a tar archive"));
    for (name, linux_mode) in [
        ("platforms-missing", None),
        ("platforms-not-executable", Some(0o644)),
        ("platforms-complete", Some(0o755)),
    ] {
        let manifest = format!(
            "{api}\naddress: example.org/conformance/platforms\nversion: 1.0.0\nimplementations:\n  server:\n    implements: ['#probe.v1']\n    command: [bin/server]\n    platforms: [darwin/arm64, linux/arm64]\n"
        );
        let contract =
            format!("{api}\nkind: Operation\nmetadata: {{name: probe.v1}}\nspec: {{}}\n");
        let mut tar = member("conduit.yaml", manifest.as_bytes(), b'0', "");
        tar.extend(member("contracts.yaml", contract.as_bytes(), b'0', ""));
        tar.extend(member_with_mode(
            "platforms/darwin/arm64/bin/server",
            b"darwin fixture",
            b'0',
            "",
            0o755,
        ));
        if let Some(mode) = linux_mode {
            tar.extend(member_with_mode(
                "platforms/linux/arm64/bin/server",
                b"linux fixture",
                b'0',
                "",
                mode,
            ));
        }
        tar.extend([0; 1024]);
        generated.insert(name.into(), gzip(&tar));
    }
    release_fixtures(&root, check, &mut generated);
    for (name, bytes) in &generated {
        let path = out.join(format!("{name}.tar.gz"));
        if check {
            assert_eq!(fs::read(&path).unwrap(), *bytes, "{}", path.display());
        } else {
            fs::write(path, bytes).unwrap();
        }
        println!("{name}: {}", conduit_core::packages::bytes_digest(bytes));
    }
    let digests: BTreeMap<_, _> = generated
        .iter()
        .map(|(name, bytes)| {
            (
                format!("{name}.tar.gz"),
                conduit_core::packages::bytes_digest(bytes),
            )
        })
        .collect();
    let mut digest_bytes = serde_json::to_vec_pretty(&digests).unwrap();
    digest_bytes.push(b'\n');
    let digest_path = out.join("digests.json");
    if check {
        assert_eq!(fs::read(digest_path).unwrap(), digest_bytes);
    } else {
        fs::write(digest_path, digest_bytes).unwrap();
    }
    println!(
        "{} reproducible archive fixtures {}",
        generated.len(),
        if check { "verified" } else { "written" }
    );
}
