# Work order 039 (E1): local names, expectedId, descriptive tools, and root's own Conduits

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count.

## What and why

The first build order of spec batch 1 ([the plan](../proposals/spec-batch-1/README.md), its order
039, E1). Orders 036a and 036b wrote the promises and the core package; this order builds the part
the rest stands on: ruling 53's `as` (a local name for a standalone Conduit) with local names unique
per consumer (the plan's F3), `expectedId` on open.v1 and close.v1, tool descriptions that name the
target and its Type, and ruling 62's C1 (only root opens, or registers, a Conduit whose consumer is
root, ACC-054). Read the plan's order 039 and its conflicts that touch it, the promises 036a wrote
(OBJ-001, OBJ-013, IFC-014, IFC-033, LIF-037, ACC-054 and their neighbours), the staged edits in
`docs/work-orders/036b-staged/039.md`, and the engine's `engine/crates/core/src/registration.rs`,
`requests.rs` and `schemas.rs`.

## What to do

1. **Build** as the plan states: local names and IFC-014's uniqueness in `registration.rs`;
   `expectedId` on open and close through LIF-037's path in `requests.rs`; ACC-054's sender check on
   open and on register; IFC-033's descriptions in `schemas.rs`. Every decision the core makes cites
   its promise, as the core does today.
2. **Land the staged edits** of `036b-staged/039.md` verbatim, and delete them from the staged file.
3. **Vectors.** The twelve the plan names: conduit-as, conduit-as-invalid, as-unique, as-declared,
   expected-id-open, expected-id-close, expected-id-access-order, e05-description-target,
   e05-description-bare, root-consumer-open, root-consumer-onward, root-consumer-register, each
   citing its promises and listed in `conformance/manifest.yaml`. Re-expect only the cases the plan
   names (the four collision cases, tool-collision, e05-local), each change justified by a promise.
   Write a declaration of these shapes and run `python3 tools/vector_delta.py --declared FILE`: it
   must end with no undeclared change. Never regenerate an expectation from the engine's output.
4. **Experiment 011.** Update its test so the translation's `as` is kept, and run it: S1's states
   must now register with `as` (finding 031-01).

## Scope

Change: `engine/`, `conformance/`, `experiments/011-short-form-spike/` (its test and a dated note in
its results), the staged file, and the order's declaration under `docs/work-orders/039-declared.yaml`.
Do not change `spec/` (report any defect you find in it as a finding) or a record. Report anything
else.

## Done when

From the repository root: `python3 tools/check_docs.py`, `python3 tools/check_rulings.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py`, `python3 -m unittest discover -s tools/tests` (with the
interpreter that has PyYAML), `python3 tools/vector_delta.py --declared docs/work-orders/039-declared.yaml`
and `git diff --check` pass; in `engine/`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo test --release`; in
`experiments/011-short-form-spike/`, `cargo test --release`. Report the totals passed and failed;
tests that bind loopback ports may fail in your sandbox: say so, and the admin reruns them. Do not
stage anything.
