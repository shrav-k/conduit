//! One attempt at one entry, step by step, to the first failure (decision 5).

use crate::changelog;
use crate::config::Config;
use crate::git::Git;
use crate::merge::{CHANGES, FRAGMENTS, FRAGMENTS_README};
use crate::message::Message;
use crate::order_text;
use crate::process::Runner;
use crate::rebase::{self, STANDING};
use crate::store::{self, END, LedgerLine, QueueEntry};
use crate::time;
use crate::worktree::{self, Worktree};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::{self, ErrorKind};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// The steps, in order; each name is the ledger's `step`.
pub const STEPS: &[&str] = &[
    "check",
    "fetch",
    "backup",
    "rebase",
    "standing",
    "fragment",
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

/// The subject of the commit the standing step makes.
pub const STANDING_SUBJECT: &str = "wip: STANDING regenerated on main";

/// How an attempt ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Merged,
    Failed,
}

/// Why a step failed: one line, then lines that name each thing.
#[derive(Debug, Clone)]
pub struct Failure {
    pub why: String,
    pub details: Vec<String>,
}

impl Failure {
    fn new(why: impl Into<String>) -> Self {
        Self {
            why: why.into(),
            details: Vec::new(),
        }
    }

    fn with(why: impl Into<String>, details: Vec<String>) -> Self {
        Self {
            why: why.into(),
            details,
        }
    }

    /// The failure as the ledger's one-line `why`.
    pub fn line(&self) -> String {
        let mut line = self.why.clone();
        for detail in &self.details {
            line.push_str("; ");
            line.push_str(detail);
        }
        line.replace(['\n', '\r'], " ")
    }
}

impl From<String> for Failure {
    fn from(why: String) -> Self {
        Self::new(why)
    }
}

type Step = Result<String, Failure>;

/// The state of one attempt as its steps run.
pub struct Attempt<'a> {
    config: &'a Config,
    runner: &'a Runner,
    entry: &'a QueueEntry,
    attempt: u64,
    started: String,
    worktree: Option<Worktree>,
    message: Option<Message>,
    pinned: Option<String>,
    head: Option<String>,
    backup: Option<String>,
    backup_commit: Option<String>,
    gate_record: Option<String>,
    pr: Option<u64>,
    merged: Option<(String, String)>,
}

/// Writes an attempt's `end` line with no step run, as a skipped entry or a stopped attempt
/// gets.
pub fn end_without_steps(
    config: &Config,
    name: &str,
    attempt: u64,
    outcome: &str,
    why: &str,
    last: Option<&LedgerLine>,
) -> Result<(), String> {
    let now = time::now();
    let line = LedgerLine {
        name: name.to_string(),
        attempt,
        step: END.to_string(),
        outcome: outcome.to_string(),
        started: now.clone(),
        ended: now,
        why: why.to_string(),
        pinned: last.and_then(|line| line.pinned.clone()),
        head: last.and_then(|line| line.head.clone()),
        backup: last.and_then(|line| line.backup.clone()),
        gate_record: last.and_then(|line| line.gate_record.clone()),
        pr: None,
        commit: None,
    };
    store::append(&config.ledger_file(), &line.to_json())
}

/// Runs one attempt at `entry`; `Err` only when the ledger cannot be written.
pub fn run_attempt(
    config: &Config,
    runner: &Runner,
    entry: &QueueEntry,
    attempt: u64,
) -> Result<Outcome, String> {
    let mut state = Attempt {
        config,
        runner,
        entry,
        attempt,
        started: time::now(),
        worktree: None,
        message: None,
        pinned: None,
        head: None,
        backup: None,
        backup_commit: None,
        gate_record: None,
        pr: None,
        merged: None,
    };
    let name = &entry.name;
    for step in STEPS {
        println!("{name} {step}");
        let started = time::now();
        let result = state.step(step);
        let ended = time::now();
        match result {
            Ok(why) => state.write(step, "ok", started, ended, &why)?,
            Err(failure) => {
                state.write(step, "failed", started, ended, &failure.line())?;
                if *step == "tidy" {
                    println!("{name} tidy: {}", failure.line());
                    break;
                }
                println!("FAILED {name} at {step}: {}", failure.why);
                for detail in &failure.details {
                    println!("  {detail}");
                }
                state.end("failed", &format!("at {step}: {}", failure.line()))?;
                return Ok(Outcome::Failed);
            }
        }
    }
    let (commit, subject) = state.merged.clone().unwrap_or_default();
    state.end("merged", &format!("merged as {commit} {subject}"))?;
    println!("MERGED {name} as {commit} {subject}");
    Ok(Outcome::Merged)
}

