//! The values a document holds, once read: the JSON value model, which I-JSON narrows
//! (RFC 7493), with the place in the file where each value was written.

use crate::diagnostic::Position;

/// The largest whole number a document may hold, under SCH-006: 2^53 - 1, as I-JSON
/// recommends.
pub const LARGEST_WHOLE: u64 = 9_007_199_254_740_991;

/// A value of a document.
///
/// A number with no fractional part is held as [`Value::Integer`] however it is written, so
/// `15`, `15.0` and `1.5e1` are the same value (SCH-005, SCH-007). A number with one is held as
/// the IEEE 754 binary64 value nearest to it (SCH-020), and as [`Value::Integer`] where that
/// value has no fractional part, which SCH-006 holds to its range: `0.99999999999999999999` is
/// the integer 1. So a [`Value::Float`] never has a fractional part of zero.
#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Node>),
    /// The members of a map, in the order they were written. No two have one key (SCH-012).
    Object(Vec<(String, Node)>),
}

/// A value and where it was written.
#[derive(Clone, Debug)]
pub struct Node {
    pub value: Value,
    pub position: Position,
}

impl Node {
    pub fn new(value: Value, position: Position) -> Self {
        Node { value, position }
    }

    /// The member of a map with this key.
    pub fn get(&self, key: &str) -> Option<&Node> {
        match &self.value {
            Value::Object(members) => members.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match &self.value {
            Value::String(text) => Some(text),
            _ => None,
        }
    }

    /// The value in the form a JSON Schema validator takes.
    pub fn to_json(&self) -> serde_json::Value {
        match &self.value {
            Value::Null => serde_json::Value::Null,
            Value::Bool(b) => serde_json::Value::Bool(*b),
            Value::Integer(i) => serde_json::Value::from(*i),
            // A Float is always finite: the reader rejects anything else (SCH-017, SCH-021).
            Value::Float(f) => serde_json::Number::from_f64(*f)
                .map_or(serde_json::Value::Null, serde_json::Value::Number),
            Value::String(s) => serde_json::Value::String(s.clone()),
            Value::Array(items) => {
                serde_json::Value::Array(items.iter().map(Node::to_json).collect())
            }
            Value::Object(members) => serde_json::Value::Object(
                members
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_json()))
                    .collect(),
            ),
        }
    }

    /// The deepest node that a JSON Pointer reaches, following it as far as it goes. Used to
    /// say where in the file a place is.
    pub fn locate(&self, pointer: &str) -> &Node {
        let mut node = self;
        for token in pointer.split('/').skip(1) {
            let token = token.replace("~1", "/").replace("~0", "~");
            let next = match &node.value {
                Value::Object(members) => members.iter().find(|(k, _)| *k == token).map(|(_, v)| v),
                Value::Array(items) => token.parse::<usize>().ok().and_then(|i| items.get(i)),
                _ => None,
            };
            match next {
                Some(found) => node = found,
                None => break,
            }
        }
        node
    }
}

/// Equality as SCH-007 defines it: numbers by value, strings character by character, lists in
/// order, and maps by their keys and values regardless of key order. Where a value was written
/// is not part of it, so the YAML and the JSON forms of one document are equal (SCH-013).
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            // An Integer is within 2^53, so it converts to binary64 exactly.
            (Value::Integer(a), Value::Float(b)) | (Value::Float(b), Value::Integer(a)) => {
                *a as f64 == *b
            }
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .all(|(key, value)| b.iter().any(|(k, v)| k == key && v == value))
            }
            _ => false,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
