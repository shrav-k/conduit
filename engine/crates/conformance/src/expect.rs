//! Expectations over step replies and state.
use super::*;
/// Maps are required-field projections; arrays are exact ordered sets/sequences. An empty
/// map asserts emptiness. Objects are compared in supplied allocation order, records in source order.
pub(super) fn projection(actual: &Value, expected: &Value, pointer: &str) -> Result<()> {
    match (actual, expected) {
        (Value::Object(a), Value::Object(e)) if !e.is_empty() => {
            for (key, value) in e {
                projection(
                    a.get(key)
                        .ok_or_else(|| format!("{pointer}/{key}: missing required field"))?,
                    value,
                    &format!("{pointer}/{key}"),
                )?;
            }
            Ok(())
        }
        (Value::Object(a), Value::Object(e)) if e.is_empty() && !a.is_empty() => Err(format!(
            "{pointer}: expected an empty map, found {} fields",
            a.len()
        )),
        (Value::Array(a), Value::Array(e)) if a.len() == e.len() => {
            for (i, (av, ev)) in a.iter().zip(e).enumerate() {
                projection(av, ev, &format!("{pointer}/{i}"))?;
            }
            Ok(())
        }
        (Value::Array(a), Value::Array(e)) => Err(format!(
            "{pointer}: expected {} items, found {}",
            e.len(),
            a.len()
        )),
        (a, e) if a == e => Ok(()),
        (a, e) => Err(format!("{pointer}: expected {e}, found {a}")),
    }
}

/// Add state-derived fields before expectations and captures.
pub(super) fn prepare(actual: &mut Value, next: &State) -> Result<()> {
    let mut object_ids = json!({});
    for o in next.objects.iter().filter(|o| o.get("spec").is_none()) {
        object_ids[o["kind"].as_str().ok_or("object kind absent")?]
            [o["name"].as_str().ok_or("object name absent")?] = o["id"].clone();
    }
    actual["objectIds"] = object_ids;
    let mut tombstone_ids = json!({});
    for stone in next.tombstones.values() {
        tombstone_ids[stone["object"]["kind"]
            .as_str()
            .ok_or("tombstone kind absent")?][stone["object"]["name"]
            .as_str()
            .ok_or("tombstone name absent")?] = stone["id"].clone();
    }
    actual["tombstoneIds"] = tombstone_ids;
    actual["tombstones"] = json!(next.tombstones.values().collect::<Vec<_>>());
    Ok(())
}

