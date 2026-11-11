//! Authored schemas (SCH-001/009/015/016/019) and standalone tools (IFC-033).
//! All package snapshots are supplied values. No I/O or ambient resolver is used.
use crate::{Context, Decision, Result, State, decision};
use conduit_loader::formats;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    pub address: String,
    pub version: Option<String>,
    /// Address -> exact supplied scope, including the selected release (NAM-014).
    pub manifest: BTreeMap<String, String>,
    pub contracts: BTreeMap<String, Value>,
    /// Present on catalog-derived package scopes; old standalone schema fixtures omit it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exports: Option<Vec<String>>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    pub scopes: BTreeMap<String, Scope>,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Contract(String, String);

fn token(s: &str) -> String {
    s.replace('~', "~0").replace('/', "~1")
}
fn fragment(s: &str) -> String {
    s.bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~!$&'()*+,;=:@/?".contains(&b) {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
fn unfragment(s: &str) -> Result<String> {
    let mut out = Vec::new();
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        out.push(if b == b'%' {
            let a = bytes.next().ok_or("incomplete URI escape")?;
            let b = bytes.next().ok_or("incomplete URI escape")?;
            u8::from_str_radix(std::str::from_utf8(&[a, b]).map_err(|e| e.to_string())?, 16)
                .map_err(|e| e.to_string())?
        } else {
            b
        });
    }
    String::from_utf8(out).map_err(|e| e.to_string())
}
/// Visit schema positions only, never annotation or instance data.
pub(crate) fn walk<F>(schema: &mut Value, at: &str, visit: &mut F) -> Result<()>
where
    F: FnMut(&mut Map<String, Value>, &str) -> Result<()> + ?Sized,
{
    let Some(map) = schema.as_object_mut() else {
        return Ok(());
    };
    visit(map, at)?;
    for key in [
        "properties",
        "$defs",
        "patternProperties",
        "dependentSchemas",
        "x-conduit-patterns",
    ] {
        if let Some(children) = map.get_mut(key).and_then(Value::as_object_mut) {
            for (name, sub) in children {
                walk(sub, &format!("{at}/{key}/{}", token(name)), visit)?;
            }
        }
    }
    for key in [
        "additionalProperties",
        "items",
        "contains",
        "propertyNames",
        "not",
        "if",
        "then",
        "else",
    ] {
        if let Some(sub) = map.get_mut(key) {
            walk(sub, &format!("{at}/{key}"), visit)?;
        }
    }
    for key in ["allOf", "anyOf", "oneOf", "prefixItems"] {
        if let Some(children) = map.get_mut(key).and_then(Value::as_array_mut) {
            for (i, sub) in children.iter_mut().enumerate() {
                walk(sub, &format!("{at}/{key}/{i}"), visit)?;
            }
        }
    }
    Ok(())
}
impl Environment {
    pub(crate) fn schema_reference(
        &self,
        scope: &str,
        root: &Value,
        reference: &str,
        core: &str,
    ) -> Result<(String, Value)> {
        if reference == "#" || reference.starts_with("#/") {
            return Ok((
                scope.into(),
                root.pointer(&unfragment(&reference[1..])?)
                    .ok_or("missing local schema pointer")?
                    .clone(),
            ));
        }
        let c = self.resolve(scope, reference, core)?;
        Ok((c.0.clone(), self.schema(&c)?.clone()))
    }
    fn scope(&self, scope: &str) -> Result<&Scope> {
        self.scopes
            .get(scope)
            .ok_or_else(|| format!("unknown scope {scope}"))
    }
    fn resolve(&self, scope: &str, reference: &str, core: &str) -> Result<Contract> {
        let (address, name) = reference
            .split_once('#')
            .ok_or("SCH-015: malformed contract reference")?;
        let own = self.scope(scope)?;
        let target = if address.is_empty() || address == own.address {
            scope.to_owned()
        } else if address == core {
            self.scopes
                .iter()
                .find(|(_, s)| s.address == core)
                .map(|(k, _)| k.clone())
                .ok_or("NAM-020: missing core snapshot")?
        } else {
            own.manifest
                .get(address)
                .cloned()
                .ok_or_else(|| format!("NAM-014: no pin for {address} in {scope}"))?
        };
        let held = self.scope(&target)?;
        if target != scope
            && held
                .exports
                .as_ref()
                .is_some_and(|e| !e.iter().any(|e| e == name))
        {
            return Err(format!("PKG-018: {reference} is not exported"));
        }
        if !address.is_empty() && held.address != address {
            return Err("NAM-014: pin address differs".into());
        }
        if !held.contracts.contains_key(name) {
            return Err(format!("SCH-015: unresolved {reference}"));
        }
        Ok(Contract(target, name.into()))
    }
    fn schema(&self, c: &Contract) -> Result<&Value> {
        let doc = &self.scope(&c.0)?.contracts[&c.1];
        if doc["kind"] != "Schema" {
            return Err(format!("SCH-015: {} is not a Schema", c.1));
        }
        doc.get("spec").ok_or("Schema has no spec".into())
    }
    fn key(&self, c: &Contract) -> Result<String> {
        let s = self.scope(&c.0)?;
        Ok(format!(
            "{}{}#{}",
            s.address,
            s.version
                .as_ref()
                .map(|v| format!("@{v}"))
                .unwrap_or_default(),
            c.1
        ))
    }
    fn uri(&self, c: &Contract) -> Result<String> {
        let index = self
            .scopes
            .keys()
            .position(|s| s == &c.0)
            .ok_or("missing scope")?;
        Ok(format!("urn:conduit:s{index}:{}", c.1))
    }
    fn contextual_root(
        &self,
        scope: &str,
        schema: &Value,
        root_uri: &str,
        core: &str,
    ) -> Result<Value> {
        let mut schema = schema.clone();
        walk(&mut schema, "", &mut |map, _| {
            if let Some(reference) = map.get("$ref").and_then(Value::as_str) {
                let rewritten = if reference == "#" || reference.starts_with("#/") {
                    format!("{root_uri}{reference}")
                } else {
                    self.uri(&self.resolve(scope, reference, core)?)?
                };
                map.insert("$ref".into(), json!(rewritten));
            }
            Ok(())
        })?;
        Ok(schema)
    }
    /// Original resource boundaries stay intact. Contract references select supplied exact
    /// releases; local pointers remain relative to their own resource, independent of publication.
    fn resources(&self, core: &str) -> Result<BTreeMap<String, Value>> {
        let mut resources = BTreeMap::new();
        for (scope, s) in &self.scopes {
            for (name, doc) in &s.contracts {
                if doc["kind"] == "Schema" {
                    let c = Contract(scope.clone(), name.clone());
                    let uri = self.uri(&c)?;
                    resources.insert(
                        uri.clone(),
                        self.contextual_root(scope, &doc["spec"], &uri, core)?,
                    );
                }
            }
        }
        Ok(resources)
    }
    pub fn publish_schema(&self, scope: &str, schema: &Value, core: &str) -> Result<Value> {
        let mut root = schema.clone();
        // Booleans cannot hold $defs. The wrapper preserves their meaning.
        if !root.is_object() {
            root = json!({"allOf":[root]});
        }
        let mut todo = BTreeSet::new();
        self.rewrite(&mut root, scope, "", core, &mut todo)?;
        self.finish_bundle(root, todo, core)
    }
    fn finish_bundle(
        &self,
        mut root: Value,
        mut todo: BTreeSet<Contract>,
        core: &str,
    ) -> Result<Value> {
        let mut copies = BTreeMap::new();
        while let Some(c) = todo.pop_first() {
            let key = self.key(&c)?;
            if copies.contains_key(&key) {
                continue;
            }
            let mut copy = self.schema(&c)?.clone();
            copies.insert(key.clone(), Value::Null); // mark before following recursion
            self.rewrite(
                &mut copy,
                &c.0,
                &format!("/$defs/{}", token(&key)),
                core,
                &mut todo,
            )?;
            copies.insert(key, copy);
        }
        if !copies.is_empty() {
            let defs = root
                .as_object_mut()
                .ok_or("schema root")?
                .entry("$defs")
                .or_insert(json!({}))
                .as_object_mut()
                .ok_or("invalid $defs")?;
            for (key, copy) in copies {
                if defs.insert(key, copy).is_some() {
                    return Err("IFC-033: reserved root $defs key".into());
                }
            }
        }
        Ok(root)
    }
    fn rewrite(
        &self,
        schema: &mut Value,
        scope: &str,
        prefix: &str,
        core: &str,
        todo: &mut BTreeSet<Contract>,
    ) -> Result<()> {
        walk(schema, "", &mut |map, _| {
            if let Some(reference) = map.get("$ref").and_then(Value::as_str) {
                let pointer = if reference == "#" || reference.starts_with("#/") {
                    format!("{prefix}{}", unfragment(&reference[1..])?)
                } else {
                    let c = self.resolve(scope, reference, core)?;
                    self.schema(&c)?;
                    let pointer = format!("/$defs/{}", token(&self.key(&c)?));
                    todo.insert(c);
                    pointer
                };
                map.insert("$ref".into(), json!(format!("#{}", fragment(&pointer))));
            }
            Ok(())
        })
    }
}

