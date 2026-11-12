# Work order 042 (E4): tombstones, lift, the inventory, provenance and reconcile limits

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count.

## What and why

The fifth build order of spec batch 1 ([the plan](../proposals/spec-batch-1/README.md), its order
042, E4): ruling 53's engine-enforced tombstones and inventory, ruling 55 A7's lift, ruling 54's
provenance (ACC-044), and ruling 62 C2 with ruling 66 point 4's reconcile limits (ACC-055). Read the
plan's order 042 and its conflicts C18, C26 and C27, the promises LIF-040, LIF-041, ACC-044, ACC-053,
ACC-055, LIF-013 and OPS-036, and the core package's tombstone, inventory and provenance schemas.

## What to do

1. **Build** as the plan states: tombstones in a separate map, made on close, on retirement and
   in `close_ends`; `lift.v1` replacing its stub; LIF-041 at every instantiation, register-replacement,
   open and setup part; the inventory, with scope and administrators; ACC-055, with the sender read
   from the invocation (hooks inherit it); ACC-044's capture; `list.v1` showing tombstones to whoever
   may see them; the runner's projections; `conduit inspect` reading the inventory. Every decision
   cites its promise.
2. **New vectors** the plan names (about 36), each citing its promises and listed in the manifest;
   **lift insertions** where a test reopens what it closed (restore 14, restore-current-template 12,
   t4 10 and 15, s1/whole 50, root-closes-own). Write a declaration of the shapes and run
   `python3 tools/vector_delta.py --declared FILE`: no undeclared change.

## Scope

Change: `engine/`, `conformance/`, and the declaration under `docs/work-orders/042-declared.yaml`. Do not change `spec/` (report any defect you find as a finding) or a record. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py`, `python3 tools/check_rulings.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py`, `python3 -m unittest discover -s tools/tests` (with the
interpreter that has PyYAML), `python3 tools/vector_delta.py --declared docs/work-orders/042-declared.yaml`
and `git diff --check` pass; in `engine/`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo test --release`; in
`experiments/011-short-form-spike/`, `cargo test --release`. Report the totals passed and failed;
tests that bind loopback ports may fail in your sandbox: say so, and the admin reruns them. Do not
stage anything.

## Record

The first work order run entirely through Conduit (A78, ruling 70): on 2026-11-11 and 12 its four
worker turns and four verifier turns ran as turns of Conduit's own Agents worker-1 and verifier-1
(codex.v2, the real store, the dogfood harness), each answering with a report valid against its
schema. Round 4's verifier found no new fault. One turn of round 3 failed when the Mac briefly lost
its network (the Agent package answered "client failure: other"; a dogfood finding), and was run
again; one guard tripped in that failed turn, when GitHub's refs could not be read. The admin then
rebased the work onto main, which had gained order 070's lazy publication and order 081, and
resolved the conflicts in the core's access.rs (the bound-coverage rule kept in a state form for
the inventory and a view form for publication) and lib.rs. One vector then failed
(e4-tombstone-register-replacement, LIF-041); a Codex round on the Mac fixed it, after which all
491 vectors passed on the normal and differential builds. That round's fresh verifier found the
Fleet reconciliation check matching a sender through a reused name; the admin fixed it to match by
ID only (ruling 71). Kept for later work: the inventory-reading vector does not exercise a Fleet
holder, and node-requests' readmission injects the expected node instead of testing readmission.
