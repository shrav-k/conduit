//! Unix fixture servers for S4; never a general effect or model provider.
mod control;
pub mod ledger;
use ledger::Ledger;
use rmcp::{ErrorData, RoleServer, ServerHandler, model::*, service::RequestContext};
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::LazyLock;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

pub const BINARY: &str = "scenario-s4-effect";
pub const ADDRESS: &str = "example.org/conduit/scenario-s4-effect";
pub const EXTENSION: &str = "io.github.shrav-k.conduit/implementation-interface";
pub const PREFIX: &str = "io.github.shrav-k.conduit/";
pub const OUTCOME: &str = "io.github.shrav-k.conduit/outcome";
pub const REASON: &str = "io.github.shrav-k.conduit/reason";
pub const RESTRICTION: &str = "io.github.shrav-k.conduit/restriction";
pub static SCHEMAS: LazyLock<Value> = LazyLock::new(|| {
    let mut schemas = serde_json::Map::new();
    for doc in serde_yaml::Deserializer::from_str(include_str!("../contracts.yaml")) {
        let v = Value::deserialize(doc).expect("embedded contract must parse");
        if v["kind"] == "Operation" {
            schemas.insert(
                v["metadata"]["name"].as_str().unwrap().into(),
                v["spec"].clone(),
            );
        }
    }
    Value::Object(schemas)
});

