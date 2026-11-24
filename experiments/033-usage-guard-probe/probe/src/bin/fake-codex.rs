fn main() {
    #[cfg(unix)]
    std::process::exit(usage_guard_probe::fake::main_entry(true));
    #[cfg(not(unix))]
    std::process::exit(1);
}
