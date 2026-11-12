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
    let config = Config::from_lookup(|name| std::env::var(name).ok())?;
    let options = config.options();
    let gate = config.gate.clone();
    let backend = Arc::new(
        SshBackend::new(Path::new(&state), config)
            .map_err(|_| "SSH backend state, owner.lock or ssh")?,
    );
    let store = Arc::new(FileStore::new(Path::new(&state)));
    tokio::runtime::Runtime::new()
        .map_err(|_| "Tokio runtime")?
        .block_on(async {
            let server = RuntimeServer::new(backend, store, Arc::new(TlsGate::new(gate)), options)
                .await
                .map_err(|_| "kit recovery")?;
            server
                .serve_stdio()
                .await
                .map_err(|_| "kit transport or cleanup")
        })
}
