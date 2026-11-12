//! Required-field and ordering checks in addition to the permissive record.v1 schema.
use super::*;

pub(super) fn transition(
    actual: &Value,
    before: &State,
    state: &State,
    context: &Context,
    supplied_key: Option<&str>,
) -> Result<()> {
    let records = actual["records"].as_array().ok_or("records absent")?;
    // Compare the request itself, even when no invocation was allocated. Child
    // invocations can have their own keys, so check the outer access and ending.
    if let Some(key) = supplied_key
        && !records.is_empty()
    {
        for kind in ["access-decision", "request-ended"] {
            if kind == "request-ended" && actual["reply"]["pending"] == true {
                continue;
            }
            let r = if kind == "access-decision" {
                records.iter().find(|r| r["type"] == kind)
            } else {
                records.iter().rev().find(|r| r["type"] == kind)
            }
            .ok_or_else(|| format!("REC-001: keyed request lacks {kind}"))?;
            if r["data"]["idempotencyKey"] != key {
                return Err(format!("REC-002: {kind} lacks the supplied request key"));
            }
        }
    }
    // Validate generated and retained built-in outputs, including replies to repeats.
    for v in &state.invocations {
        if v["state"] == "succeeded"
            && v.get("retained").is_some()
            && !before.invocations.contains(v)
        {
            builtin_output(&v["operation"], &v["retained"]["outputs"], context)?;
        }
    }
    let reply = &actual["reply"];
    if reply["_meta"][format!("{}outcome", context.meta_prefix)] == "succeeded" {
        let id = &reply["_meta"][format!("{}invocation", context.meta_prefix)];
        let v = state
            .invocations
            .iter()
            .find(|v| v["id"] == *id)
            .ok_or("succeeded reply lacks its invocation")?;
        builtin_output(&v["operation"], &reply["structuredContent"], context)?;
    }
    if records
        .iter()
        .any(|r| r["data"]["action"] == "records-purged")
        && before
            .invocations
            .iter()
            .any(|v| !state.invocations.contains(v))
    {
        return Err("REC-004 / OPS-008: purge changed the invocation index".into());
    }
    let mut ids = BTreeSet::new();
    for r in &state.records {
        if !ids.insert((r["source"].to_string(), r["id"].to_string())) {
            return Err("REC-003: duplicate occurrence identity".into());
        }
    }
    for (position, r) in records.iter().enumerate() {
        let data = &r["data"];
        if r["source"] != context.record_source {
            return Err("DL8: undocumented record source".into());
        }
        if data["operation"] == format!("{}#run.v1", context.core_package)
            && (data["inputs"].get("identity").is_some() || data["inputs"].get("secrets").is_some())
        {
            return Err("PRV-024 / REC-002: run record contains identity or secrets".into());
        }
        if r["type"] == "access-decision"
            && data["permitted"] == false
            && data.get("reason").is_none()
        {
            return Err("REC-001: refused access lacks its reason".into());
        }
        if r["type"] == "provider-call" {
            for f in ["invocation", "target", "operation"] {
                if !data[f].is_string() {
                    return Err(format!("REC-002: provider call lacks {f}"));
                }
            }
            if ["run.v1", "stop.v1", "inspect-run.v1"]
                .iter()
                .any(|n| data["operation"] == format!("{}#{n}", context.core_package))
                && !r["subject"].is_string()
            {
                return Err("REC-002: provider run record lacks CloudEvent subject".into());
            }
        }
        if ["invocation-dispatched", "invocation-ended"]
            .iter()
            .any(|kind| r["type"] == *kind)
        {
            let v = state
                .invocations
                .iter()
                .find(|v| v["id"] == data["invocation"])
                .ok_or("REC-002: invocation record lacks an index entry")?;
            for (field, stored) in [
                ("caller", "caller"),
                ("target", "target"),
                ("operation", "operation"),
                ("idempotencyKey", "key"),
                ("serving", "serving"),
            ] {
                if let Some(expected) = v.get(stored).filter(|v| !v.is_null())
                    && data.get(field) != Some(expected)
                {
                    return Err(format!(
                        "REC-002: invocation record {field} differs from its invocation"
                    ));
                }
            }
            if v["conduit"].is_string() {
                for (field, stored) in [("consumer", "sender"), ("conduit", "conduit")] {
                    if data[field] != v[stored] {
                        return Err(format!("REC-002: invocation record lacks {field}"));
                    }
                }
            }
            // SCH-001/LIF-021: engine-generated parts obey the same input contract.
            if r["type"] == "invocation-dispatched" && v.get("cascade_parent").is_some() {
                let (scope, core) = context
                    .core_environment
                    .scopes
                    .iter()
                    .find(|(_, s)| s.address == context.core_package)
                    .ok_or("core schema scope absent")?;
                let name = v["operation"]
                    .as_str()
                    .and_then(|op| op.split_once('#'))
                    .map(|(_, name)| name)
                    .ok_or("cascade child operation absent")?;
                let schema = &core.contracts[name]["spec"]["inputSchema"];
                if !context.core_environment.valid(
                    scope,
                    schema,
                    &v["arguments"],
                    &context.core_package,
                )? {
                    return Err("SCH-001: cascade child violates its inputSchema".into());
                }
            }
            if r["type"] == "invocation-dispatched" && v["hook"] != true {
                let access = records[..position].iter().find(|r| {
                    r["type"] == "access-decision"
                        && r["data"]["invocation"] == v["id"]
                        && r["data"]["permitted"] == true
                });
                if access.is_none() {
                    return Err(
                        "ACC-002 / REC-001: dispatch has no preceding permitted access record"
                            .into(),
                    );
                }
            }
            if r["type"] == "invocation-ended" && data.get("outcome").is_none() {
                return Err("REC-002: invocation ending lacks outcome".into());
            }
        }
    }
    // Invocation outputs must retain every portable inspection part. In particular, records
    // are compared as an exact ordered list, rather than accepting a projected subsequence.
    let output = &actual["reply"]["structuredContent"];
    if output.get("declared").is_some() && output.get("records").is_some() {
        let o = state
            .objects
            .iter()
            .find(|o| o["id"] == output["id"])
            .ok_or("LIF-013: inspected object absent")?;
        if output["revision"] != o["revision"] || output["packages"] != o.get("packages").cloned().unwrap_or_else(|| json!([{ "address":context.core_package,"version":context.builtin.as_ref().and_then(|b|b.manifest.as_ref()).map(|m|m["version"].clone()).unwrap_or(Value::Null)}])) { return Err("LIF-013: inspection omitted object identities or packages".into()); }
        if o.get("state").is_some() && output["status"]["state"] != o["state"] {
            return Err("LIF-013: inspection omitted lifecycle state".into());
        }
        if o.get("spec").is_some() && output["status"].get("state").is_some() {
            return Err("LIF-012: template inspection has a lifecycle state".into());
        }
        if output["declared"]["config"] != o["config"] {
            return Err("LIF-013: inspection omitted resolved configuration".into());
        }
        let own_invocation =
            &actual["reply"]["_meta"][format!("{}invocation", context.meta_prefix)];
        let relevant: BTreeSet<_> = state
            .invocations
            .iter()
            .filter(|v| {
                v["target"] == o["id"]
                    || v["object"] == o["id"]
                    || references(&v["arguments"], &o["id"])
                    || v["input_objects"]
                        .as_array()
                        .is_some_and(|ids| ids.contains(&o["id"]))
            })
            .map(|v| v["id"].to_string())
            .collect();
        let unknown: Vec<_> = state
            .invocations
            .iter()
            .filter(|v| v["state"] == "unknown" && relevant.contains(&v["id"].to_string()))
            .map(|v| v["id"].clone())
            .collect();
        if o.get("spec").is_none() && output["status"]["unreconciled"] != json!(unknown) {
            return Err("LIF-013 / OPS-008: inspection omitted an unknown invocation".into());
        }
        let expected: Vec<_> = state
            .records
            .iter()
            .filter(|r| {
                if r["data"]["invocation"] == *own_invocation
                    && ["invocation-ended", "request-ended"]
                        .iter()
                        .any(|k| r["type"] == *k)
                {
                    return false;
                }
                r["subject"] == o["id"]
                    || ["target", "object", "caller", "consumer", "conduit"]
                        .iter()
                        .any(|f| r["data"][f] == o["id"])
                    || references(&r["data"]["inputs"], &o["id"])
                    || relevant.contains(&r["data"]["invocation"].to_string())
                    || relevant.contains(&r["data"]["serving"].to_string())
            })
            .cloned()
            .collect();
        if output["records"] != json!(expected) {
            return Err("LIF-013 / REC-005: inspection omitted, added or reordered records".into());
        }
        if output["recordGaps"] != json!(state.record_gaps)
            || output["recordsComplete"] != state.record_gaps.is_empty()
        {
            return Err("REC-006: inspection hid a record gap".into());
        }
    }
    Ok(())
}
fn builtin_output(operation: &Value, output: &Value, context: &Context) -> Result<()> {
    let Some(name) = operation
        .as_str()
        .and_then(|op| op.strip_prefix(&format!("{}#", context.core_package)))
    else {
        return Ok(());
    };
    let (scope, core) = context
        .core_environment
        .scopes
        .iter()
        .find(|(_, s)| s.address == context.core_package)
        .ok_or("core schema scope absent")?;
    let contract = core
        .contracts
        .get(name)
        .ok_or("built-in output contract absent")?;
    if let Some(schema) = contract["spec"].get("outputSchema")
        && !context
            .core_environment
            .valid(scope, schema, output, &context.core_package)?
    {
        let published =
            context
                .core_environment
                .publish_schema(scope, schema, &context.core_package)?;
        let validator = conduit_loader::compile(&published).map_err(|e| e.to_string())?;
        let details = validator
            .validate(output)
            .err()
            .map(|e| e.to_string())
            .unwrap_or_default();
        return Err(format!(
            "{name}: built-in output violates outputSchema: {details}"
        ));
    }
    Ok(())
}
fn references(value: &Value, id: &Value) -> bool {
    match value {
        Value::Object(m) => m.get("id") == Some(id) || m.values().any(|v| references(v, id)),
        Value::Array(a) => a.iter().any(|v| references(v, id)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> Context {
        Context {
            builtin: None,
            core_package: "test.core".into(),
            meta_prefix: "test/".into(),
            templates: BTreeMap::new(),
            record_source: "urn:test".into(),
            core_environment: conduit_core::schemas::Environment {
                scopes: BTreeMap::new(),
            },
        }
    }

    #[test]
    fn supplied_key_is_required_on_both_records_without_an_invocation() {
        let state = State::default();
        let valid = json!({"records":[
            {"source":"urn:test","type":"access-decision","data":{"permitted":false,"reason":"refused","idempotencyKey":"key"}},
            {"source":"urn:test","type":"request-ended","data":{"reason":"refused","idempotencyKey":"key"}}
        ],"reply":{}});
        assert!(transition(&valid, &state, &state, &context(), Some("key")).is_ok());
        for indices in [vec![0], vec![1], vec![0, 1]] {
            let mut omitted = valid.clone();
            for i in indices {
                omitted["records"][i]["data"]
                    .as_object_mut()
                    .unwrap()
                    .remove("idempotencyKey");
            }
            assert!(transition(&omitted, &state, &state, &context(), Some("key")).is_err());
        }
        let mut wrong = valid;
        wrong["records"][1]["data"]["idempotencyKey"] = json!("another-key");
        assert!(transition(&wrong, &state, &state, &context(), Some("key")).is_err());
    }

    #[test]
    fn builtin_output_schema_rejects_status_without_state() {
        let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let spec = Specification::read(&repository.join("spec")).unwrap();
        let mut c = context();
        c.core_package = spec.names().core_package.clone();
        c.core_environment =
            schemas::environment(&repository, &json!({"environment":{"scopes":{}}}), &spec)
                .unwrap();
        let operation = json!(format!("{}#inspect.v1", c.core_package));
        let mut output = json!({"id":"00000000-0000-4000-8000-000000000001","revision":"r1","packages":[],"declared":{},"records":[]});
        assert!(builtin_output(&operation, &output, &c).is_ok());
        output["status"] = json!({"conditions":[],"unreconciled":[]});
        assert!(builtin_output(&operation, &output, &c).is_err());
        output["status"]["state"] = json!("registered");
        assert!(builtin_output(&operation, &output, &c).is_ok());
        output["status"]["state"] = json!("invented-template-state");
        assert!(builtin_output(&operation, &output, &c).is_err());
    }

    #[test]
    fn optional_schema_members_remain_required_where_they_apply() {
        let mut state = State::default();
        state.invocations.push(json!({"id":"invocation","caller":"caller","sender":"consumer","target":"target","conduit":"conduit","operation":"test.core#write.v1","key":"key","serving":"parent"}));
        let data = json!({"invocation":"invocation","caller":"caller","consumer":"consumer","target":"target","conduit":"conduit","operation":"test.core#write.v1","idempotencyKey":"key","serving":"parent"});
        let access = json!({"source":"urn:test","id":"access","type":"access-decision","data":{"invocation":"invocation","permitted":true}});
        let dispatch =
            json!({"source":"urn:test","id":"dispatch","type":"invocation-dispatched","data":data});
        let valid = json!({"records":[access,dispatch],"reply":{}});
        assert!(transition(&valid, &State::default(), &state, &context(), None).is_ok());
        for field in [
            "invocation",
            "caller",
            "consumer",
            "target",
            "conduit",
            "operation",
            "idempotencyKey",
            "serving",
        ] {
            let mut omitted = valid.clone();
            omitted["records"][1]["data"]
                .as_object_mut()
                .unwrap()
                .remove(field);
            assert!(
                transition(&omitted, &State::default(), &state, &context(), None).is_err(),
                "missing {field} passed"
            );
        }
        let reversed = json!({"records":[valid["records"][1],valid["records"][0]],"reply":{}});
        assert!(transition(&reversed, &State::default(), &state, &context(), None).is_err());
    }

    #[test]
    fn inspection_cannot_hide_a_record_or_an_unknown_invocation() {
        let mut state = State::default();
        state.objects.push(
            json!({"id":"object","revision":"r","packages":[],"config":{},"state":"registered"}),
        );
        state
            .invocations
            .push(json!({"id":"unknown","target":"object","state":"unknown"}));
        state.records.push(json!({"source":"urn:test","id":"event","type":"object-change","data":{"target":"object"}}));
        let valid = json!({"records":[],"reply":{"structuredContent":{"id":"object","revision":"r","packages":[],"declared":{"config":{}},"status":{"state":"registered","unreconciled":["unknown"]},"records":state.records,"recordGaps":[],"recordsComplete":true}}});
        assert!(transition(&valid, &State::default(), &state, &context(), None).is_ok());
        for path in ["records", "status"] {
            let mut omitted = valid.clone();
            if path == "status" {
                omitted["reply"]["structuredContent"]["status"]["unreconciled"] = json!([]);
            } else {
                omitted["reply"]["structuredContent"]["records"] = json!([]);
            }
            assert!(transition(&omitted, &State::default(), &state, &context(), None).is_err());
        }
    }
}
