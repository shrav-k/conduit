//! Author workflows: every management action is a root MCP request to the serving engine.
use crate::{Result, boundary, diagnostics, server, store};
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

static UNANSWERED: AtomicU64 = AtomicU64::new(0);

pub fn unanswered_count(reply: &Value) -> u64 {
    reply["_meta"]
        .as_object()
        .and_then(|meta| meta.iter().find(|(key, _)| key.ends_with("/unanswered")))
        .and_then(|(_, value)| value.as_u64())
        .unwrap_or(0)
}

pub fn note_unanswered(reply: &Value) {
    UNANSWERED.store(unanswered_count(reply), Ordering::SeqCst);
}

pub fn unanswered_hint() -> Option<String> {
    let count = UNANSWERED.load(Ordering::SeqCst);
    (count > 0).then(|| {
        format!(
            "conduit: {count} pending action(s) unanswered at root as of the last result; run conduit pending --store DIR to list them"
        )
    })
}

fn state(directory: &Path) -> Result<Value> {
    let bytes = store::private_read(&directory.join("state.json"))?;
    let mut decoder = serde_json::Deserializer::from_slice(&bytes);
    decoder.disable_recursion_limit();
    let value = Value::deserialize(&mut decoder).map_err(|e| e.to_string())?;
    Ok(value["state"].clone())
}
fn reference(value: &Value) -> &Value {
    value.get("id").unwrap_or(value)
}

/// A printed result succeeds only if every explicit result marker succeeds.
/// Do not scan content or inspection history: those are data, not this call's outcome.
pub fn succeeded(reply: &Value) -> bool {
    reply["isError"] != true
        && reply.get("accepted").is_none_or(|v| v == true)
        && reply.get("outcome").is_none_or(|v| v == "succeeded")
        && reply["_meta"].as_object().is_none_or(|meta| {
            meta.iter()
                .all(|(key, value)| !key.ends_with("/outcome") || value == "succeeded")
        })
}

pub fn ensure_success(reply: &Value, asked: &str) -> Result<()> {
    if succeeded(reply) {
        Ok(())
    } else {
        Err(format!(
            "{asked} did not succeed; see the printed result for outcome and diagnostics"
        ))
    }
}

fn outcome(reply: &Value) -> Value {
    reply["_meta"]
        .as_object()
        .and_then(|m| m.iter().find(|(k, _)| k.ends_with("/outcome")))
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| {
            json!(if succeeded(reply) {
                "succeeded"
            } else {
                "failed"
            })
        })
}

/// LIF-013: render the authorized inventory returned by inspect.v1.
pub fn inspection_row(_objects: &[Value], object: &Value, reply: Value) -> Value {
    let mut row = json!({"object":{"kind":object["kind"],"name":object["name"]},"result":reply});
    if row["result"]["isError"] == false
        && let Some(inventory) = row["result"]["structuredContent"].get("inventory").cloned()
    {
        row["consumedConduits"] = json!(inventory["consumes"].as_array().into_iter().flatten().map(|entry| json!({"name":entry["conduit"]["name"],"target":entry["target"],"open":entry["state"] == "ready"})).collect::<Vec<_>>());
        row["inventory"] = inventory;
    }
    if row["result"]["isError"] == false
        && let Some(past) = object.get("past_conditions")
    {
        row["pastConditions"] = past.clone();
    }
    row
}

pub fn inspection_objects(objects: &[Value], setup: bool) -> impl Iterator<Item = &Value> {
    objects.iter().filter(move |object| {
        !setup
            || (object.get("spec").is_none()
                && matches!(
                    object["kind"].as_str(),
                    Some("Interface" | "Pool" | "Fleet" | "Resource" | "Agent")
                ))
    })
}

