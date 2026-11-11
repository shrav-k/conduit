# Work order 037: spec batch 1, the short form

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

The short form is part of the language (ruling 55): a sectioned document, defined by numbered
translation rules into today's canonical documents, which stay the kernel, the wire and the stored
form. Ruling 63 settled its documented style and package-shipped Conduit templates; rulings 59 (A6)
and 66 (point 1) added the `fleet:` and `manages:` keys; experiment 011 showed the translation works
once `as` exists. This is SPEC-B of [the batch 1 plan](../proposals/spec-batch-1/README.md).

Read, in this order: the plan's sections "What batch 1 changes", "The order of the edits" (this
order's part and every numbered conflict C1 onward that touches what you write) and "The work
orders" (this order); the area drafts it names; the text order 036a wrote; then the records of the
rulings involved. The plan's drafts are drafts: write each item in the specification's form and
terms, with the meaning the plan gives it; where a draft and a ruling's record differ, the record
governs, and you report the difference. Each *spec pending* line of [IN_FORCE.md](../../IN_FORCE.md)
that this order writes in full becomes a pointer to what now holds it; a line written in part says
which part remains and which order writes it.
Its area draft is [short-form](../proposals/spec-batch-1/areas/short-form.md); the grammar
is in `docs/proposals/language-shape/short-form/synthesis.md`, section "grammar", with ruling 55's
decisions applied, and experiment 011's translator and results show what worked.

## What to do

1. **Write 037 as the plan's work-order section states it**: SCT-001 to SCT-009 and SCT-011 to
   SCT-016 in a new `spec/promises/sections.yaml` ([SCT-010](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N4-minimal-normative.md#L140) stays unused), PKG-031; NAM-024 and the
   other amended promises the plan lists; `spec/schemas/sectioned.yaml`; flow-style examples
   (`valid/sectioned-project`, `valid/sectioned-package` and about twelve invalid ones) with the
   routing fixes of conflict C23 in `tools/check_schemas.py` and `engine/crates/loader/tests/examples.rs`
   so that the new examples are checked; the schemas README row.
2. **The keys.** `fleet:` works per file, at most one per file, owns every Agent in the file, and
   an Agent there naming another owner is invalid. SCT-012 expands `manages:` by consumer kind
   (conflicts C8 and C20): an Agent's `manages:` holds only its own Fleet.
3. **`spec/README.md`**: the reading path and its limits, plan and apply as tooling (the commands
   are the approval, with the named flags counted), and the documented flow style (ruling 63).

## Scope

Change: `spec/` (the promises, schemas and examples above, `definitions.md` rows the plan names,
`README.md`), the routing in `tools/check_schemas.py` and `engine/crates/loader/tests/examples.rs`,
and `IN_FORCE.md`'s lines for what you write. Do not change the engine's code otherwise,
`conformance/` or a record. Report anything else.

## Done when

Every item the plan assigns to this order is written, or its absence is reported with the reason.
From the repository root: `python3 tools/check_docs.py`, `python3 tools/check_rulings.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py`, `python3 -m unittest discover -s tools/tests` (with the
interpreter that has PyYAML) and `git diff --check` pass; in `engine/`, `cargo test --release`
(report the total passed and failed; tests that bind loopback ports may fail in your sandbox: say
so). Report what you added and amended, by ID, every reading of the plan you applied, and every
vector whose expectation the new text contradicts, by path and step. Do not stage anything.

## Record

Run on 2026-11-11: the worker in 32 minutes, then a fresh verifier in 10, then a second round. The
worker added SCT-001 to 009 and 011 to 016 in `spec/promises/sections.yaml` (SCT-010 unused) and
PKG-031, ten sectioned schemas, two flow-style example projects and twelve invalid examples, amended
twenty promises, NAM-024 among them, and routed the new examples through the checks. The verifier
found that the short form forbade duplicate operations the canonical form allows, that its "fully
qualified" rule left a project's own contracts with no accepted spelling under NAM-022 (the
worker's finding 037-04), that the example's Agent configurations did not match the `.v2` types, and
two gaps in `spec/README.md`. The admin decided, strikeable by the owner, that ruling 55 A2's "fully
qualified" means never an alias or a bare name, so the short form accepts exactly the canonical
form's spellings, `#name` for a project's own contracts included; the second round applied that and
fixed the rest. Translation itself, its vectors and export resolution belong to the build orders
(finding 037-05). The admin reran the engine's suite outside the sandbox: 163 tests pass, 3 ignored.
