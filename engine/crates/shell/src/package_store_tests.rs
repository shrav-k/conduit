use super::*;
use conduit_core::packages::bytes_digest;
use store::WriteFault;

// A concurrently forked child can retain a dropped fixture's flock until exec.
// Retry only fixture opens, never production opens or competing-owner assertions.
pub(crate) fn fixture_open<T>(mut open: impl FnMut() -> Result<T>) -> T {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        match open() {
            Ok(value) => return value,
            Err(error)
                if error == "store already has an active shell" && Instant::now() < deadline =>
            {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(error) => panic!("fixture open failed: {error}"),
        }
    }
}

fn open(path: &Path) -> Shell {
    fixture_open(|| Shell::open(path, context(), Config::default()))
}

#[test]
fn fixture_reopen_keeps_competing_owners_out() {
    use std::os::{fd::AsRawFd, unix::net::UnixStream};
    let temp = temporary();
    let store = Store::open(temp.path()).unwrap();
    assert_eq!(
        Store::open(temp.path()).err().unwrap(),
        "store already has an active shell"
    );
    assert_eq!(
        Shell::open(temp.path(), context(), Config::default())
            .err()
            .unwrap(),
        "store already has an active shell"
    );
    let (parent, child) = UnixStream::pair().unwrap();
    let parent_fd = parent.as_raw_fd();
    let child_fd = child.as_raw_fd();
    // The child uses only async-signal-safe calls between fork and _exit. It
    // merely holds inherited descriptors; it never opens or reads a store file.
    let pid = unsafe { libc::fork() };
    assert!(pid >= 0);
    if pid == 0 {
        unsafe {
            libc::close(parent_fd);
            let mut byte = 0u8;
            while libc::read(child_fd, (&mut byte as *mut u8).cast(), 1) < 0 {}
            libc::_exit(0);
        }
    }
    drop(child);
    struct ChildLease(libc::pid_t, UnixStream);
    impl Drop for ChildLease {
        fn drop(&mut self) {
            let _ = self.1.write_all(b"release");
            while unsafe { libc::waitpid(self.0, std::ptr::null_mut(), 0) } < 0 {
                if std::io::Error::last_os_error().kind() != std::io::ErrorKind::Interrupted {
                    break;
                }
            }
        }
    }
    let mut child = ChildLease(pid, parent);
    drop(store);
    let mut attempts = 0;
    let reopened = fixture_open(|| {
        let result = Store::open(temp.path());
        attempts += 1;
        if attempts == 1 {
            assert_eq!(
                result.as_ref().err().unwrap(),
                "store already has an active shell"
            );
            child.1.write_all(b"release").unwrap();
        }
        result
    });
    assert!(attempts >= 2);
    assert_eq!(
        Store::open(temp.path()).err().unwrap(),
        "store already has an active shell"
    );
    drop(reopened);
}

mod programs {
    use super::*;
    use crate::runner::tests::{
        WritablePrograms, exec_output, installed_command_state, installed_program, test_runner,
    };
    use conduit_core::archives::Member;
    use std::os::unix::fs::MetadataExt;

    fn fixture(marker: &str) -> (Value, State) {
        let gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        let mut tar = tar::Builder::new(gzip);
        let script = format!("#!/bin/sh\nprintf '{marker}'");
        for (name, bytes, mode) in [
            ("lib/worker", script.as_bytes(), 0o755),
            ("lib/data", b"data".as_slice(), 0o640),
            ("lib/removed", b"removed".as_slice(), 0o444),
        ] {
            let mut header = tar::Header::new_ustar();
            header.set_mode(mode);
            header.set_uid(0);
            header.set_gid(0);
            header.set_mtime(0);
            header.set_size(bytes.len() as u64);
            tar.append_data(&mut header, name, bytes).unwrap();
        }
        let mut header = tar::Header::new_ustar();
        header.set_entry_type(tar::EntryType::Symlink);
        header.set_mode(0o777);
        header.set_size(0);
        header.set_uid(0);
        header.set_gid(0);
        header.set_mtime(0);
        tar.append_link(&mut header, "bin/worker", "../lib/worker")
            .unwrap();
        let bytes = tar.into_inner().unwrap().finish().unwrap();
        let files = boundary::archive_metadata::unpack(&bytes).unwrap();
        let digest = bytes_digest(&bytes);
        let item = json!({"command":["/old-install/bin/worker"]});
        let mut state = installed_command_state(&item, "./bin/worker", files);
        state.installed_packages[0].package.digest = digest.clone();
        state.objects[0]["held"]["contexts"]["snapshot-0"]["digest"] = json!(digest);
        (item, state)
    }

    fn root(parent: &Path, state: &State) -> PathBuf {
        parent.join("store/programs").join(
            state.installed_packages[0]
                .package
                .digest
                .strip_prefix("sha256:")
                .unwrap(),
        )
    }

