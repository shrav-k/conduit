//! Checks applied to every step result, before expectations.
use super::*;

pub(super) struct Checks {
    record_validator: jsonschema::Validator,
    diagnostic_validator: jsonschema::Validator,
    condition_validator: jsonschema::Validator,
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

        Ok(Self {
            record_validator,
            diagnostic_validator,
            condition_validator,
        })
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
