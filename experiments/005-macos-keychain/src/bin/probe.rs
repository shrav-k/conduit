//! Experiment 005: what a macOS keychain secret store faces when an engine reads a secret with
//! no person present (probe E10). See README.md.
//!
//! Safety, which the code enforces where it can:
//! - The keychain search list and the default keychain are recorded before anything else and
//!   compared byte for byte after each step that writes to a keychain and at the end.
//! - The only keychains used are two throwaway files created with `SecKeychainCreate` under
//!   `target/scratch/`, each with its own generated password. Every `security` command names
//!   one of them explicitly and is refused if it does not lie under the run's scratch directory.
//! - This process and both readers disable keychain user interaction before any keychain call.
//! - Every child process runs with stdin closed and a timeout.
//! - Each throwaway keychain is locked and its file removed at the end, and on early exit.
//!
//! The two throwaway keychains differ in one thing: the database format. Apple's
//! `CommonBlob::getCurrentVersionForDb` creates a keychain whose path contains
//! `/Library/Keychains` in the newer format (`version_partition`), which the login keychain
//! uses and which adds partition IDs to item access lists; any other path gets the older
//! format. So `legacy` lies directly in the run directory and `partition` lies in
//! `<run directory>/Library/Keychains/`. Neither is under the home directory.

use macos_keychain_probe::{self as kp, RunOutput, SERVICE};
use security_framework::os::macos::keychain::{CreateOptions, KeychainSettings, SecKeychain};
use serde_json::{Map, Value, json};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::from_secs(30);
const BUILD_TIMEOUT: Duration = Duration::from_secs(600);
const SECURITY: &str = "/usr/bin/security";
const BENCH_READS: usize = 20;
const PROCESS_READS: usize = 5;

struct Reader {
    path: PathBuf,
    desc: String,
}

#[derive(Clone)]
struct Item {
    account: String,
    access: String,
    sha256: String,
}

struct Items {
    listed: Item,
    any_app: Item,
    api_created: Item,
    listed_partition: Item,
}

struct Probe {
    exp_dir: PathBuf,
    scratch: PathBuf,
    home: String,
    before: (Vec<u8>, Vec<u8>),
    steps: Vec<Value>,
    cases: Vec<Value>,
    extra: Map<String, Value>,
    counter: usize,
    run_dir: PathBuf,
}

fn search_state() -> (RunOutput, RunOutput) {
    (
        kp::run_with_timeout(Command::new(SECURITY).args(["list-keychains", "-d", "user"]), TIMEOUT),
        kp::run_with_timeout(Command::new(SECURITY).args(["default-keychain", "-d", "user"]), TIMEOUT),
    )
}

fn list_dir(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    names
}

impl Probe {
    fn s(&self, text: &str) -> String {
        kp::sanitize(text, self.exp_dir.to_str().unwrap_or(""), &self.home)
    }

    fn sb(&self, bytes: &[u8]) -> String {
        self.s(&String::from_utf8_lossy(bytes))
    }

    fn step(&mut self, name: &str, detail: Value) {
        println!("step: {name}");
        self.steps.push(json!({"step": name, "detail": detail}));
    }

    /// Runs a command under a timeout and logs it. Arguments listed in `secret` are logged as
    /// `<generated>`, and so is any occurrence of them in the output.
    fn run(&mut self, label: &str, cmd: &mut Command, timeout: Duration, secret: &[&str]) -> RunOutput {
        let argv: Vec<String> = std::iter::once(cmd.get_program())
            .chain(cmd.get_args())
            .map(|a| {
                let a = a.to_string_lossy().to_string();
                if secret.contains(&a.as_str()) { "<generated>".to_string() } else { self.s(&a) }
            })
            .collect();
        let out = kp::run_with_timeout(cmd, timeout);
        let mask = |text: String| secret.iter().fold(text, |t, s| t.replace(s, "<generated>"));
        let detail = json!({
            "argv": argv,
            "exit_code": out.exit_code,
            "timed_out": out.timed_out,
            "elapsed_ms": out.elapsed.as_secs_f64() * 1000.0,
            "stdout": mask(self.sb(&out.stdout)),
            "stderr": mask(self.sb(&out.stderr)),
        });
        self.step(label, detail);
        out
    }

