# Work order 041a (E3a): root's defaults, Agents' owners, and the vectors' migration

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file or fixture: use
`head`, `tail`, `grep` or a count.

## What and why

The third build order of spec batch 1 ([the plan](../proposals/spec-batch-1/README.md), its order
041a, E3a). Every registered Pool and Fleet gets root's default Conduit (FLEET-005, ruling 59 A3);
an Agent's owner is required and is its Fleet (FLEET-002 to 004, ruling 59 A1); only root registers
Agents (ACC-057). Because nearly every vector registers S1 or its own Pools and Agents, about 170
case files change in known shapes; this order changes them by a scripted transform whose every
change `tools/vector_delta.py` reports, so that no expectation is regenerated from the engine. Read
the plan's order 041a and the conflicts that touch it, the promises (FLEET-002 to 005, ACC-057,
OBJ-013), the staged edits in `docs/work-orders/036b-staged/041a.md` and `041a.patch`, the
vector-impact lists in `036b-staged/` (`vector-impact.md`, `vector-impact-038a.md`), and order 036a's
list of contradicted cases in its record.

## What to do

1. **Build** as the plan states: FLEET-005's defaults for every registered Pool and Fleet, their
   objects and records allocated after every object and record of the submission; FLEET-002 to 004,
   the Agent's required owner; ACC-057; the runner as needed. Every decision cites its promise.
2. **Land the staged edits**: apply `036b-staged/041a.patch` exactly (`git apply`), then delete
   `041a.patch` and `041a.md`.
3. **The migration**, a transform in `tools/` with a unittest, that inserts only the known shapes:
   Fleet `s1` (last in the fixture), the Agents' owners, the defaults' objects and records,
   renumbered register-step IDs, and grown evidence lists; run it over the case files the plan names
   (about 109 S1 registrations, 15 cases with their own Pools, 43 with their own Agents, and first
   start). Write a declaration of exactly these shapes and run `python3 tools/vector_delta.py
   --declared FILE`: it must show no undeclared change. Report any case the transform cannot handle,
   and fix it by hand only where a promise decides the new expectation, saying which.
4. **Vectors.** The new ones the plan names (defaults pool, defaults fleet, close-refused,
   root-closes-own, agent-owner, agent-owner-retired, register-agent-non-root, defaults-ids-last),
   each citing its promises and listed in the manifest.
5. **Experiment 011.** Update its translation so that S1's Agents name their Fleet, and run it.

## Scope

Change: `engine/`, `conformance/`, `tools/` (the transform and its test), `experiments/011-short-form-spike/`,
the staged files, and the declaration under `docs/work-orders/041a-declared.yaml`. Do not change
`spec/` (report any defect you find as a finding) or a record. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py`, `python3 tools/check_rulings.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py`, `python3 -m unittest discover -s tools/tests` (with the
interpreter that has PyYAML), `python3 tools/vector_delta.py --declared docs/work-orders/041a-declared.yaml`
and `git diff --check` pass; in `engine/`, `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo test --release`; in
`experiments/011-short-form-spike/`, `cargo test --release`. Report the totals passed and failed;
tests that bind loopback ports may fail in your sandbox: say so, and the admin reruns them. Do not
stage anything.

## Record

Run on 2026-11-11: the worker in 138 minutes, then a fresh verifier in 9, which found nothing to fix
in the build. The engine gives root a default Conduit to every registered Pool and Fleet
(FLEET-005), allocated after the submission's own objects and records; an Agent's owner is
required and is its Fleet (FLEET-002 to 004); only root registers Agents (ACC-057). The staged
patch of 036b landed. `tools/migrate_041a.py`, with its test, moved 167 vector cases to the new
shapes (Fleet `s1`, the Agents' owners, the defaults' objects and records, renumbered IDs, grown
evidence lists); `tools/vector_delta.py` showed 29,517 changes, every one of a declared shape and
none a flip, and the verifier reproduced the migration exactly. Eight vectors were added.
Experiment 011's translation names S1's Fleet; its fourteen states translate exactly. The admin
repointed two lines of IN_FORCE.md and the vector count. Tests: the engine's suite, 172, with 413
vectors; experiment 011, 9.
