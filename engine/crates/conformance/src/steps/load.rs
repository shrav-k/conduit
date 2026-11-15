use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let input = step.input;
    let kind = step.kind;
    let state = step.state;
    let context = step.context;
    let spec = step.spec;
    let file_bytes = |alias: &str| step.file_bytes(alias);
    let mut next = step.state.clone();
    let actual = {
        let files = input["files"]
            .as_array()
            .ok_or("missing files")?
            .iter()
            .map(|v| file_bytes(v.as_str().ok_or("invalid file alias")?))
            .collect::<Result<Vec<_>>>()?;
        match read_submitted_files(&files, spec) {
            Ok(project) if kind == "submit-files" => {
                let mut core_input = input.clone();
                core_input["kind"] = json!("submit");
                core_input
                    .as_object_mut()
                    .ok_or("invalid input")?
                    .remove("files");
                core_input["documents"] = json!(
                    project
                        .documents
                        .iter()
                        .map(|d| d.root.to_json())
                        .collect::<Vec<_>>()
                );
                let t = conduit_core::step(
                    state,
                    serde_json::from_value(core_input).map_err(|e| e.to_string())?,
                    context,
                )?;
                next = t.state.clone();
                serde_json::to_value(t).map_err(|e| e.to_string())?
            }
            Ok(project) => {
                json!({"accepted":true,"diagnostics":[],"document_count":project.documents.len(),"manifest":project.manifest.is_some()})
            }
            Err(errors) if kind == "load" => {
                json!({"accepted":false,"diagnostics":errors.0.iter().map(diagnostic).collect::<Vec<_>>()})
            }
            Err(errors) => {
                return Err(format!(
                    "protected submission failed at loader instead of core: {errors}"
                ));
            }
        }
    };
    Ok((actual, next))
}
