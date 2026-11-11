//! Atomic registration. All allocation and machine evidence is supplied by the boundary.
use crate::packages::{self, Catalog, Project, Snapshot};
use crate::schemas::Environment;
use crate::{
    Allocation, Context, Evidence, Request, Result, Transition, authorized_path, decision,
    end_request, permits,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

type Indexes = BTreeMap<String, BTreeMap<String, Value>>;
#[derive(Clone, Serialize, Deserialize)]
struct Held {
    environment: Environment,
    indexes: Indexes,
    contexts: Value,
    scope: String,
}
#[derive(Debug)]
struct Fault {
    rule: &'static str,
    text: String,
}
type Checked<T> = std::result::Result<T, Fault>;
fn fault(rule: &'static str, text: impl Into<String>) -> Fault {
    Fault {
        rule,
        text: text.into(),
    }
}
fn items(v: &Value, key: &str) -> Vec<(String, Value)> {
    v.get(key)
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}
fn list(v: &Value, key: &str) -> Vec<String> {
    v.get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}
fn diagnostic(rule: &str, name: &str, text: &str, level: &str) -> Value {
    json!({"ruleId":rule,"level":level,"message":{"text":format!("{name}: {text}")},"locations":[{"logicalLocations":[{"fullyQualifiedName":format!("/objects/{}", name.replace('~', "~0").replace('/', "~1"))}]}]})
}
impl Held {
    fn select(&self, reference: &str, kind: &str, core: &str) -> Checked<(Self, Value, String)> {
        let own = format!("{}#", self.environment.scopes[&self.scope].address);
        let local = reference.strip_prefix(&own).map(|name| format!("#{name}"));
        let reference = local.as_deref().unwrap_or(reference);
        let (scope, name) = packages::resolve_name(
            &self.environment,
            &self.indexes,
            &self.scope,
            reference,
            kind,
            core,
        )
        .map_err(|(r, t)| fault(r, t))?;
        let doc = self.indexes[&scope][&name].clone();
        let qualified = format!("{}#{name}", self.environment.scopes[&scope].address);
        let mut held = self.clone();
        held.scope = scope;
        Ok((held, doc, qualified))
    }
    fn fill(&self, schema: &Value, value: &Value, core: &str) -> Result<Value> {
        self.environment
            .fill(&self.scope, schema, value, false, core)
    }
    fn package_list(
        &self,
        spec: &Value,
        kind: &str,
        implementations: &Value,
        core: &str,
    ) -> Result<Value> {
        let mut selections = vec![];
        packages::selections_document(
            &json!({"kind":kind,"spec":spec}),
            self.scope != "project",
            &mut selections,
        )?;
        let mut scopes = BTreeSet::new();
        for (reference, kind) in selections {
            if let Ok((held, _, _)) = self.select(&reference, &kind, core) {
                scopes.insert(held.scope);
            }
        }
        for implementation in implementations
            .as_object()
            .into_iter()
            .flat_map(|m| m.values())
        {
            if let Some(scope) = implementation["scope"].as_str() {
                scopes.insert(scope.into());
            }
            if implementation["source"] == "engine" {
                scopes.insert("core".into());
            }
        }
        let packages = scopes
            .into_iter()
            .map(|scope| {
                let context = &self.contexts[&scope];
                let mut item = json!({"address":context["address"]});
                for key in ["version", "digest"] {
                    if let Some(value) = context.get(key) {
                        item[key] = value.clone();
                    }
                }
                item
            })
            .collect::<Vec<_>>();
        Ok(json!(packages))
    }
    fn valid(&self, schema: &Value, value: &Value, core: &str) -> Result<bool> {
        self.environment.valid(&self.scope, schema, value, core)
    }
    fn valid_in(&self, root: &Value, schema: &Value, value: &Value, core: &str) -> Result<bool> {
        fn locate(root: &Value, wanted: &Value, at: String) -> Option<String> {
            if root == wanted {
                return Some(at);
            }
            match root {
                Value::Object(map) => map.iter().find_map(|(key, value)| {
                    locate(
                        value,
                        wanted,
                        format!("{at}/{}", key.replace('~', "~0").replace('/', "~1")),
                    )
                }),
                Value::Array(array) => array
                    .iter()
                    .enumerate()
                    .find_map(|(i, value)| locate(value, wanted, format!("{at}/{i}"))),
                _ => None,
            }
        }
        self.environment.valid_at(
            &self.scope,
            root,
            &locate(root, schema, String::new()).ok_or("schema branch absent from root")?,
            value,
            core,
        )
    }
}
fn implicit(spec: &mut Value, kind: &str) {
    if kind != "Conduit" && spec.get("config").is_none() {
        spec["config"] = json!({});
    }
    for key in ["conduits", "entries", "requirements"] {
        if let Some(map) = spec.get_mut(key).and_then(Value::as_object_mut) {
            for declaration in map.values_mut() {
                if declaration.get("arguments").is_none() {
                    declaration["arguments"] = json!({});
                }
            }
        }
    }
}
fn tokens(pointer: &str) -> Vec<String> {
    pointer
        .strip_prefix('/')
        .unwrap_or("")
        .split('/')
        .map(|t| t.replace("~1", "/").replace("~0", "~"))
        .collect()
}
fn destination(kind: &str, t: &[String]) -> bool {
    match t {
        [a, ..] if a == "config" => kind != "Conduit",
        [a] if a == "operations" => kind == "Conduit",
        [a, _, b, ..] if a == "conduits" => b == "arguments" || (b == "target" && t.len() == 3),
        [a, _, b, ..] if a == "entries" => {
            b == "arguments" || (["placement", "runtime"].contains(&b.as_str()) && t.len() == 3)
        }
        [a, _, b, ..] if a == "requirements" => kind == "Conduit" && b == "arguments",
        _ => false,
    }
}
fn check_destinations(spec: &Value, kind: &str) -> Checked<()> {
    let mut seen: Vec<(String, Vec<String>)> = vec![];
    for (input, value) in items(spec, "inputs") {
        for p in list(&value, "to") {
            let t = tokens(&p);
            if !destination(kind, &t) {
                return Err(fault(
                    "INS-012",
                    format!("{input}: forbidden destination {p}"),
                ));
            }
            if t.iter().any(|t| t == "-") {
                return Err(fault(
                    "OBJ-016",
                    format!("{input}: invalid destination {p}"),
                ));
            }
            let parent = p.rsplit_once('/').map(|(p, _)| p).unwrap_or("");
            if !spec
                .pointer(parent)
                .is_some_and(|p| p.is_object() || p.is_array())
            {
                return Err(fault("INS-011", format!("{input}: absent parent of {p}")));
            }
            if t.len() == 3
                && t[0] == "entries"
                && ["placement", "runtime"].contains(&t[2].as_str())
                && value.get("default").is_some()
            {
                return Err(fault(
                    "PLC-005",
                    format!("{input}: placement/runtime default"),
                ));
            }
            for (other, previous) in &seen {
                if previous.starts_with(&t) || t.starts_with(previous) {
                    return Err(fault(
                        "INS-010",
                        format!("{other} and {input}: overlapping destinations {previous:?}, {p}"),
                    ));
                }
            }
            seen.push((input.clone(), t));
        }
    }
    Ok(())
}
fn assign(spec: &mut Value, pointer: &str, value: Value) -> Checked<()> {
    let (parent, leaf) = pointer
        .rsplit_once('/')
        .ok_or_else(|| fault("INS-011", pointer))?;
    let leaf = leaf.replace("~1", "/").replace("~0", "~");
    let parent = spec
        .pointer_mut(parent)
        .ok_or_else(|| fault("INS-011", pointer))?;
    if let Some(map) = parent.as_object_mut() {
        map.insert(leaf, value);
        return Ok(());
    }
    if let Some(array) = parent.as_array_mut()
        && let Ok(i) = leaf.parse::<usize>()
        && i < array.len()
    {
        array[i] = value;
        return Ok(());
    }
    Err(fault("INS-011", pointer))
}

struct Builder<'a> {
    context: &'a Context,
    evidence: &'a Evidence,
    state: crate::State,
    allocation: Allocation<'a>,
    suffix: usize,
    objects: Vec<Value>,
    diagnostics: Vec<Value>,
    templates: Vec<Value>,
    reserved_names: BTreeSet<(String, String)>,
    snapshots: Vec<Snapshot>,
    trusted: bool,
}
impl Builder<'_> {
    fn template(
        &self,
        from: &Value,
        kind: &str,
        held: &Held,
        instantiating: bool,
    ) -> Checked<(Value, Held, Value)> {
        if let Some(id) = from.get("id") {
            if !self.trusted {
                return Err(fault("INS-002", "authored template ID"));
            }
            let template = self
                .state
                .objects
                .iter()
                .find(|o| o["id"] == *id && o["kind"] == kind && o.get("spec").is_some())
                .ok_or_else(|| fault("INS-002", "stored template no longer exists"))?;
            return self.template(
                &json!({"kind":kind,"name":template["name"]}),
                kind,
                held,
                instantiating,
            );
        }
        let spelling = from
            .as_str()
            .or_else(|| from["name"].as_str())
            .ok_or_else(|| fault("INS-002", "invalid from"))?;
        if let Some(explicit) = from.get("kind")
            && explicit != kind
        {
            return Err(fault("INS-002", "from has another family"));
        }
        if spelling.contains('#') && (!spelling.starts_with('#') || held.scope != "project") {
            let (h, doc, q) = held.select(
                spelling,
                &format!("Template:{kind}"),
                &self.context.core_package,
            )?;
            if self.trusted && instantiating && h.scope != "project" && h.scope != "core" {
                let package = &h.contexts[&h.scope];
                if !self.snapshots.iter().any(|s| {
                    package["address"] == s.address
                        && package["digest"] == s.digest
                        && package["version"]
                            .as_str()
                            .is_some_and(|v| packages::version_equal(v, &s.version))
                }) {
                    return Err(fault(
                        "PKG-007",
                        "the stored template package is no longer installed",
                    ));
                }
            }
            return Ok((doc, h, json!(q)));
        }
        let name = spelling.strip_prefix('#').unwrap_or(spelling);
        let doc = self
            .templates
            .iter()
            .chain(self.state.objects.iter())
            .find(|d| d["kind"] == kind && d["name"] == name)
            .ok_or_else(|| fault("INS-002", format!("no {kind} template {name}")))?;
        if doc.get("spec").is_none() {
            return Err(fault("INS-002", format!("{name} is an instance")));
        }
        // The definition's resolution context stays frozen, but a later catalog
        // must still contain the exact package identities that definition uses.
        for package in doc["packages"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|_| instantiating)
        {
            if package["address"] == self.context.core_package || package.get("version").is_none() {
                continue;
            }
            if !self.snapshots.iter().any(|s| {
                package["address"] == s.address
                    && packages::version_equal(
                        package["version"].as_str().unwrap_or(""),
                        &s.version,
                    )
                    && package["digest"] == s.digest
            }) {
                return Err(fault(
                    "PKG-007",
                    format!(
                        "required package {} version {} ({}) is not installed",
                        package["address"].as_str().unwrap_or(""),
                        package["version"].as_str().unwrap_or(""),
                        package["digest"].as_str().unwrap_or("")
                    ),
                ));
            }
        }
        let h = serde_json::from_value(doc["held"].clone())
            .map_err(|_| fault("INS-002", "template context missing"))?;
        Ok((
            json!({"kind":kind,"metadata":{"name":name},"spec":doc["spec"]}),
            h,
            json!({"kind":kind,"id":doc["id"]}),
        ))
    }
    fn allocate(&mut self) -> Result<(String, String)> {
        let revision = self
            .evidence
            .revisions
            .get(self.objects.len() + self.templates.len())
            .filter(|r| !r.is_empty())
            .ok_or("exhausted revisions")?
            .clone();
        Ok((self.allocation.next(&mut self.state)?, revision))
    }
    fn name(&mut self, kind: &str, template: &str) -> Result<String> {
        loop {
            let suffix = self
                .evidence
                .suffixes
                .get(self.suffix)
                .ok_or("exhausted generated suffixes")?;
            self.suffix += 1;
            if suffix.len() != 5
                || !suffix
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
            {
                return Err("invalid DL15 suffix".into());
            }
            let name = format!("{}-{suffix}", &template[..template.len().min(57)]);
            if !self.reserved_names.contains(&(kind.into(), name.clone()))
                && !self
                    .state
                    .objects
                    .iter()
                    .chain(&self.objects)
                    .chain(&self.templates)
                    .any(|o| o["kind"] == kind && o["name"] == name)
            {
                return Ok(name);
            }
        }
    }
    fn expand(
        &mut self,
        document: &Value,
        held: &Held,
        parent: Option<(&Value, &str, &str)>,
        depth: usize,
    ) -> Result<()> {
        if depth > 128 {
            self.diagnostics.push(diagnostic(
                "SUB-011",
                "nested instantiation",
                "DL10 nesting 128 exceeded",
                "error",
            ));
            return Ok(());
        }
        let kind = document["kind"].as_str().ok_or("instance has no family")?;
        let label = document["metadata"]["name"].as_str().unwrap_or("unnamed");
        let (template, h, from) = match self.template(&document["from"], kind, held, true) {
            Ok(t) => t,
            Err(f) => {
                self.diagnostics
                    .push(diagnostic(f.rule, label, &f.text, "error"));
                return Ok(());
            }
        };
        let mut spec = template["spec"].clone();
        implicit(&mut spec, kind);
        let protected = ["system", "node", "management", "access", "root"];
        if from.as_str().is_some_and(|s| {
            protected
                .iter()
                .any(|n| s == format!("{}#{n}", self.context.core_package))
        }) {
            self.diagnostics.push(diagnostic(
                "FST-006",
                label,
                "protected core template",
                "error",
            ));
            return Ok(());
        }
        let mut argument_values = json!({});
        let apply = (|| -> Checked<()> {
            for (name, _) in items(document, "arguments") {
                if ["owner", "placement", "runtime", "consumer", "target"].contains(&name.as_str())
                {
                    return Err(fault(
                        "OBJ-013",
                        format!("standard argument {name} under arguments"),
                    ));
                }
                if spec["inputs"].get(&name).is_none() {
                    return Err(fault("INS-003", format!("undeclared argument {name}")));
                }
            }
            for (name, input) in items(&spec, "inputs") {
                let value = document["arguments"]
                    .get(&name)
                    .or_else(|| input.get("default"));
                let Some(value) = value else {
                    if input["optional"] == true {
                        continue;
                    }
                    return Err(fault("INS-004", format!("missing input {name}")));
                };
                let value = h
                    .fill(&input["schema"], value, &self.context.core_package)
                    .map_err(|e| fault("INS-005", e))?;
                if let Some(failure) = h
                    .environment
                    .failure_at(
                        &h.scope,
                        &input["schema"],
                        "",
                        &value,
                        &self.context.core_package,
                    )
                    .map_err(|e| fault("INS-005", e))?
                {
                    return Err(fault("INS-005", format!("input {name}: {failure}")));
                }
                // Expand fixed-kind reference arguments before assigning to a multi-kind target.
                let value = expand_bare(
                    &h,
                    &input["schema"],
                    &input["schema"],
                    value,
                    &self.context.core_package,
                    0,
                )?;
                argument_values[&name] = value.clone();
                for p in list(&input, "to") {
                    assign(&mut spec, &p, value.clone())?;
                }
            }
            Ok(())
        })();
        if let Err(f) = apply {
            self.diagnostics
                .push(diagnostic(f.rule, label, &f.text, "error"));
            return Ok(());
        }
        let name = if let Some(name) = document["metadata"]["name"].as_str() {
            name.to_owned()
        } else {
            self.name(
                kind,
                template["metadata"]["name"]
                    .as_str()
                    .ok_or("template has no name")?,
            )?
        };
        let (id, revision) = self.allocate()?;
        let inputs = spec.get("inputs").cloned().unwrap_or(json!({}));
        let mut object = spec;
        object
            .as_object_mut()
            .ok_or("template spec is not a map")?
            .remove("inputs");
        object["kind"] = json!(kind);
        object["id"] = json!(id);
        object["revision"] = json!(revision);
        object["name"] = json!(name);
        object["from"] = from;
        object["state"] = json!("registered");
        object["metadata"] = document.get("metadata").cloned().unwrap_or(json!({}));
        object["metadata"]["name"] = json!(name);
        object["held"] = serde_json::to_value(&h).map_err(|e| e.to_string())?;
        object["_inputs"] = inputs;
        object["_argument_values"] = argument_values;
        for key in ["owner", "placement", "runtime", "consumer", "target"] {
            if let Some(v) = document.get(key) {
                object[key] = v.clone();
            }
        }
        if kind == "Conduit" {
            object["open"] = json!(false);
        }
        if let Some((owner, slot, local)) = parent {
            object[if slot == "entries" {
                "owner"
            } else {
                "consumer"
            }] = json!({"kind":owner["kind"],"id":owner["id"]});
            object["nested"] = json!({"parent":owner["id"],"slot":slot,"name":local});
            if slot == "entries" {
                object["entry_pool"] = owner["id"].clone();
            }
        }
        let position = self.objects.len();
        self.objects.push(object.clone());
        for (slot, family) in [("conduits", "Conduit"), ("entries", "Resource")] {
            for (local, mut nested) in items(&object, slot) {
                // Optional declarations are still instantiated; the mark changes setup failure only.
                nested["kind"] = json!(family);
                let before = self.objects.len();
                self.expand(&nested, &h, Some((&object, slot, &local)), depth + 1)?;
                if let Some(child) = self.objects.get(before) {
                    self.objects[position]["slots"][slot][&local] =
                        json!({"kind":family,"id":child["id"]});
                }
            }
        }
        Ok(())
    }
}

