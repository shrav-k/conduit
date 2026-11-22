use conduit_core::packages::bytes_digest;
use fs2::FileExt;
use serde_json::{Value, json};
use std::{
    fs,
    io::{BufRead, BufReader},
    net::TcpListener,
    os::unix::fs::{PermissionsExt, symlink},
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    sync::mpsc,
    time::{Duration, Instant},
};

const A: &str = "example.org/closure/root";
const B: &str = "example.org/closure/branch";
const C: &str = "example.org/closure/leaf";

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn api_version() -> String {
    conduit_loader::Specification::read(&repository().join("spec"))
        .unwrap()
        .names()
        .api_version
        .clone()
}

struct Lab {
    server: Child,
    temp: tempfile::TempDir,
    project: PathBuf,
    config: PathBuf,
    store: PathBuf,
}

impl Lab {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let project = temp.path().join("project");
        let config = temp.path().join("config");
        let store = temp.path().join("store");
        fs::create_dir(&project).unwrap();
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let url = format!("http://{}/mcp", listener.local_addr().unwrap());
        let command = |verb: &str| {
            let mut command = Command::new(env!("CARGO_BIN_EXE_conduit"));
            command
                .arg(verb)
                .arg("--store")
                .arg(&store)
                .env("CONDUIT_CONFIG_DIR", &config)
                .current_dir(&project);
            command
        };
        assert!(
            command("init")
                .args(["--url", &url])
                .output()
                .unwrap()
                .status
                .success()
        );
        drop(listener);
        let mut lab = Self {
            server: command("serve")
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .unwrap(),
            temp,
            project,
            config,
            store,
        };
        let stdout = lab.server.stdout.take().unwrap();
        let (sent, received) = mpsc::channel();
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            sent.send(line).unwrap();
            let _ = std::io::copy(&mut reader, &mut std::io::sink());
        });
        let ready = received.recv_timeout(Duration::from_secs(90)).unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&ready).unwrap()["ready"]["url"],
            url
        );
        lab
    }

    fn command(&self, verb: &str, explicit: bool) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_conduit"));
        command
            .arg(verb)
            .env("CONDUIT_CONFIG_DIR", &self.config)
            .current_dir(&self.project);
        if explicit {
            command.arg("--store").arg(&self.store);
        }
        command
    }

    fn run(&self, verb: &str, explicit: bool, args: &[&str]) -> Output {
        self.command(verb, explicit).args(args).output().unwrap()
    }

    fn install(&self, args: &[&str]) -> Output {
        self.run("install", true, args)
    }

    fn packages(&self) -> Value {
        let output = self.run("call", true, &["management.list.v1"]);
        assert!(output.status.success());
        serde_json::from_slice::<Value>(&output.stdout).unwrap()["structuredContent"]["packages"]
            .clone()
    }

    fn archive(
        &self,
        name: &str,
        address: &str,
        version: &str,
        pins: &[(&str, &str, &str, Option<&str>)],
    ) -> Archive {
        let mut manifest = format!(
            "apiVersion: {}\naddress: {address}\nversion: {version}\n",
            api_version()
        );
        if !pins.is_empty() {
            manifest.push_str("packages:\n");
        }
        for (dep, version, digest, fetch) in pins {
            manifest.push_str(&format!(
                "  {dep}:\n    version: {version}\n    digest: {digest}\n"
            ));
            if let Some(fetch) = fetch {
                manifest.push_str(&format!("    fetch: {}\n", json!(fetch)));
            }
        }
        self.pack(name, &manifest)
    }

    fn pack(&self, name: &str, manifest: &str) -> Archive {
        let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(
            Vec::new(),
            flate2::Compression::default(),
        ));
        let mut header = tar::Header::new_ustar();
        header.set_size(manifest.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "conduit.yaml", manifest.as_bytes())
            .unwrap();
        let file = self.temp.path().join(name);
        fs::write(&file, tar.into_inner().unwrap().finish().unwrap()).unwrap();
        Archive::new(file)
    }
}

