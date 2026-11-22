//! The core keeps the routes and lifecycle of reads. The shell transfers accepted
//! values to the built-in runner using read IDs, without putting them in State.
use crate::{
    Allocation, Context, Result, Transition, access, decision, lifecycle, requests, schemas,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub(crate) fn prepare_run(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    let v = &t.state.invocations[i];
    let object = access::object(&t.state, &v["object"]).ok_or("run subject missing")?;
    let (_, bindings) = lifecycle::run_inputs(object, v, c)?;
    let resources: BTreeSet<_> = bindings
        .as_object()
        .unwrap()
        .values()
        .flat_map(|m| m.as_object().unwrap().values().filter_map(Value::as_str))
        .collect();
    if object.get("runtime").is_some() && !resources.is_empty() {
        return unavailable(t, a, c, i);
    }
    if resources.is_empty() {
        return emit_run(t, a, c, i);
    }
    let v = &mut t.state.invocations[i];
    if v.get("retirement").is_some() {
        v["retirement"]["phase"] = json!("read-secrets");
    } else if v.get("provider_restart").is_none() {
        v["setup"]["run_phase"] = json!("read-secrets");
    }
    t.state.invocations[i]["secret_reads"] = json!({"targets":resources,"reads":{}});
    advance(t, a, c, i)
}

pub(crate) fn is_own_runner_provider(object: &Value, c: &Context) -> bool {
    object.get("runtime").is_none()
        && [
            "get-secret.v1",
            "run.v1",
            "list-machines.v1",
            "fetch-package.v1",
        ]
        .into_iter()
        .any(|operation| {
            object["resolved_implementations"]
                .get(c.qualified(operation))
                .is_some()
        })
}

fn emit_run(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize) -> Result<()> {
    let v = t.state.invocations[i].clone();
    let object = access::object(&t.state, &v["object"]).ok_or("run subject missing")?;
    let (inputs, mut bindings) = lifecycle::run_inputs(object, &v, c)?;
    let provider = is_own_runner_provider(object, c);
    for map in bindings.as_object_mut().unwrap().values_mut() {
        for target in map.as_object_mut().unwrap().values_mut() {
            *target = v["secret_reads"]["reads"][target.as_str().unwrap()].clone();
        }
    }
    let secret_bindings = v.get("secret_reads").map(|_| bindings);
    let stored = &mut t.state.invocations[i];
    stored.as_object_mut().unwrap().remove("secret_reads");
    if v.get("provider_restart").is_some() {
        stored["provider_restart"]["phase"] = json!("run");
    } else if v.get("retirement").is_some() {
        stored["retirement"]["phase"] = json!("run");
    } else {
        stored["setup"]["run_phase"] = json!("run");
        if provider {
            // Keep the run ledger keys written by 218 readable after upgrading.
            stored["secret_provider_run"] = json!("asked");
        }
    }
    lifecycle::emit(t, a, c, i, "run.v1", inputs, secret_bindings)
}

fn unavailable(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize) -> Result<()> {
    let v = &mut t.state.invocations[i];
    v.as_object_mut().unwrap().remove("secret_reads");
    if v.get("provider_restart").is_some() {
        return restart_ended(t, a, c, i, "unavailable");
    }
    if v.get("retirement").is_some() {
        return crate::retirement::run_unavailable(t, a, c, i);
    }
    lifecycle::run_unavailable(t, a, c, i)
}

fn advance(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize) -> Result<()> {
    let v = t.state.invocations[i].clone();
    let owner = access::object(&t.state, &v["object"]);
    if v["state"] != "in-progress" || v["cancelled"] == true {
        return Ok(());
    }
    if owner.is_none_or(|o| o["state"] == "retired" && v.get("retirement").is_none()) {
        return unavailable(t, a, c, i);
    }
    if v.get("provider_restart").is_some()
        && !t
            .state
            .invocations
            .iter()
            .any(|waiter| awaiting_restart(waiter, &v["id"]))
    {
        return unavailable(t, a, c, i);
    }
    let Some(target) = v["secret_reads"]["targets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|target| {
            v["secret_reads"]["reads"]
                .get(target.as_str().unwrap())
                .is_none()
        })
    else {
        return emit_run(t, a, c, i);
    };
    let id = a.next(&mut t.state)?;
    let serving = v["provider_restart"].get("setup").unwrap_or(&v["id"]);
    let op = c.qualified("get-secret.v1");
    t.state.invocations.push(json!({
        "id":id, "attempt":1, "state":"in-progress", "operation":op, "sender":v["object"],
        "caller":v["caller"], "serving":serving, "target":target, "arguments":{},
        "secret_read":{"run":v["id"],"phase":"deciding"}
    }));
    let read = t.state.invocations.len() - 1;
    t.state.invocations[i]["secret_reads"]["reads"][target.as_str().unwrap()] = json!(id);
    decide(t, a, c, read)
}

