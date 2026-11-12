pub mod answer;
pub mod form;
pub mod launch;
pub mod names;
pub mod observe;
pub mod report;
pub mod schema;
pub mod substitute;

use form::{Assertion, Case, Descriptor, Index, Step};
use launch::{Prepared, Run, digest};
use report::{CaseReport, Report, Tool, Verdict};
use serde_json::{Value, json};
use std::{collections::BTreeSet, fs, path::Path, time::Duration};
use substitute::{Context, Values};

pub struct Options {
    pub timeout: Duration,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
        }
    }
}
fn parse<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, &'static str> {
    let _: serde_yaml::Value =
        serde_yaml::from_slice(bytes).map_err(|_| "invalid YAML or duplicate mapping key")?;
    serde_yaml::from_slice(bytes).map_err(|_| "invalid YAML or unknown field")
}
fn metadata(
    d: &Descriptor,
    c: &Context<'_>,
    supplied: &Values,
    operation: &str,
) -> Result<Value, &'static str> {
    let mut m = json!({
        names::key("instance"):d.instance,
        names::key("config"):c.expand(&d.config)?,
        names::key("revision"):d.revision,
        names::key("operation"):schema::qualified(operation),
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{names::extension():{"revision":"v0"}}}
    });
    for (key, value) in supplied {
        m[names::key(key)] = c.expand(value)?;
    }
    Ok(m)
}
fn assertion(
    a: &Assertion,
    c: &Context<'_>,
    wire: &Value,
    decision: &answer::Decision,
) -> Result<(), &'static str> {
    match a {
        Assertion::Outcome { value } => {
            if decision != &answer::Decision::Outcome(*value) {
                return Err("outcome differs");
            }
        }
        Assertion::Reason { value } => {
            if decision != &answer::Decision::Reason(*value) {
                return Err("reason differs");
            }
        }
        Assertion::Equal { pointer, value } => {
            let pointer = substitute::expand_pointer(pointer)?;
            let value = c.expand(value)?;
            if wire.pointer(&pointer) != Some(&value) {
                return Err("JSON pointer equality differs");
            }
        }
        Assertion::Pattern { pointer, pattern } => {
            let pointer = substitute::expand_pointer(pointer)?;
            let Some(value) = wire.pointer(&pointer).and_then(Value::as_str) else {
                return Err("pattern pointer is absent or not a string");
            };
            if !regex::Regex::new(pattern)
                .map_err(|_| "invalid pattern")?
                .is_match(value)
            {
                return Err("JSON pointer pattern differs");
            }
        }
        Assertion::NoMarkerAnywhere {} => (),
    }
    Ok(())
}
async fn execute(
    case: &Case,
    d: &Descriptor,
    p: &Prepared,
    lab: &Path,
    options: &Options,
) -> Result<(), String> {
    let mut run = None::<Run>;
    let mut observed = Vec::new();
    let mut captures = Values::new();
    let mut global_marker = None;
    let result = async {
        for (step_index, step) in case.steps.iter().enumerate() {
            match step {
                Step::Pause { milliseconds } => {
                    tokio::time::sleep(Duration::from_millis(*milliseconds)).await;
                }
                Step::Restart {} => {
                    if let Some(mut old) = run.take() {
                        old.stop().await;
                    }
                    let next = Run::start(d, p, lab, options.timeout)
                        .await
                        .map_err(str::to_owned)?;
                    observed.push(next.observations.clone());
                    run = Some(next);
                }
                Step::Call {
                    operation,
                    arguments,
                    meta,
                    capture,
                    assertions,
                } => {
                    if run.is_none() {
                        let next = Run::start(d, p, lab, options.timeout)
                            .await
                            .map_err(str::to_owned)?;
                        observed.push(next.observations.clone());
                        run = Some(next);
                    }
                    let ctx = Context {
                        descriptor: d,
                        fixtures: &case.fixtures,
                        captures: &captures,
                        lab,
                    };
                    let meta = metadata(d, &ctx, meta, operation).map_err(str::to_owned)?;
                    let params: rmcp::model::CallToolRequestParams = serde_json::from_value(json!({
                        "name": operation,
                        "arguments": ctx.expand(arguments).map_err(str::to_owned)?,
                        "_meta": meta,
                    }))
                    .map_err(|_| "invalid call arguments".to_owned())?;
                    let service = &run.as_ref().unwrap().service;
                    let mut response = tokio::time::timeout(
                        options.timeout, async { service.call_tool_once(params.clone()).await.map(rmcp::model::ServerResult::from) }
                    ).await;
                    // IFC-046 repairs parameter headers once, only for read-only calls.
                    let repair = matches!(&response, Ok(Err(rmcp::service::ServiceError::McpError(e))) if e.code.0 == -32020)
                        && !matches!(operation.as_str(), "run.v1" | "stop.v1");
                    let revision = match &response {
                        Ok(Err(rmcp::service::ServiceError::McpError(error))) => {
                            retry_revision(error, run.as_ref().unwrap())
                        }
                        _ => None,
                    };
                    // Both repairs share one resend budget; never inspect the second error
                    // for another repair, even if it names the other failure category.
                    if repair && matches!(tokio::time::timeout(options.timeout, service.list_all_tools()).await, Ok(Ok(_))) {
                        response = tokio::time::timeout(options.timeout, async {
                            service.call_tool_once(params).await.map(rmcp::model::ServerResult::from)
                        }).await;
                    } else if let Some(revision) = revision {
                        response = tokio::time::timeout(options.timeout, async {
                            let mut meta = rmcp::model::RequestMetaObject::default();
                            meta.set_protocol_version(revision);
                            let result = service.send_request_with_option(
                                rmcp::model::ClientRequest::CallToolRequest(rmcp::model::CallToolRequest::new(params)),
                                rmcp::service::PeerRequestOptions::no_options().with_meta(meta),
                            ).await?.await_response().await?;
                            // Keep call_tool_once's accepted variants on this raw send too.
                            match result {
                                rmcp::model::ServerResult::CallToolResult(_)
                                | rmcp::model::ServerResult::InputRequiredResult(_)
                                | rmcp::model::ServerResult::CreateTaskResult(_) => Ok(result),
                                _ => Err(rmcp::service::ServiceError::UnexpectedResponse),
                            }
                        }).await;
                    }
                    let wire = match response {
                        Ok(Ok(v)) => {
                            serde_json::to_value(v)
                                .map_err(|_| "cannot read reply".to_owned())?
                        }
                        Ok(Err(rmcp::service::ServiceError::McpError(error))) => {
                            json!({"error":error})
                        }
                        _ => answer::transport_unknown(),
                    };
                    let decision = answer::read(operation, &wire, &meta[names::key("filters")]);
                    for (i, a) in assertions.iter().enumerate() {
                        if matches!(a, Assertion::NoMarkerAnywhere {}) {
                            global_marker.get_or_insert((step_index + 1, i + 1));
                        }
                        assertion(a, &ctx, &wire, &decision).map_err(|reason| {
                            format!("step {}, assertion {}: {reason}", step_index + 1, i + 1)
                        })?;
                    }
                    let mut next = Values::new();
                    for (name, pointer) in capture {
                        let pointer = substitute::expand_pointer(pointer).map_err(str::to_owned)?;
                        next.insert(
                            name.clone(),
                            wire.pointer(&pointer)
                                .cloned()
                                .ok_or("capture pointer missing")?,
                        );
                    }
                    captures.extend(next);
                }
            }
        }
        Ok(())
    }
    .await;
    if let Some(mut old) = run {
        old.stop().await;
    }
    if let Some((s, a)) = global_marker
        && observed.iter().any(|o| o.contains_marker(&case.markers))
    {
        return Err(format!(
            "step {s}, assertion {a}: marker observed or observation limit exceeded"
        ));
    }
    if observed.iter().any(|o| o.overflowed()) {
        return Err("observation limit exceeded".into());
    }
    result
}

