//! `queue-fixture`: plays GitHub's CLI on a bare repository, the gate and the repository's
//! Python tools, for `tests/queue.rs`.
//!
//! - `queue-fixture gh ORIGIN pr list|create|merge ...`: pull requests kept in
//!   `ORIGIN/fixture-prs.json`; `pr merge` checks `--match-head-commit` against the branch's
//!   head on ORIGIN and writes the squash onto its base with `git commit-tree` and
//!   `git update-ref`.
//! - `queue-fixture gate --base PINNED`: the mode is the file `$QUEUE_FIXTURE_GATE_MODES/NAME`,
//!   NAME being the worktree's directory name, else `pass`: `pass`, `fail`, `move-main` (which
//!   pushes a commit to `$QUEUE_FIXTURE_ORIGIN`'s `main`, then passes) or `wait` (which waits
//!   for `NAME.release` beside the mode, then passes). Each writes `target/gate/HEAD.json` and
//!   ends as the gate's last line does.
//! - `queue-fixture python [--messages DIR] tools/promise_standing.py --write`: writes
//!   `engine/STANDING.md` from the `spec/` paths and their line counts, and removes
//!   `DIR/NAME.txt` where the tree holds `drop-message`.
//! - `queue-fixture python [--messages DIR] tools/check_decisions.py BEFORE AFTER`: ends
//!   `Errors: 1` where the tree holds `decisions-fail`, else `Errors: 0`.

use serde_json::{Value, json};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, exit};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let code = match args.first().map(String::as_str) {
        Some("gh") => gh(&args[1..]),
        Some("gate") => gate(&args[1..]),
        Some("python") => python(&args[1..]),
        _ => {
            eprintln!("queue-fixture: the first argument is gh, gate or python");
            2
        }
    };
    exit(code);
}

