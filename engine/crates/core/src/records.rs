//! Record reads, retention and explicit repair. No I/O or automatic effects.
use crate::{
    Allocation, Context, Result, State, Transition, access, decision, packages, pending,
    registration, requests,
};
use serde_json::{Value, json};

pub(crate) fn handles(op: &str, c: &Context) -> bool {
    [
        "inspect.v1",
        "list.v1",
        "purge.v1",
        "replace.v1",
        "register.v1",
        "answer.v1",
    ]
    .iter()
    .any(|n| op == c.qualified(n))
}

fn template<'a>(s: &'a State, args: &Value) -> Option<&'a Value> {
    s.objects.iter().find(|o| {
        o.get("spec").is_some()
            && o["kind"] == args["template"]["kind"]
            && o["name"] == args["template"]["metadata"]["name"]
    })
}

use crate::explain::Rejection as PreflightRejection;

pub(crate) fn preflight(
    s: &State,
    route: &Value,
    c: &Context,
    a: &Allocation<'_>,
) -> Result<Option<PreflightRejection>> {
    let args = &route["arguments"];
    if route["operation"] == c.qualified("answer.v1") {
        let time = &a.evidence.time;
        let id = args["action"].as_str().ok_or("answer action has no ID")?;
        let action = pending::action_named(s, id, Some(time), c)
            .ok_or("answer action absent after access")?;
        let tool = route["tool"].as_str().ok_or("answer route has no tool")?;
        let (reason, rule, text) = if !pending::accepts_answer(&action["kind"], &args["answer"]) {
            (
                "invalid",
                "PND-009",
                if action["kind"] == "notice" {
                    format!(
                        "{tool} reports invalid under PND-009: answer.v1 answers a notice only acknowledged."
                    )
                } else if action["kind"] == "sign-in" {
                    format!(
                        "{tool} reports invalid under PND-009: answer.v1 does not answer a sign-in action; sign-in.v1 answers it signed-in once the expected account is verified, and retirement answers it void."
                    )
                } else {
                    format!(
                        "{tool} reports invalid under PND-009: answer.v1 answers a setup action only dismissed; a further set-up.v1 answers it retried, and retire.v1 or a cascade's retirement answers it retired."
                    )
                },
            )
        } else if let Some(value) = action["answer"]["value"].as_str() {
            (
                "conflict",
                "PND-003",
                format!(
                    "{tool} reports conflict under PND-003: action {id} is already answered {value}; an answered or expired action takes no further answer."
                ),
            )
        } else {
            return Ok(None);
        };
        let mut rejection = PreflightRejection::new(reason, rule);
        rejection.diagnostics =
            Some(json!([{"ruleId":rule,"level":"error","message":{"text":text}}]));
        return Ok(Some(rejection));
    }
    if route["operation"] == c.qualified("inspect.v1") {
        let o = access::object(s, &args["object"])
            .ok_or("inspection object absent after validation")?;
        if args.get("expectedId").is_some_and(|id| *id != o["id"]) {
            return Ok(Some(PreflightRejection::new("conflict", "LIF-037")));
        }
    }
    if route["operation"] == c.qualified("replace.v1") {
        let Some(o) = template(s, args) else {
            return Ok(Some(PreflightRejection::new("invalid", "OPS-029")));
        };
        // Validate the new project before comparing revisions (OPS-023).
        let trial = replacement(s, args, c, a)?;
        if trial.reply["accepted"] != true {
            return Ok(Some(PreflightRejection::new("invalid", "SUB-009")));
        }
        if o["revision"] != args["revision"] {
            return Ok(Some(PreflightRejection::new("conflict", "REV-002")));
        }
    }
    if route["operation"] == c.qualified("register.v1") {
        let mut trial = Transition {
            state: s.clone(),
            decisions: vec![],
            records: vec![],
            external_intents: vec![],
            reply: Value::Null,
            request_replies: vec![],
            input_required: vec![],
        };
        if let Some(mut rejection) = register_project(&mut trial, args, c, a, route)? {
            // Keep the chosen reason class and rule even when the diagnostics sort differently.
            rejection.diagnostics = Some(trial.reply["diagnostics"].clone());
            return Ok(Some(rejection));
        }
    }
    Ok(None)
}

fn replacement(s: &State, args: &Value, c: &Context, a: &Allocation<'_>) -> Result<Transition> {
    let old = template(s, args).ok_or("replacement template absent")?;
    let mut state = s.clone();
    state.objects.retain(|o| o["id"] != old["id"]);
    let mut docs = args["contracts"].as_array().cloned().unwrap_or_default();
    if docs.iter().any(|d| !packages::contract(d)) {
        return Err("replacement contracts were not validated by inputSchema".into());
    }
    docs.push(args["template"].clone());
    let builtin = c
        .builtin
        .clone()
        .ok_or("built-in project missing at boundary")?;
    let mut trial = Transition {
        state,
        decisions: vec![],
        records: vec![],
        external_intents: vec![],
        reply: Value::Null,
        request_replies: vec![],
        input_required: vec![],
    };
    let evidence = crate::Evidence {
        time: a.evidence.time.clone(),
        ids: a.evidence.ids[a.cursor..].to_vec(),
        revisions: a.evidence.revisions.clone(),
        suffixes: a.evidence.suffixes.clone(),
    };
    let request = crate::Request {
        sender: String::new(),
        caller: String::new(),
        conduit: String::new(),
        target: String::new(),
        operation: c.qualified("replace.v1"),
    };
    registration::prepare(
        &mut trial,
        packages::catalog_entries(s, s.preinstalled_snapshots.clone()),
        builtin,
        packages::Project {
            manifest: args.get("manifest").cloned(),
            documents: docs,
            directory: None,
        },
        &evidence,
        &request,
        c,
    )?;
    Ok(trial)
}