    /// Stops the run if the search list or the default keychain differs from the start.
    fn check_search_list(&mut self, when: &str) -> Result<(), String> {
        let (list, default) = search_state();
        let same = list.ok() && default.ok() && list.stdout == self.before.0 && default.stdout == self.before.1;
        self.step(&format!("search list check: {when}"), json!({"unchanged": same}));
        if same { Ok(()) } else { Err(format!("keychain search list or default changed ({when})")) }
    }

    fn assert_throwaway(&self, keychain: &Path) -> Result<(), String> {
        let ok = keychain.is_absolute()
            && keychain.starts_with(&self.run_dir)
            && self.run_dir.starts_with(&self.scratch)
            && self.scratch.starts_with(&self.exp_dir);
        if ok {
            Ok(())
        } else {
            Err(format!(
                "refusing a keychain outside the scratch directory: {}",
                self.s(&keychain.display().to_string())
            ))
        }
    }

    /// Runs `security <args> <keychain>`, always naming the throwaway keychain last.
    fn security(&mut self, label: &str, args: &[&str], secret: &[&str], keychain: &Path) -> Result<RunOutput, String> {
        self.assert_throwaway(keychain)?;
        let mut cmd = Command::new(SECURITY);
        cmd.args(args).arg(keychain);
        let out = self.run(label, &mut cmd, TIMEOUT, secret);
        if out.ok() { Ok(out) } else { Err(format!("{label} failed")) }
    }

    fn cargo_build(&mut self, tag: &str, bins: &[&str]) -> Result<(), String> {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let mut cmd = Command::new(cargo);
        cmd.current_dir(&self.exp_dir).args(["build", "--release", "--locked"]);
        for b in bins {
            cmd.args(["--bin", b]);
        }
        cmd.env("CARGO_TARGET_DIR", self.scratch.join("build")).env("PROBE_READER_TAG", tag);
        let out = self.run(&format!("cargo build, reader tag {tag}"), &mut cmd, BUILD_TIMEOUT, &[]);
        if out.ok() { Ok(()) } else { Err(format!("cargo build ({tag}) failed")) }
    }

    fn codesign(&mut self, label: &str, path: &Path) -> Value {
        let out = self.run(
            &format!("codesign -dv: {label}"),
            Command::new("/usr/bin/codesign").args(["-dv", "--verbose=4"]).arg(path),
            TIMEOUT,
            &[],
        );
        let text = self.sb(&out.stderr);
        let field = |name: &str| {
            text.lines()
                .find_map(|l| l.strip_prefix(&format!("{name}=")))
                .map(str::to_string)
        };
        let meta = std::fs::metadata(path).ok();
        json!({
            "binary": label,
            "path": self.s(&path.display().to_string()),
            "inode": meta.as_ref().map(std::os::unix::fs::MetadataExt::ino),
            "identifier": field("Identifier"),
            "format": field("Format"),
            "code_directory": text.lines().find(|l| l.starts_with("CodeDirectory")).map(str::to_string),
            "signature": field("Signature"),
            "team_identifier": field("TeamIdentifier"),
            "cdhash": field("CDHash"),
            "raw": text,
        })
    }

