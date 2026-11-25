//! `wo queue` end to end. Each test makes, in a temporary directory, a bare `origin`, a main
//! clone and worktrees under a `worktrees` directory, with git's global and system
//! configuration off and the identity set; `queue-fixture` plays `gh`, the gate and the
//! repository's Python tools.
#![cfg(unix)]

use serde_json::{Value, json};
use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const NAME: &str = "Conduit work order";
const EMAIL: &str = "work-orders@conduit.invalid";
const WO: &str = env!("CARGO_BIN_EXE_wo");
const FIXTURE: &str = env!("CARGO_BIN_EXE_queue-fixture");
const TRAILER: &str = "Co-Authored-By: Example <noreply@example.invalid>";
const STEPS: &[&str] = &[
    "check",
    "fetch",
    "backup",
    "rebase",
    "standing",
    "decisions",
    "identity",
    "record",
    "squash",
    "gate",
    "push",
    "pull-request",
    "main-unmoved",
    "merge",
    "tidy",
];
static NEXT: AtomicU64 = AtomicU64::new(0);

struct World {
    root: PathBuf,
    origin: PathBuf,
    clone: PathBuf,
    worktrees: PathBuf,
    messages: PathBuf,
    state: PathBuf,
    modes: PathBuf,
    config: PathBuf,
}

