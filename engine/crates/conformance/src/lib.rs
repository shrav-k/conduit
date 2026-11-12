//! File input, case validation and assertions. This crate owns all I/O for the pure core.
mod access;
mod checks;
mod expect;
mod packages;
pub mod pieces;
mod records;
mod schemas;
mod steps;
use conduit_core::{Context, Input, State};
use conduit_loader::{
    Diagnostic, Specification, Syntax, read_bytes, read_file, read_submitted_files,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    id: String,
    layers: Vec<String>,
    promises: Vec<String>,
    title: String,
    profile: String,
    initial: Initial,
    files: BTreeMap<String, String>,
    steps: Vec<Value>,
}
#[derive(Debug, Deserialize)]
#[serde(tag = "recipe", rename_all = "kebab-case", deny_unknown_fields)]
enum Initial {
    Empty,
    /// A supplied established state, not a first-start transition or a registration.
    Snapshot {
        file: String,
    },
}

#[derive(Debug, PartialEq)]
pub struct CaseReport {
    pub id: String,
    pub promises: Vec<String>,
    pub steps: usize,
    pub storage: Vec<Value>,
}

fn read_value(path: &Path) -> Result<Value> {
    let documents = read_file(path).map_err(|e| e.to_string())?;
    if documents.len() != 1 {
        return Err(format!("{}: expected one control document", path.display()));
    }
    Ok(documents[0].root.to_json())
}

fn paths(directory: &Path, found: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(directory).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.is_dir() {
            paths(&path, found)?;
        } else if path
            .file_name()
            .is_some_and(|s| s == "case.yaml" || s == "index.yaml")
        {
            found.push(path);
        }
    }
    Ok(())
}

/// Run every discovered case. Unsupported inputs and invalid assertions fail, never skip.
pub fn run_suite(repository: &Path) -> Result<Vec<CaseReport>> {
    run_matching(repository, "")
}

/// Run a development subset by stable ID prefix; inventory validation still covers all cases.
pub fn run_matching(repository: &Path, prefix: &str) -> Result<Vec<CaseReport>> {
    let threads = vector_threads(std::env::var("CONDUIT_VECTOR_THREADS"))?;
    run_matching_with_threads(repository, prefix, threads)
}

fn vector_threads(value: std::result::Result<String, std::env::VarError>) -> Result<usize> {
    match value {
        Ok(value) if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) => value
            .parse::<usize>()
            .ok()
            .filter(|n| *n > 0)
            .ok_or("CONDUIT_VECTOR_THREADS must be a positive integer".into()),
        Ok(_) | Err(std::env::VarError::NotUnicode(_)) => {
            Err("CONDUIT_VECTOR_THREADS must be a positive integer".into())
        }
        Err(std::env::VarError::NotPresent) => std::thread::available_parallelism()
            .map(usize::from)
            .map_err(|e| format!("available_parallelism: {e}")),
    }
}

