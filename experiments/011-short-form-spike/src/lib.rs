//! Experimental expansion only. Canonical validation remains the loader/core's job.
use serde_json::{Map, Value, json};
use std::collections::BTreeSet;

pub mod evidence;

pub type Result<T> = std::result::Result<T, String>;
const FAMILIES: [(&str, &str); 5] = [
    ("interfaces", "Interface"),
    ("agents", "Agent"),
    ("resources", "Resource"),
    ("pools", "Pool"),
    ("conduits", "Conduit"),
];

fn object(value: &Value) -> Result<&Map<String, Value>> {
    value.as_object().ok_or_else(|| "expected a map".into())
}

fn fields(value: &Value, allowed: &[&str]) -> Result<()> {
    if let Some(key) = object(value)?
        .keys()
        .find(|k| !allowed.contains(&k.as_str()))
    {
        return Err(format!("unexpected field: {key}"));
    }
    Ok(())
}

fn name(value: &str, generated: bool) -> Result<()> {
    if value.is_empty()
        || value.len() > 63
        || !value.as_bytes()[0].is_ascii_lowercase()
        || !value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        || (!generated
            && (value.contains("--")
                || ["system", "local", "management", "access", "root"].contains(&value)))
    {
        return Err(format!(
            "invalid {}name: {value}",
            if generated { "generated " } else { "" }
        ));
    }
    Ok(())
}

fn reference(value: &Value) -> Result<Value> {
    fields(value, &["kind", "name", "entry"])?;
    if value["kind"].as_str().is_none() || value["name"].as_str().is_none() {
        return Err("reference must name kind and name".into());
    }
    Ok(value.clone())
}

fn pick(value: &Value, keys: &[&str]) -> Value {
    Value::Object(
        keys.iter()
            .filter_map(|key| value.get(*key).map(|v| ((*key).into(), v.clone())))
            .collect(),
    )
}

struct Translator<'a> {
    version: &'a str,
    core: &'a str,
    documents: Vec<Value>,
    names: BTreeSet<String>,
}

impl Translator<'_> {
    fn emit(&mut self, family: &str, n: &str, body: Value, metadata: &Value) -> Result<()> {
        name(n, true)?;
        if !self.names.insert(n.into()) {
            return Err(format!("duplicate translated name: {n}"));
        }
        let mut doc = json!({"apiVersion": self.version, "kind": family, "metadata": {"name": n}});
        for key in ["description", "annotations"] {
            if let Some(v) = metadata.get(key) {
                doc["metadata"][key] = v.clone();
            }
        }
        doc.as_object_mut().unwrap().extend(object(&body)?.clone());
        self.documents.push(doc);
        Ok(())
    }

    fn conduit(&mut self, n: &str, value: &Value) -> Result<()> {
        fields(
            value,
            &[
                "from",
                "arguments",
                "capability",
                "operations",
                "requirements",
                "consumer",
                "target",
                "as",
                "description",
                "annotations",
            ],
        )?;
        let mut instance = pick(value, &["from", "arguments", "as"]);
        instance["consumer"] = reference(&value["consumer"])?;
        instance["target"] = reference(&value["target"])?;
        if value.get("from").is_some() {
            if ["capability", "operations", "requirements"]
                .iter()
                .any(|k| value.get(k).is_some())
            {
                return Err(format!("{n}: from cannot accompany an inline Conduit"));
            }
        } else {
            if value.get("arguments").is_some() {
                return Err(format!("{n}: arguments require from"));
            }
            if value.get("capability").is_none() || value.get("operations").is_none() {
                return Err(format!(
                    "{n}: inline Conduit requires capability and operations"
                ));
            }
            let template = format!("{n}--t");
            self.emit(
                "Conduit",
                &template,
                json!({"spec": pick(value, &["capability", "operations", "requirements"])}),
                &Value::Null,
            )?;
            instance["from"] = json!(template);
        }
        self.emit("Conduit", n, instance, value)
    }

    fn grant(&mut self, n: &str, local: &str, family: &str, value: &Value) -> Result<()> {
        // LOCAL uses OBJ-022, with the short form's additional reserved words.
        if local.is_empty()
            || local.len() > 63
            || local.contains("--")
            || ["t", "operator"].contains(&local)
            || !local.as_bytes()[0].is_ascii_lowercase()
            || !local
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Err(format!("invalid local name: {local}"));
        }
        fields(
            value,
            &[
                "to",
                "from",
                "arguments",
                "capability",
                "operations",
                "requirements",
                "description",
            ],
        )?;
        let target = reference(&value["to"])?;
        if target["kind"] == "Resource" && target["name"] == "management" {
            return Err("consumes cannot target management; use conduits".into());
        }
        if target["kind"] == "Resource" && target["name"] == "access" {
            let bound = format!("{}#bound.v1", self.core);
            if !value["requirements"]
                .as_object()
                .is_some_and(|r| r.values().any(|r| r["requirement"] == bound))
            {
                return Err("consumes to access requires an explicit bound; use conduits".into());
            }
        }
        let mut conduit = pick(
            value,
            &[
                "from",
                "arguments",
                "capability",
                "operations",
                "requirements",
                "description",
            ],
        );
        conduit["consumer"] = json!({"kind": family, "name": n});
        conduit["target"] = target;
        conduit["as"] = json!(local);
        self.conduit(&format!("{n}--{local}"), &conduit)
    }

    fn definition(&mut self, family: &str, n: &str, value: &Value) -> Result<()> {
        fields(
            value,
            &[
                "type",
                "config",
                "implementation",
                "controls",
                "conduits",
                "entries",
                "from",
                "arguments",
                "owner",
                "placement",
                "runtime",
                "description",
                "annotations",
                "consumes",
                "operator",
            ],
        )?;
        if family == "Resource" && value.get("owner").is_none() {
            return Err(format!("{n}: Resource requires owner"));
        }
        if (!["Resource", "Agent"].contains(&family) && value.get("owner").is_some())
            || (!["Resource", "Agent"].contains(&family)
                && ["placement", "runtime"]
                    .iter()
                    .any(|k| value.get(k).is_some()))
            || (family != "Pool" && value.get("entries").is_some())
            || (family == "Pool"
                && ["conduits", "consumes"]
                    .iter()
                    .any(|k| value.get(k).is_some()))
        {
            return Err(format!("{n}: field not allowed on {family}"));
        }
        let mut instance = pick(
            value,
            &["from", "arguments", "owner", "placement", "runtime"],
        );
        let inline = [
            "type",
            "config",
            "implementation",
            "controls",
            "conduits",
            "entries",
        ];
        if value.get("from").is_some() {
            if inline.iter().any(|k| value.get(k).is_some()) {
                return Err(format!("{n}: from cannot accompany inline fields"));
            }
        } else {
            if value.get("arguments").is_some() || value.get("type").is_none() {
                return Err(format!(
                    "{n}: inline definition requires type, forbids arguments"
                ));
            }
            let template = format!("{n}--t");
            self.emit(
                family,
                &template,
                json!({"spec": pick(value, &inline)}),
                &Value::Null,
            )?;
            instance["from"] = json!(template);
        }
        self.emit(family, n, instance, value)?;
        if let Some(consumes) = value.get("consumes") {
            for (local, grant) in object(consumes)? {
                self.grant(n, local, family, grant)?;
            }
        }
        // FLEET-005 supplies root's Pool/Fleet defaults within registration.
        // Only an explicit operator still expands to an authored Conduit here.
        if let Some(operator) = value.get("operator") {
            if operator.is_null() && family == "Pool" {
                return Ok(());
            }
            fields(
                operator,
                &["capability", "operations", "requirements", "description"],
            )?;
            let mut conduit = operator.clone();
            conduit["consumer"] = json!({"kind": "Interface", "name": "root"});
            conduit["target"] = json!({"kind": family, "name": n});
            conduit["as"] = json!(n);
            self.conduit(&format!("{n}--operator"), &conduit)?;
        }
        Ok(())
    }
}

