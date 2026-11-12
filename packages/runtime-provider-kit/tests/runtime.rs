#[path = "runtime/reasons.rs"]
mod reasons;
mod support;
use rmcp::{ServiceExt, service::ServiceError};
use runtime_provider_kit::*;
use serde_json::json;
use std::{
    sync::{Arc, atomic::Ordering},
    time::Duration,
};
use support::*;

const SECOND_ID: &str = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";

fn second_run_args() -> serde_json::Value {
    let mut value = run_args();
    value["instance"] = json!(SECOND_ID);
    value
}

fn owed_stop_refusal(result: &rmcp::model::CallToolResult, instances: &[&str]) {
    outcome(result, "failed");
    let wire = serde_json::to_value(result).unwrap();
    let message = wire["content"][0]["text"].as_str().unwrap();
    for instance in instances {
        assert!(message.contains(instance));
    }
    assert!(message.contains(&machine().hostname));
    assert!(message.contains("start may be tried again once those stops are carried out"));
    assert!(result.structured_content.is_none());
}

fn owed_stop_unknown(result: &rmcp::model::CallToolResult, stops: &[(&str, &str)]) {
    outcome(result, "unknown");
    let wire = serde_json::to_value(result).unwrap();
    let message = wire["content"][0]["text"].as_str().unwrap();
    assert!(message.starts_with("run may still be running"));
    for (instance, hostname) in stops {
        assert!(message.contains(&format!("instance {instance} on machine {hostname}")));
    }
    assert!(message.contains("start may be tried again once those stops are carried out"));
    assert!(result.structured_content.is_none());
}

async fn machine_owed_stop_blocks_new_run(unsaved: bool) {
    let h = Harness::new().await;
    h.backend.seed(ID);
    h.store.save(&record(ID)).await.unwrap();
    h.store.fail_save.store(unsaved, Ordering::SeqCst);
    h.backend.unavailable.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    owed_stop_unknown(&h.call("run.v1", run_args()).await, &[(ID, "test-machine")]);
    let refused = h.call("run.v1", second_run_args()).await;
    owed_stop_refusal(&refused, &[ID]);
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    assert!(!h.backend.staged_valid.load(Ordering::SeqCst));
    assert!(h.gate.handles.lock().unwrap().is_empty());
    let records = h.store.load().await.unwrap();
    assert!(records.len() == 1 && records[0].instance == ID);
    assert!(records[0].owed_stop != unsaved);
    // Keep writes failing through the refusal, then let the next call drain and start.
    h.store.fail_save.store(false, Ordering::SeqCst);
    h.backend.unavailable.store(false, Ordering::SeqCst);
    outcome(&h.call("run.v1", second_run_args()).await, "succeeded");
    assert!(h.backend.starts.load(Ordering::SeqCst) == 1);
    assert!(h.backend.staged_valid.load(Ordering::SeqCst));
    assert!(!h.backend.runs.lock().unwrap().contains_key(ID));
    assert!(h.store.load().await.unwrap().iter().all(|r| !r.owed_stop));
    h.close().await;
}

#[tokio::test]
async fn durable_machine_owed_stop_blocks_new_run_until_drained() {
    machine_owed_stop_blocks_new_run(false).await;
}

#[tokio::test]
async fn memory_only_machine_owed_stop_blocks_new_run_until_drained() {
    machine_owed_stop_blocks_new_run(true).await;
}

// B has already loaded its ledger when A publishes a new stop intent. Exercise both
// an in-progress stop and a completed failure, with durable and memory-only intent.
async fn stop_published_during_state_lookup_blocks_start(unsaved: bool, in_progress: bool) {
    let h = Harness::new().await;
    h.backend.pause_next_state.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let run = tokio::spawn(async move {
        peer.call_tool(call("run.v1", second_run_args()))
            .await
            .unwrap()
    });
    h.backend.state_entered.notified().await;
    h.store.fail_save.store(unsaved, Ordering::SeqCst);
    h.backend.unavailable.store(true, Ordering::SeqCst);
    h.backend.pause_next_stop.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let stop = tokio::spawn(async move { peer.call_tool(call("stop.v1", args())).await.unwrap() });
    h.backend.stop_entered.notified().await;
    let records = h.store.load().await.unwrap();
    assert!(records.iter().any(|r| r.instance == ID && r.owed_stop) != unsaved);
    let stop = if in_progress {
        Some(stop)
    } else {
        h.backend.resume_stop.notify_one();
        outcome(&stop.await.unwrap(), "unknown");
        None
    };
    h.backend.resume_state.notify_one();
    let refused = tokio::time::timeout(Duration::from_secs(3), run)
        .await
        .unwrap()
        .unwrap();
    owed_stop_refusal(&refused, &[ID]);
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    assert!(!h.backend.staged_valid.load(Ordering::SeqCst));
    assert!(h.gate.opens.load(Ordering::SeqCst) == 0);
    let records = h.store.load().await.unwrap();
    assert!(records.iter().all(|r| r.instance != SECOND_ID));
    assert!(records.iter().any(|r| r.instance == ID && r.owed_stop) != unsaved);
    if let Some(stop) = stop {
        h.backend.resume_stop.notify_one();
        outcome(&stop.await.unwrap(), "unknown");
    }
    h.store.fail_save.store(false, Ordering::SeqCst);
    h.backend.unavailable.store(false, Ordering::SeqCst);
    outcome(&h.call("run.v1", second_run_args()).await, "succeeded");
    assert!(h.backend.starts.load(Ordering::SeqCst) == 1);
    h.close().await;
}

#[tokio::test]
async fn new_durable_stop_during_state_lookup_blocks_start() {
    for in_progress in [true, false] {
        stop_published_during_state_lookup_blocks_start(false, in_progress).await;
    }
}

#[tokio::test]
async fn new_memory_only_stop_during_state_lookup_blocks_start() {
    for in_progress in [true, false] {
        stop_published_during_state_lookup_blocks_start(true, in_progress).await;
    }
}

async fn stop_published_during_staging_blocks_start(unsaved: bool) {
    let h = Harness::new().await;
    h.backend.seed(ID);
    h.store.save(&record(ID)).await.unwrap();
    h.backend.pause_next_stage.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let run = tokio::spawn(async move {
        peer.call_tool(call("run.v1", second_run_args()))
            .await
            .unwrap()
    });
    tokio::time::timeout(Duration::from_secs(3), h.backend.stage_entered.notified())
        .await
        .unwrap();
    assert!(h.backend.staged_drops.load(Ordering::SeqCst) == 0);
    h.store.fail_save.store(unsaved, Ordering::SeqCst);
    h.backend.unavailable.store(true, Ordering::SeqCst);
    // Completing the stop before releasing staging proves that staging holds no admission.
    let stopped = tokio::time::timeout(Duration::from_secs(3), h.call("stop.v1", args()))
        .await
        .expect("staging delayed another instance's stop");
    outcome(&stopped, "unknown");
    assert!(!run.is_finished());
    assert!(h.backend.stops.load(Ordering::SeqCst) == 1);
    let records = h.store.load().await.unwrap();
    assert!(records.iter().any(|r| r.instance == ID && r.owed_stop) != unsaved);
    // Permit B's stopped record to be saved; A's memory-only obligation stays in memory.
    h.store.fail_save.store(false, Ordering::SeqCst);
    h.backend.resume_stage.notify_one();
    let refused = tokio::time::timeout(Duration::from_secs(3), run)
        .await
        .unwrap()
        .unwrap();
    owed_stop_refusal(&refused, &[ID]);
    assert!(h.gate.opens.load(Ordering::SeqCst) == 0);
    assert!(h.gate.handles.lock().unwrap().is_empty());
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    assert!(h.backend.launches.load(Ordering::SeqCst) == 0);
    assert!(!h.backend.runs.lock().unwrap().contains_key(SECOND_ID));
    assert!(h.backend.staged_drops.load(Ordering::SeqCst) == 1);
    assert!(h.backend.stops.load(Ordering::SeqCst) == 1);
    let records = h.store.load().await.unwrap();
    assert!(
        records
            .iter()
            .any(|r| { r.instance == SECOND_ID && r.state == RunState::Stopped && !r.owed_stop })
    );
    assert!(records.iter().any(|r| r.instance == ID && r.owed_stop) != unsaved);
    h.backend.unavailable.store(false, Ordering::SeqCst);
    h.close().await;
}

