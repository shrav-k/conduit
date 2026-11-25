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
    collections::BTreeMap,
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt, symlink},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
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

fn view_command(f: &Fixture) -> Command {
    assert!(f.root.canonicalize().unwrap().starts_with(&f.scratch.path));
    let mut command = Command::new("git");
    command
        .current_dir(&f.root)
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap())
        .env("HOME", f.scratch.path.join("home"))
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CEILING_DIRECTORIES", &f.scratch.path)
        .env("GIT_AUTHOR_NAME", "Conduit test")
        .env("GIT_AUTHOR_EMAIL", "test@conduit.invalid")
        .env("GIT_COMMITTER_NAME", "Conduit test")
        .env("GIT_COMMITTER_EMAIL", "test@conduit.invalid")
        .stdin(Stdio::null());
    command
}

fn view_git(f: &Fixture, args: &[&str]) -> String {
    let output = view_command(f).args(args).output().unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn file_bytes(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, dir: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                visit(root, &entry.path(), files);
            } else {
                assert!(entry.file_type().unwrap().is_file());
                files.insert(
                    entry.path().strip_prefix(root).unwrap().to_owned(),
                    fs::read(entry.path()).unwrap(),
                );
            }
        }
    }
    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

#[tokio::test]
async fn git_view_is_written_at_setup() {
    let f = Fixture::new();
    let first = f.base();
    fs::write(f.repository.join("tracked.txt"), "second\n").unwrap();
    let base = f.scratch.commit(&f.repository);
    let config = f.config(&base);
    let client = Client::open(&f.state).await;
    succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
    let view = f.root.join(".git");
    let files = file_bytes(&view);
    assert_eq!(
        files
            .keys()
            .map(|p| p.to_str().unwrap())
            .collect::<Vec<_>>(),
        [
            "HEAD",
            "config",
            "index",
            "info/attributes",
            "objects/info/alternates"
        ]
    );
    assert_eq!(files[Path::new("HEAD")], format!("{base}\n").as_bytes());
    assert_eq!(
        files[Path::new("objects/info/alternates")],
        format!("{}\n", f.state.join("repository.git/objects").display()).as_bytes()
    );
    assert_eq!(
        files[Path::new("index")],
        fs::read(f.state.join("repository.git/index")).unwrap()
    );
    assert_eq!(files[Path::new("info/attributes")], b"* !filter !diff\n");
    for directory in ["refs/heads", "refs/tags", "objects/pack"] {
        assert_eq!(fs::read_dir(view.join(directory)).unwrap().count(), 0);
    }
    assert_eq!(
        view_git(&f, &["rev-parse", "--show-toplevel"]).trim(),
        f.root.to_str().unwrap()
    );
    assert_eq!(view_git(&f, &["rev-parse", "HEAD"]), format!("{base}\n"));
    assert_eq!(view_git(&f, &["status", "--porcelain=v1", "-uall"]), "");
    assert_eq!(
        view_git(&f, &["log", "--format=%H"]),
        format!("{base}\n{first}\n")
    );
    view_git(&f, &["merge-base", "--is-ancestor", &first, "HEAD"]);
    assert_eq!(view_git(&f, &["remote"]), "");
    assert_eq!(view_git(&f, &["for-each-ref"]), "");
    assert_eq!(view_git(&f, &["show", "HEAD:tracked.txt"]), "second\n");
    assert_eq!(view_git(&f, &["cat-file", "-t", "HEAD"]), "commit\n");
    assert!(
        view_git(&f, &["ls-files"])
            .lines()
            .any(|p| p == "tracked.txt")
    );
    let before_repeat = file_bytes(&view);
    succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
    assert_eq!(file_bytes(&view), before_repeat);
    fs::remove_dir_all(&view).unwrap();
    succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
    assert!(!view.exists());
}

