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
    write: bool,
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
        if self.write {
            let rows = [
                (
                    "list_pull_requests",
                    vec!["owner", "repo", "state", "head", "base"],
                ),
                (
                    "create_pull_request",
                    vec!["owner", "repo", "title", "head", "base", "body"],
                ),
                (
                    "pull_request_read",
                    vec!["method", "owner", "repo", "pullNumber"],
                ),
                (
                    "merge_pull_request",
                    vec![
                        "owner",
                        "repo",
                        "pullNumber",
                        "merge_method",
                        "commit_title",
                        "commit_message",
                        "expectedHeadSha",
                    ],
                ),
            ];
            let tools = rows
                .into_iter()
                .filter(|(name, _)| mode != "missing-write-tool" || *name != "merge_pull_request")
                .map(|(name, fields)| {
                    let properties: serde_json::Map<String, Value> = fields
                        .into_iter()
                        .filter(|field| {
                            mode != "missing-expected-head" || *field != "expectedHeadSha"
                        })
                        .map(|field| (field.into(), json!({})))
                        .collect();
                    serde_json::from_value(json!({"name":name,
                        "inputSchema":{"type":"object","properties":properties}}))
                    .unwrap()
                })
                .collect();
            return Ok(ListToolsResult::with_all_items(tools));
        }
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
        if self.write {
            return Ok(self.write_call(&request.name, &arguments).await);
        }
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
        write: !env::args().any(|arg| arg == "--read-only"),
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

impl StandIn {
    async fn write_call(&self, tool: &str, args: &Value) -> CallToolResponse {
        let mode = self.mode();
        let path = self.home.join("pulls.json");
        let mut pulls: Vec<Value> =
            serde_json::from_str(&fs::read_to_string(&path).unwrap_or_else(|_| "[]".into()))
                .unwrap();
        let is_write = matches!(tool, "create_pull_request" | "merge_pull_request");
        let response = |value: Value, error: bool| {
            serde_json::from_value::<CallToolResult>(json!({
                "content":[{"type":"text","text":value.to_string()}],"isError":error
            }))
            .unwrap()
            .into()
        };
        if is_write {
            match mode.as_str() {
                "write-exit" => std::process::exit(0),
                "write-hold" => std::future::pending::<()>().await,
                "error-no-effect" => {
                    return response(json!(format!("synthetic refusal {}", self.token)), true);
                }
                "write-too-big" => return response(json!({"x":"x".repeat(MAX_TEXT)}), false),
                "write-not-json" => {
                    return serde_json::from_value::<CallToolResult>(json!({
                        "content":[{"type":"text","text":"not JSON"}],"isError":false
                    }))
                    .unwrap()
                    .into();
                }
                _ => {}
            }
        }
        let result = match tool {
            "list_pull_requests" => json!(
                pulls
                    .iter()
                    .filter(|pr| {
                        pr["state"] == "open"
                            && pr["base"]["ref"] == args["base"]
                            && format!(
                                "{}:{}",
                                args["owner"].as_str().unwrap(),
                                pr["head"]["ref"].as_str().unwrap()
                            ) == args["head"].as_str().unwrap()
                    })
                    .collect::<Vec<_>>()
            ),
            "create_pull_request" => {
                if pulls.iter().any(|pr| {
                    pr["state"] == "open"
                        && pr["base"]["ref"] == args["base"]
                        && pr["head"]["ref"] == args["head"]
                }) {
                    return response(json!("already open"), true);
                }
                let number = pulls
                    .iter()
                    .filter_map(|pr| pr["number"].as_u64())
                    .max()
                    .unwrap_or(0)
                    + 1;
                pulls.push(json!({"number":number,"head":{"ref":args["head"],
                    "sha":"1111111111111111111111111111111111111111"},
                    "base":{"ref":args["base"]},"state":"open","merged":false}));
                let repo = if mode == "wrong-url" {
                    "other"
                } else {
                    args["repo"].as_str().unwrap()
                };
                json!({"id":9000 + number,"url":format!("https://github.com/{}/{repo}/pull/{number}",args["owner"].as_str().unwrap())})
            }
            "pull_request_read" => {
                let Some(pr) = pulls
                    .iter_mut()
                    .find(|pr| pr["number"] == args["pullNumber"])
                else {
                    return response(json!("not found"), true);
                };
                if pr["merged"] == true {
                    if mode == "error-after-effect-unreadable" {
                        return response(json!("synthetic readback failure"), true);
                    }
                    if mode == "error-after-effect-malformed" {
                        return response(json!({"merged":true}), false);
                    }
                }
                let result = pr.clone();
                if mode == "move-head" {
                    pr["head"]["sha"] = json!("2222222222222222222222222222222222222222");
                }
                result
            }
            "merge_pull_request" => {
                let Some(pr) = pulls
                    .iter_mut()
                    .find(|pr| pr["number"] == args["pullNumber"])
                else {
                    return response(json!("not found"), true);
                };
                if pr["head"]["sha"] != args["expectedHeadSha"] {
                    return response(json!("head changed"), true);
                }
                if pr["state"] != "open" || pr["merged"] != false {
                    return response(json!("not open"), true);
                }
                assert_eq!(args["merge_method"], "squash");
                pr["state"] = json!("closed");
                pr["merged"] = json!(true);
                json!({"sha":"3333333333333333333333333333333333333333","merged":true,"message":"merged"})
            }
            _ => return response(json!("unknown tool"), true),
        };
        fs::write(path, serde_json::to_vec(&pulls).unwrap()).unwrap();
        if is_write && mode.starts_with("error-after-effect") {
            return response(json!("synthetic error after effect"), true);
        }
        response(result, false)
    }
}