/// A resolver for authored resources only; it cannot fetch or open anything.
struct Supplied(BTreeMap<String, Value>);
impl jsonschema::Retrieve for Supplied {
    fn retrieve(
        &self,
        uri: &jsonschema::Uri<String>,
    ) -> std::result::Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        self.0
            .get(uri.as_str())
            .cloned()
            .ok_or_else(|| format!("no supplied resource: {uri}").into())
    }
}

/// patternProperties uses the loader's ECMA engine. For a finite instance, replace each
/// pattern by its exact set of matching names. Keep original schema positions in an extension
/// and redirect pointers, so recursive references and additionalProperties retain their meaning.
fn specialize(
    schema: &Value,
    value: &Value,
    base: &str,
) -> Result<(Value, BTreeMap<String, String>)> {
    fn keys(v: &Value, out: &mut BTreeSet<String>) {
        match v {
            Value::Object(m) => {
                for (k, v) in m {
                    out.insert(k.clone());
                    keys(v, out)
                }
            }
            Value::Array(a) => {
                for v in a {
                    keys(v, out)
                }
            }
            _ => {}
        }
    }
    let mut names = BTreeSet::new();
    keys(value, &mut names);
    let mut root = schema.clone();
    let mut redirects = BTreeMap::new();
    walk(&mut root, "", &mut |map, at| {
        if let Some(pattern) = map.get("pattern").and_then(Value::as_str) {
            formats::ecma_regex(pattern).map_err(|e| format!("SCH-016: {e}"))?;
        }
        if let Some(patterns) = map.remove("patternProperties") {
            let patterns = patterns
                .as_object()
                .ok_or("patternProperties is not a map")?;
            redirects.insert(
                format!("{at}/patternProperties"),
                format!("{at}/x-conduit-patterns"),
            );
            for (pattern, _) in patterns {
                let re = formats::ecma_regex(pattern).map_err(|e| format!("SCH-016: {e}"))?;
                for name in names.iter().filter(|name| re.find(name).is_some()) {
                    let properties = map
                        .entry("properties")
                        .or_insert(json!({}))
                        .as_object_mut()
                        .ok_or("invalid properties")?;
                    // Keep declared property schemas at their original addresses. Pattern
                    // assertions apply to the containing map, not to references to a property.
                    properties.entry(name.clone()).or_insert(json!(true));
                    let reference = format!(
                        "{base}#{}",
                        fragment(&format!("{at}/x-conduit-patterns/{}", token(pattern)))
                    );
                    map.entry("allOf")
                        .or_insert(json!([]))
                        .as_array_mut()
                        .ok_or("invalid allOf")?
                        .push(json!({"properties":{name:{"$ref":reference}}}));
                }
            }
            map.insert("x-conduit-patterns".into(), Value::Object(patterns.clone()));
        }
        Ok(())
    })?;
    walk(&mut root, "", &mut |map, _| {
        if let Some(reference) = map.get("$ref").and_then(Value::as_str) {
            let local = reference.strip_prefix(&format!("{base}#"));
            if let Some(local) = local {
                let pointer = redirected(unfragment(local)?, &redirects);
                map.insert(
                    "$ref".into(),
                    json!(format!("{base}#{}", fragment(&pointer))),
                );
            }
        }
        Ok(())
    })?;
    Ok((root, redirects))
}

