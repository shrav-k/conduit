use crate::Result;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

const ROOT_FIX: &str = "supply an object with only cases, a list";
const ENTRY_FIX: &str = "give each entry exactly id and reason as nonempty strings";

fn repair(label: &str, problem: &str) -> String {
    format!(
        "conformance/expected-failures.yaml: {label}{problem}. See conformance/README.md, \"Expected failures\"."
    )
}

/// Names a value's JSON type for a diagnostic.
fn kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::Number(_) => "a number",
        Value::String(_) => "a string",
        Value::Array(_) => "a list",
        Value::Object(_) => "an object",
    }
}

/// Says which members an object lacks and which it has besides `allowed`, or `None` when its
/// members are exactly `allowed`.
fn member_fault(object: &Map<String, Value>, allowed: &[&str]) -> Option<String> {
    let names = |names: Vec<&str>| match names.as_slice() {
        [name] => format!("member {name}"),
        _ => format!("members {}", names.join(" and ")),
    };
    let missing: Vec<&str> = allowed
        .iter()
        .copied()
        .filter(|name| !object.contains_key(*name))
        .collect();
    let unknown: Vec<&str> = object
        .keys()
        .map(String::as_str)
        .filter(|key| !allowed.contains(key))
        .collect();
    let mut faults = vec![];
    if !missing.is_empty() {
        faults.push(format!("lacks {}", names(missing)));
    }
    if !unknown.is_empty() {
        faults.push(format!("has unknown {}", names(unknown)));
    }
    (!faults.is_empty()).then(|| faults.join(" and "))
}

