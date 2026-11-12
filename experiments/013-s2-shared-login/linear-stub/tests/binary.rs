use std::{
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::Duration,
};

use reqwest::{Client, StatusCode};
use serde_json::{Value, json};
use uuid::Uuid;

struct Run {
    directory: PathBuf,
    child: Option<Child>,
}

impl Drop for Run {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.kill();
            let _ = child.wait();
        }
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

#[tokio::test]
async fn binary_loads_fixture_publishes_private_connection_file_and_refuses_overwrite() {
    let mut run = Run {
        directory: std::env::temp_dir().join(format!("s2-stub-test-{}", Uuid::new_v4())),
        child: None,
    };
    std::fs::create_dir(&run.directory).expect("create test directory");
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixture.json");
    let connection = run.directory.join("connection.json");
    run.child = Some(
        Command::new(env!("CARGO_BIN_EXE_s2-linear-stub"))
            .arg(&fixture)
            .arg(&connection)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("start binary"),
    );
    let info = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if let Ok(bytes) = std::fs::read(&connection)
                && let Ok(value) = serde_json::from_slice::<Value>(&bytes)
            {
                break value;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("connection file ready");
    assert_eq!(info["interface_version"], 1);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(&connection)
                .expect("file metadata")
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
    let client = Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(3))
        .build()
        .expect("client");
    let response = client
        .post(info["graphql_url"].as_str().expect("URL"))
        .header(
            "Authorization",
            info["api_key"].as_str().expect("synthetic token present"),
        )
        .json(&json!({"query": "{ issues { nodes { identifier } } }"}))
        .send()
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<Value>().await.expect("JSON")["data"]["issues"]["nodes"],
        json!([{"identifier": "SYN-1"}, {"identifier": "SYN-2"}])
    );
    let response = client
        .get(format!(
            "{}/count",
            info["control_url"].as_str().expect("URL")
        ))
        .header(
            "Authorization",
            info["control_token"]
                .as_str()
                .expect("synthetic token present"),
        )
        .send()
        .await
        .expect("response");
    assert_eq!(
        response.json::<Value>().await.expect("JSON")["api_requests"],
        1
    );
    let before = std::fs::read(&connection).expect("private connection bytes");
    let status = Command::new(env!("CARGO_BIN_EXE_s2-linear-stub"))
        .arg(&fixture)
        .arg(&connection)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("second binary status");
    assert!(!status.success());
    assert!(std::fs::read(&connection).expect("private connection bytes") == before);
}
