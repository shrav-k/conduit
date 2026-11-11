# 016 — S5: a control change under contention

> **For the owner**
>
> - Proposed P2: conflicting changes neither queue nor overwrite a newer revision.
> - Proposed P3: a control applies exactly its supplied values or does not succeed.
> - Proposed P4: discovery, validation and normalized repeats agree for E03/E04/E10/E16.
> - Proposed P5: lost receipts and reconciliation remain distinguishable from failure.
> - Proposed M0: extend the first build's author measures and selected counting rules to S5.
> - Proposed M1: count D10 conflict diagnoses using ruleId, invocation identity and status.
> - Proposed M2: count dispatches, settings, revisions and default-normalization cases.
> - Proposed V1: `s5-control-overlap` (safety).
> - Proposed V2: `s5-control-contention` (safety).
> - Proposed V3: `s5-control-discovery-defaults` (safety).
> - Proposed V4: `s5-control-lost-reconcile` (safety).
> - Proposed V5: `s5-conflict-explanation` (safety).
> - Proposed R1: two separate author runs, with branch counts and optional replay rules below.
> - Q1: Accept the proposed pass conditions, measures, counting rules and vectors?
> - Q2: Who authors the runs, and how many author runs should each scenario have?
> - Q3: Which client should exercise cases that need real-client behavior: the owner's subscription, a second subscription account signed in through Conduit's own sign-in flow, or none (scripted clients only)?
> - Q4: Which supplied control contract and verification operation should S5 qualify?

## Status

Pre-registered on 2026-11-11 by work order 102. Awaits the owner's acceptance of
what it proposes; nothing here runs before that. Ruling 45 fixes what an experiment must
show before it runs, and IN_FORCE.md topic 13 lists the criteria the owner has accepted.
No result is recorded here.

## Question

