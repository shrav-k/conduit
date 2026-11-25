use conduit_github::{Front, SUPERVISE_UPSTREAM, supervise_upstream};
use rmcp::ServiceExt;
use std::{process::ExitCode, time::Duration};
use tokio::signal::unix::{SignalKind, signal};

async fn run(write: bool) -> Result<(), ()> {
    let mut terminate = signal(SignalKind::terminate()).map_err(|_| ())?;
    let mut interrupt = signal(SignalKind::interrupt()).map_err(|_| ())?;
    let mut hangup = signal(SignalKind::hangup()).map_err(|_| ())?;
    let mut quit = signal(SignalKind::quit()).map_err(|_| ())?;
    let front = if write {
        Front::writer()
    } else {
        Front::default()
    };
    let session = async {
        let service = front
            .clone()
            .serve(rmcp::transport::stdio())
            .await
            .map_err(|_| ())?;
        service.waiting().await.map(|_| ()).map_err(|_| ())
    };
    let result = tokio::select! {
        result = session => result,
        _ = terminate.recv() => Ok(()),
        _ = interrupt.recv() => Ok(()),
        _ = hangup.recv() => Ok(()),
        _ = quit.recv() => Ok(()),
    };
    front.shutdown().await;
    result
}

fn main() -> ExitCode {
    let supervising = std::env::args_os()
        .nth(1)
        .is_some_and(|arg| arg == SUPERVISE_UPSTREAM);
    let write = std::env::args_os().any(|arg| arg == "--write");
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map(|runtime| {
            let result = runtime.block_on(async {
                if supervising {
                    supervise_upstream(write).await.map_err(|_| ())
                } else {
                    run(write).await
                }
            });
            // A signal can leave the peer's stdin pipe open. The child has already
            // been reaped, so do not wait indefinitely for Tokio's blocking read.
            runtime.shutdown_timeout(Duration::from_millis(100));
            result
        });
    if matches!(result, Ok(Ok(()))) {
        ExitCode::SUCCESS
    } else {
        if !supervising {
            let name = if write {
                "pull-request writer"
            } else {
                "repository front"
            };
            eprintln!("{name} could not serve stdio");
        }
        ExitCode::FAILURE
    }
}
