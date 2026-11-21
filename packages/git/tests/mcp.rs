#![cfg(unix)]

mod support;
use conduit_git::{ADDRESS, CORE, EXTENSION, MAX_BYTES, OUTCOME, PREFIX, REASON, RESTRICTION};
use rmcp::{
    RoleClient, ServiceExt,
    model::CallToolRequestParams,
    service::{RunningService, ServiceError},
    transport::TokioChildProcess,
};
use serde_json::{Value, json};
use std::{
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt, symlink},
    path::{Path, PathBuf},
    time::Duration,
};
use support::Scratch;

struct Client(RunningService<RoleClient, ()>);

impl Client {
    async fn open(state: &Path) -> Self {
        let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_conduit-git"));
        command
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap())
            .env("CONDUIT_STATE_DIR", state);
        let (transport, _) = TokioChildProcess::builder(command).spawn().unwrap();
        Self(
            tokio::time::timeout(Duration::from_secs(15), ().serve(transport))
                .await
                .unwrap()
                .unwrap(),
        )
    }

    async fn call(&self, config: &Value, name: &str, args: Value) -> Value {
        self.raw(request(config, name, args)).await.unwrap()
    }

    async fn raw(&self, wire: Value) -> Result<Value, ServiceError> {
        let request: CallToolRequestParams = serde_json::from_value(wire).unwrap();
        let reply = tokio::time::timeout(Duration::from_secs(30), self.0.call_tool(request))
            .await
            .expect("working-copy call timed out")?;
        Ok(serde_json::to_value(reply).unwrap())
    }
}

fn request(config: &Value, name: &str, args: Value) -> Value {
    let hook = matches!(name, "setup-hook.v1" | "retire-hook.v1");
    let package = if hook { CORE } else { ADDRESS };
    let mut req = json!({"name":name,"arguments":args,"_meta":{
        "io.modelcontextprotocol/clientCapabilities":{"extensions":{EXTENSION:{"revision":"v0"}}},
        format!("{PREFIX}instance"):"copy-instance", format!("{PREFIX}operation"):format!("{package}#{name}"),
        format!("{PREFIX}invocation"):"copy-invocation", format!("{PREFIX}caller"):"root",
        format!("{PREFIX}filters"):[], format!("{PREFIX}config"):config, format!("{PREFIX}revision"):"r1"
    }});
    let (key, value) = if hook {
        ("idempotencyKey", name)
    } else {
        ("conduit", "copy-write")
    };
    req["_meta"][format!("{PREFIX}{key}")] = json!(value);
    req
}

struct Fixture {
    scratch: Scratch,
    repository: PathBuf,
    state: PathBuf,
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let scratch = Scratch::new();
        let repository = scratch.repository();
        let state = scratch.path.join("state");
        let root = scratch.path.join("copy");
        fs::create_dir(&state).unwrap();
        Self {
            scratch,
            repository,
            state,
            root,
        }
    }

    fn base(&self) -> String {
        for (path, contents) in [
            ("tracked.txt", "original\n"),
            ("delete.txt", "remove\n"),
            ("mode.sh", "#!/bin/sh\n"),
            ("opaque.txt", "opaque original\n"),
            (".gitignore", "ignored/\n"),
        ] {
            fs::write(self.repository.join(path), contents).unwrap();
        }
        fs::set_permissions(
            self.repository.join("mode.sh"),
            fs::Permissions::from_mode(0o755),
        )
        .unwrap();
        self.scratch.commit(&self.repository)
    }

    fn config(&self, base: &str) -> Value {
        json!({"repository":self.repository,"commit":base,"root":self.root})
    }

    async fn setup(&self) -> (Client, String, Value) {
        let base = self.base();
        let config = self.config(&base);
        let client = Client::open(&self.state).await;
        succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
        (client, base, config)
    }
}

fn succeeded(reply: &Value) -> &Value {
    assert_eq!(reply["_meta"][OUTCOME], "succeeded", "{reply}");
    assert_eq!(reply["isError"], false);
    &reply["structuredContent"]
}

