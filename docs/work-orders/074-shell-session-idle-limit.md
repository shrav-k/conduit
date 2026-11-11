# Work order 074: an MCP session outlives the calls it carries

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count.

## What and why

A fault found while planning A78, Conduit running its own work orders
([ruling 70](../proposals/ruling-70/README.md)). The engine's MCP server builds its session manager
with rmcp's defaults (`engine/crates/shell/src/server.rs`, the `session_manager: Arc::default()`
in `serve()`). rmcp 3.9.0's default `SessionConfig` closes a session after 300 seconds without
activity, and a tool call that is still running does not count as activity. So every call that
runs longer than about five minutes is cancelled when its session closes, whatever the shell's
`call_timeout` allows. A critic reproduced it: "request ct CANCELLED after 305.0s". A worker's turn
on a real work order runs for one to two hours.

The fix keeps a session alive at least as long as the longest call it may carry. It changes no
outcome, record, ID, vector or spec text.

## What to do

1. **Read rmcp 3.9.0's session manager first.** It is in the cargo registry, under
   `rmcp-3.9.0/src/transport/streamable_http_server/session/local.rs`: `LocalSessionManager`,
   `SessionConfig` and its `keep_alive`, and where the idle timer is reset. Confirm the cause above,
   and say in your report what counts as activity.
2. **Build the session manager from the shell's config.** In `serve()`, set its `keep_alive` to the
   shell's `call_timeout` plus a margin, for example 60 seconds. If rmcp has a better way to keep a
   session open while a call is running, use it and say why.
3. **Tests:**
   - a unit test that the session keep-alive is longer than `call_timeout`, at the default and at a
     large value, such as 4 hours;
   - if it is practical without slowing the suite, a test that a session stays open while a call
     runs past a short keep-alive. The test may lower the margin through the config. Do not add a
     test that sleeps for minutes.
4. **`engine/CONFORMANCE.md`:** a DL10 row for the session idle limit. It is tied to
   `call_timeout`, and Config and the CLI set it.
5. **A finding** in the open increment's log under `docs/proposals/first-build/`, as the log's form
   has it: the cause, the fix, and that it was found planning A78.

## Scope

Change:

- `engine/crates/shell/`;
- `engine/CONFORMANCE.md`;
- the open increment's log under `docs/proposals/first-build/`.

Do not change:

- the core, `conformance/` or `spec/`;
- any vector;
- any other increment's log.

Order 041b also edits `server.rs`, a few lines near `serve()`. Keep your change small and local
so that either order rebases easily.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_roadmap.py`
- `git diff --check`

In `engine/`:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release -p conduit-shell`, and then the full `cargo test --release` once.

Report the totals passed and failed. Tests that bind loopback ports may fail in your sandbox; if
they do, say so, and the admin reruns them. Do not stage anything.
