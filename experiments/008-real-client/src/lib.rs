pub mod runner;
pub mod stub;

use serde_json::Value;
use std::collections::BTreeMap;

/// Read the assigned names from the specification; no assigned value is duplicated here.
#[derive(Clone)]
pub struct Names(BTreeMap<String, Value>);
impl Default for Names {
    fn default() -> Self {
        Self(serde_yaml::from_str(include_str!("../../../spec/names.yaml")).expect("spec names"))
    }
}
impl Names {
    pub fn get(&self, name: &str) -> &str {
        self.0[name].as_str().expect("assigned name is a string")
    }
    pub fn meta(&self, suffix: &str) -> String {
        format!("{}{suffix}", self.get("metaPrefix"))
    }
}
