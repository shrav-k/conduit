//! OPS-037: admission and the two transitions of a bounded engine stop.
use crate::{
    Allocation, Context, Evidence, Input, RequestReply, Result, State, Transition, access,
    decision, explain, packages, requests, schemas,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum StopState {
    Asked,
    // Transient within one transition; never returned as committed state.
    Cutting,
    Stopped,
}

pub(crate) fn cutting(state: &State) -> bool {
    state.stop == Some(StopState::Cutting)
}

fn running(state: &State) -> bool {
    state
        .invocations
        .iter()
        .any(|v| v["state"] == "in-progress")
}

fn record(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, mut data: Value) -> Result<()> {
    let system = access::management(&t.state, "system", c)?;
    data["target"] = system["id"].clone();
    t.records
        .push(a.record(&mut t.state, c, "engine-stop", data)?);
    Ok(())
}

fn ended(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, cut: bool) -> Result<()> {
    record(t, a, c, json!({"phase":"stopped","cut":cut}))?;
    t.state.stop = Some(StopState::Stopped);
    Ok(())
}

pub(crate) fn ask(
    t: &mut Transition,
    evidence: &Evidence,
    means: Option<String>,
    grace_ended: bool,
    c: &Context,
) -> Result<()> {
    if t.state.stop == Some(StopState::Stopped) {
        t.reply = json!({"stopped":true,"cut":[]});
        return Ok(());
    }
    let mut a = Allocation {
        evidence,
        cursor: 0,
    };
    if t.state.stop.is_none() {
        let mut data = json!({"phase":"asked"});
        if let Some(means) = means {
            if means.is_empty() {
                return Err("stop means must not be empty".into());
            }
            data["means"] = json!(means);
        }
        record(t, &mut a, c, data)?;
        t.state.stop = Some(StopState::Asked);
        t.decisions.push(decision(
            "ask-engine-stop",
            &["OPS-037", "REC-001"],
            &[],
            json!({}),
            vec![],
        ));
        // Discovery has not dispatched the call (REC-007). End held calls at
        // the ask, while preserving discovery that belongs to ongoing work.
        let held: Vec<_> = t
            .state
            .invocations
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                let part = ["setup_parent", "cascade_parent", "serving"]
                    .iter()
                    .any(|key| {
                        v[*key].is_string()
                            && t.state
                                .invocations
                                .iter()
                                .any(|p| p["id"] == v[*key] && p["state"] == "in-progress")
                    })
                    || work(&t.state, &v["sender"], None);
                (v["state"] == "in-progress"
                    && v["discovery"] == "pending"
                    && v["hook"] != true
                    && !part)
                    .then_some(i)
            })
            .collect();
        // Removing a fresh invocation must not shift the remaining indices.
        for i in held.into_iter().rev() {
            requests::stop_unsent(t, &mut a, c, i)?;
        }
    }
    if !grace_ended {
        t.reply = json!({"stopping":true});
        if !running(&t.state) {
            ended(t, &mut a, c, false)?;
        }
        return Ok(());
    }
    t.state.stop = Some(StopState::Cutting);
    let mut cut_entries = std::collections::BTreeSet::new();
    loop {
        let in_progress: Vec<usize> = t
            .state
            .invocations
            .iter()
            .enumerate()
            .filter(|(_, v)| v["state"] == "in-progress")
            .map(|(i, _)| i)
            .collect();
        if in_progress.is_empty() {
            break;
        }
        cut_entries.extend(in_progress.iter().copied());
        // Parts end through their owner's cancellation, including shared secret reads.
        let roots: Vec<usize> = in_progress
            .iter()
            .copied()
            .filter(|&i| {
                let v = &t.state.invocations[i];
                [
                    &v["setup_parent"],
                    &v["cascade_parent"],
                    &v["secret_read"]["run"],
                ]
                .iter()
                .all(|parent| {
                    !in_progress
                        .iter()
                        .any(|&p| t.state.invocations[p]["id"] == **parent)
                })
            })
            .collect();
        if roots.is_empty() {
            return Err("cyclic parts while stopping".into());
        }
        for i in roots {
            if t.state.invocations[i]["state"] != "in-progress" {
                continue;
            }
            if t.state.invocations[i].get("provider_restart").is_some() {
                // PRV-026's internal runner work is indexed here, but is not a
                // dispatched tool invocation and has no operation or waiting caller.
                let v = &t.state.invocations[i];
                t.external_intents.push(
                    json!({"kind":"cancel-setup-parts","invocation":v["id"],"subject":v["object"]}),
                );
                crate::secrets::cancel(t, &mut a, c, i)?;
                crate::secrets::recover(t, &mut a, c, i)?;
            } else {
                requests::cancel_with_allocation(t, &mut a, i, None, c)?;
            }
            if t.state.invocations[i]["state"] == "in-progress" {
                return Err("stop left an invocation in progress".into());
            }
        }
        // A retirement's stop may have asked for its provider's restart. Cut
        // that internal work, and its reads, too; no report is awaited.
    }
    let was_cut = !cut_entries.is_empty();
    let mut results = vec![];
    for i in cut_entries {
        let v = &mut t.state.invocations[i];
        if v.get("provider_restart").is_some() {
            continue;
        }
        let id = v["id"]
            .as_str()
            .ok_or("cut invocation lacks ID")?
            .to_owned();
        // Secret-read cancellation records its end without a consumer-facing reply.
        // Give every cut part the same diagnostic and waiting-reply treatment.
        let result = if let Some(reply) = t.request_replies.iter().find(|r| r.invocation == id) {
            reply.reply.clone()
        } else {
            let outcome = v["state"].as_str().ok_or("cut invocation lacks outcome")?;
            let cause = explain::outcome_cause(v, outcome, "OPS-037")?;
            let reply = requests::envelope(
                c,
                Some(outcome),
                None,
                Some(&v["id"]),
                Some(std::slice::from_ref(&cause)),
                None,
                json!([]),
                None,
            );
            v["cause"] = json!([cause]);
            t.request_replies.push(RequestReply {
                invocation: id,
                attempt: v["attempt"]
                    .as_u64()
                    .ok_or("cut invocation lacks attempt")?,
                reply: reply.clone(),
            });
            reply
        };
        let mut result = result;
        explain::first_text(&mut result, c);
        results.push(result);
    }
    ended(t, &mut a, c, was_cut)?;
    t.reply = json!({"stopped":true,"cut":results});
    Ok(())
}

