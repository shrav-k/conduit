//! E05's independent authored/published verdict comparison and schema admission boundary.
use super::*;
use conduit_core::schemas::{Environment, Scope};

pub(super) fn environment(
    repository: &Path,
    fixture: &Value,
    spec: &Specification,
) -> Result<Environment> {
    let mut env: Environment =
        serde_json::from_value(fixture["environment"].clone()).map_err(|e| e.to_string())?;
    let mut contracts = BTreeMap::new();
    for directory in ["spec/schemas", "spec/core"] {
        for entry in std::fs::read_dir(repository.join(directory)).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_none_or(|e| e != "yaml")
                || path.file_name().is_some_and(|n| n == "conduit.yaml")
            {
                continue;
            }
            for document in read_file(&path).map_err(|e| e.to_string())? {
                let value = document.root.to_json();
                if ["Schema", "Operation", "Type", "Capability", "Requirement"]
                    .iter()
                    .any(|k| value["kind"] == *k)
                {
                    contracts.insert(
                        value["metadata"]["name"]
                            .as_str()
                            .ok_or("missing contract name")?
                            .into(),
                        value,
                    );
                }
            }
        }
    }
    env.scopes.insert(
        "core".into(),
        Scope {
            address: spec.names().core_package.clone(),
            version: None,
            manifest: BTreeMap::new(),
            contracts,
            exports: None,
        },
    );
    for scope in env.scopes.values() {
        for value in scope.contracts.values() {
            let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
            let docs = read_bytes(Syntax::Json, &bytes).map_err(|e| e.to_string())?;
            let document = conduit_loader::Document {
                root: docs[0].clone(),
                file: "snapshot.json".into(),
                index: 1,
            };
            spec.check(&document, conduit_loader::Role::Document)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(env)
}

fn fields(value: &Value, allowed: &[&str], required: &[&str]) -> Result<()> {
    let map = value
        .as_object()
        .ok_or("schema fixture metadata must be a map")?;
    if let Some(key) = map.keys().find(|key| !allowed.contains(&key.as_str())) {
        return Err(format!("unknown schema fixture field {key}"));
    }
    if let Some(key) = required.iter().find(|key| !map.contains_key(**key)) {
        return Err(format!("missing schema fixture field {key}"));
    }
    Ok(())
}

pub(super) fn run(
    repository: &Path,
    fixture: &Value,
    kind: &str,
    spec: &Specification,
    context: &Context,
) -> Result<Value> {
    run_with_step(
        repository,
        fixture,
        kind,
        spec,
        context,
        &mut conduit_core::step,
    )
}

fn run_with_step(
    repository: &Path,
    fixture: &Value,
    kind: &str,
    spec: &Specification,
    context: &Context,
    step: &mut impl FnMut(&State, Input, &Context) -> Result<conduit_core::Transition>,
) -> Result<Value> {
    // These schema operations are read-only. Compare each actual returned state to the
    // exact snapshot used by that call, not the outer runner's unrelated state.
    let mut checked_step =
        |state: &State, input: Input, context: &Context| -> Result<conduit_core::Transition> {
            let transition = step(state, input, context)?;
            if transition.state.objects != state.objects {
                return Err("stateUnchanged: schema transition changed objects".into());
            }
            Ok(transition)
        };
    let required: &[&str] = match kind {
        "schema-case" => &[
            "environment",
            "state",
            "target",
            "consumer",
            "conduit",
            "operation",
            "authored_scope",
            "samples",
        ],
        "schema-admission" => &["environment", "scope", "documents"],
        "schema-evaluation" => &["environment", "scope", "schema", "samples"],
        _ => return Err("unknown schema fixture kind".into()),
    };
    let mut allowed = required.to_vec();
    if kind == "schema-case" {
        allowed.extend(["copy_count", "narrowings", "template_scope"]);
    }
    fields(fixture, &allowed, required)?;
    if kind == "schema-case" {
        fields(
            &fixture["samples"],
            &["inputSchema", "outputSchema"],
            &["inputSchema", "outputSchema"],
        )?;
        for key in ["inputSchema", "outputSchema"] {
            for sample in fixture["samples"][key]
                .as_array()
                .ok_or("samples must be an array")?
            {
                fields(sample, &["value", "valid"], &["value", "valid"])?;
            }
        }
    } else if kind == "schema-evaluation" {
        for sample in fixture["samples"]
            .as_array()
            .ok_or("samples must be an array")?
        {
            fields(sample, &["value", "mode"], &["value", "mode"])?;
        }
    }
    let env = environment(repository, fixture, spec)?;
    if kind == "schema-admission" {
        // Document syntax/profile checks still belong to the loader; this is the following
        // semantic check, common to installation and submission. No archive is installed.
        for (i, document) in fixture["documents"]
            .as_array()
            .ok_or("missing documents")?
            .iter()
            .enumerate()
        {
            let bytes = serde_json::to_vec(document).map_err(|e| e.to_string())?;
            let docs = read_bytes(Syntax::Json, &bytes).map_err(|e| e.to_string())?;
            let document = conduit_loader::Document {
                root: docs[0].clone(),
                file: "authored.json".into(),
                index: i + 1,
            };
            spec.check(&document, conduit_loader::Role::Document)
                .map_err(|e| e.to_string())?;
        }
        let input = json!({"kind":"check-authored","environment":env,"scope":fixture["scope"],"documents":fixture["documents"]});
        return serde_json::to_value(checked_step(
            &State::default(),
            serde_json::from_value(input).map_err(|e| e.to_string())?,
            context,
        )?)
        .map_err(|e| e.to_string());
    }
    if kind == "schema-evaluation" {
        let mut replies = vec![];
        let mut decisions = vec![];
        for sample in fixture["samples"].as_array().ok_or("missing samples")? {
            let input = json!({"kind":"evaluate-schema","environment":env,"scope":fixture["scope"],"schema":fixture["schema"],"value":sample["value"],"mode":sample["mode"]});
            let t = checked_step(
                &State::default(),
                serde_json::from_value(input).map_err(|e| e.to_string())?,
                context,
            )?;
            replies.push(t.reply);
            decisions.extend(t.decisions);
        }
        return Ok(json!({"reply":{"samples":replies},"decisions":decisions,"records":[]}));
    }
    let mut state: State =
        serde_json::from_value(fixture["state"].clone()).map_err(|e| e.to_string())?;
    let target = state
        .objects
        .iter_mut()
        .find(|o| o["id"] == fixture["target"])
        .ok_or("missing fixture target")?;
    target["schema_environment"] = serde_json::to_value(&env).map_err(|e| e.to_string())?;
    // Schema-only fixtures omit unrelated instance metadata. Supply a complete
    // target for IFC-033's description without weakening the publisher's boundary.
    for (key, default) in [
        ("kind", json!("Resource")),
        ("name", fixture["target"].clone()),
        (
            "type",
            json!(format!("{}#interface.v1", context.core_package)),
        ),
    ] {
        if target.get(key).is_none() {
            target[key] = default;
        }
    }
    let input = json!({"kind":"tool-definition","consumer":fixture["consumer"],"conduit":fixture["conduit"],"operation":fixture["operation"]});
    let mut t = checked_step(
        &state,
        serde_json::from_value(input).map_err(|e| e.to_string())?,
        context,
    )?;
    let mut verdicts = json!({});
    if !t.reply["tool"].is_null() {
        let scope = fixture["authored_scope"]
            .as_str()
            .ok_or("missing authored scope")?;
        let name = fixture["operation"]
            .as_str()
            .ok_or("missing operation")?
            .split_once('#')
            .ok_or("invalid operation")?
            .1;
        let operation = &env.scopes[scope].contracts[name];
        for key in ["inputSchema", "outputSchema"] {
            if let Some(expected) = fixture["copy_count"].as_u64() {
                let count = t.reply["tool"][key]["$defs"]
                    .as_object()
                    .map_or(0, |m| m.keys().filter(|k| k.contains('#')).count());
                if count as u64 != expected {
                    return Err(format!("{key}: expected {expected} copies, got {count}"));
                }
            }
            let Some(samples) = fixture["samples"][key].as_array() else {
                return Err(format!("missing {key} samples"));
            };
            let schema = operation["spec"].get(key).cloned().unwrap_or_else(|| {
                if key == "inputSchema" {
                    json!({"type":"object"})
                } else {
                    json!({"type":"object","additionalProperties":false})
                }
            });
            let mut results = vec![];
            for sample in samples {
                let mut authored_schema = schema.clone();
                if operation["spec"].get("changes").is_some() && key == "inputSchema" {
                    authored_schema["required"] = json!(["revision"]);
                }
                let mut authored = env.valid(
                    scope,
                    &authored_schema,
                    &sample["value"],
                    &context.core_package,
                )?;
                if key == "inputSchema"
                    && let Some(narrowings) = fixture.get("narrowings").and_then(Value::as_object)
                {
                    for (name, narrowing) in narrowings {
                        if let Some(value) = sample["value"].get(name) {
                            authored &= env.valid(
                                fixture["template_scope"]
                                    .as_str()
                                    .ok_or("missing template scope")?,
                                narrowing,
                                value,
                                &context.core_package,
                            )?;
                        }
                    }
                }
                let published = conduit_core::schemas::standalone_valid(
                    &t.reply["tool"][key],
                    &sample["value"],
                )?;
                if authored != published || sample["valid"] != authored {
                    return Err(format!(
                        "{key} verdict mismatch for {}: authored={authored}, published={published}, expected={}",
                        sample["value"], sample["valid"]
                    ));
                }
                results.push(json!({"authored":authored,"published":published}));
            }
            verdicts[key] = json!(results);
        }
    }
    t.reply["verdicts"] = verdicts;
    serde_json::to_value(t).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_state_unchanged_checks_each_actual_core_snapshot() {
        let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let spec = Specification::read(&repository.join("spec")).unwrap();
        let context = Context {
            builtin: None,
            core_package: spec.names().core_package.clone(),
            meta_prefix: read_file(&repository.join("spec/names.yaml")).unwrap()[0]
                .root
                .to_json()["metaPrefix"]
                .as_str()
                .unwrap()
                .into(),
            templates: BTreeMap::new(),
            record_source: "urn:test".into(),
            core_environment: Environment {
                scopes: BTreeMap::new(),
            },
        };
        for (case, kind) in [
            ("schema-invalid-default", "schema-admission"),
            ("schema-defaults", "schema-evaluation"),
            ("e05-control-absent-template", "schema-case"),
        ] {
            let fixture = read_value(
                &repository.join(format!("conformance/interface/schemas/{case}/fixture.json")),
            )
            .unwrap();
            let fixture = expand(&fixture, &BTreeMap::new(), &spec).unwrap();
            run_with_step(
                &repository,
                &fixture,
                kind,
                &spec,
                &context,
                &mut conduit_core::step,
            )
            .unwrap();
            let error = run_with_step(
                &repository,
                &fixture,
                kind,
                &spec,
                &context,
                &mut |state, input, context| {
                    let mut transition = conduit_core::step(state, input, context)?;
                    transition.state.objects.push(json!({"id":"unexpected"}));
                    Ok(transition)
                },
            )
            .unwrap_err();
            assert!(error.contains("stateUnchanged"), "{kind}: {error}");
        }
    }
}

#[cfg(test)]
mod regression_vectors {
    use super::*;
    fn check(name: &str) {
        let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let spec = Specification::read(&repository.join("spec")).unwrap();
        let context = Context {
            builtin: None,
            core_package: spec.names().core_package.clone(),
            meta_prefix: read_file(&repository.join("spec/names.yaml")).unwrap()[0]
                .root
                .to_json()["metaPrefix"]
                .as_str()
                .unwrap()
                .into(),
            templates: BTreeMap::new(),
            record_source: "urn:test".into(),
            core_environment: Environment {
                scopes: BTreeMap::new(),
            },
        };
        let path = repository.join(format!("conformance/interface/schemas/{name}/case.yaml"));
        let case: Case = serde_json::from_value(read_value(&path).unwrap()).unwrap();
        let mut known = BTreeSet::new();
        for directory in ["promises", "conventions", "interface"] {
            for entry in std::fs::read_dir(repository.join("spec").join(directory)).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().is_some_and(|e| e == "yaml") {
                    let value = read_value(&path).unwrap();
                    for promise in value.as_array().unwrap() {
                        known.insert(promise["id"].as_str().unwrap().to_owned());
                    }
                }
            }
        }
        run_case(&path, &case, &spec, &context, &known).unwrap();
    }
    #[test]
    fn control_local_targets() {
        check("e05-control-local-targets");
    }
    #[test]
    fn pattern_property_reference() {
        check("schema-pattern-property-ref");
    }
    #[test]
    fn applicator_defaults() {
        check("schema-applicator-defaults");
    }
    #[test]
    fn conflicting_defaults() {
        check("schema-conflicting-defaults");
    }
    #[test]
    fn equal_defaults() {
        check("schema-equal-defaults");
    }
}
