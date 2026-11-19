//! IFC-050 / ACC-056: public explanations, with disclosure determined by the sender.
use crate::{Context, Result, State, access, inventory, pending, schemas};
use chrono::{DateTime, FixedOffset};
use serde_json::{Value, json};
use std::cmp::Reverse;

/// Deciding metadata stays independent of the diagnostic's serialized ruleId.
pub(crate) struct Rejection {
    pub reason: &'static str,
    pub rule: String,
    pub diagnostics: Option<Value>,
}
impl Rejection {
    pub(crate) fn new(reason: &'static str, rule: &str) -> Self {
        Self {
            reason,
            rule: rule.into(),
            diagnostics: None,
        }
    }
}

pub const REFUSAL_CAUSES: &[(&str, &str)] = &[
    (
        "ACC-001",
        "No open Conduit from the caller to the target as named lists the operation.",
    ),
    (
        "ACC-019",
        "Nothing is dispatched for an invocation that has ended.",
    ),
    (
        "ACC-024",
        "Root may close its own Conduits except first-start and default Conduits.",
    ),
    (
        "ACC-029",
        "The bound does not cover the request; restriction names it.",
    ),
    (
        "ACC-030",
        "Passing on access may not exceed the sender's bound; restriction names it.",
    ),
    ("ACC-032", "Reserved to root acting on its own behalf."),
    (
        "ACC-033",
        "The request needs the target described by its first fitting row. Tier 1 names it only if the request itself did, otherwise it describes that row without resolving an owner name.",
    ),
    (
        "ACC-034",
        "An onward request cannot proceed after a Conduit in its served chain closes.",
    ),
    (
        "ACC-039",
        "A retired consumer can send only requests of its own retirement; this cause does not reveal whether a named object is retired at tier 1.",
    ),
    (
        "ACC-053",
        "Lifting needs access listing lift and a bound covering the kept IDs and operations.",
    ),
    (
        "ACC-054",
        "Only root on its own behalf opens or registers a Conduit whose consumer is root.",
    ),
    (
        "ACC-055",
        "Reconciliation is limited by the bound or to invocations the holder sent.",
    ),
    (
        "ACC-057",
        "Only root on its own behalf registers a submission containing an Agent.",
    ),
    (
        "IFC-021",
        "A Resource makes requests only for an invocation it serves.",
    ),
    (
        "IFC-022",
        "No instance was established by the presented proof.",
    ),
    (
        "IFC-026",
        "The implementation refused under the named restriction.",
    ),
    (
        "IFC-028",
        "The sender must serve the named invocation while it is in progress.",
    ),
    ("IFC-032", "No instance was established."),
    (
        "LIF-034",
        "First-start objects are protected against retirement and unregistration.",
    ),
    (
        "PND-009",
        "Only an Interface acting on its own behalf may answer an action raised there, with the required Conduit; other consumers may not open a Conduit listing answer.",
    ),
    (
        "PND-010",
        "Only an Interface acting on its own behalf may sign in with the required Conduit; a scoped Interface signs in only Agents of its scoped Fleets, and Resources remain root's; other consumers may not open a Conduit listing sign-in.",
    ),
    (
        "SEC-007",
        "get-secret is never dispatched for an instance request; the engine alone reads it.",
    ),
    (
        "SEC-008",
        "Only root on its own behalf opens a secret Conduit, for a Resource or Agent consumer.",
    ),
];

pub const REASON_CAUSES: &[(&str, &str)] = &[
    (
        "ACC-015",
        "The implementation must support every carried filter.",
    ),
    (
        "ACC-037",
        "Access state must be established before deciding a request.",
    ),
    (
        "CON-024",
        "A non-management request requires its target to be ready.",
    ),
    ("CON-031", "Opening requires both ends to be live."),
    (
        "FST-006",
        "Protected first-start templates may be instantiated only by the engine.",
    ),
    (
        "IFC-026",
        "An implementation must be available and able to apply the named restriction.",
    ),
    (
        "IFC-036",
        "A request must meet the protocol and extension requirements.",
    ),
    (
        "LIF-009",
        "Replacement requires a nested instance that is retired or has been removed.",
    ),
    (
        "LIF-015",
        "Setup and change require an instance that is not retired.",
    ),
    ("LIF-017", "Unregistering requires a retired instance."),
    (
        "LIF-020",
        "Retiring a Pool or Fleet requires confirmed shutdown of every member.",
    ),
    (
        "LIF-032",
        "Setup and retirement must name an instance other than a Conduit.",
    ),
    (
        "LIF-041",
        "Registration and opening must not match a retained tombstone.",
    ),
    (
        "NAM-026",
        "A requested operation must have an available selected implementation.",
    ),
    (
        "OPS-023",
        "A request must be readable before it can be decided.",
    ),
    ("OPS-029", "A reference must name an object that exists."),
    (
        "OPS-030",
        "Recording an invocation as failed requires every attempt to be fenced.",
    ),
    (
        "OPS-031",
        "A reconciliation may state revision only when recording a control effect on an existing live instance.",
    ),
    (
        "OPS-032",
        "A repeat must wait until its invocation has ended.",
    ),
    (
        "OPS-033",
        "Request arguments must conform to the operation contract.",
    ),
    (
        "OPS-034",
        "A repeat must be able to return its permitted retained result.",
    ),
    (
        "PKG-024",
        "Installation requires the archive to be available.",
    ),
    (
        "PKG-025",
        "An installed address and version must retain their original content.",
    ),
    (
        "PKG-029",
        "A package address must not be held by a submitted project.",
    ),
    (
        "PLC-011",
        "Retiring or unregistering a node requires no other instances to remain placed on it.",
    ),
    (
        "POOL-003",
        "Replacing a Pool entry requires its previous Resource to be retired or removed.",
    ),
    ("POOL-004", "Registration requires a live owning Pool."),
    (
        "PRV-020",
        "Retiring a runtime provider requires no existing instance to name it as runtime.",
    ),
    ("REV-002", "A change must name the current revision."),
    ("REV-011", "A change must name a revision."),
    (
        "REV-013",
        "Reconciled control values must preserve the Type configuration schema.",
    ),
    (
        "REV-014",
        "Setup, retirement and change must wait for any dispatched setup, retirement or change to end.",
    ),
    (
        "SCH-001",
        "Schemas may use only keywords allowed by the schema profile.",
    ),
    (
        "SUB-009",
        "Replacing a template requires its current revision and a resolved replacement of the same kind and name.",
    ),
    (
        "TPL-013",
        "A requested control must be listed by the Type and present in the template.",
    ),
    (
        "CON-025",
        "A Conduit may be instantiated only while both ends are live.",
    ),
    (
        "INS-002",
        "Instantiation requires from to resolve to a template of the same kind.",
    ),
    (
        "LIF-029",
        "Replacing a declared Conduit requires its previous instance to be retired.",
    ),
    ("OBJ-025", "An object name must be unique within its kind."),
    (
        "OPS-010",
        "A repeated key must carry the same request contents.",
    ),
];

pub fn cause(reason: &str, rule: &str) -> Result<&'static str> {
    let table = if reason == "refused" {
        REFUSAL_CAUSES
    } else {
        REASON_CAUSES
    };
    table
        .iter()
        .find(|(id, _)| *id == rule)
        .map(|(_, text)| *text)
        .ok_or_else(|| format!("explanation has no cause for {rule} ({reason})"))
}

fn is_root(state: &State, sender: &Value, c: &Context) -> bool {
    sender.is_string()
        && state.objects.iter().any(|o| {
            o["id"] == *sender && o["from"] == c.qualified("root") && o.get("spec").is_none()
        })
}

fn visible_closed<'s: 'a, 'a>(
    state: &'s State,
    sender: &'a Value,
    c: &'a Context,
) -> impl Iterator<Item = (&'s String, &'s Value)> + 'a {
    let root = is_root(state, sender, c);
    state.tombstones.iter().filter(move |(_, stone)| {
        stone["object"]["kind"] == "Conduit"
            && stone["consumer"]["id"] == *sender
            && (root || inventory::visible(state, sender, stone, c))
    })
}

fn closed_time(stone: &Value) -> (Option<DateTime<FixedOffset>>, &str) {
    let time = stone["time"].as_str().unwrap_or_default();
    DateTime::parse_from_rfc3339(time).map_or((None, time), |instant| (Some(instant), ""))
}

fn closed_path<'a>(state: &'a State, route: &Value, c: &Context) -> Option<&'a Value> {
    let sender = route.get("consumer").filter(|sender| sender.is_string())?;
    let tool = route.get("tool");
    let local = tool
        .and_then(Value::as_str)
        .and_then(|tool| tool.split_once('.').map(|(local, _)| local));
    visible_closed(state, sender, c)
        .filter(|(_, stone)| match local {
            Some(local) => stone["localName"] == local,
            None => {
                tool.is_none()
                    && route["conduit"]
                        .as_str()
                        .is_some_and(|id| stone["object"]["id"] == id)
            }
        })
        .max_by_key(|(id, stone)| (closed_time(stone), Reverse(*id)))
        .map(|(_, stone)| stone)
}

fn sender_live_path(path: &Value, sender: &Value) -> bool {
    path["kind"] == "Conduit"
        && path["state"] != "retired"
        && path.get("spec").is_none()
        && access::id(&path["consumer"]) == sender
}

fn named_objects(route: &Value) -> impl Iterator<Item = (&'static str, &Value)> {
    ["object", "instance", "conduit", "node"]
        .into_iter()
        .filter_map(|key| {
            route["arguments"]
                .get(key)
                .map(|reference| (key, reference))
        })
}

fn resolve_named<'a>(state: &'a State, key: &str, reference: &Value) -> Option<&'a Value> {
    // Object and instance strings are IDs; node strings are Resource names.
    match (key, reference.is_string()) {
        ("node", _) => access::fixed_object(state, reference, "Resource"),
        ("conduit", true) => access::object_by_id(state, reference)
            .or_else(|| access::fixed_object(state, reference, "Conduit")),
        (_, true) => access::object_by_id(state, reference),
        _ => access::object(state, reference),
    }
}

fn closed_operation<'a>(
    stone: &'a Value,
    route: &'a Value,
    c: &Context,
) -> std::borrow::Cow<'a, str> {
    let operation = route["operation"].as_str().unwrap_or_default();
    if let Some((_, name)) = route["tool"].as_str().and_then(|tool| tool.split_once('.')) {
        if let Some(listed) = stone["operations"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .find(|op| op.rsplit_once('#').is_some_and(|(_, n)| n == name))
        {
            return listed.into();
        }
        if c.core_environment.scopes.values().any(|scope| {
            scope.address == c.core_package
                && scope
                    .contracts
                    .get(name)
                    .is_some_and(|doc| doc["kind"] == "Operation")
        }) {
            return c.qualified(name).into();
        }
        return route["tool"].as_str().unwrap_or(operation).into();
    }
    operation.into()
}

fn answer_refusal_target<'a>(
    state: &'a State,
    route: &Value,
    c: &Context,
) -> Result<Option<&'a Value>> {
    let Some(sender) = access::object_by_id(state, &route["consumer"]) else {
        return Ok(None);
    };
    let Some(action) = pending::action_for_answer(
        state,
        sender,
        route["serving"].as_str(),
        &route["arguments"]["action"],
        c,
    ) else {
        return Ok(None);
    };
    let target = access::action_inspection_target(state, &action, c)?;
    Ok(access::object_by_id(state, target))
}

