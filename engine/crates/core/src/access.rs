//! Access only: a permit is not an invocation or a dispatch. The next stage must consume
//! the permit atomically with this state. Identity and state availability are boundary facts.
use crate::{Allocation, Context, Evidence, Result, State, Transition, decision, schemas};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{borrow::Cow, cell::OnceCell, collections::BTreeSet};

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
    // POOL-014: retain the raw reference lookup for helpers that take State.
    object_in(&state.objects, reference)
}
fn object_in<'a>(objects: &'a [Value], reference: &Value) -> Option<&'a Value> {
    // POOL-014 names the current entry Resource, never the Pool that owns it.
    if let Some(entry) = reference.get("entry") {
        let pool = object_in(objects, &json!({"kind":"Pool","name":reference["name"]}))?;
        return objects.iter().find(|o| {
            o["kind"] == "Resource"
                && o["state"] != "retired"
                && o["nested"]["parent"] == pool["id"]
                && o["nested"]["slot"] == "entries"
                && o["nested"]["name"] == *entry
        });
    }
    objects.iter().find(|o| {
        o["id"] == *id(reference)
            || reference.as_str().is_some_and(|s| o["name"] == s)
            || reference.get("name").is_some_and(|n| o["name"] == *n)
                && reference.get("kind").is_none_or(|k| o["kind"] == *k)
    })
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
fn subset(new: &Value, old: &Value, view: &PublicationView<'_>) -> Result<bool> {
    // ACC-029: every bound target is read through the normalized view.
    if !values(&new["arguments"], "pools")
        .all(|p| values(&old["arguments"], "pools").any(|v| id(v) == id(p)))
    {
        return Ok(false);
    }
    for v in values(&new["arguments"], "targets") {
        if !view.object(v)?.is_some_and(|o| covers(old, o)) {
            return Ok(false);
        }
    }
    Ok(values(&new["arguments"], "operations")
        .all(|op| values(&old["arguments"], "operations").any(|v| v == op)))
}

