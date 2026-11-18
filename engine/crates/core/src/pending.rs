//! Computed setup actions and stored pending actions.
use crate::{
    Allocation, Context, Result, State, Transition, access, inventory, requests, retirement,
};
use chrono::{DateTime, SecondsFormat, TimeDelta};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn week_after(time: &str) -> Option<String> {
    Some(
        DateTime::parse_from_rfc3339(time)
            .ok()?
            .checked_add_signed(TimeDelta::days(7))?
            .to_utc()
            .to_rfc3339_opts(SecondsFormat::AutoSi, true),
    )
}

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
        let ended = legacy_ending(&state.records, &setup["id"])?;
        (&ended["time"], &ended["data"]["outcome"])
    };
    if !matches!(outcome.as_str(), Some("failed" | "unknown")) {
        return None;
    }
    let expiry = week_after(time.as_str()?)?;

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

fn legacy_ending<'a>(records: &'a [Value], id: &Value) -> Option<&'a Value> {
    records.iter().rev().find(|r| {
        r["type"] == "invocation-ended"
            && r["data"]["invocation"] == *id
            && r["data"]["reconciled"] != true
    })
}

fn agent_fleet<'a>(state: &'a State, id: &Value) -> Option<&'a str> {
    let agent = access::object_by_id(state, id)
        .filter(|object| object["kind"] == "Agent" && object.get("spec").is_none())?;
    let owner = access::object_by_id(state, &agent["owner"]).filter(|o| o["kind"] == "Fleet")?;
    owner["id"].as_str()
}

pub(crate) fn raise_notices(
    state: &mut State,
    resource: &Value,
    invocation: &Value,
    time: &str,
    allocation: &mut Allocation<'_>,
    context: &Context,
) -> Result<Vec<String>> {
    if resource.get("spec").is_some()
        || access::management(state, "root", context)
            .is_ok_and(|root| root["id"] == invocation["sender"])
    {
        return Ok(vec![]);
    }
    let sender_fleet = agent_fleet(state, &invocation["sender"]);
    let mut indices = BTreeMap::new();
    let mut fleets: Vec<(String, Vec<Value>)> = Vec::new();
    for conduit in retirement::closing_conduits(state, &resource["id"], false) {
        let Some(fleet) =
            agent_fleet(state, &conduit["consumer"]).filter(|f| Some(*f) != sender_fleet)
        else {
            continue;
        };
        let index = *indices.entry(fleet).or_insert_with(|| {
            fleets.push((fleet.to_owned(), Vec::new()));
            fleets.len() - 1
        });
        fleets[index].1.push(conduit.clone());
    }
    if fleets.is_empty() {
        return Ok(vec![]);
    }

    let expiry = week_after(time).ok_or("notice expiry is out of range")?;
    let resource_id = &resource["id"];
    let mut evidence = vec![fact(
        resource_id.clone(),
        "records",
        "the Resource whose retirement closed these Conduits",
    )];
    if let Some(owner) = resource.get("owner") {
        evidence.push(fact(
            access::id(owner).clone(),
            "declared",
            "the Pool that owns that Resource",
        ));
    }
    evidence.extend([
        fact(
            invocation["sender"].clone(),
            "records",
            "the instance that sent the retirement",
        ),
        fact(
            invocation["caller"].clone(),
            "records",
            "the caller on whose behalf the retirement was sent",
        ),
        fact(
            invocation["id"].clone(),
            "records",
            "the retirement invocation that closed these Conduits",
        ),
    ]);

    let mut notices = Vec::with_capacity(fleets.len());
    for (fleet, conduits) in fleets {
        let id = allocation.next(state)?;
        let mut subjects = vec![inventory::object(resource)];
        let mut items = evidence.clone();
        let mut subject_about = vec![resource_id];
        let mut evidence_about = vec![resource_id; items.len()];
        for conduit in &conduits {
            let consumer = access::id(&conduit["consumer"]);
            subjects.push(inventory::object(conduit));
            items.push(fact(
                json!({"id":conduit["id"],"name":conduit["name"],"consumer":consumer}),
                "status.closedConduits",
                "a Conduit of an Agent of this Fleet that the retirement closed",
            ));
            subject_about.push(consumer);
            evidence_about.push(consumer);
        }
        let action =
            json!({"id":id,"kind":"notice","subject":subjects,"evidence":items,"expiry":expiry});
        state.pending_actions.insert(
            id.clone(),
            json!({
                "action":action,"dispatched":null,"raisedBy":invocation["id"],"fleet":fleet,
                "about":{"subject":subject_about,"evidence":evidence_about}
            }),
        );
        notices.push(id);
    }
    Ok(notices)
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

pub(crate) fn action_named(
    state: &State,
    id: &str,
    time: Option<&str>,
    context: &Context,
) -> Option<Value> {
    let stored = state.pending_actions.get(id);
    let operation = context.qualified("set-up.v1");
    if let Some(setup) = state
        .invocations
        .iter()
        .find(|v| v["id"] == id && v["operation"] == operation)
        && let Some(object) = access::object_by_id(state, &setup["object"])
        && let Some(mut action) = setup_action(state, object, &operation)
        && action["id"] == id
        && stored.is_none_or(|entry| entry["dispatched"] != setup["dispatched"])
    {
        if time.is_some_and(|time| !is_raised_at(&action, time)) {
            action["answer"] = expired_answer(&action);
        }
        return Some(action);
    }
    stored.map(|entry| {
        let mut action = entry["action"].clone();
        if action.get("answer").is_none() && time.is_some_and(|time| !is_raised_at(&action, time)) {
            action["answer"] = expired_answer(&action);
        }
        action
    })
}

pub(crate) fn unanswered_sign_in<'a>(
    state: &'a State,
    instance: &Value,
    time: &str,
) -> Option<&'a Value> {
    state
        .pending_actions
        .values()
        .map(|entry| &entry["action"])
        .find(|action| {
            action["kind"] == "sign-in"
                && action["subject"][0]["id"] == instance["id"]
                && action.get("answer").is_none()
                && is_raised_at(action, time)
        })
}

pub(crate) enum SignInRequest<'a> {
    Invocation {
        invocation: &'a Value,
        trigger: &'a str,
    },
    Held {
        access_record_id: &'a str,
        operation: &'a str,
    },
}

