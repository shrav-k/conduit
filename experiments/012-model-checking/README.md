# 012 — model-checking harness

Status: executed 2026-11-11 under work orders 104 (MC1) and 105 (MC2), ruling 71's proof lane.
The expected verdicts below were registered before execution; later orders extend this harness.

## Question and scope

A40 asks whether each safety promise holds under model checking in small configurations,
including two machines with message loss, reordering, partition and a lost machine, or has
an explicit reason it cannot be checked this way (readiness gate G5).

MC1 tests the harness itself, **not any Conduit promise**; MC2 below adds promise models. Its properties are labelled
`harness`. Models are pure reference models of the rules promises state, independent of
engine code, with the fewest states that can break those rules. Bounded checks establish
neither unbounded safety nor liveness or fairness.

D10 places this living harness in its own crate, outside the engine workspace, so later orders
can extend it without coupling it to the engine; it can move there later. Its sole dependency
is Stateright `=0.31.0`, evaluated in [experiment 004](../004-stateright-evaluation/README.md).
Its lock file starts from 004's; its toolchain pins Rust `1.97.1`.
CI already runs `cargo test --release` in each experiment. CI neither formats nor lints
experiments, so every order changing this crate must run both checks below itself.

## Run it

From this directory, one cargo command at a time on the shared machine:

```sh
CARGO_BUILD_JOBS=3 cargo fmt --check
CARGO_BUILD_JOBS=3 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=3 cargo test --release
CARGO_BUILD_JOBS=3 cargo test --release -- --nocapture --test-threads=1
```

The last command prints counts and counterexamples. Full runs use `CONDUIT_MC_THREADS`,
default 2, clamped to at most 2; zero or malformed values are rejected. Refutations always
use one-thread breadth-first search and `finish_when(HasDiscoveries::AnyOf)` naming only
the target safety property. Tests and helpers share one lock, serializing model checks.
Full runs separately explore the whole bounded state space, including broken variants.
An internal permanently true traversal control prevents Stateright from stopping expansion
when all modeled properties have discoveries. It is not a modeled safety claim and needs no
seeded variant. Stateright's depth includes the initial state at depth 1.

Stateright 0.31.0 checks stopping conditions after batches of 1500 states, and may replace
traces even within a single-thread batch. A visitor preserves the first target discovery;
new states are then rejected, while the checker finishes its batch and observes `AnyOf`.
This preserves the first single-thread BFS trace. Full runs do not use that early stop.
The checker state target counts generated states including repeats and may overshoot.
A separate boundary guard instead tracks exact unique states under a mutex and rejects
any new state beyond the ceiling, or after the deadline. Either truncation fails the run,
before any verdict is interpreted. The elapsed time is also checked after joining.
Bounded state transitions and action order remain pure, including during trace replay.

## Fault model and model requirements

`harness::Config` states node names, all nine global budgets, queue bounds, a unique-state
ceiling no greater than 1,000,000, a timeout, and the protocol's remaining bounds in one
place. Zero budgets require reasons. Budgets count total actions across the entire run,
including after recovery. Bounds must be justified by the protocol; exceeding a queue
bound fails rather than silently pruning a behavior.

Each directed link is a sequence, retaining separate positions for equal messages and copies.
Ordinary delivery removes position zero. Delivering a later position uses the reorder budget;
zero means FIFO per link. Loss removes any position; duplication appends one copy.
A cut blocks both directions between two nodes; healing reopens them. Existing messages
and new sends are held during a cut, and can still be lost or duplicated. Crashed and
permanently lost destinations retain queued messages. Crash erases only volatile state;
recovery restores saved state. Permanent loss can happen at any point, including while a
node is crashed; it never recovers. Live nodes can fire protocol-enabled timers.
The ledger is outside nodes and untouched by every fault.