fn fixed_kind(name: &str) -> Option<&'static str> {
    match name {
        "pool-reference.v1" => Some("Pool"),
        "resource-reference.v1" | "secret-reference.v1" => Some("Resource"),
        "agent-reference.v1" => Some("Agent"),
        "interface-reference.v1" => Some("Interface"),
        "conduit-reference.v1" => Some("Conduit"),
        _ => None,
    }
}
fn expand_bare(
    h: &Held,
    root: &Value,
    schema: &Value,
    value: Value,
    core: &str,
    depth: usize,
) -> Checked<Value> {
    if depth > 128 {
        return Err(fault("SUB-011", "schema reference nesting 128"));
    }
    if let Some(r) = schema["$ref"].as_str() {
        if let Some(name) = r.strip_prefix(&format!("{core}#"))
            && let Some(kind) = fixed_kind(name)
            && name != "secret-reference.v1"
            && value.is_string()
        {
            return Ok(json!({"kind":kind,"name":value}));
        }
        let (scope, s) = h
            .environment
            .schema_reference(&h.scope, root, r, core)
            .map_err(|e| fault("SCH-015", e))?;
        let mut h = h.clone();
        h.scope = scope;
        return expand_bare(
            &h,
            if r.starts_with("#/") || r == "#" {
                root
            } else {
                &s
            },
            &s,
            value,
            core,
            depth + 1,
        );
    }
    Ok(value)
}

