//! File boundary for snapshots and supplied archive inputs. No fetching occurs here.
use super::*;
use conduit_core::packages::{Project, Snapshot, SnapshotSource};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Recipe {
    files: BTreeMap<String, String>,
    #[serde(default)]
    directory: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotRecipe {
    source: SnapshotSource,
    address: String,
    version: String,
    digest: String,
    #[serde(default)]
    archive: Option<String>,
    project: Recipe,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Fixture {
    snapshots: Vec<SnapshotRecipe>,
    project: Recipe,
    #[serde(default)]
    artifacts: Vec<Artifact>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Artifact {
    sha256: String,
    address: String,
    implementation: String,
}
fn load(
    recipe: Recipe,
    base: &Path,
    files: &BTreeMap<String, String>,
    spec: &Specification,
    reverse_files: bool,
) -> Result<Project> {
    let mut bytes = recipe
        .files
        .iter()
        .map(|(name, alias)| {
            let path = files.get(alias).ok_or("undeclared project file alias")?;
            Ok((
                name.clone(),
                std::fs::read(base.join(path)).map_err(|e| e.to_string())?,
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    if reverse_files {
        bytes.reverse();
    }
    let p = read_submitted_files(&bytes, spec).map_err(|e| e.to_string())?;
    Ok(Project {
        manifest: p.manifest.map(|d| d.root.to_json()),
        documents: p.documents.iter().map(|d| d.root.to_json()).collect(),
        directory: recipe.directory,
    })
}
pub(super) fn run(
    base: &Path,
    file: &str,
    files: &BTreeMap<String, String>,
    state: &State,
    spec: &Specification,
    context: &Context,
    registration: Option<(&Value, &Value, usize)>,
) -> Result<conduit_core::Transition> {
    let path = files.get(file).ok_or("undeclared context fixture")?;
    let fixture: Fixture =
        serde_json::from_value(read_value(&base.join(path))?).map_err(|e| e.to_string())?;
    let mut snapshots = vec![];
    for recipe in fixture.snapshots {
        if !matches!(recipe.source, SnapshotSource::PreinstalledSnapshot) {
            return Err("fixture snapshots must be labelled preinstalled-snapshot".into());
        }
        if let Some(alias) = recipe.archive {
            let path = files.get(&alias).ok_or("undeclared snapshot archive")?;
            let bytes = std::fs::read(base.join(path)).map_err(|e| e.to_string())?;
            if conduit_core::packages::bytes_digest(&bytes) != recipe.digest {
                return Err("snapshot archive evidence digest differs".into());
            }
        }
        snapshots.push(Snapshot {
            source: recipe.source,
            address: recipe.address,
            version: recipe.version,
            digest: recipe.digest,
            project: load(recipe.project, base, files, spec, false)?,
        });
    }
    for artifact in fixture.artifacts {
        let snapshot = snapshots
            .iter()
            .find(|s| s.address == artifact.address)
            .ok_or("artifact names no snapshot")?;
        let pin = snapshot
            .project
            .manifest
            .as_ref()
            .ok_or("artifact snapshot has no manifest")?["implementations"]
            [&artifact.implementation]["package"]["fileSha256"]
            .as_str()
            .ok_or("artifact item has no fileSha256")?;
        if pin != artifact.sha256 {
            return Err("Package artifact pin differs from recorded SHA-256".into());
        }
    }
    let builtin = builtin(spec)?;
    let mut project = load(
        fixture.project,
        base,
        files,
        spec,
        registration.is_some_and(|(_, _, order)| order == 2),
    )?;
    if registration.is_some_and(|(_, _, order)| order == 1) {
        project.documents.reverse();
    }
    if registration.is_some_and(|(_, _, order)| order == 2) && !project.documents.is_empty() {
        project.documents.rotate_left(1);
    }
    let input = if let Some((evidence, request, _)) = registration {
        Input::RegisterProject {
            snapshots,
            builtin,
            project,
            evidence: serde_json::from_value(evidence.clone()).map_err(|e| e.to_string())?,
            request: serde_json::from_value(request.clone()).map_err(|e| e.to_string())?,
        }
    } else {
        Input::ResolveProject {
            snapshots,
            builtin,
            project,
        }
    };
    conduit_core::step(state, input, context)
}

fn builtin(spec: &Specification) -> Result<Project> {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let core = conduit_loader::read_project(&repository.join("spec/core"), spec)
        .map_err(|e| e.to_string())?;
    let mut documents = core
        .documents
        .iter()
        .map(|d| d.root.to_json())
        .collect::<Vec<_>>();
    let mut paths = std::fs::read_dir(repository.join("spec/schemas"))
        .map_err(|e| e.to_string())?
        .map(|entry| entry.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>>>()?;
    paths.sort();
    for path in paths {
        if path.extension().is_some_and(|e| e == "yaml") {
            documents.extend(
                read_file(&path)
                    .map_err(|e| e.to_string())?
                    .iter()
                    .map(|d| d.root.to_json()),
            );
        }
    }
    Ok(Project {
        manifest: core.manifest.map(|d| d.root.to_json()),
        documents,
        directory: Some("file:///builtin/".into()),
    })
}

pub(super) fn install(
    base: &Path,
    input: &Value,
    files: &BTreeMap<String, String>,
    state: &State,
    spec: &Specification,
    context: &Context,
) -> Result<conduit_core::Transition> {
    let path = files
        .get(input["file"].as_str().ok_or("missing archive fixture")?)
        .ok_or("undeclared archive fixture")?;
    let mut recipe = read_value(&base.join(path))?;
    let completion = recipe
        .get_mut("completion")
        .ok_or("missing fetching completion")?;
    if completion["status"] == "complete" {
        let alias = completion["file"].as_str().ok_or("missing archive alias")?;
        let path = files.get(alias).ok_or("undeclared archive alias")?;
        let bytes = std::fs::read(base.join(path)).map_err(|e| e.to_string())?;
        completion
            .as_object_mut()
            .ok_or("invalid completion")?
            .remove("file");
        completion["bytes"] = json!(bytes);
    }
    conduit_core::step(
        state,
        Input::InstallArchive {
            archive: serde_json::from_value(recipe).map_err(|e| e.to_string())?,
            snapshots: vec![],
            builtin: builtin(spec)?,
        },
        context,
    )
}
