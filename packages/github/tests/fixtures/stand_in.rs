//! Test fixture only. No external service or network access.
use conduit_github::{MAX_TEXT, OPERATIONS, TOKEN_VARIABLE};
use rmcp::{ErrorData, RoleServer, ServerHandler, ServiceExt, model::*, service::RequestContext};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

struct StandIn {
    home: PathBuf,
    token: String,
}

impl StandIn {
    fn mode(&self) -> String {
        fs::read_to_string(self.home.join("mode")).unwrap_or_default()
    }

    fn record(&self, record: Value) {
        let mut log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.home.join("seen"))
            .unwrap();
        writeln!(log, "{record}").unwrap();
    }
}

impl ServerHandler for StandIn {
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2025-06-18","serverInfo":{"name":"github-mcp-stand-in","version":"stand-in-232"},"capabilities":{"tools":{}}})).unwrap()
    }

    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let mode = self.mode();
        let tools = OPERATIONS
            .into_iter()
            .filter(|op| mode != "missing-tool" || op.tool != "pull_request_read")
            .map(|op| {
                let mut properties = json!({"owner":{"type":"string"},"repo":{"type":"string"}});
                if mode != "missing-argument" || op.tool != "list_issues" {
                    properties[op.property] = json!({"type":"integer"});
                }
                if op.get {
                    properties["method"] = json!({"type":"string"});
                }
                serde_json::from_value(
                    json!({"name":op.tool,"inputSchema":{"type":"object","properties":properties}}),
                )
                .unwrap()
            })
            .collect();
        Ok(ListToolsResult::with_all_items(tools))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let mode = self.mode();
        let arguments = Value::Object(request.arguments.unwrap_or_default());
        self.record(json!({"tool":request.name,"arguments":arguments}));
        let number = ["issue_number", "pullNumber", "perPage"]
            .iter()
            .find_map(|key| arguments.get(key))
            .unwrap();
        let data = json!({"owner":arguments["owner"],"repo":arguments["repo"],"number":number});
        let (text, error) = match mode.as_str() {
            "error" => (format!("before {} after {} {}", self.token, self.token, "é".repeat(2048)), true),
            "token" => (json!({"first":self.token,"nested":[format!("prefix {} suffix",self.token), json!({self.token.clone():"key"})]}).to_string(), false),
            "not-json" => ("not JSON".into(), false),
            "scalar" => ("42".into(), false),
            "too-big" => (json!({"x":"x".repeat(MAX_TEXT)}).to_string(), false),
            "exit" => std::process::exit(0),
            _ => (data.to_string(), false),
        };
        Ok(serde_json::from_value::<CallToolResult>(
            json!({"content":[{"type":"text","text":text}],"isError":error}),
        )
        .unwrap()
        .into())
    }
}

#[tokio::main]
async fn main() {
    let fixture = StandIn {
        home: PathBuf::from(env::var_os("HOME").unwrap()),
        token: env::var(TOKEN_VARIABLE).unwrap(),
    };
    // Collect names only. The token itself is used in the synthetic wire
    // response tests and never included in a fixture record.
    let mut names: Vec<_> = env::vars_os()
        .map(|(name, _)| {
            name.into_string()
                .expect("environment name must be Unicode")
        })
        .collect();
    names.sort();
    fixture.record(json!({
        "process":std::process::id(),
        "arguments":env::args().skip(1).collect::<Vec<_>>(),
        "environment":names,
        "digest":format!("{:x}", Sha256::digest(fixture.token.as_bytes())),
    }));
    let stay_alive = fixture.mode() == "stay-alive-after-eof";
    let _ = fixture
        .serve(rmcp::transport::stdio())
        .await
        .unwrap()
        .waiting()
        .await;
    if stay_alive {
        std::future::pending::<()>().await;
    }
}
