pub mod schemas;
#[cfg(unix)]
mod state;

use anyhow::{Result, bail};
use rmcp::{ErrorData, RoleServer, ServerHandler, model::*, service::RequestContext};
use serde_json::{Value, json};
use std::path::Path;

pub const BINARY: &str = "provider-local-machine";
pub const EXTENSION: &str = "io.github.shrav-k.conduit/implementation-interface";
pub const STATE_VARIABLE: &str = "CONDUIT_STATE_DIR";
pub const OUTCOME: &str = "io.github.shrav-k.conduit/outcome";
pub const KEY_FILE: &str = "machine.key";

pub fn canonical_hostname(raw: &str) -> Result<String> {
    let hostname = raw.trim().to_lowercase();
    if hostname.is_empty() || hostname.chars().count() > 63 {
        bail!("OS hostname must contain 1 to 63 characters after normalization; no truncation");
    }
    Ok(hostname)
}

fn go_arch(arch: &str, little_endian: bool) -> &str {
    match arch {
        "aarch64" if little_endian => "arm64",
        "aarch64" => "arm64be",
        "arm" if !little_endian => "armbe",
        "riscv32" => "riscv",
        "wasm32" | "wasm64" => "wasm",
        "x86_64" => "amd64",
        "x86" => "386",
        "loongarch64" => "loong64",
        "powerpc" => "ppc",
        "powerpc64" if little_endian => "ppc64le",
        "powerpc64" => "ppc64",
        "mips" if little_endian => "mipsle",
        "mips" => "mips",
        "mips64" if little_endian => "mips64le",
        "mips64" => "mips64",
        arch => arch,
    }
}

pub fn machine(id: &str) -> Result<Value> {
    let hostname = hostname::get()?;
    let hostname = canonical_hostname(
        hostname
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("OS hostname is not UTF-8"))?,
    )?;
    let os = match std::env::consts::OS {
        "macos" => "darwin",
        os => os,
    };
    let arch = go_arch(std::env::consts::ARCH, cfg!(target_endian = "little"));
    let value = json!({"id":id,"hostname":hostname,"state":"reachable","labels":{
        "kubernetes.io/hostname":hostname,"kubernetes.io/os":os,"kubernetes.io/arch":arch
    }});
    if !schemas::valid(
        "list-machines.v1",
        "outputSchema",
        &json!({"machines":[value.clone()]}),
    ) {
        bail!("OS hostname or platform labels do not satisfy the machine output schema");
    }
    Ok(value)
}

pub struct MachineServer {
    #[cfg(unix)]
    state: state::State,
}
impl MachineServer {
    pub fn new(directory: &Path) -> Result<Self> {
        #[cfg(unix)]
        {
            Ok(Self {
                state: state::State::open(directory)?,
            })
        }
        #[cfg(not(unix))]
        {
            let _ = directory;
            bail!("provider-local-machine requires Unix");
        }
    }
    fn report(&self) -> Result<Value> {
        #[cfg(unix)]
        {
            machine(&self.state.id()?)
        }
        #[cfg(not(unix))]
        {
            bail!("provider-local-machine requires Unix");
        }
    }
}
fn answer(outputs: Result<Value>) -> CallToolResponse {
    let wire = match outputs {
        Ok(outputs) => {
            json!({"content":[],"structuredContent":outputs,"isError":false,"_meta":{OUTCOME:"succeeded"}})
        }
        Err(error) => {
            json!({"content":[{"type":"text","text":error.to_string()}],"isError":true,"_meta":{OUTCOME:"failed"}})
        }
    };
    serde_json::from_value::<CallToolResult>(wire)
        .unwrap()
        .into()
}
impl ServerHandler for MachineServer {
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
        let tools = schemas::OPERATIONS.iter().map(|name| serde_json::from_value(json!({
            "name":name,"inputSchema":schemas::schema(name,"inputSchema").unwrap_or(json!({"type":"object"})),
            "outputSchema":schemas::schema(name,"outputSchema").unwrap()
        })).unwrap()).collect();
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
            return Err(serde_json::from_value(json!({"code":-32021,"message":"Missing required client capability","data":{"requiredCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}}}})).unwrap());
        }
        let args = Value::Object(request.arguments.unwrap_or_default());
        if !schemas::OPERATIONS.contains(&request.name.as_ref()) {
            return Ok(answer(Err(anyhow::anyhow!("unknown machine operation"))));
        }
        if !schemas::valid(&request.name, "inputSchema", &args) {
            return Ok(answer(Err(anyhow::anyhow!(
                "arguments do not match operation input schema"
            ))));
        }
        let result = self.report().map(|machine| {
            if request.name == "list-machines.v1" {
                json!({"machines":[machine]})
            } else if args["nodeSelector"]
                .as_object()
                .unwrap()
                .iter()
                .all(|(key, value)| machine["labels"].get(key) == Some(value))
            {
                json!({"hostname":machine["hostname"]})
            } else {
                json!({})
            }
        });
        Ok(answer(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn architecture_labels_use_go_spellings() {
        // Explicit expectations exercise other hosts without repeating the mapping logic.
        for (rust_arch, little_endian, expected) in [
            ("aarch64", true, "arm64"),
            ("aarch64", false, "arm64be"),
            ("arm", false, "armbe"),
            ("riscv32", true, "riscv"),
            ("wasm32", true, "wasm"),
            ("x86_64", true, "amd64"),
            ("x86", true, "386"),
            ("loongarch64", true, "loong64"),
            ("powerpc", false, "ppc"),
            ("powerpc64", true, "ppc64le"),
            ("powerpc64", false, "ppc64"),
            ("mips", true, "mipsle"),
            ("mips", false, "mips"),
            ("mips64", true, "mips64le"),
            ("mips64", false, "mips64"),
            ("arm", true, "arm"),
            ("riscv64", true, "riscv64"),
            ("s390x", false, "s390x"),
        ] {
            assert_eq!(go_arch(rust_arch, little_endian), expected, "{rust_arch}");
        }
    }

    #[test]
    fn hostname_is_normalized_without_truncation() {
        assert_eq!(
            canonical_hostname("  LAB-MACHINE\n").unwrap(),
            "lab-machine"
        );
        assert!(canonical_hostname(&"x".repeat(63)).is_ok());
        assert!(canonical_hostname(&"x".repeat(64)).is_err());
        assert!(canonical_hostname("  ").is_err());
    }
}
