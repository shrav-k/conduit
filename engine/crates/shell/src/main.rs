mod help;
use conduit_shell::{Config, MissingExecutor, Result, Shell, boundary, runner::Runner};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

fn run() -> Result<()> {
    let raw: Vec<_> = std::env::args().skip(1).collect();
    if raw.is_empty() || raw[0] == "--help" {
        return help::print(None);
    }
    if raw.iter().any(|arg| arg == "--help") {
        return help::print(Some(&raw[0]));
    }
    let mut args = raw.into_iter();
    let command = args.next().ok_or(
        "usage: conduit init|register|submit|replace|install|retire|unregister|inspect|records|serve|tools|call --store DIR [ARGUMENTS]",
    )?;
    let mut store = None;
    let mut project = None;
    let mut packages = Vec::new();
    let mut labels = BTreeMap::new();
    let mut config = Config::default();
    let mut url = None;
    let mut positional = Vec::new();
    let mut as_instance = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--trust-domain" if command == "init" => {
                config.trust_domain = Some(args.next().ok_or("--trust-domain needs DOMAIN")?);
            }
            "--package" if command == "register" => {
                packages.push(args.next().ok_or("--package needs ARCHIVE#SHA256")?)
            }
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
            "--url" => url = Some(args.next().ok_or("--url needs engine address")?),
            "--as" if command == "tools" => {
                as_instance = Some(args.next().ok_or("--as needs INSTANCE")?)
            }
            _ if [
                "call",
                "install",
                "submit",
                "replace",
                "retire",
                "unregister",
            ]
            .contains(&command.as_str())
                && !arg.starts_with('-') =>
            {
                positional.push(arg)
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
            _ if command == "register" && !arg.starts_with('-') && project.is_none() => {
                project = Some(PathBuf::from(arg))
            }
            _ => {
                return Err(format!(
                    "{command}: unknown argument {arg:?}; run conduit {command} --help for arguments"
                ));
            }
        }
    }
    if ![
        "init",
        "register",
        "inspect",
        "records",
        "serve",
        "call",
        "tools",
        "install",
        "submit",
        "replace",
        "retire",
        "unregister",
    ]
    .contains(&command.as_str())
    {
        return Err(format!(
            "unknown command {command:?}; run conduit --help for commands"
        ));
    }
    let store = store.ok_or("--store DIR is required")?;
    if command != "init" && !store.join("state.json").is_file() {
        return Err("store is not initialized; run conduit init".into());
    }
    config.engine_url = url.unwrap_or(Config::for_store(&store)?.engine_url);
    if ["install", "submit", "replace", "retire", "unregister"].contains(&command.as_str()) {
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        return runtime.block_on(conduit_shell::author::run(
            &command,
            &store,
            &positional,
            &repository,
        ));
    }

    if command == "call" || command == "tools" {
        if command == "call" && !(1..=2).contains(&positional.len()) {
            return Err("call needs TOOL [ARGUMENTS-JSON]".into());
        }
        let arguments = positional
            .get(1)
            .map(|s| serde_json::from_str(s).map_err(|_| "invalid arguments JSON".to_string()))
            .transpose()?
            .unwrap_or(json!({}));
        let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        let result = runtime.block_on(conduit_shell::server::client(
            &store,
            as_instance.as_deref(),
            positional.first().map(String::as_str),
            arguments,
        ))?;
        println!("{result}");
        let asked = format!(
            "{command} {}",
            positional
                .first()
                .map(String::as_str)
                .unwrap_or(as_instance.as_deref().unwrap_or("Interface/root"))
        );
        return conduit_shell::author::ensure_success(&result, &asked);
    }
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let shell = match Shell::open(&store, boundary::context(&repository)?, config) {
        Ok(shell) => Arc::new(shell),
        Err(error) if command == "inspect" && error == "store already has an active shell" => {
            let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
            return runtime.block_on(conduit_shell::author::run(
                "inspect",
                &store,
                &[],
                &repository,
            ));
        }
        Err(error) => return Err(error),
    };
    if command == "serve" {
        conduit_shell::server::preflight(&shell)?;
        shell.issue_client_token(&object_id(&shell.state()?, "root")?)?;
        let runner = Runner::new(&shell, std::env::current_exe().map_err(|e| e.to_string())?)?;
        let executor = Arc::new(conduit_shell::server::ServingExecutor::new(
            runner.executor(),
        ));
        let completion_executor: Arc<dyn conduit_shell::Executor> = executor.clone();
        runner.set_completion_executor(Arc::downgrade(&completion_executor));
        let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        let shutdown = tokio_util::sync::CancellationToken::new();
        // Install signal handling before potentially slow setup begins.
        let signal = shutdown.clone();
        let mut terminate = {
            let _entered = runtime.enter();
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .map_err(|e| e.to_string())?
        };
        runtime.spawn(async move {
            tokio::select! { _ = tokio::signal::ctrl_c() => {}, _ = terminate.recv() => {} }
            signal.cancel();
        });
        let setup_shell = shell.clone();
        let setup_executor = executor.clone();
        let setup_shutdown = shutdown.clone();
        let result = runtime.block_on(async {
            let setup_succeeded = tokio::task::spawn_blocking(move || {
                set_up_instances(
                    &setup_shell,
                    setup_executor.as_ref(),
                    None,
                    Some(&setup_shutdown),
                )
            })
            .await
            .map_err(|_| "startup setup failed")??;
            if !shutdown.is_cancelled() {
                conduit_shell::server::serve(shell.clone(), executor.clone(), shutdown).await?;
            }
            if setup_succeeded { Ok(()) } else { Err("serve: one or more startup setups failed; see setup diagnostics (the engine continued serving for repair)".into()) }
        });
        // Release startup tickets if a signal interrupted setup before listening.
        shell.submit(json!({"kind":"recover"}), executor.as_ref())?;
        // Runner owns a runtime: drop it on this synchronous thread.
        drop(completion_executor);
        for diagnostic in runner.diagnostics() {
            eprintln!("runner: {diagnostic}");
        }
        drop(runner);
        return result;
    }
    let mut success = true;
    match command.as_str() {
        "init" => {
            let reply = shell.submit(
                json!({"kind":"first-start","machine":conduit_shell::local_machine(labels)?}),
                &MissingExecutor,
            )?;
            shell.issue_client_token(&object_id(&shell.state()?, "root")?)?;
            println!("{reply}");
            success &= conduit_shell::author::succeeded(&reply);
        }
        "records" => {
            let export = shell.submit(json!({"kind":"export-records"}), &MissingExecutor)?;
            for record in export["records"]
                .as_array()
                .ok_or("invalid record export")?
            {
                println!("{record}");
            }
            if export["complete"] != true {
                success = false;
                eprintln!(
                    "records: export has gaps: {}; preserve the available records and check retention",
                    export["gaps"]
                );
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
                success &= conduit_shell::author::succeeded(&reply);
                println!(
                    "{}",
                    conduit_shell::author::inspection_row(&state.objects, object, reply)
                );
            }
        }
        "register" => {
            let project = project.ok_or("register needs PROJECT_DIR")?;
            for package in &packages {
                boundary::install_package(&shell, &repository, package)?;
            }
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
            let reply = shell.submit(json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,"project":boundary::project(&project,&spec)?,"request":{"sender":root,"caller":root,"conduit":conduit["id"],"target":conduit["target"],"operation":format!("{}#register.v1",shell.context.core_package)}}), &MissingExecutor)?;
            println!("{reply}");
            if reply["accepted"] != true {
                return Err("register: project was rejected; see printed diagnostics and correct the project before retrying".into());
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
                success &= conduit_shell::author::succeeded(&reply);
            }
            let runner = Runner::new(&shell, std::env::current_exe().map_err(|e| e.to_string())?)?;
            success &= set_up_instances(&shell, &runner, Some(registered), None)?;
            for diagnostic in runner.diagnostics() {
                eprintln!("runner: {diagnostic}");
            }
        }
        _ => unreachable!(),
    }
    if success {
        Ok(())
    } else {
        Err(format!(
            "{command}: one or more printed results did not succeed; see their diagnostics"
        ))
    }
}
fn set_up_instances(
    shell: &Shell,
    executor: &dyn conduit_shell::Executor,
    registered: Option<&[Value]>,
    shutdown: Option<&tokio_util::sync::CancellationToken>,
) -> Result<bool> {
    let mut success = true;
    let state = shell.state()?;
    let root = object_id(&state, "root")?;
    for kind in ["Pool", "Fleet", "Resource", "Agent", "Interface"] {
        for object in state.objects.iter().filter(|v| {
            v["kind"] == kind
                && (v["state"] == "registered"
                    || (registered.is_none()
                        && v["state"] == "ready"
                        && v["resolved_implementations"]
                            .as_object()
                            .is_some_and(|items| {
                                items.values().any(|item| {
                                    item["source"] != "engine"
                                        && (item["item"].get("command").is_some()
                                            || item["item"].get("package").is_some())
                                })
                            })))
                && v.get("spec").is_none()
                && registered.is_none_or(|items| items.iter().any(|new| new["id"] == v["id"]))
        }) {
            if shutdown.is_some_and(|s| s.is_cancelled()) {
                return Ok(success);
            }
            // ACC-033 rows 4/4A: member setup uses its owning container.
            let tool = if kind == "Resource" || kind == "Agent" {
                let route = state.objects.iter().find(|v| {
                    v["kind"] == "Conduit"
                        && reference_id(&v["consumer"]) == &json!(root)
                        && reference_id(&v["target"]) == reference_id(&object["owner"])
                        && v["open"] == true
                        && v["operations"].as_array().is_some_and(|ops| {
                            ops.contains(&json!(format!(
                                "{}#set-up.v1",
                                shell.context.core_package
                            )))
                        })
                });
                route
                    .and_then(local_name)
                    .map(|name| format!("{name}.set-up.v1"))
                    .unwrap_or_else(|| "management.set-up.v1".into())
            } else {
                "management.set-up.v1".into()
            };
            let ticket = shell.begin(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":tool,"arguments":{"instance":{"kind":kind,"name":object["name"]}}}}), executor)?;
            let reply = if let Some(shutdown) = shutdown {
                loop {
                    if let Some(reply) = ticket.try_reply()? {
                        break reply;
                    }
                    if shutdown.is_cancelled() {
                        if let Some(invocation) = ticket.invocation() {
                            shell.submit(
                                json!({"kind":"cancel","invocation":invocation}),
                                executor,
                            )?;
                        }
                        return Ok(success);
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            } else {
                ticket.wait()?
            };
            if registered.is_some() {
                println!("{reply}");
            }
            if !conduit_shell::author::succeeded(&reply) {
                let message = format!("setup of {} failed: {reply}", object["name"]);
                if registered.is_some() {
                    return Err(message);
                }
                success = false;
                eprintln!("conduit: {message}");
            }
        }
    }
    Ok(success)
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
        let asked: Vec<_> = std::env::args().skip(1).take(1).collect();
        eprintln!("conduit {}: {error}", asked.join(" "));
        std::process::exit(1);
    }
}
