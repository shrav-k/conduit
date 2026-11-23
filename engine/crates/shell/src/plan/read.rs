use super::*;
use crate::{Config, Result, author, boundary, server, user_config};
use conduit_core::{
    Context, Evidence, Input, Machine, Request, State,
    archives::{ArchiveInput, FetchCompletion},
    packages::{Catalog, Snapshot, contract, document_digest},
};
use rmcp::{
    RoleClient, ServiceExt,
    model::CallToolRequestParams,
    service::RunningService,
    transport::{
        StreamableHttpClientTransport, streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use std::{fs, io::ErrorKind, time::Duration};

const APPROVALS: &[&str] = &[
    "--reopen",
    "--recreate",
    "--replace",
    "--prune",
    "--adopt",
    "--with-resources",
    "--with-agents",
];

fn parse_approvals(text: &str) -> Option<BTreeSet<Approval>> {
    if text.is_empty() {
        return Some(BTreeSet::new());
    }
    text.split(',')
        .map(|item| {
            let (flag, operand) = item.split_once(' ')?;
            (APPROVALS.contains(&flag)
                && !operand.is_empty()
                && !operand.contains(char::is_whitespace))
            .then(|| Approval::new(flag, operand))
        })
        .collect()
}

struct Options {
    project: PathBuf,
    store: Option<PathBuf>,
    approvals: BTreeSet<Approval>,
}

impl Options {
    fn parse(args: Vec<String>, command: &str) -> Result<Self> {
        let mut project = None;
        let mut store = None;
        let mut args = args.into_iter();
        let mut approvals = BTreeSet::new();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                flag if APPROVALS.contains(&flag) => {
                    let operand = args
                        .next()
                        .filter(|name| {
                            !name.starts_with('-')
                                && !name.is_empty()
                                && !name.contains(char::is_whitespace)
                                && !name.contains(',')
                        })
                        .ok_or_else(|| format!("{flag} needs OBJECT"))?;
                    approvals.insert(Approval::new(flag, &operand));
                }
                "--store" if store.is_none() => {
                    let directory = args
                        .next()
                        .filter(|value| !value.starts_with('-'))
                        .ok_or("--store needs DIR")?;
                    store = Some(PathBuf::from(directory));
                }
                "--store" => return Err("--store requires one directory".into()),
                _ if !arg.starts_with('-') && project.is_none() => {
                    project = Some(PathBuf::from(arg))
                }
                _ => {
                    return Err(format!(
                        "unexpected argument {arg:?}; run conduit {command} --help"
                    ));
                }
            }
        }
        Ok(Self {
            project: project.unwrap_or_else(|| ".".into()),
            store,
            approvals,
        })
    }
}

pub async fn command(args: Vec<String>, repository: &Path, version: &str) -> i32 {
    invoke(args, repository, version, false).await
}

pub async fn apply_command(args: Vec<String>, repository: &Path, version: &str) -> i32 {
    invoke(args, repository, version, true).await
}

async fn invoke(args: Vec<String>, repository: &Path, version: &str, applying: bool) -> i32 {
    let command = if applying { "apply" } else { "plan" };
    match run(args, repository, version, command, applying).await {
        Ok(code) => code,
        Err((code, error)) => {
            eprintln!(
                "conduit {command}: {}",
                user_config::escape_line_breaks(&error)
            );
            code
        }
    }
}

async fn run(
    args: Vec<String>,
    repository: &Path,
    version: &str,
    command: &str,
    applying: bool,
) -> std::result::Result<i32, (i32, String)> {
    let local = |error: String| (2, error);
    let options = Options::parse(args, command).map_err(local)?;
    let spec = conduit_loader::Specification::read(&repository.join("spec"))
        .map_err(|error| local(error.to_string()))?;
    let mut project = boundary::project(&options.project, &spec).map_err(local)?;
    // The serving wire accepts values, not a submitted directory (SUB-010).
    project.directory = None;
    let mut ledger = Ledger::read(&ledger_at(&options.project)).map_err(local)?;
    if !ledger.replacements.is_empty() {
        ledger.current_files = Some(apply::project_hashes(&options.project).map_err(local)?);
    }
    let context = boundary::context(repository).map_err(local)?;
    let prepared = prepare_desired(project, &context).map_err(local)?;
    if prepared
        .desired
        .stops
        .iter()
        .any(|stop| stop.ends_with(" SEC-001"))
    {
        print!(
            "{}",
            compare(&prepared.desired, &ledger, &Engine::default()).text()
        );
        return Ok(2);
    }
    let client = connect(options.store, version, command).await?;
    let engine = read_engine(&client, &context.core_package)
        .await
        .map_err(|error| (1, error))?;
    recover_legacy_slots(&mut ledger, &engine);
    let desired = load_desired(prepared, &engine, &ledger).map_err(local)?;
    let plan = compare::plan(&desired, &ledger, &engine, &options.approvals);
    let code = if applying {
        apply::execute(&options.project, &plan, ledger, async |tool, args, key| {
            apply::send(&client, tool, args, key).await
        })
        .await
    } else {
        print!("{}", plan.text());
        i32::from(!plan.stops.is_empty()) * 2
    };
    let _ = client.cancel().await;
    Ok(code)
}

fn differs(planner: &str, serving: &str) -> (i32, String) {
    (
        2,
        format!(
            "CLI version {planner:?} differs from serving engine {serving:?}; nothing was sent; use the matching conduit build"
        ),
    )
}

async fn connect(
    store: Option<PathBuf>,
    version: &str,
    command: &str,
) -> std::result::Result<RunningService<RoleClient, ()>, (i32, String)> {
    let contextual = store.is_none();
    let invalid_context = |cause: String| {
        (
            2,
            format!("{cause}; start conduit serve --store DIR or give --store DIR"),
        )
    };
    let (store, address, label) = if let Some(store) = store {
        let fixed = Config::for_store(&store).map_err(|cause| (2, cause))?;
        (store, fixed.engine_url, None)
    } else {
        let (file, current) = user_config::current().await.map_err(invalid_context)?;
        if current.version != version {
            return Err(differs(version, &current.version));
        }
        (current.store, current.url, Some(file))
    };
    let address = crate::local_engine_url(&address).map_err(invalid_context)?;
    let client_error = |reason: String| {
        let cause = format!("store {store:?} at {address}: {reason}");
        if contextual {
            invalid_context(cause)
        } else {
            (1, cause)
        }
    };
    if !store.join("state.json").is_file() {
        return Err((
            2,
            format!("store {store:?} is not initialized; run conduit init --store DIR"),
        ));
    }
    let transport = StreamableHttpClientTransport::with_client(
        reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .build()
            .map_err(|e| client_error(e.to_string()))?,
        StreamableHttpClientTransportConfig::with_uri(address.as_str())
            .auth_header(server::root_client_token(&store).map_err(client_error)?)
            .reinit_on_expired_session(false),
    );
    let service = tokio::time::timeout(Duration::from_secs(10), ().serve(transport))
        .await
        .map_err(|_| client_error("MCP initialize timed out".into()))?
        .map_err(|_| client_error("MCP initialize failed".into()))?;
    let peer = service.peer_info();
    let info = peer.as_ref().and_then(|p| p.server_info.as_ref());
    if !info.is_some_and(|info| info.name == "conduit" && info.version == server::wire_version()) {
        let actual = info
            .map(|info| format!("{} {}", info.name, info.version))
            .unwrap_or_default();
        let error = differs(server::wire_version(), &actual);
        let _ = service.cancel().await;
        return Err(error);
    }
    if let Some(file) = label {
        eprintln!("conduit {command}: using {file:?} for store {store:?} at {address}");
    }
    Ok(service)
}

