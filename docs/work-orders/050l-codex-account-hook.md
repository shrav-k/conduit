# Work order 050l (P2l): `codex.v2`'s account hook asks Codex itself through `account/read`, and its sign-out hook

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order runs on the Mac or on the Linux node; it
needs neither in particular. It has network access only for tests that bind a loopback port; it
adds no crate, so cargo fetches nothing, and you use the network for nothing else. Run one cargo
command at a time. Never print a whole large file: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and
never read or print a key, a token or any credential. Never read or print any process's environment
or arguments, never run a command that lists every process on the machine, such as `ps -e`,
`ps aux` or `top`, and never signal a process that a test of this order did not start. Never run
the real `codex` or `claude` programs, not even for their help or version, and sign in nowhere.
This order adds a read of Codex's account through its app-server and a sign-out: every `Config` a
test builds and every wrapper a test starts uses the package's scripted client
(`--client-program`), as every test at `BASE` does, so that no read and no logout ever reaches a
real client, and every home a test uses is a fake home under a temporary directory. No test uses a
real login. The engine is run only to initialize a store and register projects.

## What and why

**Where this order starts.** Your tree is the Codex lane's branch, which the coordinator cuts from
`wo/050-agents-0.2.0` at B1, the checkpoint after orders 050f and 050h, and names as `BASE`. It
holds orders 050a, 076 and 078, 050a's Record, orders 050c, 050d and 050e, each with its Record,
and orders 050f and 050h, verified, rebased onto `main`; it holds none of the Claude lane (orders
050i to 050k). `BASE`'s tip holds this order file, committed: the coordinator rebases the branch
onto `main` after this order's text merges, and sets `BASE` to the branch's tip. Orders 050f and
050h were not built when this order was written, so each fact of theirs that this order relies on
is named in the check below. Before you change anything, check from the repository root that you
find there what that gives, and stop, change nothing and report if any check prints otherwise. The
Python lines need a Python with PyYAML, as `tools/check_schemas.py` does (AGENTS.md, "Checks").

```bash
git merge-base --is-ancestor d891543f "$BASE"; echo "main $?"
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/050l-codex-account-hook.md >/dev/null; echo "order $?"
git status --short | wc -l
grep -c '^## Record' docs/work-orders/050a-agents-v2-own-tools.md docs/work-orders/050c-agents-v2-client-home.md docs/work-orders/050d-agents-v2-process-tree.md docs/work-orders/050e-agents-v2-failed-turn-fields.md
python3 -c "import yaml; s=[d for d in yaml.safe_load_all(open('packages/agent-codex/contracts.yaml')) if d['metadata']['name']=='codex.v2'][0]['spec']; print(s['requires'], s['implementation'], 'signIn' in s, s['schema']['properties']['account'])"
python3 -c "import yaml; m=yaml.safe_load(open('packages/agent-codex/conduit.yaml'))['implementations']; print([(n, m[n]['implements'], m[n]['tools'], m[n]['plain']) for n in ('wrapper-v2', 'wrapper-v2-path')])"
grep -c 'pub const LOGOUT_DEADLINE: Duration = Duration::from_secs(60);' packages/agent-codex/src/lib.rs
grep -cE 'pub const (RETIRED|RETIRED_REFUSAL): &str|pub fn logout_arguments\(\)|fn client_command\(|async fn logout\(|fn empty_home\(|pub async fn retire_within\(|retired: bool' packages/agent-codex/src/lib.rs
grep -c 'struct Wrapper(Arc<Mutex<Session>>, bool);' packages/agent-codex/src/main.rs
grep -c 'fn logout(' packages/agent-codex/tests/fixtures/client.rs
grep -cE 'const (LOGIN_TOKEN|LOGIN_CLAIM|DOTENV_REFUSAL)' packages/agent-codex/tests/wrapper.rs
grep -cE 'async fn v2_retire_' packages/agent-codex/tests/wrapper.rs
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs
grep -cE 'account-hook|sign-out-hook|sign-in-hook|signIn|app-server|"account"|"sign-out"' packages/agent-codex/contracts.yaml packages/agent-codex/conduit.yaml packages/agent-codex/src/lib.rs packages/agent-codex/src/main.rs packages/agent-codex/tests/fixtures/client.rs packages/agent-claude-code/contracts.yaml packages/agent-claude-code/src/main.rs
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'
diff packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs | grep -c '^[<>]'
grep -cE '^`codex logout`: pass, a revocation scoped to that home.s token alone' experiments/027-a5-sign-in/results-2026-11-17-logout-source.md
grep -c 'the app-server offers `account/read`' experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md
grep -o 'results-2026-11-17-account-read-source.md' packages/agent-codex/README.md | wc -l
```

