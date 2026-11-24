fn main() {
    // The hook's one write cannot panic, so a panic with standard error gone never aborts.
    std::panic::set_hook(Box::new(|_| usage_guard_probe::panicked()));
    #[cfg(unix)]
    let code = usage_guard_probe::run::main_entry();
    #[cfg(not(unix))]
    let code = {
        let mut out = usage_guard_probe::Output::default();
        out.value("unix_required", true);
        out.finish();
        1
    };
    std::process::exit(code);
}
