//! Load core contracts, projects and explicitly requested package archives.
use crate::Result;
use conduit_core::{
    Context,
    packages::Project,
    schemas::{Environment, Scope},
};
use conduit_loader::{Specification, read_file};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

pub fn value(path: &Path) -> Result<Value> {
    let documents = read_file(path).map_err(|e| e.to_string())?;
    if documents.len() != 1 {
        return Err("expected one boundary document".into());
    }
    Ok(documents[0].root.to_json())
}

pub fn project(directory: &Path, spec: &Specification) -> Result<Project> {
    let directory = directory.canonicalize().map_err(|e| e.to_string())?;
    let loaded = conduit_loader::read_project(&directory, spec).map_err(|e| e.to_string())?;
    Ok(Project {
        manifest: loaded.manifest.map(|d| d.root.to_json()),
        documents: loaded.documents.iter().map(|d| d.root.to_json()).collect(),
        directory: Some(
            url::Url::from_directory_path(directory)
                .map_err(|_| "invalid directory URL")?
                .into(),
        ),
    })
}

pub fn context(repository: &Path) -> Result<Context> {
    let spec = Specification::read(&repository.join("spec")).map_err(|e| e.to_string())?;
    let names = value(&repository.join("spec/names.yaml"))?;
    let mut builtin = project(&repository.join("spec/core"), &spec)?;
    let mut schema_paths = std::fs::read_dir(repository.join("spec/schemas"))
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>>>()?;
    schema_paths.sort();
    for path in schema_paths {
        if path.extension().is_some_and(|e| e == "yaml") {
            builtin.documents.extend(
                read_file(&path)
                    .map_err(|e| e.to_string())?
                    .iter()
                    .map(|d| d.root.to_json()),
            );
        }
    }
    let contracts = builtin
        .documents
        .iter()
        .filter(|d| {
            ["Schema", "Operation", "Type", "Capability", "Requirement"]
                .iter()
                .any(|k| d["kind"] == *k)
        })
        .map(|d| {
            Ok((
                d["metadata"]["name"]
                    .as_str()
                    .ok_or("missing contract name")?
                    .into(),
                d.clone(),
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    let templates = read_file(&repository.join("spec/core/templates.yaml"))
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|d| {
            let d = d.root.to_json();
            Ok((
                d["metadata"]["name"]
                    .as_str()
                    .ok_or("missing template name")?
                    .into(),
                d,
            ))
        })
        .collect::<Result<_>>()?;
    let scope = Scope {
        address: spec.names().core_package.clone(),
        version: None,
        manifest: BTreeMap::new(),
        contracts,
        exports: None,
    };
    let profile = value(&repository.join("conformance/profiles/first-build.yaml"))?;
    Ok(Context {
        builtin: Some(builtin),
        core_package: spec.names().core_package.clone(),
        meta_prefix: names["metaPrefix"]
            .as_str()
            .ok_or("missing metadata prefix")?
            .into(),
        templates,
        record_source: profile["DL8"]["source"]
            .as_str()
            .ok_or("missing record source")?
            .into(),
        core_environment: Environment {
            scopes: BTreeMap::from([("core".into(), scope)]),
        },
    })
}

/// Metadata discovery for an explicit public install request. The engine reads and
/// validates these archive bytes again; this command admits no package itself.
pub fn install_arguments(argument: &str) -> Result<Value> {
    let (path, hash) = argument
        .rsplit_once('#')
        .ok_or("install needs ARCHIVE#SHA256")?;
    let path = std::path::Path::new(path).canonicalize().map_err(|error| {
        format!(
            "install archive {path:?}: cannot resolve archive: {error}; use a readable archive path"
        )
    })?;
    let bytes = std::fs::read(&path).map_err(|error| {
        format!(
            "install archive {}: cannot read archive: {error}; check its permissions",
            path.display()
        )
    })?;
    let digest = format!("sha256:{}", hash.strip_prefix("sha256:").unwrap_or(hash));
    if conduit_core::packages::bytes_digest(&bytes) != digest {
        return Err(format!(
            "install archive {path:?}: package archive digest mismatch; use the SHA-256 of these exact archive bytes"
        ));
    }
    let manifest = archive_metadata::manifest(&bytes)?;
    Ok(
        serde_json::json!({"address":manifest["address"],"version":manifest["version"],"digest":digest,"fetch":url::Url::from_file_path(path).map_err(|_| "invalid archive URL")?.as_str()}),
    )
}

/// CLI archive admission. No filesystem extraction; the core owns package validation.
pub fn install_package(shell: &crate::Shell, _repository: &Path, argument: &str) -> Result<()> {
    let (path, hash) = argument
        .rsplit_once('#')
        .ok_or("--package needs ARCHIVE#SHA256")?;
    let digest = format!("sha256:{}", hash.strip_prefix("sha256:").unwrap_or(hash));
    let bytes = std::fs::read(path).map_err(|_| "cannot read package archive")?;
    if conduit_core::packages::bytes_digest(&bytes) != digest {
        return Err(format!(
            "install archive {path:?}: package archive digest mismatch; use the SHA-256 of these exact archive bytes"
        ));
    }
    // Discover metadata with the core's virtual archive/link semantics, without extraction.
    let manifest = archive_metadata::manifest(&bytes)?;
    let address = manifest["address"]
        .as_str()
        .ok_or("package manifest has no address")?;
    let version = manifest["version"]
        .as_str()
        .ok_or("package manifest has no version")?;
    let directory = shell
        .writer
        .lock()
        .map_err(|_| "writer lock poisoned")?
        .store
        .directory
        .join("archive-catalog")
        .join(digest.trim_start_matches("sha256:"));
    let directory =
        url::Url::from_directory_path(directory).map_err(|_| "invalid package directory")?;
    let reply = shell.submit(serde_json::json!({"kind":"install-archive","archive":{"name":path,"address":address,"version":version,"digest":digest,"directory":directory.as_str(),"completion":{"status":"complete","bytes":bytes}},"snapshots":[],"builtin":shell.context.builtin}), &crate::MissingExecutor)?;
    if reply["accepted"] != true {
        return Err(format!("package admission failed: {reply}"));
    }
    Ok(())
}

#[path = "archive_metadata.rs"]
mod archive_metadata;
