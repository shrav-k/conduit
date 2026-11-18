//! Checks applied to every step result, before expectations.
use super::*;

pub(super) struct Checks {
    record_validator: jsonschema::Validator,
    diagnostic_validator: jsonschema::Validator,
    condition_validator: jsonschema::Validator,
    refusal_validator: jsonschema::Validator,
}
impl Checks {
    pub(super) fn new(spec: &Specification) -> Result<Self> {
        let record_validator = spec
            .contract_validator("record.v1")
            .map_err(|e| e.to_string())?;
        let diagnostic_validator = spec
            .contract_validator("diagnostic.v1")
            .map_err(|e| e.to_string())?;
        let condition_validator = spec
            .contract_validator("condition.v1")
            .map_err(|e| e.to_string())?;

        let refusal_validator = spec
            .contract_validator("refusal.v1")
            .map_err(|e| e.to_string())?;
        Ok(Self {
            record_validator,
            diagnostic_validator,
            condition_validator,
            refusal_validator,
        })
    }
    fn reply(&self, input: &Value, actual: &Value, c: &Context) -> Result<()> {
        if ![
            "access-fixture",
            "invoke",
            "request",
            "request-fixture",
            "lifecycle-fixture",
            "submit-files",
            "register-project",
            "registration-orders",
            "install-archive",
            "resolve-project",
            "complete",
            "cancel",
            "setup-report",
            "list-tools",
        ]
        .iter()
        .any(|kind| input["kind"] == *kind)
        {
            return Ok(());
        }
        let reply = &actual["reply"];
        if let Some((reason, pointer)) = reason(reply, c) {
            let diagnostics = actual.pointer(pointer).and_then(Value::as_array);
            if !diagnostics.is_some_and(|ds| {
                ds.iter()
                    .any(|d| d["level"] == "error" && d["ruleId"].as_str().is_some_and(rule_id))
            }) {
                return Err(format!(
                    "IFC-050: a reply with reason {reason} holds no error diagnostic with a ruleId at {pointer}"
                ));
            }
            let diagnostics = diagnostics.unwrap();
            if reason == "refused"
                && (diagnostics.len() != 1
                    || diagnostics[0].pointer("/properties/refusal").is_none())
            {
                return Err(format!(
                    "ACC-056: a refused reply holds {} diagnostics at {pointer}; it must hold exactly one",
                    diagnostics.len()
                ));
            }
            for diagnostic in diagnostics {
                self.diagnostic_validator
                    .validate(diagnostic)
                    .map_err(|e| format!("diagnostic.v1: {e}"))?;
                if let Some(refusal) = diagnostic.pointer("/properties/refusal") {
                    self.refusal_validator
                        .validate(refusal)
                        .map_err(|e| format!("refusal.v1: {e}"))?;
                }
            }
        }
        if reply["isError"] == true {
            let status = reply["_meta"]
                .get(format!("{}reason", c.meta_prefix))
                .or_else(|| reply["_meta"].get(format!("{}outcome", c.meta_prefix)))
                .and_then(Value::as_str)
                .unwrap_or("");
            let messages: Vec<&str> = reply["structuredContent"]["diagnostics"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|d| d["level"] == "error")
                .filter_map(|d| d["message"]["text"].as_str())
                .collect();
            let text = if messages.is_empty() {
                status.into()
            } else {
                format!("{status}: {}", messages.join(" "))
            };
            if reply["content"][0] != json!({"type":"text","text":text}) {
                return Err(format!(
                    "IFC-025: the first text block is {}; it must be {text}",
                    reply["content"][0]
                ));
            }
        }
        Ok(())
    }
    pub(super) fn run(
        &self,
        input: &Value,
        actual: &Value,
        state: &State,
        next: &State,
        context: &Context,
        known: &BTreeSet<String>,
    ) -> Result<()> {
        let kind = input["kind"].as_str().ok_or("input has no kind")?;
        // Boundary evidence validation also applies to submissions assembled above.
        if let Some(evidence) = input.get("evidence") {
            let uuid = conduit_loader::compile(&json!({"type":"string","format":"uuid"}))
                .map_err(|e| e.to_string())?;
            for id in evidence["ids"].as_array().ok_or("missing allocation IDs")? {
                uuid.validate(id).map_err(|e| e.to_string())?;
            }
            conduit_loader::compile(&json!({"type":"string","format":"date-time"}))
                .map_err(|e| e.to_string())?
                .validate(&evidence["time"])
                .map_err(|e| e.to_string())?;
        }
        self.reply(input, actual, context)?;
        if let Some(decisions) = actual["decisions"].as_array() {
            for d in decisions {
                let rules = d["promises"].as_array().ok_or("decision has no promises")?;
                if rules.is_empty()
                    || rules
                        .iter()
                        .any(|r| r.as_str().is_none_or(|s| !known.contains(s)))
                {
                    return Err("decision lacks a requiring known promise".into());
                }
            }
            for r in next.records.iter().chain(
                actual["records"]
                    .as_array()
                    .ok_or("transition has no records")?
                    .iter(),
            ) {
                self.record_validator
                    .validate(r)
                    .map_err(|e| format!("record.v1: {e}"))?;
                if ![
                    "access-decision",
                    "request-ended",
                    "invocation-dispatched",
                    "invocation-ended",
                    "provider-call",
                    "object-change",
                ]
                .iter()
                .any(|e| r["type"] == *e)
                {
                    return Err("undocumented record event type".into());
                }
                if r["type"] == "provider-call" {
                    for field in [
                        "serving",
                        "conduit",
                        "consumer",
                        "idempotencyKey",
                        "filters",
                    ] {
                        if r["data"].get(field).is_some() {
                            return Err(format!("PRV-005 / REC-002: provider record has {field}"));
                        }
                    }
                }
            }
            let appended = actual["records"].as_array().ok_or("missing records")?;
            let mut retained = if kind == "restore-state" && input["records_purged"] == true {
                vec![]
            } else {
                state.records.clone()
            };
            // A successful purge is the only core transition permitted to remove records.
            let purged: BTreeSet<String> = appended
                .iter()
                .filter(|r| r["data"]["action"] == "records-purged")
                .flat_map(|r| r["data"]["purged"].as_array().into_iter().flatten())
                .map(|r| format!("{}:{}", r["source"], r["id"]))
                .collect();
            retained.extend(appended.iter().cloned());
            retained.retain(|r| !purged.contains(&format!("{}:{}", r["source"], r["id"])));
            if next.records != retained {
                return Err("REC-003: records were changed or omitted from the transition".into());
            }
        }
        if actual.get("decisions").is_some() {
            records::transition(actual, state, next, context, input["key"].as_str())?;
        }
        for condition in next
            .objects
            .iter()
            .flat_map(|o| o["status"]["conditions"].as_array().into_iter().flatten())
        {
            self.condition_validator
                .validate(condition)
                .map_err(|e| format!("condition.v1: {e}"))?;
        }
        for diagnostics in [
            actual.get("diagnostics"),
            actual.get("reply").and_then(|r| r.get("diagnostics")),
        ]
        .into_iter()
        .flatten()
        {
            for d in diagnostics.as_array().ok_or("invalid diagnostic array")? {
                self.diagnostic_validator
                    .validate(d)
                    .map_err(|e| format!("diagnostic.v1: {e}"))?;
            }
        }

        Ok(())
    }
}