impl Drop for Lab {
    fn drop(&mut self) {
        let _ = self.server.kill();
        let _ = self.server.wait();
    }
}

struct Archive {
    file: PathBuf,
    digest: String,
}

impl Archive {
    fn new(file: PathBuf) -> Self {
        let digest = bytes_digest(&fs::read(&file).unwrap());
        Self { file, digest }
    }

    fn fixture(file: &str) -> Self {
        Self::new(repository().join("conformance/fixtures").join(file))
    }

    fn argument(&self) -> String {
        format!("{}#{}", self.file.display(), self.hex())
    }

    fn hex(&self) -> &str {
        self.digest.strip_prefix("sha256:").unwrap()
    }

    fn copy_to(&self, directory: &Path) {
        fs::create_dir_all(directory).unwrap();
        fs::copy(&self.file, directory.join(self.hex())).unwrap();
    }
}

fn success(output: &Output, action: &str) -> String {
    assert_eq!(output.status.code(), Some(0), "{action}");
    String::from_utf8(output.stdout.clone()).unwrap()
}

fn lines<'a>(text: &'a str, prefix: &str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| line.starts_with(prefix))
        .collect()
}

#[test]
fn install_deduplicates_semver_equivalent_pins() {
    let lab = Lab::new();
    let right_name = "example.org/closure/right";
    let leaf = lab.archive("leaf.tar.gz", C, "1.0.0", &[]);
    let left = lab.archive(
        "left.tar.gz",
        B,
        "1.0.0",
        &[(C, "1.0.0+a", &leaf.digest, None)],
    );
    let right = lab.archive(
        "right.tar.gz",
        right_name,
        "1.0.0",
        &[(C, "1.0.0+b", &leaf.digest, None)],
    );
    let root = lab.archive(
        "root.tar.gz",
        A,
        "1.0.0",
        &[
            (B, "1.0.0", &left.digest, None),
            (right_name, "1.0.0", &right.digest, None),
        ],
    );
    for archive in [&leaf, &left, &right] {
        archive.copy_to(&lab.config.join("packages"));
    }

    let text = success(
        &lab.install(&[&root.argument()]),
        "install the shared release once",
    );
    let installs = lines(&text, "install Package/");
    let names: Vec<_> = installs
        .iter()
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .collect();
    assert_eq!(
        names,
        [
            format!("Package/{C}"),
            format!("Package/{B}"),
            format!("Package/{right_name}"),
            format!("Package/{A}")
        ]
    );
    assert!(installs[0].contains(" version=1.0.0+a "));
    assert!(installs[0].contains(&format!("needed-by=[{B}@1.0.0,{right_name}@1.0.0]")));
    assert_eq!(lines(&text, "{").len(), 4);
}

#[test]
fn install_retries_missing_pin_with_later_fetch() {
    let lab = Lab::new();
    let right_name = "example.org/closure/right";
    let leaf = lab.archive("leaf.tar.gz", C, "1.0.0", &[]);
    let fetch = url::Url::from_file_path(&leaf.file).unwrap().to_string();
    let left = lab.archive(
        "left.tar.gz",
        B,
        "1.0.0",
        &[(C, "1.0.0", &leaf.digest, None)],
    );
    let right = lab.archive(
        "right.tar.gz",
        right_name,
        "1.0.0",
        &[(C, "1.0.0", &leaf.digest, Some(&fetch))],
    );
    let root = lab.archive(
        "root.tar.gz",
        A,
        "1.0.0",
        &[
            (B, "1.0.0", &left.digest, None),
            (right_name, "1.0.0", &right.digest, None),
        ],
    );
    for archive in [&left, &right] {
        archive.copy_to(&lab.config.join("packages"));
    }

    let text = success(
        &lab.install(&[&root.argument()]),
        "try the later branch's file fetch",
    );
    let installs = lines(&text, "install Package/");
    let names: Vec<_> = installs
        .iter()
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .collect();
    assert_eq!(
        names,
        [
            format!("Package/{C}"),
            format!("Package/{B}"),
            format!("Package/{right_name}"),
            format!("Package/{A}")
        ]
    );
    assert!(installs[0].contains(&format!("from=fetch:{fetch} ")));
    assert!(installs[0].contains(&format!("needed-by=[{B}@1.0.0,{right_name}@1.0.0]")));
    assert!(lines(&text, "stop ").is_empty());
}