struct References<'a> {
    objects: &'a [Value],
    core: &'a str,
    unresolved: Option<&'a RefCell<Vec<Fault>>>,
    template_entries: Option<&'a BTreeSet<String>>,
}
// Schemas may govern one value through several applicable branches. Apply only
// the reference rewrites each branch made to the original value; never treat a
// rewrite by another branch as an authored ID or undo it with an unchanged value.
fn overlay(current: &mut Value, original: &Value, resolved: Value) {
    if original == &resolved {
        return;
    }
    match (current, original, resolved) {
        (Value::Object(current), Value::Object(original), Value::Object(resolved))
            if original.keys().eq(resolved.keys()) =>
        {
            for (key, value) in resolved {
                if let Some(current) = current.get_mut(&key) {
                    overlay(current, &original[&key], value);
                }
            }
        }
        (Value::Array(current), Value::Array(original), Value::Array(resolved))
            if current.len() == original.len() && original.len() == resolved.len() =>
        {
            for ((current, original), resolved) in current.iter_mut().zip(original).zip(resolved) {
                overlay(current, original, resolved);
            }
        }
        (current, _, resolved) => *current = resolved,
    }
}
impl References<'_> {
    fn resolve(
        &self,
        value: &Value,
        kind: Option<&str>,
        entry_pool: Option<&Value>,
        authored: bool,
    ) -> Checked<Value> {
        if value.get("id").is_some() {
            if authored {
                return Err(fault("OBJ-009", "authored resolved reference"));
            }
            return Ok(value.clone());
        }
        let kind = value["kind"].as_str().or(kind);
        if let Some(entry) = value["entry"].as_str()
            && value.get("name").is_none()
            && let Some(entries) = self.template_entries
        {
            return if entries.contains(entry) {
                Ok(value.clone())
            } else {
                Err(fault("POOL-005", format!("no entry {entry}")))
            };
        }
        let found = if let Some(entry) = value["entry"].as_str() {
            let pool = if value.get("name").is_some() {
                Some(
                    self.resolve(
                        &json!({"kind":"Pool","name":value["name"]}),
                        Some("Pool"),
                        None,
                        authored,
                    )?["id"]
                        .clone(),
                )
            } else {
                entry_pool.cloned()
            };
            let pool =
                pool.ok_or_else(|| fault("POOL-005", "entry reference outside a Pool entry"))?;
            self.objects.iter().find(|o| {
                o["kind"] == "Resource"
                    && o["state"] != "retired"
                    && o["nested"]["parent"] == pool
                    && o["nested"]["slot"] == "entries"
                    && o["nested"]["name"] == entry
            })
        } else {
            let name = value
                .as_str()
                .or_else(|| value["name"].as_str())
                .ok_or_else(|| fault("OBJ-006", "reference needs kind and name"))?;
            let kind = kind.ok_or_else(|| fault("OBJ-006", "reference needs kind"))?;
            self.objects
                .iter()
                .find(|o| o["kind"] == kind && o["name"] == name)
        }
        .ok_or_else(|| fault("OBJ-009", format!("unresolved reference {value}")))?;
        if found.get("spec").is_some() {
            return Err(fault(
                "OBJ-007",
                format!("reference {value} names a template"),
            ));
        }
        Ok(json!({"kind":found["kind"],"id":found["id"]}))
    }
    #[allow(clippy::too_many_arguments)]
    fn schema(
        &self,
        h: &Held,
        root: &Value,
        schema: &Value,
        value: &Value,
        pool: Option<&Value>,
        authored: bool,
        depth: usize,
    ) -> Checked<Value> {
        if depth > 128 {
            return Err(fault("SUB-011", "schema reference nesting 128"));
        }
        let mut result = value.clone();
        if let Some(r) = schema["$ref"].as_str() {
            if let Some(name) = r.strip_prefix(&format!("{}#", self.core))
                && name.ends_with("reference.v1")
                && !["qualified-reference.v1", "template-reference.v1"].contains(&name)
            {
                return match self.resolve(value, fixed_kind(name), pool, authored) {
                    Err(f)
                        if f.rule == "OBJ-009"
                            && f.text.contains("unresolved reference")
                            && self.unresolved.is_some() =>
                    {
                        self.unresolved.unwrap().borrow_mut().push(f);
                        Ok(value.clone())
                    }
                    result => result,
                };
            }
            let (scope, s) = h
                .environment
                .schema_reference(&h.scope, root, r, self.core)
                .map_err(|e| fault("SCH-015", e))?;
            let mut h = h.clone();
            h.scope = scope;
            result = self.schema(
                &h,
                if r == "#" || r.starts_with("#/") {
                    root
                } else {
                    &s
                },
                &s,
                value,
                pool,
                authored,
                depth + 1,
            )?;
        }
        for key in ["anyOf", "oneOf"] {
            for branch in schema[key].as_array().into_iter().flatten() {
                if h.valid_in(root, branch, value, self.core)
                    .map_err(|e| fault("SCH-015", e))?
                {
                    let resolved =
                        self.schema(h, root, branch, value, pool, authored, depth + 1)?;
                    overlay(&mut result, value, resolved);
                }
            }
        }
        if let Some(map) = result.as_object_mut() {
            for (key, sub) in items(schema, "properties") {
                if let Some(v) = value.get(&key) {
                    let resolved = self
                        .schema(h, root, &sub, v, pool, authored, depth + 1)
                        .map_err(|mut f| {
                            f.text = format!("/{key}: {}", f.text);
                            f
                        })?;
                    if let Some(current) = map.get_mut(&key) {
                        overlay(current, v, resolved);
                    }
                }
            }
            let mut matched = BTreeSet::new();
            for (pattern, sub) in items(schema, "patternProperties") {
                let pattern = conduit_loader::formats::ecma_regex(&pattern)
                    .map_err(|e| fault("SCH-016", e))?;
                for (key, current) in map.iter_mut() {
                    if pattern.find(key).is_some() {
                        matched.insert(key.clone());
                        if let Some(original) = value.get(key) {
                            let resolved =
                                self.schema(h, root, &sub, original, pool, authored, depth + 1)?;
                            overlay(current, original, resolved);
                        }
                    }
                }
            }
            if let Some(sub) = schema.get("additionalProperties").filter(|s| s.is_object()) {
                for (key, v) in map.iter_mut() {
                    if schema["properties"].get(key).is_none()
                        && !matched.contains(key)
                        && let Some(original) = value.get(key)
                    {
                        let resolved =
                            self.schema(h, root, sub, original, pool, authored, depth + 1)?;
                        overlay(v, original, resolved);
                    }
                }
            }
        }
        if let Some(array) = result.as_array_mut()
            && let Some(original) = value.as_array()
            && let Some(sub) = schema.get("items")
        {
            for (current, original) in array.iter_mut().zip(original) {
                let resolved = self.schema(h, root, sub, original, pool, authored, depth + 1)?;
                overlay(current, original, resolved);
            }
        }
        Ok(result)
    }
}

