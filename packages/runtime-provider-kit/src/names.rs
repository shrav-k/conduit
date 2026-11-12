//! Assigned names, kept in one module; identity filenames come from the copied schema.
pub const META_PREFIX: &str = "io.github.shrav-k.conduit/";
pub const CORE_PACKAGE: &str = "github.com/shrav-k/conduit/core";
pub const ENGINE_URL_VARIABLE: &str = "CONDUIT_URL";
pub const IDENTITY_DIR_VARIABLE: &str = "CONDUIT_IDENTITY_DIR";
pub const STATE_DIR_VARIABLE: &str = "CONDUIT_STATE_DIR";
pub fn key(name: &str) -> String {
    format!("{META_PREFIX}{name}")
}
pub fn extension() -> String {
    key("implementation-interface")
}
