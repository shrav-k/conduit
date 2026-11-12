//! Deterministic transitions. No filesystem, clock, network, process or ambient allocation.
//! The caller supplies the built-in templates, names, time, machine evidence and unique IDs.
//! Unsupported cases and broken embedding inputs are errors, never Conduit reasons.

pub mod access;
pub mod archives;
mod cascade;
#[cfg(feature = "differential")]
mod differential;
pub mod lifecycle;
pub mod packages;
pub mod records;
pub mod registration;
pub mod requests;
mod retirement;
mod revisions;
pub mod schemas;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

type Result<T> = std::result::Result<T, String>;

/// Assigned runner variables, embedded from the single names source at build time.
/// This keeps resolution pure and lets the shell use the very same names (MAN-018).
pub fn run_variables() -> &'static BTreeMap<String, String> {
    static NAMES: std::sync::OnceLock<BTreeMap<String, String>> = std::sync::OnceLock::new();
    NAMES.get_or_init(|| {
        let names = conduit_loader::read_bytes(
            conduit_loader::Syntax::Yaml,
            include_bytes!("../../../../spec/names.yaml"),
        )
        .expect("built-in names must be valid")[0]
            .to_json();
        [
            "engineUrlVariable",
            "identityDirVariable",
            "stateDirVariable",
        ]
        .into_iter()
        .map(|key| {
            (
                key.into(),
                names[key].as_str().expect("assigned variable").into(),
            )
        })
        .collect()
    })
}

