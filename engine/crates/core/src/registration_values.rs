use super::*;

pub(super) fn type_reference(core: &str) -> String {
    format!("{core}#value.v1")
}

/// ACC-057 is an authority check. Read template identity without running catalog
/// validation or expanding user data so validity cannot outrank the refusal.
struct TemplateLookup<'a> {
    state: &'a State,
    documents: &'a [Value],
    manifest: Option<&'a Value>,
    snapshots: &'a [Snapshot],
    context: &'a Context,
}

struct Definition<'a> {
    spec: &'a Value,
    package: Option<&'a Project>,
    stored: Option<&'a Value>,
}

impl<'a> TemplateLookup<'a> {
    fn in_project(project: &'a Project, kind: &str, name: &str) -> Option<Definition<'a>> {
        project
            .documents
            .iter()
            .find(|document| document["kind"] == kind && document["metadata"]["name"] == name)
            .and_then(|document| {
                Some(Definition {
                    spec: document.get("spec")?,
                    package: Some(project),
                    stored: None,
                })
            })
    }

    fn definition(
        &self,
        kind: &str,
        from: &Value,
        package: Option<&'a Project>,
        held_by: Option<&Value>,
    ) -> Option<Definition<'a>> {
        if from.get("kind").is_some_and(|family| family != kind) {
            return None;
        }
        if let Some(id) = from.get("id").filter(|_| held_by.is_some()) {
            let object = self
                .state
                .objects
                .iter()
                .find(|object| object["id"] == *id && object["kind"] == kind)?;
            return Some(Definition {
                spec: object.get("spec")?,
                package: None,
                stored: Some(object),
            });
        }
        let spelling = from.as_str().or_else(|| from["name"].as_str())?;
        if let Some((address, name)) = spelling.split_once('#') {
            if address == self.context.core_package {
                return self
                    .context
                    .templates
                    .get(name)
                    .filter(|object| object["kind"] == kind)
                    .and_then(|object| {
                        Some(Definition {
                            spec: object.get("spec")?,
                            package: None,
                            stored: None,
                        })
                    });
            }
            if let Some(package) = package
                && (address.is_empty()
                    || package
                        .manifest
                        .as_ref()
                        .is_some_and(|m| m["address"] == address))
            {
                return Self::in_project(package, kind, name);
            }
            if !address.is_empty()
                && self
                    .manifest
                    .is_none_or(|manifest| manifest["address"] != address)
            {
                let pin = package
                    .and_then(|package| package.manifest.as_ref())
                    .or(self.manifest)
                    .and_then(|manifest| manifest["packages"].get(address));
                let retained = held_by.and_then(|holder| holder["packages"].as_array());
                let pin = pin.or_else(|| {
                    retained?
                        .iter()
                        .find(|package| package["address"] == address)
                })?;
                for snapshot in self
                    .state
                    .installed_packages
                    .iter()
                    .map(|package| &package.package)
                    .chain(self.snapshots)
                {
                    if snapshot.address == address
                        && pin["digest"] == snapshot.digest
                        && pin["version"].as_str().is_some_and(|version| {
                            packages::version_equal(version, &snapshot.version)
                        })
                        && snapshot
                            .project
                            .manifest
                            .as_ref()
                            .and_then(|manifest| manifest["exports"].as_array())
                            .is_some_and(|exports| exports.iter().any(|export| export == name))
                    {
                        return Self::in_project(&snapshot.project, kind, name);
                    }
                }
                return None;
            }
        }
        let name = spelling.rsplit('#').next()?;
        let object = self
            .documents
            .iter()
            .find(|object| {
                object["kind"] == kind
                    && object["metadata"]["name"] == name
                    && object.get("spec").is_some()
            })
            .or_else(|| {
                self.state.objects.iter().find(|object| {
                    object["kind"] == kind && object["name"] == name && object.get("spec").is_some()
                })
            })?;
        Some(Definition {
            spec: object.get("spec")?,
            package: None,
            stored: object.get("held").map(|_| object),
        })
    }

    fn makes_value(
        &self,
        from: &Value,
        package: Option<&'a Project>,
        held_by: Option<&Value>,
    ) -> bool {
        self.definition("Resource", from, package, held_by)
            .is_some_and(|definition| {
                definition.spec["type"] == type_reference(&self.context.core_package)
            })
    }
}

pub(crate) struct RestrictedSubmission {
    pub(crate) index: usize,
    pub(crate) suffix: String,
    pub(crate) value_resource: bool,
}

impl RestrictedSubmission {
    pub(crate) fn wire_location(&self) -> String {
        format!("/documents/{}{}", self.index, self.suffix)
    }

    pub(super) fn locate(&self, diagnostic: &mut Value, documents: &[Value]) {
        if let Some(name) = documents[self.index]["metadata"]["name"].as_str() {
            packages::object_location(diagnostic, name, &self.suffix);
        } else if self.value_resource {
            diagnostic["locations"] =
                json!([{"logicalLocations":[{"fullyQualifiedName":self.wire_location()}]}]);
        }
    }
}