async fn call(
    client: &RunningService<RoleClient, ()>,
    tool: &str,
    object: &str,
    args: Value,
) -> Result<Value> {
    let failed = |cause: &str| format!("{tool} for {object}: {cause}; check that root can read it");
    let request = CallToolRequestParams::new(tool.to_owned())
        .with_arguments(args.as_object().unwrap().clone());
    let result = tokio::time::timeout(Duration::from_secs(15), client.call_tool(request))
        .await
        .map_err(|_| failed("request timed out"))?
        .map_err(|_| failed("request failed"))?;
    let result = serde_json::to_value(result).map_err(|_| failed("invalid response"))?;
    if !author::succeeded(&result) {
        return Err(failed("read did not succeed"));
    }
    result
        .get("structuredContent")
        .and_then(|value| value.get("outputs").or(Some(value)))
        .filter(|value| value.is_object())
        .cloned()
        .ok_or_else(|| failed("no structured output"))
}

async fn inspect(
    client: &RunningService<RoleClient, ()>,
    tool: &str,
    name: &Name,
    id: &str,
) -> Result<Value> {
    call(
        client,
        tool,
        &name.to_string(),
        json!({"object":name.reference(),"expectedId":id}),
    )
    .await
}

async fn read_engine(client: &RunningService<RoleClient, ()>, core: &str) -> Result<Engine> {
    let listed = call(client, "management.list.v1", "management", json!({})).await?;
    let rows = listed["objects"]
        .as_array()
        .ok_or("management.list.v1 for management: missing objects")?;
    let mut engine = Engine {
        installed: array(&listed["packages"])
            .iter()
            .filter_map(Pin::from_value)
            .collect(),
        tombstones: array(&listed["tombstones"]).to_vec(),
        ..Engine::default()
    };
    let names: BTreeMap<_, _> = rows
        .iter()
        .filter_map(|row| Some((text(&row["id"]), Name::object(row)?)))
        .collect();
    let root = Name::new("Interface", "root");
    let root_id = rows
        .iter()
        .find(|row| Name::object(row).as_ref() == Some(&root))
        .map(|row| text(&row["id"]))
        .ok_or("management.list.v1 for management: missing root")?;
    let root_view = inspect(client, "management.inspect.v1", &root, root_id).await?;
    let mut inspect_routes = BTreeMap::new();
    let op = json!(format!("{core}#inspect.v1"));
    let listing_op = json!(format!("{core}#list.v1"));
    let open_op = json!(format!("{core}#open.v1"));
    let access = Name::new("Resource", "access");
    for conduit in array(&root_view["inventory"]["consumes"]) {
        let owner_id = text(&conduit["target"]["id"]);
        if conduit["state"] != "ready" {
            continue;
        }
        let Some(name) = names.get(owner_id) else {
            continue;
        };
        let local = text(&conduit["localName"]);
        let operations = array(&conduit["operations"]);
        if name == &access && local == "access" && operations.contains(&open_op) {
            engine.access_conduit = conduit["conduit"]["id"].as_str().map(str::to_owned);
        }
        if !operations.contains(&op) {
            continue;
        }
        if matches!(name.kind.as_str(), "Pool" | "Fleet") {
            engine
                .routes
                .entry(name.clone())
                .or_insert_with(|| local.to_owned());
            if operations.contains(&listing_op) {
                let members = call(
                    client,
                    &format!("{local}.list.v1"),
                    &name.to_string(),
                    json!({}),
                )
                .await?;
                for row in array(&members["objects"]) {
                    inspect_routes
                        .entry(text(&row["id"]).to_owned())
                        .or_insert_with(|| format!("{local}.inspect.v1"));
                }
            }
        } else if name.name == "management" {
            engine.routes.insert(name.clone(), local.into());
        }
    }
    for row in rows {
        let name =
            Name::object(row).ok_or("management.list.v1 for management: object has no name")?;
        let id = text(&row["id"]);
        let tool = inspect_routes
            .get(id)
            .map(String::as_str)
            .unwrap_or("management.inspect.v1");
        let observed = if name == root {
            root_view.clone()
        } else {
            inspect(client, tool, &name, id).await?
        };
        if observed["id"] != id {
            return Err(format!("{tool} for {name}: engine returned another ID"));
        }
        let declared = inspection_declaration(&observed, &names);
        let made_by_engine = observed["provenance"]["cause"] == "default"
            || observed["provenance"]["cause"] == "first-start"
            || declared["engine_made"] == true
            || declared["type"] == format!("{core}#node.v1")
            || matches!(name.kind.as_str(), "Node")
            || matches!(
                (name.kind.as_str(), name.name.as_str()),
                ("Pool", "system") | ("Interface", "root") | ("Resource", "access" | "management")
            );
        let unknown_endings = array(&observed["records"])
            .iter()
            .filter_map(|event| {
                let data = &event["data"];
                if event["type"] != "invocation-ended"
                    || data["outcome"] != "unknown"
                    || data["target"] != id
                {
                    return None;
                }
                Some((
                    data["invocation"].as_str()?.to_owned(),
                    data["operation"].as_str()?.to_owned(),
                ))
            })
            .collect();
        engine.objects.insert(
            name,
            ObservedObject {
                id: id.into(),
                revision: text(&observed["revision"]).into(),
                state: if row["template"] == true {
                    "template".into()
                } else {
                    text(&row["state"]).into()
                },
                definition: compact_definition(&declared),
                packages: array(&observed["packages"]).to_vec(),
                status: observed["status"].clone(),
                unknown_endings,
                inventory: observed["inventory"].clone(),
                implementations: observed["implementations"].clone(),
                made_by_engine,
            },
        );
    }
    Ok(engine)
}

pub(super) fn name_refs(value: &Value, names: &BTreeMap<&str, Name>) -> Value {
    match value {
        Value::Object(fields)
            if fields.len() == 2 && fields.contains_key("kind") && fields.contains_key("id") =>
        {
            names
                .get(text(&value["id"]))
                .map(Name::reference)
                .unwrap_or_else(|| value.clone())
        }
        Value::Object(fields) => Value::Object(
            fields
                .iter()
                .map(|(k, v)| (k.clone(), name_refs(v, names)))
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(|v| name_refs(v, names)).collect()),
        _ => value.clone(),
    }
}

