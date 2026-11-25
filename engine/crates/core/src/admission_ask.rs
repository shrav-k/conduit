//! PND-016: one engine-owned ask per login, shared by its waiting requests.
use crate::{
    Allocation, Context, Evidence, Result, State, Transition, access, admission, decision,
    inventory, limits, login_resource, registration, requests, schemas,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeSet;

/// DL2: stored only while the ask is in progress; the invocation owns its caller and cause.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ask {
    pub invocation: String,
    pub started_at: String,
    waiters: Vec<Waiter>,
    #[serde(default)]
    pub levers_changed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    end_cause: Option<String>,
}

/// PND-016's fixed period after an ask ends without a report.
/// Kept separately so admission_asks contains only asks still in progress.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NoReport {
    invocation: Option<String>,
    cause: String,
    ended_at: String,
}

impl NoReport {
    fn until(&self) -> Option<String> {
        let (ended, _) = limits::instant(&self.ended_at)?;
        let until = ended
            .checked_add_signed(chrono::Duration::seconds(60))?
            .with_timezone(&chrono::Utc);
        let fraction = self
            .ended_at
            .split_once('.')
            .and_then(|(_, suffix)| suffix.split(|ch: char| !ch.is_ascii_digit()).next())
            .unwrap_or_default();
        let fraction = if fraction.is_empty() {
            String::new()
        } else {
            format!(".{fraction}")
        };
        Some(format!("{}{fraction}Z", until.format("%Y-%m-%dT%H:%M:%S")))
    }

    pub(crate) fn status(&self, visible: bool) -> Value {
        json!({"invocation":if visible { self.invocation.as_deref() } else { None },
            "cause":self.cause,"until":self.until()})
    }
}

