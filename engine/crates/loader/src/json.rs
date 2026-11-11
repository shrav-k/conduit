//! Reading JSON strictly: JSON as RFC 8259 writes it, with the value model of I-JSON (RFC 7493)
//! and the same narrowings as YAML, so that a JSON document means the same as the equivalent
//! YAML document (SCH-013).
//!
//! `serde_json` checks the syntax. The values are built here from the raw text of each value,
//! so that two members with one key are found (SCH-012), which `serde_json` would otherwise
//! keep one of silently, and so that each number is read from the digits that write it
//! (SCH-005, SCH-006, SCH-020, SCH-021), not from a binary64 value `serde_json` made of it.
//! JSON has no infinity, not-a-number, hexadecimal, anchor, alias, tag or merge key, so SCH-017
//! and the rest of SCH-012 cannot arise. One leading byte order mark is accepted and ignored,
//! as the schema profile requires and RFC 8259 (section 8.1) permits a parser to do.
//! `serde_json` rejects as not JSON (SUB-005) an escape of a
//! surrogate that is not one of a pair, `\ud800`, which is not text; a pair, `\ud83d\ude00`, is
//! one character.
//!
//! A file whose value is an array holds the documents that are its items, and any other file
//! holds its value as one document (SCH-018), which the schemas then judge, as they judge a
//! YAML document: a value that is not a map is rejected under SUB-005.
//!
//! As in YAML, a text that is not JSON is rejected for that alone, and any other fault is in a
//! value: it is reported with its place as a JSON Pointer, and reading goes on, so that every
//! fault of every document is reported (SUB-001), with the same rule and the same place as in
//! the YAML form of the document (SCH-013).

use std::collections::HashSet;
use std::fmt;

use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde_json::value::RawValue;

use crate::diagnostic::{Diagnostic, Diagnostics, Locator, char_boundary, position_of};
use crate::number::read_number;
use crate::rules::{SCH_012, SUB_005};
use crate::value::{Node, Value};
use crate::yaml::number_fault;

/// The content of a JSON file.
pub(crate) struct Content {
    /// The documents, in the order written.
    pub documents: Vec<Node>,
    /// Whether the file holds an array, whose items are the documents (SCH-018).
    pub array: bool,
}

/// Read the documents of a JSON text.
pub(crate) fn read(text: &str) -> Result<Content, Diagnostics> {
    let source = text.strip_prefix('\u{feff}').unwrap_or(text);
    let root: &RawValue = serde_json::from_str(source).map_err(|error| {
        let mut diagnostic = syntax(source, &error);
        if source.len() != text.len()
            && let Some(position) = &mut diagnostic.position
            && position.line == 1
        {
            position.column = position.column.saturating_add(1);
        }
        diagnostic
    })?;
    let reader = Reader {
        text,
        locator: Locator::new(text),
        faults: std::cell::RefCell::new(Vec::new()),
    };
    let content = if root.get().starts_with('[') {
        let items: Vec<&RawValue> =
            serde_json::from_str(root.get()).map_err(|error| syntax(text, &error))?;
        let mut documents = Vec::with_capacity(items.len());
        for (index, item) in items.into_iter().enumerate() {
            documents.push(reader.node(item, 0, "", index + 1)?);
        }
        Content {
            documents,
            array: true,
        }
    } else {
        Content {
            documents: vec![reader.node(root, 0, "", 1)?],
            array: false,
        }
    };
    let faults = reader.faults.into_inner();
    if faults.is_empty() {
        Ok(content)
    } else {
        Err(Diagnostics(faults))
    }
}

/// Text that is not JSON. `serde_json` gives the line and the column in bytes; the column is
/// given here in characters, as every [`crate::Position`] is.
fn syntax(text: &str, error: &serde_json::Error) -> Diagnostic {
    let message = error.to_string();
    let message = message.split(" at line ").next().unwrap_or(&message);
    let line_start = if error.line() > 1 {
        text.match_indices('\n')
            .nth(error.line() - 2)
            .map_or(text.len(), |(at, _)| at + 1)
    } else {
        0
    };
    let offset = char_boundary(text, line_start + error.column().saturating_sub(1));
    Diagnostic::new(SUB_005, format!("not JSON: {message}")).at(position_of(text, offset))
}

