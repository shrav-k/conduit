#![cfg(unix)]

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    net::TcpListener,
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    process::{Output, Stdio},
    time::Duration,
};
use tokio::{
    process::{Child, Command},
    time::{sleep, timeout},
};
use uuid::Uuid;

const API: &str = "github.com/shrav-k/conduit/v0";
const CORE: &str = "github.com/shrav-k/conduit/core";
const FILE: &str = "example.org/conduit/provider-file";
const GITHUB: &str = "example.org/conduit/github";
const OUTCOME: &str = "io.github.shrav-k.conduit/outcome";
const WAIT: Duration = Duration::from_secs(5);

async fn cargo(directory: &Path, args: &[&str]) -> Vec<u8> {
    let output = timeout(
        Duration::from_secs(480),
        Command::new("cargo")
            .current_dir(directory)
            .env("CARGO_NET_OFFLINE", "true")
            .args(args)
            .kill_on_drop(true)
            .output(),
    )
    .await
    .expect("offline build timed out")
    .unwrap();
    if !output.status.success() {
        let start = output.stderr.len().saturating_sub(2000);
        panic!(
            "offline build failed: {}",
            String::from_utf8_lossy(&output.stderr[start..])
        );
    }
    output.stdout
}

async fn archive(repo: &Path, name: &str) -> (String, String) {
    let directory = repo.join("packages").join(name);
    let output = cargo(
        &directory,
        &[
            "run",
            "--offline",
            "--locked",
            "--release",
            "--bin",
            "package",
        ],
    )
    .await;
    let output = String::from_utf8(output).unwrap();
    let digest = output.trim().rsplit_once("sha256:").unwrap().1;
    (
        format!(
            "{}#{digest}",
            directory.join(format!("dist/{name}.tar.gz")).display()
        ),
        format!("sha256:{digest}"),
    )
}

async fn copy_stand_in(repo: &Path, path: &Path) {
    let messages = cargo(
        &repo.join("packages/github"),
        &[
            "build",
            "--offline",
            "--locked",
            "--release",
            "--bin",
            "github-mcp-stand-in",
            "--message-format=json",
        ],
    )
    .await;
    let executable = String::from_utf8(messages)
        .unwrap()
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .find_map(|line| {
            if line["target"]["name"] == "github-mcp-stand-in" {
                line["executable"].as_str().map(str::to_owned)
            } else {
                None
            }
        })
        .expect("the offline build did not name the stand-in");
    fs::copy(executable, path.join("github-mcp-server")).unwrap();
}

fn no_marker(bytes: &[u8], marker: &str) {
    assert!(
        !bytes
            .windows(marker.len())
            .any(|window| window == marker.as_bytes()),
        "synthetic marker escaped the backing process"
    );
}

fn scan(directory: &Path, marker: &str) {
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        match entry.file_type().unwrap() {
            kind if kind.is_dir() => scan(&entry.path(), marker),
            kind if kind.is_file() => no_marker(&fs::read(entry.path()).unwrap(), marker),
            kind if kind.is_symlink() => no_marker(
                fs::read_link(entry.path())
                    .unwrap()
                    .as_os_str()
                    .as_encoded_bytes(),
                marker,
            ),
            _ => {}
        }
    }
}

struct Fixture<'a> {
    repo: &'a Path,
    store: PathBuf,
    home: PathBuf,
    path: PathBuf,
    marker: &'a str,
}

