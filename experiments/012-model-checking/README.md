# 012 — model-checking harness

Status: executed 2026-11-11 under work orders 104 (MC1), 105 (MC2), 107 (MC3), 108 (MC4) and 109 (MC5a), ruling 71's proof lane.
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

## MC3 access over time: model and pre-registration

Work order 107 adds [access.rs](src/access.rs), a reference model of the promises, not the
engine. The expected table and initial description preceded the first MC3 run. Later completion
of the emission ledger and explicit U reply relay changed state counts, not expectations.

A sends `op(k1)` through c1 and then `op(k2)` through c2, queued together in that order.
R queues `open(c2)`, `close(c1)`, `close(cT)` together, on an independent FIFO link to the
engine. c1 (A to T, `op`, one filter) and cT (T to U, `op2`) start open. c2 (A to T,
`op`, no filter) starts registered. cAU (A to U, `op2`) stays open; only Borrow misuses it.
The engine saves its Conduits and keyed invocation index. T starts registered and has one
setup step; readiness is lifecycle state and survives T's crash. A close removes its route,
never reopening it; c2 makes the separate opening possible. U is ready and states support
for c1's filter. T receives a dispatch, sends one onward request, waits for its answer,
and returns its own succeeded outcome regardless of the onward answer. T's crash clears
its active calls, preserves readiness and does not give dispatched calls unavailable.

The send to T or U is atomic with the access decision. Top-level copies are checked for
access before OPS-032/OPS-009: `op` declares an effect and no outputs, is Conduit aware,
and is not deduplicating. A repeat in progress conflicts, a succeeded repeat gives only
its outcome, and an unknown repeat stays unknown. No replay or reconciliation is modelled.
An engine timer ends one selected in-progress invocation unknown; timer selection is a
local nondeterministic action so any top-level or onward invocation can time out first.
Two timers are allowed; timer selectors 0/1 name top-level keys, 2/3 their onward calls. Recovery ends all in-progress invocations unknown, under IMP-007; a pending
recovery record is flushed before the engine's next decision or local work. A single
local unreadable/readable pair supplies the unreadable window in Window.

All six safety predicates read only the ledger. Per-key histories commute, avoiding an
irrelevant total order across keys. Open/close entries retain their Conduit and version.
Decisions retain current and read versions, readable state, readiness, openness, prior
keyed state, dispatch and the answer given. Invocation entries retain dispatch version,
end and cause, original answer at reply receipt and whether the route had closed.
Onward emission entries retain caller, filter, route, version, closed chain and ended
parent even if the wire request is lost before a decision. Separate onward decision
entries retain those snapshots at decision, readability, dispatch, answer and end cause.
Immediate reasons and completion answers are observed at the engine's reply boundary;
top-level transport of those answers is not another lossy link. U's reply travels back
to the engine and is relayed to T, on separately lossy links. Recovery ends in-progress
top-level and onward invocations unknown, preserving either end against a late reply.

### Configurations and bounds

Every row starts with both scripts and all four Conduits. Each correct model runs in
every part; mutants run only in their listed part. Faults are split to bound the product
of independent failures, rather than claim their simultaneous composition was checked.
Each run has a ceiling of 190,000 unique states, a 10-second timeout (both asserted),
three messages per directed link, two keys, at most one wire copy and one onward per
top-level invocation (at most four invocations total), three version changes and one
readiness transition. There are no unbounded
retries. Full runs exhaust the bounded graph even after finding a counterexample.

| Part | Enabled faults and local window |
| --- | --- |
| Base | None; all ordinary message and setup interleavings |
| Timers | Two timer actions, with either pending invocation selectable |
| ReorderA | One out-of-order delivery on A to engine |
| ReorderService | One out-of-order delivery between engine, T and U |
| LossA | One lost request on A to engine |
| LossService | One lost message between engine, T and U |
| Copies | One duplicated request on A to engine |
| EngineCrash | One engine crash and recovery; Conduits and key index saved |
| TargetCrash | One T crash and recovery; readiness persists |
| Cut | One engine/T cut; both directions held |
| Window | One unreadable/readable pair, no network fault |

R's link cannot reorder or lose messages: the order requires opening c2 before closing
c1, and isolates request/close races from failures of R's management script. R's messages
cannot duplicate either. Only A's requests duplicate, to exercise keyed repeats without
introducing implementation idempotency into this access model. Reordering and loss are
restricted to the named links to keep those independent products small. A, R and U do
not crash: only engine recovery and loss of T's active calls are this order's faults.
Permanent machine loss and healing are omitted: lifecycle belongs to MC's lifecycle model,
and a held-link cut already suffices for the safety races; eventual reconnection is no
liveness claim. T/U traffic is mediated: T-to-engine-to-U and U-to-engine-to-T. There
is no direct T/U wire queue; LossService covers every leg of those logical calls. Only Window enables the unreadable pair and only Timers enables timers,
to isolate state-read and unknown-outcome interactions. All other budgets are zero with
reasons in `Config`. Nonzero budgets are one, except two timers.

The sole harness change consults `fault_allowed` for reordering and loss as well as its
existing fault classes. FIFO delivery remains unconditional on this hook. MC2 inherits
`true` for both classes, so its transition graphs are unchanged; a harness test checks
both the restricted MC3 links and MC2's unchanged permissions.

### Catalogue accounting (44 promises)

“Modelled” means only the slice named here. The property keys below match the following
table; other obligations of that same promise remain outside this bounded model.

| Promise | Status | Property or reason |
| --- | --- | --- |
| ACC-001 | Modelled | C |
| ACC-002 | Modelled | C (decision before atomic send) |
| ACC-003 | Not modelled | a static decision, to be shown by vectors |
| ACC-005 | Not modelled | belongs to the lifecycle model |
| ACC-006 | Not modelled | a static decision, to be shown by vectors |
| ACC-008 | Not modelled | filters are judged by the implementation (IFC-019); this model checks carriage only |
| ACC-009 | Not modelled | a static decision, to be shown by vectors |
| ACC-011 | Modelled | C |
| ACC-012 | Not modelled | a static decision, to be shown by vectors |
| ACC-014 | Not modelled | implementation restriction evaluation and its diagnostic need a filter implementation, absent here |
| ACC-015 | Not modelled | unsupported-filter path is outside this model; U explicitly supports the carried requirement |
| ACC-016 | Modelled | O |
| ACC-017 | Modelled | O (identity and carried filter) |
| ACC-018 | Not modelled | belongs to the lifecycle model |
| ACC-019 | Modelled | O |
| ACC-023 | Not modelled | a static decision, to be shown by vectors |
| ACC-024 | Not modelled | a static decision, to be shown by vectors |
| ACC-027 | Not modelled | inspection output, absent from the transition alphabet |
| ACC-028 | Not modelled | a static decision, to be shown by vectors |
| ACC-029 | Not modelled | a static decision, to be shown by vectors |
| ACC-030 | Not modelled | a static decision, to be shown by vectors |
| ACC-032 | Not modelled | a static decision, to be shown by vectors |
| ACC-033 | Not modelled | a static decision, to be shown by vectors |
| ACC-034 | Modelled | F, O |
| ACC-036 | Modelled | C |
| ACC-037 | Modelled | U |
| ACC-038 | Not modelled | inspection output naming the enforcing implementation, absent here |
| ACC-039 | Not modelled | belongs to the lifecycle model |
| ACC-044 | Not modelled | provenance, inspection, inventory and tombstone retention require a separate record model |
| ACC-053 | Not modelled | a static decision, to be shown by vectors |
| ACC-054 | Not modelled | a static decision, to be shown by vectors |
| ACC-055 | Not modelled | a static decision, to be shown by vectors |
| ACC-057 | Not modelled | a static decision, to be shown by vectors |
| ACC-056 | Not modelled | disclosure tiers and diagnostic contents are not represented |
| CON-004 | Not modelled | belongs to the lifecycle model |
| CON-008 | Not modelled | belongs to the lifecycle model |
| CON-011 | Not modelled | what an inspection shows, absent from the transition alphabet |
| CON-013 | Modelled | P (literal dispatch reading) |
| CON-014 | Not modelled | belongs to the lifecycle model |
| CON-023 | Not modelled | belongs to the lifecycle model |
| CON-024 | Modelled | N |
| CON-031 | Not modelled | belongs to the lifecycle model |
| FLEET-004 | Not modelled | a static decision, to be shown by vectors |
| FLEET-005 | Not modelled | default grant creation and its visibility-filtered collision diagnostics are absent |

Counts: 11 modelled, 16 static, 8 lifecycle, 9 other; total 44.

Static exclusions include the complete operation list and reserved secret reads, management
bounds and delegation, ownership and Fleet authority, root-only operations, tombstone lift,
registration and reconciliation. c2's lack of a filter is not checked against c1's filter:
that independence is fixed input, not a proof of all of ACC-009. Inspection, tool listing,
wire schemas, authentication, invalid arguments, diagnostic disclosure and outputs are
absent. There is no claim of checking IFC-012/013/019/025 beyond their abstract reasons,
caller identity and carried-filter facts. Invalid is an answer in the model's vocabulary,
but no input produces it. Setup/retirement hooks, consumer retirement and chain depth
beyond T to U await the lifecycle and composition models. The unreadable window concerns
Conduit state, not credential existence; its implementation-defined wait is abstracted
as immediate unavailable. No filter predicate or unsupported-version argument is invented.

### Expected table, fixed before execution

Safety keys: C = current access (ACC-001/002/011/036); P = open permits dispatch (CON-013);
F = in flight (ACC-034); O = onward ownership, caller, filter, chain and ended-parent
(ACC-016/017/019/034); U = unreadable unavailable (ACC-037); N = registered unavailable
(CON-024). H means holds, X refuted. Reach keys: r = refused after close, f = outcome
after close, o = onward dispatched, c = onward refused after chain close, u = unreadable
unavailable, n = registered unavailable. R means reached, dash means not reached.
Every enabled harness fault witness is expected reached in every row.

The order's literal P requires dispatch even for an authorized keyed repeat, although
OPS-032 forbids dispatch. Literal F requires T's outcome whenever its reply arrives,
although OPS-013 and IMP-007 preserve an earlier unknown outcome. These anticipated
counterexamples are kept, not hidden with an exemption. They concern the property as
written in the order, and require interpretation against the full promises.

| Configuration / variant | C | P | F | O | U | N | r | f | o | c | u | n |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Base / Correct | H | H | H | H | H | H | R | R | R | R | - | R |
| Timers / Correct | H | H | X | H | H | H | R | R | R | R | - | R |
| ReorderA / Correct | H | H | H | H | H | H | R | R | R | R | - | R |
| ReorderService / Correct | H | H | H | H | H | H | R | R | R | R | - | R |
| LossA / Correct | H | H | H | H | H | H | R | R | R | R | - | R |
| LossService / Correct | H | H | H | H | H | H | R | R | R | R | - | R |
| Copies / Correct | H | X | H | H | H | H | R | R | R | R | - | R |
| EngineCrash / Correct | H | H | X | H | H | H | R | R | R | R | - | R |
| TargetCrash / Correct | H | H | H | H | H | H | R | R | R | R | - | R |
| Cut / Correct | H | H | H | H | H | H | R | R | R | R | - | R |
| Window / Correct | H | H | H | H | H | H | R | R | R | R | R | R |
| Copies / Reuse | X | X | H | H | H | H | R | R | R | R | - | R |
| Base / CloseRefused | H | H | X | H | H | H | R | - | R | R | - | R |
| Base / Borrow | H | H | H | X | H | H | R | R | R | R | - | R |
| Base / Chain | H | H | H | X | H | H | R | R | R | R | - | R |
| Base / DropFilter | H | H | H | X | H | H | R | R | R | R | - | R |
| Base / Caller | H | H | H | X | H | H | R | R | R | R | - | R |
| Timers / Ended | H | H | X | X | H | H | R | R | R | R | - | R |
| Window / ReadOpen | H | H | H | H | X | H | R | R | R | R | - | R |
| Base / Registered | H | H | H | H | H | X | R | R | R | R | - | - |

Reuse keeps an earlier operation decision, including its old version. CloseRefused
answers the original call refused when c1 closes. Borrow uses A's cAU after cT closes.
Chain checks cT alone. DropFilter and Caller separately corrupt filter and identity.
Ended ignores the ended parent. ReadOpen treats unreadable access as open. Registered
sends work before setup. Tests compare every full verdict and witness with this table;
separate single-thread BFS runs print the first counterexample for each mutant, plus
the literal-property counterexamples on Correct. Ledger-only regression checks erase
all node state after each mutant refutation and ensure the failure remains visible.

### Supplemental control, registered before its first run

