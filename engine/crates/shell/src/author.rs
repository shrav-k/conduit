//! Author workflows: every management action is a root MCP request to the serving engine.
use crate::{Result, boundary, server, store};
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::Path;

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

/// CLI-only supplement: inspect.v1 has no defined consumer-Conduit inventory.
pub fn inspection_row(objects: &[Value], object: &Value, reply: Value) -> Value {
    let mut row = json!({"object":{"kind":object["kind"],"name":object["name"]},"result":reply});
    if row["result"]["isError"] == false
        && object.get("spec").is_none()
        && object["kind"] != "Conduit"
    {
        row["consumedConduits"] = json!(objects.iter().filter(|p| {
            p["kind"] == "Conduit" && p.get("spec").is_none()
                && reference(&p["consumer"]) == &object["id"]
        }).map(|p| {
            let target = objects.iter().find(|o| &o["id"] == reference(&p["target"]));
            json!({"name":p["name"],"target":target.map(|o| json!({"kind":o["kind"],"name":o["name"],"id":o["id"]})).unwrap_or_else(|| p["target"].clone()),"open":p["open"] == true})
        }).collect::<Vec<_>>());
    }
    if row["result"]["isError"] == false
        && let Some(past) = object.get("past_conditions")
    {
        row["pastConditions"] = past.clone();
    }
    row
}
fn named(argument: &str) -> Result<Value> {
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
fn route(directory: &Path, object: &Value, operation: &str) -> Result<String> {
    let state = state(directory)?;
    let objects = state["objects"].as_array().ok_or("missing objects")?;
    let target = objects
        .iter()
        .find(|o| o["kind"] == object["kind"] && o["name"] == object["name"]);
    let owner = target
        .filter(|o| o["kind"] == "Resource" && o.get("spec").is_none())
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
            .ok_or_else(|| format!("{operation} for {}/{}: no root management route to owning Pool {}; open a root Conduit to that Pool offering {operation}", object["kind"].as_str().unwrap_or("?"), object["name"].as_str().unwrap_or("?"), owner["name"].as_str().unwrap_or("?")))?;
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
async fn revision(directory: &Path, object: &Value) -> Result<Value> {
    let tool = route(directory, object, "inspect.v1")?;
    let reply = call(directory, &tool, json!({"object":object})).await?;
    reply["structuredContent"]
        .get("revision")
        .cloned()
        .ok_or("inspection omitted revision".into())
}

pub async fn run(
    command: &str,
    directory: &Path,
    args: &[String],
    repository: &Path,
) -> Result<()> {
    let expected = match command {
        "inspect" => 0,
        "replace" => 2,
        _ => 1,
    };
    if args.len() != expected {
        return Err(format!(
            "{command}: expected {expected} positional argument(s), received {}; run conduit {command} --help for usage and an example",
            args.len()
        ));
    }
    // Establish the serving path before doing any local workflow work. This never
    // opens the live store for writing or recovery.
    let endpoint = url::Url::parse(&crate::Config::for_store(directory)?.engine_url)
        .map_err(|_| "invalid engine URL")?;
    let host = match endpoint.host().ok_or("missing engine host")? {
        url::Host::Ipv6(ip) => ip.to_string(),
        _ => endpoint
            .host_str()
            .ok_or("missing engine host")?
            .to_string(),
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
    server::client(directory, None, None, json!({}))
        .await
        .map_err(|e| format!("conduit serve must hold this store: {e}"))?;
    match command {
        "inspect" => {
            let current = state(directory)?;
            let objects = current["objects"].as_array().ok_or("missing objects")?;
            let mut success = true;
            for object in objects {
                let reference = json!({"kind":object["kind"],"name":object["name"]});
                let tool = route(directory, &reference, "inspect.v1")
                    .unwrap_or_else(|_| "management.inspect.v1".into());
                let reply =
                    server::client(directory, None, Some(&tool), json!({"object":reference}))
                        .await?;
                success &= succeeded(&reply);
                println!("{}", inspection_row(objects, object, reply));
            }
            if !success {
                return Err("inspect: one or more objects could not be inspected; see their results and check root's management Conduits".into());
            }
        }
        "install" => {
            if args.len() != 1 {
                return Err("install needs ARCHIVE#SHA256".into());
            }
            call(
                directory,
                "management.install.v1",
                boundary::install_arguments(&args[0])?,
            )
            .await?;
        }
        "replace" => {
            if args.len() != 2 {
                return Err("replace needs DIR KIND/NAME".into());
            }
            let object = named(&args[1])?;
            let spec = conduit_loader::Specification::read(&repository.join("spec"))
                .map_err(|e| e.to_string())?;
            let project = boundary::project(Path::new(&args[0]), &spec).map_err(|error| format!("{command}: cannot load project {}: {error}; correct its documents and manifest before retrying", args[0]))?;
            let templates: Vec<_> = project
                .documents
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
            let mut input = json!({"template":templates[0],"contracts":project.documents.iter().filter(|d| conduit_core::packages::contract(d)).collect::<Vec<_>>(),"revision":rev});
            if let Some(manifest) = project.manifest {
                input["manifest"] = manifest;
            }
            call(directory, "management.replace.v1", input).await?;
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
            let project = boundary::project(Path::new(&args[0]), &spec).map_err(|error| format!("{command}: cannot load project {}: {error}; correct its documents and manifest before retrying", args[0]))?;
            let mut input = json!({"documents":project.documents});
            if let Some(manifest) = project.manifest {
                input["manifest"] = manifest;
            }
            if requires_directory(&input) {
                return Err("a submission through the serving engine takes absolute values only: relative fetch or program needs a directory".into());
            }
            let mut steps = Vec::new();
            let result: Result<()> = async {
            let reply = submit_step(directory, "management.register.v1", input, "registration".into(), &mut steps).await?;
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
                    && reference(&o["consumer"]) == &root["id"]
                    && created.iter().any(|n| n["id"] == o["id"])
            }) {
                submit_step(directory, "access.open.v1", json!({"conduit":o["name"]}), format!("open Conduit/{}", o["name"].as_str().unwrap_or("?")), &mut steps).await?;
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
) -> Result<Value> {
    match server::client(directory, None, Some(tool), args).await {
        Ok(reply) => {
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