pub fn print_inspection_row(objects: &[Value], object: &Value, reply: Value, setup: bool) {
    let row = if setup {
        let object = json!({"kind":object["kind"],"name":object["name"]});
        if succeeded(&reply) {
            let status = &reply["structuredContent"]["status"];
            let setup = status["setup"].as_object().map(|setup| {
                let not_succeeded: Vec<_> = setup
                    .get("parts")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter(|part| part["outcome"] != "succeeded")
                    .map(|part| json!({"name":part["name"],"outcome":part["outcome"]}))
                    .collect();
                json!({"outcome":setup.get("outcome"),"invocation":setup.get("invocation"),"notSucceeded":not_succeeded})
            });
            json!({"object":object,"state":status["state"],"setup":setup})
        } else {
            json!({"object":object,"result":reply})
        }
    } else {
        inspection_row(objects, object, reply)
    };
    println!("{row}");
}

pub(crate) fn named(argument: &str) -> Result<Value> {
    let (kind, name) = argument.split_once('/').ok_or_else(|| {
        format!(
            "object {argument:?}: expected KIND/NAME; use conduit inspect to find its kind and name"
        )
    })?;
    if !["Agent", "Interface", "Resource", "Pool", "Fleet", "Conduit"].contains(&kind)
        || name.is_empty()
        || name.contains('/')
    {
        return Err(format!(
            "object {argument:?}: expected KIND/NAME with kind Agent, Interface, Resource, Pool, Fleet or Conduit"
        ));
    }
    Ok(json!({"kind":kind,"name":name}))
}

// register.v1 carries values without the directory that SUB-010 requires.
// Check both the manifest and documents before sending that request.
fn requires_directory(value: &Value) -> bool {
    match value {
        Value::Object(fields) => {
            fields
                .get("fetch")
                .and_then(Value::as_str)
                .is_some_and(|fetch| {
                    url::Url::parse(fetch) == Err(url::ParseError::RelativeUrlWithoutBase)
                })
                || fields
                    .get("command")
                    .and_then(|v| v.get(0))
                    .and_then(Value::as_str)
                    .is_some_and(|program| {
                        program.contains('/') && Path::new(program).is_relative()
                    })
                || fields.values().any(requires_directory)
        }
        Value::Array(values) => values.iter().any(requires_directory),
        _ => false,
    }
}
pub(crate) fn route(directory: &Path, object: &Value, operation: &str) -> Result<String> {
    let state = state(directory)?;
    let objects = state["objects"].as_array().ok_or("missing objects")?;
    route_from_snapshot(objects, object, operation)
}

pub fn setup_inspection_tool(objects: &[Value], object: &Value) -> String {
    route_from_snapshot(objects, object, "inspect.v1")
        .unwrap_or_else(|_| "management.inspect.v1".into())
}

fn route_from_snapshot(objects: &[Value], object: &Value, operation: &str) -> Result<String> {
    let target = objects
        .iter()
        .find(|o| o["kind"] == object["kind"] && o["name"] == object["name"]);
    let owner = target
        .filter(|o| (o["kind"] == "Resource" || o["kind"] == "Agent") && o.get("spec").is_none())
        .and_then(|o| {
            objects
                .iter()
                .find(|p| p["id"] == *reference(&o["owner"]) && p["state"] != "retired")
        });
    if let Some(owner) = owner {
        let root = objects
            .iter()
            .find(|o| o["name"] == "root" && o["kind"] == "Interface")
            .ok_or("missing root")?;
        let path = objects
            .iter()
            .find(|p| {
                p["kind"] == "Conduit"
                    && p["open"] == true
                    && reference(&p["consumer"]) == &root["id"]
                    && reference(&p["target"]) == &owner["id"]
                    && p["operations"].as_array().is_some_and(|ops| {
                        ops.iter().any(|op| {
                            op.as_str()
                                .is_some_and(|s| s.ends_with(&format!("#{operation}")))
                        })
                    })
            })
            .ok_or_else(|| format!("{operation} for {}/{}: no root management route to owning container {}; open a root Conduit to that container offering {operation}", object["kind"].as_str().unwrap_or("?"), object["name"].as_str().unwrap_or("?"), owner["name"].as_str().unwrap_or("?")))?;
        let local = path["local_name"]
            .as_str()
            .or_else(|| path["nested"]["name"].as_str())
            .or_else(|| path["name"].as_str())
            .ok_or("missing route name")?;
        return Ok(format!("{local}.{operation}"));
    }
    Ok(format!("management.{operation}"))
}
async fn call(directory: &Path, tool: &str, args: Value) -> Result<Value> {
    let reply = server::client(directory, None, Some(tool), args).await?;
    println!("{reply}");
    ensure_success(&reply, &format!("request to {tool}"))?;
    Ok(reply)
}
fn read_author_project(
    command: &str,
    argument: &str,
    spec: &conduit_loader::Specification,
) -> Result<conduit_loader::Project> {
    conduit_loader::read_project(Path::new(argument), spec).map_err(|errors| {
        let diagnostics: Vec<_> = errors.0.iter().map(conduit_loader::diagnostic_result).collect();
        diagnostics::print_diagnostics(&diagnostics, argument, false);
        println!("{}", json!({"accepted":false,"reason":"invalid","diagnostics":diagnostics}));
        format!("{command}: cannot load project {argument}: {} diagnostics above; correct its documents and manifest before retrying", diagnostics.len())
    })
}

