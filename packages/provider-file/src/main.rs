use anyhow::Result;
use provider_file::FileServer;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<()> {
    FileServer
        .serve(rmcp::transport::stdio())
        .await?
        .waiting()
        .await?;
    Ok(())
}
