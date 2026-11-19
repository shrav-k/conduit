//! Invocation state is durable core state, separate from records. Intents are only plans:
//! the embedding shell persists the transition before carrying out any call (DL2).
use crate::{
    Allocation, Context, Evidence, Result, State, Transition, access, decision, explain, packages,
    pending, schemas, sign_in,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Completion {
    Result {
        result: Value,
    },
    ProtocolError {
        code: i64,
        #[serde(default)]
        data: Value,
    },
    Missing,
}
fn mark(c: &Context, key: &str) -> String {
    format!("{}{key}", c.meta_prefix)
}
fn read_only(spec: &Value) -> bool {
    spec.get("effect").is_none() && spec.get("changes").is_none() && spec["startsModelWork"] != true
}

// Missing marks, including attempts saved by an older engine, prove no fence.
fn fenced(v: &Value) -> bool {
    v["attempt"].as_u64().is_some_and(|last| {
        last > 0
            && (1..=last).all(|attempt| v["attempt_fences"][attempt.to_string()]["fenced"] == true)
    })
}

fn start_attempt(v: &mut Value, previous: Option<&Value>) {
    v["attempt_fences"] = previous
        .and_then(|old| old.get("attempt_fences"))
        .cloned()
        .unwrap_or(json!({}));
    let attempt = v["attempt"].as_u64().unwrap().to_string();
    let item = &v["implementation"]["item"];
    let builtin = v["implementation"]["source"] != "engine"
        && (item.get("command").is_some() || item.get("package").is_some())
        && v["held_target"].get("runtime").is_none();
    v["attempt_fences"][attempt] = json!({"fenced":false,"builtin_run":builtin});
}

fn fence_attempt(v: &mut Value, attempt: u64) {
    let key = attempt.to_string();
    if !v["attempt_fences"].is_object() {
        v["attempt_fences"] = json!({});
    }
    if !v["attempt_fences"][&key].is_object() {
        v["attempt_fences"][&key] = json!({});
    }
    v["attempt_fences"][key]["fenced"] = json!(true);
}

fn answering(completion: &Completion) -> bool {
    match completion {
        Completion::Missing => false,
        Completion::ProtocolError { .. } => true,
        Completion::Result { result } => response_ends_call(result),
    }
}

fn response_ends_call(result: &Value) -> bool {
    result
        .get("resultType")
        .is_none_or(|kind| kind == "complete")
}

pub(crate) fn builtin_run_stopped(t: &mut Transition, object: &str) {
    for v in &mut t.state.invocations {
        if v["target"] != object {
            continue;
        }
        if let Some(attempts) = v["attempt_fences"].as_object_mut() {
            for mark in attempts.values_mut() {
                if mark["builtin_run"] == true {
                    mark["fenced"] = json!(true);
                }
            }
        }
    }
    t.decisions.push(decision(
        "fence-stopped-builtin-run",
        &["OPS-014"],
        &[],
        json!({"object":object}),
        vec![],
    ));
}
// Required extensions are nested under the protocol's `extensions` capability map.
// An unidentified error does not establish that nothing was performed, even for an aware server.
fn capability_error(data: &Value, c: &Context) -> Option<bool> {
    let caps = data["requiredCapabilities"].as_object()?;
    if caps.is_empty() || !caps.values().all(Value::is_object) {
        return None;
    }
    let extensions = caps.get("extensions").and_then(Value::as_object);
    if let Some(extensions) = extensions
        && (!extensions.values().all(Value::is_object) || extensions.is_empty() && caps.len() == 1)
    {
        return None;
    }
    Some(extensions.is_some_and(|ext| ext.contains_key(&mark(c, "implementation-interface"))))
}
#[allow(clippy::too_many_arguments)]
fn envelope(
    c: &Context,
    outcome: Option<&str>,
    reason: Option<&str>,
    invocation: Option<&Value>,
    cause: Option<&[Value]>,
    outputs: Option<Value>,
    content: Value,
    restriction: Option<&Value>,
) -> Value {
    let status = outcome.or(reason).expect("ending has one status");
    let error = outcome != Some("succeeded");
    let mut meta = json!({});
    meta[mark(
        c,
        if outcome.is_some() {
            "outcome"
        } else {
            "reason"
        },
    )] = json!(status);
    if let Some(id) = invocation {
        meta[mark(c, "invocation")] = id.clone();
    }
    if let Some(r) = restriction {
        meta[mark(c, "restriction")] = r.clone();
    }
    let mut blocks = vec![];
    if error {
        blocks.push(json!({"type":"text"}));
    }
    blocks.extend(content.as_array().into_iter().flatten().cloned());
    let mut reply = json!({"_meta":meta,"isError":error,"content":blocks});
    if let Some(cause) = cause {
        reply["structuredContent"] = json!({"diagnostics":cause});
    } else if !error && let Some(outputs) = outputs {
        reply["structuredContent"] = outputs;
    }
    reply
}

pub(crate) fn ended_reply(c: &Context, old: &Value, contract: &Value, outcome: &str) -> Value {
    let cause = match outcome {
        "failed" | "unknown" => old["cause"].as_array().map(Vec::as_slice),
        _ => None,
    };
    envelope(
        c,
        Some(outcome),
        None,
        Some(&old["id"]),
        cause,
        contract["spec"]
            .get("outputSchema")
            .map(|_| old["retained"].get("outputs").cloned().unwrap_or(json!({}))),
        old["retained"].get("content").cloned().unwrap_or(json!([])),
        None,
    )
}

// Record content follows the held Type, rather than the retention of the repeat
// index. Redaction is schema-directed: arbitrary input member names are not secrets.
struct RecordInputs<'a> {
    env: &'a schemas::Environment,
    state: &'a crate::State,
    c: &'a Context,
    excluded: Vec<Value>,
    referenced: Vec<Value>,
}
impl RecordInputs<'_> {
    #[allow(clippy::too_many_arguments)]
    fn redact(
        &mut self,
        scope: &str,
        root: &Value,
        schema_at: &str,
        value: &Value,
        at: &str,
        result: &mut Value,
        depth: usize,
    ) -> Result<()> {
        if depth > 128 {
            return Err("record schema nesting exceeds 128".into());
        }
        let schema = root.pointer(schema_at).ok_or("missing record schema")?;
        if let Some(r) = schema["$ref"].as_str() {
            let core_reference = r.strip_prefix(&format!("{}#", self.c.core_package));
            if let Some(reference_kind) = core_reference.filter(|n| n.ends_with("reference.v1")) {
                let kind = match reference_kind {
                    "agent-reference.v1" => Some("Agent"),
                    "interface-reference.v1" => Some("Interface"),
                    "resource-reference.v1" | "secret-reference.v1" => Some("Resource"),
                    "pool-reference.v1" => Some("Pool"),
                    "fleet-reference.v1" => Some("Fleet"),
                    "conduit-reference.v1" => Some("Conduit"),
                    _ => None,
                };
                let reference = match (kind, value.as_str()) {
                    (Some(kind), Some(name)) => json!({"kind":kind,"name":name}),
                    _ => value.clone(),
                };
                if let Some(object) = access::object(self.state, &reference)
                    && !self.referenced.contains(&object["id"])
                {
                    self.referenced.push(object["id"].clone());
                }
            }

            if r == self.c.qualified("secret-reference.v1") {
                // Only a resolved Resource reference may reach a record (SEC-004).
                if value["kind"] == "Resource"
                    && value["id"].is_string()
                    && value.as_object().is_some_and(|m| m.len() == 2)
                    && access::object(self.state, value).is_some_and(|o| o["kind"] == "Resource")
                {
                    *result = json!({"kind":"Resource","id":value["id"]});
                } else {
                    *result = Value::Null;
                    self.excluded.push(json!({"path":at,"ruleId":"SEC-004","message":"Secret reference did not resolve; its value was omitted."}));
                }
                return Ok(());
            }
            if r == "#" || r.starts_with("#/") {
                // schema_reference also decodes URI escapes; keep the original root
                // so local references inside the selected fragment retain their scope.
                let (_, sub) = self
                    .env
                    .schema_reference(scope, root, r, &self.c.core_package)?;
                // Give the selected fragment the original definitions for local refs.
                let mut selected = root.clone();
                selected["$record"] = sub;
                self.redact(scope, &selected, "/$record", value, at, result, depth + 1)?;
            } else {
                let (scope, sub) =
                    self.env
                        .schema_reference(scope, root, r, &self.c.core_package)?;
                self.redact(&scope, &sub, "", value, at, result, depth + 1)?;
            }
        }
        for key in ["allOf", "anyOf", "oneOf"] {
            for (i, _) in schema[key].as_array().into_iter().flatten().enumerate() {
                let branch = format!("{schema_at}/{key}/{i}");
                if key == "allOf"
                    || self
                        .env
                        .valid_at(scope, root, &branch, value, &self.c.core_package)?
                {
                    self.redact(scope, root, &branch, value, at, result, depth + 1)?;
                }
            }
        }
        if schema.get("if").is_some() {
            let branch = if self.env.valid_at(
                scope,
                root,
                &format!("{schema_at}/if"),
                value,
                &self.c.core_package,
            )? {
                "then"
            } else {
                "else"
            };
            if schema.get(branch).is_some() {
                self.redact(
                    scope,
                    root,
                    &format!("{schema_at}/{branch}"),
                    value,
                    at,
                    result,
                    depth + 1,
                )?;
            }
        }
        if let (Some(original), Some(map)) = (value.as_object(), result.as_object_mut()) {
            for (key, v) in original {
                let token = key.replace('~', "~0").replace('/', "~1");
                let path = format!("{at}/{token}");
                let mut applicable = vec![];
                if schema["properties"].get(key).is_some() {
                    applicable.push(format!("{schema_at}/properties/{token}"));
                }
                for pattern in schema["patternProperties"]
                    .as_object()
                    .into_iter()
                    .flat_map(|m| m.keys())
                {
                    if conduit_loader::formats::ecma_regex(pattern)?
                        .find(key)
                        .is_some()
                    {
                        applicable.push(format!(
                            "{schema_at}/patternProperties/{}",
                            pattern.replace('~', "~0").replace('/', "~1")
                        ));
                    }
                }
                if applicable.is_empty() && schema.get("additionalProperties").is_some() {
                    applicable.push(format!("{schema_at}/additionalProperties"));
                }
                if let Some(current) = map.get_mut(key) {
                    let exclusions = self.excluded.len();
                    for sub in applicable {
                        self.redact(scope, root, &sub, v, &path, current, depth + 1)?;
                    }
                    if self.excluded[exclusions..]
                        .iter()
                        .any(|e| e["path"] == path)
                    {
                        map.remove(key);
                    }
                }
            }
            for key in schema["dependentSchemas"]
                .as_object()
                .into_iter()
                .flat_map(|m| m.keys())
            {
                if original.contains_key(key) {
                    self.redact(
                        scope,
                        root,
                        &format!(
                            "{schema_at}/dependentSchemas/{}",
                            key.replace('~', "~0").replace('/', "~1")
                        ),
                        value,
                        at,
                        result,
                        depth + 1,
                    )?;
                }
            }
        }
        if let (Some(original), Some(array)) = (value.as_array(), result.as_array_mut()) {
            for (i, (v, current)) in original.iter().zip(array).enumerate() {
                let prefix_len = schema["prefixItems"].as_array().map_or(0, Vec::len);
                let sub = if i < prefix_len {
                    Some(format!("{schema_at}/prefixItems/{i}"))
                } else {
                    schema.get("items").map(|_| format!("{schema_at}/items"))
                };
                if let Some(sub) = sub {
                    self.redact(
                        scope,
                        root,
                        &sub,
                        v,
                        &format!("{at}/{i}"),
                        current,
                        depth + 1,
                    )?;
                }
                if schema.get("contains").is_some()
                    && self.env.valid_at(
                        scope,
                        root,
                        &format!("{schema_at}/contains"),
                        v,
                        &self.c.core_package,
                    )?
                {
                    self.redact(
                        scope,
                        root,
                        &format!("{schema_at}/contains"),
                        v,
                        &format!("{at}/{i}"),
                        current,
                        depth + 1,
                    )?;
                }
            }
        }
        Ok(())
    }
}
fn record_content(t: &Transition, c: &Context, data: &mut Value) -> Result<()> {
    let invocation = t
        .state
        .invocations
        .iter()
        .find(|v| v["id"] == data["invocation"]);
    if let Some(v) = invocation {
        for (dest, source) in [
            ("idempotencyKey", "key"),
            ("target", "target"),
            ("caller", "caller"),
            ("consumer", "sender"),
            ("conduit", "conduit"),
            ("operation", "operation"),
            ("serving", "serving"),
            ("object", "object"),
            ("requiredCapabilities", "requiredCapabilities"),
            ("extensionRevision", "extensionRevision"),
        ] {
            if data.get(dest).is_none_or(Value::is_null)
                && let Some(value) = v.get(source)
            {
                data[dest] = value.clone();
            }
        }
    }
    let arguments = data
        .as_object_mut()
        .unwrap()
        .remove("arguments")
        .filter(|v| !v.is_null())
        .or_else(|| invocation.map(|v| v["arguments"].clone()));
    let outputs = data
        .as_object_mut()
        .unwrap()
        .remove("outputs")
        .filter(|v| !v.is_null())
        .or_else(|| {
            (data["outcome"] == "succeeded")
                .then(|| {
                    invocation
                        .and_then(|v| v.get("retained"))
                        .and_then(|v| v.get("outputs"))
                        .cloned()
                })
                .flatten()
        });
    let target = if let Some(v) = invocation {
        v["held_target"].clone()
    } else if let Some(o) = access::object(&t.state, &data["target"]) {
        crate::registration::publication_object(o, c)?
    } else {
        return Ok(());
    };
    let Some(ty) = target["type"].as_str() else {
        return Ok(());
    };
    let (_, _, ty) = schemas::operation(&target, ty, &c.core_package)?;
    let Some(op) = data["operation"].as_str().map(str::to_owned) else {
        return Ok(());
    };
    let (env, scope, contract) = schemas::operation(&target, &op, &c.core_package)?;
    for (value, setting, schema_key) in [
        (arguments, "inputs", "inputSchema"),
        (outputs, "outputs", "outputSchema"),
    ] {
        if ty["spec"]["records"][setting] != true
            && !(setting == "inputs" && op == c.qualified("lift.v1"))
        {
            continue;
        }
        if setting == "outputs" && op == c.qualified("get-secret.v1") {
            continue;
        }
        if let Some(mut value) = value {
            // Pending inspection evidence is scoped to the reply, including its repeats.
            // Shared records keep the rest of the inspection output.
            if setting == "outputs" && op == c.qualified("inspect.v1") {
                if let Some(status) = value.get_mut("status").and_then(Value::as_object_mut) {
                    status.remove("pending");
                }
                // Keep answer evidence in its standalone record, even when the Type
                // keeps the rest of the inspection output.
                if let Some(records) = value.get_mut("records").and_then(Value::as_array_mut) {
                    records.retain(|r| r["type"] != "pending-action-answered");
                }
            }
            if setting == "outputs"
                && op == c.qualified("list.v1")
                && let Some(outputs) = value.as_object_mut()
            {
                outputs.remove("pending");
            }
            if op == c.qualified("run.v1") && setting == "inputs" {
                value
                    .as_object_mut()
                    .ok_or("invalid run inputs")?
                    .remove("secrets");
                value.as_object_mut().unwrap().remove("identity");
            }
            if let Some(schema) = contract["spec"].get(schema_key) {
                let mut redactor = RecordInputs {
                    env: &env,
                    state: &t.state,
                    c,
                    excluded: vec![],
                    referenced: vec![],
                };
                let original = value.clone();
                redactor.redact(&scope, schema, "", &original, "", &mut value, 0)?;
                if !redactor.excluded.is_empty() {
                    data[format!("{setting}Excluded")] = json!(redactor.excluded);
                }
            }
            data[setting] = value;
        }
    }
    Ok(())
}

