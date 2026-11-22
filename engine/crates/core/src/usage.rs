//! IFC-051 usage belongs only to the implementation call's end record.
use crate::{Context, Transition, requests::Completion, schemas};
use serde_json::{Value, json};

pub(crate) fn take(completion: &mut Completion, c: &Context) -> Option<Value> {
    let Completion::Result { result } = completion else {
        return None;
    };
    result
        .get_mut("_meta")?
        .as_object_mut()?
        .remove(&format!("{}usage", c.meta_prefix))
}

pub(crate) fn record_end(
    t: &mut Transition,
    c: &Context,
    first_record: usize,
    invocation: &str,
    usage: Value,
) {
    let Some(ended) = t.records[first_record..].iter_mut().find(|record| {
        record["type"] == "invocation-ended" && record["data"]["invocation"] == invocation
    }) else {
        return;
    };
    let Some(stored) = t
        .state
        .records
        .iter_mut()
        .rev()
        .find(|record| record["id"] == ended["id"])
    else {
        return;
    };

    match c
        .core_environment
        .first_core_failure(&c.core_package, "usage.v1", &usage)
    {
        Ok(None) => ended["data"]["usage"] = usage,
        failure => {
            let mut pointer = format!(
                "/_meta/{}",
                schemas::token(&format!("{}usage", c.meta_prefix))
            );
            let mut warning = json!({"ruleId":"IFC-051","level":"warning"});
            let detail = if let Ok(Some((at, keyword))) = failure
                && let Some((_, last)) = keyword.rsplit_once('/')
            {
                pointer.push_str(&at);
                warning["properties"] = json!({"keywordLocation":keyword});
                format!("at {pointer} it does not conform to usage.v1, failing its {last} keyword")
            } else {
                "the engine could not check it against usage.v1".into()
            };
            warning["locations"] = json!([{"logicalLocations":[{"fullyQualifiedName":pointer}]}]);
            warning["message"] = json!({"text":format!(
                "The result's usage member was dropped: {detail}. The record of this invocation's end keeps no part of it, and nothing else about the call changes (IFC-051)."
            )});
            ended["data"]["usageDropped"] = warning;
        }
    }
    *stored = ended.clone();
}
