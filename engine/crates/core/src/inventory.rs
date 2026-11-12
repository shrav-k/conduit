//! LIF-013, LIF-040, LIF-041, ACC-044 and ACC-053: durable access state, apart from records.
use crate::{Context, Result, State, Transition, access, decision};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub(crate) fn object(o: &Value) -> Value {
    json!({"kind":o["kind"],"name":o["name"],"id":o["id"]})
}
pub(crate) fn endpoint(s: &State, r: &Value) -> Value {
    let Some(o) = access::object(s, r) else {
        return r.clone();
    };
    let mut end = object(o);
    if let Some(owner) = access::object(s, &o["owner"]) {
        end["owner"] = object(owner);
    }
    end
}
pub(crate) fn engine_made(o: &Value, c: &Context) -> bool {
    o["first_start"] == true
        || o["engine_made"] == true
        || o["type"] == c.qualified("node.v1")
        || matches!(
            o["provenance"]["cause"].as_str(),
            Some("default" | "first-start")
        )
}
/// One deterministic, domain-separated UUID per object. No clock or ambient ID source.
/// Keeping this allocation separate leaves runner-supplied invocation/record IDs unchanged.
pub(crate) fn make(
    t: &mut Transition,
    c: &Context,
    o: &Value,
    v: &Value,
    time: &str,
    cause: Value,
) -> Result<Option<Value>> {
    if engine_made(o, c) || o.get("spec").is_some() {
        return Ok(None);
    }
    if let Some(old) = t
        .state
        .tombstones
        .values()
        .find(|x| x["object"]["id"] == o["id"])
    {
        return Ok(Some(old["id"].clone()));
    }
    let digest = Sha256::digest(format!("conduit:tombstone:v1:{}", o["id"]));
    let mut bytes: [u8; 16] = digest[..16].try_into().unwrap();
    bytes[6] = (bytes[6] & 15) | 0x80;
    bytes[8] = (bytes[8] & 63) | 0x80;
    let h: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    let id = format!(
        "{}-{}-{}-{}-{}",
        &h[..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..]
    );
    if !t.state.allocated_ids.insert(id.clone()) {
        return Err("tombstone ID collision".into());
    }
    let mut stone = json!({"id":id,"object":object(o),"time":time,"cause":cause});
    if !v["caller"].is_null() {
        stone["by"] = v["caller"].clone();
    }
    if let Some(p) = o.get("provenance") {
        stone["provenance"] = p.clone();
    }
    if let Some(owner) = access::object(&t.state, &o["owner"]) {
        stone["owner"] = object(owner);
    }
    if o["kind"] == "Conduit" {
        stone["consumer"] = endpoint(&t.state, &o["consumer"]);
        stone["target"] = endpoint(&t.state, &o["target"]);
        stone["localName"] = o
            .get("local_name")
            .cloned()
            .unwrap_or_else(|| o["name"].clone());
        stone["capability"] = o["capability"].clone();
        stone["operations"] = o["operations"].clone();
        if let Some(b) = access::bound(o, c) {
            stone["bound"] = b.clone();
        }
    }
    t.state.tombstones.insert(id.clone(), stone);
    t.decisions.push(decision(
        "keep-tombstone",
        &["LIF-040", "ACC-044"],
        &[],
        json!({"id":id}),
        vec![],
    ));
    Ok(Some(json!(id)))
}
fn inspect_route(s: &State, actor: &Value, target: &Value, c: &Context) -> bool {
    s.objects.iter().any(|p| {
        access::open(p)
            && access::id(&p["consumer"]) == actor
            && access::id(&p["target"]) == &target["id"]
            && p["operations"]
                .as_array()
                .is_some_and(|ops| ops.contains(&json!(c.qualified("inspect.v1"))))
    })
}
fn management(s: &State, actor: &Value, c: &Context) -> bool {
    s.objects
        .iter()
        .any(|o| o["from"] == c.qualified("management") && inspect_route(s, actor, o, c))
}
fn inspect_kept(s: &State, actor: &Value, end: &Value, c: &Context) -> bool {
    if let Some(owner) = access::object_by_id(s, &end["owner"]).filter(|o| o["state"] != "retired")
    {
        inspect_route(s, actor, owner, c)
    } else if let Some(current) = access::object_by_id(s, &end["id"]) {
        access::may_inspect(s, actor, current, c).unwrap_or(false)
    } else {
        management(s, actor, c)
    }
}
pub(crate) fn visible(s: &State, actor: &Value, stone: &Value, c: &Context) -> bool {
    if stone["object"]["kind"] == "Conduit" {
        management(s, actor, c)
            || inspect_kept(s, actor, &stone["consumer"], c)
            || inspect_kept(s, actor, &stone["target"], c)
    } else if let Some(owner) =
        access::object_by_id(s, &stone["owner"]).filter(|o| o["state"] != "retired")
    {
        inspect_route(s, actor, owner, c)
    } else {
        management(s, actor, c)
    }
}
pub(crate) fn matching<'a>(s: &'a State, o: &Value) -> Option<&'a Value> {
    if o.get("spec").is_some() {
        return None;
    }
    let end_name = |r: &Value| {
        access::object(s, r)
            .map(object)
            .unwrap_or_else(|| r.clone())
    };
    let consumer = end_name(&o["consumer"]);
    let target = end_name(&o["target"]);
    s.tombstones.values().find(|x| {
        if o["kind"] == "Conduit" {
            x["object"]["kind"] == "Conduit"
                && x["consumer"]["kind"] == consumer["kind"]
                && x["consumer"]["name"] == consumer["name"]
                && (x["localName"]
                    == o.get("local_name")
                        .cloned()
                        .unwrap_or_else(|| o["name"].clone())
                    || x["target"]["kind"] == target["kind"]
                        && x["target"]["name"] == target["name"]
                        && x["capability"] == o["capability"])
        } else {
            x["object"]["kind"] == o["kind"] && x["object"]["name"] == o["name"]
        }
    })
}
pub(crate) fn diagnostic(s: &State, actor: &Value, stone: &Value, c: &Context) -> Value {
    let text = if visible(s, actor, stone, c) {
        format!(
            "LIF-041: tombstone {} holds {}; cause {}; lift.v1 removes it.",
            stone["id"], stone["object"], stone["cause"]
        )
    } else {
        "LIF-041: a tombstone prevents this operation.".into()
    };
    let mut diagnostic = json!({"ruleId":"LIF-041","level":"error","message":{"text":text}});
    if visible(s, actor, stone, c) {
        diagnostic["properties"]["tombstone"] = stone["id"].clone();
    }
    diagnostic
}
pub(crate) fn covered(s: &State, path: &Value, stone: &Value, c: &Context, opening: bool) -> bool {
    let Some(b) = access::bound(path, c) else {
        return true;
    };
    if stone["object"]["kind"] != "Conduit" {
        let mut target = stone["object"].clone();
        target["owner"] = stone["owner"].clone();
        return access::covers(b, &target);
    }
    if s.objects
        .iter()
        .any(|o| o["from"] == c.qualified("access") && o["id"] == stone["target"]["id"])
    {
        return stone
            .get("bound")
            .is_some_and(|new| access::subset(new, b, s))
            // ACC-055 incorporates ACC-030's additional operation-list condition for opens.
            && (!opening
                || stone["operations"].as_array().is_some_and(|ops| {
                    ops.iter().all(|op| {
                        path["operations"]
                            .as_array()
                            .is_some_and(|allowed| allowed.contains(op))
                    })
                }));
    }
    access::covers(b, &stone["target"])
        && stone["operations"]
            .as_array()
            .into_iter()
            .flatten()
            .all(|op| {
                b["arguments"]["operations"]
                    .as_array()
                    .is_some_and(|ops| ops.contains(op))
            })
}
pub(crate) fn lift(
    t: &mut Transition,
    path: &Value,
    actor: &Value,
    id: &Value,
    c: &Context,
) -> Value {
    let mut removed = vec![id.clone()];
    let mut descendants = vec![];
    let mut cursor = 0;
    while cursor < removed.len() {
        let parent = removed[cursor].clone();
        for stone in t.state.tombstones.values() {
            if stone["cause"]["tombstone"] == parent {
                descendants.push(stone["id"].clone());
                // Traverse even uncovered descendants to retain all caused tombstones in the result.
                if !removed.contains(&stone["id"]) {
                    removed.push(stone["id"].clone());
                }
            }
        }
        cursor += 1;
    }
    let to_remove: Vec<_> = removed
        .iter()
        .filter(|key| {
            *key == id
                || t.state
                    .tombstones
                    .get(key.as_str().unwrap_or(""))
                    .is_some_and(|stone| covered(&t.state, path, stone, c, false))
        })
        .cloned()
        .collect();
    t.state
        .tombstones
        .retain(|key, _| !to_remove.contains(&json!(key)));
    let remaining: Vec<_> = t
        .state
        .tombstones
        .values()
        .filter(|stone| descendants.contains(&stone["id"]) && visible(&t.state, actor, stone, c))
        .cloned()
        .collect();
    t.decisions.push(decision(
        "lift-covered-tombstones",
        &["ACC-053", "LIF-040", "REV-012"],
        &[],
        json!({}),
        vec![],
    ));
    json!({"remaining":remaining})
}
pub(crate) fn provenance(s: &State, v: &Value, c: &Context) -> Result<Value> {
    let parent = v
        .get("setup_parent")
        .and_then(|id| s.invocations.iter().find(|p| p["id"] == *id));
    let mut paths = vec![v["conduit"].clone()];
    if let Some(parent) = parent {
        paths.clear();
        for p in s.objects.iter().filter(|p| {
            access::open(p)
                && access::id(&p["consumer"]) == &parent["sender"]
                && p["operations"]
                    .as_array()
                    .is_some_and(|ops| ops.contains(&json!(c.qualified("open.v1"))))
        }) {
            let request = access::AccessRequest {
                identity: access::Identity::Instance {
                    id: parent["sender"]
                        .as_str()
                        .ok_or("setup sender absent")?
                        .into(),
                },
                tool: format!("{}.open.v1", p["local_name"].as_str().unwrap_or("")),
                arguments: v["arguments"].clone(),
                serving: parent["serving"].as_str().map(str::to_owned),
                readable: true,
                authority_readable: true,
            };
            // IFC-033: the call-local publication view, as every other access decision reads.
            let view = access::PublicationView::new(s, c);
            if access::evaluate(s, &view, &request, c, true)?
                .reason
                .is_none()
            {
                paths.push(p["id"].clone());
            }
        }
        paths.sort_by_key(Value::to_string);
    }
    let source = parent.unwrap_or(v);
    Ok(
        json!({"actor":source["caller"],"sentBy":source["sender"],"invocation":source["id"],"openedThrough":paths}),
    )
}
fn own_tools(o: &Value, c: &Context) -> Result<Value> {
    let published = crate::registration::publication_object(o, c)?;
    let (_, _, ty) = crate::schemas::operation(
        &published,
        o["type"].as_str().ok_or("Agent Type absent")?,
        &c.core_package,
    )?;
    let Some(pointer) = ty["spec"]["toolsDirectory"].as_str() else {
        return Ok(json!({"stated":false}));
    };
    let directory = o["config"].pointer(pointer).and_then(Value::as_str);
    let mut own = json!({"stated":true,"pointer":pointer,"directoryNamed":directory.is_some()});
    if let Some(directory) = directory {
        own["directory"] = json!(directory);
        own["enforcement"] = json!("neither");
    }
    Ok(own)
}
/// Compute each view from current Conduits; no independently mutable scope or administrator set.
pub(crate) fn read(s: &State, o: &Value, c: &Context) -> Result<Value> {
    let mut consumes = vec![];
    let mut target_of = vec![];
    let mut scope: BTreeMap<String, Value> = BTreeMap::new();
    let mut admins: BTreeMap<String, Value> = BTreeMap::new();
    for p in s
        .objects
        .iter()
        .filter(|p| p["kind"] == "Conduit" && p.get("spec").is_none())
    {
        let mut entry = json!({"conduit":object(p),"consumer":endpoint(s,&p["consumer"]),"target":endpoint(s,&p["target"]),"localName":p.get("local_name").unwrap_or(&p["name"]),"capability":p["capability"],"operations":p["operations"],"state":p["state"]});
        if let Some(provenance) = p.get("provenance") {
            entry["provenance"] = provenance.clone();
        }
        let consumed = access::id(&p["consumer"]) == &o["id"];
        let targeted = access::id(&p["target"]) == &o["id"];
        if consumed {
            if access::open(p)
                && let Some(target) =
                    access::object(s, &p["target"]).filter(|target| target["kind"] == "Agent")
            {
                let published = crate::registration::publication_object(target, c)?;
                for op in p["operations"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                {
                    if crate::schemas::operation(&published, op, &c.core_package)?.2["spec"]["startsModelWork"]
                        == true
                    {
                        entry["targetOwnTools"] = own_tools(target, c)?;
                        break;
                    }
                }
            }
            consumes.push(entry.clone());
        }
        if targeted {
            target_of.push(entry);
        }
        if !access::open(p) {
            continue;
        }
        if consumed
            && let Some(target) = access::object(s, &p["target"])
            && (o["kind"] == "Interface"
                || o["kind"] == "Agent" && access::id(&o["owner"]) == &target["id"])
            && ["Pool", "Fleet"].iter().any(|k| target["kind"] == *k)
            && target["name"] != "system"
            && p["operations"]
                .as_array()
                .is_some_and(|ops| ops.contains(&json!(c.qualified("inspect.v1"))))
        {
            let e = scope
                .entry(format!("{}:{}", target["kind"], target["name"]))
                .or_insert_with(|| json!({"target":object(target),"conduits":[]}));
            e["conduits"].as_array_mut().unwrap().push(object(p));
        }
        if targeted
            && o["kind"] == "Pool"
            && p["operations"].as_array().into_iter().flatten().any(|op| {
                [
                    "register.v1",
                    "register-replacement.v1",
                    "set-up.v1",
                    "retire.v1",
                    "retire-with-resources.v1",
                    "reconcile.v1",
                    "sign-in.v1",
                    "answer.v1",
                ]
                .iter()
                .any(|name| *op == c.qualified(name))
            })
        {
            let e = admins
                .entry(format!(
                    "{}:{}",
                    endpoint(s, &p["consumer"])["kind"],
                    endpoint(s, &p["consumer"])["name"]
                ))
                .or_insert_with(|| json!({"consumer":endpoint(s,&p["consumer"]),"conduits":[]}));
            e["conduits"].as_array_mut().unwrap().push(object(p));
        }
    }
    let stones: Vec<_> = s
        .tombstones
        .values()
        .filter(|x| {
            x["object"]["kind"] == "Conduit"
                && (x["consumer"]["id"] == o["id"] || x["target"]["id"] == o["id"])
        })
        .cloned()
        .collect();
    let mut inventory = json!({"consumes":consumes,"targetOf":target_of,"tombstones":stones});
    if o["kind"] == "Interface" || o["kind"] == "Agent" {
        inventory["scope"] = json!(scope.into_values().collect::<Vec<_>>());
    }
    if o["kind"] == "Pool" {
        inventory["administrators"] = json!(admins.into_values().collect::<Vec<_>>());
    }
    if o["kind"] == "Agent" {
        inventory["ownTools"] = own_tools(o, c)?;
    }
    Ok(inventory)
}
