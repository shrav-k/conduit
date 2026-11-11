//! I4 change routes through real registration, management requests and held packages.
mod common;
use conduit_shell::{Config, Execution, Executor, MissingExecutor, Result, Shell, boundary};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::PathBuf, sync::Mutex};

#[derive(Default)]
struct Observed {
    notices: Mutex<Vec<Value>>,
    stops: Mutex<Vec<Value>>,
    confirm: bool,
}
impl Executor for Observed {
    fn tools_changed(&self, v: &[Value]) -> Result<()> {
        self.notices.lock().unwrap().extend_from_slice(v);
        Ok(())
    }
    fn builtin_runner(
        &self,
        v: &Value,
        _: Option<&conduit_shell::RunIdentity>,
    ) -> Result<Execution> {
        if v["operation"].as_str().unwrap().ends_with("#stop.v1") {
            self.stops.lock().unwrap().push(v.clone());
            return Ok(Execution::SetupReport(
                json!({"kind":"stop","outcome":if self.confirm {"succeeded"} else {"unknown"}}),
            ));
        }
        Ok(Execution::Missing)
    }
}
fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn id(shell: &Shell, kind: &str, name: &str, template: bool) -> Value {
    shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| o["kind"] == kind && o["name"] == name && o.get("spec").is_some() == template)
        .unwrap()["id"]
        .clone()
}
fn object(shell: &Shell, kind: &str, name: &str, template: bool) -> Value {
    let key = id(shell, kind, name, template);
    shell
        .state()
        .unwrap()
        .objects
        .into_iter()
        .find(|o| o["id"] == key)
        .unwrap()
}
fn invoke(shell: &Shell, executor: &dyn Executor, tool: &str, args: Value) -> Value {
    shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":id(shell,"Interface","root",false)},"tool":tool,"arguments":args}}),executor).unwrap()
}
fn success(v: Value) -> Value {
    assert_eq!(v["isError"], false, "{v}");
    v
}
fn tools(shell: &Shell, name: &str) -> Vec<String> {
    let list=shell.submit(json!({"kind":"list-tools","identity":{"kind":"instance","id":id(shell,"Agent",name,false)}}),&MissingExecutor).unwrap();
    list["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap().to_string())
        .collect()
}
fn fixture(shell: &Shell, extra: bool) {
    shell.submit(json!({"kind":"first-start","machine":conduit_shell::local_machine(BTreeMap::new()).unwrap()}),&MissingExecutor).unwrap();
    let root = id(shell, "Interface", "root", false);
    let route = shell
        .state()
        .unwrap()
        .objects
        .into_iter()
        .find(|o| o["consumer"] == root && o["local_name"] == "management")
        .unwrap();
    let spec = conduit_loader::Specification::read(&repo().join("spec")).unwrap();
    let mut project =
        boundary::project(&repo().join("conformance/fixtures/s1/project"), &spec).unwrap();
    if extra {
        let mut doc = project
            .documents
            .iter()
            .find(|d| d["kind"] == "Agent" && d.get("from").is_some())
            .unwrap()
            .clone();
        doc["metadata"]["name"] = json!("agent-c");
        doc["from"] = json!("reader-agent");
        project.documents.push(doc);
    }
    common::install(
        shell,
        &repo(),
        common::snapshots(&repo()).unwrap(),
        &mut project,
    );
    let v=shell.submit(json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,"project":project,"request":{"sender":root,"caller":root,"conduit":route["id"],"target":route["target"],"operation":format!("{}#register.v1",shell.context.core_package)}}),&MissingExecutor).unwrap();
    assert_eq!(v["accepted"], true, "{v}");
    success(invoke(
        shell,
        &MissingExecutor,
        "access.open.v1",
        json!({"conduit":"root-workspace"}),
    ));
    for name in if extra {
        vec!["agent-a", "agent-b", "agent-c"]
    } else {
        vec!["agent-a", "agent-b"]
    } {
        success(invoke(
            shell,
            &MissingExecutor,
            "management.set-up.v1",
            json!({"instance":{"kind":"Agent","name":name}}),
        ));
    }
}
fn replacement(shell: &Shell, executor: &dyn Executor, name: &str, operations: Value) {
    let o = object(shell, "Conduit", name, true);
    let mut template = json!({"apiVersion":o["apiVersion"],"kind":"Conduit","metadata":o["metadata"],"spec":o["spec"]});
    // Stored objects keep the document's apiVersion in their authored field only
    // on some boundaries. Read the canonical name from the loader.
    template["apiVersion"] = json!(
        conduit_loader::Specification::read(&repo().join("spec"))
            .unwrap()
            .names()
            .api_version
    );
    template["metadata"]["name"] = json!(name);
    template["spec"]["operations"] = operations;
    let spec = conduit_loader::Specification::read(&repo().join("spec")).unwrap();
    let project =
        boundary::project(&repo().join("conformance/fixtures/s1/project"), &spec).unwrap();
    success(invoke(
        shell,
        executor,
        "management.replace.v1",
        json!({"template":template,"manifest":project.manifest,"revision":o["revision"]}),
    ));
}
fn replace_nested(shell: &Shell, executor: &dyn Executor, name: &str) {
    let consumer = id(shell, "Agent", name, false);
    let path = shell
        .state()
        .unwrap()
        .objects
        .into_iter()
        .find(|o| {
            o["kind"] == "Conduit"
                && o["consumer"].get("id").unwrap_or(&o["consumer"]) == &consumer
                && o["nested"]["name"] == "project"
        })
        .unwrap();
    success(invoke(
        shell,
        executor,
        "access.close.v1",
        json!({"conduit":path["name"]}),
    ));
    let v = success(invoke(
        shell,
        executor,
        "management.register-replacement.v1",
        json!({"instance":{"kind":"Agent","name":name},"nested":"project"}),
    ));
    success(invoke(
        shell,
        executor,
        "access.open.v1",
        json!({"conduit":v["structuredContent"]["object"]["name"]}),
    ));
}
fn assert_notices(shell: &Shell, observer: &Observed, name: &str) {
    let notices = std::mem::take(&mut *observer.notices.lock().unwrap());
    assert!(!notices.is_empty(), "no list-change notice for {name}");
    let expected = json!({"method":"notifications/tools/list_changed","consumer":id(shell,"Agent",name,false)});
    assert!(
        notices.iter().all(|notice| notice == &expected),
        "{notices:?}"
    );
}
#[test]
fn t4_replace_keeps_other_holders_and_tool_names_with_private_notices() {
    let temp = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &temp.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    fixture(&shell, true);
    let observer = Observed::default();
    let old_c = tools(&shell, "agent-c");
    let old_b = tools(&shell, "agent-b");
    let read = json!([
        "example.org/s1/filesystem#file.read.v1",
        "example.org/s1/filesystem#directory.list.v1"
    ]);
    let mut write = read.clone();
    write
        .as_array_mut()
        .unwrap()
        .push(json!("example.org/s1/filesystem#file.write.v1"));
    replacement(&shell, &observer, "directory-read", write);
    assert!(observer.notices.lock().unwrap().is_empty());
    replace_nested(&shell, &observer, "agent-a");
    assert!(tools(&shell, "agent-a").contains(&"project.file.write.v1".into()));
    assert_eq!(tools(&shell, "agent-c"), old_c);
    assert_eq!(tools(&shell, "agent-b"), old_b);
    assert_notices(&shell, &observer, "agent-a");
    replacement(&shell, &observer, "directory-read-write", read);
    replace_nested(&shell, &observer, "agent-b");
    assert!(!tools(&shell, "agent-b").contains(&"project.file.write.v1".into()));
    assert_notices(&shell, &observer, "agent-b");
    replace_nested(&shell, &observer, "agent-c");
    assert!(tools(&shell, "agent-c").contains(&"project.file.write.v1".into()));
    assert_notices(&shell, &observer, "agent-c");
}

