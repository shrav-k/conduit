# Work order 050h (P2h): the retire hook: each `.v2` Agent signs out at retirement (D53)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access only for tests that
bind a loopback port; it adds no crate, so cargo fetches nothing, and you use the network for
nothing else. Run one cargo command at a time. Never print a whole large file: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Never
read or print any process's environment or arguments, never run a command that lists every
process on the machine, such as `ps -e`, `ps aux` or `top`, and never signal a process that a test
of this order did not start. Never run the real `codex` or `claude` programs, not even for their
help or version, and sign in nowhere. This order adds a client logout: every `Config` a test builds
and every wrapper a test starts uses the package's scripted client (`--client-program`), as every
test at `BASE` does, so that no logout ever reaches a real client. The tests use temporary
directories only, and the engine is run only to initialize a store and register projects.

## What and why

**Where this order starts.** Your tree is the branch that the coordinator names as `BASE`, not
`main`: `wo/050-agents-0.2.0`, which holds orders 050a, 076 and 078, 050a's Record, orders 050c,
050d and 050e, each with its Record, and order 050f, verified, rebased onto `main`. `BASE`'s tip
holds this order file, committed: the coordinator rebases the branch onto `main` after this order's
text merges, and sets `BASE` to the branch's tip. Before you change anything, check from the
repository root that you find there what that gives, and stop, change nothing and report if any
check prints otherwise. The Python lines need a Python with PyYAML, as `tools/check_schemas.py` does
(AGENTS.md, "Checks").

```bash
git merge-base --is-ancestor e4ceb4c8 "$BASE"; echo "main $?"
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/050h-agents-v2-retire-hook.md >/dev/null; echo "order $?"
git status --short | wc -l
grep -c '^## Record' docs/work-orders/050a-agents-v2-own-tools.md docs/work-orders/050c-agents-v2-client-home.md docs/work-orders/050d-agents-v2-process-tree.md docs/work-orders/050e-agents-v2-failed-turn-fields.md
for p in agent-codex/conduit.yaml agent-claude-code/conduit.yaml; do echo "$p $(git rev-parse "$BASE:packages/$p")"; done
grep -c 'template sets apiKey' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs packages/agent-codex/README.md
grep -c 'since the Agent holds no API key (ruling 83)' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs
grep -cE '^`(claude auth logout|codex logout)`: pass, a revocation scoped to that home.s token alone' experiments/027-a5-sign-in/results-2026-11-17-logout-source.md
python3 -c "import yaml; [print(n, 'account' in s['schema']['required'], s['schema']['properties']['account'], 'apiKey' in s['schema']['properties'], 'oneOf' in s['schema'], s.get('toolsDirectory'), s['requires'], s['implementation']) for p, n in [('agent-codex', 'codex.v2'), ('agent-claude-code', 'claude-code.v2')] for s in [[d for d in yaml.safe_load_all(open(f'packages/{p}/contracts.yaml')) if d['metadata']['name'] == n][0]['spec']]]"
grep -c 'retire' packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml
grep -ci 'logout' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs packages/agent-codex/tests/fixtures/client.rs packages/agent-claude-code/tests/fixtures/client.rs
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'
diff packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs | grep -c '^[<>]'
diff packages/agent-codex/tests/fixtures/client.rs packages/agent-claude-code/tests/fixtures/client.rs | grep -c '^[<>]'
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs
```

They print `main 0` (`e4ceb4c8` is `main` after orders 048a-1 and 050g merged), `head 0`,
`order 0` and `0`; then `1` for each order file (the four Records); then

```text
agent-codex/conduit.yaml 58c0fbdeaaa57f670d9ed06c9a0d46ecea45d990
agent-claude-code/conduit.yaml d6e1a999e5e3718dd9209254b1c78481057da047
```