impl Fixture<'_> {
    fn command(&self, verb: &str) -> Command {
        let mut child = Command::new(env!("CARGO_BIN_EXE_conduit"));
        child
            .current_dir(self.repo)
            .env_clear()
            .env("HOME", &self.home)
            .env("PATH", &self.path)
            .arg(verb)
            .arg("--store")
            .arg(&self.store)
            .stdin(Stdio::null())
            .kill_on_drop(true);
        child
    }

    async fn cli(&self, verb: &str, args: &[&str]) -> Output {
        let wait = if verb == "submit" {
            Duration::from_secs(30)
        } else {
            WAIT
        };
        let output = timeout(wait, self.command(verb).args(args).output())
            .await
            .unwrap_or_else(|_| panic!("conduit {verb} did not finish"))
            .unwrap();
        no_marker(&output.stdout, self.marker);
        no_marker(&output.stderr, self.marker);
        output
    }

    async fn success(&self, verb: &str, args: &[&str]) -> Output {
        let output = self.cli(verb, args).await;
        assert!(
            output.status.success(),
            "conduit {verb} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }

    fn seen(&self) -> Vec<Value> {
        let bytes = fs::read(self.home.join("seen")).unwrap();
        no_marker(&bytes, self.marker);
        String::from_utf8(bytes)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    }

    async fn serve(&self, port: u16) -> Child {
        let mut child = self
            .command("serve")
            .stdout(fs::File::create(self.home.join("serve.stdout")).unwrap())
            .stderr(fs::File::create(self.home.join("serve.stderr")).unwrap())
            .spawn()
            .unwrap();
        timeout(Duration::from_secs(20), async {
            while tokio::net::TcpStream::connect(("127.0.0.1", port))
                .await
                .is_err()
            {
                assert!(
                    child.try_wait().unwrap().is_none(),
                    "serve exited before listening"
                );
                sleep(Duration::from_millis(15)).await;
            }
        })
        .await
        .expect("serve did not listen");
        child
    }
}

fn doc(kind: &str, name: &str, body: Value) -> Value {
    let mut doc = json!({"apiVersion":API,"kind":kind,"metadata":{"name":name}});
    doc.as_object_mut()
        .unwrap()
        .extend(body.as_object().unwrap().clone());
    doc
}

fn project(directory: &Path, marker_file: &Path, file: &str, github: &str) {
    fs::create_dir(directory).unwrap();
    fs::write(directory.join("conduit.yaml"),json!({
        "apiVersion":API,"address":"example.org/tests/github",
        "packages":{FILE:{"version":"0.1.0","digest":file},GITHUB:{"version":"0.1.0","digest":github}}
    }).to_string()).unwrap();
    let reader = |name, template, target: Value| {
        doc(
            "Conduit",
            name,
            json!({
                "from":template,"consumer":{"kind":"Interface","name":"root"},"target":target
            }),
        )
    };
    let allowed = json!({"kind":"Pool","name":"github-pool","entry":"allowed"});
    let docs = [
        doc(
            "Resource",
            "file-token",
            json!({"spec":{
                "type":format!("{FILE}#file-secret.v1"),"config":{"path":marker_file}
            }}),
        ),
        doc(
            "Conduit",
            "use-token",
            json!({"spec":{
                "capability":format!("{CORE}#secret-provider.v1"),"operations":[format!("{CORE}#get-secret.v1")]
            }}),
        ),
        doc(
            "Resource",
            "repository",
            json!({"spec":{
                "type":format!("{GITHUB}#repository.v1"),"config":{"repository":"octo/allowed"},
                "inputs":{"token":{"schema":{"$ref":format!("{CORE}#secret-reference.v1")},
                    "to":["/config/token","/conduits/token/target"]}},
                "conduits":{"token":{"from":"use-token"}}
            }}),
        ),
        doc(
            "Pool",
            "pool",
            json!({"spec":{"type":format!("{CORE}#pool.v1"),"entries":{
                "token":{"from":"file-token"},
                "allowed":{"from":"repository","arguments":{"token":{"entry":"token"}}}
            }}}),
        ),
        doc("Pool", "github-pool", json!({"from":"pool"})),
        doc(
            "Conduit",
            "pool-management",
            json!({"spec":{
                "capability":format!("{CORE}#pool-management.v1"),
                "operations":[format!("{CORE}#set-up.v1"),format!("{CORE}#inspect.v1")]
            }}),
        ),
        reader(
            "github-admin",
            "pool-management",
            json!({"kind":"Pool","name":"github-pool"}),
        ),
        reader(
            "allowed-read",
            "example.org/conduit/github#read",
            allowed.clone(),
        ),
        doc(
            "Conduit",
            "issues-only",
            json!({"spec":{
                "capability":format!("{GITHUB}#repository-reading.v1"),"operations":[format!("{GITHUB}#issue.read.v1")]
            }}),
        ),
        reader("allowed-issues", "issues-only", allowed),
    ];
    fs::write(
        directory.join("objects.yaml"),
        docs.iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n---\n"),
    )
    .unwrap();
}

fn reply(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("expected a CLI result")
}

#[tokio::test]
async fn github_through_the_engine() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    let (file, file_digest) = archive(&repo, "provider-file").await;
    let (github, github_digest) = archive(&repo, "github").await;
    let home = tempfile::tempdir().unwrap();
    let root = home.path().canonicalize().unwrap();
    let marker = format!("synthetic-{}", Uuid::new_v4());
    let marker_file = root.join("marker");
    fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&marker_file)
        .unwrap()
        .write_all(marker.as_bytes())
        .unwrap();
    let fixture = Fixture {
        repo: &repo,
        store: root.join("store"),
        home: root.join("home"),
        path: root.join("bin"),
        marker: &marker,
    };
    fs::create_dir(&fixture.home).unwrap();
    fs::create_dir(&fixture.path).unwrap();
    copy_stand_in(&repo, &fixture.path).await;
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    fixture
        .success("init", &["--url", &format!("http://127.0.0.1:{port}/mcp")])
        .await;
    let mut serve = fixture.serve(port).await;
    fixture.success("install", &[&file]).await;
    fixture.success("install", &[&github]).await;
    let directory = root.join("project");
    project(&directory, &marker_file, &file_digest, &github_digest);
    let submitted = fixture
        .success("submit", &[directory.to_str().unwrap()])
        .await;
    let steps: Vec<Value> = String::from_utf8(submitted.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert!(
        steps
            .iter()
            .any(|step| step["step"] == "setup Pool/github-pool" && step["outcome"] == "succeeded")
    );

    assert!(
        !fixture.home.join("seen").exists(),
        "front started upstream before a read"
    );
    for (tool, args, number) in [
        ("allowed-read.issue.read.v1", r#"{"number":1}"#, 1),
        ("allowed-read.pull-request.list.v1", r#"{"limit":5}"#, 5),
    ] {
        let result = reply(&fixture.success("call", &[tool, args]).await);
        assert_eq!(result["_meta"][OUTCOME], "succeeded");
        assert_eq!(
            result["structuredContent"],
            json!({"data":{"owner":"octo","repo":"allowed","number":number}})
        );
    }
    let seen = fixture.seen();
    assert_eq!(seen.len(), 3);
    assert!(
        seen[0]["digest"] == format!("{:x}", Sha256::digest(marker.as_bytes())),
        "engine did not deliver the file-store marker"
    );
    assert_eq!(
        seen[0]["arguments"],
        json!(["stdio", "--read-only", "--toolsets", "issues,pull_requests"])
    );
    assert_eq!(
        seen[0]["environment"],
        json!(["GITHUB_PERSONAL_ACCESS_TOKEN", "HOME", "PATH"])
    );
    assert_eq!(
        seen[1],
        json!({"tool":"issue_read","arguments":{"method":"get","owner":"octo","repo":"allowed","issue_number":1}})
    );
    assert_eq!(
        seen[2],
        json!({"tool":"list_pull_requests","arguments":{"owner":"octo","repo":"allowed","perPage":5}})
    );

    assert!(
        !fixture
            .cli(
                "call",
                &[
                    "allowed-read.issue.read.v1",
                    r#"{"number":1,"repository":"octo/other"}"#
                ]
            )
            .await
            .status
            .success()
    );
    let tools = reply(&fixture.success("tools", &[]).await);
    let names: Vec<_> = tools["tools"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|tool| tool["name"].as_str())
        .collect();
    assert!(names.contains(&"allowed-issues.issue.read.v1"));
    assert!(!names.contains(&"allowed-issues.pull-request.read.v1"));
    let refused = fixture
        .cli(
            "call",
            &["allowed-issues.pull-request.read.v1", r#"{"number":1}"#],
        )
        .await;
    assert!(!refused.status.success());
    assert!(
        refused
            .stderr
            .windows(b"refused".len())
            .any(|window| window == b"refused")
            || refused
                .stdout
                .windows(b"refused".len())
                .any(|window| window == b"refused")
    );
    assert!(
        fixture.seen() == seen,
        "a rejected call reached the backing server"
    );

    let output = fixture.success("records", &[]).await;
    let records = String::from_utf8(output.stdout).unwrap();
    for operation in ["issue.read.v1", "pull-request.list.v1"] {
        let operation = format!("{GITHUB}#{operation}");
        assert!(
            records
                .lines()
                .map(|line| serde_json::from_str::<Value>(line).unwrap())
                .any(|record| record["type"] == "request-ended"
                    && record["data"]["operation"] == operation
                    && record["data"]["outcome"] == "succeeded"
                    && record["data"]["conduit"].is_string()
                    && record["data"].get("outputs").is_none())
        );
    }

    let pid = seen[0]["process"].as_i64().unwrap() as i32;
    assert_eq!(unsafe { libc::kill(pid, 0) }, 0);
    assert_eq!(
        unsafe { libc::kill(serve.id().unwrap() as i32, libc::SIGTERM) },
        0
    );
    assert!(
        timeout(Duration::from_secs(20), serve.wait())
            .await
            .expect("serve did not stop")
            .unwrap()
            .success()
    );
    timeout(WAIT, async {
        while unsafe { libc::kill(pid, 0) } == 0 {
            sleep(Duration::from_millis(15)).await;
        }
    })
    .await
    .expect("stand-in survived serve");
    scan(&fixture.store, &marker);
    for path in ["serve.stdout", "serve.stderr"] {
        no_marker(&fs::read(fixture.home.join(path)).unwrap(), &marker);
    }
    println!("stand-in call: {}", seen[1]);
}
