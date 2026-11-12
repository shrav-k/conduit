//! REV-010: reconcile the held values of a control against the current configuration.
use crate::{
    Allocation, Context, Result, State, Transition, access, decision, lifecycle, requests, schemas,
};
use serde_json::{Value, json};

// OBJ-016: each JSON Pointer token is evaluated against its current container.
fn setting<'a>(parent: &'a mut Value, token: &str) -> Result<&'a mut Value> {
    match parent {
        Value::Object(map) => Ok(map.entry(token.to_owned()).or_insert_with(|| json!({}))),
        Value::Array(_) => parent
            .pointer_mut(&format!("/{token}"))
            .ok_or_else(|| "change array element does not exist".into()),
        _ => Err("change parent is not a container".into()),
    }
}

fn updated(object: &Value, invocation: &Value) -> Result<Value> {
    let mut config = object["config"].clone();
    for (input, pointer) in invocation["contract"]["spec"]["changes"]
        .as_object()
        .into_iter()
        .flatten()
    {
        if let Some(value) = invocation["arguments"].get(input) {
            let pointer = pointer.as_str().ok_or("control change is not a pointer")?;
            // CTR-006: the empty pointer addresses the whole configuration.
            if pointer.is_empty() {
                config = value.clone();
                continue;
            }
            // CTR-006 permits schema-described settings, including array elements
            // and new optional object settings.
            let tokens: Vec<_> = pointer
                .strip_prefix('/')
                .ok_or("invalid change pointer")?
                .split('/')
                .map(|s| s.replace("~1", "/").replace("~0", "~"))
                .collect();
            let mut parent = &mut config;
            for token in &tokens[..tokens.len() - 1] {
                parent = setting(parent, token)?;
            }
            *setting(parent, tokens.last().unwrap())? = value.clone();
        }
    }
    Ok(config)
}
pub(crate) fn preflight(
    s: &State,
    v: &Value,
    args: &Value,
    c: &Context,
) -> Result<Option<(&'static str, &'static str)>> {
    let object = access::object(s, &v["target"]);
    let stores = v["contract"]["spec"].get("changes").is_some()
        && args["occurred"] == true
        && object.is_some_and(|o| o["state"] != "retired");
    if !stores {
        if args.get("revision").is_some() {
            return Ok(Some(("invalid", "OPS-031")));
        }
        return Ok(None);
    }
    let o = object.unwrap();
    if args.get("revision").is_none() {
        return Ok(Some(("invalid", "REV-011")));
    }
    let published = crate::registration::publication_object(o, c)?;
    let (env, scope, ty) = schemas::operation(
        &published,
        published["type"].as_str().ok_or("missing type")?,
        &c.core_package,
    )?;
    // REV-010: the held change is applied to the configuration as it is now. A pointer that no
    // longer addresses a setting there, such as an array element that a later change removed,
    // cannot be stored, so the reconciliation ends invalid like one whose values break the schema
    // (REV-013), with no effect, rather than as an error of the engine (OPS-005).
    let Ok(config) = updated(o, v) else {
        return Ok(Some(("invalid", "REV-013")));
    };
    if !env.valid(&scope, &ty["spec"]["schema"], &config, &c.core_package)? {
        return Ok(Some(("invalid", "REV-013")));
    }
    if args["revision"] != o["revision"] {
        return Ok(Some(("conflict", "REV-002")));
    }
    if lifecycle::busy(s, &o["id"], None) {
        return Ok(Some(("conflict", "REV-014")));
    }
    Ok(None)
}
pub(crate) fn reconcile(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    reconciled: usize,
) -> Result<()> {
    let v = t.state.invocations[reconciled].clone();
    let args = t.state.invocations[i]["arguments"].clone();
    if args["occurred"] == true
        && v["contract"]["spec"].get("changes").is_some()
        && let Some(n) = t
            .state
            .objects
            .iter()
            .position(|o| o["id"] == v["target"] && o["state"] != "retired")
    {
        let config = updated(&t.state.objects[n], &v)?;
        if config != t.state.objects[n]["config"] {
            let revision = a
                .evidence
                .revisions
                .first()
                .filter(|r| !r.is_empty())
                .ok_or("reconciliation needs a revision")?;
            t.state.objects[n]["config"] = config;
            t.state.objects[n]["revision"] = json!(revision);
            requests::record(
                t,
                a,
                c,
                "object-change",
                json!({"target":v["target"],"invocation":t.state.invocations[i]["id"],"action":"control-reconciled","revision":revision}),
            )?;
        }
    }
    t.decisions.push(decision(
        "reconcile-control-values",
        &[
            "REV-010", "REV-002", "REV-003", "REV-011", "REV-013", "REV-014", "ACC-033",
        ],
        &[],
        json!({}),
        vec![],
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::updated;
    use serde_json::json;

    fn control(pointer: &str, value: serde_json::Value) -> serde_json::Value {
        json!({"contract": {"spec": {"changes": {"setting": pointer}}}, "arguments": {"setting": value}})
    }

    #[test]
    fn an_existing_array_element_is_updated() {
        let object = json!({"config": {"limits": [1]}});
        let config = updated(&object, &control("/limits/0", json!(2))).unwrap();
        assert_eq!(config, json!({"limits": [2]}));
    }

    #[test]
    fn a_removed_array_element_cannot_be_updated() {
        // The configuration shortened `limits` after the control was dispatched (REV-010).
        let object = json!({"config": {"limits": []}});
        assert!(updated(&object, &control("/limits/0", json!(2))).is_err());
    }
}