fn implementations(h: &Held, spec: &Value, kind: &str, core: &str) -> Checked<Value> {
    if kind == "Conduit" {
        let bounds: Vec<_> = items(spec, "requirements")
            .into_iter()
            .filter_map(|(name, r)| {
                h.select(r["requirement"].as_str().unwrap_or(""), "Requirement", core)
                    .ok()
                    .filter(|(_, _, q)| q == &format!("{core}#bound.v1"))
                    .map(|_| name)
            })
            .collect();
        if bounds.len() > 1 {
            return Err(fault(
                "CON-028",
                format!("bound applied more than once: {}", bounds.join(", ")),
            ));
        }
        let (ch, cap, _) = h.select(
            spec["capability"].as_str().unwrap_or(""),
            "Capability",
            core,
        )?;
        let members = list(&cap["spec"], "operations")
            .iter()
            .map(|op| ch.select(op, "Operation", core).map(|(_, _, q)| q))
            .collect::<Checked<BTreeSet<_>>>()?;
        for op in list(spec, "operations") {
            let (_, _, q) = h.select(&op, "Operation", core)?;
            if !members.contains(&q) {
                return Err(fault(
                    "CON-017",
                    format!("operation {q} outside capability"),
                ));
            }
        }
        return Ok(json!({}));
    }
    let (th, ty, _) = h.select(spec["type"].as_str().unwrap_or(""), "Type", core)?;
    if ty["spec"]["family"] != kind {
        return Err(fault("TPL-002", "Type has another family"));
    }
    let engine = h.indexes["core"]["engine"]["spec"].clone();
    let mut needed = BTreeSet::new();
    for cap in list(&ty["spec"], "offers") {
        let (ch, cap, _) = th.select(&cap, "Capability", core)?;
        for op in list(&cap["spec"], "operations") {
            needed.insert(ch.select(&op, "Operation", core)?.2);
        }
    }
    for op in list(&ty["spec"], "requires")
        .into_iter()
        .chain(list(&ty["spec"], "controls"))
    {
        needed.insert(th.select(&op, "Operation", core)?.2);
    }
    let mut selected = BTreeMap::new();
    for (source, scope, s) in [("type", &th, &ty["spec"]), ("template", h, spec)] {
        for (op, item) in items(s, "implementation") {
            let (_, _, qualified) = scope.select(&op, "Operation", core)?;
            if list(&engine, "implements").contains(&qualified) {
                return Err(fault(
                    "TPL-003",
                    format!("engine operation {qualified} cannot be selected"),
                ));
            }
            // An unused Type hook default is not selected (TPL-004).
            if source == "type" && !needed.contains(&qualified) {
                continue;
            }
            selected.insert(qualified, (source, scope.clone(), item));
        }
    }
    needed.extend(selected.keys().cloned());
    let mut result = json!({});
    let mut run_items = BTreeSet::new();
    for op in needed {
        if list(&engine, "implements").contains(&op) {
            result[&op] =
                json!({"source":"engine","selection":format!("{core}#engine"),"item":engine});
            continue;
        }
        let (source, scope, item) = selected
            .get(&op)
            .ok_or_else(|| fault("TPL-004", format!("no implementation for {op}")))?;
        let (ih, _, q) = scope.select(item.as_str().unwrap_or(""), "Implementation", core)?;
        let name = q.split_once('#').unwrap().1;
        let implementation = ih.contexts[&ih.scope]["implementations"][name].clone();
        let implements = list(&implementation, "implements")
            .iter()
            .map(|op| ih.select(op, "Operation", core).map(|(_, _, q)| q))
            .collect::<Checked<Vec<_>>>()?;
        if !implements.contains(&op) {
            return Err(fault("TPL-005", format!("{q} does not implement {op}")));
        }
        if implementation.get("command").is_some() || implementation.get("package").is_some() {
            run_items.insert(q.clone());
        }
        result[&op] = json!({"source":source,"selection":q,"item":implementation,"scope":ih.scope});
    }
    if run_items.len() > 1 || (!run_items.is_empty() && ["Pool", "Interface"].contains(&kind)) {
        return Err(fault("TPL-019", "invalid number of run items"));
    }
    Ok(result)
}