#[test]
fn install_keeps_comments_inside_block_pins() {
    let lab = Lab::new();
    let old = lab.archive("old.tar.gz", A, "1.0.0", &[]);
    let new = lab.archive("new.tar.gz", A, "2.0.0", &[]);
    let added = lab.archive("added.tar.gz", B, "1.0.0", &[]);
    let file = lab.project.join("conduit.yaml");
    let authored = format!(
        "apiVersion: {}\npackages:\n  {A}:\n    # retained comment\n\n    version: 1.0.0\n    digest: '{}'\n",
        api_version(),
        old.digest
    );
    fs::write(&file, &authored).unwrap();

    let output = lab.install(&[&added.argument()]);
    assert!(
        success(
            &output,
            "insert beside a block pin with a comment and blank line"
        )
        .contains("pin added ")
    );
    let inserted = format!(
        "{authored}  {B}:\n    version: 1.0.0\n    digest: '{}'\n",
        added.digest
    );
    assert_eq!(fs::read_to_string(&file).unwrap(), inserted);
    let output = lab.install(&[&new.argument()]);
    assert!(
        success(&output, "move the block pin without touching its comment")
            .contains("pin moved up ")
    );
    let moved = inserted
        .replacen("version: 1.0.0", "version: 2.0.0", 1)
        .replacen(&old.digest, &new.digest, 1);
    assert_eq!(fs::read_to_string(&file).unwrap(), moved);
}

#[test]
fn install_serializes_manifest_replacement() {
    let lab = Lab::new();
    let file = lab.project.join("conduit.yaml");
    let skeleton = format!("apiVersion: {}\npackages: {{}}\n", api_version());
    fs::write(&file, &skeleton).unwrap();
    let first = lab.archive("first.tar.gz", B, "1.0.0", &[]);
    let second = lab.archive("second.tar.gz", C, "1.0.0", &[]);
    let gate = fs::File::open(&lab.project).unwrap();
    FileExt::lock_exclusive(&gate).unwrap();

    let (report, results) = mpsc::channel();
    for (address, archive) in [(B, &first), (C, &second)] {
        let mut command = lab.command("install", true);
        command.arg(archive.argument());
        let report = report.clone();
        std::thread::spawn(move || {
            report.send((address, command.output().unwrap())).unwrap();
        });
    }
    drop(report);
    let deadline = Instant::now() + Duration::from_secs(30);
    let both_installed = loop {
        if lab.packages().as_array().unwrap().len() == 2 {
            break true;
        }
        if Instant::now() >= deadline {
            break false;
        }
        std::thread::sleep(Duration::from_millis(30));
    };
    let early = results.recv_timeout(Duration::from_secs(1)).ok();
    let while_locked = fs::read_to_string(&file).unwrap();
    drop(gate);

    let mut outputs: Vec<_> = early.into_iter().collect();
    let returned_early = !outputs.is_empty();
    while outputs.len() < 2 {
        outputs.push(results.recv_timeout(Duration::from_secs(30)).unwrap());
    }
    assert!(
        both_installed,
        "both requests finished before manifest publication"
    );
    assert!(
        !returned_early,
        "install must hold the project lock to publish a pin"
    );
    assert_eq!(while_locked, skeleton);
    outputs.sort_by_key(|(_, output)| output.status.code());
    let [(winner, installed), (loser, stopped)] = outputs.as_slice() else {
        unreachable!()
    };
    assert_eq!(installed.status.code(), Some(0));
    assert_eq!(stopped.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&stopped.stderr).contains("changed since preflight"));
    let final_manifest = fs::read_to_string(file).unwrap();
    assert!(final_manifest.contains(winner));
    assert!(!final_manifest.contains(loser));
    assert!(!lab.project.join(".conduit").exists());
}

