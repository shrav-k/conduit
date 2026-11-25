//! A Conduit Resource serving one local Git working copy.
#[cfg(not(unix))]
compile_error!("conduit-git requires Unix");

mod files;
mod objects;
mod repository;

use rmcp::{ErrorData, RoleServer, ServerHandler, model::*, service::RequestContext};
use serde_json::{Map, Value, json};
use std::path::PathBuf;

pub const BINARY: &str = "conduit-git";
pub const ADDRESS: &str = "example.org/conduit/git";
pub const CORE: &str = "github.com/shrav-k/conduit/core";
pub const PREFIX: &str = "io.github.shrav-k.conduit/";
pub const EXTENSION: &str = "io.github.shrav-k.conduit/implementation-interface";
pub const OUTCOME: &str = "io.github.shrav-k.conduit/outcome";
pub const REASON: &str = "io.github.shrav-k.conduit/reason";
pub const RESTRICTION: &str = "io.github.shrav-k.conduit/restriction";
pub const MAX_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Copy)]
enum Operation {
    Setup,
    Retire,
    Read,
    List,
    Write,
    Changes,
    Move,
}

impl Operation {
    const ALL: [Self; 7] = [
        Self::Setup,
        Self::Retire,
        Self::Read,
        Self::List,
        Self::Write,
        Self::Changes,
        Self::Move,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Setup => "setup-hook.v1",
            Self::Retire => "retire-hook.v1",
            Self::Read => "file.read.v1",
            Self::List => "directory.list.v1",
            Self::Write => "file.write.v1",
            Self::Changes => "changes.v1",
            Self::Move => "move.v1",
        }
    }

    fn hook(self) -> bool {
        matches!(self, Self::Setup | Self::Retire)
    }

    fn qualified(self) -> String {
        let package = if self.hook() { CORE } else { ADDRESS };
        format!("{package}#{}", self.name())
    }

    fn output(self) -> Option<Value> {
        match self {
            Self::Read => Some(
                json!({"type":"object","properties":{"content":{"type":"string"}},"required":["content"],"additionalProperties":false}),
            ),
            Self::List => Some(
                json!({"type":"object","properties":{"entries":{"type":"array","items":{"type":"object","properties":{"name":{"type":"string"},"kind":{"enum":["file","directory","symlink","other"]}},"required":["name","kind"],"additionalProperties":false}}},"required":["entries"],"additionalProperties":false}),
            ),
            Self::Changes => Some(
                json!({"type":"object","properties":{"base":{"type":"string","pattern":"^[0-9a-f]{40}$"},"files":{"type":"array","items":{"type":"object","properties":{"path":{"type":"string"},"change":{"enum":["added","modified","deleted"]}},"required":["path","change"],"additionalProperties":false}},"patch":{"type":"string"}},"required":["base","files","patch"],"additionalProperties":false}),
            ),
            Self::Move => {
                let mut output = Self::Changes.output().unwrap();
                output["properties"]
                    .as_object_mut()
                    .unwrap()
                    .remove("patch");
                output["required"] = json!(["base", "files"]);
                Some(output)
            }
            _ => None,
        }
    }

    fn inputs(self) -> Value {
        let path = json!({"type":"string","anyOf":[
            {"const":"."},
            {"pattern":"^[^/]+(/[^/]+)*$","not":{"pattern":"(^|/)([.]{1,2}|[.][gG][iI][tT])(/|$)"}}
        ]});
        let (properties, required) = match self {
            Self::Read | Self::List => (json!({"path":path}), json!(["path"])),
            Self::Move => (
                json!({"commit":{"type":"string","pattern":"^[0-9a-f]{40}$"},
                    "expectedBase":{"type":"string","pattern":"^[0-9a-f]{40}$"},
                    "expectedPatchSha256":{"type":"string","pattern":"^[0-9a-f]{64}$"},
                    "patch":{"type":"string"}}),
                json!(["commit", "expectedBase", "expectedPatchSha256"]),
            ),
            Self::Write => (
                json!({"path":path,"content":{"type":"string"}}),
                json!(["path", "content"]),
            ),
            _ => return json!({"type":"object","additionalProperties":false}),
        };
        json!({"type":"object","properties":properties,"required":required,"additionalProperties":false})
    }
}

pub fn schemas(name: &str) -> Option<(Value, Option<Value>)> {
    Operation::ALL
        .into_iter()
        .find(|op| op.name() == name)
        .map(|op| (op.inputs(), op.output()))
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    repository: PathBuf,
    commit: String,
    root: PathBuf,
}

impl Config {
    fn parse(value: Value) -> Result<Self, String> {
        let config: Self = serde_json::from_value(value)
            .map_err(|_| "configuration requires only repository, commit and root strings")?;
        for (name, path) in [("repository", &config.repository), ("root", &config.root)] {
            if !path.is_absolute() || path.as_os_str().as_encoded_bytes().contains(&0) {
                return Err(format!(
                    "configuration {name} must be an absolute path without NUL bytes"
                ));
            }
        }
        if config.commit.len() != 40
            || !config
                .commit
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("configuration commit must be 40 lowercase hexadecimal digits".into());
        }
        Ok(config)
    }
}