fn register_project(
    t: &mut Transition,
    args: &Value,
    c: &Context,
    a: &Allocation<'_>,
    route: &Value,
) -> Result<Option<PreflightRejection>> {
    let evidence = crate::Evidence {
        time: a.evidence.time.clone(),
        ids: a.evidence.ids[a.cursor..].to_vec(),
        revisions: a.evidence.revisions.clone(),
        suffixes: a.evidence.suffixes.clone(),
    };
    let request = crate::Request {
        sender: route["consumer"].as_str().unwrap_or("").into(),
        caller: route["caller"].as_str().unwrap_or("").into(),
        conduit: route["conduit"].as_str().unwrap_or("").into(),
        target: route["target"].as_str().unwrap_or("").into(),
        operation: c.qualified("register.v1"),
    };
    registration::prepare(
        t,
        packages::catalog_entries(&t.state, t.state.preinstalled_snapshots.clone()),
        c.builtin
            .clone()
            .ok_or("built-in project missing at boundary")?,
        packages::Project {
            manifest: args.get("manifest").cloned(),
            documents: args["documents"].as_array().cloned().unwrap_or_default(),
            directory: None,
        },
        &evidence,
        &request,
        c,
    )
}

/// Name references in resolved inputs by ID, never by the reusable spelling of a name.
fn names(value: &Value, id: &Value) -> bool {
    match value {
        Value::Object(m) => m.get("id") == Some(id) || m.values().any(|v| names(v, id)),
        Value::Array(a) => a.iter().any(|v| names(v, id)),
        _ => false,
    }
}
fn invocation_names(v: &Value, id: &Value) -> bool {
    v["target"] == *id
        || v["object"] == *id
        || names(&v["arguments"], id)
        || v["input_objects"]
            .as_array()
            .is_some_and(|ids| ids.contains(id))
}
fn record_names(r: &Value, id: &Value) -> bool {
    r["subject"] == *id
        || ["target", "object", "caller", "consumer", "conduit"]
            .iter()
            .any(|f| r["data"][f] == *id)
        || names(&r["data"]["inputs"], id)
        || r["type"] == "pending-action-answered"
            && r["data"]["answer"]["subject"]
                .as_array()
                .is_some_and(|subjects| subjects.iter().any(|subject| subject["id"] == *id))
}