Q = `ACC-034 OPS-013 IMP-007: control preserves recorded end`. This labelled control
stands beside literal F. It requires the original response at T's reply receipt to give
the recorded unknown where a timer, engine recovery or observed T crash already ended
the invocation unknown, and T's outcome otherwise. Its expected verdict is H in all 20
rows above except Base / CloseRefused, where it is X. It does not replace F. A separate
refutation prints the first trace on Q for CloseRefused. Catalogue ACC-034's modelled
slice thus includes F, O and Q. This control makes no ruling about changing the promises.

T's crash leaves a pending observation that recovery cannot erase. The engine applies it
before its next decision, or as a priority local action, ending pending calls unknown,
never unavailable. While T remains down the observation remains pending, including for
calls dispatched to the still-ready target in that interval; after recovery the engine
clears it only after applying it. The first draft omitted the observation; implementing
the order's unknown-on-crash requirement adds a late-reply failure to F in TargetCrash.
The original table remains unchanged. Setup requires T to be running.

## MC3 dated results, 2026-11-11

Measured on this shared arm64 macOS machine, Rust `1.97.1`, Stateright `0.31.0`.
No processor model is claimed: that system query was unavailable. Full runs used two
checker threads; refutations used one-thread BFS. The measured ordinary release command
passed 19 tests and zero doc tests in **11.99 seconds wall time**
(the test body took 10.56 seconds). The counted `--nocapture --test-threads=1`
command passed the same totals in **10.04 seconds wall time**, 9.88 seconds in tests.
These include MC1 and MC2 unchanged. No Linux timing was measured; the local times leave
substantial margin for the order's roughly one-minute Ubuntu target.

Each row is a full bounded run from the counted command. Safety letters are ordered
**C P F O U N Q**, reach letters **r f o c u n**; every enabled harness fault witness
was reached. Thus every safety verdict and reach verdict, including the Q control,
is stated for every row. Depth includes the initial state. The ceiling and timeout
were asserted after each full run; none was truncated.

Verifier refresh, 2026-11-11: the prior implementation enabled `TargetUnknown` only
while T was crashed, so recovery could skip the unknown end and permit queued onward
work. The pending observation now survives recovery and is flushed before further
engine decisions or local work. The six-action regression is `SetupT`; deliver k1 to
the engine; deliver its invocation to T; crash T; recover T; deliver its queued onward
request. It failed before the fix and now records the parent's unknown end and refuses
onward dispatch, with readiness preserved. All rows and refutation traces below were
refreshed. TargetCrash changes from 36,633 states / depth 22 to 51,529 / depth 21; every
other state count and depth, all verdicts and reachability, and both recorded table
differences stay the same. No timing-test rerun with `RUST_TEST_THREADS=2` was needed.

| Configuration / variant | Unique states | Depth | Seconds | Observed safety C P F O U N Q | Observed reach r f o c u n |
| --- | --- | --- | --- | --- | --- |
| Base / Correct | 3,228 | 19 | 0.026 | H H H H H H H | R R R R - R |
| Timers / Correct | 120,264 | 23 | 2.054 | H H X H H H H | R R R R - R |
| ReorderA / Correct | 4,360 | 19 | 0.043 | H H H H H H H | R R R R - R |
| ReorderService / Correct | 7,148 | 19 | 0.072 | H H H H H H H | R R R R - R |
| LossA / Correct | 3,517 | 19 | 0.029 | H H H H H H H | R R R R - R |
| LossService / Correct | 7,938 | 19 | 0.096 | H H H H H H H | R R R R - R |
| Copies / Correct | 27,629 | 21 | 0.310 | H X H H H H H | R R R R - R |
| EngineCrash / Correct | 35,259 | 21 | 0.454 | H H X H H H H | R R R R - R |
| TargetCrash / Correct | 51,529 | 21 | 0.671 | H H X H H H H | R R R R - R |
| Cut / Correct | 6,456 | 20 | 0.067 | H H H H H H H | R R R R - R |
| Window / Correct | 19,233 | 21 | 0.221 | H H H H H H H | R R R R R R |
| Copies / Reuse | 40,817 | 21 | 0.511 | X X H H H H H | R R R R - R |
| Base / CloseRefused | 3,228 | 19 | 0.025 | H H X H H H X | R - R R - R |
| Base / Borrow | 3,854 | 19 | 0.034 | H H H X H H H | R R R R - R |
| Base / Chain | 3,808 | 19 | 0.033 | H H H X H H H | R R R R - R |
| Base / DropFilter | 3,228 | 19 | 0.025 | H H H X H H H | R R R R - R |
| Base / Caller | 3,228 | 19 | 0.030 | H H H X H H H | R R R R - R |
| Timers / Ended | 150,702 | 23 | 2.322 | H H X X H H H | R R R R - R |
| Window / ReadOpen | 47,984 | 21 | 0.541 | H H H H X H H | R R R R R R |
| Base / Registered | 11,726 | 19 | 0.120 | H H H H H X H | R R R R - - |

### Findings and differences from the registered table

**MC3-001 — gap in the literal open-dispatch property's scope (CON-013, OPS-009,
OPS-032, ACC-036).** Correct/Copies refutes P: an authorized repeat in progress
reports conflict rather than dispatch. The first trace has five actions below. CON-013
speaks of the next request through a newly opened Conduit, while the order's predicate
speaks of every request through an open one, with no repeat exemption. This trace uses
c1, which starts open, rather than a first request after opening c2. It does not establish
that the specification is inconsistent. It establishes that literal P cannot serve as an
unqualified proxy for CON-013. No property or promise was weakened to hide the failure.
The expected failure in Copies (Correct and Reuse) was registered, so it is no table delta.

**MC3-002 — gap in the literal in-flight property's treatment of an ended unknown
(ACC-034, OPS-013, IMP-007).** Correct/Timers and Correct/EngineCrash refute F in
eight actions each. An invocation ends unknown, then T's late reply reaches the engine;
the original request remains answered unknown. The first traces do not need a close.
The order's unconditional reply/outcome predicate omits both ACC-034's close-while-in-progress
precondition and OPS-013's earlier unknown end. ACC-034 says the result is delivered as
for any other invocation; this model supplies no evidence that a closed route permits
turning an already ended unknown into succeeded. Q holds in every Correct configuration,
and CloseRefused refutes Q as well as F. This is a property-scope finding, not a claim of
an unsafe specification contradiction. Timers and EngineCrash failures were registered.

**MC3-003 — expectation gap, ReadOpen/Window (ACC-037, CON-024).** The table expected
u not reached; observed u reached. When state is unreadable and T is registered, the
mutant skips the unreadable check but still returns unavailable on readiness. The u
witness records the unreadable snapshot and answer; it does not isolate the reason's
branch. This overlap does not make U hold: the separate three-action trace makes T ready
and dispatches during the unreadable window, refuting U. The table remains unchanged;
the test checks the recorded observation beside its registered expectation.

**MC3-004 — expectation gap, Correct/TargetCrash (ACC-034, OPS-013, LIF-011).** The
table expected F to hold; observed F refuted. The target's crash must allow an in-progress
call to end unknown without changing readiness. That observation was missing from the
first draft and was added to apply the order, exposing rather than hiding a failure.
The first trace crashes T after dispatch but before it receives the queued call. T
recovers and receives the call; its onward request reaches the engine, which first
records the pending crash's unknown end, then refuses the onward request because the
parent ended. T still answers its own outcome. The eight-action first trace below
refutes F; Q still holds. This is
the same missing unknown qualification as MC3-002, on a third path. The registered table
remains unchanged, and the test checks this separately recorded observed verdict.

There were no other differences. All nine mutants refuted their named property. Missing
u outside Window is the disabled read-fault stimulus; CloseRefused also loses f, and
Registered loses n, as registered. All 44 catalogue rows are accounted for: **11 modelled,
16 not modelled as static decisions to be shown by vectors, 8 belonging to the lifecycle
model, 9 with the other individual reasons above**.

### First single-thread BFS traces

These are the first traces printed by the counted command, not traces inferred from a
full run. Each begins from the same initial scripts. Successive FIFO R deliveries are
open c2, close c1 and close cT; successive A deliveries follow that part's actual queue,
including an appended copy in Copies. An engine-to-T message is either the initial
invocation or its onward answer; the surrounding T-to-engine deliveries distinguish them.

**MC3 refutation Reuse: ACC-001 ACC-002 ACC-011 ACC-036: current access**

```text
1. engine SetupT
2. delivered A -> engine [position 0]
3. delivered A -> engine [position 0]
```

**MC3 refutation CloseRefused: ACC-034: in flight runs to its end**

```text
1. delivered R -> engine [position 0]
2. engine SetupT
3. delivered A -> engine [position 0]
4. delivered R -> engine [position 0]
5. delivered engine -> T [position 0]
6. delivered T -> engine [position 0]
7. delivered engine -> T [position 0]
8. delivered T -> engine [position 0]
```

**MC3 refutation Borrow: ACC-016 ACC-017 ACC-019 ACC-034: target owns onward access**

```text
1. delivered R -> engine [position 0]
2. engine SetupT
3. delivered A -> engine [position 0]
4. delivered R -> engine [position 0]
5. delivered R -> engine [position 0]
6. delivered engine -> T [position 0]
7. delivered T -> engine [position 0]
```

**MC3 refutation Chain: ACC-016 ACC-017 ACC-019 ACC-034: target owns onward access**

```text
1. delivered R -> engine [position 0]
2. engine SetupT
3. delivered A -> engine [position 0]
4. delivered R -> engine [position 0]
5. delivered engine -> T [position 0]
6. delivered T -> engine [position 0]
```

**MC3 refutation DropFilter: ACC-016 ACC-017 ACC-019 ACC-034: target owns onward access**

```text
1. engine SetupT
2. delivered A -> engine [position 0]
3. delivered engine -> T [position 0]
4. delivered T -> engine [position 0]
```

**MC3 refutation Caller: ACC-016 ACC-017 ACC-019 ACC-034: target owns onward access**

```text
1. engine SetupT
2. delivered A -> engine [position 0]
3. delivered engine -> T [position 0]
4. delivered T -> engine [position 0]
```

**MC3 refutation Ended: ACC-016 ACC-017 ACC-019 ACC-034: target owns onward access**

```text
1. engine SetupT
2. delivered A -> engine [position 0]
3. delivered engine -> T [position 0]
4. engine SelectTimer(0)
5. timer engine
6. delivered T -> engine [position 0]
```

**MC3 refutation ReadOpen: ACC-037: unreadable is unavailable**

```text
1. engine SetupT
2. engine Unreadable
3. delivered A -> engine [position 0]
```

**MC3 refutation Registered: CON-024: registered is unavailable**

```text
1. delivered A -> engine [position 0]
```

**MC3 control refutation CloseRefused: ACC-034 OPS-013 IMP-007: control preserves recorded end**

```text
1. delivered R -> engine [position 0]
2. engine SetupT
3. delivered A -> engine [position 0]
4. delivered R -> engine [position 0]
5. delivered engine -> T [position 0]
6. delivered T -> engine [position 0]
7. delivered engine -> T [position 0]
8. delivered T -> engine [position 0]
```

**MC3 literal promise refutation Copies: CON-013: open permits dispatch**

```text
1. duplicated A -> engine [position 0]
2. engine SetupT
3. delivered A -> engine [position 0]
4. delivered A -> engine [position 0]
5. delivered A -> engine [position 0]
```

**MC3 literal promise refutation Timers: ACC-034: in flight runs to its end**

```text
1. engine SetupT
2. delivered A -> engine [position 0]
3. delivered engine -> T [position 0]
4. engine SelectTimer(0)
5. timer engine
6. delivered T -> engine [position 0]
7. delivered engine -> T [position 0]
8. delivered T -> engine [position 0]
```

**MC3 literal promise refutation EngineCrash: ACC-034: in flight runs to its end**

```text
1. engine SetupT
2. delivered A -> engine [position 0]
3. delivered engine -> T [position 0]
4. crashed engine
5. recovered engine
6. delivered T -> engine [position 0]
7. delivered engine -> T [position 0]
8. delivered T -> engine [position 0]
```

**MC3 literal promise refutation TargetCrash: ACC-034: in flight runs to its end**

```text
1. engine SetupT
2. delivered A -> engine [position 0]
3. crashed T
4. recovered T
5. delivered engine -> T [position 0]
6. delivered T -> engine [position 0]
7. delivered engine -> T [position 0]
8. delivered T -> engine [position 0]
```

### What MC3 shows, and what it does not