impl Drop for World {
    fn drop(&mut self) {
        if !thread::panicking() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

fn text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[track_caller]
fn exits(output: &Output, code: i32) -> String {
    let shown = text(output);
    assert_eq!(output.status.code(), Some(code), "{shown}");
    shown
}

#[track_caller]
fn wait_for(what: &str, mut done: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(60);
    while !done() {
        assert!(Instant::now() < deadline, "timed out waiting for {what}");
        thread::sleep(Duration::from_millis(20));
    }
}

fn message_text(name: &str, title: &str) -> String {
    format!("{title}\n\nThe body of {name}'s change.\n\n{TRAILER}\n")
}

fn order_text(record: &str) -> String {
    format!("# Work order\n\nWhat to do.\n\n## Record\n\n{record}\n")
}

impl World {
    fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|elapsed| elapsed.subsec_nanos())
            .unwrap_or(0);
        let made = std::env::temp_dir().join(format!(
            "a78-coordinator-{}-{}-{nanos}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&made).unwrap();
        let root = fs::canonicalize(&made).unwrap();
        for dir in ["home", "worktrees", "messages", "modes"] {
            fs::create_dir(root.join(dir)).unwrap();
        }
        let world = Self {
            origin: root.join("origin.git"),
            clone: root.join("clone"),
            worktrees: root.join("worktrees"),
            messages: root.join("messages"),
            state: root.join("state"),
            modes: root.join("modes"),
            config: root.join("config.json"),
            root,
        };
        world.git(
            &world.root,
            &["init", "--quiet", "--bare", "-b", "main", "origin.git"],
        );
        world.git(&world.root, &["init", "--quiet", "-b", "main", "clone"]);
        let origin = world.origin.display().to_string();
        world.git(&world.clone, &["remote", "add", "origin", &origin]);
        let clone = world.clone.clone();
        for (path, content) in [
            (".gitignore", "target/\n"),
            ("README.md", "The queue's test repository.\n"),
            ("spec/a.md", "a\n"),
            ("spec/gone.md", "gone\n"),
            (
                "conformance/expected-failures.yaml",
                "- case: first\n  why: base\n",
            ),
            ("engine/src/lib.rs", "pub fn f() -> u8 {\n    1\n}\n"),
        ] {
            world.write(&clone, path, content);
        }
        world.standing(&clone);
        world.commit(&clone, "the initial tree");
        world.git(&clone, &["push", "--quiet", "origin", "main"]);
        world.git(&clone, &["fetch", "--quiet", "origin"]);
        world.write_config(&world.default_config());
        world
    }

    fn default_config(&self) -> Value {
        let path = |path: &Path| path.display().to_string();
        json!({
            "clone": path(&self.clone),
            "worktrees": path(&self.worktrees),
            "state": path(&self.state),
            "messages": path(&self.messages),
            "remote": "origin",
            "base": "main",
            "identity": {"name": NAME, "email": EMAIL},
            "python": [FIXTURE, "python", "--messages", path(&self.messages)],
            "gate": {
                "argv": [FIXTURE, "gate", "--base", "{base}"],
                "env": {
                    "QUEUE_FIXTURE_GATE_MODES": path(&self.modes),
                    "QUEUE_FIXTURE_ORIGIN": path(&self.origin),
                },
            },
            "gh": [FIXTURE, "gh", path(&self.origin)],
            "pr_footer": "Merged by the queue's tests.",
            "trailers": ["Co-Authored-By: ", "Claude-Session: "],
        })
    }

    fn write_config(&self, config: &Value) {
        fs::write(&self.config, config.to_string()).unwrap();
    }

    fn edit_config(&self, edit: impl FnOnce(&mut Value)) {
        let mut config: Value =
            serde_json::from_str(&fs::read_to_string(&self.config).unwrap()).unwrap();
        edit(&mut config);
        self.write_config(&config);
    }

    fn env(&self, command: &mut Command) {
        command
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .env("HOME", self.root.join("home"))
            .env("LC_ALL", "C")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CEILING_DIRECTORIES", &self.root)
            .env("GIT_AUTHOR_NAME", NAME)
            .env("GIT_AUTHOR_EMAIL", EMAIL)
            .env("GIT_COMMITTER_NAME", NAME)
            .env("GIT_COMMITTER_EMAIL", EMAIL);
    }

    fn try_git(&self, dir: &Path, args: &[&str]) -> Output {
        let mut command = Command::new("git");
        self.env(&mut command);
        command
            .args([
                "-c",
                "core.hooksPath=/dev/null",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .current_dir(dir)
            .stdin(Stdio::null())
            .output()
            .unwrap()
    }

    #[track_caller]
    fn git(&self, dir: &Path, args: &[&str]) -> String {
        let output = self.try_git(dir, args);
        assert!(output.status.success(), "git {args:?}: {}", text(&output));
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    #[track_caller]
    fn origin(&self, args: &[&str]) -> String {
        let mut all = vec!["--git-dir", self.origin.to_str().unwrap()];
        all.extend(args);
        self.git(&self.root, &all)
    }

    fn main_head(&self) -> String {
        self.origin(&["rev-parse", "refs/heads/main"])
    }

    fn write(&self, dir: &Path, path: &str, content: &str) {
        let path = dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    /// Regenerates `engine/STANDING.md` in `dir` as the fixture writes it.
    fn standing(&self, dir: &Path) {
        let mut command = Command::new(FIXTURE);
        self.env(&mut command);
        let output = command
            .args(["python", "tools/promise_standing.py", "--write"])
            .current_dir(dir)
            .output()
            .unwrap();
        exits(&output, 0);
    }

    fn commit(&self, dir: &Path, subject: &str) -> String {
        self.git(dir, &["add", "-A"]);
        self.git(dir, &["commit", "--quiet", "-m", subject]);
        self.git(dir, &["rev-parse", "HEAD"])
    }

    /// A worktree of the clone, `worktrees/NAME`, on a new `branch` from `start`.
    fn branch_from(&self, name: &str, branch: &str, start: &str) -> PathBuf {
        let dir = self.worktrees.join(name);
        self.git(&self.clone, &["fetch", "--quiet", "origin"]);
        self.git(
            &self.clone,
            &[
                "worktree",
                "add",
                "--quiet",
                "--no-track",
                "-b",
                branch,
                dir.to_str().unwrap(),
                start,
            ],
        );
        dir
    }

    fn branch(&self, name: &str, branch: &str) -> PathBuf {
        self.branch_from(name, branch, "refs/remotes/origin/main")
    }

    fn change(&self, dir: &Path, files: &[(&str, &str)], subject: &str) -> String {
        for (path, content) in files {
            self.write(dir, path, content);
        }
        self.commit(dir, subject)
    }

    /// A worktree with one change to `spec/NAME.md`, its message and, for a `wo/` branch,
    /// its order text with a Record.
    fn entry(&self, name: &str, branch: &str) -> PathBuf {
        let dir = self.branch(name, branch);
        let mut files = vec![(format!("spec/{name}.md"), format!("{name}\n"))];
        if let Some(rest) = branch.strip_prefix("wo/") {
            files.push((
                format!("docs/work-orders/{rest}.md"),
                order_text("Run by the queue's tests."),
            ));
        }
        let files: Vec<(&str, &str)> = files
            .iter()
            .map(|(path, content)| (path.as_str(), content.as_str()))
            .collect();
        self.change(&dir, &files, &format!("{name}'s change"));
        self.message(name, &format!("{name} lands"));
        dir
    }

    fn message(&self, name: &str, title: &str) {
        fs::write(
            self.messages.join(format!("{name}.txt")),
            message_text(name, title),
        )
        .unwrap();
    }

    /// Moves `origin`'s `main`: writes (`Some`) or removes (`None`) each path in the clone.
    fn advance_main(
        &self,
        files: &[(&str, Option<&str>)],
        regenerate: bool,
        subject: &str,
    ) -> String {
        let clone = &self.clone;
        self.git(clone, &["fetch", "--quiet", "origin"]);
        self.git(
            clone,
            &["reset", "--quiet", "--hard", "refs/remotes/origin/main"],
        );
        for (path, content) in files {
            match content {
                Some(content) => self.write(clone, path, content),
                None => fs::remove_file(clone.join(path)).unwrap(),
            }
        }
        if regenerate {
            self.standing(clone);
        }
        let head = self.commit(clone, subject);
        self.git(clone, &["push", "--quiet", "origin", "HEAD:main"]);
        self.git(clone, &["fetch", "--quiet", "origin"]);
        head
    }

    fn set_gate(&self, name: &str, mode: &str) {
        fs::write(self.modes.join(name), mode).unwrap();
    }

    fn wo_command(&self, args: &[&str]) -> Command {
        let mut command = Command::new(WO);
        self.env(&mut command);
        command
            .arg("queue")
            .args(args)
            .arg("--config")
            .arg(&self.config)
            .current_dir(&self.root)
            .stdin(Stdio::null());
        command
    }

    fn wo(&self, args: &[&str]) -> Output {
        self.wo_command(args).output().unwrap()
    }

    fn add(&self, name: &str, extra: &[&str]) -> Output {
        let mut args = vec!["add", name];
        args.extend(extra);
        self.wo(&args)
    }

    fn run(&self) -> Output {
        self.wo(&["run"])
    }

    fn spawn(&self, args: &[&str], log: &Path) -> Child {
        let file = fs::File::create(log).unwrap();
        let errors = file.try_clone().unwrap();
        self.wo_command(args)
            .stdout(file)
            .stderr(errors)
            .spawn()
            .unwrap()
    }

    fn ledger(&self) -> Vec<Value> {
        fs::read_to_string(self.state.join("ledger.jsonl"))
            .unwrap_or_default()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    }

    fn lines_of(&self, name: &str) -> Vec<Value> {
        self.ledger()
            .into_iter()
            .filter(|line| line["name"] == name)
            .collect()
    }

    fn queue_bytes(&self) -> Option<Vec<u8>> {
        fs::read(self.state.join("queue.jsonl")).ok()
    }

    fn status(&self) -> String {
        exits(&self.wo(&["status"]), 0)
    }

    fn status_line(&self, name: &str) -> String {
        self.status()
            .lines()
            .find(|line| line.starts_with(&format!("{name} ")))
            .unwrap_or_else(|| panic!("no status line for {name}"))
            .to_string()
    }

    fn pull_requests(&self) -> Vec<Value> {
        let store: Value = serde_json::from_str(
            &fs::read_to_string(self.origin.join("fixture-prs.json")).unwrap(),
        )
        .unwrap();
        store["prs"].as_array().unwrap().clone()
    }

    /// The commit message of `commit` in `origin`, after its headers.
    fn message_of(&self, commit: &str) -> String {
        let raw = self.origin(&["cat-file", "commit", commit]);
        raw.split_once("\n\n").unwrap().1.to_string()
    }

    fn backup(&self, name: &str, attempt: u64) -> String {
        self.git(
            &self.clone,
            &["rev-parse", &format!("refs/wo-backup/{name}/{attempt}")],
        )
    }
}

#[test]
fn queue_merges_in_order() {
    let world = World::new();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    world.edit_config(|config| {
        config["gate"]["cache"] = json!({
            "address": format!("127.0.0.1:{port}"),
            "env": {"QUEUE_FIXTURE_CACHE": "on"},
        });
    });
    let alpha = world.entry("alpha", "wo/101-alpha");
    let beta = world.entry("beta", "wo/102-beta");
    let moved = world.advance_main(&[("README.md", Some("main moved\n"))], false, "main moves");
    exits(&world.add("alpha", &[]), 0);
    exits(&world.add("beta", &[]), 0);
    let shown = exits(&world.run(), 0);
    assert!(
        shown.contains("queue done: merged [alpha, beta] failed [] skipped []"),
        "{shown}"
    );

    let history = world.origin(&["rev-list", "--first-parent", "refs/heads/main"]);
    let history: Vec<&str> = history.lines().collect();
    assert_eq!(history.len(), 4, "{history:?}");
    let (beta_merged, alpha_merged) = (history[0], history[1]);
    assert_eq!(history[2], moved);
    for (name, merged, parent, number, dir) in [
        ("alpha", alpha_merged, moved.as_str(), 1, &alpha),
        ("beta", beta_merged, alpha_merged, 2, &beta),
    ] {
        let parents = world.origin(&["rev-list", "--parents", "-n", "1", merged]);
        assert_eq!(parents, format!("{merged} {parent}"));
        let subject = format!("{name} lands (#{number})");
        let body = format!("The body of {name}'s change.\n\n{TRAILER}");
        assert_eq!(world.message_of(merged), format!("{subject}\n\n{body}"));
        assert!(
            shown.contains(&format!("MERGED {name} as {merged} {subject}")),
            "{shown}"
        );
        let tree = world.origin(&["rev-parse", &format!("{merged}^{{tree}}")]);
        assert_eq!(tree, world.git(dir, &["rev-parse", "HEAD^{tree}"]));
        assert!(!dir.join("target").exists(), "tidy removed target/");
        assert!(world.state.join(format!("gate-{name}-1.txt")).is_file());

        let lines = world.lines_of(name);
        let steps: Vec<&str> = lines
            .iter()
            .map(|line| line["step"].as_str().unwrap())
            .collect();
        let mut expected = STEPS.to_vec();
        expected.push("end");
        assert_eq!(steps, expected);
        assert!(
            lines[..STEPS.len()]
                .iter()
                .all(|line| line["outcome"] == "ok")
        );
        let end = lines.last().unwrap();
        assert_eq!(end["outcome"], "merged");
        assert_eq!(end["pr"], number);
        assert_eq!(end["commit"], merged);
        let gate = STEPS.iter().position(|step| *step == "gate").unwrap();
        assert!(
            lines[..gate]
                .iter()
                .all(|line| line["gate_record"].is_null())
        );
        for line in &lines[gate..] {
            let record = line["gate_record"].as_str().unwrap();
            assert!(Path::new(record).is_file(), "{record}");
        }
        let record: Value = serde_json::from_str(
            &fs::read_to_string(lines[gate]["gate_record"].as_str().unwrap()).unwrap(),
        )
        .unwrap();
        assert_eq!(
            record["cache"], "on",
            "the cache answered, so its env was added"
        );
        assert_eq!(
            record["base"], lines[gate]["pinned"],
            "{{base}} became PINNED"
        );
    }
    let ledger = world.ledger();
    assert_eq!(
        ledger.len(),
        2 * (STEPS.len() + 1),
        "a line per step and an end each"
    );
    let merged = ledger
        .iter()
        .filter(|line| line["step"] == "end" && line["outcome"] == "merged")
        .count();
    assert_eq!(merged, 2);
    let requests = world.pull_requests();
    assert_eq!(
        requests[0]["body"],
        "The body of alpha's change.\n\nMerged by the queue's tests."
    );
    assert_eq!(requests[0]["title"], "alpha lands");
    let standing = world.origin(&["cat-file", "blob", "refs/heads/main:engine/STANDING.md"]);
    assert!(standing.contains("spec/alpha.md 1") && standing.contains("spec/beta.md 1"));
    drop(listener);
}

#[test]
fn queue_onto_replays_own_commits() {
    let world = World::new();
    let lower = world.branch("lower", "stack-lower");
    let lower_head = world.change(&lower, &[("spec/lower.md", "lower\n")], "the lower change");
    let upper = world.branch_from("upper", "stack-upper", "stack-lower");
    world.change(&upper, &[("spec/upper.md", "upper\n")], "the upper change");
    world.message("upper", "upper lands");
    world.advance_main(&[("README.md", Some("main moved\n"))], false, "main moves");
    exits(&world.add("upper", &["--onto", &lower_head]), 0);
    let shown = exits(&world.run(), 0);
    assert!(shown.contains("MERGED upper as"), "{shown}");
    let landed = world.main_head();
    let changed = world.origin(&[
        "diff-tree",
        "--no-commit-id",
        "--name-only",
        "-r",
        &format!("{landed}^"),
        &landed,
    ]);
    let changed: Vec<&str> = changed.lines().collect();
    assert_eq!(changed, ["engine/STANDING.md", "spec/upper.md"]);
    let lower_there = world.try_git(
        &world.root,
        &[
            "--git-dir",
            world.origin.to_str().unwrap(),
            "cat-file",
            "-e",
            "refs/heads/main:spec/lower.md",
        ],
    );
    assert!(
        !lower_there.status.success(),
        "the lower branch's change did not land"
    );
    let rebase = world
        .lines_of("upper")
        .into_iter()
        .find(|line| line["step"] == "rebase")
        .unwrap();
    assert!(
        rebase["why"]
            .as_str()
            .unwrap()
            .contains("replayed 1 commits"),
        "{rebase}"
    );
}

#[test]
fn queue_skips_dependents() {
    let world = World::new();
    world.entry("first", "skip-first");
    world.entry("second", "skip-second");
    world.entry("third", "skip-third");
    world.set_gate("first", "fail");
    exits(&world.add("first", &[]), 0);
    exits(&world.add("second", &["--after", "first"]), 0);
    exits(&world.add("third", &[]), 0);
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("FAILED first at gate: the gate ended exit 1"),
        "{shown}"
    );
    assert!(
        shown.contains("  FAIL fixture row: the gate's fixture was set to fail"),
        "{shown}"
    );
    assert!(
        shown.contains("SKIPPED second: first did not merge"),
        "{shown}"
    );
    assert!(shown.contains("MERGED third as"), "{shown}");
    assert!(
        shown.contains("queue done: merged [third] failed [first] skipped [second]"),
        "{shown}"
    );
    let second = world.lines_of("second");
    assert_eq!(second.len(), 1);
    assert_eq!(second[0]["step"], "end");
    assert_eq!(second[0]["outcome"], "skipped");
    assert_eq!(second[0]["why"], "first did not merge");
    let first_end = world.lines_of("first").pop().unwrap();
    assert_eq!(first_end["outcome"], "failed");
    assert!(Path::new(first_end["gate_record"].as_str().unwrap()).is_file());
}

#[test]
fn queue_add_refuses() {
    let world = World::new();
    let plain = world.branch("plain", "plain-branch");
    world.change(&plain, &[("spec/plain.md", "plain\n")], "the plain change");
    let message = world.messages.join("plain.txt");
    let refused = |name: &str, extra: &[&str], why: &str| {
        let before = world.queue_bytes();
        let shown = exits(&world.add(name, extra), 2);
        assert!(shown.contains(why), "{why}: {shown}");
        assert_eq!(world.queue_bytes(), before, "{why}: queue.jsonl changed");
    };
    refused("plain", &[], "cannot be read");
    for (text, why) in [
        (
            format!("\n\nThe body.\n\n{TRAILER}\n"),
            "the title, is empty",
        ),
        (
            format!("Title\nThe body.\n\n{TRAILER}\n"),
            "second line is not blank",
        ),
        (format!("Title\n\n{TRAILER}\n"), "has no body"),
        (
            "Title\n\nThe body.\n".to_string(),
            "does not end with a trailer line",
        ),
    ] {
        fs::write(&message, text).unwrap();
        refused("plain", &[], why);
    }
    let missing = world.branch("missing", "wo/310-missing");
    world.change(
        &missing,
        &[("spec/missing.md", "missing\n")],
        "no order text",
    );
    world.message("missing", "missing lands");
    refused("missing", &[], "has no order text");
    let unrun = world.branch("unrun", "wo/311-unrun");
    world.change(
        &unrun,
        &[("docs/work-orders/311-unrun.md", &order_text("Not yet run."))],
        "an order not yet run",
    );
    world.message("unrun", "unrun lands");
    refused("unrun", &[], "Record is `Not yet run.`");

    world.message("plain", "plain lands");
    exits(&world.add("plain", &[]), 0);
    refused("plain", &[], "already in the queue");
    let ahead = world.branch("ahead", "drafts-ahead");
    world.change(
        &ahead,
        &[("docs/work-orders/500-later.md", &order_text("Not yet run."))],
        "a text written ahead",
    );
    world.message("ahead", "ahead lands");
    refused("ahead", &["--after", "nobody"], "--after nobody");
    refused("ahead", &["--onto", "abc123"], "not a full commit ID");
    exits(&world.add("ahead", &[]), 0);

    let gone = world.entry("gone", "wo/320-gone");
    exits(&world.add("gone", &[]), 0);
    let gone_head = world.git(&gone, &["rev-parse", "HEAD"]);
    fs::remove_file(world.messages.join("gone.txt")).unwrap();
    let revert = world.entry("revert", "wo/321-revert");
    exits(&world.add("revert", &[]), 0);
    world.change(
        &revert,
        &[(
            "docs/work-orders/321-revert.md",
            &order_text("Not yet run."),
        )],
        "the Record reverted",
    );
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("MERGED plain as") && shown.contains("MERGED ahead as"),
        "{shown}"
    );
    assert!(
        shown.contains("FAILED gone at check: the message"),
        "{shown}"
    );
    assert_eq!(world.git(&gone, &["rev-parse", "HEAD"]), gone_head);
    assert!(
        shown.contains(
            "FAILED revert at record: docs/work-orders/321-revert.md's Record is `Not yet run.`"
        ),
        "{shown}"
    );
}

#[test]
fn queue_resolves_standing() {
    let world = World::new();
    let dir = world.branch("standing", "standing-branch");
    world.write(&dir, "spec/y.md", "y\ny\n");
    world.standing(&dir);
    world.commit(&dir, "the branch changes STANDING");
    world.message("standing", "standing lands");
    world.advance_main(&[("spec/z.md", Some("z\n"))], true, "main changes STANDING");
    exits(&world.add("standing", &[]), 0);
    let shown = exits(&world.run(), 0);
    assert!(shown.contains("MERGED standing as"), "{shown}");
    let rebase = world
        .lines_of("standing")
        .into_iter()
        .find(|line| line["step"] == "rebase")
        .unwrap();
    assert!(
        rebase["why"].as_str().unwrap().contains("settled 1 stops"),
        "{rebase}"
    );
    let standing = world.origin(&["cat-file", "blob", "refs/heads/main:engine/STANDING.md"]);
    assert!(
        standing.contains("spec/y.md 2") && standing.contains("spec/z.md 1"),
        "{standing}"
    );
    let clone = &world.clone;
    world.git(clone, &["fetch", "--quiet", "origin"]);
    world.git(
        clone,
        &["reset", "--quiet", "--hard", "refs/remotes/origin/main"],
    );
    world.standing(clone);
    assert_eq!(
        world.git(clone, &["status", "--porcelain"]),
        "",
        "the merged STANDING is the fixture's"
    );
}

#[test]
fn queue_other_conflict_stops() {
    let world = World::new();
    let failures = world.branch("failures", "conflict-failures");
    world.change(
        &failures,
        &[(
            "conformance/expected-failures.yaml",
            "- case: first\n  why: the branch\n",
        )],
        "the branch's expected failure",
    );
    let source = world.branch("source", "conflict-source");
    world.change(
        &source,
        &[("engine/src/lib.rs", "pub fn f() -> u8 {\n    2\n}\n")],
        "the branch's source",
    );
    let deleted = world.branch("deleted", "conflict-deleted");
    world.change(
        &deleted,
        &[("spec/gone.md", "gone\nchanged\n")],
        "the branch changes a spec file",
    );
    for name in ["failures", "source", "deleted"] {
        world.message(name, &format!("{name} lands"));
    }
    world.advance_main(
        &[
            (
                "conformance/expected-failures.yaml",
                Some("- case: first\n  why: main\n"),
            ),
            ("engine/src/lib.rs", Some("pub fn f() -> u8 {\n    3\n}\n")),
            ("spec/gone.md", None),
        ],
        true,
        "main changes the same lines",
    );
    let main = world.main_head();
    for name in ["failures", "source", "deleted"] {
        exits(&world.add(name, &[]), 0);
    }
    let shown = exits(&world.run(), 1);
    for (name, why) in [
        (
            "failures",
            "conformance/expected-failures.yaml: a conflict, its first hunk at line 2:   why: main",
        ),
        (
            "source",
            "engine/src/lib.rs: a conflict, its first hunk at line 2:     3",
        ),
        (
            "deleted",
            "spec/gone.md: deleted by ours, changed by theirs",
        ),
    ] {
        assert!(
            shown.contains(&format!("FAILED {name} at rebase: the rebase stopped")),
            "{shown}"
        );
        assert!(shown.contains(&format!("  {why}")), "{why}: {shown}");
        let dir = world.worktrees.join(name);
        let gitdir = world.git(&dir, &["rev-parse", "--absolute-git-dir"]);
        assert!(
            !Path::new(&gitdir).join("rebase-merge").exists(),
            "no rebase is in progress"
        );
        assert!(
            !Path::new(&gitdir).join("rebase-apply").exists(),
            "no rebase is in progress"
        );
        assert_eq!(
            world.git(&dir, &["rev-parse", "HEAD"]),
            world.backup(name, 1)
        );
        assert_eq!(
            world.git(&dir, &["status", "--porcelain", "--untracked-files=all"]),
            ""
        );
    }
    assert_eq!(world.main_head(), main, "nothing merged");
}

#[test]
fn queue_squash_keeps_head() {
    let world = World::new();
    let dup = world.branch("dup", "dup-branch");
    world.change(
        &dup,
        &[("spec/dup.md", "same\n")],
        "a change main holds already",
    );
    world.message("dup", "dup lands");
    let drop_dir = world.branch("drop", "drop-branch");
    world.change(
        &drop_dir,
        &[
            ("spec/drop.md", "drop\n"),
            ("drop-message", "the fixture removes the message\n"),
        ],
        "a change whose STANDING step drops the message",
    );
    world.message("drop", "drop lands");
    let pinned = world.advance_main(
        &[("spec/dup.md", Some("same\n"))],
        true,
        "main holds the change",
    );
    exits(&world.add("dup", &[]), 0);
    exits(&world.add("drop", &[]), 0);
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("FAILED dup at squash: nothing to merge"),
        "{shown}"
    );
    assert_eq!(
        world.git(&dup, &["rev-parse", "HEAD"]),
        pinned,
        "HEAD is the rebased head"
    );
    let staged = world.try_git(&dup, &["diff", "--cached", "--quiet"]);
    assert!(staged.status.success(), "nothing is staged");
    assert!(shown.contains("MERGED drop as"), "{shown}");
    assert!(
        !world.messages.join("drop.txt").exists(),
        "the fixture removed the message"
    );
    let landed = world.main_head();
    assert_eq!(
        world.message_of(&landed),
        format!("drop lands (#1)\n\nThe body of drop's change.\n\n{TRAILER}")
    );
}

#[test]
fn queue_decisions_check_stops() {
    let world = World::new();
    let dir = world.branch("decide", "decide-branch");
    world.change(
        &dir,
        &[("spec/decide.md", "decide\n"), ("decisions-fail", "x\n")],
        "a failing decision",
    );
    world.message("decide", "decide lands");
    let main = world.main_head();
    exits(&world.add("decide", &[]), 0);
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("FAILED decide at decisions: tools/check_decisions.py ended \"Errors: 1\""),
        "{shown}"
    );
    assert!(
        shown.contains("  error: docs/decisions/log.md: fixture: the tree holds decisions-fail"),
        "{shown}"
    );
    assert_eq!(world.main_head(), main);
}

#[test]
fn queue_refuses_foreign_identity() {
    let world = World::new();
    let dir = world.branch("foreign", "foreign-branch");
    world.write(&dir, "spec/foreign.md", "foreign\n");
    world.git(&dir, &["add", "-A"]);
    world.git(
        &dir,
        &[
            "commit",
            "--quiet",
            "--author",
            "Someone Else <someone@example.invalid>",
            "-m",
            "another author's commit",
        ],
    );
    world.message("foreign", "foreign lands");
    let main = world.main_head();
    exits(&world.add("foreign", &[]), 0);
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("FAILED foreign at identity: 1 of the 2 commits"),
        "{shown}"
    );
    assert!(
        shown.contains("author Someone Else <someone@example.invalid>"),
        "{shown}"
    );
    assert_eq!(world.main_head(), main);
}

#[test]
fn queue_refuses_when_main_moved() {
    let world = World::new();
    world.entry("mover", "mover-branch");
    world.set_gate("mover", "move-main");
    let main = world.main_head();
    exits(&world.add("mover", &[]), 0);
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("FAILED mover at main-unmoved: main moved during the gate"),
        "{shown}"
    );
    let now = world.main_head();
    assert_ne!(now, main);
    assert_eq!(
        world.origin(&["log", "-1", "--format=%s", &now]),
        "main moved during the gate"
    );
    assert_eq!(
        world.origin(&["rev-parse", &format!("{now}^")]),
        main,
        "only the gate's commit landed"
    );
    let requests = world.pull_requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0]["state"], "open", "the pull request stays open");
}

