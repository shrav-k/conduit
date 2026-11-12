//! Pure transformations of the coverage index and case-schema file contents.
use super::{Result, Value};
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

pub type Files = BTreeMap<String, Vec<u8>>;
pub type Changes = BTreeMap<String, Option<Vec<u8>>>;

/// Objects retain their authored order without changing serde_json's workspace features.
#[derive(Clone, Debug, PartialEq)]
pub enum Ordered {
    Object(Vec<(String, Ordered)>),
    Array(Vec<Ordered>),
    Scalar(Value),
}
impl<'de> Deserialize<'de> for Ordered {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct Read;
        impl<'de> Visitor<'de> for Read {
            type Value = Ordered;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a JSON value with unique object keys")
            }
            fn visit_map<M: MapAccess<'de>>(
                self,
                mut map: M,
            ) -> std::result::Result<Ordered, M::Error> {
                let mut entries = vec![];
                while let Some((key, value)) = map.next_entry::<String, Ordered>()? {
                    if entries.iter().any(|(k, _)| k == &key) {
                        return Err(serde::de::Error::custom(format!("duplicate key {key}")));
                    }
                    entries.push((key, value));
                }
                Ok(Ordered::Object(entries))
            }
            fn visit_seq<S: SeqAccess<'de>>(
                self,
                mut seq: S,
            ) -> std::result::Result<Ordered, S::Error> {
                let mut entries = vec![];
                while let Some(value) = seq.next_element()? {
                    entries.push(value);
                }
                Ok(Ordered::Array(entries))
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Scalar(Value::Bool(v)))
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Scalar(Value::from(v)))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Scalar(Value::from(v)))
            }
            fn visit_f64<E: serde::de::Error>(self, v: f64) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Scalar(Value::from(v)))
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Scalar(Value::from(v)))
            }
            fn visit_string<E: serde::de::Error>(
                self,
                v: String,
            ) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Scalar(Value::from(v)))
            }
            fn visit_unit<E: serde::de::Error>(self) -> std::result::Result<Ordered, E> {
                Ok(Ordered::Scalar(Value::Null))
            }
        }
        deserializer.deserialize_any(Read)
    }
}
impl Serialize for Ordered {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::Object(entries) => {
                let mut map = serializer.serialize_map(Some(entries.len()))?;
                for (key, value) in entries {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
            Self::Array(entries) => {
                let mut seq = serializer.serialize_seq(Some(entries.len()))?;
                for value in entries {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            Self::Scalar(value) => value.serialize(serializer),
        }
    }
}
impl Ordered {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(|e| e.to_string())
    }
    pub fn pretty(&self) -> Result<Vec<u8>> {
        let mut bytes = serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?;
        bytes.push(b'\n');
        Ok(bytes)
    }
    pub fn value(&self) -> Value {
        serde_json::to_value(self).expect("ordered JSON is serializable")
    }
    fn object(&self) -> Result<&Vec<(String, Self)>> {
        match self {
            Self::Object(entries) => Ok(entries),
            _ => Err("expected an object".into()),
        }
    }
    fn array(&self) -> Result<&Vec<Self>> {
        match self {
            Self::Array(entries) => Ok(entries),
            _ => Err("expected a list".into()),
        }
    }
    fn at(&self, path: &[&str]) -> Result<&Self> {
        match path.split_first() {
            None => Ok(self),
            Some((key, rest)) => self
                .object()?
                .iter()
                .find(|(k, _)| k == key)
                .ok_or_else(|| format!("missing {key}"))?
                .1
                .at(rest),
        }
    }
    fn at_mut(&mut self, path: &[&str]) -> Result<&mut Self> {
        match path.split_first() {
            None => Ok(self),
            Some((key, rest)) => match self {
                Self::Object(entries) => entries
                    .iter_mut()
                    .find(|(k, _)| k == key)
                    .ok_or_else(|| format!("missing {key}"))?
                    .1
                    .at_mut(rest),
                _ => Err("expected an object".into()),
            },
        }
    }
    fn remove(&mut self, key: &str) -> Result<Option<Self>> {
        let Self::Object(entries) = self else {
            return Err("expected an object".into());
        };
        Ok(entries
            .iter()
            .position(|(k, _)| k == key)
            .map(|i| entries.remove(i).1))
    }
}
const INPUTS: &[&str] = &[
    "properties",
    "steps",
    "items",
    "properties",
    "input",
    "oneOf",
];
const EXPECT: &[&str] = &[
    "properties",
    "steps",
    "items",
    "properties",
    "expect",
    "properties",
];
const MANIFEST: &str = "manifest.yaml";
const SCHEMA: &str = "case.schema.json";

fn read(files: &Files, name: &str) -> Result<Ordered> {
    Ordered::parse(
        files
            .get(name)
            .ok_or_else(|| format!("{name}: missing file"))?,
    )
    .map_err(|e| format!("{name}: {e}"))
}

/// Common repair instructions for metadata left inline by new work or a rebase.
pub fn repair(file: &str, id: &str, problem: &str) -> String {
    format!(
        "conformance/{file}: {id}{problem}. See conformance/README.md, \"The coverage index and the case schema in pieces\". New inline elements can be moved from engine/ with cargo run --release -p conduit-conformance --example pieces; a branch made before the split and rebased after it needs -- --merge ROOT."
    )
}
fn entry_id(entry: &Ordered) -> Result<String> {
    let members = entry.object()?;
    for (key, _) in members {
        if !["id", "case", "promises", "safety_rows"].contains(&key.as_str()) {
            return Err(format!(
                "entry has unknown member {key}; remove it rather than dropping information"
            ));
        }
    }
    let value = entry.value();
    for key in ["id", "case", "promises"] {
        if value.get(key).is_none() {
            return Err(format!("entry needs {key}"));
        }
    }
    value["id"]
        .as_str()
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or("entry needs a nonempty ID".into())
}
/// Path relative to the conformance directory of the piece owning a case.
pub fn index_for(case: &str) -> Result<String> {
    let path = Path::new(case);
    if path
        .components()
        .any(|c| !matches!(c, Component::Normal(_)))
        || path.file_name().is_none_or(|n| n != "case.yaml")
    {
        return Err(format!(
            "invalid case path {case}; use a relative path ending in case.yaml"
        ));
    }
    let parent = path
        .parent()
        .and_then(Path::parent)
        .ok_or("case path needs a case directory")?;
    Ok(parent.join("index.yaml").to_string_lossy().into_owned())
}
fn entry_index(entry: &Ordered) -> Result<String> {
    index_for(
        entry.value()["case"]
            .as_str()
            .ok_or("entry case must be a path string")?,
    )
}

/// Validated entries paired with their piece, in piece and ID order.
pub fn entries(files: &Files) -> Result<Vec<(String, Value)>> {
    let mut result = vec![];
    let mut ids = BTreeMap::new();
    for (name, bytes) in files.iter().filter(|(name, _)| {
        Path::new(name)
            .file_name()
            .is_some_and(|n| n == "index.yaml")
    }) {
        let piece = Ordered::parse(bytes)
            .map_err(|e| repair(name, "", &format!("{e}; supply a JSON object")))?;
        let members = piece.object().map_err(|e| repair(name, "", &e))?;
        if members.len() != 1 || members[0].0 != "cases" {
            return Err(repair(
                name,
                "",
                "a piece must contain only cases; remove other members",
            ));
        }
        let list = members[0]
            .1
            .array()
            .map_err(|e| repair(name, "", &format!("{e}; cases must be a nonempty list")))?;
        if list.is_empty() {
            return Err(repair(name, "", "cases is empty; delete the empty piece"));
        }
        let mut previous = None;
        for entry in list {
            let label = entry.value()["id"]
                .as_str()
                .map(|id| format!("{id}: "))
                .unwrap_or_default();
            let id = entry_id(entry).map_err(|e| repair(name, &label, &e))?;
            let label = format!("{id}: ");
            let expected = entry_index(entry).map_err(|e| repair(name, &label, &e))?;
            if &expected != name {
                return Err(repair(
                    name,
                    &label,
                    &format!("entry belongs in {expected}; move it there"),
                ));
            }
            if let Some(other) = ids.insert(id.clone(), name.clone()) {
                return Err(repair(
                    name,
                    &label,
                    &format!("duplicate ID also in {other}; keep one entry"),
                ));
            }
            if previous.as_ref().is_some_and(|p| p >= &id) {
                return Err(repair(
                    name,
                    &label,
                    "entries must be sorted by ID in byte order; sort them",
                ));
            }
            previous = Some(id);
            result.push((name.clone(), entry.value()));
        }
    }
    Ok(result)
}
fn inline_entries(manifest: &Ordered) -> Result<Option<BTreeMap<String, Ordered>>> {
    let Some((_, cases)) = manifest.object()?.iter().find(|(k, _)| k == "cases") else {
        return Ok(None);
    };
    let mut result = BTreeMap::new();
    for entry in cases.array()? {
        let label = entry.value()["id"]
            .as_str()
            .map(|id| format!("{id}: "))
            .unwrap_or_default();
        let id = entry_id(entry).map_err(|e| format!("{MANIFEST}: {label}{e}"))?;
        entry_index(entry).map_err(|e| format!("{MANIFEST}: {id}: {e}"))?;
        if result.insert(id.clone(), entry.clone()).is_some() {
            return Err(format!("{MANIFEST}: duplicate ID {id}"));
        }
    }
    Ok(Some(result))
}
fn input_name(input: &Ordered) -> Result<String> {
    let value = input.value();
    let kind = &value["properties"]["kind"];
    let name = kind["const"]
        .as_str()
        .or_else(|| kind["enum"][0].as_str())
        .ok_or("input branch needs a kind const or enum")?;
    safe_name(name)?;
    Ok(name.into())
}
fn safe_name(name: &str) -> Result<()> {
    if name.is_empty()
        || !name
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    {
        return Err(format!("invalid schema piece name {name}"));
    }
    Ok(())
}
fn inline_schema(schema: &Ordered) -> Result<BTreeMap<String, Ordered>> {
    let mut result = BTreeMap::new();
    for branch in schema.at(INPUTS)?.array()? {
        if branch.value() == Value::Bool(false) {
            continue;
        }
        let name = format!("case-schema/inputs/{}.json", input_name(branch)?);
        if result.insert(name.clone(), branch.clone()).is_some() {
            return Err(format!("{SCHEMA}: duplicate input file {name}"));
        }
    }
    for (name, property) in schema.at(EXPECT)?.object()? {
        safe_name(name)?;
        result.insert(format!("case-schema/expect/{name}.json"), property.clone());
    }
    Ok(result)
}
fn schema_file(name: &str, directory: &str) -> bool {
    Path::new(name).parent() == Some(Path::new(directory))
        && Path::new(name).extension().is_some_and(|e| e == "json")
}

/// Assemble input and expectation pieces, then append external rules in filename order.
pub fn assemble(files: &Files) -> Result<Value> {
    let schema = read(files, SCHEMA)?;
    if !inline_schema(&schema)?.is_empty() {
        return Err(repair(
            SCHEMA,
            "",
            "inline input branches or expectation properties remain; move them to their pieces",
        ));
    }
    if schema.at(INPUTS)?.array()?.as_slice() != [Ordered::Scalar(Value::Bool(false))] {
        return Err(repair(
            SCHEMA,
            "",
            "input oneOf needs the single false placeholder; run the pieces tool",
        ));
    }
    let mut value = schema.value();
    let mut inputs = vec![];
    let mut expect = serde_json::Map::new();
    for (name, bytes) in files {
        if schema_file(name, "case-schema/inputs") {
            let branch = Ordered::parse(bytes).map_err(|e| format!("{name}: {e}"))?;
            let expected = format!("{}.json", input_name(&branch)?);
            if Path::new(name)
                .file_name()
                .is_none_or(|n| n != expected.as_str())
            {
                return Err(format!("{name}: name the input piece {expected}"));
            }
            inputs.push(branch.value());
        } else if schema_file(name, "case-schema/expect") {
            let key = Path::new(name)
                .file_stem()
                .unwrap()
                .to_str()
                .ok_or("non-UTF8 expectation filename")?;
            expect.insert(
                key.into(),
                Ordered::parse(bytes)
                    .map_err(|e| format!("{name}: {e}"))?
                    .value(),
            );
        }
    }
    if inputs.is_empty() {
        return Err(repair(
            SCHEMA,
            "",
            "no input pieces found; supply the files under case-schema/inputs",
        ));
    }
    *value
        .pointer_mut("/properties/steps/items/properties/input/oneOf")
        .ok_or("missing input placeholder")? = Value::Array(inputs);
    *value
        .pointer_mut("/properties/steps/items/properties/expect/properties")
        .ok_or("missing expectation placeholder")? = Value::Object(expect);
    let rules = value
        .pointer_mut("/properties/steps/items/allOf")
        .and_then(Value::as_array_mut)
        .ok_or("case.schema.json: allOf must be a list")?;
    for (name, bytes) in files {
        if schema_file(name, "case-schema/rules") {
            rules.push(
                Ordered::parse(bytes)
                    .map_err(|e| format!("{name}: {e}"))?
                    .value(),
            );
        }
    }
    Ok(value)
}
fn canonical(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).expect("parsed JSON has canonical bytes")
}
fn same(a: &Ordered, b: &Ordered) -> bool {
    canonical(&a.value()) == canonical(&b.value())
}

