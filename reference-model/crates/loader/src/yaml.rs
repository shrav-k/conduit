//! Reading YAML strictly: YAML 1.2.2 with its core schema, narrowed as the schema profile says
//! ("YAML features not accepted", in `spec/schema-profile.md`).
//!
//! The text is parsed by `saphyr-parser` into events, and the values are built here from the
//! events, so that every narrowing is a decision of this file and none is a default of a crate:
//!
//! - an anchor, an alias, a tag (`!`, `!!str` or any other) and a merge key, a plain `<<` used
//!   as a key, are rejected (SCH-012), and so is a duplicate key and a key that is not a
//!   string, a plain scalar that the core schema reads as null, a boolean or a number, or a
//!   key that is a map or a list (SCH-012);
//! - a plain scalar is read by the core schema's own rules (YAML 1.2.2, section 10.3.2), and
//!   any other scalar is a string: `yes` is a string, `"15"` is a string, `15` is a number;
//! - `.inf`, `.nan`, `0x1F` and `0o17` have no JSON form and are rejected (SCH-017);
//! - a number outside its range is rejected (SCH-006, SCH-021), as `crate::number` reads it;
//! - values nest at most [`crate::MAX_DEPTH`] deep (DL10).
//!
//! Three things the parser does not check, this file checks, so that a text the parser takes
//! but YAML 1.2.2 does not is not read:
//!
//! - the characters of the stream (YAML 1.2.2, section 5.1): a C0 control character other than
//!   tab, line feed and carriage return is not YAML anywhere, and DEL, a C1 control character
//!   other than NEL, U+FFFE and U+FFFF only inside a quoted scalar, which takes any character
//!   but a C0 control (`nb-json`);
//! - a `%YAML` directive for a version other than 1.x, which YAML 1.2.2 has a reader reject
//!   (section 6.8.1); a `%YAML 1.1` document is read as YAML 1.2;
//! - a line that begins with `---` or `...` inside a block scalar at the root, which is a
//!   marker of the document and never its content (`c-forbidden`), and which the parser reads
//!   as content.
//!
//! A file holds any number of documents. A text that is not YAML is rejected for that alone,
//! under SUB-005, where reading stopped. Any other fault is in a value: it is reported with the
//! place of the value as a JSON Pointer, and reading goes on, so that every fault of every
//! document is reported, whatever the order of the documents and of their members (SUB-001).

use std::borrow::Cow;
use std::collections::HashSet;

use saphyr_parser::{Event, Parser, ScalarStyle, Span, Tag};

use crate::diagnostic::{Diagnostic, Diagnostics, Position, position_of};
use crate::number::{NumberFault, is_decimal, read_number};
use crate::rules::{DL10, SCH_006, SCH_012, SCH_017, SCH_021, SUB_005};
use crate::value::{Node, Value};

/// Read every document of a YAML text.
pub(crate) fn read(text: &str) -> Result<Vec<Node>, Diagnostics> {
    let characters = Characters::scan(text)?;
    let mut reader = Reader::default();
    let mut offsets = Offsets::new(text);
    let mut parser = Parser::new_from_str(text);
    // The character index where the previous event ended: directives lie between it and an
    // explicit start of a document.
    let mut previous_end = 0;
    let mut quoted = characters.quoted_only.iter().peekable();
    while let Some(next) = parser.next_event() {
        let (event, span) = match next {
            Ok(next) => next,
            Err(error) => return Err(reader.stopped(&error).into()),
        };
        let at = start_of(&span);
        match event {
            Event::Nothing | Event::StreamStart => {}
            Event::StreamEnd => break,
            Event::DocumentStart(explicit) => {
                if explicit {
                    // A document that ends at a `---` ends where that marker ends, after it
                    // begins: the directives lie before both.
                    let from = offsets.byte(previous_end.min(span.start.index()));
                    let to = offsets.byte(span.start.index());
                    directives(text, from, to)?;
                }
            }
            Event::DocumentEnd => reader.end_document(at),
            Event::Alias(_) => reader.alias(at),
            Event::Scalar(value, style, anchor, tag) => {
                match style {
                    ScalarStyle::SingleQuoted | ScalarStyle::DoubleQuoted => {
                        // The characters that only a quoted scalar may hold are each in one.
                        while let Some((index, c, position)) =
                            quoted.next_if(|(index, _, _)| *index < span.end.index())
                        {
                            if *index < span.start.index() {
                                return Err(not_printable(*c, *position).into());
                            }
                        }
                    }
                    ScalarStyle::Literal | ScalarStyle::Folded => {
                        let from = offsets.byte(span.start.index());
                        let to = offsets.byte(span.end.index());
                        no_marker(text, from, to, span.start.col())?;
                    }
                    ScalarStyle::Plain => {}
                }
                reader.scalar(value, style, anchor, tag.as_ref(), at);
            }
            Event::SequenceStart(anchor, tag) => {
                reader.open(Frame::Sequence(Vec::new()), anchor, tag.as_ref(), at);
            }
            Event::MappingStart(anchor, tag) => {
                let frame = Frame::Mapping {
                    members: Vec::new(),
                    keys: HashSet::new(),
                    key: None,
                };
                reader.open(frame, anchor, tag.as_ref(), at);
            }
            Event::SequenceEnd | Event::MappingEnd => reader.close()?,
        }
        previous_end = span.end.index();
    }
    if let Some((_, c, position)) = quoted.next() {
        return Err(not_printable(*c, *position).into());
    }
    if reader.faults.is_empty() {
        Ok(reader.documents)
    } else {
        Err(Diagnostics(reader.faults))
    }
}