fn resolve_instance(
    object: &Value,
    all: &[Value],
    context: &Context,
    trusted: bool,
) -> Checked<Value> {
    let mut o = object.clone();
    let core = &context.core_package;
    let h: Held =
        serde_json::from_value(o["held"].clone()).map_err(|e| fault("NAM-009", e.to_string()))?;
    let refs = References {
        objects: all,
        core,
        unresolved: None,
        template_entries: None,
    };
    let kind = o["kind"].as_str().unwrap_or("").to_owned();
    let slot = o["nested"]["slot"].as_str().unwrap_or("").to_owned();
    let pool_value = o.get("entry_pool").cloned();
    let entry_pool = pool_value.as_ref();
    let supplied_placement = o.get("placement").cloned();
    let supplied_runtime = o.get("runtime").cloned();
    let mut standards = vec![];
    if kind == "Resource" {
        standards.push(("owner", Some("Pool")));
    }
    if kind == "Conduit" {
        standards.extend([("consumer", None), ("target", None)]);
    }
    for (key, k) in standards {
        let implicit =
            (key == "owner" && slot == "entries") || (key == "consumer" && slot == "conduits");
        o[key] = refs.resolve(&o[key], k, entry_pool, !implicit && !trusted)?;
        let target = all
            .iter()
            .find(|v| v["id"] == o[key]["id"])
            .ok_or_else(|| fault("OBJ-009", "missing referenced instance"))?;
        if target["state"] == "retired" {
            return Err(fault(
                if key == "owner" {
                    "POOL-004"
                } else {
                    "CON-025"
                },
                format!("{key} is retired"),
            ));
        }
        if key == "owner" && target["kind"] != "Pool" {
            return Err(fault("POOL-001", "owner is not a Pool"));
        }
    }
    let mut config_schema = None;
    let mut type_scope = None;
    if kind != "Conduit" {
        let (th, ty, q) = h.select(o["type"].as_str().unwrap_or(""), "Type", core)?;
        o["type"] = json!(q);
        let unconstrained = json!({});
        let schema = ty["spec"].get("schema").unwrap_or(&unconstrained);
        let config = th
            .fill(schema, &o["config"], core)
            .map_err(|e| fault("INS-014", e))?;
        if let Some(failure) = th
            .environment
            .failure_at(&th.scope, schema, "", &config, core)
            .map_err(|e| fault("INS-014", e))?
        {
            return Err(fault(
                "INS-014",
                format!("configuration fails Type {q}: {failure}"),
            ));
        }
        o["config"] = refs.schema(&th, schema, schema, &config, entry_pool, !trusted, 0)?;
        if !th
            .valid(schema, &o["config"], core)
            .map_err(|e| fault("INS-014", e))?
        {
            return Err(fault("INS-014", "stored configuration fails Type"));
        }
        config_schema = Some(schema.clone());
        type_scope = Some(th);
    }
    for (name, mut applied) in items(&o, "requirements") {
        let (rh, requirement, qualified) = h.select(
            applied["requirement"].as_str().unwrap_or(""),
            "Requirement",
            core,
        )?;
        let mut arguments = json!({});
        for (key, _) in items(&applied, "arguments") {
            if requirement["spec"]["inputs"].get(&key).is_none() {
                return Err(fault(
                    "INS-003",
                    format!("requirement {name}: undeclared argument {key}"),
                ));
            }
        }
        for (key, input) in items(&requirement["spec"], "inputs") {
            let value = applied["arguments"]
                .get(&key)
                .or_else(|| input.get("default"));
            let Some(value) = value else {
                if input["optional"] == true {
                    continue;
                }
                return Err(fault(
                    "INS-004",
                    format!("requirement {name}: missing input {key}"),
                ));
            };
            let value = rh
                .fill(&input["schema"], value, core)
                .map_err(|e| fault("INS-005", e))?;
            if let Some(failure) = rh
                .environment
                .failure_at(&rh.scope, &input["schema"], "", &value, core)
                .map_err(|e| fault("INS-005", e))?
            {
                return Err(fault(
                    "INS-005",
                    format!("requirement {name}, input {key}: {failure}"),
                ));
            }
            arguments[&key] = refs.schema(
                &rh,
                &input["schema"],
                &input["schema"],
                &value,
                entry_pool,
                !trusted,
                0,
            )?;
        }
        applied["requirement"] = json!(qualified);
        applied["arguments"] = arguments;
        o["requirements"][&name] = applied;
    }
    for (name, value) in items(&o, "_argument_values") {
        let input = o["_inputs"][&name].clone();
        // Reject authored IDs using the original schema-governed value. A Type may
        // already have resolved the destination, so the second pass accepts stored IDs.
        refs.schema(
            &h,
            &input["schema"],
            &input["schema"],
            &value,
            entry_pool,
            !trusted,
            0,
        )?;
        for pointer in list(&input, "to") {
            if let Some(value) = o.pointer(&pointer) {
                let value = refs.schema(
                    &h,
                    &input["schema"],
                    &input["schema"],
                    value,
                    entry_pool,
                    false,
                    0,
                )?;
                assign(&mut o, &pointer, value)?;
            }
        }
    }
    if let (Some(schema), Some(th)) = (&config_schema, &type_scope)
        && !th
            .valid(schema, &o["config"], core)
            .map_err(|e| fault("INS-014", e))?
    {
        return Err(fault("INS-014", "stored configuration fails Type"));
    }
    // Resolve the retained nested declarations as well as the instantiated children.
    for (local, mut declaration) in items(&o, "conduits") {
        if let Some(target) = declaration.get("target") {
            declaration["target"] =
                refs.resolve(target, None, entry_pool, target.get("id").is_none())?;
        }
        o["conduits"][&local] = declaration;
    }
    if ["Agent", "Resource"].contains(&kind.as_str()) {
        let placement = supplied_placement.unwrap_or(json!("local"));
        let placement = refs
            .resolve(&placement, Some("Resource"), entry_pool, !trusted)
            .map_err(|f| fault("PLC-001", f.text))?;
        let node = all
            .iter()
            .find(|n| n["id"] == placement["id"])
            .ok_or_else(|| fault("PLC-001", "unknown node"))?;
        if node["type"] != format!("{core}#node.v1") {
            return Err(fault("PLC-001", "placement is not a node"));
        }
        if node["state"] == "retired" {
            return Err(fault("PLC-018", "placement is retired"));
        }
        o["placement"] = placement;
        if let Some(runtime) = &supplied_runtime {
            let runtime = refs.resolve(runtime, Some("Resource"), entry_pool, !trusted)?;
            let provider = all
                .iter()
                .find(|n| n["id"] == runtime["id"])
                .ok_or_else(|| fault("PRV-006", "unknown runtime provider"))?;
            if provider["kind"] != "Resource"
                || provider["state"] == "retired"
                || !offered(provider, core)?.contains(&format!("{core}#runtime-provider.v1"))
            {
                return Err(fault(
                    "PRV-006",
                    format!("runtime {runtime} is not a live runtime provider"),
                ));
            }
            o["runtime"] = runtime;
        }
        let running = items(&o, "resolved_implementations")
            .iter()
            .any(|(_, i)| i["item"].get("command").is_some() || i["item"].get("package").is_some());
        if running {
            let (_, ty, _) = h.select(o["type"].as_str().unwrap(), "Type", core)?;
            let mut selector = ty["spec"].get("nodeSelector").cloned().unwrap_or(json!({}));
            for (_, implementation) in items(&o, "resolved_implementations") {
                for (key, value) in items(&implementation["item"], "nodeSelector") {
                    if selector.get(&key).is_some_and(|v| *v != value) {
                        return Err(fault("PLC-025", "conflicting node selector labels"));
                    }
                    selector[&key] = value;
                }
            }
            if o.get("runtime").is_none() {
                let local = all
                    .iter()
                    .find(|n| n["kind"] == "Resource" && n["name"] == "local")
                    .ok_or_else(|| fault("PLC-001", "local missing"))?;
                let hostname = &local["status"]["labels"]["kubernetes.io/hostname"];
                if selector
                    .get("kubernetes.io/hostname")
                    .is_some_and(|value| value != hostname)
                {
                    return Err(fault(
                        "PLC-007",
                        "selector excludes the engine runner's machine",
                    ));
                }
                selector["kubernetes.io/hostname"] = hostname.clone();
            }
            for (key, value) in selector.as_object().into_iter().flatten() {
                if node["status"]["labels"][key] != *value {
                    return Err(fault("PLC-007", format!("node does not match {key}")));
                }
            }
        }
    }
    if kind == "Conduit" {
        let (ch, cap, q) = h.select(o["capability"].as_str().unwrap_or(""), "Capability", core)?;
        let members = list(&cap["spec"], "operations")
            .iter()
            .map(|op| ch.select(op, "Operation", core).map(|(_, _, q)| q))
            .collect::<Checked<BTreeSet<_>>>()?;
        let operations = list(&o, "operations")
            .iter()
            .map(|op| h.select(op, "Operation", core).map(|(_, _, q)| q))
            .collect::<Checked<Vec<_>>>()?;
        if let Some(op) = operations.iter().find(|op| !members.contains(*op)) {
            return Err(fault(
                "INS-023",
                format!("operation {op} outside capability"),
            ));
        }
        let target = all
            .iter()
            .find(|v| v["id"] == o["target"]["id"])
            .ok_or_else(|| fault("OBJ-009", "target absent"))?;
        if !offered(target, core)?.contains(&q) {
            return Err(fault(
                "CON-021",
                format!("target {} does not offer {q}", target["name"]),
            ));
        }
        o["capability"] = json!(q);
        o["operations"] = json!(operations);
    }
    o.as_object_mut().unwrap().remove("entry_pool");
    o.as_object_mut().unwrap().remove("_inputs");
    o.as_object_mut().unwrap().remove("_argument_values");
    Ok(o)
}
fn offered(object: &Value, core: &str) -> Checked<BTreeSet<String>> {
    if let Some(held) = object.get("held") {
        let h: Held =
            serde_json::from_value(held.clone()).map_err(|e| fault("NAM-009", e.to_string()))?;
        let (th, ty, _) = h.select(object["type"].as_str().unwrap_or(""), "Type", core)?;
        list(&ty["spec"], "offers")
            .iter()
            .map(|cap| th.select(cap, "Capability", core).map(|(_, _, q)| q))
            .collect()
    } else {
        // First-start provenance is already supplied by the trusted core context.
        Ok(BTreeSet::from([
            match object["type"]
                .as_str()
                .and_then(|s| s.strip_prefix(&format!("{core}#")))
            {
                Some("pool.v1") => format!("{core}#pool-management.v1"),
                Some("management.v1") => format!("{core}#object-management.v1"),
                Some("access.v1") => format!("{core}#access-control.v1"),
                _ => String::new(),
            },
        ]))
    }
}