#[test]
fn queue_refuses_a_clone() {
    let world = World::new();
    let dir = world.entry("cloned", "cloned-branch");
    exits(&world.add("cloned", &[]), 0);
    world.git(
        &world.clone,
        &["worktree", "remove", "--force", dir.to_str().unwrap()],
    );
    world.git(
        &world.root,
        &["clone", "--quiet", "origin.git", dir.to_str().unwrap()],
    );
    let config = fs::read(dir.join(".git/config")).unwrap();
    let refs = world.git(&dir, &["for-each-ref"]);
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("FAILED cloned at check: the worktree"),
        "{shown}"
    );
    assert!(shown.contains("a clone of its own"), "{shown}");
    assert_eq!(fs::read(dir.join(".git/config")).unwrap(), config);
    assert_eq!(world.git(&dir, &["for-each-ref"]), refs);
    assert_eq!(world.lines_of("cloned").len(), 2, "check, then end");

    let other = world.worktrees.join("other");
    world.git(
        &world.root,
        &["clone", "--quiet", "origin.git", other.to_str().unwrap()],
    );
    world.message("other", "other lands");
    let shown = exits(&world.add("other", &[]), 2);
    assert!(shown.contains("a clone of its own"), "{shown}");

    let second = world.root.join("second");
    world.git(
        &world.root,
        &["clone", "--quiet", "origin.git", second.to_str().unwrap()],
    );
    let stranger = world.worktrees.join("stranger");
    world.git(
        &second,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "stranger",
            stranger.to_str().unwrap(),
        ],
    );
    world.message("stranger", "stranger lands");
    let shown = exits(&world.add("stranger", &[]), 2);
    assert!(
        shown.contains("not a directory under the clone's .git/worktrees/"),
        "{shown}"
    );
}