#[tokio::test]
async fn git_view_status_lists_what_changes_returns() {
    let f = Fixture::new();
    let (client, _, config) = f.setup().await;
    fs::write(f.root.join("tracked.txt"), "edited \n").unwrap();
    fs::remove_file(f.root.join("delete.txt")).unwrap();
    fs::set_permissions(f.root.join("mode.sh"), fs::Permissions::from_mode(0o644)).unwrap();
    fs::write(f.root.join("new.txt"), "new\n").unwrap();
    fs::create_dir(f.root.join("ignored")).unwrap();
    fs::write(f.root.join("ignored/extra"), "excluded\n").unwrap();
    let reply = client.call(&config, "changes.v1", json!({})).await;
    let changes = succeeded(&reply);
    let expected: BTreeMap<_, _> = changes["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| {
            let code = match file["change"].as_str().unwrap() {
                "added" => "??",
                "deleted" => " D",
                "modified" => " M",
                other => panic!("unexpected change {other}"),
            };
            (file["path"].as_str().unwrap().to_owned(), code.to_owned())
        })
        .collect();
    assert_eq!(expected.len(), 4);
    assert!(!expected.contains_key("ignored/extra"));
    let args = ["status", "--porcelain=v1", "-z", "-uall", "--no-renames"];
    let before = view_git(&f, &args);
    let listed: BTreeMap<_, _> = before
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(|s| (s[3..].to_owned(), s[..2].to_owned()))
        .collect();
    assert_eq!(listed, expected);
    let check = view_command(&f).args(["diff", "--check"]).output().unwrap();
    assert!(!check.status.success());
    assert!(String::from_utf8_lossy(&check.stdout).contains("tracked.txt"));
    view_git(&f, &["add", "-N", "new.txt"]);
    assert_eq!(
        view_git(&f, &["diff", "--name-only", "HEAD"]),
        "delete.txt\nmode.sh\nnew.txt\ntracked.txt\n"
    );
    view_git(&f, &["reset", "-q", "--", "new.txt"]);
    assert_eq!(view_git(&f, &args), before);
    let after = client.call(&config, "changes.v1", json!({})).await;
    assert_eq!(succeeded(&after), changes);
}

#[tokio::test]
async fn git_view_refuses_commit_and_transport() {
    let f = Fixture::new();
    let (_client, base, _) = f.setup().await;
    let global = f.scratch.path.join("global-config");
    fs::write(
        &global,
        "[user]\n name = Conduit test\n email = test@conduit.invalid\n",
    )
    .unwrap();
    let agent = |args: &[&str]| -> Output {
        view_command(&f)
            .env("GIT_CONFIG_GLOBAL", &global)
            .env_remove("GIT_AUTHOR_NAME")
            .env_remove("GIT_AUTHOR_EMAIL")
            .env_remove("GIT_COMMITTER_NAME")
            .env_remove("GIT_COMMITTER_EMAIL")
            .args(args)
            .output()
            .unwrap()
    };
    fs::write(f.root.join("tracked.txt"), "edited\n").unwrap();
    let commit = agent(&["commit", "-am", "refused"]);
    assert!(!commit.status.success());
    assert!(String::from_utf8_lossy(&commit.stderr).contains("empty ident name"));
    assert_eq!(view_git(&f, &["rev-parse", "HEAD"]), format!("{base}\n"));
    let bare = f.scratch.path.join("bare.git");
    f.scratch.git(
        &f.scratch.path,
        &[
            "init",
            "--bare",
            "--quiet",
            "--template=",
            bare.to_str().unwrap(),
        ],
        None,
    );
    for args in [
        vec!["push", bare.to_str().unwrap(), "HEAD:refs/heads/x"],
        vec!["fetch", f.repository.to_str().unwrap()],
    ] {
        let output = agent(&args);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("transport 'file' not allowed"));
    }
    assert_eq!(f.scratch.git(&bare, &["for-each-ref"], None), "");
}

