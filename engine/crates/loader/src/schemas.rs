//! The schemas of the specification, bundled, and the validation of a document against them.
//!
//! The Schema contracts are read from `spec/schemas/` and `spec/core/` when the specification is
//! read, never copied, and bundled exactly as `bundle()` in `tools/check_schemas.py` bundles
//! them: the `spec` of every Schema contract under one `$defs`, keyed by its name, with each
//! `$ref` rewritten to point inside it. A `$ref` takes one of three forms (SCH-015): `#` and a
//! JSON Pointer, which is rewritten into the contract's own copy; `#name`, a Schema contract of
//! the same package; and a qualified reference to the core package, whose address is read from
//! `spec/names.yaml`. Only the places of a schema that hold schemas are visited, so a `$ref`
//! written inside `const`, `enum`, `default` or `examples` is a value and is left alone.

use std::collections::BTreeMap;
use std::path::Path;

use jsonschema::Validator;
use jsonschema::error::ValidationErrorKind;
use serde_json::{Map, Value as Json};
use std::sync::OnceLock;

use crate::authored;
use crate::diagnostic::{Diagnostic, Diagnostics, SchemaLocation};
use crate::formats;
use crate::project;
use crate::read::{Document, read_file};
use crate::rules::{NAM_027, SCH_001, SCH_015, SCH_016, SCT_002, SUB_005};
use crate::value::Node;

/// The names that `spec/names.yaml` defines and the loader uses. They are read from that file
/// and never written here (OBJ-024).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Names {
    /// The address of the core package, which a qualified `$ref` in its schemas names.
    pub core_package: String,
    /// The value of `apiVersion` in a document written for this version.
    pub api_version: String,
}

impl Names {
    /// The names, from the one document of `spec/names.yaml`.
    pub fn from_node(node: &Node) -> Result<Names, Diagnostic> {
        let read = |key: &str| {
            node.get(key)
                .and_then(Node::as_str)
                .map(str::to_string)
                .ok_or_else(|| Diagnostic::new(SCH_001, format!("names.yaml states no `{key}`")))
        };
        Ok(Names {
            core_package: read("corePackage")?,
            api_version: read("apiVersion")?,
        })
    }
}

/// What a document is validated as: a document of one of the ten kinds, against `document.v1`,
/// or a manifest, against `manifest.v1` (spec/schemas/README.md, "Which schema to use").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Document,
    Manifest,
}

/// The six families and the five kinds of contract (OBJ-020).
const FAMILIES: [&str; 6] = ["Interface", "Agent", "Resource", "Pool", "Fleet", "Conduit"];
const CONTRACTS: [&str; 5] = ["Type", "Capability", "Operation", "Requirement", "Schema"];

/// The schemas of the specification, read and bundled, with a validator for each schema that
/// a document is checked against.
pub struct Specification {
    names: Names,
    contracts: BTreeMap<String, Json>,
    envelope: Validator,
    document: Validator,
    manifest: Validator,
    /// The closed schema of each role, which says plainly why a document is not valid where
    /// `document.v1`, which joins them, would only say that no branch matched.
    template: Validator,
    instance: Validator,
    contract: Validator,
    sectioned: OnceLock<Result<Validator, Diagnostics>>,
}

impl std::fmt::Debug for Specification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Specification")
            .field("names", &self.names)
            .field("contracts", &self.contracts.len())
            .finish_non_exhaustive()
    }
}

impl Specification {
    /// Read the specification from its directory, `spec/`: the names from `names.yaml`, and the
    /// Schema contracts from the documents of `schemas/` and `core/`, each directory read as
    /// MAN-011 reads a project's. Reads the file system.
    pub fn read(spec: &Path) -> Result<Specification, Diagnostics> {
        let names = read_file(&spec.join("names.yaml"))?;
        let [names] = names.as_slice() else {
            return Err(Diagnostic::new(SCH_001, "names.yaml does not hold one document").into());
        };
        let names = Names::from_node(&names.root)?;
        let mut documents = Vec::new();
        for directory in ["schemas", "core"] {
            let (files, mut faults) = project::directory_files(&spec.join(directory))?;
            let (read, read_faults) = project::read_files(&files, Some(&spec.join(directory)));
            faults.extend(read_faults);
            if !faults.is_empty() {
                return Err(Diagnostics(faults));
            }
            documents.extend(read.documents.into_iter().flatten());
        }
        Specification::from_documents(names, &documents)
    }