fn decide(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize) -> Result<()> {
    let v = t.state.invocations[i].clone();
    let op = v["operation"].as_str().unwrap();
    let target = access::object(&t.state, &v["target"]);
    let mut paths = t.state.objects.iter().filter(|path| {
        access::open(path)
            && access::id(&path["consumer"]) == &v["sender"]
            && access::id(&path["target"]) == &v["target"]
            && path["operations"]
                .as_array()
                .is_some_and(|ops| ops.contains(&v["operation"]))
    });
    let first = paths.next();
    let multiple = paths.next().is_some();
    let path = first.filter(|_| !multiple);
    let run = find(t, &v["secret_read"]["run"])?;
    let retiring = t.state.invocations[run].get("retirement").is_some();
    let consumer = access::object(&t.state, &v["sender"]);
    let implementation = target
        .and_then(|o| o["resolved_implementations"].get(op))
        .cloned();
    let rejection = if consumer.is_none_or(|o| o["state"] == "retired" && !retiring) {
        Some(("refused", "ACC-039"))
    } else if first.is_none() {
        Some(("refused", "ACC-001"))
    } else if multiple {
        Some(("refused", "SEC-010"))
    } else if target.is_none_or(|o| o["state"] != "ready")
        || implementation
            .as_ref()
            .is_none_or(|imp| imp["item"].get("remote").is_some())
        || target.is_some_and(|o| o.get("runtime").is_some())
    {
        Some(("unavailable", "CON-024"))
    } else {
        None
    };
    let path = path.cloned();
    if let Some(p) = &path {
        t.state.invocations[i]["conduit"] = p["id"].clone();
    } else {
        t.state.invocations[i]
            .as_object_mut()
            .unwrap()
            .remove("conduit");
    }
    if let Some((reason, rule)) = rejection {
        record_decision(t, a, c, i, Some((reason, rule)))?;
        return end(t, a, c, i, reason, Some(rule), None);
    }
    if v["secret_read"]["phase"] != "discovered" && restart_if_needed(t, a, c, i, None)? {
        return Ok(());
    }
    let path = path.unwrap();
    let filters: Vec<_> = path["requirements"]
        .as_object()
        .into_iter()
        .flat_map(|m| m.values())
        .filter(|r| {
            r["requirement"]
                .as_str()
                .is_some_and(|q| !q.starts_with(&format!("{}#", c.core_package)))
        })
        .cloned()
        .collect();
    let object = access::object(&t.state, &v["target"]).unwrap().clone();
    let held = crate::registration::publication_object(&object, c)?;
    let (_, _, contract) = schemas::operation(&held, op, &c.core_package)?;
    let implementation = implementation.unwrap();
    let read = &mut t.state.invocations[i];
    read["filters"] = json!(filters);
    read["plain"] = json!(implementation["item"]["plain"] == true);
    read["implementation"] = implementation;
    read["config"] = object["config"].clone();
    read["revision"] = object["revision"].clone();
    read["held_target"] = held;
    read["contract"] = contract;
    if read["plain"] == true || v["secret_read"]["phase"] == "discovered" {
        read["secret_read"]["phase"] = json!("dispatched");
        record_decision(t, a, c, i, None)?;
        requests::record(
            t,
            a,
            c,
            "invocation-dispatched",
            json!({"invocation":v["id"],"attempt":1}),
        )?;
        t.external_intents
            .push(requests::dispatch_intent(&t.state.invocations[i], false, c));
    } else {
        read["secret_read"]["phase"] = json!("discover");
        t.external_intents.push(requests::discovery_intent(read, c));
    }
    t.reply = json!({"pending":true,"invocation":t.state.invocations[run]["id"]});
    Ok(())
}

