#![allow(dead_code)]
use provider_keychain::{EXTENSION, OPERATION, OUTCOME};
use rmcp::{
    RoleClient, ServiceExt, model::*, service::RunningService, transport::TokioChildProcess,
};
use serde_json::{Value, json};
use std::{process::Stdio, time::Duration};
use tokio::{io::AsyncReadExt, task::JoinHandle};

pub struct Client {
    service: RunningService<RoleClient, ()>,
    stderr: JoinHandle<Vec<u8>>,
}

impl Client {
    pub fn service(&self) -> &RunningService<RoleClient, ()> {
        &self.service
    }

    pub async fn start() -> Self {
        let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_provider-keychain"));
        command.env_clear();
        let (transport, stderr) = TokioChildProcess::builder(command)
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn provider");
        let mut stderr = stderr.expect("capture stderr");
        let stderr = tokio::spawn(async move {
            let mut bytes = Vec::new();
            stderr
                .read_to_end(&mut bytes)
                .await
                .expect("capture provider stderr");
            bytes
        });
        let service = tokio::time::timeout(Duration::from_secs(15), ().serve(transport))
            .await
            .expect("provider startup timed out")
            .unwrap_or_else(|_| panic!("provider startup failed"));
        Self { service, stderr }
    }

    pub async fn call(&self, request: CallToolRequestParams) -> Value {
        let result = tokio::time::timeout(Duration::from_secs(5), self.service.call_tool(request))
            .await
            .expect("call timed out")
            .unwrap_or_else(|_| panic!("MCP call failed"));
        serde_json::to_value(result).expect("serialize result")
    }

    pub async fn stop(self) {
        self.service
            .cancel()
            .await
            .unwrap_or_else(|_| panic!("provider shutdown failed"));
        let bytes = tokio::time::timeout(Duration::from_secs(20), self.stderr)
            .await
            .expect("stderr capture timed out")
            .expect("stderr task failed");
        assert!(bytes.is_empty(), "provider wrote a stderr line");
    }
}

pub fn request(config: Value) -> CallToolRequestParams {
    serde_json::from_value(json!({"name":"get-secret.v1","arguments":{},"_meta":{
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}},
        "io.github.shrav-k.conduit/instance":"secret-instance",
        "io.github.shrav-k.conduit/operation":OPERATION,
        "io.github.shrav-k.conduit/invocation":"test-invocation",
        "io.github.shrav-k.conduit/caller":"setup-caller",
        "io.github.shrav-k.conduit/filters":[],
        "io.github.shrav-k.conduit/config":config,
        "io.github.shrav-k.conduit/revision":"r1",
        "io.github.shrav-k.conduit/conduit":"instance-secret-conduit"
    }}))
    .expect("build request")
}

pub fn with_meta(request: CallToolRequestParams, key: &str, value: Value) -> CallToolRequestParams {
    let mut wire = serde_json::to_value(request).expect("serialize request");
    wire["_meta"][key] = value;
    serde_json::from_value(wire).expect("build request")
}

pub fn failed(wire: &Value, message: &str) {
    assert!(
        wire["isError"] == true && wire["_meta"][OUTCOME] == "failed",
        "expected failed outcome"
    );
    assert!(
        wire.get("structuredContent").is_none(),
        "failed read included a value"
    );
    assert!(
        wire["content"][0]["text"] == message,
        "unexpected failure diagnostic"
    );
}
