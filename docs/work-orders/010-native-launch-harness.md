# Work order 010: the harness of experiment 009 (A4), wrappers that run each client as an Agent

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `002-real-client-harness.schema.json`. This order
has network access, to fetch crates; use it for nothing else. You run no agent.

## What and why

Experiment 009, native launch and identity (A4), asks whether Claude Code and Codex can each be
run as an Agent is run on the engine's own runner. Its
[pre-registration](../../experiments/009-native-launch/README.md) fixes what it must show; read
it first, then [what increment 3 needs](../implementation/FIRST_BUILD.md#what-increment-3-needs),
experiment 008's [harness](../../experiments/008-real-client/HARNESS.md) and both its results,
and PRV-003, PRV-007, PRV-022, PRV-023, TPL-019, IFC-022 and DL16.

## The decisions to apply

1. **One Cargo project in [experiments/009-native-launch/](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch)**, with `Cargo.lock`, the pinned
   toolchain of `reference-model/`, and build output ignored. Reuse experiment 008's stub as a
   path dependency on its library rather than copying it; change 008 only if a small public
   interface is needed, and say so.
2. **Two wrappers**, each a program that, started with its standard input and output as an MCP
   server, serves one tool that runs one turn of its client and returns the client's final text
   with the events it reports. Claude Code's wrapper keeps one process on streamed input
   (`--input-format stream-json`) and sends each turn's message after the last result; Codex's
   resumes the same session for each turn (`codex exec resume <id>`). Each starts its client with
   the isolation of experiment 008 (Claude Code: `--setting-sources`, `--strict-mcp-config`,
   `--mcp-config`, `--tools ""` and an allow of the stub's server only; Codex:
   `--ignore-user-config`, `--strict-config`, `--disable apps`, a read-only sandbox and the stub's
   server approved), plus, for Claude Code, settings that deny `SendMessage`, `ListAgents` and
   `RemoteTrigger`, turn off the claude.ai connectors and refuse messages from other sessions:
   find each setting's exact name in Claude Code's documentation or `claude --help`, and say
   which you could not confirm. Each client reads its token by its header helper from
   `CONDUIT_IDENTITY_DIR`, and each wrapper removes `ANTHROPIC_API_KEY` and `OPENAI_API_KEY`
   from its client's environment.
3. **A runner** that starts each wrapper as a run would be started: only the variables PRV-022
   and PRV-023 name, a private empty working directory, and an identity directory with its own
   token naming its own instance. It asks for two turns, with experiment 008's stub opening one
   Conduit and closing another between them, and records per run the stub's log, the wrapper's
   exchange, the client's start report (its tools, its MCP servers, its authentication) and the
   model's text. It also runs both Agents at once, against one stub, with two tokens. Three runs
   of each; `--dry-run` prints them all; the output directory is an argument with no default.
4. **Tests that need no agent**: the wrappers' MCP interface driven by a scripted client with a
   fake client program standing in for `claude` and `codex`, the environment each run is given,
   and the dry run.
5. **Where a promise is not clear enough to build the launch from**, follow the nearest sound
   reading and list it under `findings`.

## Scope

Create and change only files under [experiments/009-native-launch/](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch) other than its
`README.md`, and experiment 008's library only as decision 1 allows.

## Done when

In [experiments/009-native-launch/](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch): `cargo test --release`, `cargo clippy --all-targets -- -D
warnings` and `cargo fmt --check` pass, and the dry run prints every run. From the repository
root, `python3 tools/check_docs.py`, `python3 tools/check_roadmap.py` and `git diff --check`
pass. Your report gives the command the admin runs and what it needs.

## Record

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 13 minutes, then a second round. Two wrappers in Rust, one per client, and a
  runner of nine scenarios, reusing experiment 008's stub.
- **Verifier**: 4 minutes. Two `should`: a test of attribution that would pass with the
  credentials swapped, and a test of redaction that never reached the redaction.
- **Admin**: ran the harness against the real clients. Codex failed twice, because it runs its
  header helper without the run's environment and rejects `env_vars` for an HTTP server; the
  admin had the Codex wrapper write the identity directory into the helper's command line, and
  the third attempt completed all nine scenarios. It sent the worker back once to test that
  change and fix the two tests. The worker's sandbox, run without network access, refused the
  tests that bind a loopback port, so the admin ran them: all pass.