// A stop held behind provider recovery has not been sent yet. Preserve that
// required intent when cancellation drops the wait, including a cascade's parts.
pub(crate) fn take_waiting_stop(t: &mut Transition, i: usize, c: &Context) -> Option<Value> {
    if !cutting(&t.state)
        || t.state.invocations[i]["retirement"]["phase"] != "stop"
        || t.state.invocations[i]["provider_wait"]["intent"]["operation"] != c.qualified("stop.v1")
    {
        return None;
    }
    t.state.invocations[i]
        .as_object_mut()?
        .remove("provider_wait")
        .map(|wait| wait["intent"].clone())
}

pub(crate) fn send_waiting_stop(t: &mut Transition, intent: Option<Value>) {
    if let Some(intent) = intent
        && !t.external_intents.iter().any(|sent| {
            sent["invocation"] == intent["invocation"] && sent["operation"] == intent["operation"]
        })
    {
        t.external_intents.push(intent);
    }
}

// Called once after an input's entire transition, so the stop record follows every
// end/part record and can never split a completion that starts its next part.
pub(crate) fn after(t: &mut Transition, evidence: &Evidence, c: &Context) -> Result<()> {
    if t.state.stop == Some(StopState::Asked) && !running(&t.state) {
        let cursor = evidence
            .ids
            .iter()
            .rposition(|id| t.state.allocated_ids.contains(id))
            .map_or(0, |n| n + 1);
        ended(t, &mut Allocation { evidence, cursor }, c, false)?;
    }
    Ok(())
}

pub(crate) fn evidence(input: &Input) -> Option<&Evidence> {
    match input {
        Input::Invoke { evidence, .. }
        | Input::Access { evidence, .. }
        | Input::RegisterProject { evidence, .. }
        | Input::Submit { evidence, .. }
        | Input::Request { evidence, .. }
        | Input::Complete { evidence, .. }
        | Input::SetupReport { evidence, .. }
        | Input::Cancel { evidence, .. }
        | Input::Respond { evidence, .. }
        | Input::SignInWaitEnded { evidence, .. }
        | Input::AdmissionAskDeadline { evidence, .. }
        | Input::BuiltinRunStopped { evidence, .. }
        | Input::BuiltinSecretProviderExited { evidence, .. }
        | Input::Recover { evidence }
        | Input::Stop { evidence, .. } => Some(evidence),
        _ => None,
    }
}

pub(crate) fn work(state: &State, sender: &Value, serving: Option<&str>) -> bool {
    state.invocations.iter().any(|v| {
        v["state"] == "in-progress"
            && (sender.is_string() && v["target"] == *sender
                || serving.is_some_and(|id| v["id"] == id))
    })
}