Every property has a name and promise IDs (or `harness`). Each enabled fault, including
healing and recovery, gets a reachability property. Every safety-relevant outcome needs
an explicit reachability property. Every safety property must have a seeded variant whose
full run refutes only that property, plus a separate refutation run printing the trace.
Tests check configuration, metadata, witnesses, seeded failures, FIFO and positional copies,
partition/crash queue retention, permanent loss, global budgets and truncation rejection.
Later models must use these checks and record their own bounds and outcomes here.

## Pre-registered model and expected verdicts

A sender sends ID 0 and retries on a timer. The receiver applies an effect and answers.
The external ledger counts effects per ID and records every confirmation with the effect
count at receipt; later effects or sender crashes cannot erase an earlier false confirmation.
Both safety properties read only this ledger.

The configuration in `retry::configuration` has two nodes, all fault budgets 1, timers 2,
one request ID, at most four messages per directed link, 1,000,000 unique states and a
15-second timeout per run. One initial send, two retries and one wire copy bound requests,
effects and confirmations by four each. No fault is disabled. Either node may crash or be
lost; either direction may reorder, lose or duplicate messages. Witnesses cover applied
effect, received confirmation, and receiver lost for good after an effect, plus every fault.

Expected verdicts, fixed before results:

| Variant | Expected safety verdict | Expected reachability |
| --- | --- | --- |
| Saved | At most once and no false confirmation hold: applied IDs survive recovery | Every witness reached |
| VolatileIds | Only at most once refuted; IDs disappear on receiver crash | Every witness reached |
| AnswerFirst | Only no false confirmation refuted; confirmation precedes a separate apply step | Every witness reached |

AnswerFirst saves its ID before answering, reserving a single deferred effect. Crash can lose
that deferred effect; recovery cannot apply it twice. VolatileIds also has a control run with
crash and recovery disabled to verify that its failure requires a crash; all other budgets
and witnesses stay enabled, with reasons recorded for those two zero budgets.

## Results

Measured 2026-11-11 on an Apple M2 Max, 12 cores (8 performance, 4 efficiency),
Rust `1.97.1` (`8bab26f4f`, 2026-07-14), Stateright `0.31.0`. Full runs used 2 checker
threads (the default); refutation runs used 1. Counts below come from the separate full
runs in `cargo test --release -- --nocapture --test-threads=1`.

| Date | Variant | Unique states | Depth | Seconds | Observed verdict |
| --- | --- | --- | --- | --- | --- |
| 2026-11-11 | Saved | 6,090 | 16 | 0.050 | Both safety properties hold; every witness reached |
| 2026-11-11 | VolatileIds | 8,910 | 16 | 0.087 | Only at most once refuted; every witness reached |
| 2026-11-11 | AnswerFirst | 18,360 | 17 | 0.201 | Only no false confirmation refuted; every witness reached |
| 2026-11-11 | VolatileIds without crash/recovery (control) | 1,395 | 14 | 0.011 | Both hold; every enabled witness reached |

The ordinary `cargo test --release` passed **8 unit tests, 0 failed**, plus 0 doc tests,
in **2.10 seconds wall time**, including incremental compilation (test execution: 0.38 s).
The counts command also passed 8 unit tests, 0 failed, plus 0 doc tests, in 0.55 s wall time
(test execution: 0.39 s). Both were measured with `CARGO_BUILD_JOBS=3` and `time -p`.
Formatting and clippy with warnings denied passed. The truncation regression intentionally
catches two panics (ceiling and timeout), visible with `--nocapture`; neither is a verdict
of holds. All full model runs above finished exhaustively within their limits.

The first single-thread BFS counterexample for VolatileIds had five actions:

```text
1. delivered sender -> receiver [position 0]
2. timer sender
3. crashed receiver
4. recovered receiver
5. delivered sender -> receiver [position 0]
```

The first counterexample for AnswerFirst had two actions:

```text
1. delivered sender -> receiver [position 0]
2. delivered receiver -> sender [position 0]
```

Tests assert these trace lengths, the receiver crash and recovery in the first trace, and
no applied effect at the end of the second. A further regression applies the deferred
effect and crashes the sender, then checks that the external false-confirmation record remains.