/// Run a subset with an explicit worker count, without changing the process environment.
pub fn run_matching_with_threads(
    repository: &Path,
    prefix: &str,
    threads: usize,
) -> Result<Vec<CaseReport>> {
    if threads == 0 {
        return Err("vector thread count must be positive".into());
    }
    let directory = repository.join("conformance");
    let mut found = vec![];
    paths(&directory, &mut found)?;
    found.sort();
    let mut metadata = pieces::Files::new();
    for name in ["manifest.yaml", "case.schema.json"] {
        metadata.insert(
            name.into(),
            std::fs::read(directory.join(name)).map_err(|e| format!("{name}: {e}"))?,
        );
    }
    let mut cases = vec![];
    for path in found {
        if path.file_name().is_some_and(|s| s == "index.yaml") {
            let relative = path.strip_prefix(&directory).map_err(|e| e.to_string())?;
            metadata.insert(
                relative.to_string_lossy().into_owned(),
                std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?,
            );
        } else {
            cases.push(path);
        }
    }
    for kind in ["inputs", "expect", "rules"] {
        let folder = directory.join("case-schema").join(kind);
        if !folder.exists() {
            continue;
        }
        for entry in std::fs::read_dir(&folder).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_some_and(|e| e == "json") && path.is_file() {
                let relative = path.strip_prefix(&directory).map_err(|e| e.to_string())?;
                metadata.insert(
                    relative.to_string_lossy().into_owned(),
                    std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?,
                );
            }
        }
    }
    if cases.is_empty() {
        return Err("no conformance cases found".into());
    }
    let manifest = read_value(&directory.join("manifest.yaml"))?;
    if manifest.get("cases").is_some() {
        return Err(pieces::repair(
            "manifest.yaml",
            "",
            "still has cases; an entry belongs in the index.yaml of the directory holding its case directory",
        ));
    }
    let indexed = pieces::entries(&metadata)?;
    let inventory: Vec<_> = indexed.iter().map(|(_, entry)| entry.clone()).collect();
    let spec = Specification::read(&repository.join("spec")).map_err(|e| e.to_string())?;
    let schema = pieces::assemble(&metadata)?;
    let validator = jsonschema::options()
        .build(&schema)
        .map_err(|e| e.to_string())?;
    let profile = read_value(&repository.join("conformance/profiles/first-build.yaml"))?;
    let mut context = Context {
        builtin: Some(packages::builtin(&spec)?),
        core_package: spec.names().core_package.clone(),
        meta_prefix: read_file(&repository.join("spec/names.yaml")).unwrap()[0]
            .root
            .to_json()["metaPrefix"]
            .as_str()
            .unwrap()
            .into(),
        templates: BTreeMap::new(),
        core_environment: schemas::environment(
            repository,
            &json!({"environment":{"scopes":{}}}),
            &spec,
        )?,
        record_source: profile["DL8"]["source"]
            .as_str()
            .ok_or("profile needs DL8.source")?
            .into(),
    };
    for document in
        read_file(&repository.join("spec/core/templates.yaml")).map_err(|e| e.to_string())?
    {
        let value = document.root.to_json();
        let name = value["metadata"]["name"]
            .as_str()
            .ok_or("built-in template has no name")?;
        context.templates.insert(name.into(), value);
    }
    if profile["DL10"]["nesting"] != conduit_loader::MAX_DEPTH {
        return Err("profile/loader depth mismatch".into());
    }
    let mut known_promises = BTreeSet::new();
    for directory in ["promises", "conventions", "interface"] {
        for entry in
            std::fs::read_dir(repository.join("spec").join(directory)).map_err(|e| e.to_string())?
        {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_none_or(|e| e != "yaml") {
                continue;
            }
            for document in read_file(&path).map_err(|e| e.to_string())? {
                if let Some(items) = document.root.to_json().as_array() {
                    for item in items {
                        if let Some(id) = item["id"].as_str() {
                            known_promises.insert(id.to_string());
                        }
                    }
                }
            }
        }
    }
    let mut ids = BTreeSet::new();
    let s1_promises = read_value(&directory.join("s1-promises.yaml"))?;
    validate_s1_coverage(&manifest, &inventory, &s1_promises, &known_promises)?;
    let mut prepared = vec![];
    for path in cases {
        let value = read_value(&path)?;
        validator
            .validate(&value)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        let case: Case =
            serde_json::from_value(value).map_err(|e| format!("{}: {e}", path.display()))?;
        if !ids.insert(case.id.clone()) {
            return Err(format!(
                "{}: duplicate case ID {}; give each case a unique ID",
                path.display(),
                case.id
            ));
        }
        let relative = path
            .strip_prefix(&directory)
            .map_err(|e| e.to_string())?
            .to_string_lossy();
        let (piece, entry) = indexed
            .iter()
            .find(|(_, entry)| entry["id"] == case.id)
            .ok_or_else(|| {
                pieces::repair(
                    &relative,
                    &format!("{}: ", case.id),
                    &format!(
                        "case missing from coverage index; add its entry to {}",
                        pieces::index_for(&relative).unwrap_or_default()
                    ),
                )
            })?;
        if entry["case"] != relative.as_ref() || entry["promises"] != json!(case.promises) {
            return Err(pieces::repair(
                piece,
                &format!("{}: ", case.id),
                "coverage index differs from case; correct its case path and promises",
            ));
        }
        for promise in &case.promises {
            if !known_promises.contains(promise) {
                return Err(format!("{}: unknown promise {promise}", case.id));
            }
        }
        prepared.push((path, case));
    }
    for (piece, entry) in &indexed {
        let id = entry["id"].as_str().expect("validated index ID");
        if !ids.contains(id) {
            return Err(pieces::repair(
                piece,
                &format!("{id}: "),
                "entry lists a missing case; restore the case or remove its entry",
            ));
        }
    }
    let matching: Vec<_> = prepared
        .iter()
        .filter(|(_, case)| case.id.starts_with(prefix))
        .collect();
    let cursor = std::sync::atomic::AtomicUsize::new(0);
    let batches = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads.min(matching.len()))
            .map(|_| {
                let matching = &matching;
                let cursor = &cursor;
                let spec = &spec;
                let context = &context;
                let known = &known_promises;
                scope.spawn(move || {
                    let mut completed = vec![];
                    loop {
                        let index = cursor.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some((path, case)) = matching.get(index) else {
                            break;
                        };
                        completed.push((index, run_case(path, case, spec, context, known)));
                    }
                    completed
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join())
            .collect::<Vec<_>>()
    });
    let mut completed = vec![];
    for batch in batches {
        completed.extend(batch.map_err(|_| "conformance case worker panicked")?);
    }
    completed.sort_by_key(|(index, _)| *index);
    let mut reports = vec![];
    let mut failures = vec![];
    for (index, result) in completed {
        let (_, case) = matching[index];
        match result {
            Ok(storage) => reports.push(CaseReport {
                id: case.id.clone(),
                promises: case.promises.clone(),
                steps: case.steps.len(),
                storage,
            }),
            Err(error) => failures.push(format!(
                "{} [{}]: {error}",
                case.id,
                case.promises.join(", ")
            )),
        }
    }
    if !failures.is_empty() {
        return Err(format!(
            "{} cases passed; {} failed\n{}",
            reports.len(),
            failures.len(),
            failures.join("\n")
        ));
    }
    if reports.is_empty() {
        return Err("no cases matched the requested prefix".into());
    }
    Ok(reports)
}

