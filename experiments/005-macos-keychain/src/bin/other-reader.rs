//! The other reader: a different binary, never put on any item's access list (case 2).

fn main() {
    macos_keychain_probe::reader_main("other-reader", "other");
}