Within each separate fault bound, fresh versioned access, removal of a closed route,
caller and filter carriage, the sender's own onward authority, all links of its chain,
ended-parent rejection, unreadable fail-closed decisions and readiness checks can hold
together. Every required outcome is reachable where its stimulus is enabled. Deliberate
violations are detected through external ledger facts even after node data is erased.
The P and F scope findings remain visible beside Q; they must not be counted as proof
that the literal properties hold. The 11 modelled promises are slices, not 11 complete
proofs. MC8 must retain these qualifications and the 33 explicit exclusions.

This is bounded safety and reachability evidence about the reference model. It proves
neither the engine nor implementations conform, nor unbounded behavior, fairness,
progress, reliable delivery, compound faults, long chains, consumer retirement,
filter enforcement or the excluded static rules. The engine-boundary observation of answers excludes transport losses after a top-level
answer. Onward replies and their relay remain explicitly lossy.

| Property | S1 traces it stands beside |
| --- | --- |
| C: fresh decision and closed-route refusal | S1-exact-access, S1-close, S1-reason-order (access before repeat lookup only) |
| P: open route (scope finding retained) | S1-exact-access; also CON-013's opening rule, for which S1 has no dedicated trace |
| F and Q: original result across a close and recorded unknown | S1-close-in-flight; S1-close distinguishes the later repeat |
| O: own onward route, caller, filter, chain, live parent | S1-close-in-flight, E08-broker (filter carriage only; no broker or unsupported-filter behavior) |
| U: unreadable state unavailable | S1-authority-unreadable (Conduit state only; no credential-existence or diagnostic-disclosure claim) |
| N: registered target unavailable | S1-target-registered |

All changed Rust and documentation files are confined to this experiment. The pinned
lockfile and toolchain remain in force, and `target/` remains ignored.

## MC4 placement and a lost machine: model and pre-registration

Registered 2026-11-11 under work order 108, **before any MC4 run**. The model is
[`src/placement.rs`](src/placement.rs); it follows the promises, independently of engine code.
M1 holds the engine, its local provider servers and consumer A. M2 holds I, J and later I′,
and the runtime provider's independent keeper. These are three harness nodes, not three
machines. The engine saves its script and object state; its provider servers end with its
crash and return on recovery. The keeper's durable stop mailbox belongs to the provider,
not the engine's saved instance records. The mailbox abstracts the provider contract's
custody of an accepted stop; it is not a claim about a replication or networking protocol.
A deliberately volatile variant keeps it only in the local server's volatile state instead.

The machine provider reports M2 and registers its node. Placement calls name M2 for I and J,
with matching selectors; their node's setup is a separate later step. An early setup probes
the registered/not-ready interval. The engine then sets I and J up, A sends two independent
requests to I, and an explicit further setup inspects I again. I may exit and be restarted by
its provider, without a new caller request, dispatch or secret read. The script retires I
with its selected retire hook under TPL-018, then unregisters it under LIF-036. I′ may then
be registered and placed. J and I′ are finally removed and the provider is retired; its
independent keeper can still carry out its accepted stops. The root Interface declares
M2's node lost once in Declared. Gone allows physical disappearance and a new machine under
the same hostname. Permanent loss is never harness recovery of the old M2.

Provider reports and inspect-run answers are untimed local actions. A machine report is
enabled when physical state or the engine's epoch changes. The engine's latest received
report, rather than the cut itself, determines unreachable: explicit unreachable, omission,
or no report since start all count. A cut before the next report can therefore leave the
engine acting on an earlier reachable report. Inspect-run answers record both value and
truth at their observation time. Each status names the report and retains its heartbeat
time; unavailable observations report Unknown. The external ledger keeps per-instance
histories (independent instances commute), machine reports, node registrations, declarations
and observations of server lifecycle. Each run, exit, stop, provider call and answer,
late stop, request dispatch and answer, retirement, hook and secret read is recorded.
Properties read only that ledger; refutation tests erase all node data and recheck failure.

### MC4 promise accounting

Each of the 22 catalogue promises appears once in this accounting. “Modelled” means the
bounded slice described here, not every clause or input representation of a promise.

| Promise | Account | Property or reason |
| --- | --- | --- |
| PLC-002 | Modelled | P5, provider names placement |
| PLC-012 | Not modelled | Co-location authorization is a static decision, to be shown by vectors; MC3 covers access races |
| PLC-017 | Modelled | P3, declaration retires placed instances with shutdown unknown |
| PLC-019 | Modelled | P7, registered node blocks setup |
| PLC-020 | Not modelled | Node-local path interpretation is a static decision, to be shown by vectors |
| PLC-023 | Modelled | P2, reports determine unavailability without retirement |
| PLC-024 | Modelled | P3, provider gone declares loss |
| PLC-026 | Modelled | P8, registration and fresh node IDs |
| PLC-027 | Modelled | P3, no lost-node hook, run or secret read |
| PRV-002 | Modelled | P9, engine and local provider server lifecycle |
| PRV-005 | Not modelled | Provider metadata/caller fields are a static decision, to be shown by vectors |
| PRV-007 | Modelled | P1, no run asked on an unknown inspect answer |
| PRV-008 | Modelled | P4, addressed stops and unknown answers |
| PRV-009 | Modelled | P6, status provenance and time |
| PRV-010 | Modelled | P1, provider restart makes no dispatch or secret read |
| PRV-012 | Not modelled | Unsupported artifact/transport rejection is a static decision, to be shown by vectors |
| PRV-014 | Modelled | P1/P2, cut is not evidence of gone |
| PRV-015 | Modelled | P5, M2 selected by provider |
| PRV-019 | Modelled | P5, runtime executes only on selected machine |
| PRV-021 | Modelled | P4, accepted stop survives restart and retirement |
| PRV-027 | Modelled | P4, stop instance and placement machine |
| POOL-016 | Not modelled | Needs a Pool and repeats placement already checked in P5 |

Counts: **17 modelled; 4 static decisions for vectors; 1 needs a Pool**. Supporting promises,
outside those counts: PLC-009 (placement fixed for life), LIF-025 (never set up), LIF-036
(unregister asks stop despite unreachability), TPL-018 (retire hook), OBS-003 (unknown status).
Enrollment is **spec pending, needed before S3**; its promises are not yet written and are
not supplied by this model or counted among the 22.

### MC4 properties, configurations and bounds

P1 = at most one live run of I and I′ together, no run asked after unknown and no
restart-triggered dispatch; root's explicit lost-node risk is excepted. P2 = unreachable is
not gone (no retirement for silence, request dispatch, run or secret read). P3 = lost-node
retirement has unknown shutdown and no hook/run/read (never-set-up and no-hook cases are
excepted). P4 = stops are addressed and only the named run is stopped; at quiescence every
stop answered unknown is carried. Quiescence requires reachable M2, no wire message and no
enabled action, including an unused enabled fault. The observation of quiescence itself is
excluded. P5 = runs only where placed. P6 = Running True/False uses a report true when
made, with its time; unreachable or unobserved means Unknown. P7 = setup waits for a ready
node. P8 = returned machine gets a new node. P9 = provider servers end with engine.

All runs use three harness nodes, three instance IDs, two independent consumer requests,
one exit/restart, a finite 17-step script, at most six messages per directed link, a
**190,000 unique-state ceiling and 10-second timeout**, checked both during and after each
full run. No configuration truncates an exhaustive search into a pass. Each nonzero fault
budget is one. Provider calls on M1 stdio are sent once: duplication budget zero. There
are no timers: report/inspect/stop actions are enabled by changed observations or obligations.
The existing models remain unchanged. The harness now consults `fault_allowed` for heal,
recovery and timers too, in addition to its existing fault checks.

| Configuration | Enabled faults | Why restricted |
| --- | --- | --- |
| Base | None | Ordinary script and message interleavings |
| Partition | Cut and heal, engine↔M2 only | M1-local stdio and A's M1 link do not partition |
| Combined | Cut/heal engine↔M2 and crash/recovery engine only | All four together exercise volatile-stop loss |
| LossA | One loss, A↔engine | Isolates consumer request/response loss |
| LossM2 | One loss, engine↔M2 | Isolates runtime transport loss |
| ReorderA | One reorder, A↔engine | Isolates the two independent requests |
| ReorderM2 | One reorder, engine↔M2 | Isolates runtime message ordering |
| Gone | M2 lost once | M1 and A are not the lost machine |
| Declared | Root declaration; cut/heal engine↔M2 | Allows return after explicit declaration |

A never crashes; only the engine has crash/recovery. M2 loss is distinct from engine crash,
which leaves remote runs and saved obligations intact. Loss and reordering affect both
directions of the named link, and no unrelated link. No new infrastructure, access policy,
secrets transport, provider metadata schemas, enrollment, real clocks, unbounded retries or
restart policies are proved. Stop custody is an abstract durable mailbox; S3 must establish
that a real provider implements it, including acceptance during a partition and after removal.
The node itself dispatches nothing on setup: whether LIF-025 therefore confirms its shutdown
on declaration of loss is ambiguous against the default unknown in PLC-017. This model
records node retirement separately and checks the setup/hook exceptions on placed instances;
it does not settle that ambiguity.

### MC4 expected table, registered before execution

H = holds; F = refuted. The nine-character vector orders P1 through P9. Reach columns
order R = unreachable then reachable, G = provider gone, D = root declaration, L = late
stop, U = unavailable request during cut, X = exit/restart, I = I′ placed, N = new node on
return, Q = unknown run blocks a call, B = not-ready setup unavailable, K = stop carried
after provider retirement. `+` means reached, `-` means absent. Enabled harness faults
must all be reached too. Correct runs in every part; each mutant runs only in its listed
part. Tests compare all properties and all reaches with this table; any observed difference
is recorded beside it, without changing these registered expectations.

| Variant/configuration | P1…P9 | R G D L U X I N Q B K |
| --- | --- | --- |
| Correct/Base | HHHHHHHHH | - - - + - + + - - + + |
| Correct/Partition | HHHHHHHHH | + - - + + + + - + + + |
| Correct/Combined | HHHHHHHHH | + - - + + + + - + + + |
| Correct/LossA | HHHHHHHHH | - - - + - + + - - + + |
| Correct/LossM2 | HHHHHHHHH | - - - + - + + - - + + |
| Correct/ReorderA | HHHHHHHHH | - - - + - + + - - + + |
| Correct/ReorderM2 | HHHHHHHHH | - - - + - + + - - + + |
| Correct/Gone | HHHHHHHHH | - + - + - + + + - + + |
| Correct/Declared | HHHHHHHHH | + - + + + + + + + + + |
| UnreachableGone/Partition | FFHHHHHHH | + - - + + + + - + + + |
| UnknownRun/Partition | FHHHHHHHH | + - - + + + + - + + + |
| VolatileStop/Combined | HHHFHHHHH | + - - + + + + - + + + |
| WrongStop/Base | HHHFHHHHH | - - - + - + + - - + + |
| ConfirmLost/Declared | HHFHHHHHH | + - + + + + + + + + + |
| LostHook/Declared | HHFHHHHHH | + - + + + + + + + + + |
| WrongMachine/Base | HHHHFHHHH | - - - + - + + - - + + |
| StaleRunning/Partition | HHHHHFHHH | + - - + + + + - + + + |
| EarlySetup/Base | HHHHHHFHH | - - - + - + + - - - + |
| ReuseNode/Declared | HHHHHHHFH | + - + + + + + - + + + |

The first attempted Base exploration hit the 190,000-state ceiling (no verdict was
interpreted). Before the next run, the script window was made explicit: A sends at step 6;
the script waits for ordinary exchanges, or for an unavailable report, before proceeding.
It also waits for ordinary run replies while the machine is reported reachable and for the
retirement's kept stop before unregistering while reachable. An unregister stop remains
allowed during a cut, and replacement does not wait for it. These are script bounds, not
extra language rules. The registered expected table above stays unchanged.

For compatibility, `gate_remaining_faults` opts a protocol into the new heal/recovery/timer
permissions. The hook is consulted in both modes; earlier models keep their original
ungated behavior for these three classes. MC4 opts in, so every fault kind is restricted
through `fault_allowed`. A regression tests both modes. This avoids changing MC3, whose
existing hook deliberately returned false for classes the old harness never consulted.

Partition also exceeded the ceiling after the explicit script window. It is now split by
script phase, with the added `PartitionRetire` configuration. A default-true
`fault_enabled` harness hook bounds the scenario window without changing any earlier model.
Partition cuts at step 6 and heals at step 7; PartitionRetire cuts at step 11 and heals at
step 13. Declared cuts at step 14, after I is unregistered and I′ is placed; root declares
loss there, and the cut heals at step 15. I′ inherits I's selected retire hook. Combined cuts at step 11, unregisters at step 12, crashes and recovers at step 13,
then heals there. Script advancement waits at those heal barriers. Gone permits M2 loss at
step 10. Faults remain optional, but any executed fault is confined to its stated window;
there is no claim about other crash/cut timings. Every enabled fault must have a witness.
Combined still includes all four required faults in one execution. Loss/reorder parts keep
the whole script. These windows and the extra row below were registered before their runs.

