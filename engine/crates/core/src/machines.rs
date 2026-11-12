//! Enrollment is engine state, never an object or an MCP surface (ENR-003/004).
use crate::{Allocation, Context, Result, State, Transition, access, decision, requests};
use chrono::{DateTime, FixedOffset};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub(crate) fn handles(op: &str, c: &Context) -> bool {
    ["enroll-machine.v1", "admit-machine.v1", "refuse-machine.v1"]
        .iter()
        .any(|n| op == c.qualified(n))
}
fn timestamp(s: &str) -> Result<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(s).map_err(|e| e.to_string())
}
fn live(e: &Value, time: &str) -> bool {
    e["state"] == "open"
        && timestamp(e["expires"].as_str().unwrap_or(""))
            .ok()
            .zip(timestamp(time).ok())
            .is_some_and(|(expiry, now)| expiry > now)
}
pub(crate) fn offers(o: &Value, c: &Context) -> bool {
    crate::registration::offers_machine_provider(o, &c.core_package)
}
fn system(s: &State, c: &Context) -> Value {
    s.objects
        .iter()
        .find(|o| o["from"] == c.qualified("system"))
        .map(|o| o["id"].clone())
        .unwrap_or(Value::Null)
}
pub(crate) fn provider<'a>(s: &'a State, c: &Context) -> Option<&'a Value> {
    s.objects.iter().find(|o| {
        o["kind"] == "Resource"
            && o["state"] != "retired"
            && access::id(&o["owner"]) == &system(s, c)
            && offers(o, c)
    })
}
fn report_machines(s: &State) -> &[Value] {
    s.machine_report
        .as_ref()
        .and_then(|r| r["outputs"]["machines"].as_array())
        .map(Vec::as_slice)
        .unwrap_or(&[])
}
// PLC-026 applies to comparisons, without rewriting provider-reported values.
fn canonical_hostname(value: &Value) -> Option<String> {
    value.as_str().map(|s| s.trim().to_lowercase())
}
fn same_hostname(left: &Value, right: &Value) -> bool {
    canonical_hostname(left)
        .zip(canonical_hostname(right))
        .is_some_and(|(left, right)| left == right)
}
fn node_for<'a>(s: &'a State, provider: &Value, id: &Value, c: &Context) -> Option<&'a Value> {
    s.objects.iter().find(|o| {
        o["type"] == c.qualified("node.v1")
            && o["state"] != "retired"
            && (o["status"]["machine"]["provider"] == *provider && o["status"]["machine"]["id"] == *id
                // PLC-026: local represents its hostname alone, without a provider id.
                || o["name"] == "local" && (
                    s.machine_report.as_ref().is_some_and(|r| r["provider"] == *provider) && report_machines(s).iter().any(|m|m["id"] == *id && same_hostname(&m["hostname"], &o["status"]["hostname"]))
                    || s.enrollments.iter().filter(|e| e["provider"] == *provider).flat_map(|e|e["machines"].as_array().into_iter().flatten()).any(|m|m["id"] == *id && same_hostname(&m["hostname"], &o["status"]["hostname"]))
                ))
    })
}
fn drop_action(e: &mut Value) {
    for key in ["action", "message", "carriesSecret"] {
        e.as_object_mut().unwrap().remove(key);
    }
}
fn change(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    index: usize,
    transition: &str,
) -> Result<()> {
    let e = &t.state.enrollments[index];
    let mut data = json!({"subject":e["provider"],"enrollment":e["id"],"transition":transition});
    let keys: &[&str] = match transition {
        "opened" => &[
            "expires",
            "require",
            "headless",
            "acceptWeakerCheck",
            "carriesSecret",
            "evidence",
        ],
        "admitted" => &[
            "machine",
            "acceptWeakerCheck",
            "fingerprintCompared",
            "weakerCheckAccepted",
        ],
        _ => &[],
    };
    for key in keys {
        data[*key] = e[*key].clone();
    }
    // This evidence is the declared kinds, never machine evidence (ENR-003/007).
    t.records
        .push(a.record(&mut t.state, c, "enrollment-change", data)?);
    t.decisions.push(decision(
        "enrollment-change",
        &[
            "ENR-001", "ENR-003", "ENR-007", "ENR-008", "ENR-009", "REC-001",
        ],
        &["DL8"],
        json!({"transition":transition}),
        vec![],
    ));
    Ok(())
}
pub(crate) fn expire(t: &mut Transition, a: &mut Allocation<'_>, c: &Context) -> Result<()> {
    for i in 0..t.state.enrollments.len() {
        let e = &t.state.enrollments[i];
        if e["state"] == "open" && !live(e, &a.evidence.time) {
            t.state.enrollments[i]["state"] = json!("expired");
            drop_action(&mut t.state.enrollments[i]);
            change(t, a, c, i, "expired")?;
        }
    }
    Ok(())
}
/// Number and cause for every failed ENR-006 check 1 through 7; no evidence values.
fn checks(
    s: &State,
    e: Option<&Value>,
    machine: Option<&Value>,
    p: Option<&Value>,
    time: &str,
) -> Vec<Value> {
    let empty = json!({});
    let e = e.unwrap_or(&empty);
    let m = machine.unwrap_or(&empty);
    let mut failures = vec![];
    let mut check = |number: u8, ok: bool, cause: &str| {
        if !ok {
            failures.push(json!({"check":number,"cause":cause}));
        }
    };
    check(
        1,
        p.is_some_and(|p| {
            s.machine_report
                .as_ref()
                .is_some_and(|r| r["provider"] == p["id"])
                && e["provider"] == p["id"]
                && m["state"] == "pending"
                && !e["id"].is_null()
                && m["enrollment"] == e["id"]
        }),
        "provider in force does not report this machine pending and attributed to this enrollment",
    );
    check(
        2,
        p.is_some_and(|p| e["provider"] == p["id"])
            && live(e, time)
            && e["machines"]
                .as_array()
                .is_some_and(|ms| ms.len() == 1 && ms[0]["id"] == m["id"]),
        &if e["id"].is_null() {
            "enrollment does not exist".into()
        } else if e["state"] != "open" {
            format!(
                "enrollment is {}",
                e["state"].as_str().unwrap_or("not open")
            )
        } else if !live(e, time) {
            "enrollment has expired".into()
        } else if p.is_none_or(|p| e["provider"] != p["id"]) {
            "enrollment belongs to another provider".into()
        } else {
            format!(
                "enrollment has {} joined machines, or the named machine did not join it",
                e["machines"].as_array().map_or(0, Vec::len)
            )
        },
    );
    let hostname = &m["hostname"];
    let node_type = s
        .objects
        .iter()
        .find(|n| n["kind"] == "Resource" && n["name"] == "local")
        .and_then(|n| n.get("type"));
    check(
        3,
        !hostname.is_null()
            && !s.objects.iter().any(|n| {
                node_type.is_some_and(|ty| n["type"] == *ty)
                    && n["state"] != "retired"
                    && same_hostname(&n["status"]["hostname"], hostname)
            })
            && !report_machines(s).iter().any(|n| {
                n["state"] == "pending"
                    && n["id"] != m["id"]
                    && same_hostname(&n["hostname"], hostname)
            }),
        "hostname is local's, a live node's, or another pending machine's",
    );
    let labels = m["labels"].as_object();
    let mut expected = e["labels"].as_object().cloned().unwrap_or_default();
    for key in [
        "kubernetes.io/hostname",
        "kubernetes.io/os",
        "kubernetes.io/arch",
    ] {
        if let Some(v) = m["labels"].get(key) {
            expected.insert(key.into(), v.clone());
        }
    }
    check(
        4,
        labels.is_some_and(|l| {
            *l == expected
                && l.get("kubernetes.io/hostname")
                    .is_some_and(|label| same_hostname(label, hostname))
                && ["kubernetes.io/os", "kubernetes.io/arch"].iter().all(|k| {
                    l.get(*k)
                        .and_then(Value::as_str)
                        .is_some_and(|v| !v.is_empty())
                })
        }),
        "labels are not exactly the descriptive labels and the enrollment's bound labels",
    );
    let items = m["evidence"].as_array().cloned().unwrap_or_default();
    let has = |kind| items.iter().any(|v| v["kind"] == kind);
    check(
        5,
        has("fingerprint") || has("attestation"),
        "evidence lacks fingerprint or attestation",
    );
    let weaker = e["headless"] == true && e["acceptWeakerCheck"] == true;
    let attestation = e["require"]
        .as_array()
        .is_some_and(|v| v.contains(&json!("attestation")))
        || e["headless"] == true && !weaker;
    check(
        6,
        !attestation || has("attestation"),
        "required attestation is absent",
    );
    check(
        7,
        weaker || items.iter().filter(|v| v["kind"] == "fingerprint").count() == 1,
        &format!(
            "provider reports {} fingerprint items; exactly one is required",
            items.iter().filter(|v| v["kind"] == "fingerprint").count()
        ),
    );
    failures
}
#[allow(clippy::too_many_arguments)]
fn rejection(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    route: &Value,
    reason: &str,
    rule: &str,
    cause: &str,
) -> Result<()> {
    // ENR-007: even rejections must scrub the typed input before record policy runs.
    let mut safe = route.clone();
    if route["operation"] == c.qualified("admit-machine.v1") {
        safe["arguments"]
            .as_object_mut()
            .unwrap()
            .remove("fingerprint");
    }
    requests::reject(t, a, c, &safe, reason, rule, None, None)?;
    t.reply["structuredContent"] = json!({"diagnostics":[{"ruleId":rule,"level":"error","message":{"text":format!("{rule}: {cause}")}}]});
    Ok(())
}
pub(crate) fn preflight(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    route: &Value,
) -> Result<bool> {
    let op = route["operation"].as_str().unwrap_or("");
    if !handles(op, c) {
        return Ok(false);
    }
    let args = &route["arguments"];
    let p = provider(&t.state, c);
    let mut reason = None;
    if op == c.qualified("enroll-machine.v1") {
        let now = timestamp(&a.evidence.time)?;
        let expiry = timestamp(args["expires"].as_str().ok_or("missing expiry")?)?;
        if args["labels"].as_object().is_some_and(|l| {
            l.keys().any(|k| {
                k == "os"
                    || k == "arch"
                    || k.starts_with("kubernetes.io/")
                    || k.starts_with("k8s.io/")
            })
        }) {
            reason = Some((
                "invalid",
                "ENR-001",
                "labels contains a reserved key".into(),
            ));
        } else if expiry <= now || expiry - now > chrono::Duration::hours(24) {
            reason = Some((
                "invalid",
                "ENR-001",
                "expires must be later than the request and at most 24 hours later (DL5)".into(),
            ));
        } else if args["acceptWeakerCheck"] == true && args["headless"] != true {
            reason = Some((
                "invalid",
                "ENR-001",
                "acceptWeakerCheck requires headless".into(),
            ));
        }
    } else if op == c.qualified("admit-machine.v1") {
        let e = t
            .state
            .enrollments
            .iter()
            .find(|e| e["id"] == args["enrollment"]);
        let m = report_machines(&t.state)
            .iter()
            .find(|m| m["id"] == args["machine"]);
        let mut failures = checks(&t.state, e, m, p, &a.evidence.time);
        let weaker = e.is_some_and(|e| e["headless"] == true && e["acceptWeakerCheck"] == true);
        let seven = !failures.iter().any(|f| f["check"] == 7);
        let mismatch = !weaker
            && (args.get("fingerprint").is_none()
                || seven
                    && m.is_some_and(|m| {
                        m["evidence"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|v| v["kind"] == "fingerprint")
                            .is_some_and(|v| v["value"] != args["fingerprint"])
                    }));
        if mismatch {
            failures.insert(0,json!({"check":8,"cause":if args.get("fingerprint").is_none() {"typed fingerprint is missing"} else {"typed fingerprint does not equal the reported value"}}));
        }
        if !failures.is_empty() {
            let reason = if mismatch { "invalid" } else { "conflict" };
            let diagnostics=failures.iter().map(|f|json!({"ruleId":"ENR-006","level":"error","message":{"text":format!("ENR-006 check {}: {}",f["check"],f["cause"].as_str().unwrap())}})).collect::<Vec<_>>();
            rejection(
                t,
                a,
                c,
                route,
                reason,
                "ENR-006",
                diagnostics[0]["message"]["text"].as_str().unwrap(),
            )?;
            t.reply["structuredContent"] = json!({"diagnostics":diagnostics});
            return Ok(true);
        }
    } else {
        let e = t
            .state
            .enrollments
            .iter()
            .find(|e| e["id"] == args["enrollment"]);
        let exists = args.get("machine").is_none()
            || report_machines(&t.state)
                .iter()
                .any(|m| m["id"] == args["machine"])
            || e.is_some_and(|e| {
                e["machines"]
                    .as_array()
                    .is_some_and(|ms| ms.iter().any(|m| m["id"] == args["machine"]))
            });
        if args.get("enrollment").is_some() && e.is_none() || !exists {
            reason = Some((
                "invalid",
                "ENR-008",
                "named enrollment or machine does not exist".into(),
            ));
        } else {
            let pid = e
                .map(|e| e["provider"].clone())
                .or_else(|| p.map(|p| p["id"].clone()))
                .unwrap_or(Value::Null);
            let ids = removal_ids(args, e);
            if let Some(n) = ids
                .iter()
                .find_map(|id| node_for(&t.state, &pid, &json!(id), c))
            {
                reason = Some((
                    "conflict",
                    "ENR-008",
                    format!("machine is represented by node {} ({})", n["name"], n["id"]),
                ));
            } else if e.is_some_and(|e| p.is_none_or(|p| p["id"] != e["provider"])) {
                reason = Some((
                    "unavailable",
                    "ENR-008",
                    "enrollment's provider is retired or not in force".into(),
                ));
            }
        }
    }
    if reason.is_none() && p.is_none_or(|p| p["state"] != "ready") {
        let rule = if op == c.qualified("enroll-machine.v1") {
            "ENR-001"
        } else if op == c.qualified("admit-machine.v1") {
            "ENR-006"
        } else {
            "ENR-008"
        };
        reason = Some((
            "unavailable",
            rule,
            p.map(|p| format!("PRV-003: machine provider {} is not ready", p["id"]))
                .unwrap_or("PRV-003: no machine provider is in force".into()),
        ));
    }
    if let Some((reason, rule, cause)) = reason {
        rejection(t, a, c, route, reason, rule, &cause)?;
        return Ok(true);
    }
    Ok(false)
}
fn removal_ids(args: &Value, e: Option<&Value>) -> BTreeSet<String> {
    args.get("machine")
        .and_then(Value::as_str)
        .into_iter()
        .chain(
            e.into_iter()
                .flat_map(|e| e["machines"].as_array().into_iter().flatten())
                .filter_map(|m| m["id"].as_str()),
        )
        .map(str::to_owned)
        .collect()
}
#[allow(clippy::too_many_arguments)]
fn emit(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    p: &Value,
    op: &str,
    inputs: Value,
) -> Result<()> {
    let v = &t.state.invocations[i];
    let intent = json!({"kind":"provider-call","provider":p["id"],"config":p["config"],"revision":p["revision"],"invocation":v["id"],"caller":v["caller"],"subject":p["id"],"operation":c.qualified(op),"inputs":inputs});
    requests::record(
        t,
        a,
        c,
        "provider-call",
        json!({"phase":"made","target":p["id"],"subject":p["id"],"operation":intent["operation"],"invocation":intent["invocation"],"caller":intent["caller"],"arguments":intent["inputs"]}),
    )?;
    t.external_intents.push(intent);
    t.reply = json!({"pending":true,"invocation":t.state.invocations[i]["id"]});
    t.decisions.push(decision(
        "machine-provider-call",
        &["PRV-005", "REC-001", "ENR-001", "ENR-007", "ENR-008"],
        &[],
        json!({"operation":c.qualified(op)}),
        vec![],
    ));
    Ok(())
}
pub(crate) fn execute(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    let v = t.state.invocations[i].clone();
    let args = &v["arguments"];
    let p = provider(&t.state, c)
        .ok_or("provider missing after preflight")?
        .clone();
    t.state.invocations[i]["machine_provider"] = p["id"].clone();
    if v["operation"] == c.qualified("enroll-machine.v1") {
        let id = a.next(&mut t.state)?;
        let pending = a.next(&mut t.state)?;
        let e = json!({"id":id,"provider":p["id"],"pendingAction":pending,"labels":args.get("labels").cloned().unwrap_or(json!({})),"expires":args["expires"],"require":args.get("require").cloned().unwrap_or(json!([])),"headless":args["headless"] == true,"acceptWeakerCheck":args["acceptWeakerCheck"] == true,"state":"opening","machines":[]});
        t.state.invocations[i]["enrollment"] = json!(id);
        t.state.invocations[i]["machine_wait"] = json!("enroll");
        let inputs = json!({"enrollment":id,"labels":e["labels"],"expires":e["expires"],"require":e["require"],"headless":e["headless"]});
        t.state.enrollments.push(e);
        emit(t, a, c, i, &p, "enroll.v1", inputs)
    } else if v["operation"] == c.qualified("admit-machine.v1") {
        t.state.invocations[i]["machine_wait"] = json!("accept-machine");
        let m = report_machines(&t.state)
            .iter()
            .find(|m| m["id"] == args["machine"])
            .ok_or("admission machine missing")?
            .clone();
        let e = t
            .state
            .enrollments
            .iter_mut()
            .find(|e| e["id"] == args["enrollment"])
            .unwrap();
        if !e["accepting"].is_object() {
            e["accepting"] = json!({});
        }
        // The checked identity/evidence stays fixed; descriptive labels may be
        // corrected by a later report before admission completes (ENR-010).
        e["accepting"][v["id"].as_str().unwrap()] = json!({"machine":m,"labels":m["labels"]});
        emit(
            t,
            a,
            c,
            i,
            &p,
            "accept-machine.v1",
            json!({"id":args["machine"],"enrollment":args["enrollment"]}),
        )
    } else {
        let ei = t
            .state
            .enrollments
            .iter()
            .position(|e| e["id"] == args["enrollment"]);
        let ids = removal_ids(args, ei.map(|i| &t.state.enrollments[i]));
        if let Some(ei) = ei
            && t.state.enrollments[ei]["state"] == "open"
        {
            t.state.enrollments[ei]["state"] = json!("refused");
            drop_action(&mut t.state.enrollments[ei]);
            change(t, a, c, ei, "refused")?;
        }
        t.state.invocations[i]["removals"] = json!(
            ids.iter()
                .map(|id| (id.clone(), json!("pending")))
                .collect::<serde_json::Map<_, _>>()
        );
        for id in ids {
            emit(t, a, c, i, &p, "remove-machine.v1", json!({"id":id}))?;
        }
        settle_removals(t, a, c, i)
    }
}
fn output_valid(c: &Context, op: &str, outputs: &Value) -> Result<bool> {
    let (scope, space) = c
        .core_environment
        .scopes
        .iter()
        .find(|(_, s)| s.address == c.core_package)
        .ok_or("missing core scope")?;
    let contract = space.contracts.get(op).ok_or("missing core operation")?;
    c.core_environment.valid(
        scope,
        &contract["spec"]["outputSchema"],
        outputs,
        &c.core_package,
    )
}
#[allow(clippy::too_many_arguments)]
fn finish(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    outcome: &str,
    outputs: Value,
    rule: &str,
    cause: &str,
) -> Result<()> {
    requests::finish(t, a, c, i, outcome, outputs, json!([]))?;
    if outcome != "succeeded" {
        let diagnostics =
            json!([{"ruleId":rule,"level":"error","message":{"text":format!("{rule}: {cause}")}}]);
        t.reply["structuredContent"] = json!({"diagnostics":diagnostics});
        if let Some(reply) = t.request_replies.last_mut() {
            reply.reply = t.reply.clone();
        }
    }
    Ok(())
}
pub(crate) fn report(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    report: crate::lifecycle::Report,
) -> Result<()> {
    use crate::lifecycle::Report;
    let v = t.state.invocations[i].clone();
    let (kind, outcome, outputs, id) = match report {
        Report::Enroll { outcome, outputs } => (
            "enroll",
            outcome,
            outputs.unwrap_or(Value::Null),
            Value::Null,
        ),
        Report::AcceptMachine { outcome } => ("accept-machine", outcome, Value::Null, Value::Null),
        Report::RemoveMachine { id, outcome } => {
            ("remove-machine", outcome, Value::Null, json!(id))
        }
        _ => {
            t.reply = json!({"ignored":true});
            return Ok(());
        }
    };
    if !["succeeded", "failed", "unknown"].contains(&outcome.as_str()) {
        return Err("invalid machine provider outcome".into());
    }
    if kind == "remove-machine" {
        if v["removals"][id.as_str().unwrap()] != "pending" {
            t.reply = json!({"ignored":true});
            return Ok(());
        }
    } else if v["machine_wait"] != kind {
        t.reply = json!({"ignored":true});
        return Ok(());
    }
    let invalid_output =
        kind == "enroll" && outcome == "succeeded" && !output_valid(c, "enroll.v1", &outputs)?;
    let outcome = if invalid_output {
        "unknown"
    } else {
        outcome.as_str()
    };
    let op = format!("{kind}.v1");
    let mut safe = outputs.clone();
    if let Some(o) = safe.as_object_mut() {
        o.remove("action");
        o.remove("message");
    }
    requests::record(
        t,
        a,
        c,
        "provider-call",
        json!({"phase":"ended","target":v["machine_provider"],"subject":v["machine_provider"],"operation":c.qualified(&op),"invocation":v["id"],"caller":v["caller"],"outcome":outcome,"outputs":if outcome == "succeeded" {safe} else {Value::Null}}),
    )?;
    if kind == "remove-machine" {
        t.state.invocations[i]["removals"][id.as_str().unwrap()] = json!(outcome);
        return settle_removals(t, a, c, i);
    }
    t.state.invocations[i]
        .as_object_mut()
        .unwrap()
        .remove("machine_wait");
    let eid = if kind == "enroll" {
        &v["enrollment"]
    } else {
        &v["arguments"]["enrollment"]
    };
    let ei = t
        .state
        .enrollments
        .iter()
        .position(|e| &e["id"] == eid)
        .ok_or("report enrollment missing")?;
    if outcome != "succeeded" {
        if kind == "enroll" {
            t.state.enrollments.remove(ei);
        } else {
            release_acceptance(&mut t.state.enrollments[ei], &v["id"]);
        }
        return finish(
            t,
            a,
            c,
            i,
            outcome,
            json!({}),
            if invalid_output {
                "PRV-005"
            } else if kind == "enroll" {
                "ENR-001"
            } else {
                "ENR-007"
            },
            if invalid_output {
                "OPS-007: enroll.v1 output failed validation"
            } else {
                "machine provider call did not succeed; nothing retried"
            },
        );
    }
    if kind == "enroll" {
        let e = &mut t.state.enrollments[ei];
        let kinds = outputs["evidence"].as_array().unwrap();
        let has = |k| kinds.contains(&json!(k));
        let weaker = e["headless"] == true && e["acceptWeakerCheck"] == true;
        let mut missing = vec![];
        if !weaker && !has("fingerprint") {
            missing.push("fingerprint");
        }
        if (e["headless"] == true && !weaker
            || e["require"]
                .as_array()
                .unwrap()
                .contains(&json!("attestation")))
            && !has("attestation")
        {
            missing.push("attestation");
        }
        if weaker && !has("fingerprint") && !has("attestation") {
            missing.extend(["fingerprint", "attestation"]);
        }
        if !missing.is_empty() {
            t.state.enrollments.remove(ei);
            return finish(
                t,
                a,
                c,
                i,
                "failed",
                json!({}),
                "ENR-001",
                &format!("missing evidence kinds: {}", missing.join(" or ")),
            );
        }
        if timestamp(outputs["expires"].as_str().unwrap())?
            < timestamp(e["expires"].as_str().unwrap())?
        {
            e["expires"] = outputs["expires"].clone();
        }
        for key in ["action", "message", "carriesSecret", "evidence"] {
            e[key] = outputs[key].clone();
        }
        e["state"] = json!("open");
        let result = json!({"enrollment":e["id"],"pendingAction":e["pendingAction"]});
        change(t, a, c, ei, "opened")?;
        expire(t, a, c)?;
        finish(t, a, c, i, "succeeded", result, "ENR-001", "")
    } else {
        admit_node(t, a, c, i, ei)
    }
}
fn settle_removals(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
) -> Result<()> {
    let removals = t.state.invocations[i]["removals"].as_object().unwrap();
    if removals.values().any(|v| v == "pending") {
        return Ok(());
    }
    let outcome = if removals.values().any(|v| v == "failed") {
        "failed"
    } else if removals.values().any(|v| v == "unknown") {
        "unknown"
    } else {
        "succeeded"
    };
    let diagnostics = removals.iter().filter(|(_,v)| *v != "succeeded").map(|(id,v)|json!({"ruleId":"ENR-008","level":"error","message":{"text":format!("ENR-008: removal of machine {id} ended {v}; no removal claimed or retried")}})).collect::<Vec<_>>();
    requests::finish(t, a, c, i, outcome, json!({}), json!([]))?;
    if !diagnostics.is_empty() {
        t.reply["structuredContent"] = json!({"diagnostics":diagnostics});
        if let Some(r) = t.request_replies.last_mut() {
            r.reply = t.reply.clone();
        }
    }
    Ok(())
}
fn release_acceptance(e: &mut Value, invocation: &Value) {
    if let Some(pending) = e.get_mut("accepting").and_then(Value::as_object_mut) {
        pending.remove(invocation.as_str().unwrap());
        if pending.is_empty() {
            e.as_object_mut().unwrap().remove("accepting");
        }
    }
}
fn admit_node(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    ei: usize,
) -> Result<()> {
    let e = t.state.enrollments[ei].clone();
    let invocation = t.state.invocations[i]["id"].clone();
    let acceptance = e["accepting"]
        .get(invocation.as_str().unwrap())
        .ok_or("ENR-007: admission has no checked machine snapshot")?;
    let m = &acceptance["machine"];
    let mut labels = e["labels"].as_object().cloned().unwrap_or_default();
    for key in [
        "kubernetes.io/hostname",
        "kubernetes.io/os",
        "kubernetes.io/arch",
    ] {
        if let Some(value) = acceptance["labels"].get(key) {
            labels.insert(key.into(), value.clone());
        }
    }
    let id = a.next(&mut t.state)?;
    let hostname = m["hostname"].as_str().unwrap();
    let short = hostname.split('.').next().unwrap();
    let valid = !short.is_empty()
        && short.len() <= 63
        && short.bytes().next().is_some_and(|b| b.is_ascii_lowercase())
        && short
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
    let free = |name: &str| {
        !t.state
            .objects
            .iter()
            .any(|o| o["kind"] == "Resource" && o["name"] == name)
    };
    let name = if valid && free(short) {
        short.to_owned()
    } else {
        a.evidence
            .suffixes
            .iter()
            .filter(|s| {
                s.len() == 5
                    && s.bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
            })
            .map(|s| format!("node-{s}"))
            .find(|n| free(n))
            .ok_or("node needs an unused OBJ-003 suffix")?
    };
    let revision = a
        .evidence
        .revisions
        .first()
        .ok_or("admission needs a revision")?;
    let mut node = json!({"id":id,"revision":revision,"kind":"Resource","name":name,"from":c.qualified("node"),"type":c.template("node")?["spec"]["type"],"state":"ready","config":{},"owner":system(&t.state,c),"placement":id,"status":{"hostname":hostname,"machine":{"provider":e["provider"],"id":m["id"],"hostname":hostname},"enrollment":e["id"],"labels":labels}});
    condition(
        &mut node,
        "Ready",
        "Unknown",
        "NoReportSinceAdmission",
        "PLC-023: no report has given this machine a state since admission",
        &a.evidence.time,
        false,
    );
    t.state.objects.push(node);
    for (action, previous, state) in [
        ("registered", Value::Null, "registered"),
        ("lifecycle-changed", json!("registered"), "ready"),
    ] {
        requests::record(
            t,
            a,
            c,
            "object-change",
            json!({"target":id,"action":action,"previous":previous,"state":state,"cause":"admission","invocation":t.state.invocations[i]["id"]}),
        )?;
    }
    let en = &mut t.state.enrollments[ei];
    en["state"] = json!("admitted");
    en["machine"] = m["id"].clone();
    let weaker = e["headless"] == true && e["acceptWeakerCheck"] == true;
    en["fingerprintCompared"] = json!(!weaker);
    en["weakerCheckAccepted"] = json!(weaker);
    drop_action(en);
    release_acceptance(en, &invocation);
    change(t, a, c, ei, "admitted")?;
    t.decisions.push(decision(
        "admit-node",
        &[
            "ENR-006", "ENR-007", "ENR-010", "ENR-011", "PLC-008", "PLC-023", "PLC-026",
        ],
        &["DL5"],
        json!({"node":id}),
        vec![id.clone()],
    ));
    finish(
        t,
        a,
        c,
        i,
        "succeeded",
        json!({"node":{"kind":"Resource","name":name,"id":id}}),
        "ENR-007",
        "",
    )
}
fn condition(
    n: &mut Value,
    kind: &str,
    status: &str,
    reason: &str,
    message: &str,
    time: &str,
    heartbeat: bool,
) {
    if !n["status"]["conditions"].is_array() {
        n["status"]["conditions"] = json!([]);
    }
    let cs = n["status"]["conditions"].as_array_mut().unwrap();
    let at = cs.iter().position(|v| v["type"] == kind);
    let old = at.map(|at| cs[at].clone()).unwrap_or(Value::Null);
    let mut value = json!({"type":kind,"status":status,"reason":reason,"message":message,"source":"urn:conduit:machine-provider","lastTransitionTime":if old["status"] == status {old["lastTransitionTime"].clone()} else {json!(time)}});
    if heartbeat {
        value["lastHeartbeatTime"] = json!(time);
    } else if let Some(time) = old.get("lastHeartbeatTime") {
        value["lastHeartbeatTime"] = time.clone();
    }
    if let Some(at) = at {
        cs[at] = value;
    } else {
        cs.push(value);
    }
}
pub(crate) fn omit_nodes(s: &mut State, time: &str, reason: &str, cause: &str, c: &Context) {
    for n in &mut s.objects {
        if n["type"] == c.qualified("node.v1") && n["name"] != "local" {
            condition(n, "Ready", "Unknown", reason, cause, time, false);
        }
    }
    unobserve_instances(s, time, c);
}
// PLC-023 / OBS-003: losing the node makes every observation on its
// instances unknown, without changing lifecycle or last successful times.
fn unobserve_instances(s: &mut State, time: &str, c: &Context) {
    let unavailable = s
        .objects
        .iter()
        .filter(|n| {
            n["type"] == c.qualified("node.v1")
                && n["name"] != "local"
                && !n["status"]["conditions"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .any(|v| v["type"] == "Ready" && v["status"] == "True")
        })
        .map(|n| {
            (
                n["id"].clone(),
                n["status"]["conditions"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .find(|v| v["type"] == "Ready")
                    .map(|v| v["message"].as_str().unwrap_or("").to_owned())
                    .unwrap_or_default(),
            )
        })
        .collect::<Vec<_>>();
    for object in &mut s.objects {
        if object["type"] == c.qualified("node.v1") {
            continue;
        }
        let Some((node, cause)) = unavailable
            .iter()
            .find(|(id, _)| access::id(&object["placement"]) == id)
        else {
            continue;
        };
        if !object["status"].is_object() {
            object["status"] = json!({});
        }
        if !object["status"]["conditions"].is_array() {
            object["status"]["conditions"] = json!([]);
        }
        if object["status"]["conditions"]
            .as_array()
            .unwrap()
            .is_empty()
        {
            condition(
                object,
                "Running",
                "Unknown",
                "NodeUnavailable",
                &format!("PLC-023 OBS-003: node {node} is unavailable; {cause}"),
                time,
                false,
            );
        }
        let mut past = vec![];
        for observation in object["status"]["conditions"].as_array_mut().unwrap() {
            if observation["status"] != "Unknown" {
                past.push(json!({"condition":observation,"endedAt":time}));
                observation["lastTransitionTime"] = json!(time);
            }
            observation["status"] = json!("Unknown");
            observation["reason"] = json!("NodeUnavailable");
            observation["message"] = json!(format!(
                "PLC-023 OBS-003: node {node} is unavailable; {cause}"
            ));
        }
        if !past.is_empty() {
            if !object["past_conditions"].is_array() {
                object["past_conditions"] = json!([]);
            }
            object["past_conditions"]
                .as_array_mut()
                .unwrap()
                .extend(past);
        }
    }
}
pub(crate) fn retired(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    id: &Value,
) -> Result<()> {
    for ei in 0..t.state.enrollments.len() {
        if t.state.enrollments[ei]["provider"] == *id && t.state.enrollments[ei]["state"] == "open"
        {
            t.state.enrollments[ei]["state"] = json!("refused");
            drop_action(&mut t.state.enrollments[ei]);
            change(t, a, c, ei, "refused")?;
        }
    }
    if t.state
        .machine_report
        .as_ref()
        .is_some_and(|r| r["provider"] == *id)
    {
        t.state.machine_report = None;
    }
    omit_nodes(
        &mut t.state,
        &a.evidence.time,
        "ProviderRetired",
        "PLC-023 PRV-013: machine provider retired",
        c,
    );
    Ok(())
}
pub(crate) fn machine_report(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    pid: Value,
    outcome: &str,
    outputs: Option<Value>,
) -> Result<()> {
    if !["succeeded", "failed", "unknown"].contains(&outcome) {
        return Err("invalid machine report outcome".into());
    }
    let outputs = outputs.unwrap_or(Value::Null);
    let valid = outcome != "succeeded" || output_valid(c, "list-machines.v1", &outputs)?;
    let outcome = if valid { outcome } else { "unknown" };
    for phase in ["made", "ended"] {
        let mut data = json!({"target":pid,"subject":pid,"operation":c.qualified("list-machines.v1"),"phase":phase});
        if phase == "ended" {
            data["outcome"] = json!(outcome);
        }
        // PND-008/DL8: autonomous list records never carry outputs or evidence.
        t.records
            .push(a.record(&mut t.state, c, "provider-call", data)?);
    }
    let ms = outputs["machines"].as_array().cloned().unwrap_or_default();
    let unique = |key: &str| {
        ms.iter()
            .map(|m| {
                if key == "hostname" {
                    canonical_hostname(&m[key]).unwrap_or_default()
                } else {
                    m[key].as_str().unwrap_or("").to_owned()
                }
            })
            .collect::<BTreeSet<_>>()
            .len()
            == ms.len()
    };
    let use_report = outcome == "succeeded"
        && provider(&t.state, c).is_some_and(|p| p["id"] == pid)
        && unique("id")
        && unique("hostname");
    t.decisions.push(decision(
        "machine-report",
        &[
            "ENR-004", "ENR-010", "PLC-023", "PLC-026", "OBS-003", "PRV-005", "PRV-014", "OPS-007",
            "REC-001",
        ],
        &["DL5", "DL8"],
        json!({"used":use_report,"outcome":outcome}),
        vec![],
    ));
    if !use_report {
        t.reply = json!({"used":false});
        return Ok(());
    }
    // Preserve the last labels reported during acceptance even if a later
    // report omits the machine. Identity and checked evidence never change.
    for e in &mut t.state.enrollments {
        if e["provider"] == pid
            && let Some(accepting) = e["accepting"].as_object_mut()
        {
            for acceptance in accepting.values_mut() {
                if let Some(m) = ms.iter().find(|m| m["id"] == acceptance["machine"]["id"]) {
                    acceptance["labels"] = m["labels"].clone();
                }
            }
        }
    }
    for m in &ms {
        if m["state"] == "pending"
            && let Some(ei) = t.state.enrollments.iter().position(|e| {
                e["id"] == m["enrollment"] && e["provider"] == pid && live(e, &a.evidence.time)
            })
        {
            // Remember the actual latest join separately from the report's
            // optional attribution. Earlier memberships remain history.
            for e in &mut t.state.enrollments {
                if e["provider"] == pid {
                    for joined in e["machines"].as_array_mut().unwrap() {
                        if joined["id"] == m["id"] {
                            joined["currentJoin"] = json!(false);
                        }
                    }
                }
            }
            let e = &mut t.state.enrollments[ei];
            let mut machine = m.clone();
            machine["currentJoin"] = json!(true);
            let joined = e["machines"].as_array_mut().unwrap();
            if let Some(old) = joined.iter_mut().find(|v| v["id"] == m["id"]) {
                *old = machine;
            } else {
                joined.push(machine);
            }
            drop_action(e);
        }
    }
    for n in &mut t.state.objects {
        if n["type"] != c.qualified("node.v1") || n["name"] == "local" {
            continue;
        }
        let triple = &n["status"]["machine"];
        let matching = ms.iter().find(|m| {
            triple["provider"] == pid
                && triple["id"] == m["id"]
                && same_hostname(&triple["hostname"], &m["hostname"])
        });
        match matching {
            Some(m) if m["state"] != "pending" => {
                let reachable = m["state"] == "reachable";
                let reason = if reachable {
                    "MachineReachable"
                } else {
                    "MachineUnreachable"
                };
                condition(
                    n,
                    "Ready",
                    if reachable { "True" } else { "False" },
                    reason,
                    &format!("PLC-023: reported machine state {}", m["state"]),
                    &a.evidence.time,
                    true,
                );
            }
            _ => {
                let cause = if matching.is_some() {
                    "PLC-023: represented machine reported pending"
                } else {
                    "PLC-023 PLC-026: report omitted the provider/id/hostname triple (absent or mismatched id or hostname)"
                };
                condition(
                    n,
                    "Ready",
                    "Unknown",
                    if matching.is_some() {
                        "MachineReportedPending"
                    } else {
                        "MachineOmitted"
                    },
                    cause,
                    &a.evidence.time,
                    false,
                );
            }
        }
        if let Some(m) = matching {
            let fixed = n["status"]["labels"]
                .as_object()
                .cloned()
                .unwrap_or_default();
            let seen = m["labels"].as_object().unwrap();
            let keys = fixed
                .keys()
                .chain(seen.keys())
                .filter(|k| fixed.get(*k) != seen.get(*k))
                .cloned()
                .collect::<BTreeSet<_>>();
            if !keys.is_empty() {
                condition(
                    n,
                    "LabelsDrift",
                    "True",
                    "ReportedLabelsDiffer",
                    &format!(
                        "ENR-010: differing label keys: {}",
                        keys.into_iter().collect::<Vec<_>>().join(", ")
                    ),
                    &a.evidence.time,
                    true,
                );
            } else if let Some(cs) = n["status"]["conditions"].as_array_mut() {
                cs.retain(|v| v["type"] != "LabelsDrift");
            }
        }
    }
    unobserve_instances(&mut t.state, &a.evidence.time, c);
    t.state.machine_report = Some(json!({"provider":pid,"outputs":outputs}));
    t.reply = json!({"used":true});
    Ok(())
}
/// Root's panel alone consumes this pure view. No tool exposes it (PND-008).
pub fn panel(s: &State, time: &str) -> Value {
    let enrollments = s
        .enrollments
        .iter()
        .filter(|e| live(e, time))
        .map(|e| {
            let mut v = json!({});
            for k in [
                "id",
                "provider",
                "labels",
                "expires",
                "require",
                "headless",
                "acceptWeakerCheck",
            ] {
                v[k] = e[k].clone();
            }
            if e["machines"].as_array().is_some_and(Vec::is_empty) {
                for k in ["action", "message"] {
                    if let Some(value) = e.get(k) {
                        v[k] = value.clone();
                    }
                }
            }
            v
        })
        .collect::<Vec<_>>();
    let machines = report_machines(s)
        .iter()
        .filter(|m| m["state"] == "pending")
        .map(|m| {
            let provider = &s.machine_report.as_ref().unwrap()["provider"];
            let joins = || {
                s.enrollments.iter().rev().filter(|e| {
                    e["provider"] == *provider
                        && e["machines"]
                            .as_array()
                            .is_some_and(|ms| ms.iter().any(|v| v["id"] == m["id"]))
                })
            };
            let e = joins()
                .find(|e| {
                    e["machines"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|v| v["id"] == m["id"] && v["currentJoin"] == true)
                })
                // Older snapshots have no marker; use only recorded memberships.
                .or_else(|| joins().find(|e| e["id"] == m["enrollment"]))
                .or_else(|| joins().next());
            let p = s
                .objects
                .iter()
                .find(|p| p["id"] == *provider && p["state"] != "retired");
            let mut view = json!({
                "hostname":m["hostname"], "id":m["id"], "labels":m["labels"],
                "evidence":m.get("evidence").cloned().unwrap_or(json!([])),
                "checks":checks(s, e, Some(m), p, time)
            });
            if let Some(e) = e {
                view["enrollment"] = e["id"].clone();
            }
            view
        })
        .collect::<Vec<_>>();
    json!({"enrollments":enrollments,"machines":machines})
}

/// PLC-023 gates dispatch, never registration or local's historic status.
pub(crate) fn unavailable_node<'a>(s: &'a State, o: &Value) -> Option<&'a Value> {
    let n = access::object(s, &o["placement"])?;
    (n["name"].as_str().is_some_and(|name| name != "local")
        && !n["status"]["conditions"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|v| v["type"] == "Ready" && v["status"] == "True"))
    .then_some(n)
}