pub(crate) fn tier(state: &State, route: &Value, c: &Context) -> Result<u8> {
    let sender = &route["consumer"];
    if is_root(state, sender, c) {
        return Ok(2);
    }
    if !sender.is_string() {
        return Ok(1);
    }
    let view = access::PublicationView::new(state, c);
    let path = route["conduit"]
        .as_str()
        .map(|id| view.find(|p| p["id"] == id))
        .transpose()?
        .flatten()
        .filter(|path| sender_live_path(path, sender));
    let kept_path;
    let (path, op) = if let Some(path) = path {
        (path, route["operation"].as_str().unwrap_or_default().into())
    } else if let Some(stone) = closed_path(state, route, c) {
        // An ID reference cannot resolve to a different object with the ID as its name.
        kept_path = json!({"target":{"id":stone["target"]["id"]}});
        (&kept_path, closed_operation(stone, route, c))
    } else {
        return Ok(1);
    };
    // ACC-033 row 9 (enrollment) is missing from routing, so the default Conduit target cannot
    // establish disclosure; row 16 (sign-in.v1) stays at tier 1 so that a refusal discloses
    // nothing of the named instance (PND-010, OPS-023).
    if matches!(
        op.strip_prefix(&c.core_package)
            .and_then(|op| op.strip_prefix('#')),
        Some("enroll-machine.v1" | "admit-machine.v1" | "refuse-machine.v1" | "sign-in.v1")
    ) {
        return Ok(1);
    }
    let args = &route["arguments"];
    let mut routed_args;
    let routing_args = if route.get("conduit").is_some()
        && route.get("tool").is_none()
        && let Some(object) = args.get("object")
    {
        routed_args = args.clone();
        routed_args["instance"] = object.clone();
        &routed_args
    } else {
        args
    };
    let (target, named) = if op == c.qualified("answer.v1") {
        (answer_refusal_target(state, route, c)?, None)
    } else {
        access::routing(state, &view, path, &op, routing_args, c)?
    };
    let Some(target) = target else {
        return Ok(1);
    };
    if !access::may_inspect_unfiltered(state, sender, target, c)? {
        return Ok(1);
    }
    if op == c.qualified("register.v1") {
        for doc in args["documents"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|d| d.get("from").is_some())
        {
            let family = match doc["kind"].as_str() {
                Some("Resource") => "Pool",
                Some("Agent") => "Fleet",
                _ => continue,
            };
            // Submit also finds an owner by ID; a name is tried first.
            if let Some(owner) = view
                .fixed(&doc["owner"], family)?
                .or_else(|| access::object_by_id(state, &doc["owner"]))
                .filter(|p| p["kind"] == family && p["state"] != "retired")
                && !access::may_inspect_unfiltered(state, sender, owner, c)?
            {
                return Ok(1);
            }
        }
    }
    if op == c.qualified("register-replacement.v1")
        && let Some(pool) = named.filter(|p| p["kind"] == "Pool" && p["state"] != "retired")
        && !access::may_inspect_unfiltered(state, sender, pool, c)?
    {
        return Ok(1);
    }
    for (key, reference) in named_objects(route) {
        let inspectable = match resolve_named(state, key, reference) {
            Some(object) => access::may_inspect_unfiltered(state, sender, object, c)?,
            None => access::may_inspect_missing(state, sender, c)?,
        };
        if !inspectable {
            return Ok(1);
        }
    }
    Ok(2)
}