    fn check_tree(root: &Path, state: &State) {
        fn paths(root: &Path, path: &Path, found: &mut BTreeSet<String>) {
            for entry in fs::read_dir(path).unwrap() {
                let path = entry.unwrap().path();
                found.insert(path.strip_prefix(root).unwrap().to_str().unwrap().into());
                if fs::symlink_metadata(&path).unwrap().is_dir() {
                    paths(root, &path, found);
                }
            }
        }
        let files = &state.installed_packages[0].files;
        let mut found = BTreeSet::new();
        paths(root, root, &mut found);
        assert_eq!(found, files.keys().cloned().collect());
        assert_eq!(
            fs::symlink_metadata(root).unwrap().permissions().mode() & 0o7777,
            0o555
        );
        for (name, member) in files {
            let path = root.join(name);
            let metadata = fs::symlink_metadata(&path).unwrap();
            match member {
                Member::Directory => {
                    assert!(metadata.is_dir());
                    assert_eq!(metadata.permissions().mode() & 0o7777, 0o555);
                }
                Member::File { bytes, mode } => {
                    assert!(metadata.is_file());
                    assert_eq!(metadata.permissions().mode() & 0o7777, mode & 0o555);
                    assert_eq!(fs::read(path).unwrap(), *bytes);
                }
                Member::Link { target } => {
                    assert!(metadata.is_symlink());
                    assert_eq!(
                        path.canonicalize().unwrap(),
                        root.join(target).canonicalize().unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn runs_of_one_digest_share_one_read_only_copy() {
        let parent = temporary();
        let _writable = WritablePrograms(parent.path().join("store/programs"));
        let (_shell, runner) = test_runner(parent.path());
        let (item, state) = fixture("shared-program");
        let (first, args) = installed_program(&runner, &item, &state).unwrap();
        let inode = fs::metadata(&first).unwrap().ino();
        assert_eq!(
            exec_output(&runner, &first, &args, parent.path(), &BTreeMap::new()),
            "shared-program"
        );
        let (second, args) = installed_program(&runner, &item, &state).unwrap();
        assert_eq!(first, second, "runs must share one program directory");
        assert_eq!(fs::metadata(&second).unwrap().ino(), inode);
        assert_eq!(second, root(parent.path(), &state).join("./bin/worker"));
        check_tree(&root(parent.path(), &state), &state);
        assert_eq!(
            exec_output(&runner, &second, &args, parent.path(), &BTreeMap::new()),
            "shared-program"
        );
        std::thread::scope(|scope| {
            for _ in 0..4 {
                scope.spawn(|| {
                    assert_eq!(installed_program(&runner, &item, &state).unwrap().0, first)
                });
            }
        });
        let (other_item, other_state) = fixture("another-program");
        std::thread::scope(|scope| {
            for _ in 0..4 {
                scope.spawn(|| {
                    assert_eq!(
                        installed_program(&runner, &other_item, &other_state)
                            .unwrap()
                            .0,
                        root(parent.path(), &other_state).join("./bin/worker")
                    );
                });
            }
        });
        let (other, args) = installed_program(&runner, &other_item, &other_state).unwrap();
        assert_ne!(other, first);
        check_tree(&root(parent.path(), &other_state), &other_state);
        assert_eq!(
            exec_output(&runner, &other, &args, parent.path(), &BTreeMap::new()),
            "another-program"
        );
        assert_eq!(
            fs::read_dir(parent.path().join("store/programs"))
                .unwrap()
                .count(),
            2
        );
        for invalid in [
            "sha256:bad".to_string(),
            format!("sha256:{}", "A".repeat(64)),
            "other:digest".into(),
        ] {
            let mut bad = state.clone();
            bad.installed_packages[0].package.digest = invalid.clone();
            bad.objects[0]["held"]["contexts"]["snapshot-0"]["digest"] = json!(invalid);
            let error = installed_program(&runner, &item, &bad).unwrap_err();
            for expected in [
                &invalid,
                &bad.installed_packages[0].package.address,
                &bad.installed_packages[0].package.version,
            ] {
                assert!(error.contains(expected));
            }
        }
    }

    #[test]
    fn an_altered_program_copy_is_made_again_before_the_next_run() {
        let parent = temporary();
        let _writable = WritablePrograms(parent.path().join("store/programs"));
        let (shell, runner) = test_runner(parent.path());
        let (item, state) = fixture("shared-program");
        let (first, args) = installed_program(&runner, &item, &state).unwrap();
        assert_eq!(
            exec_output(&runner, &first, &args, parent.path(), &BTreeMap::new()),
            "shared-program"
        );
        assert_eq!(
            first,
            installed_program(&runner, &item, &state).unwrap().0,
            "runs must share one program directory"
        );
        let root = root(parent.path(), &state);
        fs::set_permissions(root.join("lib"), fs::Permissions::from_mode(0o755)).unwrap();
        fs::set_permissions(root.join("lib/worker"), fs::Permissions::from_mode(0o755)).unwrap();
        fs::write(root.join("lib/worker"), b"altered").unwrap();
        fs::write(root.join("lib/extra"), b"extra").unwrap();
        fs::remove_file(root.join("lib/removed")).unwrap();
        let (second, args) = installed_program(&runner, &item, &state).unwrap();
        assert_eq!(first, second);
        check_tree(&root, &state);
        assert_eq!(
            exec_output(&runner, &second, &args, parent.path(), &BTreeMap::new()),
            "shared-program"
        );
        let stale: Vec<_> = fs::read_dir(parent.path().join("store/programs"))
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| {
                p.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with(".stale-")
            })
            .collect();
        assert_eq!(stale.len(), 1);
        assert_eq!(fs::read(stale[0].join("lib/worker")).unwrap(), b"altered");
        // Relative links remain with the quarantined tree, not the replacement.
        assert_eq!(fs::read(stale[0].join("bin/worker")).unwrap(), b"altered");
        let outside = parent.path().join("outside-sentinel");
        fs::write(&outside, b"untouched").unwrap();
        fs::set_permissions(&outside, fs::Permissions::from_mode(0o400)).unwrap();
        // Isolate each mismatch so mode checks cannot conceal missing byte, target,
        // kind or membership checks. Restore temporary write access after each edit.
        for change in [
            "bytes",
            "file-mode",
            "directory-mode",
            "link-target",
            "file-kind",
            "extra",
            "missing",
            "root-mode",
        ] {
            match change {
                "bytes" => {
                    fs::set_permissions(root.join("lib/worker"), fs::Permissions::from_mode(0o755))
                        .unwrap();
                    fs::write(root.join("lib/worker"), b"different bytes").unwrap();
                    fs::set_permissions(root.join("lib/worker"), fs::Permissions::from_mode(0o555))
                        .unwrap();
                }
                "file-mode" => {
                    fs::set_permissions(root.join("lib/worker"), fs::Permissions::from_mode(0o755))
                        .unwrap()
                }
                "directory-mode" => {
                    fs::set_permissions(root.join("lib"), fs::Permissions::from_mode(0o755))
                        .unwrap()
                }
                "link-target" => {
                    fs::set_permissions(root.join("bin"), fs::Permissions::from_mode(0o755))
                        .unwrap();
                    let target = fs::read_link(root.join("bin/worker")).unwrap();
                    fs::remove_file(root.join("bin/worker")).unwrap();
                    // Same resolution, different target bytes (even Path equality normalizes this).
                    std::os::unix::fs::symlink(
                        format!("{}/./worker", target.parent().unwrap().display()),
                        root.join("bin/worker"),
                    )
                    .unwrap();
                    fs::set_permissions(root.join("bin"), fs::Permissions::from_mode(0o555))
                        .unwrap();
                }
                "file-kind" => {
                    fs::set_permissions(root.join("lib"), fs::Permissions::from_mode(0o755))
                        .unwrap();
                    fs::remove_file(root.join("lib/data")).unwrap();
                    std::os::unix::fs::symlink(&outside, root.join("lib/data")).unwrap();
                    fs::set_permissions(root.join("lib"), fs::Permissions::from_mode(0o555))
                        .unwrap();
                }
                "extra" | "missing" => {
                    fs::set_permissions(root.join("lib"), fs::Permissions::from_mode(0o755))
                        .unwrap();
                    if change == "extra" {
                        fs::write(root.join("lib/extra"), b"extra").unwrap();
                    } else {
                        fs::remove_file(root.join("lib/removed")).unwrap();
                    }
                    fs::set_permissions(root.join("lib"), fs::Permissions::from_mode(0o555))
                        .unwrap();
                }
                "root-mode" => {
                    fs::set_permissions(&root, fs::Permissions::from_mode(0o755)).unwrap()
                }
                _ => unreachable!(),
            }
            let count = fs::read_dir(parent.path().join("store/programs"))
                .unwrap()
                .count();
            assert_eq!(installed_program(&runner, &item, &state).unwrap().0, first);
            check_tree(&root, &state);
            assert_eq!(
                fs::read_dir(parent.path().join("store/programs"))
                    .unwrap()
                    .count(),
                count + 1,
                "mismatch not quarantined: {change}"
            );
        }
        // A failed quarantine/materialization identifies the complete package.
        fs::set_permissions(&root, fs::Permissions::from_mode(0o755)).unwrap();
        let programs = parent.path().join("store/programs");
        fs::set_permissions(&programs, fs::Permissions::from_mode(0o500)).unwrap();
        let failed = installed_program(&runner, &item, &state);
        fs::set_permissions(&programs, fs::Permissions::from_mode(0o700)).unwrap();
        let error = failed.unwrap_err();
        let package = &state.installed_packages[0].package;
        for expected in [&package.address, &package.version, &package.digest] {
            assert!(error.contains(expected));
        }
        installed_program(&runner, &item, &state).unwrap();
        check_tree(&root, &state);
        let inode = fs::metadata(&root).unwrap().ino();
        let building = parent.path().join("store/programs/.building-interrupted");
        fs::create_dir(&building).unwrap();
        fs::write(building.join("partial"), b"partial").unwrap();
        std::os::unix::fs::symlink(&outside, building.join("link")).unwrap();
        fs::set_permissions(&building, fs::Permissions::from_mode(0o555)).unwrap();
        drop(runner);
        drop(shell);
        let (_shell, _runner) = test_runner(parent.path());
        assert!(!stale[0].exists());
        assert!(!building.exists());
        check_tree(&root, &state);
        assert_eq!(fs::metadata(&root).unwrap().ino(), inode);
        assert_eq!(fs::read(&outside).unwrap(), b"untouched");
        assert_eq!(
            fs::metadata(&outside).unwrap().permissions().mode() & 0o7777,
            0o400
        );
        assert_eq!(
            fs::read_dir(parent.path().join("store/programs"))
                .unwrap()
                .count(),
            1
        );
        let state_file = parent.path().join("store/state.json");
        let before = fs::read(&state_file).unwrap();
        let blocked = programs.join(".building-blocked");
        fs::create_dir(&blocked).unwrap();
        fs::set_permissions(&programs, fs::Permissions::from_mode(0o500)).unwrap();
        let failed = crate::runner::cleanup_program_copies(&programs);
        fs::set_permissions(&programs, fs::Permissions::from_mode(0o700)).unwrap();
        let error = failed.unwrap_err();
        assert!(error.contains(&format!(
            "cannot remove the leftover program copy {}:",
            blocked.display()
        )));
        assert!(error.contains("state.json is unchanged by program cleanup"));
        assert!(error.contains(&format!(
            "make {} writable or remove it, and start again",
            blocked.display()
        )));
        assert_eq!(fs::read(&state_file).unwrap(), before);
        crate::runner::cleanup_program_copies(&programs).unwrap();
        assert!(!blocked.exists());
    }
}

const ADDRESS: &str = "example.org/package-store";

fn pack(version: &str) -> Vec<u8> {
    let manifest = serde_json::to_vec(&json!({
        "apiVersion":"github.com/shrav-k/conduit/v0", "address":ADDRESS, "version":version
    }))
    .unwrap();
    let gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    let mut tar = tar::Builder::new(gzip);
    for (name, bytes, mode) in [
        ("conduit.json", manifest.as_slice(), 0o644),
        ("payload", b"original".as_slice(), 0o755),
    ] {
        let mut header = tar::Header::new_ustar();
        header.set_mode(mode);
        header.set_uid(0);
        header.set_gid(0);
        header.set_mtime(0);
        header.set_size(bytes.len() as u64);
        tar.append_data(&mut header, name, bytes).unwrap();
    }
    tar.into_inner().unwrap().finish().unwrap()
}

fn start(path: &Path) -> Shell {
    let shell = open(path);
    shell
        .submit(
            json!({"kind":"first-start","machine":{"hostname":"package-store.test","state":"reachable","labels":{"kubernetes.io/hostname":"package-store.test","kubernetes.io/os":"darwin","kubernetes.io/arch":"arm64"}}}),
            &MissingExecutor,
        )
        .unwrap();
    shell
}

fn install_input(shell: &Shell, bytes: &[u8], version: &str) -> Value {
    json!({"kind":"install-archive","snapshots":[],"builtin":shell.context.builtin,
        "archive":{"name":"package.tar.gz","address":ADDRESS,"version":version,
        "digest":bytes_digest(bytes),"directory":"file:///archive/package/",
        "completion":{"status":"complete","bytes":bytes}}})
}

fn public_input(shell: &Shell, source: &Path, bytes: &[u8]) -> Value {
    fs::write(source, bytes).unwrap();
    let root = shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| o["kind"] == "Interface" && o["name"] == "root")
        .unwrap()["id"]
        .clone();
    json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":root},
        "tool":"management.install.v1","arguments":{"address":ADDRESS,"version":"1.0.0",
        "digest":bytes_digest(bytes),"fetch":url::Url::from_file_path(source).unwrap().as_str()}}})
}