#[test]
fn queue_single_runner() {
    let world = World::new();
    let kit = world.root.join("kit-lock");
    world.edit_config(|config| config["legacy_lock"] = json!(kit.display().to_string()));
    world.entry("held", "held-branch");
    world.set_gate("held", "wait");
    exits(&world.add("held", &[]), 0);
    let log = world.root.join("first-run.log");
    let mut first = world.spawn(&["run"], &log);
    let waiting = world.modes.join("held.waiting");
    wait_for("the held gate", || waiting.exists());

    world.entry("meanwhile", "meanwhile-branch");
    exits(&world.add("meanwhile", &[]), 0);
    let shown = exits(&world.wo(&["run", "--no-wait"]), 3);
    assert!(
        shown.contains("queue.lock is held by another run"),
        "{shown}"
    );
    assert!(world.lines_of("meanwhile").is_empty(), "no entry started");
    let made = fs::create_dir(&kit);
    assert_eq!(made.unwrap_err().kind(), std::io::ErrorKind::AlreadyExists);
    let owner = fs::read_to_string(kit.join("owner")).unwrap();
    assert_eq!(
        owner.trim_end(),
        format!(
            "wo queue pid {} lock {}",
            first.id(),
            world.state.join("queue.lock").display()
        )
    );
    fs::write(world.modes.join("held.release"), "").unwrap();
    assert!(first.wait().unwrap().success());
    let shown = fs::read_to_string(&log).unwrap();
    assert!(
        shown.contains("MERGED held as") && shown.contains("MERGED meanwhile as"),
        "{shown}"
    );
    assert!(!kit.exists(), "the kit's lock is gone once the run exits");

    fs::create_dir(&kit).unwrap();
    fs::write(kit.join("owner"), "kit batch 4242\n").unwrap();
    world.entry("later", "later-branch");
    exits(&world.add("later", &[]), 0);
    let shown = exits(&world.wo(&["run", "--no-wait"]), 3);
    assert!(
        shown.contains(&format!(
            "the kit's lock {} is held, owner: kit batch 4242",
            kit.display()
        )),
        "{shown}"
    );
    assert!(world.lines_of("later").is_empty(), "no entry started");
    let log = world.root.join("second-run.log");
    let mut second = world.spawn(&["run"], &log);
    wait_for("the wait to be said", || {
        fs::read_to_string(&log)
            .unwrap_or_default()
            .contains("waiting for the kit's lock")
    });
    thread::sleep(Duration::from_millis(1200));
    fs::remove_dir_all(&kit).unwrap();
    assert!(second.wait().unwrap().success());
    let shown = fs::read_to_string(&log).unwrap();
    assert_eq!(
        shown.matches("waiting for the kit's lock").count(),
        1,
        "{shown}"
    );
    assert!(shown.contains("MERGED later as"), "{shown}");
    assert!(!kit.exists());

    fs::create_dir(&kit).unwrap();
    fs::write(kit.join("owner"), "wo queue pid 999999\n").unwrap();
    world.entry("stale", "stale-branch");
    exits(&world.add("stale", &[]), 0);
    let shown = exits(&world.wo(&["run", "--no-wait"]), 0);
    assert!(
        shown.contains("left by a stopped run; owner: wo queue pid 999999"),
        "{shown}"
    );
    assert!(shown.contains("MERGED stale as"), "{shown}");
    assert!(!kit.exists());
}

