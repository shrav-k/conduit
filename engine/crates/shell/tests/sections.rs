use serde_json::{Value, json};
use std::{
    fs,
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    time::{Duration, Instant},
};

const API: &str = "github.com/shrav-k/conduit/v0";
const CORE: &str = "github.com/shrav-k/conduit/core";

struct Server {
    child: Child,
    directory: tempfile::TempDir,
}

impl Server {
    fn start() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let url = format!("http://{address}/mcp");
        drop(listener);
        let mut init = Self::command(directory.path(), "init");
        assert!(init.args(["--url", &url]).status().unwrap().success());
        let child = Self::command(directory.path(), "serve")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let mut server = Self { child, directory };
        let start = Instant::now();
        while TcpStream::connect(address).is_err() {
            assert!(server.child.try_wait().unwrap().is_none(), "server exited");
            assert!(
                start.elapsed() < Duration::from_secs(30),
                "server did not start"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
        server
    }

    fn command(directory: &Path, verb: &str) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_conduit"));
        command
            .current_dir(directory)
            .env("CONDUIT_CONFIG_DIR", directory.join("config"))
            .arg(verb)
            .arg("--store")
            .arg(directory.join("store"))
            .stdout(Stdio::null());
        command
    }

    fn reject(&self, verb: &str, argument: &str) -> Output {
        let output = Self::command(self.directory.path(), verb)
            .arg(argument)
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        assert!(!output.status.success(), "expected {verb} rejection");
        output
    }

