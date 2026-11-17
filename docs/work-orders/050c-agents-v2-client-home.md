# Work order 050c (P2c): each `.v2` Agent's own client home inside its state directory

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Never print a whole large file: use
`head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`,
never print an environment variable's value, and never read or print a key, a token or any
credential. Never run the real `codex` or `claude` programs, not even for their help or version,
and sign in nowhere. The tests use each package's scripted client, in temporary directories only:
no test reads or writes the home directory of the user who runs it.

## What and why

**Where this order starts.** Your tree is the branch that the coordinator names as `BASE`, not
`main`. Before this order is dispatched the coordinator cuts `wo/050-agents-0.2.0` from `main`, by
rebasing order 050a's, 076's and 078's checkpoints onto it and cherry-picking the Record of 050a,
as [the split of order 050](#who-decided-what) has it, and sets `BASE` to that branch's tip.
`BASE`'s tip holds this order file, committed: the coordinator commits it on
`wo/050-agents-0.2.0`, or rebases the branch onto `main` after the order merges. Before
you change anything, check from the repository root that you find there what that cut gives, and
stop, change nothing and report if any check prints otherwise:

```bash
git merge-base --is-ancestor cbb49fc1 "$BASE"; echo "main $?"
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/050c-agents-v2-client-home.md >/dev/null; echo "order $?"
git status --short | wc -l
grep -c '^## Record' docs/work-orders/050a-agents-v2-own-tools.md
grep -c '^version: 0.2.0$' packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml
grep -c '^  wrapper-v2-path:$' packages/agent-codex/conduit.yaml
grep -c 'ownTools' packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml
grep -c 'CONDUIT_STATE_DIR\|CODEX_HOME\|CLAUDE_CONFIG_DIR\|cli_auth_credentials_store' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs
git diff --quiet 559ecd0b "$BASE" -- packages/agent-codex packages/agent-claude-code; echo "packages $?"
```

They print `main 0`, `head 0`, `order 0` and `0`; then `1` (050a's Record, cherry-picked); `1` for each
`conduit.yaml` (050a's version); `1` (078's item); `0` for each `contracts.yaml` (076 removed the
switch); `0` for each `lib.rs` (no home yet); `2` (the two sources differ in line 17 alone, the
`CLAUDE` constant); `10` for each `wrapper.rs`; and `packages 0`. The rebase changed no package
file: no commit on `main` since `50e7cc5` touches the two packages. So the line numbers below, those
of the stack's tip `559ecd0b`, hold in your tree. The last check, `packages 0`, is what checks that
claim.

**What it builds.** The `.v2` wrappers launch their client with the engine user's environment,
so a `.v2` Agent today uses the client home that `HOME` gives: the owner's everyday login, history
and settings, shared by every Agent. A retire or sign-out hook that order 050 adds later would
then sign the owner out (FR-422's hazard), and a `.v2` Agent with its own tools that runs
`codex login` or `codex logout` in its shell reaches the same home. This order gives each `.v2`
Agent its own client home, inside the state directory that the engine's own runner gives each
instance (PRV-029, `spec/promises/providers.yaml:966-985`), and refuses to launch the client
where that home cannot be shown to lie inside it or, for Codex, where it holds a `.env`. One
behavior, in both packages: every `.v2` launch runs the client with `CLAUDE_CONFIG_DIR` (Claude
Code) or `CODEX_HOME` (Codex) set to that home, and Codex keeps its login in a file there. The
`.v1` launch does not change.

PRV-029 says (`spec/promises/providers.yaml:981-983`): "A Type whose implementation needs one
states that requirement; setup under a runner that gives none fails with an explicit diagnostic."
After this order the `.v2` implementation needs one. The coordinator's reading: the README
section that decision 8 adds states the requirement as the package's contract prose
([definitions](../../spec/definitions.md), "a type's contract"), so no `contracts.yaml` needs to
state it. The one departure from PRV-029's observable is that the explicit diagnostic comes at
each turn rather than at setup. That stays open for the coordinator.

It carries, from the targeted review of the 0.2.0 sign-in surface,
[section 6](../proposals/review-signin-2026-11-16/README.md#6-must-state-lists),
[`claude-code.v2` item 4](../proposals/review-signin-2026-11-16/README.md#050-claude-codev2) and
the launch half of [`codex.v2` item 4](../proposals/review-signin-2026-11-16/README.md#050-codexv2)
(its verdicts M57, M59 to M62 and M88; V02, V15, V26 and V27 in
[its findings](../proposals/review-signin-2026-11-16/findings.md)). Of item 4's done-when it
carries the turn part of the marker test and the `.v1` test; the undo and retirement parts come
with the hooks, in orders 050h, 050i and 050l. Of `codex.v2` item 4 it carries the setting on
every launch, stated in the README; the setting on `codex login` comes with the login, in order
050m, which uses the constant this order adds (decision 4). D53
([the full review's decisions](../proposals/full-review/decisions.md#the-providers-and-the-second-machine))
decided the refusal; [IN_FORCE.md, topic 9](../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach)
keeps the refusal out of the Types' descriptions (82 point 5). The descriptions are unchanged
here because Q1's sentences belong to 050f and later, and the home's location is README matter by
the coordinator's choice, so no `contracts.yaml` changes here.

**What the branch already does, so that you do not redo it.** In each package's `src/lib.rs` (the
two files are one source; line 17's `CLAUDE` selects the client):

- `Config::from_env` (`:42-138`) reads the wrapper's environment once, at start. The variable of
  `turnDeadlineSeconds` selects `.v2` (`:86-87`), and the `.v2` checks (`:95-131`) set
  `configuration_error` on the first invalid input, which then fails every turn, naming it, with no
  client started.
- The client's environment is `c.env`: `HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM` and `USER` where
  the wrapper has them (`BASE`, `:23`), `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR`, and, for a `.v2`
  Claude Code, `MCP_TOOL_TIMEOUT` (`:127-130`). `Stream::spawn` (`:452-472`) clears the rest and
  removes both API keys. `CONDUIT_STATE_DIR`, which the engine's runner gives the wrapper, reaches
  no client.
- A Codex turn launches a new process, a resumed one after the first; a Claude Code conversation
  is one process, launched at its first turn. `run_turn` (`:522-529`) launches through
  `Stream::spawn`, and an error there fails the turn with its text and poisons the conversation.
- Codex's `.v2` arguments end with the seven `-c` settings (`:344-362`) and then, in
  `if c.v2 {` (`:363`), `-c mcp_servers.engine.tool_timeout_sec=N` and the optional effort.
- The scripted client (`tests/fixtures/client.rs`) reports its arguments, environment and working
  directory in its reply, and writes `client-started` in its working directory when it starts.

**Codex's bootstrap in the home.** Order 197 read Codex 0.167.1's source ([its
results](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md), R4, the
paragraph that begins "In this reading, the CLI's bootstrap"). Every `codex` subcommand, `exec`
included, first runs a bootstrap in the home that `CODEX_HOME` names: it reads the home's `.env`, if
there is one, and sets each variable it names in the client's own environment, except names that
begin `CODEX_`; and, for its helper aliases, unless a release build refuses the aliases first (then
it creates nothing, prints a WARNING line on stderr and goes on), it creates the home if missing,
and in it `tmp/arg0-` and the system's name, mode 0700, and in that a temporary directory with a
lock file and links to its executable. Until this order, every Agent's Codex read the `.env` of the
engine user's default home. What this order and its tests expect of that bootstrap:

- The wrapper writes nothing in the home but the directory itself, reads nothing in it, looks up
  no name in it but `.env` (Codex alone, decision 3), and never needs it empty: the client's own
  files, `tmp/` among them, stay, and a later launch in the same
  home goes on as the first did. The scripted client stands in for the bootstrap (decision 7):
  asked, it reads the names in the `.env` of the home it resolves and creates `tmp/arg0-fixture/`
  with a lock file there.
- A `.env` in the engine user's default home reaches no `.v2` launch, and the test shows it with a
  synthetic one. A `.env` in the Agent's own home would be read by Codex itself, past the wrapper's
  list: it can set any variable not beginning `CODEX_`, an API key among them, and one that names
  `OPENAI_API_KEY` would reach Codex past the wrapper's `env_remove` (`:458-459`). Only the Agent's
  own unconfined tools (ruling 70) or a process of the engine's user can write one there, so a key
  that reached Codex that way would be the ruling-70 reach of an Agent's own tools, not a switch by
  Conduit, which 82 point 1 bars ("Nothing switches a Codex Agent to an API key"). The owner's rule
  is that an API key is used only where a template sets `apiKey` explicitly, never as a fallback
  ([IN_FORCE.md, topic 9](../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach),
  82 points 1 and 2). So the coordinator decides that every `.v2` launch of Codex, and every hook
  of `codex.v2` once one exists, refuses a home that holds a `.env` (decision 3), with a
  diagnostic that names the cause and holds no path and no file content. The check runs at every
  launch. It and Codex's read are two steps, so a `.env` written between them is read by that
  launch alone, and the next launch refuses; one written during a turn is read by no launch.
  Claude Code's home gets no such refusal: no source this order can cite shows Claude Code reading
  a `.env` from the directory that `CLAUDE_CONFIG_DIR` names. Order 197 read Codex alone, and no
  record in this repository reads Claude Code's start-up for that.
- The wrapper discards the client's stderr (`:464`), so the WARNING line reaches no result, and
  the wrapper does not look at where the state directory lies. The tests' state directories are
  under the system's temporary directory; whether a release Codex refuses its aliases there is a
  comparison R4 did not settle. The scripted client does not imitate the alias refusal, and no test
  depends on it. Whether the live check of A5 stage 1
  moves its scratch root is the coordinator's, recorded in order 197's Record, not this order's.

## The decisions it applies

1. **The home.** Each `.v2` Agent's client home is the directory `client-home` inside the directory
   that `CONDUIT_STATE_DIR` names, the same name in both packages. Add to `src/lib.rs`, after `BASE`
   (`:23`):

   ```rust
   /// The client's home variable (claude-code.v2 and codex.v2, item 4).
   pub const HOME_VARIABLE: &str = if CLAUDE {
       "CLAUDE_CONFIG_DIR"
   } else {
       "CODEX_HOME"
   };
   /// The home's name inside the instance's state directory.
   pub const HOME_NAME: &str = "client-home";
   /// Codex keeps its login in a file in its home (codex.v2 item 4).
   pub const CREDENTIAL_STORE: &str = "cli_auth_credentials_store=\"file\"";
   ```

2. **`CONDUIT_STATE_DIR` is read from the environment alone, for `.v2` alone.** `Config` gains
   `pub state_dir: Option<PathBuf>`, after `effort`, `None` in `from_env`'s first value. At the end
   of the `if v2 {` block (after the `MCP_TOOL_TIMEOUT` insertion), read the wrapper's own
   `CONDUIT_STATE_DIR` with `std::env::var_os`. Absent or empty, the configuration fails with
   `client home (H): CONDUIT_STATE_DIR is unset`; not an absolute path, with
   `client home (H): CONDUIT_STATE_DIR is not an absolute path`; otherwise `state_dir` holds it as
   given. Here and below, H is `HOME_VARIABLE`'s value, written with `{HOME_VARIABLE}` in the
   format string. No item binds the variable (MAN-018 reserves it) and no input names the home.
   The wrapper reads no `CLAUDE_CONFIG_DIR` or `CODEX_HOME` from its own environment: one there
   reaches no client, as before. A `.v1` configuration never reads `CONDUIT_STATE_DIR`.

3. **Every `.v2` launch resolves the home, and refuses one outside.** Add
   `pub fn client_home(c: &Config) -> Result<PathBuf>`. It does the following, in this order; each
   text below is the function's whole error, and none holds a path:
   - `state_dir` is `None`: `client home (H): CONDUIT_STATE_DIR is unset`.
   - Canonicalize `state_dir`. Where that fails or the result is not a directory:
     `client home (H): CONDUIT_STATE_DIR is not an existing directory`.
   - Let the home be the canonical state directory joined with `HOME_NAME`. Where
     `fs::symlink_metadata` fails with `NotFound`, create it, that directory alone, with mode 0700
     on Unix (`std::os::unix::fs::DirBuilderExt`), and with `fs::create_dir` elsewhere; a failure
     gives `client home (H): cannot create the home in CONDUIT_STATE_DIR`. Any other failure goes on
     to the next step. Never create through a link, never change an existing home's mode, and
     never read, list or change what it holds; the next step's look-up of the one name `.env`
     alone is allowed.
   - Canonicalize the home. Where that fails: `client home (H): the home is not a directory`.
     Where the result is not strictly inside the canonical state directory (`starts_with`, and not
     equal to it): `client home (H): the home resolves outside CONDUIT_STATE_DIR`. Where it is
     inside but not a directory: `client home (H): the home is not a directory`.
   - For Codex alone (`if !CLAUDE`), call `fs::symlink_metadata` on the canonical home joined with
     `.env`, opening nothing. Where it succeeds, whatever the entry is (a file, a directory or a
     link, dangling or not): `client home (H): the home holds a .env, which Codex would read into
     its environment; the wrapper refuses it, since an API key is used only where a template sets
     apiKey`. Where it fails with anything but `NotFound`:
     `client home (H): cannot check the home for a .env`. Neither text holds the entry's content,
     a name it sets or a path. Claude Code's copy runs no such step (What and why).
   - Otherwise return the canonical home.

   In `Stream::spawn`, before the command is built, write
   `let home = if c.v2 { Some(client_home(c)?) } else { None };` and, right after `.envs(&c.env)`,
   the line `.envs(home.as_ref().map(|home| (HOME_VARIABLE, home)))`. Its error is the launch's
   error, unchanged, so the turn fails with a message that holds the text, no client starts, and the
   conversation does not restart implicitly, as for any failed launch. It is checked at every
   launch, so a home that a link replaced, or that gained a `.env`, between two Codex turns is
   refused at the second. The
   function is public: every hook that orders 050h, 050i, 050l and 050m add calls it before it runs
   the client and gives the client the same variable. This order adds no hook.

4. **Codex keeps its login in a file there.** In `arguments`, in Codex's `if c.v2 {` (`:363`),
   first add `-c` and `CREDENTIAL_STORE`, before `mcp_servers.engine.tool_timeout_sec`.
   `--ignore-user-config` skips the home's `config.toml`, so the setting goes on the command line,
   on every `.v2` launch, first and resumed. `.v1`'s arguments do not change. Claude Code's
   arguments do not change. The constant compiles into the Claude Code package as well, unused
   there, so that the two sources stay one; order 050m passes it to `codex login`.

5. **The coordinator's decision on G11** (the review's section 8, point 9): the refusal applies to
   `.v2` launches and, once they exist, hooks only. The `.v1` launch's environment stays equal to
   0.1.1's: `c.env` as `from_env` builds it for `.v1`, with no home variable, whatever the
   wrapper's environment holds, `CONDUIT_STATE_DIR` included. A `.v1` Agent keeps the engine user's
   default home and its shared login, as in 0.1.1 (V26: a `.v1` Type states no `signIn`, so an
   empty home would fail every turn for good). A `.v1` Codex still reads that default home's
   `.env`, as in 0.1.1: decision 3's `.env` refusal is `.v2`'s alone.

6. **What the `.v2` launch gives the client.** The canonical home's path, not the path as
   `CONDUIT_STATE_DIR` writes it: the check and the value agree, and the runner gives the same
   directory on every run (PRV-029). `HOME` stays as the runner passes it (M60). The client's
   environment for `.v2` is `.v1`'s, plus the home variable, plus `MCP_TOOL_TIMEOUT` for Claude
   Code.

7. **The scripted client.** In both packages' `tests/fixtures/client.rs`, make the same change, in
   the branch of `turn` that builds the evidence (not `ENGINE`). Where the message is `HOME`, it
   resolves its home as the client would: the value of `CLAUDE_CONFIG_DIR` for Claude Code (its
   `claude` flag) or `CODEX_HOME` for Codex, else `HOME` joined with `.claude` or `.codex`. Then it
   creates that directory if missing (`create_dir_all`) and writes in it `fixture-home-write`,
   holding the turn's number; for Codex alone, it also creates `tmp/arg0-fixture/` there with an
   empty file `lock`, and reads the home's `.env`, if there is one, keeping of each line that holds
   `=` the text before the first `=`. Its evidence gains `home`, the resolved path as a string, and
   `dotenv`, the names kept (an empty list for Claude Code), both only where the message is `HOME`.
   No other message reads or writes a home, so the engine's test that runs this client, with the
   engine user's `HOME`, touches no real home. The two copies differ after this change only as they
   do at `BASE`.

8. **The READMEs**, both. Write no absolute local path; a client version only where this decision
   names one (the Codex 0.167.1 that order 197 read). Q1 (i)'s rule of no client version governs
   the descriptions in `contracts.yaml`, which this order does not touch. Name no store for Claude
   Code's login (no keychain or file named, M58); the Codex README names the file store as this
   decision says (M51).
   - In the paragraph that begins "The runner supplies a private empty working directory", replace
     "The owner's subscription login is shared; a separate login home is future work." with: "A
     `.v1` client uses the default home of the engine's user, and with it that user's subscription
     login, shared by every `.v1` Agent; each `.v2` Agent has its own home (below).", its last
     word a link to the new section's anchor, `the-v2-agents-own-home`.
   - In the `.v2` section's reach paragraph, replace "and the owner's client login, which every
     Agent shares until order 050 gives each its own home" (wrapped over two lines there) with
     "the default home of the engine's user, with its login, and every other Agent's own home in
     the engine's store".
   - In `agent-codex/README.md`, the paragraph on the `-c` settings: "Then
     `-c mcp_servers.engine.tool_timeout_sec=N`" becomes
     "Then `-c cli_auth_credentials_store="file"`, then
     `-c mcp_servers.engine.tool_timeout_sec=N`", and "Its client environment is unchanged from
     `.v1`." becomes "Its client environment is `.v1`'s, plus `CODEX_HOME`." In
     `agent-claude-code/README.md`, "This is the only client environment addition; `.v1`'s
     environment and mcp.json stay unchanged." becomes "With `CLAUDE_CONFIG_DIR`, these are the only
     client environment additions; `.v1`'s environment and mcp.json stay unchanged."
   - Add, right before `## The wrapper-v2-path item` in `agent-codex/README.md` and before
     `## Turn boundaries and failures (both versions)` in `agent-claude-code/README.md`, a section
     `## The .v2 Agent's own home` that says, in the package's own client's terms: the wrapper reads
     `CONDUIT_STATE_DIR` from its own environment alone, as the engine's runner gives it (PRV-029),
     and no input sets it or the home; the home is `client-home` in that directory, made with mode
     0700 when missing; at every launch the wrapper checks it and sets the home variable to its
     canonical path; each refusal's text (decisions 2 and 3); that a runner that gives no state
     directory gets a `.v2` Agent whose every turn fails naming it; that the `.v1` launch is
     0.1.1's; that the home is removed with the instance on the engine's own runner (PRV-029,
     LIF-035), and under another runtime provider as that provider's contract states; and that the
     login it holds is the client's, kept where the client keeps it for that directory. In
     `agent-codex/README.md` add that every `.v2` launch passes
     `-c cli_auth_credentials_store="file"`, since `--ignore-user-config` skips the home's
     `config.toml`, so that Codex keeps the login in a file in the home and two Agents' logins
     never share a store; and order 197's facts, cited to
     [its results](../../experiments/027-a5-sign-in/results-2026-11-17-account-read-source.md)
     at Codex 0.167.1: Codex reads the home's `.env` into its own environment, except names that
     begin `CODEX_`, past the wrapper's list, so a `.env` there could set any variable not
     beginning `CODEX_`, an API key among them, and only the Agent's own unconfined tools or a
     process of the engine's user can write one (ruling 70); since an API key is used only where a
     template sets `apiKey` explicitly, never as a fallback (ruling 82, points 1 and 2), every `.v2`
     launch, and every hook once one exists, refuses a home that holds a `.env`, with decision 3's
     text, and a `.env` written between the check and Codex's read is read by that launch alone;
     it makes `tmp/`
     in the home for its helper aliases; and a release build may skip those aliases for a home
     under the system's temporary directory, with a warning that the wrapper discards; whether it
     does for a given directory is a comparison order 197 did not settle. In
     `agent-claude-code/README.md` add that the wrapper does not refuse a home that holds a `.env`,
     since no source the package cites shows Claude Code reading one from that directory.
   - In `agent-codex/README.md`, in the paragraph that begins "The paragraph above on what nothing
     confines", replace "with that user's files and whatever client login that user has there."
     (wrapped over two lines there) with "with that user's files. A `.v1` client uses whatever
     client login that user has there; a `.v2` client's home is `client-home` in the state
     directory that provider gives, and a provider that gives none fails every `.v2` turn, naming
     it."

9. **The tests**, in each package's `tests/wrapper.rs`, each name the same in both:
   - `config()` creates `parent/state`, with mode 0700, and sets `state_dir` to its canonical
     path, for every test. Every existing test keeps passing unchanged but for this and the next
     three points.
   - `expected_v2_args` gains, for Codex, `-c` and `CREDENTIAL_STORE` where decision 4 puts them,
     so every exact `.v2` launch, first and resumed, with and without a directory, shows it.
     `expected_args` (`.v1`) does not change.
   - `v2_without_directory_keeps_v1_tools_and_sandbox`, in its `if !CLAUDE` strip list, also
     removes `CREDENTIAL_STORE` and the `-c` before it, as it does for the timeout, and asserts
     that it was present. The Claude Code copy is unchanged in behavior: the strip stays inside
     `if !CLAUDE`.
   - `stdio_v2_defaults_and_invalid_configuration_remain_callable` gives the wrapper
     `CONDUIT_STATE_DIR` (the config's) and a stray `HOME_VARIABLE` naming `parent/elsewhere` in
     every case, and in each valid case asserts that the client's environment holds `HOME_VARIABLE`
     equal to the canonical `parent/state/client-home`. It gains one valid case, `state-link`, whose
     `CONDUIT_STATE_DIR` is `parent/state-link`, a link to `parent/state`, and asserts that the
     client's `HOME_VARIABLE` equals the canonical `parent/state/client-home`. It gains two invalid
     cases, each with the existing assertions (two failed turns naming it, no `client-started`):
     `state-unset`, with no `CONDUIT_STATE_DIR`, expecting
     `client home (H): CONDUIT_STATE_DIR is unset`; and `state-relative`, with `state`, expecting
     `client home (H): CONDUIT_STATE_DIR is not an absolute path`. It gains one more invalid case,
     `state-empty`, with `CONDUIT_STATE_DIR` set to the empty string, expecting
     `client home (H): CONDUIT_STATE_DIR is unset`. That makes 17 cases, and the test count stays
     13.
   - New `v2_client_home_in_state_directory_leaves_default_home_untouched`, the marker test's turn
     part. For a directory null and named: plant the synthetic default home `parent/.codex` or
     `parent/.claude` (the config's `HOME` is `parent`) holding `marker` (`synthetic-marker`) and,
     for Codex, `.env` holding `FIXTURE_DOTENV=synthetic`. For Claude Code the test inserts
     `MCP_TOOL_TIMEOUT` into `c.env`, as `v2_exact_launch_directories_and_private_files` does, and
     then calls `prepare(&c).unwrap()` before the `Session` starts (`prepare` does nothing for
     Codex, so the call can be unconditional). Run two `.v2` turns of `HOME` in one `Session`. At
     each turn assert: the client's environment holds `HOME_VARIABLE` equal to the canonical
     `parent/state/client-home`, and its keys are exactly those of `c.env` minus the two API keys
     plus `HOME_VARIABLE` (with the existing allowance for `__CF_USER_TEXT_ENCODING`), and each of
     `c.env`'s other keys, `HOME` among them, holds `c.env`'s value, as
     `four_reach_combinations_exact_launch_and_same_conversation` checks for `.v1`; the
     evidence's `home` is that path; its `dotenv` is empty; for Codex the arguments hold `-c`
     followed by `CREDENTIAL_STORE`. After the turns assert: the default home's entries are exactly
     those planted and `marker` holds its bytes; the Agent's home holds `fixture-home-write`,
     holding `2`, the last turn's number (for Codex the second turn is a second launch, for Claude
     Code the same one) and, for Codex, `tmp/arg0-fixture/lock`; that the Agent's home's top-level
     entries are exactly `['fixture-home-write']` for Claude Code and
     `['fixture-home-write', 'tmp']` for Codex; and, on Unix, the home's mode is 0700. Then, for
     Codex alone, remove `client-started`, write `.env` holding
     `OPENAI_API_KEY=synthetic-dotenv-value` in the Agent's own home and run a third turn of `HOME`
     in the same `Session`, a resumed launch. Assert that it fails with decision 3's `.env` text,
     that its message holds neither `OPENAI_API_KEY` nor `synthetic-dotenv-value` nor a path of the
     test's directories, that no `client-started` is written, that the `.env` still holds its bytes
     and that the home's top-level entries are the two above and `.env`: the wrapper refuses the
     launch and changes nothing in the home.
   - New `v2_client_home_refusals_start_no_client`. Each case runs a `.v2` turn of `HOME` and
     asserts that it fails with a message holding the text, that the message holds no path of the
     test's directories, that no `client-started` is written and that the planted default home is
     untouched: `state_dir` `None` (unset); `state_dir` a missing path (not an existing directory);
     `client-home` a regular file (not a directory); `client-home` a link to the default home, and
     a link to `parent/elsewhere`, a directory outside the state directory (resolves outside); and
     `client-home` a dangling link (not a directory). For Codex alone, one more: after the first
     turn passes, remove `client-started`, replace `client-home` with the link to the default
     home, and assert that the second turn fails with the outside text, writes no
     `client-started` and leaves the default home untouched. For Codex alone, two more, each with
     a fresh `client-home` made before the turn: holding `.env`, a regular file holding
     `OPENAI_API_KEY=synthetic-dotenv-value`, and holding `.env`, a dangling link; each fails with
     decision 3's `.env` text, its message holds neither `OPENAI_API_KEY` nor
     `synthetic-dotenv-value`, and the `.env` is still there, the file with its bytes. For Claude
     Code alone, the one case here that passes: a fresh `client-home` holding the same `.env` file
     gives a turn that passes, with the evidence's `home` the canonical `parent/state/client-home`,
     and the `.env` still holds its bytes, which pins that Claude Code's home has no `.env`
     refusal.
   - New `v1_launch_environment_equals_0_1_1`. Start the wrapper binary as a `.v1` Agent, as
     `stdio_exposes_only_turn_and_setup_never_launches_client` does, with `HOME` and `PATH` as
     `c.env` has them (`HOME` the temporary parent), `LOGNAME` and `USER` both `synthetic-user`,
     `TERM` `dumb` and `SHELL` `/bin/sh`, so that no literal in the test holds `/home/`, `/Users/`
     or `/tmp/` and the private-path count below stays 0, the two identity variables,
     `CONDUIT_STATE_DIR` naming the config's state directory, a stray `HOME_VARIABLE`, both API
     keys, `AMBIENT_SENTINEL`, and `AGENT_ACCOUNT_SERVICES` and `AGENT_PEER_MESSAGING` set to false,
     as that test does; the last two are not among the client's expected variables. One turn of
     `TEST`. Assert that the client's environment is exactly the six base variables and the two
     identity variables with the values given (with the existing allowance for
     `__CF_USER_TEXT_ENCODING`): the environment 0.1.1's `Config::from_env` and `Stream::spawn`
     give. Assert too that the state directory holds no `client-home` after the turn, and that
     `arguments` for a `.v1` config holds no `CREDENTIAL_STORE`.
   - No test sleeps longer than a few seconds, and none reads or writes outside its temporary
     directories.

10. **What this order does not do.** No hook, no `signIn`, no `account` or `apiKey`, no change to
    `conduit.yaml`, `contracts.yaml`, `src/main.rs`, `Cargo.toml` or `Cargo.lock`, and no change
    under `engine/`, `conformance/`, `spec/` or `docs/`. The descriptions stay as they are (Q1 is
    050f's and later orders'). The engine's test
    `live_s1_scripted_agents_call_engine_with_distinct_run_identities`
    (`engine/crates/shell/tests/agents.rs`) fails on this branch by design until order 050n moves
    `conformance/fixtures/agents/` to 0.2.0, as [order 050a](050a-agents-v2-own-tools.md) records.
    It is the one expected failure on `wo/050-agents-0.2.0`, and it is not this order's: neither
    you nor the verifier counts it against this order. Do not run the engine's tests and do not
    change it.

## Who decided what

The carrier, the items and their done-whens are the review's must-state lists, which the owner
accepted with [ruling 82](../proposals/ruling-82/README.md). The split of order 050 into 050b to
050o, this order's place on `wo/050-agents-0.2.0` and its base are the coordinator's, open to the
owner's strike ([ruling 48](../proposals/ruling-48/README.md)). The decision on G11 (decision 5) is
the coordinator's, which the review left open, open to the owner's strike. So is the refusal of a
Codex home that holds a `.env` (decision 3): a key that a `.env` there set would be the ruling-70
reach of an Agent's own tools, not a switch by Conduit, which 82 point 1 bars, and the coordinator
reads the owner's rule that an API key is used only where a template sets `apiKey` explicitly,
never as a fallback (82 points 1 and 2), as asking the wrapper to refuse it rather than leave it
open; so too is giving Claude Code's home no such refusal, for want of a source. Everything else
here is the coordinator's too, each open to the owner's strike: the name `client-home`, mode 0700,
the canonical path given to the client, the refusal texts and their order, reading the state
directory only for `.v2`, keeping the `codex login` half for 050m, the scripted client's `HOME`
message, the reading of PRV-029's clause on a Type that needs a state
directory, and the README's wording. Apply them as written and report them under `decisions` as the
coordinator's.

## Scope

Change only, in `packages/agent-codex/` and `packages/agent-claude-code/`: `src/lib.rs`,
`tests/wrapper.rs`, `tests/fixtures/client.rs` and `README.md`. Nothing else, in the packages or
outside them. The build's ignored output (`target/`, `dist/`) may change.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

Each prints what is said beside it:

```bash
git status --short | grep -vE '^ M packages/agent-(codex|claude-code)/(src/lib\.rs|tests/wrapper\.rs|tests/fixtures/client\.rs|README\.md)$' | wc -l   # 0
git diff --quiet "$BASE" -- packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml packages/agent-codex/src/main.rs packages/agent-claude-code/src/main.rs; echo $?   # 0
diff packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs | grep -c '^[<>]'   # 2
grep -c 'cli_auth_credentials_store' packages/agent-codex/README.md   # at least 1
grep -c 'CLAUDE_CONFIG_DIR' packages/agent-claude-code/README.md   # at least 1
grep -c 'until order 050' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 0 each
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs   # 13 each
grep -rnE '/(Users|home)/|/(private/)?tmp/' packages/agent-codex packages/agent-claude-code --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

In each of `packages/agent-codex/` and `packages/agent-claude-code/`, one command at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

`cargo test --release` reports, for `tests/wrapper.rs`, 13 passed and 0 failed, and every other
test target as at `BASE`. Report the totals and the digest that `package` prints: it is a host
build's.

Then show that the marker test fails without decision 3's launch change. In `packages/agent-codex/`:

```bash
cp src/lib.rs target/lib.rs.fixed
git show "$BASE":packages/agent-codex/src/lib.rs > target/lib.rs.base
```

Make one edit to `src/lib.rs`: delete the `.envs(home.as_ref()...)` line alone; the chain still
compiles without it, with only an unused-variable warning, which `cargo test` allows. Then run
`cargo test --release --test wrapper v2_client_home_in_state_directory`; it must fail, at the
assertion on the client's home variable or on the default home. Restore with
`cp target/lib.rs.fixed src/lib.rs` (not `mv`, so that cargo rebuilds), rerun the same command,
which passes, and check that `cmp target/lib.rs.fixed src/lib.rs` prints nothing.
`target/lib.rs.base` is for your reading only.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, give the last line of every check and each count above, and
each refusal's text as a test saw it. Tests that bind loopback ports may fail in your sandbox; if
they do, say so, and the admin reruns them.

The admin then stages any new file and runs `python3 tools/check_private_paths.py` and
`git diff --cached --check`. This order is a checkpoint on `wo/050-agents-0.2.0`: it does not merge
alone, and its Record is written at the foot of this file. A fresh Codex verifier and a fresh
read-only Opus reader read the result before the checkpoint stands
([ruling 79](../proposals/ruling-79/README.md), point 2). The local gate that
[ruling 80](../proposals/ruling-80/README.md) has a merge wait on, `tools/gate.py`, runs at order
050n's merge, read with the expected engine failure known.