#[tokio::test]
async fn durable_stop_during_staging_refuses_start_and_discards_staged_material() {
    stop_published_during_staging_blocks_start(false).await;
}

#[tokio::test]
async fn memory_only_stop_during_staging_refuses_start_and_discards_staged_material() {
    stop_published_during_staging_blocks_start(true).await;
}

#[tokio::test]
async fn starts_on_same_machine_share_admission() {
    let h = Harness::new().await;
    h.backend.pause_next_start.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let run = tokio::spawn(async move {
        peer.call_tool(call("run.v1", second_run_args()))
            .await
            .unwrap()
    });
    tokio::time::timeout(Duration::from_secs(3), h.backend.entered.notified())
        .await
        .unwrap();
    outcome(
        &tokio::time::timeout(Duration::from_secs(3), h.call("run.v1", run_args()))
            .await
            .expect("start delayed another instance's start"),
        "succeeded",
    );
    assert!(!run.is_finished());
    assert!(h.backend.starts.load(Ordering::SeqCst) == 2);
    h.backend.resume_start.notify_one();
    outcome(
        &tokio::time::timeout(Duration::from_secs(3), run)
            .await
            .unwrap()
            .unwrap(),
        "succeeded",
    );
    h.close().await;
}

#[tokio::test]
async fn same_machine_stop_waits_for_in_progress_start_then_proceeds() {
    let h = Harness::new().await;
    h.backend.seed(ID);
    h.store.save(&record(ID)).await.unwrap();
    h.backend.pause_next_start.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let run = tokio::spawn(async move {
        peer.call_tool(call("run.v1", second_run_args()))
            .await
            .unwrap()
    });
    tokio::time::timeout(Duration::from_secs(3), h.backend.entered.notified())
        .await
        .unwrap();
    let loads = h.store.loads.load(Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let stop = tokio::spawn(async move { peer.call_tool(call("stop.v1", args())).await.unwrap() });
    // The stop has read both its drain and instance records. With this synchronous fake
    // store it then runs to the admission await before yielding to this test.
    eventually(|| h.store.loads.load(Ordering::SeqCst) >= loads + 2).await;
    assert!(!stop.is_finished());
    assert!(h.backend.stops.load(Ordering::SeqCst) == 0);
    assert!(h.backend.launches.load(Ordering::SeqCst) == 0);
    assert!(h.store.load().await.unwrap().iter().all(|r| !r.owed_stop));
    h.backend.resume_start.notify_one();
    outcome(
        &tokio::time::timeout(Duration::from_secs(3), run)
            .await
            .unwrap()
            .unwrap(),
        "succeeded",
    );
    outcome(
        &tokio::time::timeout(Duration::from_secs(3), stop)
            .await
            .unwrap()
            .unwrap(),
        "succeeded",
    );
    assert!(h.backend.starts.load(Ordering::SeqCst) == 1);
    assert!(h.backend.launches.load(Ordering::SeqCst) == 1);
    assert!(h.backend.stops.load(Ordering::SeqCst) == 1);
    assert!(!h.backend.runs.lock().unwrap().contains_key(ID));
    assert!(h.backend.runs.lock().unwrap().contains_key(SECOND_ID));
    assert!(
        h.store
            .load()
            .await
            .unwrap()
            .iter()
            .any(|r| { r.instance == ID && r.state == RunState::Stopped && !r.owed_stop })
    );
    h.close().await;
}

#[tokio::test]
async fn machine_owed_stop_preserves_already_running_repeat() {
    let h = Harness::new().await;
    let first = h.call("run.v1", second_run_args()).await;
    outcome(&first, "succeeded");
    h.backend.unavailable.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    let repeated = h.call("run.v1", second_run_args()).await;
    outcome(&repeated, "succeeded");
    assert!(first.structured_content == repeated.structured_content);
    assert!(h.backend.starts.load(Ordering::SeqCst) == 1);
    assert!(
        h.store
            .load()
            .await
            .unwrap()
            .iter()
            .any(|r| r.instance == ID && r.owed_stop)
    );
    h.backend.unavailable.store(false, Ordering::SeqCst);
    h.close().await;
}

#[tokio::test]
async fn owed_stop_on_another_machine_does_not_block_new_run() {
    let h = Harness::new().await;
    h.backend.unavailable.store(true, Ordering::SeqCst);
    let mut elsewhere = args();
    elsewhere["machine"]["hostname"] = json!("other-machine");
    outcome(&h.call("stop.v1", elsewhere).await, "unknown");
    outcome(&h.call("run.v1", second_run_args()).await, "succeeded");
    assert!(h.backend.starts.load(Ordering::SeqCst) == 1);
    assert!(h.store.load().await.unwrap().iter().any(|r| {
        r.instance == ID && r.context.machine.hostname == "other-machine" && r.owed_stop
    }));
    h.backend.unavailable.store(false, Ordering::SeqCst);
    h.close().await;
}

#[tokio::test]
async fn machine_owed_stop_refusal_names_every_owed_instance() {
    let h = Harness::new().await;
    let third_id = "cccccccc-cccc-4ccc-8ccc-cccccccccccc";
    h.backend.unavailable.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    let mut third = args();
    third["instance"] = json!(third_id);
    outcome(&h.call("stop.v1", third).await, "unknown");
    owed_stop_refusal(&h.call("run.v1", second_run_args()).await, &[ID, third_id]);
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    assert!(!h.backend.staged_valid.load(Ordering::SeqCst));
    assert!(h.gate.handles.lock().unwrap().is_empty());
    h.backend.unavailable.store(false, Ordering::SeqCst);
    h.close().await;
}

#[tokio::test]
async fn discovery_extension_and_exact_tools() {
    let h = Harness::new().await;
    let discover = h.client.discover(serde_json::from_value(json!({"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}})).unwrap()).await.unwrap();
    let wire = serde_json::to_value(discover).unwrap();
    assert!(wire["capabilities"]["extensions"][names::extension()]["revision"] == "v0");
    let tools = h.client.list_all_tools().await.unwrap();
    let mut names: Vec<_> = tools.iter().map(|t| t.name.as_ref()).collect();
    names.sort();
    assert!(names == ["inspect-run.v1", "run.v1", "stop.v1"]);
    for tool in tools {
        let wire = serde_json::to_value(&tool).unwrap();
        assert!(wire["inputSchema"] == schemas::schema(&tool.name, "inputSchema").unwrap());
        if let Some(output) = schemas::schema(&tool.name, "outputSchema") {
            assert!(wire["outputSchema"] == output);
        }
    }
    h.close().await;
}
#[tokio::test]
async fn missing_or_other_extension_revision_never_calls_backend() {
    let h = Harness::new().await;
    let before = h.backend.calls.load(Ordering::SeqCst);
    for revision in [None, Some("v1")] {
        let mut request = serde_json::to_value(call("run.v1", run_args())).unwrap();
        request["_meta"]["io.modelcontextprotocol/clientCapabilities"] = match revision {
            None => json!({}),
            Some(revision) => json!({"extensions":{names::extension():{"revision":revision}}}),
        };
        let error = h
            .client
            .call_tool(serde_json::from_value(request).unwrap())
            .await
            .expect_err("expected capability error");
        if let ServiceError::McpError(error) = error {
            assert!(error.code.0 == -32021);
            assert!(
                error.data.unwrap()["requiredCapabilities"]["extensions"][names::extension()]["revision"]
                    == "v0"
            );
        } else {
            panic!("expected MCP error");
        }
    }
    assert!(h.backend.calls.load(Ordering::SeqCst) == before);
    h.close().await;
}
#[tokio::test]
async fn invalid_schemas_fail_without_backend_calls() {
    let h = Harness::new().await;
    let before = h.backend.calls.load(Ordering::SeqCst);
    let mut invalid = Vec::new();
    for pointer in [
        "/instance",
        "/identity/files",
        "/machine/hostname",
        "/item/command",
    ] {
        let mut value = run_args();
        *value.pointer_mut(pointer).unwrap() = json!(17);
        invalid.push(value);
    }
    let mut relative = run_args();
    relative["item"]["remote"] =
        json!({"type":"streamable-http","url":"https://example.invalid/mcp"});
    invalid.push(relative);
    let mut missing_file = run_args();
    missing_file["identity"]["files"]
        .as_object_mut()
        .unwrap()
        .remove(schemas::identity_file_names()[0]);
    invalid.push(missing_file);
    for value in invalid {
        outcome(&h.call("run.v1", value).await, "failed");
    }
    for name in ["stop.v1", "inspect-run.v1"] {
        outcome(&h.call(name, json!({"instance":ID})).await, "failed");
    }
    assert!(h.backend.calls.load(Ordering::SeqCst) == before);
    h.close().await;
}
#[tokio::test]
async fn declaration_refuses_unsupported_registry_transport_command_and_machine() {
    let h = Harness::new().await;
    let mut values = Vec::new();
    let mut item = run_args();
    item["machine"]["hostname"] = json!("elsewhere");
    values.push(item);
    for program in ["./server", "relative/server"] {
        let mut value = run_args();
        value["item"]["command"] = json!([program]);
        values.push(value);
    }
    for (registry, transport) in [("npm", "stdio"), ("oci", "streamable-http")] {
        let mut value = run_args();
        value["item"].as_object_mut().unwrap().remove("command");
        value["item"]["package"] = json!({"registryType":registry,"identifier":format!("example@sha256:{}","0".repeat(64)),"fileSha256":"0".repeat(64),"transport":{"type":transport,"url":"https://example.invalid/mcp"}});
        values.push(value);
    }
    for value in values {
        outcome(&h.call("run.v1", value).await, "failed");
    }
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    assert!(!h.backend.staged_valid.load(Ordering::SeqCst));
    h.close().await;
}
#[tokio::test]
async fn run_repeats_inspect_current_url_and_stop_all_have_outcomes() {
    let h = Harness::new().await;
    let first = h.call("run.v1", run_args()).await;
    outcome(&first, "succeeded");
    let repeated = h.call("run.v1", run_args()).await;
    outcome(&repeated, "succeeded");
    assert!(first.structured_content == repeated.structured_content);
    assert!(h.backend.starts.load(Ordering::SeqCst) == 1);
    assert!(h.backend.staged_valid.load(Ordering::SeqCst));
    let inspected = h.call("inspect-run.v1", args()).await;
    outcome(&inspected, "succeeded");
    assert!(inspected.structured_content.unwrap()["state"] == "running");
    *h.gate.handles.lock().unwrap()[0].url.lock().unwrap() = "https://127.0.0.1:49999/mcp".into();
    let inspected = h.call("inspect-run.v1", args()).await;
    assert!(inspected.structured_content.unwrap()["url"] == "https://127.0.0.1:49999/mcp");
    outcome(&h.call("stop.v1", args()).await, "succeeded");
    outcome(&h.call("stop.v1", args()).await, "succeeded");
    let state = h.call("inspect-run.v1", args()).await;
    outcome(&state, "succeeded");
    assert!(state.structured_content.unwrap()["state"] == "stopped");
    assert!(
        h.store
            .load()
            .await
            .unwrap()
            .iter()
            .all(|r| r.state == RunState::Stopped && !r.owed_stop)
    );
    h.close().await;
}
#[tokio::test]
async fn unknown_state_inspection_succeeds_and_run_starts_nothing() {
    let h = Harness::new().await;
    h.backend.unknown.store(true, Ordering::SeqCst);
    let result = h.call("run.v1", run_args()).await;
    outcome(&result, "unknown");
    let wire = serde_json::to_value(result).unwrap();
    assert!(wire["content"][0]["text"] == "run may still be running");
    let answer = h.call("inspect-run.v1", args()).await;
    outcome(&answer, "succeeded");
    assert!(answer.structured_content.unwrap()["state"] == "unknown");
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    h.close().await;
}
#[tokio::test]
async fn channel_end_stops_tree_and_updates_ledger() {
    let h = Harness::new().await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    h.backend.end_channel();
    eventually(|| !h.backend.runs.lock().unwrap().contains_key(ID)).await;
    let records = h.store.load().await.unwrap();
    assert!(records.len() == 1 && records[0].state == RunState::Stopped && !records[0].owed_stop);
    assert!(h.backend.stops.load(Ordering::SeqCst) > 0);
    h.close().await;
}
#[tokio::test]
async fn gate_end_stops_tree() {
    let h = Harness::new().await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    let handle = h.gate.handles.lock().unwrap()[0].clone();
    handle.close().await;
    eventually(|| !h.backend.runs.lock().unwrap().contains_key(ID)).await;
    h.close().await;
}
#[tokio::test]
async fn configured_lifetime_is_backstop() {
    let h = Harness::with_options(Options {
        max_lifetime: Duration::from_millis(90),
        ..options()
    })
    .await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    eventually(|| !h.backend.runs.lock().unwrap().contains_key(ID)).await;
    assert!(h.backend.launches.load(Ordering::SeqCst) == 1);
    h.close().await;
}
#[tokio::test]
async fn lifetime_stop_uses_the_newest_call_context() {
    // A run started under r1, then an inspection under r2: the stop that the configured lifetime
    // triggers carries r2, never the context captured when the run started (order 124, decision 1).
    let h = Harness::with_options(Options {
        max_lifetime: Duration::from_millis(1500),
        ..options()
    })
    .await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    let mut wire = serde_json::to_value(call("inspect-run.v1", args())).unwrap();
    wire["_meta"][names::key("config")] = json!({"changed":true});
    wire["_meta"][names::key("revision")] = json!("r2");
    outcome(
        &h.client
            .call_tool(serde_json::from_value(wire).unwrap())
            .await
            .unwrap(),
        "succeeded",
    );
    h.backend.contexts.lock().unwrap().clear();
    eventually(|| !h.backend.runs.lock().unwrap().contains_key(ID)).await;
    {
        let contexts = h.backend.contexts.lock().unwrap();
        assert!(!contexts.is_empty());
        assert!(contexts.iter().all(|c| c.revision == "r2"));
    }
    h.close().await;
}
#[tokio::test]
async fn owed_stop_survives_server_restart_and_drains_after_instance_removal_without_server() {
    let h = Harness::new().await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    h.backend.unavailable.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    assert!(h.store.load().await.unwrap()[0].owed_stop);
    let backend = h.backend.clone();
    let gate = h.gate.clone();
    let directory = h.finish().await;
    let store = Arc::new(FileStore::new(directory.path().join("ledger.json")));
    let server = Server::new(backend.clone(), store.clone(), gate, options())
        .await
        .unwrap();
    assert!(store.load().await.unwrap()[0].owed_stop);
    assert!(server.shutdown().await.is_err());
    drop(server);
    // No instance registry and no running provider server: the durable obligation is enough.
    backend.unavailable.store(false, Ordering::SeqCst);
    assert!(
        drain_owed_stops(backend.as_ref(), store.as_ref())
            .await
            .unwrap()
            == 1
    );
    assert!(!store.load().await.unwrap()[0].owed_stop);
    assert!(backend.runs.lock().unwrap().is_empty());
}
#[tokio::test]
async fn successful_stop_cancels_inflight_start_and_next_explicit_run_may_start() {
    let h = Harness::new().await;
    h.backend.block.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let run =
        tokio::spawn(async move { peer.call_tool(call("run.v1", run_args())).await.unwrap() });
    h.backend.entered.notified().await;
    outcome(&h.call("stop.v1", args()).await, "succeeded");
    outcome(&run.await.unwrap(), "failed");
    assert!(h.backend.launches.load(Ordering::SeqCst) == 0);
    assert!(h.backend.runs.lock().unwrap().is_empty());
    h.backend.block.store(false, Ordering::SeqCst);
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    assert!(h.backend.launches.load(Ordering::SeqCst) == 1);
    h.close().await;
}

async fn blocked_start_does_not_delay_unrelated_stop(interval: bool) {
    const OTHER: &str = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
    let h = Harness::with_options(Options {
        drain_interval: if interval {
            Duration::from_millis(60)
        } else {
            options().drain_interval
        },
        ..options()
    })
    .await;
    h.backend.block.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let run =
        tokio::spawn(async move { peer.call_tool(call("run.v1", run_args())).await.unwrap() });
    tokio::time::timeout(Duration::from_secs(3), h.backend.entered.notified())
        .await
        .unwrap();
    h.backend.seed(OTHER);
    let mut other = record(OTHER);
    other.context.machine.hostname = "other-machine".into();
    h.backend.runs.lock().unwrap().get_mut(OTHER).unwrap().0 = other.clone();
    other.owed_stop = interval;
    h.store.save(&other).await.unwrap();

    if interval {
        eventually(|| !h.backend.runs.lock().unwrap().contains_key(OTHER)).await;
    } else {
        let mut input = args();
        input["instance"] = json!(OTHER);
        input["machine"]["hostname"] = json!("other-machine");
        let stopped = tokio::time::timeout(Duration::from_secs(3), h.call("stop.v1", input))
            .await
            .expect("unrelated stop blocked behind start");
        outcome(&stopped, "succeeded");
    }
    assert!(!h.backend.runs.lock().unwrap().contains_key(OTHER));
    assert!(!run.is_finished());
    assert!(h.backend.launches.load(Ordering::SeqCst) == 0);
    let saved = h.store.load().await.unwrap();
    assert!(
        saved
            .iter()
            .any(|r| r.instance == OTHER && r.state == RunState::Stopped && !r.owed_stop)
    );

    outcome(&h.call("stop.v1", args()).await, "succeeded");
    outcome(&run.await.unwrap(), "failed");
    h.close().await;
}

#[tokio::test]
async fn blocked_start_does_not_delay_stop_call_for_another_instance() {
    blocked_start_does_not_delay_unrelated_stop(false).await;
}

#[tokio::test]
async fn blocked_start_does_not_delay_interval_stop_for_another_instance() {
    blocked_start_does_not_delay_unrelated_stop(true).await;
}

#[tokio::test]
async fn stop_overtakes_run_paused_in_initial_ledger_read() {
    let h = Harness::new().await;
    h.store.pause_next_load.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let run =
        tokio::spawn(async move { peer.call_tool(call("run.v1", run_args())).await.unwrap() });
    tokio::time::timeout(Duration::from_secs(3), h.store.load_entered.notified())
        .await
        .unwrap();
    outcome(&h.call("stop.v1", args()).await, "succeeded");
    h.store.resume_load.notify_one();
    outcome(
        &tokio::time::timeout(Duration::from_secs(3), run)
            .await
            .unwrap()
            .unwrap(),
        "failed",
    );
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    assert!(h.backend.runs.lock().unwrap().is_empty());
    h.close().await;
}

#[tokio::test]
async fn unreadable_ledger_stop_still_cancels_inflight_start() {
    let h = Harness::new().await;
    h.backend.block.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let run =
        tokio::spawn(async move { peer.call_tool(call("run.v1", run_args())).await.unwrap() });
    tokio::time::timeout(Duration::from_secs(3), h.backend.entered.notified())
        .await
        .unwrap();
    h.store.fail_load.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    outcome(&run.await.unwrap(), "failed");
    assert!(h.backend.launches.load(Ordering::SeqCst) == 0);
    assert!(h.backend.stops.load(Ordering::SeqCst) > 0);
    h.close().await;
}
#[tokio::test]
async fn startup_sweeps_both_ledger_runs_and_unrecorded_backend_runs() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(FileStore::new(directory.path().join("ledger.json")));
    let backend = Arc::new(FakeBackend::default());
    let orphan = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
    backend.seed(ID);
    backend.seed(orphan);
    store.save(&record(ID)).await.unwrap();
    let server = Server::new(
        backend.clone(),
        store.clone(),
        Arc::new(FakeGate::default()),
        options(),
    )
    .await
    .unwrap();
    assert!(backend.runs.lock().unwrap().is_empty());
    let records = store.load().await.unwrap();
    assert!(
        records.len() == 2
            && records
                .iter()
                .all(|r| r.state == RunState::Stopped && !r.owed_stop)
    );
    assert!(backend.stops.load(Ordering::SeqCst) == 2);
    server.shutdown().await.unwrap();
}
#[tokio::test]
async fn interval_and_each_call_drain_owed_stops() {
    let h = Harness::with_options(Options {
        drain_interval: Duration::from_millis(60),
        ..options()
    })
    .await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    h.backend.unavailable.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    h.backend.unavailable.store(false, Ordering::SeqCst);
    eventually(|| h.backend.runs.lock().unwrap().is_empty()).await;
    eventually(|| {
        serde_json::from_slice::<Vec<RunRecord>>(&h.store.bytes())
            .unwrap()
            .iter()
            .all(|r| !r.owed_stop)
    })
    .await;
    h.backend.seed(ID);
    let mut owed = record(ID);
    owed.owed_stop = true;
    h.store.save(&owed).await.unwrap();
    let answer = h.call("inspect-run.v1", args()).await;
    outcome(&answer, "succeeded");
    assert!(answer.structured_content.unwrap()["state"] == "stopped");
    h.close().await;
}
#[tokio::test]
async fn current_config_revision_and_machine_reach_every_runtime_effect() {
    let h = Harness::new().await;
    for (name, args) in [
        ("run.v1", run_args()),
        ("inspect-run.v1", args()),
        ("stop.v1", args()),
    ] {
        let mut wire = serde_json::to_value(call(name, args)).unwrap();
        wire["_meta"][names::key("config")] = json!({"new":true});
        wire["_meta"][names::key("revision")] = json!("r2");
        outcome(
            &h.client
                .call_tool(serde_json::from_value(wire).unwrap())
                .await
                .unwrap(),
            "succeeded",
        );
    }
    assert!(
        h.backend
            .contexts
            .lock()
            .unwrap()
            .iter()
            .all(|c| c.config == json!({"new":true})
                && c.revision == "r2"
                && c.machine == machine())
    );
    h.close().await;
}
#[tokio::test]
async fn staging_start_and_gate_failures_never_leave_unaccounted_trees() {
    for failure in 0..3 {
        let h = Harness::new().await;
        match failure {
            0 => h.backend.fail_stage.store(true, Ordering::SeqCst),
            1 => h.backend.fail_start.store(true, Ordering::SeqCst),
            _ => h.gate.fail.store(true, Ordering::SeqCst),
        }
        outcome(&h.call("run.v1", run_args()).await, "failed");
        assert!(h.backend.runs.lock().unwrap().is_empty());
        for marker in markers() {
            assert!(
                !String::from_utf8_lossy(&h.store.bytes()).contains(&marker),
                "sensitive ledger"
            );
        }
        h.close().await;
    }
}
#[tokio::test]
async fn provider_transport_loss_stops_all_held_runs() {
    let h = Harness::new().await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    let backend = h.backend.clone();
    h.close().await;
    assert!(backend.runs.lock().unwrap().is_empty());
}

#[tokio::test]
async fn transport_loss_with_unreadable_ledger_closes_gate_and_stops_tree() {
    let h = Harness::new().await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    let backend = h.backend.clone();
    let store = h.store.clone();
    let handle = h.gate.handles.lock().unwrap()[0].clone();
    store.fail_load.store(true, Ordering::SeqCst);
    let directory = h.finish().await;
    tokio::time::timeout(Duration::from_secs(3), handle.ended())
        .await
        .unwrap();
    assert!(backend.runs.lock().unwrap().is_empty());
    let records: Vec<RunRecord> = serde_json::from_slice(&store.bytes()).unwrap();
    assert!(records.len() == 1);
    assert!(records[0].state == RunState::Stopped && !records[0].owed_stop);
    drop(directory);
}

#[tokio::test]
async fn unreadable_ledger_stop_closes_gate_and_persists_owed_historical_and_call_machines() {
    let h = Harness::new().await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    let handle = h.gate.handles.lock().unwrap()[0].clone();
    h.store.fail_load.store(true, Ordering::SeqCst);
    h.backend.unavailable.store(true, Ordering::SeqCst);
    let mut input = args();
    input["machine"]["hostname"] = json!("new-machine");
    outcome(&h.call("stop.v1", input).await, "unknown");
    tokio::time::timeout(Duration::from_secs(3), handle.ended())
        .await
        .unwrap();
    let records: Vec<RunRecord> = serde_json::from_slice(&h.store.bytes()).unwrap();
    assert!(records.len() == 2 && records.iter().all(|r| r.owed_stop));
    assert!(
        records
            .iter()
            .any(|r| r.context.machine.hostname == machine().hostname)
    );
    assert!(
        records
            .iter()
            .any(|r| r.context.machine.hostname == "new-machine")
    );
    let backend = h.backend.clone();
    let store = h.store.clone();
    let directory = h.finish().await;
    store.fail_load.store(false, Ordering::SeqCst);
    backend.unavailable.store(false, Ordering::SeqCst);
    assert!(
        drain_owed_stops(backend.as_ref(), store.as_ref())
            .await
            .unwrap()
            == 2
    );
    assert!(store.load().await.unwrap().iter().all(|r| !r.owed_stop));
    assert!(backend.runs.lock().unwrap().is_empty());
    drop(directory);
}

#[tokio::test]
async fn unreadable_ledger_stop_still_stops_tree_when_writes_succeed() {
    let h = Harness::new().await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    h.store.fail_load.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    assert!(h.backend.runs.lock().unwrap().is_empty());
    let records: Vec<RunRecord> = serde_json::from_slice(&h.store.bytes()).unwrap();
    assert!(records.len() == 1 && records[0].state == RunState::Stopped && !records[0].owed_stop);
    h.close().await;
}

#[tokio::test]
async fn inspect_and_stop_refresh_current_machine_labels_but_preserve_historical_hosts() {
    let h = Harness::new().await;
    let mut historical = record(ID);
    historical.context.machine.hostname = "historical-machine".into();
    historical
        .context
        .machine
        .labels
        .insert("status".into(), "historical".into());
    h.store.save(&historical).await.unwrap();
    let mut run = run_args();
    run["machine"]["labels"] = json!({"status":"at-run"});
    outcome(&h.call("run.v1", run).await, "succeeded");
    for (name, status) in [("inspect-run.v1", "at-inspect"), ("stop.v1", "at-stop")] {
        h.backend.contexts.lock().unwrap().clear();
        let mut input = args();
        input["machine"]["labels"] = json!({"status":status});
        outcome(&h.call(name, input).await, "succeeded");
        let contexts = h.backend.contexts.lock().unwrap();
        assert!(
            contexts
                .iter()
                .any(|c| c.machine.hostname == machine().hostname)
        );
        assert!(
            contexts
                .iter()
                .any(|c| c.machine.hostname == "historical-machine")
        );
        for context in contexts.iter() {
            let expected = if context.machine.hostname == machine().hostname {
                status
            } else {
                "historical"
            };
            assert!(context.machine.labels.get("status").map(String::as_str) == Some(expected));
        }
    }
    h.close().await;
}
#[tokio::test]
async fn concurrent_runs_start_once() {
    let h = Harness::new().await;
    let mut tasks = Vec::new();
    for _ in 0..8 {
        let peer = h.client.peer().clone();
        tasks.push(tokio::spawn(async move {
            peer.call_tool(call("run.v1", run_args())).await.unwrap()
        }));
    }
    let mut outputs = Vec::new();
    for task in tasks {
        let answer = task.await.unwrap();
        outcome(&answer, "succeeded");
        outputs.push(answer.structured_content);
    }
    assert!(outputs.iter().all(|o| o == &outputs[0]));
    assert!(h.backend.launches.load(Ordering::SeqCst) == 1);
    h.close().await;
}
#[tokio::test]
async fn missing_metadata_unknown_tool_and_environment_collisions_fail_safely() {
    let h = Harness::new().await;
    let mut missing = serde_json::to_value(call("run.v1", run_args())).unwrap();
    missing["_meta"]
        .as_object_mut()
        .unwrap()
        .remove(&names::key("config"));
    outcome(
        &h.client
            .call_tool(serde_json::from_value(missing).unwrap())
            .await
            .unwrap(),
        "failed",
    );
    outcome(&h.call("other.v1", json!({})).await, "failed");
    for key in [
        "PRIVATE",
        names::ENGINE_URL_VARIABLE,
        names::IDENTITY_DIR_VARIABLE,
    ] {
        let mut value = run_args();
        value["environment"][key] = json!("overlap");
        outcome(&h.call("run.v1", value).await, "failed");
    }
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    h.close().await;
}

#[derive(Clone, Default)]
struct LogCapture(Arc<std::sync::Mutex<Vec<u8>>>);
impl std::io::Write for LogCapture {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[tokio::test]
async fn secrets_and_identity_never_enter_records_logs_or_answers_even_at_trace() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(FileStore::new(directory.path().join("ledger.json")));
    let backend = Arc::new(FakeBackend::default());
    let gate = Arc::new(FakeGate::default());
    let server = Server::new(backend.clone(), store.clone(), gate, options())
        .await
        .unwrap();
    let logs = LogCapture::default();
    let writer = logs.clone();
    let (client, transport) = tokio::io::duplex(65536);
    // Keep all server tasks on a dedicated thread with a trace-level subscriber. The client
    // is intentionally outside this subscriber: its outgoing request owns the test inputs.
    let thread = std::thread::spawn(move || {
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_ansi(false)
            .without_time()
            .with_writer(move || writer.clone())
            .finish();
        let _subscriber = tracing::subscriber::set_default(subscriber);
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(server.serve_transport(transport))
            .unwrap();
    });
    let client = ().serve(client).await.unwrap();
    for (name, args) in [
        ("run.v1", run_args()),
        ("inspect-run.v1", args()),
        ("stop.v1", args()),
    ] {
        outcome(
            &client.call_tool(call(name, args)).await.unwrap(),
            "succeeded",
        );
    }
    // rmcp may log answers. Exercise rejected runtime reasons at trace level too.
    let input = run_args();
    let mut sensitive_values: Vec<String> = [
        &input["identity"]["environment"],
        &input["identity"]["files"],
        &input["secrets"]["environment"],
        &input["secrets"]["packageArguments"],
    ]
    .into_iter()
    .flat_map(|bindings| bindings.as_object().unwrap().values())
    .map(|value| value.as_str().unwrap().to_owned())
    .collect();
    let mut multiline = input.clone();
    let filename = schemas::identity_file_names()[0];
    // No other binding contains this marker or the whole normalized file content.
    let content = format!("{}\nidentity-file-continuation", markers()[0]);
    multiline["identity"]["files"][filename] = json!(content);
    sensitive_values.push(content.replace('\n', " "));
    for (i, value) in sensitive_values.iter().enumerate() {
        for staging in [true, false] {
            backend.fail_stage.store(staging, Ordering::SeqCst);
            backend.fail_start.store(!staging, Ordering::SeqCst);
            let reason = format!("runtime refused {value}");
            *backend.stage_reason.lock().unwrap() = Some(reason.clone());
            *backend.start_reason.lock().unwrap() = Some(reason);
            let input = if i == sensitive_values.len() - 1 {
                multiline.clone()
            } else {
                input.clone()
            };
            outcome(
                &client.call_tool(call("run.v1", input)).await.unwrap(),
                "failed",
            );
        }
    }
    client.cancel().await.unwrap();
    thread.join().unwrap();
    let logs = logs.0.lock().unwrap();
    assert!(!logs.is_empty(), "capture must exercise logging");
    for marker in markers() {
        assert!(
            !String::from_utf8_lossy(&logs).contains(&marker),
            "sensitive log line"
        );
        assert!(
            !String::from_utf8_lossy(&store.bytes()).contains(&marker),
            "sensitive durable record"
        );
    }
}
#[tokio::test]
async fn held_unknown_run_keeps_gate_url_and_starts_nothing() {
    let h = Harness::new().await;
    let started = h.call("run.v1", run_args()).await;
    h.backend.unknown.store(true, Ordering::SeqCst);
    outcome(&h.call("run.v1", run_args()).await, "unknown");
    let inspected = h.call("inspect-run.v1", args()).await;
    outcome(&inspected, "succeeded");
    assert!(inspected.structured_content.as_ref().unwrap()["state"] == "unknown");
    assert!(
        inspected.structured_content.as_ref().unwrap()["url"]
            == started.structured_content.as_ref().unwrap()["url"]
    );
    assert!(h.backend.starts.load(Ordering::SeqCst) == 1);
    // Direct shutdown remains usable without a client call.
    h.server.shutdown().await.unwrap();
    h.close().await;
}

#[tokio::test]
async fn store_failures_do_not_launch_or_prevent_tree_stop_and_inspection_stays_succeeded() {
    let h = Harness::new().await;
    h.store.fail_save.store(true, Ordering::SeqCst);
    outcome(&h.call("run.v1", run_args()).await, "failed");
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    h.store.fail_save.store(false, Ordering::SeqCst);
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    h.store.fail_load.store(true, Ordering::SeqCst);
    let inspected = h.call("inspect-run.v1", args()).await;
    outcome(&inspected, "succeeded");
    assert!(inspected.structured_content.unwrap()["state"] == "unknown");
    h.store.fail_load.store(false, Ordering::SeqCst);
    h.store.fail_save.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    assert!(h.backend.runs.lock().unwrap().is_empty());
    h.store.fail_save.store(false, Ordering::SeqCst);
    h.close().await;
}
#[tokio::test]
async fn old_lifetime_task_cannot_stop_new_run_or_restore_old_configuration() {
    let h = Harness::new().await;
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    let before = h.backend.contexts.lock().unwrap().len();
    let mut stop = serde_json::to_value(call("stop.v1", args())).unwrap();
    stop["_meta"][names::key("revision")] = json!("new-revision");
    outcome(
        &h.client
            .call_tool(serde_json::from_value(stop).unwrap())
            .await
            .unwrap(),
        "succeeded",
    );
    // Yield until the cancelled lifetime task has had a chance to acquire the lock.
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
    assert!(
        h.backend.contexts.lock().unwrap()[before..]
            .iter()
            .all(|c| c.revision == "new-revision")
    );
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    assert!(h.backend.runs.lock().unwrap().contains_key(ID));
    h.close().await;
}

#[test]
fn command_declaration_uses_the_specification_path_rule() {
    let mut declaration = FakeBackend::default().declaration();
    for program in ["server", r"name\with\backslashes", "/absolute/server"] {
        assert!(declaration.supports(&json!({"command":[program]}), &machine()));
    }
    for program in ["./server", "relative/server"] {
        assert!(!declaration.supports(&json!({"command":[program]}), &machine()));
    }
    declaration.command = false;
    assert!(!declaration.supports(&json!({"command":["server"]}), &machine()));
}

#[tokio::test]
async fn call_drain_refreshes_owed_contexts_and_preserves_other_host_machine() {
    let h = Harness::new().await;
    let mut current = record(ID);
    current.owed_stop = true;
    let mut other = current.clone();
    other.context.machine.hostname = "historical-machine".into();
    other
        .context
        .machine
        .labels
        .insert("status".into(), "historical".into());
    h.store.save(&current).await.unwrap();
    h.store.save(&other).await.unwrap();
    h.backend.unavailable.store(true, Ordering::SeqCst);
    let mut input = args();
    input["machine"]["labels"] = json!({"status":"fresh"});
    let mut wire = serde_json::to_value(call("inspect-run.v1", input)).unwrap();
    wire["_meta"][names::key("config")] = json!({"changed":true});
    wire["_meta"][names::key("revision")] = json!("r2");
    h.backend.contexts.lock().unwrap().clear();
    outcome(
        &h.client
            .call_tool(serde_json::from_value(wire).unwrap())
            .await
            .unwrap(),
        "succeeded",
    );
    {
        let contexts = h.backend.contexts.lock().unwrap();
        assert!(contexts.len() >= 3);
        assert!(
            contexts
                .iter()
                .all(|c| c.revision == "r2" && c.config == json!({"changed":true}))
        );
        assert!(contexts.iter().any(|c| c.machine == other.context.machine));
        assert!(
            contexts
                .iter()
                .filter(|c| c.machine.hostname == machine().hostname)
                .all(|c| c.machine.labels.get("status").map(String::as_str) == Some("fresh"))
        );
    }
    // Even a stale ledger must not overwrite what the call taught the server at shutdown.
    h.store.save(&current).await.unwrap();
    h.store.fail_load.store(true, Ordering::SeqCst);
    h.backend.contexts.lock().unwrap().clear();
    assert!(h.server.shutdown().await.is_err());
    assert!(
        h.backend
            .contexts
            .lock()
            .unwrap()
            .iter()
            .all(|c| c.revision == "r2")
    );
    h.close().await;
}

async fn unsaved_obligation_drains(interval: bool) {
    let h = Harness::with_options(Options {
        drain_interval: if interval {
            Duration::from_millis(60)
        } else {
            options().drain_interval
        },
        ..options()
    })
    .await;
    // A durable running record predates the lost intent. No gate is needed for this run.
    h.backend.seed(ID);
    h.store.save(&record(ID)).await.unwrap();
    h.store.fail_save.store(true, Ordering::SeqCst);
    h.backend.unavailable.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    assert!(!h.store.load().await.unwrap()[0].owed_stop);
    h.store.fail_save.store(false, Ordering::SeqCst);
    h.backend.unavailable.store(false, Ordering::SeqCst);
    if interval {
        eventually(|| {
            let records: Vec<RunRecord> = serde_json::from_slice(&h.store.bytes()).unwrap();
            records[0].state == RunState::Stopped && !records[0].owed_stop
        })
        .await;
    } else {
        let result = h.call("inspect-run.v1", args()).await;
        outcome(&result, "succeeded");
        assert!(result.structured_content.unwrap()["state"] == "stopped");
    }
    assert!(h.backend.runs.lock().unwrap().is_empty());
    let records = h.store.load().await.unwrap();
    assert!(records[0].state == RunState::Stopped && !records[0].owed_stop);
    h.close().await;
}
#[tokio::test]
async fn unsaved_obligation_is_retried_by_call_drain() {
    unsaved_obligation_drains(false).await;
}
#[tokio::test]
async fn unsaved_obligation_is_retried_by_interval_drain() {
    unsaved_obligation_drains(true).await;
}
#[tokio::test]
async fn unsaved_obligation_blocks_start_and_inspection_even_when_observed_stopped() {
    let h = Harness::new().await;
    h.store.fail_save.store(true, Ordering::SeqCst);
    h.backend.unavailable.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    // Backend state says stopped, and no durable obligation exists.
    let inspected = h.call("inspect-run.v1", args()).await;
    assert!(inspected.structured_content.unwrap()["state"] == "unknown");
    outcome(&h.call("run.v1", run_args()).await, "unknown");
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    // A confirmed stop clears the memory obligation even while writes still fail.
    h.backend.unavailable.store(false, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    h.store.fail_save.store(false, Ordering::SeqCst);
    outcome(&h.call("run.v1", run_args()).await, "succeeded");
    h.close().await;
}
#[tokio::test]
async fn startup_write_failure_still_stops_every_reported_run_and_refuses_construction() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(FileStore::new(directory.path().join("ledger.json")));
    let backend = Arc::new(FakeBackend::default());
    for id in [
        ID,
        "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb",
        "cccccccc-cccc-4ccc-8ccc-cccccccccccc",
    ] {
        backend.seed(id);
    }
    store.fail_save.store(true, Ordering::SeqCst);
    let result = Server::new(
        backend.clone(),
        store,
        Arc::new(FakeGate::default()),
        options(),
    )
    .await;
    assert!(result.is_err());
    assert!(backend.stops.load(Ordering::SeqCst) == 3);
    assert!(backend.runs.lock().unwrap().is_empty());
}
#[tokio::test]
async fn startup_stop_failures_with_saved_intents_still_serve() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(FileStore::new(directory.path().join("ledger.json")));
    let backend = Arc::new(FakeBackend::default());
    backend.seed(ID);
    backend.unavailable.store(true, Ordering::SeqCst);
    let server = Server::new(
        backend.clone(),
        store.clone(),
        Arc::new(FakeGate::default()),
        options(),
    )
    .await
    .unwrap();
    assert!(store.load().await.unwrap()[0].owed_stop);
    assert!(server.shutdown().await.is_err());
}
#[tokio::test]
async fn recovered_context_survives_unreadable_ledger_before_any_call() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(FileStore::new(directory.path().join("ledger.json")));
    let backend = Arc::new(FakeBackend::default());
    backend.seed(ID);
    backend.unavailable.store(true, Ordering::SeqCst);
    let server = Server::new(
        backend.clone(),
        store.clone(),
        Arc::new(FakeGate::default()),
        options(),
    )
    .await
    .unwrap();
    backend.unavailable.store(false, Ordering::SeqCst);
    store.fail_load.store(true, Ordering::SeqCst);
    let before = backend.stops.load(Ordering::SeqCst);
    assert!(server.shutdown().await.is_err());
    assert!(backend.stops.load(Ordering::SeqCst) == before + 1);
    assert!(backend.runs.lock().unwrap().is_empty());
}

#[tokio::test]
async fn startup_final_state_write_failure_refuses_construction() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(FileStore::new(directory.path().join("ledger.json")));
    let backend = Arc::new(FakeBackend::default());
    backend.seed(ID);
    store.fail_stopped_save.store(true, Ordering::SeqCst);
    let result = Server::new(
        backend.clone(),
        store.clone(),
        Arc::new(FakeGate::default()),
        options(),
    )
    .await;
    assert!(result.is_err());
    assert!(backend.runs.lock().unwrap().is_empty());
    assert!(store.load().await.unwrap()[0].owed_stop);
}
#[tokio::test]
async fn startup_read_failure_still_stops_inventory_and_refuses_construction() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(FileStore::new(directory.path().join("ledger.json")));
    let backend = Arc::new(FakeBackend::default());
    backend.seed(ID);
    store.fail_load.store(true, Ordering::SeqCst);
    let result = Server::new(
        backend.clone(),
        store,
        Arc::new(FakeGate::default()),
        options(),
    )
    .await;
    assert!(result.is_err());
    assert!(backend.stops.load(Ordering::SeqCst) == 1);
    assert!(backend.runs.lock().unwrap().is_empty());
}
#[tokio::test]
async fn interval_drain_preserves_fresher_context_than_saved_ledger() {
    let h = Harness::with_options(Options {
        drain_interval: Duration::from_millis(60),
        ..options()
    })
    .await;
    let mut input = args();
    input["machine"]["labels"] = json!({"status":"fresh"});
    let mut wire = serde_json::to_value(call("inspect-run.v1", input)).unwrap();
    wire["_meta"][names::key("config")] = json!({"changed":true});
    wire["_meta"][names::key("revision")] = json!("r2");
    outcome(
        &h.client
            .call_tool(serde_json::from_value(wire).unwrap())
            .await
            .unwrap(),
        "succeeded",
    );
    let mut stale = record(ID);
    stale.owed_stop = true;
    h.backend.contexts.lock().unwrap().clear();
    h.store.save(&stale).await.unwrap();
    eventually(|| h.backend.stops.load(Ordering::SeqCst) > 0).await;
    assert!(
        h.backend
            .contexts
            .lock()
            .unwrap()
            .iter()
            .all(|c| c.revision == "r2"
                && c.config == json!({"changed":true})
                && c.machine.labels.get("status").map(String::as_str) == Some("fresh"))
    );
    h.close().await;
}