    /// Bundle the Schema contracts among these documents and make the validators. Pure.
    pub fn from_documents(
        names: Names,
        documents: &[Document],
    ) -> Result<Specification, Diagnostics> {
        let mut contracts = BTreeMap::new();
        for document in documents {
            let root = &document.root;
            if root.get("kind").and_then(Node::as_str) != Some("Schema") {
                continue;
            }
            let name = root
                .get("metadata")
                .and_then(|m| m.get("name"))
                .and_then(Node::as_str);
            let (Some(name), Some(spec)) = (name, root.get("spec")) else {
                continue;
            };
            if contracts.insert(name.to_string(), spec.to_json()).is_some() {
                return Err(Diagnostic::new(
                    NAM_027,
                    format!("two Schema contracts are named {name}"),
                )
                .in_file(document.file.clone())
                .into());
            }
        }
        let bundle = bundle(&contracts, &names.core_package)?;
        let build = |name: &str| validator(&bundle, &contracts, name);
        Ok(Specification {
            envelope: build("envelope.v1")?,
            document: build("document.v1")?,
            manifest: build("manifest.v1")?,
            template: build("template.v1")?,
            instance: build("instance.v1")?,
            contract: build("contract.v1")?,
            sectioned: OnceLock::new(),
            names,
            contracts,
        })
    }

    pub fn names(&self) -> &Names {
        &self.names
    }

    /// The `spec` of a Schema contract of the core package, as written.
    pub fn contract(&self, name: &str) -> Option<&Json> {
        self.contracts.get(name)
    }

    /// Compile a built-in Schema contract with its references resolved, using the same
    /// profile as document validation. Pure; used by the conformance runner for records.
    pub fn contract_validator(&self, name: &str) -> Result<Validator, Diagnostics> {
        let bundle = bundle(&self.contracts, &self.names.core_package)?;
        validator(&bundle, &self.contracts, name).map_err(Into::into)
    }

    /// The names of the Schema contracts, in order.
    pub fn contract_names(&self) -> impl Iterator<Item = &str> {
        self.contracts.keys().map(String::as_str)
    }

    /// Check one document as the loader checks every document it reads: against the schemas,
    /// by [`Specification::validate`], and then, where the schemas accept it, each pattern
    /// its own schemas hold, which must be one of ECMA-262 (SCH-016). This is the verdict that
    /// [`crate::read_project`] gives a document.
    pub fn check(&self, document: &Document, role: Role) -> Result<(), Diagnostics> {
        self.validate(document, role)?;
        let faults = match role {
            Role::Document => authored::check_patterns(document),
            Role::Manifest => Vec::new(),
        };
        if faults.is_empty() {
            Ok(())
        } else {
            Err(Diagnostics(faults))
        }
    }

    /// Validate one document against the schemas alone, as `validate()` in
    /// `tools/check_schemas.py` does: a manifest against `manifest.v1`; any other document
    /// against `envelope.v1`, and then `document.v1`. A rejection rests on SUB-005, whichever
    /// promise the Schema contract encodes, as SUB-005 says, and names the Schema contract that
    /// rejected the document, and the place as a JSON Pointer. The rule on the patterns a
    /// document's own schemas hold is not applied: [`Specification::check`] applies both.
    pub fn validate(&self, document: &Document, role: Role) -> Result<(), Diagnostic> {
        let instance = document.root.to_json();
        let failure = match role {
            Role::Manifest => first_failure(&self.manifest, &instance),
            Role::Document => first_failure(&self.envelope, &instance).or_else(|| {
                let found = first_failure(&self.document, &instance)?;
                let narrow = match narrow_role(&instance) {
                    Some("contract.v1") => &self.contract,
                    Some("instance.v1") => &self.instance,
                    Some(_) => &self.template,
                    None => return Some(found),
                };
                Some(first_failure(narrow, &instance).unwrap_or(found))
            }),
        };
        failure.map_or(Ok(()), |failure| {
            Err(schema_diagnostic(document, failure, false))
        })
    }

