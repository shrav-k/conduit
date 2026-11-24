use super::{Fixture, deadline, groups, id, invoke, log, outcome, repository, run_part};
use conduit_core::{Input, packages::bytes_digest, step_owned};
use conduit_shell::{MissingExecutor, Shell, boundary, local_machine};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path, time::Duration};

const ADDRESS: &str = "example.org/test/platform-resource";
const VERSION: &str = "1.0.0";
const API: &str = "github.com/shrav-k/conduit/v0";
const CORE: &str = "github.com/shrav-k/conduit/core";

struct Archive {
    pair: String,
    bytes: Vec<u8>,
    digest: String,
}

fn host_pair() -> String {
    let machine = local_machine(BTreeMap::new()).unwrap();
    format!(
        "{}/{}",
        machine["labels"]["kubernetes.io/os"].as_str().unwrap(),
        machine["labels"]["kubernetes.io/arch"].as_str().unwrap()
    )
}

impl Archive {
    fn new() -> Self {
        let pair = host_pair();
        let other = if pair == "linux/arm64" {
            "darwin/arm64"
        } else {
            "linux/arm64"
        };
        Self::for_platforms([pair.as_str(), other])
    }

    fn for_platforms(platforms: [&str; 2]) -> Self {
        let pair = host_pair();
        let retire = format!("{CORE}#retire-hook.v1");
        let manifest = json!({
            "apiVersion":API, "address":ADDRESS, "version":VERSION, "exports":["service.v1"],
            "implementations":{"server":{"command":["bin/server"], "platforms":platforms,
                "implements":[retire], "environment":{"TEST_PROVIDER_DIR":"/directory",
                    "FIXTURE_EXECUTABLE":"/executable"}}}
        });
        let contract = json!({
            "apiVersion":API, "kind":"Type", "metadata":{"name":"service.v1"},
            "spec":{"family":"Resource", "requires":[retire],
                "implementation":{retire:"#server"},
                "schema":{"type":"object","required":["directory","executable"],
                    "properties":{"directory":{"type":"string"},"executable":{"type":"string"}}}}
        });
        let manifest = serde_json::to_vec(&manifest).unwrap();
        let contract = serde_json::to_vec(&contract).unwrap();
        let gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        let mut tar = tar::Builder::new(gzip);
        let server = b"#!/bin/sh\nexec \"$FIXTURE_EXECUTABLE\" --exact scripted_runtime_provider --ignored --nocapture\n";
        let mut append = |path: &str, contents: &[u8], mode| {
            let mut header = tar::Header::new_ustar();
            header.set_mode(mode);
            header.set_uid(0);
            header.set_gid(0);
            header.set_mtime(0);
            header.set_size(contents.len() as u64);
            tar.append_data(&mut header, path, contents).unwrap();
        };
        append("conduit.json", &manifest, 0o644);
        append("type.json", &contract, 0o644);
        for platform in platforms {
            let contents: &[u8] = if platform == pair {
                server
            } else {
                b"#!/bin/sh\nexit 1\n"
            };
            append(&format!("platforms/{platform}/bin/server"), contents, 0o755);
        }
        let bytes = tar.into_inner().unwrap().finish().unwrap();
        Self {
            pair,
            digest: bytes_digest(&bytes),
            bytes,
        }
    }

    fn kept_path(&self, root: &Path) -> std::path::PathBuf {
        root.join("store/packages")
            .join(self.digest.strip_prefix("sha256:").unwrap())
    }

    fn install(&self, fixture: &Fixture) {
        let source = fixture.temp.path().join("source archive.tar.gz");
        fs::write(&source, &self.bytes).unwrap();
        let reply = invoke(
            &fixture.shell,
            &MissingExecutor,
            "management.install.v1",
            json!({"address":ADDRESS,"version":VERSION,"digest":self.digest,
                "fetch":url::Url::from_file_path(source).unwrap().to_string()}),
        );
        assert!(
            !reply["isError"].as_bool().unwrap(),
            "archive install failed"
        );
    }

