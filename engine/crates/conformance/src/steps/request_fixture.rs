use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let (state, input) = prepare(step)?;
    let transition = conduit_core::step(&state, input, step.context)?;
    if step.kind == "request-fixture"
        && (transition.state.objects != state.objects
            || transition.state.installed_packages != state.installed_packages)
    {
        return Err("request changed objects or packages outside its scope".into());
    }
    let next = transition.state.clone();
    let actual = serde_json::to_value(transition).map_err(|e| e.to_string())?;
    Ok((actual, next))
}

pub(crate) fn prepare(step: &Step<'_>) -> Result<(State, Input)> {
    let base = step.base;
    let case = step.case;
    let input = step.input;
    let kind = step.kind;
    let spec = step.spec;
    let symbols = step.symbols;
    let file_bytes = |alias: &str| step.file_bytes(alias);
    let mut next = step.state.clone();
    let alias = input["file"].as_str().ok_or("missing request fixture")?;
    let path = case.files.get(alias).ok_or("undeclared request fixture")?;
    let fixtures = read_value(&base.join(path))?;
    let index = input["index"]
        .as_u64()
        .ok_or("missing request observation index")? as usize;
    let fixture = expand(
        fixtures["requests"]
            .get(index)
            .ok_or("request observation out of range")?,
        symbols,
        spec,
    )?;
    access::patch(&mut next, &fixture["patches"])?;
    if kind == "lifecycle-fixture"
        && let Some(invocations) = fixture.get("invocations")
    {
        next.invocations =
            serde_json::from_value(invocations.clone()).map_err(|e| e.to_string())?;
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
                if doc["kind"].as_str() == Some(update["kind"].as_str().unwrap_or("Operation"))
                    && doc["metadata"]["name"] == update["name"]
                {
                    for (k, v) in update["set"].as_object().ok_or("missing contract update")? {
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
            .ok_or("missing observed operation")?] = update["implementation"].clone();
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
    Ok((
        next,
        serde_json::from_value(call).map_err(|e| e.to_string())?,
    ))
}
