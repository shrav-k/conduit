# Work order 050a (P2a): the .v2 Agent Types, with the Agent's own tools

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count. Never run the real `codex` or `claude` programs, not even for
their help or version. The tests use each package's scripted client. After you finish, the admin
replays your launch arguments against the real clients, with probes that sign in to nothing.

## What and why

[Ruling 70](../proposals/ruling-70/README.md)
([IN_FORCE.md, topic 9](../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach))
says that Agents run code. The first build's `codex.v1` and `claude-code.v1` turn the clients'
own shell, code execution and file tools off. So an Agent that Conduit starts cannot build or test
anything. This order adds `codex.v2` and `claude-code.v2` beside the `.v1` Types, in
`packages/agent-codex` and `packages/agent-claude-code`:

- a `.v2` Agent works in a directory that its template names;
- it uses its client's own tools there;
- its template sets its turn deadline;
- one input turns its own tools off.

This is the first step of A78, Conduit running its own work orders. The admin installs this
branch's archives in a store outside the repository and sends orders to `.v2` Agents through it.
Turns longer than about five minutes also need order 074's fix in the engine. That is not this
order's concern.

This order is the first part of spec batch 1's order 050 (P2), split off as 050a
([the plan](../proposals/spec-batch-1/README.md): C12 at line 315 and order 050 at line 620).
The plan does not list this order's inputs yet; the admin amends it. This order states them in
full. Where the plan differs, follow this order. The order is about 1.2 to 1.6k lines with the
tests and the two READMEs. About half of that is the second package's copy, since each package
carries its own source, tests and README.

**This order's branch is kept, not merged on its own.** Order 050 builds on it. It adds `account`,
`apiKey`, `signIn`, the hooks and each Agent's own home. Version 0.2.0 then merges once, complete,
with 050. The reason is NAM-024 (`spec/promises/naming.yaml:414-440`): once a release publishes
`codex.v2` and `claude-code.v2`, a later release at the same address cannot add an input with a
default, or change anything the Types hold. Anything else that the admin's trials show `.v2` needs
also goes onto this branch before 050.

The decisions. Apply them, and do not reopen them:

1. **Agents run code** (ruling 70, point 1). In `.v2` the input `ownTools` defaults to true. The
   client's shell, code execution, file edits and network are then on.
2. **Nothing confines them in v0** (point 2). The client's own sandbox and approval prompts are
   off. The tools run as the engine's operating-system user. The Type's contract and the README say
   so plainly.
3. **One input turns them off** (point 3). `ownTools: false` gives exactly `.v1`'s tools and
   sandbox.
   - The switch is temporary ([ruling 74](../proposals/ruling-74/README.md), point 3). A more
     durable way to state and bound this reach is to be put to the owner.
   - Order 075 (S70), which the admin is preparing, may let a Type declare the switch, as a member
     that names `/ownTools`. Once 075 merges, such a member may be added on this branch before 050.
   - Do not add one. `type-spec.v1` rejects every member it does not list
     (`spec/schemas/contracts.yaml` line 182, `additionalProperties: false`), so the engine would
     refuse to install the package.
4. **Hosted search is an account service** (ruling 48, point 1). Codex's `web_search` and Claude
   Code's `WebSearch` are on only when `accountServices` and `ownTools` are both true.
5. **The turn deadline is an input**, `turnDeadlineSeconds`.
   - Its default is 180, the same as `.v1`'s fixed deadline, so nothing moves S1 (ruling 71,
     point 4).
   - Its maximum is 604800 seconds, 7 days. The deadline in milliseconds then stays below
     2^31−1, the largest delay that Node.js timers accept. Claude Code runs on a JavaScript
     runtime, and the admin has not checked how it treats a larger delay.
   - NAM-024 fixes the maximum once 0.2.0 is published, so it is set now.
6. **`effort` is an input.** Its default, the empty string, leaves the client's own default.
7. **The client's MCP tool timeout equals the turn deadline.** Then the client does not cut off a
   long call to the engine, such as a call to another Agent's turn.
8. **Project instruction files stay off** in `.v2` too: `project_doc_max_bytes=0` and
   `--setting-sources ""` stay. Work orders tell a worker to read AGENTS.md.
9. **A failed turn names its cause** (ruling 52), in both versions. In both versions an `error`
   event no longer ends a turn (step 7).
