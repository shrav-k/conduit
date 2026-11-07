//! The trusted reader: the binary the probe puts on an item's access list.
//!
//! Its build tag is fixed at compile time from `PROBE_READER_TAG`, so rebuilding it with another
//! tag changes its code and so its code-directory hash (case 4).

const BUILD_TAG: &str = match option_env!("PROBE_READER_TAG") {
    Some(tag) => tag,
    None => "untagged",
};

fn main() {
    macos_keychain_probe::reader_main("reader", BUILD_TAG);
}
