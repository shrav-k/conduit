//! Reading a project: the files handed over together in one registration, or a directory
//! (MAN-001, MAN-002, MAN-011, SUB-005, SUB-010).
//!
//! A directory is read as MAN-011 says: its documents are the files in the directory itself
//! whose names end in `.yaml`, `.yml` or `.json` and do not begin with a dot, apart from the
//! manifest, which is the one file named `conduit.yaml`, `conduit.yml` or `conduit.json`
//! (MAN-001). Nothing in a directory below it is read, and no other file is. More than one
//! manifest is rejected (MAN-002). Each document is then validated against `document.v1` and
//! the manifest against `manifest.v1`.
//!
//! Names and endings are compared exactly, as MAN-011 says, so `Conduit.yaml` is not the
//! manifest and `notes.YAML` is not a document. A name that begins with a dot is not read, so
//! neither `.notes.yaml` nor the `._templates.yaml` that `tar` on macOS stores beside
//! `templates.yaml` is. A link is followed as the file system follows it: one that leads to a
//! directory is not entered, and one that leads nowhere, under a name that MAN-011 reads, is a
//! file that cannot be read, as is anything there but a regular file, rejected under SUB-010
//! and never skipped. The first build's log of increment 1 records these as the coordinator's
//! decision on its finding 1-06.

use std::path::{Path, PathBuf};

use crate::diagnostic::{Diagnostic, Diagnostics};
use crate::read::{Document, Syntax, content_of, documents_of};
use crate::rules::{MAN_001, MAN_002, SUB_010};
use crate::schemas::{Role, Specification};

/// The names a manifest may have (MAN-001).
pub const MANIFEST_NAMES: [&str; 3] = ["conduit.yaml", "conduit.yml", "conduit.json"];

/// What a file of a project is, by its name alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileRole {
    /// The manifest (MAN-001).
    Manifest(Syntax),
    /// A file of documents (MAN-011).
    Documents(Syntax),
    /// Not a document, and not read (MAN-011): a name with another ending, or one that begins
    /// with a dot.
    NotRead,
}

impl FileRole {
    pub fn of(name: &str) -> FileRole {
        if name.starts_with('.') {
            return FileRole::NotRead;
        }
        match Syntax::of(name) {
            Some(syntax) if MANIFEST_NAMES.contains(&name) => FileRole::Manifest(syntax),
            Some(syntax) => FileRole::Documents(syntax),
            None => FileRole::NotRead,
        }
    }
}

/// A project, read and validated against the schemas.
#[derive(Clone, Debug)]
pub struct Project {
    /// The directory it was read from, where it was handed over as one. A relative `fetch` and
    /// a program given as a relative path are later resolved against it (MAN-006, MAN-010,
    /// SUB-010); the loader resolves nothing.
    pub directory: Option<PathBuf>,
    /// The one manifest, where the project has one.
    pub manifest: Option<Document>,
    /// Every other document, in the order of the files' names and then as written. The order
    /// has no meaning (SUB-001).
    pub documents: Vec<Document>,
}

/// Files, each by its name and its bytes.
pub(crate) type Named = Vec<(String, Vec<u8>)>;

/// The files of a project, read strictly and not yet validated.
pub(crate) struct Files {
    pub manifest: Option<Document>,
    pub documents: Vec<Document>,
}

/// Read a project's directory and validate what it holds (SUB-010). Reads the file system.
pub fn read_project(directory: &Path, spec: &Specification) -> Result<Project, Diagnostics> {
    let (files, unread) = directory_files(directory)?;
    load(&files, unread, Some(directory), spec)
}

/// Read files that are handed over together without a directory, each given by its name and
/// its bytes, and validate what they hold. Several files named as a manifest are rejected under
/// MAN-002, as in a directory. Pure.
pub fn read_submitted_files(
    files: &[(String, Vec<u8>)],
    spec: &Specification,
) -> Result<Project, Diagnostics> {
    load(files, Vec::new(), None, spec)
}

/// Read and validate the files of a project, beside the faults of those that could not be
/// read.
fn load(
    files: &[(String, Vec<u8>)],
    mut faults: Vec<Diagnostic>,
    directory: Option<&Path>,
    spec: &Specification,
) -> Result<Project, Diagnostics> {
    let (read, read_faults) = read_files(files, directory);
    faults.extend(read_faults);
    if let Some(manifest) = &read.manifest
        && let Err(found) = spec.check(manifest, Role::Manifest)
    {
        faults.extend(found.0);
    }
    for document in &read.documents {
        if let Err(found) = spec.check(document, Role::Document) {
            faults.extend(found.0);
        }
    }
    if !faults.is_empty() {
        return Err(Diagnostics(faults));
    }
    Ok(Project {
        directory: directory.map(Path::to_path_buf),
        manifest: read.manifest,
        documents: read.documents,
    })
}

