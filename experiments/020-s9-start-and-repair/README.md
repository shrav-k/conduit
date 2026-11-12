# 020 — S9: first initialization, later start and explicit repair

> **For the owner**
> - **Accepted under ruling 75 D on 2026-11-11.** Every item labelled proposed below is accepted,
>   with the answers under "Open for the owner" applied.
>
> - Proposed P2: first initialization makes exactly the fixed ready foundation and trust domain.
> - Proposed P3: later/repeated starts preserve identity and interrupted-work uncertainty.
> - Proposed P4: failed/unknown setup stays inspectable; reconciliation precedes retry of an unknown hook.
> - Proposed P5: repair cannot recreate retired authored objects or bypass foundational protections.
> - Proposed M0: extend the first build's author measures and selected counting rules to S9.
> - Proposed M1: count foundational objects, identities and calls across start boundaries.
> - Proposed M2: count diagnosis and explicit repair steps, with panel actions apart.
> - Proposed V1: `s9-first-initialize` (safety).
> - Proposed V2: `s9-later-start` (safety).
> - Proposed V3: `s9-interrupted-initialize` (safety, reference-engine policy).
> - Proposed V4: `s9-explicit-repair` (safety).
> - Proposed V5: `s9-protected-foundation` (safety).
> - Proposed R1: two separate author runs, with branch counts and optional replay rules below.
> - Q1: Accept the proposed pass conditions, measures, counting rules and vectors?
> - Q2: Who authors the runs, and how many author runs should each scenario have?
> - Q3: Which client should exercise cases that need real-client behavior: the owner's subscription, a second subscription account signed in through Conduit's own sign-in flow, or none (scripted clients only)?
> - Q4: Which repair scope should S9 exercise before any additional DL6 policy is chosen?

## Status

Pre-registered on 2026-11-11 by work order 102. Accepted by the owner on 2026-11-11 under
[ruling 75](../../docs/proposals/ruling-75/README.md), D: its pass conditions, measures,
counting rules and planned vectors, with the answers under "Open for the owner" applied. Ruling
45 fixes what an experiment must show before it runs. No result is recorded here.

Its order-level plan is [plan.md](plan.md), written by work order 118; the plan changes nothing accepted here.

## Question

