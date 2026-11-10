//! Record reads, retention and explicit repair. No I/O or automatic effects.
use crate::{
    Allocation, Context, Result, State, Transition, access, decision, packages, registration,
    requests,
};
use serde_json::{Value, json};

pub(crate) fn handles(op: &str, c: &Context) -> bool {
    ["inspect.v1", "purge.v1", "replace.v1", "register.v1"]
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

pub(crate) fn preflight(
    s: &State,
    route: &Value,
    c: &Context,
    a: &Allocation<'_>,
) -> Result<Option<(&'static str, &'static str)>> {
    let args = &route["arguments"];
    if route["operation"] == c.qualified("inspect.v1") {
        let o = access::object(s, &args["object"])
            .ok_or("inspection object absent after validation")?;
        if args.get("expectedId").is_some_and(|id| *id != o["id"]) {
            return Ok(Some(("conflict", "LIF-037")));
        }
    }
    if route["operation"] == c.qualified("replace.v1") {
        let Some(o) = template(s, args) else {
            return Ok(Some(("invalid", "OPS-029")));
        };
        // Validate the new project before comparing revisions (OPS-023).
        let trial = replacement(s, args, c, a)?;
        if trial.reply["accepted"] != true {
            return Ok(Some(("invalid", "SUB-009")));
        }
        if o["revision"] != args["revision"] {
            return Ok(Some(("conflict", "REV-002")));
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
        };
        register_project(&mut trial, args, c, a, route)?;
        if trial.reply["accepted"] != true {
            let rule = trial.reply["diagnostics"]
                .as_array()
                .and_then(|d| d.first())
                .and_then(|d| d["ruleId"].as_str())
                .unwrap_or("SUB-003");
            // Requiring rule IDs here are selected from a finite vocabulary by the resolver.
            return Ok(Some((
                if trial.reply["reason"] == "conflict" {
                    "conflict"
                } else {
                    "invalid"
                },
                match rule {
                    "OBJ-002" => "OBJ-002",
                    "FST-006" => "FST-006",
                    "SUB-009" => "SUB-009",
                    _ => "SUB-003",
                },
            )));
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
) -> Result<()> {
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
    if v["operation"] == c.qualified("register.v1") {
        let start = t.records.len();
        let route = json!({"consumer":v["sender"],"caller":v["caller"],"conduit":v["conduit"],"target":v["target"]});
        register_project(t, args, c, a, &route)?;
        if t.reply["accepted"] != true {
            return Err("registration changed between validation and dispatch".into());
        }
        let created = t.reply["objects"].clone();
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
        outputs = json!({"objects":created,"diagnostics":[]});
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
            "shutdown",
            "first_start",
            "id",
            "revision",
            "packages",
            "setup_result",
            "nested",
            "open",
            "retirement",
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
            .cloned()
            .collect();
        outputs = json!({"id":o["id"],"revision":o["revision"],"packages":o.get("packages").cloned().unwrap_or_else(|| json!([{ "address":c.core_package,"version":c.builtin.as_ref().and_then(|b| b.manifest.as_ref()).map(|m|m["version"].clone()).unwrap_or(Value::Null)}])),"declared":declared,"status":status,"records":records,"recordGaps":t.state.record_gaps,"recordsComplete":t.state.record_gaps.is_empty()});
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
                "CON-022",
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
        let trial = replacement(&t.state, args, c, a)?;
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
        outputs = json!({"revision":new["revision"],"diagnostics":[]});
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
    requests::finish(t, a, c, i, "succeeded", outputs, json!([]))
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