#[tokio::test]
async fn waiting_drain_rechecks_memory_after_confirmed_stop_clears_unsaved_obligation() {
    let h = Harness::new().await;
    h.store.fail_save.store(true, Ordering::SeqCst);
    h.backend.unavailable.store(true, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "unknown");
    h.backend.unavailable.store(false, Ordering::SeqCst);
    h.backend.pause_next_stop.store(true, Ordering::SeqCst);
    let before = h.backend.stops.load(Ordering::SeqCst);
    let first = h.client.call_tool(call("inspect-run.v1", args()));
    let check = async {
        h.backend.stop_entered.notified().await;
        let loads = h.store.loads.load(Ordering::SeqCst);
        let second = h.client.call_tool(call("inspect-run.v1", args()));
        let release = async {
            eventually(|| h.store.loads.load(Ordering::SeqCst) > loads).await;
            h.backend.resume_stop.notify_one();
        };
        let (second, ()) = tokio::join!(second, release);
        outcome(&second.unwrap(), "succeeded");
    };
    let (first, ()) =
        tokio::time::timeout(Duration::from_secs(3), async { tokio::join!(first, check) })
            .await
            .unwrap();
    let first = first.unwrap();
    assert!(first.structured_content.unwrap()["state"] == "stopped");
    assert!(h.backend.stops.load(Ordering::SeqCst) == before + 1);
    h.store.fail_save.store(false, Ordering::SeqCst);
    h.close().await;
}
#[tokio::test]
async fn run_whose_own_drain_clears_owed_stop_proceeds() {
    let h = Harness::new().await;
    h.backend.seed(ID);
    let mut owed = record(ID);
    owed.owed_stop = true;
    h.store.save(&owed).await.unwrap();
    let result = h.call("run.v1", run_args()).await;
    outcome(&result, "succeeded");
    assert!(h.backend.stops.load(Ordering::SeqCst) == 1);
    assert!(h.backend.starts.load(Ordering::SeqCst) == 1);
    assert!(*h.backend.call_order.lock().unwrap() == [("stop", ID.into()), ("start", ID.into())]);
    h.close().await;
}