Times on this shared machine are indicative only. Concurrent checker scheduling can affect
reported depth and timing; these are the observations from this dated run. The target for the
ordinary release test run on `ubuntu-latest` is about 60 seconds; no Linux run was available,
and no Linux result is claimed.

## Relationship and next models

Experiment 004 evaluated Stateright and found it suitable for small pure models. This harness
adds explicit partitions, total fault budgets, limits that fail closed, external history and
repeatable helpers. The safety catalogue is `docs/implementation/SAFETY_CATALOGUE.md`;
later models take their promise lists from it. This self-test provides no catalogue coverage.

MC2 below covers operation identity, unknown outcomes, reconciliation and revisions.
Later orders take the rest of the safety catalogue.

## MC2 operation identities: model and pre-registration

Registered 2026-11-11 under work order 105, **before any MC2 run**. This extends the
harness rule about seeded variants: a variant may refute several properties, provided its
entire expected verdict is in the table and a separate refutation targets its named defect.
The earlier `retry.rs` rules and tests remain unchanged.

Nodes are sending instance A, the engine and one implementation, with one caller and target.
Scope adds B with the same key and contents. A initially sends `(k,c1)`, repeats on its timer
(up to two times), then sends `(k,c2)`, then two equal requests without keys. To reduce states,
the script suffix advances only with empty links and no invocation in progress; repeats can
race the initial dispatch, replies, timeouts and reconciliation. A's request queue has at most
two entries before a wire copy. This is a bounded scheduling restriction, not a language rule.
All sends have at most three queued messages per directed link. Fault budgets are one,
timer budget two; each full run has a hard 200,000 unique-state ceiling and 10-second timeout.
Identities abstract the fixed caller/target plus sender/key; each keyless delivery allocates a
fresh identity. Per-identity ledger histories preserve dispatch contents, first contents and
attempt, every applied effect, each emitted reply with its attempt (including deduplicated
attempts), outcomes with the effect count then, and request answers.
Different identities' histories commute. Observer labels distinguish the two scripted keyless
requests. The keyless witness requires distinct identities, each with a dispatched decision
for its scripted request and an effect with that request's contents. A copied first request
or one shared identity/effect cannot satisfy both labels. Properties read only these histories
and answers.
Saved invocation indexes and implementation deduplication survive crashes. Recovery marks
in-progress invocations unknown without redispatch. An emitted reply for the current attempt
that is no longer queued enables lost-reply unknown, even if deduplication suppressed a new
effect; an older attempt's reply emission is insufficient. Reconciliation is an atomic authorized
actor's read of effects and recording of succeeded/failed. Late replies are ignored.

Fault parts (all also enable two timers): T=timers only; L=loss; R=reordering; D=request
wire duplication; P=cut and heal; C=crash and recovery; M=permanent implementation loss.
Every non-timer fault has budget one in its named part and zero elsewhere, to isolate fault
interactions within the bound. D permits copies only from A/B to the engine. P cuts only
engine/implementation; C crashes only engine/implementation; M loses only implementation.
Across parts each relevant fault reaches each safety property's model, but combinations of
different non-timer fault kinds are outside these bounds. Ordinary message delay remains in
all parts. Scope uses T. Each enabled harness fault must have a witness.

Safety abbreviations: O=at most once (OPS-018/008/012/024); W=no wrong outcome
(OPS-012/014); U=unknown stays unknown (OPS-013/IMP-007); A=repeat answered without dispatch
(OPS-009/032); C=different contents conflict (OPS-010). H=holds, F=refuted.
Reachability K=two keyless effects; S/F=reconcile succeeded/failed; sources=timer, lost reply,
implementation crash/loss and recovery when enabled. Every enabled reachability is expected
reached. Fenced enables failed reconciliation reachability only in L: without a lost dispatch,
its fence cannot certify absence before an atomic implementation effect. TimerFailed/C still reaches reconciliation via implementation crash or recovery;
its timer deliberately produces failed, so has no unknown-on-timer witness. VolatileIndex/C
has no recovery-unknown witness because it deliberately loses the record. Scope requires
both first A and B requests to dispatch as separate invocations without a repeat answer.