struct Reader<'t> {
    text: &'t str,
    locator: Locator<'t>,
    /// The faults found in values, in the order of the text.
    faults: std::cell::RefCell<Vec<Diagnostic>>,
}

impl<'t> Reader<'t> {
    /// The byte offset of a raw value that was borrowed from the text.
    fn offset(&self, raw: &RawValue) -> usize {
        (raw.get().as_ptr() as usize).saturating_sub(self.text.as_ptr() as usize)
    }

    fn fault(&self, diagnostic: Diagnostic) {
        self.faults.borrow_mut().push(diagnostic);
    }

    /// The node of a raw value, at this depth and place, in this document. A fault in a value
    /// is recorded and the value read as null, so that reading goes on; a fault in the syntax,
    /// which `serde_json` has already ruled out, stops it.
    fn node(
        &self,
        raw: &'t RawValue,
        depth: usize,
        pointer: &str,
        document: usize,
    ) -> Result<Node, Diagnostic> {
        let at = self.locator.position(self.offset(raw));
        let fault = |d: Diagnostic| {
            self.fault(d.at(at).at_pointer(pointer).in_document(document));
            Value::Null
        };
        let source = raw.get();
        let value = match source.as_bytes().first() {
            Some(b'{' | b'[') if depth >= crate::MAX_DEPTH => fault(crate::too_deep()),
            Some(b'{') => {
                let Members(members) = serde_json::from_str(source)
                    .map_err(|error| syntax(self.text, &error).at(at))?;
                let mut keys = HashSet::new();
                let mut out = Vec::with_capacity(members.len());
                for (key, raw) in members {
                    let place = format!("{pointer}/{}", key.replace('~', "~0").replace('/', "~1"));
                    if !keys.insert(key.clone()) {
                        let at = self.locator.position(self.offset(raw));
                        self.fault(
                            Diagnostic::new(SCH_012, format!("a duplicate key, `{key}`"))
                                .at(at)
                                .at_pointer(place.clone())
                                .in_document(document),
                        );
                    }
                    out.push((key, self.node(raw, depth + 1, &place, document)?));
                }
                Value::Object(out)
            }
            Some(b'[') => {
                let items: Vec<&RawValue> = serde_json::from_str(source)
                    .map_err(|error| syntax(self.text, &error).at(at))?;
                let mut out = Vec::with_capacity(items.len());
                for (index, raw) in items.into_iter().enumerate() {
                    out.push(self.node(raw, depth + 1, &format!("{pointer}/{index}"), document)?);
                }
                Value::Array(out)
            }
            Some(b'"') => Value::String(
                serde_json::from_str(source).map_err(|error| syntax(self.text, &error).at(at))?,
            ),
            Some(b't') => Value::Bool(true),
            Some(b'f') => Value::Bool(false),
            Some(b'n') => Value::Null,
            // A number: `serde_json` has checked that it has JSON's form.
            _ => read_number(source).unwrap_or_else(|f| fault(number_fault(source, f))),
        };
        Ok(Node::new(value, at))
    }
}

/// The members of a JSON object as written, duplicates kept, each value still raw.
struct Members<'t>(Vec<(String, &'t RawValue)>);

impl<'de: 't, 't> Deserialize<'de> for Members<'t> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MembersVisitor<'t>(std::marker::PhantomData<&'t ()>);

        impl<'de: 't, 't> Visitor<'de> for MembersVisitor<'t> {
            type Value = Members<'t>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a JSON object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut members = Vec::new();
                while let Some(key) = map.next_key::<String>()? {
                    let value: &'de RawValue = map.next_value()?;
                    members.push((key, value));
                }
                Ok(Members(members))
            }
        }

        deserializer.deserialize_map(MembersVisitor(std::marker::PhantomData))
    }
}
