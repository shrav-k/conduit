mod help;
use conduit_shell::{
    Config, MissingExecutor, Result, Shell, boundary, runner::Runner, user_config,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

fn run() -> Result<()> {
    let raw: Vec<_> = std::env::args().skip(1).collect();
    if raw.is_empty() || raw[0] == "--help" {
        return help::print(None);
    }
    if raw == ["--version"] {
        println!("{}", version_line());
        return Ok(());
    }
    let help_requested = if raw[0] == "answer" {
        raw[1..] == ["--help"]
    } else {
        (1..raw.len()).any(|i| {
            raw[i] == "--help"
                && !matches!(
                    (raw[0].as_str(), raw[i - 1].as_str()),
                    ("sign-in", "--login") | ("call", "--arguments-file") | ("pending", "--agent")
                )
        })
    };
    if help_requested {
        return help::print(Some(&raw[0]));
    }
    let mut args = raw.into_iter();
    let command = args
        .next()
        .ok_or("usage: conduit COMMAND [ARGUMENTS]; run conduit --help for commands")?;
    if command == "value" {
        use std::io::IsTerminal;
        let runtime = tokio::runtime::Runtime::new().map_err(|error| error.to_string())?;
        let arguments: Vec<_> = args.collect();
        let mut person = conduit_shell::sign_in::Terminal;
        let code = runtime.block_on(conduit_shell::value::run(
            &arguments,
            &mut person,
            std::io::stdin().is_terminal(),
            &mut std::io::stdout().lock(),
        ));
        if code != 0 {
            std::process::exit(code);
        }
        return Ok(());
    }
    if matches!(command.as_str(), "plan" | "apply") {
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let code = match tokio::runtime::Runtime::new() {
            Ok(runtime) => {
                let version = version_line();
                let args = args.collect();
                if command == "plan" {
                    runtime.block_on(conduit_shell::plan::command(args, &repository, &version))
                } else {
                    runtime.block_on(conduit_shell::plan::apply_command(
                        args,
                        &repository,
                        &version,
                    ))
                }
            }
            Err(error) => {
                eprintln!("conduit {command}: cannot start client: {error}");
                1
            }
        };
        if code != 0 {
            std::process::exit(code);
        }
        return Ok(());
    }
    let mut store = None;
    let mut project = None;
    let mut packages = Vec::new();
    let mut labels = BTreeMap::new();
    let mut config = Config::default();
    let mut url = None;
    let mut positional = Vec::new();
    let mut as_instance = None;
    let mut login = None;
    let mut arguments_file = None;
    let mut agent = None;
    let mut setup = false;
    let mut install_mirrors = Vec::new();
    let mut install_dry_run = false;
    let mut init_store_option = None;
    while let Some(arg) = args.next() {
        // ANSWER is passed as given, even where it looks like an option; ACTION is an
        // engine-issued ID and goes through the normal match.
        if command == "answer" && positional.len() == 1 {
            positional.push(arg);
            continue;
        }
        if command == "init"
            && matches!(
                arg.as_str(),
                "--label"
                    | "--trust-domain"
                    | "--url"
                    | "--output-cap-bytes"
                    | "--output-age-seconds"
                    | "--initialize-timeout-seconds"
                    | "--call-timeout-seconds"
                    | "--pass-environment"
            )
        {
            init_store_option.get_or_insert_with(|| arg.clone());
        }
        match arg.as_str() {
            "--help" if command == "answer" => return help::print(Some(&command)),
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
            // DL13/IFC-042: only serve can name the fixed HTTPS token audience.
            "--https-url" if command == "serve" => {
                config.engine_https_url = Some(args.next().ok_or("--https-url needs URL")?);
            }
            "--https-listen" if command == "serve" => {
                // DL13: parse a numeric socket address; never resolve a host name.
                config.https_listen = Some(
                    args.next()
                        .ok_or("--https-listen needs ADDRESS")?
                        .parse()
                        .map_err(|_| "--https-listen must be 127.0.0.1:PORT or [::1]:PORT")?,
                );
            }
            // DL13/PRV-022: read here; Shell::open checks and copies under its lease.
            "--https-roots" if command == "serve" => {
                let file = args.next().ok_or("--https-roots needs FILE")?;
                config.https_roots = Some(
                    std::fs::read_to_string(&file)
                        .map_err(|e| format!("--https-roots cannot read FILE: {e}"))?,
                );
            }
            "--as" if command == "tools" => {
                as_instance = Some(args.next().ok_or("--as needs INSTANCE")?)
            }
            "--mirror" if command == "install" => {
                let Some(directory) = args.next() else {
                    eprintln!("conduit install: --mirror needs DIR");
                    std::process::exit(2);
                };
                install_mirrors.push(PathBuf::from(directory));
            }
            "--dry-run" if command == "install" => install_dry_run = true,
            "--login" if command == "sign-in" => {
                login = Some(args.next().ok_or("--login needs NAME")?)
            }
            "--arguments-file" if command == "call" => {
                arguments_file = Some(args.next().ok_or("--arguments-file needs PATH or -")?)
            }
            "--agent" if command == "pending" => {
                agent = Some(args.next().ok_or("--agent needs NAME")?)
            }
            "--setup" if command == "inspect" => setup = true,
            _ if [
                "call",
                "install",
                "submit",
                "replace",
                "retire",
                "unregister",
                "pending",
                "answer",
                "sign-in",
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
        "pending",
        "answer",
        "sign-in",
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
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let contextual = store.is_none();
    let store = match store {
        Some(store) => store,
        None if command == "init" => {
            if let Some(option) = init_store_option {
                return Err(format!("init: {option} requires --store DIR"));
            }
            return init_project(&repository);
        }
        None if matches!(command.as_str(), "serve" | "register") => {
            return Err("--store DIR is required".into());
        }
        None => {
            let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
            match runtime.block_on(user_config::current()) {
                Ok((file, context)) => {
                    eprintln!(
                        "conduit {command}: using {file:?} for store {:?} at {}",
                        context.store,
                        user_config::escape_line_breaks(&context.url)
                    );
                    context.store
                }
                Err(error) => {
                    eprintln!(
                        "conduit {command}: {error}; start conduit serve --store DIR or give --store DIR"
                    );
                    std::process::exit(2);
                }
            }
        }
    };
    if command != "init" && !store.join("state.json").is_file() {
        return Err("store is not initialized; run conduit init".into());
    }
    let fixed = Config::for_store(&store)?;
    config.engine_url = url.unwrap_or(fixed.engine_url);
    config.engine_https_url = config.engine_https_url.or(fixed.engine_https_url);
    // PRV-022: Shell::open inherits stored roots under the lease when none are supplied.
    // DL13: check serve's option combinations before opening or writing the store.
    if command == "serve" {
        config.validate_serve()?;
    }
    if [
        "install",
        "submit",
        "replace",
        "retire",
        "unregister",
        "pending",
        "answer",
    ]
    .contains(&command.as_str())
        || (contextual && command == "inspect")
    {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        return runtime.block_on(conduit_shell::author::run(
            &command,
            &store,
            &positional,
            &repository,
            conduit_shell::author::Options {
                agent: agent.as_deref(),
                setup,
                serving_checked: contextual,
                install_mirrors: &install_mirrors,
                install_dry_run,
            },
        ));
    }

    if contextual && command == "records" {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        return print_records(runtime.block_on(conduit_shell::server::records_client(&store))?);
    }

    if command == "sign-in" {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        let mut person = conduit_shell::sign_in::Terminal;
        return runtime.block_on(conduit_shell::sign_in::run(
            &store,
            &positional,
            login.as_deref(),
            &repository,
            &mut person,
        ));
    }

    if command == "call" || command == "tools" {
        if command == "call" && !(1..=2).contains(&positional.len()) {
            return Err("call needs TOOL [ARGUMENTS-JSON | --arguments-file PATH|-]".into());
        }
        let arguments = if let Some(path) = arguments_file.as_deref() {
            read_arguments(path, positional.get(1).is_some())?
        } else {
            positional
                .get(1)
                .map(|s| serde_json::from_str(s).map_err(|_| "invalid arguments JSON".to_string()))
                .transpose()?
                .unwrap_or(json!({}))
        };
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
    let shell = match Shell::open(&store, boundary::context(&repository)?, config) {
        Ok(shell) => Arc::new(shell),
        Err(error) if command == "inspect" && error == "store already has an active shell" => {
            let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
            return runtime.block_on(conduit_shell::author::run(
                "inspect",
                &store,
                &[],
                &repository,
                conduit_shell::author::Options {
                    setup,
                    ..Default::default()
                },
            ));
        }
        Err(error) if command == "records" && error == "store already has an active shell" => {
            let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
            let export = runtime.block_on(conduit_shell::server::records_client(&store))?;
            return print_records(export);
        }
        Err(error) => return Err(error),
    };
    if command == "serve" {
        conduit_shell::server::preflight(&shell)?;
        let directory = store
            .canonicalize()
            .map_err(|error| format!("cannot resolve store path: {error}"))?;
        let current = user_config::EngineContext {
            store: directory,
            url: shell.config.engine_url.clone(),
            version: version_line(),
        };
        let ready = json!({"ready":{"url":current.url,"store":current.store}});
        // DL2/DL13: token subjects come from the shared last-commit snapshot.
        shell.issue_client_token(&object_id(shell.state()?.as_ref(), "root")?)?;
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
                conduit_shell::server::serve_with_ready(shell.clone(), executor.clone(), shutdown, || {
                    if let Err(error) = current.record() {
                        eprintln!("conduit serve: {error}");
                    }
                    let mut stdout = io::stdout().lock();
                    if let Err(error) = writeln!(stdout, "{ready}").and_then(|()| stdout.flush()) {
                        let _ = writeln!(io::stderr().lock(), "conduit serve: cannot write ready line: {error}");
                    }
                }).await?;
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
            // DL2/DL13: token subjects come from the shared last-commit snapshot.
            shell.issue_client_token(&object_id(shell.state()?.as_ref(), "root")?)?;
            println!("{reply}");
            success &= conduit_shell::author::succeeded(&reply);
        }
        "records" => {
            let export = shell.submit(json!({"kind":"export-records"}), &MissingExecutor)?;
            return print_records(export);
        }
        "inspect" => {
            let state = shell.state()?;
            let root = object_id(&state, "root")?;
            for object in conduit_shell::author::inspection_objects(&state.objects, setup) {
                let reference = json!({"kind":object["kind"],"name":object["name"]});
                let tool = if setup {
                    conduit_shell::author::setup_inspection_tool(&state.objects, &reference)
                } else {
                    let route = state.objects.iter().find(|v| {
                        v["kind"] == "Conduit"
                            && reference_id(&v["consumer"]) == &json!(root)
                            && reference_id(&v["target"]) == reference_id(&object["owner"])
                            && v["open"] == true
                    });
                    route
                        .and_then(local_name)
                        .map(|name| format!("{name}.inspect.v1"))
                        .unwrap_or_else(|| "management.inspect.v1".into())
                };
                let reply = shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},"tool":tool,"arguments":{"object":reference}}}), &MissingExecutor)?;
                success &= conduit_shell::author::succeeded(&reply);
                conduit_shell::author::print_inspection_row(&state.objects, object, reply, setup);
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

fn version_line() -> String {
    format!(
        "conduit {} (commit {})",
        env!("CARGO_PKG_VERSION"),
        env!("CLI_GIT_COMMIT")
    )
}

fn init_project(repository: &Path) -> Result<()> {
    use std::fs::{self, OpenOptions};
    use std::io::ErrorKind;

    for name in conduit_loader::MANIFEST_NAMES {
        match fs::symlink_metadata(name) {
            Ok(_) => {
                eprintln!("conduit init: {name} already exists; nothing changed");
                return Ok(());
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(format!("init: cannot inspect {name}: {error}")),
        }
    }
    let spec = conduit_loader::Specification::read(&repository.join("spec"))
        .map_err(|error| error.to_string())?;
    let contents = format!("apiVersion: {}\npackages: {{}}\n", spec.names().api_version);
    let file = "conduit.yaml";
    match OpenOptions::new().write(true).create_new(true).open(file) {
        Ok(mut manifest) => {
            if let Err(error) = manifest.write_all(contents.as_bytes()) {
                drop(manifest);
                let _ = fs::remove_file(file);
                return Err(format!("init: cannot write {file}: {error}"));
            }
            Ok(())
        }
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            eprintln!("conduit init: {file} already exists; nothing changed");
            Ok(())
        }
        Err(error) => Err(format!("init: cannot create {file}: {error}")),
    }
}

fn read_arguments(path: &str, has_inline: bool) -> Result<Value> {
    let source = if path == "-" {
        "standard input".to_string()
    } else {
        format!("{path:?}")
    };
    if has_inline {
        return Err(format!(
            "cannot combine inline ARGUMENTS-JSON with --arguments-file from {source}"
        ));
    }
    let bytes = if path == "-" {
        let mut bytes = Vec::new();
        io::stdin().read_to_end(&mut bytes).map(|_| bytes)
    } else {
        std::fs::read(path)
    }
    .map_err(|error| format!("cannot read arguments from {source}: {error}"))?;
    serde_json::from_slice(&bytes).map_err(|error| {
        format!(
            "invalid arguments JSON from {source} at line {}, column {}",
            error.line(),
            error.column()
        )
    })
}

fn print_records(export: Value) -> Result<()> {
    for record in export["records"]
        .as_array()
        .ok_or("invalid record export")?
    {
        println!("{record}");
    }
    if export["complete"] != true {
        eprintln!(
            "records: export has gaps: {}; preserve the available records and check retention",
            export["gaps"]
        );
        return Err(
            "records: one or more printed results did not succeed; see their diagnostics".into(),
        );
    }
    Ok(())
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
    conduit_shell::install_crypto_provider();
    if std::env::args_os()
        .nth(1)
        .is_some_and(|arg| arg == "__http-bridge")
    {
        std::process::exit(conduit_shell::http_bridge::main_entry());
    }
    let asked = std::env::args().nth(1).unwrap_or_default();
    let result = if asked == "__run-guard" {
        conduit_shell::runner::guard_main()
    } else {
        run()
    };
    if let Err(error) = &result {
        eprintln!("conduit {asked}: {error}");
    }
    if !matches!(asked.as_str(), "pending" | "__run-guard")
        && let Some(hint) = conduit_shell::author::unanswered_hint()
    {
        eprintln!("{hint}");
    }
    if result.is_err() {
        std::process::exit(1);
    }
}
