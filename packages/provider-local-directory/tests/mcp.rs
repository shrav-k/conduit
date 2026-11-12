#![cfg(unix)]
use base64::{Engine, engine::general_purpose::STANDARD};
use provider_local_directory::{CONFIG, EXTENSION, MAX_ARCHIVE_BYTES, OUTCOME, schemas};
use rmcp::{
    RoleClient, ServiceExt,
    model::*,
    service::{RunningService, ServiceError},
    transport::TokioChildProcess,
};
use serde_json::{Value, json};
use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
    path::Path,
    process::Stdio,
    time::{Duration, SystemTime},
};
use url::Url;

async fn start() -> RunningService<RoleClient, ()> {
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_provider-local-directory"));
    command.env_clear().stderr(Stdio::null());
    tokio::time::timeout(
        Duration::from_secs(15),
        ().serve(TokioChildProcess::new(command).unwrap()),
    )
    .await
    .unwrap()
    .unwrap()
}
fn config(root: &Path) -> Value {
    json!({"schemes":["file"],"root":root})
}
fn args(fetch: &str) -> Value {
    json!({"address":"example.org/test/package","version":"0.1.0","digest":format!("sha256:{}", "0".repeat(64)),"fetch":fetch})
}
fn call(configuration: Value, arguments: Value) -> CallToolRequestParams {
    serde_json::from_value(json!({"name":"fetch-package.v1","arguments":arguments,"_meta":{
        "io.github.shrav-k.conduit/instance":"provider-test",
        CONFIG:configuration,
        "io.github.shrav-k.conduit/revision":"r1",
        "io.github.shrav-k.conduit/caller":"root",
        "io.github.shrav-k.conduit/operation":"github.com/shrav-k/conduit/core#fetch-package.v1",
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}}
    }})).unwrap()
}
fn file_url(path: &Path) -> String {
    Url::from_file_path(path).unwrap().to_string()
}
async fn wire(
    client: &RunningService<RoleClient, ()>,
    configuration: Value,
    arguments: Value,
) -> Value {
    let result = tokio::time::timeout(
        Duration::from_secs(15),
        client.call_tool(call(configuration, arguments)),
    )
    .await
    .expect("provider must not hang")
    .unwrap();
    serde_json::to_value(result).unwrap()
}
async fn failed(
    client: &RunningService<RoleClient, ()>,
    configuration: Value,
    fetch: &str,
    reason: &str,
) {
    let result = wire(client, configuration, args(fetch)).await;
    assert_eq!(result["_meta"][OUTCOME], "failed");
    assert_eq!(result["isError"], true);
    assert!(result.get("structuredContent").is_none());
    let message = result["content"][0]["text"].as_str().unwrap();
    assert!(
        message.contains(reason),
        "expected reason {reason}, got {message}"
    );
    assert!(!message.contains("DO_NOT_ECHO_FILE_BYTES"));
    assert!(!message.contains("private-password"));
}
async fn succeeds(
    client: &RunningService<RoleClient, ()>,
    configuration: Value,
    fetch: &str,
    expected: &[u8],
) {
    let result = wire(client, configuration, args(fetch)).await;
    assert_eq!(result["_meta"][OUTCOME], "succeeded", "{result}");
    assert_eq!(result["isError"], false);
    assert!(schemas::valid(
        "fetch-package.v1",
        "outputSchema",
        &result["structuredContent"]
    ));
    let archive = result["structuredContent"]["archive"].as_str().unwrap();
    assert_eq!(archive, STANDARD.encode(expected));
    assert_eq!(STANDARD.decode(archive).unwrap(), expected);
}
fn fifo(path: &Path) {
    // mknodat is not in rustix on every Unix (macOS lacks it); mkfifo(1) is.
    let status = std::process::Command::new("mkfifo")
        .arg("-m")
        .arg("600")
        .arg(path)
        .status()
        .unwrap();
    assert!(status.success());
}

