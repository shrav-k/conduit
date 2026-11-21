use anyhow::Result;
use conduit_git::WorkingCopyServer;
use rmcp::{ServiceExt, transport::async_rw::AsyncRwTransport};

#[tokio::main]
async fn main() -> Result<()> {
    let (input, output) = rmcp::transport::stdio();
    // A UTF-8 payload can expand to six wire bytes per byte under JSON escaping.
    let transport = AsyncRwTransport::new(input, output).with_max_line_length(64 * 1024 * 1024);
    WorkingCopyServer.serve(transport).await?.waiting().await?;
    Ok(())
}
