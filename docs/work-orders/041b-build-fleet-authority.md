# Work order 041b (E3b): authority through Fleets

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count.

## What and why

The fourth build order of spec batch 1 ([the plan](../proposals/spec-batch-1/README.md), its order
041b, E3b): setting up, retiring, inspecting and reconciling an Agent go through its Fleet, as a
Resource's go through its Pool (ruling 59 A1, ACC-033's rows 1A, 4A, 6A, 8A and 14A); a Fleet
retires as a Pool does, with `retire-with-agents.v1` (ruling 59 A2). Read the plan's order 041b and
the conflicts that touch it, ACC-033 as order 036a wrote it, LIF-020, LIF-021, CON-024, REV-010 and
REV-014, and the shell's setup loop.

## What to do

1. **Build** as the plan states: ACC-033's rows 1A, 4A, 6A, 8A and 14A, Agents leaving rows 5, 7
   and 14, and the retired-Fleet fallbacks; LIF-020 and LIF-021 for Pools and Fleets alike;
   `retire-with-agents.v1`; CON-024, REV-010, REV-014; the shell's setup loop sets up a Fleet before
   its Agents, and Agents through root's Fleet default. Every decision cites its promise.
2. **The shell's tests and the vectors the plan names**: the set-up requests in the 22 directories,
   changes/t4 step 2, t5 step 5, `engine/examples/s1` (keep `root-workspace`), and the shell tests at
   the lines the plan lists; each change justified by a promise. Write a declaration of the shapes
   and run `python3 tools/vector_delta.py --declared FILE`: no undeclared change.
3. **New vectors** the plan names (about 14: Fleet routing and its retired case,
   ownership-permits-nothing, retire-conflict, retire-with-agents with its partial and authority
   cases, setup-registered, list pool and list fleet, partial-authority-no-count, bound-fleet-target,
   manages-agent-own-fleet), each citing its promises and listed in the manifest.

## Scope

Change: `engine/`, `conformance/`, and the declaration under `docs/work-orders/041b-declared.yaml`. Do not change `spec/` (report any defect you find as a finding) or a record. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py`, `python3 tools/check_rulings.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py`, `python3 -m unittest discover -s tools/tests` (with the
interpreter that has PyYAML), `python3 tools/vector_delta.py --declared docs/work-orders/041b-declared.yaml`
and `git diff --check` pass; in `engine/`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo test --release`; in
`experiments/011-short-form-spike/`, `cargo test --release`. Report the totals passed and failed;
tests that bind loopback ports may fail in your sandbox: say so, and the admin reruns them. Do not
stage anything.
