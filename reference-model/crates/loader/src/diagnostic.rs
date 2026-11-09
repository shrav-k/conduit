//! Rejections as values. Every rejection the loader makes is a [`Diagnostic`] that names the
//! promise it rests on, the file, the document within the file and the place within the
//! document, as far as each is known.

use std::fmt;

/// A place in a file: a line and a column, both counted from 1, the column in characters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Position {
    /// A position from a line and a column counted from 1, saturated at the largest `u32`.
    pub fn new(line: usize, column: usize) -> Self {
        let narrow = |n: usize| u32::try_from(n).unwrap_or(u32::MAX);
        Position {
            line: narrow(line),
            column: narrow(column),
        }
    }
}

/// Why a file, a document or a project was rejected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// The ID of the promise the rejection rests on, such as `SCH-012`: a diagnostic's
    /// `ruleId` (IFC-025). See [`crate::rules`].
    pub rule: &'static str,
    /// The file, as it was named to the loader, where there is one.
    pub file: Option<String>,
    /// The document within the file, counted from 1, where the rejection is of one document.
    pub document: Option<usize>,
    /// The place within the document, as a JSON Pointer.
    pub pointer: Option<String>,
    /// Where that place, or the fault, is in the file.
    pub position: Option<Position>,
    /// What is wrong, in words.
    pub message: String,
}

impl Diagnostic {
    pub fn new(rule: &'static str, message: impl Into<String>) -> Self {
        Diagnostic {
            rule,
            file: None,
            document: None,
            pointer: None,
            position: None,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn in_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    #[must_use]
    pub fn in_document(mut self, index: usize) -> Self {
        self.document = Some(index);
        self
    }

    #[must_use]
    pub fn at_pointer(mut self, pointer: impl Into<String>) -> Self {
        self.pointer = Some(pointer.into());
        self
    }

    #[must_use]
    pub fn at(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut place = Vec::new();
        if let Some(file) = &self.file {
            place.push(file.clone());
        }
        if let Some(document) = self.document {
            place.push(format!("document {document}"));
        }
        if let Some(position) = self.position {
            place.push(format!(
                "line {}, column {}",
                position.line, position.column
            ));
        }
        if let Some(pointer) = &self.pointer {
            place.push(format!(
                "at {}",
                if pointer.is_empty() { "/" } else { pointer }
            ));
        }
        if !place.is_empty() {
            write!(f, "{}: ", place.join(", "))?;
        }
        write!(f, "{} ({})", self.message, self.rule)
    }
}

/// One or more rejections. A project, a file or the specification is rejected as a whole when
/// any part of it is, and every rejection found is reported. A file that is not YAML or JSON
/// at all is the exception: reading stops there, and that is its one rejection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostics(pub Vec<Diagnostic>);

impl Diagnostics {
    /// The rules the rejections rest on, in order, each once.
    pub fn rules(&self) -> Vec<&'static str> {
        let mut rules: Vec<&'static str> = Vec::new();
        for diagnostic in &self.0 {
            if !rules.contains(&diagnostic.rule) {
                rules.push(diagnostic.rule);
            }
        }
        rules
    }

    /// The same rejections, each in this file.
    #[must_use]
    pub fn in_file(self, file: &str) -> Self {
        Diagnostics(self.0.into_iter().map(|d| d.in_file(file)).collect())
    }
}

impl From<Diagnostic> for Diagnostics {
    fn from(diagnostic: Diagnostic) -> Self {
        Diagnostics(vec![diagnostic])
    }
}

impl fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, diagnostic) in self.0.iter().enumerate() {
            if index > 0 {
                writeln!(f)?;
            }
            write!(f, "{diagnostic}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Diagnostic {}

impl std::error::Error for Diagnostics {}

/// The largest offset at or before this one that begins a character of the text.
pub(crate) fn char_boundary(text: &str, offset: usize) -> usize {
    let mut offset = offset.min(text.len());
    while !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

/// The line and column of a byte offset in a text. It reads the text up to the offset: for
/// many offsets of one text, use a [`Locator`].
pub(crate) fn position_of(text: &str, offset: usize) -> Position {
    Locator::new(text).position(offset)
}

/// The line and column of byte offsets in one text. Asked for offsets in increasing order, as
/// a reader asks for them, it reads the text once in all, so that reading a large file takes
/// time in proportion to its size; an earlier offset is found again from the start.
pub(crate) struct Locator<'t> {
    text: &'t str,
    /// The last offset asked for, and its line and column.
    cursor: std::cell::Cell<(usize, usize, usize)>,
}

impl<'t> Locator<'t> {
    pub(crate) fn new(text: &'t str) -> Self {
        Locator {
            text,
            cursor: std::cell::Cell::new((0, 1, 1)),
        }
    }

    pub(crate) fn position(&self, offset: usize) -> Position {
        let offset = char_boundary(self.text, offset);
        let (mut at, mut line, mut column) = self.cursor.get();
        if offset < at {
            (at, line, column) = (0, 1, 1);
        }
        for c in self.text[at..offset].chars() {
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        self.cursor.set((offset, line, column));
        Position::new(line, column)
    }
}
