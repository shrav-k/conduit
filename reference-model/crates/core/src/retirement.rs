//! Explicit retirement/removal plans. Shutdown work is reported by the boundary.
use crate::{
    Allocation, Context, Result, State, Transition, access, decision, lifecycle, requests,
};
use serde_json::{Value, json};

pub(crate) fn preflight(
    s: &State,
    o: &Value,
    unregister: bool,
) -> Option<(&'static str, &'static str)> {
    if o.get("spec").is_some() {
        return None;
    }
    if unregister && o["state"] != "retired" {
        return Some(("conflict", "LIF-017"));
    }
    if o["kind"] == "Pool"
        && !unregister
        && s.objects
            .iter()
            .any(|x| x["kind"] == "Resource" && access::id(&x["owner"]) == &o["id"])
    {
        return Some(("conflict", "LIF-020"));
    }
    if s.objects
        .iter()
        .any(|x| x["id"] != o["id"] && access::id(&x["placement"]) == &o["id"])
    {
        return Some(("conflict", "PLC-011"));
    }
    if !unregister
        && s.objects
            .iter()
            .any(|x| access::id(&x["runtime"]) == &o["id"])
    {
        return Some(("conflict", "PRV-020"));
    }
    None
}
pub(crate) fn blocking_diagnostic(s: &State, route: &Value, rule: &str) -> Option<String> {
    let reference = route["arguments"]
        .get("instance")
        .or_else(|| route["arguments"].get("object"))?;
    let object = access::object(s, reference)?;
    let field = match rule {
        "LIF-020" => "owner",
        "PLC-011" => "placement",
        "PRV-020" => "runtime",
        _ => return None,
    };
    let blockers: Vec<_> = s
        .objects
        .iter()
        .filter(|x| x["id"] != object["id"] && access::id(&x[field]) == &object["id"])
        .map(|x| {
            format!(
                "{}/{} ({})",
                x["kind"].as_str().unwrap_or(""),
                x["name"].as_str().unwrap_or(""),
                x["id"].as_str().unwrap_or("")
            )
        })
        .collect();
    Some(format!(
        "The request is blocked by {}.",
        blockers.join(", ")
    ))
}
fn run(o: &Value) -> Option<Value> {
    o["resolved_implementations"]
        .as_object()
        .into_iter()
        .flat_map(|m| m.values())
        .find(|x| x["item"].get("command").is_some() || x["item"].get("package").is_some())
        .cloned()
}
fn event(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    id: &Value,
    action: &str,
    extra: Value,
) -> Result<()> {
    let v = &t.state.invocations[i];
    let mut data = json!({"target":id,"invocation":v["id"],"caller":v["caller"],"consumer":v["sender"],"conduit":v["conduit"],"operation":v["operation"],"action":action});
    data.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    requests::record(t, a, c, "object-change", data)
}
fn close_ends(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    id: &Value,
    consumer: bool,
) -> Result<()> {
    let paths: Vec<_> = t
        .state
        .objects
        .iter()
        .filter(|p| {
            p["kind"] == "Conduit"
                && p.get("spec").is_none()
                && (access::id(&p["target"]) == id || consumer && access::id(&p["consumer"]) == id)
        })
        .cloned()
        .collect();
    for p in paths {
        if let Some(o) = t
            .state
            .objects
            .iter_mut()
            .find(|o| o["id"] == *access::id(&p["consumer"]))
        {
            o["status"]["closedConduits"][p["id"].as_str().unwrap()] =
                json!({"name":p["name"],"capability":p["capability"]});
        }
        event(
            t,
            a,
            c,
            i,
            &p["id"],
            "closed",
            json!({"state":"retired","shutdown":"confirmed"}),
        )?;
        t.state.objects.retain(|o| o["id"] != p["id"]);
        event(t, a, c, i, &p["id"], "removed", json!({}))?;
    }
    Ok(())
}
pub(crate) fn remove(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    id: &Value,
) -> Result<()> {
    close_ends(t, a, c, i, id, true)?;
    t.state.objects.retain(|o| o["id"] != *id);
    event(t, a, c, i, id, "removed", json!({}))
}
pub(crate) fn execute(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    o: &Value,
) -> Result<()> {
    let unregister = t.state.invocations[i]["operation"] == c.qualified("unregister.v1");
    t.decisions.push(decision(
        if unregister {
            "unregister-object"
        } else {
            "retire-instance"
        },
        if unregister {
            &[
                "LIF-012", "LIF-017", "LIF-019", "LIF-036", "REV-002", "CON-008",
            ]
        } else {
            &[
                "LIF-005", "LIF-024", "LIF-025", "LIF-026", "LIF-035", "TPL-009", "TPL-018",
                "PRV-008", "CON-008",
            ]
        },
        &[],
        json!({}),
        vec![o["id"].as_str().unwrap().into()],
    ));
    t.state.invocations[i]["retirement"] =
        json!({"phase":"begin","unregister":unregister,"parts":[]});
    if unregister {
        if o.get("spec").is_some() {
            return settle(t, a, c, i, o);
        }
        return stop(t, a, c, i, o);
    }
    let v = t.state.invocations[i].clone();
    let lost = access::object(&t.state, &o["placement"]).is_some_and(|n| n["state"] == "retired");
    let shutdown = if lost { "unknown" } else { "pending" };
    t.state.invocations[i]["retirement_of"] = o["id"].clone();
    let object = t
        .state
        .objects
        .iter_mut()
        .find(|x| x["id"] == o["id"])
        .unwrap();
    object["state"] = json!("retired");
    object["shutdown"] = json!(shutdown);
    object["retirement"] = v["id"].clone();
    event(
        t,
        a,
        c,
        i,
        &o["id"],
        "retired",
        json!({"state":"retired","shutdown":shutdown}),
    )?;
    close_ends(t, a, c, i, &o["id"], false)?;
    let ever_setup = t
        .state
        .invocations
        .iter()
        .any(|v| v["operation"] == c.qualified("set-up.v1") && v["object"] == o["id"]);
    if !ever_setup {
        return settle(t, a, c, i, o);
    }
    if o["resolved_implementations"]
        .get(c.qualified("retire-hook.v1"))
        .is_some()
    {
        // TPL-018: only prepare an implementation for a new hook attempt.
        // Reuse the retained outcome through the same path as hook dispatch.
        if !requests::hook_needs_dispatch(&t.state, o, &c.qualified("retire-hook.v1")) {
            return hook(t, a, c, i, o);
        }
        if lost {
            add(t, i, "hook", "unavailable");
            return stop(t, a, c, i, o);
        }
        if let Some(item) = run(o) {
            t.state.invocations[i]["setup"] = json!({"item":item});
            t.state.invocations[i]["retirement"]["phase"] = json!("inspect-run");
            return lifecycle::emit(t, a, c, i, "inspect-run.v1", json!({}));
        }
        let implementation = &o["resolved_implementations"][c.qualified("retire-hook.v1")];
        if implementation["item"]["remote"].get("headers").is_some() {
            t.state.invocations[i]["retirement"]["phase"] = json!("connect");
            t.external_intents.push(json!({"kind":"connect","part":"retire-hook","invocation":v["id"],"subject":o["id"],"implementation":implementation}));
            t.reply = json!({"pending":true,"invocation":v["id"]});
            return Ok(());
        }
        return hook(t, a, c, i, o);
    }
    stop(t, a, c, i, o)
}
fn hook(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    o: &Value,
) -> Result<()> {
    t.state.invocations[i]["retirement"]["phase"] = json!("hook");
    requests::hook(
        t,
        a,
        c,
        i,
        o,
        &o["resolved_implementations"][c.qualified("retire-hook.v1")],
        "retire-hook.v1",
    )
}
fn add(t: &mut Transition, i: usize, name: &str, outcome: &str) {
    t.state.invocations[i]["retirement"]["parts"]
        .as_array_mut()
        .unwrap()
        .push(json!({"name":name,"outcome":outcome,"effect":name != "hook-connect"}));
}
fn stop(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    o: &Value,
) -> Result<()> {
    if run(o).is_some() {
        t.state.invocations[i]["retirement"]["phase"] = json!("stop");
        return lifecycle::emit(t, a, c, i, "stop.v1", json!({}));
    }
    settle(t, a, c, i, o)
}
fn settle(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    o: &Value,
) -> Result<()> {
    let v = t.state.invocations[i].clone();
    let parts = v["retirement"]["parts"].as_array().unwrap();
    let lost = access::object(&t.state, &o["placement"]).is_some_and(|n| n["state"] == "retired");
    let outcome = if v["retirement"]["unregister"] == true
        || parts.iter().all(|p| p["outcome"] == "succeeded")
    {
        "succeeded"
    } else if parts
        .iter()
        .any(|p| p["effect"] == true && (p["outcome"] == "succeeded" || p["outcome"] == "unknown"))
    {
        "unknown"
    } else {
        "failed"
    };
    if outcome == "succeeded" {
        remove(t, a, c, i, &o["id"])?;
    } else if let Some(object) = t.state.objects.iter_mut().find(|x| x["id"] == o["id"]) {
        object["shutdown"] = json!(if outcome == "unknown" || lost {
            "unknown"
        } else {
            "pending"
        });
    }
    event(
        t,
        a,
        c,
        i,
        &o["id"],
        "retirement-ended",
        json!({"outcome":outcome,"parts":parts}),
    )?;
    t.state.invocations[i]["retirement"]["phase"] = json!("ended");
    requests::finish(t, a, c, i, outcome, json!({}), json!([]))
}
pub(crate) fn hook_ended(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    outcome: &str,
) -> Result<()> {
    add(t, i, "hook", outcome);
    let o = access::object(&t.state, &t.state.invocations[i]["object"])
        .ok_or("retirement object disappeared")?
        .clone();
    stop(t, a, c, i, &o)
}
pub(crate) fn report(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    report: lifecycle::Report,
) -> Result<()> {
    let v = t.state.invocations[i].clone();
    let o = access::object(&t.state, &v["object"])
        .ok_or("retirement object disappeared")?
        .clone();
    let phase = v["retirement"]["phase"].as_str().unwrap();
    let (operation, outcome) = match &report {
        lifecycle::Report::InspectRun { .. } => ("inspect-run.v1", "succeeded"),
        lifecycle::Report::Run { outcome, .. } => ("run.v1", outcome.as_str()),
        lifecycle::Report::Stop { outcome } => ("stop.v1", outcome.as_str()),
        lifecycle::Report::Connected { outcome, .. } => ("connect", outcome.as_str()),
    };
    if !["succeeded", "failed", "unknown"].contains(&outcome) {
        return Err("invalid retirement part outcome".into());
    }
    if let Some(runtime) = o.get("runtime").and_then(|r| access::object(&t.state, r)) {
        requests::record(
            t,
            a,
            c,
            "provider-call",
            json!({"target":runtime["id"],"subject":o["id"],"operation":c.qualified(operation),"invocation":v["id"],"caller":v["caller"],"phase":"ended","outcome":outcome}),
        )?;
    }
    event(
        t,
        a,
        c,
        i,
        &o["id"],
        "retirement-part-reported",
        json!({"partOperation":c.qualified(operation),"outcome":outcome}),
    )?;
    match report {
        lifecycle::Report::InspectRun { state } if phase == "inspect-run" => {
            if state == "running" {
                return hook(t, a, c, i, &o);
            }
            if state == "stopped" && v["cancelled"] != true {
                t.state.invocations[i]["retirement"]["phase"] = json!("run");
                let extra = lifecycle::run_inputs(&o, &v, c)?;
                return lifecycle::emit(t, a, c, i, "run.v1", extra);
            }
            if state != "unknown" && state != "stopped" {
                return Err("invalid run state".into());
            }
            add(t, i, "hook-run", "unknown");
        }
        lifecycle::Report::Run { outcome, .. } if phase == "run" => {
            if outcome == "succeeded" && v["cancelled"] != true {
                return hook(t, a, c, i, &o);
            }
            add(t, i, "hook-run", &outcome);
        }
        lifecycle::Report::Connected { outcome, .. } if phase == "connect" => {
            if outcome == "succeeded" && v["cancelled"] != true {
                return hook(t, a, c, i, &o);
            }
            add(t, i, "hook-connect", &outcome);
        }
        lifecycle::Report::Stop { outcome } if phase == "stop" => {
            add(t, i, "stop", &outcome);
            return settle(t, a, c, i, &o);
        }
        _ if v["cancelled"] == true => return Ok(()),
        _ => return Err("report does not match retirement part".into()),
    }
    stop(t, a, c, i, &o)
}
pub(crate) fn cancel(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    t.state.invocations[i]["cancelled"] = json!(true);
    add(t, i, "cancelled", "unknown");
    let v = t.state.invocations[i].clone();
    t.external_intents
        .push(json!({"kind":"cancel-setup-parts","invocation":v["id"],"subject":v["object"]}));
    let children: Vec<_> = t
        .state
        .invocations
        .iter()
        .enumerate()
        .filter(|(_, x)| x["setup_parent"] == v["id"] && x["state"] == "in-progress")
        .map(|(i, _)| i)
        .collect();
    for child in children {
        t.external_intents.push(json!({"kind":"cancel","invocation":t.state.invocations[child]["id"],"attempt":t.state.invocations[child]["attempt"]}));
        requests::finish(t, a, c, child, "unknown", json!({}), json!([]))?;
    }
    if v["retirement"]["phase"] == "stop" {
        t.reply = json!({"pending":true,"invocation":v["id"]});
        return Ok(());
    }
    let o = access::object(&t.state, &v["object"])
        .ok_or("retirement object disappeared")?
        .clone();
    stop(t, a, c, i, &o)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Evidence, schemas::Environment};
    use std::collections::BTreeMap;

    fn fixture(lost: bool, prior_hook: Option<&str>) -> (Transition, Context, Evidence) {
        let c = Context {
            builtin: None,
            core_package: "test.core".into(),
            meta_prefix: "test/".into(),
            templates: BTreeMap::new(),
            record_source: "urn:test:retirement".into(),
            core_environment: Environment {
                scopes: BTreeMap::new(),
            },
        };
        let object = json!({"id":"subject","kind":"Agent","name":"test","state":"retired","placement":"node","resolved_implementations":{c.qualified("retire-hook.v1"):{"item":{"command":["/test/program"]}}}});
        let mut invocations = vec![
            json!({"id":"setup","operation":c.qualified("set-up.v1"),"object":"subject","state":"failed"}),
            json!({"id":"parent","attempt":1,"object":"subject","state":"in-progress","sender":"root","caller":"root","target":"management","operation":c.qualified("retire.v1"),"contract":{"spec":{"effect":"retire"}}}),
        ];
        if let Some(outcome) = prior_hook {
            invocations.push(json!({"id":"hook","hook":true,"attempt":1,"target":"subject","state":outcome,"operation":c.qualified("retire-hook.v1"),"setup_parent":"old-parent"}));
        }
        let t = Transition {
            state: State {
                objects: vec![
                    object,
                    json!({"id":"node","state":if lost {"retired"} else {"ready"},"status":{"labels":{}}}),
                ],
                invocations,
                ..State::default()
            },
            decisions: vec![],
            records: vec![],
            external_intents: vec![],
            reply: Value::Null,
            request_replies: vec![],
        };
        let evidence = Evidence {
            time: "2026-11-10T12:00:00Z".into(),
            ids: (0..100).map(|n| format!("record-{n}")).collect(),
            revisions: vec![],
            suffixes: vec![],
        };
        (t, c, evidence)
    }

    fn assert_only_stop(t: &Transition, c: &Context) {
        assert_eq!(t.external_intents.len(), 1, "{:?}", t.external_intents);
        assert_eq!(t.external_intents[0]["operation"], c.qualified("stop.v1"));
    }

    #[test]
    fn further_retirement_reuses_unknown_hook_after_successful_stop() {
        let (mut t, c, evidence) = fixture(false, Some("unknown"));
        let mut a = Allocation {
            evidence: &evidence,
            cursor: 0,
        };
        // Complete the first retirement's hook and stop, retaining its unknown hook.
        t.state.invocations[1]["retirement"] =
            json!({"phase":"hook","unregister":false,"parts":[]});
        hook_ended(&mut t, &mut a, &c, 1, "unknown").unwrap();
        assert_only_stop(&t, &c);
        report(
            &mut t,
            &mut a,
            &c,
            1,
            lifecycle::Report::Stop {
                outcome: "succeeded".into(),
            },
        )
        .unwrap();
        assert_eq!(t.state.invocations[1]["state"], "unknown");
        assert_eq!(t.state.objects[0]["shutdown"], "unknown");
        let mut next = t.state.invocations[1].clone();
        next["id"] = json!("next-parent");
        next["state"] = json!("in-progress");
        t.state.invocations.push(next);
        t.external_intents.clear();
        let object = t.state.objects[0].clone();
        execute(&mut t, &mut a, &c, 3, &object).unwrap();
        assert_only_stop(&t, &c);
        assert_eq!(t.state.invocations[2]["attempt"], 1);
        assert_eq!(
            t.state.invocations[3]["retirement"]["parts"][0]["outcome"],
            "unknown"
        );
        report(
            &mut t,
            &mut a,
            &c,
            3,
            lifecycle::Report::Stop {
                outcome: "succeeded".into(),
            },
        )
        .unwrap();
        assert_eq!(t.state.invocations[3]["state"], "unknown");
        assert_eq!(t.state.objects[0]["shutdown"], "unknown");
    }

    #[test]
    fn lost_node_shutdown_stays_unknown_when_stop_fails() {
        let (mut t, c, evidence) = fixture(true, None);
        let mut a = Allocation {
            evidence: &evidence,
            cursor: 0,
        };
        let object = t.state.objects[0].clone();
        execute(&mut t, &mut a, &c, 1, &object).unwrap();
        assert_eq!(t.state.objects[0]["shutdown"], "unknown");
        assert_only_stop(&t, &c);
        report(
            &mut t,
            &mut a,
            &c,
            1,
            lifecycle::Report::Stop {
                outcome: "failed".into(),
            },
        )
        .unwrap();
        assert_eq!(t.state.invocations[1]["state"], "failed");
        assert_eq!(t.state.objects[0]["shutdown"], "unknown");
    }

    #[test]
    fn lost_node_keeps_unfinished_hook_unavailable_but_reuses_successful_hook() {
        for prior_hook in [None, Some("failed"), Some("succeeded")] {
            let (mut t, c, evidence) = fixture(true, prior_hook);
            let mut a = Allocation {
                evidence: &evidence,
                cursor: 0,
            };
            let object = t.state.objects[0].clone();
            execute(&mut t, &mut a, &c, 1, &object).unwrap();
            assert_only_stop(&t, &c);
            assert_eq!(
                t.state.invocations[1]["retirement"]["parts"][0]["outcome"],
                if prior_hook == Some("succeeded") {
                    "succeeded"
                } else {
                    "unavailable"
                }
            );
            report(
                &mut t,
                &mut a,
                &c,
                1,
                lifecycle::Report::Stop {
                    outcome: "succeeded".into(),
                },
            )
            .unwrap();
            if prior_hook == Some("succeeded") {
                assert_eq!(t.state.invocations[1]["state"], "succeeded");
                assert!(t.state.objects.iter().all(|o| o["id"] != "subject"));
            } else {
                assert_eq!(t.state.invocations[1]["state"], "unknown");
                assert_eq!(t.state.objects[0]["state"], "retired");
                assert_eq!(t.state.objects[0]["shutdown"], "unknown");
            }
        }
    }

    #[test]
    fn cancelling_retirement_settles_hook_and_still_requests_stop() {
        let c = Context {
            builtin: None,
            core_package: "test.core".into(),
            meta_prefix: "test/".into(),
            templates: BTreeMap::new(),
            record_source: "urn:test:retirement".into(),
            core_environment: Environment {
                scopes: BTreeMap::new(),
            },
        };
        let object = json!({"id":"subject","kind":"Agent","name":"test","state":"retired","placement":"node","resolved_implementations":{"run":{"item":{"command":["test"]}}}});
        let state = State {
            objects: vec![object, json!({"id":"node","status":{"labels":{}}})],
            invocations: vec![
                json!({"id":"parent","attempt":1,"object":"subject","state":"in-progress","sender":"root","caller":"root","target":"management","operation":c.qualified("retire.v1"),"contract":{"spec":{"effect":"retire"}},"retirement":{"phase":"hook","parts":[]}}),
                json!({"id":"hook","attempt":1,"state":"in-progress","sender":"root","caller":"root","target":"subject","operation":c.qualified("retire-hook.v1"),"contract":{"spec":{"effect":"hook"}},"setup_parent":"parent"}),
            ],
            ..State::default()
        };
        let mut t = Transition {
            state,
            decisions: vec![],
            records: vec![],
            external_intents: vec![],
            reply: Value::Null,
            request_replies: vec![],
        };
        let evidence = Evidence {
            time: "2026-11-10T12:00:00Z".into(),
            ids: (0..100).map(|n| format!("record-{n}")).collect(),
            revisions: vec![],
            suffixes: vec![],
        };
        let mut a = Allocation {
            evidence: &evidence,
            cursor: 0,
        };
        cancel(&mut t, &mut a, &c, 0).unwrap();
        assert_eq!(t.state.invocations[1]["state"], "unknown");
        assert_eq!(t.state.invocations[0]["retirement"]["phase"], "stop");
        assert!(
            t.external_intents
                .iter()
                .any(|i| i["operation"] == c.qualified("stop.v1"))
        );
        report(
            &mut t,
            &mut a,
            &c,
            0,
            lifecycle::Report::Stop {
                outcome: "succeeded".into(),
            },
        )
        .unwrap();
        assert_eq!(t.state.invocations[0]["state"], "unknown");
    }
}