| Added configuration | P1…P9 expected | R G D L U X I N Q B K expected |
| --- | --- | --- |
| Correct/PartitionRetire | HHHHHHHHH | + - - + - + + - + + + |

The healed suffix also waits for the provider's changed machine report before the next
script step; a cut can still race before its report. Independent consumer histories commute
in the ledger, retaining each dispatch/answer and its observation facts. These state-space
reductions retain the observed Base non-overlap failure; no safety behavior is changed to
make it pass. Earlier ceiling-rejected runs yielded no Partition verdict.

A provisional Base refutation exposed a modelling defect, not a promise counterexample:
the transition asked for a run to serve a hook and stopped it before waiting for the run
and hook. TPL-018 orders them. The corrected model has a separate `FinishRetire` action,
waits for the run answer (or an unavailable observation), then dispatches the hook and asks
for the stop. Returned nodes also have a separate setup action before the script proceeds.
The provisional Base failure is withdrawn; its counts are not final results. The expected
table is unchanged. Any aggregate I/I′ overlap that remains is checked on the corrected
ordering, especially an unknown unregister stop during the retirement partition.

The final loss handling also separates hook suppression from stop suppression: a further
retirement on a lost node still asks for its stop when the latest machine report permits it;
unregister always asks. The UnreachableGone mutant explicitly recreates I′ on the returned
node before stopping old I, as its registered broken behavior requires. It does not inherit
the correct retirement's stop barrier. The correct model never takes that action.

The final Declared script preserves the order's root-declaration ordering: I is retired and
unregistered first, I′ is placed and set up, then root may declare the node lost at step 14.
I′ has the same selected retire hook as I. Its cut is at step 14 and heal at step 15; this
replaces the earlier Declared window, while leaving the expected table untouched. This
checks lost-hook suppression on I′ and no-hook handling on J. The loss windows begin after
the initial setups; the encoded never-set-up exception is not a separate coverage claim.

## MC4 earlier-round results, 2026-11-11

Historical measurements and traces below predate the admin review. The revised model and
current results are recorded in the final section; these earlier counts are retained as evidence.

Final source passed `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
and both complete release test commands: **25 passed, 0 failed**, plus 0 doc tests.
The ordinary release command took **38.31 s wall time**, including compilation,
with 27.78 s in tests. The output-capture command took **29.14 s wall time**, with
28.32 s in tests. These are local measurements, not an ubuntu-latest result.
`CARGO_BUILD_JOBS=3` was used throughout; `RUSTC_WRAPPER=` bypassed the shared
sccache daemon after its first dependency write was denied by the sandbox. No owner
configuration was changed. All earlier models and tests pass unchanged.

Every full run below exhausted its bounded graph, including broken variants. All
enabled harness fault witnesses were reached. The largest graph has **128,265 states**;
every run is below the 190,000-state ceiling and the 10-second timeout. Full runs used
2 checker threads; named refutations used single-thread BFS. The columns use the
registered P1…P9 and R G D L U X I N Q B K orders. Missing reach witnesses are
explicit `-` entries, not safety passes. Report times are logical observation sequence
numbers, with truth snapshots; no clock or timeout policy is inferred.

| Variant/configuration | Unique states | Depth | Seconds | P1…P9 observed | R G D L U X I N Q B K observed |
| --- | ---: | ---: | ---: | --- | --- |
| Correct/Base | 151 | 42 | 0.005 | HHHHHHHHH | - - - + - + + - - + + |
| Correct/Partition | 127,793 | 46 | 4.560 | HHHHHHHHH | + - - + + + + - + + + |
| Correct/PartitionRetire | 1,435 | 43 | 0.018 | FHHHHHHHH | + - - + - + + - + + + |
| Correct/Combined | 11,659 | 49 | 0.292 | FHHHHHHHH | + - - + - + + - + + + |
| Correct/LossA | 638 | 42 | 0.008 | HHHHHHHHH | - - - + - + + - - + + |
| Correct/LossM2 | 1,009 | 42 | 0.028 | HHHHHHHHH | - - - + - + + - + + + |
| Correct/ReorderA | 287 | 42 | 0.016 | HHHHHHHHH | - - - + - + + - - + + |
| Correct/ReorderM2 | 309 | 42 | 0.008 | HHHHHHHHH | - - - + - + + - - + + |
| Correct/Gone | 1,956 | 43 | 0.043 | HHHHHHHHH | - + - + - + + + + + + |
| Correct/Declared | 1,747 | 47 | 0.047 | HHHHHHHHH | + - + + - + + + - + + |
| UnreachableGone/Partition | 83,667 | 45 | 2.764 | FFHHHHHHH | + - - + + + + + + + + |
| UnknownRun/Partition | 128,265 | 48 | 4.422 | FHHHHHHHH | + - - + + + + - - + + |
| VolatileStop/Combined | 3,955 | 48 | 0.081 | FHHFHHHHH | + - - + - + + - + + + |
| WrongStop/Base | 100 | 41 | 0.009 | FHHFHHHHH | - - - + - + + - - + + |
| ConfirmLost/Declared | 1,747 | 47 | 0.042 | HHFHHHHHH | + - + + - + + + - + + |
| LostHook/Declared | 1,747 | 47 | 0.038 | HHFHHHHHH | + - + + - + + + - + + |
| WrongMachine/Base | 151 | 42 | 0.011 | HHHHFHHHH | - - - + - + + - - + + |
| StaleRunning/Partition | 127,793 | 46 | 4.267 | HHHHHFHHH | + - - + + + + - + + + |
| EarlySetup/Base | 151 | 42 | 0.004 | HHHHHHFHH | - - - + - + + - - - + |
| ReuseNode/Declared | 1,597 | 46 | 0.083 | HHHHHHHFH | + - + + - + + - - + + |

### MC4 findings and differences from registration

The registered expected table remains unchanged. The test compares each full run with
that table and the separately recorded observed differences; it rejects any further
unrecorded difference. Differences in reach concern the table or scenario bounds, not
new behavior imposed on the promises.

| ID | Class | Finding and affected rows |
| --- | --- | --- |
| MC4-001 | Ambiguity | PLC-017 defaults a lost node's shutdown to unknown, but LIF-025 confirms an instance for which no setup was ever dispatched, and PLC-026 says a node's setup dispatches nothing. Does completing node setup count for that exception? Projection: report/register node → set node up without an implementation call → root declares loss. The model records node retirement but leaves that shutdown classification undecided. No specification change was made. |
| MC4-002 | Gap | P1 was expected H but is F in Correct/PartitionRetire and Correct/Combined, and additionally in VolatileStop/Combined. PRV-007 and PRV-010 concern one instance ID, while LIF-036 can remove I after a stop answered unknown and PRV-021 owes its later stop. Nothing among PRV-007/PRV-010/PRV-014 fences I′ until that stop is carried. The shortest correct-model trace below has 33 actions: cut → report unreachable → retire I (hook unavailable) → unregister I (stop unknown, retained) → heal → report reachable → place/run I′ before Carry(I). Root never declares loss. Both IDs are live. This is a gap between the requested aggregate invariant and the promises, not a claim that PRV-007 explicitly forbids another ID. The model and refuted property were retained. |
| MC4-003 | Gap | U was expected reached but is absent in Combined and VolatileStop/Combined, and in every Declared row (Correct, ConfirmLost, LostHook, ReuseNode). Those final bounded cuts start after A's exchange. Partition reaches U; PartitionRetire registered U absent and agrees. |
| MC4-004 | Gap | N was expected absent but is reached in UnreachableGone/Partition: falsely retiring the node causes a fresh node on the reachable report, as the remaining correct registration rule requires. |
| MC4-005 | Gap | Q was expected reached but is absent in UnknownRun/Partition: in this bounded window the mutant takes the new-run branch on unknown, rather than the blocking branch that Q witnesses. The named P1 refutation remains reached. |
| MC4-006 | Gap | Q was expected absent but is reached in Correct/LossM2 and Correct/Gone. A lost run transport answer, or physical disappearance before its machine report, gives an unknown inspect answer even without a partition. |
| MC4-007 | Gap | P1 was expected H but is also F in WrongStop/Base: stopping J for I leaves I live while I′ starts. The registered named P4 failure is also present. |
| MC4-008 | Gap | Q was expected reached but is absent in all four Declared rows. The final ordered root cut occurs after the explicit inspection window. The table did not account for that restriction; the runtime reports no unknown inspect answer in that scenario. |

MC4-003 through MC4-008 are pre-registration/accounting gaps, not defects that bind a
language implementation. MC4-001 needs clarification and MC4-002 needs coordinator/owner
review of the aggregate claim. Files for decisions, specification fixes, vectors and the
tracker are outside this order's scope. The withdrawn provisional Base trace described
above is not counted as a language finding.

### MC4 first single-thread BFS traces

`Script(n)` has these meanings: 0 place I; 1 place J; 2 probe setup before node readiness;
3 set node up; 4 set I up; 5 set J up; 6 explicit further setup after A sends;
7 inspect I; 8 skip optional exit; 9 skip optional restart; 10 no-op script boundary;
11 retire I; 12 unregister I; 13 place/setup I′; 14 declare loss in Declared, then
retire/unregister J; 15 retire/unregister I′; 16 retire provider. `Exit` and `Restart`
replace their optional script steps. `FinishRetire` completes the hook after its run
answer, then asks the stop. `SetupReturnedNode` is the later setup of a new registration.
`RecreateAfterSilence` belongs only to the deliberately broken UnreachableGone policy.
`Carry` is the independent provider keeper. Deliveries are FIFO unless labelled out of
order. The printed first paths are the unedited single-thread harness descriptions.

```text
MC4 refutation UnreachableGone: PLC-023: unreachable is not gone
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
8. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
9. engine+providers (M1) Script(5)
10. cut engine+providers (M1) <-> runs+keeper (M2)
11. engine+providers (M1) Report(Unreachable)
(11 actions)

MC4 refutation UnknownRun: PRV-007 PRV-010 PRV-014: at most one live run
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
8. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
9. engine+providers (M1) Script(5)
10. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
11. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
12. A (M1) Send
13. delivered A (M1) -> engine+providers (M1) [position 0]
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. delivered engine+providers (M1) -> A (M1) [position 0]
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. cut engine+providers (M1) <-> runs+keeper (M2)
21. delivered engine+providers (M1) -> A (M1) [position 0]
22. engine+providers (M1) Script(6)
(22 actions)

MC4 refutation VolatileStop: PRV-008 PRV-021 PRV-027: stops kept and addressed
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
8. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
9. engine+providers (M1) Script(5)
10. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
11. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
12. A (M1) Send
13. delivered A (M1) -> engine+providers (M1) [position 0]
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. delivered engine+providers (M1) -> A (M1) [position 0]
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. engine+providers (M1) Script(6)
22. engine+providers (M1) Script(7)
23. engine+providers (M1) Script(8)
24. engine+providers (M1) Script(9)
25. engine+providers (M1) Script(10)
26. cut engine+providers (M1) <-> runs+keeper (M2)
27. engine+providers (M1) Report(Unreachable)
28. engine+providers (M1) Script(11)
29. engine+providers (M1) Script(12)
30. crashed engine+providers (M1)
31. recovered engine+providers (M1)
32. healed engine+providers (M1) <-> runs+keeper (M2)
33. engine+providers (M1) Recover
34. engine+providers (M1) Report(Reachable)
35. engine+providers (M1) Script(13)
36. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
37. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
38. engine+providers (M1) Script(14)
39. engine+providers (M1) Script(15)
40. engine+providers (M1) Script(16)
41. runs+keeper (M2) Carry(1)
42. runs+keeper (M2) Carry(2)
43. A (M1) ObserveServers
44. engine+providers (M1) Quiescent
(44 actions)

MC4 refutation WrongStop: PRV-008 PRV-021 PRV-027: stops kept and addressed
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
8. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
9. engine+providers (M1) Script(5)
10. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
11. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
12. A (M1) Send
13. delivered A (M1) -> engine+providers (M1) [position 0]
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. delivered engine+providers (M1) -> A (M1) [position 0]
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. engine+providers (M1) Script(6)
22. engine+providers (M1) Script(7)
23. engine+providers (M1) Script(8)
24. engine+providers (M1) Script(9)
25. engine+providers (M1) Script(10)
26. engine+providers (M1) Script(11)
(26 actions)

