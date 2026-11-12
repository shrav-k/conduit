use crate::{form::Descriptor, names};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    path::{Component, Path},
};
pub type Values = BTreeMap<String, Value>;
pub fn relative(p: &str) -> bool {
    !p.is_empty()
        && Path::new(p)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
}
pub struct Context<'a> {
    pub descriptor: &'a Descriptor,
    pub fixtures: &'a Values,
    pub captures: &'a Values,
    pub lab: &'a Path,
}
impl Context<'_> {
    pub fn expand(&self, value: &Value) -> Result<Value, &'static str> {
        self.inner(value, 0)
    }
    fn inner(&self, value: &Value, depth: usize) -> Result<Value, &'static str> {
        if depth > 32 {
            return Err("substitution cycle or depth exceeded");
        }
        match value {
            Value::Object(m) => {
                if m.keys().any(|k| k.starts_with('$')) {
                    if m.len() != 1 {
                        return Err("placeholder must be the whole value");
                    }
                    let (kind, n) = m.iter().next().unwrap();
                    let n = n.as_str().ok_or("placeholder requires name")?;
                    let v = match kind.as_str() {
                        "$param" => self.descriptor.parameters.get(n),
                        "$fixture" => self.fixtures.get(n),
                        "$capture" => {
                            return self.captures.get(n).cloned().ok_or("unknown capture");
                        }
                        "$lab" => {
                            let p = self.descriptor.lab_paths.get(n).ok_or("unknown lab path")?;
                            if !relative(p) {
                                return Err("lab path must be relative");
                            }
                            let path = self.lab.join(p);
                            let root = std::fs::canonicalize(self.lab)
                                .map_err(|_| "lab directory missing")?;
                            let mut ancestor = path.as_path();
                            loop {
                                if ancestor.symlink_metadata().is_ok() {
                                    let resolved = std::fs::canonicalize(ancestor)
                                        .map_err(|_| "lab fixture link is invalid")?;
                                    if !resolved.starts_with(&root) {
                                        return Err("lab fixture escapes lab");
                                    }
                                    break;
                                }
                                ancestor = ancestor.parent().ok_or("lab fixture escapes lab")?;
                            }
                            return Ok(Value::String(path.to_string_lossy().into_owned()));
                        }
                        _ => return Err("unknown placeholder"),
                    }
                    .ok_or("unknown substitution name")?;
                    self.inner(v, depth + 1)
                } else {
                    m.iter()
                        .map(|(k, v)| Ok((k.clone(), self.inner(v, depth + 1)?)))
                        .collect::<Result<serde_json::Map<_, _>, _>>()
                        .map(Value::Object)
                }
            }
            Value::Array(a) => a
                .iter()
                .map(|v| self.inner(v, depth + 1))
                .collect::<Result<Vec<_>, _>>()
                .map(Value::Array),
            Value::String(s) => Ok(Value::String(expand_meta(s)?)),
            _ => Ok(value.clone()),
        }
    }
}
pub fn expand_meta(s: &str) -> Result<String, &'static str> {
    let re = regex::Regex::new(r"\{meta:([A-Za-z]+)\}").unwrap();
    let result = re
        .replace_all(s, |c: &regex::Captures<'_>| names::key(&c[1]))
        .into_owned();
    if result.contains("{meta:") {
        return Err("invalid metadata placeholder");
    }
    Ok(result)
}

pub fn expand_pointer(s: &str) -> Result<String, &'static str> {
    let re = regex::Regex::new(r"\{meta:([A-Za-z]+)\}").unwrap();
    let result = re
        .replace_all(s, |c: &regex::Captures<'_>| {
            names::key(&c[1]).replace('~', "~0").replace('/', "~1")
        })
        .into_owned();
    if result.contains("{meta:") {
        return Err("invalid metadata placeholder");
    }
    Ok(result)
}
