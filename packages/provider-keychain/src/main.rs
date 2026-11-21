use anyhow::Result;
use provider_keychain::KeychainServer;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<()> {
    KeychainServer
        .serve(rmcp::transport::stdio())
        .await?
        .waiting()
        .await?;
    Ok(())
}