pub(super) fn inspection_declaration(observed: &Value, names: &BTreeMap<&str, Name>) -> Value {
    let mut declared = name_refs(&observed["declared"], names);
    if let Some(provenance) = observed.get("provenance") {
        declared["provenance"] = provenance.clone();
    }
    declared
}

pub(super) fn compact_definition(declared: &Value) -> Value {
    let spec = declared.get("spec");
    let mut definition = spec.cloned().unwrap_or_else(|| json!({}));
    for field in ["description", "annotations"] {
        if let Some(value) = declared["metadata"]
            .get(field)
            .or_else(|| declared.get(field))
        {
            definition[field] = value.clone();
        }
    }
    if spec.is_some() {
        return definition;
    }
    for key in [
        "from",
        "owner",
        "consumer",
        "target",
        "placement",
        "runtime",
        "provenance",
        "slots",
    ] {
        if let Some(value) = declared.get(key) {
            definition[key] = if matches!(key, "owner" | "from") && value["name"].is_string() {
                value["name"].clone()
            } else {
                value.clone()
            };
        }
    }
    if let Some(local) = declared.get("local_name").or_else(|| declared.get("as")) {
        definition["as"] = local.clone();
    }
    for key in [
        "type",
        "config",
        "capability",
        "operations",
        "requirements",
        "entries",
        "conduits",
        "controls",
    ] {
        if let Some(value) = declared
            .get(key)
            .filter(|v| !v.is_null() && v.as_object().is_none_or(|m| !m.is_empty()))
        {
            definition["resolved"][key] = value.clone();
        }
    }
    definition
}