fn record_decision(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    refusal: Option<(&str, &str)>,
) -> Result<()> {
    let mut data = json!({"invocation":t.state.invocations[i]["id"],"permitted":refusal.is_none()});
    if let Some((reason, rule)) = refusal {
        data["dispatched"] = json!(false);
        data["reason"] = json!(reason);
        data["ruleId"] = json!(rule);
        if reason == "refused" {
            data["refusalTier"] = json!(refusal_tier(t, c, i)?);
        }
    }
    requests::record(t, a, c, "access-decision", data.clone())?;
    t.decisions.push(decision(
        "decide-secret-read",
        &["SEC-010", "SEC-011", "REC-001"],
        &[],
        data,
        vec![],
    ));
    Ok(())
}

fn refusal_tier(t: &Transition, c: &Context, i: usize) -> Result<u8> {
    crate::explain::tier(&t.state, &read_route(&t.state.invocations[i]), c)
}

fn read_route(read: &Value) -> Value {
    json!({"consumer":read["caller"],"readingConsumer":read["sender"],
        "target":read["target"],"conduit":read["conduit"],"operation":read["operation"]})
}

fn refused_result(
    state: &crate::State,
    c: &Context,
    read: &Value,
    rule: &str,
    restriction: Option<&Value>,
) -> Result<Value> {
    let diagnostic = crate::explain::diagnostic(state, &read_route(read), c, "refused", rule)?;
    let message = diagnostic["message"]["text"].clone();
    let mut result = requests::envelope(
        c,
        None,
        Some("refused"),
        Some(&read["id"]),
        Some(&[diagnostic]),
        None,
        json!([]),
        restriction,
    );
    result["content"][0]["text"] = message;
    Ok(result)
}

fn find(t: &Transition, id: &Value) -> Result<usize> {
    t.state
        .invocations
        .iter()
        .position(|v| v["id"] == *id)
        .ok_or("secret read invocation missing".into())
}

pub(crate) fn complete(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    attempt: u64,
    completion: requests::Completion,
) -> Result<()> {
    let v = t.state.invocations[i].clone();
    if v["state"] != "in-progress" || v["attempt"] != attempt {
        t.reply = json!({"ignored":true});
        return Ok(());
    }
    if v["secret_read"]["phase"] == "discover" {
        let extension = format!("{}implementation-interface", c.meta_prefix);
        if let requests::Completion::Result { result } = completion
            && result["capabilities"]["extensions"][&extension]["revision"] == "v0"
        {
            t.state.invocations[i]["secret_read"]["phase"] = json!("discovered");
            return decide(t, a, c, i);
        }
        t.state.invocations[i]["extensionRevision"] = json!("v0");
        t.state.invocations[i]["requiredCapabilities"] =
            json!({"extensions":{extension:{"revision":"v0"}}});
        record_decision(t, a, c, i, Some(("unavailable", "IFC-037")))?;
        return end(t, a, c, i, "unavailable", Some("IFC-037"), None);
    }
    if v["secret_read"]["phase"] != "dispatched" {
        t.reply = json!({"ignored":true});
        return Ok(());
    }
    let mut outcome = "unknown";
    if let requests::Completion::Result { result } = completion {
        let meta = &result["_meta"];
        let stated = meta.get(format!("{}outcome", c.meta_prefix));
        let reason = meta.get(format!("{}reason", c.meta_prefix));
        if result.get("resultType").is_none_or(|r| r == "complete") {
            if v["plain"] == true {
                outcome = if result["isError"] == true {
                    "failed"
                } else {
                    "succeeded"
                };
            } else if stated.is_none()
                && result["isError"] == true
                && let Some(reason @ ("refused" | "unavailable")) = reason.and_then(Value::as_str)
                && let Some(restriction) = meta.get(format!("{}restriction", c.meta_prefix))
                && v["filters"]
                    .as_array()
                    .is_some_and(|f| f.contains(restriction))
            {
                let rule = if reason == "refused" {
                    "ACC-008"
                } else {
                    "ACC-014"
                };
                t.validated_reason = Some((reason.into(), restriction.clone()));
                return end(t, a, c, i, reason, Some(rule), Some(restriction));
            } else if reason.is_none() {
                outcome = match (stated.and_then(Value::as_str), result["isError"] == true) {
                    (Some("succeeded"), false) => "succeeded",
                    (Some("failed"), true) => "failed",
                    _ => "unknown",
                };
            }
        }
        if outcome == "succeeded" {
            let operation = v["operation"].as_str().unwrap();
            let (env, scope, _) =
                schemas::operation(&v["held_target"], operation, &c.core_package)?;
            let outputs = &result["structuredContent"];
            if !outputs["value"].is_string()
                || !env.valid(
                    &scope,
                    &v["contract"]["spec"]["outputSchema"],
                    outputs,
                    &c.core_package,
                )?
            {
                outcome = "unknown";
            }
        }
    } else if let requests::Completion::ProtocolError { code, .. } = completion {
        if code == -32020 && v["wire_retried"] != true {
            let mut intent = requests::dispatch_intent(&v, false, c);
            intent["kind"] = json!("refresh-tools-and-call");
            t.external_intents.push(intent);
            t.state.invocations[i]["wire_retried"] = json!(true);
            return Ok(());
        }
        if v["plain"] == true {
            outcome = "failed";
        }
    }
    end(t, a, c, i, outcome, None, None)
}