#[test]
fn install_walks_the_closure_leaves_first() {
    for (source, explicit) in [
        ("cache", true),
        ("fetch", true),
        ("mirror", false),
        ("config", false),
    ] {
        let lab = Lab::new();
        let leaf = lab.archive("c.tar.gz", C, "1.0.0", &[]);
        let cache = lab.config.join("packages");
        let mirror = lab.temp.path().join("mirror");
        let fetch = url::Url::from_file_path(&leaf.file).unwrap().to_string();
        let leaf_fetch = (source == "fetch").then_some(fetch.as_str());
        let branch = lab.archive(
            "b.tar.gz",
            B,
            "1.0.0",
            &[(C, "1.0.0", &leaf.digest, leaf_fetch)],
        );
        let root = lab.archive(
            "a.tar.gz",
            A,
            "1.0.0",
            &[(B, "1.0.0", &branch.digest, None)],
        );
        branch.copy_to(&cache);
        let mut args = Vec::new();
        let mirror_arg = mirror.to_str().unwrap();
        match source {
            "cache" => leaf.copy_to(&cache),
            "fetch" => fs::write(cache.join(leaf.hex()), "mismatch").unwrap(),
            "mirror" | "config" => {
                leaf.copy_to(&mirror);
                if source == "mirror" {
                    args.extend(["--mirror", mirror_arg]);
                } else {
                    fs::write(
                        lab.config.join("config.json"),
                        json!({"mirrors":[mirror]}).to_string(),
                    )
                    .unwrap();
                }
            }
            _ => unreachable!(),
        }
        let arg = root.argument();
        args.push(&arg);
        let output = lab.run("install", explicit, &args);
        let first = success(&output, "first install walks the closure");
        let installs = lines(&first, "install Package/");
        assert_eq!(installs.len(), 3, "one printed request per archive");
        for (line, address, needer) in [
            (installs[0], C, format!("[{B}@1.0.0]")),
            (installs[1], B, format!("[{A}@1.0.0]")),
            (installs[2], A, "[]".into()),
        ] {
            assert!(line.starts_with(&format!("install Package/{address} ")));
            assert!(line.contains(&format!("needed-by={needer}")));
        }
        let source = match source {
            "cache" => "cache".into(),
            "fetch" => format!("fetch:{fetch}"),
            _ => format!("mirror:{mirror_arg}"),
        };
        assert!(installs[0].contains(&format!("from={source} ")));
        assert!(installs[1].contains("from=cache "));
        assert!(installs[2].contains("from=argument "));
        for archive in [&leaf, &branch, &root] {
            assert_eq!(
                fs::read(cache.join(archive.hex())).unwrap(),
                fs::read(&archive.file).unwrap()
            );
        }

        fs::remove_file(cache.join(branch.hex())).unwrap();
        fs::remove_file(cache.join(leaf.hex())).unwrap();
        let repeated = success(
            &lab.run("install", explicit, &args),
            "repeat install prunes an installed branch",
        );
        let present = lines(&repeated, "present Package/");
        assert_eq!(present.len(), 1);
        assert!(present[0].starts_with(&format!("present Package/{B} ")));
        assert!(present[0].ends_with(" with its dependencies (PKG-028)"));
        assert!(!repeated.contains(&format!("Package/{C}")));
        assert_eq!(lines(&repeated, "install Package/").len(), 1);

        assert_eq!(
            lab.install(&["--mirror", root.file.to_str().unwrap(), &arg])
                .status
                .code(),
            Some(2)
        );
        for invalid in [
            json!([]),
            json!({"mirrors":["relative"]}),
            json!({"mirrors":[root.file]}),
            json!({"mirrors":[],"other":true}),
        ] {
            fs::write(lab.config.join("config.json"), invalid.to_string()).unwrap();
            assert_eq!(lab.install(&[&arg]).status.code(), Some(2));
        }
    }
}

