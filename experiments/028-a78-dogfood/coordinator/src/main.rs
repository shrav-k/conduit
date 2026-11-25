use std::process::exit;

fn main() {
    let mut args = Vec::new();
    for arg in std::env::args_os().skip(1) {
        match arg.into_string() {
            Ok(arg) => args.push(arg),
            Err(_) => {
                eprintln!("wo: an argument is not valid UTF-8");
                exit(a78_coordinator::EXIT_USAGE);
            }
        }
    }
    exit(a78_coordinator::cli::main(&args));
}
