//! Access only: a permit is not an invocation or a dispatch. The next stage must consume
//! the permit atomically with this state. Identity and state availability are boundary facts.
use crate::{Allocation, Context, Evidence, Result, State, Transition, decision, schemas};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Identity {
    None,
    Instance { id: String },
    Unreadable,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessRequest {
    pub identity: Identity,
    pub tool: String,
    pub arguments: Value,
    #[serde(default)]
    pub serving: Option<String>,
    #[serde(default = "yes")]
    pub readable: bool,
    #[serde(default = "yes")]
    pub authority_readable: bool,
}
fn yes() -> bool {
    true
}
pub(crate) fn id(value: &Value) -> &Value {
    value.get("id").unwrap_or(value)
}
pub(crate) fn object<'a>(state: &'a State, reference: &Value) -> Option<&'a Value> {
    // POOL-014 names the current entry Resource, never the Pool that owns it.
    if let Some(entry) = reference.get("entry") {
        let pool = object(state, &json!({"kind":"Pool","name":reference["name"]}))?;
        return state.objects.iter().find(|o| {
            o["kind"] == "Resource"
                && o["state"] != "retired"
                && o["nested"]["parent"] == pool["id"]
                && o["nested"]["slot"] == "entries"
                && o["nested"]["name"] == *entry
        });
    }
    state.objects.iter().find(|o| {
        o["id"] == *id(reference)
            || reference.as_str().is_some_and(|s| o["name"] == s)
            || reference.get("name").is_some_and(|n| o["name"] == *n)
                && reference.get("kind").is_none_or(|k| o["kind"] == *k)
    })
}
// OBJ-006: a bare name means the kind fixed by the governing reference schema.
fn fixed_object<'a>(state: &'a State, reference: &Value, kind: &str) -> Option<&'a Value> {
    if let Some(name) = reference.as_str() {
        object(state, &json!({"kind":kind,"name":name}))
    } else {
        object(state, reference)
    }
}
fn open(path: &Value) -> bool {
    path["kind"] == "Conduit" && path["open"] == true && path["state"] == "ready"
}
fn values<'a>(v: &'a Value, key: &str) -> impl Iterator<Item = &'a Value> {
    v.get(key).and_then(Value::as_array).into_iter().flatten()
}
fn requirements(path: &Value) -> impl Iterator<Item = &Value> {
    path["requirements"]
        .as_object()
        .into_iter()
        .flat_map(|m| m.values())
}
fn bound<'a>(path: &'a Value, context: &Context) -> Option<&'a Value> {
    requirements(path).find(|r| r["requirement"] == context.qualified("bound.v1"))
}
fn covers(b: &Value, target: &Value) -> bool {
    values(&b["arguments"], "targets").any(|v| id(v) == &target["id"])
        || values(&b["arguments"], "pools").any(|v| {
            target["kind"] == "Pool" && id(v) == &target["id"]
                || target["kind"] == "Resource" && id(v) == id(&target["owner"])
        })
}
fn subset(new: &Value, old: &Value, state: &State) -> bool {
    values(&new["arguments"], "pools")
        .all(|p| values(&old["arguments"], "pools").any(|v| id(v) == id(p)))
        && values(&new["arguments"], "targets")
            .all(|v| object(state, v).is_some_and(|o| covers(old, o)))
        && values(&new["arguments"], "operations")
            .all(|op| values(&old["arguments"], "operations").any(|v| v == op))
}