/// The files of a directory that MAN-011 reads, each with its name and bytes, in the order of
/// their names, and a fault for each that has a name MAN-011 reads and cannot be read. A
/// directory below is not entered, and a file with any other ending, or whose name begins with
/// a dot, is not read. Only a directory that cannot be read at all is an error. Reads the file
/// system.
pub(crate) fn directory_files(directory: &Path) -> Result<(Named, Vec<Diagnostic>), Diagnostic> {
    let unreadable = |error: std::io::Error| {
        Diagnostic::new(SUB_010, format!("the directory cannot be read: {error}"))
            .in_file(directory.display().to_string())
    };
    let mut files = Vec::new();
    let mut faults = Vec::new();
    for entry in std::fs::read_dir(directory).map_err(unreadable)? {
        let entry = entry.map_err(unreadable)?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if FileRole::of(&name) == FileRole::NotRead {
            continue;
        }
        let path = entry.path();
        let cannot = |why: String| {
            Diagnostic::new(SUB_010, format!("cannot be read: {why}"))
                .in_file(path.display().to_string())
        };
        // `metadata` follows a link, so a link is read as what it leads to.
        match std::fs::metadata(&path) {
            Ok(metadata) if metadata.is_dir() => {}
            Ok(metadata) if metadata.is_file() => match std::fs::read(&path) {
                Ok(bytes) => files.push((name, bytes)),
                Err(error) => faults.push(cannot(error.to_string())),
            },
            Ok(_) => faults.push(cannot("it is not a regular file".to_string())),
            Err(error) => faults.push(cannot(error.to_string())),
        }
    }
    files.sort();
    faults.sort_by(|a, b| a.file.cmp(&b.file));
    Ok((files, faults))
}

/// Read the files of one project strictly: which is the manifest, each file's syntax, and that
/// the manifest holds one document. What was read, and every fault found.
pub(crate) fn read_files(
    files: &[(String, Vec<u8>)],
    directory: Option<&Path>,
) -> (Files, Vec<Diagnostic>) {
    let shown = |name: &str| match directory {
        Some(directory) => directory.join(name).display().to_string(),
        None => name.to_string(),
    };
    let mut sorted: Vec<&(String, Vec<u8>)> = files.iter().collect();
    sorted.sort();
    let mut faults = Vec::new();
    let mut read = Files {
        manifest: None,
        documents: Vec::new(),
    };
    let manifests: Vec<&str> = sorted
        .iter()
        .filter(|(name, _)| matches!(FileRole::of(name), FileRole::Manifest(_)))
        .map(|(name, _)| name.as_str())
        .collect();
    if manifests.len() > 1 {
        let mut fault = Diagnostic::new(
            MAN_002,
            format!(
                "more than one manifest: {}; a project has at most one",
                manifests.join(", ")
            ),
        );
        if let Some(directory) = directory {
            fault = fault.in_file(directory.display().to_string());
        }
        faults.push(fault);
    }
    for (name, bytes) in sorted {
        let file = shown(name);
        match FileRole::of(name) {
            FileRole::NotRead => {}
            FileRole::Manifest(syntax) => {
                if manifests.len() > 1 {
                    continue;
                }
                match content_of(syntax, bytes) {
                    Err(found) => faults.extend(found.in_file(&file).0),
                    Ok(content) if content.json_array => faults.push(
                        Diagnostic::new(
                            MAN_001,
                            "a JSON manifest holds an array; a manifest is one document",
                        )
                        .in_file(file),
                    ),
                    Ok(content) if content.documents.len() != 1 => faults.push(
                        Diagnostic::new(
                            MAN_001,
                            format!(
                                "the manifest holds {} documents, and it holds exactly one",
                                content.documents.len()
                            ),
                        )
                        .in_file(file),
                    ),
                    Ok(content) => {
                        read.manifest = documents_of(&file, content.documents).pop();
                    }
                }
            }
            FileRole::Documents(syntax) => match content_of(syntax, bytes) {
                Err(found) => faults.extend(found.in_file(&file).0),
                Ok(content) => read
                    .documents
                    .extend(documents_of(&file, content.documents)),
            },
        }
    }
    (read, faults)
}