MC4 refutation ConfirmLost: PLC-017 PLC-024 PLC-027: lost shutdown unknown
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
8. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
9. engine+providers (M1) Script(5)
10. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
11. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
12. A (M1) Send
13. delivered A (M1) -> engine+providers (M1) [position 0]
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. delivered engine+providers (M1) -> A (M1) [position 0]
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. engine+providers (M1) Script(6)
22. engine+providers (M1) Script(7)
23. engine+providers (M1) Script(8)
24. engine+providers (M1) Script(9)
25. engine+providers (M1) Script(10)
26. engine+providers (M1) Script(11)
27. runs+keeper (M2) Carry(0)
28. engine+providers (M1) Script(12)
29. engine+providers (M1) Script(13)
30. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
31. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
32. engine+providers (M1) Script(14)
(32 actions)

MC4 refutation LostHook: PLC-017 PLC-024 PLC-027: lost shutdown unknown
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
8. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
9. engine+providers (M1) Script(5)
10. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
11. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
12. A (M1) Send
13. delivered A (M1) -> engine+providers (M1) [position 0]
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. delivered engine+providers (M1) -> A (M1) [position 0]
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. engine+providers (M1) Script(6)
22. engine+providers (M1) Script(7)
23. engine+providers (M1) Script(8)
24. engine+providers (M1) Script(9)
25. engine+providers (M1) Script(10)
26. engine+providers (M1) Script(11)
27. runs+keeper (M2) Carry(0)
28. engine+providers (M1) Script(12)
29. engine+providers (M1) Script(13)
30. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
31. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
32. engine+providers (M1) Script(14)
(32 actions)

MC4 refutation WrongMachine: PLC-002 PRV-015 PRV-019: runs only where placed
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
(7 actions)

MC4 refutation StaleRunning: PRV-009: status rests on a true report
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
8. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
9. engine+providers (M1) Script(5)
10. cut engine+providers (M1) <-> runs+keeper (M2)
11. engine+providers (M1) Report(Unreachable)
(11 actions)

MC4 refutation EarlySetup: PLC-019: setup waits for a ready node
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
(4 actions)

MC4 refutation ReuseNode: PLC-026: returned machine gets a new node
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
8. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
9. engine+providers (M1) Script(5)
10. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
11. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
12. A (M1) Send
13. delivered A (M1) -> engine+providers (M1) [position 0]
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. delivered engine+providers (M1) -> A (M1) [position 0]
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. engine+providers (M1) Script(6)
22. engine+providers (M1) Script(7)
23. engine+providers (M1) Script(8)
24. engine+providers (M1) Script(9)
25. engine+providers (M1) Script(10)
26. engine+providers (M1) Script(11)
27. runs+keeper (M2) Carry(0)
28. engine+providers (M1) Script(12)
29. engine+providers (M1) Script(13)
30. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
31. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
32. cut engine+providers (M1) <-> runs+keeper (M2)
33. engine+providers (M1) Script(14)
34. engine+providers (M1) Report(Unreachable)
(34 actions)

MC4 literal refutation Correct/PartitionRetire: PRV-007 PRV-010 PRV-014: at most one live run
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
8. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
9. engine+providers (M1) Script(5)
10. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
11. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
12. A (M1) Send
13. delivered A (M1) -> engine+providers (M1) [position 0]
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. delivered engine+providers (M1) -> A (M1) [position 0]
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. engine+providers (M1) Script(6)
22. engine+providers (M1) Script(7)
23. engine+providers (M1) Script(8)
24. engine+providers (M1) Script(9)
25. engine+providers (M1) Script(10)
26. cut engine+providers (M1) <-> runs+keeper (M2)
27. engine+providers (M1) Report(Unreachable)
28. engine+providers (M1) Script(11)
29. engine+providers (M1) Script(12)
30. healed engine+providers (M1) <-> runs+keeper (M2)
31. engine+providers (M1) Report(Reachable)
32. engine+providers (M1) Script(13)
33. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
(33 actions)

```

### What MC4 establishes and what S3 must add

In these finite scripts, all named properties other than the aggregate replacement P1
hold in the correct model, including unknown status on unreachable reports, suppression of
lost-node hooks, addressed durable stops at reachable quiescence, placement and fresh node
registration. P1 holds in the other eight correct configurations and fails in the two
retirement-cut configurations above. All ten deliberately broken policies refute their
registered named properties. Quiescent discharge is a bounded safety assertion, not a
fairness or eventual-connectivity proof: held or lost messages can leave a non-quiescent
state forever. Root's explicit loss risk is excepted only for run overlap.

This neither proves every clause of the 17 modelled promises nor covers arbitrary scripts,
all fault timings, unbounded machines, real authorization/gates, enrollment or a concrete
provider's durable-storage implementation. The fixed selectors and machine names are
abstractions. The independent provider mailbox abstracts accepted-stop custody; this model
cannot show that a real provider can deliver, persist or execute that custody correctly
across a physical network cut. The node shutdown ambiguity remains open. No conformance
vector, engine behavior, accepted promise or ruling was changed.

[S3's pre-registration](../014-s3-two-machines/README.md) stands beside this bounded evidence:
its real machines must distinguish the E15 cut-before/after-effect and revocation cells,
unavailable from unknown from retired from confirmed stopped, and prove actual placement.
In particular S3-T6 must witness a nonempty provider-held stop before removal, engine/provider
restart and retirement where applicable, then match the stop carried after return to the
old instance/run. It must establish the real provider's custody and network behavior that
this abstract model assumes. Enrollment remains blocked on written promises. S3 should also
observe the old and replacement run together in MC4-002's ordering, or demonstrate and
specify the extra fence its implementation relies on; this model supplies no such fence.


## MC4 admin-review revision: model and pre-registration

Registered 2026-11-11 **before running the revised model**. The original expected table
above remains unchanged. All ten configurations still run Correct; the original ten mutants
retain their configurations. Two additional mutants run as follows (same P1…P9 and reach order):

| Added variant/configuration | P1…P9 expected | R G D L U X I N Q B K expected |
| --- | --- | --- |
| ConfirmLaterLost/Declared | HHFHHHHHH | + - + + + + + + + + + |
| BadSetupAnswer/Base | HHHHHHFHH | - - - + - + + - - + + |

The machines, nodes, instances, script, fault budgets, bounds, promise accounting (17 modelled,
four static decisions for vectors, one requiring a Pool), supporting promises and enrollment
limits above continue to apply. Inspect answers now occupy a separate local action; a second
local action uses the saved answer. Script advancement waits for that inspection. The provider's
one optional restart can occur after Exit at any later script step until a stop is asked for,
including between the retirement inspection and its run request or delivery. It consumes its
own one-use allowance and does not advance the engine's script. Reports remain untimed.
These changes remove the review's accidental serialization; no new promise is assumed.

Request ends are saved engine state. The ledger distinguishes that end, caller delivery,
and a late implementation reply. An unreachable report ends an outstanding dispatched request
unknown and sends that answer to A. A late reply cannot change it without reconciliation
(OPS-013, supporting outside the 22). Reconciliation is outside this bounded script.

Every retirement now records whether its placement is lost, independently of the retirement's
reason. P3 checks that fact on all subsequent retirements. ConfirmLaterLost confirms shutdown
only on a later request retirement, leaving the declaration's unknown intact. P7 now requires
both no dispatch and an unavailable answer for every not-ready setup. BadSetupAnswer instead
answers unknown without dispatching. These mutants must refute P3 and P7 respectively.

Concrete regressions replay cut → unknown end → unknown delivered to A → heal → late reply,
and stopped inspection → provider restart → run request → second live I. Ledger-only checks
also reject later lost-node confirmation and either incorrect answer to a blocked setup.
The full-run test will preserve every newly observed difference from the registered table.


The first revised Partition run exceeded the 190,000-state ceiling; it yielded no verdict.
Before rerunning, its consumer bound is reduced to one request (the other configurations
retain two). PartitionRetire separately exercises the retirement cut; Base still explores
the restart race with two requests, and ReorderA/ReorderM2 still explore both requests.
All Partition variants use that same one-request bound. The cut/heal regression also uses
one request. This is a stated script bound, not a restriction on permitted provider restart:
that restart remains freely interleaved through retirement in every configuration. Original
expectations and their reach entries remain unchanged.


The reduced Partition graph also exceeded the ceiling without a verdict. It is split before
its next run: Partition cuts only before the one consumer request has been dispatched;
PartitionReply cuts only after dispatch. Both enable exactly one cut and heal on engine↔M2,
keep the one-request bound, and retain the full retirement/restart suffix. Together they cover
the one-request cut window. PartitionReply supplies the concrete late-reply regression.
The original Partition variants remain in the before-dispatch part. The added Correct run
is registered below; U is absent because its sole request was dispatched before the cut.

| Added configuration | P1…P9 expected | R G D L U X I N Q B K expected |
| --- | --- | --- |
| Correct/PartitionReply | HHHHHHHHH | + - - + - + + - + + + |


The before-dispatch part still exceeded the ceiling. Before another run, the consumer
partition is also split by the optional restart's script window: Partition and PartitionReply
allow it only at step 9; PartitionRestartRace and PartitionReplyRestartRace allow it from
step 10 through the first stop. The remaining configurations keep the unrestricted window.
All four consumer parts use one request, exactly one cut/heal budget on engine↔M2 and the
same 190,000-state/10-second bounds. The two dimensions cover before/after dispatch and
all restart timings of the reduced script. No verdict from a ceiling-rejected search is used.
The restart race stays reachable in both added parts, Base and the retirement-fault parts.
The previous statement of a free restart in every configuration is superseded by this split.

| Added configuration | P1…P9 expected | R G D L U X I N Q B K expected |
| --- | --- | --- |
| Correct/PartitionRestartRace | HHHHHHHHH | + - - + + + + - + + + |
| Correct/PartitionReplyRestartRace | HHHHHHHHH | + - - + - + + - + + + |


Partition and PartitionReply exhausted their split graphs, but PartitionRestartRace still
exceeded the ceiling. Before the next run, all four consumer partition parts additionally
wait at step 8 for healed consumer messages to drain, before the optional Exit. This keeps
the cut/heal/late-reply exchange within its own script phase, while permitting the full
late-reply sequence and both retirement restart windows. Other parts retain their windows.
All registered verdicts remain unchanged. This excludes consumer messages delayed beyond
that phase; it does not serialize inspection with restart or add a safety fence.


That run again hit the ceiling in PartitionRestartRace. Its next pre-registered bound
(and PartitionReplyRestartRace's) narrows the delayed restart to the race itself: after
retirement's inspection answer, before that retirement asks a stop, including while its
run call is in flight. Other parts still allow restart before the inspection. The two
race parts therefore sample this later window rather than claiming all delayed restart
timings. This preserves the permitted counterexample, while excluding non-racing delayed
restarts from these two fault configurations. Expected verdicts remain untouched.


The four consumer parts then exhausted below the bound. Combined exceeded the ceiling.
Before its next run, Combined is split into the same early-restart window (step 9) and
CombinedRestartRace (after retirement's inspection answer through its stop). Both still
include cut → crash → recovery → heal in a single execution; none of the four faults is
removed. VolatileStop stays in Combined, where its kept-stop-loss trace needs all four.
The original table remains unchanged. The added row is registered before execution:

| Added configuration | P1…P9 expected | R G D L U X I N Q B K expected |
| --- | --- | --- |
| Correct/CombinedRestartRace | HHHHHHHHH | + - - + + + + - + + + |


### Revision findings recorded before accepting the full-run table

**MC4-009 — gap (PRV-007, PRV-010, PRV-014, with TPL-018 supporting).** Correct/Base
refutes P1 without any fault or replacement. I exits; retirement's separate inspection
answers stopped; the engine asks for a run to serve the hook; before that call arrives,
the provider restarts I; delivery of the call starts a second I. There is no outstanding
stop, unknown inspection or root declaration. The first BFS trace has 38 actions, reproduced
below in the revised results. A concrete regression additionally places the restart between
the inspection answer and the run request. PRV-007 requires the run call after a stopped
answer, and PRV-010 permits the restart. Neither supplies atomic exclusion or deduplication.
The order explicitly models a provider that starts a run on every call. This is a gap in
guaranteeing the requested at-most-one invariant, rather than a claim that PRV-007 explicitly
states that aggregate invariant. The model and P1 refutation remain. The same race adds P1
failures in the unrestricted and race configurations and several mutants. The early
consumer parts alone still pass P1; early Combined still has MC4-002's replacement gap.

**MC4-010 — pre-registration gap (supporting PRV-007, PLC-023).** Q was registered reached
but is absent in Correct/Partition, Correct/PartitionRestartRace, UnreachableGone/Partition
and StaleRunning/Partition (UnknownRun's absence was already MC4-005). The before-dispatch
cut cannot overlap I's inspection window: the unavailable machine report suppresses its
setup inspection, and its next inspection follows heal. Q observes I, not J. The after-dispatch
parts do reach Q. These are bounded witness differences, not new language defects.

**MC4-011 — pre-registration gap (PLC-019).** BadSetupAnswer/Base was registered with B
reached, but its only blocked setup answers unknown, so B is absent. P7 correctly refutes
that answer. This is an expected-table accounting error, not a language defect. Its P1
failure is MC4-009. All original and added expected tables remain unchanged; tests compare
observations against those tables plus these explicitly recorded differences.


## MC4 revised results, 2026-11-11

All four admin-review findings are fixed within MC4. `cargo fmt --check` and
`cargo clippy --all-targets -- -D warnings` pass. Both full release commands pass:
**30 tests passed, 0 failed, 0 ignored**, plus 0 doc tests. Ordinary release wall time
is **60.10 s**, including compilation (48.79 s in tests); the single-test-thread
output run takes **49.63 s** (48.82 s in tests). These are local timings,
not ubuntu-latest measurements. All commands use `CARGO_BUILD_JOBS=3`; model checking
uses two checker threads for full graphs and one for first BFS refutations. No existing
model or harness source changed in this review round. No timing-sensitive shell test failed.
The lane coordinator's fixes described below changed `placement.rs` after this round; with them,
33 tests pass.

Every graph below exhausted; every enabled harness fault has a witness. The largest graph
has **154,232 unique states**, and the slowest takes
**4.865 s**, below the unchanged 190,000-state
and 10-second bounds. Ceiling-rejected development attempts above have no verdict.
P1…P9 and R G D L U X I N Q B K use the registered orders above. H holds, F refutes,
+ reached and - absent. Report times remain logical observation sequence numbers.

| Variant/configuration | Unique states | Depth | Seconds | P1…P9 observed | R G D L U X I N Q B K observed |
| --- | ---: | ---: | ---: | --- | --- |
| Correct/Base | 1,094 | 64 | 0.028 | FHHHHHHHH | - - - + - + + - - + + |
| Correct/Partition | 120,571 | 61 | 4.079 | HHHHHHHHH | + - - + + + + - - + + |
| Correct/PartitionReply | 61,980 | 63 | 1.910 | HHHHHHHHH | + - - + - + + - + + + |
| Correct/PartitionRestartRace | 135,818 | 62 | 4.567 | FHHHHHHHH | + - - + + + + - - + + |
| Correct/PartitionReplyRestartRace | 69,027 | 64 | 2.135 | FHHHHHHHH | + - - + - + + - + + + |
| Correct/PartitionRetire | 25,500 | 64 | 0.724 | FHHHHHHHH | + - - + - + + - + + + |
| Correct/Combined | 114,348 | 69 | 3.627 | FHHHHHHHH | + - - + - + + - + + + |
| Correct/CombinedRestartRace | 93,351 | 69 | 2.976 | FHHHHHHHH | + - - + - + + - + + + |
| Correct/LossA | 5,149 | 64 | 0.117 | FHHHHHHHH | - - - + - + + - - + + |
| Correct/LossM2 | 8,146 | 64 | 0.218 | FHHHHHHHH | - - - + - + + - + + + |
| Correct/ReorderA | 2,167 | 64 | 0.043 | FHHHHHHHH | - - - + - + + - - + + |
| Correct/ReorderM2 | 2,181 | 64 | 0.049 | FHHHHHHHH | - - - + - + + - - + + |
| Correct/Gone | 12,272 | 64 | 0.241 | FHHHHHHHH | - + - + - + + + + + + |
| Correct/Declared | 11,654 | 62 | 0.250 | FHHHHHHHH | + - + + - + + + - + + |
| UnreachableGone/Partition | 90,769 | 59 | 2.851 | FFHHHHHHH | + - - + + + + + - + + |
| UnknownRun/Partition | 154,232 | 61 | 4.865 | FHHHHHHHH | + - - + + + + - - + + |
| VolatileStop/Combined | 38,604 | 69 | 1.174 | FHHFHHHHH | + - - + - + + - + + + |
| WrongStop/Base | 626 | 63 | 0.012 | FHHFHHHHH | - - - + - + + - - + + |
| ConfirmLost/Declared | 11,654 | 62 | 0.243 | FHFHHHHHH | + - + + - + + + - + + |
| LostHook/Declared | 11,654 | 62 | 0.245 | FHFHHHHHH | + - + + - + + + - + + |
| WrongMachine/Base | 1,094 | 64 | 0.019 | FHHHFHHHH | - - - + - + + - - + + |
| StaleRunning/Partition | 120,571 | 61 | 4.141 | HHHHHFHHH | + - - + + + + - - + + |
| EarlySetup/Base | 1,096 | 66 | 0.030 | FHHHHHFHH | - - - + - + + - - - + |
| ReuseNode/Declared | 10,574 | 61 | 0.281 | FHHHHHHFH | + - + + - + + - - + + |
| ConfirmLaterLost/Declared | 11,654 | 62 | 0.243 | FHFHHHHHH | + - + + - + + + - + + |
| BadSetupAnswer/Base | 1,094 | 64 | 0.029 | FHHHHHFHH | - - - + - + + - - - + |

Five rows above were measured again on 2026-11-11, on the same machine, after the lane
coordinator fixed two defects that the second round's verifier found in the correct model.
No verdict or reach changed; only these rows' counts and times did.

- A further retirement of an instance whose node was declared lost now asks for the stop,
  whatever M2's reported health: gone, unreachable or omitted. PLC-023 covers only an instance
  on a node that represents the machine, and a lost instance's node represents none (PLC-026).
  PLC-027 keeps its hook and that hook's run from being asked for, and its stop is asked for
  all the same. Tests: `further_retirement_after_gone_asks_stop` and
  `retirement_on_declared_lost_node_asks_stop_when_unreachable`.
- A report that omits M2 no longer registers a node; only an explicit report of reachable or
  unreachable does (PLC-026; test `omitted_report_registers_no_node`).

Each of the three tests was run against the code without its fix and failed. A fresh reader
checked the fixes; its one remaining finding, the unreachable and omitted cases, is fixed here.

### Every difference from the registered tables

This table lists all differing entries, including earlier-round findings. Omitted rows
agree in every safety and reach entry. The expected tables remain unchanged. MC4-009
accounts for the added P1 failures; MC4-002 and MC4-007 already account for replacement
and wrong-stop overlap. U differences are MC4-003, N is MC4-004, Q differences are
MC4-005/006/008/010, and B is MC4-011. The original node ambiguity MC4-001 remains open.

| Variant/configuration | Expected → observed |
| --- | --- |
| Correct/Base | P1: H→F |
| Correct/Partition | Q: +→- |
| Correct/PartitionRestartRace | P1: H→F; Q: +→- |
| Correct/PartitionReplyRestartRace | P1: H→F |
| Correct/PartitionRetire | P1: H→F |
| Correct/Combined | P1: H→F; U: +→- |
| Correct/CombinedRestartRace | P1: H→F; U: +→- |
| Correct/LossA | P1: H→F |
| Correct/LossM2 | P1: H→F; Q: -→+ |
| Correct/ReorderA | P1: H→F |
| Correct/ReorderM2 | P1: H→F |
| Correct/Gone | P1: H→F; Q: -→+ |
| Correct/Declared | P1: H→F; U: +→-; Q: +→- |
| UnreachableGone/Partition | N: -→+; Q: +→- |
| UnknownRun/Partition | Q: +→- |
| VolatileStop/Combined | P1: H→F; U: +→- |
| WrongStop/Base | P1: H→F |
| ConfirmLost/Declared | P1: H→F; U: +→-; Q: +→- |
| LostHook/Declared | P1: H→F; U: +→-; Q: +→- |
| WrongMachine/Base | P1: H→F |
| StaleRunning/Partition | Q: +→- |
| EarlySetup/Base | P1: H→F |
| ReuseNode/Declared | P1: H→F; U: +→-; Q: +→- |
| ConfirmLaterLost/Declared | P1: H→F; U: +→-; Q: +→- |
| BadSetupAnswer/Base | P1: H→F; B: +→- |

### First revised single-thread BFS traces

Each mutant's first trace refutes its named safety property, independently of other failures.
ConfirmLaterLost's replay also asserts that the root retirement reported unknown before the
later request confirmed it. BadSetupAnswer's replay asserts no setup dispatch and an unknown
answer. After those checks, every mutant replay erases all node data and still refutes its
property using only the ledger. Correct traces retain both model findings. The current first
PartitionRetire trace reaches the restart race; the original replacement gap also remains
refuted in the early-restart Combined graph.

**UnreachableGone — 13 actions; PLC-023: unreachable is not gone.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. cut engine+providers (M1) <-> runs+keeper (M2)
13. engine+providers (M1) Report(Unreachable)
```