fn reconcile(
    name: &str,
    incoming: &Ordered,
    base: Option<&Ordered>,
    held: Option<&Ordered>,
    merging: bool,
) -> Result<bool> {
    if base.is_some_and(|b| same(b, incoming)) {
        return Ok(false);
    }
    if held.is_some_and(|h| same(h, incoming)) {
        return Ok(false);
    }
    if held.is_none() && base.is_none()
        || merging && held.zip(base).is_some_and(|(h, b)| same(h, b))
    {
        return Ok(true);
    }
    Err(if merging {
        format!("{name}: changed on both sides; resolve the source changes before --merge")
    } else {
        format!(
            "{name}: differs from its piece; use --merge ROOT for a branch made before the split"
        )
    })
}

/// Plan the entire split or three-way merge before any caller writes a file.
/// Existing case paths are relative to the conformance directory.
pub fn split(
    files: &Files,
    base: Option<&Files>,
    existing_cases: &BTreeSet<String>,
) -> Result<Changes> {
    let mut output = files.clone();
    let mut manifest = read(files, MANIFEST)?;
    let mut schema = read(files, SCHEMA)?;
    let baseline_manifest = base.map(|b| read(b, MANIFEST)).transpose()?;
    let baseline_entries = baseline_manifest
        .as_ref()
        .map(|m| {
            inline_entries(m).and_then(|e| {
                e.ok_or(
                    "merge ROOT manifest.yaml has no cases; use a tree from before the split"
                        .into(),
                )
            })
        })
        .transpose()?;
    let baseline_schema = base.map(|b| read(b, SCHEMA)).transpose()?;
    let baseline_elements = baseline_schema.as_ref().map(inline_schema).transpose()?;
    let mut held = BTreeMap::new();
    // Validate existing pieces before planning any changes.
    entries(files)?;
    for (name, bytes) in files {
        if Path::new(name)
            .file_name()
            .is_some_and(|n| n == "index.yaml")
        {
            for entry in Ordered::parse(bytes)?.at(&["cases"])?.array()? {
                held.insert(entry_id(entry)?, entry.clone());
            }
        }
    }
    if let Some(incoming) = inline_entries(&manifest)? {
        for (id, entry) in &incoming {
            let old = baseline_entries.as_ref().and_then(|b| b.get(id));
            if reconcile(id, entry, old, held.get(id), base.is_some())? {
                held.insert(id.clone(), entry.clone());
            }
        }
        if let Some(baseline) = &baseline_entries {
            for (id, old) in baseline
                .iter()
                .filter(|(id, _)| !incoming.contains_key(*id))
            {
                let case = old.value()["case"]
                    .as_str()
                    .ok_or("baseline entry needs case")?
                    .to_string();
                if existing_cases.contains(&case) {
                    return Err(format!(
                        "{id}: removed inline but case {case} still exists; keep its entry"
                    ));
                }
                if !held.get(id).is_some_and(|h| same(h, old)) {
                    return Err(format!(
                        "{id}: removed entry changed on both sides; cannot --merge"
                    ));
                }
                held.remove(id);
            }
        }
        manifest.remove("cases")?;
        output.insert(MANIFEST.into(), manifest.pretty()?);
        let mut grouped: BTreeMap<String, Vec<Ordered>> = BTreeMap::new();
        for entry in held.values() {
            grouped
                .entry(entry_index(entry)?)
                .or_default()
                .push(entry.clone());
        }
        output.retain(|name, _| {
            Path::new(name)
                .file_name()
                .is_none_or(|n| n != "index.yaml")
        });
        for (name, list) in grouped {
            output.insert(
                name,
                Ordered::Object(vec![("cases".into(), Ordered::Array(list))]).pretty()?,
            );
        }
    }
    let incoming = inline_schema(&schema)?;
    if incoming.is_empty()
        && schema.at(INPUTS)?.array()?.as_slice() != [Ordered::Scalar(Value::Bool(false))]
    {
        return Err(format!(
            "{SCHEMA}: schema elements removed inline; retain the placeholder and retire elements by deleting their pieces, not --merge"
        ));
    }
    if !incoming.is_empty() {
        if let Some(baseline) = &baseline_elements {
            for name in baseline.keys() {
                if !incoming.contains_key(name) {
                    return Err(format!(
                        "{name}: schema element removed inline; retire it by deleting its piece, not --merge"
                    ));
                }
            }
        }
        for (name, element) in incoming {
            let held = files.get(&name).map(|b| Ordered::parse(b)).transpose()?;
            let old = baseline_elements.as_ref().and_then(|b| b.get(&name));
            if reconcile(&name, &element, old, held.as_ref(), base.is_some())? {
                output.insert(name, element.pretty()?);
            }
        }
        *schema.at_mut(INPUTS)? = Ordered::Array(vec![Ordered::Scalar(Value::Bool(false))]);
        *schema.at_mut(EXPECT)? = Ordered::Object(vec![]);
        output.insert(SCHEMA.into(), schema.pretty()?);
    }
    let mut changes = BTreeMap::new();
    for name in files.keys().chain(output.keys()).collect::<BTreeSet<_>>() {
        if files.get(name) != output.get(name) {
            changes.insert(name.clone(), output.get(name).cloned());
        }
    }
    Ok(changes)
}
fn normalized_schema(mut schema: Value) -> Result<Value> {
    let branches = schema
        .pointer_mut("/properties/steps/items/properties/input/oneOf")
        .and_then(Value::as_array_mut)
        .ok_or("schema has no input branches")?;
    branches.sort_by_key(|b| serde_json::to_vec(b).expect("JSON bytes"));
    Ok(schema)
}

