//! Resolving an immutable catalog of snapshots and packages installed from archive inputs.
use crate::{
    Decision, Result, decision,
    schemas::{Environment, Scope},
};
use fluent_uri::UriRef;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub manifest: Option<Value>,
    pub documents: Vec<Value>,
    /// An absolute directory URL supplied by the boundary; never consulted as a filesystem.
    pub directory: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub source: SnapshotSource,
    pub address: String,
    pub version: String,
    pub digest: String,
    pub project: Project,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SnapshotSource {
    PreinstalledSnapshot,
    InstalledArchive,
}

pub(crate) fn catalog_entries(state: &crate::State, snapshots: Vec<Snapshot>) -> Vec<Snapshot> {
    state
        .installed_packages
        .iter()
        .map(|p| p.package.clone())
        .chain(snapshots)
        .collect()
}

/// Owns copies. Installation publishes a new catalog entry atomically in the state.
#[derive(Clone, Debug)]
pub struct Catalog {
    builtin: Project,
    snapshots: Vec<Snapshot>,
    core_address: String,
}

pub struct Resolution {
    pub reply: Value,
    pub decisions: Vec<Decision>,
}

pub fn contract(document: &Value) -> bool {
    ["Schema", "Type", "Operation", "Capability", "Requirement"]
        .iter()
        .any(|k| document["kind"] == *k)
}
/// SUB-008: only contracts and the original manifest; UTF-8 ordering before RFC 8785.
pub fn project_digest(project: &Project) -> Result<String> {
    let mut documents: Vec<_> = project
        .documents
        .iter()
        .filter(|d| contract(d))
        .cloned()
        .collect();
    documents.sort_by(|a, b| {
        (a["kind"].as_str(), a["metadata"]["name"].as_str())
            .cmp(&(b["kind"].as_str(), b["metadata"]["name"].as_str()))
    });
    let bytes = serde_json_canonicalizer::to_vec(
        &json!({"documents":documents,"manifest":project.manifest}),
    )
    .map_err(|e| e.to_string())?;
    Ok(bytes_digest(&bytes))
}
pub fn bytes_digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
pub(crate) fn version_equal(a: &str, b: &str) -> bool {
    match (Version::parse(a), Version::parse(b)) {
        (Ok(a), Ok(b)) => a.cmp_precedence(&b).is_eq(),
        _ => false,
    }
}
fn diag(rule: &str, text: impl Into<String>) -> Value {
    json!({"ruleId":rule,"level":"error","message":{"text":text.into()}})
}
fn items<'a>(value: &'a Value, key: &str) -> impl Iterator<Item = (&'a String, &'a Value)> {
    value
        .get(key)
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
}
fn strings<'a>(value: &'a Value, key: &str) -> impl Iterator<Item = &'a str> {
    value
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
}
fn manifest(project: &Project) -> Value {
    project.manifest.clone().unwrap_or(json!({}))
}
pub(crate) fn index(project: &Project) -> BTreeMap<String, Value> {
    project
        .documents
        .iter()
        .filter_map(|d| {
            d["metadata"]["name"]
                .as_str()
                .map(|n| (n.into(), d.clone()))
        })
        .chain(
            items(&manifest(project), "implementations")
                .map(|(n, d)| (n.clone(), json!({"kind":"Implementation","spec":d}))),
        )
        .collect()
}

// Each part retains its reference scope and local-pointer root (SCH-015).
type DefaultParts = Vec<(String, Value, Value)>;

struct DefaultComparison<'a> {
    env: &'a Environment,
    core: &'a str,
    name: &'a str,
    diagnostics: Vec<Value>,
}
impl DefaultComparison<'_> {
    fn value(&mut self, before: Option<&Value>, after: Option<&Value>, at: &str) -> Result<()> {
        let change = match (before, after) {
            (None, Some(_)) => "added",
            (Some(_), None) => "removed",
            (Some(a), Some(b)) => {
                // SCH-007: JSON Schema equality, including nested numeric values.
                if conduit_loader::compile(&json!({"const":a}))
                    .map_err(|e| e.to_string())?
                    .is_valid(b)
                {
                    return Ok(());
                }
                "changed"
            }
            (None, None) => return Ok(()),
        };
        self.diagnostics.push(diag(
            "NAM-024",
            format!("contract {}: {change} default at {at}/default", self.name),
        ));
        Ok(())
    }
    fn resolved(&self, mut todo: DefaultParts) -> Result<DefaultParts> {
        let mut out = vec![];
        let mut seen = BTreeSet::new();
        while let Some((scope, root, mut schema)) = todo.pop() {
            let identity =
                serde_json::to_string(&(&scope, &root, &schema)).map_err(|e| e.to_string())?;
            if !seen.insert(identity) {
                continue;
            }
            if let Some(reference) = schema["$ref"].as_str() {
                let (target, sub) = self
                    .env
                    .schema_reference(&scope, &root, reference, self.core)?;
                let target_root = if reference == "#" || reference.starts_with("#/") {
                    root.clone()
                } else {
                    sub.clone()
                };
                todo.push((target, target_root, sub));
            }
            // Siblings still apply. Ref edges add no token to the use-site pointer;
            // erase the edge itself so equivalent reference spellings converge.
            if let Some(map) = schema.as_object_mut() {
                map.remove("$ref");
                if !map.is_empty() {
                    out.push((scope, root, schema));
                }
            }
        }
        Ok(out)
    }
    fn children(parts: DefaultParts) -> BTreeMap<String, DefaultParts> {
        let mut children: BTreeMap<String, DefaultParts> = BTreeMap::new();
        for (scope, root, schema) in parts {
            let mut add = |path: String, sub: &Value| {
                children
                    .entry(path)
                    .or_default()
                    .push((scope.clone(), root.clone(), sub.clone()));
            };
            // Schema positions only, never instance data in const/enum/default.
            for key in [
                "properties",
                "$defs",
                "patternProperties",
                "dependentSchemas",
                "x-conduit-patterns",
            ] {
                for (name, sub) in items(&schema, key) {
                    add(
                        format!("/{key}/{}", name.replace('~', "~0").replace('/', "~1")),
                        sub,
                    );
                }
            }
            for key in [
                "additionalProperties",
                "items",
                "contains",
                "propertyNames",
                "not",
                "if",
                "then",
                "else",
            ] {
                if let Some(sub) = schema.get(key) {
                    add(format!("/{key}"), sub);
                }
            }
            for key in ["allOf", "anyOf", "oneOf", "prefixItems"] {
                if let Some(subs) = schema[key].as_array() {
                    for (i, sub) in subs.iter().enumerate() {
                        add(format!("/{key}/{i}"), sub);
                    }
                }
            }
        }
        children
    }
    fn schemas(
        &mut self,
        before: DefaultParts,
        after: DefaultParts,
        at: &str,
        seen: &BTreeSet<String>,
    ) -> Result<()> {
        let before = self.resolved(before)?;
        let after = self.resolved(after)?;
        let mut seen = seen.clone();
        // Compare pairs of resolved frontiers, not a fixed number of ref expansions.
        // A repeated pair at a deeper position has the same defaults and successors.
        // Keeping this per path still diagnoses distinct properties using one schema.
        let identity = serde_json::to_string(&(&before, &after)).map_err(|e| e.to_string())?;
        if !seen.insert(identity) {
            return Ok(());
        }
        self.value(
            before.iter().find_map(|(_, _, s)| s.get("default")),
            after.iter().find_map(|(_, _, s)| s.get("default")),
            at,
        )?;
        let mut before = Self::children(before);
        let mut after = Self::children(after);
        let paths: BTreeSet<_> = before.keys().chain(after.keys()).cloned().collect();
        for path in paths {
            self.schemas(
                before.remove(&path).unwrap_or_default(),
                after.remove(&path).unwrap_or_default(),
                &format!("{at}{path}"),
                &seen,
            )?;
        }
        Ok(())
    }
}

