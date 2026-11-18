//! Pending actions computed from invocation history, without storing an action.
use crate::State;
use chrono::{DateTime, SecondsFormat, TimeDelta};
use serde_json::{Value, json};

pub(crate) fn setup_action(state: &State, object: &Value, setup_operation: &str) -> Option<Value> {
    if object["state"] == "retired" {
        return None;
    }
    let object_id = object["id"].as_str()?;
    // Missing dispatch numbers rank below numbered dispatches. Ties use the last match.
    let setup = state
        .invocations
        .iter()
        .filter(|v| v["operation"] == setup_operation && v["object"] == object_id)
        .max_by_key(|v| v["dispatched"].as_u64())?;
    if setup["state"] == "in-progress" {
        return None;
    }
    let id = setup["id"].as_str()?;
    let (time, outcome) = if let Some(ended) = setup.get("ended") {
        (&ended["time"], &ended["outcome"])
    } else {
        let ended = state.records.iter().rev().find(|r| {
            r["type"] == "invocation-ended"
                && r["data"]["invocation"] == id
                && r["data"]["reconciled"] != true
        })?;
        (&ended["time"], &ended["data"]["outcome"])
    };
    if !matches!(outcome.as_str(), Some("failed" | "unknown")) {
        return None;
    }
    let expiry = DateTime::parse_from_rfc3339(time.as_str()?)
        .ok()?
        .checked_add_signed(TimeDelta::days(7))?
        .to_utc()
        .to_rfc3339_opts(SecondsFormat::AutoSi, true);

    let mut evidence = vec![
        fact(
            json!(id),
            "records",
            "the set-up.v1 invocation that did not succeed",
        ),
        fact(
            outcome.clone(),
            "records",
            "the outcome that setup ended with",
        ),
    ];
    let cause = setup.get("setup_cause").unwrap_or(&setup["cause"]);
    if cause.as_array().is_some_and(|v| !v.is_empty()) {
        evidence.push(fact(
            cause.clone(),
            "records",
            "the diagnostics that setup ended with",
        ));
    }
    let parts = setup["setup"]["parts"]
        .as_array()
        .map_or(&[][..], Vec::as_slice);
    let unsuccessful: Vec<_> = parts
        .iter()
        .filter(|part| part["outcome"] != "succeeded")
        .map(|part| {
            let mut detail = json!({"name":part["name"]});
            for field in ["outcome", "reason", "diagnostics"] {
                if let Some(value) = part.get(field) {
                    detail[field] = value.clone();
                }
            }
            detail
        })
        .collect();
    if !unsuccessful.is_empty() {
        evidence.push(fact(
            json!(unsuccessful),
            "status.setup",
            "the parts of that setup that did not succeed",
        ));
    }
    let mut unknown = Vec::new();
    for id in std::iter::once(id).chain(parts.iter().filter_map(|p| p["invocation"].as_str())) {
        if !unknown.contains(&id)
            && state
                .invocations
                .iter()
                .any(|v| v["id"] == id && v["state"] == "unknown")
        {
            unknown.push(id);
        }
    }
    if !unknown.is_empty() {
        evidence.push(fact(
            json!(unknown),
            "status.unreconciled",
            "the invocations of that setup whose outcome is still unknown",
        ));
    }
    Some(json!({
        "id": id,
        "kind": "setup",
        "subject": [{"kind": object["kind"], "name": object["name"], "id": object_id}],
        "evidence": evidence,
        "expiry": expiry
    }))
}

pub(crate) fn is_raised_at(action: &Value, time: &str) -> bool {
    let Some(expiry) = action["expiry"].as_str() else {
        return false;
    };
    matches!(
        (DateTime::parse_from_rfc3339(time), DateTime::parse_from_rfc3339(expiry)),
        (Ok(time), Ok(expiry)) if time < expiry
    )
}