/// A tool's spelling is resolved through the sending instance, even for onward calls.
pub(crate) fn access_route(state: &State, request: &access::AccessRequest, c: &Context) -> Value {
    let sender = access::established(state, &request.identity).ok();
    let path = request.tool.split_once('.').and_then(|(local, _)| {
        state.objects.iter().find(|p| {
            p["kind"] == "Conduit"
                && p["state"] != "retired"
                && sender.is_some_and(|s| access::id(&p["consumer"]) == &s["id"])
                && p.get("local_name")
                    .or_else(|| p["nested"].get("name"))
                    .unwrap_or(&p["name"])
                    == local
        })
    });
    let operation = path
        .and_then(|p| {
            let (_, name) = request.tool.split_once('.')?;
            // The found Conduit's own listed operation names its package exactly.
            if let Some(op) = p["operations"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .find(|op| op.rsplit_once('#').is_some_and(|(_, n)| n == name))
            {
                return Some(op.to_owned());
            }
            let target =
                crate::registration::publication_object(access::object(state, &p["target"])?, c)
                    .ok()?;
            // Then the Conduit's own Capability, which pins the package of each member exactly.
            if let Some(cap) = p["capability"].as_str()
                && let Ok((env, scope, doc)) = schemas::operation(&target, cap, &c.core_package)
                && doc["kind"] == "Capability"
                && let Some(held) = env.scopes.get(&scope)
                && let Some(op) = doc["spec"]["operations"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .filter_map(|member| {
                        let (address, n) = member.rsplit_once('#')?;
                        (n == name).then(|| {
                            let address = if address.is_empty() {
                                &held.address
                            } else {
                                address
                            };
                            format!("{address}#{name}")
                        })
                    })
                    .find(|qualified| {
                        schemas::operation(&target, qualified, &c.core_package)
                            .is_ok_and(|(_, _, doc)| doc["kind"] == "Operation")
                    })
            {
                return Some(op);
            }
            // Then the target's own contract scope, then the core, independent of publication
            // and opening; never the environment's order, since packages may share an export name.
            [format!("#{name}"), c.qualified(name)]
                .into_iter()
                .find_map(|reference| {
                    let (env, scope, doc) =
                        schemas::operation(&target, &reference, &c.core_package).ok()?;
                    if doc["kind"] != "Operation" {
                        return None;
                    }
                    Some(format!("{}#{name}", env.scopes.get(&scope)?.address))
                })
        })
        .unwrap_or_else(|| request.tool.clone());
    json!({"consumer":sender.map(|s| &s["id"]), "tool":request.tool,
        "operation":operation, "conduit":path.map(|p| &p["id"]), "target":path.map(|p| &p["target"]), "arguments":request.arguments,
        "serving":request.serving})
}

pub(crate) fn ids(request: &crate::Request, arguments: Value) -> Value {
    json!({"consumer":request.sender,"caller":request.caller,"conduit":request.conduit,
        "target":request.target,"operation":request.operation,"arguments":arguments})
}

fn acc033_target(state: &State, route: &Value, c: &Context) -> String {
    let args = &route["arguments"];
    let op = route["operation"].as_str().unwrap_or("");
    match op.strip_prefix(&format!("{}#", c.core_package)) {
        Some("register-replacement.v1") => {
            // Routing requires management first, then authority on the named live Pool.
            if access::object(state, &route["target"])
                .is_none_or(|o| o["from"] != c.qualified("management"))
            {
                return "management".into();
            }
            let reference = &args["instance"];
            if let Some(pool) = access::object(state, reference)
                .filter(|p| p["kind"] == "Pool" && p["state"] != "retired")
                && !state.objects.iter().any(|path| {
                    crate::permits(
                        path,
                        route["consumer"].as_str().unwrap_or(""),
                        &pool["id"],
                        op,
                    )
                })
                && let (Some(kind), Some(name)) =
                    (reference["kind"].as_str(), reference["name"].as_str())
            {
                return format!("the named {kind} {name}");
            }
            "management".into()
        }
        Some("register.v1") => {
            if let Some(doc) = args["documents"]
                .as_array()
                .into_iter()
                .flatten()
                .find(|d| {
                    (d["kind"] == "Resource" || d["kind"] == "Agent")
                        && d.get("from").is_some()
                        && access::fixed_object(
                            state,
                            &d["owner"],
                            if d["kind"] == "Agent" {
                                "Fleet"
                            } else {
                                "Pool"
                            },
                        )
                        .is_some_and(|p| {
                            p["state"] != "retired"
                                && !state.objects.iter().any(|path| {
                                    crate::permits(
                                        path,
                                        route["consumer"].as_str().unwrap_or(""),
                                        &p["id"],
                                        op,
                                    )
                                })
                        })
                })
            {
                let family = if doc["kind"] == "Agent" {
                    "Fleet"
                } else {
                    "Pool"
                };
                let owner = doc["owner"]
                    .as_str()
                    .or_else(|| doc["owner"]["name"].as_str());
                if let Some(owner) = owner {
                    return format!("management and the owning {family} {owner}");
                }
            }
            "management".into()
        }
        Some("open.v1" | "close.v1" | "lift.v1") => "access".into(),
        Some("answer.v1") => "the target that authorizes inspection of the action's subject".into(),
        Some("sign-in.v1") => "the named Agent's owning Fleet, from an Interface acting on its own behalf; a Resource only from root, through its owning Pool (PND-010)".into(),
        Some("set-up.v1" | "retire.v1" | "inspect.v1") => {
            let reference = args
                .get("instance")
                .or_else(|| args.get("object"))
                .unwrap_or(&Value::Null);
            let named = access::object(state, reference);
            match named {
                Some(o)
                    if o.get("spec").is_none()
                        && (o["kind"] == "Resource" || o["kind"] == "Agent") =>
                {
                    let family = if o["kind"] == "Agent" {
                        "Fleet"
                    } else {
                        "Pool"
                    };
                    if op == c.qualified("inspect.v1")
                        && !access::object(state, &o["owner"])
                            .is_some_and(|owner| owner["state"] != "retired")
                    {
                        "management".into()
                    } else {
                        format!(
                            "the owning {family} of the named {}",
                            o["kind"].as_str().unwrap()
                        )
                    }
                }
                _ => "management".into(),
            }
        }
        Some("retire-with-resources.v1" | "retire-with-agents.v1" | "list.v1") => {
            let target = access::object(state, &route["target"]);
            if target.is_some_and(|o| o["from"] == c.qualified("management")) {
                return "management".into();
            }
            if let Some(reference) = args.get("object").or_else(|| args.get("instance"))
                && let (Some(kind), Some(name)) =
                    (reference["kind"].as_str(), reference["name"].as_str())
            {
                return format!("the named {kind} {name}");
            }
            "the target of ACC-033's first fitting row".into()
        }
        Some("reconcile.v1") => {
            let invocation = state
                .invocations
                .iter()
                .find(|v| v["id"] == args["invocation"]);
            if let Some(v) = invocation
                && let Some(object) = access::object(state, v.get("object").unwrap_or(&v["target"]))
            {
                let target = if v["management"] == true {
                    access::object(state, &v["target"])
                } else if object["kind"] == "Resource" || object["kind"] == "Agent" {
                    access::object(state, &object["owner"])
                } else {
                    None
                }
                .filter(|o| o["state"] != "retired");
                if let Some(target) = target {
                    if target["from"] == c.qualified("access") {
                        return "access".into();
                    }
                    if target["kind"] == "Pool" {
                        return "the owning Pool of the named Resource".into();
                    }
                    if target["kind"] == "Fleet" {
                        return "the owning Fleet of the named Agent".into();
                    }
                }
            }
            "management".into()
        }
        Some("replace.v1" | "unregister.v1" | "install.v1" | "declare-lost.v1" | "purge.v1") => {
            "management".into()
        }
        _ if op.contains('#') => "the named instance itself".into(),
        _ => "the target of ACC-033's first fitting row".into(),
    }
}

/// One diagnostic for a reason. Routes carry only the inputs already held at the deciding site.
pub(crate) fn diagnostic(
    state: &State,
    route: &Value,
    c: &Context,
    reason: &str,
    rule: &str,
) -> Result<Value> {
    let operation = route["operation"]
        .as_str()
        .ok_or("explanation lacks operation")?;
    let view = access::PublicationView::new(state, c);
    let path = route["conduit"]
        .as_str()
        .map(|id| view.find(|p| p["id"] == id))
        .transpose()?
        .flatten();
    let tool = route["tool"].as_str().map(str::to_owned).or_else(|| {
        path.and_then(|p| p["local_name"].as_str()).map(|local| {
            format!(
                "{local}.{}",
                operation
                    .rsplit_once('#')
                    .map_or(operation, |(_, name)| name)
            )
        })
    });
    let subject = tool.as_deref().unwrap_or(operation);
    let cause = cause(reason, rule)?;
    if reason != "refused" {
        return Ok(
            json!({"ruleId":rule,"level":"error","message":{"text":format!("{} reports {reason} under {rule}: {cause}", route["tool"].as_str().unwrap_or(operation))}}),
        );
    }
    let cause = if rule == "ACC-033" {
        format!(
            "The request needs an open Conduit listing the operation to {}.",
            acc033_target(state, route, c)
        )
    } else {
        cause.into()
    };
    let mut named = json!({});
    if let Some(tool) = &tool {
        named["tool"] = json!(tool);
    }
    if let Some(local) = path.and_then(|p| p["local_name"].as_str()) {
        named["conduit"] = json!(local);
    }
    let objects: Vec<_> = named_objects(route).collect();
    if !objects.is_empty() {
        named["objects"] = json!(objects.iter().map(|(_, value)| value).collect::<Vec<_>>());
    }
    let tier = tier(state, route, c)?;
    let mut refusal = json!({"tier":tier,"operation":operation,"named":named,"cause":cause});
    if tier == 2 {
        let sender = &route["consumer"];
        let path = path.filter(|path| is_root(state, sender, c) || sender_live_path(path, sender));
        let stone = path
            .is_none()
            .then(|| closed_path(state, route, c))
            .flatten();
        let presented = if let Some(path) = path {
            access::object(state, &path["target"])
        } else {
            stone.and_then(|stone| stone.get("target"))
        };
        // The operation that tier() resolved, through a closed Conduit as well.
        let resolved = stone.map_or(operation.into(), |stone| closed_operation(stone, route, c));
        // PND-009 never resolves an action in the diagnostic, even at root.
        let target = if resolved == c.qualified("answer.v1") && rule != "PND-009" {
            answer_refusal_target(state, route, c)?.or(presented)
        } else {
            presented
        };
        let mut conduits = vec![];
        if let Some(target) = target {
            refusal["target"] = inventory::object(target);
            conduits.extend(state.objects.iter()
                .filter(|p| sender_live_path(p, sender) && access::id(&p["target"]) == &target["id"])
                .map(|p| json!({"name":p["name"],"as":p.get("local_name").or_else(|| p["nested"].get("name")).unwrap_or(&p["name"]),"state":if p["open"] == true {"open"} else {"registered"},"operations":p["operations"]})));
            let mut closed: Vec<_> = visible_closed(state, sender, c)
                .filter(|(_, stone)| stone["target"]["id"] == target["id"])
                .collect();
            closed.sort_by_cached_key(|(id, stone)| (closed_time(stone), *id));
            conduits.extend(closed.into_iter().map(|(_, stone)| {
                json!({"name":stone["object"]["name"],"as":stone["localName"],"state":"closed","operations":stone["operations"]})
            }));
        }
        refusal["exists"] = json!(
            path.is_some()
                && objects
                    .iter()
                    .all(|(key, reference)| resolve_named(state, key, reference).is_some())
        );
        refusal["conduits"] = json!(conduits);
    }
    Ok(
        json!({"ruleId":rule,"level":"error","message":{"text":format!("{subject} is refused under {rule}: {cause}")},"properties":{"refusal":refusal}}),
    )
}

pub const OUTCOME_CAUSES: &[(&str, &str)] = &[
    (
        "IFC-008",
        "The implementation rejected the declared Conduit extension revision and performed nothing.",
    ),
    (
        "IFC-010",
        "The plain implementation's answer does not establish whether the operation acted.",
    ),
    (
        "IFC-046",
        "The implementation rejected the parameter headers and the operation does not permit a resend.",
    ),
    (
        "IFC-047",
        "The implementation required an undeclared capability and performed nothing.",
    ),
    (
        "IMP-007",
        "The invocation was in progress when the engine restarted.",
    ),
    (
        "LIF-021",
        "The cascade was cancelled and dispatched no further retirement.",
    ),
    (
        "OPS-007",
        "The implementation gave no valid result establishing the outcome.",
    ),
    (
        "OPS-027",
        "The invocation was cancelled without confirmation that nothing was done.",
    ),
];

pub(crate) fn outcome_cause(invocation: &Value, outcome: &str, rule: &str) -> Result<Value> {
    let cause = OUTCOME_CAUSES
        .iter()
        .find_map(|(id, cause)| (*id == rule).then_some(*cause))
        .ok_or_else(|| format!("no outcome cause for {rule}"))?;
    outcome_diagnostic(invocation, outcome, rule, cause)
}

pub(crate) fn outcome_diagnostic(
    invocation: &Value,
    outcome: &str,
    rule: &str,
    cause: &str,
) -> Result<Value> {
    let operation = invocation["operation"]
        .as_str()
        .ok_or("outcome explanation lacks operation")?;
    let subject = invocation["named_subject"]
        .as_str()
        .map_or_else(String::new, |subject| format!(" for {subject}"));
    Ok(json!({
        "ruleId":rule,"level":"error",
        "message":{"text":format!("{operation}{subject} ended {outcome} under {rule}: {cause}")}
    }))
}

pub(crate) fn settle_cause(
    invocation: &Value,
    outcome: &str,
    parts: &[Value],
) -> Result<Option<Vec<Value>>> {
    if outcome == "succeeded" {
        return Ok(None);
    }
    let part = parts
        .iter()
        .find(|p| {
            p["optional"] != true
                && p["outcome"] != "succeeded"
                && !(p["name"] == "account" && invocation["setup"]["accountCounted"].is_string())
        })
        .ok_or("unsucceeded invocation lacks an unsucceeded required part")?;
    let name = part["name"].as_str().ok_or("part lacks name")?;
    let status = part["outcome"].as_str().ok_or("part lacks outcome")?;
    let prefix = if invocation["cancelled"] == true {
        "The invocation was cancelled. "
    } else {
        ""
    };
    let cause = format!("{prefix}Required part {name} ended {status}.");
    Ok(Some(vec![outcome_diagnostic(
        invocation, outcome, "OPS-022", &cause,
    )?]))
}

/// IFC-025: run after all diagnostics are final, once per returned or stored reply.
pub fn first_text(reply: &mut Value, c: &Context) {
    if reply["isError"] != true {
        return;
    }
    let status = reply["_meta"]
        .get(format!("{}reason", c.meta_prefix))
        .or_else(|| reply["_meta"].get(format!("{}outcome", c.meta_prefix)))
        .and_then(Value::as_str)
        .unwrap_or("");
    let messages: Vec<&str> = reply["structuredContent"]["diagnostics"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|d| d["level"] == "error")
        .filter_map(|d| d["message"]["text"].as_str())
        .collect();
    let text = if messages.is_empty() {
        status.into()
    } else {
        format!("{status}: {}", messages.join(" "))
    };
    let first = json!({"type":"text","text":text});
    if let Some(content) = reply["content"].as_array_mut() {
        if content.is_empty() {
            content.push(first);
        } else {
            content[0] = first;
        }
    } else {
        reply["content"] = json!([first]);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::{
        Evidence, Transition,
        access::{AccessRequest, Identity},
    };
    use std::collections::BTreeMap;

    #[test]
    fn settle_cause_skips_a_counted_account_check() {
        let setup = json!({
            "operation":"set-up.v1",
            "setup":{"accountCounted":"failed"}
        });
        let parts = json!([
            {"name":"account","outcome":"failed"},
            {"name":"hook","outcome":"failed"}
        ]);
        let cause = settle_cause(&setup, "unknown", parts.as_array().unwrap())
            .unwrap()
            .unwrap();
        assert!(
            cause[0]["message"]["text"]
                .as_str()
                .unwrap()
                .ends_with("Required part hook ended failed.")
        );
    }

    #[test]
    fn outcome_causes_are_sentences_and_unknown_rules_are_errors() {
        for (rule, cause) in OUTCOME_CAUSES {
            let body = cause
                .strip_suffix('.')
                .unwrap_or_else(|| panic!("{rule}: {cause}"));
            assert!(
                !body.trim().is_empty() && !body.contains(['.', '?', '!']),
                "{rule}: {cause}"
            );
        }
        let error =
            outcome_cause(&json!({"operation":"test#work.v1"}), "unknown", "ZZZ-999").unwrap_err();
        assert!(error.contains("ZZZ-999"), "{error}");
    }

    pub(crate) fn assert_cause(reply: &Value, rule: &str, text: &str) {
        let diagnostics = reply["structuredContent"]["diagnostics"]
            .as_array()
            .unwrap();
        assert_eq!(diagnostics.len(), 1, "{reply}");
        assert_eq!(diagnostics[0]["ruleId"], rule);
        assert_eq!(diagnostics[0]["level"], "error");
        let message = diagnostics[0]["message"]["text"].as_str().unwrap();
        assert!(message.contains(text), "{message}");
    }

    pub(crate) fn fixture() -> (State, Context) {
        let core = "test.core";
        let mut contracts = BTreeMap::new();
        for name in [
            "list.v1",
            "retire.v1",
            "open.v1",
            "close.v1",
            "register.v1",
            "register-replacement.v1",
            "reconcile.v1",
            "get-secret.v1",
            "reserved.v1",
            "set-up.v1",
            "control.v1",
        ] {
            contracts.insert(name.into(),json!({"kind":"Operation","metadata":{"name":name},"spec":{"inputSchema":{"type":"object"},"reservedToRoot":name == "reserved.v1"}}));
        }
        contracts.insert(
            "type.v1".into(),
            json!({"kind":"Type","metadata":{"name":"type.v1"},"spec":{"controls":[]}}),
        );
        let env = schemas::Environment {
            scopes: BTreeMap::from([(
                "core".into(),
                schemas::Scope {
                    address: core.into(),
                    version: None,
                    manifest: BTreeMap::new(),
                    contracts,
                    exports: None,
                },
            )]),
        };
        let c = Context {
            builtin: None,
            core_package: core.into(),
            meta_prefix: "test/".into(),
            templates: BTreeMap::new(),
            record_source: "urn:test".into(),
            core_environment: env.clone(),
        };
        let mut state = State {
            started: true,
            ..State::default()
        };
        for (id, kind) in [
            ("root", "Interface"),
            ("agent", "Agent"),
            ("other", "Interface"),
            ("management", "Resource"),
            ("access", "Resource"),
            ("owners", "Pool"),
        ] {
            state.objects.push(json!({"kind":kind,"id":id,"name":id,"from":c.qualified(id),"type":c.qualified("type.v1"),"state":"ready","config":{},"schema_environment":env,"contract_scope":"core"}));
        }
        for consumer in ["root", "agent", "other"] {
            for target in ["management", "access"] {
                state.objects.push(json!({"kind":"Conduit","id":format!("{consumer}-{target}"),"name":format!("{consumer}-{target}"),"consumer":consumer,"target":target,"local_name":target,"state":"ready","open":true,
                    "operations":[c.qualified("list.v1"),c.qualified("retire.v1"),c.qualified("register.v1"),c.qualified("register-replacement.v1"),c.qualified("reconcile.v1"),c.qualified("open.v1"),c.qualified("close.v1"),c.qualified("set-up.v1"),c.qualified("get-secret.v1"),c.qualified("reserved.v1")] }));
            }
        }
        (state, c)
    }
    pub(crate) fn evidence() -> Evidence {
        Evidence {
            time: "2026-11-09T13:00:00Z".into(),
            ids: (1..50)
                .map(|n| format!("00000000-0000-4000-8046-a0000000{n:04}"))
                .collect(),
            revisions: vec![],
            suffixes: vec![],
        }
    }
    pub(crate) fn transition(state: State) -> Transition {
        Transition {
            state,
            decisions: vec![],
            records: vec![],
            external_intents: vec![],
            reply: Value::Null,
            request_replies: vec![],
            input_required: vec![],
        }
    }
    fn request(sender: &str, tool: &str, args: Value) -> AccessRequest {
        AccessRequest {
            identity: Identity::Instance { id: sender.into() },
            tool: tool.into(),
            arguments: args,
            serving: None,
            readable: true,
            authority_readable: true,
            client_capabilities: None,
            sign_in_login: None,
            sign_in_return: None,
        }
    }

    fn inspector_fixture() -> (State, Context) {
        let (mut s, c) = fixture();
        for (id, kind) in [
            ("v", "Interface"),
            ("x", "Resource"),
            ("p", "Pool"),
            ("sys", "Pool"),
        ] {
            s.objects.push(json!({"kind":kind,"id":id,"name":id,"state":"ready",
                "type":c.qualified("type.v1"),"config":{},"schema_environment":c.core_environment,"contract_scope":"core"}));
        }
        item_mut(&mut s, "x")["owner"] = json!("p");
        item_mut(&mut s, "management")["owner"] = json!("sys");
        s.objects.push(
            json!({"kind":"Conduit","id":"v-x","name":"v-x","local_name":"x",
            "consumer":"v","target":"x","open":true,"state":"ready","operations":[]}),
        );
        (s, c)
    }

    fn item_mut<'a>(s: &'a mut State, id: &str) -> &'a mut Value {
        s.objects.iter_mut().find(|o| o["id"] == id).unwrap()
    }

    fn inspection(s: &mut State, target: &str, c: &Context) {
        let id = format!("v-{target}");
        s.objects.push(
            json!({"kind":"Conduit","id":id,"name":id,"local_name":target,"consumer":"v",
            "target":target,"open":true,"state":"ready","operations":[c.qualified("inspect.v1")]}),
        );
    }

    fn refusal(s: &State, c: &Context, route: &Value, rule: &str) -> Value {
        diagnostic(s, route, c, "refused", rule).unwrap()["properties"]["refusal"].clone()
    }

    fn expected_inspection(s: &State, c: &Context, sender: &str, id: &str, expected: bool) {
        let item = access::object_by_id(s, &json!(id)).unwrap();
        let sender = json!(sender);
        assert_eq!(access::may_inspect(s, &sender, item, c).unwrap(), expected);
        assert_eq!(
            access::may_inspect_unfiltered(s, &sender, item, c).unwrap(),
            expected
        );
    }

    #[test]
    fn tier_two_requires_an_open_unfiltered_inspection_path_for_interfaces_and_agents() {
        let (mut s, c) = inspector_fixture();
        let route = access_route(&s, &request("v", "x.ping.v1", json!({})), &c);
        inspection(&mut s, "p", &c);
        let expected = json!([{"name":"v-x","as":"x","state":"open","operations":[]}]);
        for kind in ["Interface", "Agent"] {
            item_mut(&mut s, "v")["kind"] = json!(kind);
            expected_inspection(&s, &c, "v", "x", true);
            let r = refusal(&s, &c, &route, "ACC-001");
            assert_eq!(r["tier"], 2);
            assert_eq!(r["target"], json!({"kind":"Resource","id":"x","name":"x"}));
            assert_eq!(r["conduits"], expected);
        }
        for (field, value) in [
            ("open", json!(false)),
            ("operations", json!([])),
            (
                "requirements",
                json!({"filter":{"requirement":"example.org/p#filter.v1"}}),
            ),
            (
                "requirements",
                json!({"bound":{"requirement":c.qualified("bound.v1")}}),
            ),
        ] {
            let mut changed = s.clone();
            item_mut(&mut changed, "v-p")[field] = value.clone();
            let want = if value.get("bound").is_some() { 2 } else { 1 };
            assert_eq!(refusal(&changed, &c, &route, "ACC-001")["tier"], want);
            if value.get("filter").is_none() {
                expected_inspection(&changed, &c, "v", "x", want == 2);
            }
        }
    }

    #[test]
    fn tier_two_checks_named_objects_and_uses_management_itself_for_missing_objects() {
        let (mut s, c) = inspector_fixture();
        inspection(&mut s, "p", &c);
        let mut route = access_route(&s, &request("v", "x.ping.v1", json!({"object":"p"})), &c);
        expected_inspection(&s, &c, "v", "x", true);
        expected_inspection(&s, &c, "v", "p", false);
        expected_inspection(&s, &c, "v", "management", false);
        assert_eq!(refusal(&s, &c, &route, "ACC-001")["tier"], 1);
        route["arguments"]["object"] = json!("missing");
        assert_eq!(refusal(&s, &c, &route, "ACC-001")["tier"], 1);
        for target in ["sys", "management"] {
            let mut changed = s.clone();
            inspection(&mut changed, target, &c);
            expected_inspection(&changed, &c, "v", "x", true);
            expected_inspection(&changed, &c, "v", "management", target == "sys");
            let r = refusal(&changed, &c, &route, "ACC-001");
            if target == "management" {
                assert_eq!(r["tier"], 2);
                assert_eq!(r["exists"], false);
            } else {
                assert_eq!(r["tier"], 1);
                assert!(r.get("exists").is_none());
            }
        }
        // Without a live owner, both readings route a missing object to management itself.
        let mut ownerless = s.clone();
        item_mut(&mut ownerless, "management")
            .as_object_mut()
            .unwrap()
            .remove("owner");
        expected_inspection(&ownerless, &c, "v", "management", false);
        assert_eq!(refusal(&ownerless, &c, &route, "ACC-001")["tier"], 1);
        inspection(&mut ownerless, "management", &c);
        expected_inspection(&ownerless, &c, "v", "management", true);
        let r = refusal(&ownerless, &c, &route, "ACC-001");
        assert_eq!(r["tier"], 2);
        assert_eq!(r["exists"], false);
    }

    #[test]
    fn tier_two_checks_the_pool_that_authorizes_retirement() {
        let (mut s, c) = inspector_fixture();
        inspection(&mut s, "p", &c);
        let route = access_route(
            &s,
            &request("v", "p.retire.v1", json!({"instance":"x"})),
            &c,
        );
        assert_eq!(route["operation"], c.qualified("retire.v1"));
        expected_inspection(&s, &c, "v", "x", true);
        expected_inspection(&s, &c, "v", "p", false);
        assert_eq!(refusal(&s, &c, &route, "ACC-001")["tier"], 1);
        inspection(&mut s, "management", &c);
        expected_inspection(&s, &c, "v", "x", true);
        expected_inspection(&s, &c, "v", "p", true);
        assert_eq!(refusal(&s, &c, &route, "ACC-001")["tier"], 2);
    }

    fn assert_refusal_records(t: &Transition, refusal: &Value, tier: u8) {
        assert_eq!(refusal["tier"], tier);
        for event in ["access-decision", "request-ended"] {
            let data = &t.records.iter().find(|r| r["type"] == event).unwrap()["data"];
            assert_eq!(data["refusalTier"], refusal["tier"]);
            assert!(data.get("exists").is_none() && data.get("conduits").is_none());
            assert!(data["target"].is_null() || data["target"].is_string());
        }
    }

    #[test]
    fn id_retirement_checks_the_pool_and_records_the_diagnostic_tier() {
        let (mut s, c) = inspector_fixture();
        item_mut(&mut s, "x")["owner"] = json!("sys");
        inspection(&mut s, "sys", &c);
        for tier in [1, 2] {
            expected_inspection(&s, &c, "v", "x", true);
            expected_inspection(&s, &c, "v", "sys", tier == 2);
            let t = crate::step(
                &s,
                crate::Input::Request {
                    evidence: evidence(),
                    object: "x".into(),
                    request: crate::Request {
                        sender: "v".into(),
                        caller: "v".into(),
                        conduit: "v-sys".into(),
                        target: "sys".into(),
                        operation: c.qualified("retire.v1"),
                    },
                },
                &c,
            )
            .unwrap();
            assert_refusal_records(
                &t,
                &t.reply["diagnostics"][0]["properties"]["refusal"],
                tier,
            );
            if tier == 1 {
                inspection(&mut s, "management", &c);
            }
        }
    }

    #[test]
    fn id_refusal_cannot_use_a_foreign_live_conduit_for_disclosure() {
        let (mut s, c) = inspector_fixture();
        inspection(&mut s, "p", &c);
        inspection(&mut s, "management", &c);
        expected_inspection(&s, &c, "v", "p", true);
        expected_inspection(&s, &c, "v", "x", true);
        expected_inspection(&s, &c, "v", "management", false);

        let request = crate::Request {
            sender: "v".into(),
            caller: "v".into(),
            conduit: "root-management".into(),
            target: "management".into(),
            operation: c.qualified("retire.v1"),
        };
        let t = crate::step(
            &s,
            crate::Input::Request {
                evidence: evidence(),
                object: "x".into(),
                request,
            },
            &c,
        )
        .unwrap();
        let r = &t.reply["diagnostics"][0]["properties"]["refusal"];
        assert_refusal_records(&t, r, 1);
        assert_eq!(r["operation"], c.qualified("retire.v1"));
        assert_eq!(
            r["named"],
            json!({"tool":"management.retire.v1","conduit":"management","objects":["x"]})
        );
        for key in ["target", "exists", "conduits"] {
            assert!(r.get(key).is_none());
        }

        let root_route =
            json!({"consumer":"root","conduit":"v-x","operation":c.qualified("retire.v1")});
        let r = refusal(&s, &c, &root_route, "ACC-001");
        assert_eq!(r["tier"], 2);
        assert_eq!(r["target"], json!({"kind":"Resource","name":"x","id":"x"}));
        assert_eq!(r["exists"], true);
        assert_eq!(r["conduits"], json!([]));
    }

    #[test]
    fn unimplemented_authorizing_routes_keep_non_root_refusals_at_tier_one() {
        let (mut s, c) = inspector_fixture();
        inspection(&mut s, "p", &c);
        expected_inspection(&s, &c, "v", "x", true);
        expected_inspection(&s, &c, "v", "p", false);
        expected_inspection(&s, &c, "v", "management", false);
        // sign-in.v1's route is built (order 048a-5b); its tier 1 is pinned by
        // sign_in_access_refuses_out_of_scope_callers_without_detail.
        for op in ["enroll-machine.v1", "admit-machine.v1", "refuse-machine.v1"] {
            let operation = c.qualified(op);
            item_mut(&mut s, "v-x")["operations"] = json!([operation]);
            let route = access_route(
                &s,
                &request("v", &format!("x.{op}"), json!({"instance":"x"})),
                &c,
            );
            let r = refusal(&s, &c, &route, "ACC-001");
            assert_eq!(r["operation"], operation);
            assert_eq!(r["tier"], 1, "{op}");
            assert!(r.get("target").is_none());
        }
    }

    fn sign_in_fixture() -> (State, Context) {
        let (mut state, mut c) = fixture();
        let operation = c.qualified("sign-in.v1");
        let contracts = &mut c.core_environment.scopes.get_mut("core").unwrap().contracts;
        contracts.insert(
            "sign-in.v1".into(),
            json!({"kind":"Operation","metadata":{"name":"sign-in.v1"},
                "spec":{"inputSchema":{"type":"object","required":["instance"],
                    "properties":{"instance":{"type":"object"}}},
                    "outputSchema":{"type":"object","required":["account"],
                        "properties":{"account":{"type":"string"}}},"effect":"Sign in"}}),
        );
        for name in ["fleet-management.v1", "pool-management.v1"] {
            contracts.insert(
                name.into(),
                json!({"kind":"Capability","metadata":{"name":name},
                    "spec":{"operations":[operation]}}),
            );
        }
        for (id, name, kind, owner) in [
            ("f", "f", "Fleet", None),
            ("f2", "f2", "Fleet", None),
            ("p", "p", "Pool", None),
            ("v", "v", "Interface", None),
            ("w", "w", "Interface", None),
            ("signed-instance-id", "signed", "Agent", Some("f")),
            ("s2", "s2", "Agent", Some("f")),
            ("x", "x", "Resource", Some("p")),
        ] {
            let mut object = json!({"id":id,"name":name,"kind":kind,"state":"ready",
                "type":c.qualified("type.v1"),"config":{},"schema_environment":c.core_environment,
                "contract_scope":"core"});
            if let Some(owner) = owner {
                object["owner"] = json!(owner);
            }
            state.objects.push(object);
        }
        for (id, consumer, target, capability) in [
            ("r-f", "root", "f", "fleet-management.v1"),
            ("r-p", "root", "p", "pool-management.v1"),
            ("v-f", "v", "f", "fleet-management.v1"),
            ("v-f2", "v", "f2", "fleet-management.v1"),
            ("v-p", "v", "p", "pool-management.v1"),
            ("s2-f", "s2", "f", "fleet-management.v1"),
        ] {
            state.objects.push(
                json!({"kind":"Conduit","id":id,"name":id,"local_name":target,
                "consumer":consumer,"target":target,"state":"ready","open":true,
                "capability":c.qualified(capability),"operations":[operation]}),
            );
        }
        state.objects.push(
            json!({"kind":"Conduit","id":"w-v","name":"w-v","local_name":"v",
            "consumer":"w","target":"v","state":"ready","open":true,"operations":[]}),
        );
        state
            .invocations
            .push(json!({"id":"call","target":"v","caller":"w",
            "conduit":"w-v","state":"in-progress"}));
        (state, c)
    }

    #[test]
    fn sign_in_access_refuses_out_of_scope_callers_without_detail() {
        let (mut state, c) = sign_in_fixture();
        let unscoped = state.clone();
        for target in ["f", "f2"] {
            state
                .objects
                .push(json!({"kind":"Conduit","id":format!("v-inspect-{target}"),
                "name":format!("v-inspect-{target}"),"local_name":format!("inspect-{target}"),
                "consumer":"v","target":target,"open":true,"state":"ready",
                "operations":[c.qualified("inspect.v1")]}));
        }
        let signed = json!({"kind":"Agent","name":"signed"});
        let resource = json!({"kind":"Resource","name":"x"});
        let evidence = evidence();
        let view = access::PublicationView::new(&state, &c);
        for (sender, tool, subject) in [
            ("root", "f.sign-in.v1", &signed),
            ("root", "p.sign-in.v1", &resource),
            ("v", "f.sign-in.v1", &signed),
        ] {
            let request = request(sender, tool, json!({"instance":subject}));
            let verdict = access::evaluate(&state, &view, &request, &c, false).unwrap();
            assert_eq!(verdict.reason, None, "{sender} {tool}");
            let mut route = verdict.route;
            route["tool"] = json!(request.tool);
            assert!(
                crate::sign_in::preflight(&state, &route, &request, &evidence.time, &c).is_ok(),
                "{sender} {tool}"
            );
        }

        let missing = json!({"kind":"Agent","name":"nobody"});
        for (state, sender, tool, subject, serving, rule) in [
            (&state, "v", "f2.sign-in.v1", &signed, None, "ACC-033"),
            (&state, "s2", "f.sign-in.v1", &signed, None, "PND-010"),
            (&state, "v", "p.sign-in.v1", &resource, None, "ACC-033"),
            (
                &state,
                "v",
                "f.sign-in.v1",
                &signed,
                Some("call"),
                "PND-010",
            ),
            (
                &state,
                "v",
                "f2.sign-in.v1",
                &missing,
                Some("call"),
                "PND-010",
            ),
            (&state, "v", "f.sign-in.v1", &missing, None, "ACC-033"),
            (&unscoped, "v", "f.sign-in.v1", &signed, None, "ACC-033"),
        ] {
            let mut request = request(sender, tool, json!({"instance":subject}));
            request.serving = serving.map(str::to_owned);
            let mut t = transition(state.clone());
            access::decide(&mut t, &evidence, &request, &c).unwrap();
            assert_eq!(t.reply["reason"], "refused", "{sender} {tool}: {}", t.reply);
            let diagnostics = t.reply["diagnostics"].as_array().unwrap();
            assert_eq!(diagnostics.len(), 1);
            let diagnostic = &diagnostics[0];
            assert_eq!(diagnostic["ruleId"], rule, "{sender} {tool}: {}", t.reply);
            let refusal = &diagnostic["properties"]["refusal"];
            assert_eq!(refusal["tier"], 1);
            for key in ["target", "conduits", "exists"] {
                assert!(refusal.get(key).is_none(), "{sender} {tool}: {refusal}");
            }
            let cause = if rule == "PND-010" {
                "Only an Interface acting on its own behalf may sign in with the required Conduit; a scoped Interface signs in only Agents of its scoped Fleets, and Resources remain root's; other consumers may not open a Conduit listing sign-in."
            } else {
                concat!(
                    "the named Agent's owning Fleet, from an Interface acting on its own behalf; ",
                    "a Resource only from root, through its owning Pool (PND-010)."
                )
            };
            assert!(
                diagnostic["message"]["text"]
                    .as_str()
                    .unwrap()
                    .ends_with(cause)
            );
            let reply = t.reply.to_string();
            for detail in [
                "signed-instance-id",
                "account",
                "in progress",
                "conflict",
                "invalid",
            ] {
                assert!(!reply.contains(detail), "{sender} {tool}: {reply}");
            }
        }
    }

    #[test]
    fn sign_in_rejects_a_resolved_instance_after_access() {
        let (state, c) = sign_in_fixture();
        let args = json!({"instance":{"kind":"Agent","id":"signed-instance-id"}});
        let reason = format!("{}reason", c.meta_prefix);
        for (sender, tool) in [("root", "f.sign-in.v1"), ("v", "f2.sign-in.v1")] {
            let mut request = request(sender, tool, args.clone());
            request.client_capabilities = Some(json!({"elicitation":{"url":{}}}));
            let mut t = transition(state.clone());
            crate::requests::invoke(&mut t, &evidence(), &request, None, &c).unwrap();
            if sender == "root" {
                assert_eq!(t.reply["_meta"][&reason], "invalid");
                assert_cause(&t.reply, "LIF-031", "Agent/signed-instance-id");
            } else {
                assert_eq!(t.reply["_meta"][&reason], "refused");
                assert_eq!(
                    t.reply["structuredContent"]["diagnostics"][0]["ruleId"],
                    "ACC-033"
                );
            }
            assert!(t.external_intents.is_empty());
            assert_eq!(t.state.invocations, state.invocations);
        }
    }

    #[test]
    fn sign_in_keyed_repeats_require_url_elicitation() {
        let (mut state, c) = sign_in_fixture();
        let operation = c.qualified("sign-in.v1");
        let hook = c.qualified("account-hook.v1");
        let outcome = format!("{}outcome", c.meta_prefix);
        let reason = format!("{}reason", c.meta_prefix);
        let fleet = item_mut(&mut state, "f");
        fleet["resolved_implementations"][&operation] = json!({"source":"engine","item":{}});
        let agent = item_mut(&mut state, "signed-instance-id");
        let mut environment = c.core_environment.clone();
        let contracts = &mut environment.scopes.get_mut("core").unwrap().contracts;
        contracts.get_mut("type.v1").unwrap()["spec"]["signIn"] = json!({"account":"/account"});
        contracts.insert(
            "account-hook.v1".into(),
            json!({"kind":"Operation",
            "metadata":{"name":"account-hook.v1"},"spec":{"outputSchema":{"type":"object",
                "required":["account"],"properties":{"account":{"type":["string","null"]}}}}}),
        );
        let indexes = json!({"core":contracts});
        agent["held"] =
            json!({"environment":environment,"indexes":indexes,"contexts":{},"scope":"core"});
        agent["config"] = json!({"account":"primary-account"});
        agent["resolved_implementations"][&hook] =
            json!({"source":"project","item":{"command":["test"]}});

        let args = json!({"instance":{"kind":"Agent","name":"signed"}});
        let without_url = request("root", "f.sign-in.v1", args);
        let mut with_url = without_url.clone();
        with_url.client_capabilities = Some(json!({"elicitation":{"url":{}}}));
        let invoke = |state: &State, request: &AccessRequest| {
            let mut t = transition(state.clone());
            let mut fresh = evidence();
            fresh.ids.retain(|id| !state.allocated_ids.contains(id));
            crate::requests::invoke(&mut t, &fresh, request, Some("same-key".into()), &c).unwrap();
            t
        };
        let pending = invoke(&state, &with_url);
        assert_eq!(pending.reply["pending"], true, "{}", pending.reply);
        assert_eq!(pending.external_intents.len(), 1);
        let mut fresh = evidence();
        fresh
            .ids
            .retain(|id| !pending.state.allocated_ids.contains(id));
        let ended = crate::step(
            &pending.state,
            crate::Input::Complete {
                evidence: fresh,
                invocation: pending.reply["hook"].as_str().unwrap().into(),
                attempt: 1,
                completion: crate::requests::Completion::Result {
                    result: json!({"isError":false,
                        "_meta":{&outcome:"succeeded"},
                "structuredContent":{"account":"primary-account"}}),
                },
            },
            &c,
        )
        .unwrap();
        assert_eq!(
            ended.reply["_meta"][&outcome], "succeeded",
            "{}",
            ended.reply
        );

        for state in [&pending.state, &ended.state] {
            let rejected = invoke(state, &without_url);
            let error = &rejected.reply["error"];
            assert_eq!(error["code"], -32021, "{}", rejected.reply);
            assert_eq!(error["data"][&reason], "invalid");
            assert_eq!(
                error["data"]["requiredCapabilities"],
                json!({"elicitation":{"url":{}}})
            );
            assert_eq!(error["data"]["diagnostics"][0]["ruleId"], "IFC-049");
            assert!(rejected.external_intents.is_empty());
            assert_eq!(rejected.state.invocations, state.invocations);
        }
        let replayed = invoke(&ended.state, &with_url);
        assert_eq!(
            replayed.reply["structuredContent"],
            json!({"account":"primary-account"})
        );
        assert!(replayed.external_intents.is_empty());
    }

    #[test]
    fn answer_refusals_disclose_only_the_authority_the_caller_may_inspect() {
        let (mut s, c) = inspector_fixture();
        let operation = c.qualified("answer.v1");
        item_mut(&mut s, "v-x")["operations"] = json!([operation]);
        inspection(&mut s, "p", &c);
        s.invocations.push(
            json!({"id":"setup","operation":c.qualified("set-up.v1"),"object":"x",
            "state":"failed","ended":{"time":"2026-11-09T12:00:00Z","outcome":"failed"}}),
        );
        let route = |state: &State, action| {
            access_route(
                state,
                &request(
                    "v",
                    "x.answer.v1",
                    json!({"action":action,"answer":"dismissed"}),
                ),
                &c,
            )
        };
        let wrong_target = refusal(&s, &c, &route(&s, "setup"), "ACC-033");
        assert_eq!(wrong_target["tier"], 1);
        assert!(wrong_target.get("target").is_none());

        inspection(&mut s, "management", &c);
        let wrong_target = refusal(&s, &c, &route(&s, "setup"), "ACC-033");
        assert_eq!(wrong_target["tier"], 2);
        assert_eq!(wrong_target["target"]["id"], "p");
        assert_eq!(wrong_target["conduits"][0]["as"], "p");

        // Through a closed Conduit whose kept target, x, is not the authorizing one, p.
        let mut closed_state = s.clone();
        closed_state.objects.retain(|o| o["id"] != "v-x");
        let mut closed = stone(&closed_state, "answer", "v", "x", "2026-11-11T13:00:00Z");
        closed["localName"] = json!("x");
        closed["operations"] = json!([operation]);
        closed_state.tombstones.insert("answer".into(), closed);
        let closed_route = route(&closed_state, "setup");
        assert_eq!(closed_route["conduit"], Value::Null);
        let wrong_target = refusal(&closed_state, &c, &closed_route, "ACC-033");
        assert_eq!(wrong_target["tier"], 2);
        assert_eq!(wrong_target["target"]["id"], "p");
        assert_eq!(wrong_target["conduits"][0]["as"], "p");
        item_mut(&mut closed_state, "v-p")["open"] = json!(false);
        let ineligible = refusal(&closed_state, &c, &route(&closed_state, "setup"), "PND-009");
        assert_eq!(ineligible["tier"], 1);
        assert_eq!(
            ineligible,
            refusal(
                &closed_state,
                &c,
                &route(&closed_state, "missing"),
                "PND-009"
            )
        );

        item_mut(&mut s, "v-p")["open"] = json!(false);
        let ineligible = refusal(&s, &c, &route(&s, "setup"), "PND-009");
        assert_eq!(ineligible["tier"], 1);
        assert_eq!(
            ineligible,
            refusal(&s, &c, &route(&s, "missing"), "PND-009")
        );

        item_mut(&mut s, "root-management")["operations"] = json!([operation]);
        let root = |action| {
            access_route(
                &s,
                &request("root", "management.answer.v1", json!({"action":action})),
                &c,
            )
        };
        let mut serving = root("setup");
        serving["serving"] = json!("parent");
        let unknown = refusal(&s, &c, &root("missing"), "PND-009");
        assert_eq!(unknown["tier"], 2);
        assert_eq!(unknown["target"]["id"], "management");
        assert_eq!(unknown, refusal(&s, &c, &serving, "PND-009"));
    }

    fn submit_registration(s: &State, c: &Context, owner: &str) -> Transition {
        crate::step(
            s,
            crate::Input::Submit {
                evidence: evidence(),
                request: crate::Request {
                    sender: "v".into(),
                    caller: "v".into(),
                    conduit: "v-register".into(),
                    target: "management".into(),
                    operation: c.qualified("register.v1"),
                },
                documents: vec![json!({"kind":"Resource","from":"template","owner":owner})],
            },
            c,
        )
        .unwrap()
    }

    fn registration_fixture() -> (State, Context) {
        let (mut s, c) = inspector_fixture();
        s.objects.push(
            json!({"kind":"Conduit","id":"v-register","name":"v-register",
            "local_name":"register","consumer":"v","target":"management","open":true,
            "state":"ready","operations":[c.qualified("register.v1")]}),
        );
        inspection(&mut s, "sys", &c);
        (s, c)
    }

    #[test]
    fn tier_two_checks_management_and_the_live_registration_owner() {
        let (mut s, c) = registration_fixture();
        for tier in [1, 2] {
            expected_inspection(&s, &c, "v", "management", true);
            expected_inspection(&s, &c, "v", "p", tier == 2);
            let t = submit_registration(&s, &c, "p");
            let diagnostic = &t.reply["diagnostics"][0];
            assert_eq!(diagnostic["ruleId"], "ACC-033");
            assert_eq!(t.reply["reason"], "refused");
            assert!(
                t.records
                    .iter()
                    .filter(|r| r["data"].get("ruleId").is_some())
                    .all(|r| r["data"]["ruleId"] == "ACC-033")
            );
            assert_refusal_records(&t, &diagnostic["properties"]["refusal"], tier);
            inspection(&mut s, "management", &c);
        }
    }

    #[test]
    fn registration_owner_given_by_id_needs_inspection_of_that_pool() {
        let (mut s, c) = registration_fixture();
        item_mut(&mut s, "p")["name"] = json!("pool-p");
        expected_inspection(&s, &c, "v", "management", true);
        expected_inspection(&s, &c, "v", "p", false);
        let t = submit_registration(&s, &c, "p");
        let diagnostic = &t.reply["diagnostics"][0];
        assert_eq!(diagnostic["ruleId"], "ACC-033");
        assert_refusal_records(&t, &diagnostic["properties"]["refusal"], 1);
    }

    #[test]
    fn invoke_and_access_record_the_diagnostic_tier() {
        let (mut s, c) = inspector_fixture();
        inspection(&mut s, "p", &c);
        expected_inspection(&s, &c, "v", "x", true);
        let r = request("v", "x.ping.v1", json!({}));
        let t = crate::step(
            &s,
            crate::Input::Invoke {
                evidence: evidence(),
                request: r.clone(),
                key: None,
                archive: None,
            },
            &c,
        )
        .unwrap();
        assert_refusal_records(
            &t,
            &t.reply["error"]["data"]["diagnostics"][0]["properties"]["refusal"],
            2,
        );
        let mut t = transition(s);
        access::decide(&mut t, &evidence(), &r, &c).unwrap();
        assert_refusal_records(&t, &t.reply["diagnostics"][0]["properties"]["refusal"], 2);
    }

    fn stone(s: &State, id: &str, consumer: &str, target: &str, time: &str) -> Value {
        json!({"id":id,"object":{"kind":"Conduit","id":format!("closed-{id}"),"name":id},
            "consumer":inventory::endpoint(s, &json!(consumer)),"target":inventory::endpoint(s, &json!(target)),
            "time":time,"localName":"y","operations":["example.org/p#op"]})
    }

    #[test]
    fn closed_conduits_follow_live_ones_and_only_the_senders_are_used() {
        let (mut s, c) = inspector_fixture();
        let route = access_route(&s, &request("root", "y.op", json!({})), &c);
        expected_inspection(&s, &c, "root", "x", false);
        s.tombstones.insert(
            "a-later".into(),
            stone(&s, "a-later", "root", "x", "2026-11-11T13:00:00.001Z"),
        );
        let r = refusal(&s, &c, &route, "ACC-001");
        assert_eq!(r["target"], json!({"kind":"Resource","name":"x","id":"x"}));
        assert_eq!(r["exists"], false);
        assert_eq!(
            r["conduits"],
            json!([{"name":"a-later","as":"y","state":"closed","operations":["example.org/p#op"]}])
        );
        item_mut(&mut s, "v-x")["consumer"] = json!("root");
        s.tombstones.insert(
            "z-earlier".into(),
            stone(&s, "z-earlier", "root", "x", "2026-11-11T13:00:00Z"),
        );
        s.tombstones.insert(
            "foreign".into(),
            stone(&s, "foreign", "agent", "management", "2026-11-11T13:00:02Z"),
        );
        let r = refusal(&s, &c, &route, "ACC-001");
        assert_eq!(r["target"]["id"], "x");
        let names: Vec<_> = r["conduits"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| &p["name"])
            .collect();
        assert_eq!(json!(names), json!(["v-x", "z-earlier", "a-later"]));
        let by_id =
            json!({"consumer":"root","conduit":"closed-z-earlier","operation":"example.org/p#op"});
        assert_eq!(refusal(&s, &c, &by_id, "ACC-001"), {
            let mut expected = r;
            expected["named"] = json!({});
            expected["operation"] = json!("example.org/p#op");
            expected
        });
        let route = access_route(&s, &request("agent", "y.op", json!({})), &c);
        expected_inspection(&s, &c, "agent", "management", false);
        let r = refusal(&s, &c, &route, "ACC-001");
        assert_eq!(r["tier"], 1);
        assert_eq!(r["operation"], "y.op");
        assert_eq!(r["named"], json!({"tool":"y.op"}));
        for key in ["target", "exists", "conduits"] {
            assert!(r.get(key).is_none());
        }
    }

    #[test]
    fn closed_conduit_routes_core_operations_without_reconstructing_the_tool() {
        let (mut s, c) = inspector_fixture();
        for target in ["management", "p"] {
            inspection(&mut s, target, &c);
        }
        expected_inspection(&s, &c, "v", "management", false);
        expected_inspection(&s, &c, "v", "p", true);
        expected_inspection(&s, &c, "v", "x", true);
        let mut closed = stone(&s, "retire", "v", "management", "2026-11-11T13:00:00Z");
        closed["localName"] = json!("m");
        closed["operations"] = json!([c.qualified("retire.v1")]);
        s.tombstones.insert("retire".into(), closed);
        let route = access_route(
            &s,
            &request("v", "m.retire.v1", json!({"instance":"x"})),
            &c,
        );
        let r = refusal(&s, &c, &route, "ACC-001");
        assert_eq!(r["tier"], 2);
        assert_eq!(r["operation"], "m.retire.v1");
        assert!(r["named"].get("conduit").is_none());
        s.tombstones.get_mut("retire").unwrap()["operations"] = json!(["example.org/p#retire.v1"]);
        assert_eq!(refusal(&s, &c, &route, "ACC-001")["tier"], 1);
        s.tombstones.get_mut("retire").unwrap()["operations"] = json!([]);
        assert_eq!(refusal(&s, &c, &route, "ACC-001")["tier"], 2);
    }

    #[test]
    fn tier_two_omits_a_tombstone_outside_the_senders_inventory_visibility() {
        let (mut s, c) = inspector_fixture();
        inspection(&mut s, "p", &c);
        s.objects
            .push(json!({"kind":"Pool","name":"q","id":"q","state":"ready"}));
        expected_inspection(&s, &c, "v", "x", true);
        expected_inspection(&s, &c, "v", "q", false);
        let mut closed = stone(&s, "hidden", "v", "x", "2026-11-11T13:00:00Z");
        closed["target"]["owner"] = json!({"kind":"Pool","name":"q","id":"q"});
        s.tombstones.insert("hidden".into(), closed);
        let route = access_route(&s, &request("v", "x.op", json!({})), &c);
        let r = refusal(&s, &c, &route, "ACC-001");
        assert_eq!(r["tier"], 2);
        assert_eq!(
            r["conduits"],
            json!([{"name":"v-x","as":"x","state":"open","operations":[]}])
        );
        let hidden_route = access_route(&s, &request("v", "y.op", json!({})), &c);
        assert_eq!(refusal(&s, &c, &hidden_route, "ACC-001")["tier"], 1);
    }
    fn verdict(s: &State, c: &Context, r: &AccessRequest) -> String {
        let v = access::evaluate(s, &access::PublicationView::new(s, c), r, c, true).unwrap();
        assert_eq!(v.reason, Some("refused"));
        v.rules[0].into()
    }
    #[test]
    fn retired_sender_precedes_missing_tool() {
        let (mut s, c) = fixture();
        s.objects[1]["state"] = json!("retired");
        assert_eq!(
            verdict(&s, &c, &request("agent", "absent.list.v1", json!({}))),
            "ACC-039"
        );
    }
    #[test]
    fn secret_on_presented_conduit_is_sec007() {
        let (s, c) = fixture();
        assert_eq!(
            verdict(
                &s,
                &c,
                &request("agent", "management.get-secret.v1", json!({}))
            ),
            "SEC-007"
        );
    }
    #[test]
    fn secret_on_unready_conduit_is_acc001() {
        let (mut s, c) = fixture();
        s.objects
            .iter_mut()
            .find(|o| o["id"] == "agent-management")
            .unwrap()["state"] = json!("registered");
        assert_eq!(
            verdict(
                &s,
                &c,
                &request("agent", "management.get-secret.v1", json!({}))
            ),
            "ACC-001"
        );
    }
    #[test]
    fn reserved_operation_precedes_routing() {
        let (mut s, c) = fixture();
        s.objects[3]["state"] = json!("retired");
        assert_eq!(
            verdict(
                &s,
                &c,
                &request("agent", "management.reserved.v1", json!({}))
            ),
            "ACC-032"
        );
    }
    #[test]
    fn agent_registration_precedes_routing() {
        let (s, c) = fixture();
        assert_eq!(
            verdict(
                &s,
                &c,
                &request(
                    "agent",
                    "access.register.v1",
                    json!({"documents":[{"kind":"Agent","from":"x"}]})
                )
            ),
            "ACC-057"
        );
    }
    #[test]
    fn bound_precedes_root_consumer_and_secret_open_checks() {
        let (mut s, c) = fixture();
        s.objects
            .iter_mut()
            .find(|o| o["id"] == "agent-access")
            .unwrap()["requirements"] = json!({"limit":{"requirement":c.qualified("bound.v1"),"arguments":{"targets":[],"pools":[],"operations":[]}}});
        s.objects.push(json!({"kind":"Conduit","id":"named","name":"named","consumer":"root","target":"management","state":"registered","open":false,"operations":[c.qualified("get-secret.v1")]}));
        let r = request("agent", "access.open.v1", json!({"conduit":"named"}));
        assert_eq!(verdict(&s, &c, &r), "ACC-029");
        s.objects.last_mut().unwrap()["consumer"] = json!("agent");
        assert_eq!(verdict(&s, &c, &r), "ACC-029");
    }
    #[test]
    fn registration_owner_authority_precedes_root_consumer() {
        let (s, c) = fixture();
        assert_eq!(
            verdict(
                &s,
                &c,
                &request(
                    "agent",
                    "management.register.v1",
                    json!({"documents":[{"kind":"Resource","from":"x","owner":"owners"},{"kind":"Conduit","consumer":"root"}]})
                )
            ),
            "ACC-033"
        );
    }
    #[test]
    fn registration_boundary_orders_acc001_acc057_acc033() {
        let (mut s, c) = fixture();
        s.objects.push(
            json!({"kind":"Fleet","id":"fleet-owners","name":"fleet-owners","state":"ready"}),
        );
        let req = crate::Request {
            sender: "agent".into(),
            caller: "agent".into(),
            conduit: "agent-management".into(),
            target: "management".into(),
            operation: c.qualified("register.v1"),
        };
        for (permitted, kind, want) in [
            (false, "Agent", "ACC-001"),
            (true, "Agent", "ACC-057"),
            (true, "Resource", "ACC-033"),
        ] {
            let mut t = transition(s.clone());
            if !permitted {
                t.state
                    .objects
                    .iter_mut()
                    .find(|o| o["id"] == req.conduit)
                    .unwrap()["open"] = json!(false);
            }
            let empty = crate::packages::Project {
                manifest: None,
                documents: vec![],
                directory: None,
            };
            let project = crate::packages::Project {
                documents: vec![
                    json!({"kind":kind,"from":"x","owner":if kind == "Agent" {"fleet-owners"} else {"owners"}}),
                ],
                ..empty.clone()
            };
            assert!(!t.state.objects.iter().any(|path| crate::permits(
                path,
                &req.sender,
                &project.documents[0]["owner"],
                &req.operation,
            )));
            crate::registration::register(&mut t, vec![], empty, project, &evidence(), &req, &c)
                .unwrap();
            assert_eq!(t.reply["diagnostics"][0]["ruleId"], want);
            assert!(
                t.records
                    .iter()
                    .all(|r| r["data"]["ruleId"] == want && r["data"]["refusalTier"] == 1)
            );
        }
    }
    #[test]
    fn ids_request_missing_authority_precedes_lif034() {
        let (mut s, c) = fixture();
        s.objects[5]["first_start"] = json!(true);
        let req = crate::Request {
            sender: "agent".into(),
            caller: "agent".into(),
            conduit: "missing".into(),
            target: "management".into(),
            operation: c.qualified("retire.v1"),
        };
        let t = crate::step(
            &s,
            crate::Input::Request {
                evidence: evidence(),
                request: req,
                object: "owners".into(),
            },
            &c,
        )
        .unwrap();
        assert_eq!(t.reply["diagnostics"][0]["ruleId"], "ACC-001");
    }
    #[test]
    fn tiers_and_ids_conduit_follow_sender() {
        let (s, c) = fixture();
        for (sender, want) in [
            (Some("root"), 2),
            (Some("agent"), 1),
            (Some("other"), 1),
            (None, 1),
        ] {
            let route = json!({"consumer":sender,"conduit":sender.map(|s|format!("{s}-management")),"operation":c.qualified("list.v1"),"arguments":{}});
            let d = diagnostic(&s, &route, &c, "refused", "ACC-001").unwrap();
            let r = &d["properties"]["refusal"];
            assert_eq!(r["tier"], want);
            if want == 1 {
                for k in ["target", "exists", "conduits"] {
                    assert!(r.get(k).is_none());
                }
            } else {
                assert_eq!(r["target"]["id"], "management");
                assert_eq!(r["conduits"][0]["as"], "management");
            }
        }
    }
    #[test]
    fn operation_identification_ignores_open_and_operation_list() {
        let (mut s, c) = fixture();
        let path = s
            .objects
            .iter_mut()
            .find(|o| o["id"] == "agent-management")
            .unwrap();
        path["open"] = json!(false);
        path["operations"] = json!([]);
        assert_eq!(
            access_route(&s, &request("agent", "management.list.v1", json!({})), &c)["operation"],
            c.qualified("list.v1")
        );
        for tool in ["absent.list.v1", "management.missing.v1"] {
            assert_eq!(
                access_route(&s, &request("agent", tool, json!({})), &c)["operation"],
                tool
            );
        }
    }
    /// A package's operation may share an export name with the core's
    /// (spec/interface/implementation.yaml): the refusal names the package's.
    #[test]
    fn operation_identification_names_the_package_operation_on_a_name_collision() {
        let (mut s, c) = fixture();
        let mut env: schemas::Environment =
            serde_json::from_value(s.objects[0]["schema_environment"].clone()).unwrap();
        let mut project = env.scopes["core"].clone();
        project.address = "example.org/p".into();
        project
            .contracts
            .retain(|name, _| name == "list.v1" || name == "type.v1");
        project.contracts.get_mut("list.v1").unwrap()["spec"]["reservedToRoot"] = json!(true);
        env.scopes.insert("project".into(), project);
        s.objects.push(json!({"kind":"Resource","id":"files","name":"files","from":c.qualified("files"),"type":c.qualified("type.v1"),"state":"ready","config":{},"schema_environment":env,"contract_scope":"project"}));
        s.objects.push(json!({"kind":"Conduit","id":"agent-files","name":"agent-files","consumer":"agent","target":"files","local_name":"files","state":"ready","open":true,"operations":["example.org/p#list.v1"]}));
        let want = json!("example.org/p#list.v1");
        let r = request("agent", "files.list.v1", json!({}));
        assert_eq!(access_route(&s, &r, &c)["operation"], want);
        // access::decide, refused after selection found the Conduit's operation.
        let mut t = transition(s.clone());
        access::decide(&mut t, &evidence(), &r, &c).unwrap();
        assert_eq!(t.reply["reason"], "refused", "{}", t.reply);
        assert_eq!(
            t.reply["diagnostics"][0]["ruleId"], "ACC-032",
            "{}",
            t.reply
        );
        assert_eq!(
            t.reply["diagnostics"][0]["properties"]["refusal"]["operation"],
            want
        );
        // A refusal before selection: the closed Conduit lists no operation, so the spelling
        // resolves in the target's own contract scope.
        let path = s
            .objects
            .iter_mut()
            .find(|o| o["id"] == "agent-files")
            .unwrap();
        path["open"] = json!(false);
        path["operations"] = json!([]);
        assert_eq!(access_route(&s, &r, &c)["operation"], want);
        let result = crate::step(
            &s,
            crate::Input::Invoke {
                evidence: evidence(),
                key: None,
                archive: None,
                request: r,
            },
            &c,
        )
        .unwrap();
        let d = &result.reply["error"]["data"]["diagnostics"][0];
        assert_eq!(d["ruleId"], "ACC-001", "{}", result.reply);
        assert_eq!(d["properties"]["refusal"]["operation"], want);
    }
    #[test]
    fn operation_identification_names_a_dependency_operation_the_conduit_does_not_list() {
        let (mut s, c) = fixture();
        let mut env: schemas::Environment =
            serde_json::from_value(s.objects[0]["schema_environment"].clone()).unwrap();
        let mut dep = env.scopes["core"].clone();
        dep.address = "example.org/dep".into();
        let fetch = dep.contracts["list.v1"].clone();
        dep.contracts.clear();
        dep.contracts.insert("fetch.v1".into(), fetch);
        dep.exports = Some(vec!["fetch.v1".into()]);
        let mut adapter = env.scopes["core"].clone();
        adapter.address = "example.org/adapter".into();
        adapter.contracts.retain(|name, _| name == "type.v1");
        adapter.contracts.insert(
            "cap.v1".into(),
            json!({"kind":"Capability","spec":{"operations":["example.org/dep#fetch.v1"]}}),
        );
        adapter
            .manifest
            .insert("example.org/dep".into(), "dep".into());
        env.scopes.insert("dep".into(), dep);
        env.scopes.insert("adapter".into(), adapter);
        let target = json!({"schema_environment":env,"contract_scope":"adapter"});
        assert!(schemas::operation(&target, "example.org/dep#fetch.v1", &c.core_package).is_ok());
        s.objects.push(json!({"kind":"Resource","id":"files","name":"files","from":c.qualified("files"),"type":c.qualified("type.v1"),"state":"ready","config":{},"schema_environment":env,"contract_scope":"adapter"}));
        s.objects.push(json!({"kind":"Conduit","id":"agent-files","name":"agent-files","consumer":"agent","target":"files","local_name":"files","state":"ready","open":false,"capability":"example.org/adapter#cap.v1","operations":[]}));
        let r = request("agent", "files.fetch.v1", json!({}));
        assert_eq!(
            access_route(&s, &r, &c)["operation"],
            "example.org/dep#fetch.v1"
        );
    }
    #[test]
    fn named_uses_sender_on_onward_calls_and_keeps_object_order() {
        let (s, c) = fixture();
        let mut r = request(
            "agent",
            "management.list.v1",
            json!({"node":"n","conduit":"c","instance":"i","object":"o"}),
        );
        r.serving = Some("root-call".into());
        let route = access_route(&s, &r, &c);
        assert_eq!(route["conduit"], "agent-management");
        let d = diagnostic(&s, &route, &c, "refused", "ACC-001").unwrap();
        let named = &d["properties"]["refusal"]["named"];
        assert_eq!(named["objects"], json!(["o", "i", "c", "n"]));
        assert_eq!(named["conduit"], "management");
        let d = diagnostic(
            &s,
            &access_route(&s, &request("agent", "absent.list.v1", json!({})), &c),
            &c,
            "refused",
            "ACC-001",
        )
        .unwrap();
        assert!(d["properties"]["refusal"]["named"].get("conduit").is_none());
    }
    #[test]
    fn acc033_target_forms_do_not_resolve_hidden_owner_names() {
        let (mut s, c) = fixture();
        s.objects
            .push(json!({"kind":"Resource","id":"resource","name":"r","owner":"owners"}));
        s.objects
            .push(json!({"kind":"Agent","id":"peer","name":"peer","owner":"owners"}));
        for (op, args, target, want) in [
            ("register.v1", json!({}), "management", "management"),
            ("open.v1", json!({}), "access", "access"),
            (
                "retire-with-resources.v1",
                json!({"object":{"kind":"Pool","name":"as-written"}}),
                "owners",
                "the named Pool as-written",
            ),
            (
                "retire.v1",
                json!({"instance":{"kind":"Resource","name":"r"}}),
                "management",
                "the owning Pool of the named Resource",
            ),
            (
                "retire.v1",
                json!({"instance":{"kind":"Agent","name":"peer"}}),
                "management",
                "the owning Fleet of the named Agent",
            ),
            (
                "register.v1",
                json!({"documents":[{"kind":"Resource","from":"x","owner":"owners"}]}),
                "management",
                "management and the owning Pool owners",
            ),
            (
                "register.v1",
                json!({"documents":[{"kind":"Agent","from":"x","owner":{"kind":"Fleet","name":"owners"}}]}),
                "management",
                "management",
            ),
            (
                "control.v1",
                json!({}),
                "owners",
                "the named instance itself",
            ),
        ] {
            let route = json!({"consumer":"agent","operation":c.qualified(op),"arguments":args,"target":target});
            assert_eq!(acc033_target(&s, &route, &c), want);
        }
        s.objects[5]["kind"] = json!("Fleet");
        let route = json!({"consumer":"agent","operation":c.qualified("register.v1"),"arguments":{"documents":[{"kind":"Agent","from":"x","owner":"owners"}]}});
        assert_eq!(
            acc033_target(&s, &route, &c),
            "management and the owning Fleet owners"
        );
    }
    #[test]
    fn setup_and_retirement_keep_owner_requirement_when_owner_is_retired() {
        let (mut state, c) = fixture();
        for (kind, family) in [("Resource", "Pool"), ("Agent", "Fleet")] {
            state.objects.push(
                json!({"kind":family,"id":"retired-owner","name":"hidden-owner","state":"retired"}),
            );
            state
                .objects
                .push(json!({"kind":kind,"id":"member","name":"member","owner":"retired-owner"}));
            for op in ["set-up.v1", "retire.v1"] {
                let args = json!({"instance":{"kind":kind,"name":"member"}});
                let r = request("root", &format!("management.{op}"), args);
                assert_eq!(verdict(&state, &c, &r), "ACC-033");
                let d = diagnostic(
                    &state,
                    &access_route(&state, &r, &c),
                    &c,
                    "refused",
                    "ACC-033",
                )
                .unwrap();
                assert_eq!(
                    d["properties"]["refusal"]["cause"],
                    format!(
                        "The request needs an open Conduit listing the operation to the owning {family} of the named {kind}."
                    )
                );
                assert!(
                    !d["message"]["text"]
                        .as_str()
                        .unwrap()
                        .contains("hidden-owner")
                );
            }
            let route = json!({"operation":c.qualified("inspect.v1"),"arguments":{"object":{"kind":kind,"name":"member"}}});
            assert_eq!(acc033_target(&state, &route, &c), "management");
            state.objects.pop();
            state.objects.pop();
        }
    }

    #[test]
    fn tools_list_refusal_has_no_tool_and_tier_one() {
        let (s, c) = fixture();
        let mut t = transition(s);
        access::list(&mut t, &Identity::None, &c).unwrap();
        let r = &t.reply["diagnostics"][0]["properties"]["refusal"];
        assert_eq!(r["operation"], "tools/list");
        assert_eq!(r["named"], json!({}));
        assert_eq!(r["tier"], 1);
    }
    #[test]
    fn tables_are_explicit_and_selected_by_reason() {
        for table in [REFUSAL_CAUSES, REASON_CAUSES] {
            for (_, text) in table {
                assert!(!text.is_empty() && text.ends_with('.'));
            }
        }
        assert_ne!(
            cause("refused", "IFC-026").unwrap(),
            cause("unavailable", "IFC-026").unwrap()
        );
        assert_eq!(
            cause("refused", "IFC-026").unwrap(),
            REFUSAL_CAUSES
                .iter()
                .find(|(r, _)| *r == "IFC-026")
                .unwrap()
                .1
        );
        assert!(
            cause("invalid", "MISSING-000")
                .unwrap_err()
                .contains("MISSING-000")
        );
        let (s, c) = fixture();
        assert!(
            diagnostic(
                &s,
                &json!({"operation":"tools/list"}),
                &c,
                "invalid",
                "MISSING-000"
            )
            .is_err()
        );
    }
    #[test]
    fn error_text_uses_final_diagnostics_and_keeps_plain_content() {
        let (_, c) = fixture();
        for outcome in ["failed", "unknown"] {
            let mut reply = json!({"isError":true,"_meta":{"test/outcome":outcome},"content":[{"type":"text"},{"type":"text","text":"plain"}]});
            first_text(&mut reply, &c);
            assert_eq!(reply["content"][0]["text"], outcome);
            assert!(reply.get("structuredContent").is_none());
            reply["structuredContent"] =
                json!({"diagnostics":[{"level":"error","message":{"text":"final"}}]});
            first_text(&mut reply, &c);
            assert_eq!(reply["content"][0]["text"], format!("{outcome}: final"));
            assert_eq!(reply["content"][1]["text"], "plain");
        }
    }
    #[test]
    fn unreadable_authority_does_not_disclose_named_path_facts() {
        let (s, c) = fixture();
        let mut r = request("root", "management.list.v1", json!({}));
        r.authority_readable = false;
        let t = crate::step(
            &s,
            crate::Input::Invoke {
                evidence: evidence(),
                request: r,
                key: None,
                archive: None,
            },
            &c,
        )
        .unwrap();
        assert_eq!(
            t.reply["structuredContent"]["diagnostics"][0]["ruleId"],
            "ACC-037"
        );
        for record in &t.records {
            assert_eq!(record["data"]["ruleId"], "ACC-037");
            for key in ["conduit", "target", "operation"] {
                assert!(record["data"].get(key).is_none());
            }
        }
    }

    #[test]
    fn protocol_error_carries_message_diagnostic_and_record_rule() {
        let (s, c) = fixture();
        let t = crate::step(
            &s,
            crate::Input::Invoke {
                evidence: evidence(),
                request: request("root", "absent.list.v1", json!({})),
                key: None,
                archive: None,
            },
            &c,
        )
        .unwrap();
        assert_eq!(t.reply["error"]["code"], -32602);
        assert_eq!(t.reply["error"]["data"]["test/reason"], "refused");
        let d = &t.reply["error"]["data"]["diagnostics"][0];
        assert_eq!(d["ruleId"], "ACC-001");
        assert_eq!(
            t.reply["error"]["message"],
            format!(
                "No tool named absent.list.v1 is presented to this instance. {}",
                d["message"]["text"].as_str().unwrap()
            )
        );
        assert!(t.records.iter().all(|r| r["data"]["ruleId"] == "ACC-001"
            && r["data"]["refusalTier"] == 2
            && r["data"].get("conduits").is_none()));
        let (state, context) = fixture();
        let malformed = crate::step(
            &state,
            crate::Input::Invoke {
                evidence: evidence(),
                request: request("root", "nope.bad#spelling", json!({})),
                key: None,
                archive: None,
            },
            &context,
        )
        .unwrap();
        assert_eq!(
            malformed.reply["error"]["data"]["diagnostics"][0]["properties"]["refusal"]["operation"],
            "nope.bad#spelling"
        );
        assert!(
            malformed
                .records
                .iter()
                .all(|r| r["data"].get("operation").is_none())
        );
    }
    #[test]
    fn tier_two_exists_resolves_ids_and_typed_names() {
        let (mut state, c) = fixture();
        state
            .objects
            .push(json!({"kind":"Resource","id":"resource-id","name":"local"}));
        let base = json!({"consumer":"root","conduit":"root-management", "operation":c.qualified("retire.v1")});
        for (arguments, exists) in [
            (json!({"object":"resource-id"}), true),
            (json!({"instance":"resource-id"}), true),
            (json!({"object":"root-management"}), true),
            (json!({"conduit":"root-management"}), true),
            (json!({"conduit":"agent-management"}), true),
            (json!({"object":"local"}), false),
            (json!({"instance":"local"}), false),
            (json!({"conduit":"missing"}), false),
            (json!({"node":{"kind":"Resource","name":"local"}}), true),
            (json!({"node":"local"}), true),
            (json!({"node":"missing"}), false),
            (json!({"object":"resource-id","instance":"missing"}), false),
        ] {
            let mut route = base.clone();
            route["arguments"] = arguments.clone();
            let diagnostic = diagnostic(&state, &route, &c, "refused", "LIF-034").unwrap();
            let refusal = &diagnostic["properties"]["refusal"];
            assert_eq!(refusal["exists"], exists, "{arguments}");
            let objects: Vec<_> = ["object", "instance", "conduit", "node"]
                .into_iter()
                .filter_map(|key| arguments.get(key).cloned())
                .collect();
            assert_eq!(refusal["named"]["objects"], json!(objects));
        }
        // A Conduit's name can differ from its ID; only the conduit argument uses that name.
        state
            .objects
            .push(json!({"kind":"Conduit","id":"path-id","name":"bare-path"}));
        for (key, exists) in [
            ("conduit", true),
            ("object", false),
            ("instance", false),
            ("node", false),
        ] {
            let mut route = base.clone();
            route["arguments"] = json!({key:"bare-path"});
            assert_eq!(
                diagnostic(&state, &route, &c, "refused", "ACC-001").unwrap()["properties"]["refusal"]
                    ["exists"],
                exists
            );
        }
    }

    #[test]
    fn replacement_refusal_describes_named_pool_authority() {
        let (s, c) = fixture();
        let access = crate::step(
            &s,
            crate::Input::Access {
                evidence: evidence(),
                request: request(
                    "root",
                    "management.register-replacement.v1",
                    json!({"instance":{"kind":"Pool","name":"owners"},"nested":"entry"}),
                ),
            },
            &c,
        )
        .unwrap();
        assert_eq!(
            access.reply["diagnostics"][0]["properties"]["refusal"]["cause"],
            "The request needs an open Conduit listing the operation to the named Pool owners."
        );
        let result = crate::step(
            &s,
            crate::Input::Invoke {
                evidence: evidence(),
                key: None,
                archive: None,
                request: request(
                    "root",
                    "management.register-replacement.v1",
                    json!({"instance":{"kind":"Pool","name":"owners"},"nested":"entry"}),
                ),
            },
            &c,
        )
        .unwrap();
        let diagnostic = &result.reply["structuredContent"]["diagnostics"][0];
        assert_eq!(diagnostic["ruleId"], "ACC-033");
        assert_eq!(
            diagnostic["properties"]["refusal"]["cause"],
            "The request needs an open Conduit listing the operation to the named Pool owners."
        );
    }

    #[test]
    fn reconciliation_description_follows_authorizing_target_and_retired_fallback() {
        let (mut state, c) = fixture();
        state
            .objects
            .push(json!({"kind":"Resource","id":"resource","name":"member","owner":"owners"}));
        state
            .objects
            .push(json!({"kind":"Fleet","id":"fleet","name":"hidden-fleet","state":"ready"}));
        state
            .objects
            .push(json!({"kind":"Agent","id":"peer","name":"peer","owner":"fleet"}));
        for (invocation, want) in [
            (
                json!({"target":"resource"}),
                "the owning Pool of the named Resource",
            ),
            (
                json!({"target":"peer"}),
                "the owning Fleet of the named Agent",
            ),
            (
                json!({"management":true,"object":"resource","target":"owners"}),
                "the owning Pool of the named Resource",
            ),
            (
                json!({"management":true,"object":"peer","target":"fleet"}),
                "the owning Fleet of the named Agent",
            ),
            (
                json!({"management":true,"target":"management"}),
                "management",
            ),
            (json!({"management":true,"target":"access"}), "access"),
            (
                json!({"management":true,"object":"missing","target":"owners"}),
                "management",
            ),
            (json!({"target":"other"}), "management"),
        ] {
            let mut invocation = invocation;
            invocation["id"] = json!("prior");
            state.invocations = vec![invocation.clone()];
            let route = access_route(
                &state,
                &request(
                    "root",
                    "management.reconcile.v1",
                    json!({"invocation":"prior"}),
                ),
                &c,
            );
            assert_eq!(acc033_target(&state, &route, &c), want, "{invocation}");
            let result = crate::step(
                &state,
                crate::Input::Invoke {
                    evidence: evidence(),
                    key: None,
                    archive: None,
                    request: request(
                        "root",
                        "management.reconcile.v1",
                        json!({"invocation":"prior"}),
                    ),
                },
                &c,
            )
            .unwrap();
            if want != "management" {
                let diagnostic = &result.reply["structuredContent"]["diagnostics"][0];
                assert_eq!(diagnostic["ruleId"], "ACC-033", "{invocation}");
                assert!(
                    diagnostic["message"]["text"]
                        .as_str()
                        .unwrap()
                        .contains(want)
                );
                assert!(
                    !diagnostic["message"]["text"]
                        .as_str()
                        .unwrap()
                        .contains("hidden-fleet")
                );
            }
            for object in &mut state.objects {
                if object["id"] == "owners" || object["id"] == "fleet" {
                    object["state"] = json!("retired");
                }
            }
            if want != "access" {
                assert_eq!(
                    acc033_target(&state, &route, &c),
                    "management",
                    "{invocation}"
                );
            }
            for object in &mut state.objects {
                if object["id"] == "owners" || object["id"] == "fleet" {
                    object["state"] = json!("ready");
                }
            }
        }
    }

    #[test]
    fn replacing_live_pool_entry_returns_conflict_diagnostic_and_record() {
        let (mut state, c) = fixture();
        state
            .objects
            .iter_mut()
            .find(|o| o["id"] == "owners")
            .unwrap()["entries"] = json!({"entry":{}});
        state.objects.push(
            json!({"kind":"Resource","id":"entry-id","name":"entry","state":"ready",
            "nested":{"parent":"owners","slot":"entries","name":"entry"}}),
        );
        state.objects.push(json!({"kind":"Conduit","id":"root-owners","name":"root-owners","local_name":"owners",
            "consumer":"root","target":"owners","state":"ready","open":true,"operations":[c.qualified("register-replacement.v1")]}));
        let result = crate::step(
            &state,
            crate::Input::Invoke {
                evidence: evidence(),
                key: None,
                archive: None,
                request: request(
                    "root",
                    "management.register-replacement.v1",
                    json!({"instance":{"kind":"Pool","name":"owners"},"nested":"entry"}),
                ),
            },
            &c,
        )
        .unwrap();
        assert_eq!(result.reply["_meta"]["test/reason"], "conflict");
        assert_eq!(
            result.reply["structuredContent"]["diagnostics"][0]["ruleId"],
            "POOL-003"
        );
        assert!(
            result.reply["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains("POOL-003")
        );
        assert!(result.records.iter().any(|r| r["type"] == "request-ended"
            && r["data"]["reason"] == "conflict"
            && r["data"]["ruleId"] == "POOL-003"));
        assert!(result.external_intents.is_empty());
    }
}
