# Work order 021 (I3-02): the engine's own runner, and the unchanged filesystem server

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates, to fetch one npm
package named below, and for tests that bind a loopback port; use it for nothing else.

## What and why

The second order of increment 3, item H02 of
[the handoff](../proposals/first-build/increment-3-handoff.md#work-the-shells-orders-can-take).
Order 020 built the shell's store, loop and identities, and left an `Executor` with a scripted
stand-in. This order gives it the engine's own runner (PRV-003): it starts the processes of
runs, speaks MCP to them over stdio, and carries out the intents that reach a server. With it,
S1's setup starts the unchanged reference filesystem server, once for the project directory and
once for the history directory, and requests reach it.

Read: the handoff, its sections on intents and completions above all; order 020 and its record;
the shell as order 020 left it (`reference-model/crates/shell/`); `CONFORMANCE.md`;
`conformance/fixtures/s1/` and its README; and these promises: PRV-003, PRV-007, PRV-009,
PRV-010, PRV-022, PRV-023, PRV-024, MAN-010, IFC-001, IFC-006, IFC-007, IFC-010, IFC-046,
OPS-013, OPS-027, TPL-012. Experiment 009's runner (`experiments/009-native-launch/src/`)
shows a working stdio client with `rmcp` 3.9.0; reuse what fits, in the shell's crate.

## The decisions to apply

Order 020's `Executor` answers each intent with an `Execution`. An intent whose work takes time
answers `Deferred` and returns its completion later through `Shell::complete` or
`Shell::setup_report`, never by holding the shell's lock while it waits. The engine's address
is `Config::engine_url`.

1. **Intents served**: `builtin-runner` (`run.v1` and `inspect-run.v1`), `dispatch`,
   `refresh-tools-and-call`, `cancel` and `cancel-setup-parts`, each exactly as the handoff
   gives its shape and its returning input. `provider-call` and `connect` stay unserved in the
   first build, which has no runtime provider and no remote item; they end as order 020's loop
   ends an unserved intent, and `CONFORMANCE.md` says so.
2. **Items the runner runs.** A `package` item with `registryType` `npm`: the runner fetches
   the pinned version with `npm pack`, checks the file's SHA-256 against the item's
   `fileSha256` before anything else and refuses to run on a mismatch, unpacks it into a cache
   in the store keyed by that hash, installs its dependencies there with `npm install
   --omit=dev --ignore-scripts`, and runs the package's `bin` with `node`. Say in
   `CONFORMANCE.md` that the hash pins the artifact and not its dependency graph (MAN-010). A
   `command` item: its program, as a relative path, is taken from the installed package's
   files, written out under the store; an absolute program is run as given. Any other
   `registryType` makes the run fail, with a diagnostic that names it, as the DL16 choice of
   supported registry types.
3. **The arguments and the environment.** `packageArguments` are bound as MAN-010 gives them,
   positional by `valueHint`; for S1 that binds the filesystem server's directory to the held
   root, and nothing else is passed. The process gets the environment of PRV-023 exactly: the
   six variables where the engine has them and their value does not begin with `()`, the
   variables a list in the shell's configuration names (empty by default), the run's
   `environment` and secrets, and `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR` from order 020's
   identity for that instance, whose values win. A test shows a variable of the engine's own
   environment, such as one named `GITHUB_TOKEN`, does not reach the process.
4. **The process.** Working directory: a private, empty directory per run under the store,
   mode 0700, which is not the shared directory. Each run gets its own process group; the
   runner ends every group when the shell stops, and no run outlives the engine, which is the
   condition under which PRV-022 lets `CONDUIT_URL` be `http` on loopback. Restart policy
   (PRV-010): none in the first build. A run that stops unasked is reported `stopped` by the
   next `inspect-run.v1`, and its outstanding attempts end as the core says when the runner
   reports them missing. After the shell restarts, every run from before is `stopped`. Each of
   these is a DL16 choice, written in `CONFORMANCE.md`.
5. **MCP over stdio.** One client session per run, opened at `run.v1`, with `rmcp` pinned as
   experiment 009 pins it. The runner declares **no** `roots` capability, so that the
   filesystem server keeps the directory it was given and no client can widen it through MCP
   Roots; a test with the real server shows a path outside the root is refused by the server
   itself, and the record shows the engine's own decision came first. `run.v1` reports
   `succeeded` only after the session's `initialize` has answered; a run that starts and never
   answers reports `failed` after a deadline (DL10, default 30 seconds, settable).
6. **Dispatch.** A `dispatch` sends exactly the `call` the intent holds, `tools/call` with the
   operation's arguments only, to the session of the selected implementation, and returns
   `complete` with the same invocation and attempt: `result`, `protocol-error` with the code
   and data, or `missing` when the process ended or the deadline passed (DL10, default 120
   seconds, settable). No Conduit key, identity, configuration or revision crosses to a `plain`
   server (IFC-007); a test inspects the bytes on the pipe. `refresh-tools-and-call` refreshes
   the tool list once and sends the read once, within its attempt, never generalized to a write
   (IFC-046). `cancel` sends `notifications/cancelled` for that attempt's request ID.
   `cancel-setup-parts` ends every run part that setup started and starts none.
7. **S1 for real.** `conduit register` on S1's project, as order 020 wired it, now completes
   setup with the real filesystem server `@modelcontextprotocol/server-filesystem` 2.1.0 for
   both directories, whose `fileSha256` the fixture pins. A test, run only when the variable
   `CONDUIT_TEST_NPM` is set, does that against temporary directories, makes one read and one
   write through the core as S1's B, and checks the bytes on disk and the records. Run it and
   report its result; the default `cargo test` stays offline.
8. **Findings and limits**: as order 020's decision 10, in the log of increment 3.

## Scope

Create and change only: `reference-model/crates/shell/`, `reference-model/Cargo.toml`,
`reference-model/Cargo.lock`, `reference-model/README.md`, `reference-model/CONFORMANCE.md` and
`docs/proposals/first-build/log-increment-3.md`. Report anything else that needs changing; do
not change `spec/`, `conformance/` or the core's decisions.

## Done when

S1's setup starts the unchanged filesystem server for both directories and a read and a write
reach it through the core, with their records; nothing the engine holds reaches a `plain`
server; no variable but PRV-023's reaches a run; no run outlives the shell. From the repository
root: `python3 tools/check_docs.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass;
in `reference-model/`, `cargo test --release`, `cargo clippy --all-targets -- -D warnings`,
`cargo fmt --check` and `CONDUIT_TEST_NPM=1 cargo test --release` pass. Check the files you
create for trailing white space yourself; do not stage anything.

## Record

Run on 2026-11-10: the worker in 49 minutes, in parallel with order 022, then a fresh verifier
in 7. The verifier found five defects, each real, all in how the runner composes a server's
command line and environment from its Package: bare program names resolved inside the package
rather than through `PATH`, symlinks with `..` refused, a fixed positional argument without
`valueHint` refused, Package environment values and defaults never applied, and server.json
variables in argument values never substituted. The admin sent the worker back once with all
five and two further decisions: the names checker gains an exact exception for the test switch
`CONDUIT_TEST_NPM`, rather than the switch being spelled by concatenation to get past it; and
the filesystem server 2.1.0 declares Node 24 or later but was qualified on Node 21.5.0, which
`CONFORMANCE.md` and the log of increment 3 now state as a limit (the runner does not check a
Package's declared engines). The admin reran every check: with `CONDUIT_TEST_NPM=1`, 122 Rust
tests pass, including S1's setup, reads, writes and history against the real server and its
refusal of a path outside the root, and clippy, fmt and the five repository checks are clean.
