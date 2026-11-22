//! Ordered setup parts and Conduit lifecycle. All runner work is an intent; reports are
//! explicit inputs. Pending plans live with their invocation and survive serialization.
use crate::{
    Allocation, Context, Evidence, Result, State, Transition, access, decision, explain, pending,
    requests, sign_in,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Report {
    InspectRun {
        state: String,
    },
    Run {
        outcome: String,
        #[serde(default)]
        url: Option<String>,
    },
    Stop {
        outcome: String,
    },
    Connected {
        part: String,
        outcome: String,
    },
    ProviderCallReady {
        operation: String,
    },
    ProviderDiscoveryUnavailable {
        operation: String,
    },
    ProviderCallUnavailable {
        operation: String,
    },
    ProviderRunUnproved {
        operation: String,
    },
    ProviderBridgeUninitialized {
        operation: String,
    },
}
pub(crate) fn handles(op: &str, c: &Context) -> bool {
    [
        "retire.v1",
        "unregister.v1",
        "set-up.v1",
        "open.v1",
        "close.v1",
        "lift.v1",
        "register-replacement.v1",
    ]
    .iter()
    .any(|n| op == c.qualified(n))
        || crate::cascade::handles(op, c)
}
fn named<'a>(s: &'a State, route: &Value, c: &Context) -> Option<&'a Value> {
    if crate::cascade::handles(route["operation"].as_str().unwrap_or(""), c) {
        return access::object(s, &route["target"]);
    }
    if route["operation"] == c.qualified("lift.v1") {
        return access::object(s, &route["target"]);
    }
    let key = if route["operation"] == c.qualified("unregister.v1") {
        "object"
    } else if route["operation"] == c.qualified("open.v1")
        || route["operation"] == c.qualified("close.v1")
    {
        "conduit"
    } else {
        "instance"
    };
    let value = &route["arguments"][key];
    // A bare conduit name has a fixed family.
    if key == "conduit" && value.is_string() {
        s.objects
            .iter()
            .find(|o| o["kind"] == "Conduit" && o["name"] == *value)
    } else {
        access::object(s, value)
    }
}
pub(crate) fn preflight(
    s: &State,
    route: &Value,
    c: &Context,
) -> Result<Option<(&'static str, &'static str)>> {
    let op = route["operation"].as_str().unwrap_or("");
    if !handles(op, c) && op != c.qualified("retire.v1") {
        return Ok(None);
    }
    if op == c.qualified("lift.v1") {
        return Ok(
            if route["arguments"]["id"]
                .as_str()
                .is_some_and(|id| s.tombstones.contains_key(id))
            {
                None
            } else {
                Some(("invalid", "OPS-029"))
            },
        );
    }
    let o = named(s, route, c).ok_or("authorized lifecycle route has no object")?;
    let setup = op == c.qualified("set-up.v1");
    let retire = op == c.qualified("retire.v1");
    let open = op == c.qualified("open.v1");
    let close = op == c.qualified("close.v1");
    if (setup || retire) && (o.get("spec").is_some() || o["kind"] == "Conduit") {
        return Ok(Some(("invalid", "LIF-032")));
    }
    if (open || close) && (o["kind"] != "Conduit" || o.get("spec").is_some()) {
        return Ok(Some(("invalid", "OPS-029")));
    }
    if route["arguments"]
        .get("expectedId")
        .is_some_and(|id| *id != o["id"])
    {
        return Ok(Some(("conflict", "LIF-037")));
    }
    if setup && o["state"] == "retired" {
        return Ok(Some(("conflict", "LIF-015")));
    }
    if open
        && o["open"] != true
        && !crate::inventory::engine_made(o, c)
        && crate::inventory::matching(s, o).is_some()
    {
        return Ok(Some(("conflict", "LIF-041")));
    }
    if open
        && o["open"] != true
        && ["consumer", "target"]
            .iter()
            .any(|k| access::object(s, &o[k]).is_some_and(|v| v["state"] == "retired"))
    {
        return Ok(Some(("conflict", "CON-031")));
    }
    if op == c.qualified("register-replacement.v1") {
        let slot = if o["kind"] == "Pool" {
            "entries"
        } else {
            "conduits"
        };
        let n = route["arguments"]["nested"].as_str().unwrap_or("");
        if o["first_start"] == true || o.get("spec").is_some() || o[slot].get(n).is_none() {
            return Ok(Some(("invalid", "LIF-009")));
        }
        if s.objects.iter().any(|v| {
            v["nested"]["parent"] == o["id"]
                && v["nested"]["slot"] == slot
                && v["nested"]["name"] == n
                && v["state"] != "retired"
        }) {
            return Ok(Some((
                "conflict",
                if slot == "entries" {
                    "POOL-003"
                } else {
                    "LIF-029"
                },
            )));
        }
    }
    if retire || op == c.qualified("unregister.v1") {
        if let Some(reason) = crate::retirement::preflight(s, o, !retire) {
            return Ok(Some(reason));
        }
        if !retire && o["revision"] != route["arguments"]["revision"] {
            return Ok(Some(("conflict", "REV-002")));
        }
    }
    if busy(s, &o["id"], route.get("cascade_parent")) {
        return Ok(Some(("conflict", "REV-014")));
    }
    Ok(None)
}
// REV-014 includes controls and internally dispatched retirements.
pub(crate) fn busy(s: &State, object: &Value, except: Option<&Value>) -> bool {
    s.invocations.iter().any(|v| {
        v["state"] == "in-progress"
            && except.is_none_or(|id| v["id"] != *id)
            && (v["object"] == *object && v["management"] == true
                || v["target"] == *object && v["contract"]["spec"].get("changes").is_some())
    })
}
fn cite(t: &mut Transition, action: &str, rules: &[&str], object: &Value) {
    t.decisions.push(decision(
        action,
        rules,
        &[],
        json!({}),
        object.as_str().map(str::to_owned).into_iter().collect(),
    ));
}
fn change(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    subject: &Value,
    action: &str,
    extra: Value,
) -> Result<()> {
    let v = &t.state.invocations[i];
    let mut data = json!({"target":subject,"invocation":v["id"],"caller":v["caller"],"consumer":v["sender"],"conduit":v["conduit"],"operation":v["operation"],"action":action});
    data.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    requests::record(t, a, c, "object-change", data)
}
pub(crate) fn execute(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    if crate::cascade::handles(
        t.state.invocations[i]["operation"].as_str().unwrap_or(""),
        c,
    ) {
        return crate::cascade::begin(t, a, c, i);
    }
    let v = t.state.invocations[i].clone();
    let route = json!({"operation":v["operation"],"arguments":v["arguments"]});
    if v["operation"] == c.qualified("lift.v1") {
        let path = access::object(&t.state, &v["conduit"])
            .ok_or("lift authority absent")?
            .clone();
        let stone = t
            .state
            .tombstones
            .get(v["arguments"]["id"].as_str().ok_or("lift id absent")?)
            .ok_or("lift tombstone absent")?
            .clone();
        t.state.invocations[i]["management"] = json!(true);
        t.state.invocations[i]["lifted_tombstone"] = stone;
        let outputs = crate::inventory::lift(t, &path, &v["caller"], &v["arguments"]["id"], c);
        return requests::finish(t, a, c, i, "succeeded", outputs, json!([]), None);
    }
    let o = named(&t.state, &route, c)
        .ok_or("lifecycle object missing")?
        .clone();
    t.state.invocations[i]["object"] = o["id"].clone();
    t.state.invocations[i]["management"] = json!(true);
    let op = v["operation"].as_str().unwrap();
    if op == c.qualified("retire.v1") || op == c.qualified("unregister.v1") {
        return crate::retirement::execute(t, a, c, i, &o);
    }
    if op == c.qualified("open.v1") {
        if o["open"] != true {
            let provenance = match v.get("opening_provenance") {
                Some(p) => p.clone(),
                None => crate::inventory::provenance(&t.state, &v, c)?,
            };
            let object = t
                .state
                .objects
                .iter_mut()
                .find(|v| v["id"] == o["id"])
                .unwrap();
            object["provenance"] = provenance;
            object["open"] = json!(true);
            object["state"] = json!("ready");
            change(t, a, c, i, &o["id"], "opened", json!({"state":"ready"}))?;
        }
        cite(
            t,
            "open-conduit",
            &[
                "ACC-006", "ACC-044", "LIF-041", "CON-023", "LIF-003", "REV-004",
            ],
            &o["id"],
        );
        return requests::finish(t, a, c, i, "succeeded", json!({}), json!([]), None);
    }
    if op == c.qualified("close.v1") {
        let stone = crate::inventory::make(
            t,
            c,
            &o,
            &v,
            &a.evidence.time,
            json!({"invocation":v["id"]}),
        )?;
        if let Some(id) = stone {
            t.state.invocations[i]["access_subject"] =
                t.state.tombstones[id.as_str().unwrap()].clone();
        }
        if let Some(consumer) = t
            .state
            .objects
            .iter_mut()
            .find(|v| v["id"] == *access::id(&o["consumer"]))
        {
            consumer["status"]["closedConduits"][o["id"].as_str().unwrap()] =
                json!({"name":o["name"],"capability":o["capability"]});
        }
        change(
            t,
            a,
            c,
            i,
            &o["id"],
            "closed",
            json!({"state":"retired","shutdown":"confirmed"}),
        )?;
        t.state.objects.retain(|v| v["id"] != o["id"]);
        change(t, a, c, i, &o["id"], "removed", json!({}))?;
        cite(
            t,
            "close-remove-conduit",
            &[
                "CON-008", "CON-011", "CON-014", "LIF-024", "LIF-035", "LIF-019", "ACC-034",
                "IFC-013", "REV-004",
            ],
            &o["id"],
        );
        return requests::finish(t, a, c, i, "succeeded", json!({}), json!([]), None);
    }
    if op == c.qualified("register-replacement.v1") {
        let outputs = crate::registration::replacement(t, a, c, &o, &v["arguments"]["nested"])?;
        if let Some(rule) = outputs["rule"].as_str() {
            // Validation precedes dispatch in the public wrapper (see replacement preflight).
            return Err(format!("replacement was not validated: {rule}"));
        }
        change(
            t,
            a,
            c,
            i,
            &outputs["object"]["id"],
            "registered",
            json!({"state":"registered"}),
        )?;
        cite(
            t,
            "register-nested-replacement",
            &[
                "LIF-008", "LIF-009", "INS-002", "INS-020", "OBJ-003", "ACC-033",
            ],
            &outputs["object"]["id"],
        );
        return requests::finish(t, a, c, i, "succeeded", outputs, json!([]), None);
    }
    t.state.invocations[i]["setup"] = json!({"phase":"declarations","parts":[],"cursor":0});
    cite(
        t,
        "begin-setup",
        &["TPL-008", "TPL-012", "IMP-002", "REV-004"],
        &o["id"],
    );
    advance(t, a, c, i)
}
fn part(
    t: &mut Transition,
    i: usize,
    name: &str,
    outcome: &str,
    effect: bool,
    optional: bool,
    invocation: Value,
) {
    t.state.invocations[i]["setup"]["parts"].as_array_mut().unwrap().push(json!({"name":name,"outcome":outcome,"effect":effect,"optional":optional,"invocation":invocation}));
}
pub(crate) fn report_sign_in_undone(
    t: &mut Transition,
    instance: &Value,
    outcome: &str,
    hook: &Value,
    time: &str,
    polling: bool,
) -> Result<()> {
    let object = t
        .state
        .objects
        .iter_mut()
        .find(|o| o["id"] == *instance)
        .ok_or("sign-out instance disappeared")?;
    sign_in::set_sign_in_undone_false(object, outcome, hook, time, polling);
    t.decisions.push(decision(
        "report-sign-in-undone",
        &["PND-011"],
        &[],
        json!({"instance":instance,"hook":hook}),
        vec![],
    ));
    Ok(())
}