#[test]
fn queue_status_and_retry() {
    let world = World::new();
    world.entry("first", "first-branch");
    world.entry("second", "second-branch");
    world.set_gate("first", "fail");
    exits(&world.add("first", &[]), 0);
    exits(&world.add("second", &["--after", "first"]), 0);
    let line = world.status_line("first");
    assert!(
        line.starts_with(
            "first branch=first-branch state=pending attempt=1 step=- pr=- commit=- gate_record=-"
        ),
        "{line}"
    );
    exits(&world.run(), 1);
    let shown = world.status();
    assert!(
        !shown.contains("The body of"),
        "status prints no message: {shown}"
    );
    let line = world.status_line("first");
    assert!(line.contains("state=failed attempt=1 step=gate"), "{line}");
    let record = line
        .split("gate_record=")
        .nth(1)
        .unwrap()
        .split(' ')
        .next()
        .unwrap();
    assert!(Path::new(record).is_file(), "{line}");
    assert!(line.ends_with("origin/main_holds_head=no"), "{line}");
    assert!(world.status_line("second").contains("state=skipped"));

    let shown = exits(&world.wo(&["retry", "first"]), 0);
    assert!(shown.contains("retry first: attempt 2 opened"), "{shown}");
    let shown = exits(&world.wo(&["retry", "first"]), 2);
    assert!(
        shown.contains("attempt 2 has no end line: it is pending"),
        "{shown}"
    );
    assert!(
        world
            .status_line("first")
            .contains("state=pending attempt=2 step=-")
    );
    world.set_gate("first", "pass");
    let shown = exits(&world.run(), 0);
    assert!(shown.contains("MERGED first as"), "{shown}");
    let end = world.lines_of("first").pop().unwrap();
    assert_eq!(
        (end["attempt"].clone(), end["outcome"].clone()),
        (json!(2), json!("merged"))
    );
    let line = world.status_line("first");
    assert!(
        line.contains("state=merged attempt=2 step=tidy pr=1 commit="),
        "{line}"
    );
    assert!(line.ends_with("origin/main_holds_commit=yes"), "{line}");
    exits(&world.wo(&["retry", "first"]), 2);
    exits(&world.wo(&["retry", "second"]), 0);
    let shown = exits(&world.run(), 0);
    assert!(shown.contains("MERGED second as"), "{shown}");

    world.entry("third", "third-branch");
    world.set_gate("third", "wait");
    exits(&world.add("third", &[]), 0);
    let mut run = world.spawn(&["run"], &world.root.join("killed-run.log"));
    let waiting = world.modes.join("third.waiting");
    wait_for("the third gate", || waiting.exists());
    assert!(world.status_line("third").contains("state=running"));
    run.kill().unwrap();
    run.wait().unwrap();
    let line = world.status_line("third");
    assert!(
        line.contains("state=stopped attempt=1 step=squash"),
        "{line}"
    );
    let backup = world.backup("third", 1);
    fs::write(world.modes.join("third.release"), "").unwrap();
    wait_for("the orphaned gate", || !waiting.exists());
    world.set_gate("third", "pass");
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("FAILED third at gate: the run that held it stopped"),
        "{shown}"
    );
    assert!(
        shown.contains("queue done: merged [] failed [third] skipped []"),
        "{shown}"
    );
    assert_eq!(
        world.backup("third", 1),
        backup,
        "the backup ref is unchanged"
    );
    let end = world.lines_of("third").pop().unwrap();
    assert_eq!(end["why"], "the run that held it stopped");
    assert!(
        world
            .status_line("third")
            .contains("state=failed attempt=1")
    );
    exits(&world.wo(&["retry", "third"]), 0);
    let shown = exits(&world.run(), 0);
    assert!(shown.contains("MERGED third as"), "{shown}");
    let line = world.status_line("third");
    assert!(line.contains("state=merged attempt=2"), "{line}");
    assert!(line.ends_with("origin/main_holds_commit=yes"), "{line}");
}

