#[cfg(unix)]
fn main() {
    if let Err(place) = serve() {
        eprintln!("SSH-R refuses startup or service: {place}");
        std::process::exit(1);
    }
}
#[cfg(not(unix))]
fn main() {
    eprintln!("provider-ssh-runtime requires Unix");
    std::process::exit(1);
}
#[cfg(unix)]
fn serve() -> Result<(), &'static str> {
    use provider_ssh_runtime::{Config, FileStore, SshBackend};
    use runtime_provider_kit::{RuntimeServer, gate::TlsGate, names};
    use std::{path::Path, sync::Arc};
    let state = std::env::var_os(names::STATE_DIR_VARIABLE)
        .filter(|s| !s.is_empty())
        .ok_or(names::STATE_DIR_VARIABLE)?;
    let config = Config::from_lookup(|name| std::env::var(name).ok()).inspect_err(|place| {
        if *place == "SSH_RUNTIME_PRIVATE_KEY"
            && let Some(durable) = std::env::var_os("SSH_RUNTIME_DURABLE_DIR")
        {
            let expected = provider_ssh_runtime::durable::instance_directory(
                Path::new(&durable),
                Path::new(&state),
            );
            eprintln!(
                "SSH-R refuses startup: SSH_RUNTIME_PRIVATE_KEY must lie in {}",
                expected.display()
            );
        }
    })?;
    let options = config.options();
    let gate = config.gate.clone();
    let root =
        provider_ssh_runtime::durable::instance_directory(&config.durable_dir, Path::new(&state));
    let store = Arc::new(FileStore::new(&root));
    tokio::runtime::Runtime::new()
        .map_err(|_| "Tokio runtime")?
        .block_on(async {
            let owner =
                provider_ssh_runtime::startup::prepare(&config, Path::new(&state), &root, || {
                    let launch = provider_ssh_runtime::startup::Launch::new(
                        &std::env::current_exe()?,
                        &root,
                        Path::new(&state),
                        |name| std::env::var_os(name),
                    );
                    launch.spawn()
                })
                .await
                .map_err(|_| "owner.lock, state-dir or provider-ssh-runtime-drain")?;
            let service = async {
                let backend = Arc::new(
                    provider_ssh_runtime::startup::configured_backend(&root, &config, || {
                        SshBackend::new(&root, Some(Path::new(&state)), config.clone(), Some(owner))
                    })
                    .map_err(|_| "SSH backend state, owner.lock, ssh or drainer-config")?,
                );
                let server =
                    RuntimeServer::new(backend, store, Arc::new(TlsGate::new(gate)), options)
                        .await
                        .map_err(|_| "kit recovery")?;
                server
                    .serve_stdio()
                    .await
                    .map_err(|_| "kit transport or cleanup")
            };
            let keeper = provider_ssh_runtime::startup::keep_alive(&root, || {
                provider_ssh_runtime::startup::Launch::new(
                    &std::env::current_exe()?,
                    &root,
                    Path::new(&state),
                    |name| std::env::var_os(name),
                )
                .spawn()
            });
            tokio::select! {
                result = service => result,
                _ = keeper => unreachable!("drainer keeper runs for the provider's lifetime"),
            }
        })
}