/// Normalize only a private publication view; registration's stored references stay frozen.
pub(crate) fn publication_state(state: &State, context: &Context) -> Result<State> {
    let mut view = state.clone();
    for o in &mut view.objects {
        if o.get("spec").is_some() {
            continue;
        }
        if o["kind"] == "Conduit" {
            for key in ["consumer", "target"] {
                o[key] = id(&o[key]).clone();
            }
            o["local_name"] = o
                .get("local_name")
                .cloned()
                .or_else(|| o["nested"].get("name").cloned())
                .unwrap_or_else(|| o["name"].clone());
        } else {
            *o = crate::registration::publication_object(o, context)?;
        }
    }
    Ok(view)
}
pub(crate) fn established<'a>(
    state: &'a State,
    identity: &Identity,
) -> std::result::Result<&'a Value, (&'static str, &'static str)> {
    match identity {
        Identity::Unreadable => Err(("unavailable", "ACC-037")),
        Identity::None => Err(("refused", "IFC-032")),
        Identity::Instance { id: instance } => state
            .objects
            .iter()
            .find(|o| o["id"] == *instance && o.get("spec").is_none() && o["kind"] != "Conduit")
            .ok_or(("refused", "IFC-022")),
    }
}
fn tools(state: &State, instance: &str, context: &Context) -> Result<Vec<Value>> {
    let mut tools = vec![];
    for path in state
        .objects
        .iter()
        .filter(|o| open(o) && id(&o["consumer"]) == &json!(instance))
    {
        for op in values(path, "operations") {
            let (published, _) = schemas::publish(
                state,
                instance,
                path["id"].as_str().ok_or("missing Conduit ID")?,
                op.as_str().ok_or("missing operation name")?,
                context,
            )?;
            if let Some(tool) = published.get("tool").filter(|v| !v.is_null()) {
                tools.push(tool.clone());
            }
        }
    }
    tools.sort_by_key(|v| v["name"].as_str().unwrap_or("").to_owned());
    Ok(tools)
}
pub(crate) fn list(t: &mut Transition, identity: &Identity, context: &Context) -> Result<()> {
    match established(&t.state, identity) {
        Ok(instance) => {
            t.reply = json!({"tools":tools(&t.state,instance["id"].as_str().ok_or("missing instance ID")?,context)?, "cacheScope":"private", "capabilities":{"tools":{"listChanged":true}}});
            t.decisions.push(decision(
                "list-current-tools",
                &["IFC-012", "IFC-013", "IFC-033", "IFC-038", "CON-017"],
                &[],
                json!({}),
                vec![],
            ));
        }
        Err((reason, rule)) => {
            t.reply = json!({"reason":reason});
            t.decisions.push(decision(
                "reject-tool-list",
                &[rule, "OPS-023"],
                if rule == "ACC-037" { &["DL4"] } else { &[] },
                json!({}),
                vec![],
            ));
        }
    }
    Ok(())
}
/// The shell routes each returned notification to subscribed streams of that consumer.
/// Comparing state, rather than connections or prior calls, keeps the lists private.
pub fn tool_list_changes(before: &State, after: &State, context: &Context) -> Result<Vec<Value>> {
    let consumers: BTreeSet<_> = before
        .objects
        .iter()
        .chain(&after.objects)
        .filter(|o| o["kind"] == "Conduit")
        .filter_map(|o| id(&o["consumer"]).as_str())
        .collect();
    let mut changes = vec![];
    for consumer in consumers {
        if tools(before, consumer, context)? != tools(after, consumer, context)? {
            changes.push(json!({"consumer":consumer,"method":"notifications/tools/list_changed"}));
        }
    }
    Ok(changes)
}
pub(crate) struct Verdict {
    pub(crate) reason: Option<&'static str>,
    pub(crate) rules: Vec<&'static str>,
    pub(crate) restriction: Option<Value>,
    pub(crate) route: Value,
}
impl Verdict {
    fn reason(reason: &'static str, rule: &'static str) -> Self {
        Self {
            reason: Some(reason),
            rules: vec![rule],
            restriction: None,
            route: Value::Null,
        }
    }
    fn restriction(reason: &'static str, rule: &'static str, r: &Value) -> Self {
        let mut v = Self::reason(reason, rule);
        v.restriction = Some(json!({"requirement":r["requirement"],"arguments":r["arguments"]}));
        v
    }
}
fn management<'a>(state: &'a State, name: &str, context: &Context) -> Result<&'a Value> {
    state
        .objects
        .iter()
        .find(|o| o["from"] == context.qualified(name))
        .ok_or_else(|| format!("missing built-in {name}"))
}
fn owner<'a>(state: &'a State, resource: &Value) -> Option<&'a Value> {
    object(state, &resource["owner"])
}
fn current_path(state: &State, consumer: &Value, target: &Value, op: &str) -> bool {
    target["state"] != "retired"
        && state.objects.iter().any(|p| {
            open(p)
                && id(&p["consumer"]) == consumer
                && id(&p["target"]) == &target["id"]
                && values(p, "operations").any(|v| v == op)
        })
}
/// First matching row of ACC-033. Unknown named objects are diagnosed only after access.
fn routing<'a>(
    state: &'a State,
    path: &'a Value,
    op: &str,
    args: &Value,
    context: &Context,
) -> Result<(Option<&'a Value>, Option<&'a Value>)> {
    let mgmt = management(state, "management", context)?;
    let core_op = op.strip_prefix(&format!("{}#", context.core_package));
    let named = match core_op {
        Some("inspect.v1" | "unregister.v1") => object(state, &args["object"]),
        Some("replace.v1") => state.objects.iter().find(|o| {
            o["kind"] == args["template"]["kind"]
                && o["name"] == args["template"]["metadata"]["name"]
                && o.get("spec").is_some()
        }),
        Some("set-up.v1" | "retire.v1" | "register-replacement.v1") => {
            object(state, &args["instance"])
        }
        Some("open.v1" | "close.v1") => fixed_object(state, &args["conduit"], "Conduit"),
        Some("declare-lost.v1") => fixed_object(state, &args["node"], "Resource"),
        _ => None,
    };
    let route = match core_op {
        Some(
            "register.v1"
            | "replace.v1"
            | "unregister.v1"
            | "register-replacement.v1"
            | "install.v1"
            | "declare-lost.v1"
            | "purge.v1",
        ) => Some(mgmt),
        Some("set-up.v1" | "retire.v1") => match named {
            Some(o) if o["kind"] == "Resource" => {
                owner(state, o).filter(|p| p["state"] != "retired")
            }
            _ => Some(mgmt),
        },
        Some("inspect.v1") => match named {
            Some(o) if o["kind"] == "Resource" && o.get("spec").is_none() => owner(state, o)
                .filter(|p| p["state"] != "retired")
                .or(Some(mgmt)),
            _ => Some(mgmt),
        },
        Some("retire-with-resources.v1") => object(state, &path["target"])
            .filter(|o| o["kind"] == "Pool" && o["state"] != "retired"),
        Some("open.v1" | "close.v1") => Some(management(state, "access", context)?),
        Some("reconcile.v1") => {
            let invocation = state
                .invocations
                .iter()
                .find(|v| v["id"] == args["invocation"]);
            match invocation {
                Some(v) => match object(state, v.get("object").unwrap_or(&v["target"])) {
                    None => Some(mgmt),
                    Some(_) if v["management"] == true => object(state, &v["target"])
                        .filter(|p| p["state"] != "retired")
                        .or(Some(mgmt)),
                    Some(o) if o["kind"] == "Resource" => owner(state, o)
                        .filter(|p| p["state"] != "retired")
                        .or(Some(mgmt)),
                    _ => Some(mgmt),
                },
                None => Some(mgmt),
            }
        }
        _ => object(state, &path["target"]).filter(|o| o["state"] != "retired"),
    };
    Ok((route, named))
}

