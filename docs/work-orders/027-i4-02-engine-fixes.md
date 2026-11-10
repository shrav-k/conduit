# Work order 027 (I4-02): what the coordinator's run and order 025's review left open

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port and run the npm server; use it for nothing else. You cannot run `claude` or
`codex` as clients here.

## What and why

Order 025's second verifier found three faults in retirement with a retire hook, and the
coordinator's run of S1's tasks
([results](../proposals/first-build/increment-4/coordinator-run-2026-11-10.md)) found more. Each
is logged in [the log of increment 4](../proposals/first-build/log-increment-4.md). This order
fixes those that are defects of the build and settles none that is the owner's (4-01, 4-03, 4-08,
4-11 stay open).

Read: the log's findings 4-04 to 4-10; order 025 and its record; `reference-model/CONFORMANCE.md`;
the shell (`reference-model/crates/shell/`) and the core's retirement
(`reference-model/crates/core/src/retirement.rs`); the two Agent packages under `packages/`; and
these promises: TPL-018, TPL-019, OPS-022, OPS-029, LIF-008, CON-004, IFC-012.

## The decisions to apply

1. **Finding 4-04, the retire hook.** Choose the hook's own implementation before preparing any
   run, so that a remote hook is connected to as TPL-018 says and an unrelated run is never
   started for it; record a run started for the hook as a part of the retirement with its
   effect; and where the run's state is unknown, do not dispatch the hook and record its part
   unavailable, with no effectful part invented (TPL-018, OPS-022). A test for each, failing
   before and passing after.
2. **Finding 4-05, the runner test.** Make the test wait for the PID file's contents, not its
   existence. Change nothing else in the runner for it.
3. **Finding 4-07, a store that cannot be repaired.** `conduit serve` starts even where setting
   up a registered instance fails: the failure is that instance's state and its records, as for
   any setup that fails, and serve listens, so that `submit` and the other commands can repair
   what is missing. Test it: register a project whose Resource's setup fails for want of root's
   Conduit to its Pool, then serve, submit that Conduit, set the Resource up through it, and see it
   ready.
4. **Finding 4-09, inspection.** (a) `conduit inspect` works while the engine serves, by asking
   the serving engine as the root Interface, as the author's commands do; without a serving
   engine it behaves as now. (b) Inspecting an instance lists every Conduit of which it is the
   consumer, the standalone ones with the declared ones, each with its name, target and whether
   it is open. If the core package's output for `inspect.v1` has no place for this, do not change
   `spec/`: report it as a finding and add it to the CLI's output alone.
5. **Finding 4-10, the diagnostic.** Where a request names an object that does not exist
   (OPS-029), the diagnostic's text says that the object named does not exist, naming its kind
   and name as given, and that a closed Conduit and a removed instance no longer exist.
6. **Finding 4-06, Codex's own agent tools.** Codex 0.167.0 offered `collaboration.spawn_agent`,
   `collaboration.send_message`, `collaboration.list_agents`, `collaboration.wait_agent`,
   `collaboration.interrupt_agent` and `collaboration.followup_task` with `multi_agent` disabled.
   Find, from `codex features list` run with `--ignore-user-config` and from the binary's own
   strings if needed, which features offer them (likely `multi_agent_v2` or a related one), and
   disable each in `packages/agent-codex` as the other machine tools are disabled, with a test
   of the arguments. You cannot run Codex as a client; the admin checks it with the real client
   afterwards. Keep the package's version at 0.1.1 and say in its README that the admin's check
   is pending.

## Scope

Create and change: `reference-model/`, `conformance/`, `packages/agent-codex/`. Report anything
else that needs changing; do not change `spec/`.

## Done when

Each decision is applied with its tests; nothing of 4-01, 4-03, 4-08 or 4-11 is settled. From the
repository root: `python3 tools/check_docs.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass; in
`reference-model/` and `packages/agent-codex/`, `cargo test --release`,
`cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` pass, and in
`reference-model/`, `CONDUIT_TEST_NPM=1 cargo test --release`. Report the total passed and failed
across all test binaries of each cargo run. Check the files you create for trailing white space
yourself; do not stage anything.
