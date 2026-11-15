use super::*;

pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    let input = step.input;
    let state = step.state;
    let context = step.context;
    let mut next: State;
    let actual = {
        // A boundary snapshot after restart, optionally observing records already
        // purged by the records service. This does not implement purge.v1.
        let bytes = serde_json::to_vec(state).map_err(|e| e.to_string())?;
        next = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        if input["records_purged"] == true {
            next.record_gaps.push(json!({"cause":"purged-observation","records":next.records.iter().map(|r| json!({"source":r["source"],"id":r["id"]})).collect::<Vec<_>>()}));
            next.records.clear();
        }
        let evidence = input
            .get("evidence")
            .cloned()
            .unwrap_or_else(|| json!({"time":"2026-11-09T00:00:00Z","ids":[]}));
        if next.invocations.iter().any(|v| v["state"] == "in-progress")
            && input.get("evidence").is_none()
        {
            return Err("interrupted recovery requires supplied record evidence".into());
        }
        let t = conduit_core::step(
            &next,
            Input::Recover {
                evidence: serde_json::from_value(evidence).map_err(|e| e.to_string())?,
            },
            context,
        )?;
        next = t.state.clone();
        serde_json::to_value(t).map_err(|e| e.to_string())?
    };
    Ok((actual, next))
}
