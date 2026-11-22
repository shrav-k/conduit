use agent_adapter::{
    Config, FailedTurn, LOGIN_MESSAGE, LoginLapsed, Mode, Session, SignIn, SignInCall, WRAPPER,
    header, prepare,
};
use anyhow::Result;
use rmcp::{
    ErrorData, RoleServer, ServerHandler, Service, ServiceExt,
    model::*,
    service::{NotificationContext, RequestContext},
};
use serde_json::json;
use std::{
    path::PathBuf,
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
            serde_json::from_value(json!({
                "name": name, "description": description,
                "inputSchema":{"type":"object","properties":{},"additionalProperties":false}
            }))
            .unwrap()
        };
        if self.1 {
            let description = match self.2 {
                Mode::Agent => {
                    "Sign the client out of this Agent's own home and empty the home, at the Agent's retirement."
                }
                Mode::SharedLoginAgent => {
                    "Stop the Agent's client and empty its own home, leaving the login Resource signed in."
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
            tools.push(empty_tool("account", &format!("Report the account the client is signed in to in {home}, or none, as the client itself reports it.")));
            tools.push(empty_tool(
                "sign-out",
                &format!("Sign the client out of {home}, keeping the home."),
            ));
            tools.push(empty_tool(
                "sign-in",
                &format!("Start the client's login in {home} and give its URL to the engine."),
            ));
        }
        if self.2 == Mode::SharedLoginAgent {
            tools.push(empty_tool(
                "setup",
                "Link the Agent's home to the login Resource's login file.",
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
            let answer = if r.name == "setup" {
                session.setup_shared_login().map(str::to_owned)
            } else {
                // Once the hook has the session, no cancellation cuts the sign-out.
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
        if self.has_login_hooks() && (r.name == "account" || r.name == "sign-out") {
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
            // Once the hook has the session, it finishes the operation and its cleanup.
            let result = if r.name == "account" {
                session.account().await.map(|report| {
                    json!({"content":[{"type":"text","text":report.text}],"structuredContent":{"account":report.account},"isError":false})
                })
            } else {
                session
                    .sign_out()
                    .await
                    .map(|text| json!({"content":[{"type":"text","text":text}],"isError":false}))
            };
            let result = match result {
                Ok(value) => {
                    let mut result = serde_json::from_value::<CallToolResult>(value).unwrap();
                    result.result_type = Some(ResultType::COMPLETE);
                    result
                }
                Err(error) => CallToolResult::error(vec![ContentBlock::text(error.to_string())]),
            };
            return Ok(result.into());
        }
        if self.has_login_hooks() && r.name == "sign-in" {
            if r.arguments.as_ref().is_some_and(|args| !args.is_empty()) {
                return Err(ErrorData::invalid_params("expected no arguments", None));
            }
            let mut session = tokio::select! {
                biased;
                _ = context.ct.cancelled() => {
                    return Err(ErrorData::internal_error("sign-in cancelled", None));
                }
                session = self.0.lock() => session,
            };
            let call = SignInCall {
                login: context
                    .meta
                    .get("io.github.shrav-k.conduit/signInLogin")
                    .cloned(),
                returned: context
                    .meta
                    .contains_key("io.github.shrav-k.conduit/signInReturn"),
                request_state: r.request_state,
            };
            return Ok(match session.sign_in(call).await {
                Ok(SignIn::Url { url, request_state }) => {
                    let message = if self.2 == Mode::LoginResource {
                        "Open this page to sign Codex in for the login Resource."
                    } else {
                        LOGIN_MESSAGE
                    };
                    let result = serde_json::from_value::<InputRequiredResult>(json!({
                        "resultType": "input_required",
                        "inputRequests": {"sign-in": {
                            "method": "elicitation/create",
                            "params": {"mode":"url", "message":message, "url":url, "elicitationId":"sign-in"}
                        }},
                        "requestState": request_state,
                    })).unwrap();
                    CallToolResponse::InputRequired(result)
                }
                Ok(SignIn::Text(text)) => {
                    let mut result = serde_json::from_value::<CallToolResult>(
                        json!({"content":[{"type":"text","text":text}],"isError":false}),
                    )
                    .unwrap();
                    result.result_type = Some(ResultType::COMPLETE);
                    result.into()
                }
                Err(error) => {
                    CallToolResult::error(vec![ContentBlock::text(error.to_string())]).into()
                }
            });
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
                return Ok(serde_json::from_value::<InputRequiredResult>(json!({
                    "resultType": "input_required",
                    "inputRequests": {"sign-in": {
                        "method": "elicitation/create",
                        "params": {
                            "mode": "url",
                            "message": "The login Resource's Codex login is missing or has lapsed.",
                            "url": "https://learn.chatgpt.com/docs/auth"
                        }
                    }}
                }))
                .unwrap()
                .into());
            }
            Ok(mut value) => {
                let meta = value
                    .as_object_mut()
                    .and_then(|fields| fields.remove("_meta"));
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
        config.program = PathBuf::from(program);
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