/// Pure expansion. The caller supplies the manifest's version and the assigned core address.
pub fn translate(value: &Value, manifest_version: &str, core: &str) -> Result<Vec<Value>> {
    fields(
        value,
        &[
            "apiVersion",
            "fleet",
            "templates",
            "interfaces",
            "agents",
            "resources",
            "pools",
            "conduits",
        ],
    )?;
    let version = value.get("apiVersion").map_or(Ok(manifest_version), |v| {
        v.as_str().ok_or("apiVersion must be a string")
    })?;
    let mut t = Translator {
        version,
        core,
        documents: vec![],
        names: BTreeSet::new(),
    };
    let mut authored = BTreeSet::new();
    if let Some(templates) = value.get("templates") {
        fields(templates, &FAMILIES.map(|(section, _)| section))?;
        for (section, family) in FAMILIES {
            if let Some(definitions) = templates.get(section) {
                for (n, body) in object(definitions)? {
                    name(n, false)?;
                    if !authored.insert(n.clone()) {
                        return Err(format!("duplicate authored name: {n}"));
                    }
                    let mut spec = object(body)?.clone();
                    spec.remove("description");
                    spec.remove("annotations");
                    t.emit(family, n, json!({"spec": spec}), body)?;
                }
            }
        }
    }
    for (section, family) in FAMILIES {
        if let Some(definitions) = value.get(section) {
            for (n, body) in object(definitions)? {
                name(n, false)?;
                if !authored.insert(n.clone()) {
                    return Err(format!("duplicate authored name: {n}"));
                }
                if family == "Conduit" {
                    t.conduit(n, body)?;
                } else {
                    let mut body = body.clone();
                    if family == "Agent" {
                        // FLEET-002 / OBJ-013 / SCT-011: a file's Fleet supplies
                        // ownership without overriding a contradictory authored owner.
                        if let Some(fleet) = value.get("fleet") {
                            if let Some(owner) = body.get("owner")
                                && owner != fleet
                            {
                                return Err(format!(
                                    "{n}: owner {owner} differs from file fleet {fleet}"
                                ));
                            }
                            body["owner"] = fleet.clone();
                        } else if body.get("owner").is_none() {
                            return Err(format!("{n}: Agent requires owner or fleet"));
                        }
                    }
                    t.definition(family, n, &body)?;
                }
            }
        }
    }
    if let Some(fleet) = value.get("fleet") {
        let fleet = fleet.as_str().ok_or("fleet must be a name")?;
        t.emit(
            "Fleet",
            fleet,
            json!({"from":format!("{core}#fleet")}),
            &json!({}),
        )?;
    }
    if t.documents.is_empty() {
        return Err("expected at least one section definition".into());
    }
    Ok(t.documents)
}
