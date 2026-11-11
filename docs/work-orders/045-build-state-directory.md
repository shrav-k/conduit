# Work order 045 (S1): each instance's state directory

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count.

## What and why

An order of spec batch 1's shell lane ([the plan](../proposals/spec-batch-1/README.md), its order
045, S1). Order 038a wrote that each instance gets a private state directory, which is an Agent's
home (ruling 49): the engine makes it, keyed by the instance's ID, and removes it with the instance;
its path reaches the implementation as the variable named by `stateDirVariable` in
`spec/names.yaml` (MAN-018). Read the plan's order 045, the promises 038a wrote for it (search
`spec/` for `stateDirVariable` and the state directory), DL16, and the shell's runner.

## What to do

1. **Build** the state directory: created with mode 0700, keyed by instance ID; passed as the
   variable `spec/names.yaml` names; removed when the instance is removed or unregistered, with a
   failed delete retried; MAN-018's check that no item binds the variable itself. Every decision
   cites its promise.
2. **Vectors and tests**: state-dir-binding, citing its promises and listed in the manifest, and the
   shell tests the plan calls for (creation, mode, removal, retry). Write a declaration of the shapes
   and run `python3 tools/vector_delta.py --declared FILE`: no undeclared change.

## Scope

Change: `engine/`, `conformance/`, and the declaration under `docs/work-orders/045-declared.yaml`. Do not change `spec/` (report any defect as a finding) or a record. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py`, `python3 tools/check_rulings.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py`, `python3 -m unittest discover -s tools/tests` (with the
interpreter that has PyYAML), `python3 tools/vector_delta.py --declared docs/work-orders/045-declared.yaml`
and `git diff --check` pass; in `engine/`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo test --release`; in
`experiments/011-short-form-spike/`, `cargo test --release`. Report the totals passed and failed;
tests that bind loopback ports may fail in your sandbox: say so, and the admin reruns them. Do not
stage anything.