impl<'a> Attempt<'a> {
    fn line(
        &self,
        step: &str,
        outcome: &str,
        started: String,
        ended: String,
        why: &str,
    ) -> LedgerLine {
        LedgerLine {
            name: self.entry.name.clone(),
            attempt: self.attempt,
            step: step.to_string(),
            outcome: outcome.to_string(),
            started,
            ended,
            why: why.replace(['\n', '\r'], " "),
            pinned: self.pinned.clone(),
            head: self.head.clone(),
            backup: self.backup.clone(),
            gate_record: self.gate_record.clone(),
            pr: None,
            commit: None,
        }
    }

    fn write(
        &self,
        step: &str,
        outcome: &str,
        started: String,
        ended: String,
        why: &str,
    ) -> Result<(), String> {
        let line = self.line(step, outcome, started, ended, why);
        store::append(&self.config.ledger_file(), &line.to_json())
    }

    fn end(&self, outcome: &str, why: &str) -> Result<(), String> {
        let mut line = self.line(END, outcome, self.started.clone(), time::now(), why);
        if outcome == "merged" {
            line.pr = self.pr;
            line.commit = self.merged.as_ref().map(|(commit, _)| commit.clone());
        }
        store::append(&self.config.ledger_file(), &line.to_json())
    }

    fn step(&mut self, step: &str) -> Step {
        match step {
            "check" => self.check(),
            "fetch" => self.fetch(),
            "backup" => self.backup(),
            "rebase" => self.rebase(),
            "standing" => self.standing(),
            "fragment" => self.fragment(),
            "decisions" => self.decisions(),
            "identity" => self.identity(),
            "record" => self.record(),
            "squash" => self.squash(),
            "gate" => self.gate(),
            "push" => self.push(),
            "pull-request" => self.pull_request(),
            "main-unmoved" => self.main_unmoved(),
            "merge" => self.merge(),
            "tidy" => self.tidy(),
            other => Err(Failure::new(format!("no step is named {other}"))),
        }
    }

    fn dir(&self) -> &Path {
        &self
            .worktree
            .as_ref()
            .expect("check located the worktree")
            .dir
    }