fn start_of(span: &Span) -> Position {
    Position::new(span.start.line(), span.start.col() + 1)
}

/// What YAML 1.2.2 allows of the characters of a stream (section 5.1, `c-printable`), beyond
/// what the parser checks.
struct Characters {
    /// The index, the character and the place of each character that YAML allows only inside
    /// a quoted scalar: DEL, a C1 control character other than NEL, U+FFFE and U+FFFF.
    quoted_only: Vec<(usize, char, Position)>,
}

impl Characters {
    /// Reject a C0 control character other than tab, line feed and carriage return, which YAML
    /// allows nowhere, and find those that only a quoted scalar may hold.
    fn scan(text: &str) -> Result<Characters, Diagnostic> {
        let mut quoted_only = Vec::new();
        let (mut line, mut column) = (1, 1);
        for (index, c) in text.chars().enumerate() {
            let position = Position::new(line, column);
            match c {
                '\t' | '\n' | '\r' => {}
                '\0'..='\x1f' => return Err(not_printable(c, position)),
                '\x7f' | '\u{80}'..='\u{84}' | '\u{86}'..='\u{9f}' | '\u{fffe}' | '\u{ffff}' => {
                    quoted_only.push((index, c, position));
                }
                _ => {}
            }
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        Ok(Characters { quoted_only })
    }
}

/// A character that YAML 1.2.2 does not allow where it stands.
fn not_printable(c: char, at: Position) -> Diagnostic {
    Diagnostic::new(
        SUB_005,
        format!(
            "not YAML: U+{:04X}, a character that YAML 1.2.2 does not allow here",
            u32::from(c)
        ),
    )
    .at(at)
}

/// The byte offsets of the character indices that the parser gives, each found from the one
/// asked for before it, so that a file is read once in all when they are asked for in
/// increasing order.
struct Offsets<'t> {
    text: &'t str,
    /// The last character index asked for, and its byte offset.
    cursor: (usize, usize),
}

impl<'t> Offsets<'t> {
    fn new(text: &'t str) -> Self {
        Offsets {
            text,
            cursor: (0, 0),
        }
    }

    fn byte(&mut self, index: usize) -> usize {
        let (from, byte) = self.cursor;
        let offset = if index >= from {
            self.text[byte..]
                .char_indices()
                .nth(index - from)
                .map_or(self.text.len(), |(at, _)| byte + at)
        } else {
            self.text[..byte]
                .char_indices()
                .rev()
                .nth(from - index - 1)
                .map_or(0, |(at, _)| at)
        };
        self.cursor = (index, offset);
        offset
    }
}

