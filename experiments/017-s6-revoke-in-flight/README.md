# 017 — S6: revocation and retirement in flight

> **For the owner**
> - **Accepted under ruling 75 D on 2026-11-11.** Every item labelled proposed below is accepted,
>   with the answers under "Open for the owner" applied.
>
> - Proposed P2: closing access stops new dispatch while the original response retains its outcome.
> - Proposed P3: retirement closes incoming access immediately and reports shutdown truthfully.
> - Proposed P4: retirement affects only owned work and preserves records and backing data.
> - Proposed P5: cascade dependencies hold under failure with a documented DL1 order.
> - Proposed M0: extend the first build's author measures and selected counting rules to S6.
> - Proposed M1: count dispatch decisions around close and retirement barriers.
> - Proposed M2: count shutdown duties, cascade ordering and residual data.
> - Proposed V1: `s6-close-in-flight` (safety).
> - Proposed V2: `s6-retire-in-flight` (safety).
> - Proposed V3: `s6-retired-consumer-onward` (safety).
> - Proposed V4: `s6-retirement-duties` (safety).
> - Proposed V5: `s6-cascade-order` (safety).
> - Proposed R1: two separate author runs, with branch counts and optional replay rules below.
> - Q1: Accept the proposed pass conditions, measures, counting rules and vectors?
> - Q2: Who authors the runs, and how many author runs should each scenario have?
> - Q3: Which client should exercise cases that need real-client behavior: the owner's subscription, a second subscription account signed in through Conduit's own sign-in flow, or none (scripted clients only)?
> - Q4: Should S6 include unordered and cyclic retirements once the engine documents DL1?

## Status

Pre-registered on 2026-11-11 by work order 102. Accepted by the owner on 2026-11-11 under
[ruling 75](../../docs/proposals/ruling-75/README.md), D: its pass conditions, measures,
counting rules and planned vectors, with the answers under "Open for the owner" applied. Ruling
45 fixes what an experiment must show before it runs. No result is recorded here.

## Question