fn end(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    status: &str,
    rule: Option<&str>,
    restriction: Option<&Value>,
) -> Result<()> {
    // Only record the diagnostic's rule and tier. The result belongs to the engine;
    // the enclosing request receives its own ending for the unavailable part.
    let refusal = if status == "refused" {
        let rule = rule.ok_or("refused read has no rule")?;
        Some(refused_result(
            &t.state,
            c,
            &t.state.invocations[i],
            rule,
            restriction,
        )?)
    } else {
        None
    };
    let v = &mut t.state.invocations[i];
    v["state"] = json!(status);
    let mut data = json!({"invocation":v["id"]});
    let run = v["secret_read"]["run"].clone();
    if let Some(rule) = rule {
        data["reason"] = json!(status);
        data["ruleId"] = json!(rule);
        if let Some(refusal) = refusal {
            let diagnostic = &refusal["structuredContent"]["diagnostics"][0];
            data["ruleId"] = diagnostic["ruleId"].clone();
            data["refusalTier"] = diagnostic["properties"]["refusal"]["tier"].clone();
        }
        if let Some(restriction) = restriction {
            data["restriction"] = restriction.clone();
        }
    } else {
        data["outcome"] = json!(status);
    }
    if v["secret_read"]["phase"] == "dispatched" && rule.is_none() {
        requests::record(t, a, c, "invocation-ended", data.clone())?;
    }
    requests::record(t, a, c, "request-ended", data)?;
    let i = find(t, &run)?;
    if t.state.invocations[i]["state"] != "in-progress" {
        return Ok(());
    }
    if status == "succeeded" {
        advance(t, a, c, i)
    } else {
        unavailable(t, a, c, i)
    }
}

pub(crate) fn record_fault(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    answer: Option<&(String, Value)>,
    unsent_after_rename: bool,
) -> Result<()> {
    let v = t.state.invocations[i].clone();
    if unsent_after_rename && v["secret_read"]["phase"] == "dispatched" {
        let diagnostic =
            crate::explain::dispatch_record_read_back(&c.qualified("get-secret.v1"), "SEC-016");
        t.state.invocations[i]["cause"] = json!([diagnostic]);
        let previous_replies = t.request_replies.len();
        end(t, a, c, i, "unavailable", Some("SEC-016"), None)?;
        for reply in &mut t.request_replies[previous_replies..] {
            if let Some(diagnostics) =
                reply.reply["structuredContent"]["diagnostics"].as_array_mut()
            {
                diagnostics.push(diagnostic.clone());
            }
        }
        return Ok(());
    }
    if let Some((reason, restriction)) = answer {
        let rule = if reason == "refused" {
            "ACC-008"
        } else {
            "ACC-014"
        };
        requests::record(
            t,
            a,
            c,
            "invocation-ended",
            json!({
                "invocation":v["id"],"reason":reason,"ruleId":rule,"restriction":restriction
            }),
        )?;
        return end(t, a, c, i, reason, Some(rule), Some(restriction));
    }
    let proposed = t
        .records
        .iter()
        .any(|r| r["type"] == "invocation-dispatched" && r["data"]["invocation"] == v["id"]);
    if v["secret_read"]["phase"] == "dispatched" && !proposed {
        let cause = crate::explain::outcome_cause(&v, "unknown", "REC-007")?;
        t.state.invocations[i]["cause"] = json!([cause]);
        return end(t, a, c, i, "unknown", None, None);
    }
    unrecorded_read(t, a, c, i, "SEC-016")
}

