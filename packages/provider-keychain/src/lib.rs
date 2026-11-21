//! Stateless secret store backed by an explicitly selected macOS keychain.
use rmcp::{ErrorData, RoleServer, ServerHandler, model::*, service::RequestContext};
use serde_json::{Value, json};
use std::path::Path;

#[cfg(target_os = "macos")]
mod keychain;

pub const BINARY: &str = "provider-keychain";
pub const EXTENSION: &str = "io.github.shrav-k.conduit/implementation-interface";
pub const PREFIX: &str = "io.github.shrav-k.conduit/";
pub const OPERATION: &str = "github.com/shrav-k/conduit/core#get-secret.v1";
pub const OUTCOME: &str = "io.github.shrav-k.conduit/outcome";
pub const REASON: &str = "io.github.shrav-k.conduit/reason";
pub const RESTRICTION: &str = "io.github.shrav-k.conduit/restriction";
pub const MAX_VALUE_BYTES: usize = 65536;

pub fn output_schema() -> Value {
    serde_json::from_str(include_str!("output-schema.json")).unwrap()
}

async fn read_secret(config: &Value) -> Result<String, &'static str> {
    const CONFIG: &str = "keychain store could not be read: configuration must contain only keychain, service and account";
    let config = config.as_object().ok_or(CONFIG)?;
    if config.len() != 3
        || !["keychain", "service", "account"]
            .iter()
            .all(|key| config.contains_key(*key))
    {
        return Err(CONFIG);
    }
    let path = config["keychain"]
        .as_str()
        .filter(|path| Path::new(path).is_absolute() && !path.contains('\0'))
        .ok_or("keychain store could not be read: keychain must be an absolute path")?;
    let service = config["service"]
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or("keychain store could not be read: service must be a non-empty string")?;
    let account = config["account"]
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or("keychain store could not be read: account must be a non-empty string")?;
    read_keychain(Path::new(path), service, account).await
}

#[cfg(target_os = "macos")]
use keychain::read as read_keychain;

#[cfg(not(target_os = "macos"))]
async fn read_keychain(_: &Path, _: &str, _: &str) -> Result<String, &'static str> {
    Err("keychain store could not be read: the store runs only on macOS")
}

fn answer(result: Result<String, &'static str>) -> CallToolResponse {
    let wire = match result {
        Ok(value) => {
            json!({"content":[],"structuredContent":{"value":value},"isError":false,"_meta":{OUTCOME:"succeeded"}})
        }
        Err(message) => {
            json!({"content":[{"type":"text","text":message}],"isError":true,"_meta":{OUTCOME:"failed"}})
        }
    };
    serde_json::from_value::<CallToolResult>(wire)
        .unwrap()
        .into()
}

pub struct KeychainServer;

impl ServerHandler for KeychainServer {
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
        Ok(ListToolsResult::with_all_items(vec![serde_json::from_value(json!({"name":"get-secret.v1","inputSchema":{"type":"object"},"outputSchema":output_schema()})).unwrap()]))
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
            return Ok(answer(Err("request metadata filters must be a list")));
        };
        if let Some(restriction) = filters.first() {
            return Ok(serde_json::from_value::<CallToolResult>(json!({"content":[{"type":"text","text":"keychain store cannot apply the requested filter; performed nothing"}],"isError":true,"_meta":{REASON:"unavailable",RESTRICTION:restriction}})).unwrap().into());
        }
        if request.name != "get-secret.v1" || meta[format!("{PREFIX}operation")] != OPERATION {
            return Ok(answer(Err("unknown keychain store operation")));
        }
        for key in ["instance", "invocation", "caller", "revision", "conduit"] {
            if !meta[format!("{PREFIX}{key}")]
                .as_str()
                .is_some_and(|value| !value.is_empty())
            {
                return Ok(answer(Err(
                    "request metadata requires non-empty instance, operation, invocation, caller, revision and conduit",
                )));
            }
        }
        Ok(answer(read_secret(&meta[format!("{PREFIX}config")]).await))
    }
}