| Variant | Part | O | W | U | A | C | Named refutation | Reachability |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Plain, promises as written | T | F | H | H | H | H | O in T | all enabled |
| Plain, promises as written | L | F | H | H | H | H | O in T | all enabled |
| Plain, promises as written | R | F | H | H | H | H | O in T | all enabled |
| Plain, promises as written | D | F | H | H | H | H | O in T | all enabled |
| Plain, promises as written | P | F | H | H | H | H | O in T | all enabled |
| Plain, promises as written | C | F | H | H | H | H | O in T | all enabled |
| Plain, promises as written | M | F | H | H | H | H | O in T | all enabled |
| Deduplicating implementation | T | H | H | H | H | H | none | all enabled |
| Deduplicating implementation | L | H | H | H | H | H | none | all enabled |
| Deduplicating implementation | R | H | H | H | H | H | none | all enabled |
| Deduplicating implementation | D | H | H | H | H | H | none | all enabled |
| Deduplicating implementation | P | H | H | H | H | H | none | all enabled |
| Deduplicating implementation | C | H | H | H | H | H | none | all enabled |
| Deduplicating implementation | M | H | H | H | H | H | none | all enabled |
| Fenced failed reconciliation, plain control | T | H | H | H | H | H | none | all enabled |
| Fenced failed reconciliation, plain control | L | H | H | H | H | H | none | all enabled |
| Fenced failed reconciliation, plain control | R | H | H | H | H | H | none | all enabled |
| Fenced failed reconciliation, plain control | D | H | H | H | H | H | none | all enabled |
| Fenced failed reconciliation, plain control | P | H | H | H | H | H | none | all enabled |
| Fenced failed reconciliation, plain control | C | H | H | H | H | H | none | all enabled |
| Fenced failed reconciliation, plain control | M | H | H | H | H | H | none | all enabled |
| BlindReplay, plain | T | F | H | H | H | H | O | all enabled |
| VolatileIndex, plain | C | F | H | H | F | F | O | all enabled |
| TimerFailed, plain | C | F | F | H | H | H | W | all enabled |
| LateReply, plain | T | F | H | F | H | H | U | all enabled |
| KeyOnly, plain | T | F | H | H | H | F | C | all enabled |
| RepeatSucceeded, plain | T | F | H | H | F | H | A | all enabled |
| Deduplicating scope, A and B | T | H | H | H | H | H | none | all enabled + scope |

Plain is expected to fail: engine timer marks unknown while a dispatch is delayed; actor
reads zero effects and records failed under OPS-014; original dispatch lands; A's repeat
causes a new attempt under OPS-024, applying a second effect contrary to OPS-018. The
Fenced control permits failed reconciliation only when no dispatch remains queued and none
can subsequently apply (implementation application is atomic). This is **stronger than
OPS-014**, and is never substituted for the promises-as-written model.

Excluded: OPS-008's complete saved fields (fixed operation/caller/target/Conduit suffice for
this slice); OPS-026 defaults (c1/c2 already denote normalized contents); filters in OPS-024
(the operation is effectful); setups and retirements (separate lifecycle models); REV-010
control configuration storage and reconciliation (the operation model records only outcomes);
and wire copies of engine dispatches (transport delivers each send at most once). Caller
answers are atomic at the engine, without an additional return link. Output/content retention,
authorization, unknown invocation IDs and unauthorized reconciliation are outside this model.
This outcome abstraction does not establish OPS-009's content-retention/OPS-034 cases.

## MC2 revisions: model and pre-registration