#[tokio::test]
async fn discovery_and_capability_refusal_before_any_file_read() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("blocking-fifo");
    fifo(&path);
    let regular = directory.path().join("archive");
    fs::write(&regular, b"bytes").unwrap();
    let old_access = SystemTime::UNIX_EPOCH + Duration::from_secs(10);
    fs::File::open(&regular)
        .unwrap()
        .set_times(fs::FileTimes::new().set_accessed(old_access))
        .unwrap();
    let client = start().await;
    let discovery = client.discover(serde_json::from_value(json!({"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}})).unwrap()).await.unwrap();
    let value = serde_json::to_value(discovery).unwrap();
    assert_eq!(
        value["capabilities"]["extensions"][EXTENSION]["revision"],
        "v0"
    );
    let tools = client.list_all_tools().await.unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "fetch-package.v1");
    for kind in ["inputSchema", "outputSchema"] {
        assert_eq!(
            serde_json::to_value(&tools[0]).unwrap()[kind],
            schemas::schema("fetch-package.v1", kind).unwrap()
        );
    }
    // With no writer a read of this FIFO would block. -32021 must take priority
    // over all configuration, schema, path and file work.
    for revision in [None, Some("v1")] {
        for target in [&path, &regular] {
            let mut request =
                serde_json::to_value(call(config(directory.path()), args(&file_url(target))))
                    .unwrap();
            request["_meta"]["io.modelcontextprotocol/clientCapabilities"] = match revision {
                None => json!({}),
                Some(revision) => json!({"extensions":{EXTENSION:{"revision":revision}}}),
            };
            let error = tokio::time::timeout(
                Duration::from_secs(5),
                client.call_tool(serde_json::from_value(request).unwrap()),
            )
            .await
            .unwrap()
            .unwrap_err();
            let ServiceError::McpError(error) = error else {
                panic!("expected capability error")
            };
            assert_eq!(error.code.0, -32021);
            assert_eq!(
                error.data.unwrap()["requiredCapabilities"]["extensions"][EXTENSION]["revision"],
                "v0"
            );
            assert_eq!(
                fs::metadata(&regular).unwrap().accessed().unwrap(),
                old_access
            );
        }
    }
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn exact_bytes_corrupt_bytes_file_urls_symlinks_and_no_cache() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("root");
    fs::create_dir(&root).unwrap();
    let path = root.join("an archive %.tar.gz");
    let bytes: Vec<u8> = (0..=255).collect();
    fs::write(&path, &bytes).unwrap();
    let client = start().await;
    succeeds(&client, config(&root), &file_url(&path), &bytes).await;
    let localhost = file_url(&path).replacen("file://", "FiLe://localhost", 1);
    succeeds(&client, config(&root), &localhost, &bytes).await;
    let link = root.join("internal-link");
    symlink(&path, &link).unwrap();
    let root_link = directory.path().join("root-link");
    symlink(&root, &root_link).unwrap();
    succeeds(&client, config(&root_link), &file_url(&link), &bytes).await;
    let corrupt = b"DO_NOT_ECHO_FILE_BYTES\0\xffthis is not a tar or gzip archive";
    fs::write(&path, corrupt).unwrap();
    succeeds(&client, config(&root), &file_url(&path), corrupt).await;
    fs::write(&path, []).unwrap();
    succeeds(&client, config(&root), &file_url(&path), &[]).await;
    fs::remove_file(&path).unwrap();
    failed(&client, config(&root), &file_url(&path), "missing").await;
    // Configuration is taken from each call, including a different root.
    let another = directory.path().join("another");
    fs::create_dir(&another).unwrap();
    let another_file = another.join("archive");
    fs::write(&another_file, b"another").unwrap();
    succeeds(
        &client,
        config(&another),
        &file_url(&another_file),
        b"another",
    )
    .await;
    failed(
        &client,
        config(&root),
        &file_url(&another_file),
        "outside root",
    )
    .await;
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn all_containment_escapes_fail() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("r");
    let sibling = directory.path().join("r2");
    fs::create_dir(&root).unwrap();
    fs::create_dir(&sibling).unwrap();
    let outside = directory.path().join("outside");
    fs::write(&outside, b"DO_NOT_ECHO_FILE_BYTES").unwrap();
    let sibling_file = sibling.join("archive");
    fs::write(&sibling_file, b"DO_NOT_ECHO_FILE_BYTES").unwrap();
    let link = root.join("escape");
    symlink(&outside, &link).unwrap();
    let dir_link = root.join("escape-directory");
    symlink(&sibling, &dir_link).unwrap();
    let root_url = Url::from_directory_path(&root).unwrap().to_string();
    let client = start().await;
    for fetch in [
        file_url(&outside),
        format!("{root_url}../outside"),
        format!("{root_url}%2e%2e/outside"),
        format!("{root_url}%2e%2e%2foutside"),
        file_url(&link),
        file_url(&dir_link.join("archive")),
        file_url(&sibling_file),
    ] {
        failed(&client, config(&root), &fetch, "outside root").await;
    }
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn invalid_urls_missing_and_nonregular_files_fail_explicitly() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let client = start().await;
    for (fetch, reason) in [
        ("https://example.org/archive", "scheme is not claimed"),
        ("archive.tar.gz", "relative"),
        ("/archive.tar.gz", "relative"),
        (
            "file://another-host/archive",
            "host must be empty or localhost",
        ),
        (
            "file://user:private-password@localhost/archive",
            "user information",
        ),
        (
            "https://user:private-password@example.org/archive",
            "user information",
        ),
        ("file://user@localhost/archive", "user information"),
    ] {
        failed(&client, config(root), fetch, reason).await;
    }
    failed(
        &client,
        config(root),
        &file_url(&root.join("missing")),
        "missing",
    )
    .await;
    failed(&client, config(root), &file_url(root), "not a regular file").await;
    let subdir = root.join("directory");
    fs::create_dir(&subdir).unwrap();
    failed(
        &client,
        config(root),
        &file_url(&subdir),
        "not a regular file",
    )
    .await;
    let pipe = root.join("fifo");
    fifo(&pipe);
    failed(
        &client,
        config(root),
        &file_url(&pipe),
        "not a regular file",
    )
    .await;
    let socket = root.join("socket");
    let _socket = std::os::unix::net::UnixListener::bind(&socket).unwrap();
    failed(
        &client,
        config(root),
        &file_url(&socket),
        "cannot be opened",
    )
    .await;
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn size_bound_includes_limit_and_rejects_next_byte() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("bounded");
    let bytes = vec![0xff; MAX_ARCHIVE_BYTES as usize];
    fs::write(&path, &bytes).unwrap();
    let client = start().await;
    succeeds(&client, config(directory.path()), &file_url(&path), &bytes).await;
    fs::OpenOptions::new()
        .write(true)
        .open(&path)
        .unwrap()
        .set_len(MAX_ARCHIVE_BYTES + 1)
        .unwrap();
    failed(&client, config(directory.path()), &file_url(&path), "8 MiB").await;
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn readable_file_through_search_only_directories() {
    if rustix::process::getuid().is_root() {
        eprintln!("SKIPPED search-only-directory test: running as root bypasses Unix permissions");
        return;
    }
    let directory = tempfile::tempdir().unwrap();
    let ancestor = directory.path().join("ancestor");
    let root = ancestor.join("root");
    let nested = root.join("nested");
    fs::create_dir_all(&nested).unwrap();
    let path = nested.join("archive");
    let bytes = b"readable archive through directories without listing permission";
    fs::write(&path, bytes).unwrap();

    // Restore permissions before TempDir cleanup, even if an assertion fails.
    struct RestorePermissions<'a>(&'a [&'a Path]);
    impl Drop for RestorePermissions<'_> {
        fn drop(&mut self) {
            for path in self.0 {
                fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
            }
        }
    }
    let directories = [ancestor.as_path(), root.as_path(), nested.as_path()];
    let _restore = RestorePermissions(&directories);
    for directory in directories {
        fs::set_permissions(directory, fs::Permissions::from_mode(0o100)).unwrap();
        assert_eq!(
            fs::read_dir(directory).unwrap_err().kind(),
            std::io::ErrorKind::PermissionDenied
        );
    }
    assert_eq!(fs::read(&path).unwrap(), bytes);
    let client = start().await;
    succeeds(&client, config(&root), &file_url(&path), bytes).await;
    // Traversal permission is still necessary at each level.
    for directory in directories {
        fs::set_permissions(directory, fs::Permissions::from_mode(0o0)).unwrap();
        failed(&client, config(&root), &file_url(&path), "unreadable").await;
        fs::set_permissions(directory, fs::Permissions::from_mode(0o100)).unwrap();
    }
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn unreadable_file_fails_unless_running_as_root() {
    if rustix::process::getuid().is_root() {
        eprintln!("SKIPPED unreadable-file test: running as root bypasses Unix file permissions");
        return;
    }
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("unreadable");
    fs::write(&path, b"DO_NOT_ECHO_FILE_BYTES").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o0)).unwrap();
    let client = start().await;
    failed(
        &client,
        config(directory.path()),
        &file_url(&path),
        "unreadable",
    )
    .await;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn invalid_configuration_and_arguments_fail() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("archive");
    fs::write(&path, b"bytes").unwrap();
    let client = start().await;
    for configuration in [
        json!({}),
        json!({"root":directory.path()}),
        json!({"schemes":[],"root":directory.path()}),
        json!({"schemes":["https"],"root":directory.path()}),
        json!({"schemes":["file"],"root":"relative"}),
        json!({"schemes":["file"],"root":directory.path(),"extra":true}),
    ] {
        failed(
            &client,
            configuration,
            &file_url(&path),
            "configuration requires",
        )
        .await;
    }
    failed(
        &client,
        config(&path),
        &file_url(&path),
        "root must be an accessible directory",
    )
    .await;
    failed(
        &client,
        config(&directory.path().join("missing-root")),
        &file_url(&path),
        "root directory cannot be resolved",
    )
    .await;
    for arguments in [
        json!({}),
        json!({"fetch":file_url(&path)}),
        json!({"address":"bad", "version":"bad", "digest":"bad", "fetch":file_url(&path)}),
    ] {
        let result = wire(&client, config(directory.path()), arguments).await;
        assert_eq!(result["_meta"][OUTCOME], "failed");
        assert_eq!(result["isError"], true);
        assert!(
            result["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains("input schema")
        );
    }
    client.cancel().await.unwrap();
}