    pub(crate) fn check_sectioned(
        &self,
        document: &Document,
        version: Option<&Node>,
    ) -> Result<(), Diagnostics> {
        let validator = self
            .sectioned
            .get_or_init(|| self.contract_validator("sectioned.v1"));
        let validator = validator.as_ref().map_err(Clone::clone)?;
        // A stated version belongs to this schema check. The manifest's check owns
        // the diagnostic for an inherited version.
        let instance = document.root.to_json();
        if document.root.get("metadata").is_some() || document.root.get("spec").is_some() {
            let mut failure =
                first_failure(validator, &instance).expect("framed keys fail sectioned.v1");
            failure.message = "a framed document needs kind".into();
            failure.contract = "sectioned.v1".into();
            return Err(schema_diagnostic(document, failure, true).into());
        }
        let mut failures = Vec::new();
        for error in validator.iter_errors(&instance) {
            sectioned_failures(&error, &mut failures);
        }
        locate_keywords(validator, &instance, &mut failures);
        let mut diagnostics: Vec<_> = failures
            .into_iter()
            .map(|failure| schema_diagnostic(document, failure, true))
            .collect();
        if version.is_none() {
            diagnostics.push(
                Diagnostic::new(SCT_002, "apiVersion is required when the manifest has none")
                    .in_file(document.file.clone())
                    .in_document(document.index)
                    .at_pointer("/apiVersion")
                    .at(document.root.position),
            );
        }
        diagnostics.sort_by(|a, b| {
            a.pointer
                .cmp(&b.pointer)
                .then(a.keyword_location().cmp(&b.keyword_location()))
        });
        diagnostics.dedup();
        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(Diagnostics(diagnostics))
        }
    }
}

fn schema_diagnostic(document: &Document, failure: Failure, authored_member: bool) -> Diagnostic {
    let mut pointer = failure.instance_path;
    if authored_member && let Some(member) = failure.member {
        pointer.push('/');
        pointer.push_str(&member.replace('~', "~0").replace('/', "~1"));
    }
    let position = document.root.locate(&pointer).position;
    let mut diagnostic = Diagnostic::new(
        SUB_005,
        format!(
            "not valid against `{}`: {}",
            failure.contract, failure.message
        ),
    )
    .in_file(document.file.clone())
    .in_document(document.index)
    .at_pointer(pointer)
    .at(position);
    diagnostic.set_schema_location(SchemaLocation {
        keyword_location: failure.keyword_location,
        absolute_keyword_location: failure.absolute_keyword_location,
    });
    diagnostic
}

/// Compile a schema with the validator's options (see [`formats::options`]). Used for the
/// schemas of the specification, and by tests for schemas written for a case.
pub fn compile(schema: &Json) -> Result<Validator, Diagnostic> {
    formats::options().build(schema).map_err(|error| {
        let message = error.to_string();
        let rule = if message.contains(formats::NOT_ECMA) {
            SCH_016
        } else {
            SCH_001
        };
        Diagnostic::new(rule, format!("the schema cannot be compiled: {message}"))
            .at_pointer(error.schema_path().as_str().to_string())
    })
}

fn validator(
    bundle: &Json,
    contracts: &BTreeMap<String, Json>,
    name: &str,
) -> Result<Validator, Diagnostic> {
    if !contracts.contains_key(name) {
        return Err(Diagnostic::new(
            SCH_015,
            format!("no Schema contract is named {name}"),
        ));
    }
    let mut schema = bundle.clone();
    if let Json::Object(root) = &mut schema {
        root.insert(
            "$ref".into(),
            Json::String(format!("#/$defs/{}", token(name))),
        );
    }
    compile(&schema).map_err(|mut d| {
        d.message = format!("{name}: {}", d.message).into_boxed_str();
        d
    })
}

/// A name as one reference token of a JSON Pointer (RFC 6901). A contract's name has neither
/// `~` nor `/`, so this changes nothing for the names of the specification.
fn token(name: &str) -> String {
    name.replace('~', "~0").replace('/', "~1")
}

