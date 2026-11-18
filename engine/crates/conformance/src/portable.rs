//! D36: cases added after order 046a exercise the public boundary.
use super::*;

#[derive(Default)]
pub(super) struct Controls {
    internal: BTreeSet<String>,
    exceptions: BTreeMap<String, Exception>,
}
struct Exception {
    initial: bool,
    steps: BTreeSet<usize>,
}
fn repair(problem: impl std::fmt::Display) -> String {
    format!(
        "conformance/internal-cases.yaml: {problem}. See conformance/README.md, \"Portable cases\"."
    )
}
fn members(value: &Value, keys: &[&str]) -> bool {
    value
        .as_object()
        .is_some_and(|o| o.len() == keys.len() && keys.iter().all(|k| o.contains_key(*k)))
}
impl Controls {
    pub(super) fn read(value: &Value, cases: &BTreeMap<&str, &Case>) -> Result<Self> {
        if !members(value, &["cases", "exceptions"]) {
            return Err(repair("supply an object with only cases and exceptions"));
        }
        let list = value["cases"]
            .as_array()
            .ok_or_else(|| repair("make cases a list of case IDs"))?;
        let mut controls = Self::default();
        let mut previous: Option<&str> = None;
        for id in list {
            let id = id
                .as_str()
                .ok_or_else(|| repair("make every cases entry a string"))?;
            if !cases.contains_key(id) {
                return Err(repair(format!(
                    "{id}: restore the missing case or remove its ID"
                )));
            }
            if previous.is_some_and(|p| p >= id) {
                return Err(repair(format!(
                    "{id}: sort cases in byte order without duplicates"
                )));
            }
            controls.internal.insert(id.into());
            previous = Some(id);
        }
        let list = value["exceptions"]
            .as_array()
            .ok_or_else(|| repair("make exceptions a list"))?;
        previous = None;
        for entry in list {
            if !members(entry, &["case", "initial", "steps", "state", "why"]) {
                return Err(repair(
                    "give each exception only case, initial, steps, state and why",
                ));
            }
            let id = entry["case"]
                .as_str()
                .ok_or_else(|| repair("give each exception a case ID string"))?;
            let case = cases.get(id).ok_or_else(|| {
                repair(format!(
                    "{id}: restore the exception's missing case or remove its exception"
                ))
            })?;
            if controls.internal.contains(id) {
                return Err(repair(format!(
                    "{id}: remove the exception for an internal case"
                )));
            }
            if previous.is_some_and(|p| p >= id) {
                return Err(repair(format!(
                    "{id}: sort exceptions by case in byte order without duplicates"
                )));
            }
            previous = Some(id);
            let initial = entry["initial"]
                .as_bool()
                .ok_or_else(|| repair(format!("{id}: make initial a boolean")))?;
            if !["recovery", "crash", "restore"]
                .iter()
                .any(|state| entry["state"] == *state)
            {
                return Err(repair(format!(
                    "{id}: state must be recovery, crash or restore"
                )));
            }
            if !entry["why"]
                .as_str()
                .is_some_and(|why| !why.trim().is_empty() && why.trim().ends_with(['.', '!', '?']))
            {
                return Err(repair(format!(
                    "{id}: give why a nonempty sentence explaining why requests cannot reach the state"
                )));
            }
            let steps = entry["steps"]
                .as_array()
                .ok_or_else(|| repair(format!("{id}: make steps a list of step numbers")))?;
            let mut allowed = BTreeSet::new();
            let mut last = 0;
            for number in steps {
                let n = number
                    .as_u64()
                    .and_then(|n| usize::try_from(n).ok())
                    .ok_or_else(|| {
                        repair(format!("{id}: give steps integer numbers starting from 1"))
                    })?;
                if n == 0 || n > case.steps.len() {
                    return Err(repair(format!(
                        "{id}: keep exception step numbers within 1 to {}",
                        case.steps.len()
                    )));
                }
                if n <= last {
                    return Err(repair(format!(
                        "{id}: sort steps ascending without duplicates"
                    )));
                }
                allowed.insert(n);
                last = n;
            }
            controls.exceptions.insert(
                id.into(),
                Exception {
                    initial,
                    steps: allowed,
                },
            );
        }
        Ok(controls)
    }
    pub(super) fn check_case(&self, case: &Case) -> Result<()> {
        if self.internal.contains(&case.id) {
            return Ok(());
        }
        let exception = self.exceptions.get(&case.id);
        if !matches!(case.initial, Initial::Empty) && !exception.is_some_and(|e| e.initial) {
            return Err("portable case: initial must be {\"recipe\":\"empty\"} unless a recovery, crash or restore exception permits it".into());
        }
        for (index, step) in case.steps.iter().enumerate() {
            let n = index + 1;
            let kind = step["input"]["kind"].as_str().unwrap_or("");
            let allowed = [
                "first-start",
                "install-archive",
                "resolve-project",
                "register-project",
                "registration-orders",
                "submit-files",
                "invoke",
                "complete",
                "setup-report",
                "cancel",
                "restore-state",
                "list-tools",
                "export-records",
                "load",
                "read",
            ];
            let special = ["lifecycle-fixture", "request-fixture"].contains(&kind)
                && exception.is_some_and(|e| e.steps.contains(&n));
            if !allowed.contains(&kind) && !special {
                return Err(format!(
                    "portable case: step {n} input kind {kind} is not a client or provider input"
                ));
            }
            let expect = &step["expect"];
            for key in [
                "object_facts",
                "invocation_facts",
                "object_count",
                "absent_objects",
                "tombstone_count",
                "tombstone_facts",
                "tombstones",
                "absent_tombstones",
                "objectIds",
                "inventory_facts",
                "stored_record_facts",
            ] {
                if expect.get(key).is_some() {
                    return Err(format!("portable case: step {n} may not assert {key}"));
                }
            }
            if ![
                "first-start",
                "submit-files",
                "register-project",
                "registration-orders",
                "install-archive",
                "resolve-project",
            ]
            .contains(&kind)
            {
                for key in ["state", "stateUnchanged", "required_promises"] {
                    if expect.get(key).is_some() {
                        return Err(format!(
                            "portable case: step {n} may not assert {key} outside a step that builds state"
                        ));
                    }
                }
            }
            for pointer in expect["absent"].as_array().into_iter().flatten() {
                if !pointer.as_str().is_some_and(|p| {
                    ["/reply", "/records", "/external_intents"]
                        .iter()
                        .any(|prefix| p.starts_with(prefix))
                }) {
                    return Err(format!(
                        "portable case: step {n} absent pointers must begin /reply, /records or /external_intents"
                    ));
                }
            }
            for pointer in step["capture"]
                .as_object()
                .into_iter()
                .flat_map(|m| m.values())
            {
                if !pointer.as_str().is_some_and(|p| {
                    ["/reply", "/records"]
                        .iter()
                        .any(|prefix| p.starts_with(prefix))
                }) {
                    return Err(format!(
                        "portable case: step {n} capture pointers must begin /reply or /records"
                    ));
                }
            }
        }
        Ok(())
    }
    pub(super) fn after_step(
        &self,
        id: &str,
        n: usize,
        expected: &Value,
        actual: &Value,
        c: &Context,
    ) -> Result<()> {
        if self.internal.contains(id) {
            return Ok(());
        }
        if let Some((reason, place)) = checks::reason(&actual["reply"], c) {
            let same = checks::reason(&expected["reply"], c)
                .is_some_and(|(r, p)| r == reason && p == place);
            let asserts_rule = expected
                .pointer(place)
                .and_then(Value::as_array)
                .and_then(|a| a.first())
                .is_some_and(|d| d["ruleId"].as_str().is_some());
            if !same || !asserts_rule {
                return Err(format!(
                    "portable case: step {n} ends with reason {reason} but does not assert it and its deciding ruleId"
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn case(kind: &str) -> Case {
        serde_json::from_value(json!({"id":"new","layers":["convention"],"promises":["ACC-001"],"title":"case","profile":"first-build","initial":{"recipe":"empty"},"files":{},"steps":[{"input":{"kind":kind},"expect":{"reply":{}}}]})).unwrap()
    }
    #[test]
    fn control_file_rejects_every_malformed_member() {
        let sample = case("invoke");
        let cases = BTreeMap::from([("new", &sample)]);
        let good = json!({"cases":[],"exceptions":[]});
        Controls::read(&good, &cases).unwrap();
        for (value, message) in [
            (json!([]), "supply an object with only cases and exceptions"),
            (
                json!({"cases":[],"exceptions":[],"extra":true}),
                "supply an object with only cases and exceptions",
            ),
            (
                json!({"cases":{},"exceptions":[]}),
                "make cases a list of case IDs",
            ),
            (
                json!({"cases":[1],"exceptions":[]}),
                "make every cases entry a string",
            ),
            (
                json!({"cases":["missing"],"exceptions":[]}),
                "restore the missing case or remove its ID",
            ),
            (
                json!({"cases":["new","new"],"exceptions":[]}),
                "sort cases in byte order without duplicates",
            ),
            (
                json!({"cases":[],"exceptions":{}}),
                "make exceptions a list",
            ),
            (
                json!({"cases":[],"exceptions":[{}]}),
                "give each exception only case, initial, steps, state and why",
            ),
        ] {
            let error = Controls::read(&value, &cases).err().unwrap();
            assert!(
                error.starts_with("conformance/internal-cases.yaml: ") && error.contains(message),
                "{error}"
            );
        }
        let entry = json!({"case":"new","initial":false,"steps":[1],"state":"recovery","why":"Requests cannot reach this state."});
        for (key, value, message) in [
            ("case", json!(1), "case ID string"),
            ("case", json!("missing"), "missing case"),
            ("initial", json!("no"), "initial a boolean"),
            (
                "state",
                json!("normal"),
                "state must be recovery, crash or restore",
            ),
            ("why", json!(""), "nonempty sentence"),
            ("steps", json!(true), "list of step numbers"),
            ("steps", json!(["1"]), "integer numbers starting from 1"),
            ("steps", json!([0]), "within 1 to 1"),
            ("steps", json!([2]), "within 1 to 1"),
            (
                "steps",
                json!([1, 1]),
                "sort steps ascending without duplicates",
            ),
        ] {
            let mut e = entry.clone();
            e[key] = value;
            let error = Controls::read(&json!({"cases":[],"exceptions":[e]}), &cases)
                .err()
                .unwrap();
            assert!(
                error.starts_with("conformance/internal-cases.yaml: ") && error.contains(message),
                "{error}"
            );
        }
        assert!(
            Controls::read(&json!({"cases":["new"],"exceptions":[entry]}), &cases)
                .err()
                .unwrap()
                .contains("exception for an internal case")
        );
        assert!(
            Controls::read(&json!({"cases":[],"exceptions":[entry,entry]}), &cases)
                .err()
                .unwrap()
                .contains("sort exceptions")
        );
        let a = case("invoke");
        let z = case("invoke");
        let cases = BTreeMap::from([("a", &a), ("z", &z)]);
        assert!(
            Controls::read(&json!({"cases":["z","a"],"exceptions":[]}), &cases)
                .err()
                .unwrap()
                .contains("byte order")
        );
    }
    #[test]
    fn portable_inputs_state_assertions_and_pointers_are_enforced() {
        let controls = Controls::default();
        for kind in [
            "access-fixture",
            "request-fixture",
            "lifecycle-fixture",
            "request",
            "expire-outputs",
            "schema-case",
            "schema-admission",
            "schema-evaluation",
            "validate-values",
            "future",
        ] {
            assert_eq!(
                controls.check_case(&case(kind)).unwrap_err(),
                format!(
                    "portable case: step 1 input kind {kind} is not a client or provider input"
                )
            );
        }
        for kind in [
            "first-start",
            "install-archive",
            "resolve-project",
            "register-project",
            "registration-orders",
            "submit-files",
            "invoke",
            "complete",
            "setup-report",
            "cancel",
            "restore-state",
            "list-tools",
            "export-records",
            "load",
            "read",
        ] {
            controls.check_case(&case(kind)).unwrap();
        }
        let mut c = case("invoke");
        c.initial = Initial::Snapshot {
            file: "fixture".into(),
        };
        assert!(
            controls
                .check_case(&c)
                .unwrap_err()
                .starts_with("portable case: initial must be")
        );
        for key in [
            "object_facts",
            "invocation_facts",
            "object_count",
            "absent_objects",
            "tombstone_count",
            "tombstone_facts",
            "tombstones",
            "absent_tombstones",
            "objectIds",
            "inventory_facts",
            "stored_record_facts",
        ] {
            let mut c = case("invoke");
            c.steps[0]["expect"][key] = json!([]);
            assert_eq!(
                controls.check_case(&c).unwrap_err(),
                format!("portable case: step 1 may not assert {key}")
            );
        }
        for key in ["state", "stateUnchanged", "required_promises"] {
            let mut c = case("invoke");
            c.steps[0]["expect"][key] = json!({});
            assert_eq!(
                controls.check_case(&c).unwrap_err(),
                format!(
                    "portable case: step 1 may not assert {key} outside a step that builds state"
                )
            );
            c.steps[0]["input"]["kind"] = json!("first-start");
            controls.check_case(&c).unwrap();
        }
        let mut c = case("invoke");
        c.steps[0]["expect"]["absent"] = json!(["/state"]);
        assert_eq!(
            controls.check_case(&c).unwrap_err(),
            "portable case: step 1 absent pointers must begin /reply, /records or /external_intents"
        );
        c.steps[0]["expect"]["absent"] = json!(["/reply/x", "/records/0", "/external_intents/0"]);
        controls.check_case(&c).unwrap();
        c.steps[0]["capture"] = json!({"x":"/state"});
        assert_eq!(
            controls.check_case(&c).unwrap_err(),
            "portable case: step 1 capture pointers must begin /reply or /records"
        );
        c.steps[0]["capture"] = json!({"x":"/records/0"});
        controls.check_case(&c).unwrap();
    }
    #[test]
    fn recovery_exception_lifts_only_listed_initial_and_fixture_rules() {
        let mut c = case("lifecycle-fixture");
        c.initial = Initial::Snapshot {
            file: "fixture".into(),
        };
        let cases = BTreeMap::from([("new", &c)]);
        let mut value = json!({"cases":[],"exceptions":[{"case":"new","initial":true,"steps":[1],"state":"recovery","why":"Requests cannot reach this state."}]});
        Controls::read(&value, &cases)
            .unwrap()
            .check_case(&c)
            .unwrap();
        value["exceptions"][0]["steps"] = json!([]);
        assert!(
            Controls::read(&value, &cases)
                .unwrap()
                .check_case(&c)
                .unwrap_err()
                .contains("lifecycle-fixture")
        );
        value["exceptions"][0]["state"] = json!("normal");
        assert!(
            Controls::read(&value, &cases)
                .err()
                .unwrap()
                .contains("state must be recovery, crash or restore")
        );
    }
    #[test]
    fn portable_reasons_assert_reason_and_first_deciding_rule_at_same_place() {
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
        let controls = Controls::default();
        let actual = json!({"reply":{"reason":"invalid","diagnostics":[{"ruleId":"OPS-029"}]}});
        for expected in [
            json!({"reply":{}}),
            json!({"reply":{"reason":"invalid"}}),
            json!({"reply":{"reason":"refused","diagnostics":[{"ruleId":"OPS-029"}]}}),
        ] {
            assert_eq!(
                controls
                    .after_step("new", 1, &expected, &actual, &c)
                    .unwrap_err(),
                "portable case: step 1 ends with reason invalid but does not assert it and its deciding ruleId"
            );
        }
        controls.after_step("new", 1, &actual, &actual, &c).unwrap();
    }
}