fn fact(value: Value, place: &str, proves: &str) -> Value {
    json!({"kind":"fact", "value":value, "where":place, "proves":proves})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_action_follows_the_latest_setup() {
        let time = "2026-11-09T12:30:00Z";
        let operation = "core#set-up.v1";
        let object = json!({"id":"o1", "kind":"Resource", "name":"shared", "state":"registered"});
        let diagnostic = json!({
            "ruleId":"OPS-022", "level":"error", "message":{"text":"setup ended failed"}
        });
        let failed = json!({
            "id":"s1", "operation":operation, "object":"o1", "state":"failed",
            "ended":{"time":time, "outcome":"failed"}, "cause":[diagnostic],
            "setup":{"parts":[{"name":"run", "outcome":"failed", "effect":false,
                "optional":false, "invocation":"r1"}]}
        });
        let mut state = State {
            invocations: vec![failed.clone(), json!({"id":"r1", "state":"failed"})],
            ..State::default()
        };
        let action = |state: &State| setup_action(state, &object, operation);
        let expected = json!({
            "id":"s1", "kind":"setup", "subject":[{"kind":"Resource", "name":"shared", "id":"o1"}],
            "evidence":[
                {"kind":"fact", "value":"s1", "where":"records",
                    "proves":"the set-up.v1 invocation that did not succeed"},
                {"kind":"fact", "value":"failed", "where":"records",
                    "proves":"the outcome that setup ended with"},
                {"kind":"fact", "value":[diagnostic], "where":"records",
                    "proves":"the diagnostics that setup ended with"},
                {"kind":"fact", "value":[{"name":"run", "outcome":"failed"}], "where":"status.setup",
                    "proves":"the parts of that setup that did not succeed"}
            ],
            "expiry":"2026-11-16T12:30:00Z"
        });
        assert_eq!(action(&state), Some(expected.clone()));
        assert!(is_raised_at(&expected, "2026-11-16T12:29:59Z"));
        assert!(!is_raised_at(&expected, "2026-11-16T12:30:00Z"));

        let setup = &mut state.invocations[0];
        setup["state"] = json!("unknown");
        setup["ended"]["outcome"] = json!("unknown");
        setup["setup"]["parts"][0]["outcome"] = json!("unknown");
        state.invocations[1]["state"] = json!("unknown");
        let mut unknown = expected.clone();
        unknown["evidence"][1]["value"] = json!("unknown");
        unknown["evidence"][3]["value"][0]["outcome"] = json!("unknown");
        unknown["evidence"].as_array_mut().unwrap().push(json!({
            "kind":"fact", "value":["s1", "r1"], "where":"status.unreconciled",
            "proves":"the invocations of that setup whose outcome is still unknown"
        }));
        assert_eq!(action(&state), Some(unknown.clone()));
        state.invocations[0]["state"] = json!("succeeded");
        unknown["evidence"][4]["value"] = json!(["r1"]);
        assert_eq!(action(&state), Some(unknown));

        state.invocations[0] = failed.clone();
        state.invocations.push(json!({
            "id":"s2", "operation":operation, "object":"o1", "state":"in-progress"
        }));
        assert_eq!(action(&state), None);
        state.invocations[2]["state"] = json!("succeeded");
        state.invocations[2]["ended"] = json!({"time":time, "outcome":"succeeded"});
        assert_eq!(action(&state), None);

        state.invocations[0]["dispatched"] = json!(1);
        state.invocations[2]["dispatched"] = json!(2);
        state.invocations[2]["state"] = json!("failed");
        state.invocations[2]["ended"]["outcome"] = json!("failed");
        state.invocations[0] = json!({
            "id":"s1", "operation":operation, "object":"o1", "state":"in-progress", "dispatched":3
        });
        assert_eq!(action(&state), None);
        state.invocations[0]["state"] = json!("succeeded");
        state.invocations[0]["ended"] = json!({"time":time, "outcome":"succeeded"});
        assert_eq!(action(&state), None);

        state.invocations.truncate(2);
        state.invocations[0] = failed.clone();
        state.invocations[1]["state"] = json!("failed");
        state.invocations[0]["dispatched"] = json!(1);
        assert_eq!(action(&state), Some(expected.clone()));
        state.invocations[0]["dispatched"] = json!(2);
        state.invocations[0]["ended"]["time"] = json!("2026-11-17T12:30:00Z");
        let mut retried = expected.clone();
        retried["expiry"] = json!("2026-11-24T12:30:00Z");
        assert_eq!(action(&state), Some(retried));

        state.invocations[0] = failed;
        let mut retired = object.clone();
        retired["state"] = json!("retired");
        assert_eq!(setup_action(&state, &retired, operation), None);

        state.invocations[0]
            .as_object_mut()
            .unwrap()
            .remove("ended");
        state.records = vec![
            json!({"type":"invocation-ended", "time":"2026-11-10T08:00:00Z",
            "data":{"invocation":"s1", "outcome":"failed"}}),
        ];
        let mut legacy = expected;
        legacy["expiry"] = json!("2026-11-17T08:00:00Z");
        assert_eq!(action(&state), Some(legacy));
        state.records.clear();
        assert_eq!(action(&state), None);
    }
}