/// One JSON Schema that holds every Schema contract under `$defs`, with each `$ref` rewritten
/// to point inside it.
fn bundle(contracts: &BTreeMap<String, Json>, core: &str) -> Result<Json, Diagnostics> {
    let qualified = format!("{core}#");
    let mut faults = Vec::new();
    let mut defs = Map::new();
    for (name, spec) in contracts {
        let mut spec = spec.clone();
        walk(&mut spec, &mut |schema| {
            let Some(Json::String(reference)) = schema.get("$ref") else {
                return;
            };
            let rewritten = if reference == "#" || reference.starts_with("#/") {
                Some(format!("#/$defs/{}{}", token(name), &reference[1..]))
            } else if let Some(target) =
                local_name(reference).or_else(|| reference.strip_prefix(&qualified))
            {
                contracts
                    .contains_key(target)
                    .then(|| format!("#/$defs/{}", token(target)))
            } else {
                None
            };
            match rewritten {
                Some(rewritten) => {
                    schema.insert("$ref".into(), Json::String(rewritten));
                }
                None => faults.push(Diagnostic::new(
                    SCH_015,
                    format!("{name}: `$ref` {reference} is not `#` and a JSON Pointer, `#name` or a qualified reference to a Schema contract of the core package"),
                )),
            }
        });
        defs.insert(name.clone(), spec);
    }
    if !faults.is_empty() {
        return Err(Diagnostics(faults));
    }
    let mut root = Map::new();
    root.insert(
        "$schema".into(),
        Json::String("https://json-schema.org/draft/2020-12/schema".into()),
    );
    root.insert("$defs".into(), Json::Object(defs));
    Ok(Json::Object(root))
}

/// The name in `#name`: `#` and segments of a lowercase letter and then lowercase letters,
/// digits and hyphens, joined by dots.
fn local_name(reference: &str) -> Option<&str> {
    let name = reference.strip_prefix('#')?;
    let segment = |s: &str| {
        let mut chars = s.chars();
        chars.next().is_some_and(|c| c.is_ascii_lowercase())
            && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    };
    name.split('.').all(segment).then_some(name)
}

/// The keywords of the profile whose value is a map of schemas, one schema, or a list of
/// schemas, as `subschemas()` in `tools/check_schemas.py` lists them.
pub(crate) const MAP_OF_SCHEMAS: [&str; 4] = [
    "properties",
    "$defs",
    "patternProperties",
    "dependentSchemas",
];
pub(crate) const ONE_SCHEMA: [&str; 8] = [
    "additionalProperties",
    "items",
    "contains",
    "propertyNames",
    "not",
    "if",
    "then",
    "else",
];
pub(crate) const LIST_OF_SCHEMAS: [&str; 4] = ["allOf", "anyOf", "oneOf", "prefixItems"];

/// Visit every schema object inside a schema, the schema itself first.
fn walk(schema: &mut Json, visit: &mut dyn FnMut(&mut Map<String, Json>)) {
    let Json::Object(map) = schema else {
        return;
    };
    visit(map);
    for keyword in MAP_OF_SCHEMAS {
        if let Some(Json::Object(members)) = map.get_mut(keyword) {
            for sub in members.values_mut() {
                walk(sub, visit);
            }
        }
    }
    for keyword in ONE_SCHEMA {
        if let Some(sub) = map.get_mut(keyword) {
            walk(sub, visit);
        }
    }
    for keyword in LIST_OF_SCHEMAS {
        if let Some(Json::Array(items)) = map.get_mut(keyword) {
            for sub in items {
                walk(sub, visit);
            }
        }
    }
}

/// The closed schema that a document of this role is most plainly meant for, as `expected()`
/// in `tools/check_schemas.py` chooses it.
fn narrow_role(instance: &Json) -> Option<&'static str> {
    let kind = instance.get("kind")?.as_str()?;
    if CONTRACTS.contains(&kind) {
        Some("contract.v1")
    } else if FAMILIES.contains(&kind) {
        Some(if instance.get("from").is_some() {
            "instance.v1"
        } else {
            "template.v1"
        })
    } else {
        None
    }
}

/// Why a value is not valid, in the terms a diagnostic needs.
#[derive(Clone)]
struct Failure {
    /// Where in the document, as a JSON Pointer.
    instance_path: String,
    member: Option<String>,
    /// The Schema contract whose keyword failed.
    contract: String,
    keyword_location: String,
    schema_location: String,
    absolute_keyword_location: Option<String>,
    message: String,
}