/// Prove that entries, coverage metadata and the assembled schema equal the unsplit tree.
pub fn compare(files: &Files, base: &Files) -> Result<String> {
    let mut original = read(base, MANIFEST)?;
    let expected =
        inline_entries(&original)?.ok_or("comparison ROOT manifest.yaml has no cases")?;
    original.remove("cases")?;
    let current = read(files, MANIFEST)?;
    if inline_entries(&current)?.is_some() {
        return Err(repair(
            MANIFEST,
            "",
            "still has cases; move them into index.yaml pieces",
        ));
    }
    if !same(&current, &original) {
        return Err(
            "manifest.yaml: status, s1_coverage or s1_coverage_source differs from ROOT".into(),
        );
    }
    let entries = entries(files)?;
    let actual: BTreeMap<_, _> = entries
        .iter()
        .map(|(_, e)| (e["id"].as_str().unwrap().to_string(), e))
        .collect();
    for (id, entry) in &expected {
        if actual
            .get(id)
            .is_none_or(|e| canonical(e) != canonical(&entry.value()))
        {
            return Err(format!("{id}: missing or changed coverage-index entry"));
        }
    }
    if actual.len() != expected.len() {
        return Err("coverage-index pieces hold extra entries".into());
    }
    let assembled = assemble(files)?;
    if normalized_schema(assembled.clone())? != normalized_schema(read(base, SCHEMA)?.value())? {
        return Err("case.schema.json: assembled schema differs from ROOT".into());
    }
    let pieces = entries
        .iter()
        .map(|(name, _)| name)
        .collect::<BTreeSet<_>>()
        .len();
    let tagged = entries
        .iter()
        .filter(|(_, e)| e["safety_rows"].as_array().is_some_and(|a| !a.is_empty()))
        .count();
    let empty = entries
        .iter()
        .filter(|(_, e)| e["safety_rows"].as_array().is_some_and(Vec::is_empty))
        .count();
    let absent = entries
        .iter()
        .filter(|(_, e)| e.get("safety_rows").is_none())
        .count();
    Ok(format!(
        "{pieces} pieces; {} entries ({tagged} with tags, {empty} empty, {absent} without safety_rows); {} s1_coverage rows; {} input branches; {} expectation properties.",
        entries.len(),
        current.value()["s1_coverage"]
            .as_array()
            .ok_or("missing s1_coverage")?
            .len(),
        assembled
            .pointer("/properties/steps/items/properties/input/oneOf")
            .unwrap()
            .as_array()
            .unwrap()
            .len(),
        assembled
            .pointer("/properties/steps/items/properties/expect/properties")
            .unwrap()
            .as_object()
            .unwrap()
            .len()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn ordered(value: Value) -> Ordered {
        Ordered::parse(&serde_json::to_vec(&value).unwrap()).unwrap()
    }
    fn put(files: &mut Files, name: &str, value: Value) {
        files.insert(name.into(), ordered(value).pretty().unwrap());
    }
    fn apply(files: &Files, changes: Changes) -> Files {
        let mut result = files.clone();
        for (name, bytes) in changes {
            if let Some(bytes) = bytes {
                result.insert(name, bytes);
            } else {
                result.remove(&name);
            }
        }
        result
    }
    fn entry(id: &str, directory: &str) -> Value {
        json!({"id":id, "case":format!("{directory}/{id}/case.yaml"), "promises":["A-001"]})
    }
    fn fixture() -> Files {
        let manifest = br#"{
  "status": "unchanged",
  "cases": [
    {
      "id": "a",
      "case": "language/group/a/case.yaml",
      "promises": ["A-001"]
    },
    {
      "id": "b",
      "case": "language/other/b/case.yaml",
      "promises": ["A-001"],
      "safety_rows": []
    }
  ],
  "s1_coverage": [
    {"promise": "A-001", "vectors": ["a"]}
  ],
  "s1_coverage_source": "frozen"
}"#;
        let mut files = Files::new();
        files.insert(
            MANIFEST.into(),
            Ordered::parse(manifest).unwrap().pretty().unwrap(),
        );
        put(
            &mut files,
            SCHEMA,
            json!({
                "$schema":"https://json-schema.org/draft/2020-12/schema",
                "properties":{"steps":{"items":{
                    "properties":{
                        "input":{"oneOf":[{"properties":{"kind":{"const":"one"}}}]},
                        "expect":{"properties":{"reply":{"type":"object"}}},
                        "capture":{"type":"object"}
                    },
                    "allOf":[{"required":["input"]}],
                    "required":["input","expect"]
                }}},
                "required":["steps"]
            }),
        );
        files
    }
    fn split_fixture() -> Files {
        let files = fixture();
        apply(&files, split(&files, None, &BTreeSet::new()).unwrap())
    }
    fn with_inline(files: &Files, entries: Value) -> Files {
        let mut result = files.clone();
        let mut manifest = read(&fixture(), MANIFEST).unwrap().value();
        manifest["cases"] = entries;
        put(&mut result, MANIFEST, manifest);
        result
    }
    fn assert_unchanged_error(
        files: &Files,
        base: Option<&Files>,
        cases: &BTreeSet<String>,
        word: &str,
    ) {
        let before = files.clone();
        let error = split(files, base, cases).unwrap_err();
        assert!(error.contains(word), "{error}");
        assert_eq!(*files, before);
    }

    #[test]
    fn split_preserves_members_bytes_and_schema_outside_placeholders() {
        let original = fixture();
        let result = split_fixture();
        let mut manifest = read(&original, MANIFEST).unwrap();
        manifest.remove("cases").unwrap();
        assert_eq!(result[MANIFEST], manifest.pretty().unwrap());
        let source = String::from_utf8(original[MANIFEST].clone()).unwrap();
        let start = source.find("  \"cases\": [").unwrap();
        let end = source.find("  \"s1_coverage\":").unwrap();
        assert_eq!(
            String::from_utf8(result[MANIFEST].clone()).unwrap(),
            format!("{}{}", &source[..start], &source[end..])
        );
        let mut schema = read(&original, SCHEMA).unwrap();
        *schema.at_mut(INPUTS).unwrap() = Ordered::Array(vec![Ordered::Scalar(json!(false))]);
        *schema.at_mut(EXPECT).unwrap() = Ordered::Object(vec![]);
        assert_eq!(result[SCHEMA], schema.pretty().unwrap());
        assert!(
            jsonschema::options()
                .build(&read(&result, SCHEMA).unwrap().value())
                .is_ok()
        );
        assert_eq!(entries(&result).unwrap().len(), 2);
        let a = read(&result, "language/group/index.yaml").unwrap().value();
        assert!(a["cases"][0].get("safety_rows").is_none());
        let b = read(&result, "language/other/index.yaml").unwrap().value();
        assert_eq!(b["cases"][0]["safety_rows"], json!([]));
        compare(&result, &original).unwrap();
    }

    #[test]
    fn second_split_is_a_noop() {
        let files = split_fixture();
        assert!(split(&files, None, &BTreeSet::new()).unwrap().is_empty());
    }

    #[test]
    fn additions_join_in_id_order_and_create_a_piece() {
        let files = split_fixture();
        let incoming = with_inline(
            &files,
            json!([
                entry("z", "language/group"),
                entry("c", "language/new"),
                entry("aa", "language/group")
            ]),
        );
        let result = apply(&incoming, split(&incoming, None, &BTreeSet::new()).unwrap());
        let group = read(&result, "language/group/index.yaml").unwrap().value();
        assert_eq!(
            group["cases"]
                .as_array()
                .unwrap()
                .iter()
                .map(|e| e["id"].as_str().unwrap())
                .collect::<Vec<_>>(),
            ["a", "aa", "z"]
        );
        assert!(result.contains_key("language/new/index.yaml"));
    }

    #[test]
    fn identical_inline_elements_drop_and_conflicts_need_merge() {
        let files = split_fixture();
        let mut incoming = files.clone();
        incoming.insert(MANIFEST.into(), fixture()[MANIFEST].clone());
        incoming.insert(SCHEMA.into(), fixture()[SCHEMA].clone());
        assert_eq!(
            apply(&incoming, split(&incoming, None, &BTreeSet::new()).unwrap()),
            files
        );
        let mut changed = entry("a", "language/group");
        changed["promises"] = json!(["CHANGED"]);
        let incoming = with_inline(&files, json!([changed]));
        assert_unchanged_error(&incoming, None, &BTreeSet::new(), "--merge");
        for path in [
            "case-schema/inputs/one.json",
            "case-schema/expect/reply.json",
        ] {
            let mut incoming = files.clone();
            incoming.insert(SCHEMA.into(), fixture()[SCHEMA].clone());
            put(&mut incoming, path, json!({"type":"null"}));
            let error = split(&incoming, None, &BTreeSet::new()).unwrap_err();
            assert!(error.contains(path) && error.contains("--merge"), "{error}");
        }
    }

    #[test]
    fn merge_adds_changes_removes_and_deletes_empty_piece() {
        let original = fixture();
        let files = split_fixture();
        let mut changed = entry("a", "language/group");
        changed["promises"] = json!(["CHANGED"]);
        let incoming = with_inline(&files, json!([changed, entry("c", "language/new")]));
        let cases = BTreeSet::from([
            "language/group/a/case.yaml".into(),
            "language/new/c/case.yaml".into(),
        ]);
        let result = apply(
            &incoming,
            split(&incoming, Some(&original), &cases).unwrap(),
        );
        assert!(!result.contains_key("language/other/index.yaml"));
        assert!(result.contains_key("language/new/index.yaml"));
        assert_eq!(
            read(&result, "language/group/index.yaml").unwrap().value()["cases"][0]["promises"],
            json!(["CHANGED"])
        );
        assert!(split(&result, Some(&original), &cases).unwrap().is_empty());
        // A schema change and an addition take the same three-way path.
        let mut incoming = files.clone();
        let mut schema = read(&original, SCHEMA).unwrap().value();
        schema
            .pointer_mut("/properties/steps/items/properties/expect/properties/reply")
            .unwrap()["minProperties"] = json!(1);
        schema
            .pointer_mut("/properties/steps/items/properties/expect/properties")
            .unwrap()["added"] = json!({"type":"string"});
        put(&mut incoming, SCHEMA, schema);
        let merged = apply(
            &incoming,
            split(&incoming, Some(&original), &BTreeSet::new()).unwrap(),
        );
        assert_eq!(
            read(&merged, "case-schema/expect/reply.json")
                .unwrap()
                .value()["minProperties"],
            json!(1)
        );
        assert!(merged.contains_key("case-schema/expect/added.json"));
    }

    #[test]
    fn merge_rejects_both_sides_changes_and_existing_case_removal() {
        let original = fixture();
        let mut files = split_fixture();
        let mut index = read(&files, "language/group/index.yaml").unwrap().value();
        index["cases"][0]["promises"] = json!(["PIECE"]);
        put(&mut files, "language/group/index.yaml", index);
        let mut changed = entry("a", "language/group");
        changed["promises"] = json!(["BRANCH"]);
        let incoming = with_inline(&files, json!([changed, entry("b", "language/other")]));
        assert_unchanged_error(&incoming, Some(&original), &BTreeSet::new(), "both sides");
        let incoming = with_inline(&split_fixture(), json!([entry("a", "language/group")]));
        assert_unchanged_error(
            &incoming,
            Some(&original),
            &BTreeSet::from(["language/other/b/case.yaml".into()]),
            "still exists",
        );
    }

    #[test]
    fn merge_rejects_schema_removal_and_split_root() {
        let original = fixture();
        let mut incoming = split_fixture();
        let mut schema = read(&original, SCHEMA).unwrap().value();
        schema
            .pointer_mut("/properties/steps/items/properties/expect/properties")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .clear();
        put(&mut incoming, SCHEMA, schema);
        assert_unchanged_error(
            &incoming,
            Some(&original),
            &BTreeSet::new(),
            "removed inline",
        );
        assert_unchanged_error(
            &incoming,
            Some(&split_fixture()),
            &BTreeSet::new(),
            "has no cases",
        );
    }

    #[test]
    fn merge_rejects_removing_all_inline_schema_elements() {
        let original = fixture();
        let mut incoming = split_fixture();
        let mut schema = read(&original, SCHEMA).unwrap().value();
        *schema
            .pointer_mut("/properties/steps/items/properties/input/oneOf")
            .unwrap() = json!([]);
        *schema
            .pointer_mut("/properties/steps/items/properties/expect/properties")
            .unwrap() = json!({});
        put(&mut incoming, SCHEMA, schema);
        assert_unchanged_error(
            &incoming,
            Some(&original),
            &BTreeSet::new(),
            "removed inline",
        );
    }

    #[test]
    fn placeholders_and_inline_branches_can_coexist() {
        let mut files = split_fixture();
        let mut schema = read(&files, SCHEMA).unwrap().value();
        schema
            .pointer_mut("/properties/steps/items/properties/input/oneOf")
            .unwrap()
            .as_array_mut()
            .unwrap()
            .push(json!({"properties":{"kind":{"enum":["two","three"]}}}));
        put(&mut files, SCHEMA, schema);
        let result = apply(&files, split(&files, None, &BTreeSet::new()).unwrap());
        assert!(result.contains_key("case-schema/inputs/two.json"));
        assert_eq!(
            assemble(&result)
                .unwrap()
                .pointer("/properties/steps/items/properties/input/oneOf")
                .unwrap()
                .as_array()
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn unknown_entry_member_is_never_dropped() {
        let files = split_fixture();
        let mut extra = entry("c", "language/new");
        extra["unknown"] = json!("keep this information visible");
        let incoming = with_inline(&files, json!([extra]));
        assert_unchanged_error(&incoming, None, &BTreeSet::new(), "unknown member");
    }

    #[test]
    fn assembly_appends_rule_files_in_filename_order() {
        let mut files = split_fixture();
        put(
            &mut files,
            "case-schema/rules/z.json",
            json!({"title":"last"}),
        );
        put(
            &mut files,
            "case-schema/rules/a.json",
            json!({"title":"first"}),
        );
        assert_eq!(
            assemble(&files)
                .unwrap()
                .pointer("/properties/steps/items/allOf")
                .unwrap(),
            &json!([{"required":["input"]},{"title":"first"},{"title":"last"}])
        );
    }

    #[test]
    fn comparison_rejects_changed_missing_extra_coverage_and_inputs() {
        let original = fixture();
        let files = split_fixture();
        for mutation in 0..5 {
            let mut altered = files.clone();
            match mutation {
                0 => {
                    let mut index = read(&altered, "language/group/index.yaml").unwrap().value();
                    index["cases"][0]["promises"] = json!(["CHANGED"]);
                    put(&mut altered, "language/group/index.yaml", index);
                }
                1 => {
                    altered.remove("language/group/index.yaml");
                }
                2 => {
                    put(
                        &mut altered,
                        "language/new/index.yaml",
                        json!({"cases":[entry("c","language/new")]}),
                    );
                }
                3 => {
                    let mut manifest = read(&altered, MANIFEST).unwrap().value();
                    manifest["s1_coverage"][0]["vectors"] = json!([]);
                    put(&mut altered, MANIFEST, manifest);
                }
                _ => {
                    put(
                        &mut altered,
                        "case-schema/inputs/one.json",
                        json!({"properties":{"kind":{"const":"one"}},"title":"changed"}),
                    );
                }
            }
            assert!(compare(&altered, &original).is_err(), "mutation {mutation}");
        }
    }

    #[test]
    fn comparison_uses_canonical_bytes_including_numeric_signs() {
        let mut original = fixture();
        let mut manifest = read(&original, MANIFEST).unwrap().value();
        manifest["cases"][0]["safety_rows"] = json!([-0.0]);
        put(&mut original, MANIFEST, manifest);
        let mut files = apply(&original, split(&original, None, &BTreeSet::new()).unwrap());
        let mut index = read(&files, "language/group/index.yaml").unwrap().value();
        index["cases"][0]["safety_rows"] = json!([0.0]);
        put(&mut files, "language/group/index.yaml", index);
        assert!(compare(&files, &original).unwrap_err().contains("changed"));
    }

    #[test]
    fn inventory_rejects_malformed_misplaced_duplicate_and_unsorted_pieces() {
        for value in [
            json!({}),
            json!({"cases":[]}),
            json!({"cases":[entry("a","language/group")],"extra":true}),
            json!({"cases":[entry("b","language/other")]}),
            json!({"cases":[entry("a","language/group"),entry("a","language/group")]}),
            json!({"cases":[entry("z","language/group"),entry("a","language/group")]}),
        ] {
            let mut files = Files::new();
            put(&mut files, "language/group/index.yaml", value);
            assert!(entries(&files).is_err());
        }
    }

    #[test]
    fn repository_metadata_round_trips_its_own_bytes() {
        let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("conformance");
        for name in [MANIFEST, SCHEMA] {
            let bytes = std::fs::read(directory.join(name)).unwrap();
            assert_eq!(
                Ordered::parse(&bytes).unwrap().pretty().unwrap(),
                bytes,
                "{name}"
            );
        }
    }
}
