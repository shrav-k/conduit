//! What the tests share: where the repository is, the specification read once, and documents
//! written for a case with the names that `spec/names.yaml` defines.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use conduit_loader::{Diagnostics, Project, Specification, read_submitted_files};

/// The root of the repository: this crate is `engine/crates/loader`.
pub fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("the repository holds this crate")
}

pub fn spec_dir() -> PathBuf {
    repository().join("spec")
}

pub fn examples() -> PathBuf {
    spec_dir().join("schemas").join("examples")
}

/// The specification, read from `spec/` once for all the tests of one binary.
pub fn specification() -> &'static Specification {
    static SPECIFICATION: OnceLock<Specification> = OnceLock::new();
    SPECIFICATION.get_or_init(|| {
        Specification::read(&spec_dir())
            .unwrap_or_else(|faults| panic!("the specification: {faults}"))
    })
}

/// The `apiVersion` of this version, from `spec/names.yaml`.
pub fn api() -> String {
    specification().names().api_version.clone()
}

/// The address of the core package, from `spec/names.yaml`.
pub fn core() -> String {
    specification().names().core_package.clone()
}

/// The files directly in a directory, in the order of their names, but for a name that begins
/// with a dot, which is not read (MAN-011), as `tools/check_schemas.py` does not read it.
pub fn files_in(directory: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(directory)
        .unwrap_or_else(|e| panic!("{}: {e}", directory.display()))
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|path| path.is_file() && !name_of(path).starts_with('.'))
        .collect();
    files.sort();
    files
}

/// The name of a file, as text.
pub fn name_of(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// The directories directly in a directory, in the order of their names.
pub fn directories_in(directory: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(directory)
        .unwrap_or_else(|e| panic!("{}: {e}", directory.display()))
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|path| path.is_dir())
        .collect();
    found.sort();
    found
}

/// A template of the family Interface, valid against the schemas, with this name.
pub fn interface(name: &str) -> String {
    format!(
        "apiVersion: {api}\nkind: Interface\nmetadata:\n  name: {name}\nspec:\n  type: {core}#interface.v1\n",
        api = api(),
        core = core()
    )
}

/// A manifest that is valid against `manifest.v1`.
pub fn manifest() -> String {
    format!("apiVersion: {api}\n", api = api())
}

/// Hand files over together, without a directory, as name and text.
pub fn submit(files: &[(&str, &str)]) -> Result<Project, Diagnostics> {
    let files: Vec<(String, Vec<u8>)> = files
        .iter()
        .map(|(name, text)| (name.to_string(), text.as_bytes().to_vec()))
        .collect();
    read_submitted_files(&files, specification())
}
