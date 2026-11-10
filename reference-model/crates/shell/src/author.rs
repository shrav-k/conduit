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
fn named(argument: &str) -> Result<Value> {
    let (kind, name) = argument.split_once('/').ok_or("expected KIND/NAME")?;
    if !["Agent", "Interface", "Resource", "Pool", "Conduit"].contains(&kind)
        || name.is_empty()
        || name.contains('/')
    {
        return Err("expected KIND/NAME with a document kind".into());
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
            .ok_or("no root management route to owning Pool")?;
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
    if reply["isError"] == true {
        return Err("management request did not succeed".into());
    }
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
            let project = boundary::project(Path::new(&args[0]), &spec)?;
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
            let project = boundary::project(Path::new(&args[0]), &spec)?;
            let mut input = json!({"documents":project.documents});
            if let Some(manifest) = project.manifest {
                input["manifest"] = manifest;
            }
            if requires_directory(&input) {
                return Err("a submission through the serving engine takes absolute values only: relative fetch or program needs a directory".into());
            }
            let reply = call(directory, "management.register.v1", input).await?;
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
                call(directory, "access.open.v1", json!({"conduit":o["name"]})).await?;
            }
            for kind in ["Pool", "Resource", "Agent", "Interface"] {
                for o in objects.iter().filter(|o| {
                    o["kind"] == kind
                        && o.get("spec").is_none()
                        && created.iter().any(|n| n["id"] == o["id"])
                }) {
                    let object = json!({"kind":kind,"name":o["name"]});
                    call(
                        directory,
                        &route(directory, &object, "set-up.v1")?,
                        json!({"instance":object}),
                    )
                    .await?;
                }
            }
        }
        _ => return Err("unknown author command".into()),
    }
    Ok(())
}