    fn git(&self) -> Git<'a> {
        Git::new(self.runner, self.dir())
    }

    fn pinned(&self) -> &str {
        self.pinned.as_deref().expect("fetch pinned the base")
    }

    fn message(&self) -> &Message {
        self.message.as_ref().expect("check read the message")
    }

    fn python(&self, args: &[&str]) -> Vec<String> {
        let mut argv = self.config.python.clone();
        argv.extend(args.iter().map(|arg| arg.to_string()));
        argv
    }

    fn gh(&self, args: &[&str]) -> Result<crate::process::Finished, String> {
        let mut argv = self.config.gh.clone();
        argv.extend(args.iter().map(|arg| arg.to_string()));
        self.runner.run(&argv, self.dir(), &BTreeMap::new(), None)
    }

    /// The PINNED of each earlier attempt of this entry whose rebase passed, oldest first.
    fn earlier_rebased(&self) -> Result<Vec<String>, String> {
        Ok(store::read_ledger(&self.config.ledger_file())?
            .into_iter()
            .filter(|line| {
                line.name == self.entry.name
                    && line.attempt < self.attempt
                    && line.step == "rebase"
                    && line.outcome == "ok"
            })
            .filter_map(|line| line.pinned)
            .collect())
    }

    /// The commit below the entry's own commits in `head`, which a rebase replays from. Without
    /// `--onto` it is PINNED. With it, it is the newest of ONTO, the PINNED of each earlier attempt
    /// whose rebase passed, and PINNED that is an ancestor of `head`. Once an attempt has rebased
    /// the branch, ONTO is no longer one, and ONTO..HEAD would replay `main`'s commits as the
    /// entry's own; with none, the entry's own commits cannot be told from others.
    fn upstream_of(&self, git: &Git, head: &str) -> Result<String, Failure> {
        let pinned = self.pinned().to_string();
        let Some(onto) = &self.entry.onto else {
            return Ok(pinned);
        };
        let mut candidates = vec![onto.clone()];
        candidates.extend(self.earlier_rebased()?);
        candidates.push(pinned.clone());
        let mut newest: Option<String> = None;
        for candidate in candidates {
            if git.is_ancestor(&candidate, head) != Some(true) {
                continue;
            }
            newest = match newest {
                Some(best) if git.is_ancestor(&best, &candidate) != Some(true) => Some(best),
                _ => Some(candidate),
            };
        }
        newest.ok_or_else(|| {
            Failure::new(format!(
                "--onto {onto} is not an ancestor of HEAD {head}, nor is PINNED {pinned} or the PINNED of an earlier attempt whose rebase passed: the entry's own commits cannot be told from others"
            ))
        })
    }

    fn refresh_head(&mut self) -> Result<String, String> {
        let head = self.git().commit("HEAD")?;
        self.head = Some(head.clone());
        Ok(head)
    }

    fn check(&mut self) -> Step {
        let config = self.config;
        let name = &self.entry.name;
        let message = Message::read(&config.message_file(name), &config.trailers)?;
        let worktree = worktree::locate(config, name)?;
        let checked = worktree::check(&worktree, &Git::new(self.runner, &worktree.dir))?;
        if checked.branch != self.entry.branch {
            return Err(Failure::new(format!(
                "the worktree is on {}, not the entry's branch {}",
                checked.branch, self.entry.branch
            )));
        }
        self.head = Some(checked.head.clone());
        self.message = Some(message);
        self.worktree = Some(worktree);
        Ok(format!(
            "the message is well formed; the worktree is clean on {} at {}",
            checked.branch, checked.head
        ))
    }

    fn fetch(&mut self) -> Step {
        let config = self.config;
        self.git().text(&["fetch", &config.remote])?;
        let pinned = self.git().commit(&config.base_ref())?;
        self.pinned = Some(pinned.clone());
        Ok(format!("{} is {pinned}, pinned", config.base_name()))
    }

    fn backup(&mut self) -> Step {
        let name = format!("refs/wo-backup/{}/{}", self.entry.name, self.attempt);
        let git = self.git();
        let head = git.commit("HEAD")?;
        if Some(&head) != self.head.as_ref() {
            return Err(Failure::new(format!(
                "HEAD moved since check: it is {head}"
            )));
        }
        if let Ok(there) = git.text(&["rev-parse", "--verify", "--quiet", &name]) {
            return Err(Failure::new(format!(
                "{name} already names {there}; a backup is never written over"
            )));
        }
        git.text(&["update-ref", "-m", "wo queue: backup", &name, &head, ""])?;
        self.backup = Some(name.clone());
        self.backup_commit = Some(head.clone());
        Ok(format!("{name} names {head}"))
    }

    fn rebase(&mut self) -> Step {
        let pinned = self.pinned().to_string();
        let onto = self.entry.onto.clone();
        let git = self.git();
        let backup = self.backup_commit.clone().expect("backup named HEAD");
        let from = self.upstream_of(&git, &backup)?;
        let range = format!("{from}..HEAD");
        let merges = git.text(&["rev-list", "--merges", &range])?;
        if let Some(first) = merges.lines().next() {
            return Err(Failure::new(format!(
                "{range} holds a merge commit, {first}: such a branch is never rebased"
            )));
        }
        let count: usize = git
            .text(&["rev-list", "--count", &range])?
            .parse()
            .unwrap_or(0);
        let mut args = vec!["-c", "merge.conflictstyle=diff3", "rebase"];
        match &onto {
            Some(_) => args.extend(["--onto", pinned.as_str(), from.as_str()]),
            None => args.push(pinned.as_str()),
        }
        let mut finished = git.run(&args)?;
        let mut settled = Vec::new();
        let mut stops = 0;
        loop {
            let stopped =
                worktree::in_progress(self.worktree.as_ref().expect("located")) == Some("a rebase");
            if !stopped {
                if finished.success() {
                    break;
                }
                return Err(self.after_rebase_error(&finished));
            }
            stops += 1;
            if stops > count + 1 {
                return Err(self.abort(vec![format!(
                    "the rebase stopped {stops} times for {count} commits"
                )]));
            }
            let own = match rebase::own_fragment_commit(&git) {
                Ok(own) => own,
                Err(error) => return Err(self.abort(vec![error])),
            };
            let next = match own {
                Some(commit) => {
                    settled.push(format!(
                        "skipped the queue's own fragment commit {commit} of an earlier attempt, which the fragment step makes again"
                    ));
                    "--skip"
                }
                None => match rebase::settle(&git, self.runner, &self.config.python) {
                    Ok(done) => {
                        settled.push(done);
                        "--continue"
                    }
                    Err(problems) => return Err(self.abort(problems)),
                },
            };
            finished = match git.run(&["-c", "merge.conflictstyle=diff3", "rebase", next]) {
                Ok(finished) => finished,
                Err(error) => return Err(self.abort(vec![error])),
            };
        }
        let head = self.refresh_head()?;
        let mut why = match &onto {
            Some(onto) if *onto == from => {
                format!("replayed {count} commits of {from}..HEAD onto {pinned}")
            }
            Some(onto) => format!(
                "replayed {count} commits of {from}..HEAD onto {pinned}, since {from}, not --onto {onto}, is the newest base HEAD holds"
            ),
            None => format!("rebased {count} commits onto {pinned}"),
        };
        if !settled.is_empty() {
            why.push_str(&format!(
                "; settled {} stops: {}",
                settled.len(),
                settled.join("; ")
            ));
        }
        Ok(format!("{why}; HEAD is {head}"))
    }

    fn after_rebase_error(&mut self, finished: &crate::process::Finished) -> Failure {
        let mut details = vec![format!(
            "git rebase ended {}: {}",
            finished.status(),
            crate::cut(&finished.last_line(), 200)
        )];
        if let Ok(head) = self.refresh_head()
            && Some(&head) != self.backup_commit.as_ref()
        {
            details.push(format!("HEAD is {head}, not the backup's commit"));
        }
        Failure::with("git rebase failed and left no rebase in progress", details)
    }

    fn abort(&mut self, problems: Vec<String>) -> Failure {
        let mut details = problems;
        let git = self.git();
        match git.run(&["rebase", "--abort"]) {
            Ok(finished) if finished.success() => {}
            Ok(finished) => details.push(format!(
                "git rebase --abort ended {}: {}",
                finished.status(),
                crate::cut(&finished.last_line(), 200)
            )),
            Err(error) => details.push(error),
        }
        match self.refresh_head() {
            Ok(head) if Some(&head) == self.backup_commit.as_ref() => {}
            Ok(head) => details.push(format!(
                "after the abort HEAD is {head}, not the backup's commit {}",
                self.backup_commit.clone().unwrap_or_default()
            )),
            Err(error) => details.push(error),
        }
        Failure::with(
            "the rebase stopped on a conflict the queue does not settle, and was aborted",
            details,
        )
    }

    fn standing(&mut self) -> Step {
        let finished = self.runner.run(
            &self.python(&["tools/promise_standing.py", "--write"]),
            self.dir(),
            &BTreeMap::new(),
            None,
        )?;
        if !finished.success() {
            return Err(Failure::new(format!(
                "tools/promise_standing.py --write ended {}: {}",
                finished.status(),
                crate::cut(&finished.last_line(), 200)
            )));
        }
        let git = self.git();
        let changed = git.status_lines()?;
        if changed.is_empty() {
            return Ok(format!("{STANDING} is current"));
        }
        let others: Vec<String> = changed
            .iter()
            .filter(|line| line.get(3..) != Some(STANDING))
            .map(|line| crate::cut(line, 200))
            .collect();
        if !others.is_empty() {
            return Err(Failure::with(
                format!("tools/promise_standing.py --write changed paths besides {STANDING}"),
                others.into_iter().take(20).collect(),
            ));
        }
        git.text(&["add", "--", STANDING])?;
        git.text(&["commit", "--quiet", "-m", STANDING_SUBJECT])?;
        let head = self.refresh_head()?;
        Ok(format!(
            "{STANDING} regenerated and committed alone as {head}"
        ))
    }

    /// Moves the branch's new `Unreleased` items to its fragment (order 401b, decision 2), only
    /// where PINNED holds `spec/changes/README.md`.
    fn fragment(&mut self) -> Step {
        let pinned = self.pinned().to_string();
        let git = self.git();
        if git.blob(&pinned, FRAGMENTS_README)?.is_none() {
            return Ok(format!(
                "PINNED holds no {FRAGMENTS_README}, so no item moves"
            ));
        }
        let before = git.blob(&pinned, CHANGES)?.unwrap_or_default();
        let after = git.blob("HEAD", CHANGES)?.unwrap_or_default();
        let Some(moved) = changelog::split(&before, &after) else {
            return Ok(format!(
                "{CHANGES} holds no Unreleased item that PINNED's lacks"
            ));
        };
        let order_text = order_text::find(&git, &self.entry.branch, "HEAD")?;
        let name = changelog::fragment_name(&self.entry.branch, order_text.as_deref())?;
        let fragment = format!("{FRAGMENTS}{name}");
        let dirty = git.status_lines()?;
        if !dirty.is_empty() {
            return Err(Failure::with(
                "the worktree is not clean before the fragment step",
                dirty
                    .iter()
                    .take(20)
                    .map(|line| crate::cut(line, 200))
                    .collect(),
            ));
        }
        let target = self.dir().join(&fragment);
        let mut text = match fs::symlink_metadata(&target) {
            Ok(meta) if meta.is_file() => fs::read_to_string(&target)
                .map_err(|error| format!("{fragment} cannot be read: {error}"))?,
            Ok(_) => {
                return Err(Failure::new(format!(
                    "{fragment} exists and is not a regular file"
                )));
            }
            Err(error) if error.kind() == ErrorKind::NotFound => String::new(),
            Err(error) => return Err(Failure::new(format!("{fragment}: {error}"))),
        };
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        text.push_str(&moved.fragment);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("{} cannot be made: {error}", parent.display()))?;
        }
        fs::write(&target, text)
            .map_err(|error| format!("{fragment} cannot be written: {error}"))?;
        fs::write(self.dir().join(CHANGES), &moved.changes)
            .map_err(|error| format!("{CHANGES} cannot be written: {error}"))?;
        git.text(&["add", "--", CHANGES, &fragment])?;
        git.text(&["commit", "--quiet", "-m", changelog::SUBJECT])?;
        let head = self.refresh_head()?;
        let items = if moved.count == 1 { "item" } else { "items" };
        Ok(format!(
            "moved {} {items} from {CHANGES} to {fragment}, committed alone as {head}",
            moved.count
        ))
    }

    fn decisions(&mut self) -> Step {
        let pinned = self.pinned().to_string();
        let finished = self.runner.run(
            &self.python(&["tools/check_decisions.py", &pinned, "WORKTREE"]),
            self.dir(),
            &BTreeMap::new(),
            None,
        )?;
        let last = crate::last_line(&finished.stdout);
        if last == "Errors: 0" && finished.success() {
            return Ok("tools/check_decisions.py ended Errors: 0".into());
        }
        let errors: Vec<String> = finished
            .stdout
            .lines()
            .filter(|line| line.starts_with("error:"))
            .take(20)
            .map(|line| crate::cut(line, 200))
            .collect();
        Err(Failure::with(
            format!(
                "tools/check_decisions.py ended {:?} ({})",
                crate::cut(&last, 200),
                finished.status()
            ),
            errors,
        ))
    }

    /// Checks PINNED..HEAD, and the branch's own commits as each attempt's backup holds them: the
    /// rebase runs with the identity as committer, so it rewrites a foreign committer away.
    fn identity(&mut self) -> Step {
        let identity = &self.config.identity;
        let git = self.git();
        let range = format!("{}..HEAD", self.pinned());
        let mut backups = Vec::new();
        for attempt in 1..=self.attempt {
            let name = format!("refs/wo-backup/{}/{attempt}", self.entry.name);
            let Ok(backup) = git.commit(&name) else {
                continue;
            };
            let from = self
                .upstream_of(&git, &backup)
                .unwrap_or_else(|_| self.pinned().to_string());
            backups.push(format!("{from}..{backup}"));
        }
        let mut seen = BTreeSet::new();
        let mut foreign = Vec::new();
        for each in std::iter::once(&range).chain(&backups) {
            let log = git.text(&["log", "--format=%H%x00%an%x00%ae%x00%cn%x00%ce", each])?;
            for line in log.lines().filter(|line| !line.is_empty()) {
                let fields: Vec<&str> = line.split('\0').collect();
                let [commit, author, author_email, committer, committer_email] = fields[..] else {
                    return Err(Failure::new(format!(
                        "git log gave an unreadable line for {each}"
                    )));
                };
                if !seen.insert(commit.to_string()) {
                    continue;
                }
                if author != identity.name
                    || author_email != identity.email
                    || committer != identity.name
                    || committer_email != identity.email
                {
                    foreign.push(format!(
                        "{commit}: author {author} <{author_email}>, committer {committer} <{committer_email}>"
                    ));
                }
            }
        }
        let count = seen.len();
        let held = if backups.is_empty() {
            String::new()
        } else {
            format!(" and, as the backups hold them, in {}", backups.join(", "))
        };
        if !foreign.is_empty() {
            return Err(Failure::with(
                format!(
                    "{} of the {count} commits in {range}{held} do not carry the identity {} <{}> as author and committer",
                    foreign.len(),
                    identity.name,
                    identity.email
                ),
                foreign.into_iter().take(20).collect(),
            ));
        }
        Ok(format!(
            "each of the {count} commits in {range}{held} carries the identity"
        ))
    }

    fn record(&mut self) -> Step {
        Ok(order_text::check_record(
            &self.git(),
            &self.entry.branch,
            "HEAD",
        )?)
    }

    fn squash(&mut self) -> Step {
        let pinned = self.pinned().to_string();
        let git = self.git();
        let head = git.commit("HEAD")?;
        if git.is_ancestor(&pinned, &head) != Some(true) {
            return Err(Failure::new(format!(
                "{pinned} is not an ancestor of HEAD {head}"
            )));
        }
        let tree = git.tree(&head)?;
        if tree == git.tree(&pinned)? {
            return Err(Failure::new(format!(
                "nothing to merge: HEAD's tree is the tree of {pinned}"
            )));
        }
        let made = git.run_input(
            &["commit-tree", &tree, "-p", &pinned],
            self.message().text(),
        )?;
        if !made.success() {
            return Err(Failure::new(format!(
                "git commit-tree ended {}: {}",
                made.status(),
                crate::cut(&made.last_line(), 200)
            )));
        }
        let squash = made.stdout.trim().to_string();
        let parents = git.text(&["rev-list", "--parents", "-n", "1", &squash])?;
        if git.tree(&squash)? != tree || parents != format!("{squash} {pinned}") {
            return Err(Failure::new(format!(
                "the squash {squash} does not have HEAD's tree and the one parent {pinned}"
            )));
        }
        let paths = git.changed_paths(&pinned, &squash)?;
        let branch = format!("refs/heads/{}", self.entry.branch);
        git.text(&[
            "update-ref",
            "-m",
            "wo queue: squash",
            &branch,
            &squash,
            &head,
        ])?;
        let head = self.refresh_head()?;
        if head != squash {
            return Err(Failure::new(format!(
                "HEAD is {head}, not the squash {squash}"
            )));
        }
        println!("{} squash: {} changed paths", self.entry.name, paths.len());
        for path in paths.iter().take(60) {
            println!("  {path}");
        }
        if paths.len() > 60 {
            println!("  and {} more", paths.len() - 60);
        }
        Ok(format!(
            "squashed into {squash}, its one parent {pinned}; {} changed paths",
            paths.len()
        ))
    }

    fn gate(&mut self) -> Step {
        let config = self.config;
        let name = &self.entry.name;
        let pinned = self.pinned().to_string();
        let argv: Vec<String> = config
            .gate
            .argv
            .iter()
            .map(|arg| arg.replace("{base}", &pinned))
            .collect();
        let mut env = config.gate.env.clone();
        let mut cache_note = String::new();
        if let Some(cache) = &config.gate.cache {
            if accepts(&cache.address) {
                env.extend(cache.env.clone());
                cache_note = format!("; the cache at {} answered", cache.address);
            } else {
                cache_note = format!("; the cache at {} did not answer within 2 s", cache.address);
            }
        }
        let records = config.records_dir();
        fs::create_dir_all(&records)
            .map_err(|error| format!("{} cannot be made: {error}", records.display()))?;
        let output_path = config
            .state
            .join(format!("gate-{name}-{}.txt", self.attempt));
        let output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&output_path)
            .map_err(|error| format!("{} cannot be made: {error}", output_path.display()))?;
        let errors = output
            .try_clone()
            .map_err(|error| format!("{}: {error}", output_path.display()))?;
        let status = self
            .runner
            .command(&argv, self.dir(), &env)
            .stdout(output)
            .stderr(errors)
            .status()
            .map_err(|error| format!("the gate, {}, could not start: {error}", argv[0]))?;
        let text = fs::read(&output_path)
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            .map_err(|error| format!("{} cannot be read: {error}", output_path.display()))?;
        let last = crate::last_line(&text);
        let mut problems = Vec::new();
        match last
            .split_once("Record: ")
            .and_then(|(_, rest)| rest.split_whitespace().next())
        {
            None => problems.push("its last line names no record".to_string()),
            Some(named) => match self.copy_record(named) {
                Ok(copy) => self.gate_record = Some(copy),
                Err(error) => problems.push(error),
            },
        }
        if !status.success() {
            let ended = match status.code() {
                Some(code) => format!("exit {code}"),
                None => "ended by a signal".into(),
            };
            problems.insert(0, format!("the gate ended {ended}"));
        }
        match self.git().commit("HEAD") {
            Ok(head) if Some(&head) == self.head.as_ref() => {}
            Ok(head) => problems.push(format!("HEAD moved during the gate, to {head}")),
            Err(error) => problems.push(error),
        }
        if !problems.is_empty() {
            let mut details: Vec<String> = text
                .lines()
                .filter(|line| line.starts_with("FAIL"))
                .map(|line| crate::cut(line, 200))
                .collect();
            details.push(format!("last line: {last}"));
            return Err(Failure::with(problems.join("; "), details));
        }
        println!("{name} gate: {last}");
        Ok(format!("the gate passed{cache_note}: {last}"))
    }

    fn copy_record(&self, named: &str) -> Result<String, String> {
        let source = if Path::new(named).is_absolute() {
            PathBuf::from(named)
        } else {
            self.dir().join(named)
        };
        let file_name = source
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or(format!("the record {named} has no file name"))?;
        let target = self
            .config
            .records_dir()
            .join(format!("{}-{}-{file_name}", self.entry.name, self.attempt));
        let mut from = fs::File::open(&source)
            .map_err(|error| format!("the record {named} cannot be read: {error}"))?;
        let mut to = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&target)
            .map_err(|error| match error.kind() {
                ErrorKind::AlreadyExists => format!(
                    "{} exists already; a record is never copied over a file",
                    target.display()
                ),
                _ => format!("{} cannot be made: {error}", target.display()),
            })?;
        io::copy(&mut from, &mut to)
            .map_err(|error| format!("the record {named} was not copied: {error}"))?;
        Ok(target.display().to_string())
    }

    fn push(&mut self) -> Step {
        let config = self.config;
        let target = format!("HEAD:refs/heads/{}", self.entry.branch);
        self.git()
            .text(&["push", "--force-with-lease", &config.remote, &target])?;
        Ok(format!(
            "pushed {} to {}",
            self.head.clone().unwrap_or_default(),
            config.remote
        ))
    }

    fn pull_request(&mut self) -> Step {
        let branch = self.entry.branch.clone();
        let listed = self.gh(&[
            "pr", "list", "--head", &branch, "--state", "open", "--json", "number",
        ])?;
        if !listed.success() {
            return Err(Failure::new(format!(
                "gh pr list ended {}: {}",
                listed.status(),
                crate::cut(&listed.last_line(), 200)
            )));
        }
        let numbers: Vec<u64> = serde_json::from_str::<Value>(listed.stdout.trim())
            .ok()
            .and_then(|value| {
                value.as_array().map(|items| {
                    items
                        .iter()
                        .filter_map(|item| item.get("number").and_then(Value::as_u64))
                        .collect()
                })
            })
            .ok_or("gh pr list did not print a JSON list".to_string())?;
        match numbers[..] {
            [number] => {
                self.pr = Some(number);
                return Ok(format!("pull request {number} is open for {branch}"));
            }
            [] => {}
            _ => {
                return Err(Failure::new(format!(
                    "{} pull requests are open for {branch}",
                    numbers.len()
                )));
            }
        }
        let message = self.message().clone();
        let body = message.pull_request_body(&self.config.pr_footer);
        let base = self.config.base.clone();
        let created = self.gh(&[
            "pr",
            "create",
            "--base",
            &base,
            "--head",
            &branch,
            "--title",
            message.title(),
            "--body",
            &body,
        ])?;
        if !created.success() {
            return Err(Failure::new(format!(
                "gh pr create ended {}: {}",
                created.status(),
                crate::cut(&created.last_line(), 200)
            )));
        }
        let url = crate::last_line(&created.stdout);
        let number = url
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .and_then(|segment| segment.parse::<u64>().ok())
            .ok_or(format!(
                "gh pr create printed no URL ending in a number: {}",
                crate::cut(&url, 200)
            ))?;
        self.pr = Some(number);
        Ok(format!("pull request {number} opened for {branch}"))
    }

    fn main_unmoved(&mut self) -> Step {
        let config = self.config;
        let pinned = self.pinned().to_string();
        let git = self.git();
        git.text(&["fetch", &config.remote])?;
        let now = git.commit(&config.base_ref())?;
        if now != pinned {
            return Err(Failure::new(format!(
                "main moved during the gate: {} is {now}, not {pinned}; pull request {} stays open",
                config.base_name(),
                self.pr.unwrap_or_default()
            )));
        }
        Ok(format!("{} still names {pinned}", config.base_name()))
    }

    fn merge(&mut self) -> Step {
        let config = self.config;
        let pinned = self.pinned().to_string();
        let number = self.pr.expect("pull-request numbered it");
        let head = self.head.clone().unwrap_or_default();
        let subject = format!("{} (#{number})", self.message().title());
        let body = self.message().merge_body();
        let merged = self.gh(&[
            "pr",
            "merge",
            &number.to_string(),
            "--squash",
            "--match-head-commit",
            &head,
            "--subject",
            &subject,
            "--body",
            &body,
        ])?;
        if !merged.success() {
            return Err(Failure::new(format!(
                "gh pr merge ended {}: {}",
                merged.status(),
                crate::cut(&merged.last_line(), 200)
            )));
        }
        let git = self.git();
        git.text(&["fetch", &config.remote])?;
        let landed = git.commit(&config.base_ref())?;
        let mut differs = Vec::new();
        let parents = git.text(&["rev-list", "--parents", "-n", "1", &landed])?;
        if parents != format!("{landed} {pinned}") {
            differs.push(format!("its parents are not {pinned} alone: {parents}"));
        }
        if git.tree(&landed)? != git.tree(&head)? {
            differs.push("its tree is not HEAD's".to_string());
        }
        let landed_subject = git.text(&["log", "-1", "--format=%s", &landed])?;
        if landed_subject != subject {
            differs.push(format!("its subject is {landed_subject:?}"));
        }
        if !differs.is_empty() {
            return Err(Failure::with(
                format!(
                    "merged commit differs: {} names {landed}",
                    config.base_name()
                ),
                differs,
            ));
        }
        self.merged = Some((landed.clone(), subject.clone()));
        Ok(format!(
            "{} names {landed}: one parent {pinned}, HEAD's tree, the subject {subject:?}",
            config.base_name()
        ))
    }

    fn tidy(&mut self) -> Step {
        let mut removed = Vec::new();
        let mut errors = Vec::new();
        remove_targets(self.dir(), 1, &mut removed, &mut errors);
        let note = format!("removed {} target directories", removed.len());
        if errors.is_empty() {
            Ok(note)
        } else {
            Err(Failure::with(note, errors))
        }
    }
}