fn author_diagnostics(
    reply: &mut Value,
    source: Option<(&conduit_loader::Project, Option<&str>)>,
    path: &str,
    archive: bool,
) {
    if succeeded(reply) {
        return;
    }
    if let Some(rejections) = diagnostics::diagnostics(reply) {
        if let Some((project, template)) = source {
            for diagnostic in rejections.iter_mut() {
                // The replacement preflight's generic diagnostic concerns the one requested template.
                if diagnostic["ruleId"] == "SUB-009"
                    && diagnostic.get("locations").is_none()
                    && let Some(name) = template
                {
                    let name = name.replace('~', "~0").replace('/', "~1");
                    diagnostic["locations"] = json!([{"logicalLocations":[{"fullyQualifiedName":format!("/objects/{name}")}]}]);
                }
                project.locate_diagnostic(diagnostic);
            }
        }
        diagnostics::print_diagnostics(rejections, path, archive);
    }
}

async fn list_pending(directory: &Path, agent: Option<&str>) -> Result<Vec<Value>> {
    let mut reply = server::client(directory, None, Some("management.list.v1"), json!({})).await?;
    if !succeeded(&reply) {
        println!("{reply}");
        return Err("listing root's pending actions through management.list.v1 did not succeed; see the printed result".into());
    }
    if let Some(name) = agent
        && !reply["structuredContent"]["objects"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|object| {
                object["kind"] == "Agent" && object["name"] == name && object["template"] == false
            })
    {
        return Err(format!("root sees no Agent named {name:?}"));
    }
    let mut actions = match reply
        .get_mut("structuredContent")
        .and_then(|content| content.get_mut("pending"))
        .map(Value::take)
    {
        Some(Value::Array(actions)) => actions,
        _ => Vec::new(),
    };
    if let Some(name) = agent {
        actions.retain(|action| {
            action["subject"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|subject| subject["kind"] == "Agent" && subject["name"] == name)
        });
    }
    Ok(actions)
}

fn answer_subject(action: &Value) -> Result<Value> {
    let id = action["id"].as_str().unwrap_or("?");
    let kind = action["kind"].as_str().unwrap_or("?");
    if kind != "setup" {
        return Err(format!(
            "action {id} is of kind {kind}, and conduit answer routes only setup actions in this build; nothing was sent. Answer it with conduit call through root's Conduit to the target that PND-009 names for its kind, or with its kind's own command"
        ));
    }
    if let Some(subject) = action["subject"].as_array().and_then(|items| items.first())
        && let (Some(kind), Some(name)) = (subject["kind"].as_str(), subject["name"].as_str())
    {
        Ok(json!({"kind":kind,"name":name}))
    } else {
        Err(format!("action {id} names no subject; nothing was sent"))
    }
}

async fn revision(directory: &Path, object: &Value) -> Result<Value> {
    let tool = route(directory, object, "inspect.v1")?;
    let reply = call(directory, &tool, json!({"object":object})).await?;
    reply["structuredContent"]
        .get("revision")
        .cloned()
        .ok_or("inspection omitted revision".into())
}