#[tokio::test]
async fn run_whose_own_drain_cannot_clear_owed_stop_names_it_and_starts_nothing() {
    let h = Harness::new().await;
    h.backend.seed(ID);
    let mut owed = record(ID);
    owed.owed_stop = true;
    h.store.save(&owed).await.unwrap();
    h.backend.unavailable.store(true, Ordering::SeqCst);
    owed_stop_unknown(&h.call("run.v1", run_args()).await, &[(ID, "test-machine")]);
    assert!(h.backend.stops.load(Ordering::SeqCst) == 1);
    assert!(h.backend.stages.load(Ordering::SeqCst) == 0);
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    assert!(h.backend.runs.lock().unwrap().contains_key(ID));
    h.backend.unavailable.store(false, Ordering::SeqCst);
    h.close().await;
}

#[tokio::test]
async fn unknown_run_names_own_historical_stops_and_other_stops_on_call_machine() {
    for unsaved in [false, true] {
        let h = Harness::new().await;
        let mut historical = record(ID);
        historical.context.machine.hostname = "historical-machine".into();
        h.store.save(&historical).await.unwrap();
        h.store.fail_save.store(unsaved, Ordering::SeqCst);
        h.backend.unavailable.store(true, Ordering::SeqCst);
        outcome(&h.call("stop.v1", args()).await, "unknown");
        let mut other = args();
        other["instance"] = json!(SECOND_ID);
        outcome(&h.call("stop.v1", other).await, "unknown");
        owed_stop_unknown(
            &h.call("run.v1", run_args()).await,
            &[
                (ID, "test-machine"),
                (ID, "historical-machine"),
                (SECOND_ID, "test-machine"),
            ],
        );
        assert!(h.backend.stages.load(Ordering::SeqCst) == 0);
        assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
        h.store.fail_save.store(false, Ordering::SeqCst);
        h.backend.unavailable.store(false, Ordering::SeqCst);
        h.close().await;
    }
}