/// IFC-012/IFC-033: a call-local, objects-only publication view. Unread objects
/// are never normalized; templates borrow their unchanged stored representation.
pub struct PublicationView<'a> {
    objects: &'a [Value],
    context: &'a Context,
    normalized: Vec<OnceCell<Result<Cow<'a, Value>>>>,
}
impl<'a> PublicationView<'a> {
    pub fn new(state: &'a State, context: &'a Context) -> Self {
        // IFC-033: no records, invocations, snapshots or package files enter the view.
        Self {
            objects: &state.objects,
            context,
            normalized: (0..state.objects.len()).map(|_| OnceCell::new()).collect(),
        }
    }
    fn at(&self, index: usize) -> Result<&Value> {
        // IFC-033: normalize a looked-up object at most once, including a failed lookup.
        self.normalized[index]
            .get_or_init(|| {
                let object = &self.objects[index];
                if object.get("spec").is_some() {
                    return Ok(Cow::Borrowed(object));
                }
                if object["kind"] == "Conduit" {
                    if ["consumer", "target"]
                        .iter()
                        .all(|key| object.get(key).is_some_and(|v| id(v) == v))
                        && object.get("local_name").is_some()
                    {
                        return Ok(Cow::Borrowed(object));
                    }
                    let mut o = object.clone();
                    for key in ["consumer", "target"] {
                        o[key] = id(&o[key]).clone();
                    }
                    o["local_name"] = o
                        .get("local_name")
                        .cloned()
                        .or_else(|| o["nested"].get("name").cloned())
                        .unwrap_or_else(|| o["name"].clone());
                    Ok(Cow::Owned(o))
                } else {
                    let normalized = crate::registration::publication_object(object, self.context)?;
                    Ok(if normalized == *object {
                        Cow::Borrowed(object)
                    } else {
                        Cow::Owned(normalized)
                    })
                }
            })
            .as_ref()
            .map(|o| o.as_ref())
            .map_err(Clone::clone)
    }
    pub fn find(&self, predicate: impl Fn(&Value) -> bool) -> Result<Option<&Value>> {
        // IFC-033: raw ID/name/kind selection does not normalize unrelated instances.
        self.objects
            .iter()
            .position(predicate)
            .map(|i| self.at(i))
            .transpose()
    }
    pub fn object(&self, reference: &Value) -> Result<Option<&Value>> {
        // ACC-033: retain POOL-014's exact reference lookup before normalization.
        if let Some(entry) = reference.get("entry") {
            let Some(pool) = self.object(&json!({"kind":"Pool","name":reference["name"]}))? else {
                return Ok(None);
            };
            return self.find(|o| {
                o["kind"] == "Resource"
                    && o["state"] != "retired"
                    && o["nested"]["parent"] == pool["id"]
                    && o["nested"]["slot"] == "entries"
                    && o["nested"]["name"] == *entry
            });
        }
        let Some(raw) = object_in(self.objects, reference) else {
            return Ok(None);
        };
        self.find(|o| std::ptr::eq(o, raw))
    }
    pub fn paths<'b>(
        &'b self,
        consumer: &'b Value,
    ) -> impl Iterator<Item = Result<&'b Value>> + 'b {
        // IFC-012: preserve open Conduit order, normalizing only matching paths.
        self.objects
            .iter()
            .enumerate()
            .filter(move |(_, o)| open(o) && id(&o["consumer"]) == consumer)
            .map(|(i, _)| self.at(i))
    }
    fn fixed(&self, reference: &Value, kind: &str) -> Result<Option<&Value>> {
        // ACC-024/OBJ-006: a bare name uses the governing reference kind.
        if let Some(name) = reference.as_str() {
            self.object(&json!({"kind":kind,"name":name}))
        } else {
            self.object(reference)
        }
    }
    fn management(&self, name: &str) -> Result<&Value> {
        // ACC-033: management/root reads use the same normalized objects as publication.
        self.find(|o| o["from"] == self.context.qualified(name))?
            .ok_or_else(|| format!("missing built-in {name}"))
    }
    fn owner(&self, object: &Value) -> Result<Option<&Value>> {
        // ACC-033: resolve the named instance's owner in the publication view.
        self.object(&object["owner"])
    }
    fn current_path(&self, consumer: &Value, target: &Value, op: &str) -> Result<bool> {
        // ACC-033: preserve the current open-path scan and retired-target check.
        if target["state"] == "retired" {
            return Ok(false);
        }
        for path in self.paths(consumer) {
            let path = path?;
            if id(&path["target"]) == &target["id"] && values(path, "operations").any(|v| v == op) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
pub(crate) fn established<'a>(
    state: &'a State,
    identity: &Identity,
) -> std::result::Result<&'a Value, (&'static str, &'static str)> {
    match identity {
        Identity::Unreadable => Err(("unavailable", "ACC-037")),
        Identity::None => Err(("refused", "IFC-032")),
        // FLEET-004: no credential or presented server names a Fleet.
        Identity::Instance { id: instance } => state
            .objects
            .iter()
            .find(|o| {
                o["id"] == *instance
                    && o.get("spec").is_none()
                    && o["kind"] != "Conduit"
                    && o["kind"] != "Fleet"
            })
            .ok_or(("refused", "IFC-022")),
    }
}
pub(crate) fn tools(
    view: &PublicationView<'_>,
    instance: &str,
    context: &Context,
) -> Result<Vec<Value>> {
    // IFC-012/IFC-033: every tool in this list shares one publication view.
    let mut tools = vec![];
    visit_tools(view, instance, context, |tool| tools.push(tool.clone()))?;
    tools.sort_by_key(|v| v["name"].as_str().unwrap_or("").to_owned());
    Ok(tools)
}
fn visit_tools(
    view: &PublicationView<'_>,
    instance: &str,
    context: &Context,
    mut visit: impl FnMut(&Value),
) -> Result<()> {
    // IFC-033/IFC-038: publication errors also matter when no list must be collected.
    let consumer = json!(instance);
    for path in view.paths(&consumer) {
        let path = path?;
        for op in values(path, "operations") {
            let (published, _) = schemas::publish(
                view,
                instance,
                path["id"].as_str().ok_or("missing Conduit ID")?,
                op.as_str().ok_or("missing operation name")?,
                context,
            )?;
            if let Some(tool) = published.get("tool").filter(|v| !v.is_null()) {
                visit(tool);
            }
        }
    }
    Ok(())
}
pub(crate) fn list(t: &mut Transition, identity: &Identity, context: &Context) -> Result<()> {
    // IFC-012: publish the established consumer's private current list.
    match established(&t.state, identity) {
        Ok(instance) => {
            t.reply = json!({"tools":tools(&PublicationView::new(&t.state, context),instance["id"].as_str().ok_or("missing instance ID")?,context)?, "cacheScope":"private", "capabilities":{"tools":{"listChanged":true}}});
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
    // IFC-038: preserve the public error while retaining its consumer for differential checks.
    tool_list_changes_attributed(before, after, context).map_err(|(_, error)| error)
}
pub(crate) fn tool_list_changes_attributed(
    before: &State,
    after: &State,
    context: &Context,
) -> std::result::Result<Vec<Value>, (String, String)> {
    // IFC-038: unchanged dependencies imply equal lists, but publication can still fail.
    let consumers: BTreeSet<_> = before
        .objects
        .iter()
        .chain(&after.objects)
        .filter(|o| o["kind"] == "Conduit")
        .filter_map(|o| id(&o["consumer"]).as_str())
        .collect();
    let before_view = PublicationView::new(before, context);
    let after_view = PublicationView::new(after, context);
    let mut changes = vec![];
    for consumer in consumers {
        let old = list_dependencies(before, consumer);
        let new = list_dependencies(after, consumer);
        let changed = if old.is_some() && old == new {
            // IFC-033/IFC-038: validate once without collecting, cloning or sorting a list.
            visit_tools(&before_view, consumer, context, |_| {})
                .map_err(|error| (consumer.to_owned(), error))?;
            false
        } else {
            tools(&before_view, consumer, context)
                .and_then(|old| tools(&after_view, consumer, context).map(|new| old != new))
                .map_err(|error| (consumer.to_owned(), error))?
        };
        if changed {
            changes.push(json!({"consumer":consumer,"method":"notifications/tools/list_changed"}));
        }
    }
    Ok(changes)
}
fn list_dependencies<'a>(state: &'a State, consumer: &str) -> Option<Vec<(&'a Value, &'a Value)>> {
    // IFC-038: a missing target deliberately falls back to publication's original error.
    state
        .objects
        .iter()
        .filter(|o| open(o) && id(&o["consumer"]) == consumer)
        .map(|p| {
            state
                .objects
                .iter()
                .find(|o| o["id"] == *id(&p["target"]))
                .map(|target| (p, target))
        })
        .collect()
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
/// OPS-036: package visibility requires inspect.v1 on management itself, independently
/// of the Pool authority needed to inspect the management Resource as an object.
pub(crate) fn may_inspect_packages(
    state: &State,
    actor: &Value,
    context: &Context,
) -> Result<bool> {
    Ok(current_path(
        state,
        actor,
        management(state, "management", context)?,
        &context.qualified("inspect.v1"),
    ))
}
/// OPS-036 / ACC-033: listing never supplies inspection authority of its own.
pub(crate) fn may_inspect(
    state: &State,
    actor: &Value,
    item: &Value,
    context: &Context,
) -> Result<bool> {
    let management = management(state, "management", context)?;
    let route =
        if item.get("spec").is_none() && ["Resource", "Agent"].iter().any(|k| item["kind"] == *k) {
            owner(state, item)
                .filter(|o| o["state"] != "retired")
                .unwrap_or(management)
        } else {
            management
        };
    Ok(current_path(
        state,
        actor,
        route,
        &context.qualified("inspect.v1"),
    ))
}
/// First matching row of ACC-033. Unknown named objects are diagnosed only after access.
fn routing<'a>(
    state: &State,
    view: &'a PublicationView<'_>,
    path: &'a Value,
    op: &str,
    args: &Value,
    context: &Context,
) -> Result<(Option<&'a Value>, Option<&'a Value>)> {
    // ACC-033: only reconciliation invocation reads use the transition State.
    let mgmt = view.management("management")?;
    let core_op = op.strip_prefix(&format!("{}#", context.core_package));
    let named = match core_op {
        Some("inspect.v1" | "unregister.v1") => view.object(&args["object"])?,
        Some("replace.v1") => view.find(|o| {
            o["kind"] == args["template"]["kind"]
                && o["name"] == args["template"]["metadata"]["name"]
                && o.get("spec").is_some()
        })?,
        Some("set-up.v1" | "retire.v1" | "register-replacement.v1") => {
            view.object(&args["instance"])?
        }
        Some("open.v1" | "close.v1") => view.fixed(&args["conduit"], "Conduit")?,
        Some("declare-lost.v1") => view.fixed(&args["node"], "Resource")?,
        _ => None,
    };
    let route = match core_op {
        Some("list.v1") => view.object(&path["target"])?.filter(|o| {
            o["from"] == context.qualified("management")
                || o["kind"] == "Pool"
                || o["kind"] == "Fleet"
        }),
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
            Some(o)
                if (o["kind"] == "Resource" || o["kind"] == "Agent") && o.get("spec").is_none() =>
            {
                view.owner(o)?.filter(|p| p["state"] != "retired")
            }
            _ => Some(mgmt),
        },
        Some("inspect.v1") => match named {
            Some(o)
                if (o["kind"] == "Resource" || o["kind"] == "Agent") && o.get("spec").is_none() =>
            {
                view.owner(o)?
                    .filter(|p| p["state"] != "retired")
                    .or(Some(mgmt))
            }
            _ => Some(mgmt),
        },
        Some("retire-with-resources.v1") => view
            .object(&path["target"])?
            .filter(|o| o["kind"] == "Pool" && o["state"] != "retired"),
        Some("retire-with-agents.v1") => view
            .object(&path["target"])?
            .filter(|o| o["kind"] == "Fleet" && o["state"] != "retired"),
        Some("open.v1" | "close.v1") => Some(view.management("access")?),
        Some("reconcile.v1") => {
            let invocation = state
                .invocations
                .iter()
                .find(|v| v["id"] == args["invocation"]);
            match invocation {
                Some(v) => match view.object(v.get("object").unwrap_or(&v["target"]))? {
                    None => Some(mgmt),
                    Some(_) if v["management"] == true => view
                        .object(&v["target"])?
                        .filter(|p| p["state"] != "retired")
                        .or(Some(mgmt)),
                    Some(o) if o["kind"] == "Resource" || o["kind"] == "Agent" => view
                        .owner(o)?
                        .filter(|p| p["state"] != "retired")
                        .or(Some(mgmt)),
                    _ => Some(mgmt),
                },
                None => Some(mgmt),
            }
        }
        _ => view
            .object(&path["target"])?
            .filter(|o| o["state"] != "retired"),
    };
    Ok((route, named))
}