/// Commits what `dir` holds with `Someone Else` as committer, the identity as author.
fn commit_by_another_committer(world: &World, dir: &Path, subject: &str) -> String {
    world.git(dir, &["add", "-A"]);
    let mut command = Command::new("git");
    world.env(&mut command);
    let output = command
        .env("GIT_COMMITTER_NAME", "Someone Else")
        .env("GIT_COMMITTER_EMAIL", "someone@example.invalid")
        .args([
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "--quiet",
            "-m",
            subject,
        ])
        .current_dir(dir)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    exits(&output, 0);
    world.git(dir, &["rev-parse", "HEAD"])
}

#[test]
fn queue_onto_retry_replays_own_commits() {
    let world = World::new();
    let lower = world.branch("lower", "stack-lower");
    let lower_head = world.change(&lower, &[("spec/lower.md", "lower\n")], "the lower change");
    let upper = world.branch_from("upper", "stack-upper", "stack-lower");
    world.change(&upper, &[("spec/upper.md", "upper\n")], "the upper change");
    world.message("upper", "upper lands");
    world.advance_main(&[("spec/m1.md", Some("m1\n"))], true, "main adds m1");
    world.set_gate("upper", "fail");
    exits(&world.add("upper", &["--onto", &lower_head]), 0);
    let shown = exits(&world.run(), 1);
    assert!(shown.contains("FAILED upper at gate"), "{shown}");
    let first_pinned = world
        .lines_of("upper")
        .into_iter()
        .find(|line| line["step"] == "rebase")
        .map(|line| line["pinned"].as_str().unwrap().to_string())
        .unwrap();

    world.advance_main(&[("spec/m1.md", None)], true, "main reverts m1");
    exits(&world.wo(&["retry", "upper"]), 0);
    world.set_gate("upper", "pass");
    let shown = exits(&world.run(), 0);
    assert!(shown.contains("MERGED upper as"), "{shown}");
    let landed = world.main_head();
    let changed = world.origin(&[
        "diff-tree",
        "--no-commit-id",
        "--name-only",
        "-r",
        &format!("{landed}^"),
        &landed,
    ]);
    let changed: Vec<&str> = changed.lines().collect();
    assert_eq!(
        changed,
        ["engine/STANDING.md", "spec/upper.md"],
        "only the entry's own paths land"
    );
    for gone in ["spec/m1.md", "spec/lower.md"] {
        let there = world.try_git(
            &world.root,
            &[
                "--git-dir",
                world.origin.to_str().unwrap(),
                "cat-file",
                "-e",
                &format!("refs/heads/main:{gone}"),
            ],
        );
        assert!(!there.status.success(), "{gone} is not on main");
    }
    let rebase = world
        .lines_of("upper")
        .into_iter()
        .filter(|line| line["step"] == "rebase")
        .nth(1)
        .unwrap();
    assert_eq!(rebase["attempt"], 2);
    assert!(
        rebase["why"]
            .as_str()
            .unwrap()
            .contains(&format!("replayed 1 commits of {first_pinned}..HEAD")),
        "{rebase}"
    );

    // An --onto that is not an ancestor of the branch's head is refused.
    let revert = world.main_head();
    let late = world.branch_from("late", "stack-late", "stack-lower");
    world.change(&late, &[("spec/late.md", "late\n")], "the late change");
    world.message("late", "late lands");
    let before = world.queue_bytes();
    let shown = exits(&world.add("late", &["--onto", &revert]), 2);
    assert!(
        shown.contains(&format!("--onto {revert} is not an ancestor of")),
        "{shown}"
    );
    assert_eq!(world.queue_bytes(), before);

    // A branch moved off ONTO after `add` fails at rebase, and is not replayed.
    exits(&world.add("late", &["--onto", &lower_head]), 0);
    let initial = world.git(&late, &["rev-list", "--max-parents=0", "HEAD"]);
    world.git(&late, &["reset", "--quiet", "--hard", &initial]);
    world.change(
        &late,
        &[("spec/late.md", "moved\n")],
        "the late change, moved",
    );
    let main = world.main_head();
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains(&format!(
            "FAILED late at rebase: --onto {lower_head} is not an ancestor of HEAD"
        )),
        "{shown}"
    );
    assert_eq!(world.main_head(), main, "nothing merged");
}