pub(crate) fn restricted_submission(
    state: &State,
    documents: &[Value],
    manifest: Option<&Value>,
    snapshots: &[Snapshot],
    context: &Context,
) -> Option<RestrictedSubmission> {
    let lookup = TemplateLookup {
        state,
        documents,
        manifest,
        snapshots,
        context,
    };
    documents.iter().enumerate().find_map(|(index, document)| {
        let from = document.get("from")?;
        let suffix = match document["kind"].as_str()? {
            "Agent" => {
                return Some(RestrictedSubmission {
                    index,
                    suffix: String::new(),
                    value_resource: false,
                });
            }
            "Resource" if lookup.makes_value(from, None, None) => String::new(),
            "Pool" => {
                let definition = lookup.definition("Pool", from, None, None)?;
                let (name, _) =
                    definition.spec["entries"]
                        .as_object()?
                        .iter()
                        .find(|(_, entry)| {
                            lookup.makes_value(
                                &entry["from"],
                                definition.package,
                                definition.stored,
                            )
                        })?;
                format!("/entries/{}", crate::schemas::token(name))
            }
            _ => return None,
        };
        Some(RestrictedSubmission {
            index,
            suffix,
            value_resource: true,
        })
    })
}

pub(crate) fn restricted_replacement(state: &State, arguments: &Value, context: &Context) -> bool {
    let Some(parent) = crate::access::object(state, &arguments["instance"])
        .filter(|parent| parent["kind"] == "Pool")
    else {
        return false;
    };
    let Some(name) = arguments["nested"].as_str() else {
        return false;
    };
    let lookup = TemplateLookup {
        state,
        documents: &[],
        manifest: None,
        snapshots: &state.preinstalled_snapshots,
        context,
    };
    lookup.makes_value(&parent["entries"][name]["from"], None, Some(parent))
}

pub(super) type ValueOrigins = BTreeMap<String, ValueOrigin>;

#[derive(Clone)]
pub(super) struct ValueOrigin {
    id: Value,
    node: Option<Value>,
    place: DocumentPlace,
    name: String,
}

impl ValueOrigin {
    pub(super) fn reference(&self) -> Value {
        json!({"kind":"Resource","id":self.id})
    }

    pub(super) fn failure(
        &self,
        rule: &'static str,
        check: &str,
        receiving: Option<&str>,
    ) -> Fault {
        let mut failure = fault(
            rule,
            format!(
                "{} names {{kind: Resource, name: {}}}: {check}",
                self.place.pointer, self.name
            ),
        );
        failure.pointer = Some(FaultPointer::Authored(
            self.place.clone(),
            receiving.into_iter().map(str::to_owned).collect(),
        ));
        failure
    }

    pub(super) fn check_placement(
        &self,
        placement: Option<&Value>,
        objects: &[Value],
    ) -> Checked<()> {
        if let Some(node) = &self.node
            && placement != Some(node)
        {
            fn name<'a>(node: &'a Value, objects: &'a [Value]) -> &'a str {
                objects
                    .iter()
                    .find(|object| object["id"] == node["id"])
                    .and_then(|object| object["name"].as_str())
                    .or_else(|| node["id"].as_str())
                    .unwrap_or("unknown")
            }
            let node = name(node, objects);
            let (detail, receiving) = if let Some(placement) = placement {
                (
                    format!(
                        "the instance is placed on node {}",
                        name(placement, objects)
                    ),
                    Some("/placement"),
                )
            } else {
                (
                    "an Interface, a Pool or a Conduit takes no value tied to a node".into(),
                    None,
                )
            };
            return Err(self.failure(
                "OBJ-009",
                &format!("the value Resource is tied to node {node}; {detail}"),
                receiving,
            ));
        }
        Ok(())
    }
}

pub(super) fn below<'a>(pointer: &'a str, parent: &str) -> Option<&'a str> {
    pointer
        .strip_prefix(parent)
        .filter(|suffix| suffix.is_empty() || suffix.starts_with('/'))
}

pub(super) fn rebase(origins: &ValueOrigins, from: &str, to: &str) -> ValueOrigins {
    origins
        .iter()
        .filter_map(|(at, origin)| Some((format!("{to}{}", below(at, from)?), origin.clone())))
        .collect()
}

pub(super) fn affected<'a>(origins: &'a ValueOrigins, at: &str) -> Option<&'a ValueOrigin> {
    origins
        .iter()
        .find(|(pointer, _)| below(pointer, at).is_some() || below(at, pointer).is_some())
        .map(|(_, origin)| origin)
}

/// A second walk through the same schema branches before any reference rewrite.
/// The tracked paths are scalar leaves, even when the argument has several members.
#[derive(Clone, Default)]
pub(super) struct ValueTrace<'a>(Option<(&'a ValueOrigins, String)>);

impl<'a> ValueTrace<'a> {
    pub(super) fn at(origins: &'a ValueOrigins, pointer: &str) -> Self {
        Self(Some((origins, pointer.into())))
    }

    pub(super) fn child(&self, key: &str) -> Self {
        Self(self.0.as_ref().map(|(origins, pointer)| {
            (
                *origins,
                format!("{pointer}/{}", crate::schemas::token(key)),
            )
        }))
    }