What happens when access is revoked and a Resource is retired while requests are in flight?
[S6's roadmap row](../../ROADMAP.md#scenarios) tests current-access checks and retirement duties:
which already-dispatched result survives, what can still be dispatched, and what shutdown
actually confirms.

## What it must show

1. **P1 — ruled**, [ruling 47](../../docs/proposals/ruling-47/README.md#when-to-publish):
   S6 runs end to end on the reference engine with its vectors passing, including every
   safety vector it adds ([ruling 68, point 2](../../docs/proposals/ruling-68/README.md)). Judge
   by task transcripts, independent state checks and vector verdicts. Missing cases are
   missing evidence, never passes.
2. **P2 — proposed, for the owner's acceptance**: after a Conduit closes, new requests, repeats
   and onward requests in that chain are refused without dispatch. An original in-flight
   response is delivered with its actual outcome and outputs; a later task-result fetch is
   checked anew and cannot use the closed route. Judge barrier order, dispatch logs and results.
3. **P3 — proposed, for the owner's acceptance**: at retirement dispatch the Resource is retired
   and incoming Conduits close immediately. No new call enters it; shutdown is pending/unknown
   until confirmed. Do not infer a stopped external effect from access revocation. Judge
   lifecycle, shutdown, provider stop evidence and independently observed external state.
4. **P4 — proposed, for the owner's acceptance**: the retire hook precedes stopping the run;
   the stop is asked for whatever the hook's outcome. No setup-ever means neither hook nor stop.
   Retirement/removal erase no backing service data or records and stop only what the instance
   holds. Judge external ledger, neighboring Resource health, records and stop targets.
5. **P5 — proposed, for the owner's acceptance**: in a cascade, each dependent retires before
   its dependency and confirms shutdown before that dependency begins. A failed/unknown member
   holds back only successors; independent branches proceed; the Pool waits for all members.
   Judge the recorded partial order and the engine's declared DL1 choice, not an invented
   portable tie order.

## What it covers

There is no S6 row in [round 6's case table](../../ROADMAP.md#round-6s-evidence-and-measurements):
no E-case or D-measure is assigned to it. E18 remains S4's. Adjacent traces
[S1-close-in-flight, S1-unknown-write, S1-reconcile and S1-setup-sender](../../conformance/s1-safety-cases.md)
provide boundaries, not evidence of the complete scenario.

Promise IDs, by today's files:

- [access.yaml](../../spec/conventions/access.yaml): ACC-001, ACC-011, ACC-017–019,
  ACC-033, ACC-034, ACC-036, ACC-039, ACC-055.
- [lifecycle.yaml](../../spec/conventions/lifecycle.yaml): LIF-005–007, LIF-013–021,
  LIF-024–028, LIF-035, LIF-036, LIF-040;
  [conduits.yaml](../../spec/promises/conduits.yaml): CON-008.
- [operations.yaml](../../spec/conventions/operations.yaml): OPS-001–005, OPS-008,
  OPS-012–014, OPS-018, OPS-021, OPS-022, OPS-027, OPS-028;
  [revisions.yaml](../../spec/conventions/revisions.yaml): REV-012, REV-014.
- [templates.yaml](../../spec/promises/templates.yaml): TPL-009, TPL-018;
  [providers.yaml](../../spec/promises/providers.yaml): PRV-008, PRV-009;
  [pools.yaml](../../spec/promises/pools.yaml): POOL-001, POOL-006.
- [records.yaml](../../spec/conventions/records.yaml): REC-001–004;
  [implementation.yaml](../../spec/interface/implementation.yaml): IFC-016, IFC-025, IFC-028;
  [pending.yaml](../../spec/promises/pending.yaml): PND-007 (shared-Pool retirement notice).

Feeds A38/A39/A50/A68 and A40's later safety model; overlaps A65's lifecycle/enforcement work
and A71's cached-tool observations. S6's row is its register row.

## The tasks

Preparation, apart from author work: supply local servers with an independent effect ledger,
barriers before dispatch/completion and in a retire hook, plus a task-result variant. The
harness holds an original request, closes access or dispatches retirement, then releases the
original result before allowing the retire hook and stop to finish. Separate branches lose a
reply or report uncertain stop. Provide a dependent Resource pair, an independent third member
and, if Q4 is accepted, a legal cycle fixture. Inject failures at fixed barriers, not by timing
sleep intervals. Providers and harness fault controls are prepared before author timing.

1. **S6-T1:** Author one shared Resource, two scripted consumers' independent access and an
   unrelated working Resource. Start a call through A's route, close that route, then compare
   the original response, A's fresh call/repeat and B's still-permitted call.
2. **S6-T2:** Hold an onward chain across the close; ask for another onward call after it, both
   directly and through a nested invocation. Compare with a task result available only by a
   later fetch after closing. Inspect records through a separate permitted management route.
3. **S6-T3:** Retire the shared Resource with a call held in flight. Inspect retired/pending
   before shutdown, try both consumers' new calls, release the original result, then inspect
   hook/stop completion. In another branch lose completion and verify unknown rather than success.
4. **S6-T4:** Retire a consumer while one of its invocations is active; compare an ordinary
   onward call with a retire-hook onward call through an otherwise open route. Verify the
   retirement exception under ACC-039 without widening authority for ordinary work.
5. **S6-T5:** Exercise retire-hook success, failure and unknown, uncertain stop, further explicit
   retirement/reconciliation and no-setup-ever branches. Check removal, retained records,
   backing data and unrelated Resource health. As an authorized non-root sender, retire a
   separate shared Resource and inspect the PND-007 notice to another Fleet.
6. **S6-T6:** Request Pool retirement with its Resources. Explain dependency order and inspect
   failure blocking only successors, independent progress and final Pool retirement. Test
   unordered/cyclic members only after the Q4/DL1 prerequisite; no task chooses that policy.

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

- **M1 — proposed, for the owner's acceptance**: one row for each request in T1–T4, with sender,
  caller, `serving`, Conduit ID, barrier position, dispatch/no dispatch, outcome/reason and
  returned outputs. Count forbidden post-close/post-retirement dispatches, original results
  delivered, later task fetches refused and B's successful checks separately. List every
  unknown rather than scoring it as no effect. Record explanation and permitted next action.
- **M2 — proposed, for the owner's acceptance**: per T3/T5/T6 branch count hook and stop calls,
  external effect totals, removed IDs, retained record/data checks and shutdown transitions.
  For each cascade edge record predecessor shutdown confirmation and successor dispatch;
  count violations, blocked successors and independent members completed. List DL1 tie/cycle
  choices and trace separately; never average a failed branch into a successful cascade.
  Count notices by affected Fleet, with closed Conduit IDs and sender/caller evidence.

## Vectors expected

Every row, its name and its safety classification is **proposed, for the owner's
acceptance**. Safety means failure could grant access, leak a secret, lose/repeat an effect
or report a wrong result under [the tracker](../../docs/AUDIT.md#how-to-use-this-document).
The safety promise column names the future required tag; it does not assert tagging exists.

| ID and proposed name | Promises cited; observation | Safety; safety promise |
| --- | --- | --- |
| V1 `s6-close-in-flight` | ACC-011/034/036, IFC-016/025, REC-002; original response survives, repeats/task fetch/onward refused | yes; ACC-034 |
| V2 `s6-retire-in-flight` | LIF-005/014/024, CON-008, ACC-034; immediate closure, actual original outcome, truthful shutdown | yes; CON-008 |
| V3 `s6-retired-consumer-onward` | ACC-017–019/039, IFC-028; only consumer's retirement chain may use its retained open route | yes; ACC-039 |
| V4 `s6-retirement-duties` | TPL-009/018, PRV-008, OPS-021/022/028, LIF-016/019/025/035/036; hook then stop, no collateral data removal | yes; LIF-016 |
| V5 `s6-cascade-order` | LIF-020/021/024, REV-014, DL1; dependency confirmation gates, independent progress, container last | yes; LIF-021 |

Existing slices: [setup-close-in-flight](../../conformance/conventions/setup/close-in-flight/case.yaml),
[changes-retirement](../../conformance/conventions/changes/retirement/case.yaml),
[review-retire-run-unknown](../../conformance/conventions/setup/review-retire-run-unknown/case.yaml)
and [review-retire-connection-unknown](../../conformance/conventions/setup/review-retire-connection-unknown/case.yaml).
They do not establish task-result revocation or the full cascade.

## Runs

**Proposed, for the owner's acceptance (Q2)**: one coordinator author run and one fresh
Codex author run, with separate counts, each doing every task and listed branch once. The
fresh author receives the specification, tasks and CLI format reference; coordinator knowledge
is a limitation. A rerun after a fix is a new dated run retaining the earlier failure.
Pin engine, package and client versions; record authored files, commands, task times, mistakes,
all author requests and intervals without access, panel actions, fault/barrier events,
inspection and independent state snapshots, records and vector verdicts. Dated results go
beside this README. State unrun branches and deviations explicitly.

One local Unix machine and fresh stores; use engine clients and scripted consumers rather
than model Agents. Each author does T1–T6 once with every listed branch and resets branches
outside their active barrier sequence. Local packages: ledger-backed Resource, onward caller,
retire-hook/task fixture and runtime stop-report fixture. No external service, subscription
account or second machine is required. A shared-Pool notice fixture includes two Fleets and
scripted consumers; Fleet authority pending batch 2 is not assumed for the basic access tests.

No S6 case needs real-client behavior for its conformance observation. A71 already separates
cached tool visibility from engine refusal; optional model awareness is Q3, not a substitute
for dispatch logs. Owner presence/time: acceptance and review, no synchronous presence for
scripted runs. An optional subscription replay would add Conduit sign-in/prompt time, recorded
apart with its account role and protocol version; freeze its trial count before running.

## What it needs first

[CONFORMANCE.md](../../engine/CONFORMANCE.md#choices-still-required) leaves
`retire-with-resources.v1` and DL1 ordering unfinished. Implement LIF-021, document the order
among unordered/cyclic members, and qualify it before T6. Single-instance retirement exists;
qualify in-flight task results (IFC-016/ACC-034), consumer retirement (ACC-039), hook/stop
branches (TPL-009/018, PRV-008) and notices (PND-007) through runnable packages first.
Implement safety tagging before gate claims. The owner accepts Q1–Q4 and supplies a subscription
sign-in only for an optional replay. No account or network is used by this paper order.

## What `spec/` does not yet decide

- **DL1 opening:** [LIF-021](../../spec/conventions/lifecycle.yaml) fixes dependency ordering
  and the shutdown gates; [DL1](../../spec/implementation-defined.yaml) deliberately leaves
  unordered/cyclic ordering to the engine. No portable tie order is promised, and the first
  build has not chosen/implemented it. This is an engine prerequisite, not authority to invent
  a language rule. Q4 does not choose an order for other engines.
- **G3:** Safety vector field/tagging is *spec pending* in
  [IN_FORCE.md topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement).
  Names and classifications here do not fill that specification gap.
- Shutdown deadlines and late completions also use the engine's DL10/DL2 policy. Revocation
  itself does not promise cancellation or external-effect rollback; ACC-034 already decides
  original-result delivery versus later task fetch. No task assumes a missing guarantee.

## Open for the owner

- **Q1:** Accept the proposed pass conditions, measures, counting rules and vectors? Options:
  accept, amend or defer. **Recommendation:** accept after the scenario's prerequisites are settled.
- **Q2:** Who authors the runs, and how many author runs should each scenario have? Options:
  coordinator alone, coordinator plus fresh Codex author, or another named plan.
  **Recommendation:** two separate author runs as above; this introduces no S1 numeric bar.
- **Q3:** Which client should exercise cases that need real-client behavior: the owner's subscription, a second subscription account signed in through Conduit's own sign-in flow, or none (scripted clients only)?
  **Recommendation:** none: use scripted clients here.
  If a replay is chosen, a second account signs in through Conduit as in
  [ruling 72](../../docs/proposals/ruling-72/README.md)'s S3/S10 replays; that requirement is not
  ruled for S6. Freeze client/trial choices before running.
- **Q4:** Should S6 include unordered and cyclic retirements once the engine documents DL1?
  Options: include both after implementation, or start with dependency-only evidence and list
  the missing branches. **Recommendation:** include both, awaiting the engine's documented
  choice; partial runs must not be reported as the completed S6 gate.

**Answers, under [ruling 75](../../docs/proposals/ruling-75/README.md), D (2026-11-11):**

- Q1: accepted.
- Q2: written twice, by the coordinator and by a fresh Codex author, with scripted clients (D3).
- Q3: none; scripted clients only (D4).
- Q4: unordered and cyclic retirements are included once the engine documents DL1.

## Limits

This demonstrates local mediated access and selected interleavings, not client confinement,
instant stopping of remote effects, every runtime provider or distributed partition safety.
An unknown shutdown is not confirmation of stopped work. A deterministic cascade is not a
load benchmark or A40's model checking. No run, result, promise or vector is written here.