pub(crate) fn evaluate(
    state: &State,
    view: &PublicationView<'_>,
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
    let instance = view
        .find(|o| o["id"] == instance["id"])?
        .ok_or("missing established instance")?;
    let mut selected = None;
    for path in view.paths(&instance["id"]) {
        let path = path?;
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
        state, view, request, context, instance, &caller, filters, path, op, defer,
    )?;
    if verdict.route.is_null() {
        verdict.route = json!({"consumer":instance["id"],"caller":caller,"conduit":path["id"],"target":path["target"],"operation":op});
    }
    Ok(verdict)
}

#[allow(clippy::too_many_arguments)]
fn evaluate_path(
    state: &State,
    view: &PublicationView<'_>,
    request: &AccessRequest,
    context: &Context,
    instance: &Value,
    caller: &Value,
    mut filters: Vec<Value>,
    path: &Value,
    op: &str,
    defer: bool,
) -> Result<Verdict> {
    // ACC-024/ACC-033/ACC-039/ACC-054: objects come from view, invocations from State.
    let target = view
        .object(&path["target"])?
        .ok_or("missing Conduit target")?;

    let (env, scope, contract) = match schemas::operation(target, op, &context.core_package) {
        Ok(c) => c,
        Err(e) if e.starts_with("SCH-015: unresolved") => {
            return Ok(Verdict::reason("refused", "ACC-001"));
        }
        Err(e) => return Err(e),
    };
    let (route, named) = routing(state, view, path, op, &request.arguments, context)?;
    if route.is_none_or(|r| r["id"] != path["target"]) {
        return Ok(Verdict::reason("refused", "ACC-033"));
    }
    let root = view.management("root")?;
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
    // ACC-057: neither delegation nor an onward request serving root may make Agents.
    if (registration || op == context.qualified("register-replacement.v1"))
        && (instance["id"] != root["id"] || request.serving.is_some())
        && (values(&request.arguments, "documents")
            .any(|d| d["kind"] == "Agent" && d.get("from").is_some())
            || (request.arguments["template"]["kind"] == "Agent"
                && request.arguments["template"].get("from").is_some()))
    {
        return Ok(Verdict::reason("refused", "ACC-057"));
    }
    if registration {
        // ACC-054 / SEC-008 / OPS-023: decide authority before validating any document.
        // An onward request, including one whose sender is root, is not root's own act.
        if (instance["id"] != root["id"] || request.serving.is_some())
            && values(&request.arguments, "documents")
                .any(|doc| crate::registration::root_consumed(state, doc, root))
        {
            return Ok(Verdict::reason("refused", "ACC-054"));
        }
        for doc in values(&request.arguments, "documents").filter(|d| {
            (d["kind"] == "Resource" || d["kind"] == "Agent") && d.get("from").is_some()
        }) {
            let family = if doc["kind"] == "Agent" {
                "Fleet"
            } else {
                "Pool"
            };
            if let Some(pool) = view
                .fixed(&doc["owner"], family)?
                .filter(|p| p["kind"] == family && p["state"] != "retired")
                && !view.current_path(&instance["id"], pool, op)?
            {
                return Ok(Verdict::reason("refused", "ACC-033"));
            }
        }
    }
    if op == context.qualified("register-replacement.v1")
        && let Some(pool) = named.filter(|p| p["kind"] == "Pool" && p["state"] != "retired")
        && !view.current_path(&instance["id"], pool, op)?
    {
        return Ok(Verdict::reason("refused", "ACC-033"));
    }
    // LIF-034 also protects the container named implicitly by a cascade.
    if crate::cascade::handles(op, context) && target["first_start"] == true {
        return Ok(Verdict::reason("refused", "LIF-034"));
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
        // ACC-024: only root's own act can close its ordinary Conduits;
        // first-start declarations and FLEET-005 defaults stay protected.
        if closing
            && id(&conduit["consumer"]) == &root["id"]
            && (instance["id"] != root["id"]
                || request.serving.is_some()
                || conduit["provenance"]["cause"] == "default"
                || conduit["provenance"]["cause"] == "first-start"
                || (["management", "access", "system"]
                    .iter()
                    .any(|name| conduit["local_name"] == *name)
                    && ["root-management", "root-access", "root-pool"]
                        .iter()
                        .any(|name| conduit["from"] == context.qualified(name))))
        {
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
                        || view
                            .object(&conduit["consumer"])?
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
        let access_target = id(&conduit["target"]) == &view.management("access")?["id"];
        let old = bound(path, context);
        let new = bound(conduit, context);
        let allowed = if access_target {
            (match (new, old) {
                (_, None) => true,
                (None, Some(_)) => false,
                (Some(new), Some(old)) => subset(new, old, view)?,
            }) && (!opening
                || values(conduit, "operations")
                    .all(|op| values(path, "operations").any(|v| v == op)))
        } else {
            match old {
                None => true,
                Some(b) => {
                    view.object(&conduit["target"])?
                        .is_some_and(|t| covers(b, t))
                        && values(conduit, "operations")
                            .all(|op| values(&b["arguments"], "operations").any(|v| v == op))
                }
            }
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
        view,
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
    if registration {
        for d in values(&request.arguments, "documents")
            .filter(|d| d["kind"] == "Resource" && d.get("from").is_some())
        {
            if view
                .fixed(&d["owner"], "Pool")?
                .is_some_and(|p| p["state"] == "retired")
            {
                return Ok(Verdict::reason("invalid", "POOL-004"));
            }
        }
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
    // CON-024 classifies the operation by its management capability, not its runner.
    let management_verb = context
        .core_environment
        .scopes
        .values()
        .flat_map(|scope| scope.contracts.values())
        .filter(|doc| {
            doc["kind"] == "Capability"
                && [
                    "object-management.v1",
                    "pool-management.v1",
                    "fleet-management.v1",
                    "access-control.v1",
                ]
                .iter()
                .any(|name| doc["metadata"]["name"] == *name)
        })
        .any(|doc| values(&doc["spec"], "operations").any(|operation| operation == op));
    if !defer && target["state"] != "ready" && !management_verb {
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
            "ACC-030", "ACC-033", "ACC-036", "ACC-024", "ACC-057", "IFC-012", "IFC-020", "IFC-022",
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
    let mut verdict = evaluate(
        &t.state,
        &PublicationView::new(&t.state, context),
        request,
        context,
        false,
    )?;
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

#[cfg(test)]
mod notification_tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn unchanged_dependencies_preserve_publication_error_and_consumer() {
        // IFC-033/IFC-038: an unchanged invalid tool must still fail after an earlier consumer.
        let context = Context {
            builtin: None,
            core_package: "test.core".into(),
            meta_prefix: "test/".into(),
            templates: BTreeMap::new(),
            record_source: "urn:test".into(),
            core_environment: schemas::Environment {
                scopes: BTreeMap::from([(
                    "core".into(),
                    schemas::Scope {
                        address: "test.core".into(),
                        version: None,
                        manifest: BTreeMap::new(),
                        contracts: BTreeMap::new(),
                        exports: None,
                    },
                )]),
            },
        };
        let state = State {
            objects: vec![
                json!({"kind":"Resource","id":"target"}),
                json!({"kind":"Conduit","id":"first-path","consumer":"a-first",
                    "target":"target","state":"ready","open":true,"operations":[]}),
                json!({"kind":"Conduit","id":"invalid-path","consumer":"b-failing",
                    "target":"target","state":"ready","open":true,
                    "operations":["example.org/list#act.v1"]}),
            ],
            ..State::default()
        };
        #[cfg(feature = "differential")]
        crate::differential::check(&state, &state, &context).unwrap();
        let view = PublicationView::new(&state, &context);
        let expected = tools(&view, "b-failing", &context).unwrap_err();
        assert!(expected.starts_with("NAM-014:"));
        assert_eq!(
            tool_list_changes(&state, &state, &context),
            Err(expected.clone())
        );
        assert_eq!(
            tool_list_changes_attributed(&state, &state, &context),
            Err(("b-failing".into(), expected))
        );
    }
}
