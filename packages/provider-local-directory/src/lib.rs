//! Unix local-directory source. No cache and no archive or digest interpretation.
#[cfg(not(unix))]
compile_error!("provider-local-directory requires Unix");

pub mod schemas;

use anyhow::{Context, Result, bail, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use rmcp::{ErrorData, RoleServer, ServerHandler, model::*, service::RequestContext};
use rustix::fs::{Mode, OFlags, open, openat};
use serde_json::{Value, json};
use std::{fs::File, io::Read, os::fd::OwnedFd, path::Path};
use url::{ParseError, Url};

pub const BINARY: &str = "provider-local-directory";
pub const EXTENSION: &str = "io.github.shrav-k.conduit/implementation-interface";
pub const OUTCOME: &str = "io.github.shrav-k.conduit/outcome";
pub const CONFIG: &str = "io.github.shrav-k.conduit/config";
pub const MAX_ARCHIVE_BYTES: u64 = 8 * 1024 * 1024;

pub fn configuration_schema() -> Value {
    json!({"type":"object","properties":{
        "schemes":{"type":"array","minItems":1,"items":{"const":"file"}},
        "root":{"type":"string","pattern":"^/"}
    },"required":["schemes","root"],"additionalProperties":false})
}

fn file_url(fetch: &str) -> Result<Url> {
    // url rejects file user information at parse time. Identify it first so that
    // the diagnostic names credentials, without ever echoing the URL or value.
    if let Some((_, rest)) = fetch.split_once(':')
        && let Some(authority) = rest.strip_prefix("//")
    {
        ensure!(
            !authority
                .split(['/', '?', '#'])
                .next()
                .unwrap()
                .contains('@'),
            "fetch must not carry user information (credentials)"
        );
    }
    let url = Url::parse(fetch).map_err(|error| match error {
        ParseError::RelativeUrlWithoutBase => {
            anyhow::anyhow!("fetch must be an absolute URL; relative references are not accepted")
        }
        _ => anyhow::anyhow!("fetch is an invalid URL"),
    })?;
    ensure!(
        url.username().is_empty() && url.password().is_none(),
        "fetch must not carry user information (credentials)"
    );
    ensure!(
        url.scheme().eq_ignore_ascii_case("file"),
        "fetch scheme is not claimed: this provider claims only file"
    );
    ensure!(
        url.host_str()
            .is_none_or(|host| host.eq_ignore_ascii_case("localhost")),
        "fetch host must be empty or localhost"
    );
    Ok(url)
}

// Directory descriptors are traversal anchors, not directory-listing handles.
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "emscripten"
))]
const DIRECTORY_ACCESS: OFlags = OFlags::PATH;
#[cfg(any(
    target_vendor = "apple",
    target_os = "netbsd",
    target_os = "solaris",
    target_os = "illumos",
    target_os = "aix",
    target_os = "cygwin"
))]
const DIRECTORY_ACCESS: OFlags = OFlags::from_bits_retain(libc::O_SEARCH as _);
#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "emscripten",
    target_vendor = "apple",
    target_os = "netbsd",
    target_os = "solaris",
    target_os = "illumos",
    target_os = "aix",
    target_os = "cygwin"
)))]
compile_error!("provider-local-directory requires Unix directory handles with O_PATH or O_SEARCH");

const DIRECTORY_FLAGS: OFlags = DIRECTORY_ACCESS
    .union(OFlags::DIRECTORY)
    .union(OFlags::NOFOLLOW)
    .union(OFlags::CLOEXEC);

fn open_root(path: &Path) -> Result<OwnedFd> {
    // Walk an already canonical absolute path without following replacement
    // symlinks. Descriptor-relative reads stay anchored if directories move.
    let mut directory = open("/", DIRECTORY_FLAGS, Mode::empty())?;
    for component in path.components().skip(1) {
        directory = openat(
            &directory,
            component.as_os_str(),
            DIRECTORY_FLAGS,
            Mode::empty(),
        )?;
    }
    Ok(directory)
}

