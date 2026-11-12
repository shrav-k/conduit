mod support;
use provider_ssh_runtime::{FileStore, SshBackend, codec, instance_key, parse_state, ssh};
use runtime_provider_kit::*;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    sync::{Arc, atomic::Ordering},
    time::Duration,
};
use support::*;
use tokio_util::sync::CancellationToken;

#[test]
fn remote_quoting_round_trips_through_a_real_posix_shell() {
    let args = [
        "",
        "two words",
        "both'\"quotes",
        "one\ntwo",
        "$HOME `x`",
        "100%\\",
        " leading ",
    ];
    let words: Vec<_> = args.iter().map(|s| (*s).to_owned()).collect();
    let script = format!("printf '%s\\0' {}", codec::command(&words));
    let output = std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg(script)
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        args.iter()
            .flat_map(|s| s.bytes().chain([0]))
            .collect::<Vec<_>>()
    );
    assert_eq!(support::words(&codec::command(&words)), words);
}
#[test]
fn state_parser_requires_known_load_and_active_states() {
    for (text, state) in [
        ("LoadState=loaded\nActiveState=active\n", RunState::Running),
        ("ActiveState=failed\nLoadState=loaded\n", RunState::Stopped),
        ("LoadState=not-found\n", RunState::Stopped),
        (
            "LoadState=loaded\nActiveState=activating\n",
            RunState::Unknown,
        ),
        ("LoadState=error\nActiveState=active\n", RunState::Unknown),
        ("", RunState::Unknown),
    ] {
        assert_eq!(parse_state(text.as_bytes()), state);
    }
    assert_eq!(parse_state(&[255]), RunState::Unknown);
}
#[tokio::test]
async fn stages_private_files_precedence_and_stdin_only_values_then_starts_and_stops() {
    let f = Fixture::new();
    let cancel = CancellationToken::new();
    let ctx = context();
    let staged = f.backend.stage(&ctx, &input(ID), &cancel).await.unwrap();
    let dir = f.directory(ID);
    for name in ["", "identity"] {
        assert_eq!(
            fs::metadata(dir.join(name)).unwrap().permissions().mode() & 0o777,
            0o700
        );
    }
    for name in ["identity/token", "environment"] {
        assert_eq!(
            fs::metadata(dir.join(name)).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
    assert_eq!(
        fs::read(dir.join("identity/token")).unwrap(),
        b"synthetic-identity"
    );
    let env = fs::read_to_string(dir.join("environment")).unwrap();
    for line in [
        "PATH=\"/usr/bin:/bin\"",
        "ORDINARY=\"synthetic-ordinary\"",
        "SECRET_TEST=\"synthetic-secret\"",
        "OVERRIDE=\"identity\"",
    ] {
        assert!(env.contains(line));
    }
    assert!(env.contains(names::IDENTITY_DIR_VARIABLE));
    assert!(env.contains(names::STATE_DIR_VARIABLE));
    let channel = f
        .backend
        .start(&ctx, ID, staged, &cancel, Duration::from_secs(30))
        .await
        .unwrap();
    assert!(!dir.join("environment").exists());
    assert_eq!(f.backend.state(&ctx, ID).await.unwrap(), RunState::Running);
    {
        let calls = f.fake.calls.lock().unwrap();
        for call in calls.iter() {
            assert!(!call.args.iter().any(|s| s.contains("synthetic-secret")
                || s.contains("synthetic-identity")
                || s.contains("synthetic-ordinary")));
            assert!(!call.args.iter().any(|s| s.contains("show-environment")));
        }
        assert!(
            calls
                .iter()
                .any(|c| c.input.windows(6).any(|w| w == b"umask "))
        );
        let run = calls
            .iter()
            .map(|c| support::words(c.args.last().unwrap()))
            .find(|w| w.iter().any(|s| s == "--pipe"))
            .unwrap();
        assert_eq!(
            &run[..4],
            &[
                "/usr/bin/env",
                "XDG_RUNTIME_DIR=/run/user/1001",
                "/usr/bin/systemd-run",
                "--user"
            ]
        );
        let expected = vec![
            "--pipe".into(),
            "--wait".into(),
            "--collect".into(),
            "--quiet".into(),
            "--service-type=exec".into(),
            "--expand-environment=no".into(),
            format!("--unit={}", f.unit(ID)),
            "-p".into(),
            format!("EnvironmentFile=-{}/environment", dir.display()),
            "-p".into(),
            format!("ExecStartPre=:/bin/test -f {}/environment", dir.display()),
            "-p".into(),
            format!("WorkingDirectory={}", dir.display()),
            "-p".into(),
            "RuntimeMaxSec=30.000000000s".into(),
            "-p".into(),
            "TimeoutStopSec=2s".into(),
            "-p".into(),
            format!("ExecStopPost=:/bin/rm -rf -- {}", dir.display()),
            "--".into(),
            "/bin/cat".into(),
            "arg space".into(),
            "quote'\"".into(),
            "line\n$%".into(),
        ];
        assert_eq!(&run[4..], expected);
        let log = fs::read_dir(f.state.join("stderr"))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        assert_eq!(
            fs::metadata(&log).unwrap().permissions().mode() & 0o777,
            0o600
        );
        assert_eq!(fs::read(log).unwrap(), b"synthetic run stderr\n");
    }
    f.backend.stop(&ctx, ID).await.unwrap();
    assert!(!dir.exists());
    assert!(f.backend.inventory().await.unwrap().is_empty());
    assert!(
        f.remote
            .path()
            .join(f.backend.part())
            .join("state")
            .join(instance_key(ID))
            .exists()
    );
    let calls = f.fake.calls.lock().unwrap();
    let remote: Vec<_> = calls
        .iter()
        .map(|c| support::words(c.args.last().unwrap()))
        .collect();
    let stop = remote
        .iter()
        .rposition(|w| w.iter().any(|s| s == "stop"))
        .unwrap();
    assert_eq!(remote[stop + 1][0], "/bin/rm");
    assert!(remote[stop + 2].iter().any(|s| s == "show"));
    drop(channel);
}
#[test]
fn ssh_options_are_exact_and_paths_escape_spaces_and_percent() {
    let f = Fixture::new();
    let machine = &f.config.machines[0];
    let pinned = f.durable.as_path().join("known_hosts");
    let socket = f.backend.socket_directory().join("socket");
    let call = ssh::invocation(
        std::path::Path::new("/synthetic/ssh"),
        &f.config,
        machine,
        &pinned,
        &socket,
        None,
    )
    .unwrap();
    let p = |path: &std::path::Path| codec::option_path(path.to_str().unwrap()).unwrap();
    let options = vec![
        "IdentitiesOnly=yes".into(),
        "IdentityAgent=none".into(),
        format!("IdentityFile={}", p(&f.config.private_key)),
        "User=conduit-run".into(),
        "Port=2222".into(),
        format!("UserKnownHostsFile={}", p(&pinned)),
        "GlobalKnownHostsFile=/dev/null".into(),
        "HostKeyAlias=synthetic-node".into(),
        "StrictHostKeyChecking=yes".into(),
        "UpdateHostKeys=no".into(),
        "HostKeyAlgorithms=ssh-ed25519".into(),
        "BatchMode=yes".into(),
        "ConnectTimeout=5".into(),
        "ServerAliveInterval=10".into(),
        "ServerAliveCountMax=3".into(),
        "LogLevel=ERROR".into(),
        format!("ControlPath={}", p(&socket)),
    ];
    let mut expected: Vec<String> = ["-F", "none", "-T", "-e", "none"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    for option in options {
        expected.extend(["-o".into(), option]);
    }
    expected.extend(
        [
            "-N",
            "-o",
            "ControlMaster=yes",
            "-o",
            "ControlPersist=no",
            "synthetic.invalid",
        ]
        .into_iter()
        .map(str::to_owned),
    );
    assert_eq!(call.args, expected);
    assert!(p(&f.config.private_key).contains("%%"));
    for value in [
        "~/key",
        "relative/key",
        "/bad\nkey",
        "/bad\"key",
        "/bad\\key",
    ] {
        assert!(codec::option_path(value).is_err());
    }
    assert!(f.backend.socket_directory().as_os_str().len() + 19 < 80);
}
#[tokio::test]
async fn marker_inventory_is_complete_offline_and_owner_part_are_persistent() {
    let f = Fixture::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &CancellationToken::new())
        .await
        .unwrap();
    f.fake.reachable.store(false, Ordering::SeqCst);
    assert_eq!(f.backend.inventory().await.unwrap().len(), 1);
    assert_eq!(
        f.backend.state(&context(), ID).await.unwrap(),
        RunState::Unknown
    );
    assert!(
        SshBackend::with_executor(
            f.durable.as_path(),
            Some(&f.state),
            f.config.clone(),
            None,
            "/synthetic/ssh".into(),
            f.fake.clone()
        )
        .is_err()
    );
    let part = f.backend.part().to_owned();
    let config = f.config.clone();
    let root = f.durable.as_path().to_path_buf();
    drop(staged);
    tokio::task::yield_now().await;
    // A second provider uses a different part even when serving the same remote user.
    let other = Fixture::new();
    assert_ne!(part, other.backend.part());
    let marker = fs::read_dir(root.join("markers"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    assert_eq!(
        fs::metadata(marker).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert!(config.private_key.starts_with(root));
}
#[tokio::test]
async fn bare_program_and_invalid_bindings_refuse_with_names_only() {
    let f = Fixture::new();
    let cancel = CancellationToken::new();
    let mut i = input(ID);
    i.item = serde_json::json!({"command":["cat"]});
    let staged = f.backend.stage(&context(), &i, &cancel).await.unwrap();
    f.backend.stop(&context(), ID).await.unwrap();
    drop(staged);
    i.instance = OTHER.into();
    i.item = serde_json::json!({"command":["synthetic-missing-program"]});
    assert!(f.backend.stage(&context(), &i, &cancel).await.is_err());
    assert!(f.diagnostics().contains("command pre-check"));
    let mut i = input("invalid-variable");
    i.environment
        .insert("INVALID-NAME".into(), "synthetic-sensitive-value".into());
    assert!(f.backend.stage(&context(), &i, &cancel).await.is_err());
    assert!(f.diagnostics().contains("INVALID-NAME"));
    assert!(!f.diagnostics().contains("synthetic-sensitive-value"));
    for program in ["./relative", "sub/program"] {
        let mut i = input(program);
        i.item = serde_json::json!({"command":[program]});
        assert!(f.backend.stage(&context(), &i, &cancel).await.is_err());
    }
    let mut i = input("package");
    i.item = serde_json::json!({"package":{}});
    assert!(f.backend.stage(&context(), &i, &cancel).await.is_err());
    let mut i = input("arguments");
    i.package_arguments.insert("x".into(), "private".into());
    assert!(f.backend.stage(&context(), &i, &cancel).await.is_err());
}
#[tokio::test]
async fn each_first_contact_check_refuses_and_names_the_program() {
    for (check, text, success, diagnostic) in [
        ("id", "not-a-uid\n", true, "id -u"),
        ("manager", "starting\n", true, "systemctl"),
        ("linger", "Linger=no\n", true, "loginctl"),
        ("version", "systemd 253\n", true, "systemd-run"),
    ] {
        let f = Fixture::new();
        f.fake.response(check, text, success);
        assert!(
            f.backend
                .stage(&context(), &input(ID), &CancellationToken::new())
                .await
                .is_err()
        );
        assert!(f.diagnostics().contains(diagnostic));
        assert_eq!(
            fs::metadata(f.durable.as_path().join("diagnostics.log"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
    let f = Fixture::new();
    f.fake.response("manager", "degraded\n", false);
    let s = f
        .backend
        .stage(&context(), &input(ID), &CancellationToken::new())
        .await
        .unwrap();
    drop(s);
}
#[tokio::test]
async fn start_waits_for_active_and_failed_session_is_an_error() {
    let f = Fixture::new();
    let cancel = CancellationToken::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &cancel)
        .await
        .unwrap();
    f.fake.delayed.store(true, Ordering::SeqCst);
    let b = f.backend.clone();
    let token = cancel.clone();
    let start = tokio::spawn(async move {
        b.start(&context(), ID, staged, &token, Duration::from_secs(30))
            .await
    });
    f.fake.sent.notified().await;
    tokio::time::sleep(Duration::from_millis(60)).await;
    assert!(!start.is_finished());
    f.fake
        .units
        .lock()
        .unwrap()
        .insert(f.unit(ID), "LoadState=loaded\nActiveState=active\n".into());
    let channel = start.await.unwrap().unwrap();
    f.backend.stop(&context(), ID).await.unwrap();
    drop(channel);
    let s = f
        .backend
        .stage(&context(), &input(OTHER), &cancel)
        .await
        .unwrap();
    let b = f.backend.clone();
    let start = tokio::spawn(async move {
        b.start(
            &context(),
            OTHER,
            s,
            &CancellationToken::new(),
            Duration::from_secs(30),
        )
        .await
    });
    f.fake.sent.notified().await;
    f.fake.units.lock().unwrap().insert(
        f.unit(OTHER),
        "LoadState=loaded\nActiveState=failed\n".into(),
    );
    f.fake.end_sessions();
    assert!(start.await.unwrap().is_err());
}
#[tokio::test]
async fn cancellation_before_send_and_after_send_and_late_arrival_cannot_launch() {
    let f = Fixture::new();
    let token = CancellationToken::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &token)
        .await
        .unwrap();
    token.cancel();
    assert!(
        f.backend
            .start(&context(), ID, staged, &token, Duration::from_secs(30))
            .await
            .is_err()
    );
    assert!(f.fake.sessions.lock().unwrap().is_empty());
    f.backend.stop(&context(), ID).await.unwrap();
    let token = CancellationToken::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &token)
        .await
        .unwrap();
    f.fake.delayed.store(true, Ordering::SeqCst);
    let b = f.backend.clone();
    let t = token.clone();
    let start = tokio::spawn(async move {
        b.start(&context(), ID, staged, &t, Duration::from_secs(30))
            .await
    });
    f.fake.sent.notified().await;
    token.cancel();
    assert!(start.await.unwrap().is_err());
    assert!(f.diagnostics().contains("cancelled after systemd-run send"));
    f.backend.stop(&context(), ID).await.unwrap();
    // Deliver the recorded systemd-run request after stop. Its actual required files
    // have gone; systemd's exec service cannot enter WorkingDirectory/read EnvironmentFile.
    let recorded_args = {
        let calls = f.fake.calls.lock().unwrap();
        calls
            .iter()
            .find(|c| {
                support::words(c.args.last().unwrap())
                    .iter()
                    .any(|s| s == "--pipe")
            })
            .unwrap()
            .args
            .clone()
    };
    use provider_ssh_runtime::ssh::Executor;
    use std::os::unix::fs::OpenOptionsExt;
    let stderr = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(f.durable.as_path().join("late.log"))
        .unwrap();
    let late = f
        .fake
        .session(
            ssh::Invocation {
                program: "/synthetic/ssh".into(),
                args: recorded_args.clone(),
            },
            stderr,
        )
        .await
        .unwrap();
    assert!(late.ended.is_cancelled());
    assert!(!f.fake.units.lock().unwrap().contains_key(&f.unit(ID)));
    assert!(!f.directory(ID).exists());
    // Even an existing working directory cannot bypass the start-only file
    // requirement now that EnvironmentFile itself is optional for stop-post.
    let old_directory = support::words(recorded_args.last().unwrap())
        .into_iter()
        .find_map(|w| {
            w.strip_prefix("WorkingDirectory=")
                .map(std::path::PathBuf::from)
        })
        .unwrap();
    fs::create_dir_all(old_directory).unwrap();
    let late = f
        .fake
        .session(
            ssh::Invocation {
                program: "/synthetic/ssh".into(),
                args: recorded_args,
            },
            tempfile::tempfile().unwrap(),
        )
        .await
        .unwrap();
    assert!(late.ended.is_cancelled());
    assert!(!f.fake.units.lock().unwrap().contains_key(&f.unit(ID)));
}
#[tokio::test(start_paused = true)]
async fn reconnect_sweeps_only_own_orphans_and_dropped_unstarted_runs() {
    let f = Fixture::new();
    let s = f
        .backend
        .stage(&context(), &input(ID), &CancellationToken::new())
        .await
        .unwrap();
    f.fake.reachable.store(false, Ordering::SeqCst);
    drop(s);
    tokio::task::yield_now().await;
    assert!(f.directory(ID).exists());
    let hash = instance_key("orphan");
    let orphan = f
        .remote
        .path()
        .join(f.backend.part())
        .join("runs")
        .join(&hash);
    fs::create_dir(&orphan).unwrap();
    let unit = format!("conduit-run-{}-{hash}.service", f.backend.part());
    let foreign = "conduit-run-otherpart-orphan.service";
    f.fake.units.lock().unwrap().insert(
        unit.clone(),
        "LoadState=loaded\nActiveState=active\n".into(),
    );
    f.fake
        .units
        .lock()
        .unwrap()
        .insert(foreign.into(), "active".into());
    let count = f.fake.masters.load(Ordering::SeqCst);
    assert_eq!(
        f.backend.state(&context(), ID).await.unwrap(),
        RunState::Unknown
    );
    assert_eq!(f.fake.masters.load(Ordering::SeqCst), count);
    f.fake.reachable.store(true, Ordering::SeqCst);
    tokio::time::advance(Duration::from_secs(3)).await;
    f.backend.state(&context(), ID).await.unwrap();
    assert!(!orphan.exists());
    assert!(!f.directory(ID).exists());
    assert!(!f.fake.units.lock().unwrap().contains_key(&unit));
    assert!(f.fake.units.lock().unwrap().contains_key(foreign));
    assert!(f.diagnostics().contains("sweep stopped orphan unit"));
    assert!(f.diagnostics().contains("sweep removed"));
}
#[tokio::test]
async fn dropped_staging_cleans_immediately_on_a_live_master() {
    let f = Fixture::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &CancellationToken::new())
        .await
        .unwrap();
    drop(staged);
    tokio::time::timeout(Duration::from_secs(2), async {
        while f.directory(ID).exists() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
}
#[tokio::test]
async fn session_end_sends_stop_without_waiting_for_the_kit() {
    let f = Fixture::new();
    let t = CancellationToken::new();
    let s = f.backend.stage(&context(), &input(ID), &t).await.unwrap();
    let channel = f
        .backend
        .start(&context(), ID, s, &t, Duration::from_secs(30))
        .await
        .unwrap();
    f.fake.end_sessions();
    tokio::time::timeout(Duration::from_secs(2), async {
        while f.directory(ID).exists() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    drop(channel);
}

struct NeverGate;
#[async_trait::async_trait]
impl Gate for NeverGate {
    async fn open(
        &self,
        _: Channel,
        _: &Identity,
        _: &CancellationToken,
    ) -> Result<Arc<dyn GateHandle>> {
        Err(Unavailable)
    }
}
fn call(id: &str) -> rmcp::model::CallToolRequestParams {
    let files: std::collections::BTreeMap<_, _> = schemas::identity_file_names()
        .into_iter()
        .map(|n| (n, "synthetic-file"))
        .collect();
    let identity = serde_json::json!({"environment":{names::ENGINE_URL_VARIABLE:"https://synthetic.invalid/engine", names::IDENTITY_DIR_VARIABLE:"/synthetic/identity"},"files":files});
    serde_json::from_value(serde_json::json!({"name":"run.v1","arguments":{"instance":id,"machine":context().machine,"item":{"implements":["#synthetic.v1"],"command":["/bin/cat"]},"identity":identity},"_meta":{names::key("config"): {}, names::key("revision"):"r1","io.modelcontextprotocol/clientCapabilities":{"extensions":{names::extension():{"revision":"v0"}}}}})).unwrap()
}

#[tokio::test(start_paused = true)]
async fn unreachable_stop_is_owed_blocks_another_run_and_drains_next_contact() {
    use rmcp::ServiceExt;
    let f = Fixture::new();
    let token = CancellationToken::new();
    let s = f
        .backend
        .stage(&context(), &input(ID), &token)
        .await
        .unwrap();
    // Recovery sees this marker even with the machine off, and persists its owed stop.
    f.fake.reachable.store(false, Ordering::SeqCst);
    let store = Arc::new(FileStore::new(f.durable.as_path()));
    let server = RuntimeServer::new(
        Arc::new(f.backend.clone()),
        store.clone(),
        Arc::new(NeverGate),
        f.config.options(),
    )
    .await
    .unwrap();
    let (client_io, server_io) = tokio::io::duplex(1024 * 1024);
    let serving = tokio::spawn({
        let server = server.clone();
        async move { server.serve_transport(server_io).await }
    });
    let client = ().serve(client_io).await.unwrap();
    let answer = client.call_tool(call(OTHER)).await.unwrap();
    let text = serde_json::to_string(&answer).unwrap();
    assert_eq!(answer.is_error, Some(true));
    // Kit finding: unknown state is returned before its named owed-stop admission.
    assert!(text.contains("run may still be running"));
    assert!(
        store
            .load()
            .await
            .unwrap()
            .iter()
            .any(|r| r.instance == ID && r.owed_stop)
    );
    assert!(f.fake.sessions.lock().unwrap().is_empty());
    f.fake.reachable.store(true, Ordering::SeqCst);
    tokio::time::advance(Duration::from_secs(3)).await;
    runtime_provider_kit::drain_owed_stops(&f.backend, store.as_ref())
        .await
        .unwrap();
    assert!(store.load().await.unwrap().iter().all(|r| !r.owed_stop));
    assert!(!f.directory(ID).exists());
    client.cancel().await.unwrap();
    serving.await.unwrap().unwrap();
    drop(s);
}

#[tokio::test]
async fn stop_published_during_staging_refuses_start_and_drops_remote_material() {
    use rmcp::ServiceExt;
    let f = Fixture::new();
    let store = Arc::new(FileStore::new(f.durable.as_path()));
    let server = RuntimeServer::new(
        Arc::new(f.backend.clone()),
        store.clone(),
        Arc::new(NeverGate),
        f.config.options(),
    )
    .await
    .unwrap();
    let (client_io, server_io) = tokio::io::duplex(1024 * 1024);
    let serving = tokio::spawn({
        let server = server.clone();
        async move { server.serve_transport(server_io).await }
    });
    let client = ().serve(client_io).await.unwrap();
    f.fake.stage_hold.store(true, Ordering::SeqCst);
    let peer = client.peer().clone();
    let running = tokio::spawn(async move { peer.call_tool(call(OTHER)).await.unwrap() });
    f.fake.staged.notified().await;
    f.fake.reachable.store(false, Ordering::SeqCst);
    let mut request = serde_json::to_value(call(ID)).unwrap();
    request["name"] = serde_json::json!("stop.v1");
    request["arguments"] = serde_json::json!({"instance":ID,"machine":context().machine});
    let stopped = tokio::time::timeout(
        Duration::from_secs(2),
        client.call_tool(serde_json::from_value(request).unwrap()),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(stopped.is_error, Some(true));
    f.fake.stage_release.notify_one();
    let result = running.await.unwrap();
    let text = serde_json::to_string(&result).unwrap();
    assert!(text.contains(ID));
    assert!(text.contains("synthetic-node"));
    assert!(f.fake.sessions.lock().unwrap().is_empty());
    assert!(f.directory(OTHER).exists());
    f.fake.reachable.store(true, Ordering::SeqCst);
    tokio::time::sleep(Duration::from_secs(3)).await;
    drain_owed_stops(&f.backend, store.as_ref()).await.unwrap();
    assert!(!f.directory(OTHER).exists());
    client.cancel().await.unwrap();
    serving.await.unwrap().unwrap();
}

#[tokio::test]
async fn corrupt_inventory_prevents_kit_startup_instead_of_omitting_a_run() {
    let f = Fixture::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &CancellationToken::new())
        .await
        .unwrap();
    let marker = fs::read_dir(f.durable.as_path().join("markers"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::write(marker, b"malformed").unwrap();
    assert!(f.backend.inventory().await.is_err());
    assert!(
        RuntimeServer::new(
            Arc::new(f.backend.clone()),
            Arc::new(FileStore::new(f.durable.as_path())),
            Arc::new(NeverGate),
            f.config.options()
        )
        .await
        .is_err()
    );
    assert!(f.diagnostics().contains("inventory marker JSON invalid"));
    drop(staged);
}

#[tokio::test(start_paused = true)]
async fn failed_master_backs_off_and_removes_stale_socket_before_retry() {
    let f = Fixture::new();
    let socket = f
        .backend
        .socket_directory()
        .join(&instance_key("synthetic-node")[..16]);
    fs::write(&socket, b"stale socket placeholder").unwrap();
    f.fake.reachable.store(false, Ordering::SeqCst);
    assert!(
        f.backend
            .stage(&context(), &input(ID), &CancellationToken::new())
            .await
            .is_err()
    );
    assert!(!socket.exists());
    assert_eq!(f.fake.masters.load(Ordering::SeqCst), 1);
    assert!(f.backend.inventory().await.unwrap().is_empty());
    for _ in 0..3 {
        assert_eq!(
            f.backend.state(&context(), ID).await.unwrap(),
            RunState::Unknown
        );
    }
    assert_eq!(f.fake.masters.load(Ordering::SeqCst), 1);
    tokio::time::advance(Duration::from_secs(2)).await;
    assert_eq!(
        f.backend.state(&context(), ID).await.unwrap(),
        RunState::Unknown
    );
    assert_eq!(f.fake.masters.load(Ordering::SeqCst), 2);
    f.fake.reachable.store(true, Ordering::SeqCst);
    tokio::time::advance(Duration::from_secs(4)).await;
    let s = f
        .backend
        .stage(&context(), &input(ID), &CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(f.fake.masters.load(Ordering::SeqCst), 3);
    drop(s);
}
#[tokio::test(start_paused = true)]
async fn cancellation_with_no_loaded_acknowledgement_reaps_session_at_the_bound() {
    let f = Fixture::new();
    let token = CancellationToken::new();
    let s = f
        .backend
        .stage(&context(), &input(ID), &token)
        .await
        .unwrap();
    f.fake.delayed.store(true, Ordering::SeqCst);
    f.fake.shows.lock().unwrap().extend(std::iter::repeat_n(
        "LoadState=not-found\nActiveState=inactive\n".into(),
        1000,
    ));
    let b = f.backend.clone();
    let t = token.clone();
    let starting = tokio::spawn(async move {
        b.start(&context(), ID, s, &t, Duration::from_secs(30))
            .await
    });
    f.fake.sent.notified().await;
    token.cancel();
    tokio::time::advance(Duration::from_secs(11)).await;
    assert!(starting.await.unwrap().is_err());
    assert!(
        f.fake
            .sessions
            .lock()
            .unwrap()
            .iter()
            .all(|(ended, _)| ended.is_cancelled())
    );
    assert!(!f.directory(ID).exists());
    assert!(f.diagnostics().contains("active confirmation timed out"));
    f.fake.shows.lock().unwrap().clear();
    f.backend.stop(&context(), ID).await.unwrap();
    assert!(f.backend.inventory().await.unwrap().is_empty());
    let staged = f
        .backend
        .stage(&context(), &input(ID), &CancellationToken::new())
        .await
        .unwrap();
    drop(staged);
}

#[tokio::test(start_paused = true)]
async fn cancellation_and_deadline_include_waiting_for_the_connection_mutex() {
    use tokio::io::AsyncReadExt;
    let f = Fixture::new();
    let token = CancellationToken::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &token)
        .await
        .unwrap();
    f.fake.delayed.store(true, Ordering::SeqCst);
    let backend = f.backend.clone();
    let cancel = token.clone();
    let starting = tokio::spawn(async move {
        backend
            .start(&context(), ID, staged, &cancel, Duration::from_secs(30))
            .await
    });
    f.fake.sent.notified().await;
    // An unrelated inspection holds the machine mutex indefinitely.
    *f.fake.show_hold.lock().unwrap() = Some(f.unit(OTHER));
    let backend = f.backend.clone();
    let inspecting = tokio::spawn(async move { backend.state(&context(), OTHER).await });
    f.fake.showing.notified().await;
    token.cancel();
    let (ended, mut peer) = f.fake.sessions.lock().unwrap().pop().unwrap();
    let mut byte = [0];
    assert_eq!(
        tokio::time::timeout(Duration::from_secs(1), peer.read(&mut byte))
            .await
            .unwrap()
            .unwrap(),
        0
    );
    assert!(!inspecting.is_finished());
    // Advance through confirmation and cleanup deadlines, keeping inspection blocked.
    tokio::time::advance(Duration::from_secs(10)).await;
    tokio::task::yield_now().await;
    tokio::time::advance(Duration::from_secs(5)).await;
    assert!(
        tokio::time::timeout(Duration::from_secs(1), starting)
            .await
            .unwrap()
            .unwrap()
            .is_err()
    );
    assert!(ended.is_cancelled());
    assert!(!inspecting.is_finished());
    assert!(f.diagnostics().contains("active confirmation timed out"));
    f.fake.show_release.notify_one();
    inspecting.await.unwrap().unwrap();
}

#[tokio::test]
async fn stop_post_cleans_identity_after_environment_removal_without_provider_help() {
    let f = Fixture::new();
    let cancel = CancellationToken::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &cancel)
        .await
        .unwrap();
    let channel = f
        .backend
        .start(&context(), ID, staged, &cancel, Duration::from_secs(30))
        .await
        .unwrap();
    assert!(!f.directory(ID).join("environment").exists());
    assert!(f.directory(ID).join("identity/token").exists());
    let calls_before = f.fake.calls.lock().unwrap().len();
    // The remote manager stops the service (e.g. RuntimeMaxSec), while the
    // provider sends no cleanup command and the SSH session remains open.
    assert!(f.fake.finish_unit(&f.unit(ID)));
    assert!(!f.directory(ID).exists());
    assert_eq!(f.fake.calls.lock().unwrap().len(), calls_before);
    assert!(
        f.remote
            .path()
            .join(f.backend.part())
            .join("state")
            .join(instance_key(ID))
            .is_dir()
    );
    drop(channel);
}

#[tokio::test]
async fn leading_hyphen_prefix_uses_option_terminators_for_sweep_show_and_stop() {
    let f = Fixture::with_prefix("-synthetic-");
    let cancel = CancellationToken::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &cancel)
        .await
        .unwrap();
    // Force a reconnection sweep with an orphan belonging to this provider.
    let orphan = f.unit(OTHER);
    f.fake.units.lock().unwrap().insert(
        orphan.clone(),
        "LoadState=loaded\nActiveState=active\n".into(),
    );
    f.fake.master_alive.store(false, Ordering::SeqCst);
    assert_eq!(
        f.backend.state(&context(), ID).await.unwrap(),
        RunState::Unknown
    );
    tokio::time::sleep(Duration::from_secs(2)).await;
    f.backend.state(&context(), ID).await.unwrap();
    assert!(!f.fake.units.lock().unwrap().contains_key(&orphan));
    // Reconnection sweeps unstarted material too; stage it again.
    drop(staged);
    f.backend.stop(&context(), ID).await.unwrap();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &cancel)
        .await
        .unwrap();
    let channel = f
        .backend
        .start(&context(), ID, staged, &cancel, Duration::from_secs(30))
        .await
        .unwrap();
    assert_eq!(
        f.backend.state(&context(), ID).await.unwrap(),
        RunState::Running
    );
    f.backend.stop(&context(), ID).await.unwrap();
    let calls = f.fake.calls.lock().unwrap();
    for operation in ["list-units", "show", "stop"] {
        assert!(
            calls
                .iter()
                .map(|c| support::words(c.args.last().unwrap()))
                .any(|words| {
                    words.iter().any(|word| word == operation)
                        && words
                            .windows(2)
                            .any(|pair| pair[0] == "--" && pair[1].starts_with("-synthetic-"))
                })
        );
    }
    drop(channel);
}

#[tokio::test]
async fn timed_out_launch_stop_clears_owed_and_old_generation_stays_fenced() {
    use provider_ssh_runtime::ssh::Executor;
    use rmcp::ServiceExt;
    let f = Fixture::new();
    let store = Arc::new(FileStore::new(f.durable.as_path()));
    let server = RuntimeServer::new(
        Arc::new(f.backend.clone()),
        store.clone(),
        Arc::new(NeverGate),
        f.config.options(),
    )
    .await
    .unwrap();
    let (client_io, server_io) = tokio::io::duplex(1024 * 1024);
    let serving = tokio::spawn(async move { server.serve_transport(server_io).await });
    let client = ().serve(client_io).await.unwrap();
    f.fake.queued.store(true, Ordering::SeqCst);
    let peer = client.peer().clone();
    let running = tokio::spawn(async move { peer.call_tool(call(ID)).await.unwrap() });
    f.fake.sent.notified().await;
    tokio::time::pause();
    let old_directory = f.directory(ID);
    let args = f
        .fake
        .calls
        .lock()
        .unwrap()
        .iter()
        .rev()
        .find(|c| {
            support::words(c.args.last().unwrap())
                .iter()
                .any(|w| w == "--pipe")
        })
        .unwrap()
        .args
        .clone();
    // Lose contact before confirmation and the kit's stop, leaving a durable debt.
    f.fake.reachable.store(false, Ordering::SeqCst);
    tokio::time::advance(Duration::from_secs(11)).await;
    let answer = running.await.unwrap();
    let text = serde_json::to_string(&answer).unwrap();
    assert_eq!(answer.is_error, Some(true));
    assert!(text.contains("start did not complete: systemd-run active confirmation timed out"));
    assert!(
        store
            .load()
            .await
            .unwrap()
            .iter()
            .any(|r| r.instance == ID && r.owed_stop)
    );
    assert_eq!(f.fake.programs.load(Ordering::SeqCst), 0);
    f.fake.reachable.store(true, Ordering::SeqCst);
    tokio::time::advance(Duration::from_secs(40)).await;
    tokio::time::resume();
    drain_owed_stops(&f.backend, store.as_ref()).await.unwrap();
    assert!(store.load().await.unwrap().iter().all(|r| !r.owed_stop));
    assert!(f.backend.inventory().await.unwrap().is_empty());
    assert!(!f.parent(ID).exists());
    f.fake.queued.store(false, Ordering::SeqCst);
    let late = f
        .fake
        .session(
            ssh::Invocation {
                program: "/synthetic/ssh".into(),
                args: args.clone(),
            },
            tempfile::tempfile().unwrap(),
        )
        .await
        .unwrap();
    assert!(late.ended.is_cancelled());
    assert_eq!(f.fake.programs.load(Ordering::SeqCst), 0);
    let cancel = CancellationToken::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &cancel)
        .await
        .unwrap();
    let new_directory = f.directory(ID);
    assert_ne!(old_directory, new_directory);
    let generation = new_directory.file_name().unwrap().to_str().unwrap();
    assert_eq!(generation.len(), 16);
    assert!(
        generation
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    );
    assert!(!old_directory.exists());
    assert!(new_directory.join("environment").is_file());
    let late = f
        .fake
        .session(
            ssh::Invocation {
                program: "/synthetic/ssh".into(),
                args,
            },
            tempfile::tempfile().unwrap(),
        )
        .await
        .unwrap();
    assert!(late.ended.is_cancelled());
    assert_eq!(f.fake.programs.load(Ordering::SeqCst), 0);
    assert!(new_directory.join("environment").is_file());
    // An old completion must not remove a newer marker/generation.
    tokio::task::yield_now().await;
    let channel = f
        .backend
        .start(&context(), ID, staged, &cancel, Duration::from_secs(30))
        .await
        .unwrap();
    assert_eq!(f.fake.programs.load(Ordering::SeqCst), 1);
    f.backend.stop(&context(), ID).await.unwrap();
    drop(channel);
    client.cancel().await.unwrap();
    serving.await.unwrap().unwrap();
}

#[tokio::test(start_paused = true)]
async fn sweep_keeps_started_generation_only_and_stage_removes_all_old_names() {
    let f = Fixture::new();
    let cancel = CancellationToken::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &cancel)
        .await
        .unwrap();
    let current = f.directory(ID);
    let channel = f
        .backend
        .start(&context(), ID, staged, &cancel, Duration::from_secs(30))
        .await
        .unwrap();
    let stale = f.parent(ID).join("0123456789abcdef");
    fs::create_dir(&stale).unwrap();
    fs::write(stale.join("secret"), "synthetic-stale").unwrap();
    fs::write(f.parent(ID).join("not-a-generation"), "synthetic").unwrap();
    std::os::unix::fs::symlink(&current, f.parent(ID).join("symlink")).unwrap();
    f.fake.master_alive.store(false, Ordering::SeqCst);
    f.backend.state(&context(), ID).await.unwrap();
    tokio::time::advance(Duration::from_secs(3)).await;
    f.fake.master_alive.store(true, Ordering::SeqCst);
    assert_eq!(
        f.backend.state(&context(), ID).await.unwrap(),
        RunState::Running
    );
    assert!(current.join("identity/token").is_file());
    let remaining: Vec<_> = fs::read_dir(f.parent(ID)).unwrap().collect();
    assert_eq!(remaining.len(), 1);
    assert!(!stale.exists());
    f.backend.stop(&context(), ID).await.unwrap();
    drop(channel);
    fs::create_dir_all(&stale).unwrap();
    fs::write(f.parent(ID).join("junk"), "synthetic").unwrap();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &cancel)
        .await
        .unwrap();
    assert_ne!(current, f.directory(ID));
    assert!(!stale.exists());
    assert!(!f.parent(ID).join("junk").exists());
    assert_eq!(fs::read_dir(f.parent(ID)).unwrap().count(), 1);
    drop(staged);
}

// The kit normally probes state before stage. Contact failures at that probe
// suppress stage entirely, so isolate stage refusals with a known-stopped state.
struct StageProbe(SshBackend);
#[async_trait::async_trait]
impl RuntimeBackend for StageProbe {
    type Staged = provider_ssh_runtime::Staged;
    fn declaration(&self) -> Declaration {
        self.0.declaration()
    }
    async fn inventory(&self) -> Result<Vec<RunRecord>> {
        self.0.inventory().await
    }
    async fn stage(
        &self,
        context: &CallContext,
        input: &RunInput,
        cancel: &CancellationToken,
    ) -> Result<Self::Staged> {
        self.0.stage(context, input, cancel).await
    }
    async fn start(
        &self,
        context: &CallContext,
        instance: &str,
        staged: Self::Staged,
        cancel: &CancellationToken,
        lifetime: Duration,
    ) -> Result<Channel> {
        self.0
            .start(context, instance, staged, cancel, lifetime)
            .await
    }
    async fn stop(&self, context: &CallContext, instance: &str) -> Result<()> {
        self.0.stop(context, instance).await
    }
    async fn state(&self, _: &CallContext, _: &str) -> Result<RunState> {
        Ok(RunState::Stopped)
    }
}

#[tokio::test]
async fn run_answers_carry_backend_reasons_without_values() {
    use rmcp::ServiceExt;
    // These calls traverse the real kit; it must preserve reasons from contact and sweep.
    for (failure, response, success, expected) in [
        ("id", "invalid\n", true, "id -u invalid uid"),
        ("id", "", false, "id -u failed"),
        (
            "manager",
            "starting\n",
            true,
            "systemctl --user is-system-running failed",
        ),
        (
            "linger",
            "Linger=no\n",
            true,
            "loginctl show-user Linger not enabled",
        ),
        ("linger", "", false, "loginctl show-user Linger failed"),
        (
            "version",
            "systemd 253\n",
            true,
            "systemd-run --version needs systemd 254 or later",
        ),
        ("version", "", false, "systemd-run --version failed"),
        ("sweep", "", false, "systemctl sweep list-units failed"),
        (
            "sweep-directories",
            "",
            false,
            "sweep run-directory inventory failed",
        ),
        ("connection", "", false, "ssh master connection failed"),
        (
            "name",
            "",
            false,
            "environment variable name INVALID-NAME invalid",
        ),
        (
            "value",
            "",
            false,
            "environment variable SYNTHETIC_BAD cannot be encoded",
        ),
        (
            "program",
            "",
            false,
            "command pre-check or staging failed for program synthetic-missing-program",
        ),
        ("start", "", false, "ssh systemd-run session failed"),
    ] {
        let f = Fixture::new();
        let server = RuntimeServer::new(
            Arc::new(StageProbe(f.backend.clone())),
            Arc::new(FileStore::new(f.durable.as_path())),
            Arc::new(NeverGate),
            f.config.options(),
        )
        .await
        .unwrap();
        let (client_io, server_io) = tokio::io::duplex(1024 * 1024);
        let serving = tokio::spawn(async move { server.serve_transport(server_io).await });
        let client = ().serve(client_io).await.unwrap();
        let mut request = serde_json::to_value(call(ID)).unwrap();
        match failure {
            "connection" => f.fake.reachable.store(false, Ordering::SeqCst),
            "name" => {
                request["arguments"]["environment"] =
                    serde_json::json!({"INVALID-NAME":"synthetic-sensitive"})
            }
            "value" => {
                request["arguments"]["environment"] =
                    serde_json::json!({"SYNTHETIC_BAD":"synthetic-sensitive\u{0000}"})
            }
            "program" => {
                request["arguments"]["item"]["command"] =
                    serde_json::json!(["synthetic-missing-program"])
            }
            "start" => f.fake.session_failure.store(true, Ordering::SeqCst),
            _ => f.fake.response(failure, response, success),
        }
        let answer = client
            .call_tool(serde_json::from_value(request).unwrap())
            .await
            .unwrap();
        assert_eq!(answer.is_error, Some(true));
        let text = serde_json::to_string(&answer).unwrap();
        let prefix = if failure == "start" {
            "start did not complete: "
        } else {
            "staging failed: "
        };
        assert!(
            text.contains(&format!("{prefix}{expected}")),
            "missing reason for {failure}"
        );
        assert!(
            f.diagnostics().contains(expected),
            "missing diagnostic for {failure}"
        );
        assert!(!text.contains("synthetic-sensitive"));
        assert!(!f.diagnostics().contains("synthetic-sensitive"));
        // Restore contact for the kit's shutdown stop of its remembered candidate.
        f.fake.checks.lock().unwrap().clear();
        f.fake.reachable.store(true, Ordering::SeqCst);
        tokio::time::pause();
        tokio::time::advance(Duration::from_secs(40)).await;
        tokio::time::resume();
        client.cancel().await.unwrap();
        serving.await.unwrap().unwrap();
    }
}

#[tokio::test]
async fn per_machine_limit_counts_staging_and_running_before_a_session_opens() {
    use rmcp::ServiceExt;
    let f = Fixture::with_limit(1);
    let server = RuntimeServer::new(
        Arc::new(f.backend.clone()),
        Arc::new(FileStore::new(f.durable.as_path())),
        Arc::new(NeverGate),
        f.config.options(),
    )
    .await
    .unwrap();
    let (client_io, server_io) = tokio::io::duplex(1024 * 1024);
    let serving = tokio::spawn(async move { server.serve_transport(server_io).await });
    let client = ().serve(client_io).await.unwrap();
    let cancel = CancellationToken::new();
    let staged = f
        .backend
        .stage(&context(), &input(ID), &cancel)
        .await
        .unwrap();
    let before = f.fake.calls.lock().unwrap().len();
    let answer = client.call_tool(call(OTHER)).await.unwrap();
    assert_eq!(answer.is_error, Some(true));
    assert!(
        serde_json::to_string(&answer)
            .unwrap()
            .contains("staging failed: machine synthetic-node runLimit 1 reached")
    );
    // The kit's state probe may run, but the refused stage must send no staging transfer.
    {
        let calls = f.fake.calls.lock().unwrap();
        assert!(
            calls[before..]
                .iter()
                .all(|c| !c.input.windows(9).any(|w| w == b"umask 077"))
        );
    }
    let channel = f
        .backend
        .start(&context(), ID, staged, &cancel, Duration::from_secs(30))
        .await
        .unwrap();
    let before = f.fake.calls.lock().unwrap().len();
    assert!(
        f.backend
            .stage(&context(), &input(OTHER), &cancel)
            .await
            .is_err()
    );
    assert_eq!(f.fake.calls.lock().unwrap().len(), before);
    f.backend.stop(&context(), ID).await.unwrap();
    drop(channel);
    let staged = f
        .backend
        .stage(&context(), &input(OTHER), &cancel)
        .await
        .unwrap();
    drop(staged);
    client.cancel().await.unwrap();
    serving.await.unwrap().unwrap();
}
