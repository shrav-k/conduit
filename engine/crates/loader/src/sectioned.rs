//! Pure translation of sectioned project documents into the existing framed form.

use std::collections::{BTreeMap, btree_map::Entry};

use crate::{Diagnostic, Document, Node, Specification, Value};

const FAMILIES: [(&str, &str); 5] = [
    ("interfaces", "Interface"),
    ("agents", "Agent"),
    ("resources", "Resource"),
    ("pools", "Pool"),
    ("conduits", "Conduit"),
];
type Members = Vec<(String, Node)>;

/// An authored place in a project, before any translation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslationSource {
    pub file: String,
    pub document: usize,
    pub pointer: String,
}

/// The rule and authored places of a generated document or supplied Fleet owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Translation {
    pub rule: &'static str,
    pub sources: Vec<TranslationSource>,
}

#[derive(Default)]
struct Output {
    documents: Vec<Document>,
    translations: Vec<Option<Translation>>,
}

impl Output {
    fn push(&mut self, document: Document, translation: Option<Translation>) {
        self.documents.push(document);
        self.translations.push(translation);
    }
}

fn sectioned(document: &Document) -> bool {
    matches!(document.root.value, Value::Object(_)) && document.root.get("kind").is_none()
}

fn has_framed_keys(document: &Document) -> bool {
    document.root.get("metadata").is_some() || document.root.get("spec").is_some()
}

fn usable_version<'a>(
    document: &'a Document,
    inherited: Option<&'a Node>,
    spec: &Specification,
) -> Option<&'a Node> {
    document
        .root
        .get("apiVersion")
        .or(inherited)
        .filter(|version| version.as_str() == Some(spec.names().api_version.as_str()))
}

fn member_pointer(parent: &str, name: &str) -> String {
    format!("{parent}/{}", name.replace('~', "~0").replace('/', "~1"))
}

