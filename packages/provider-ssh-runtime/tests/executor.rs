use provider_ssh_runtime::ssh::{Executor, Invocation, ProcessExecutor};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn waits_for_eof() -> Invocation {
    Invocation {
        program: "/bin/sh".into(),
        args: vec!["-c".into(), "/bin/cat; printf eof-received".into()],
    }
}

#[tokio::test]
async fn command_drops_stdin_after_delivery_so_child_can_exit() {
    let output = tokio::time::timeout(
        Duration::from_secs(10),
        ProcessExecutor.execute(waits_for_eof(), b"synthetic-input\n".to_vec()),
    )
    .await
    .expect("command must close its input pipe")
    .unwrap();
    assert!(output.success);
    assert_eq!(output.stdout, b"synthetic-input\neof-received");
}

#[tokio::test]
async fn session_shutdown_closes_stdin_and_preserves_stdout_until_eof() {
    let mut session = ProcessExecutor
        .session(waits_for_eof(), tempfile::tempfile().unwrap())
        .await
        .unwrap();
    session
        .channel
        .write_all(b"synthetic-session\n")
        .await
        .unwrap();
    session.channel.shutdown().await.unwrap();
    session.channel.shutdown().await.unwrap();
    assert!(session.channel.write_all(b"after shutdown").await.is_err());
    session.channel.flush().await.unwrap();
    let mut output = vec![];
    tokio::time::timeout(Duration::from_secs(10), async {
        session.channel.read_to_end(&mut output).await.unwrap();
        session.ended.cancelled().await;
    })
    .await
    .expect("session shutdown must deliver EOF to the child");
    assert_eq!(output, b"synthetic-session\neof-received");
}