Engine, implementation of one instance, and one caller A. Two requests naming revision r0
are initially queued together, followed by one naming none after both have reached the
engine, then one naming the revision produced by the first successful change. Revisions are
opaque equality tokens represented by small integers. Dispatch, implementation application
and result are separate transitions. The engine stores revisions on successful completion;
unknown ends a change for REV-012 even if its dispatch can still arrive. Every request,
dispatch (named and stored revision), applied effect and end is logged in an external ledger.
Properties reconstruct in-flight changes from dispatch/end history, never from engine state.

T,L,R,C have the same budgets, queue/state/time bounds as above; C crashes only the engine.
Loss and reordering apply to all links. Four unique changes and at most two timers bound the
script. No duplication, cut or lost-machine configuration is claimed for revisions: this
order's revision fault slice explicitly names loss, reordering, engine crash/recovery and
timers. Setups and retirements are excluded to isolate change serialization; REV-010 is
excluded because it additionally stores a control's configuration and requires its schema,
authority and lifecycle, not present here. The operation model separately explores outcome
reconciliation. No end-to-end claim about control reconciliation follows.

V=revision checked (REV-002/011), J=one change at a time (REV-012). Reachability covers success,
the missing-revision and followup requests, timer unknown, lost-reply unknown in L, recovery
unknown in C, and every enabled harness fault. All are expected reached, including in mutants.

| Variant | Part | V | J | Named refutation | Reachability |
| --- | --- | --- | --- | --- | --- |
| Checked | T | H | H | none | all enabled |
| Checked | L | H | H | none | all enabled |
| Checked | R | H | H | none | all enabled |
| Checked | C | H | H | none | all enabled |
| Unchecked | T | F | H | V in T | all enabled |
| Unchecked | L | F | H | V in T | all enabled |
| Unchecked | R | F | H | V in T | all enabled |
| Unchecked | C | F | H | V in T | all enabled |
| Concurrent | T | H | F | J in T | all enabled |
| Concurrent | L | H | F | J in T | all enabled |
| Concurrent | R | H | F | J in T | all enabled |
| Concurrent | C | H | F | J in T | all enabled |

These tables remain fixed if observations disagree; observed differences go beside them.

## MC2 dated results, 2026-11-11

Measured on this shared Darwin/arm64 machine, Rust `1.97.1`
(`8bab26f4f`, 2026-07-14), Stateright `0.31.0`, `CARGO_BUILD_JOBS=3`.
CPU model/count could not be queried through the sandbox, so no hardware identification is
inferred from MC1's run. Full runs used two checker threads; first refutations used one-thread
BFS. `time -p` under Bash measured **5.57 s** wall for `cargo test --release` (3.36 s incremental
compilation, 1.85 s unit execution), and **2.00 s** for the counts command (1.85 s unit execution).
Both passed **15 unit tests, zero failed, zero doc tests**: eight unchanged MC1 tests and seven
MC2 tests, including the two verifier regressions described below. Formatting and clippy with warnings denied passed. The intentionally caught
ceiling/timeout panics in the unchanged harness regression remain visible with `--nocapture`.
No run below was truncated. All 40 full MC2 configurations exhausted their state spaces;
every enabled harness and protocol witness was reached. Timing and depth are observations
from this particular counts run; concurrent BFS scheduling can change depth and timing.
No Linux/`ubuntu-latest` run is claimed.

The columns below give **every safety verdict** using the abbreviations above, followed by
all enabled reachability witnesses as `reached`. `Scope` means the extra-B configuration.