pub(crate) fn evaluate(
    state: &State,
    request: &AccessRequest,
    context: &Context,
    defer: bool,
) -> Result<Verdict> {
    if !request.readable {
        return Ok(Verdict::reason("invalid", "OPS-023"));
    }
    // ACC-037 explicitly forbids even reading a named object on this branch.
    if !request.authority_readable {
        return Ok(Verdict::reason("unavailable", "ACC-037"));
    }
    let instance = match established(state, &request.identity) {
        Ok(o) => o,
        Err((reason, rule)) => return Ok(Verdict::reason(reason, rule)),
    };
    let mut caller = instance["id"].clone();
    let mut filters = vec![];
    if let Some(serving) = &request.serving {
        let Some(invocation) = state.invocations.iter().find(|v| {
            v["id"] == *serving && v["target"] == instance["id"] && v["state"] == "in-progress"
        }) else {
            return Ok(Verdict::reason("refused", "IFC-028"));
        };
        caller = invocation["caller"].clone();
        filters.extend(values(invocation, "filters").cloned());
        let mut cursor = Some(invocation);
        let mut seen = BTreeSet::new();
        while let Some(v) = cursor {
            if !seen.insert(v["id"].to_string()) {
                return Err("cyclic serving chain".into());
            }
            if v["state"] != "in-progress" {
                return Ok(Verdict::reason("refused", "ACC-019"));
            }
            if v["hook"] != true {
                let Some(path) = state
                    .objects
                    .iter()
                    .find(|p| p["id"] == v["conduit"] && open(p))
                else {
                    return Ok(Verdict::reason("refused", "ACC-034"));
                };
                if object(state, &path["consumer"])
                    .is_some_and(|c| c["state"] == "retired" && v["retirement_of"] != c["id"])
                {
                    return Ok(Verdict::reason("refused", "ACC-039"));
                }
            }
            cursor = match v.get("serving").and_then(Value::as_str) {
                None => None,
                Some(parent) => Some(
                    state
                        .invocations
                        .iter()
                        .find(|v| v["id"] == parent)
                        .ok_or("broken serving chain")?,
                ),
            };
        }
    } else if instance["kind"] != "Agent" && instance["kind"] != "Interface" {
        return Ok(Verdict::reason("refused", "IFC-021"));
    }
    let view = publication_state(state, context)?;
    let mut selected = None;
    for path in view
        .objects
        .iter()
        .filter(|p| open(p) && p["consumer"] == instance["id"])
    {
        let local = path["local_name"].as_str().ok_or("missing local name")?;
        for op in values(path, "operations").filter_map(Value::as_str) {
            if op
                .rsplit_once('#')
                .is_some_and(|(_, name)| request.tool == format!("{local}.{name}"))
            {
                selected = Some((path, op));
            }
        }
    }
    let Some((path, op)) = selected else {
        return Ok(Verdict::reason("refused", "ACC-001"));
    };
    if op == context.qualified("get-secret.v1") {
        return Ok(Verdict::reason("refused", "SEC-007"));
    }
    let mut verdict = evaluate_path(
        &view, request, context, instance, &caller, filters, path, op, defer,
    )?;
    if verdict.route.is_null() {
        verdict.route = json!({"consumer":instance["id"],"caller":caller,"conduit":path["id"],"target":path["target"],"operation":op});
    }
    Ok(verdict)
}