**UnknownRun — 14 actions; PRV-007 PRV-010 PRV-014: at most one live run.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. cut engine+providers (M1) <-> runs+keeper (M2)
13. engine+providers (M1) InspectAnswer
14. engine+providers (M1) UseInspect
```

**VolatileStop — 56 actions; PRV-008 PRV-021 PRV-027: stops kept and addressed.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. engine+providers (M1) InspectAnswer
13. engine+providers (M1) UseInspect
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. A (M1) Send
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. delivered A (M1) -> engine+providers (M1) [position 0]
22. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
23. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
24. delivered engine+providers (M1) -> A (M1) [position 0]
25. engine+providers (M1) Script(6)
26. engine+providers (M1) InspectAnswer
27. engine+providers (M1) UseInspect
28. engine+providers (M1) Script(7)
29. engine+providers (M1) InspectAnswer
30. engine+providers (M1) UseInspect
31. engine+providers (M1) Script(8)
32. engine+providers (M1) Script(9)
33. engine+providers (M1) Script(10)
34. cut engine+providers (M1) <-> runs+keeper (M2)
35. engine+providers (M1) Report(Unreachable)
36. engine+providers (M1) Script(11)
37. engine+providers (M1) Script(12)
38. crashed engine+providers (M1)
39. recovered engine+providers (M1)
40. healed engine+providers (M1) <-> runs+keeper (M2)
41. engine+providers (M1) Recover
42. engine+providers (M1) Report(Reachable)
43. engine+providers (M1) Script(13)
44. engine+providers (M1) InspectAnswer
45. engine+providers (M1) UseInspect
46. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
47. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
48. engine+providers (M1) Script(14)
49. engine+providers (M1) Script(15)
50. engine+providers (M1) InspectAnswer
51. engine+providers (M1) UseInspect
52. engine+providers (M1) Script(16)
53. runs+keeper (M2) Carry(1)
54. runs+keeper (M2) Carry(2)
55. A (M1) ObserveServers
56. engine+providers (M1) Quiescent
```

**WrongStop — 36 actions; PRV-008 PRV-021 PRV-027: stops kept and addressed.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. engine+providers (M1) InspectAnswer
13. engine+providers (M1) UseInspect
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. A (M1) Send
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. delivered A (M1) -> engine+providers (M1) [position 0]
22. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
23. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
24. delivered engine+providers (M1) -> A (M1) [position 0]
25. engine+providers (M1) Script(6)
26. engine+providers (M1) InspectAnswer
27. engine+providers (M1) UseInspect
28. engine+providers (M1) Script(7)
29. engine+providers (M1) InspectAnswer
30. engine+providers (M1) UseInspect
31. engine+providers (M1) Script(8)
32. engine+providers (M1) Script(9)
33. engine+providers (M1) Script(10)
34. engine+providers (M1) Script(11)
35. engine+providers (M1) InspectAnswer
36. engine+providers (M1) UseInspect
```

**ConfirmLost — 43 actions; PLC-017 PLC-024 PLC-027: lost shutdown unknown.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. engine+providers (M1) InspectAnswer
13. engine+providers (M1) UseInspect
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. A (M1) Send
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. delivered A (M1) -> engine+providers (M1) [position 0]
22. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
23. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
24. delivered engine+providers (M1) -> A (M1) [position 0]
25. engine+providers (M1) Script(6)
26. engine+providers (M1) InspectAnswer
27. engine+providers (M1) UseInspect
28. engine+providers (M1) Script(7)
29. engine+providers (M1) InspectAnswer
30. engine+providers (M1) UseInspect
31. engine+providers (M1) Script(8)
32. engine+providers (M1) Script(9)
33. engine+providers (M1) Script(10)
34. engine+providers (M1) Script(11)
35. engine+providers (M1) InspectAnswer
36. engine+providers (M1) UseInspect
37. runs+keeper (M2) Carry(0)
38. engine+providers (M1) Script(12)
39. engine+providers (M1) Script(13)
40. cut engine+providers (M1) <-> runs+keeper (M2)
41. engine+providers (M1) InspectAnswer
42. engine+providers (M1) UseInspect
43. engine+providers (M1) Script(14)
```

