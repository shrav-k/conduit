use crate::failure::Step;
use crate::{
    Output,
    fields::{self, Node},
    listing::Listing,
    process::{self, Client, Launch, Scratch, ScratchRoot},
    send,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

struct Config {
    codex: bool,
    home: PathBuf,
    path: PathBuf,
    version: String,
    model: String,
    deadline: Duration,
    hold: Duration,
}
impl Config {
    fn parse() -> Result<Self, &'static str> {
        let mut args = std::env::args_os().skip(1);
        let codex = match args.next().and_then(|s| s.into_string().ok()).as_deref() {
            Some("codex") => true,
            Some("claude") => false,
            _ => return Err("command"),
        };
        let mut flags = BTreeMap::new();
        while let Some(key) = args.next() {
            let key = key.into_string().map_err(|_| "flags")?;
            if ![
                "--home",
                "--codex",
                "--claude",
                "--expect-version",
                "--model",
                "--deadline-seconds",
                "--hold-seconds",
            ]
            .contains(&key.as_str())
            {
                return Err("flags");
            }
            let value = args.next().ok_or("flags")?;
            if value.is_empty() || flags.insert(key, value).is_some() {
                return Err("flags");
            }
        }
        let home = PathBuf::from(flags.remove("--home").ok_or("home")?);
        if !home.is_dir() {
            return Err("home_directory");
        }
        let home = if home.is_absolute() {
            home
        } else {
            std::env::current_dir().map_err(|_| "home")?.join(home)
        };
        let path = PathBuf::from(
            flags
                .remove(if codex { "--codex" } else { "--claude" })
                .ok_or("client_path")?,
        );
        if !path.is_absolute() {
            return Err("client_path_absolute");
        }
        let version = flags
            .remove("--expect-version")
            .ok_or("expect_version")?
            .into_string()
            .map_err(|_| "expect_version")?;
        let model = if codex {
            String::new()
        } else {
            flags
                .remove("--model")
                .ok_or("model")?
                .into_string()
                .map_err(|_| "model")?
        };
        let mut number = |key: &str, default: u64, min, max| -> Result<Duration, &'static str> {
            let value = match flags.remove(key) {
                None => default,
                Some(v) => v
                    .to_str()
                    .and_then(|s| s.parse::<u64>().ok())
                    .ok_or("seconds")?,
            };
            if !(min..=max).contains(&value) {
                return Err("seconds");
            }
            Ok(Duration::from_secs(value))
        };
        let deadline = number("--deadline-seconds", 60, 1, 600)?;
        let hold = number("--hold-seconds", 10, 0, 60)?;
        if !flags.is_empty() {
            return Err("flags");
        }
        if codex && !std::fs::metadata(home.join("auth.json")).is_ok_and(|m| m.is_file()) {
            return Err("login_file");
        }
        Ok(Self {
            codex,
            home,
            path,
            version,
            model,
            deadline,
            hold,
        })
    }
    fn spawn(
        &self,
        args: &[String],
        version_home: Option<&std::path::Path>,
        settings: bool,
        root: &ScratchRoot,
    ) -> io::Result<Client> {
        Client::spawn(
            root,
            &self.path,
            version_home.unwrap_or(&self.home),
            args,
            Launch {
                codex: self.codex,
                deadline: self.deadline,
                version: version_home.is_some(),
                settings,
            },
        )
    }
}
fn version_token(s: &str) -> bool {
    let b = s.as_bytes();
    let mut i = 0;
    for part in 0..3 {
        let start = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == start {
            return false;
        }
        if part < 2 {
            if b.get(i) != Some(&b'.') {
                return false;
            }
            i += 1;
        }
    }
    true
}
fn numeric_version(s: &str) -> Option<&str> {
    let mut end = 0;
    for part in 0..3 {
        let start = end;
        while s.as_bytes().get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
        if end == start {
            return None;
        }
        if part < 2 {
            if s.as_bytes().get(end) != Some(&b'.') {
                return None;
            }
            end += 1;
        }
    }
    (end <= 64).then_some(&s[..end])
}
fn version(c: &Config, out: &mut Output, root: &ScratchRoot) -> io::Result<bool> {
    let home = Scratch::new(root)?;
    let mut child = c.spawn(&["--version".into()], Some(&home.0), false, root)?;
    let mut found = None;
    while let Some(line) = child.line() {
        if let Some(token) = line.text.split_whitespace().find(|s| version_token(s)) {
            found = Some(token.to_owned());
            break;
        }
    }
    let stopped = child.stop();
    let strays = child.strays();
    let prefix = if c.codex { "codex" } else { "claude" };
    let matched = found.as_deref() == Some(c.version.as_str());
    out.value(&format!("{prefix}.version_matches"), matched);
    if !matched {
        // Only the numeric core is printable; an arbitrary version suffix is not.
        let core = found.as_deref().and_then(numeric_version);
        out.value(&format!("{prefix}.version_found"), core.unwrap_or("none"));
    }
    out.value(&format!("{prefix}.version.group_stopped"), stopped);
    Ok(matched && stopped && strays == 0 && !process::interrupted())
}
/// The signs of model work in a Codex notification, in one place. A notification's method is
/// lowercased, `_` and `-` are dropped, and it is split at `/`: a part that starts with one of
/// `CODEX_WORK_PARTS` or holds one of `CODEX_WORK_WORDS` is a sign. So every `turn/`, `item/`
/// and agent-message notification, `thread/tokenUsage/updated`, and the older
/// `codex/event/task_started`, `codex/event/agent_message_delta`, `codex/event/agent_reasoning`
/// and `codex/event/token_count` are signs; `account/rateLimits/updated` is not.
pub const CODEX_WORK_PARTS: [&str; 3] = ["turn", "item", "task"];
pub const CODEX_WORK_WORDS: [&str; 4] = ["agentmessage", "tokenusage", "tokencount", "reasoning"];
pub fn codex_work(v: &Value) -> bool {
    let Some(method) = v.get("method").and_then(Value::as_str) else {
        return false;
    };
    let method: String = method
        .chars()
        .filter(|c| !matches!(c, '_' | '-'))
        .collect::<String>()
        .to_ascii_lowercase();
    method.split('/').any(|part| {
        CODEX_WORK_PARTS.iter().any(|p| part.starts_with(p))
            || CODEX_WORK_WORDS.iter().any(|w| part.contains(w))
    })
}
/// What a Codex client showed that ends the run: a sign of model work, or, after its answer, a
/// line that is not JSON (whose sign could not be read).
#[derive(Clone, Copy, Default)]
struct Signs {
    work: bool,
    unreadable: bool,
}
impl Signs {
    fn any(self) -> bool {
        self.work || self.unreadable
    }
    /// Checks a line printed after the answer, or before a stop; true at a sign.
    fn check(&mut self, line: &process::Line) -> bool {
        match serde_json::from_str::<Value>(&line.text) {
            Ok(v) => {
                let sign = codex_work(&v);
                self.work |= sign;
                sign
            }
            Err(_) => {
                self.unreadable = true;
                true
            }
        }
    }
    /// Prints why the run stops, if it does; true if it does.
    fn stop(self, out: &mut Output) -> bool {
        if self.work {
            out.value("codex.stopped_on_model_work", true);
        }
        if self.unreadable {
            out.value("codex.stopped_on_unreadable_line", true);
        }
        self.any()
    }
}
pub struct Reply {
    at: Instant,
    value: Value,
    node: Node,
}
/// The answer with this id. Every notification before it is checked for a sign of model work:
/// at the first, no answer is taken and `signs.work` is set; any other is skipped.
fn reply(client: &Client, id: u64, signs: &mut Signs) -> Option<Reply> {
    while let Some(line) = client.line() {
        let value: Value = serde_json::from_str(&line.text).ok()?;
        if value.get("method").is_some() {
            if codex_work(&value) {
                signs.work = true;
                return None;
            }
            continue;
        }
        if value["id"] != json!(id) {
            return None;
        }
        if value.get("result").is_none() && value.get("error").is_none() {
            return None;
        }
        return Some(Reply {
            at: line.at,
            node: Node::parse(&line.text)?,
            value,
        });
    }
    None
}
fn initialize(client: &mut Client, signs: &mut Signs) -> io::Result<Option<Instant>> {
    send::codex(client.input.as_mut().unwrap(), &send::initialize())?;
    let Some(answer) = reply(client, 1, signs) else {
        return Ok(None);
    };
    if answer.value.get("error").is_some() {
        return Ok(None);
    }
    send::codex(client.input.as_mut().unwrap(), &send::initialized())?;
    Ok(Some(answer.at))
}
fn request(client: &mut Client) -> io::Result<Instant> {
    send::codex(client.input.as_mut().unwrap(), &send::rate_limits_read())?;
    Ok(Instant::now())
}
fn metrics(
    out: &mut Output,
    prefix: &str,
    client: &Client,
    initialized: Option<Instant>,
    sent: Option<Instant>,
    answer: Option<&Reply>,
    stopped: bool,
) {
    out.value(&format!("{prefix}.answered"), answer.is_some());
    let error = answer.and_then(|r| r.value.get("error"));
    out.value(&format!("{prefix}.error_answer"), error.is_some());
    if let Some(code) = error.and_then(|v| v["code"].as_i64()) {
        out.value(&format!("{prefix}.error_code"), code);
    }
    if let Some(at) = initialized {
        out.value(
            &format!("{prefix}.ms_to_initialized"),
            at.saturating_duration_since(client.start).as_millis(),
        );
    }
    if let Some(r) = answer {
        out.value(
            &format!("{prefix}.ms_to_answer"),
            r.at.saturating_duration_since(client.start).as_millis(),
        );
        if let Some(at) = sent {
            out.value(
                &format!("{prefix}.ms_request_to_answer"),
                r.at.saturating_duration_since(at).as_millis(),
            );
        }
    }
    out.value(&format!("{prefix}.group_stopped"), stopped);
}
fn after(before: &Listing, c: &Config, out: &mut Output, stopped: bool) -> io::Result<bool> {
    let after = Listing::read(&c.home)?;
    let unchanged = before.compare(&after, c.codex, out);
    if c.codex && !unchanged {
        out.value("codex.stopped_on_login_change", true);
    }
    if !stopped {
        out.value(
            if c.codex {
                "codex.stopped_on_survivor"
            } else {
                "claude.stopped_on_survivor"
            },
            true,
        );
    }
    Ok((!c.codex || unchanged) && stopped && !process::interrupted())
}
/// A Codex client held after its answer: until `end`, or stopped at once where it has none.
struct Held<'a> {
    client: &'a mut Client,
    signs: Signs,
    end: Option<Instant>,
    stopped: Option<bool>,
}
impl<'a> Held<'a> {
    fn new(client: &'a mut Client, signs: Signs, end: Option<Instant>) -> Self {
        let end = if signs.any() { None } else { end };
        Self {
            client,
            signs,
            end,
            stopped: None,
        }
    }
}
/// Holds each client to its end, reading every line each prints meanwhile; the first sign in
/// any of them, or an interrupt, stops them all at once. Each is stopped at its own end, the
/// earlier first, so a slow peer cannot cut a hold short. Then what each printed before its stop
/// is read too.
fn hold(held: &mut [Held]) {
    loop {
        let sign = held.iter().any(|h| h.signs.any());
        let mut open = false;
        for h in held.iter_mut().filter(|h| h.stopped.is_none()) {
            if sign || process::interrupted() || h.end.is_none_or(|end| Instant::now() >= end) {
                h.stopped = Some(h.client.stop());
                continue;
            }
            open = true;
            while let Some(line) = h.client.poll() {
                if h.signs.check(&line) {
                    break;
                }
            }
        }
        if !open {
            break;
        }
        thread::sleep(Duration::from_millis(5));
    }
    for h in held.iter_mut() {
        let signs = &mut h.signs;
        h.client.drain(|line| signs.check(line));
    }
}
/// Whether another client may start: not after an interrupt, nor once a line could not be
/// written, since what the next client did could not be printed.
fn go_on(out: &Output) -> bool {
    !process::interrupted() && !out.failed
}
fn codex(c: &Config, out: &mut Output, root: &ScratchRoot) -> io::Result<bool> {
    let args = [
        "app-server",
        "--stdio",
        "-c",
        "cli_auth_credentials_store=\"file\"",
    ]
    .map(str::to_owned);
    let mut first = BTreeSet::new();
    for n in 1..=3 {
        if !go_on(out) {
            return Ok(false);
        }
        let before = Listing::read(&c.home)?;
        let mut client = c.spawn(&args, None, false, root)?;
        let mut signs = Signs::default();
        let initialized = initialize(&mut client, &mut signs)?;
        let sent = if initialized.is_some() {
            Some(request(&mut client)?)
        } else {
            None
        };
        let answer = if sent.is_some() {
            reply(&client, 2, &mut signs)
        } else {
            None
        };
        // Held from its answer, still read: a sign during the hold or the stop ends the run.
        let mut held = [Held::new(
            &mut client,
            signs,
            answer.as_ref().map(|_| Instant::now() + c.hold),
        )];
        hold(&mut held);
        let [Held { signs, stopped, .. }] = held;
        let stopped = stopped.unwrap_or(false);
        let prefix = format!("codex.read.{n}");
        metrics(
            out,
            &prefix,
            &client,
            initialized,
            sent,
            answer.as_ref(),
            stopped,
        );
        out.value(&format!("{prefix}.model_work"), signs.work);
        let fields = answer.as_ref().and_then(|r| r.node.get("result"));
        if n == 1 {
            if let Some(node) = fields {
                first = fields::print(node, &prefix, true, out);
            }
        } else {
            // Compute shape without emitting another answer's values.
            let same = fields.is_some_and(|node| shape(node) == first);
            out.value(&format!("{prefix}.same_fields"), same);
        }
        let safe = after(&before, c, out, stopped && client.strays() == 0)?;
        if !signs.stop(out) && answer.is_none() {
            out.value("codex.stopped_on_no_answer", true);
        }
        if !safe || answer.is_none() || signs.any() {
            return Ok(false);
        }
    }
    if !go_on(out) {
        return Ok(false);
    }
    let before = Listing::read(&c.home)?;
    let mut a = c.spawn(&args, None, false, root)?;
    let mut b = c.spawn(&args, None, false, root)?;
    let (mut sa, mut sb) = (Signs::default(), Signs::default());
    let ai = initialize(&mut a, &mut sa)?;
    // After a sign in one, nothing more is sent to either.
    let bi = if sa.any() {
        None
    } else {
        initialize(&mut b, &mut sb)?
    };
    let (ar, br) = if ai.is_some() && bi.is_some() {
        (Some(request(&mut a)?), Some(request(&mut b)?))
    } else {
        (None, None)
    };
    let (aa, ba) = if ar.is_some() && br.is_some() {
        let aa = reply(&a, 2, &mut sa);
        let ba = if sa.any() {
            None
        } else {
            reply(&b, 2, &mut sb)
        };
        (aa, ba)
    } else {
        (None, None)
    };
    // Each hold starts at the timestamp of that client's answer, including time spent
    // receiving its peer.
    let hold_until = |r: &Option<Reply>| r.as_ref().map(|r| r.at + c.hold);
    let mut held = [
        Held::new(&mut a, sa, hold_until(&aa)),
        Held::new(&mut b, sb, hold_until(&ba)),
    ];
    hold(&mut held);
    let [
        Held {
            signs: sa,
            stopped: astop,
            ..
        },
        Held {
            signs: sb,
            stopped: bstop,
            ..
        },
    ] = held;
    let (astop, bstop) = (astop.unwrap_or(false), bstop.unwrap_or(false));
    metrics(out, "codex.pair.1", &a, ai, ar, aa.as_ref(), astop);
    out.value("codex.pair.1.model_work", sa.work);
    metrics(out, "codex.pair.2", &b, bi, br, ba.as_ref(), bstop);
    out.value("codex.pair.2.model_work", sb.work);
    let both = aa.is_some() && ba.is_some();
    out.value("codex.pair.both_answered", both);
    let signs = Signs {
        work: sa.work || sb.work,
        unreadable: sa.unreadable || sb.unreadable,
    };
    if !signs.stop(out) && !both {
        out.value("codex.stopped_on_no_answer", true);
    }
    let overlap = match (ar, br, aa.as_ref(), ba.as_ref()) {
        (Some(ar), Some(br), Some(aa), Some(ba)) => {
            ar < aa.at && ar < ba.at && br < aa.at && br < ba.at
        }
        _ => false,
    };
    out.value("codex.pair.overlapped", overlap);
    let stopped = astop && bstop && a.strays() == 0 && b.strays() == 0;
    Ok(after(&before, c, out, stopped)? && both && !signs.any())
}
fn shape(node: &Node) -> BTreeSet<(String, String)> {
    // Reuse the field walker with a collector, never an output sink.
    fields::shape(node, true)
}
fn positive_tokens(v: &Value) -> bool {
    match v {
        Value::Object(map) => map.iter().any(|(k, v)| {
            (k.to_ascii_lowercase().contains("token") && v.as_f64().is_some_and(|n| n > 0.0))
                || positive_tokens(v)
        }),
        Value::Array(a) => a.iter().any(positive_tokens),
        _ => false,
    }
}
fn model_work(v: &Value) -> bool {
    match v["type"].as_str().unwrap_or("") {
        "stream_event" => true,
        "system" => v["subtype"] == "api_retry",
        "assistant" => positive_tokens(&v["message"]["usage"]),
        "result" => {
            positive_tokens(&v["usage"])
                || v["modelUsage"].as_object().is_some_and(|m| !m.is_empty())
                || v["total_cost_usd"].as_f64().is_some_and(|n| n > 0.0)
                || v["duration_api_ms"].as_f64().is_some_and(|n| n > 0.0)
        }
        _ => false,
    }
}
fn last_known(v: &Value) -> bool {
    match v {
        Value::String(s) => s.to_ascii_lowercase().contains("last-known"),
        Value::Array(a) => a.iter().any(last_known),
        Value::Object(m) => m.values().any(last_known),
        _ => false,
    }
}
fn claude(c: &Config, out: &mut Output, root: &ScratchRoot) -> io::Result<bool> {
    let before = Listing::read(&c.home)?;
    let mut args = [
        "--print",
        "--input-format",
        "stream-json",
        "--output-format",
        "stream-json",
        "--verbose",
        "--include-partial-messages",
        "--no-session-persistence",
        "--setting-sources",
        "",
        "--strict-mcp-config",
        "--tools",
        "",
        "--model",
    ]
    .map(str::to_owned)
    .to_vec();
    args.push(c.model.clone());
    let mut client = c.spawn(&args, None, true, root)?;
    send::claude(client.input.as_mut().unwrap(), "/usage")?;
    let mut result = None;
    let mut reports = Vec::new();
    let mut rates = Vec::new();
    let mut report_at = None;
    let mut note = None;
    let mut work = false;
    let mut unreadable = false;
    let mut stopped = None;
    while let Some(line) = client.line() {
        let Ok(v) = serde_json::from_str::<Value>(&line.text) else {
            unreadable = true;
            break;
        };
        work = model_work(&v);
        if work {
            stopped = Some(client.stop());
        }
        if note.is_none() && last_known(&v) {
            note = Some(match v["type"].as_str() {
                Some("assistant") => "assistant",
                Some("result") => "result",
                Some("system") => "system",
                Some("user") => "user",
                _ => "other",
            });
        }
        let node = Node::parse(&line.text).unwrap();
        let mut found = Vec::new();
        node.find("usage_report", &mut found);
        if !found.is_empty() {
            report_at.get_or_insert(line.at);
            reports.extend(found.into_iter().cloned());
        }
        // Some SDK versions name the event itself usage_report.
        if v["type"] == "usage_report" && reports.is_empty() {
            report_at.get_or_insert(line.at);
            reports.push(node.clone());
        }
        if v["type"] == "rate_limit_event"
            && let Some(info) = node.get("rate_limit_info")
        {
            rates.push(info.clone());
        }
        let terminal = v["type"] == "result";
        if terminal {
            result = Some((line.at, v));
        }
        if work {
            break;
        }
        if terminal {
            break;
        }
    }
    client.input.take();
    // Each line after the result is read as well: a sign of model work, or a line that is not
    // JSON, stops the group at once, during the wait for exit as before the result.
    let mut check = |line: &process::Line| match serde_json::from_str::<Value>(&line.text) {
        Ok(v) => {
            let sign = model_work(&v);
            work |= sign;
            sign
        }
        Err(_) => {
            unreadable = true;
            true
        }
    };
    if stopped.is_none() && result.is_some() && client.watch_exit(c.hold, &mut check) {
        stopped = Some(client.stop());
    }
    let stopped = stopped.unwrap_or_else(|| client.stop());
    // What the client printed just before any stop is checked too.
    client.drain(&mut check);
    let prefix = "claude.read";
    out.value(&format!("{prefix}.model_work"), work);
    if work {
        out.value(&format!("{prefix}.stopped_on_model_work"), true);
    }
    if unreadable {
        out.value(&format!("{prefix}.stopped_on_unreadable_line"), true);
    }
    out.value(&format!("{prefix}.answered"), result.is_some());
    if let Some(at) = report_at {
        out.value(
            &format!("{prefix}.ms_to_report"),
            at.duration_since(client.start).as_millis(),
        );
    }
    if let Some((at, v)) = &result {
        out.value(
            &format!("{prefix}.ms_to_result"),
            at.duration_since(client.start).as_millis(),
        );
        if let Some(ms) = v["duration_api_ms"].as_f64() {
            out.value(&format!("{prefix}.ms_api"), ms);
        }
    }
    out.value(
        &format!("{prefix}.terminal_reason_absent"),
        result
            .as_ref()
            .is_none_or(|(_, v)| v.get("terminal_reason").is_none_or(Value::is_null)),
    );
    out.value(
        &format!("{prefix}.usage_report_present"),
        !reports.is_empty(),
    );
    out.value(
        &format!("{prefix}.rate_limits_null"),
        reports
            .iter()
            .any(|r| matches!(r.get("rate_limits"), Some(Node::Leaf(Value::Null)))),
    );
    out.value(&format!("{prefix}.last_known_note"), note.is_some());
    out.value(
        &format!("{prefix}.last_known_where"),
        note.unwrap_or("none"),
    );
    out.value(&format!("{prefix}.group_stopped"), stopped);
    for node in &reports {
        fields::print(node, "claude.read.usage_report", false, out);
    }
    for node in &rates {
        fields::print(node, "claude.read.rate_limit_info", false, out);
    }
    let stopped = stopped && client.strays() == 0;
    Ok(after(&before, c, out, stopped)? && result.is_some() && !work && !unreadable)
}
pub fn main_entry() -> i32 {
    process::install_signals();
    let mut out = Output::default();
    let result = (|| -> io::Result<bool> {
        let mut c = match Config::parse() {
            Ok(c) => c,
            Err(check) => {
                out.value(&format!("refused.{check}"), true);
                return Ok(false);
            }
        };
        c.home = c.home.canonicalize().map_err(|e| Step::Listing.error(e))?;
        let root = ScratchRoot::validate(&c.home)?;
        if !version(&c, &mut out, &root)? || !go_on(&out) {
            return Ok(false);
        }
        if c.codex {
            codex(&c, &mut out, &root)
        } else {
            claude(&c, &mut out, &root)
        }
    })();
    // All owned clients and scratch directories have dropped before these final lines, so a
    // failed write (`out.failed`) is reported and exits 1 only after every group is stopped and
    // reaped and every scratch directory removed.
    if let Err(error) = &result {
        let step = error
            .get_ref()
            .and_then(|e| e.downcast_ref::<Step>())
            .copied()
            .unwrap_or(Step::Write);
        // A start or write withheld after an interrupt is reported by `interrupted` alone.
        if !matches!(step, Step::Interrupted) {
            out.value(&format!("probe_failed.{}", step.name()), true);
            if let Some(reason) = crate::failure::reason() {
                out.value("probe_failed.reason", reason);
            }
        }
    }
    // Once any client has started: how many of its descendants outlived its group's stop.
    let strays = process::strays();
    if process::spawned() {
        out.value("stray_processes", strays);
    }
    let left = process::scratch_left();
    if left {
        out.value("scratch_left", true);
    }
    if process::interrupted() {
        out.value("interrupted", true);
    }
    let clean = out.finish();
    if result.unwrap_or(false) && clean && !process::interrupted() && strays == 0 && !left {
        0
    } else {
        1
    }
}

/// Both requests must be sent before consuming either answer.
pub fn pair_answers(a: &Client, b: &Client) -> (Option<Reply>, Option<Reply>) {
    (
        reply(a, 2, &mut Signs::default()),
        reply(b, 2, &mut Signs::default()),
    )
}
