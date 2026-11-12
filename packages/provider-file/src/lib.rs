//! Unix-only, stateless file secret store. Never format a read buffer or decoding error.
#[cfg(not(unix))]
compile_error!("provider-file requires Unix");

use rmcp::{ErrorData, RoleServer, ServerHandler, model::*, service::RequestContext};
use rustix::fs::{FileType, Mode, OFlags, fstat, open};
use serde_json::{Value, json};
use std::{fs::File, io::Read, path::Path};

pub const BINARY: &str = "provider-file";
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

fn read_secret(config: &Value) -> Result<String, &'static str> {
    let config = config
        .as_object()
        .ok_or("configuration must be an object containing only path")?;
    if config.len() != 1 || !config.contains_key("path") {
        return Err("configuration must contain path and no extra keys");
    }
    let path = config["path"]
        .as_str()
        .ok_or("configuration path must be an absolute path string")?;
    if !Path::new(path).is_absolute() || path.contains('\0') {
        return Err("configuration path must be an absolute path without NUL bytes");
    }
    // NONBLOCK prevents opening a FIFO from waiting before the descriptor is checked.
    // Parent links are allowed: do not canonicalize or compare path spellings.
    let fd = open(
        path,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .map_err(|error| match error {
        rustix::io::Errno::NOENT => "secret file is missing",
        rustix::io::Errno::ACCESS | rustix::io::Errno::PERM => {
            "secret file is unreadable: permission denied"
        }
        rustix::io::Errno::LOOP => {
            "secret file is a symbolic link or a path contains a symbolic-link loop"
        }
        _ => "secret file could not be opened for reading",
    })?;
    let stat = fstat(&fd).map_err(|_| "secret file descriptor could not be inspected")?;
    if FileType::from_raw_mode(stat.st_mode) != FileType::RegularFile {
        return Err("secret file is not a regular file");
    }
    if stat.st_mode & 0o077 != 0 {
        return Err(
            "secret file permissions allow group or other access; mode & 0077 must be zero",
        );
    }
    if stat.st_size > MAX_VALUE_BYTES as i64 {
        return Err("secret file exceeds the 65536-byte size bound");
    }
    let mut bytes = Vec::new();
    // One extra byte detects growth after fstat without an unbounded allocation/read.
    File::from(fd)
        .take((MAX_VALUE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| "secret file could not be read")?;
    if bytes.len() > MAX_VALUE_BYTES {
        return Err("secret file exceeds the 65536-byte size bound");
    }
    let mut value = String::from_utf8(bytes).map_err(|_| "secret file is not valid UTF-8")?;
    if value.ends_with('\n') {
        value.pop();
    }
    Ok(value)
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

pub struct FileServer;

impl ServerHandler for FileServer {
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
            return Ok(serde_json::from_value::<CallToolResult>(json!({"content":[{"type":"text","text":"file store cannot apply the requested filter; performed nothing"}],"isError":true,"_meta":{REASON:"unavailable",RESTRICTION:restriction}})).unwrap().into());
        }
        if request.name != "get-secret.v1" || meta[format!("{PREFIX}operation")] != OPERATION {
            return Ok(answer(Err("unknown file store operation")));
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
        Ok(answer(read_secret(&meta[format!("{PREFIX}config")])))
    }
}