fn disk(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path.join("state.json")).unwrap()).unwrap()
}

fn same_disk(shell: &Shell) {
    let stored = store::read_stored(&shell.directory).unwrap();
    assert!(
        *shell.state().unwrap() == stored.state,
        "memory differs from disk"
    );
    let writer = shell.writer.lock().unwrap();
    let memory = writer.store.take().unwrap();
    assert_eq!(memory.output_times, stored.output_times);
    writer.store.remember(memory);
}

fn copy(path: &Path, bytes: &[u8]) -> PathBuf {
    path.join(bytes_digest(bytes).strip_prefix("sha256:").unwrap())
}

// Compare the public failure surface except for generated IDs, times and diagnostics.
fn normalize(value: &mut Value, ids: &BTreeSet<String>) {
    match value {
        Value::String(s) if ids.contains(s) => *s = "<id>".into(),
        Value::String(s) if chrono::DateTime::parse_from_rfc3339(s).is_ok() => *s = "<time>".into(),
        Value::Array(items) => items.iter_mut().for_each(|v| normalize(v, ids)),
        Value::Object(fields) => {
            for (key, value) in fields {
                if matches!(key.as_str(), "message" | "text") && value.is_string() {
                    *value = json!("<diagnostic>");
                } else {
                    normalize(value, ids);
                }
            }
        }
        _ => {}
    }
}