pub(super) fn validate(value: &Value, ids: &BTreeSet<String>) -> Result<BTreeMap<String, String>> {
    let root = value.as_object().ok_or_else(|| {
        repair(
            "",
            &format!("the root is {}, not an object; {ROOT_FIX}", kind(value)),
        )
    })?;
    if let Some(fault) = member_fault(root, &["cases"]) {
        return Err(repair("", &format!("the root {fault}; {ROOT_FIX}")));
    }
    let cases = root["cases"].as_array().ok_or_else(|| {
        repair(
            "",
            &format!(
                "the root's cases is {}, not a list; {ROOT_FIX}",
                kind(&root["cases"])
            ),
        )
    })?;
    let mut expected = BTreeMap::new();
    let mut positions: BTreeMap<&str, usize> = BTreeMap::new();
    let mut previous: Option<&str> = None;
    for (index, entry) in cases.iter().enumerate() {
        let position = index + 1;
        let label = match entry.get("id").and_then(Value::as_str) {
            Some(id) if !id.is_empty() => format!("{id}: "),
            _ => format!("entry {position}: "),
        };
        let fail = |problem: String| repair(&label, &problem);
        let object = entry
            .as_object()
            .ok_or_else(|| fail(format!("is {}, not an object; {ENTRY_FIX}", kind(entry))))?;
        if let Some(fault) = member_fault(object, &["id", "reason"]) {
            return Err(fail(format!("{fault}; {ENTRY_FIX}")));
        }
        let nonempty = |name: &str| {
            let fault = match &object[name] {
                Value::String(text) if !text.is_empty() => return Ok(text.as_str()),
                Value::String(_) => format!("{name} is empty"),
                other => format!("{name} is {}, not a string", kind(other)),
            };
            Err(fail(format!("{fault}; set {name} to a nonempty string")))
        };
        let id = nonempty("id")?;
        let reason = nonempty("reason")?;
        if let Some(first) = positions.get(id) {
            return Err(fail(format!(
                "is listed again as entry {position}, after entry {first}; keep only one entry for this ID"
            )));
        }
        if let Some(previous) = previous.filter(|previous| previous.as_bytes() > id.as_bytes()) {
            return Err(fail(format!(
                "follows {previous}, out of byte order; sort entries by id"
            )));
        }
        if !ids.contains(id) {
            return Err(fail(
                "no case has this ID; remove the entry or restore the case".into(),
            ));
        }
        expected.insert(id.to_owned(), reason.to_owned());
        positions.insert(id, position);
        previous = Some(id);
    }
    Ok(expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn ids() -> BTreeSet<String> {
        BTreeSet::from(["a".into(), "b".into()])
    }

    fn message(problem: &str) -> String {
        format!(
            "conformance/expected-failures.yaml: {problem}. See conformance/README.md, \"Expected failures\"."
        )
    }

    #[test]
    fn an_empty_list_gives_an_empty_map() {
        assert_eq!(validate(&json!({"cases": []}), &ids()), Ok(BTreeMap::new()));
    }

    #[test]
    fn two_sorted_entries_give_both_reasons() {
        let value = json!({"cases": [
            {"id": "a", "reason": "missing a; order 1"},
            {"id": "b", "reason": "missing b; order 2"}
        ]});
        assert_eq!(
            validate(&value, &ids()),
            Ok(BTreeMap::from([
                ("a".into(), "missing a; order 1".into()),
                ("b".into(), "missing b; order 2".into())
            ]))
        );
    }

    #[test]
    fn a_malformed_root_says_what_is_wrong_and_the_repair() {
        for (value, fault) in [
            (json!(null), "the root is null, not an object"),
            (json!([]), "the root is a list, not an object"),
            (json!("cases"), "the root is a string, not an object"),
            (json!({}), "the root lacks member cases"),
            (
                json!({"other": []}),
                "the root lacks member cases and has unknown member other",
            ),
            (
                json!({"cases": [], "extra": true}),
                "the root has unknown member extra",
            ),
            (
                json!({"cases": null}),
                "the root's cases is null, not a list",
            ),
            (
                json!({"cases": {}}),
                "the root's cases is an object, not a list",
            ),
        ] {
            assert_eq!(
                validate(&value, &ids()),
                Err(message(&format!("{fault}; {ROOT_FIX}"))),
                "{value}"
            );
        }
    }

    #[test]
    fn a_malformed_entry_names_its_id_or_position_and_says_what_is_wrong() {
        let a = json!({"id": "a", "reason": "order 1"});
        for (cases, fault, fix) in [
            (json!([null]), "entry 1: is null, not an object", ENTRY_FIX),
            (
                json!([a, []]),
                "entry 2: is a list, not an object",
                ENTRY_FIX,
            ),
            (
                json!([{}]),
                "entry 1: lacks members id and reason",
                ENTRY_FIX,
            ),
            (json!([{"id": "a"}]), "a: lacks member reason", ENTRY_FIX),
            (
                json!([a, {"reason": "order 2"}]),
                "entry 2: lacks member id",
                ENTRY_FIX,
            ),
            (
                json!([{"id": "a", "other": "order 1"}]),
                "a: lacks member reason and has unknown member other",
                ENTRY_FIX,
            ),
            (
                json!([{"id": "a", "reason": "order 1", "extra": true}]),
                "a: has unknown member extra",
                ENTRY_FIX,
            ),
            (
                json!([{"id": null, "reason": "order 1"}]),
                "entry 1: id is null, not a string",
                "set id to a nonempty string",
            ),
            (
                json!([a, {"id": 1, "reason": "order 2"}]),
                "entry 2: id is a number, not a string",
                "set id to a nonempty string",
            ),
            (
                json!([{"id": "", "reason": "order 1"}]),
                "entry 1: id is empty",
                "set id to a nonempty string",
            ),
            (
                json!([{"id": "a", "reason": null}]),
                "a: reason is null, not a string",
                "set reason to a nonempty string",
            ),
            (
                json!([{"id": "a", "reason": 1}]),
                "a: reason is a number, not a string",
                "set reason to a nonempty string",
            ),
            (
                json!([{"id": "a", "reason": ""}]),
                "a: reason is empty",
                "set reason to a nonempty string",
            ),
        ] {
            assert_eq!(
                validate(&json!({ "cases": cases }), &ids()),
                Err(message(&format!("{fault}; {fix}"))),
                "{cases}"
            );
        }
    }

    #[test]
    fn invalid_ids_name_the_id_and_the_repair() {
        for (cases, problem) in [
            (
                json!([
                    {"id": "a", "reason": "order 1"},
                    {"id": "a", "reason": "order 2"}
                ]),
                "a: is listed again as entry 2, after entry 1; keep only one entry for this ID",
            ),
            (
                json!([
                    {"id": "a", "reason": "order 1"},
                    {"id": "b", "reason": "order 2"},
                    {"id": "a", "reason": "order 3"}
                ]),
                "a: is listed again as entry 3, after entry 1; keep only one entry for this ID",
            ),
            (
                json!([
                    {"id": "a", "reason": "order 1"},
                    {"id": "b", "reason": "order 2"},
                    {"id": "b", "reason": "order 3"}
                ]),
                "b: is listed again as entry 3, after entry 2; keep only one entry for this ID",
            ),
            (
                json!([
                    {"id": "b", "reason": "order 1"},
                    {"id": "a", "reason": "order 2"}
                ]),
                "a: follows b, out of byte order; sort entries by id",
            ),
            (
                json!([{"id": "missing", "reason": "order 1"}]),
                "missing: no case has this ID; remove the entry or restore the case",
            ),
        ] {
            assert_eq!(
                validate(&json!({ "cases": cases }), &ids()),
                Err(message(problem)),
                "{cases}"
            );
        }
    }
}
