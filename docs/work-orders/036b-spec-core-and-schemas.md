# Work order 036b: spec batch 1, the core package and schemas for 036a

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

The second spec order of [the batch 1 plan](../proposals/spec-batch-1/README.md), SPEC-A2: the core
package's operations, capabilities, templates and types, and the document schemas, that the
promises order 036a wrote need. The engine reads `spec/core/` and `spec/schemas/` at run time, so
this order lands only the edits that keep the engine's tests green and stages the rest verbatim
for the build orders that will land them (the plan's conflict C25).

Read, in this order: the plan's sections "What batch 1 changes", "The order of the edits" (this
order's part and every numbered conflict C1 onward that touches what you write) and "The work
orders" (this order); the area drafts it names; the text order 036a wrote; then the records of the
rulings involved. The plan's drafts are drafts: write each item in the specification's form and
terms, with the meaning the plan gives it; where a draft and a ruling's record differ, the record
governs, and you report the difference. Each *spec pending* line of [IN_FORCE.md](../../IN_FORCE.md)
that this order writes in full becomes a pointer to what now holds it; a line written in part says
which part remains and which order writes it.
Its area drafts are [additions](../proposals/spec-batch-1/areas/additions.md),
[fleet](../proposals/spec-batch-1/areas/fleet.md) and [short-form](../proposals/spec-batch-1/areas/short-form.md)
(for `as` and owner branches only).

## What to do

1. **Write 036b as the plan's work-order section states it**: `fleet.v1`, `fleet-management.v1`,
   `list.v1`, `retire-with-agents.v1`, `lift.v1` (input `id`); the ten templates of conflict C8,
   without `answer` and `sign-in` (order 038a adds them); the schemas inventory (with scope and
   administrators), tombstone (C18), provenance, fleet-reference and trust-domain; the Fleet, `as`
   and owner branches of `instance.v1`, `template.v1` and `type-spec.v1`; `expectedId` on open.v1
   and close.v1; the first-start example; and the contracts the plan names as changed.
2. **Staging (C25).** What the engine would reject today goes, verbatim and in final form, into
   `docs/work-orders/036b-staged/<order>.md`, one file per build order that lands it (039, 040,
   041a, 049a, 049b), each edit with its target file and the exact text. Then apply all staged
   text in a scratch copy of the tree (outside the repository), run `tools/check_schemas.py` and
   `tools/check_docs.py` there, and report both last lines.
3. **Citations.** Every promise ID a contract or schema cites must exist after this order
   (check_schemas step 5).

## Scope

Change: `spec/core/`, `spec/schemas/`, `spec/README.md` and `spec/core/README.md` where they list
what changed, `IN_FORCE.md`'s lines for what you write, and create `docs/work-orders/036b-staged/`.
Do not change promise files except a citation the plan names, `engine/`, `conformance/` or a
record. Report anything else.

## Done when

Every item the plan assigns to this order is written, or its absence is reported with the reason.
From the repository root: `python3 tools/check_docs.py`, `python3 tools/check_rulings.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py`, `python3 -m unittest discover -s tools/tests` (with the
interpreter that has PyYAML) and `git diff --check` pass; in `engine/`, `cargo test --release`
(report the total passed and failed; tests that bind loopback ports may fail in your sandbox: say
so). Report what you added and amended, by ID, every reading of the plan you applied, and every
vector whose expectation the new text contradicts, by path and step. Do not stage anything.
