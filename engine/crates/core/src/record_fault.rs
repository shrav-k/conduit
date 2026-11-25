//! REC-007: describe a failed write without retaining the proposed State.
use crate::{
    Allocation, Context, Evidence, Input, RequestReply, Result, State, Transition, decision,
    explain, lifecycle, requests, secrets,
};
use serde_json::{Value, json};

pub struct RecordFault {
    evidence: Evidence,
    action: Action,
}

enum Action {
    Reject {
        route: Value,
        boundary: Boundary,
    },
    Preserve {
        reply: Value,
        records: Vec<(String, Value)>,
    },
    Continue {
        invocation: String,
        attempt: Option<u64>,
        reason: Option<(String, Value)>,
        reads: Vec<(Value, &'static str)>,
        cancellations: Vec<Value>,
    },
    Renamed {
        reply: Value,
        replies: Vec<RequestReply>,
        intents: Vec<Value>,
        reason: Option<Value>,
    },
}

#[derive(Clone, Copy)]
enum Boundary {
    Tool,
    Registration,
    Access,
}

impl RecordFault {
    pub fn covers(kind: &str) -> bool {
        matches!(
            kind,
            "invoke"
                | "register-project"
                | "request"
                | "submit"
                | "access"
                | "complete"
                | "setup-report"
                | "respond"
                | "sign-in-wait-ended"
                | "admission-ask-deadline"
                | "cancel"
        )
    }

    pub fn from_trial(input: Input, trial: &Transition, renamed: bool) -> Result<Self> {
        let reason = trial.validated_reason.clone();
        let boundary = if matches!(&input, Input::Invoke { .. }) {
            Boundary::Tool
        } else {
            Boundary::Access
        };
        let (evidence, action) = match input {
            Input::Invoke {
                evidence, request, ..
            }
            | Input::Access { evidence, request } => {
                let access_records = trial.records.iter().filter(|r| {
                    matches!(
                        r["type"].as_str(),
                        Some("access-decision" | "request-ended")
                    )
                });
                let access = trial
                    .records
                    .iter()
                    .find(|r| r["type"] == "access-decision");
                let action = if !request.readable
                    || access.is_some_and(|r| r["data"]["ruleId"] == "ACC-037")
                {
                    Action::Preserve {
                        reply: trial.reply.clone(),
                        records: access_records
                            .map(|r| (r["type"].as_str().unwrap().into(), r["data"].clone()))
                            .collect(),
                    }
                } else {
                    let mut route = access.map(|r| r["data"].clone()).unwrap_or(json!({}));
                    route["tool"] = json!(request.tool);
                    Action::Reject { route, boundary }
                };
                (evidence, action)
            }
            Input::RegisterProject {
                evidence, request, ..
            }
            | Input::Submit {
                evidence, request, ..
            } => (
                evidence,
                Action::Reject {
                    route: explain::ids(&request, json!({})),
                    boundary: Boundary::Registration,
                },
            ),
            Input::Request {
                evidence,
                request,
                object,
            } => (
                evidence,
                Action::Reject {
                    route: explain::ids(&request, json!({"object":object})),
                    boundary: Boundary::Access,
                },
            ),
            Input::Complete {
                evidence,
                invocation,
                attempt,
                ..
            }
            | Input::Respond {
                evidence,
                invocation,
                attempt,
                ..
            }
            | Input::AdmissionAskDeadline {
                evidence,
                invocation,
                attempt,
            }
            | Input::SignInWaitEnded {
                evidence,
                invocation,
                attempt,
            } => (
                evidence,
                continuation(trial, invocation, Some(attempt), reason.clone()),
            ),
            Input::SetupReport {
                evidence,
                invocation,
                ..
            }
            | Input::Cancel {
                evidence,
                invocation,
                ..
            } => (evidence, continuation(trial, invocation, None, None)),
            _ => return Err("input has no fail-closed transition".into()),
        };
        let action = if renamed {
            let reason = reason.and_then(|(rule, restriction)| {
                // A normal IFC-027 reply does not record an invocation-ended event.
                // Its already-kept request-ended record stays as written.
                let (id, attempt) = match &action {
                    Action::Continue {
                        invocation,
                        attempt,
                        ..
                    } => (invocation, attempt),
                    _ => return None,
                };
                let mut data = trial.records.iter().rev().find(|r| {
                    let data = &r["data"];
                    r["type"] == "request-ended"
                        && data["restriction"] == restriction
                        && data["reason"] == rule
                })?["data"]
                    .clone();
                data["invocation"] = json!(id);
                data["attempt"] = json!(attempt);
                Some(data)
            });
            Action::Renamed {
                reply: trial.reply.clone(),
                replies: trial.request_replies.clone(),
                intents: trial.external_intents.clone(),
                reason,
            }
        } else {
            action
        };
        Ok(Self { evidence, action })
    }

