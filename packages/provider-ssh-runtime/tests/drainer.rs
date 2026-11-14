#![cfg(unix)]
#[path = "support/settle.rs"]
mod settle;
#[allow(dead_code)]
mod support;
use provider_ssh_runtime::{Config, FileStore, SshBackend, drainer, durable::OwnerLock, startup};
use runtime_provider_kit::{RunRecord, RunState, StateStore};
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    sync::{Arc, Mutex, atomic::Ordering},
    time::Duration,
};
use support::*;
use tokio::{
    task::JoinHandle,
    time::{Instant, advance},
};

type Log = Arc<Mutex<Vec<(Instant, &'static str)>>>;
struct Retired {
    _root: tempfile::TempDir,
    _remote: tempfile::TempDir,
    root: PathBuf,
    state: PathBuf,
    config: Config,
    fake: Arc<Fake>,
}
impl Retired {
    fn new() -> Self {
        let home = tempfile::tempdir().unwrap();
        let remote = tempfile::tempdir().unwrap();
        let state = home.path().join("instances/instance");
        fs::create_dir_all(&state).unwrap();
        let durable = home.path().join("durable");
        fs::create_dir(&durable).unwrap();
        fs::set_permissions(&durable, fs::Permissions::from_mode(0o700)).unwrap();
        let root = provider_ssh_runtime::durable::instance_directory(&durable, &state);
        fs::create_dir(&root).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
        let config = support::config(&root, remote.path());
        let fake = Arc::new(Fake::default());
        // This fixture has no remote run directories. Keep the drainer's paused clock
        // independent of the OS scheduler used by the shell-script fidelity tests.
        fake.checks.lock().unwrap().insert(
            "sweep-directories".into(),
            provider_ssh_runtime::ssh::Output {
                success: true,
                stdout: vec![],
            },
        );
        drop(
            SshBackend::with_executor(
                &root,
                Some(&state),
                config.clone(),
                None,
                "/synthetic/ssh".into(),
                fake.clone(),
            )
            .unwrap(),
        );
        released(&root);
        Self {
            _root: home,
            _remote: remote,
            root,
            state,
            config,
            fake,
        }
    }
    async fn owe(&self) {
        FileStore::new(&self.root)
            .save(&RunRecord {
                instance: ID.into(),
                context: context(),
                state: RunState::Unknown,
                owed_stop: true,
            })
            .await
            .unwrap();
    }
    async fn prepare(&self) -> OwnerLock {
        let owner = startup::prepare(&self.config, &self.state, &self.root, || Ok(()))
            .await
            .unwrap();
        startup::configured_backend(&self.root, &self.config, || {
            SshBackend::with_executor(
                &self.root,
                Some(&self.state),
                self.config.clone(),
                Some(owner.clone()),
                "/synthetic/ssh".into(),
                self.fake.clone(),
            )
        })
        .unwrap();
        owner
    }
    async fn spawn(&self) -> (JoinHandle<()>, Log) {
        let owner = self.prepare().await;
        let (worker, log) = self.worker();
        let handle = tokio::spawn(worker);
        drop(owner);
        released(&self.root);
        (handle, log)
    }
    fn worker(&self) -> (impl Future<Output = ()> + Send + 'static, Log) {
        let (root, state, fake) = (self.root.clone(), self.state.clone(), self.fake.clone());
        let log = Log::default();
        let events = log.clone();
        (
            async move {
                drainer::run(
                    &root,
                    &state,
                    |config, owner| {
                        SshBackend::with_executor(
                            &root,
                            None,
                            config.clone(),
                            Some(owner),
                            "/synthetic/ssh".into(),
                            fake.clone(),
                        )
                    },
                    |message| events.lock().unwrap().push((Instant::now(), message)),
                )
                .await;
            },
            log,
        )
    }
    async fn owed(&self) -> bool {
        FileStore::new(&self.root)
            .load()
            .await
            .unwrap()
            .iter()
            .any(|r| r.owed_stop)
    }
}
fn contains(log: &Log, message: &str) -> bool {
    log.lock().unwrap().iter().any(|(_, m)| m.contains(message))
}
async fn tick(seconds: u64) {
    for _ in 0..seconds {
        advance(Duration::from_secs(1)).await;
        tokio::task::yield_now().await;
    }
}

async fn poll_once<F: Future>(mut future: std::pin::Pin<&mut F>) -> std::task::Poll<F::Output> {
    std::future::poll_fn(|cx| std::task::Poll::Ready(future.as_mut().poll(cx))).await
}

fn publish(
    f: &Retired,
    config: &Config,
    owner: OwnerLock,
) -> runtime_provider_kit::Result<SshBackend> {
    startup::configured_backend(&f.root, config, || {
        SshBackend::with_executor(
            &f.root,
            Some(&f.state),
            config.clone(),
            Some(owner),
            "/synthetic/ssh".into(),
            f.fake.clone(),
        )
    })
}

fn drainer_held(f: &Retired) -> bool {
    !startup::poll_drainer(&f.root, || Ok(())).unwrap()
}
/// True once drainer.lock can be taken; waits out a forked child's copy (support/settle.rs).
fn drainer_free(f: &Retired) -> bool {
    settle::settled(|| if drainer_held(f) { Err(()) } else { Ok(()) }).is_ok()
}
/// Returns once owner.lock, which the test has just released, can be taken again. Under a
/// paused clock the drainer's retries take no real time, so it must not start before then.
fn released(root: &std::path::Path) {
    drop(settle::settled(|| OwnerLock::acquire(root)).unwrap());
}

#[tokio::test(start_paused = true)]
async fn waiting_drainer_survives_provider_restart_and_reads_new_machine_configuration() {
    let f = Retired::new();
    let owner = f.prepare().await;
    let (worker, _) = f.worker();
    let mut worker = std::pin::pin!(worker);
    assert!(poll_once(worker.as_mut()).await.is_pending());
    assert!(drainer_held(&f));
    drop(owner);
    let mut starts = 0;
    let owner = startup::prepare(&f.config, &f.state, &f.root, || {
        starts += 1;
        Ok(())
    })
    .await
    .unwrap();
    assert_eq!(starts, 0);
    let mut newest = f.config.clone();
    let mut machine = newest.machines[0].clone();
    machine.hostname = "new-node".into();
    machine.host = "new.invalid".into();
    newest.machines.push(machine);
    drop(publish(&f, &newest, owner.clone()).unwrap());
    let mut context = context();
    context.machine.hostname = "new-node".into();
    FileStore::new(&f.root)
        .save(&RunRecord {
            instance: ID.into(),
            context,
            state: RunState::Unknown,
            owed_stop: true,
        })
        .await
        .unwrap();
    drop(owner);
    released(&f.root);
    advance(Duration::from_secs(1)).await;
    assert!(poll_once(worker.as_mut()).await.is_pending());
    assert!(!f.owed().await);
    assert!(
        f.fake
            .calls
            .lock()
            .unwrap()
            .iter()
            .any(|c| c.args.iter().any(|a| a.contains("new.invalid")))
    );
    tick(60).await;
    assert!(poll_once(worker.as_mut()).await.is_ready());
}

#[tokio::test(start_paused = true)]
async fn rejected_backend_keeps_previous_config_and_write_failure_refuses_after_launch() {
    let f = Retired::new();
    drop(f.prepare().await);
    let previous = fs::read(f.root.join("drainer-config")).unwrap();
    fs::set_permissions(&f.config.private_key, fs::Permissions::from_mode(0o644)).unwrap();
    let mut starts = 0;
    let owner = startup::prepare(&f.config, &f.state, &f.root, || {
        starts += 1;
        Ok(())
    })
    .await
    .unwrap();
    let mut changed = f.config.clone();
    changed.prefix = "changed-".into();
    assert!(publish(&f, &changed, owner).is_err());
    assert!(previous == fs::read(f.root.join("drainer-config")).unwrap());
    assert_eq!(starts, 1);
    fs::set_permissions(&f.config.private_key, fs::Permissions::from_mode(0o600)).unwrap();
    fs::remove_file(f.root.join("drainer-config")).unwrap();
    fs::create_dir(f.root.join("drainer-config")).unwrap();
    let owner = startup::prepare(&f.config, &f.state, &f.root, || {
        starts += 1;
        Ok(())
    })
    .await
    .unwrap();
    let error = publish(&f, &changed, owner)
        .err()
        .expect("configuration write must refuse");
    assert!(
        error
            == runtime_provider_kit::Unavailable::because(format!(
                "cannot write {}",
                f.root.join("drainer-config").display()
            ))
    );
    assert_eq!(starts, 2);
    assert!(settle::settled(|| OwnerLock::acquire(&f.root)).is_ok());
    assert_eq!(f.fake.live_masters.load(Ordering::SeqCst), 0);
}

#[tokio::test(start_paused = true)]
async fn config_contains_only_schema_places_and_no_secret_values_from_elsewhere() {
    let f = Retired::new();
    let marker = "test-only-sensitive-marker-never-in-drainer-config";
    fs::write(&f.config.private_key, marker).unwrap();
    fs::write(f.state.join("identity-input"), marker).unwrap();
    fs::write(f.root.join("diagnostics.log"), marker).unwrap();
    drop(f.prepare().await);
    let bytes = fs::read(f.root.join("drainer-config")).unwrap();
    assert!(!bytes.windows(marker.len()).any(|b| b == marker.as_bytes()));
    assert_eq!(
        fs::metadata(f.root.join("drainer-config"))
            .unwrap()
            .permissions()
            .mode()
            & 0o7777,
        0o600
    );
    let config: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let contract: serde_json::Value =
        serde_yaml::from_str(include_str!("../contracts.yaml")).unwrap();
    let validator = jsonschema::validator_for(&contract["spec"]["schema"]).unwrap();
    assert!(validator.is_valid(&config));
    assert!(config["privateKey"].as_str().is_some());
    let restored = Config::read_drainer(&f.root).unwrap();
    assert_eq!(restored.private_key, f.config.private_key);
    assert_eq!(restored.drain_interval, f.config.drain_interval);
    assert_eq!(restored.machines.len(), f.config.machines.len());
}

#[tokio::test(start_paused = true)]
async fn missing_or_invalid_config_is_named_and_exit_releases_drainer_before_owner() {
    for case in ["missing", "syntax", "unknown", "type", "value", "valid"] {
        let f = Retired::new();
        let owner = f.prepare().await;
        match case {
            "missing" => fs::remove_file(f.root.join("drainer-config")).unwrap(),
            "syntax" => fs::write(f.root.join("drainer-config"), b"invalid").unwrap(),
            "unknown" | "type" | "value" => {
                let mut config: serde_json::Value =
                    serde_json::from_slice(&fs::read(f.root.join("drainer-config")).unwrap())
                        .unwrap();
                match case {
                    "unknown" => {
                        config["extra"] = true.into();
                    }
                    "type" => {
                        config["runLimit"] = "62".into();
                    }
                    _ => {
                        config["runLimit"] = 0.into();
                    }
                }
                fs::write(
                    f.root.join("drainer-config"),
                    serde_json::to_vec(&config).unwrap(),
                )
                .unwrap();
            }
            _ => (),
        }
        drop(owner);
        let mut released = false;
        let mut named = false;
        let worker = drainer::run(
            &f.root,
            &f.state,
            |config, owner| {
                SshBackend::with_executor(
                    &f.root,
                    None,
                    config.clone(),
                    Some(owner),
                    "/synthetic/ssh".into(),
                    f.fake.clone(),
                )
            },
            |message| {
                if message == "drainer.lock released before owner.lock" {
                    assert!(drainer_free(&f));
                    assert!(OwnerLock::try_existing(&f.root).unwrap().is_none());
                    assert_eq!(f.fake.live_masters.load(Ordering::SeqCst), 0);
                    released = true;
                }
                named |= message.contains("drainer-config missing or invalid");
            },
        );
        worker.await;
        assert!(released);
        assert_eq!(named, case != "valid");
        assert!(settle::settled(|| OwnerLock::acquire(&f.root)).is_ok());
    }
}

#[tokio::test(start_paused = true)]
async fn duplicate_drainer_exits_on_its_first_poll_while_waiter_keeps_the_lock() {
    let f = Retired::new();
    let owner = f.prepare().await;
    let (worker, _) = f.worker();
    let mut worker = std::pin::pin!(worker);
    assert!(poll_once(worker.as_mut()).await.is_pending());
    let (duplicate, log) = f.worker();
    let mut duplicate = std::pin::pin!(duplicate);
    assert!(poll_once(duplicate.as_mut()).await.is_ready());
    assert!(contains(&log, "drainer.lock held; exiting"));
    assert!(drainer_held(&f));
    drop(owner);
}

#[tokio::test(start_paused = true)]
async fn keeper_replaces_aborted_waiter_or_child_that_died_before_lock_within_one_poll() {
    for before_lock in [false, true] {
        let f = Retired::new();
        let mut children = Vec::new();
        let mut start = || {
            let (worker, _) = f.worker();
            children.push(tokio::spawn(worker));
            Ok(())
        };
        let owner = startup::prepare(&f.config, &f.state, &f.root, &mut start)
            .await
            .unwrap();
        drop(publish(&f, &f.config, owner.clone()).unwrap());
        if !before_lock {
            tokio::task::yield_now().await;
            assert!(drainer_held(&f));
        }
        children[0].abort();
        assert!(children.remove(0).await.unwrap_err().is_cancelled());
        let keeper = startup::keep_alive(&f.root, || {
            let (worker, _) = f.worker();
            children.push(tokio::spawn(worker));
            Ok(())
        });
        {
            let mut keeper = std::pin::pin!(keeper);
            assert!(poll_once(keeper.as_mut()).await.is_pending());
            advance(Duration::from_secs(1)).await;
            assert!(poll_once(keeper.as_mut()).await.is_pending());
            tokio::task::yield_now().await;
            assert!(drainer_held(&f));
            advance(Duration::from_secs(1)).await;
            assert!(poll_once(keeper.as_mut()).await.is_pending());
        }
        assert_eq!(children.len(), 1);
        children[0].abort();
        assert!(children.remove(0).await.unwrap_err().is_cancelled());
        drop(owner);
    }
}

#[tokio::test(start_paused = true)]
async fn keeper_logs_failed_start_and_retries_at_each_poll() {
    let f = Retired::new();
    let owner = f.prepare().await;
    let mut starts = 0;
    {
        let keeper = startup::keep_alive(&f.root, || {
            starts += 1;
            Err(std::io::Error::other("synthetic start failure"))
        });
        let mut keeper = std::pin::pin!(keeper);
        assert!(poll_once(keeper.as_mut()).await.is_pending());
        for _ in 0..2 {
            advance(Duration::from_secs(1)).await;
            assert!(poll_once(keeper.as_mut()).await.is_pending());
        }
    }
    assert_eq!(starts, 2);
    let log = fs::read_to_string(f.root.join("diagnostics.log")).unwrap();
    assert_eq!(
        log.lines()
            .filter(|line| line.contains(startup::DRAINER))
            .count(),
        2
    );
    drop(owner);
}

#[tokio::test(start_paused = true)]
async fn waiter_retries_owner_lock_errors_and_exits_only_when_ledger_disappears() {
    let f = Retired::new();
    let owner = f.prepare().await;
    let (worker, log) = f.worker();
    let worker = tokio::spawn(worker);
    tokio::task::yield_now().await;
    fs::remove_file(f.root.join("owner.lock")).unwrap();
    fs::create_dir(f.root.join("owner.lock")).unwrap();
    tick(2).await;
    assert!(!worker.is_finished());
    assert!(drainer_held(&f));
    assert!(contains(&log, "cannot open owner.lock; retrying"));
    fs::remove_file(f.root.join("ledger.json")).unwrap();
    tick(1).await;
    worker.await.unwrap();
    assert!(contains(&log, "ledger.json gone"));
    assert!(drainer_free(&f));
    drop(owner);
}

#[tokio::test(start_paused = true)]
async fn killed_waiting_draining_or_yielding_is_replaced_once_at_next_start_and_still_drains() {
    for phase in ["waiting", "draining", "yielding"] {
        let f = Retired::new();
        f.owe().await;
        f.fake.stop_hold.store(true, Ordering::SeqCst);
        let mut owner = Some(f.prepare().await);
        let (worker, _) = f.worker();
        let worker = tokio::spawn(worker);
        tokio::task::yield_now().await;
        if phase != "waiting" {
            owner = None;
            f.fake.stopping.notified().await;
            assert_eq!(f.fake.stops_in_flight.load(Ordering::SeqCst), 1);
            if phase == "yielding" {
                owner = Some(OwnerLock::acquire_provider(&f.root).await.unwrap());
                assert_eq!(f.fake.stops_in_flight.load(Ordering::SeqCst), 0);
            }
        }
        assert!(drainer_held(&f));
        worker.abort();
        assert!(worker.await.unwrap_err().is_cancelled());
        assert_eq!(f.fake.stops_in_flight.load(Ordering::SeqCst), 0);
        assert_eq!(f.fake.live_masters.load(Ordering::SeqCst), 0);
        assert!(f.owed().await);
        assert!(drainer_free(&f));
        drop(owner);
        let mut starts = 0;
        let mut replacement = None;
        let owner = startup::prepare(&f.config, &f.state, &f.root, || {
            starts += 1;
            let (worker, _) = f.worker();
            replacement = Some(tokio::spawn(worker));
            Ok(())
        })
        .await
        .unwrap();
        drop(publish(&f, &f.config, owner.clone()).unwrap());
        tokio::task::yield_now().await;
        assert!(
            !startup::poll_drainer(&f.root, || {
                starts += 1;
                Ok(())
            })
            .unwrap()
        );
        assert_eq!(starts, 1);
        f.fake.stop_hold.store(false, Ordering::SeqCst);
        drop(owner);
        released(&f.root);
        tick(2).await;
        assert!(!f.owed().await, "{phase}");
        tick(60).await;
        replacement.unwrap().await.unwrap();
    }
}

#[tokio::test(start_paused = true)]
async fn starts_after_binding_before_backend_refusal_and_launch_failure_refuses() {
    let f = Retired::new();
    fs::remove_file(f.root.join("part")).unwrap();
    fs::remove_file(f.root.join("known_hosts")).unwrap();
    fs::set_permissions(&f.config.private_key, fs::Permissions::from_mode(0o644)).unwrap();
    fs::remove_file(f.root.join("state-dir")).unwrap();
    fs::remove_file(f.root.join("ledger.json")).unwrap();
    let mut called = false;
    let owner = startup::prepare(&f.config, &f.state, &f.root, || {
        assert_eq!(
            fs::read(f.root.join("state-dir")).unwrap(),
            f.state.as_os_str().as_encoded_bytes()
        );
        assert!(OwnerLock::try_existing(&f.root).unwrap().is_none());
        assert!(!f.root.join("part").exists());
        assert!(!f.root.join("known_hosts").exists());
        assert!(!f.root.join("drainer-config").exists());
        let mut looked_up = Vec::new();
        let launch = startup::Launch::new(
            &f.root.join("bin/provider-ssh-runtime"),
            &f.root,
            &f.state,
            |name| {
                looked_up.push(name.to_owned());
                Some("synthetic".into())
            },
        );
        assert_eq!(looked_up, ["PATH"]);
        assert_eq!(
            launch.program,
            f.root.join("bin/provider-ssh-runtime-drain")
        );
        assert_eq!(
            launch.arguments,
            [
                f.root.as_os_str().to_owned(),
                f.state.as_os_str().to_owned()
            ]
        );
        assert_eq!(launch.log, f.root.join("drainer.log"));
        assert!(launch.environment.iter().all(|(n, _)| n == "PATH"));
        called = true;
        Ok(())
    })
    .await
    .unwrap();
    assert!(called);
    assert!(
        startup::configured_backend(&f.root, &f.config, || {
            SshBackend::with_executor(
                &f.root,
                Some(&f.state),
                f.config.clone(),
                Some(owner),
                "/synthetic/ssh".into(),
                f.fake.clone(),
            )
        })
        .is_err()
    );
    assert!(!f.root.join("drainer-config").exists());
    assert!(
        startup::prepare(&f.config, &f.state, &f.root, || Err(std::io::Error::other(
            "synthetic launch failure"
        )))
        .await
        .is_err()
    );
    let launch = startup::Launch::new(&f.root.join("missing/provider"), &f.root, &f.state, |_| {
        None
    });
    assert!(launch.spawn().is_err());
}

#[tokio::test(start_paused = true)]
async fn retries_with_doubling_waits_then_stops_once_remote_answers_without_starting_runs() {
    let f = Retired::new();
    f.owe().await;
    f.fake.reachable.store(false, Ordering::SeqCst);
    let start = Instant::now();
    let (worker, log) = f.spawn().await;
    tokio::task::yield_now().await;
    tick(15).await;
    let attempts: Vec<_> = log
        .lock()
        .unwrap()
        .iter()
        .filter(|(_, m)| *m == "drain attempt")
        .map(|(t, _)| t.duration_since(start).as_secs())
        .collect();
    assert_eq!(attempts, [0, 1, 3, 7, 15]);
    assert!(f.owed().await);
    f.fake.reachable.store(true, Ordering::SeqCst);
    // SSH's independent connection backoff has also expired by this next attempt.
    tick(17).await;
    assert!(!f.owed().await);
    tick(60).await;
    worker.await.unwrap();
    let calls = f.fake.calls.lock().unwrap();
    let words: Vec<_> = calls
        .iter()
        .map(|c| support::words(c.args.last().unwrap()))
        .collect();
    assert_eq!(
        words
            .iter()
            .filter(|w| w.iter().any(|s| s == "stop"))
            .count(),
        1
    );
    assert!(!words.iter().any(|w| w.iter().any(|s| s == "--pipe")));
    assert_eq!(f.fake.live_masters.load(Ordering::SeqCst), 0);
}

#[tokio::test(start_paused = true)]
async fn bound_makes_final_attempt_and_unreadable_ledger_is_failure() {
    for corrupt in [false, true] {
        let mut f = Retired::new();
        f.config.max_lifetime = Duration::from_secs(4);
        f.owe().await;
        if corrupt {
            fs::write(f.root.join("ledger.json"), b"invalid").unwrap();
        }
        f.fake.reachable.store(false, Ordering::SeqCst);
        let start = Instant::now();
        let (worker, log) = f.spawn().await;
        tokio::task::yield_now().await;
        tick(6).await;
        assert!(contains(&log, "last drain attempt"));
        assert!(
            log.lock()
                .unwrap()
                .iter()
                .any(|(t, m)| m.contains("last drain")
                    && t.duration_since(start) == Duration::from_secs(6))
        );
        tick(60).await;
        worker.await.unwrap();
        if !corrupt {
            assert!(f.owed().await);
        }
        assert!(f.config.private_key.exists());
        assert!(contains(&log, "drain attempt failed"));
    }
}

#[tokio::test(start_paused = true)]
async fn removed_ledger_or_directory_is_logged_and_never_recreated() {
    for directory in [false, true] {
        let f = Retired::new();
        f.owe().await;
        f.fake.reachable.store(false, Ordering::SeqCst);
        let (worker, log) = f.spawn().await;
        tokio::task::yield_now().await;
        if directory {
            fs::remove_dir_all(&f.root).unwrap();
        } else {
            fs::remove_file(f.root.join("ledger.json")).unwrap();
        }
        tick(2).await;
        worker.await.unwrap();
        assert!(contains(
            &log,
            if directory {
                "subdirectory gone"
            } else {
                "ledger.json gone"
            }
        ));
        assert!(!f.root.join("ledger.json").exists());
    }
}

#[tokio::test(start_paused = true)]
async fn key_cleanup_requires_retirement_existing_parent_matching_binding_and_no_owed_stop() {
    // Fresh provider with no run still cleans its key. All other cases retain it.
    for case in [
        "retired",
        "already-retired",
        "drained",
        "present",
        "parent-gone",
        "binding-changed",
        "dangling-symlink",
        "owed",
    ] {
        let mut f = Retired::new();
        if case == "drained" {
            f.owe().await;
        }
        if case == "owed" {
            f.owe().await;
            f.fake.reachable.store(false, Ordering::SeqCst);
            f.config.max_lifetime = Duration::from_secs(1);
        }
        let owner = f.prepare().await;
        if case == "already-retired" {
            fs::remove_dir_all(&f.state).unwrap();
        }
        let (worker, _) = f.worker();
        let worker = tokio::spawn(worker);
        drop(owner);
        tokio::task::yield_now().await;
        if case != "present" && case != "already-retired" {
            fs::remove_dir_all(&f.state).unwrap();
        }
        if case == "dangling-symlink" {
            std::os::unix::fs::symlink(f.state.with_file_name("absent"), &f.state).unwrap();
        }
        if case == "parent-gone" {
            fs::remove_dir(f.state.parent().unwrap()).unwrap();
        }
        if case == "binding-changed" {
            fs::write(f.root.join("state-dir"), b"changed").unwrap();
        }
        tick(65).await;
        worker.await.unwrap();
        assert_eq!(
            f.config.private_key.exists(),
            !matches!(case, "retired" | "already-retired" | "drained"),
            "{case}"
        );
        assert_eq!(
            f.root.join("known_hosts").exists(),
            !matches!(case, "retired" | "already-retired" | "drained"),
            "{case}"
        );
        for name in ["ledger.json", "markers", "state-dir", "part", "generations"] {
            assert!(f.root.join(name).exists());
        }
    }
}

#[tokio::test(start_paused = true)]
async fn yields_cancel_stop_and_masters_before_next_owner_then_waits_and_resumes() {
    let f = Retired::new();
    f.owe().await;
    f.fake.stop_hold.store(true, Ordering::SeqCst);
    let (worker, log) = f.spawn().await;
    f.fake.stopping.notified().await;
    assert_eq!(f.fake.stops_in_flight.load(Ordering::SeqCst), 1);
    let start = Instant::now();
    let owner = OwnerLock::acquire_provider(&f.root).await.unwrap();
    assert!(start.elapsed() <= Duration::from_secs(10));
    assert_eq!(f.fake.stops_in_flight.load(Ordering::SeqCst), 0);
    assert_eq!(f.fake.live_masters.load(Ordering::SeqCst), 0);
    assert!(f.owed().await);
    assert!(OwnerLock::try_existing(&f.root).unwrap().is_none());
    assert!(drainer_held(&f));
    tick(12).await;
    assert!(!worker.is_finished());
    assert!(contains(&log, "waiting for owner.lock"));
    assert!(drainer_held(&f));
    let mut newest = f.config.clone();
    let mut machine = newest.machines[0].clone();
    machine.hostname = "after-yield".into();
    newest.machines.push(machine);
    drop(publish(&f, &newest, owner.clone()).unwrap());
    let mut context = context();
    context.machine.hostname = "after-yield".into();
    FileStore::new(&f.root)
        .save(&RunRecord {
            instance: OTHER.into(),
            context,
            state: RunState::Unknown,
            owed_stop: true,
        })
        .await
        .unwrap();
    f.fake.stop_hold.store(false, Ordering::SeqCst);
    drop(owner);
    released(&f.root);
    tick(3).await;
    assert!(!f.owed().await);
    tick(60).await;
    worker.await.unwrap();
}

#[tokio::test(start_paused = true)]
async fn giving_up_removes_yield_and_stale_yield_does_not_stop_next_drainer() {
    let f = Retired::new();
    let (worker, log) = f.spawn().await;
    let owner = settle::settled(|| OwnerLock::acquire(&f.root)).unwrap();
    tokio::task::yield_now().await;
    let start = Instant::now();
    assert!(OwnerLock::acquire_provider(&f.root).await.is_err());
    assert_eq!(start.elapsed(), Duration::from_secs(10));
    assert!(!f.root.join("yield").exists());
    drop(owner);
    released(&f.root);
    fs::write(f.root.join("yield"), b"stale").unwrap();
    tick(2).await;
    assert!(contains(&log, "nothing owed"));
    assert!(!f.root.join("yield").exists());
    tick(60).await;
    worker.await.unwrap();
}

#[tokio::test(start_paused = true)]
async fn retry_wait_is_capped_at_five_minutes() {
    let mut f = Retired::new();
    f.config.max_lifetime = Duration::from_secs(2000);
    f.owe().await;
    f.fake.reachable.store(false, Ordering::SeqCst);
    let (worker, log) = f.spawn().await;
    tokio::task::yield_now().await;
    tick(812).await;
    let attempts: Vec<_> = log
        .lock()
        .unwrap()
        .iter()
        .filter(|(_, message)| *message == "drain attempt")
        .map(|(time, _)| *time)
        .collect();
    let gaps: Vec<_> = attempts
        .windows(2)
        .map(|w| w[1].duration_since(w[0]).as_secs())
        .collect();
    assert_eq!(gaps, [1, 2, 4, 8, 16, 32, 64, 128, 256, 300]);
    worker.abort();
    assert!(worker.await.unwrap_err().is_cancelled());
    assert!(settle::settled(|| OwnerLock::acquire(&f.root)).is_ok());
}

#[tokio::test(start_paused = true)]
async fn lifetime_bound_drops_a_stalled_attempt_before_the_final_attempt() {
    let mut f = Retired::new();
    f.config.max_lifetime = Duration::from_secs(1);
    f.owe().await;
    f.fake.stop_hold.store(true, Ordering::SeqCst);
    let (worker, log) = f.spawn().await;
    f.fake.stopping.notified().await;
    assert_eq!(f.fake.stops_in_flight.load(Ordering::SeqCst), 1);
    tick(3).await;
    assert!(contains(&log, "last drain attempt"));
    assert_eq!(f.fake.stops_in_flight.load(Ordering::SeqCst), 1);
    assert_eq!(f.fake.live_masters.load(Ordering::SeqCst), 1);
    assert_eq!(f.fake.masters.load(Ordering::SeqCst), 2);
    f.fake.stop_hold.store(false, Ordering::SeqCst);
    f.fake.stop_release.notify_one();
    tokio::task::yield_now().await;
    tick(60).await;
    worker.await.unwrap();
    assert!(!f.owed().await);
    assert_eq!(f.fake.stops_in_flight.load(Ordering::SeqCst), 0);
    assert_eq!(f.fake.live_masters.load(Ordering::SeqCst), 0);
}

#[tokio::test(start_paused = true)]
async fn actual_launch_uses_new_session_null_input_and_private_append_log() {
    use std::{ffi::CString, os::unix::ffi::OsStrExt};
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let bin = root.join("bin");
    fs::create_dir(&bin).unwrap();
    let fifo = root.join("release");
    let name = CString::new(fifo.as_os_str().as_bytes()).unwrap();
    // SAFETY: NUL-terminated path in this test's private directory.
    assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
    let script = bin.join(startup::DRAINER);
    fs::write(&script, b"#!/bin/sh\nif read value; then exit 11; fi\ncase ${CONDUIT_URL+x}${CONDUIT_IDENTITY_DIR+x}${SSH_AUTH_SOCK+x} in '') ;; *) exit 12;; esac\n[ \"$2\" = \"$1/state\" ] || exit 13\n[ \"$#\" = 2 ] || exit 14\nprintf '%s\\n' \"$$\" > \"$1/pid\"\nprintf 'stdout\\n'\nprintf 'stderr\\n' >&2\nprintf 'ready' > \"$1/ready\"\nread value < \"$1/release\"\nprintf 'done' > \"$1/done\"\n").unwrap();
    fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
    fs::write(root.join("drainer.log"), b"previous\n").unwrap();
    let launch = startup::Launch::new(&bin.join("provider"), root, &root.join("state"), |_| None);
    launch.spawn().unwrap();
    // No sleeps: the child uses a FIFO to stay alive until the test releases it.
    for _ in 0..100_000 {
        if root.join("ready").exists() {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert!(root.join("ready").exists());
    let pid: i32 = fs::read_to_string(root.join("pid"))
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    // SAFETY: query only the child's process/session group; no signals are sent.
    assert_eq!(unsafe { libc::getsid(pid) }, pid);
    assert_eq!(unsafe { libc::getpgid(pid) }, pid);
    assert_eq!(
        fs::read(root.join("drainer.log")).unwrap(),
        b"previous\nstdout\nstderr\n"
    );
    assert_eq!(
        fs::metadata(root.join("drainer.log"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    tokio::fs::write(&fifo, b"release\n").await.unwrap();
    for _ in 0..100_000 {
        if root.join("done").exists() {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert!(root.join("done").exists());
}
