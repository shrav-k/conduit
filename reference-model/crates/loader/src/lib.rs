//! The strict loader of the first build: increment 1, action item A43.
//!
//! Not binding: the specification in `spec/` alone binds, and this crate follows it. It reads a
//! Conduit document in YAML or JSON exactly as the specification says (SCH-012, SCH-013,
//! SCH-017, SCH-018, SCH-005, SCH-006, SCH-020, SCH-021), reads a project's directory as
//! MAN-001, MAN-002 and MAN-011 say, and validates each document against the JSON Schemas of
//! the specification, bundled as `tools/check_schemas.py` bundles them, with the formats of the
//! profile asserted (SCH-001, SCH-004) and patterns evaluated as ECMA-262 (SCH-016).
//!
//! Every rejection is a value, a [`Diagnostic`], that names the promise it rests on, the file,
//! the document and the place. A file that is not YAML or JSON is rejected for that alone;
//! otherwise every fault of every document is reported. Nothing here panics on any input.
//!
//! What touches the file system is at the edge, and says so: [`read_file`], [`read_project`]
//! and [`Specification::read`]. Everything else takes what was read: [`read_text`],
//! [`read_bytes`], [`read_submitted_files`], [`Specification::from_documents`] and
//! [`Specification::check`], which gives one document the verdict that reading a project gives
//! it.

mod authored;
mod diagnostic;
pub mod formats;
mod json;
mod number;
mod project;
mod read;
pub mod rules;
mod schemas;
mod value;
mod yaml;

pub use diagnostic::{Diagnostic, Diagnostics, Position};
pub use project::{FileRole, MANIFEST_NAMES, Project, read_project, read_submitted_files};
pub use read::{Document, Syntax, read_bytes, read_file, read_text};
pub use schemas::{Names, Role, Specification, compile};
pub use value::{LARGEST_WHOLE, Node, Value};

/// How deeply the values of a document may nest: a map or a list inside another counts one
/// level. A limit of an engine, which DL10 of the implementation-defined list leaves to it,
/// past which a document is rejected under SUB-011; it keeps a document from exhausting the
/// stack of the reader or of the validator.
pub const MAX_DEPTH: usize = 128;

/// The rejection of a document whose values nest deeper than [`MAX_DEPTH`], which names the
/// limit (SUB-011).
pub(crate) fn too_deep() -> Diagnostic {
    Diagnostic::new(
        rules::SUB_011,
        format!(
            "values nested more than {MAX_DEPTH} levels deep, the limit of nesting that this \
             engine states under DL10"
        ),
    )
}
