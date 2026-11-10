# Work order 028 (I4-03): commands an author can rely on

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port and run the npm server; use it for nothing else. You cannot run `claude` or
`codex` as clients here.

## What and why

A fresh Codex author did S1's tasks with only the specification and the engine's commands
([its run](../proposals/first-build/increment-4/fresh-author-run-2026-11-10.md)) and wrote 130
lines of scripts to drive them, because the commands did not tell it reliably what had happened.
The owner ruled that every connection is to be as explicit and descriptive as it can be without
disclosing what the caller may not learn
([ruling 52](../proposals/ruling-52/README.md), point 2). This order makes the commands meet it,
and removes the engine's last knowledge of S1's own packages.

Read: findings 4-13 to 4-16 in [the log of increment 4](../proposals/first-build/log-increment-4.md);
the shell (`reference-model/crates/shell/`, `main.rs`, `author.rs` and `boundary.rs` above all);
`reference-model/README.md` and `CONFORMANCE.md`; the two Agent packages' READMEs; and the
fresh author's log, `docs/proposals/first-build/increment-4/fresh-author-run/log.txt`.

## The decisions to apply

1. **Exit status (4-13).** Every command exits non-zero when any result it prints did not
   succeed, `isError` or an outcome other than `succeeded`, and zero only when every one did.
   `submit` prints, after its results, one line per step it took (registration, each open, each
   setup) with that step's outcome, so that a partial success says exactly which steps succeeded.
2. **Discovery (4-14).** `conduit --help` and `conduit COMMAND --help` describe every command, its
   arguments and an example. `tools --as NAME` accepts `NAME` or `KIND/NAME`, and where the
   instance exists but has no presented tools yet, or does not exist, says which, and what to do.
   Every error a command prints says what was asked, what failed and, where the engine knows it,
   what would fix it. Update both Agent packages' READMEs to the `install` command in place of
   `register --package`.
3. **Timeouts (4-15).** Find why the commands' client intermittently times out initializing its
   session with the serving engine (the fresh author saw it with calls in parallel and in
   sequence). Fix the cause if it is the engine's or the client's; if it is not found, the error
   names the step and the time waited, and the client retries initialization, never a request
   already sent. Add a test that runs several commands at once against one serving engine.
4. **Stale conditions (4-16).** Inspection shows a condition that no longer holds as past (with
   when it ended), not beside the current state as if it held. Do not change `spec/`; if the
   specification's status form gives no place for this, report it as a finding.
5. **S1's packages are packages like any other.** `conduit register` stops preinstalling S1's
   filesystem and history snapshots: the engine knows nothing of S1. Every test, fixture and
   README that relied on it installs them with `install` (or the library's equivalent) first. The
   conformance vectors that read S1's snapshots from fixtures keep doing so, as fixtures.

## Scope

Create and change: `reference-model/`, `conformance/`, and the READMEs of `packages/agent-claude-code/`
and `packages/agent-codex/`. Report anything else that needs changing; do not change `spec/`.

## Done when

Each decision is applied with its tests. From the repository root: `python3 tools/check_docs.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py` and `git diff --check` pass; in `reference-model/`,
`cargo test --release`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check` and
`CONDUIT_TEST_NPM=1 cargo test --release` pass. Report the total passed and failed across all
test binaries of each cargo run. Check the files you create for trailing white space yourself;
do not stage anything.