/// The directives before an explicit start of a document, between two byte offsets. YAML
/// 1.2.2 (section 6.8.1) has a processor reject a document whose `%YAML` directive names
/// another major version, and read one that names version 1.1 as version 1.2. A `%TAG`
/// directive is allowed, and a tag is still rejected wherever it is used; a reserved
/// directive, such as `%YAMLX`, is ignored, as YAML 1.2.2 says.
fn directives(text: &str, from: usize, to: usize) -> Result<(), Diagnostic> {
    let mut start = from;
    for line in text.get(from..to).unwrap_or("").split_inclusive('\n') {
        let directive = line.trim_start_matches('\u{feff}');
        if let Some(rest) = directive.strip_prefix("%YAML")
            && rest.starts_with([' ', '\t'])
        {
            let version = rest.split_whitespace().next().unwrap_or("");
            let major = version
                .split('.')
                .next()
                .and_then(|m| m.parse::<u32>().ok());
            if major != Some(1) {
                return Err(Diagnostic::new(
                    SUB_005,
                    format!("not YAML 1.2: a `%YAML {version}` directive"),
                )
                .at(position_of(text, start + line.len() - directive.len())));
            }
        }
        start += line.len();
    }
    Ok(())
}

/// A block scalar, between two byte offsets, whose first character is at this column. A line
/// of it that begins at the first column with `---` or `...`, and then a space, a tab or the
/// end of the line, is a marker of the document and not content (YAML 1.2.2, `c-forbidden`).
/// The parser reads such a line into a block scalar at the root; the text is then not read as
/// YAML 1.2.2 reads it, and is rejected.
fn no_marker(text: &str, from: usize, to: usize, column: usize) -> Result<(), Diagnostic> {
    let mut start = from;
    for (index, line) in text
        .get(from..to)
        .unwrap_or("")
        .split_inclusive('\n')
        .enumerate()
    {
        let at_first_column = index > 0 || column == 0;
        let marker = ["---", "...", "\u{feff}---", "\u{feff}..."]
            .iter()
            .find_map(|m| line.strip_prefix(m));
        if at_first_column
            && let Some(rest) = marker
            && (rest.is_empty() || rest.starts_with([' ', '\t', '\r', '\n']))
        {
            return Err(Diagnostic::new(
                SUB_005,
                "not YAML as YAML 1.2.2 reads it: a line that marks a document inside a block \
                 scalar, which the parser takes for content",
            )
            .at(position_of(text, start)));
        }
        start += line.len();
    }
    Ok(())
}

/// Whether a node has an anchor or a tag, each rejected (SCH-012), and which.
fn properties(anchor: usize, tag: Option<&Cow<'_, Tag>>) -> Option<Diagnostic> {
    if let Some(tag) = tag {
        return Some(Diagnostic::new(
            SCH_012,
            format!("a tag, `{}{}`", tag.handle, tag.suffix),
        ));
    }
    (anchor != 0).then(|| Diagnostic::new(SCH_012, "an anchor"))
}

/// A reference token of a JSON Pointer (RFC 6901).
fn token(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}

enum Frame {
    Sequence(Vec<Node>),
    Mapping {
        members: Vec<(String, Node)>,
        keys: HashSet<String>,
        /// The key whose value comes next.
        key: Option<String>,
    },
}

/// A map or a list being read: what it holds so far, where it starts, and its place in the
/// document as a JSON Pointer.
struct Open {
    frame: Frame,
    at: Position,
    pointer: String,
}

#[derive(Default)]
struct Reader {
    stack: Vec<Open>,
    root: Option<Node>,
    documents: Vec<Node>,
    faults: Vec<Diagnostic>,
    /// How many levels deep the reader is inside a map or a list that is beyond the limit of
    /// nesting, and where it began: its content is not read (DL10).
    skipping: usize,
    skipped_at: Position,
}

impl Reader {
    /// A fault in the document being read, at this place.
    fn fault(&mut self, diagnostic: Diagnostic, at: Position, pointer: String) {
        let index = self.documents.len() + 1;
        self.faults
            .push(diagnostic.at(at).in_document(index).at_pointer(pointer));
    }

    /// The parser stopped on an error, and nothing after it can be read. The parser keeps at
    /// most 255 levels of flow collections, and reads ahead of the events it gives, so a deeper
    /// document may stop it before the reader's own limit, or just after it: that is the limit
    /// (DL10), reported as the reader found it where it did, and not text that is not YAML.
    fn stopped(&mut self, error: &saphyr_parser::ScanError) -> Diagnostic {
        let marker = error.marker();
        let at = Position::new(marker.line(), marker.col() + 1);
        if error.info() != "recursion limit exceeded" {
            return Diagnostic::new(SUB_005, format!("not YAML: {}", error.info())).at(at);
        }
        let index = self.documents.len() + 1;
        let found = self
            .faults
            .iter()
            .position(|d| d.rule == DL10 && d.document == Some(index));
        match found {
            Some(found) => self.faults.swap_remove(found),
            None => Diagnostic::new(
                DL10,
                format!("values nested more than {} deep", crate::MAX_DEPTH),
            )
            .at(at)
            .in_document(index),
        }
    }