/// Removes each directory named `target` one to three levels under `dir`, never following a
/// symbolic link or entering `.git`.
fn remove_targets(dir: &Path, depth: usize, removed: &mut Vec<String>, errors: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(meta) = fs::symlink_metadata(&path) else {
            continue;
        };
        if !meta.is_dir() || meta.file_type().is_symlink() {
            continue;
        }
        let name = entry.file_name();
        if name == ".git" {
            continue;
        }
        if name == "target" {
            match fs::remove_dir_all(&path) {
                Ok(()) => removed.push(path.display().to_string()),
                Err(error) => errors.push(format!("{}: {error}", path.display())),
            }
        } else if depth < 3 {
            remove_targets(&path, depth + 1, removed, errors);
        }
    }
}

/// Whether `address` accepts a TCP connection within 2 s.
pub fn accepts(address: &str) -> bool {
    let deadline = Instant::now() + Duration::from_secs(2);
    let Ok(addresses) = address.to_socket_addrs() else {
        return false;
    };
    for socket in addresses {
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return false;
        }
        if TcpStream::connect_timeout(&socket, left).is_ok() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{Failure, accepts};
    use std::net::TcpListener;

    #[test]
    fn a_cache_answers_only_while_it_listens() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap().to_string();
        assert!(accepts(&address));
        drop(listener);
        assert!(!accepts(&address));
        assert!(!accepts("no-such-host.invalid:1"));
    }

    #[test]
    fn a_failure_is_one_line() {
        let failure = Failure::with("why", vec!["a\nb".into(), "c".into()]);
        assert_eq!(failure.line(), "why; a b; c");
    }
}