/// Reason and diagnostic place, measured from a step result, shared with portable assertions.
pub(super) fn reason<'a>(reply: &'a Value, c: &Context) -> Option<(&'a str, &'static str)> {
    if let Some(reason) = reply["reason"].as_str() {
        return Some((reason, "/reply/diagnostics"));
    }
    if let Some(reason) = reply["_meta"]
        .get(format!("{}reason", c.meta_prefix))
        .and_then(Value::as_str)
    {
        return Some((reason, "/reply/structuredContent/diagnostics"));
    }
    reply["error"]["data"]
        .get(format!("{}reason", c.meta_prefix))
        .and_then(Value::as_str)
        .map(|reason| (reason, "/reply/error/data/diagnostics"))
}
fn rule_id(rule: &str) -> bool {
    rule.split_once('-').is_some_and(|(prefix, digits)| {
        !prefix.is_empty()
            && prefix.bytes().all(|b| b.is_ascii_uppercase())
            && digits.len() == 3
            && digits.bytes().all(|b| b.is_ascii_digit())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Checks, Context) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let spec = Specification::read(&root.join("spec")).unwrap();
        let c = Context {
            builtin: None,
            core_package: spec.names().core_package.clone(),
            meta_prefix: "test/".into(),
            templates: BTreeMap::new(),
            record_source: "urn:test".into(),
            core_environment: conduit_core::schemas::Environment {
                scopes: BTreeMap::new(),
            },
        };
        (Checks::new(&spec).unwrap(), c)
    }
    #[test]
    fn refusal_causes_match_spec_table_verbatim() {
        let text = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../spec/conventions/access.yaml"),
        )
        .unwrap();
        for (rule, cause) in conduit_core::explain::REFUSAL_CAUSES {
            assert!(
                text.lines()
                    .any(|l| l == format!("    | {rule} | {cause} |")),
                "{rule}"
            );
        }
    }
    #[test]
    fn reason_checks_reject_each_fault_at_all_three_places() {
        let (checks, c) = fixture();
        let input = json!({"kind":"invoke"});
        for (base, place) in [
            (json!({"reason":"refused"}), "/reply/diagnostics"),
            (
                json!({"_meta":{"test/reason":"refused"}}),
                "/reply/structuredContent/diagnostics",
            ),
            (
                json!({"error":{"data":{"test/reason":"refused"}}}),
                "/reply/error/data/diagnostics",
            ),
        ] {
            for diagnostics in [
                Value::Null,
                json!([]),
                json!([{"ruleId":"bad","level":"error"}]),
                json!([{"ruleId":"ACC-001","level":"warning"}]),
            ] {
                let mut actual = json!({"reply":base});
                insert(&mut actual, place, diagnostics);
                assert_eq!(
                    checks.reply(&input, &actual, &c).unwrap_err(),
                    format!(
                        "IFC-050: a reply with reason refused holds no error diagnostic with a ruleId at {place}"
                    )
                );
            }
            for ds in [
                json!([{"ruleId":"ACC-001","level":"error"}]),
                json!([{"ruleId":"ACC-001","level":"error"},{"ruleId":"ACC-001","level":"error"}]),
            ] {
                let mut actual = json!({"reply":base});
                insert(&mut actual, place, ds.clone());
                assert_eq!(
                    checks.reply(&input, &actual, &c).unwrap_err(),
                    format!(
                        "ACC-056: a refused reply holds {} diagnostics at {place}; it must hold exactly one",
                        ds.as_array().unwrap().len()
                    )
                );
            }
            let mut actual = json!({"reply":base});
            insert(
                &mut actual,
                place,
                json!([{"ruleId":"ACC-001","level":"error","message":{"text":"cause"},"properties":{"refusal":{"tier":1,"operation":"tool","named":{},"cause":"Cause.","exists":true}}}]),
            );
            assert!(
                checks
                    .reply(&input, &actual, &c)
                    .unwrap_err()
                    .starts_with("refusal.v1: ")
            );
            insert(
                &mut actual,
                place,
                json!([{"ruleId":"ACC-001","level":"error","properties":{"refusal":{"tier":1}}}]),
            );
            assert!(
                checks
                    .reply(&input, &actual, &c)
                    .unwrap_err()
                    .starts_with("diagnostic.v1: ")
            );
        }
    }
    fn insert(actual: &mut Value, path: &str, value: Value) {
        let parts: Vec<_> = path.trim_start_matches('/').split('/').collect();
        let mut at = actual;
        for key in &parts[..parts.len() - 1] {
            if !at[*key].is_object() {
                at[*key] = json!({});
            }
            at = &mut at[*key];
        }
        at[parts[parts.len() - 1]] = value;
    }
    #[test]
    fn first_text_checks_final_messages_and_excludes_non_requests() {
        let (checks, c) = fixture();
        let mut actual = json!({"reply":{"isError":true,"_meta":{"test/outcome":"failed"},"content":[{"type":"text","text":"failed"}],"structuredContent":{"diagnostics":[{"ruleId":"LIF-021","level":"error","message":{"text":"final"}}]}}});
        assert_eq!(
            checks
                .reply(&json!({"kind":"complete"}), &actual, &c)
                .unwrap_err(),
            "IFC-025: the first text block is {\"text\":\"failed\",\"type\":\"text\"}; it must be failed: final"
        );
        actual["reply"]["content"][0]["text"] = json!("failed: final");
        checks
            .reply(&json!({"kind":"complete"}), &actual, &c)
            .unwrap();
        for kind in [
            "schema-case",
            "schema-admission",
            "schema-evaluation",
            "validate-values",
            "load",
            "read",
        ] {
            checks
                .reply(
                    &json!({"kind":kind}),
                    &json!({"reply":{"reason":"invalid"}}),
                    &c,
                )
                .unwrap();
        }
    }
}