// This identifies a matching, nondispatching repeat, without evaluating access.
// Once admitted it goes through invoke_inner's ordinary authorization and reply path.
fn kept_repeat(
    state: &State,
    request: &access::AccessRequest,
    key: Option<&str>,
    c: &Context,
) -> Result<bool> {
    let Some(key) = key else { return Ok(false) };
    let access::Identity::Instance { id } = &request.identity else {
        return Ok(false);
    };
    if !state
        .invocations
        .iter()
        .any(|v| v["key"] == key && v["sender"] == *id && v["caller"] == *id)
    {
        return Ok(false);
    }
    let view = access::PublicationView::new(state, c);
    let consumer = json!(id);
    for path in view.paths(&consumer) {
        let path = path?;
        let local = path["local_name"]
            .as_str()
            .ok_or("Conduit lacks local name")?;
        for op in path["operations"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            let Some((_, name)) = op.rsplit_once('#') else {
                continue;
            };
            if request.tool != format!("{local}.{name}") {
                continue;
            }
            let Some(old) = state.invocations.iter().find(|v| {
                v["key"] == key
                    && v["sender"] == *id
                    && v["caller"] == *id
                    && v["target"] == path["target"]
            }) else {
                return Ok(false);
            };
            let Some(target) = view.object(&path["target"])? else {
                return Ok(false);
            };
            let (env, scope, contract) = schemas::operation(target, op, &c.core_package)?;
            let spec = &contract["spec"];
            let schema = spec
                .get("inputSchema")
                .cloned()
                .unwrap_or(json!({"type":"object"}));
            let arguments = if spec.get("changes").is_some() {
                env.fill_control_inputs(&scope, &schema, &request.arguments, &c.core_package)?
            } else {
                env.fill(&scope, &schema, &request.arguments, true, &c.core_package)?
            };
            let digest = packages::bytes_digest(
                &serde_json_canonicalizer::to_vec(&json!({"operation":op,"arguments":arguments}))
                    .map_err(|e| e.to_string())?,
            );
            if old["digest"] != digest {
                return Ok(false);
            }
            let filtered = path["requirements"]
                .as_object()
                .into_iter()
                .flat_map(|m| m.values())
                .any(|r| {
                    r["requirement"]
                        .as_str()
                        .is_some_and(|q| !q.starts_with(&format!("{}#", c.core_package)))
                });
            return Ok(old["state"] == "in-progress"
                || old["state"] == "unknown"
                    && (old["plain"] == true || spec["deduplicates"] != true)
                || old["state"] == "succeeded" && !(filtered && requests::read_only(spec)));
        }
    }
    Ok(false)
}

pub(crate) fn takes(
    state: &State,
    request: &access::AccessRequest,
    key: Option<&str>,
    c: &Context,
) -> bool {
    if state.stop.is_none() || !request.readable {
        return true;
    }
    let sender = match &request.identity {
        access::Identity::Instance { id } => json!(id),
        _ => Value::Null,
    };
    let answer = request.tool.ends_with(".answer.v1")
        && crate::pending::waited_for(state, &request.arguments["action"])
        && access::PublicationView::new(state, c)
            .paths(&sender)
            .any(|path| {
                path.is_ok_and(|path| {
                    path["local_name"]
                        .as_str()
                        .is_some_and(|local| request.tool == format!("{local}.answer.v1"))
                        && path["operations"]
                            .as_array()
                            .is_some_and(|ops| ops.contains(&json!(c.qualified("answer.v1"))))
                })
            });
    work(state, &sender, request.serving.as_deref())
        || answer
        || kept_repeat(state, request, key, c).unwrap_or(false)
}

pub(crate) fn request_route(
    state: &State,
    request: &access::AccessRequest,
    key: Option<&str>,
) -> Value {
    let consumer = access::established(state, &request.identity)
        .ok()
        .map(|o| o["id"].clone());
    let mut route = json!({"tool":request.tool,"consumer":consumer,"caller":consumer});
    if consumer.is_none() {
        route["identityStatus"] = json!(
            if matches!(request.identity, access::Identity::Unreadable) {
                "unreadable"
            } else {
                "not-established"
            }
        );
    }
    if let Some(key) = key {
        route["idempotencyKey"] = json!(key);
    }
    route
}

pub(crate) fn reject(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    route: Value,
) -> Result<()> {
    requests::reject(t, a, c, &route, "unavailable", "OPS-037", None, None)
}

// The older input forms are also request boundaries. They must not bypass the
// admission check just because they already carry a resolved route.
pub(crate) fn reject_input(t: &mut Transition, input: &Input, c: &Context) -> Result<bool> {
    if t.state.stop.is_none() {
        return Ok(false);
    }
    let (evidence, route) = match input {
        Input::Access { evidence, request } if !takes(&t.state, request, None, c) => {
            (evidence, request_route(&t.state, request, None))
        }
        Input::RegisterProject {
            evidence, request, ..
        }
        | Input::Submit {
            evidence, request, ..
        }
        | Input::Request {
            evidence, request, ..
        } if !work(&t.state, &json!(request.sender), None) => (
            evidence,
            json!({"operation":request.operation,"caller":request.caller,"consumer":request.sender}),
        ),
        _ => return Ok(false),
    };
    reject(
        t,
        &mut Allocation {
            evidence,
            cursor: 0,
        },
        c,
        route,
    )?;
    Ok(true)
}
