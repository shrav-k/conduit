#[cfg(unix)]
fn main() {
    if std::env::args_os().nth(1).is_some_and(|a| a == "--guard") {
        if provider_local_process::guard_main().is_err() {
            std::process::exit(1);
        }
        return;
    }
    if let Err(message) = serve() {
        eprintln!("{message}");
        std::process::exit(1);
    }
}
#[cfg(not(unix))]
fn main() {
    eprintln!("provider-local-process requires Unix");
    std::process::exit(1);
}

#[cfg(unix)]
fn serve() -> Result<(), &'static str> {
    use provider_local_process::{FileStore, LocalBackend, configuration};
    use runtime_provider_kit::{RuntimeServer, gate::TlsGate, names};
    use std::{path::Path, sync::Arc};
    let state = std::env::var_os(names::STATE_DIR_VARIABLE)
        .filter(|p| !p.is_empty())
        .ok_or("provider-local-process needs CONDUIT_STATE_DIR (stateDirVariable) to start")?;
    let binary = std::env::current_exe().map_err(|_| "cannot locate provider executable")?;
    let backend = Arc::new(
        LocalBackend::new(Path::new(&state), &binary)
            .map_err(|_| "cannot open private provider state directory")?,
    );
    let store = Arc::new(FileStore::new(Path::new(&state)));
    let (gate, options) =
        configuration().map_err(|_| "invalid local-process startup configuration")?;
    tokio::runtime::Runtime::new()
        .map_err(|_| "cannot start runtime")?
        .block_on(async move {
            let server = RuntimeServer::new(backend, store, Arc::new(TlsGate::new(gate)), options)
                .await
                .map_err(|_| "provider recovery failed")?;
            server
                .serve_stdio()
                .await
                .map_err(|_| "provider transport or cleanup failed")
        })
}
