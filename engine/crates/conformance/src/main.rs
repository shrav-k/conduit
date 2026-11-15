use std::path::PathBuf;
fn main() {
    let repository = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.."));
    let prefix = std::env::args().nth(2).unwrap_or_default();
    match conduit_conformance::run_matching(&repository, &prefix) {
        Ok(reports) => {
            for r in &reports.passed {
                println!(
                    "PASS {} ({} steps; {})",
                    r.id,
                    r.steps,
                    r.promises.join(", ")
                );
            }
            for r in &reports.expected_failures {
                println!(
                    "EXPECTED FAILURE {} ({}): {}",
                    r.id,
                    r.promises.join(", "),
                    r.reason
                );
                for line in r.error.lines() {
                    println!("  {line}");
                }
            }
            for r in &reports.passed {
                for sample in &r.storage {
                    println!("D19 {} {}", r.id, sample);
                }
            }
            if reports.expected_failures.is_empty() {
                println!("{} cases passed", reports.passed.len());
            } else {
                println!(
                    "{} cases passed; {} expected failures",
                    reports.passed.len(),
                    reports.expected_failures.len()
                );
            }
        }
        Err(error) => {
            eprintln!("FAIL {error}");
            std::process::exit(1);
        }
    }
}
