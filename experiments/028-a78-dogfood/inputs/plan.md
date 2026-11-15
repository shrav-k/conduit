# summary

I changed nothing in the repo. main has moved to 147f058 (ruling 73). The 041b worker is still running: it started at 10:12, its events file was last written at 12:29, and it has no report yet.

**One fault in the earlier draft is fatal, and I confirmed it. "No engine change is needed" was wrong.**
- The engine's MCP server builds its session manager with rmcp 3.9.0's default settings (engine/crates/shell/src/server.rs:750; line 752 in 041b). That default closes a session after 300 seconds without activity (rmcp session/local.rs:1106-1126, 1252).
- A tool call that is still waiting does not count as activity. So every `conduit call` is cancelled about 305 seconds in. The wrapper then kills the client and marks the conversation failed, and the CLI waits forever.
- The critic reproduced it: "request ct CANCELLED after 305.0s" (scratchpad idle-repro/long-run.log).
- The fix is about 5 lines plus a test: proposed order 069 below, in the admin lane. Dispatch it now, beside 050a.

**What an Agent can do after 050a and 069.** A codex.v2 or claude-code.v2 Agent works in the directory its template names. It runs shell commands, builds, tests, edits files and uses the network. Each turn can run as long as its template allows.
- **Default limit:** 180 seconds, the same as .v1, so nothing changes for S1. The dogfood templates set 4 hours for a worker and 1 hour for a verifier.
- **Failures say why:** a failed turn names its cause, such as the deadline, a usage limit, a sign-in failure or the client's exit.
- **Off switch:** one template input, `ownTools: false`, turns its own tools off and gives it exactly .v1's tools.

**Plainly: in v0 such an Agent is you.** It runs as your macOS user, with no sandbox and no prompts. It can:
- read, change or delete anything your account can, including ~/.ssh, cloud credentials, other projects and the dogfood store's root token;
- run your own `codex` and `claude`, with their connectors;
- push to GitHub through the keychain;
- kill processes.

The contract and README will say so. The cheap guards below are speed bumps, not confinement:
- a separate clone that cannot push;
- a check that nothing changed on GitHub after each turn;
- protection on main, which you would turn on.

**Shortest path, in hours of work from about 12:45 on 2026-11-11:**
1. **+0 to +0.5 h:** write and dispatch 069 (shell only) and 050a (packages/ only), both in parallel with 041b. I write the harness (H) outside the repo at the same time.
2. **+2 to +3.5 h:** 069 merges, 050a is checked on its branch and its archives are built, and 041b is expected to merge. Its worker had run 2.3 h at last check; 041a's ran 2.3 h.
3. **+3 to +4 h:** a 1-hour smoke test on main with 069 in it (041b not needed). It checks:
   - the shell and the directory;
   - that the off switch works;
   - that Claude Code runs without prompts;
   - that a 20-minute turn survives;
   - that a forced deadline names itself;
   - that no processes are left behind;
   - that the Agents are not inside the coordinator's sandbox.
4. **About +4.5 h:** I pin the engine to a main that has both 041b and 069. Root sends order 1 (042) through Conduit to worker-1. With the verifier and a likely second round, an order takes 2.5 to 4 h, then review and merge.
5. **About 13 to 17 h of work in all (about 2 working days):** orders 1 to 3 run through Conduit. Orders 4 and 5 need a coordinator Agent appointed by root. That is batch 2, now on a branch (ruling 71), so it has no date yet.

**About speed.** Stage 1 runs one order at a time and restarts the engine for each fresh conversation. It is slower than today's parallel run-order.sh, so it serves your second purpose (refining Conduit) first, and the other lanes keep using run-order.sh meanwhile. The speed-up comes from the first follow-up: a fresh conversation without a restart, which lets two worker slots run at once. The first order has a time limit; if it runs past twice the usual time, the harness finishes it with plain `codex exec` on the same branch.

**One change to the plan.** 050a stays on its branch. It merges together with order 050 as the single 0.2.0 release, because NAM-024 freezes the first published codex.v2. Dogfooding runs on 050a's built archive until then.

# package_changes

