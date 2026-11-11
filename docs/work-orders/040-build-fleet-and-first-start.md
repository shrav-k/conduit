# Work order 040 (E2): the Fleet family, first start and list.v1

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count.

## What and why

The second build order of spec batch 1 ([the plan](../proposals/spec-batch-1/README.md), its order
040, E2). The Fleet family (rulings 59 and 60) enters the engine: the sixth family in every family
list, root's default Conduit to `system` at first start with root's final tool lists (the plan's
C11), the trust domain named at first start (ruling 51), and the `list.v1` handler that lists only
what the caller may inspect, with no counts (ruling 59, A4). Read the plan's order 040 and the
conflicts that touch it (C9, C10, C11), the promises 036a wrote (FLEET-001 to 006, OPS-036, FST-002,
FST-007), the core package 036b wrote, the staged edits in `docs/work-orders/036b-staged/040.md` and
`040.patch`, and the vector-impact list in `036b-staged/vector-impact.md`.

## What to do

1. **Build** as the plan states: Fleet in every family list it names (the loader's `FAMILIES`,
   `tools/check_schemas.py`, and the shell and core sites listed, under `engine/`); FLEET-001 and
   FLEET-006; first start with root's `system` default from `root-pool`, the final root lists, and
   `trustDomain` validated and recorded; the `list.v1` handler (the may-inspect filter, no counts,
   `kind`); stubs for `lift`, `answer` and `sign-in` that report unavailable naming the order that
   builds them; `conduit init` makes or takes the trust domain. Every decision cites its promise.
2. **Land the staged edits**: apply `036b-staged/040.patch` exactly (`git apply`), then delete
   `040.patch` and `040.md`.
3. **Vectors.** The new ones the plan names (fleet-instance, fleet-names, fleet-consumer,
   first-start root-system, trust-domain, list management, kind-filter, refused-without-list,
   list-from-first-start), each citing its promises and listed in the manifest; the ones the plan
   changes once (first-start initial, repeat and lifecycle; protected; s1/whole step 0;
   `case.schema.json`; the snapshot; access/fixed-kind-references step 2), each change justified by
   a promise. Write a declaration of these shapes and run `python3 tools/vector_delta.py --declared
   FILE`: no undeclared change. Never regenerate an expectation from the engine's output.

## Scope

Change: `engine/`, `conformance/`, `tools/check_schemas.py` (its family list), `experiments/011-short-form-spike/` (only if its test needs the new family), the staged files, and the declaration under `docs/work-orders/040-declared.yaml`. Do not change `spec/` (report any defect you find as a finding) or a record. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py`, `python3 tools/check_rulings.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py`, `python3 -m unittest discover -s tools/tests` (with the
interpreter that has PyYAML), `python3 tools/vector_delta.py --declared docs/work-orders/040-declared.yaml`
and `git diff --check` pass; in `engine/`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo test --release`; in
`experiments/011-short-form-spike/`, `cargo test --release`. Report the totals passed and failed;
tests that bind loopback ports may fail in your sandbox: say so, and the admin reruns them. Do not
stage anything.
