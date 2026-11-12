use super::support::*;
use runtime_provider_kit::*;
use serde_json::{Value, json};
use std::sync::{Arc, atomic::Ordering};

const WITHHELD: &str =
    "runtime's reason was withheld because it contained a value supplied to the run";

fn text(result: &rmcp::model::CallToolResult) -> String {
    serde_json::to_value(result).unwrap()["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_owned()
}

async fn records_have_no_reason(h: &Harness, reason: Option<&str>) {
    let records = h.store.load().await.unwrap();
    let backend = h.backend.runs.lock().unwrap();
    let records = records.iter().chain(backend.values().map(|(r, _)| r));
    let mut stored = vec![String::from_utf8(h.store.bytes()).unwrap()];
    for record in records {
        let wire = serde_json::to_value(record).unwrap();
        assert!(wire.as_object().unwrap().len() == 4);
        assert!(wire.get("reason").is_none());
        stored.push(wire.to_string());
    }
    for bytes in stored {
        if let Some(reason) = reason.filter(|r| !r.is_empty()) {
            assert!(!bytes.contains(reason), "reason entered a record");
        }
        for marker in markers() {
            assert!(!bytes.contains(&marker), "sensitive record");
        }
    }
}

// Exercise staging, confirmed start cleanup, and unconfirmed start cleanup for each reason.
async fn failures(reason: Option<&str>, expected: Option<&str>, input: Value) {
    for failure in 0..3 {
        let h = Harness::new().await;
        if failure == 0 {
            h.backend.fail_stage.store(true, Ordering::SeqCst);
            *h.backend.stage_reason.lock().unwrap() = reason.map(str::to_owned);
        } else {
            h.backend.fail_start.store(true, Ordering::SeqCst);
            *h.backend.start_reason.lock().unwrap() = reason.map(str::to_owned);
            h.backend.unavailable.store(failure == 2, Ordering::SeqCst);
        }
        let result = h.call("run.v1", input.clone()).await;
        outcome(&result, if failure == 2 { "unknown" } else { "failed" });
        let mut message = if failure == 0 {
            "staging failed".to_owned()
        } else {
            "start did not complete".to_owned()
        };
        if let Some(reason) = expected {
            message.push_str(": ");
            message.push_str(reason);
        }
        if failure != 0 {
            message.push_str(if failure == 1 {
                "; tree stopped"
            } else {
                "; stop could not be confirmed and persisted"
            });
        }
        // Never print either side: some test inputs deliberately contain sensitive markers.
        assert!(text(&result) == message);
        assert!(result.structured_content.is_none());
        records_have_no_reason(&h, reason).await;
        let records = h.store.load().await.unwrap();
        assert!(records.len() == 1);
        assert!(records[0].owed_stop == (failure == 2));
        assert!(
            records[0].state
                == if failure == 2 {
                    RunState::Unknown
                } else {
                    RunState::Stopped
                }
        );
        if failure == 2 {
            assert!(h.backend.runs.lock().unwrap().len() == 1);
        } else {
            assert!(h.backend.runs.lock().unwrap().is_empty());
        }
        h.backend.unavailable.store(false, Ordering::SeqCst);
        h.server.shutdown().await.unwrap();
        records_have_no_reason(&h, reason).await;
        h.close().await;
    }
}

#[tokio::test]
async fn runtime_reasons_reach_only_failure_text_with_unchanged_outcomes() {
    failures(
        Some("program fake-server cannot be resolved on test-machine"),
        Some("program fake-server cannot be resolved on test-machine"),
        run_args(),
    )
    .await;
    failures(None, None, run_args()).await;
}

#[tokio::test]
async fn start_reason_survives_unconfirmed_persistence_without_entering_the_store() {
    let h = Harness::new().await;
    let reason = "program fake-server failed to initialize";
    h.backend.fail_start.store(true, Ordering::SeqCst);
    *h.backend.start_reason.lock().unwrap() = Some(reason.into());
    h.store.fail_stopped_save.store(true, Ordering::SeqCst);
    let result = h.call("run.v1", run_args()).await;
    outcome(&result, "unknown");
    assert!(
        text(&result)
            == format!(
                "start did not complete: {reason}; stop could not be confirmed and persisted"
            )
    );
    assert!(h.backend.runs.lock().unwrap().is_empty());
    assert!(h.store.load().await.unwrap()[0].owed_stop);
    records_have_no_reason(&h, Some(reason)).await;
    h.store.fail_stopped_save.store(false, Ordering::SeqCst);
    h.server.shutdown().await.unwrap();
    records_have_no_reason(&h, Some(reason)).await;
    h.close().await;
}

#[tokio::test]
async fn whole_values_from_each_sensitive_source_are_withheld() {
    let input = run_args();
    for bindings in [
        &input["identity"]["environment"],
        &input["identity"]["files"],
        &input["secrets"]["environment"],
        &input["secrets"]["packageArguments"],
    ] {
        // Check every supplied binding, rather than just one member of each source.
        for value in bindings.as_object().unwrap().values() {
            let reason = format!("runtime refused {} during setup", value.as_str().unwrap());
            failures(Some(&reason), Some(WITHHELD), input.clone()).await;
        }
    }
}

#[tokio::test]
async fn multiline_values_are_withheld_in_raw_and_one_line_reasons_before_cutting() {
    let mut input = run_args();
    let filename = schemas::identity_file_names()[0];
    // Replace the only binding containing this marker; no other complete sensitive value
    // may match the one-line reason and hide a failure of normalized matching.
    let value = format!(
        "{}\nidentity-file-continuation\r\nend\t\u{2028}\u{2029}",
        markers()[0]
    );
    input["identity"]["files"][filename] = json!(value);
    let cleaned = value.replace(['\n', '\r', '\t', '\u{2028}', '\u{2029}'], " ");
    for reason in [
        format!("cannot stage {value}"),
        format!("cannot stage {cleaned}"),
        format!("{} {cleaned}", "x".repeat(600)),
    ] {
        failures(Some(&reason), Some(WITHHELD), input.clone()).await;
    }
}

#[tokio::test]
async fn reasons_are_one_line_bounded_and_blank_reasons_disappear() {
    failures(
        Some("program\nname\r\t\0\u{7f}\u{85}\u{2028}\u{2029}missing"),
        Some("program name       missing"),
        run_args(),
    )
    .await;
    let long = format!("{}é界🦀tail", "x".repeat(510));
    let bounded = format!("{}é界", "x".repeat(510));
    failures(Some(&long), Some(&bounded), run_args()).await;
    for blank in ["", " \t\n\u{2028}\u{2029}\u{2003}"] {
        failures(Some(blank), None, run_args()).await;
    }
}

#[tokio::test]
async fn ordinary_values_arguments_fragments_and_empty_sensitive_values_do_not_withhold() {
    let mut input = run_args();
    input["secrets"]["environment"]["EMPTY"] = json!("");
    input["environment"]["ORDINARY"] = json!("p");
    input["packageArguments"]["ordinary"] = json!("r");
    input["item"]["command"] = json!(["fake-server", "program"]);
    failures(
        Some("program public sensitive- failed"),
        Some("program public sensitive- failed"),
        input,
    )
    .await;
}

#[test]
fn unavailable_keeps_its_value_type_comparisons_and_private_debug() {
    let plain: std::result::Result<(), Unavailable> = Err(Unavailable);
    let alias: runtime_provider_kit::Result<()> = None.ok_or(Unavailable);
    let mapped = Err::<(), _>(()).map_err(|_| Unavailable);
    assert!(plain == alias && alias == mapped);
    assert!(plain == Err(Unavailable));
    let with_reason = Unavailable::because(markers()[0].clone());
    assert!(with_reason.clone() == with_reason);
    assert!(format!("{with_reason:?}") == "Unavailable");
    assert!(format!("{Unavailable:?}") == "Unavailable");
    assert!(format!("{:?}", Err::<(), _>(with_reason)) == "Err(Unavailable)");
}

#[tokio::test]
async fn all_other_failed_start_callers_keep_exact_text_and_drop_unrelated_reasons() {
    for unconfirmed in [false, true] {
        for failure in 0..5 {
            let h = Harness::new().await;
            let ignored = "unrelated setup diagnostic";
            *h.backend.failure_reason.lock().unwrap() = Some(ignored.into());
            *h.gate.failure_reason.lock().unwrap() = Some(ignored.into());
            *h.store.failure_reason.lock().unwrap() = Some(ignored.into());
            h.backend.unavailable.store(unconfirmed, Ordering::SeqCst);
            match failure {
                0 => h.backend.cancel_after_stage.store(true, Ordering::SeqCst),
                1 => h.backend.cancel_after_start.store(true, Ordering::SeqCst),
                2 => h.gate.fail.store(true, Ordering::SeqCst),
                3 => *h.gate.url_override.lock().unwrap() = Some("invalid URL".into()),
                _ => h.store.fail_running_save.store(true, Ordering::SeqCst),
            }
            let result = h.call("run.v1", run_args()).await;
            outcome(&result, if unconfirmed { "unknown" } else { "failed" });
            assert!(
                text(&result)
                    == if unconfirmed {
                        "start did not complete; stop could not be confirmed and persisted"
                    } else {
                        "start did not complete; tree stopped"
                    }
            );
            records_have_no_reason(&h, Some(ignored)).await;
            h.backend.unavailable.store(false, Ordering::SeqCst);
            h.server.shutdown().await.unwrap();
            records_have_no_reason(&h, Some(ignored)).await;
            h.close().await;
        }
    }
}

#[tokio::test]
async fn stop_state_store_and_inventory_reasons_are_dropped() {
    let ignored = "unrelated setup diagnostic";
    let h = Harness::new().await;
    *h.backend.failure_reason.lock().unwrap() = Some(ignored.into());
    *h.store.failure_reason.lock().unwrap() = Some(ignored.into());
    h.backend.fail_state.store(true, Ordering::SeqCst);
    let result = h.call("run.v1", run_args()).await;
    outcome(&result, "unknown");
    assert!(text(&result) == "run may still be running");
    let result = h.call("inspect-run.v1", args()).await;
    outcome(&result, "succeeded");
    assert!(text(&result) == "run state inspected");
    h.backend.fail_state.store(false, Ordering::SeqCst);
    h.backend.unavailable.store(true, Ordering::SeqCst);
    let result = h.call("stop.v1", args()).await;
    outcome(&result, "unknown");
    assert!(text(&result) == "run stop could not be confirmed and persisted");
    records_have_no_reason(&h, Some(ignored)).await;
    h.backend.unavailable.store(false, Ordering::SeqCst);
    outcome(&h.call("stop.v1", args()).await, "succeeded");
    h.store.fail_save.store(true, Ordering::SeqCst);
    let result = h.call("run.v1", run_args()).await;
    outcome(&result, "failed");
    assert!(text(&result) == "cannot persist run intent");
    h.store.fail_save.store(false, Ordering::SeqCst);
    h.store.fail_load.store(true, Ordering::SeqCst);
    let result = h.call("run.v1", run_args()).await;
    outcome(&result, "unknown");
    assert!(text(&result) == "cannot read durable stop obligations");
    assert!(drain_owed_stops(h.backend.as_ref(), h.store.as_ref()).await == Err(Unavailable));
    h.store.fail_load.store(false, Ordering::SeqCst);
    records_have_no_reason(&h, Some(ignored)).await;
    h.close().await;

    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(FileStore::new(directory.path().join("ledger.json")));
    let backend = Arc::new(FakeBackend::default());
    backend.fail_inventory.store(true, Ordering::SeqCst);
    *backend.failure_reason.lock().unwrap() = Some(ignored.into());
    let result = Server::new(
        backend,
        store.clone(),
        Arc::new(FakeGate::default()),
        options(),
    )
    .await;
    assert!(result.err() == Some(Unavailable));
    assert!(!String::from_utf8(store.bytes()).unwrap().contains(ignored));
}

#[tokio::test]
async fn gate_urls_still_check_every_sensitive_source() {
    let input = run_args();
    for bindings in [
        &input["identity"]["environment"],
        &input["identity"]["files"],
        &input["secrets"]["environment"],
        &input["secrets"]["packageArguments"],
    ] {
        for value in bindings.as_object().unwrap().values() {
            let h = Harness::new().await;
            *h.gate.url_override.lock().unwrap() = Some(format!(
                "https://example.invalid/mcp?value={}",
                value.as_str().unwrap()
            ));
            let result = h.call("run.v1", input.clone()).await;
            outcome(&result, "failed");
            assert!(text(&result) == "start did not complete; tree stopped");
            records_have_no_reason(&h, None).await;
            h.close().await;
        }
    }
}