#[test]
fn install_pins_exactly_up_or_down() {
    let lab = Lab::new();
    let low = lab.archive("low.tar.gz", A, "1.0.0", &[]);
    let dep = lab.archive("dependency.tar.gz", B, "1.0.0", &[]);
    let high = lab.archive(
        "high.tar.gz",
        A,
        "2.0.0",
        &[(B, "1.0.0", &dep.digest, None)],
    );
    dep.copy_to(&lab.config.join("packages"));
    let low_arg = low.argument();
    let high_arg = high.argument();
    let api = api_version();
    let skeleton = lab.project.join("conduit.yaml");
    fs::write(&skeleton, format!("apiVersion: {api}\npackages: {{}}\n")).unwrap();
    fs::set_permissions(&skeleton, fs::Permissions::from_mode(0o640)).unwrap();
    let text = success(
        &lab.install(&[&low_arg]),
        "install adds an exact pin to the init skeleton",
    );
    assert!(text.contains(&format!("pin added {A} version=1.0.0")));
    let row = |version: &str, digest: &str| {
        format!("  {A}: {{version: {version}, digest: '{digest}'}}\n")
    };
    assert_eq!(
        fs::read_to_string(&skeleton).unwrap(),
        format!(
            "apiVersion: {api}\npackages:\n{}",
            row("1.0.0", &low.digest)
        )
    );
    assert_eq!(
        fs::metadata(&skeleton).unwrap().permissions().mode() & 0o777,
        0o640
    );
    fs::remove_file(&skeleton).unwrap();

    for (name, template) in [
        (
            "conduit.yaml",
            format!(
                "# retained\napiVersion: {api}\npackages:\n  {A}: {{version: VERSION, digest: 'DIGEST'}}\n# retained after\n"
            ),
        ),
        (
            "conduit.yml",
            format!(
                "apiVersion: {api}\npackages:\n  {A}:\n    version: VERSION\n    digest: 'DIGEST'\n# retained after\n"
            ),
        ),
        (
            "conduit.json",
            format!(
                "{{\n  \"apiVersion\": \"{api}\",\n  \"packages\": {{ \"{A}\": {{ \"version\": \"VERSION\", \"digest\": \"DIGEST\" }} }}\n}}\n"
            ),
        ),
    ] {
        let manifest = lab.project.join(name);
        let low_text = template
            .replace("VERSION", "1.0.0")
            .replace("DIGEST", &low.digest);
        let high_text = template
            .replace("VERSION", "2.0.0")
            .replace("DIGEST", &high.digest);
        let unrelated = low_text.replacen(A, C, 1);
        let with_pin = match name {
            "conduit.yaml" => unrelated.replace(
                "# retained after",
                &format!("{}# retained after", row("1.0.0", &low.digest)),
            ),
            "conduit.yml" => unrelated.replace(
                "# retained after",
                &format!(
                    "  {A}:\n    version: 1.0.0\n    digest: '{}'\n# retained after",
                    low.digest
                ),
            ),
            _ => unrelated.replace(
                " } }\n",
                &format!(
                    " }}, \"{A}\": {{\"version\": \"1.0.0\", \"digest\": \"{}\"}} }}\n",
                    low.digest
                ),
            ),
        };
        fs::write(&manifest, &unrelated).unwrap();
        let added = success(
            &lab.install(&[&low_arg]),
            "install adds a pin beside an unrelated pin",
        );
        assert!(added.contains(&format!("pin added {A}")));
        assert_eq!(fs::read_to_string(&manifest).unwrap(), with_pin);
        fs::write(&manifest, &low_text).unwrap();
        for (arg, expected, action) in [
            (&high_arg, &high_text, "moved up"),
            (&low_arg, &low_text, "moved down"),
            (&low_arg, &low_text, "unchanged"),
        ] {
            let output = success(&lab.install(&[arg]), "install moves the named pin only");
            assert!(output.contains(&format!("pin {action} {A}")));
            assert_eq!(&fs::read_to_string(&manifest).unwrap(), expected);
        }
        fs::remove_file(manifest).unwrap();
    }
    assert!(success(&lab.install(&[&low_arg]), "install without a manifest").contains("pin none:"));
    for text in [
        format!(
            "apiVersion: {api}\npackages: {{{A}: {{version: 1.0.0, digest: '{}'}}}}\n",
            low.digest
        ),
        format!(
            "apiVersion: {api}\npackages:\n  {A}: {{version: 1.0.0, digest: '{}', fetch: 'file:///fixture/archive'}}\n",
            low.digest
        ),
        format!("apiVersion: {api}\npackages: []\n"),
    ] {
        fs::write(&skeleton, &text).unwrap();
        let output = lab.install(&[&high_arg]);
        assert_eq!(output.status.code(), Some(2));
        assert!(
            String::from_utf8(output.stdout)
                .unwrap()
                .contains("stop Manifest/-")
        );
        assert_eq!(fs::read_to_string(&skeleton).unwrap(), text);
    }
    fs::write(&skeleton, format!("apiVersion: {api}\npackages: {{}}\n")).unwrap();
    fs::copy(&skeleton, lab.project.join("conduit.yml")).unwrap();
    let output = lab.install(&[&low_arg]);
    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("stop Manifest/-")
    );
}