They print `main 0` (`d891543f` is `main` after the texts of orders 050f and 050h merged, #366),
`head 0`, `order 0` and `0`; then `1` for each order file (the four Records); then, from what orders
050f and 050h bring,

```text
['github.com/shrav-k/conduit/core#retire-hook.v1'] {'#turn.v1': '#wrapper-v2', 'github.com/shrav-k/conduit/core#retire-hook.v1': '#wrapper-v2'} False {'type': ['string', 'null']}
[('wrapper-v2', ['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1'], {'#turn.v1': 'turn', 'github.com/shrav-k/conduit/core#retire-hook.v1': 'retire'}, True), ('wrapper-v2-path', ['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1'], {'#turn.v1': 'turn', 'github.com/shrav-k/conduit/core#retire-hook.v1': 'retire'}, True)]
```

(`codex.v2` requiring the retire hook alone, with no `signIn`, its `account` a string or null, the
shape it keeps if the live check fails; and the two `.v2` items serving the hook as the plain tool
`retire`); then `1` (050h's logout deadline); `8` (050h's two texts, the logout's arguments, the
command builder, the logout, the emptying, the retire hook and the session's `retired`); `1` (050h's
wrapper with its `.v2` flag); `1` (the scripted client's logout); `3` (050h's two canaries and 050f's
`.env` refusal text); `3` (050h's three retire tests); `18` for each `wrapper.rs`; `0` for each of
the seven files (no account or sign-out hook, no `signIn` and no app-server yet, and no Claude lane);
`2` (the two `lib.rs` still differ in line 18 alone, the `CLAUDE` constant); `0` (the two `main.rs`
are one file); `1` (order 050g's Codex outcome line, a pass); `1` (order 197's method line); and
`3` (the README's links to order 197's results that the 050c to 050e checkpoints wrote: two in the
credential-store and non-Unicode `CODEX_HOME` paragraphs, and one to R4).
These greps and Python lines, and not a Record, are what check orders 050f and 050h: their Records
are written only after their readers, so this order may start from 050h's verified branch before
either Record, and no line above asks for one.

**What it builds.** Two hooks in `packages/agent-codex`, and nothing in `packages/agent-claude-code`:

1. `codex.v2`'s **account hook** learns the ChatGPT account that the Agent's own home is signed in
   to by asking Codex itself: it starts Codex's app-server in that home (`CODEX_HOME`, with the
   file store override), calls `account/read` with `refreshToken` false within a deadline, never
   opens the login file, and fails closed. Where Codex gives no answer or an error, the hook fails;
   where Codex answers that it has no account, or an account with no email, the hook reports no
   account; in each case a text names the step and the cause, and the hook never reports an
   account Codex did not give.
2. `codex.v2`'s **sign-out hook** (the undo of PND-011) signs the client out of the Agent's own
   home with Codex's own logout, as order 050h's retire hook does, and keeps the home.

`codex.v2` lists both under `requires` with `wrapper-v2` as their default, the item that already
serves `turn.v1` and the retire hook, and that item serves them as the plain tools `account` and
`sign-out`. After it the Codex lane holds everything of `codex.v2`'s pass shape but `signIn`, its
logins and its sign-in hook, which are order 050m's.

It carries, from [the coordinator's plan of 050](117-plan-s2-s3.md) (order 117's 050 entry: "050l,
codex.v2's account hook asks Codex itself through account/read, and its sign-out hook. After 050h,
the coordinator's reading that order 197 confirms the route (if not, the owner first, under ruling
82, point 1), and 048a-4 written"):

- [Ruling 82](../proposals/ruling-82/README.md#the-decisions), point 1, the owner's: "`codex.v2`'s
  account hook learns the signed-in ChatGPT account by asking Codex itself, through its
  app-server's `account/read`, in the Agent's own home, and never opens the login file", and, as
  [experiment 027's README](../../experiments/027-a5-sign-in/README.md#stage-1-step-1-the-account-check)
  pre-registers the route: "Where Codex gives no answer, an error or no account, the hook reports
  that, with a diagnostic naming the step and the cause, and never reports an account that Codex
  did not give." The coordinator reads order 197's results as confirming `account/read` at Codex
  0.167.1, so this order is written on that route and nothing goes to the owner first (ruling 82,
  point 1).
- [Order 197](197-a5-stage-1-step-1-codex-account-read.md)'s read of Codex's source at the tag
  `rust-v0.167.1`
  ([its results](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md)), whose
  facts this order applies, each in decision 3 or decision 9: the method `account/read`, with one
  parameter, `refreshToken`, false by default; its answer's `account`, null or tagged by `type`,
  with the `chatgpt` variant's `email` a string or null; the `initialize` request that must come
  first; stdio as the default transport, with `--stdio` selecting it; `CODEX_HOME` naming the home,
  and `-c` overrides accepted by `app-server`; the bootstrap's read of the home's `.env` and the
  helper aliases it creates in the home; the refresh, the network call and the workspace routing
  that answering may still bring; and an email claim that is absent answered as a null email.
- From the targeted review's [section 6](../proposals/review-signin-2026-11-16/README.md#6-must-state-lists),
  which the owner accepted with ruling 82,
  [`codex.v2`](../proposals/review-signin-2026-11-16/README.md#050-codexv2)'s item 1, its
  `requires` part on the account fact's pass ("`requires` lists [...] `account-hook.v1`,
  `sign-out-hook.v1` and `retire-hook.v1`, each with a default in `implementation`"; `signIn` and
  `sign-in-hook.v1` are 050m's); item 4's hook clause and its undo ("Every `.v2` launch and hook
  runs the client with `CODEX_HOME` inside `CONDUIT_STATE_DIR` [...] Done when a marker in a
  synthetic default home stays untouched after an undo, a retirement and a turn": the turn is order
  050c's, the retirement 050h's, the undo this order's); and item 5's account-hook clauses ("with
  the account hook's diagnostics naming only the step and the cause, never file content, a claim or
  the token. Done when the two canary tests pass"). Item 5's URL canary and its port clause belong
  to the sign-in hook, order 050m's.
- Item 8, through [order 050g](050g-logout-revocation-read.md), merged: Codex's outcome line passes,
  "a revocation scoped to that home's token alone", so the sign-out hook runs Codex's own logout, as
  order 050h's retire hook does, and item 8's fail branch is not taken.
- Ruling 82's o1, accepted with its answer 8: "the sign-in hook carries Conduit's keys (the other
  hooks stay plain)". So the two hooks are plain tools of the plain turn item.

**What the specification already says, so that nothing here changes it.**
`account-hook.v1` (`spec/core/operations.yaml`) reports "the account the instance client is signed
in to, or null when none, without reading it through the engine", in an `outputSchema` whose
`account` is a string or null, and states no `effect`; `sign-out-hook.v1` removes "the instance
client login from where its client keeps it" and states an `effect`. Neither takes an input.
[CTR-013](../../spec/promises/contracts.yaml) lets a Type state `signIn` only where `account-hook.v1`
reports an account the service attests, "returned at login, such as in an ID token", and says that
the engine "reads only what account-hook reports, never the instance's state directory".
[PND-011](../../spec/promises/pending.yaml) has no account or another account proceed to the
sign-in hook before it, has "the account that the check before sign-in-hook.v1 found" count as no
account yet while polling, and says "A failed or unknown account hook does not complete sign-in".
The interface's table for a plain server (`spec/interface/implementation.yaml`) reads a complete
result that is not an error as succeeded, an error on an operation with no `effect` as failed, and
an error on one with an `effect`, as the sign-out hook has, as unknown; a succeeded result's
`structuredContent` is validated against the `outputSchema` (OPS-007). Order 048a-4 dispatches both
hooks as parts of a setup, for a Type that states `signIn` alone, and reads `account` from the
validated `structuredContent`. Nothing in this order calls the hooks from the engine: `codex.v2`
states no `signIn` until order 050m.

**What the branch already does, so that you do not redo it.** In `packages/agent-codex/src/lib.rs`
and `src/main.rs`:

- `Config::from_env` reads `CONDUIT_STATE_DIR` first in its `.v2` block (order 050h), so a refusal
  of a later `.v2` input keeps `state_dir` set; `client_home` checks and, where missing, creates
  `client-home` in the state directory with mode 0700, refuses with texts that begin
  `client home (CODEX_HOME): `, refuses a home that holds a `.env` with 050f's words, and refuses a
  canonical home path that is not valid Unicode. `CREDENTIAL_STORE` is
  `cli_auth_credentials_store="file"`.
- `client_command(c, home, args)` (order 050h) builds the client's command as every launch does:
  `process_group(0)` for `.v2`, `env_clear`, the config's `env`, `CODEX_HOME` set to the home,
  `ANTHROPIC_API_KEY` and `OPENAI_API_KEY` removed, the client directory as its working directory,
  and `kill_on_drop`.
- `logout(c, home, tree, deadline)` (order 050h) runs `logout_arguments()`, that is
  `codex logout -c cli_auth_credentials_store="file"`, with its output discarded, records its group
  in the tree, and answers `Ok(())` on status 0 and fails with `client logout did not start`,
  `client logout failed: client exit: ...` or `client logout failed: no exit within N seconds`.
- `ClientTree::stop(false)` stops every group the tree recorded, TERM then KILL, and returns a
  `StopReport` whose `note()` names survivors or an unconfirmed stop (orders 050d and 050e);
  `exit_cause` gives `client exit: status N`, `client exit: signal N` or
  `client exit: status unknown`; `EXIT_WAIT` is 2 seconds; `append_stop_note(Some(text), note)` gives
  `text; note`.
- `Session` holds `retired`, set by `retire_within`, after which every turn fails with
  `RETIRED_REFUSAL`; `Session::shutdown` stops the client's tree. A Codex turn runs `codex exec` and
  ends its process at the turn's end, so no Codex client runs between turns.
- `main.rs`'s `Wrapper(Arc<Mutex<Session>>, bool)` lists `turn` and, where its second field (the
  config's `v2`) is true, `retire`; it answers `retire` before the `turn` check, refusing arguments
  with `ErrorData::invalid_params("expected no arguments", None)`, and answers any other unknown name
  with `ErrorData::invalid_params("unknown tool", None)`.
- The scripted client (`tests/fixtures/client.rs`) reads its arguments first; for the logout's
  arguments it runs `logout(&args)`, which reads `CODEX_HOME`, appends one line to `logouts.jsonl`
  in its working directory, prints `fixture-login`'s content to stdout and stderr, honours
  `logout-exit` and `logout-hang` in its working directory, removes `fixture-login` from the home
  and exits 0, all before it writes `client-started`. `tests/wrapper.rs` has `LOGIN_TOKEN`
  (`synthetic-login-token-canary`), `LOGIN_CLAIM` (`synthetic-claim-canary`), order 050h's helper
  that writes `fixture-login` holding both into a directory, `plant_default_home`, `entry_names`,
  `assert_default_home`, `assert_client_environment`, `DOTENV_REFUSAL` and `AGENT_DOTENV`.

## The decisions it applies

1. **The Type.** In `packages/agent-codex/contracts.yaml`, in the document `codex.v2` alone:
   - the line `  requires: ['github.com/shrav-k/conduit/core#retire-hook.v1']` becomes the four
     lines

     ```yaml
       requires:
       - 'github.com/shrav-k/conduit/core#account-hook.v1'
       - 'github.com/shrav-k/conduit/core#sign-out-hook.v1'
       - 'github.com/shrav-k/conduit/core#retire-hook.v1'
     ```

   - and under `implementation`, after the line
     `    'github.com/shrav-k/conduit/core#retire-hook.v1': '#wrapper-v2'`, add

     ```yaml
         'github.com/shrav-k/conduit/core#account-hook.v1': '#wrapper-v2'
         'github.com/shrav-k/conduit/core#sign-out-hook.v1': '#wrapper-v2'
     ```

   Nothing else in the document changes: no `signIn`, no `sign-in-hook.v1` and no description
   sentence, which are order 050m's (Q1 (v) included). `codex.v1` does not change.

2. **The items.** In `packages/agent-codex/conduit.yaml`, in `wrapper-v2` and in `wrapper-v2-path`,
   line for line the same, which `tests/manifest.rs` checks:
   - the line `    implements: ['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1']`
     becomes the five lines

     ```yaml
         implements:
         - '#turn.v1'
         - 'github.com/shrav-k/conduit/core#retire-hook.v1'
         - 'github.com/shrav-k/conduit/core#account-hook.v1'
         - 'github.com/shrav-k/conduit/core#sign-out-hook.v1'
     ```

   - under `tools`, after `      'github.com/shrav-k/conduit/core#retire-hook.v1': retire`, add

     ```yaml
           'github.com/shrav-k/conduit/core#account-hook.v1': account
           'github.com/shrav-k/conduit/core#sign-out-hook.v1': sign-out
     ```

   `plain: true` stays. The `.v1` item `wrapper` does not change, and implements no hook.

3. **The account hook, in `src/lib.rs`.** Add, after order 050h's `RETIRED_REFUSAL`:

   ```rust
   /// How long the account hook waits, in all, for Codex's answers (ruling 82, point 1).
   pub const ACCOUNT_DEADLINE: Duration = Duration::from_secs(30);
   /// The account hook's text where Codex reported a ChatGPT account; the account is not in it.
   pub const ACCOUNT_REPORTED: &str = "account hook (account/read): Codex reported a ChatGPT account";
   /// The sign-out hook's answer once the client has signed out of the Agent's own home.
   pub const SIGNED_OUT: &str = "client signed out of this Agent's own home";
   ```

   and, after `logout_arguments`:

   ```rust
   /// Codex's app-server on stdio, with the Agent's home and its file store (ruling 82, point 1).
   pub fn account_arguments() -> Vec<String> {
       ["app-server", "--stdio", "-c", CREDENTIAL_STORE]
           .map(str::to_owned)
           .into()
   }

   /// What the account hook reports: the account Codex gave, or none, and a text that names the
   /// step and the cause and never a value.
   #[derive(Clone, Debug, PartialEq, Eq)]
   pub struct AccountReport {
       pub account: Option<String>,
       pub text: String,
   }
   ```

   Then add, after order 050h's `empty_home`, three private items, which never read or write a
   file, never print, and put none of Codex's output in any text.

   - **`enum Reply { Result(Value), Failed(String), Eof }`**, and
     `async fn ask(input: &mut ChildStdin, lines: &mut Lines<BufReader<ChildStdout>>, id: u64, method: &str, params: Value) -> Reply`,
     which writes one line, `json!({"id": id, "method": method, "params": params})` serialized and
     followed by `\n`, and flushes; a write or flush error gives `Reply::Eof`. Then it reads lines:
     the end of the output or a read error gives `Reply::Eof`; a line that does not parse as JSON
     gives `Reply::Failed("invalid answer")`; a message that has a `method` member (a notification
     or a request of the server's) is skipped; a message whose `id` is not `json!(id)` gives
     `Reply::Failed("invalid answer")`; a message with an `error` member gives
     `Reply::Failed("error answer, code N")`, N being `error["code"]` where `as_i64` reads it, and
     otherwise `Reply::Failed("error answer")`; a message with a `result` member gives
     `Reply::Result` of it; any other message gives `Reply::Failed("invalid answer")`. No message
     carries a `jsonrpc` member, and each message is one line of JSON. That framing is the
     coordinator's understanding of Codex's app-server, not order 197's reading, whose results do
     not cover it: it is an assumption that A5's stage 1, step 1 tests against Codex.
   - **`fn account_answer(result: &Value) -> Option<AccountReport>`**, which reads `account/read`'s
     result fail-closed, `None` meaning an answer that is not the protocol's shape:

     ```rust
     fn account_answer(result: &Value) -> Option<AccountReport> {
         let none = |cause: &str| AccountReport {
             account: None,
             text: format!("account hook (account/read): no account: {cause}"),
         };
         let account = result.as_object()?.get("account")?;
         if account.is_null() {
             return Some(none("Codex answered with none"));
         }
         let account = account.as_object()?;
         if account.get("type")?.as_str()? != "chatgpt" {
             return Some(none("the login is not a ChatGPT login"));
         }
         match account.get("email")? {
             Value::String(email) if !email.is_empty() => Some(AccountReport {
                 account: Some(email.clone()),
                 text: ACCOUNT_REPORTED.into(),
             }),
             Value::String(_) | Value::Null => Some(none("the ChatGPT login names no email")),
             _ => None,
         }
     }
     ```

     It reads no other member: not `requiresOpenaiAuth`, not `planType` or `workspaceName`, and not
     the experimental `workspaceRouting`.
   - **`async fn read_account(c: &Config, home: &Path, tree: &ClientTree, deadline: Duration) -> Result<AccountReport>`**,
     which, with `until = Instant::now() + deadline` and the text
     `account hook (start): client did not start` for every failure to start:
     1. takes the tree's state lock, fails with that text where the tree is closed, spawns
        `client_command(c, Some(home), &account_arguments())` with stdin and stdout piped and
        stderr `Stdio::null()` (the app-server's log lines and warnings are never read), fails with
        that text where the spawn, its PID, its stdin or its stdout fails, records its group's leader
        with its start time as `Stream::spawn` does, keeps that PID and start time for step 4, and
        releases the lock;
     2. with `step` a `&'static str` that starts as `"initialize"`, runs within
        `tokio::time::timeout_at(until, ...)`: `ask` with id 1, the method `initialize` and the
        params

        ```rust
        json!({
            "clientInfo": {"name": WRAPPER, "version": env!("CARGO_PKG_VERSION")},
            "capabilities": {"experimentalApi": false},
        })
        ```

        and, where it gives `Reply::Result`, writes the line `{"method":"initialized"}` and flushes
        (an error gives `Reply::Eof`), sets `step` to `"account/read"`, and then `ask` with id 2,
        the method `account/read` and the params `json!({"refreshToken": false})`; any other reply
        of `initialize` ends the exchange with that reply;
     3. reads the outcome into a cause or a report: the deadline gives the cause
        `no answer within N seconds`, N being `deadline.as_secs()`; `Reply::Eof` gives, after
        waiting at most `EXIT_WAIT` for the app-server's exit, `exit_cause(status)`,
        `client exit: status unknown` where the wait fails, and `EOF before answer` where it does not
        exit in that time; `Reply::Failed(cause)` gives that cause; and `Reply::Result(result)` gives
        `account_answer(&result)`, or the cause `invalid answer` where it is `None`;
     4. whatever came, drops stdin, runs `tree.stop_group(pid, start)` on the app-server's own
        group and its descendants alone, then kills and reaps the child; it never runs
        `tree.stop(false)`, which would
        stop every group the tree recorded, a login of order 050m's that is still running in the
        tree included;
     5. answers: a report with no note from the stop is the answer; a report with a note fails
        with `account hook (stop): ` followed by the note; a cause fails with
        `account hook (STEP): CAUSE`, STEP being `step`, followed by `; ` and the stop's note where
        it has one.

   Add to `impl ClientTree`, after `stop`, two methods:

   - **`pub async fn stop_group(&self, group: i32, start: Option<u64>) -> StopReport`**, which
     stops the one recorded group `group`, whose leader started at `start`, and every process
     descended from its processes, TERM then KILL with the existing `STOP_GRACE` and `KILL_WAIT`,
     and leaves every other recorded group alone. Where the tree is closed it waits for good, as
     `stop(false)` does; it takes the stop lock (the tree's second field), so that it never runs
     beside another stop; where `start` is `None` it sends nothing and answers
     `StopReport::UNCONFIRMED`. Each reading of the process table finds, as `TreeState::find` does
     but seeded from this one group alone, not from the tree's other groups or its `seen`: the
     processes that are not zombies and either have `group` as their group or were found at an
     earlier reading of this stop with the same start time, and then, following parent links from
     them, every process descended from those. As `find` does, where the table shows a process whose
     PID is `group` with a start time other than `start`, the group is gone: it seeds nothing from
     the group again and signals it no more. It keeps its own set of what it found, so that a
     process reparented once the app-server dies is still found, and leaves the tree's `seen` as it
     was. It sends TERM to the group with `signal_group(group, start, libc::SIGTERM, false)` and to
     each found process outside the group directly, as `read_and_signal` does (its start time
     checked first, and each process signalled once), then reads the table every `POLL`,
     signalling a newly found process the same way, until it finds none or `STOP_GRACE` has passed;
     then it sends KILL the same way and reads the table every `POLL` until it finds none or
     `KILL_WAIT` has passed. It answers the default report where it finds none, a report whose
     `survivors` are the found processes' PIDs, ascending, where it does not, and
     `StopReport::UNCONFIRMED` where the table cannot be read; and it removes `group` from the
     tree's groups once it finds none. Survivors are only that group's processes and their
     descendants, so a process of another recorded group, or descended from one, never fails a
     read, while a process that the app-server starts in a new group or session (such as a local
     code-mode host, R1) is found through its parent and stopped, as 050d's rule for every `.v2`
     launch has it.
   - **`pub fn record_group(&self, pid: i32) -> bool`**, which records `pid` as a group's leader
     with its start time, as `Stream::spawn` does, under the tree's state lock, and answers false,
     recording nothing, where the tree is closed. Only the first new test calls it, to record a
     group the hook did not start.

   So a read always asks `initialize` first, then `account/read` with `refreshToken` false, and
   never asks for a refresh; the deadline holds for the two steps together; and the app-server
   never outlives the hook, whatever its ending at the end of its input would be: that it keeps
   running after it answers is the coordinator's understanding, not order 197's reading, so the hook
   stops it after every read rather than rely on either.

   No comment in `src/lib.rs` names `jsonrpc`, `refreshToken` or `experimentalApi`: `refreshToken`
   and `experimentalApi` each appear only once, in the code given above, and `jsonrpc` nowhere, as
   the done-when counts check.

   The texts are exactly these, VAR being `CODEX_HOME`:

   | Where | Answer |
   | --- | --- |
   | Codex reported a ChatGPT account with a non-empty `email` | `Ok`, `account` the `email`, text `ACCOUNT_REPORTED` |
   | `account` null | `Ok`, no account, text `account hook (account/read): no account: Codex answered with none` |
   | `account` of a `type` other than `chatgpt` | `Ok`, no account, text `account hook (account/read): no account: the login is not a ChatGPT login` |
   | `chatgpt`, `email` null or empty | `Ok`, no account, text `account hook (account/read): no account: the ChatGPT login names no email` |
   | the app-server does not start | `account hook (start): client did not start` |
   | no answer by the deadline | `account hook (STEP): no answer within N seconds` |
   | the output ends | `account hook (STEP): client exit: status N`, `... signal N`, `... status unknown`, or `account hook (STEP): EOF before answer` |
   | an error answer | `account hook (STEP): error answer, code N`, or `account hook (STEP): error answer` |
   | anything else | `account hook (STEP): invalid answer` |
   | a process outlived the stop, or the stop is unconfirmed | `account hook (stop): NOTE` after a report; `; NOTE` appended to a failure |

   Why some are failures and some report no account: PND-011 reads no account as "proceeds to"
   the sign-in hook, and a failed account hook as one that "does not complete sign-in". Where Codex
   gave no answer or an error, the hook cannot tell that the home holds no login, so it fails, and
   the engine starts no login over a home whose login it could not read; where Codex answered that
   it has no account, or an account that names none, the hook reports none and sign-in proceeds.
   Order 197 found, at the tag, that a home with no login, a provider that needs no OpenAI login and
   a remembered refresh failure each answer `account` null (R2), and that a ChatGPT login whose ID
   token holds no email claim answers `email` null (R6); none of them reports an account.

4. **The two hooks on the session, in `src/lib.rs`.** In `impl Session`, after `retire_within`, add:

   ```rust
   /// The checks the account and sign-out hooks make first, as the retire hook does.
   fn hook_ready(&self, hook: &str) -> Result<()> {
       ensure!(self.config.v2, "{hook}: a .v1 Agent has none");
       if self.config.state_dir.is_none()
           && let Some(error) = &self.config.configuration_error
       {
           bail!("{error}");
       }
       ensure!(!self.retired, "{RETIRED_REFUSAL}");
       Ok(())
   }
   pub async fn account(&mut self) -> Result<AccountReport> {
       self.account_within(ACCOUNT_DEADLINE).await
   }
   /// The account hook, with its deadline given; `account` gives ACCOUNT_DEADLINE.
   pub async fn account_within(&mut self, deadline: Duration) -> Result<AccountReport> {
       self.hook_ready("account hook")?;
       let home = client_home(&self.config)?;
       read_account(&self.config, &home, &self.tree, deadline).await
   }
   pub async fn sign_out(&mut self) -> Result<String> {
       self.sign_out_within(LOGOUT_DEADLINE).await
   }
   /// The sign-out hook, with the logout's deadline given; `sign_out` gives LOGOUT_DEADLINE.
   pub async fn sign_out_within(&mut self, deadline: Duration) -> Result<String> {
       self.hook_ready("sign-out hook")?;
       if let Some(note) = self.shutdown().await.note() {
           bail!("{note}");
       }
       let home = client_home(&self.config)?;
       logout(&self.config, &home, &self.tree, deadline).await?;
       Ok(SIGNED_OUT.into())
   }
   ```

   So each hook needs only the state directory, as the retire hook does, and answers a
   configuration's refusal only where that directory is unset; checks the home with every refusal
   of a launch, the `.env` included, and creates it where it is missing; and is refused once the
   retire hook has begun, since an emptied home would gain the app-server's or the logout's files.
   The account hook does not stop the conversation: no Codex client runs between turns, and the hook
   holds the session, so no turn runs during it. The sign-out hook stops the client's tree first, as
   the retire hook does, signs out with the logout of order 050h, and keeps every entry of the home
   that the logout leaves; a failed logout leaves the login in place for the next attempt. Neither
   hook sets `failed` or `retired`, so the next turn goes on in the same conversation.

5. **The tools, in `src/main.rs`**, which imports nothing new.
   - `list_tools` lists, where the second field is true, after `retire`, in this order:
     `{"name":"account","description":"Report the account the client is signed in to in this Agent's own home, or none, as the client itself reports it.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}}`
     and
     `{"name":"sign-out","description":"Sign the client out of this Agent's own home, keeping the home.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}}`.
     A `.v1` wrapper lists `turn` alone.
   - `call_tool` gains, after the `retire` block and before the `turn` check, a block for
     `self.1 && (r.name == "account" || r.name == "sign-out")`, built as the `retire` block is:
     arguments that are present and not empty give
     `ErrorData::invalid_params("expected no arguments", None)`; the session is taken in the same
     `tokio::select!`, a cancellation before it answering
     `ErrorData::internal_error("account cancelled", None)` or
     `ErrorData::internal_error("sign-out cancelled", None)`; once the hook has the session, no
     cancellation cuts it. For `account`, `Ok(report)` gives the result
     `json!({"content":[{"type":"text","text":report.text}],"structuredContent":{"account":report.account},"isError":false})`
     with `result_type` `Some(ResultType::COMPLETE)`; for `sign-out`, `Ok(text)` gives
     `json!({"content":[{"type":"text","text":text}],"isError":false})` with the same
     `result_type`; an `Err` of either gives `CallToolResult::error(vec![ContentBlock::text(error.to_string())])`.
     `structuredContent` is `{"account": STRING}` or `{"account": null}`, which `account-hook.v1`'s
     `outputSchema` admits; neither result has `_meta`.
   - A `.v1` wrapper answers `account` and `sign-out` as any unknown tool.

6. **The scripted client**, in `packages/agent-codex/tests/fixtures/client.rs` alone. In `main`,
   after order 050h's logout branch and before `client-started` is written, where
   `args.first().map(String::as_str) == Some("app-server")`, call `app_server(&args)` and return.
   Add `fn app_server(args: &[String])`, a stand-in for Codex's app-server on stdio, using
   synchronous std::io (no tokio), serde_json and the libc the fixture already uses, which:
   1. reads `CODEX_HOME`, and panics with `account read without the client home` where it is unset,
      so that a read never reaches a default home;
   2. takes `ENTRIES`, the names in the home, sorted; then, as Codex's bootstrap does (order 197,
      R4), creates `tmp/arg0-fixture` in the home and an empty `lock` in it;
   3. appends one line to `account-reads.jsonl` in its working directory, the JSON object
      `{"args":ARGS,"home":HOME,"environment":ENV,"entries":ENTRIES,"cwd":CWD,"pid":PID}`;
   4. reads `fixture-login` in the home, or an empty text, as `LOGIN`, and writes it to stderr after
      `app-server error ` (output the hook discards);
   5. reads `account-script.json` in its working directory as a JSON object, or `{}` where it is
      absent: `initialize` and `read` each name one behavior, `answer` by default; `answer` is the
      result of `account/read`, by default `{"account":null,"requiresOpenaiAuth":true}`; `notify` is
      false by default; and `linger` is true by default;
   6. reads stdin by lines and appends each line it reads to `account-requests.jsonl` in its working
      directory, as the JSON it parses to, re-serialized on one line (a line that does not parse
      panics); the end of input exits 0 at any point;
   7. answers the first line, whatever it holds, with its `initialize` behavior, the `id` being that
      line's `id` and the result `{"userAgent":"fixture"}`; then reads lines until one whose
      `method` is `account/read`, and answers it with its `read` behavior, the result being
      `answer`; where `notify` is true, it writes, before each answer, the notification
      `{"method":"account/updated","params":{"fixture":LOGIN}}` and the request
      `{"id":"fixture-request","method":"fixture/request","params":{"fixture":LOGIN}}`;
   8. then, where `linger` is true, sleeps 60 seconds, ignoring its input, and exits 0; otherwise
      exits 0.

   The behaviors, each writing one line and flushing: `answer` writes `{"id":ID,"result":RESULT}`;
   `error` writes `{"id":ID,"error":{"code":-32600,"message":"sentinel-secret LOGIN"}}`; `invalid`
   writes `sentinel-not-json LOGIN`; `wrong-id` writes `{"id":99,"result":RESULT}`; `hang` writes
   nothing, sleeps 60 seconds and exits 0; `exit:N` writes nothing and exits with status N; and
   `eof` closes its standard output with `libc::close(1)`, sleeps 60 seconds and exits 0. The line
   after `sentinel-secret` and `sentinel-not-json` is LOGIN itself, so the canaries reach the hook
   on every path.

7. **The tests**, in `packages/agent-codex/tests/wrapper.rs` alone. Import `ACCOUNT_DEADLINE` only if
   a test uses it, and `AccountReport`, `ACCOUNT_REPORTED`, `SIGNED_OUT` and `account_arguments`.
   Every result and error text a test below reads holds neither `LOGIN_TOKEN` nor `LOGIN_CLAIM`, nor
   `sentinel`, and the test asserts it. EXPECTED below is the string `synthetic-expected-account`,
   and CHATGPT(E) is
   `{"account":{"type":"chatgpt","email":E,"planType":"plus","workspaceName":null},"requiresOpenaiAuth":true,"workspaceRouting":null}`.
   Each test plants the default home with `plant_default_home`, sets `c.v2` true, runs `prepare`,
   and writes `account-script.json` in the client directory (`c.directory`, or `c.cwd` where it is
   null) before each read.
   - **Two existing tests change in one assertion each:** in
     `stdio_v2_defaults_and_invalid_configuration_remain_callable`, the names of the listed tools
     are `["turn"]` for the case `v1` and `["turn", "retire", "account", "sign-out"]` for every other
     case; in order 050h's `v2_retire_tool_through_the_wrapper`, the `.v2` wrapper's listed names
     are `["turn", "retire", "account", "sign-out"]`. Every other test keeps passing unchanged.
   - **A new test, `v2_account_hook_reads_codex_in_its_own_home`**, through `Session`, for a
     `directory` null and then a named one, set up as
     `v2_client_home_in_state_directory_leaves_default_home_untouched` sets up each, with the client
     home made with mode 0700 holding `fixture-login`, and the script
     `{"notify":true,"answer":CHATGPT(EXPECTED)}`. `account()` answers
     `AccountReport { account: Some(EXPECTED), text: ACCOUNT_REPORTED }`. `account-requests.jsonl`
     holds exactly three lines, equal as JSON to
     `{"id":1,"method":"initialize","params":{"clientInfo":{"name":"agent-codex","version":VERSION},"capabilities":{"experimentalApi":false}}}`,
     `{"method":"initialized"}` and
     `{"id":2,"method":"account/read","params":{"refreshToken":false}}`, VERSION being the test's
     `env!("CARGO_PKG_VERSION")`. `account-reads.jsonl` holds one line, whose `args` equal
     `account_arguments()`, whose `environment` equals, under `assert_client_environment`, the
     launch's (`c.env` less both API-key variables, with `CODEX_HOME` the home's canonical path),
     whose `cwd` is the client directory, whose `entries` are `["fixture-login"]`, and whose `pid`
     names no live process (`libc::kill(pid, 0)` is -1). The client home, still of mode 0700, then
     holds exactly `fixture-login` (with the content the test wrote) and `tmp`; the default home
     passes `assert_default_home`; `client-started` is absent from the client directory. A second
     `account()` answers the same, with a second line in `account-reads.jsonl` whose `entries` are
     `["fixture-login", "tmp"]`. Then another live group is recorded in the session's tree: the
     test starts the scripted client itself as a logout that hangs, with `logout_arguments()`,
     `process_group(0)`, `CODEX_HOME` a second fake home under the temporary directory holding
     `fixture-login`, and as its working directory a directory `other` beside the client directory
     that holds `logout-hang`. The command is built with `env_clear()`, then `envs(&c.env)` less
     `ANTHROPIC_API_KEY` and `OPENAI_API_KEY`, then `CODEX_HOME` set to the second home's canonical
     path, so that the test process's own environment never reaches the client or its file; its
     stdin, stdout and stderr are `Stdio::null()`. The test records that client's PID with
     `session.tree().record_group(pid)`, which answers true, and waits until `other/logouts.jsonl`
     holds its line, whose `environment` equals, under `assert_client_environment`, that
     environment, so that a run that inherits the test's environment fails. A third `account()`
     answers the same, with a third line in `account-reads.jsonl`; that client still runs after it
     (`libc::kill(pid, 0)` is 0), and the second home still holds `fixture-login`. The test then
     kills that client's group with `libc::kill(-pid, libc::SIGKILL)` and reaps it. Then the turn
     `TEST` succeeds. It `eprintln!`s each report.
   - **A new test, `v2_account_hook_reports_no_account_without_a_chatgpt_email`**, through
     `Session`, `directory` null, one session, the client home not made by the test. For each
     `answer` below, in order, with `notify` true, `account()` answers `Ok` with no account and
     exactly the text beside it, `account hook (account/read): no account: ` followed by the cause:

     | `answer` | Cause |
     | --- | --- |
     | `{"account":null,"requiresOpenaiAuth":true}` | `Codex answered with none` |
     | `{"account":null,"requiresOpenaiAuth":false}` | `Codex answered with none` |
     | `{"account":{"type":"apiKey"},"requiresOpenaiAuth":true}` | `the login is not a ChatGPT login` |
     | `{"account":{"type":"amazonBedrock","usesCodexManagedCredentials":false},"requiresOpenaiAuth":true}` | `the login is not a ChatGPT login` |
     | CHATGPT(`null`) | `the ChatGPT login names no email` |
     | CHATGPT(`""`) | `the ChatGPT login names no email` |

     `account-reads.jsonl` then holds six lines, the first with `entries` `[]` (the hook created
     the home, with mode 0700); the default home passes `assert_default_home`. It `eprintln!`s each
     text.
   - **A new test, `v2_account_hook_error_paths_hold_no_canary`**, through `Session`, one case at a
     time, each in a fresh temporary directory, `directory` null, the client home made with mode
     0700 holding `fixture-login` unless the case says otherwise, and the script `{"notify":true}`
     with the case's members added. Each call is `account()`, but in the two `hang` cases, which
     call `account_within(Duration::from_secs(3))`.

     | Case | Set up | Answer | Reads |
     | --- | --- | --- | --- |
     | `no-client` | `c.program` a missing file under the temporary directory | `account hook (start): client did not start` | 0 |
     | `initialize-exit` | `"initialize":"exit:3"` | `account hook (initialize): client exit: status 3` | 1 |
     | `initialize-error` | `"initialize":"error"` | `account hook (initialize): error answer, code -32600` | 1 |
     | `initialize-hang` | `"initialize":"hang"` | `account hook (initialize): no answer within 3 seconds` | 1 |
     | `read-eof` | `"read":"eof"` | `account hook (account/read): EOF before answer` | 1 |
     | `read-error` | `"read":"error"` | `account hook (account/read): error answer, code -32600` | 1 |
     | `read-invalid` | `"read":"invalid"` | `account hook (account/read): invalid answer` | 1 |
     | `read-wrong-id` | `"read":"wrong-id"` | `account hook (account/read): invalid answer` | 1 |
     | `read-shape` | `"answer":{"account":{"type":"chatgpt","planType":"plus"},"requiresOpenaiAuth":true}` | `account hook (account/read): invalid answer` | 1 |
     | `read-hang` | `"read":"hang"` | `account hook (account/read): no answer within 3 seconds` | 1 |
     | `dotenv` | the home also holds `.env` with `AGENT_DOTENV` | `client home (VAR): ` then `DOTENV_REFUSAL` | 0 |
     | `outside-link` | the home a link to `elsewhere`, a directory beside `state` holding `fixture-login` | `client home (VAR): the home resolves outside CONDUIT_STATE_DIR` | 0 |
     | `unset` | `c.state_dir` `None` | `client home (VAR): CONDUIT_STATE_DIR is unset` | 0 |
     | `configuration-unset` | `c.state_dir` `None`, `c.configuration_error` `invalid configuration: client home (VAR): CONDUIT_STATE_DIR is unset` | that configuration text | 0 |
     | `configuration` | `c.configuration_error` `invalid configuration: effort (AGENT_EFFORT): expected lowercase ASCII letters` | `Ok`, no account, `account hook (account/read): no account: Codex answered with none` | 1 |
     | `retired` | `retire()` first, which answers `RETIRED` | `RETIRED_REFUSAL` | 0 |
     | `v1` | `c.v2` false | `account hook: a .v1 Agent has none` | 0 |

     VAR is `CODEX_HOME`, and each answer is compared exactly: the `Ok` report as a whole, the
     others as the error's text. Reads is the number of lines in `account-reads.jsonl`. Then, in
     each case: the text holds no canary and no `sentinel`; `fixture-login` keeps the content the
     test wrote, in the home or in `elsewhere`, but in `retired`, whose home 050h's retire hook
     emptied; each line's `pid` names no live process; the default home passes
     `assert_default_home`; and `client-started` is absent from `c.cwd`. It `eprintln!`s each
     case's answer.
   - **A new test, `v2_sign_out_hook_signs_out_and_keeps_the_home`**, through `Session`, `directory`
     null, each case in a fresh temporary directory, the client home made with mode 0700 holding
     `fixture-login` and `nested/file`. In the first case `sign_out()` answers `SIGNED_OUT`;
     `logouts.jsonl` in `c.cwd` holds one line, whose `args` equal order 050h's
     `logout_arguments()`, whose `environment` equals the launch's under
     `assert_client_environment`, whose `cwd` is `c.cwd` and whose `entries` are
     `["fixture-login", "nested"]`; the home then holds `nested` alone, with `nested/file` kept and
     mode 0700; the default home passes `assert_default_home`; `client-started` is absent; and the
     turn `TEST` then succeeds. Then the failures, each text compared exactly:
     - `exit`: `logout-exit` holding `3` in `c.cwd`: `client logout failed: client exit: status 3`,
       with `fixture-login` kept; with `logout-exit` removed, a second `sign_out()` answers
       `SIGNED_OUT` and the home holds `nested` alone;
     - `deadline`: `logout-hang` in `c.cwd`, and the call `sign_out_within(Duration::from_secs(3))`:
       `client logout failed: no exit within 3 seconds`, with `fixture-login` kept;
     - `dotenv`: the home also holds `.env` with `AGENT_DOTENV`: `client home (CODEX_HOME): `
       then `DOTENV_REFUSAL`, with no logout;
     - `retired`: `retire()` first, which answers `RETIRED`: `RETIRED_REFUSAL`, and `logouts.jsonl`
       holds the retirement's line alone;
     - `v1`: `c.v2` false: `sign-out hook: a .v1 Agent has none`, with no logout.

     In each, the text holds no canary and no `sentinel`, the default home passes
     `assert_default_home`, and `client-started` is absent but after the first case's turn. It
     `eprintln!`s each answer.
   - **A new test, `v2_account_and_sign_out_tools_through_the_wrapper`**, through the wrapper binary
     over stdio, started as order 050h's `v2_retire_tool_through_the_wrapper` starts it, with the
     default home planted and the client home made with mode 0700 holding `fixture-login`. The
     listed tools' names are `["turn", "retire", "account", "sign-out"]`. An `account` call and a
     `sign-out` call, each with the arguments `{"extra":true}`, are each an error of the call. With
     the script `{"notify":true,"answer":CHATGPT(EXPECTED)}`, the `account` call with `{}` answers,
     as JSON, `isError` false, `resultType` `complete`, `content` exactly one text block
     `ACCOUNT_REPORTED`, `structuredContent` exactly `{"account":"synthetic-expected-account"}`,
     and no `_meta` member. With the script `{"notify":true}`, it answers `structuredContent`
     exactly `{"account":null}` and the one text block
     `account hook (account/read): no account: Codex answered with none`. With the script
     `{"notify":true,"read":"error"}`, it answers `isError` true, `resultType` `complete`, one text
     block `account hook (account/read): error answer, code -32600` and no `structuredContent`. The
     `sign-out` call with `{}` answers `isError` false, `resultType` `complete`, one text block
     `SIGNED_OUT`, and no `structuredContent` and no `_meta`. `account-reads.jsonl` holds three
     lines and `logouts.jsonl` one, each with `environment["CODEX_HOME"]` the client home's
     canonical path; the default home passes `assert_default_home`. After `cancel`, the wrapper
     exits successfully, and its stderr holds neither canary nor `sentinel`, nor
     `synthetic-test-token`, `synthetic-anthropic-key` or `synthetic-openai-key`. Then a `.v1`
     wrapper, started in a fresh temporary directory as
     `stdio_exposes_only_turn_and_setup_never_launches_client` starts it, lists `["turn"]`, answers
     an `account` call and a `sign-out` call with `{}` each as an error of the call, and leaves no
     `account-reads.jsonl` and no `logouts.jsonl`. It `eprintln!`s the three `.v2` `account`
     results.

   The test count goes from 18 to 23 in `agent-codex`, and stays 18 in `agent-claude-code`. Item
   5's error-path canary test is the third new test: Codex's output, its notifications, its
   requests and its stderr holding the canaries on every error path. The first new test adds a
   canary on the success path, the same output holding them on a read that succeeds. Item 5's URL
   canary test remains order 050m's.

8. **The README**, `packages/agent-codex/README.md` alone. Write no absolute local path, no login
   literal and no client version beyond those the README already records and 0.167.1, the
   version order 197 read. Leave the opening paragraph's sentence on the tools as order 050h wrote
   it, since order 203 starts from it in both READMEs.
   - In `## The wrapper-v2-path item`: "The defaults of `codex.v2` for `turn.v1` and
     `retire-hook.v1` are still `wrapper-v2`." becomes "The defaults of `codex.v2` for `turn.v1`,
     `retire-hook.v1`, `account-hook.v1` and `sign-out-hook.v1` are still `wrapper-v2`."; the
     template sentence's `implementation: {...}` gains, after the retire hook's pair,
     `github.com/shrav-k/conduit/core#account-hook.v1: example.org/conduit/agent-codex#wrapper-v2-path, github.com/shrav-k/conduit/core#sign-out-hook.v1: example.org/conduit/agent-codex#wrapper-v2-path`;
     and "A template that selects it for `turn.v1` alone takes `wrapper-v2` for the retire hook by
     default, two items that are run, and is rejected (TPL-019)." becomes "A template that selects
     it for some of them alone takes `wrapper-v2` for the others by default, two items that are run,
     and is rejected (TPL-019)."
   - Add, at the end, `## The .v2 account and sign-out hooks`, which says, in this order:
     - `codex.v2` lists `account-hook.v1` and `sign-out-hook.v1` under `requires`, beside the
       retire hook, each with the default `wrapper-v2`, which serves them as the tools `account` and
       `sign-out`, taking no arguments; the `.v1` item lists neither; the engine calls them only as
       parts of `sign-in.v1` or of a setup, for a Type that states `signIn` (CTR-013, PND-011), and
       `codex.v2` states no `signIn` yet, so the engine calls neither yet;
     - the account hook's steps, as a numbered list: it makes the retire hook's checks, refusing a
       `.v1` Agent, answering the configuration's refusal where the state directory is unset, and
       refusing once the retire hook has begun; it checks the home as every launch does, with the
       same refusals, creating it where it is missing; it starts `codex app-server --stdio -c
       cli_auth_credentials_store="file"` with `CODEX_HOME` set to the home and the launch's
       environment, discarding its stderr; it sends `initialize`, with `clientInfo` naming
       `agent-codex` and this package's version and `experimentalApi` false, then the
       `initialized` notification, then `account/read` with `refreshToken` false, one JSON message
       a line and no `jsonrpc` member, skipping the notifications and requests that the app-server
       sends; it waits at most 30 seconds for the two answers together; and, whatever came, it
       stops the app-server's own process group and every process descended from it, following
       parent links as every `.v2` stop does, TERM then KILL with a failed turn's stop's grace and
       wait, leaving any other process the Agent's tree recorded, such as a login still running,
       alone;
     - what it reports: the `email` of a `chatgpt` account, as `account`, with the text
       `account hook (account/read): Codex reported a ChatGPT account`; and no account, with a text
       naming the step and the cause, where Codex answered with none, with a login that is not a
       ChatGPT login, or with a ChatGPT login whose email is null or empty;
     - its failures, each the fixed text of decision 3's table, and why Codex's silence or error is
       a failure and not "no account": a failed account hook does not complete sign-in and starts no
       login (PND-011); its texts hold the step and fixed words and numbers, never Codex's output,
       an error's message, a claim, a token or a file's content; the wrapper never opens the home's
       login file, which Codex alone reads; and it never reports an account that Codex did not give;
     - the sign-out hook: it makes the same checks, stops the client's process tree as the retire
       hook does, runs `codex logout -c cli_auth_credentials_store="file"` with the home as the
       retire hook does, with the same failures and deadline, and answers
       `client signed out of this Agent's own home`; it keeps the home and every entry that the
       logout leaves; a failed sign-out leaves the login for the next attempt, and the engine then
       holds the instance under PND-011 until an account check finds the expected account or none;
     - what answering does at Codex 0.167.1, as
       [order 197](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md)
       read the source at that tag, with no client run, so a reading of the source and not an
       observation: before the app-server starts, Codex's bootstrap reads a `.env` in the home,
       which the hook refuses, and creates helper aliases under `tmp/` in the home, which a release
       build refuses for a home under the system's temporary directory, printing a warning on
       stderr that the hook discards
       ([R4](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r4-what-answering-reads-writes-refreshes-or-calls));
       its start loads the login from the home's file store into memory and also creates and
       writes state files in the home; the email is the one Codex parsed from the ID token's email
       claim, or null where the claim is absent
       ([R3](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r3-where-the-email-comes-from),
       [R6](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r6-what-it-returns-when-the-claim-is-absent));
       `refreshToken` false asks for no refresh, but answering may still refresh a ChatGPT login
       and rewrite the home's login file. That can happen when the configuration acquires auth (a
       token near expiry, or a cloud configuration bundle), and on an unauthorized routing call.
       Starting the app-server and answering may also call OpenAI's service: for a configuration
       bundle, for workspace routing (also discovered in the background after initialization) and
       for analytics at initialization. A remembered refresh failure answers no account (R2, R4);
       the CLI marks the `app-server`
       command experimental, while `account/read` and its parameter are not marked
       ([R5](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md#r5-whether-it-is-marked-experimental));
     - and, as limits: the hook was built against a scripted stand-in, and only A5's stage 1, step 1
       observes it against Codex
       ([pre-registered](../../experiments/027-a5-sign-in/README.md#stage-1-step-1-the-account-check));
       the account is the one OpenAI's ID token named at sign-in, as Codex parsed it, and the hook
       does not check the token itself; a ChatGPT login whose ID token holds no email claim never
       completes sign-in, the action staying pending; a configuration that the machine manages can
       set Codex's credential store, which the hook's `-c` does not show it overriding; unlike a
       turn, which passes `--ignore-user-config`, the app-server reads the home's `config.toml`,
       which the Agent's own tools can write, and such a file can make Codex answer no account (a
       provider that needs no OpenAI login) or change what the read calls, since the `-c` override
       pins only the credential store, and A5's stage 1, step 1 runs with a fresh home (this is the
       coordinator's understanding, not order 197's reading); each read
       starts a new app-server, so each poll of a sign-in starts one; the message framing, one JSON
       message a line with no `jsonrpc` member, is an assumption that was not read at the tag, and
       A5's stage 1, step 1 tests it; and a version of Codex other than 0.167.1 has not been read.

9. **Check the Type with the engine**, as order 050h's decision 9 does, for `agent-codex` alone.
   This checks what the package tests cannot: that the hooks' defaults resolve, and that the turn
   item and the three hooks are one run.
   - In `engine/`, run `cargo build --release`. Change nothing there.
   - In `packages/agent-codex/`, run `cargo run --release --bin package`, which prints
     `ARCHIVE#SHA256`.
   - In a new temporary directory T outside the repository, run
     `engine/target/release/conduit init --store T/store`, then write projects under T, each a
     `conduit.yaml` that pins `example.org/conduit/agent-codex` at version 0.2.0 with that digest,
     as `engine/crates/shell/tests/agents.rs` writes one, and one template file, `kind: Agent`,
     `spec.type` `example.org/conduit/agent-codex#codex.v2`, `config: {account: example-account}`:
     - R: the template `v2-hooks`, with no `implementation`.
     - N, the negative control: the template `v2-account-v1-item`, with
       `implementation: {github.com/shrav-k/conduit/core#account-hook.v1: example.org/conduit/agent-codex#wrapper}`.
     - P1: the template `v2-path-turn-and-retire`, with
       `implementation: {example.org/conduit/agent-codex#turn.v1: example.org/conduit/agent-codex#wrapper-v2-path, github.com/shrav-k/conduit/core#retire-hook.v1: example.org/conduit/agent-codex#wrapper-v2-path}`.
     - P2: the template `v2-path-all`, with that map and the account and sign-out hooks' pairs, each
       to `example.org/conduit/agent-codex#wrapper-v2-path`, in it.
   - Register each in turn with
     `engine/target/release/conduit register --store T/store --package ARCHIVE#SHA256 PROJECT`. No
     project holds an instance, so none starts a wrapper or a client. Report each reply's
     `accepted` and its diagnostics.
   - Expected: R and P2 are accepted with no diagnostic; N is not accepted, with an error citing
     TPL-005 (`wrapper` does not implement the hook); and P1 is not accepted, with an error citing
     TPL-019 (two items that are run). If a reply differs, change nothing to make it pass: report
     it, since this order changes no engine source, and the coordinator decides.
   - Registration into the throwaway store under T, with no instance, which is removed after the
     check, is not an installation in the sense of the rule that no archive built from the Codex
     lane is installed as `codex.v2` anywhere, as with order 050h's decision 9 and order 050a's step
     11. Once you have recorded the replies, remove T.

10. **What this order does not do.** No `signIn`, no login, no sign-in hook and no description
    sentence, which are order 050m's; no change to `packages/agent-claude-code/`, whose hooks are
    orders 050i to 050k's, so the two packages' `lib.rs`, `main.rs` and scripted clients differ on
    this lane until order 050n joins the lanes; no input added or changed, and no item binding
    `account` (the engine compares the expected account, PND-011, and the hook never learns it); no
    change to `.v1`, to `turn.v1`, to `conversation.v1`, to the `wrapper` item, to the retire hook,
    or to `Cargo.toml`, `Cargo.lock`, `src/bin/package.rs`, `tests/package.rs` or
    `tests/manifest.rs`; no wording of an existing refusal changed; and nothing under `engine/`,
    `conformance/`, `spec/`, `docs/` or `experiments/`. The loader vector of item 1's done-when
    and the move of `conformance/fixtures/agents/` to 0.2.0 are order 050n's. The engine's test
    `live_s1_scripted_agents_call_engine_with_distinct_run_identities`
    (`engine/crates/shell/tests/agents.rs`) fails on this branch by design until order 050n moves
    `conformance/fixtures/agents/` to 0.2.0, as [order 050a](050a-agents-v2-own-tools.md) records.
    It is the one expected failure on `wo/050-agents-0.2.0` and on this lane, and it is not this
    order's: neither you nor the verifier counts it against this order. Do not run the engine's
    tests and do not change it.

## Who decided what

The route is the owner's: ruling 82, point 1, "asking Codex itself, through its app-server's
`account/read`, in the Agent's own home", never opening the login file, as experiment 027's README
pre-registers it with "no answer, an error or no account" each reported with the step and the
cause. Reading order 197's results as confirming `account/read` at Codex 0.167.1 is the
coordinator's, and the owner may strike it before the live check. The review's item 1 (`requires`
and the defaults), item 4's undo, item 5's account-hook clauses and its error-path canary test
are the review's must-state lists, which the owner accepted with ruling 82; the hooks' plain items are
ruling 82's o1; and the sign-out by Codex's own logout rests on order 050g's outcome line, as order
050h's retire hook does. The split of order 050 into 050b to 050o, this order's place as the first
of the Codex lane after B1, its branch and its base are the coordinator's, open to the owner's
strike ([ruling 48](../proposals/ruling-48/README.md)). Everything else here is the coordinator's
too, each open to the owner's strike: how order 197's facts apply (the `.env` refusal kept as the
answer to the bootstrap's read, the helper aliases and state files left in the home, the refresh,
the network call and the workspace routing stated as limits, and a null email reported as no
account); which outcomes are failures and which report no account, a non-ChatGPT login and an empty
email included; `--stdio`, sent though stdio is the default; `refreshToken` and `experimentalApi`
sent as false, though false is each one's default; the `initialized` notification, which Codex only
logs; `clientInfo`'s name and version; the 30-second deadline for the two steps together, under the
engine's default call timeout of 120 seconds; the stop of the app-server after every read, of its
own group and the processes descended from it alone, following parent links as 050d's rule has
every `.v2` stop do, through `ClientTree::stop_group` (with `record_group` for the test), and a
stop's note failing a read; the understanding, not order 197's reading, that unlike a turn the
app-server reads the home's `config.toml`, which the Agent's own tools can write and which can make
Codex answer no account or change what the read calls, the `-c` override pinning only the
credential store, stated as a README limit; that the app-server keeps running after it answers, which is the
coordinator's understanding and not order 197's reading; the message framing, one JSON message a
line with no `jsonrpc` member, an assumption of the coordinator's that order 197 did not read and
A5's stage 1, step 1 tests; requests and notifications from the server skipped, and an answer for
another ID read as invalid; both hooks refused once the retire hook has begun; the sign-out hook
keeping the home; `codex.v2` requiring the two hooks on this lane before order 050m gives it
`signIn`; the tool names and descriptions, the texts, the README's words, the scripted app-server
and its script, and the registration probe with its negative control. Apply them as written and
report them under `decisions` as the coordinator's.

## Scope

Change only, in `packages/agent-codex/`: `contracts.yaml`, `conduit.yaml`, `src/lib.rs`,
`src/main.rs`, `tests/wrapper.rs`, `tests/fixtures/client.rs` and `README.md`. Nothing else, in the
package or outside it, and nothing in `packages/agent-claude-code/`. The build's ignored output
(`target/`, `dist/`) may change, and so may anything under T.

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
git status --short | grep -vE '^ M packages/agent-codex/(contracts\.yaml|conduit\.yaml|src/lib\.rs|src/main\.rs|tests/wrapper\.rs|tests/fixtures/client\.rs|README\.md)$' | wc -l   # 0
git diff --quiet "$BASE" -- packages/agent-claude-code packages/agent-codex/Cargo.toml packages/agent-codex/Cargo.lock packages/agent-codex/src/bin packages/agent-codex/tests/package.rs packages/agent-codex/tests/manifest.rs engine spec conformance docs experiments; echo $?   # 0
head -c "$(wc -c < conformance/fixtures/agents/agent-codex/contracts.yaml)" packages/agent-codex/contracts.yaml | cmp - conformance/fixtures/agents/agent-codex/contracts.yaml; echo $?   # 0
grep -cE "^  - 'github.com/shrav-k/conduit/core#(account|sign-out|retire)-hook.v1'$" packages/agent-codex/contracts.yaml   # 3
grep -c "^      'github.com/shrav-k/conduit/core#account-hook.v1': account$" packages/agent-codex/conduit.yaml   # 2
grep -c "^      'github.com/shrav-k/conduit/core#sign-out-hook.v1': sign-out$" packages/agent-codex/conduit.yaml   # 2
grep -c 'pub const ACCOUNT_DEADLINE: Duration = Duration::from_secs(30);' packages/agent-codex/src/lib.rs   # 1
grep -c 'refreshToken' packages/agent-codex/src/lib.rs   # 1
grep -c 'jsonrpc' packages/agent-codex/src/lib.rs   # 0
grep -ciE 'auth\.json|fixture-login' packages/agent-codex/src/lib.rs packages/agent-codex/src/main.rs   # 0 each
grep -c 'experimentalApi' packages/agent-codex/src/lib.rs   # 1
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs   # 23 for Codex, 18 for Claude Code
grep -c '^## The .v2 account and sign-out hooks$' packages/agent-codex/README.md   # 1
grep -o 'results-2026-11-17-account-read-source.md' packages/agent-codex/README.md | wc -l   # 8
tr '\n' ' ' < packages/agent-codex/README.md | grep -c 'The defaults of `codex.v2` for `turn.v1`, `retire-hook.v1`, `account-hook.v1` and `sign-out-hook.v1` are still `wrapper-v2`\.'   # 1
tr '\n' ' ' < packages/agent-codex/README.md | grep -c 'The plain MCP wrapper exposes `turn`, and each `.v2` item also `retire`; the manifest maps each versioned operation or hook to its tool\.'   # 1
grep -ci 'apikey' packages/agent-codex/contracts.yaml   # 0
git diff "$BASE" -- packages/agent-codex | grep '^+' | grep -c "$(printf '\100')"   # 0
grep -rnE '/(Users|home)/|/(private/)?tmp/' packages/agent-codex --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

The third line is 050a's byte check: the 0.1.1 contracts stay a byte prefix of `contracts.yaml`.
The line that counts `results-2026-11-17-account-read-source.md` counts the README's links to
order 197's results: the 3 at `BASE` (the start check counts them) and the five new ones in the new
section (the read, R3, R4, R5 and R6).

The script below, run from the repository root with a Python that has PyYAML, must print exactly
the two lines below it: for `codex.v2`, whether its `requires` are the two hooks and then the retire
hook, whether `BASE`'s were the retire hook alone, whether its `implementation` is `BASE`'s with the
two defaults added, whether the rest of its `spec` is `BASE`'s, whether the rest of the document is
`BASE`'s, and whether every other document of the file is `BASE`'s; then, for the manifest, whether
`wrapper` is `BASE`'s, whether the rest of the manifest is `BASE`'s, whether each `.v2` item is
`BASE`'s with the two hooks added to `implements` and `tools` alone, and whether the items' names
are `BASE`'s.

```bash
python3 - "$BASE" <<'EOF'
import subprocess, sys, yaml
base = sys.argv[1]
core = "github.com/shrav-k/conduit/core#"
hooks = [core + "account-hook.v1", core + "sign-out-hook.v1"]
retire = core + "retire-hook.v1"
show = lambda path: subprocess.run(["git", "show", f"{base}:{path}"], capture_output=True, text=True, check=True).stdout
less = lambda d, keys: {k: v for k, v in d.items() if k not in keys}
path = "packages/agent-codex/contracts.yaml"
docs_now, docs_was = list(yaml.safe_load_all(open(path))), list(yaml.safe_load_all(show(path)))
index = [d["metadata"]["name"] for d in docs_was].index("codex.v2")
now, was = docs_now[index], docs_was[index]
print(now["metadata"]["name"], now["spec"]["requires"] == hooks + [retire],
      was["spec"]["requires"] == [retire],
      now["spec"]["implementation"] == {**was["spec"]["implementation"], **{h: "#wrapper-v2" for h in hooks}},
      less(now["spec"], ("requires", "implementation")) == less(was["spec"], ("requires", "implementation")),
      less(now, ("spec",)) == less(was, ("spec",)),
      docs_now[:index] + docs_now[index + 1:] == docs_was[:index] + docs_was[index + 1:])
path = "packages/agent-codex/conduit.yaml"
now, was = yaml.safe_load(open(path)), yaml.safe_load(show(path))
out = ["agent-codex", now["implementations"]["wrapper"] == was["implementations"]["wrapper"],
       less(now, ("implementations",)) == less(was, ("implementations",))]
for item in ["wrapper-v2", "wrapper-v2-path"]:
    n, w = now["implementations"][item], was["implementations"][item]
    out.append(n["implements"] == w["implements"] + hooks
               and n["tools"] == {**w["tools"], hooks[0]: "account", hooks[1]: "sign-out"}
               and less(n, ("implements", "tools")) == less(w, ("implements", "tools")))
out.append(sorted(now["implementations"]) == sorted(was["implementations"]))
print(*out)
EOF
```

```text
codex.v2 True True True True True True
agent-codex True True True True True
```

In `packages/agent-codex/`, one command at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

`cargo test --release` reports, for `tests/wrapper.rs`, 23 passed and 0 failed, and every other
test target as at `BASE` (the library's unit tests 2, the package test 1 and the manifest test 1);
the package and manifest tests read the new `contracts.yaml` and `conduit.yaml`. Report the totals;
what the five new tests printed, taken from `cargo test --release --test wrapper v2_account --
--nocapture` and `cargo test --release --test wrapper v2_sign_out -- --nocapture`; the digest that
`package` prints (it is a host build's); and decision 9's replies, each `accepted` and each
diagnostic's `ruleId` and text. Say on which system the tests ran.

Then show that the first new test fails where the hook asks for a refresh. In
`packages/agent-codex/`:

```bash
cp src/lib.rs target/lib.rs.fixed
```

Make one edit to `src/lib.rs`: in the params of the `account/read` request, `json!({"refreshToken":
false})`, replace `false` with `true`. Then run
`cargo test --release --test wrapper v2_account_hook_reads_codex`; it must fail, at the assertion on
the lines of `account-requests.jsonl`. Restore with `cp target/lib.rs.fixed src/lib.rs` (not `mv`,
so that cargo rebuilds), rerun the same command, which passes, and check that
`cmp target/lib.rs.fixed src/lib.rs` prints nothing.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, and give the last line of every check and each count above.
Tests that bind loopback ports may fail in your sandbox; if they do, say so, and the admin reruns
them. Where the sandbox denies PID-scoped `ps`, `v2_client_tree_stop_leaves_no_survivor` stops at
its preflight; say so, report 22 passed and that one failed for that reason, and the reader reruns
it on the Mac.

The admin then runs `python3 tools/check_private_paths.py` and `git diff --cached --check` on the
staged change. This order is a checkpoint on the Codex lane: it does not merge alone, and its Record
is written at the foot of this file on the lane's branch, as orders 050c's to 050h's are on
`wo/050-agents-0.2.0`. A fresh Codex verifier and a fresh read-only Opus reader read the result
before the checkpoint stands ([ruling 79](../proposals/ruling-79/README.md), point 2). The local
gate that [ruling 80](../proposals/ruling-80/README.md) has a merge wait on, `tools/gate.py`, runs at
order 050n's merge, read with the expected engine failure known. No archive built from this lane is
installed as `codex.v2` anywhere; the coordinator builds `codex-account-check.v1` from the lane's
tip, after order 050m, only in A5's scratch state directory, as experiment 027 pre-registers.