fn settle(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize) -> Result<()> {
    let v = t.state.invocations[i].clone();
    let parts = v["setup"]["parts"].as_array().unwrap();
    let all = parts.iter().all(|p| {
        p["optional"] == true
            || p["outcome"] == "succeeded"
            || (p["name"] == "account" && v["setup"]["accountCounted"].is_string())
    });
    let effect = v["setup"]["run_effect_observed"] == true
        || parts.iter().any(|p| {
            p["effect"] == true && (p["outcome"] == "succeeded" || p["outcome"] == "unknown")
        });
    let outcome = if all {
        "succeeded"
    } else if effect {
        "unknown"
    } else {
        "failed"
    };
    if let Some(object) = t.state.objects.iter_mut().find(|o| o["id"] == v["object"]) {
        if all && object["state"] != "retired" {
            object["state"] = json!("ready");
        }
        object["status"]["setup"] = json!({"invocation":v["id"],"outcome":outcome,"parts":parts});
    }
    t.state.invocations[i]["setup"]["phase"] = json!("ended");
    change(
        t,
        a,
        c,
        i,
        &v["object"],
        "setup-ended",
        json!({"outcome":outcome}),
    )?;
    cite(
        t,
        "settle-setup-parts",
        &[
            "OPS-022", "CON-004", "LIF-003", "LIF-004", "LIF-006", "LIF-027", "REV-004",
        ],
        &v["object"],
    );
    let mut cause = explain::settle_cause(&v, outcome, parts)?;
    if let Some(sign_out) = v["setup"]["signOut"].as_str() {
        cause.get_or_insert_with(Vec::new).push(explain::outcome_diagnostic(
            &v,
            outcome,
            "PND-011",
            &format!("The sign-out hook ended {sign_out} after the account check found an account other than the expected one, so SignInUndone is False and requests to the instance other than the parts of sign-in.v1, of a setup and of its retirement report unavailable until an account check finds the expected account or none."),
        )?);
    }
    requests::finish(t, a, c, i, outcome, json!({}), json!([]), cause)
}
fn request_for(
    t: &Transition,
    parent: &Value,
    target: &Value,
    operation: &str,
    arguments: Value,
    c: &Context,
) -> Result<Option<access::AccessRequest>> {
    // ACC-033: all candidate evaluations share one call-local view.
    let view = access::PublicationView::new(&t.state, c);
    let mut refused = None;
    for p in view.paths(&parent["sender"]) {
        let p = p?;
        if p["target"] != *target {
            continue;
        }
        if !p["operations"]
            .as_array()
            .is_some_and(|ops| ops.contains(&json!(operation)))
        {
            continue;
        }
        let local = operation.rsplit_once('#').unwrap().1;
        let request = access::AccessRequest {
            identity: access::Identity::Instance {
                id: parent["sender"].as_str().unwrap().into(),
            },
            tool: format!("{}.{local}", p["local_name"].as_str().unwrap()),
            arguments: arguments.clone(),
            serving: if parent.get("setup_parent").is_some() {
                parent["authorization_serving"].as_str()
            } else {
                parent["serving"].as_str()
            }
            .map(str::to_owned),
            readable: true,
            authority_readable: true,
            client_capabilities: None,
            sign_in_login: None,
            sign_in_return: None,
        };
        if access::evaluate(&t.state, &view, &request, c, true)?
            .reason
            .is_none()
        {
            return Ok(Some(request));
        }
        refused = Some(request);
    }
    Ok(refused)
}
// POOL-006 / POOL-013: references are frozen Resource IDs, including those
// introduced by the Resource template's configuration or declared Conduits.
fn entry_dependencies(s: &State, o: &Value, name: &str, declaration: &Value) -> BTreeSet<String> {
    fn collect(v: &Value, ids: &BTreeMap<String, String>, out: &mut BTreeSet<String>) {
        match v {
            Value::Object(m) => {
                if v["kind"] == "Resource"
                    && let Some(name) = v["id"].as_str().and_then(|id| ids.get(id))
                {
                    out.insert(name.clone());
                }
                for value in m.values() {
                    collect(value, ids, out);
                }
            }
            Value::Array(values) => {
                for value in values {
                    collect(value, ids, out);
                }
            }
            _ => {}
        }
    }
    let ids = o["slots"]["entries"]
        .as_object()
        .into_iter()
        .flatten()
        .filter_map(|(name, r)| r["id"].as_str().map(|id| (id.into(), name.clone())))
        .collect();
    let mut out = BTreeSet::new();
    collect(declaration, &ids, &mut out);
    if let Some(child) = s.objects.iter().find(|ch| {
        ch["nested"]["parent"] == o["id"]
            && ch["nested"]["name"] == name
            && ch["nested"]["slot"] == "entries"
            && ch["state"] != "retired"
    }) {
        for key in ["config", "conduits", "requirements", "placement", "runtime"] {
            collect(&child[key], &ids, &mut out);
        }
    }
    out
}
fn declarations(s: &State, o: &Value, slot: &str) -> Result<Vec<(String, Value)>> {
    let mut remaining: BTreeMap<_, _> = o[slot]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(n, d)| (n.clone(), d.clone()))
        .collect();
    let mut ordered = vec![];
    let mut done = BTreeSet::new();
    while !remaining.is_empty() {
        let name = remaining
            .iter()
            .find(|(n, d)| slot != "entries" || entry_dependencies(s, o, n, d).is_subset(&done))
            .map(|(n, _)| n.clone())
            .ok_or("POOL-011: cyclic entry dependencies")?;
        done.insert(name.clone());
        let declaration = remaining.remove(&name).unwrap();
        ordered.push((name, declaration));
    }
    Ok(ordered)
}
fn advance(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize) -> Result<()> {
    let v = t.state.invocations[i].clone();
    let o = access::object(&t.state, &v["object"])
        .ok_or("setup object disappeared")?
        .clone();
    if v["cancelled"] == true {
        return settle(t, a, c, i);
    }
    if o["state"] == "retired" {
        // TPL-012: retirement prohibits new work, but cannot discard a dispatched
        // part's eventual effect. In particular a connection cannot settle a run.
        if v["setup"]["phase"] == "entry"
            || v["setup"]["phase"] == "hook-pending"
            || v["setup"]["phase"] == "account-pending"
            || v["setup"]["phase"] == "undo-pending"
            || v["setup"]["run_phase"]
                .as_str()
                .is_some_and(|p| p != "ended")
            || v["setup"]["connections"]
                .as_object()
                .into_iter()
                .flat_map(|m| m.values())
                .any(|p| p == "pending")
        {
            t.reply = json!({"pending":true,"invocation":v["id"]});
            return Ok(());
        }
        let phase = v["setup"]["phase"].as_str().unwrap_or("");
        let hook = o["resolved_implementations"]
            .get(c.qualified("setup-hook.v1"))
            .is_some();
        let implementations = o["resolved_implementations"]
            .as_object()
            .into_iter()
            .flat_map(|m| m.values())
            .any(|x| {
                x["item"].get("command").is_some()
                    || x["item"].get("package").is_some()
                    || x["item"]["remote"].get("headers").is_some()
            });
        let slot = if o["kind"] == "Pool" {
            "entries"
        } else {
            "conduits"
        };
        let declarations_left = phase == "declarations"
            && declarations(&t.state, &o, slot)?
                .iter()
                .skip(v["setup"]["cursor"].as_u64().unwrap_or(0) as usize)
                .any(|(_, d)| d["optional"] != true);
        let account_left = ["declarations", "implementations", "work", "account"].contains(&phase)
            && matches!(
                sign_in::expected_account(&o, c)?,
                sign_in::ExpectedAccount::Set { .. }
            );
        if declarations_left
            || (phase == "declarations" || phase == "implementations") && implementations
            || account_left
            || hook
                && ["declarations", "implementations", "work", "account", "hook"].contains(&phase)
        {
            part(
                t,
                i,
                "retired-before-part",
                "failed",
                false,
                false,
                Value::Null,
            );
        }
        cite(
            t,
            "stop-setup-after-retirement",
            &["TPL-012", "OPS-022", "LIF-006"],
            &o["id"],
        );
        return settle(t, a, c, i);
    }
    if v["setup"]["phase"] == "declarations" {
        let slot = if o["kind"] == "Pool" {
            "entries"
        } else {
            "conduits"
        };
        let declarations = declarations(&t.state, &o, slot)?;
        let cursor = v["setup"]["cursor"].as_u64().unwrap() as usize;
        for (n, (name, d)) in declarations.into_iter().enumerate().skip(cursor) {
            let child = t
                .state
                .objects
                .iter()
                .find(|ch| {
                    ch["nested"]["parent"] == o["id"]
                        && ch["nested"]["slot"] == slot
                        && ch["nested"]["name"] == name
                        && ch["state"] != "retired"
                })
                .cloned();
            let optional = d["optional"] == true;
            t.state.invocations[i]["setup"]["cursor"] = json!(n + 1);
            let mut part_diagnostics = Value::Null;
            let mut outcome = "conflict".to_owned();
            let mut id = Value::Null;
            let mut waiting_for = Vec::new();
            if slot == "entries" {
                let parts = t.state.invocations[i]["setup"]["parts"].as_array().unwrap();
                waiting_for.extend(
                    entry_dependencies(&t.state, &o, &name, &d)
                        .into_iter()
                        .filter(|dependency| {
                            !parts
                                .iter()
                                .any(|p| p["name"] == *dependency && p["outcome"] == "succeeded")
                        }),
                );
            }
            if !waiting_for.is_empty() {
                outcome = "unavailable".into();
                let entries = waiting_for.join(", ");
                part_diagnostics = json!([{
                    "ruleId":"POOL-013","level":"error",
                    "message":{"text":format!("Setup part {name} was held back pending successful setup of {entries}.")}
                }]);
            } else if let Some(child) = child {
                if child["state"] == "ready" {
                    outcome = "succeeded".into();
                } else {
                    let (target, op, args) = if slot == "conduits" {
                        let target = t
                            .state
                            .objects
                            .iter()
                            .find(|o| o["from"] == c.qualified("access"))
                            .ok_or("access absent")?["id"]
                            .clone();
                        (
                            target,
                            c.qualified("open.v1"),
                            json!({"conduit":child["name"]}),
                        )
                    } else {
                        (
                            access::id(&child["owner"]).clone(),
                            c.qualified("set-up.v1"),
                            json!({"instance":{"kind":"Resource","name":child["name"]}}),
                        )
                    };
                    if let Some(request) = request_for(t, &v, &target, &op, args.clone(), c)? {
                        requests::invoke_part(t, a, &request, None, c, Some(&v))?;
                        id = t
                            .reply
                            .get("invocation")
                            .cloned()
                            .or_else(|| {
                                t.reply["_meta"]
                                    .get(format!("{}invocation", c.meta_prefix))
                                    .cloned()
                            })
                            .unwrap_or(Value::Null);
                        outcome = t.reply["_meta"]
                            .get(format!("{}outcome", c.meta_prefix))
                            .and_then(Value::as_str)
                            .or_else(|| {
                                t.reply["_meta"]
                                    .get(format!("{}reason", c.meta_prefix))
                                    .or_else(|| {
                                        t.reply["error"]["data"]
                                            .get(format!("{}reason", c.meta_prefix))
                                    })
                                    .and_then(Value::as_str)
                            })
                            .unwrap_or("refused")
                            .into();
                        part_diagnostics = t.reply["structuredContent"]
                            .get("diagnostics")
                            .or_else(|| t.reply["error"]["data"].get("diagnostics"))
                            .cloned()
                            .unwrap_or(Value::Null);
                        if t.reply["pending"] == true {
                            t.state.invocations[i]["setup"]["phase"] = json!("entry");
                            t.state.invocations[i]["setup"]["waiting"] =
                                json!({"id":id,"name":name,"optional":optional});
                            t.reply = json!({"pending":true,"invocation":v["id"]});
                            return Ok(());
                        }
                    } else {
                        outcome = "refused".into();
                        let route = json!({"consumer":v["sender"],"operation":op,"arguments":args});
                        let tier = crate::explain::tier(&t.state, &route, c)?;
                        part_diagnostics = json!([crate::explain::diagnostic(
                            &t.state, &route, c, "refused", "ACC-001"
                        )?]);
                        for event in ["access-decision", "request-ended"] {
                            requests::record(
                                t,
                                a,
                                c,
                                event,
                                json!({"target":target,"caller":v["caller"],"consumer":v["sender"],"operation":op,"serving":v["id"],"reason":"refused","ruleId":"ACC-001","refusalTier":tier,"dispatched":false}),
                            )?;
                        }
                    }
                }
            }
            part(t, i, &name, &outcome, true, optional, id);
            if let Some(last) = t.state.invocations[i]["setup"]["parts"]
                .as_array_mut()
                .and_then(|v| v.last_mut())
            {
                last["object"] = o["slots"][slot][&name].clone();
                if part_diagnostics.is_array() {
                    last["diagnostics"] = part_diagnostics;
                } else if outcome == "conflict" {
                    last["diagnostics"] = json!([{"ruleId":"CON-004","level":"error","message":{"text":format!("The nested instance {} is missing or retired.",o["slots"][slot][&name]["id"])}}]);
                }
            }

            cite(
                t,
                "setup-declaration",
                &[
                    "CON-004", "ACC-005", "ACC-006", "LIF-008", "POOL-013", "POOL-015",
                ],
                &o["id"],
            );
            if outcome != "succeeded" && !optional {
                return settle(t, a, c, i);
            }
        }
        t.state.invocations[i]["setup"]["phase"] = json!("implementations");
    }
    if t.state.invocations[i]["setup"]["phase"] == "implementations" {
        let items = o["resolved_implementations"].as_object();
        let run = items
            .into_iter()
            .flat_map(|m| m.values())
            .find(|x| x["item"].get("command").is_some() || x["item"].get("package").is_some())
            .cloned();
        let mut remotes: Vec<Value> = vec![];
        for x in items
            .into_iter()
            .flat_map(|m| m.values())
            .filter(|x| x["item"]["remote"].get("headers").is_some())
        {
            if !remotes.contains(x) {
                remotes.push(x.clone());
            }
        }
        t.state.invocations[i]["setup"]["phase"] = json!("work");
        t.state.invocations[i]["setup"]["connections"] = json!({});
        t.state.invocations[i]["setup"]["run_phase"] = json!("ended");
        if let Some(run) = run {
            t.state.invocations[i]["setup"]["item"] = run;
            t.state.invocations[i]["setup"]["run_phase"] = json!("inspect-run");
            let runtime_unavailable = o
                .get("runtime")
                .is_some_and(|r| access::object(&t.state, r).is_none_or(|v| v["state"] != "ready"));
            if runtime_unavailable {
                part(t, i, "run", "unavailable", true, false, Value::Null);
                t.state.invocations[i]["setup"]["run_phase"] = json!("ended");
            } else {
                emit(t, a, c, i, "inspect-run.v1", json!({}), None)?;
                if t.state.invocations[i]["state"] != "in-progress" {
                    return Ok(());
                }
            }
        }
        for remote in remotes {
            let selection = remote["selection"]
                .as_str()
                .ok_or("remote has no selection")?;
            t.state.invocations[i]["setup"]["connections"][selection] = json!("pending");
            t.external_intents.push(json!({"kind":"connect","part":selection,"invocation":v["id"],"subject":o["id"],"implementation":remote}));
            cite(
                t,
                "connect-setup-item",
                &["TPL-012", "IFC-041", "IMP-002"],
                &o["id"],
            );
        }
    }
    if t.state.invocations[i]["setup"]["phase"] == "work" {
        if t.state.invocations[i]["setup"]["run_phase"] != "ended"
            || t.state.invocations[i]["setup"]["connections"]
                .as_object()
                .into_iter()
                .flat_map(|m| m.values())
                .any(|v| v == "pending")
        {
            t.reply = json!({"pending":true,"invocation":v["id"]});
            return Ok(());
        }
        if t.state.invocations[i]["setup"]["parts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["optional"] != true && p["outcome"] != "succeeded")
        {
            return settle(t, a, c, i);
        }
        t.state.invocations[i]["setup"]["phase"] = json!("account");
    }
    if t.state.invocations[i]["setup"]["phase"] == "account" {
        if matches!(
            sign_in::expected_account(&o, c)?,
            sign_in::ExpectedAccount::Set { .. }
        ) {
            return dispatch_sign_in_part(t, a, c, i, &o, true);
        }
        t.state.invocations[i]["setup"]["phase"] = json!("hook");
    }
    if t.state.invocations[i]["setup"]["phase"] == "hook" {
        let op = c.qualified("setup-hook.v1");
        if let Some(implementation) = o["resolved_implementations"].get(&op) {
            crate::requests::setup_hook(t, a, c, i, &o, implementation)?;
            return Ok(());
        }
        return settle(t, a, c, i);
    }
    Ok(())
}

fn dispatch_sign_in_part(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    object: &Value,
    account: bool,
) -> Result<()> {
    let (operation, phase, name, promises): (_, _, _, &[_]) = if account {
        (
            "account-hook.v1",
            "account-pending",
            "account",
            &["TPL-012", "PND-011", "CTR-013", "ACC-018", "IMP-002"],
        )
    } else {
        (
            "sign-out-hook.v1",
            "undo-pending",
            "sign-out",
            &["TPL-012", "PND-011", "ACC-018", "IMP-002"],
        )
    };
    if requests::setup_sign_in_hook(t, a, c, i, object, operation, promises, None)? {
        t.state.invocations[i]["setup"]["phase"] = json!(phase);
        return Ok(());
    }
    part(t, i, name, "failed", !account, false, Value::Null);
    settle(t, a, c, i)
}
pub(crate) fn emit(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    operation: &str,
    extra: Value,
    secret_bindings: Option<Value>,
) -> Result<()> {
    let v = t.state.invocations[i].clone();
    let o = access::object(&t.state, &v["object"])
        .ok_or("run subject missing")?
        .clone();
    let machine = access::object(&t.state, &o["placement"])
        .map(|n| n["status"].clone())
        .ok_or("run node missing")?;
    let mut inputs = json!({"instance":o["id"],"machine":machine});
    inputs
        .as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    let runtime = o.get("runtime").and_then(|r| access::object(&t.state, r));
    let mut intent = json!({"kind":if runtime.is_some(){"provider-call"}else{"builtin-runner"},"operation":c.qualified(operation),"invocation":v["id"],"caller":v["caller"],"subject":o["id"],"inputs":inputs});
    if operation == "run.v1" && crate::secrets::is_own_runner_provider(&o, c) {
        intent["observeProviderExit"] = json!(true);
    }
    if let Some(bindings) = secret_bindings {
        intent["secretBindings"] = bindings;
    }
    if let Some(runtime) = runtime {
        intent["provider"] = runtime["id"].clone();
        intent["config"] = runtime["config"].clone();
        intent["revision"] = runtime["revision"].clone();
        let mut arguments = intent["inputs"].as_object().unwrap().clone();
        // The engine's identity materialization stays in the runner.
        arguments.remove("identity");
        arguments.remove("secrets");
        let mut facts = json!({
            "target":runtime["id"],"subject":o["id"],"operation":c.qualified(operation),
            "caller":v["caller"],"arguments":arguments
        });
        requests::record_content(t, c, &mut facts)?;
        facts["invocation"] = v["id"].clone();
        t.state.invocations[i]["provider_call"] = facts;
    }
    let provider_call = intent["kind"] == "provider-call";
    if !provider_call || !crate::secrets::restart_if_needed(t, a, c, i, Some(&intent))? {
        t.external_intents.push(intent);
    }
    if t.state.invocations[i]["state"] == "in-progress" {
        t.reply = json!({"pending":true,"invocation":v["id"]});
    }
    cite(
        t,
        "setup-runner-intent",
        &[
            "PRV-003", "PRV-005", "PRV-007", "PRV-022", "TPL-012", "IMP-002",
        ],
        &o["id"],
    );
    Ok(())
}
fn bindings(config: &Value, bindings: &Value, secrets: &mut Value) -> Result<Value> {
    let mut values = json!({});
    for (name, pointer) in bindings.as_object().into_iter().flatten() {
        if let Some(value) = config
            .pointer(pointer.as_str().ok_or("non-pointer run binding")?)
            .filter(|v| !v.is_null())
        {
            values[name] = match value {
                Value::String(s) => json!(s),
                Value::Bool(_) | Value::Number(_) => json!(
                    String::from_utf8(
                        serde_json_canonicalizer::to_vec(value).map_err(|e| e.to_string())?
                    )
                    .map_err(|e| e.to_string())?
                ),
                Value::Object(reference)
                    if reference.len() == 2
                        && value["kind"] == "Resource"
                        && value["id"].is_string() =>
                {
                    // MAN-017 permits only scalar values and secret references at run pointers.
                    secrets[name] = value["id"].clone();
                    continue;
                }
                _ => return Err("non-scalar run binding".into()),
            };
        }
    }
    Ok(values)
}
pub(crate) fn run_inputs(o: &Value, v: &Value, c: &Context) -> Result<(Value, Value)> {
    let item = &v["setup"]["item"]["item"];
    let mut secrets = json!({"environment":{},"packageArguments":{}});
    let environment = bindings(
        &o["config"],
        &item["environment"],
        &mut secrets["environment"],
    )?;
    let arguments = bindings(
        &o["config"],
        &item["packageArguments"],
        &mut secrets["packageArguments"],
    )?;
    let run_schema = &c
        .core_environment
        .scopes
        .values()
        .flat_map(|s| s.contracts.values())
        .find(|d| d["kind"] == "Operation" && d["metadata"]["name"] == "run.v1")
        .ok_or("missing run contract")?["spec"]["inputSchema"]["properties"]["identity"];
    let identity = json!({"materializeFor":v["object"],"environmentNames":run_schema["properties"]["environment"]["required"],"fileNames":run_schema["properties"]["files"]["required"]});
    Ok((
        json!({"item":item,"environment":environment,"packageArguments":arguments,"identity":identity}),
        secrets,
    ))
}
enum ProviderRefusal {
    Discovery,
    NotReached,
    Restart,
}

fn withhold_provider_call(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    operation: &str,
    failure: ProviderRefusal,
) -> Result<()> {
    let v = &t.state.invocations[i];
    let mut facts = v["provider_call"].clone();
    if facts["operation"] != operation || facts["made"] == true || v["cancelled"] == true {
        t.reply = json!({"ignored":true});
        return Ok(());
    }
    let subject = v["object"].clone();
    let provider = facts["target"]
        .as_str()
        .ok_or("provider call has no target")?;
    let (rule, message) = match failure {
        ProviderRefusal::Discovery => {
            let extension = format!("{}implementation-interface", c.meta_prefix);
            let message = format!(
                "Provider {provider} did not advertise {extension} at v0, so the call for instance {subject} was not sent."
            );
            facts["missingExtension"] = json!(extension);
            facts["extensionRevision"] = json!("v0");
            ("IFC-037", message)
        }
        ProviderRefusal::NotReached
            if crate::secrets::provider_setup_has_ambiguous_end(&t.state, c, &facts["target"]) =>
        {
            (
                "PRV-026",
                format!(
                    "Provider {provider} needs explicit setup before a call for instance {subject}. The stored history cannot establish that its run ended at engine shutdown."
                ),
            )
        }
        ProviderRefusal::NotReached => (
            "PRV-026",
            format!(
                "The engine could not reach provider {provider} for instance {subject}, so the call was not sent."
            ),
        ),
        ProviderRefusal::Restart => (
            "PRV-026",
            format!(
                "The engine could not restart provider {provider} for instance {subject}: a required secret read or the provider's run did not succeed. The call was not sent."
            ),
        ),
    };
    facts.as_object_mut().unwrap().remove("inputs");
    facts["phase"] = json!("ended");
    facts["reason"] = json!("unavailable");
    facts["ruleId"] = json!(rule);
    requests::record(t, a, c, "provider-call", facts)?;
    t.state.invocations[i]
        .as_object_mut()
        .unwrap()
        .remove("provider_call");
    cite(
        t,
        "withhold-unavailable-provider-call",
        &[rule, "PRV-005"],
        &subject,
    );
    provider_part_unavailable(
        t,
        a,
        c,
        i,
        json!({"ruleId":rule,"level":"error","message":{"text":message}}),
    )
}

pub(crate) fn provider_restart_failed(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    let operation = t.state.invocations[i]["provider_call"]["operation"].clone();
    withhold_provider_call(
        t,
        a,
        c,
        i,
        operation.as_str().ok_or("provider call has no operation")?,
        ProviderRefusal::Restart,
    )
}

fn provider_part_unavailable(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    diagnostic: Value,
) -> Result<()> {
    if t.state.invocations[i].get("retirement").is_some() {
        crate::retirement::provider_unavailable(t, a, c, i, diagnostic)
    } else {
        finish_unavailable_run(t, a, c, i, Some(diagnostic))
    }
}

pub(crate) fn report(
    t: &mut Transition,
    evidence: &Evidence,
    id: &str,
    report: Report,
    c: &Context,
) -> Result<()> {
    let i = t
        .state
        .invocations
        .iter()
        .position(|v| v["id"] == id)
        .ok_or("report names no setup")?;
    let v = t.state.invocations[i].clone();
    if v["state"] != "in-progress" {
        t.reply = json!({"ignored":true});
        cite(
            t,
            "ignore-settled-setup-report",
            &["OPS-013", "IMP-007"],
            &v["object"],
        );
        return Ok(());
    }
    let mut a = Allocation {
        evidence,
        cursor: 0,
    };
    if let Report::ProviderCallReady { operation } = &report {
        let mut facts = v["provider_call"].clone();
        if facts["operation"] != *operation || facts["made"] == true || v["cancelled"] == true {
            t.reply = json!({"ignored":true});
            return Ok(());
        }
        facts["phase"] = json!("made");
        requests::record(t, &mut a, c, "provider-call", facts)?;
        t.state.invocations[i]["provider_call"]["made"] = json!(true);
        t.reply = json!({"accepted":true});
        return Ok(());
    }
    if let Report::ProviderDiscoveryUnavailable { operation } = &report {
        return withhold_provider_call(t, &mut a, c, i, operation, ProviderRefusal::Discovery);
    }
    if let Report::ProviderCallUnavailable { operation } = &report {
        return withhold_provider_call(t, &mut a, c, i, operation, ProviderRefusal::NotReached);
    }
    let bridge_failure = match &report {
        Report::ProviderRunUnproved { operation } => Some((
            operation,
            "PRV-028",
            "the server did not prove that instance under PRV-028 and IFC-017.",
        )),
        Report::ProviderBridgeUninitialized { operation } => Some((
            operation,
            "IFC-017",
            "the bridge's MCP session could not be initialized.",
        )),
        _ => None,
    };
    if let Some((operation, _, _)) = bridge_failure
        && (v["provider_call"]["operation"] != *operation
            || v["provider_call"]["made"] != true
            || v["cancelled"] == true)
    {
        t.reply = json!({"ignored":true});
        return Ok(());
    }
    let provider_outcome = match &report {
        Report::InspectRun { state } => {
            Some(("inspect-run.v1", "succeeded", json!({"state":state})))
        }
        Report::Run { outcome, url } => Some(("run.v1", outcome.as_str(), json!({"url":url}))),
        Report::Stop { outcome } => Some(("stop.v1", outcome.as_str(), json!({}))),
        Report::ProviderRunUnproved { operation }
        | Report::ProviderBridgeUninitialized { operation } => {
            if *operation == c.qualified("inspect-run.v1") {
                Some(("inspect-run.v1", "succeeded", json!({"state":"running"})))
            } else if *operation == c.qualified("run.v1") {
                Some(("run.v1", "succeeded", json!({})))
            } else {
                return Err("a provider bridge report requires run.v1 or inspect-run.v1".into());
            }
        }
        _ => None,
    };
    if let Some((operation, outcome, outputs)) = provider_outcome
        && v["provider_call"]["operation"] == c.qualified(operation)
    {
        let mut facts = t.state.invocations[i]
            .as_object_mut()
            .unwrap()
            .remove("provider_call")
            .unwrap();
        if facts.as_object_mut().unwrap().remove("made") == Some(json!(true)) {
            facts.as_object_mut().unwrap().remove("inputs");
            facts["phase"] = json!("ended");
            facts["outcome"] = json!(outcome);
            facts["outputs"] = outputs;
            requests::record(t, &mut a, c, "provider-call", facts)?;
        }
    }
    if let Some((operation, rule, message)) = bridge_failure {
        let object = t
            .state
            .objects
            .iter_mut()
            .find(|o| o["id"] == v["object"])
            .ok_or("run subject missing")?;
        observe_run(object, "running", &evidence.time)?;
        // The run succeeded even though the engine cannot use its endpoint.
        // OPS-022 still counts that effect when settling the enclosing change.
        if *operation == c.qualified("run.v1") {
            let part = if v.get("retirement").is_some() {
                "retirement"
            } else {
                "setup"
            };
            t.state.invocations[i][part]["run_effect_observed"] = json!(true);
        }
        if v.get("retirement").is_none() {
            change(
                t,
                &mut a,
                c,
                i,
                &v["object"],
                "setup-part-reported",
                json!({"partOperation":operation,"outcome":"unavailable","runState":"running","ruleId":rule}),
            )?;
        }
        if rule == "PRV-028" {
            cite(
                t,
                "refuse-unproved-provider-run",
                &["PRV-028", "IFC-017", "PRV-009"],
                &v["object"],
            );
        } else {
            cite(
                t,
                "report-uninitialized-provider-bridge",
                &["IFC-017", "PRV-009"],
                &v["object"],
            );
        }
        let diagnostic = json!({"ruleId":rule,"level":"error","message":{
            "text":format!("The runtime provider reported a running process for instance {}, but {message}",v["object"].as_str().unwrap())
        }});
        return provider_part_unavailable(t, &mut a, c, i, diagnostic);
    }
    if crate::secrets::lifecycle_report(t, &mut a, c, i, &report)? {
        return Ok(());
    }
    if v.get("retirement").is_some() {
        return crate::retirement::report(t, &mut a, c, i, report);
    }
    let phase = v["setup"]["run_phase"].as_str().unwrap_or("ended");
    let report_facts = match &report {
        Report::InspectRun { state } => {
            json!({"operation":c.qualified("inspect-run.v1"),"runState":state})
        }
        Report::Run { outcome, .. } => json!({"operation":c.qualified("run.v1"),"outcome":outcome}),
        Report::Stop { .. } => return Err("stop report for setup".into()),
        Report::Connected { part, outcome } => json!({"part":part,"outcome":outcome}),
        _ => unreachable!("provider discovery is handled before setup parts"),
    };
    let mut change_facts = report_facts;
    if let Some(op) = change_facts.as_object_mut().unwrap().remove("operation") {
        change_facts["partOperation"] = op;
    }
    change(
        t,
        &mut a,
        c,
        i,
        &v["object"],
        "setup-part-reported",
        change_facts,
    )?;
    cite(
        t,
        "record-setup-part-report",
        &["REC-001", "REC-002", "REC-003", "PRV-009"],
        &v["object"],
    );
    match report {
        Report::InspectRun { state } if phase == "inspect-run" => {
            if !["running", "stopped", "unknown"].contains(&state.as_str()) {
                return Err("invalid run state".into());
            }
            let o = t
                .state
                .objects
                .iter_mut()
                .find(|o| o["id"] == v["object"])
                .ok_or("run subject missing")?;
            observe_run(o, &state, &evidence.time)?;
            cite(
                t,
                "observe-run",
                &["PRV-007", "PRV-009", "LIF-011", "OBS-002", "OBS-003"],
                &v["object"],
            );
            if state == "stopped"
                && v["cancelled"] != true
                && access::object(&t.state, &v["object"]).unwrap()["state"] != "retired"
            {
                let o = access::object(&t.state, &v["object"]).unwrap();
                let selected = &v["setup"]["item"];
                let relative = selected["selection"].as_str().and_then(|q| q.rsplit_once('#')).is_some_and(|(_, name)| {
                    o["held"]["contexts"][selected["scope"].as_str().unwrap_or("")]["relative_programs"]
                        .as_array().is_some_and(|names| names.contains(&json!(name)))
                });
                if o.get("runtime").is_some() && relative {
                    part(t, i, "run", "failed", true, false, Value::Null);
                    t.state.invocations[i]["setup"]["run_phase"] = json!("ended");
                    cite(
                        t,
                        "refuse-relative-program-provider-run",
                        &["PRV-007"],
                        &v["object"],
                    );
                    advance(t, &mut a, c, i)?;
                    if t.state.invocations[i]["state"] != "in-progress" {
                        child_ended(t, &mut a, c, i)?;
                    }
                    return Ok(());
                }
                return crate::secrets::prepare_run(t, &mut a, c, i);
            }
            part(
                t,
                i,
                "run",
                if state == "running" {
                    "succeeded"
                } else {
                    "unavailable"
                },
                true,
                false,
                Value::Null,
            );
            t.state.invocations[i]["setup"]["run_phase"] = json!("ended");
        }
        Report::Run { outcome, url } if phase == "run" => {
            if !["succeeded", "failed", "unknown"].contains(&outcome.as_str()) {
                return Err("invalid run outcome".into());
            }
            let runtime = access::object(&t.state, &v["object"])
                .unwrap()
                .get("runtime")
                .is_some();
            let outcome = if outcome == "succeeded" && runtime && url.is_none() {
                "unknown"
            } else {
                outcome.as_str()
            };
            if outcome != "failed" {
                let object = t
                    .state
                    .objects
                    .iter_mut()
                    .find(|o| o["id"] == v["object"])
                    .ok_or("run subject missing")?;
                observe_run(
                    object,
                    if outcome == "succeeded" {
                        "running"
                    } else {
                        "unknown"
                    },
                    &evidence.time,
                )?;
            }
            part(t, i, "run", outcome, true, false, Value::Null);
            t.state.invocations[i]["setup"]["run_phase"] = json!("ended");
        }
        Report::Connected {
            part: selection,
            outcome,
        } if v["setup"]["connections"][&selection] == "pending" => {
            if !["succeeded", "failed", "unknown"].contains(&outcome.as_str()) {
                return Err("invalid connection outcome".into());
            }
            part(t, i, &selection, &outcome, false, false, Value::Null);
            t.state.invocations[i]["setup"]["connections"][&selection] = json!(outcome);
        }
        _ => return Err("report does not match outstanding setup part".into()),
    }
    advance(t, &mut a, c, i)?;
    if t.state.invocations[i]["state"] != "in-progress" {
        child_ended(t, &mut a, c, i)?;
    }
    Ok(())
}
pub(crate) fn run_unavailable(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    finish_unavailable_run(t, a, c, i, None)
}

fn finish_unavailable_run(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    diagnostic: Option<Value>,
) -> Result<()> {
    let effect = t.state.invocations[i]["setup"]["run_effect_observed"] == true;
    part(t, i, "run", "unavailable", effect, false, Value::Null);
    if let Some(diagnostic) = diagnostic {
        t.state.invocations[i]["setup"]["parts"]
            .as_array_mut()
            .unwrap()
            .last_mut()
            .unwrap()["diagnostics"] = json!([diagnostic]);
    }
    t.state.invocations[i]["setup"]["run_phase"] = json!("ended");
    advance(t, a, c, i)?;
    if t.state.invocations[i]["state"] != "in-progress" {
        child_ended(t, a, c, i)?;
    }
    Ok(())
}

pub(crate) fn record_fault_end(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    let v = t.state.invocations[i].clone();
    if v.get("provider_restart").is_some() {
        return crate::secrets::restart_ended(t, a, c, i, "unknown");
    }
    if v.get("setup").is_some() || v.get("retirement").is_some() {
        crate::secrets::cancel(t, a, c, i)?;
        t.external_intents
            .push(json!({"kind":"cancel-setup-parts","invocation":v["id"],"subject":v["object"]}));
        if v.get("setup").is_some() {
            t.state.invocations[i]["setup"]["phase"] = json!("ended");
            if let Some(object) = t.state.objects.iter_mut().find(|o| o["id"] == v["object"]) {
                object["status"]["setup"] =
                    json!({"invocation":v["id"],"outcome":"unknown","parts":v["setup"]["parts"]});
            }
        }
        if let Some(id) = v.get("retirement_of")
            && let Some(object) = t.state.objects.iter_mut().find(|o| o["id"] == *id)
        {
            object["shutdown"] = json!("unknown");
        }
    }
    if v.get("sign_in").is_some() {
        t.state.invocations[i]["sign_in"]["phase"] = json!("ended");
    }
    requests::finish_record_fault(t, a, c, i)?;
    let own_reply = t.reply.clone();
    let children: Vec<_> = t
        .state
        .invocations
        .iter()
        .enumerate()
        .filter(|(_, child)| child["setup_parent"] == v["id"] && child["state"] == "in-progress")
        .map(|(child, _)| child)
        .collect();
    for child in children {
        let entry = &t.state.invocations[child];
        t.external_intents
            .push(json!({"kind":"cancel","invocation":entry["id"],"attempt":entry["attempt"]}));
        record_fault_end(t, a, c, child)?;
    }
    t.reply = own_reply;
    child_ended(t, a, c, i)
}

pub(crate) fn child_ended(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    child: usize,
) -> Result<()> {
    let v = t.state.invocations[child].clone();
    let outcome = v["state"].as_str().unwrap();
    child_answered(t, a, c, &v, outcome)
}
pub(crate) fn child_answered(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    v: &Value,
    outcome: &str,
) -> Result<()> {
    let Some(parent) = v.get("setup_parent") else {
        return Ok(());
    };
    let i = t
        .state
        .invocations
        .iter()
        .position(|v| v["id"] == *parent)
        .ok_or("missing setup parent")?;
    if t.state.invocations[i]["state"] != "in-progress" {
        return Ok(());
    }
    if t.state.invocations[i]["operation"] == c.qualified("sign-in.v1") {
        return crate::sign_in::part_answered(t, a, c, i, v, outcome);
    }
    if t.state.invocations[i].get("retirement").is_some() {
        if t.state.invocations[i]["retirement"]["phase"] == "hook" {
            return crate::retirement::hook_ended(t, a, c, i, outcome);
        }
        return Ok(());
    }
    let phase = t.state.invocations[i]["setup"]["phase"]
        .as_str()
        .unwrap_or("")
        .to_owned();
    if matches!(phase.as_str(), "account-pending" | "undo-pending") {
        sign_in_part_answered(t, a, c, i, v, outcome, phase == "account-pending")?;
        if t.state.invocations[i]["state"] != "in-progress" {
            child_ended(t, a, c, i)?;
        }
        return Ok(());
    }
    if phase != "hook-pending" && phase != "entry" {
        return Ok(());
    }
    let waiting = t.state.invocations[i]["setup"]["waiting"].clone();
    part(
        t,
        i,
        if phase == "entry" {
            waiting["name"].as_str().unwrap()
        } else {
            "hook"
        },
        outcome,
        v["contract"]["spec"].get("effect").is_some(),
        waiting["optional"] == true,
        v["id"].clone(),
    );
    if phase == "hook-pending" || outcome != "succeeded" && waiting["optional"] != true {
        settle(t, a, c, i)?;
    } else {
        t.state.invocations[i]["setup"]["phase"] = json!("declarations");
        advance(t, a, c, i)?;
    }
    if t.state.invocations[i]["state"] != "in-progress" {
        child_ended(t, a, c, i)?;
    }
    Ok(())
}

fn raise_setup_sign_in(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    i: usize,
    instance: &Value,
    setup: &Value,
    (pointer, account): (&str, &str),
    trigger: &str,
) -> Result<()> {
    if let Some(action) = pending::raise_sign_in(
        &mut t.state,
        instance,
        account,
        pointer,
        pending::SignInRequest::Setup {
            invocation: setup,
            trigger,
        },
        a,
    )? {
        t.state.invocations[i]["setup"]["raised"] = action["id"].clone();
        t.decisions.push(decision(
            "raise-sign-in-action",
            &["PND-001", "PND-005", "PND-011", "PND-012", "TPL-012"],
            &["DL19"],
            json!({"action":action["id"],"instance":instance["id"]}),
            vec![],
        ));
    }
    Ok(())
}

fn sign_in_part_answered(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    child: &Value,
    outcome: &str,
    account_check: bool,
) -> Result<()> {
    part(
        t,
        i,
        if account_check { "account" } else { "sign-out" },
        outcome,
        child["contract"]["spec"].get("effect").is_some(),
        false,
        child["id"].clone(),
    );
    let setup = t.state.invocations[i].clone();
    let instance = access::object(&t.state, &setup["object"])
        .ok_or("setup object disappeared")?
        .clone();
    let retired = instance["state"] == "retired";
    if account_check && matches!(outcome, "failed" | "unknown") && !retired {
        if let sign_in::ExpectedAccount::Set { pointer, account } =
            sign_in::expected_account(&instance, c)?
        {
            raise_setup_sign_in(
                t,
                a,
                i,
                &instance,
                &setup,
                (&pointer, account),
                &format!("setup-check-{outcome}"),
            )?;
        }
        t.state.invocations[i]["setup"]["accountCounted"] = json!(outcome);
    } else if outcome != "succeeded" {
        if !account_check {
            report_sign_in_undone(
                t,
                &instance["id"],
                outcome,
                &child["id"],
                &a.evidence.time,
                false,
            )?;
            t.state.invocations[i]["setup"]["signOut"] = json!(outcome);
        }
        if !retired {
            return settle(t, a, c, i);
        }
    } else if account_check {
        let expected = sign_in::expected_account(&instance, c)?;
        let found = child["retained"]["outputs"]["account"].as_str();
        let matches_expected = matches!(&expected, sign_in::ExpectedAccount::Set { account, .. }
            if found.is_some_and(|found| found.eq_ignore_ascii_case(account)));
        if retired
            && found.is_some()
            && !matches_expected
            && matches!(&expected, sign_in::ExpectedAccount::Set { .. })
        {
            // TPL-012 bars dispatch after retirement, but OPS-022 still counts this required undo.
            part(t, i, "sign-out", "failed", true, false, Value::Null);
        }
        if let sign_in::ExpectedAccount::Set { pointer, account } = &expected
            && !matches_expected
            && !retired
        {
            let trigger = if found.is_some() {
                "setup-other-account"
            } else {
                "setup-no-account"
            };
            raise_setup_sign_in(t, a, i, &instance, &setup, (pointer, account), trigger)?;
            if found.is_some() {
                return dispatch_sign_in_part(t, a, c, i, &instance, false);
            }
        }
        if (matches_expected
            || found.is_none()
                && (retired || matches!(expected, sign_in::ExpectedAccount::Set { .. })))
            && let Some(object) = t
                .state
                .objects
                .iter_mut()
                .find(|o| o["id"] == instance["id"])
            && sign_in::clear_sign_in_undone(object, &a.evidence.time, "setup")
        {
            t.decisions.push(decision(
                "clear-sign-in-undone",
                &["PND-011", "TPL-012"],
                &[],
                json!({"instance":instance["id"]}),
                vec![],
            ));
        }
    }
    t.state.invocations[i]["setup"]["phase"] = json!("hook");
    advance(t, a, c, i)
}

pub(crate) fn cancel(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    crate::secrets::cancel(t, a, c, i)?;
    if t.state.invocations[i].get("retirement").is_some() {
        return crate::retirement::cancel(t, a, c, i);
    }
    let v = t.state.invocations[i].clone();
    t.state.invocations[i]["cancelled"] = json!(true);
    let children: Vec<_> = t
        .state
        .invocations
        .iter()
        .enumerate()
        .filter(|(_, ch)| ch["setup_parent"] == v["id"] && ch["state"] == "in-progress")
        .map(|(i, _)| i)
        .collect();
    for child in children {
        if t.state.invocations[child].get("setup").is_some() {
            cancel(t, a, c, child)?;
            let child = t.state.invocations[child].clone();
            part(
                t,
                i,
                v["setup"]["waiting"]["name"].as_str().unwrap(),
                child["state"].as_str().unwrap(),
                child["contract"]["spec"].get("effect").is_some(),
                v["setup"]["waiting"]["optional"] == true,
                child["id"].clone(),
            );
        } else {
            t.external_intents.push(json!({"kind":"cancel","invocation":t.state.invocations[child]["id"],"attempt":t.state.invocations[child]["attempt"]}));
            let cause = explain::outcome_cause(&t.state.invocations[child], "unknown", "OPS-027")?;
            requests::finish(
                t,
                a,
                c,
                child,
                "unknown",
                json!({}),
                json!([]),
                Some(vec![cause]),
            )?;
            part(
                t,
                i,
                "cancelled-hook",
                "unknown",
                v["setup"]["phase"] != "account-pending",
                false,
                t.state.invocations[child]["id"].clone(),
            );
            if v["setup"]["phase"] == "undo-pending" {
                let hook = t.state.invocations[child]["id"].clone();
                report_sign_in_undone(t, &v["object"], "unknown", &hook, &a.evidence.time, false)?;
                t.state.invocations[i]["setup"]["signOut"] = json!("unknown");
            }
        }
    }
    if v["setup"]["phase"] == "work" {
        t.external_intents
            .push(json!({"kind":"cancel-setup-parts","invocation":v["id"],"subject":v["object"]}));
        part(
            t,
            i,
            "cancelled-work",
            "unknown",
            v["setup"]["run_phase"] == "run",
            false,
            Value::Null,
        );
    } else {
        part(t, i, "cancelled", "failed", false, false, Value::Null);
    }
    cite(
        t,
        "cancel-setup-parts",
        &["OPS-027", "TPL-012", "LIF-027"],
        &v["object"],
    );
    settle(t, a, c, i)
}

// Keep ended observations internally; CLI inspection can describe their history
// separately without extending the specification's current-condition schema.
pub(crate) fn observe_run(object: &mut Value, state: &str, time: &str) -> Result<()> {
    let status = match state {
        "running" => "True",
        "stopped" => "False",
        _ => "Unknown",
    };
    let source = object
        .get("runtime")
        .map(|r| format!("urn:uuid:{}", access::id(r).as_str().unwrap()))
        .unwrap_or_else(|| "urn:conduit:builtin-runner".into());
    if !object["status"]["conditions"].is_array() {
        object["status"]["conditions"] = json!([]);
    }
    let previous = object["status"]["conditions"]
        .as_array()
        .unwrap()
        .iter()
        .position(|p| p["type"] == "Running");
    let old = previous
        .map(|p| object["status"]["conditions"][p].clone())
        .unwrap_or(Value::Null);
    let changed = old["status"] != status;
    if changed && !old.is_null() {
        if !object["past_conditions"].is_array() {
            object["past_conditions"] = json!([]);
        }
        object["past_conditions"]
            .as_array_mut()
            .unwrap()
            .push(json!({"condition":old,"endedAt":time}));
    }
    let transition = if changed {
        json!(time)
    } else {
        old["lastTransitionTime"].clone()
    };
    let mut condition = json!({"type":"Running","status":status,"reason":match state {"running"=>"RunRunning","stopped"=>"RunStopped",_=>"RunUnknown"},"message":format!("The run is {state}."),"lastTransitionTime":transition,"source":source});
    if state != "unknown" {
        condition["lastHeartbeatTime"] = json!(time);
    } else if let Some(time) = old.get("lastHeartbeatTime") {
        condition["lastHeartbeatTime"] = time.clone();
    }
    let conditions = object["status"]["conditions"].as_array_mut().unwrap();
    if let Some(p) = previous {
        conditions[p] = condition;
    } else {
        conditions.push(condition);
    }
    Ok(())
}

#[cfg(test)]
mod observation_tests {
    use super::*;
    #[test]
    fn ended_run_condition_is_past_and_running_is_current() {
        let mut object = json!({"status":{},"state":"ready"});
        observe_run(&mut object, "stopped", "2026-11-10T00:00:00Z").unwrap();
        observe_run(&mut object, "running", "2026-11-10T00:00:01Z").unwrap();
        assert_eq!(object["status"]["conditions"][0]["reason"], "RunRunning");
        assert_eq!(
            object["past_conditions"][0]["condition"]["reason"],
            "RunStopped"
        );
        assert_eq!(
            object["past_conditions"][0]["endedAt"],
            "2026-11-10T00:00:01Z"
        );
        observe_run(&mut object, "running", "2026-11-10T00:00:02Z").unwrap();
        assert_eq!(
            object["status"]["conditions"][0]["lastTransitionTime"],
            "2026-11-10T00:00:01Z"
        );
        assert_eq!(object["past_conditions"].as_array().unwrap().len(), 1);
        observe_run(&mut object, "unknown", "2026-11-10T00:00:03Z").unwrap();
        assert_eq!(object["status"]["conditions"][0]["status"], "Unknown");
        assert_eq!(
            object["status"]["conditions"][0]["lastHeartbeatTime"],
            "2026-11-10T00:00:02Z"
        );
        assert_eq!(object["state"], "ready");
    }
}

#[cfg(test)]
mod explanation_tests {
    use super::*;
    use crate::{
        Input,
        explain::tests::{assert_cause, evidence, fixture, transition},
        step,
    };

    fn setup_fixture(phase: &str) -> (State, Context) {
        let (mut state, c) = fixture();
        state.invocations.push(json!({
            "id":"setup","state":"in-progress","target":"management","object":"owners",
            "caller":"root","sender":"root","operation":c.qualified("set-up.v1"),"attempt":1,
            "setup":{"phase":phase,"run_phase":"run","cursor":0,"parts":[]}
        }));
        (state, c)
    }

    #[test]
    fn setup_result_names_the_sign_in_action_it_raised() {
        let (mut state, c) = setup_fixture("declarations");
        state.invocations[0]["object"] = json!("agent");
        let mut environment = c.core_environment.clone();
        let contracts = &mut environment.scopes.get_mut("core").unwrap().contracts;
        contracts.get_mut("type.v1").unwrap()["spec"]["signIn"] = json!({"account":"/account"});
        contracts.insert(
            "account-hook.v1".into(),
            json!({"kind":"Operation","metadata":{"name":"account-hook.v1"},
                "spec":{"outputSchema":{"type":"object","required":["account"],
                    "properties":{"account":{"type":["string","null"]}}}}}),
        );
        let indexes = json!({"core":contracts});
        let agent = &mut state.objects[1];
        agent["held"] =
            json!({"environment":environment,"indexes":indexes,"contexts":{},"scope":"core"});
        agent["config"] = json!({"account":"expected-account"});
        agent["resolved_implementations"][c.qualified("account-hook.v1")] =
            json!({"source":"project","item":{}});

        let mut pending = transition(state);
        let fresh = evidence();
        let mut allocation = Allocation {
            evidence: &fresh,
            cursor: 0,
        };
        advance(&mut pending, &mut allocation, &c, 0).unwrap();
        let hook = pending.reply["hook"].as_str().unwrap().to_owned();
        let complete = |state: &State, completion| {
            let mut fresh = evidence();
            fresh.ids.retain(|id| !state.allocated_ids.contains(id));
            step(
                state,
                Input::Complete {
                    evidence: fresh,
                    invocation: hook.clone(),
                    attempt: 1,
                    completion,
                },
                &c,
            )
            .unwrap()
        };
        let ready = complete(&pending.state, requests::advertised_extension(&c));
        for account in [json!("expected-account"), Value::Null] {
            let ended = complete(
                &ready.state,
                requests::Completion::Result {
                    result: json!({"isError":false,"content":[],"_meta":{"test/outcome":"succeeded"},
                        "structuredContent":{"account":account}}),
                },
            );
            assert_eq!(ended.reply["_meta"]["test/outcome"], "succeeded");
            let stored = ended
                .request_replies
                .iter()
                .find(|reply| reply.invocation == "setup")
                .unwrap();
            if account.is_null() {
                let record = ended
                    .records
                    .iter()
                    .find(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == "setup")
                    .unwrap();
                let action = &record["data"]["pendingAction"];
                let id = action[0].as_str().unwrap();
                assert_eq!(action, &json!([id]));
                assert!(ended.state.pending_actions.contains_key(id));
                assert_eq!(&ended.reply["_meta"]["test/pendingAction"], action);
                assert_eq!(&stored.reply["_meta"]["test/pendingAction"], action);
            } else {
                assert!(ended.state.pending_actions.is_empty());
                assert!(ended.reply["_meta"].get("test/pendingAction").is_none());
                assert!(stored.reply["_meta"].get("test/pendingAction").is_none());
            }
        }
    }

    fn cancel_setup(state: &State, c: &Context) -> Transition {
        step(
            state,
            Input::Cancel {
                evidence: evidence(),
                invocation: "setup".into(),
                confirmation: None,
            },
            c,
        )
        .unwrap()
    }

    #[test]
    fn setup_cancellation_names_ops022_in_work_and_before_work() {
        for (phase, part, outcome) in [
            ("work", "cancelled-work", "unknown"),
            ("declarations", "cancelled", "failed"),
        ] {
            let (state, c) = setup_fixture(phase);
            let ended = cancel_setup(&state, &c);
            assert_eq!(ended.reply["_meta"]["test/outcome"], outcome);
            assert_cause(
                &ended.reply,
                "OPS-022",
                &format!("The invocation was cancelled. Required part {part} ended {outcome}."),
            );
        }
    }

    #[test]
    fn setup_failure_names_first_required_part_without_cancellation() {
        let (mut state, c) = setup_fixture("work");
        let parts = &mut state.invocations[0]["setup"]["parts"];
        *parts = json!([
            {"name":"optional","outcome":"failed","optional":true},
            {"name":"required","outcome":"unavailable"},
            {"name":"later","outcome":"failed"}
        ]);
        let mut t = transition(state);
        let evidence = evidence();
        let mut a = Allocation {
            evidence: &evidence,
            cursor: 0,
        };
        settle(&mut t, &mut a, &c, 0).unwrap();
        assert_cause(
            &t.reply,
            "OPS-022",
            "Required part required ended unavailable.",
        );
        assert!(
            !t.reply["structuredContent"]
                .to_string()
                .contains("cancelled")
        );
    }

    #[test]
    fn setup_outcome_names_the_requested_reference_without_resolving_it() {
        let (mut state, c) = fixture();
        let operation = c.qualified("set-up.v1");
        let management = &mut state.objects[3];
        management["schema_environment"]["scopes"]["core"]["contracts"]["capability.v1"] =
            json!({"kind":"Capability","spec":{"operations":[operation]}});
        management["resolved_implementations"][&operation] = json!({"source":"engine","item":{}});
        let path = state
            .objects
            .iter_mut()
            .find(|o| o["id"] == "agent-management")
            .unwrap();
        path["capability"] = json!(c.qualified("capability.v1"));
        state.objects[5]["name"] = json!("resolved-pool");
        state.objects[5]["entries"] = json!({"missing":{}});
        state.objects[1]["config"]["pool"] = json!({"kind":"Pool","id":"owners"});
        state.invocations.push(json!({
            "id":"serving-hook","state":"in-progress","target":"agent",
            "caller":"root","hook":true
        }));

        let ended = step(
            &state,
            Input::Invoke {
                evidence: evidence(),
                request: access::AccessRequest {
                    identity: access::Identity::Instance { id: "agent".into() },
                    tool: "management.set-up.v1".into(),
                    arguments: json!({"instance":state.objects[1]["config"]["pool"]}),
                    serving: Some("serving-hook".into()),
                    readable: true,
                    authority_readable: true,
                    client_capabilities: None,
                    sign_in_login: None,
                    sign_in_return: None,
                },
                key: None,
                archive: None,
            },
            &c,
        )
        .unwrap();
        assert_cause(
            &ended.reply,
            "OPS-022",
            "test.core#set-up.v1 for Pool/owners via tool management.set-up.v1 ended failed under OPS-022: Required part missing ended conflict.",
        );
        assert!(!ended.reply.to_string().contains("resolved-pool"));
    }

    #[test]
    fn cancelling_setup_ends_hook_under_ops027() {
        let (mut state, c) = setup_fixture("hook-pending");
        state.invocations.push(json!({
            "id":"hook","state":"in-progress","setup_parent":"setup","attempt":1,
            "sender":"root","caller":"root","target":"owners","operation":c.qualified("set-up-hook.v1")
        }));
        let t = cancel_setup(&state, &c);
        let hook = &t
            .request_replies
            .iter()
            .find(|r| r.invocation == "hook")
            .unwrap()
            .reply;
        assert_eq!(hook["_meta"]["test/outcome"], "unknown");
        assert_cause(hook, "OPS-027", "The invocation was cancelled");
        assert_cause(
            &t.reply,
            "OPS-022",
            "The invocation was cancelled. Required part cancelled-hook ended unknown.",
        );
    }

    #[test]
    fn held_back_pool_part_names_only_its_unmet_dependencies() {
        let (mut state, c) = setup_fixture("declarations");
        let pool = &mut state.objects[5];
        pool["entries"] = json!({
            "base-a":{"optional":true},
            "base-b":{"optional":true},
            "dependent":{"references":[{"kind":"Resource","id":"a"},{"kind":"Resource","id":"b"}]}
        });
        pool["slots"]["entries"] = json!({"base-a":{"id":"a"},"base-b":{"id":"b"}});
        state.objects.push(json!({
            "kind":"Resource","id":"a","name":"base-a","state":"ready",
            "nested":{"parent":"owners","slot":"entries","name":"base-a"}
        }));
        let mut t = transition(state);
        let evidence = evidence();
        let mut a = Allocation {
            evidence: &evidence,
            cursor: 0,
        };
        advance(&mut t, &mut a, &c, 0).unwrap();
        let parts = &t.state.invocations[0]["setup"]["parts"];
        assert_eq!(parts[0]["outcome"], "succeeded");
        assert_eq!(parts[1]["diagnostics"][0]["ruleId"], "CON-004");
        assert_eq!(parts[2]["outcome"], "unavailable");
        assert_eq!(
            parts[2]["diagnostics"],
            json!([{
                "ruleId":"POOL-013","level":"error",
                "message":{"text":"Setup part dependent was held back pending successful setup of base-b."}
            }])
        );
        assert_eq!(*parts, t.state.objects[5]["status"]["setup"]["parts"]);
        assert!(t.reply["structuredContent"].get("parts").is_none());
    }

    fn refused_part(child_request: bool, sender: &str) -> Transition {
        let (mut state, mut c) = fixture();
        state.objects[0]["conduits"] = json!({"child":{}});
        state.objects[0]["slots"] = json!({"conduits":{"child":{"id":"child"}}});
        state.objects.push(json!({"kind":"Conduit","id":"child","name":"child","consumer":"root","target":"management","state":"registered","open":false,"operations":[],"nested":{"parent":"root","slot":"conduits","name":"child"}}));
        if child_request {
            // A candidate request exists, but the target's operation contract cannot resolve.
            state.objects[4]["schema_environment"]["scopes"]["core"]["contracts"]
                .as_object_mut()
                .unwrap()
                .remove("open.v1");
            state.objects[4].as_object_mut().unwrap().remove("type");
        } else {
            let access_path = format!("{sender}-access");
            state.objects.retain(|o| o["id"] != access_path);
        }
        // Supply the subject invocation directly to exercise each setup-part branch.
        state.invocations.push(json!({"id":"setup","state":"in-progress","target":"management","object":"root","caller":"root","sender":sender,"conduit":format!("{sender}-management"),"operation":c.qualified("set-up.v1"),"arguments":{"instance":"root"},"attempt":1,"contract":{"spec":{}},"setup":{"phase":"declarations","cursor":0,"parts":[]}}));
        c.templates.clear();
        let mut t = transition(state);
        let e = evidence();
        let mut a = Allocation {
            evidence: &e,
            cursor: 0,
        };
        advance(&mut t, &mut a, &c, 0).unwrap();
        crate::explain::first_text(&mut t.reply, &c);
        let part = &t.state.invocations[0]["setup"]["parts"][0];
        assert_eq!(part["outcome"], "refused");
        assert_eq!(part["diagnostics"][0]["ruleId"], "ACC-001");
        assert_eq!(t.state.objects[0]["status"]["setup"]["parts"][0], *part);
        assert_cause(&t.reply, "OPS-022", "Required part child ended refused.");
        assert!(
            t.reply["content"][0]["text"]
                .as_str()
                .unwrap()
                .starts_with("failed: ")
        );
        t
    }
    #[test]
    fn setup_part_without_request_has_operation_but_no_tool_and_records_rule() {
        let t = refused_part(false, "root");
        let r =
            &t.state.invocations[0]["setup"]["parts"][0]["diagnostics"][0]["properties"]["refusal"];
        assert_eq!(r["operation"], "test.core#open.v1");
        assert!(r["named"].get("tool").is_none());
        assert_eq!(r["tier"], 2);
        let records: Vec<_> = t
            .records
            .iter()
            .filter(|r| r["data"]["reason"] == "refused")
            .collect();
        assert_eq!(records.len(), 2);
        for r in records {
            assert_eq!(r["data"]["ruleId"], "ACC-001");
            assert_eq!(r["data"]["refusalTier"], 2);
        }
    }
    #[test]
    fn setup_part_without_request_uses_the_sender_for_disclosure() {
        let t = refused_part(false, "agent");
        let refusal =
            &t.state.invocations[0]["setup"]["parts"][0]["diagnostics"][0]["properties"]["refusal"];
        assert_eq!(refusal["tier"], 1);
        for key in ["target", "exists", "conduits"] {
            assert!(refusal.get(key).is_none());
        }
        for event in ["access-decision", "request-ended"] {
            let record = t.records.iter().find(|r| r["type"] == event).unwrap();
            assert_eq!(record["data"]["ruleId"], "ACC-001");
            assert_eq!(record["data"]["caller"], "root");
            assert_eq!(record["data"]["consumer"], "agent");
            assert_eq!(record["data"]["refusalTier"], refusal["tier"]);
        }
    }
    #[test]
    fn setup_part_copies_child_protocol_error_diagnostics_and_reason() {
        let t = refused_part(true, "root");
        let r =
            &t.state.invocations[0]["setup"]["parts"][0]["diagnostics"][0]["properties"]["refusal"];
        assert_eq!(r["named"]["tool"], "access.open.v1");
        assert_eq!(r["named"]["conduit"], "access");
    }
}