#[test]
fn install_stops_before_any_request_on_a_miss_or_conflict() {
    let lab = Lab::new();
    let check = |arg: &str, cause: &str, count: usize| {
        let before = lab.packages();
        let output = lab.install(&[arg]);
        assert_eq!(
            output.status.code(),
            Some(2),
            "preflight stops without an install.v1"
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(
            lines(&stdout, "stop ").len(),
            count,
            "stop count for {cause}"
        );
        assert!(stdout.contains(cause));
        assert!(lines(&stdout, "install ").is_empty());
        assert_eq!(lab.packages(), before);
        stdout
    };
    let missing = Archive::fixture("install-archives/missing-dependency.tar.gz");
    check(&missing.argument(), "archive=absent", 1);
    let leaf = lab.archive("leaf.tar.gz", C, "1.0.0", &[]);
    let branch = lab.archive("branch.tar.gz", B, "1.0.0", &[]);
    let two = lab.archive(
        "two.tar.gz",
        A,
        "1.0.0",
        &[
            (
                B,
                "1.0.0",
                &branch.digest,
                Some("https://example.org/archive"),
            ),
            (C, "1.0.0", &leaf.digest, Some("./archive")),
        ],
    );
    let stdout = check(&two.argument(), "(unbuilt)", 2);
    assert!(stdout.contains(B) && stdout.contains(C));

    leaf.copy_to(&lab.config.join("packages"));
    let wrong = lab.archive(
        "wrong.tar.gz",
        A,
        "1.0.0",
        &[(B, "1.0.0", &leaf.digest, None)],
    );
    check(&wrong.argument(), &format!("manifest={C}@1.0.0"), 1);
    success(
        &lab.install(&[&leaf.argument()]),
        "install a release before the conflicting pin",
    );
    branch.copy_to(&lab.config.join("packages"));
    let different = lab.archive(
        "different.tar.gz",
        C,
        "1.0.0",
        &[(B, "1.0.0", &branch.digest, None)],
    );
    check(&different.argument(), "installed=", 1);
    let conflict = lab.archive(
        "conflict.tar.gz",
        A,
        "1.0.0",
        &[(C, "1.0.0", &different.digest, None)],
    );
    check(&conflict.argument(), "installed=", 1);

    let other = Lab::new();
    let first = other.archive("first.tar.gz", C, "1.0.0", &[]);
    let second = other.archive(
        "second.tar.gz",
        C,
        "1.0.0",
        &[(B, "1.0.0", &branch.digest, None)],
    );
    let root = other.archive(
        "recursive-conflict.tar.gz",
        C,
        "1.0.0",
        &[(C, "1.0.0", &second.digest, None)],
    );
    first.copy_to(&other.config.join("packages"));
    second.copy_to(&other.config.join("packages"));
    let output = other.install(&[&root.argument()]);
    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("listed=")
    );
}