fn redirected(mut pointer: String, redirects: &BTreeMap<String, String>) -> String {
    // Deepest first: a pattern schema may itself hold patternProperties.
    for (from, to) in redirects.iter().rev() {
        if pointer == *from || pointer.starts_with(&format!("{from}/")) {
            pointer = format!("{to}{}", &pointer[from.len()..]);
        }
    }
    pointer
}

/// Standalone validation has no package resolver. First check every schema-position $ref
/// is a local fragment and resolves in the supplied definition, then compile with the loader.
pub fn standalone_valid(schema: &Value, value: &Value) -> Result<bool> {
    let mut checked = schema.clone();
    walk(&mut checked, "", &mut |map, _| {
        if let Some(reference) = map.get("$ref").and_then(Value::as_str) {
            let pointer = reference
                .strip_prefix('#')
                .ok_or("IFC-033: nonlocal published reference")?;
            let pointer = unfragment(pointer)?;
            if schema.pointer(&pointer).is_none() {
                return Err(format!("IFC-033: dangling {reference}"));
            }
        }
        Ok(())
    })?;
    let (schema, _) = specialize(schema, value, "")?;
    Ok(conduit_loader::compile(&schema)
        .map_err(|e| e.to_string())?
        .is_valid(value))
}

impl Environment {
    pub(crate) fn failure_at(
        &self,
        scope: &str,
        root: &Value,
        pointer: &str,
        value: &Value,
        core: &str,
    ) -> Result<Option<String>> {
        let uri = "urn:conduit:root";
        let mut resources = self.resources(core)?;
        resources.insert(uri.into(), self.contextual_root(scope, root, uri, core)?);
        let mut adapted = BTreeMap::new();
        let mut entry_pointer = pointer.to_owned();
        for (resource_uri, schema) in resources {
            let (schema, redirects) = specialize(&schema, value, &resource_uri)?;
            if resource_uri == uri {
                entry_pointer = redirected(entry_pointer, &redirects);
            }
            adapted.insert(resource_uri, schema);
        }
        let entry = json!({"$ref":format!("{uri}#{}",fragment(&entry_pointer))});
        let validator = formats::options()
            .with_retriever(Supplied(adapted))
            .build(&entry)
            .map_err(|e| e.to_string())?;
        Ok(validator
            .iter_errors(value)
            .next()
            .map(|e| format!("{}; failing schema keyword {}", e, e.schema_path())))
    }
    pub(crate) fn valid_at(
        &self,
        scope: &str,
        root: &Value,
        pointer: &str,
        value: &Value,
        core: &str,
    ) -> Result<bool> {
        Ok(self
            .failure_at(scope, root, pointer, value, core)?
            .is_none())
    }
    pub fn check_profile(&self, scope: &str, schema: &Value, core: &str) -> Result<()> {
        let contract = self.resolve(scope, &format!("{core}#schema.v1"), core)?;
        if let Some(why) =
            self.failure_at(&contract.0, self.schema(&contract)?, "", schema, core)?
        {
            return Err(format!(
                "SUB-005: authored schema is outside the profile: {why}"
            ));
        }
        Ok(())
    }
    pub fn valid(&self, scope: &str, schema: &Value, value: &Value, core: &str) -> Result<bool> {
        self.valid_at(scope, schema, "", value, core)
    }
    fn parts(
        &self,
        mut todo: Vec<(String, Value, Value)>,
        core: &str,
        all_of: bool,
    ) -> Result<Vec<(String, Value, Value)>> {
        let mut out = vec![];
        let mut seen = BTreeSet::new();
        while let Some((scope, root, schema)) = todo.pop() {
            let identity =
                serde_json::to_string(&(&scope, &root, &schema)).map_err(|e| e.to_string())?;
            if !seen.insert(identity) {
                continue;
            }
            if let Some(reference) = schema["$ref"].as_str() {
                if reference == "#" || reference.starts_with("#/") {
                    let sub = root
                        .pointer(&unfragment(&reference[1..])?)
                        .ok_or("SCH-015: dangling pointer")?
                        .clone();
                    todo.push((scope.clone(), root.clone(), sub));
                } else {
                    let c = self.resolve(&scope, reference, core)?;
                    let sub = self.schema(&c)?.clone();
                    todo.push((c.0, sub.clone(), sub));
                }
            }
            if all_of && let Some(branches) = schema["allOf"].as_array() {
                for sub in branches.iter().rev() {
                    todo.push((scope.clone(), root.clone(), sub.clone()));
                }
            }
            out.push((scope, root, schema));
        }
        Ok(out)
    }
    fn default_conflicts(
        &self,
        parts: Vec<(String, Value, Value)>,
        at: &str,
        core: &str,
        seen: &mut BTreeSet<String>,
        out: &mut Vec<Value>,
    ) -> Result<()> {
        let parts = self.parts(parts, core, true)?;
        // Revisiting the same schemas at a deeper instance position adds no new conflict.
        let identity = serde_json::to_string(&parts).map_err(|e| e.to_string())?;
        if !seen.insert(identity) {
            return Ok(());
        }
        let mut defaults = vec![];
        for (_, _, part) in &parts {
            if let Some(default) = part.get("default")
                && !defaults.iter().any(|previous| {
                    conduit_loader::compile(&json!({"const":previous}))
                        .is_ok_and(|v| v.is_valid(default))
                })
            {
                defaults.push(default.clone());
            }
        }
        if defaults.len() > 1 {
            let mut values = defaults.iter().map(Value::to_string).collect::<Vec<_>>();
            values.sort();
            let values = values.join(" and ");
            out.push(diagnostic(
                "SCH-009",
                at,
                &format!("conflicting unconditional defaults at {at}: {values}"),
            ));
        }
        let mut children: BTreeMap<String, Vec<(String, Value, Value)>> = BTreeMap::new();
        for (scope, root, part) in parts {
            for keyword in ["properties", "patternProperties"] {
                if let Some(members) = part[keyword].as_object() {
                    for (name, sub) in members {
                        children
                            .entry(format!("/{keyword}/{}", token(name)))
                            .or_default()
                            .push((scope.clone(), root.clone(), sub.clone()));
                    }
                }
            }
            for keyword in ["items", "additionalProperties"] {
                if let Some(sub) = part.get(keyword) {
                    children.entry(format!("/{keyword}")).or_default().push((
                        scope.clone(),
                        root.clone(),
                        sub.clone(),
                    ));
                }
            }
            if let Some(prefix) = part["prefixItems"].as_array() {
                for (index, sub) in prefix.iter().enumerate() {
                    children
                        .entry(format!("/prefixItems/{index}"))
                        .or_default()
                        .push((scope.clone(), root.clone(), sub.clone()));
                }
            }
        }
        for (path, parts) in children {
            self.default_conflicts(parts, &format!("{at}{path}"), core, &mut seen.clone(), out)?;
        }
        Ok(())
    }
    fn filled(
        &self,
        scope: &str,
        root: &Value,
        schema: &Value,
        value: &Value,
        core: &str,
        depth: usize,
    ) -> Result<Value> {
        if depth > conduit_loader::MAX_DEPTH {
            return Err("SUB-011: default expansion exceeds DL10 nesting limit".into());
        }
        let parts = self.parts(
            vec![(scope.to_owned(), root.clone(), schema.clone())],
            core,
            false,
        )?;
        let mut value = value.clone();
        for (scope, root, part) in parts {
            if let Some(object) = value.as_object_mut() {
                let properties = part.get("properties").and_then(Value::as_object);
                if let Some(properties) = properties {
                    for (name, sub) in properties {
                        if !object.contains_key(name)
                            && let Some(default) = sub.get("default")
                        {
                            object.insert(name.clone(), default.clone());
                        }
                    }
                }
                let patterns = part
                    .get("patternProperties")
                    .and_then(Value::as_object)
                    .into_iter()
                    .flatten()
                    .map(|(pattern, sub)| {
                        Ok((
                            formats::ecma_regex(pattern).map_err(|e| format!("SCH-016: {e}"))?,
                            sub,
                        ))
                    })
                    .collect::<Result<Vec<_>>>()?;
                for (name, held) in object.iter_mut() {
                    let property = properties.and_then(|p| p.get(name));
                    let mut applicable: Vec<&Value> = property.into_iter().collect();
                    applicable.extend(
                        patterns
                            .iter()
                            .filter(|(re, _)| re.find(name).is_some())
                            .map(|(_, sub)| *sub),
                    );
                    if applicable.is_empty()
                        && let Some(sub) = part.get("additionalProperties")
                    {
                        applicable.push(sub);
                    }
                    for sub in applicable {
                        *held = self.filled(&scope, &root, sub, held, core, depth + 1)?;
                    }
                }
            } else if let Some(items) = value.as_array_mut() {
                let prefix = part.get("prefixItems").and_then(Value::as_array);
                for (index, item) in items.iter_mut().enumerate() {
                    let sub = prefix
                        .and_then(|a| a.get(index))
                        .or_else(|| part.get("items"));
                    if let Some(sub) = sub {
                        *item = self.filled(&scope, &root, sub, item, core, depth + 1)?;
                    }
                }
            }
        }
        Ok(value)
    }
    pub fn fill(
        &self,
        scope: &str,
        schema: &Value,
        value: &Value,
        operation_inputs: bool,
        core: &str,
    ) -> Result<Value> {
        if operation_inputs {
            let Some(object) = value.as_object() else {
                return Ok(value.clone());
            };
            let mut value = object.clone();
            if let Some(properties) = schema["properties"].as_object() {
                for (name, sub) in properties {
                    if !value.contains_key(name)
                        && let Some(default) = sub.get("default")
                    {
                        value.insert(name.clone(), default.clone());
                    }
                    if let Some(held) = value.get_mut(name) {
                        *held = self.filled(scope, schema, sub, held, core, 0)?;
                    }
                }
            }
            Ok(Value::Object(value))
        } else {
            self.filled(scope, schema, schema, value, core, 0)
        }
    }
}

