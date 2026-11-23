use crate::support::{Harness, outcome, run_args};
use rmcp::model::CallToolResult;
use runtime_provider_kit::{names, schemas};
use serde_json::{Value, json};
use std::{fs, sync::atomic::Ordering};

const PROGRAM: &str = "platforms/linux/arm64/bin/server";
const ITEM_REASON: &str =
    "run.v2 item must be a command item with a relative program that states platforms";
const PROGRAM_REASON: &str = "run.v2 program must be a relative path inside the archive";

fn packaged_input() -> Value {
    let mut input = run_args();
    input["item"]["command"] = json!(["bin/server", "--flag"]);
    input["item"]["platforms"] = json!(["linux/arm64"]);
    input["program"] = json!(PROGRAM);
    input["archive"] = json!({
        "digest":format!("sha256:{}", "a".repeat(64)),
        "size":100,
        "urls":["file:///synthetic/package.tar.gz"]
    });
    input
}

fn failed(result: &CallToolResult, reason: &str) {
    outcome(result, "failed");
    let wire = serde_json::to_value(result).unwrap();
    assert!(
        wire["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains(reason),
        "missing diagnostic: {reason}"
    );
}

async fn tool_names(h: &Harness) -> Vec<String> {
    let mut names: Vec<_> = h
        .client
        .list_all_tools()
        .await
        .unwrap()
        .into_iter()
        .map(|tool| tool.name.into_owned())
        .collect();
    names.sort();
    names
}

#[tokio::test]
async fn run_v2_reaches_staging_with_the_program_and_the_archive() {
    let h = Harness::new().await;
    h.backend.packaged_programs.store(true, Ordering::SeqCst);
    assert_eq!(
        tool_names(&h).await,
        ["inspect-run.v1", "run.v1", "run.v2", "stop.v1"]
    );
    let input = packaged_input();
    let answer = h.call("run.v2", input.clone()).await;
    outcome(&answer, "succeeded");
    assert!(
        answer.structured_content.as_ref().unwrap()["url"]
            .as_str()
            .unwrap()
            .starts_with("https://")
    );
    assert_eq!(
        *h.backend.staged_programs.lock().unwrap(),
        [json!({"program":PROGRAM,"archive":input["archive"]})]
    );
    let ledger = fs::read_to_string(h.directory.path().join("ledger.json")).unwrap();
    assert!(!ledger.contains("\"program\"") && !ledger.contains("\"archive\""));
    h.close().await;
}

#[tokio::test]
async fn run_v2_accepts_a_schema_valid_decimal_archive_size() {
    let h = Harness::new().await;
    h.backend.packaged_programs.store(true, Ordering::SeqCst);
    let mut input = packaged_input();
    input["archive"]["size"] = serde_json::from_str("512.0").unwrap();
    assert!(schemas::valid("run.v2", "inputSchema", &input));
    let answer = serde_json::to_value(h.call("run.v2", input).await).unwrap();
    assert!(
        answer["_meta"][names::key("outcome")] == "succeeded",
        "schema-valid decimal archive size must reach staging"
    );
    assert_eq!(
        h.backend.staged_programs.lock().unwrap()[0]["archive"]["size"],
        512
    );
    h.close().await;
}

#[tokio::test]
async fn run_v2_is_refused_unless_declared_and_well_formed() {
    let h = Harness::new().await;
    assert_eq!(
        tool_names(&h).await,
        ["inspect-run.v1", "run.v1", "stop.v1"]
    );
    let calls = h.backend.calls.load(Ordering::SeqCst);
    failed(
        &h.call("run.v2", packaged_input()).await,
        "unknown runtime operation",
    );
    h.backend.packaged_programs.store(true, Ordering::SeqCst);
    for path in ["platforms/../bin/server", "/bin/server"] {
        let mut input = packaged_input();
        input["program"] = json!(path);
        failed(&h.call("run.v2", input).await, PROGRAM_REASON);
    }
    let mut input = packaged_input();
    input["item"].as_object_mut().unwrap().remove("platforms");
    failed(&h.call("run.v2", input).await, ITEM_REASON);
    let mut input = packaged_input();
    input["item"]["command"][0] = json!("/bin/server");
    failed(&h.call("run.v2", input).await, ITEM_REASON);
    assert_eq!(h.backend.calls.load(Ordering::SeqCst), calls);
    failed(
        &h.call("run.v1", packaged_input()).await,
        "runtime does not run this item on this machine",
    );
    assert_eq!(h.backend.stages.load(Ordering::SeqCst), 0);
    let mut input = packaged_input();
    input["item"].as_object_mut().unwrap().remove("platforms");
    input["item"]["command"][0] = json!("/bin/server");
    // v1 has always ignored extra members, including ones of the wrong type.
    input["program"] = json!(false);
    input["archive"] = json!("ignored");
    outcome(&h.call("run.v1", input).await, "succeeded");
    assert_eq!(
        *h.backend.staged_programs.lock().unwrap(),
        [json!({"program":null,"archive":null})]
    );
    h.close().await;
}