**Order 050a, amended for ruling 70.** It touches packages/agent-codex and packages/agent-claude-code only.
- It ships from branch `wo/050a-agents-v2`, with a draft PR for CI (checks.yml builds packages/*). It is not merged.
- Order 050 builds on that branch (`BASE=wo/050a-agents-v2`) after 048b, and 0.2.0 merges once, complete. The reason is NAM-024 (spec/promises/naming.yaml:414-440): once codex.v2 and claude-code.v2 are published, a later release cannot add signIn or any input that has a default.
- When I dispatch 050a, I amend C12 and order 050 in docs/proposals/spec-batch-1/README.md (around lines 315-320 and 620-624) to list the inputs below. That is a docs-only edit by me.

**1. What stays byte for byte.** codex.v1, claude-code.v1, the `wrapper` item and the .v1 launch arguments.
- tests/wrapper.rs:30-41 builds `Config` field by field, so it gains the new fields. The .v1 tests keep their expected arguments unchanged.
- The wrapper treats the presence of AGENT_OWN_TOOLS as a .v2 launch.

**2. The .v2 inputs.** All are required because MAN-017 requires every bound input, SCH-009 fills the defaults, and `additionalProperties` is false. Order 050 adds account, apiKey and signIn on the branch.

| Input | Type and default | Variable | Meaning |
|---|---|---|---|
| `ownTools` | boolean, **true** | AGENT_OWN_TOOLS | The off switch (ruling 70 point 3) |
| `directory` | string or null, null | AGENT_DIRECTORY (absent when null, PRV-007) | The working directory; null keeps the runner's private empty directory (runner.rs:464) |
| `turnDeadlineSeconds` | integer, minimum 1, **default 180** | AGENT_TURN_DEADLINE_SECONDS | Replaces lib.rs:72's fixed 180. The default stays 180 so that nothing moves S1 (ruling 71 point 4); the dogfood templates set 14400 and 3600 |
| `effort` | string, '' | AGENT_EFFORT | '' leaves the client's own default. Otherwise Codex gets `-c model_reasoning_effort="E"` and Claude Code gets `--effort E` (listed in `claude --help`: low, medium, high, xhigh, max) |
| `accountServices` | boolean, false | AGENT_ACCOUNT_SERVICES | As in C12. In .v2 it also turns on hosted search when ownTools is true (section 5) |
| `model` | string, '' | AGENT_MODEL | As in C12 |
| `peerMessaging` | boolean, false | AGENT_PEER_MESSAGING | claude-code.v2 only. A .v2 launch treats an absent variable as false, so lib.rs:70 does not fail for codex.v2 |

- **Directory checks:** if the path is not absolute or is not an existing directory, no client starts, and every turn fails naming the `directory` input and the path.

**3. Manifest.**
- Version 0.2.0 in conduit.yaml and Cargo.toml. Export codex.v2 or claude-code.v2.
- Add a separate item `wrapper-v2`, with the same command, `plain: true` and the same tools as `wrapper`. It binds:
  - AGENT_ACCOUNT_SERVICES to /accountServices
  - AGENT_MODEL to /model
  - AGENT_OWN_TOOLS to /ownTools
  - AGENT_DIRECTORY to /directory
  - AGENT_TURN_DEADLINE_SECONDS to /turnDeadlineSeconds
  - AGENT_EFFORT to /effort
  - for Claude Code only, AGENT_PEER_MESSAGING to /peerMessaging
- It has to be a separate item: binding `wrapper` to /ownTools would fail MAN-017 for every .v1 template.
- Set `implementation: {'#turn.v1': '#wrapper-v2'}`. Records inputs and outputs stay false.

**4. Codex 0.167.0 arguments (checked in the grounding's no-login probes).**

**ownTools true**, in this order:
- `exec --sandbox danger-full-access --color never [--model M] --cd DIR [resume]`. The `--sandbox`, `--model` and `--cd` options must come before `resume`, because `exec resume` rejects `-C`, and a resume without them falls back to the process's directory and read-only.
- `--ignore-user-config --strict-config --ignore-rules --skip-git-repo-check --json`
- `--disable|--enable apps`
- `--disable` for each of: browser_use, browser_use_external, in_app_browser, computer_use, multi_agent, multi_agent_v2, multi_agent_v2_dynamic_tools, hooks, plugins, goals, image_generation. That is lib.rs:204-221 without shell_tool, unified_exec and view_image.
- lib.rs:222-233's `-c` settings, unchanged except that `web_search` is `"live"` when accountServices is true and `"disabled"` otherwise. `project_doc_max_bytes=0` stays.
- New: `-c mcp_servers.engine.tool_timeout_sec=<turnDeadlineSeconds>`. The key is in the 0.167 binary. Without it, a future coordinator Agent's call to a worker's turn would hit Codex's default MCP tool timeout.
- `[-c model_reasoning_effort="E"] [THREAD] -`

What the probes showed:
- danger-full-access gives approval `never`, no filesystem sandbox, and network on.
- `shell_tool` is what adds exec_command and write_stdin; `--disable unified_exec` changes nothing.
- apply_patch is always offered.

**ownTools false:** lib.rs:142-239 unchanged (read-only, all fourteen features off, web search off), plus `--cd`, the tool timeout and effort when set.

**5. Claude Code 2.1.318 arguments.**

**ownTools true:**
- lib.rs:131-180's arguments, but with `--tools "Bash,Read,Edit,Write,Glob,Grep,WebFetch"`.
  - Add `,WebSearch` when accountServices is true. The eight-name list was verified to give exactly those eight tools.
  - Add `,SendMessage,ListAgents,RemoteTrigger` when peerMessaging is true.
- Keep `--setting-sources ""`, `--strict-mcp-config`, `--disable-slash-commands`, `--allowedTools mcp__engine__*`, `--no-session-persistence` and the `--disallowedTools` line.
- Add `--permission-mode bypassPermissions --permission-prompts none [--effort E]`. With `none`, a prompt is denied rather than left waiting on the wrapper.
- settings() (lib.rs:108) sets `defaultMode` to `bypassPermissions`, and `dontAsk` otherwise.
- The client's environment gains `MCP_TOOL_TIMEOUT=<turnDeadlineSeconds×1000>`. That variable is in the binary, described as the default per-call MCP tool timeout.
- Never use `--tools default`, and never leave `--tools` out: either gives 26 tools, including SendMessage, Task and CronCreate.

**ownTools false:** today's arguments, plus `--effort` and MCP_TOOL_TIMEOUT.

**Not verified:**
- the seven-name tool list;
- whether bypassPermissions still prompts for writes inside .git (the smoke test checks this);
- which wins between the flag and the settings file (both are set the same way, so it does not matter).

**6. Working directory and files.**
- `Stream::spawn` uses `current_dir(directory or cwd)` (agent-codex lib.rs:257; agent-claude-code :255).
- prepare() keeps writing settings.json and mcp.json into the runner's run directory, never into `directory`, so a checkpoint `git add -A` cannot pick them up.

**7. Failures say why (ruling 52), in both versions; .v1's Types and arguments are unchanged.**
- main.rs:56-58 says only "client turn failed; session cannot restart implicitly". The new text names one fixed cause:
  - `deadline` (N seconds, input turnDeadlineSeconds);
  - `cancelled`;
  - `client exit` (with its status);
  - a client failure class read from `turn.failed`/`error` or Claude's error `result`: usage-limit, authentication, rate-limit, context-window or other, matched by fixed substrings;
  - `no thread`;
  - `EOF before turn boundary`;
  - `invalid configuration` (naming the input).
- No raw client text is passed out (lib.rs:325).
- A later turn on a failed session says "session failed at turn K: <cause>; implicit restart refused". Configuration errors that today die silently with stderr discarded (runner.rs:384) are reported on every turn.

**8. Other code changes.**
- Restore the two multi_agent_v2 lines in the Claude copy of lib.rs, so the two copies differ only at line 17.
- Tests:
  - exact arguments for .v1 (unchanged) and for .v2 with tools on and off, across directory, effort, accountServices and peerMessaging;
  - `--sandbox` and `--cd` placed before `resume`;
  - the scripted client records its working directory;
  - a 1-second deadline fails and names itself, and the next turn names the failed turn;
  - an invalid directory fails;
  - nothing is written into the directory.
- The worker runs no real model turn. I replay the .v2 arguments against the real binaries with the grounding's no-login probes (scratchpad probe/ and cprobe/).
- **Size:** about 0.5-0.7k lines, including tests and the two READMEs.

**9. Type description for codex.v2** (claude-code.v2 the same, with its own tool names):

> "Runs an explicit native-client conversation in the directory that `directory` names, or in a private empty directory when it is null. With ownTools true, the default, the client's own tools are on: shell, code execution, file edits and network, with the client's sandbox and approval prompts off. They run as the engine's operating-system user and nothing confines them. They reach every file, process, network address, credential and signed-in account that user can reach: the engine's store, root's client token, this run's token, the user's git and cloud credentials, and the user's own client logins with their connectors and apps. That reach does not pass through Conduit: no Conduit grants, limits or records it (ruling 70). The client's own connector, app, browser, desktop-control, sub-agent, hook and plugin tools stay off, and hosted search follows accountServices. The shell can still reach the same services and the user's browser and desktop, so accountServices false shapes only the client's tool list. With ownTools false the client has codex.v1's tools and sandbox. A turn longer than turnDeadlineSeconds fails and ends the conversation, and the engine's call timeout (`conduit serve --call-timeout-seconds`) must be longer."

**10. README.**
- Replace "The per-turn deadline is 180 seconds" with: "180 seconds for .v1; `turnDeadlineSeconds` for .v2".
- Add a .v2 section listing the arguments above.
- Add a paragraph on reach that nothing confines. It names:
  - the store and root's token;
  - CONDUIT_IDENTITY_DIR;
  - the shared client login;
  - pushes through the keychain, whatever HOME is;
  - other worktrees that share one .git;
  - Codex's collaboration tools (finding 4-06).
- Say that ACC-027's paragraph on mediated access holds only for .v1 and for ownTools false.
- Say that serve's `--call-timeout-seconds` must be above turnDeadlineSeconds.
- engine/CONFORMANCE.md I3-04 stays true on main until 050 merges, and 050 updates it.

# dogfood_path

This is stage 1 of A78. It runs on an engine pinned to a main that has both 041b and 069, plus 050a's branch archives. Paths are relative to the repo unless absolute. Codex fills both roles, so results stay comparable with the record. Everything lives in <dogfood>, outside the repo, as run-order.sh does.

**Setup, once (setup.sh, about 50 lines)**

**0. Pins, as separate clones, not worktrees.**
- The engine binary reads spec/core from its own source tree (shell main.rs:129, 165 use `env!("CARGO_MANIFEST_DIR")`). So the engine is a clone at dogfood/engine, checked out at the pinned commit and built with `cargo build --release` in engine/.
- 050a's archives: `cargo run --release --bin package` in both packages, at 050a's checked commit. The .tar.gz and .sha256 files are copied to dogfood/archives.
- The slot is its own clone: `git clone <github url> .wo/slot-1`, then `git -C .wo/slot-1 config remote.origin.pushurl no-push:`. It shares no .git with the eight lane worktrees, and a plain push from it fails.
- A Codex build can be started from either clone.

**1. Store.** `conduit init --store dogfood/store --url http://127.0.0.1:3917/mcp`, on its own port.

**2. Serve.**
- `nohup caffeinate -i conduit serve --store S --call-timeout-seconds 15000 &`.
- Pass the flag at every start: it is not stored (lib.rs:181-195), and `call` and `init` ignore it (main.rs:85-92).
- Start serve outside any sandbox, the same way run-order.sh is run today. A sandbox profile is inherited by child processes and would confine every Agent. The smoke test checks this.

**3. Install.** The filesystem fixture (conformance/fixtures/s1/archives/filesystem.tar.gz at S1's pin) and agent-codex 0.2.0. Never install 050a's 0.2.0 and 050's 0.2.0 into one store: same version, different digests. Re-initialize after 050.

**4. Project files (dogfood/project, three YAML files copied from engine/examples/s1 at the pin).**
- The manifest pins filesystem 0.1.0, and agent-codex 0.2.0 at its host digest.
- Templates:
  - a Pool on core#pool.v1;
  - a Resource template `tree` (local-directory.v1, input `root`);
  - `codex-turn` (conversation.v1 / turn.v1);
  - `codex-worker`: codex.v2 with ownTools true, accountServices false, turnDeadlineSeconds 14400, effort high, and `directory` taken from an input;
  - `codex-verifier`: the same with turnDeadlineSeconds 3600.
- Instances:
  - Fleet `conduit-core` (core#fleet);
  - Pool `trees`;
  - Resource `tree-1` with root = slot-1, which is B12's tree slot;
  - Agents `worker-1` and `verifier-1`, owner conduit-core, directory = slot-1;
  - Conduits `turn-worker-1` and `turn-verifier-1`, consumer Interface root;
  - S1's explicit root Conduit to the Pool, if the example at the pin keeps it.
- **No tree Conduits in stage 1.** In v0 they would grant nothing the Agents' own shell lacks, and before S70 inspection would show verifier-1 as read-only, which is false. Per-order grants come with orders 4 and 5.

**5. Submit.** `conduit submit --store S /abs/project` (author.rs:276-347) does three things in order:
1. It registers. Root's default Conduit to conduit-core is made and opened during registration (FLEET-005).
2. It opens root's Conduits.
3. It sets up Pool, Fleet, Resource and Agents. With 041b, Agent setup goes through the Fleet's default Conduit (041b diffs to author.rs:124 and main.rs:364).

The Fleet's Conduit gives no right to use an Agent (capabilities.yaml:214-219), so the turn Conduits are needed. Setup starts only the wrappers; no model work runs.

**Sign-in of each Agent**
- **Stage 1: none.** The runner passes the engine's HOME (runner.rs:425-432), so both Agents use the owner's existing Codex login, as run-order.sh's workers already do. API keys are stripped (lib.rs:255-256). Nobody reads or copies an auth file or runs a login.
- **After order 050:** re-initialize the store. The owner signs each slot Agent in once by Codex's device flow (IN_FORCE.md:190). Order 050 moves CODEX_HOME and CLAUDE_CONFIG_DIR, not HOME, so that cargo, rustup, Python packages and git config stay reachable. Moving HOME would not stop keychain pushes anyway.

**Smoke test (about 1 h, throwaway store, engine from main with 069; 041b not needed)**
1. worker-1 runs `pwd`, `cargo --version`, `python3 -c 'import yaml, jsonschema'` and `git status`. It writes under `$HOME/.cache` and fetches `https://crates.io` over the network, which shows it is not confined by an inherited sandbox.
2. A codex.v2 Agent with ownTools false cannot run a command.
3. A claude-code.v2 Agent runs `pwd` in the slot without a prompt, and tries a write inside `.git`: is it allowed or denied?
4. A `sleep 1200` turn returns. That shows the session idle limit is fixed (069), and that the call holds past both the 120 s default and the coordinator's 10-minute foreground limit.
5. An Agent with turnDeadlineSeconds 60 fails, naming its deadline. Its next turn names the failed turn.
6. A turn that runs `nohup sleep 900 &` returns promptly. List what it left with `lsof -d cwd | grep slot-1`; Codex's PTY children have their own sessions, which the run's process-group kill misses (runner.rs:104-108).

**Per order: dogfood/conduit-order.sh NNN-name [net]**

It takes run-order.sh's arguments and prints the same "worker exit=… seconds=…" lines. It is run-order.sh with worker() and verifier() (run-order.sh:28-40) replaced by turn(). A lock directory allows one order at a time, which is the Fleet's one queue. Run the script in the background (run_in_background or nohup), never in the foreground.

**a. Slot.**
- Require `git -C slot-1 status --porcelain` to be empty.
- `git -C slot-1 fetch -q origin && git -C slot-1 switch -q -C wo/NAME $BASE && git -C slot-1 clean -fdq`. Without `-x`, target/ survives between orders.
- Save `git -C slot-1 ls-remote origin` as the before-state.
- Order files come from origin/main as at run-order.sh:18, into $R/work-orders.

**b. Prompt.** run-order.sh:26, plus a paragraph telling the worker:
- its final message is one JSON object valid against `$O/<schema>.schema.json` (absolute path), with no prose and no code fence;
- export `CARGO_BUILD_JOBS=$JOBS RUST_TEST_THREADS=$TT` in its build commands, because the wrapper's fixed environment drops them (lib.rs:23, 48-61);
- never commit, push, switch branches, stop processes it did not start, or read, print or copy a credential.

**c. How root gives the worker its order.**
1. Restart serve:
   - SIGTERM, then wait for it to exit;
   - while it is stopped, `conduit records --store S > $R/records-N.jsonl`;
   - start it again as in step 2;
   - poll `conduit tools --store S` until it answers.
2. Serve's startup sets the ready Agents up again (main.rs:343-357), so the wrapper starts a fresh conversation.
3. Call: `conduit call --store S turn-worker-1.turn.v1 "$(jq -Rs '{message:.}' < prompt)" > $R/worker-call.json &`. This is root acting: the CLI reads root's token itself and never prints it (server.rs:809-812).
4. A bash watchdog kills the call after turnDeadlineSeconds + 600, because the CLI has no limit (server.rs:849-859) and macOS has no `timeout` command. If it fires, restart serve and count it as friction.
5. The engine sends the turn over stdio to the wrapper, which runs `codex exec` in slot-1.

**d. How root gets the report.**
- Take `.structuredContent.text`, or `.content[0].text` on isError, and strip any code fence.
- Validate it with `/usr/bin/python3` and jsonschema against `$O/${SCHEMA:-$NAME}.schema.json`. Write it to `$R/worker-report.json`, so that run-order.sh's round logic reads it unchanged.
- If it is invalid: one repair turn in the same conversation, with no restart, giving the validator's errors. If it is still invalid, stop for the admin.
- If the call fails: print the named cause (050a) and stop, as run-order.sh:44 does.

**e. Long turns.**
- Worker 14400 s, verifier 3600 s, serve 15000 s: the wrapper's explicit failure arrives before the engine reports the call missing (runner.rs:1022-1048).
- With 069, the engine's session idle limit is the call timeout + 60 s. The 15-second SSE pings (server.rs:752) keep the HTTP stream open.
- A dropped `conduit call` cancels the turn and ends the conversation (server.rs:237-258, 446-466; wrapper main.rs:44-49). There is no way to fetch the result again, hence the background run.

**f. Guards and checkpoint.**
- After the turn, HEAD and the branch must be unchanged, and `ls-remote origin` must match the before-state. Otherwise stop and count a guard trip.
- Then the checkpoint commit (run-order.sh:45).

**g. The verifier.**
- Restart serve, then call `turn-verifier-1` with run-order.sh:35's prompt, plus "Change nothing in this tree."
- Afterwards, require `git status --porcelain` to be empty, HEAD unchanged and ls-remote unchanged. Report a changed Cargo.lock apart from other changes.
- Validate against verify.schema.json and write `$R/verify-report.json`.

**h. Rounds.** run-order.sh:49-76 unchanged. Every worker and verifier turn starts with a restart; a repair turn never does.

**i. After a failed turn.** List and kill leftover processes whose working directory is in slot-1, and report them.

**j. Fallbacks.**
- `VIA=exec` runs run-order.sh's own worker() and verifier() with `-C slot-1` on the same branch.
- `START=verify` skips to the verifier when the worker's checkpoint exists.
- Use them when the Conduit path breaks, or when an order exceeds its time limit.

**Branches and merging (outside Conduit, as today)**
- The admin brings the branch over with `git -C conduit fetch .wo/slot-1 wo/NAME:wo/NAME`.
- Then as today: rebase in `git worktree add .wo/NUM wo/NAME`, checks, diff review, private-path scan, push, PR and squash-merge with gh, and the record's row.
- The slot then moves on to the next order.

**Records**
- **Engine:** each request gets an access decision, a dispatch and an ending. Messages and replies are not recorded (contracts.yaml:25-27). Records are exported at each restart.
- **The harness keeps:** prompts, call JSON, reports, timings, restart seconds, repair turns and guard trips.
- **I pre-register this friction in docs/proposals/first-build/log-increment-4.md:**
  - turn length and the session idle limit;
  - restarts;
  - results tied to the connection that asked;
  - the CLI hanging;
  - no structured result;
  - no read-only verifier;
  - no trail of client events;
  - records off, against WORK_ORDERS.md:156;
  - failure causes;
  - the fixed environment list;
  - argv-only arguments;
  - store churn;
  - leftover processes.
- **Measures per order:**
  - worker and verifier minutes;
  - rounds;
  - must, should and nit counts;
  - repair turns;
  - restarts;
  - guard trips;
  - watchdog fires;
  - admin interventions;
  - whether tests that bind loopback ports now run (blocked in 026, 029, 061, 063).

**Gaps (file:line, and what closes each)**

Closed by 050a:
- **G1. No code execution:** packages/agent-codex/src/lib.rs:142-145, 204-221.
- **G2. No directory:** runner.rs:464-465; lib.rs:65, 257; contracts.yaml:12-20; conduit.yaml:12-15.
- **G3. A fixed 180 s deadline that ends the session:** lib.rs:72, 288-298. Closed by the input; the default stays 180.
- **G13. No effort input:** lib.rs:146-148.
- **G15. A coordinator Agent's MCP tool timeout is unset:** lib.rs:222-233 and the Claude environment. 050a sets `tool_timeout_sec` and `MCP_TOOL_TIMEOUT`.
- **G17. The failure text has no cause:** wrapper main.rs:56-58.
- **G18. Startup errors are invisible:** runner.rs:384.

Closed by 069 (an engine order):
- **G5 (fatal). The session idle limit of 300 s cancels every call at about 305 s:** shell server.rs:750 (752 in 041b); rmcp session/local.rs:1106-1126, 1252.

Handled in stage 1, logged as findings:
- **G4. Dispatch timeout:** 120 s, for the whole store, not stored, ignored on call and init, with no cancel sent at the deadline: shell lib.rs:40; main.rs:85-92; runner.rs:1022-1048; help.rs:84. Passed as a serve flag at every start.
- **G6. A dropped call cancels the turn:** server.rs:237-258, 446-466. Background run.
- **G7. The CLI waits forever when the session is gone:** server.rs:849-859. Watchdog.
- **G8. The result is only the last agent message:** lib.rs:363-365. Prompt, validation and a repair turn.
- **G9. No event trail, and records need serve stopped:** lib.rs:261, 325-327. Export at restarts.
- **G10. A fresh conversation needs a restart:** lib.rs:289; runner.rs:258-266; main.rs:343-357. A restart per turn; first follow-up.
- **G11. AGENTS.md suppressed:** lib.rs:186-189, 232. The orders tell the worker to read it.
- **G12. Fixed client environment:** lib.rs:23, 48-61. The prompt tells the worker to export the variables.
- **G14. No read-only verifier:** ruling 70 point 2. Instruction plus the post-check.

Handled by setup or guards:
- **G19. The engine is tied to its source tree:** main.rs:129, 165. A pinned clone.
- **G20. Reach that nothing confines:** root's token in the store, keychain pushes, shared .git, other projects. A separate clone, pushurl, the ls-remote guard and owner protection of main; accepted for v0.
- **G21. Serve started inside a sandbox would confine the Agents.** Start it outside; the smoke test checks.
- **G22. Leftover processes in other sessions:** runner.rs:104-108. Smoke test and lsof.

Elsewhere:
- **G16. Inspection does not show reach through the client's own tools:** ruling 70 point 2. Spec order S70.
- **G23. CLI session start, 10 s × 3 attempts:** server.rs:814-847, finding 4-15. Minor.

# orders

**What can start today, in parallel with 041b:** 069, 050a, H, the docs amendments and S70. 069 touches engine/crates/shell. 041b changes server.rs by 3 lines only (its line 752), so either can merge first and the other rebases trivially. 050a and H touch nothing 041b touches.

**1. 069 (admin lane, engine/crates/shell): the MCP session idle limit**
- **Change:** in serve() (server.rs:750), build `let mut sessions = LocalSessionManager::default(); sessions.session_config.keep_alive = Some(shell.config.call_timeout + Duration::from_secs(60));` and use `Arc::new(sessions)` in place of `Arc::default()`.
- **Test:** a unit test that keep_alive is longer than call_timeout, at the default and at a large value.
- **CONFORMANCE.md:** a DL10 row for the session idle limit.
- **Size:** about 0.05k lines. Worker about 30-45 min.
- **Depends on:** nothing. Dispatch now with run-order.sh, and merge as merge authority.
- **Why it is allowed now:** it is a fault, not a feature, and moves no S1 count.

**2. 050a (packages lane): the .v2 Agent packages**
- **Content:** as in package_changes.
- **Size:** about 0.5-0.7k lines. Worker about 1-1.5 h, verifier about 10 min, a likely second round.
- **Depends on:** nothing; 044 and 045 are merged.
- **Dispatch:** now, with `run-order.sh 050a-agents-v2 net`.
- **Not merged:** it is the stacked base of 050, with a draft PR for CI.
- **Acceptance:**
  - the package tests pass;
  - I replay the .v2 arguments against the real binaries with the no-login probes;
  - the worker runs no real model turn.
- **Later changes:** whatever stage 1 shows .v2 needs, such as a newConversation member or an output schema, goes onto this branch before 0.2.0 is final.

**3. Docs only (me, admin):**
- amend C12 and order 050 in docs/proposals/spec-batch-1/README.md to list the .v2 inputs;
- pre-register the friction and measures in docs/proposals/first-build/log-increment-4.md;
- log my decisions in to-review-with-the-owner.md.

**4. H: the dogfood harness (me, outside the repo in .wo/dogfood)**
- **Parts:**
  - setup.sh, about 50 lines;
  - project/, three YAML files of about 150 lines, copied from engine/examples/s1 at the pin;
  - conduit-order.sh, about 200 lines: run-order.sh with turn(), restart, watchdog, validate and repair, guards, records export, and the `VIA=exec` and `START=verify` fallbacks.
- **Time:** about 1.5-2 h, starting now.
- **Smoke test:** about 1 h, needs 069 merged and 050a's archive, not 041b.
- **First order:** needs the pin with 041b.

**5. S70 (admin lane, spec/): ruling 70 in the specification**
- **Content:**
  - how a Type states reach through the client's own tools, and how inspect.v1 shows it. The smallest form is a Type member naming the boolean pointer (/ownTools), as signIn names /account (CTR-013);
  - the I3-04 wording, applied in 050.
- **Size:** about 0.2-0.4k lines.
- **When:** in parallel from today. It must merge before 050 publishes 0.2.0, since NAM-024 forbids adding the member later.
- **Not on A78's path.**

**6. 050, as planned, after 048b**
- **Base:** 050a's branch (`BASE=wo/050a-agents-v2`).
- **Adds:**
  - account, apiKey, signIn, the three hooks and retire-hook.v1;
  - per-Agent homes through CODEX_HOME and CLAUDE_CONFIG_DIR;
  - CONFORMANCE.md I3-04, the READMEs and the accounts guide.
- **Then:** 0.2.0 merges complete, the dogfood store is re-initialized, and the slot Agents are signed in.

**Follow-ups after orders 1 to 3, chosen from the friction log (each small)**
- **(i) A fresh conversation without a restart.** An optional `newConversation` on turn.v1, with no default; NAM-024 allows that. Packages only, about 0.1k lines. It goes onto 050a's branch. It removes restarts and allows two slot pairs at once, which is where the speed-up comes from.
- **(ii) Cancel the provider call when the dispatch deadline fires, naming the deadline** (runner.rs:1022-1048). Admin lane, about 0.1-0.2k lines. It is a fault fix. Do it before the S1 rerun, because today a turn reported unknown keeps running.
- **(iii) Fetch a kept result again by idempotency key** (server.rs:403-412, 851). About 0.2k lines. It changes the CLI, so ruling 71 point 4 keeps it until after the S1 rerun.
- **(iv) An optional output schema on turn,** if the repair-turn count justifies it.

**A78's own orders**
- **Orders 1 to 3:** the next orders on the critical path after 041b: 042 first (already written), then 046 and 047, one at a time on slot-1. The first is time-limited to twice its run-order.sh estimate, then continues with `VIA=exec`.
- **Other lanes:** keep using run-order.sh in parallel.
- **Orders 4 and 5:** on the batch-2 branch's engine, once appointment exists. They rely on 050a's MCP tool timeouts and on 069.

# later

There is no confinement order now. In v0, ruling 70 offers only tools on or off.

**The route later.** A runtime provider that confines its consumer, chosen through `runtime` (ACC-027, PRV-006). An example is a container, or a macOS sandbox profile, that mounts only the Agent's directory and home.

**Its first requirements, taken from stage 1:**
- a read-only verifier (G14);
- the store and root's token out of reach (G20);
- no keychain or ssh push;
- a network policy;
- killing every process a turn starts, including those in other sessions (G22).

**Notes for that work:**
- Claude Code refuses bypassPermissions as root unless IS_SANDBOX=1, so a root container must set it.
- Codex's own workspace-write mode could give weak, write-only confinement.
- A dedicated macOS user for the engine is the cheapest strong step.
- Per-Agent homes (050) are not confinement.

# owner_rulings

Nothing blocks A78. Three things are yours:

1. **Protect main on GitHub before order 1.** This is your action; I can't do it. I checked: main has no branch protection and no rulesets. In v0 an Agent pushes as you through the keychain.
   - **Recommendation:** require a pull request, block force-pushes and deletion, and include administrators. `gh pr merge --squash` keeps working.
2. **Optional: run the dogfood engine as a dedicated macOS user.** That takes your home, keychain, gh and cloud credentials out of the Agents' reach in one step. It is still within ruling 70's "the engine's user". It costs about an hour, plus a Codex login for that user and `sudo -u` for `conduit call`.
   - **Recommendation:** not now. Revisit if a guard trips.
3. **Before the S1 rerun (053), not before A78.** The rerun uses .v2, where the Agent's own tools are on by default. S1's checks that access goes through Conduit therefore need `ownTools: false` in S1's Agent templates, which is one line the author writes.
   - **Recommendation:** write it and count it under ruling 63. S1 tests the communication framework, which is what ruling 70 point 3's switch is for.

**My decisions, logged in to-review-with-the-owner.md for you to strike (none blocks):**
- 050a stays unmerged until 050, because of NAM-024.
- Hosted search (Codex web_search, Claude WebSearch) is treated as an account service: on only when accountServices and ownTools are both true. This is my reading of ruling 48 point 1.
- turnDeadlineSeconds defaults to 180, as .v1 does; the dogfood templates set their own values.
- An `effort` input, defaulting to the client's own setting.
- The MCP tool timeouts are set to the turn deadline.
- Project instruction files stay off; the orders tell workers to read AGENTS.md.
- One order at a time in stage 1, with a serve restart for each fresh conversation.
- The verifier is permissive: read-only by instruction, with a check afterwards.
- No tree Conduits in stage 1.
- Each slot is a separate clone that cannot push.
- Order 069 is an engine fault fix I dispatch as merge authority.