fn failed(reply: &Value, cause: &str) {
    assert_eq!(reply["_meta"][OUTCOME], "failed", "{reply}");
    assert_eq!(reply["isError"], true);
    assert!(reply.get("structuredContent").is_none());
    assert!(
        reply["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains(cause),
        "{reply}"
    );
}

#[tokio::test]
async fn setup_makes_the_copy_at_the_commit() {
    let f = Fixture::new();
    let base = f.base();
    fs::write(f.repository.join("tracked.txt"), "later\n").unwrap();
    assert_ne!(f.scratch.commit(&f.repository), base);
    let client = Client::open(&f.state).await;
    let config = f.config(&base);

    let occupied = f.scratch.path.join("occupied");
    fs::create_dir(&occupied).unwrap();
    fs::write(occupied.join("keep"), b"kept").unwrap();
    let empty_source = f.scratch.path.join("non-repository");
    fs::create_dir(&empty_source).unwrap();
    let link_parent = f.scratch.path.join("linked");
    symlink(&occupied, &link_parent).unwrap();
    for (field, value, cause) in [
        ("root", json!(occupied), "working-copy root is not empty"),
        (
            "commit",
            json!(format!("{:040}", 0)),
            "commit is not in the repository",
        ),
        (
            "commit",
            json!("b".repeat(39)),
            "configuration commit must be 40 lowercase hexadecimal digits",
        ),
        (
            "repository",
            json!(empty_source),
            "repository is unreadable",
        ),
        (
            "root",
            json!("relative"),
            "configuration root must be an absolute path without NUL bytes",
        ),
        (
            "root",
            json!(link_parent.join("missing")),
            "working-copy root parent is missing, not a directory, or contains a symbolic link",
        ),
    ] {
        let mut invalid = config.clone();
        invalid[field] = value;
        failed(
            &client.call(&invalid, "setup-hook.v1", json!({})).await,
            cause,
        );
        assert!(!f.root.exists());
        assert!(!f.state.join("repository.git").exists());
        assert_eq!(fs::read(occupied.join("keep")).unwrap(), b"kept");
        assert!(!occupied.join("missing").exists());
    }

    fs::create_dir(&f.root).unwrap();
    succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
    assert_eq!(fs::read(f.root.join("tracked.txt")).unwrap(), b"original\n");
    assert_ne!(
        fs::metadata(f.root.join("mode.sh")).unwrap().mode() & 0o111,
        0
    );
    assert!(!f.root.join(".git").exists());
    assert_eq!(
        fs::read_to_string(f.state.join("repository.git/HEAD"))
            .unwrap()
            .trim(),
        base
    );
    let inode = fs::metadata(&f.root).unwrap().ino();
    succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
    assert_eq!(fs::metadata(&f.root).unwrap().ino(), inode);
    let changes = client.call(&config, "changes.v1", json!({})).await;
    assert_eq!(
        *succeeded(&changes),
        json!({"base":base,"files":[],"patch":""})
    );
}

#[tokio::test]
async fn reads_lists_and_writes_stay_inside_root() {
    let f = Fixture::new();
    let (client, _, config) = f.setup().await;
    let outside = f.scratch.path.join("outside");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("keep"), b"outside").unwrap();
    symlink(&outside, f.root.join("linked")).unwrap();
    symlink(outside.join("keep"), f.root.join("linked-file")).unwrap();
    let write = |path: &str, content: &str| json!({"path":path,"content":content});
    succeeded(
        &client
            .call(&config, "file.write.v1", write("z/added", "written"))
            .await,
    );
    let read = client
        .call(&config, "file.read.v1", json!({"path":"z/added"}))
        .await;
    assert_eq!(succeeded(&read), &json!({"content":"written"}));
    let list = client
        .call(&config, "directory.list.v1", json!({"path":"."}))
        .await;
    let entries = succeeded(&list)["entries"].as_array().unwrap();
    let names: Vec<_> = entries
        .iter()
        .map(|item| item["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        names,
        [
            ".gitignore",
            "delete.txt",
            "linked",
            "linked-file",
            "mode.sh",
            "opaque.txt",
            "tracked.txt",
            "z"
        ]
    );
    assert_eq!(entries[2]["kind"], "symlink");
    assert_eq!(entries[7]["kind"], "directory");

    for path in [
        "..",
        "/absolute",
        ".git",
        "z/.GIT/test",
        "z//bad",
        "linked/keep",
        "linked-file",
        "z",
        ".",
    ] {
        failed(
            &client
                .call(&config, "file.write.v1", write(path, "overwritten"))
                .await,
            path,
        );
        failed(
            &client
                .call(&config, "file.read.v1", json!({"path":path}))
                .await,
            path,
        );
    }
    failed(
        &client
            .call(&config, "directory.list.v1", json!({"path":"linked"}))
            .await,
        "symbolic link",
    );
    fs::write(f.root.join("non-utf8"), [0xff]).unwrap();
    failed(
        &client
            .call(&config, "file.read.v1", json!({"path":"non-utf8"}))
            .await,
        "not valid UTF-8",
    );
    fs::write(f.root.join("too-large"), vec![b'x'; MAX_BYTES + 1]).unwrap();
    failed(
        &client
            .call(&config, "file.read.v1", json!({"path":"too-large"}))
            .await,
        "exceeds 8 MiB",
    );
    failed(
        &client
            .call(
                &config,
                "file.write.v1",
                write("z/added", &"x".repeat(MAX_BYTES + 1)),
            )
            .await,
        "exceeds 8 MiB",
    );
    let mut other = config.clone();
    other["root"] = json!(outside);
    failed(
        &client
            .call(&other, "file.write.v1", write("keep", "changed"))
            .await,
        "configured root is not the working copy this instance made",
    );
    assert_eq!(fs::read(outside.join("keep")).unwrap(), b"outside");
    assert_eq!(fs::read(f.root.join("z/added")).unwrap(), b"written");
}

#[tokio::test]
async fn changes_rebuild_the_copy_from_its_base() {
    let f = Fixture::new();
    let (client, base, config) = f.setup().await;
    fs::write(f.root.join("tracked.txt"), "edited\n").unwrap();
    fs::remove_file(f.root.join("delete.txt")).unwrap();
    fs::set_permissions(f.root.join("mode.sh"), fs::Permissions::from_mode(0o644)).unwrap();
    symlink("tracked.txt", f.root.join("new-link")).unwrap();
    fs::write(f.root.join("binary"), [0, 255, 17, 23]).unwrap();
    fs::write(f.root.join("opaque.txt"), "new opaque text\n").unwrap();
    fs::write(
        f.root.join(".gitattributes"),
        "* text eol=crlf\nopaque.txt -diff\n",
    )
    .unwrap();
    fs::write(f.root.join("crlf.txt"), "one\r\ntwo\r\n").unwrap();
    fs::write(f.root.join(":literal"), "literal path\n").unwrap();
    fs::create_dir(f.root.join("ignored")).unwrap();
    fs::write(f.root.join("ignored/extra"), "excluded").unwrap();
    fs::create_dir(f.root.join("ignored/.git")).unwrap();
    fs::write(f.root.join(".gitignore"), "ignored/\n*.drop\n").unwrap();
    fs::write(f.root.join("other.drop"), "excluded").unwrap();

    let reply = client.call(&config, "changes.v1", json!({})).await;
    let changes = succeeded(&reply);
    assert_eq!(changes["base"], base);
    assert_eq!(
        changes["files"],
        json!([
            {"path":".gitattributes","change":"added"}, {"path":".gitignore","change":"modified"},
            {"path":":literal","change":"added"},
            {"path":"binary","change":"added"}, {"path":"crlf.txt","change":"added"},
            {"path":"delete.txt","change":"deleted"}, {"path":"mode.sh","change":"modified"},
            {"path":"new-link","change":"added"}, {"path":"opaque.txt","change":"modified"},
            {"path":"tracked.txt","change":"modified"}
        ])
    );
    let patch = changes["patch"].as_str().unwrap();
    assert!(patch.contains("GIT binary patch"));
    let applied = f.scratch.clone_at(&f.repository, &base);
    f.scratch.git(&applied, &["apply", "--index"], Some(patch));
    for path in [
        ".gitattributes",
        ".gitignore",
        ":literal",
        "binary",
        "crlf.txt",
        "mode.sh",
        "opaque.txt",
        "tracked.txt",
    ] {
        assert_eq!(
            fs::read(applied.join(path)).unwrap(),
            fs::read(f.root.join(path)).unwrap(),
            "{path}"
        );
    }
    assert_eq!(
        fs::metadata(applied.join("mode.sh")).unwrap().mode() & 0o111,
        0
    );
    assert_eq!(
        fs::read_link(applied.join("new-link")).unwrap(),
        Path::new("tracked.txt")
    );
    assert!(!applied.join("delete.txt").exists());
    assert!(!applied.join("ignored").exists());
    assert!(!applied.join("other.drop").exists());
    assert!(!f.root.join(".git").exists());
}

#[tokio::test]
async fn planted_git_entries_are_never_read_or_run() {
    let f = Fixture::new();
    let (client, _, config) = f.setup().await;
    let marker = f.scratch.path.join("unexpected");
    let script = f.scratch.path.join("planted");
    fs::write(
        &script,
        format!("#!/bin/sh\ntouch '{}'\n", marker.display()),
    )
    .unwrap();
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    let dot_git = f.root.join(".git");
    let hooks = dot_git.join("hooks");
    fs::create_dir_all(&hooks).unwrap();
    fs::write(
        dot_git.join("config"),
        format!(
            "[core]\n  hooksPath = {}\n  fsmonitor = {}\n  pager = {}\n",
            hooks.display(),
            script.display(),
            script.display()
        ),
    )
    .unwrap();
    for hook in [
        "pre-commit",
        "post-checkout",
        "post-index-change",
        "fsmonitor-watchman",
    ] {
        fs::copy(&script, hooks.join(hook)).unwrap();
    }
    fs::create_dir_all(f.root.join("sub/.git")).unwrap();
    succeeded(
        &client
            .call(
                &config,
                "file.write.v1",
                json!({"path":"sub/entry","content":"added\n"}),
            )
            .await,
    );
    succeeded(
        &client
            .call(&config, "file.read.v1", json!({"path":"sub/entry"}))
            .await,
    );
    succeeded(
        &client
            .call(&config, "directory.list.v1", json!({"path":"."}))
            .await,
    );
    failed(
        &client.call(&config, "changes.v1", json!({})).await,
        "nested .git entry in \"sub\"",
    );
    fs::remove_dir(f.root.join("sub/.git")).unwrap();
    let reply = client.call(&config, "changes.v1", json!({})).await;
    let changes = succeeded(&reply);
    assert_eq!(
        changes["files"],
        json!([{"path":"sub/entry","change":"added"}])
    );
    assert!(!changes["patch"].as_str().unwrap().contains("/.git/"));
    succeeded(&client.call(&config, "retire-hook.v1", json!({})).await);
    assert!(!marker.exists());
}

#[tokio::test]
async fn retire_removes_the_copy_and_nothing_outside() {
    let f = Fixture::new();
    let (client, _, config) = f.setup().await;
    let outside = f.scratch.path.join("outside");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("remains"), b"outside").unwrap();
    symlink(&outside, f.root.join("outside-link")).unwrap();
    succeeded(&client.call(&config, "retire-hook.v1", json!({})).await);
    assert!(!f.root.exists());
    assert!(!f.state.join("repository.git").exists());
    assert_eq!(fs::read(outside.join("remains")).unwrap(), b"outside");
    succeeded(&client.call(&config, "retire-hook.v1", json!({})).await);
    succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
    assert!(f.root.join("tracked.txt").is_file());
}

