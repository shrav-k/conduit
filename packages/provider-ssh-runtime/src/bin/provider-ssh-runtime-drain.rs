#[cfg(unix)]
#[tokio::main]
async fn main() {
    use provider_ssh_runtime::{SshBackend, drainer};
    use std::path::PathBuf;
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    if arguments.len() != 2 {
        eprintln!("SSH-R drainer: durable subdirectory and original state path required");
        return;
    }
    let root = PathBuf::from(&arguments[0]);
    let state = PathBuf::from(&arguments[1]);
    drainer::run(
        &root,
        &state,
        |config, owner| SshBackend::new(&root, None, config.clone(), Some(owner)),
        |message| eprintln!("SSH-R drainer: {message}"),
    )
    .await;
}

#[cfg(not(unix))]
fn main() {
    eprintln!("provider-ssh-runtime-drain requires Unix");
}
