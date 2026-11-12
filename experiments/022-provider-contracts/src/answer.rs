use crate::{
    form::{Outcome, Reason},
    names, schema,
};
use serde_json::{Value, json};
#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    Outcome(Outcome),
    Reason(Reason),
}
pub fn read(operation: &str, wire: &Value, filters: &Value) -> Decision {
    use Decision::*;
    let unknown = Outcome(crate::form::Outcome::Unknown);
    if let Some(error) = wire.get("error") {
        if error["code"] == -32021 {
            let caps = &error["data"]["requiredCapabilities"];
            // IFC-008/047: extension revision mismatch or an undeclared capability.
            // An unqualified -32021 does not establish that nothing was performed.
            if caps.as_object().is_some_and(|m| {
                !m.is_empty()
                    && m.values().all(Value::is_object)
                    && m.get("extensions").is_none_or(|e| {
                        e.as_object().is_some_and(|e| {
                            e.values().all(Value::is_object) && (!e.is_empty() || m.len() > 1)
                        })
                    })
            }) {
                return Outcome(crate::form::Outcome::Failed);
            }
        }
        return unknown;
    }
    if wire.get("resultType").is_some_and(|v| v != "complete") {
        return unknown;
    }
    let meta = &wire["_meta"];
    let outcome = meta.get(names::key("outcome"));
    let reason = meta.get(names::key("reason"));
    let error = wire["isError"] == true;
    match (outcome, reason) {
        (Some(v), None) => {
            let Ok(o) = serde_json::from_value::<crate::form::Outcome>(v.clone()) else {
                return unknown;
            };
            if error != (o != crate::form::Outcome::Succeeded) {
                return unknown;
            }
            if o == crate::form::Outcome::Succeeded
                && !schema::output_valid(operation, &wire["structuredContent"])
            {
                return unknown;
            }
            Outcome(o)
        }
        (None, Some(v)) if error => {
            let Ok(r) = serde_json::from_value::<crate::form::Reason>(v.clone()) else {
                return unknown;
            };
            let restriction = &meta[names::key("restriction")];
            if restriction.get("requirement").is_none()
                || restriction.get("arguments").is_none()
                || !filters.as_array().is_some_and(|f| f.contains(restriction))
            {
                return unknown;
            }
            Reason(r)
        }
        _ => unknown,
    }
}
pub fn transport_unknown() -> Value {
    json!({"error":{"code":-32603}})
}
