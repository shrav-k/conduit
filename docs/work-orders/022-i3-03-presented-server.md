# Work order 022 (I3-03): the server consumers connect to

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests
that bind a loopback port; use it for nothing else.

## What and why

The third order of increment 3, items H04 and H03 of
[the handoff](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3-handoff.md#work-the-shells-orders-can-take).
The engine presents MCP at its address, and every Agent reaches what it may reach only through
it. This order serves that address on loopback, establishes who each request comes from by
the token order 020 issues, and gives each consumer exactly its own tools, as the core decides.
It runs no Agent; order 023 does, and the admin's closeout runs real clients against it.

Read: the handoff, its section on the tool contract above all; orders 020 and 021 and their
records; the shell as they left it; `CONFORMANCE.md`; and these promises: IFC-012, IFC-013,
IFC-022, IFC-025, IFC-032, IFC-033, IFC-036, IFC-038, IFC-042, IFC-045, ACC-001, ACC-027,
ACC-036, OPS-027, REC-001. Experiment 008's stub (`experiments/008-real-client/src/stub.rs`)
serves Streamable HTTP with `rmcp` 3.9.0 and `axum`, and both real clients connected to it;
reuse what fits, in the shell's crate.

## The decisions to apply

Order 020's `Executor` answers each intent with an `Execution`. An intent whose work takes time
answers `Deferred` and returns its completion later through `Shell::complete` or
`Shell::setup_report`, never by holding the shell's lock while it waits. The engine's address
is `Config::engine_url`.

1. **`conduit serve --store DIR`** opens the store (recovering as order 020 does), starts the
   runner of order 021 and serves the engine's address: Streamable HTTP at path `/mcp`, bound
   to a loopback address only, on the port the store's configuration names. The address is the
   one order 020 put in every token's audience; if order 020 did not fix it at first start, fix
   it there now, and refuse to serve at any other. It also runs S1's setup, as `register` left
   it, if setup is outstanding. It stops cleanly on a signal, ending every run.
2. **Who a request comes from (IFC-022, IFC-042).** A request with `Authorization: Bearer`
   and a token that order 020 verifies is from that instance. A request with no token is
   identity `none`; one whose token does not verify, names another audience, or names an
   instance that no longer exists is answered with HTTP 401 and the `WWW-Authenticate` header,
   and reaches no tool. The shell never takes identity from anything else: not the client's
   name, not a header it sets, not the machine. Requests whose `Origin` is present and is not
   the engine's own origin are refused with 403, against DNS rebinding.
3. **The tools (IFC-012, IFC-033, IFC-038).** `tools/list` answers exactly the core's
   `list-tools` for that identity, with each tool's standalone schema, its annotations, and
   the private cache scope and `listChanged` the handoff names; nothing is added, renamed or
   hidden by the shell. The server declares `tools.listChanged`. After every committed
   transition, the shell calls `access::tool_list_changes` and sends
   `notifications/tools/list_changed` to that consumer's open sessions only, never to another
   consumer's. State in `CONFORMANCE.md` that no `ttlMs` is set (DL10, DL12) and that the
   engine does not infer a client's model catalog from what it sends.
4. **A call (ACC-001, ACC-036, IFC-025).** `tools/call` becomes the core's `invoke` with the
   established identity, the tool, the arguments and the serving key the request carries in
   its `_meta`; current authority is checked on every call by the core, never from a cached
   permit. A final reply is the core's MCP result unchanged. A pending reply is held, inside
   the shell, until the invocation ends, and the HTTP request stays open meanwhile. A tool not
   presented to that identity is a protocol error with `refused` in `error.data`, as the core
   answers, never a successful result. `notifications/cancelled` from the client becomes the
   core's `cancel` for that invocation (OPS-027). Every request has its record before its reply
   leaves (REC-001); a test reads them back.
5. **The management route.** `conduit call --store DIR TOOL [ARGUMENTS-JSON]` sends one
   `tools/call` to the running server as the Interface `root`, through which the person who
   runs the engine acts (FST-001, FST-002), by a token order 020's identity issues for `root`
   and keeps in the store with mode 0600, and prints the result. `conduit tools --store DIR [--as INSTANCE]`
   lists tools. These are what the admin uses to close and restore a Conduit in T7, so test
   both: closing A's Conduit to the directory, as T7 does, is seen in A's next `tools/list`,
   B's list does not change, B is sent no notification and B keeps working, and A's Conduit
   restored by explicit replacement is seen in A's next list.
6. **What is not finished (H03).** Before serving, the shell compares every operation that
   `list-tools` can present with what the core can execute, from the handoff's last section. If
   any identity in S1 would be presented an operation the core cannot carry out, stop and
   report it as a finding; do not hide it in the shell. Operations presented only to the
   management identity that the core cannot carry out yet are answered with JSON-RPC internal
   error `-32603` and a message that names the operation as unfinished, with nothing committed.
   List them in `CONFORMANCE.md` as limits of the first build.
7. **Tests** start `conduit serve` on S1 with the scripted executor or, where
   `CONDUIT_TEST_NPM` is set, the real filesystem server, and connect with an `rmcp` client as
   A and as B by the tokens the runner's identity directories hold: A lists read, list and the
   history tools and is refused a write; B writes and reads it back; a request with no token
   and one with a forged token are refused as decision 2 says; T7's close and explicit
   restoration through `conduit call` behave as decision 5 says. Run the `CONDUIT_TEST_NPM`
   tests and report them.
8. **Findings and limits**: as order 020's decision 10, in the log of increment 3.

## Scope

Create and change only: `reference-model/crates/shell/`, `reference-model/Cargo.toml`,
`reference-model/Cargo.lock`, `reference-model/README.md`, `reference-model/CONFORMANCE.md` and
`https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-3.md`. Report anything else that needs changing; do
not change `spec/`, `conformance/` or the core's decisions.

## Done when

Two consumers connected by their tokens see exactly their own tools, A is refused a write and
B makes one, an unauthenticated or forged request reaches no tool, a close is seen by the one
consumer it affects, and every request has its record before its reply. From the repository
root: `python3 tools/check_docs.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass;
in `reference-model/`, `cargo test --release`, `cargo clippy --all-targets -- -D warnings`,
`cargo fmt --check` and `CONDUIT_TEST_NPM=1 cargo test --release` pass. Check the files you
create for trailing white space yourself; do not stage anything.

## Record

Run on 2026-11-10, in parallel with order 021: the worker in 48 minutes, then a fresh verifier
in 5. The verifier found three defects, each real: a closed response stream did not cancel its
invocation (OPS-027), the inventory of unfinished operations skipped custom controls, and a
malformed `tools/call` with no identity was recorded as a refusal instead of getting the
protocol's error and no record (IFC-036). The admin sent the worker back with all three. Order
021 merged meanwhile, and the merge of main into this order's tree conflicted in six files; a
third round resolved them, joined `conduit serve` to the runner, and added the end-to-end test
the order asked for, run as a process on S1 with the real filesystem server: A refused a write,
B writing and reading back, no token and a forged token refused, T7's close of A's Conduit and
its explicit restoration through `conduit call` with B unaffected and sent nothing, and no run
left after the server stops. A second fresh verifier then read the joined result and found four
more: a session mismatch with an established identity returned 401 with no record (REC-001),
rejection records dropped the caller's idempotency and serving metadata (REC-002), a session
that overflowed its notification queue stopped hearing changes while staying open (IFC-038),
and loopback hosts the HTTP layer refuses were accepted at first start. A fourth round fixed
all four, the last by refusing every loopback host but `127.0.0.1`, `localhost` and `::1`.
The admin reran every check: with `CONDUIT_TEST_NPM=1`, 134 Rust tests pass, and clippy, fmt
and the five repository checks are clean.