/// The first failure, in the order of the places it is at, or none where the value is valid.
fn first_failure(validator: &Validator, instance: &Json) -> Option<Failure> {
    let mut errors: Vec<_> = validator.iter_errors(instance).collect();
    errors.sort_by(|a, b| a.instance_path().as_str().cmp(b.instance_path().as_str()));
    let mut failure = errors.first().map(|error| explain(error))?;
    locate_keywords(validator, instance, std::slice::from_mut(&mut failure));
    Some(failure)
}

fn locate_keywords(validator: &Validator, instance: &Json, failures: &mut [Failure]) {
    if failures.is_empty() {
        return;
    }
    // Custom keywords in jsonschema 0.61 lose the reference tracker in iter_errors.
    // Its evaluation tree retains it: use that path for JSON Schema 2020-12's
    // keywordLocation, which must include every by-reference applicator.
    let evaluation = validator.evaluate(instance);
    let mut units = vec![evaluation.root()];
    let mut locations = BTreeMap::new();
    while let Some(unit) = units.pop() {
        if !unit.valid() && !unit.errors().is_empty() {
            locations
                .entry((
                    unit.schema_location().to_string(),
                    unit.instance_location().as_str().to_string(),
                ))
                .or_insert_with(|| unit.evaluation_path().as_str().to_string());
        }
        units.extend(unit.children());
    }
    for failure in failures {
        if let Some(location) = locations.get(&(
            failure.schema_location.clone(),
            failure.instance_path.clone(),
        )) {
            failure.keyword_location.clone_from(location);
        }
    }
}

/// Say why a value is not valid. Where a schema offers alternatives, follow the one the value
/// was most plainly meant for, as `explain()` in `tools/check_schemas.py` does: the one whose
/// `kind` it has, then one whose type it has, then the one it breaks in the fewest places.
fn best_branch<'a>(
    error: &'a jsonschema::ValidationError<'_>,
) -> Option<&'a [jsonschema::ValidationError<'static>]> {
    let branches = match error.kind() {
        ValidationErrorKind::AnyOf { context } | ValidationErrorKind::OneOfNotValid { context } => {
            Some(context)
        }
        _ => None,
    };
    let parent = error.instance_path().as_str();
    branches.and_then(|branches| {
        branches
            .iter()
            .filter(|subs| !subs.is_empty())
            .min_by_key(|subs| {
                let kind = format!("{parent}/kind");
                let other_kind = subs.iter().any(|s| {
                    matches!(
                        s.kind(),
                        ValidationErrorKind::Constant { .. } | ValidationErrorKind::Enum { .. }
                    ) && s.instance_path().as_str() == kind
                });
                let wrong_type = subs.iter().any(|s| {
                    matches!(s.kind(), ValidationErrorKind::Type { .. })
                        && s.instance_path().as_str() == parent
                });
                let deepest = subs
                    .iter()
                    .map(|s| depth(s.instance_path().as_str()))
                    .max()
                    .unwrap_or(0);
                (other_kind, wrong_type, subs.len(), usize::MAX - deepest)
            })
            .map(Vec::as_slice)
    })
}

fn explain(error: &jsonschema::ValidationError<'_>) -> Failure {
    if let Some(subs) = best_branch(error) {
        let has_context = |s: &jsonschema::ValidationError<'_>| {
            matches!(
                s.kind(),
                ValidationErrorKind::AnyOf { context } | ValidationErrorKind::OneOfNotValid { context }
                    if !context.is_empty()
            )
        };
        if let Some(chosen) = subs
            .iter()
            .max_by_key(|s| (has_context(s), depth(s.instance_path().as_str())))
        {
            return explain(chosen);
        }
    }
    describe(error)
}

fn sectioned_failures(error: &jsonschema::ValidationError<'_>, failures: &mut Vec<Failure>) {
    if let Some(branch) = best_branch(error) {
        for error in branch {
            sectioned_failures(error, failures);
        }
        return;
    }
    let mut failure = describe(error);
    match error.kind() {
        ValidationErrorKind::AdditionalProperties { unexpected } if unexpected.len() > 1 => {
            for member in unexpected {
                let mut fault = failure.clone();
                fault.member = Some(member.clone());
                fault.message = format!("additional property {member:?} is not allowed");
                failures.push(fault);
            }
            return;
        }
        ValidationErrorKind::Required { property } if !failure.instance_path.is_empty() => {
            failure.member = property.as_str().map(str::to_owned);
        }
        _ => {}
    }
    failures.push(failure);
}