    fn project(&self, files: &[(&str, &str)]) -> PathBuf {
        let project = self.directory.path().join("project");
        fs::create_dir_all(&project).unwrap();
        for (name, text) in files {
            fs::write(project.join(name), text).unwrap();
        }
        project
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn assert_line(output: &Output, prefix: &str, suffix: &str) {
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .lines()
            .any(|line| line.starts_with(prefix) && line.ends_with(suffix)),
        "missing diagnostic line {prefix} … {suffix}"
    );
}

fn has_reply_location(output: &Output, rule: &str, pointer: &str) -> bool {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .flat_map(|reply| {
            reply
                .pointer("/structuredContent/diagnostics")
                .or_else(|| reply.get("diagnostics"))
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
        })
        .any(|diagnostic| {
            diagnostic["ruleId"] == rule
                && diagnostic["locations"][0]["logicalLocations"][0]["fullyQualifiedName"]
                    == pointer
        })
}

#[test]
fn submit_prints_authored_places() {
    let server = Server::start();
    let authored = format!(
        "apiVersion: {API}\ninterfaces:\n  auditor:\n    type: {CORE}#interface.v1\n    consumes:\n      view:\n        to: {{kind: Interface, name: auditor}}\n        capability: {CORE}#pool-management.v1\n        operations: []\n"
    );
    server.project(&[("a.yaml", &authored)]);
    let capability = server.reject("submit", "./project");
    assert_line(
        &capability,
        "./project/a.yaml:8:21: error: ",
        "[CON-021; SCT-007 auditor--view]",
    );
    assert!(has_reply_location(
        &capability,
        "CON-021",
        "/interfaces/auditor/consumes/view/capability"
    ));

    let framed = format!(
        "apiVersion: {API}\nkind: Pool\nmetadata: {{name: auditor--t}}\nspec: {{type: {CORE}#pool.v1}}\n"
    );
    server.project(&[("b.yaml", &framed)]);
    let duplicate = server.reject("submit", "./project");
    assert_line(&duplicate, "./project/b.yaml:3:18: error: ", "[NAM-027]");
    assert_line(
        &duplicate,
        "./project/a.yaml:4:5: note: also here ",
        "[NAM-027]",
    );
}

#[test]
fn install_prints_archive_member_places() {
    let server = Server::start();
    let manifest = format!("apiVersion: {API}\naddress: example.org/sectioned\nversion: 0.1.0\n");
    let authored =
        format!("templates:\n  interfaces:\n    auditor: {{type: {CORE}#interface.v1}}\n");
    let framed = format!(
        "apiVersion: {API}\nkind: Pool\nmetadata: {{name: auditor}}\nspec: {{type: {CORE}#pool.v1}}\n"
    );
    let mut archive = tar::Builder::new(flate2::write::GzEncoder::new(
        Vec::new(),
        flate2::Compression::default(),
    ));
    for (name, content) in [
        ("conduit.yaml", manifest),
        ("a.yaml", framed),
        ("b.yaml", authored),
    ] {
        let mut header = tar::Header::new_ustar();
        header.set_size(content.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive
            .append_data(&mut header, name, content.as_bytes())
            .unwrap();
    }
    let bytes = archive.into_inner().unwrap().finish().unwrap();
    let digest = conduit_core::packages::bytes_digest(&bytes);
    fs::write(server.directory.path().join("sample.tar.gz"), bytes).unwrap();
    let output = server.reject("install", &format!("./sample.tar.gz#{digest}"));
    assert_line(
        &output,
        "./sample.tar.gz(b.yaml):3:14: error: ",
        "[NAM-027; SCT-005 auditor]",
    );
    assert!(has_reply_location(
        &output,
        "NAM-027",
        "/templates/interfaces/auditor"
    ));
}

#[test]
fn located_lines_echo_no_secret_value() {
    let literal = "synthetic-value-for-redaction";
    let fixtures =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../conformance/language/sectioned");
    let mut disclosed = Vec::new();
    let mut missing_locations = Vec::new();
    let mut missing_rules = Vec::new();
    for form in ["framed", "sectioned", "long-chain"] {
        let fixture = fixtures.join(format!("secret-{form}"));
        let [manifest, contract, authored] = ["conduit.json", "contracts.json", "authored.json"]
            .map(|name| fs::read_to_string(fixture.join(name)).unwrap());
        let mut server = Server::start();
        let directory = server.directory.path().to_path_buf();
        let project = server.project(&[
            ("conduit.json", &manifest),
            ("contracts.json", &contract),
            ("authored.json", &authored),
        ]);
        let submit = server.reject("submit", "project");

        let authored: Value = serde_json::from_str(&authored).unwrap();
        let mut documents = vec![serde_json::from_str::<Value>(&contract).unwrap()];
        if let Value::Array(framed) = authored {
            documents.extend(framed);
        } else {
            documents.push(authored);
        }
        let arguments = json!({
            "manifest": serde_json::from_str::<Value>(&manifest).unwrap(),
            "documents": documents,
        });
        let argument_file = directory.join("arguments.json");
        fs::write(&argument_file, arguments.to_string()).unwrap();
        let wire = Server::command(&directory, "call")
            .args(["management.register.v1", "--arguments-file"])
            .arg(argument_file)
            .stdout(Stdio::piped())
            .output()
            .unwrap();

        let archive = fs::read(fixture.join("package.tar.gz")).unwrap();
        let digest = conduit_core::packages::bytes_digest(&archive);
        fs::write(directory.join("package.tar.gz"), archive).unwrap();
        let install = server.reject("install", &format!("package.tar.gz#{digest}"));

        let framed_pointer = "/objects/auditor--t/spec/config/sample";
        if form == "framed" {
            if !has_reply_location(&wire, "INS-014", framed_pointer) {
                missing_locations.push("framed wire registration");
            }
        } else {
            if !has_reply_location(&submit, "INS-014", "/interfaces/auditor/config/sample")
                || !String::from_utf8_lossy(&submit.stderr)
                    .contains("[INS-014; SCT-006 auditor--t]")
            {
                missing_locations.push("sectioned submit");
            }
            if !has_reply_location(
                &install,
                "SCH-019",
                "/templates/interfaces/auditor/inputs/example/default",
            ) {
                missing_locations.push("sectioned install");
            }
        }
        server.child.kill().unwrap();
        server.child.wait().unwrap();
        let register = Server::command(&directory, "register")
            .arg(project)
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        if !has_reply_location(&register, "INS-014", framed_pointer) {
            missing_locations.push("register configuration field");
        }
        let records = Server::command(&directory, "records")
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        assert!(
            records.status.success() && !records.stdout.is_empty(),
            "{form} records"
        );
        for (operation, rule, output) in [
            ("submit", "INS-014", submit),
            (
                "wire register",
                if form == "framed" {
                    "INS-014"
                } else {
                    "SUB-005"
                },
                wire,
            ),
            ("install", "SCH-019", install),
            ("register", "INS-014", register),
        ] {
            assert!(
                !output.status.success(),
                "{form} {operation} unexpectedly succeeded"
            );
            if !String::from_utf8_lossy(&output.stdout).contains(rule) {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let category = if stderr.contains("cannot load project") {
                    "load"
                } else if stderr.contains("did not succeed") {
                    "reply"
                } else {
                    "other"
                };
                missing_rules.push((form, operation, rule, category));
            }
            if captured_value(&output, literal) {
                disclosed.push((form, operation));
            }
        }
        if captured_value(&records, literal) {
            disclosed.push((form, "records"));
        }
    }
    assert!(
        disclosed.is_empty() && missing_rules.is_empty(),
        "withheld value appeared in {disclosed:?}; missing rules {missing_rules:?}"
    );
    assert!(
        missing_locations.is_empty(),
        "authored location absent in {missing_locations:?}"
    );
}

fn captured_value(output: &Output, value: &str) -> bool {
    [&output.stdout, &output.stderr]
        .into_iter()
        .any(|bytes| String::from_utf8_lossy(bytes).contains(value))
}
