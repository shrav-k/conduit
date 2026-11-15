use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let input = step.input;
    let file_bytes = |alias: &str| step.file_bytes(alias);
    let next = step.state.clone();
    let actual = {
        let (_, bytes) = file_bytes(input["file"].as_str().ok_or("missing file")?)?;
        let documents = read_bytes(Syntax::Json, &bytes).map_err(|e| e.to_string())?;
        let validator = conduit_loader::compile(&input["schema"]).map_err(|e| e.to_string())?;
        json!({"valid":documents.iter().map(|d| validator.is_valid(&d.to_json())).collect::<Vec<_>>()})
    };
    Ok((actual, next))
}