pub(crate) fn unrecorded_read(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    rule: &str,
) -> Result<()> {
    let id = t.state.invocations[i]["id"].clone();
    requests::discard_proposed_dispatch(t, &id);
    record_decision(t, a, c, i, Some(("unavailable", rule)))?;
    end(t, a, c, i, "unavailable", Some(rule), None)
}

fn awaiting_restart(waiter: &Value, restart: &Value) -> bool {
    waiter["state"] == "in-progress"
        && waiter["cancelled"] != true
        && (waiter["secret_read"]["restart"] == *restart
            || waiter["provider_wait"]["restart"] == *restart)
}

fn wait_for_restart(waiter: &mut Value, restart: &Value, call: Option<&Value>) {
    if let Some(call) = call {
        waiter["provider_wait"] = json!({"restart":restart,"intent":call});
    } else {
        waiter["secret_read"]["restart"] = restart.clone();
    }
}

pub(crate) fn provider_setup_has_ambiguous_end(
    state: &crate::State,
    c: &Context,
    provider: &Value,
) -> bool {
    if access::object(state, provider)
        .is_none_or(|object| object["state"] == "retired" || !is_own_runner_provider(object, c))
    {
        return false;
    }
    // Before runtime providers had a ledger, shutdown could report that a
    // process group was absent without observing when or why its run ended.
    state
        .invocations
        .iter()
        .rev()
        .filter(|setup| setup["object"] == *provider)
        .find(|setup| {
            setup.get("secret_provider_run").is_some()
                || setup["setup"]["parts"].as_array().is_some_and(|parts| {
                    parts
                        .iter()
                        .any(|part| part["name"] == "run" && part["outcome"] == "succeeded")
                })
        })
        .is_some_and(|setup| setup.get("secret_provider_run").is_none())
}

pub(crate) fn restart_if_needed(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    waiter: usize,
    call: Option<&Value>,
) -> Result<bool> {
    let v = t.state.invocations[waiter].clone();
    let provider = call.map_or(&v["target"], |intent| &intent["provider"]);
    if access::object(&t.state, provider).is_none_or(|object| object["state"] == "retired") {
        return Ok(false);
    }
    let Some(setup) = t
        .state
        .invocations
        .iter()
        .rev()
        .find(|p| p["object"] == *provider && p.get("secret_provider_run").is_some())
        .cloned()
    else {
        return Ok(false);
    };
    if setup["secret_provider_run"] != "restart-needed" {
        return Ok(false);
    }
    let active =
        t.state.invocations.iter().position(|p| {
            p["state"] == "in-progress" && p["provider_restart"]["setup"] == setup["id"]
        });
    if let Some(i) = active {
        // A provider whose own run depends on itself cannot be made available.
        if call.is_none() {
            let mut run = v["secret_read"]["run"].clone();
            loop {
                let parent = &t.state.invocations[find(t, &run)?];
                if parent["id"] == t.state.invocations[i]["id"] {
                    record_decision(t, a, c, waiter, Some(("unavailable", "PRV-026")))?;
                    end(t, a, c, waiter, "unavailable", Some("PRV-026"), None)?;
                    return Ok(true);
                }
                let Some(read) = parent["provider_restart"]["first_read"].as_str() else {
                    break;
                };
                run = t.state.invocations[find(t, &json!(read))?]["secret_read"]["run"].clone();
            }
        }
        let restart = t.state.invocations[i]["id"].clone();
        wait_for_restart(&mut t.state.invocations[waiter], &restart, call);
        return Ok(true);
    }
    let id = a.next(&mut t.state)?;
    wait_for_restart(&mut t.state.invocations[waiter], &json!(id), call);
    let mut restart = json!({"setup":setup["id"],"phase":"prepare"});
    if call.is_none() {
        restart["first_read"] = v["id"].clone();
    }
    t.state.invocations.push(json!({
        "id":id,"attempt":1,"state":"in-progress","object":provider,"caller":setup["caller"],
        "setup":{"item":setup["setup"]["item"]},
        "provider_restart":restart
    }));
    t.decisions.push(decision(
        if call.is_some() {
            "restart-provider-for-call"
        } else {
            "restart-secret-provider-for-read"
        },
        &["PRV-026", "OBS-004"],
        &[],
        json!({"provider":provider,"setup":setup["id"],"invocation":id}),
        vec![],
    ));
    prepare_run(t, a, c, t.state.invocations.len() - 1)?;
    Ok(true)
}