    pub(super) fn skip_branch(&self) -> bool {
        self.0.as_ref().is_some_and(|(origins, at)| {
            !origins.keys().any(|pointer| below(pointer, at).is_some())
        })
    }

    pub(super) fn check_reference(&self) -> Checked<bool> {
        let Some((origins, at)) = &self.0 else {
            return Ok(false);
        };
        if let Some(origin) = affected(origins, at) {
            return Err(origin.failure(
                "OBJ-009",
                "a reference schema governs this place",
                Some(at),
            ));
        }
        Ok(true)
    }
}

#[derive(Default)]
pub(super) struct AppliedInputs {
    pub(super) values: ValueOrigins,
    pub(super) destinations: ValueOrigins,
    pub(super) arguments: Value,
    pub(super) node_written: Option<WrittenNode>,
}

pub(super) struct WrittenNode {
    reference: Value,
    pub(super) place: DocumentPlace,
}

pub(super) enum SubmittedResource {
    Other,
    PendingValue,
    AttemptedValue,
}

fn entry_place(declaration: &DocumentPlace, section: &str, name: &str) -> DocumentPlace {
    DocumentPlace {
        name: declaration.name.clone(),
        pointer: format!(
            "{}/{section}/{}",
            declaration.pointer,
            crate::schemas::token(name)
        ),
    }
}

fn at_document(mut failure: Fault, place: DocumentPlace, related: Vec<String>) -> Fault {
    failure.pointer = Some(FaultPointer::Authored(place, related));
    failure
}

fn is_value_resource(object: &Value, core: &str) -> bool {
    object["kind"] == "Resource" && object["type"] == type_reference(core)
}

fn system_pool(objects: &[Value], context: &Context) -> Option<Value> {
    objects.iter().find_map(|object| {
        (object["kind"] == "Pool" && object["from"] == context.qualified("system"))
            .then(|| object["id"].clone())
    })
}

pub(super) fn validate_owner(
    object: &Value,
    written: &Value,
    all: &[Value],
    context: &Context,
) -> Checked<()> {
    if is_value_resource(object, &context.core_package)
        && system_pool(all, context).as_ref() != object["owner"].get("id")
    {
        let name = all.iter().find(|pool| pool["id"] == object["owner"]["id"]);
        let message = if let Some(entry) = object["nested"]["name"].as_str() {
            format!(
                "entry {entry} belongs to Pool {}; a value Resource must be owned by system",
                name.map(|pool| &pool["name"]).unwrap_or(written)
            )
        } else {
            format!("/owner {written}: a value Resource must be owned by system")
        };
        let failure = fault("OBJ-009", message);
        return Err(if object.get("nested").is_some() {
            failure
        } else {
            failure.at_argument("owner")
        });
    }
    Ok(())
}

fn value_node(
    reference: &Value,
    written: &Value,
    refs: &References<'_>,
    entry_pool: Option<&Value>,
    authored: bool,
) -> Checked<Value> {
    let invalid = || {
        let mut failure = fault(
            "OBJ-009",
            format!(
                "/config/node {written}: the reference must resolve to a Resource of the core Type node.v1"
            ),
        );
        failure.pointer = Some(FaultPointer::ValueNode);
        failure
    };
    let resolved = refs
        .resolve(reference, Some("Resource"), entry_pool, authored)
        .map_err(|_| invalid())?;
    if !refs.objects.iter().any(|object| {
        object["id"] == resolved["id"]
            && object["kind"] == "Resource"
            && object["type"] == format!("{}#node.v1", refs.core)
    }) {
        return Err(invalid());
    }
    Ok(resolved)
}

pub(super) fn validate_node(
    object: &Value,
    config: &Value,
    refs: &References<'_>,
    entry_pool: Option<&Value>,
    authored: bool,
    node_written: Option<&WrittenNode>,
) -> Checked<()> {
    if is_value_resource(object, refs.core)
        && let Some(node) = config.get("node")
    {
        value_node(
            node,
            node_written.map_or(node, |written| &written.reference),
            refs,
            entry_pool,
            authored,
        )?;
    }
    Ok(())
}

