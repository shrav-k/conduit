//! File boundary for preinstalled snapshots. Archive bytes are hashed, never installed.
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
    file: String,
    address: String,
    implementation: String,
}
fn load(
    recipe: Recipe,
    base: &Path,
    files: &BTreeMap<String, String>,
    spec: &Specification,
) -> Result<Project> {
    let bytes = recipe
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
) -> Result<conduit_core::Transition> {
    let path = files.get(file).ok_or("undeclared context fixture")?;
    let fixture: Fixture =
        serde_json::from_value(read_value(&base.join(path))?).map_err(|e| e.to_string())?;
    let mut snapshots = vec![];
    for recipe in fixture.snapshots {
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
            project: load(recipe.project, base, files, spec)?,
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
        let path = files
            .get(&artifact.file)
            .ok_or("undeclared artifact file")?;
        let bytes = std::fs::read(base.join(path)).map_err(|e| e.to_string())?;
        if conduit_core::packages::bytes_digest(&bytes) != format!("sha256:{pin}") {
            return Err("Package artifact hash differs from retained bytes".into());
        }
    }
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let core = conduit_loader::read_project(&repository.join("spec/core"), spec)
        .map_err(|e| e.to_string())?;
    let mut documents = core
        .documents
        .iter()
        .map(|d| d.root.to_json())
        .collect::<Vec<_>>();
    for entry in std::fs::read_dir(repository.join("spec/schemas")).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().is_some_and(|e| e == "yaml") {
            documents.extend(
                read_file(&path)
                    .map_err(|e| e.to_string())?
                    .iter()
                    .map(|d| d.root.to_json()),
            );
        }
    }
    let builtin = Project {
        manifest: core.manifest.map(|d| d.root.to_json()),
        documents,
        directory: Some("file:///builtin/".into()),
    };
    conduit_core::step(
        state,
        Input::ResolveProject {
            snapshots,
            builtin,
            project: load(fixture.project, base, files, spec)?,
        },
        context,
    )
}