pub(crate) fn restart_ended(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    outcome: &str,
) -> Result<()> {
    let v = &mut t.state.invocations[i];
    v["state"] = json!(outcome);
    let id = v["id"].clone();
    let phase = v["provider_restart"]["phase"].clone();
    let setup = v["provider_restart"]["setup"].clone();
    let subject = v["object"].clone();
    let setup = find(t, &setup)?;
    if phase == "run" {
        t.state.invocations[setup]["secret_provider_run"] = json!(if outcome == "succeeded" {
            "asked"
        } else {
            "ended"
        });
        t.state.invocations[setup]["secret_provider_launch"] = id.clone();
        if outcome == "succeeded"
            && let Some(provider) = t.state.objects.iter_mut().find(|o| o["id"] == subject)
        {
            lifecycle::observe_run(provider, "running", &a.evidence.time)?;
        }
    }
    let waiting: Vec<_> = t
        .state
        .invocations
        .iter()
        .enumerate()
        .filter(|(_, waiter)| awaiting_restart(waiter, &id))
        .map(|(i, _)| i)
        .collect();
    for waiter in waiting {
        if !awaiting_restart(&t.state.invocations[waiter], &id) {
            continue;
        }
        if let Some(call) = t.state.invocations[waiter]
            .as_object_mut()
            .unwrap()
            .remove("provider_wait")
        {
            if outcome == "succeeded"
                && access::object(&t.state, &subject).is_some_and(|o| o["state"] != "retired")
            {
                t.external_intents.push(call["intent"].clone());
            } else {
                lifecycle::provider_restart_failed(t, a, c, waiter)?;
            }
            continue;
        }
        let parent = find(t, &t.state.invocations[waiter]["secret_read"]["run"])?;
        if t.state.invocations[parent]["state"] != "in-progress" || outcome != "succeeded" {
            record_decision(t, a, c, waiter, Some(("unavailable", "PRV-026")))?;
            end(t, a, c, waiter, "unavailable", Some("PRV-026"), None)?;
        } else {
            decide(t, a, c, waiter)?;
        }
    }
    Ok(())
}

pub(crate) fn lifecycle_report(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    report: &lifecycle::Report,
) -> Result<bool> {
    if t.state.invocations[i].get("provider_restart").is_some() {
        let lifecycle::Report::Run { outcome, .. } = report else {
            return Err("invalid secret provider restart report".into());
        };
        if !["succeeded", "failed", "unknown"].contains(&outcome.as_str()) {
            return Err("invalid secret provider restart outcome".into());
        }
        restart_ended(t, a, c, i, outcome)?;
        return Ok(true);
    }
    if let lifecycle::Report::Run { outcome, .. } = report
        && t.state.invocations[i].get("secret_provider_run").is_some()
        && outcome == "failed"
    {
        t.state.invocations[i]["secret_provider_run"] = json!("ended");
    }
    Ok(false)
}

pub(crate) fn provider_exited(
    t: &mut Transition,
    evidence: &crate::Evidence,
    subject: &str,
    launch: &str,
    cause: crate::ProviderEndCause,
) -> Result<()> {
    let Some(setup) = provider_launch(&mut t.state, subject, launch) else {
        return Ok(());
    };
    if setup["secret_provider_run"] == "ended" || cause == crate::ProviderEndCause::RunnerShutdown {
        return Ok(());
    }
    setup["secret_provider_run"] = json!("ended");
    setup["secret_provider_end"] = json!({"launch":launch,"cause":cause});
    if let Some(provider) = t.state.objects.iter_mut().find(|o| o["id"] == subject) {
        lifecycle::observe_run(provider, "stopped", &evidence.time)?;
    }
    t.decisions.push(decision(
        "observe-secret-provider-exit",
        &["PRV-026", "OBS-004"],
        &[],
        json!({"provider":subject,"launch":launch,"cause":cause}),
        vec![subject.into()],
    ));
    Ok(())
}

fn provider_launch<'a>(
    state: &'a mut crate::State,
    subject: &str,
    launch: &str,
) -> Option<&'a mut Value> {
    state
        .invocations
        .iter_mut()
        .rev()
        .find(|v| v["object"] == subject && v.get("secret_provider_run").is_some())
        .filter(|setup| setup.get("secret_provider_launch").unwrap_or(&setup["id"]) == launch)
}

