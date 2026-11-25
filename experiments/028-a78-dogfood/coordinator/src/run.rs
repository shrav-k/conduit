//! `wo queue run`: one runner at a time, entries in the file's order (decision 4).

use crate::config::Config;
use crate::entry::{self, Outcome};
use crate::lock;
use crate::process::Runner;
use crate::store::{self, State, View};
use crate::{EXIT_FAILED, EXIT_LOCKED, EXIT_OK, EXIT_USAGE};
use std::fs;

/// The why of an attempt a run left with step lines and no `end` line.
pub const STOPPED: &str = "the run that held it stopped";

/// The step a stopped attempt had reached: the one after its last step line that passed.
fn reached(last: Option<&store::LedgerLine>) -> &str {
    match last {
        Some(line) if line.outcome == "ok" => entry::STEPS
            .iter()
            .position(|step| *step == line.step)
            .and_then(|index| entry::STEPS.get(index + 1))
            .copied()
            .unwrap_or(line.step.as_str()),
        Some(line) => line.step.as_str(),
        None => entry::STEPS[0],
    }
}

/// Runs every pending entry; returns the exit status.
pub fn run(config: &Config, no_wait: bool) -> i32 {
    match run_inner(config, no_wait) {
        Ok(status) => status,
        Err(error) => {
            eprintln!("wo queue run: {error}");
            EXIT_USAGE
        }
    }
}

fn run_inner(config: &Config, no_wait: bool) -> Result<i32, String> {
    fs::create_dir_all(&config.state)
        .map_err(|error| format!("{} cannot be made: {error}", config.state.display()))?;
    let Some(_queue_lock) = lock::queue_lock(&config.lock_file(), no_wait)? else {
        return Ok(EXIT_LOCKED);
    };
    let _legacy = match &config.legacy_lock {
        None => None,
        Some(dir) => match lock::legacy_lock(dir, &config.lock_file(), no_wait)? {
            Some(held) => Some(held),
            None => return Ok(EXIT_LOCKED),
        },
    };
    let runner = Runner::new(&config.identity);
    let mut merged = Vec::new();
    let mut failed = Vec::new();
    let mut skipped = Vec::new();

    let queue = store::read_queue(&config.queue_file())?;
    let ledger = store::read_ledger(&config.ledger_file())?;
    for item in &queue {
        let view = View::of(&item.name, &ledger);
        if view.state(false) == State::Stopped {
            let last = view.last_step();
            entry::end_without_steps(config, &item.name, view.attempt, "failed", STOPPED, last)?;
            println!("FAILED {} at {}: {STOPPED}", item.name, reached(last));
            failed.push(item.name.clone());
        }
    }

    loop {
        let queue = store::read_queue(&config.queue_file())?;
        let ledger = store::read_ledger(&config.ledger_file())?;
        let Some(item) = queue
            .iter()
            .find(|item| View::of(&item.name, &ledger).state(false) == State::Pending)
        else {
            break;
        };
        let attempt = View::of(&item.name, &ledger).attempt;
        let unmerged = item
            .after
            .iter()
            .find(|after| View::of(after, &ledger).state(false) != State::Merged);
        if let Some(after) = unmerged {
            let why = format!("{after} did not merge");
            entry::end_without_steps(config, &item.name, attempt, "skipped", &why, None)?;
            println!("SKIPPED {}: {why}", item.name);
            skipped.push(item.name.clone());
            continue;
        }
        match entry::run_attempt(config, &runner, item, attempt)? {
            Outcome::Merged => merged.push(item.name.clone()),
            Outcome::Failed => failed.push(item.name.clone()),
        }
    }
    println!(
        "queue done: merged [{}] failed [{}] skipped [{}]",
        merged.join(", "),
        failed.join(", "),
        skipped.join(", ")
    );
    Ok(if failed.is_empty() && skipped.is_empty() {
        EXIT_OK
    } else {
        EXIT_FAILED
    })
}