#[tokio::test]
async fn discovery_and_extension_refusal_make_nothing() {
    let f = Fixture::new();
    let base = f.base();
    let client = Client::open(&f.state).await;
    let discovery = client
        .0
        .discover(
            serde_json::from_value(json!({
                "io.modelcontextprotocol/protocolVersion":"2026-07-28",
                "io.modelcontextprotocol/clientCapabilities":{}
            }))
            .unwrap(),
        )
        .await
        .unwrap();
    let discovery = serde_json::to_value(discovery).unwrap();
    assert_eq!(
        discovery["capabilities"]["extensions"][EXTENSION]["revision"],
        "v0"
    );
    let tools = client.0.list_all_tools().await.unwrap();
    assert_eq!(tools.len(), 6);
    for tool in tools {
        let (input, output) = conduit_git::schemas(&tool.name).unwrap();
        let wire = serde_json::to_value(tool).unwrap();
        assert_eq!(wire["inputSchema"], input);
        assert_eq!(wire.get("outputSchema"), output.as_ref());
    }
    for revision in [None, Some("v1")] {
        let mut req = request(&f.config(&base), "setup-hook.v1", json!({}));
        let key = "io.modelcontextprotocol/clientCapabilities";
        if let Some(revision) = revision {
            req["_meta"][key]["extensions"][EXTENSION]["revision"] = json!(revision);
        } else {
            req["_meta"].as_object_mut().unwrap().remove(key);
        }
        let error = client.raw(req).await.unwrap_err();
        let ServiceError::McpError(error) = error else {
            panic!("expected extension refusal")
        };
        let error = serde_json::to_value(error).unwrap();
        assert_eq!(error["code"], -32021);
        assert_eq!(
            error["data"]["requiredCapabilities"]["extensions"][EXTENSION]["revision"],
            "v0"
        );
        assert!(!f.root.exists());
        assert!(fs::read_dir(&f.state).unwrap().next().is_none());
    }
}

