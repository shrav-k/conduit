//! Offline boundary, matching the conformance runner's submitted-project route.
use crate::{Result, translate};
use conduit_core::{
    Context, Evidence, Input, Machine, Request, State,
    packages::{Project, Snapshot, SnapshotSource, bytes_digest, contract},
    schemas::{Environment, Scope},
};
use conduit_loader::{
    Specification, Syntax, read_file, read_project, read_submitted_files, read_text,
};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

pub const STAGES: [&str; 7] = ["t1", "t2", "t4", "t5", "t6", "t7-revoked", "t7"];
pub const STYLES: [&str; 2] = ["flow", "block"];

pub fn directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).into()
}

pub fn read(path: &Path) -> Result<Value> {
    let docs = read_file(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if docs.len() != 1 {
        return Err(format!("{}: expected one document", path.display()));
    }
    Ok(docs[0].root.to_json())
}

fn core_project(p: conduit_loader::Project) -> Project {
    Project {
        manifest: p.manifest.map(|d| d.root.to_json()),
        documents: p.documents.iter().map(|d| d.root.to_json()).collect(),
        directory: Some("file:///short-form-spike/".into()),
    }
}

fn allocation(start: usize) -> Evidence {
    Evidence {
        time: "2026-11-10T12:00:00Z".into(),
        ids: (start..start + 500)
            .map(|i| format!("00000000-0000-4000-8000-{i:012}"))
            .collect(),
        revisions: (start..start + 200).map(|i| format!("r{i}")).collect(),
        suffixes: (start..start + 200).map(|i| format!("{i:05}")).collect(),
    }
}

pub struct Harness {
    spec: Specification,
    context: Context,
    initial: State,
    snapshots: Vec<Snapshot>,
}

#[derive(Debug)]
pub struct Checked {
    pub manifest: Value,
    pub documents: Vec<Value>,
    pub objects: Vec<Value>,
}

impl Harness {
    pub fn new() -> Result<Self> {
        let root = directory().join("../..");
        let spec = Specification::read(&root.join("spec")).map_err(|e| e.to_string())?;
        let mut builtin =
            core_project(read_project(&root.join("spec/core"), &spec).map_err(|e| e.to_string())?);
        let mut paths = std::fs::read_dir(root.join("spec/schemas"))
            .map_err(|e| e.to_string())?
            .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>>>()?;
        paths.sort();
        for path in paths {
            if path.extension().is_some_and(|e| e == "yaml") {
                builtin.documents.extend(
                    read_file(&path)
                        .map_err(|e| e.to_string())?
                        .iter()
                        .map(|d| d.root.to_json()),
                );
            }
        }
        let contracts = builtin
            .documents
            .iter()
            .filter(|d| contract(d))
            .map(|d| (d["metadata"]["name"].as_str().unwrap().into(), d.clone()))
            .collect();
        let names = read(&root.join("spec/names.yaml"))?;
        let core = spec.names().core_package.clone();
        let context = Context {
            builtin: Some(builtin.clone()),
            core_package: core.clone(),
            meta_prefix: names["metaPrefix"]
                .as_str()
                .ok_or("missing metaPrefix")?
                .into(),
            templates: builtin
                .documents
                .iter()
                .filter(|d| !contract(d))
                .map(|d| (d["metadata"]["name"].as_str().unwrap().into(), d.clone()))
                .collect(),
            record_source: "urn:short-form-spike".into(),
            core_environment: Environment {
                scopes: BTreeMap::from([(
                    "core".into(),
                    Scope {
                        address: core,
                        version: None,
                        manifest: BTreeMap::new(),
                        contracts,
                        exports: None,
                    },
                )]),
            },
        };
        let mut first_evidence = allocation(1);
        // FST-002 / FST-007: three declared root Conduits and a fixed test domain.
        first_evidence.revisions.truncate(8);
        first_evidence.suffixes.truncate(3);
        let initial = conduit_core::step(
            &State::default(),
            Input::FirstStart {
                evidence: first_evidence,
                trust_domain: "short-form-spike.example".into(),
                machine: Machine {
                    hostname: "spike.example".into(),
                    labels: BTreeMap::from([
                        ("kubernetes.io/hostname".into(), "spike.example".into()),
                        ("kubernetes.io/os".into(), "darwin".into()),
                        ("kubernetes.io/arch".into(), "arm64".into()),
                    ]),
                    state: "reachable".into(),
                },
            },
            &context,
        )?
        .state;
        let fixture = root.join("conformance/fixtures/s1");
        let mut snapshots = vec![];
        for (package, archive) in [
            ("filesystem", "filesystem"),
            ("filesystem-0.2.0", "filesystem-0.2.0"),
            ("history", "history"),
            ("agents", "agents"),
        ] {
            let project = core_project(
                read_project(&fixture.join("packages").join(package), &spec)
                    .map_err(|e| e.to_string())?,
            );
            let manifest = project
                .manifest
                .as_ref()
                .ok_or("package manifest missing")?;
            let bytes = std::fs::read(fixture.join("archives").join(format!("{archive}.tar.gz")))
                .map_err(|e| e.to_string())?;
            snapshots.push(Snapshot {
                source: SnapshotSource::PreinstalledSnapshot,
                address: manifest["address"]
                    .as_str()
                    .ok_or("package address missing")?
                    .into(),
                version: manifest["version"]
                    .as_str()
                    .ok_or("package version missing")?
                    .into(),
                digest: bytes_digest(&bytes),
                project,
            });
        }
        Ok(Self {
            spec,
            context,
            initial,
            snapshots,
        })
    }

    pub fn check(&self, style: &str, task: &str) -> Result<Checked> {
        self.check_mode(style, task, false)
    }

    /// Diagnostic probe only: drops the unsupported alias, changing tool prefixes.
    pub fn without_as(&self, style: &str, task: &str) -> Result<Checked> {
        self.check_mode(style, task, true)
    }

    fn check_mode(&self, style: &str, task: &str, without_as: bool) -> Result<Checked> {
        let path = directory().join("states").join(style).join(task);
        let manifest = read(&path.join("conduit.yaml"))?;
        let document = read(&path.join("s1.yaml"))?;
        for (address, pin) in manifest["packages"].as_object().ok_or("missing pins")? {
            if !self.snapshots.iter().any(|s| {
                s.address == *address && pin["version"] == s.version && pin["digest"] == s.digest
            }) {
                return Err(format!(
                    "{style}/{task}: pin has no matching archive: {address}"
                ));
            }
        }
        let documents = translate(
            &document,
            manifest["apiVersion"].as_str().ok_or("missing version")?,
            &self.context.core_package,
        )?;
        let mut submitted = documents.clone();
        if without_as {
            for doc in &mut submitted {
                doc.as_object_mut().unwrap().remove("as");
            }
        }
        // Rendered documents re-enter the strict loader, not a direct core-only submission.
        let files = [
            (
                "conduit.json".into(),
                serde_json::to_vec(&manifest).map_err(|e| e.to_string())?,
            ),
            (
                "documents.json".into(),
                serde_json::to_vec(&submitted).map_err(|e| e.to_string())?,
            ),
        ];
        let project = core_project(
            read_submitted_files(&files, &self.spec)
                .map_err(|e| format!("{style}/{task}: loader: {e}"))?,
        );
        let root = self
            .initial
            .objects
            .iter()
            .find(|o| o["name"] == "root")
            .ok_or("no root")?;
        let management = self
            .initial
            .objects
            .iter()
            .find(|o| o["name"] == "management")
            .ok_or("no management")?;
        let conduit = self
            .initial
            .objects
            .iter()
            .find(|o| o["kind"] == "Conduit" && o["target"] == management["id"])
            .ok_or("no management Conduit")?;
        let transition = conduit_core::step(
            &self.initial,
            Input::RegisterProject {
                snapshots: self.snapshots.clone(),
                builtin: self.context.builtin.clone().unwrap(),
                project,
                evidence: allocation(1000),
                request: Request {
                    sender: root["id"].as_str().unwrap().into(),
                    caller: root["id"].as_str().unwrap().into(),
                    conduit: conduit["id"].as_str().unwrap().into(),
                    target: management["id"].as_str().unwrap().into(),
                    operation: format!("{}#register.v1", self.context.core_package),
                },
            },
            &self.context,
        )?;
        if transition.reply["accepted"] != true {
            return Err(format!("{style}/{task}: core: {}", transition.reply));
        }
        Ok(Checked {
            manifest,
            documents,
            objects: transition.state.objects,
        })
    }
}

pub fn by_name(documents: &[Value]) -> BTreeMap<String, Value> {
    documents
        .iter()
        .map(|d| (d["metadata"]["name"].as_str().unwrap().into(), d.clone()))
        .collect()
}

pub fn instance<'a>(checked: &'a Checked, n: &str) -> &'a Value {
    checked
        .objects
        .iter()
        .find(|o| o["name"] == n && o.get("spec").is_none())
        .unwrap()
}