fn path_contains(parent: &str, child: &str) -> bool {
    parent == child
        || child
            .strip_prefix(parent)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

#[derive(Default)]
struct MemberFaults(Vec<String>);

impl MemberFaults {
    fn blocks(&self, pointer: &str) -> bool {
        self.0
            .iter()
            .any(|fault| path_contains(fault, pointer) || path_contains(pointer, fault))
    }
}

pub(crate) fn translate(
    files: Vec<Vec<Document>>,
    manifest: Option<&Document>,
    spec: &Specification,
    archive: bool,
    faults: &mut Vec<Diagnostic>,
) -> (Vec<Document>, Vec<Option<Translation>>) {
    let mut output = Output::default();
    let mut shared_fleets: BTreeMap<String, usize> = BTreeMap::new();
    let inherited = manifest.and_then(|manifest| manifest.root.get("apiVersion"));
    let core = &spec.names().core_package;
    for documents in files {
        let mut fleet_faults = Vec::new();
        let file_fleet = if archive {
            None
        } else {
            check_file_fleets(&documents, &mut fleet_faults)
        };
        let mut owner_usable = false;
        let mut checked = Vec::new();
        for document in documents {
            let mut member_faults = MemberFaults::default();
            if sectioned(&document) {
                let version = document.root.get("apiVersion").or(inherited);
                let before = faults.len();
                if let Err(found) = spec.check_sectioned(&document, version) {
                    faults.extend(found.0);
                }
                if archive {
                    for key in std::iter::once("fleet").chain(FAMILIES.map(|(section, _)| section))
                    {
                        if document.root.get(key).is_some() {
                            faults.push(fault(
                                &document,
                                "MAN-011",
                                &format!("/{key}"),
                                format!("an archive's sectioned document cannot hold {key}"),
                            ));
                        }
                    }
                }
                if has_framed_keys(&document) {
                    continue;
                }
                member_faults.0.extend(
                    faults[before..]
                        .iter()
                        .map(|fault| fault.pointer.clone().unwrap_or_default()),
                );
                if file_fleet
                    .as_ref()
                    .is_some_and(|(_, place)| place.document == document.index)
                {
                    owner_usable = !member_faults.blocks("/fleet");
                }
            }
            checked.push((document, member_faults));
        }
        faults.extend(fleet_faults);
        let fleet = file_fleet
            .as_ref()
            .filter(|_| owner_usable)
            .map(|(name, place)| (name.as_str(), place));
        if let Some((name, location)) = fleet
            && let Some((document, member_faults)) = checked
                .iter()
                .find(|(document, _)| document.index == location.document)
            && let Some(version) = usable_version(document, inherited, spec)
        {
            match shared_fleets.entry(name.to_owned()) {
                Entry::Occupied(found) => output.translations[*found.get()]
                    .as_mut()
                    .expect("Fleet provenance")
                    .sources
                    .push(location.clone()),
                Entry::Vacant(place) => {
                    place.insert(output.documents.len());
                    let mut expansion = Expansion {
                        document,
                        version: Some(version),
                        member_faults,
                        core,
                        output: &mut output,
                        faults,
                    };
                    expansion.emit(
                        ("/fleet", "SCT-011"),
                        "Fleet",
                        name,
                        None,
                        vec![(
                            "from".into(),
                            string(format!("{core}#fleet"), &document.root),
                        )],
                    );
                }
            }
        }
        for (mut document, member_faults) in checked {
            if sectioned(&document) {
                let mut expansion = Expansion {
                    document: &document,
                    version: usable_version(&document, inherited, spec),
                    member_faults: &member_faults,
                    core,
                    output: &mut output,
                    faults,
                };
                expansion.expand(fleet.map(|(name, _)| name));
            } else {
                let translation = fleet
                    .and_then(|(name, place)| framed_owner(&mut document, name, place, faults));
                output.push(document, translation);
            }
        }
    }
    (output.documents, output.translations)
}

fn check_file_fleets(
    documents: &[Document],
    faults: &mut Vec<Diagnostic>,
) -> Option<(String, TranslationSource)> {
    let mut declarations = documents
        .iter()
        .filter(|document| sectioned(document) && !has_framed_keys(document))
        .filter_map(|document| {
            document
                .root
                .get("fleet")
                .map(|fleet| (document, fleet.as_str()))
        });
    let (first, first_name) = declarations.next()?;
    let first_label = first_name.unwrap_or("declaration");
    for (document, name) in declarations {
        let label = name.unwrap_or("declaration");
        faults.push(fault(document, "SCT-011", "/fleet", format!(
            "a second fleet {label} in the file; the first fleet {first_label} is in document {} at /fleet",
            first.index
        )));
    }
    first_name.map(|name| (name.to_owned(), source(first, "/fleet")))
}

fn source(document: &Document, pointer: &str) -> TranslationSource {
    TranslationSource {
        file: document.file.clone(),
        document: document.index,
        pointer: pointer.into(),
    }
}

fn fault(
    document: &Document,
    rule: &'static str,
    pointer: &str,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic::new(rule, message)
        .in_file(document.file.clone())
        .in_document(document.index)
        .at_pointer(pointer)
        .at(document.root.locate(pointer).position)
}

fn framed_owner(
    document: &mut Document,
    fleet: &str,
    place: &TranslationSource,
    faults: &mut Vec<Diagnostic>,
) -> Option<Translation> {
    if document.root.get("kind").and_then(Node::as_str) != Some("Agent")
        || document.root.get("from").is_none()
    {
        return None;
    }
    if let Some(owner) = document.root.get("owner") {
        check_owner(document, owner, fleet, "/owner", faults);
        return None;
    }
    let owner = string(fleet, &document.root);
    if let Value::Object(fields) = &mut document.root.value {
        fields.push(("owner".into(), owner));
    }
    Some(Translation {
        rule: "SCT-011",
        sources: vec![place.clone()],
    })
}

fn check_owner(
    document: &Document,
    owner: &Node,
    fleet: &str,
    pointer: &str,
    faults: &mut Vec<Diagnostic>,
) {
    let name = owner
        .as_str()
        .or_else(|| owner.get("name").and_then(Node::as_str));
    if let Some(name) = name
        && name != fleet
    {
        faults.push(fault(
            document,
            "SCT-011",
            pointer,
            format!("the file's /fleet names {fleet}, but the Agent's owner is {name}"),
        ));
    }
}

struct Expansion<'a> {
    document: &'a Document,
    version: Option<&'a Node>,
    member_faults: &'a MemberFaults,
    core: &'a str,
    output: &'a mut Output,
    faults: &'a mut Vec<Diagnostic>,
}