/// NAM-024/SCH-009: compare published defaults at resolved schema use sites.
/// The environment has already passed admission, including default conflicts.
pub(crate) fn release_default_diagnostics(
    env: &Environment,
    earlier: &str,
    later: &str,
    core: &str,
) -> Result<Vec<Value>> {
    let old = &env.scopes[earlier];
    let new = &env.scopes[later];
    let mut diagnostics = vec![];
    for name in old.exports.iter().flatten() {
        if !new.exports.iter().flatten().any(|n| n == name) {
            continue;
        }
        let (Some(a), Some(b)) = (old.contracts.get(name), new.contracts.get(name)) else {
            continue;
        };
        let mut comparison = DefaultComparison {
            env,
            core,
            name,
            diagnostics: vec![],
        };
        if a["kind"] == "Type" && b["kind"] == "Type" {
            let a = a["spec"]["schema"].clone();
            let b = b["spec"]["schema"].clone();
            comparison.schemas(
                vec![(earlier.into(), a.clone(), a)],
                vec![(later.into(), b.clone(), b)],
                "/spec/schema",
                &BTreeSet::new(),
            )?;
        }
        // NAM-024 also preserves published Requirement input defaults.
        if a["kind"] == "Requirement" && b["kind"] == "Requirement" {
            let inputs: BTreeSet<_> = items(&a["spec"], "inputs")
                .chain(items(&b["spec"], "inputs"))
                .map(|(name, _)| name)
                .collect();
            for input in inputs {
                comparison.value(
                    a["spec"]["inputs"][input].get("default"),
                    b["spec"]["inputs"][input].get("default"),
                    &format!(
                        "/spec/inputs/{}",
                        input.replace('~', "~0").replace('/', "~1")
                    ),
                )?;
            }
        }
        diagnostics.extend(comparison.diagnostics);
    }
    Ok(diagnostics)
}
impl Catalog {
    pub fn new(builtin: Project, snapshots: Vec<Snapshot>, core: &str) -> Result<Self> {
        if manifest(&builtin)["address"] != core {
            return Err("built-in catalog address differs from names file".into());
        }
        let mut keys = BTreeSet::new();
        for s in &snapshots {
            if !s.digest.strip_prefix("sha256:").is_some_and(|h| {
                h.len() == 64
                    && h.bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            }) {
                return Err("invalid snapshot digest metadata".into());
            }
            let mut v =
                Version::parse(&s.version).map_err(|e| format!("invalid snapshot version: {e}"))?;
            v.build = semver::BuildMetadata::EMPTY;
            if !keys.insert((s.address.clone(), v)) {
                return Err("ambiguous snapshot release in catalog".into());
            }
        }
        Ok(Self {
            builtin,
            snapshots,
            core_address: core.into(),
        })
    }
    fn pin(
        &self,
        address: &str,
        item: &Value,
    ) -> std::result::Result<usize, (&'static str, String)> {
        let Some((i, s)) = self.snapshots.iter().enumerate().find(|(_, s)| {
            s.address == address
                && version_equal(&s.version, item["version"].as_str().unwrap_or(""))
        }) else {
            let rule = if self.snapshots.iter().any(|s| s.address == address) {
                "PKG-003"
            } else {
                "PKG-006"
            };
            return Err((
                rule,
                format!(
                    "{address}: requested version {}, digest {}; no matching installed snapshot",
                    item["version"], item["digest"]
                ),
            ));
        };
        if s.digest != item["digest"] {
            return Err((
                "PKG-021",
                format!(
                    "{address}: requested digest {}; installed {}",
                    item["digest"], s.digest
                ),
            ));
        }
        Ok(i)
    }
    pub fn resolve(&self, project: &Project) -> Result<Resolution> {
        let core = self.core_address.as_str();
        let mut diagnostics = vec![];
        let mut decisions = vec![];
        let mut projects = BTreeMap::from([
            ("core".to_owned(), self.builtin.clone()),
            ("project".to_owned(), project.clone()),
        ]);
        for (i, s) in self.snapshots.iter().enumerate() {
            let m = manifest(&s.project);
            if s.address == core {
                diagnostics.push(diag(
                    "NAM-021",
                    format!("snapshot cannot replace built-in {core}"),
                ));
            }
            if m["address"] != s.address
                || !version_equal(m["version"].as_str().unwrap_or(""), &s.version)
            {
                diagnostics.push(diag(
                    "PKG-017",
                    format!(
                        "snapshot {}/{} disagrees with its manifest address/version",
                        s.address, s.version
                    ),
                ));
            }
            for d in &s.project.documents {
                if d.get("from").is_some() {
                    diagnostics.push(diag(
                        "MAN-011",
                        format!(
                            "snapshot {} holds instance {}",
                            s.address, d["metadata"]["name"]
                        ),
                    ));
                }
            }
            projects.insert(format!("snapshot-{i}"), s.project.clone());
        }
        let m = manifest(project);
        let has_items = items(&m, "implementations").next().is_some();
        if (project.documents.iter().any(contract) || has_items) && m.get("address").is_none() {
            diagnostics.push(diag(
                "MAN-015",
                "submitted contracts/implementations need a manifest address",
            ));
        }
        if m["address"] == core {
            diagnostics.push(diag(
                "NAM-021",
                format!("submitted project cannot claim {core}"),
            ));
        }
        if self.snapshots.iter().any(|s| m["address"] == s.address) {
            diagnostics.push(diag(
                "MAN-016",
                format!(
                    "project address {} belongs to an installed snapshot",
                    m["address"]
                ),
            ));
        }
        let mut environments = BTreeMap::new();
        let mut expanded = BTreeMap::new();
        let mut indexes = BTreeMap::new();
        for (scope, p) in &projects {
            let m = manifest(p);
            let own = index(p);
            let mut seen = BTreeSet::new();
            for d in &p.documents {
                if let Some(n) = d["metadata"]["name"].as_str()
                    && (!seen.insert(n) || m["implementations"].get(n).is_some())
                {
                    diagnostics.push(diag(
                        "NAM-027",
                        format!("{scope}: duplicate document/implementation name {n}"),
                    ));
                }
            }
            let mut exports = BTreeSet::new();
            for name in strings(&m, "exports") {
                if !exports.insert(name) {
                    diagnostics.push(diag("PKG-012", format!("{scope}: repeated export {name}")));
                }
                if !p.documents.iter().any(|d| d["metadata"]["name"] == name) {
                    diagnostics.push(diag(
                        "MAN-007",
                        format!("{scope}: export {name} names no document"),
                    ));
                }
            }
            let mut pins = BTreeMap::new();
            for (address, item) in items(&m, "packages") {
                if address == core {
                    diagnostics.push(diag(
                        "NAM-021",
                        format!("{scope}: manifest lists built-in {core}"),
                    ));
                    continue;
                }
                match self.pin(address, item) {
                    Ok(i) => {
                        pins.insert(address.clone(), format!("snapshot-{i}"));
                    }
                    Err((rule, why)) => {
                        diagnostics.push(diag(rule, format!("{scope}: {why}")));
                        if rule != "PKG-006" {
                            diagnostics.push(diag(
                                if scope == "project" {
                                    "PKG-006"
                                } else {
                                    "PKG-028"
                                },
                                format!("{scope}: unsatisfied package {address}"),
                            ));
                        }
                    }
                }
            }
            let runtime = expand_manifest(p, &mut diagnostics)?;
            expanded.insert(scope.clone(), runtime);
            environments.insert(
                scope.clone(),
                Scope {
                    address: m["address"].as_str().unwrap_or("").into(),
                    version: m["version"].as_str().map(str::to_owned),
                    manifest: pins,
                    contracts: own
                        .iter()
                        .filter(|(_, d)| contract(d))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                    exports: if scope == "project" {
                        None
                    } else {
                        Some(exports.into_iter().map(str::to_owned).collect())
                    },
                },
            );
            indexes.insert(scope.clone(), own);
        }
        // Pin failures stop before resolving; no fallback, fetching, or partially exposed context.
        if !diagnostics.is_empty() {
            return Ok(rejected(diagnostics));
        }
        let env = Environment {
            scopes: environments,
        };
        let mut references = vec![];
        let mut used = BTreeMap::<String, BTreeSet<String>>::new();
        for (scope, p) in &projects {
            // The built-in package is trusted, supplied separately, and keeps its own provenance.
            if scope == "core" {
                continue;
            }
            let mut selections = vec![];
            for d in &p.documents {
                selections_document(d, scope != "project", &mut selections)?;
            }
            for (_, item) in items(&manifest(p), "implementations") {
                selections_spec(item, &mut selections);
            }
            for (reference, kind) in selections {
                match resolve_name(&env, &indexes, scope, &reference, &kind, core) {
                    Ok((target, name)) => {
                        let address = &env.scopes[&target].address;
                        if target != *scope {
                            used.entry(scope.clone())
                                .or_default()
                                .insert(address.clone());
                        }
                        references.push(json!({"scope":scope,"reference":reference,"target_scope":target,"name":name,"kind":indexes[&target][&name]["kind"]}));
                    }
                    Err((rule, why)) => {
                        diagnostics.push(diag(rule, format!("{scope}: {why}")));
                        if rule == "PKG-018" {
                            diagnostics.push(diag(
                                "NAM-028",
                                format!("{scope}: unresolved export {reference}"),
                            ));
                        }
                    }
                }
            }
        }
        if !diagnostics.is_empty() {
            return Ok(rejected(diagnostics));
        }
        check_items(&env, &indexes, &projects, core, &mut diagnostics);
        for (scope, p) in &projects {
            if scope == "core" {
                continue;
            }
            for d in &p.documents {
                // FLEET-001: only the core package defines this family's template and Type.
                if (d["kind"] == "Fleet" && d.get("from").is_none())
                    || (d["kind"] == "Type" && d["spec"]["family"] == "Fleet")
                {
                    diagnostics.push(diag(
                        "FLEET-001",
                        format!(
                            "{scope}: {} is a non-core Fleet template or Type",
                            d["metadata"]["name"]
                        ),
                    ));
                }
                diagnostics.extend(env.check_document(scope, d, core)?);
            }
            for (address, _) in items(&manifest(p), "packages") {
                if !used.get(scope).is_some_and(|u| u.contains(address)) {
                    let mut d = diag(
                        "NAM-008",
                        format!("{scope}: unused satisfied package {address}"),
                    );
                    d["level"] = json!("note");
                    diagnostics.push(d);
                }
            }
        }
        if diagnostics.iter().any(|d| d["level"] == "error") {
            return Ok(rejected(diagnostics));
        }
        let digest = project_digest(project)?;
        let mut contexts = BTreeMap::new();
        for (scope, p) in &projects {
            let m = manifest(p);
            let relative_programs: Vec<_> = items(&m, "implementations")
                .filter_map(|(name, item)| {
                    item["command"][0]
                        .as_str()
                        .filter(|s| s.contains('/') && !s.starts_with('/'))
                        .map(|_| name.clone())
                })
                .collect();
            let mut held = json!({"packages":m.get("packages").cloned().unwrap_or(json!({})),"implementations":expanded[scope].get("implementations").cloned().unwrap_or(json!({})),"relative_programs":relative_programs});
            for key in ["address", "version"] {
                if let Some(v) = m.get(key) {
                    held[key] = v.clone();
                }
            }
            if scope == "project" {
                held["digest"] = json!(digest);
            } else if let Some(i) = scope.strip_prefix("snapshot-") {
                let package = &self.snapshots[i.parse::<usize>().map_err(|e| e.to_string())?];
                held["digest"] = json!(package.digest);
                held["source"] = json!(package.source);
            }
            contexts.insert(scope.clone(), held);
        }
        decisions.push(decision(
            "resolve-snapshot-context",
            &[
                "PKG-006", "PKG-003", "PKG-021", "PKG-017", "PKG-018", "PKG-012", "MAN-011",
                "MAN-007", "MAN-015", "MAN-016", "NAM-007", "NAM-008", "NAM-014", "NAM-018",
                "NAM-020", "NAM-021", "NAM-022", "NAM-027", "NAM-028", "SCH-015", "SUB-010",
            ],
            &[],
            json!({"sources":self.snapshots.iter().map(|s| &s.source).collect::<Vec<_>>()}),
            vec![],
        ));
        decisions.push(decision(
            "digest-submitted-project",
            &["SUB-008"],
            &[],
            json!({"digest":digest}),
            vec![],
        ));
        decisions.push(decision(
            "check-implementation-context",
            &[
                "PKG-019", "PKG-020", "PKG-030", "MAN-010", "MAN-017", "MAN-018", "NAM-022",
                "TPL-004",
            ],
            &[],
            json!({}),
            vec![],
        ));
        Ok(Resolution {
            reply: json!({"accepted":true,"diagnostics":diagnostics,"digest":digest,"environment":env,"references":references,"expanded_manifest":expanded["project"],"contexts":contexts,"source":if self.snapshots.iter().any(|s| s.source == SnapshotSource::InstalledArchive) {"package-catalog"} else {"preinstalled-snapshot"}}),
            decisions,
        })
    }
}
fn rejected(diagnostics: Vec<Value>) -> Resolution {
    let rules: BTreeSet<_> = diagnostics
        .iter()
        .filter_map(|d| d["ruleId"].as_str())
        .collect();
    let d = decision(
        "reject-project-context",
        &rules.into_iter().collect::<Vec<_>>(),
        &[],
        json!({}),
        vec![],
    );
    Resolution {
        reply: json!({"accepted":false,"reason":"invalid","diagnostics":diagnostics}),
        decisions: vec![d],
    }
}
fn expand_manifest(p: &Project, diagnostics: &mut Vec<Value>) -> Result<Value> {
    let mut m = manifest(p);
    let base = p
        .directory
        .as_ref()
        .map(|s| UriRef::parse(s.as_str()).map_err(|e| e.to_string()))
        .transpose()?;
    if let Some(base) = &base
        && (base.scheme().is_none_or(|s| s.as_str() != "file")
            || !base.path().as_str().ends_with('/')
            || !base.path().as_str().starts_with('/')
            || base.query().is_some()
            || base.fragment().is_some())
    {
        return Err("directory must be an absolute file URL ending in /".into());
    }
    if let Some(packages) = m["packages"].as_object_mut() {
        for (address, item) in packages {
            if let Some(fetch) = item["fetch"].as_str()
                && UriRef::parse(fetch)
                    .map_err(|e| e.to_string())?
                    .scheme()
                    .is_none()
            {
                if let Some(base) = &base {
                    item["fetch"] = json!(file_reference(base, fetch)?);
                } else {
                    diagnostics.push(diag(
                        "MAN-006",
                        format!("{address}: relative fetch {fetch} has no directory"),
                    ));
                }
            }
        }
    }
    if let Some(implementations) = m["implementations"].as_object_mut() {
        for (name, item) in implementations {
            if let Some(program) = item["command"][0].as_str()
                && program.contains('/')
                && !program.starts_with('/')
            {
                if let Some(base) = &base {
                    let directory = percent_encoding::percent_decode_str(base.path().as_str())
                        .decode_utf8()
                        .map_err(|e| e.to_string())?;
                    // MAN-010/SUB-010: anchor to the manifest directory, keeping
                    // `..` for filesystem lookup, which may traverse symlinks.
                    let path: PathBuf = PathBuf::from(directory.as_ref())
                        .join(program)
                        .components()
                        .collect();
                    item["command"][0] = json!(path.to_str().ok_or("non-UTF8 program")?);
                } else {
                    diagnostics.push(diag(
                        "MAN-010",
                        format!("{name}: relative program {program} has no directory"),
                    ));
                }
            }
        }
    }
    Ok(m)
}
pub(crate) fn resolve_name(
    env: &Environment,
    indexes: &BTreeMap<String, BTreeMap<String, Value>>,
    scope: &str,
    reference: &str,
    kind: &str,
    core: &str,
) -> std::result::Result<(String, String), (&'static str, String)> {
    let Some((address, name)) = reference.split_once('#') else {
        return Err((
            "NAM-018",
            format!("selection {reference} is not qualified or #local"),
        ));
    };
    let own = &env.scopes[scope];
    let target = if address.is_empty() {
        scope.to_owned()
    } else if address == core {
        "core".into()
    } else {
        own.manifest
            .get(address)
            .cloned()
            .ok_or(("NAM-007", format!("no manifest pin for {reference}")))?
    };
    let Some(doc) = indexes[&target].get(name) else {
        return Err(("NAM-028", format!("unresolved {reference}")));
    };
    if target != scope
        && doc["kind"] != "Implementation"
        && !env.scopes[&target]
            .exports
            .as_ref()
            .is_some_and(|e| e.iter().any(|e| e == name))
    {
        return Err(("PKG-018", format!("{reference} is not exported (NAM-028)")));
    }
    if let Some(family) = kind.strip_prefix("Template:") {
        if contract(doc) || doc["kind"] == "Implementation" || doc.get("from").is_some() {
            return Err(("INS-002", format!("{reference} is not a template")));
        }
        if doc["kind"] != family {
            return Err(("INS-002", format!("{reference} is not a {family} template")));
        }
    } else if doc["kind"] != kind {
        return Err((
            if kind == "Schema" {
                "SCH-015"
            } else {
                "NAM-018"
            },
            format!("{reference} selects {}, expected {kind}", doc["kind"]),
        ));
    }
    Ok((target, name.into()))
}
fn add(v: &Value, kind: &str, out: &mut Vec<(String, String)>) {
    if let Some(s) = v.as_str() {
        out.push((s.into(), kind.into()));
    }
}
fn selections_spec(spec: &Value, out: &mut Vec<(String, String)>) {
    for (key, kind) in [
        ("type", "Type"),
        ("capability", "Capability"),
        ("requirement", "Requirement"),
    ] {
        add(&spec[key], kind, out);
    }
    for (key, kind) in [
        ("offers", "Capability"),
        ("requires", "Operation"),
        ("operations", "Operation"),
        ("implements", "Operation"),
        ("supports", "Requirement"),
    ] {
        for r in strings(spec, key) {
            out.push((r.into(), kind.into()));
        }
    }
    for r in strings(spec, "controls") {
        out.push((r.into(), "Operation".into()));
    }
    for (key, kind) in [
        ("implementation", "Implementation"),
        ("controls", "Control"),
        ("tools", "Tool"),
    ] {
        for (name, v) in items(spec, key) {
            out.push((name.clone(), "Operation".into()));
            if kind == "Implementation" {
                add(v, "Implementation", out);
            }
        }
    }
    for (_, r) in items(spec, "requirements") {
        add(&r["requirement"], "Requirement", out);
    }
}
pub(crate) fn selections_document(
    d: &Value,
    packaged: bool,
    out: &mut Vec<(String, String)>,
) -> Result<()> {
    if let Some(from) = d["from"].as_str()
        && (from.contains('#') && (!from.starts_with('#') || packaged))
    {
        out.push((
            from.into(),
            format!("Template:{}", d["kind"].as_str().unwrap_or("Conduit")),
        ));
    }
    if d["kind"] != "Schema" {
        selections_spec(&d["spec"], out);
    }
    for (_, mut root) in crate::schemas::roots(d) {
        crate::schemas::walk(&mut root, "", &mut |map, _| {
            if let Some(r) = map.get("$ref").and_then(Value::as_str)
                && r != "#"
                && !r.starts_with("#/")
            {
                out.push((r.into(), "Schema".into()));
            }
            Ok(())
        })?;
    }
    for key in ["conduits", "entries"] {
        for (_, nested) in items(&d["spec"], key) {
            let mut nested = nested.clone();
            nested["kind"] = json!(if key == "conduits" {
                "Conduit"
            } else {
                "Resource"
            });
            selections_document(&nested, packaged, out)?;
        }
    }
    Ok(())
}