10. **`.v1` stays byte for byte**: the `.v1` Types, the `wrapper` item and `.v1`'s launch
    arguments. The shell's tests and the S1 example use `.v1`, and the example pins version 0.1.1.

Decisions 4 to 8 are the admin's, not the owner's. The admin adds them to the list for the
owner's review, `docs/proposals/first-build/to-review-with-the-owner.md`, where the owner may
strike them. Do not edit that file.

What the admin checked against Codex 0.167.0 and Claude Code 2.1.318, without signing in:

- `codex exec --sandbox danger-full-access` gives approval `never`, no file sandbox and network
  access. Leaving `shell_tool` on adds the tools `exec_command` and `write_stdin`.
- `codex exec resume` rejects `-C`. A resume without `--sandbox` and `--cd` before `resume` falls
  back to read-only, in the process's directory.
- Claude Code in the permission mode `bypassPermissions`, with
  `--tools "Bash,Read,Edit,Write,Glob,Grep,WebFetch,WebSearch"`, offers exactly those eight tools.
  `--tools default`, or no `--tools`, offers 26, among them SendMessage, Task and CronCreate.
- Claude Code 2.1.318's code, read but not run, aborts an MCP call to an HTTP server after 300
  seconds with no response or progress. A `timeout` in that server's entry raises this limit;
  `MCP_TOOL_TIMEOUT` alone does not. The engine sends no progress during a call.
- Codex 0.167.0's source: `codex exec --json` prints every error notification as
  `{"type":"error","message":...}`. That includes a stream error that Codex retries, which does
  not interrupt the turn; the line does not say which kind it is. A fatal error ends the turn with
  `turn.failed`, which carries the error's message. `codex exec` itself ignores retried errors
  for its exit status.
- Claude Code 2.1.318's code reports a retry as a `system` event of subtype `api_retry`, which
  carries no `is_error`.

Not checked yet: the seven-name list without WebSearch; whether bypassPermissions still prompts
for a write inside `.git`; and whether Claude Code accepts `timeout` in the engine's entry of
`--mcp-config`. The admin's replay checks them.

Read:

- the records of ruling 70 and ruling 74; NAM-024; MAN-017 (`spec/promises/manifest.yaml:363`);
  SCH-009 (`spec/promises/schemas.yaml:121`); PRV-007 (`spec/promises/providers.yaml:197`): "A
  place that holds null gives no variable and no argument";
- in each package: `conduit.yaml`, `contracts.yaml`, `src/lib.rs`, `src/main.rs`,
  `tests/wrapper.rs`, `tests/fixtures/client.rs` and `README.md`;
- in `engine/crates/shell/src/runner.rs`: the run's private working directory (lines 464-465) and
  the discarded stderr (line 384);
- `engine/crates/shell/tests/agents.rs`, which builds these packages and runs their scripted
  client.

Line numbers are from `main` at 1fb3a48. None of the cited files changed since c77968a. They are
`agent-codex`'s unless stated. In `agent-claude-code/src/lib.rs`, the lines after 211 are two
lower until step 8.

## What to do

1. **Version and manifest**, in both packages.
   - Set version 0.2.0 in `Cargo.toml` and `conduit.yaml`. `Cargo.lock` changes only in the
     package's own version.
   - Add `codex.v2`, or `claude-code.v2`, to `exports`.
   - Add an item `wrapper-v2` after `wrapper`. It has the same `implements`, `command`,
     `plain: true` and `tools` as `wrapper`. Its `environment` binds:
     - `AGENT_OWN_TOOLS: /ownTools`;
     - `AGENT_DIRECTORY: /directory`;
     - `AGENT_TURN_DEADLINE_SECONDS: /turnDeadlineSeconds`;
     - `AGENT_EFFORT: /effort`;
     - `AGENT_ACCOUNT_SERVICES: /accountServices`;
     - `AGENT_MODEL: /model`;
     - in `agent-claude-code` only, `AGENT_PEER_MESSAGING: /peerMessaging`.
   - It must be a separate item: a binding of `/ownTools` in `wrapper` would fail MAN-017 for
     every `.v1` template.
   - Change no existing line of `conduit.yaml` but `version`. Only add lines.