pub(crate) fn raise_sign_in(
    state: &mut State,
    instance: &Value,
    account: &str,
    pointer: &str,
    request: SignInRequest<'_>,
    a: &mut Allocation<'_>,
) -> Result<Option<Value>> {
    let time = &a.evidence.time;
    if instance["state"] == "retired" || unanswered_sign_in(state, instance, time).is_some() {
        return Ok(None);
    }
    let instance_id = instance["id"]
        .as_str()
        .ok_or("sign-in instance has no ID")?;
    let expiry = week_after(time).ok_or("cannot compute sign-in action expiry")?;
    let id = a.next(state)?;
    let (raised_by, request_id, operation, trigger, request_fact, trigger_fact) = match request {
        SignInRequest::Invocation {
            invocation,
            trigger,
        } => (
            invocation["id"].clone(),
            invocation["id"].clone(),
            invocation["operation"].clone(),
            trigger,
            "the request whose call reported a missing or lapsed login",
            "how the implementation reported it",
        ),
        SignInRequest::Held {
            access_record_id,
            operation,
        } => (
            Value::Null,
            json!(access_record_id),
            json!(operation),
            "held-request",
            "the request that found the instance held",
            "how the instance was found awaiting sign-in",
        ),
    };
    let action = json!({
        "id": id,
        "kind": "sign-in",
        "subject": [{"kind":instance["kind"], "name":instance["name"], "id":instance["id"]}],
        "evidence": [
            fact(json!(account), &format!("config{pointer}"), "the account this instance is configured for"),
            fact(request_id, "records", request_fact),
            fact(operation, "records", "the operation of that request"),
            fact(json!(trigger), "records", trigger_fact)
        ],
        "expiry": expiry
    });
    state.pending_actions.insert(
        id,
        json!({"action":action, "dispatched":null, "raisedBy":raised_by}),
    );
    state
        .sign_in_holds
        .entry(instance_id.to_owned())
        .or_insert_with(|| json!({"since":time}));
    Ok(Some(action))
}

pub(crate) fn void_sign_in(
    state: &mut State,
    instance: &Value,
    time: &str,
    invocation: &Value,
) -> Option<Value> {
    let id = unanswered_sign_in(state, instance, time)?["id"]
        .as_str()?
        .to_owned();
    let action = &mut state.pending_actions.get_mut(&id)?["action"];
    action["answer"] = invocation_answer("void", invocation, time);
    Some(action.clone())
}

// Sign-in actions are answered by sign-in or retirement, not answer.v1.
const ACCEPTED_ANSWERS: &[(&str, &str)] = &[("setup", "dismissed"), ("notice", "acknowledged")];

pub(crate) fn accepts_answer(kind: &Value, answer: &Value) -> bool {
    ACCEPTED_ANSWERS
        .iter()
        .any(|(k, a)| kind == *k && answer == *a)
}

fn expired_answer(action: &Value) -> Value {
    json!({"value":"expired","at":action["expiry"]})
}

pub(crate) fn invocation_answer(value: &str, invocation: &Value, time: &str) -> Value {
    json!({"value":value,"actor":invocation["caller"],"invocation":invocation["id"],"at":time})
}

fn setup_dispatch<'a>(state: &'a State, id: &str) -> &'a Value {
    state
        .invocations
        .iter()
        .find(|v| v["id"] == id)
        .map_or(&Value::Null, |v| &v["dispatched"])
}

pub(crate) fn store_answer(state: &mut State, mut action: Value, answer: Value) -> Value {
    let id = action["id"]
        .as_str()
        .expect("pending action has an ID")
        .to_owned();
    action["answer"] = answer;
    if action["kind"] != "setup"
        && let Some(entry) = state.pending_actions.get_mut(&id)
    {
        entry["action"]["answer"] = action["answer"].clone();
    } else {
        let entry = json!({"action":action,"dispatched":setup_dispatch(state, &id)});
        state.pending_actions.insert(id, entry);
    }
    action
}

pub(crate) fn answer_before_change(
    state: &mut State,
    object: &Value,
    time: &str,
    value: &str,
    invocation: &Value,
    context: &Context,
) -> Option<Value> {
    let action = setup_action(state, object, &context.qualified("set-up.v1"))?;
    let id = action["id"].as_str()?;
    if state
        .pending_actions
        .get(id)
        .is_some_and(|entry| &entry["dispatched"] == setup_dispatch(state, id))
    {
        return None;
    }
    if !is_raised_at(&action, time) {
        let answer = expired_answer(&action);
        store_answer(state, action, answer);
        return None;
    }
    Some(store_answer(
        state,
        action,
        invocation_answer(value, invocation, time),
    ))
}

pub(crate) fn record_answer(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    action: &Value,
) -> Result<()> {
    requests::record(
        t,
        a,
        c,
        "pending-action-answered",
        json!({"subject":action["subject"][0]["id"],"pendingAction":[action["id"]],
            "answer":action,"invocation":action["answer"]["invocation"]}),
    )
}

fn is_interface(holder: &Value) -> bool {
    holder["kind"] == "Interface" && holder.get("spec").is_none()
}

fn is_root(holder: &Value, state: &State, context: &Context) -> bool {
    access::management(state, "root", context).is_ok_and(|root| root["id"] == holder["id"])
}

pub(crate) fn own_interface<'a>(
    state: &'a State,
    caller: &Value,
    sender: &Value,
) -> Option<&'a Value> {
    if !caller.is_string() || caller != sender {
        return None;
    }
    access::object_by_id(state, caller).filter(|holder| is_interface(holder))
}

/// PND-002: a subject can qualify by its own scope membership or through an owner.
fn is_raised_for(
    action: &Value,
    holder: &Value,
    state: &State,
    time: &str,
    context: &Context,
) -> bool {
    is_raised_at(action, time)
        && action.get("answer").is_none()
        && action["id"]
            .as_str()
            .filter(|id| state.pending_actions.contains_key(*id))
            .and_then(|id| action_named(state, id, Some(time), context))
            .is_none_or(|action| action.get("answer").is_none())
        && is_raised_at_holder(action, holder, state, context)
}

fn is_raised_at_holder(action: &Value, holder: &Value, state: &State, context: &Context) -> bool {
    is_interface(holder)
        && (is_root(holder, state, context)
            || scope_contains_subject(action, holder, state, context))
}

pub(crate) fn may_answer(
    state: &State,
    instance: &Value,
    serving: Option<&str>,
    id: &Value,
    context: &Context,
) -> bool {
    action_for_answer(state, instance, serving, id, context).is_some()
}

pub(crate) fn action_for_answer(
    state: &State,
    instance: &Value,
    serving: Option<&str>,
    id: &Value,
    context: &Context,
) -> Option<Value> {
    if serving.is_some() || !is_interface(instance) {
        return None;
    }
    action_named(state, id.as_str()?, None, context)
        .filter(|action| is_raised_at_holder(action, instance, state, context))
}