#[tokio::test]
async fn filters_are_refused_before_working_copy_access() {
    let f = Fixture::new();
    let base = f.base();
    let client = Client::open(&f.state).await;
    let mut req = request(&f.config(&base), "setup-hook.v1", json!({}));
    let restriction = json!({"requirement":"example.org/test#filter.v1","value":{"name":"one"}});
    req["_meta"][format!("{PREFIX}filters")] = json!([restriction]);
    let reply = client.raw(req).await.unwrap();
    assert_eq!(reply["_meta"][REASON], "unavailable");
    assert_eq!(reply["_meta"][RESTRICTION], restriction);
    assert!(!f.root.exists());
    assert!(fs::read_dir(f.state).unwrap().next().is_none());
}

#[tokio::test]
async fn interrupted_checkout_retries_without_removing_other_paths() {
    let f = Fixture::new();
    let base = f.base();
    let config = f.config(&base);
    let staging = f.scratch.path.join("interrupted-checkout");
    fs::create_dir(&staging).unwrap();
    fs::write(staging.join("partial"), b"checkout in progress").unwrap();
    let unrelated = f.scratch.path.join("keep");
    fs::write(&unrelated, b"keep").unwrap();
    fs::create_dir(f.state.join("repository.git")).unwrap();
    let metadata = fs::metadata(&staging).unwrap();
    fs::write(
        f.state.join("copy.json"),
        serde_json::to_vec(&json!({
            "root":f.root, "base":base, "staging":staging,
            "device":metadata.dev(), "inode":metadata.ino()
        }))
        .unwrap(),
    )
    .unwrap();
    let client = Client::open(&f.state).await;
    succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
    assert!(!staging.exists());
    assert_eq!(fs::read(unrelated).unwrap(), b"keep");
    assert_eq!(fs::read(f.root.join("tracked.txt")).unwrap(), b"original\n");
}