2. **The `.v2` Type.** Append it to the end of `contracts.yaml`, as a new YAML document, so that
   the 0.1.1 file stays a byte prefix of the new one.
   - `family: Agent`, `offers: ['#conversation.v1']`, `requires: []`,
     `implementation: {'#turn.v1': '#wrapper-v2'}` and `records: {inputs: false, outputs: false}`.
     Add no other member to `spec` (decision 3).
   - Its schema is an object with `additionalProperties: false`. Every property below is in
     `required`: MAN-017 requires each bound place to be required, and SCH-009 fills the defaults,
     so an author may omit any of them. Quote `'null'` so that YAML reads it as a string.
     - `ownTools`: `{type: boolean, default: true}`;
     - `directory`: `{type: [string, 'null'], default: null}`;
     - `turnDeadlineSeconds`: `{type: integer, minimum: 1, maximum: 604800, default: 180}`;
     - `effort`: `{type: string, default: ''}`;
     - `accountServices`: `{type: boolean, default: false}`;
     - `model`: `{type: string, default: ''}`;
     - in `claude-code.v2` only, `peerMessaging`: `{type: boolean, default: false}`.
   - Its `metadata.description`, folded as `.v1`'s is. For `codex.v2`:

     > Runs an explicit native-client conversation in the directory that `directory` names, or in
     > a private directory of the run's own when it is null. With ownTools true, the default, the
     > client's own tools are on: shell, code execution, file edits and network, with the
     > client's sandbox and approval prompts off. They run as the engine's operating-system user
     > and nothing confines them. They reach every file, process, network address, credential and
     > signed-in account that user can reach: the engine's store, root's client token, this run's
     > token, the user's git and cloud credentials, and the user's own client logins with their
     > connectors and apps. That reach does not pass through Conduit: no Conduit grants, limits or
     > records it (ruling 70). The client's own browser, desktop-control, goal, image-generation,
     > hook and plugin tools stay off, and so do its multi-agent features, although Codex 0.167.0
     > still offers its collaboration tools. Apps follow accountServices, and so does hosted
     > search when ownTools is true. The shell can still reach the same services and the user's
     > browser and desktop, so accountServices false shapes only the client's tool list. With
     > ownTools false the client has codex.v1's tools and sandbox. A turn longer than
     > turnDeadlineSeconds fails and ends the conversation, and the engine's call timeout
     > (`conduit serve --call-timeout-seconds`) must be longer.

     For `claude-code.v2`:

     > Runs an explicit native-client conversation in the directory that `directory` names, or in
     > a private directory of the run's own when it is null. With ownTools true, the default, the
     > client's own tools are on: Bash, Read, Edit, Write, Glob, Grep and WebFetch, which give
     > shell, code execution, file edits and network, with permission prompts off. They run as the
     > engine's operating-system user and nothing confines them. They reach every file, process,
     > network address, credential and signed-in account that user can reach: the engine's store,
     > root's client token, this run's token, the user's git and cloud credentials, and the user's
     > own client logins with their connectors and apps. That reach does not pass through
     > Conduit: no Conduit grants, limits or records it (ruling 70). The client's sub-agent,
     > scheduling, hook and plugin tools stay off. claude.ai connectors follow accountServices,
     > and so does WebSearch when ownTools is true; SendMessage, ListAgents and RemoteTrigger
     > follow peerMessaging. The shell can still reach the same services and the user's browser
     > and desktop, so accountServices false shapes only the client's tool list. With ownTools
     > false the client has claude-code.v1's tools. A turn longer than turnDeadlineSeconds fails
     > and ends the conversation, and the engine's call timeout
     > (`conduit serve --call-timeout-seconds`) must be longer.

3. **Reading the configuration** (`Config::from_env`, `src/lib.rs:38-74`).
   - A launch is `.v2` when `AGENT_OWN_TOOLS` is set. Otherwise it is `.v1`, and reads the same
     variables as today.
   - In a `.v2` launch:
     - `AGENT_OWN_TOOLS` and `AGENT_ACCOUNT_SERVICES` are `true` or `false`;
     - an absent `AGENT_PEER_MESSAGING` means false, because `codex.v2` binds none. Line 70 must
       not fail for it;
     - `AGENT_TURN_DEADLINE_SECONDS` is a whole number from 1 to 604800. It replaces line 72's
       fixed 180 seconds;
     - an absent `AGENT_DIRECTORY` means no directory (PRV-007). When it is set, it must be an
       absolute path to an existing directory;
     - an absent or empty `AGENT_EFFORT` or `AGENT_MODEL` means the client's default. An effort
       holds lowercase ASCII letters only.
   - **An invalid configuration does not stop the wrapper**, in either version. Today `main` exits
     (`src/main.rs:77`, `:94-99`), the runner discards its stderr (`runner.rs:384`), and the engine
     sees only a failed initialization. Instead:
     - the wrapper completes MCP initialization and lists `turn`;
     - every turn fails, naming the input or variable, the reason and, for `directory`, the path:
       for example `invalid configuration: directory: not an existing directory: PATH`;
     - no client starts.
   - `Config::cwd` stays the wrapper's own directory, the run's private directory, where
     `prepare()` writes. Hold the working directory in a field of its own.