#[test]
fn a_failed_keep_leaves_the_store_as_it_was() {
    let bytes = pack("1.0.0");
    let temp = temporary();
    let shell = start(temp.path());
    fs::write(temp.path().join("packages"), b"blocked").unwrap();
    let before = fs::read(temp.path().join("state.json")).unwrap();
    assert!(
        shell
            .submit(install_input(&shell, &bytes, "1.0.0"), &MissingExecutor)
            .is_err()
    );
    assert_eq!(fs::read(temp.path().join("state.json")).unwrap(), before);
    assert!(shell.state().unwrap().installed_packages.is_empty());
    same_disk(&shell);

    let mut surfaces = Vec::new();
    for broken in [true, false] {
        let temp = temporary();
        let shell = start(temp.path());
        let input = public_input(&shell, &temp.path().join("source"), &bytes);
        if broken {
            fs::write(temp.path().join("packages"), b"blocked").unwrap();
        } else {
            *shell.writer.lock().unwrap().store.fault.borrow_mut() = Some(WriteFault::Write);
        }
        let reply = shell.submit(input, &MissingExecutor).unwrap();
        let state = shell.state().unwrap();
        assert!(state.installed_packages.is_empty());
        let mut surface =
            json!({"reply":reply,"records":state.records,"packages":state.installed_packages});
        normalize(&mut surface, &state.allocated_ids);
        surfaces.push(surface);
        same_disk(&shell);
    }
    assert_eq!(surfaces[0], surfaces[1]);

    let temp = temporary();
    let shell = start(temp.path());
    let input = install_input(&shell, &bytes, "1.0.0");
    *shell.writer.lock().unwrap().store.fault.borrow_mut() = Some(WriteFault::Rename);
    assert!(shell.submit(input.clone(), &MissingExecutor).is_err());
    assert!(copy(&temp.path().join("packages"), &bytes).is_file());
    assert!(shell.state().unwrap().installed_packages.is_empty());
    same_disk(&shell);
    assert_eq!(
        shell.submit(input, &MissingExecutor).unwrap()["changed"],
        true
    );
    same_disk(&shell);
    drop(shell);
    let shell = open(temp.path());
    assert_eq!(shell.state().unwrap().installed_packages.len(), 1);
    same_disk(&shell);
}