pub async fn run_suite(
    index_path: &Path,
    descriptor_path: &Path,
    lab: &Path,
    options: &Options,
) -> Result<Report, &'static str> {
    let index_bytes = fs::read(index_path).map_err(|_| "cannot read suite index")?;
    let index: Index = parse(&index_bytes)?;
    let descriptor_bytes = fs::read(descriptor_path).map_err(|_| "cannot read descriptor")?;
    let d: Descriptor = parse(&descriptor_bytes)?;
    if d.revision.is_empty()
        || !d.config.is_object()
        || !d.type_ref.contains('#')
        || d.parameters.keys().any(|s| !form::safe_id(s))
    {
        return Err("invalid descriptor configuration, Type or parameter name");
    }
    let lab = std::path::absolute(lab).map_err(|_| "invalid lab directory")?;
    launch::private_dir(&lab)?;
    let rev = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .filter(|s| s.trim().len() == 40 && s.trim().bytes().all(|c| c.is_ascii_hexdigit()))
        .map(|s| s.trim().to_owned())
        .unwrap_or("unavailable".into());
    let descriptor_digest = digest(&descriptor_bytes);
    let mut sensitive: BTreeSet<String> = local_names().into_iter().collect();
    sensitive.insert(d.instance.clone());
    sensitive.insert(lab.to_string_lossy().into_owned());
    collect_strings(&d.config, &mut sensitive);
    collect_strings(
        &serde_json::to_value(&d.parameters).unwrap(),
        &mut sensitive,
    );
    for path in d.lab_paths.values() {
        sensitive.insert(path.clone());
        sensitive.insert(lab.join(path).to_string_lossy().into_owned());
    }
    let prepared = launch::prepare(
        &d,
        descriptor_path.parent().unwrap_or(Path::new(".")),
        &lab,
        &descriptor_digest,
    );
    let tool = tool_version(&d, &lab, options.timeout).await;
    let archive_digest = d
        .archive
        .rsplit_once('#')
        .and_then(|(path, _)| {
            fs::read(
                descriptor_path
                    .parent()
                    .unwrap_or(Path::new("."))
                    .join(path),
            )
            .ok()
        })
        .map(|b| digest(&b));
    let mut report = Report {
        repository_revision: rev,
        contract: index.contract,
        suite_digest: digest(&index_bytes),
        archive_digest,
        descriptor_digest,
        parameter_names: d.parameters.keys().cloned().collect(),
        tool: tool.as_ref().ok().map(|v| Tool {
            name: v.name.clone(),
            version: v.version.clone(),
        }),
        cases: Vec::new(),
        totals: Default::default(),
    };
    let mut ids = BTreeSet::new();
    for (n, path) in index.cases.iter().enumerate() {
        if !substitute::relative(path) {
            return Err("case path must stay in suite");
        }
        let base = index_path.parent().unwrap_or(Path::new("."));
        let canonical = fs::canonicalize(base.join(path)).map_err(|_| "cannot read case")?;
        if !canonical.starts_with(fs::canonicalize(base).map_err(|_| "cannot read suite")?) {
            return Err("case path escapes suite");
        }
        let bytes = fs::read(canonical).map_err(|_| "cannot read case")?;
        // Gather across the entire suite, including invalid and untested cases, before
        // redacting any report field. A later case can identify an earlier field's value.
        if let Ok(raw) = serde_yaml::from_slice::<Value>(&bytes) {
            collect_strings(&raw["markers"], &mut sensitive);
            collect_strings(&raw["fixtures"], &mut sensitive);
            if let Some(steps) = raw["steps"].as_array() {
                for step in steps {
                    if step["step"] != "call" {
                        continue;
                    }
                    for field in ["identity", "secrets"] {
                        collect_literals(&step["arguments"][field], &mut sensitive);
                    }
                    if step["operation"] == "get-secret.v1"
                        && let Some(assertions) = step["assertions"].as_array()
                    {
                        for assertion in assertions {
                            if assertion["assert"] == "equal" {
                                collect_literals(&assertion["value"], &mut sensitive);
                            }
                        }
                    }
                }
            }
        }
        let case: Result<Case, _> = parse(&bytes);
        let id = case
            .as_ref()
            .ok()
            .filter(|c| form::safe_id(&c.id))
            .map(|c| c.id.clone())
            .unwrap_or(format!("invalid-case-{}", n + 1));
        let mut cell = CaseReport {
            id,
            digest: digest(&bytes),
            verdict: Verdict::Failed,
            detail: None,
        };
        let result = async {
            let case = case?;
            case.validate()?;
            if case.contract != index.contract || !ids.insert(case.id.clone()) {
                return Err("contract mismatch or duplicate case id");
            }
            if case.features.iter().any(|f| !d.features.contains(f)) {
                cell.verdict = Verdict::Untested;
                cell.detail = Some("required tool feature absent".into());
                return Ok(());
            }
            let p = prepared.as_ref().map_err(|e| *e)?;
            if p.item.get("package").is_some() {
                cell.verdict = Verdict::Untested;
                cell.detail = Some("registry package item is outside H1".into());
                return Ok(());
            }
            if p.item.get("remote").is_some() {
                cell.verdict = Verdict::Untested;
                cell.detail = Some("remote item is outside H1 stdio launcher".into());
                return Ok(());
            }
            tool.as_ref().map_err(|e| *e)?;
            let result = execute(&case, &d, p, &lab, options).await;
            cell.verdict = if result.is_ok() {
                Verdict::Passed
            } else {
                Verdict::Failed
            };
            cell.detail = result.err();
            Ok::<_, &'static str>(())
        }
        .await;
        if let Err(e) = result {
            cell.detail = Some(e.into());
        }
        report.cases.push(cell);
    }
    report.finish();
    report.redact(&sensitive);
    Ok(report)
}
async fn tool_version(d: &Descriptor, lab: &Path, limit: Duration) -> Result<Tool, &'static str> {
    let (program, args) = d
        .tool_version_command
        .split_first()
        .ok_or("tool version command missing")?;
    let mut command = tokio::process::Command::new(program);
    command
        .args(args)
        .env_clear()
        .current_dir(lab)
        .kill_on_drop(true);
    for key in ["PATH", "HOME", "USER", "LOGNAME", "SHELL", "TERM"]
        .into_iter()
        .chain(d.pass_environment.iter().map(String::as_str))
    {
        if let Ok(v) = std::env::var(key)
            && !v.starts_with("()")
        {
            command.env(key, v);
        }
    }
    let output = tokio::time::timeout(limit, command.output())
        .await
        .map_err(|_| "tool version timed out")?
        .map_err(|_| "tool version command failed")?;
    if !output.status.success() {
        return Err("tool version command failed");
    }
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Version {
        name: String,
        version: String,
    }
    let v: Version = serde_json::from_slice(&output.stdout)
        .map_err(|_| "tool version command must print name/version JSON")?;
    if !form::safe_id(&v.name)
        || !regex::Regex::new(r"^[0-9]+\.[0-9]+(?:\.[0-9]+)?(?:[-+][A-Za-z0-9.-]+)?$")
            .unwrap()
            .is_match(&v.version)
    {
        return Err("tool name or version format invalid");
    }
    Ok(Tool {
        name: v.name,
        version: v.version,
    })
}
fn collect_strings(v: &Value, sensitive: &mut BTreeSet<String>) {
    match v {
        Value::String(value) if !value.is_empty() => {
            sensitive.insert(value.clone());
        }
        Value::Array(a) => a.iter().for_each(|v| collect_strings(v, sensitive)),
        Value::Object(m) => {
            if let Some(labels) = m.get("labels").and_then(Value::as_object) {
                sensitive.extend(labels.keys().filter(|k| !k.starts_with('$')).cloned());
            }
            m.values().for_each(|v| collect_strings(v, sensitive));
        }
        _ => (),
    }
}