    /// Whether the next node is a key of a map.
    fn expects_key(&self) -> bool {
        matches!(
            self.stack.last(),
            Some(Open {
                frame: Frame::Mapping { key: None, .. },
                ..
            })
        )
    }

    /// The place of the next node: the next item of a list, the value of a map's key, or the
    /// root. Where the next node is a key, the place of its map.
    fn place_of_next(&self) -> String {
        match self.stack.last() {
            None => String::new(),
            Some(open) => match &open.frame {
                Frame::Sequence(items) => format!("{}/{}", open.pointer, items.len()),
                Frame::Mapping { key: Some(key), .. } => {
                    format!("{}/{}", open.pointer, token(key))
                }
                Frame::Mapping { key: None, .. } => open.pointer.clone(),
            },
        }
    }

    fn open(&mut self, frame: Frame, anchor: usize, tag: Option<&Cow<'_, Tag>>, at: Position) {
        if self.skipping > 0 {
            self.skipping += 1;
            return;
        }
        let pointer = self.place_of_next();
        if let Some(fault) = properties(anchor, tag) {
            self.fault(fault, at, pointer.clone());
        }
        if self.expects_key() {
            let fault = Diagnostic::new(SCH_012, "a key that is not a string: a map or a list");
            self.fault(fault, at, pointer.clone());
        }
        if self.stack.len() >= crate::MAX_DEPTH {
            let fault = Diagnostic::new(
                DL10,
                format!("values nested more than {} deep", crate::MAX_DEPTH),
            );
            self.fault(fault, at, pointer);
            self.skipping = 1;
            self.skipped_at = at;
            return;
        }
        self.stack.push(Open { frame, at, pointer });
    }

    fn close(&mut self) -> Result<(), Diagnostic> {
        if self.skipping > 0 {
            self.skipping -= 1;
            if self.skipping == 0 {
                self.place(Node::new(Value::Null, self.skipped_at));
            }
            return Ok(());
        }
        // The parser ends only what it started; were it ever not to, the text is not read.
        let Some(Open { frame, at, .. }) = self.stack.pop() else {
            return Err(Diagnostic::new(
                SUB_005,
                "not YAML: the end of a collection that did not start",
            ));
        };
        let value = match frame {
            Frame::Sequence(items) => Value::Array(items),
            Frame::Mapping { members, .. } => Value::Object(members),
        };
        self.place(Node::new(value, at));
        Ok(())
    }

    fn alias(&mut self, at: Position) {
        if self.skipping > 0 {
            return;
        }
        let pointer = self.place_of_next();
        self.fault(Diagnostic::new(SCH_012, "an alias"), at, pointer);
        self.place(Node::new(Value::Null, at));
    }

    fn scalar(
        &mut self,
        text: Cow<'_, str>,
        style: ScalarStyle,
        anchor: usize,
        tag: Option<&Cow<'_, Tag>>,
        at: Position,
    ) {
        if self.skipping > 0 {
            return;
        }
        let property = properties(anchor, tag);
        let tagged = tag.is_some();
        if self.expects_key() {
            let pointer = format!("{}/{}", self.place_of_next(), token(&text));
            if let Some(fault) = property {
                self.fault(fault, at, pointer.clone());
            }
            // A key is a string (SCH-012): a quoted or block scalar, or a plain scalar that the
            // core schema reads as a string, where no tag gives it another type. A plain `<<` is
            // a merge key, rejected too.
            let plain = style == ScalarStyle::Plain && !tagged;
            if plain && !matches!(resolve(&text), Plain::String) {
                let fault =
                    Diagnostic::new(SCH_012, format!("a key that is not a string: `{text}`"));
                self.fault(fault, at, pointer.clone());
            } else if plain && text == "<<" {
                self.fault(
                    Diagnostic::new(SCH_012, "a merge key, `<<`"),
                    at,
                    pointer.clone(),
                );
            }
            let Some(Open {
                frame: Frame::Mapping { keys, key, .. },
                ..
            }) = self.stack.last_mut()
            else {
                return;
            };
            if !keys.insert(text.to_string()) {
                let fault = Diagnostic::new(SCH_012, format!("a duplicate key, `{text}`"));
                *key = Some(text.into_owned());
                self.fault(fault, at, pointer);
            } else {
                *key = Some(text.into_owned());
            }
            return;
        }
        let pointer = self.place_of_next();
        let value = if let Some(fault) = property {
            // The value an anchor or a tag stands on is not read further.
            self.fault(fault, at, pointer);
            Value::Null
        } else if style == ScalarStyle::Plain {
            plain_value(&text).unwrap_or_else(|fault| {
                self.fault(fault, at, pointer);
                Value::Null
            })
        } else {
            Value::String(text.into_owned())
        };
        self.place(Node::new(value, at));
    }