impl Ledger {
    pub fn read(file: &Path) -> Result<Self> {
        let invalid = |place: &str, cause: &str| {
            format!("{file:?} {place}: {cause}; repair kept state before planning")
        };
        match fs::metadata(file) {
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => return Err(invalid("/", &e.to_string())),
        }
        let value = boundary::value(file).map_err(|e| invalid("/", &e))?;
        let fields = value
            .as_object()
            .ok_or_else(|| invalid("/", "expected a mapping"))?;
        for key in fields.keys() {
            if ![
                "format",
                "files",
                "objects",
                "tombstones",
                "plans",
                "journals",
                "journalUpdate",
                "cascades",
                "adopted",
            ]
            .contains(&key.as_str())
            {
                return Err(invalid(&format!("/{key}"), "unknown member"));
            }
        }
        if value["format"] != 1 {
            return Err(invalid("/format", "expected format 1"));
        }
        for key in ["files", "journals"] {
            let map = value[key]
                .as_object()
                .ok_or_else(|| invalid(&format!("/{key}"), "expected a mapping"))?;
            for (name, child) in map {
                let valid = if key == "journals" {
                    qualified_digest(name)
                } else {
                    Path::new(name).is_relative()
                        && !Path::new(name)
                            .components()
                            .any(|c| c == std::path::Component::ParentDir)
                        && child.as_str().is_some_and(hex_digest)
                };
                if !valid {
                    return Err(invalid(
                        &format!("/{key}"),
                        "invalid file entry or plan digest",
                    ));
                }
            }
        }
        for key in ["objects", "plans", "tombstones"] {
            value[key]
                .as_array()
                .ok_or_else(|| invalid(&format!("/{key}"), "expected a sequence"))?;
        }
        for (index, plan) in array(&value["plans"]).iter().enumerate() {
            if !plan.as_str().is_some_and(qualified_digest) {
                return Err(invalid(&format!("/plans/{index}"), "invalid plan digest"));
            }
        }
        let journal_update = value
            .get("journalUpdate")
            .map(|update| {
                update
                    .as_u64()
                    .ok_or_else(|| invalid("/journalUpdate", "expected a counter"))
            })
            .transpose()?
            .unwrap_or(0);
        let mut ledger = Self {
            files: value["files"].clone(),
            tombstones: array(&value["tombstones"]).to_vec(),
            plans: array(&value["plans"]).to_vec(),
            journals: value["journals"].as_object().unwrap().clone(),
            journal_update,
            registration_flags: BTreeMap::new(),
            ..Self::default()
        };
        let current_digest = bytes_digest(value.to_string().as_bytes());
        let mut recovered = None;
        for digest in ledger.plans.iter().filter_map(Value::as_str) {
            let Some(location) = ledger.journals.get(digest) else {
                continue;
            };
            let hex = digest.trim_start_matches("sha256:");
            if location != &format!(".conduit/results/{hex}.yaml") {
                continue;
            }
            let results = file.parent().unwrap().join("results");
            let journal = results.join(format!("{hex}.yaml"));
            if !fs::symlink_metadata(&results).is_ok_and(|meta| meta.is_dir())
                || !fs::symlink_metadata(&journal).is_ok_and(|meta| meta.is_file())
            {
                return Err(invalid("/journals", "expected a regular journal file"));
            }
            let kept = boundary::value(&journal).map_err(|error| invalid("/journals", &error))?;
            let base = file.parent().unwrap();
            let plans = base.join("plans");
            let plan_file = plans.join(format!("{hex}.txt"));
            let plan_text = fs::symlink_metadata(&plans)
                .ok()
                .filter(|metadata| metadata.is_dir())
                .and_then(|_| fs::symlink_metadata(&plan_file).ok())
                .filter(|metadata| metadata.is_file())
                .and_then(|_| fs::read_to_string(&plan_file).ok())
                .filter(|body| bytes_digest(body.as_bytes()) == digest);
            let update = &kept["ledgerUpdate"];
            if update["from"] == current_digest {
                let next = &update["state"];
                if recovered.is_some()
                    || journal_update == u64::MAX
                    || array(&kept["entries"]).last().is_none_or(|entry| {
                        entry["result"]["outcome"] != "succeeded"
                            && !(entry["retirementDispatched"] == true
                                && apply::dispatched_unsuccessful_retirement(&entry["result"]))
                    })
                    || next["journalUpdate"].as_u64() != journal_update.checked_add(1)
                    || ["format", "files", "plans", "journals"]
                        .iter()
                        .any(|key| next[*key] != value[*key])
                    || !next["objects"].is_array()
                    || !next["tombstones"].is_array()
                {
                    return Err(invalid("/journals", "invalid or competing ledger update"));
                }
                recovered = Some(next.clone());
            }
            for entry in array(&kept["entries"]) {
                if let Some(flag) = entry["flag"].as_str() {
                    let approvals = parse_approvals(flag)
                        .ok_or_else(|| invalid("/journals", "unknown registration flag"))?;
                    for object in array(&entry["result"]["objects"])
                        .iter()
                        .chain(entry["result"].get("object"))
                    {
                        if let Some(id) = object["id"].as_str() {
                            ledger
                                .registration_flags
                                .insert(id.to_owned(), approvals.clone());
                        }
                    }
                }
            }
            for entry in array(&kept["priorRemovals"])
                .iter()
                .map(|saved| &saved["entry"])
                .chain(array(&kept["entries"]))
            {
                if let (Some(name), Some(id), Some(invocation)) = (
                    Name::object(&entry["arguments"]["instance"]),
                    entry["arguments"]["expectedId"].as_str(),
                    entry["result"]["invocation"].as_str(),
                ) && entry["tool"]
                    .as_str()
                    .is_some_and(|tool| tool.ends_with(".retire.v1"))
                    && entry["flag"]
                        .as_str()
                        .and_then(parse_approvals)
                        .is_some_and(|flags| flags.contains(&Approval::new("--prune", &name.name)))
                    && apply::dispatched_unsuccessful_retirement(&entry["result"])
                {
                    ledger.prune_attempts.insert(id.into(), invocation.into());
                }
            }
            if let Some(recorded) = plan_text.as_deref() {
                let files = kept.get("files").or_else(|| {
                    (ledger.plans.last().and_then(Value::as_str) == Some(digest))
                        .then_some(&ledger.files)
                });
                if let Some(files) = files {
                    keep_replacements(
                        &mut ledger.replacements,
                        recorded,
                        array(&kept["entries"]),
                        array(&kept["priorRemovals"]),
                        files,
                    );
                }
            }
        }
        let value = if let Some(recovered) = recovered {
            ledger.journal_update = recovered["journalUpdate"].as_u64().unwrap();
            ledger.recovered_update = true;
            ledger.tombstones = array(&recovered["tombstones"]).to_vec();
            recovered
        } else {
            value
        };
        if let Some(adopted) = value.get("adopted") {
            let bad = || invalid("/adopted", "expected a sequence of object IDs");
            for id in adopted.as_array().ok_or_else(bad)? {
                let id = id
                    .as_str()
                    .filter(|id| uuid::Uuid::parse_str(id).is_ok())
                    .ok_or_else(bad)?;
                ledger.adopted.insert(id.into());
            }
        }
        if let Some(cascades) = value.get("cascades") {
            let entries = cascades
                .as_array()
                .ok_or_else(|| invalid("/cascades", "expected a sequence"))?;
            for entry in entries {
                let bad = || {
                    invalid(
                        "/cascades",
                        "invalid temporary Conduit identity or permission",
                    )
                };
                let conduit = Name::object(&entry["conduit"])
                    .filter(|name| name.kind == "Conduit")
                    .ok_or_else(bad)?;
                let id = entry["id"]
                    .as_str()
                    .filter(|id| uuid::Uuid::parse_str(id).is_ok())
                    .ok_or_else(bad)?;
                let container = Name::object(&entry["container"]).ok_or_else(bad)?;
                let flag = match container.kind.as_str() {
                    "Pool" => "--with-resources",
                    "Fleet" => "--with-agents",
                    _ => return Err(bad()),
                };
                let approval = Approval::new(flag, &container.name);
                let retirement = entry
                    .get("retirement")
                    .map(|retirement| {
                        let identities = |field: &str| -> Result<BTreeMap<Name, String>> {
                            let mut identities = BTreeMap::new();
                            for row in retirement[field].as_array().ok_or_else(bad)? {
                                let name = Name::object(row).ok_or_else(bad)?;
                                let id = row["id"]
                                    .as_str()
                                    .filter(|id| uuid::Uuid::parse_str(id).is_ok())
                                    .ok_or_else(bad)?;
                                if identities.insert(name, id.into()).is_some() {
                                    return Err(bad());
                                }
                            }
                            Ok(identities)
                        };
                        let objects = identities("objects")?;
                        if !objects.contains_key(&container) {
                            return Err(bad());
                        }
                        Ok(CascadeRetirement {
                            objects,
                            templates: identities("templates")?,
                            confirmed: retirement["confirmed"].as_bool().ok_or_else(bad)?,
                        })
                    })
                    .transpose()?;
                let temporary_done = entry
                    .get("temporaryDone")
                    .map_or(Ok(false), |done| done.as_bool().ok_or_else(bad))?;
                if entry["flag"] != approval.to_string()
                    || ledger
                        .cascades
                        .insert(
                            conduit,
                            KeptCascade {
                                id: id.into(),
                                container,
                                approval,
                                temporary_done,
                                retirement,
                            },
                        )
                        .is_some()
                {
                    return Err(bad());
                }
            }
        }
        let mut ids = BTreeSet::new();
        for (index, row) in array(&value["objects"]).iter().enumerate() {
            let place = format!("/objects/{index}");
            let [kind, name, id, revision, state, definition, pins] = array(row) else {
                return Err(invalid(&place, "expected seven object fields"));
            };
            let id = text(id);
            if uuid::Uuid::parse_str(id).is_err() || !ids.insert(id) {
                return Err(invalid(
                    &format!("{place}/2"),
                    "invalid or duplicate object ID",
                ));
            }
            if text(kind).is_empty()
                || text(name).is_empty()
                || text(revision).is_empty()
                || !matches!(text(state), "template" | "registered" | "ready" | "retired")
                || !definition.is_object()
                || !pins.is_array()
                || array(pins).iter().any(|p| Pin::from_value(p).is_none())
            {
                return Err(invalid(
                    &place,
                    "invalid object definition, identity or pins",
                ));
            }
            if ledger
                .objects
                .insert(
                    Name::new(text(kind), text(name)),
                    ObservedObject {
                        id: id.into(),
                        revision: text(revision).into(),
                        state: text(state).into(),
                        definition: definition.clone(),
                        packages: array(pins).to_vec(),
                        status: Value::Null,
                        inventory: Value::Null,
                        implementations: Value::Null,
                        made_by_engine: definition["engine-made"] == true,
                        unknown_endings: BTreeMap::new(),
                    },
                )
                .is_some()
            {
                return Err(invalid(&place, "duplicate object name"));
            }
        }
        Ok(ledger)
    }
}