fn scope_contains_subject(
    action: &Value,
    holder: &Value,
    state: &State,
    context: &Context,
) -> bool {
    let fleet = notice_fleet(state, action);
    let subjects: Vec<_> = action["subject"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|subject| access::object_by_id(state, &subject["id"]))
        .collect();
    state
        .objects
        .iter()
        .filter(|conduit| inventory::is_scope_conduit(holder, conduit, state, context))
        .filter_map(|conduit| access::object(state, &conduit["target"]))
        .any(|target| {
            let id = &target["id"];
            fleet == Some(id)
                || subjects.iter().any(|subject| {
                    &subject["id"] == id
                        || access::id(&subject["owner"]) == id
                        || subject["kind"] == "Conduit"
                            && ["consumer", "target"].iter().any(|end| {
                                access::object_by_id(state, &subject[*end])
                                    .is_some_and(|object| access::id(&object["owner"]) == id)
                            })
                })
        })
}

pub(crate) fn notice_fleet<'a>(state: &'a State, action: &Value) -> Option<&'a Value> {
    if action["kind"] != "notice" {
        return None;
    }
    state
        .pending_actions
        .get(action["id"].as_str()?)?
        .get("fleet")
}

fn visible_items(
    items: &[Value],
    about: Option<&[Value]>,
    holder: &Value,
    state: &State,
    context: &Context,
) -> Result<Vec<Value>> {
    let mut visible = Vec::new();
    for (index, item) in items.iter().enumerate() {
        let id = match about {
            Some(ids) => ids.get(index),
            None => item.get("id"),
        };
        if let Some(object) = id.and_then(|id| access::object_by_id(state, id))
            && access::may_inspect(state, &holder["id"], object, context)?
        {
            visible.push(item.clone());
        }
    }
    Ok(visible)
}

/// PND-004: disclose evidence with its inspection authority.
fn action_view(
    action: &Value,
    holder: &Value,
    state: &State,
    context: &Context,
) -> Result<Option<Value>> {
    let mut view = action.clone();
    let mut all_visible = true;
    if !is_root(holder, state, context) {
        let about = action["id"]
            .as_str()
            .and_then(|id| state.pending_actions.get(id))
            .and_then(|entry| entry.get("about"));
        let subjects = view["subject"]
            .as_array_mut()
            .ok_or("action lacks subjects")?;
        let subject_ids = about
            .map(|about| {
                about["subject"]
                    .as_array()
                    .ok_or("action lacks subject attribution")
            })
            .transpose()?;
        let visible = visible_items(
            subjects,
            subject_ids.map(Vec::as_slice),
            holder,
            state,
            context,
        )?;
        if visible.is_empty() {
            return Ok(None);
        }
        if about.is_none() {
            all_visible = visible.len() == subjects.len();
        }
        *subjects = visible;
        if let Some(about) = about {
            view["evidence"] = json!(visible_items(
                action["evidence"]
                    .as_array()
                    .ok_or("action lacks evidence")?,
                Some(
                    about["evidence"]
                        .as_array()
                        .ok_or("action lacks evidence attribution")?
                        .as_slice()
                ),
                holder,
                state,
                context,
            )?);
        }
    }
    let evidence = view["evidence"]
        .as_array_mut()
        .ok_or("action lacks evidence")?;
    evidence.retain(|item| {
        all_visible && !matches!(item["kind"].as_str(), Some("code" | "fingerprint"))
    });
    Ok(Some(view))
}

/// PND-004: an answer's evidence follows the action's holder and inspection rules.
pub(crate) fn answer_evidence(
    action: &Value,
    holder: Option<&Value>,
    state: &State,
    context: &Context,
) -> Result<Value> {
    if let Some(holder) = holder
        && is_raised_at_holder(action, holder, state, context)
        && let Some(view) = action_view(action, holder, state, context)?
    {
        return Ok(view["evidence"].clone());
    }
    Ok(json!([]))
}

fn held_actions(state: &State, context: &Context) -> Vec<Value> {
    let setup_operation = context.qualified("set-up.v1");
    let mut actions: Vec<_> = state
        .objects
        .iter()
        .filter(|object| object.get("spec").is_none())
        .filter_map(|object| setup_action(state, object, &setup_operation))
        .collect();
    actions.extend(
        state
            .pending_actions
            .values()
            .map(|entry| &entry["action"])
            .filter(|action| action["kind"] != "setup")
            .cloned(),
    );
    actions.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
    actions
}

pub(crate) fn raised_view(
    action: &Value,
    holder: &Value,
    state: &State,
    time: &str,
    context: &Context,
) -> Result<Option<Value>> {
    if is_raised_for(action, holder, state, time, context) {
        action_view(action, holder, state, context)
    } else {
        Ok(None)
    }
}

pub(crate) fn raised_views(
    holder: &Value,
    state: &State,
    time: &str,
    context: &Context,
) -> Result<Vec<Value>> {
    let mut views = Vec::new();
    for action in held_actions(state, context) {
        if let Some(view) = raised_view(&action, holder, state, time, context)? {
            views.push(view);
        }
    }
    Ok(views)
}