/// Schema roots held by actual authored documents; paths also locate admission diagnostics.
pub(crate) fn roots(document: &Value) -> Vec<(String, Value)> {
    let mut roots = vec![];
    let kind = document["kind"].as_str().unwrap_or("");
    match kind {
        "Schema" => roots.push(("/spec".into(), document["spec"].clone())),
        "Type" => {
            if let Some(s) = document["spec"].get("schema") {
                roots.push(("/spec/schema".into(), s.clone()))
            }
        }
        "Operation" => {
            for key in ["inputSchema", "outputSchema"] {
                if let Some(s) = document["spec"].get(key) {
                    roots.push((format!("/spec/{key}"), s.clone()));
                }
            }
        }
        _ => {}
    }
    if kind == "Requirement"
        || (["Agent", "Resource", "Pool", "Interface", "Conduit"].contains(&kind)
            && document.get("from").is_none())
    {
        if let Some(inputs) = document["spec"]["inputs"].as_object() {
            for (name, input) in inputs {
                roots.push((
                    format!("/spec/inputs/{}/schema", token(name)),
                    input["schema"].clone(),
                ));
            }
        }
        if let Some(controls) = document["spec"]["controls"].as_object() {
            for (name, control) in controls {
                if let Some(inputs) = control["inputs"].as_object() {
                    for (input, v) in inputs {
                        roots.push((
                            format!(
                                "/spec/controls/{}/inputs/{}/schema",
                                token(name),
                                token(input)
                            ),
                            v["schema"].clone(),
                        ));
                    }
                }
            }
        }
    }
    roots
}
fn diagnostic(rule: &str, pointer: &str, message: &str) -> Value {
    json!({"ruleId":rule,"level":"error","message":{"text":message},"locations":[{"logicalLocations":[{"fullyQualifiedName":pointer}]}]})
}
impl Environment {
    /// Admission checks after the strict document loader, shared by installation/submission.
    pub fn check_document(&self, scope: &str, document: &Value, core: &str) -> Result<Vec<Value>> {
        let mut diagnostics = vec![];
        // The grammar comes from the specification, supplied as its Schema contract.
        let qualified = self.resolve(scope, &format!("{core}#qualified-reference.v1"), core)?;
        let qualified_schema = self.schema(&qualified)?;
        for (at, mut root) in roots(document) {
            if let Some(defs) = root["$defs"].as_object() {
                for key in defs.keys() {
                    // Copy keys may carry an @version; remove it before checking the qualified form.
                    let (address, name) = key.split_once('#').unwrap_or(("", ""));
                    let (address, version) = address
                        .split_once('@')
                        .map_or((address, None), |(a, v)| (a, Some(v)));
                    let version_ok = if let Some(version) = version {
                        let c = self.resolve(scope, &format!("{core}#version.v1"), core)?;
                        self.valid(&c.0, self.schema(&c)?, &json!(version), core)?
                    } else {
                        true
                    };
                    if version_ok
                        && !name.is_empty()
                        && self.valid(
                            &qualified.0,
                            qualified_schema,
                            &json!(format!("{address}#{name}")),
                            core,
                        )?
                    {
                        diagnostics.push(diagnostic(
                            "IFC-033",
                            &format!("{at}/$defs/{}", token(key)),
                            &format!("reserved contract-copy key `{key}` in root $defs"),
                        ));
                    }
                }
            }
            let original = root.clone();
            let before = diagnostics.len();
            walk(&mut root, "", &mut |map, pointer| {
                if let Some(reference) = map.get("$ref").and_then(Value::as_str) {
                    let failure = if reference == "#" || reference.starts_with("#/") {
                        if original.pointer(&unfragment(&reference[1..])?).is_none() {
                            Some(format!("unresolved local schema pointer {reference}"))
                        } else {
                            None
                        }
                    } else {
                        self.resolve(scope, reference, core)
                            .and_then(|c| self.schema(&c).map(|_| ()))
                            .err()
                    };
                    if let Some(why) = failure {
                        diagnostics.push(diagnostic(
                            "SCH-015",
                            &format!("{at}{pointer}/$ref"),
                            &why,
                        ));
                    }
                }
                Ok(())
            })?;
            if diagnostics.len() != before {
                continue;
            }
            let mut conflicts = vec![];
            walk(&mut root, "", &mut |map, pointer| {
                self.default_conflicts(
                    vec![(
                        scope.to_owned(),
                        original.clone(),
                        Value::Object(map.clone()),
                    )],
                    &format!("{at}{pointer}"),
                    core,
                    &mut BTreeSet::new(),
                    &mut conflicts,
                )?;
                Ok(())
            })?;
            for conflict in conflicts {
                if !diagnostics.contains(&conflict) {
                    diagnostics.push(conflict);
                }
            }
            walk(&mut root, "", &mut |map, pointer| {
                if let Some(default) = map.get("default") {
                    let filled = self.filled(
                        scope,
                        &original,
                        &Value::Object(map.clone()),
                        default,
                        core,
                        0,
                    )?;
                    if let Some(why) = self.failure_at(scope, &original, pointer, &filled, core)? {
                        diagnostics.push(diagnostic(
                            "SCH-019",
                            &format!("{at}{pointer}/default"),
                            &format!("default does not satisfy its schema: {why}"),
                        ));
                    }
                }
                if let Some(pattern) = map.get("pattern").and_then(Value::as_str)
                    && let Err(why) = formats::ecma_regex(pattern)
                {
                    diagnostics.push(diagnostic(
                        "SCH-016",
                        &format!("{at}{pointer}/pattern"),
                        &format!("{pattern}: {why}"),
                    ));
                }
                if let Some(patterns) = map.get("patternProperties").and_then(Value::as_object) {
                    for pattern in patterns.keys() {
                        if let Err(why) = formats::ecma_regex(pattern) {
                            diagnostics.push(diagnostic(
                                "SCH-016",
                                &format!("{at}{pointer}/patternProperties/{}", token(pattern)),
                                &format!("{pattern}: {why}"),
                            ));
                        }
                    }
                }
                Ok(())
            })?;
            // Template/Requirement input defaults live beside their schema.
            if let Some(input) =
                document.pointer(at.strip_suffix("/schema").unwrap_or("/nonexistent"))
                && let Some(default) = input.get("default")
            {
                let filled = self.fill(scope, &original, default, false, core)?;
                if let Some(why) = self.failure_at(scope, &original, "", &filled, core)? {
                    diagnostics.push(diagnostic(
                        "SCH-019",
                        &format!("{}/default", at.trim_end_matches("/schema")),
                        &format!("input default does not satisfy its schema: {why}"),
                    ));
                }
            }
        }
        Ok(diagnostics)
    }
}