fn describe(error: &jsonschema::ValidationError<'_>) -> Failure {
    let parent = error.instance_path().as_str();
    let schema_path = error.schema_path().as_str();
    let mut tokens = schema_path.split('/');
    let contract = tokens
        .by_ref()
        .find(|t| *t == "$defs")
        .and_then(|_| tokens.next())
        .map_or_else(
            || "the bundle".to_string(),
            |t| t.replace("~1", "/").replace("~0", "~"),
        );
    let mut protected = vec![];
    protected_schema_values(error, &mut protected);
    let mut message = crate::secret::validation_message(error, &protected);
    if message.chars().count() > 200 {
        message = message.chars().take(197).collect::<String>() + "...";
    }
    Failure {
        instance_path: parent.to_string(),
        member: match error.kind() {
            ValidationErrorKind::AdditionalProperties { unexpected } => unexpected.first().cloned(),
            ValidationErrorKind::PropertyNames { error } => {
                error.instance().as_str().map(str::to_owned)
            }
            _ => None,
        },
        contract,
        keyword_location: error.evaluation_path().as_str().to_string(),
        schema_location: error
            .absolute_keyword_location()
            .map_or_else(|| schema_path.to_string(), ToString::to_string),
        absolute_keyword_location: error.absolute_keyword_location().map(ToString::to_string),
        message,
    }
}

fn protected_schema_values<'a>(
    error: &'a jsonschema::ValidationError<'_>,
    protected: &mut Vec<&'a Json>,
) {
    let points_to_secret = error
        .schema_path()
        .as_str()
        .contains("/$defs/secret-reference.v1/")
        || error.absolute_keyword_location().is_some_and(|uri| {
            uri.as_str().contains("/$defs/secret-reference.v1/")
                || uri.as_str().ends_with("#secret-reference.v1")
        });
    if points_to_secret {
        protected.push(error.instance());
    } else if let ValidationErrorKind::AnyOf { context }
    | ValidationErrorKind::OneOfNotValid { context } = error.kind()
    {
        for child in context.iter().flatten() {
            protected_schema_values(child, protected);
        }
    }
}

fn depth(pointer: &str) -> usize {
    pointer.matches('/').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_secret_reference_failure_withholds_its_value() {
        let schema = serde_json::json!({
            "properties": {
                "protected": {"$ref": "#/$defs/secret-reference.v1"},
                "secret-reference.v1": {"type": "object"},
            },
            "$defs": {"secret-reference.v1": {"type": "object"}},
        });
        let validator = compile(&schema).unwrap();
        let sample = "synthetic-value-for-redaction";
        for (field, protected) in [("protected", true), ("secret-reference.v1", false)] {
            let instance = serde_json::json!({(field): sample});
            let failure = first_failure(&validator, &instance).unwrap();
            assert_eq!(failure.instance_path, format!("/{field}"));
            assert_eq!(failure.message.contains(sample), !protected, "{field}");
            assert_eq!(
                failure.message.contains(crate::secret::WITHHELD_VALUE),
                protected
            );
        }
    }

    #[test]
    fn a_known_absolute_keyword_location_is_kept() {
        for (constraint, instance, keyword) in [
            (
                serde_json::json!({"type": "string"}),
                serde_json::json!({"label": 1}),
                "type",
            ),
            (
                serde_json::json!({"pattern": "^a$"}),
                serde_json::json!({"label": "b"}),
                "pattern",
            ),
        ] {
            let validator = compile(&serde_json::json!({
                "$id": "https://example.org/schema",
                "properties": {"label": constraint}
            }))
            .unwrap();
            let failure = first_failure(&validator, &instance).unwrap();
            assert_eq!(
                failure.keyword_location,
                format!("/properties/label/{keyword}")
            );
            assert_eq!(
                failure.absolute_keyword_location,
                Some(format!(
                    "https://example.org/schema#/properties/label/{keyword}"
                ))
            );
        }
    }
}