#[allow(clippy::too_many_arguments)]
fn evaluate_path(
    state: &State,
    request: &AccessRequest,
    context: &Context,
    instance: &Value,
    caller: &Value,
    mut filters: Vec<Value>,
    path: &Value,
    op: &str,
    defer: bool,
) -> Result<Verdict> {
    let target = object(state, &path["target"]).ok_or("missing Conduit target")?;

    let (env, scope, contract) = match schemas::operation(target, op, &context.core_package) {
        Ok(c) => c,
        Err(e) if e.starts_with("SCH-015: unresolved") => {
            return Ok(Verdict::reason("refused", "ACC-001"));
        }
        Err(e) => return Err(e),
    };
    let (route, named) = routing(state, path, op, &request.arguments, context)?;
    if route.is_none_or(|r| r["id"] != path["target"]) {
        return Ok(Verdict::reason("refused", "ACC-033"));
    }
    let root = management(state, "root", context)?;
    if contract["spec"]["reservedToRoot"] == true
        && (instance["id"] != root["id"] || request.serving.is_some())
    {
        return Ok(Verdict::reason("refused", "ACC-032"));
    }
    if instance["state"] == "retired"
        && !request.serving.as_ref().is_some_and(|serving| {
            state
                .invocations
                .iter()
                .any(|v| v["id"] == *serving && v["retirement_of"] == instance["id"])
        })
    {
        return Ok(Verdict::reason("refused", "ACC-039"));
    }
    let registration = op == context.qualified("register.v1");
    if registration {
        // ACC-054 / SEC-008 / OPS-023: decide authority before validating any document.
        // An onward request, including one whose sender is root, is not root's own act.
        if (instance["id"] != root["id"] || request.serving.is_some())
            && values(&request.arguments, "documents")
                .any(|doc| crate::registration::root_consumed(state, doc, root))
        {
            return Ok(Verdict::reason("refused", "ACC-054"));
        }
        for doc in values(&request.arguments, "documents")
            .filter(|d| d["kind"] == "Resource" && d.get("from").is_some())
        {
            if let Some(pool) = fixed_object(state, &doc["owner"], "Pool")
                .filter(|p| p["kind"] == "Pool" && p["state"] != "retired")
                && !current_path(state, &instance["id"], pool, op)
            {
                return Ok(Verdict::reason("refused", "ACC-033"));
            }
        }
    }
    if op == context.qualified("register-replacement.v1")
        && let Some(pool) = named.filter(|p| p["kind"] == "Pool" && p["state"] != "retired")
        && !current_path(state, &instance["id"], pool, op)
    {
        return Ok(Verdict::reason("refused", "ACC-033"));
    }
    if named.is_some_and(|o| {
        o["first_start"] == true
            && (["retire.v1", "unregister.v1"]
                .iter()
                .any(|name| op == context.qualified(name))
                || op == context.qualified("declare-lost.v1") && o["name"] == "local")
    }) {
        return Ok(Verdict::reason("refused", "LIF-034"));
    }
    let opening = op == context.qualified("open.v1");
    let closing = op == context.qualified("close.v1");
    if let Some(conduit) = named.filter(|_| opening || closing) {
        if closing && id(&conduit["consumer"]) == &root["id"] {
            return Ok(Verdict::reason("refused", "ACC-024"));
        }
        if opening {
            // ACC-054 / SEC-008: an onward request is never root's own act.
            if id(&conduit["consumer"]) == &root["id"]
                && (instance["id"] != root["id"] || request.serving.is_some())
            {
                return Ok(Verdict::reason("refused", "ACC-054"));
            }
            for operation in values(conduit, "operations").filter_map(Value::as_str) {
                if operation == context.qualified("get-secret.v1")
                    && (instance["id"] != root["id"]
                        || request.serving.is_some()
                        || object(state, &conduit["consumer"])
                            .is_none_or(|c| c["kind"] != "Resource" && c["kind"] != "Agent"))
                {
                    return Ok(Verdict::reason("refused", "SEC-008"));
                }
                if context
                    .core_environment
                    .scopes
                    .values()
                    .flat_map(|s| s.contracts.values())
                    .any(|d| {
                        d["spec"]["reservedToRoot"] == true
                            && operation
                                == context.qualified(d["metadata"]["name"].as_str().unwrap_or(""))
                    })
                    && id(&conduit["consumer"]) != &root["id"]
                {
                    return Ok(Verdict::reason("refused", "ACC-032"));
                }
            }
        }
        let access_target = id(&conduit["target"]) == &management(state, "access", context)?["id"];
        let old = bound(path, context);
        let new = bound(conduit, context);
        let allowed = if access_target {
            (match (new, old) {
                (_, None) => true,
                (None, Some(_)) => false,
                (Some(new), Some(old)) => subset(new, old, state),
            }) && (!opening
                || values(conduit, "operations")
                    .all(|op| values(path, "operations").any(|v| v == op)))
        } else {
            old.is_none_or(|b| {
                object(state, &conduit["target"]).is_some_and(|t| covers(b, t))
                    && values(conduit, "operations")
                        .all(|op| values(&b["arguments"], "operations").any(|v| v == op))
            })
        };
        if !allowed {
            return Ok(match old {
                Some(b) => Verdict::restriction(
                    "refused",
                    if access_target { "ACC-030" } else { "ACC-029" },
                    b,
                ),
                None => Verdict::reason("refused", "ACC-030"),
            });
        }
    }
    let (publication, _) = schemas::publish(
        state,
        instance["id"].as_str().ok_or("missing instance ID")?,
        path["id"].as_str().ok_or("missing Conduit ID")?,
        op,
        context,
    )?;
    if publication["reason"] == "invalid" {
        return Ok(Verdict::reason("invalid", "TPL-013"));
    }
    if publication["tool"].is_null() {
        return Ok(Verdict::reason("refused", "ACC-001"));
    }
    let schema = contract["spec"]
        .get("inputSchema")
        .cloned()
        .unwrap_or(json!({"type":"object"}));
    let args = env.fill(
        &scope,
        &schema,
        &request.arguments,
        contract["spec"].get("changes").is_none(),
        &context.core_package,
    )?;
    if !schemas::standalone_valid(&publication["tool"]["inputSchema"], &args)? {
        return Ok(Verdict::reason("invalid", "SCH-001"));
    }
    if [
        "inspect.v1",
        "set-up.v1",
        "retire.v1",
        "replace.v1",
        "unregister.v1",
        "register-replacement.v1",
        "open.v1",
        "close.v1",
        "declare-lost.v1",
    ]
    .iter()
    .any(|name| op == context.qualified(name))
        && named.is_none()
    {
        return Ok(Verdict::reason("invalid", "OPS-029"));
    }
    if registration
        && values(&request.arguments, "documents").any(|d| {
            d["kind"] == "Resource"
                && d.get("from").is_some()
                && fixed_object(state, &d["owner"], "Pool").is_some_and(|p| p["state"] == "retired")
        })
    {
        return Ok(Verdict::reason("invalid", "POOL-004"));
    }
    if registration
        && values(&request.arguments, "documents").any(|d| {
            ["root", "system", "node", "management", "access"]
                .iter()
                .any(|n| d["from"] == context.qualified(n))
        })
    {
        return Ok(Verdict::reason("invalid", "FST-006"));
    }
    if !defer
        && target["state"] != "ready"
        && target["resolved_implementations"][op]["source"] != "engine"
    {
        return Ok(Verdict::reason("unavailable", "CON-024"));
    }
    if !defer
        && !schemas::capability_includes(target, &path["capability"], op, &context.core_package)?
    {
        return Ok(Verdict::reason("unavailable", "NAM-026"));
    }
    for filter in &filters {
        if !defer
            && !crate::registration::supports_filter(target, op, &filter["requirement"], context)?
        {
            return Ok(Verdict::restriction("unavailable", "ACC-015", filter));
        }
    }
    filters.extend(
        requirements(path)
            .filter(|r| {
                r["requirement"]
                    .as_str()
                    .is_some_and(|q| !q.starts_with(&format!("{}#", context.core_package)))
            })
            .cloned(),
    );
    Ok(Verdict {
        reason: None,
        rules: vec![
            "ACC-001", "ACC-002", "ACC-003", "ACC-008", "ACC-009", "ACC-016", "ACC-017", "ACC-029",
            "ACC-030", "ACC-033", "ACC-036", "IFC-012", "IFC-020", "IFC-022",
        ],
        restriction: None,
        route: json!({"consumer":instance["id"],"caller":caller,"conduit":path["id"],"target":target["id"],"operation":op,"arguments":args,"filters":filters,"restrictionMode":"mediated"}),
    })
}