fn end_id(value: &Value) -> &Value {
    value.get("id").unwrap_or(value)
}

/// Definition/access projection. Held resolver contexts and package pins are reported
/// separately: fresh registration is not the planner's preservation of existing pins.
pub fn stable_changes(old: &Checked, new: &Checked, task: &str) -> Result<Vec<String>> {
    let before = by_name(&old.documents);
    let after = by_name(&new.documents);
    if before.keys().ne(after.keys()) {
        return Err(format!("{task}: names changed"));
    }
    let changed = before
        .keys()
        .filter(|n| before[*n] != after[*n])
        .cloned()
        .collect::<Vec<_>>();
    if task == "t5" {
        if !changed.is_empty() {
            return Err("T5: canonical bodies changed".into());
        }
        let mut expected = old.manifest.clone();
        expected["packages"]["example.org/s1/filesystem"] =
            new.manifest["packages"]["example.org/s1/filesystem"].clone();
        if expected != new.manifest {
            return Err("T5: unrelated manifest change".into());
        }
    } else if task == "t6" {
        if changed != ["shared--t"] || old.manifest != new.manifest {
            return Err(format!("T6: unexpected canonical changes: {changed:?}"));
        }
        let mut expected = before["shared--t"].clone();
        expected["spec"]["config"]["root"] = Value::String("/lab/replacement".into());
        if expected != after["shared--t"] {
            return Err("T6: unrelated template change".into());
        }
    } else {
        return Err("stable_changes supports only T5/T6".into());
    }
    let project = |objects: &[Value]| -> BTreeMap<String, Value> {
        objects
            .iter()
            .map(|o| {
                let mut v = o.clone();
                for k in ["held", "packages"] {
                    v.as_object_mut().unwrap().remove(k);
                }
                (o["name"].as_str().unwrap().to_owned(), v)
            })
            .collect()
    };
    let a = project(&old.objects);
    let b = project(&new.objects);
    if a.keys().ne(b.keys()) {
        return Err(format!("{task}: stored names changed"));
    }
    let changed = a
        .keys()
        .filter(|n| a[*n] != b[*n])
        .cloned()
        .collect::<Vec<_>>();
    if changed != ["shared", "shared--t"] {
        return Err(format!(
            "{task}: unrelated resolved definition/access changes: {changed:?}"
        ));
    }
    let shared_before = instance(old, "shared");
    let shared_after = instance(new, "shared");
    for (n, checked, version) in [
        ("before", old, if task == "t5" { "2.1.0" } else { "2.2.0" }),
        ("after", new, "2.2.0"),
    ] {
        for resource in ["shared", "history"] {
            let r = instance(checked, resource);
            let selections = r["resolved_implementations"]
                .as_object()
                .ok_or("missing implementations")?;
            if selections.len() != 3
                || selections.values().any(|s| {
                    s["item"]["package"]["version"]
                        != if resource == "history" {
                            "2.1.0"
                        } else {
                            version
                        }
                })
            {
                return Err(format!(
                    "{task}/{n}/{resource}: unexpected implementation version"
                ));
            }
        }
    }
    if task == "t5" && shared_before["config"] != shared_after["config"] {
        return Err("T5 changed the root".into());
    }
    if task == "t6"
        && (shared_before["config"]["root"] != "/lab/shared"
            || shared_after["config"]["root"] != "/lab/replacement")
    {
        return Err("T6 did not replace the root".into());
    }
    for n in ["agent-a--project", "agent-b--project"] {
        if end_id(&instance(old, n)["target"]) != &shared_before["id"]
            || end_id(&instance(new, n)["target"]) != &shared_after["id"]
        {
            return Err(format!("{task}: {n} did not target shared"));
        }
    }
    Ok(changed)
}