    pub fn step(&self, state: State, c: &Context) -> Result<Transition> {
        let cursor = self
            .evidence
            .ids
            .iter()
            .rposition(|id| state.allocated_ids.contains(id))
            .map_or(0, |i| i + 1);
        let mut a = Allocation {
            evidence: &self.evidence,
            cursor,
        };
        let mut t = Transition {
            state,
            decisions: vec![decision(
                "fail-closed-for-record",
                &["REC-007"],
                &["DL10"],
                json!({}),
                vec![],
            )],
            records: vec![],
            external_intents: vec![],
            reply: Value::Null,
            request_replies: vec![],
            input_required: vec![],
            validated_reason: None,
        };
        let mut propagate_cause = false;
        match &self.action {
            Action::Reject { route, boundary } => reject(&mut t, &mut a, c, route, *boundary)?,
            Action::Preserve { reply, records } => {
                for (kind, data) in records {
                    requests::record(&mut t, &mut a, c, kind, data.clone())?;
                }
                t.reply = reply.clone();
            }
            Action::Continue {
                invocation,
                attempt,
                reason,
                reads,
                cancellations,
            } => {
                t.external_intents.extend(cancellations.iter().cloned());
                for (read, rule) in reads {
                    let i = if let Some(i) = index(&t.state, &read["id"]) {
                        if t.state.invocations[i]["state"] != "in-progress"
                            || t.state.invocations[i]["secret_read"]["phase"] == "dispatched"
                        {
                            continue;
                        }
                        t.state.invocations[i]["conduit"] = read["conduit"].clone();
                        i
                    } else {
                        let mut read = read.clone();
                        read["id"] = json!(a.next(&mut t.state)?);
                        read["secret_read"]["phase"] = json!("deciding");
                        t.state.invocations.push(read);
                        t.state.invocations.len() - 1
                    };
                    secrets::unrecorded_read(&mut t, &mut a, c, i, rule)?;
                }
                if let Some(i) = index(&t.state, &json!(invocation))
                    && t.state.invocations[i]["state"] == "in-progress"
                    && attempt.is_none_or(|attempt| t.state.invocations[i]["attempt"] == attempt)
                {
                    if t.state.invocations[i]["secret_read"].is_object() {
                        secrets::record_fault(&mut t, &mut a, c, i, reason.as_ref(), false)?;
                    } else if let Some((reason, restriction)) = reason {
                        requests::record_fault_reason(&mut t, &mut a, c, i, reason, restriction)?;
                    } else if t.state.invocations[i]["discovery"] == "pending" {
                        requests::record_fault_unavailable(
                            &mut t,
                            &mut a,
                            c,
                            i,
                            requests::UnsentDispatch::DecisionKept,
                        )?;
                    } else {
                        propagate_cause = true;
                        lifecycle::record_fault_end(&mut t, &mut a, c, i)?;
                    }
                }
                if t.reply.is_null() {
                    t.reply = json!({"ignored":true});
                }
            }
            Action::Renamed {
                reply,
                replies,
                intents,
                reason,
            } => {
                // DL2: this completion's records and result can be read back.
                // At a restart IMP-007 will assess whatever survived.
                t.reply = reply.clone();
                t.request_replies.clone_from(replies);
                if let Some(data) = reason {
                    requests::record(&mut t, &mut a, c, "invocation-ended", data.clone())?;
                }
                let root_reply = t.reply.clone();
                for intent in intents {
                    suppress(&mut t, &mut a, c, intent, true)?;
                }
                t.reply = root_reply;
            }
        }
        // Child endings can advance a parent; refuse any work the new step proposes.
        let primary = t.reply.clone();
        drain_proposed_dispatches(&mut t, &mut a, c)?;
        crate::admission_ask::settle_record_fault(&mut t, &mut a, c)?;
        if primary["pending"] == true {
            if !t
                .request_replies
                .iter()
                .any(|r| primary["invocation"] == r.invocation)
                && let Some(i) = index(&t.state, &primary["invocation"])
                && t.state.invocations[i]["state"] == "in-progress"
            {
                lifecycle::record_fault_end(&mut t, &mut a, c, i)?;
                drain_proposed_dispatches(&mut t, &mut a, c)?;
                crate::admission_ask::settle_record_fault(&mut t, &mut a, c)?;
            }
            t.reply = t
                .request_replies
                .iter()
                .rev()
                .find(|r| primary["invocation"] == r.invocation)
                .map(|r| r.reply.clone())
                .ok_or("failed record left a request pending")?;
        } else {
            t.reply = primary;
        }
        if propagate_cause {
            for reply in &mut t.request_replies {
                add_parent_cause(&t.state, &mut reply.reply, c)?;
            }
            add_parent_cause(&t.state, &mut t.reply, c)?;
        }
        t.input_required.clear();
        explain::first_text(&mut t.reply, c);
        for reply in &mut t.request_replies {
            explain::first_text(&mut reply.reply, c);
        }
        Ok(t)
    }
}

fn index(state: &State, id: &Value) -> Option<usize> {
    state
        .invocations
        .iter()
        .position(|entry| entry["id"] == *id)
}

fn is_cancellation(intent: &Value) -> bool {
    matches!(
        intent["kind"].as_str(),
        Some("cancel" | "cancel-setup-parts")
    )
}

fn continuation(
    trial: &Transition,
    invocation: String,
    attempt: Option<u64>,
    reason: Option<(String, Value)>,
) -> Action {
    if trial.reply["ignored"] == true
        || trial.reply["waitEnded"] == false
        || trial.reply.get("error").is_some()
    {
        return Action::Preserve {
            reply: trial.reply.clone(),
            records: vec![],
        };
    }
    let reads = trial
        .state
        .invocations
        .iter()
        .filter(|v| v["secret_read"].is_object() && v["id"] != invocation)
        .filter_map(|read| {
            let decision = trial.records.iter().find(|r| {
                r["type"] == "access-decision" && r["data"]["invocation"] == read["id"]
            })?;
            let would_dispatch = trial.external_intents.iter().any(|intent| {
                intent["invocation"] == read["id"] && intent["call"]["method"] == "tools/call"
            });
            if would_dispatch {
                Some((read.clone(), "SEC-016"))
            } else if decision["data"]["dispatched"] == false {
                Some((read.clone(), "REC-007"))
            } else {
                None
            }
        })
        .collect();
    Action::Continue {
        invocation,
        attempt,
        reason,
        reads,
        cancellations: trial
            .external_intents
            .iter()
            .filter(|v| is_cancellation(v))
            .cloned()
            .collect(),
    }
}

fn reject(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    route: &Value,
    boundary: Boundary,
) -> Result<()> {
    let mut route = route.clone();
    for field in [
        "invocation",
        "attempt",
        "reason",
        "ruleId",
        "permitted",
        "dispatched",
        "refusalTier",
        "restriction",
    ] {
        route.as_object_mut().unwrap().remove(field);
    }
    let mut access = route.clone();
    access["permitted"] = json!(false);
    access["dispatched"] = json!(false);
    access["reason"] = json!("unavailable");
    access["ruleId"] = json!("REC-007");
    requests::record(t, a, c, "access-decision", access)?;
    requests::reject(t, a, c, &route, "unavailable", "REC-007", None, None)?;
    let diagnostics = &t.reply["structuredContent"]["diagnostics"];
    match boundary {
        Boundary::Tool => {}
        Boundary::Registration => {
            t.reply = json!({"accepted":false,"reason":"unavailable","diagnostics":diagnostics})
        }
        Boundary::Access => t.reply = json!({"reason":"unavailable","diagnostics":diagnostics}),
    }
    Ok(())
}

fn suppress(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    intent: &Value,
    renamed: bool,
) -> Result<()> {
    if is_cancellation(intent) {
        t.external_intents.push(intent.clone());
    } else if let Some(i) = index(&t.state, &intent["invocation"])
        && t.state.invocations[i]["state"] == "in-progress"
    {
        match intent["kind"].as_str() {
            Some("dispatch") if t.state.invocations[i]["secret_read"].is_object() => {
                secrets::record_fault(t, a, c, i, None, renamed)?;
            }
            Some("dispatch") => {
                let stage = if renamed {
                    requests::UnsentDispatch::ReadBack
                } else {
                    requests::UnsentDispatch::Proposed
                };
                requests::record_fault_unavailable(t, a, c, i, stage)?;
            }
            _ => lifecycle::record_fault_end(t, a, c, i)?,
        }
    }
    Ok(())
}

fn drain_proposed_dispatches(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
) -> Result<()> {
    while !t.external_intents.iter().all(is_cancellation) {
        let intents = std::mem::take(&mut t.external_intents);
        for intent in intents {
            suppress(t, a, c, &intent, false)?;
        }
    }
    Ok(())
}

fn add_parent_cause(state: &State, reply: &mut Value, c: &Context) -> Result<()> {
    if reply["_meta"][format!("{}outcome", c.meta_prefix)] == "unknown"
        && reply["structuredContent"]["diagnostics"][0]["ruleId"] != "REC-007"
        && let Some(i) = index(
            state,
            &reply["_meta"][format!("{}invocation", c.meta_prefix)],
        )
    {
        reply["structuredContent"]["diagnostics"] = json!([explain::outcome_cause(
            &state.invocations[i],
            "unknown",
            "REC-007"
        )?]);
    }
    Ok(())
}