fn keep_replacements(
    replacements: &mut BTreeMap<String, ReplacementJournal>,
    recorded: &str,
    entries: &[Value],
    prior_removals: &[Value],
    files: &Value,
) {
    let registered: BTreeSet<_> = entries
        .iter()
        .filter(|entry| entry["result"]["outcome"] == "succeeded")
        .flat_map(|entry| {
            array(&entry["result"]["objects"])
                .iter()
                .chain(entry["result"].get("object"))
                .chain(entry.get("replaces"))
        })
        .filter_map(Name::object)
        .collect();
    replacements.retain(|_, proof| {
        proof.remade.retain(|name| !registered.contains(name));
        !proof.remade.is_empty()
    });
    let approval_line = recorded
        .lines()
        .find_map(|line| line.strip_prefix("flags [")?.strip_suffix(']'));
    let Some(approvals) = approval_line
        .and_then(parse_approvals)
        .filter(|flags| flags.iter().any(|flag| flag.flag == "--replace"))
    else {
        return;
    };
    let listed: BTreeSet<_> = recorded
        .lines()
        .filter_map(|line| line.strip_prefix("detail objects=[")?.strip_suffix(']'))
        .flat_map(|names| names.split(','))
        .filter_map(|name| {
            name.split_once('/')
                .map(|(kind, name)| Name::new(kind, name))
        })
        .collect();
    for (entry, files) in prior_removals
        .iter()
        .map(|saved| (&saved["entry"], &saved["files"]))
        .chain(entries.iter().map(|entry| (entry, files)))
    {
        let tool = text(&entry["tool"]);
        let invocation = text(&entry["result"]["invocation"]);
        let outcome = &entry["result"]["outcome"];
        if invocation.is_empty()
            || !(tool.ends_with(".retire.v1")
                && (outcome == "succeeded"
                    || apply::dispatched_unsuccessful_retirement(&entry["result"]))
                || tool.ends_with(".close.v1") && outcome == "succeeded")
        {
            continue;
        }
        let names = entry
            .get("remakes")
            .map(|names| array(names).iter().filter_map(Name::object).collect())
            .unwrap_or_else(|| listed.clone());
        let remade: BTreeSet<_> = names.difference(&registered).cloned().collect();
        if remade.is_empty() {
            continue;
        }
        let step_approvals = entry["flag"]
            .as_str()
            .and_then(parse_approvals)
            .unwrap_or_default();
        let args = &entry["arguments"];
        let removed = Name::object(&args["conduit"])
            .or_else(|| Name::object(&args["instance"]))
            .zip(args["expectedId"].as_str().map(str::to_owned));
        replacements.insert(
            invocation.into(),
            ReplacementJournal {
                approvals: approvals.clone(),
                step_approvals,
                remade,
                slots: array(&entry["replacementSlots"])
                    .iter()
                    .filter_map(ReplacementSlot::from_kept)
                    .collect(),
                removed,
                files: files.clone(),
            },
        );
    }
}

fn recover_legacy_slots(ledger: &mut Ledger, engine: &Engine) {
    for proof in ledger.replacements.values_mut() {
        let Some((name, id)) = &proof.removed else {
            continue;
        };
        if !proof.remade.contains(name) || proof.slots.contains_key(name) {
            continue;
        }
        if let Some((parent, field, member)) = compare::nested_slot_for(engine, name, id) {
            proof.slots.insert(
                name.clone(),
                ReplacementSlot {
                    parent: parent.clone(),
                    parent_id: engine.objects[parent].id.clone(),
                    field,
                    member: member.into(),
                    child_id: id.clone(),
                },
            );
        }
    }
}

fn hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn qualified_digest(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(hex_digest)
}

fn manifest_pins(manifest: &Value, core: &str) -> Vec<Pin> {
    manifest["packages"]
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(address, _)| address.as_str() != core)
        .filter_map(|(address, value)| {
            let version = value["version"].as_str()?;
            let digest = value["digest"].as_str()?.strip_prefix("sha256:")?;
            if !hex_digest(digest) {
                return None;
            }
            Some(Pin {
                address: address.clone(),
                version: Some(version.into()),
                digest: Some(format!("sha256:{digest}")),
            })
        })
        .collect()
}

pub(super) struct Cache<'a> {
    directory: PathBuf,
    context: &'a Context,
    archives: BTreeMap<Pin, Archive>,
    loading: BTreeSet<Pin>,
    state: State,
}

impl Cache<'_> {
    fn read(&mut self, pin: &Pin, required: bool, stops: &mut BTreeSet<String>) -> Result<()> {
        if let Some(archive) = self.archives.get(pin) {
            Self::stop(pin, archive, required, stops);
            return Ok(());
        }
        if !self.loading.insert(pin.clone()) {
            return Err(format!(
                "package dependency cycle at {}; repair the package manifest",
                pin.label()
            ));
        }
        let digest = pin.digest.as_deref().ok_or("missing archive digest")?;
        let hex = digest
            .strip_prefix("sha256:")
            .filter(|h| hex_digest(h))
            .ok_or("malformed archive digest")?;
        let file = self.directory.join(hex);
        let archive = match fs::read(&file) {
            Err(error) if error.kind() == ErrorKind::NotFound => Archive::Absent,
            Err(error) => {
                return Err(format!(
                    "cache archive for {} cannot be read: {error}; check the package cache",
                    pin.label()
                ));
            }
            Ok(bytes) if bytes_digest(&bytes) != digest => Archive::Mismatch,
            Ok(bytes) => {
                let manifest = boundary::install_manifest(&bytes).map_err(|_| {
                    format!(
                        "cache archive for {} has no readable manifest; repair the package cache",
                        pin.label()
                    )
                })?;
                let dependencies = manifest_pins(&manifest, &self.context.core_package);
                for dependency in &dependencies {
                    self.read(dependency, required, stops)?;
                }
                if dependencies
                    .iter()
                    .any(|p| !matches!(self.archives.get(p), Some(Archive::Read(_))))
                {
                    self.loading.remove(pin);
                    return Ok(());
                }
                let archive = ArchiveInput {
                    name: pin.label(),
                    address: pin.address.clone(),
                    version: pin.version.clone().ok_or("missing archive version")?,
                    digest: digest.into(),
                    directory: Some(format!("file:///conduit-plan-cache/{hex}/")),
                    completion: FetchCompletion::Complete { bytes },
                };
                let result = conduit_core::step(
                    &self.state,
                    Input::InstallArchive {
                        archive,
                        snapshots: vec![],
                        builtin: self.context.builtin.clone().ok_or("no core package")?,
                    },
                    self.context,
                )?;
                if result.reply["accepted"] != true {
                    let rules = array(&result.reply["diagnostics"])
                        .iter()
                        .map(|d| text(&d["ruleId"]))
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(format!(
                        "cache archive for {} failed validation ({rules}); install a valid package",
                        pin.label()
                    ));
                }
                self.state = result.state;
                Archive::Read(Box::new(
                    self.state
                        .installed_packages
                        .iter()
                        .find(|p| p.package.digest == digest)
                        .ok_or("archive validation returned no package")?
                        .clone(),
                ))
            }
        };
        Self::stop(pin, &archive, required, stops);
        self.archives.insert(pin.clone(), archive);
        self.loading.remove(pin);
        Ok(())
    }

    pub(super) fn stop(pin: &Pin, archive: &Archive, required: bool, stops: &mut BTreeSet<String>) {
        if required {
            match archive {
                Archive::Absent => {
                    stops.insert(
                        pin.package_stop("archive=absent requires=conduit-install", "PKG-004"),
                    );
                }
                Archive::Mismatch => {
                    stops.insert(pin.package_stop("archive=mismatch", "PKG-004"));
                }
                Archive::Read(_) => {}
            }
        }
    }
}

