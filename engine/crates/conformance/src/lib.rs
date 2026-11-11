//! File input, case validation and assertions. This crate owns all I/O for the pure core.
mod access;
mod packages;
mod records;
mod schemas;
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

#[derive(Debug)]
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
        } else if path.file_name().is_some_and(|s| s == "case.yaml") {
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
    let mut cases = vec![];
    paths(&repository.join("conformance"), &mut cases)?;
    cases.sort();
    if cases.is_empty() {
        return Err("no conformance cases found".into());
    }
    let spec = Specification::read(&repository.join("spec")).map_err(|e| e.to_string())?;
    let schema = read_value(&repository.join("conformance/case.schema.json"))?;
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
    let mut reports = vec![];
    let manifest = read_value(&repository.join("conformance/manifest.yaml"))?;
    let inventory = manifest["cases"]
        .as_array()
        .ok_or("coverage index has no cases")?;
    let s1_promises = read_value(&repository.join("conformance/s1-promises.yaml"))?;
    validate_s1_coverage(&manifest, &s1_promises, &known_promises)?;
    let mut inventory_ids = BTreeSet::new();
    for entry in inventory {
        let id = entry["id"]
            .as_str()
            .ok_or("coverage index entry has no ID")?;
        if !inventory_ids.insert(id.to_string()) {
            return Err("duplicate coverage index ID".into());
        }
    }
    let mut failures = vec![];
    for path in cases {
        let value = read_value(&path)?;
        validator
            .validate(&value)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        let case: Case = serde_json::from_value(value).map_err(|e| e.to_string())?;
        if !ids.insert(case.id.clone()) {
            return Err(format!("duplicate case ID {}", case.id));
        }
        let entry = inventory
            .iter()
            .find(|entry| entry["id"] == case.id)
            .ok_or("case missing from coverage index")?;
        let relative = path
            .strip_prefix(repository.join("conformance"))
            .map_err(|e| e.to_string())?;
        if entry["case"] != relative.to_string_lossy().as_ref()
            || entry["promises"] != json!(case.promises)
        {
            return Err(format!("{}: coverage index differs from case", case.id));
        }
        for promise in &case.promises {
            if !known_promises.contains(promise) {
                return Err(format!("{}: unknown promise {promise}", case.id));
            }
        }
        if !case.id.starts_with(prefix) {
            continue;
        }
        let storage = match run_case(&path, &case, &spec, &context, &known_promises) {
            Ok(storage) => storage,
            Err(error) => {
                failures.push(format!(
                    "{} [{}]: {error}",
                    case.id,
                    case.promises.join(", ")
                ));
                continue;
            }
        };
        reports.push(CaseReport {
            id: case.id,
            promises: case.promises,
            steps: case.steps.len(),
            storage,
        });
    }
    if ids != inventory_ids {
        return Err("coverage index lists missing cases".into());
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
            let case = manifest["cases"]
                .as_array()
                .ok_or("coverage index has no cases")?
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

/// Maps are required-field projections; arrays are exact ordered sets/sequences. An empty
/// map asserts emptiness. Objects are compared in supplied allocation order, records in source order.
fn projection(actual: &Value, expected: &Value, pointer: &str) -> Result<()> {
    match (actual, expected) {
        (Value::Object(a), Value::Object(e)) if !e.is_empty() => {
            for (key, value) in e {
                projection(
                    a.get(key)
                        .ok_or_else(|| format!("{pointer}/{key}: missing required field"))?,
                    value,
                    &format!("{pointer}/{key}"),
                )?;
            }
            Ok(())
        }
        (Value::Object(a), Value::Object(e)) if e.is_empty() && !a.is_empty() => Err(format!(
            "{pointer}: expected an empty map, found {} fields",
            a.len()
        )),
        (Value::Array(a), Value::Array(e)) if a.len() == e.len() => {
            for (i, (av, ev)) in a.iter().zip(e).enumerate() {
                projection(av, ev, &format!("{pointer}/{i}"))?;
            }
            Ok(())
        }
        (Value::Array(a), Value::Array(e)) => Err(format!(
            "{pointer}: expected {} items, found {}",
            e.len(),
            a.len()
        )),
        (a, e) if a == e => Ok(()),
        (a, e) => Err(format!("{pointer}: expected {e}, found {a}")),
    }
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
    let record_validator = spec
        .contract_validator("record.v1")
        .map_err(|e| e.to_string())?;
    let diagnostic_validator = spec
        .contract_validator("diagnostic.v1")
        .map_err(|e| e.to_string())?;
    let condition_validator = spec
        .contract_validator("condition.v1")
        .map_err(|e| e.to_string())?;
    for (index, raw_step) in case.steps.iter().enumerate() {
        let run = || -> Result<(Value, State)> {
            let input = expand(&raw_step["input"], &symbols, spec)?;
            let kind = input["kind"].as_str().ok_or("input has no kind")?;
            let file_bytes = |alias: &str| -> Result<(String, Vec<u8>)> {
                let name = case
                    .files
                    .get(alias)
                    .ok_or_else(|| format!("undeclared file {alias}"))?;
                Ok((
                    alias.into(),
                    std::fs::read(base.join(name)).map_err(|e| format!("{name}: {e}"))?,
                ))
            };
            let mut next = state.clone();
            let actual = match kind {
                "request-fixture" | "lifecycle-fixture" => {
                    let alias = input["file"].as_str().ok_or("missing request fixture")?;
                    let path = case.files.get(alias).ok_or("undeclared request fixture")?;
                    let fixtures = read_value(&base.join(path))?;
                    let index = input["index"]
                        .as_u64()
                        .ok_or("missing request observation index")?
                        as usize;
                    let fixture = expand(
                        fixtures["requests"]
                            .get(index)
                            .ok_or("request observation out of range")?,
                        &symbols,
                        spec,
                    )?;
                    access::patch(&mut next, &fixture["patches"])?;
                    if kind == "lifecycle-fixture"
                        && let Some(invocations) = fixture.get("invocations")
                    {
                        next.invocations = serde_json::from_value(invocations.clone())
                            .map_err(|e| e.to_string())?;
                    }
                    for update in fixture["contract_updates"].as_array().into_iter().flatten() {
                        let object = next
                            .objects
                            .iter_mut()
                            .find(|o| o["id"] == update["target"])
                            .ok_or("contract observation target missing")?;
                        let mut found = false;
                        for scope in object["held"]["environment"]["scopes"]
                            .as_object_mut()
                            .ok_or("missing scopes")?
                            .values_mut()
                        {
                            for doc in scope["contracts"]
                                .as_object_mut()
                                .ok_or("missing contracts")?
                                .values_mut()
                            {
                                if doc["kind"].as_str()
                                    == Some(update["kind"].as_str().unwrap_or("Operation"))
                                    && doc["metadata"]["name"] == update["name"]
                                {
                                    for (k, v) in update["set"]
                                        .as_object()
                                        .ok_or("missing contract update")?
                                    {
                                        doc["spec"][k] = v.clone();
                                    }
                                    found = true;
                                }
                            }
                        }
                        if !found {
                            return Err("contract observation names no matching contract".into());
                        }
                    }
                    for update in fixture["implementation_updates"]
                        .as_array()
                        .into_iter()
                        .flatten()
                    {
                        let object = next
                            .objects
                            .iter_mut()
                            .find(|o| o["id"] == update["target"])
                            .ok_or("implementation observation target missing")?;
                        object["resolved_implementations"][update["operation"]
                            .as_str()
                            .ok_or("missing observed operation")?] =
                            update["implementation"].clone();
                    }
                    if fixture["aware"] == true {
                        let object = next
                            .objects
                            .iter_mut()
                            .find(|o| o["id"] == fixture["target"])
                            .ok_or("aware observation target missing")?;
                        for implementation in object["resolved_implementations"]
                            .as_object_mut()
                            .ok_or("missing implementations")?
                            .values_mut()
                        {
                            implementation["item"]["plain"] = json!(false);
                        }
                    }
                    let mut call = fixture["call"].clone();
                    if let Some(alias) = fixture["archive_file"].as_str() {
                        let (_, bytes) = file_bytes(alias)?;
                        call["archive"]["completion"] = json!({"status":"complete","bytes":bytes});
                    }
                    let t = conduit_core::step(
                        &next,
                        serde_json::from_value(call).map_err(|e| e.to_string())?,
                        context,
                    )?;
                    if kind == "request-fixture"
                        && (t.state.objects != next.objects
                            || t.state.installed_packages != next.installed_packages)
                    {
                        return Err("request changed objects or packages outside its scope".into());
                    }
                    next = t.state.clone();
                    serde_json::to_value(t).map_err(|e| e.to_string())?
                }
                "restore-state" => {
                    // A boundary snapshot after restart, optionally observing records already
                    // purged by the records service. This does not implement purge.v1.
                    let bytes = serde_json::to_vec(&state).map_err(|e| e.to_string())?;
                    next = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                    if input["records_purged"] == true {
                        next.record_gaps.push(json!({"cause":"purged-observation","records":next.records.iter().map(|r| json!({"source":r["source"],"id":r["id"]})).collect::<Vec<_>>()}));
                        next.records.clear();
                    }
                    let evidence = input
                        .get("evidence")
                        .cloned()
                        .unwrap_or_else(|| json!({"time":"2026-11-09T00:00:00Z","ids":[]}));
                    if next.invocations.iter().any(|v| v["state"] == "in-progress")
                        && input.get("evidence").is_none()
                    {
                        return Err("interrupted recovery requires supplied record evidence".into());
                    }
                    let t = conduit_core::step(
                        &next,
                        Input::Recover {
                            evidence: serde_json::from_value(evidence)
                                .map_err(|e| e.to_string())?,
                        },
                        context,
                    )?;
                    next = t.state.clone();
                    serde_json::to_value(t).map_err(|e| e.to_string())?
                }
                "access-fixture" => {
                    let alias = input["file"].as_str().ok_or("missing access fixture")?;
                    let path = case.files.get(alias).ok_or("undeclared access fixture")?;
                    let observations = read_value(&base.join(path))?;
                    let index =
                        input["index"].as_u64().ok_or("missing observation index")? as usize;
                    let observation = observations["observations"]
                        .as_array()
                        .and_then(|a| a.get(index))
                        .ok_or("observation index out of range")?;
                    let fixture = expand(observation, &symbols, spec)?;
                    let t = access::run(&fixture, &state, context)?;
                    next = t.state.clone();
                    serde_json::to_value(t).map_err(|e| e.to_string())?
                }

                "install-archive" => {
                    let t = packages::install(base, &input, &case.files, &state, spec, context)?;
                    next = t.state.clone();
                    serde_json::to_value(t).map_err(|e| e.to_string())?
                }
                "resolve-project" | "register-project" | "registration-orders" => {
                    let mut t = packages::run(
                        base,
                        input["file"].as_str().ok_or("missing context fixture")?,
                        &case.files,
                        &state,
                        spec,
                        context,
                        (kind != "resolve-project").then_some((
                            &input["evidence"],
                            &input["request"],
                            usize::from(input["reverse"] == true),
                        )),
                    )?;
                    if kind == "registration-orders" {
                        let original = serde_json::to_value(&t).map_err(|e| e.to_string())?;
                        for order in [1, 2] {
                            let other = packages::run(
                                base,
                                input["file"].as_str().unwrap(),
                                &case.files,
                                &state,
                                spec,
                                context,
                                Some((&input["evidence"], &input["request"], order)),
                            )?;
                            if serde_json::to_value(other).map_err(|e| e.to_string())? != original {
                                return Err(
                                    "SUB-001: registration differs by document/file order".into()
                                );
                            }
                        }
                        t.reply["orders_compared"] = json!(3);
                    }
                    next = t.state.clone();
                    serde_json::to_value(t).map_err(|e| e.to_string())?
                }
                "read" => {
                    let alias = input["file"].as_str().ok_or("missing file")?;
                    let (_, bytes) = file_bytes(alias)?;
                    let syntax = match input["syntax"].as_str() {
                        Some("yaml") => Syntax::Yaml,
                        Some("json") => Syntax::Json,
                        _ => return Err("unsupported syntax".into()),
                    };
                    match read_bytes(syntax, &bytes) {
                        Ok(documents) => {
                            json!({"accepted":true,"diagnostics":[],"document_count":documents.len(),
                            "documents":documents.iter().map(|d| d.to_json()).collect::<Vec<_>>()})
                        }
                        Err(errors) => {
                            json!({"accepted":false,"diagnostics":errors.0.iter().map(diagnostic).collect::<Vec<_>>()})
                        }
                    }
                }
                "load" | "submit-files" => {
                    let files = input["files"]
                        .as_array()
                        .ok_or("missing files")?
                        .iter()
                        .map(|v| file_bytes(v.as_str().ok_or("invalid file alias")?))
                        .collect::<Result<Vec<_>>>()?;
                    match read_submitted_files(&files, spec) {
                        Ok(project) if kind == "submit-files" => {
                            let mut core_input = input.clone();
                            core_input["kind"] = json!("submit");
                            core_input
                                .as_object_mut()
                                .ok_or("invalid input")?
                                .remove("files");
                            core_input["documents"] = json!(
                                project
                                    .documents
                                    .iter()
                                    .map(|d| d.root.to_json())
                                    .collect::<Vec<_>>()
                            );
                            let t = conduit_core::step(
                                &state,
                                serde_json::from_value(core_input).map_err(|e| e.to_string())?,
                                context,
                            )?;
                            next = t.state.clone();
                            serde_json::to_value(t).map_err(|e| e.to_string())?
                        }
                        Ok(project) => {
                            json!({"accepted":true,"diagnostics":[],"document_count":project.documents.len(),"manifest":project.manifest.is_some()})
                        }
                        Err(errors) if kind == "load" => {
                            json!({"accepted":false,"diagnostics":errors.0.iter().map(diagnostic).collect::<Vec<_>>()})
                        }
                        Err(errors) => {
                            return Err(format!(
                                "protected submission failed at loader instead of core: {errors}"
                            ));
                        }
                    }
                }
                "schema-case" | "schema-admission" | "schema-evaluation" => {
                    let alias = input["file"].as_str().ok_or("missing schema fixture")?;
                    let file = case.files.get(alias).ok_or("undeclared schema fixture")?;
                    let fixture = expand(&read_value(&base.join(file))?, &symbols, spec)?;
                    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
                    schemas::run(&repository, &fixture, kind, spec, context)?
                }
                "validate-values" => {
                    let (_, bytes) = file_bytes(input["file"].as_str().ok_or("missing file")?)?;
                    let documents = read_bytes(Syntax::Json, &bytes).map_err(|e| e.to_string())?;
                    let validator =
                        conduit_loader::compile(&input["schema"]).map_err(|e| e.to_string())?;
                    json!({"valid":documents.iter().map(|d| validator.is_valid(&d.to_json())).collect::<Vec<_>>()})
                }
                _ => {
                    let typed: Input = serde_json::from_value(input.clone())
                        .map_err(|e| format!("unsupported core input: {e}"))?;
                    let t = conduit_core::step(&state, typed, context)?;
                    next = t.state.clone();
                    serde_json::to_value(t).map_err(|e| e.to_string())?
                }
            };
            // Boundary evidence validation also applies to submissions assembled above.
            if let Some(evidence) = input.get("evidence") {
                let uuid = conduit_loader::compile(&json!({"type":"string","format":"uuid"}))
                    .map_err(|e| e.to_string())?;
                for id in evidence["ids"].as_array().ok_or("missing allocation IDs")? {
                    uuid.validate(id).map_err(|e| e.to_string())?;
                }
                conduit_loader::compile(&json!({"type":"string","format":"date-time"}))
                    .map_err(|e| e.to_string())?
                    .validate(&evidence["time"])
                    .map_err(|e| e.to_string())?;
            }
            if let Some(decisions) = actual["decisions"].as_array() {
                for d in decisions {
                    let rules = d["promises"].as_array().ok_or("decision has no promises")?;
                    if rules.is_empty()
                        || rules
                            .iter()
                            .any(|r| r.as_str().is_none_or(|s| !known.contains(s)))
                    {
                        return Err("decision lacks a requiring known promise".into());
                    }
                }
                for r in next.records.iter().chain(
                    actual["records"]
                        .as_array()
                        .ok_or("transition has no records")?
                        .iter(),
                ) {
                    record_validator
                        .validate(r)
                        .map_err(|e| format!("record.v1: {e}"))?;
                    if ![
                        "access-decision",
                        "request-ended",
                        "invocation-dispatched",
                        "invocation-ended",
                        "provider-call",
                        "object-change",
                    ]
                    .iter()
                    .any(|e| r["type"] == *e)
                    {
                        return Err("undocumented record event type".into());
                    }
                    if r["type"] == "provider-call" {
                        for field in [
                            "serving",
                            "conduit",
                            "consumer",
                            "idempotencyKey",
                            "filters",
                        ] {
                            if r["data"].get(field).is_some() {
                                return Err(format!(
                                    "PRV-005 / REC-002: provider record has {field}"
                                ));
                            }
                        }
                    }
                }
                let appended = actual["records"].as_array().ok_or("missing records")?;
                let mut retained = if kind == "restore-state" && input["records_purged"] == true {
                    vec![]
                } else {
                    state.records.clone()
                };
                // A successful purge is the only core transition permitted to remove records.
                let purged: BTreeSet<String> = appended
                    .iter()
                    .filter(|r| r["data"]["action"] == "records-purged")
                    .flat_map(|r| r["data"]["purged"].as_array().into_iter().flatten())
                    .map(|r| format!("{}:{}", r["source"], r["id"]))
                    .collect();
                retained.extend(appended.iter().cloned());
                retained.retain(|r| !purged.contains(&format!("{}:{}", r["source"], r["id"])));
                if next.records != retained {
                    return Err(
                        "REC-003: records were changed or omitted from the transition".into(),
                    );
                }
            }
            if actual.get("decisions").is_some() {
                records::transition(&actual, &state, &next, context, input["key"].as_str())?;
            }
            for condition in next
                .objects
                .iter()
                .flat_map(|o| o["status"]["conditions"].as_array().into_iter().flatten())
            {
                condition_validator
                    .validate(condition)
                    .map_err(|e| format!("condition.v1: {e}"))?;
            }
            for diagnostics in [
                actual.get("diagnostics"),
                actual.get("reply").and_then(|r| r.get("diagnostics")),
            ]
            .into_iter()
            .flatten()
            {
                for d in diagnostics.as_array().ok_or("invalid diagnostic array")? {
                    diagnostic_validator
                        .validate(d)
                        .map_err(|e| format!("diagnostic.v1: {e}"))?;
                }
            }
            Ok((actual, next))
        };
        let (actual, next) = run().map_err(|e| format!("step {}: {e}", index + 1))?;
        let expected = expand(&raw_step["expect"], &symbols, spec)?;
        let mut fields = expected.as_object().ok_or("missing expectation")?.clone();
        if let Some(count) = fields.remove("object_count") {
            projection(&json!(next.objects.len()), &count, "/object_count")?;
        }
        for (key, values) in [
            ("object_facts", &next.objects),
            ("invocation_facts", &next.invocations),
        ] {
            if let Some(facts) = fields.remove(key) {
                for fact in facts.as_array().ok_or("facts must be an array")? {
                    let found = values
                        .iter()
                        .find(|v| {
                            fact.get("id").is_some_and(|id| v["id"] == *id)
                                || fact
                                    .get("name")
                                    .is_some_and(|n| v["name"] == *n && v.get("spec").is_none())
                        })
                        .ok_or_else(|| format!("{key}: missing {fact}"))?;
                    projection(found, fact, key)?;
                }
            }
        }
        for key in ["record_facts", "stored_record_facts"] {
            if let Some(facts) = fields.remove(key) {
                let mut cursor = 0;
                let records = if key == "stored_record_facts" {
                    &next.records
                } else {
                    actual["records"].as_array().ok_or("missing records")?
                };
                for fact in facts.as_array().ok_or("record facts must be an array")? {
                    let found = records[cursor..]
                        .iter()
                        .position(|r| projection(r, fact, key).is_ok())
                        .ok_or_else(|| format!("missing ordered record {fact}"))?;
                    cursor += found + 1;
                }
            }
        }
        if let Some(absent) = fields.remove("absent_objects") {
            for id in absent.as_array().ok_or("absent_objects must be an array")? {
                if next.objects.iter().any(|o| o["id"] == *id) {
                    return Err(format!("unexpected object {id}"));
                }
            }
        }
        if let Some(unchanged) = fields.remove("stateUnchanged") {
            if unchanged != true {
                return Err("stateUnchanged must be true when present".into());
            }
            if next.objects != state.objects || next.installed_packages != state.installed_packages
            {
                return Err(format!("step {}: objects changed", index + 1));
            }
        }
        if let Some(absent) = fields.remove("absent") {
            for pointer in absent.as_array().ok_or("invalid absent pointers")? {
                let pointer = pointer.as_str().ok_or("invalid pointer")?;
                if actual.pointer(pointer).is_some() {
                    return Err(format!("unexpected field {pointer}"));
                }
            }
        }
        if let Some(rules) = fields.remove("required_promises") {
            let cited: BTreeSet<&str> = actual["decisions"]
                .as_array()
                .ok_or("no decisions")?
                .iter()
                .flat_map(|d| {
                    d["promises"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_str)
                })
                .collect();
            for rule in rules.as_array().ok_or("invalid required promises")? {
                if !cited.contains(rule.as_str().ok_or("invalid rule")?) {
                    return Err(format!(
                        "uncited expected promise {rule}; reply: {}",
                        actual["reply"]
                    ));
                }
            }
        }
        if let Some(assertions) = fields.remove("diagnostic_facts") {
            for assertion in assertions.as_array().ok_or("invalid diagnostic facts")? {
                let n = assertion["index"]
                    .as_u64()
                    .ok_or("diagnostic index missing")? as usize;
                let d = actual["diagnostics"]
                    .get(n)
                    .ok_or("expected diagnostic absent")?;
                for word in assertion["message_contains"]
                    .as_array()
                    .ok_or("missing message assertions")?
                {
                    if !d["message"]["text"]
                        .as_str()
                        .is_some_and(|s| s.contains(word.as_str().unwrap_or("\0")))
                    {
                        return Err(format!("diagnostic does not state {word}"));
                    }
                }
                if assertion["keyword_location"] == true {
                    let pointer = d["properties"]["keywordLocation"]
                        .as_str()
                        .ok_or("keywordLocation absent")?;
                    if !pointer.starts_with('/') {
                        return Err("keywordLocation is not a pointer".into());
                    }
                    let keyword = pointer.rsplit('/').next().ok_or("empty keywordLocation")?;
                    if assertion["keyword"] != keyword {
                        return Err(format!(
                            "expected keyword {}, got {keyword}",
                            assertion["keyword"]
                        ));
                    }
                    // The fixture states the portable contract/keyword, not a bundler path.
                    let contract = assertion["contract"].as_str().ok_or("missing contract")?;
                    let schema = spec
                        .contract(contract)
                        .ok_or("unknown diagnostic contract")?;
                    let expected_keyword = schema
                        .pointer(
                            assertion["pointer"]
                                .as_str()
                                .ok_or("missing keyword pointer")?,
                        )
                        .ok_or("assertion points to no contract keyword")?;
                    let root = assertion["root_schema"]
                        .as_str()
                        .ok_or("missing pinned root schema")?;
                    if &keyword_value(spec, root, pointer)? != expected_keyword {
                        return Err(
                            "keywordLocation does not reach the asserted failing keyword".into(),
                        );
                    }
                }
            }
        }
        projection(&actual, &Value::Object(fields), "")
            .map_err(|e| format!("step {}: {e}; reply: {}", index + 1, actual["reply"]))?;
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
    fn unexplained_missing_and_false_coverage_fail() {
        let known = BTreeSet::from(["ACC-001".into(), "IFC-045".into()]);
        let inventory = json!(["ACC-001", "IFC-045"]);
        let valid = json!({"cases":[{"id":"access","promises":["ACC-001"]}],
            "s1_coverage":[{"promise":"ACC-001","vectors":["access"]},
            {"promise":"IFC-045","vectors":[],"reason":"Protocol negotiation is shell work."}]});
        assert!(validate_s1_coverage(&valid, &inventory, &known).is_ok());
        for invalid in [
            json!([]),
            json!(["ACC-001", "ACC-001"]),
            json!(["BAD-001"]),
            json!([1]),
            json!({}),
        ] {
            assert!(validate_s1_coverage(&valid, &invalid, &known).is_err());
        }
        let mut missing = valid.clone();
        missing["s1_coverage"].as_array_mut().unwrap().pop();
        assert!(validate_s1_coverage(&missing, &inventory, &known).is_err());
        let mut unexplained = valid.clone();
        unexplained["s1_coverage"][1]["reason"] = json!(" ");
        assert!(validate_s1_coverage(&unexplained, &inventory, &known).is_err());
        let mut false_claim = valid;
        false_claim["s1_coverage"][1] = json!({"promise":"IFC-045","vectors":["access"]});
        assert!(validate_s1_coverage(&false_claim, &inventory, &known).is_err());
    }
}