4. **Codex arguments** (`arguments`, `src/lib.rs:129-242`). `.v1`'s stay exactly as they are. A
   `.v2` launch gives, in this order:
   - `exec --sandbox S --color never`. S is `danger-full-access` when ownTools is true and
     `read-only` when it is false;
   - `--model M` when model is set;
   - `--cd DIR`, always. DIR is `directory`, or the run's directory when it is null;
   - `resume` on a resumed turn. `--sandbox`, `--model` and `--cd` must come before it;
   - `--ignore-user-config --strict-config --ignore-rules --skip-git-repo-check --json`;
   - `--enable apps` when accountServices is true, else `--disable apps`, as today;
   - `--disable F` for each feature F:
     - ownTools false: the fourteen of lines 204-219, in their order;
     - ownTools true: the same list without `shell_tool`, `unified_exec` and `view_image`. That is
       `browser_use`, `browser_use_external`, `in_app_browser`, `computer_use`, `multi_agent`,
       `multi_agent_v2`, `multi_agent_v2_dynamic_tools`, `hooks`, `plugins`, `goals` and
       `image_generation`;
   - the seven `-c` settings of lines 222-233, unchanged, except `web_search="live"` when
     accountServices and ownTools are both true;
   - `-c mcp_servers.engine.tool_timeout_sec=N`, where N is turnDeadlineSeconds;
   - `-c model_reasoning_effort=E` when effort is set, with E quoted by `json!`, as line 223
     quotes the URL;
   - the thread on a resumed turn, then `-`.

5. **Claude Code arguments, settings, files and environment.** `.v1`'s stay exactly as they are.
   - ownTools false: `.v1`'s arguments exactly, then `--effort E` when effort is set.
   - ownTools true: `.v1`'s arguments in their order, with these changes:
     - `--tools` gets `Bash,Read,Edit,Write,Glob,Grep,WebFetch`, then `,WebSearch` when
       accountServices is true, then `,SendMessage,ListAgents,RemoteTrigger` when peerMessaging
       is true;
     - `--allowedTools`, `--disallowedTools` and every other argument stay as in `.v1`;
     - after the last of them come `--permission-mode bypassPermissions --permission-prompts none`,
       then `--effort E` when effort is set. With `none`, the client denies a prompt instead of
       waiting for an answer the wrapper cannot give;
     - never pass `--tools default`, and never leave `--tools` out.
   - `settings()` (`src/lib.rs:103-113`): `defaultMode` is `bypassPermissions` for `.v2` with
     ownTools true, and `dontAsk` otherwise. Nothing else in it changes.
   - `prepare()` (`src/lib.rs:114-128`) still writes `settings.json` and `mcp.json` into the run's
     directory, never into `directory`, so that a commit of the working tree cannot pick them up.
     In a `.v2` launch, the engine's entry in `mcp.json` also gets `"timeout": N×1000`, the turn
     deadline in milliseconds.
   - In a `.v2` launch, the client's environment also gets `MCP_TOOL_TIMEOUT=N×1000`. Add nothing
     else to the client's environment (lines 23 and 48-61).
   - Compute N×1000 with checked arithmetic, in both places.

6. **The working directory.** `Stream::spawn` (`src/lib.rs:249-269`; `current_dir` at line 257)
   starts the client in `directory` when it is set, and in the run's directory otherwise. `.v1`
   always uses the run's directory.

