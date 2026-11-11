# Work order 023 (I3-04): the Agent types for Claude Code and Codex, as packages of their own

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests
that bind a loopback port; use it for nothing else. You cannot run `claude` or `codex` as clients
here: tests use a scripted client, and the admin runs the real ones at the closeout.

## What and why

The fourth order of increment 3, items H05 to H09 of
[the handoff](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3-handoff.md#work-the-shells-orders-can-take),
and the first build's [needs 1 to 10](../implementation/FIRST_BUILD.md#what-increment-3-needs).
An Agent in S1 is a real client, run by the engine's runner, reaching the shared directory and
the history only through the engine. The owner ruled (ruling 47's review, point 4) that these
two Agent types are built from the start as self-contained packages, each able to move to a
repository of its own, with a pinned copy kept in this one. Experiment 009 found how each client
can be launched this way; this order makes that a product.

Read: the handoff; orders 020 to 022 and their records; the shell as they left it; needs 1 to 10
of the first build; [ruling 48](../proposals/ruling-48/README.md), point 1 and 2;
[experiment 009](../../experiments/009-native-launch/README.md), its results and its code
(`src/client.rs`, `src/wrapper.rs`, `src/runner.rs`), and experiment 008's later-turn results;
`conformance/fixtures/s1/packages/agents/`; and these promises: MAN-010, PKG-020, PRV-007,
PRV-022, PRV-023, TPL-019, IMP-001, IMP-002, IFC-012, IFC-013, IFC-038.

## The decisions to apply

1. **Two packages**, `packages/agent-claude-code/` and `packages/agent-codex/`, each a Rust
   crate of its own (its own `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` pinned as
   `reference-model/` pins it, and ignored build output), with no path dependency on anything
   outside its directory: what both need is copied, not shared. Each directory also holds the
   package's `conduit.yaml` and `contracts.yaml`, a `README.md` that names its maintainer as the
   project's owner and the client versions it was tested with (Claude Code 2.1.318, Codex
   0.167.0, or what `--version` reports here), and a test of its own.
2. **The Type.** Each exports one Agent Type, `claude-code.v1` and `codex.v1`, whose schema
   keeps the fixture's `accountServices` and `peerMessaging` booleans and adds an optional
   `model` string. Its implementation item is a `command` whose program is the package's
   wrapper binary at a path relative to the archive's root, with `environment` binding each of
   the three inputs to a variable by a JSON Pointer (MAN-010). It offers one operation, `turn`,
   whose input is a message string and whose output is the reply text, marked `startsModelWork`
   (TPL-019), with `records` that keep neither inputs nor outputs. Setup starts no model work:
   the wrapper starts its client only at the first `turn`.
3. **The wrapper** is an MCP server over stdio, as experiment 009's is, presenting only `turn`.
   It reads `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR` from its environment (PRV-022), and
   configures its client with the engine's server as its only MCP server. One conversation per
   Agent for the wrapper's life: Claude Code as one streamed session (`--input-format
   stream-json`), Codex resumed by thread in a new process per turn (H06). The client runs in
   the private, empty working directory that the runner gives the run (DL16), never in the
   shared directory, and the client's configuration that the wrapper writes goes there.
4. **The owner's configuration kept out** (need 3, H07): Claude Code with `--setting-sources`
   naming no user source, `--settings` and `--mcp-config` from the wrapper's own files, and
   `--strict-mcp-config`; Codex with `--ignore-user-config`, `--strict-config` and
   `--ignore-rules`. `ANTHROPIC_API_KEY` and `OPENAI_API_KEY` are removed from the client's
   environment, so the login is the owner's shared one (ruling 48, point 2).
5. **The Agent's own reach, off by default** (need 4, H08, ruling 48 point 1). With
   `accountServices` false: Claude Code's claude.ai connectors off and Codex's apps disabled
   (`--disable apps`). With `peerMessaging` false: Claude Code's `SendMessage`, `ListAgents`
   and `RemoteTrigger` disallowed and messages from other sessions refused. Each input set
   true turns its part on for that Agent alone. The client's own built-in tools that reach the
   machine are off for Claude Code (`--tools ""`, with the engine's server's tools allowed), and
   for Codex its sandbox is `read-only` in the private directory, with web search disabled.
   Codex's read-only shell can still read files outside the engine: say so in the package's
   README, as a route around the engine the closeout must report (ACC-027), not as a solved
   problem.
6. **The token from its file** (need 6, H09). Claude Code reads it by `headersHelper`, Codex by
   `http_headers_helper`, each running the wrapper binary in a helper mode that prints the
   `Authorization` header from `token` in the identity directory. Codex's helper is given the
   directory on its command line, since Codex runs it without the run's environment and rejects
   `env_vars` for an HTTP server (experiment 009). The token is never in a variable, an argument
   or a log.
7. **One approval** (need 7): an allow rule for the engine server's tools in Claude Code's
   settings, and `default_tools_approval_mode = "approve"` for that server in Codex, so that the
   engine's decision is the only one per call.
8. **Access changes at a turn's boundary** (need 2, H06). When the wrapper's client is sent
   `notifications/tools/list_changed`, the change reaches the model at its next turn, as
   experiment 008 found. Each `turn` result's structured content reports the engine's tool list
   the client saw at that turn's start, as the client's own events show it, apart from the
   engine's list, so the closeout can compare them.
9. **Build and pin.** A script-free step: `cargo run --release --bin package` in each crate
   (or one documented `cargo` command) builds the wrapper for the host and writes a gzip
   archive in the form of the S1 fixture's archives (USTAR, sorted names, zero owner and time,
   the binary at mode 0755, the two YAML files at 0644) with its SHA-256. The shell's
   `conduit register` gains `--package ARCHIVE#SHA256`, which installs an archive through the
   core's `install-archive` before registration, checking the digest, so the runner can run its
   `command` item. Add a project for the live S1 at `reference-model/examples/s1/`: S1's
   fixture project with its two Agent templates using these Types, each with both reach inputs
   false. Copy each package's two YAML files into `conformance/fixtures/agents/<package>/` as
   the pinned copy, with a README saying where they come from, and add one vector that
   registers the live project against them; touch no other vector. The packages' YAML holds
   the names of `spec/names.yaml`, so extend `tools/set_names.py` to rewrite and check
   `packages/` too, as it does `conformance/`.
10. **Tests without the real clients**: a scripted client binary, as experiment 009's, stands in
    for each, and tests check the exact arguments and environment the wrapper gives it for each
    combination of the two inputs, that the helper prints the header from the file and nothing
    else, that no key or token reaches an argument or a log, and that the wrapper presents only
    `turn`. One test, in the shell's crate, runs S1's setup with both Agents on the scripted
    clients, calls `turn` on each through `conduit call`, and shows that each scripted client
    reached the engine with its own identity.
11. **What only the real clients can show** (need 10): write
    `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3/closeout.md`, not binding, as the admin's plan for
    the closeout: each check of need 10 and of increment 3's done-when that needs a real client,
    with the sentinel or evidence that would show it (such as an instruction file, a hook and a
    user setting each planted where the client would load it if it could), the command to run
    and what a pass and a fail look like. Do not run it.
12. **Findings and limits**: as order 020's decision 10, in the log of increment 3.

## Scope

Create and change only: `packages/agent-claude-code/`, `packages/agent-codex/`,
`reference-model/crates/shell/`, `reference-model/examples/s1/`, `reference-model/Cargo.lock`,
`reference-model/README.md`, `reference-model/CONFORMANCE.md`,
`conformance/fixtures/agents/`, `tools/set_names.py`, one new vector under `conformance/` with its manifest entry,
`https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-3.md` and
`https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3/closeout.md`. Report anything else that needs changing;
do not change `spec/` or the core's decisions.

## Done when

Both packages build and pack on their own, their wrappers launch the scripted clients with
exactly the flags, environment and helper of decisions 3 to 7 for every combination of the
reach inputs, S1's setup runs both Agents on the scripted clients through the engine with their
own identities, the pinned copies register in a vector, and the closeout plan is written. From
the repository root: `python3 tools/check_docs.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass;
in `reference-model/` and in each package directory, `cargo test --release`,
`cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` pass. Check the files you
create for trailing white space yourself; do not stage anything.

## Record

Run on 2026-11-10: the worker in 32 minutes, then a fresh verifier in 9. The worker reported
one finding: the order asked for an optional `model` input bound to a variable, which MAN-017
does not allow; the admin accepted its settlement, a required string with an empty default,
logged as finding 3-02. The verifier found four defects and two smaller faults, each real:
`--package` read only `conduit.yaml` and not a linked manifest; a turn whose message was `-`
reached Codex as its marker for standard input; Claude Code's repeated start-of-session tool
list was reported as current, which experiment 009 had warned against; a cancelled turn
queued behind another could still start model work; and a test assumed Cargo's default
target directory. The admin sent the worker back once with all six. The admin also updated
`AGENTS.md` and `CONTRIBUTING.md`, which this order's scope left out, to say the names check
covers `packages/`. The admin reran every check: 136 Rust tests in `reference-model/` and six
in each package pass, and clippy, fmt and the five repository checks are clean. No real client
ran; the [closeout plan](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#how-to-run-the-open-checks) is for the admin.
