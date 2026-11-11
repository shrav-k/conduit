# Work order 038a: spec batch 1, refusals, pending actions and sign-in

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Rulings 49 (sign-in through the engine), 50 and 51 (the Interface as a control panel, one pending
action mechanism with expiry), 52 (refusals in two tiers, and the explicitness rule), 56 (evidence
shown by the engine, the word "answer") and 66 (point 3: pending actions at scoped Interfaces,
answers limited by holdings) are accepted and not yet written. This is SPEC-C1 of
[the batch 1 plan](../proposals/spec-batch-1/README.md).

Read, in this order: the plan's sections "What batch 1 changes", "The order of the edits" (this
order's part and every numbered conflict C1 onward that touches what you write) and "The work
orders" (this order); the area drafts it names; the text order 036a wrote; then the records of the
rulings involved. The plan's drafts are drafts: write each item in the specification's form and
terms, with the meaning the plan gives it; where a draft and a ruling's record differ, the record
governs, and you report the difference. Each *spec pending* line of [IN_FORCE.md](../../IN_FORCE.md)
that this order writes in full becomes a pointer to what now holds it; a line written in part says
which part remains and which order writes it.
Its area drafts are [pending-actions](../proposals/spec-batch-1/areas/pending-actions.md) and
[refusals-and-identity](../proposals/spec-batch-1/areas/refusals-and-identity.md). Read also
[the K1 check](../proposals/spec-batch-1/k1-mcp-elicitation.md) of the protocol: **where it differs
from the plan or a draft, K1 governs**, because the drafts assumed an earlier revision.

## What to do

1. **Write 038a as the plan's work-order section and "The order of the edits" state it**, in that
   internal order: refusals first (ACC-056, IFC-050 and the sentences the plan lists); the pending
   core (PND-001 to 004, PND-009, PND-012, IFC-049, with IFC-007, 010, 016 and 045); the kinds
   (sign-in, setup, notice) and expiry (DL19); the contracts and schemas (answer.v1, sign-in.v1,
   sign-in-hook.v1, account-hook.v1, sign-out-hook.v1, pending-action.v1, evidence-item.v1,
   refusal.v1, inspect.v1's `status.pending`), adding `answer` and `sign-in` to the capabilities and
   templates 036b wrote (C8); ACC-033's two new rows and ACC-056's disclosure sentence only (C14);
   ACC-056's row table last.
2. **K1's corrections**, applied throughout: no `elicitationId` and no completion notification; the
   engine's correlation lives in its own integrity-protected `requestState`, and a retry is a poll
   (block or answer another `InputRequiredResult` until the account appears or the DL18 wait ends;
   "no account yet" is not failed); URL mode is declared as `{"url": {}}`, and a caller that has not
   declared it gets `-32021` with `requiredCapabilities`, not invalid; a task's input requests use
   `tasks/get` and `tasks/update`; codes and fingerprints appear in no elicitation message, tool
   result, `_meta`, notification or record of a request, only in the engine's own panel and with the
   recorded answer (C5).
3. **Names.** `stateDirVariable` in `spec/names.yaml`, with `tools/set_names.py` taught it, as the
   plan says.
4. **`spec/README.md`'s limits**: ruling 51's decisions 1, 4, 6, 7, 9 and 10 and the network
   principle; peer mode named as waiting on the owner's question R3; A5 not gating the rerun.
5. **Staging.** Edits the engine would reject today are staged as order 036b staged them, in
   `docs/work-orders/036b-staged/<order>.md`, and checked in a scratch tree the same way.

## Scope

Change: `spec/`, `spec/names.yaml` and `tools/set_names.py` (the one name), the staged files, and
`IN_FORCE.md`'s lines for what you write. Do not change `engine/` code, `conformance/` or a record.
Report anything else.

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

Run on 2026-11-11: the worker in 32 minutes, then a fresh verifier in 11, then a second round. The
worker added 16 promises and amended 23: refusals in two tiers (ACC-056, IFC-050) and the
explicitness rule; the one pending-action mechanism (PND-001 to 004, 009, 012, IFC-049) with its
kinds sign-in, setup and notice and its expiry (DL19); sign-in through a person's Interface
(PND-005, 010, 011, CTR-013, PRV-029), with the K1 check of the protocol applied; the core's answer,
sign-in and hook contracts, and `stateDirVariable` in the names. It listed the vector steps the new
text contradicts for the build orders. The verifier found that setup's exhaustive list of parts left
no place for the account check, that sign-in.v1 gave two paths for an account found before the
person signs in, that ACC-056's table missed IMP-008's refusal, and that "hook" was defined for two
hooks only; the second round fixed all of them, with the counts. The admin reran the engine's
suite outside the sandbox: 163 tests pass, 3 ignored.