pub(crate) fn provider_runner_stopped(
    state: &mut crate::State,
    subject: &str,
    end: &crate::ProviderRunEnd,
) -> bool {
    let Some(setup) = provider_launch(state, subject, &end.launch) else {
        return false;
    };
    if end.cause != crate::ProviderEndCause::RunnerShutdown {
        setup["secret_provider_run"] = json!("ended");
    } else if setup["secret_provider_run"] == "asked" {
        setup["secret_provider_run"] = json!("restart-needed");
    } else {
        return true;
    }
    setup["secret_provider_end"] = json!(end);
    true
}

pub(crate) fn recover(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<bool> {
    let v = &mut t.state.invocations[i];
    v.as_object_mut().unwrap().remove("secret_reads");
    v.as_object_mut().unwrap().remove("provider_wait");
    if v["secret_provider_run"] == "asked" {
        v["secret_provider_run"] = json!("restart-needed");
    }
    if v["state"] != "in-progress" {
        return Ok(false);
    }
    if v.get("provider_restart").is_some() {
        v["state"] = json!("unknown");
        return Ok(true);
    }
    if v.get("secret_read").is_some() {
        if matches!(
            v["secret_read"]["phase"].as_str(),
            Some("deciding" | "discover")
        ) {
            v["state"] = json!("unavailable");
            let data = json!({"invocation":v["id"],"reason":"unavailable","ruleId":"SEC-016"});
            record_decision(t, a, c, i, Some(("unavailable", "SEC-016")))?;
            requests::record(t, a, c, "request-ended", data)?;
            return Ok(true);
        }
        v["state"] = json!("unknown");
        let data = json!({"invocation":v["id"],"outcome":"unknown"});
        if v["secret_read"]["phase"] == "dispatched" {
            requests::record(t, a, c, "invocation-ended", data.clone())?;
        }
        requests::record(t, a, c, "request-ended", data)?;
        return Ok(true);
    }
    Ok(false)
}

pub(crate) fn cancel(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    let owner = &mut t.state.invocations[i];
    owner.as_object_mut().unwrap().remove("secret_reads");
    owner.as_object_mut().unwrap().remove("provider_wait");
    let run = owner["id"].clone();
    let mut cancelled = Vec::new();
    for v in &mut t.state.invocations {
        if v["state"] != "in-progress" || v["secret_read"]["run"] != run {
            continue;
        }
        v["state"] = json!("unknown");
        let dispatched = v["secret_read"]["phase"] == "dispatched";
        if dispatched || v["secret_read"]["phase"] == "discover" {
            t.external_intents
                .push(json!({"kind":"cancel","invocation":v["id"],"attempt":v["attempt"]}));
        }
        cancelled.push((v["id"].clone(), dispatched));
    }
    for (id, dispatched) in cancelled {
        let data = json!({"invocation":id,"outcome":"unknown"});
        if dispatched {
            requests::record(t, a, c, "invocation-ended", data.clone())?;
        }
        requests::record(t, a, c, "request-ended", data)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ambiguous_read_has_an_internal_refusal_diagnostic() {
        let (mut state, c) = crate::explain::tests::fixture();
        let operation = c.qualified("get-secret.v1");
        state
            .objects
            .push(json!({"kind":"Conduit","id":"second","name":"second",
            "consumer":"agent","target":"management","local_name":"second",
            "state":"ready","open":true,"operations":[operation]}));
        let read = json!({"id":"read", "caller":"root", "sender":"agent",
            "target":"management", "operation":operation});
        let result = refused_result(&state, &c, &read, "SEC-010", None).unwrap();
        assert_eq!(result["_meta"]["test/reason"], "refused");
        assert_eq!(result["isError"], true);
        let diagnostics = result["structuredContent"]["diagnostics"]
            .as_array()
            .unwrap();
        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        let refusal = &diagnostic["properties"]["refusal"];
        assert_eq!(diagnostic["ruleId"], "SEC-010");
        assert_eq!(refusal["tier"], 2);
        assert_eq!(refusal["operation"], operation);
        assert_eq!(refusal["exists"], true);
        assert_eq!(refusal["conduits"].as_array().unwrap().len(), 2);
        assert_eq!(
            refusal["cause"],
            "The engine reads a secret only through exactly one open Conduit of the instance's own that lists it; more than one is open (SEC-007)."
        );
        assert_eq!(result["content"][0]["text"], diagnostic["message"]["text"]);
    }
}