    /// Runs a reader with stdin and stdout closed, stderr to a file and in a new session, so it
    /// has no controlling terminal, and returns what it wrote.
    fn spawn_reader(&mut self, reader: &Reader, args: &[String]) -> (Value, Option<i32>, bool, Duration, String) {
        self.counter += 1;
        let n = self.counter;
        let out_path = self.run_dir.join(format!("reader-{n}.json"));
        let err_path = self.run_dir.join(format!("reader-{n}.stderr"));
        let mut full: Vec<String> = args.to_vec();
        full.push(out_path.display().to_string());
        let stderr = match std::fs::File::create(&err_path) {
            Ok(f) => Stdio::from(f),
            Err(_) => Stdio::null(),
        };
        let mut cmd = Command::new(&reader.path);
        cmd.args(&full).stdin(Stdio::null()).stdout(Stdio::null()).stderr(stderr);
        unsafe {
            cmd.pre_exec(|| {
                if libc::setsid() < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let start = Instant::now();
        let (code, timed_out) = match cmd.spawn() {
            Ok(mut child) => kp::wait_with_timeout(&mut child, TIMEOUT),
            Err(e) => {
                let _ = std::fs::write(&err_path, format!("spawn failed: {e}"));
                (None, false)
            }
        };
        let wall = start.elapsed();
        let result = std::fs::read_to_string(&out_path)
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or(Value::Null);
        let err_text = std::fs::read_to_string(&err_path).unwrap_or_default();
        (result, code, timed_out, wall, self.s(&err_text))
    }

    fn read_case(&mut self, id: &str, question: &str, reader: &Reader, item: &Item, tk: &Throwaway) -> Value {
        let args = vec!["read".to_string(), tk.actual.display().to_string(), item.account.clone()];
        let (result, code, timed_out, wall, stderr) = self.spawn_reader(reader, &args);
        let read = &result["read"];
        let ok = read["ok"].as_bool() == Some(true);
        let outcome = if timed_out {
            "timed out (killed after 30 s)"
        } else if ok {
            "read"
        } else if read.is_null() {
            "reader failed before reading"
        } else {
            "refused"
        };
        let case = json!({
            "keychain": tk.label,
            "case": id,
            "question": question,
            "reader": reader.desc,
            "reader_build_tag": result["build_tag"],
            "item": item.account,
            "item_access": item.access,
            "outcome": outcome,
            "status": read["status"],
            "status_name": read["status_name"],
            "message": read["message"],
            "value_matches": read["value_sha256"].as_str().map(|h| h == item.sha256),
            "read_us": read["elapsed_us"],
            "process_wall_ms": wall.as_secs_f64() * 1000.0,
            "process_exit": code,
            "keychain_before_read": result["keychain_before"],
            "user_interaction_allowed_in_reader": result["user_interaction_allowed"],
            "reader_context": result["context"],
            "reader_error": result["error"],
            "reader_stderr": stderr,
        });
        println!(
            "case {:<9} {id:<4} {:<31} {:<17} -> {outcome} {} {}",
            tk.label,
            reader.desc,
            item.account,
            case["status"],
            case["status_name"].as_str().unwrap_or("")
        );
        self.cases.push(case.clone());
        case
    }
}

/// A throwaway keychain. Dropping it locks it and removes its file.
struct Throwaway {
    label: &'static str,
    kc: Option<SecKeychain>,
    password: String,
    requested: PathBuf,
    actual: PathBuf,
    run_dir: PathBuf,
    cleanup: Option<Value>,
}

impl Throwaway {
    fn create(p: &mut Probe, label: &'static str, path: &Path) -> Result<Self, String> {
        p.assert_throwaway(path)?;
        let parent = path.parent().ok_or("no parent directory")?.to_path_buf();
        std::fs::create_dir_all(&parent).map_err(|e| format!("create {label} directory: {e}"))?;
        let plain = PathBuf::from(path.display().to_string().trim_end_matches("-db"));
        let db = PathBuf::from(format!("{}-db", plain.display()));
        if plain.exists() || db.exists() {
            return Err(format!("a keychain file already exists at the {label} path"));
        }
        let password = kp::random_hex(32);
        let start = Instant::now();
        let mut kc = CreateOptions::new()
            .password(&password)
            .create(path)
            .map_err(|e| format!("SecKeychainCreate ({label}) failed: {} {}", e.code(), kp::status_name(e.code())))?;
        let created_ms = start.elapsed().as_secs_f64() * 1000.0;
        let actual = kp::keychain_path(&kc).map(PathBuf::from).unwrap_or_else(|_| path.to_path_buf());
        let settings = kc.set_settings(&KeychainSettings::new()).map_err(|e| e.code());
        let t = Throwaway {
            label,
            kc: Some(kc),
            password,
            requested: path.to_path_buf(),
            actual: actual.clone(),
            run_dir: p.run_dir.clone(),
            cleanup: None,
        };
        p.step(
            &format!("create throwaway keychain '{label}' (SecKeychainCreate, generated password)"),
            json!({
                "requested_path": p.s(&path.display().to_string()),
                "path_reported_by_framework": p.s(&actual.display().to_string()),
                "files_in_its_directory": list_dir(&parent),
                "settings_no_auto_lock": settings.is_ok(),
                "status": kp::keychain_unlocked(t.kc()),
                "elapsed_ms": created_ms,
            }),
        );
        p.assert_throwaway(&actual)?;
        Ok(t)
    }

    fn kc(&self) -> &SecKeychain {
        self.kc.as_ref().expect("keychain open")
    }

    fn clean(&mut self) -> Value {
        if let Some(done) = &self.cleanup {
            return done.clone();
        }
        let lock = self.kc.as_ref().map(kp::lock_keychain);
        self.kc = None;
        let plain = PathBuf::from(self.requested.display().to_string().trim_end_matches("-db"));
        let candidates = [
            self.actual.clone(),
            self.requested.clone(),
            plain.clone(),
            PathBuf::from(format!("{}-db", plain.display())),
        ];
        let mut removed = Vec::new();
        for c in candidates {
            if c.starts_with(&self.run_dir) && c.exists() && std::fs::remove_file(&c).is_ok() {
                removed.push(c.file_name().map(|n| n.to_string_lossy().to_string()));
            }
        }
        let remaining: Vec<String> = candidates_exist(&self.actual, &plain);
        let v = json!({
            "keychain": self.label,
            "lock_status": lock,
            "removed": removed,
            "keychain_files_remaining": remaining,
        });
        self.cleanup = Some(v.clone());
        v
    }
}

fn candidates_exist(actual: &Path, plain: &Path) -> Vec<String> {
    [actual.to_path_buf(), plain.to_path_buf(), PathBuf::from(format!("{}-db", plain.display()))]
        .into_iter()
        .filter(|c| c.exists())
        .map(|c| c.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default())
        .collect()
}

impl Drop for Throwaway {
    fn drop(&mut self) {
        self.clean();
    }
}

fn stats_us(values: &[u64]) -> Value {
    if values.is_empty() {
        return Value::Null;
    }
    let mut v = values.to_vec();
    v.sort_unstable();
    json!({
        "n": v.len(),
        "min_us": v[0],
        "median_us": v[v.len() / 2],
        "max_us": v[v.len() - 1],
    })
}

fn environment(p: &mut Probe) -> Value {
    let sw = p.run("sw_vers", &mut Command::new("/usr/bin/sw_vers"), TIMEOUT, &[]);
    let arch = p.run("uname -m", Command::new("/usr/bin/uname").arg("-m"), TIMEOUT, &[]);
    let cpu = p.run(
        "cpu",
        Command::new("/usr/sbin/sysctl").args(["-n", "machdep.cpu.brand_string"]),
        TIMEOUT,
        &[],
    );
    let exp = p.exp_dir.clone();
    let rustc = p.run("rustc -V", Command::new("rustc").arg("-V").current_dir(&exp), TIMEOUT, &[]);
    let manager = p.run(
        "launchctl managername",
        Command::new("/bin/launchctl").arg("managername"),
        TIMEOUT,
        &[],
    );
    let trim = |o: &RunOutput| String::from_utf8_lossy(&o.stdout).trim().to_string();
    json!({
        "sw_vers": trim(&sw),
        "machine": trim(&arch),
        "cpu": trim(&cpu),
        "rustc": trim(&rustc),
        "security_framework_crate": "3.7.0",
        "launchctl_managername": trim(&manager),
        "probe_process": kp::process_context(),
    })
}

/// Adds the four items to one throwaway keychain and records their access control lists.
fn setup_items(p: &mut Probe, tk: &Throwaway, reader_v1: &Reader, sig_v1: &Value) -> Result<Items, String> {
    let label = tk.label;
    let kc = tk.actual.clone();
    let kcs = kc.display().to_string();
    let reader_path = reader_v1.path.display().to_string();

    // api-created: added by reader v1 itself through the API; the creator is trusted by default.
    let (created, ..) = p.spawn_reader(reader_v1, &["create".into(), kcs.clone(), "api-created".into()]);
    p.step(
        &format!("{label}: item api-created, added by reader v1 through SecKeychainAddGenericPassword"),
        json!({
            "create": created["create"],
            "reader_context": created["context"],
            "user_interaction_allowed_in_reader": created["user_interaction_allowed"],
        }),
    );
    if created["create"]["ok"].as_bool() != Some(true) {
        return Err(format!("{label}: reader v1 could not create its item"));
    }
    let api_created = Item {
        account: "api-created".into(),
        access: "created by reader v1 through the API; default access".into(),
        sha256: created["create"]["value_sha256"].as_str().unwrap_or("").to_string(),
    };

    // listed: added with the security tool, reader v1 on its access list (-T).
    let val = kp::random_hex(24);
    p.security(
        &format!("{label}: item listed, security add-generic-password -T <reader v1>"),
        &["add-generic-password", "-s", SERVICE, "-a", "listed", "-w", &val, "-T", &reader_path],
        &[&val],
        &kc,
    )?;
    let listed = Item {
        account: "listed".into(),
        access: "security add-generic-password -T reader v1".into(),
        sha256: kp::sha256_hex(val.as_bytes()),
    };

    // any-app: added with the security tool for any application (-A).
    let val = kp::random_hex(24);
    p.security(
        &format!("{label}: item any-app, security add-generic-password -A"),
        &["add-generic-password", "-s", SERVICE, "-a", "any-app", "-w", &val, "-A"],
        &[&val],
        &kc,
    )?;
    let any_app = Item {
        account: "any-app".into(),
        access: "security add-generic-password -A (any application)".into(),
        sha256: kp::sha256_hex(val.as_bytes()),
    };

    // listed-partition: as listed, then its partition list set, with the keychain's password,
    // to the security tool's partition and reader v1's, as found on api-created (or, if that
    // item has none, reader v1's code-directory hash).
    let acl_api = kp::acl_dump(tk.kc(), "api-created");
    let found = kp::partition_ids(&acl_api).into_iter().find(|id| !id.starts_with("apple"));
    let reader_partition = found
        .clone()
        .or_else(|| sig_v1["cdhash"].as_str().map(|h| format!("cdhash:{}", h.to_lowercase())));
    let partition_list = match &reader_partition {
        Some(id) => format!("apple-tool:,{id}"),
        None => "apple-tool:".to_string(),
    };
    let val = kp::random_hex(24);
    p.security(
        &format!("{label}: item listed-partition, security add-generic-password -T <reader v1>"),
        &["add-generic-password", "-s", SERVICE, "-a", "listed-partition", "-w", &val, "-T", &reader_path],
        &[&val],
        &kc,
    )?;
    let password = tk.password.clone();
    p.security(
        &format!(
            "{label}: item listed-partition, security set-generic-password-partition-list -S apple-tool:,<reader v1> -k <keychain password>"
        ),
        &[
            "set-generic-password-partition-list",
            "-s",
            SERVICE,
            "-a",
            "listed-partition",
            "-S",
            &partition_list,
            "-k",
            &password,
        ],
        &[&password],
        &kc,
    )?;
    let listed_partition = Item {
        account: "listed-partition".into(),
        access: "security add-generic-password -T reader v1, then partition list set to apple-tool: and reader v1's".into(),
        sha256: kp::sha256_hex(val.as_bytes()),
    };

    let acls = json!({
        "api-created": acl_api,
        "listed": kp::acl_dump(tk.kc(), "listed"),
        "any-app": kp::acl_dump(tk.kc(), "any-app"),
        "listed-partition": kp::acl_dump(tk.kc(), "listed-partition"),
        "reader_v1_partition_id_found_on_api_created": found,
        "partition_list_set_on_listed_partition": partition_list,
    });
    let acls = serde_json::from_str::<Value>(&p.s(&acls.to_string())).unwrap_or(acls);
    let mut all = p.extra.remove("access_control_lists").unwrap_or_else(|| json!({}));
    all[label] = acls;
    p.extra.insert("access_control_lists".into(), all);

    Ok(Items { listed, any_app, api_created, listed_partition })
}

/// Questions 1, 2, 3 and 6 on an unlocked keychain.
fn unlocked_cases(p: &mut Probe, tk: &Throwaway, it: &Items, reader_v1: &Reader, other: &Reader) {
    let q1 = "1: a binary on the access list reads, unlocked, unattended";
    p.read_case("1a", q1, reader_v1, &it.listed, tk);
    p.read_case("1b", q1, reader_v1, &it.listed_partition, tk);
    p.read_case("1c", q1, reader_v1, &it.api_created, tk);
    let q2 = "2: a binary not on the access list reads";
    p.read_case("2a", q2, other, &it.listed, tk);
    p.read_case("2b", q2, other, &it.listed_partition, tk);
    p.read_case("2c", q2, other, &it.api_created, tk);
    let q3 = "3: an item open to any application";
    p.read_case("3a", q3, other, &it.any_app, tk);
    p.read_case("3b", q3, reader_v1, &it.any_app, tk);

    // Question 6: timing, on the first item reader v1 read above.
    let all = [&it.listed, &it.listed_partition, &it.api_created, &it.any_app];
    let readable = p
        .cases
        .iter()
        .filter(|c| c["keychain"] == json!(tk.label))
        .filter(|c| c["reader"] == json!(reader_v1.desc) && c["outcome"] == json!("read"))
        .filter_map(|c| c["item"].as_str().map(str::to_string))
        .next();
    let timing = match readable.and_then(|acc| all.into_iter().find(|i| i.account == acc).cloned()) {
        Some(item) => {
            let (bench, code, timed_out, wall, _) = p.spawn_reader(
                reader_v1,
                &["bench".into(), tk.actual.display().to_string(), item.account.clone(), BENCH_READS.to_string()],
            );
            let reads = bench["reads"].as_array().cloned().unwrap_or_default();
            let us: Vec<u64> = reads.iter().filter_map(|r| r["elapsed_us"].as_u64()).collect();
            let all_ok = !reads.is_empty()
                && reads.iter().all(|r| r["value_sha256"].as_str() == Some(item.sha256.as_str()));
            let mut per_process = Vec::new();
            for i in 0..PROCESS_READS {
                let c = p.read_case(&format!("6.{}", i + 1), "6: timing, one read per process", reader_v1, &item, tk);
                per_process.push(json!({
                    "read_us": c["read_us"],
                    "process_wall_ms": c["process_wall_ms"],
                    "outcome": c["outcome"],
                }));
            }
            // Every refused read in this keychain so far: questions 1 to 3, which in `partition`
            // includes case 1a. The key below is older than that and says "2 and 3"; it is kept
            // so that the recorded results files match this code.
            let refused: Vec<u64> = p
                .cases
                .iter()
                .filter(|c| c["keychain"] == json!(tk.label) && c["outcome"] == json!("refused"))
                .filter_map(|c| c["read_us"].as_u64())
                .collect();
            json!({
                "item": item.account,
                "in_one_process": {
                    "reads": us.len(),
                    "all_read_and_matched": all_ok,
                    "first_read_us": us.first(),
                    "later_reads": stats_us(us.get(1..).unwrap_or(&[])),
                    "process_exit": code,
                    "timed_out": timed_out,
                    "process_wall_ms": wall.as_secs_f64() * 1000.0,
                },
                "one_read_per_process": per_process,
                "refused_reads_in_questions_2_and_3": stats_us(&refused),
            })
        }
        None => json!("not measured: reader v1 read no item successfully"),
    };
    let mut all_timing = p.extra.remove("timing").unwrap_or_else(|| json!({}));
    all_timing[tk.label] = timing;
    p.extra.insert("timing".into(), all_timing);
}

/// Question 5: lock the keychain, read, unlock it with its password, read again.
fn lock_cases(p: &mut Probe, tk: &mut Throwaway, it: &Items, reader_v1: &Reader) -> Result<(), String> {
    let label = tk.label;
    let lock = kp::lock_keychain(tk.kc());
    let status = kp::keychain_unlocked(tk.kc());
    p.step(&format!("{label}: SecKeychainLock"), json!({"status": lock, "keychain": status}));
    let q5 = "5: locked keychain, then unlocked through the API";
    p.read_case("5a", q5, reader_v1, &it.api_created, tk);
    p.read_case("5b", q5, reader_v1, &it.listed_partition, tk);
    p.read_case("5c", q5, reader_v1, &it.any_app, tk);
    let password = tk.password.clone();
    let unlock = tk.kc.as_mut().expect("keychain open").unlock(Some(&password)).map_err(|e| e.code());
    let status = kp::keychain_unlocked(tk.kc());
    p.step(
        &format!("{label}: SecKeychainUnlock with its generated password"),
        json!({"result": format!("{unlock:?}"), "keychain": status}),
    );
    p.check_search_list(&format!("{label}: after lock and unlock"))?;
    p.read_case("5d", q5, reader_v1, &it.api_created, tk);
    p.read_case("5e", q5, reader_v1, &it.listed_partition, tk);
    p.read_case("5f", q5, reader_v1, &it.any_app, tk);
    Ok(())
}

/// Question 4: reads by the rebuilt reader.
fn rebuild_cases(p: &mut Probe, tk: &Throwaway, it: &Items, reader_v2: &Reader) {
    let q4 = "4: the trusted binary rebuilt with changed code, same path";
    p.read_case("4a", q4, reader_v2, &it.listed, tk);
    p.read_case("4b", q4, reader_v2, &it.listed_partition, tk);
    p.read_case("4c", q4, reader_v2, &it.api_created, tk);
    p.read_case("4d", q4, reader_v2, &it.any_app, tk);
}

fn phases(p: &mut Probe, kcs: &mut [Throwaway], reader_v1: &Reader, other: &Reader, sig_v1: Value, sig_other: Value) -> Result<(), String> {
    let mut items = Vec::new();
    for tk in kcs.iter() {
        items.push(setup_items(p, tk, reader_v1, &sig_v1)?);
    }
    p.check_search_list("after adding items")?;
    for (tk, it) in kcs.iter_mut().zip(&items) {
        unlocked_cases(p, tk, it, reader_v1, other);
        lock_cases(p, tk, it, reader_v1)?;
    }

    // Rebuild reader at the same path with a changed constant.
    p.cargo_build("v2", &["reader"])?;
    let reader_v2 = Reader { path: reader_v1.path.clone(), desc: "reader v2 (rebuilt, same path)".into() };
    let sig_v2 = p.codesign("reader v2", &reader_v2.path);
    let changed = sig_v1["cdhash"] != sig_v2["cdhash"];
    p.extra.insert(
        "signatures".into(),
        json!({
            "reader_v1": sig_v1,
            "reader_v2": sig_v2,
            "other_reader": sig_other,
            "cdhash_changed_by_rebuild": changed,
        }),
    );
    for (tk, it) in kcs.iter().zip(&items) {
        rebuild_cases(p, tk, it, &reader_v2);
    }
    Ok(())
}

fn body(p: &mut Probe) -> Result<(), String> {
    let env = environment(p);
    p.extra.insert("environment".into(), env);

    let _ui = SecKeychain::disable_user_interaction()
        .map_err(|e| format!("could not disable user interaction: {}", e.code()))?;
    let ui = SecKeychain::user_interaction_allowed().ok();
    p.step("disable keychain user interaction in the probe", json!({"user_interaction_allowed": ui}));
    if ui != Some(false) {
        return Err("user interaction still allowed in the probe".into());
    }

    // Build both readers as cargo produces them, into the scratch directory.
    p.cargo_build("v1", &["reader", "other-reader"])?;
    let build = p.scratch.join("build/release");
    let reader_v1 = Reader { path: build.join("reader"), desc: "reader v1 (trusted binary)".into() };
    let other = Reader { path: build.join("other-reader"), desc: "other-reader (never listed)".into() };
    let sig_v1 = p.codesign("reader v1", &reader_v1.path);
    let sig_other = p.codesign("other-reader", &other.path);

    std::fs::create_dir_all(&p.run_dir).map_err(|e| format!("create run dir: {e}"))?;
    let mut kcs: Vec<Throwaway> = Vec::new();
    let legacy_path = p.run_dir.join("e10-legacy.keychain");
    let created = Throwaway::create(p, "legacy", &legacy_path);
    let result = match created {
        Ok(tk) => {
            kcs.push(tk);
            p.check_search_list("after creating 'legacy'")
        }
        Err(e) => Err(e),
    };
    let result = result.and_then(|()| {
        let partition_path = p.run_dir.join("Library/Keychains/e10-partition.keychain-db");
        let tk = Throwaway::create(p, "partition", &partition_path)?;
        kcs.push(tk);
        p.check_search_list("after creating 'partition'")
    });
    let result = result.and_then(|()| phases(p, &mut kcs, &reader_v1, &other, sig_v1, sig_other));

    let mut cleanup = Vec::new();
    for tk in kcs.iter_mut() {
        cleanup.push(tk.clean());
    }
    drop(kcs);
    p.step("lock and delete the throwaway keychains", json!(cleanup));
    p.extra.insert("cleanup".into(), json!(cleanup));
    if p.run_dir.starts_with(&p.scratch) {
        let _ = std::fs::remove_dir_all(&p.run_dir);
    }
    p.extra.insert("run_dir_removed".into(), json!(!p.run_dir.exists()));
    result
}

fn main() {
    // Before anything else: the keychain search list and the default keychain.
    let (before_list, before_default) = search_state();
    if !before_list.ok() || !before_default.ok() {
        eprintln!("could not record the keychain search list or default keychain; nothing was done");
        std::process::exit(1);
    }
    let exp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let scratch = exp_dir.join("target").join("scratch");
    let started = kp::now_secs();
    let mut p = Probe {
        run_dir: scratch.join(format!("run-{started}")),
        exp_dir: exp_dir.clone(),
        scratch,
        home: std::env::var("HOME").unwrap_or_default(),
        before: (before_list.stdout.clone(), before_default.stdout.clone()),
        steps: Vec::new(),
        cases: Vec::new(),
        extra: Map::new(),
        counter: 0,
    };

    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| body(&mut p)));
    let run = match outcome {
        Ok(Ok(())) => "completed".to_string(),
        Ok(Err(e)) => format!("stopped: {e}"),
        Err(_) => "stopped: panicked".to_string(),
    };

    let (after_list, after_default) = search_state();
    let list_same = after_list.ok() && after_list.stdout == before_list.stdout;
    let default_same = after_default.ok() && after_default.stdout == before_default.stdout;
    let side = |o: &RunOutput| {
        json!({
            "output": p.sb(&o.stdout),
            "bytes": o.stdout.len(),
            "sha256": kp::sha256_hex(&o.stdout),
        })
    };
    let check = json!({
        "commands": ["security list-keychains -d user", "security default-keychain -d user"],
        "before": {"list": side(&before_list), "default": side(&before_default)},
        "after": {"list": side(&after_list), "default": side(&after_default)},
        "list_byte_for_byte_unchanged": list_same,
        "default_byte_for_byte_unchanged": default_same,
    });

    let mut results = json!({
        "experiment": "005-macos-keychain",
        "question": "E10: Does Keychain work unattended? Access from a daemon, a rebuilt unsigned binary and a locked session.",
        "started_utc": kp::iso_utc(started),
        "finished_utc": kp::iso_utc(kp::now_secs()),
        "run": run,
        "search_list_check": check,
        "cases": p.cases,
    });
    for (k, v) in std::mem::take(&mut p.extra) {
        results[k] = v;
    }
    results["steps"] = json!(p.steps);

    let mut text = p.s(&serde_json::to_string_pretty(&results).expect("serialize"));
    let user = std::env::var("USER").unwrap_or_default();
    if user.len() > 2 && text.contains(&user) {
        text = text.replace(&user, "<user>");
    }
    let path = p.exp_dir.join("results.json");
    if let Err(e) = std::fs::write(&path, text + "\n") {
        eprintln!("could not write results.json: {e}");
    }

    println!();
    println!("run: {run}");
    println!("search list unchanged: {list_same}; default keychain unchanged: {default_same}");
    println!("results: results.json");
    if !list_same || !default_same {
        eprintln!();
        eprintln!("!!!!!!!! THE KEYCHAIN SEARCH LIST OR DEFAULT KEYCHAIN CHANGED !!!!!!!!");
        eprintln!("before list:    {:?}", String::from_utf8_lossy(&before_list.stdout));
        eprintln!("after list:     {:?}", String::from_utf8_lossy(&after_list.stdout));
        eprintln!("before default: {:?}", String::from_utf8_lossy(&before_default.stdout));
        eprintln!("after default:  {:?}", String::from_utf8_lossy(&after_default.stdout));
        std::process::exit(3);
    }
    if run != "completed" {
        std::process::exit(1);
    }
}
