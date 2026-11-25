//! The `wo` command line: `wo queue add`, `run`, `status` and `retry`.

use crate::config::{self, Config};
use crate::git::Git;
use crate::lock;
use crate::message::Message;
use crate::order_text;
use crate::process::Runner;
use crate::store::{self, LedgerLine, QueueEntry, RETRY, State, View};
use crate::time;
use crate::worktree;
use crate::{EXIT_OK, EXIT_USAGE};
use std::fs::{self, File};
use std::path::PathBuf;

/// The usage `wo` prints.
pub const USAGE: &str = "\
usage: wo queue add NAME [--onto COMMIT] [--after NAME]... --config FILE
       wo queue run [--no-wait] --config FILE
       wo queue status --config FILE
       wo queue retry NAME --config FILE";

/// Runs `wo` with its arguments, less the program's name; returns the exit status.
pub fn main(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("queue") => queue(&args[1..]),
        Some("--help" | "-h" | "help") => {
            println!("{USAGE}");
            EXIT_OK
        }
        _ => usage("the first argument is not `queue`"),
    }
}

fn usage(why: &str) -> i32 {
    eprintln!("wo: {why}\n{USAGE}");
    EXIT_USAGE
}

fn refuse(command: &str, why: &str) -> i32 {
    eprintln!("wo queue {command}: refused: {why}");
    EXIT_USAGE
}

/// The arguments of one `wo queue` command.
#[derive(Debug, Default)]
struct Args {
    config: Option<PathBuf>,
    name: Option<String>,
    onto: Option<String>,
    after: Vec<String>,
    no_wait: bool,
}

fn parse(command: &str, rest: &[String]) -> Result<Args, String> {
    let mut args = Args::default();
    let mut items = rest.iter();
    while let Some(item) = items.next() {
        let mut value = |flag: &str| items.next().cloned().ok_or(format!("{flag} needs a value"));
        match item.as_str() {
            "--config" => args.config = Some(PathBuf::from(value("--config")?)),
            "--onto" if command == "add" => {
                if args.onto.is_some() {
                    return Err("--onto is given twice".into());
                }
                args.onto = Some(value("--onto")?);
            }
            "--after" if command == "add" => args.after.push(value("--after")?),
            "--no-wait" if command == "run" => args.no_wait = true,
            flag if flag.starts_with('-') => {
                return Err(format!("wo queue {command} takes no {flag}"));
            }
            name if args.name.is_none() && matches!(command, "add" | "retry") => {
                args.name = Some(name.to_string());
            }
            other => return Err(format!("wo queue {command} takes no argument {other:?}")),
        }
    }
    if args.config.is_none() {
        return Err("--config FILE is required".into());
    }
    if matches!(command, "add" | "retry") && args.name.is_none() {
        return Err(format!("wo queue {command} needs a NAME"));
    }
    Ok(args)
}

fn queue(rest: &[String]) -> i32 {
    let Some(command) = rest.first().map(String::as_str) else {
        return usage("wo queue needs a command");
    };
    if !matches!(command, "add" | "run" | "status" | "retry") {
        return usage(&format!("wo queue has no command {command:?}"));
    }
    let args = match parse(command, &rest[1..]) {
        Ok(args) => args,
        Err(why) => return usage(&why),
    };
    let config = match config::load(args.config.as_deref().expect("parse requires it")) {
        Ok(config) => config,
        Err(why) => {
            eprintln!("wo queue {command}: {why}");
            return EXIT_USAGE;
        }
    };
    match command {
        "add" => add(&config, &args),
        "run" => crate::run::run(&config, args.no_wait),
        "status" => status(&config),
        _ => retry(&config, args.name.as_deref().expect("parse requires it")),
    }
}

fn add(config: &Config, args: &Args) -> i32 {
    let name = args.name.as_deref().expect("parse requires it");
    match add_entry(config, name, args) {
        Ok(entry) => {
            println!(
                "added {name}: branch {}, onto {}, after [{}]",
                entry.branch,
                entry.onto.as_deref().unwrap_or("none"),
                entry.after.join(", ")
            );
            EXIT_OK
        }
        Err(why) => refuse("add", &why),
    }
}

fn add_entry(config: &Config, name: &str, args: &Args) -> Result<QueueEntry, String> {
    if !worktree::valid_name(name) {
        return Err(format!(
            "{name:?} cannot name an entry: letters, digits, `.`, `_` and `-`, beginning with a letter or digit"
        ));
    }
    let path = config.queue_file();
    known(&store::read_queue(&path)?, name, &args.after)?;
    let runner = Runner::new(&config.identity);
    if let Some(onto) = &args.onto {
        let full = matches!(onto.len(), 40 | 64)
            && onto
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase());
        let known = full && Git::new(&runner, &config.clone).commit(onto).as_deref() == Ok(onto);
        if !known {
            return Err(format!(
                "--onto {onto} is not a full commit ID of a commit in the clone"
            ));
        }
    }
    Message::read(&config.message_file(name), &config.trailers)?;
    let located = worktree::locate(config, name)?;
    let git = Git::new(&runner, &located.dir);
    let checked = worktree::check(&located, &git)?;
    if let Some(onto) = &args.onto
        && git.is_ancestor(onto, &checked.head) != Some(true)
    {
        return Err(format!(
            "--onto {onto} is not an ancestor of {}'s head {}",
            checked.branch, checked.head
        ));
    }
    order_text::check_record(&git, &checked.branch, "HEAD")?;
    fs::create_dir_all(&config.state)
        .map_err(|error| format!("{} cannot be made: {error}", config.state.display()))?;
    let mut file = store::open_append(&path)?;
    file.lock()
        .map_err(|error| format!("{} cannot be locked: {error}", path.display()))?;
    known(&store::read_queue(&path)?, name, &args.after)?;
    let entry = QueueEntry {
        name: name.to_string(),
        branch: checked.branch,
        onto: args.onto.clone(),
        after: args.after.clone(),
        added: time::now(),
    };
    store::append_to(&mut file, &path, &entry.to_json())?;
    Ok(entry)
}