/// Values read at the edge from the specification, not a second copy of its assigned names.
#[derive(Clone, Debug)]
pub struct Context {
    pub builtin: Option<packages::Project>,
    pub core_package: String,
    pub meta_prefix: String,
    pub templates: BTreeMap<String, Value>,
    pub record_source: String,
    pub core_environment: schemas::Environment,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub started: bool,
    pub objects: Vec<Value>,
    pub records: Vec<Value>,
    pub allocated_ids: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub record_gaps: Vec<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invocations: Vec<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub installed_packages: Vec<archives::InstalledPackage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preinstalled_snapshots: Vec<packages::Snapshot>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub time: String,
    pub ids: Vec<String>,
    #[serde(default)]
    pub revisions: Vec<String>,
    #[serde(default)]
    pub suffixes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Machine {
    pub hostname: String,
    pub labels: BTreeMap<String, String>,
    pub state: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub sender: String,
    pub caller: String,
    pub conduit: String,
    /// The actual Conduit target, distinct from an object named in operation inputs.
    pub target: String,
    /// Qualified operation reference, as held by the target.
    pub operation: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Input {
    ExportRecords {
        #[serde(default)]
        unreadable: Vec<String>,
    },
    Invoke {
        evidence: Evidence,
        request: access::AccessRequest,
        #[serde(default)]
        key: Option<String>,
        #[serde(default)]
        archive: Option<archives::ArchiveInput>,
    },
    Complete {
        evidence: Evidence,
        invocation: String,
        attempt: u64,
        completion: requests::Completion,
    },
    Cancel {
        evidence: Evidence,
        invocation: String,
        #[serde(default)]
        confirmation: Option<Value>,
    },
    SetupReport {
        evidence: Evidence,
        invocation: String,
        report: lifecycle::Report,
    },
    Recover {
        evidence: Evidence,
    },
    BuiltinRunStopped {
        evidence: Evidence,
        object: String,
    },
    ExpireOutputs {
        invocation: String,
    },
    Access {
        evidence: Evidence,
        request: access::AccessRequest,
    },
    ListTools {
        identity: access::Identity,
    },
    InstallArchive {
        archive: archives::ArchiveInput,
        snapshots: Vec<packages::Snapshot>,
        builtin: packages::Project,
    },
    RegisterProject {
        snapshots: Vec<packages::Snapshot>,
        builtin: packages::Project,
        project: packages::Project,
        evidence: Evidence,
        request: Request,
    },
    ResolveProject {
        snapshots: Vec<packages::Snapshot>,
        builtin: packages::Project,
        project: packages::Project,
    },
    ToolDefinition {
        consumer: String,
        conduit: String,
        operation: String,
    },
    CheckAuthored {
        environment: schemas::Environment,
        scope: String,
        documents: Vec<Value>,
    },
    EvaluateSchema {
        environment: schemas::Environment,
        scope: String,
        schema: Value,
        value: Value,
        mode: String,
    },
    FirstStart {
        evidence: Evidence,
        machine: Machine,
        #[serde(rename = "trustDomain")]
        trust_domain: String,
    },
    Submit {
        evidence: Evidence,
        request: Request,
        documents: Vec<Value>,
    },
    Request {
        evidence: Evidence,
        request: Request,
        /// Already-resolved object named by this lifecycle operation, not its target.
        object: String,
    },
}

#[derive(Clone, Debug, Serialize)]
pub struct Decision {
    pub action: String,
    pub promises: Vec<String>,
    /// Implementation-defined choices are never promise IDs or diagnostic rule IDs.
    pub implementation_choices: Vec<String>,
    pub facts: Value,
    pub affected: Vec<String>,
}

/// A final core reply for callers waiting on one dispatched request attempt.
/// It remains available even when the invocation is removed or a previous result restored.
#[derive(Clone, Debug, Serialize)]
pub struct RequestReply {
    pub invocation: String,
    pub attempt: u64,
    pub reply: Value,
}

#[derive(Clone, Debug, Serialize)]
pub struct Transition {
    pub state: State,
    pub decisions: Vec<Decision>,
    pub records: Vec<Value>,
    pub external_intents: Vec<Value>,
    pub reply: Value,
    pub request_replies: Vec<RequestReply>,
}

fn decision(
    action: &str,
    promises: &[&str],
    choices: &[&str],
    facts: Value,
    affected: Vec<String>,
) -> Decision {
    Decision {
        action: action.into(),
        promises: promises.iter().map(|s| (*s).into()).collect(),
        implementation_choices: choices.iter().map(|s| (*s).into()).collect(),
        facts,
        affected,
    }
}

struct Allocation<'a> {
    evidence: &'a Evidence,
    cursor: usize,
}
impl Allocation<'_> {
    fn next(&mut self, state: &mut State) -> Result<String> {
        let id = self
            .evidence
            .ids
            .get(self.cursor)
            .ok_or("exhausted ID allocations")?
            .clone();
        if !state.allocated_ids.insert(id.clone()) {
            return Err("reused ID allocation".into());
        }
        self.cursor += 1;
        Ok(id)
    }
    fn record(
        &mut self,
        state: &mut State,
        context: &Context,
        event: &str,
        data: Value,
    ) -> Result<Value> {
        let mut record = json!({"specversion":"1.0", "id": self.next(state)?,
            "source":context.record_source, "type": event, "time":self.evidence.time, "data":data});
        if let Some(subject) = record["data"].as_object_mut().unwrap().remove("subject") {
            record["subject"] = subject;
        }
        state.records.push(record.clone());
        Ok(record)
    }
}

impl Context {
    fn qualified(&self, name: &str) -> String {
        format!("{}#{name}", self.core_package)
    }
    fn template(&self, name: &str) -> Result<&Value> {
        self.templates
            .get(name)
            .ok_or_else(|| format!("missing built-in template {name}"))
    }
}

/// One atomic transition. Errors leave the caller's state untouched, including allocations.
pub fn step(state: &State, input: Input, context: &Context) -> Result<Transition> {
    // DL6: the borrowed entry leaves its caller's State intact on every error.
    step_owned(state.clone(), input, context)
}

/// DL2/DL6: consume the caller's State; an error consumes it as well.
pub fn step_owned(state: State, input: Input, context: &Context) -> Result<Transition> {
    #[cfg(feature = "differential")]
    let before = state.clone();
    let transition = step_body(state, input, context)?;
    #[cfg(feature = "differential")]
    differential::check(&before, &transition.state, context)?;
    Ok(transition)
}

