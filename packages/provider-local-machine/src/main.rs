use anyhow::{Context, Result, ensure};
use provider_local_machine::{MachineServer, STATE_VARIABLE};
use rmcp::ServiceExt;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let directory = std::env::var_os(STATE_VARIABLE).context(
        "CONDUIT_STATE_DIR is required: supply this provider's own private state directory",
    )?;
    ensure!(
        !directory.is_empty(),
        "CONDUIT_STATE_DIR must name a state directory"
    );
    MachineServer::new(&PathBuf::from(directory))?
        .serve(rmcp::transport::stdio())
        .await?
        .waiting()
        .await?;
    Ok(())
}