fn template_diagnostics(
    builder: &Builder<'_>,
    references: &References<'_>,
    template: &Value,
    h: &Held,
) -> Vec<Value> {
    let name = template["name"].as_str().unwrap();
    let core = references.core;
    let mut diagnostics = vec![];
    let mut missing_templates = vec![];
    let mut report = |path: &str, f: Fault| {
        let unresolved = f.rule == "OBJ-009" && f.text.contains("unresolved reference");
        diagnostics.push(diagnostic(
            if unresolved { "OBJ-008" } else { f.rule },
            name,
            &if path.is_empty() {
                f.text
            } else {
                format!("{path}: {}", f.text)
            },
            if unresolved { "note" } else { "error" },
        ));
    };
    // Continue after each unresolved leaf, so a single missing name cannot hide
    // other names in an array, map, or another applicable schema branch.
    let entries: BTreeSet<_> = items(&template["spec"], "entries")
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    let mut inspect = |scope: &Held, schema: &Value, value: &Value, path: &str, in_entry: bool| {
        let unresolved = RefCell::new(vec![]);
        let refs = References {
            objects: references.objects,
            core,
            unresolved: Some(&unresolved),
            template_entries: in_entry.then_some(&entries),
        };
        if let Err(f) = refs.schema(scope, schema, schema, value, None, true, 0) {
            report(path, f);
        }
        for f in unresolved.into_inner() {
            report(path, f);
        }
    };
    let kind = template["kind"].as_str().unwrap();
    let spec = &template["spec"];
    if kind != "Conduit"
        && let Ok((th, ty, _)) = h.select(spec["type"].as_str().unwrap_or(""), "Type", core)
    {
        let unconstrained = json!({});
        inspect(
            &th,
            ty["spec"].get("schema").unwrap_or(&unconstrained),
            &spec["config"],
            "",
            false,
        );
    }
    for (input, declaration) in items(spec, "inputs") {
        if let Some(value) = declaration.get("default") {
            inspect(
                h,
                &declaration["schema"],
                value,
                &format!("input {input}"),
                false,
            );
        }
    }
    for (local, applied) in items(spec, "requirements") {
        if let Ok((rh, requirement, _)) = h.select(
            applied["requirement"].as_str().unwrap_or(""),
            "Requirement",
            core,
        ) {
            for (input, declaration) in items(&requirement["spec"], "inputs") {
                if let Some(value) = applied["arguments"]
                    .get(&input)
                    .or_else(|| declaration.get("default"))
                {
                    inspect(
                        &rh,
                        &declaration["schema"],
                        value,
                        &format!("requirements/{local}/arguments/{input}"),
                        false,
                    );
                }
            }
        }
    }
    for (slot, family) in [("conduits", "Conduit"), ("entries", "Resource")] {
        for (local, declaration) in items(spec, slot) {
            for key in ["target", "placement", "runtime"] {
                if let Some(value) = declaration.get(key) {
                    let schema = json!({"$ref": format!("{core}#{}-reference.v1", if key == "target" { "target" } else { "resource" })});
                    inspect(
                        h,
                        &schema,
                        value,
                        &format!("{slot}/{local}/{key}"),
                        slot == "entries",
                    );
                }
            }
            match builder.template(&declaration["from"], family, h, false) {
                Ok((child, ch, _)) => {
                    for (input, definition) in items(&child["spec"], "inputs") {
                        if let Some(value) = declaration["arguments"].get(&input) {
                            inspect(
                                &ch,
                                &definition["schema"],
                                value,
                                &format!("{slot}/{local}/arguments/{input}"),
                                slot == "entries",
                            );
                        }
                    }
                }
                Err(f) => missing_templates.push((
                    format!("{slot}/{local}/from"),
                    if f.rule == "INS-002" && f.text.starts_with(&format!("no {family} template "))
                    {
                        fault(
                            "OBJ-009",
                            format!("unresolved reference {}", declaration["from"]),
                        )
                    } else {
                        f
                    },
                )),
            }
        }
    }
    for (path, f) in missing_templates {
        report(&path, f);
    }
    diagnostics.sort_by_key(Value::to_string);
    diagnostics.dedup();
    diagnostics
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn register(
    t: &mut Transition,
    snapshots: Vec<Snapshot>,
    builtin: Project,
    project: Project,
    evidence: &Evidence,
    request: &Request,
    context: &Context,
) -> Result<()> {
    let permitted = authorized_path(&t.state, request, context)?;
    let management = t
        .state
        .objects
        .iter()
        .find(|o| o["from"] == context.qualified("management"))
        .ok_or("management absent")?;
    if request.operation != context.qualified("register.v1") || management["id"] != request.target {
        return Err("unsupported registration operation/target".into());
    }
    let mut pool_authority = true;
    for doc in &project.documents {
        if doc["kind"] != "Resource" || doc.get("from").is_none() {
            continue;
        }
        if let Some(pool) =
            crate::access::object(&t.state, &doc["owner"]).filter(|p| p["kind"] == "Pool")
            && pool["state"] != "retired"
        {
            pool_authority &= t
                .state
                .objects
                .iter()
                .any(|c| permits(c, &request.sender, &pool["id"], &request.operation));
        }
    }
    if !permitted || !pool_authority {
        t.decisions.push(decision(
            "refuse-unauthorized-registration",
            &["ACC-033", "OPS-023"],
            &[],
            json!({}),
            vec![],
        ));
        end_request(t, evidence, request, None, "refused", context)?;
        t.reply = json!({"accepted":false,"reason":"refused"});
        return Ok(());
    }
    prepare(t, snapshots, builtin, project, evidence, request, context)
}

/// Resolve and stage an already authorized submission, also used for one-template replacement.
#[allow(clippy::too_many_arguments)]
pub(crate) fn prepare(
    t: &mut Transition,
    snapshots: Vec<Snapshot>,
    builtin: Project,
    mut project: Project,
    evidence: &Evidence,
    request: &Request,
    context: &Context,
) -> Result<()> {
    // Canonical traversal fixes allocations, diagnostics and nested names independently of file order.
    project.documents.sort_by_key(|d| {
        (
            d["kind"].to_string(),
            d["metadata"]["name"].to_string(),
            d.to_string(),
        )
    });
    let mut diagnostics = vec![];
    let mut named = BTreeMap::<(String, String), Value>::new();
    for doc in &project.documents {
        if packages::contract(doc) {
            continue;
        }
        if let (Some(kind), Some(name)) = (doc["kind"].as_str(), doc["metadata"]["name"].as_str()) {
            let key = (kind.to_owned(), name.to_owned());
            if let Some(previous) = named.insert(key, doc.clone()) {
                diagnostics.push(diagnostic(
                    "SUB-004",
                    name,
                    &format!("duplicate documents {previous} and {doc}"),
                    "error",
                ));
            }
            if let Some(existing) = t
                .state
                .objects
                .iter()
                .find(|o| o["kind"] == kind && o["name"] == name)
            {
                diagnostics.push(diagnostic(
                    "OBJ-002",
                    name,
                    &format!(
                        "submitted object {doc} clashes with existing {}",
                        existing["id"]
                    ),
                    "error",
                ));
            }
        }
    }
    if diagnostics.iter().any(|d| d["ruleId"] == "SUB-004") {
        return reject(t, diagnostics, evidence, request, context);
    }
    let catalog = Catalog::new(builtin.clone(), snapshots.clone(), &context.core_package)?;
    let resolution = catalog.resolve(&project)?;
    t.decisions.extend(resolution.decisions);
    if resolution.reply["accepted"] != true {
        diagnostics.extend(
            resolution.reply["diagnostics"]
                .as_array()
                .cloned()
                .unwrap_or_default(),
        );
        return reject(t, diagnostics, evidence, request, context);
    }
    diagnostics.extend(
        resolution.reply["diagnostics"]
            .as_array()
            .cloned()
            .unwrap_or_default(),
    );
    let mut indexes = BTreeMap::from([
        ("core".into(), packages::index(&builtin)),
        ("project".into(), packages::index(&project)),
    ]);
    for (i, s) in snapshots.iter().enumerate() {
        indexes.insert(format!("snapshot-{i}"), packages::index(&s.project));
    }
    let h = Held {
        environment: serde_json::from_value(resolution.reply["environment"].clone())
            .map_err(|e| e.to_string())?,
        indexes,
        contexts: resolution.reply["contexts"].clone(),
        scope: "project".into(),
    };
    let mut b = Builder {
        context,
        evidence,
        state: t.state.clone(),
        allocation: Allocation {
            evidence,
            cursor: 0,
        },
        suffix: 0,
        objects: vec![],
        diagnostics,
        templates: vec![],
        reserved_names: named.keys().cloned().collect(),
        snapshots,
        trusted: false,
    };
    for doc in &project.documents {
        if packages::contract(doc) || doc.get("from").is_some() {
            continue;
        }
        let kind = doc["kind"].as_str().ok_or("template family missing")?;
        let name = doc["metadata"]["name"]
            .as_str()
            .ok_or("template name missing")?;
        let mut spec = doc["spec"].clone();
        implicit(&mut spec, kind);
        let check = check_destinations(&spec, kind)
            .and_then(|()| implementations(&h, &spec, kind, &context.core_package));
        let implementations = match check {
            Ok(i) => i,
            Err(f) => {
                b.diagnostics
                    .push(diagnostic(f.rule, name, &f.text, "error"));
                continue;
            }
        };
        let (id, revision) = b.allocate()?;
        let packages = h.package_list(&spec, kind, &implementations, &context.core_package)?;
        b.templates.push(json!({"kind":kind,"name":name,"id":id,"revision":revision,"metadata":doc["metadata"],"spec":spec,"resolved_implementations":implementations,"held":h,"packages":packages}));
    }
    for doc in &project.documents {
        if doc.get("from").is_some() {
            b.expand(doc, &h, None, 0)?;
        }
    }
    // Inherit the resolved implementation table from the actual selected template.
    for i in 0..b.objects.len() {
        let o = &b.objects[i];
        let held: Held = serde_json::from_value(o["held"].clone()).map_err(|e| e.to_string())?;
        let table = if o["kind"] == "Conduit" {
            Ok(json!({}))
        } else {
            implementations(&held, o, o["kind"].as_str().unwrap(), &context.core_package)
        };
        match table {
            Ok(table) => b.objects[i]["resolved_implementations"] = table,
            Err(f) => b.diagnostics.push(diagnostic(
                f.rule,
                o["name"].as_str().unwrap(),
                &f.text,
                "error",
            )),
        }
        b.objects[i]["packages"] = held.package_list(
            &b.objects[i],
            b.objects[i]["kind"].as_str().unwrap(),
            &b.objects[i]["resolved_implementations"],
            &context.core_package,
        )?;
    }
    let all = b
        .state
        .objects
        .iter()
        .chain(&b.templates)
        .chain(&b.objects)
        .cloned()
        .collect::<Vec<_>>();
    // Templates may name future instances. Missing names are notes; an instance
    // must resolve every such name when it is actually made (OBJ-008/009).
    let references = References {
        objects: &all,
        core: &context.core_package,
        unresolved: None,
        template_entries: None,
    };
    for template in &b.templates {
        b.diagnostics
            .extend(template_diagnostics(&b, &references, template, &h));
    }
    let mut resolved = vec![];
    for object in &b.objects {
        match resolve_instance(object, &all, context, false) {
            Ok(o) => resolved.push(o),
            Err(f) => b.diagnostics.push(diagnostic(
                f.rule,
                object["name"].as_str().unwrap(),
                &f.text,
                "error",
            )),
        }
    }
    // The parent keeps the declaration used for a future nested replacement,
    // including default-filled arguments and references frozen to their IDs.
    for child in &b.objects {
        let Some(nested) = child.get("nested") else {
            continue;
        };
        let resolved_child = resolved.iter().find(|o| o["id"] == child["id"]).cloned();
        let Some(parent) = resolved.iter_mut().find(|o| o["id"] == nested["parent"]) else {
            continue;
        };
        let slot = nested["slot"].as_str().unwrap();
        let local = nested["name"].as_str().unwrap();
        let child_held: Held =
            serde_json::from_value(child["held"].clone()).map_err(|e| e.to_string())?;
        for (name, value) in items(child, "_argument_values") {
            let schema = &child["_inputs"][&name]["schema"];
            match references.schema(
                &child_held,
                schema,
                schema,
                &value,
                child.get("entry_pool"),
                true,
                0,
            ) {
                Ok(value) => parent[slot][local]["arguments"][&name] = value,
                Err(f) => b.diagnostics.push(diagnostic(
                    f.rule,
                    child["name"].as_str().unwrap(),
                    &f.text,
                    "error",
                )),
            }
        }
        if slot == "conduits"
            && let Some(target) = child.get("target")
            && let Ok(value) = references.resolve(target, None, child.get("entry_pool"), true)
        {
            parent[slot][local]["target"] = value;
        }
        if slot == "entries"
            && let Some(child) = resolved_child
        {
            for key in ["placement", "runtime"] {
                if parent[slot][local].get(key).is_some()
                    && let Some(value) = child.get(key)
                {
                    parent[slot][local][key] = value.clone();
                }
            }
        }
    }
    if b.diagnostics.iter().any(|d| d["level"] == "error") {
        return reject(t, b.diagnostics, evidence, request, context);
    }
    let mut all_resolved = b.state.objects.clone();
    all_resolved.extend(resolved.iter().cloned());
    for path in resolved.iter().filter(|o| o["kind"] == "Conduit") {
        if let Some((rule, text)) = check_conduit(path, &all_resolved, context)? {
            b.diagnostics.push(diagnostic(
                rule,
                path["name"].as_str().unwrap_or(""),
                &text,
                "error",
            ));
        }
    }
    if b.diagnostics.iter().any(|d| d["level"] == "error") {
        return reject(t, b.diagnostics, evidence, request, context);
    }
    // Nothing has been published before this point. The caller receives only this commit.
    let registered = b
        .templates
        .iter()
        .chain(&resolved)
        .map(|o| {
            // object.v1 names the enclosing instance, not the core's storage slot
            // (INS-016). An ordinary object has no nested member.
            let mut result = json!({"kind":o["kind"],"name":o["name"],"id":o["id"]});
            if let Some(nested) = o.get("nested") {
                result["nested"] = json!({"instance":nested["parent"],"name":nested["name"]});
            }
            result
        })
        .collect::<Vec<_>>();
    for snapshot in &b.snapshots {
        if snapshot.source == packages::SnapshotSource::PreinstalledSnapshot
            && !b.state.preinstalled_snapshots.contains(snapshot)
        {
            b.state.preinstalled_snapshots.push(snapshot.clone());
        }
    }
    b.state.objects.extend(b.templates);
    b.state.objects.extend(resolved);
    for object in &registered {
        let record=b.allocation.record(&mut b.state,context,"object-change",json!({"target":object["id"],"caller":request.caller,"consumer":request.sender,"conduit":request.conduit,"operation":request.operation,"action":"registered","state":"registered"}))?;
        t.records.push(record);
    }
    t.state = b.state;
    t.decisions.push(decision(
        "register-project-atomically",
        &[
            "SUB-001", "SUB-002", "SUB-003", "SUB-006", "SUB-008", "OBJ-002", "OBJ-003", "OBJ-009",
            "OBJ-010", "OBJ-011", "OBJ-018", "OBJ-021", "INS-002", "INS-003", "INS-004", "INS-005",
            "INS-006", "INS-007", "INS-008", "INS-009", "INS-010", "INS-011", "INS-012", "INS-013",
            "INS-014", "INS-016", "INS-017", "INS-018", "INS-019", "INS-020", "INS-023", "INS-024",
            "INS-025", "TPL-002", "TPL-003", "TPL-004", "TPL-005", "TPL-019", "NAM-009", "NAM-010",
            "PLC-001", "PLC-002", "PLC-007", "POOL-001", "POOL-002", "CON-002", "ACC-033",
            "REC-001", "REC-002", "REC-003", "POOL-004", "CON-017", "CON-021", "OBJ-008",
            "CTR-004", "OBJ-013", "NAM-019", "SCH-009", "PLC-005", "PKG-007", "POOL-009",
            "POOL-014", "POOL-016", "PRV-006",
        ],
        &["DL15"],
        json!({"objects":registered.len(),"generated_suffixes_consumed":b.suffix}),
        registered
            .iter()
            .filter_map(|o| o["id"].as_str().map(str::to_owned))
            .collect(),
    ));
    t.reply = json!({"accepted":true,"objects":registered,"diagnostics":b.diagnostics});
    Ok(())
}
fn reject(
    t: &mut Transition,
    mut diagnostics: Vec<Value>,
    evidence: &Evidence,
    request: &Request,
    context: &Context,
) -> Result<()> {
    diagnostics.sort_by_key(Value::to_string);
    diagnostics.dedup();
    let invalid = diagnostics
        .iter()
        .any(|d| d["level"] == "error" && d["ruleId"] != "OBJ-002");
    let reason = if invalid { "invalid" } else { "conflict" };
    let mut promises = diagnostics
        .iter()
        .filter_map(|d| d["ruleId"].as_str())
        .collect::<BTreeSet<_>>();
    promises.extend([
        "SUB-001",
        "SUB-003",
        "OPS-023",
        if invalid { "OBJ-023" } else { "OBJ-025" },
    ]);
    t.decisions.push(decision(
        "reject-registration",
        &promises.into_iter().collect::<Vec<_>>(),
        &[],
        json!({}),
        vec![],
    ));
    end_request(t, evidence, request, None, reason, context)?;
    t.reply = json!({"accepted":false,"reason":reason,"diagnostics":diagnostics});
    Ok(())
}

/// Adapt frozen registration contexts to the schema publisher's view, without changing
/// the stored instance or resolving its names against a newer submission.
pub(crate) fn publication_object(object: &Value, context: &Context) -> Result<Value> {
    let mut view = object.clone();
    if let Some(held) = object.get("held") {
        let h: Held = serde_json::from_value(held.clone()).map_err(|e| e.to_string())?;
        let (th, ty, _) = h
            .select(
                object["type"].as_str().ok_or("missing Type")?,
                "Type",
                &context.core_package,
            )
            .map_err(|f| f.text)?;
        view["schema_environment"] =
            serde_json::to_value(&h.environment).map_err(|e| e.to_string())?;
        view["contract_scope"] = json!(th.scope);
        view["template_scope"] = json!(h.scope);
        view["type_controls"] = json!(
            list(&ty["spec"], "controls")
                .iter()
                .map(|op| th
                    .select(op, "Operation", &context.core_package)
                    .map(|(_, _, q)| q)
                    .map_err(|f| f.text))
                .collect::<Result<Vec<_>>>()?
        );
        if object.get("controls").is_some() {
            let mut controls = json!({});
            for (op, narrowing) in items(object, "controls") {
                let (_, _, q) = h
                    .select(&op, "Operation", &context.core_package)
                    .map_err(|f| f.text)?;
                controls[&q] = narrowing;
            }
            view["controls"] = controls;
        }
    } else if object.get("schema_environment").is_none() {
        view["schema_environment"] =
            serde_json::to_value(&context.core_environment).map_err(|e| e.to_string())?;
        view["contract_scope"] = json!("core");
    }
    if object.get("held").is_none()
        && object["type"]
            .as_str()
            .is_some_and(|ty| ty.starts_with(&format!("{}#", context.core_package)))
    {
        let (_, _, ty) = crate::schemas::operation(
            &view,
            object["type"].as_str().unwrap(),
            &context.core_package,
        )?;
        for op in list(&ty["spec"], "requires") {
            if view["resolved_implementations"].get(&op).is_none() {
                view["resolved_implementations"][&op] = json!({"source":"engine"});
            }
        }
    }
    Ok(view)
}

fn check_conduit(
    path: &Value,
    all: &[Value],
    context: &Context,
) -> Result<Option<(&'static str, String)>> {
    let core = &context.core_package;
    let target = all
        .iter()
        .find(|o| o["id"] == *crate::access::id(&path["target"]))
        .ok_or("missing target")?;
    let bounds = items(path, "requirements")
        .iter()
        .filter(|(_, r)| r["requirement"] == context.qualified("bound.v1"))
        .count();
    if bounds > 1 || bounds == 1 && target["from"] != context.qualified("access") {
        return Ok(Some((
            "CON-028",
            "bound is permitted once on a Conduit to access".into(),
        )));
    }
    for (_, restriction) in items(path, "requirements") {
        if restriction["requirement"]
            .as_str()
            .is_some_and(|r| r.starts_with(&format!("{core}#")))
        {
            continue;
        }
        for op in list(path, "operations") {
            let supported = supports_filter(target, &op, &restriction["requirement"], context)?;
            if !supported {
                return Ok(Some((
                    "CON-022",
                    format!("filter {} unsupported for {op}", restriction["requirement"]),
                )));
            }
        }
    }
    let local = |p: &Value| {
        p["nested"]["name"]
            .as_str()
            .or_else(|| p["local_name"].as_str())
            .or_else(|| p["name"].as_str())
            .unwrap_or("")
            .to_owned()
    };
    let names = |p: &Value| -> Result<BTreeSet<String>> {
        let target = all
            .iter()
            .find(|o| o["id"] == *crate::access::id(&p["target"]))
            .ok_or("missing target")?;
        let target = publication_object(target, context)?;
        let mut names = BTreeSet::new();
        for operation in list(p, "operations") {
            if crate::schemas::publication_omission(&target, &operation, context)?.is_none()
                && let Some((_, export)) = operation.split_once('#')
            {
                names.insert(format!("{}.{export}", local(p)));
            }
        }
        Ok(names)
    };
    let presented = names(path)?;
    for other in all.iter().filter(|o| {
        o["kind"] == "Conduit"
            && o["id"] != path["id"]
            && crate::access::id(&o["consumer"]) == crate::access::id(&path["consumer"])
    }) {
        if let Some(name) = names(other)?.intersection(&presented).next() {
            return Ok(Some((
                "IFC-014",
                format!(
                    "tool name {name} collides with Conduit {} ({})",
                    other["name"].as_str().unwrap_or(""),
                    other["id"].as_str().unwrap_or("")
                ),
            )));
        }
    }
    Ok(None)
}

pub(crate) fn supports_filter(
    target: &Value,
    operation: &str,
    requirement: &Value,
    context: &Context,
) -> Result<bool> {
    let implementation = &target["resolved_implementations"][operation];
    if implementation["source"] == "engine" || implementation["item"]["plain"] == true {
        return Ok(false);
    }
    if let Some(held) = target.get("held") {
        let mut h: Held = serde_json::from_value(held.clone()).map_err(|e| e.to_string())?;
        h.scope = implementation["scope"]
            .as_str()
            .unwrap_or(&h.scope)
            .to_owned();
        Ok(list(&implementation["item"], "supports").iter().any(|r| {
            h.select(r, "Requirement", &context.core_package)
                .is_ok_and(|(_, _, q)| *requirement == q)
        }))
    } else {
        Ok(list(&implementation["item"], "supports")
            .iter()
            .any(|r| requirement == r))
    }
}

/// Reuse the registration resolver for an explicit nested replacement. The resolved
/// declaration is trusted stored data; actor submissions still reject authored IDs.
pub(crate) fn replacement(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    context: &Context,
    parent: &Value,
    name: &Value,
) -> Result<Value> {
    let slot = if parent["kind"] == "Pool" {
        "entries"
    } else {
        "conduits"
    };
    let name = name.as_str().ok_or("replacement slot is not a name")?;
    let mut doc = parent[slot][name].clone();
    doc["kind"] = json!(if slot == "entries" {
        "Resource"
    } else {
        "Conduit"
    });
    let held: Held = serde_json::from_value(parent["held"].clone()).map_err(|e| e.to_string())?;
    let mut b = Builder {
        context,
        evidence: a.evidence,
        state: t.state.clone(),
        allocation: Allocation {
            evidence: a.evidence,
            cursor: a.cursor,
        },
        suffix: 0,
        objects: vec![],
        diagnostics: vec![],
        templates: vec![],
        reserved_names: BTreeSet::new(),
        snapshots: packages::catalog_entries(&t.state, t.state.preinstalled_snapshots.clone()),
        trusted: true,
    };
    b.expand(&doc, &held, Some((parent, slot, name)), 0)?;
    for object in &mut b.objects {
        let h: Held = serde_json::from_value(object["held"].clone()).map_err(|e| e.to_string())?;
        match implementations(
            &h,
            object,
            object["kind"].as_str().unwrap(),
            &context.core_package,
        ) {
            Ok(table) => object["resolved_implementations"] = table,
            Err(f) => return Ok(json!({"rule":f.rule,"message":f.text})),
        }
        object["packages"] = h.package_list(
            object,
            object["kind"].as_str().unwrap(),
            &object["resolved_implementations"],
            &context.core_package,
        )?;
    }
    if let Some(d) = b.diagnostics.first() {
        return Ok(json!({"rule":d["ruleId"],"message":d["message"]["text"]}));
    }
    let all: Vec<_> = b.state.objects.iter().chain(&b.objects).cloned().collect();
    let mut resolved = vec![];
    for object in &b.objects {
        match resolve_instance(object, &all, context, true) {
            Ok(o) => resolved.push(o),
            Err(f) => return Ok(json!({"rule":f.rule,"message":f.text})),
        }
    }
    let all: Vec<_> = b.state.objects.iter().chain(&resolved).cloned().collect();
    for o in resolved.iter().filter(|o| o["kind"] == "Conduit") {
        if let Some((rule, message)) = check_conduit(o, &all, context)? {
            return Ok(json!({"rule":rule,"message":message}));
        }
    }
    let object = resolved.first().ok_or("replacement produced no object")?;
    let result = json!({"object":{"kind":object["kind"],"name":object["name"],"id":object["id"]},"diagnostics":[]});
    let p = b
        .state
        .objects
        .iter_mut()
        .find(|o| o["id"] == parent["id"])
        .unwrap();
    p["slots"][slot][name] = json!({"kind":object["kind"],"id":object["id"]});
    b.state.objects.extend(resolved);
    a.cursor = b.allocation.cursor;
    t.state = b.state;
    Ok(result)
}