#[tokio::test]
async fn unreadable_own_state_names_every_other_owed_stop_on_call_machine() {
    for unsaved in [false, true] {
        for state_error in [false, true] {
            let h = Harness::new().await;
            h.backend.unavailable.store(true, Ordering::SeqCst);
            h.store.fail_save.store(unsaved, Ordering::SeqCst);
            outcome(&h.call("stop.v1", args()).await, "unknown");
            let third_id = "cccccccc-cccc-4ccc-8ccc-cccccccccccc";
            let mut third = args();
            third["instance"] = json!(third_id);
            outcome(&h.call("stop.v1", third).await, "unknown");
            let mut elsewhere = args();
            let elsewhere_id = "dddddddd-dddd-4ddd-8ddd-dddddddddddd";
            elsewhere["instance"] = json!(elsewhere_id);
            elsewhere["machine"]["hostname"] = json!("elsewhere");
            outcome(&h.call("stop.v1", elsewhere).await, "unknown");
            h.backend.fail_state.store(state_error, Ordering::SeqCst);
            h.backend.unknown.store(!state_error, Ordering::SeqCst);
            let result = h.call("run.v1", second_run_args()).await;
            owed_stop_unknown(&result, &[(ID, "test-machine"), (third_id, "test-machine")]);
            let wire = serde_json::to_value(result).unwrap();
            let message = wire["content"][0]["text"].as_str().unwrap();
            assert!(!message.contains(elsewhere_id));
            assert!(!message.contains("elsewhere"));
            assert!(h.backend.stages.load(Ordering::SeqCst) == 0);
            assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
            h.store.fail_save.store(false, Ordering::SeqCst);
            h.backend.unavailable.store(false, Ordering::SeqCst);
            h.close().await;
        }
    }
}