#[test]
fn install_stops_before_sending_if_a_dependency_manifest_cannot_be_read() {
    let lab = Lab::new();
    let readable = lab.archive("readable.tar.gz", B, "1.0.0", &[]);
    let unreadable = lab.pack("unreadable.tar.gz", "packages: {\n");
    let root = lab.archive(
        "root.tar.gz",
        A,
        "1.0.0",
        &[
            (B, "1.0.0", &readable.digest, None),
            (C, "1.0.0", &unreadable.digest, None),
        ],
    );
    let mirror = lab.project.join("mirror");
    readable.copy_to(&mirror);
    unreadable.copy_to(&mirror);

    let before = lab.packages();
    let output = lab.install(&["--mirror", "mirror", &root.argument()]);
    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut stdout_lines = stdout.lines();
    assert_eq!(stdout_lines.next(), Some("list Package/* as=root"));
    let stop = stdout_lines.next().unwrap();
    assert!(
        stdout_lines.next().is_none(),
        "only the list and preflight stop are printed"
    );
    assert!(stop.starts_with(&format!(
        "stop Package/{C} version=1.0.0 digest={} ",
        unreadable.digest
    )));
    assert!(stop.contains("manifest=unreadable from=mirror:mirror"));
    assert!(stop.contains(&format!("/mirror/{}", unreadable.hex())));
    assert!(stop.ends_with(&format!("needed-by=[{A}@1.0.0] PKG-023")));
    assert!(String::from_utf8_lossy(&output.stderr).contains("no install.v1 sent"));
    assert_eq!(lab.packages(), before);
}

#[test]
fn install_stops_partway_and_names_what_it_installed() {
    let lab = Lab::new();
    let agents = Archive::fixture("s1/archives/agents-0.2.1.tar.gz");
    let invalid = Archive::fixture("install-archives/invalid-document.tar.gz");
    let mirror = lab.temp.path().join("mirror");
    agents.copy_to(&mirror);
    invalid.copy_to(&mirror);
    let root = lab.archive(
        "root.tar.gz",
        A,
        "1.0.0",
        &[
            ("example.org/s1/agents", "0.2.1", &agents.digest, None),
            ("example.org/s1/filesystem", "0.1.0", &invalid.digest, None),
        ],
    );
    let output = lab.install(&["--mirror", mirror.to_str().unwrap(), &root.argument()]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "the engine rejects the second request"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(lines(&stdout, "install Package/").len(), 2);
    assert!(stdout.contains(&format!("not sent [{A}@1.0.0]")));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("example.org/s1/filesystem@0.1.0"));
    assert!(stderr.contains("example.org/s1/agents@0.2.1"));
    assert!(stderr.contains("nothing was rolled back"));
    let packages = lab.packages();
    assert_eq!(packages.as_array().unwrap().len(), 1);
    assert_eq!(packages[0]["address"], "example.org/s1/agents");
}