7. **A failed turn names its cause**, in both versions (`Session::turn`, `src/lib.rs:288-298`;
   `src/main.rs:52-59`).
   - The failing turn's result has `isError` true and the text
     `client turn K failed: CAUSE; session cannot restart implicitly`. K is the turn's number in
     this conversation, counting from 1.
   - A later turn on the failed session answers
     `session failed at turn K: CAUSE; implicit restart refused`, naming the turn that failed.
   - **When a turn ends.** Today an event of type `error` or `turn.failed`, or with `is_error`
     true, ends the turn (lines 358-362). Change this, in both versions:
     - An `error` event does not end a turn, in either client. Codex sends it for a stream error
       that it retries, too, and a turn that lasts hours would otherwise fail at its first
       reconnect. Keep the CLASS of the latest `error` event's `message` in this turn.
     - Codex: the turn succeeds at `turn.completed`, then the client's successful exit, as today
       (lines 371-373 and 376-383). It fails at `turn.failed`, with the CLASS of its
       `error.message`.
     - Claude Code: the turn ends at its `result` event, as today. With `is_error` true it fails,
       with the CLASS of its `result`.
     - At EOF before the turn boundary, or at an error on the client's pipes, wait at most 2
       seconds for the client to exit, without killing it. Make the 2 seconds one named
       constant. The wait counts toward the turn deadline. The pipe errors are a missing stdin
       (lines 309 and 313), writing, flushing or closing the message (lines 310-311 and
       314-315) and reading a line (line 323). An `EPIPE` there means the client has already
       ended. Then CAUSE is the first of these that holds:
       1. `client failure: CLASS`, with the kept CLASS, when an `error` event came in this turn;
       2. `client exit: status N` when the client exited with nonzero code N, or
          `client exit: signal N` when signal N ended it;
       3. `EOF before turn boundary` when it exited with 0, or had not exited within the wait.
     - Today a client that rejects an argument exits at once, and the turn reports only an EOF.
       Codex's `complete` check (line 375) comes before its wait (lines 377-380), and Claude Code
       is never waited for. With this rule the turn names the exit status. The admin's replay
       relies on it.
   - CAUSE is one of these fixed texts. Map each error path in `Session` as listed, and report
     the mapping:
     - `deadline: no turn boundary within N seconds`, followed in `.v2` by
       ` (input turnDeadlineSeconds)`;
     - `cancelled`, after an active cancellation (`src/main.rs:44-51`);
     - `client failure: CLASS`, from Codex's `turn.failed`, from a Claude Code `result` with
       `is_error` true, or from a kept `error` event, as above;
     - `client exit: status N` or `client exit: signal N`: by the rule above, or when Codex exits
       unsuccessfully after `turn.completed` (lines 377-380). `client exit: status unknown` when
       the wait itself fails, at line 378 or in the bounded wait, unless an `error` event came;
     - `EOF before turn boundary`, by the rule above;
     - `no thread`, when Codex reported none (line 381);
     - `invalid client output`: a line that is not JSON (lines 326-327), or a `thread.started`
       event without a thread ID (line 329);
     - `resume changed thread` (lines 330-333);
     - `client did not start` (lines 262-263 and 266).
   - CLASS is found by fixed substrings, matched without regard to case, in one constant table.
     The first class with a match wins, in this order:
     - `usage-limit`: "usage limit", "usage_limit", "quota";
     - `rate-limit`: "rate limit", "rate_limit", "too many requests", "429";
     - `authentication`: "authentication", "unauthorized", "401", "login", "log in", "api key",
       "credential";
     - `context-window`: "context window", "context_window", "context length",
       "context_length", "prompt is too long";
     - `other`, when nothing matches, and when the event holds no text.

     These substrings are a first list, not checked against real failures.
   - No client text reaches the result, an error or stderr (the rule at line 325). Only the fixed
     texts, and the numbers and paths in them, do.
   - An invalid configuration answers every turn as step 3 says, not as a failed session.

