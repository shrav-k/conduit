#![cfg(unix)]
use provider_local_machine::{EXTENSION, KEY_FILE, OUTCOME, STATE_VARIABLE, schemas};
use rmcp::{
    RoleClient, ServiceExt,
    model::*,
    service::{RunningService, ServiceError},
    transport::TokioChildProcess,
};
use serde_json::{Value, json};
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};

async fn start(path: &Path) -> RunningService<RoleClient, ()> {
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_provider-local-machine"));
    command
        .env_clear()
        .env(STATE_VARIABLE, path)
        .stderr(Stdio::null());
    tokio::time::timeout(
        Duration::from_secs(15),
        ().serve(TokioChildProcess::new(command).unwrap()),
    )
    .await
    .unwrap()
    .unwrap()
}
fn call(name: &str, args: Value) -> CallToolRequestParams {
    serde_json::from_value(json!({"name":name,"arguments":args,"_meta":{
        "io.github.shrav-k.conduit/instance":"provider-test",
        "io.github.shrav-k.conduit/config":{},
        "io.github.shrav-k.conduit/revision":"r1",
        "io.github.shrav-k.conduit/operation":format!("github.com/shrav-k/conduit/core#{name}"),
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}}
    }}))
    .unwrap()
}
async fn outputs(client: &RunningService<RoleClient, ()>, name: &str, args: Value) -> Value {
    let result = client.call_tool(call(name, args)).await.unwrap();
    let wire = serde_json::to_value(result).unwrap();
    assert!(wire["_meta"][OUTCOME] == "succeeded");
    assert!(wire["isError"] == false);
    assert!(schemas::valid(
        name,
        "outputSchema",
        &wire["structuredContent"]
    ));
    wire["structuredContent"].clone()
}
async fn list(client: &RunningService<RoleClient, ()>) -> Value {
    let result = outputs(client, "list-machines.v1", json!({})).await;
    assert!(result["machines"].as_array().unwrap().len() == 1);
    result["machines"][0].clone()
}
#[tokio::test]
async fn discovery_and_capability_refusal_do_not_initialize_identity() {
    let directory = tempfile::tempdir().unwrap();
    let client = start(directory.path()).await;
    let discovery = client.discover(serde_json::from_value(json!({"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}})).unwrap()).await.unwrap();
    let wire = serde_json::to_value(discovery).unwrap();
    assert!(wire["capabilities"]["extensions"][EXTENSION]["revision"] == "v0");
    let tools = client.list_all_tools().await.unwrap();
    assert!(tools.len() == 2);
    for tool in tools {
        let wire = serde_json::to_value(&tool).unwrap();
        assert!(schemas::OPERATIONS.contains(&tool.name.as_ref()));
        assert!(wire["outputSchema"] == schemas::schema(&tool.name, "outputSchema").unwrap());
        assert!(
            wire["inputSchema"]
                == schemas::schema(&tool.name, "inputSchema").unwrap_or(json!({"type":"object"}))
        );
    }
    for operation in schemas::OPERATIONS {
        for revision in [None, Some("v1")] {
            let mut request = serde_json::to_value(call(operation, json!({}))).unwrap();
            request["_meta"]["io.modelcontextprotocol/clientCapabilities"] = match revision {
                None => json!({}),
                Some(value) => json!({"extensions":{EXTENSION:{"revision":value}}}),
            };
            let error = client
                .call_tool(serde_json::from_value(request).unwrap())
                .await
                .err()
                .unwrap();
            let ServiceError::McpError(error) = error else {
                panic!("expected MCP capability error")
            };
            assert!(error.code.0 == -32021);
            assert!(
                error.data.unwrap()["requiredCapabilities"]["extensions"][EXTENSION]["revision"]
                    == "v0"
            );
            assert!(fs::read_dir(directory.path()).unwrap().count() == 0);
        }
    }
    client.cancel().await.unwrap();
}
#[tokio::test]
async fn canonical_report_and_all_selectors() {
    let directory = tempfile::tempdir().unwrap();
    let client = start(directory.path()).await;
    let machine = list(&client).await;
    let raw = hostname::get().unwrap();
    let expected_hostname = raw.to_str().unwrap().trim().to_lowercase();
    assert!(machine["hostname"] == expected_hostname);
    assert!(machine["state"] == "reachable");
    let os = match std::env::consts::OS {
        "macos" => "darwin",
        os => os,
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" if cfg!(target_endian = "little") => "arm64",
        "aarch64" => "arm64be",
        "arm" if cfg!(target_endian = "big") => "armbe",
        "riscv32" => "riscv",
        "wasm32" | "wasm64" => "wasm",
        "x86" => "386",
        "loongarch64" => "loong64",
        "powerpc" => "ppc",
        "powerpc64" if cfg!(target_endian = "little") => "ppc64le",
        "powerpc64" => "ppc64",
        "mips" if cfg!(target_endian = "little") => "mipsle",
        "mips64" if cfg!(target_endian = "little") => "mips64le",
        other => other,
    };
    assert!(
        machine["labels"]
            == json!({"kubernetes.io/hostname":expected_hostname,"kubernetes.io/os":os,"kubernetes.io/arch":arch})
    );
    for selector in [
        json!({}),
        machine["labels"].clone(),
        json!({"kubernetes.io/hostname":expected_hostname}),
    ] {
        let placed = outputs(
            &client,
            "place.v1",
            json!({"instance":"test-instance","nodeSelector":selector}),
        )
        .await;
        assert!(placed["hostname"] == expected_hostname);
    }
    let another = if expected_hostname == "another-machine" {
        "different-machine"
    } else {
        "another-machine"
    };
    for selector in [
        json!({"kubernetes.io/hostname":another}),
        json!({"unknown-label":"value"}),
    ] {
        let placed = outputs(
            &client,
            "place.v1",
            json!({"instance":"test-instance","nodeSelector":selector}),
        )
        .await;
        assert!(placed == json!({}));
    }
    // No input schema means arbitrary object properties remain accepted.
    let result = outputs(&client, "list-machines.v1", json!({"ignored":true})).await;
    assert!(result["machines"][0]["id"] == machine["id"]);
    client.cancel().await.unwrap();
}
#[tokio::test]
async fn invalid_place_arguments_fail_before_creating_a_key() {
    let directory = tempfile::tempdir().unwrap();
    let client = start(directory.path()).await;
    for args in [
        json!({}),
        json!({"instance":"x","nodeSelector":null}),
        json!({"instance":"","nodeSelector":{}}),
        json!({"instance":"x","nodeSelector":{"bad key":"value"}}),
        json!({"instance":"x","nodeSelector":{"label":7}}),
    ] {
        let wire =
            serde_json::to_value(client.call_tool(call("place.v1", args)).await.unwrap()).unwrap();
        assert!(wire["_meta"][OUTCOME] == "failed");
        assert!(wire["isError"] == true);
        assert!(wire.get("structuredContent").is_none());
        assert!(fs::read_dir(directory.path()).unwrap().count() == 0);
    }
    client.cancel().await.unwrap();
}
#[tokio::test]
async fn identity_survives_restart_is_private_and_is_provider_scoped() {
    use std::os::unix::fs::PermissionsExt;
    let directory = tempfile::tempdir().unwrap();
    let client = start(directory.path()).await;
    let first = list(&client).await;
    client.cancel().await.unwrap();
    assert!(fs::metadata(directory.path()).unwrap().permissions().mode() & 0o777 == 0o700);
    assert!(
        fs::metadata(directory.path().join(KEY_FILE))
            .unwrap()
            .permissions()
            .mode()
            & 0o777
            == 0o600
    );
    assert!(fs::read_dir(directory.path()).unwrap().count() == 1);
    let client = start(directory.path()).await;
    let second = list(&client).await;
    assert!(first["id"] == second["id"]);
    client.cancel().await.unwrap();
    let fresh = tempfile::tempdir().unwrap();
    let client = start(fresh.path()).await;
    assert!(first["id"] != list(&client).await["id"]);
    client.cancel().await.unwrap();
}
#[tokio::test]
async fn simultaneous_first_runs_install_one_key() {
    let directory = tempfile::tempdir().unwrap();
    let a = start(directory.path()).await;
    let b = start(directory.path()).await;
    let (one, two) = tokio::join!(list(&a), list(&b));
    assert!(one["id"] == two["id"]);
    a.cancel().await.unwrap();
    b.cancel().await.unwrap();
    assert!(fs::read_dir(directory.path()).unwrap().count() == 1);
}
#[test]
fn missing_or_empty_state_directory_refuses_start() {
    for empty in [false, true] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_provider-local-machine"));
        command.env_clear().stdin(Stdio::null());
        if empty {
            command.env(STATE_VARIABLE, "");
        }
        let result = command.output().unwrap();
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains(STATE_VARIABLE));
    }
}
#[tokio::test]
async fn corrupt_or_symlinked_key_is_never_replaced() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let directory = tempfile::tempdir().unwrap();
    let key = directory.path().join(KEY_FILE);
    fs::write(&key, b"invalid").unwrap();
    fs::set_permissions(&key, fs::Permissions::from_mode(0o600)).unwrap();
    let client = start(directory.path()).await;
    for _ in 0..2 {
        let wire = serde_json::to_value(
            client
                .call_tool(call("list-machines.v1", json!({})))
                .await
                .unwrap(),
        )
        .unwrap();
        assert!(wire["_meta"][OUTCOME] == "failed");
        assert!(fs::metadata(&key).unwrap().len() == 7);
    }
    client.cancel().await.unwrap();
    fs::remove_file(&key).unwrap();
    let elsewhere = tempfile::tempdir().unwrap();
    let destination = elsewhere.path().join("missing-key");
    symlink(&destination, &key).unwrap();
    let client = start(directory.path()).await;
    let wire = serde_json::to_value(
        client
            .call_tool(call("list-machines.v1", json!({})))
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(wire["_meta"][OUTCOME] == "failed");
    assert!(!destination.exists());
    assert!(fs::symlink_metadata(key).unwrap().file_type().is_symlink());
    client.cancel().await.unwrap();
}