pub(crate) fn operation(
    target: &Value,
    operation: &str,
    core: &str,
) -> Result<(Environment, String, Value)> {
    let env: Environment =
        serde_json::from_value(target["schema_environment"].clone()).map_err(|e| e.to_string())?;
    let contract = env.resolve(
        target["contract_scope"]
            .as_str()
            .ok_or("missing contract scope")?,
        operation,
        core,
    )?;
    let doc = env.scope(&contract.0)?.contracts[&contract.1].clone();
    Ok((env, contract.0, doc))
}

/// NAM-026 compares selections in the Capability's own held scope.
pub(crate) fn capability_includes(
    target: &Value,
    capability: &Value,
    operation: &str,
    core: &str,
) -> Result<bool> {
    let (env, scope, contract) = self::operation(
        target,
        capability.as_str().ok_or("missing Capability")?,
        core,
    )?;
    if contract["kind"] != "Capability" {
        return Err("Conduit contract is not a Capability".into());
    }
    let wanted = env.resolve(
        target["contract_scope"]
            .as_str()
            .ok_or("missing contract scope")?,
        operation,
        core,
    )?;
    for member in contract["spec"]["operations"]
        .as_array()
        .into_iter()
        .flatten()
    {
        if env.resolve(
            &scope,
            member.as_str().ok_or("invalid Capability operation")?,
            core,
        )? == wanted
        {
            return Ok(true);
        }
    }
    Ok(false)
}