8. **The shared source.** Restore `multi_agent_v2` and `multi_agent_v2_dynamic_tools` in
   `agent-claude-code/src/lib.rs`, in the Codex branch of `arguments` (lines 212-213 of
   `agent-codex`'s copy). Restore them in the same place of the expected arguments in
   `agent-claude-code/tests/wrapper.rs` (lines 129-130 of `agent-codex`'s copy). The two `lib.rs`
   then differ only at line 17, and the two `main.rs` stay identical.

9. **Tests**, with the scripted client, in both packages. Use only synthetic tokens, as the
   existing tests do.
   - The scripted client (`tests/fixtures/client.rs`):
     - reads `settings.json` from the `--settings` argument, not from its working directory (line
       87; line 94 in `agent-claude-code`), and returns the `mcp.json` it read beside `settings`;
     - still writes `client-started` in its working directory and still handles `ENGINE`. The
       engine's `agents.rs` relies on both;
     - gains the messages that the tests below need:
       - a failure event whose text holds a sentinel and a substring of one CLASS: Codex's
         `turn.failed`, and Claude Code's `result` with `is_error` true;
       - for Codex, an `error` event and then `turn.completed`;
       - for Codex, an `error` event and then `turn.failed`, whose texts hold substrings of two
         different CLASSes;
       - for Codex, an `error` event, then an exit with status 1 and no turn boundary;
       - an exit with a given nonzero status after the first event, and an exit by a signal the
         client sends itself (`kill -KILL` on its own process ID);
       - a line that is not JSON;
       - for Codex, a turn that reports no thread, a `thread.started` without `thread_id`, and a
         resumed turn that reports another thread.
     - exits with status 2 before it reads its input, as a client that rejects an argument does,
       when its model is `exit-at-start`. It still writes `client-started` first.
   - `config()` in `tests/wrapper.rs` (lines 12-42) builds `Config` field by field. Give it the
     new fields. The `.v1` tests keep their expected arguments unchanged and pass.
   - Add tests that check:
     - the exact `.v2` arguments, written out in the test and not computed by `arguments()`:
       ownTools true and false, accountServices false and true, directory null and set, effort
       empty and `high`, and for `claude-code.v2` peerMessaging false and true. For Codex, check a
       first turn and a resumed turn;
     - for Codex, that `--sandbox` and `--cd` come before `resume`;
     - the client's working directory, as the scripted client records it: `directory` when set,
       the run's directory otherwise. Canonicalize `directory` before you compare it, as
       `config()` canonicalizes the run's directory (line 15). On macOS a temporary path under
       `/var` resolves to `/private/var`;
     - that `settings.json` and `mcp.json` are in the run's directory, and that after a turn
       `directory` holds nothing but the scripted client's own `client-started`;
     - `defaultMode`; the `.v2` `mcp.json` `timeout` and `MCP_TOOL_TIMEOUT`; and `.v1`'s
       `mcp.json` and client environment, unchanged;
     - a 1-second deadline: the turn fails, naming the deadline and the input; the next turn
       names turn 1 and that cause; no second client starts;
     - each CAUSE but `client exit: status unknown`, which no test can provoke; each CLASS; and
       that the sentinel appears nowhere in the result;
     - for Codex, that an `error` event followed by `turn.completed` gives a successful turn;
       that an `error` event followed by `turn.failed` fails with `turn.failed`'s CLASS; and that
       an `error` event followed by an exit fails with the `error` event's CLASS;
     - in both packages, that the model `exit-at-start` gives `client exit: status 2`;
     - over stdio, with the wrapper binary and `--client-program`:
       - a `.v2` environment without `AGENT_PEER_MESSAGING` runs a turn;
       - a relative `AGENT_DIRECTORY`, a missing directory, `AGENT_TURN_DEADLINE_SECONDS=0` and
         `AGENT_TURN_DEADLINE_SECONDS=604801` each initialize, list `turn`, fail two turns in a
         row naming the input, and start no client;
       - after an active cancellation, the next turn names `cancelled`.
   - No test sleeps longer than a few seconds. No test reads or writes outside its temporary
     directories.

10. **The READMEs**, both.
    - Replace "The per-turn deadline is 180 seconds" (line 45; line 44 in `agent-claude-code`)
      with: 180 seconds for `.v1`, and `turnDeadlineSeconds` for `.v2`, at most 604800.
    - Add a `.v2` section: the inputs and their variables; the arguments of steps 4 and 5; the
      working directory; where `settings.json` and `mcp.json` go; the MCP tool timeouts; when a
      turn ends; and the failure causes.
    - Say that `ownTools` is a temporary switch (ruling 74, point 3): a later Type version may
      replace it.
    - Add a paragraph on the reach that nothing confines when ownTools is true. It names:
      - the engine's store and root's client token;
      - `CONDUIT_IDENTITY_DIR`, with this run's token;
      - the owner's client login, which every Agent shares until order 050;
      - pushes through the keychain, whatever HOME is;
      - other git worktrees that share one `.git`;
      - for Codex, its collaboration tools (finding 4-06).
    - Say that the paragraph on mediated access holds only for `.v1` and for ownTools false. In
      `agent-codex/README.md` it is the paragraph that starts "Mediated access (ACC-027)" (line
      83). In `agent-claude-code/README.md` it is the paragraph that ends "remain mediated limits;
      these launch settings do not provide OS isolation" (lines 70-76).
    - Say that `conduit serve --call-timeout-seconds` must be longer than turnDeadlineSeconds.
      With the defaults, 120 and 180, the engine reports a turn longer than 120 seconds before the
      wrapper's deadline.
    - Write no absolute local path.

11. **Check the manifest with the engine.** This checks what the package tests cannot: MAN-017's
    bindings, the schema and SCH-009's defaults.
    - In `engine/`, run `cargo build --release`. Change nothing there.
    - In a new temporary directory T outside the repository, run
      `engine/target/release/conduit init --store T/store`.
    - For each package, write a project as `engine/crates/shell/tests/agents.rs:150-151` writes
      one. Its `conduit.yaml` pins the package's address at version 0.2.0, with the digest that
      `cargo run --release --bin package` printed. Its Agent templates, each with its own name:
      - the `.v2` Type with `config: {}`;
      - the `.v2` Type with `ownTools: false`, `directory` set to T, `turnDeadlineSeconds: 3600`
        and `effort: high`;
      - for `claude-code.v2` only, one with `peerMessaging: true`;
      - the `.v1` Type with `accountServices: false` and `peerMessaging: false`.
    - Run `conduit register --store T/store --package ARCHIVE#SHA256 PROJECT` for each. A project
      of templates alone starts no wrapper and no client. Report each reply's `accepted` and its
      diagnostics.
    - Then, for each package, register a second project whose one template is the `.v2` Type
      with `turnDeadlineSeconds: 604801`. Report its reply and its diagnostics.

## Scope

Change:

- `packages/agent-codex/`;
- `packages/agent-claude-code/`.

Do not change:

- `engine/`, `conformance/`, `spec/`, `docs/` or any other path. `engine/CONFORMANCE.md`'s
  I3-04 stays true on `main` until 050 merges, and order 050 updates it;
- the `.v1` Types, the `wrapper` item, and `.v1`'s launch arguments, settings and `mcp.json`;
- the client environment list (`src/lib.rs:23`), except `MCP_TOOL_TIMEOUT` in a `.v2` launch of
  Claude Code.

Do not add `account`, `apiKey`, `signIn`, the hooks, `retire-hook.v1`, each Agent's own home
(`CODEX_HOME`, `CLAUDE_CONFIG_DIR`), an input for a new conversation, an output schema, or a Type
member that declares the own-tools switch (decision 3). Order 050 and later changes add what is
needed, on this branch.

One engine test fails on this branch, and the admin expects it to.
`live_s1_scripted_agents_call_engine_with_distinct_run_identities`
(`engine/crates/shell/tests/agents.rs:101`) checks, at lines 120-133, that these packages'
`conduit.yaml` and `contracts.yaml` equal the pinned 0.1.1 copies under
`conformance/fixtures/agents/`. It also pins version 0.1.1 at line 150. Order 050 updates the
copies and the test. Do not change either, and do not run the engine's test suite.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_roadmap.py`
- `git diff --check`

From the repository root, these give the result stated:

- `git status --short` lists only paths under the two packages.
- For NAME `agent-codex` and then `agent-claude-code`, the 0.1.1 contracts are a byte prefix of
  the new ones. This prints nothing and exits 0:
  `head -c "$(wc -c < conformance/fixtures/agents/NAME/contracts.yaml)" packages/NAME/contracts.yaml | cmp - conformance/fixtures/agents/NAME/contracts.yaml`
- This prints only the two `version: 0.1.1` lines:
  `git diff -U0 -- packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml | grep '^-' | grep -v '^--- '`
- `diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs` shows line 17
  only, and `diff packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs` prints
  nothing.
- No file under the two packages, tracked or new, holds a home-directory path or a temporary
  path. This prints nothing:
  `grep -rnE '/(Users|home)/|/(private/)?tmp/' packages/agent-codex packages/agent-claude-code --exclude-dir=target --exclude-dir=dist`

In each of `packages/agent-codex/` and `packages/agent-claude-code/`:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release`;
- `cargo run --release --bin package`. Report the digest it prints; it is a host build's.

Step 11's first registrations are each accepted, with no diagnostic. Each registration with
`turnDeadlineSeconds: 604801` is not accepted, and its diagnostic cites INS-014 and names
`turnDeadlineSeconds` and the keyword `maximum`.

Report the totals passed and failed. Tests that bind loopback ports may fail in your sandbox; if
they do, say so, and the admin reruns them. Do not stage anything.