fn fact(value: Value, place: &str, proves: &str) -> Value {
    json!({"kind":"fact", "value":value, "where":place, "proves":proves})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notices_follow_their_fleet() {
        let (_, context) = crate::explain::tests::fixture();
        let time = "2026-11-12T12:00:00Z";
        let later = "2026-11-12T12:00:01Z";
        let expiry = "2026-11-19T12:00:00Z";
        let root = json!({"id":"r","kind":"Interface","from":context.qualified("root")});
        let viewer = json!({"id":"v","kind":"Interface"});
        let pool_viewer = json!({"id":"w","kind":"Interface"});
        let unscoped = json!({"id":"u","kind":"Interface"});
        let resource = json!({"id":"x","name":"x","kind":"Resource","owner":"p"});
        let conduit = |id, consumer, target, open| {
            json!({"id":id,"name":id,"kind":"Conduit","consumer":consumer,"target":target,
                "open":open,"state":"ready","operations":[context.qualified("inspect.v1")]})
        };
        let initial = State {
            objects: vec![
                root.clone(),
                json!({"id":"management","kind":"Resource","from":context.qualified("management")}),
                json!({"id":"f","kind":"Fleet"}),
                json!({"id":"g","kind":"Fleet"}),
                json!({"id":"p","kind":"Pool"}),
                resource.clone(),
                json!({"id":"a1","kind":"Agent","owner":"f"}),
                json!({"id":"a2","kind":"Agent","owner":"f"}),
                json!({"id":"b1","kind":"Agent","owner":"g"}),
                viewer.clone(),
                pool_viewer.clone(),
                unscoped.clone(),
                conduit("vf", "v", "f", true),
                conduit("wp", "w", "p", true),
                conduit("k1", "a1", "x", true),
                conduit("k2", "a2", "x", false),
                conduit("k3", "b1", "x", true),
                conduit("k4", "v", "x", true),
            ],
            records: vec![json!({"type":"object-change","data":{"target":"x"}})],
            ..State::default()
        };
        let evidence = crate::Evidence {
            time: time.into(),
            ids: (1..=9).map(|n| format!("n{n}")).collect(),
            revisions: vec![],
            suffixes: vec![],
        };
        let raise = |state: &mut State, resource: &Value, sender: &str| {
            let mut allocation = Allocation {
                evidence: &evidence,
                cursor: 0,
            };
            let invocation = json!({"id":"t1","sender":sender,"caller":"u",
                "operation":context.qualified("retire.v1")});
            let ids = raise_notices(
                state,
                resource,
                &invocation,
                time,
                &mut allocation,
                &context,
            )
            .unwrap();
            (ids, allocation.cursor)
        };
        let named = |state: &State, id, time| action_named(state, id, time, &context).unwrap();
        let mut state = initial.clone();
        assert_eq!(
            raise(&mut state, &resource, "u"),
            (vec!["n1".into(), "n2".into()], 2)
        );
        let expected = json!({
            "id":"n1","kind":"notice",
            "subject":[
                {"kind":"Resource","name":"x","id":"x"},
                {"kind":"Conduit","name":"k1","id":"k1"},
                {"kind":"Conduit","name":"k2","id":"k2"}
            ],
            "evidence":[
                {"kind":"fact","value":"x","where":"records","proves":"the Resource whose retirement closed these Conduits"},
                {"kind":"fact","value":"p","where":"declared","proves":"the Pool that owns that Resource"},
                {"kind":"fact","value":"u","where":"records","proves":"the instance that sent the retirement"},
                {"kind":"fact","value":"u","where":"records","proves":"the caller on whose behalf the retirement was sent"},
                {"kind":"fact","value":"t1","where":"records","proves":"the retirement invocation that closed these Conduits"},
                {"kind":"fact","value":{"id":"k1","name":"k1","consumer":"a1"},"where":"status.closedConduits",
                    "proves":"a Conduit of an Agent of this Fleet that the retirement closed"},
                {"kind":"fact","value":{"id":"k2","name":"k2","consumer":"a2"},"where":"status.closedConduits",
                    "proves":"a Conduit of an Agent of this Fleet that the retirement closed"}
            ],
            "expiry":expiry
        });
        let about = json!({"subject":["x","a1","a2"],"evidence":["x","x","x","x","x","a1","a2"]});
        assert_eq!(named(&state, "n1", Some(later)), expected);
        assert_eq!(
            state.pending_actions["n1"],
            json!({
                "action":expected,"dispatched":null,"raisedBy":"t1","fleet":"f","about":about
            })
        );
        let second = named(&state, "n2", Some(later));
        assert_eq!(state.pending_actions["n2"]["fleet"], "g");
        assert_eq!(
            second["subject"],
            json!([
                {"kind":"Resource","name":"x","id":"x"},
                {"kind":"Conduit","name":"k3","id":"k3"}
            ])
        );

        let mut reordered = initial.clone();
        let k3 = reordered
            .objects
            .iter()
            .position(|o| o["id"] == "k3")
            .unwrap();
        let k1 = reordered
            .objects
            .iter()
            .position(|o| o["id"] == "k1")
            .unwrap();
        let first_conduit = reordered.objects.remove(k3);
        reordered.objects.insert(k1, first_conduit);
        assert_eq!(
            raise(&mut reordered, &resource, "u"),
            (vec!["n1".into(), "n2".into()], 2)
        );
        assert_eq!(reordered.pending_actions["n1"]["fleet"], "g");
        assert_eq!(reordered.pending_actions["n2"]["fleet"], "f");

        let mut agent_retirement = initial.clone();
        assert_eq!(
            raise(&mut agent_retirement, &resource, "a1"),
            (vec!["n1".into()], 1)
        );
        assert_eq!(agent_retirement.pending_actions["n1"]["fleet"], "g");
        assert_eq!(raise(&mut initial.clone(), &resource, "r"), (vec![], 0));
        let mut template = resource.clone();
        template["spec"] = json!({});
        assert_eq!(raise(&mut initial.clone(), &template, "u"), (vec![], 0));

        // Retirement removes the Conduits, but the notice keeps their resolved identities.
        state
            .objects
            .retain(|o| !matches!(o["id"].as_str(), Some("k1" | "k2" | "k3" | "k4")));
        let raised = |state: &State, holder: &Value, time| {
            is_raised_for(&expected, holder, state, time, &context)
        };
        let views = |state: &State, holder: &Value, time| {
            raised_views(holder, state, time, &context).unwrap()
        };
        let mut fleet_view = expected.clone();
        fleet_view["subject"] = json!(&expected["subject"].as_array().unwrap()[1..]);
        fleet_view["evidence"] = json!(&expected["evidence"].as_array().unwrap()[5..]);
        let mut pool_view = expected.clone();
        pool_view["subject"] = json!([expected["subject"][0]]);
        pool_view["evidence"] = json!(&expected["evidence"].as_array().unwrap()[..5]);
        let mut second_pool_view = pool_view.clone();
        second_pool_view["id"] = json!("n2");
        assert_eq!(views(&state, &root, later), vec![expected.clone(), second]);
        assert_eq!(views(&state, &viewer, later), vec![fleet_view.clone()]);
        assert_eq!(
            views(&state, &pool_viewer, later),
            vec![pool_view, second_pool_view]
        );
        assert_eq!(
            (
                raised(&state, &unscoped, later),
                views(&state, &unscoped, later)
            ),
            (false, vec![])
        );
        state.objects.push(conduit("vp", "v", "p", true));
        assert_eq!(views(&state, &viewer, later)[0], expected);

        let expired = named(&state, "n1", Some(expiry));
        assert_eq!(expired["answer"], json!({"value":"expired","at":expiry}));
        for holder in [&root, &viewer, &pool_viewer, &unscoped] {
            assert_eq!(
                (
                    raised(&state, holder, expiry),
                    views(&state, holder, expiry).len()
                ),
                (false, 0)
            );
            assert_eq!(
                may_answer(&state, holder, None, &json!("n1"), &context),
                holder != &unscoped
            );
            assert_eq!(
                is_raised_at_holder(&expired, holder, &state, &context),
                holder != &unscoped
            );
        }
        state.objects.retain(|o| o["id"] != "x");
        assert_eq!(
            (
                raised(&state, &pool_viewer, later),
                may_answer(&state, &pool_viewer, None, &json!("n1"), &context)
            ),
            (false, false)
        );
        assert!(may_answer(&state, &viewer, None, &json!("n1"), &context));
        assert_eq!(views(&state, &viewer, later)[0], fleet_view);
        assert_eq!(
            access::action_inspection_target(&state, &expected, &context).unwrap(),
            "f"
        );
        let mut with_retired_fleet = state.clone();
        with_retired_fleet
            .objects
            .iter_mut()
            .find(|o| o["id"] == "f")
            .unwrap()["state"] = json!("retired");
        assert_eq!(
            access::action_inspection_target(&with_retired_fleet, &expected, &context).unwrap(),
            "management"
        );
        with_retired_fleet.objects.retain(|o| o["id"] != "f");
        assert_eq!(
            access::action_inspection_target(&with_retired_fleet, &expected, &context).unwrap(),
            "management"
        );

        let answered = "2026-11-12T13:00:00Z";
        let answer = json!({"value":"acknowledged","actor":"r","invocation":"d1","at":answered});
        let stored = store_answer(&mut state, expected.clone(), answer.clone());
        let after_expiry = "2026-11-20T12:00:00Z";
        assert_eq!(named(&state, "n1", Some(after_expiry))["answer"], answer);
        assert_eq!(
            state.pending_actions["n1"],
            json!({
                "action":stored,"dispatched":null,"raisedBy":"t1","fleet":"f","about":about
            })
        );
        for holder in [&root, &viewer] {
            assert_eq!(
                (
                    raised(&state, holder, answered),
                    may_answer(&state, holder, None, &json!("n1"), &context)
                ),
                (false, true)
            );
        }
        state.records.clear();
        assert_eq!(named(&state, "n1", Some(after_expiry)), stored);

        for (kind, answer, accepted) in [
            ("notice", "acknowledged", true),
            ("notice", "dismissed", false),
            ("notice", "expired", false),
            ("setup", "dismissed", true),
            ("setup", "acknowledged", false),
            ("setup", "expired", false),
        ] {
            assert_eq!(accepts_answer(&json!(kind), &json!(answer)), accepted);
        }
    }

    #[test]
    fn sign_in_action_raised_by_a_lapsed_login() {
        let (_, context) = crate::explain::tests::fixture();
        let time = "2026-11-12T12:00:00Z";
        let later = "2026-11-12T12:00:01Z";
        let expiry = "2026-11-19T12:00:00Z";
        let retired = "2026-11-12T13:00:00Z";
        let root = json!({"id":"r","kind":"Interface","from":context.qualified("root")});
        let viewer = json!({"id":"v","kind":"Interface"});
        let agent = json!({"id":"s1","name":"s1","kind":"Agent","owner":"f",
            "config":{"account":"primary-account"}});
        let mut state = State {
            objects: vec![
                root.clone(),
                viewer.clone(),
                agent.clone(),
                json!({"id":"u","kind":"Interface"}),
                json!({"id":"f","name":"f","kind":"Fleet"}),
                json!({"id":"p","name":"p","kind":"Pool"}),
                json!({"id":"x","name":"x","kind":"Resource","owner":"p"}),
                json!({"id":"a1","name":"a1","kind":"Agent","owner":"f"}),
                json!({"id":"a2","kind":"Agent","owner":"f"}),
                json!({"id":"management","kind":"Resource","from":context.qualified("management")}),
                json!({"id":"scope","kind":"Conduit","consumer":"v","target":"f",
                    "open":true,"state":"ready","operations":[context.qualified("inspect.v1")]}),
            ],
            records: vec![json!({"type":"invocation-ended","data":{"invocation":"c1"}})],
            ..State::default()
        };
        let raise = |state: &mut State, at, invocation, id, trigger| {
            let evidence = serde_json::from_value(json!({"time":at,"ids":[id]})).unwrap();
            let mut allocation = Allocation {
                evidence: &evidence,
                cursor: 0,
            };
            let action = raise_sign_in(
                state,
                &agent,
                "primary-account",
                "/account",
                SignInRequest::Invocation {
                    invocation: &json!({"id":invocation,"operation":"example.org/agent#ping.v1"}),
                    trigger,
                },
                &mut allocation,
            )
            .unwrap();
            assert_eq!(allocation.cursor, usize::from(action.is_some()));
            action
        };
        let action = raise(&mut state, time, "c1", "first-action", "lapsed-login").unwrap();
        assert_eq!(
            action,
            json!({
                "id":"first-action", "kind":"sign-in", "subject":[{"kind":"Agent","name":"s1","id":"s1"}],
                "evidence":[
                    {"kind":"fact","value":"primary-account","where":"config/account",
                        "proves":"the account this instance is configured for"},
                    {"kind":"fact","value":"c1","where":"records",
                        "proves":"the request whose call reported a missing or lapsed login"},
                    {"kind":"fact","value":"example.org/agent#ping.v1","where":"records",
                        "proves":"the operation of that request"},
                    {"kind":"fact","value":"lapsed-login","where":"records",
                        "proves":"how the implementation reported it"}
                ],
                "expiry":expiry
            })
        );
        assert_eq!(
            state.pending_actions["first-action"],
            json!({"action":action,"dispatched":null,"raisedBy":"c1"})
        );
        let mut before_expiry = state.clone();
        assert!(raise(&mut state, later, "c2", "unused-action", "url-elicitation").is_none());
        assert_eq!(state.pending_actions, before_expiry.pending_actions);
        let views = |state: &State, holder, at| raised_views(holder, state, at, &context).unwrap();
        assert_eq!(views(&state, &root, later), vec![action.clone()]);
        assert_eq!(views(&state, &viewer, later), vec![action.clone()]);
        assert_eq!(views(&state, &viewer, later).len(), 1);
        assert!(views(&state, &root, expiry).is_empty());
        assert!(views(&state, &viewer, expiry).is_empty());
        let lookup =
            |state: &State, at| action_named(state, "first-action", Some(at), &context).unwrap();
        let expired_answer = json!({"value":"expired","at":expiry});
        assert_eq!(lookup(&state, expiry)["answer"], expired_answer);
        let replacement = raise(
            &mut state,
            expiry,
            "c3",
            "replacement-action",
            "lapsed-login",
        )
        .unwrap();
        assert_ne!(replacement["id"], action["id"]);
        assert_eq!(replacement["expiry"], "2026-11-26T12:00:00Z");
        assert!(may_answer(&state, &root, None, &action["id"], &context));
        assert!(ACCEPTED_ANSWERS.iter().all(|(kind, _)| *kind != "sign-in"));
        assert!(!accepts_answer(&action["kind"], &json!("dismissed")));

        let retirement = json!({"id":"retirement","caller":"r"});
        let voided = void_sign_in(&mut before_expiry, &agent, retired, &retirement).unwrap();
        let void_answer =
            json!({"value":"void","actor":"r","invocation":"retirement","at":retired});
        assert_eq!(voided["answer"], void_answer);
        assert_eq!(
            before_expiry.pending_actions["first-action"],
            json!({"action":voided,"dispatched":null,"raisedBy":"c1"})
        );
        assert!(views(&before_expiry, &root, retired).is_empty());
        assert!(void_sign_in(&mut before_expiry, &agent, retired, &retirement).is_none());
        state.records.clear();
        before_expiry.records.clear();
        assert_eq!(lookup(&state, expiry)["answer"], expired_answer);
        assert_eq!(lookup(&before_expiry, expiry)["answer"], void_answer);
    }

    #[test]
    fn hold_outlasts_expiry() {
        let (_, context) = crate::explain::tests::fixture();
        let time = "2026-11-12T12:00:00Z";
        let expiry = "2026-11-19T12:00:00Z";
        let agent = json!({"id":"s1","name":"s1","kind":"Agent","owner":"f",
            "config":{"account":"primary-account"}});
        let mut state = State {
            objects: vec![
                json!({"id":"r","kind":"Interface","from":context.qualified("root")}),
                json!({"id":"v","kind":"Interface"}),
                agent.clone(),
                json!({"id":"u","kind":"Interface"}),
                json!({"id":"f","name":"f","kind":"Fleet"}),
                json!({"id":"p","name":"p","kind":"Pool"}),
                json!({"id":"x","name":"x","kind":"Resource","owner":"p"}),
                json!({"id":"a1","name":"a1","kind":"Agent","owner":"f"}),
                json!({"id":"a2","kind":"Agent","owner":"f"}),
                json!({"id":"management","kind":"Resource","from":context.qualified("management")}),
                json!({"id":"scope","kind":"Conduit","consumer":"v","target":"f",
                    "open":true,"state":"ready","operations":[context.qualified("inspect.v1")]}),
            ],
            records: vec![json!({"type":"invocation-ended","data":{"invocation":"c1"}})],
            ..State::default()
        };
        let raise = |state: &mut State, at, id, request| {
            let evidence = serde_json::from_value(json!({"time":at,"ids":[id]})).unwrap();
            let mut allocation = Allocation {
                evidence: &evidence,
                cursor: 0,
            };
            let action = raise_sign_in(
                state,
                &agent,
                "primary-account",
                "/account",
                request,
                &mut allocation,
            )
            .unwrap();
            assert_eq!(allocation.cursor, usize::from(action.is_some()));
            action
        };
        let operation = "example.org/sign-in-hold#work.v1";
        let invocation = json!({"id":"c1","operation":operation});
        let first = raise(
            &mut state,
            time,
            "first-action",
            SignInRequest::Invocation {
                invocation: &invocation,
                trigger: "lapsed-login",
            },
        )
        .unwrap();
        let hold = json!({"since":time});
        assert_eq!(state.sign_in_holds["s1"], hold);
        let expired = action_named(&state, "first-action", Some(expiry), &context).unwrap();
        assert_eq!(expired["id"], first["id"]);
        assert_eq!(expired["answer"], json!({"value":"expired","at":expiry}));
        assert_eq!(unanswered_sign_in(&state, &agent, expiry), None);
        assert_eq!(state.sign_in_holds["s1"], hold);

        let replacement = raise(
            &mut state,
            expiry,
            "replacement-action",
            SignInRequest::Held {
                access_record_id: "h1",
                operation,
            },
        )
        .unwrap();
        assert_ne!(replacement["id"], first["id"]);
        assert_eq!(replacement["expiry"], "2026-11-26T12:00:00Z");
        assert_eq!(
            replacement["evidence"],
            json!([
                {"kind":"fact","value":"primary-account","where":"config/account",
                    "proves":"the account this instance is configured for"},
                {"kind":"fact","value":"h1","where":"records",
                    "proves":"the request that found the instance held"},
                {"kind":"fact","value":operation,"where":"records",
                    "proves":"the operation of that request"},
                {"kind":"fact","value":"held-request","where":"records",
                    "proves":"how the instance was found awaiting sign-in"}
            ])
        );
        assert_eq!(
            state.pending_actions["replacement-action"],
            json!({"action":replacement,"dispatched":null,"raisedBy":null})
        );
        assert_eq!(state.sign_in_holds["s1"], hold);
        let actions = state.pending_actions.clone();
        assert_eq!(actions.len(), 2);
        assert_eq!(
            raise(
                &mut state,
                expiry,
                "unused-action",
                SignInRequest::Held {
                    access_record_id: "h2",
                    operation,
                },
            ),
            None
        );
        assert_eq!(state.pending_actions, actions);
        assert_eq!(
            unanswered_sign_in(&state, &agent, expiry),
            Some(&replacement)
        );

        state.records.clear();
        assert_eq!(state.sign_in_holds["s1"], hold);
        assert_eq!(state.pending_actions, actions);
        let mut retired = state.clone();
        let at = "2026-11-19T13:00:00Z";
        let voided = void_sign_in(
            &mut retired,
            &agent,
            at,
            &json!({"id":"retirement","caller":"r"}),
        )
        .unwrap();
        assert_eq!(voided["id"], replacement["id"]);
        assert_eq!(
            retired.pending_actions["replacement-action"]["action"]["answer"],
            json!({"value":"void","actor":"r","invocation":"retirement","at":at})
        );
        assert_eq!(retired.sign_in_holds["s1"], hold);
    }

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

    #[test]
    fn actions_raised_by_scope() {
        let (_, context) = crate::explain::tests::fixture();
        let ended = "2026-11-09T12:30:00Z";
        let time = "2026-11-09T12:30:01Z";
        let expiry = "2026-11-16T12:30:00Z";
        let operation = context.qualified("set-up.v1");
        let inspect = context.qualified("inspect.v1");
        let list = context.qualified("list.v1");
        let root = json!({"id":"r","kind":"Interface","from":context.qualified("root")});
        let viewer = json!({"id":"v","kind":"Interface"});
        let unscoped = json!({"id":"u","kind":"Interface"});
        let agent = json!({"id":"a2","kind":"Agent","owner":"f"});
        let a1 = json!({"id":"a1","name":"a1","kind":"Agent","owner":"f"});
        let resource = json!({"id":"x","name":"x","kind":"Resource","owner":"p"});
        let conduit = |id, consumer, target, open, op: &str| {
            json!({"id":id,"name":id,"kind":"Conduit","consumer":consumer,"target":target,
                "open":open,"state":"ready","operations":[op]})
        };
        let failed_setup = |id, object| {
            json!({"id":id,"operation":operation,"object":object,"state":"failed",
                "ended":{"time":ended,"outcome":"failed"}})
        };
        let mut state = State {
            objects: vec![
                root.clone(),
                json!({"id":"management","kind":"Resource","from":context.qualified("management")}),
                json!({"id":"f","name":"f","kind":"Fleet"}),
                json!({"id":"p","name":"p","kind":"Pool"}),
                viewer.clone(),
                unscoped.clone(),
                resource.clone(),
                a1.clone(),
                agent.clone(),
                conduit("c1", "v", "f", true, &inspect),
                conduit("c2", "a2", "f", true, &inspect),
                conduit("c3", "v", "p", false, &inspect),
                conduit("c4", "v", "p", true, &list),
            ],
            invocations: vec![failed_setup("s2", "x"), failed_setup("s1", "a1")],
            ..State::default()
        };
        let a1_action = setup_action(&state, &a1, &operation).unwrap();
        let x_action = setup_action(&state, &resource, &operation).unwrap();
        let both = vec![a1_action.clone(), x_action];
        let views = |state, holder, at| raised_views(holder, state, at, &context).unwrap();
        assert_eq!(views(&state, &root, time), both);
        let scoped = views(&state, &viewer, time);
        assert_eq!(scoped, vec![a1_action]);
        assert_eq!(scoped.len(), 1);
        assert_eq!(views(&state, &agent, time).len(), 0);
        assert_eq!(views(&state, &unscoped, time).len(), 0);

        let mut open_scope = state.clone();
        for path in &mut open_scope.objects {
            if path["id"] == "c3" {
                path["open"] = json!(true);
            }
        }
        let scoped = views(&open_scope, &viewer, time);
        assert_eq!(scoped, both);
        assert_eq!(scoped.len(), 2);
        assert_eq!(views(&open_scope, &root, expiry).len(), 0);
        assert_eq!(views(&open_scope, &viewer, expiry).len(), 0);

        let observed = fact(json!("observed"), "status", "a fact");
        let notice = json!({"id":"n1","kind":"notice",
            "subject":[inventory::object(&a1),inventory::object(&resource)],
            "evidence":[
                {"kind":"code","value":"example-code","where":"panel","proves":"a code"},
                {"kind":"fingerprint","value":"123456","where":"panel","proves":"a fingerprint"},
                observed
            ],
            "expiry":expiry
        });
        let mut full_view = notice.clone();
        full_view["evidence"] = json!([observed]);
        let mut partial_view = notice.clone();
        partial_view["subject"] = json!([inventory::object(&a1)]);
        partial_view["evidence"] = json!([]);
        let raised = |state: &State, holder: &Value, action: &Value| {
            is_raised_for(action, holder, state, time, &context)
        };
        let view = |state: &State, holder: &Value, action: &Value| {
            action_view(action, holder, state, &context).unwrap()
        };
        assert_eq!(
            (raised(&state, &root, &notice), view(&state, &root, &notice)),
            (true, Some(full_view.clone()))
        );
        assert_eq!(
            (
                raised(&state, &viewer, &notice),
                view(&state, &viewer, &notice)
            ),
            (true, Some(partial_view))
        );
        assert_eq!(
            (
                raised(&open_scope, &viewer, &notice),
                view(&open_scope, &viewer, &notice)
            ),
            (true, Some(full_view))
        );

        let endpoint_conduit = conduit("c5", "a2", "x", true, &inspect);
        let conduit_notice = json!({"id":"n2","kind":"notice",
            "subject":[inventory::object(&endpoint_conduit)],"evidence":[observed],"expiry":expiry});
        state.objects.push(endpoint_conduit);
        assert_eq!(
            (
                raised(&state, &viewer, &conduit_notice),
                view(&state, &viewer, &conduit_notice)
            ),
            (true, None)
        );
        assert_eq!(view(&state, &root, &conduit_notice), Some(conduit_notice));
    }

    #[test]
    fn stored_answers_follow_their_ending() {
        let (_, context) = crate::explain::tests::fixture();
        let ended = "2026-11-09T12:30:00Z";
        let first = "2026-11-09T12:30:01Z";
        let answered = "2026-11-09T13:30:00Z";
        let second_end = "2026-11-09T14:30:00Z";
        let expiry = "2026-11-16T12:30:00Z";
        let eight_days = "2026-11-17T12:30:00Z";
        let nine_days = "2026-11-18T12:30:00Z";
        let operation = context.qualified("set-up.v1");
        let root = json!({"id":"r","kind":"Interface","from":context.qualified("root")});
        let viewer = json!({"id":"v","kind":"Interface"});
        let agent = json!({"id":"a1","name":"a1","kind":"Agent","owner":"f"});
        let resource = json!({"id":"x","name":"x","kind":"Resource","owner":"p"});
        let initial = State {
            objects: vec![
                root.clone(),
                viewer.clone(),
                agent.clone(),
                resource.clone(),
                json!({"id":"f","name":"f","kind":"Fleet"}),
                json!({"id":"p","name":"p","kind":"Pool"}),
                json!({"id":"management","kind":"Resource","from":context.qualified("management")}),
                json!({"id":"c1","name":"c1","kind":"Conduit","consumer":"v","target":"f",
                    "open":true,"state":"ready","operations":[context.qualified("inspect.v1")]}),
            ],
            invocations: vec![
                json!({"id":"s1","operation":operation,"object":"a1","state":"failed",
                "dispatched":1,"ended":{"time":ended,"outcome":"failed"}}),
            ],
            records: vec![json!({"type":"invocation-ended","time":ended,
                "data":{"invocation":"s1","outcome":"failed"}})],
            ..State::default()
        };
        let lookup = |state: &State, id, time| action_named(state, id, time, &context).unwrap();
        let original = lookup(&initial, "s1", Some(first));
        let expired = json!({"value":"expired","at":expiry});
        let assert_answer = |state: &State, id, time, expected: &Value| {
            assert_eq!(&lookup(state, id, Some(time))["answer"], expected);
            let mut purged = state.clone();
            purged.records.clear();
            assert_eq!(&lookup(&purged, id, Some(time))["answer"], expected);
        };
        assert_eq!(original.get("answer"), None);
        assert_answer(&initial, "s1", expiry, &expired);

        let mut dismissed = initial.clone();
        let dismissal = json!({"value":"dismissed","actor":"r","invocation":"d1","at":answered});
        let stored = store_answer(&mut dismissed, original.clone(), dismissal.clone());
        assert_eq!(
            dismissed.pending_actions["s1"],
            json!({"action":stored,"dispatched":1})
        );
        assert_answer(&dismissed, "s1", answered, &dismissal);
        assert_answer(&dismissed, "s1", eight_days, &dismissal);
        let raised = |state, holder, time| is_raised_for(&original, holder, state, time, &context);
        assert_eq!(
            [
                raised(&dismissed, &root, answered),
                raised(&dismissed, &viewer, answered)
            ],
            [false, false]
        );

        let allowed =
            |state, holder, serving, id| may_answer(state, holder, serving, &json!(id), &context);
        let mut expired_state = initial.clone();
        let retry = json!({"id":"s2","caller":"r"});
        assert_eq!(
            answer_before_change(
                &mut expired_state,
                &agent,
                eight_days,
                "retried",
                &retry,
                &context
            ),
            None
        );
        for state in [&initial, &dismissed, &expired_state] {
            assert_eq!(
                [
                    allowed(state, &root, None, "s1"),
                    allowed(state, &viewer, None, "s1")
                ],
                [true, true]
            );
        }
        let mut closed = dismissed.clone();
        closed.objects.iter_mut().find(|o| o["id"] == "c1").unwrap()["open"] = json!(false);
        assert_eq!(
            [
                allowed(&closed, &viewer, None, "s1"),
                allowed(&initial, &agent, None, "s1"),
                allowed(&initial, &root, Some("d1"), "s1"),
                allowed(&initial, &root, None, "absent"),
            ],
            [false; 4]
        );

        let target = |state: &State, action| {
            access::action_inspection_target(state, action, &context)
                .unwrap()
                .clone()
        };
        assert_eq!(target(&initial, &original), json!("f"));
        let mut retired_owner = initial.clone();
        retired_owner
            .objects
            .iter_mut()
            .find(|o| o["id"] == "f")
            .unwrap()["state"] = json!("retired");
        assert_eq!(target(&retired_owner, &original), json!("management"));
        let resource_action = json!({"subject":[inventory::object(&resource)]});
        assert_eq!(target(&initial, &resource_action), json!("p"));
        retired_owner.objects.retain(|o| o["id"] != "a1");
        assert_eq!(target(&retired_owner, &original), json!("management"));

        let mut retried = initial.clone();
        let retry_answer = json!({"value":"retried","actor":"r","invocation":"s2","at":answered});
        let action =
            answer_before_change(&mut retried, &agent, answered, "retried", &retry, &context)
                .unwrap();
        assert_eq!(action["answer"], retry_answer);
        assert_answer(&retried, "s1", answered, &retry_answer);
        retried
            .invocations
            .push(json!({"id":"s2","operation":operation,"object":"a1",
            "state":"failed","dispatched":2,"ended":{"time":second_end,"outcome":"failed"}}));
        assert_eq!(lookup(&retried, "s2", Some(second_end)).get("answer"), None);
        assert_answer(&retried, "s1", second_end, &retry_answer);

        let mut retired = initial.clone();
        let retirement = json!({"id":"t1","caller":"r"});
        let retire_answer = json!({"value":"retired","actor":"r","invocation":"t1","at":answered});
        let action = answer_before_change(
            &mut retired,
            &agent,
            answered,
            "retired",
            &retirement,
            &context,
        )
        .unwrap();
        assert_eq!(action["answer"], retire_answer);
        assert_eq!(
            answer_before_change(
                &mut dismissed,
                &agent,
                answered,
                "retired",
                &retirement,
                &context
            ),
            None
        );
        for state in [&mut retired, &mut dismissed] {
            state.objects.iter_mut().find(|o| o["id"] == "a1").unwrap()["state"] = json!("retired");
        }
        assert_answer(&retired, "s1", answered, &retire_answer);
        assert_answer(&dismissed, "s1", answered, &dismissal);

        let redispatch = |state: &mut State| {
            state.invocations[0] = json!({"id":"s1","operation":operation,"object":"a1",
                "state":"in-progress","dispatched":2,"caller":"r"});
        };
        let end_redispatch = |state: &mut State, time| {
            state.invocations[0]["state"] = json!("failed");
            state.invocations[0]["ended"] = json!({"time":time,"outcome":"failed"});
            lookup(state, "s1", Some(time))
        };
        let mut redispatched = initial.clone();
        store_answer(&mut redispatched, original.clone(), dismissal.clone());
        let retry = json!({"id":"s1","caller":"r"});
        assert_eq!(
            answer_before_change(
                &mut redispatched,
                &agent,
                eight_days,
                "retried",
                &retry,
                &context
            ),
            None
        );
        redispatch(&mut redispatched);
        assert_answer(&redispatched, "s1", eight_days, &dismissal);
        let action = end_redispatch(&mut redispatched, eight_days);
        assert_eq!(action.get("answer"), None);
        assert_eq!(
            (
                &action["expiry"],
                is_raised_for(
                    &action,
                    &root,
                    &redispatched,
                    "2026-11-17T12:30:01Z",
                    &context
                ),
            ),
            (&json!("2026-11-24T12:30:00Z"), true)
        );

        redispatch(&mut expired_state);
        assert_answer(&expired_state, "s1", eight_days, &expired);
        let action = end_redispatch(&mut expired_state, nine_days);
        assert_eq!(action.get("answer"), None);
        assert_eq!(action["expiry"], json!("2026-11-25T12:30:00Z"));

        assert_eq!(
            ["dismissed", "retried", "retired", "expired"]
                .map(|value| accepts_answer(&json!("setup"), &json!(value))),
            [true, false, false, false]
        );
    }
}
