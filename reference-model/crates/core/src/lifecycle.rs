//! Ordered setup parts and Conduit lifecycle. All runner work is an intent; reports are
//! explicit inputs. Pending plans live with their invocation and survive serialization.
use crate::{Allocation, Context, Evidence, Result, State, Transition, access, decision, requests};
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
}
pub(crate) fn handles(op: &str, c: &Context) -> bool {
    [
        "retire.v1",
        "unregister.v1",
        "set-up.v1",
        "open.v1",
        "close.v1",
        "register-replacement.v1",
    ]
    .iter()
    .any(|n| op == c.qualified(n))
}
fn named<'a>(s: &'a State, route: &Value, c: &Context) -> Option<&'a Value> {
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
    if s.invocations
        .iter()
        .any(|v| v["state"] == "in-progress" && v["object"] == o["id"] && v["management"] == true)
    {
        return Ok(Some(("conflict", "REV-014")));
    }
    Ok(None)
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
    let v = t.state.invocations[i].clone();
    let route = json!({"operation":v["operation"],"arguments":v["arguments"]});
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
            let object = t
                .state
                .objects
                .iter_mut()
                .find(|v| v["id"] == o["id"])
                .unwrap();
            object["open"] = json!(true);
            object["state"] = json!("ready");
            change(t, a, c, i, &o["id"], "opened", json!({"state":"ready"}))?;
        }
        cite(
            t,
            "open-conduit",
            &["ACC-006", "CON-023", "LIF-003", "REV-004"],
            &o["id"],
        );
        return requests::finish(t, a, c, i, "succeeded", json!({}), json!([]));
    }
    if op == c.qualified("close.v1") {
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
        return requests::finish(t, a, c, i, "succeeded", json!({}), json!([]));
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
        return requests::finish(t, a, c, i, "succeeded", outputs, json!([]));
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
fn settle(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize) -> Result<()> {
    let v = t.state.invocations[i].clone();
    let parts = v["setup"]["parts"].as_array().unwrap();
    let all = parts
        .iter()
        .all(|p| p["optional"] == true || p["outcome"] == "succeeded");
    let effect = parts
        .iter()
        .any(|p| p["effect"] == true && (p["outcome"] == "succeeded" || p["outcome"] == "unknown"));
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
    requests::finish(t, a, c, i, outcome, json!({}), json!([]))
}
fn request_for(
    t: &Transition,
    parent: &Value,
    target: &Value,
    operation: &str,
    arguments: Value,
    c: &Context,
) -> Result<Option<access::AccessRequest>> {
    let tools = crate::access::publication_state(&t.state, c)?;
    let mut refused = None;
    for p in tools.objects.iter().filter(|p| {
        p["kind"] == "Conduit"
            && p["consumer"] == parent["sender"]
            && p["target"] == *target
            && p["open"] == true
            && p["state"] == "ready"
    }) {
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
            serving: parent["serving"].as_str().map(str::to_owned),
            readable: true,
            authority_readable: true,
        };
        if access::evaluate(&t.state, &request, c, true)?
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
        if declarations_left
            || (phase == "declarations" || phase == "implementations") && implementations
            || hook && ["declarations", "implementations", "work", "hook"].contains(&phase)
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
            let mut outcome = "conflict".to_owned();
            let mut id = Value::Null;
            let dependencies_succeeded = slot != "entries"
                || entry_dependencies(&t.state, &o, &name, &d)
                    .iter()
                    .all(|dependency| {
                        t.state.invocations[i]["setup"]["parts"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .any(|p| p["name"] == *dependency && p["outcome"] == "succeeded")
                    });
            if !dependencies_succeeded {
                outcome = "unavailable".into();
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
                                    .and_then(Value::as_str)
                            })
                            .unwrap_or("refused")
                            .into();
                        if t.reply["pending"] == true {
                            t.state.invocations[i]["setup"]["phase"] = json!("entry");
                            t.state.invocations[i]["setup"]["waiting"] =
                                json!({"id":id,"name":name,"optional":optional});
                            t.reply = json!({"pending":true,"invocation":v["id"]});
                            return Ok(());
                        }
                    } else {
                        outcome = "refused".into();
                        for event in ["access-decision", "request-ended"] {
                            requests::record(
                                t,
                                a,
                                c,
                                event,
                                json!({"target":target,"caller":v["caller"],"consumer":v["sender"],"operation":op,"serving":v["id"],"reason":"refused","dispatched":false}),
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
                if outcome == "conflict" {
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
                emit(t, a, c, i, "inspect-run.v1", json!({}))?;
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
pub(crate) fn emit(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    operation: &str,
    extra: Value,
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
    if let Some(runtime) = runtime {
        intent["provider"] = runtime["id"].clone();
        intent["config"] = runtime["config"].clone();
        intent["revision"] = runtime["revision"].clone();
        requests::record(
            t,
            a,
            c,
            "provider-call",
            json!({"phase":"made","target":runtime["id"],"subject":o["id"],"operation":c.qualified(operation),"invocation":v["id"],"caller":v["caller"],"arguments":intent["inputs"]}),
        )?;
    }
    t.external_intents.push(intent);
    t.reply = json!({"pending":true,"invocation":v["id"]});
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
fn bindings(config: &Value, bindings: &Value) -> Result<Value> {
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
                _ => return Err("non-scalar run binding".into()),
            };
        }
    }
    Ok(values)
}
pub(crate) fn run_inputs(o: &Value, v: &Value, c: &Context) -> Result<Value> {
    let item = &v["setup"]["item"]["item"];
    let environment = bindings(&o["config"], &item["environment"])?;
    let arguments = bindings(&o["config"], &item["packageArguments"])?;
    let run_schema = &c
        .core_environment
        .scopes
        .values()
        .flat_map(|s| s.contracts.values())
        .find(|d| d["kind"] == "Operation" && d["metadata"]["name"] == "run.v1")
        .ok_or("missing run contract")?["spec"]["inputSchema"]["properties"]["identity"];
    let identity = json!({"materializeFor":v["object"],"environmentNames":run_schema["properties"]["environment"]["required"],"fileNames":run_schema["properties"]["files"]["required"]});
    Ok(
        json!({"item":item,"environment":environment,"packageArguments":arguments,"identity":identity}),
    )
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
    };
    if (phase == "inspect-run" && matches!(&report, Report::InspectRun { .. })
        || phase == "run" && matches!(&report, Report::Run { .. }))
        && let Some(o) = access::object(&t.state, &v["object"])
        && let Some(runtime) = o.get("runtime").and_then(|r| access::object(&t.state, r))
    {
        let outcome = match &report {
            Report::InspectRun { .. } => "succeeded",
            Report::Run { outcome, .. } => outcome.as_str(),
            _ => unreachable!(),
        };
        let outputs = match &report {
            Report::InspectRun { state } => json!({"state":state}),
            Report::Run { url, .. } => json!({"url":url}),
            _ => unreachable!(),
        };
        requests::record(
            t,
            &mut a,
            c,
            "provider-call",
            json!({"target":runtime["id"],"subject":o["id"],"operation":report_facts["operation"],"invocation":v["id"],"caller":v["caller"],"phase":"ended","outcome":outcome,"outputs":outputs}),
        )?;
    }
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
                let extra = run_inputs(o, &v, c)?;
                t.state.invocations[i]["setup"]["run_phase"] = json!("run");
                emit(t, &mut a, c, i, "run.v1", extra)?;
                return Ok(());
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

pub(crate) fn cancel(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
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
            requests::finish(t, a, c, child, "unknown", json!({}), json!([]))?;
            part(
                t,
                i,
                "cancelled-hook",
                "unknown",
                true,
                false,
                t.state.invocations[child]["id"].clone(),
            );
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