pub(super) fn relative_stops(project: &Project, stops: &mut BTreeSet<String>) {
    let Some(manifest) = project.manifest.as_ref() else {
        return;
    };
    for (address, pin) in manifest["packages"].as_object().into_iter().flatten() {
        if pin["fetch"]
            .as_str()
            .is_some_and(|url| !url.is_empty() && url::Url::parse(url).is_err())
        {
            stops.insert(format!(
                "stop Manifest/- place=/packages/{}/fetch SUB-010",
                escape_pointer(address)
            ));
        }
    }
    for (name, item) in manifest["implementations"]
        .as_object()
        .into_iter()
        .flatten()
    {
        if item["command"][0]
            .as_str()
            .is_some_and(|program| program.contains('/') && !program.starts_with('/'))
        {
            stops.insert(format!(
                "stop Manifest/- place=/implementations/{}/command/0 SUB-010",
                escape_pointer(name)
            ));
        }
    }
}

fn escape_pointer(place: &str) -> String {
    place.replace('~', "~0").replace('/', "~1")
}

struct Prepared<'a> {
    desired: Desired,
    cache: Cache<'a>,
    pins: Vec<Pin>,
}

fn prepare_desired(project: Project, context: &Context) -> Result<Prepared<'_>> {
    let mut desired = Desired {
        project,
        objects: BTreeMap::new(),
        nested: Vec::new(),
        core_address: context.core_package.clone(),
        core_pin: json!({"address":context.core_package,
            "version":context.builtin.as_ref().and_then(|p|p.manifest.as_ref()).and_then(|m|m.get("version"))}),
        stops: BTreeSet::new(),
        available_archives: BTreeMap::new(),
    };
    relative_stops(&desired.project, &mut desired.stops);
    let empty = Value::Null;
    let pins = manifest_pins(
        desired.project.manifest.as_ref().unwrap_or(&empty),
        &context.core_package,
    );
    let mut cache = Cache {
        directory: user_config::directory()?.join("packages"),
        context,
        archives: BTreeMap::new(),
        loading: BTreeSet::new(),
        state: State::default(),
    };
    for pin in &pins {
        cache.read(pin, true, &mut desired.stops)?;
    }
    if !desired.stops.is_empty() {
        return Ok(Prepared {
            desired,
            cache,
            pins,
        });
    }
    let snapshots: Vec<_> = cache
        .state
        .installed_packages
        .iter()
        .map(|p| p.package.clone())
        .collect();
    let builtin = context.builtin.clone().ok_or("missing core package")?;
    let resolution = Catalog::new(builtin.clone(), snapshots.clone(), &context.core_package)?
        .resolve(&desired.project)?;
    if !resolution.reply["accepted"].as_bool().unwrap_or(false) {
        let rules = array(&resolution.reply["diagnostics"])
            .iter()
            .map(|d| text(&d["ruleId"]))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "project resolution failed ({rules}); correct the manifest or documents"
        ));
    }
    let environment: conduit_core::schemas::Environment =
        serde_json::from_value(resolution.reply["environment"].clone())
            .map_err(|e| e.to_string())?;
    secret_stops(
        &desired.project,
        &snapshots,
        &builtin,
        &environment,
        &context.core_package,
        &mut desired.stops,
    );
    Ok(Prepared {
        desired,
        cache,
        pins,
    })
}

fn load_desired(prepared: Prepared<'_>, engine: &Engine, ledger: &Ledger) -> Result<Desired> {
    let Prepared {
        mut desired,
        mut cache,
        pins,
    } = prepared;
    for pin in &pins {
        check_installed(pin, engine, &mut desired.stops);
    }
    if !desired.stops.is_empty() {
        return Ok(desired);
    }
    resolve_objects(&mut desired, &cache.state, engine, ledger, cache.context)?;
    // Only previously held releases relevant to a moved pin are needed for row 19.
    let need_old: BTreeSet<_> = desired
        .objects
        .iter()
        .filter_map(|(name, want)| {
            let old = engine.objects.get(name)?;
            let kept = ledger.objects.get(name)?;
            (!old.is_template() && old.id == kept.id).then_some((want, old))
        })
        .flat_map(|(want, old)| {
            old.packages
                .iter()
                .filter(|package| package.get("documents").is_none())
                .filter_map(Pin::from_value)
                .filter(|p| {
                    p.digest.is_some()
                        && p.version.is_some()
                        && want
                            .resolved
                            .pins()
                            .any(|new| new.address == p.address && new.digest != p.digest)
                })
                .collect::<Vec<_>>()
        })
        .collect();
    for pin in &need_old {
        cache.read(pin, true, &mut desired.stops)?;
    }
    let core = desired.core_address.as_str();
    let agent_type_pins = desired
        .objects
        .keys()
        .filter(|name| name.kind == "Agent")
        .filter_map(|name| engine.objects.get(name))
        .flat_map(|agent| {
            let address = text(&agent.definition["resolved"]["type"])
                .split_once('#')
                .map(|(address, _)| address);
            agent.pins().filter(move |pin| {
                Some(pin.address.as_str()) == address
                    && pin.version.is_some()
                    && pin.digest.is_some()
                    && pin.address != core
            })
        })
        .collect::<BTreeSet<_>>();
    for pin in &agent_type_pins {
        cache.read(pin, false, &mut desired.stops)?;
    }
    desired.available_archives = cache.archives;
    Ok(desired)
}

pub(super) fn check_installed(pin: &Pin, engine: &Engine, stops: &mut BTreeSet<String>) {
    match engine
        .installed
        .iter()
        .find(|installed| same_version(pin, installed))
    {
        None => {
            stops.insert(pin.package_stop("installed=no requires=conduit-install", "PKG-006"));
        }
        Some(installed) if installed.digest != pin.digest => {
            stops.insert(pin.package_stop(
                &format!("installed={}", installed.digest.as_deref().unwrap_or("-")),
                "PKG-025",
            ));
        }
        _ => {}
    }
}

fn scope_of<'a>(
    env: &'a conduit_core::schemas::Environment,
    scope: &'a str,
    reference: &'a str,
    core: &str,
) -> Option<(&'a str, &'a str)> {
    let (address, name) = reference.split_once('#').unwrap_or(("", reference));
    let own = env.scopes.get(scope)?;
    let target = if address == core {
        "core"
    } else if address.is_empty() || address == own.address {
        scope
    } else {
        own.manifest.get(address)?.as_str()
    };
    Some((target, name))
}