/// Keep the frozen S1 inventory visible, including unvectorized boundaries.
/// A case tag is evidence for the stated slice, not proof of the whole promise.
fn validate_s1_coverage(
    manifest: &Value,
    entries: &[Value],
    inventory: &Value,
    known: &BTreeSet<String>,
) -> Result<()> {
    let mut expected = BTreeSet::new();
    for value in inventory.as_array().ok_or("S1 inventory must be a list")? {
        let id = value.as_str().ok_or("S1 inventory ID must be a string")?;
        if !known.contains(id) {
            return Err(format!("S1 inventory has unknown promise {id}"));
        }
        if !expected.insert(id.to_string()) {
            return Err(format!("duplicate S1 inventory promise {id}"));
        }
    }
    if expected.is_empty() {
        return Err("S1 inventory is empty".into());
    }
    let rows = manifest["s1_coverage"]
        .as_array()
        .ok_or("coverage index has no s1_coverage")?;
    let mut found = BTreeSet::new();
    for row in rows {
        let promise = row["promise"].as_str().ok_or("S1 row has no promise")?;
        if !found.insert(promise.to_string()) {
            return Err(format!("duplicate S1 coverage row {promise}"));
        }
        let cases = row["vectors"].as_array().ok_or("S1 row needs vectors")?;
        let reason = row["reason"].as_str().unwrap_or("").trim();
        if cases.is_empty() == reason.is_empty() {
            return Err(format!(
                "{promise}: supply vectors or a reason, exclusively"
            ));
        }
        let mut seen = BTreeSet::new();
        for id in cases {
            let id = id.as_str().ok_or("S1 vector ID is not a string")?;
            if !seen.insert(id) {
                return Err(format!("{promise}: duplicate S1 vector {id}"));
            }
            let case = entries
                .iter()
                .find(|case| case["id"] == id)
                .ok_or_else(|| format!("{promise}: unknown S1 vector {id}"))?;
            if !case["promises"]
                .as_array()
                .is_some_and(|ids| ids.contains(&json!(promise)))
            {
                return Err(format!("{promise}: vector {id} does not claim it"));
            }
        }
    }
    if found != expected {
        return Err(format!(
            "S1 coverage differs from frozen inventory: missing {:?}; extra {:?}",
            expected.difference(&found).collect::<Vec<_>>(),
            found.difference(&expected).collect::<Vec<_>>()
        ));
    }
    Ok(())
}