#[tokio::test]
async fn git_view_writes_reach_no_result() {
    let f = Fixture::new();
    let (client, base, config) = f.setup().await;
    let private = file_bytes(&f.state.join("repository.git"));
    fs::write(f.root.join("tracked.txt"), "edited\n").unwrap();
    view_git(&f, &["commit", "-am", "view only"]);
    view_git(&f, &["branch", "view-branch"]);
    view_git(&f, &["tag", "view-tag"]);
    let marker = f.scratch.path.join("unexpected");
    let script = f.scratch.path.join("monitor");
    fs::write(
        &script,
        format!("#!/bin/sh\ntouch '{}'\n", marker.display()),
    )
    .unwrap();
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    view_git(&f, &["config", "core.fsmonitor", script.to_str().unwrap()]);
    let reply = client.call(&config, "changes.v1", json!({})).await;
    let changes = succeeded(&reply);
    assert_eq!(changes["base"], base);
    assert_eq!(
        changes["files"],
        json!([{"path":"tracked.txt","change":"modified"}])
    );
    let applied = f.scratch.clone_at(&f.repository, &base);
    f.scratch
        .git(&applied, &["apply", "--index"], changes["patch"].as_str());
    for path in [
        "tracked.txt",
        "delete.txt",
        "mode.sh",
        "opaque.txt",
        ".gitignore",
    ] {
        assert_eq!(
            fs::read(applied.join(path)).unwrap(),
            fs::read(f.root.join(path)).unwrap()
        );
    }
    assert_eq!(file_bytes(&f.state.join("repository.git")), private);
    succeeded(&client.call(&config, "retire-hook.v1", json!({})).await);
    assert!(!f.root.exists());
    assert!(!marker.exists());
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
    assert_eq!(
        fs::read_to_string(f.root.join(".git/HEAD")).unwrap(),
        format!("{base}\n")
    );
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

    let view_before = file_bytes(&f.root.join(".git"));
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
    assert_eq!(file_bytes(&f.root.join(".git")), view_before);
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
    assert_eq!(tools.len(), 7);
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

fn move_args(commit: &str, read: &Value) -> Value {
    use sha2::{Digest, Sha256};
    json!({"commit":commit,"expectedBase":read["base"],
        "expectedPatchSha256":format!("{:x}", Sha256::digest(read["patch"].as_str().unwrap().as_bytes()))})
}

async fn read_copy(client: &Client, config: &Value) -> Value {
    succeeded(&client.call(config, "changes.v1", json!({})).await).clone()
}

fn tree_state(root: &Path) -> BTreeMap<PathBuf, (u32, Vec<u8>)> {
    use std::os::unix::ffi::OsStrExt;
    fn visit(root: &Path, path: &Path, out: &mut BTreeMap<PathBuf, (u32, Vec<u8>)>) {
        let meta = fs::symlink_metadata(path).unwrap();
        let bytes = if meta.is_symlink() {
            fs::read_link(path).unwrap().as_os_str().as_bytes().to_vec()
        } else if meta.is_file() {
            fs::read(path).unwrap()
        } else {
            Vec::new()
        };
        out.insert(
            path.strip_prefix(root).unwrap().to_owned(),
            (meta.mode(), bytes),
        );
        if meta.is_dir() {
            for entry in fs::read_dir(path).unwrap() {
                visit(root, &entry.unwrap().path(), out);
            }
        }
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out);
    out
}

fn no_move_staging(f: &Fixture) {
    assert!(!f.state.join("repository.git/move-staging").exists());
    assert!(fs::read_dir(&f.scratch.path).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".conduit-")
    }));
}