#[test]
fn queue_turns_rerere_off() {
    let world = World::new();
    let clone = world.clone.clone();
    world.git(&clone, &["config", "rerere.enabled", "true"]);
    world.git(&clone, &["config", "rerere.autoUpdate", "true"]);
    let dir = world.branch("rr", "rr-branch");
    world.write(&dir, "engine/src/lib.rs", "pub fn f() -> u8 {\n    2\n}\n");
    world.write(&dir, "spec/rr.md", "rr\n");
    world.standing(&dir);
    let branch_head = world.commit(&dir, "the branch's source and STANDING");
    world.message("rr", "rr lands");
    world.advance_main(
        &[
            ("engine/src/lib.rs", Some("pub fn f() -> u8 {\n    3\n}\n")),
            ("spec/z.md", Some("z\n")),
        ],
        true,
        "main changes the source and STANDING",
    );
    // An earlier session recorded a resolution of the source, 2 against 3 settled as 4.
    world.git(&clone, &["checkout", "--quiet", "--detach", &branch_head]);
    let merged = world.try_git(
        &clone,
        &[
            "-c",
            "merge.conflictstyle=diff3",
            "merge",
            "--no-edit",
            "main",
        ],
    );
    assert!(!merged.status.success(), "the merge conflicts");
    world.write(
        &clone,
        "engine/src/lib.rs",
        "pub fn f() -> u8 {\n    4\n}\n",
    );
    let recorded = text(&world.try_git(&clone, &["rerere"]));
    assert!(
        recorded.contains("Recorded resolution for 'engine/src/lib.rs'"),
        "{recorded}"
    );
    world.git(&clone, &["merge", "--abort"]);
    world.git(&clone, &["checkout", "--quiet", "main"]);
    let main = world.main_head();

    exits(&world.add("rr", &[]), 0);
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("FAILED rr at rebase: the rebase stopped"),
        "{shown}"
    );
    assert!(
        shown.contains("  engine/src/lib.rs: a conflict, its first hunk at line 2:     3"),
        "{shown}"
    );
    assert_eq!(
        world.git(&dir, &["rev-parse", "HEAD"]),
        world.backup("rr", 1)
    );
    assert_eq!(
        world.git(&dir, &["status", "--porcelain", "--untracked-files=all"]),
        ""
    );
    assert_eq!(world.main_head(), main, "nothing merged");
}

