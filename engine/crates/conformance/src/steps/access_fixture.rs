use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let base = step.base;
    let case = step.case;
    let input = step.input;
    let state = step.state;
    let context = step.context;
    let spec = step.spec;
    let symbols = step.symbols;
    let next: State;
    let actual = {
        let alias = input["file"].as_str().ok_or("missing access fixture")?;
        let path = case.files.get(alias).ok_or("undeclared access fixture")?;
        let observations = read_value(&base.join(path))?;
        let index = input["index"].as_u64().ok_or("missing observation index")? as usize;
        let observation = observations["observations"]
            .as_array()
            .and_then(|a| a.get(index))
            .ok_or("observation index out of range")?;
        let fixture = expand(observation, symbols, spec)?;
        let t = access::run(&fixture, state, context)?;
        next = t.state.clone();
        serde_json::to_value(t).map_err(|e| e.to_string())?
    };
    Ok((actual, next))
}
