use agent_adapter::sign_in::{CAPTURE_VARIABLE, SIGN_IN_MESSAGE, SIGN_IN_RETURN_REFUSED, capture};
use agent_adapter::{
    Config, FailedTurn, LoginLapsed, Mode, Session, USAGE_MEMBER, WRAPPER, header, prepare,
};
use anyhow::Result;
use rmcp::{
    ErrorData, RoleServer, ServerHandler, Service, ServiceExt,
    model::*,
    service::{NotificationContext, RequestContext},
};
use serde_json::{Value, json};
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
struct Wrapper(Arc<Mutex<Session>>, bool, Mode);

impl Wrapper {
    fn has_login_hooks(&self) -> bool {
        self.1 && self.2 != Mode::SharedLoginAgent
    }
}

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
        let mut tools = Vec::new();
        if self.2 != Mode::LoginResource {
            tools.push(tool);
        }
        let empty_tool = |name: &str, description: &str| {
            serde_json::from_value(json!({"name":name,"description":description,"inputSchema":{"type":"object","properties":{},"additionalProperties":false}})).unwrap()
        };
        if self.2 == Mode::SharedLoginAgent {
            tools.push(empty_tool(
                "setup",
                "Check the client version and report its path.",
            ));
        }
        if self.1 {
            let description = match self.2 {
                Mode::Agent => {
                    "Sign the client out of this Agent's own home and empty the home, at the Agent's retirement."
                }
                Mode::SharedLoginAgent => {
                    "Stop the Agent's client and leave the account home to its login Resource."
                }
                Mode::LoginResource => {
                    "Sign out of the login Resource and empty its home, at retirement."
                }
            };
            tools.push(empty_tool("retire", description));
        }
        if self.has_login_hooks() {
            let home = if self.2 == Mode::LoginResource {
                "the login Resource's home"
            } else {
                "this Agent's own home"
            };
            tools.push(empty_tool(
                "account",
                &format!(
                    "Report the account the client is signed in to in {home}, or null where none."
                ),
            ));
            tools.push(empty_tool(
                "sign-out",
                &format!("Sign the client out of {home}, keeping the home."),
            ));
            tools.push(empty_tool(
                "sign-in",
                &format!("Start the client's browser login in {home} and give its URL."),
            ));
        }
        if self.2 != Mode::LoginResource {
            tools.push(
            serde_json::from_value(json!({
                "name": "new-conversation",
                "description": "Explicitly end this Agent's conversation and start a new one. The next turn resumes no earlier conversation. Files in the Agent's directory and home stay. This starts no model work.",
                "inputSchema": {"type": "object", "properties": {}, "additionalProperties": false},
            }))
            .unwrap(),
            );
        }
        Ok(ListToolsResult::with_all_items(tools))
    }
    async fn call_tool(
        &self,
        r: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        if r.name == "sign-in" && self.has_login_hooks() {
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
        if (r.name == "account" || r.name == "sign-out") && self.has_login_hooks() {
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
        if (r.name == "retire" && self.1) || (r.name == "setup" && self.2 == Mode::SharedLoginAgent)
        {
            if r.arguments.as_ref().is_some_and(|args| !args.is_empty()) {
                return Err(ErrorData::invalid_params("expected no arguments", None));
            }
            let mut session = tokio::select! {
                biased;
                _ = context.ct.cancelled() => {
                    return Err(ErrorData::internal_error(format!("{} cancelled", r.name), None));
                }
                session = self.0.lock() => session,
            };
            // Once the hook has the session, no cancellation cuts the sign-out.
            let answer = if r.name == "setup" {
                session.setup_shared_login().await
            } else {
                session.retire().await
            };
            let result = match answer {
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
        if r.name == "new-conversation" && self.2 != Mode::LoginResource {
            if r.arguments.as_ref().is_some_and(|args| !args.is_empty()) {
                return Err(ErrorData::invalid_params("expected no arguments", None));
            }
            let mut session = tokio::select! {
                biased;
                _ = context.ct.cancelled() => {
                    return Err(ErrorData::internal_error("new conversation cancelled", None));
                }
                session = self.0.lock() => session,
            };
            // Once the call has the session, let the stop finish.
            let result = match session.new_conversation().await {
                Ok(value) => {
                    let mut result = serde_json::from_value::<CallToolResult>(json!({
                        "content": [{"type": "text", "text": value.to_string()}],
                        "structuredContent": value,
                        "isError": false,
                    }))
                    .unwrap();
                    result.result_type = Some(ResultType::COMPLETE);
                    result
                }
                Err(error) => CallToolResult::error(vec![ContentBlock::text(error.to_string())]),
            };
            return Ok(result.into());
        }
        if r.name != "turn" || self.2 == Mode::LoginResource {
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
            Err(error) if error.is::<LoginLapsed>() => {
                let mut result = serde_json::from_value::<InputRequiredResult>(json!({
                    "resultType": "input_required",
                    "inputRequests": {"sign-in": {
                        "method": "elicitation/create",
                        "params": {
                            "mode": "url",
                            "message": "The login Resource's Claude Code login is missing or has lapsed.",
                            "url": "https://code.claude.com/docs/en/authentication"
                        }
                    }}
                })).unwrap();
                result.meta = error
                    .downcast::<LoginLapsed>()
                    .unwrap()
                    .usage
                    .map(usage_meta);
                return Ok(result.into());
            }
            Ok(mut value) => {
                let meta = value.as_object_mut().unwrap().remove("_meta");
                let mut reply = json!({"content":[{"type":"text","text":value["text"]}],"structuredContent":value,"isError":false});
                if let Some(meta) = meta {
                    reply["_meta"] = meta;
                }
                let mut result = serde_json::from_value::<CallToolResult>(reply).unwrap();
                result.result_type = Some(ResultType::COMPLETE);
                result
            }
            Err(error) => {
                let mut content = vec![ContentBlock::text(error.to_string())];
                let failed = error.downcast_ref::<FailedTurn>();
                if let Some(fields) = failed.and_then(FailedTurn::fields) {
                    content.push(ContentBlock::text(fields.to_string()));
                }
                let mut result = CallToolResult::error(content);
                result.meta = failed
                    .and_then(|failed| failed.usage.clone())
                    .map(usage_meta);
                result
            }
        };
        Ok(result.into())
    }
}

fn usage_meta(report: Value) -> MetaObject {
    MetaObject(serde_json::Map::from_iter([(
        USAGE_MEMBER.to_owned(),
        report,
    )]))
}

struct WrapperProtocol(Wrapper);

impl Service<RoleServer> for WrapperProtocol {
    async fn handle_request(
        &self,
        request: ClientRequest,
        context: RequestContext<RoleServer>,
    ) -> Result<ServerResult, ErrorData> {
        let response = self.0.handle_request(request, context).await?;
        if let ServerResult::InputRequiredResult(ref input) = response
            && self.0.2 == Mode::SharedLoginAgent
            && input.request_state.is_none()
        {
            // rmcp emits an empty elicitationId for a URL request that omitted it.
            // A shared-login lapse is a report, so its wire form has no elicitationId.
            let mut value = serde_json::to_value(input).unwrap();
            value["inputRequests"]["sign-in"]["params"]
                .as_object_mut()
                .unwrap()
                .remove("elicitationId");
            return Ok(ServerResult::CustomResult(CustomResult::new(value)));
        }
        Ok(response)
    }

    async fn handle_notification(
        &self,
        notification: ClientNotification,
        context: NotificationContext<RoleServer>,
    ) -> Result<(), ErrorData> {
        self.0.handle_notification(notification, context).await
    }

    fn get_info(&self) -> ServerConfig {
        ServerHandler::get_info(&self.0)
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
    // Test injection is explicit, and is never part of a package's implementation item.
    let (program, mode_args) = match args.as_slice() {
        [flag, program, rest @ ..] if flag == "--client-program" => (Some(program), rest),
        args => (None, args),
    };
    let mode = match mode_args {
        [] => Mode::Agent,
        [flag] if flag == "--login-resource" => Mode::LoginResource,
        [flag] if flag == "--shared-login" => Mode::SharedLoginAgent,
        _ => anyhow::bail!("unknown wrapper arguments"),
    };
    let mut config = Config::from_env(mode)?;
    if let Some(program) = program {
        config.set_client_program(std::path::Path::new(program));
    }
    if mode != Mode::LoginResource
        && config.configuration_error.is_none()
        && prepare(&config).is_err()
    {
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
        let service = WrapperProtocol(Wrapper(serving.clone(), v2, mode))
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
