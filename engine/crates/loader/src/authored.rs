//! The schemas an author writes inside a document, and the one rule the loader applies to them:
//! each `pattern`, and each pattern that keys `patternProperties`, is a regular expression of
//! ECMA-262 in Unicode mode (SCH-016).
//!
//! SCH-016 says what a pattern is, and that a schema whose pattern is not one makes the
//! document that holds it rejected, as SCH-002 rejects a keyword outside the profile; the
//! coordinator settled that on finding 1-03 of the first build's log of increment 1. The loader
//! rejects the document when it is read. The rule is applied only to a document that the
//! schemas accept, so that the places below hold what they should.

use crate::diagnostic::Diagnostic;
use crate::formats::ecma_regex;
use crate::read::Document;
use crate::rules::SCH_016;
use crate::schemas::{LIST_OF_SCHEMAS, MAP_OF_SCHEMAS, ONE_SCHEMA};
use crate::value::{Node, Value};

/// Where a document of each role holds a schema, as `SCHEMA_PLACES` in
/// `tools/check_schemas.py` lists them. `*` is every member of a map.
const SCHEMA_PLACES: &[(&str, &[&str])] = &[
    ("Schema", &["spec"]),
    ("Type", &["spec", "schema"]),
    ("Operation", &["spec", "inputSchema"]),
    ("Operation", &["spec", "outputSchema"]),
    ("Requirement", &["spec", "inputs", "*", "schema"]),
    ("template", &["spec", "inputs", "*", "schema"]),
    (
        "template",
        &["spec", "controls", "*", "inputs", "*", "schema"],
    ),
];

const FAMILIES: [&str; 6] = ["Interface", "Agent", "Resource", "Pool", "Fleet", "Conduit"];

/// What SCH-016 finds in the schemas of one document.
pub(crate) fn check_patterns(document: &Document) -> Vec<Diagnostic> {
    let root = &document.root;
    let kind = root.get("kind").and_then(Node::as_str).unwrap_or("");
    let role = if FAMILIES.contains(&kind) {
        if root.get("from").is_some() {
            "instance"
        } else {
            "template"
        }
    } else {
        kind
    };
    let mut faults = Vec::new();
    for (place_role, path) in SCHEMA_PLACES {
        if *place_role != role {
            continue;
        }
        let mut found = Vec::new();
        places(root, path, String::new(), &mut found);
        for (pointer, schema) in found {
            walk(schema, &pointer, &mut |node, at| {
                let mut patterns = Vec::new();
                if let Some(pattern) = node.get("pattern")
                    && let Some(text) = pattern.as_str()
                {
                    patterns.push((format!("{at}/pattern"), text.to_string(), pattern));
                }
                if let Some(Node {
                    value: Value::Object(members),
                    ..
                }) = node.get("patternProperties")
                {
                    for (key, member) in members {
                        patterns.push((
                            format!("{at}/patternProperties/{}", escape(key)),
                            key.clone(),
                            member,
                        ));
                    }
                }
                for (pointer, pattern, held) in patterns {
                    if let Err(why) = ecma_regex(&pattern) {
                        faults.push(
                            Diagnostic::new(
                                SCH_016,
                                format!("`{pattern}` is not an interoperable pattern of SCH-016 in Unicode mode: {why}"),
                            )
                            .in_file(document.file.clone())
                            .in_document(document.index)
                            .at_pointer(pointer)
                            .at(held.position),
                        );
                    }
                }
            });
        }
    }
    faults
}

fn escape(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

/// The nodes that a path reaches, with the pointer to each.
fn places<'n>(node: &'n Node, path: &[&str], at: String, out: &mut Vec<(String, &'n Node)>) {
    let Some((first, rest)) = path.split_first() else {
        out.push((at, node));
        return;
    };
    let Value::Object(members) = &node.value else {
        return;
    };
    for (key, member) in members {
        if *first == "*" || key == first {
            places(member, rest, format!("{at}/{}", escape(key)), out);
        }
    }
}

/// Visit every schema object inside a schema, the schema itself first, knowing which members of
/// a schema are schemas, so that a value under `const`, `enum`, `default` or `examples` is not
/// taken for one.
fn walk(schema: &Node, at: &str, visit: &mut dyn FnMut(&Node, &str)) {
    let Value::Object(_) = &schema.value else {
        return;
    };
    visit(schema, at);
    for keyword in MAP_OF_SCHEMAS {
        if let Some(Node {
            value: Value::Object(members),
            ..
        }) = schema.get(keyword)
        {
            for (name, sub) in members {
                walk(sub, &format!("{at}/{keyword}/{}", escape(name)), visit);
            }
        }
    }
    for keyword in ONE_SCHEMA {
        if let Some(sub) = schema.get(keyword) {
            walk(sub, &format!("{at}/{keyword}"), visit);
        }
    }
    for keyword in LIST_OF_SCHEMAS {
        if let Some(Node {
            value: Value::Array(items),
            ..
        }) = schema.get(keyword)
        {
            for (index, sub) in items.iter().enumerate() {
                walk(sub, &format!("{at}/{keyword}/{index}"), visit);
            }
        }
    }
}
