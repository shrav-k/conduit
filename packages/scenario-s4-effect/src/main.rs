use rmcp::ServiceExt;
use scenario_s4_effect::{EffectServer, ledger};

#[tokio::main]
async fn main() {
    if let Err(cause) = run().await {
        eprintln!("{cause}");
        std::process::exit(1);
    }
}
async fn run() -> Result<(), &'static str> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.first().is_some_and(|a| a == "readback") && args.len() == 2 {
        let output = ledger::readback(std::path::Path::new(&args[1]), None)
            .map_err(|_| "S4 readback ledger I/O failed")?;
        println!("{output}");
        return Ok(());
    }
    if args.len() != 1 {
        return Err("expected plain, keyed, read or readback LEDGER");
    }
    let server = EffectServer::from_environment(args[0].to_str().unwrap_or(""))?;
    server
        .serve(rmcp::transport::stdio())
        .await
        .map_err(|_| "S4 stdio transport initialization failed")?
        .waiting()
        .await
        .map_err(|_| "S4 stdio transport failed")?;
    Ok(())
}