fn secret_stops(
    project: &Project,
    snapshots: &[Snapshot],
    builtin: &Project,
    env: &conduit_core::schemas::Environment,
    core: &str,
    stops: &mut BTreeSet<String>,
) {
    let find = |scope: &str, name: &str| {
        let documents = match scope {
            "core" => &builtin.documents,
            "project" => &project.documents,
            _ => {
                &snapshots[scope.strip_prefix("snapshot-")?.parse::<usize>().ok()?]
                    .project
                    .documents
            }
        };
        documents
            .iter()
            .find(|doc| doc["metadata"]["name"] == name && doc.get("spec").is_some())
    };
    for doc in &project.documents {
        let Some(name) = Name::document(doc) else {
            continue;
        };
        let (template_scope, template) = if doc.get("spec").is_some() {
            ("project", doc)
        } else {
            let Some((scope, from)) = scope_of(env, "project", text(&doc["from"]), core) else {
                continue;
            };
            let Some(template) = find(scope, from) else {
                continue;
            };
            (scope, template)
        };
        let check = |scope: &str,
                     schema: &Value,
                     value: &Value,
                     prefix: &str,
                     stops: &mut BTreeSet<String>| {
            for path in env.secret_paths(scope, schema, schema, value, core) {
                let value = &value.pointer(&path).unwrap_or(&Value::Null);
                if !value.is_null()
                    && !(value.is_object()
                        && ((value["kind"] == "Resource" && value["name"].is_string())
                            || value["entry"].is_string()))
                {
                    stops.insert(format!("stop {name} place={prefix}{path} SEC-001"));
                }
            }
        };
        let spec = &template["spec"];
        if doc.get("spec").is_some()
            && let Some((scope, ty)) = scope_of(env, template_scope, text(&spec["type"]), core)
            && let Some(contract) = env.scopes[scope].contracts.get(ty)
        {
            check(
                scope,
                &contract["spec"]["schema"],
                &spec["config"],
                "/spec/config",
                stops,
            );
        }
        for (key, input) in spec["inputs"].as_object().into_iter().flatten() {
            if let Some(value) = doc["arguments"].get(key) {
                check(
                    template_scope,
                    &input["schema"],
                    value,
                    &format!("/arguments/{}", escape_pointer(key)),
                    stops,
                );
            }
        }
        for (key, requirement) in spec["requirements"].as_object().into_iter().flatten() {
            let Some((scope, contract)) =
                scope_of(env, template_scope, text(&requirement["requirement"]), core)
            else {
                continue;
            };
            for (arg, value) in requirement["arguments"].as_object().into_iter().flatten() {
                let schema =
                    &env.scopes[scope].contracts[contract]["spec"]["inputs"][arg]["schema"];
                check(
                    scope,
                    schema,
                    value,
                    &format!(
                        "/spec/requirements/{}/arguments/{}",
                        escape_pointer(key),
                        escape_pointer(arg)
                    ),
                    stops,
                );
            }
        }
    }
}

fn evidence(supplied: usize, count: usize) -> Evidence {
    Evidence {
        time: "2000-01-01T00:00:00Z".into(),
        ids: (supplied..supplied + count)
            .map(|n| uuid::Uuid::from_u128(0x40008000000000000000 + n as u128).to_string())
            .collect(),
        revisions: (supplied..supplied + count)
            .map(|n| format!("plan-{n}"))
            .collect(),
        suffixes: (supplied..supplied + count)
            .map(|n| format!("{n:05x}"))
            .collect(),
    }
}