#[tokio::test]
async fn stop_v1_cancels_start_paused_in_staging_or_start() {
    for staging in [true, false] {
        let h = Harness::new().await;
        h.backend.pause_next_stage.store(staging, Ordering::SeqCst);
        h.backend.pause_next_start.store(!staging, Ordering::SeqCst);
        let peer = h.client.peer().clone();
        let run =
            tokio::spawn(async move { peer.call_tool(call("run.v1", run_args())).await.unwrap() });
        tokio::time::timeout(Duration::from_secs(3), async {
            if staging {
                h.backend.stage_entered.notified().await;
            } else {
                h.backend.entered.notified().await;
            }
            outcome(&h.call("stop.v1", args()).await, "succeeded");
            outcome(&run.await.unwrap(), "failed");
        })
        .await
        .unwrap();
        assert!(h.backend.starts.load(Ordering::SeqCst) == usize::from(!staging));
        assert!(h.backend.launches.load(Ordering::SeqCst) == 0);
        assert!(h.backend.runs.lock().unwrap().is_empty());
        assert!(
            h.store
                .load()
                .await
                .unwrap()
                .iter()
                .all(|r| !r.owed_stop && r.state == RunState::Stopped)
        );
        h.close().await;
    }
}

#[tokio::test]
async fn stop_v1_during_own_drain_cancels_waiting_start() {
    let h = Harness::new().await;
    h.backend.seed(ID);
    let mut owed = record(ID);
    owed.owed_stop = true;
    h.store.save(&owed).await.unwrap();
    h.backend.pause_next_stop.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let run =
        tokio::spawn(async move { peer.call_tool(call("run.v1", run_args())).await.unwrap() });
    tokio::time::timeout(Duration::from_secs(3), h.backend.stop_entered.notified())
        .await
        .unwrap();
    let loads = h.store.loads.load(Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let stop = tokio::spawn(async move { peer.call_tool(call("stop.v1", args())).await.unwrap() });
    // stop.v1 advances the epoch before its first load, even while the drain holds the lock.
    eventually(|| h.store.loads.load(Ordering::SeqCst) > loads).await;
    h.backend.resume_stop.notify_one();
    tokio::time::timeout(Duration::from_secs(3), async {
        outcome(&stop.await.unwrap(), "succeeded");
        outcome(&run.await.unwrap(), "failed");
    })
    .await
    .unwrap();
    assert!(h.backend.stages.load(Ordering::SeqCst) == 0);
    assert!(h.backend.starts.load(Ordering::SeqCst) == 0);
    assert!(h.backend.runs.lock().unwrap().is_empty());
    h.close().await;
}

#[tokio::test]
async fn another_calls_drain_before_operation_lock_does_not_cancel_start() {
    let h = Harness::new().await;
    h.backend.seed(ID);
    let mut owed = record(ID);
    owed.owed_stop = true;
    h.store.save(&owed).await.unwrap();
    h.store.pause_next_load.store(true, Ordering::SeqCst);
    let peer = h.client.peer().clone();
    let run =
        tokio::spawn(async move { peer.call_tool(call("run.v1", run_args())).await.unwrap() });
    tokio::time::timeout(Duration::from_secs(3), h.store.load_entered.notified())
        .await
        .unwrap();
    let mut other = args();
    other["instance"] = json!(SECOND_ID);
    outcome(&h.call("inspect-run.v1", other).await, "succeeded");
    h.store.resume_load.notify_one();
    outcome(
        &tokio::time::timeout(Duration::from_secs(3), run)
            .await
            .unwrap()
            .unwrap(),
        "succeeded",
    );
    assert!(h.backend.stops.load(Ordering::SeqCst) == 1);
    assert!(h.backend.starts.load(Ordering::SeqCst) == 1);
    assert!(*h.backend.call_order.lock().unwrap() == [("stop", ID.into()), ("start", ID.into())]);
    h.close().await;
}