| Operation variant | Part | Unique states | Depth | Seconds | O | W | U | A | C | Reachability |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Plain | Timers | 420 | 21 | 0.002 | F | H | H | H | H | reached |
| Plain | Loss | 2,491 | 23 | 0.017 | F | H | H | H | H | reached |
| Plain | Reorder | 854 | 21 | 0.007 | F | H | H | H | H | reached |
| Plain | Duplication | 3,093 | 25 | 0.034 | F | H | H | H | H | reached |
| Plain | Partition | 1,260 | 23 | 0.012 | F | H | H | H | H | reached |
| Plain | Crash | 11,812 | 31 | 0.141 | F | H | H | H | H | reached |
| Plain | Lost | 1,657 | 24 | 0.011 | F | H | H | H | H | reached |
| Deduplicating | Timers | 384 | 21 | 0.003 | H | H | H | H | H | reached |
| Deduplicating | Loss | 2,353 | 23 | 0.017 | H | H | H | H | H | reached |
| Deduplicating | Reorder | 814 | 21 | 0.006 | H | H | H | H | H | reached |
| Deduplicating | Duplication | 2,891 | 25 | 0.028 | H | H | H | H | H | reached |
| Deduplicating | Partition | 1,152 | 23 | 0.011 | H | H | H | H | H | reached |
| Deduplicating | Crash | 13,145 | 31 | 0.150 | H | H | H | H | H | reached |
| Deduplicating | Lost | 1,552 | 24 | 0.007 | H | H | H | H | H | reached |
| Fenced | Timers | 333 | 19 | 0.003 | H | H | H | H | H | reached |
| Fenced | Loss | 1,993 | 21 | 0.020 | H | H | H | H | H | reached |
| Fenced | Reorder | 652 | 19 | 0.006 | H | H | H | H | H | reached |
| Fenced | Duplication | 2,263 | 24 | 0.022 | H | H | H | H | H | reached |
| Fenced | Partition | 999 | 21 | 0.006 | H | H | H | H | H | reached |
| Fenced | Crash | 6,549 | 25 | 0.077 | H | H | H | H | H | reached |
| Fenced | Lost | 1,185 | 22 | 0.006 | H | H | H | H | H | reached |
| BlindReplay | Timers | 384 | 21 | 0.004 | F | H | F | H | H | reached |
| VolatileIndex | Crash | 13,028 | 31 | 0.153 | F | H | F | F | F | reached |
| TimerFailed | Crash | 7,473 | 31 | 0.086 | F | F | H | H | H | reached |
| LateReply | Timers | 408 | 21 | 0.002 | F | H | F | H | H | reached |
| KeyOnly | Timers | 424 | 21 | 0.004 | F | H | H | H | F | reached |
| RepeatSucceeded | Timers | 533 | 23 | 0.005 | F | H | H | F | H | reached |
| Deduplicating | Scope | 2,628 | 25 | 0.028 | H | H | H | H | H | reached |

| Revision variant | Part | Unique states | Depth | Seconds | V | J | Reachability |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Checked | Timers | 426 | 15 | 0.003 | H | H | reached |
| Checked | Loss | 1,310 | 15 | 0.009 | H | H | reached |
| Checked | Reorder | 1,404 | 15 | 0.009 | H | H | reached |
| Checked | Crash | 2,864 | 17 | 0.020 | H | H | reached |
| Unchecked | Timers | 1,367 | 17 | 0.010 | F | H | reached |
| Unchecked | Loss | 4,825 | 18 | 0.034 | F | H | reached |
| Unchecked | Reorder | 4,927 | 17 | 0.037 | F | H | reached |
| Unchecked | Crash | 11,170 | 20 | 0.109 | F | H | reached |
| Concurrent | Timers | 1,671 | 15 | 0.006 | H | F | reached |
| Concurrent | Loss | 5,876 | 15 | 0.044 | H | F | reached |
| Concurrent | Reorder | 5,734 | 15 | 0.045 | H | F | reached |
| Concurrent | Crash | 10,502 | 17 | 0.089 | H | F | reached |

### Verifier fixes, 2026-11-11

The observed tables above were rerun after both admin verifier findings were fixed; the
pre-registered tables remain unchanged. The must finding now has a separate `ReplyEmitted`
ledger event per attempt. Lost-reply detection uses that event instead of an applied effect,
so a deduplicated attempt still becomes unknown when its reply is lost. Its regression
replays: deliver initial request; timer engine; timer A; deliver repeat; deliver dispatch 1;
deliver stale reply 1; deliver deduplicated dispatch 2; drop reply 2. With both timer credits
spent, lost-reply unknown is enabled and reconciliation records succeeded with one effect.
The test also rejects lost-reply detection before any reply, from only an older attempt's
completion, and while the current reply is still queued.