pub(crate) fn unavailable_diagnostic(t: &mut Transition, cause: &str) {
    t.reply["structuredContent"] =
        json!({"diagnostics":[{"ruleId":"PLC-023","level":"error","message":{"text":cause}}]});
    if let Some(reply) = t.request_replies.last_mut() {
        reply.reply = t.reply.clone();
    }
    t.decisions.push(decision(
        "hold-dispatch-for-node",
        &["PLC-023"],
        &[],
        json!({"cause":cause}),
        vec![],
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Evidence, Input, schemas};
    use std::collections::BTreeMap;
    struct Harness {
        c: Context,
        s: State,
        sequence: usize,
    }
    fn read(path: &std::path::Path) -> Vec<Value> {
        conduit_loader::read_file(path)
            .unwrap()
            .into_iter()
            .map(|d| d.root.to_json())
            .collect()
    }
    fn context() -> Context {
        static C: std::sync::OnceLock<Context> = std::sync::OnceLock::new();
        C.get_or_init(|| {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
            let spec = conduit_loader::Specification::read(&root.join("spec")).unwrap();
            let core = conduit_loader::read_project(&root.join("spec/core"), &spec).unwrap();
            let mut documents = core
                .documents
                .into_iter()
                .map(|d| d.root.to_json())
                .collect::<Vec<_>>();
            for file in std::fs::read_dir(root.join("spec/schemas")).unwrap() {
                let path = file.unwrap().path();
                if path.extension().is_some_and(|e| e == "yaml") {
                    documents.extend(read(&path));
                }
            }
            let contracts = documents
                .iter()
                .filter(|v| {
                    ["Operation", "Type", "Schema", "Capability"]
                        .iter()
                        .any(|k| v["kind"] == *k)
                })
                .map(|v| (v["metadata"]["name"].as_str().unwrap().into(), v.clone()))
                .collect();
            let environment = schemas::Environment {
                scopes: BTreeMap::from([(
                    "core".into(),
                    schemas::Scope {
                        address: spec.names().core_package.clone(),
                        version: None,
                        manifest: BTreeMap::new(),
                        contracts,
                        exports: None,
                    },
                )]),
            };
            let templates = documents
                .iter()
                .filter(|v| {
                    v.get("spec").is_some()
                        && !["Operation", "Type", "Schema", "Capability"]
                            .iter()
                            .any(|k| v["kind"] == *k)
                })
                .map(|v| (v["metadata"]["name"].as_str().unwrap().into(), v.clone()))
                .collect();
            Context {
                builtin: Some(crate::packages::Project {
                    manifest: core.manifest.map(|d| d.root.to_json()),
                    documents,
                    directory: Some("file:///builtin/".into()),
                }),
                core_package: spec.names().core_package.clone(),
                meta_prefix: read(&root.join("spec/names.yaml"))[0]["metaPrefix"]
                    .as_str()
                    .unwrap()
                    .into(),
                templates,
                record_source: "urn:conduit:first-build".into(),
                core_environment: environment,
            }
        })
        .clone()
    }
    fn expand(v: &mut Value, c: &Context) {
        match v {
            Value::String(s) if s.starts_with("@core#") => *s = c.qualified(&s[6..]),
            Value::Array(vs) => {
                for v in vs {
                    expand(v, c);
                }
            }
            Value::Object(m) => {
                for v in m.values_mut() {
                    expand(v, c);
                }
            }
            _ => {}
        }
    }
    impl Harness {
        fn new() -> Self {
            let c = context();
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
            let mut value =
                read(&root.join("conformance/fixtures/root-system-state.json")).remove(0);
            expand(&mut value, &c);
            let s = serde_json::from_value(value).unwrap();
            Self { c, s, sequence: 0 }
        }
        fn evidence(&mut self, time: &str) -> Evidence {
            self.sequence += 1;
            Evidence {
                time: time.into(),
                ids: (0..100)
                    .map(|n| {
                        format!(
                            "00000000-0000-4000-8000-{:012}",
                            self.sequence * 100 + n + 10000
                        )
                    })
                    .collect(),
                revisions: (0..20).map(|n| format!("r{n}")).collect(),
                suffixes: vec!["abcde".into(), "fghij".into(), "klmno".into()],
            }
        }
        fn call(&mut self, mut value: Value) -> Transition {
            if value.get("evidence").is_none() {
                value["evidence"] =
                    serde_json::to_value(self.evidence("2026-11-11T12:00:00Z")).unwrap();
            }
            let t = crate::step(&self.s, serde_json::from_value(value).unwrap(), &self.c).unwrap();
            self.s = t.state.clone();
            t
        }
        fn invoke(&mut self, op: &str, args: Value, key: Option<&str>) -> Transition {
            let mut v = json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":self.s.objects[4]["id"]},"tool":format!("{}.{op}",if matches!(op, "retire.v1" | "inspect.v1") {"root-system"} else {"management"}),"arguments":args}});
            if let Some(key) = key {
                v["key"] = json!(key);
            }
            self.call(v)
        }
        fn provider(&mut self) -> Value {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
            let project = crate::packages::Project {
                manifest: Some(
                    read(&root.join("conformance/fixtures/machine-provider/manifest.json"))
                        .remove(0),
                ),
                documents: read(&root.join("conformance/fixtures/machine-provider/documents.json")),
                directory: Some("file:///fixtures/machines/".into()),
            };
            let input = Input::RegisterProject {
                snapshots: vec![],
                builtin: self.c.builtin.clone().unwrap(),
                project,
                evidence: self.evidence("2026-11-11T12:00:00Z"),
                request: crate::Request {
                    sender: self.s.objects[4]["id"].as_str().unwrap().into(),
                    caller: self.s.objects[4]["id"].as_str().unwrap().into(),
                    target: self.s.objects[2]["id"].as_str().unwrap().into(),
                    conduit: self.s.objects[5]["id"].as_str().unwrap().into(),
                    operation: self.c.qualified("register.v1"),
                },
            };
            let t = crate::step(&self.s, input, &self.c).unwrap();
            assert_eq!(t.reply["accepted"], true, "{}", t.reply);
            self.s = t.state;
            self.s
                .objects
                .iter_mut()
                .find(|o| o["name"] == "machines")
                .unwrap()["state"] = json!("ready");
            provider(&self.s, &self.c).unwrap()["id"].clone()
        }
        fn open(&mut self, flags: Value, kinds: Value) -> (Value, Transition) {
            let mut args = json!({"expires":"2026-11-11T13:00:00Z"});
            args.as_object_mut()
                .unwrap()
                .extend(flags.as_object().unwrap().clone());
            let t = self.invoke("enroll-machine.v1", args, None);
            let id = t.external_intents[0]["inputs"]["enrollment"].clone();
            let ended=self.call(json!({"kind":"setup-report","invocation":t.reply["invocation"],"report":{"kind":"enroll","outcome":"succeeded","outputs":{"action":{"command":"ACTION_MARKER"},"message":"ACTION_MARKER","carriesSecret":true,"expires":"2026-11-11T13:00:00Z","evidence":kinds}}}));
            (id, ended)
        }
        fn machine(e: &Value, id: &str, evidence: Value) -> Value {
            json!({"id":id,"enrollment":e,"hostname":"worker.example","state":"pending","labels":{"kubernetes.io/hostname":"worker.example","kubernetes.io/os":"linux","kubernetes.io/arch":"amd64"},"evidence":evidence})
        }
        fn fingerprint() -> Value {
            json!([{"kind":"fingerprint","value":"aB+/xyz","where":"fixture","proves":"key"}])
        }
        fn report(&mut self, p: &Value, ms: Value) -> Transition {
            self.call(json!({"kind":"machine-report","provider":p,"outcome":"succeeded","outputs":{"machines":ms}}))
        }
        fn accept(&mut self, e: &Value, fp: &str, key: Option<&str>) -> Transition {
            self.invoke(
                "admit-machine.v1",
                json!({"enrollment":e,"machine":"m1","fingerprint":fp}),
                key,
            )
        }
        fn end_accept(&mut self, t: &Transition) -> Transition {
            self.call(json!({"kind":"setup-report","invocation":t.reply["invocation"],"report":{"kind":"accept-machine","outcome":"succeeded"}}))
        }
    }
    fn register(h: &mut Harness, documents: Vec<Value>) -> Transition {
        let input = Input::RegisterProject {
            snapshots: vec![],
            builtin: h.c.builtin.clone().unwrap(),
            project: crate::packages::Project {
                manifest: None,
                documents,
                directory: Some("file:///fixtures/registration/".into()),
            },
            evidence: h.evidence("2026-11-11T12:00:00Z"),
            request: crate::Request {
                sender: h.s.objects[4]["id"].as_str().unwrap().into(),
                caller: h.s.objects[4]["id"].as_str().unwrap().into(),
                target: h.s.objects[2]["id"].as_str().unwrap().into(),
                conduit: h.s.objects[5]["id"].as_str().unwrap().into(),
                operation: h.c.qualified("register.v1"),
            },
        };
        crate::step(&h.s, input, &h.c).unwrap()
    }
    #[test]
    fn prv013_registration_rejects_second_provider_and_other_owner() {
        let mut h = Harness::new();
        h.provider();
        let api = h.c.template("node").unwrap()["apiVersion"].clone();
        let second = json!({"apiVersion":api,"kind":"Resource","metadata":{"name":"second"},"from":"fixture-machine","owner":"system"});
        let t = register(&mut h, vec![second.clone()]);
        assert_eq!(t.reply["reason"], "invalid");
        assert!(
            t.reply["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v["ruleId"] == "PRV-013")
        );
        assert_eq!(t.state.objects, h.s.objects);
        let root = h.s.objects[4]["id"].clone();
        h.s.objects.push(json!({"id":"other-pool","name":"other","kind":"Pool","from":h.c.qualified("pool"),"type":h.c.qualified("pool.v1"),"state":"ready"}));
        h.s.objects.push(json!({"id":"other-authority","kind":"Conduit","consumer":root,"target":"other-pool","open":true,"state":"ready","operations":[h.c.qualified("register.v1")]}));
        h.s.objects
            .iter_mut()
            .find(|o| o["name"] == "machines")
            .unwrap()["state"] = json!("retired");
        let mut other = second;
        other["owner"] = json!("other");
        let t = register(&mut h, vec![other]);
        assert_eq!(t.reply["reason"], "invalid");
        assert!(
            t.reply["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v["ruleId"] == "PRV-013"),
            "{}",
            t.reply
        );
    }
    #[test]
    fn enrollment_invalid_output_is_unknown_and_failed_calls_open_nothing() {
        for (outcome, outputs, ending) in [
            (
                "succeeded",
                json!({"action":{"command":"BAD_ACTION"}}),
                "unknown",
            ),
            ("failed", Value::Null, "failed"),
            ("unknown", Value::Null, "unknown"),
        ] {
            let mut h = Harness::new();
            h.provider();
            let t = h.invoke(
                "enroll-machine.v1",
                json!({"expires":"2026-11-11T13:00:00Z"}),
                None,
            );
            let ended=h.call(json!({"kind":"setup-report","invocation":t.reply["invocation"],"report":{"kind":"enroll","outcome":outcome,"outputs":outputs}}));
            assert_eq!(
                ended.reply["_meta"][format!("{}outcome", h.c.meta_prefix)],
                ending
            );
            assert!(h.s.enrollments.is_empty());
            assert_eq!(
                ended.reply["structuredContent"]["diagnostics"][0]["ruleId"],
                if outcome == "succeeded" {
                    "PRV-005"
                } else {
                    "ENR-001"
                }
            );
            assert!(!serde_json::to_string(&h.s).unwrap().contains("BAD_ACTION"));
        }
    }
    #[test]
    fn admission_unknown_until_report_and_recovery_omits_every_nonlocal_node() {
        let mut h = Harness::new();
        let p = h.provider();
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        let mut m = Harness::machine(&e, "m1", Harness::fingerprint());
        h.report(&p, json!([m]));
        let t = h.accept(&e, "aB+/xyz", None);
        h.end_accept(&t);
        let node = |s: &State| {
            s.objects.iter().find(|o| o["name"] == "worker").unwrap()["status"]["conditions"][0]
                .clone()
        };
        assert_eq!(node(&h.s)["reason"], "NoReportSinceAdmission");
        h.report(&p, json!([m]));
        assert_eq!(node(&h.s)["reason"], "MachineReportedPending");
        m["state"] = json!("reachable");
        h.report(&p, json!([m]));
        assert_eq!(node(&h.s)["status"], "True");
        let local = h.s.objects[1].clone();
        h.call(json!({"kind":"recover"}));
        assert_eq!(node(&h.s)["reason"], "AwaitingReportAfterRecovery");
        assert_eq!(h.s.objects[1], local);
    }
    #[test]
    fn node_loss_invalidates_instance_observations_in_inspection() {
        for ending in ["unreachable", "omitted", "recover"] {
            let mut h = Harness::new();
            // This fixture's system Conduit lists register/retire only.
            h.s.objects
                .iter_mut()
                .find(|o| o["name"] == "root-system")
                .unwrap()["operations"]
                .as_array_mut()
                .unwrap()
                .push(json!(h.c.qualified("inspect.v1")));
            let p = h.provider();
            let (e, _) = h.open(json!({}), json!(["fingerprint"]));
            let mut m = Harness::machine(&e, "m1", Harness::fingerprint());
            h.report(&p, json!([m]));
            let call = h.accept(&e, "aB+/xyz", None);
            let admitted = h.end_accept(&call);
            let node = admitted.reply["structuredContent"]["node"]["id"].clone();
            m["state"] = json!("reachable");
            h.report(&p, json!([m]));
            let mut instance = h.s.objects[2].clone();
            instance["id"] = json!("00000000-0000-4000-8000-000000099999");
            instance["name"] = json!("observed");
            instance["placement"] = node;
            instance["status"] = json!({});
            crate::lifecycle::observe_run(&mut instance, "running", "2026-11-11T11:59:00Z")
                .unwrap();
            instance["status"]["conditions"].as_array_mut().unwrap().push(json!({
                "type":"Healthy", "status":"True", "source":"urn:fixture:health",
                "lastHeartbeatTime":"2026-11-11T11:58:00Z", "lastTransitionTime":"2026-11-11T11:58:00Z"
            }));
            let observed = instance["status"]["conditions"].clone();
            let lifecycle = instance["state"].clone();
            let reference = json!({"kind":"Resource","name":"observed"});
            h.s.objects.push(instance.clone());
            instance["id"] = json!("00000000-0000-4000-8000-000000099998");
            instance["name"] = json!("unobserved");
            instance["status"] = json!({});
            h.s.objects.push(instance.clone());
            instance["id"] = json!("00000000-0000-4000-8000-000000099997");
            instance["name"] = json!("unaffected");
            instance["placement"] = h.s.objects[1]["id"].clone();
            instance["status"]["conditions"] = observed.clone();
            h.s.objects.push(instance.clone());
            match ending {
                "recover" => {
                    h.call(json!({"kind":"recover"}));
                }
                "omitted" => {
                    h.report(&p, json!([]));
                }
                _ => {
                    m["state"] = json!(ending);
                    h.report(&p, json!([m]));
                }
            }
            let t = h.invoke("inspect.v1", json!({"object":reference}), None);
            let status = &t.reply["structuredContent"]["status"];
            assert_eq!(status["state"], lifecycle, "{}", t.reply);
            for (i, old) in observed.as_array().unwrap().iter().enumerate() {
                let current = &status["conditions"][i];
                assert_eq!(current["status"], "Unknown", "{ending}: {}", t.reply);
                assert_eq!(current["lastHeartbeatTime"], old["lastHeartbeatTime"]);
                assert_eq!(current["source"], old["source"]);
                assert_eq!(current["reason"], "NodeUnavailable");
                assert!(current["message"].as_str().unwrap().contains("OBS-003"));
            }
            let t = h.invoke(
                "inspect.v1",
                json!({"object":{"kind":"Resource","name":"unobserved"}}),
                None,
            );
            assert_eq!(
                t.reply["structuredContent"]["status"]["conditions"][0]["status"],
                "Unknown"
            );
            assert!(
                t.reply["structuredContent"]["status"]["conditions"][0]
                    .get("lastHeartbeatTime")
                    .is_none()
            );
            assert_eq!(
                h.s.objects
                    .iter()
                    .find(|o| o["name"] == "unaffected")
                    .unwrap(),
                &instance
            );
        }
    }
    #[test]
    fn admission_fixes_last_reported_labels_but_keeps_checked_identity() {
        let mut h = Harness::new();
        let p = h.provider();
        let (e, _) = h.open(json!({"labels":{"site":"bound"}}), json!(["fingerprint"]));
        let mut m = Harness::machine(&e, "m1", Harness::fingerprint());
        m["labels"]["site"] = json!("bound");
        h.report(&p, json!([m]));
        let call = h.accept(&e, "aB+/xyz", None);
        m["labels"]["kubernetes.io/os"] = json!("corrected");
        m["labels"]["kubernetes.io/arch"] = json!("arm64");
        // A later report cannot replace the enrollment's bound labels or
        // change the identity/evidence that passed ENR-006.
        m["labels"]["site"] = json!("unbound");
        m["labels"]["extra"] = json!("ignored");
        m["evidence"] = json!([]);
        h.report(&p, json!([m]));
        h.report(&p, json!([]));
        h.end_accept(&call);
        let node = h.s.objects.iter().find(|o| o["name"] == "worker").unwrap();
        assert_eq!(
            node["status"]["labels"],
            json!({"kubernetes.io/hostname":"worker.example","kubernetes.io/os":"corrected","kubernetes.io/arch":"arm64","site":"bound"})
        );
        assert_eq!(node["status"]["machine"]["id"], "m1");
        assert_eq!(h.s.enrollments[0]["fingerprintCompared"], true);
    }
    #[test]
    fn panel_retains_join_when_report_omits_or_changes_attribution() {
        let mut h = Harness::new();
        let p = h.provider();
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        let mut m = Harness::machine(&e, "m1", Harness::fingerprint());
        h.report(&p, json!([m]));
        for attribution in [None, Some(json!("not-a-joined-enrollment"))] {
            m.as_object_mut().unwrap().remove("enrollment");
            if let Some(value) = attribution {
                m["enrollment"] = value;
            }
            h.report(&p, json!([m]));
            let view = panel(&h.s, "2026-11-11T12:00:00Z");
            assert_eq!(view["machines"][0]["enrollment"], e);
            let failures = view["machines"][0]["checks"].as_array().unwrap();
            assert!(failures.iter().any(|f| f["check"] == 1));
            assert!(!failures.iter().any(|f| f["check"] == 2));
            assert!(view["enrollments"][0].get("action").is_none());
        }
    }
    #[test]
    fn hostname_comparisons_are_canonical_for_reports_admission_and_local() {
        let mut h = Harness::new();
        let p = h.provider();
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        let mut m = Harness::machine(&e, "m1", Harness::fingerprint());
        let mut duplicate = m.clone();
        duplicate["id"] = json!("other");
        duplicate["hostname"] = json!("WORKER.EXAMPLE");
        duplicate["labels"]["kubernetes.io/hostname"] = duplicate["hostname"].clone();
        let t = h.report(&p, json!([m, duplicate]));
        assert_eq!(t.reply["used"], false);
        assert!(h.s.machine_report.is_none());
        // Check 3 also protects a candidate against a different pending id
        // using a case variant, independently of duplicate-report rejection.
        h.s.machine_report = Some(json!({"provider":p,"outputs":{"machines":[duplicate]}}));
        let failures = checks(
            &h.s,
            Some(&h.s.enrollments[0]),
            Some(&m),
            provider(&h.s, &h.c),
            "2026-11-11T12:00:00Z",
        );
        assert!(failures.iter().any(|f| f["check"] == 3));
        h.s.machine_report = None;
        m["hostname"] = json!(
            h.s.objects[1]["status"]["hostname"]
                .as_str()
                .unwrap()
                .to_uppercase()
        );
        m["labels"]["kubernetes.io/hostname"] = m["hostname"].clone();
        h.report(&p, json!([m]));
        let t = h.accept(&e, "aB+/xyz", None);
        assert_eq!(
            t.reply["_meta"][format!("{}reason", h.c.meta_prefix)],
            "conflict"
        );
        assert!(t.reply.to_string().contains("check 3"));
        assert!(t.external_intents.is_empty());
        let t = h.invoke("refuse-machine.v1", json!({"machine":"m1"}), None);
        assert_eq!(
            t.reply["_meta"][format!("{}reason", h.c.meta_prefix)],
            "conflict"
        );
        assert!(t.external_intents.is_empty());
        m = Harness::machine(&e, "m1", Harness::fingerprint());
        h.report(&p, json!([m]));
        let call = h.accept(&e, "aB+/xyz", None);
        h.end_accept(&call);
        m["hostname"] = json!("WORKER.EXAMPLE");
        m["labels"]["kubernetes.io/hostname"] = m["hostname"].clone();
        m["state"] = json!("reachable");
        h.report(&p, json!([m]));
        let n = h.s.objects.iter().find(|o| o["name"] == "worker").unwrap();
        assert_eq!(n["status"]["conditions"][0]["status"], "True");
        assert!(same_hostname(
            &json!(" Worker.Example "),
            &json!("worker.example")
        ));
    }
    #[test]
    fn headless_minima_and_comparison_exception() {
        for (weaker, kinds, ev, success) in [
            (false, json!(["fingerprint"]), Harness::fingerprint(), false),
            (
                false,
                json!(["fingerprint", "attestation"]),
                json!([{"kind":"fingerprint","value":"aB+/xyz","where":"fixture","proves":"key"},{"kind":"attestation","value":"verified","where":"fixture","proves":"platform"}]),
                true,
            ),
            (
                true,
                json!(["attestation"]),
                json!([{"kind":"attestation","value":"verified","where":"fixture","proves":"platform"}]),
                true,
            ),
            (true, json!(["fact"]), json!([]), false),
        ] {
            let mut h = Harness::new();
            let p = h.provider();
            let (e, t) = h.open(json!({"headless":true,"acceptWeakerCheck":weaker}), kinds);
            assert_eq!(
                t.reply["_meta"][format!("{}outcome", h.c.meta_prefix)],
                if success { "succeeded" } else { "failed" }
            );
            if success {
                h.report(&p, json!([Harness::machine(&e, "m1", ev)]));
                let t = h.accept(&e, if weaker { "ignored" } else { "aB+/xyz" }, None);
                assert_eq!(t.external_intents.len(), 1);
                let t = h.end_accept(&t);
                assert_eq!(t.state.enrollments[0]["fingerprintCompared"], !weaker);
            }
        }
    }
    #[test]
    fn one_failed_acceptance_does_not_erase_another_calls_checked_machine() {
        let mut h = Harness::new();
        let p = h.provider();
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        h.report(
            &p,
            json!([Harness::machine(&e, "m1", Harness::fingerprint())]),
        );
        let first = h.accept(&e, "aB+/xyz", Some("first"));
        let second = h.accept(&e, "aB+/xyz", Some("second"));
        h.call(json!({"kind":"setup-report","invocation":second.reply["invocation"],"report":{"kind":"accept-machine","outcome":"failed"}}));
        assert_eq!(h.s.enrollments[0]["state"], "open");
        let ended = h.end_accept(&first);
        assert_eq!(
            ended.reply["_meta"][format!("{}outcome", h.c.meta_prefix)],
            "succeeded"
        );
        assert_eq!(ended.reply["structuredContent"]["node"]["name"], "worker");
        assert!(h.s.enrollments[0].get("accepting").is_none());
    }
    #[test]
    fn repeat_digest_covers_typed_input_without_retaining_it() {
        let mut h = Harness::new();
        let p = h.provider();
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        h.report(
            &p,
            json!([Harness::machine(&e, "m1", Harness::fingerprint())]),
        );
        let t = h.accept(&e, "aB+/xyz", Some("admission"));
        let done = h.end_accept(&t);
        let repeat = h.accept(&e, "aB+/xyz", Some("admission"));
        assert_eq!(repeat.reply, done.reply);
        assert!(repeat.external_intents.is_empty());
        let changed = h.accept(&e, "aB+/xyZ", Some("admission"));
        assert_eq!(
            changed.reply["_meta"][format!("{}reason", h.c.meta_prefix)],
            "conflict"
        );
        assert!(!serde_json::to_string(&changed).unwrap().contains("aB+/xyZ"));
    }
    #[test]
    fn refuse_before_join_and_represented_machine_conflict() {
        let mut h = Harness::new();
        let p = h.provider();
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        let t = h.invoke("refuse-machine.v1", json!({"enrollment":e}), None);
        assert!(t.external_intents.is_empty());
        assert_eq!(h.s.enrollments[0]["state"], "refused");
        h.report(
            &p,
            json!([Harness::machine(&e, "m1", Harness::fingerprint())]),
        );
        assert!(
            panel(&h.s, "2026-11-11T12:00:00Z")["machines"][0]
                .get("enrollment")
                .is_none()
        );
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        h.report(
            &p,
            json!([Harness::machine(&e, "m1", Harness::fingerprint())]),
        );
        let t = h.accept(&e, "aB+/xyz", None);
        h.end_accept(&t);
        let t = h.invoke("refuse-machine.v1", json!({"machine":"m1"}), None);
        assert_eq!(
            t.reply["_meta"][format!("{}reason", h.c.meta_prefix)],
            "conflict"
        );
        assert!(t.external_intents.is_empty());
        let mut local = Harness::machine(&e, "local-id", Harness::fingerprint());
        local.as_object_mut().unwrap().remove("enrollment");
        local["hostname"] = h.s.objects[1]["status"]["hostname"].clone();
        local["labels"] = h.s.objects[1]["status"]["labels"].clone();
        h.report(&p, json!([local]));
        let t = h.invoke("refuse-machine.v1", json!({"machine":"local-id"}), None);
        assert_eq!(
            t.reply["_meta"][format!("{}reason", h.c.meta_prefix)],
            "conflict"
        );
        assert!(
            t.reply["structuredContent"]["diagnostics"][0]["message"]["text"]
                .as_str()
                .unwrap()
                .contains("local")
        );
        assert!(t.external_intents.is_empty());
    }
    #[test]
    fn readmission_after_retirement_does_not_revive_the_old_node() {
        let mut h = Harness::new();
        let p = h.provider();
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        h.report(
            &p,
            json!([Harness::machine(&e, "m1", Harness::fingerprint())]),
        );
        let t = h.accept(&e, "aB+/xyz", None);
        let first = h.end_accept(&t).reply["structuredContent"]["node"]["id"].clone();
        let t = h.invoke(
            "retire.v1",
            json!({"instance":{"kind":"Resource","name":"worker"}}),
            None,
        );
        assert!(t.external_intents.is_empty());
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        h.report(
            &p,
            json!([Harness::machine(&e, "m1", Harness::fingerprint())]),
        );
        let t = h.accept(&e, "aB+/xyz", None);
        let second = h.end_accept(&t).reply["structuredContent"]["node"].clone();
        assert_ne!(first, second["id"]);
        assert_eq!(second["name"], "worker");
        assert!(!h.s.objects.iter().any(|o| o["id"] == first));
    }
    #[test]
    fn retiring_provider_closes_enrollments_and_omits_nodes() {
        let mut h = Harness::new();
        let p = h.provider();
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        h.report(
            &p,
            json!([Harness::machine(&e, "m1", Harness::fingerprint())]),
        );
        let t = h.accept(&e, "aB+/xyz", None);
        h.end_accept(&t);
        let mut m = Harness::machine(&e, "m1", Harness::fingerprint());
        m["state"] = json!("reachable");
        h.report(&p, json!([m]));
        h.open(json!({}), json!(["fingerprint"]));
        let t = h.invoke(
            "retire.v1",
            json!({"instance":{"kind":"Resource","name":"machines"}}),
            None,
        );
        assert!(t.external_intents.is_empty());
        assert_eq!(h.s.enrollments[1]["state"], "refused");
        let n = h.s.objects.iter().find(|o| o["name"] == "worker").unwrap();
        assert_eq!(n["status"]["conditions"][0]["status"], "Unknown");
        assert!(provider(&h.s, &h.c).is_none());
    }
    #[test]
    fn finishing_an_old_providers_retirement_does_not_omit_replacement_nodes() {
        let mut h = Harness::new();
        let old = h.provider();
        let mut replacement = access::object(&h.s, &old).unwrap().clone();
        let id = json!("00000000-0000-4000-8000-000000900000");
        replacement["id"] = id.clone();
        replacement["name"] = json!("replacement");
        h.s.allocated_ids.insert(id.as_str().unwrap().into());
        let previous = h.s.objects.iter_mut().find(|o| o["id"] == old).unwrap();
        previous["state"] = json!("retired");
        previous["shutdown"] = json!("unknown");
        h.s.objects.push(replacement);
        let (e, _) = h.open(json!({}), json!(["fingerprint"]));
        let mut m = Harness::machine(&e, "m1", Harness::fingerprint());
        h.report(&id, json!([m]));
        let t = h.accept(&e, "aB+/xyz", None);
        h.end_accept(&t);
        m["state"] = json!("reachable");
        h.report(&id, json!([m]));
        let before =
            h.s.objects
                .iter()
                .find(|o| o["name"] == "worker")
                .unwrap()
                .clone();
        let report = h.s.machine_report.clone();
        h.invoke(
            "retire.v1",
            json!({"instance":{"kind":"Resource","name":"machines"}}),
            None,
        );
        assert_eq!(
            h.s.objects.iter().find(|o| o["name"] == "worker").unwrap(),
            &before
        );
        assert_eq!(h.s.machine_report, report);
        assert_eq!(provider(&h.s, &h.c).unwrap()["id"], id);
    }
    #[test]
    fn recover_ends_all_machine_verbs_unknown_and_ignores_late_endings() {
        for op in ["enroll-machine.v1", "admit-machine.v1", "refuse-machine.v1"] {
            let mut h = Harness::new();
            let p = h.provider();
            let (e, _) = h.open(json!({}), json!(["fingerprint"]));
            h.report(
                &p,
                json!([Harness::machine(&e, "m1", Harness::fingerprint())]),
            );
            let args = if op == "enroll-machine.v1" {
                json!({"expires":"2026-11-11T13:00:00Z"})
            } else {
                json!({"enrollment":e,"machine":"m1","fingerprint":"aB+/xyz"})
            };
            let t = h.invoke(op, args, None);
            let id = t.reply["invocation"].clone();
            h.call(json!({"kind":"recover"}));
            assert!(
                h.s.invocations
                    .iter()
                    .any(|v| v["id"] == id && v["state"] == "unknown")
            );
            let t=h.call(json!({"kind":"setup-report","invocation":id,"report":{"kind":"accept-machine","outcome":"succeeded"}}));
            assert_eq!(t.reply["ignored"], true);
            assert!(t.external_intents.is_empty());
        }
    }
    #[test]
    fn panel_uses_the_actual_join_and_preserves_its_history_after_expiry() {
        let mut h = Harness::new();
        let p = h.provider();
        let (first, _) = h.open(json!({}), json!(["fingerprint"]));
        h.report(
            &p,
            json!([Harness::machine(&first, "m1", Harness::fingerprint())]),
        );
        let (second, _) = h.open(json!({}), json!(["fingerprint"]));
        h.report(
            &p,
            json!([Harness::machine(&second, "m1", Harness::fingerprint())]),
        );
        assert_eq!(
            panel(&h.s, "2026-11-11T12:00:00Z")["machines"][0]["enrollment"],
            second
        );
        h.report(
            &p,
            json!([Harness::machine(&first, "m1", Harness::fingerprint())]),
        );
        let mut without_attribution = Harness::machine(&first, "m1", Harness::fingerprint());
        without_attribution
            .as_object_mut()
            .unwrap()
            .remove("enrollment");
        h.report(&p, json!([without_attribution]));
        assert_eq!(
            panel(&h.s, "2026-11-11T12:00:00Z")["machines"][0]["enrollment"],
            first
        );
        // Expiry preserves the recorded attribution.
        let evidence = h.evidence("2026-11-11T13:00:00Z");
        h.call(json!({"kind":"clock","evidence":evidence}));
        let view = panel(&h.s, "2026-11-11T13:00:00Z");
        assert_eq!(view["enrollments"], json!([]));
        assert_eq!(view["machines"][0]["enrollment"], first);
        assert!(
            view["machines"][0]["checks"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v["check"] == 2)
        );
        let evidence = h.evidence("2026-11-11T13:01:00Z");
        h.call(json!({"kind":"machine-report","evidence":evidence,"provider":p,"outcome":"succeeded","outputs":{"machines":[Harness::machine(&first,"late",Harness::fingerprint())]}}));
        assert!(
            panel(&h.s, "2026-11-11T13:00:00Z")["machines"][0]
                .get("enrollment")
                .is_none()
        );
    }
    #[test]
    fn timed_input_expires_secrets_before_deciding_and_panel_filters_without_mutating() {
        let mut h = Harness::new();
        h.provider();
        h.open(json!({}), json!(["fingerprint"]));
        assert_eq!(
            panel(&h.s, "2026-11-11T13:00:00Z")["enrollments"],
            json!([])
        );
        assert!(
            serde_json::to_string(&h.s)
                .unwrap()
                .contains("ACTION_MARKER")
        );
        let evidence = h.evidence("2026-11-11T13:00:00Z");
        let t = h.call(json!({"kind":"recover","evidence":evidence}));
        assert_eq!(t.state.enrollments[0]["state"], "expired");
        assert!(
            t.records
                .iter()
                .any(|r| r["type"] == "enrollment-change" && r["data"]["transition"] == "expired")
        );
        for key in ["action", "message", "carriesSecret"] {
            assert!(t.state.enrollments[0].get(key).is_none());
        }
        assert!(
            !serde_json::to_string(&h.s)
                .unwrap()
                .contains("ACTION_MARKER")
        );
    }
    #[test]
    fn unused_reports_only_record_two_endings_and_measure_duplicate_report_growth() {
        let mut h = Harness::new();
        let p = h.provider();
        h.report(&p, json!([]));
        let before = serde_json::to_vec(&h.s).unwrap().len();
        let t = h.report(&p, json!([]));
        let growth = serde_json::to_vec(&h.s).unwrap().len() - before;
        eprintln!("049a1 compact state growth per identical empty machine report: {growth} bytes");
        assert_eq!(t.records.len(), 2);
        let m = Harness::machine(&json!("unattributed"), "m1", Harness::fingerprint());
        let mut duplicate = m.clone();
        duplicate["id"] = json!("m2");
        for (outcome, outputs, recorded) in [
            ("failed", json!({}), "failed"),
            ("succeeded", json!({"machines":"bad"}), "unknown"),
            (
                "succeeded",
                json!({"machines":[m.clone(),duplicate]}),
                "succeeded",
            ),
        ] {
            let objects = h.s.objects.clone();
            let latest = h.s.machine_report.clone();
            let t = h.call(
                json!({"kind":"machine-report","provider":p,"outcome":outcome,"outputs":outputs}),
            );
            assert_eq!(t.records.len(), 2);
            assert_eq!(t.records[1]["data"]["outcome"], recorded);
            assert_eq!(h.s.objects, objects);
            assert_eq!(h.s.machine_report, latest);
            assert_eq!(t.reply["used"], false);
        }
    }
    #[test]
    fn node_without_machine_emits_only_descriptive_fields_and_matches_no_report() {
        let mut h = Harness::new();
        let p = h.provider();
        h.s.objects.push(json!({"id":"node","name":"manual","type":h.c.qualified("node.v1"),"state":"ready","status":{"hostname":"manual.example","labels":{},"conditions":[{"type":"Ready","status":"True"}]}}));
        let provider = access::object(&h.s, &p).unwrap().clone();
        let mut o = json!({"id":"subject","placement":"node","config":{}});
        o["runtime"] = p.clone();
        h.s.objects.push(o);
        h.s.invocations
            .push(json!({"id":"setup","object":"subject","caller":"root"}));
        let evidence = h.evidence("2026-11-11T12:00:00Z");
        let mut t = Transition {
            state: h.s.clone(),
            decisions: vec![],
            records: vec![],
            external_intents: vec![],
            reply: Value::Null,
            request_replies: vec![],
        };
        let i = t.state.invocations.len() - 1;
        crate::lifecycle::emit(
            &mut t,
            &mut Allocation {
                evidence: &evidence,
                cursor: 0,
            },
            &h.c,
            i,
            "inspect-run.v1",
            json!({}),
        )
        .unwrap();
        assert_eq!(
            t.external_intents[0]["inputs"]["machine"],
            json!({"hostname":"manual.example","labels":{}})
        );
        h.report(&provider["id"], json!([]));
        assert_eq!(
            h.s.objects.iter().find(|o| o["id"] == "node").unwrap()["status"]["conditions"][0]["status"],
            "Unknown"
        );
    }
}
