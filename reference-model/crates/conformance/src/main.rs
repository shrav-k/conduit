use std::path::PathBuf;
fn main() {
    let repository = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.."));
    match conduit_conformance::run_suite(&repository) {
        Ok(reports) => {
            for r in &reports {
                println!(
                    "PASS {} ({} steps; {})",
                    r.id,
                    r.steps,
                    r.promises.join(", ")
                );
            }
            println!("{} cases passed", reports.len());
        }
        Err(error) => {
            eprintln!("FAIL {error}");
            std::process::exit(1);
        }
    }
}
