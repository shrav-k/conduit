fn main() {
    if let Err(message) = client_facts::run::main() {
        eprintln!("{message}");
        std::process::exit(1);
    }
}