    /// Put a node in its place. The values of a document with a fault are never returned, so
    /// what is put in place of a faulty one only keeps the reading going.
    fn place(&mut self, node: Node) {
        match self.stack.last_mut() {
            None => self.root = Some(node),
            Some(Open { frame, .. }) => match frame {
                Frame::Sequence(items) => items.push(node),
                Frame::Mapping { members, key, .. } => match key.take() {
                    Some(key) => members.push((key, node)),
                    // A map or a list used as a key, which was rejected when it opened.
                    None => *key = Some(String::new()),
                },
            },
        }
    }

    fn end_document(&mut self, at: Position) {
        let root = self
            .root
            .take()
            .unwrap_or_else(|| Node::new(Value::Null, at));
        self.documents.push(root);
        self.stack.clear();
        self.skipping = 0;
    }
}

/// What the core schema of YAML 1.2.2 (section 10.3.2) reads a plain scalar as.
enum Plain {
    Null,
    Bool(bool),
    Decimal,
    HexOrOctal,
    InfinityOrNotANumber,
    String,
}

fn resolve(text: &str) -> Plain {
    match text {
        "" | "~" | "null" | "Null" | "NULL" => Plain::Null,
        "true" | "True" | "TRUE" => Plain::Bool(true),
        "false" | "False" | "FALSE" => Plain::Bool(false),
        ".nan" | ".NaN" | ".NAN" => Plain::InfinityOrNotANumber,
        _ if is_infinity(text) => Plain::InfinityOrNotANumber,
        _ if is_hex_or_octal(text) => Plain::HexOrOctal,
        _ if is_decimal(text) => Plain::Decimal,
        _ => Plain::String,
    }
}

/// `[-+]?\.(inf|Inf|INF)`
fn is_infinity(text: &str) -> bool {
    let unsigned = text.strip_prefix(['-', '+']).unwrap_or(text);
    matches!(unsigned, ".inf" | ".Inf" | ".INF")
}

/// `0o[0-7]+` and `0x[0-9a-fA-F]+`: integers of the core schema with no JSON form.
fn is_hex_or_octal(text: &str) -> bool {
    if let Some(digits) = text.strip_prefix("0o") {
        return !digits.is_empty() && digits.bytes().all(|b| (b'0'..=b'7').contains(&b));
    }
    if let Some(digits) = text.strip_prefix("0x") {
        return !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_hexdigit());
    }
    false
}

fn plain_value(text: &str) -> Result<Value, Diagnostic> {
    Ok(match resolve(text) {
        Plain::Null => Value::Null,
        Plain::Bool(b) => Value::Bool(b),
        Plain::String => Value::String(text.to_string()),
        Plain::HexOrOctal => {
            return Err(Diagnostic::new(
                SCH_017,
                format!("`{text}`, an integer in hexadecimal or octal, has no JSON form"),
            ));
        }
        Plain::InfinityOrNotANumber => {
            return Err(Diagnostic::new(
                SCH_017,
                format!("`{text}` has no JSON form"),
            ));
        }
        Plain::Decimal => read_number(text).map_err(|fault| number_fault(text, fault))?,
    })
}

/// A number rejected for its range, in YAML or in JSON.
pub(crate) fn number_fault(text: &str, fault: NumberFault) -> Diagnostic {
    match fault {
        NumberFault::WholeOutOfRange => Diagnostic::new(
            SCH_006,
            format!("`{text}` has no fractional part and is outside the range of I-JSON"),
        ),
        NumberFault::OutsideBinary64 => Diagnostic::new(
            SCH_021,
            format!("`{text}` is outside the range of IEEE 754 binary64"),
        ),
    }
}
