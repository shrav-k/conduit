use anyhow::Result;
use provider_local_directory::DirectoryServer;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<()> {
    DirectoryServer
        .serve(rmcp::transport::stdio())
        .await?
        .waiting()
        .await?;
    Ok(())
}
