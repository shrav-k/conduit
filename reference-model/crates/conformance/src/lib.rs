//! File input, case validation and assertions. This crate owns all I/O for the pure core.
mod packages;
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
        core_package: spec.names().core_package.clone(),
        templates: BTreeMap::new(),
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
    let mut inventory_ids = BTreeSet::new();
    for entry in inventory {
        let id = entry["id"]
            .as_str()
            .ok_or("coverage index entry has no ID")?;
        if !inventory_ids.insert(id.to_string()) {
            return Err("duplicate coverage index ID".into());
        }
    }
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
        run_case(&path, &case, &spec, &context, &known_promises)
            .map_err(|e| format!("{} [{}]: {e}", case.id, case.promises.join(", ")))?;
        reports.push(CaseReport {
            id: case.id,
            promises: case.promises,
            steps: case.steps.len(),
        });
    }
    if ids != inventory_ids {
        return Err("coverage index lists missing cases".into());
    }
    Ok(reports)
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
                .map(|(k, v)| Ok((k.clone(), expand(v, symbols, spec)?)))
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
) -> Result<()> {
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
    let mut symbols = BTreeMap::new();
    let record_validator = spec
        .contract_validator("record.v1")
        .map_err(|e| e.to_string())?;
    let diagnostic_validator = spec
        .contract_validator("diagnostic.v1")
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
                for r in actual["records"]
                    .as_array()
                    .ok_or("transition has no records")?
                {
                    record_validator
                        .validate(r)
                        .map_err(|e| format!("record.v1: {e}"))?;
                }
                let appended = actual["records"].as_array().ok_or("missing records")?;
                let mut retained = state.records.clone();
                retained.extend(appended.iter().cloned());
                if next.records != retained {
                    return Err(
                        "REC-003: records were changed or omitted from the transition".into(),
                    );
                }
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
                    return Err(format!("uncited expected promise {rule}"));
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
            .map_err(|e| format!("step {}: {e}", index + 1))?;
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
        state = next;
    }
    Ok(())
}
