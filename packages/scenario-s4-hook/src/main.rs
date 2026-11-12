use scenario_s4_hook::{BINARY, ledger, serve_hook, serve_reader};
#[tokio::main]
async fn main() {
    if run().await.is_err() {
        eprintln!("scenario-s4-hook: invalid command, ledger or stdio failure");
        std::process::exit(1);
    }
}
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [mode] if mode == "hook" => serve_hook(BINARY, None).await,
        [mode] if mode == "read" => serve_reader(BINARY).await,
        [mode, path] if mode == "readback" => {
            println!("{}", ledger::readback(std::path::Path::new(path), None)?);
            Ok(())
        }
        _ => Err("expected hook, read, or readback LEDGER".into()),
    }
}
