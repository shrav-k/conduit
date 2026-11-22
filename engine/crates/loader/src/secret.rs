use serde_json::Value;

/// SEC-003 wording for diagnostics that would otherwise describe a protected value.
pub const WITHHELD_VALUE: &str = "the value at that place is withheld under SEC-003";

/// Schema error messages can quote an entire instance or a member of it.
fn quotes_value(message: &str, value: &Value) -> bool {
    message.contains(&value.to_string())
        || match value {
            Value::Array(items) => items.iter().any(|item| quotes_value(message, item)),
            Value::Object(members) => members.iter().any(|(key, item)| {
                message.contains(&format!("{key:?}")) || quotes_value(message, item)
            }),
            _ => false,
        }
}

/// Format a validation error after deciding which values it is allowed to include.
pub fn validation_message(error: &jsonschema::ValidationError<'_>, protected: &[&Value]) -> String {
    if protected.is_empty() {
        return error.to_string();
    }
    let message = error.masked_with(WITHHELD_VALUE).to_string();
    // Some keyword messages still contain instance keys or schema values.
    if protected.iter().any(|value| quotes_value(&message, value)) {
        WITHHELD_VALUE.into()
    } else {
        message
    }
}