**LostHook — 43 actions; PLC-017 PLC-024 PLC-027: lost shutdown unknown.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. engine+providers (M1) InspectAnswer
13. engine+providers (M1) UseInspect
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. A (M1) Send
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. delivered A (M1) -> engine+providers (M1) [position 0]
22. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
23. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
24. delivered engine+providers (M1) -> A (M1) [position 0]
25. engine+providers (M1) Script(6)
26. engine+providers (M1) InspectAnswer
27. engine+providers (M1) UseInspect
28. engine+providers (M1) Script(7)
29. engine+providers (M1) InspectAnswer
30. engine+providers (M1) UseInspect
31. engine+providers (M1) Script(8)
32. engine+providers (M1) Script(9)
33. engine+providers (M1) Script(10)
34. engine+providers (M1) Script(11)
35. engine+providers (M1) InspectAnswer
36. engine+providers (M1) UseInspect
37. runs+keeper (M2) Carry(0)
38. engine+providers (M1) Script(12)
39. engine+providers (M1) Script(13)
40. cut engine+providers (M1) <-> runs+keeper (M2)
41. engine+providers (M1) InspectAnswer
42. engine+providers (M1) UseInspect
43. engine+providers (M1) Script(14)
```

**WrongMachine — 9 actions; PLC-002 PRV-015 PRV-019: runs only where placed.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
```

**StaleRunning — 13 actions; PRV-009: status rests on a true report.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. cut engine+providers (M1) <-> runs+keeper (M2)
13. engine+providers (M1) Report(Unreachable)
```

**EarlySetup — 4 actions; PLC-019: setup waits for a ready node.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
```

**ReuseNode — 44 actions; PLC-026: returned machine gets a new node.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. engine+providers (M1) InspectAnswer
13. engine+providers (M1) UseInspect
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. A (M1) Send
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. delivered A (M1) -> engine+providers (M1) [position 0]
22. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
23. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
24. delivered engine+providers (M1) -> A (M1) [position 0]
25. engine+providers (M1) Script(6)
26. engine+providers (M1) InspectAnswer
27. engine+providers (M1) UseInspect
28. engine+providers (M1) Script(7)
29. engine+providers (M1) InspectAnswer
30. engine+providers (M1) UseInspect
31. engine+providers (M1) Script(8)
32. engine+providers (M1) Script(9)
33. engine+providers (M1) Script(10)
34. engine+providers (M1) Script(11)
35. engine+providers (M1) InspectAnswer
36. engine+providers (M1) UseInspect
37. runs+keeper (M2) Carry(0)
38. engine+providers (M1) Script(12)
39. engine+providers (M1) Script(13)
40. cut engine+providers (M1) <-> runs+keeper (M2)
41. engine+providers (M1) InspectAnswer
42. engine+providers (M1) UseInspect
43. engine+providers (M1) Script(14)
44. engine+providers (M1) Report(Unreachable)
```

**ConfirmLaterLost — 45 actions; PLC-017 PLC-024 PLC-027: lost shutdown unknown.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. engine+providers (M1) InspectAnswer
13. engine+providers (M1) UseInspect
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. A (M1) Send
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. delivered A (M1) -> engine+providers (M1) [position 0]
22. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
23. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
24. delivered engine+providers (M1) -> A (M1) [position 0]
25. engine+providers (M1) Script(6)
26. engine+providers (M1) InspectAnswer
27. engine+providers (M1) UseInspect
28. engine+providers (M1) Script(7)
29. engine+providers (M1) InspectAnswer
30. engine+providers (M1) UseInspect
31. engine+providers (M1) Script(8)
32. engine+providers (M1) Script(9)
33. engine+providers (M1) Script(10)
34. engine+providers (M1) Script(11)
35. engine+providers (M1) InspectAnswer
36. engine+providers (M1) UseInspect
37. runs+keeper (M2) Carry(0)
38. engine+providers (M1) Script(12)
39. engine+providers (M1) Script(13)
40. cut engine+providers (M1) <-> runs+keeper (M2)
41. engine+providers (M1) InspectAnswer
42. engine+providers (M1) UseInspect
43. engine+providers (M1) Script(14)
44. healed engine+providers (M1) <-> runs+keeper (M2)
45. engine+providers (M1) Script(15)
```

**BadSetupAnswer — 4 actions; PLC-019: setup waits for a ready node.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
```

**Correct/PartitionRetire — 38 actions; PRV-007 PRV-010 PRV-014: at most one live run.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. engine+providers (M1) InspectAnswer
13. engine+providers (M1) UseInspect
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. A (M1) Send
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. delivered A (M1) -> engine+providers (M1) [position 0]
22. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
23. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
24. delivered engine+providers (M1) -> A (M1) [position 0]
25. engine+providers (M1) Script(6)
26. engine+providers (M1) InspectAnswer
27. engine+providers (M1) UseInspect
28. engine+providers (M1) Script(7)
29. engine+providers (M1) InspectAnswer
30. engine+providers (M1) UseInspect
31. runs+keeper (M2) Exit
32. engine+providers (M1) Script(9)
33. engine+providers (M1) Script(10)
34. engine+providers (M1) Script(11)
35. engine+providers (M1) InspectAnswer
36. engine+providers (M1) UseInspect
37. runs+keeper (M2) Restart
38. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
```

**Correct/Base — 38 actions; PRV-007 PRV-010 PRV-014: at most one live run.**

```text
1. engine+providers (M1) Report(Reachable)
2. engine+providers (M1) Script(0)
3. engine+providers (M1) Script(1)
4. engine+providers (M1) Script(2)
5. engine+providers (M1) Script(3)
6. engine+providers (M1) Script(4)
7. engine+providers (M1) InspectAnswer
8. engine+providers (M1) UseInspect
9. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
10. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
11. engine+providers (M1) Script(5)
12. engine+providers (M1) InspectAnswer
13. engine+providers (M1) UseInspect
14. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
15. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
16. A (M1) Send
17. delivered A (M1) -> engine+providers (M1) [position 0]
18. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
19. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
20. delivered engine+providers (M1) -> A (M1) [position 0]
21. delivered A (M1) -> engine+providers (M1) [position 0]
22. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
23. delivered runs+keeper (M2) -> engine+providers (M1) [position 0]
24. delivered engine+providers (M1) -> A (M1) [position 0]
25. engine+providers (M1) Script(6)
26. engine+providers (M1) InspectAnswer
27. engine+providers (M1) UseInspect
28. engine+providers (M1) Script(7)
29. engine+providers (M1) InspectAnswer
30. engine+providers (M1) UseInspect
31. runs+keeper (M2) Exit
32. engine+providers (M1) Script(9)
33. engine+providers (M1) Script(10)
34. engine+providers (M1) Script(11)
35. engine+providers (M1) InspectAnswer
36. engine+providers (M1) UseInspect
37. runs+keeper (M2) Restart
38. delivered engine+providers (M1) -> runs+keeper (M2) [position 0]
```

### What the revision shows and what S3 must add

The revised bounded model keeps unknown request ends through late implementation replies,
checks lost-placement shutdown on later retirements, and requires unavailable for every
blocked setup. The added mutants demonstrate the latter two checks. P2 through P9 hold in
all 14 correct configurations. P1's restart race is a gap even without a fault or a new ID;
the earlier unknown-stop/replacement gap remains separate. Neither counterexample is
repaired by an unstated fence. The lost node's own LIF-025 shutdown classification still
needs the owner's clarification. The accounting remains **17 modelled, four static decisions
for vectors, one requiring a Pool**; OPS-013 is an additional supporting promise outside the
22. Enrollment remains spec pending and needed before S3.

The model checks finite scripts, selected cut and restart windows, and abstract durable-stop
custody. It does not prove every fault timing, arbitrary provider policies, unbounded safety,
fairness, actual persistence, authorization or enrollment. In particular, the consumer cut
parts use one request and drain its healed exchange before the exit/restart phase; their
race parts restrict restart to after retirement's inspection answer. Base, PartitionRetire,
LossA/LossM2, ReorderA/ReorderM2, Gone and Declared retain the unrestricted pre-stop restart
window. Combined's two parts each keep all four faults together. Physical machine return
clears the vanished run's restart allowance; it represents a new machine, not resurrection
of the lost process.

[S3](../014-s3-two-machines/README.md) must supply E15's real-machine observations: actual
placement, an unknown request that stays unknown after a late reply until reconciled, durable
stop completion after removal/restart/retirement, distinct unavailable/retired/unknown/stopped
states, and fresh node identity on return. It must also exercise the inspected-stopped versus
provider-restart race and the old-ID/replacement overlap, or identify the provider contract
that prevents them. This experiment adds no such contract, and changes no specification,
engine code, conformance vector, ruling or record.

## MC5a — gates and identity: model registered before execution

Work order 109, ruling 71's proof lane, A40/G5. This is a reference model of the
promises, independent of engine code. [src/gates.rs](src/gates.rs) defines it.
The following model, bounds and expected table were written before its first run.

### Nodes, script and bounds

M1 contains the engine and P's server, which runs on the engine's own runner and
ends on an engine crash. A is a separate consumer node on M1, sending two requests
for I. M2 holds P's remote part, I's server and gate at URL `u`, and process X.
X is neither root, I's user nor P's user. The engine's own runner also runs L1 and
L2, with separate identity and state directories, secrets and listed variables.
Their runs end with the engine; a further setup after recovery starts them again.
This is an explicit setup action, not implicit repetition of an ordinary run.

One initial setup starts P, L1 and L2 and sends P's `run.v1` values to M2.
The remote delivery carries the secret and identity classes. Identity explicitly
models the variables and file paths, with symbolic contents and token/certificate
subjects, of the PRV-022 input: the engine URL, identity directory, trust bundle, engine ID,
I's signed token, and certificate/private key naming I. I presents its token in
one request to the engine. X can probe the gate without proof. After an engine
request reaches I, I can stop and X can take `u`, sending a reset that can be
lost or reordered. X sends one request from I's former address, without a token.
After a connect reaches X, P may restart I, once. These are M2 local actions,
independent of the engine crash budget. No other run restart is enabled.

A connect is an atomic transport step carrying no application data, guarded locally
against a cut; the `Connection`/`ConnectLog` records its process and incarnation.
Its proof reply is an ordinary lossy, reorderable M2-to-engine message. This abstracts
transport connection establishment, not an application request or certificate validation
algorithm. A connection binds delivery to the process that answered, never a later owner
of the URL. Each connection has a fresh ID, M2 incarnation, cut epoch and engine crash
epoch. An old proof cannot validate a different connection; an old wire call cannot
reach a new process. The engine holds its proof only in volatile state. The cut
invalidates **all** M1-to-M2 connections, including provider delivery and identity
requests, and blocks both directions; healing requires a fresh server connection and
proof. Queued provider values can finish delivery after healing over the authenticated,
confidential provider channel; this assumes that transport re-establishes that channel.

The harness cannot call the model back on cut/heal, nor can `fault_allowed` inspect
state. Local connect/send guards and receive checks enforce connection incarnation
and cut epochs. Stop/takeover and remote restart are guarded M2 actions, not harness
crashes. The harness crash is restricted to the engine with `fault_allowed`.
Recovery and heal have no hook in the harness; their only eligible crashed node and
cut pair follow from those restrictions. No harness code is changed.

Seven configurations split faults: Base (none), ReorderA and LossA (one fault on
A-to-engine), ReorderM2 and LossM2 (one on either direction between engine and M2),
Cut (one cut and one heal between engine and M2), Crash (one engine crash and one
recovery). Fault permissions are restricted with `fault_allowed`. Splitting keeps
fault timing exhaustive within each part without multiplying independent fault histories.
No simultaneous combinations across parts are claimed. Duplication, a permanently lost
machine and timers are outside this order; their budgets are zero with reasons.

Every run has two consumer requests, at most three connections, one remote start,
one stop/takeover, one provider restart, one X gate probe, two identity requests and
two generations of local setups. Each directed queue holds at most eight messages;
exceeding it fails, never prunes. The ceiling is 190,000 unique states (strictly below
200,000), and timeout is 10 seconds, asserted after exploration as well as guarded
during it. The target for all MC5a tests together is at most about 15 seconds.
State directories are symbolic relative paths containing instance IDs, initially empty
with mode 0700; persistence is modelled, filesystem permissions are not executed.

