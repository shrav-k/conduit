use agent_adapter::sign_in::{CAPTURE_VARIABLE, SIGN_IN_MESSAGE, SIGN_IN_RETURN_REFUSED, capture};
use agent_adapter::{Config, FailedTurn, Session, WRAPPER, header, prepare};
use anyhow::Result;
use rmcp::{ErrorData, RoleServer, ServerHandler, ServiceExt, model::*, service::RequestContext};
use serde_json::json;
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::{
    signal::unix::{SignalKind, signal},
    sync::Mutex,
};
#[derive(Clone)]
struct Wrapper(Arc<Mutex<Session>>, bool);
impl ServerHandler for Wrapper {
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2026-07-28","serverInfo":{"name":agent_adapter::WRAPPER,"version":env!("CARGO_PKG_VERSION")},"capabilities":{"tools":{}}})).unwrap()
    }
    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let tool = serde_json::from_value(json!({"name":"turn","description":"Explicitly run one message in this Agent's conversation.","inputSchema":{"type":"object","properties":{"message":{"type":"string"}},"required":["message"],"additionalProperties":false}})).unwrap();
        let mut tools = vec![tool];
        if self.1 {
            tools.push(serde_json::from_value(json!({"name":"retire","description":"Sign the client out of this Agent's own home and empty the home, at the Agent's retirement.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}})).unwrap());
            tools.push(serde_json::from_value(json!({"name":"account","description":"Report the account the client is signed in to in this Agent's own home, or null where none.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}})).unwrap());
            tools.push(serde_json::from_value(json!({"name":"sign-out","description":"Sign the client out of this Agent's own home, keeping the home.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}})).unwrap());
            tools.push(serde_json::from_value(json!({"name":"sign-in","description":"Start the client's browser login in this Agent's own home and give its URL.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}})).unwrap());
        }
        Ok(ListToolsResult::with_all_items(tools))
    }
    async fn call_tool(
        &self,
        r: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        if r.name == "sign-in" && self.1 {
            if r.request_state.is_some() || r.input_responses.is_some() {
                let refused =
                    CallToolResult::error(vec![ContentBlock::text(SIGN_IN_RETURN_REFUSED)]);
                return Ok(refused.into());
            }
            if r.arguments.as_ref().is_some_and(|args| !args.is_empty()) {
                return Err(ErrorData::invalid_params("expected no arguments", None));
            }
            if !context
                .meta
                .client_capabilities()
                .and_then(|caps| caps.elicitation)
                .is_some_and(|elicitation| elicitation.url.is_some())
            {
                let required = ClientCapabilities::builder()
                    .enable_elicitation_with(
                        ElicitationCapability::new().with_url(UrlElicitationCapability::new()),
                    )
                    .build();
                return Err(ErrorData::missing_required_client_capability(required));
            }
            let mut session = tokio::select! {
                biased;
                _ = context.ct.cancelled() => {
                    return Err(ErrorData::internal_error("hook cancelled", None));
                }
                session = self.0.lock() => session,
            };
            // Once the hook has the session, no cancellation cuts it; it has its own deadline.
            return Ok(match session.sign_in().await {
                Ok(url) => {
                    let request = json!({"method":"elicitation/create","params":{"mode":"url","message":SIGN_IN_MESSAGE,"url":url}});
                    let value =
                        json!({"resultType":"input_required","inputRequests":{"login":request}});
                    CallToolResponse::InputRequired(serde_json::from_value(value).unwrap())
                }
                Err(error) => {
                    CallToolResult::error(vec![ContentBlock::text(error.to_string())]).into()
                }
            });
        }
        if (r.name == "account" || r.name == "sign-out") && self.1 {
            if r.arguments.as_ref().is_some_and(|args| !args.is_empty()) {
                return Err(ErrorData::invalid_params("expected no arguments", None));
            }
            let mut session = tokio::select! {
                biased;
                _ = context.ct.cancelled() => {
                    return Err(ErrorData::internal_error("hook cancelled", None));
                }
                session = self.0.lock() => session,
            };
            // Once the hook has the session, no cancellation cuts it; each has its own deadline.
            let answer = if r.name == "account" {
                session.account().await.map(|account| {
                    let structured = json!({"account": account});
                    json!({"content":[{"type":"text","text":structured.to_string()}],"structuredContent":structured,"isError":false})
                })
            } else {
                session
                    .sign_out()
                    .await
                    .map(|text| json!({"content":[{"type":"text","text":text}],"isError":false}))
            };
            let result = match answer {
                Ok(value) => {
                    let mut result = serde_json::from_value::<CallToolResult>(value).unwrap();
                    result.result_type = Some(ResultType::COMPLETE);
                    result
                }
                Err(error) => CallToolResult::error(vec![ContentBlock::text(error.to_string())]),
            };
            return Ok(result.into());
        }
        if r.name == "retire" && self.1 {
            if r.arguments.as_ref().is_some_and(|args| !args.is_empty()) {
                return Err(ErrorData::invalid_params("expected no arguments", None));
            }
            let mut session = tokio::select! {
                biased;
                _ = context.ct.cancelled() => {
                    return Err(ErrorData::internal_error("retire cancelled", None));
                }
                session = self.0.lock() => session,
            };
            // Once the hook has the session, no cancellation cuts the sign-out.
            let result = match session.retire().await {
                Ok(text) => {
                    let mut result = serde_json::from_value::<CallToolResult>(
                        json!({"content":[{"type":"text","text":text}],"isError":false}),
                    )
                    .unwrap();
                    result.result_type = Some(ResultType::COMPLETE);
                    result
                }
                Err(error) => CallToolResult::error(vec![ContentBlock::text(error.to_string())]),
            };
            return Ok(result.into());
        }
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
                Err(session.cancel().await)
            }
            result = session.turn(message) => result,
        };
        let result = match turn {
            Ok(value) => {
                let mut result = serde_json::from_value::<CallToolResult>(
                    json!({"content":[{"type":"text","text":value["text"]}],"structuredContent":value,"isError":false}),
                )
                .unwrap();
                result.result_type = Some(ResultType::COMPLETE);
                result
            }
            Err(error) => {
                let mut content = vec![ContentBlock::text(error.to_string())];
                if let Some(fields) = error
                    .downcast_ref::<FailedTurn>()
                    .and_then(FailedTurn::fields)
                {
                    content.push(ContentBlock::text(fields.to_string()));
                }
                CallToolResult::error(content)
            }
        };
        Ok(result.into())
    }
}
async fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    // Run as a login's BROWSER by the sign-in hook alone, which sets the variable on that login.
    if let Some(file) = std::env::var_os(CAPTURE_VARIABLE).filter(|_| args.len() == 1) {
        return capture(Path::new(&file), &args[0]);
    }
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
    if config.configuration_error.is_none() && prepare(&config).is_err() {
        config.configuration_error = Some("invalid configuration: cannot prepare run files".into());
    }
    let term = config
        .v2
        .then(|| signal(SignalKind::terminate()))
        .transpose()?;
    let v2 = config.v2;
    let session = Session::new(config);
    let tree = session.tree();
    // Held until the process exits, so a TERM stop never drops the client before its tree read.
    let session = Arc::new(Mutex::new(session));
    let terminating = Arc::new(AtomicBool::new(false));
    if let Some(mut term) = term {
        let terminating = terminating.clone();
        // Serving goes on while this task stops the tree, so no in-flight turn is cancelled by
        // TERM; the tree runs one stop at a time, and once this stop begins no other starts.
        tokio::spawn(async move {
            if term.recv().await.is_some() {
                terminating.store(true, Ordering::SeqCst);
                if let Some(note) = tree.stop(true).await.note() {
                    eprintln!("{WRAPPER}: {note}");
                }
                std::process::exit(143);
            }
        });
    }
    let serving = session.clone();
    let serve = async move {
        let service = Wrapper(serving.clone(), v2)
            .serve(rmcp::transport::stdio())
            .await?;
        let result = service.waiting().await;
        let note = if terminating.load(Ordering::SeqCst) {
            None
        } else {
            serving.lock().await.shutdown().await.note()
        };
        if terminating.load(Ordering::SeqCst) {
            // The TERM task owns the stop, its note and the exit.
            std::future::pending::<()>().await;
        }
        if let Some(note) = note {
            eprintln!("{WRAPPER}: {note}");
        }
        result?;
        Ok(())
    };
    serve.await
}
#[tokio::main]
async fn main() {
    if run().await.is_err() {
        eprintln!("Agent wrapper failed");
        std::process::exit(1);
    }
}