#[tokio::test]
async fn move_drops_the_change_and_keeps_ignored_files() {
    let f = Fixture::new();
    let (client, _, config) = f.setup().await;
    fs::write(f.repository.join("tracked.txt"), "next\n").unwrap();
    fs::remove_file(f.repository.join("delete.txt")).unwrap();
    fs::write(f.repository.join("next.txt"), "next file\n").unwrap();
    fs::write(f.repository.join(".gitignore"), "/ignored/\n").unwrap();
    fs::create_dir_all(f.repository.join("lib/ignored")).unwrap();
    fs::write(f.repository.join("lib/ignored/file"), "new library\n").unwrap();
    let next = f.scratch.commit(&f.repository);
    fs::write(f.root.join("tracked.txt"), "worker\n").unwrap();
    fs::write(f.root.join("new.txt"), "new\n").unwrap();
    fs::write(f.root.join(".gitignore"), "ignored/\n*.tmp\n").unwrap();
    fs::write(f.root.join("scratch.tmp"), "discard\n").unwrap();
    let outside = f.scratch.path.join("outside");
    fs::write(&outside, "outside\n").unwrap();
    fs::remove_file(f.root.join("opaque.txt")).unwrap();
    symlink(&outside, f.root.join("opaque.txt")).unwrap();
    fs::create_dir_all(f.root.join("ignored/cache")).unwrap();
    fs::write(f.root.join("ignored/cache/tool"), "cached\n").unwrap();
    fs::set_permissions(
        f.root.join("ignored/cache/tool"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    fs::create_dir_all(f.root.join("lib/ignored")).unwrap();
    fs::write(f.root.join("lib/ignored/stale"), "stale\n").unwrap();
    view_git(&f, &["branch", "worker"]);
    let read = read_copy(&client, &config).await;
    let root_meta = fs::metadata(&f.root).unwrap();
    let kept = tree_state(&f.root.join("ignored"));
    let tool_inode = fs::metadata(f.root.join("ignored/cache/tool"))
        .unwrap()
        .ino();
    let inode = fs::metadata(f.root.join("ignored")).unwrap().ino();
    let reply = client
        .call(&config, "move.v1", move_args(&next, &read))
        .await;
    assert_eq!(succeeded(&reply), &json!({"base":next,"files":[]}));
    assert_eq!(fs::metadata(&f.root).unwrap().ino(), root_meta.ino());
    assert_eq!(fs::metadata(&f.root).unwrap().mode(), root_meta.mode());
    assert_eq!(fs::metadata(f.root.join("ignored")).unwrap().ino(), inode);
    assert_eq!(tree_state(&f.root.join("ignored")), kept);
    assert_eq!(
        fs::metadata(f.root.join("ignored/cache/tool"))
            .unwrap()
            .ino(),
        tool_inode
    );
    let mut names: Vec<_> = fs::read_dir(&f.root)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            ".git",
            ".gitignore",
            "ignored",
            "lib",
            "mode.sh",
            "next.txt",
            "opaque.txt",
            "tracked.txt"
        ]
    );
    assert_eq!(
        file_bytes(&f.root.join("lib")),
        BTreeMap::from([(PathBuf::from("ignored/file"), b"new library\n".to_vec())])
    );
    assert_eq!(fs::read(f.root.join("tracked.txt")).unwrap(), b"next\n");
    assert_eq!(fs::read(&outside).unwrap(), b"outside\n");
    assert_eq!(view_git(&f, &["rev-parse", "HEAD"]).trim(), next);
    assert_eq!(view_git(&f, &["status", "--porcelain=v1", "-uall"]), "");
    assert_eq!(view_git(&f, &["for-each-ref"]), "");
    assert_eq!(
        read_copy(&client, &config).await,
        json!({"base":next,"files":[],"patch":""})
    );
    no_move_staging(&f);
}

