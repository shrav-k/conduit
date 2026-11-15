use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let input = step.input;
    let state = step.state;
    let context = step.context;
    let next: State;
    let actual = {
        let typed: Input = serde_json::from_value(input.clone())
            .map_err(|e| format!("unsupported core input: {e}"))?;
        let t = conduit_core::step(state, typed, context)?;
        next = t.state.clone();
        serde_json::to_value(t).map_err(|e| e.to_string())?
    };
    Ok((actual, next))
}
