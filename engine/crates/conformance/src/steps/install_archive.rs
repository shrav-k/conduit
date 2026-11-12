use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let base = step.base;
    let case = step.case;
    let input = step.input;
    let state = step.state;
    let context = step.context;
    let spec = step.spec;
    let next: State;
    let actual = {
        let t = packages::install(base, input, &case.files, state, spec, context)?;
        next = t.state.clone();
        serde_json::to_value(t).map_err(|e| e.to_string())?
    };
    Ok((actual, next))
}
