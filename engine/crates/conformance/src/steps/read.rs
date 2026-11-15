use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let input = step.input;
    let file_bytes = |alias: &str| step.file_bytes(alias);
    let next = step.state.clone();
    let actual = {
        let alias = input["file"].as_str().ok_or("missing file")?;
        let (_, bytes) = file_bytes(alias)?;
        let syntax = match input["syntax"].as_str() {
            Some("yaml") => Syntax::Yaml,
            Some("json") => Syntax::Json,
            _ => return Err("unsupported syntax".into()),
        };
        match read_bytes(syntax, &bytes) {
            Ok(documents) => {
                json!({"accepted":true,"diagnostics":[],"document_count":documents.len(),
                "documents":documents.iter().map(|d| d.to_json()).collect::<Vec<_>>()})
            }
            Err(errors) => {
                json!({"accepted":false,"diagnostics":errors.0.iter().map(diagnostic).collect::<Vec<_>>()})
            }
        }
    };
    Ok((actual, next))
}