pub struct EffectServer {
    item: String,
    mode: String,
    ledger: PathBuf,
    control: Option<PathBuf>,
    process: String,
    counter: AtomicU64,
}
fn mark<'a>(meta: &'a Value, key: &str) -> &'a Value {
    &meta[format!("{PREFIX}{key}")]
}
fn nonempty(v: &Value) -> bool {
    v.as_str().is_some_and(|s| !s.is_empty())
}
fn failed(cause: &str) -> CallToolResponse {
    serde_json::from_value::<CallToolResult>(
        json!({"content":[{"type":"text","text":cause}],"isError":true,"_meta":{OUTCOME:"failed"}}),
    )
    .unwrap()
    .into()
}
fn io_error(_: std::io::Error) -> ErrorData {
    ErrorData::internal_error(
        "S4 effect ledger I/O failed; inspect independent readback",
        None,
    )
}
fn valid_label(v: &Value) -> bool {
    v.as_str().is_some_and(|s| {
        !s.is_empty()
            && s.len() <= 63
            && s.bytes()
                .enumerate()
                .all(|(i, b)| b.is_ascii_lowercase() || b.is_ascii_digit() || i > 0 && b == b'-')
    })
}
impl EffectServer {
    pub fn from_environment(item: &str) -> Result<Self, &'static str> {
        if !["plain", "keyed", "read"].contains(&item) {
            return Err("expected plain, keyed, read or readback LEDGER");
        }
        let ledger = std::env::var_os("S4_EFFECT_LEDGER")
            .map(PathBuf::from)
            .ok_or(
                "S4_EFFECT_LEDGER must name an absolute ledger path with an existing directory",
            )?;
        if !ledger.is_absolute() || !ledger.parent().is_some_and(|p| p.is_dir()) {
            return Err(
                "S4_EFFECT_LEDGER must name an absolute ledger path with an existing directory",
            );
        }
        let control = if item == "read" {
            None
        } else {
            std::env::var_os("S4_EFFECT_CONTROL").map(PathBuf::from)
        };
        if control
            .as_ref()
            .is_some_and(|p| !p.is_absolute() || !control::validate(p))
        {
            return Err(
                "S4_EFFECT_CONTROL must name an absolute, owned directory writable by no group or other user",
            );
        }
        let mode = if item == "plain" {
            std::env::var("S4_EFFECT_MODE")
                .map_err(|_| "S4_EFFECT_MODE must be complete or partial-error")?
        } else {
            "complete".into()
        };
        if !["complete", "partial-error"].contains(&mode.as_str()) {
            return Err("S4_EFFECT_MODE must be complete or partial-error");
        }
        Ledger::create(&ledger).map_err(|_| "S4_EFFECT_LEDGER cannot be created or synced")?;
        Ok(Self {
            item: item.into(),
            mode,
            ledger,
            control,
            process: format!(
                "{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ),
            counter: AtomicU64::new(0),
        })
    }
    fn aware(&self) -> bool {
        self.item != "plain"
    }
    fn operations(&self) -> Vec<&'static str> {
        match self.item.as_str() {
            "plain" => vec!["apply-effect", "start-model-work"],
            "keyed" => vec!["apply-effect.v1"],
            _ => vec!["read-effects.v1"],
        }
    }
    fn validate_meta(&self, meta: &Value, operation: &str) -> Option<CallToolResponse> {
        for key in ["instance", "operation", "invocation", "caller", "revision"] {
            if !nonempty(mark(meta, key)) {
                return Some(failed(&format!("missing or malformed {key} metadata")));
            }
        }
        if *mark(meta, "operation") != format!("{ADDRESS}#{operation}") {
            return Some(failed("operation metadata does not match tool"));
        }
        for key in ["idempotencyKey", "conduit"] {
            if meta.get(format!("{PREFIX}{key}")).is_some() && !nonempty(mark(meta, key)) {
                return Some(failed(&format!("malformed {key} metadata")));
            }
        }
        // Configuration is checked for its required IFC-006 form but never used to configure this run.
        if !mark(meta, "config").is_object() {
            return Some(failed("missing or malformed config metadata"));
        }
        let Some(filters) = mark(meta, "filters").as_array() else {
            return Some(failed("missing or malformed filters metadata"));
        };
        if let Some(filter) = filters.first() {
            return Some(serde_json::from_value::<CallToolResult>(json!({"content":[{"type":"text","text":"unsupported filter requirement"}],"isError":true,"_meta":{REASON:"unavailable",RESTRICTION:filter}})).unwrap().into());
        }
        None
    }
    fn append(&self, call: &Value, kind: &str, extra: Value) -> Result<(), ErrorData> {
        let mut line = call.clone();
        line["kind"] = json!(kind);
        line.as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        Ledger::open(&self.ledger, true)
            .and_then(|mut l| l.append(&line))
            .map_err(io_error)
    }
    fn receive(&self, operation: &str, label: &Value, meta: &Value) -> Result<Value, ErrorData> {
        let mut call = json!({"version":1,"kind":"received","call":format!("{}-{}",self.process,self.counter.fetch_add(1,Ordering::Relaxed)),"mode":self.mode,"operation":operation,"label":label,"instance":null,"caller":null,"key":null,"invocation":null,"attempt":1});
        if self.aware() {
            for (field, key) in [
                ("instance", "instance"),
                ("caller", "caller"),
                ("key", "idempotencyKey"),
                ("invocation", "invocation"),
            ] {
                call[field] = mark(meta, key).clone();
            }
        }
        let mut ledger = Ledger::open(&self.ledger, true).map_err(io_error)?;
        let (lines, _) = ledger.lines().map_err(io_error)?;
        call["attempt"] = json!(
            1 + lines
                .iter()
                .filter(|l| l["kind"] == "received" && self.same_scope(l, &call))
                .count()
        );
        ledger.append(&call).map_err(io_error)?;
        Ok(call)
    }
    fn same_scope(&self, a: &Value, b: &Value) -> bool {
        let fields: &[&str] = if self.aware() {
            &["instance", "caller", "key", "invocation"]
        } else {
            &["operation", "label"]
        };
        fields.iter().all(|f| a[f] == b[f])
    }
    fn apply(&self, call: &Value) -> Result<String, ErrorData> {
        // One lock spans lookup and all missing parts; each part is a separate committed write.
        let mut ledger = Ledger::open(&self.ledger, true).map_err(io_error)?;
        let (lines, _) = ledger.lines().map_err(io_error)?;
        let existing = lines.iter().find(|l| {
            self.aware()
                && !call["key"].is_null()
                && l["kind"] == "entry"
                && self.same_scope(l, call)
        });
        let record = existing.map_or_else(
            || format!("{}-record", call["call"].as_str().unwrap()),
            |l| l["record"].as_str().unwrap().into(),
        );
        let parts: Vec<_> = lines
            .iter()
            .filter(|l| l["kind"] == "entry" && l["record"] == record)
            .map(|l| l["part"].clone())
            .collect();
        let mut line = call.clone();
        line["record"] = json!(record);
        if parts.contains(&json!(1)) && parts.contains(&json!(2)) {
            line["kind"] = json!("deduplicated");
            ledger.append(&line).map_err(io_error)?;
        } else {
            for part in 1..=if self.mode == "partial-error" { 1 } else { 2 } {
                if !parts.contains(&json!(part)) {
                    line["kind"] = json!("entry");
                    line["part"] = json!(part);
                    line["of"] = json!(2);
                    ledger.append(&line).map_err(io_error)?;
                }
            }
        }
        Ok(record)
    }
    async fn barrier(
        &self,
        call: &Value,
        name: &str,
        answer: &Value,
        context: &RequestContext<RoleServer>,
    ) -> Result<bool, ErrorData> {
        let base = self
            .control
            .as_ref()
            .map(|dir| dir.join(format!("{}.{}", call["call"].as_str().unwrap(), name)));
        let release = if let Some(base) = &base {
            let mut details = call.clone();
            details.as_object_mut().unwrap().remove("kind");
            control::wait(base,&json!({"version":1,"package":ADDRESS,"barrier":name,"call":call["call"],"details":details})).await
        } else {
            Ok(true)
        };
        let lost = release.is_err();
        let proceed = release.unwrap_or(false);
        if lost {
            self.append(call,"control-lost",json!({"barrier":name,"cause":"barrier file could not be written or read, or release verdict was invalid"}))?;
        }
        if !proceed || name == "after-commit" {
            let reply = if !proceed {
                "dropped"
            } else if context.ct.is_cancelled() {
                "cancelled"
            } else {
                "released"
            };
            self.append(call, "reply", json!({"reply":reply,"answer":answer}))?;
        }
        if let Some(base) = base
            && control::atomic(&control::file(&base, "acknowledged"), b"").is_err()
        {
            self.append(
                call,
                "control-lost",
                json!({"barrier":name,"cause":"barrier acknowledgement could not be written"}),
            )?;
            self.append(call, "reply", json!({"reply":"dropped","answer":answer}))?;
            return Ok(false);
        }
        Ok(proceed)
    }
}
impl ServerHandler for EffectServer {
    fn get_info(&self) -> ServerConfig {
        let mut capabilities = json!({"tools":{}});
        if self.aware() {
            capabilities["extensions"] = json!({EXTENSION:{"revision":"v0"}});
        }
        serde_json::from_value(json!({"protocolVersion":"2026-07-28","serverInfo":{"name":BINARY,"version":env!("CARGO_PKG_VERSION")},"capabilities":capabilities})).unwrap()
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
        let schemas = &*SCHEMAS;
        Ok(ListToolsResult::with_all_items(self.operations().into_iter().map(|name| {
            let op = if self.aware() {name.into()} else {format!("{name}.v1")};
            serde_json::from_value(json!({"name":name,"inputSchema":schemas[&op]["inputSchema"],"outputSchema":schemas[&op]["outputSchema"]})).unwrap()
        }).collect()))
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        if !self.operations().contains(&request.name.as_ref()) {
            return Err(ErrorData::invalid_params("unknown S4 effect tool", None));
        }
        let meta = serde_json::to_value(&context.meta).unwrap();
        if self.aware()
            && meta["io.modelcontextprotocol/clientCapabilities"]["extensions"][EXTENSION]["revision"]
                != "v0"
        {
            return Err(serde_json::from_value(json!({"code":-32021,"message":"Conduit implementation extension revision v0 is required; nothing performed","data":{"requiredCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}}}})).unwrap());
        }
        let op = if self.aware() {
            request.name.to_string()
        } else {
            format!("{}.v1", request.name)
        };
        if self.aware()
            && let Some(result) = self.validate_meta(&meta, &op)
        {
            return Ok(result);
        }
        let args = Value::Object(request.arguments.unwrap_or_default());
        let read = self.item == "read";
        if args.as_object().unwrap().keys().any(|k| k != "label")
            || (!read || args.get("label").is_some()) && !valid_label(&args["label"])
        {
            return if self.aware() {
                Ok(failed(
                    "label arguments do not match operation input schema",
                ))
            } else {
                Err(ErrorData::invalid_params(
                    "label arguments do not match operation input schema",
                    None,
                ))
            };
        }
        if read {
            let output =
                ledger::readback(&self.ledger, args["label"].as_str()).map_err(io_error)?;
            return Ok(serde_json::from_value::<CallToolResult>(json!({"content":[{"type":"text","text":"independent S4 effect ledger readback"}],"structuredContent":output,"isError":false,"_meta":{OUTCOME:"succeeded"}})).unwrap().into());
        }
        let call = self.receive(&op, &args["label"], &meta)?;
        if !self
            .barrier(&call, "before-apply", &Value::Null, &context)
            .await?
        {
            return std::future::pending().await;
        }
        let record = self.apply(&call)?;
        let answer = if self.mode == "partial-error" {
            json!({"error":{"code":-32021,"message":"partial effect committed; form-mode elicitation is required","data":{"requiredCapabilities":{"elicitation":{"form":{}}}}}})
        } else {
            let mut result = json!({"content":[{"type":"text","text":format!("record {record}")}],"structuredContent":{"record":record},"isError":false});
            if self.aware() {
                result["_meta"] = json!({OUTCOME:"succeeded"});
            }
            json!({"result":result})
        };
        if !self
            .barrier(&call, "after-commit", &answer, &context)
            .await?
        {
            return std::future::pending().await;
        }
        if let Some(error) = answer.get("error") {
            Err(serde_json::from_value(error.clone()).unwrap())
        } else {
            Ok(
                serde_json::from_value::<CallToolResult>(answer["result"].clone())
                    .unwrap()
                    .into(),
            )
        }
    }
}
