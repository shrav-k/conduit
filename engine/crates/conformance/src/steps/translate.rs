use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let files = step.input["files"]
        .as_array()
        .ok_or("missing files")?
        .iter()
        .map(|alias| step.file_bytes(alias.as_str().ok_or("invalid file alias")?))
        .collect::<Result<Vec<_>>>()?;
    let reply = match read_submitted_files(&files, step.spec) {
        Ok(project) => {
            let documents = project
                .documents
                .iter()
                .map(|d| d.root.to_json())
                .collect::<Vec<_>>();
            let mut translations = documents
                .iter()
                .zip(project.translations())
                .filter_map(|(document, origin)| {
                    origin.as_ref().map(|origin| {
                        (
                            document["kind"].as_str().unwrap(),
                            document["metadata"]["name"].as_str(),
                            origin.rule,
                        )
                    })
                })
                .collect::<Vec<_>>();
            translations.sort_unstable();
            let translations = translations
                .into_iter()
                .map(|(kind, name, rule)| json!({"kind":kind,"name":name,"rule":rule}))
                .collect::<Vec<_>>();
            json!({"accepted":true,"diagnostics":[],"framed_documents":documents,"translations":translations})
        }
        Err(errors) => {
            json!({"accepted":false,"diagnostics":errors.0.iter().map(diagnostic).collect::<Vec<_>>()})
        }
    };
    Ok((reply, step.state.clone()))
}
