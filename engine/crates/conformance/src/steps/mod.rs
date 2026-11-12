//! One module per existing input-dispatch arm.
use super::*;
mod access_fixture;
mod core;
mod install_archive;
mod load;
mod read;
mod request_fixture;
mod resolve_project;
mod restore_state;
mod schema_case;
mod validate_values;
pub(super) struct Step<'a> {
    pub base: &'a Path,
    pub case: &'a Case,
    pub input: &'a Value,
    pub kind: &'a str,
    pub state: &'a State,
    pub context: &'a Context,
    pub spec: &'a Specification,
    pub symbols: &'a BTreeMap<String, Value>,
}
impl Step<'_> {
    fn file_bytes(&self, alias: &str) -> Result<(String, Vec<u8>)> {
        let name = self
            .case
            .files
            .get(alias)
            .ok_or_else(|| format!("undeclared file {alias}"))?;
        Ok((
            alias.into(),
            std::fs::read(self.base.join(name)).map_err(|e| format!("{name}: {e}"))?,
        ))
    }
}
// Keep one line per dispatch arm for later input kinds.
#[rustfmt::skip]
pub(super) fn run(step: &Step<'_>) -> Result<(Value, State)> {
    match step.kind {
        "request-fixture" | "lifecycle-fixture" => request_fixture::run(step),
        "restore-state" => restore_state::run(step),
        "access-fixture" => access_fixture::run(step),
        "install-archive" => install_archive::run(step),
        "resolve-project" | "register-project" | "registration-orders" => resolve_project::run(step),
        "read" => read::run(step),
        "load" | "submit-files" => load::run(step),
        "schema-case" | "schema-admission" | "schema-evaluation" => schema_case::run(step),
        "validate-values" => validate_values::run(step),
        _ => core::run(step),
    }
}