How does an engine first initialize, start later and support explicit repair while keeping
its root Interface and foundational setup correct? This is
[S9's roadmap row](../../ROADMAP.md#scenarios). It distinguishes the fixed foundation from
later authored instances and from engine-specific recovery policy.

## What it must show

1. **P1 — ruled**, [ruling 47](../../docs/proposals/ruling-47/README.md#when-to-publish):
   S9 runs end to end on the reference engine with its vectors passing, including every
   safety vector it adds ([ruling 68, point 2](../../docs/proposals/ruling-68/README.md)). Judge
   by task transcripts, independent state checks and vector verdicts. Missing cases are
   missing evidence, never passes.
2. **P2 — proposed, for the owner's acceptance**: empty-store initialization creates exactly
   the five first-start objects and three root declared open Conduits, with their core-template
   provenance, all ready, and no calls to other implementations. The trust domain is valid and
   fixed; malformed configuration is rejected. Judge privileged inspection, records and dispatch logs.
3. **P3 — proposed, for the owner's acceptance**: repeated initialization and later starts
   preserve foundational IDs and trust domain; they do not create extra objects or erase
   invocation state. Interrupted effectful work stays unknown without implicit redispatch.
   Judge snapshots and effect ledgers against the engine's documented DL6/DL2 policy.
4. **P4 — proposed, for the owner's acceptance**: a failed/unknown authored setup remains
   inspectable after startup, with its setup pending action and parts. The author explains
   what needs repair, independently verifies and explicitly reconciles an unknown hook under
   OPS-014 before requesting further setup, performs permitted repair and verifies readiness. Judge both
   branches by inspection, PND-006 evidence, records and external checks; restart alone proves no repair.
5. **P5 — proposed, for the owner's acceptance**: startup/repair do not recreate closed/retired
   authored instances or replace their IDs implicitly. First-start objects cannot be submitted
   as ordinary instances, retired, unregistered or local declared lost. Judge rejection
   diagnostics, unchanged foundation and explicit replacement/tombstone steps when applicable.

## What it covers

S9 has no row of assigned E-cases or D-measures in
[round 6's evidence table](../../ROADMAP.md#round-6s-evidence-and-measurements). It feeds
A38/A39/A50/A68 and overlaps S4's partial setup and S6's lifecycle boundaries. A71 is only
adjacent if optional real-client behavior is requested. S9's row is its register row.

Promise IDs, by today's files:

- [first-start.yaml](../../spec/promises/first-start.yaml): FST-001–003, FST-005–007.
- [lifecycle.yaml](../../spec/conventions/lifecycle.yaml): LIF-003, LIF-004, LIF-008, LIF-009,
  LIF-013, LIF-022, LIF-027, LIF-028, LIF-033, LIF-034, LIF-040, LIF-041.
- [operations.yaml](../../spec/conventions/operations.yaml): OPS-008, OPS-012–014, OPS-018,
  OPS-021, OPS-022, OPS-028; [records.yaml](../../spec/conventions/records.yaml): REC-001–004.
- [access.yaml](../../spec/conventions/access.yaml): ACC-005, ACC-024, ACC-033, ACC-053;
  [fleets.yaml](../../spec/promises/fleets.yaml): FLEET-005 (root's later defaults).
- [instantiation.yaml](../../spec/promises/instantiation.yaml): INS-013, INS-016;
  [templates.yaml](../../spec/promises/templates.yaml): TPL-012, TPL-017;
  [pools.yaml](../../spec/promises/pools.yaml): POOL-013, POOL-015.
- [pending.yaml](../../spec/promises/pending.yaml): PND-001–004, PND-006, PND-009, PND-012;
  [implementation.yaml](../../spec/interface/implementation.yaml): IFC-025, IFC-028.

[DL6](../../spec/implementation-defined.yaml) delegates repeated/interrupted initialization
and repair policy to each engine; the promises above still constrain every policy.
[S1-setup-sender, S1-unknown-write and S1-reconcile](../../conformance/s1-safety-cases.md)
are adjacent slices, not the whole S9 run.

## The tasks

Preparation, apart from author work: fresh stores, a local package whose setup hook reports
failed with no effect until an explicit repair of its backing precondition, a separate ready
repair Resource with a narrowly stated operation to establish that precondition, independent
readback, and a lost-reply fixture. The harness supplies stop/restart and before/after durable
initialization-commit faults; their implementation follows the engine's stated atomic policy.
For T4's separate unknown branch, the fixture loses the failed hook's reply after it has ended
without an effect; independent readback exposes effect absence and confirms no hook work remains.
Corrupt-store or missing-foundation seeds are not silently used as ordinary authored states.
The package contract must define the repair effect; it is not a new engine repair operation.

1. **S9-T1:** Initialize an empty store, inspect root and every foundational object/Conduit,
   explain their core provenance and record the trust domain. Compare configured and generated
   domains in separate fresh labs, and reject malformed configured domains.
2. **S9-T2:** Author a small Pool/Resource setup and one keyed effect. Start later with the same
   store and repeat initialization. Verify identity, declared state, retained records and that
   successful hooks/effects are not repeated; compare a separately interrupted unknown call.
3. **S9-T3:** Inspect the outcomes of the supplied before/after-commit initialization faults,
   then explicitly restart under the documented DL6 policy. Explain intact empty versus committed
   state and any interruption evidence; never guess a half-initialized foundation was repaired.
4. **S9-T4:** In separate stores, restart with (a) the failed hook and (b) the lost-reply unknown
   hook. Inspect setup parts, pending action and hook invocation ID. In (a), the engine's startup
   requests the outstanding setup again (`serve`, engine/README.md), which dispatches the failed
   hook once as a new attempt; it fails again before the repair. Record it and its outcome. In (b), verify
   that restart made no further hook call (OPS-021), independently verify effect absence and
   that hook work has ended, then use an open Conduit listing `reconcile.v1` to the owning Pool (ACC-033) to reconcile the
   hook as failed under OPS-014, recording what was checked. Verify the retained unknown record
   and reconciliation. In each branch, identify the backing precondition, explicitly call the
   permitted contract-specific repair, then request setup again. Verify readiness, retained
   successful parts and exactly one hook attempt after the repair, after reconciliation in (b).
5. **S9-T5:** Close an authored declared Conduit, restart and verify no recreation. Explicitly
   lift its tombstone when authorized, register its replacement and set up/open it; verify its
   new ID. Try submitting foundational instance documents, retiring/unregistering foundational
   objects, closing root's protected routes and declaring local lost; inspect unchanged state.

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

- **M1 — proposed, for the owner's acceptance**: one inventory row for T1's empty initialization,
  T2's repeated initialization and later start, and both T3 commit-cut branches. Count first-start
  objects and root Conduits, compare IDs/from/state/provenance/trust domain, and list additional
  authored objects separately. Record implementation dispatches, hook attempts and external
  effects before/after each boundary. Count mismatches and unknowns separately, with no inferred success.
- **M2 — proposed, for the owner's acceptance**: for both T4 branches and each T5 branch list
  observed diagnostics/parts, pending action evidence/answer, diagnosis, verification, each
  explicit request/manual action and final status. For T4 count hook attempts at restart (one
  in branch (a); none in branch (b) before reconciliation), reconciliation and further setup; retain independent checks and reconciliation records.
  Count correct diagnoses over all branches,
  manual recovery steps, changed files/lines, time without intended access and unexpected
  recreations/effects. List panel actions apart and never count harness fault preparation as
  author repair. Provider package work uses M0's separate account.

## Vectors expected

Every row, its name and its safety classification is **proposed, for the owner's
acceptance**. Safety means failure could grant access, leak a secret, lose/repeat an effect
or report a wrong result under [the tracker](../../docs/AUDIT.md#how-to-use-this-document).
The safety promise column names the future required tag; it does not assert tagging exists.

| ID and proposed name | Promises cited; observation | Safety; safety promise |
| --- | --- | --- |
| V1 `s9-first-initialize` | FST-001–003/005/007, REC-001/002; exact foundation/provenance/domain, no extra calls | yes; FST-003 |
| V2 `s9-later-start` | FST-007, OPS-008/012/018/021/028, LIF-008; stable domain/state, no implicit repeated effect or recreation | yes; OPS-018 |
| V3 `s9-interrupted-initialize` | FST-001/002/003/007, OPS-008; before/after commit under reference-engine DL6 policy | yes; FST-003 |
| V4 `s9-explicit-repair` | LIF-004/027/028, OPS-014/021/022, ACC-005/033, PND-006; failed-hook branch: restart's one new attempt recorded, repair, then one attempt to readiness; unknown-hook branch: restart dispatches no hook call, independently verify absence, authorize and record the reconciliation before any further hook call or setup request, repair to readiness (OPS-021); in both, no repeated successful parts | yes; LIF-027, OPS-021 |
| V5 `s9-protected-foundation` | FST-006, LIF-022/033/034/041, ACC-024/053; protected objects/routes unchanged, explicit authored replacement only | yes; LIF-034 |

Existing slices: [first-start](../../conformance/conventions/first-start/initial/case.yaml),
[first-start-repeat](../../conformance/conventions/first-start/repeat/case.yaml),
[trust-domain](../../conformance/conventions/first-start/trust-domain/case.yaml),
[lifecycle](../../conformance/conventions/first-start/lifecycle/case.yaml),
[root-system](../../conformance/conventions/first-start/root-system/case.yaml) and
[setup-restore](../../conformance/conventions/setup/restore/case.yaml). Reuse these; V3's
atomic crash behavior is reference-engine evidence, not a new portable atomicity promise.

## Runs

**Proposed, for the owner's acceptance (Q2)**: one coordinator author run and one fresh
Codex author run, with separate counts, each doing every task and listed branch once. The
fresh author receives the specification, tasks and CLI format reference; coordinator knowledge
is a limitation. A rerun after a fix is a new dated run retaining the earlier failure.
Pin engine, package and client versions; record authored files, commands, task times, mistakes,
all author requests and intervals without access, panel actions, fault/barrier events,
inspection and independent state snapshots, records and vector verdicts. Dated results go
beside this README. State unrun branches and deviations explicitly.

One local Unix machine, fresh engine stores, built-in runtime/machine providers, a local
repair/precondition package, independent readback and an effect/lost-reply fixture. Use the
engine CLI and scripted sending instances; no model Agent, external account/service, second
machine or subscription is required. Each author does T1–T5 once, with separate configured,
generated and malformed-domain labs, both T3 commit cuts and both T4 branches (failed and
unknown hook): two T4 branch trials per author, four across the proposed runs. Keep each T2/T4
sequence in its own store through restart/repair; reset independent branches. Retain snapshots without
credential contents; record trust-domain/object identity, never tokens or private keys.

No S9 case requires real-client behavior. Q3 leaves optional first-session/model presentation
to A71's evidence unless a replay is chosen. Owner time/presence is acceptance and review only for
scripted runs; no sign-in session is needed. Optional subscription replay adds Conduit sign-in
and prompt time, counted apart; pin client/protocol and freeze trial count before that run.

## What it needs first

[CONFORMANCE.md](../../engine/CONFORMANCE.md#implemented-choices) documents atomic first start,
repeated first start preserving identities, and interrupted work ending unknown without
reissue (DL6). Its [startup-repair section](../../engine/CONFORMANCE.md#i4-02-startup-repair-retirement-parts-and-inspection)
says setup failure leaves the serving engine available for explicit repair. Qualify those
paths in the actual shell, initialization fault barriers, PND-006 evidence, retained invocation
state (OPS-008), protected foundation (FST-006/LIF-034/ACC-024) and explicit replacement/lift.
Package contracts must supply T4's actual repair/readback before author runs, without editing
provider code during the tasks. Implement safety tagging before claiming its gate. The owner
accepts Q1–Q4; no machine/account setup beyond the local lab is required by the default run.

## What `spec/` does not yet decide

- **DL6 opening:** [first-start.yaml](../../spec/promises/first-start.yaml) and
  [DL6](../../spec/implementation-defined.yaml) deliberately leave initialization triggers,
  interruption/repetition and repair mechanisms to each engine. The current engine documents
  atomic initialization and startup setup repair; it does not thereby promise arbitrary damaged
  foundation reconstruction. No task assumes a portable repair verb or recreates root/its grants.
- **G3:** Safety vector field/tagging is *spec pending* in
  [IN_FORCE.md topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement).
  Names and classifications here do not fill that specification gap.
- A missing root-to-Pool route appears in the engine's repair examples, but FLEET-005 now
  automatically makes a protected default. As with S4 G2, a damaged/legacy snapshot must not
  be claimed as a state reached by current author requests. Q4 keeps such recovery outside
  the default run until its preparation and engine policy are fixed.

## Open for the owner

- **Q1:** Accept the proposed pass conditions, measures, counting rules and vectors? Options:
  accept, amend or defer. **Recommendation:** accept after the scenario's prerequisites are settled.
- **Q2:** Who authors the runs, and how many author runs should each scenario have? Options:
  coordinator alone, coordinator plus fresh Codex author, or another named plan.
  **Recommendation:** two separate author runs as above; this introduces no S1 numeric bar.
- **Q3:** Which client should exercise cases that need real-client behavior: the owner's subscription, a second subscription account signed in through Conduit's own sign-in flow, or none (scripted clients only)?
  **Recommendation:** none: run S9 with engine clients.
  If a replay is chosen, a second account signs in through Conduit as
  [ruling 72](../../docs/proposals/ruling-72/README.md) requires for S3/S10, not as an existing
  S9 requirement. Freeze that replay's client/trial choices first.
- **Q4:** Which repair scope should S9 exercise before any additional DL6 policy is chosen?
  Options: documented initialization cuts plus reachable authored-setup repair, or also
  damaged-foundation/legacy-store repair after an engine policy and fixtures exist.
  **Recommendation:** the former; keep unsupported foundation reconstruction explicitly unshown.

**Answers, under [ruling 75](../../docs/proposals/ruling-75/README.md), D (2026-11-11):**

- Q1: accepted.
- Q2: written twice, by the coordinator and by a fresh Codex author, with scripted clients (D3).
- Q3: none; engine clients only (D4).
- Q4: the documented initialization cuts, and reachable repair of authored setup.

## Limits

These runs show a local reference engine's declared initialization and repair policy, not
arbitrary corrupt-store recovery, distributed failover, all runtime providers or population
usability. Stable IDs do not prove durable backup quality. Engine-specific crash cuts add no
portable atomicity guarantee. This paper order runs nothing and writes no promise or vector.