The should finding's regression gives both keyless labels one identity and one effect,
including a second request accidentally answered as a succeeded repeat. The witness rejects
it, and still rejects the shared identity if both decisions claim dispatch. Separate identities
without a second effect also fail; two dispatched identities with their own effects pass.
An effect without a dispatched decision for its script label does not qualify. Both new
regressions were run before the fixes and failed at the verifier's reported conditions.

Deduplicating state counts changed in L (2,120 to 2,353), R (788 to 814), D (2,840 to 2,891)
and C (11,449 to 13,145). The separate reply events also distinguish completion histories that
previously merged. All other unique-state counts are unchanged. The stronger keyless witness
is reached in every configuration. No additional verdict differs from pre-registration;
MC2-002 and MC2-003 below remain the only two differences. All first refutation traces below
were regenerated and are unchanged. No model, harness or specification behavior outside the
operation module was changed, and `retry.rs` remains unchanged.

### Differences from pre-registration

The pre-registered predictions above are unchanged. The initial compact lists of parts were
expanded to one row per configuration when recording results; no expected verdict changed.
There are exactly two differences:

| Finding | Variant/part | Property | Registered | Observed | Reason |
| --- | --- | --- | --- | --- | --- |
| MC2-002 | BlindReplay/T | U | H | F | A plain unknown invocation cannot start another attempt under OPS-012; blind replay also changes unknown to in progress illegally. |
| MC2-003 | VolatileIndex/C | U | H | F | After a timeout, losing the saved index lets an identical request dispatch again, changing unknown illegally. |

These are gaps in the predictions about deliberately broken variants, not new contradictions
in the correct rules. During implementation review the U predicate was tightened to require
that a new attempt from unknown is authorized by the immutable deduplication contract fact
in the ledger. An initial development run with a weaker predicate missed these two collateral
failures and is not the evidence table above. Tests compare each full verdict with the original
prediction plus exactly these two documented differences; any further discrepancy fails.

### First single-thread BFS traces

These are the first traces printed by separate named-property refutation runs. Positions are
per directed queue. `Next` advances A's script: the first such step finishes repeats; the
second sends c2. Applying a dispatch is atomic at implementation receipt. Reply delivery
ends the invocation at the engine and returns its outcome to the caller atomically.

MC2 refutation Plain: OPS-018 OPS-008 OPS-012 OPS-024: at most once (7 actions):

```text
1. delivered A -> engine [position 0]
2. timer A
3. timer engine
4. engine Reconcile(0)
5. delivered A -> engine [position 0]
6. delivered engine -> implementation [position 0]
7. delivered engine -> implementation [position 0]
```

MC2 refutation BlindReplay: OPS-018 OPS-008 OPS-012 OPS-024: at most once (6 actions):

```text
1. delivered A -> engine [position 0]
2. timer A
3. delivered engine -> implementation [position 0]
4. timer engine
5. delivered A -> engine [position 0]
6. delivered engine -> implementation [position 0]
```

MC2 difference refutation BlindReplay: OPS-013 IMP-007: unknown stays unknown (4 actions):

```text
1. delivered A -> engine [position 0]
2. timer A
3. timer engine
4. delivered A -> engine [position 0]
```

MC2 refutation VolatileIndex: OPS-018 OPS-008 OPS-012 OPS-024: at most once (7 actions):

```text
1. delivered A -> engine [position 0]
2. timer A
3. delivered engine -> implementation [position 0]
4. crashed engine
5. recovered engine
6. delivered A -> engine [position 0]
7. delivered engine -> implementation [position 0]
```

MC2 difference refutation VolatileIndex: OPS-013 IMP-007: unknown stays unknown (6 actions):

