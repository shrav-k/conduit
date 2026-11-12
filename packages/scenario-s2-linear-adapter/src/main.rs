use rmcp::ServiceExt;
use scenario_s2_linear_adapter::{Adapter, BASE_VARIABLE, KEY_VARIABLE, redactor::Redactor};

#[tokio::main]
async fn main() {
    if run().await.is_err() {
        eprintln!("S2 Linear adapter stopped: configuration or transport failure");
        std::process::exit(1);
    }
}
async fn run() -> Result<(), ()> {
    let endpoint = std::env::var(BASE_VARIABLE).map_err(|_| ())?;
    let key = std::env::var(KEY_VARIABLE).map_err(|_| ())?;
    let output = Redactor::new(tokio::io::stdout(), &key);
    let adapter = Adapter::new(endpoint, key).map_err(|_| ())?;
    adapter
        .serve((tokio::io::stdin(), output))
        .await
        .map_err(|_| ())?
        .waiting()
        .await
        .map_err(|_| ())?;
    Ok(())
}