fn referenced_objects(
    state: &crate::State,
    target: &Value,
    op: &str,
    arguments: &Value,
    c: &Context,
) -> Result<Vec<Value>> {
    let (env, scope, contract) = schemas::operation(target, op, &c.core_package)?;
    let mut walk = RecordInputs {
        env: &env,
        state,
        c,
        excluded: vec![],
        referenced: vec![],
    };
    if let Some(schema) = contract["spec"].get("inputSchema") {
        walk.redact(&scope, schema, "", arguments, "", &mut arguments.clone(), 0)?;
    }
    Ok(walk.referenced)
}

pub(crate) fn record(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    event: &str,
    mut data: Value,
) -> Result<()> {
    // PRV-005 / REC-002: the enclosing invocation supplies correlation only.
    // A provider call has its own target and inputs, and no request route.
    if event == "provider-call" {
        let enclosing = data.as_object_mut().unwrap().remove("invocation");
        record_content(t, c, &mut data)?;
        if let Some(id) = enclosing {
            data["invocation"] = id;
        }
        for field in [
            "consumer",
            "conduit",
            "serving",
            "idempotencyKey",
            "filters",
        ] {
            data.as_object_mut().unwrap().remove(field);
        }
    } else if event != "pending-action-answered" {
        // The answer event carries its own defined data. Enrich other request records.
        record_content(t, c, &mut data)?;
    }
    if let Some(fields) = data.as_object_mut() {
        fields.retain(|_, v| !v.is_null());
    }
    t.records.push(a.record(&mut t.state, c, event, data)?);
    Ok(())
}
#[allow(clippy::too_many_arguments)]
fn end(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    route: &Value,
    outcome: Option<&str>,
    reason: Option<&str>,
    id: Option<&Value>,
) -> Result<()> {
    let mut data = if route.is_object() {
        route.clone()
    } else {
        json!({})
    };
    // Inputs and outputs are not unconditionally recorded (REC-002, SEC-004).
    data.as_object_mut().unwrap().remove("filters");
    if let Some(s) = outcome {
        data["outcome"] = json!(s);
    }
    if let Some(s) = reason {
        data["reason"] = json!(s);
    }
    if let Some(id) = id {
        data["invocation"] = id.clone();
    }
    record(t, a, c, "request-ended", data)
}
#[allow(clippy::too_many_arguments)]
pub(crate) fn reject(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    route: &Value,
    reason: &str,
    rule: &str,
    id: Option<&Value>,
    restriction: Option<&Value>,
) -> Result<()> {
    reject_diagnostics(t, a, c, route, reason, rule, id, restriction, None)
}

fn rejection_route(route: &Value) -> Cow<'_, Value> {
    if route.get("operation").is_some() {
        return Cow::Borrowed(route);
    }
    let mut explanation = route.clone();
    explanation["operation"] = route["tool"].clone();
    Cow::Owned(explanation)
}

fn reject_preflight(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    route: &Value,
    rejection: crate::explain::Rejection,
) -> Result<()> {
    reject_diagnostics(
        t,
        a,
        c,
        route,
        rejection.reason,
        &rejection.rule,
        None,
        None,
        rejection.diagnostics,
    )
}

#[allow(clippy::too_many_arguments)]
fn reject_diagnostics(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    route: &Value,
    reason: &str,
    rule: &str,
    id: Option<&Value>,
    restriction: Option<&Value>,
    diagnostics: Option<Value>,
) -> Result<()> {
    t.reply = envelope(
        c,
        None,
        Some(reason),
        id,
        None,
        None,
        json!([]),
        restriction,
    );
    let subject = route["tool"]
        .as_str()
        .or_else(|| route["operation"].as_str())
        .ok_or("rejection lacks subject")?;
    let specific = match rule {
        "PKG-023" => Some("install fetch requires an absolute local file URL; https fetching and package-source providers are unfinished".into()),
        "LIF-037" => {
            let args = &route["arguments"];
            let reference = args
                .get("object")
                .or_else(|| args.get("instance"))
                .unwrap_or(&args["conduit"]);
            let conduit = reference
                .as_str()
                .map(|name| json!({"kind":"Conduit","name":name}));
            access::object(&t.state, conduit.as_ref().unwrap_or(reference)).map(|o| {
                format!(
                    "expected ID {}, resolved ID {}",
                    args["expectedId"], o["id"]
                )
            })
        }
        _ => None,
    };
    let explanation = rejection_route(route);
    let diagnostic = if let Some(diagnostics) = &diagnostics {
        diagnostics
            .as_array()
            .and_then(|ds| ds.first())
            .cloned()
            .ok_or("specific rejection diagnostics are empty")?
    } else if let Some(message) = specific {
        json!({"ruleId":rule,"level":"error","message":{"text":message}})
    } else {
        crate::explain::diagnostic(&t.state, &explanation, c, reason, rule)?
    };
    t.reply["structuredContent"] =
        json!({"diagnostics":diagnostics.unwrap_or_else(|| json!([diagnostic]))});
    if rule == "OPS-029"
        && let Some(named) = named_reference(&route["operation"], &route["arguments"], c)
    {
        t.reply["structuredContent"]["diagnostics"][0]["message"]["text"] = json!(format!(
            "{named} does not exist. A closed Conduit and a removed instance no longer exist."
        ));
    }
    if rule == "ACC-001" || rule == "TPL-013" {
        let diagnostics = t.reply["structuredContent"]["diagnostics"].clone();
        let message = diagnostics[0]["message"]["text"]
            .as_str()
            .ok_or("diagnostic has no message")?;
        t.reply = json!({"error":{"code":-32602,"message":format!("No tool named {subject} is presented to this instance. {message}"),"data":{mark(c,"reason"):reason,"diagnostics":diagnostics}}});
    }
    if rule == "IFC-049" {
        t.reply = json!({"error":{
            "code":-32021,"message":"Missing required client capability",
            "data":{
                "requiredCapabilities":{"elicitation":{"url":{}}},
                mark(c,"reason"):reason,
                "diagnostics":t.reply["structuredContent"]["diagnostics"]
            }
        }});
    }
    if rule == "LIF-037" {
        let args = &route["arguments"];
        let reference = args
            .get("object")
            .or_else(|| args.get("instance"))
            .unwrap_or(&args["conduit"]);
        let conduit = reference
            .as_str()
            .map(|name| json!({"kind":"Conduit","name":name}));
        if let Some(o) = access::object(&t.state, conduit.as_ref().unwrap_or(reference)) {
            t.reply["structuredContent"]["diagnostics"] = json!([{"ruleId":rule,"level":"error","message":{"text":format!("expected ID {}, resolved ID {}", args["expectedId"],o["id"])},"properties":{"expectedId":args["expectedId"],"actualId":o["id"]}}]);
        }
    }
    t.decisions.push(decision(
        "end-with-reason",
        &[rule, "OPS-005", "OPS-023", "IFC-025"],
        &[],
        json!({"reason":reason}),
        vec![],
    ));
    let mut ending = route.clone();
    ending["ruleId"] = json!(rule);
    if reason == "refused" {
        ending["refusalTier"] = json!(crate::explain::tier(&t.state, &explanation, c)?);
    }
    end(t, a, c, &ending, None, Some(reason), id)
}

pub(crate) fn respond(t: &mut Transition, invocation: &str, attempt: u64, c: &Context) {
    let message = format!(
        "invocation {invocation} has no input request outstanding, so the retry was not passed on; no request in this build asks for input yet (order 048a-5e)"
    );
    t.decisions.push(decision(
        "refuse-input-response",
        &["IFC-049"],
        &[],
        json!({"invocation":invocation,"attempt":attempt}),
        vec![],
    ));
    t.reply = json!({
        "error": {
            "code": -32602,
            "message": message,
            "data": {
                mark(c, "reason"): "invalid",
                "diagnostics": [{"ruleId":"IFC-049","level":"error","message":{"text":message}}]
            }
        }
    });
}

fn named_reference(operation: &Value, arguments: &Value, c: &Context) -> Option<String> {
    let operation = operation.as_str()?;
    let core_op = operation.strip_prefix(&format!("{}#", c.core_package))?;
    let (key, fixed_kind) = match core_op {
        "inspect.v1" | "unregister.v1" => ("object", "Object"),
        "replace.v1" => ("template", "Object"),
        "set-up.v1" | "retire.v1" | "register-replacement.v1" => ("instance", "Object"),
        "sign-in.v1" => ("instance", "Object"),
        "open.v1" | "close.v1" => ("conduit", "Conduit"),
        "declare-lost.v1" => ("node", "Resource"),
        "reconcile.v1" => ("invocation", "Invocation"),
        _ => return None,
    };
    let reference = &arguments[key];
    let kind = reference["kind"].as_str().unwrap_or(fixed_kind);
    let name = reference
        .as_str()
        .or_else(|| reference["name"].as_str())
        .or_else(|| reference["metadata"]["name"].as_str())
        .or_else(|| reference["id"].as_str())?;
    let mut label = format!("{kind}/{name}");
    if let Some(entry) = reference["entry"].as_str() {
        label.push_str(&format!(" (entry {entry})"));
    }
    Some(label)
}
/// The call for a request's or a hook's dispatch. IFC-045: it declares URL elicitation only toward
/// an instance whose Type states signIn, whatever its account holds.
fn intent(v: &Value, state: &State, c: &Context) -> Result<Value> {
    let url_elicitation = match access::object_by_id(state, &v["target"]) {
        Some(object) => !matches!(
            sign_in::expected_account(object, c)?,
            sign_in::ExpectedAccount::NoSignIn
        ),
        None => false,
    };
    Ok(dispatch_intent(v, url_elicitation, c))
}

fn dispatch_intent(v: &Value, url_elicitation: bool, c: &Context) -> Value {
    let implementation = &v["implementation"];
    let operation = v["operation"].as_str().unwrap_or("");
    let local = operation
        .rsplit_once('#')
        .map(|(_, n)| n)
        .unwrap_or(operation);
    let tool = if v["plain"] == true {
        implementation["item"]["tools"]
            .get(operation)
            .or_else(|| implementation["item"]["tools"].get(format!("#{local}")))
            .cloned()
            .unwrap_or(json!(local))
    } else {
        json!(local)
    };
    let mut call = json!({"method":"tools/call","params":{"name":tool,"arguments":v["arguments"]}});
    if v["plain"] != true {
        let mut meta = json!({});
        for (key, value) in [
            ("instance", &v["target"]),
            ("operation", &v["operation"]),
            ("invocation", &v["id"]),
            ("caller", &v["caller"]),
            ("filters", &v["filters"]),
            ("config", &v["config"]),
            ("revision", &v["revision"]),
        ] {
            meta[mark(c, key)] = value.clone();
        }
        if v["hook"] != true {
            meta[mark(c, "conduit")] = v["conduit"].clone();
        }
        if !v["key"].is_null() {
            meta[mark(c, "idempotencyKey")] = v["key"].clone();
        }
        call["params"]["_meta"] = meta;
    }
    // IFC-045: URL elicitation is declared only toward an instance whose Type states signIn.
    if url_elicitation {
        call["params"]["_meta"]["io.modelcontextprotocol/clientCapabilities"] =
            json!({"elicitation": {"url": {}}});
    }
    json!({"kind":"dispatch","invocation":v["id"],"attempt":v["attempt"],"target":v["target"],"implementation":implementation,"call":call})
}

/// The retry of a part whose implementation answered with its own InputRequiredResult
/// (MCP 2026-07-28): the part's first call as the core wrote it, unchanged but for
/// `params.requestState`, that implementation's state, echoed exactly, and
/// `params.inputResponses`, keyed as its input requests were. The runner sends it as written.
/// The core keeps that state with the part and never relays, records or shows it.
/// IFC-049: the requestState the shell relays to an Interface is another value.
#[allow(dead_code)] // Orders 048a-5d and 048a-7 call it.
pub(crate) fn retry_intent(
    first: &Value,
    request_state: &str,
    responses: &serde_json::Map<String, Value>,
) -> Value {
    let mut retry = first.clone();
    retry["call"]["params"]["requestState"] = json!(request_state);
    retry["call"]["params"]["inputResponses"] = Value::Object(responses.clone());
    retry
}

pub(crate) fn invoke(
    t: &mut Transition,
    evidence: &Evidence,
    request: &access::AccessRequest,
    key: Option<String>,
    c: &Context,
) -> Result<()> {
    let mut a = Allocation {
        evidence,
        cursor: 0,
    };
    invoke_part(t, &mut a, request, key, c, None)
}

pub(crate) fn invoke_archive(
    t: &mut Transition,
    evidence: &Evidence,
    request: &access::AccessRequest,
    key: Option<String>,
    archive: Option<crate::archives::ArchiveInput>,
    c: &Context,
) -> Result<()> {
    let mut a = Allocation {
        evidence,
        cursor: 0,
    };
    invoke_inner(t, &mut a, request, key, c, None, archive)
}

pub(crate) fn invoke_part(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    request: &access::AccessRequest,
    key: Option<String>,
    c: &Context,
    parent: Option<&Value>,
) -> Result<()> {
    invoke_inner(t, a, request, key, c, parent, None)
}