#[tokio::test]
async fn move_applies_a_patch_uncommitted() {
    let first = Fixture::new();
    let (client, base, config) = first.setup().await;
    fs::write(first.root.join("tracked.txt"), "worker\n").unwrap();
    fs::remove_file(first.root.join("delete.txt")).unwrap();
    fs::write(first.root.join("new.txt"), "added\n").unwrap();
    fs::create_dir(first.root.join("ignored")).unwrap();
    fs::write(first.root.join("ignored/extra"), "cache\n").unwrap();
    let patch = read_copy(&client, &config).await;
    let state = first.scratch.path.join("second-state");
    let root = first.scratch.path.join("second-copy");
    fs::create_dir(&state).unwrap();
    let second = Client::open(&state).await;
    let mut config2 = config.clone();
    config2["root"] = json!(root);
    succeeded(&second.call(&config2, "setup-hook.v1", json!({})).await);
    let read = read_copy(&second, &config2).await;
    let mut args = move_args(&base, &read);
    args["patch"] = patch["patch"].clone();
    let reply = second.call(&config2, "move.v1", args).await;
    assert_eq!(
        succeeded(&reply),
        &json!({"base":base,"files":patch["files"]})
    );
    let visible = |root: &Path| {
        tree_state(root)
            .into_iter()
            .filter(|(p, _)| !p.starts_with(".git") && !p.starts_with("ignored"))
            .collect::<BTreeMap<_, _>>()
    };
    assert_eq!(visible(&root), visible(&first.root));
    assert_eq!(read_copy(&second, &config2).await["files"], patch["files"]);
    let mut command = view_command(&first);
    command
        .current_dir(&root)
        .args(["diff", "--cached", "--name-only"]);
    let output = command.output().unwrap();
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    let output = view_command(&first)
        .current_dir(&root)
        .args(["status", "--porcelain=v1", "-uall"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        " D delete.txt\n M tracked.txt\n?? new.txt\n"
    );
    fs::write(first.repository.join("tracked.txt"), "incompatible\n").unwrap();
    let next = first.scratch.commit(&first.repository);
    let read = read_copy(&second, &config2).await;
    let before = tree_state(&root);
    let mut args = move_args(&next, &read);
    args["patch"] = patch["patch"].clone();
    failed(
        &second.call(&config2, "move.v1", args).await,
        "the patch does not apply",
    );
    assert_eq!(tree_state(&root), before);
    assert_eq!(read_copy(&second, &config2).await, read);
    for (invalid_patch, cause) in [
        (
            patch["patch"]
                .as_str()
                .unwrap()
                .replace("new.txt", "nested/.GiT/file"),
            "the patch does not apply",
        ),
        ("x".repeat(MAX_BYTES + 1), "the patch is over 8 MiB"),
    ] {
        let mut args = move_args(&base, &read);
        args["patch"] = json!(invalid_patch);
        let reply = second.call(&config2, "move.v1", args).await;
        failed(&reply, cause);
        failed(&reply, "nothing was changed");
        failed(&reply, "provide a patch");
        assert_eq!(tree_state(&root), before);
        assert_eq!(read_copy(&second, &config2).await, read);
    }
}

#[tokio::test]
async fn move_is_refused_unless_the_copy_is_as_read() {
    let f = Fixture::new();
    let (client, base, config) = f.setup().await;
    let old = read_copy(&client, &config).await;
    fs::write(f.root.join("tracked.txt"), "turn\n").unwrap();
    let read = read_copy(&client, &config).await;
    let before = tree_state(&f.root);
    let mut wrong_base = move_args(&base, &read);
    wrong_base["expectedBase"] = json!("0".repeat(40));
    for (args, cause) in [
        (move_args(&base, &old), "read changes.v1 again"),
        (wrong_base, "not at expectedBase; read changes.v1 again"),
        (
            move_args(&"0".repeat(40), &read),
            "commit is not in the repository",
        ),
    ] {
        let reply = client.call(&config, "move.v1", args).await;
        failed(&reply, cause);
        failed(&reply, "nothing was changed");
        assert_eq!(tree_state(&f.root), before);
        assert_eq!(read_copy(&client, &config).await, read);
    }
    // A failure to save the pending record is still an unchanged refusal.
    fs::set_permissions(&f.state, fs::Permissions::from_mode(0o500)).unwrap();
    let reply = client
        .call(&config, "move.v1", move_args(&base, &read))
        .await;
    fs::set_permissions(&f.state, fs::Permissions::from_mode(0o700)).unwrap();
    for cause in [
        "cannot save working-copy state record",
        "retry the move",
        "nothing was changed",
    ] {
        failed(&reply, cause);
    }
    assert_eq!(tree_state(&f.root), before);
    assert_eq!(read_copy(&client, &config).await, read);
    succeeded(
        &client
            .call(&config, "move.v1", move_args(&base, &read))
            .await,
    );
}

#[tokio::test]
async fn move_keeps_its_commit_across_setup_and_retirement() {
    let f = Fixture::new();
    let (client, _, config) = f.setup().await;
    fs::write(f.repository.join("tracked.txt"), "next\n").unwrap();
    let next = f.scratch.commit(&f.repository);
    let read = read_copy(&client, &config).await;
    succeeded(
        &client
            .call(&config, "move.v1", move_args(&next, &read))
            .await,
    );
    let before = tree_state(&f.root);
    succeeded(&client.call(&config, "setup-hook.v1", json!({})).await);
    assert_eq!(tree_state(&f.root), before);
    assert_eq!(
        fs::read_to_string(f.root.join(".git/HEAD")).unwrap(),
        format!("{next}\n")
    );
    failed(
        &client
            .call(&f.config(&next), "setup-hook.v1", json!({}))
            .await,
        "another root or commit",
    );
    succeeded(&client.call(&config, "retire-hook.v1", json!({})).await);
    assert!(!f.root.exists());
    assert!(!f.state.join("repository.git").exists());
    no_move_staging(&f);
}

#[tokio::test]
async fn move_that_ends_part_way_is_finished() {
    let f = Fixture::new();
    let (client, base, config) = f.setup().await;
    fs::write(f.repository.join("tracked.txt"), "next\n").unwrap();
    let next = f.scratch.commit(&f.repository);
    fs::create_dir_all(f.root.join("ignored/cache")).unwrap();
    fs::write(f.root.join("ignored/cache/tool"), "cache\n").unwrap();
    fs::create_dir_all(f.root.join("holder/ignored")).unwrap();
    fs::write(f.root.join("holder/ignored/cache"), "nested cache\n").unwrap();
    let nested_inode = fs::metadata(f.root.join("holder/ignored")).unwrap().ino();
    fs::create_dir(f.root.join("locked")).unwrap();
    fs::write(f.root.join("locked/file"), "locked\n").unwrap();
    fs::set_permissions(f.root.join("locked"), fs::Permissions::from_mode(0o500)).unwrap();
    let read = read_copy(&client, &config).await;
    let args = move_args(&next, &read);
    let inode = fs::metadata(f.root.join("ignored")).unwrap().ino();
    let reply = client.call(&config, "move.v1", args.clone()).await;
    // Restore even when the assertion fails, so the temporary directory can be removed.
    fs::set_permissions(f.root.join("locked"), fs::Permissions::from_mode(0o700)).unwrap();
    for cause in [
        "ended part way",
        "locked",
        "Permission denied",
        "the copy is left as the record names",
        "repeat the move with the same inputs",
        "retire the Resource and set it up again",
    ] {
        failed(&reply, cause);
    }
    for reply in [
        client.call(&config, "changes.v1", json!({})).await,
        client
            .call(&config, "move.v1", move_args(&base, &read))
            .await,
        client.call(&config, "move.v1", json!({})).await,
    ] {
        for cause in [
            "ended part way",
            "the copy is left as the record names",
            "repeat the move with the same inputs",
            "retire the Resource and set it up again",
        ] {
            failed(&reply, cause);
        }
    }
    succeeded(
        &client
            .call(
                &config,
                "file.write.v1",
                json!({"path":"meanwhile.txt","content":"temporary repair"}),
            )
            .await,
    );
    succeeded(
        &client
            .call(&config, "file.read.v1", json!({"path":"meanwhile.txt"}))
            .await,
    );
    succeeded(
        &client
            .call(&config, "directory.list.v1", json!({"path":"."}))
            .await,
    );
    let record: Value =
        serde_json::from_slice(&fs::read(f.state.join("copy.json")).unwrap()).unwrap();
    assert_eq!(record["pending"]["inputs"]["commit"], next);
    assert_eq!(record["pending"]["inputs"]["expectedBase"], base);
    assert_eq!(
        record["pending"]["inputs"]["expectedPatchSha256"],
        args["expectedPatchSha256"]
    );
    assert!(
        record["pending"]["kept"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["inode"] == inode)
    );
    no_move_staging(&f);
    // A process killed in staging leaves only this package-owned directory to clean.
    let abandoned = f.state.join("repository.git/move-staging");
    fs::create_dir(&abandoned).unwrap();
    fs::write(abandoned.join("incomplete"), "staged bytes").unwrap();
    fs::rename(f.root.join("holder"), f.root.join("holder-old")).unwrap();
    fs::create_dir(f.root.join("holder")).unwrap();
    fs::rename(
        f.root.join("holder-old/ignored"),
        f.root.join("holder/ignored"),
    )
    .unwrap();
    fs::remove_dir(f.root.join("holder-old")).unwrap();
    drop(client);
    let client = Client::open(&f.state).await;
    succeeded(&client.call(&config, "move.v1", args).await);
    assert_eq!(fs::metadata(f.root.join("ignored")).unwrap().ino(), inode);
    assert_eq!(
        fs::read(f.root.join("ignored/cache/tool")).unwrap(),
        b"cache\n"
    );
    assert!(!f.root.join("locked").exists());
    assert!(!f.root.join("meanwhile.txt").exists());
    assert_eq!(
        fs::metadata(f.root.join("holder/ignored")).unwrap().ino(),
        nested_inode
    );
    assert_eq!(
        fs::read(f.root.join("holder/ignored/cache")).unwrap(),
        b"nested cache\n"
    );
    assert_eq!(
        read_copy(&client, &config).await,
        json!({"base":next,"files":[],"patch":""})
    );
    no_move_staging(&f);
}
