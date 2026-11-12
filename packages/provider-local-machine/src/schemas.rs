//! Standalone schema copies, checked against spec when it is available.
use serde_json::Value;
use std::sync::LazyLock;
pub static COPIES: LazyLock<Value> =
    LazyLock::new(|| serde_json::from_str(include_str!("schemas.json")).unwrap());
pub const OPERATIONS: [&str; 2] = ["list-machines.v1", "place.v1"];
pub fn schema(operation: &str, kind: &str) -> Option<Value> {
    let mut schema = COPIES["operations"][operation].get(kind)?.clone();
    schema["$defs"] = COPIES["definitions"].clone();
    Some(schema)
}
pub fn valid(operation: &str, kind: &str, value: &Value) -> bool {
    schema(operation, kind).is_none_or(|schema| {
        jsonschema::options()
            .with_draft(jsonschema::Draft::Draft202012)
            .should_validate_formats(true)
            .build(&schema)
            .expect("copied schema")
            .is_valid(value)
    })
}
