use std::path::Path;
#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 5 {
        eprintln!("usage: provider-contracts SUITE-INDEX DESCRIPTOR LAB REPORT.json REPORT.md");
        std::process::exit(2);
    }
    let report = match provider_contracts::run_suite(
        Path::new(&args[0]),
        Path::new(&args[1]),
        Path::new(&args[2]),
        &Default::default(),
    )
    .await
    {
        Ok(r) => r,
        Err(reason) => {
            eprintln!("{reason}");
            std::process::exit(2);
        }
    };
    if std::fs::write(&args[3], serde_json::to_vec_pretty(&report).unwrap()).is_err()
        || std::fs::write(&args[4], report.markdown()).is_err()
    {
        eprintln!("cannot write report");
        std::process::exit(2);
    }
    println!(
        "passed: {}; failed: {}; untested: {}",
        report.totals.passed, report.totals.failed, report.totals.untested
    );
    if report.totals.failed > 0 {
        std::process::exit(1);
    }
}
