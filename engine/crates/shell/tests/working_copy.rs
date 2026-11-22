//! A real installed working-copy Resource, reached over its two exported Conduits.
#![cfg(unix)]

#[path = "../../../../packages/git/tests/support/mod.rs"]
mod scratch_git;

mod package_target;

use package_target::package_target_dir;
use scratch_git::Scratch;
use serde_json::{Value, json};
use std::{
    fs,
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    time::{Duration, Instant},
};

const PACKAGE: &str = "example.org/conduit/git";
const OUTCOME: &str = "io.github.shrav-k.conduit/outcome";

fn pack(repository: &Path) -> (String, String) {
    let package = repository.join("packages/git");
    let output = Command::new("cargo")
        .current_dir(&package)
        .envs(package_target_dir(&package).map(|dir| ("CARGO_TARGET_DIR", dir)))
        .env("CARGO_NET_OFFLINE", "true")
        .args([
            "run",
            "--offline",
            "--locked",
            "--release",
            "--bin",
            "package",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "cannot build working-copy archive: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let digest = String::from_utf8(output.stdout).unwrap();
    let digest = digest.trim().rsplit_once("sha256:").unwrap().1;
    (
        format!(
            "{}#sha256:{digest}",
            package.join("dist/git.tar.gz").display()
        ),
        format!("sha256:{digest}"),
    )
}

struct Server(Child);

impl Drop for Server {
    fn drop(&mut self) {
        if self.0.try_wait().ok().flatten().is_none() {
            // It is our own child. The engine shuts down its package run on TERM.
            unsafe { libc::kill(self.0.id() as i32, libc::SIGTERM) };
            let deadline = Instant::now() + Duration::from_secs(30);
            while Instant::now() < deadline {
                if self.0.try_wait().ok().flatten().is_some() {
                    return;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

struct Cli<'a> {
    store: &'a Path,
    home: &'a Path,
}

impl Cli<'_> {
    fn command(&self, verb: &str) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_conduit"));
        command
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap())
            .env("HOME", self.home)
            .arg(verb)
            .arg("--store")
            .arg(self.store);
        command
    }

    fn run(&self, verb: &str, args: &[&str]) -> Output {
        self.command(verb).args(args).output().unwrap()
    }

    fn succeeded(&self, verb: &str, args: &[&str]) -> Value {
        let output = self.run(verb, args);
        assert!(
            output.status.success(),
            "conduit {verb}: {} {}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
        String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .rfind(|line| verb != "install" || line.starts_with('{'))
            .map(|line| serde_json::from_str(line).unwrap())
            .unwrap_or(Value::Null)
    }

    fn serve(&self, port: u16) -> Server {
        let log = self.home.join("serve.stderr");
        let mut server = Server(
            self.command("serve")
                .stdout(Stdio::null())
                .stderr(fs::File::create(&log).unwrap())
                .spawn()
                .unwrap(),
        );
        let deadline = Instant::now() + Duration::from_secs(60);
        while TcpStream::connect(("127.0.0.1", port)).is_err() {
            assert!(
                server.0.try_wait().unwrap().is_none(),
                "serve exited: {}",
                fs::read_to_string(&log).unwrap()
            );
            assert!(Instant::now() < deadline, "serve did not bind");
            std::thread::sleep(Duration::from_millis(20));
        }
        server
    }
}

fn write_project(
    project: &Path,
    spec: &Path,
    digest: &str,
    source: &Path,
    base: &str,
    root: &Path,
) {
    let spec = conduit_loader::Specification::read(spec).unwrap();
    let api = &spec.names().api_version;
    let core = &spec.names().core_package;
    fs::create_dir(project).unwrap();
    let manifest = json!({"apiVersion":api,"address":"example.org/test/working-copy","packages":{PACKAGE:{"version":"0.1.0","digest":digest}}});
    let mut documents = vec![
        json!({"apiVersion":api,"kind":"Pool","metadata":{"name":"workspace-pool"},"spec":{"type":format!("{core}#pool.v1")}}),
        json!({"apiVersion":api,"kind":"Pool","metadata":{"name":"workspace"},"from":"workspace-pool"}),
        json!({"apiVersion":api,"kind":"Resource","metadata":{"name":"copy-template"},"spec":{"type":format!("{PACKAGE}#working-copy.v1"),"config":{"repository":source,"commit":base,"root":root}}}),
        json!({"apiVersion":api,"kind":"Resource","metadata":{"name":"copy"},"from":"copy-template","owner":"workspace"}),
    ];
    for (name, template) in [("copy-read", "read"), ("copy-write", "read-write")] {
        documents.push(
            json!({"apiVersion":api,"kind":"Conduit","metadata":{"name":name},
            "from":format!("{PACKAGE}#{template}"),"consumer":{"kind":"Interface","name":"root"},
            "target":{"kind":"Resource","name":"copy"}}),
        );
    }
    fs::write(
        project.join("conduit.json"),
        serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
    fs::write(
        project.join("documents.json"),
        serde_json::to_vec(&documents).unwrap(),
    )
    .unwrap();
}

#[test]
fn working_copy_through_the_engine() {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    let (archive, digest) = pack(&repository);
    let temp = Scratch::new();
    let source = temp.repository();
    fs::write(source.join("hello.txt"), b"base\n").unwrap();
    let base = temp.commit(&source);
    let root = temp.path.join("working-copy");
    let store = temp.path.join("store");
    let home = temp.path.join("home");
    let cli = Cli {
        store: &store,
        home: &home,
    };
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    cli.succeeded("init", &["--url", &format!("http://127.0.0.1:{port}/mcp")]);
    let server = cli.serve(port);
    cli.succeeded("install", &[&archive]);
    let project = temp.path.join("project");
    write_project(
        &project,
        &repository.join("spec"),
        &digest,
        &source,
        &base,
        &root,
    );
    cli.succeeded("submit", &[project.to_str().unwrap()]);
    assert_eq!(fs::read(root.join("hello.txt")).unwrap(), b"base\n");
    assert!(!root.join(".git").exists());
    let state: Value =
        serde_json::from_slice(&fs::read(store.join("state.json")).unwrap()).unwrap();
    let instance = state["state"]["objects"]
        .as_array()
        .unwrap()
        .iter()
        .find(|o| o["kind"] == "Resource" && o["name"] == "copy" && o.get("spec").is_none())
        .unwrap();
    assert!(
        store
            .join("instances")
            .join(instance["id"].as_str().unwrap())
            .join("repository.git")
            .is_dir()
    );

    let tools = cli.succeeded("tools", &[]);
    let tools = tools["tools"].as_array().unwrap();
    assert!(
        tools
            .iter()
            .any(|tool| tool["name"] == "copy-write.file.write.v1")
    );
    assert!(
        !tools
            .iter()
            .any(|tool| tool["name"] == "copy-read.file.write.v1")
    );
    let write = cli.succeeded(
        "call",
        &[
            "copy-write.file.write.v1",
            r#"{"path":"hello.txt","content":"from root Interface\n"}"#,
        ],
    );
    assert_eq!(write["_meta"][OUTCOME], "succeeded");
    let rejected = cli.run(
        "call",
        &[
            "copy-read.file.write.v1",
            r#"{"path":"hello.txt","content":"forbidden"}"#,
        ],
    );
    assert!(!rejected.status.success());
    assert!(rejected.stdout.is_empty());
    let stderr = String::from_utf8(rejected.stderr).unwrap();
    assert!(
        stderr.contains("No tool named copy-read.file.write.v1 is presented to this instance."),
        "{stderr}"
    );
    assert_eq!(
        fs::read(root.join("hello.txt")).unwrap(),
        b"from root Interface\n"
    );
    let changes = cli.succeeded("call", &["copy-read.changes.v1", "{}"]);
    assert_eq!(changes["_meta"][OUTCOME], "succeeded");
    let changes = &changes["structuredContent"];
    assert_eq!(changes["base"], base);
    assert_eq!(
        changes["files"],
        json!([{"path":"hello.txt","change":"modified"}])
    );
    let patch = changes["patch"].as_str().unwrap();
    assert!(
        patch.starts_with("diff --git a/hello.txt b/hello.txt\n")
            && patch.contains("-base\n+from root Interface\n")
    );
    let applied = temp.clone_at(&source, &base);
    temp.git(&applied, &["apply", "--index"], Some(patch));
    assert_eq!(
        fs::read(applied.join("hello.txt")).unwrap(),
        fs::read(root.join("hello.txt")).unwrap()
    );

    cli.succeeded("retire", &["Resource/copy"]);
    assert!(!root.exists());
    drop(server);
}