#[test]
fn unregister_run_templates_removes_them_without_stopping_instances() {
    let temp = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &temp.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    fixture(&shell, false);
    let spec = conduit_loader::Specification::read(&repo().join("spec")).unwrap();
    let project =
        boundary::project(&repo().join("conformance/fixtures/s1/project"), &spec).unwrap();
    let mut manifest = project.manifest.unwrap();
    manifest["address"] = json!("example.org/test/run-templates");
    let hook = format!("{}#retire-hook.v1", shell.context.core_package);
    let file_operations = ["file.read.v1", "file.write.v1", "directory.list.v1"]
        .map(|name| format!("example.org/s1/filesystem#{name}"));
    let implements: Vec<_> = std::iter::once(hook.clone())
        .chain(file_operations.clone())
        .collect();
    manifest["implementations"] =
        json!({"run-hook":{"command":["/test/program"],"implements":implements}});
    let documents: Vec<_> = ["shared-directory", "writer-agent"]
        .iter()
        .map(|name| {
            let mut d = project
                .documents
                .iter()
                .find(|d| d["metadata"]["name"] == *name)
                .unwrap()
                .clone();
            d["metadata"]["name"] = json!(format!("{name}-run"));
            d["spec"].as_object_mut().unwrap().remove("conduits");
            d["spec"]["implementation"] = json!({hook.clone():"#run-hook"});
            if d["kind"] == "Resource" {
                for operation in &file_operations {
                    d["spec"]["implementation"][operation] = json!("#run-hook");
                }
            }
            d
        })
        .collect();
    let observer = Observed::default();
    success(invoke(
        &shell,
        &observer,
        "management.register.v1",
        json!({"manifest":manifest,"documents":documents}),
    ));
    for (kind, name) in [
        ("Resource", "shared-directory-run"),
        ("Agent", "writer-agent-run"),
    ] {
        let before = object(&shell, kind, name, true);
        assert!(
            before["resolved_implementations"]
                .as_object()
                .unwrap()
                .values()
                .any(|i| i["item"].get("command").is_some())
        );
        success(invoke(
            &shell,
            &observer,
            "management.unregister.v1",
            json!({"object":{"kind":kind,"name":name},"revision":before["revision"]}),
        ));
        assert!(
            shell
                .state()
                .unwrap()
                .objects
                .iter()
                .all(|o| o["id"] != before["id"])
        );
    }
    assert!(observer.stops.lock().unwrap().is_empty());
    assert_eq!(
        object(&shell, "Resource", "shared", false)["state"],
        "registered"
    );
    assert_eq!(object(&shell, "Agent", "agent-b", false)["state"], "ready");
}
#[test]
fn retirement_closes_targets_immediately_and_unregister_keeps_records() {
    unregister_retries_state_directory_deletion(false);
}