fn expand(value: &Value, symbols: &BTreeMap<String, Value>, spec: &Specification) -> Result<Value> {
    Ok(match value {
        Value::String(s) if s.starts_with('$') => symbols
            .get(&s[1..])
            .cloned()
            .ok_or_else(|| format!("unresolved symbol {s}"))?,
        Value::String(s) if s == "@apiVersion" => json!(spec.names().api_version),
        Value::String(s) if s.starts_with("@core#") => {
            json!(format!("{}#{}", spec.names().core_package, &s[6..]))
        }
        Value::Array(a) => Value::Array(
            a.iter()
                .map(|v| expand(v, symbols, spec))
                .collect::<Result<_>>()?,
        ),
        Value::Object(o) => Value::Object(
            o.iter()
                .map(|(k, v)| {
                    let key = if let Some(local) = k.strip_prefix("@core#") {
                        format!("{}#{local}", spec.names().core_package)
                    } else {
                        k.clone()
                    };
                    Ok((key, expand(v, symbols, spec)?))
                })
                .collect::<Result<_>>()?,
        ),
        v => v.clone(),
    })
}

fn diagnostic(d: &Diagnostic) -> Value {
    let mut location = json!({});
    if let Some(pointer) = &d.pointer {
        location["logicalLocations"] = json!([{"fullyQualifiedName":pointer}]);
    }
    if let Some(file) = &d.file {
        location["physicalLocation"] = json!({"artifactLocation":{"uri":file}});
    }
    if let Some(position) = d.position {
        location["physicalLocation"]["region"] =
            json!({"startLine":position.line,"startColumn":position.column});
    }
    let mut result = json!({"ruleId":d.rule,"level":"error","message":{"text":d.message},
        "locations":[location],"properties":{"file":d.file,"document":d.document}});
    if let Some(keyword) = d.keyword_location() {
        result["properties"]["keywordLocation"] = json!(keyword);
    }
    if let Some(absolute) = d.absolute_keyword_location() {
        result["properties"]["absoluteKeywordLocation"] = json!(absolute);
    }
    result
}

/// Follow JSON Schema's evaluation pointer, resolving each by-reference applicator rather
/// than treating its traversal as a pointer into one physical schema file.
fn keyword_value(spec: &Specification, root: &str, location: &str) -> Result<Value> {
    let mut document = spec
        .contract(root)
        .ok_or("unknown root Schema contract")?
        .clone();
    let mut current = json!({"$ref":"#"});
    for token in location
        .strip_prefix('/')
        .ok_or("keywordLocation is not a pointer")?
        .split('/')
    {
        if token == "$ref" {
            let reference = current["$ref"]
                .as_str()
                .ok_or("keywordLocation traverses a missing $ref")?;
            current = if let Some(pointer) = reference.strip_prefix('#') {
                document
                    .pointer(pointer)
                    .ok_or("unresolved schema pointer")?
                    .clone()
            } else {
                let name = reference
                    .strip_prefix(&format!("{}#", spec.names().core_package))
                    .ok_or("unsupported schema reference")?;
                document = spec
                    .contract(name)
                    .ok_or("unresolved Schema contract")?
                    .clone();
                document.clone()
            };
        } else {
            current = current
                .pointer(&format!("/{token}"))
                .ok_or("keywordLocation reaches no schema keyword")?
                .clone();
        }
    }
    Ok(current)
}