#[allow(clippy::too_many_arguments)]
fn invoke_inner(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    request: &access::AccessRequest,
    key: Option<String>,
    c: &Context,
    parent: Option<&Value>,
    archive: Option<crate::archives::ArchiveInput>,
) -> Result<()> {
    if key.as_ref().is_some_and(String::is_empty) {
        return Err("empty idempotency key at boundary".into());
    }
    if !request.readable && access::established(&t.state, &request.identity).is_err() {
        t.reply = envelope(c, None, Some("invalid"), None, None, None, json!([]), None);
        t.reply["structuredContent"] = json!({"diagnostics":[crate::explain::diagnostic(&t.state, &crate::explain::access_route(&t.state, request, c), c, "invalid", "IFC-036")?]});
        t.decisions.push(decision(
            "unreadable-unidentified-request",
            &["IFC-036", "REC-001", "OPS-023"],
            &[],
            json!({}),
            vec![],
        ));
        return Ok(());
    }
    let v = access::evaluate(
        &t.state,
        &access::PublicationView::new(&t.state, c),
        request,
        c,
        true,
    )?;
    let mut route = v.route;
    let explanation = if v.reason == Some("refused") {
        crate::explain::access_route(&t.state, request, c)
    } else {
        // ACC-037: an unavailable access state must reveal no named-object facts.
        json!({"consumer":access::established(&t.state, &request.identity).ok().map(|o| &o["id"]),"tool":request.tool,"operation":request.tool,"arguments":request.arguments})
    };
    if !route.is_object() {
        route = json!({});
    }
    for key in ["consumer", "conduit", "operation", "arguments"] {
        if route.get(key).is_none()
            && !explanation[key].is_null()
            && (key != "operation"
                || explanation[key]
                    .as_str()
                    .is_some_and(|op| op != request.tool))
        {
            route[key] = explanation[key].clone();
        }
    }
    route["tool"] = json!(request.tool);
    // OPS-029 follows authorization. Preserve the caller's spelling for its diagnostic
    // even when access evaluation has no resolved object from which to build a route.
    if v.rules.contains(&"OPS-029") {
        if !route.is_object() {
            route = json!({});
        }
        route["arguments"] = request.arguments.clone();
    }
    if let Some(parent) = parent
        && route.is_object()
    {
        route["caller"] = parent["caller"].clone();
        route["serving"] = parent["id"].clone();
    }
    // A readable rejection still has the caller's inputs when the selected Type
    // retains them. Accepted inputs already contain normalized defaults.
    if route.get("arguments").is_none()
        && route.get("target").is_some()
        && request.arguments.is_object()
    {
        route["arguments"] = request.arguments.clone();
    }
    if parent.is_none()
        && let Some(serving) = &request.serving
    {
        route["serving"] = json!(serving);
    }
    let mut access_data = route.clone();
    if !access_data.is_object() {
        access_data = json!({});
    }
    access_data.as_object_mut().unwrap().remove("filters");
    access_data["tool"] = json!(request.tool);
    access_data["permitted"] = json!(v.reason.is_none());
    if let Some(reason) = v.reason {
        access_data["reason"] = json!(reason);
        access_data["ruleId"] = json!(if v.rules[0] == "SCH-001" {
            "OPS-033"
        } else {
            v.rules[0]
        });
        access_data["dispatched"] = json!(false);
    }
    if access_data.get("caller").is_none()
        && let Ok(instance) = access::established(&t.state, &request.identity)
    {
        access_data["consumer"] = instance["id"].clone();
        if request.serving.is_none() {
            access_data["caller"] = instance["id"].clone();
        } else if let Some(parent) = request.serving.as_ref().and_then(|s| {
            t.state
                .invocations
                .iter()
                .find(|v| v["id"] == *s && v["target"] == instance["id"])
        }) {
            access_data["caller"] = parent["caller"].clone();
        }
    }
    if access_data.get("caller").is_none() {
        access_data["identityStatus"] = json!(if matches!(
            request.identity,
            access::Identity::Unreadable
        ) {
            "unreadable"
        } else {
            "not-established"
        });
    }
    if let Some(key) = &key {
        access_data["idempotencyKey"] = json!(key);
    }
    // An undispatched ending has no invocation to enrich it: carry its identity
    // and supplied key directly from this request (REC-002).
    if !route.is_object() {
        route = json!({});
    }
    for field in ["caller", "consumer", "identityStatus", "idempotencyKey"] {
        if let Some(value) = access_data.get(field) {
            route[field] = value.clone();
        }
    }
    if v.reason == Some("refused") {
        access_data["refusalTier"] =
            json!(crate::explain::tier(&t.state, &rejection_route(&route), c)?);
    }
    record(t, a, c, "access-decision", access_data)?;
    let access_record_id = t
        .records
        .last()
        .and_then(|record| record["id"].as_str())
        .ok_or("access record has no ID")?
        .to_owned();
    let mut access_rules = v.rules.clone();
    access_rules.extend(["REC-001", "REC-002", "SEC-004"]);
    t.decisions.push(decision(
        "request-access",
        &access_rules,
        &[],
        json!({"permitted":v.reason.is_none()}),
        vec![],
    ));
    if let Some(reason) = v.reason {
        let rule = if v.rules[0] == "SCH-001" {
            "OPS-033"
        } else {
            v.rules[0]
        };
        return reject(t, a, c, &route, reason, rule, None, v.restriction.as_ref());
    }
    if request.sign_in_return.is_some() && route["operation"] != c.qualified("sign-in.v1") {
        let target = access::object(&t.state, &route["target"]).ok_or("missing target")?;
        let kind = target["kind"].as_str().ok_or("target has no kind")?;
        let name = target["name"].as_str().ok_or("target has no name")?;
        let tool = route["tool"].as_str().ok_or("route has no tool")?;
        let key = mark(c, "signInReturn");
        let text = format!(
            "{tool} reports invalid under PND-013 for {kind}/{name}: {key} is a return only on a retry of sign-in.v1 whose requestState IFC-049 accepts, so this request carries none; its value was not read, kept or passed on."
        );
        t.decisions.push(decision(
            "refuse-sign-in-return",
            &["PND-013", "PND-014", "OPS-023"],
            &[],
            json!({}),
            vec![],
        ));
        return reject_diagnostics(
            t,
            a,
            c,
            &route,
            "invalid",
            "PND-013",
            None,
            None,
            Some(json!([{"ruleId":"PND-013","level":"error","message":{"text":text}}])),
        );
    }
    let target = crate::registration::publication_object(
        access::object(&t.state, &route["target"]).ok_or("missing target")?,
        c,
    )?;
    let op = route["operation"].as_str().ok_or("missing operation")?;
    let (_, _, contract) = schemas::operation(&target, op, &c.core_package)?;
    // LIF-031 and IFC-049 apply to each sign-in request, including keyed repeats.
    if op == c.qualified("sign-in.v1")
        && (route["arguments"]["instance"].get("id").is_some()
            || !sign_in::declares_url_elicitation(request))
        && let Some(rejection) = sign_in::preflight(&t.state, &route, request, &a.evidence.time, c)?
    {
        return reject_preflight(t, a, c, &route, rejection);
    }
    // OPS-023: an unsupported answer is invalid even if the key conflicts.
    // Defer PND-003 conflict so a matching repeat can return its retained outcome.
    let answer = op == c.qualified("answer.v1");
    let answer_conflict = if answer {
        match crate::records::preflight(&t.state, &route, c, a)? {
            Some(rejection) if rejection.reason == "invalid" => {
                return reject_preflight(t, a, c, &route, rejection);
            }
            rejection => rejection,
        }
    } else {
        None
    };
    let digest = packages::bytes_digest(
        &serde_json_canonicalizer::to_vec(&json!({"operation":op,"arguments":route["arguments"]}))
            .map_err(|e| e.to_string())?,
    );
    let existing = key.as_ref().and_then(|key| {
        t.state.invocations.iter().position(|i| {
            i["key"] == *key
                && i["sender"] == route["consumer"]
                && i["caller"] == route["caller"]
                && i["target"] == route["target"]
        })
    });
    let previous = existing.map(|i| t.state.invocations[i].clone());
    let mut redispatch = false;
    if let Some(old) = &previous {
        let id = &old["id"];
        if old["digest"] == digest {
            link_access(t, id);
        }
        let conflict = if old["digest"] != digest {
            Some("OPS-010")
        } else if old["state"] == "in-progress" {
            Some("OPS-032")
        } else {
            None
        };
        if let Some(rule) = conflict {
            return reject(
                t,
                a,
                c,
                &route,
                "conflict",
                rule,
                if rule == "OPS-010" { None } else { Some(id) },
                None,
            );
        }
        let filtered = route["filters"].as_array().is_some_and(|f| !f.is_empty());
        let readonly = read_only(&contract["spec"]);
        redispatch = old["state"] == "failed"
            || old["state"] == "unknown"
                && old["plain"] != true
                && contract["spec"]["deduplicates"] == true
            || old["state"] == "succeeded" && filtered && readonly;
        if old["state"] == "succeeded"
            && !redispatch
            && old["needs_result"] == true
            && (filtered || old.get("retained").is_none())
        {
            return reject(t, a, c, &route, "conflict", "OPS-034", Some(id), None);
        }
    }
    if (previous.is_none() || redispatch)
        && let Some((reason, rule)) = crate::lifecycle::preflight(&t.state, &route, c)?
    {
        reject(t, a, c, &route, reason, rule, None, None)?;
        if rule == "LIF-041"
            && let Some(o) =
                access::fixed_object(&t.state, &route["arguments"]["conduit"], "Conduit")
            && let Some(stone) = crate::inventory::matching(&t.state, o)
        {
            t.reply["structuredContent"]["diagnostics"] = json!([crate::inventory::diagnostic(
                &t.state,
                &route["caller"],
                stone,
                c
            )]);
        }
        if let Some(message) = crate::retirement::blocking_diagnostic(&t.state, &route, rule) {
            t.reply["structuredContent"]["diagnostics"][0]["message"]["text"] = json!(message);
        }
        return Ok(());
    }
    if previous.is_none() || redispatch {
        let rejection = if answer {
            answer_conflict
        } else {
            crate::records::preflight(&t.state, &route, c, a)?
        };
        if let Some(rejection) = rejection {
            return reject_preflight(t, a, c, &route, rejection);
        }
    }
    if (previous.is_none() || redispatch)
        && let Some(rejection) = sign_in::preflight(&t.state, &route, request, &a.evidence.time, c)?
    {
        return reject_preflight(t, a, c, &route, rejection);
    }
    // OPS-023: after authority/schema, decide reconciliation's invalid/conflict
    // before ordinary unavailability. A kept repeat performs no new reconciliation.
    let reconcile = op == c.qualified("reconcile.v1");
    let reconciled = if reconcile && (previous.is_none() || redispatch) {
        let i = t
            .state
            .invocations
            .iter()
            .position(|v| v["id"] == route["arguments"]["invocation"]);
        let Some(i) = i else {
            return reject(t, a, c, &route, "invalid", "OPS-029", None, None);
        };
        // REV-010/OPS-031 validate whether this reconciliation stores anything.
        if let Some((reason, rule)) =
            crate::revisions::preflight(&t.state, &t.state.invocations[i], &route["arguments"], c)?
        {
            return reject(t, a, c, &route, reason, rule, None, None);
        }
        if t.state.invocations[i]["state"] != "unknown" {
            return reject(t, a, c, &route, "conflict", "OPS-030", None, None);
        }
        if route["arguments"]["occurred"] == false && !fenced(&t.state.invocations[i]) {
            reject(t, a, c, &route, "conflict", "OPS-030", None, None)?;
            let message = format!(
                concat!(
                    "reconcile.v1 cannot record invocation {} as failed: it is not yet fenced ",
                    "(OPS-030). Every attempt needs an answer ending its call, an observed end ",
                    "of its run, or the implementation's confirmation that it can no longer ",
                    "act (OPS-014). Missing responses, unconfirmed cancellation and recovery ",
                    "establish no fence. Recording that the effect occurred needs no fence."
                ),
                route["arguments"]["invocation"]
                    .as_str()
                    .ok_or("invocation ID is not a string")?
            );
            t.reply["structuredContent"]["diagnostics"][0]["message"]["text"] = json!(&message);
            return Ok(());
        }
        let old = &t.state.invocations[i];
        if ["set-up.v1", "retire.v1", "open.v1", "close.v1"]
            .iter()
            .any(|op| old["operation"] == c.qualified(op))
            && crate::lifecycle::busy(&t.state, old.get("object").unwrap_or(&old["target"]), None)
        {
            return reject(t, a, c, &route, "conflict", "REV-014", None, None);
        }
        Some(i)
    } else {
        None
    };
    // OPS-023 puts ordinary unavailability after conflicts, including the repeat index.
    let available = access::evaluate(
        &t.state,
        &access::PublicationView::new(&t.state, c),
        request,
        c,
        false,
    )?;
    if let Some(reason) = available.reason {
        return reject(
            t,
            a,
            c,
            &route,
            reason,
            available.rules[0],
            None,
            available.restriction.as_ref(),
        );
    }
    if let Some(old) = &previous
        && !redispatch
    {
        let outcome = old["state"].as_str().ok_or("invalid invocation state")?;
        t.reply = ended_reply(c, old, &contract, outcome);
        t.decisions.push(decision(
            "answer-repeat",
            &[
                "OPS-008", "OPS-009", "OPS-012", "OPS-013", "OPS-018", "IFC-025", "ACC-036",
            ],
            &["DL10"],
            json!({"invocation":old["id"]}),
            vec![],
        ));
        return end(t, a, c, &route, Some(outcome), None, Some(&old["id"]));
    }
    let held_for_sign_in = sign_in::request_is_held(&t.state, &target, &contract);
    let held_for_undo = sign_in::request_is_held_for_sign_in_undone(&t.state, &target["id"]);
    if held_for_sign_in || held_for_undo {
        let tool = route["tool"]
            .as_str()
            .ok_or("held request has no tool")?
            .to_owned();
        let mut diagnostics = Vec::new();
        if held_for_sign_in {
            let mut facts = json!({"instance":target["id"]});
            if let sign_in::ExpectedAccount::Set { pointer, account } =
                sign_in::expected_account(&target, c)?
            {
                if let Some(action) = pending::raise_sign_in(
                    &mut t.state,
                    &target,
                    account,
                    &pointer,
                    pending::SignInRequest::Held {
                        access_record_id: &access_record_id,
                        operation: op,
                    },
                    a,
                )? {
                    t.decisions.push(decision(
                        "raise-sign-in-action",
                        &["PND-001", "PND-005", "PND-012"],
                        &["DL19"],
                        json!({"action":action["id"], "instance":target["id"]}),
                        vec![],
                    ));
                }
                if let Some(action) =
                    pending::unanswered_sign_in(&t.state, &target, &a.evidence.time)
                {
                    facts["action"] = action["id"].clone();
                    route["pendingAction"] = json!([action["id"]]);
                }
            }
            t.decisions.push(decision(
                "hold-for-sign-in",
                &["PND-005", "IFC-025", "OPS-023"],
                &[],
                facts,
                vec![],
            ));
            diagnostics.push(json!({
            "ruleId": "PND-005",
            "level": "error",
            "message": {"text": format!(
                "{tool} reports unavailable under PND-005: the instance awaits sign-in to its expected account, so the request was not dispatched; send it again once sign-in.v1 has signed the instance in."
            )}
        }));
        }
        if held_for_undo {
            t.decisions.push(decision(
                "hold-for-sign-in-undone",
                &["PND-011", "OPS-023"],
                &[],
                json!({"instance":target["id"]}),
                vec![],
            ));
            diagnostics.push(json!({
                "ruleId":"PND-011","level":"error","message":{"text":format!(
                    "{tool} reports unavailable under PND-011: SignInUndone is False for the instance, since a sign-out after an account check did not succeed, so the request was not dispatched; send it again once an account check within sign-in.v1 or setup finds the expected account or none."
                )}
            }));
        }
        reject_diagnostics(
            t,
            a,
            c,
            &route,
            "unavailable",
            if held_for_sign_in {
                "PND-005"
            } else {
                "PND-011"
            },
            None,
            None,
            Some(json!(diagnostics)),
        )?;
        if let Some(ids) = route.get("pendingAction") {
            t.reply["_meta"][mark(c, "pendingAction")] = ids.clone();
        }
        return Ok(());
    }
    if op == c.qualified("register-replacement.v1") {
        let parent = access::object(&t.state, &route["arguments"]["instance"])
            .ok_or("replacement parent absent")?
            .clone();
        // DL6 / LIF-041: include tombstones so the trial rejects the same replacements
        // as dispatch, without copying records, invocations or package file contents.
        let mut trial = Transition {
            state: State {
                objects: t.state.objects.clone(),
                allocated_ids: t.state.allocated_ids.clone(),
                tombstones: t.state.tombstones.clone(),
                installed_packages: t
                    .state
                    .installed_packages
                    .iter()
                    .map(|p| crate::archives::InstalledPackage {
                        package: p.package.clone(),
                        files: Default::default(),
                    })
                    .collect(),
                preinstalled_snapshots: t.state.preinstalled_snapshots.clone(),
                ..State::default()
            },
            decisions: vec![],
            records: vec![],
            external_intents: vec![],
            reply: Value::Null,
            request_replies: vec![],
            input_required: vec![],
        };
        let mut allocation = Allocation {
            evidence: a.evidence,
            cursor: a.cursor,
        };
        let result = crate::registration::replacement(
            &mut trial,
            &mut allocation,
            c,
            &parent,
            &route["arguments"]["nested"],
        )?;
        if let Some(rule) = result["rule"].as_str() {
            reject_diagnostics(
                t,
                a,
                c,
                &route,
                if rule == "LIF-041" {
                    "conflict"
                } else {
                    "invalid"
                },
                rule,
                None,
                None,
                result["message"].as_str().map(|message| {
                    json!([
                        {"ruleId":rule,"level":"error","message":{"text":message}}
                    ])
                }),
            )?;
            if rule == "LIF-041" {
                t.reply["structuredContent"]["diagnostics"] =
                    json!([crate::inventory::diagnostic(
                        &t.state,
                        &route["caller"],
                        &result["tombstone"],
                        c
                    )]);
            }
            return Ok(());
        }
    }
    let installation = if op == c.qualified("install.v1") {
        let args = &route["arguments"];
        let fetch = args["fetch"].as_str().and_then(|s| url::Url::parse(s).ok());
        let installed = packages::catalog_entries(&t.state, t.state.preinstalled_snapshots.clone())
            .iter()
            .any(|p| {
                p.address == args["address"]
                    && packages::version_equal(&p.version, args["version"].as_str().unwrap_or(""))
            });
        if args.get("fetch").is_some()
            && fetch
                .as_ref()
                .is_none_or(|u| !u.username().is_empty() || u.password().is_some())
            || !installed
                && fetch
                    .as_ref()
                    .is_none_or(|u| u.scheme() != "file" || u.to_file_path().is_err())
        {
            reject(t, a, c, &route, "invalid", "PKG-023", None, None)?;
            return Ok(());
        }
        let input = archive.unwrap_or(crate::archives::ArchiveInput {
            name: args["fetch"].as_str().unwrap_or("install.v1").into(),
            address: args["address"].as_str().unwrap().into(),
            version: args["version"].as_str().unwrap().into(),
            digest: args["digest"].as_str().unwrap().into(),
            directory: None,
            completion: crate::archives::FetchCompletion::Unavailable,
        });
        if input.address != args["address"]
            || input.version != args["version"]
            || input.digest != args["digest"]
        {
            return Err("installation evidence does not match the request".into());
        }
        // DL6: the trial reads objects, allocator IDs and catalog metadata only.
        let mut trial = Transition {
            state: State {
                objects: t.state.objects.clone(),
                installed_packages: t
                    .state
                    .installed_packages
                    .iter()
                    .map(|p| crate::archives::InstalledPackage {
                        package: p.package.clone(),
                        files: Default::default(),
                    })
                    .collect(),
                ..State::default()
            },
            decisions: t.decisions.clone(),
            records: vec![],
            external_intents: vec![],
            reply: Value::Null,
            request_replies: vec![],
            input_required: vec![],
        };
        let rejection = crate::archives::install(
            &mut trial,
            input,
            t.state.preinstalled_snapshots.clone(),
            c.builtin.clone().ok_or("missing built-in project")?,
            c,
        )?;
        if let Some(rejection) = rejection {
            t.decisions
                .extend(trial.decisions.into_iter().skip(t.decisions.len()));
            // These generic archive reasons must name the invoking tool. Specific
            // archive and loader faults retain their original diagnostics.
            let diagnostics =
                if matches!(rejection.rule.as_str(), "PKG-024" | "PKG-025" | "PKG-029") {
                    None
                } else {
                    Some(trial.reply["diagnostics"].clone())
                };
            reject_diagnostics(
                t,
                a,
                c,
                &route,
                rejection.reason,
                &rejection.rule,
                None,
                None,
                diagnostics,
            )?;
            return Ok(());
        }
        Some(trial)
    } else {
        None
    };
    let id = match &previous {
        Some(p) => p["id"].clone(),
        None => json!(a.next(&mut t.state)?),
    };
    let implementation = target["resolved_implementations"][op].clone();
    if implementation.is_null() {
        return Err("missing selected implementation".into());
    }
    if implementation["source"] == "engine"
        && !reconcile
        && !crate::lifecycle::handles(op, c)
        && !crate::records::handles(op, c)
        && op != c.qualified("install.v1")
        && op != c.qualified("sign-in.v1")
    {
        return Err("engine operation belongs to its lifecycle or records transition".into());
    }
    let plain = implementation["item"]["plain"] == true;
    let mut invocation = json!({"id":id,"key":key,"sender":route["consumer"],"caller":route["caller"],"target":route["target"],"conduit":route["conduit"],"operation":op,"arguments":route["arguments"],"digest":digest,"filters":route["filters"],"state":"in-progress","plain":plain,"contract":contract,"held_target":target,"implementation":implementation,"needs_result":plain || contract["spec"].get("outputSchema").is_some(),"attempt":previous.as_ref().and_then(|v|v["attempt"].as_u64()).unwrap_or(0)+1,"wire_retried":false,"config":target["config"],"revision":target["revision"]});
    // Keep the caller's spelling for outcome diagnostics. A cascade names its container
    // through the tool. Resolved target metadata and arguments stay private.
    let tool = &request.tool;
    let subject =
        if let Some(reference) = named_reference(&route["operation"], &request.arguments, c) {
            format!("{reference} via tool {tool}")
        } else if crate::cascade::handles(op, c) {
            format!("the container named by tool {tool}")
        } else {
            format!("tool {tool}")
        };
    invocation["named_subject"] = json!(subject);
    start_attempt(&mut invocation, previous.as_ref());
    if ["open.v1", "close.v1"]
        .iter()
        .any(|name| *op == c.qualified(name))
        && let Some(subject) =
            access::fixed_object(&t.state, &route["arguments"]["conduit"], "Conduit")
    {
        let mut held = json!({"object":{"kind":"Conduit"},"target":crate::inventory::endpoint(&t.state,&subject["target"]),"operations":subject["operations"]});
        if let Some(b) = access::bound(subject, c) {
            held["bound"] = b.clone();
        }
        invocation["access_subject"] = held;
    }
    let related = referenced_objects(&t.state, &target, op, &route["arguments"], c)?;
    if !related.is_empty() {
        invocation["input_objects"] = json!(related);
    }
    if let Some(serving) = &request.serving {
        invocation["serving"] = json!(serving);
    }
    if let Some(parent) = parent {
        invocation["sender"] = parent["sender"].clone();
        invocation["setup_parent"] = parent["id"].clone();
        invocation["serving"] = parent["id"].clone();
        invocation["caller"] = parent["caller"].clone();
        invocation["filters"] = parent["filters"].clone();
    }
    if op == c.qualified("open.v1") {
        invocation["opening_provenance"] = crate::inventory::provenance(&t.state, &invocation, c)?;
    }
    if let Some(old) = previous {
        if let Some(notices) = old.get("notices") {
            invocation["notices"] = notices.clone();
        }
        invocation["previous"] = old;
    }
    let retried = if op == c.qualified("set-up.v1") {
        let last = t
            .state
            .invocations
            .iter()
            .filter_map(|v| v["dispatched"].as_u64())
            .max()
            .unwrap_or(0);
        let dispatched = last
            .checked_add(1)
            .ok_or("setup dispatch order exhausted")?;
        invocation["dispatched"] = json!(dispatched);
        // The instance lifecycle sets up, at a redispatch too: a reused name names the replacement.
        access::object(&t.state, &route["arguments"]["instance"])
            .cloned()
            .and_then(|subject| {
                pending::answer_before_change(
                    &mut t.state,
                    &subject,
                    &a.evidence.time,
                    "retried",
                    &invocation,
                    c,
                )
            })
    } else {
        None
    };
    let i = if let Some(i) = existing {
        t.state.invocations[i] = invocation.clone();
        i
    } else {
        t.state.invocations.push(invocation.clone());
        t.state.invocations.len() - 1
    };
    link_access(t, &id);
    let mut data = json!({"target":route["target"],"caller":route["caller"],"consumer":route["consumer"],"conduit":route["conduit"],"operation":op,"invocation":id,"attempt":invocation["attempt"]});
    if let Some(action) = &retried {
        data["pendingAction"] = json!([action["id"]]);
    }
    record(t, a, c, "invocation-dispatched", data)?;
    if let Some(action) = retried {
        pending::record_answer(t, a, c, &action)?;
        t.decisions.push(decision(
            "answer-setup-retried",
            &["PND-003", "PND-006", "PND-012"],
            &["DL19"],
            json!({"action":action["id"]}),
            vec![],
        ));
    }
    t.decisions.push(decision(
        "dispatch-invocation",
        &[
            "OPS-008", "OPS-010", "OPS-011", "OPS-012", "OPS-024", "OPS-025", "OPS-026", "OPS-018",
            "IMP-007", "IFC-001", "IFC-007", "REC-002",
        ],
        &["DL2"],
        json!({"invocation":id}),
        vec![],
    ));
    if let Some(reconciled) = reconciled {
        if t.state.invocations[reconciled]["contract"]["spec"]
            .get("changes")
            .is_some()
        {
            crate::revisions::reconcile(t, a, c, i, reconciled)?;
        }
        let args = &route["arguments"];
        let outcome = if args["occurred"] == true {
            "succeeded"
        } else {
            "failed"
        };
        t.state.invocations[reconciled]["state"] = json!(outcome);
        let previous_end = t.state.invocations[reconciled].as_object_mut().unwrap();
        previous_end.remove("retained");
        if let Some(cause) = previous_end.remove("cause")
            && previous_end["operation"] == c.qualified("set-up.v1")
        {
            // Older setups have only the reply's cause. Preserve it for action evidence.
            previous_end.entry("setup_cause").or_insert(cause);
        }
        record(
            t,
            a,
            c,
            "invocation-ended",
            json!({"target":t.state.invocations[reconciled]["target"],"invocation":args["invocation"],"outcome":outcome,"actor":route["consumer"],"checked":args["checked"],"reconciled":true}),
        )?;
        t.decisions.push(decision(
            "reconcile-checked-effect",
            &["OPS-014", "OPS-030", "ACC-033"],
            &[],
            json!({"checked":args["checked"]}),
            vec![],
        ));
        if args["occurred"] == true
            && t.state.invocations[reconciled]["operation"] == c.qualified("open.v1")
        {
            let original = t.state.invocations[reconciled].clone();
            let provenance = match original.get("opening_provenance") {
                Some(p) => p.clone(),
                None => crate::inventory::provenance(&t.state, &original, c)?,
            };
            // ACC-033 row 12: settle a removed subject without touching a replacement.
            if let Some(subject) = access::object_by_id(&t.state, &original["object"]) {
                // LIF-028 / CON-031: retirement cannot be undone by an old opening.
                let consumer_retired = access::object_by_id(&t.state, &subject["consumer"])
                    .is_none_or(|consumer| consumer["state"] == "retired");
                let id = subject["id"].clone();
                let subject = t.state.objects.iter_mut().find(|o| o["id"] == id).unwrap();
                if subject.get("provenance").is_none() {
                    subject["provenance"] = provenance;
                }
                subject["open"] = json!(!consumer_retired);
                subject["state"] = json!(if consumer_retired {
                    "registered"
                } else {
                    "ready"
                });
            }
            t.decisions.push(decision(
                "reconcile-original-opening",
                &["ACC-044", "ACC-033", "LIF-028", "CON-031"],
                &[],
                json!({"object":original["object"]}),
                vec![],
            ));
        }
        if args["occurred"] == true
            && t.state.invocations[reconciled]["operation"] == c.qualified("retire.v1")
        {
            let object = t.state.invocations[reconciled]["object"].clone();
            if access::object(&t.state, &object).is_some() {
                crate::retirement::remove(t, a, c, i, &object)?;
            }
        }
        finish(t, a, c, i, "succeeded", json!({}), json!([]), None)?;
    } else if let Some(mut trial) = installation {
        // DL2: restore untouched existing file contents by move, preserving format version 1.
        for (existing, checked) in t
            .state
            .installed_packages
            .iter_mut()
            .zip(&mut trial.state.installed_packages)
        {
            checked.files = std::mem::take(&mut existing.files);
        }
        t.state.installed_packages = trial.state.installed_packages;
        t.decisions.extend(trial.decisions);
        if trial.reply["changed"] != false {
            record(
                t,
                a,
                c,
                "object-change",
                json!({"action":"package-installed","target":route["target"],"invocation":id,"address":route["arguments"]["address"],"version":route["arguments"]["version"],"digest":route["arguments"]["digest"]}),
            )?;
        }
        finish(
            t,
            a,
            c,
            i,
            "succeeded",
            json!({"diagnostics":trial.reply["diagnostics"]}),
            json!([]),
            None,
        )?;
    } else if crate::records::handles(op, c) {
        crate::records::execute(t, a, c, i)?;
    } else if crate::lifecycle::handles(op, c) {
        crate::lifecycle::execute(t, a, c, i)?;
    } else if op == c.qualified("sign-in.v1") {
        crate::sign_in::begin(t, a, c, i)?;
    } else {
        let call = intent(&invocation, &t.state, c)?;
        t.external_intents.push(call);
        t.reply = json!({"pending":true,"invocation":id,"attempt":invocation["attempt"]});
    }
    let v = &mut t.state.invocations[i];
    // Only actual execution observes the end of engine work: recover, missing
    // completions and unconfirmed cancellation establish no fence.
    // Composite engine work needs evidence for every dispatched part. Until the
    // core has that evidence, ending its outer invocation proves no fence.
    if v["state"] != "in-progress"
        && v["implementation"]["source"] == "engine"
        && !["set-up.v1", "retire.v1", "open.v1", "close.v1"]
            .iter()
            .any(|op| v["operation"] == c.qualified(op))
        && v.get("cascade").is_none()
    {
        let attempt = v["attempt"].as_u64().ok_or("invocation has no attempt")?;
        fence_attempt(v, attempt);
        t.decisions.push(decision(
            "fence-ended-engine-invocation",
            &["OPS-014"],
            &[],
            json!({"invocation":v["id"],"attempt":attempt}),
            vec![],
        ));
    }
    Ok(())
}
fn link_access(t: &mut Transition, id: &Value) {
    if let Some(r) = t
        .records
        .iter_mut()
        .rev()
        .find(|r| r["type"] == "access-decision")
    {
        r["data"]["invocation"] = id.clone();
        if let Some(stored) = t.state.records.iter_mut().find(|s| s["id"] == r["id"]) {
            *stored = r.clone();
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn finish(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    outcome: &str,
    outputs: Value,
    content: Value,
    cause: Option<Vec<Value>>,
) -> Result<()> {
    let v = &mut t.state.invocations[i];
    v["state"] = json!(outcome);
    let cause = cause.filter(|_| outcome != "succeeded");
    if v.get("ended").is_none() {
        v["ended"] = json!({"time":a.evidence.time,"outcome":outcome});
        if matches!(outcome, "failed" | "unknown")
            && v["operation"] == c.qualified("set-up.v1")
            && let Some(cause) = &cause
        {
            v["setup_cause"] = json!(cause);
        }
    }
    v.as_object_mut().unwrap().remove("previous");
    if let Some(cause) = &cause {
        v["cause"] = json!(cause);
    } else {
        v.as_object_mut().unwrap().remove("cause");
    }
    if outcome == "succeeded" {
        v["retained"] = json!({"outputs":outputs,"content":content});
    } else {
        v.as_object_mut().unwrap().remove("retained");
    }
    let v = v.clone();
    t.reply = envelope(
        c,
        Some(outcome),
        None,
        Some(&v["id"]),
        cause.as_deref(),
        v["contract"]["spec"]
            .get("outputSchema")
            .map(|_| outputs.clone()),
        content,
        None,
    );
    if v["notices"].as_array().is_some_and(|ids| !ids.is_empty()) {
        t.reply["_meta"][mark(c, "pendingAction")] = v["notices"].clone();
    }
    t.request_replies.push(crate::RequestReply {
        invocation: v["id"].as_str().ok_or("invocation has no ID")?.into(),
        attempt: v["attempt"].as_u64().ok_or("invocation has no attempt")?,
        reply: t.reply.clone(),
    });
    t.decisions.push(decision(
        "end-invocation",
        &[
            "OPS-001", "OPS-002", "OPS-003", "OPS-004", "OPS-007", "IFC-010", "IFC-009", "IFC-025",
            "OPS-013", "REC-002", "SEC-004",
        ],
        &["DL10"],
        json!({"invocation":v["id"],"outcome":outcome}),
        vec![],
    ));
    record(
        t,
        a,
        c,
        "invocation-ended",
        json!({"target":v["target"],"caller":v["caller"],"consumer":v["sender"],"conduit":v["conduit"],"operation":v["operation"],"invocation":v["id"],"attempt":v["attempt"],"outcome":outcome,"outputs":if outcome == "succeeded" { outputs.clone() } else { Value::Null }}),
    )?;
    end(
        t,
        a,
        c,
        &json!({"target":v["target"],"caller":v["caller"],"consumer":v["sender"],"conduit":v["conduit"],"operation":v["operation"],"serving":v["serving"]}),
        Some(outcome),
        None,
        Some(&v["id"]),
    )
}
pub(crate) fn complete(
    t: &mut Transition,
    evidence: &Evidence,
    id: &str,
    attempt: u64,
    completion: Completion,
    c: &Context,
) -> Result<()> {
    let i = t
        .state
        .invocations
        .iter()
        .position(|v| v["id"] == id)
        .ok_or("completion names no invocation")?;
    let v = t.state.invocations[i].clone();
    // IFC-046's first response ends no call: its resend remains in flight.
    let resending = matches!(completion, Completion::ProtocolError { code: -32020, .. })
        && v["wire_retried"] != true
        && read_only(&v["contract"]["spec"])
        && v["attempt"] == attempt
        && v["state"] == "in-progress";
    if answering(&completion)
        && !resending
        && attempt > 0
        && v["attempt"].as_u64().is_some_and(|last| attempt <= last)
    {
        fence_attempt(&mut t.state.invocations[i], attempt);
        t.decisions.push(decision(
            "fence-answered-attempt",
            &["OPS-014"],
            &[],
            json!({"invocation":id,"attempt":attempt}),
            vec![],
        ));
    }
    if v["attempt"] != attempt || v["state"] != "in-progress" {
        // Late or duplicate completions never change a settled or cancelled outcome.
        t.decisions.push(decision(
            "ignore-late-completion",
            &["OPS-013", "OPS-014", "OPS-018", "IMP-007"],
            &[],
            json!({"invocation":id}),
            vec![],
        ));
        t.reply = json!({"ignored":true});
        return Ok(());
    }
    let mut a = Allocation {
        evidence,
        cursor: 0,
    };
    let plain = v["plain"] == true;
    let readonly = read_only(&v["contract"]["spec"]);
    let mut content = json!([]);
    let mut outputs = json!({});
    let mut outcome = "unknown";
    let mut rule = Some(if plain { "IFC-010" } else { "OPS-007" });
    let mut lapse = None;
    match completion {
        Completion::ProtocolError { code: -32020, .. } if v["wire_retried"] != true && readonly => {
            t.state.invocations[i]["wire_retried"] = json!(true);
            let mut call = intent(&v, &t.state, c)?;
            call["kind"] = json!("refresh-tools-and-call");
            t.external_intents.push(call);
            t.decisions.push(decision(
                "repair-parameter-headers-once",
                &["IFC-046", "IMP-007"],
                &[],
                json!({"invocation":id}),
                vec![],
            ));
            t.reply = json!({"pending":true,"invocation":id,"attempt":attempt});
            return Ok(());
        }
        Completion::ProtocolError { code: -32021, data }
            if !plain && capability_error(&data, c).is_some() =>
        {
            // IFC-045 has a call declare Conduit's extension, and URL elicitation only toward a Type
            // that states signIn. Both IFC-008 and IFC-047 promise that nothing was performed.
            let extension = capability_error(&data, c).unwrap();
            t.state.invocations[i]["requiredCapabilities"] = data["requiredCapabilities"].clone();
            if extension {
                t.state.invocations[i]["extensionRevision"] = json!("v0");
            }
            let deciding_rule = if extension { "IFC-008" } else { "IFC-047" };
            t.decisions.push(decision(
                "capability-error-performed-nothing",
                &["OPS-007", deciding_rule],
                &[],
                data,
                vec![],
            ));
            outcome = "failed";
            rule = Some(deciding_rule);
        }
        Completion::ProtocolError { code: -32020, .. }
            if v["wire_retried"] != true && !readonly =>
        {
            rule = Some("IFC-046");
        }
        Completion::ProtocolError { .. } if plain => {
            if readonly {
                outcome = "failed";
                rule = None;
            }
        }
        Completion::Result { result } if sign_in::url_elicitation(&result) => {
            lapse = Some(sign_in::Trigger::UrlElicitation);
        }
        Completion::Result { result } => {
            if plain {
                content = result.get("content").cloned().unwrap_or(json!([]));
                if result.get("resultType").is_none_or(|v| v == "complete") {
                    outcome = if result["isError"] == true {
                        if readonly { "failed" } else { "unknown" }
                    } else {
                        "succeeded"
                    };
                    if outcome != "unknown" {
                        rule = None;
                    }
                }
            } else {
                let meta = &result["_meta"];
                let stated_value = meta.get(mark(c, "outcome"));
                let reason_value = meta.get(mark(c, "reason"));
                let stated = stated_value.and_then(Value::as_str);
                let reason = reason_value.and_then(Value::as_str);
                if stated_value.is_none()
                    && matches!(reason, Some("refused" | "unavailable"))
                    && result["isError"] == true
                {
                    let restriction = meta.get(mark(c, "restriction"));
                    if restriction
                        .is_some_and(|r| v["filters"].as_array().is_some_and(|f| f.contains(r)))
                    {
                        if let Some(previous) = v.get("previous") {
                            t.state.invocations[i]["state"] = previous["state"].clone();
                            if let Some(retained) = previous.get("retained") {
                                t.state.invocations[i]["retained"] = retained.clone();
                            }
                            if let Some(cause) = previous.get("cause") {
                                t.state.invocations[i]["cause"] = cause.clone();
                            } else {
                                t.state.invocations[i]
                                    .as_object_mut()
                                    .unwrap()
                                    .remove("cause");
                            }
                            t.state.invocations[i]
                                .as_object_mut()
                                .unwrap()
                                .remove("previous");
                        } else {
                            t.state.invocations.remove(i);
                        }
                        reject(
                            t,
                            &mut a,
                            c,
                            &json!({"target":v["target"],"caller":v["caller"],"consumer":v["sender"],"conduit":v["conduit"],"operation":v["operation"],"arguments":v["arguments"]}),
                            reason.unwrap(),
                            "IFC-026",
                            None,
                            restriction,
                        )?;
                        t.request_replies.push(crate::RequestReply {
                            invocation: id.into(),
                            attempt,
                            reply: t.reply.clone(),
                        });
                        return crate::lifecycle::child_answered(t, &mut a, c, &v, reason.unwrap());
                    }
                }
                if reason_value.is_none()
                    && let Some(stated) = stated
                    && ["succeeded", "failed", "unknown"].contains(&stated)
                    && (result["isError"] == true) == (stated != "succeeded")
                {
                    outcome = match stated {
                        "succeeded" => "succeeded",
                        "failed" => "failed",
                        _ => "unknown",
                    };
                    rule = None;
                }
                if rule.is_none()
                    && response_ends_call(&result)
                    && matches!(outcome, "failed" | "unknown")
                    && sign_in::lapse_diagnostic(&result)
                {
                    lapse = Some(sign_in::Trigger::LapsedLogin);
                }
            }
            if outcome == "succeeded" {
                outputs = result
                    .get("structuredContent")
                    .cloned()
                    .unwrap_or(json!({}));
                if let Some(schema) = v["contract"]["spec"].get("outputSchema") {
                    let (env, scope, _) = schemas::operation(
                        &v["held_target"],
                        v["operation"].as_str().unwrap(),
                        &c.core_package,
                    )?;
                    if !env.valid(&scope, schema, &outputs, &c.core_package)? {
                        outcome = "unknown";
                        rule = Some("OPS-007");
                        outputs = json!({});
                    }
                } else {
                    outputs = json!({});
                }
            }
        }
        _ => {}
    }
    if let Some(trigger) = lapse {
        finish_login_lapse(t, &mut a, c, i, &v, outcome, trigger)?;
    } else {
        if outcome != "succeeded" {
            outputs = json!({});
        }
        let cause = rule
            .map(|rule| explain::outcome_cause(&v, outcome, rule).map(|d| vec![d]))
            .transpose()?;
        finish(t, &mut a, c, i, outcome, outputs, content, cause)?;
    }
    crate::lifecycle::child_ended(t, &mut a, c, i)
}

fn finish_login_lapse(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    v: &Value,
    outcome: &str,
    trigger: sign_in::Trigger,
) -> Result<()> {
    let instance = access::object_by_id(&t.state, &v["target"])
        .cloned()
        .unwrap_or(Value::Null);
    let expected = sign_in::expected_account(&instance, c)?;
    let cause = explain::outcome_diagnostic(
        v,
        outcome,
        trigger.rule(),
        trigger.cause(&expected, instance["state"] == "retired"),
    )?;
    finish(t, a, c, i, outcome, json!({}), json!([]), Some(vec![cause]))?;
    if let sign_in::ExpectedAccount::Set { pointer, account } = expected
        && let Some(action) = pending::raise_sign_in(
            &mut t.state,
            &instance,
            account,
            &pointer,
            pending::SignInRequest::Invocation {
                invocation: v,
                trigger: trigger.label(),
            },
            a,
        )?
    {
        let ended = t
            .records
            .iter_mut()
            .rev()
            .find(|r| r["type"] == "invocation-ended" && r["data"]["invocation"] == v["id"])
            .ok_or("sign-in lapse invocation has no ending record")?;
        ended["data"]["pendingAction"] = json!([action["id"]]);
        let stored = t
            .state
            .records
            .iter_mut()
            .rev()
            .find(|r| r["id"] == ended["id"])
            .ok_or("sign-in lapse invocation ending record absent from state")?;
        *stored = ended.clone();
        let mut rules = vec!["PND-001", "PND-005", "PND-012"];
        if matches!(trigger, sign_in::Trigger::UrlElicitation) {
            rules.push("IFC-016");
        }
        t.decisions.push(decision(
            "raise-sign-in-action",
            &rules,
            &["DL19"],
            json!({"action":action["id"], "instance":instance["id"]}),
            vec![],
        ));
    }
    Ok(())
}

pub(crate) fn cancel(
    t: &mut Transition,
    evidence: &Evidence,
    id: &str,
    confirmation: Option<Value>,
    c: &Context,
) -> Result<()> {
    let i = t
        .state
        .invocations
        .iter()
        .position(|v| v["id"] == id && v["state"] == "in-progress")
        .ok_or("cancellation names no running invocation")?;
    let mut a = Allocation {
        evidence,
        cursor: 0,
    };
    if t.state.invocations[i].get("cascade").is_some() {
        return crate::cascade::cancel(t, &mut a, c, i);
    }
    if t.state.invocations[i].get("setup").is_some()
        || t.state.invocations[i].get("retirement").is_some()
    {
        return crate::lifecycle::cancel(t, &mut a, c, i);
    }
    t.external_intents
        .push(json!({"kind":"cancel","invocation":id,"attempt":t.state.invocations[i]["attempt"]}));
    t.decisions.push(decision(
        "forward-cancellation",
        &["OPS-027"],
        &[],
        json!({"invocation":id}),
        vec![],
    ));
    let failed = t.state.invocations[i]["plain"] != true
        && confirmation.as_ref().is_some_and(|r| {
            response_ends_call(r)
                && r["isError"] == true
                && r["_meta"][mark(c, "outcome")] == "failed"
                && r["_meta"].get(mark(c, "reason")).is_none()
        });
    if failed {
        let attempt = t.state.invocations[i]["attempt"].as_u64().unwrap();
        fence_attempt(&mut t.state.invocations[i], attempt);
        t.decisions.push(decision(
            "fence-confirmed-cancellation",
            &["OPS-014"],
            &[],
            json!({"invocation":id,"attempt":attempt}),
            vec![],
        ));
    }
    let cause = if failed {
        None
    } else {
        Some(vec![explain::outcome_cause(
            &t.state.invocations[i],
            "unknown",
            "OPS-027",
        )?])
    };
    finish(
        t,
        &mut a,
        c,
        i,
        if failed { "failed" } else { "unknown" },
        json!({}),
        json!([]),
        cause,
    )
}
pub(crate) fn expire(t: &mut Transition, id: &str) -> Result<()> {
    let v = t
        .state
        .invocations
        .iter_mut()
        .find(|v| v["id"] == id)
        .ok_or("expiry names no invocation")?;
    // Pending repeats can hold retained results in previous snapshots. Expiry is
    // about outputs, never the invocation state or the snapshot that holds them.
    let mut snapshot = Some(v);
    while let Some(v) = snapshot {
        let object = v.as_object_mut().ok_or("invalid invocation")?;
        object.remove("retained");
        snapshot = object.get_mut("previous");
    }
    t.decisions.push(decision(
        "expire-kept-result",
        &["OPS-009", "OPS-034"],
        &["DL10"],
        json!({"invocation":id}),
        vec![],
    ));
    t.reply = json!({"expired":true});
    Ok(())
}

/// Recover a persisted dispatch without reissuing it. No clock, timeout or shell
/// observation can establish its outcome; only reconciliation (or an eligible
/// caller repeat under OPS-012) can do so after this transition.
pub(crate) fn recover(t: &mut Transition, evidence: &Evidence, c: &Context) -> Result<()> {
    let mut a = Allocation {
        evidence,
        cursor: 0,
    };
    for i in 0..t.state.invocations.len() {
        if t.state.invocations[i]["state"] == "in-progress" {
            if let Some(id) = t.state.invocations[i].get("retirement_of")
                && let Some(o) = t.state.objects.iter_mut().find(|o| o["id"] == *id)
            {
                o["shutdown"] = json!("unknown");
            }
            let cause = explain::outcome_cause(&t.state.invocations[i], "unknown", "IMP-007")?;
            finish(
                t,
                &mut a,
                c,
                i,
                "unknown",
                json!({}),
                json!([]),
                Some(vec![cause]),
            )?;
            let v = &t.state.invocations[i];
            if v["hook"] == true
                && v["operation"] == c.qualified("sign-out-hook.v1")
                && let Some(parent) = v.get("setup_parent")
                && let Some(setup) = t.state.invocations.iter().find(|s| s["id"] == *parent)
                && setup["setup"]["phase"] == "undo-pending"
            {
                let (instance, hook) = (setup["object"].clone(), v["id"].clone());
                crate::lifecycle::report_sign_in_undone(
                    t,
                    &instance,
                    "unknown",
                    &hook,
                    &evidence.time,
                )?;
            }
            crate::lifecycle::record_setup_sign_in(t, i)?;
        }
    }
    t.decisions.push(decision(
        "recover-interrupted-invocations",
        &["IMP-007", "OPS-013", "OPS-018"],
        &["DL2"],
        json!({}),
        vec![],
    ));
    t.reply = json!({"restored":true});
    Ok(())
}

/// A hook is an invocation with a stable per-instance key and no Conduit. It is
/// authorized by its setup, and retains the caller and filters of that invocation.
pub(crate) fn setup_hook(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    parent: usize,
    object: &Value,
    implementation: &Value,
) -> Result<()> {
    hook(t, a, c, parent, object, implementation, "setup-hook.v1")
}

fn previous_hook(s: &crate::State, object: &Value, operation: &str) -> Option<usize> {
    s.invocations.iter().position(|v| {
        v["hook"] == true && v["target"] == object["id"] && v["operation"] == operation
    })
}

pub(crate) fn hook_needs_dispatch(s: &crate::State, object: &Value, operation: &str) -> bool {
    previous_hook(s, object, operation).is_none_or(|i| s.invocations[i]["state"] == "failed")
}

pub(crate) fn hook(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    parent: usize,
    object: &Value,
    implementation: &Value,
    name: &str,
) -> Result<()> {
    let p = t.state.invocations[parent].clone();
    let op = c.qualified(name);
    let previous = previous_hook(&t.state, object, &op);
    if let Some(i) = previous
        && t.state.invocations[i]["state"] != "failed"
    {
        let old = t.state.invocations[i].clone();
        t.state.invocations[parent]["setup"]["phase"] = json!("hook-pending");
        t.state.invocations[i]["setup_parent"] = p["id"].clone();
        t.decisions.push(decision(
            "keep-hook-outcome",
            &["OPS-021", "OPS-028"],
            &[],
            json!({"hook":old["id"]}),
            vec![],
        ));
        return crate::lifecycle::child_ended(t, a, c, i);
    }
    t.state.invocations[parent]["setup"]["phase"] = json!("hook-pending");
    let id = dispatch_hook(
        t,
        a,
        c,
        parent,
        object,
        (op, implementation),
        HookKey::Stable(previous),
    )?;
    t.decisions.push(decision(
        "dispatch-setup-hook",
        &[
            "TPL-012", "TPL-017", "OPS-021", "OPS-028", "ACC-018", "IMP-002",
        ],
        &[],
        json!({"hook":id}),
        vec![],
    ));
    Ok(())
}

pub(crate) fn setup_sign_in_hook(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    parent: usize,
    object: &Value,
    name: &str,
    promises: &[&str],
) -> Result<bool> {
    let op = c.qualified(name);
    let Some(implementation) = object["resolved_implementations"].get(&op) else {
        return Ok(false);
    };
    let id = dispatch_hook(
        t,
        a,
        c,
        parent,
        object,
        (op, implementation),
        HookKey::Fresh,
    )?;
    let decision_name = if name == "account-hook.v1" {
        "dispatch-account-check"
    } else {
        "dispatch-sign-out"
    };
    t.decisions.push(decision(
        decision_name,
        promises,
        &[],
        json!({"hook":id}),
        vec![],
    ));
    Ok(true)
}

enum HookKey {
    Stable(Option<usize>),
    Fresh,
}

fn dispatch_hook(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    parent: usize,
    object: &Value,
    (op, implementation): (String, &Value),
    key: HookKey,
) -> Result<Value> {
    let p = t.state.invocations[parent].clone();
    let (key, previous) = match key {
        HookKey::Stable(previous) => (
            json!(format!("{}:{op}", object["id"].as_str().unwrap())),
            previous,
        ),
        HookKey::Fresh => (Value::Null, None),
    };
    let target = crate::registration::publication_object(object, c)?;
    let (_, _, contract) = schemas::operation(&target, &op, &c.core_package)?;
    let id = if let Some(i) = previous {
        t.state.invocations[i]["id"].clone()
    } else {
        json!(a.next(&mut t.state)?)
    };
    let attempt = previous
        .and_then(|i| t.state.invocations[i]["attempt"].as_u64())
        .unwrap_or(0)
        + 1;
    let mut v = json!({"id":id,"key":key,"hook":true,"setup_parent":p["id"],"serving":p["id"],"sender":p["sender"],"caller":p["caller"],"target":object["id"],"operation":op,"arguments":{},"filters":p["filters"],"state":"in-progress","plain":implementation["item"]["plain"] == true,"contract":contract,"held_target":target,"implementation":implementation,"needs_result":false,"attempt":attempt,"config":object["config"],"revision":object["revision"]});
    start_attempt(&mut v, previous.map(|i| &t.state.invocations[i]));
    if let Some(id) = p.get("retirement_of") {
        v["retirement_of"] = id.clone();
    }
    if let Some(i) = previous {
        t.state.invocations[i] = v.clone();
    } else {
        t.state.invocations.push(v.clone());
    }
    record(
        t,
        a,
        c,
        "invocation-dispatched",
        json!({"target":object["id"],"invocation":id,"caller":p["caller"],"consumer":p["sender"],"serving":p["id"],"operation":op,"attempt":attempt}),
    )?;
    t.external_intents.push(intent(&v, &t.state, c)?);
    t.reply = json!({"pending":true,"invocation":p["id"],"hook":id});
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Input,
        access::{AccessRequest, Identity},
        explain::tests::{assert_cause, evidence, fixture, transition},
        step,
    };
    use std::collections::BTreeMap;

    #[test]
    fn intent_declares_url_elicitation_only_toward_sign_in_types() {
        const CAP: &str = "io.modelcontextprotocol/clientCapabilities";
        let (_, c) = fixture();
        let v = json!({
            "id": "v1",
            "attempt": 1,
            "target": "s1",
            "operation": "example.org/p#ping.v1",
            "arguments": {"x": 1},
            "implementation": {"item": {}},
            "caller": "r",
            "conduit": "k",
            "filters": [],
            "config": {},
            "revision": "1",
            "key": null
        });
        let without_url = dispatch_intent(&v, false, &c);
        let meta = without_url["call"]["params"]["_meta"].as_object().unwrap();
        assert_eq!(meta.len(), 8);
        assert_eq!(meta.get(CAP), None);

        let first = dispatch_intent(&v, true, &c);
        let mut with_url = first.clone();
        let meta = with_url["call"]["params"]["_meta"].as_object_mut().unwrap();
        assert_eq!(meta.len(), 9);
        assert_eq!(meta.remove(CAP), Some(json!({"elicitation":{"url":{}}})));
        assert_eq!(with_url, without_url);

        let mut p = v.clone();
        p["plain"] = json!(true);
        p["implementation"] = json!({"item":{"tools":{"example.org/p#ping.v1":"ping_tool"}}});
        assert_eq!(
            dispatch_intent(&p, false, &c)["call"]["params"],
            json!({"name":"ping_tool","arguments":{"x":1}})
        );
        assert_eq!(
            dispatch_intent(&p, true, &c)["call"]["params"],
            json!({
                "name": "ping_tool",
                "arguments": {"x": 1},
                "_meta": {CAP: {"elicitation": {"url": {}}}}
            })
        );

        for object in [
            json!({"id":"s1","kind":"Agent","name":"signed"}),
            json!({"id":"s1","kind":"Agent","name":"signed","spec":{"type":"x"},"held":{}}),
            json!({"id":"other","kind":"Agent"}),
        ] {
            let state = State {
                objects: vec![object],
                ..State::default()
            };
            assert_eq!(intent(&v, &state, &c).unwrap(), without_url);
        }

        let responses = json!({"sign-in":{"action":"accept"}});
        let mut retry = retry_intent(&first, "impl-state-1", responses.as_object().unwrap());
        let params = retry["call"]["params"].as_object_mut().unwrap();
        assert_eq!(params.remove("requestState"), Some(json!("impl-state-1")));
        assert_eq!(params.remove("inputResponses"), Some(responses));
        assert_eq!(retry, first);
    }

    #[test]
    fn respond_is_refused_until_input_requests_are_built() {
        let (state, c) = fixture();
        let ids: Vec<_> = (1..50)
            .map(|n| format!("00000000-0000-4000-8046-a0000000{n:04}"))
            .collect();
        let input: Input = serde_json::from_value(json!({
            "kind": "respond",
            "evidence": {"time":"2026-11-09T13:00:00Z","ids":ids},
            "invocation": "absent-invocation",
            "attempt": 1,
            "identity": {"kind":"instance","id":"root"},
            "responses": {"url":{"action":"accept"}},
            "meta": {}
        }))
        .unwrap();
        let t = step(&state, input, &c).unwrap();
        assert_eq!(t.reply["error"]["code"], -32602);
        let data = &t.reply["error"]["data"];
        assert_eq!(data[mark(&c, "reason")], "invalid");
        let diagnostics = data["diagnostics"].as_array().unwrap();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0]["ruleId"], "IFC-049");
        assert!(t.records.is_empty());
        assert!(t.input_required.is_empty());
        assert_eq!(t.state, state);
        assert_eq!(t.decisions.len(), 1);
        assert_eq!(t.decisions[0].action, "refuse-input-response");
    }

    fn call_fixture(plain: bool, spec: Value) -> (Transition, Context) {
        let (mut state, c, _) = repeat_fixture(false);
        state.invocations.push(json!({
            "id":"invocation-id","plain":plain,"contract":{"spec":spec},"state":"in-progress",
            "attempt":1,"operation":c.qualified("reserved.v1"),"target":"management",
            "held_target":state.objects[3],"sender":"root","caller":"root"
        }));
        (transition(state), c)
    }

    fn unused_evidence(state: &State) -> Evidence {
        let mut next = evidence();
        next.ids.retain(|id| !state.allocated_ids.contains(id));
        next
    }

    fn interrupted_setup() -> (Transition, Context) {
        let (mut state, c) = fixture();
        let inspect = c.qualified("inspect.v1");
        let reconcile = c.qualified("reconcile.v1");
        let capability = c.qualified("capability.v1");
        let pool = &mut state.objects[5];
        let contracts = &mut pool["schema_environment"]["scopes"]["core"]["contracts"];
        contracts["type.v1"]["spec"]["records"]["outputs"] = json!(true);
        contracts["capability.v1"] =
            json!({"kind":"Capability","spec":{"operations":[inspect,reconcile]}});
        contracts["inspect.v1"] = json!({"kind":"Operation","metadata":{"name":"inspect.v1"},
            "spec":{"inputSchema":{"type":"object"},"outputSchema":{"type":"object"}}});
        for operation in [&inspect, &reconcile] {
            pool["resolved_implementations"][operation] = json!({"source":"engine"});
        }
        let held_pool = pool.clone();
        state
            .objects
            .push(json!({"id":"o1","kind":"Resource","name":"shared",
            "owner":"owners","state":"registered"}));
        for caller in ["root", "agent"] {
            state
                .objects
                .push(json!({"id":format!("{caller}-owners"),"kind":"Conduit",
                "local_name":"owners","consumer":caller,"target":"owners","state":"ready",
                "open":true,"capability":capability,"operations":[inspect,reconcile]}));
        }
        state
            .invocations
            .push(json!({"id":"s1","operation":c.qualified("set-up.v1"),
            "state":"in-progress","object":"o1","target":"owners","held_target":held_pool,
            "caller":"root","sender":"root","attempt":1}));
        let mut evidence = evidence();
        evidence.time = "2026-11-09T12:30:00Z".into();
        let recovered = step(&state, Input::Recover { evidence }, &c).unwrap();
        (recovered, c)
    }

    fn inspect_setup(
        state: &State,
        c: &Context,
        caller: &str,
        time: &str,
        key: Option<&str>,
    ) -> Transition {
        let mut evidence = unused_evidence(state);
        evidence.time = time.into();
        step(
            state,
            Input::Invoke {
                evidence,
                request: AccessRequest {
                    identity: Identity::Instance { id: caller.into() },
                    tool: "owners.inspect.v1".into(),
                    arguments: json!({"object":{"kind":"Resource","name":"shared"}}),
                    serving: None,
                    readable: true,
                    authority_readable: true,
                    client_capabilities: None,
                    sign_in_login: None,
                    sign_in_return: None,
                },
                key: key.map(str::to_owned),
                archive: None,
            },
            c,
        )
        .unwrap()
    }

    #[test]
    fn setup_action_is_private_to_root_inspection_and_its_retained_reply() {
        let (setup, c) = interrupted_setup();
        let before = "2026-11-16T12:29:59Z";
        let expiry = "2026-11-16T12:30:00Z";
        let root = inspect_setup(&setup.state, &c, "root", before, Some("key"));
        assert_eq!(root.reply["isError"], false, "{}", root.reply);
        let mut recorded_outputs = root.reply["structuredContent"].clone();
        assert_eq!(recorded_outputs["status"]["pending"][0]["id"], "s1");
        recorded_outputs["status"]
            .as_object_mut()
            .unwrap()
            .remove("pending");

        let agent = inspect_setup(&root.state, &c, "agent", before, None);
        assert_eq!(agent.reply["isError"], false, "{}", agent.reply);
        let outputs = &agent.reply["structuredContent"];
        assert!(outputs["status"].get("pending").is_none());
        let inspection = &root.reply["_meta"][mark(&c, "invocation")];
        for event in ["invocation-ended", "request-ended"] {
            let record = outputs["records"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["type"] == event && r["data"]["invocation"] == *inspection)
                .unwrap();
            assert_eq!(record["data"]["outputs"], recorded_outputs);
        }

        let repeated = inspect_setup(&agent.state, &c, "root", expiry, Some("key"));
        assert_eq!(
            repeated.reply["structuredContent"],
            root.reply["structuredContent"]
        );
        let unanswered = mark(&c, "unanswered");
        assert_eq!(root.reply["_meta"][&unanswered], 1);
        assert!(repeated.reply["_meta"].get(&unanswered).is_none());
        let repeated_record = repeated
            .records
            .iter()
            .find(|r| r["type"] == "request-ended");
        assert_eq!(
            repeated_record.unwrap()["data"]["outputs"],
            recorded_outputs
        );
        assert!(repeated.request_replies.is_empty());
        let fresh = inspect_setup(&repeated.state, &c, "root", expiry, None);
        assert_eq!(fresh.reply["isError"], false, "{}", fresh.reply);
        assert!(
            fresh.reply["structuredContent"]["status"]
                .get("pending")
                .is_none()
        );
    }

    #[test]
    fn setup_action_preserves_first_ending_diagnostics_after_reconciliation() {
        let (setup, c) = interrupted_setup();
        let original = &setup.request_replies[0].reply["structuredContent"]["diagnostics"];
        assert_eq!(original[0]["ruleId"], "IMP-007");
        for legacy in [false, true] {
            let mut state = setup.state.clone();
            if legacy {
                let invocation = state.invocations[0].as_object_mut().unwrap();
                invocation.remove("ended");
                invocation.remove("setup_cause");
            }
            let reconciled = step(
                &state,
                Input::Invoke {
                    evidence: unused_evidence(&state),
                    request: AccessRequest {
                        identity: Identity::Instance { id: "root".into() },
                        tool: "owners.reconcile.v1".into(),
                        arguments: json!({"invocation":"s1","occurred":true,"checked":"observed"}),
                        serving: None,
                        readable: true,
                        authority_readable: true,
                        client_capabilities: None,
                        sign_in_login: None,
                        sign_in_return: None,
                    },
                    key: None,
                    archive: None,
                },
                &c,
            )
            .unwrap();
            assert_eq!(reconciled.reply["isError"], false, "{}", reconciled.reply);
            let invocation = &reconciled.state.invocations[0];
            assert_eq!(invocation["state"], "succeeded");
            assert!(invocation.get("cause").is_none());
            let inspected = inspect_setup(&reconciled.state, &c, "root", &evidence().time, None);
            let action = &inspected.reply["structuredContent"]["status"]["pending"][0];
            assert_eq!(action["id"], "s1");
            assert_eq!(action["evidence"][1]["value"], "unknown");
            assert_eq!(action["evidence"][2]["value"], *original);
            assert_eq!(action["expiry"], "2026-11-16T12:30:00Z");
        }
    }

    fn answer(t: &Transition, c: &Context, completion: Completion) -> Transition {
        let invocation = &t.state.invocations[0];
        step(
            &t.state,
            Input::Complete {
                evidence: unused_evidence(&t.state),
                invocation: invocation["id"].as_str().unwrap().into(),
                attempt: invocation["attempt"].as_u64().unwrap(),
                completion,
            },
            c,
        )
        .unwrap()
    }

    fn assert_ending(t: &Transition, c: &Context, outcome: &str, rule: Option<&str>) {
        let reply = &t.request_replies.last().unwrap().reply;
        assert_eq!(reply["_meta"][mark(c, "outcome")], outcome);
        if let Some(rule) = rule {
            let invocation = &t.state.invocations[0];
            let operation = invocation["operation"].as_str().unwrap();
            assert_cause(reply, rule, &format!(" ended {outcome} under {rule}: "));
            assert_eq!(
                reply["structuredContent"]["diagnostics"],
                invocation["cause"]
            );
            let message = reply["structuredContent"]["diagnostics"][0]["message"]["text"]
                .as_str()
                .unwrap();
            assert!(message.starts_with(operation), "{message}");
            assert_eq!(reply["content"][0]["text"], format!("{outcome}: {message}"));
        } else {
            assert!(reply.get("structuredContent").is_none(), "{reply}");
            assert!(t.state.invocations[0].get("cause").is_none());
            assert_eq!(reply["content"][0]["text"], outcome);
        }
    }

    fn rpc(code: i64, data: Value) -> Completion {
        Completion::ProtocolError { code, data }
    }

    fn plain_unknown(completion: Completion, spec: Value) {
        let (t, c) = call_fixture(true, spec);
        assert_ending(&answer(&t, &c, completion), &c, "unknown", Some("IFC-010"));
    }

    #[test]
    fn plain_effectful_protocol_error_names_ifc010() {
        plain_unknown(rpc(-32602, Value::Null), json!({"effect":"write"}));
    }

    #[test]
    fn plain_effectful_error_result_names_ifc010() {
        plain_unknown(
            Completion::Result {
                result: json!({"isError":true}),
            },
            json!({"effect":"write"}),
        );
    }

    #[test]
    fn plain_file_write_outcome_names_the_tool_and_omits_the_arguments() {
        let (mut state, c, mut request) = repeat_fixture(false);
        let operation = "example.org/files#file.write.v1";
        let target = &mut state.objects[3];
        target["schema_environment"]["scopes"]["files"] = json!({
            "address":"example.org/files","manifest":{},"contracts":{
                "file.write.v1":{"kind":"Operation","metadata":{"name":"file.write.v1"},"spec":{
                    "effect":"Writes content at path",
                    "inputSchema":{"type":"object","properties":{
                        "path":{"type":"string"},"content":{"type":"string"}
                    },"required":["path","content"]}
                }},
                "directory.v1":{"kind":"Capability","spec":{"operations":[operation]}}
            }
        });
        target["contract_scope"] = json!("files");
        target["schema_environment"]["scopes"]["files"]["manifest"][&c.core_package] =
            json!("core");
        target["resolved_implementations"][operation] =
            json!({"source":"project","item":{"remote":"http://localhost","plain":true}});
        let path = state
            .objects
            .iter_mut()
            .find(|o| o["id"] == "root-management")
            .unwrap();
        path["local_name"] = json!("project");
        path["capability"] = json!("example.org/files#directory.v1");
        path["operations"] = json!([operation]);
        request.tool = "project.file.write.v1".into();
        request.arguments = json!({"path":"./hello.txt","content":"caller-supplied file body"});

        let pending = invoke_repeat(&state, &c, &request);
        assert_eq!(pending.reply["pending"], true, "{}", pending.reply);
        let ended = answer(
            &pending,
            &c,
            Completion::Result {
                result: json!({"isError":true}),
            },
        );
        assert_ending(&ended, &c, "unknown", Some("IFC-010"));
        assert_cause(
            &ended.reply,
            "IFC-010",
            "example.org/files#file.write.v1 for tool project.file.write.v1 ended unknown",
        );
        let reply = ended.reply.to_string();
        assert!(!reply.contains("caller-supplied file body"));
        assert!(!reply.contains("hello.txt"));
    }

    #[test]
    fn plain_input_required_names_ifc010() {
        plain_unknown(
            Completion::Result {
                result: json!({"resultType":"input_required"}),
            },
            json!({}),
        );
    }

    #[test]
    fn plain_unrecognized_result_type_names_ifc010() {
        plain_unknown(
            Completion::Result {
                result: json!({"resultType":"unrecognized"}),
            },
            json!({}),
        );
    }

    #[test]
    fn plain_missing_result_names_ifc010() {
        plain_unknown(Completion::Missing, json!({}));
    }

    #[test]
    fn aware_missing_and_malformed_results_name_ops007() {
        let (t, c) = call_fixture(
            false,
            json!({"outputSchema":{"type":"object","required":["value"]}}),
        );
        for completion in [
            Completion::Missing,
            rpc(-32602, Value::Null),
            Completion::Result { result: json!({}) },
            Completion::Result {
                result: json!({"_meta":{"test/outcome":"failed","test/reason":"unavailable"},"isError":true}),
            },
            Completion::Result {
                result: json!({"_meta":{"test/outcome":"failed"},"isError":false}),
            },
            Completion::Result {
                result: json!({"_meta":{"test/outcome":"succeeded"},"structuredContent":{}}),
            },
        ] {
            assert_ending(&answer(&t, &c, completion), &c, "unknown", Some("OPS-007"));
        }
    }

    #[test]
    fn aware_capability_errors_name_their_deciding_promise() {
        let (t, c) = call_fixture(false, json!({}));
        for (required, rule) in [
            (
                json!({"extensions":{"test/implementation-interface":{}}}),
                "IFC-008",
            ),
            (json!({"sampling":{}}), "IFC-047"),
        ] {
            let ended = answer(
                &t,
                &c,
                rpc(-32021, json!({"requiredCapabilities":required})),
            );
            assert_ending(&ended, &c, "failed", Some(rule));
        }
    }

    #[test]
    fn unsafe_parameter_header_errors_name_ifc046() {
        for spec in [
            json!({"effect":"write"}),
            json!({"changes":{}}),
            json!({"startsModelWork":true}),
        ] {
            for plain in [true, false] {
                let (t, c) = call_fixture(plain, spec.clone());
                assert_ending(
                    &answer(&t, &c, rpc(-32020, Value::Null)),
                    &c,
                    "unknown",
                    Some("IFC-046"),
                );
            }
        }
    }

    #[test]
    fn plain_read_failures_have_no_diagnostic() {
        let (t, c) = call_fixture(true, json!({}));
        for completion in [
            rpc(-32602, Value::Null),
            rpc(-32021, json!({"requiredCapabilities":{"sampling":{}}})),
            Completion::Result {
                result: json!({"isError":true}),
            },
        ] {
            assert_ending(&answer(&t, &c, completion), &c, "failed", None);
        }
    }

    #[test]
    fn second_parameter_header_error_on_a_plain_read_has_no_diagnostic() {
        let (t, c) = call_fixture(true, json!({}));
        let resent = answer(&t, &c, rpc(-32020, Value::Null));
        assert_eq!(resent.external_intents[0]["kind"], "refresh-tools-and-call");
        assert_ending(
            &answer(&resent, &c, rpc(-32020, Value::Null)),
            &c,
            "failed",
            None,
        );
    }

    #[test]
    fn aware_stated_failure_has_no_diagnostic() {
        let (t, c) = call_fixture(false, json!({}));
        let ended = answer(
            &t,
            &c,
            Completion::Result {
                result: json!({"_meta":{"test/outcome":"failed"},"isError":true}),
            },
        );
        assert_ending(&ended, &c, "failed", None);
    }

    #[test]
    fn aware_nonterminal_result_does_not_take_the_lapse_path() {
        let (t, c) = call_fixture(false, json!({}));
        let task = json!({"resultType":"task","task":{"taskId":"t"},
            "_meta":{"test/outcome":"failed"},"isError":true});
        let mut keyed = task.clone();
        keyed["structuredContent"] = json!({"diagnostics":[{"ruleId":"PND-005","level":"error"}]});
        let [plain, lapse] =
            [task, keyed].map(|result| answer(&t, &c, Completion::Result { result }));
        for ended in [&plain, &lapse] {
            assert_ending(ended, &c, "failed", None);
            assert!(ended.state.pending_actions.is_empty());
            assert!(
                !serde_json::to_string(&ended.records)
                    .unwrap()
                    .contains("PND-005")
            );
        }
        assert_eq!(
            plain.request_replies.last().unwrap().reply,
            lapse.request_replies.last().unwrap().reply
        );
        assert_eq!(plain.state.invocations[0], lapse.state.invocations[0]);
    }

    #[test]
    fn only_unconfirmed_cancellation_names_ops027() {
        let (t, c) = call_fixture(false, json!({}));
        for (confirmation, outcome, rule) in [
            (None, "unknown", Some("OPS-027")),
            (
                Some(json!({"_meta":{"test/outcome":"failed"},"isError":true})),
                "failed",
                None,
            ),
        ] {
            let ended = step(
                &t.state,
                Input::Cancel {
                    evidence: evidence(),
                    invocation: "invocation-id".into(),
                    confirmation,
                },
                &c,
            )
            .unwrap();
            assert_ending(&ended, &c, outcome, rule);
        }
    }

    #[test]
    fn interrupted_invocation_names_imp007_even_if_cancelled() {
        let (mut t, c) = call_fixture(false, json!({}));
        for cancelled in [false, true] {
            t.state.invocations[0]["cancelled"] = json!(cancelled);
            let ended = step(
                &t.state,
                Input::Recover {
                    evidence: evidence(),
                },
                &c,
            )
            .unwrap();
            assert_ending(&ended, &c, "unknown", Some("IMP-007"));
        }
    }

    fn repeat_fixture(filtered: bool) -> (State, Context, AccessRequest) {
        let (mut state, c) = fixture();
        let operation = c.qualified("reserved.v1");
        let target = &mut state.objects[3];
        target["schema_environment"]["scopes"]["core"]["contracts"]["capability.v1"] =
            json!({"kind":"Capability","spec":{"operations":[operation]}});
        target["resolved_implementations"][&operation] =
            json!({"source":"project","item":{"command":["test"]}});
        let path = state
            .objects
            .iter_mut()
            .find(|o| o["id"] == "root-management")
            .unwrap();
        path["capability"] = json!(c.qualified("capability.v1"));
        if filtered {
            path["requirements"] =
                json!({"limit":{"requirement":"example.org/filter#limit.v1","arguments":{}}});
        }
        let request = AccessRequest {
            identity: Identity::Instance { id: "root".into() },
            tool: "management.reserved.v1".into(),
            arguments: json!({}),
            serving: None,
            readable: true,
            authority_readable: true,
            client_capabilities: None,
            sign_in_login: None,
            sign_in_return: None,
        };
        (state, c, request)
    }

    fn invoke_repeat(state: &State, c: &Context, request: &AccessRequest) -> Transition {
        step(
            state,
            Input::Invoke {
                evidence: unused_evidence(state),
                request: request.clone(),
                key: Some("key".into()),
                archive: None,
            },
            c,
        )
        .unwrap()
    }

    #[test]
    fn sign_in_return_not_taken_is_kept_nowhere() {
        let (state, c, request) = repeat_fixture(false);
        let pending = invoke_repeat(&state, &c, &request);
        assert_eq!(pending.reply["pending"], true);
        let value = "http://127.0.0.1:49152/callback?code=not-a-code&state=not-a-state";
        let mut input = json!({
            "identity":{"kind":"instance","id":"root"},
            "tool":"management.reserved.v1",
            "arguments":{},
            "signInReturn":{"value":value,"via":"paste"}
        });
        let repeat = serde_json::from_value::<AccessRequest>(input.clone()).unwrap();
        let invalid = invoke_repeat(&pending.state, &c, &repeat);
        assert_eq!(invalid.reply["_meta"][mark(&c, "reason")], "invalid");
        let diagnostics = invalid.reply["structuredContent"]["diagnostics"]
            .as_array()
            .unwrap();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0]["ruleId"], "PND-013");
        let message = diagnostics[0]["message"]["text"].as_str().unwrap();
        assert!(message.contains(&mark(&c, "signInReturn")));
        let target =
            access::object(&pending.state, &pending.state.invocations[0]["target"]).unwrap();
        let kind = target["kind"].as_str().unwrap();
        let name = target["name"].as_str().unwrap();
        assert!(message.contains(&format!(" under PND-013 for {kind}/{name}: ")));
        assert!(invalid.external_intents.is_empty());
        assert_eq!(
            invalid.state.invocations.len(),
            pending.state.invocations.len()
        );
        assert_eq!(
            invalid
                .decisions
                .iter()
                .filter(|d| d.action == "refuse-sign-in-return")
                .count(),
            1
        );

        input["tool"] = json!("nope.list.v1");
        let unpresented = serde_json::from_value::<AccessRequest>(input.clone()).unwrap();
        let refused = invoke_repeat(&pending.state, &c, &unpresented);
        assert_eq!(
            refused.reply["error"]["data"][mark(&c, "reason")],
            "refused"
        );
        for (transition, request) in [(&invalid, &repeat), (&refused, &unpresented)] {
            for rendered in [
                serde_json::to_string(transition).unwrap(),
                format!("{transition:?}"),
                format!("{request:?}"),
            ] {
                for withheld in [value, "not-a-code", "not-a-state"] {
                    assert!(!rendered.contains(withheld));
                }
            }
        }

        input["signInReturn"] = Value::Null;
        let null_return = serde_json::from_value::<AccessRequest>(input).unwrap();
        assert!(null_return.sign_in_return.is_none());
    }

    #[test]
    fn unknown_repeat_replays_engine_cause_and_keeps_stated_or_legacy_unknown_bare() {
        let (state, c, request) = repeat_fixture(false);
        let pending = invoke_repeat(&state, &c, &request);
        assert_eq!(pending.reply["pending"], true, "{}", pending.reply);
        let engine = answer(&pending, &c, Completion::Missing);
        let stated = answer(
            &pending,
            &c,
            Completion::Result {
                result: json!({"_meta":{"test/outcome":"unknown"},"isError":true}),
            },
        );
        for ended in [&engine, &stated] {
            let repeated = invoke_repeat(&ended.state, &c, &request);
            assert_eq!(repeated.reply, ended.reply);
            assert!(repeated.external_intents.is_empty());
        }
        assert_ending(&engine, &c, "unknown", Some("OPS-007"));
        assert_cause(
            &engine.reply,
            "OPS-007",
            "test.core#reserved.v1 for tool management.reserved.v1 ended unknown",
        );
        assert_ending(&stated, &c, "unknown", None);
        let mut legacy = engine.state;
        legacy.invocations[0]
            .as_object_mut()
            .unwrap()
            .remove("cause");
        assert_eq!(invoke_repeat(&legacy, &c, &request).reply, stated.reply);
    }

    #[test]
    fn filtered_redispatch_restores_failed_outcome_diagnostic() {
        let (state, c, request) = repeat_fixture(true);
        let pending = invoke_repeat(&state, &c, &request);
        assert_eq!(pending.reply["pending"], true, "{}", pending.reply);
        let ended = answer(
            &pending,
            &c,
            rpc(-32021, json!({"requiredCapabilities":{"sampling":{}}})),
        );
        assert_ending(&ended, &c, "failed", Some("IFC-047"));
        let redispatched = invoke_repeat(&ended.state, &c, &request);
        assert_eq!(redispatched.reply["attempt"], 2, "{}", redispatched.reply);
        let restriction = redispatched.state.invocations[0]["filters"][0].clone();
        assert!(restriction.is_object());
        let filtered = answer(
            &redispatched,
            &c,
            Completion::Result {
                result: json!({"_meta":{"test/reason":"unavailable","test/restriction":restriction},"isError":true}),
            },
        );
        assert_eq!(filtered.reply["_meta"]["test/reason"], "unavailable");
        let restored = &filtered.state.invocations[0];
        assert_eq!(restored["state"], "failed");
        assert_eq!(restored["cause"], ended.state.invocations[0]["cause"]);
        // Under OPS-024 a matching failed request dispatches again. Exercise the
        // stored reply path directly for the restored outcome.
        let mut reply = ended_reply(&c, restored, &restored["contract"], "failed");
        explain::first_text(&mut reply, &c);
        assert_eq!(reply, ended.reply);
    }

    #[test]
    fn first_filter_refusal_counts_actions_for_the_waiting_interface() {
        let (setup, _) = interrupted_setup();
        let (mut state, c, request) = repeat_fixture(true);
        let subject = setup
            .state
            .objects
            .iter()
            .find(|o| o["id"] == "o1")
            .unwrap();
        state.objects.push(subject.clone());
        let mut pending = invoke_repeat(&state, &c, &request);
        assert_eq!(pending.reply["pending"], true);
        let restriction = pending.state.invocations[0]["filters"][0].clone();
        assert!(restriction.is_object());
        pending
            .state
            .invocations
            .push(setup.state.invocations[0].clone());
        let result = Completion::Result {
            result: json!({"_meta":{"test/reason":"refused","test/restriction":restriction},"isError":true}),
        };
        let refused = answer(&pending, &c, result.clone());
        assert_eq!(refused.state.invocations.len(), 1);
        assert_eq!(refused.state.invocations[0]["id"], "s1");
        assert_eq!(refused.reply["_meta"][mark(&c, "reason")], "refused");
        assert_eq!(refused.reply["_meta"][mark(&c, "unanswered")], 1);
        assert_eq!(refused.request_replies[0].reply, refused.reply);

        pending.state.invocations[0]["sender"] = json!("agent");
        let delegated = answer(&pending, &c, result);
        assert_eq!(delegated.reply["_meta"][mark(&c, "reason")], "refused");
        assert!(
            delegated.reply["_meta"]
                .get(mark(&c, "unanswered"))
                .is_none()
        );
        assert_eq!(delegated.request_replies[0].reply, delegated.reply);
    }

    fn fence_fixture() -> (Transition, Context, Evidence) {
        let (mut t, c) = recording_fixture("write.v1", json!({"type":"object"}), json!({}));
        t.state.invocations[0] = json!({
            "id":"invocation-id", "target":"resource-id", "state":"unknown", "attempt":2,
            "implementation":{"source":"project","item":{"command":"server"}},
            "held_target":{}, "attempt_fences":{
                "1":{"fenced":false,"builtin_run":true},
                "2":{"fenced":false,"builtin_run":true}
            }
        });
        let evidence = Evidence {
            time: "2026-11-11T12:00:00Z".into(),
            ids: vec![],
            revisions: vec![],
            suffixes: vec![],
        };
        (t, c, evidence)
    }

    #[test]
    fn fence_requires_every_attempt_and_late_answers_never_settle_unknown() {
        let (mut t, c, evidence) = fence_fixture();
        for attempt in [2, 1] {
            complete(
                &mut t,
                &evidence,
                "invocation-id",
                attempt,
                Completion::Result {
                    result: json!({"resultType":"complete","task":"ordinary data","content":[]}),
                },
                &c,
            )
            .unwrap();
            assert_eq!(t.reply, json!({"ignored":true}));
            assert_eq!(t.state.invocations[0]["state"], "unknown");
            assert_eq!(fenced(&t.state.invocations[0]), attempt == 1);
            assert!(t.records.is_empty());
            assert!(t.external_intents.is_empty());
        }
        assert!(
            t.decisions
                .iter()
                .filter(|d| d.action == "ignore-late-completion")
                .all(|d| d.promises.iter().any(|id| id == "OPS-014"))
        );
    }

    #[test]
    fn missing_task_creation_and_input_required_fence_nothing() {
        for completion in [
            Completion::Missing,
            Completion::Result {
                result: json!({"resultType":"task","task":{"taskId":"pending"}}),
            },
            Completion::Result {
                result: json!({"resultType":"input_required"}),
            },
            Completion::Result {
                result: json!({"resultType":"unrecognized"}),
            },
        ] {
            let (mut t, c, evidence) = fence_fixture();
            complete(&mut t, &evidence, "invocation-id", 2, completion, &c).unwrap();
            assert_eq!(
                t.state.invocations[0]["attempt_fences"]["2"]["fenced"],
                false
            );
            assert!(!fenced(&t.state.invocations[0]));
        }
    }

    #[test]
    fn legacy_attempts_are_unfenced_and_new_dispatch_keeps_earlier_marks() {
        let (t, _, _) = fence_fixture();
        let mut old = t.state.invocations[0].clone();
        old.as_object_mut().unwrap().remove("attempt_fences");
        assert!(!fenced(&old));
        let mut next = old.clone();
        next["attempt"] = json!(3);
        start_attempt(&mut next, Some(&old));
        fence_attempt(&mut next, 3);
        assert!(!fenced(&next));
        fence_attempt(&mut next, 1);
        assert!(!fenced(&next));
        fence_attempt(&mut next, 2);
        assert!(fenced(&next));
        let mut another = next.clone();
        another["attempt"] = json!(4);
        start_attempt(&mut another, Some(&next));
        assert_eq!(another["attempt_fences"]["1"]["fenced"], true);
        assert!(!fenced(&another));
    }

    #[test]
    fn recovery_missing_and_unconfirmed_cancel_never_fence_engine_work() {
        for ending in ["recover", "missing", "cancel"] {
            let (mut t, c) = recording_fixture("write.v1", json!({"type":"object"}), json!({}));
            let v = &mut t.state.invocations[0];
            v["target"] = json!("resource-id");
            v["state"] = json!("in-progress");
            v["attempt"] = json!(1);
            v["plain"] = json!(false);
            v["contract"] = json!({"spec":{}});
            v["implementation"] = json!({"source":"engine"});
            v["attempt_fences"] = json!({"1":{"fenced":false,"builtin_run":false}});
            let evidence = Evidence {
                time: "2026-11-11T12:00:00Z".into(),
                ids: vec![
                    "00000000-0000-4000-8000-000000000001".into(),
                    "00000000-0000-4000-8000-000000000002".into(),
                ],
                revisions: vec![],
                suffixes: vec![],
            };
            match ending {
                "recover" => recover(&mut t, &evidence, &c).unwrap(),
                "missing" => complete(
                    &mut t,
                    &evidence,
                    "invocation-id",
                    1,
                    Completion::Missing,
                    &c,
                )
                .unwrap(),
                _ => cancel(&mut t, &evidence, "invocation-id", None, &c).unwrap(),
            }
            assert_eq!(t.state.invocations[0]["state"], "unknown");
            assert!(!fenced(&t.state.invocations[0]), "{ending}");
            assert!(!t.decisions.iter().any(|d| d.action.starts_with("fence-")));
        }
    }

    #[test]
    fn parameter_repair_fences_only_when_the_resent_call_is_answered() {
        for answered in [false, true] {
            let (mut t, c) = recording_fixture("read.v1", json!({"type":"object"}), json!({}));
            let v = &mut t.state.invocations[0];
            v["target"] = json!("resource-id");
            v["state"] = json!("in-progress");
            v["attempt"] = json!(1);
            v["plain"] = json!(true);
            v["contract"] = json!({"spec":{}});
            v["implementation"] = json!({"source":"project","item":{"command":"server"}});
            start_attempt(v, None);
            let evidence = Evidence {
                time: "2026-11-12T12:00:00Z".into(),
                ids: vec![
                    "00000000-0000-4000-8000-000000000001".into(),
                    "00000000-0000-4000-8000-000000000002".into(),
                ],
                revisions: vec![],
                suffixes: vec![],
            };
            complete(
                &mut t,
                &evidence,
                "invocation-id",
                1,
                Completion::ProtocolError {
                    code: -32020,
                    data: Value::Null,
                },
                &c,
            )
            .unwrap();
            assert_eq!(t.state.invocations[0]["state"], "in-progress");
            assert!(!fenced(&t.state.invocations[0]));
            assert_eq!(t.external_intents[0]["kind"], "refresh-tools-and-call");
            t.external_intents.clear();
            let completion = if answered {
                Completion::ProtocolError {
                    code: -32602,
                    data: Value::Null,
                }
            } else {
                Completion::Missing
            };
            complete(&mut t, &evidence, "invocation-id", 1, completion, &c).unwrap();
            assert_eq!(fenced(&t.state.invocations[0]), answered);
            assert_eq!(
                t.state.invocations[0]["state"],
                if answered { "failed" } else { "unknown" }
            );
            assert!(t.external_intents.is_empty());
        }
    }

    #[test]
    fn builtin_stop_fences_only_dispatches_already_sent_to_that_builtin_run() {
        let (mut t, c, evidence) = fence_fixture();
        let mut remote = t.state.invocations[0].clone();
        remote["id"] = json!("remote-id");
        remote["implementation"]["item"] = json!({"remote":{"url":"https://example.invalid"}});
        remote["attempt"] = json!(1);
        start_attempt(&mut remote, None);
        let mut provider = t.state.invocations[0].clone();
        provider["id"] = json!("provider-id");
        provider["held_target"]["runtime"] = json!({"id":"provider"});
        start_attempt(&mut provider, None);
        let mut other = t.state.invocations[0].clone();
        other["target"] = json!("other-run");
        t.state.invocations.extend([remote, other, provider]);
        let stopped = crate::step(
            &t.state,
            crate::Input::BuiltinRunStopped {
                evidence,
                object: "resource-id".into(),
            },
            &c,
        )
        .unwrap();
        assert!(fenced(&stopped.state.invocations[0]));
        assert!(!fenced(&stopped.state.invocations[1]));
        assert!(!fenced(&stopped.state.invocations[2]));
        assert!(!fenced(&stopped.state.invocations[3]));
        assert_eq!(stopped.state.invocations[0]["state"], "unknown");
        assert!(stopped.records.is_empty());
        assert!(stopped.external_intents.is_empty());
        let mut next = stopped.state.invocations[0].clone();
        next["attempt"] = json!(3);
        start_attempt(&mut next, Some(&stopped.state.invocations[0]));
        assert!(!fenced(&next));
    }

    fn recording_fixture(
        operation: &str,
        input_schema: Value,
        arguments: Value,
    ) -> (Transition, Context) {
        let core = "test.core";
        let scope = schemas::Scope {
            address: core.into(),
            version: None,
            manifest: BTreeMap::new(),
            exports: None,
            contracts: BTreeMap::from([
                (
                    "recording.v1".into(),
                    json!({"kind":"Type","spec":{"records":{"inputs":true,"outputs":true}}}),
                ),
                (
                    operation.into(),
                    json!({"kind":"Operation","spec":{"inputSchema":input_schema}}),
                ),
                (
                    "secret-alias.v1".into(),
                    json!({"kind":"Schema","spec":{"$ref":format!("{core}#secret-reference.v1")}}),
                ),
            ]),
        };
        let env = schemas::Environment {
            scopes: BTreeMap::from([("core".into(), scope)]),
        };
        let c = Context {
            builtin: None,
            core_package: core.into(),
            meta_prefix: "test/".into(),
            templates: BTreeMap::new(),
            record_source: "urn:test".into(),
            core_environment: env.clone(),
        };
        let target =
            json!({"type":"#recording.v1","contract_scope":"core","schema_environment":env});
        let mut state = crate::State::default();
        state
            .objects
            .push(json!({"id":"resource-id","kind":"Resource"}));
        state.invocations.push(json!({"id":"invocation-id","key":"key","operation":format!("{core}#{operation}"),"held_target":target,"arguments":arguments}));
        (
            Transition {
                state,
                decisions: vec![],
                records: vec![],
                external_intents: vec![],
                reply: Value::Null,
                request_replies: vec![],
                input_required: vec![],
            },
            c,
        )
    }

    #[test]
    fn records_exclude_run_secrets_and_identity_but_retain_ordinary_inputs() {
        let (t, c) = recording_fixture(
            "run.v1",
            json!({"type":"object"}),
            json!({"secrets":{"token":"synthetic-secret"},"identity":{"proof":"synthetic-proof"},"instance":"instance-id"}),
        );
        let mut data = json!({"invocation":"invocation-id"});
        record_content(&t, &c, &mut data).unwrap();
        assert_eq!(data["inputs"], json!({"instance":"instance-id"}));
        assert!(!data.to_string().contains("synthetic-"));
    }

    #[test]
    fn records_omit_secret_read_outputs_whatever_the_type_retains() {
        let (t, c) = recording_fixture(
            "get-secret.v1",
            json!({"type":"object"}),
            json!({"name":"token"}),
        );
        let mut data = json!({"invocation":"invocation-id","outputs":{"value":"synthetic-secret"}});
        record_content(&t, &c, &mut data).unwrap();
        assert_eq!(data["inputs"], json!({"name":"token"}));
        assert!(data.get("outputs").is_none());
        assert!(!data.to_string().contains("synthetic-secret"));
    }

    #[test]
    fn record_secret_references_follow_aliases_and_keep_only_resolved_values() {
        let (t, c) = recording_fixture(
            "read.v1",
            json!({"type":"object","properties":{
                "credential":{"$ref":"#secret-alias.v1"},
                "known":{"$ref":"#secret-alias.v1"},
                "ordinary":{"type":"string"}
            }}),
            json!({"credential":{"kind":"Resource","name":"synthetic-name"},
            "known":{"kind":"Resource","id":"resource-id"},"ordinary":"keep"}),
        );
        let mut data = json!({"invocation":"invocation-id"});
        record_content(&t, &c, &mut data).unwrap();
        assert_eq!(
            data["inputs"],
            json!({"known":{"kind":"Resource","id":"resource-id"},"ordinary":"keep"})
        );
        assert_eq!(data["inputsExcluded"][0]["path"], "/credential");
        assert_eq!(data["inputsExcluded"][0]["ruleId"], "SEC-004");
        assert!(!data.to_string().contains("synthetic-name"));
    }
    #[test]
    fn required_capabilities_distinguish_extension_and_unidentified_errors() {
        let (_, c) = recording_fixture("read.v1", json!({"type":"object"}), json!({}));
        assert_eq!(
            capability_error(
                &json!({"requiredCapabilities":{"extensions":{
                    "test/implementation-interface":{"revision":"v0"}
                }}}),
                &c
            ),
            Some(true)
        );
        assert_eq!(
            capability_error(&json!({"requiredCapabilities":{"sampling":{}}}), &c),
            Some(false)
        );
        for data in [
            json!({}),
            json!({"requiredCapabilities":null}),
            json!({"requiredCapabilities":{}}),
            json!({"requiredCapabilities":{"extensions":{}}}),
            json!({"requiredCapabilities":{"sampling":null}}),
            json!({"requiredCapabilities":{"extensions":{"other":null}}}),
        ] {
            assert_eq!(capability_error(&data, &c), None);
        }
    }
}
