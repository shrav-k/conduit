# Work order 036a: spec batch 1, the Fleet family and the access additions

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

The owner's rulings 53, 55 (A7), 59, 60, 62 (C1, C2), 66 (point 4) and 54 (points 1 and 3) are
accepted and not yet written into the specification: [IN_FORCE.md](../../IN_FORCE.md) marks them
*spec pending, batch 1*. [The batch 1 plan](../proposals/spec-batch-1/README.md) turns them into
promise changes and splits the writing into five spec orders. This is the first, SPEC-A1: the
structure and access promises that every later order cites. Read, in this order: the plan's
sections "What batch 1 changes", "The order of the edits" (036a's internal order and every
numbered conflict C1 onward that touches what you write), and "The work orders" (036a); then the
area drafts [additions](../proposals/spec-batch-1/areas/additions.md) and
[fleet](../proposals/spec-batch-1/areas/fleet.md), which hold the draft text of each new promise;
then the records of the rulings named above.

## What to do

1. **Write 036a as the plan states it**, in its internal order: the rows of `spec/definitions.md`;
   the objects and kinds (OBJ-001, 002, 013, 020 with `as`, `owner` and Fleet; FLEET-001 to 006;
   CON-024, CON-027, CTR-004, TPL-001, TPL-002, PLC-006); access (ACC-024; ACC-033 written whole
   with the rows the plan lists; ACC-044, 053, 054, 055, 057; ACC-005, 006, 012, 022, 029; OPS-008,
   014, 019, 029 and OPS-036; REV-010, REV-014); the lifecycle (LIF-040, LIF-041 and the amended
   LIF, CON, REC, FST, PRV and DL entries); the interface (IFC-012, 014, 025, 033); and
   `spec/README.md`'s contents and limits. New promise files follow the house form; `spec/README.md`
   lists any new file.
2. **The plan's drafts are drafts.** Write each promise in the specification's form and terms
   (seven fields, the writing rules in `CONTRIBUTING.md`, the words of `spec/definitions.md`), with
   the meaning the plan gives it. Where a draft and a ruling's record differ, the record governs;
   report the difference. Where the plan applies a reading the owner may strike, write it and list
   it in your report.
3. **IDs.** Use the IDs of the plan's C1. Check each against every head comment's register of
   removed IDs and `spec/README.md`'s list before using it; `tools/check_schemas.py` fails a reused
   ID.
4. **What this order does not write.** The core package's contracts and schemas (order 036b), the
   short form (037), refusals, pending actions and sign-in (038a), enrollment (038b), and anything
   of ruling 61 (batch 2). Where a promise here must cite one of them, write the citation the plan
   gives and list it; if `tools/check_schemas.py` then fails because the cited contract does not
   exist yet, stage that sentence as the plan's C25 describes and report it.
5. **IN_FORCE.md.** Each *spec pending* line this order writes in full becomes a pointer to the
   promises that now hold it, in that file's form; a line written only in part says which part
   remains and which order writes it.
6. **The vectors and the engine do not change in this order.** Run the engine's suite to show it
   still passes on the changed text, and report every vector whose expectation the new text
   contradicts (by path and step), for the build orders.

## Scope

Change: `spec/` (promise files, `definitions.md`, `README.md`, `implementation-defined.yaml`,
`names.yaml` only if the plan asks), and `IN_FORCE.md`'s lines for what you write. Do not change
`spec/core/`, `spec/schemas/`, `engine/`, `conformance/` or any record. Report anything else.

## Done when

Every promise the plan assigns to 036a is written, or its absence is reported with the reason. From
the repository root: `python3 tools/check_docs.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py`, `python3 tools/check_rulings.py`,
`python3 -m unittest discover -s tools/tests` (with the interpreter that has PyYAML) and
`git diff --check` pass; in `engine/`, `cargo test --release` (report the total passed and failed;
tests that bind loopback ports may fail in your sandbox: say so). Report the promises added and
amended, by ID, and the lines of `spec/` before and after. Do not stage anything.