fn step_body(state: State, input: Input, context: &Context) -> Result<Transition> {
    // DL2: vectors and the shell execute this same transition body.
    let mut transition = Transition {
        state,
        decisions: vec![],
        records: vec![],
        external_intents: vec![],
        reply: Value::Null,
        request_replies: vec![],
    };
    match input {
        Input::ExportRecords { unreadable } => records::export(&mut transition, &unreadable)?,
        Input::Invoke {
            evidence,
            request,
            key,
            archive,
        } => {
            if archive.is_some() {
                requests::invoke_archive(
                    &mut transition,
                    &evidence,
                    &request,
                    key,
                    archive,
                    context,
                )?;
            } else {
                requests::invoke(&mut transition, &evidence, &request, key, context)?;
            }
        }
        Input::Complete {
            evidence,
            invocation,
            attempt,
            completion,
        } => requests::complete(
            &mut transition,
            &evidence,
            &invocation,
            attempt,
            completion,
            context,
        )?,
        Input::Cancel {
            evidence,
            invocation,
            confirmation,
        } => requests::cancel(
            &mut transition,
            &evidence,
            &invocation,
            confirmation,
            context,
        )?,
        Input::SetupReport {
            evidence,
            invocation,
            report,
        } => {
            lifecycle::report(&mut transition, &evidence, &invocation, report, context)?;
        }
        Input::Recover { evidence } => requests::recover(&mut transition, &evidence, context)?,
        Input::BuiltinRunStopped { evidence, object } => {
            if let Some(subject) = transition
                .state
                .objects
                .iter_mut()
                .find(|o| o["id"] == object)
            {
                if subject.get("runtime").is_some() {
                    return Err("builtin runner cannot observe a provider-owned run".into());
                }
                lifecycle::observe_run(subject, "stopped", &evidence.time)?;
                transition.decisions.push(decision(
                    "observe-builtin-run-stopped",
                    &["OBS-002", "LIF-011"],
                    &[],
                    json!({"object":object,"state":"stopped"}),
                    vec![object.clone()],
                ));
            }
            requests::builtin_run_stopped(&mut transition, &object);
        }
        Input::ExpireOutputs { invocation } => requests::expire(&mut transition, &invocation)?,
        Input::Access { evidence, request } => {
            access::decide(&mut transition, &evidence, &request, context)?
        }
        Input::ListTools { identity } => access::list(&mut transition, &identity, context)?,
        Input::InstallArchive {
            archive,
            snapshots,
            builtin,
        } => {
            archives::install(&mut transition, archive, snapshots, builtin, context)?;
        }
        Input::RegisterProject {
            snapshots,
            builtin,
            project,
            evidence,
            request,
        } => {
            let mut documents = project.documents.clone();
            documents.sort_by_key(|d| {
                (
                    d["kind"].to_string(),
                    d["metadata"]["name"].to_string(),
                    d.to_string(),
                )
            });
            let arguments = json!({"documents":documents,"manifest":project.manifest});
            let entries = packages::catalog_entries(&transition.state, snapshots);
            registration::register(
                &mut transition,
                entries,
                builtin,
                project,
                &evidence,
                &request,
                context,
            )?;
            records::registration_boundary(
                &mut transition,
                &evidence,
                &request,
                arguments,
                context,
            )?;
        }
        Input::ResolveProject {
            snapshots,
            builtin,
            project,
        } => {
            let resolution = packages::Catalog::new(
                builtin,
                packages::catalog_entries(&transition.state, snapshots),
                &context.core_package,
            )?
            .resolve(&project)?;
            transition.reply = resolution.reply;
            transition.decisions = resolution.decisions;
        }
        Input::ToolDefinition {
            consumer,
            conduit,
            operation,
        } => {
            let (reply, d) = schemas::publish(
                &access::PublicationView::new(&transition.state, context),
                &consumer,
                &conduit,
                &operation,
                context,
            )?;
            transition.reply = reply;
            transition.decisions.push(d);
        }
        Input::CheckAuthored {
            environment,
            scope,
            documents,
        } => {
            let mut diagnostics = vec![];
            for document in documents {
                diagnostics.extend(environment.check_document(
                    &scope,
                    &document,
                    &context.core_package,
                )?);
            }
            transition.reply =
                json!({"accepted": diagnostics.is_empty(), "diagnostics": diagnostics});
            transition.decisions.push(decision(
                "check-authored-schemas",
                &["IFC-033", "SCH-009", "SCH-019", "SCH-016", "SCH-015"],
                &[],
                json!({"scope":scope}),
                vec![],
            ));
        }
        Input::EvaluateSchema {
            environment,
            scope,
            schema,
            value,
            mode,
        } => {
            environment.check_profile(&scope, &schema, &context.core_package)?;
            let filled = match mode.as_str() {
                "supplied" | "operation-inputs" => environment.fill(
                    &scope,
                    &schema,
                    &value,
                    mode == "operation-inputs",
                    &context.core_package,
                )?,
                "output" => value,
                _ => return Err("unsupported schema evaluation mode".into()),
            };
            let valid = environment.valid(&scope, &schema, &filled, &context.core_package)?;
            transition.reply = json!({"valid":valid,"value":filled});
            transition.decisions.push(decision(
                "evaluate-authored-schema",
                &["SCH-001", "SCH-009", "SCH-015", "SCH-016"],
                &[],
                json!({"scope":scope,"mode":mode}),
                vec![],
            ));
        }
        Input::FirstStart {
            evidence,
            machine,
            trust_domain,
        } => first_start(&mut transition, &evidence, machine, &trust_domain, context)?,
        Input::Submit {
            evidence,
            request,
            documents,
        } => {
            let management_authority = authorized_path(&transition.state, &request, context)?;
            let management = transition
                .state
                .objects
                .iter()
                .find(|o| o["from"] == context.qualified("management"))
                .ok_or("management is absent")?;
            if request.operation != context.qualified("register.v1")
                || documents.is_empty()
                || management["id"] != request.target
            {
                return Err("unsupported submission".into());
            }
            // ACC-033 row 1 requires a second open Conduit for each existing,
            // non-retired owning Pool. Check it before protected-template diagnostics.
            let mut pool_authority = true;
            for document in &documents {
                if document["kind"] != "Resource" {
                    continue;
                }
                let owner = document["owner"]
                    .as_str()
                    .ok_or("unsupported submission owner reference")?;
                let pool = transition
                    .state
                    .objects
                    .iter()
                    .find(|o| o["kind"] == "Pool" && (o["name"] == owner || o["id"] == owner))
                    .ok_or("unsupported submission owner")?;
                if pool["state"] != "retired" {
                    pool_authority &= transition
                        .state
                        .objects
                        .iter()
                        .any(|o| permits(o, &request.sender, &pool["id"], &request.operation));
                }
            }
            if !management_authority || !pool_authority {
                transition.decisions.push(decision(
                    "refuse-unauthorized-registration",
                    &["ACC-033", "OPS-023"],
                    &[],
                    json!({"request":request}),
                    vec![],
                ));
                end_request(
                    &mut transition,
                    &evidence,
                    &request,
                    None,
                    "refused",
                    context,
                )?;
                transition.reply = json!({"reason":"refused"});
                return Ok(transition);
            }
            let protected = ["system", "node", "management", "access", "root"];
            let mut diagnostics = vec![];
            for (index, document) in documents.iter().enumerate() {
                let from = document
                    .get("from")
                    .and_then(Value::as_str)
                    .ok_or("unsupported submission document")?;
                if !protected.iter().any(|name| from == context.qualified(name)) {
                    return Err("unsupported submission: not a protected core template".into());
                }
                diagnostics.push(json!({"ruleId":"FST-006", "level":"error",
                    "properties":{"document":index+1},
                    "locations":[{"logicalLocations":[{"fullyQualifiedName":"/from"}]}],
                    "message":{"text":format!("protected core template {from}")}}));
            }
            transition.decisions.push(decision(
                "reject-protected-instantiation",
                &["FST-006", "NAM-020", "SUB-003", "ACC-033", "OPS-023"],
                &[],
                json!({"documents":documents,"request":request}),
                vec![],
            ));
            end_request(
                &mut transition,
                &evidence,
                &request,
                None,
                "invalid",
                context,
            )?;
            transition.reply = json!({"reason":"invalid", "diagnostics":diagnostics});
        }
        Input::Request {
            evidence,
            request,
            object,
        } => {
            // Validate the supplied path. LIF-034 forbids these operations whether or
            // not that path grants access, so its refusal also covers missing authority.
            authorized_path(&transition.state, &request, context)?;
            let named = transition
                .state
                .objects
                .iter()
                .find(|o| o["id"] == object)
                .ok_or("unsupported named object")?;
            let protected = named["first_start"] == true;
            let refuse = protected
                && (["retire.v1", "unregister.v1"]
                    .iter()
                    .any(|op| request.operation == context.qualified(op))
                    || named["from"] == context.qualified("node")
                        && request.operation == context.qualified("declare-lost.v1"));
            if !refuse {
                return Err("unsupported lifecycle request".into());
            }
            transition.decisions.push(decision(
                "protect-first-start-object",
                &["LIF-034", "ACC-033", "OPS-023"],
                &[],
                json!({"request":request, "object":object, "first_start":protected}),
                vec![object.clone()],
            ));
            end_request(
                &mut transition,
                &evidence,
                &request,
                Some(&object),
                "refused",
                context,
            )?;
            transition.reply = json!({"reason":"refused"});
        }
    }
    Ok(transition)
}