(the two manifests 050e's Record describes, which 050f does not change; the greps after them and
the Python line check what 050f brings, its `.env` words included); then `0` for each of the five
files (no `.env` refusal still gives the reason "since an API key is used only where a template
sets apiKey") and `1` for each `lib.rs` (the reason order 050f gives it, under ruling 83's What
follows); `2` (order 050g's two outcome lines, both a pass); then, from what order 050f brings,

```text
codex.v2 True {'type': ['string', 'null']} False False {'directory': '/directory'} [] {'#turn.v1': '#wrapper-v2'}
claude-code.v2 True {'type': ['string', 'null']} False False {'directory': '/directory'} [] {'#turn.v1': '#wrapper-v2'}
```

(`account` required, a string or null with no default, no `apiKey` and no `oneOf`, under
[ruling 83](../proposals/ruling-83/README.md); `toolsDirectory`; and still no `requires` and one
default). These two greps and this Python line, and not a Record, are what check order 050f's
work: its Record is written only after its reader, so this order may start from 050f's verified
branch before that Record, and no line above asks for it.
Then the rest print `0` for each of the four files (no retire hook yet); `0` for each of the four
sources (no logout yet); `2` (the two `lib.rs` differ in line 18 alone, the `CLAUDE` constant); `0`
(the two `main.rs` are one file); `7` (the two fixtures differ in Claude Code's `STALE_INIT` block
alone); and `15` for each `wrapper.rs`.

**What it builds.** One behavior, in both packages: at an instance's retirement, each `.v2` Agent
signs its client out of the login it keeps for the Agent's own home, and empties that home. Each
`.v2` Type lists `retire-hook.v1` under `requires` with its default in `implementation`, the item
that already serves `turn.v1`; that item serves the hook as a second tool, `retire`; the hook stops
the client's tree, runs the client's own logout in the Agent's home alone, and on its success
removes every entry in the home; and each `.v2` description gains Q1 (iv)'s sentence. After it the
branch holds `codex.v2` in the shape it keeps if the live check fails (`requires` holding
`retire-hook.v1` alone), and its tip is where the Claude and Codex lanes split (order 117's 050
entry).

It carries, from [the coordinator's plan of 050](117-plan-s2-s3.md) (order 117's 050 entry: "050h,
the retire hook: each `.v2` Agent signs out at retirement (D53)"):

- [D53](../proposals/full-review/decisions.md), the full review's decision: "In 050 each Agent
  package's retire hook signs its client out and empties its home, and its contract says so", which
  answers [FR-431](../proposals/full-review/findings/E04-signin-isolation.md#fr-431): "No operation
  signs an Agent out [...] It can only be the package's own retire hook", "as `docker logout` and
  `gh auth logout` remove one stored credential". The review's verdict M54
  ([verdicts.md](../proposals/review-signin-2026-11-16/verdicts.md)) gives the form: "listed under
  `requires` with a default pointing at the wrapper, signing the client out and emptying its home".
- From the targeted review's [section 6](../proposals/review-signin-2026-11-16/README.md#6-must-state-lists),
  which the owner accepted with [ruling 82](../proposals/ruling-82/README.md):
  [`claude-code.v2`](../proposals/review-signin-2026-11-16/README.md#050-claude-codev2)'s item 1,
  its `retire-hook.v1` part ("`requires` lists [...] `retire-hook.v1`, each with a default in
  `implementation`"; its three sign-in hooks are orders 050i and 050j's), and
  [`codex.v2`](../proposals/review-signin-2026-11-16/README.md#050-codexv2)'s item 1 in the shape
  common to both outcomes of the live check ("`requires` holds `retire-hook.v1` alone with its
  default" on its fail; orders 050l and 050m add the rest on a pass); item 4's done-when, its
  retirement part ("a marker in a synthetic default home stays untouched after an undo, a
  retirement and a turn": the turn part is order 050c's, the undo orders 050i and 050l's), and
  M61's "the wrapper refuses to start [...] a logout where `CONDUIT_STATE_DIR` is unset or the home
  resolves outside it"; item 5, for the retire hook's error paths ("a fake home with a canary
  token and claims driven through every error path, each canary absent from every result,
  diagnostic, record and log"); and item 6, for Q1 (iv).
- Q1 (iv), as [ruling 82](../proposals/ruling-82/README.md#the-decisions), point 5, accepts it:
  "D53's retire-hook sentence, with the login kept where the client keeps it for that directory and
  no store named". Sentences (i) and (ii) are order 050f's, (iii) is at `BASE`, (v) is 050m's and
  (vi) 050k's.
- Item 8 of both lists, through [order 050g](050g-logout-revocation-read.md), merged: both outcome
  lines pass, "a revocation scoped to that home's token alone", so the hook signs out with each
  client's own command, and item 8's fail branch (removing the credential without the client
  command) is taken for neither. 050g's Record leaves its reader's R2 to the coordinator before
  this order: Claude Code's pages state a revocation of the credential that a keyless Console
  sign-in wrote, and "for a claude.ai login the pages read state no revocation". The coordinator
  reads that as item 8's pass by silence, the state finding V10 began from, and runs
  `claude auth logout` all the same: only the client knows where it keeps the login for a directory
  (on macOS, a Keychain entry keyed to that directory, which emptying the home would not reach),
  so emptying the home alone could leave the login live.

**What the specification already says, so that nothing here changes it.**
[TPL-018](../../spec/promises/templates.yaml) dispatches `retire-hook.v1` "within that invocation,
before the stop of TPL-009", for an instance "for which a setup was ever dispatched", asking first
for the run where its runtime provider reports it `stopped`; "the hook must not depend on the state
of a run that an earlier retirement stopped"; and a template "takes the default that the type
states" where its type lists the hook under `requires` (TPL-004). The core Operation
(`spec/core/operations.yaml`) adds: "It has no inputs", "The stop is asked for whatever came of the
hook. A hook that did not succeed keeps the shutdown from being confirmed: the shutdown reports
unknown or pending under OPS-022 and LIF-024. A further retirement dispatches it again". TPL-019
lets an Agent have "at most one item that is run": "One item that implements several of the
selected operations is one run, which serves them all". [PKG-020](../../spec/promises/packages.yaml)
has a plain item give `tools` for each operation it implements, and "the arguments of a call are the
operation's inputs under their own names"; ruling 82's o1 keeps the retire hook plain ("the
account, sign-out and retire hooks need no keys"). So the default is the turn item itself, marked
`plain` as it is, with a second tool that takes no arguments.

**What the branch already does, so that you do not redo it.** In each package's `src/lib.rs` (the
two files are one source; line 18's `CLAUDE` selects the client) and `src/main.rs` (one file):

- `Config::from_env` reads `CONDUIT_STATE_DIR` last in its `.v2` block, after
  `AGENT_TURN_DEADLINE_SECONDS`, `AGENT_DIRECTORY` and `AGENT_EFFORT`, so that any earlier refusal
  leaves `state_dir` unset; it refuses an unset, empty or relative value with
  `client home (VAR): CONDUIT_STATE_DIR is unset` or `... is not an absolute path`, VAR being
  `HOME_VARIABLE`.
- `client_home` (order 050c) checks and, where it is missing, creates `client-home` in the state
  directory with mode 0700, refuses with texts that begin `client home (`, and in Codex refuses a
  home that holds a `.env`. `CREDENTIAL_STORE` is `cli_auth_credentials_store="file"`.
- `Stream::spawn` takes the tree's state lock, refuses with `client did not start` once TERM's stop
  has closed the tree, builds the client's command (`process_group(0)` for `.v2`, `env_clear`, the
  config's `env`, `HOME_VARIABLE` set to the home, both API-key variables removed, `current_dir`
  the client directory, `kill_on_drop`), spawns it, and records its group's leader with its start
  time.
- `ClientTree::stop` and `Session::shutdown` return a `StopReport` whose `note()` names survivors or
  an unconfirmed stop (orders 050d and 050e); `exit_cause` gives `client exit: status N`,
  `client exit: signal N` or `client exit: status unknown`.
- `Session::turn` refuses with the configuration's error, then with
  `session failed at turn K: CAUSE; implicit restart refused`, before any launch.
- `main.rs`'s `Wrapper` lists one tool, `turn`, answers any other name with
  `ErrorData::invalid_params("unknown tool", None)`, builds a success with
  `result.result_type = Some(ResultType::COMPLETE)` and a failure with `CallToolResult::error`.
- The scripted client (`tests/fixtures/client.rs`) writes `client-started` in its working directory
  at every start, then reads its arguments; it has no logout.

## The decisions it applies

1. **The Types.** In each `.v2` Type of `contracts.yaml` (`codex.v2`, `claude-code.v2`), and in no
   other document of the file:
   - `  requires: []` becomes `  requires: ['github.com/shrav-k/conduit/core#retire-hook.v1']`;
   - `  implementation: {'#turn.v1': '#wrapper-v2'}` becomes the three lines

     ```yaml
       implementation:
         '#turn.v1': '#wrapper-v2'
         'github.com/shrav-k/conduit/core#retire-hook.v1': '#wrapper-v2'
     ```

   - and `metadata.description` gains, at its end after `must be longer.`, one space and the
     sentence S, the same in both:

     ```text
     When the Agent is retired, its retire hook signs the client out of the login it keeps for this Agent's own home, wherever the client keeps that login for that directory, and then empties that home.
     ```

     Both descriptions are folded scalars (`>-`): wrap S at the same four-space indent within 100
     columns; the check in "Done when" compares the text YAML reads.

   The core operation is named by the core package's full address, since `#retire-hook.v1` would
   name an export of this package (NAM-022). Nothing else in either Type changes.

2. **The items.** In each package's `conduit.yaml`, in `wrapper-v2` (and, in `agent-codex`, in
   `wrapper-v2-path`, line for line the same, which `tests/manifest.rs` checks):
   - `    implements: ['#turn.v1']` becomes
     `    implements: ['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1']`;
   - under `tools`, after `      '#turn.v1': turn`, add
     `      'github.com/shrav-k/conduit/core#retire-hook.v1': retire`.

   `plain: true` stays. The `.v1` item `wrapper` does not change, and implements no hook.

3. **The state directory first.** In `Config::from_env`, move the lines that read
   `CONDUIT_STATE_DIR`, check that it is absolute and set `c.state_dir` from the end of the
   `if v2 {` block to its start, before `let n = required("AGENT_TURN_DEADLINE_SECONDS")?;`,
   unchanged. A configuration with one refusal answers as at `BASE`; one whose state directory is
   good keeps `state_dir` set whatever input is refused after it, so that the hook can sign the
   client out of an instance whose `directory` no longer exists.

4. **The hook, in `src/lib.rs`.** Add, after `TREE_UNCONFIRMED`:

   ```rust
   /// How long the retire hook waits for the client's logout (D53).
   pub const LOGOUT_DEADLINE: Duration = Duration::from_secs(60);
   /// The retire hook's answer once the client is signed out and its home emptied.
   pub const RETIRED: &str = "client signed out and its home emptied";
   /// Every turn once the retire hook has begun.
   pub const RETIRED_REFUSAL: &str = "session retired; implicit restart refused";
   ```

   and, after `client_home`:

   ```rust
   /// The client's own logout, run with the Agent's home alone (order 050g).
   pub fn logout_arguments() -> Vec<String> {
       if CLAUDE {
           vec!["auth".into(), "logout".into()]
       } else {
           vec!["logout".into(), "-c".into(), CREDENTIAL_STORE.into()]
       }
   }
   ```

   - **One command builder.** Add `fn client_command(c: &Config, home: Option<&Path>, args:
     &[String]) -> tokio::process::Command`, which holds exactly the builder calls of
     `Stream::spawn` named above but the three `std*` calls and `spawn`: `Command::new(&c.program)`,
     `process_group(0)` where `c.v2`, then `args`, `env_clear`, `envs(&c.env)`, `HOME_VARIABLE` set
     to `home` where it is `Some`, `env_remove("ANTHROPIC_API_KEY")`,
     `env_remove("OPENAI_API_KEY")`, `current_dir(c.client_directory())` and `kill_on_drop(true)`.
     `Stream::spawn` builds its command with it and sets its three `std*` as at `BASE`; its
     behavior does not change.
   - **The logout.** Add `async fn logout(c: &Config, home: &Path, tree: &ClientTree, deadline:
     Duration) -> Result<()>`, which:
     1. takes the tree's state lock, refuses with `client logout did not start` where the tree is
        closed, spawns `client_command(c, Some(home), &logout_arguments())` with stdin, stdout and
        stderr all `Stdio::null()` (the client's output is never read), refuses with
        `client logout did not start` where the spawn or its PID fails, records its group's leader
        with its start time as `Stream::spawn` does, and releases the lock;
     2. waits for it, within `deadline`: exit status 0 returns `Ok(())`; another status fails with
        `client logout failed: ` followed by `exit_cause(status)`; an error of the wait fails with
        `client logout failed: client exit: status unknown`;
     3. at the deadline runs `tree.stop(false)`, then kills and reaps the child, and fails with
        `client logout failed: no exit within N seconds`, N being `deadline.as_secs()`, followed by
        `; ` and the stop's `note()` where it has one.
   - **Emptying.** Add `fn empty_home(home: &Path) -> Result<()>`, which reads the home's entries
     and removes each: by `fs::remove_dir_all` where its `fs::symlink_metadata` is a directory, and
     by `fs::remove_file` otherwise, so that a link is removed and what it points to is not. Any
     error of the read or a removal fails with `client home (VAR): cannot empty the home`, written
     with `HOME_VARIABLE`, naming no entry and no path. The home itself, and its mode, stay.
   - **The session.** `Session` gains `retired: bool`, false in `Session::new`. In `Session::turn`,
     after the configuration's refusal and before the failed session's,
     `if self.retired { bail!("{RETIRED_REFUSAL}"); }`. Add:

     ```rust
     pub async fn retire(&mut self) -> Result<String> {
         self.retire_within(LOGOUT_DEADLINE).await
     }
     /// The retire hook, with the logout's deadline given; `retire` gives LOGOUT_DEADLINE.
     pub async fn retire_within(&mut self, deadline: Duration) -> Result<String> {
         ensure!(self.config.v2, "retire hook: a .v1 Agent has none");
         if self.config.state_dir.is_none()
             && let Some(error) = &self.config.configuration_error
         {
             bail!("{error}");
         }
         self.retired = true;
         if let Some(note) = self.shutdown().await.note() {
             bail!("{note}");
         }
         let home = client_home(&self.config)?;
         logout(&self.config, &home, &self.tree, deadline).await?;
         empty_home(&home)?;
         Ok(RETIRED.into())
     }
     ```

     So the hook stops the conversation's client first, as a failed turn does, since a running
     client may still hold or rewrite the login; fails with the stop's note, running no logout,
     where a process outlived the stop or the stop is unconfirmed; checks the home with every
     refusal of a launch (Codex's `.env` included) and creates it where it is missing, so that a
     login the client keeps outside the home for that directory is still signed out; empties the
     home only after the logout exits 0, so that a failed logout leaves the login in place for the
     next attempt to revoke; and can be called again, in the same run or a new one, by a further
     retirement. Once it has begun, every turn is refused, whatever its outcome. An error of
     `client_home`, `logout` and `empty_home` is the hook's answer as it is.

5. **The tool, in `src/main.rs`**, which calls `Session::retire` and imports nothing new.
   - `Wrapper` becomes `struct Wrapper(Arc<Mutex<Session>>, bool);`, the second field
     `config.v2`, read in `run` before `Session::new` takes the config, and passed where the serving
     task builds `Wrapper(serving.clone(), v2)`.
   - `list_tools` lists `turn` as at `BASE`, and, where the second field is true, after it:
     `{"name":"retire","description":"Sign the client out of this Agent's own home and empty the home, at the Agent's retirement.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}}`.
     A `.v1` wrapper lists `turn` alone, as in 0.1.1.
   - `call_tool` begins, before the `turn` check:

     ```rust
     if r.name == "retire" && self.1 {
         if r.arguments.as_ref().is_some_and(|args| !args.is_empty()) {
             return Err(ErrorData::invalid_params("expected no arguments", None));
         }
         let mut session = tokio::select! {
             biased;
             _ = context.ct.cancelled() => {
                 return Err(ErrorData::internal_error("retire cancelled", None));
             }
             session = self.0.lock() => session,
         };
         // Once the hook has the session, no cancellation cuts the sign-out.
         let result = match session.retire().await {
             Ok(text) => {
                 let mut result = serde_json::from_value::<CallToolResult>(
                     json!({"content":[{"type":"text","text":text}],"isError":false}),
                 )
                 .unwrap();
                 result.result_type = Some(ResultType::COMPLETE);
                 result
             }
             Err(error) => CallToolResult::error(vec![ContentBlock::text(error.to_string())]),
         };
         return Ok(result.into());
     }
     ```

     A `.v1` wrapper answers `retire` as any unknown tool. The hook's result has one text block, no
     `structuredContent` and no `_meta`. A turn in flight when the hook arrives holds the session,
     and the hook waits for it. The TERM task and the end of input do not change. The two
     `main.rs` stay one file.

6. **The scripted client**, in each `tests/fixtures/client.rs`, the same in both. In `main`, read
   `args` first (the line that reads them moves above the write of `client-started`), and, where
   `args == ["auth", "logout"]` or `args == ["logout", "-c", "cli_auth_credentials_store=\"file\""]`,
   call `logout(&args)` and return before `client-started` is written. Add `fn logout(args:
   &[String])`, which:
   - reads `CLAUDE_CONFIG_DIR` where `args[0]` is `auth`, else `CODEX_HOME`, and panics with
     `logout without the client home` where it is unset, so that a logout never reaches a default
     home;
   - appends one line to `logouts.jsonl` in its working directory, the JSON object
     `{"args":ARGS,"home":HOME,"environment":ENV,"entries":ENTRIES,"cwd":CWD}`, `ENTRIES` the names
     in the home, sorted;
   - reads `fixture-login` in the home, or an empty text, and writes it to its stdout after
     `logout output ` and to its stderr after `logout error ` (client output that the wrapper
     discards);
   - where `logout-exit` is in its working directory, exits with the status that file holds;
   - where `logout-hang` is, sleeps 30 seconds;
   - removes `fixture-login` from the home where it is there, and exits 0.

7. **The tests**, in each package's `tests/wrapper.rs`, the same in both. Add to the test file the
   constants `LOGIN_TOKEN` `synthetic-login-token-canary` and `LOGIN_CLAIM`
   `synthetic-claim-canary`, and a helper that writes `fixture-login` into a directory holding
   `{"token":LOGIN_TOKEN,"claims":{"account":LOGIN_CLAIM}}`. Import `RETIRED`, `RETIRED_REFUSAL`
   and `logout_arguments`. Every result and error text that a test below reads holds neither
   canary, and the test asserts it.
   - **`stdio_v2_defaults_and_invalid_configuration_remain_callable`** changes in one assertion:
     the names of the listed tools are `["turn"]` for the case `v1` and `["turn", "retire"]` for
     every other case. Every other test keeps passing unchanged.
   - **A new test, `v2_retire_signs_out_and_empties_only_its_home`**, through `Session`, for a
     `directory` null and then a named one, set up as
     `v2_client_home_in_state_directory_leaves_default_home_untouched` sets up each (the default
     home planted, `MCP_TOOL_TIMEOUT` for Claude Code, `prepare`). It runs the turn `HOME`, then
     writes `fixture-login` into the client home, creates `nested/deeper/file` in it, and a link
     `outside-link` in it to the default home. `retire()` answers `RETIRED`. `logouts.jsonl` in the
     client directory holds one line, whose `args` equal `logout_arguments()`, whose `environment`
     equals, under `assert_client_environment`, the launch's (`c.env` less both API-key variables,
     with `HOME_VARIABLE` the home's canonical path), whose `cwd` is the client directory, and
     whose `entries` are `fixture-home-write`, `fixture-login`, `nested` and `outside-link`, with
     `tmp` for Codex, sorted. The client home then holds no entry, and its mode is 0700; the
     default home passes `assert_default_home`. A second `retire()` answers `RETIRED`, with a
     second line whose `entries` are empty. With `client-started` removed, the turn `TEST` fails
     with `RETIRED_REFUSAL` exactly, and starts no client. It `eprintln!`s each line's `args` and
     `entries`.
   - **A new test, `v2_retire_error_paths_hold_no_canary`**, through `Session`, one case at a time,
     each in a fresh temporary directory with the default home planted, `c.v2` true and
     `directory` null, `prepare` run, and the client home made with mode 0700 holding
     `fixture-login` unless the case says otherwise. Each call is `retire()` but in `deadline`.

     | Case | Set up | Answer | Logouts | Home's entries after |
     | --- | --- | --- | --- | --- |
     | `no-client` | `c.program` a missing file under the temporary directory | `client logout did not start` | none | `fixture-login` |
     | `exit` | `logout-exit` holding `3` in `c.cwd` | `client logout failed: client exit: status 3` | 1 | `fixture-login` |
     | `deadline` | `logout-hang` in `c.cwd`; the call is `retire_within(Duration::from_secs(3))` | `client logout failed: no exit within 3 seconds` | 1 | `fixture-login` |
     | `empty` | the home also holds `locked/file`, `locked` with mode 0500 | `client home (VAR): cannot empty the home` | 1 | `locked` |
     | `outside-link` | the home a link to `elsewhere`, a directory beside `state` holding `fixture-login` | `client home (VAR): the home resolves outside CONDUIT_STATE_DIR` | none | `elsewhere` keeps `fixture-login` |
     | `unset` | `c.state_dir` `None` | `client home (VAR): CONDUIT_STATE_DIR is unset` | none | `fixture-login` |
     | `configuration` | `c.configuration_error` `invalid configuration: effort (AGENT_EFFORT): expected lowercase ASCII letters` | `RETIRED` | 1 | none |
     | `configuration-unset` | `c.state_dir` `None`, `c.configuration_error` `invalid configuration: client home (VAR): CONDUIT_STATE_DIR is unset` | that configuration text | none | `fixture-login` |
     | `dotenv`, Codex alone | the home also holds `.env` with `AGENT_DOTENV` | `client home (VAR): ` then `DOTENV_REFUSAL` | none | `.env`, `fixture-login` |

     VAR is `HOME_VARIABLE`, and each answer is compared exactly: `RETIRED` as the `Ok` text, the
     others as the error's text. The `empty` case is skipped, with an `eprintln!` naming it, where
     `libc::geteuid()` is 0. Then, in each case: the turn `TEST` fails with the configuration text
     in the two `configuration` cases and with `RETIRED_REFUSAL` in the others; the default home
     passes `assert_default_home`; and `client-started` is absent from `c.cwd`. And the failures
     that leave a login are tried again, which a further retirement does: in `exit`, with
     `logout-exit` removed; in `deadline`, with `logout-hang` removed; and in `empty`, with
     `locked` given mode 0700; each second `retire()` answers `RETIRED`, with one more logout and a
     home with no entry. The survivor path (a stop's note) is order 050d's machinery and is not a
     case here. It `eprintln!`s each case's answers.
   - **A new test, `v2_retire_tool_through_the_wrapper`**, through the wrapper binary over stdio,
     started as `stdio_v2_defaults_and_invalid_configuration_remain_callable` starts it
     (`--client-program`, `AGENT_ACCOUNT_SERVICES` `false`, `AGENT_TURN_DEADLINE_SECONDS` `60`,
     `CONDUIT_STATE_DIR` the config's, `HOME_VARIABLE` set to a path elsewhere), over the 2026-07-28
     discovery as its case `modern` serves it, with the default home planted and the client home
     made with mode 0700 holding `fixture-login`. The listed tools' names are
     `["turn", "retire"]`. A `retire` call with the arguments `{"extra":true}` is an error of the
     call. The turn `TEST` succeeds. The `retire` call with `{}` answers, as JSON, `isError` false,
     `resultType` `complete`, `content` exactly one text block `RETIRED`, and no
     `structuredContent` and no `_meta` member. The turn `TEST` then answers `isError` true,
     `resultType` `complete` and one text block, `RETIRED_REFUSAL`. `logouts.jsonl` in `c.cwd`
     holds one line, whose `args` equal `logout_arguments()` and whose
     `environment[HOME_VARIABLE]` is the client home's canonical path; the client home holds no
     entry, and the default home passes `assert_default_home`. After `cancel`, the wrapper exits
     successfully, and its stderr holds neither canary, nor `synthetic-test-token`,
     `synthetic-anthropic-key` or `synthetic-openai-key`. Then a `.v1` wrapper, started in a fresh
     temporary directory as `stdio_exposes_only_turn_and_setup_never_launches_client` starts it,
     lists `["turn"]`, answers a `retire` call with `{}` as an error of the call, and leaves no
     `logouts.jsonl`. It `eprintln!`s the `.v2` `retire` result.

   The test count goes from 15 to 18 in each package.

8. **The READMEs**, both, in the package's own client's terms: Codex and `codex logout` with
   `CODEX_HOME`, or Claude Code and `claude auth logout` with `CLAUDE_CONFIG_DIR`. Write no
   absolute local path, and no client version beyond those the README already records.
   - In the opening paragraph, the sentence "The plain MCP wrapper exposes only `turn`; the
     manifest maps the versioned operation to it." becomes "The plain MCP wrapper exposes `turn`,
     and each `.v2` item also `retire`; the manifest maps each versioned operation or hook to its
     tool.", the same words in both READMEs, since order 203 starts from them.
   - In `## The .v2 Agent's own home`, after "The engine's own runner removes the home with the
     instance (PRV-029, LIF-035).", add "Before that, the retire hook signs CLIENT out and
     empties the home.", CLIENT being the client's name and "retire hook" a link to the anchor
     `the-v2-retire-hook`.
   - In `agent-codex/README.md` alone, in `## The wrapper-v2-path item`: "The default of
     `codex.v2` is still `wrapper-v2`." becomes "The defaults of `codex.v2` for `turn.v1` and
     `retire-hook.v1` are still `wrapper-v2`."; the template sentence's
     `implementation: {...}` becomes
     `implementation: {example.org/conduit/agent-codex#turn.v1: example.org/conduit/agent-codex#wrapper-v2-path, github.com/shrav-k/conduit/core#retire-hook.v1: example.org/conduit/agent-codex#wrapper-v2-path}`;
     and after that sentence add "A template that selects it for `turn.v1` alone takes `wrapper-v2`
     for the retire hook by default, two items that are run, and is rejected (TPL-019)."
   - Add, at the end, `## The .v2 retire hook`, which says, in this order:
     - the Type lists `retire-hook.v1` under `requires`, and its default is `wrapper-v2`, the item
       that serves `turn.v1`, so one run serves both (TPL-004, TPL-019); the item serves it as the
       tool `retire`, which takes no arguments, and the `.v1` item lists no such tool; at the
       retirement of an instance that was ever set up, the engine calls it before it stops the
       run, asking for a run first where none is running (TPL-018);
     - the hook's four steps, as a numbered list: it stops the client's process tree as a failed
       turn does, and from then on refuses every turn with `session retired; implicit restart
       refused`, and where a process outlived the stop or the stop is unconfirmed it fails with
       the stop's note; it checks the home as every launch does, with the same refusals, creating
       it where it is missing; it runs the client's logout, written as `logout_arguments()` gives
       it, with the home variable set to the home and the launch's environment, discards its
       output, and waits at most 60 seconds; and where the logout exits with status 0 it removes
       every entry in the home, removing a link and not what it points to, keeps the home itself,
       and answers `client signed out and its home emptied`;
     - its failures, each a fixed text: `client logout did not start`;
       `client logout failed: client exit: status N`, `... signal N` or `... status unknown`;
       `client logout failed: no exit within 60 seconds`, after which the logout's tree is stopped;
       and `client home (VAR): cannot empty the home`; with the configuration's refusal where the
       state directory is unset or not absolute. A failure before the logout's exit leaves the
       home as it was, so that the next attempt can still revoke the login. A failed hook keeps the
       shutdown from being confirmed (OPS-022, LIF-024), and a further retirement calls it again
       (TPL-018). Its answers hold fixed words and numbers, and the configuration's refusal where
       there is one, never the client's output or the home's content;
     - of the `.v2` inputs read after it, `turnDeadlineSeconds`, `directory` and `effort`, the
       hook needs none, only the state directory: an instance whose `directory` no longer
       exists, or whose other such input the wrapper refuses, is still signed out; a refusal of
       an input read before it, such as `accountServices` (`AGENT_ACCOUNT_SERVICES`), answers
       with that configuration text, and without `CONDUIT_URL` or `CONDUIT_IDENTITY_DIR`, which
       the engine always sets, the wrapper cannot tell that it is `.v2`, lists no `retire` and
       answers it as an unknown tool; a turn still running when the hook arrives is waited for;
     - what the logout revokes, as [order 050g](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#the-outcome-under-item-8)
       read it, with no client run: in `agent-codex/README.md`, that Codex's source at the tag that
       read names sends one revocation request naming the home's own stored token, then removes
       the home's login, [C3](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#c3-whether-it-revokes-at-the-vendor-and-the-scope),
       a reading of the source and not an observation; in `agent-claude-code/README.md`, that
       Claude Code's pages state that its logout removes and revokes the credential a keyless
       Console sign-in wrote, and state no revocation for a claude.ai login,
       [L2](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#l2-whether-it-revokes-anything-and-the-scope),
       and that the hook runs the client's own logout because only the client knows where it
       keeps the login for that directory;
     - and, as limits: the hook takes any exit status but 0 as a failure, and what each client's
       logout exits with in a home that holds no login was not read: were it not 0, the
       retirement of an Agent never signed in, as one whose `account` is null, would stay
       unconfirmed, the hook failing again at each further retirement; a turn in flight that
       holds the session past the engine's call timeout (120 seconds by default) leaves the
       hook's outcome unknown, and a hook whose outcome is unknown is not dispatched again until
       it is reconciled, so the login stays; in `agent-claude-code/README.md` alone, the pages do
       not say whether `claude auth logout` in one `CLAUDE_CONFIG_DIR` leaves untouched an
       Anthropic profile outside it, where Claude Code keeps a keyless Console sign-in,
       [L3](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#l3-one-login-for-each-home);
       a login on a machine the
       hook cannot reach, as on a node declared lost (PLC-027), stays until it is revoked at the
       vendor; under another runtime provider the hook runs where the run runs, on the instance's
       node; and the hook empties the home but does not remove it, which the engine's own runner
       does with the instance (PRV-029, LIF-035).

9. **Check the Types with the engine**, as order 050f's decision 6
   does. This checks what the package tests cannot: that each Type's `requires` and default
   resolve, and that the turn item and the hook are one run.
   - In `engine/`, run `cargo build --release`. Change nothing there.
   - In each package, run `cargo run --release --bin package`, which prints `ARCHIVE#SHA256`.
   - In a new temporary directory T outside the repository, for NAME `agent-codex` (Type
     `codex.v2`) and then `agent-claude-code` (`claude-code.v2`), run
     `engine/target/release/conduit init --store T/store-NAME`, then write projects under T, each
     a `conduit.yaml` that pins `example.org/conduit/NAME` at version 0.2.0 with that digest, as
     `engine/crates/shell/tests/agents.rs` writes one, and one template file, `kind: Agent`,
     `spec.type` `example.org/conduit/NAME#TYPE`, `config: {account: example-account}`:
     - R: the template `v2-retire`, with no `implementation`.
     - N, the negative control: the template `v2-retire-v1-item`, with
       `implementation: {github.com/shrav-k/conduit/core#retire-hook.v1: example.org/conduit/NAME#wrapper}`.
     - For `agent-codex` alone, P1: the template `v2-path-turn-only`, with
       `implementation: {example.org/conduit/agent-codex#turn.v1: example.org/conduit/agent-codex#wrapper-v2-path}`;
       and P2: the template `v2-path-both`, with that map and
       `github.com/shrav-k/conduit/core#retire-hook.v1: example.org/conduit/agent-codex#wrapper-v2-path`
       in it.
   - Register each in turn with
     `engine/target/release/conduit register --store T/store-NAME --package ARCHIVE#SHA256 PROJECT`.
     No project holds an instance, so none starts a wrapper or a client. Report each reply's
     `accepted` and its diagnostics.
   - Expected: R and P2 are accepted with no diagnostic; N is not accepted, with an error citing
     TPL-005 (`wrapper` does not implement the hook); and P1 is not accepted, with an error citing
     TPL-019 (two items that are run). If a reply differs, change nothing to make it pass: report
     it, since this order changes no engine source, and the coordinator decides.

10. **What this order does not do.** No `signIn`, no sign-in, account or sign-out hook and no login,
    which are orders 050i to 050m's; no input added or changed, and nothing about `apiKey`, which
    [ruling 83](../proposals/ruling-83/README.md) leaves out of `.v2`; no description sentence but
    S; no change to `.v1`, to `turn.v1`, to `conversation.v1`, to the `wrapper` item, or to
    `Cargo.toml`, `Cargo.lock`, `src/bin/package.rs`, `tests/package.rs`, `tests/manifest.rs` or
    Claude Code's `tests/fixtures/stale-init.json`; no wording of an existing refusal changed; and
    nothing under `engine/`, `conformance/`, `spec/` or `docs/`. The loader vector of item 1's
    done-when, the move of `conformance/fixtures/agents/` to 0.2.0 and the accounts guide's line
    that a lost machine's login stays until it is revoked at the vendor (D53) are order 050n's.
    This order's `dotenv` case reads the refusal of a home that holds a `.env` through the
    `DOTENV_REFUSAL` constant, so that it reads the words order 050f gives it. The engine's test
    `live_s1_scripted_agents_call_engine_with_distinct_run_identities`
    (`engine/crates/shell/tests/agents.rs`) fails on this branch by design until order 050n moves
    `conformance/fixtures/agents/` to 0.2.0, as [order 050a](050a-agents-v2-own-tools.md) records.
    It is the one expected failure on `wo/050-agents-0.2.0`, and it is not this order's: neither you
    nor the verifier counts it against this order. Do not run the engine's tests and do not change
    it.

## Who decided what

The sign-out at retirement and the emptied home are D53, the full review's decision, which stays the
coordinator's, open to the owner's strike; the review's item 1 (`requires` and a default), item 4's
retirement marker, item 5's canaries and item 6 are the review's must-state lists, which the owner
accepted with ruling 82; Q1 (iv) is the owner's (ruling 82, point 5, answer 7); and the hooks' plain
items are ruling 82's o1. [Ruling 83](../proposals/ruling-83/README.md) (the owner's, 2026-11-18)
gives the inputs that `BASE` holds. Order 050g's two outcome lines are its read under item 8;
reading Claude Code's line, with its reader's R2, as a pass by silence that still runs the client's
logout is the coordinator's; so is taking that logout as scoped to the Agent's home although order
050g's
[L3](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#l3-one-login-for-each-home)
found no statement of whether `claude auth logout` in one `CLAUDE_CONFIG_DIR` leaves an Anthropic
profile outside it untouched, which the owner may strike before a real retirement. The hook's
behavior for a home that holds no login (an Agent never signed in, such as one whose `account` is
null) rests on what each client's logout exits with there, which no order has read: the coordinator
settles it, from Codex's `run_logout` at the tag order 050g read and from Claude Code's pages,
before order 050n or the first real retirement, whichever comes first. The split of order 050 into
050b to 050o, this order's place on `wo/050-agents-0.2.0` after 050f and 050g, and its base are the
coordinator's, open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)).
Everything else here is the coordinator's too, each open to the owner's strike: the default being
the turn item, with the tool `retire`; the logout's arguments, with Codex's file store on its logout
too; the stop before the logout, and a survivor failing the hook; the state directory read first, so
that the hook runs whatever `.v2` input read after it is refused; the home created where it is
missing; emptying only after the logout exits 0, by entry, links removed and not followed, the home
kept; a turn in flight waited for, not cancelled, when the hook arrives; the 60-second deadline,
under the engine's default call timeout of 120 seconds; the refusal of every turn once the hook has
begun; the texts; a cancellation that never cuts a sign-out once begun; the wording of S, of the
tool's description and of the READMEs; the scripted client's logout; and the registration probe with
its negative control. Apply them as written and report them under `decisions` as the coordinator's.

## Scope

Change only, in `packages/agent-codex/` and `packages/agent-claude-code/`: `contracts.yaml`,
`conduit.yaml`, `src/lib.rs`, `src/main.rs`, `tests/wrapper.rs`, `tests/fixtures/client.rs` and
`README.md`. Nothing else, in the packages or outside them. The build's ignored output (`target/`,
`dist/`) may change, and so may anything under T.

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
git status --short | grep -vE '^ M packages/agent-(codex|claude-code)/(contracts\.yaml|conduit\.yaml|src/lib\.rs|src/main\.rs|tests/wrapper\.rs|tests/fixtures/client\.rs|README\.md)$' | wc -l   # 0
git diff --quiet "$BASE" -- packages/agent-codex/Cargo.toml packages/agent-claude-code/Cargo.toml packages/agent-codex/Cargo.lock packages/agent-claude-code/Cargo.lock packages/agent-codex/src/bin packages/agent-claude-code/src/bin packages/agent-codex/tests/package.rs packages/agent-claude-code/tests/package.rs packages/agent-codex/tests/manifest.rs packages/agent-claude-code/tests/fixtures/stale-init.json; echo $?   # 0
for NAME in agent-codex agent-claude-code; do head -c "$(wc -c < conformance/fixtures/agents/$NAME/contracts.yaml)" packages/$NAME/contracts.yaml | cmp - conformance/fixtures/agents/$NAME/contracts.yaml; echo "$NAME $?"; done   # agent-codex 0, agent-claude-code 0
grep -c "^  requires: \['github.com/shrav-k/conduit/core#retire-hook.v1'\]$" packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml   # 1 each
grep -c "^    implements: \['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1'\]$" packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml   # 2 for Codex, 1 for Claude Code
grep -c "^      'github.com/shrav-k/conduit/core#retire-hook.v1': retire$" packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml   # 2 for Codex, 1 for Claude Code
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'   # 2
diff packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs | grep -c '^[<>]'   # 0
diff packages/agent-codex/tests/fixtures/client.rs packages/agent-claude-code/tests/fixtures/client.rs | grep -c '^[<>]'   # 7
grep -c 'pub const LOGOUT_DEADLINE: Duration = Duration::from_secs(60);' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs   # 1 each
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs   # 18 each
grep -c '^## The .v2 retire hook$' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -c '(#the-v2-retire-hook)' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -c 'github.com/shrav-k/conduit/core#retire-hook.v1: example.org/conduit/agent-codex#wrapper-v2-path' packages/agent-codex/README.md   # 1
for NAME in agent-codex agent-claude-code; do tr '\n' ' ' < packages/$NAME/README.md | grep -c 'The plain MCP wrapper exposes `turn`, and each `.v2` item also `retire`; the manifest maps each versioned operation or hook to its tool\.'; done   # 1, 1
for NAME in agent-codex agent-claude-code; do tr '\n' ' ' < packages/$NAME/README.md | grep -c 'exposes only `turn`'; done   # 0, 0
grep -ci 'apikey' packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml   # 0 each
grep -rnE '/(Users|home)/|/(private/)?tmp/' packages/agent-codex packages/agent-claude-code --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

The third line is 050a's byte check: the 0.1.1 contracts stay a byte prefix of each package's
`contracts.yaml`.

The script below, run from the repository root with a Python that has PyYAML, must print exactly
the four lines below it: for each `.v2` Type, its `requires`, whether its `implementation` is
`BASE`'s with the hook's default added, whether `BASE`'s `requires` was empty, whether the rest of
its `spec` is `BASE`'s, whether its description is `BASE`'s followed by one space and S, and
whether the rest of the document is `BASE`'s; then, for each manifest, whether `wrapper` is
`BASE`'s, whether the rest of the manifest is `BASE`'s, whether each `.v2` item is `BASE`'s with the
hook added to `implements` and `tools` alone, and whether the items' names are `BASE`'s.

```bash
python3 - "$BASE" <<'EOF'
import subprocess, sys, yaml
base = sys.argv[1]
core = "github.com/shrav-k/conduit/core#retire-hook.v1"
s = ("When the Agent is retired, its retire hook signs the client out of the login it keeps for "
     "this Agent's own home, wherever the client keeps that login for that directory, and then "
     "empties that home.")
show = lambda path: subprocess.run(["git", "show", f"{base}:{path}"], capture_output=True, text=True, check=True).stdout
less = lambda d, keys: {k: v for k, v in d.items() if k not in keys}
for package, name in [("agent-codex", "codex.v2"), ("agent-claude-code", "claude-code.v2")]:
    path = f"packages/{package}/contracts.yaml"
    pick = lambda text: [d for d in yaml.safe_load_all(text) if d["metadata"]["name"] == name][0]
    now, was = pick(open(path).read()), pick(show(path))
    print(name, now["spec"]["requires"],
          now["spec"]["implementation"] == {**was["spec"]["implementation"], core: "#wrapper-v2"},
          was["spec"]["requires"] == [],
          less(now["spec"], ("requires", "implementation")) == less(was["spec"], ("requires", "implementation")),
          now["metadata"]["description"] == was["metadata"]["description"] + " " + s,
          less(now, ("spec", "metadata")) == less(was, ("spec", "metadata"))
          and less(now["metadata"], ("description",)) == less(was["metadata"], ("description",)))
for package, items in [("agent-codex", ["wrapper-v2", "wrapper-v2-path"]), ("agent-claude-code", ["wrapper-v2"])]:
    path = f"packages/{package}/conduit.yaml"
    now, was = yaml.safe_load(open(path)), yaml.safe_load(show(path))
    out = [package, now["implementations"]["wrapper"] == was["implementations"]["wrapper"],
           less(now, ("implementations",)) == less(was, ("implementations",))]
    for item in items:
        n, w = now["implementations"][item], was["implementations"][item]
        out.append(n["implements"] == w["implements"] + [core]
                   and n["tools"] == {**w["tools"], core: "retire"}
                   and less(n, ("implements", "tools")) == less(w, ("implements", "tools")))
    out.append(sorted(now["implementations"]) == sorted(was["implementations"]))
    print(*out)
EOF
```

```text
codex.v2 ['github.com/shrav-k/conduit/core#retire-hook.v1'] True True True True True
claude-code.v2 ['github.com/shrav-k/conduit/core#retire-hook.v1'] True True True True True
agent-codex True True True True True
agent-claude-code True True True True
```

In each of `packages/agent-codex/` and `packages/agent-claude-code/`, one command at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

`cargo test --release` reports, for `tests/wrapper.rs`, 18 passed and 0 failed, and every other
test target as at `BASE` (the library's unit tests 2 and the package test 1 in each, and in
`agent-codex` the manifest test 1); the package and manifest tests read the new `contracts.yaml`
and `conduit.yaml`. Report the totals; what the three new tests printed, taken from
`cargo test --release --test wrapper v2_retire -- --nocapture`; the digest that `package` prints
(it is a host build's); and decision 9's replies, each `accepted` and each diagnostic's `ruleId`
and text. Say on which system the tests ran.

Then show that the first new test fails without the emptying. In `packages/agent-codex/`:

```bash
cp src/lib.rs target/lib.rs.fixed
```

Make one edit to `src/lib.rs`: in `retire_within`, replace the statement `empty_home(&home)?;` with
`let _ = &home;`. Then run `cargo test --release --test wrapper v2_retire_signs_out`; it must fail,
at the assertion that the client home holds no entry after the first `retire()`. Restore with
`cp target/lib.rs.fixed src/lib.rs` (not `mv`, so that cargo rebuilds), rerun the same command,
which passes, and check that `cmp target/lib.rs.fixed src/lib.rs` prints nothing.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, and give the last line of every check and each count above.
Tests that bind loopback ports may fail in your sandbox; if they do, say so, and the admin reruns
them. Where the sandbox denies PID-scoped `ps`, `v2_client_tree_stop_leaves_no_survivor` stops at
its preflight; say so, report 17 passed and that one failed for that reason, and the reader reruns
it on the Mac.

The admin then runs `python3 tools/check_private_paths.py` and `git diff --cached --check` on the
staged change. This order is a checkpoint on `wo/050-agents-0.2.0`: it does not merge alone, and its
Record is written at the foot of this file on that branch, as orders 050c's to 050f's are. A fresh
Codex verifier and a fresh read-only Opus reader read the result before the checkpoint stands
([ruling 79](../proposals/ruling-79/README.md), point 2). The local gate that
[ruling 80](../proposals/ruling-80/README.md) has a merge wait on, `tools/gate.py`, runs at order
050n's merge, read with the expected engine failure known.