The external ledger records each connect and its verified certificate, sends by request
ID with contents/secret classes and the proof at send time, admitted requests with
engine proof, attribution with its token/address basis, identity/secret appearances,
all local-run variables and files, state directories, channel authentication and
confidentiality flags, and X's observations. Connect records stand for the
nothing-secret message class. Ordinary public surfaces carry that class; the seeded
Diagnostic instead adds an identity appearance. Fixed request slots avoid treating
irrelevant log order as a new behavior. No safety property reads a node's own state.

The fixed engine environment contains HOME, PATH, USER, GITHUB_TOKEN and deliberately
wrong CONDUIT_URL/CONDUIT_IDENTITY_DIR values; LOGNAME and SHELL
are absent, and TERM starts with `()`. The pass list is empty. Listed PATH and LOGNAME
exercise overrides; PASSWORD is a listed secret; LISTED distinguishes L1/L2.
Filtered inheritance is computed from that fixed input, then the listed variables,
secrets, identity and state are installed. Identity variables and files override other sources. The runner's GITHUB_TOKEN variant
passes the engine's token to L1. OtherIdentity gives L2 L1's directory and files/token.
OtherState gives L2 L1's state directory only in the post-recovery setup.

### Catalogue accounting (nine promises)

Modelled means the safety slice named here, not every obligation in that promise.
Each counted promise occurs in exactly one row.

| Promise | Status | Property |
| --- | --- | --- |
| PRV-003 | Modelled | R: own-runner identity and secret delivery |
| PRV-011 | Modelled | C: remote value delivery; R: local secret delivery |
| PRV-018 | Modelled | A: identity rests on the presented token, never address |
| PRV-022 | Modelled | H: holders and public exclusion; C: identity channel; R: local identity delivery |
| PRV-023 | Modelled | R: listed variables, allowed inherited environment and overrides |
| PRV-024 | Modelled | H: no identity in record, inspection, observation, diagnostic, result or `_meta` |
| PRV-025 | Modelled | G: only engine proof reaches I's gated server |
| PRV-028 | Modelled | B: instance certificate proved on this connection before application sends |
| PRV-029 | Modelled | R: each instance retains its own state directory after recovery |

Counts: **9 modelled, 0 wholly not modelled**. Partial exclusions: PRV-003's “on no
other” machine, machine provider and fetching (placement and provider kinds absent),
and PRV-025's silence-to-close bound (time is absent). The model also does not test
provider contract prose, file encodings, cryptography, OS enforcement, unsupported
package formats, removal/unregistration, or every runtime environment combination.
Supporting IFC-017, IFC-022, IFC-041, IFC-042 and PRV-026 inform connection,
credential, token and restart semantics; they are **outside these counts**. IFC-041's
remote-item behavior is not modelled: I is a runtime-provider run, not a Remote.

### Expected table, fixed before execution

Safety: G = gate admits only proof; B = proof before sending; A = identity rests on
what is presented; H = identity material stays with its holders; R = own runner keeps
its promises; C = channel authenticates M2. H means holds, X means refuted.
The channel property explicitly checks the model's authentication **and confidentiality
flags**, not TLS or another channel implementation. X can observe both classes if
either flag is false; hence Unauthenticated also breaks H.

Reachability: s = proven request reaches I; g = X refused at gate; x = takeover and
connect to X; n = X attributed to no instance; i = I attributed by token; r = L1/L2
run again after engine recovery; c = provider channel carries values to M2.
R means reached, dash means unreachable in that row. Every enabled harness fault
witness is expected reached in every row. Correct runs in all configurations; each
variant runs only in the configuration it needs. Early sends carry contents and a
secret, not an identity input, so they do not independently break H.

| Configuration / variant | G | B | A | H | R | C | s | g | x | n | i | r | c |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Base / Correct | H | H | H | H | H | H | R | R | R | R | R | - | R |
| ReorderA / Correct | H | H | H | H | H | H | R | R | R | R | R | - | R |
| LossA / Correct | H | H | H | H | H | H | R | R | R | R | R | - | R |
| ReorderM2 / Correct | H | H | H | H | H | H | R | R | R | R | R | - | R |
| LossM2 / Correct | H | H | H | H | H | H | R | R | R | R | R | - | R |
| Cut / Correct | H | H | H | H | H | H | R | R | R | R | R | - | R |
| Crash / Correct | H | H | H | H | H | H | R | R | R | R | R | R | R |
| Base / OpenGate | X | H | H | H | H | H | R | - | R | R | R | - | R |
| Base / SendFirst | H | X | H | H | H | H | R | R | R | R | R | - | R |
| Base / StaleProof | H | X | H | H | H | H | R | R | R | R | R | - | R |
| Base / Address | H | H | X | H | H | H | R | R | R | - | R | - | R |
| Base / Diagnostic | H | H | H | X | H | H | R | R | R | R | R | - | R |
| Base / InheritToken | H | H | H | H | X | H | R | R | R | R | R | - | R |
| Base / OtherIdentity | H | H | H | H | X | H | R | R | R | R | R | - | R |
| Crash / OtherState | H | H | H | H | X | H | R | R | R | R | R | R | R |
| Base / Unauthenticated | H | H | H | X | H | X | R | R | R | R | R | - | R |

The full-run test checks every safety verdict and every reachability verdict,
including enabled harness faults, against this table. A separate single-thread BFS
prints the first trace on each variant's named property. A replay test erases all
node data and checks that the ledger still refutes the property.


### MC5a results — 2026-11-11

The pinned Rust 1.97.1 toolchain is unavailable in this worker; its rustup directory
is read-only. Network use is restricted to crates, so no toolchain was downloaded.
These results use installed Rust/Cargo **1.99.1**, invoked with `+stable`, with the
repository pin unchanged. `CARGO_BUILD_JOBS=6` was inherited without an override;
full searches used two checker threads and refutations used one. Re-run with the
pinned compiler in CI before treating compiler-specific validation as complete.

`cargo +stable fmt --check` and `cargo +stable clippy --all-targets -- -D warnings`
passed. The complete `cargo +stable test --release` passed **21 tests**, 0 failed,
and 0 doc tests, in **8.58 seconds wall time** (7.19 seconds in the unit tests).
The complete `cargo +stable test --release -- --nocapture --test-threads=1` also
passed 21 tests and 0 doc tests, in **7.01 seconds wall time** (6.90 seconds in
unit tests). The final module's separate filtered check passed its two tests in
1.35 seconds. The following counts and traces come from the complete uncaptured
run, not from estimates. MC5a's 16 full searches total 0.943 seconds as printed.
No run reached its ceiling or timeout. No observed verdict differs from registration.

The keys and verdict alphabet are exactly those of the expected table above.
All enabled harness fault witnesses were reached in every row.

| Configuration / variant | Unique states | Depth | Seconds | G | B | A | H | R | C | s | g | x | n | i | r | c |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Base / Correct | 516 | 23 | 0.014 | H | H | H | H | H | H | R | R | R | R | R | - | R |
| ReorderA / Correct | 1,422 | 23 | 0.042 | H | H | H | H | H | H | R | R | R | R | R | - | R |
| LossA / Correct | 648 | 23 | 0.017 | H | H | H | H | H | H | R | R | R | R | R | - | R |
| ReorderM2 / Correct | 1,220 | 23 | 0.035 | H | H | H | H | H | H | R | R | R | R | R | - | R |
| LossM2 / Correct | 1,375 | 23 | 0.039 | H | H | H | H | H | H | R | R | R | R | R | - | R |
| Cut / Correct | 3,952 | 25 | 0.140 | H | H | H | H | H | H | R | R | R | R | R | - | R |
| Crash / Correct | 5,110 | 26 | 0.212 | H | H | H | H | H | H | R | R | R | R | R | R | R |
| Base / OpenGate | 516 | 23 | 0.021 | X | H | H | H | H | H | R | - | R | R | R | - | R |
| Base / SendFirst | 3,772 | 23 | 0.127 | H | X | H | H | H | H | R | R | R | R | R | - | R |
| Base / StaleProof | 1,112 | 23 | 0.034 | H | X | H | H | H | H | R | R | R | R | R | - | R |
| Base / Address | 516 | 23 | 0.014 | H | H | X | H | H | H | R | R | R | - | R | - | R |
| Base / Diagnostic | 516 | 23 | 0.013 | H | H | H | X | H | H | R | R | R | R | R | - | R |
| Base / InheritToken | 516 | 23 | 0.013 | H | H | H | H | X | H | R | R | R | R | R | - | R |
| Base / OtherIdentity | 516 | 23 | 0.014 | H | H | H | H | X | H | R | R | R | R | R | - | R |
| Crash / OtherState | 5,110 | 26 | 0.194 | H | H | H | H | X | H | R | R | R | R | R | R | R |
| Base / Unauthenticated | 516 | 23 | 0.014 | H | H | H | X | H | X | R | R | R | R | R | - | R |

### MC5a first refutations

**OpenGate: PRV-025: gate admits only proof (3 actions)**

```text
1. engine/M1 Setup
2. delivered engine/M1 -> M2 [position 0]
3. M2 Probe
```

**SendFirst: PRV-028: proof before sending (5 actions)**

```text
1. engine/M1 Setup
2. delivered engine/M1 -> M2 [position 0]
3. delivered A -> engine/M1 [position 0]
4. engine/M1 Connect
5. engine/M1 Send
```

**StaleProof: PRV-028: proof before sending (11 actions)**

```text
1. engine/M1 Setup
2. delivered engine/M1 -> M2 [position 0]
3. delivered A -> engine/M1 [position 0]
4. engine/M1 Connect
5. delivered M2 -> engine/M1 [position 0]
6. engine/M1 Send
7. delivered engine/M1 -> M2 [position 0]
8. M2 StopTake
9. delivered A -> engine/M1 [position 0]
10. engine/M1 Connect
11. engine/M1 Send
```

**Address: PRV-018: identity rests on what is presented (11 actions)**

```text
1. engine/M1 Setup
2. delivered engine/M1 -> M2 [position 0]
3. delivered A -> engine/M1 [position 0]
4. engine/M1 Connect
5. delivered M2 -> engine/M1 [position 0]
6. engine/M1 Send
7. delivered engine/M1 -> M2 [position 0]
8. M2 StopTake
9. delivered M2 -> engine/M1 [position 0]
10. M2 IdentifyX
11. delivered M2 -> engine/M1 [position 0]
```

**Diagnostic: PRV-022 PRV-024: identity material stays with its holders (1 actions)**

```text
1. engine/M1 Setup
```

**InheritToken: PRV-003 PRV-023 PRV-029: own runner keeps its promises (1 actions)**

```text
1. engine/M1 Setup
```

**OtherIdentity: PRV-003 PRV-023 PRV-029: own runner keeps its promises (1 actions)**

```text
1. engine/M1 Setup
```

**OtherState: PRV-003 PRV-023 PRV-029: own runner keeps its promises (4 actions)**

```text
1. engine/M1 Setup
2. crashed engine/M1
3. recovered engine/M1
4. engine/M1 Setup
```

**Unauthenticated: PRV-011 PRV-022: channel authenticates the other machine (2 actions)**

```text
1. engine/M1 Setup
2. delivered engine/M1 -> M2 [position 0]
```

### What MC5a shows, and what it does not

Within these separate bounds, admission by engine proof, per-connection server proof,
presented-token attribution, protected identity delivery, filtered local environment,
per-instance persistent state paths and authenticated confidential remote delivery
hold together. Each required precondition has a reachable witness, including an
engine restart and repeated L1/L2 setup. All nine seeded defects are detected;
the unauthenticated provider channel also exposes identity material, as registered.
There are no differences from the expected table and no promise counterexamples.

This does not prove cryptography, real TLS or transport implementations, operating-system
isolation, filesystem enforcement, liveness, unbounded behavior, or simultaneous fault
combinations. Authentication and confidentiality are checked flags in this model.
Connect is an abstract transport step; the certificate proof reply and application
messages, not TCP handshake packets, are subject to modeled loss and reordering.
The excluded placement, machine-provider, fetching and silence-time obligations remain
outside this evidence. State-directory removal and credential revocation on instance
removal are absent; none of the instances is removed. No engine conformance claim or
new conformance vector follows from these reference-model results.

The domain of trust is a limit: the engine machine and user, provider value storage,
run machine and gate, and signing keys/backups/operators are trusted for their values.
Root, the run's user and the provider's privileged user are excluded from X's powers;
programs under the engine's user can read keys, make the engine's proof and get past
gates. Runs sharing a user can read each other's credentials where the kernel permits
it. The model checks what the runner delivers, not whether its usual same-user runs
could later read another process's environment or directory. A process given a value
can use it for all its callers, and its model or children can read it. Confinement,
plain remote servers and their reach, and data deliberately returned by implementations
are outside this model. Real-machine evidence remains the separate S3/A74 work.