#[derive(Default)]
pub struct Options<'a> {
    pub agent: Option<&'a str>,
    pub setup: bool,
    pub serving_checked: bool,
}

/// Establish the serving path without opening the live store for writing or recovery.
pub(crate) async fn require_serving(
    directory: &Path,
    endpoint: &url::Url,
    quiet: bool,
) -> Result<()> {
    let host = match endpoint.host().ok_or("missing engine host")? {
        url::Host::Ipv6(ip) => ip.to_string(),
        host => host.to_string(),
    };
    let port = endpoint
        .port_or_known_default()
        .ok_or("missing engine port")?;
    tokio::time::timeout(
        std::time::Duration::from_secs(2),
        tokio::net::TcpStream::connect((host.as_str(), port)),
    )
    .await
    .map_err(|_| "conduit serve must hold this store: connection timed out")?
    .map_err(|_| "conduit serve must hold this store: cannot connect to engine")?;
    server::probe_root(directory, quiet.then_some(endpoint.as_str()))
        .await
        .map_err(|error| format!("conduit serve must hold this store: {error}"))
}

pub async fn run(
    command: &str,
    directory: &Path,
    args: &[String],
    repository: &Path,
    options: Options<'_>,
) -> Result<()> {
    let expected = match command {
        "inspect" | "pending" => 0,
        "replace" | "answer" => 2,
        _ => 1,
    };
    if args.len() != expected {
        return Err(format!(
            "{command}: expected {expected} positional argument(s), received {}; run conduit {command} --help for usage and an example",
            args.len()
        ));
    }
    if !options.serving_checked {
        let endpoint = url::Url::parse(&crate::Config::for_store(directory)?.engine_url)
            .map_err(|_| "invalid engine URL")?;
        require_serving(directory, &endpoint, false).await?;
    }
    match command {
        "pending" => {
            let actions = list_pending(directory, options.agent).await?;
            if actions.is_empty() {
                if let Some(name) = options.agent {
                    eprintln!("conduit pending: no pending action names Agent {name:?}");
                } else {
                    eprintln!("conduit pending: no pending action is raised at root");
                }
            }
            for action in actions {
                println!("{action}");
            }
        }
        "answer" => {
            let id = &args[0];
            let actions = list_pending(directory, None).await?;
            let action = actions.iter().find(|action| action["id"] == id.as_str()).ok_or_else(|| {
                format!(
                    "no action {id} is raised at root now: it is unknown, already answered or expired; nothing was sent. Run conduit pending to list the raised actions, or conduit call to ask the engine itself"
                )
            })?;
            let subject = answer_subject(action)?;
            let tool = route(directory, &subject, "answer.v1")?;
            call(directory, &tool, json!({"action":id,"answer":args[1]})).await?;
        }
        "inspect" => {
            let current = state(directory)?;
            let objects = current["objects"].as_array().ok_or("missing objects")?;
            let mut success = true;
            for object in inspection_objects(objects, options.setup) {
                let reference = json!({"kind":object["kind"],"name":object["name"]});
                let tool = if options.setup {
                    setup_inspection_tool(objects, &reference)
                } else {
                    route(directory, &reference, "inspect.v1")
                        .unwrap_or_else(|_| "management.inspect.v1".into())
                };
                let reply =
                    server::client(directory, None, Some(&tool), json!({"object":reference}))
                        .await?;
                success &= succeeded(&reply);
                print_inspection_row(objects, object, reply, options.setup);
            }
            if !success {
                return Err("inspect: one or more objects could not be inspected; see their results and check root's management Conduits".into());
            }
        }
        "install" => {
            if args.len() != 1 {
                return Err("install needs ARCHIVE#SHA256".into());
            }
            let tool = "management.install.v1";
            let (archive, request) =
                boundary::named_install_arguments(&args[0], crate::user_config::cache_archive)?;
            let mut reply = server::client(directory, None, Some(tool), request).await?;
            author_diagnostics(&mut reply, None, archive, true);
            println!("{reply}");
            ensure_success(&reply, &format!("request to {tool}"))?;
        }
        "replace" => {
            if args.len() != 2 {
                return Err("replace needs DIR KIND/NAME".into());
            }
            let object = named(&args[1])?;
            let spec = conduit_loader::Specification::read(&repository.join("spec"))
                .map_err(|e| e.to_string())?;
            let project = read_author_project(command, &args[0], &spec)?;
            let documents: Vec<_> = project
                .documents
                .iter()
                .map(|document| document.root.to_json())
                .collect();
            let templates: Vec<_> = documents
                .iter()
                .filter(|d| {
                    d["kind"] == object["kind"]
                        && d["metadata"]["name"] == object["name"]
                        && d.get("spec").is_some()
                })
                .collect();
            if templates.len() != 1 {
                return Err("project must contain exactly one matching template".into());
            }
            let rev = revision(directory, &object).await?;
            let mut input = json!({"template":templates[0],"contracts":documents.iter().filter(|d| conduit_core::packages::contract(d)).collect::<Vec<_>>(),"revision":rev});
            if let Some(manifest) = &project.manifest {
                input["manifest"] = manifest.root.to_json();
            }
            let tool = "management.replace.v1";
            let mut reply = server::client(directory, None, Some(tool), input).await?;
            author_diagnostics(
                &mut reply,
                Some((&project, object["name"].as_str())),
                &args[0],
                false,
            );
            println!("{reply}");
            ensure_success(&reply, &format!("request to {tool}"))?;
        }
        "retire" | "unregister" => {
            if args.len() != 1 {
                return Err(format!("{command} needs KIND/NAME"));
            }
            let object = named(&args[0])?;
            if command == "retire" {
                call(
                    directory,
                    &route(directory, &object, "retire.v1")?,
                    json!({"instance":object}),
                )
                .await?;
            } else {
                let rev = revision(directory, &object).await?;
                call(
                    directory,
                    "management.unregister.v1",
                    json!({"object":object,"revision":rev}),
                )
                .await?;
            }
        }
        "submit" => {
            if args.len() != 1 {
                return Err("submit needs DIR".into());
            }
            let spec = conduit_loader::Specification::read(&repository.join("spec"))
                .map_err(|e| e.to_string())?;
            let project = read_author_project(command, &args[0], &spec)?;
            let documents: Vec<_> = project
                .documents
                .iter()
                .map(|document| document.root.to_json())
                .collect();
            let mut input = json!({"documents":documents});
            if let Some(manifest) = &project.manifest {
                input["manifest"] = manifest.root.to_json();
            }
            if requires_directory(&input) {
                return Err("a submission through the serving engine takes absolute values only: relative fetch or program needs a directory".into());
            }
            let mut steps = Vec::new();
            let source = (&project, args[0].as_str());
            let result: Result<()> = async {
            let reply = submit_step(directory, "management.register.v1", input, "registration".into(), &mut steps, source).await?;
            let created = reply["structuredContent"]["objects"]
                .as_array()
                .ok_or("registration omitted objects")?;
            let current = state(directory)?;
            let objects = current["objects"].as_array().ok_or("missing objects")?;
            let root = objects
                .iter()
                .find(|o| o["name"] == "root" && o["kind"] == "Interface")
                .ok_or("missing root")?;
            for o in objects.iter().filter(|o| {
                o["kind"] == "Conduit"
                    && o.get("spec").is_none()
                    // FLEET-005 has already opened defaults within registration.
                    && o["open"] != true
                    && reference(&o["consumer"]) == &root["id"]
                    && created.iter().any(|n| n["id"] == o["id"])
            }) {
                submit_step(directory, "access.open.v1", json!({"conduit":o["name"]}), format!("open Conduit/{}", o["name"].as_str().unwrap_or("?")), &mut steps, source).await?;
            }
            for kind in ["Pool", "Fleet", "Resource", "Agent", "Interface"] {
                for o in objects.iter().filter(|o| {
                    o["kind"] == kind
                        && o.get("spec").is_none()
                        && created.iter().any(|n| n["id"] == o["id"])
                }) {
                    let object = json!({"kind":kind,"name":o["name"]});
                    let step = format!("setup {kind}/{}", o["name"].as_str().unwrap_or("?"));
                    let tool = match route(directory, &object, "set-up.v1") {
                        Ok(tool) => tool,
                        Err(error) => {
                            steps.push(json!({"step":step,"outcome":"failed","error":error}));
                            return Err(format!("{step}: {error}; open a root management Conduit to the owning Pool and resume setup with conduit call"));
                        }
                    };
                    submit_step(
                        directory,
                        &tool,
                        json!({"instance":object}),
                        step,
                        &mut steps,
                        source,
                    )
                    .await?;
                }
            }
            Ok(())
            }.await;
            for step in steps {
                println!("{step}");
            }
            result?;
        }
        _ => return Err("unknown author command".into()),
    }
    Ok(())
}

