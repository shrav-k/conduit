//! S4-only setup hook and independent ledger reader, with a reusable v1 gate for S9.
pub mod barrier;
pub mod ledger;
use barrier::Verdict;
use ledger::{Call, append, new_id};
use rmcp::{ErrorData, RoleServer, ServerHandler, ServiceExt, model::*, service::RequestContext};
use serde_json::{Value, json};
use std::{
    future::pending,
    path::Path,
    sync::{Arc, LazyLock},
};

pub const BINARY: &str = "scenario-s4-hook";
pub const ADDRESS: &str = "example.org/conduit/scenario-s4-hook";
pub const CORE: &str = "github.com/shrav-k/conduit/core";
// Names from spec/names.yaml; extension from spec/definitions.md.
pub const PREFIX: &str = "io.github.shrav-k.conduit/";
pub const EXTENSION: &str = "io.github.shrav-k.conduit/implementation-interface";
pub const OUTCOME: &str = "io.github.shrav-k.conduit/outcome";
pub static SCHEMAS: LazyLock<Value> =
    LazyLock::new(|| serde_json::from_str(include_str!("schemas.json")).unwrap());
/// The gate runs synchronously after blockedBy, before the effect. It must finish its own work.
pub enum GateVerdict {
    Proceed,
    Fail(String),
}
pub type Gate = Arc<dyn Fn(&Call) -> GateVerdict + Send + Sync>;
pub struct HookServer {
    package: String,
    reader: bool,
    gate: Option<Gate>,
}
impl HookServer {
    pub fn new(package: impl Into<String>, reader: bool, gate: Option<Gate>) -> Self {
        Self {
            package: package.into(),
            reader,
            gate,
        }
    }
    fn tools(&self) -> Vec<&'static str> {
        if self.reader {
            vec!["read-hook-ledger.v1"]
        } else {
            vec!["setup-hook.v1", "set-blocked-by.v1"]
        }
    }
}
fn mark<'a>(meta: &'a Value, name: &str) -> &'a Value {
    &meta[format!("{PREFIX}{name}")]
}
fn text(value: &Value) -> bool {
    value.as_str().is_some_and(|s| !s.is_empty())
}
fn answer(state: &str, diagnostic: &str, output: Option<Value>) -> Value {
    let mut v = json!({"content":[{"type":"text","text":diagnostic}],"isError":state!="succeeded","_meta":{OUTCOME:state}});
    if let Some(output) = output {
        v["structuredContent"] = output;
    }
    v
}
fn failed(diagnostic: &str) -> Value {
    answer("failed", diagnostic, None)
}
fn absolute(v: &Value) -> bool {
    v.as_str().is_some_and(|s| Path::new(s).is_absolute())
}
fn validate(name: &str, meta: &Value, args: &Value) -> Result<Call, String> {
    for key in ["instance", "operation", "invocation", "caller", "revision"] {
        if !text(mark(meta, key)) {
            return Err(format!("missing or malformed {key}"));
        }
    }
    let address = if name == "setup-hook.v1" {
        CORE
    } else {
        ADDRESS
    };
    if *mark(meta, "operation") != format!("{address}#{name}") {
        return Err("operation does not match tool".into());
    }
    for key in ["conduit", "idempotencyKey"] {
        if (key == "idempotencyKey" && name == "setup-hook.v1"
            || meta.get(format!("{PREFIX}{key}")).is_some())
            && !text(mark(meta, key))
        {
            return Err(format!("missing or malformed {key}"));
        }
    }
    if !mark(meta, "filters").is_array() {
        return Err("missing or malformed filters".into());
    }
    let config = mark(meta, "config");
    if !config.is_object() {
        return Err("missing or malformed config".into());
    }
    if !absolute(&config["ledger"])
        || !Path::new(config["ledger"].as_str().unwrap())
            .parent()
            .is_some_and(|p| p.is_dir())
    {
        return Err("ledger must be absolute with an existing directory".into());
    }
    if config.as_object().unwrap().keys().any(|k| {
        !if name == "read-hook-ledger.v1" {
            &["ledger"][..]
        } else {
            &["ledger", "blockedBy", "control"][..]
        }
        .contains(&k.as_str())
    }) {
        return Err("config has an undeclared place".into());
    }
    if let Some(v) = config.get("blockedBy")
        && !v.is_null()
        && !absolute(v)
    {
        return Err("blockedBy must be null or absolute".into());
    }
    if let Some(v) = config.get("control")
        && !v
            .as_str()
            .is_some_and(|s| barrier::validate_directory(Path::new(s)))
    {
        return Err(
            "control must be an absolute directory owned by this user, with no group/other write"
                .into(),
        );
    }
    let map = args.as_object().ok_or("arguments must be an object")?;
    match name {
        "setup-hook.v1" if !map.is_empty() => return Err("setup-hook.v1 has no inputs".into()),
        "set-blocked-by.v1" => {
            if !text(&args["revision"])
                || args
                    .get("blockedBy")
                    .is_some_and(|v| !v.is_null() && !absolute(v))
                || map
                    .keys()
                    .any(|k| !["revision", "blockedBy"].contains(&k.as_str()))
            {
                return Err("revision is required; supplied blockedBy must be null or absolute; no other control inputs are allowed".into());
            }
        }
        "read-hook-ledger.v1"
            if map.keys().any(|k| k != "instance")
                || args.get("instance").is_some_and(|v| !v.is_string()) =>
        {
            return Err("instance must be a string".into());
        }
        _ => {}
    }
    Ok(Call {
        id: new_id(),
        instance: mark(meta, "instance").as_str().unwrap().into(),
        invocation: mark(meta, "invocation").as_str().unwrap().into(),
        caller: mark(meta, "caller").as_str().unwrap().into(),
        key: mark(meta, "idempotencyKey").as_str().map(str::to_owned),
        revision: mark(meta, "revision").as_str().unwrap().into(),
        attempt: 0,
        configuration: config.clone(),
    })
}
impl HookServer {
    async fn pause(&self, call: &Call, name: &str) -> std::io::Result<Verdict> {
        match call.configuration["control"].as_str() {
            Some(dir) => {
                barrier::wait(
                    Path::new(dir),
                    &self.package,
                    call,
                    Path::new(call.configuration["ledger"].as_str().unwrap()),
                    name,
                )
                .await
            }
            None => Ok(Verdict::Proceed),
        }
    }
    async fn execute(
        &self,
        mut call: Call,
        context: &RequestContext<RoleServer>,
    ) -> std::io::Result<Value> {
        let path = Path::new(call.configuration["ledger"].as_str().unwrap()).to_owned();
        ledger::receive(&path, &self.package, &mut call)?;
        let verdict = self.pause(&call, "before-apply").await?;
        let mut cause = "controller";
        let failure = match verdict {
            Verdict::Fail => Some("controller failed before-apply".into()),
            Verdict::Drop => Some("controller dropped before-apply".into()),
            Verdict::Proceed => {
                cause = "blockedBy";
                let blocked = call.configuration["blockedBy"].as_str();
                match blocked.map(std::fs::metadata) {
                    Some(Ok(m)) if m.is_file() => Some(format!(
                        "blockedBy: {} blocks this attempt",
                        blocked.unwrap()
                    )),
                    Some(Err(e)) if e.kind() != std::io::ErrorKind::NotFound => Some(format!(
                        "blockedBy: {} metadata could not be read",
                        blocked.unwrap()
                    )),
                    _ => {
                        cause = "gate";
                        self.gate.as_ref().and_then(|gate| match gate(&call) {
                            GateVerdict::Proceed => None,
                            GateVerdict::Fail(d) => Some(d),
                        })
                    }
                }
            }
        };
        let effect = failure.is_none().then(new_id);
        let diagnostic = match &failure {
            Some(d) => format!("attempt {}: {d}", call.attempt),
            None => format!(
                "attempt {} applied effect {}",
                call.attempt,
                effect.as_ref().unwrap()
            ),
        };
        let mut lines = Vec::new();
        if let Some(id) = &effect {
            lines.push(call.line(&self.package, "effect", json!({"effect":id})));
        }
        lines.push(call.line(&self.package,"ended",json!({"effect":effect,"cause":if failure.is_some(){Some(cause)}else{None},"diagnostic":diagnostic})));
        append(&path, &lines)?;
        let reply_verdict = if verdict == Verdict::Drop {
            Verdict::Drop
        } else {
            self.pause(&call, "before-reply").await?
        };
        let reply = if reply_verdict == Verdict::Drop {
            "dropped"
        } else if context.ct.is_cancelled() {
            "cancelled"
        } else {
            "released"
        };
        append(
            &path,
            &[call.line(&self.package, "reply", json!({"outcome":reply}))],
        )?;
        if reply != "released" {
            return pending().await;
        }
        Ok(answer(
            if effect.is_some() {
                "succeeded"
            } else {
                "failed"
            },
            &diagnostic,
            None,
        ))
    }
}
impl ServerHandler for HookServer {
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({"protocolVersion":"2026-07-28","serverInfo":{"name":self.package,"version":env!("CARGO_PKG_VERSION")},"capabilities":{"tools":{},"extensions":{EXTENSION:{"revision":"v0"}}}})).unwrap()
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
            self.tools()
                .into_iter()
                .map(|name| {
                    let mut v = json!({"name":name,"inputSchema":SCHEMAS[name]["inputSchema"]});
                    if let Some(s) = SCHEMAS[name].get("outputSchema") {
                        v["outputSchema"] = s.clone();
                    }
                    serde_json::from_value(v).unwrap()
                })
                .collect(),
        ))
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let meta = serde_json::to_value(&context.meta).unwrap();
        if meta["io.modelcontextprotocol/clientCapabilities"]["extensions"][EXTENSION]["revision"]
            != "v0"
        {
            return Err(serde_json::from_value(json!({"code":-32021,"message":"Conduit implementation extension revision v0 required","data":{"requiredCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}}}})).unwrap());
        }
        let name = request.name.as_ref();
        let args = Value::Object(request.arguments.unwrap_or_default());
        let result = if !self.tools().contains(&name) {
            failed("operation/tool is not served by this item")
        } else {
            match validate(name, &meta, &args) {
                Err(d) => failed(&d),
                Ok(call) => {
                    if let Some(filter) = mark(&meta, "filters").as_array().unwrap().first() {
                        json!({"content":[{"type":"text","text":"filters are unsupported"}],"isError":true,"_meta":{format!("{PREFIX}reason"):"unavailable",format!("{PREFIX}restriction"):filter}})
                    } else if name == "set-blocked-by.v1" {
                        answer(
                            "succeeded",
                            "blockedBy accepted; engine stores the supplied configuration value",
                            None,
                        )
                    } else if self.reader {
                        match ledger::readback(
                            Path::new(call.configuration["ledger"].as_str().unwrap()),
                            args["instance"].as_str(),
                        ) {
                            Ok(v) => answer("succeeded", "hook ledger v1", Some(v)),
                            Err(_) => failed("ledger read failed"),
                        }
                    } else {
                        match self.execute(call, &context).await {
                            Ok(v) => v,
                            Err(_) => {
                                return Err(ErrorData::internal_error(
                                    "ledger I/O failed; consult independent evidence",
                                    None,
                                ));
                            }
                        }
                    }
                }
            }
        };
        // rmcp can still receive a cancellation between this check and its transport send.
        if context.ct.is_cancelled() {
            return pending().await;
        }
        Ok(serde_json::from_value::<CallToolResult>(result)
            .unwrap()
            .into())
    }
}
/// Serve a hook over stdio. S9 passes its package name and its backing precondition gate.
pub async fn serve_hook(
    package: &str,
    gate: Option<Gate>,
) -> Result<(), Box<dyn std::error::Error>> {
    HookServer::new(package, false, gate)
        .serve((tokio::io::stdin(), tokio::io::stdout()))
        .await?
        .waiting()
        .await?;
    Ok(())
}
pub async fn serve_reader(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    HookServer::new(package, true, None)
        .serve((tokio::io::stdin(), tokio::io::stdout()))
        .await?
        .waiting()
        .await?;
    Ok(())
}