    fn install_legacy(&self, root: &Path, shell: Shell) -> Shell {
        let input: Input = serde_json::from_value(json!({
            "kind":"install-archive","snapshots":[],"builtin":shell.context.builtin,
            "archive":{"name":"platforms-resource.tar.gz","address":ADDRESS,"version":VERSION,
                "digest":self.digest,"directory":"file:///archive/platform-resource/",
                "completion":{"status":"complete","bytes":self.bytes}}
        }))
        .unwrap();
        let transition =
            step_owned((*shell.state().unwrap()).clone(), input, &shell.context).unwrap();
        assert_eq!(transition.reply["changed"], true);
        let config = shell.config.clone();
        drop(shell);
        let directory = root.join("store");
        let state_file = directory.join("state.json");
        let mut stored: Value = serde_json::from_slice(&fs::read(&state_file).unwrap()).unwrap();
        stored["state"]["installed_packages"] = json!(transition.state.installed_packages);
        fs::write(state_file, serde_json::to_vec(&stored).unwrap()).unwrap();
        assert!(!self.kept_path(root).exists());
        Shell::open(
            &directory,
            boundary::context(&repository()).unwrap(),
            config,
        )
        .unwrap()
    }

    fn register(&self, fixture: &Fixture, runtime: Option<&str>) {
        let reply = self.register_project(fixture, runtime);
        assert_eq!(
            reply["accepted"], true,
            "platform Resource registration failed"
        );
    }

    fn register_project(&self, fixture: &Fixture, runtime: Option<&str>) -> Value {
        let directory = fixture.temp.path().join("local-service");
        fs::create_dir(&directory).unwrap();
        fs::write(directory.join("mode"), "success").unwrap();
        let mut resource = json!({"apiVersion":API,"kind":"Resource","metadata":{"name":"service"},
            "from":"platform-template","owner":"system"});
        if let Some(runtime) = runtime {
            resource["runtime"] = json!(runtime);
        }
        let state = fixture.shell.state().unwrap();
        let root = id(&fixture.shell, "root");
        let route = state
            .objects
            .iter()
            .find(|o| o["local_name"] == "management" && o["consumer"] == root)
            .unwrap();
        fixture.shell.submit(json!({"kind":"register-project","snapshots":[],"builtin":fixture.shell.context.builtin,
            "project":{"manifest":{"apiVersion":API,"address":"example.org/test/platform-project",
                "packages":{ADDRESS:{"version":VERSION,"digest":self.digest}}},
                "documents":[{"apiVersion":API,"kind":"Resource","metadata":{"name":"platform-template"},
                    "spec":{"type":format!("{ADDRESS}#service.v1"),"config":{
                        "directory":directory,"executable":std::env::current_exe().unwrap()}}},resource]},
            "request":{"sender":root,"caller":root,"conduit":route["id"],"target":route["target"],
                "operation":format!("{CORE}#register.v1")}}), &MissingExecutor).unwrap()
    }
}

fn fixture(run_v2: bool, archive: Option<&Archive>) -> Fixture {
    Fixture::with_store(
        "success",
        Duration::from_secs(5),
        run_v2,
        |root, shell| match archive {
            Some(archive) => archive.install_legacy(root, shell),
            None => shell,
        },
    )
}

fn service_request(fixture: &Fixture, tool: &str) -> Value {
    invoke(
        &fixture.shell,
        &fixture.runner,
        tool,
        json!({"instance":{"kind":"Resource","name":"service"}}),
    )
}

fn provider_calls(fixture: &Fixture) -> Vec<Value> {
    log(fixture.temp.path())
        .into_iter()
        .filter(|message| message["method"] == "tools/call")
        .map(|message| message["params"].clone())
        .collect()
}

