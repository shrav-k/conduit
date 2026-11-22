//! A83 Part E: the specification's examples through the operations a client calls.
use conduit_loader::{Specification, read_file, read_project};
use conduit_shell::{Config, MissingExecutor, Shell, boundary};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

struct Outcome {
    class: &'static str,
    reason: String,
    rules: BTreeSet<String>,
    messages: Vec<String>,
}

impl Outcome {
    fn read(
        set: &str,
        reply: conduit_shell::Result<Value>,
        prefix: &str,
        named: Option<&str>,
    ) -> Self {
        let reply = match reply {
            Ok(reply) if reply.get("error").is_none() => reply,
            result => {
                let error = match result {
                    Ok(reply) => reply["error"].to_string(),
                    Err(error) => error,
                };
                return Self {
                    class: "error",
                    reason: "error".into(),
                    rules: BTreeSet::new(),
                    messages: vec![error],
                };
            }
        };
        let meta = &reply["_meta"];
        let reason = meta
            .get(format!("{prefix}reason"))
            .or_else(|| meta.get(format!("{prefix}outcome")))
            .and_then(Value::as_str)
            .unwrap_or("-")
            .to_string();
        let diagnostics: Vec<_> = reply["structuredContent"]["diagnostics"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|d| d["level"] == "error")
            .collect();
        let rules: BTreeSet<_> = diagnostics
            .iter()
            .filter_map(|d| d["ruleId"].as_str().map(str::to_owned))
            .collect();
        let messages = diagnostics
            .iter()
            .map(|d| {
                d["message"]["text"]
                    .as_str()
                    .or_else(|| d["message"].as_str())
                    .map(str::to_owned)
                    .unwrap_or_else(|| d["message"].to_string())
            })
            .collect();
        let class = match reply["isError"].as_bool() {
            Some(false) => "accepted",
            Some(true) if reason == "invalid" && named.is_some_and(|r| rules.contains(r)) => {
                "named"
            }
            Some(true) => "other",
            None => panic!("{set}: the client result has no boolean isError: {reply}"),
        };
        Self {
            class,
            reason,
            rules,
            messages,
        }
    }

    fn line(&self, name: &str) -> String {
        let rules = if self.rules.is_empty() {
            "-".into()
        } else {
            self.rules.iter().cloned().collect::<Vec<_>>().join(",")
        };
        format!("{name} {} {} {rules}", self.class, self.reason)
    }
}

fn invoke(shell: &Shell, set: &str, tool: &str, arguments: Value, named: Option<&str>) -> Outcome {
    let state = shell
        .state()
        .unwrap_or_else(|e| panic!("{set}: reading the state before {tool} failed: {e}"));
    let root = state
        .objects
        .iter()
        .find(|o| o["kind"] == "Interface" && o["name"] == "root" && o.get("spec").is_none())
        .unwrap_or_else(|| panic!("{set}: no root Interface instance after first start"))["id"]
        .clone();
    Outcome::read(
        set,
        shell.submit(
            json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":tool,"arguments":arguments}}),
            &MissingExecutor,
        ),
        &shell.context.meta_prefix,
        named,
    )
}

fn fresh(set: &str, repository: &Path, store: &Path) -> Shell {
    let context = boundary::context(repository)
        .unwrap_or_else(|e| panic!("{set}: boundary::context of the repository failed: {e}"));
    let shell = Shell::open(store, context, Config::default())
        .unwrap_or_else(|e| panic!("{set}: Shell::open of a fresh store failed: {e}"));
    let reply = shell
        .submit(
            json!({"kind":"first-start","machine":{"hostname":"by-rule","labels":{"kubernetes.io/hostname":"by-rule","kubernetes.io/os":"linux","kubernetes.io/arch":"amd64"},"state":"reachable"}}),
            &MissingExecutor,
        )
        .unwrap_or_else(|e| panic!("{set}: first start failed: {e}"));
    assert!(
        reply.get("error").is_none(),
        "{set}: first start replied with an error: {}",
        reply["error"]
    );
    shell
}