fn run_case(
    path: &Path,
    case: &Case,
    spec: &Specification,
    context: &Context,
    known: &BTreeSet<String>,
) -> Result<Vec<Value>> {
    if case.profile != "first-build" || case.title.is_empty() || case.layers.is_empty() {
        return Err("unsupported profile or initial recipe".into());
    }
    let base = path.parent().ok_or("case has no directory")?;
    let mut state = match &case.initial {
        Initial::Empty => State::default(),
        Initial::Snapshot { file } => {
            let path = case.files.get(file).ok_or("undeclared initial snapshot")?;
            let value = expand(&read_value(&base.join(path))?, &BTreeMap::new(), spec)?;
            serde_json::from_value(value).map_err(|e| format!("invalid state snapshot: {e}"))?
        }
    };
    let mut storage = vec![];
    let mut symbols = BTreeMap::new();
    let checks = checks::Checks::new(spec)?;
    for (index, raw_step) in case.steps.iter().enumerate() {
        let run = || -> Result<(Value, State)> {
            let input = expand(&raw_step["input"], &symbols, spec)?;
            let kind = input["kind"].as_str().ok_or("input has no kind")?;
            let (actual, next) = steps::run(&steps::Step {
                base,
                case,
                input: &input,
                kind,
                state: &state,
                context,
                spec,
                symbols: &symbols,
            })?;
            checks.run(&input, &actual, &state, &next, context, known)?;
            Ok((actual, next))
        };
        let (actual, next) = run().map_err(|e| format!("step {}: {e}", index + 1))?;
        expect::check(raw_step, &symbols, spec, &state, &next, &actual, index)?;
        if let Some(captures) = raw_step["capture"].as_object() {
            for (name, pointer) in captures {
                if symbols.contains_key(name) {
                    return Err(format!("symbol {name} rebound"));
                }
                let value = actual
                    .pointer(pointer.as_str().ok_or("invalid capture pointer")?)
                    .ok_or("capture pointer missing")?;
                symbols.insert(name.clone(), value.clone());
            }
        }
        if actual.get("decisions").is_some() {
            let bytes = |v: &Vec<Value>| -> Result<i64> {
                Ok(serde_json::to_vec(v).map_err(|e| e.to_string())?.len() as i64)
            };
            storage.push(json!({"step":index+1,"input":raw_step["input"]["kind"],"records_bytes":bytes(&next.records)?,"record_gap_bytes":bytes(&next.record_gaps)?,"invocation_index_bytes":bytes(&next.invocations)?,"records_growth":bytes(&next.records)?-bytes(&state.records)?,"invocation_index_growth":bytes(&next.invocations)?-bytes(&state.invocations)?,"invocations":next.invocations.iter().map(|v| json!({"id":v["id"],"keyed":v["key"].is_string(),"state":v["state"],"bytes":serde_json::to_vec(v).unwrap().len()})).collect::<Vec<_>>()}));
        }
        state = next;
    }
    Ok(storage)
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    #[test]
    fn thread_count_requires_positive_ascii_integers() {
        for value in [
            "",
            "0",
            "-1",
            "+1",
            " 1",
            "1 ",
            "1.0",
            "no",
            "999999999999999999999999",
        ] {
            assert!(
                vector_threads(Ok(value.into()))
                    .unwrap_err()
                    .contains("CONDUIT_VECTOR_THREADS")
            );
        }
        assert_eq!(vector_threads(Ok("1".into())).unwrap(), 1);
        assert_eq!(vector_threads(Ok("4".into())).unwrap(), 4);
        assert_eq!(
            vector_threads(Err(std::env::VarError::NotPresent)).unwrap(),
            usize::from(std::thread::available_parallelism().unwrap())
        );
        assert!(
            vector_threads(Err(std::env::VarError::NotUnicode(
                std::ffi::OsString::from("invalid")
            )))
            .unwrap_err()
            .contains("CONDUIT_VECTOR_THREADS")
        );
    }

    #[test]
    fn unexplained_missing_and_false_coverage_fail() {
        let known = BTreeSet::from(["ACC-001".into(), "IFC-045".into()]);
        let inventory = json!(["ACC-001", "IFC-045"]);
        let entries = vec![json!({"id":"access","promises":["ACC-001"]})];
        let valid = json!({"s1_coverage":[{"promise":"ACC-001","vectors":["access"]},
            {"promise":"IFC-045","vectors":[],"reason":"Protocol negotiation is shell work."}]});
        assert!(validate_s1_coverage(&valid, &entries, &inventory, &known).is_ok());
        for invalid in [
            json!([]),
            json!(["ACC-001", "ACC-001"]),
            json!(["BAD-001"]),
            json!([1]),
            json!({}),
        ] {
            assert!(validate_s1_coverage(&valid, &entries, &invalid, &known).is_err());
        }
        let mut missing = valid.clone();
        missing["s1_coverage"].as_array_mut().unwrap().pop();
        assert!(validate_s1_coverage(&missing, &entries, &inventory, &known).is_err());
        let mut unexplained = valid.clone();
        unexplained["s1_coverage"][1]["reason"] = json!(" ");
        assert!(validate_s1_coverage(&unexplained, &entries, &inventory, &known).is_err());
        let mut false_claim = valid;
        false_claim["s1_coverage"][1] = json!({"promise":"IFC-045","vectors":["access"]});
        assert!(validate_s1_coverage(&false_claim, &entries, &inventory, &known).is_err());
    }
}
