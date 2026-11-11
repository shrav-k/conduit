//! Supplied observations of open/closed Conduits, not lifecycle transitions. Exercise the
//! pure access core on registration's real state; validate its unchanged snapshot exactly.
use super::*;
pub(super) fn run(
    fixture: &Value,
    state: &State,
    context: &Context,
) -> Result<conduit_core::Transition> {
    let mut snapshot = state.clone();
    patch(&mut snapshot, &fixture["patches"])?;
    // Older access observations supply only Operation/Capability contracts. Complete
    // their synthetic targets for IFC-033; registered instances already hold a Type.
    for target in &mut snapshot.objects {
        if target.get("type").is_none()
            && target["schema_environment"]["scopes"]
                .as_object()
                .into_iter()
                .flat_map(|m| m.values())
                .flat_map(|scope| {
                    scope["contracts"]
                        .as_object()
                        .into_iter()
                        .flat_map(|m| m.values())
                })
                .any(|doc| doc["kind"] == "Operation")
        {
            target["type"] = json!(format!("{}#interface.v1", context.core_package));
            target["schema_environment"]["scopes"]["core"] =
                serde_json::to_value(&context.core_environment.scopes["core"])
                    .map_err(|e| e.to_string())?;
        }
    }
    if let Some(invocations) = fixture.get("invocations") {
        snapshot.invocations =
            serde_json::from_value(invocations.clone()).map_err(|e| e.to_string())?;
    }
    let before = snapshot.clone();
    let input: Input =
        serde_json::from_value(fixture["call"].clone()).map_err(|e| e.to_string())?;
    let mut t = conduit_core::step(&snapshot, input, context)?;
    if t.state.started != snapshot.started
        || t.state.installed_packages != snapshot.installed_packages
        || t.state.objects != snapshot.objects
        || t.state.invocations != snapshot.invocations
        || !t.external_intents.is_empty()
        || t.records
            .iter()
            .any(|r| r["type"] == "invocation-dispatched")
    {
        return Err("access changed objects/invocations or dispatched".into());
    }
    if ["refused", "unavailable"]
        .iter()
        .any(|reason| t.reply["reason"] == *reason)
    {
        let allowed: BTreeSet<_> = ["reason", "restriction"].into_iter().collect();
        if t.reply
            .as_object()
            .ok_or("reply must be a map")?
            .keys()
            .any(|k| !allowed.contains(k.as_str()))
        {
            return Err("refusal/unavailability disclosed extra fields".into());
        }
    }
    if let Some(after) = fixture.get("after") {
        let mut next = before.clone();
        patch(&mut next, after)?;
        t.reply["changes"] = json!(conduit_core::access::tool_list_changes(
            &before, &next, context
        )?);
    }
    // The observation is local to this call. Preserve only its appended records/IDs in
    // the outer sequence, so the next observation starts from the registered state again.
    t.state.objects = state.objects.clone();
    t.state.invocations = state.invocations.clone();
    Ok(t)
}
pub(super) fn patch(state: &mut State, patches: &Value) -> Result<()> {
    if patches.is_null() {
        return Ok(());
    }
    for patch in patches.as_array().ok_or("patches must be an array")? {
        if let Some(id) = patch.get("remove") {
            let before = state.objects.len();
            state.objects.retain(|o| o["id"] != *id);
            if state.objects.len() == before {
                return Err("removal observation names no object".into());
            }
            continue;
        }
        if patch.get("append").is_some() {
            state.objects.push(patch["append"].clone());
            continue;
        }
        let o = state
            .objects
            .iter_mut()
            .find(|o| {
                patch.get("id").is_some_and(|id| o["id"] == *id)
                    || patch
                        .get("name")
                        .is_some_and(|name| o["name"] == *name && o.get("spec").is_none())
            })
            .ok_or_else(|| format!("observation names no object: {patch}"))?;
        for (key, value) in patch["set"].as_object().into_iter().flatten() {
            o[key] = value.clone();
        }
    }
    Ok(())
}