pub(super) fn check(
    raw_step: &Value,
    symbols: &BTreeMap<String, Value>,
    spec: &Specification,
    state: &State,
    next: &State,
    actual: &Value,
    index: usize,
) -> Result<()> {
    let expected = expand(&raw_step["expect"], symbols, spec)?;
    let mut fields = expected.as_object().ok_or("missing expectation")?.clone();
    if let Some(count) = fields.remove("object_count") {
        projection(&json!(next.objects.len()), &count, "/object_count")?;
    }
    let tombstones = next.tombstones.values().cloned().collect::<Vec<_>>();
    for (key, values) in [
        ("tombstone_facts", &tombstones),
        ("object_facts", &next.objects),
        ("invocation_facts", &next.invocations),
    ] {
        if let Some(facts) = fields.remove(key) {
            for fact in facts.as_array().ok_or("facts must be an array")? {
                let found = values
                    .iter()
                    .find(|v| {
                        (key == "tombstone_facts" && projection(v, fact, key).is_ok())
                            || fact.get("id").is_some_and(|id| v["id"] == *id)
                            || fact
                                .get("name")
                                .is_some_and(|n| v["name"] == *n && v.get("spec").is_none())
                    })
                    .ok_or_else(|| format!("{key}: missing {fact}"))?;
                projection(found, fact, key)?;
            }
        }
    }
    if let Some(groups) = fields.remove("inventory_facts") {
        for (key, facts) in groups.as_object().ok_or("inventory_facts must be a map")? {
            let entries = actual["reply"]["structuredContent"]["inventory"][key]
                .as_array()
                .ok_or("inventory array absent")?;
            for fact in facts.as_array().ok_or("inventory facts must be arrays")? {
                if !entries
                    .iter()
                    .any(|entry| projection(entry, fact, key).is_ok())
                {
                    return Err(format!("inventory {key}: missing {fact}"));
                }
            }
        }
    }
    for key in ["record_facts", "stored_record_facts"] {
        if let Some(facts) = fields.remove(key) {
            let mut cursor = 0;
            let records = if key == "stored_record_facts" {
                &next.records
            } else {
                actual["records"].as_array().ok_or("missing records")?
            };
            for fact in facts.as_array().ok_or("record facts must be an array")? {
                let found = records[cursor..]
                    .iter()
                    .position(|r| projection(r, fact, key).is_ok())
                    .ok_or_else(|| format!("missing ordered record {fact}"))?;
                cursor += found + 1;
            }
        }
    }
    if let Some(count) = fields.remove("tombstone_count") {
        projection(&json!(next.tombstones.len()), &count, "/tombstone_count")?;
    }
    if let Some(absent) = fields.remove("absent_tombstones") {
        for id in absent
            .as_array()
            .ok_or("absent_tombstones must be an array")?
        {
            if next
                .tombstones
                .contains_key(id.as_str().ok_or("tombstone ID must be a string")?)
            {
                return Err(format!("unexpected tombstone {id}"));
            }
        }
    }
    if let Some(absent) = fields.remove("absent_objects") {
        for id in absent.as_array().ok_or("absent_objects must be an array")? {
            if next.objects.iter().any(|o| o["id"] == *id) {
                return Err(format!("unexpected object {id}"));
            }
        }
    }
    if let Some(unchanged) = fields.remove("stateUnchanged") {
        if unchanged != true {
            return Err("stateUnchanged must be true when present".into());
        }
        if next.objects != state.objects
            || next.installed_packages != state.installed_packages
            || next.tombstones != state.tombstones
        {
            return Err(format!("step {}: objects changed", index + 1));
        }
    }
    if let Some(absent) = fields.remove("absent") {
        for pointer in absent.as_array().ok_or("invalid absent pointers")? {
            if let Some(text) = pointer["text"].as_str() {
                for path in pointer["under"].as_array().ok_or("missing absence scope")? {
                    let path = path.as_str().ok_or("invalid absence scope")?;
                    let part = actual.pointer(path).ok_or("absence scope missing")?;
                    if contains_text(part, text) {
                        return Err(format!("unexpected text under {path}"));
                    }
                }
                continue;
            }
            let pointer = pointer.as_str().ok_or("invalid pointer")?;
            if actual.pointer(pointer).is_some() {
                return Err(format!("unexpected field {pointer}"));
            }
        }
    }
    if let Some(rules) = fields.remove("required_promises") {
        let cited: BTreeSet<&str> = actual["decisions"]
            .as_array()
            .ok_or("no decisions")?
            .iter()
            .flat_map(|d| {
                d["promises"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
            })
            .collect();
        for rule in rules.as_array().ok_or("invalid required promises")? {
            if !cited.contains(rule.as_str().ok_or("invalid rule")?) {
                return Err(format!(
                    "uncited expected promise {rule}; reply: {}",
                    actual["reply"]
                ));
            }
        }
    }
    if let Some(assertions) = fields.remove("diagnostic_facts") {
        for assertion in assertions.as_array().ok_or("invalid diagnostic facts")? {
            let n = assertion["index"]
                .as_u64()
                .ok_or("diagnostic index missing")? as usize;
            let d = actual["diagnostics"]
                .get(n)
                .ok_or("expected diagnostic absent")?;
            for word in assertion["message_contains"]
                .as_array()
                .ok_or("missing message assertions")?
            {
                if !d["message"]["text"]
                    .as_str()
                    .is_some_and(|s| s.contains(word.as_str().unwrap_or("\0")))
                {
                    return Err(format!("diagnostic does not state {word}"));
                }
            }
            if assertion["keyword_location"] == true {
                let pointer = d["properties"]["keywordLocation"]
                    .as_str()
                    .ok_or("keywordLocation absent")?;
                if !pointer.starts_with('/') {
                    return Err("keywordLocation is not a pointer".into());
                }
                let keyword = pointer.rsplit('/').next().ok_or("empty keywordLocation")?;
                if assertion["keyword"] != keyword {
                    return Err(format!(
                        "expected keyword {}, got {keyword}",
                        assertion["keyword"]
                    ));
                }
                // The fixture states the portable contract/keyword, not a bundler path.
                let contract = assertion["contract"].as_str().ok_or("missing contract")?;
                let schema = spec
                    .contract(contract)
                    .ok_or("unknown diagnostic contract")?;
                let expected_keyword = schema
                    .pointer(
                        assertion["pointer"]
                            .as_str()
                            .ok_or("missing keyword pointer")?,
                    )
                    .ok_or("assertion points to no contract keyword")?;
                let root = assertion["root_schema"]
                    .as_str()
                    .ok_or("missing pinned root schema")?;
                if &keyword_value(spec, root, pointer)? != expected_keyword {
                    return Err(
                        "keywordLocation does not reach the asserted failing keyword".into(),
                    );
                }
            }
        }
    }
    let expected = Value::Object(fields);
    if raw_step["input"]["kind"] == "invoke"
        && raw_step["input"]["request"]["tool"]
            .as_str()
            .is_some_and(|t| t.ends_with(".list.v1"))
    {
        let mut comparable = actual.clone();
        list_sets(&mut comparable, &expected);
        projection(&comparable, &expected, "")
    } else {
        projection(actual, &expected, "")
    }
    .map_err(|e| format!("step {}: {e}; reply: {}", index + 1, actual["reply"]))?;
    Ok(())
}

fn contains_text(value: &Value, text: &str) -> bool {
    match value {
        Value::String(value) => value.contains(text),
        Value::Array(values) => values.iter().any(|value| contains_text(value, text)),
        Value::Object(values) => values
            .iter()
            .any(|(key, value)| key.contains(text) || contains_text(value, text)),
        _ => false,
    }
}

/// OPS-036 states no list ordering. Match projections by augmenting paths, so an early
/// broad expected item cannot consume the only match of a later specific item.
fn list_sets(actual: &mut Value, expected: &Value) {
    let Some(expected) = expected
        .pointer("/reply/structuredContent")
        .and_then(Value::as_object)
    else {
        return;
    };
    for (key, expected) in expected {
        let Some(expected) = expected.as_array() else {
            continue;
        };
        let Some(actual) = actual["reply"]["structuredContent"][key].as_array_mut() else {
            continue;
        };
        if expected.len() != actual.len() {
            continue;
        }
        let edges: Vec<Vec<usize>> = expected
            .iter()
            .map(|item| {
                actual
                    .iter()
                    .enumerate()
                    .filter_map(|(i, a)| projection(a, item, "").is_ok().then_some(i))
                    .collect()
            })
            .collect();
        fn augment(
            e: usize,
            edges: &[Vec<usize>],
            matched: &mut [Option<usize>],
            seen: &mut [bool],
        ) -> bool {
            for &a in &edges[e] {
                if seen[a] {
                    continue;
                }
                seen[a] = true;
                if matched[a].is_none_or(|old| augment(old, edges, matched, seen)) {
                    matched[a] = Some(e);
                    return true;
                }
            }
            false
        }
        let mut matched = vec![None; actual.len()];
        if (0..expected.len())
            .all(|e| augment(e, &edges, &mut matched, &mut vec![false; actual.len()]))
        {
            let mut ordered = vec![Value::Null; actual.len()];
            for (a, e) in matched.into_iter().enumerate() {
                ordered[e.unwrap()] = actual[a].clone();
            }
            *actual = ordered;
        }
    }
}

#[cfg(test)]
mod list_set_tests {
    use super::*;
    #[test]
    fn reversed_list_items_pass_and_missing_item_fails() {
        let expected =
            json!({"reply":{"structuredContent":{"objects":[{"name":"b"},{"name":"a"}]}}});
        let mut actual = json!({"reply":{"structuredContent":{"objects":[{"name":"a","id":"1"},{"name":"b","id":"2"}]}}});
        list_sets(&mut actual, &expected);
        projection(&actual, &expected, "").unwrap();
        actual["reply"]["structuredContent"]["objects"]
            .as_array_mut()
            .unwrap()
            .pop();
        list_sets(&mut actual, &expected);
        assert!(
            projection(&actual, &expected, "")
                .unwrap_err()
                .contains("expected 2 items, found 1")
        );
    }
    #[test]
    fn augmenting_paths_preserve_specific_matches() {
        let expected = json!({"reply":{"structuredContent":{"objects":[{"kind":"Pool"},{"kind":"Pool","name":"only"}]}}});
        let mut actual = json!({"reply":{"structuredContent":{"objects":[{"kind":"Pool","name":"only"},{"kind":"Pool","name":"other"}]}}});
        list_sets(&mut actual, &expected);
        projection(&actual, &expected, "").unwrap();
        actual["reply"]["structuredContent"]["objects"][1]["name"] = json!("wrong");
        list_sets(&mut actual, &expected);
        assert!(projection(&actual, &expected, "").is_err());
    }
}