fn fetch(config: &Value, args: &Value) -> Result<Value> {
    ensure!(
        jsonschema::is_valid(&configuration_schema(), config),
        "configuration requires schemes [file] and an absolute root directory, with no extra properties"
    );
    let url = file_url(args["fetch"].as_str().context("fetch must be a string")?)?;
    let path = url
        .to_file_path()
        .map_err(|_| anyhow::anyhow!("fetch is an invalid file URL"))?;
    let root = Path::new(config["root"].as_str().unwrap())
        .canonicalize()
        .context("root directory cannot be resolved (missing or unreadable)")?;
    let mut directory = open_root(&root)
        .context("root must be an accessible directory (missing, unreadable or not searchable)")?;
    let target = path
        .canonicalize()
        .context("fetch path cannot be resolved (missing or unreadable)")?;
    let relative = target.strip_prefix(&root).map_err(|_| {
        anyhow::anyhow!("fetch path is outside root after symbolic link resolution")
    })?;
    let mut components = relative.components().peekable();
    while let Some(component) = components.next() {
        if components.peek().is_some() {
            directory = openat(
                &directory,
                component.as_os_str(),
                DIRECTORY_FLAGS,
                Mode::empty(),
            )
            .context("fetch directory is unreadable or changed during resolution")?;
        } else {
            // NONBLOCK prevents a FIFO swapped in before open from hanging a call.
            let fd = openat(
                &directory,
                component.as_os_str(),
                OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC,
                Mode::empty(),
            )
            .context("fetch file cannot be opened (unreadable or changed during resolution)")?;
            let file = File::from(fd);
            let metadata = file
                .metadata()
                .context("fetch file metadata cannot be read")?;
            ensure!(metadata.is_file(), "fetch path is not a regular file");
            ensure!(
                metadata.len() <= MAX_ARCHIVE_BYTES,
                "archive exceeds 8388608-byte (8 MiB) limit"
            );
            let mut bytes = Vec::new();
            file.take(MAX_ARCHIVE_BYTES + 1)
                .read_to_end(&mut bytes)
                .context("fetch file is unreadable")?;
            ensure!(
                bytes.len() as u64 <= MAX_ARCHIVE_BYTES,
                "archive exceeds 8388608-byte (8 MiB) limit"
            );
            return Ok(json!({"archive":STANDARD.encode(bytes)}));
        }
    }
    bail!("fetch path is not a regular file")
}

fn answer(outputs: Result<Value>) -> CallToolResponse {
    let wire = match outputs {
        Ok(outputs) => {
            json!({"content":[],"structuredContent":outputs,"isError":false,"_meta":{OUTCOME:"succeeded"}})
        }
        Err(error) => {
            json!({"content":[{"type":"text","text":format!("{error:#}")}],"isError":true,"_meta":{OUTCOME:"failed"}})
        }
    };
    serde_json::from_value::<CallToolResult>(wire)
        .unwrap()
        .into()
}

pub struct DirectoryServer;
impl ServerHandler for DirectoryServer {
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
        let tools = schemas::OPERATIONS
            .iter()
            .map(|name| {
                serde_json::from_value(json!({
                    "name":name,"inputSchema":schemas::schema(name,"inputSchema").unwrap(),
                    "outputSchema":schemas::schema(name,"outputSchema").unwrap()
                }))
                .unwrap()
            })
            .collect();
        Ok(ListToolsResult::with_all_items(tools))
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
        if request.name != "fetch-package.v1" {
            return Ok(answer(Err(anyhow::anyhow!(
                "unknown package-source operation"
            ))));
        }
        let args = Value::Object(request.arguments.unwrap_or_default());
        // Classify URL refusal before schema validation, so even malformed file
        // credential URLs get an explicit credential diagnostic, without I/O.
        if let Some(url) = args["fetch"].as_str()
            && let Err(error) = file_url(url)
        {
            return Ok(answer(Err(error)));
        }
        if !schemas::valid(&request.name, "inputSchema", &args) {
            return Ok(answer(Err(anyhow::anyhow!(
                "arguments do not match operation input schema"
            ))));
        }
        let config = meta[CONFIG].clone();
        let outputs = tokio::task::spawn_blocking(move || fetch(&config, &args))
            .await
            .unwrap_or_else(|_| Err(anyhow::anyhow!("file read worker failed")));
        Ok(answer(outputs))
    }
}