fn first_start(
    t: &mut Transition,
    evidence: &Evidence,
    machine: Machine,
    trust_domain: &str,
    context: &Context,
) -> Result<()> {
    // FST-007: the embedding supplies randomness; the pure core validates the namespace.
    if !valid_trust_domain(trust_domain) {
        return Err("malformed trustDomain (FST-007)".into());
    }
    if t.state.started {
        if t.state
            .objects
            .iter()
            .find(|o| o["from"] == context.qualified("system"))
            .is_none_or(|o| o["status"]["trustDomain"] != trust_domain)
        {
            return Err("trustDomain never changes (FST-007)".into());
        }
        t.decisions.push(decision(
            "keep-first-start-identities",
            &["FST-001", "FST-005", "FST-007"],
            &["DL6"],
            json!({"started":true}),
            t.state
                .objects
                .iter()
                .filter_map(|o| o["id"].as_str().map(str::to_owned))
                .collect(),
        ));
        t.reply = json!({"started":false});
        return Ok(());
    }
    if !t.state.objects.is_empty() {
        return Err("first start requires an empty object store (DL6)".into());
    }
    let hostname = machine.hostname.trim().to_lowercase();
    if !["reachable", "unreachable"].contains(&machine.state.as_str())
        || hostname.is_empty()
        || hostname.len() > 63
        || machine.labels.get("kubernetes.io/hostname") != Some(&hostname)
        || !["kubernetes.io/os", "kubernetes.io/arch"]
            .iter()
            .all(|k| machine.labels.get(*k).is_some_and(|v| !v.is_empty()))
    {
        return Err("invalid first-start machine evidence (PRV-014)".into());
    }
    if evidence.revisions.len() != 8
        || evidence.revisions.iter().any(String::is_empty)
        || evidence.suffixes.len() != 3
        || evidence.suffixes.iter().any(|s| {
            s.len() != 5
                || !s
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
    {
        return Err("first start needs eight revisions and three five-character suffixes".into());
    }
    let mut allocations = Allocation {
        evidence,
        cursor: 0,
    };
    let ids = (0..8)
        .map(|_| allocations.next(&mut t.state))
        .collect::<Result<Vec<_>>>()?;
    let templates = [
        "system",
        "node",
        "management",
        "access",
        "root",
        "root-management",
        "root-access",
        "root-pool",
    ];
    let names = [
        "system".into(),
        "local".into(),
        "management".into(),
        "access".into(),
        "root".into(),
        format!("root-management-{}", evidence.suffixes[0]),
        format!("root-access-{}", evidence.suffixes[1]),
        format!("root-pool-{}", evidence.suffixes[2]),
    ];
    for (i, template_name) in templates.iter().enumerate() {
        let template = context.template(template_name)?;
        let mut object = json!({"id":ids[i], "revision":evidence.revisions[i], "kind":template["kind"],
            "name":names[i], "from":context.qualified(template_name), "state":"ready", "config":{}, "first_start":i < 5});
        if let Some(ty) = template["spec"].get("type") {
            object["type"] = ty.clone();
        }
        let mut promises = vec![
            "FST-001", "FST-002", "FST-003", "FST-005", "NAM-020", "REV-001",
        ];
        if [1, 2, 3].contains(&i) {
            object["owner"] = json!(ids[0]);
            object["placement"] = json!(ids[1]);
            promises.extend(["PLC-001", "PLC-002"]);
        }
        if i == 1 {
            object["status"] = json!({"hostname":hostname, "labels":machine.labels});
            promises.extend(["PLC-008", "PLC-026", "PRV-014"]);
        }
        if i == 0 {
            object["status"] = json!({"trustDomain":trust_domain});
            promises.push("FST-007");
        }
        if i >= 5 {
            let local_name = ["management", "access", "system"][i - 5];
            let declaration = &context.template("root")?["spec"]["conduits"][local_name];
            if declaration["from"] != context.qualified(template_name) {
                return Err("invalid built-in root declaration".into());
            }
            object["consumer"] = json!(ids[4]);
            object["target"] = json!(ids[if i == 7 { 0 } else { i - 3 }]);
            object["local_name"] = json!(local_name);
            object["capability"] = template["spec"]["capability"].clone();
            object["operations"] = template["spec"]["operations"].clone();
            object["open"] = json!(true);
            object["provenance"] = json!({"cause":"first-start"});
            promises.extend([
                "CON-002",
                "OBJ-003",
                "ACC-044",
                "ACC-024",
                if i == 5 {
                    "ACC-032"
                } else if i == 6 {
                    "ACC-022"
                } else {
                    "FLEET-005"
                },
            ]);
        }
        t.decisions.push(decision("make-ready-first-start-instance", &promises, if i >= 5 { &["DL6", "DL15"] } else { &["DL6"] },
            json!({"template":context.qualified(template_name), "machine":if i == 1 { json!(&machine) } else { Value::Null }}), vec![ids[i].clone()]));
        t.state.objects.push(object);
        for (action, previous, state) in [
            ("registered", Value::Null, "registered"),
            ("lifecycle-changed", json!("registered"), "ready"),
        ] {
            let record = allocations.record(
                &mut t.state,
                context,
                "object-change",
                json!({"target":ids[i],
                "cause":"first-start", "action":action, "previous":previous, "state":state}),
            )?;
            t.records.push(record);
        }
    }
    t.decisions.push(decision(
        "keep-first-start-records",
        &["REC-001", "REC-002", "REC-003", "REC-005"],
        &["DL8"],
        json!({"cause":"first-start", "source":context.record_source}),
        ids,
    ));
    t.state.started = true;
    t.reply = json!({"started":true});
    Ok(())
}

/// FST-007's SPIFFE trust-domain alphabet and length.
pub fn valid_trust_domain(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 255
        && value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b".-_".contains(&b))
}

fn permits(conduit: &Value, sender: &str, target: &Value, operation: &str) -> bool {
    conduit["kind"] == "Conduit"
        && access::id(&conduit["consumer"]) == &json!(sender)
        && access::id(&conduit["target"]) == target
        && conduit["open"] == true
        && conduit["state"] == "ready"
        && conduit["operations"]
            .as_array()
            .is_some_and(|ops| ops.contains(&json!(operation)))
}

/// Internal established path for the registration boundary. Tool calls use access::decide.
fn authorized_path(state: &State, request: &Request, _context: &Context) -> Result<bool> {
    let Some(sender) = state
        .objects
        .iter()
        .find(|o| o["id"] == request.sender && o.get("spec").is_none())
    else {
        return Ok(false);
    };
    if request.sender != request.caller
        || sender["kind"] != "Agent" && sender["kind"] != "Interface"
    {
        return Ok(false);
    }
    Ok(state.objects.iter().any(|p| {
        p["id"] == request.conduit
            && permits(
                p,
                &request.sender,
                &json!(request.target),
                &request.operation,
            )
    }))
}

fn end_request(
    t: &mut Transition,
    evidence: &Evidence,
    request: &Request,
    object: Option<&str>,
    reason: &str,
    context: &Context,
) -> Result<()> {
    let mut allocation = Allocation {
        evidence,
        cursor: 0,
    };
    for event in ["access-decision", "request-ended"] {
        let mut data = json!({"caller":request.caller,
            "consumer":request.sender, "conduit":request.conduit, "target":request.target,
            "operation":request.operation, "reason":reason, "dispatched":false});
        if let Some(object) = object {
            data["object"] = json!(object);
        }
        let record = allocation.record(&mut t.state, context, event, data)?;
        t.records.push(record);
    }
    t.decisions.push(decision(
        "record-undispatched-request",
        &["REC-001", "REC-002", "REC-003", "OPS-005"],
        &["DL8"],
        json!({"reason":reason}),
        vec![request.target.clone()],
    ));
    Ok(())
}
