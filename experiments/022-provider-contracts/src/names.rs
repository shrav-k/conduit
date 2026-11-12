//! Specification-assigned names live only here; experiments are outside set_names.py.
pub const CORE: &str = "github.com/shrav-k/conduit/core";
pub const PREFIX: &str = "io.github.shrav-k.conduit/";
pub const ENGINE_URL: &str = "CONDUIT_URL";
pub const IDENTITY_DIR: &str = "CONDUIT_IDENTITY_DIR";
pub const STATE_DIR: &str = "CONDUIT_STATE_DIR";
pub const IDENTITY_FILES: [(&str, &str); 7] = [
    ("x509BundleFile", "ca.crt"),
    ("jwtBundleFile", "jwks.json"),
    ("engineIdFile", "engine-id"),
    ("tokenFile", "token"),
    ("certificateFile", "tls.crt"),
    ("keyFile", "tls.key"),
    // The directory variable is checked together with file names.
    ("identityDirVariable", IDENTITY_DIR),
];
pub fn key(name: &str) -> String {
    format!("{PREFIX}{name}")
}
pub fn extension() -> String {
    key("implementation-interface")
}
pub fn operation(name: &str) -> String {
    format!("{CORE}#{name}")
}
