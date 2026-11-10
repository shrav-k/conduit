use conduit_shell::{Config, MissingExecutor, Result, Shell, boundary, runner::Runner};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

fn run() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let command = args.next().ok_or("usage: conduit init|register|inspect|records --store DIR [PROJECT_DIR] [--label KEY=VALUE]")?;
    let mut store = None;
    let mut project = None;
    let mut labels = BTreeMap::new();
    let mut config = Config::default();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--store" => store = Some(PathBuf::from(args.next().ok_or("--store needs DIR")?)),
            "--label" if command == "init" => {
                let label = args.next().ok_or("--label needs KEY=VALUE")?;
                let (key, value) = label.split_once('=').ok_or("--label needs KEY=VALUE")?;
                labels.insert(key.into(), value.into());
            }
            "--output-cap-bytes" => {
                config.max_output_bytes = args
                    .next()
                    .ok_or("missing output cap")?
                    .parse()
                    .map_err(|_| "invalid output cap")?
            }
            "--output-age-seconds" => {
                config.max_output_age = std::time::Duration::from_secs(
                    args.next()
                        .ok_or("missing output age")?
                        .parse()
                        .map_err(|_| "invalid output age")?,
                )
            }
            "--pass-environment" => config
                .pass_environment
                .push(args.next().ok_or("missing environment name")?),
            "--initialize-timeout-seconds" => {
                config.initialize_timeout = std::time::Duration::from_secs(
                    args.next()
                        .ok_or("missing initialize timeout")?
                        .parse()
                        .map_err(|_| "invalid initialize timeout")?,
                )
            }
            "--call-timeout-seconds" => {
                config.call_timeout = std::time::Duration::from_secs(
                    args.next()
                        .ok_or("missing call timeout")?
                        .parse()
                        .map_err(|_| "invalid call timeout")?,
                )
            }
            "--url" => config.engine_url = args.next().ok_or("--url needs engine address")?,
            _ if command == "register" && !arg.starts_with('-') && project.is_none() => {
                project = Some(PathBuf::from(arg))
            }
            _ => return Err("unknown command argument".into()),
        }
    }
    if !["init", "register", "inspect", "records"].contains(&command.as_str()) {
        return Err("unknown command; serve comes in order 022".into());
    }
    let store = store.ok_or("--store DIR is required")?;
    if command != "init" && !store.join("state.json").is_file() {
        return Err("store is not initialized; run conduit init".into());
    }
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let shell = Arc::new(Shell::open(
        &store,
        boundary::context(&repository)?,
        config,
    )?);
    match command.as_str() {
        "init" => println!(
            "{}",
            shell.submit(
                json!({"kind":"first-start","machine":conduit_shell::local_machine(labels)?}),
                &MissingExecutor
            )?
        ),
        "records" => {
            let export = shell.submit(json!({"kind":"export-records"}), &MissingExecutor)?;
            for record in export["records"]
                .as_array()
                .ok_or("invalid record export")?
            {
                println!("{record}");
            }
            if export["complete"] != true {
                eprintln!("record export has gaps: {}", export["gaps"]);
            }
        }
        "inspect" => {
            let state = shell.state()?;
            let root = object_id(&state, "root")?;
            for object in &state.objects {
                let route = state.objects.iter().find(|v| {
                    v["kind"] == "Conduit"
                        && reference_id(&v["consumer"]) == &json!(root)
                        && reference_id(&v["target"]) == reference_id(&object["owner"])
                        && v["open"] == true
                });
                let tool = route
                    .and_then(local_name)
                    .map(|name| format!("{name}.inspect.v1"))
                    .unwrap_or_else(|| "management.inspect.v1".into());
                let reference = json!({"kind":object["kind"],"name":object["name"]});
                let reply = shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":tool,"arguments":{"object":reference}}}), &MissingExecutor)?;
                println!("{}", json!({"object":reference,"result":reply}));
            }
        }
        "register" => {
            let project = project.ok_or("register needs PROJECT_DIR")?;
            let spec = conduit_loader::Specification::read(&repository.join("spec"))
                .map_err(|e| e.to_string())?;
            let state = shell.state()?;
            let root = object_id(&state, "root")?;
            let conduit = state
                .objects
                .iter()
                .find(|v| {
                    reference_id(&v["consumer"]) == &json!(root) && v["local_name"] == "management"
                })
                .ok_or("root management route is absent")?;
            let reply = shell.submit(json!({"kind":"register-project","snapshots":boundary::s1_snapshots(&repository)?,"builtin":shell.context.builtin,"project":boundary::project(&project,&spec)?,"request":{"sender":root,"caller":root,"conduit":conduit["id"],"target":conduit["target"],"operation":format!("{}#register.v1",shell.context.core_package)}}), &MissingExecutor)?;
            println!("{reply}");
            if reply["accepted"] != true {
                return Ok(());
            }
            let registered = reply["objects"]
                .as_array()
                .ok_or("registration omitted its objects")?;
            // This CLI workflow explicitly asks for setup after registration. Every open
            // and setup still goes through Invoke and the core's current authority.
            let state = shell.state()?;
            for conduit in state.objects.iter().filter(|v| {
                v["kind"] == "Conduit"
                    && reference_id(&v["consumer"]) == &json!(root)
                    && v["open"] != true
                    && v.get("spec").is_none()
                    && registered.iter().any(|new| new["id"] == v["id"])
            }) {
                let reply = shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":"access.open.v1","arguments":{"conduit":conduit["name"]}}}), &MissingExecutor)?;
                println!("{reply}");
            }
            let runner = Runner::new(&shell, std::env::current_exe().map_err(|e| e.to_string())?)?;
            for kind in ["Pool", "Resource", "Agent", "Interface"] {
                for object in state.objects.iter().filter(|v| {
                    v["kind"] == kind
                        && v["state"] == "registered"
                        && v.get("spec").is_none()
                        && registered.iter().any(|new| new["id"] == v["id"])
                }) {
                    let tool = if kind == "Resource" {
                        let route = state
                            .objects
                            .iter()
                            .find(|v| {
                                v["kind"] == "Conduit"
                                    && reference_id(&v["consumer"]) == &json!(root)
                                    && reference_id(&v["target"]) == reference_id(&object["owner"])
                                    && v["operations"].as_array().is_some_and(|ops| {
                                        ops.contains(&json!(format!(
                                            "{}#set-up.v1",
                                            shell.context.core_package
                                        )))
                                    })
                            })
                            .ok_or("no root setup route to owning Pool")?;
                        format!(
                            "{}.set-up.v1",
                            local_name(route).ok_or("setup route lacks local name")?
                        )
                    } else {
                        "management.set-up.v1".into()
                    };
                    let reply = shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":tool,"arguments":{"instance":{"kind":kind,"name":object["name"]}}}}), &runner)?;
                    println!("{reply}");
                }
            }
            for diagnostic in runner.diagnostics() {
                eprintln!("runner: {diagnostic}");
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
fn reference_id(value: &Value) -> &Value {
    value.get("id").unwrap_or(value)
}
fn local_name(value: &Value) -> Option<&str> {
    value["local_name"]
        .as_str()
        .or_else(|| value["nested"]["name"].as_str())
        .or_else(|| value["name"].as_str())
}
fn object_id(state: &conduit_core::State, name: &str) -> Result<String> {
    state
        .objects
        .iter()
        .find(|v| v["name"] == name && v.get("spec").is_none())
        .and_then(|v| v["id"].as_str())
        .map(str::to_owned)
        .ok_or_else(|| format!("missing first-start object {name}"))
}
fn main() {
    let result = if std::env::args().nth(1).as_deref() == Some("__run-guard") {
        conduit_shell::runner::guard_main()
    } else {
        run()
    };
    if let Err(error) = result {
        eprintln!("conduit: {error}");
        std::process::exit(1);
    }
}