/// Refuses a name already added and an `--after` no entry has.
fn known(entries: &[QueueEntry], name: &str, after: &[String]) -> Result<(), String> {
    if entries.iter().any(|entry| entry.name == name) {
        return Err(format!("{name} is already in the queue"));
    }
    for after in after {
        if !entries.iter().any(|entry| &entry.name == after) {
            return Err(format!(
                "--after {after}: no entry of that name is in the queue"
            ));
        }
    }
    Ok(())
}

fn status(config: &Config) -> i32 {
    let queue = match store::read_queue(&config.queue_file()) {
        Ok(queue) => queue,
        Err(why) => return refuse("status", &why),
    };
    let ledger = match store::read_ledger(&config.ledger_file()) {
        Ok(ledger) => ledger,
        Err(why) => return refuse("status", &why),
    };
    let held = lock::queue_lock_held(&config.lock_file());
    let runner = Runner::new(&config.identity);
    let git = Git::new(&runner, &config.clone);
    let base_ref = config.base_ref();
    let base = config.base_name();
    if queue.is_empty() {
        println!("the queue is empty");
    }
    for item in &queue {
        let view = View::of(&item.name, &ledger);
        let state = view.state(held);
        let step = view
            .last_step()
            .map(|line| line.step.clone())
            .unwrap_or_else(|| "-".into());
        let end = view.end.as_ref();
        let pr = end
            .and_then(|line| line.pr)
            .map(|pr| pr.to_string())
            .unwrap_or_else(|| "-".into());
        let commit = end.and_then(|line| line.commit.clone());
        let record = view.gate_record().unwrap_or_else(|| "-".into());
        let holds = if state == State::Merged {
            let shown = match commit
                .as_deref()
                .and_then(|commit| git.is_ancestor(commit, &base_ref))
            {
                Some(true) => "yes",
                Some(false) => "no",
                None => "unknown",
            };
            format!("{base}_holds_commit={shown}")
        } else {
            let head = git.commit(&format!("refs/heads/{}", item.branch)).ok();
            let shown = match head
                .as_deref()
                .and_then(|head| git.is_ancestor(head, &base_ref))
            {
                Some(true) => "yes",
                Some(false) => "no",
                None => "unknown",
            };
            format!("{base}_holds_head={shown}")
        };
        println!(
            "{} branch={} state={} attempt={} step={step} pr={pr} commit={} gate_record={record} {holds}",
            item.name,
            item.branch,
            state.word(),
            view.attempt,
            commit.as_deref().unwrap_or("-"),
        );
    }
    EXIT_OK
}

fn retry(config: &Config, name: &str) -> i32 {
    match retry_entry(config, name) {
        Ok(attempt) => {
            println!("retry {name}: attempt {attempt} opened");
            EXIT_OK
        }
        Err(why) => refuse("retry", &why),
    }
}

fn retry_entry(config: &Config, name: &str) -> Result<u64, String> {
    let queue = store::read_queue(&config.queue_file())?;
    if !queue.iter().any(|entry| entry.name == name) {
        return Err(format!("{name} is not in the queue"));
    }
    let path = config.ledger_file();
    retryable(config, name, &store::read_ledger(&path)?)?;
    let mut file: File = store::open_append(&path)?;
    file.lock()
        .map_err(|error| format!("{} cannot be locked: {error}", path.display()))?;
    let view = retryable(config, name, &store::read_ledger(&path)?)?;
    let now = time::now();
    let line = LedgerLine {
        name: name.to_string(),
        attempt: view.attempt + 1,
        step: RETRY.to_string(),
        outcome: "ok".to_string(),
        started: now.clone(),
        ended: now,
        why: format!("wo queue retry opened attempt {}", view.attempt + 1),
        pinned: None,
        head: None,
        backup: None,
        gate_record: None,
        pr: None,
        commit: None,
    };
    store::append_to(&mut file, &path, &line.to_json())?;
    Ok(view.attempt + 1)
}

/// The current attempt of an entry `retry` may follow: one that ended and did not merge.
fn retryable(config: &Config, name: &str, ledger: &[LedgerLine]) -> Result<View, String> {
    let view = View::of(name, ledger);
    match view.end.as_ref().map(|end| end.outcome.as_str()) {
        None => Err(format!(
            "{name}'s attempt {} has no end line: it is {}",
            view.attempt,
            view.state(lock::queue_lock_held(&config.lock_file()))
                .word()
        )),
        Some("merged") => Err(format!("{name} merged in attempt {}", view.attempt)),
        Some(_) => Ok(view),
    }
}