impl Builder<'_> {
    pub(super) fn expand_project_instances(
        &mut self,
        project: &Project,
        held: &Held,
    ) -> Result<()> {
        let documents = &project.documents;
        let lookup = TemplateLookup {
            state: &self.state,
            documents,
            manifest: project.manifest.as_ref(),
            snapshots: &self.snapshots,
            context: self.context,
        };
        let mut pending: BTreeSet<usize> = documents
            .iter()
            .enumerate()
            .filter_map(|(i, document)| {
                let from = document.get("from")?;
                if document["kind"] != "Resource" {
                    return None;
                }
                lookup.makes_value(from, None, None).then_some(i)
            })
            .collect();
        let sources = pending.clone();
        let named: BTreeMap<_, _> = pending
            .iter()
            .filter_map(|&i| Some((documents[i]["metadata"]["name"].as_str()?.to_owned(), i)))
            .collect();
        self.submitted_resources = documents
            .iter()
            .enumerate()
            .filter_map(|(i, document)| {
                if document["kind"] != "Resource" || document.get("from").is_none() {
                    return None;
                }
                Some((
                    document["metadata"]["name"].as_str()?.to_owned(),
                    if sources.contains(&i) {
                        SubmittedResource::PendingValue
                    } else {
                        SubmittedResource::Other
                    },
                ))
            })
            .collect();
        let before = self.objects.len();
        self.defer_children = true;
        while let Some(&first) = pending.first() {
            // A scalar may itself be supplied by another value Resource. Do not
            // resolve a sibling's reference from an unfinished configuration.
            let ready = pending.iter().copied().find(|&i| {
                items(&documents[i], "values").iter().all(|(_, reference)| {
                    reference["name"]
                        .as_str()
                        .and_then(|name| named.get(name))
                        .is_none_or(|dependency| !pending.contains(dependency))
                })
            });
            let i = ready.unwrap_or(first);
            pending.remove(&i);
            // If no document is ready, the first attempt sees the pending
            // dependency that blocked it, including a possible self-reference.
            self.expand(&documents[i], held, None, 0)?;
            if let Some(name) = documents[i]["metadata"]["name"].as_str() {
                self.submitted_resources
                    .insert(name.to_owned(), SubmittedResource::AttemptedValue);
            }
        }
        self.defer_children = false;
        for position in before..self.objects.len() {
            let object = self.objects[position].clone();
            let held = serde_json::from_value(object["held"].clone()).map_err(|e| e.to_string())?;
            self.expand_children(&object, &held, position, 0)?;
        }
        for (i, document) in documents.iter().enumerate() {
            if document.get("from").is_some() && !sources.contains(&i) {
                self.expand(document, held, None, 0)?;
            }
        }
        Ok(())
    }

    fn installation_value(
        &self,
        entry: &Value,
        place: DocumentPlace,
    ) -> Checked<(Value, ValueOrigin)> {
        let name = entry["name"].as_str().unwrap_or("");
        let mut origin = ValueOrigin {
            id: Value::Null,
            node: None,
            place,
            name: name.into(),
        };
        if entry["kind"] != "Resource" || name.is_empty() || entry.get("id").is_some() {
            return Err(origin.failure("OBJ-009", "an entry must name a Resource", None));
        }
        let Some(source) = self
            .state
            .objects
            .iter()
            .chain(&self.objects)
            .find(|object| {
                object["kind"] == "Resource"
                    && object["name"] == name
                    && object.get("spec").is_none()
            })
        else {
            let check = match self.submitted_resources.get(name) {
                Some(SubmittedResource::PendingValue) => {
                    "the value Resource could not be made because a dependency cycle blocks it in this registration"
                }
                Some(SubmittedResource::AttemptedValue) => {
                    "the value Resource was not made because its own document was rejected; see its diagnostic"
                }
                Some(SubmittedResource::Other) => "the Resource must have the core Type value.v1",
                None => "no Resource has this name; register the value Resource as root",
            };
            return Err(origin.failure("OBJ-009", check, None));
        };
        if !is_value_resource(source, &self.context.core_package) {
            return Err(origin.failure(
                "OBJ-009",
                "the Resource must have the core Type value.v1",
                None,
            ));
        }
        if source["state"] == "retired" {
            return Err(origin.failure("OBJ-009", "the value Resource is retired", None));
        }
        let stored = self
            .state
            .objects
            .iter()
            .any(|object| object["id"] == source["id"]);
        // Successful value sources are owned by the existing first-start Pool. Nodes are
        // already enrolled or made at first start, so no staged instance can supply either.
        let refs = References {
            objects: &self.state.objects,
            core: &self.context.core_package,
            unresolved: None,
            template_entries: None,
            collected: None,
        };
        let owner = refs
            .resolve(&source["owner"], Some("Pool"), None, !stored)
            .ok();
        if owner.as_ref().and_then(|owner| owner.get("id"))
            != system_pool(&self.state.objects, self.context).as_ref()
        {
            return Err(origin.failure(
                "OBJ-009",
                "the value Resource must be owned by system",
                None,
            ));
        }
        if let Some(node) = source["config"].get("node") {
            origin.node = Some(value_node(node, node, &refs, None, !stored).map_err(|_| {
                origin.failure(
                    "OBJ-009",
                    "the value Resource's node must resolve to a node",
                    None,
                )
            })?);
        }
        let scalar = &source["config"]["value"];
        if !(scalar.is_string() || scalar.is_number() || scalar.is_boolean()) {
            return Err(origin.failure(
                "OBJ-009",
                "the Resource must hold an installation scalar",
                None,
            ));
        }
        origin.id = source["id"].clone();
        Ok((scalar.clone(), origin))
    }

    pub(super) fn apply_inputs(
        &self,
        document: &Value,
        spec: &mut Value,
        held: &Held,
        value_declaration: &DocumentPlace,
        definition: &DocumentPlace,
        parent: Option<(&Value, &str, &str)>,
    ) -> Checked<AppliedInputs> {
        let inherited = parent
            .and_then(|(parent, slot, local)| {
                let source = self.sources.get(parent["id"].as_str()?)?;
                Some(rebase(
                    &source.destinations,
                    &format!("/{slot}/{}", crate::schemas::token(local)),
                    "",
                ))
            })
            .unwrap_or_default();
        for (name, _) in items(document, "values") {
            let place = entry_place(value_declaration, "values", &name);
            if spec["inputs"].get(&name).is_none() {
                return Err(at_document(
                    fault("INS-003", format!("undeclared input {name} under values")),
                    place,
                    vec![],
                ));
            }
            if document["arguments"].get(&name).is_some() {
                let argument = entry_place(value_declaration, "arguments", &name);
                let argument_at = format!("/arguments/{}", crate::schemas::token(&name));
                let argument =
                    affected(&inherited, &argument_at).map_or(&argument, |origin| &origin.place);
                return Err(at_document(
                    fault(
                        "INS-003",
                        format!("input {name} appears under both values and arguments"),
                    ),
                    place,
                    vec![argument.qualified("")],
                ));
            }
        }
        for (name, _) in items(document, "arguments") {
            if ["owner", "placement", "runtime", "consumer", "target", "as"]
                .contains(&name.as_str())
            {
                return Err(fault(
                    "OBJ-013",
                    format!("standard argument {name} under arguments"),
                ));
            }
            if spec["inputs"].get(&name).is_none() {
                let pointer = format!("/arguments/{}", crate::schemas::token(&name));
                if let Some(origin) = affected(&inherited, &pointer) {
                    return Err(origin.failure(
                        "INS-003",
                        "the receiving argument names no declared input",
                        Some(&pointer),
                    ));
                }
                return Err(fault("INS-003", format!("undeclared argument {name}")));
            }
        }
        let core = &self.context.core_package;
        let value_resource = document["kind"] == "Resource" && spec["type"] == type_reference(core);
        let refs = References {
            objects: &[],
            core,
            unresolved: None,
            template_entries: None,
            collected: None,
        };
        let mut result = AppliedInputs {
            arguments: json!({}),
            node_written: value_resource
                .then(|| {
                    spec.pointer("/config/node").map(|node| WrittenNode {
                        reference: node.clone(),
                        place: DocumentPlace {
                            name: definition.name.clone(),
                            pointer: format!("{}/config/node", definition.pointer),
                        },
                    })
                })
                .flatten(),
            ..AppliedInputs::default()
        };
        for (name, input) in items(spec, "inputs") {
            let key = crate::schemas::token(&name);
            let input_at = format!("/inputs/{key}");
            let argument_at = format!("/arguments/{key}");
            let mut origins = rebase(&inherited, &argument_at, &input_at);
            let supplied;
            let value = if let Some(entry) = document["values"].get(&name) {
                let (scalar, origin) = self
                    .installation_value(entry, entry_place(value_declaration, "values", &name))?;
                origins.insert(input_at.clone(), origin);
                supplied = scalar;
                Some(&supplied)
            } else {
                document["arguments"]
                    .get(&name)
                    .or_else(|| input.get("default"))
            };
            let Some(value) = value else {
                if input["optional"] == true {
                    continue;
                }
                return Err(fault("INS-004", format!("missing input {name}")));
            };
            let written = value;
            let schema = &input["schema"];
            let redacted = |rule, check, pointer: &str| {
                affected(&origins, pointer)
                    .or_else(|| origins.values().next())
                    .map(|origin| origin.failure(rule, check, Some(pointer)))
            };
            let value = held.fill(schema, value, core).map_err(|error| {
                redacted(
                    "INS-005",
                    "the value does not satisfy the input schema",
                    &input_at,
                )
                .unwrap_or_else(|| fault("INS-005", error))
            })?;
            refs.guard_schema(held, schema, &value, &origins, &input_at)?;
            if let Some(failure) = held
                .environment
                .failure_at(&held.scope, schema, "", &value, core)
                .map_err(|error| {
                    redacted(
                        "INS-005",
                        "the input schema could not be checked",
                        &input_at,
                    )
                    .unwrap_or_else(|| fault("INS-005", error))
                })?
            {
                let at = format!("{input_at}{}", failure.instance_pointer);
                return Err(redacted(
                    "INS-005",
                    &format!(
                        "the value does not satisfy the input schema ({})",
                        failure.keyword
                    ),
                    &at,
                )
                .unwrap_or_else(|| fault("INS-005", format!("input {name}: {failure}"))));
            }
            let value = if origins.is_empty() {
                expand_bare(held, schema, schema, value, core, 0)?
            } else {
                value
            };
            result.arguments[&name] = value.clone();
            result
                .values
                .extend(rebase(&origins, &input_at, &argument_at));
            for pointer in list(&input, "to") {
                let tokens = tokens(&pointer);
                if let [section, _, field] = tokens.as_slice()
                    && ((section == "entries"
                        && ["placement", "runtime"].contains(&field.as_str()))
                        || (section == "conduits" && field == "target"))
                    && let Some(origin) = origins.values().next()
                {
                    return Err(origin.failure(
                        "OBJ-009",
                        "a reference schema governs this place",
                        Some(&pointer),
                    ));
                }
                if let Some(origin) = origins.values().next() {
                    let mismatch = match tokens.as_slice() {
                        [field] if field == "operations" && !value.is_array() => {
                            Some("the receiving operations require a list")
                        }
                        [section, _, field]
                            if ["conduits", "entries", "requirements"]
                                .contains(&section.as_str())
                                && field == "arguments"
                                && !value.is_object() =>
                        {
                            Some("the receiving arguments require an object")
                        }
                        _ => None,
                    };
                    if let Some(check) = mismatch {
                        return Err(origin.failure("OBJ-009", check, Some(&pointer)));
                    }
                }
                assign(spec, &pointer, value.clone()).map_err(|failure| {
                    origins.values().next().map_or_else(
                        || failure,
                        |origin| {
                            origin.failure(
                                "INS-011",
                                "the receiving destination cannot be assigned",
                                Some(&pointer),
                            )
                        },
                    )
                })?;
                result
                    .destinations
                    .extend(rebase(&origins, &input_at, &pointer));
                if value_resource {
                    let node = "/config/node";
                    let (reference, suffix) = if let Some(suffix) = below(node, &pointer) {
                        (written.pointer(suffix), suffix)
                    } else if below(&pointer, node).is_some() {
                        (spec.pointer(node), "")
                    } else {
                        continue;
                    };
                    result.node_written = reference.map(|reference| {
                        let place = if document["arguments"].get(&name).is_some() {
                            entry_place(value_declaration, "arguments", &name)
                        } else {
                            DocumentPlace {
                                name: definition.name.clone(),
                                pointer: format!("{}/inputs/{key}/default", definition.pointer),
                            }
                        };
                        WrittenNode {
                            reference: reference.clone(),
                            place: DocumentPlace {
                                pointer: format!("{}{suffix}", place.pointer),
                                ..place
                            },
                        }
                    });
                }
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MISPLACED: &str =
        include_str!("../../../../conformance/language/registration/values-misplaced/case.yaml");

    fn case_context() -> (Value, State, Context) {
        let case: Value = serde_json::from_str(MISPLACED).unwrap();
        let c = crate::login_resource_tests::context();
        let run = |state: &State, i: usize| {
            crate::step(
                state,
                serde_json::from_value(case["steps"][i]["input"].clone()).unwrap(),
                &c,
            )
            .unwrap()
        };
        let started = run(&State::default(), 0);
        let seeded = run(&started.state, 1);
        assert_eq!(seeded.reply["isError"], false);
        (case, seeded.state, c)
    }

    fn submit(case: &Value, state: &State, c: &Context, documents: Value) -> Transition {
        let mut input = case["steps"][2]["input"].clone();
        input["request"]["arguments"]["documents"] = documents;
        let mut manifest = case["steps"][1]["input"]["request"]["arguments"]["manifest"].clone();
        manifest["address"] = json!("example.org/conformance/value-regression");
        input["request"]["arguments"]["manifest"] = manifest;
        crate::step(state, serde_json::from_value(input).unwrap(), c).unwrap()
    }

    fn doc(kind: &str, name: &str, body: Value) -> Value {
        let mut document = json!({
            "apiVersion":"github.com/shrav-k/conduit/v0","kind":kind,"metadata":{"name":name}
        });
        document
            .as_object_mut()
            .unwrap()
            .extend(body.as_object().unwrap().clone());
        document
    }

    fn source(name: &str) -> Value {
        json!({"kind":"Resource","name":name})
    }

    fn first_diagnostic(t: &Transition) -> &Value {
        &t.reply["structuredContent"]["diagnostics"][0]
    }

    fn location(diagnostic: &Value, index: usize) -> &Value {
        &diagnostic["locations"][index]["logicalLocations"][0]["fullyQualifiedName"]
    }

    #[test]
    fn values_regression_noncanonical_array_destination() {
        let (case, state, c) = case_context();
        let ty = doc(
            "Type",
            "array-display.v1",
            json!({"spec":{
                "family":"Interface","schema":{"type":"object","properties":{
                    "items":{"type":"array","items":{"$ref":c.qualified("resource-reference.v1")}},
                    "01":{"type":"string"}
                }}
            }}),
        );
        let mut template = doc(
            "Interface",
            "array-display",
            json!({"spec":{
                "type":"#array-display.v1",
                "config":{"items":["management","management"],"01":"previous"},
                "inputs":{"setting":{"schema":{"type":"string"},"to":["/config/items/01"]}}
            }}),
        );
        let instance = doc(
            "Interface",
            "indexed",
            json!({"from":"#array-display",
            "values":{"setting":source("looks-like-reference")}}),
        );
        let bad = submit(&case, &state, &c, json!([ty, template, instance]));
        assert_eq!(
            first_diagnostic(&bad)["ruleId"],
            "INS-011",
            "reject noncanonical array index"
        );
        template["spec"]["inputs"]["setting"]["to"] = json!(["/config/01"]);
        let good = submit(&case, &state, &c, json!([ty, template, instance]));
        assert_eq!(
            good.reply["isError"], false,
            "preserve numeric-looking object keys"
        );
        let object = good
            .state
            .objects
            .iter()
            .find(|o| o["name"] == "indexed")
            .unwrap();
        assert_eq!(object["config"]["01"], "management");
        assert_eq!(object["_value_sources"]["/config/01"]["kind"], "Resource");
    }

    #[test]
    fn values_regression_requirement_failure_selects_the_failing_member() {
        let (case, state, c) = case_context();
        let requirement = doc(
            "Requirement",
            "members.v1",
            json!({"spec":{"inputs":{
                "setting":{"schema":{"type":"object","properties":{
                    "first":{"type":"string"},"second":{"type":"number"}
                },"required":["first","second"]}}
            }}}),
        );
        let template = doc(
            "Conduit",
            "members-t",
            json!({"spec":{
                "capability":c.qualified("access-control.v1"),"operations":[],
                "inputs":{
                    "first":{"schema":{"type":"string"},"to":["/requirements/members/arguments/setting/first"]},
                    "second":{"schema":{"type":"string"},"to":["/requirements/members/arguments/setting/second"]}
                },
                "requirements":{"members":{"requirement":"#members.v1",
                               "arguments":{"setting":{}}}}
            }}),
        );
        let instance = doc(
            "Conduit",
            "members",
            json!({
                "from":"#members-t","consumer":{"kind":"Interface","name":"root"},"target":source("access"),
                "values":{"first":source("looks-like-reference"),"second":source("bad-scalar")}
            }),
        );
        let result = submit(&case, &state, &c, json!([requirement, template, instance]));
        let d = first_diagnostic(&result);
        assert_eq!(d["ruleId"], "INS-005");
        assert_eq!(
            location(d, 0),
            "/objects/members/values/second",
            "point to the failing value entry"
        );
        assert!(
            location(d, 1)
                .as_str()
                .unwrap()
                .ends_with("/arguments/setting/second")
        );
        assert!(
            d["message"]["text"]
                .as_str()
                .unwrap()
                .contains("name: bad-scalar")
        );
        assert!(!d.to_string().contains("synthetic-nonnumeric-value"));
    }

    #[test]
    fn values_regression_node_uses_the_authored_configuration_or_default() {
        let (case, state, c) = case_context();
        let mut template = doc(
            "Resource",
            "wrong-node-t",
            json!({"spec":{
                "type":type_reference(&c.core_package),"config":{"value":"synthetic-fixed","node":"management"}
            }}),
        );
        let instance = doc(
            "Resource",
            "wrong-node",
            json!({"from":"#wrong-node-t","owner":"system"}),
        );
        let result = submit(&case, &state, &c, json!([template, instance]));
        let d = first_diagnostic(&result);
        assert_eq!(d["ruleId"], "OBJ-009");
        assert_eq!(
            location(d, 0),
            "/objects/wrong-node-t/spec/config/node",
            "locate the fixed node"
        );
        assert_eq!(location(d, 1), "/config/node");
        assert!(
            d["message"]["text"]
                .as_str()
                .unwrap()
                .contains(r#""management""#)
        );

        template["spec"]["config"]
            .as_object_mut()
            .unwrap()
            .remove("node");
        template["spec"]["inputs"] = json!({"node":{
            "schema":{"$ref":c.qualified("resource-reference.v1")},
            "to":["/config/node"],"default":"management"
        }});
        let result = submit(&case, &state, &c, json!([template, instance]));
        let d = first_diagnostic(&result);
        assert_eq!(d["ruleId"], "OBJ-009");
        assert_eq!(
            location(d, 0),
            "/objects/wrong-node-t/spec/inputs/node/default"
        );
        assert_eq!(location(d, 1), "/config/node");
    }

    #[test]
    fn values_regression_duplicate_points_to_the_forwarded_entry() {
        let (case, state, c) = case_context();
        let template = doc(
            "Pool",
            "duplicate-t",
            json!({"spec":{
                "type":c.qualified("pool.v1"),
                "inputs":{"outer":{"schema":{"type":"string"},"to":["/entries/worker/arguments/setting"]}},
                "entries":{"worker":{"from":"forwarded-t","values":{"setting":source("bad-scalar")}}}
            }}),
        );
        let instance = doc(
            "Pool",
            "duplicate",
            json!({
                "from":"#duplicate-t","values":{"outer":source("looks-like-reference")}
            }),
        );
        let result = submit(&case, &state, &c, json!([template, instance]));
        let d = first_diagnostic(&result);
        assert_eq!(d["ruleId"], "INS-003");
        assert_eq!(
            location(d, 0),
            "/objects/duplicate-t/spec/entries/worker/values/setting"
        );
        assert_eq!(
            location(d, 1),
            "/objects/duplicate/values/outer",
            "point to the forwarded value entry"
        );
    }

    #[test]
    fn values_regression_placement_diagnostic_distinguishes_unplaced_instances() {
        let (case, mut state, c) = case_context();
        let input = serde_json::from_value(case["steps"][2]["input"].clone()).unwrap();
        let result = crate::step(&state, input, &c).unwrap();
        let d = first_diagnostic(&result);
        assert_eq!(d["ruleId"], "OBJ-009");
        assert_eq!(
            d["locations"].as_array().unwrap().len(),
            1,
            "no placement location for an Interface"
        );
        assert!(d["message"]["text"].as_str().unwrap().contains(
            "the value Resource is tied to node local; an Interface, a Pool or a Conduit takes no value tied to a node"
        ));

        let mut second = state
            .objects
            .iter()
            .find(|o| o["name"] == "local")
            .unwrap()
            .clone();
        second["name"] = json!("elsewhere");
        second["id"] = json!("00000000-0000-4000-a288-990000000001");
        state.objects.push(second);
        let ty = doc(
            "Type",
            "placed.v1",
            json!({"spec":{"family":"Resource",
                     "schema":{"type":"object","properties":{"setting":{"type":"string"}}}}}),
        );
        let template = doc(
            "Resource",
            "placed-t",
            json!({"spec":{
                "type":"#placed.v1","inputs":{"setting":{"schema":{"type":"string"},"to":["/config/setting"]}}
            }}),
        );
        let instance = doc(
            "Resource",
            "placed",
            json!({
                "from":"#placed-t","owner":"system","placement":"elsewhere",
                "values":{"setting":source("local-only")}
            }),
        );
        let result = submit(&case, &state, &c, json!([ty, template, instance]));
        let d = first_diagnostic(&result);
        assert_eq!(d["ruleId"], "OBJ-009");
        assert_eq!(location(d, 1), "/objects/placed/placement");
        assert!(d["message"]["text"].as_str().unwrap().contains(
            "the value Resource is tied to node local; the instance is placed on node elsewhere"
        ));
    }

    #[test]
    fn values_regression_names_the_receiving_definition() {
        let (case, state, c) = case_context();
        let input = serde_json::from_value(case["steps"][7]["input"].clone()).unwrap();
        let result = crate::step(&state, input, &c).unwrap();
        let d = first_diagnostic(&result);
        assert_eq!(location(d, 0), "/objects/filtered/values/setting");
        assert_eq!(
            location(d, 1),
            "/objects/filtered-t/spec/requirements/value-filter/arguments/setting",
            "identify the receiving template",
        );
    }

    #[test]
    fn values_regression_distinguishes_cycles_from_rejected_source_documents() {
        let (case, state, c) = case_context();
        let make = |name, dependency| {
            doc(
                "Resource",
                name,
                json!({
                    "from":c.qualified("value"),"owner":"system","values":{"value":source(dependency)}
                }),
            )
        };
        let a = make("cycle-a", "cycle-b");
        let b = make("cycle-b", "cycle-a");
        let result = submit(&case, &state, &c, json!([a, b]));
        let d = first_diagnostic(&result);
        assert_eq!(d["ruleId"], "OBJ-009");
        assert!(
            d["message"]["text"]
                .as_str()
                .unwrap()
                .contains("dependency cycle"),
            "diagnose a dependency cycle"
        );
        assert_eq!(location(d, 0), "/objects/cycle-a/values/value");

        let bad = make("rejected-source", "missing-source");
        let dependant = make("dependant", "rejected-source");
        let result = submit(&case, &state, &c, json!([dependant, bad]));
        let diagnostics = result.reply["structuredContent"]["diagnostics"]
            .as_array()
            .unwrap();
        let d = diagnostics
            .iter()
            .find(|d| location(d, 0) == "/objects/dependant/values/value")
            .unwrap();
        assert!(
            d["message"]["text"]
                .as_str()
                .unwrap()
                .contains("its own document was rejected"),
            "diagnose the rejected source document"
        );
        assert!(
            diagnostics
                .iter()
                .any(|d| location(d, 0) == "/objects/rejected-source/values/value")
        );
    }

    #[test]
    fn acc057_uses_the_type_of_the_selected_template_and_of_a_held_entry() {
        let context = Context {
            builtin: None,
            core_package: "github.com/shrav-k/conduit/core".into(),
            meta_prefix: "io.github.shrav-k.conduit/".into(),
            templates: BTreeMap::new(),
            record_source: "urn:test".into(),
            core_environment: Environment {
                scopes: BTreeMap::new(),
            },
        };
        let core_type = type_reference(&context.core_package);
        let state = State {
            objects: vec![
                json!({"id":"entry-template","name":"entry-t","kind":"Resource","spec":{"type":core_type}}),
                json!({"id":"owner","name":"owner","kind":"Pool","entries":{"entry":{"from":{"kind":"Resource","id":"entry-template"}}}}),
            ],
            ..State::default()
        };
        assert!(restricted_replacement(
            &state,
            &json!({"instance":{"kind":"Pool","name":"owner"},"nested":"entry"}),
            &context
        ));
        let definition = json!({"kind":"Pool","metadata":{"name":"owner-t"},"spec":{"entries":{"entry":{"from":"entry-t"}}}});
        assert!(
            restricted_submission(
                &state,
                std::slice::from_ref(&definition),
                None,
                &[],
                &context
            )
            .is_none()
        );
        let documents = [definition, json!({"kind":"Pool","from":"#owner-t"})];
        let restriction = restricted_submission(&state, &documents, None, &[], &context).unwrap();
        assert_eq!(restriction.wire_location(), "/documents/1/entries/entry");
    }
}