fn archive(directory: &Path, name: &str, files: Vec<(&str, Vec<u8>)>) -> (PathBuf, String) {
    let gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut tar = tar::Builder::new(gzip);
    for (file, bytes) in files {
        let mut header = tar::Header::new_ustar();
        header.set_entry_type(tar::EntryType::Regular);
        header.set_size(bytes.len() as u64);
        header.set_mode(0o644);
        header.set_mtime(0);
        header.set_cksum();
        tar.append_data(&mut header, file, bytes.as_slice())
            .unwrap_or_else(|e| panic!("scaffold archive {name}: adding {file} failed: {e}"));
    }
    let bytes = tar
        .into_inner()
        .and_then(|gzip| gzip.finish())
        .unwrap_or_else(|e| panic!("scaffold archive {name}: finishing the gzip tar failed: {e}"));
    let digest = conduit_core::packages::bytes_digest(&bytes);
    let path = directory.join(name);
    fs::write(&path, bytes)
        .unwrap_or_else(|e| panic!("scaffold archive {name}: writing it failed: {e}"));
    (path, digest)
}

fn archives(
    repository: &Path,
    temp: &Path,
    specification: &Specification,
) -> [(PathBuf, String); 2] {
    let api = &specification.names().api_version;
    let common = archive(
        temp,
        "common.tar.gz",
        vec![
            ("conduit.json", serde_json::to_vec(&json!({"apiVersion":api,"address":"example.org/common","version":"2.0.1","exports":["timestamp.v1"]})).expect("a JSON value serializes")),
            ("contracts.json", serde_json::to_vec(&json!({"apiVersion":api,"kind":"Schema","metadata":{"name":"timestamp.v1"},"spec":{"type":"string","format":"date-time"}})).expect("a JSON value serializes")),
        ],
    );
    let source = repository.join("spec/schemas/examples/valid/files-package");
    let mut manifest = read_file(&source.join("conduit.yaml")).unwrap_or_else(|e| {
        panic!("scaffold archive files.tar.gz: read_file of valid/files-package/conduit.yaml failed: {e}")
    })[0]
        .root
        .to_json();
    manifest["packages"]["example.org/common"]["digest"] = json!(common.1);
    let files = archive(
        temp,
        "files.tar.gz",
        vec![
            (
                "conduit.json",
                serde_json::to_vec(&manifest).expect("a JSON value serializes"),
            ),
            (
                "contracts.yaml",
                fs::read(source.join("contracts.yaml")).unwrap_or_else(|e| {
                    panic!("scaffold archive files.tar.gz: reading valid/files-package/contracts.yaml failed: {e}")
                }),
            ),
        ],
    );
    [common, files]
}

fn install(shell: &Shell, set: &str, archive: &(PathBuf, String)) -> Outcome {
    let file = archive.0.file_name().map_or_else(
        || archive.0.display().to_string(),
        |file| file.to_string_lossy().into_owned(),
    );
    let arguments = boundary::install_arguments(&format!("{}#{}", archive.0.display(), archive.1))
        .unwrap_or_else(|e| panic!("{set}: install_arguments for {file} failed: {e}"));
    invoke(shell, set, "management.install.v1", arguments, None)
}

fn scaffold(shell: &Shell, set: &str, archives: &[(PathBuf, String); 2]) -> Option<Outcome> {
    for archive in archives {
        let mut outcome = install(shell, set, archive);
        if outcome.class != "accepted" {
            outcome.class = "scaffold";
            return Some(outcome);
        }
    }
    None
}

fn entries(examples: &Path, subdirectory: &str) -> Vec<PathBuf> {
    let mut paths: Vec<_> = fs::read_dir(examples.join(subdirectory))
        .unwrap_or_else(|e| panic!("listing examples/{subdirectory} failed: {e}"))
        .map(|entry| {
            entry
                .unwrap_or_else(|e| {
                    panic!("reading an entry of examples/{subdirectory} failed: {e}")
                })
                .path()
        })
        .collect();
    paths.sort();
    paths
}