Can two actors change a live setting under contention without overwriting another change,
and recover from a lost receipt? [S5's roadmap row](../../ROADMAP.md#scenarios) exercises revisions,
a control that applies exactly what it was sent or fails ([ruling 41](../../docs/proposals/ruling-41/README.md),
item 15, replacing partial confirmation), and a lost receipt.

## What it must show

1. **P1 — ruled**, [ruling 47](../../docs/proposals/ruling-47/README.md#when-to-publish):
   S5 runs end to end on the reference engine with its vectors passing, including every
   safety vector it adds ([ruling 68, point 2](../../docs/proposals/ruling-68/README.md)). Judge
   by task transcripts, independent state checks and vector verdicts. Missing cases are
   missing evidence, never passes.
2. **P2 — proposed, for the owner's acceptance**: only one setup/retirement/change of an instance
   is dispatched at a time; a competing change conflicts immediately without queuing. After a
   changing success, the old revision conflicts without dispatch. Judge barrier logs, stored
   values and opaque revisions, including a no-op success and a change back to an earlier value.
3. **P3 — proposed, for the owner's acceptance**: successful controls apply exactly the supplied
   values; omitted inputs leave settings alone. Failed or unknown controls do not change stored
   configuration/revision; unknown settings are labelled unknown. Compare implementation state,
   sent arguments and engine inspection; a partial physical effect cannot be reported failed.
4. **P4 — proposed, for the owner's acceptance**: E03/E04 reject equal/ancestor `changes`
   pointers; E10 shows the narrowed schema, rejects 75 under a max-50 template and omits an
   excluded control tool. E16 uses one normalized value for repeats. Judge rejection diagnostics,
   tool schemas and per-case dispatches/settings, including the root-$ref default exception.
5. **P5 — proposed, for the owner's acceptance**: after lost success or unknown, the actor
   distinguishes the conflict causes in M1 and chooses an evidence-based next step. A control
   reconciled occurred uses the current revision and the original sent values; a stale revision
   conflicts, invalid resulting configuration stays unknown, and retirement/removal stores
   nothing. Judge records, external verification and before/after configuration.

## What it covers

[Round 6's allocation](../../ROADMAP.md#round-6s-evidence-and-measurements): E03/E04 (equal and
ancestor pointer overlap), E10 (narrowed/excluded controls), E16 (default normalization); D10
(frontend distinctions). The [critique's cases](../../docs/proposals/reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage)
use older promise references; the following are today's IDs:

- [revisions.yaml](../../spec/conventions/revisions.yaml): REV-001–007, REV-009–015.
- [operations.yaml](../../spec/conventions/operations.yaml): OPS-003, OPS-004, OPS-008–014,
  OPS-020, OPS-023, OPS-026, OPS-030–034; [records.yaml](../../spec/conventions/records.yaml): REC-001–003.
- [contracts.yaml](../../spec/promises/contracts.yaml): CTR-002, CTR-006, CTR-012;
  [templates.yaml](../../spec/promises/templates.yaml): TPL-010, TPL-011, TPL-013.
- [schemas.yaml](../../spec/promises/schemas.yaml): SCH-007–011;
  [instantiation.yaml](../../spec/promises/instantiation.yaml): INS-013, INS-014.
- [access.yaml](../../spec/conventions/access.yaml): ACC-033, ACC-055;
  [lifecycle.yaml](../../spec/conventions/lifecycle.yaml): LIF-013, LIF-015, LIF-035;
  [implementation.yaml](../../spec/interface/implementation.yaml): IFC-012, IFC-025, IFC-033, IFC-050.

Feeds A38/A39/A50/A68 and overlaps S4's D10 recovery, A14's retained control values and A71's
client visibility. S5's row is its register row.
S1's [S1-reconcile](../../conformance/s1-safety-cases.md) borders this work but does not show controls.

## The tasks

Preparation, apart from author work: supply a test Resource Type with two disjoint settings,
a control declaring a receipt output, external-state readback and a cross-setting constraint. The package
can hold a call at a barrier, succeed exactly, report failed with independently confirmed no
effect, or lose a reply. Drop a reply separately before the engine (unknown) and after it
records success (lost receipt). Malformed contracts and schema/default inputs are supplied
fixtures; the harness expires the receipt for OPS-034. Fault controls and barriers belong to the harness, not to the measured author.

1. **S5-T1:** Author the Resource and two actors' control/inspection routes. Inspect one revision,
   ask for one change, and while it is held ask for another with the same revision. After
   completion, ask with the old revision, then inspect and explicitly choose fresh work.
2. **S5-T2:** Change one setting, omit another, make a no-op change, then change a value back.
   Explain the revisions and verify exact values, including failed and unknown branches.
3. **S5-T3:** Submit E03's equal pointers and E04's ancestor pointers in both map orders.
   Inspect E10's max-50 tool under its max-100 contract, request 75, then use a separate
   template/instance whose controls map excludes it; test discovery and a direct call.
4. **S5-T4:** Exercise E16: omitted input, explicit null, empty map, nested default, array-map
   element, omitted control input and an ordinary operation's root-$ref-only default. Compare omitted/explicit
   default repeats with the same key, and changed normalized contents with that key.
5. **S5-T5:** Recover a lost success receipt and an unknown control. Verify externally before
   reconciliation; make a later change, then reconcile the original with current/stale/missing
   revision, a schema-invalid merge and `occurred: false` with/without revision in reset branches.
   Reconcile separate unknown controls after retirement and removal; explain why nothing stores.
6. **S5-T6:** Explain every M1 conflict from permitted diagnostics/status, and state what can
   be verified and done next, without treating conflict as proof of failure or missing effect.

## Measures

**M0 — proposed, for the owner's acceptance**, lane coordinator's
[decision 4](../../docs/work-orders/102-pre-b-s4-s5-s6-s9.md#the-decisions-it-applies): carry over
by reference the first build's [measure table](../../docs/implementation/FIRST_BUILD.md#measures)
and three [counting rules](../../docs/implementation/FIRST_BUILD.md#counting-rules-added-after-the-baseline-ran):
“Provider work apart”, “A client's concepts count” and “Requests and time without access”.
**Ruled counting rules**: [ruling 63](../../docs/proposals/ruling-63/README.md)'s lines,
[ruling 68 R1](../../docs/proposals/ruling-68/README.md)'s panel actions apart, and
[IN_FORCE.md topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement)'s
per-run rules, except S1's local name `project`. S1's A70 comparison/bar, T3's reach-around rule
and T4/T5's better-baseline rules do not carry over. No numeric bar or comparison is proposed;
observed time and mistakes precede any target (ruling 46, point 3).

- **M1 — proposed, for the owner's acceptance**: D10 has five conflict classes: REV-014 busy,
  REV-002 stale revision, OPS-010 changed contents under one key, OPS-032 repeat in progress,
  OPS-034 success without replayable outputs. Count one diagnosis per class per author run;
  record `ruleId`, invocation ID when promised, permitted status, conclusion, verification
  source and authorized next action. Count correctly distinguished classes over five, and
  missing/mistaken diagnoses apart. Do not demand an invocation ID for a new rejected request.
- **M2 — proposed, for the owner's acceptance**: for every task branch record barrier order,
  call/attempt counts, physical settings, stored settings, unknown markers and revision
  equality before/after. Count E03/E04 rejections, E10 schema/discovery/call verdicts and each
  of E16's seven input cases separately; compare normalized arguments for each keyed pair.
  A failed branch is retained in the denominator; no automatic retry is counted as author work.
  Record provider preparation separately under M0.

## Vectors expected

Every row, its name and its safety classification is **proposed, for the owner's
acceptance**. Safety means failure could grant access, leak a secret, lose/repeat an effect
or report a wrong result under [the tracker](../../docs/AUDIT.md#how-to-use-this-document).
The safety promise column names the future required tag; it does not assert tagging exists.

| ID and proposed name | Promises cited; observation | Safety; safety promise |
| --- | --- | --- |
| V1 `s5-control-overlap` | CTR-006; E03/E04 reject equal/ancestor pointers in either map order | yes; CTR-006 |
| V2 `s5-control-contention` | REV-001–007/011/012/014/015; exact values, one dispatch, stale/busy/no-op/back-change branches | yes; REV-012 |
| V3 `s5-control-discovery-defaults` | TPL-010/011/013, IFC-012/033, SCH-007–011, OPS-009/010/026, CTR-012, REV-006; E10/E16 | yes; REV-006 |
| V4 `s5-control-lost-reconcile` | REV-009/010/013, OPS-008/014/030/031, ACC-033/055; original values/current revision, no store after retirement/removal | yes; REV-010 |
| V5 `s5-conflict-explanation` | REV-002/014, OPS-010/032/034, IFC-025/050; distinct ruleIds/identities/status without false failure claims | yes; IFC-025 |

Existing slices: [e05-control](../../conformance/interface/schemas/e05-control/case.yaml),
[e05-control-absent-template](../../conformance/interface/schemas/e05-control-absent-template/case.yaml),
[requests-defaults](../../conformance/conventions/requests/defaults/case.yaml),
[schema-root-ref-default](../../conformance/interface/schemas/schema-root-ref-default/case.yaml)
and [reconcile-revision-in-progress](../../conformance/conventions/requests/reconcile-revision-in-progress/case.yaml).
These do not qualify live control dispatch or control reconciliation.

## Runs

**Proposed, for the owner's acceptance (Q2)**: one coordinator author run and one fresh
Codex author run, with separate counts, each doing every task and listed branch once. The
fresh author receives the specification, tasks and CLI format reference; coordinator knowledge
is a limitation. A rerun after a fix is a new dated run retaining the earlier failure.
Pin engine, package and client versions; record authored files, commands, task times, mistakes,
all author requests and intervals without access, panel actions, fault/barrier events,
inspection and independent state snapshots, records and vector verdicts. Dated results go
beside this README. State unrun branches and deviations explicitly.

One local Unix machine, fresh engine stores and the supplied control/readback package; two
scripted actors with engine clients are sufficient. No model Agent, subscription, external
service or second machine is needed. Each author does T1–T6, including five M1 classes, seven
E16 input cases and every T5 reconciliation branch; reset branches without resetting a live
contention sequence. Default frontend is the engine's CLI, with wire evidence kept apart.
No S5 case requires a real model client: scripts can show the D10 protocol facts and a person
can explain them. Q3 leaves additional unchanged-client presentation behavior to A71's evidence unless
the owner chooses a replay. Owner presence/time: acceptance and results review, no synchronous
scripted-run presence; any optional subscription replay adds sign-in and prompt time recorded
apart. Record selected client, accounts by role and protocol version without credentials.

## What it needs first

[Engine conformance](../../engine/CONFORMANCE.md#consumer-server-order-022) says controls and
control reconciliation remain outside its implemented slice. They must exist for OPS-020,
REV-006/007/009/010/013/014/015 and ACC-033/055, including retained original values and unknown
markers. Qualify tool narrowing, schema normalization, barrier execution and external readback
through the chosen package. Supply author-readable permitted diagnostics/status for M1, and
implement the pending safety field/tagging before claiming the safety gate. The owner accepts
Q1–Q4; supplies account/sign-in only if an optional replay is selected.

## What `spec/` does not yet decide

- **G3:** Safety vector field/tagging is *spec pending* in
  [IN_FORCE.md topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement).
  Names and classifications here do not fill that specification gap.
- No additional missing control/reconciliation rule is established here: REV-010 already
  specifies original values applied to the current configuration, revision requirements,
  authorization, and retirement/removal behavior; REV-013 decides an invalid merge. The engine's
  missing implementation is not a new language rule. Do not replace these with a new repair API.
- A generic frontend layout and a generic operation that verifies any provider's physical
  settings are not specified. Q4 selects a package's contract-specific readback; no task assumes
  generic automatic reconciliation. Late-result policy remains the engine's DL2 choice,
  documented in [CONFORMANCE.md](../../engine/CONFORMANCE.md#implemented-choices).

## Open for the owner

- **Q1:** Accept the proposed pass conditions, measures, counting rules and vectors? Options:
  accept, amend or defer. **Recommendation:** accept after the scenario's prerequisites are settled.
- **Q2:** Who authors the runs, and how many author runs should each scenario have? Options:
  coordinator alone, coordinator plus fresh Codex author, or another named plan.
  **Recommendation:** two separate author runs as above; this introduces no S1 numeric bar.
- **Q3:** Which client should exercise cases that need real-client behavior: the owner's subscription, a second subscription account signed in through Conduit's own sign-in flow, or none (scripted clients only)?
  **Recommendation:** none: use scripted clients and
  the engine CLI here. If a replay is chosen, a second account signs in through Conduit; [ruling 72](../../docs/proposals/ruling-72/README.md)
  requires that for S3/S10, not S5. Freeze the replay's client and trial count before running.
- **Q4:** Which supplied control contract and verification operation should S5 qualify? Options:
  a packaged local two-setting fixture, a real service with an existing suitable contract, or
  defer until one exists. **Recommendation:** packaged local fixture with explicit readback,
  counted as provider work; it exercises the convention without inventing a generic repair.

## Limits

No numeric usability bar, arbitrary control semantics, generic effect detection, real-client
metadata visibility or distributed locking is shown. A scripted race is a deterministic
interleaving, not a contention-load benchmark. Physical settings are judged only under the
selected contract. No run, result, new promise or vector is produced by this paper order.