fn response(result: Result<Option<Value>, String>) -> CallToolResponse {
    let wire = match result {
        Ok(outputs) => {
            let mut wire = json!({"content":[],"isError":false,"_meta":{OUTCOME:"succeeded"}});
            if let Some(outputs) = outputs {
                wire["structuredContent"] = outputs;
            }
            wire
        }
        Err(message) => {
            json!({"content":[{"type":"text","text":message}],"isError":true,"_meta":{OUTCOME:"failed"}})
        }
    };
    serde_json::from_value::<CallToolResult>(wire)
        .unwrap()
        .into()
}

fn invoke(
    operation: Operation,
    args: Map<String, Value>,
    config: Config,
) -> Result<Option<Value>, String> {
    let path = match operation {
        Operation::Read | Operation::List | Operation::Write => {
            let expected = if matches!(operation, Operation::Write) {
                2
            } else {
                1
            };
            if args.len() != expected {
                return Err("unexpected working-copy arguments".into());
            }
            let path = args
                .get("path")
                .and_then(Value::as_str)
                .ok_or("path must be a string")?;
            Some(files::relative(path)?)
        }
        Operation::Move => None,
        _ if !args.is_empty() => return Err("operation takes no arguments".into()),
        _ => None,
    };
    let copy = repository::State::locked()?;
    match operation {
        Operation::Setup => copy.setup(&config).map(|()| None),
        Operation::Retire => copy.retire(&config).map(|()| None),
        Operation::Move => copy.move_copy(&config, &args).map(Some),
        _ => {
            let root = copy.root(&config)?;
            match operation {
                Operation::Read => Ok(Some(json!({"content":files::read(&root, path.unwrap())?}))),
                Operation::List => Ok(Some(json!({"entries":files::list(&root, path.unwrap())?}))),
                Operation::Write => {
                    let content = args
                        .get("content")
                        .and_then(Value::as_str)
                        .ok_or("content must be a string")?;
                    files::write(&root, path.unwrap(), content)?;
                    Ok(None)
                }
                Operation::Changes => copy.changes(&root).map(Some),
                _ => unreachable!("hooks are handled before opening the copy"),
            }
        }
    }
}

pub struct WorkingCopyServer;

impl ServerHandler for WorkingCopyServer {
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2026-07-28","serverInfo":{"name":BINARY,"version":env!("CARGO_PKG_VERSION")},"capabilities":{"tools":{},"extensions":{EXTENSION:{"revision":"v0"}}}})).unwrap()
    }

    async fn discover(&self, _: RequestContext<RoleServer>) -> Result<DiscoverResult, ErrorData> {
        Ok(DiscoverResult::from_server_info(
            self.supported_protocol_versions().into_owned(),
            self.get_info(),
        )
        .with_cache_scope(CacheScope::Private))
    }

    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult::with_all_items(
            Operation::ALL
                .into_iter()
                .map(|op| {
                    let mut tool = json!({"name":op.name(),"inputSchema":op.inputs()});
                    if let Some(output) = op.output() {
                        tool["outputSchema"] = output;
                    }
                    serde_json::from_value(tool).unwrap()
                })
                .collect(),
        ))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let meta = serde_json::to_value(context.meta).unwrap_or(Value::Null);
        if meta["io.modelcontextprotocol/clientCapabilities"]["extensions"][EXTENSION]["revision"]
            != "v0"
        {
            return Err(serde_json::from_value(json!({"code":-32021,"message":"Missing required client capability; performed nothing","data":{"requiredCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}}}})).unwrap());
        }
        let Some(filters) = meta[format!("{PREFIX}filters")].as_array() else {
            return Ok(response(Err(
                "request metadata filters must be a list".into()
            )));
        };
        if let Some(restriction) = filters.first() {
            return Ok(serde_json::from_value::<CallToolResult>(json!({"content":[{"type":"text","text":"working copy cannot apply the requested filter; performed nothing"}],"isError":true,"_meta":{REASON:"unavailable",RESTRICTION:restriction}})).unwrap().into());
        }
        let Some(operation) = Operation::ALL.into_iter().find(|op| {
            request.name == op.name() && meta[format!("{PREFIX}operation")] == op.qualified()
        }) else {
            return Ok(response(Err("unknown working-copy operation".into())));
        };
        let required = |key| {
            meta[format!("{PREFIX}{key}")]
                .as_str()
                .is_some_and(|v| !v.is_empty())
        };
        if !["instance", "invocation", "caller", "revision"]
            .into_iter()
            .all(required)
            || (operation.hook()
                && (meta.get(format!("{PREFIX}conduit")).is_some() || !required("idempotencyKey")))
            || (!operation.hook() && !required("conduit"))
        {
            return Ok(response(Err(
                "invalid keyed working-copy request metadata".into()
            )));
        }
        let config = match Config::parse(meta[format!("{PREFIX}config")].clone()) {
            Ok(config) => config,
            Err(error) => return Ok(response(Err(error))),
        };
        let args = request.arguments.unwrap_or_default();
        Ok(response(
            tokio::task::spawn_blocking(move || invoke(operation, args, config))
                .await
                .unwrap_or_else(|_| Err("working-copy operation could not finish".into())),
        ))
    }
}