fn resolve_objects(
    desired: &mut Desired,
    cache: &State,
    engine: &Engine,
    ledger: &Ledger,
    context: &Context,
) -> Result<()> {
    let mut start = evidence(1, 64);
    start.revisions.truncate(8);
    start.suffixes.truncate(3);
    let mut state = conduit_core::step(
        cache,
        Input::FirstStart {
            evidence: start,
            machine: Machine {
                hostname: "plan".into(),
                labels: BTreeMap::from([
                    ("kubernetes.io/hostname".into(), "plan".into()),
                    ("kubernetes.io/os".into(), "linux".into()),
                    ("kubernetes.io/arch".into(), "amd64".into()),
                ]),
                state: "reachable".into(),
            },
            trust_domain: "plan.example".into(),
        },
        context,
    )?
    .state;
    let core = &context.core_package;
    let root = state
        .objects
        .iter()
        .find(|o| o["from"] == format!("{core}#root"))
        .ok_or("missing built-in root")?["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let management = state
        .objects
        .iter()
        .find(|o| o["from"] == format!("{core}#management"))
        .ok_or("missing built-in management")?["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let route = state
        .objects
        .iter()
        .find(|o| o["kind"] == "Conduit" && o["target"]["id"] == management)
        .or_else(|| {
            state
                .objects
                .iter()
                .find(|o| o["kind"] == "Conduit" && o["target"] == management)
        })
        .ok_or("missing built-in management Conduit")?["id"]
        .as_str()
        .unwrap()
        .to_owned();
    // A row 6 collision is a comparison stop, not a trial registration error.
    let mut project = desired.project.clone();
    project
        .documents
        .retain(|document| Name::document(document).is_none_or(|name| !engine.protects(&name)));
    let authored: BTreeMap<_, _> = project
        .documents
        .iter()
        .filter(|d| !contract(d))
        .filter_map(|d| Some((Name::document(d)?, d.clone())))
        .collect();
    // The public, named objects provide the reference endpoints of the local trial.
    // Project objects themselves come solely from the current file.
    for (name, old) in &engine.objects {
        if authored.contains_key(name)
            || old.made_by_engine
            || old.is_template()
            || name.kind == "Conduit"
        {
            continue;
        }
        let mut value = json!({"id":old.id,"kind":name.kind,"name":name.name,"state":old.state,"revision":old.revision});
        if let Some(resolved) = old.definition["resolved"].as_object() {
            value.as_object_mut().unwrap().extend(resolved.clone());
        }
        for key in ["from", "owner", "placement", "runtime"] {
            if let Some(child) = old.definition.get(key) {
                value[key] = child.clone();
            }
        }
        value["schema_environment"] =
            serde_json::to_value(&context.core_environment).map_err(|e| e.to_string())?;
        state.allocated_ids.insert(old.id.clone());
        state.objects.push(value);
        if matches!(name.kind.as_str(), "Pool" | "Fleet") && engine.routes.contains_key(name) {
            state.objects.push(json!({"kind":"Conduit","consumer":root,"target":old.id,"state":"ready","open":true,"operations":[format!("{core}#register.v1")]}));
        }
    }
    let before: BTreeSet<_> = state
        .objects
        .iter()
        .filter_map(|o| o["id"].as_str().map(str::to_owned))
        .collect();
    let count = (desired.project.documents.len().saturating_mul(48)).max(128);
    let trial = conduit_core::step_owned(
        state,
        Input::RegisterProject {
            snapshots: vec![],
            builtin: context.builtin.clone().ok_or("missing core package")?,
            project,
            evidence: evidence(128, count),
            request: Request {
                sender: root.clone(),
                caller: root,
                conduit: route,
                target: management,
                operation: format!("{core}#register.v1"),
            },
        },
        context,
    )?;
    if trial.reply["accepted"] != true {
        let cases = array(&trial.reply["diagnostics"])
            .iter()
            .map(|d| text(&d["ruleId"]))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "project definitions could not be resolved ({cases}); correct the project documents"
        ));
    }
    let mut names: BTreeMap<_, _> = trial
        .state
        .objects
        .iter()
        .filter_map(|row| Some((row["id"].as_str()?, Name::object(row)?)))
        .collect();
    let all: BTreeMap<_, _> = trial
        .state
        .objects
        .iter()
        .filter_map(|row| Some((Name::object(row)?, row)))
        .collect();
    for (name, object) in &all {
        let Some(nested) = object.get("nested") else {
            continue;
        };
        let Some(parent) = names.get(text(&nested["parent"])) else {
            continue;
        };
        desired.nested.push(NestedRegistration {
            parent: parent.clone(),
            local: text(&nested["name"]).into(),
            name: name.clone(),
            resolved: resolved_view(object, &names, Value::Null)?,
        });
    }
    let mut resolved = Vec::new();
    for (name, document) in authored {
        let object = all
            .get(&name)
            .filter(|row| !before.contains(text(&row["id"])))
            .ok_or_else(|| format!("local resolution omitted {name}"))?;
        resolved.push((name, document, *object));
    }
    // Existing nested members are compared through their parent's declared slot.
    // Newly registered parents get their members from the parent registration.
    let mut index = 0;
    let mut included: BTreeSet<_> = resolved.iter().map(|(name, _, _)| name.clone()).collect();
    while let Some((parent, _, object)) = resolved.get(index) {
        index += 1;
        let Some(observed) = engine.objects.get(parent) else {
            continue;
        };
        let mut children = Vec::new();
        for field in ["entries", "conduits"] {
            for (slot, child) in object["slots"][field].as_object().into_iter().flatten() {
                let prior = &observed.definition["slots"][field][slot];
                let Some(name) = Name::object(prior)
                    .or_else(|| {
                        engine
                            .objects
                            .iter()
                            .find(|(_, old)| prior["id"] == old.id)
                            .map(|(name, _)| name.clone())
                    })
                    .or_else(|| {
                        ledger
                            .replacements
                            .values()
                            .flat_map(|proof| {
                                proof
                                    .slots
                                    .iter()
                                    .filter(|(name, _)| proof.remade.contains(*name))
                            })
                            .find(|(_, saved)| {
                                saved.parent == *parent
                                    && saved.parent_id == observed.id
                                    && saved.field == field
                                    && saved.member == *slot
                                    && prior["id"] == saved.child_id
                            })
                            .map(|(name, _)| name.clone())
                    })
                else {
                    continue;
                };
                let id = text(&child["id"]);
                let Some(current) = names.get(id).and_then(|name| all.get(name)).copied() else {
                    continue;
                };
                if included.insert(name.clone()) {
                    names.insert(id, name.clone());
                    let doc = json!({"kind": name.kind, "metadata": {"name": name.name}, "from": current["from"]});
                    children.push((name, doc, current));
                }
            }
        }
        resolved.extend(children);
    }
    for (name, document, object) in resolved {
        let defined = name_refs(object, &names);
        let held = &object["held"];
        let implementations = conduit_core::registration::inspected_implementations(object, core)
            .unwrap_or(Value::Null);
        let env: conduit_core::schemas::Environment =
            serde_json::from_value(held["environment"].clone())
                .map_err(|_| "resolved object has no schema environment")?;
        let scope = text(&held["scope"]);
        let own_tools = if name.kind == "Agent" && object.get("spec").is_none() {
            Some(conduit_core::inventory::own_tools(object, context)?)
        } else {
            None
        };
        let project_index = &held["indexes"]["project"];
        let mut reached_documents = BTreeMap::new();
        let mut reached_packages = BTreeSet::new();
        let mut reach = vec![];
        let supplied = json!({"kind":name.kind,"from":object["from"],"spec":object.get("spec").unwrap_or(object)});
        conduit_core::packages::selections_document(&supplied, true, &mut reach)?;
        for (operation, _) in implementations.as_object().into_iter().flatten() {
            reach.push((operation.clone(), "Operation".into()));
        }
        let own_address = text(&held["contexts"]["project"]["address"]);
        while let Some((reference, _)) = reach.pop() {
            let Some((address, contract_name)) = reference.split_once('#') else {
                continue;
            };
            if address.is_empty() || address == own_address {
                let document = &project_index[contract_name];
                if contract(document) {
                    let key = (text(&document["kind"]).to_owned(), contract_name.to_owned());
                    if let std::collections::btree_map::Entry::Vacant(entry) =
                        reached_documents.entry(key)
                    {
                        entry.insert(document_digest(document)?);
                        conduit_core::packages::selections_document(document, true, &mut reach)?;
                    }
                }
            } else if address != core {
                reached_packages.insert(address.to_owned());
            }
        }
        let ty = object.get("spec").unwrap_or(object);
        let mut controls: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        if let Some((type_scope, name)) = scope_of(&env, scope, text(&ty["type"]), core) {
            let contract = &env.scopes[type_scope].contracts.get(name);
            if let Some(contract) = contract {
                for op in array(&contract["spec"]["controls"])
                    .iter()
                    .filter_map(Value::as_str)
                {
                    if let Some((operation_scope, op_name)) = scope_of(&env, type_scope, op, core) {
                        let resolved_op =
                            format!("{}#{op_name}", env.scopes[operation_scope].address);
                        if ty.get("controls").is_none_or(|map| {
                            map.get(&resolved_op).is_some() || map.get(op).is_some()
                        }) && let Some(contract) =
                            env.scopes[operation_scope].contracts.get(op_name)
                        {
                            for pointer in contract["spec"]["changes"]
                                .as_object()
                                .into_iter()
                                .flat_map(|changes| changes.values())
                                .filter_map(Value::as_str)
                            {
                                controls
                                    .entry(pointer.into())
                                    .or_default()
                                    .insert(resolved_op.clone());
                            }
                        }
                    }
                }
            }
        }
        let model_work = array(&defined["operations"])
            .iter()
            .filter_map(Value::as_str)
            .any(|op| {
                scope_of(&env, scope, op, core)
                    .and_then(|(s, n)| env.scopes[s].contracts.get(n))
                    .is_some_and(|doc| doc["spec"]["startsModelWork"] == true)
            });
        desired.objects.insert(
            name,
            DesiredObject {
                document,
                resolved: resolved_view(object, &names, implementations)?,
                own_tools,
                controls,
                reached_documents,
                reached_packages,
                model_work,
            },
        );
    }
    Ok(())
}

fn resolved_view(
    object: &Value,
    names: &BTreeMap<&str, Name>,
    implementations: Value,
) -> Result<ObservedObject> {
    let mut packages = object["packages"].clone();
    conduit_core::registration::inspect_project_documents(object, &mut packages)?;
    Ok(ObservedObject {
        id: text(&object["id"]).into(),
        revision: text(&object["revision"]).into(),
        state: if object.get("spec").is_some() {
            "template".into()
        } else {
            text(&object["state"]).into()
        },
        definition: compact_definition(&name_refs(object, names)),
        packages: array(&packages).to_vec(),
        status: Value::Null,
        inventory: Value::Null,
        implementations,
        made_by_engine: false,
        unknown_endings: BTreeMap::new(),
    })
}