```text
1. delivered A -> engine [position 0]
2. timer A
3. timer engine
4. crashed engine
5. recovered engine
6. delivered A -> engine [position 0]
```

MC2 refutation TimerFailed: OPS-012 OPS-014: no wrong outcome (3 actions):

```text
1. delivered A -> engine [position 0]
2. delivered engine -> implementation [position 0]
3. timer engine
```

MC2 refutation LateReply: OPS-013 IMP-007: unknown stays unknown (4 actions):

```text
1. delivered A -> engine [position 0]
2. delivered engine -> implementation [position 0]
3. timer engine
4. delivered implementation -> engine [position 0]
```

MC2 refutation KeyOnly: OPS-010: different contents conflict (8 actions):

```text
1. delivered A -> engine [position 0]
2. timer A
3. delivered A -> engine [position 0]
4. delivered engine -> implementation [position 0]
5. delivered implementation -> engine [position 0]
6. A Next
7. A Next
8. delivered A -> engine [position 0]
```

MC2 refutation RepeatSucceeded: OPS-009 OPS-032: repeat answered without dispatch (5 actions):

```text
1. delivered A -> engine [position 0]
2. timer A
3. delivered engine -> implementation [position 0]
4. delivered implementation -> engine [position 0]
5. delivered A -> engine [position 0]
```

MC2 revision refutation Unchecked: REV-002 REV-011: revision checked (4 actions):

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. A Next
4. delivered A -> engine [position 0]
```

MC2 revision refutation Concurrent: REV-012: one change at a time (2 actions):

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
```

### What follows, and limits

**MC2-001: candidate unsafe contradiction, OPS-014/OPS-024 against OPS-018.** The seven-action
Plain trace has one identity. At step 3 its first dispatch is still queued when the timer
marks unknown. At step 4 an authorized actor reads zero effects and reconciles failed. At
step 5 OPS-024 dispatches the already queued repeat as another attempt. Steps 6 and 7 apply
both dispatches, giving two effects. This is experiment 004's inspect-then-reissue path under
the current promises. It requires no wire duplication of a dispatch, no fabricated evidence
and no lost saved state. The model leaves that path intact for the admin to judge. Neither
`spec/` nor a ruling was changed. The deduplicating implementation and stronger Fenced control
satisfy all five safety properties in every checked part.

The revision model satisfies revision comparison, missing-revision refusal and one-change
serialization; each corresponding seeded mutation is detected. This is bounded evidence
about these rule projections, not a proof about any engine, a proof at larger sizes, or a
liveness/fairness result. The fault parts do not explore simultaneous non-timer fault kinds;
the script suffix is serialized and scope fixes caller and target. No claim about all of
OPS-008's identity dimensions, outputs, defaults, access or control reconciliation follows.

| Property | S1 trace beside which it stands |
| --- | --- |
| O: effect at most once | S1-unknown-write, S1-reconcile |
| W: no wrong outcome | S1-reconcile |
| U: unknown changes only legally | S1-unknown-write |
| A: succeeded/in-progress repeat | S1-key-scope |
| C: changed contents conflict | S1-key-scope |
| Keyless separation and A/B sender scope | S1-key-scope |
| Reconciliation and unknown-source witnesses | S1-reconcile, S1-unknown-write |
| V: revision checked; J: one change at a time | None of these three S1 traces; revision contention is separate evidence |

The trace names refer to [S1's proposed safety cases](../../conformance/s1-safety-cases.md),
not to new conformance vectors. No vector was written. The
[safety catalogue](../../docs/implementation/SAFETY_CATALOGUE.md) was a placeholder when read;
there was no filled classification against which to check these promise IDs. Catalogue
coverage and the candidate contradiction need the admin's follow-up outside this order's
scope. The catalogue was filled in later on 2026-11-11 (work order 100). The lane
coordinator then checked the sixteen promises this order names: each is listed in it as a
safety promise. The original harness model and `retry.rs` tests remain unchanged and passing.
