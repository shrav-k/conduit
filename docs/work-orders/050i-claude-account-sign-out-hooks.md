# Work order 050i (P2i): `claude-code.v2`'s account and sign-out hooks

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order runs on the Mac or on the Linux node;
it needs neither. It has network access only for tests that bind a loopback port; it adds no
crate, so cargo fetches nothing, and you use the network for nothing else. Run one cargo command
at a time. Never print a whole large file: use `head`, `tail`, `grep`, `sed -n` or a count. Never
run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and
never read or print a key, a token or any credential. Never read or print any process's
environment or arguments, never run a command that lists every process on the machine, such as
`ps -e`, `ps aux` or `top`, and never signal a process that a test of this order did not start.
Never run the real `claude` or `codex` programs, not even for their help or version, and sign in
nowhere. This order adds a client status read and a client logout: every `Config` a test builds
and every wrapper a test starts uses the package's scripted client (`--client-program`), as every
test at `BASE` does, so that no status read and no logout ever reaches a real client. The tests
use temporary directories only, and the engine is run only to initialize a store and register a
project.

## What and why

**Where this order starts.** Your tree is the branch that the coordinator names as `BASE`, not
`main`: the Claude lane's branch, cut from `wo/050-agents-0.2.0` at order 050h's tip, `f817139e`,
which holds orders 050a, 076 and 078, 050a's Record, and orders 050c, 050d, 050e, 050f and 050h,
each with its Record. This order is the first of the Claude lane (050i, 050j, 050k). The Codex lane
(050l, 050m) starts from the same tip, `f817139e`, on a branch of its own, and does not share
`BASE`; neither lane holds the other's work. `BASE`'s tip holds this order file, committed, in this
refreshed text rather than the one first merged on `main` (#369): the coordinator puts it on the
lane's branch after this refreshed text merges, or rebases the lane onto `main` once it has, and
sets `BASE` to that branch's tip. Orders 050f and 050h were not built when this order was first
written; it was then read again against `f817139e`, and every line below that reads what they
brought is checked here (order 050h's Record, R1 and R8, changed its `logout` and `retire_within`,
as "What the branch already does" says). Before you change anything, check from the repository root
that you find there what that gives, and stop, change nothing and report if any check prints
otherwise. The first Python line needs a Python with PyYAML, as `tools/check_schemas.py` does
(AGENTS.md, "Checks").

```bash
P=packages/agent-claude-code
git merge-base --is-ancestor d891543f "$BASE"; echo "main $?"
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/050i-claude-account-sign-out-hooks.md >/dev/null; echo "order $?"
git status --short | wc -l
grep -c '^## Record' docs/work-orders/050a-agents-v2-own-tools.md docs/work-orders/050c-agents-v2-client-home.md docs/work-orders/050d-agents-v2-process-tree.md docs/work-orders/050e-agents-v2-failed-turn-fields.md docs/work-orders/050f-agents-v2-inputs.md docs/work-orders/050h-agents-v2-retire-hook.md
python3 -c "import yaml; s=[d for d in yaml.safe_load_all(open('packages/agent-claude-code/contracts.yaml')) if d['metadata']['name'] == 'claude-code.v2'][0]['spec']; print(s['schema']['required'].count('account'), s['schema']['properties']['account'], 'apiKey' in s['schema']['properties'], 'signIn' in s, s.get('toolsDirectory'), s['requires'], s['implementation'])"
grep -c "^    implements: \['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1'\]$" $P/conduit.yaml
grep -c "^      'github.com/shrav-k/conduit/core#retire-hook.v1': retire$" $P/conduit.yaml
for s in 'pub const LOGOUT_DEADLINE: Duration = Duration::from_secs(60);' 'pub const RETIRED: &str = "client signed out and its home emptied";' 'pub fn logout_arguments() -> Vec<String>' 'pub async fn retire_within(&mut self, deadline: Duration) -> Result<String>' '    retired: bool,' '    failed: Option<(usize, String)>,' '    turns_started: usize,' '    stream: Option<Stream>,' 'pub async fn shutdown(&mut self) -> StopReport' 'fn exit_cause(status: std::process::ExitStatus) -> String' '    groups: BTreeMap<i32, Option<u64>>,' '    closed: bool,'; do grep -cF "$s" $P/src/lib.rs; done
grep -cE 'fn process\((pid|_): i32\) -> io::Result<Option<Process>>' $P/src/lib.rs
tr '\n' ' ' < $P/src/lib.rs | grep -cE 'fn client_command\( *c: &Config, *home: Option<&Path>, *args: &\[String\],? *\) -> tokio::process::Command'
tr '\n' ' ' < $P/src/lib.rs | grep -cE 'async fn logout\( *c: &Config, *home: &Path, *tree: &ClientTree, *deadline: Duration,? *\) -> Result<\(\)>'
for s in 'struct Wrapper(Arc<Mutex<Session>>, bool);' '"name":"retire"' 'if r.name == "retire" && self.1 {'; do grep -cF "$s" $P/src/main.rs; done
grep -q 'logouts.jsonl' $P/tests/fixtures/client.rs; echo "logout fixture $?"
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs $P/tests/wrapper.rs
for s in 'async fn v2_retire_tool_through_the_wrapper()' 'const LOGIN_TOKEN: &str = "synthetic-login-token-canary";' 'const LOGIN_CLAIM: &str = "synthetic-claim-canary";' 'fn plant_default_home(' 'fn entry_names(' 'fn assert_default_home(' 'fn assert_client_environment('; do grep -cF "$s" $P/tests/wrapper.rs; done
diff packages/agent-codex/src/lib.rs $P/src/lib.rs | grep -c '^[<>]'
diff packages/agent-codex/src/main.rs $P/src/main.rs | grep -c '^[<>]'
diff packages/agent-codex/tests/fixtures/client.rs $P/tests/fixtures/client.rs | grep -c '^[<>]'
test ! -e $P/src/sign_in.rs; echo "module $?"
grep -c '^#\[cfg(test)\]$' $P/src/lib.rs
grep -cE 'account-hook|sign-out-hook' $P/conduit.yaml $P/contracts.yaml
grep -c '^  signIn:' $P/contracts.yaml
grep -cE 'sign_in|statuses\.jsonl|status-output|"name":"account"|"name":"sign-out"|SIGNED_OUT|fn write_signed_in_login|fn hook_lines|fn assert_no_hook_canary' $P/src/lib.rs $P/src/main.rs $P/tests/wrapper.rs $P/tests/fixtures/client.rs
grep -c '^## The .v2 retire hook$' $P/README.md
tr '\n' ' ' < $P/README.md | grep -c 'The plain MCP wrapper exposes `turn`, and each `.v2` item also `retire`; the manifest maps each versioned operation or hook to its tool\.'
python3 -c "import json; c=[x for x in json.load(open('experiments/027-a5-sign-in/results-2026-11-16.json'))['clients'] if x['client'] == 'claude-code'][0]; s=c['steps']; print(c['version'], [(s[k]['exit'], s[k]['login'], s[k]['account_field']) for k in ('2', '3', '5', '6')])"
for s in 'serde_json::from_str::<Value>(stdout.trim())' 'path.join(".")' 'matches!(label.as_str(), "loggedin" | "signedin")'; do grep -cF "$s" experiments/027-a5-sign-in/client-facts/src/lib.rs; done
```

They print, in order:

- `main 0` (`d891543f` is `main` with the texts of orders 050f, 050h and 048a-4 and this plan's
  rows), `head 0`, `order 0` and `0`; then `1` for each of the six order files (their Records,
  050f's and 050h's among them, both at `f817139e`). The lines below, not a Record, check what
  orders 050f and 050h brought.
- From order 050f, with order 050h's `requires` and default (one line):

  ```text
  1 {'type': ['string', 'null']} False False {'directory': '/directory'} ['github.com/shrav-k/conduit/core#retire-hook.v1'] {'#turn.v1': '#wrapper-v2', 'github.com/shrav-k/conduit/core#retire-hook.v1': '#wrapper-v2'}
  ```

  (`account` required, a string or null with no default, no `apiKey`, no `signIn`,
  `toolsDirectory`, `retire-hook.v1` under `requires` with the turn item as its default).
- From order 050h: `1` and `1` (the `wrapper-v2` item implements and serves the retire hook);
  `1` for each of the twelve strings of `lib.rs` (050h's constants, `logout_arguments`,
  `retire_within` and `retired`, and the fields and functions at `BASE` that this order uses); `3`
  (the three platform forms of `process`); `1` (050h's `client_command`); `1` (050h's `logout`);
  `1` for each of the three strings of `main.rs`; `logout fixture 0`; `18` for each
  `wrapper.rs`; and `1` for each of the seven strings of the test file.
- The two packages still one source: `2` (the `CLAUDE` line), `0` and `7` (Claude Code's
  `STALE_INIT` block).
- Nothing of this order yet: `module 0`; `1` (one `#[cfg(test)]` line, before `mod tests`); `0`
  for each of the two YAML files; `0`; `0` for each of the four sources; then `1` and `1` (050h's
  README section and opening sentence).
- The sources of the status's form (see "Where the status's form comes from" below): stage 0's
  facts for Claude Code (one line)

  ```text
  2.1.318 [(0, 'yes', 'email'), (1, 'no', None), (1, 'no', None), (0, 'yes', 'email')]
  ```

  (version 2.1.318; steps 2 and 6, a signed-in home, exit 0 with a login and the account in the
  field `email`; steps 3 and 5, an empty home, exit 1 with no login); then `1`, `2` and `1` (the
  probe applies its JSON rules where standard output parses as a JSON object, writes a JSON field
  as its keys joined by dots, as printed, and reads a login from a boolean whose key is
  `loggedin` or `signedin` once lowercased with everything but letters removed).

**The status's form, checked first.** After the lines above and before you change anything,
compare the scripted client's own status output, as decision 4 sets it where `status-output`,
`status-exit` and `status-hang` are absent, with what the sources quoted under "Where the status's
form comes from" state: a JSON object on standard output with no flag given; exit 0 with a login and
1 without; `authMethod` one of the reference's six values; a `configDirectory` member; a login read
from a boolean whose key, lowercased with everything but letters removed, is `loggedin` or
`signedin` (the fixture's `loggedIn` is one such key), true with a login and false without; and the
account in a top-level `email`. Where decision 4's status or decision 2's `status_arguments()`
differs from those sources in any of these, or the sources as quoted there disagree with each other
or with the start check's lines, stop, change nothing and report the difference, quoting each
source. That the key is spelled `loggedIn` exactly is stated by no source; it is not a difference,
and the hook fails closed on any other spelling.

**What it builds.** Two of the three sign-in hooks that `claude-code.v2` will require, served by
the `.v2` item and not yet required by the Type: the account hook, `account-hook.v1`, which reads
Claude Code's own status in the Agent's own home and reports the account it names, or null where
it reports no login; and the sign-out hook, `sign-out-hook.v1`, which signs the client out of that
home with the client's own logout, as order 050h's retire hook does, and keeps the home. Both are
plain tools of the `wrapper-v2` item, `account` and `sign-out`, and hold no client output, claim or
token in any answer. The Type gains no `requires` entry, no default and no `signIn` here: order
050j adds them together, since CTR-013 asks for all three hooks with `signIn`. Until then the
items are unused on this unpublished branch, which harms nothing (the coordinator's reading, in
its split of 050). The two hooks stay together in this one order, rather than one behavior per
order as [FR-269](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-269) (D6) asks,
as the plan's 050i entry pairs them (the coordinator's, open to the owner's strike).

It carries, from [the coordinator's plan of 050](117-plan-s2-s3.md) (order 117's 050 entry: "050i,
claude-code.v2's account and sign-out hooks. After 050h, and 048a-4 written"), the 050i entry of
the coordinator's split:

- "The account hook: reads the client's status in the Agent's home, which names the account (A5
  stage 0), and reports only that". [A5's stage 0](../../experiments/027-a5-sign-in/results-2026-11-16.md#pass)
  found that Claude Code 2.1.318's status, `claude auth status`, reported a login and named the
  account in its field `email` in a signed-in home, exiting 0, and reported no login in an empty
  home, exiting 1.
- "The sign-out hook (undo), reusing 050h's sign-out, as 050g's outcome says": order 050g's
  outcome for Claude Code is a pass, so the hook runs `claude auth logout` with order 050h's
  `logout`, in the Agent's home alone.
- From the targeted review's
  [`claude-code.v2` list](../proposals/review-signin-2026-11-16/README.md#050-claude-codev2), which
  the owner accepted with [ruling 82](../proposals/ruling-82/README.md): item 4, its hook part
  ("Every `.v2` launch and hook runs the client with `CLAUDE_CONFIG_DIR` inside
  `CONDUIT_STATE_DIR` [...] Done when a marker in a synthetic default home stays untouched after an
  undo, a retirement and a turn": the undo is this order's); and item 5, its hook clauses ("No
  client output, URL, claim or token in any result, diagnostic, log or record [...] a fake home
  with a canary token and claims driven through every error path, each canary absent from every
  result, diagnostic, record and log"). The URL canary and item 1's `requires` are order 050j's.
- Ruling 82's o1 ([point 6](../proposals/ruling-82/README.md#the-decisions)): "the account,
  sign-out and retire hooks need no keys", so both items stay `plain`.

**Where the status's form comes from.** Claude Code's
[CLI reference](https://code.claude.com/docs/en/cli-reference#cli-commands), section "CLI
commands", the row `claude auth status`, read on 2026-11-18, says: "Show authentication status as
JSON. Use `--text` for human-readable output. Exits with code 0 if logged in, 1 if not. The JSON
includes a `configDirectory` field naming the configuration directory the CLI uses. The field
requires Claude Code v2.1.268 or later. The JSON's `authMethod` field is one of `none`,
`claude.ai`, `oauth_token`, `api_key`, `api_key_helper`, or `third_party`". So JSON is the
default, and `status_arguments()` gives no flag. That page describes the current release, not
2.1.318 alone, and it names neither the login's key nor `email`; Claude Code's
[authentication page](https://code.claude.com/docs/en/authentication), read the same day, does
not describe the status's output. Stage 0 at 2.1.318 gives the rest
([what the probe records](../../experiments/027-a5-sign-in/README.md#what-is-recorded), and its
[results](../../experiments/027-a5-sign-in/results-2026-11-16.json)): its probe applies its JSON
rules wherever standard output parses as a JSON object, and writes a JSON field as its keys joined
by dots, as printed. With the output JSON, the account was therefore in a top-level key `email`,
and the login a boolean whose key, lowercased with everything but letters removed, is `loggedin`
or `signedin`, true in the signed-in home (exit 0) and false in the empty one (exit 1). The text
rule, which also writes `email` for a line labelled "Email", does not apply to a JSON output. The
key's exact spelling, `loggedIn`, and its place at the top level are stated by no source: they are
the coordinator's assumption, since the probe finds the boolean at any depth. The hook fails
closed on them and on any other output it cannot parse, answering
`client status unreadable`, so that a status of another form completes no sign-in; a real
client's closeout, not this order, confirms it. The start check reads stage 0's facts and the
probe's rules, and "The status's form, checked first" stops you where the scripted client's
status differs from these sources.

**What the specification already says, so that nothing here changes it.**
[`spec/core/operations.yaml`](../../spec/core/operations.yaml): `account-hook.v1` "Report[s] the
account the instance client is signed in to, or null when none, without reading it through the
engine (CTR-013, PND-011). A part of sign-in.v1 or setup only, dispatched through no Conduit,
starting no model work and changing nothing outside itself", with no input and the output
`{account: string or null}`, required; `sign-out-hook.v1` "Remove[s] the instance client login
from where its client keeps it", with no input and no output.
[CTR-013](../../spec/promises/contracts.yaml): "A Type states signIn only where account-hook.v1
reports an account attested by the service: returned at login, such as in an ID token or profile,
or reported when asked"; "The engine reads only what account-hook reports, never the instance's
state directory". [PND-011](../../spec/promises/pending.yaml): "Only the expected account,
compared ignoring ASCII case, completes sign-in"; "A failed or unknown account hook does not
complete sign-in"; a different account "is undone by a sign-out-hook.v1 part"; "If sign-out does
not succeed, the engine reports SignInUndone=False" and holds the instance, which order 048a-4
builds. `spec/interface/implementation.yaml`: a plain tool's "`arguments` are the operation's
inputs, and the result's `structuredContent` is the" operation's outputs. So the account tool
takes no arguments and answers `{"account": ...}` as `structuredContent`; the sign-out tool takes
no arguments and answers success or an error; and the engine compares and decides, never the
hook.

**What the branch already does, so that you do not redo it.** At `BASE`, in
`packages/agent-claude-code/`, as the start check shows:

- `src/lib.rs` (one source with Codex's but for line 18's `CLAUDE`) has `client_home`, which
  checks and creates the home with every launch's refusals; `client_command` (order 050h), which
  builds the client's command with the launch's environment, `CLAUDE_CONFIG_DIR` set to the home,
  both API-key variables removed, the client directory as its working directory, its own process
  group and `kill_on_drop`; `logout` (order 050h), which runs `logout_arguments()` (here
  `["auth", "logout"]`) under the tree's state lock, in the client directory or, where that is
  gone, in the wrapper's working directory or else the home (050h's Record, R1), records its
  group, waits within its deadline, stopping the tree at it, and fails with
  `client logout did not start`, `client logout failed: client exit: ...` or
  `client logout failed: no exit within N seconds`; `exit_cause`; `process`; the tree's state with
  `groups` and `closed`; and `Session` with `config`, `tree`, `stream`, `failed`, `turns_started`
  and `retired`, `shutdown`, and `retire_within`, which reads the configuration's refusal only
  where the state directory is unset and, after the logout, checks the home again before emptying
  it (050h's Record, R8).
- `src/main.rs`'s `Wrapper(Arc<Mutex<Session>>, bool)`, the second field `config.v2`, lists `turn`
  and, for `.v2`, `retire`; its `call_tool` serves `retire` before `turn` and answers any other name
  with `ErrorData::invalid_params("unknown tool", None)`.
- The scripted client (`tests/fixtures/client.rs`) reads its arguments first, serves order 050h's
  logout before it writes `client-started` (appending a line to `logouts.jsonl` in its working
  directory, and removing `fixture-login` from the home), and otherwise runs turns.
- `tests/wrapper.rs` has 18 tests, among them order 050h's `v2_retire_tool_through_the_wrapper`,
  whose three `.v2` wrapper runs (no named directory; a named directory removed after a turn; the
  same with a logout exit 3) share one tool-list assertion, `tests/wrapper.rs:2126`, and the
  helpers `plant_default_home`, `entry_names`, `assert_default_home` and
  `assert_client_environment`, and the constants `LOGIN_TOKEN` and `LOGIN_CLAIM`. Order 050h also
  added `plant_login`, `assert_no_login_canary` and `recorded_logouts`, which this order leaves as
  they are.

## The decisions it applies

Every change is in `packages/agent-claude-code/`. Nothing in `packages/agent-codex/` changes: the
Codex lane's orders 050l and 050m write that package's hooks, and order 050n joins the two sources
again (see "Who decided what").

1. **The item.** In `conduit.yaml`, in `wrapper-v2` alone:
   - its `implements` line becomes

     ```yaml
         implements: ['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1', 'github.com/shrav-k/conduit/core#account-hook.v1', 'github.com/shrav-k/conduit/core#sign-out-hook.v1']
     ```

   - and under `tools`, after the `retire` line, add the two lines

     ```yaml
           'github.com/shrav-k/conduit/core#account-hook.v1': account
           'github.com/shrav-k/conduit/core#sign-out-hook.v1': sign-out
     ```

   `plain: true` stays, and nothing else in the manifest changes. `contracts.yaml` does not change.

2. **The module.** In `src/lib.rs`, add exactly one line, `pub mod sign_in;`, immediately before
   the line `#[cfg(test)]`, with no blank line added. Create `src/sign_in.rs` holding the code
   below, formatted with `cargo fmt`. A child module may use its parent's private items, so
   `lib.rs` changes in that line alone.

   ```rust
   //! claude-code.v2's account and sign-out hooks (order 050i): Claude Code's own status and
   //! logout, run in this Agent's own home alone.
   use super::{
       ClientTree, Config, LOGOUT_DEADLINE, Session, client_command, client_home, exit_cause,
       logout, process,
   };
   use anyhow::{Result, anyhow, bail, ensure};
   use serde_json::Value;
   use std::{path::Path, process::Stdio, time::Duration};
   use tokio::{io::AsyncReadExt, time::timeout};

   /// How long the account hook waits for the client's status (A5 stage 0's limit).
   pub const STATUS_DEADLINE: Duration = Duration::from_secs(30);
   /// The most bytes of the status's standard output that the account hook reads.
   pub const STATUS_LIMIT: u64 = 65_536;
   pub const STATUS_DID_NOT_START: &str = "client status did not start";
   pub const STATUS_UNREADABLE: &str = "client status unreadable";
   pub const STATUS_NO_ACCOUNT: &str = "client status names no account";
   /// The sign-out hook's answer once the client is signed out.
   pub const SIGNED_OUT: &str = "client signed out of this Agent's own home";
   /// The cause a sign-out gives a conversation whose client it stopped.
   pub const SIGNED_OUT_CAUSE: &str = "client signed out";

   /// The client's own status, run with the Agent's home alone (A5 stage 0).
   pub fn status_arguments() -> Vec<String> {
       vec!["auth".into(), "status".into()]
   }

   /// The account the status names, or None where it reports no login. Its errors are fixed
   /// texts: no byte of the client's output reaches them.
   async fn account_status(
       c: &Config,
       home: &Path,
       tree: &ClientTree,
       deadline: Duration,
   ) -> Result<Option<String>> {
       let (mut child, pid) = {
           let mut state = tree.0.lock().unwrap();
           ensure!(!state.closed, "{STATUS_DID_NOT_START}");
           let child = client_command(c, Some(home), &status_arguments())
               .stdin(Stdio::null())
               .stdout(Stdio::piped())
               .stderr(Stdio::null())
               .spawn()
               .map_err(|_| anyhow!("{STATUS_DID_NOT_START}"))?;
           let pid = child.id().ok_or_else(|| anyhow!("{STATUS_DID_NOT_START}"))? as i32;
           let start = process(pid).ok().flatten().map(|p| p.start);
           state.groups.insert(pid, start);
           (child, pid)
       };
       let stdout = child
           .stdout
           .take()
           .ok_or_else(|| anyhow!("{STATUS_DID_NOT_START}"))?;
       let finished = timeout(deadline, async {
           let mut output = Vec::new();
           // `take` consumes the pipe, so a longer output cannot block the wait.
           let read = stdout.take(STATUS_LIMIT + 1).read_to_end(&mut output).await;
           (read.map(|_| output), child.wait().await)
       })
       .await;
       let Ok((output, status)) = finished else {
           // The status's own group alone: a conversation's client, if one runs, is not stopped.
           unsafe { libc::kill(-pid, libc::SIGKILL) };
           let _ = child.kill().await;
           bail!(
               "client status failed: no exit within {} seconds",
               deadline.as_secs()
           );
       };
       let output = match output {
           Ok(output) if output.len() as u64 <= STATUS_LIMIT => output,
           _ => bail!("{STATUS_UNREADABLE}"),
       };
       let status =
           status.map_err(|_| anyhow!("client status failed: client exit: status unknown"))?;
       if !matches!(status.code(), Some(0 | 1)) {
           bail!("client status failed: {}", exit_cause(status));
       }
       let reading: Value =
           serde_json::from_slice(&output).map_err(|_| anyhow!("{STATUS_UNREADABLE}"))?;
       match (
           status.code(),
           reading.get("loggedIn").and_then(Value::as_bool),
       ) {
           (Some(0), Some(true)) => match reading.get("email").and_then(Value::as_str) {
               Some(account) if !account.is_empty() => Ok(Some(account.to_owned())),
               _ => bail!("{STATUS_NO_ACCOUNT}"),
           },
           (Some(1), Some(false)) => Ok(None),
           _ => bail!("{STATUS_UNREADABLE}"),
       }
   }

   impl Session {
       /// The account hook: the account the client's status names in this Agent's own home.
       pub async fn account(&self) -> Result<Option<String>> {
           self.account_within(STATUS_DEADLINE).await
       }
       /// The account hook, with the status's deadline given; `account` gives STATUS_DEADLINE.
       pub async fn account_within(&self, deadline: Duration) -> Result<Option<String>> {
           ensure!(self.config.v2, "account hook: a .v1 Agent has none");
           if self.config.state_dir.is_none()
               && let Some(error) = &self.config.configuration_error
           {
               bail!("{error}");
           }
           let home = client_home(&self.config)?;
           account_status(&self.config, &home, &self.tree, deadline).await
       }
       /// The sign-out hook: the client's own logout in this Agent's own home, the home kept.
       pub async fn sign_out(&mut self) -> Result<String> {
           self.sign_out_within(LOGOUT_DEADLINE).await
       }
       /// The sign-out hook, with the logout's deadline given; `sign_out` gives LOGOUT_DEADLINE.
       pub async fn sign_out_within(&mut self, deadline: Duration) -> Result<String> {
           ensure!(self.config.v2, "sign-out hook: a .v1 Agent has none");
           if self.config.state_dir.is_none()
               && let Some(error) = &self.config.configuration_error
           {
               bail!("{error}");
           }
           if self.stream.is_some() && self.failed.is_none() {
               self.failed = Some((self.turns_started, SIGNED_OUT_CAUSE.into()));
           }
           if let Some(note) = self.shutdown().await.note() {
               bail!("{note}");
           }
           let home = client_home(&self.config)?;
           logout(&self.config, &home, &self.tree, deadline).await?;
           Ok(SIGNED_OUT.into())
       }
   }
   ```

   So the account hook checks the home with every launch's refusals and creates it where it is
   missing; runs `claude auth status` in it with the launch's environment, its output read and never
   kept, its standard error discarded; answers the `email` that a JSON object with `loggedIn` true
   names on exit 0, as the status gives it, and null for `loggedIn` false on exit 1; and fails with
   a fixed text on anything else. Unlike the sign-out hook, which inherits `logout`'s fallback
   (050h's Record, R1), it runs the status in the client directory alone: where the named directory
   is gone, the status cannot start, and the hook fails as `client status did not start`. It leaves
   a running conversation's client running, and at its deadline kills its own process group alone,
   not the tree, since stopping the tree would stop the conversation too. The sign-out hook stops
   the conversation's client first, as order 050h's retire hook does, since a running client may
   still hold or rewrite the login; where a client was running, it ends that conversation as a
   failed turn does, so that no later turn restarts it implicitly with a new login; where no client
   was running, later turns run as before. It then runs order 050h's `logout` unchanged and keeps
   the home and every other entry in it, where the conversation's files live. Neither hook sets
   order 050h's `retired`. Where the code cannot be applied as written (a name at `BASE` differs
   from what the start check read), stop and report rather than adapt it.

3. **The tools, in `src/main.rs`.**
   - `list_tools`, where the second field is true, lists after `retire`, in this order:

     ```json
     {"name":"account","description":"Report the account the client is signed in to in this Agent's own home, or null where none.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}}
     {"name":"sign-out","description":"Sign the client out of this Agent's own home, keeping the home.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}}
     ```

     A `.v1` wrapper lists `turn` alone, as at `BASE`.
   - `call_tool` begins, before order 050h's `retire` block:

     ```rust
     if (r.name == "account" || r.name == "sign-out") && self.1 {
         if r.arguments.as_ref().is_some_and(|args| !args.is_empty()) {
             return Err(ErrorData::invalid_params("expected no arguments", None));
         }
         let mut session = tokio::select! {
             biased;
             _ = context.ct.cancelled() => {
                 return Err(ErrorData::internal_error("hook cancelled", None));
             }
             session = self.0.lock() => session,
         };
         // Once the hook has the session, no cancellation cuts it; each has its own deadline.
         let answer = if r.name == "account" {
             session.account().await.map(|account| {
                 let structured = json!({"account": account});
                 json!({"content":[{"type":"text","text":structured.to_string()}],"structuredContent":structured,"isError":false})
             })
         } else {
             session.sign_out().await.map(|text| {
                 json!({"content":[{"type":"text","text":text}],"isError":false})
             })
         };
         let result = match answer {
             Ok(value) => {
                 let mut result = serde_json::from_value::<CallToolResult>(value).unwrap();
                 result.result_type = Some(ResultType::COMPLETE);
                 result
             }
             Err(error) => CallToolResult::error(vec![ContentBlock::text(error.to_string())]),
         };
         return Ok(result.into());
     }
     ```

     A `.v1` wrapper answers `account` and `sign-out` as any unknown tool. Each answer has one
     text block and no `_meta`; only the account's success has `structuredContent`. A turn in
     flight holds the session, and the hook waits for it. `run`, the TERM task and the end of input
     do not change, and `main.rs` imports nothing new. Order 050l may take this block as it is,
     with `Session::account` and `Session::sign_out` of its own, so that the two `main.rs` stay one
     file; that is its order's choice.

4. **The scripted client**, `tests/fixtures/client.rs`. In `main`, after order 050h's logout
   branch and before `client-started` is written, where `args == ["auth", "status"]`, call
   `status()` and return. Add `fn status()`, which:
   - reads `CLAUDE_CONFIG_DIR`, and panics with `status without the client home` where it is
     unset, so that a status never reads a default home;
   - appends one line to `statuses.jsonl` in its working directory, the JSON object
     `{"args":["auth","status"],"home":HOME,"environment":ENV,"entries":ENTRIES,"cwd":CWD}`, `ENV`
     its environment as an object of names to values, as the turn's evidence gives `environment`,
     and `ENTRIES` the names in the home, sorted;
   - where `status-hang` is in its working directory, sleeps 30 seconds;
   - reads `fixture-login` in the home as JSON, where it is there;
   - writes to its standard output the bytes of `status-output` in its working directory where
     that file is there, and otherwise one JSON object: with a login,
     `{"loggedIn":true,"authMethod":"claude.ai","configDirectory":HOME,"orgName":CLAIM,"accessToken":TOKEN}`,
     HOME the value of `CLAUDE_CONFIG_DIR`, CLAIM the login's `claims.account` and TOKEN its
     `token`, with the member `"email"` added holding the login's `email` where it has one;
     without one, `{"loggedIn":false,"authMethod":"none","configDirectory":HOME}`. `loggedIn`,
     `authMethod`, `configDirectory` and `email` follow "Where the status's form comes from";
     `orgName` and `accessToken` are the fixture's canaries, not members that a source states;
   - writes to its standard error `status error ` followed by the login's `token`, or nothing
     (client output that the wrapper discards);
   - exits with the status that `status-exit` in its working directory holds where that file is
     there, and otherwise with 0 where there is a login and 1 where there is none.

   It never writes in the home. Nothing else in the file changes.

5. **The tests**, in `tests/wrapper.rs`. Add the constant `ACCOUNT` `Synthetic-Account`; import
   `SIGNED_OUT`, `SIGNED_OUT_CAUSE`, `STATUS_DID_NOT_START`, `STATUS_NO_ACCOUNT`,
   `STATUS_UNREADABLE` and `status_arguments` from `agent_adapter::sign_in`; and add three helpers:
   `write_signed_in_login(home: &Path, email: Option<&str>)`, which writes `fixture-login` in `home`
   holding `{"token":LOGIN_TOKEN,"claims":{"account":LOGIN_CLAIM}}`, with the member `"email"`
   holding `email` where it is `Some`, by calling order 050h's `plant_login` where `email` is
   `None`; `hook_lines(path: &Path) -> Vec<Value>`, the JSON lines of a `.jsonl` file, empty where
   the file is missing, used for `statuses.jsonl`, while `logouts.jsonl` is read with order 050h's
   `recorded_logouts`; and `assert_no_hook_canary(text: &str)`, which calls order 050h's
   `assert_no_login_canary` and asserts that `text` does not hold `ACCOUNT` either. "Set up" below
   means: a fresh temporary directory with the default home planted (`plant_default_home`),
   `config(...)` as order 050h's tests build it with `c.v2` true and `directory` null,
   `MCP_TOOL_TIMEOUT` `10000` in `c.env`, `prepare` run, and the client home `state/client-home`
   made with mode 0700; "the launch's environment" is `c.env` less both API-key variables with
   `CLAUDE_CONFIG_DIR` the home's canonical path, compared with `assert_client_environment`; and the
   client directory is `c.cwd`.
   - **Two tests change in their tool lists alone.** In
     `stdio_v2_defaults_and_invalid_configuration_remain_callable`, where order 050h's
     assertion expects the names `["turn", "retire"]` (`tests/wrapper.rs:1405` at `BASE`) it
     expects `["turn", "retire", "account", "sign-out"]`, and where it expects `["turn"]` it does
     not change. In `v2_retire_tool_through_the_wrapper`, the listed names of its `.v2` runs, which
     one assertion checks in each of the three (`tests/wrapper.rs:2126`), become
     `["turn", "retire", "account", "sign-out"]`; its `.v1` list stays `["turn"]`. Every other test
     keeps passing unchanged.
   - **A new test, `v2_account_reports_only_the_status_account`**, through `Session`. Set up, with
     no login. Run the turn `HOME`, which succeeds, and remove `client-started` from the client
     directory. `account()` answers `Ok(None)`. Write `write_signed_in_login(home, Some(ACCOUNT))`;
     `account()` answers `Ok(Some(ACCOUNT))` exactly. `statuses.jsonl` holds two lines, each with
     `args` equal to `status_arguments()`, `environment` the launch's environment and `cwd` the
     client directory; their `entries` are `["fixture-home-write"]` and
     `["fixture-home-write", "fixture-login"]`. The home's entries are then
     `["fixture-home-write", "fixture-login"]`, `fixture-login`'s bytes are those written, and the
     home's mode is 0700. The turn `HOME` runs again and succeeds, and `client-started` is still
     absent (the account hook neither stopped nor restarted the conversation's client). The
     default home passes `assert_default_home`. It `eprintln!`s each line's `args` and `entries`.
   - **A new test, `v2_account_error_paths_hold_no_canary`**, through `Session`, one case at a
     time. Each case is set up, the home holding `write_signed_in_login(home, Some(ACCOUNT))`
     unless the case says otherwise; files named in "Set up" are written in the client directory.
     Each call is `account()` but in `deadline`.

     | Case | Set up | Answer | Status lines |
     | --- | --- | --- | --- |
     | `no-email` | the login written with `None` | `STATUS_NO_ACCOUNT` | 1 |
     | `empty-email` | the login written with `Some("")` | `STATUS_NO_ACCOUNT` | 1 |
     | `not-json` | `status-output` holding `Logged in as ` then `ACCOUNT`, ` token ` and `LOGIN_TOKEN` | `STATUS_UNREADABLE` | 1 |
     | `no-flag` | `status-output` holding `{"email":ACCOUNT,"accessToken":LOGIN_TOKEN}` | `STATUS_UNREADABLE` | 1 |
     | `logged-in-exit-1` | `status-exit` holding `1` | `STATUS_UNREADABLE` | 1 |
     | `logged-out-exit-0` | no login; `status-exit` holding `0` | `STATUS_UNREADABLE` | 1 |
     | `exit` | `status-exit` holding `2` | `client status failed: client exit: status 2` | 1 |
     | `too-long` | `status-output` holding the JSON object `{"loggedIn":true,"email":ACCOUNT,"accessToken":LOGIN_TOKEN,"padding":X}`, `X` 70,000 `x` | `STATUS_UNREADABLE` | 1 |
     | `deadline` | `status-hang`; the call is `account_within(Duration::from_secs(3))` | `client status failed: no exit within 3 seconds` | 1 |
     | `no-client` | `c.program` a missing file under the temporary directory | `STATUS_DID_NOT_START` | 0 |
     | `outside-link` | the home a link to `elsewhere`, a directory beside `state` holding the login | `client home (CLAUDE_CONFIG_DIR): the home resolves outside CONDUIT_STATE_DIR` | 0 |
     | `unset` | `c.state_dir` `None` | `client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is unset` | 0 |
     | `configuration-unset` | `c.state_dir` `None`, `c.configuration_error` `invalid configuration: client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is unset` | that configuration text | 0 |
     | `configuration` | `c.configuration_error` `invalid configuration: effort (AGENT_EFFORT): expected lowercase ASCII letters` | `Ok(Some(ACCOUNT))` | 1 |

     Each answer is compared exactly: the error's text, or the `Ok` value. In every case but
     `configuration`, the error's text passes `assert_no_hook_canary`. In each case: the lines of
     `statuses.jsonl` in the client directory are as many as the table says; `client-started` is
     absent from the client directory; the home (in `outside-link`, `elsewhere`) holds the entries
     it was set up with and `fixture-login`'s bytes as written, where it holds one; and the default
     home passes `assert_default_home`. In `deadline`, the call returns within 10 seconds of its
     start. It `eprintln!`s each case's answer.
   - **A new test, `v2_sign_out_keeps_the_home_and_ends_a_running_conversation`**, through
     `Session`, in two parts, each set up afresh.
     - With no client running: the home holds `write_signed_in_login(home, Some(ACCOUNT))` and
       `keep` holding `kept`. `sign_out()` answers `SIGNED_OUT`. `logouts.jsonl` in the client
       directory holds one line, with `args` equal to `logout_arguments()`, `environment` the
       launch's environment, `cwd` the client directory and `entries` `["fixture-login", "keep"]`.
       The home's entries are then `["keep"]`, `keep` still holds `kept`, and the home's mode is
       0700. `account()` answers `Ok(None)`. The turn `HOME` then succeeds (no client was
       running, so no conversation ended).
     - With a client running: run the turn `HOME`, which succeeds; write
       `write_signed_in_login(home, Some(ACCOUNT))`; remove `client-started` from the client
       directory. `sign_out()` answers `SIGNED_OUT`; the home's entries are then
       `["fixture-home-write"]`. The turn `TEST` fails with exactly
       `session failed at turn 1: client signed out; implicit restart refused`, and
       `client-started` stays absent. `account()` answers `Ok(None)`.

     In both, the default home passes `assert_default_home` (the undo part of item 4's marker
     test). It `eprintln!`s each logout line's `args` and `entries`.
   - **A new test, `v2_sign_out_error_paths_hold_no_canary`**, through `Session`, one case at a
     time, each set up with the home holding `write_signed_in_login(home, Some(ACCOUNT))`. Each call
     is `sign_out()` but in `deadline`.

     | Case | Set up | Answer | Logout lines | Login after |
     | --- | --- | --- | --- | --- |
     | `no-client` | `c.program` a missing file under the temporary directory | `client logout did not start` | 0 | kept |
     | `exit` | `logout-exit` holding `3` in the client directory | `client logout failed: client exit: status 3` | 1 | kept |
     | `deadline` | `logout-hang` in the client directory; the call is `sign_out_within(Duration::from_secs(3))` | `client logout failed: no exit within 3 seconds` | 1 | kept |
     | `outside-link` | the home a link to `elsewhere`, a directory beside `state` holding the login | `client home (CLAUDE_CONFIG_DIR): the home resolves outside CONDUIT_STATE_DIR` | 0 | kept in `elsewhere` |
     | `unset` | `c.state_dir` `None` | `client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is unset` | 0 | kept |
     | `configuration-unset` | as in the account cases | that configuration text | 0 | kept |
     | `configuration` | `c.configuration_error` `invalid configuration: effort (AGENT_EFFORT): expected lowercase ASCII letters` | `SIGNED_OUT` | 1 | removed |

     Each answer is compared exactly: the error's text, or the `Ok` value. In every case but
     `configuration`, the error's text passes `assert_no_hook_canary`. "Kept" means
     `fixture-login` is still there with its bytes as written; "removed" means the home no longer
     holds it. In each case `client-started` is
     absent and the default home passes `assert_default_home`. Then, in `exit`, with `logout-exit`
     removed, a second `sign_out()` answers `SIGNED_OUT`, with one more logout line and a home with
     no entry. It `eprintln!`s each case's answer.
   - **A new test, `v2_account_and_sign_out_tools_through_the_wrapper`**, through the wrapper
     binary over stdio, its `.v2` wrapper started as order 050h's
     `v2_retire_tool_through_the_wrapper` starts its first run, with no `AGENT_DIRECTORY`, with the
     default home planted and the client home made with mode 0700 and no login. In this order:
     1. the listed tools' names are `["turn", "retire", "account", "sign-out"]`;
     2. `account` and then `sign-out`, each with the arguments `{"extra":true}`, are each an error
        of the call, and each error's text passes `assert_no_hook_canary`;
     3. `account` with `{}` answers, as JSON, `isError` false, `resultType` `complete`, `content`
        exactly one text block `{"account":null}`, `structuredContent` `{"account":null}`, and no
        `_meta` member;
     4. after `write_signed_in_login(home, Some(ACCOUNT))`, `account` with `{}` answers the same
        with `{"account":"Synthetic-Account"}` as both the text and `structuredContent`;
     5. the turn `TEST` succeeds;
     6. `sign-out` with `{}` answers `isError` false, `resultType` `complete`, `content` exactly one
        text block `SIGNED_OUT`, and no `structuredContent` and no `_meta` member;
     7. the turn `TEST` answers `isError` true, `resultType` `complete` and one text block,
        `session failed at turn 1: client signed out; implicit restart refused`;
     8. `account` with `{}` answers `{"account":null}` as in step 3.

     `statuses.jsonl` in the client directory then holds three lines, each with `args` equal to
     `status_arguments()` and `environment[CLAUDE_CONFIG_DIR]` the client home's canonical path;
     `logouts.jsonl` holds one line, with `args` equal to `logout_arguments()`; and the default home
     passes `assert_default_home`. The test keeps the wrapper's raw standard output, as order 050h's
     test does. After `cancel`, the wrapper exits successfully; its stderr holds none of
     `LOGIN_TOKEN`, `LOGIN_CLAIM`, `ACCOUNT`, `synthetic-test-token`, `synthetic-anthropic-key` and
     `synthetic-openai-key`; and its raw stdout holds none of `LOGIN_TOKEN`, `LOGIN_CLAIM`,
     `synthetic-test-token`, `synthetic-anthropic-key` and `synthetic-openai-key` (not `ACCOUNT`,
     which step 4 rightly answers). Then a `.v1` wrapper, started in a fresh temporary directory as
     `stdio_exposes_only_turn_and_setup_never_launches_client` starts it, lists `["turn"]`, answers
     `account` and `sign-out`, each with `{}`, as errors of the call, and leaves neither
     `statuses.jsonl` nor `logouts.jsonl`. It `eprintln!`s the `.v2` answers of steps 4 and 6.

   The test count in `packages/agent-claude-code/tests/wrapper.rs` goes from 18 to 23; Codex's
   stays 18.

6. **The README**, `packages/agent-claude-code/README.md` alone. Write no absolute local path, and
   no client version beyond those the README and the stage 0 results already record.
   - In the opening paragraph, leave order 050h's sentence "The plain MCP wrapper exposes `turn`,
     and each `.v2` item also `retire`; the manifest maps each versioned operation or hook to its
     tool." as it is (order 203 starts from it), and add after it: "In this package the `.v2` item
     also exposes `account` and `sign-out`, the account and sign-out hooks, which the Type does not
     require yet.", "account and sign-out hooks" a link to the anchor
     `the-v2-account-and-sign-out-hooks`.
   - Add, at the end, `## The .v2 account and sign-out hooks`, which says, in this order:
     - the `wrapper-v2` item implements `account-hook.v1` and `sign-out-hook.v1`, served as the
       tools `account` and `sign-out`, which take no arguments; the `.v1` item lists neither;
       `claude-code.v2` does not yet state `signIn` or list them under `requires`, so no template
       takes them by default, and the engine dispatches them only as parts of `sign-in.v1` or of a
       setup, for a Type that states `signIn` (CTR-013, PND-011);
     - the account hook's three steps, as a numbered list: it checks the home as every launch does,
       with the same refusals, creating it where it is missing; it runs `claude auth status` with
       `CLAUDE_CONFIG_DIR` set to the home and the launch's environment, reads at most 65,536
       bytes of its standard output, discards its standard error, and waits at most 30 seconds;
       and it answers `{"account": A}` where the status exits 0 and its output is a JSON object
       whose `loggedIn` is true and whose `email` is a non-empty string A, reported as the status
       gives it, and `{"account": null}` where it exits 1 and `loggedIn` is false. It leaves a
       running conversation's client running and keeps nothing of the output;
     - its failures, each a fixed text: `client status did not start`, also where the named
       directory is gone, since the status, unlike the sign-out hook's logout, runs in the client
       directory alone;
       `client status failed: client exit: status N`, `... signal N` or `... status unknown`;
       `client status failed: no exit within 30 seconds`, after which the status's own process
       group is killed; `client status unreadable`, for an output longer than the limit, not a JSON
       object, without a boolean `loggedIn`, or whose `loggedIn` disagrees with the exit status;
       and `client status names no account`, for a login with no non-empty `email`; with the home's
       refusals, and the configuration's refusal where the state directory is unset or not
       absolute. Its answers hold fixed words and numbers and, on success, the account alone;
     - the sign-out hook's three steps, as a numbered list: it stops the client's process tree as a
       failed turn does, and where a conversation's client was running, it ends that conversation,
       every later turn refused with `session failed at turn K: client signed out; implicit
       restart refused`, and where a process outlived the stop or the stop is unconfirmed it fails
       with the stop's note; it checks the home as the account hook does; and it runs
       `claude auth logout` as the retire hook does, waiting at most 60 seconds, and answers
       `client signed out of this Agent's own home`, "the retire hook" a link to the anchor
       `the-v2-retire-hook`. It keeps the home and
       every other entry in it, unlike the retire hook; its failures are the retire hook's logout
       failures, and a failure before the logout's exit leaves the login in place;
     - where the status's reading comes from: Claude Code's
       [CLI reference](https://code.claude.com/docs/en/cli-reference#cli-commands), section "CLI
       commands", read on 2026-11-18, says that `claude auth status` shows the status as JSON
       (`--text` gives text instead), exiting 0 if logged in and 1 if not, with an `authMethod`
       field; [A5's stage 0](../../experiments/027-a5-sign-in/results-2026-11-16.md#pass) recorded
       Claude Code 2.1.318's status naming the account in the field `email`, exiting 0 with a login
       and 1 in an empty home; with the output JSON, as the reference says, that is a top-level key;
       neither source spells the login's key or says where it sits, which the hook reads as a
       top-level boolean `loggedIn`; the hook reads the status as a JSON object with `loggedIn` and
       `email`, and any other form fails as `client status unreadable`, so a status whose form
       differs completes no sign-in; the engine, not the hook, compares the account with the
       expected one, ignoring ASCII case (PND-011);
     - and, as limits: the status is the client's report of its stored login, and the hook does not
       ask the service itself; the hook does not read the login's kind, the status's
       `authMethod`; and a sign-out with a conversation's client running ends that conversation
       for the rest of the run.

7. **Check the item with the engine**, as order 050h's decision 9 does, for this package alone.
   This checks what the package tests cannot: that the manifest, with an item implementing two
   hooks its Type does not require yet, still installs.
   - In `engine/`, run `cargo build --release`. Change nothing there.
   - In `packages/agent-claude-code/`, run `cargo run --release --bin package`, which prints
     `ARCHIVE#SHA256`.
   - In a new temporary directory T outside the repository, run
     `engine/target/release/conduit init --store T/store`, then write the project R under T: a
     `conduit.yaml` that pins `example.org/conduit/agent-claude-code` at version 0.2.0 with that
     digest, as `engine/crates/shell/tests/agents.rs` writes one, and one template file, the
     template `v2-hooks`, `kind: Agent`, `spec.type`
     `example.org/conduit/agent-claude-code#claude-code.v2`, `config: {account: example-account}`,
     with no `implementation`.
   - Register it with
     `engine/target/release/conduit register --store T/store --package ARCHIVE#SHA256 R`. It holds
     no instance, so it starts no wrapper and no client. Report the reply's `accepted` and its
     diagnostics.
   - Expected: accepted, with no diagnostic. If the reply differs, change nothing to make it pass:
     report it, since this order changes no engine source, and the coordinator decides.

8. **What this order does not do.** No `signIn`, no login, no `sign-in-hook.v1`, no URL and no
   `requires` entry or default for the two hooks, which are order 050j's; nothing about `apiKey`,
   which [ruling 83](../proposals/ruling-83/README.md) leaves out of `.v2`; no change to
   `contracts.yaml`, to `.v1`, to `turn.v1`, to the `wrapper` item, to order 050h's retire hook or
   `logout`, or to `Cargo.toml`, `Cargo.lock`, `src/bin/package.rs`, `tests/package.rs` or
   `tests/fixtures/stale-init.json`; no wording of an existing refusal changed; nothing in
   `packages/agent-codex/`; and nothing under `engine/`, `conformance/`, `spec/` or `docs/`. The
   engine's test `live_s1_scripted_agents_call_engine_with_distinct_run_identities`
   (`engine/crates/shell/tests/agents.rs`) fails on this branch by design until order 050n moves
   `conformance/fixtures/agents/` to 0.2.0, as [order 050a](050a-agents-v2-own-tools.md) records.
   It is the one expected failure on `wo/050-agents-0.2.0` and its lanes, and it is not this
   order's: neither you nor the verifier counts it against this order. Do not run the engine's
   tests and do not change it.

## Who decided what

The account hook's route, the client's own status in the Agent's home, is the coordinator's
reading of A5's stage 0 under the plan that order 117 records. That the status is a JSON object
by default, exiting 0 with a login and 1 without, is Claude Code's CLI reference's, read on
2026-11-18; that the account is in a top-level `email` is the coordinator's reading of stage 0's
`email` field under the probe's JSON rule, given that JSON output; and that the login's key is
spelled `loggedIn` and sits at the top level is the coordinator's assumption, which no source
states (the probe finds the boolean at any depth) and on which the hook fails closed (see "Where
the status's form comes from"). The hook does not read the login's kind,
the status's `authMethod`, which the reference names; checking it is left to order 050j's login,
or to a later order. The sign-out by the client's own logout is order 050g's outcome, as
order 050h applies it. Item 4's undo marker and item 5's canaries are the review's must-state
lists, which the owner accepted with ruling 82; the hooks' plain items are ruling 82's o1.
[Ruling 83](../proposals/ruling-83/README.md) (the owner's) gives the inputs that `BASE` holds. The
split of order 050 into 050b to 050o, this order's place first in the Claude lane after 050h, and
its base are the coordinator's, open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)). Everything else here is the coordinator's too,
each open to the owner's strike: the items implementing the hooks before the Type requires them;
the hooks in one Claude-only module, so that the two packages' sources differ until order 050n
joins them; the tool names `account` and `sign-out`; the status's limits of 30 seconds and 65,536
bytes; the mapping of exit status and `loggedIn` to an account, null or a failure; the login's kind
left unread; the account reported as the status gives it; killing only the status's own group at its
deadline; the sign-out stopping the client first and ending a running conversation; the home kept on
sign-out; a turn in flight waited for; no cancellation once a hook has the session; the texts; the
wording of the tools' descriptions and of the README; the scripted client's status; and the
registration probe. Apply them as written and report them under `decisions` as the coordinator's.

## Scope

Change only, in `packages/agent-claude-code/`: `conduit.yaml`, `src/lib.rs`, `src/sign_in.rs`
(new), `src/main.rs`, `tests/wrapper.rs`, `tests/fixtures/client.rs` and `README.md`. Nothing else,
in the packages or outside them. The build's ignored output (`target/`, `dist/`) may change, and so
may anything under T.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
python3 tools/set_names.py --check
git diff --check
```

Each prints what is said beside it:

```bash
P=packages/agent-claude-code
git status --short | grep -vE '^( M packages/agent-claude-code/(conduit\.yaml|src/lib\.rs|src/main\.rs|tests/wrapper\.rs|tests/fixtures/client\.rs|README\.md)|\?\? packages/agent-claude-code/src/sign_in\.rs)$' | wc -l   # 0
git diff --quiet "$BASE" -- packages/agent-codex $P/contracts.yaml $P/Cargo.toml $P/Cargo.lock $P/src/bin $P/tests/package.rs $P/tests/fixtures/stale-init.json; echo $?   # 0
git diff --numstat "$BASE" -- $P/src/lib.rs   # 1, 0 and the path
grep -c '^pub mod sign_in;$' $P/src/lib.rs   # 1
diff packages/agent-codex/src/lib.rs $P/src/lib.rs | grep -c '^[<>]'   # 3
grep -c 'pub const STATUS_DEADLINE: Duration = Duration::from_secs(30);' $P/src/sign_in.rs   # 1
grep -c 'pub const STATUS_LIMIT: u64 = 65_536;' $P/src/sign_in.rs   # 1
grep -c 'vec!\["auth".into(), "status".into()\]' $P/src/sign_in.rs   # 1
grep -c 'Synthetic-Account' $P/src/sign_in.rs $P/src/main.rs $P/tests/fixtures/client.rs   # 0 each
grep -c '"name":"account"' $P/src/main.rs   # 1
grep -c '"name":"sign-out"' $P/src/main.rs   # 1
grep -c "^    implements: \['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1', 'github.com/shrav-k/conduit/core#account-hook.v1', 'github.com/shrav-k/conduit/core#sign-out-hook.v1'\]$" $P/conduit.yaml   # 1
grep -c "^      'github.com/shrav-k/conduit/core#account-hook.v1': account$" $P/conduit.yaml   # 1
grep -c "^      'github.com/shrav-k/conduit/core#sign-out-hook.v1': sign-out$" $P/conduit.yaml   # 1
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs $P/tests/wrapper.rs   # 18, 23
grep -c '^## The .v2 account and sign-out hooks$' $P/README.md   # 1
grep -c '(#the-v2-account-and-sign-out-hooks)' $P/README.md   # 1
tr '\n' ' ' < $P/README.md | grep -c 'The plain MCP wrapper exposes `turn`, and each `.v2` item also `retire`; the manifest maps each versioned operation or hook to its tool\.'   # 1
grep -ci 'apikey' $P/contracts.yaml $P/conduit.yaml   # 0 each
grep -rnE '/(Users|home)/|/(private/)?tmp/' $P --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

The script below, run from the repository root with a Python that has PyYAML, must print exactly
the line below it: whether `wrapper` is `BASE`'s, whether the rest of the manifest is `BASE`'s,
whether `wrapper-v2` is `BASE`'s with the two hooks added to `implements` and `tools` alone, and
whether the items' names are `BASE`'s.

```bash
python3 - "$BASE" <<'EOF'
import subprocess, sys, yaml
base = sys.argv[1]
core = "github.com/shrav-k/conduit/core#"
path = "packages/agent-claude-code/conduit.yaml"
was = yaml.safe_load(subprocess.run(["git", "show", f"{base}:{path}"], capture_output=True, text=True, check=True).stdout)
now = yaml.safe_load(open(path))
less = lambda d, keys: {k: v for k, v in d.items() if k not in keys}
n, w = now["implementations"]["wrapper-v2"], was["implementations"]["wrapper-v2"]
print(now["implementations"]["wrapper"] == was["implementations"]["wrapper"],
      less(now, ("implementations",)) == less(was, ("implementations",)),
      n["implements"] == w["implements"] + [core + "account-hook.v1", core + "sign-out-hook.v1"]
      and n["tools"] == {**w["tools"], core + "account-hook.v1": "account", core + "sign-out-hook.v1": "sign-out"}
      and less(n, ("implements", "tools")) == less(w, ("implements", "tools")),
      sorted(now["implementations"]) == sorted(was["implementations"]))
EOF
```

```text
True True True True
```

In `packages/agent-claude-code/`, one command at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

`cargo test --release` reports, for `tests/wrapper.rs`, 23 passed and 0 failed, and every other
test target as at `BASE` (the library's unit tests 2 and the package test 1); the package test reads
the new `conduit.yaml`. Report the totals; what the five new tests printed, taken from
`cargo test --release --test wrapper -- --nocapture v2_account v2_sign_out`; the digest that
`package` prints (it is a host build's); and decision 7's reply, its `accepted` and each
diagnostic's `ruleId` and text. Say on which system the tests ran.

Then show that the first new test fails when the hook reports another field of the status:

```bash
cp src/sign_in.rs target/sign_in.rs.fixed
```

Make one edit to `src/sign_in.rs`: replace `reading.get("email")` with `reading.get("orgName")`.
Then run `cargo test --release --test wrapper v2_account_reports_only`; it must fail, at the
assertion that the second `account()` answers `Ok(Some(ACCOUNT))` (it answers the canary claim).
Restore with `cp target/sign_in.rs.fixed src/sign_in.rs` (not `mv`, so that cargo rebuilds), rerun
the same command, which passes, and check that `cmp target/sign_in.rs.fixed src/sign_in.rs` prints
nothing.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, and give the last line of every check and each count above.
Tests that bind loopback ports may fail in your sandbox; if they do, say so, and the admin reruns
them. Where the sandbox denies PID-scoped `ps`, `v2_client_tree_stop_leaves_no_survivor` stops at
its preflight, and any test that stops a tree with recorded groups may fail with the stop's note (on
macOS every sign-out stops the tree first, and the account hook and the logout record their groups,
so new sign-out cases and order 050h's retire tests among them); say so, name each such test, report
the counts passed and failed for that reason, and the reader reruns them on the Mac.

The admin then runs `python3 tools/check_private_paths.py` and `git diff --cached --check` on the
staged change. This order is a checkpoint on the Claude lane's branch, cut from
`wo/050-agents-0.2.0` at order 050h's tip, `f817139e`: it does not merge alone, and its Record is
written at the foot of this file on that branch, as orders 050c's to 050h's are. A fresh Codex
verifier and a fresh read-only Opus reader read the result before the checkpoint stands ([ruling
79](../proposals/ruling-79/README.md), point 2). The local gate that [ruling
80](../proposals/ruling-80/README.md) has a merge wait on, `tools/gate.py`, runs at order 050n's
merge, read with the expected engine failure known.

## Record

Run on 2026-11-19, from about 03:16 to 03:40 UTC, on the Windows computer (WSL2, Linux x86_64),
from `wo/050-lane-claude` at `7bf4cb47` (order 050h's last shoulds), in two rounds: the worker
(657 s) and a fresh Codex verifier (266 s), then the worker again (293 s) and a second fresh Codex
verifier (238 s). The change is the seven files of Scope. The `wrapper-v2` item implements
`account-hook.v1` and `sign-out-hook.v1`, mapped to the tools `account` and `sign-out`;
`src/lib.rs` gains the one line `pub mod sign_in;`; the new `src/sign_in.rs` holds decision 2's
code: the account hook checks the home, runs `claude auth status` in the client directory alone,
reads at most 65,536 bytes under a 30-second deadline and answers the top-level `email` of a JSON
object whose `loggedIn` is true, or null where it is false on exit 1; the sign-out hook stops the
client first, ends a running conversation and runs order 050h's logout unchanged. `src/main.rs`
lists both tools on a `.v2` wrapper and rejects arguments; the scripted client gains a status;
`tests/wrapper.rs` gains five tests (23); the README gains `## The .v2 account and sign-out hooks`.
In both rounds the wrapper passed 23 tests, the library 2 and the package test 1, decision 7's
registration gave `accepted: true` with no diagnostic, and the mutation of "Done when" failed at
the expected assertion and passed after the restore; the host archive's digest was
`9f8f3756...e099` both times. The worker marked decision 6 as the nearest sound reading in round 1
and as written in round 2. Its own choices: in round 1, table-driven error cases, the login's bytes
checked without printing them, and cargo run offline; in round 2, in answer to the first verifier,
a status-descendant mode in the scripted client (a check inside `status-hang` that spawns
`fixture-status-descendant` when the test has bound its socket, and a new argument branch), and
assertions added inside the prescribed tests: a valid status padded to 65,536 and 65,537 bytes, a
deadline with a conversation running whose descendant must exit and whose conversation then takes
another turn without a restart, and stdio calls that fail the status and the logout with a
synthetic login present. The worker reported decisions 4 and 5 as written; these additions are its
own choices under them (R2 below).

The first verifier found no must and six shoulds: the README said that any missing named directory
keeps the status from starting, where a directory refused at startup leaves the status in the
wrapper's working directory; the README promised that a failure before the logout leaves the login
in place, though stopping a running client comes first; the oversized-output case could pass under
a silent truncation; the deadline case did not check that a running conversation survives and the
status's own group is killed; no stdio call captured a hook failure after the client had emitted
canaries; and round 1's report carried a local directory prefix in its package line. Round 2
answered each, as above, and wrote the package line with a placeholder for the directory. The
second verifier found no must and three shoulds: the sign-out test with a running conversation
would pass if sign-out marked the session failed and logged out while the conversation's process
still lived; the account tests would pass if the status inherited logout's directory fallback, with
no case that removes an accepted directory before an account call; and the README does not say that
the deadline's failure takes precedence over an oversized output when the client has not exited.

A fresh read-only Opus reader then read the result on the Mac (macOS, arm64; ruling 79, point 2), on
the branch at `28c04902` on `7bf4cb47`. `check_docs`, `check_private_paths` and `set_names --check`
gave Errors: 0, and `git diff --check` was clean, also against the base; the diff held the seven
files of Scope, the files that must not change were unchanged, every count of "Done when" printed
what the order says and its script printed `True True True True`. Formatted with rustfmt, decision
2's code block was identical to `src/sign_in.rs`. `cargo fmt --check` (on a scratch copy) and
`clippy -D warnings` were clean; `cargo test --release` gave the library's 2, the package test 1
and 23 wrapper tests passed, 0 failed, and the five new tests printed what the worker reported. The
host archive's digest was `183f938a...b6da`, a host build's. The mutation failed at
`tests/wrapper.rs:2455` (the second `account()` gave `Some("synthetic-claim-canary")`) and passed
after the restore, `cmp` silent. With an engine built on the Mac and a fresh store, decision 7's
registration gave `accepted: true` with no diagnostic. A scan of the added lines found no private
detail, and both commits had the work-order identity. It found no must and three shoulds:

- R1: the README's new section departs from decision 6: its sign-out failure sentence used the
  test fixture's wording in place of the order's, and named the wrapper, not the hook; it dropped
  "unlike the retire hook" and, from the did-not-start item, "unlike the sign-out hook's logout";
  its two source links stood on lines of their own, not inline; and it used curly quotes, which the
  README at the base does not.
- R2: round 2's status-descendant mode in the scripted client goes beyond decision 4's "Nothing
  else in the file changes", and its added assertions beyond decision 5's cases, though the worker
  reported both as written; they are sound and test decision 2's kill of the status's group alone.
- R3: round 1's worker report holds a local directory path in its package line, which the Record
  must not carry.

The fixer kept the branch on `wo/050-lane-claude` at `7bf4cb47`, without a rebase, and applied
each finding:

- R1: in the README, the did-not-start item adds ", since the status, unlike the sign-out hook's
  logout, runs in the client directory alone"; the paragraph after the sign-out hook's steps reads
  "The sign-out hook keeps the home and every other entry in it, unlike the retire hook. Its
  failures are the retire hook's logout failures. The hook itself removes nothing before the
  client's logout runs, so a failure before the logout starts leaves the login in place; after a
  failed or timed-out logout, the account hook reports whether a login remains."; the sources open
  "Claude Code's CLI reference, section "CLI commands", read on 2026-11-18, says that" and "A5's
  stage 0 recorded Claude Code 2.1.318", each with its link inline as decision 6 has it, and the
  two standalone link lines are gone; the README holds no curly quote. Decision 6 stands as the
  nearest sound reading, with these changes, not as written.
- R2: no code change; the status-descendant mode and the added assertions stand as the worker's
  own round-2 choices under decisions 4 and 5, made in answer to the first verifier, open to the
  coordinator's strike, which would drop the mode and the deadline case with a running
  conversation.
- R3: this Record gives each package line as the archive's file name and digest alone:
  `agent-claude-code.tar.gz#9f8f3756...e099` from the Linux worker, and `183f938a...b6da` from the
  Mac, a separate host build.

The second verifier's three shoulds, which the reader did not carry (the sign-out test's check
that the conversation's process stopped before logout, an account case whose accepted directory is
removed before the call, and the precedence of the deadline over an oversized output), stay open
for the coordinator. The fixer then ran on the Mac, from the worktree's root, on the files of this
commit before it was made: every count of "Done when" against `wo/050-lane-claude` (each printed
what the order says, the status count read as `git diff --name-status`, which listed the seven
files of Scope and, once the Record was added, this order file), its script under
`/usr/bin/python3` (`True True True True`), `check_docs`,
`check_private_paths` and `set_names --check` (Errors: 0 each), `git diff --check` and
`git diff --check wo/050-lane-claude...HEAD` (clean). In `packages/agent-claude-code/`, one cargo
command at a time with `RUST_TEST_THREADS=4`: `cargo fmt --check` (exit 0),
`cargo clippy --all-targets -- -D warnings` (clean), `cargo test --release` (the library's 2, the
package test 1 and 23 wrapper tests passed, 0 failed),
`cargo test --release --test wrapper -- v2_account v2_sign_out` (5 passed) and
`cargo run --release --bin package` (`183f938a...b6da`, the reader's). The mutation of "Done when"
failed at `tests/wrapper.rs:2455` with `Some("synthetic-claim-canary")` and passed after the
restore, `cmp` silent. The engine's tests were not run, as the order says. The gate's run on this
order's squash is the coordinator's, at order 050n's merge.
