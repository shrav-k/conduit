//! LIF-021: durable, explicit container retirement; no actor retire permission is borrowed.
use crate::{
    Allocation, Context, Result, State, Transition, access, decision, explain, lifecycle, requests,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn handles(op: &str, c: &Context) -> bool {
    op == c.qualified("retire-with-resources.v1") || op == c.qualified("retire-with-agents.v1")
}
// LIF-021: runtime chains can pass through providers outside this container.
fn runtime_reaches(s: &State, member: &Value, target: &Value) -> bool {
    let mut next = member.get("runtime");
    let mut seen = BTreeSet::new();
    while let Some(provider) = next.and_then(|r| access::object(s, r)) {
        let id = provider["id"].as_str().unwrap();
        if provider["id"] == *target {
            return true;
        }
        if !seen.insert(id) {
            break;
        }
        next = provider.get("runtime");
    }
    false
}
fn reachable(edges: &BTreeMap<String, BTreeSet<String>>, from: &str, to: &str) -> bool {
    let mut seen = BTreeSet::new();
    let mut todo = vec![from.to_owned()];
    while let Some(n) = todo.pop() {
        if !seen.insert(n.clone()) {
            continue;
        }
        for next in &edges[&n] {
            if next == to {
                return true;
            }
            todo.push(next.clone());
        }
    }
    false
}
pub(crate) fn begin(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    let container = t.state.invocations[i]["target"].clone();
    let owner = access::object(&t.state, &container).ok_or("cascade container absent")?;
    let family = if owner["kind"] == "Pool" {
        "Resource"
    } else {
        "Agent"
    };
    let members: Vec<_> = t
        .state
        .objects
        .iter()
        .filter(|o| {
            o["kind"] == family
                && o.get("spec").is_none()
                && access::id(&o["owner"]) == &container
                && o["shutdown"] != "confirmed"
        })
        .cloned()
        .collect();
    let mut edges: BTreeMap<String, BTreeSet<String>> = members
        .iter()
        .map(|m| (m["id"].as_str().unwrap().to_owned(), BTreeSet::new()))
        .collect();
    for member in &members {
        let references = crate::registration::instance_references(member, &t.state.objects, c)?;
        for other in &members {
            let refers = references.contains(other["id"].as_str().unwrap())
                || (family == "Resource" && runtime_reaches(&t.state, member, &other["id"]));
            if member["id"] != other["id"] && refers {
                edges
                    .get_mut(member["id"].as_str().unwrap())
                    .unwrap()
                    .insert(other["id"].as_str().unwrap().into());
            }
        }
    }
    for conduit in t
        .state
        .objects
        .iter()
        .filter(|o| o["kind"] == "Conduit" && o.get("spec").is_none())
    {
        if let (Some(consumer), Some(target)) = (
            access::id(&conduit["consumer"]).as_str(),
            access::id(&conduit["target"]).as_str(),
        ) && consumer != target
            && edges.contains_key(consumer)
            && edges.contains_key(target)
        {
            edges.get_mut(consumer).unwrap().insert(target.into());
        }
    }
    // DL1: collapse cycles, then order their members by ID. Preserve all edges between cycles.
    let mut order = edges.clone();
    for (from, successors) in &edges {
        for to in successors {
            if reachable(&edges, to, from) {
                order.get_mut(from).unwrap().remove(to);
            }
        }
        for to in edges.keys() {
            if from < to && reachable(&edges, from, to) && reachable(&edges, to, from) {
                order.get_mut(from).unwrap().insert(to.clone());
            }
        }
    }
    let parts: Vec<_> = members.iter().map(|m| {
        let predecessors: Vec<_> = order.iter().filter(|(_, targets)| targets.contains(m["id"].as_str().unwrap()))
            .map(|(id, _)| id.clone()).collect();
        json!({"id":m["id"],"name":m["name"],"kind":m["kind"],"predecessors":predecessors,"status":"unattempted"})
    }).collect();
    t.state.invocations[i]["object"] = container;
    t.state.invocations[i]["management"] = json!(true);
    t.state.invocations[i]["cascade"] = json!({"parts":parts,"phase":"members","advancing":false});
    t.decisions.push(decision(
        "begin-container-retirement",
        &["LIF-021", "ACC-033", "REV-014"],
        &["DL1"],
        json!({}),
        vec![],
    ));
    advance(t, a, c, i)
}
fn confirmed(s: &State, id: &Value) -> bool {
    access::object(s, id).is_none_or(|o| o["shutdown"] == "confirmed")
}

enum RetirementDispatch {
    Dispatched(usize),
    Held {
        reason: &'static str,
        rule: &'static str,
    },
}

fn retire_part(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    parent: usize,
    object: &Value,
) -> Result<RetirementDispatch> {
    let p = t.state.invocations[parent].clone();
    let route = json!({"operation":c.qualified("retire.v1"),"target":p["target"],"arguments":{"instance":{"kind":object["kind"],"id":object["id"]}},"cascade_parent":p["id"]});
    if let Some((reason, rule)) = lifecycle::preflight(&t.state, &route, c)? {
        t.decisions.push(decision(
            "hold-cascade-member",
            &[rule, "LIF-021"],
            &[],
            json!({"reason":reason}),
            vec![],
        ));
        for event in ["access-decision", "request-ended"] {
            requests::record(
                t,
                a,
                c,
                event,
                json!({"target":p["target"],"object":object["id"],"caller":p["caller"],"consumer":p["sender"],"operation":route["operation"],"serving":p["id"],"reason":reason,"ruleId":rule,"dispatched":false}),
            )?;
        }
        return Ok(RetirementDispatch::Held { reason, rule });
    }
    let target = &p["held_target"];
    let (_, _, contract) =
        crate::schemas::operation(target, &c.qualified("retire.v1"), &c.core_package)?;
    let id = a.next(&mut t.state)?;
    let child = json!({"id":id,"sender":p["sender"],"caller":p["caller"],"target":p["target"],"conduit":p["conduit"],"operation":route["operation"],"arguments":route["arguments"],"filters":p["filters"],"state":"in-progress","attempt":1,"contract":contract,"held_target":target,"plain":false,"needs_result":false,"object":object["id"],"management":true,"cascade_parent":p["id"],"serving":p["id"]});
    t.state.invocations.push(child.clone());
    let index = t.state.invocations.len() - 1;
    requests::record(
        t,
        a,
        c,
        "access-decision",
        json!({"target":p["target"],"object":object["id"],"caller":p["caller"],"consumer":p["sender"],"conduit":p["conduit"],"operation":route["operation"],"invocation":id,"serving":p["id"],"permitted":true,"partOf":p["id"]}),
    )?;
    requests::record(
        t,
        a,
        c,
        "invocation-dispatched",
        json!({"target":p["target"],"object":object["id"],"caller":p["caller"],"consumer":p["sender"],"conduit":p["conduit"],"operation":route["operation"],"invocation":id,"attempt":1,"serving":p["id"]}),
    )?;
    t.decisions.push(decision(
        "dispatch-cascade-retirement",
        &[
            "LIF-021", "LIF-024", "ACC-033", "REV-014", "OPS-008", "REC-002",
        ],
        &[],
        json!({"object":object["id"]}),
        vec![],
    ));
    crate::retirement::execute(t, a, c, index, object)?;
    Ok(RetirementDispatch::Dispatched(index))
}
pub(crate) fn advance(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    if t.state.invocations[i]["cascade"]["advancing"] == true
        || t.state.invocations[i]["state"] != "in-progress"
    {
        return Ok(());
    }
    t.state.invocations[i]["cascade"]["advancing"] = json!(true);
    loop {
        let parts = t.state.invocations[i]["cascade"]["parts"]
            .as_array()
            .unwrap()
            .clone();
        let next = parts.iter().position(|part| {
            part["status"] == "unattempted"
                && part["predecessors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|id| confirmed(&t.state, id))
        });
        let Some(n) = next else {
            break;
        };
        let id = parts[n]["id"].clone();
        t.state.invocations[i]["cascade"]["parts"][n]["status"] = json!("attempted");
        if let Some(object) = access::object(&t.state, &id).cloned()
            && object["shutdown"] != "confirmed"
            && let RetirementDispatch::Dispatched(child) = retire_part(t, a, c, i, &object)?
        {
            t.state.invocations[i]["cascade"]["parts"][n]["invocation"] =
                t.state.invocations[child]["id"].clone();
        }
    }
    t.state.invocations[i]["cascade"]["advancing"] = json!(false);
    let parent = t.state.invocations[i].clone();
    if t.state
        .invocations
        .iter()
        .any(|v| v["cascade_parent"] == parent["id"] && v["state"] == "in-progress")
    {
        t.reply = json!({"pending":true,"invocation":parent["id"],"attempt":parent["attempt"]});
        return Ok(());
    }
    if parent["cascade"]["phase"] == "container" {
        let child = t
            .state
            .invocations
            .iter()
            .find(|v| v["id"] == parent["cascade"]["container_invocation"])
            .ok_or("container retirement missing")?;
        let outcome = child["state"].as_str().unwrap().to_owned();
        let cause = if outcome == "succeeded" {
            None
        } else {
            let mut detail = format!("The container retirement ended {outcome}.");
            for diagnostic in child["cause"].as_array().into_iter().flatten() {
                if let Some(message) = diagnostic["message"]["text"].as_str() {
                    detail.push(' ');
                    detail.push_str(message);
                }
            }
            Some(vec![container_diagnostic(&parent, &outcome, &detail)?])
        };
        return requests::finish(t, a, c, i, &outcome, json!({}), json!([]), cause);
    }
    // LIF-020/021: a member registered during this invocation was not in its
    // initial plan, but still prevents container retirement and must be named.
    let family = if parent["held_target"]["kind"] == "Pool" {
        "Resource"
    } else {
        "Agent"
    };
    let remaining: Vec<_> = t
        .state
        .objects
        .iter()
        .filter(|o| {
            o["kind"] == family
                && o.get("spec").is_none()
                && access::id(&o["owner"]) == &parent["object"]
                && o["shutdown"] != "confirmed"
        })
        .map(|o| json!({"id":o["id"],"kind":o["kind"],"name":o["name"]}))
        .collect();
    if !remaining.is_empty() || parent["cancelled"] == true {
        let mut cause = Vec::new();
        if parent["cancelled"] == true {
            cause.push(explain::outcome_cause(&parent, "failed", "LIF-021")?);
        }
        for p in &remaining {
            let kind = p["kind"].as_str().unwrap();
            let name = p["name"].as_str().unwrap();
            let id = p["id"].as_str().unwrap();
            cause.push(json!({"ruleId":"LIF-021","level":"error","message":{"text":format!("Shutdown is unconfirmed for {kind}/{name} ({id}).")}}));
        }
        return requests::finish(t, a, c, i, "failed", json!({}), json!([]), Some(cause));
    }
    t.state.invocations[i]["cascade"]["phase"] = json!("container");
    t.state.invocations[i]["cascade"]["advancing"] = json!(true);
    let container = access::object(&t.state, &parent["object"])
        .ok_or("container disappeared")?
        .clone();
    let dispatch = retire_part(t, a, c, i, &container)?;
    t.state.invocations[i]["cascade"]["advancing"] = json!(false);
    match dispatch {
        RetirementDispatch::Dispatched(child) => {
            t.state.invocations[i]["cascade"]["container_invocation"] =
                t.state.invocations[child]["id"].clone();
            advance(t, a, c, i)
        }
        RetirementDispatch::Held { reason, rule } => {
            let detail = explain::cause(reason, rule)?;
            let cause = container_diagnostic(
                &parent,
                "failed",
                &format!(
                    "The container retirement reported {reason} under {rule} before dispatch. {detail}"
                ),
            )?;
            requests::finish(
                t,
                a,
                c,
                i,
                "failed",
                json!({}),
                json!([]),
                Some(vec![cause]),
            )
        }
    }
}

fn container_diagnostic(parent: &Value, outcome: &str, detail: &str) -> Result<Value> {
    let prefix = if parent["cancelled"] == true {
        "The cascade was cancelled. "
    } else {
        ""
    };
    explain::outcome_diagnostic(parent, outcome, "LIF-021", &format!("{prefix}{detail}"))
}
pub(crate) fn child_ended(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    child: usize,
) -> Result<()> {
    if let Some(parent) = t.state.invocations[child].get("cascade_parent").cloned() {
        let i = t
            .state
            .invocations
            .iter()
            .position(|v| v["id"] == parent)
            .ok_or("cascade parent missing")?;
        advance(t, a, c, i)?;
    }
    Ok(())
}
pub(crate) fn cancel(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    t.state.invocations[i]["cancelled"] = json!(true);
    for part in t.state.invocations[i]["cascade"]["parts"]
        .as_array_mut()
        .unwrap()
    {
        if part["status"] == "unattempted" {
            part["status"] = json!("cancelled");
        }
    }
    t.state.invocations[i]["cascade"]["advancing"] = json!(true);
    let children: Vec<_> = t
        .state
        .invocations
        .iter()
        .enumerate()
        .filter(|(_, v)| {
            v["cascade_parent"] == t.state.invocations[i]["id"] && v["state"] == "in-progress"
        })
        .map(|(n, _)| n)
        .collect();
    for child in children {
        crate::retirement::cancel(t, a, c, child)?;
    }
    t.state.invocations[i]["cascade"]["advancing"] = json!(false);
    t.decisions.push(decision(
        "cancel-container-retirement",
        &["LIF-021", "OPS-027"],
        &[],
        json!({}),
        vec![],
    ));
    advance(t, a, c, i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Input,
        explain::tests::{assert_cause, evidence, fixture, transition},
        step,
    };

    fn cascade_fixture(members: &[&str]) -> (State, Context) {
        let (mut state, c) = fixture();
        state.objects[5]["kind"] = json!("Fleet");
        for member in members {
            state.objects.push(json!({
                "kind":"Agent","id":member,"name":member,"owner":"owners","shutdown":"pending"
            }));
        }
        state.invocations.push(json!({
            "id":"cascade","state":"in-progress","attempt":1,"target":"owners","object":"owners",
            "caller":"root","sender":"root","held_target":{"kind":"Fleet"},
            "operation":c.qualified("retire-with-agents.v1"),
            "named_subject":"the container named by tool workspace.retire-with-agents.v1",
            "cascade":{"phase":"members","parts":[]}
        }));
        (state, c)
    }

    fn cancel_cascade(state: &State, c: &Context) -> Transition {
        step(
            state,
            Input::Cancel {
                evidence: evidence(),
                invocation: "cascade".into(),
                confirmation: None,
            },
            c,
        )
        .unwrap()
    }

    #[test]
    fn cascade_cancel_lists_cancellation_before_any_remaining_member() {
        for members in [&[][..], &["member"][..]] {
            let (state, c) = cascade_fixture(members);
            let ended = cancel_cascade(&state, &c);
            let diagnostics = ended.reply["structuredContent"]["diagnostics"]
                .as_array()
                .unwrap();
            assert_eq!(diagnostics.len(), 1 + members.len());
            assert!(diagnostics.iter().all(|d| d["ruleId"] == "LIF-021"));
            assert!(
                diagnostics[0]["message"]["text"]
                    .as_str()
                    .unwrap()
                    .ends_with("The cascade was cancelled and dispatched no further retirement.")
            );
            if !members.is_empty() {
                assert_eq!(
                    diagnostics[1]["message"]["text"],
                    "Shutdown is unconfirmed for Agent/member (member)."
                );
            }
            assert_eq!(ended.reply["_meta"]["test/outcome"], "failed");
            assert!(ended.external_intents.is_empty());
        }
    }

    #[test]
    fn cascade_in_container_phase_reports_why_retirement_did_not_succeed() {
        let (mut state, c) = cascade_fixture(&[]);
        state.invocations[0]["cascade"]["phase"] = json!("container");
        state.invocations[0]["cascade"]["container_invocation"] = json!("retirement");
        state.invocations.push(json!({
            "id":"retirement","state":"in-progress","attempt":1,"target":"owners","object":"owners",
            "caller":"root","sender":"root","cascade_parent":"cascade","operation":c.qualified("retire.v1"),
            "retirement":{"phase":"stop","parts":[]}
        }));
        let pending = cancel_cascade(&state, &c);
        assert_eq!(pending.reply["pending"], true);
        for (state, stop_outcome, outcome, detail) in [
            (
                &pending.state,
                "succeeded",
                "unknown",
                "The cascade was cancelled. The container retirement ended unknown. test.core#retire.v1 ended unknown under OPS-022: The invocation was cancelled. Required part cancelled ended unknown.",
            ),
            (
                &state,
                "failed",
                "failed",
                "The container retirement ended failed. test.core#retire.v1 ended failed under OPS-022: Required part stop ended failed.",
            ),
        ] {
            let ended = step(
                state,
                Input::SetupReport {
                    evidence: evidence(),
                    invocation: "retirement".into(),
                    report: lifecycle::Report::Stop {
                        outcome: stop_outcome.into(),
                    },
                },
                &c,
            )
            .unwrap();
            assert_eq!(ended.reply["_meta"]["test/outcome"], outcome);
            assert_cause(&ended.reply, "LIF-021", detail);
        }
    }

    #[test]
    fn cascade_repeat_replays_diagnostics_for_all_remaining_members() {
        let (state, c) = cascade_fixture(&["first", "second"]);
        let mut t = transition(state);
        let evidence = evidence();
        let mut a = Allocation {
            evidence: &evidence,
            cursor: 0,
        };
        advance(&mut t, &mut a, &c, 0).unwrap();
        explain::first_text(&mut t.reply, &c);
        let diagnostics = t.reply["structuredContent"]["diagnostics"]
            .as_array()
            .unwrap();
        assert_eq!(diagnostics.len(), 2);
        for (diagnostic, name) in diagnostics.iter().zip(["first", "second"]) {
            assert_eq!(diagnostic["ruleId"], "LIF-021");
            assert_eq!(
                diagnostic["message"]["text"],
                format!("Shutdown is unconfirmed for Agent/{name} ({name}).")
            );
        }
        // Exercise the ended reply path. OPS-024 redispatches a normal failed repeat.
        let invocation = &t.state.invocations[0];
        let mut repeated = requests::ended_reply(&c, invocation, &invocation["contract"], "failed");
        explain::first_text(&mut repeated, &c);
        assert_eq!(repeated, t.reply);
    }

    #[test]
    fn cascade_with_undispatched_container_retirement_names_lif021() {
        let (mut state, c) = cascade_fixture(&[]);
        state.invocations.push(json!({
            "id":"busy","state":"in-progress","object":"owners","management":true
        }));
        let mut t = transition(state);
        let evidence = evidence();
        let mut a = Allocation {
            evidence: &evidence,
            cursor: 0,
        };
        advance(&mut t, &mut a, &c, 0).unwrap();
        assert_eq!(t.reply["_meta"]["test/outcome"], "failed");
        assert_cause(
            &t.reply,
            "LIF-021",
            "The container retirement reported conflict under REV-014 before dispatch. Setup, retirement and change must wait for any dispatched setup, retirement or change to end.",
        );
    }

    /// Decision 5 of order 046a: a held member's records name the deciding rule beside the
    /// reason, and the hold is unchanged.
    #[test]
    fn held_member_records_carry_the_rule() {
        let (mut state, c) = fixture();
        state.objects.push(
            json!({"kind":"Agent","id":"member","name":"member","state":"ready","owner":"owners"}),
        );
        state.invocations.push(json!({"id":"cascade","state":"in-progress","sender":"root","caller":"root","target":"owners","operation":c.qualified("retire-with-agents.v1")}));
        // REV-014: another management invocation on the member is in progress.
        state
            .invocations
            .push(json!({"id":"busy","state":"in-progress","object":"member","management":true}));
        let mut t = transition(state);
        let evidence = evidence();
        let mut a = Allocation {
            evidence: &evidence,
            cursor: 0,
        };
        let member = t.state.objects.last().unwrap().clone();
        assert!(matches!(
            retire_part(&mut t, &mut a, &c, 0, &member).unwrap(),
            RetirementDispatch::Held { .. }
        ));
        assert_eq!(t.state.invocations.len(), 2);
        let decision = t.decisions.last().unwrap();
        assert_eq!(decision.action, "hold-cascade-member");
        assert_eq!(decision.promises, ["REV-014", "LIF-021"]);
        let held: Vec<_> = t
            .records
            .iter()
            .filter(|r| r["data"]["reason"] == "conflict")
            .collect();
        assert_eq!(held.len(), 2, "{:?}", t.records);
        for record in held {
            assert_eq!(record["data"]["ruleId"], "REV-014", "{record}");
            assert_eq!(record["data"]["dispatched"], false);
        }
    }
}