/// Check the stored operations, independent of which template a grant used.
pub fn access(checked: &Checked, task: &str) -> Result<()> {
    for agent in ["a", "b"] {
        for local in ["project", "history"] {
            let n = format!("agent-{agent}--{local}");
            let found = checked.objects.iter().find(|o| o["name"] == n);
            let absent = (task == "t1" && local == "history")
                || (task == "t7-revoked" && agent == "a" && local == "project");
            if absent {
                if found.is_some() {
                    return Err(format!("{task}: unexpected {n}"));
                }
                continue;
            }
            let conduit = found.ok_or_else(|| format!("{task}: missing {n}"))?;
            let write = local == "history"
                || if ["t1", "t2"].contains(&task) {
                    agent == "b"
                } else {
                    agent == "a"
                };
            let operations = conduit["operations"]
                .as_array()
                .ok_or_else(|| format!("{n}: missing resolved operations: {conduit}"))?;
            let expected = ["file.read.v1", "directory.list.v1", "file.write.v1"];
            let mut actual = operations
                .iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect::<Vec<_>>();
            actual.sort();
            let mut expected = expected[..if write { 3 } else { 2 }]
                .iter()
                .map(|op| format!("example.org/s1/filesystem#{op}"))
                .collect::<Vec<_>>();
            expected.sort();
            if actual != expected {
                return Err(format!("{task}/{n}: unexpected operations: {operations:?}"));
            }
            let target = instance(
                checked,
                if local == "project" {
                    "shared"
                } else {
                    "history"
                },
            );
            let consumer = instance(checked, &format!("agent-{agent}"));
            if end_id(&conduit["target"]) != &target["id"]
                || end_id(&conduit["consumer"]) != &consumer["id"]
            {
                return Err(format!("{task}/{n}: wrong resolved ends"));
            }
        }
    }
    Ok(())
}