fn check_items(
    env: &Environment,
    indexes: &BTreeMap<String, BTreeMap<String, Value>>,
    projects: &BTreeMap<String, Project>,
    core: &str,
    diagnostics: &mut Vec<Value>,
) {
    let provider_ops: BTreeSet<_> = [
        "runtime-provider.v1",
        "machine-provider.v1",
        "package-source.v1",
    ]
    .iter()
    .flat_map(|n| strings(&indexes["core"][*n]["spec"], "operations"))
    .map(str::to_owned)
    .collect();
    for (scope, p) in projects {
        if scope == "core" {
            continue;
        }
        for (name, item) in items(&manifest(p), "implementations") {
            if item["plain"] == true {
                for op in strings(item, "implements") {
                    if item["tools"].get(op).is_none() || provider_ops.contains(op) {
                        diagnostics.push(diag("PKG-020",format!("{scope}#{name}: plain implementation has no tool mapping for {op}, or implements a provider operation")));
                    }
                }
            }
            if item.get("remote").is_some()
                && strings(item, "implements").any(|op| provider_ops.contains(op))
            {
                diagnostics.push(diag(
                    "PKG-030",
                    format!("{scope}#{name}: remote item implements provider operation"),
                ));
            }
            for (bindings, source, declared) in [
                ("packageArguments", "package", "packageArguments"),
                ("environment", "package", "environmentVariables"),
                ("headers", "remote", "headers"),
            ] {
                if item.get(source).is_none() {
                    continue;
                }
                for (binding, _) in items(item, bindings) {
                    let count = item[source][declared]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter(|a| {
                            let key = if declared == "packageArguments" && a["type"] == "positional"
                            {
                                &a["valueHint"]
                            } else {
                                &a["name"]
                            };
                            key == binding
                        })
                        .collect::<Vec<_>>();
                    if count.len() != 1
                        || (bindings == "packageArguments" && count[0].get("value").is_some())
                    {
                        diagnostics.push(diag("MAN-010",format!("{scope}#{name}: binding {binding} is not one configurable declared {declared} input")));
                    }
                }
            }
        }
        for d in &p.documents {
            if d.get("from").is_some() || contract(d) || d["spec"].get("type").is_none() {
                continue;
            }
            let Some(r) = d["spec"]["type"].as_str() else {
                continue;
            };
            let Ok((ts, tn)) = resolve_name(env, indexes, scope, r, "Type", core) else {
                continue;
            };
            let t = &indexes[&ts][&tn];
            if t["spec"]["family"] != d["kind"] {
                diagnostics.push(diag(
                    "TPL-002",
                    format!(
                        "template {} selects Type {r} of another family",
                        d["metadata"]["name"]
                    ),
                ));
            }
            // TPL-004: only required, offered or control operations use Type defaults.
            let mut needed = BTreeSet::new();
            for op in strings(&t["spec"], "requires").chain(strings(&t["spec"], "controls")) {
                if let Ok(operation) = resolve_name(env, indexes, &ts, op, "Operation", core) {
                    needed.insert(operation);
                }
            }
            for cap in strings(&t["spec"], "offers") {
                if let Ok((cs, cn)) = resolve_name(env, indexes, &ts, cap, "Capability", core) {
                    for op in strings(&indexes[&cs][&cn]["spec"], "operations") {
                        if let Ok(operation) =
                            resolve_name(env, indexes, &cs, op, "Operation", core)
                        {
                            needed.insert(operation);
                        }
                    }
                }
            }
            let mut selected = BTreeMap::new();
            // NAM-022/TPL-004: compare the operations selected by each key in
            // its own scope, retaining that scope for the implementation value.
            for (is_default, selection_scope, spec) in [
                (true, ts.as_str(), &t["spec"]),
                (false, scope.as_str(), &d["spec"]),
            ] {
                for (op, item) in items(spec, "implementation") {
                    if let Ok(operation) =
                        resolve_name(env, indexes, selection_scope, op, "Operation", core)
                    {
                        selected.insert(operation, (selection_scope, item, is_default));
                    }
                }
            }
            for (operation, (selection_scope, item, is_default)) in selected {
                let Some(r) = item.as_str() else {
                    continue;
                };
                let Ok((is, iname)) =
                    resolve_name(env, indexes, selection_scope, r, "Implementation", core)
                else {
                    continue;
                };
                let implementation = &indexes[&is][&iname]["spec"];
                // MAN-018/TPL-004: only effective selections reserve variables.
                // Secret-reference bindings use this same environment map;
                // run materialization later moves their values under secrets.
                if !is_default || needed.contains(&operation) {
                    for (name, pointer) in items(implementation, "environment") {
                        if crate::run_variables()
                            .values()
                            .any(|reserved| name.eq_ignore_ascii_case(reserved))
                        {
                            diagnostics.push(diag(
                                "MAN-018",
                                format!(
                                    "template {} selects {r}: reserved variable {name} -> {pointer}",
                                    d["metadata"]["name"]
                                ),
                            ));
                        }
                    }
                }
                for (bindings, source, declared) in [
                    ("packageArguments", "package", "packageArguments"),
                    ("environment", "package", "environmentVariables"),
                    ("headers", "remote", "headers"),
                ] {
                    for (name, pointer) in items(implementation, bindings) {
                        let Some(pointer) = pointer.as_str() else {
                            continue;
                        };
                        let tokens: Vec<_> = pointer
                            .strip_prefix('/')
                            .map(|p| {
                                p.split('/')
                                    .map(|s| s.replace("~1", "/").replace("~0", "~"))
                                    .collect()
                            })
                            .unwrap_or_default();
                        let required_input = implementation[source][declared]
                            .as_array()
                            .into_iter()
                            .flatten()
                            .any(|a| {
                                a["isRequired"] == true
                                    && (a["name"] == *name || a["valueHint"] == *name)
                            });
                        let ok = binding(env, &ts, &t["spec"]["schema"], &tokens, core)
                            .is_some_and(|nullable| !required_input || !nullable);
                        if !ok {
                            diagnostics.push(diag("MAN-017",format!("template {} selects {r}: {name} -> {pointer} is not a required scalar place, or required input admits null",d["metadata"]["name"])));
                        }
                    }
                }
            }
        }
    }
}
#[derive(Clone)]
struct BindingPart {
    scope: String,
    root: Value,
    schema: Value,
    secret: bool,
}