mod migration {
    use super::*;
    use conduit_core::archives::Member;
    use std::os::unix::fs::MetadataExt;

    fn earlier(path: &Path, count: usize) -> (Stored, Vec<Vec<u8>>) {
        let shell = start(path);
        let mut state = (*shell.state().unwrap()).clone();
        let mut archives = Vec::new();
        for n in 1..=count {
            let version = format!("{n}.0.0");
            let bytes = pack(&version);
            let input = serde_json::from_value(install_input(&shell, &bytes, &version)).unwrap();
            let transition = conduit_core::step_owned(state, input, &shell.context).unwrap();
            assert_eq!(transition.reply["changed"], true);
            state = transition.state;
            archives.push(bytes);
        }
        let stored = Stored {
            version: store::VERSION,
            state,
            output_times: BTreeMap::new(),
        };
        drop(shell);
        write_earlier(path, &stored);
        (stored, archives)
    }

    fn write_earlier(path: &Path, stored: &Stored) {
        fs::write(path.join("state.json"), serde_json::to_vec(stored).unwrap()).unwrap();
    }

    fn place(path: &Path, bytes: &[u8]) {
        fs::create_dir_all(path).unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
        let file = copy(path, bytes);
        fs::write(&file, bytes).unwrap();
        fs::set_permissions(file, fs::Permissions::from_mode(0o600)).unwrap();
    }

