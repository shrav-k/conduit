# Work order 002: the harness of experiment 008, a stub of the presented server and a runner

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `002-real-client-harness.schema.json` beside this
file. This order has network access, to fetch crates; use it for nothing else.

## What and why

Experiment 008 (A71) asks what Claude Code and Codex, unchanged, show their models when the
engine's presented server answers as the promises say. Its pre-registration,
[experiments/008-real-client/README.md](../../experiments/008-real-client/README.md), fixes the
question, the cases, the variants, what is recorded and the limits: read it first and keep to
it. This order builds the harness: a stub of the presented server and a runner, in Rust. You
run no agent: the admin session runs the runner afterwards.

## Read

The pre-registration above; in `spec/interface/implementation.yaml` IFC-012, IFC-013, IFC-022,
IFC-025, IFC-033, IFC-036, IFC-038 and IFC-042; PRV-022 in `spec/promises/providers.yaml`;
DL13 in `spec/implementation-defined.yaml`; ACC-011; OPS-001 and OPS-005 in
`spec/conventions/operations.yaml`; `spec/names.yaml` for the `_meta` prefix and the names under
`identity`. For the clients: `claude --help`, `codex exec --help`, and their documentation for
an HTTP MCP server with headers from a helper command (Claude Code's `headersHelper`, Codex's
`mcp_servers.<id>.http_headers_helper`), for Claude Code's discovery cache
(`MCP_DISCOVERY_CACHE`) and for Codex's cached catalog (`startup_readiness`).

## The decisions to apply

1. **One Cargo project in `experiments/008-real-client/`**, with `Cargo.lock`, a
   `rust-toolchain.toml` pinning 1.97.1 as `reference-model/` does, and build output ignored.
   Use existing crates: the official Rust SDK of the Model Context Protocol (`rmcp`) if it lets
   the stub set every field these promises need (`_meta`, `isError`, `cacheScope`,
   `listChanged`, notifications on an open stream, the protocol revisions below); otherwise an
   HTTP server crate and JSON-RPC by hand. Say which, and why.
2. **The stub** does what the pre-registration's "The stub" says. Name the tools as IFC-012
   does (the Conduit's local name and the operation's export name, joined by a dot). Take every
   `_meta` key from `spec/names.yaml`, never written by hand. Accept the protocol revisions that
   these clients may offer, 2025-06-18, 2025-11-25 and 2026-07-28, as far as the crate allows,
   and log which one each session negotiates. Where a promise cannot be followed for a revision,
   the stub follows the revision and your report says so.
3. **The scenario** is fixed in code: a Conduit `files` listing `read` and `write` on a
   directory of a few dummy files the stub serves from memory, and a Conduit `notes` listing
   `read`, closed at first. `control.next` steps through the states each case needs. Write the
   states for C1 to C5 as a table in a `HARNESS.md` in the directory.
4. **The runner** holds the matrix and the prompts of the pre-registration, fixed in code, and
   for each run: starts the stub on a free loopback port, writes the token file, starts the
   client headless in a fresh lab directory under an output directory it is given (never inside
   the repository), with the isolation of the pre-registration's method, step 3, and records
   the stub's log, the client's event stream, its standard error and the model's final text,
   one directory per run. It never edits a file of the owner's configuration, and gives each
   client no tool but the stub's: for Claude Code, check whether `--tools ""` removes its
   built-in tools, and allow only the stub's server; for Codex, a read-only sandbox. It closes
   each client's standard input. It has `--dry-run`, which prints every command without running
   one, and `--only <cell>`.
5. **Tests that need no agent**: a scripted client drives the stub over HTTP through every case
   and checks each answer against the promise it follows, citing the ID in the test's name or a
   comment: admission by token, and 401 on the first request when asked; the list per state;
   `listChanged` and the notification on `control.next`; `cacheScope`; the `_meta` keys and
   `isError` for succeeded, failed, unknown and refused; the answer to a tool no longer
   presented. And a test that the runner's `--dry-run` produces the whole matrix.
6. **A finding of the first build**: where a promise is not clear enough to build the stub from,
   follow the nearest sound reading and list it in your report under `findings`, with the IDs.

## Scope

Create and change only files under `experiments/008-real-client/`, apart from the
pre-registration `README.md`, which you leave as it is. Report anything else that needs
changing.

## Done when

In `experiments/008-real-client/`: `cargo test --release`, `cargo clippy --all-targets -- -D
warnings` and `cargo fmt --check` pass, and `cargo run --release -- --dry-run` (or the runner's
own command) prints the whole matrix. From the repository root, `python3 tools/check_docs.py`,
`python3 tools/check_roadmap.py` and `git diff --check` pass. Your report gives the exact command
the admin runs, its expected duration, and what it needs installed.