pub(crate) fn export(t: &mut Transition, unreadable: &[String]) -> Result<()> {
    if unreadable
        .iter()
        .any(|id| !t.state.records.iter().any(|r| r["id"] == *id))
    {
        return Err("unreadable range names no stored record".into());
    }
    let mut gaps = t.state.record_gaps.clone();
    for r in t
        .state
        .records
        .iter()
        .filter(|r| unreadable.iter().any(|id| r["id"] == *id))
    {
        gaps.push(json!({"source":r["source"],"ids":[r["id"]],"cause":"unreadable"}));
    }
    t.reply = json!({"records":t.state.records.iter().filter(|r| !unreadable.iter().any(|id| r["id"] == *id)).collect::<Vec<_>>(),"gaps":gaps,"complete":gaps.is_empty()});
    t.decisions.push(decision(
        "export-records",
        &["REC-002", "REC-003", "REC-005", "REC-006"],
        &["DL8"],
        json!({}),
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
    let mut outputs;
    if v["operation"] == c.qualified("answer.v1") {
        let time = &a.evidence.time;
        let id = args["action"].as_str().ok_or("answer action has no ID")?;
        let value = args["answer"]
            .as_str()
            .ok_or("answer value is not a string")?;
        let action = pending::action_named(&t.state, id, Some(time), c)
            .ok_or("answer action absent after preflight")?;
        let action = pending::store_answer(
            &mut t.state,
            action,
            pending::invocation_answer(value, &v, time),
        );
        requests::finish(t, a, c, i, "succeeded", json!({}), json!([]), None)?;
        let ended = t
            .records
            .iter_mut()
            .rev()
            .find(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == v["id"])
            .ok_or("answer invocation has no ending record")?;
        ended["data"]["pendingAction"] = json!([id]);
        let stored = t
            .state
            .records
            .iter_mut()
            .rev()
            .find(|r| r["id"] == ended["id"])
            .ok_or("answer invocation ending record absent from state")?;
        *stored = ended.clone();
        pending::record_answer(t, a, c, &action)?;
        t.decisions.push(decision(
            "answer-pending-action",
            &["PND-003", "PND-006", "PND-009", "PND-012"],
            &["DL19"],
            json!({"action":id,"answer":value}),
            vec![],
        ));
        return Ok(());
    } else if v["operation"] == c.qualified("register.v1") {
        let start = t.records.len();
        let route = json!({"consumer":v["sender"],"caller":v["caller"],"conduit":v["conduit"],"target":v["target"]});
        register_project(t, args, c, a, &route)?;
        if t.reply["accepted"] != true {
            return Err("registration changed between validation and dispatch".into());
        }
        packages::format_submission_notes(&mut t.reply["diagnostics"]);
        outputs = json!({"objects":t.reply["objects"],"diagnostics":t.reply["diagnostics"]});
        for r in &mut t.records[start..] {
            r["data"]["invocation"] = v["id"].clone();
        }
        for r in &mut t.state.records {
            if let Some(new) = t.records[start..].iter().find(|n| n["id"] == r["id"]) {
                *r = new.clone();
            }
        }
        while a
            .evidence
            .ids
            .get(a.cursor)
            .is_some_and(|id| t.state.allocated_ids.contains(id))
        {
            a.cursor += 1;
        }
    } else if v["operation"] == c.qualified("list.v1") {
        // OPS-036/ACC-033: list reads stored fields only; package visibility uses State.
        let view = &t.state;
        let container = access::object(view, &v["target"]).ok_or("list container absent")?;
        let mut objects = vec![];
        for object in &view.objects {
            let candidate = match container["kind"].as_str() {
                Some("Pool") => {
                    object["kind"] == "Resource"
                        && object.get("spec").is_none()
                        && access::id(&object["owner"]) == &container["id"]
                }
                Some("Fleet") => {
                    object["kind"] == "Agent"
                        && object.get("spec").is_none()
                        && access::id(&object["owner"]) == &container["id"]
                }
                _ => true,
            };
            if !candidate
                || args.get("kind").is_some_and(|k| object["kind"] != *k)
                // OPS-036 / IFC-020: the caller's inspection rights determine visibility.
                || !access::may_inspect(view, &v["caller"], object, c)?
            {
                continue;
            }
            // OPS-036 / LIF-012: contracts are neither templates nor lifecycle instances.
            let family = matches!(
                object["kind"].as_str(),
                Some("Interface" | "Agent" | "Resource" | "Pool" | "Fleet" | "Conduit")
            );
            let template = family && object.get("spec").is_some();
            let mut item = json!({"kind":object["kind"],"name":object["name"],"id":object["id"],"template":template});
            if family && !template {
                item["state"] = object["state"].clone();
            }
            objects.push(item);
        }
        let mut packages = vec![];
        if container["from"] == c.qualified("management")
            && access::may_inspect_packages(view, &v["caller"], c)?
        {
            for installed in &view.installed_packages {
                let snapshot = &installed.package;
                packages.push(json!({"address":snapshot.address,"version":snapshot.version,"digest":snapshot.digest}));
            }
        }
        let tombstones: Vec<_> = if container["from"] == c.qualified("management") {
            view.tombstones
                .values()
                .filter(|stone| {
                    args.get("kind")
                        .is_none_or(|kind| *kind == stone["object"]["kind"])
                        && crate::inventory::visible(view, &v["caller"], stone, c)
                })
                .cloned()
                .collect()
        } else {
            vec![]
        };
        outputs = json!({"objects":objects,"packages":packages,"tombstones":tombstones});
        t.decisions.push(decision(
            "list-inspectable-items",
            &["OPS-036", "ACC-033", "LIF-012", "LIF-040"],
            &[],
            json!({"kind":args.get("kind")}),
            vec![],
        ));
        if let Some(holder) = pending::own_interface(view, &v["caller"], &v["sender"]) {
            let actions = pending::raised_views(holder, view, &a.evidence.time, c)?;
            if !actions.is_empty() {
                let ids: Vec<_> = actions.iter().map(|action| &action["id"]).collect();
                t.decisions.push(decision(
                    "list-pending-actions",
                    &["PND-001", "PND-002", "PND-003", "PND-004"],
                    &["DL19"],
                    json!({"actions":ids}),
                    vec![],
                ));
                outputs["pending"] = json!(actions);
            }
        }
    } else if v["operation"] == c.qualified("inspect.v1") {
        let o = access::object(&t.state, &args["object"])
            .ok_or("inspection object absent")?
            .clone();
        t.state.invocations[i]["object"] = o["id"].clone();
        let mut declared = o.clone();
        for field in [
            "held",
            "schema_environment",
            "contract_scope",
            "resolved_implementations",
            "state",
            "status",
            "past_conditions",
            "shutdown",
            "first_start",
            "id",
            "revision",
            "packages",
            "setup_result",
            "nested",
            "open",
            "retirement",
            "retirement_tombstone",
        ] {
            declared.as_object_mut().unwrap().remove(field);
        }
        for field in ["owner", "placement", "runtime", "consumer", "target"] {
            if let Some(reference) = declared.get_mut(field)
                && reference.is_string()
                && let Some(object) = access::object(&t.state, reference)
            {
                *reference = json!({"kind":object["kind"],"id":object["id"]});
            }
        }
        if o["kind"] == "Conduit" && o["open"] == true {
            let target = access::object(&t.state, &o["target"]).ok_or("Conduit target absent")?;
            for r in declared["requirements"]
                .as_object_mut()
                .into_iter()
                .flat_map(|m| m.values_mut())
            {
                // First build accepts bound.v1 (engine) and selected implementation filters.
                let key = r["requirement"].as_str().unwrap_or("").to_owned();
                r["mode"] = json!("mediated");
                if key.starts_with(&format!("{}#", c.core_package)) {
                    r["implementation"] = json!(c.qualified("engine"));
                } else {
                    let target = registration::publication_object(target, c)?;
                    let mut implementations = serde_json::Map::new();
                    for op in o["operations"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_str)
                    {
                        let selection = &target["resolved_implementations"][op]["selection"];
                        if !selection.is_string() {
                            return Err("filter implementation has no qualified reference".into());
                        }
                        implementations.insert(op.into(), selection.clone());
                    }
                    r["implementations"] = Value::Object(implementations);
                }
            }
        }
        let mut status = o.get("status").cloned().unwrap_or(json!({}));
        if let Some(state) = o.get("state") {
            status["state"] = state.clone();
        }
        if let Some(shutdown) = o.get("shutdown") {
            status["shutdown"] = shutdown.clone();
        }
        if let Some(result) = o.get("setup_result") {
            status["setup"] = result.clone();
        }
        status["conditions"] = status.get("conditions").cloned().unwrap_or(json!([]));
        status["unreconciled"] = json!(
            t.state
                .invocations
                .iter()
                .filter(|v| v["state"] == "unknown" && invocation_names(v, &o["id"]))
                .map(|v| &v["id"])
                .collect::<Vec<_>>()
        );
        if o.get("spec").is_none()
            && let Some(holder) = pending::own_interface(&t.state, &v["caller"], &v["sender"])
        {
            let actions: Vec<_> = pending::raised_views(holder, &t.state, &a.evidence.time, c)?
                .into_iter()
                .filter(|action| {
                    action["subject"].as_array().is_some_and(|subjects| {
                        subjects.iter().any(|subject| subject["id"] == o["id"])
                    })
                })
                .collect();
            for action in &actions {
                let (name, promises): (&str, &[&str]) = if action["kind"] == "notice" {
                    ("show-notice", &["PND-001", "PND-002", "PND-004", "PND-007"])
                } else if action["kind"] == "sign-in" {
                    (
                        "show-sign-in-action",
                        &["PND-001", "PND-002", "PND-004", "PND-005"],
                    )
                } else {
                    (
                        "raise-setup-action",
                        &["PND-001", "PND-002", "PND-003", "PND-004", "PND-006"],
                    )
                };
                t.decisions.push(decision(
                    name,
                    promises,
                    &["DL19"],
                    json!({"action":action["id"]}),
                    vec![],
                ));
            }
            if !actions.is_empty() {
                status["pending"] = json!(actions);
            }
        }
        let holder = pending::own_interface(&t.state, &v["caller"], &v["sender"]);
        let records: Vec<_> = t
            .state
            .records
            .iter()
            .filter(|r| {
                record_names(r, &o["id"])
                    || t.state.invocations.iter().any(|v| {
                        invocation_names(v, &o["id"])
                            && (r["data"]["invocation"] == v["id"]
                                || r["data"]["serving"] == v["id"])
                    })
            })
            .map(|r| {
                let mut record = r.clone();
                if r["type"] == "pending-action-answered"
                    && r["data"]["answer"]["kind"] == "sign-in"
                {
                    record["data"]["answer"]["evidence"] =
                        pending::answer_evidence(&r["data"]["answer"], holder, &t.state, c)?;
                }
                Ok(record)
            })
            .collect::<Result<_>>()?;
        outputs = json!({"id":o["id"],"revision":o["revision"],"packages":o.get("packages").cloned().unwrap_or_else(|| json!([{ "address":c.core_package,"version":c.builtin.as_ref().and_then(|b| b.manifest.as_ref()).map(|m|m["version"].clone()).unwrap_or(Value::Null)}])),"declared":declared,"status":status,"records":records,"recordGaps":t.state.record_gaps,"recordsComplete":t.state.record_gaps.is_empty()});
        if o.get("spec").is_none() && o["kind"] != "Conduit" {
            outputs["inventory"] = crate::inventory::read(&t.state, &o, c)?;
        }
        if let Some(provenance) = o.get("provenance") {
            outputs["provenance"] = provenance.clone();
        }
        // LIF-012: templates have no lifecycle state. inspect.v1 permits status
        // to be absent, but requires state whenever it is present.
        if o.get("spec").is_some() {
            outputs.as_object_mut().unwrap().remove("status");
        }
        t.state.invocations[i]["object"] = o["id"].clone();
        t.decisions.push(decision(
            "inspect-object",
            &[
                "LIF-012", "LIF-013", "REC-002", "REC-005", "REC-006", "OBJ-009", "NAM-009",
                "CON-022", "ACC-044",
            ],
            &["DL8"],
            json!({}),
            vec![],
        ));
    } else if v["operation"] == c.qualified("purge.v1") {
        let before =
            chrono::DateTime::parse_from_rfc3339(args["before"].as_str().ok_or("before absent")?)
                .map_err(|e| e.to_string())?;
        let mut removed = vec![];
        for r in &t.state.records {
            let time = chrono::DateTime::parse_from_rfc3339(
                r["time"].as_str().ok_or("record time absent")?,
            )
            .map_err(|e| e.to_string())?;
            if time < before && r["data"]["action"] != "records-purged" {
                removed.push(json!({"id":r["id"],"source":r["source"]}));
            }
        }
        t.state.records.retain(|r| {
            !removed
                .iter()
                .any(|p| p["id"] == r["id"] && p["source"] == r["source"])
        });
        if !removed.is_empty() {
            t.state
                .record_gaps
                .push(json!({"cause":"purged","before":args["before"],"records":removed}));
        }
        requests::record(
            t,
            a,
            c,
            "object-change",
            json!({"target":v["target"],"invocation":v["id"],"action":"records-purged","before":args["before"],"purged":removed}),
        )?;
        outputs = json!({});
        t.decisions.push(decision(
            "purge-records-only",
            &["REC-003", "REC-004", "REC-006", "OPS-008", "ACC-032"],
            &["DL8"],
            json!({"purged":removed.len()}),
            vec![],
        ));
    } else {
        let old = template(&t.state, args)
            .ok_or("replacement template absent")?
            .clone();
        let mut trial = replacement(&t.state, args, c, a)?;
        let mut new = trial
            .state
            .objects
            .iter()
            .find(|o| o["kind"] == old["kind"] && o["name"] == old["name"])
            .ok_or("staged template absent")?
            .clone();
        new["id"] = old["id"].clone();
        new["revision"] = old["revision"].clone();
        let changed = new["spec"] != old["spec"]
            || new["metadata"] != old["metadata"]
            || new["packages"] != old["packages"];
        if changed {
            let revision = a
                .evidence
                .revisions
                .first()
                .ok_or("replacement needs new revision evidence")?;
            if *revision == old["revision"] {
                return Err("replacement revision did not advance".into());
            }
            new["revision"] = json!(revision);
            *t.state
                .objects
                .iter_mut()
                .find(|o| o["id"] == old["id"])
                .unwrap() = new.clone();
            requests::record(
                t,
                a,
                c,
                "object-change",
                json!({"target":old["id"],"invocation":v["id"],"action":"template-replaced","previousRevision":old["revision"],"revision":new["revision"]}),
            )?;
        }
        t.decisions.extend(trial.decisions);
        packages::format_submission_notes(&mut trial.reply["diagnostics"]);
        outputs = json!({"revision":new["revision"],"diagnostics":trial.reply["diagnostics"]});
        t.decisions.push(decision(
            "replace-one-template",
            &[
                "SUB-008", "SUB-009", "NAM-009", "NAM-022", "INS-020", "REV-002", "REV-004",
            ],
            &[],
            json!({"changed":changed}),
            vec![],
        ));
    }
    requests::finish(t, a, c, i, "succeeded", outputs, json!([]), None)
}

/// The project boundary predates Invoke. Complete its request accounting without changing
/// the allocations of the registered objects. New occurrence IDs use remaining evidence.
pub(crate) fn registration_boundary(
    t: &mut Transition,
    evidence: &crate::Evidence,
    request: &crate::Request,
    arguments: Value,
    c: &Context,
) -> Result<()> {
    if t.reply["accepted"] != true {
        return Ok(());
    }
    let published = t.records.clone();
    let first = t.state.records.len() - published.len();
    t.state.records.truncate(first);
    t.records.clear();
    let cursor = evidence
        .ids
        .iter()
        .position(|id| !t.state.allocated_ids.contains(id))
        .ok_or("registration accounting needs five more IDs")?;
    let mut a = Allocation { evidence, cursor };
    let id = a.next(&mut t.state)?;
    let target = registration::publication_object(
        access::object(&t.state, &json!(request.target)).ok_or("registration target absent")?,
        c,
    )?;
    let (_, _, contract) = crate::schemas::operation(&target, &request.operation, &c.core_package)?;
    let outputs = json!({"objects":t.reply["objects"],"diagnostics":t.reply["diagnostics"]});
    let digest = packages::bytes_digest(
        &serde_json_canonicalizer::to_vec(
            &json!({"operation":request.operation,"arguments":arguments}),
        )
        .map_err(|e| e.to_string())?,
    );
    t.state.invocations.push(json!({"id":id,"boundary":"register-project","caller":request.caller,"sender":request.sender,"conduit":request.conduit,"target":request.target,"operation":request.operation,"arguments":arguments,"digest":digest,"state":"succeeded","attempt":1,"plain":false,"needs_result":true,"contract":contract,"held_target":target,"retained":{"outputs":outputs,"content":[]}}));
    let route = json!({"caller":request.caller,"consumer":request.sender,"conduit":request.conduit,"target":request.target,"operation":request.operation,"invocation":id});
    let mut access = route.clone();
    access["permitted"] = json!(true);
    requests::record(t, &mut a, c, "access-decision", access)?;
    requests::record(t, &mut a, c, "invocation-dispatched", route.clone())?;
    for mut r in published {
        r["data"]["invocation"] = json!(id);
        t.state.records.push(r.clone());
        t.records.push(r);
    }
    let mut ended = route;
    ended["outcome"] = json!("succeeded");
    requests::record(t, &mut a, c, "invocation-ended", ended.clone())?;
    requests::record(t, &mut a, c, "request-ended", ended)?;
    t.decisions.push(decision(
        "record-registration-request",
        &["REC-001", "REC-002", "REC-003", "OPS-008", "ACC-002"],
        &["DL2", "DL8"],
        json!({"invocation":id}),
        vec![],
    ));
    Ok(())
}

#[cfg(test)]
mod answer_tests {
    use super::*;
    use crate::{
        Evidence, Input,
        access::{AccessRequest, Identity},
        explain::tests::fixture,
    };

    const SETUP: &str = "00000000-0000-4000-8000-000000001000";
    const ENDED: &str = "2026-11-09T12:00:00Z";
    const NOW: &str = "2026-11-10T12:00:00Z";

    fn setup_fixture() -> (State, Context) {
        let (mut s, mut c) = fixture();
        let names = [
            "answer.v1",
            "inspect.v1",
            "purge.v1",
            "retire.v1",
            "retire-with-resources.v1",
            "set-up.v1",
        ];
        let operations: Vec<_> = names.iter().map(|name| c.qualified(name)).collect();
        let capability = c.qualified("capability.v1");
        let contracts = &mut c.core_environment.scopes.get_mut("core").unwrap().contracts;
        for name in names {
            let mut spec = json!({"inputSchema":{"type":"object"}});
            if name == "inspect.v1" {
                spec["outputSchema"] = json!({"type":"object"});
            }
            contracts.insert(
                name.into(),
                json!({"kind":"Operation","metadata":{"name":name},"spec":spec}),
            );
        }
        contracts.get_mut("type.v1").unwrap()["spec"]["requires"] = json!(operations);
        contracts.insert(
            "capability.v1".into(),
            json!({"kind":"Capability","spec":{"operations":operations}}),
        );
        s.objects.retain(|o| o["id"] != "root-management");
        s.objects.extend([
            json!({"kind":"Fleet","name":"fleet","id":"fleet","state":"ready","type":c.qualified("type.v1"),"config":{}}),
            json!({"kind":"Conduit","name":"group","local_name":"group","id":"group","consumer":"root","target":"fleet","open":true,"state":"ready","capability":capability,"operations":operations}),
            json!({"kind":"Conduit","name":"own","local_name":"own","id":"own","consumer":"agent","target":"fleet","open":true,"state":"ready","capability":capability,"operations":[c.qualified("inspect.v1")]}),
            json!({"kind":"Conduit","name":"management","local_name":"management","id":"root-management","consumer":"root","target":"management","open":true,"state":"ready","capability":capability,"operations":operations}),
        ]);
        for o in &mut s.objects {
            o.as_object_mut().unwrap().remove("schema_environment");
            if o["id"] == "agent" {
                o["owner"] = json!("fleet");
            }
        }
        s.invocations.push(
            json!({"id":SETUP,"operation":c.qualified("set-up.v1"),"object":"agent",
            "state":"failed","dispatched":1,"ended":{"time":ENDED,"outcome":"failed"}}),
        );
        (s, c)
    }

    fn evidence(step: u32, time: &str) -> Evidence {
        Evidence {
            time: time.into(),
            ids: (0..20)
                .map(|i| format!("00000000-0000-4000-8000-{step:08}{i:04}"))
                .collect(),
            revisions: vec![],
            suffixes: vec![],
        }
    }

    fn invoke(
        state: &State,
        c: &Context,
        step: u32,
        time: &str,
        route: (&str, &str),
        args: Value,
        key: Option<&str>,
    ) -> Transition {
        crate::step(
            state,
            Input::Invoke {
                evidence: evidence(step, time),
                request: AccessRequest {
                    identity: Identity::Instance { id: route.0.into() },
                    tool: route.1.into(),
                    arguments: args,
                    serving: None,
                    readable: true,
                    authority_readable: true,
                    client_capabilities: None,
                    sign_in_login: None,
                    sign_in_return: None,
                },
                key: key.map(str::to_owned),
                archive: None,
            },
            c,
        )
        .unwrap()
    }

    fn answer_record(records: &Value) -> &Value {
        records
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["type"] == "pending-action-answered")
            .unwrap()
    }

    #[test]
    fn register_and_replace_results_report_unused_packages() {
        let (mut state, mut c) = fixture();
        let operations = [c.qualified("register.v1"), c.qualified("replace.v1")];
        let capability = c.qualified("object-management.v1");
        let contracts = &mut c.core_environment.scopes.get_mut("core").unwrap().contracts;
        for name in ["register.v1", "replace.v1"] {
            contracts.insert(
                name.into(),
                json!({"kind":"Operation","metadata":{"name":name},"spec":{
                    "inputSchema":{"type":"object"},"outputSchema":{"type":"object"},
                    "deduplicates":true
                }}),
            );
        }
        for name in [
            "runtime-provider.v1",
            "machine-provider.v1",
            "package-source.v1",
        ] {
            contracts.insert(
                name.into(),
                json!({"kind":"Capability","metadata":{"name":name},"spec":{"operations":[]}}),
            );
        }
        contracts.insert(
            "qualified-reference.v1".into(),
            json!({"kind":"Schema","metadata":{"name":"qualified-reference.v1"},
                "spec":{"type":"string"}}),
        );
        contracts.insert(
            "object-management.v1".into(),
            json!({"kind":"Capability","metadata":{"name":"object-management.v1"},
                "spec":{"operations":operations}}),
        );
        c.builtin = Some(packages::Project {
            manifest: Some(json!({"address":c.core_package,"version":"1.0.0",
                "exports":["qualified-reference.v1"],"implementations":{"engine":{"implements":[]}}})),
            documents: contracts.values().cloned().collect(),
            directory: None,
        });
        for object in &mut state.objects {
            if object["id"] == "management" {
                object["schema_environment"] = json!(c.core_environment);
                for operation in &operations {
                    object["resolved_implementations"][operation] = json!({"source":"engine"});
                }
            } else if object["id"] == "root-management" {
                object["operations"] = json!(operations);
                object["capability"] = json!(capability);
            }
        }

        let types = "example.org/types";
        let schemas = "example.org/schemas";
        let mut manifest = json!({"address":"example.org/submission","packages":{}});
        for (address, name, document) in [
            (
                types,
                "type.v1",
                json!({"kind":"Type","metadata":{"name":"type.v1"},
                "spec":{"family":"Resource","schema":{"type":"object"}}}),
            ),
            (
                schemas,
                "config.v1",
                json!({"kind":"Schema","metadata":{"name":"config.v1"},
                "spec":{"type":"object"}}),
            ),
        ] {
            let project = packages::Project {
                manifest: Some(json!({"address":address,"version":"1.0.0","exports":[name]})),
                documents: vec![document],
                directory: None,
            };
            let digest = packages::project_digest(&project).unwrap();
            manifest["packages"][address] = json!({"version":"1.0.0","digest":digest});
            state.preinstalled_snapshots.push(packages::Snapshot {
                source: packages::SnapshotSource::PreinstalledSnapshot,
                address: address.into(),
                version: "1.0.0".into(),
                digest,
                project,
            });
        }
        let template = json!({"kind":"Resource","metadata":{"name":"shared"},
            "spec":{"type":format!("{types}#type.v1"),"config":{}}});
        let schema = json!({"kind":"Schema","metadata":{"name":"config.v1"},
            "spec":{"$ref":format!("{schemas}#config.v1")}});
        let note = json!([{"ruleId":"NAM-008","level":"note",
            "message":{"text":format!("Unused package {schemas}; submission accepted.")}}]);
        let invoke = |state: &State, step, tool: &str, arguments, key: &str| {
            let mut evidence = evidence(step, NOW);
            evidence.revisions = vec!["r1".into(), "r2".into()];
            crate::step(
                state,
                Input::Invoke {
                    evidence,
                    request: AccessRequest {
                        identity: Identity::Instance { id: "root".into() },
                        tool: tool.into(),
                        arguments,
                        serving: None,
                        readable: true,
                        authority_readable: true,
                        client_capabilities: None,
                        sign_in_login: None,
                        sign_in_return: None,
                    },
                    key: Some(key.into()),
                    archive: None,
                },
                &c,
            )
            .unwrap_or_else(|error| panic!("{tool} at step {step}: {error}"))
        };
        let mut diagnostics = Vec::new();
        for use_both in [false, true] {
            let contracts = if use_both {
                vec![schema.clone()]
            } else {
                vec![]
            };
            let mut documents = contracts.clone();
            documents.push(template.clone());
            let args = json!({"manifest":manifest,"documents":documents});
            let registered = invoke(
                &state,
                1,
                "management.register.v1",
                args.clone(),
                "register",
            );
            assert_eq!(
                registered.reply["_meta"]["test/outcome"], "succeeded",
                "{}",
                registered.reply
            );
            let repeated = invoke(
                &registered.state,
                2,
                "management.register.v1",
                args,
                "register",
            );
            assert_eq!(
                repeated.reply["structuredContent"],
                registered.reply["structuredContent"]
            );
            let revision = &registered
                .state
                .objects
                .iter()
                .find(|o| o["name"] == "shared")
                .unwrap()["revision"];
            let args = json!({"manifest":manifest,"contracts":contracts,"template":template,"revision":revision});
            let replaced = invoke(
                &registered.state,
                3,
                "management.replace.v1",
                args.clone(),
                "replace",
            );
            assert_eq!(
                replaced.reply["_meta"]["test/outcome"], "succeeded",
                "{}",
                replaced.reply
            );
            let repeated = invoke(&replaced.state, 4, "management.replace.v1", args, "replace");
            assert_eq!(
                repeated.reply["structuredContent"],
                replaced.reply["structuredContent"]
            );
            diagnostics.push((
                registered.reply["structuredContent"]["diagnostics"].clone(),
                replaced.reply["structuredContent"]["diagnostics"].clone(),
            ));
        }
        assert_eq!(diagnostics, [(note.clone(), note), (json!([]), json!([]))]);
    }

    fn notice_fixture() -> (State, Context) {
        let (mut state, c) = setup_fixture();
        let capability = c.qualified("capability.v1");
        let operations = [
            c.qualified("inspect.v1"),
            c.qualified("retire.v1"),
            c.qualified("retire-with-resources.v1"),
        ];
        state.objects.extend([
            json!({"id":"shared","kind":"Resource","name":"shared","state":"ready","owner":"owners",
                "type":c.qualified("type.v1"),"config":{},"placement":"node",
                "resolved_implementations":{"example.org/fixture#run.v1":{"item":{"command":["test"]}}}}),
            json!({"id":"node","kind":"Resource","name":"node","state":"ready","status":{}}),
            json!({"id":"to-shared","kind":"Conduit","name":"to-shared","state":"ready",
                "open":true,"consumer":"agent","target":"shared"}),
            json!({"id":"other-pool","kind":"Conduit","name":"other-pool","local_name":"pool",
                "state":"ready","open":true,"consumer":"other","target":"owners",
                "capability":capability,"operations":operations}),
            json!({"id":"root-pool","kind":"Conduit","name":"root-pool","local_name":"pool",
                "state":"ready","open":true,"consumer":"root","target":"owners",
                "capability":capability,"operations":operations}),
        ]);
        state.invocations = vec![json!({
            "id":SETUP,"operation":c.qualified("set-up.v1"),"object":"shared","state":"succeeded"
        })];
        (state, c)
    }

    #[test]
    fn keyed_retirement_retries_keep_their_notices() {
        for (tool, arguments) in [
            (
                "pool.retire.v1",
                json!({"instance":{"kind":"Resource","name":"shared"}}),
            ),
            ("pool.retire-with-resources.v1", json!({})),
        ] {
            let (state, c) = notice_fixture();
            let retire = |state, step| {
                invoke(
                    state,
                    &c,
                    step,
                    NOW,
                    ("other", tool),
                    arguments.clone(),
                    Some("retirement-key"),
                )
            };
            let stop = |pending: &Transition, report_step: u32, outcome: &str| {
                assert_eq!(pending.reply["pending"], true, "{}", pending.reply);
                assert_eq!(pending.external_intents.len(), 1);
                assert_eq!(
                    pending.external_intents[0]["operation"],
                    c.qualified("stop.v1")
                );
                crate::step(
                    &pending.state,
                    Input::SetupReport {
                        evidence: evidence(report_step, NOW),
                        invocation: pending.external_intents[0]["invocation"]
                            .as_str()
                            .unwrap()
                            .into(),
                        report: crate::lifecycle::Report::Stop {
                            outcome: outcome.into(),
                        },
                    },
                    &c,
                )
                .unwrap()
            };

            let first = retire(&state, 1);
            let failed = stop(&first, 2, "failed");
            assert_eq!(failed.reply["_meta"]["test/outcome"], "failed");
            let notice = failed.state.pending_actions.keys().next().unwrap();
            let notices = json!([notice]);
            assert_eq!(failed.reply["_meta"]["test/pendingAction"], notices);

            let retry = retire(&failed.state, 3);
            assert_eq!(retry.reply["invocation"], first.reply["invocation"]);
            let ended = stop(&retry, 4, "succeeded");
            assert_eq!(ended.reply["_meta"]["test/outcome"], "succeeded");
            assert_eq!(
                ended.reply["_meta"]["test/invocation"],
                first.reply["invocation"]
            );
            assert_eq!(ended.reply["_meta"]["test/pendingAction"], notices);
            assert_eq!(ended.state.pending_actions, failed.state.pending_actions);
            assert!(!retry.decisions.iter().any(|d| d.action == "raise-notice"));
        }
    }

    #[test]
    fn inspection_returns_the_stored_notice_answer() {
        let (state, c) = notice_fixture();
        let retired = invoke(
            &state,
            &c,
            1,
            NOW,
            ("other", "pool.retire.v1"),
            json!({"instance":{"kind":"Resource","name":"shared"}}),
            None,
        );
        assert_eq!(retired.reply["pending"], true, "{}", retired.reply);
        let notice = retired.state.pending_actions.keys().next().unwrap();
        let inspect = |state, step, caller| {
            invoke(
                state,
                &c,
                step,
                NOW,
                (caller, "pool.inspect.v1"),
                json!({"object":{"kind":"Resource","name":"shared"}}),
                None,
            )
        };
        let live = inspect(&retired.state, 2, "other");
        let pool_view = &live.reply["structuredContent"]["status"]["pending"][0];
        assert_eq!(
            pool_view["subject"],
            json!([{"kind":"Resource","name":"shared","id":"shared"}])
        );
        assert_eq!(pool_view["evidence"].as_array().unwrap().len(), 5);

        let answered = invoke(
            &live.state,
            &c,
            3,
            NOW,
            ("root", "group.answer.v1"),
            json!({"action":notice,"answer":"acknowledged"}),
            None,
        );
        assert_eq!(answered.reply["_meta"]["test/outcome"], "succeeded");
        let stored_records = json!(answered.state.records);
        let stored = answer_record(&stored_records);
        let full = &stored["data"]["answer"];
        assert_eq!(full["subject"][1]["id"], "to-shared");
        assert_eq!(full["evidence"][5]["value"]["consumer"], "agent");

        // LIF-013 and REC-005 return the records that name the instance as stored; PND-004's
        // disclosure of this answer's evidence to a Pool-only Interface waits for a later order.
        let inspection = inspect(&answered.state, 4, "other");
        let inspected = answer_record(&inspection.reply["structuredContent"]["records"]);
        assert_eq!(inspected, stored);
        assert_eq!(answer_record(&json!(inspection.state.records)), stored);
        let root = inspect(&inspection.state, 5, "root");
        assert_eq!(
            answer_record(&root.reply["structuredContent"]["records"]),
            stored
        );
    }

    #[test]
    fn answer_records_name_the_subject() {
        let (s, c) = setup_fixture();
        let dismiss = invoke(
            &s,
            &c,
            1,
            NOW,
            ("root", "group.answer.v1"),
            json!({"action":SETUP,"answer":"dismissed"}),
            None,
        );
        let inspect = |state, step, route| {
            invoke(
                state,
                &c,
                step,
                NOW,
                route,
                json!({"object":{"kind":"Agent","name":"agent"}}),
                None,
            )
        };
        let root = inspect(&dismiss.state, 2, ("root", "group.inspect.v1"));
        let answer = &answer_record(&root.reply["structuredContent"]["records"])["data"]["answer"];
        assert_eq!(answer["answer"]["value"], "dismissed");
        assert!(!answer["evidence"].as_array().unwrap().is_empty());

        let retry = invoke(
            &s,
            &c,
            3,
            NOW,
            ("root", "group.set-up.v1"),
            json!({"instance":{"kind":"Agent","name":"agent"}}),
            None,
        );
        let agent = inspect(&retry.state, 4, ("agent", "own.inspect.v1"));
        let answer = &answer_record(&agent.reply["structuredContent"]["records"])["data"]["answer"];
        assert_eq!(answer["answer"]["value"], "retried");
        // Inspection returns the stored record whole; PND-004's withholding is a later order's.
        assert_eq!(
            answer_record(&agent.reply["structuredContent"]["records"]),
            answer_record(&json!(retry.state.records))
        );
    }

    #[test]
    fn inspection_withholds_sign_in_answer_evidence_from_agents() {
        let (mut s, c) = setup_fixture();
        let evidence = json!([{
            "kind":"fact","value":"primary-account","where":"config/account",
            "proves":"the account this instance is configured for"
        }]);
        s.records.push(json!({
            "type":"pending-action-answered","subject":"agent",
            "data":{"answer":{"kind":"sign-in","subject":[{"id":"agent"}],
                "evidence":evidence,"answer":{"value":"void"}}}
        }));
        let mut scope = access::object_by_id(&s, &json!("group")).unwrap().clone();
        scope["id"] = json!("other-group");
        scope["consumer"] = json!("other");
        s.objects.push(scope);

        for (step, route, expected) in [
            (1, ("root", "group.inspect.v1"), evidence.clone()),
            (2, ("other", "group.inspect.v1"), evidence.clone()),
            (3, ("agent", "own.inspect.v1"), json!([])),
        ] {
            let inspected = invoke(
                &s,
                &c,
                step,
                NOW,
                route,
                json!({"object":{"kind":"Agent","name":"agent"}}),
                None,
            );
            let answer =
                &answer_record(&inspected.reply["structuredContent"]["records"])["data"]["answer"];
            assert_eq!(answer["answer"]["value"], "void");
            assert_eq!(answer["evidence"], expected);
            assert_eq!(
                inspected.state.records[0]["data"]["answer"]["evidence"],
                evidence
            );
        }
    }

    #[test]
    fn unsupported_keyed_answer_is_invalid_and_matching_repeat_succeeds() {
        let (s, c) = setup_fixture();
        let answer = |state, step, value| {
            invoke(
                state,
                &c,
                step,
                NOW,
                ("root", "group.answer.v1"),
                json!({"action":SETUP,"answer":value}),
                Some("answer-key"),
            )
        };
        let first = answer(&s, 1, "dismissed");
        let invalid = answer(&first.state, 2, "retried");
        assert_eq!(invalid.reply["_meta"]["test/reason"], "invalid");
        assert_eq!(
            invalid.reply["structuredContent"]["diagnostics"][0]["ruleId"],
            "PND-009"
        );
        let repeat = answer(&first.state, 3, "dismissed");
        assert_eq!(first.reply["_meta"]["test/outcome"], "succeeded");
        assert_eq!(repeat.reply, first.reply);
        assert!(
            repeat
                .records
                .iter()
                .all(|r| r["type"] != "pending-action-answered")
        );
    }
}