pub(crate) fn no_report<'a>(state: &'a State, login: &str, time: &str) -> Option<&'a NoReport> {
    state.admission_no_reports.get(login).filter(|period| {
        period
            .until()
            .as_deref()
            .and_then(limits::instant)
            .zip(limits::instant(time))
            .is_some_and(|(until, now)| now < until)
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Waiter {
    invocation: String,
    attempt: u64,
}

/// Return true only when this prepared request must wait, before any dispatch record or intent.
pub(crate) fn wait(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    route: &Value,
) -> Result<bool> {
    let v = t.state.invocations[i].clone();
    if v["contract"]["spec"]["startsModelWork"] != true {
        return Ok(false);
    }
    let Some(login) = login_resource::resource(&t.state, &v["held_target"]).cloned() else {
        return Ok(false);
    };
    let ty = inventory::instance_type(&login, c)?;
    let Some(place) = ty["spec"]["admission"]["levers"].as_str() else {
        return Ok(false);
    };
    let login_id = login["id"].as_str().ok_or("login has no ID")?;
    if no_report(&t.state, login_id, &a.evidence.time).is_some()
        || admission::current(&t.state, login_id, &a.evidence.time).is_some()
    {
        return decide_report(t, a, c, i, login_id).map(|dispatch| !dispatch);
    }
    t.state.admission_no_reports.remove(login_id);
    let ask = if let Some(ask) = t.state.admission_asks.get(login_id) {
        ask.invocation.clone()
    } else {
        let target = registration::publication_object(&login, c)?;
        let op = c.qualified("admission-hook.v1");
        let (_, _, contract) = schemas::operation(&target, &op, &c.core_package)?;
        let implementation = target["resolved_implementations"][&op].clone();
        let id = a.next(&mut t.state)?;
        let arguments = json!({
            "levers":login["config"].pointer(place),
            "limits":limits::status(&t.state, &login, &a.evidence.time, c),
            "previous":admission::newest(&t.state, login_id)
        });
        // This is neither a request nor a part. In particular no sender, serving,
        // Conduit, key or inherited filter can grant this invocation authority.
        let ask = json!({"id":id,"hook":true,"caller":v["caller"],"causedBy":v["id"],
            "target":login["id"],"operation":op,"arguments":arguments,"filters":[],
            "state":"in-progress","plain":false,"contract":contract,"held_target":target,
            "implementation":implementation,"needs_result":false,"attempt":1,
            "config":login["config"],"revision":login["revision"],"discovery":"pending"});
        t.external_intents.push(requests::discovery_intent(&ask, c));
        t.state.invocations.push(ask);
        t.state.admission_asks.insert(
            login_id.into(),
            Ask {
                invocation: id.clone(),
                started_at: a.evidence.time.clone(),
                waiters: vec![],
                levers_changed: false,
                end_cause: None,
            },
        );
        t.decisions.push(decision(
            "start-admission-ask",
            &["PND-016", "IFC-054"],
            &["DL2", "DL10"],
            json!({"login":login_id,"invocation":id,"causedBy":v["id"]}),
            vec![],
        ));
        id
    };
    t.state
        .admission_asks
        .get_mut(login_id)
        .unwrap()
        .waiters
        .push(Waiter {
            invocation: v["id"].as_str().ok_or("waiting request has no ID")?.into(),
            attempt: v["attempt"]
                .as_u64()
                .ok_or("waiting request has no attempt")?,
        });
    t.decisions.push(decision(
        "admission-wait",
        &["PND-016"],
        &[],
        json!({"invocation":v["id"],"ask":ask,"login":login_id}),
        vec![],
    ));
    t.state.invocations[i]["admission_wait"] = json!(ask);
    t.state.invocations[i]["admission_tool"] = route["tool"].clone();
    t.reply = json!({"pending":true,"invocation":v["id"],"attempt":v["attempt"],"hook":ask});
    Ok(true)
}

/// The hold's order changes this one decision point. For now a refusal, a spent
/// single-use report and no current report all allow dispatch, without another ask.
fn decide_report(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    _c: &Context,
    _i: usize,
    login: &str,
) -> Result<bool> {
    let time = &a.evidence.time;
    if no_report(&t.state, login, time).is_none()
        && admission::current(&t.state, login, time)
            .is_some_and(|report| report["admit"] == true && report["singleUse"] == true)
    {
        admission::end(
            &mut t.state.admission_reports,
            login,
            time,
            admission::End::SingleUse,
            None,
        );
    }
    // PND-016's report/no-report hold is the next engine order.
    Ok(true)
}

/// Called after completion has kept the report. Synthetic entries used by order
/// 317 are not in admission_asks and have no effect on waiting requests.
pub(crate) fn settle(t: &mut Transition, evidence: &Evidence, c: &Context) -> Result<()> {
    // complete_call has its own allocator. Continue after its consumed prefix.
    let cursor = evidence
        .ids
        .iter()
        .take_while(|id| t.state.allocated_ids.contains(*id))
        .count();
    let mut a = Allocation { evidence, cursor };
    settle_ended(t, &mut a, c, false)
}

/// REC-007 settles only after its child endings and dispatch suppression are done,
/// with that transition's allocator. A selected dispatch still fails closed.
pub(crate) fn settle_record_fault(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
) -> Result<()> {
    settle_ended(t, a, c, true)
}

fn settle_ended(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    record_fault: bool,
) -> Result<()> {
    let ended: Vec<_> = t
        .state
        .admission_asks
        .iter()
        .filter(|(_, ask)| {
            !t.state
                .invocations
                .iter()
                .any(|v| v["id"] == ask.invocation && v["state"] == "in-progress")
        })
        .map(|(login, _)| login.clone())
        .collect();
    let reply = t.reply.clone();
    for login in ended {
        let ask = t.state.admission_asks.remove(&login).unwrap();
        let kept = admission::kept_by(&t.state, &login, &ask.invocation);
        if record_fault || !kept {
            let invocation = t
                .state
                .invocations
                .iter()
                .find(|v| v["id"] == ask.invocation);
            let cause = ask.end_cause.as_deref().unwrap_or_else(|| {
                if invocation.is_none_or(|v| v["discovery"] == "pending") {
                    "not-dispatched"
                } else if invocation.is_some_and(|v| v["state"] == "failed") {
                    "failed"
                } else {
                    "unknown"
                }
            });
            t.state.admission_no_reports.insert(
                login.clone(),
                NoReport {
                    invocation: (cause != "not-dispatched").then(|| ask.invocation.clone()),
                    cause: cause.into(),
                    ended_at: a.evidence.time.clone(),
                },
            );
        }
        for waiter in ask.waiters {
            let Some(i) = t.state.invocations.iter().position(|v| {
                v["id"] == waiter.invocation
                    && v["attempt"] == waiter.attempt
                    && v["state"] == "in-progress"
                    && v["admission_wait"] == ask.invocation
            }) else {
                continue;
            };
            let v = t.state.invocations[i].clone();
            let mut route = json!({"target":v["target"],"caller":v["caller"],"consumer":v["sender"],
                "conduit":v["conduit"],"operation":v["operation"],"serving":v["serving"],
                "invocation":v["id"],"attempt":v["attempt"],"idempotencyKey":v["key"],
                "tool":v["admission_tool"],"dispatched":false});
            if requests::hold_before_dispatch(
                t,
                a,
                c,
                &v["held_target"],
                &v["contract"],
                &mut route,
                v["login_request_record"]
                    .as_str()
                    .ok_or("waiting request has no access record")?,
            )? {
                requests::end_waiting(t, i)?;
                continue;
            }
            if kept && ask.levers_changed && !record_fault {
                // Keep the request's identity and position; the first remaining
                // waiter causes the fresh ask and the rest join it.
                wait(t, a, c, i, &route)?;
                continue;
            }
            if let Some(condition) = dispatch_condition(&t.state, &v, c) {
                let message = format!(
                    "{condition} The call was not dispatched, started no model work and is not queued; a call sent again is decided afresh."
                );
                requests::reject_waiting(t, a, c, i, &message)?;
                continue;
            }
            if decide_report(t, a, c, i, &login)? {
                if record_fault {
                    // Its initial access decision was already kept. Suppress the
                    // selected dispatch without fabricating a second access decision.
                    requests::record_fault_unavailable(
                        t,
                        a,
                        c,
                        i,
                        requests::UnsentDispatch::DecisionKept,
                    )?;
                } else {
                    requests::dispatch_waiting(t, a, c, i)?;
                }
            }
        }
    }
    t.reply = reply;
    Ok(())
}

/// Read the original IDs, never reselect a tool by name. Each chain link must
/// still list that link's operation. This checks dispatch, making no access decision.
fn dispatch_condition(state: &State, waiting: &Value, c: &Context) -> Option<String> {
    let during = "while the call waited for its login's admission report";
    let mut cursor = waiting;
    let mut seen = BTreeSet::new();
    loop {
        if !seen.insert(cursor["id"].clone().to_string()) {
            return Some(format!(
                "The invocation chain was cyclic and could not be checked {during}."
            ));
        }
        if cursor["hook"] != true {
            let Some(path) = access::object_by_id(state, &cursor["conduit"]) else {
                if let Some(stone) = state
                    .tombstones
                    .values()
                    .find(|stone| stone["object"]["id"] == cursor["conduit"])
                {
                    let object = &stone["object"];
                    let label = if access::may_inspect(state, &waiting["caller"], object, c)
                        .unwrap_or(false)
                    {
                        login_resource::label(object)
                    } else {
                        "A Conduit in the dispatch chain".into()
                    };
                    return Some(format!("{label} was closed {during}."));
                }
                return Some(format!(
                    "A Conduit in the dispatch chain could not be found {during}."
                ));
            };
            let label = if access::may_inspect(state, &waiting["caller"], path, c).unwrap_or(false)
            {
                login_resource::label(path)
            } else {
                "A Conduit in the dispatch chain".into()
            };
            if path["kind"] != "Conduit" || !path["open"].is_boolean() || !path["state"].is_string()
            {
                return Some(format!(
                    "{label}'s open state could not be checked {during}."
                ));
            }
            if !access::open(path) {
                return Some(format!("{label} was closed {during}."));
            }
            let Some(operations) = path["operations"]
                .as_array()
                .filter(|ops| ops.iter().all(Value::is_string))
            else {
                return Some(format!(
                    "{label}'s operations could not be checked {during}."
                ));
            };
            if !operations.contains(&cursor["operation"]) {
                return Some(format!("{label} no longer listed the operation {during}."));
            }
            let Some(consumer) = access::object_by_id(state, &path["consumer"]) else {
                return Some(format!("{label}'s consumer could not be checked {during}."));
            };
            if !consumer["state"].is_string() {
                return Some(format!(
                    "{label}'s consumer's state could not be checked {during}."
                ));
            }
            if consumer["state"] == "retired" {
                return Some(format!("{label} had a retired consumer {during}."));
            }
        }
        let serving = cursor.get("serving").filter(|id| !id.is_null())?;
        let Some(parent) = state.invocations.iter().find(|v| v["id"] == *serving) else {
            return Some(format!(
                "The invocation served could not be found {during}."
            ));
        };
        if parent["state"] != "in-progress" || parent.get("admission_wait").is_some() {
            let label = if access::object_by_id(state, &parent["target"]).is_some_and(|target| {
                access::may_inspect(state, &waiting["caller"], target, c).unwrap_or(false)
            }) {
                format!("Invocation {}", parent["id"].as_str().unwrap_or_default())
            } else {
                "The invocation served".into()
            };
            let cause = if !parent["state"].is_string() {
                "had a state that could not be checked"
            } else if parent["state"] != "in-progress" {
                "was no longer in progress"
            } else {
                "was still waiting and had not been dispatched"
            };
            return Some(format!("{label} {cause} {during}."));
        }
        cursor = parent;
    }
}

/// Recovery has already ended every waiter; settle the asks without dispatching.
pub(crate) fn recovered(t: &mut Transition, a: &mut Allocation<'_>, c: &Context) -> Result<()> {
    for ask in t.state.admission_asks.values_mut() {
        ask.end_cause = Some("unknown".into());
    }
    settle_ended(t, a, c, false)
}

pub(crate) fn deadline(
    t: &mut Transition,
    evidence: &Evidence,
    invocation: &str,
    attempt: u64,
    c: &Context,
) -> Result<()> {
    let login = t
        .state
        .admission_asks
        .iter()
        .find(|(_, ask)| ask.invocation == invocation)
        .map(|(login, _)| login.clone());
    let index = t.state.invocations.iter().position(|v| {
        v["id"] == invocation && v["attempt"] == attempt && v["state"] == "in-progress"
    });
    let ended = login.is_some() && index.is_some();
    if let (Some(login), Some(i)) = (login, index) {
        t.state.admission_asks.get_mut(&login).unwrap().end_cause = Some("deadline".into());
        let mut a = Allocation {
            evidence,
            cursor: 0,
        };
        t.external_intents
            .push(json!({"kind":"cancel","invocation":invocation,"attempt":attempt}));
        let elapsed = limits::instant(&evidence.time)
            .zip(limits::instant(&t.state.admission_asks[&login].started_at))
            .map(|((end, _), (start, _))| {
                format!(
                    " ({} seconds after it started)",
                    (end - start).num_milliseconds() as f64 / 1000.0
                )
            })
            .unwrap_or_default();
        let cause = crate::explain::outcome_diagnostic(
            &t.state.invocations[i],
            "unknown",
            "PND-016",
            &format!(
                "The login's admission ask gave no report before its DL21 deadline{elapsed} and ended unknown."
            ),
        )?;
        requests::finish(
            t,
            &mut a,
            c,
            i,
            "unknown",
            json!({}),
            json!([]),
            Some(vec![cause]),
        )?;
        settle_ended(t, &mut a, c, false)?;
    }
    t.reply = json!({"askEnded":ended});
    t.decisions.push(decision(
        "admission-ask-deadline",
        &["PND-016"],
        &["DL21"],
        json!({"invocation":invocation,"attempt":attempt,"ended":ended}),
        vec![],
    ));
    Ok(())
}
