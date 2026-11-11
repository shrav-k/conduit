# 015 — S4: partial setup, repeats and lost replies

> **For the owner**
> - **Accepted under ruling 75 D on 2026-11-11.** Every item labelled proposed below is accepted,
>   with the answers under "Open for the owner" applied.
>
> - Proposed P2: resolve uncertainty from evidence without guessing that an effect failed.
> - Proposed P3: retain completed setup parts and retry only eligible unfinished work.
> - Proposed P4: distinguish keyed retries from new work in all twelve D02 cells.
> - Proposed P5: preserve invocation truth through purge, expiry and changed authority.
> - Proposed M0: extend the first build's author measures and selected counting rules to S4.
> - Proposed M1: count the full D02 matrix, including intended second actions.
> - Proposed M2: count D10 conclusions, verification and next actions for each recovery case.
> - Proposed M3: count setup parts, attempts, effects and surviving relationships.
> - Proposed V1: `s4-partial-setup-retry` (safety).
> - Proposed V2: `s4-plain-partial-error-task` (safety).
> - Proposed V3: `s4-key-matrix` (safety for keyed cases).
> - Proposed V4: `s4-purge-expiry` (safety).
> - Proposed V5: `s4-owner-state-reconcile` (safety; preparation needs Q4).
> - Proposed V6: `s4-hook-late-onward` (safety).
> - Proposed R1: two separate author runs, with branch counts and optional replay rules below.
> - Q1: Accept the proposed pass conditions, measures, counting rules and vectors?
> - Q2: Who authors the runs, and how many author runs should each scenario have?
> - Q3: Which client should exercise cases that need real-client behavior: the owner's subscription, a second subscription account signed in through Conduit's own sign-in flow, or none (scripted clients only)?
> - Q4: How should E18's surviving Resource under a retired Pool be prepared?
> - Q5: How should a genuinely partial effect be handled while its reconciliation remains open?

## Status

Pre-registered on 2026-11-11 by work order 102. Accepted by the owner on 2026-11-11 under
[ruling 75](../../docs/proposals/ruling-75/README.md), D: its pass conditions, measures,
counting rules and planned vectors, with the answers under "Open for the owner" applied. Ruling
45 fixes what an experiment must show before it runs. No result is recorded here.

## Question