impl Expansion<'_> {
    fn expand(&mut self, fleet: Option<&str>) {
        let root = &self.document.root;
        for (section, family) in FAMILIES {
            let templates = root
                .get("templates")
                .and_then(|templates| templates.get(section));
            for (name, item) in members(templates) {
                let place = member_pointer(&format!("/templates/{section}"), name);
                if self.member_faults.blocks(&place) {
                    continue;
                }
                let spec = members(Some(item))
                    .iter()
                    .filter(|(key, _)| key != "description" && key != "annotations")
                    .cloned()
                    .collect();
                self.emit(
                    (&place, "SCT-005"),
                    family,
                    name,
                    Some(item),
                    vec![("spec".into(), object(spec, item))],
                );
            }
            for (name, item) in members(root.get(section)) {
                let place = member_pointer(&format!("/{section}"), name);
                if family == "Conduit" {
                    if self.member_faults.blocks(&place) {
                        continue;
                    }
                    let mut ends = copy(item, &["consumer", "target", "as"]);
                    self.conduit((&place, "SCT-016"), name, item, &mut ends);
                } else {
                    self.instance(&place, family, name, item, fleet);
                }
            }
        }
    }

    fn instance(
        &mut self,
        place: &str,
        family: &str,
        name: &str,
        item: &Node,
        fleet: Option<&str>,
    ) {
        if !self.member_faults.blocks(place) {
            self.base_instance(place, family, name, item, fleet);
        }
        for (label, consumed) in members(item.get("consumes")) {
            let at = member_pointer(&format!("{place}/consumes"), label);
            if self.member_faults.blocks(&at) {
                continue;
            }
            let target = consumed.get("to").expect("checked target");
            let reserved = target.get("kind").and_then(Node::as_str) == Some("Resource");
            let target_name = target.get("name").and_then(Node::as_str);
            let explicit_bound = consumed.get("from").is_none()
                && members(consumed.get("requirements"))
                    .iter()
                    .any(|(_, requirement)| {
                        requirement.get("requirement").and_then(Node::as_str)
                            == Some(&format!("{}#bound.v1", self.core))
                    });
            if reserved
                && (target_name == Some("management")
                    || (target_name == Some("access") && !explicit_bound))
            {
                self.faults.push(fault(
                    self.document,
                    "SCT-008",
                    &at,
                    "write this Conduit under conduits with its consumer and target",
                ));
                continue;
            }
            self.conduit(
                (&at, "SCT-007"),
                &format!("{name}--{label}"),
                consumed,
                &mut vec![
                    ("consumer".into(), reference(family, name, consumed)),
                    ("target".into(), target.clone()),
                    ("as".into(), string(label, consumed)),
                ],
            );
        }
        let at = format!("{place}/operator");
        if let Some(operator) = item.get("operator")
            && !self.member_faults.blocks(&at)
        {
            self.conduit(
                (&at, "SCT-009"),
                &format!("{name}--operator"),
                operator,
                &mut vec![
                    ("consumer".into(), reference("Interface", "root", operator)),
                    ("target".into(), reference(family, name, operator)),
                    ("as".into(), string(name, operator)),
                ],
            );
        }
    }

    fn base_instance(
        &mut self,
        place: &str,
        family: &str,
        name: &str,
        item: &Node,
        fleet: Option<&str>,
    ) {
        let mut fields = copy(item, &["arguments", "owner", "placement", "runtime"]);
        if family == "Agent"
            && let Some(fleet) = fleet
        {
            if let Some(owner) = item.get("owner") {
                check_owner(
                    self.document,
                    owner,
                    fleet,
                    &format!("{place}/owner"),
                    self.faults,
                );
            } else {
                fields.push(("owner".into(), string(fleet, item)));
            }
        }
        let origin = (place, "SCT-006");
        let from = self.from(
            origin,
            family,
            name,
            item,
            &[
                "type",
                "config",
                "implementation",
                "controls",
                "conduits",
                "entries",
            ],
        );
        fields.push(("from".into(), from));
        self.emit(origin, family, name, Some(item), fields);
    }

    fn conduit(
        &mut self,
        origin: (&str, &'static str),
        name: &str,
        item: &Node,
        fields: &mut Members,
    ) {
        fields.extend(copy(item, &["arguments"]));
        fields.push((
            "from".into(),
            self.from(
                origin,
                "Conduit",
                name,
                item,
                &["capability", "operations", "requirements"],
            ),
        ));
        self.emit(origin, "Conduit", name, Some(item), std::mem::take(fields));
    }

    fn from(
        &mut self,
        origin: (&str, &'static str),
        family: &str,
        name: &str,
        item: &Node,
        keys: &[&str],
    ) -> Node {
        if let Some(from) = item.get("from") {
            return from.clone();
        }
        let template = format!("{name}--t");
        self.emit(
            origin,
            family,
            &template,
            None,
            vec![("spec".into(), object(copy(item, keys), item))],
        );
        string(template, item)
    }

    fn emit(
        &mut self,
        (pointer, rule): (&str, &'static str),
        family: &str,
        name: &str,
        metadata: Option<&Node>,
        fields: Members,
    ) {
        if name.len() > 63 {
            self.faults.push(fault(
                self.document,
                "SCT-013",
                pointer,
                format!("the generated name {name} has more than 63 characters"),
            ));
            return;
        }
        // A dependent instance would have the same invalid name in its from field.
        if fields.iter().any(|(key, value)| {
            key == "from"
                && value
                    .as_str()
                    .is_some_and(|value| value == format!("{name}--t") && value.len() > 63)
        }) {
            return;
        }
        // Name checks still run without a usable version. No frame can be emitted
        // in that case, and the authored version or manifest supplies the diagnostic.
        let Some(version) = self.version else {
            return;
        };
        let anchor = self.document.root.locate(pointer);
        let mut meta = vec![("name".into(), string(name, anchor))];
        if let Some(item) = metadata {
            meta.extend(copy(item, &["description", "annotations"]));
        }
        let mut body = vec![
            ("apiVersion".into(), version.clone()),
            ("kind".into(), string(family, anchor)),
            ("metadata".into(), object(meta, anchor)),
        ];
        body.extend(fields);
        self.output.push(
            Document {
                file: self.document.file.clone(),
                index: self.document.index,
                root: object(body, anchor),
            },
            Some(Translation {
                rule,
                sources: vec![source(self.document, pointer)],
            }),
        );
    }
}

fn members(node: Option<&Node>) -> &[(String, Node)] {
    if let Some(Node {
        value: Value::Object(members),
        ..
    }) = node
    {
        members
    } else {
        &[]
    }
}

fn copy(node: &Node, keys: &[&str]) -> Members {
    keys.iter()
        .filter_map(|key| node.get(key).map(|value| ((*key).into(), value.clone())))
        .collect()
}

fn object(members: Members, anchor: &Node) -> Node {
    Node::new(Value::Object(members), anchor.position)
}

fn string(value: impl Into<String>, anchor: &Node) -> Node {
    Node::new(Value::String(value.into()), anchor.position)
}

fn reference(family: &str, name: &str, anchor: &Node) -> Node {
    object(
        vec![
            ("kind".into(), string(family, anchor)),
            ("name".into(), string(name, anchor)),
        ],
        anchor,
    )
}