pub(crate) fn decide(
    t: &mut Transition,
    evidence: &Evidence,
    request: &AccessRequest,
    context: &Context,
) -> Result<()> {
    let mut verdict = evaluate(&t.state, request, context, false)?;
    verdict.rules.extend([
        "OPS-023", "IFC-025", "REC-001", "REC-002", "REC-003", "ACC-027", "ACC-036",
    ]);
    let reason = verdict.reason;
    t.reply = match reason {
        Some(reason) => {
            let mut reply = json!({"reason":reason});
            if let Some(restriction) = &verdict.restriction {
                reply["restriction"] = restriction.clone();
            }
            if reason == "invalid" {
                reply["diagnostics"] = json!([{"ruleId":verdict.rules[0],"level":"error","message":{"text":"The request is invalid."}}]);
            }
            reply
        }
        None => json!({"permitted":true,"request":verdict.route}),
    };
    if !request.readable && established(&t.state, &request.identity).is_err() {
        t.decisions.push(decision(
            "unreadable-unidentified-request",
            &["IFC-036", "REC-001", "OPS-023"],
            &[],
            json!({}),
            vec![],
        ));
        return Ok(());
    }
    let mut data = json!({"tool":request.tool,"permitted":reason.is_none(),"dispatched":false});
    for key in ["caller", "consumer", "conduit", "target", "operation"] {
        if let Some(value) = verdict.route.get(key) {
            data[key] = value.clone();
        }
    }
    if let Some(serving) = &request.serving {
        data["serving"] = json!(serving);
    }
    if let Some(reason) = reason {
        data["reason"] = json!(reason);
    }
    if let Some(restriction) = verdict.restriction {
        data["restriction"] = restriction;
    }
    if data.get("caller").is_none()
        && let Ok(instance) = established(&t.state, &request.identity)
    {
        data["consumer"] = instance["id"].clone();
        if request.serving.is_none() {
            data["caller"] = instance["id"].clone();
        } else if let Some(parent) = request.serving.as_ref().and_then(|s| {
            t.state
                .invocations
                .iter()
                .find(|v| v["id"] == *s && v["target"] == instance["id"])
        }) {
            data["caller"] = parent["caller"].clone();
        }
    }
    if data.get("caller").is_none() {
        data["identity"] = json!(match request.identity {
            Identity::Unreadable => "unreadable",
            _ => "not-established",
        });
    }
    if let Some(arguments) = verdict.route.get("arguments") {
        data["arguments"] = arguments.clone();
    }
    let mut allocation = Allocation {
        evidence,
        cursor: 0,
    };
    for event in if reason.is_some() {
        vec!["access-decision", "request-ended"]
    } else {
        vec!["access-decision"]
    } {
        crate::requests::record(t, &mut allocation, context, event, data.clone())?;
    }
    t.decisions.push(decision(
        "decide-current-access",
        &verdict.rules,
        if reason == Some("unavailable") && verdict.rules.contains(&"ACC-037") {
            &["DL4", "DL8"]
        } else {
            &["DL8"]
        },
        json!({"reason":reason,"permitted":reason.is_none()}),
        vec![],
    ));
    Ok(())
}