pub fn parse(text: &str) -> Result<Value> {
    let docs = read_text(Syntax::Yaml, text).map_err(|e| e.to_string())?;
    if docs.len() != 1 {
        return Err("expected one YAML document".into());
    }
    Ok(docs[0].to_json())
}

fn lines(text: &str) -> Vec<&str> {
    // Fixtures/ledgers contain no quoted # comment ambiguities or multiline strings.
    text.lines()
        .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        .collect()
}

/// LCS edits, charging max(deleted, inserted) per contiguous replacement hunk.
pub fn edits(old: &str, new: &str) -> usize {
    let a = lines(old);
    let b = lines(new);
    let mut lcs = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            lcs[i][j] = if a[i] == b[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }
    let (mut i, mut j, mut removed, mut added, mut count) = (0, 0, 0usize, 0usize, 0);
    while i < a.len() || j < b.len() {
        if i < a.len() && j < b.len() && a[i] == b[j] {
            count += removed.max(added);
            (removed, added) = (0, 0);
            i += 1;
            j += 1;
        } else if i < a.len() && (j == b.len() || lcs[i + 1][j] >= lcs[i][j + 1]) {
            removed += 1;
            i += 1;
        } else {
            added += 1;
            j += 1;
        }
    }
    count + removed.max(added)
}

pub fn counts(style: &str) -> Result<Vec<[usize; 3]>> {
    let mut previous = [String::new(), String::new()];
    let mut rows = vec![];
    for task in ["t1", "t2", "t3", "t4", "t5", "t6", "t7"] {
        let mut count = 0;
        let mut digests = 0;
        if task != "t3" {
            for stage in if task == "t7" {
                vec!["t7-revoked", "t7"]
            } else {
                vec![task]
            } {
                for (i, file) in ["conduit.yaml", "s1.yaml"].iter().enumerate() {
                    let current = std::fs::read_to_string(
                        directory()
                            .join("states")
                            .join(style)
                            .join(stage)
                            .join(file),
                    )
                    .map_err(|e| e.to_string())?;
                    count += edits(&previous[i], &current);
                    let digest_lines = |s: &str| {
                        s.lines()
                            .filter(|l| l.contains("digest:"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    };
                    digests += edits(&digest_lines(&previous[i]), &digest_lines(&current));
                    previous[i] = current;
                }
            }
        }
        let commands =
            std::fs::read_to_string(directory().join("commands").join(format!("{task}.txt")))
                .map_err(|e| e.to_string())?;
        rows.push([count, lines(&commands).len(), digests]);
    }
    Ok(rows)
}