fn git(
    dir: Option<&Path>,
    git_dir: Option<&Path>,
    args: &[&str],
    input: Option<&str>,
) -> Result<String, String> {
    let mut command = Command::new("git");
    command.args([
        "-c",
        "core.hooksPath=/dev/null",
        "-c",
        "commit.gpgsign=false",
    ]);
    if let Some(git_dir) = git_dir {
        command.arg("--git-dir").arg(git_dir);
    }
    command
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(dir) = dir {
        command.current_dir(dir);
    }
    command.stdin(if input.is_some() {
        Stdio::piped()
    } else {
        Stdio::null()
    });
    let mut child = command.spawn().map_err(|error| format!("git: {error}"))?;
    if let Some(text) = input {
        child
            .stdin
            .take()
            .expect("piped")
            .write_all(text.as_bytes())
            .map_err(|error| format!("git: {error}"))?;
    }
    let output = child
        .wait_with_output()
        .map_err(|error| format!("git: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|arg| arg == name)
        .and_then(|index| args.get(index + 1))
        .map(String::as_str)
}

fn gh(args: &[String]) -> i32 {
    let Some(origin) = args.first().map(PathBuf::from) else {
        eprintln!("queue-fixture gh: ORIGIN is missing");
        return 2;
    };
    match gh_inner(&origin, &args[1..]) {
        Ok(()) => 0,
        Err(why) => {
            eprintln!("queue-fixture gh: {why}");
            1
        }
    }
}

fn store_path(origin: &Path) -> PathBuf {
    origin.join("fixture-prs.json")
}

fn read_store(origin: &Path) -> Value {
    fs::read_to_string(store_path(origin))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_else(|| json!({"next": 1, "prs": []}))
}

fn write_store(origin: &Path, store: &Value) -> Result<(), String> {
    fs::write(store_path(origin), store.to_string()).map_err(|error| error.to_string())
}

fn gh_inner(origin: &Path, args: &[String]) -> Result<(), String> {
    let words: Vec<&str> = args.iter().take(2).map(String::as_str).collect();
    let mut store = read_store(origin);
    match words[..] {
        ["pr", "list"] => {
            let head = flag(args, "--head").ok_or("--head is missing")?;
            let open: Vec<Value> = store["prs"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|pr| pr["head"] == head && pr["state"] == "open")
                .map(|pr| json!({"number": pr["number"]}))
                .collect();
            println!("{}", Value::Array(open));
            Ok(())
        }
        ["pr", "create"] => {
            let number = store["next"].as_u64().unwrap_or(1);
            let pr = json!({
                "number": number,
                "base": flag(args, "--base").ok_or("--base is missing")?,
                "head": flag(args, "--head").ok_or("--head is missing")?,
                "title": flag(args, "--title").ok_or("--title is missing")?,
                "body": flag(args, "--body").ok_or("--body is missing")?,
                "state": "open",
            });
            store["prs"]
                .as_array_mut()
                .ok_or("the store is malformed")?
                .push(pr);
            store["next"] = json!(number + 1);
            write_store(origin, &store)?;
            println!("https://github.invalid/example/repository/pull/{number}");
            Ok(())
        }
        ["pr", "merge"] => {
            let number: u64 = args
                .get(2)
                .and_then(|text| text.parse().ok())
                .ok_or("the pull request's number is missing")?;
            if !args.iter().any(|arg| arg == "--squash") {
                return Err("only --squash is played".into());
            }
            let expected =
                flag(args, "--match-head-commit").ok_or("--match-head-commit is missing")?;
            let subject = flag(args, "--subject").ok_or("--subject is missing")?;
            let body = flag(args, "--body").ok_or("--body is missing")?;
            let prs = store["prs"]
                .as_array_mut()
                .ok_or("the store is malformed")?;
            let pr = prs
                .iter_mut()
                .find(|pr| pr["number"] == number && pr["state"] == "open")
                .ok_or(format!("no open pull request {number}"))?;
            let head_branch = pr["head"].as_str().unwrap_or_default().to_string();
            let base = pr["base"].as_str().unwrap_or_default().to_string();
            let head = git(
                None,
                Some(origin),
                &[
                    "rev-parse",
                    "--verify",
                    &format!("refs/heads/{head_branch}"),
                ],
                None,
            )?;
            if head != expected {
                return Err(format!(
                    "the head of {head_branch} is {head}, not {expected}"
                ));
            }
            let tree = git(
                None,
                Some(origin),
                &["rev-parse", &format!("{head}^{{tree}}")],
                None,
            )?;
            let main = git(
                None,
                Some(origin),
                &["rev-parse", "--verify", &format!("refs/heads/{base}")],
                None,
            )?;
            let message = format!("{subject}\n\n{body}\n");
            let squash = git(
                None,
                Some(origin),
                &["commit-tree", &tree, "-p", &main],
                Some(&message),
            )?;
            git(
                None,
                Some(origin),
                &["update-ref", &format!("refs/heads/{base}"), &squash, &main],
                None,
            )?;
            pr["state"] = json!("merged");
            pr["merge_commit"] = json!(squash);
            write_store(origin, &store)?;
            println!("merged pull request {number} as {squash}");
            Ok(())
        }
        _ => Err(format!("{} is not played", args.join(" "))),
    }
}

fn gate(args: &[String]) -> i32 {
    match gate_inner(args) {
        Ok(code) => code,
        Err(why) => {
            println!("FAIL queue-fixture gate: {why}");
            println!("Gate: FAIL. The fixture could not run.");
            1
        }
    }
}

fn gate_inner(args: &[String]) -> Result<i32, String> {
    let base = flag(args, "--base").ok_or("--base is missing")?.to_string();
    let cwd = env::current_dir().map_err(|error| error.to_string())?;
    let name = cwd
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("the worktree has no name")?
        .to_string();
    let modes = env::var_os("QUEUE_FIXTURE_GATE_MODES").map(PathBuf::from);
    let mode = modes
        .as_ref()
        .and_then(|dir| fs::read_to_string(dir.join(&name)).ok())
        .map(|text| text.trim().to_string())
        .unwrap_or_else(|| "pass".into());
    let head = git(Some(&cwd), None, &["rev-parse", "HEAD"], None)?;
    match mode.as_str() {
        "move-main" => {
            let origin =
                env::var_os("QUEUE_FIXTURE_ORIGIN").ok_or("QUEUE_FIXTURE_ORIGIN is not set")?;
            let tree = git(
                Some(&cwd),
                None,
                &["rev-parse", &format!("{base}^{{tree}}")],
                None,
            )?;
            let moved = git(
                Some(&cwd),
                None,
                &["commit-tree", &tree, "-p", &base],
                Some("main moved during the gate\n"),
            )?;
            let origin = origin.to_string_lossy().into_owned();
            git(
                Some(&cwd),
                None,
                &[
                    "push",
                    "--quiet",
                    &origin,
                    &format!("{moved}:refs/heads/main"),
                ],
                None,
            )?;
        }
        "wait" => {
            let dir = modes.ok_or("QUEUE_FIXTURE_GATE_MODES is not set")?;
            fs::write(
                dir.join(format!("{name}.waiting")),
                format!("{}\n", std::process::id()),
            )
            .map_err(|error| error.to_string())?;
            let deadline = Instant::now() + Duration::from_secs(120);
            while !dir.join(format!("{name}.release")).exists() {
                if Instant::now() > deadline {
                    return Err("no release within 120 s".into());
                }
                thread::sleep(Duration::from_millis(20));
            }
        }
        "pass" | "fail" => {}
        other => return Err(format!("no mode {other}")),
    }
    let record = format!("target/gate/{head}.json");
    fs::create_dir_all(cwd.join("target/gate")).map_err(|error| error.to_string())?;
    let body = json!({
        "head": head,
        "base": base,
        "mode": mode,
        "cache": env::var("QUEUE_FIXTURE_CACHE").ok(),
    });
    fs::write(cwd.join(&record), body.to_string()).map_err(|error| error.to_string())?;
    let code = if mode == "fail" {
        println!("FAIL fixture row: the gate's fixture was set to fail");
        println!(
            "Gate: FAIL. Failed: fixture row; required, not run: none; 0 passed. Record: {record}"
        );
        1
    } else {
        println!("Gate: PASS. 1 passed; not run: none. Record: {record}");
        0
    };
    if mode == "wait" {
        let dir = env::var_os("QUEUE_FIXTURE_GATE_MODES")
            .map(PathBuf::from)
            .unwrap_or_default();
        let _ = fs::remove_file(dir.join(format!("{name}.waiting")));
    }
    Ok(code)
}

fn python(args: &[String]) -> i32 {
    let mut rest = args;
    let mut messages = None;
    if rest.first().map(String::as_str) == Some("--messages") {
        messages = rest.get(1).map(PathBuf::from);
        rest = rest.get(2..).unwrap_or_default();
    }
    let cwd = env::current_dir().expect("a working directory");
    let script: Vec<&str> = rest.iter().map(String::as_str).collect();
    match script[..] {
        ["tools/promise_standing.py", "--write"] => {
            if let Err(why) = write_standing(&cwd) {
                println!("error: {why}");
                println!("Errors: 1");
                return 1;
            }
            if cwd.join("drop-message").exists()
                && let (Some(dir), Some(name)) = (&messages, cwd.file_name())
            {
                let _ = fs::remove_file(dir.join(format!("{}.txt", name.to_string_lossy())));
            }
            println!("Wrote engine/STANDING.md.");
            println!("Errors: 0");
            0
        }
        ["tools/check_decisions.py", _, _] => {
            if cwd.join("decisions-fail").exists() {
                println!("error: docs/decisions/log.md: fixture: the tree holds decisions-fail");
                println!("Errors: 1");
                1
            } else {
                println!("Errors: 0");
                0
            }
        }
        _ => {
            println!(
                "error: queue-fixture python does not play {}",
                rest.join(" ")
            );
            println!("Errors: 1");
            2
        }
    }
}

/// The fixture's STANDING: a heading, then each `spec/` path and its line count, sorted.
fn write_standing(root: &Path) -> Result<(), String> {
    let mut files = Vec::new();
    walk(root, &root.join("spec"), &mut files)?;
    files.sort();
    let mut text = String::from("# STANDING, as the queue's fixture writes it\n\n");
    for (path, lines) in files {
        text.push_str(&format!("{path} {lines}\n"));
    }
    fs::create_dir_all(root.join("engine")).map_err(|error| error.to_string())?;
    fs::write(root.join("engine/STANDING.md"), text).map_err(|error| error.to_string())
}

fn walk(root: &Path, dir: &Path, files: &mut Vec<(String, usize)>) -> Result<(), String> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(root, &path, files)?;
        } else {
            let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
            let relative = path
                .strip_prefix(root)
                .map_err(|error| error.to_string())?
                .to_string_lossy()
                .replace('\\', "/");
            files.push((relative, text.lines().count()));
        }
    }
    Ok(())
}