#[test]
fn install_dry_run_and_restoring_write_nothing() {
    let lab = Lab::new();
    let leaf = lab.archive("leaf.tar.gz", C, "1.0.0", &[]);
    let root = lab.archive(
        "root.tar.gz",
        A,
        "1.0.0",
        &[(C, "1.0.0", &leaf.digest, None)],
    );
    let mirror = lab.temp.path().join("mirror");
    leaf.copy_to(&mirror);
    root.copy_to(&mirror);
    let mirror_arg = mirror.to_str().unwrap();
    let manifest = lab.project.join("conduit.yaml");
    let skeleton = format!("apiVersion: {}\npackages: {{}}\n", api_version());
    fs::write(&manifest, &skeleton).unwrap();
    let before = lab.packages();
    let output = lab.install(&["--mirror", mirror_arg, "--dry-run", &root.argument()]);
    let stdout = success(&output, "dry-run succeeds on available closure");
    assert_eq!(stdout.lines().next(), Some("list Package/* as=root"));
    assert_eq!(lines(&stdout, "list "), ["list Package/* as=root"]);
    assert!(stdout.contains("dry-run: no install.v1 sent, nothing written"));
    assert!(lines(&stdout, "{").is_empty());
    assert!(!lab.config.join("packages").exists());
    assert_eq!(lab.packages(), before);
    assert_eq!(fs::read_to_string(&manifest).unwrap(), skeleton);

    let pins = format!(
        "apiVersion: {}\npackages:\n  {A}: {{version: 1.0.0, digest: '{}'}}\n",
        api_version(),
        root.digest
    );
    fs::write(&manifest, &pins).unwrap();
    let output = lab.install(&["--mirror", mirror_arg]);
    let stdout = success(&output, "bare archive argument restores pins");
    assert_eq!(stdout.lines().next(), Some("list Package/* as=root"));
    assert_eq!(lines(&stdout, "list "), ["list Package/* as=root"]);
    assert!(lines(&stdout, "pin ").is_empty());
    assert_eq!(fs::read_to_string(&manifest).unwrap(), pins);
    assert_eq!(lab.packages().as_array().unwrap().len(), 2);
    fs::remove_file(manifest).unwrap();
    let output = lab.install(&[]);
    assert_eq!(output.status.code(), Some(2));
    let diagnostic = String::from_utf8(output.stderr).unwrap();
    for name in conduit_loader::MANIFEST_NAMES {
        assert!(diagnostic.contains(name));
    }
}

#[test]
fn s1_t2_is_one_install_line() {
    let lab = Lab::new();
    symlink(
        repository().join("conformance"),
        lab.project.join("conformance"),
    )
    .unwrap();
    success(&lab.run("init", false, &[]), "initialize S1 manifest");
    let k3 = fs::read_to_string(repository().join("docs/implementation/plan-and-apply/README.md"))
        .unwrap();
    let t1 = lines(&k3, "T1: conduit install ");
    let t2 = lines(&k3, "T2: conduit install ");
    assert_eq!(t1.len(), 2);
    assert_eq!(t2.len(), 1, "K3 T2 is one author command");
    for line in t1 {
        let args: Vec<_> = line.split_whitespace().skip(3).collect();
        success(&lab.run("install", false, &args), "K3 T1 install");
    }
    let args: Vec<_> = t2[0].split_whitespace().skip(3).collect();
    let output = lab.run("install", false, &args);
    let stdout = success(&output, "K3 T2 installs the filesystem beside T1's release");
    let installs = lines(&stdout, "install Package/");
    assert_eq!(installs.len(), 2);
    assert!(installs[0].starts_with("install Package/example.org/s1/filesystem version=0.1.0 "));
    assert!(installs[0].ends_with(
        " from=mirror:conformance/fixtures/s1/mirror needed-by=[example.org/s1/history@0.1.0]"
    ));
    assert!(installs[1].starts_with("install Package/example.org/s1/history version=0.1.0 "));
    assert_eq!(lines(&stdout, "pin ").len(), 1);
    assert!(stdout.contains("pin added example.org/s1/history "));
    let manifest = lab.project.join("conduit.yaml");
    let contents = fs::read_to_string(manifest).unwrap();
    assert!(contents.contains("example.org/s1/filesystem: {version: 0.1.1"));
}
