use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let base = step.base;
    let case = step.case;
    let input = step.input;
    let kind = step.kind;
    let state = step.state;
    let context = step.context;
    let spec = step.spec;
    let next: State;
    let actual = {
        let mut t = packages::run(
            base,
            input["file"].as_str().ok_or("missing context fixture")?,
            &case.files,
            state,
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
                    state,
                    spec,
                    context,
                    Some((&input["evidence"], &input["request"], order)),
                )?;
                if serde_json::to_value(other).map_err(|e| e.to_string())? != original {
                    return Err("SUB-001: registration differs by document/file order".into());
                }
            }
            t.reply["orders_compared"] = json!(3);
        }
        next = t.state.clone();
        serde_json::to_value(t).map_err(|e| e.to_string())?
    };
    Ok((actual, next))
}