// Placeholder names are references, not sensitive literal values.
fn collect_literals(v: &Value, sensitive: &mut BTreeSet<String>) {
    match v {
        Value::Object(m) if m.keys().any(|k| k.starts_with('$')) => (),
        Value::Object(m) => m.values().for_each(|v| collect_literals(v, sensitive)),
        Value::Array(a) => a.iter().for_each(|v| collect_literals(v, sensitive)),
        Value::String(s) if !s.is_empty() => {
            sensitive.insert(s.clone());
        }
        _ => (),
    }
}

// A present but malformed supported list grants no retry; discovery is the
// fallback only when data.supported is absent.
fn retry_revision(
    error: &rmcp::model::ErrorData,
    run: &Run,
) -> Option<rmcp::model::ProtocolVersion> {
    use rmcp::model::{ErrorCode, ProtocolVersion};
    if error.code != ErrorCode::UNSUPPORTED_PROTOCOL_VERSION {
        return None;
    }
    let rejected = run.service.peer_info()?.protocol_version.clone();
    let advertised = error
        .data
        .as_ref()
        .and_then(|d| d.get("supported"))
        .unwrap_or(&run.supported_versions);
    let supported = advertised
        .as_array()?
        .iter()
        .map(Value::as_str)
        .collect::<Option<Vec<_>>>()?;
    ProtocolVersion::KNOWN_VERSIONS
        .iter()
        .rev()
        .find(|v| **v != rejected && supported.contains(&v.as_str()))
        .cloned()
}

fn local_names() -> Vec<String> {
    let mut result: Vec<String> = ["USER", "LOGNAME"]
        .iter()
        .filter_map(|k| std::env::var(k).ok())
        .filter(|v| !v.is_empty())
        .collect();
    #[cfg(unix)]
    {
        let mut hostname = [0u8; 256];
        // Read only to suppress the host's own name from report metadata.
        if unsafe { libc::gethostname(hostname.as_mut_ptr().cast(), hostname.len()) } == 0 {
            let end = hostname
                .iter()
                .position(|b| *b == 0)
                .unwrap_or(hostname.len());
            if let Ok(name) = std::str::from_utf8(&hostname[..end])
                && !name.is_empty()
            {
                result.push(name.to_owned());
            }
        }
    }
    result
}