#[test]
fn queue_refuses_a_foreign_committer() {
    let world = World::new();
    let dir = world.branch("committer", "committer-branch");
    world.write(&dir, "spec/committer.md", "committer\n");
    commit_by_another_committer(&world, &dir, "another committer's commit");
    world.message("committer", "committer lands");
    let rewritten = world.branch("rewritten", "rewritten-branch");
    world.write(&rewritten, "spec/rewritten.md", "rewritten\n");
    world.write(&rewritten, "decisions-fail", "x\n");
    commit_by_another_committer(&world, &rewritten, "another committer's commit, again");
    world.message("rewritten", "rewritten lands");
    world.advance_main(&[("README.md", Some("main moved\n"))], false, "main moves");
    let main = world.main_head();
    exits(&world.add("committer", &[]), 0);
    exits(&world.add("rewritten", &[]), 0);
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("FAILED committer at identity: 1 of the"),
        "{shown}"
    );
    assert!(
        shown.contains("committer Someone Else <someone@example.invalid>"),
        "{shown}"
    );
    assert!(shown.contains("FAILED rewritten at decisions"), "{shown}");

    // Its rebase rewrote the committer; the retry still checks the commit as it was.
    world.git(&rewritten, &["rm", "--quiet", "decisions-fail"]);
    world.commit(&rewritten, "the decisions pass");
    exits(&world.wo(&["retry", "rewritten"]), 0);
    let shown = exits(&world.run(), 1);
    assert!(
        shown.contains("FAILED rewritten at identity: 1 of the"),
        "{shown}"
    );
    assert!(
        shown.contains("committer Someone Else <someone@example.invalid>"),
        "{shown}"
    );
    assert_eq!(world.main_head(), main, "nothing merged");
}

#[test]
fn queue_appends_after_a_torn_line() {
    let world = World::new();
    for name in ["one", "two", "three"] {
        world.entry(name, &format!("torn-{name}"));
    }
    exits(&world.add("one", &[]), 0);
    exits(&world.add("two", &[]), 0);
    let tear = |file: &str, bytes: &str| {
        let path = world.state.join(file);
        let mut text = fs::read(&path).unwrap_or_default();
        text.extend_from_slice(bytes.as_bytes());
        fs::write(&path, text).unwrap();
    };
    tear("queue.jsonl", "{\"name\":\"thr");
    assert!(world.status_line("two").contains("state=pending"));
    exits(&world.add("three", &[]), 0);
    tear(
        "ledger.jsonl",
        "{\"name\":\"one\",\"attempt\":1,\"step\":\"ch",
    );
    assert!(world.status_line("one").contains("state=pending"));
    let shown = exits(&world.run(), 0);
    assert!(
        shown.contains("queue done: merged [one, two, three] failed [] skipped []"),
        "{shown}"
    );
    for name in ["one", "two", "three"] {
        assert!(world.status_line(name).contains("state=merged"), "{name}");
    }
    let ledger = world.ledger();
    assert_eq!(ledger.len(), 3 * (STEPS.len() + 1), "every line reads");
    let queue = fs::read_to_string(world.state.join("queue.jsonl")).unwrap();
    assert_eq!(queue.lines().count(), 3, "{queue}");
}

#[test]
fn queue_legacy_lock_of_another_queue() {
    let world = World::new();
    let kit = world.root.join("kit-lock");
    world.edit_config(|config| config["legacy_lock"] = json!(kit.display().to_string()));
    let other_config = world.root.join("other-config.json");
    let mut other: Value =
        serde_json::from_str(&fs::read_to_string(&world.config).unwrap()).unwrap();
    other["state"] = json!(world.root.join("other-state").display().to_string());
    fs::write(&other_config, other.to_string()).unwrap();
    let other_wo = |args: &[&str]| {
        let mut command = Command::new(WO);
        world.env(&mut command);
        command
            .arg("queue")
            .args(args)
            .arg("--config")
            .arg(&other_config)
            .current_dir(&world.root)
            .stdin(Stdio::null())
            .output()
            .unwrap()
    };

    world.entry("held", "held-branch");
    world.set_gate("held", "wait");
    exits(&world.add("held", &[]), 0);
    let mut first = world.spawn(&["run"], &world.root.join("first-run.log"));
    let waiting = world.modes.join("held.waiting");
    wait_for("the held gate", || waiting.exists());
    let lock = world.state.join("queue.lock");
    let owner = fs::read_to_string(kit.join("owner")).unwrap();
    assert_eq!(
        owner.trim_end(),
        format!("wo queue pid {} lock {}", first.id(), lock.display())
    );

    world.entry("elsewhere", "elsewhere-branch");
    exits(&other_wo(&["add", "elsewhere"]), 0);
    let shown = exits(&other_wo(&["run", "--no-wait"]), 3);
    assert!(
        shown.contains(&format!("the kit's lock {} is held", kit.display())),
        "{shown}"
    );
    assert!(kit.join("owner").is_file(), "the live run's lock stays");
    fs::write(world.modes.join("held.release"), "").unwrap();
    assert!(first.wait().unwrap().success());
    assert!(!kit.exists());

    fs::create_dir(&kit).unwrap();
    fs::write(
        kit.join("owner"),
        format!("wo queue pid 999999 lock {}\n", lock.display()),
    )
    .unwrap();
    let shown = exits(&other_wo(&["run", "--no-wait"]), 0);
    assert!(
        shown.contains("left by a stopped run; owner: wo queue pid 999999"),
        "{shown}"
    );
    assert!(shown.contains("MERGED elsewhere as"), "{shown}");
    assert!(!kit.exists());
}
