//! Scripted provider endings only. No machine provider or network is run.
use conduit_shell::{
    Config, Execution, Executor, MissingExecutor, Result, RunIdentity, Shell, boundary,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn id(shell: &Shell, name: &str) -> Value {
    shell
        .state()
        .unwrap()
        .objects
        .iter()
        .find(|o| o["name"] == name && o.get("spec").is_none())
        .unwrap()["id"]
        .clone()
}
fn invoke(shell: &Shell, executor: &dyn Executor, op: &str, args: Value) -> Value {
    shell.submit(json!({"kind":"invoke","request":{"identity":{"kind":"instance","id":id(shell,"root")},"tool":format!("{}.{op}",if op=="set-up.v1" {"system"} else {"management"}),"arguments":args}}),executor).unwrap()
}
fn fixture(shell: &Shell) {
    shell.submit(json!({"kind":"first-start","machine":conduit_shell::local_machine(BTreeMap::new()).unwrap()}),&MissingExecutor).unwrap();
    let mut project = conduit_core::packages::Project {
        manifest: Some(
            boundary::value(&repo().join("conformance/fixtures/machine-provider/manifest.json"))
                .unwrap(),
        ),
        documents: conduit_loader::read_file(
            &repo().join("conformance/fixtures/machine-provider/documents.json"),
        )
        .unwrap()
        .into_iter()
        .map(|d| d.root.to_json())
        .collect(),
        directory: Some("file:///fixtures/machines/".into()),
    };
    let api = shell.context.templates["root"]["apiVersion"].clone();
    project.documents.extend([
        json!({"apiVersion":api,"kind":"Type","metadata":{"name":"consumer.v1"},"spec":{"family":"Interface","schema":{"type":"object"}}}),
        json!({"apiVersion":api,"kind":"Interface","metadata":{"name":"consumer"},"spec":{"type":"#consumer.v1"}}),
        json!({"apiVersion":api,"kind":"Interface","metadata":{"name":"other"},"from":"consumer"}),
    ]);
    let s = shell.state().unwrap();
    let root = id(shell, "root");
    let path = s
        .objects
        .iter()
        .find(|o| o["consumer"] == root && o["local_name"] == "management")
        .unwrap();
    let result=shell.submit(json!({"kind":"register-project","snapshots":[],"builtin":shell.context.builtin,"project":project,"request":{"sender":root,"caller":root,"conduit":path["id"],"target":path["target"],"operation":format!("{}#register.v1",shell.context.core_package)}}),&MissingExecutor).unwrap();
    assert_eq!(result["accepted"], true, "{result}");
    assert_eq!(
        invoke(
            shell,
            &Ready,
            "set-up.v1",
            json!({"instance":{"kind":"Resource","name":"machines"}})
        )["isError"],
        false
    );
}
struct Ready;
impl Executor for Ready {
    fn builtin_runner(
        &self,
        _intent: &Value,
        _identity: Option<&RunIdentity>,
    ) -> Result<Execution> {
        Ok(Execution::SetupReport(
            json!({"kind":"inspect-run","state":"running"}),
        ))
    }
}
struct Script;
impl Executor for Script {
    fn provider_call(&self, intent: &Value, _identity: Option<&RunIdentity>) -> Result<Execution> {
        assert!(intent["provider"].is_string());
        assert_eq!(intent["provider"], intent["subject"]);
        for key in ["config", "revision", "invocation", "caller"] {
            assert!(intent.get(key).is_some());
        }
        Ok(Execution::SetupReport(
            json!({"kind":"enroll","outcome":"succeeded","outputs":{"action":{"command":"SHELL_JOIN_MARKER"},"message":"SHELL_JOIN_MARKER","carriesSecret":true,"expires":intent["inputs"]["expires"],"evidence":["fingerprint"]}}),
        ))
    }
}
fn expiry() -> String {
    (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339()
}
#[test]
fn missing_machine_executor_fails_request_without_embedding_error() {
    let dir = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &dir.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    fixture(&shell);
    let result = invoke(
        &shell,
        &MissingExecutor,
        "enroll-machine.v1",
        json!({"expires":expiry()}),
    );
    assert_eq!(
        result["_meta"][format!("{}outcome", shell.context.meta_prefix)],
        "failed"
    );
    assert!(result["structuredContent"]["diagnostics"][0]["message"]["text"].is_string());
    assert_eq!(
        result["structuredContent"]["diagnostics"][0]["ruleId"],
        "ENR-001"
    );
    assert!(shell.state().unwrap().enrollments.is_empty());
}
#[test]
fn scripted_action_only_lives_in_private_store_until_join_and_root_sees_verbs() {
    let dir = tempfile::tempdir().unwrap();
    let shell = Shell::open(
        &dir.path().join("store"),
        boundary::context(&repo()).unwrap(),
        Config::default(),
    )
    .unwrap();
    fixture(&shell);
    let tools = shell
        .submit(
            json!({"kind":"list-tools","identity":{"kind":"instance","id":id(&shell,"root")}}),
            &MissingExecutor,
        )
        .unwrap();
    for op in ["enroll-machine.v1", "admit-machine.v1", "refuse-machine.v1"] {
        assert!(
            tools["tools"]
                .as_array()
                .unwrap()
                .iter()
                .any(|t| t["name"] == format!("management.{op}"))
        );
    }
    let other = shell
        .submit(
            json!({"kind":"list-tools","identity":{"kind":"instance","id":id(&shell,"other")}}),
            &MissingExecutor,
        )
        .unwrap();
    assert!(!other.to_string().contains("enroll-machine.v1"));
    assert!(!other.to_string().contains("admit-machine.v1"));
    assert!(!other.to_string().contains("refuse-machine.v1"));
    let result = invoke(
        &shell,
        &Script,
        "enroll-machine.v1",
        json!({"expires":expiry()}),
    );
    assert!(!result.to_string().contains("SHELL_JOIN_MARKER"));
    let outputs = result["structuredContent"].as_object().unwrap();
    assert_eq!(outputs.len(), 2);
    assert_ne!(outputs["enrollment"], outputs["pendingAction"]);
    let state = shell.state().unwrap();
    assert!(
        serde_json::to_string(&state.enrollments)
            .unwrap()
            .contains("SHELL_JOIN_MARKER")
    );
    assert!(
        !serde_json::to_string(&state.records)
            .unwrap()
            .contains("SHELL_JOIN_MARKER")
    );
    assert!(
        !serde_json::to_string(&state.invocations)
            .unwrap()
            .contains("SHELL_JOIN_MARKER")
    );
    let export = shell
        .submit(json!({"kind":"export-records"}), &MissingExecutor)
        .unwrap();
    assert!(!export.to_string().contains("SHELL_JOIN_MARKER"));
    shell.submit(json!({"kind":"machine-report","provider":id(&shell,"machines"),"outcome":"succeeded","outputs":{"machines":[{"id":"fixture-id","hostname":"fixture.example","state":"pending","enrollment":outputs["enrollment"],"labels":{"kubernetes.io/hostname":"fixture.example","kubernetes.io/os":"linux","kubernetes.io/arch":"amd64"},"evidence":[{"kind":"fingerprint","value":"aB+/xyz","where":"fixture","proves":"key"}]}]}}),&MissingExecutor).unwrap();
    assert!(
        !serde_json::to_string(&shell.state().unwrap())
            .unwrap()
            .contains("SHELL_JOIN_MARKER")
    );
}