async fn submit_step(
    directory: &Path,
    tool: &str,
    args: Value,
    step: String,
    steps: &mut Vec<Value>,
    (project, path): (&conduit_loader::Project, &str),
) -> Result<Value> {
    match server::client(directory, None, Some(tool), args).await {
        Ok(mut reply) => {
            author_diagnostics(&mut reply, Some((project, None)), path, false);
            println!("{reply}");
            steps.push(json!({"step":step,"outcome":outcome(&reply)}));
            ensure_success(&reply, &step)?;
            Ok(reply)
        }
        Err(error) => {
            steps.push(json!({"step":step,"outcome":"unknown","error":error}));
            Err(format!(
                "{step}: {error}; inspect before resuming, because a sent request is never retried automatically"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unanswered_count_reads_only_its_meta_key() {
        let key = concat!("example.invalid", "/unanswered");
        assert_eq!(unanswered_count(&json!({"_meta":{key:2}})), 2);
        for reply in [
            json!({}),
            json!({"_meta":{}}),
            json!({"structuredContent":{"unanswered":3}}),
            json!({"structuredContent":{key:3}}),
            json!({"_meta":{key:-1}}),
            json!({"_meta":{key:"2"}}),
        ] {
            assert_eq!(unanswered_count(&reply), 0, "{reply}");
        }
    }

    #[test]
    fn answer_routes_setup_actions_only() {
        let mut action =
            json!({"id":"a","kind":"setup","subject":[{"kind":"Resource","name":"r","id":"x"}]});
        assert_eq!(
            answer_subject(&action).unwrap(),
            json!({"kind":"Resource","name":"r"})
        );
        action["kind"] = json!("notice");
        let error = answer_subject(&action).unwrap_err();
        assert!(error.contains("of kind notice") && error.contains("nothing was sent"));
        action["kind"] = json!("setup");
        action["subject"] = json!([]);
        assert!(
            answer_subject(&action)
                .unwrap_err()
                .contains("names no subject")
        );
    }

    #[test]
    fn inspection_history_is_separate_and_requires_successful_inspection() {
        let object = json!({"kind":"Agent","name":"a","id":"a","past_conditions":[{"condition":{"reason":"RunStopped"},"endedAt":"2026-11-10T00:00:01Z"}]});
        let row = inspection_row(
            &[],
            &object,
            json!({"isError":false,"structuredContent":{"status":{"conditions":[{"reason":"RunRunning"}]}}}),
        );
        assert_eq!(row["pastConditions"], object["past_conditions"]);
        assert_eq!(
            row["result"]["structuredContent"]["status"]["conditions"][0]["reason"],
            "RunRunning"
        );
        let refused = inspection_row(&[], &object, json!({"isError":true}));
        assert!(refused.get("pastConditions").is_none());
    }
    #[test]
    fn printed_result_outcomes_determine_success() {
        for result in [
            json!({"isError":true}),
            json!({"accepted":false}),
            json!({"_meta":{"example/outcome":"unknown"}}),
            json!({"isError":false,"_meta":{"example/outcome":"failed"}}),
        ] {
            assert!(!succeeded(&result));
        }
        assert!(succeeded(
            &json!({"isError":false,"_meta":{"example/outcome":"succeeded"},"structuredContent":{"status":{"setup":{"outcome":"failed"}}}})
        ));
    }
}