#[test]
fn reopen_retries_failed_state_directory_deletion_before_returning() {
    unregister_retries_state_directory_deletion(true);
}

fn unregister_retries_state_directory_deletion(retry_on_reopen: bool) {
    let temp = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &temp.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    fixture(&shell, false);
    let observer = Observed::default();
    let blocked = invoke(
        &shell,
        &observer,
        "management.retire.v1",
        json!({"instance":{"kind":"Pool","name":"workspace"}}),
    );
    assert_eq!(
        blocked["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "conflict"
    );
    let message = blocked["structuredContent"]["diagnostics"][0]["message"]["text"]
        .as_str()
        .unwrap();
    assert!(message.contains("Resource/shared") && message.contains("Resource/history"));
    let before = object(&shell, "Resource", "shared", false);
    // PRV-029/LIF-035: state survives retirement while shutdown is unconfirmed.
    let state_dir = temp
        .path()
        .join("store/instances")
        .join(before["id"].as_str().unwrap());
    std::fs::create_dir(&state_dir).unwrap();
    std::fs::write(state_dir.join("synthetic-state"), "test").unwrap();
    let conflict = invoke(
        &shell,
        &observer,
        "management.unregister.v1",
        json!({"object":{"kind":"Resource","name":"shared"},"revision":before["revision"]}),
    );
    assert_eq!(
        conflict["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "conflict"
    );
    // The setup is dispatched, though no npm process is run by this unit test.
    let _ = invoke(
        &shell,
        &observer,
        "root-workspace.set-up.v1",
        json!({"instance":{"kind":"Resource","name":"shared"}}),
    );
    let retired = invoke(
        &shell,
        &observer,
        "root-workspace.retire.v1",
        json!({"instance":{"kind":"Resource","name":"shared"}}),
    );
    assert_eq!(
        retired["_meta"][format!("{}outcome", shell.context.meta_prefix)],
        "unknown",
        "{retired}"
    );
    assert_eq!(
        object(&shell, "Resource", "shared", false)["state"],
        "retired"
    );
    assert!(state_dir.exists());
    assert!(
        !tools(&shell, "agent-a")
            .iter()
            .any(|n| n.starts_with("project."))
    );
    let invalid = invoke(
        &shell,
        &observer,
        "management.register-replacement.v1",
        json!({"instance":{"kind":"Agent","name":"agent-a"},"nested":"project"}),
    );
    assert_eq!(
        invalid["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "invalid"
    );
    assert_eq!(
        invalid["structuredContent"]["diagnostics"][0]["ruleId"],
        "CON-025"
    );
    let count = shell.state().unwrap().records.len();
    use std::os::unix::fs::PermissionsExt;
    // Fail a real recursive delete without relying on a fault-only implementation.
    std::fs::set_permissions(&state_dir, std::fs::Permissions::from_mode(0o500)).unwrap();
    success(invoke(
        &shell,
        &observer,
        "management.unregister.v1",
        json!({"object":{"kind":"Resource","name":"shared"},"revision":before["revision"]}),
    ));
    assert!(
        shell
            .state()
            .unwrap()
            .objects
            .iter()
            .all(|o| o["id"] != before["id"])
    );
    assert!(shell.state().unwrap().records.len() > count);
    assert!(
        state_dir.exists(),
        "failed deletion must leave a retry marker"
    );
    drop(shell);
    if retry_on_reopen {
        std::fs::set_permissions(&state_dir, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let shell = Shell::open(
        &temp.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    if retry_on_reopen {
        // PRV-029/LIF-035 and DL16: reopening itself must retry cleanup.
        assert!(
            !state_dir.exists(),
            "reopen must delete the directory before returning"
        );
    } else {
        assert!(state_dir.exists());
        std::fs::set_permissions(&state_dir, std::fs::Permissions::from_mode(0o700)).unwrap();
        shell
            .submit(json!({"kind":"export-records"}), &MissingExecutor)
            .unwrap();
        assert!(
            !state_dir.exists(),
            "a later commit retries the failed deletion"
        );
    }
    assert_eq!(observer.stops.lock().unwrap().len(), 2);
    // No setup was dispatched for history: retirement removes it with no stop.
    success(invoke(
        &shell,
        &observer,
        "root-workspace.retire.v1",
        json!({"instance":{"kind":"Resource","name":"history"}}),
    ));
    assert_eq!(observer.stops.lock().unwrap().len(), 2);
}
#[test]
fn t5_public_install_is_live_and_existing_instances_keep_versions() {
    let temp = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &temp.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    fixture(&shell, false);
    let observer = Observed::default();
    let archive = repo().join("conformance/fixtures/s1/archives/filesystem-0.2.0.tar.gz");
    let digest = conduit_core::packages::bytes_digest(&std::fs::read(&archive).unwrap());
    let args = boundary::install_arguments(&format!("{}#{digest}", archive.display())).unwrap();
    let installed = success(invoke(
        &shell,
        &observer,
        "management.install.v1",
        args.clone(),
    ));
    let invocation = &installed["_meta"][format!("{}invocation", shell.context.meta_prefix)];
    assert!(shell.state().unwrap().records.iter().any(|r| r["type"]=="invocation-dispatched" && r["data"]["invocation"]==*invocation));
    let count = shell.state().unwrap().installed_packages.len();
    let mut again = args.clone();
    again["fetch"] = json!("file:///does-not-exist");
    success(invoke(&shell, &observer, "management.install.v1", again));
    assert_eq!(shell.state().unwrap().installed_packages.len(), count);
    let mut wrong = args.clone();
    wrong["digest"] = json!(format!("sha256:{}", "0".repeat(64)));
    let conflict = invoke(&shell, &observer, "management.install.v1", wrong);
    assert_eq!(
        conflict["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "conflict"
    );
    for (version, fetch, reason) in [
        ("0.3.0", "file:///no-such-conduit-package", "unavailable"),
        ("0.3.0", "https://example.invalid/package.tar.gz", "invalid"),
        ("0.3.0", "relative.tar.gz", "invalid"),
    ] {
        let mut input = args.clone();
        input["version"] = json!(version);
        input["fetch"] = json!(fetch);
        let before = shell.state().unwrap().invocations.len();
        let reply = invoke(&shell, &observer, "management.install.v1", input);
        assert_eq!(
            reply["_meta"][format!("{}reason", shell.context.meta_prefix)],
            reason,
            "{reply}"
        );
        assert_eq!(shell.state().unwrap().invocations.len(), before);
        if fetch.starts_with("https:") {
            assert!(
                reply["structuredContent"]["diagnostics"][0]["message"]["text"]
                    .as_str()
                    .unwrap()
                    .contains("unfinished")
            );
        }
    }
    // The compressed archive has the right digest but declares another version.
    let mut mismatch = args.clone();
    mismatch["version"] = json!("0.3.0");
    let before = shell.state().unwrap().invocations.len();
    let reply = invoke(&shell, &observer, "management.install.v1", mismatch);
    assert_eq!(
        reply["_meta"][format!("{}reason", shell.context.meta_prefix)],
        "invalid"
    );
    assert_eq!(shell.state().unwrap().invocations.len(), before);
    let spec = conduit_loader::Specification::read(&repo().join("spec")).unwrap();
    let project =
        boundary::project(&repo().join("conformance/fixtures/s1/project"), &spec).unwrap();
    let mut manifest = project.manifest.unwrap();
    let packages = manifest["packages"].as_object_mut().unwrap();
    let package = packages.get_mut("example.org/s1/filesystem").unwrap();
    package["version"] = json!("0.2.0");
    package["digest"] = json!(digest);
    let mut resource = project
        .documents
        .iter()
        .find(|d| d["kind"] == "Resource" && d["metadata"]["name"] == "shared-directory")
        .unwrap()
        .clone();
    resource["metadata"]["name"] = json!("upgraded-directory");
    let mut path = project
        .documents
        .iter()
        .find(|d| d["kind"] == "Conduit" && d["metadata"]["name"] == "directory-read-write")
        .unwrap()
        .clone();
    path["metadata"]["name"] = json!("upgraded-access");
    let mut agent = project
        .documents
        .iter()
        .find(|d| d["kind"] == "Agent" && d["metadata"]["name"] == "writer-agent")
        .unwrap()
        .clone();
    agent["metadata"]["name"] = json!("upgraded-agent");
    agent["spec"].as_object_mut().unwrap().remove("conduits");
    let api = spec.names().api_version.clone();
    let documents = json!([resource,path,agent,
        {"apiVersion":api,"kind":"Resource","metadata":{"name":"shared-new"},"from":"upgraded-directory","owner":"workspace","arguments":{"root":"/another/shared"}},
        {"apiVersion":api,"kind":"Agent","metadata":{"name":"new-consumer"},"from":"upgraded-agent","owner":"s1"},
        {"apiVersion":api,"kind":"Conduit","metadata":{"name":"upgraded"},"from":"upgraded-access","consumer":{"kind":"Agent","name":"new-consumer"},"target":{"kind":"Resource","name":"shared-new"}}]);
    success(invoke(
        &shell,
        &observer,
        "management.register.v1",
        json!({"manifest":manifest,"documents":documents}),
    ));
    success(invoke(
        &shell,
        &observer,
        "access.open.v1",
        json!({"conduit":"upgraded"}),
    ));
    assert_eq!(
        tools(&shell, "new-consumer"),
        vec![
            "upgraded.directory.list.v1",
            "upgraded.file.read.v1",
            "upgraded.file.write.v1"
        ]
    );
    for name in ["agent-a", "agent-b", "new-consumer"] {
        assert!(
            tools(&shell, name)
                .iter()
                .all(|n| !n.contains("append_file") && !n.contains("copy_file"))
        );
    }
    for (name, version) in [("shared", "0.1.0"), ("shared-new", "0.2.0")] {
        assert!(
            object(&shell, "Resource", name, false)["packages"]
                .as_array()
                .unwrap()
                .iter()
                .any(|p| p["address"] == "example.org/s1/filesystem" && p["version"] == version)
        );
    }
}

struct HookBoundary {
    intents: Mutex<Vec<Value>>,
    run_state: &'static str,
}
impl Executor for HookBoundary {
    fn builtin_runner(
        &self,
        intent: &Value,
        _: Option<&conduit_shell::RunIdentity>,
    ) -> Result<Execution> {
        self.intents.lock().unwrap().push(intent.clone());
        let op = intent["operation"].as_str().unwrap();
        Ok(Execution::SetupReport(if op.ends_with("#inspect-run.v1") {
            json!({"kind":"inspect-run","state":self.run_state})
        } else if op.ends_with("#run.v1") {
            json!({"kind":"run","outcome":"succeeded"})
        } else {
            json!({"kind":"stop","outcome":"failed"})
        }))
    }
    fn connect(&self, intent: &Value) -> Result<Execution> {
        self.intents.lock().unwrap().push(intent.clone());
        Ok(Execution::SetupReport(
            json!({"kind":"connected","part":intent["part"],"outcome":"succeeded"}),
        ))
    }
    fn dispatch(&self, intent: &Value) -> Result<Execution> {
        self.intents.lock().unwrap().push(intent.clone());
        Ok(Execution::Complete(
            json!({"kind":"result","result":{"isError":true,"_meta":{format!("{}outcome", boundary::context(&repo())?.meta_prefix):"failed"}}}),
        ))
    }
}
fn hook_resource(shell: &Shell, remote: bool) {
    fixture(shell, false);
    let spec = conduit_loader::Specification::read(&repo().join("spec")).unwrap();
    let project =
        boundary::project(&repo().join("conformance/fixtures/s1/project"), &spec).unwrap();
    let mut manifest = project.manifest.unwrap();
    manifest["address"] = json!("example.org/test/retirement");
    let hook = format!("{}#retire-hook.v1", shell.context.core_package);
    let operations = ["file.read.v1", "file.write.v1", "directory.list.v1"]
        .map(|n| format!("example.org/s1/filesystem#{n}"));
    let implements: Vec<_> = operations
        .iter()
        .cloned()
        .chain((!remote).then_some(hook.clone()))
        .collect();
    manifest["implementations"] =
        json!({"run":{"command":["/test/program"],"implements":implements}});
    if remote {
        manifest["implementations"]["hook"] = json!({"remote":{"type":"streamable-http","url":"https://example.invalid/hook","headers":[]},"implements":[hook.clone()]});
    }
    let mut template = project
        .documents
        .iter()
        .find(|d| d["metadata"]["name"] == "shared-directory")
        .unwrap()
        .clone();
    template["metadata"]["name"] = json!("hook-directory");
    template["spec"]["implementation"] = json!({hook:if remote {"#hook"} else {"#run"}});
    for op in operations {
        template["spec"]["implementation"][op] = json!("#run");
    }
    success(invoke(
        shell,
        &MissingExecutor,
        "management.register.v1",
        json!({"manifest":manifest,"documents":[template,{"apiVersion":spec.names().api_version,"kind":"Resource","metadata":{"name":"hook-resource"},"from":"hook-directory","owner":"workspace","arguments":{"root":"/test"}}]}),
    ));
    let reply = invoke(
        shell,
        &MissingExecutor,
        "root-workspace.set-up.v1",
        json!({"instance":{"kind":"Resource","name":"hook-resource"}}),
    );
    assert_eq!(reply["isError"], true);
}
fn retire_hook_resource(shell: &Shell, boundary: &HookBoundary) -> Value {
    invoke(
        shell,
        boundary,
        "root-workspace.retire.v1",
        json!({"instance":{"kind":"Resource","name":"hook-resource"}}),
    )
}
fn retirement_parts(shell: &Shell) -> Value {
    shell
        .state()
        .unwrap()
        .invocations
        .iter()
        .rev()
        .find(|v| v.get("retirement").is_some())
        .unwrap()["retirement"]["parts"]
        .clone()
}
#[test]
fn remote_retire_hook_connects_without_preparing_unrelated_run() {
    let temp = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &temp.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    hook_resource(&shell, true);
    let boundary = HookBoundary {
        intents: Mutex::new(vec![]),
        run_state: "unknown",
    };
    retire_hook_resource(&shell, &boundary);
    let intents = boundary.intents.lock().unwrap();
    assert_eq!(
        intents
            .iter()
            .map(|v| v["kind"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["connect", "dispatch", "builtin-runner"]
    );
    assert_eq!(
        intents[0]["implementation"]["item"]["remote"]["url"],
        "https://example.invalid/hook"
    );
    assert_eq!(
        intents[2]["operation"],
        format!("{}#stop.v1", shell.context.core_package)
    );
}
#[test]
fn successful_run_for_retire_hook_counts_as_an_effectful_part() {
    let temp = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &temp.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    hook_resource(&shell, false);
    let boundary = HookBoundary {
        intents: Mutex::new(vec![]),
        run_state: "stopped",
    };
    let reply = retire_hook_resource(&shell, &boundary);
    assert_eq!(
        reply["_meta"][format!("{}outcome", shell.context.meta_prefix)],
        "unknown"
    );
    assert!(
        retirement_parts(&shell)
            .as_array()
            .unwrap()
            .contains(&json!({"name":"hook-run","outcome":"succeeded","effect":true}))
    );
}
#[test]
fn unknown_run_makes_retire_hook_unavailable_without_an_effect() {
    let temp = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &temp.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    hook_resource(&shell, false);
    let boundary = HookBoundary {
        intents: Mutex::new(vec![]),
        run_state: "unknown",
    };
    let reply = retire_hook_resource(&shell, &boundary);
    assert_eq!(
        reply["_meta"][format!("{}outcome", shell.context.meta_prefix)],
        "failed"
    );
    assert_eq!(
        retirement_parts(&shell),
        json!([{"name":"hook","outcome":"unavailable","effect":false},{"name":"stop","outcome":"failed","effect":true}])
    );
    let intents = boundary.intents.lock().unwrap();
    assert_eq!(intents.len(), 2);
    assert!(intents.iter().all(|v| v["kind"] == "builtin-runner"));
}

#[test]
fn missing_object_diagnostic_ignores_unrelated_additional_inputs() {
    let temp = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &temp.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    fixture(&shell, false);
    for extra in [json!("unrelated"), Value::Null, json!({})] {
        let reply = invoke(
            &shell,
            &MissingExecutor,
            "management.inspect.v1",
            json!({"object":{"kind":"Agent","name":"missing-agent"},"conduit":extra}),
        );
        let diagnostic = &reply["structuredContent"]["diagnostics"][0];
        assert_eq!(diagnostic["ruleId"], "OPS-029", "{reply}");
        assert_eq!(
            diagnostic["message"]["text"],
            "Agent/missing-agent does not exist. A closed Conduit and a removed instance no longer exist.",
            "{reply}"
        );
    }
}

#[test]
fn missing_object_diagnostics_name_the_reference_and_explain_removal() {
    let temp = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &temp.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    fixture(&shell, false);
    let consumer = id(&shell, "Agent", "agent-b", false);
    let closed = shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| {
            o["kind"] == "Conduit"
                && o["nested"]["parent"] == consumer
                && o["nested"]["name"] == "project"
        })
        .unwrap()["name"]
        .clone();
    let closed_label = format!("Conduit/{}", closed.as_str().unwrap());
    success(invoke(
        &shell,
        &MissingExecutor,
        "access.close.v1",
        json!({"conduit":closed}),
    ));
    success(invoke(
        &shell,
        &MissingExecutor,
        "management.retire.v1",
        json!({"instance":{"kind":"Agent","name":"agent-a"}}),
    ));
    let revision = object(&shell, "Agent", "agent-b", false)["revision"].clone();
    for (tool, args, label) in [
        (
            "management.inspect.v1",
            json!({"object":{"kind":"Conduit","name":closed}}),
            closed_label.as_str(),
        ),
        (
            "access.close.v1",
            json!({"conduit":closed}),
            closed_label.as_str(),
        ),
        (
            "access.open.v1",
            json!({"conduit":closed}),
            closed_label.as_str(),
        ),
        (
            "management.unregister.v1",
            json!({"object":{"kind":"Conduit","name":closed},"revision":revision}),
            closed_label.as_str(),
        ),
        (
            "management.retire.v1",
            json!({"instance":{"kind":"Agent","name":"agent-a"}}),
            "Agent/agent-a",
        ),
        (
            "management.set-up.v1",
            json!({"instance":{"kind":"Agent","name":"never-here"}}),
            "Agent/never-here",
        ),
        (
            "management.declare-lost.v1",
            json!({"node":"missing-node"}),
            "Resource/missing-node",
        ),
        (
            "management.reconcile.v1",
            json!({"invocation":"00000000-0000-4000-8000-000000009999","occurred":false,"checked":"No invocation with this ID"}),
            "Invocation/00000000-0000-4000-8000-000000009999",
        ),
    ] {
        let reply = invoke(&shell, &MissingExecutor, tool, args);
        assert_eq!(
            reply["structuredContent"]["diagnostics"][0]["ruleId"], "OPS-029",
            "{reply}"
        );
        let text = reply["structuredContent"]["diagnostics"][0]["message"]["text"]
            .as_str()
            .unwrap();
        assert!(
            text.contains(label)
                && text.contains("does not exist")
                && text.contains("closed Conduit")
                && text.contains("removed instance"),
            "{text}"
        );
    }
}