Can a setup that fails partway, is retried and loses a response be understood and recovered
without repeating an effect? [S4's roadmap row](../../ROADMAP.md#scenarios) covers setup parts
and the setup hook that replaced graphs ([ruling 41](../../docs/proposals/ruling-41/README.md),
item 1), the outcome model and recovery.

## What it must show

1. **P1 — ruled**, [ruling 47](../../docs/proposals/ruling-47/README.md#when-to-publish):
   S4 runs end to end on the reference engine with its vectors passing, including every
   safety vector it adds ([ruling 68, point 2](../../docs/proposals/ruling-68/README.md)). Judge
   by task transcripts, independent state checks and vector verdicts. Missing cases are
   missing evidence, never passes.
2. **P2 — proposed, for the owner's acceptance**: the user resolves uncertainty without guessing
   that an effect failed. This extends S1's accepted criterion in
   [topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement). For each unknown,
   judge M2: the author states known facts, checks independent evidence and either reconciles
   on that evidence or leaves it unknown. Timeout, missing outputs and model guesses prove nothing.
3. **P3 — proposed, for the owner's acceptance**: inspection and the external ledger agree on
   completed parts. No automatic rollback or hook redispatch occurs; explicit further setup can
   finish once blocked parts are resolved. Compare identities and dispatches at each boundary.
4. **P4 — proposed, for the owner's acceptance**: all twelve D02 cells have their specified
   semantics. Key-preserving retries apply effects at most once; unkeyed calls are new work.
   Intended second actions use fresh keys or none. Judge IDs, attempts and external effects.
5. **P5 — proposed, for the owner's acceptance**: purge/expiry preserve invocation truth;
   inspection/reconciliation follow ACC-033's current target. E20 reports the latest hook
   caller and its attribution limit. Judge M2/M3 and V4–V6; identify boundary-only E18 evidence.

## What it covers

[Round 6's roadmap allocation](../../ROADMAP.md#round-6s-evidence-and-measurements) assigns S4
E01, E09, E18, E19 and E20; D02 and D10. The
[critique](../../docs/proposals/reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage)
is evidence planning, not binding behavior. E03/E04/E10/E16 belong to S5.

Promise IDs, in today's files:

- [operations.yaml](../../spec/conventions/operations.yaml): OPS-001–014, OPS-018, OPS-021,
  OPS-022, OPS-024, OPS-028, OPS-030–034 (outcomes, repeats, hooks and reconciliation).
- [lifecycle.yaml](../../spec/conventions/lifecycle.yaml): LIF-003, LIF-004, LIF-013,
  LIF-017–021, LIF-027, LIF-028, LIF-035 (partial setup, owner and removal states).
- [access.yaml](../../spec/conventions/access.yaml): ACC-005, ACC-017–019, ACC-033, ACC-055;
  [records.yaml](../../spec/conventions/records.yaml): REC-001–006.
- [templates.yaml](../../spec/promises/templates.yaml): TPL-008, TPL-012, TPL-017;
  [pools.yaml](../../spec/promises/pools.yaml): POOL-013, POOL-015;
  [instantiation.yaml](../../spec/promises/instantiation.yaml): INS-013, INS-016.
- [contracts.yaml](../../spec/promises/contracts.yaml): CTR-002;
  [implementation.yaml](../../spec/interface/implementation.yaml): IFC-006, IFC-010,
  IFC-016, IFC-023, IFC-025, IFC-028; [pending.yaml](../../spec/promises/pending.yaml): PND-006.

Feeds A38/A39/A50/A68 and A6: A6's lost-response trace comes from S4's E09 runs, by the admin's
decision of 2026-11-11, logged for the owner to strike: from the real-client replay if Q3 chooses
one, and otherwise from the scripted runs. Overlaps A71 (client visibility), A65 (E20) and A14
(durable invocation storage). S4's row is its register row. Adjacent S1 traces are
[S1-unknown-write, S1-reconcile and S1-setup-sender](../../conformance/s1-safety-cases.md).

## The tasks

Preparation, supplied by the run's packages and harness, apart from author work:
a plain server writes the first of two external ledger entries and answers generic JSON-RPC
`-32021`, with no no-effect guarantee. Its task variant creates a task and ends failed after
that partial work. Exercise both an operation declaring an effect and one marked
`startsModelWork: true` with none. A lost-reply server completes an effect and drops once
before the engine receives it (unknown), and once after the engine records success but before
the client receives it (lost success). A hook server loses its reply, can independently prove
no effect for reconciliation as failed, then holds A's onward call until B's new attempt runs.
Also supplied: a non-plain deduplicating server, durable ledger, output-expiry controls and
barriers. Their construction belongs to the implementation order; faults are not author tasks.

1. **S4-T1:** Author a Pool with dependent entries and a setup hook. Request setup, with a
   required part blocked after earlier success; inspect surviving parts, repair the cause and
   ask for setup again. Explain outer and part outcomes.
2. **S4-T2:** Call E01 directly and through tasks; inspect unknown, repeat the key, and explain
   why the generic error does not establish no effect. Leave genuine partial effects unknown.
3. **S4-T3:** Perform all twelve D02 cells in M1. Verify external evidence, take permitted
   recovery actions, then deliberately request a second identical action as new work.
4. **S4-T4:** Purge records and expire outputs; repeat an old keyed success and inspect a
   separate unknown invocation. Explain OPS-034 conflict without asking for a new effect.
5. **S4-T5:** Inspect/reconcile with a live owner, management after owner retirement, and again
   after instance removal, with wrong-route probes at each state. Use Q4's boundary fixture
   only if accepted; also demonstrate the reachable public retirement/removal path.
6. **S4-T6:** Inspect the hook's lost reply; reconcile failed only after independent no-effect
   verification, ask for setup as B and inspect A's delayed onward call during B's attempt.

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

- **M1 — proposed, for the owner's acceptance**: D02 is 2 × 2 × 3: key-preserving/unkeyed client;
  plain/non-plain deduplicating implementation; lost success/unknown outcome/intended second
  identical action. Count one fresh trial per cell, twelve per author run. Lost success means
  the engine kept success; unknown means it got no conclusive reply. Record supplied key or
  absence, wire calls, invocation IDs, attempts, outcome/reason, outputs replayed or withheld
  and external effect counts. Expected extra unkeyed effects are new work, not keyed duplication.
- **M2 — proposed, for the owner's acceptance**: D10 has one row per unknown, conflict and
  partial setup: conclusion, unknown facts, verification source, authorized next action,
  actual action and result. Count correct explanations over all rows; list missing answers,
  unsupported assertions, unsafe retry suggestions and manual recovery steps. Preserve the
  author's explanation before revealing the harness ledger used to judge it.
- **M3 — proposed, for the owner's acceptance**: at each T1/T6 boundary count parts by
  outcome/reason, attempts per hook invocation, effects, surviving Conduits and runs with IDs.
  Attribute records to sender, caller and `serving`; compare A's late call with B's active
  attempt. Count package/fault preparation apart from author work.

## Vectors expected

Every row, its name and its safety classification is **proposed, for the owner's
acceptance**. Safety means failure could grant access, leak a secret, lose/repeat an effect
or report a wrong result under [the tracker](../../docs/AUDIT.md#how-to-use-this-document).
The safety promise column names the future required tag; it does not assert tagging exists.

| ID and proposed name | Promises cited; observation | Safety; safety promise |
| --- | --- | --- |
| V1 `s4-partial-setup-retry` | TPL-012/017, OPS-022, LIF-004/027, ACC-005; preserve parts, no rollback | yes; LIF-027 |
| V2 `s4-plain-partial-error-task` | IFC-010/016, OPS-012/018; partial errors stay unknown, no plain repeat | yes; OPS-018 |
| V3 `s4-key-matrix` | OPS-008–012/018, CTR-002; twelve subcases distinguish repeats/new actions | yes for keyed cases; OPS-018 |
| V4 `s4-purge-expiry` | OPS-008/013/034, REC-004; retained state prevents duplicate effects | yes; OPS-008 |
| V5 `s4-owner-state-reconcile` | ACC-033/055, OPS-014/030, LIF-017/035; current authority before/after removal | yes; ACC-033 |
| V6 `s4-hook-late-onward` | OPS-028, ACC-017–019, IFC-028, REC-002; latest caller in progress, refusal after end | yes; ACC-019 |

Existing slices: [unknown-write](../../conformance/conventions/requests/unknown-write/case.yaml),
[reconcile-true](../../conformance/conventions/requests/reconcile-true/case.yaml),
[sender-no-access](../../conformance/conventions/setup/sender-no-access/case.yaml),
[hook-unknown](../../conformance/conventions/setup/hook-unknown/case.yaml) and
[purge-succeeded](../../conformance/conventions/records/purge-succeeded/case.yaml).
They do not establish the whole matrix, real-client key behavior or E20.

## Runs

**Proposed, for the owner's acceptance (Q2)**: one coordinator author run and one fresh
Codex author run, with separate counts, each doing every task and listed branch once. The
fresh author receives the specification, tasks and CLI format reference; coordinator knowledge
is a limitation. A rerun after a fix is a new dated run retaining the earlier failure.
Pin engine, package and client versions; record authored files, commands, task times, mistakes,
all author requests and intervals without access, panel actions, fault/barrier events,
inspection and independent state snapshots, records and vector verdicts. Dated results go
beside this README. State unrun branches and deviations explicitly.

Use one local Unix machine, fresh stores, engine clients and scripted sending instances;
no autonomous Agent or model subscription by default. Reset between matrix cells/fault branches,
keeping each task's recovery sequence intact. Each author does T1–T6 and twelve matrix cells.
Packages: partial-effect/task server, deduplicating server, setup-hook fixture and independent
ledger reader, using the built-in local runner. No external service account is required.
E09's unchanged client key behavior and model recovery choices need a real-client replay. A6's lost-response trace
comes from that replay if Q3 chooses one, and otherwise from the scripted E09 runs.
[A71's results](../008-real-client/results-2026-11-09.md) showed Claude offering retry after
unknown and Codex labelling its item failed; neither showed actual retry or repeated effect.
Q3 chooses whether to add one trial per matrix cell with a real client. Record wire receipt,
client exposure and model's next action apart; unavailable exposure stays unshown. Owner time
is zero during scripted execution, apart from acceptance/review. For a replay reserve sign-in
and availability for prompts; record actual panel time, with no target.

## What it needs first

Engine support for setup parts/hooks (TPL-012/017, OPS-021/022/028), task failures
(IFC-010/016), deduplicating attempts (OPS-012), reconciliation/retention (OPS-008/014/034),
purge (REC-004) and current authority (ACC-033/055). [CONFORMANCE.md](../../engine/CONFORMANCE.md)
records existing slices and limits; do not assume task transport or cascades are implemented.
Implementation orders must qualify packages, fault barriers and vectors first. The owner
accepts Q1–Q5 and supplies an account/sign-in only if a replay is chosen. This order uses no network.

## What `spec/` does not yet decide

- **G1:** OPS-004/014/022 give no generic representation for reconciling only part of an effect
  as done; this overlaps [4-12](../../docs/proposals/first-build/log-increment-4.md). T2 leaves
  genuinely partial effects unknown. A contract's explicit repair must be specified before use.
- **G2:** E18's extant Resource under a retired owner has no demonstrated public preparation:
  LIF-020 blocks owner retirement before members' confirmed shutdown; LIF-035 removes confirmed
  members. ACC-033 defines a fallback if that state exists, not the transition making it.
  Q4 must distinguish a boundary seed from a reachable end-to-end case.
- **G3:** Safety vector field/tagging is *spec pending* in
  [IN_FORCE.md topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement).
  Names and classifications here do not fill that specification gap.

OPS-028's latest-caller attribution is decided, not a new gap; E20 preserves it for A65.

## Open for the owner

- **Q1:** Accept the proposed pass conditions, measures, counting rules and vectors? Options:
  accept, amend or defer. **Recommendation:** accept after the scenario's prerequisites are settled.
- **Q2:** Who authors the runs, and how many author runs should each scenario have? Options:
  coordinator alone, coordinator plus fresh Codex author, or another named plan.
  **Recommendation:** two separate author runs as above; this introduces no S1 numeric bar.
- **Q3:** Which client should exercise cases that need real-client behavior: the owner's subscription, a second subscription account signed in through Conduit's own sign-in flow, or none (scripted clients only)?
  **Recommendation:** second subscription replay for E09 through Conduit's sign-in flow,
  as [ruling 72](../../docs/proposals/ruling-72/README.md) chose for S3/S10. That ruling does
  not require this for S4. With no replay, A6's lost-response trace comes from the scripted E09 runs.
- **Q4:** How should E18's surviving Resource under a retired Pool be prepared? Options: first
  prove a legal public path, accept a boundary fixture plus the reachable removal path, or
  defer. **Recommendation:** boundary fixture and reachable path, labelled separately.
- **Q5:** How should a genuinely partial effect be handled while its reconciliation remains
  open? Options: keep it unknown and inspect, or first specify a contract's explicit repair.
  **Recommendation:** leave it unknown, without whole-effect assertions from partial evidence.

**Answers, under [ruling 75](../../docs/proposals/ruling-75/README.md), D (2026-11-11):**

- Q1: accepted.
- Q2: written twice, by the coordinator and by a fresh Codex author, with scripted clients (D3).
- Q3: one real-client run: E09's lost-response replay, on a second subscription account signed
  in through Conduit. It also gives A6 its real client's trace (D4). Where the runs speak of a
  trial per matrix cell, this one replay governs.
- Q4: a boundary fixture and the reachable path, labelled apart.
- Q5: a genuinely partial effect stays unknown, with no assertion about the whole effect.

## Limits

These local contracts/faults do not show arbitrary effect detection, every client's retry
behavior, partition tolerance or model checking (A40). Scripts do not qualify real clients;
boundary seeds do not prove public reachability. This pre-registration writes no result,
promise or vector.