#[test]
fn placement_rejects_platform_labels_from_different_pairs() {
    let pair = host_pair();
    let (os, arch) = pair.split_once('/').unwrap();
    let other_os = if os == "linux" { "darwin" } else { "linux" };
    let other_arch = if arch == "arm64" { "amd64" } else { "arm64" };
    let first = format!("{os}/{other_arch}");
    let second = format!("{other_os}/{arch}");
    let archive = Archive::for_platforms([&first, &second]);
    let fixture = fixture(false, None);
    archive.install(&fixture);
    let reply = archive.register_project(&fixture, None);
    assert_eq!(
        reply["accepted"], false,
        "crossed platform labels must reject placement"
    );
    let diagnostics = reply["diagnostics"].as_array().unwrap();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0]["ruleId"], "PLC-007");
    assert_eq!(
        diagnostics[0]["message"]["text"],
        format!("service: server lists platforms {first}, {second}; node local is {pair}")
    );
    assert!(
        !fixture
            .shell
            .state()
            .unwrap()
            .objects
            .iter()
            .any(|object| object["name"] == "service")
    );
}

#[test]
fn run_v2_without_a_url_ends_unknown_without_a_bridge() {
    let archive = Archive::new();
    let fixture = Fixture::with_store("no-url", Duration::from_secs(5), true, |_, shell| shell);
    archive.install(&fixture);
    archive.register(&fixture, Some("provider"));
    let groups_before = groups(&fixture).len();
    let reply = service_request(&fixture, "system.set-up.v1");
    assert_eq!(outcome(&fixture.shell, &reply), "unknown");

    let subject = id(&fixture.shell, "service");
    let state = fixture.shell.state().unwrap();
    let object = state
        .objects
        .iter()
        .find(|object| object["id"] == subject)
        .unwrap();
    assert_eq!(
        run_part(&object["status"])["outcome"],
        "unknown",
        "the run part must be unknown without a URL"
    );
    let ended = state
        .records
        .iter()
        .rev()
        .find(|record| {
            record["type"] == "provider-call"
                && record["subject"] == subject
                && record["data"]["phase"] == "ended"
        })
        .expect("provider call must have an end record");
    assert_eq!(ended["data"]["operation"], format!("{CORE}#run.v2"));
    assert_eq!(ended["data"]["outcome"], "unknown");
    assert_eq!(
        groups(&fixture).len(),
        groups_before,
        "the shell must launch no bridge without a URL"
    );
    let gate_count = fixture.temp.path().join("gate-count");
    deadline(|| gate_count.is_file());
    assert_eq!(fs::read_to_string(gate_count).unwrap().trim(), "0");
}

#[test]
fn run_v2_sends_the_pair_program_and_the_kept_archive() {
    let archive = Archive::new();
    let fixture = fixture(true, None);
    archive.install(&fixture);
    archive.register(&fixture, Some("provider"));
    let reply = service_request(&fixture, "system.set-up.v1");
    assert_eq!(
        outcome(&fixture.shell, &reply),
        "succeeded",
        "platform provider setup"
    );
    let calls = provider_calls(&fixture);
    assert_eq!(
        calls
            .iter()
            .map(|call| call["name"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["inspect-run.v1", "run.v2"]
    );
    let inputs = &calls[1]["arguments"];
    assert_eq!(
        inputs["program"],
        format!("platforms/{}/bin/server", archive.pair)
    );
    assert_eq!(inputs["item"]["command"][0], "bin/server");
    assert_eq!(inputs["archive"]["digest"], archive.digest);
    assert_eq!(inputs["archive"]["size"], archive.bytes.len());
    assert!(
        inputs["archive"]["urls"]
            == json!([
                url::Url::from_file_path(archive.kept_path(fixture.temp.path()))
                    .unwrap()
                    .to_string()
            ])
    );
    assert_eq!(inputs["identity"]["files"].as_object().unwrap().len(), 6);
    assert_eq!(
        inputs["identity"]["environment"].as_object().unwrap().len(),
        2
    );
    assert!(
        fs::read_to_string(fixture.temp.path().join("gate-count"))
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap()
            > 0
    );

    let subject = id(&fixture.shell, "service");
    let state = fixture.shell.state().unwrap();
    let records: Vec<_> = state
        .records
        .iter()
        .filter(|record| {
            record["type"] == "provider-call"
                && record["data"]["operation"] == format!("{CORE}#run.v2")
        })
        .collect();
    assert_eq!(records.len(), 2);
    assert!(records.iter().all(|record| record["subject"] == subject));
    let recorded = &records[0]["data"]["inputs"];
    assert_eq!(recorded["archive"], json!({"digest":archive.digest}));
    assert!(recorded.get("identity").is_none() && recorded.get("secrets").is_none());
    assert_eq!(records[1]["data"]["outcome"], "succeeded");
}

fn assert_refused_setup(fixture: &Fixture, reason: &str) -> String {
    let reply = service_request(fixture, "system.set-up.v1");
    assert_eq!(outcome(&fixture.shell, &reply), "failed");
    let state = fixture.shell.state().unwrap();
    let subject = id(&fixture.shell, "service");
    let object = state
        .objects
        .iter()
        .find(|object| object["id"] == subject)
        .unwrap();
    let part = object["status"]["setup"]["parts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|part| part["name"] == "run")
        .unwrap();
    let message = assert_part_diagnostic(part, reason);
    let calls = provider_calls(fixture);
    assert!(!calls.is_empty());
    assert!(calls.iter().all(|call| call["name"] == "inspect-run.v1"));
    message.to_owned()
}

fn assert_part_diagnostic<'a>(part: &'a Value, reason: &str) -> &'a str {
    assert_eq!(part["outcome"], "failed");
    let diagnostics = part["diagnostics"]
        .as_array()
        .expect("run part needs its PRV-007 diagnostic");
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0]["ruleId"], "PRV-007");
    let message = diagnostics[0]["message"]["text"].as_str().unwrap();
    assert!(
        message.contains("server") && message.contains("provider") && message.ends_with(reason)
    );
    message
}

