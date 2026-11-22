//! SCT-014 locations belong to the loader's project and its replies.

use serde_json::{Value, json};

use crate::diagnostic::{AuthoredLocation, DiagnosticOrigin, add_origin};
use crate::sectioned::{Translation, TranslationSource};
use crate::{Diagnostic, Document, Project};

fn name(document: &Document) -> Option<&str> {
    document.root.get("metadata")?.get("name")?.as_str()
}

fn at(document: &Document, pointer: &str) -> AuthoredLocation {
    let (node, pointer) = document.root.located_prefix(pointer);
    AuthoredLocation {
        file: document.file.clone(),
        document: document.index,
        pointer: pointer.to_owned(),
        position: node.position,
    }
}

fn escape(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

impl Project {
    fn authored_location(&self, source: &TranslationSource, suffix: &str) -> AuthoredLocation {
        let document = self
            .authored
            .iter()
            .find(|d| d.index == source.document && d.file == source.file)
            .expect("translation refers to an authored document");
        at(document, &format!("{}{suffix}", source.pointer))
    }

    fn translated_places(
        &self,
        document: &Document,
        translation: &Translation,
        rule: &str,
        pointer: &str,
    ) -> (AuthoredLocation, DiagnosticOrigin) {
        let mut suffix = pointer;
        if suffix.is_empty() {
            suffix = match rule {
                "CON-021" => "/capability",
                "IFC-014" if translation.rule == "SCT-016" => "/as",
                "FLEET-002" | "FLEET-003" | "POOL-001" | "POOL-004" => "/owner",
                _ => "",
            };
        }
        let mut sources = translation.sources.iter();
        let source = sources.next().expect("translation has an authored place");
        let primary = if let Some(owner) = &translation.owner
            && (suffix == "/owner" || suffix.starts_with("/owner/"))
        {
            self.authored_location(owner, "")
        } else {
            if translation.rule != "SCT-011" {
                suffix = strip_field(suffix, "/spec")
                    .or_else(|| strip_field(suffix, "/metadata"))
                    .unwrap_or(suffix);
            }
            if let Some(target) = strip_field(suffix, "/target")
                && translation.rule == "SCT-007"
            {
                self.authored_location(source, &format!("/to{target}"))
            } else {
                self.authored_location(source, suffix)
            }
        };
        let origin = DiagnosticOrigin {
            translation: name(document).map(|name| (translation.rule, name.to_owned())),
            related: sources
                .map(|place| self.authored_location(place, ""))
                .collect(),
        };
        (primary, origin)
    }

    pub(crate) fn locate_load_fault(&self, index: usize, mut fault: Diagnostic) -> Diagnostic {
        if let Some(origin) = &self.translations[index] {
            let (primary, origin) = self.translated_places(
                &self.documents[index],
                origin,
                fault.rule,
                fault.pointer.as_deref().unwrap_or_default(),
            );
            fault.file = Some(primary.file);
            fault.document = Some(primary.document);
            fault.pointer = Some(primary.pointer);
            fault.position = Some(primary.position);
            fault.set_origin(origin);
        }
        fault
    }

    /// Locate a SARIF diagnostic from a check of the values read from this project.
    /// The input may name the document by an object path or a document property.
    pub fn locate_diagnostic(&self, diagnostic: &mut Value) {
        let pointer = diagnostic
            .pointer("/locations/0/logicalLocations/0/fullyQualifiedName")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let (object, pointer) = pointer.strip_prefix("/objects/").map_or(
            (
                diagnostic["properties"]["document"]
                    .as_str()
                    .map(str::to_owned),
                pointer,
            ),
            |path| {
                let end = path.find('/').unwrap_or(path.len());
                (
                    Some(path[..end].replace("~1", "/").replace("~0", "~")),
                    &path[end..],
                )
            },
        );
        let rule = diagnostic["ruleId"].as_str().unwrap_or_default();
        let file = diagnostic["properties"]["file"].as_str();
        let index = diagnostic["properties"]["document"].as_u64();
        if let Some(object) = &object
            && matches!(rule, "NAM-027" | "SUB-004")
        {
            if let Some((primary, origin)) = self.collision(object) {
                write(diagnostic, &primary, &origin);
            }
            return;
        }
        let candidates: Vec<_> = self
            .documents
            .iter()
            .zip(&self.translations)
            .filter(|(d, _)| {
                if let Some(object) = &object {
                    name(d) == Some(object.as_str())
                } else {
                    index == Some(d.index as u64) && file.is_none_or(|file| file == d.file)
                }
            })
            .collect();
        let only = if candidates.len() == 1 {
            candidates.first().copied()
        } else {
            let mut at_pointer = candidates.into_iter().filter(|(d, _)| {
                !pointer.is_empty() && d.root.located_prefix(pointer).1 == pointer
            });
            at_pointer.next().filter(|_| at_pointer.next().is_none())
        };
        if let Some((document, Some(origin))) = only {
            let (primary, origin) = self.translated_places(document, origin, rule, pointer);
            write(diagnostic, &primary, &origin);
        }
    }

    fn collision(&self, object: &str) -> Option<(AuthoredLocation, DiagnosticOrigin)> {
        let mut places = vec![];
        let mut translated = false;
        for (document, origin) in self.documents.iter().zip(&self.translations) {
            if name(document) != Some(object) {
                continue;
            }
            if let Some(origin) = origin {
                translated = true;
                let (primary, origin) = self.translated_places(document, origin, "", "");
                places.push((primary, origin));
            } else {
                places.push((at(document, "/metadata/name"), empty_origin()));
            }
        }
        if !translated {
            return None;
        }
        if let Some(manifest) = &self.manifest
            && manifest
                .root
                .get("implementations")
                .and_then(|map| map.get(object))
                .is_some()
        {
            places.push((
                at(manifest, &format!("/implementations/{}", escape(object))),
                empty_origin(),
            ));
        }
        places.sort_by(|(a, _), (b, _)| place_order(a).cmp(&place_order(b)));
        let (primary, mut origin) = places.pop()?;
        for (place, earlier) in places {
            origin.related.push(place);
            origin.related.extend(earlier.related);
        }
        origin
            .related
            .sort_by(|a, b| place_order(a).cmp(&place_order(b)));
        Some((primary, origin))
    }
}

fn empty_origin() -> DiagnosticOrigin {
    DiagnosticOrigin {
        translation: None,
        related: vec![],
    }
}

fn place_order(place: &AuthoredLocation) -> (&str, usize, u32, u32) {
    (
        &place.file,
        place.document,
        place.position.line,
        place.position.column,
    )
}

fn strip_field<'a>(pointer: &'a str, field: &str) -> Option<&'a str> {
    pointer
        .strip_prefix(field)
        .filter(|suffix| suffix.is_empty() || suffix.starts_with('/'))
}

fn write(diagnostic: &mut Value, primary: &AuthoredLocation, origin: &DiagnosticOrigin) {
    if diagnostic["locations"].as_array().is_none_or(Vec::is_empty) {
        diagnostic["locations"] = json!([primary.result()]);
    } else {
        diagnostic["locations"][0] = primary.result();
    }
    diagnostic["properties"]["file"] = json!(primary.file);
    diagnostic["properties"]["document"] = json!(primary.document);
    add_origin(diagnostic, origin);
}