    fn open_places(path: &Path, places: &[PathBuf]) -> Shell {
        fixture_open(|| {
            Shell::open(
                path,
                context(),
                Config {
                    archive_places: places.to_vec(),
                    ..Config::default()
                },
            )
        })
    }

    fn preserved(shell: &Shell, before: &Stored) {
        assert!(
            *shell.state().unwrap() == before.state,
            "migration changed State"
        );
        assert_eq!(
            store::read_stored(&shell.directory).unwrap().output_times,
            before.output_times
        );
        same_disk(shell);
    }

    fn inline(path: &Path) -> Vec<bool> {
        disk(path)["state"]["installed_packages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p.get("files").is_some())
            .collect()
    }

    #[test]
    fn an_earlier_stores_members_move_out_once() {
        let temp = temporary();
        let (before, archives) = earlier(temp.path(), 2);
        let source = temp.path().join("source");
        place(&source, &archives[0]);
        place(&temp.path().join("packages"), &archives[1]);
        let shell = open_places(temp.path(), std::slice::from_ref(&source));
        assert_eq!(inline(temp.path()), [false, false]);
        preserved(&shell, &before);
        assert!(shell.package_store_messages().is_empty());
        assert_eq!(
            fs::metadata(temp.path().join("packages"))
                .unwrap()
                .permissions()
                .mode()
                & 0o7777,
            0o700
        );
        let identities: Vec<_> = archives
            .iter()
            .map(|bytes| {
                let meta = fs::metadata(copy(&temp.path().join("packages"), bytes)).unwrap();
                assert_eq!(meta.permissions().mode() & 0o7777, 0o600);
                (meta.ino(), meta.mtime(), meta.mtime_nsec())
            })
            .collect();
        drop(shell);
        fs::remove_dir_all(&source).unwrap();
        let store = fixture_open(|| Store::open(temp.path()));
        *store.fault.borrow_mut() = Some(WriteFault::Write);
        assert!(store.move_package_members(&[source]).unwrap().is_empty());
        assert_eq!(*store.fault.borrow(), Some(WriteFault::Write));
        for (bytes, identity) in archives.iter().zip(identities) {
            let meta = fs::metadata(copy(&temp.path().join("packages"), bytes)).unwrap();
            assert_eq!((meta.ino(), meta.mtime(), meta.mtime_nsec()), identity);
        }
    }

    #[test]
    fn a_package_found_nowhere_keeps_its_members_and_the_store_opens() {
        let temp = temporary();
        let (mut before, archives) = earlier(temp.path(), 3);
        before.state.installed_packages[2].files.insert(
            "payload".into(),
            Member::File {
                bytes: b"changed".to_vec(),
                mode: 0o755,
            },
        );
        write_earlier(temp.path(), &before);
        let source = temp.path().join("source");
        place(&source, &archives[0]);
        place(&source, &archives[2]);
        let shell = open_places(temp.path(), std::slice::from_ref(&source));
        assert_eq!(inline(temp.path()), [false, true, true]);
        preserved(&shell, &before);
        let lines = shell.package_store_messages();
        assert_eq!(lines.len(), 2);
        for (line, index) in lines.iter().zip([1, 2]) {
            assert!(line.starts_with(&format!(
                "package {ADDRESS} {}.0.0 ({}) keeps its members in state.json: ",
                index + 1,
                bytes_digest(&archives[index])
            )));
            for path in [temp.path().join("packages"), source.clone()] {
                assert!(line.contains(copy(&path, &archives[index]).to_str().unwrap()));
            }
            assert!(line.contains("absent"));
            // Only public, placeholder paths appear in diagnostic evidence.
            eprintln!("{}", line.replace(temp.path().to_str().unwrap(), "<store>"));
        }
        assert!(lines[0].ends_with("put the archive, named by its SHA-256 hex, in the package cache or a mirror, and open the store again"));
        assert!(lines[1].contains("members differ at payload"));
        assert!(
            lines[1].ends_with("no archive can move it; keep the store as it is and report it")
        );
        drop(shell);
        place(&source, &archives[1]);
        let shell = open_places(temp.path(), &[source]);
        assert_eq!(inline(temp.path()), [false, false, true]);
        assert_eq!(shell.package_store_messages().len(), 1);
        preserved(&shell, &before);
    }

    #[test]
    fn a_cut_off_move_leaves_state_json_as_it_was() {
        for fault in [
            Some(WriteFault::Rename),
            None,
            Some(WriteFault::DirectorySync),
        ] {
            let temp = temporary();
            let (before, archives) = earlier(temp.path(), 2);
            let source = temp.path().join("source");
            for bytes in &archives {
                place(&source, bytes);
            }
            let original = fs::read(temp.path().join("state.json")).unwrap();
            let store = fixture_open(|| Store::open(temp.path()));
            *store.fault.borrow_mut() = fault;
            let blocked = copy(&temp.path().join("packages"), &archives[1]);
            if fault.is_none() {
                store::private_directory(&temp.path().join("packages")).unwrap();
                fs::create_dir(&blocked).unwrap();
            }
            let error = store
                .move_package_members(std::slice::from_ref(&source))
                .unwrap_err();
            if fault.is_none() {
                assert!(error.starts_with(&format!(
                    "moving package {ADDRESS} 2.0.0 ({}) out of state.json: cannot keep {}:",
                    bytes_digest(&archives[1]),
                    blocked.display()
                )));
                assert!(error.ends_with(&format!(
                    "state.json is unchanged; fix {} and open the store again",
                    blocked.display()
                )));
            } else {
                for (index, bytes) in archives.iter().enumerate() {
                    assert!(error.contains(&format!(
                        "{ADDRESS} {}.0.0 ({})",
                        index + 1,
                        bytes_digest(bytes)
                    )));
                }
                let path = temp.path().join("state.json");
                assert!(
                    error.contains(&format!("cannot rewrite state.json at {}:", path.display()))
                );
                assert!(
                    error.ends_with(&format!("fix {} and open the store again", path.display()))
                );
                if fault == Some(WriteFault::DirectorySync) {
                    assert!(error.contains("state.json was already rewritten; the next open reads the rewrite and finishes the move"));
                    assert!(!error.contains("state.json is unchanged"));
                } else {
                    assert!(error.contains("state.json is unchanged"));
                }
            }
            if fault == Some(WriteFault::DirectorySync) {
                assert_eq!(inline(temp.path()), [false, false]);
            } else {
                assert_eq!(fs::read(temp.path().join("state.json")).unwrap(), original);
            }
            assert_eq!(
                fs::read(copy(&temp.path().join("packages"), &archives[0])).unwrap(),
                archives[0]
            );
            if fault.is_none() {
                assert!(blocked.is_dir());
                fs::remove_dir(&blocked).unwrap();
            } else {
                assert_eq!(fs::read(&blocked).unwrap(), archives[1]);
                fs::remove_file(copy(&source, &archives[1])).unwrap();
            }
            fs::remove_file(copy(&source, &archives[0])).unwrap();
            drop(store);
            let shell = open_places(temp.path(), std::slice::from_ref(&source));
            preserved(&shell, &before);
            assert_eq!(inline(temp.path()), [false, false]);
            assert!(shell.package_store_messages().is_empty());
            drop(shell);
            let shell = open_places(temp.path(), &[]);
            preserved(&shell, &before);
            assert!(shell.package_store_messages().is_empty());
        }
    }

    #[test]
    fn a_shared_digest_moves_only_when_every_entry_matches() {
        for mixed in [false, true] {
            let temp = temporary();
            let (mut before, archives) = earlier(temp.path(), 1);
            let mut other = before.state.installed_packages[0].clone();
            other.package.address = "example.org/other-entry".into();
            other.package.project.manifest.as_mut().unwrap()["address"] =
                json!(other.package.address);
            other.files.insert(
                "payload".into(),
                Member::File {
                    bytes: b"different".to_vec(),
                    mode: 0o755,
                },
            );
            before.state.installed_packages.push(other);
            write_earlier(temp.path(), &before);
            let source = temp.path().join("source");
            place(&source, &archives[0]);
            if mixed {
                place(&temp.path().join("packages"), &archives[0]);
                let mut value = disk(temp.path());
                value["state"]["installed_packages"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("files");
                fs::write(
                    temp.path().join("state.json"),
                    serde_json::to_vec(&value).unwrap(),
                )
                .unwrap();
            }
            let shell = open_places(temp.path(), std::slice::from_ref(&source));
            preserved(&shell, &before);
            assert_eq!(inline(temp.path()), [true, true]);
            assert_eq!(shell.package_store_messages().len(), 2);
            assert!(
                shell.package_store_messages()[0]
                    .contains("held back: another entry of this digest differs at payload")
            );
            assert!(shell.package_store_messages()[1].contains("members differ at payload"));
            for line in shell.package_store_messages() {
                assert!(line.contains(&bytes_digest(&archives[0])));
            }
            // An unrelated template registration commits through the shell normally.
            let input = json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,
            "project":{"manifest":null,"documents":[{"apiVersion":"github.com/shrav-k/conduit/v0","kind":"Pool","metadata":{"name":"unrelated"},"spec":{"type":format!("{}#pool.v1",shell.context.core_package)}}],"directory":null},
            "request":{"sender":before.state.objects[4]["id"],"caller":before.state.objects[4]["id"],"conduit":before.state.objects[5]["id"],"target":before.state.objects[2]["id"],"operation":format!("{}#register.v1",shell.context.core_package)}});
            let transition = shell.commit(input).unwrap();
            assert!(transition.intents.is_empty());
            let reply = transition.ticket.wait().unwrap();
            assert_eq!(reply["accepted"], true, "registration reply: {reply}");
            assert!(shell.state().unwrap().objects != before.state.objects);
            assert_eq!(
                shell.state().unwrap().installed_packages,
                before.state.installed_packages
            );
            same_disk(&shell);
            assert_eq!(inline(temp.path()), [true, true]);
            drop(shell);
            let shell = open_places(temp.path(), std::slice::from_ref(&source));
            assert_eq!(
                shell.state().unwrap().installed_packages,
                before.state.installed_packages
            );
            same_disk(&shell);
            assert_eq!(inline(temp.path()), [true, true]);
            drop(shell);
            if !mixed {
                before.state.installed_packages[1].files =
                    before.state.installed_packages[0].files.clone();
                write_earlier(temp.path(), &before);
                let shell = open_places(temp.path(), &[source]);
                preserved(&shell, &before);
                assert_eq!(inline(temp.path()), [false, false]);
                assert!(shell.package_store_messages().is_empty());
            }
        }
    }

