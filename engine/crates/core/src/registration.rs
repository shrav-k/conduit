//! Atomic registration. All allocation and machine evidence is supplied by the boundary.
use crate::packages::{self, Catalog, Project, Snapshot};
use crate::schemas::Environment;
use crate::{
    Allocation, Context, Evidence, Request, Result, State, Transition, authorized_path, decision,
    end_request, permits,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

#[path = "registration_values.rs"]
mod installation_values;
use installation_values::{ValueOrigins, ValueTrace};
pub(crate) use installation_values::{restricted_replacement, restricted_submission};

type Indexes = BTreeMap<String, BTreeMap<String, Value>>;
#[derive(Clone, Serialize, Deserialize)]
struct Held {
    environment: Environment,
    indexes: Indexes,
    contexts: Value,
    scope: String,
    // Freeze the input schemas governing instance destinations, even if the template
    // is later removed. LIF-021 needs these as well as the Type's config schema.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    instance_inputs: BTreeMap<String, Value>,
}
#[derive(Debug)]
struct Fault {
    rule: &'static str,
    text: String,
    pointer: Option<FaultPointer>,
}

#[derive(Debug)]
enum FaultPointer {
    Instance(String),
    Configuration(String),
    Authored(DocumentPlace, Vec<String>),
    ValueNode,
}

impl Fault {
    fn at_argument(mut self, argument: &str) -> Self {
        self.pointer = Some(FaultPointer::Instance(format!(
            "/{}",
            crate::schemas::token(argument)
        )));
        self
    }

    fn at_configuration(mut self, pointer: &str) -> Self {
        self.pointer = Some(FaultPointer::Configuration(format!("/config{pointer}")));
        self
    }
}
type Checked<T> = std::result::Result<T, Fault>;
fn fault(rule: &'static str, text: impl Into<String>) -> Fault {
    Fault {
        rule,
        text: text.into(),
        pointer: None,
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
// Keep the deciding rule apart from the public diagnostic (record.v1 / IFC-050).
// Loader faults arrive as public diagnostics; capture their rule on admission, before
// any registration diagnostic is serialized or rewritten.
#[derive(Clone, PartialEq)]
pub(crate) struct ReportedFault {
    pub(crate) rule: String,
    pub(crate) diagnostic: Value,
    pub(crate) prior_sort_key: Option<String>,
}
impl ReportedFault {
    pub(crate) fn received(diagnostic: Value) -> Result<Self> {
        let rule = diagnostic["ruleId"]
            .as_str()
            .ok_or("incoming registration fault lacks its rule")?
            .into();
        Ok(Self {
            rule,
            diagnostic,
            prior_sort_key: None,
        })
    }

    fn catalog(diagnostic: Value) -> Result<Self> {
        let mut fault = Self::received(diagnostic)?;
        let mut prior = fault.diagnostic.clone();
        if prior["properties"]["document"].is_string() {
            let properties = prior["properties"].as_object_mut().unwrap();
            properties.remove("document");
            if properties.is_empty() {
                prior.as_object_mut().unwrap().remove("properties");
            }
        } else if prior
            .pointer("/locations/0/logicalLocations/0/fullyQualifiedName")
            .and_then(Value::as_str)
            .is_some_and(|location| location.starts_with("/objects/"))
        {
            prior.as_object_mut().unwrap().remove("locations");
        }
        fault.prior_sort_key = Some(prior.to_string());
        Ok(fault)
    }
}
impl std::ops::Deref for ReportedFault {
    type Target = Value;
    fn deref(&self) -> &Value {
        &self.diagnostic
    }
}
impl Serialize for ReportedFault {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        self.diagnostic.serialize(serializer)
    }
}
fn diagnostic(rule: &str, name: &str, text: &str, level: &str) -> ReportedFault {
    ReportedFault {
        rule: rule.into(),
        diagnostic: json!({"ruleId":rule,"level":level,"message":{"text":format!("{name}: {text}")},"locations":[{"logicalLocations":[{"fullyQualifiedName":format!("/objects/{}", name.replace('~', "~0").replace('/', "~1"))}]}]}),
        prior_sort_key: None,
    }
}

fn package_item(context: &Value) -> Value {
    let mut item = json!({"address":context["address"]});
    for key in ["version", "digest"] {
        if let Some(value) = context.get(key) {
            item[key] = value.clone();
        }
    }
    item
}

pub fn inspect_project_documents(object: &Value, packages: &mut Value) -> Result<()> {
    let Some(project) = object.pointer("/held/contexts/project") else {
        return Ok(());
    };
    let (Some(address), Some(digest)) = (project["address"].as_str(), project["digest"].as_str())
    else {
        return Ok(());
    };
    let Some(package) = packages.as_array_mut().and_then(|items| {
        items
            .iter_mut()
            .find(|item| item["address"] == address && item["digest"] == digest)
    }) else {
        return Ok(());
    };
    let Some(index) = object
        .pointer("/held/indexes/project")
        .and_then(Value::as_object)
    else {
        return Ok(());
    };
    let mut documents: Vec<_> = index
        .values()
        .filter(|doc| packages::contract(doc))
        .collect();
    documents.sort_by_key(|doc| (doc["kind"].as_str(), doc["metadata"]["name"].as_str()));
    let entries = documents
        .into_iter()
        .map(|doc| {
            Ok(json!({
                "kind": doc["kind"],
                "name": doc["metadata"]["name"],
                "digest": packages::document_digest(doc)?,
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    package["documents"] = Value::Array(entries);
    Ok(())
}

fn inspected_item(item: &Value) -> Value {
    fn mask_input(input: &mut Value) {
        if input["isSecret"] == true {
            for field in ["value", "default"] {
                if let Some(literal) = input.get_mut(field) {
                    *literal = json!("(secret: not shown)");
                }
            }
        }
    }

    fn mask_variables(parent: &mut Value) {
        if let Some(variables) = parent.get_mut("variables").and_then(Value::as_object_mut) {
            variables.values_mut().for_each(mask_input);
        }
    }

    fn mask_input_list(parent: &mut Value, field: &str) {
        if let Some(inputs) = parent.get_mut(field).and_then(Value::as_array_mut) {
            for input in inputs {
                mask_input(input);
                mask_variables(input);
            }
        }
    }

    let mut item = item.clone();
    if let Some(package) = item.get_mut("package") {
        for field in [
            "runtimeArguments",
            "packageArguments",
            "environmentVariables",
        ] {
            mask_input_list(package, field);
        }
        if let Some(transport) = package.get_mut("transport") {
            mask_input_list(transport, "headers");
        }
    }
    if let Some(remote) = item.get_mut("remote") {
        mask_input_list(remote, "headers");
        mask_variables(remote);
    }
    item
}

/// TPL-004 and NAM-009: report the item's held scope, not a new resolution.
pub fn inspected_implementations(object: &Value, core: &str) -> Option<Value> {
    if object["kind"] == "Conduit" || object["first_start"] == true {
        return None;
    }
    let contexts = object.pointer("/held/contexts")?.as_object()?;
    let implementations = object.get("resolved_implementations")?.as_object()?;
    let selections = implementations
        .iter()
        .filter_map(|(operation, implementation)| {
            let selected = if implementation["source"] == "engine" {
                json!({
                    "selection": format!("{core}#engine"),
                    "package": package_item(contexts.get("core")?),
                })
            } else {
                let scope = implementation["scope"].as_str()?;
                json!({
                    "selection": implementation["selection"].as_str()?,
                    "package": package_item(contexts.get(scope)?),
                    "item": inspected_item(implementation.get("item")?),
                })
            };
            Some((operation.clone(), selected))
        })
        .collect();
    Some(Value::Object(selections))
}

#[cfg(test)]
#[test]
fn inspection_masks_only_marked_input_literals() {
    let marked = |value: &str, default: &str| json!({"isSecret":true,"value":value,"default":default,"placeholder":"sample"});
    let package = |value: &str, default: &str| {
        let mut argument = marked(value, default);
        argument["type"] = json!("named");
        argument["name"] = json!("--example");
        let mut header = marked(value, default);
        header["name"] = json!("X-Example");
        header["variables"]["nested"] = marked(value, default);
        json!({"implements":["#read.v1"],"package":{
            "registryType":"npm","identifier":"example",
            "transport":{"type":"streamable-http","url":"https://example.org/mcp",
                "headers":[header]},
            "runtimeArguments":[argument],
            "packageArguments":[argument,{"type":"named","name":"--public","isSecret":false,
                "value":"public value","default":"public default"}],
            "environmentVariables":[header,{"name":"WITHOUT_LITERAL","isSecret":true}]
        }})
    };
    let remote = |value: &str, default: &str| {
        let mut header = marked(value, default);
        header["name"] = json!("X-Example");
        header["variables"]["nested"] = marked(value, default);
        json!({"implements":["#other.v1"],"remote":{
            "type":"streamable-http","url":"https://example.org/mcp",
            "headers":[header], "variables":{"nested":marked(value, default)}
        }})
    };
    let command = json!({"implements":["#command.v1"],"command":["example","public argument"]});
    let object = json!({"kind":"Resource","held":{"contexts":{"project":{"address":"example.org/demo"}}},
    "resolved_implementations":{
        "example.org/demo#read.v1":{"selection":"example.org/demo#reader","scope":"project",
            "item":package("sample value", "sample default")},
        "example.org/demo#other.v1":{"selection":"example.org/demo#other","scope":"project",
            "item":remote("sample value", "sample default")},
        "example.org/demo#command.v1":{"selection":"example.org/demo#command","scope":"project",
            "item":command}
    }});
    let inspection =
        inspected_implementations(&object, "example.org/fixture-engine-package").unwrap();
    let marker = "(secret: not shown)";
    assert!(
        inspection["example.org/demo#read.v1"]["item"] == package(marker, marker),
        "inspection must mask marked Package input literals and preserve the rest of the item"
    );
    assert!(
        inspection["example.org/demo#other.v1"]["item"] == remote(marker, marker),
        "inspection must mask marked Remote input literals and preserve the rest of the item"
    );
    assert_eq!(inspection["example.org/demo#command.v1"]["item"], command);
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
        let packages = self
            .package_scopes(spec, kind, implementations, core)?
            .into_keys()
            .map(|scope| package_item(&self.contexts[&scope]))
            .collect::<Vec<_>>();
        Ok(json!(packages))
    }
    fn package_scopes(
        &self,
        spec: &Value,
        kind: &str,
        implementations: &Value,
        core: &str,
    ) -> Result<BTreeMap<String, Option<String>>> {
        let mut selections = vec![];
        packages::selections_document(
            &json!({"kind":kind,"spec":spec}),
            self.scope != "project",
            &mut selections,
        )?;
        // Direct selections take precedence over a project contract that reaches the same scope.
        let mut scopes = BTreeMap::new();
        let mut direct = selections.into_iter();
        let mut project = None;
        let mut project_selections = Vec::new();
        let mut visited = BTreeSet::new();
        while let Some((source, (reference, kind), contract)) = direct
            .next()
            .map(|selection| (self, selection, None))
            .or_else(|| {
                let (selection, contract) = project_selections.pop()?;
                Some((project.as_ref()?, selection, contract))
            })
        {
            let Ok((held, document, qualified)) = source.select(&reference, &kind, core) else {
                continue;
            };
            scopes.entry(held.scope.clone()).or_insert(contract);
            // A release's digest already covers its own pins.
            if held.scope == "project" && packages::contract(&document) && visited.insert(qualified)
            {
                let contract = document["kind"]
                    .as_str()
                    .zip(document["metadata"]["name"].as_str())
                    .map(|(kind, name)| format!("{kind} {name}"));
                let mut selections = Vec::new();
                packages::selections_document(&document, false, &mut selections)?;
                project_selections.extend(
                    selections
                        .into_iter()
                        .map(|selection| (selection, contract.clone())),
                );
                project.get_or_insert(held);
            }
        }
        for implementation in implementations
            .as_object()
            .into_iter()
            .flat_map(|m| m.values())
        {
            if let Some(scope) = implementation["scope"].as_str() {
                scopes.insert(scope.into(), None);
            }
            if implementation["source"] == "engine" {
                scopes.insert("core".into(), None);
            }
        }
        Ok(scopes)
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
        && leaf == i.to_string()
        && let Some(item) = array.get_mut(i)
    {
        *item = value;
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
    diagnostics: Vec<ReportedFault>,
    sources: BTreeMap<String, InstanceSource>,
    templates: Vec<Value>,
    reserved_names: BTreeSet<(String, String)>,
    snapshots: Vec<Snapshot>,
    trusted: bool,
    replacement_declaration: Option<DocumentPlace>,
    submitted_resources: BTreeMap<String, installation_values::SubmittedResource>,
    defer_children: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DocumentPlace {
    name: String,
    pointer: String,
}

impl DocumentPlace {
    fn qualified(&self, suffix: &str) -> String {
        format!(
            "/objects/{}{}{suffix}",
            crate::schemas::token(&self.name),
            self.pointer
        )
    }

    fn locate(&self, mut diagnostic: ReportedFault, suffix: &str) -> ReportedFault {
        // Added locations must not change which existing rule decides the reply.
        diagnostic.prior_sort_key = Some(diagnostic.diagnostic.to_string());
        packages::object_location(
            &mut diagnostic.diagnostic,
            &self.name,
            &format!("{}{suffix}", self.pointer),
        );
        diagnostic
    }
}

fn related_locations(diagnostic: &mut ReportedFault, pointers: impl IntoIterator<Item = String>) {
    if let Some(locations) = diagnostic.diagnostic["locations"].as_array_mut() {
        locations.extend(
            pointers
                .into_iter()
                .map(|pointer| json!({"logicalLocations":[{"fullyQualifiedName":pointer}]})),
        );
    }
}

fn authored_fault(
    label: &str,
    mut failure: Fault,
    template_name: &str,
    declaration: Option<&DocumentPlace>,
    replacement: Option<&DocumentPlace>,
) -> ReportedFault {
    if let Some(FaultPointer::Authored(place, pointers)) = failure.pointer {
        if let Some(holder) = replacement {
            if let Some(receiving) = pointers.iter().find_map(|pointer| {
                if let Some(input) = pointer.strip_prefix("/inputs/") {
                    Some(format!("input {input}"))
                } else if pointer != "/inputs"
                    && pointer != "/placement"
                    && !pointer.starts_with("/objects/")
                {
                    Some(format!("destination {pointer}"))
                } else {
                    None
                }
            }) {
                failure
                    .text
                    .push_str(&format!(" ({receiving} in template {template_name})"));
            }
            failure.text.push_str(&format!(
                "; the replacement takes the value held at registration: replace {template_name} so it accepts it, or register {} again to resolve its values anew",
                holder.name
            ));
        }
        let mut reported = place.locate(
            diagnostic(failure.rule, &place.name, &failure.text, "error"),
            "",
        );
        let definition = DocumentPlace {
            name: template_name.to_owned(),
            pointer: "/spec".into(),
        };
        related_locations(
            &mut reported,
            pointers.into_iter().map(|pointer| {
                if pointer.starts_with("/objects/") {
                    pointer
                } else if pointer == "/placement"
                    || installation_values::below(&pointer, "/arguments").is_some()
                {
                    declaration.map_or_else(
                        || format!("/objects/{}{pointer}", crate::schemas::token(label)),
                        |declaration| declaration.qualified(&pointer),
                    )
                } else {
                    definition.qualified(&pointer)
                }
            }),
        );
        return reported;
    }
    diagnostic(failure.rule, label, &failure.text, "error")
}

struct InstanceSource {
    declaration: Option<DocumentPlace>,
    template: Option<String>,
    template_name: String,
    authored_arguments: BTreeSet<String>,
    input_values: ValueOrigins,
    destinations: ValueOrigins,
    value_node_written: Option<installation_values::WrittenNode>,
}

impl Builder<'_> {
    fn nested_value_place(&self, parent: &str, slot: &str, name: &str) -> Option<DocumentPlace> {
        Some(DocumentPlace {
            name: self.sources.get(parent)?.template_name.clone(),
            pointer: format!("/spec/{slot}/{}", crate::schemas::token(name)),
        })
    }

    fn declaration_place(
        &self,
        document: &Value,
        parent: Option<(&Value, &str, &str)>,
    ) -> Option<DocumentPlace> {
        if let Some((parent, slot, name)) = parent {
            let Some(source) = self.sources.get(parent["id"].as_str()?) else {
                // A replacement's holder already exists outside this expansion.
                // Its held slot is the declaration, never the refused instance.
                return self.replacement_declaration.clone();
            };
            if let Some(template) = &source.template {
                return Some(DocumentPlace {
                    name: template.clone(),
                    pointer: format!("/spec/{slot}/{}", crate::schemas::token(name)),
                });
            }
            return source.declaration.clone();
        }
        Some(DocumentPlace {
            name: document["metadata"]["name"].as_str()?.to_owned(),
            pointer: String::new(),
        })
    }

    fn instance_diagnostic(&self, object: &Value, fault: Fault) -> ReportedFault {
        let name = object["name"].as_str().unwrap_or("unnamed");
        let source = object["id"].as_str().and_then(|id| self.sources.get(id));
        if matches!(fault.pointer, Some(FaultPointer::Authored(..))) {
            let nested = &object["nested"];
            let declaration = (|| {
                self.nested_value_place(
                    nested["parent"].as_str()?,
                    nested["slot"].as_str()?,
                    nested["name"].as_str()?,
                )
            })();
            return authored_fault(
                name,
                fault,
                source.map_or(name, |source| source.template_name.as_str()),
                declaration
                    .as_ref()
                    .or_else(|| source.and_then(|source| source.declaration.as_ref())),
                self.replacement_declaration.as_ref(),
            );
        }
        if matches!(fault.pointer, Some(FaultPointer::ValueNode)) {
            let mut diagnostic =
                if let Some(node) = source.and_then(|source| source.value_node_written.as_ref()) {
                    node.place
                        .locate(diagnostic(fault.rule, name, &fault.text, "error"), "")
                } else {
                    self.instance_diagnostic(object, fault.at_configuration("/node"))
                };
            related_locations(&mut diagnostic, ["/config/node".to_owned()]);
            return diagnostic;
        }
        let label = if fault.rule == "CTR-015" {
            source.map_or(name, |source| {
                let authored = match &fault.pointer {
                    Some(FaultPointer::Configuration(pointer)) => {
                        items(object, "_inputs").iter().any(|(input, spec)| {
                            source.authored_arguments.contains(input)
                                && list(spec, "to").iter().any(|destination| {
                                    pointer.strip_prefix(destination).is_some_and(|suffix| {
                                        suffix.is_empty() || suffix.starts_with('/')
                                    })
                                })
                        })
                    }
                    _ => false,
                };
                if authored {
                    source
                        .declaration
                        .as_ref()
                        .map_or(name, |place| place.name.as_str())
                } else {
                    source.template_name.as_str()
                }
            })
        } else {
            name
        };
        let diagnostic = diagnostic(fault.rule, label, &fault.text, "error");
        let Some(source) = source else {
            return diagnostic;
        };
        match fault.pointer {
            Some(FaultPointer::Configuration(pointer)) => {
                if let Some(declaration) = &source.declaration {
                    for (input, spec) in items(object, "_inputs") {
                        if source.authored_arguments.contains(&input)
                            && let Some(suffix) = list(&spec, "to").iter().find_map(|destination| {
                                pointer
                                    .strip_prefix(destination.as_str())
                                    .filter(|p| p.is_empty() || p.starts_with('/'))
                            })
                        {
                            return declaration.locate(
                                diagnostic,
                                &format!("/arguments/{}{suffix}", crate::schemas::token(&input)),
                            );
                        }
                    }
                }
                if let Some(template) = &source.template {
                    return DocumentPlace {
                        name: template.clone(),
                        pointer: "/spec".into(),
                    }
                    .locate(diagnostic, &pointer);
                }
            }
            Some(FaultPointer::Instance(pointer)) => {
                if let Some(declaration) = &source.declaration {
                    return declaration.locate(diagnostic, &pointer);
                }
            }
            _ => {}
        }
        if let Some(declaration) = &source.declaration
            && !declaration.pointer.is_empty()
        {
            return declaration.locate(diagnostic, "");
        }
        diagnostic
    }

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
        let held = serde_json::from_value::<Held>(doc["held"].clone());
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
                let mut message = format!(
                    "required package {} version {} ({}) is not installed",
                    package["address"].as_str().unwrap_or(""),
                    package["version"].as_str().unwrap_or(""),
                    package["digest"].as_str().unwrap_or("")
                );
                let contract = held.as_ref().ok().and_then(|h| {
                    h.package_scopes(
                        &doc["spec"],
                        kind,
                        &doc["resolved_implementations"],
                        &self.context.core_package,
                    )
                    .ok()?
                    .into_iter()
                    .find(|(scope, _)| package_item(&h.contexts[scope]) == *package)
                    .and_then(|(_, contract)| contract)
                });
                if let Some(contract) = contract {
                    message.push_str(&format!(
                        "; template {name} reaches it through its project's {contract}"
                    ));
                }
                return Err(fault("PKG-007", message));
            }
        }
        let h = held.map_err(|_| fault("INS-002", "template context missing"))?;
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
            // FLEET-006: reserve the shared Pool/Fleet namespace even for a named
            // instance that canonical traversal has not expanded yet.
            if !self
                .reserved_names
                .iter()
                .any(|(reserved_kind, reserved_name)| {
                    reserved_name == &name
                        && (reserved_kind == kind
                            || (["Pool", "Fleet"].contains(&kind)
                                && ["Pool", "Fleet"].contains(&reserved_kind.as_str())))
                })
                && !self
                    .state
                    .objects
                    .iter()
                    .chain(&self.objects)
                    .chain(&self.templates)
                    .any(|o| {
                        o["name"] == name
                            && (o["kind"] == kind
                                || (["Pool", "Fleet"].contains(&kind)
                                    && o.get("spec").is_none()
                                    && ["Pool", "Fleet"].iter().any(|k| o["kind"] == *k)))
                    })
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
        let declaration = self.declaration_place(document, parent);
        let expanded_fault = |rule, name, text, suffix: &str| {
            let diagnostic = diagnostic(rule, name, text, "error");
            if parent.is_some()
                && let Some(declaration) = &declaration
            {
                return declaration.locate(diagnostic, suffix);
            }
            diagnostic
        };
        if depth > 128 {
            self.diagnostics.push(expanded_fault(
                "SUB-011",
                "nested instantiation",
                "DL10 nesting 128 exceeded",
                "",
            ));
            return Ok(());
        }
        let kind = document["kind"].as_str().ok_or("instance has no family")?;
        let label = document["metadata"]["name"].as_str().unwrap_or("unnamed");
        // FLEET-001: no alternative template or standard/custom arguments exist.
        if kind == "Fleet"
            && (document["from"] != self.context.qualified("fleet")
                || document.as_object().is_none_or(|m| {
                    m.keys()
                        .any(|k| !["apiVersion", "kind", "metadata", "from"].contains(&k.as_str()))
                }))
        {
            self.diagnostics.push(expanded_fault(
                "FLEET-001",
                label,
                "Fleet must instantiate core#fleet with no arguments",
                "",
            ));
            return Ok(());
        }

        let (template, h, from) = match self.template(&document["from"], kind, held, true) {
            Ok(t) => t,
            Err(f) => {
                self.diagnostics
                    .push(expanded_fault(f.rule, label, &f.text, "/from"));
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
            self.diagnostics.push(expanded_fault(
                "FST-006",
                label,
                "protected core template",
                "/from",
            ));
            return Ok(());
        }
        let fallback = DocumentPlace {
            name: label.to_owned(),
            pointer: String::new(),
        };
        let template_name = template["metadata"]["name"].as_str().unwrap_or(label);
        let definition = DocumentPlace {
            name: template_name.to_owned(),
            pointer: "/spec".into(),
        };
        let nested = parent.and_then(|(owner, slot, name)| {
            self.nested_value_place(owner["id"].as_str()?, slot, name)
        });
        let value_declaration = nested.as_ref().or(declaration.as_ref());
        let applied = match self.apply_inputs(
            document,
            &mut spec,
            &h,
            value_declaration.unwrap_or(&fallback),
            &definition,
            parent,
        ) {
            Ok(applied) => applied,
            Err(f) => {
                let reported = if matches!(f.pointer, Some(FaultPointer::Authored(..))) {
                    authored_fault(
                        label,
                        f,
                        template_name,
                        value_declaration,
                        self.replacement_declaration.as_ref(),
                    )
                } else {
                    expanded_fault(f.rule, label, &f.text, "")
                };
                self.diagnostics.push(reported);
                return Ok(());
            }
        };
        let argument_values = applied.arguments;
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
        let submitted_template = self.templates.iter().find(|candidate| {
            if let Some(id) = from["id"].as_str() {
                candidate["id"] == id
            } else {
                h.scope == "project"
                    && candidate["kind"] == kind
                    && candidate["name"] == template["metadata"]["name"]
            }
        });
        self.sources.insert(
            id.clone(),
            InstanceSource {
                declaration,
                template: submitted_template
                    .and_then(|t| t["name"].as_str())
                    .map(str::to_owned),
                template_name: template_name.to_owned(),
                authored_arguments: document["arguments"]
                    .as_object()
                    .into_iter()
                    .flat_map(|arguments| arguments.keys().cloned())
                    .collect(),
                input_values: applied.values,
                destinations: applied.destinations,
                value_node_written: applied.node_written,
            },
        );
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
        let mut instance_held = h.clone();
        instance_held.instance_inputs =
            serde_json::from_value(inputs.clone()).map_err(|e| e.to_string())?;
        instance_held
            .instance_inputs
            .retain(|name, _| argument_values.get(name).is_some());
        object["held"] = serde_json::to_value(&instance_held).map_err(|e| e.to_string())?;
        object["_inputs"] = inputs;
        object["_argument_values"] = argument_values;
        for key in ["owner", "placement", "runtime", "consumer", "target"] {
            if let Some(v) = document.get(key) {
                object[key] = v.clone();
            }
        }
        if kind == "Conduit" {
            object["open"] = json!(false);
            // OBJ-013 / IFC-012: a standalone Conduit's alias is local to its consumer.
            object["local_name"] = document.get("as").cloned().unwrap_or(json!(name));
        }
        if let Some((owner, slot, local)) = parent {
            object[if slot == "entries" {
                "owner"
            } else {
                "consumer"
            }] = json!({"kind":owner["kind"],"id":owner["id"]});
            object["nested"] = json!({"parent":owner["id"],"slot":slot,"name":local});
            if slot == "conduits" {
                object["local_name"] = json!(local); // IFC-012: the declaration supplies the name.
            }
            if slot == "entries" {
                object["entry_pool"] = owner["id"].clone();
            } else if let Some(pool) = owner.get("entry_pool") {
                // A Pool entry can pass its sibling reference into a declared Conduit.
                object["entry_pool"] = pool.clone();
            }
        }
        let position = self.objects.len();
        self.objects.push(object.clone());
        if !self.defer_children {
            self.expand_children(&object, &h, position, depth)?;
        }
        Ok(())
    }

    fn expand_children(
        &mut self,
        object: &Value,
        held: &Held,
        position: usize,
        depth: usize,
    ) -> Result<()> {
        for (slot, family) in [("conduits", "Conduit"), ("entries", "Resource")] {
            for (local, mut nested) in items(object, slot) {
                // Optional declarations are still instantiated; the mark changes setup failure only.
                nested["kind"] = json!(family);
                let before = self.objects.len();
                self.expand(&nested, held, Some((object, slot, &local)), depth + 1)?;
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
        "fleet-reference.v1" => Some("Fleet"),
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
    collected: Option<&'a RefCell<BTreeSet<String>>>,
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
        self.schema_with_trace(
            h,
            root,
            schema,
            value,
            pool,
            authored,
            depth,
            ValueTrace::default(),
        )
    }

    fn guard_schema(
        &self,
        held: &Held,
        schema: &Value,
        value: &Value,
        origins: &ValueOrigins,
        at: &str,
    ) -> Checked<()> {
        if origins.is_empty() {
            return Ok(());
        }
        self.schema_with_trace(
            held,
            schema,
            schema,
            value,
            None,
            false,
            0,
            ValueTrace::at(origins, at),
        )
        .map(|_| ())
        .map_err(|failure| {
            if matches!(failure.pointer, Some(FaultPointer::Authored(..))) {
                return failure;
            }
            installation_values::affected(origins, at)
                .map(|origin| {
                    origin.failure(
                        failure.rule,
                        "the receiving schema could not be checked",
                        Some(at),
                    )
                })
                .unwrap_or(failure)
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn schema_with_trace(
        &self,
        h: &Held,
        root: &Value,
        schema: &Value,
        value: &Value,
        pool: Option<&Value>,
        authored: bool,
        depth: usize,
        trace: ValueTrace<'_>,
    ) -> Checked<Value> {
        if trace.skip_branch() {
            return Ok(value.clone());
        }
        if depth > 128 {
            return Err(fault("SUB-011", "schema reference nesting 128"));
        }
        let mut result = value.clone();
        if let Some(r) = schema["$ref"].as_str() {
            if let Some(name) = r.strip_prefix(&format!("{}#", self.core))
                && name.ends_with("reference.v1")
                && !["qualified-reference.v1", "template-reference.v1"].contains(&name)
            {
                if trace.check_reference()? {
                    return Ok(result);
                }
                let resolution = self
                    .resolve(value, fixed_kind(name), pool, authored)
                    .map_err(|mut fault| {
                        if name == "secret-reference.v1" {
                            let withheld = conduit_loader::secret::WITHHELD_VALUE;
                            if fault.text.starts_with("unresolved reference ") {
                                // The unresolved-reference note keeps its existing classification.
                                fault.text = format!("unresolved reference: {withheld}");
                            } else if fault.text.starts_with("reference ")
                                || fault.text.starts_with("no entry ")
                            {
                                fault.text = withheld.into();
                            }
                        }
                        fault
                    });
                let resolved = match resolution {
                    Err(f)
                        if f.rule == "OBJ-009"
                            && f.text.contains("unresolved reference")
                            && self.unresolved.is_some() =>
                    {
                        self.unresolved.unwrap().borrow_mut().push(f);
                        Ok(value.clone())
                    }
                    result => result,
                }?;
                if let Some(collected) = self.collected
                    && ["Agent", "Resource"].contains(&resolved["kind"].as_str().unwrap_or(""))
                    && let Some(id) = resolved["id"].as_str()
                {
                    collected.borrow_mut().insert(id.to_owned());
                }
                return Ok(resolved);
            }
            let (scope, s) = h
                .environment
                .schema_reference(&h.scope, root, r, self.core)
                .map_err(|e| fault("SCH-015", e))?;
            let mut h = h.clone();
            h.scope = scope;
            result = self.schema_with_trace(
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
                trace.clone(),
            )?;
        }
        // SCH-001 / OBJ-021: follow the applicators at this instance location.
        // allOf applies every branch; disjunctions apply only matching branches.
        for key in ["allOf", "anyOf", "oneOf"] {
            for branch in schema[key].as_array().into_iter().flatten() {
                if key == "allOf"
                    || h.valid_in(root, branch, value, self.core)
                        .map_err(|e| fault("SCH-015", e))?
                {
                    let resolved = self.schema_with_trace(
                        h,
                        root,
                        branch,
                        value,
                        pool,
                        authored,
                        depth + 1,
                        trace.clone(),
                    )?;
                    overlay(&mut result, value, resolved);
                }
            }
        }
        if let Some(condition) = schema.get("if") {
            let matches = h
                .valid_in(root, condition, value, self.core)
                .map_err(|e| fault("SCH-015", e))?;
            if matches {
                let resolved = self.schema_with_trace(
                    h,
                    root,
                    condition,
                    value,
                    pool,
                    authored,
                    depth + 1,
                    trace.clone(),
                )?;
                overlay(&mut result, value, resolved);
            }
            if let Some(branch) = schema.get(if matches { "then" } else { "else" }) {
                let resolved = self.schema_with_trace(
                    h,
                    root,
                    branch,
                    value,
                    pool,
                    authored,
                    depth + 1,
                    trace.clone(),
                )?;
                overlay(&mut result, value, resolved);
            }
        }
        for (key, branch) in items(schema, "dependentSchemas") {
            if value.as_object().is_some_and(|map| map.contains_key(&key)) {
                let resolved = self.schema_with_trace(
                    h,
                    root,
                    &branch,
                    value,
                    pool,
                    authored,
                    depth + 1,
                    trace.clone(),
                )?;
                overlay(&mut result, value, resolved);
            }
        }
        if let Some(map) = result.as_object_mut() {
            for (key, sub) in items(schema, "properties") {
                if let Some(v) = value.get(&key) {
                    let resolved = self
                        .schema_with_trace(
                            h,
                            root,
                            &sub,
                            v,
                            pool,
                            authored,
                            depth + 1,
                            trace.child(&key),
                        )
                        .map_err(|mut f| {
                            if !matches!(f.pointer, Some(FaultPointer::Authored(..))) {
                                f.text = format!("/{key}: {}", f.text);
                            }
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
                            let resolved = self.schema_with_trace(
                                h,
                                root,
                                &sub,
                                original,
                                pool,
                                authored,
                                depth + 1,
                                trace.child(key),
                            )?;
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
                        let resolved = self.schema_with_trace(
                            h,
                            root,
                            sub,
                            original,
                            pool,
                            authored,
                            depth + 1,
                            trace.child(key),
                        )?;
                        overlay(v, original, resolved);
                    }
                }
            }
        }
        if let Some(array) = result.as_array_mut()
            && let Some(original) = value.as_array()
        {
            let prefix = schema["prefixItems"].as_array();
            for (index, (current, original)) in array.iter_mut().zip(original).enumerate() {
                // items applies only after prefixItems (JSON Schema 2020-12).
                if let Some(sub) = prefix
                    .and_then(|p| p.get(index))
                    .or_else(|| schema.get("items"))
                {
                    let resolved = self.schema_with_trace(
                        h,
                        root,
                        sub,
                        original,
                        pool,
                        authored,
                        depth + 1,
                        trace.child(&index.to_string()),
                    )?;
                    overlay(current, original, resolved);
                }
                if let Some(sub) = schema.get("contains")
                    && h.valid_in(root, sub, original, self.core)
                        .map_err(|e| fault("SCH-015", e))?
                {
                    let resolved = self.schema_with_trace(
                        h,
                        root,
                        sub,
                        original,
                        pool,
                        authored,
                        depth + 1,
                        trace.child(&index.to_string()),
                    )?;
                    overlay(current, original, resolved);
                }
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

/// CTR-015 checks only the forbidden pair, never the meaning of a report.
pub(crate) fn admission_pair(ty: &Value, config: &Value) -> Option<(String, String)> {
    let place = ty["spec"]["admission"]["levers"].as_str()?;
    let levers = config.pointer(place)?;
    (levers["whenUnknown"] == "admit"
        && matches!(
            levers["paidUsage"].as_str(),
            Some("refuse" | "refuse-if-possible")
        ))
    .then(|| {
        (
            place.into(),
            format!("{place}/whenUnknown admit cannot stand beside a refusing {place}/paidUsage"),
        )
    })
}

// A pair fault does not discard the login. Check every other schema error so its
// Agents can resolve it without hiding an unrelated INS-014 failure.
fn admission_other_schema_fault(
    held: &Held,
    schema: &Value,
    config: &Value,
    place: &str,
    ty: &str,
    core: &str,
) -> Checked<()> {
    let published = held
        .environment
        .publish_schema(&held.scope, schema, core)
        .map_err(|e| fault("INS-014", e))?;
    let validator = crate::schemas::standalone_validator(&published, config)
        .map_err(|e| fault("INS-014", e))?;
    // An independent restriction may mention these same fields and words. Keep
    // its error if it survives any single-lever repair of the pair, including
    // either non-admitting whenUnknown value. Compare schema locations, not words.
    let mut surviving = BTreeSet::new();
    for (lever, value) in [
        ("paidUsage", "allow"),
        ("whenUnknown", "refuse"),
        ("whenUnknown", "one-trial"),
    ] {
        let mut repaired = config.clone();
        repaired.pointer_mut(place).expect("forbidden pair exists")[lever] = json!(value);
        surviving.extend(validator.iter_errors(&repaired).map(|error| {
            (
                error.instance_path().to_string(),
                error.schema_path().to_string(),
            )
        }));
    }
    for error in validator.iter_errors(config) {
        let at = error.instance_path().as_str();
        let keyword = error.schema_path().as_str();
        let pair_field = at == format!("{place}/paidUsage") || at == format!("{place}/whenUnknown");
        if pair_field
            && keyword.split('/').any(|part| part == "then")
            && !surviving.contains(&(at.to_owned(), keyword.to_owned()))
        {
            continue;
        }
        return Err(fault(
            "INS-014",
            format!("configuration fails Type {ty}: failing schema keyword {keyword}"),
        )
        .at_configuration(at));
    }
    Ok(())
}

fn admission_login_fault(
    agent: &Value,
    resource: &Value,
    context: &Context,
) -> Checked<Option<Fault>> {
    let core = &context.core_package;
    let rh: Held = serde_json::from_value(resource["held"].clone())
        .map_err(|e| fault("CTR-015", e.to_string()))?;
    let (_, login_type, login_name) =
        rh.select(resource["type"].as_str().unwrap_or(""), "Type", core)?;
    if login_type["spec"].get("admission").is_none() {
        return Ok(None);
    }
    let ah: Held = serde_json::from_value(agent["held"].clone())
        .map_err(|e| fault("CTR-015", e.to_string()))?;
    let (ah, agent_type, agent_name) =
        ah.select(agent["type"].as_str().unwrap_or(""), "Type", core)?;
    for selection in list(&agent_type["spec"]["admission"], "loginTypes") {
        if ah.select(&selection, "Type", core)?.2 == login_name {
            return Ok(None);
        }
    }
    let place = agent["login_resource_place"].as_str().unwrap_or("");
    Ok(Some(fault("CTR-015", format!(
        "Agent Type {agent_name} does not list login Type {login_name} under admission.loginTypes"
    )).at_configuration(place)))
}

/// Apply supplied control values to a trial configuration only (REV-006).
pub(crate) fn admission_control_pair(
    target: &Value,
    contract: &Value,
    supplied: &Value,
    filled: &Value,
    context: &Context,
) -> Result<Option<String>> {
    if target["kind"] != "Resource" || contract["spec"].get("changes").is_none() {
        return Ok(None);
    }
    let ty = crate::inventory::instance_type(target, context)?;
    if ty["spec"].get("admission").is_none() {
        return Ok(None);
    }
    let mut config = target["config"].clone();
    for (input, pointer) in items(&contract["spec"], "changes") {
        if supplied.get(&input).is_some()
            && let Some(pointer) = pointer.as_str()
        {
            if pointer.is_empty() {
                config = filled[&input].clone();
            } else {
                // Missing parents belong to the general control schema check;
                // the levers place is required by this Type's schema.
                let _ = assign(&mut config, pointer, filled[&input].clone());
            }
        }
    }
    Ok(admission_pair(&ty, &config).map(|(_, text)| {
        format!(
            "Type {}: {text}",
            target["type"].as_str().unwrap_or_default()
        )
    }))
}

fn resolve_instance(
    object: &Value,
    all: &[Value],
    context: &Context,
    trusted: bool,
    source: Option<&InstanceSource>,
    admission_faults: &mut Vec<Fault>,
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
        collected: None,
    };
    let kind = o["kind"].as_str().unwrap_or("").to_owned();
    let slot = o["nested"]["slot"].as_str().unwrap_or("").to_owned();
    let pool_value = o.get("entry_pool").cloned();
    let entry_pool = pool_value.as_ref();
    let supplied_placement = o.get("placement").cloned();
    let supplied_runtime = o.get("runtime").cloned();
    let empty = ValueOrigins::new();
    let destinations = source.map_or(&empty, |source| &source.destinations);
    let input_values = source.map_or(&empty, |source| &source.input_values);
    let placement = || {
        refs.resolve(
            supplied_placement.as_ref().unwrap_or(&json!("local")),
            Some("Resource"),
            entry_pool,
            !trusted,
        )
        .map_err(|failure| fault("PLC-001", failure.text).at_argument("placement"))
    };
    let placed_for_values = if input_values.is_empty() {
        None
    } else {
        let placed = matches!(kind.as_str(), "Agent" | "Resource")
            .then(placement)
            .transpose()?;
        for source in input_values.values() {
            source.check_placement(placed.as_ref(), all)?;
        }
        placed
    };
    let mut standards = vec![];
    if kind == "Resource" {
        standards.push(("owner", Some("Pool")));
    }
    // FLEET-002 / OBJ-013: the Agent's fixed owner is a Fleet, never a Pool.
    if kind == "Agent" {
        if o.get("owner").is_none() {
            return Err(
                fault("FLEET-002", "owner is required and must name a Fleet").at_argument("owner"),
            );
        }
        standards.push(("owner", Some("Fleet")));
    }
    if kind == "Conduit" {
        standards.extend([("consumer", None), ("target", None)]);
    }
    for (key, k) in standards {
        let implicit =
            (key == "owner" && slot == "entries") || (key == "consumer" && slot == "conduits");
        o[key] = refs
            .resolve(&o[key], k, entry_pool, !implicit && !trusted)
            .map_err(|f| {
                let fault = if kind == "Agent" && key == "owner" {
                    fault("FLEET-002", format!("owner must name a Fleet: {}", f.text))
                } else {
                    f
                };
                fault.at_argument(key)
            })?;
        let target = all
            .iter()
            .find(|v| v["id"] == o[key]["id"])
            .ok_or_else(|| fault("OBJ-009", "missing referenced instance").at_argument(key))?;
        if key == "consumer" && target["kind"] == "Fleet" {
            return Err(
                fault("FLEET-004", "a Fleet is never a consumer or caller").at_argument(key)
            );
        }
        if key == "owner" && kind == "Agent" && target["kind"] != "Fleet" {
            return Err(fault("FLEET-002", "owner is not a Fleet").at_argument(key));
        }
        if target["state"] == "retired" {
            return Err(fault(
                if key == "owner" {
                    if kind == "Agent" {
                        "FLEET-003"
                    } else {
                        "POOL-004"
                    }
                } else {
                    "CON-025"
                },
                format!("{key} {} is retired", target["name"]),
            )
            .at_argument(key));
        }
        if key == "owner" && kind == "Resource" && target["kind"] != "Pool" {
            return Err(fault("POOL-001", "owner is not a Pool").at_argument(key));
        }
    }
    let mut config_schema = None;
    let mut type_scope = None;
    let mut admission_place = None;
    if kind != "Conduit" {
        let (th, ty, q) = h.select(o["type"].as_str().unwrap_or(""), "Type", core)?;
        o["type"] = json!(q);
        installation_values::validate_owner(&o, &object["owner"], all, context)?;
        let unconstrained = json!({});
        let schema = ty["spec"].get("schema").unwrap_or(&unconstrained);
        let config = th.fill(schema, &o["config"], core).map_err(|e| {
            installation_values::affected(destinations, "/config").map_or_else(
                || fault("INS-014", e),
                |source| {
                    source.failure(
                        "INS-014",
                        "the receiving configuration schema could not be applied",
                        Some("/config"),
                    )
                },
            )
        })?;
        refs.guard_schema(&th, schema, &config, destinations, "/config")?;
        installation_values::validate_node(
            &o,
            &config,
            &refs,
            entry_pool,
            !trusted,
            source.and_then(|source| source.value_node_written.as_ref()),
        )?;
        let pair = (kind == "Resource")
            .then(|| admission_pair(&ty, &config))
            .flatten();
        if let Some((place, text)) = &pair {
            // Attribute the pair to an instance argument supplying either of its
            // participating values, even when that input targets just one lever.
            // Otherwise both instances of a faulty template share its one place.
            let authored_lever = ["whenUnknown", "paidUsage"]
                .into_iter()
                .map(|lever| format!("{place}/{lever}"))
                .find(|pointer| {
                    source.is_some_and(|source| {
                        items(object, "_inputs").iter().any(|(input, spec)| {
                            source.authored_arguments.contains(input)
                                && list(spec, "to").iter().any(|destination| {
                                    format!("/config{pointer}")
                                        .strip_prefix(destination)
                                        .is_some_and(|suffix| {
                                            suffix.is_empty() || suffix.starts_with('/')
                                        })
                                })
                        })
                    })
                });
            admission_faults.push(
                fault("CTR-015", format!("Type {q}: {text}"))
                    .at_configuration(authored_lever.as_deref().unwrap_or(place)),
            );
            admission_other_schema_fault(&th, schema, &config, place, &q, core)?;
            admission_place = Some(place.clone());
        }
        if pair.is_none()
            && let Some(failure) = th
                .environment
                .failure_at(&th.scope, schema, "", &config, core)
                .map_err(|e| fault("INS-014", e))?
        {
            let pointer = format!("/config{}", failure.instance_pointer);
            if let Some(source) = installation_values::affected(destinations, &pointer) {
                return Err(source.failure(
                    "INS-014",
                    &format!("the receiving Type schema failed ({})", failure.keyword),
                    Some(&pointer),
                ));
            }
            return Err(fault(
                "INS-014",
                format!("configuration fails Type {q}: {failure}"),
            )
            .at_configuration(&failure.instance_pointer));
        }
        o["config"] = refs.schema(&th, schema, schema, &config, entry_pool, !trusted, 0)?;
        if kind == "Agent"
            && let Some(place) = ty["spec"]["loginResource"].get("place")
        {
            o["login_resource_place"] = place.clone();
        }
        if pair.is_none()
            && let Some(failure) = th
                .environment
                .failure_at(&th.scope, schema, "", &o["config"], core)
                .map_err(|e| fault("INS-014", e))?
        {
            return Err(fault("INS-014", "stored configuration fails Type")
                .at_configuration(&failure.instance_pointer));
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
                let at = format!(
                    "/requirements/{}/arguments/{}",
                    crate::schemas::token(&name),
                    crate::schemas::token(&key)
                );
                if let Some(source) = installation_values::affected(destinations, &at) {
                    return Err(source.failure(
                        "INS-003",
                        "the receiving requirement does not declare this input",
                        Some(&at),
                    ));
                }
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
            let at = format!(
                "/requirements/{}/arguments/{}",
                crate::schemas::token(&name),
                crate::schemas::token(&key)
            );
            let provenance = installation_values::affected(destinations, &at);
            let failure_for_requirement = |error: String| {
                provenance.map_or_else(
                    || fault("INS-005", error),
                    |source| {
                        source.failure(
                            "INS-005",
                            "the value does not satisfy the requirement input schema",
                            Some(&at),
                        )
                    },
                )
            };
            let value = rh
                .fill(&input["schema"], value, core)
                .map_err(failure_for_requirement)?;
            refs.guard_schema(&rh, &input["schema"], &value, destinations, &at)?;
            if let Some(failure) = rh
                .environment
                .failure_at(&rh.scope, &input["schema"], "", &value, core)
                .map_err(failure_for_requirement)?
            {
                let pointer = format!("{at}{}", failure.instance_pointer);
                if let Some(source) = installation_values::affected(destinations, &pointer) {
                    return Err(source.failure(
                        "INS-005",
                        &format!("the requirement input schema failed ({})", failure.keyword),
                        Some(&pointer),
                    ));
                }
                if provenance.is_some() {
                    return Err(fault(
                        "INS-005",
                        format!(
                            "requirement {name}, input {key}: the requirement input schema failed ({}) at {pointer}",
                            failure.keyword
                        ),
                    ));
                }
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
    if let (Some(schema), Some(th)) = (&config_schema, &type_scope) {
        if let Some(place) = &admission_place {
            admission_other_schema_fault(
                th,
                schema,
                &o["config"],
                place,
                o["type"].as_str().unwrap_or(""),
                core,
            )?;
        } else if let Some(failure) = th
            .environment
            .failure_at(&th.scope, schema, "", &o["config"], core)
            .map_err(|e| fault("INS-014", e))?
        {
            return Err(fault("INS-014", "stored configuration fails Type")
                .at_configuration(&failure.instance_pointer));
        }
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
        let placement = placed_for_values.map_or_else(placement, Ok)?;
        let node = all
            .iter()
            .find(|n| n["id"] == placement["id"])
            .ok_or_else(|| fault("PLC-001", "unknown node").at_argument("placement"))?;
        if node["type"] != format!("{core}#node.v1") {
            return Err(fault("PLC-001", "placement is not a node").at_argument("placement"));
        }
        if node["state"] == "retired" {
            return Err(fault("PLC-018", "placement is retired").at_argument("placement"));
        }
        o["placement"] = placement;
        if let Some(runtime) = &supplied_runtime {
            let runtime = refs
                .resolve(runtime, Some("Resource"), entry_pool, !trusted)
                .map_err(|fault| fault.at_argument("runtime"))?;
            let provider = all
                .iter()
                .find(|n| n["id"] == runtime["id"])
                .ok_or_else(|| {
                    fault("PRV-006", "unknown runtime provider").at_argument("runtime")
                })?;
            if provider["kind"] != "Resource"
                || provider["state"] == "retired"
                || !offered(provider, core)
                    .map_err(|fault| fault.at_argument("runtime"))?
                    .contains(&format!("{core}#runtime-provider.v1"))
            {
                return Err(fault(
                    "PRV-006",
                    format!("runtime {runtime} is not a live runtime provider"),
                )
                .at_argument("runtime"));
            }
            o["runtime"] = runtime;
        }
        let implementations = items(&o, "resolved_implementations");
        let running = implementations
            .iter()
            .any(|(_, i)| i["item"].get("command").is_some() || i["item"].get("package").is_some());
        if running {
            let (_, ty, _) = h.select(o["type"].as_str().unwrap(), "Type", core)?;
            let mut selector = ty["spec"].get("nodeSelector").cloned().unwrap_or(json!({}));
            for (_, implementation) in &implementations {
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
            let pair = crate::platform_pair(&node["status"]["labels"]);
            for (_, implementation) in implementations {
                let item = &implementation["item"];
                let platforms = item["platforms"]
                    .as_array()
                    .map(Vec::as_slice)
                    .unwrap_or_default();
                if (item.get("command").is_some() || item.get("package").is_some())
                    && item.get("platforms").is_some()
                    && !pair
                        .as_ref()
                        .is_some_and(|pair| platforms.iter().any(|platform| platform == pair))
                {
                    let name = implementation["selection"]
                        .as_str()
                        .and_then(|selection| selection.rsplit_once('#'))
                        .map_or("implementation", |(_, name)| name);
                    let node_name = node["name"].as_str().unwrap_or("unknown");
                    return Err(fault(
                        "PLC-007",
                        crate::platform_mismatch_message(
                            name,
                            node_name,
                            pair.as_deref(),
                            platforms,
                        ),
                    ));
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
    if !destinations.is_empty() {
        o["_value_sources"] = Value::Object(
            destinations
                .iter()
                .map(|(at, source)| (at.clone(), source.reference()))
                .collect(),
        );
    }
    if kind == "Fleet" {
        // FLEET-001: no configuration is exposed on the instance.
        o.as_object_mut().unwrap().remove("config");
    }
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
) -> Vec<ReportedFault> {
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
            collected: None,
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
    diagnostics.sort_by_key(|d| d.diagnostic.to_string());
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
        if (doc["kind"] != "Resource" && doc["kind"] != "Agent") || doc.get("from").is_none() {
            continue;
        }
        let family = if doc["kind"] == "Agent" {
            "Fleet"
        } else {
            "Pool"
        };
        let reference = doc["owner"]
            .as_str()
            .map(|n| json!({"kind":family,"name":n}))
            .unwrap_or_else(|| doc["owner"].clone());
        if let Some(pool) =
            crate::access::object(&t.state, &reference).filter(|p| p["kind"] == family)
            && pool["state"] != "retired"
        {
            pool_authority &= t
                .state
                .objects
                .iter()
                .any(|c| permits(c, &request.sender, &pool["id"], &request.operation));
        }
    }
    let root = t
        .state
        .objects
        .iter()
        .find(|o| o["from"] == context.qualified("root"))
        .ok_or("root absent")?;
    let root_own = root["id"] == request.sender && root["id"] == request.caller;
    let restricted = (!root_own && permitted)
        .then(|| {
            restricted_submission(
                &t.state,
                &project.documents,
                project.manifest.as_ref(),
                &snapshots,
                context,
            )
        })
        .flatten();
    let rule = if !permitted {
        Some("ACC-001")
    } else if restricted.is_some() {
        Some("ACC-057")
    } else if !pool_authority {
        Some("ACC-033")
    } else {
        None
    };
    if let Some(rule) = rule {
        t.decisions.push(decision(
            "refuse-unauthorized-registration",
            &[rule, "OPS-023"],
            &[],
            json!({}),
            vec![],
        ));
        let route = crate::explain::ids(request, json!({"documents":project.documents}));
        end_request(t, evidence, &route, None, "refused", rule, context)?;
        let mut diagnostic =
            crate::explain::diagnostic(&t.state, &route, context, "refused", rule)?;
        if let Some(subject) = restricted.filter(|subject| subject.value_resource) {
            subject.locate(&mut diagnostic, &project.documents);
        }
        t.reply = json!({"accepted":false,"reason":"refused","diagnostics":[diagnostic]});
        return Ok(());
    }
    prepare(t, snapshots, builtin, project, evidence, request, context).map(|_| ())
}

/// ACC-054 / OPS-023: inspect the consumer independently of template or target validity.
/// Standalone consumers are standard arguments (OBJ-013), so no expansion is needed.
pub(crate) fn root_consumed(state: &crate::State, document: &Value, root: &Value) -> bool {
    document["kind"] == "Conduit"
        && crate::access::object(state, &document["consumer"])
            .is_some_and(|consumer| consumer["id"] == root["id"])
}

/// ACC-032 / ACC-054: authority is decided before catalog validity or allocation.
/// The scan uses the same held templates and input substitution as instantiation, but
/// publishes no object and consumes no boundary-supplied ID.
pub(crate) struct RegistrationRestriction {
    pub(crate) rule: &'static str,
    place: DocumentPlace,
}

impl RegistrationRestriction {
    pub(crate) fn location(&self) -> String {
        self.place.qualified("")
    }
}

impl Builder<'_> {
    fn reservation_candidates(
        &self,
        document: &Value,
        held: &Held,
        parent: Option<(&Value, &str, &str)>,
        place: DocumentPlace,
        candidates: &mut Vec<(Value, DocumentPlace)>,
        depth: usize,
    ) {
        if depth > 128 || document.get("from").is_none() {
            return;
        }
        let Some(kind) = document["kind"].as_str() else {
            return;
        };
        let Ok((template, h, _)) = self.template(&document["from"], kind, held, false) else {
            return;
        };
        let mut spec = template["spec"].clone();
        if !spec.is_object() {
            return;
        }
        implicit(&mut spec, kind);
        let definition = DocumentPlace {
            name: template["metadata"]["name"].as_str().unwrap_or("").into(),
            pointer: "/spec".into(),
        };
        let mut filled = spec.clone();
        // Invalid inputs cannot hide a restriction written directly in the template.
        // Input-filled operations are inspected only when substitution succeeds.
        if self
            .apply_inputs(document, &mut filled, &h, &place, &definition, parent)
            .is_ok()
        {
            spec = filled;
        }
        spec["kind"] = json!(kind);
        spec["name"] = document["metadata"]["name"].clone();
        spec["held"] = serde_json::to_value(&h).expect("held contexts serialize");
        for key in ["consumer", "target", "owner"] {
            if let Some(value) = document.get(key) {
                spec[key] = value.clone();
            }
        }
        if let Some((parent, slot, local)) = parent {
            let reference = if parent["id"].is_string() {
                json!({"kind":parent["kind"],"id":parent["id"]})
            } else {
                json!({"kind":parent["kind"],"name":parent["name"]})
            };
            spec[if slot == "entries" {
                "owner"
            } else {
                "consumer"
            }] = reference.clone();
            // Match instantiation's entry_pool: a declared Conduit of a Pool entry
            // resolves {entry: name} in that entry's enclosing Pool.
            if slot == "entries" {
                spec["reservation_entry_pool"] = reference.clone();
            } else if let Some(pool) = parent.get("reservation_entry_pool") {
                spec["reservation_entry_pool"] = pool.clone();
            } else if parent["nested"]["slot"] == "entries" {
                // A replacement can start from an already registered Pool entry.
                spec["reservation_entry_pool"] =
                    json!({"kind":"Pool","id":parent["nested"]["parent"]});
            }
            spec["reservation_parent"] = reference;
            spec["reservation_slot"] = json!(slot);
            spec["reservation_name"] = json!(local);
        }
        candidates.push((spec.clone(), place));
        for (slot, family) in [("conduits", "Conduit"), ("entries", "Resource")] {
            for (local, mut nested) in items(&spec, slot) {
                if !nested.is_object() {
                    continue;
                }
                nested["kind"] = json!(family);
                self.reservation_candidates(
                    &nested,
                    &h,
                    Some((&spec, slot, &local)),
                    DocumentPlace {
                        name: definition.name.clone(),
                        pointer: format!("/{slot}/{}", crate::schemas::token(&local)),
                    },
                    candidates,
                    depth + 1,
                );
            }
        }
    }
}

fn reservation_object<'a>(
    objects: &'a [Value],
    reference: &Value,
    entry_pool: Option<&Value>,
) -> Option<&'a Value> {
    if reference.is_null() {
        return None;
    }
    if let Some(entry) = reference.get("entry") {
        let pool_reference = reference
            .get("name")
            .map(|name| json!({"kind":"Pool","name":name}));
        let pool = reservation_object(objects, pool_reference.as_ref().or(entry_pool)?, None)?;
        return objects.iter().find(|object| {
            object["kind"] == "Resource"
                && object["state"] != "retired"
                && ((object["nested"]["slot"] == "entries"
                    && object["nested"]["name"] == *entry
                    && pool["id"].is_string()
                    && object["nested"]["parent"] == pool["id"])
                    || (object["reservation_slot"] == "entries"
                        && object["reservation_name"] == *entry
                        && reservation_object(objects, &object["reservation_parent"], None)
                            .is_some_and(|parent| std::ptr::eq(parent, pool))))
        });
    }
    objects.iter().find(|object| {
        object.get("spec").is_none()
            && reference
                .get("kind")
                .is_none_or(|kind| object["kind"] == *kind)
            && (reference.get("id").is_some_and(|id| object["id"] == *id)
                || reference
                    .as_str()
                    .is_some_and(|name| object["id"] == name || object["name"] == name)
                || reference
                    .get("name")
                    .is_some_and(|name| object["name"] == *name))
    })
}

fn reserved_conduit(object: &Value, objects: &[Value], context: &Context) -> bool {
    if object["kind"] != "Conduit" {
        return false;
    }
    if reservation_object(objects, &object["consumer"], None)
        .is_some_and(|consumer| consumer["from"] == context.qualified("root"))
    {
        return false;
    }
    let Some(target) = reservation_object(
        objects,
        &object["target"],
        object.get("reservation_entry_pool"),
    ) else {
        return false;
    };
    // Read the target's contract scope without publishing its other operations:
    // an invalid control or configuration must not hide this refusal (OPS-023).
    let target = if let Some(held) = target.get("held") {
        let Ok(held) = serde_json::from_value::<Held>(held.clone()) else {
            return false;
        };
        let Some(ty) = target["type"].as_str() else {
            return false;
        };
        let Ok((scope, _, _)) = held.select(ty, "Type", &context.core_package) else {
            return false;
        };
        json!({"schema_environment":held.environment,"contract_scope":scope.scope})
    } else if target.get("schema_environment").is_some() {
        target.clone()
    } else {
        json!({"schema_environment":context.core_environment,"contract_scope":"core"})
    };
    let held = object
        .get("held")
        .and_then(|h| serde_json::from_value::<Held>(h.clone()).ok());
    list(object, "operations").iter().any(|operation| {
        let qualified = if let Some(held) = &held {
            let Ok((_, _, qualified)) = held.select(operation, "Operation", &context.core_package)
            else {
                return false;
            };
            Some(qualified)
        } else {
            None
        };
        crate::schemas::operation(
            &target,
            qualified.as_deref().unwrap_or(operation),
            &context.core_package,
        )
        .is_ok_and(|(_, _, contract)| contract["spec"]["reservedToRoot"] == true)
    })
}

fn registration_restriction(
    state: &State,
    project: Project,
    snapshots: Vec<Snapshot>,
    context: &Context,
    root_own: bool,
    replacement: Option<(&Value, &str)>,
) -> Option<RegistrationRestriction> {
    let builtin = context.builtin.as_ref()?;
    let (environment, indexes) = Catalog::diagnostic_context(builtin, &snapshots, &project);
    let h = Held {
        environment,
        indexes,
        contexts: json!({}),
        scope: "project".into(),
        instance_inputs: BTreeMap::new(),
    };
    let evidence = Evidence {
        time: String::new(),
        ids: vec![],
        revisions: vec![],
        suffixes: vec![],
    };
    let b = Builder {
        context, evidence: &evidence, state: state.clone(), allocation: Allocation { evidence: &evidence, cursor: 0 },
        suffix: 0, objects: vec![], diagnostics: vec![], sources: BTreeMap::new(),
        templates: project.documents.iter().filter(|d| !packages::contract(d) && d.get("from").is_none())
            .map(|d| json!({"kind":d["kind"],"name":d["metadata"]["name"],"metadata":d["metadata"],"spec":d["spec"],"held":h})).collect(),
        reserved_names: BTreeSet::new(), snapshots, trusted: true, replacement_declaration: None,
        submitted_resources: BTreeMap::new(), defer_children: false,
    };
    let mut candidates = vec![];
    if let Some((parent, name)) = replacement {
        let slot = if parent["kind"] == "Pool" {
            "entries"
        } else {
            "conduits"
        };
        let mut document = parent[slot][name].clone();
        if !document.is_object() {
            return None;
        }
        document["kind"] = json!(if slot == "entries" {
            "Resource"
        } else {
            "Conduit"
        });
        let held = serde_json::from_value(parent["held"].clone()).ok()?;
        b.reservation_candidates(
            &document,
            &held,
            Some((parent, slot, name)),
            DocumentPlace {
                name: parent["name"].as_str()?.into(),
                pointer: format!("/{slot}/{}", crate::schemas::token(name)),
            },
            &mut candidates,
            0,
        );
    } else {
        for document in &project.documents {
            let Some(name) = document["metadata"]["name"].as_str() else {
                continue;
            };
            b.reservation_candidates(
                document,
                &h,
                None,
                DocumentPlace {
                    name: name.into(),
                    pointer: String::new(),
                },
                &mut candidates,
                0,
            );
        }
    }
    let mut objects = state.objects.clone();
    objects.extend(candidates.iter().map(|(object, _)| object.clone()));
    // ACC-054 precedes ACC-032 even when another declaration would be refused too.
    if !root_own
        && let Some((_, place)) = candidates.iter().find(|(object, _)| {
            object["kind"] == "Conduit"
                && reservation_object(&objects, &object["consumer"], None)
                    .is_some_and(|consumer| consumer["from"] == context.qualified("root"))
        })
    {
        return Some(RegistrationRestriction {
            rule: "ACC-054",
            place: place.clone(),
        });
    }
    candidates
        .into_iter()
        .find(|(object, _)| reserved_conduit(object, &objects, context))
        .map(|(_, place)| RegistrationRestriction {
            rule: "ACC-032",
            place,
        })
}

pub(crate) fn reserved_submission(
    state: &State,
    documents: &[Value],
    manifest: Option<&Value>,
    snapshots: Vec<Snapshot>,
    context: &Context,
    root_own: bool,
) -> Option<RegistrationRestriction> {
    registration_restriction(
        state,
        Project {
            manifest: manifest.cloned(),
            documents: documents.to_vec(),
            directory: None,
        },
        snapshots,
        context,
        root_own,
        None,
    )
}

pub(crate) fn reserved_replacement(
    state: &State,
    arguments: &Value,
    context: &Context,
    root_own: bool,
) -> Option<RegistrationRestriction> {
    let parent = crate::access::object(state, &arguments["instance"])?;
    let name = arguments["nested"].as_str()?;
    registration_restriction(
        state,
        Project {
            manifest: None,
            documents: vec![],
            directory: None,
        },
        packages::catalog_entries(state, state.preinstalled_snapshots.clone()),
        context,
        root_own,
        Some((parent, name)),
    )
}

fn refuse_reserved_registration(
    t: &mut Transition,
    evidence: &Evidence,
    request: &Request,
    documents: &[Value],
    context: &Context,
    restriction: RegistrationRestriction,
) -> Result<Option<crate::explain::Rejection>> {
    let rule = restriction.rule;
    t.decisions.push(decision(
        "refuse-reserved-conduit-registration",
        &[rule, "OPS-023"],
        &[],
        json!({}),
        vec![],
    ));
    let route = crate::explain::ids(request, json!({"documents":documents}));
    end_request(t, evidence, &route, None, "refused", rule, context)?;
    let mut diagnostic = crate::explain::diagnostic(&t.state, &route, context, "refused", rule)?;
    packages::object_location(
        &mut diagnostic,
        &restriction.place.name,
        &restriction.place.pointer,
    );
    t.reply = json!({"accepted":false,"reason":"refused","diagnostics":[diagnostic]});
    Ok(Some(crate::explain::Rejection::new("refused", rule)))
}

fn refuse_root_registration(
    t: &mut Transition,
    evidence: &Evidence,
    request: &Request,
    documents: &[Value],
    context: &Context,
) -> Result<Option<crate::explain::Rejection>> {
    t.decisions.push(decision(
        "refuse-root-consumer-registration",
        &["ACC-054", "OPS-023"],
        &[],
        json!({}),
        vec![],
    ));
    let route = crate::explain::ids(request, json!({"documents":documents}));
    end_request(t, evidence, &route, None, "refused", "ACC-054", context)?;
    let mut diagnostic =
        crate::explain::diagnostic(&t.state, &route, context, "refused", "ACC-054")?;
    if let Some(root) = t
        .state
        .objects
        .iter()
        .find(|o| o["from"] == context.qualified("root"))
        && let Some(name) = documents
            .iter()
            .find(|document| root_consumed(&t.state, document, root))
            .and_then(|document| document["metadata"]["name"].as_str())
    {
        packages::object_location(&mut diagnostic, name, "");
    }
    t.reply = json!({"accepted":false,"reason":"refused","diagnostics":[diagnostic]});
    Ok(Some(crate::explain::Rejection::new("refused", "ACC-054")))
}

fn diagnostic_documents(
    project: &Project,
    builtin: &Project,
    snapshots: &[Snapshot],
    state: &State,
    core: &str,
) -> Vec<Value> {
    let (environment, indexes) = Catalog::diagnostic_context(builtin, snapshots, project);
    let withhold_arguments = |instance: &mut Value| {
        if let Some(arguments) = instance.get_mut("arguments") {
            *arguments = json!(conduit_loader::secret::WITHHELD_VALUE);
        }
    };
    let redact_arguments = |instance: &mut Value, kind: &str| {
        let Some(from) = instance
            .get("from")
            .filter(|from| from.get("kind").is_none_or(|explicit| explicit == kind))
            .and_then(|from| from.as_str().or_else(|| from["name"].as_str()))
        else {
            withhold_arguments(instance);
            return;
        };
        let local = from.strip_prefix('#').unwrap_or(from);
        let redact = |instance: &mut Value,
                      template: &Value,
                      environment: &crate::schemas::Environment,
                      scope: &str| {
            for (name, input) in items(&template["spec"], "inputs") {
                if let Some(value) = instance
                    .get_mut("arguments")
                    .and_then(|args| args.get_mut(&name))
                {
                    environment.withhold_values(
                        scope,
                        &input["schema"],
                        &input["schema"],
                        value,
                        core,
                    );
                }
            }
        };
        if from.contains('#') && !from.starts_with('#') {
            if let Ok((scope, name)) = packages::resolve_name(
                &environment,
                &indexes,
                "project",
                from,
                &format!("Template:{kind}"),
                core,
            ) {
                redact(instance, &indexes[&scope][&name], &environment, &scope);
                return;
            }
        } else if let Some(template) = project.documents.iter().find(|template| {
            template["kind"] == kind
                && template["metadata"]["name"] == local
                && template.get("spec").is_some()
        }) {
            redact(instance, template, &environment, "project");
            return;
        } else if let Some(template) = state.objects.iter().find(|template| {
            template["kind"] == kind && template["name"] == local && template.get("spec").is_some()
        }) && let Ok(held) = serde_json::from_value::<Held>(template["held"].clone())
        {
            redact(instance, template, &held.environment, &held.scope);
            return;
        }
        withhold_arguments(instance);
    };
    let mut documents = project.documents.clone();
    for document in &mut documents {
        environment.withhold_defaults("project", document, core);
        let kind = document["kind"].as_str().unwrap_or_default().to_owned();
        if document.get("from").is_some() {
            redact_arguments(document, &kind);
            continue;
        }
        let spec = &mut document["spec"];
        let config_type = spec["type"].as_str().and_then(|reference| {
            packages::resolve_name(&environment, &indexes, "project", reference, "Type", core).ok()
        });
        if let Some(config) = spec.get_mut("config") {
            if let Some((scope, name)) = config_type {
                let schema = &indexes[&scope][&name]["spec"]["schema"];
                environment.withhold_values(&scope, schema, schema, config, core);
            } else {
                *config = json!(conduit_loader::secret::WITHHELD_VALUE);
            }
        }
        for (slot, family) in [("entries", "Resource"), ("conduits", "Conduit")] {
            if let Some(members) = spec.get_mut(slot).and_then(Value::as_object_mut) {
                for instance in members.values_mut() {
                    redact_arguments(instance, family);
                }
            }
        }
        if let Some(requirements) = spec.get_mut("requirements").and_then(Value::as_object_mut) {
            for applied in requirements.values_mut() {
                if let Some(reference) = applied["requirement"].as_str()
                    && let Ok((scope, name)) = packages::resolve_name(
                        &environment,
                        &indexes,
                        "project",
                        reference,
                        "Requirement",
                        core,
                    )
                {
                    for (input, definition) in items(&indexes[&scope][&name]["spec"], "inputs") {
                        if let Some(value) = applied
                            .get_mut("arguments")
                            .and_then(|args| args.get_mut(&input))
                        {
                            let schema = &definition["schema"];
                            environment.withhold_values(&scope, schema, schema, value, core);
                        }
                    }
                }
            }
        }
    }
    documents
}

/// Resolve and stage a submission, also used for one-template replacement.
#[allow(clippy::too_many_arguments)]
pub(crate) fn prepare(
    t: &mut Transition,
    snapshots: Vec<Snapshot>,
    builtin: Project,
    mut project: Project,
    evidence: &Evidence,
    request: &Request,
    context: &Context,
) -> Result<Option<crate::explain::Rejection>> {
    let root = t
        .state
        .objects
        .iter()
        .find(|o| o["from"] == context.qualified("root"))
        .ok_or("root absent")?;
    let root_own = root["id"] == request.sender && root["id"] == request.caller;
    // ACC-057 / SEC-008: authority precedes validation, including replacement.
    if !root_own
        && let Some(subject) = restricted_submission(
            &t.state,
            &project.documents,
            project.manifest.as_ref(),
            &snapshots,
            context,
        )
    {
        t.decisions.push(decision(
            "refuse-agent-registration",
            &["ACC-057", "OPS-023"],
            &[],
            json!({}),
            vec![],
        ));
        let route = crate::explain::ids(request, json!({"documents":project.documents}));
        end_request(t, evidence, &route, None, "refused", "ACC-057", context)?;
        let mut diagnostic =
            crate::explain::diagnostic(&t.state, &route, context, "refused", "ACC-057")?;
        subject.locate(&mut diagnostic, &project.documents);
        t.reply = json!({"accepted":false,"reason":"refused","diagnostics":[diagnostic]});
        return Ok(Some(crate::explain::Rejection::new("refused", "ACC-057")));
    }
    // ACC-054 precedes every validation or conflict, even if catalog resolution fails.
    if !root_own
        && project
            .documents
            .iter()
            .any(|doc| root_consumed(&t.state, doc, root))
    {
        return refuse_root_registration(t, evidence, request, &project.documents, context);
    }
    if let Some(restriction) = reserved_submission(
        &t.state,
        &project.documents,
        project.manifest.as_ref(),
        snapshots.clone(),
        context,
        root_own,
    ) {
        return refuse_reserved_registration(
            t,
            evidence,
            request,
            &project.documents,
            context,
            restriction,
        );
    }
    // Canonical traversal fixes allocations, diagnostics and nested names independently of file order.
    project.documents.sort_by_key(|d| {
        (
            d["kind"].to_string(),
            d["metadata"]["name"].to_string(),
            d.to_string(),
        )
    });
    let mut diagnostics = vec![];
    let mut named = BTreeMap::<(String, String), usize>::new();
    let mut clashing_containers = BTreeMap::new();
    let mut display = None;
    for (index, doc) in project.documents.iter().enumerate() {
        if packages::contract(doc) {
            continue;
        }
        if let (Some(kind), Some(name)) = (doc["kind"].as_str(), doc["metadata"]["name"].as_str()) {
            // FLEET-006: Pools and Fleets reserve these names and share one namespace.
            if ["Pool", "Fleet"].contains(&kind) && doc.get("from").is_some() {
                if ["management", "access", "system"].contains(&name) {
                    diagnostics.push(diagnostic(
                        "FLEET-006",
                        name,
                        "reserved Pool or Fleet instance name",
                        "error",
                    ));
                } else if let Some(existing) = t.state.objects.iter().find(|o| {
                    o.get("spec").is_none()
                        && o["name"] == name
                        && ["Pool", "Fleet"].iter().any(|k| o["kind"] == *k)
                }) {
                    clashing_containers.insert((kind, name), &existing["id"]);
                    // The same-kind clash is diagnosed below under OBJ-002.
                    if existing["kind"] != kind {
                        diagnostics.push(diagnostic(
                            "OBJ-025",
                            name,
                            &format!(
                                "{kind}/{name} clashes with {}/{}",
                                existing["kind"], existing["name"]
                            ),
                            "error",
                        ));
                    }
                }
            }

            let key = (kind.to_owned(), name.to_owned());
            if let Some(previous) = named.insert(key, index) {
                let display = display.get_or_insert_with(|| {
                    diagnostic_documents(
                        &project,
                        &builtin,
                        &snapshots,
                        &t.state,
                        &context.core_package,
                    )
                });
                diagnostics.push(diagnostic(
                    "SUB-004",
                    name,
                    &format!(
                        "duplicate documents {} and {}",
                        display[previous], display[index]
                    ),
                    "error",
                ));
            }
            if let Some(existing) = t
                .state
                .objects
                .iter()
                .find(|o| o["kind"] == kind && o["name"] == name)
            {
                let display = display.get_or_insert_with(|| {
                    diagnostic_documents(
                        &project,
                        &builtin,
                        &snapshots,
                        &t.state,
                        &context.core_package,
                    )
                });
                diagnostics.push(diagnostic(
                    "OBJ-002",
                    name,
                    &format!(
                        "submitted object {} clashes with existing {}",
                        display[index], existing["id"]
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
                .unwrap_or_default()
                .into_iter()
                .map(ReportedFault::catalog)
                .collect::<Result<Vec<_>>>()?,
        );
        return reject(t, diagnostics, evidence, request, context);
    }
    diagnostics.extend(
        resolution.reply["diagnostics"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(ReportedFault::catalog)
            .collect::<Result<Vec<_>>>()?,
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
        instance_inputs: BTreeMap::new(),
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
        sources: BTreeMap::new(),
        templates: vec![],
        reserved_names: named.keys().cloned().collect(),
        snapshots,
        trusted: false,
        replacement_declaration: None,
        submitted_resources: BTreeMap::new(),
        defer_children: false,
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
    b.expand_project_instances(&project, &h)?;
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
            Err(f) => b.diagnostics.push(b.instance_diagnostic(o, f)),
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
        collected: None,
    };
    for template in &b.templates {
        b.diagnostics
            .extend(template_diagnostics(&b, &references, template, &h));
    }
    let mut resolved = vec![];
    for object in &b.objects {
        let source = object["id"].as_str().and_then(|id| b.sources.get(id));
        let mut admission_faults = vec![];
        match resolve_instance(object, &all, context, false, source, &mut admission_faults) {
            Ok(o) => resolved.push(o),
            Err(f) => b.diagnostics.push(b.instance_diagnostic(object, f)),
        }
        for failure in admission_faults {
            b.diagnostics.push(b.instance_diagnostic(object, failure));
        }
    }
    keep_nested_inputs(&mut b, &mut resolved, &references)?;
    // ACC-054 / OPS-023: expansion must not make authority depend on valid targets.
    if !root_own && b.objects.iter().any(|o| root_consumed(&b.state, o, root)) {
        return refuse_root_registration(t, evidence, request, &project.documents, context);
    }
    let mut all_resolved = b.state.objects.clone();
    all_resolved.extend(resolved.iter().cloned());
    for object in &resolved {
        if reserved_conduit(object, &all_resolved, context) {
            let source = object["id"].as_str().and_then(|id| b.sources.get(id));
            let place = source
                .and_then(|source| source.declaration.clone())
                .unwrap_or(DocumentPlace {
                    name: object["name"].as_str().unwrap_or("").into(),
                    pointer: String::new(),
                });
            return refuse_reserved_registration(
                t,
                evidence,
                request,
                &project.documents,
                context,
                RegistrationRestriction {
                    rule: "ACC-032",
                    place,
                },
            );
        }
    }

    for object in &resolved {
        let Some(place) = object["login_resource_place"].as_str() else {
            continue;
        };
        let Some(resource) = crate::login_resource::id(object)
            .and_then(|id| all_resolved.iter().find(|resource| &resource["id"] == id))
        else {
            b.diagnostics.push(
                b.instance_diagnostic(
                    object,
                    fault(
                        "CTR-013",
                        format!("login place {place} does not resolve to a login Resource"),
                    )
                    .at_configuration(place),
                ),
            );
            continue;
        };
        if let Some(pool) = b.state.objects.iter().find(|pool| {
            &pool["id"] == crate::access::id(&resource["owner"])
                && pool["kind"] == "Pool"
                && pool["state"] != "retired"
        }) && !b
            .state
            .objects
            .iter()
            .any(|path| permits(path, &request.sender, &pool["id"], &request.operation))
        {
            let mut route = crate::explain::ids(request, json!({"documents":project.documents}));
            end_request(t, evidence, &route, None, "refused", "ACC-033", context)?;
            route["loginPool"] = pool["id"].clone();
            let diagnostic =
                crate::explain::diagnostic(&t.state, &route, context, "refused", "ACC-033")?;
            t.decisions.push(decision(
                "refuse-login-pool-authorization",
                &["ACC-033", "OPS-023"],
                &[],
                json!({}),
                vec![],
            ));
            t.reply = json!({"accepted":false,"reason":"refused","diagnostics":[diagnostic]});
            return Ok(Some(crate::explain::Rejection::new("refused", "ACC-033")));
        }
        if let Some(failure) =
            admission_login_fault(object, resource, context).map_err(|failure| failure.text)?
        {
            let authored = b
                .objects
                .iter()
                .find(|o| o["id"] == object["id"])
                .unwrap_or(object);
            b.diagnostics.push(b.instance_diagnostic(authored, failure));
        }
        let expected = crate::sign_in::expected_account(resource, context)?;
        if resource["state"] == "retired"
            || !matches!(expected, crate::sign_in::ExpectedAccount::Set { .. })
        {
            let why = if resource["state"] == "retired" {
                "is retired"
            } else {
                "does not state signIn with a string expected account"
            };
            let label = crate::login_resource::label(resource);
            b.diagnostics.push(
                b.instance_diagnostic(
                    object,
                    fault("CTR-013", format!("login place {place}: {label} {why}"))
                        .at_configuration(place),
                ),
            );
        }
    }
    if b.diagnostics.iter().any(invalid_registration_fault) {
        return reject(t, b.diagnostics, evidence, request, context);
    }
    // FLEET-005 / IFC-014: defaults are open within registration, so a local-name
    // collision rejects the entire submission before publishing anything.
    for target in resolved
        .iter()
        .filter(|o| ["Pool", "Fleet"].iter().any(|k| o["kind"] == *k))
    {
        let clashing_container = clashing_containers
            .get(&(
                target["kind"].as_str().unwrap_or_default(),
                target["name"].as_str().unwrap_or_default(),
            ))
            .copied();
        if let Some(other) = all_resolved.iter().find(|o| {
            o["kind"] == "Conduit"
                && crate::access::id(&o["consumer"]) == &root["id"]
                && o["state"] != "closed"
                && o["state"] != "retired"
                && o["local_name"] == target["name"]
                // A conflicting container cannot open a second default.
                && !(o["provenance"]["cause"] == "default"
                    && clashing_container == Some(crate::access::id(&o["target"])))
        }) {
            let visible = b.state.objects.iter().any(|p| {
                permits(
                    p,
                    &request.sender,
                    &json!(request.target),
                    &context.qualified("inspect.v1"),
                )
            });
            let text = if visible || root_own {
                format!(
                    "default local name {} collides with Conduit {}",
                    target["name"], other["name"]
                )
            } else {
                format!("default local name {} is already in use", target["name"])
            };
            b.diagnostics.push(diagnostic(
                "IFC-014",
                target["name"].as_str().unwrap(),
                &text,
                "error",
            ));
        }
    }
    for path in resolved.iter().filter(|o| o["kind"] == "Conduit") {
        if let Some((rule, text)) = check_conduit(path, &all_resolved, context)? {
            b.diagnostics
                .push(b.instance_diagnostic(path, fault(rule, text)));
        }
    }
    if b.diagnostics.iter().any(invalid_registration_fault) {
        return reject(t, b.diagnostics, evidence, request, context);
    }
    let mut matching_state = b.state.clone();
    matching_state.objects.extend(resolved.iter().cloned());
    for object in &resolved {
        if let Some(stone) = crate::inventory::matching(&matching_state, object) {
            let mut diagnostic =
                crate::inventory::diagnostic(&b.state, &json!(request.caller), stone, context);
            if let Some(name) = object["name"].as_str() {
                packages::object_location(&mut diagnostic, name, "");
            }
            b.diagnostics.push(ReportedFault {
                rule: "LIF-041".into(),
                diagnostic,
                prior_sort_key: None,
            });
        }
    }
    // Replace checks the original state after deciding whether the trial changes anything.
    let client_conflicts = if request.operation == context.qualified("replace.v1") {
        Vec::new()
    } else {
        crate::client_ids::submission_diagnostics(
            &b.state,
            &b.snapshots,
            project.manifest.as_ref(),
            &project.documents,
        )
    };
    if !client_conflicts.is_empty() {
        return reject_with_client_conflicts(
            t,
            b.diagnostics,
            client_conflicts,
            evidence,
            request,
            context,
        );
    }
    if b.diagnostics.iter().any(|d| d["level"] == "error") {
        return reject(t, b.diagnostics, evidence, request, context);
    }
    // Nothing has been published before this point. The caller receives only this commit.
    let mut registered = b
        .templates
        .iter()
        .chain(&resolved)
        .map(|o| {
            // object.v1 names the enclosing instance, not the core's storage slot
            // (INS-016). An ordinary object has no nested member.
            let mut result =
                json!({"kind":o["kind"],"name":o["name"],"id":o["id"],"revision":o["revision"]});
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
    b.state.objects.append(&mut b.templates);
    b.state.objects.extend(resolved);
    crate::access::keep_bound_targets(&mut b.state, context);
    for object in &registered {
        let record=b.allocation.record(&mut b.state,context,"object-change",json!({"target":object["id"],"caller":request.caller,"consumer":request.sender,"conduit":request.conduit,"operation":request.operation,"action":"registered","state":"registered"}))?;
        t.records.push(record);
    }
    // FLEET-005: allocate every default object and its records AFTER all authored
    // objects and records. Generated names use OBJ-003 and DL15, as other instances do.
    let targets = b
        .state
        .objects
        .iter()
        .filter(|o| {
            registered.iter().any(|r| r["id"] == o["id"])
                && ["Pool", "Fleet"].iter().any(|k| o["kind"] == *k)
                && o.get("spec").is_none()
        })
        .cloned()
        .collect::<Vec<_>>();
    let authored_count = registered.len();
    for (i, target) in targets.iter().enumerate() {
        let template_name = if target["kind"] == "Pool" {
            "root-pool"
        } else {
            "root-fleet"
        };
        let template = context.template(template_name)?;
        let name = b.name("Conduit", template_name)?;
        let id = b.allocation.next(&mut b.state)?;
        let revision = evidence
            .revisions
            .get(authored_count + i)
            .ok_or("exhausted default revisions")?;
        let object = json!({"kind":"Conduit","name":name,"id":id,"revision":revision,
            "from":context.qualified(template_name),"state":"ready","config":{},
            "consumer":root["id"],"target":target["id"],"local_name":target["name"],
            "capability":template["spec"]["capability"],"operations":template["spec"]["operations"],
            "open":true,"provenance":{"cause":"default"}});
        b.state.objects.push(object);
        registered.push(json!({"kind":"Conduit","name":name,"id":id,"revision":revision}));
        for (action, previous, state) in [
            ("registered", Value::Null, "registered"),
            ("lifecycle-changed", json!("registered"), "ready"),
        ] {
            let record = b.allocation.record(&mut b.state, context, "object-change", json!({"target":id,
                "caller":request.caller,"consumer":request.sender,"conduit":request.conduit,"operation":request.operation,
                "cause":"default","action":action,"previous":previous,"state":state}))?;
            t.records.push(record);
        }
        t.decisions.push(decision(
            "open-root-default",
            &[
                "FLEET-005",
                "OBJ-003",
                "ACC-044",
                "CON-002",
                "REC-001",
                "REC-002",
                "REC-003",
            ],
            &["DL15"],
            json!({"target":target["id"]}),
            vec![id],
        ));
    }
    t.state = b.state;
    t.decisions.push(decision(
        "register-project-atomically",
        &[
            "SUB-001",
            "SUB-002",
            "SUB-003",
            "SUB-006",
            "SUB-008",
            "OBJ-002",
            "OBJ-003",
            "OBJ-009",
            "OBJ-010",
            "OBJ-011",
            "OBJ-018",
            "OBJ-021",
            "INS-002",
            "INS-003",
            "INS-004",
            "INS-005",
            "INS-006",
            "INS-007",
            "INS-008",
            "INS-009",
            "INS-010",
            "INS-011",
            "INS-012",
            "INS-013",
            "INS-014",
            "INS-016",
            "INS-017",
            "INS-018",
            "INS-019",
            "INS-020",
            "INS-023",
            "INS-024",
            "INS-025",
            "TPL-002",
            "TPL-003",
            "TPL-004",
            "TPL-005",
            "TPL-019",
            "NAM-009",
            "NAM-010",
            "PLC-001",
            "PLC-002",
            "PLC-007",
            "POOL-001",
            "POOL-002",
            "CON-002",
            "ACC-033",
            "REC-001",
            "REC-002",
            "REC-003",
            "POOL-004",
            "CON-017",
            "CON-021",
            "OBJ-008",
            "CTR-004",
            "OBJ-013",
            "NAM-019",
            "SCH-009",
            "PLC-005",
            "PKG-007",
            "POOL-009",
            "POOL-014",
            "POOL-016",
            "PRV-006",
            "IFC-012",
            "IFC-014",
            "ACC-054",
            "FLEET-001",
            "FLEET-006",
            "FLEET-002",
            "FLEET-003",
            "FLEET-004",
            "FLEET-005",
            "ACC-057",
        ],
        &["DL15"],
        json!({"objects":registered.len(),"generated_suffixes_consumed":b.suffix}),
        registered
            .iter()
            .filter_map(|o| o["id"].as_str().map(str::to_owned))
            .collect(),
    ));
    t.reply = json!({"accepted":true,"objects":registered,"diagnostics":b.diagnostics});
    Ok(None)
}
fn invalid_registration_fault(diagnostic: &ReportedFault) -> bool {
    diagnostic["level"] == "error"
        && !matches!(diagnostic.rule.as_str(), "OBJ-002" | "OBJ-025" | "LIF-041")
}

fn reject(
    t: &mut Transition,
    diagnostics: Vec<ReportedFault>,
    evidence: &Evidence,
    request: &Request,
    context: &Context,
) -> Result<Option<crate::explain::Rejection>> {
    reject_with_client_conflicts(t, diagnostics, Vec::new(), evidence, request, context)
}

fn reject_with_client_conflicts(
    t: &mut Transition,
    mut diagnostics: Vec<ReportedFault>,
    client_conflicts: Vec<Value>,
    evidence: &Evidence,
    request: &Request,
    context: &Context,
) -> Result<Option<crate::explain::Rejection>> {
    diagnostics.sort_by_key(|d| {
        d.prior_sort_key
            .clone()
            .unwrap_or_else(|| d.diagnostic.to_string())
    });
    diagnostics.dedup();
    let invalid = diagnostics.iter().any(invalid_registration_fault);
    let reason = if invalid { "invalid" } else { "conflict" };
    let mut promises = diagnostics
        .iter()
        .map(|d| d.rule.as_str())
        .collect::<BTreeSet<_>>();
    if !client_conflicts.is_empty() {
        promises.insert("CTR-013");
    }
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
    let rule = diagnostics
        .iter()
        .find(|d| d["level"] == "error" && (!invalid || invalid_registration_fault(d)))
        .map(|d| d.rule.as_str())
        .or_else(|| (!client_conflicts.is_empty()).then_some("CTR-013"))
        .ok_or("registration rejection lacks a deciding rule")?;
    end_request(
        t,
        evidence,
        &crate::explain::ids(request, json!({})),
        None,
        reason,
        rule,
        context,
    )?;
    let rejection = crate::explain::Rejection::new(reason, rule);
    let mut diagnostics = serde_json::to_value(diagnostics).map_err(|e| e.to_string())?;
    diagnostics.as_array_mut().unwrap().extend(client_conflicts);
    t.reply = json!({"accepted":false,"reason":reason,"diagnostics":diagnostics});
    Ok(Some(rejection))
}

/// OBJ-009 / LIF-021: collect references through the same governing schemas as resolution.
/// In particular, an author's ordinary kind/id configuration map is not a reference.
pub(crate) fn instance_references(
    object: &Value,
    all: &[Value],
    context: &Context,
) -> Result<BTreeSet<String>> {
    let collected = RefCell::new(BTreeSet::new());
    let refs = References {
        objects: all,
        core: &context.core_package,
        unresolved: None,
        template_entries: None,
        collected: Some(&collected),
    };
    let published = publication_object(object, context)?;
    let (environment, scope, ty) = crate::schemas::operation(
        &published,
        object["type"].as_str().ok_or("missing Type")?,
        &context.core_package,
    )?;
    let type_held = Held {
        environment,
        scope,
        indexes: BTreeMap::new(),
        contexts: json!({}),
        instance_inputs: BTreeMap::new(),
    };
    let inspect = |h: &Held, schema: &Value, value: &Value| -> Result<()> {
        refs.schema(h, schema, schema, value, None, false, 0)
            .map(|_| ())
            .map_err(|f| f.text)
    };
    inspect(&type_held, &ty["spec"]["schema"], &object["config"])?;
    if let Some(held) = object.get("held") {
        let h: Held = serde_json::from_value(held.clone()).map_err(|e| e.to_string())?;
        for input in h.instance_inputs.values() {
            for pointer in list(input, "to") {
                if let Some(value) = object.pointer(&pointer) {
                    inspect(&h, &input["schema"], value)?;
                }
            }
        }
        for (_, applied) in items(object, "requirements") {
            let (rh, requirement, _) = h
                .select(
                    applied["requirement"]
                        .as_str()
                        .ok_or("missing Requirement")?,
                    "Requirement",
                    &context.core_package,
                )
                .map_err(|f| f.text)?;
            for (name, input) in items(&requirement["spec"], "inputs") {
                if let Some(value) = applied["arguments"].get(&name) {
                    inspect(&rh, &input["schema"], value)?;
                }
            }
        }
    }
    // These standard fields and nested targets are reference positions by definition,
    // independently of any author-defined configuration schema (OBJ-013, CON-001).
    for value in [object.get("placement"), object.get("runtime")]
        .into_iter()
        .flatten()
        .chain(items(object, "conduits").iter().map(|(_, c)| &c["target"]))
    {
        if let Some(id) = crate::access::id(value).as_str() {
            collected.borrow_mut().insert(id.to_owned());
        }
    }
    Ok(collected.into_inner())
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

pub(crate) fn requires_operation(
    object: &Value,
    operation: &str,
    context: &Context,
) -> Result<bool> {
    if object["resolved_implementations"].get(operation).is_none() {
        return Ok(false);
    }
    let held: Held = serde_json::from_value(object["held"].clone()).map_err(|e| e.to_string())?;
    let core = &context.core_package;
    let (type_scope, ty, _) = held
        .select(object["type"].as_str().ok_or("missing Type")?, "Type", core)
        .map_err(|fault| fault.text)?;
    Ok(list(&ty["spec"], "requires").iter().any(|reference| {
        type_scope
            .select(reference, "Operation", core)
            .is_ok_and(|(_, _, qualified)| qualified == operation)
    }))
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
            && o["state"] != "closed"
            && o["id"] != path["id"]
            && crate::access::id(&o["consumer"]) == crate::access::id(&path["consumer"])
    }) {
        // IFC-014: local names collide even when neither Conduit presents a tool.
        if local(path) == local(other) {
            return Ok(Some((
                "IFC-014",
                format!(
                    "local name {} collides with Conduit {} ({})",
                    local(path),
                    other["name"].as_str().unwrap_or(""),
                    other["id"].as_str().unwrap_or("")
                ),
            )));
        }
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

// Shared by initial registration and replacement: newly made holders also keep
// origins for their own nested instances, outside their declared definition.
fn keep_nested_inputs(
    b: &mut Builder<'_>,
    resolved: &mut [Value],
    references: &References<'_>,
) -> Result<()> {
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
        parent[slot][local]
            .as_object_mut()
            .unwrap()
            .remove("values");
        let input_values = child["id"]
            .as_str()
            .and_then(|id| b.sources.get(id))
            .map(|source| &source.input_values);
        if resolved_child.is_none() && input_values.is_some_and(|origins| !origins.is_empty()) {
            continue;
        }
        if let Some(input_values) = input_values {
            let prefix = format!("/{slot}/{}", crate::schemas::token(local));
            for (at, source) in input_values {
                parent["_value_sources"][format!("{prefix}{at}")] = source.reference();
                parent["_value_origins"][format!("{prefix}{at}")] =
                    serde_json::to_value(source).map_err(|e| e.to_string())?;
            }
        }
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
                Err(f) => b.diagnostics.push(b.instance_diagnostic(child, f)),
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
    Ok(())
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
        sources: BTreeMap::new(),
        templates: vec![],
        reserved_names: BTreeSet::new(),
        snapshots: packages::catalog_entries(&t.state, t.state.preinstalled_snapshots.clone()),
        trusted: true,
        replacement_declaration: Some(DocumentPlace {
            name: parent["name"]
                .as_str()
                .ok_or("replacement holder has no name")?
                .into(),
            pointer: format!("/{slot}/{}", crate::schemas::token(name)),
        }),
        submitted_resources: BTreeMap::new(),
        defer_children: false,
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
        return Ok(
            json!({"rule":d.rule,"message":d["message"]["text"],"diagnostics":[d.diagnostic]}),
        );
    }
    let all: Vec<_> = b.state.objects.iter().chain(&b.objects).cloned().collect();
    let mut resolved = vec![];
    for object in &b.objects {
        let source = object["id"].as_str().and_then(|id| b.sources.get(id));
        let mut admission_faults = vec![];
        let result = resolve_instance(object, &all, context, true, source, &mut admission_faults);
        for failure in admission_faults {
            b.diagnostics.push(b.instance_diagnostic(object, failure));
        }
        match result {
            Ok(o) => resolved.push(o),
            Err(f) => {
                if !matches!(f.pointer, Some(FaultPointer::Authored(..))) {
                    return Ok(json!({"rule":f.rule,"message":f.text}));
                }
                let d = b.instance_diagnostic(object, f);
                return Ok(
                    json!({"rule":d.rule,"message":d["message"]["text"],"diagnostics":[d.diagnostic]}),
                );
            }
        }
    }
    let references = References {
        objects: &all,
        core: &context.core_package,
        unresolved: None,
        template_entries: None,
        collected: None,
    };
    keep_nested_inputs(&mut b, &mut resolved, &references)?;
    if let Some(d) = b.diagnostics.first() {
        return Ok(
            json!({"rule":d.rule,"message":d["message"]["text"],"diagnostics":[d.diagnostic]}),
        );
    }
    let all: Vec<_> = b.state.objects.iter().chain(&resolved).cloned().collect();
    for o in resolved.iter().filter(|o| o["kind"] == "Conduit") {
        if let Some((rule, message)) = check_conduit(o, &all, context)? {
            return Ok(json!({"rule":rule,"message":message}));
        }
    }
    // CTR-015 / LIF-009: current Agents retain the replaced Resource's ID.
    if slot == "entries"
        && let Some(resource) = resolved.first()
    {
        let replaced = crate::access::id(&parent["slots"][slot][name]);
        for agent in b.state.objects.iter().filter(|o| {
            o["kind"] == "Agent"
                && o["state"] != "retired"
                && crate::login_resource::id(o).is_some_and(|id| id == replaced)
        }) {
            if let Some(failure) =
                admission_login_fault(agent, resource, context).map_err(|failure| failure.text)?
            {
                let d = b.instance_diagnostic(agent, failure);
                return Ok(
                    json!({"rule":d.rule,"message":d["message"]["text"],"diagnostics":[d.diagnostic]}),
                );
            }
        }
    }
    let mut matching_state = b.state.clone();
    matching_state.objects.extend(resolved.iter().cloned());
    if let Some(stone) = resolved
        .iter()
        .find_map(|o| crate::inventory::matching(&matching_state, o))
    {
        return Ok(json!({"rule":"LIF-041","tombstone":stone}));
    }
    let object = resolved.first().ok_or("replacement produced no object")?;
    let result = json!({"object":{"kind":object["kind"],"name":object["name"],"id":object["id"],"revision":object["revision"]},"diagnostics":[]});
    let p = b
        .state
        .objects
        .iter_mut()
        .find(|o| o["id"] == parent["id"])
        .unwrap();
    p["slots"][slot][name] = json!({"kind":object["kind"],"id":object["id"]});
    b.state.objects.extend(resolved);
    // ACC-053 / ACC-055: replacement bounds must survive removal of their targets.
    crate::access::keep_bound_targets(&mut b.state, context);
    a.cursor = b.allocation.cursor;
    t.state = b.state;
    Ok(result)
}

#[cfg(test)]
mod explanation_record_tests {
    use super::*;
    #[test]
    fn registration_record_rule_is_independent_of_serialized_diagnostic() {
        let (state, c) = crate::explain::tests::fixture();
        let mut t = crate::explain::tests::transition(state);
        let mut fault = diagnostic("SCH-001", "submitted", "invalid schema", "error");
        fault.diagnostic.as_object_mut().unwrap().remove("ruleId");
        let request = Request {
            sender: "root".into(),
            caller: "root".into(),
            conduit: "root-management".into(),
            target: "management".into(),
            operation: c.qualified("register.v1"),
        };
        let rejection = reject(
            &mut t,
            vec![
                diagnostic("OBJ-002", "a-conflict", "existing name", "error"),
                fault,
            ],
            &crate::explain::tests::evidence(),
            &request,
            &c,
        )
        .unwrap();
        let rejection = rejection.unwrap();
        assert_eq!(rejection.reason, "invalid");
        assert_eq!(rejection.rule, "SCH-001");
        assert!(t.records.iter().all(|r| r["data"]["ruleId"] == "SCH-001"));
    }
}

#[cfg(test)]
#[path = "registration_value_replacement_tests.rs"]
mod value_replacement_tests;
