use crate::names;
use serde_json::{Value, json};

pub fn copied() -> Value {
    serde_json::from_str(include_str!("output-schemas.json")).unwrap()
}
fn localize(v: &mut Value) {
    match v {
        Value::Object(m) => {
            if let Some(Value::String(r)) = m.get_mut("$ref")
                && let Some(n) = r.strip_prefix(&format!("{}#", names::CORE))
            {
                *r = format!("#/$defs/{n}");
            }
            for v in m.values_mut() {
                localize(v);
            }
        }
        Value::Array(a) => {
            for v in a {
                localize(v);
            }
        }
        _ => (),
    }
}
pub fn output_valid(operation: &str, output: &Value) -> bool {
    let bundle = copied();
    let Some(schema) = bundle["outputs"].get(operation) else {
        return false;
    };
    if schema.is_null() {
        return true;
    }
    let mut schema = schema.clone();
    schema["$defs"] = bundle["definitions"].clone();
    localize(&mut schema);
    jsonschema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .should_validate_formats(true)
        .build(&schema)
        .is_ok_and(|v| v.is_valid(output))
}
pub fn qualified(operation: &str) -> Value {
    json!(names::operation(operation))
}
