use crate::{
    Client,
    client::{Config, Session, prepare},
    run_environment,
};
use anyhow::Result;
use rmcp::{ErrorData, RoleServer, ServerHandler, ServiceExt, model::*, service::RequestContext};
use serde_json::json;
use std::{path::PathBuf, sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(clap::Parser)]
pub struct Args {
    /// Experiment evidence, kept outside the private empty working directory.
    #[arg(long)]
    pub evidence: PathBuf,
    /// Allows a scripted fixture in offline tests; normally the installed client name.
    #[arg(long)]
    pub client_program: Option<PathBuf>,
    #[arg(long, default_value_t = 180, value_parser = clap::value_parser!(u64).range(1..))]
    pub timeout_secs: u64,
}
#[derive(Clone)]
struct Wrapper(Arc<Mutex<Session>>);
impl ServerHandler for Wrapper {
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2026-07-28","serverInfo":{"name":"native-launch-wrapper","version":"0.1.0"},"capabilities":{"tools":{}}})).unwrap()
    }
    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let tool: Tool = serde_json::from_value(json!({"name":"turn","description":"Run one explicit user turn in this Agent's existing client session.","inputSchema":{"type":"object","properties":{"message":{"type":"string"}},"required":["message"],"additionalProperties":false},"outputSchema":{"type":"object","properties":{"text":{"type":"string"},"events":{"type":"array"},"start_report":{"type":"object"}},"required":["text","events","start_report"]}})).unwrap();
        Ok(ListToolsResult::with_all_items(vec![tool]))
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        if request.name != "turn" {
            return Err(ErrorData::invalid_params("unknown tool", None));
        }
        let args = request.arguments.unwrap_or_default();
        let message = args
            .get("message")
            .and_then(|value| value.as_str())
            .filter(|_| args.len() == 1)
            .ok_or_else(|| ErrorData::invalid_params("expected only a string message", None))?;
        let mut session = self.0.lock().await;
        let result = match session.turn(message).await {
            Ok(value) => {
                json!({"content":[{"type":"text","text":value["text"]}],"structuredContent":value,"isError":false})
            }
            Err(error) => {
                json!({"content":[{"type":"text","text":format!("{error:#}")}],"isError":true})
            }
        };
        Ok(serde_json::from_value::<CallToolResult>(result)
            .unwrap()
            .into())
    }
}

pub async fn serve(client: Client, args: Args) -> Result<()> {
    let names = real_client_harness::Names::default();
    let url = std::env::var(names.get("engineUrlVariable"))?;
    let identity = PathBuf::from(std::env::var(names.get("identityDirVariable"))?);
    anyhow::ensure!(
        args.evidence.is_absolute(),
        "evidence path must be absolute"
    );
    let config = Config {
        client,
        program: args.client_program.unwrap_or_else(|| client.name().into()),
        helper: std::env::current_exe()?,
        evidence: args.evidence,
        cwd: std::env::current_dir()?,
        env: run_environment(crate::inherited_environment(), &url, &identity),
        url,
        timeout: Duration::from_secs(args.timeout_secs),
    };
    prepare(&config)?;
    let session = Arc::new(Mutex::new(Session::new(config)));
    let service = Wrapper(session.clone())
        .serve(rmcp::transport::stdio())
        .await?;
    let result = service.waiting().await;
    session.lock().await.shutdown().await;
    result?;
    Ok(())
}