#[test]
fn run_v2_is_never_sent_to_a_run_v1_provider_or_without_a_kept_archive() {
    let archive = Archive::new();
    let missing_operation =
        "the provider does not implement run.v2; its type does not list it under requires";
    let v1 = fixture(false, None);
    archive.install(&v1);
    archive.register(&v1, Some("provider"));
    assert_refused_setup(&v1, missing_operation);
    let _ = service_request(&v1, "system.retire.v1");
    let state = v1.shell.state().unwrap();
    let retirement = state
        .records
        .iter()
        .find(|r| r["data"]["action"] == "retirement-ended")
        .unwrap();
    let part = retirement["data"]["parts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|part| part["name"] == "hook-run")
        .unwrap();
    assert_part_diagnostic(part, missing_operation);
    assert!(
        provider_calls(&v1)
            .iter()
            .all(|call| matches!(call["name"].as_str(), Some("inspect-run.v1" | "stop.v1")))
    );

    let legacy = fixture(true, Some(&archive));
    archive.register(&legacy, Some("provider"));
    let message = assert_refused_setup(&legacy, "no kept archive");
    assert!(message.starts_with(&format!(
        "server on provider provider: run.v2: package {ADDRESS} {VERSION}: archive "
    )));
    assert!(message.contains(archive.kept_path(legacy.temp.path()).to_str().unwrap()));
    assert!(!archive.kept_path(legacy.temp.path()).exists());

    let kept = fixture(true, None);
    archive.install(&kept);
    archive.register(&kept, Some("provider"));
    let path = archive.kept_path(kept.temp.path());
    fs::remove_file(&path).unwrap();
    assert_refused_setup(&kept, "absent");
    assert!(!path.exists());
    fs::create_dir(&path).unwrap();
    assert_refused_setup(&kept, "found directory, expected a regular file");
    assert!(path.is_dir());
}

#[test]
fn own_runner_runs_the_program_for_the_nodes_pair() {
    let archive = Archive::new();
    let fixture = fixture(false, None);
    archive.install(&fixture);
    archive.register(&fixture, None);
    let reply = service_request(&fixture, "system.set-up.v1");
    assert_eq!(
        outcome(&fixture.shell, &reply),
        "succeeded",
        "platform own-runner setup"
    );
    assert!(
        log(&fixture.temp.path().join("local-service"))
            .iter()
            .any(|m| m["method"] == "initialize")
    );
}
