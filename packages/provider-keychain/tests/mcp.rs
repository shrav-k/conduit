mod common;

use common::{Client, failed, request, with_meta};
use provider_keychain::{EXTENSION, OUTCOME, PREFIX, REASON, RESTRICTION};
use rmcp::service::ServiceError;
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn discovery_advertises_the_extension_and_requires_it_for_reads() {
    let client = Client::start().await;
    let discovery = timeout(
        Duration::from_secs(5),
        client.service().discover(
            serde_json::from_value(json!({
                "io.modelcontextprotocol/protocolVersion":"2026-07-28",
                "io.modelcontextprotocol/clientCapabilities":{}
            }))
            .unwrap(),
        ),
    )
    .await
    .expect("discovery timed out")
    .unwrap_or_else(|_| panic!("discovery failed"));
    let info = serde_json::to_value(discovery).unwrap();
    assert_eq!(
        info["capabilities"]["extensions"][EXTENSION]["revision"],
        "v0"
    );
    let tools = timeout(Duration::from_secs(5), client.service().list_all_tools())
        .await
        .expect("tool listing timed out")
        .unwrap_or_else(|_| panic!("tool listing failed"));
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "get-secret.v1");
    let tool = serde_json::to_value(&tools[0]).unwrap();
    assert!(tool["outputSchema"] == provider_keychain::output_schema());
    for capabilities in [
        json!({}),
        json!({"extensions":{EXTENSION:{"revision":"wrong"}}}),
    ] {
        let response = timeout(
            Duration::from_secs(5),
            client.service().call_tool(with_meta(
                request(json!({})),
                "io.modelcontextprotocol/clientCapabilities",
                capabilities,
            )),
        )
        .await
        .expect("extension refusal timed out");
        let Err(ServiceError::McpError(error)) = response else {
            panic!("missing or unsupported extension was not refused");
        };
        assert_eq!(error.code.0, -32021);
        assert_eq!(
            error.data.unwrap()["requiredCapabilities"]["extensions"][EXTENSION]["revision"],
            "v0"
        );
    }
    client.stop().await;
}

#[tokio::test]
async fn filters_are_refused_before_config_or_read() {
    let client = Client::start().await;
    let restriction = json!({"operation":"filter.v1","arguments":{"label":"selected"}});
    let response = client
        .call(with_meta(
            request(json!("invalid locator")),
            &format!("{PREFIX}filters"),
            json!([restriction]),
        ))
        .await;
    assert!(response["isError"] == true);
    assert!(response["_meta"][REASON] == "unavailable");
    assert!(response["_meta"][RESTRICTION] == restriction);
    assert!(response["_meta"].get(OUTCOME).is_none());
    assert!(response.get("structuredContent").is_none());
    client.stop().await;
}

#[tokio::test]
async fn malformed_locators_fail_before_any_read() {
    let client = Client::start().await;
    let absolute = std::env::temp_dir().join("conduit-nonexistent-keychain");
    let keys = "keychain store could not be read: configuration must contain only keychain, service and account";
    for (locator, message) in [
        (json!("locator"), keys),
        (json!({"keychain":absolute,"service":"store"}), keys),
        (
            json!({"keychain":absolute,"service":"store","account":"test","extra":true}),
            keys,
        ),
        (
            json!({"keychain":"relative","service":"store","account":"test"}),
            "keychain store could not be read: keychain must be an absolute path",
        ),
        (
            json!({"keychain":absolute,"service":"","account":"test"}),
            "keychain store could not be read: service must be a non-empty string",
        ),
        (
            json!({"keychain":absolute,"service":"store","account":""}),
            "keychain store could not be read: account must be a non-empty string",
        ),
    ] {
        failed(&client.call(request(locator)).await, message);
    }
    client.stop().await;
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn a_well_formed_read_requires_macos() {
    let client = Client::start().await;
    let absolute = std::env::temp_dir().join("conduit-nonexistent-keychain");
    failed(
        &client
            .call(request(
                json!({"keychain":absolute,"service":"store","account":"test"}),
            ))
            .await,
        "keychain store could not be read: the store runs only on macOS",
    );
    client.stop().await;
}
