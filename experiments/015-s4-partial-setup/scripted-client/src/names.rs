// Sources: metaPrefix in spec/names.yaml; spec/definitions.md, "Conduit's extension".
// tools/set_names.py does not reach experiments/: keep these in step with those sources.
pub const PREFIX: &str = "io.github.shrav-k.conduit/";
pub const EXTENSION: &str = "io.github.shrav-k.conduit/implementation-interface";
pub fn key(name: &str) -> String {
    format!("{PREFIX}{name}")
}
