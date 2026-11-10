//! Invocation state is durable core state, separate from records. Intents are only plans:
//! the embedding shell persists the transition before carrying out any call (DL2).
use crate::{
    Allocation, Context, Evidence, Result, Transition, access, decision, packages, schemas,
};
use serde::Deserialize;
use serde_json::{Value, json};

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
    rule: &str,
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
        blocks.push(json!({"type":"text","text":status}));
    }
    blocks.extend(content.as_array().into_iter().flatten().cloned());
    let mut reply = json!({"_meta":meta,"isError":error,"content":blocks});
    if !error && let Some(outputs) = outputs {
        reply["structuredContent"] = outputs;
    }
    if matches!(reason, Some("invalid" | "conflict")) {
        reply["structuredContent"] = json!({"diagnostics":[{"ruleId":rule,"level":"error","message":{"text":format!("The request reports {status}.")}}]});
    }
    reply
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
        if ty["spec"]["records"][setting] != true {
            continue;
        }
        if setting == "outputs" && op == c.qualified("get-secret.v1") {
            continue;
        }
        if let Some(mut value) = value {
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
    } else {
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
    t.reply = envelope(
        c,
        None,
        Some(reason),
        id,
        rule,
        None,
        json!([]),
        restriction,
    );
    if rule == "ACC-001" || rule == "TPL-013" {
        t.reply = json!({"error":{"code":-32602,"message":"The tool is not presented.","data":{mark(c,"reason"):reason}}});
    }
    if rule == "LIF-037" {
        let args = &route["arguments"];
        if let Some(o) = access::object(&t.state, args.get("object").unwrap_or(&args["instance"])) {
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
    end(t, a, c, route, None, Some(reason), id)
}
fn intent(v: &Value, c: &Context) -> Value {
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
            ("conduit", &v["conduit"]),
            ("filters", &v["filters"]),
            ("config", &v["config"]),
            ("revision", &v["revision"]),
        ] {
            meta[mark(c, key)] = value.clone();
        }
        if !v["key"].is_null() {
            meta[mark(c, "idempotencyKey")] = v["key"].clone();
        }
        call["params"]["_meta"] = meta;
    }
    json!({"kind":"dispatch","invocation":v["id"],"attempt":v["attempt"],"target":v["target"],"implementation":implementation,"call":call})
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
        t.reply = envelope(
            c,
            None,
            Some("invalid"),
            None,
            "IFC-036",
            None,
            json!([]),
            None,
        );
        t.decisions.push(decision(
            "unreadable-unidentified-request",
            &["IFC-036", "REC-001", "OPS-023"],
            &[],
            json!({}),
            vec![],
        ));
        return Ok(());
    }
    let v = access::evaluate(&t.state, request, c, true)?;
    let mut route = v.route;
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
    record(t, a, c, "access-decision", access_data)?;
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
    let target = crate::registration::publication_object(
        access::object(&t.state, &route["target"]).ok_or("missing target")?,
        c,
    )?;
    let op = route["operation"].as_str().ok_or("missing operation")?;
    let (_, _, contract) = schemas::operation(&target, op, &c.core_package)?;
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
        if let Some(message) = crate::retirement::blocking_diagnostic(&t.state, &route, rule) {
            t.reply["structuredContent"]["diagnostics"][0]["message"]["text"] = json!(message);
        }
        return Ok(());
    }
    if (previous.is_none() || redispatch)
        && let Some((reason, rule)) = crate::records::preflight(&t.state, &route, c, a)?
    {
        return reject(t, a, c, &route, reason, rule, None, None);
    }
    // OPS-023 puts ordinary unavailability after conflicts, including the repeat index.
    let available = access::evaluate(&t.state, request, c, false)?;
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
        t.reply = envelope(
            c,
            Some(outcome),
            None,
            Some(&old["id"]),
            "",
            contract["spec"]
                .get("outputSchema")
                .map(|_| old["retained"].get("outputs").cloned().unwrap_or(json!({}))),
            old["retained"].get("content").cloned().unwrap_or(json!([])),
            None,
        );
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
    if op == c.qualified("register-replacement.v1") {
        let parent = access::object(&t.state, &route["arguments"]["instance"])
            .ok_or("replacement parent absent")?
            .clone();
        let mut trial = t.clone();
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
            return reject(t, a, c, &route, "invalid", rule, None, None);
        }
    }
    // Validate reconciliation's state after authority/schema, before allocating or dispatching.
    let reconcile = op == c.qualified("reconcile.v1");
    let reconciled = if reconcile {
        let i = t
            .state
            .invocations
            .iter()
            .position(|v| v["id"] == route["arguments"]["invocation"]);
        let Some(i) = i else {
            return reject(t, a, c, &route, "invalid", "OPS-029", None, None);
        };
        if route["arguments"].get("revision").is_some()
            && t.state.invocations[i]["contract"]["spec"]
                .get("changes")
                .is_none()
        {
            return reject(t, a, c, &route, "invalid", "OPS-031", None, None);
        }
        if t.state.invocations[i]["state"] != "unknown" {
            return reject(t, a, c, &route, "conflict", "OPS-030", None, None);
        }
        if t.state.invocations[i]["contract"]["spec"]
            .get("changes")
            .is_some()
        {
            return Err("control reconciliation requires REV-010 lifecycle work".into());
        }
        Some(i)
    } else {
        None
    };
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
            t.reply["structuredContent"]["diagnostics"][0]["message"]["text"] = json!(
                "install fetch requires an absolute local file URL; https fetching and package-source providers are unfinished"
            );
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
        let mut trial = t.clone();
        crate::archives::install(
            &mut trial,
            input,
            t.state.preinstalled_snapshots.clone(),
            c.builtin.clone().ok_or("missing built-in project")?,
            c,
        )?;
        if trial.reply["accepted"] != true {
            let reason = trial.reply["reason"].as_str().unwrap_or("invalid");
            let rule = trial.reply["diagnostics"]
                .as_array()
                .and_then(|d| d.first())
                .and_then(|d| d["ruleId"].as_str())
                .unwrap_or(if reason == "conflict" {
                    "PKG-025"
                } else {
                    "PKG-024"
                });
            t.decisions
                .extend(trial.decisions.into_iter().skip(t.decisions.len()));
            reject(t, a, c, &route, reason, rule, None, None)?;
            if trial.reply["diagnostics"]
                .as_array()
                .is_some_and(|d| !d.is_empty())
            {
                t.reply["structuredContent"] = json!({"diagnostics":trial.reply["diagnostics"]});
            }
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
    {
        return Err("engine operation belongs to its lifecycle or records transition".into());
    }
    let plain = implementation["item"]["plain"] == true;
    let mut invocation = json!({"id":id,"key":key,"sender":route["consumer"],"caller":route["caller"],"target":route["target"],"conduit":route["conduit"],"operation":op,"arguments":route["arguments"],"digest":digest,"filters":route["filters"],"state":"in-progress","plain":plain,"contract":contract,"held_target":target,"implementation":implementation,"needs_result":plain || contract["spec"].get("outputSchema").is_some(),"attempt":previous.as_ref().and_then(|v|v["attempt"].as_u64()).unwrap_or(0)+1,"wire_retried":false,"config":target["config"],"revision":target["revision"]});
    let related = referenced_objects(&t.state, &target, op, &route["arguments"], c)?;
    if !related.is_empty() {
        invocation["input_objects"] = json!(related);
    }
    if let Some(serving) = &request.serving {
        invocation["serving"] = json!(serving);
    }
    if let Some(parent) = parent {
        invocation["setup_parent"] = parent["id"].clone();
        invocation["serving"] = parent["id"].clone();
        invocation["caller"] = parent["caller"].clone();
        invocation["filters"] = parent["filters"].clone();
    }
    if let Some(old) = previous {
        invocation["previous"] = old;
    }
    let i = if let Some(i) = existing {
        t.state.invocations[i] = invocation.clone();
        i
    } else {
        t.state.invocations.push(invocation.clone());
        t.state.invocations.len() - 1
    };
    link_access(t, &id);
    record(
        t,
        a,
        c,
        "invocation-dispatched",
        json!({"target":route["target"],"caller":route["caller"],"consumer":route["consumer"],"conduit":route["conduit"],"operation":op,"invocation":id,"attempt":invocation["attempt"]}),
    )?;
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
        let args = &route["arguments"];
        let outcome = if args["occurred"] == true {
            "succeeded"
        } else {
            "failed"
        };
        t.state.invocations[reconciled]["state"] = json!(outcome);
        t.state.invocations[reconciled]
            .as_object_mut()
            .unwrap()
            .remove("retained");
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
            && t.state.invocations[reconciled]["operation"] == c.qualified("retire.v1")
        {
            let object = t.state.invocations[reconciled]["object"].clone();
            if access::object(&t.state, &object).is_some() {
                crate::retirement::remove(t, a, c, i, &object)?;
            }
        }
        finish(t, a, c, i, "succeeded", json!({}), json!([]))?;
    } else if let Some(trial) = installation {
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
        )?;
    } else if crate::records::handles(op, c) {
        crate::records::execute(t, a, c, i)?;
    } else if crate::lifecycle::handles(op, c) {
        crate::lifecycle::execute(t, a, c, i)?;
    } else {
        t.external_intents.push(intent(&invocation, c));
        t.reply = json!({"pending":true,"invocation":id,"attempt":invocation["attempt"]});
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

pub(crate) fn finish(
    t: &mut Transition,
    a: &mut Allocation<'_>,
    c: &Context,
    i: usize,
    outcome: &str,
    outputs: Value,
    content: Value,
) -> Result<()> {
    let v = &mut t.state.invocations[i];
    v["state"] = json!(outcome);
    v.as_object_mut().unwrap().remove("previous");
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
        "",
        v["contract"]["spec"]
            .get("outputSchema")
            .map(|_| outputs.clone()),
        content,
        None,
    );
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
    if v["attempt"] != attempt || v["state"] != "in-progress" {
        // Late or duplicate completions never change a settled or cancelled outcome.
        t.decisions.push(decision(
            "ignore-late-completion",
            &["OPS-013", "OPS-018", "IMP-007"],
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
    match completion {
        Completion::ProtocolError { code: -32020, .. } if v["wire_retried"] != true && readonly => {
            t.state.invocations[i]["wire_retried"] = json!(true);
            let mut call = intent(&v, c);
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
            // IFC-045 declares only Conduit's extension: every other named capability
            // is undeclared. Both IFC-008 and IFC-047 promise that nothing was performed.
            let extension = capability_error(&data, c).unwrap();
            t.state.invocations[i]["requiredCapabilities"] = data["requiredCapabilities"].clone();
            if extension {
                t.state.invocations[i]["extensionRevision"] = json!("v0");
            }
            t.decisions.push(decision(
                "capability-error-performed-nothing",
                &["OPS-007", if extension { "IFC-008" } else { "IFC-047" }],
                &[],
                data,
                vec![],
            ));
            outcome = "failed";
        }
        Completion::ProtocolError { .. } if plain => {
            if readonly {
                outcome = "failed";
            }
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
                            &json!({"target":v["target"],"caller":v["caller"],"consumer":v["sender"]}),
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
                        outputs = json!({});
                    }
                } else {
                    outputs = json!({});
                }
            }
        }
        _ => {}
    }
    if outcome != "succeeded" {
        outputs = json!({});
    }
    finish(t, &mut a, c, i, outcome, outputs, content)?;
    crate::lifecycle::child_ended(t, &mut a, c, i)
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
            r["isError"] == true
                && r["_meta"][mark(c, "outcome")] == "failed"
                && r["_meta"].get(mark(c, "reason")).is_none()
        });
    finish(
        t,
        &mut a,
        c,
        i,
        if failed { "failed" } else { "unknown" },
        json!({}),
        json!([]),
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
            finish(t, &mut a, c, i, "unknown", json!({}), json!([]))?;
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
    let mut v = json!({"id":id,"key":format!("{}:{op}",object["id"].as_str().unwrap()),"hook":true,"setup_parent":p["id"],"serving":p["id"],"sender":p["sender"],"caller":p["caller"],"target":object["id"],"operation":op,"arguments":{},"filters":p["filters"],"state":"in-progress","plain":implementation["item"]["plain"] == true,"contract":contract,"held_target":target,"implementation":implementation,"needs_result":false,"attempt":attempt,"config":object["config"],"revision":object["revision"]});
    if let Some(id) = p.get("retirement_of") {
        v["retirement_of"] = id.clone();
    }
    if let Some(i) = previous {
        t.state.invocations[i] = v.clone();
    } else {
        t.state.invocations.push(v.clone());
    }
    t.state.invocations[parent]["setup"]["phase"] = json!("hook-pending");
    record(
        t,
        a,
        c,
        "invocation-dispatched",
        json!({"target":object["id"],"invocation":id,"caller":p["caller"],"consumer":p["sender"],"serving":p["id"],"operation":op,"attempt":attempt}),
    )?;
    let mut call = intent(&v, c);
    if let Some(meta) = call["call"]["params"]["_meta"].as_object_mut() {
        meta.remove(&mark(c, "conduit"));
    }
    t.external_intents.push(call);
    t.reply = json!({"pending":true,"invocation":p["id"],"hook":id});
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

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