    #[test]
    fn an_unusable_candidate_gives_way_to_a_later_one() {
        let temp = temporary();
        let (mut before, archives) = earlier(temp.path(), 3);
        let invalid = b"not gzip";
        before.state.installed_packages[1].package.digest = bytes_digest(invalid);
        write_earlier(temp.path(), &before);
        let places: Vec<_> = (0..5)
            .map(|n| temp.path().join(format!("place-{n}")))
            .collect();
        for path in &places {
            fs::create_dir(path).unwrap();
        }
        place(&places[4], &archives[0]);
        std::os::unix::fs::symlink(
            copy(&places[4], &archives[0]),
            copy(&places[0], &archives[0]),
        )
        .unwrap();
        fs::create_dir(copy(&places[1], &archives[0])).unwrap();
        fs::write(copy(&places[2], &archives[0]), b"wrong digest").unwrap();
        place(&places[3], &archives[0]);
        fs::set_permissions(
            copy(&places[3], &archives[0]),
            fs::Permissions::from_mode(0o0),
        )
        .unwrap();
        assert!(
            fs::read(copy(&places[3], &archives[0])).is_err(),
            "fixture must be unreadable to this test user"
        );
        place(&places[4], invalid);
        // The third package has no usable candidate. Following its symbolic link
        // would incorrectly move it, so this guards both refusal and each outcome.
        let hidden = temp.path().join("not-a-place");
        place(&hidden, &archives[2]);
        std::os::unix::fs::symlink(copy(&hidden, &archives[2]), copy(&places[0], &archives[2]))
            .unwrap();
        fs::create_dir(copy(&places[1], &archives[2])).unwrap();
        fs::write(copy(&places[2], &archives[2]), b"wrong digest").unwrap();
        place(&places[3], &archives[2]);
        fs::set_permissions(
            copy(&places[3], &archives[2]),
            fs::Permissions::from_mode(0o0),
        )
        .unwrap();
        assert!(fs::read(copy(&places[3], &archives[2])).is_err());
        let shell = open_places(temp.path(), &places);
        assert_eq!(inline(temp.path()), [false, true, true]);
        preserved(&shell, &before);
        assert_eq!(shell.package_store_messages().len(), 2);
        let line = &shell.package_store_messages()[0];
        assert!(line.contains("cannot be unpacked:"));
        assert!(line.ends_with("no archive can move it; keep the store as it is and report it"));
        let line = &shell.package_store_messages()[1];
        assert!(line.starts_with(&format!(
            "package {ADDRESS} 3.0.0 ({}) keeps its members in state.json:",
            bytes_digest(&archives[2])
        )));
        let mut remaining = line.as_str();
        for (place, outcome) in places.iter().zip([
            "a symbolic link".to_string(),
            "not a regular file".into(),
            format!("digest found {}", bytes_digest(b"wrong digest")),
            "unreadable:".into(),
            "absent".into(),
        ]) {
            let wanted = format!("{}: {outcome}", copy(place, &archives[2]).display());
            let (_, rest) = remaining
                .split_once(&wanted)
                .expect("candidate outcome missing or out of order");
            remaining = rest;
        }
        assert!(line.ends_with("put the archive, named by its SHA-256 hex, in the package cache or a mirror, and open the store again"));
        fs::set_permissions(
            copy(&places[3], &archives[2]),
            fs::Permissions::from_mode(0o600),
        )
        .unwrap();
        fs::set_permissions(
            copy(&places[3], &archives[0]),
            fs::Permissions::from_mode(0o600),
        )
        .unwrap();
    }
}