#[test]
fn by_rule_examples_through_public_routes() {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let examples = repository.join("spec/schemas/examples");
    let specification = Specification::read(&repository.join("spec"))
        .unwrap_or_else(|e| panic!("reading spec/ failed: {e}"));
    let temp = tempfile::tempdir()
        .unwrap_or_else(|e| panic!("creating the temporary directory failed: {e}"));
    let archives = archives(&repository, temp.path(), &specification);
    let mut observed = BTreeMap::new();
    for path in entries(&examples, "invalid-by-rule") {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_else(|| panic!("invalid-by-rule: an entry has no UTF-8 name: {path:?}"));
        assert!(
            path.is_file(),
            "invalid-by-rule/{name}: not a file; every by-rule entry is one file"
        );
        let mut documents = Vec::new();
        let mut manifest = None;
        for document in read_file(&path).unwrap_or_else(|e| panic!("{name}: read_file failed: {e}"))
        {
            let value = document.root.to_json();
            if value.get("kind").is_none() {
                assert!(manifest.replace(value).is_none(), "{name}: two manifests");
            } else {
                documents.push(value);
            }
        }
        let needs_files = documents
            .iter()
            .any(|value| value.to_string().contains("example.org/files#"));
        let own_manifest = manifest.is_some();
        let manifest = manifest.unwrap_or_else(|| {
            let mut manifest = json!({"apiVersion":specification.names().api_version,"address":"example.org/by-rule"});
            if needs_files {
                manifest["packages"] = json!({"example.org/files":{"version":"0.3.0","digest":archives[1].1}});
            }
            manifest
        });
        let shell = fresh(name, &repository, &temp.path().join(name));
        let scaffold_failure = if !own_manifest && needs_files {
            scaffold(&shell, name, &archives)
        } else {
            None
        };
        let outcome = scaffold_failure.unwrap_or_else(|| {
            let mut parts = name.splitn(3, '-');
            let rule = match (parts.next(), parts.next()) {
                (Some(prefix), Some(number)) => format!("{prefix}-{number}"),
                _ => panic!("{name}: the file name does not begin with a rule ID (PREFIX-NUMBER-)"),
            };
            invoke(
                &shell,
                name,
                "management.register.v1",
                json!({"documents":documents,"manifest":manifest}),
                Some(&rule),
            )
        });
        observed.insert(name.to_string(), outcome);
    }

    let excluded = ["first-start", "project"];
    for path in entries(&examples, "valid") {
        if !path.is_dir() {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_else(|| panic!("valid: a directory has no UTF-8 name: {path:?}"));
        if excluded.contains(&name) {
            continue;
        }
        let set = format!("valid/{name}");
        assert!(
            [
                "equal-defaults",
                "files-package",
                "sectioned-project",
                "sectioned-package"
            ]
            .contains(&name),
            "{set}: directory is neither sent nor excluded"
        );
        let shell = fresh(&set, &repository, &temp.path().join(name));
        let outcome = if name == "files-package" {
            let mut common = install(&shell, &set, &archives[0]);
            if common.class == "accepted" {
                install(&shell, &set, &archives[1])
            } else {
                common.class = "scaffold";
                common
            }
        } else if name == "sectioned-package" {
            let api = &specification.names().api_version;
            let operations = ["ticket.list.v1", "ticket.assign.v1", "ticket.resolve.v1"];
            let exports = std::iter::once("queue.v1")
                .chain(operations)
                .collect::<Vec<_>>();
            let mut contracts = vec![json!({
                "apiVersion": api,
                "kind": "Capability",
                "metadata": {"name": "queue.v1"},
                "spec": {"operations": operations.map(|name| format!("#{name}"))},
            })];
            contracts.extend(operations.map(|name| {
                json!({
                    "apiVersion": api,
                    "kind": "Operation",
                    "metadata": {"name": name},
                    "spec": {"inputSchema": {"type": "object"}, "outputSchema": {"type": "object"}},
                })
            }));
            let dependency = archive(
                temp.path(),
                "ticket-queue.tar.gz",
                vec![
                    (
                        "conduit.json",
                        serde_json::to_vec(&json!({
                            "apiVersion": api,
                            "address": "example.org/ticket-queue",
                            "version": "0.1.0",
                            "exports": exports,
                        }))
                        .expect("a JSON value serializes"),
                    ),
                    (
                        "contracts.json",
                        serde_json::to_vec(&contracts).expect("JSON values serialize"),
                    ),
                ],
            );
            let mut prerequisite = install(&shell, &set, &dependency);
            if prerequisite.class == "accepted" {
                let mut manifest = read_file(&path.join("conduit.yaml")).unwrap()[0]
                    .root
                    .to_json();
                manifest["packages"]["example.org/ticket-queue"]["digest"] = json!(dependency.1);
                let package = archive(
                    temp.path(),
                    "sectioned.tar.gz",
                    vec![
                        ("conduit.json", serde_json::to_vec(&manifest).unwrap()),
                        (
                            "templates.yaml",
                            fs::read(path.join("templates.yaml")).unwrap(),
                        ),
                    ],
                );
                install(&shell, &set, &package)
            } else {
                prerequisite.class = "scaffold";
                prerequisite
            }
        } else {
            let project = read_project(&path, &specification)
                .unwrap_or_else(|e| panic!("{set}: read_project failed: {e}"));
            let mut manifest = project
                .manifest
                .unwrap_or_else(|| panic!("{set}: read_project gave no manifest"))
                .root
                .to_json();
            if name == "sectioned-project" {
                assert!(
                    scaffold(&shell, &set, &archives).is_none(),
                    "{set}: dependency install failed"
                );
                manifest["packages"]["example.org/files"]["digest"] = json!(archives[1].1);
            }
            invoke(
                &shell,
                &set,
                "management.register.v1",
                json!({"documents":project.documents.iter().map(|d|d.root.to_json()).collect::<Vec<_>>(),"manifest":manifest}),
                None,
            )
        };
        observed.insert(set, outcome);
    }

    let table: BTreeMap<_, _> = observed
        .iter()
        .map(|(name, outcome)| (name.clone(), outcome.line(name)))
        .collect();
    for line in table.values() {
        println!("{line}");
    }
    for class in ["named", "other", "scaffold", "accepted", "error"] {
        println!(
            "{class}: {}",
            observed.values().filter(|o| o.class == class).count()
        );
    }
    let canonical = temp
        .path()
        .canonicalize()
        .unwrap_or_else(|e| panic!("canonicalizing the temporary directory failed: {e}"));
    let canonical = canonical
        .to_str()
        .expect("the temporary directory's canonical path is UTF-8");
    let created = temp
        .path()
        .to_str()
        .expect("the temporary directory's path is UTF-8");
    for (name, outcome) in &observed {
        if ["other", "scaffold", "error"].contains(&outcome.class) {
            println!("{name} ({}):", outcome.class);
            for message in &outcome.messages {
                println!(
                    "  {}",
                    message.replace(canonical, "TMP").replace(created, "TMP")
                );
            }
        }
    }

    let expected_text = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/expected/by-rule-outcomes.txt"),
    )
    .unwrap_or_else(|e| panic!("reading tests/expected/by-rule-outcomes.txt failed: {e}"));
    let mut expected = BTreeMap::new();
    for line in expected_text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split_whitespace().collect();
        assert_eq!(
            fields.len(),
            4,
            "malformed expected line (want NAME CLASS REASON RULES): {line}"
        );
        assert!(
            expected
                .insert(fields[0].to_string(), line.to_string())
                .is_none(),
            "duplicate expected set in tests/expected/by-rule-outcomes.txt: {}",
            fields[0]
        );
    }
    let names: BTreeSet<_> = expected.keys().chain(table.keys()).collect();
    let differences: Vec<_> = names
        .into_iter()
        .filter(|name| expected.get(*name) != table.get(*name))
        .map(|name| {
            format!(
                "{name}: expected {}; observed {}",
                expected
                    .get(name)
                    .map(String::as_str)
                    .unwrap_or("<missing>"),
                table.get(name).map(String::as_str).unwrap_or("<missing>")
            )
        })
        .collect();
    assert!(
        differences.is_empty(),
        "outcomes differ from tests/expected/by-rule-outcomes.txt; a line changes only in the \
         pull request that changes what the engine does with that set:\n{}",
        differences.join("\n")
    );
}