// Keep IFC-012 exclusions identical for publication and IFC-014's would-present check.
pub(crate) fn publication_omission(
    target: &Value,
    operation: &str,
    context: &Context,
) -> Result<Option<(&'static str, &'static str)>> {
    if operation == context.qualified("get-secret.v1") {
        return Ok(Some(("SEC-007", "refused")));
    }
    let (_, _, doc) = match self::operation(target, operation, &context.core_package) {
        Ok(c) => c,
        Err(e) if e.starts_with("SCH-015: unresolved") => return Ok(Some(("NAM-023", "refused"))),
        Err(e) => return Err(e),
    };
    if doc["kind"] != "Operation" {
        return Err("tool contract is not an Operation".into());
    }
    if doc["spec"].get("changes").is_some()
        && (!target["type_controls"]
            .as_array()
            .is_some_and(|a| a.contains(&json!(operation)))
            || target
                .get("controls")
                .is_some_and(|m| m.get(operation).is_none()))
    {
        return Ok(Some(("TPL-013", "invalid")));
    }
    Ok(None)
}

pub fn publish(
    state: &State,
    consumer: &str,
    conduit: &str,
    operation: &str,
    context: &Context,
) -> Result<(Value, Decision)> {
    let view = crate::access::publication_state(state, context)?;
    let state = &view;
    let path = state
        .objects
        .iter()
        .find(|o| o["id"] == conduit)
        .ok_or("unknown Conduit")?;
    if path["kind"] != "Conduit"
        || path["consumer"] != consumer
        || path["open"] != true
        || path["state"] != "ready"
        || !path["operations"]
            .as_array()
            .is_some_and(|ops| ops.contains(&json!(operation)))
    {
        return Ok((
            json!({"tool":null,"reason":"refused"}),
            decision(
                "omit-unavailable-tool",
                &["IFC-012", "ACC-001"],
                &[],
                json!({"conduit":conduit}),
                vec![],
            ),
        ));
    }
    let target = state
        .objects
        .iter()
        .find(|o| o["id"] == path["target"])
        .ok_or("missing target")?;
    if let Some((rule, reason)) = publication_omission(target, operation, context)? {
        let (action, rules): (&str, &[&str]) = match rule {
            "SEC-007" => ("omit-secret-tool", &["IFC-012", "SEC-007"]),
            "NAM-023" => ("omit-missing-contract", &["IFC-033", "NAM-023"]),
            _ => ("omit-absent-control", &["IFC-012", "TPL-013"]),
        };
        return Ok((
            json!({"tool":null,"reason":reason}),
            decision(action, rules, &[], json!({"operation":operation}), vec![]),
        ));
    }
    let (env, contract_scope, doc) = self::operation(target, operation, &context.core_package)?;
    let scope = target["contract_scope"]
        .as_str()
        .ok_or("missing target contract scope")?;
    let contract = Contract(
        contract_scope,
        doc["metadata"]["name"]
            .as_str()
            .ok_or("missing operation name")?
            .into(),
    );
    let spec = &doc["spec"];
    let control = spec.get("changes").is_some();
    let mut input = spec
        .get("inputSchema")
        .cloned()
        .unwrap_or(json!({"type":"object"}));
    let mut todo = BTreeSet::new();
    env.rewrite(
        &mut input,
        &contract.0,
        "",
        &context.core_package,
        &mut todo,
    )?;
    if control {
        let relocations: BTreeMap<String, String> = target["controls"][operation]["inputs"]
            .as_object()
            .into_iter()
            .flat_map(|m| m.keys())
            .map(|name| {
                let from = format!("/properties/{}", token(name));
                (from.clone(), format!("{from}/allOf/0"))
            })
            .collect();
        walk(&mut input, "", &mut |map, _| {
            if let Some(reference) = map.get("$ref").and_then(Value::as_str) {
                let pointer = unfragment(reference.strip_prefix('#').ok_or("nonlocal reference")?)?;
                for (from, to) in &relocations {
                    if pointer == *from || pointer.starts_with(&format!("{from}/")) {
                        map.insert(
                            "$ref".into(),
                            json!(format!(
                                "#{}",
                                fragment(&format!("{to}{}", &pointer[from.len()..]))
                            )),
                        );
                        break;
                    }
                }
            }
            Ok(())
        })?;
        input["required"] = json!(["revision"]);
        if let Some(inputs) = target["controls"][operation]["inputs"].as_object() {
            for (name, narrowing) in inputs {
                let a = input["properties"][name].clone();
                let template_scope = target["template_scope"].as_str().unwrap_or(scope);
                let mut b = narrowing["schema"].clone();
                env.rewrite(
                    &mut b,
                    template_scope,
                    &format!("/properties/{}/allOf/1", token(name)),
                    &context.core_package,
                    &mut todo,
                )?;
                input["properties"][name] = json!({"allOf":[a,b]});
            }
        }
    }
    let input = env.finish_bundle(input, todo, &context.core_package)?;
    let mut tool = json!({"name":format!("{}.{}",path["local_name"].as_str().ok_or("missing local name")?,contract.1),
        "inputSchema":input,"annotations":{}});
    if let Some(output) = spec.get("outputSchema") {
        tool["outputSchema"] = env.publish_schema(&contract.0, output, &context.core_package)?;
    }
    // IFC-033: the held Operation and Type describe the actual target, in this order.
    let ty = target["type"].as_str().ok_or("missing target Type")?;
    let type_contract = env.resolve(scope, ty, &context.core_package)?;
    let type_doc = &env.scope(&type_contract.0)?.contracts[&type_contract.1];
    let mut descriptions = vec![];
    if let Some(description) = doc["metadata"]["description"].as_str() {
        descriptions.push(description.to_owned());
    }
    descriptions.push(format!(
        "Target: {} {}, of Type {}.",
        target["kind"].as_str().ok_or("missing target kind")?,
        target["name"].as_str().ok_or("missing target name")?,
        ty
    ));
    if let Some(description) = type_doc["metadata"]["description"].as_str() {
        descriptions.push(description.to_owned());
    }
    tool["description"] = json!(descriptions.join("\n\n"));
    if spec.get("effect").is_none() && !control && spec["startsModelWork"] != true {
        tool["annotations"]["readOnlyHint"] = json!(true);
    }
    for hint in ["idempotentHint", "destructiveHint", "openWorldHint"] {
        if let Some(v) = spec.get(hint) {
            tool["annotations"][hint] = v.clone();
        }
    }
    Ok((
        json!({"tool":tool}),
        decision(
            "publish-standalone-tool",
            &["IFC-033", "IFC-011", "NAM-023", "SCH-015", "TPL-011"],
            &["DL12"],
            json!({"operation":operation,"scope":contract.0}),
            vec![],
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standalone_validation_refuses_nonlocal_and_dangling_references() {
        for reference in [
            "https://example.org/schema",
            "example.org/p#schema.v1",
            "#/$defs/absent",
        ] {
            assert!(standalone_valid(&json!({"$ref":reference}), &json!({})).is_err());
        }
    }
}
