use super::*;
use crate::Config;
use rmcp::ServiceExt;
use std::time::Duration;
use tokio::sync::Notify;

#[test]
fn session_keep_alive_outlasts_call_deadline() {
    for call_timeout in [
        Config::default().call_timeout,
        Duration::from_secs(4 * 3600),
    ] {
        let config = Config {
            call_timeout,
            ..Config::default()
        };
        let keep_alive = session_manager(&config).session_config.keep_alive.unwrap();
        assert!(keep_alive > call_timeout);
        assert_eq!(keep_alive - call_timeout, Duration::from_secs(60));
    }
    let config = Config {
        call_timeout: Duration::MAX,
        ..Config::default()
    };
    assert!(session_manager(&config).session_config.keep_alive.is_none());
}

struct WaitingTool {
    started: Arc<Notify>,
    finish: Arc<Notify>,
}
impl ServerHandler for WaitingTool {
    async fn call_tool(
        &self,
        _: CallToolRequestParams,
        _: RequestContext<RoleServer>,
    ) -> std::result::Result<CallToolResponse, ErrorData> {
        self.started.notify_one();
        self.finish.notified().await;
        Ok(CallToolResult::success(vec![]).into())
    }
}

#[tokio::test(start_paused = true)]
async fn running_call_outlives_rmcp_default_idle_limit() {
    let config = Config {
        call_timeout: Duration::from_secs(4 * 3600),
        ..Config::default()
    };
    let manager = session_manager(&config);
    let (id, transport) = manager.create_session().await.unwrap();
    let started = Arc::new(Notify::new());
    let finish = Arc::new(Notify::new());
    let handler = WaitingTool {
        started: started.clone(),
        finish: finish.clone(),
    };
    let server = tokio::spawn(async move { handler.serve(transport).await.unwrap() });
    manager
        .initialize_session(
            &id,
            serde_json::from_value(json!({
                "jsonrpc":"2.0", "id":0, "method":"initialize",
                "params": {
                    "protocolVersion":"2025-06-18", "capabilities":{},
                    "clientInfo":{"name":"session-test","version":"1"}
                }
            }))
            .unwrap(),
        )
        .await
        .unwrap();
    manager
        .accept_message(
            &id,
            serde_json::from_value(json!({
                "jsonrpc":"2.0", "method":"notifications/initialized"
            }))
            .unwrap(),
        )
        .await
        .unwrap();
    let server = server.await.unwrap();
    let mut stream = manager
        .create_stream(
            &id,
            serde_json::from_value(json!({
                "jsonrpc":"2.0", "id":1, "method":"tools/call",
                "params":{"name":"wait","arguments":{}}
            }))
            .unwrap(),
        )
        .await
        .unwrap();
    started.notified().await;
    // Virtual time: no wall-clock sleep and no messages to reset the timer.
    tokio::time::advance(Duration::from_secs(301)).await;
    // Let expired timers settle before the handler can send its reply.
    tokio::time::sleep(Duration::from_millis(1)).await;
    finish.notify_one();
    let response = tokio::time::timeout(Duration::from_secs(1), async {
        while let Some(event) = stream.next().await {
            if let Some(message) = event.message {
                return serde_json::to_value(message.as_ref()).unwrap();
            }
        }
        panic!("session closed before the call returned");
    })
    .await
    .unwrap();
    assert_eq!(response["id"], 1);
    assert_eq!(response["result"]["isError"], false);
    manager.close_session(&id).await.unwrap();
    server.cancel().await.unwrap();
}