fn branches(
    env: &Environment,
    part: BindingPart,
    core: &str,
    seen: &BTreeSet<String>,
) -> Option<Vec<Vec<BindingPart>>> {
    let mut seen = seen.clone();
    let mut own = part.clone();
    let mut conjunctions = vec![];
    if let Some(reference) = part.schema["$ref"].as_str() {
        if reference == format!("{core}#secret-reference.v1") {
            own.secret = true;
        } else if seen
            .insert(serde_json::to_string(&json!([part.scope, part.root, reference])).ok()?)
        {
            let (scope, schema) = env
                .schema_reference(&part.scope, &part.root, reference, core)
                .ok()?;
            let root = if reference == "#" || reference.starts_with("#/") {
                part.root.clone()
            } else {
                schema.clone()
            };
            conjunctions.push(BindingPart {
                scope,
                root,
                schema,
                secret: false,
            });
        }
    }
    conjunctions.extend(
        part.schema["allOf"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|schema| BindingPart {
                schema: schema.clone(),
                ..part.clone()
            }),
    );
    if let Some(map) = own.schema.as_object_mut() {
        for key in ["$ref", "allOf", "anyOf", "oneOf"] {
            map.remove(key);
        }
    }
    let mut result = vec![vec![own]];
    for conjunct in conjunctions {
        let other = branches(env, conjunct, core, &seen)?;
        result = result
            .iter()
            .flat_map(|a| other.iter().map(|b| [a.clone(), b.clone()].concat()))
            .collect();
    }
    for key in ["anyOf", "oneOf"] {
        if let Some(choices) = part.schema[key].as_array() {
            let mut other = vec![];
            for choice in choices {
                other.extend(branches(
                    env,
                    BindingPart {
                        schema: choice.clone(),
                        ..part.clone()
                    },
                    core,
                    &seen,
                )?);
            }
            result = result
                .iter()
                .flat_map(|a| other.iter().map(|b| [a.clone(), b.clone()].concat()))
                .collect();
        }
    }
    Some(result)
}
/// Conjunctions keep their separate roots and scopes. Each disjunct must guarantee the
/// required pointer and a scalar leaf; a sibling never overwrites a referenced constraint.
fn binding(
    env: &Environment,
    scope: &str,
    schema: &Value,
    tokens: &[String],
    core: &str,
) -> Option<bool> {
    binding_parts(
        env,
        vec![BindingPart {
            scope: scope.into(),
            root: schema.clone(),
            schema: schema.clone(),
            secret: false,
        }],
        tokens,
        core,
    )
}
fn binding_parts(
    env: &Environment,
    parts: Vec<BindingPart>,
    tokens: &[String],
    core: &str,
) -> Option<bool> {
    let mut alternatives = vec![vec![]];
    for part in parts {
        let expanded = branches(env, part, core, &BTreeSet::new())?;
        alternatives = alternatives
            .iter()
            .flat_map(|a| expanded.iter().map(|b| [a.clone(), b.clone()].concat()))
            .collect();
    }
    let mut nullable = false;
    for parts in alternatives {
        if let Some((head, tail)) = tokens.split_first() {
            // MAN-017: required/properties constrain objects only. Every admitted
            // value here must be an object for the next member to have a value.
            if binding_mask(&parts) & !32 != 0
                || !parts
                    .iter()
                    .any(|p| strings(&p.schema, "required").any(|n| n == head))
            {
                return None;
            }
            let mut subs = vec![];
            for p in parts {
                let mut applicable = vec![];
                if let Some(sub) = p.schema["properties"].get(head) {
                    applicable.push(sub.clone());
                }
                for (pattern, sub) in items(&p.schema, "patternProperties") {
                    if conduit_loader::formats::ecma_regex(pattern)
                        .ok()?
                        .find(head)
                        .is_some()
                    {
                        applicable.push(sub.clone());
                    }
                }
                if applicable.is_empty()
                    && let Some(sub) = p.schema.get("additionalProperties")
                {
                    applicable.push(sub.clone());
                }
                subs.extend(applicable.into_iter().map(|schema| BindingPart {
                    schema,
                    ..p.clone()
                }));
            }
            if subs.is_empty() {
                return None;
            }
            nullable |= binding_parts(env, subs, tail, core)?;
        } else {
            if parts.iter().any(|p| p.secret) {
                continue;
            }
            let mask = binding_mask(&parts);
            if mask & 48 != 0 {
                return None;
            }
            nullable |= mask & 1 != 0;
        }
    }
    Some(nullable)
}
/// null, boolean, number, string, array, object. Conjuncts only narrow the set.
fn binding_mask(parts: &[BindingPart]) -> u8 {
    let mut mask = 63;
    for p in parts {
        if p.schema == false {
            mask = 0;
            continue;
        }
        if let Some(c) = p.schema.get("const") {
            mask &= value_mask(c);
        }
        if let Some(values) = p.schema["enum"].as_array() {
            mask &= values.iter().fold(0, |m, v| m | value_mask(v));
        }
        let ty = |s: &str| match s {
            "null" => 1,
            "boolean" => 2,
            "number" | "integer" => 4,
            "string" => 8,
            "array" => 16,
            "object" => 32,
            _ => 63,
        };
        if let Some(t) = p.schema["type"].as_str() {
            mask &= ty(t);
        }
        if let Some(types) = p.schema["type"].as_array() {
            mask &= types
                .iter()
                .filter_map(Value::as_str)
                .fold(0, |m, t| m | ty(t));
        }
    }
    mask
}
fn value_mask(v: &Value) -> u8 {
    match v {
        Value::Null => 1,
        Value::Bool(_) => 2,
        Value::Number(_) => 4,
        Value::String(_) => 8,
        Value::Array(_) => 16,
        Value::Object(_) => 32,
    }
}

