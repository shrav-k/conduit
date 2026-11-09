//! Reading one file: its syntax, given by the ending of its name, and its documents.

use std::path::Path;

use crate::diagnostic::{Diagnostic, Diagnostics, position_of};
use crate::rules::{SUB_005, SUB_010};
use crate::value::Node;
use crate::{json, yaml};

/// The syntax of a file, which the ending of its name gives (MAN-001, MAN-011).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syntax {
    /// A name ending in `.yaml` or `.yml`.
    Yaml,
    /// A name ending in `.json`, the JSON form that SCH-013 allows.
    Json,
}

impl Syntax {
    /// The syntax of a file by its name, or none for a name with any other ending, which is not
    /// a document and is not read (MAN-011). The ending is compared exactly, as MAN-011 says, so
    /// `.YAML` is another ending.
    pub fn of(name: &str) -> Option<Syntax> {
        if name.ends_with(".yaml") || name.ends_with(".yml") {
            Some(Syntax::Yaml)
        } else if name.ends_with(".json") {
            Some(Syntax::Json)
        } else {
            None
        }
    }
}

/// One document, as read from a file.
#[derive(Clone, Debug)]
pub struct Document {
    /// The file, as it was named to the loader.
    pub file: String,
    /// Which document of the file it is, counted from 1. For a JSON file that holds an array,
    /// the item's place in the array, counted from 1.
    pub index: usize,
    pub root: Node,
}

/// What a file holds.
pub(crate) struct Content {
    pub documents: Vec<Node>,
    /// Whether the file is JSON and holds an array (SCH-018, MAN-001).
    pub json_array: bool,
}

/// Read the documents of a text, strictly. Pure. A text that is not YAML or JSON is rejected
/// for that alone; otherwise every fault of every document is reported.
pub fn read_text(syntax: Syntax, text: &str) -> Result<Vec<Node>, Diagnostics> {
    content(syntax, text).map(|content| content.documents)
}

pub(crate) fn content(syntax: Syntax, text: &str) -> Result<Content, Diagnostics> {
    match syntax {
        Syntax::Yaml => yaml::read(text).map(|documents| Content {
            documents,
            json_array: false,
        }),
        Syntax::Json => json::read(text).map(|content| Content {
            documents: content.documents,
            json_array: content.array,
        }),
    }
}

/// The text of a file's bytes. Every file is UTF-8, the schema profile's own rule, with I-JSON
/// (RFC 7493, section 2.1) as its precedent, and YAML's other encodings are not read: a file that is
/// not UTF-8 holds no document, and is rejected under SUB-005.
pub(crate) fn text_of(bytes: &[u8]) -> Result<&str, Diagnostic> {
    std::str::from_utf8(bytes).map_err(|error| {
        let valid = &bytes[..error.valid_up_to()];
        let at = position_of(std::str::from_utf8(valid).unwrap_or(""), valid.len());
        Diagnostic::new(SUB_005, "not UTF-8 text").at(at)
    })
}

/// Read the documents of a file's bytes, strictly. Pure.
pub fn read_bytes(syntax: Syntax, bytes: &[u8]) -> Result<Vec<Node>, Diagnostics> {
    read_text(syntax, text_of(bytes)?)
}

/// Read what a file's bytes hold, strictly. Pure.
pub(crate) fn content_of(syntax: Syntax, bytes: &[u8]) -> Result<Content, Diagnostics> {
    content(syntax, text_of(bytes)?)
}

/// Read a file whose name ends in `.yaml`, `.yml` or `.json`. Reads the file system.
pub fn read_file(path: &Path) -> Result<Vec<Document>, Diagnostics> {
    let name = path.display().to_string();
    let syntax = Syntax::of(&name).ok_or_else(|| {
        Diagnostic::new(
            SUB_005,
            "not a document: the name does not end in .yaml, .yml or .json",
        )
        .in_file(name.clone())
    })?;
    let bytes = std::fs::read(path).map_err(|error| {
        Diagnostic::new(SUB_010, format!("cannot be read: {error}")).in_file(name.clone())
    })?;
    let nodes = read_bytes(syntax, &bytes).map_err(|d| d.in_file(&name))?;
    Ok(documents_of(&name, nodes))
}

pub(crate) fn documents_of(file: &str, nodes: Vec<Node>) -> Vec<Document> {
    nodes
        .into_iter()
        .enumerate()
        .map(|(index, root)| Document {
            file: file.to_string(),
            index: index + 1,
            root,
        })
        .collect()
}
