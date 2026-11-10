use agent_adapter::{Config, Session, header, prepare};
use anyhow::Result;
use rmcp::{ErrorData, RoleServer, ServerHandler, ServiceExt, model::*, service::RequestContext};
use serde_json::json;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
#[derive(Clone)]
struct Wrapper(Arc<Mutex<Session>>);
impl ServerHandler for Wrapper {
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2026-07-28","serverInfo":{"name":agent_adapter::WRAPPER,"version":"0.1.0"},"capabilities":{"tools":{}}})).unwrap()
    }
    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let tool = serde_json::from_value(json!({"name":"turn","description":"Explicitly run one message in this Agent's conversation.","inputSchema":{"type":"object","properties":{"message":{"type":"string"}},"required":["message"],"additionalProperties":false}})).unwrap();
        Ok(ListToolsResult::with_all_items(vec![tool]))
    }
    async fn call_tool(
        &self,
        r: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        if r.name != "turn" {
            return Err(ErrorData::invalid_params("unknown tool", None));
        }
        let args = r.arguments.unwrap_or_default();
        let message = args
            .get("message")
            .and_then(|v| v.as_str())
            .filter(|_| args.len() == 1)
            .ok_or_else(|| ErrorData::invalid_params("expected only message string", None))?;
        let cancelled = || ErrorData::internal_error("turn cancelled", None);
        let mut session = tokio::select! {
            biased;
            _ = context.ct.cancelled() => return Err(cancelled()),
            session = self.0.lock() => session,
        };
        if context.ct.is_cancelled() {
            return Err(cancelled());
        }
        let turn = tokio::select! {
            biased;
            _ = context.ct.cancelled() => {
                session.cancel().await;
                return Err(cancelled());
            }
            result = session.turn(message) => result,
        };
        let result = match turn {
            Ok(value) => {
                json!({"content":[{"type":"text","text":value["text"]}],"structuredContent":value,"isError":false})
            }
            Err(_) => {
                json!({"content":[{"type":"text","text":"client turn failed; session cannot restart implicitly"}],"isError":true})
            }
        };
        Ok(serde_json::from_value::<CallToolResult>(result)
            .unwrap()
            .into())
    }
}
async fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--headers-helper") {
        let dir = if args.len() == 3 && args[1] == "--identity-dir" {
            PathBuf::from(&args[2])
        } else {
            anyhow::ensure!(args.len() == 1, "invalid helper arguments");
            PathBuf::from(std::env::var("CONDUIT_IDENTITY_DIR")?)
        };
        println!("{}", header(&dir)?);
        return Ok(());
    }
    let mut config = Config::from_env()?;
    // Test injection is explicit, and is never part of a package's implementation item.
    if args.len() == 2 && args[0] == "--client-program" {
        config.program = PathBuf::from(&args[1]);
    } else {
        anyhow::ensure!(args.is_empty(), "unknown wrapper arguments");
    }
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
#[tokio::main]
async fn main() {
    if run().await.is_err() {
        eprintln!("Agent wrapper failed");
        std::process::exit(1);
    }
}
