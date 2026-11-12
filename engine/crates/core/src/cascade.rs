//! LIF-021: durable, explicit container retirement; no actor retire permission is borrowed.
use crate::{
    Allocation, Context, Result, State, Transition, access, decision, lifecycle, requests,
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
fn retire_part(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    parent: usize,
    object: &Value,
) -> Result<Option<usize>> {
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
                json!({"target":p["target"],"object":object["id"],"caller":p["caller"],"consumer":p["sender"],"operation":route["operation"],"serving":p["id"],"reason":reason,"dispatched":false}),
            )?;
        }
        return Ok(None);
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
    Ok(Some(index))
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
            && let Some(child) = retire_part(t, a, c, i, &object)?
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
        return requests::finish(t, a, c, i, &outcome, json!({}), json!([]));
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
        requests::finish(t, a, c, i, "failed", json!({}), json!([]))?;
        let diagnostics: Vec<_> = remaining.iter().map(|p| json!({"ruleId":"LIF-021","level":"error","message":{"text":format!("Shutdown is unconfirmed for {}/{} ({}).",p["kind"].as_str().unwrap(),p["name"].as_str().unwrap(),p["id"].as_str().unwrap())}})).collect();
        t.reply["structuredContent"] = json!({"diagnostics":diagnostics});
        if let Some(reply) = t.request_replies.last_mut() {
            reply.reply = t.reply.clone();
        }
        return Ok(());
    }
    t.state.invocations[i]["cascade"]["phase"] = json!("container");
    t.state.invocations[i]["cascade"]["advancing"] = json!(true);
    let container = access::object(&t.state, &parent["object"])
        .ok_or("container disappeared")?
        .clone();
    let child = retire_part(t, a, c, i, &container)?;
    t.state.invocations[i]["cascade"]["advancing"] = json!(false);
    if let Some(child) = child {
        t.state.invocations[i]["cascade"]["container_invocation"] =
            t.state.invocations[child]["id"].clone();
        advance(t, a, c, i)
    } else {
        requests::finish(t, a, c, i, "failed", json!({}), json!([]))
    }
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
