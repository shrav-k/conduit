//! Controls are the operations that declare which settings they change (CTR-006).
use serde_json::Value;

pub(crate) fn is_control(spec: &Value) -> bool {
    spec.get("changes").is_some()
}