#[tokio::test]
async fn committed_eol_attributes_checkout_and_verify_exactly() {
    let f = Fixture::new();
    fs::write(
        f.repository.join(".gitattributes"),
        "file.txt text eol=crlf\n",
    )
    .unwrap();
    fs::write(f.repository.join("file.txt"), "base\n").unwrap();
    let base = f.scratch.commit(&f.repository);
    let config = f.config(&base);
    let applied = f.scratch.clone_at(&f.repository, &base);
    let client = Client::open(&f.state).await;
    succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
    assert_eq!(fs::read(applied.join("file.txt")).unwrap(), b"base\r\n");
    assert_eq!(fs::read(f.root.join("file.txt")).unwrap(), b"base\r\n");
    let clean = client.call(&config, "changes.v1", json!({})).await;
    assert_eq!(
        succeeded(&clean),
        &json!({"base":base,"files":[],"patch":""})
    );

    fs::write(f.root.join("file.txt"), "edited\n").unwrap();
    let impossible = client.call(&config, "changes.v1", json!({})).await;
    failed(
        &impossible,
        "cannot reproduce working-copy path \"file.txt\" exactly",
    );

    fs::write(f.root.join("file.txt"), "edited\r\n").unwrap();
    let changes = client.call(&config, "changes.v1", json!({})).await;
    let changes = succeeded(&changes);
    assert_eq!(
        changes["files"],
        json!([{"path":"file.txt","change":"modified"}])
    );
    f.scratch
        .git(&applied, &["apply", "--index"], changes["patch"].as_str());
    assert_eq!(fs::read(applied.join("file.txt")).unwrap(), b"edited\r\n");
}

#[tokio::test]
async fn setup_ignores_source_upload_pack_settings() {
    let f = Fixture::new();
    let base = f.base();
    f.scratch.git(&f.repository, &["repack", "-ad"], None);
    f.scratch.git(&f.repository, &["prune-packed"], None);
    f.scratch.git(
        &f.repository,
        &["config", "uploadpack.allowFilter", "invalid-boolean"],
        None,
    );
    let client = Client::open(&f.state).await;
    succeeded(
        &client
            .call(&f.config(&base), "setup-hook.v1", json!({}))
            .await,
    );
    assert_eq!(fs::read(f.root.join("tracked.txt")).unwrap(), b"original\n");
}

#[tokio::test]
async fn setup_imports_alternate_objects_from_a_linked_worktree() {
    let f = Fixture::new();
    let base = f.base();
    let shared = f.scratch.path.join("shared-with-alternates");
    f.scratch.git(
        &f.scratch.path,
        &[
            "clone",
            "--shared",
            "--quiet",
            "--",
            f.repository.to_str().unwrap(),
            shared.to_str().unwrap(),
        ],
        None,
    );
    let linked = f.scratch.path.join("linked");
    f.scratch.git(
        &shared,
        &[
            "worktree",
            "add",
            "--detach",
            "--quiet",
            linked.to_str().unwrap(),
            &base,
        ],
        None,
    );
    let mut config = f.config(&base);
    config["repository"] = json!(linked);
    let client = Client::open(&f.state).await;
    succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
    assert_eq!(fs::read(f.root.join("tracked.txt")).unwrap(), b"original\n");
    assert!(
        !f.state
            .join("repository.git/objects/info/alternates")
            .exists()
    );
}