/// RFC 3986 section 5.2 for a relative reference against an absolute file directory.
/// Percent-encoded dots remain encoded; command paths use filesystem tokens separately.
fn file_reference(base: &UriRef<&str>, reference: &str) -> Result<String> {
    let r = UriRef::parse(reference).map_err(|e| e.to_string())?;
    let authority = r
        .authority()
        .or_else(|| base.authority())
        .map(|a| format!("//{}", a.as_str()))
        .unwrap_or_default();
    let path = r.path().as_str();
    let path = if r.authority().is_some() || path.starts_with('/') {
        path.to_owned()
    } else {
        format!("{}{path}", base.path().as_str())
    };
    let mut input = path.as_str();
    let mut output = String::new();
    while !input.is_empty() {
        if let Some(rest) = input
            .strip_prefix("../")
            .or_else(|| input.strip_prefix("./"))
        {
            input = rest;
        } else if input.starts_with("/./") || input == "/." {
            input = &input[2..];
            if input.is_empty() {
                input = "/";
            }
        } else if input.starts_with("/../") || input == "/.." {
            input = &input[3..];
            if input.is_empty() {
                input = "/";
            }
            output.truncate(output.rfind('/').unwrap_or(0));
        } else if input == "." || input == ".." {
            input = "";
        } else {
            let end = input[usize::from(input.starts_with('/'))..]
                .find('/')
                .map(|i| i + usize::from(input.starts_with('/')))
                .unwrap_or(input.len());
            output.push_str(&input[..end]);
            input = &input[end..];
        }
    }
    let query = r
        .query()
        .map(|q| format!("?{}", q.as_str()))
        .unwrap_or_default();
    let fragment = r
        .fragment()
        .map(|f| format!("#{}", f.as_str()))
        .unwrap_or_default();
    Ok(format!("file:{authority}{output}{query}{fragment}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn compare_defaults(earlier: &Project, later: &Project) -> Vec<Value> {
        let scopes = [("old", earlier), ("new", later)]
            .into_iter()
            .map(|(key, p)| {
                (
                    key.into(),
                    Scope {
                        address: "example.org/test".into(),
                        version: None,
                        manifest: BTreeMap::new(),
                        contracts: index(p),
                        exports: Some(
                            strings(&manifest(p), "exports")
                                .map(str::to_owned)
                                .collect(),
                        ),
                    },
                )
            })
            .collect();
        release_default_diagnostics(&Environment { scopes }, "old", "new", "builtin").unwrap()
    }
    #[test]
    fn release_defaults_visit_schema_locations_and_escape_property_pointers() {
        let project = |schema| Project {
            manifest: Some(json!({"exports":["test.v1"]})),
            documents: vec![
                json!({"kind":"Type","metadata":{"name":"test.v1"},"spec":{"schema":schema}}),
            ],
            directory: None,
        };
        let old = project(json!({"type":"object","properties":{"kept":{"default":1}}}));
        let later = project(json!({"type":"object","properties":{
            "kept":{"default":1}, "optional":{"type":"string"},
            "array":{"items":{"properties":{"a/b~c":{"default":false}}}},
            "data":{"const":{"properties":{"fake":{"default":1}}}}
        },"$defs":{"nested":{"properties":{"depth":{"default":null}}}}}));
        let diagnostics = compare_defaults(&old, &later);
        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics.iter().all(|d| d["ruleId"] == "NAM-024"));
        let messages = diagnostics
            .iter()
            .map(|d| d["message"]["text"].as_str().unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            messages.contains("/spec/schema/properties/array/items/properties/a~1b~0c/default")
        );
        assert!(messages.contains("/spec/schema/$defs/nested/properties/depth/default"));
        assert!(compare_defaults(&old, &old).is_empty());
    }
    #[test]
    fn release_defaults_follow_pinned_contracts_and_local_roots_with_cycles() {
        let ty = json!({"kind":"Type","spec":{"schema":{"properties":{
            "settings":{"$ref":"example.org/settings#settings.v1"}
        }}}});
        let schema = |value| {
            json!({"kind":"Schema","spec":{
                "$defs":{"node":{"properties":{
                    "a/b~c":{"default":value},
                    "next":{"$ref":"#/$defs/node"}
                }}},
                "$ref":"#/$defs/node"
            }})
        };
        let mut scopes = BTreeMap::new();
        for (scope, dependency, value) in [("old", "old-dep", false), ("new", "new-dep", true)] {
            scopes.insert(
                scope.into(),
                Scope {
                    address: "example.org/types".into(),
                    version: None,
                    manifest: BTreeMap::from([("example.org/settings".into(), dependency.into())]),
                    contracts: BTreeMap::from([("test.v1".into(), ty.clone())]),
                    exports: Some(vec!["test.v1".into()]),
                },
            );
            scopes.insert(
                dependency.into(),
                Scope {
                    address: "example.org/settings".into(),
                    version: None,
                    manifest: BTreeMap::new(),
                    contracts: BTreeMap::from([("settings.v1".into(), schema(value))]),
                    exports: Some(vec!["settings.v1".into()]),
                },
            );
        }
        let env = Environment { scopes };
        let diagnostics = release_default_diagnostics(&env, "old", "new", "builtin").unwrap();
        assert!(diagnostics.iter().any(|d| d["message"]["text"] ==
            "contract test.v1: changed default at /spec/schema/properties/settings/properties/a~1b~0c/default"));
        assert!(diagnostics.iter().all(|d| d["ruleId"] == "NAM-024"));
        assert!(
            release_default_diagnostics(&env, "old", "old", "builtin")
                .unwrap()
                .is_empty()
        );
    }
    #[test]
    fn submitted_digest_uses_jcs_numbers_and_utf16_member_order() {
        let p = Project {
            manifest: Some(json!({"z":-0.0,"a":1e-7,"b":1e21,"😀":1,"\u{e000}":2})),
            documents: vec![],
            directory: None,
        };
        // Independent SHA-256 of the JCS text (UTF-16 orders the astral key before U+E000).
        assert_eq!(
            project_digest(&p).unwrap(),
            "sha256:5a88a814374add54c902454384cdfa8d3f104ac533968bc3f4cf7a54dc8f01c5"
        );
    }
}
