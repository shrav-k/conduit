use std::path::Path;

use refresh_probe::{Form, output_path};

fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let status = match args.first().and_then(|arg| arg.to_str()) {
        Some("run") => run(&args[1..]),
        #[cfg(target_os = "linux")]
        Some("__mount") => linux::mount_mode(&args[1..]),
        #[cfg(target_os = "linux")]
        Some("__observe") => linux::observe_mode(&args[1..]),
        _ => Err("usage: refresh-probe run --codex PATH --out PATH [--form issuer|variable]"),
    };
    match status {
        Ok(code) => std::process::exit(code),
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    }
}

fn run(args: &[std::ffi::OsString]) -> Result<i32, &'static str> {
    let mut codex = None;
    let mut out = None;
    let mut form = None;
    let mut pairs = args.chunks_exact(2);
    for pair in &mut pairs {
        let value = &pair[1];
        match pair[0].to_str() {
            Some("--codex") if codex.is_none() => codex = Some(Path::new(value)),
            Some("--out") if out.is_none() => out = Some(Path::new(value)),
            Some("--form") if form.is_none() => {
                form = Some(match value.to_str() {
                    Some("issuer") => Form::Issuer,
                    Some("variable") => Form::Variable,
                    _ => return Err("unknown sitting form"),
                });
            }
            _ => return Err("unknown or repeated option"),
        }
    }
    if !pairs.remainder().is_empty() {
        return Err("option has no value");
    }
    let out = output_path(out.ok_or("missing --out")?)
        .map_err(|_| "output exists or cannot be resolved")?;
    let client = codex.ok_or("missing --codex")?;
    #[cfg(target_os = "linux")]
    return linux::run(client, &out, form.unwrap_or(Form::Issuer));
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (client, out, form);
        Err("stop rule 1: Linux node required")
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::collections::BTreeSet;
    use std::ffi::{CString, OsString};
    use std::fs::{self, File};
    use std::io;
    use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::{DirBuilderExt, FileTypeExt, PermissionsExt};
    use std::os::unix::process::CommandExt;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use std::time::Duration;

    use refresh_probe::{Form, Scratch, isolated_command, report, sitting, today, write_report};
    use serde_json::{Value, json};

    const PATH: &str = "/usr/bin:/bin";
    const ISOLATION: i32 = 125;
    const MASKS: [&str; 5] = ["/run", "/tmp", "/var/tmp", "/dev/shm", "/mnt"];
    const SOCKETS: [(&str, &str); 4] = [
        ("/run", "zero sockets in runtime directory"),
        ("/tmp", "zero sockets in temporary directory"),
        ("/var/tmp", "zero sockets in persistent temporary directory"),
        ("/dev/shm", "zero sockets in shared memory directory"),
    ];

    struct Context {
        exe: PathBuf,
        probe: PathBuf,
        client: PathBuf,
        home: PathBuf,
        form: Form,
        date: String,
        uid: u32,
        gid: u32,
    }

    impl Context {
        fn new(client: PathBuf, form: Form, date: String) -> Result<Self, &'static str> {
            let exe = std::env::current_exe().map_err(|_| "resolve probe executable")?;
            let probe = exe
                .ancestors()
                .nth(3)
                .ok_or("resolve probe directory")?
                .to_path_buf();
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_default();
            // These calls obtain IDs only.
            let uid = unsafe { libc::getuid() };
            let gid = unsafe { libc::getgid() };
            Ok(Self {
                exe,
                probe,
                client,
                home,
                form,
                date,
                uid,
                gid,
            })
        }

        fn from_args(args: &[OsString]) -> Result<Self, &'static str> {
            let [exe, probe, client, home, form, date, uid, gid] = args else {
                return Err("invalid namespace arguments");
            };
            let form = match form.to_str() {
                Some("issuer") => Form::Issuer,
                Some("variable") => Form::Variable,
                _ => return Err("invalid namespace form"),
            };
            let date = date.to_str().ok_or("invalid namespace date")?.to_owned();
            let uid = uid
                .to_str()
                .and_then(|s| s.parse().ok())
                .ok_or("invalid namespace user ID")?;
            let gid = gid
                .to_str()
                .and_then(|s| s.parse().ok())
                .ok_or("invalid namespace group ID")?;
            Ok(Self {
                exe: PathBuf::from(exe),
                probe: PathBuf::from(probe),
                client: PathBuf::from(client),
                home: PathBuf::from(home),
                form,
                date,
                uid,
                gid,
            })
        }

        fn args(&self, command: &mut Command, mode: &str) {
            command
                .arg(&self.exe)
                .arg(mode)
                .args([&self.exe, &self.probe, &self.client, &self.home])
                .args([
                    self.form.name(),
                    &self.date,
                    &self.uid.to_string(),
                    &self.gid.to_string(),
                ]);
        }

        fn scratch(&self) -> PathBuf {
            self.probe.join("target/scratch")
        }

        fn logs(&self) -> PathBuf {
            self.probe.join("target/runs")
        }
    }

    fn command(program: &str) -> Command {
        let mut command = Command::new(program);
        command
            .env_clear()
            .env("PATH", PATH)
            .stdin(Stdio::null())
            .stderr(Stdio::null());
        command
    }

    fn empty(dir: &Path) -> bool {
        fs::read_dir(dir).is_ok_and(|mut entries| entries.next().is_none())
    }

    fn missing(path: &Path) -> bool {
        matches!(fs::symlink_metadata(path), Err(error) if error.kind() == io::ErrorKind::NotFound)
    }

    fn check(report: &mut Value, section: &str, name: &str, passes: bool) -> bool {
        report[section][name] = json!(passes);
        if !passes && report["stop"].is_null() {
            report["stop"] = json!({"rule": 1, "check": name});
        }
        passes
    }

    fn processes() -> io::Result<BTreeSet<u32>> {
        let output = command("pgrep")
            .args(["-f", "codex[ ]app-server"])
            .output()?;
        match output.status.code() {
            Some(1) => Ok(BTreeSet::new()),
            Some(0) => String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|line| line.parse().map_err(io::Error::other))
                .collect(),
            _ => Err(io::Error::other("pgrep failed")),
        }
    }

    fn node_os() -> bool {
        fs::read_to_string("/etc/os-release").is_ok_and(|release| {
            release.lines().any(|line| line == "ID=ubuntu")
                && release.lines().any(|line| line == "VERSION_ID=\"24.04\"")
        })
    }

    pub fn run(client: &Path, out: &Path, form: Form) -> Result<i32, &'static str> {
        let date = today();
        let mut result = report(form, &date);
        // Only paths supplied for this probe are resolved outside the namespace.
        let resolved = client.canonicalize().ok();
        let context = Context::new(resolved.clone().unwrap_or_default(), form, date)?;
        let scratch = context.scratch();
        if !check(
            &mut result,
            "machine",
            "scratch absent or empty",
            missing(&scratch) || empty(&scratch),
        ) {
            return finish(out, &result);
        }
        let created = match fs::DirBuilder::new().mode(0o700).create(&scratch) {
            Ok(()) => true,
            Err(error) => error.kind() == io::ErrorKind::AlreadyExists,
        };
        if !check(
            &mut result,
            "machine",
            "scratch root created",
            created && empty(&scratch),
        ) {
            return finish(out, &result);
        }
        let wsl = fs::read_to_string("/proc/version")
            .is_ok_and(|version| version.to_ascii_lowercase().contains("microsoft"));
        let resolved_client = resolved.as_ref().is_some_and(|client| {
            fs::metadata(client)
                .is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o100 != 0)
        });
        let before = processes();
        let home_valid = context.home.is_absolute()
            && context.home.parent().is_some()
            && context.uid != 0
            && context.gid != 0;
        for (name, passes) in [
            ("Linux x86_64", std::env::consts::ARCH == "x86_64"),
            ("WSL", wsl),
            ("Ubuntu 24.04", node_os()),
            ("absolute home and non-root IDs", home_valid),
            ("Codex executable resolved", resolved_client),
            ("process baseline read", before.is_ok()),
        ] {
            check(&mut result, "machine", name, passes);
        }
        let before = match before {
            Ok(before) if result["stop"].is_null() => before,
            _ => return finish(out, &result),
        };
        let mut launch = command("unshare");
        launch.args([
            "--user",
            "--map-root-user",
            "--net",
            "--mount",
            "--pid",
            "--fork",
            "--mount-proc",
            "--kill-child",
        ]);
        context.args(&mut launch, "__mount");
        let observed = launch
            .output()
            .map_err(|_| "stop rule 1: could not launch unshare")?;
        if observed.status.success() {
            let inner: Value = serde_json::from_slice(&observed.stdout)
                .map_err(|_| "stop rule 1: missing namespace report")?;
            for key in ["self_test", "codex_version", "stop", "runs"] {
                result[key] = inner["report"][key].clone();
            }
            if let Some(counts) = inner["socket_counts"].as_array() {
                for count in counts.iter().filter_map(Value::as_u64) {
                    println!("{count}");
                }
            }
        }
        let clean_processes = processes().is_ok_and(|after| after.is_subset(&before));
        let clean_scratch = empty(&scratch);
        if !observed.status.success() {
            if !clean_processes || !clean_scratch {
                return Err("stop rule 3: cleanup check after namespace failure");
            }
            return Err(match observed.status.code() {
                Some(ISOLATION) => "stop rule 1: namespace setup failed",
                _ => "stop rule 1: unshare failed before the self-test",
            });
        }
        result["machine"]["no new app-server processes"] = json!(clean_processes);
        result["machine"]["scratch root empty after sitting"] = json!(clean_scratch);
        if !clean_processes || !clean_scratch {
            let failed = if clean_processes {
                "scratch root empty after sitting"
            } else {
                "no new app-server processes"
            };
            result["stop"] = json!({"rule": 3, "check": failed});
        }
        finish(out, &result)
    }

    fn finish(out: &Path, result: &Value) -> Result<i32, &'static str> {
        write_report(out, result).map_err(|_| "write report")?;
        if let Some(stop) = result["stop"]["rule"].as_u64() {
            println!(
                "stop rule {stop}: {}",
                result["stop"]["check"].as_str().unwrap_or("unknown check")
            );
            Ok(1)
        } else {
            println!(
                "sitting complete: {} runs",
                result["runs"].as_array().map_or(0, Vec::len)
            );
            Ok(0)
        }
    }

    fn mount(
        source: Option<&Path>,
        target: &Path,
        filesystem: Option<&str>,
        flags: libc::c_ulong,
    ) -> io::Result<()> {
        let source = source
            .map(|path| CString::new(path.as_os_str().as_encoded_bytes()))
            .transpose()?;
        let target = CString::new(target.as_os_str().as_encoded_bytes())?;
        let filesystem = filesystem.map(CString::new).transpose()?;
        // The C strings are live for the duration of mount(2); a missing argument is null.
        let result = unsafe {
            libc::mount(
                source.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()),
                target.as_ptr(),
                filesystem.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()),
                flags,
                std::ptr::null(),
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    fn mask(path: &Path) -> io::Result<()> {
        mount(None, path, Some("tmpfs"), libc::MS_NODEV | libc::MS_NOSUID)
    }

    fn id_map(path: &str, outer: u32) -> bool {
        fs::read_to_string(path)
            .is_ok_and(|map| map.split_whitespace().eq(["0", &outer.to_string(), "1"]))
    }

    fn apply_masks(context: &Context) -> io::Result<()> {
        let scope = File::open(&context.probe)?;
        let source = PathBuf::from(format!("/proc/self/fd/{}", scope.as_raw_fd()));
        std::env::set_current_dir("/")?;
        if context.probe.starts_with(&context.home) {
            mask(&context.home)?;
            fs::create_dir_all(&context.probe)?;
            mount(Some(&source), &context.probe, None, libc::MS_BIND)?;
        } else {
            mask(&context.home)?;
        }
        for path in MASKS.map(Path::new) {
            mask(path)?;
        }
        let etc = Path::new("/etc/codex");
        if !missing(etc) {
            mask(etc)?;
        }
        std::env::set_current_dir(&context.probe)?;
        Ok(())
    }

    pub fn mount_mode(args: &[OsString]) -> Result<i32, &'static str> {
        let context = Context::from_args(args)?;
        if std::process::id() != 1
            || context.uid == 0
            || !id_map("/proc/self/uid_map", context.uid)
            || !id_map("/proc/self/gid_map", context.gid)
        {
            return Ok(ISOLATION);
        }
        if apply_masks(&context).is_err()
            || !command("ip")
                .args(["link", "set", "lo", "up"])
                .status()
                .is_ok_and(|s| s.success())
        {
            return Ok(ISOLATION);
        }
        // Prevent a descendant's exec from granting privileges even if a binary has file caps.
        if unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) } != 0 {
            return Ok(ISOLATION);
        }
        let mut launch = command("unshare");
        launch
            .arg("--user")
            .arg(format!("--map-user={}", context.uid))
            .arg(format!("--map-group={}", context.gid));
        context.args(&mut launch, "__observe");
        let _ = launch.exec();
        Ok(ISOLATION)
    }

    fn self_test(context: &Context, result: &mut Value) -> Vec<Option<usize>> {
        let interfaces = command("ip").args(["-o", "link", "show"]).output();
        let loopback_only = interfaces.is_ok_and(|out| {
            out.status.success()
                && String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .map(|line| line.split(": ").nth(1))
                    .eq([Some("lo")])
        });
        let proc_local = std::process::id() == 1 && local_proc();
        if !check(
            result,
            "self_test",
            "only namespace processes in process table",
            proc_local,
        ) || !check(
            result,
            "self_test",
            "only loopback interface",
            loopback_only,
        ) {
            return Vec::new();
        }
        let check_target = SocketAddr::from(([192, 0, 2, 1], 443));
        let connect_fails =
            TcpStream::connect_timeout(&check_target, Duration::from_secs(2)).is_err();
        let resolve_fails = ("refresh.invalid", 443).to_socket_addrs().is_err();
        let loopback_works = (|| -> io::Result<()> {
            let listen = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
            listen.set_nonblocking(true)?;
            let _client =
                TcpStream::connect_timeout(&listen.local_addr()?, Duration::from_secs(2))?;
            listen.accept()?;
            Ok(())
        })()
        .is_ok();
        // A namespace UID is meaningful only with the corresponding GID and capability checks.
        let ids = unsafe {
            libc::getuid() == context.uid
                && libc::geteuid() == context.uid
                && libc::getgid() == context.gid
                && libc::getegid() == context.gid
        };
        let no_capabilities = fs::read_to_string("/proc/self/status").is_ok_and(|status| {
            ["CapInh:", "CapPrm:", "CapEff:", "CapAmb:"]
                .iter()
                .all(|field| {
                    status.lines().any(|line| {
                        line.strip_prefix(field)
                            .is_some_and(|value| u64::from_str_radix(value.trim(), 16) == Ok(0))
                    })
                })
        });
        for (name, passes) in [
            ("external TCP fails within 2 s", connect_fails),
            ("reserved host lookup fails", resolve_fails),
            ("loopback accepts TCP", loopback_works),
            ("own user and group IDs", ids),
            ("no capabilities", no_capabilities),
            (
                "scratch root outside Windows mount directory",
                context
                    .scratch()
                    .canonicalize()
                    .is_ok_and(|p| !p.starts_with("/mnt")),
            ),
            (
                "real home client directory absent",
                missing(&context.home.join(".codex")),
            ),
            (
                "system client directory absent",
                missing(Path::new("/etc/codex")),
            ),
            ("Windows mount directory empty", empty(Path::new("/mnt"))),
            ("runtime directory empty", empty(Path::new("/run"))),
            (
                "working directory is the probe",
                std::env::current_dir().is_ok_and(|dir| dir == context.probe),
            ),
        ] {
            check(result, "self_test", name, passes);
        }
        SOCKETS
            .iter()
            .map(|(path, label)| {
                let count = socket_count(Path::new(path)).ok();
                check(result, "self_test", label, count == Some(0));
                count
            })
            .collect()
    }

    fn local_proc() -> bool {
        let Ok(entries) = fs::read_dir("/proc") else {
            return false;
        };
        for entry in entries {
            let Ok(entry) = entry else {
                return false;
            };
            let name = entry.file_name();
            let Some(pid) = name.to_str().filter(|entry| entry.parse::<u32>().is_ok()) else {
                continue;
            };
            if !fs::read_to_string(entry.path().join("status")).is_ok_and(|status| {
                status.lines().any(|line| {
                    line.strip_prefix("NSpid:")
                        .is_some_and(|ids| ids.split_whitespace().eq([pid]))
                })
            }) {
                return false;
            }
        }
        fs::read_to_string("/proc/1/status").is_ok_and(|status| {
            status.lines().any(|line| {
                line.strip_prefix("NSpid:")
                    .is_some_and(|value| value.trim() == "1")
            })
        })
    }

    fn socket_count(dir: &Path) -> io::Result<usize> {
        let mut count = 0;
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            if kind.is_dir() {
                count += socket_count(&entry.path())?;
            } else if kind.is_socket() {
                count += 1;
            }
        }
        Ok(count)
    }

    fn version(context: &Context) -> io::Result<Option<String>> {
        let scratch = Scratch::new(&context.scratch(), "version")?;
        let home = scratch.path().join("home");
        let codex_home = scratch.path().join("codex-home");
        for path in [&home, &codex_home] {
            fs::DirBuilder::new().mode(0o700).create(path)?;
        }
        let output = isolated_command(&context.client, scratch.path(), &home, &codex_home)
            .arg("--version")
            .stdin(Stdio::null())
            .output();
        scratch.remove()?;
        let output = output?;
        if !output.status.success() {
            return Ok(None);
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(version_token(&stdout)
            .or_else(|| version_token(&stderr))
            .map(str::to_owned))
    }

    fn version_token(text: &str) -> Option<&str> {
        text.split_whitespace().find(|word| {
            let mut parts = word.splitn(3, '.');
            let [Some(major), Some(minor), Some(rest)] = [parts.next(), parts.next(), parts.next()]
            else {
                return false;
            };
            let digits = |part: &str| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit());
            let suffix = rest.trim_start_matches(|ch: char| ch.is_ascii_digit());
            let valid_suffix = match suffix.as_bytes() {
                [] => true,
                [b'-' | b'+' | b'.', tail @ ..] if !tail.is_empty() => tail
                    .iter()
                    .all(|b| b.is_ascii_alphanumeric() || b".-".contains(b)),
                _ => false,
            };
            digits(major) && digits(minor) && rest.len() > suffix.len() && valid_suffix
        })
    }

    pub fn observe_mode(args: &[OsString]) -> Result<i32, &'static str> {
        let context = Context::from_args(args)?;
        let mut result = report(context.form, &context.date);
        let socket_counts = self_test(&context, &mut result);
        if result["stop"].is_null() {
            let version = version(&context).ok().flatten();
            let pinned = version.as_deref() == Some("0.167.1");
            result["codex_version"] = json!(version);
            if check(&mut result, "self_test", "Codex version is 0.167.1", pinned)
                && check(
                    &mut result,
                    "self_test",
                    "run log directory ready",
                    fs::create_dir_all(context.logs()).is_ok(),
                )
            {
                sitting(
                    &mut result,
                    &context.client,
                    &context.scratch(),
                    &context.logs(),
                    context.form,
                    Duration::from_secs(10),
                    None,
                );
            }
        }
        println!(
            "{}",
            json!({"report": result, "socket_counts": socket_counts})
        );
        Ok(0)
    }

    #[cfg(test)]
    mod tests {
        use super::version_token;

        #[test]
        fn first_version_shaped_token_may_have_a_suffix() {
            for first in ["0.167.1-dev", "0.167.1+build.2", "0.167.1.2"] {
                let text = format!("codex-cli {first} 0.167.1");
                assert_eq!(version_token(&text), Some(first));
            }
            assert_eq!(version_token("codex-cli 0.167.1- 0.167.1"), Some("0.167.1"));
            assert_eq!(
                version_token("codex-cli 1234567.167.1"),
                Some("1234567.167.1")
            );
        }
    }
}
