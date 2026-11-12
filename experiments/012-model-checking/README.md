# 012 — model-checking harness

Status: executed 2026-11-11 under work orders 104 (MC1), 105 (MC2), 107 (MC3), 108 (MC4), 109 (MC5a), 110 (MC5b), 111 and 114 (MC6), 112 (MC7a), and 113 and 115 (MC7b), ruling 71's proof lane.
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

## MC5b — reading and confining secrets (work order 110)

### Model registered before execution

This model reads the promises, independently of engine code. Its five nodes are the engine
(including runtime provider P's server, run by its own runner), secret Resource K, I's node
(including P's part that holds I's run's value), R's server at `h`, and O. Each of I, P, R
and O has its own initially open Conduit listing `get-secret.v1` to K; I also has open
Conduits to R and O. P and R's reading Conduits can each close once in their respective
configurations. Distinct setup callers are 10 (P), 11 (I), 12 (R), with invocation IDs
100, 101 and 102. O serves I in invocation 103. No caller or invocation is inferred from
an implementation's current state by a property.

The saved script asks for P's setup run (read/run 0), I's run (read/run 1), R's connection
(read 2), two HTTP requests (IDs 0 and 1, with re-reads 3 and 4 when needed), then I's own
and O's onward `get-secret.v1` requests. P's local restart of I is an independent action on
I's node after I's run starts, using its held value and keeping run ID 1. It can interleave
with the rest of the script. The engine can crash at every state in recovery configurations.
Its own runner's P server ends; I's run and R's connection metadata survive. Recovery ends
pending reads unknown and discards late answers. Before the next provider call (I's `run.v1`
if still needed, otherwise a local `inspect-run.v1` of the existing I run), P is asked for
again only if its run was asked for before the crash. That run/read has ID 5, belongs to P's
original setup and has its caller. Failed P re-reads suppress the dependent provider call.
A failed setup read skips its work; independent setups and subsequent HTTP requests may
still proceed. Read 6 exists only in the mutant that refreshes I's lasting run.

R's connection holds its setup value until the engine crashes; every HTTP request uses it.
With values lost, the request gets its own re-read through R's current Conduit, belonging
to R's original setup, before sending. A successful re-read becomes the held connection
value for subsequent requests. The server answers the first HTTP request it receives with
a redirect to `h2`. Correct ignores it; the redirect mutant sends the header to `h2`.
P's handing I's value to its part on I's node and holding it there are represented as run
`secrets`, leaving the provider's transport/storage obligations to PRV-011 and MC5a.

Every event is in an external append-only ledger, stamped with the engine epoch when sent:
read dispatch decisions (instance, caller, invocation, work ID, Conduit, current openness,
dispatch and record availability), read endings and responders, instance requests, work
starts and source read IDs, value destinations, HTTP sends and URL, provider calls and P's
current run, Conduit closes, local restarts, recovery, redirects, failed records and discarded
answers. HTTP work is logged only at dispatch: consuming a successful header re-read
prepares the held value without logging work. A ledger-only bookkeeping invariant,
`MC5b: HTTP dispatches match work entries`, pairs the ordered HTTP work and dispatch
entries by epoch, HTTP ID and source read, including the redirect mutant. It is expected
to hold in every configuration and variant; it adds no promise to the five safety predicates.
The epoch advances at recovery, so remote events sent while the engine is down
keep the preceding epoch. Read IDs double as symbolic secret values: these are invented provenance tokens,
not credentials or real stores. A delivery with no source read is the environment mutant's
engine-environment value, delivered into I's run `secrets`. Properties use only this ledger. A test erases all node
data after each mutant's first refutation and verifies that the refutation remains.

### Promise accounting

Each assigned promise occurs exactly once in this accounting table. Supporting promises below
are outside its counts. P, S, V, F and H name the safety predicates in the next paragraph.

| Promise | Disposition | Property or reason |
| --- | --- | --- |
| SEC-004 | Not modelled | Static resolved-reference record form; to be shown by vectors |
| SEC-007 | Modelled | P: I's own and O's onward requests never dispatch |
| SEC-008 | Not modelled | Static authority to open a reading Conduit; to be shown by vectors |
| SEC-010 | Modelled | P: own open Conduit and secret delivered for its run |
| SEC-011 | Modelled | P: original setup invocation and caller, also for re-reads |
| SEC-012 | Modelled | S: failed reads prevent their identified work |
| SEC-013 | Modelled | H: setup read, connection-held header and original URL |
| SEC-014 | Modelled | F: no further read for I's lasting run |
| SEC-015 | Modelled | V: no value in a record |
| SEC-016 | Modelled | S: failed dispatch recording means unavailable without dispatch |
| SEC-017 | Modelled | V: no cache substituting for a later read |
| SEC-018 | Modelled | V: only the identified run's secrets/runner or connection's header |
| SEC-020 | Modelled | P: engine never answers a successful read or supplies its environment as secrets |
| SEC-023 | Modelled | H: current-authority re-read after value loss before HTTP |
| SEC-024 | Not modelled | Static template rejection; to be shown by vectors |
| PRV-026 | Modelled | F: provider run and fresh read precede post-restart calls |

Counts: **13 modelled; 3 not modelled, all static decisions to be shown by vectors**.
Supporting assumptions, outside those counts: PRV-002 (P dies with engine), PRV-007
(run inputs), PRV-010 (provider's local restart), PRV-011 (provider hand-off), REC-001
(dispatch records), IFC-017 (no redirect for non-plain R), IFC-041 (headers per HTTP),
ACC-011 (closure), ACC-034 (in-flight work may complete), IMP-007 (ended work is not replayed).

Safety properties, checked at every reachable state:

- **P**, `one read path`: own open Conduit, setup invocation/caller, no instance request
  dispatched, no engine-sourced successful answer or environment-sourced secrets.
- **S**, `failed read stops its work`: every work start has a succeeded source read;
  dispatch requires an open Conduit and kept dispatch record; an unkept record is unavailable.
- **V**, `values stay where sent`: each delivery names its read and permitted work/destination;
  no record/cache destination; engine-held values have the current epoch. I's provider-held
  value may survive because its run does. This covers the actual provider re-read/HTTP uses.
- **F**, `fixed run and provider re-read`: after I's run starts no further read for it;
  P's new run has a post-restart open-Conduit read, and post-restart calls follow P's run
  in that epoch.
- **H**, `remote header`: connection creation has its setup read; every HTTP send uses a
  successful current-epoch read through R's open Conduit, held for this connection, only to `h`.

### Configurations, bounds and fault restrictions

Each run has a **150,000 unique-state ceiling** (well below 200,000), **10-second timeout**,
five nodes, at most seven reads, one I run and local restart, one P rerun, one R connection,
two HTTP requests and one redirect. Seven messages per directed link safely bound all reads
and their answers, including old answers held across restart. No queue overflow is pruned.
The harness rejects truncation and asserts elapsed time after each full/refutation run.

Configurations split fault dimensions to keep total model time near 15 seconds. Enabled
harness faults have budget one each; all omitted dimensions have budget zero with reasons.
`fault_allowed` restricts crash to the engine, and loss/reordering to the named pair in both
directions. Recovery needs no additional restriction because only the engine can crash.
Read-result and recording failures are model-local actions, each guarded by its own one-use
flag: the hook cannot restrict state-dependent local actions. Closures are local one-use
saved changes. No harness change is needed.

| Configuration | Enabled harness faults | Additional local fault |
| --- | --- | --- |
| Base | None | None |
| ReadFault | None | One read ends with a reason, failed or unknown |
| RecordFault | None | One dispatch record cannot be kept |
| Recovery | Engine crash and recovery | None |
| CloseP | Engine crash and recovery | P's Conduit closes once |
| CloseR | Engine crash and recovery | R's Conduit closes once |
| LossK | One loss between engine and K | None |
| ReorderK | One reorder between engine and K; engine crash and recovery | None |
| LossH | One loss between engine and h | None |
| ReorderH | One reorder between engine and h | None |

ReorderK includes crash/recovery to overlap an ended read's undelivered messages and a later
read; without that overlap the serial reads cannot reorder. ReorderH can reorder the two
HTTP sends. No retries are implicit. A loss may leave a read pending forever; safety and
existential reachability require no progress or fairness. Duplication, partition, permanent
machine loss, timers, provider crashes independent of the engine and combined fault dimensions
are outside this model's bounded question.

Witness columns below: **u** unknown read; **k** failed record; **i/o** I/O request;
**l** I's local restart; **e** engine restart; **p** P run again; **c** P closed refusal;
**r** R closed re-read refusal; **n** HTTP after restart on old connection with a new read;
**d** redirect; **a** late answer discarded. R = reached, dash = unreachable.
Harness witnesses for every enabled fault are additional and expected reached.

### Expected verdicts registered before the first run

H = holds, X = refuted. Every cell is registered, including collateral mutant failures.
The Rust test's `expected` table checks every full run. Each mutant also has a separate
single-thread first refutation targeting its named property, printed with its action trace.

| Configuration / variant | P S V F H | u k i o l e p c r n d a |
| --- | --- | --- |
| Base / Correct | H H H H H | - - R R R - - - - - R - |
| ReadFault / Correct | H H H H H | R - R R R - - - - - R R |
| RecordFault / Correct | H H H H H | - R R R R - - - - - R - |
| Recovery / Correct | H H H H H | R - R R R R R - - R R R |
| CloseP / Correct | H H H H H | R - R R R R R R - R R R |
| CloseR / Correct | H H H H H | R - R R R R R - R R R R |
| LossK / Correct | H H H H H | - - R R R - - - - - R - |
| ReorderK / Correct | H H H H H | R - R R R R R - - R R R |
| LossH / Correct | H H H H H | - - R R R - - - - - R - |
| ReorderH / Correct | H H H H H | - - R R R - - - - - R - |
| Base / Onward | X H H H H | - - R R R - - - - - R - |
| Base / Borrow | X H H H H | - - R R R - - - - - R - |
| Recovery / Caller | X H H H H | R - R R R R R - - R R R |
| Base / Environment | X H H H H | - - R R R - - - - - R - |
| Recovery / EngineAnswer | X H H H H | R - R R R R R - - R R R |
| ReadFault / FailedRun | H X H H H | R - R R R - - - - - R R |
| RecordFault / Unrecorded | H X H H H | - R R R R - - - - - R - |
| Recovery / CachedP | H H X X H | R - R R R R R - - R R R |
| Base / RecordValue | H H X H H | - - R R R - - - - - R - |
| Base / RefreshI | H H H X H | - - R R R - - - - - R - |
| CloseP / ClosedP | X X H X H | R - R R R R R - - R R R |
| Recovery / CachedHeader | H H X H X | R - R R R R R - - R R R |
| Base / Redirect | H H H H X | - - R R R - - - - - R - |

### Limits registered with the model

Static document validation, real value contents/partial leaks, multiple secrets per work,
filters with predicates beyond current Conduit openness, retirement, connection replacement,
node retirement, several engine parts and arbitrary script lengths are omitted. The original
URL is an authenticated non-plain `https` URL by assumption; gate and identity enforcement
belong to MC5a. Provider transport confidentiality and storage are PRV-011's question,
not established by treating a hand-off as run secrets. No implementation or engine is tested.
The engine's machine and operating-system user, programs running as that user, the provider's
storage, I's machine/gate and signing-key storage/operators are within the domain of trust.
The model establishes no isolation from those parties, no protection from a malicious
provider or implementation, and no claim that plaintext becomes distinguishable from data
after an implementation sends it. These are limits, not additional safety promises.

### Bound split registered before its first run

The first CloseP full run hit the 150,000-state guard; it produced no interpreted verdict.
Split CloseP and CloseR into Early (`stage < 3`, before R's setup) and the original name
(`stage >= 3`, R's setup onward). Both enable the same single engine crash/recovery and
one closure, and their union preserves all closure positions. No loss/reordering is enabled.
The original expected rows above apply to the union and to each late part. ClosePEarly has
the same expected row as CloseP; CloseREarly has the same as CloseR except **r is unreachable**
(its closure prevents connection creation rather than a re-read). All safety cells remain H.
This addendum is registered before the split configurations' first execution.

P's local restart is placed at script step 2 on I's node; the engine waits for that action
before advancing to R's setup. It remains executable while the engine is crashed. The initial
implementation allowed that scripted action to commute with all later steps; this unnecessary
freedom enlarged the ledger permutations. A bookkeeping correction also records the actual
P run ID on provider calls: when the engine crashes before P's first setup starts, that first
run can occur in epoch 1 with ID 0, rather than the rerun ID 5. Neither correction changes
a promise or an expected verdict.

### First observed difference (table retained)

**MC5b-001 — expectation gap in Unrecorded/RecordFault (S and H).** The table expected
only S refuted; the full run also refutes H. The mutant dispatches a read whose record
cannot be kept and logs unavailable, yet leaves it pending. When K answers, the mutant
accepts that answer and can send a header based on a read that already ended unavailable.
The H predicate requires a successful source read, as the promises do. This is collateral
mutant behavior missed by the registered table, not a contradiction in the specification.
The table is unchanged; the test checks this separately recorded observation. A separate
BFS refutation prints the shortest first H trace alongside the named S refutation.

**MC5b-002 — expectation gap in CachedHeader/Recovery (fresh-read witness n).** The table
expected n reached; exhaustive exploration finds it unreachable. If an existing connection
crosses the restart, this mutant retains its header, so it never takes the missing-value
branch that makes a fresh read. A connection first made after restart does not satisfy n's
old-connection condition. The expected V/H refutations still occur. The table stays unchanged,
and the test checks n missing beside it. This is a reachability expectation defect, not a
specification defect; the named H trace shows the stale header send.

### Recovery/local-fault combinations registered before their first run

Two additional configurations exercise failure on the re-reads themselves, rather than only
on initial setup reads. **RecoveryReadFault** enables one engine crash/recovery and one local
read-result failure; **RecoveryRecordFault** enables one engine crash/recovery and one failed
dispatch record. The same bounds apply. Their full rows, registered before execution, are:

| Configuration / variant | P S V F H | u k i o l e p c r n d a |
| --- | --- | --- |
| RecoveryReadFault / Correct | H H H H H | R - R R R R R - - R R R |
| RecoveryRecordFault / Correct | H H H H H | R R R R R R R - - R R R |

All enabled harness witnesses are expected reached. Local faults remain globally bounded by
one, so an HTTP re-read that fails can be followed by a successful read for the second HTTP
request's different ID. Instance-request ledger entries explicitly hold the refused answer.

### Dated results — 2026-11-11

Verifier refresh: successful HTTP re-reads now prepare the connection's held value without
logging HTTP work before dispatch. The new bookkeeping invariant first refuted the unfixed
Correct/Recovery model, then held in all 27 full runs after the correction. The redirect
mutant also logs work for its additional dispatch. Registered safety and reachability cells
remain unchanged; the table below refreshes the observed counts and times. All 14 printed
first-refutation traces, including the collateral H trace, are unchanged.

Measured on Linux x86_64 with the pinned Rust **1.97.1** (`8bab26f4f`, 2026-07-14),
Stateright 0.31.0, inherited `CARGO_BUILD_JOBS=6`, two checker threads for full runs and
one for first refutations. This refresh uses the locally installed pinned toolchain;
the earlier round used installed stable Rust 1.99.1 because the pin was then unavailable.
No toolchain was downloaded in this round.

The ordinary release command passed **23 unit tests, 0 failures, 0 doc tests**, in
**14.67 seconds wall time**, including compilation (test execution 10.29 s). The counts
command passed the same totals in **9.74 seconds wall time** (test execution 9.57 s).
MC5b contributes four tests: full runs, named refutations, the collateral refutation and
saved-state/metadata checks. Formatting and clippy with warnings denied passed.

Counts below come from `cargo test --release -- --nocapture --test-threads=1`.
All full runs completed without truncation; enabled harness witnesses were reached.
The safety and reach columns use the complete legends of the registered table. The HTTP
bookkeeping invariant held in every row and is outside the promise accounting.

| Configuration / variant | Unique states | Depth | Seconds | Observed P S V F H | Observed u k i o l e p c r n d a |
| --- | --- | --- | --- | --- | --- |
| Base / Correct | 57 | 22 | 0.001 | H H H H H | - - R R R - - - - - R - |
| ReadFault / Correct | 765 | 22 | 0.005 | H H H H H | R - R R R - - - - - R R |
| RecordFault / Correct | 165 | 22 | 0.001 | H H H H H | - R R R R - - - - - R - |
| Recovery / Correct | 2,111 | 34 | 0.020 | H H H H H | R - R R R R R - - R R R |
| RecoveryReadFault / Correct | 24,800 | 37 | 0.283 | H H H H H | R - R R R R R - - R R R |
| RecoveryRecordFault / Correct | 4,849 | 34 | 0.043 | H H H H H | R R R R R R R - - R R R |
| CloseP / Correct | 18,711 | 35 | 0.251 | H H H H H | R - R R R R R R - R R R |
| ClosePEarly / Correct | 15,214 | 34 | 0.206 | H H H H H | R - R R R R R R - R R R |
| CloseR / Correct | 17,981 | 35 | 0.250 | H H H H H | R - R R R R R - R R R R |
| CloseREarly / Correct | 4,867 | 34 | 0.039 | H H H H H | R - R R R R R - - R R R |
| LossK / Correct | 60 | 22 | 0.001 | H H H H H | - - R R R - - - - - R - |
| ReorderK / Correct | 2,601 | 34 | 0.024 | H H H H H | R - R R R R R - - R R R |
| LossH / Correct | 104 | 22 | 0.001 | H H H H H | - - R R R - - - - - R - |
| ReorderH / Correct | 81 | 22 | 0.001 | H H H H H | - - R R R - - - - - R - |
| Base / Onward | 74 | 23 | 0.001 | X H H H H | - - R R R - - - - - R - |
| Base / Borrow | 57 | 22 | 0.001 | X H H H H | - - R R R - - - - - R - |
| Recovery / Caller | 2,111 | 34 | 0.020 | X H H H H | R - R R R R R - - R R R |
| Base / Environment | 57 | 22 | 0.001 | X H H H H | - - R R R - - - - - R - |
| Recovery / EngineAnswer | 5,811 | 34 | 0.073 | X H H H H | R - R R R R R - - R R R |
| ReadFault / FailedRun | 1,065 | 23 | 0.008 | H X H H H | R - R R R - - - - - R R |
| RecordFault / Unrecorded | 211 | 22 | 0.002 | H X H H X | - R R R R - - - - - R - |
| Recovery / CachedP | 1,612 | 31 | 0.013 | H H X X H | R - R R R R R - - R R R |
| Base / RecordValue | 57 | 22 | 0.001 | H H X H H | - - R R R - - - - - R - |
| Base / RefreshI | 110 | 24 | 0.001 | H H H X H | - - R R R - - - - - R - |
| CloseP / ClosedP | 21,351 | 35 | 0.272 | X X H X H | R - R R R R R - - R R R |
| Recovery / CachedHeader | 2,015 | 30 | 0.021 | H H X H X | R - R R R R R - - - R R |
| Base / Redirect | 77 | 22 | 0.001 | H H H H X | - - R R R - - - - - R - |

MC5b full runs total **1.541 s**; maximum **24,800 states**,
**depth 37**, and **0.283 s** per full run.
Every correct row holds all five properties. Every mutant refutes its named property.
The only differences from registered verdicts/reachability are MC5b-001 (Unrecorded also
refutes H) and MC5b-002 (CachedHeader does not reach n), described above. No correct-model
property asks for a control or yields a counterexample against the promises.

### First refutation traces

These are the first single-thread BFS traces printed by the harness, including the extra
collateral H trace. `E` is the engine with P's server; `I-node` is I's node with P's part.
`deliver A -> B` delivers position zero on that link. `Advance` executes the next scripted
read, consumes its result, asks for its work, or sends the next instance/HTTP request.
The final action is the first failing state of the named property. No trace is inferred
from an expected outcome.

**Unrecorded (collateral H): 15 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. E Advance
6. deliver E -> K
7. deliver K -> E
8. E Advance
9. I-node RestartI
10. E Advance
11. E RecordFault
12. deliver E -> K
13. deliver K -> E
14. E Advance
15. E Advance
```

**Onward: 18 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. E Advance
6. deliver E -> K
7. deliver K -> E
8. E Advance
9. I-node RestartI
10. E Advance
11. E Advance
12. deliver E -> K
13. deliver K -> E
14. E Advance
15. E Advance
16. E Advance
17. E Advance
18. E Advance
```

**Borrow: 5 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. E Advance
```

**Caller: 11 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. crashed E
6. recovered E
7. E RecoverLedger
8. E Advance
9. deliver E -> K
10. deliver K -> E
11. E Advance
```

**Environment: 8 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. E Advance
6. deliver E -> K
7. deliver K -> E
8. E Advance
```

**EngineAnswer: 11 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. crashed E
6. recovered E
7. E RecoverLedger
8. E Advance
9. deliver E -> K
10. deliver K -> E
11. E Advance
```

**FailedRun: 7 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. E Advance
6. E ReadFault(Reason)
7. E Advance
```

**Unrecorded: 1 actions.**

```text
1. E RecordFault
```

**CachedP: 7 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. crashed E
6. recovered E
7. E RecoverLedger
```

**RecordValue: 3 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
```

**RefreshI: 9 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. E Advance
6. deliver E -> K
7. deliver K -> E
8. E Advance
9. I-node RestartI
```

**ClosedP: 15 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. E Advance
6. deliver E -> K
7. deliver K -> E
8. E Advance
9. I-node RestartI
10. E Advance
11. crashed E
12. recovered E
13. E RecoverLedger
14. E Close(0)
15. E Advance
```

**CachedHeader: 23 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. E Advance
6. deliver E -> K
7. deliver K -> E
8. E Advance
9. I-node RestartI
10. E Advance
11. E Advance
12. deliver E -> K
13. deliver K -> E
14. E Advance
15. crashed E
16. recovered E
17. E RecoverLedger
18. E Advance
19. deliver E -> K
20. deliver K -> E
21. E Advance
22. E Advance
23. E Advance
```

**Redirect: 17 actions.**

```text
1. E Advance
2. deliver E -> K
3. deliver K -> E
4. E Advance
5. E Advance
6. deliver E -> K
7. deliver K -> E
8. E Advance
9. I-node RestartI
10. E Advance
11. E Advance
12. deliver E -> K
13. deliver K -> E
14. E Advance
15. E Advance
16. deliver E -> h
17. deliver h -> E
```

### What MC5b shows and leaves open

Within these scripts, saved/volatile boundaries and fault budgets, the five ledger predicates
hold in all 14 correct configurations, and all 13 seeded defects are detected. The twelve
model witnesses and enabled harness-fault witnesses rule out the listed vacuous branches;
the observed missing witnesses are printed and checked per row. The unrecorded-read mutant's
additional failure and the stale-header mutant's missing fresh-read witness remain visible
beside the unchanged registration.

This is bounded reference-model evidence for 13 of the 16 assigned promises, with the three
static decisions left to vectors. It proves no unbounded result, fairness, implementation
conformance, confidentiality of a real transport/store, or isolation from the stated domain
of trust. The omitted cases and supporting assumptions above remain limits. Pinned-toolchain
validation passed in this refresh.


## MC7a — controls, revisions and outcomes (work order 112)

### Model and expectations registered before execution

Registered 2026-11-11, before the first MC7a model-checker run. This is an independent
reference model in `src/controls.rs`, not engine validation. It applies the coordinator's
readings in the order, including ruling 75 A1's pending fence; the specification is not edited.
The prior MC2 omission of REV-010 is superseded by this section's accounting, once only for MC8.

Nodes are callers A and B, the engine, and one implementation process. The engine saves its
invocation, sent patch, outcome, fence, configuration, revision, lifecycle, script position and
unknown-setting mask across its crash. Recovery records an active invocation as unknown before
processing another engine event; it neither fences nor replays it. The implementation has a
separate job and run lifetime. Every property reads only the external ledger. Event emission
logs the message and the facts then, before delivery; fault transitions cannot rewrite history.
The ledger records physical application separately from implementation reports and engine stores.

**Part 1, controls and revisions.** Initial stored configuration is `(x=0,y=2)` with revision
r0 and schema `x <= y`. A sends a two-setting control `(1,1)` and a probe without waiting.
On dispatch, B sends seven requests without waiting: a competing control, setup, retirement,
unauthorized request, invalid request, in-progress repeat, and a pre-existing succeeded repeat
with no outputs. The implementation link carries the dispatch and a probe without waiting.
Thus both caller and implementation reordering have two messages on one directed link.
The last repeat is answered succeeded without dispatch even while the control is active;
refused and invalid precede contention. The other permitted fresh contenders conflict at once.

A bounded script barrier drains these requests and probes before implementation completion,
cancellation, retirement, timers or the engine crash. This isolates admission contention from
completion without enumerating independent contender successes. Ordinary deliveries remain
FIFO and enabled; the barrier is a model bound, not a claim about a real engine's scheduler.
After it, application, reply delivery, a timer and retirement race freely. The implementation
may apply both settings, neither, or only the first; it reports succeeded, failed, or unknown
accordingly, and can report unknown without knowing whether an effect occurred. Silence is a
separate one-use implementation choice in Silence, not a hidden transport fault.

Unknown changes neither stored values nor revision and labels exactly the supplied settings
unknown. An actor may inspect and send one reconciliation, choosing a current, stale or missing
revision for occurred, or no revision for absent. Reconciliation does not fabricate another
implementation report. A later valid engine-implemented change to `(0,0)` may be stored before
the actor reconciles; it is B's management change, not an automatic repair. The serialized suffix
stores `(0,0)`, a no-op `(0,0)`, and the original `(0,2)`, exercising advance, no advance and
revision freshness on return. Each store is recorded, including no-ops.

Schema uses a one-setting control `x=1`, initially valid with y=2. It requires the later stored
change to `(0,0)` before reconciliation: applying the original x=1 now produces `(1,0)` and
is invalid. No invalid merge is manufactured against the original configuration. Retired races
one immediate PLC-017 retirement against completion; it dispatches nothing and fences nothing.
Removed additionally removes the retired instance before reconciliation. These branches store
nothing after retirement, including on success, and reconciliation changes only the outcome.

There is one attempt, so “every attempt is fenced” is a conjunction over one element; MC2 owns
multiple-attempt identity/retry behavior. An answer fences even when late, while leaving a
recorded outcome unchanged. A timer, cancellation, silence or engine restart fences nothing.
The run's end fences; a queued dispatch addresses that ended run and cannot apply afterward.
Work an implementation retains after answering, remote APIs, and independent external actors
changing the physical settings are outside this model. Unfenced deliberately follows OPS-014's
present text: an actor can record absent before the attempt is fenced, and a late effect can
then land. It is an expected refutation of the pending fence and the registered absence control,
not a new interpretation of what an implementation's failed report means.

**Part 2, outcomes, cancellation and model work.** Outcomes uses the same single dispatched
invocation and physical two-setting effect, plus malformed responses (missing outcome/reason,
both, disagreeing isError, invalid declared output), and valid implementation reasons with no
effect. It models the OPS-007 cases as explicit response classes, not a JSON/MCP parser. The
protocol's -32021 and IFC-046 exceptions are outside those response classes and are not treated
as missing results. OPS-006 is accounted as a static wire-shape decision; malformed implementations
are adversarial input to the engine's OPS-007 handling, not evidence that such implementations
conform. Stopping before a response is represented by the separate silence choice and run end.

Cancellation closes the request's response stream while active, forwards cancellation, then
records its end in a separate transition. A failed result received before that record confirms
no effect; a later failed result fences but changes no outcome. The implementation may finish
meanwhile or ignore cancellation, and the timer may end the invocation. All physical claims
are checked at report emission, never inferred from a reconciled result.

Repeats starts from three explicitly recorded succeeded invocations: an effectful operation
with declared outputs and a filter, an effectful operation with no outputs and a filter, and an
operation with outputs no longer kept and no filter. They are kept invocation facts, not new
implementation calls; matching caller, sender, target, key and contents are fixed. The first
and third answer conflict with the invocation ID and no outputs; the second answers succeeded
under supporting OPS-009. No variant requires redispatch of a keyed repeat.

ModelWork runs all fourteen IMP-002 causes, including setup and retire hook dispatch, runtime
restart, machine reachability and an engine restart, as explicit script events with no model
work. It then sends unmarked and marked parent operations through an open listing Conduit,
and marked requests through closed and non-listing Conduits. Both admitted implementations
send an onward request for a marked operation while serving their parent. Only the marked
parent is admitted; its leaf model work has its own dispatch. All other access and parent
lifetime checks are fixed to permitted/in-progress, so the positive branch says only that
IMP-008 is not its reason for refusal. These are finite event abstractions, not implementations
of registration, lifecycle, hook orchestration, transport parsing or a model provider.

### MC7a catalogue accounting (31 promises)

Each row below is counted once. “Static” means **a static decision, to be shown by vectors**;
there is no race or fault in the omitted document/schema/wire-shape decision. No vector is
written by this order. OPS-006's syntax is static even though malformed inputs are exercised
for OPS-007. Modelled means the bounded property projection described below, not all possible
instances of that promise.

| Promise | Accounting | Property or reason |
| --- | --- | --- |
| OPS-001 | Modelled | O |
| OPS-002 | Modelled | C |
| OPS-003 | Modelled | C |
| OPS-004 | Modelled | C |
| OPS-005 | Modelled | N |
| OPS-006 | Not modelled | Static: a static decision, to be shown by vectors |
| OPS-007 | Modelled | M |
| OPS-019 | Not modelled | Static: a static decision, to be shown by vectors |
| OPS-020 | Not modelled | Static: a static decision, to be shown by vectors |
| OPS-023 | Not modelled | Static: a static decision, to be shown by vectors |
| OPS-026 | Not modelled | Static: a static decision, to be shown by vectors |
| OPS-027 | Modelled | K |
| OPS-029 | Not modelled | Static: a static decision, to be shown by vectors |
| OPS-033 | Not modelled | Static: a static decision, to be shown by vectors |
| OPS-034 | Modelled | P |
| OPS-036 | Not modelled | Static: a static decision, to be shown by vectors |
| REV-001 | Not modelled | Static: a static decision, to be shown by vectors |
| REV-003 | Modelled | A |
| REV-005 | Modelled | F |
| REV-006 | Not modelled | Static: a static decision, to be shown by vectors |
| REV-007 | Modelled | E |
| REV-009 | Modelled | U |
| REV-010 | Modelled | R |
| REV-013 | Modelled | V |
| REV-014 | Modelled | T |
| REV-015 | Not modelled | Static: a static decision, to be shown by vectors |
| IMP-001 | Modelled | W |
| IMP-002 | Modelled | W |
| IMP-008 | Modelled | I |
| TPL-011 | Not modelled | Static: a static decision, to be shown by vectors |
| TPL-013 | Not modelled | Static: a static decision, to be shown by vectors |

Totals: **18 modelled, 13 not modelled, all 13 static**. Supporting promises are outside the
31 and the count: OPS-008–014, OPS-018, OPS-024, IMP-007, REV-002, REV-004, REV-011,
REV-012, CTR-006, ACC-019, ACC-033, PLC-017 and IFC-025. The fence is supporting ruling
75 A1/order 077, spec pending. L checks supporting OPS-013 and is also outside the count.

### MC7a properties and preconditions

Safety order used in the tables: **A F E U R V T O C N M K P W I G B L**. All are invariants
of recorded events, not eventual claims; no fairness or liveness is assumed. G checks the
pending fence, B checks the absence control, and L checks late replies. H means holds,
F means refuted. A/F apply only at actual stored changes; R excludes schema-invalid merges
which V checks, and C applies only to implementation reports, not to actor reconciliations.
Each modeled promise's complete when/then is quoted here, with its level where it negates then.

**A** — REV-003.

> REV-003 when: An object's stored definition or configuration changes.
>
> must then: Its revision advances.


**F** — REV-005.

> REV-005 when: An object's revision advances.
>
> must not then: It takes a value the object has had before.


**E** — REV-007.

> REV-007 when: A control succeeds.
>
> must then: It has applied exactly the values it was sent and, unless its instance was retired while it ran, exactly those values are stored, each at the pointer that the control's `changes` gives for its input. It returns no `settings`, and an implementation never applies another value in the place of one it was sent.


**U** — REV-009.

> REV-009 when: A control's outcome is unknown.
>
> must then: The stored configuration and revision are unchanged, and the settings the control addressed are reported as unknown. They are the settings of the inputs the request supplied, at the pointers the control's `changes` gives for those inputs.


**R** — REV-010.

> REV-010 when: A control whose outcome is unknown is reconciled under OPS-014.
>
> must then: A reconciliation that records the control as having taken effect, on an instance that exists and is not retired, is a request to change that instance: it names the instance's current revision, and the values the control was sent, which the engine keeps under OPS-008, are stored at the pointers of its `changes`. A reconciliation that records that the effect did not occur names no revision and stores nothing. Where the instance is retired or no longer exists, a reconciliation of the control names no revision, whichever way it records the effect: it records the outcome and stores nothing.

> observable (the remainder of REV-010, quoted in full): After a reconciliation that stores values, the stored configuration equals the current configuration with the values the control was sent applied at those pointers, whatever other change was stored after the control was dispatched. The revision advances under REV-003 if anything changed, and those settings are no longer reported as unknown. Such a reconciliation is a request to change an object, so REV-002 and REV-011 apply to it as to any other: one that names a revision that is not current reports conflict, and one that names none reports invalid. One where those values, applied to the current configuration, would make it fail the type's schema is REV-013. A reconciliation that records that the effect did not occur changes no object and so names no revision: the control's outcome becomes failed, and the settings it addressed are no longer reported as unknown. One that names a revision all the same reports invalid, under OPS-031. In `reconcile.v1` the revision is the input `revision`. The values are not stated in the request: the engine stores those it sent. The request is authorized under ACC-033 against the target that manages the instance: the Pool that owns a Resource or the Fleet that owns an Agent, and `management` for other families, a retired owner or an instance that no longer exists. It is not authorized by a Conduit to the instance itself, as the control was. A retired instance takes no change, under LIF-015, so a reconciliation of a control dispatched to it records the outcome alone, whichever way it records the effect, as REV-007 stores nothing for a control that succeeds after its instance was retired, and the settings the control addressed are no longer reported as unknown. Once the instance no longer exists there is no current revision to name and no configuration to store values in: the reconciliation is authorized against `management`, under ACC-033, and records that the effect occurred or that it did not, and nothing else. One that names a revision all the same reports invalid, under OPS-031. ACC-055 also limits which invocations a holder other than root may reconcile.


**V** — REV-013.

> REV-013 when: A reconciliation records a control as having taken effect, and the values the control was sent, applied to the current configuration at the pointers of its `changes`, would make that configuration fail the type's schema.
>
> must then: The reconciliation is not dispatched, and reports invalid.


**T** — REV-014.

> REV-014 when: A request to set up, retire or change an instance arrives while another setup, retirement or change of it, the engine's own included, has been dispatched and has not ended, and the request is not a repeat that is answered without a dispatch, under OPS-009, OPS-012, OPS-032 or OPS-034.
>
> must then: It is not dispatched, and reports conflict at once.


**O** — OPS-001.

> OPS-001 when: An invocation that was dispatched has ended, and its implementation did not answer with a reason in place of an outcome under IFC-026.
>
> must then: It has exactly one outcome: succeeded, failed or unknown.


**C** — OPS-002, OPS-003, OPS-004.

> OPS-002 when: An implementation reports the outcome succeeded.
>
> must then: The effect the operation's contract describes has occurred.

> OPS-003 when: An implementation reports the outcome failed.
>
> must then: The effect the operation's contract describes has not occurred, in whole or in part.

> OPS-004 when: An implementation cannot confirm that the effect occurred, cannot confirm that it did not, or knows that it occurred only in part.
>
> must then: It reports the outcome unknown.


**N** — OPS-005.

> OPS-005 when: A request is not dispatched, and it is not a repeat that OPS-009 or OPS-012 answers with the invocation's outcome; or the implementation answers a request with a reason in place of an outcome, under IFC-026.
>
> must then: It has no outcome. It reports why, as exactly one of: refused, invalid, conflict or unavailable.


**M** — OPS-007.

> OPS-007 when: No result arrives from an implementation that carries Conduit's keys, other than the protocol's error -32021 whose `data.requiredCapabilities` names Conduit's extension, with which IFC-008 has such an implementation say that it performed nothing, or names a capability that the call did not declare, which IFC-047 has such an implementation answer only where it has performed nothing for the call, and other than an error after which the engine sends the call once more under IFC-046; the implementation stops before returning one; a result from an implementation that carries Conduit's keys states neither an outcome nor a reason under IFC-026, or states both; such a result states an outcome with which its `isError` disagrees; or a declared output fails validation in a result that would otherwise have the outcome succeeded, whether or not the implementation is marked `plain`.
>
> must then: The invocation's outcome is unknown.


**K** — OPS-027.

> OPS-027 when: An invocation is cancelled while it is in progress: the request that dispatched it is cancelled by the protocol's own means, which are `notifications/cancelled` on stdio, the close of the response stream on Streamable HTTP, and `tasks/cancel` for a task.
>
> must then: The engine passes the cancellation on to the implementation by the protocol's means. The invocation's outcome is unknown, unless its implementation confirms that nothing was done.


**P** — OPS-034.

> OPS-034 when: A request arrives that matches, as OPS-009 describes a match, the request of an invocation that succeeded, of an operation that declares outputs, where no filter applies to the request and the engine does not keep those outputs, or where a filter applies to it and the operation is not one that OPS-024 dispatches again: its contract states an `effect`, it is a control, or its contract is marked `startsModelWork: true`.
>
> must then: It is a repeat. Nothing is dispatched, and it reports conflict. Its result's `_meta` holds the engine's ID for that invocation.


**W** — IMP-001, IMP-002.

> IMP-001 when: Model work starts.
>
> must then: It is the effect of an operation whose contract declares that it starts model work, dispatched through an open Conduit that lists that operation.

> IMP-002 when: An object is registered, instantiated, set up, retired, inspected or observed; a Conduit is opened or closed; an operation not marked as starting model work is invoked on an Agent, whatever it delivers to the Agent; a setup hook or a retire hook is dispatched; a runtime provider restarts a run; a machine is reported reachable again; or the engine restarts.
>
> must not then: Model work starts because of it.


**I** — IMP-008.

> IMP-008 when: A request for an operation marked as starting model work names, in `io.github.shrav-k.conduit/serving`, an invocation of an operation that is not so marked.
>
> must then: The request is not dispatched, and reports refused.


**G/B**: when an authorized actor records an unknown invocation absent (G), every attempt
must already be fenced under the order's reading; when its absence was recorded (B), the
modeled attempt cannot later apply any part of its effect. G and B are controls for the
pending fence, not catalogue additions. **L**: when a response arrives after the invocation
ended, without a new attempt or reconciliation, the earlier recorded outcome stays unchanged.
This deliberately avoids MC3's unconditional late-reply/outcome property.

### MC7a configurations, bounds and variants

Correct runs in Control with Base, LossCaller, ReorderCaller, LossImplementation,
ReorderImplementation, Crash and Silence, then Schema, Retired, Removed, Outcomes,
Cancellation, Repeats and ModelWork with Base. Every run enables one engine timer. Loss
and reordering budgets are one each in their named part, on both directions of caller/engine
or engine/implementation links respectively. Crash and recovery each have budget one, only
for the engine, after the contention batch drained and while the invocation is active.
`gate_remaining_faults=true` routes timer and recovery through `fault_allowed`; windows are
in `fault_enabled` and timer eligibility in `timer_enabled`. Faults cannot affect the late
serialized suffix. Silence has its own bound of one and must have a reconciliation witness.
Every enabled harness fault has its automatic reach property, expected reached in every run.

Zero budgets: duplication is outside this order (explicit repeats are script requests, and
dispatch transport does not copy); partition/heal are outside this order; permanent machine
loss is outside it (the PLC-017 declaration is a script event, not physical death); other
zero loss/reorder/crash/recovery budgets isolate the named fault in another configuration.
All configurations have four nodes, seven messages per link, one primary attempt, one timer,
one reconciliation, at most one retirement/removal/run end and three suffix stores. Queues
are asserted, never silently pruned. The unique-state ceiling is 150,000 and timeout nine
seconds, below this order's limits. A full run must finish exhaustively for any verdict.

Variants, run only with Base in their named scene, change exactly the following mechanism:

| Variant | Scene | Seeded defect | Target |
| --- | --- | --- | --- |
| StaleRevision | Control | Stores change without advancing revision | A |
| ReusedRevision | Control | Alternates two issued revisions, reusing one | F |
| Substitute | Control | Applies x=0 instead of supplied x=1, reports success | E (also C) |
| StoreUnknown | Control | Stores sent values on unknown | U |
| WrongReconcile | Control | Stores initial values instead of sent values | R |
| AcceptInvalid | Schema | Skips merged schema check | V |
| QueueContender | Control | Keeps the competing change queued | T |
| DoubleOutcome | Outcomes | Records two outcomes together | O (also M) |
| FalseSuccess | Outcomes | Reports success without an effect | C |
| PartialFailed | Outcomes | Reports failed after a partial effect | C |
| UncertainFailed | Outcomes | Reports failed when it cannot confirm the effect | C |
| OutcomeForReason | Outcomes | Reports failed for an unauthorized undispatched request | N (also T) |
| TrustMalformed | Outcomes | Records failed on malformed response | M |
| DropCancellation | Cancellation | Does not forward the cancellation | K |
| CancelFailed | Cancellation | Records failed with no confirmation | K |
| LeakOutputs | Repeats | Gives kept outputs through the filter | P |
| RepeatNoOutputsConflict | Repeats | Reports conflict for no-output succeeded repeat | P |
| ImplicitWork | ModelWork | Starts work on an implicit event | W |
| ClosedWork | ModelWork | Starts work without an open listing Conduit | W |
| AllowOnward | ModelWork | Admits onward work for an unmarked parent | I |
| Unfenced | Control | Present OPS-014 permits recording absent before fence | B (also G) |
| LateConfirmation | Cancellation | Late failed reply replaces unknown | L |

### MC7a expected table, fixed before the first run

Every unlisted safety entry is H. Each row expands to the full safety order above.
Correct/Control/* means **each** of the seven fault parts listed above; mutants run only
where the preceding table says. Every reach not in the row's set is expected unreached.
The Rust `expected` function encodes these same sets, and tests compare every full run.
One separate one-thread BFS per mutant prints its first targeted trace. Later observations
that differ must be recorded beside this table, never substituted into it.

| Configuration/variant | Expected refuted safety | Expected reached w indices |
| --- | --- | --- |
| Correct/Control/* | None | 0–8, 10, 15–18, 29 |
| Correct/Schema/Base | None | 0–4, 6, 9–10, 15–17, 29 |
| Correct/Retired/Base | None | 0–8, 10–13, 15–18, 29 |
| Correct/Removed/Base | None | 0–6, 10–12, 14–18, 29 |
| Correct/Outcomes/Base | None | 10, 15–19 |
| Correct/Cancellation/Base | None | 10, 15–18, 20–22 |
| Correct/Repeats/Base | None | 10, 15–18, 23–25 |
| Correct/ModelWork/Base | None | 10, 15–18, 26–28 |
| StaleRevision | A | Control's set |
| ReusedRevision | F | Control's set |
| Substitute | E, C | Control's set |
| StoreUnknown | U | Control's set |
| WrongReconcile | R | Control's set |
| AcceptInvalid | V | Schema's set minus 9 |
| QueueContender | T | Control's set |
| DoubleOutcome | O, M | Outcomes' set |
| FalseSuccess | C | Outcomes' set minus 16 |
| PartialFailed | C | Outcomes' set |
| UncertainFailed | C | Outcomes' set minus 17 |
| OutcomeForReason | N, T | Outcomes' set |
| TrustMalformed | M | Outcomes' set |
| DropCancellation | K | Cancellation's set |
| CancelFailed | K | Cancellation's set |
| LeakOutputs | P | Repeats' set |
| RepeatNoOutputsConflict | P | Repeats' set |
| ImplicitWork | W | ModelWork's set |
| ClosedWork | W | ModelWork's set |
| AllowOnward | I | ModelWork's set |
| Unfenced | G, B | Control's set |
| LateConfirmation | L | Cancellation's set |

Reach indices (each is a `sometimes` property of the external ledger):

| Index | Required recorded precondition/event |
| --- | --- |
| 0 | Stored values change |
| 1 | A store leaves values equal (supporting REV-004) |
| 2 | Stored values return to the original after a different value |
| 3 | A control succeeds |
| 4 | A control ends unknown |
| 5 | An actor reconciliation records occurred |
| 6 | An actor reconciliation records absent |
| 7 | A reconciliation names a stale revision and reports conflict (R separately checks valid merges) |
| 8 | A valid merge reconciliation names no revision and is invalid |
| 9 | After a later stored change, a current-revision merge fails the schema |
| 10 | B's competing control arrives while A's control is active |
| 11 | Retirement happens while a control is active |
| 12 | A control succeeds after that retirement |
| 13 | A retired, existing instance's unknown control is reconciled |
| 14 | A removed instance's unknown control is reconciled |
| 15 | An implementation reports succeeded |
| 16 | An implementation reports failed |
| 17 | An implementation reports unknown with no confirmed effect |
| 18 | The implementation applies only part of a two-setting effect |
| 19 | A malformed implementation response is emitted |
| 20 | A cancelled invocation is recorded unknown |
| 21 | A cancelled invocation is recorded failed on prior confirmation |
| 22 | A failed confirmation arrives after a cancelled invocation ended |
| 23 | A filtered repeat of a succeeded invocation with outputs arrives |
| 24 | A filtered repeat of a succeeded invocation without outputs arrives |
| 25 | A repeat arrives with no filter and outputs no longer kept |
| 26 | All fourteen implicit causes occur (W separately checks that they start no model work) |
| 27 | A marked parent sends an onward marked request |
| 28 | An unmarked parent sends an onward marked request |
| 29 | A reconciliation is reached after the configured fault; Silence requires silence |

The same run need not consume all budgets on one path: the full run must reach each enabled
fault, and w29 additionally couples the selected fault to a subsequent unknown/reconciliation.
The timer is reachable in every configuration, including those with crash/recovery. Schema's
invalid merge is not a proxy for stale/missing revision; it has its own current-revision witness.
This model supplies bounded projections for S5 P2/P3/P5. Static validation/default/discovery
cases in S5 P4 remain for vectors; real-client behavior and external verification remain S5's.

Supporting preconditions quoted before the first run (outside the catalogue count):

> REV-004 when: An object's stored definition and configuration do not change.
>
> must not then: Its revision advances.

> REV-002 when: A request to change an object names a revision that is not the object's current revision.
>
> must then: The change is not dispatched, and reports conflict.

> REV-011 when: A request to change an object names no revision.
>
> must then: The change is not dispatched, and reports invalid.

> OPS-009 when: A request arrives from the instance that sent an earlier request, with the same caller, target, idempotency key and contents as that request; the invocation that request began succeeded; and either no filter applies to the request, and (the operation declares no outputs and is not implemented by a plain server, or the engine still keeps the invocation's outputs and any plain content), or a filter applies to it, and the operation declares no outputs and is not one that OPS-024 dispatches again: its contract states an `effect`, it is a control, or its contract is marked `startsModelWork: true`.
>
> must then: The request is a repeat, not a new request. Nothing is dispatched, and the result is that invocation's: the outcome succeeded, its outputs and any plain content.

> OPS-013 when: An invocation's outcome is unknown, and no new attempt at it is dispatched under OPS-012.
>
> must then: The outcome stays unknown until it is reconciled.

### MC7a first observation and inspection refinement

The first full matrix run completed all 36 runs (no bound truncation), with one difference:
**MC7a-001 — expectation gap, AcceptInvalid/Schema/Base**. V was refuted as expected, but
w5 and w7 were reached as well. Skipping the invalid-merge check lets a current revision
record occurred, or lets a stale revision reach conflict. The original table omitted those
two consequences of the mutation. No correct-model property failed, and this is not a
specification finding. The table and `expected()` remain unchanged; the test checks the
observed extra reaches beside the registration. All other entries matched.

Before the next run, the probes were refined into explicit inspection observations: a
caller probe returns whether the engine has accepted the control, and the implementation
probe returns whether it has accepted the dispatch. They record that fact in the ledger.
Reordering a probe before its preceding request therefore has an observable difference,
without fencing the control. Both are real inspection requests; their result transport is
abstracted as an atomic reply to that inspection. The script barrier still precedes physical
application, as registered. This adds observer history, not a changed safety behavior. The
immutable fault-part selector was moved into the ledger too: all custom property predicates
now read only the ledger, including w29. Bounds, verdicts and reach expectations are unchanged.

### MC7a bound refinement registered before its first run

The inspection-history refinement completed Correct/Control/Base at 57,533 unique states,
depth 21, 0.854 s, with all safety holding and expected reaches. The next LossCaller run
hit the 150,000-state ceiling and was rejected: **no verdict is claimed for it**. All 22
separate refutation runs completed before that rejection. No safety property failed on the
correct model, and the expected table is unchanged.

Before rerunning, each inspection observation is stored in its own immutable ledger slot,
rather than interleaved in the ordered history of stores, reports and request decisions.
There is exactly one inspection per destination and duplication is disabled. Their acceptance
snapshots remain independent, observable facts, but their ordering relative to unrelated
contention results is irrelevant to every property. No node state, message, fault, application
choice or timing window is changed. This representation lets independent observations commute.
Two additional reach properties, registered before this refined run, explicitly require the
caller inspection to overtake its control in ReorderCaller, and the implementation inspection
to overtake its dispatch in ReorderImplementation. Each is present only in its named part,
and is expected reached. All other safety and reach entries remain as registered above.

### MC7a return-link configurations registered before their first run

To exercise caller result transport as well as request transport, add two Correct/Control
parts, **LossReceipt** and **ReorderReceipt**. Each enables the same one engine timer and one
loss or reorder budget respectively. The original LossCaller/ReorderCaller now explicitly
restrict their fault to caller-to-engine request links; the new parts restrict it to
engine-to-caller result links. This splits directions rather than treating an unused return
link as fault coverage. The original seven Control rows and their expected verdicts are
retained; these two rows extend that table.

In these two parts the engine sends A its persisted primary result, and sends B each of the
seven immediate contention answers. B sends its requests without waiting for those answers,
so several answers can be queued on engine-to-B and genuinely reordered. Callers' received
results occupy eight separate immutable ledger slots. Losing A's successful receipt does not
change the engine's saved success. The bounded completion barrier also drains B's response
batch, and queued result messages drain before the serialized suffix. Every emitted result
is still returned at the engine's decision, without waiting for the competing invocation.
Other parts retain atomic delivery of caller results to isolate their own faults.

| Additional configuration | Expected refuted safety | Expected reach |
| --- | --- | --- |
| Correct/Control/LossReceipt | None | Control's w set, every enabled fault, primary result received |
| Correct/Control/ReorderReceipt | None | Control's w set, every enabled fault, primary result received |

The last witness requires some path to deliver A's result; it does not demand delivery on a
path that loses it. w29 still requires reconciliation after the configured return-link fault.
Queue bound seven and all state/time limits remain unchanged. There are now **38 full runs**:
16 correct configurations and 22 variants. This extension is registered after the earlier
36-run refined matrix passed, before either new configuration is run.

LossReceipt additionally requires a lost **successful** primary receipt: the ledger has its
succeeded end, no A receipt, and all three suffix stores. Those stores can happen only after
all queues drained, so this cannot be an undelivered result still in flight. The engine's
saved success remains unchanged. This reach property is expected reached only in LossReceipt,
where it is registered; it is registered before that configuration's first run.


### MC7a physical partial-effect refinement, registered before its first run

Final review found a defect in the model data, rather than a language counterexample: the
initial two-setting control sent `(1,2)` against stored `(0,2)`. Applying only x changed one
setting but left the physical values equal to those of full success, because y was already 2.
Before the first run with the corrected data, change the two-setting control to `(1,1)`.
Both supplied values now differ from the initial `(0,2)`; partial application produces
`(1,2)`, physically distinct from full `(1,1)`. Both full configurations satisfy `x <= y`.
Schema still sends only x=1 and reaches the same invalid `(1,0)` after the later change.
No expected safety or reach entry changes. The earlier 42-test runs passed, but their final
matrix measurements are superseded by the reruns recorded below. This repairs the model's
own partial-effect witness; no safety counterexample was removed.



### MC7a dated results — 2026-11-11

Measured on Linux x86_64, AMD Ryzen 9 5900X, Rust `1.97.1`
(`8bab26f4f`, 2026-07-14), Stateright `0.31.0`. `CARGO_BUILD_JOBS` was left as the
environment set it. Full runs used the harness default two checker threads; first
refutations used one-thread BFS. The final ordinary release command passed **42 unit
tests, 0 failed**, and **0 doc tests**, in **50.32 seconds wall time**, including compilation
(test execution 42.19 s). The counts command, `cargo test --release -- --nocapture
--test-threads=1`, also passed **42 unit tests and 0 doc tests**, in **41.59 seconds wall
time** (test execution 41.33 s). The three MC7a tests are included in those totals.

All **38 final MC7a full runs** completed exhaustively below their bounds. Safety column:
“all H” means every entry in A F E U R V T O C N M K P W I G B L holds; otherwise the
listed entries are F and every other entry is H. Reach column lists observed w indices;
every other w index was unreached, as registered or explicitly recorded for MC7a-001.
All enabled harness fault witnesses were reached. The two inspection-overtaking witnesses,
both return-link primary-receipt witnesses, and the lost-success-receipt witness were all
reached in the configurations that register them.

| Variant/scene/part | Unique states | Depth | Seconds | Observed safety | Observed w reach |
| --- | --- | --- | --- | --- | --- |
| Correct/Control/Base | 843 | 21 | 0.008 | all H | 0–8, 10, 15–18, 29 |
| Correct/Control/LossCaller | 7,865 | 21 | 0.091 | all H | 0–8, 10, 15–18, 29 |
| Correct/Control/ReorderCaller | 19,008 | 21 | 0.210 | all H | 0–8, 10, 15–18, 29 |
| Correct/Control/LossReceipt | 9,271 | 29 | 0.108 | all H | 0–8, 10, 15–18, 29 |
| Correct/Control/ReorderReceipt | 2,535 | 29 | 0.029 | all H | 0–8, 10, 15–18, 29 |
| Correct/Control/LossImplementation | 2,324 | 21 | 0.024 | all H | 0–8, 10, 15–18, 29 |
| Correct/Control/ReorderImplementation | 1,678 | 21 | 0.017 | all H | 0–8, 10, 15–18, 29 |
| Correct/Control/Crash | 1,622 | 23 | 0.016 | all H | 0–8, 10, 15–18, 29 |
| Correct/Control/Silence | 982 | 21 | 0.010 | all H | 0–8, 10, 15–18, 29 |
| Correct/Schema/Base | 189 | 21 | 0.002 | all H | 0–4, 6, 9–10, 15–17, 29 |
| Correct/Retired/Base | 1,168 | 21 | 0.011 | all H | 0–8, 10–13, 15–18, 29 |
| Correct/Removed/Base | 726 | 20 | 0.007 | all H | 0–6, 10–12, 14–18, 29 |
| Correct/Outcomes/Base | 202 | 17 | 0.002 | all H | 10, 15–19 |
| Correct/Cancellation/Base | 404 | 19 | 0.004 | all H | 10, 15–18, 20–22 |
| Correct/Repeats/Base | 238 | 23 | 0.003 | all H | 10, 15–18, 23–25 |
| Correct/ModelWork/Base | 679 | 44 | 0.006 | all H | 10, 15–18, 26–28 |
| StaleRevision/Control/Base | 843 | 21 | 0.007 | A F; rest H | 0–8, 10, 15–18, 29 |
| ReusedRevision/Control/Base | 843 | 21 | 0.008 | F F; rest H | 0–8, 10, 15–18, 29 |
| Substitute/Control/Base | 843 | 21 | 0.007 | E, C F; rest H | 0–8, 10, 15–18, 29 |
| StoreUnknown/Control/Base | 843 | 21 | 0.008 | U F; rest H | 0–8, 10, 15–18, 29 |
| WrongReconcile/Control/Base | 843 | 21 | 0.009 | R F; rest H | 0–8, 10, 15–18, 29 |
| AcceptInvalid/Schema/Base | 189 | 21 | 0.002 | V F; rest H | 0–7, 10, 15–17, 29 |
| QueueContender/Control/Base | 843 | 21 | 0.008 | T F; rest H | 0–8, 10, 15–18, 29 |
| DoubleOutcome/Outcomes/Base | 202 | 17 | 0.002 | O, M F; rest H | 10, 15–19 |
| FalseSuccess/Outcomes/Base | 202 | 17 | 0.002 | C F; rest H | 10, 15, 17–19 |
| PartialFailed/Outcomes/Base | 202 | 17 | 0.002 | C F; rest H | 10, 15–19 |
| UncertainFailed/Outcomes/Base | 202 | 17 | 0.002 | C F; rest H | 10, 15–16, 18–19 |
| OutcomeForReason/Outcomes/Base | 202 | 17 | 0.002 | T, N F; rest H | 10, 15–19 |
| TrustMalformed/Outcomes/Base | 202 | 17 | 0.002 | M F; rest H | 10, 15–19 |
| DropCancellation/Cancellation/Base | 287 | 18 | 0.003 | K F; rest H | 10, 15–18, 20–22 |
| CancelFailed/Cancellation/Base | 341 | 19 | 0.004 | K F; rest H | 10, 15–18, 20–22 |
| LeakOutputs/Repeats/Base | 238 | 23 | 0.003 | P F; rest H | 10, 15–18, 23–25 |
| RepeatNoOutputsConflict/Repeats/Base | 238 | 23 | 0.003 | P F; rest H | 10, 15–18, 23–25 |
| ImplicitWork/ModelWork/Base | 679 | 44 | 0.007 | W F; rest H | 10, 15–18, 26–28 |
| ClosedWork/ModelWork/Base | 679 | 44 | 0.007 | W F; rest H | 10, 15–18, 26–28 |
| AllowOnward/ModelWork/Base | 700 | 45 | 0.007 | I F; rest H | 10, 15–18, 26–28 |
| Unfenced/Control/Base | 779 | 21 | 0.008 | G, B F; rest H | 0–8, 10, 15–18, 29 |
| LateConfirmation/Cancellation/Base | 404 | 19 | 0.004 | L F; rest H | 10, 15–18, 20–22 |

Largest final run: **19,008 unique states**. Sum of the full-run checker times above: **0.655 s**; this is not the whole crate's wall time.

**Every difference from registration:** AcceptInvalid/Schema/Base reaches w5 and w7 in
addition to the registered set (MC7a-001). There is no other safety, reach or enabled-fault
difference. The rejected intermediate inspection-history run described above has no verdict;
its bound rejection is not a safety finding. The representation refinement preserves all
registered safety behavior, and the two new return-link parts were registered before running.

**MC7a-002 — pending fence gap, reproduced from MC2-001.** Unfenced follows current
OPS-014 rather than the order's pending ruling 75 A1 reading. It refutes G and B: the actor
records absence before the attempt is fenced, then the implementation applies the original
control. In the first 15-action trace below, actions 1–11 dispatch and drain the contention
and inspection batch; 12 records unknown, 13 sends absent reconciliation, 14 records failed,
and 15 applies the full effect. No engine state was lost and no implementation reported
failed. This is not a counterexample to OPS-003, which binds implementation reports, nor a
new claim about repeated effects: MC2's original trace supplies the repeat. It demonstrates
why the already accepted fence still needs writing in OPS-014, outside this order's scope.
The Correct control, registered first, keeps the pending reading and holds G/B in every part.

### MC7a first single-thread BFS refutations

Each trace below is from its own final one-thread refutation run, distinct from the full
matrix run. The test replays each trace, erases every node's data and clears the queues,
and checks that its safety failure remains in the external ledger. All 22 variants refute
their target; no variant is accepted merely for traversing a mutation branch.

**StaleRevision**, 13 actions; target A.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. implementation Finish(Full, Succeeded)
13. delivered implementation -> engine [position 0]
```

**ReusedRevision**, 16 actions; target F.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. implementation Finish(Full, Succeeded)
13. delivered implementation -> engine [position 0]
14. engine Suffix
15. engine Suffix
16. engine Suffix
```

**Substitute**, 13 actions; target E.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. implementation Finish(Full, Succeeded)
13. delivered implementation -> engine [position 0]
```

**StoreUnknown**, 12 actions; target U.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
```

**WrongReconcile**, 15 actions; target R.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
13. implementation Finish(Full, Succeeded)
14. engine Reconcile(true, 0)
15. delivered B -> engine [position 0]
```

**AcceptInvalid**, 17 actions; target V.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
13. implementation Finish(Full, Succeeded)
14. delivered implementation -> engine [position 0]
15. engine LaterChange
16. engine Reconcile(true, 0)
17. delivered B -> engine [position 0]
```

**QueueContender**, 2 actions; target T.

```text
1. delivered A -> engine [position 0]
2. delivered B -> engine [position 0]
```

**DoubleOutcome**, 12 actions; target O.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
```

**FalseSuccess**, 12 actions; target C.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. implementation Finish(None, Failed)
```

**PartialFailed**, 12 actions; target C.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. implementation Finish(Partial, Unknown)
```

**UncertainFailed**, 12 actions; target C.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. implementation Finish(None, Unknown)
```

**OutcomeForReason**, 5 actions; target N.

```text
1. delivered A -> engine [position 0]
2. delivered B -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
```

**TrustMalformed**, 13 actions; target M.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. implementation Finish(Full, MissingTag)
13. delivered implementation -> engine [position 0]
```

**DropCancellation**, 12 actions; target K.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. engine CloseStream
```

**CancelFailed**, 13 actions; target K.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. engine CloseStream
13. engine RecordCancel
```

**LeakOutputs**, 15 actions; target P.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
13. engine StopRun
14. engine Script
15. delivered A -> engine [position 0]
```

**RepeatNoOutputsConflict**, 17 actions; target P.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
13. engine StopRun
14. engine Script
15. delivered A -> engine [position 0]
16. engine Script
17. delivered A -> engine [position 0]
```

**ImplicitWork**, 14 actions; target W.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
13. engine StopRun
14. engine Script
```

**ClosedWork**, 38 actions; target W.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
13. engine StopRun
14. engine Script
15. engine Script
16. engine Script
17. engine Script
18. engine Script
19. engine Script
20. engine Script
21. engine Script
22. engine Script
23. engine Script
24. engine Script
25. engine Script
26. engine Script
27. engine Script
28. engine Script
29. delivered A -> engine [position 0]
30. delivered engine -> implementation [position 0]
31. delivered implementation -> engine [position 0]
32. engine Script
33. delivered A -> engine [position 0]
34. delivered engine -> implementation [position 0]
35. delivered implementation -> engine [position 0]
36. delivered engine -> implementation [position 0]
37. engine Script
38. delivered A -> engine [position 0]
```

**AllowOnward**, 31 actions; target I.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
13. engine StopRun
14. engine Script
15. engine Script
16. engine Script
17. engine Script
18. engine Script
19. engine Script
20. engine Script
21. engine Script
22. engine Script
23. engine Script
24. engine Script
25. engine Script
26. engine Script
27. engine Script
28. engine Script
29. delivered A -> engine [position 0]
30. delivered engine -> implementation [position 0]
31. delivered implementation -> engine [position 0]
```

**Unfenced**, 15 actions; target B.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
13. engine Reconcile(false, 0)
14. delivered B -> engine [position 0]
15. implementation Finish(Full, Succeeded)
```

**LateConfirmation**, 14 actions; target L.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
13. implementation Finish(None, Failed)
14. delivered implementation -> engine [position 0]
```

### What MC7a establishes and leaves to S5

The 18 modelled catalogue projections hold on every correct configuration, including loss,
reordering, crash/recovery, silence, retirement and the schema-invalid reconciliation. Every
one of the thirty common precondition witnesses is reached in at least one correct run. The
22 variants and the pending-text control all have counterexamples. Thirteen static decisions
remain for vectors; supporting promises and the pending fence are outside the 31-count ledger.
No conformance vector, specification, engine file or ruling is changed.

The bounded contention batch precedes completion; later changes are serialized, and one
attempt cannot prove the multi-attempt fence. Multiple non-timer fault classes do not occur
together. Explicit silence here applies no effect; an effect with a missing response is also
covered by timer/reply races and implementation-link loss. There is no partition, plain-server
adapter, remote API continuation, provider gate, parser, actual model provider or real caller.
The implicit-event projection does not prove the independent lifecycle algorithms. The
succeeded repeat fixtures fix identity/content matching rather than rechecking MC2. S5 must
still supply real implementation verification, author measures, static discovery/default/schema
vectors and user-visible lost-receipt recovery. These bounded results claim neither liveness,
unbounded safety nor correctness of any build.

### MC7a admin review: additions registered before execution, 2026-11-11

The three review findings identify defects in the model's checks and coverage. They do not
change a promise or its quoted preconditions above. This registration precedes every run of
the revised model. The original expected tables and dated results remain historical evidence.

**Outcome precondition (must).** Each implementation reply is logged as `Received`, with
whether the invocation was still in progress, before any outcome conversion. `End` additionally
records the actual received reply, or none for a timer, recovery or cancellation record. O's
OPS-001 exemption now depends on that reply being an implementation reason, rather than on
the engine's chosen answer being a reason. N also checks the implementation-reason branch of
OPS-005. New `ReasonForOutcome/Outcomes/Base` converts an ordinary succeeded reply to conflict;
it must refute O while leaving every other safety entry holding.

**Cancellation confirmation (must).** `CancelEnd` carries only the chosen outcome. K scans the
preceding ledger events for a received `Reply::Failed` while the invocation was in progress;
neither a chosen failed outcome nor a failed reply received after the end supplies confirmation.
w21 likewise requires an independently received failed reply before the cancelled end. New
`CancelUnknownFailed/Cancellation/Base` converts a cancelled invocation's received unknown reply
to failed; it must refute K while leaving every other safety entry holding. The implementation
report remains unknown, so C does not mistake this engine defect for an implementation claim.

**Current configuration (should).** New `CurrentMerge/Base` uses the same four nodes, links,
fault gates and bounds as Control/Base. Its control supplies only x=1 against INITIAL `(0,2)`;
partial physical application is excluded because this control addresses one setting. After
unknown, the script requires an intervening stored change to `(0,1)` before reconciliation.
Applying x=1 to that current configuration yields valid `(1,1)`, preserving the untouched y=1;
applying it to INITIAL yields `(1,2)`. Current/stale/missing revisions and absence remain script
choices. New `OriginalReconcile/CurrentMerge/Base` merges into INITIAL, rather than into the
current configuration. R must refute it. This complements Schema's invalid merge and retains
the original `WrongReconcile` variant and its expected row.

All three additions use Base (timer budget one, other harness faults isolated in the existing
Control parts); the timer and existing unknown/reconciliation witnesses remain mandatory.
Each new variant has its own full exhaustive run and first one-thread BFS refutation. No
property consults node state, including after the ledger-independence test clears every node.

Three new ledger reach properties distinguish these preconditions without demanding the
correct result, so the broken variants must still reach them:

| Index | Required recorded precondition/event |
| --- | --- |
| 30 | After a later store, a live existing instance's occurred reconciliation names the current revision, addresses only part of its settings, and has a valid merge differing from the merge into INITIAL |
| 31 | An ordinary succeeded, failed or unknown reply is received while the invocation is in progress |
| 32 | An unknown reply is received while the invocation is in progress, after stream cancellation |

Expected table extension, fixed before this revision's first run: every original row retains
its safety and w0–w29 registration. w31 is reached in **every** row. w32 is reached exactly in
Cancellation rows. w30 is reached exactly in the two CurrentMerge rows. Every other new reach
entry is unreached. The previously recorded AcceptInvalid w5/w7 difference remains separate.

| Added variant/scene/part | Expected refuted safety (all others H) | Expected w0–w32 reaches (all others unreached) |
| --- | --- | --- |
| Correct/CurrentMerge/Base | None | 0–8, 10, 15–17, 29–31 |
| OriginalReconcile/CurrentMerge/Base | R | 0–8, 10, 15–17, 29–31 |
| ReasonForOutcome/Outcomes/Base | O | 10, 15–19, 31 |
| CancelUnknownFailed/Cancellation/Base | K | 10, 15–18, 20–22, 31–32 |

Accounting remains **18 modelled, 13 not modelled, all 13 static**. Supporting promises remain
outside the count. There are now 42 MC7a full runs, 25 broken variants and 33 common witnesses.

### MC7a admin review results — 2026-11-11

All three verifier findings above are fixed within the order’s scope. The final ordinary
release run passed **42 unit tests, 0 failed**, and **0 doc tests**, in **40.84 seconds wall
time** (test execution 37.49 s). The counts run, `cargo test --release -- --nocapture
--test-threads=1`, passed the same totals in **39.50 seconds wall time** (test execution
39.24 s). Both commands used the environment’s `CARGO_BUILD_JOBS`, unchanged. The targeted
MC7a check also passed its three tests. No timing-sensitive test failed or needed a rerun.

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, all five repository Python
checkers, the Python unittest suite (**41 tests**), and `git diff --check` passed. The two
required grep scans found no matches (exit 1). The Python interpreter was `/usr/bin/python3`,
with PyYAML and jsonschema available. Only this README and `src/controls.rs` changed;
nothing was staged, and build output remains ignored.

Every full run below completed exhaustively. In the safety column, the listed entries are
**refuted**, every other entry in **A F E U R V T O C N M K P W I G B L** holds, and “all H”
means all eighteen hold. The reach column lists every reached w index; all others are
unreached. Every enabled harness fault and every part-specific inspection/receipt witness
was reached. All 33 common witnesses are reached by at least one correct configuration.

| Variant/scene/part | Unique states | Depth | Seconds | Observed safety | Observed w reach |
| --- | --- | --- | --- | --- | --- |
| Correct/Control/Base | 843 | 21 | 0.008 | all H | 0–8, 10, 15–18, 29, 31 |
| Correct/Control/LossCaller | 7,865 | 21 | 0.091 | all H | 0–8, 10, 15–18, 29, 31 |
| Correct/Control/ReorderCaller | 19,008 | 21 | 0.227 | all H | 0–8, 10, 15–18, 29, 31 |
| Correct/Control/LossReceipt | 9,271 | 29 | 0.108 | all H | 0–8, 10, 15–18, 29, 31 |
| Correct/Control/ReorderReceipt | 2,535 | 29 | 0.030 | all H | 0–8, 10, 15–18, 29, 31 |
| Correct/Control/LossImplementation | 2,324 | 21 | 0.023 | all H | 0–8, 10, 15–18, 29, 31 |
| Correct/Control/ReorderImplementation | 1,678 | 21 | 0.016 | all H | 0–8, 10, 15–18, 29, 31 |
| Correct/Control/Crash | 1,622 | 23 | 0.015 | all H | 0–8, 10, 15–18, 29, 31 |
| Correct/Control/Silence | 982 | 21 | 0.010 | all H | 0–8, 10, 15–18, 29, 31 |
| Correct/Schema/Base | 189 | 21 | 0.002 | all H | 0–4, 6, 9–10, 15–17, 29, 31 |
| Correct/CurrentMerge/Base | 189 | 21 | 0.002 | all H | 0–8, 10, 15–17, 29–31 |
| Correct/Retired/Base | 1,168 | 21 | 0.011 | all H | 0–8, 10–13, 15–18, 29, 31 |
| Correct/Removed/Base | 726 | 20 | 0.007 | all H | 0–6, 10–12, 14–18, 29, 31 |
| Correct/Outcomes/Base | 202 | 17 | 0.002 | all H | 10, 15–19, 31 |
| Correct/Cancellation/Base | 404 | 19 | 0.004 | all H | 10, 15–18, 20–22, 31–32 |
| Correct/Repeats/Base | 238 | 23 | 0.002 | all H | 10, 15–18, 23–25, 31 |
| Correct/ModelWork/Base | 679 | 44 | 0.006 | all H | 10, 15–18, 26–28, 31 |
| StaleRevision/Control/Base | 843 | 21 | 0.007 | A refuted; rest H | 0–8, 10, 15–18, 29, 31 |
| ReusedRevision/Control/Base | 843 | 21 | 0.008 | F refuted; rest H | 0–8, 10, 15–18, 29, 31 |
| Substitute/Control/Base | 843 | 21 | 0.008 | E, C refuted; rest H | 0–8, 10, 15–18, 29, 31 |
| StoreUnknown/Control/Base | 843 | 21 | 0.008 | U refuted; rest H | 0–8, 10, 15–18, 29, 31 |
| WrongReconcile/Control/Base | 843 | 21 | 0.008 | R refuted; rest H | 0–8, 10, 15–18, 29, 31 |
| OriginalReconcile/CurrentMerge/Base | 189 | 21 | 0.002 | R refuted; rest H | 0–8, 10, 15–17, 29–31 |
| AcceptInvalid/Schema/Base | 189 | 21 | 0.002 | V refuted; rest H | 0–7, 10, 15–17, 29, 31 |
| QueueContender/Control/Base | 843 | 21 | 0.008 | T refuted; rest H | 0–8, 10, 15–18, 29, 31 |
| DoubleOutcome/Outcomes/Base | 202 | 17 | 0.002 | O, M refuted; rest H | 10, 15–19, 31 |
| ReasonForOutcome/Outcomes/Base | 202 | 17 | 0.002 | O refuted; rest H | 10, 15–19, 31 |
| FalseSuccess/Outcomes/Base | 202 | 17 | 0.002 | C refuted; rest H | 10, 15, 17–19, 31 |
| PartialFailed/Outcomes/Base | 202 | 17 | 0.002 | C refuted; rest H | 10, 15–19, 31 |
| UncertainFailed/Outcomes/Base | 202 | 17 | 0.002 | C refuted; rest H | 10, 15–16, 18–19, 31 |
| OutcomeForReason/Outcomes/Base | 202 | 17 | 0.002 | T, N refuted; rest H | 10, 15–19, 31 |
| TrustMalformed/Outcomes/Base | 202 | 17 | 0.002 | M refuted; rest H | 10, 15–19, 31 |
| DropCancellation/Cancellation/Base | 287 | 18 | 0.003 | K refuted; rest H | 10, 15–18, 20–22, 31–32 |
| CancelFailed/Cancellation/Base | 341 | 19 | 0.003 | K refuted; rest H | 10, 15–18, 20–22, 31–32 |
| CancelUnknownFailed/Cancellation/Base | 404 | 19 | 0.005 | K refuted; rest H | 10, 15–18, 20–22, 31–32 |
| LeakOutputs/Repeats/Base | 238 | 23 | 0.003 | P refuted; rest H | 10, 15–18, 23–25, 31 |
| RepeatNoOutputsConflict/Repeats/Base | 238 | 23 | 0.003 | P refuted; rest H | 10, 15–18, 23–25, 31 |
| ImplicitWork/ModelWork/Base | 679 | 44 | 0.007 | W refuted; rest H | 10, 15–18, 26–28, 31 |
| ClosedWork/ModelWork/Base | 679 | 44 | 0.007 | W refuted; rest H | 10, 15–18, 26–28, 31 |
| AllowOnward/ModelWork/Base | 700 | 45 | 0.008 | I refuted; rest H | 10, 15–18, 26–28, 31 |
| Unfenced/Control/Base | 779 | 21 | 0.008 | G, B refuted; rest H | 0–8, 10, 15–18, 29, 31 |
| LateConfirmation/Cancellation/Base | 404 | 19 | 0.004 | L refuted; rest H | 10, 15–18, 20–22, 31–32 |

Largest run: **19,008 unique states**. Sum of MC7a full-run checker
times: **0.680 s**. These times exclude compilation and the other models.


**Every difference from the registered tables:** the retained MC7a-001 difference is still
AcceptInvalid/Schema/Base reaching w5 and w7 in addition to its expected set. No new safety,
reachability or fault-witness difference occurred. The original table was not rewritten.
The three new mutants refute exactly their registered target, while their implementation
reports remain valid. The pending-text Unfenced control still refutes G/B (MC7a-002); the
specification correction belongs to order 077, outside this order’s scope.

The catalogue count remains **18 modelled / 13 not modelled (all 13 static decisions for
vectors)**, with supporting promises excluded. These remain bounded reference-model results,
with the earlier limits and S5 obligations unchanged.

#### New first one-thread BFS refutations

Each trace below was printed by its own target-only refutation run. Its first eleven actions
dispatch the primary invocation and drain the contention/probe batch. The existing test also
replays every trace, clears every node and link, and confirms the target remains refuted using
only the external ledger.

**ReasonForOutcome**, 13 actions. O: the received reply is succeeded, but the engine records conflict. No implementation reason supplies the exemption.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. implementation Finish(Full, Succeeded)
13. delivered implementation -> engine [position 0]
```

**CancelUnknownFailed**, 14 actions. K: after stream closure the implementation reports unknown for a partial effect. The engine records failed without a received failed confirmation.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. engine CloseStream
13. implementation Finish(Partial, Unknown)
14. delivered implementation -> engine [position 0]
```

**OriginalReconcile**, 17 actions. R: after unknown, the late full-effect reply fences the attempt. A later store changes y to 1. Reconciliation stores `(1,2)` by merging x=1 into INITIAL, instead of preserving y=1 in `(1,1)`.

```text
1. delivered A -> engine [position 0]
2. delivered A -> engine [position 0]
3. delivered B -> engine [position 0]
4. delivered B -> engine [position 0]
5. delivered B -> engine [position 0]
6. delivered B -> engine [position 0]
7. delivered B -> engine [position 0]
8. delivered B -> engine [position 0]
9. delivered B -> engine [position 0]
10. delivered engine -> implementation [position 0]
11. delivered engine -> implementation [position 0]
12. timer engine
13. implementation Finish(Full, Succeeded)
14. delivered implementation -> engine [position 0]
15. engine LaterChange
16. engine Reconcile(true, 0)
17. delivered B -> engine [position 0]
```

## MC6 — setup, retirement, removal and Pools: pre-registration

Registered 2026-11-11 for work order 111, before the first model run. The reference
model is [src/lifecycle.rs](src/lifecycle.rs); it imports no engine code. The expected
table is immutable. Observations differing from it will be recorded separately.

### Nodes, storage, scripts and bounds

R sends actor requests to E (the engine); H implements runs, hooks, stops and one
ordinary operation. E saves instance IDs, lifecycle, shutdown, invocation/part outcomes,
hook keys and attempts, tombstones and the resolved definitions. Crash discards only a
volatile cache. Recovery restores saved work without resending any dispatch; an already
sent reply may finish that work. H saves its effects and run state. The external ledger
records dispatches, results and state observations at their send/decision boundaries;
no property consults node data, queues, fault counters or live engine state. The test
replays every first refutation and erases every node and queue before rechecking it.

IDs are I, T, c, c-prime, d, rT, P, e1, e2, e2-prime, J and K. Closed and removed IDs
remain distinct from replacements. c and c-prime occupy one declared slot of I and
target T; d is another Conduit consumed by I, and rT is R's independent route to T.
T starts registered: the first explicit open of c happens before T is set up. d and
rT start open. e2 refers to e1, fixing setup order e1/e2 and retirement order e2/e1.
All hooks are non-deduplicating, effectful hooks selected from the instance's resolved
definition. Every modelled setup part is mandatory; account hooks, remote connections,
secrets, filters, cycles, Fleets, machines and cancellation are absent.

Each script stage waits for the preceding stage's actor answers, never for unrelated
late messages. Two requests in brackets are sent without waiting for either answer:

- Setup: [set up I, set up J]; set up I as another caller; set up I again; reconcile
  I's setup hook; reconcile an unknown setup of I; set up I again; set up J; replace
  and unregister the template; observe I. I's first hook fails, its second is unknown,
  and the reconciled hook is not repeated. J's first run fails without an effect.
- Removal: open c while T is registered; set up I (hook unknown); reconcile its hook;
  set up I again (ready); set up J (hook unknown); reconcile J's setup (ready);
  ask for c's replacement while c exists; explicitly close c while I is ready; set
  up I against the closed slot; replacement requests with denied access, invalid
  inputs and matching tombstone; lift c's tombstone; replace the template; explicitly
  register c-prime; ask for a second replacement. Timer/Crash additionally ask to open
  c-prime here. Retire I with a failed hook and successful stop (unknown); ask to open
  c-prime and set up I; reconcile I's old unknown setup and any unknown open; make an
  ordinary request; unregister I (another stop, then remove); ask to set up removed I;
  set up T; [ordinary request to T, retire T]; request T again; retire J twice;
  reconcile the earlier retirement as not occurred, then the latest as occurred;
  retire never-set-up K; unregister the template; observe T. J's second stop returns
  unknown. rT lets the ordinary request be in flight when T's retirement is dispatched,
  even though I is already removed. I's c-prime is registered at its retirement in
  Base; its opening then conflicts while I exists. Unregistration closes d and c-prime.
- Pool: [set up e2 directly, set up J]; set up P twice; ask for a Resource while e1
  is live; plainly retire P while members are unconfirmed; retire with Resources;
  reconcile e2's unknown retirement; register e2-prime, a fresh name matching no old
  Resource tombstone; retire with Resources again; reconcile e1's retirement; retire
  with Resources a third time; ask for a Resource while P is retired but exists;
  retire P again. e2's first direct run fails, without setting up e1. The first cascade
  waits on e2's unknown hook, the second on e1's failed hook plus successful stop, and
  the third reaches P. Dispatch of the cascade itself is never P's retirement.

A setup opens its declaration, asks for the run, then dispatches the setup hook.
Already-open slots succeed without another open; closed slots conflict and instantiate
nothing. A retirement takes effect at dispatch and, when any setup was ever dispatched,
asks for the hook's run, dispatches an eligible hook, then asks for the stop whatever
came of the hook. An unknown hook is retained; a failed hook alone gets a new attempt,
with the same instance/operation key and the current caller. A setup that opened c and
failed later is unknown; a further setup retaining an unknown hook is unknown even
when that hook dispatches nothing. Setup failure causes no rollback.

The outcome function requires all parts to succeed for success; any unknown or any
partial effect yields unknown; failed parts with no effect yield failed. Shutdown is
pending during retirement, unknown after an unknown retirement, confirmed after success
or positive reconciliation, and confirmed immediately for no setup ever dispatched.
A negative reconciliation is permitted only after all relevant dispatches and H jobs
are fenced; reconciling an older retirement leaves the latest shutdown classification.
Unknown setup/open reconciliation never revives a retired instance or consumer.
An explicit open has separate stored-effect and confirmation steps, allowing Timer to
end it unknown before retirement and reconciliation. No eventual completion is claimed:
a quiescent-state witness means all queued work has drained and all invocations ended.

Eight configurations per part split faults: Base has none; LossR/ReorderR allow one
loss/reorder in either R/E direction; LossH/ReorderH allow one in either E/H direction;
Crash allows one E crash/recovery; Timer allows one E timer; Silence lets H omit one
hook reply and allows one E timer. H's omission is a bounded model action, not a harness
fault. ReorderR has the two bracketed actor requests. ReorderH has their two independent
implementation requests (runs for Setup/Pool; call and retire-hook run for Removal).
Timers use `timer_enabled`, not `fault_allowed`. There is no implicit retry of a lost
actor request or answer. A lossy branch can stop; safety still applies to its prefix.
Every enabled harness fault must have a witness in its own full run.

Zero budgets: duplication is MC2's dispatch identity question; cuts/heals and permanently
lost machines belong to MC4's placement model; each remaining fault is enabled only in
its named configuration to avoid multiplying independent histories. No simultaneous
fault kinds beyond crash/recovery and silence/timer are claimed. The queue bound is six
per directed link, with at most two actor requests together and at most three late
implementation messages; overflow fails. There are twelve IDs, two entry slots, fewer
than 64 root requests and 32 child invocations, one silence and one of each enabled
fault. Each run has a 190,000 unique-state ceiling and a 10-second deadline, with a
half-second target; the intended addition to crate test time is about 20 seconds.

### Properties and catalogue accounting

L checks monotone lifecycle, ready after successful setup, unchanged after unsuccessful
setup, retirement at dispatch, observation without lifecycle change, and conflict on
retired setup versus invalid after removal. S checks setup part order and the initiating
actor on declared opens. H checks hook dispatch eligibility; K checks the stable key,
current caller, selected implementation and absence of a Conduit. O checks aggregate
outcomes of setup and retirement against their dispatched/retained parts. B checks no
rollback and that explicit closure leaves a ready consumer ready. C checks closure of
all relevant ends, retired access and opening restrictions. R checks explicit new IDs,
registered replacements, live-slot conflicts and retired-owner invalid. T checks
creation of tombstones and access/invalid precedence before a matching conflict.
D checks shutdown classification, removal only after confirmation or unregistration,
the additional stop on unregistration, and no retirement work without a dispatched
setup. P checks dependency order, ready-member skips and the Pool retirement gate.
F checks retained resolved definitions, records, backing data and unrelated objects.
Q checks reconciliation before and after retirement, including latest versus older
retirements. A checks negative fencing and at most one hook effect under these scripts.

Two additional literal predicates are deliberately retained: V (the contract's failed
retire hook keeps shutdown pending) and N (LIF-019 forbids changing any referrer on
removal). They conflict with the order's chosen readings. HookPending follows V and
refutes D; PreserveReferences follows N and refutes C. Unfenced follows present OPS-014's
unfenced negative reconciliation and is expected to refute A, as MC2-001 did. These are
controls, not silently adopted language changes. Supporting OPS-012, OPS-014, OPS-024,
OPS-029, IMP-007, ACC-012 and ACC-053, and the open/stop/retire-hook contracts are named
outside the 58-promise count.

Each row below occurs once in the accounting list. “Modelled” means the stated bounded
slice, not all behavior of a promise. Each quoted when/then is complete, including its
preconditions; `must not` is stated where the YAML level supplies the negation.

| Promise | Accounting | Predicate / reason |
| --- | --- | --- |
| LIF-001 | Modelled | L |
| LIF-003 | Modelled | L |
| LIF-004 | Modelled | L |
| LIF-005 | Modelled | L |
| LIF-006 | Modelled | L |
| LIF-008 | Modelled | R |
| LIF-009 | Modelled | R |
| LIF-010 | Modelled | L |
| LIF-011 | Not modelled | A static decision, to be shown by vectors. |
| LIF-012 | Not modelled | A static decision, to be shown by vectors. |
| LIF-013 | Not modelled | A static decision, to be shown by vectors. |
| LIF-014 | Modelled | D |
| LIF-015 | Modelled | L |
| LIF-016 | Modelled | F |
| LIF-017 | Modelled | D |
| LIF-018 | Modelled | D |
| LIF-019 | Modelled | F; N retains the literal referrer prohibition |
| LIF-020 | Modelled | P |
| LIF-021 | Modelled | P |
| LIF-022 | Not modelled | A static decision, to be shown by vectors. |
| LIF-023 | Modelled | L |
| LIF-024 | Modelled | D |
| LIF-025 | Modelled | D |
| LIF-026 | Modelled | H |
| LIF-027 | Modelled | B |
| LIF-028 | Modelled | Q |
| LIF-029 | Modelled | R |
| LIF-034 | Not modelled | A static decision, to be shown by vectors. |
| LIF-035 | Modelled | D |
| LIF-036 | Modelled | D |
| LIF-037 | Not modelled | A static decision, to be shown by vectors. |
| LIF-040 | Modelled | T |
| LIF-041 | Modelled | T |
| OPS-021 | Modelled | H |
| OPS-022 | Modelled | O |
| OPS-028 | Modelled | K |
| POOL-003 | Modelled | R |
| POOL-007 | Modelled | P |
| POOL-013 | Modelled | P |
| POOL-015 | Not modelled | A static decision, to be shown by vectors. |
| POOL-017 | Modelled | R |
| TPL-009 | Modelled | O |
| TPL-012 | Modelled | S |
| TPL-017 | Modelled | H |
| TPL-018 | Modelled | H |
| INS-020 | Modelled | F |
| SUB-009 | Not modelled | A static decision, to be shown by vectors. |
| FST-002 | Not modelled | A static decision, to be shown by vectors. |
| FST-003 | Not modelled | A static decision, to be shown by vectors. |
| FST-007 | Not modelled | A static decision, to be shown by vectors. |
| ACC-005 | Modelled | S |
| ACC-018 | Modelled | K |
| ACC-039 | Modelled | C |
| CON-004 | Modelled | S |
| CON-008 | Modelled | C |
| CON-014 | Modelled | B |
| CON-023 | Modelled | C |
| CON-031 | Modelled | C |

Counts: **47 modelled; 11 not modelled, all static decisions to be shown by vectors**.
FST-002/FST-003 additionally have S1's `bootstrap` trace. Partial slices outside this
model: TPL-017/ACC-018 filters; TPL-018's run reported unknown; LIF-021/TPL-012
cancellation (OPS-027); LIF-040 inventory visibility and full tombstone field encoding;
LIF-016 type-specific held resources; LIF-021 cycles/Fleets; optional setup parts,
account hooks, remote items and secrets. F checks a fixed unrelated object and symbolic
retained data; it does not execute a filesystem. P's direct-member check covers the
no-implicit-other-entry slice, not Pool authorization. Closed Conduits are the explicit
exception in F; N preserves the conflicting literal clause separately.

### Exact promise predicates

**LIF-001 (L; must).** When: “An instance of any family is inspected.” Then: “It reports exactly one lifecycle state: registered, ready or retired.”

**LIF-003 (L; must).** When: “An instance's setup succeeds: the invocation of `set-up.v1` that names it or, for a Conduit, of `open.v1`.” Then: “Its state is ready.”

**LIF-004 (L; must).** When: “An instance's setup fails, or has an unknown outcome, or is not dispatched.” Then: “Its state is unchanged, and its status reports how that setup ended.”

**LIF-005 (L; must).** When: “A request to retire an instance is dispatched.” Then: “Its state is retired from that moment, whatever the outcome of the invocation of `retire.v1` or, for a Conduit, of `close.v1`.”

**LIF-006 (L; must not).** When: “An instance is ready or retired.” Then: “Any operation returns it to an earlier state.”

**LIF-008 (R; must not).** When: “An instance made by a nested instantiation, such as a declared Conduit or the Resource of a Pool's entry, is retired.” Then: “Setup, anything else that happens to the instance it belongs to, or anything other than an explicit request under LIF-009, makes another instance in its place.”

**LIF-009 (R; must).** When: “An actor explicitly asks for a new instance for a nested instantiation whose instance is retired or has been removed.” Then: “A new instance is instantiated from the nested instantiation as the instance it belongs to holds it in its resolved definition, with a new ID, in the state registered. It is made from the template that the `from` of the nested instantiation names, as that template stands when the request is made. The retired instance is not changed. The request is a registration: it is authorized under ACC-033 as registering that instance would be, and it sets nothing up and opens nothing. It is the operation `register-replacement.v1`, which names, in `instance`, the instance that holds the nested instantiation and, in `nested`, the name of the declared Conduit or the entry. Where a rule on instantiation does not make the new instance, as where that template no longer exists, the request is not dispatched, and reports invalid, as the registration of a document that the same rule rejects does under OBJ-023.”

**LIF-010 (L; must).** When: “What an instance is doing cannot be observed, for example because the machine provider reports its machine unreachable, under PLC-023.” Then: “Its lifecycle state is unchanged.”

**LIF-014 (D; must).** When: “An instance is retired.” Then: “Its status reports its shutdown as exactly one of confirmed, pending or unknown.”

**LIF-015 (L; must).** When: “A retired instance is addressed by a request to set it up or to change it.” Then: “The request is not dispatched, and reports conflict.”

**LIF-016 (F; must not).** When: “An instance is retired.” Then: “Anything is stopped or released other than what the instance itself holds, as its type contract defines, and its own run, which its retirement stops under PRV-008.”

**LIF-017 (D; must).** When: “An actor asks to unregister an instance.” Then: “The request is dispatched only where the instance is retired, and it removes the instance. Otherwise it is not dispatched, and reports conflict.”

**LIF-018 (D; must).** When: “The shutdown of a retired instance can never be confirmed by its implementation.” Then: “It stays unknown until an actor reconciles, under OPS-014, the invocation of `retire.v1` whose outcome is unknown, or a further retirement under LIF-026 succeeds. The reconciliation is authorized against the target that authorized the retirement, as ACC-033 states.”

**LIF-019 (F; must not).** When: “An object is unregistered or removed.” Then: “Records, backing files or service data are erased, or any object that refers to it is changed, retired or unregistered.”

**LIF-020 (P; must).** When: “An actor asks to retire a Pool owning a Resource, or a Fleet owning an Agent, whose shutdown is not confirmed.” Then: “The request is not dispatched and reports conflict.”

**LIF-021 (P; must).** When: “An actor explicitly asks to retire a Pool with its Resources or a Fleet with its Agents.” Then: “The members are retired in dependency order, and the container's retirement is dispatched only once every member's shutdown is confirmed. The order puts a member before each other member of that container it refers to or is the consumer of a Conduit to, and, for a Resource, before any member it names in `runtime`, transitively. Among cycles the order is the engine's (DL1). A member's retirement is dispatched only once the shutdown of every member the order puts before it is confirmed. The request dispatches a retirement for each member whose shutdown is not confirmed: a further retirement under LIF-026 if already retired, and none where shutdown is confirmed. A retirement that does not confirm shutdown, or a reason in place of dispatch, holds back only the members ordered after it; every other retirement proceeds.”

**LIF-023 (L; must not).** When: “No actor has asked for the retirement of an instance, and the instance is none of these: a Conduit whose consumer or target is retired or declared lost, under CON-008; a node declared lost, or an instance placed on one, under PLC-017 or PLC-024; a Resource that a Pool owns, retired with the Pool under LIF-021; an Agent retired with its Fleet under LIF-021.” Then: “The instance is retired.”

**LIF-024 (D; must).** When: “The shutdown of a retired instance is reported.” Then: “A Conduit's shutdown is confirmed when it is closed, however it was closed: it holds nothing to shut down. The shutdown of any other instance is confirmed once an invocation of `retire.v1` for it has succeeded, and at once for an instance for which no setup was ever dispatched, under LIF-025. A shutdown that is not confirmed is unknown where the outcome of the latest invocation of `retire.v1` for the instance is unknown, and for an instance for which a setup was dispatched and whose node was declared lost, under PLC-017, or whose machine the machine provider reported gone, under PLC-024. It is pending otherwise.”

**LIF-025 (D; must).** When: “An instance for which no setup was ever dispatched is retired.” Then: “Its retire hook is not dispatched, no stop of a run is asked for, and its shutdown is confirmed at once.”

**LIF-026 (H; must).** When: “An actor asks to retire an instance that is retired and whose shutdown is not confirmed.” Then: “The request is decided like any other request to retire the instance. LIF-015 does not stop it.”

**LIF-027 (B; must not).** When: “An instance's setup does not succeed: it fails, has an unknown outcome or is not dispatched.” Then: “A Conduit that the setup opened is closed, an instance that it set up is retired, or a run that it asked for is stopped, because of it.”

**LIF-028 (Q; must).** When: “An actor reconciles, under OPS-014, an invocation of `set-up.v1`, `open.v1`, `retire.v1` or `close.v1` whose outcome is unknown.” Then: “Recording that a setup or an open occurred makes the instance ready or the Conduit open, unless the instance, or the Conduit's consumer, has been retired. Recording that a retirement or a close occurred confirms the shutdown. Recording that it did not occur gives the outcome failed, and changes nothing else.”

**LIF-029 (R; must).** When: “An actor asks under LIF-009 for a new Conduit for a declared Conduit while the Conduit made for it is not retired.” Then: “The request is not dispatched, and reports conflict.”

**LIF-035 (D; must).** When: “The shutdown of a retired instance is confirmed, under LIF-024.” Then: “The instance is removed: it no longer exists, and its records are kept as LIF-019 states. A Conduit is so removed when it is closed.”

**LIF-036 (D; must).** When: “A request to unregister an instance is dispatched, under LIF-017, and an implementation item that is run is selected for the instance.” Then: “Before it removes the instance, the engine asks the instance's runtime provider once more to stop its run, with `stop.v1` naming the instance's ID and giving, in `machine`, the machine of the instance's node as the node's status holds it, under PRV-027, as PRV-008 asks it in a retirement.”

**LIF-040 (T; must).** When: “A Conduit is closed, by request or under CON-008, or another instance is retired.” Then: “Unless the object was made by the engine, the engine makes a tombstone and keeps it as state apart from records until lifted under ACC-053. It retains the object's kind, name and ID; the time; the tombstone's own ID; the cause; `by`, who closed or retired it; its provenance; and its owning Pool or Fleet by kind, name and ID at closing or retirement, where it had one. A Conduit's tombstone also retains both ends by kind, name and ID, its local name, capability and operations. Optional `within` is absent in this version and means the engine.”

**LIF-041 (T; must).** When: “A request would instantiate an instance, including a nested instantiation during registration or under LIF-009, or open a Conduit by `open.v1` or a setup part under ACC-005, and a tombstone matches it.” Then: “After the access decision and every check giving invalid (OPS-023), the request is not dispatched and reports conflict; nothing changes. A Conduit matches where the tombstone has the same consumer and either the same local name or the same target and capability. Another instance matches the kind and name the tombstone holds. Names are compared engine-wide, as `within` absent means the engine.”

**OPS-021 (H; must).** When: “An instance is set up or retired again, and the invocation of its setup hook or its retire hook from an earlier setup or retirement has the outcome unknown.” Then: “That hook is not dispatched again, except as a new attempt of the same invocation where OPS-012 dispatches one, with the idempotency key that OPS-028 gave it.”

**OPS-022 (O; must).** When: “An invocation of `set-up.v1` or of `retire.v1` has ended.” Then: “Its outcome is succeeded only if every part not marked optional succeeded, failed only if no part had an effect, and otherwise unknown. In a further setup or retirement, a part that is not dispatched counts with the outcome it has from before or from its reconciliation.”

**OPS-028 (K; must).** When: “The engine dispatches a setup hook or a retire hook, under TPL-017 or TPL-018.” Then: “It gives the invocation an idempotency key, derived from the instance's ID and from the hook's operation, and sends it. The idempotency key is the same in every setup of that instance, or in every retirement of it, and is scoped to the instance alone, not to a caller, a sending instance or a target: a later dispatch of the hook matches the earlier invocation as a repeat does under OPS-009, whoever is the caller of the setup or the retirement it belongs to. So a further setup or retirement dispatches the hook again only as a new attempt, where OPS-012 or OPS-024 would dispatch one for a repeat, and a hook that succeeded is not dispatched again, as OPS-009 dispatches nothing for a repeat. A new attempt is made for the caller of the setup or the retirement within which it is dispatched, as TPL-017 and TPL-018 state, and from then on that is the caller of the hook's invocation, as an invocation counts as dispatched through the Conduit of its latest attempt under OPS-024.”

**POOL-003 (R; must).** When: “An actor asks for a new Resource for an entry while the Resource instantiated from that entry is not retired.” Then: “The request is not dispatched, and reports conflict.”

**POOL-007 (P; must not).** When: “A Resource instantiated from an entry is set up by a request that names that Resource.” Then: “The Resource of any other entry is set up because of it.”

**POOL-013 (P; must).** When: “A Pool is set up.” Then: “The setup of the Resource of each entry is a part of the Pool's setup, under the entry's name, begun only once the parts for the entries it refers to have succeeded. The part sets up that entry's Resource that is not retired, under POOL-015. For a Resource that is already ready nothing is dispatched, and the part counts as succeeded. Where the entry has no Resource that is not retired, the part is not dispatched and reports conflict.”

**POOL-017 (R; must).** When: “An actor asks under LIF-009 for a new Resource for an entry of a retired Pool.” Then: “The request is not dispatched, and reports invalid.”

**TPL-009 (O; must).** When: “An instance is retired by an invocation of `retire.v1`.” Then: “Within that invocation, where a setup of the instance was ever dispatched, the engine dispatches its retire hook, where its template selects one, under TPL-018, and then asks its runtime provider to stop its run, where an item that is run is selected for it, under PRV-008, whatever came of the hook: where it succeeded, failed or ended unknown, and where it was not dispatched. Nothing else is done, apart from the run or the connection that TPL-018 asks for to serve the hook, with the reads of secrets that it needs under SEC-010 and SEC-013, and the closing of its Conduits under CON-008. Where the invocation is cancelled under OPS-027 before the hook is dispatched, the hook is not dispatched, and the stop is asked for all the same.”

**TPL-012 (S; must).** When: “An instance is set up by an invocation of `set-up.v1`.” Then: “The setup is these parts, in this order: first opening its declared Conduits, under CON-004, or for a Pool setting up the Resources of its entries, under POOL-013; then asking for its run, under PRV-007, and connecting to each `remote` item among its selected implementations, under IFC-041; then, where its Type states signIn and its account pointer holds a string, checking the account with account-hook.v1 and undoing any different account with sign-out-hook.v1, under PND-005 and PND-011; then dispatching its setup hook, under TPL-017. A part is begun only once every part that comes before it in that order, and is not marked optional, has succeeded. No part is begun once the instance is retired, or once the setup is cancelled under OPS-027. Nothing else is dispatched, apart from the reads of secrets that the run or a connection needs, under SEC-010 and SEC-013.”

**TPL-017 (H; must).** When: “An instance is set up whose template selects an implementation for `setup-hook.v1`.” Then: “Within each invocation of `set-up.v1` for the instance, as the last part of the setup under TPL-012, the engine dispatches `setup-hook.v1` to that implementation, through no Conduit, for the caller of the setup, with the idempotency key that OPS-028 gives it.”

**TPL-018 (H; must).** When: “An instance for which a setup was ever dispatched is retired by an invocation of `retire.v1`, and its template selects an implementation for `retire-hook.v1`.” Then: “Within that invocation, before the stop of TPL-009, the engine dispatches `retire-hook.v1` to that implementation, through no Conduit, for the caller of the retirement, with the idempotency key that OPS-028 gives it, where the hook is to be dispatched: where no earlier invocation of it exists for the instance, or where OPS-012 or OPS-024 would dispatch a new attempt of the earlier one, under OPS-021 and OPS-028. Where the hook is to be dispatched, its implementation is an item that is run and its runtime provider reports the run `stopped`, the engine first asks for the run, under PRV-007; where it reports the run's state `unknown`, the hook is not dispatched and its part reports unavailable. Where the hook is to be dispatched, its implementation is a `remote` item and no connection to it lasts whose values the engine holds, the engine first connects to it, under IFC-041 and SEC-013. Where the hook is not to be dispatched, no run is asked for, no connection is made, and the retirement goes on to the stop.”

**INS-020 (F; must).** When: “A template from which instances have been made is replaced or unregistered.” Then: “Each of those instances keeps the definition and configuration it resolved when it was made. Having instances does not prevent a template from being replaced or unregistered.”

**ACC-005 (S; must).** When: “A setup opens a declared Conduit, under CON-004.” Then: “The part of the setup that opens it is a request for `open.v1` that names that Conduit, sent by the instance that sent the request to set up. It is decided under ACC-006 like any other request of that instance to open a Conduit: by that instance's own Conduit to `access`.”

**ACC-018 (K; must).** When: “The engine dispatches a setup hook or a retire hook, under TPL-017 or TPL-018.” Then: “The hook is dispatched to the implementation that its own instance has selected for the hook's operation, and is not a request through any Conduit. It carries the filters of the invocation of `set-up.v1` or `retire.v1` that it belongs to, as ACC-017 has an onward request that serves that invocation carry them, and that implementation applies them as IFC-019 states. A request that the implementation makes while serving the hook is an onward request, under ACC-016, which names the hook's invocation in `io.github.shrav-k.conduit/serving`.”

**ACC-039 (C; must).** When: “A request is made through an open Conduit whose consumer is retired. Or an onward request is made, through any Conduit, that serves an invocation in whose chain such a Conduit stands: the invocation that it names was dispatched through one, or an invocation of that invocation's chain was.” Then: “The request is permitted only if it belongs to the retirement of that consumer: it is an onward request that serves the consumer's retire hook, directly or through others, or the engine's read of a secret for a run that the retirement asks for, or for a connection that it makes, to serve that hook, under TPL-018 and SEC-011. Every other such request is not dispatched, and reports refused.”

**CON-004 (S; must).** When: “An instance that declares Conduits is set up.” Then: “The opening of each declared Conduit is a part of the setup, under the declared Conduit's name. Where the Conduit made for it is in the state registered, the part is a request for `open.v1` that names that Conduit, made by the instance that sent the request to set up, under ACC-005. Where that Conduit is open, nothing is dispatched and the part counts as succeeded. Where the nested instantiation has no Conduit that is not retired, the part reports conflict. Setup instantiates no Conduit.”

**CON-008 (C; must).** When: “An actor permitted under ACC-012 closes a Conduit, or its consumer or its target is retired or declared lost.” Then: “The Conduit is closed. A Conduit whose target is retired, or either of whose ends is declared lost or is placed on a machine that is reported gone, under PLC-024, is closed at once. A Conduit whose consumer is retired is closed once the consumer's shutdown is confirmed, or once a request to unregister the consumer removes it, under LIF-017. From the moment the retirement of a target is dispatched, a request to that target is a request through a closed Conduit: it is not dispatched, and reports refused. A closed Conduit is removed, under LIF-035.”

**CON-014 (B; must).** When: “A Conduit whose consumer is ready is closed.” Then: “The consumer is not retired or interrupted.”

**CON-023 (C; must).** When: “An actor asks to open a Conduit whose target is in the state registered.” Then: “The Conduit is opened as it would be to a target that is ready. A Conduit may be opened to any target that is not retired.”

**CON-031 (C; must).** When: “An actor asks to open a Conduit that is not open and whose consumer or whose target is retired.” Then: “The request is not dispatched, and reports conflict.”

### Expected verdict table (fixed before execution)

Safety columns in order are L S H K O B C R T D P F Q A V N, corresponding to
`SAFETY[0..16]`. Unlisted safety columns hold. A listed letter is refuted. Reach
columns are the 26 `REACH` predicates, numbered 0 through 25 in source order:

0 successful setup; 1 failed setup; 2 unknown setup; 3 failed hook's new attempt;
4 unknown hook retained without dispatch; 5 unknown observation; 6 ready explicit close;
7 tombstone conflict; 8 access refusal first; 9 invalid first; 10 new Conduit;
11 retired setup conflict; 12 removed setup invalid; 13 registered Conduit/retired
consumer; 14 unknown setup reconciled ready; 15 unknown setup reconciled retired;
16 unknown open reconciled after consumer retirement; 17 older retirement reconciled
failed; 18 retirement reconciled confirmed; 19 no setup ever dispatched;
20 target retirement while call in flight; 21 Pool setup twice; 22 plain Pool retirement
blocked; 23 new Resource name; 24 H silence; 25 quiescent. Unlisted reaches are absent.

The following row expansion is part of the table: run each Correct part with all eight
configurations. Start with its Base row; Silence adds reach 24, Pool Timer/Silence add
reach 2, Removal Crash removes reach 13, and Removal Timer adds reach 16. Every enabled
harness fault is reached in every row. The full-run test checks absent witnesses too.

| Part / configuration / variant | Refuted safety | Reached predicates |
| --- | --- | --- |
| Setup / Base / Correct | none | 0 1 2 3 4 5 14 25 |
| Removal / Base / Correct | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 |
| Pool / Base / Correct | V | 0 1 18 19 21 22 23 25 |

Variants use their part's expanded Correct row, with the changes below; every remaining
safety and reach entry stays exactly as in that row. One full run exhausts each variant,
and one separate single-thread BFS prints its first target refutation.

| Variant | Part/configuration | Safety changes | Reach changes |
| --- | --- | --- | --- |
| Regress | Setup/Base | L refuted | none |
| EarlyPart | Setup/Base | S refuted | none |
| RepeatHook | Setup/Base | H refuted | remove 4 |
| WrongKey | Setup/Base | K refuted | none |
| FalseSuccess | Setup/Base | O refuted | remove 2 14 |
| Rollback | Setup/Base | B refuted | remove 3 4 14 |
| KeepConduit | Removal/Base | C refuted, N holds | none |
| AutoReplace | Removal/Base | R refuted | remove 10 |
| IgnoreTombstone | Removal/Base | T refuted | none |
| EarlyRemove | Removal/Base | D refuted | remove 11 13 17 18 |
| EarlyPool | Pool/Base | P refuted | remove 18 19 23 25 |
| OtherData | Removal/Base | F refuted | none |
| Revive | Removal/Base | Q refuted | remove 12 |
| Unfenced | Setup/Timer | A refuted | none |
| HookPending | Removal/Base | D refuted, V holds | none |
| PreserveReferences | Removal/Base | C refuted, N holds | none |

No model run has occurred at the point this table was written. Results and any difference
will be appended below, preserving these expected rows.

### MC6 first-run observations and bounded revision (before the next run)

The first Setup runs exhausted: Base 2,876 states/0.089 s, LossR 3,915/0.126 s,
ReorderR 6,841/0.213 s, LossH 3,464/0.097 s, ReorderH 4,999/0.159 s,
Crash 56,480/2.168 s, Timer 88,897/3.828 s, Silence 161,346/7.215 s.
Their expected verdicts agreed. Removal/Base exhausted 1,757 states/0.089 s:
V and N were refuted as expected, but reach 20 was absent. The reach predicate
incorrectly remembered an earlier call's end across a new dispatch. It is corrected
to track the set of in-flight invocation IDs, preserving the expected table. This
was a witness defect, not a language counterexample (MC6-003).

Before the next run, the timing bounds are narrowed to meet the order's total-time
target: Setup crash only during the initial pair, Removal crash during c-prime's open
or the call/target-retirement pair, Pool crash during the first cascade. Setup timer
only during the second setup (including the unfenced hook control), Removal timer
during c-prime's explicit open, Pool timer during the first Pool setup. Silence is
allowed once during the first two stages, and its timer only after an omitted hook
reply while that part is still awaiting it. Recovery remains available after any
allowed crash. These are explicit bounded windows, not harness permissions inferred
from `fault_allowed`. Loss and reordering retain the full script. No verdict table
entry changes. Backing data and records are now saved symbolic state, observed by F
on removal; OtherData actually clears backing state before that observation.

The bounded revision exhausted 22 Correct rows with their expected verdicts, before
Pool/Timer exposed an implementation defect in this reference model: expiring a parent
while it awaited an entry child left no slot for the child's later answer. No verdict
was obtained for that row. Timers now expire only implementation wire parts or explicit
opens, not parent/child bookkeeping. A regression will cover that distinction.

Before rerunning, the scripts also complete several previously thin slices: Setup now
closes c, lifts its tombstone, registers c-prime and sets I up to open it. Removal asks
to unregister ready I in the existing live-slot request stage; it retires I twice more
(the second hook is unknown, the third retains it without dispatch), reconciles that
hook and probes an ordinary request through I's still-open d. The target-retirement
crash window consequently moves to stage 31. No old expected cell is changed; Setup's
additional reaches are observations against the original table. New reach columns,
registered before their first run, are 26 closed declaration slot (Removal), 27 replacement
opened by setup (Setup), 28 retired Pool entry invalid (Pool), 29 unknown retirement
hook retained, 30 retire hook reconciled, 31 active unregistration conflict, 32 retire
hook onward and 33 ordinary retired consumer refused (all five in Removal). Each is
expected absent in the other parts. Their parent variant initially inherits those entries;
any difference will be retained with the old table's other differences.

Pool/Timer then hit its 10-second deadline; that truncated run supplies no verdict.
Before the next run, Pool has a stated phase boundary: it drains implementation messages
and jobs from the first Pool setup before sending the second one. The unknown outcomes
stay unknown; retirement still owes hook and stop after an unknown setup. This bounds
late setup messages rather than excluding a safety discovery. Other parts retain their
late-message interleavings. The timer's wire-part restriction remains in force.

The pre-run source review also restricts retire-hook onward requests through d to I,
d's actual consumer; other instances have no such route. An ordinary request through
rT to registered T is unavailable, rather than dispatched. S now independently checks
the complete mandatory order represented by the model, D checks each retired setup's
stop debt, F checks the instance ID of each stop, and P checks that a direct member's
setup creates no child setup. These checks retain their registered expected verdicts.

The 40-row run next exhausted in 20.22 seconds. The differences are retained below;
none required changing a promise. A source check before the next run corrects unnecessary
run dispatch: a run already reported running is a succeeded no-op; a retained unknown or
succeeded retire hook asks for no run. Actual run/stop effects now reflect a transition
in H's saved run state. L also checks lifecycle monotonicity in reconciliation events,
so Revive may refute L as well as Q. No prior table cell is rewritten.

A separate pre-registered control column A2, appended after N, checks only the hook effect
count. It is expected refuted for Unfenced and RepeatHook, and holds in every other row.
It has the same OPS-021/OPS-028 preconditions as A; it adds no demand to a failed hook's
new attempt (a failed hook has had no effect). A second Unfenced refutation targets A2,
so the printed trace reaches the repeated effect, beyond the first bad negative record.

### MC6 differences retained beside the expected table

The next 40-row run exhausted in 16.80 seconds after the run-dispatch correction.
The following is the complete observed delta from the original table plus its registered
extensions. “+ safety” means additionally refuted, “- safety” means holds instead.
“+ reach” means reached instead of absent, “- reach” means absent instead of reached.
Unlisted cells agree. These are MC6-004, an expected-table/accounting gap; they are not
additional unsafe findings in the language. `registered` and `observed` are separate
functions, and the test rejects any difference beyond these recorded observations.

| Rows | + safety | - safety | + reach | - reach |
| --- | --- | --- | --- | --- |
| Setup / every Correct configuration | none | none | 6 10 | none |
| Pool / Timer and Silence / Correct | none | none | 4 | none |
| Setup/Base / Regress, EarlyPart, WrongKey, FalseSuccess | none | none | 6 10 | none |
| Setup/Base / RepeatHook | A | none | 6 10 | 14 |
| Setup/Base / Rollback | none | none | 3 10 14 26 | none |
| Removal/Base / IgnoreTombstone | none | none | none | 7 |
| Removal/Base / EarlyRemove | none | none | none | 29 30 32 33 |
| Pool/Base / EarlyPool | none | none | 18 25 | none |
| Removal/Base / Revive | L | N | 27 | none |
| Setup/Timer / Unfenced | none | none | 6 10 | none |

The additional Setup reaches follow its explicitly registered completion of the replacement
sequence. A Pool setup after a timer or omitted hook can retain that unknown hook in its
second setup. RepeatHook also repeats an effect and makes I ready before the reconciliation
witness's registered-to-ready condition. Rollback still permits later repair and a new
failed hook attempt. IgnoreTombstone removes the sole matching-conflict answer. EarlyRemove
precludes the later retire-hook and open-Conduit probes; EarlyPool still permits retirement
reconciliation and eventual quiescence within this finite script. Revive also violates lifecycle
monotonicity, prevents unregistration by making I active, and permits its replacement to open;
therefore its N refutation disappears. These interactions are retained, not hidden by weakening
predicates or deleting their original expected cells.

### Final pre-run coverage additions

The complete crate passed 39 tests in 56.75 test seconds (59.56 seconds wall). Before
its final measurement, two remaining conditions receive explicit witnesses. T's failed
retire hook now has a failed stop as well: its run is already running, so the run part
is a no-op, no part has an effect, retirement fails and shutdown stays pending. I's
failed hook and successful stop still exercise MC6-001 unchanged.

One additional Correct configuration, **Removal/ShutdownTimer**, has a timer budget of
one and every other fault budget zero for the same split-fault reasons. Its timer is
enabled only while existing I is retired with unknown shutdown, and records the before
and after shutdown without changing it. The new **TimeConfirms** variant in that same
configuration confirms and removes I on this tick; D must be refuted. It inherits
Removal/Base's V/N refutations. Its expected reaches are the Removal/Base row minus
11, 13, 29, 30, 32 and 33. Every other expected entry inherits the registered row and
its earlier extensions. New reach 34 (tick after unknown shutdown) is expected in these
two new rows only; reach 35 (a retirement ends failed) is expected in every Removal
row only. A tick's before/after equality is D's LIF-018 check, with its complete when/then
quoted above. These additions precede their first run. The final matrix has 42 full runs
and 17 variants, with no increase in any state ceiling, timeout or fault budget.

ReorderH's Removal witness, after the already-running run became a no-op, is the ordinary
call and **retire-hook itself**, both outstanding on E-to-H; no answer to the call is
required before R sends the target retirement.

The first added-row run exposed a missing clause in the new D predicate: it did not
compare the tick's recorded before and after values, so TimeConfirms initially refuted
nothing beyond the existing V/N contradictions. That reference-property defect is fixed;
its originally registered D refutation is retained. The refutation test caught it.
The optional tick also permits every original Removal reach before it fires (or on a
branch where it does not fire): TimeConfirms therefore additionally reaches 11, 13,
29, 30, 32 and 33 against its registered exclusions. This is another row of MC6-004;
the original expected entries are unchanged. No specification counterexample is claimed
from either bookkeeping defect. L's successful-setup condition explicitly includes
`open.v1`, and C also checks the registered-target opening case with all its modelled
access, existence and tombstone preconditions.

The final coverage review also observes H's actual consumer run on both sides of explicit
closure, rather than inferring “not interrupted” solely from lifecycle. Before running it,
**CloseStops/Removal/Base** is registered: it stops I's run when R closes c while I is
ready, and must refute B alone in addition to Base's V/N contradictions. Its reach entries
are exactly the registered Removal/Base row plus the earlier observed deltas and reach
35. The baseline closure never changes the run. This adds one full run and one first
refutation: the completed matrix is **43 runs, 25 Correct configurations and 18 variants**.

### MC6 findings, traces and limits

**MC6-001 — contradiction: retire-hook contract versus OPS-022/LIF-024.** The first
single-thread BFS trace on Correct/Removal/Base has 92 actions. Its relevant projection
is open c to registered T; setup I ends unknown; reconcile its hook; setup I succeeds;
close c while I stays ready; reject setup against the closed slot; lift the tombstone;
register c-prime; dispatch I's retirement; retain its already-running run without another
run dispatch; dispatch its retire hook (failed); dispatch its stop (succeeded); end the
retirement unknown and report shutdown unknown. The last part has an effect, so failed
is not an allowed outcome; the mandatory failed hook excludes succeeded. Literal V,
“a hook that did not succeed keeps the shutdown pending”, is refuted. HookPending's
separate 92-action first trace has the same dispatches and outcomes but reports pending:
V holds and D (LIF-024) is refuted. Both exact wire/local traces are printed by the tests.
The model follows the order's OPS-022/LIF-024 reading; the specification is untouched.

**MC6-002 — contradiction: LIF-019 versus CON-008/LIF-017.** Correct/Removal/Base's
first N refutation has 147 actions. After the preceding retirement, I stays retired
with unknown shutdown; two further retirements retain an unknown hook until reconciliation.
The script's ordinary request through d is refused. The actor then unregisters I;
E asks H for the additional stop, H succeeds, and E removes I and closes its registered
c-prime and open d. Both Conduits refer to I as consumer. Closing retires them, which
LIF-019's literal prohibition forbids. PreserveReferences has the corresponding
147-action first C refutation: it keeps both Conduits when removing I, making literal N
hold but breaking CON-008. F excludes these expressly permitted Conduit changes and
checks that resolved definitions, records, backing data and the unrelated object remain.

**Unfenced control — existing reconciliation gap, OPS-014/OPS-024/OPS-028.** Its first
A refutation is 37 actions: a timeout records I's second hook attempt unknown while the
dispatch can still act; the actor observes no effect and records failed without fencing.
Its separate A2 refutation is 50 actions: the next eligible setup dispatches a third
attempt with the same key; H completes both the old unknown attempt and the new attempt,
recording two hook effects. No copied dispatch or lost saved state is needed. Correct
requires no dispatch or implementation job capable of a later effect before recording
failed, and refutes neither A nor A2. This reproduces MC2-001 in lifecycle context under
present OPS-014; the in-force/spec-pending fenced reading prevents it in this model.

**MC6-003 — reference-model defects found during construction.** The original in-flight
witness remembered an earlier invocation's end. A Pool parent timer left no slot for a
late child answer. The final tick variant caught a missing before/after comparison in
D. These were defects in this experiment, not contradictions in Conduit; the two
structural regressions and seeded TimeConfirms refutation check their fixes. Truncated
searches were rejected and supply no verdict. The original table is retained beside
the revised source bounds and MC6-004's complete observed deltas.

The first-trace lengths for the 18 variants are printed in the counted run. Properties
read the ledger after all node data and queues are erased on replay. Whole-invocation
positive reconciliation assumes an authorized actor's checked evidence; this model
does not prove its evidence collection. Hook reconciliation explicitly observes H's
effect state, and every negative reconciliation fences queued dispatches and H jobs.
The model tests saved engine state, symbolic resources and fixed definitions, not an
engine implementation, filesystem, cryptography, provider contract, unbounded execution,
or simultaneous independent fault kinds. It establishes no liveness or fairness.

For MC8's slice accounting: ACC-039 covers direct retire-hook onward requests and direct
ordinary requests from the retired consumer; indirect invocation chains and retirement
secret reads are absent. ACC-005 carries the initiating caller with fixed unlimited
access; comparisons among narrower bounds are outside this model. Positive reconciliation
records are actor inputs, not a model of provider-specific evidence or external repair.


### MC6 final dated results — 2026-11-11

Final source: formatting and clippy with warnings denied passed. `cargo test --release`
passed **39 tests**, with **0 doc tests**, in **64.46 test seconds / 71.36 seconds wall**
(including compilation). The counted command passed the same **39 tests / 0 doc tests**
in **62.47 test seconds / 62.74 seconds wall**. The MC6 full searches alone total
**16.093 seconds** in that counted run; the separate first-refutation searches add to
that. The largest MC6 search has **47,256 unique states**; the slowest is **2.013 seconds**.
All 43 final searches exhausted; none was truncated. All enabled harness fault witnesses
were reached, including recovery and every configured timer. Every observed verdict and
reach agrees with the retained observed table; differences from the original expected
table are exactly MC6-004, including the later TimeConfirms row.

The table gives actual unique states, depth (initial state counts as one), seconds,
refuted safety columns, and every reached protocol predicate. All unlisted safety
columns hold; all unlisted protocol predicates are absent. Harness witnesses are separate
and all passed. Safety columns are L S H K O B C R T D P F Q A V N A2; reaches 0–35
are defined above. These are measurements from the final counted run, not forecasts.

| Part/configuration/variant | Unique states | Depth | Seconds | Refuted | Reached |
| --- | --- | --- | --- | --- | --- |
| Setup/Base/Correct | 3,121 | 89 | 0.094 | none | 0 1 2 3 4 5 6 10 14 25 27 |
| Setup/LossR/Correct | 4,440 | 89 | 0.149 | none | 0 1 2 3 4 5 6 10 14 25 27 |
| Setup/ReorderR/Correct | 7,478 | 89 | 0.285 | none | 0 1 2 3 4 5 6 10 14 25 27 |
| Setup/LossH/Correct | 3,604 | 89 | 0.129 | none | 0 1 2 3 4 5 6 10 14 25 27 |
| Setup/ReorderH/Correct | 5,426 | 89 | 0.180 | none | 0 1 2 3 4 5 6 10 14 25 27 |
| Setup/Crash/Correct | 33,902 | 92 | 1.449 | none | 0 1 2 3 4 5 6 10 14 25 27 |
| Setup/Timer/Correct | 44,141 | 90 | 1.937 | none | 0 1 2 3 4 5 6 10 14 25 27 |
| Setup/Silence/Correct | 11,810 | 89 | 0.380 | none | 0 1 2 3 4 5 6 10 14 24 25 27 |
| Removal/Base/Correct | 1,232 | 232 | 0.073 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/LossR/Correct | 1,737 | 232 | 0.099 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/ReorderR/Correct | 3,878 | 232 | 0.260 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/LossH/Correct | 1,493 | 232 | 0.087 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/ReorderH/Correct | 1,992 | 232 | 0.131 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/Crash/Correct | 17,086 | 239 | 1.399 | V N | 0 1 2 5 6 7 8 9 10 11 12 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/Timer/Correct | 3,540 | 236 | 0.209 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/Silence/Correct | 2,449 | 232 | 0.150 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 24 25 26 29 30 31 32 33 35 |
| Pool/Base/Correct | 2,967 | 143 | 0.106 | V | 0 1 18 19 21 22 23 25 28 |
| Pool/LossR/Correct | 3,714 | 143 | 0.154 | V | 0 1 18 19 21 22 23 25 28 |
| Pool/ReorderR/Correct | 10,453 | 143 | 0.411 | V | 0 1 18 19 21 22 23 25 28 |
| Pool/LossH/Correct | 3,731 | 143 | 0.160 | V | 0 1 18 19 21 22 23 25 28 |
| Pool/ReorderH/Correct | 5,332 | 143 | 0.214 | V | 0 1 18 19 21 22 23 25 28 |
| Pool/Crash/Correct | 19,410 | 146 | 0.947 | V | 0 1 18 19 21 22 23 25 28 |
| Pool/Timer/Correct | 34,131 | 144 | 1.636 | V | 0 1 2 4 18 19 21 22 23 25 28 |
| Pool/Silence/Correct | 12,911 | 143 | 0.510 | V | 0 1 2 4 18 19 21 22 23 24 25 28 |
| Removal/ShutdownTimer/Correct | 13,396 | 233 | 0.850 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 34 35 |
| Setup/Base/Regress | 3,121 | 89 | 0.097 | L | 0 1 2 3 4 5 6 10 14 25 27 |
| Setup/Base/EarlyPart | 846 | 83 | 0.019 | S | 0 1 2 3 4 5 6 10 14 25 27 |
| Setup/Base/RepeatHook | 3,436 | 98 | 0.104 | H A A2 | 0 1 2 3 5 6 10 25 27 |
| Setup/Base/WrongKey | 3,121 | 89 | 0.094 | K | 0 1 2 3 4 5 6 10 14 25 27 |
| Setup/Base/FalseSuccess | 3,121 | 89 | 0.096 | O | 0 1 3 4 5 6 10 25 27 |
| Setup/Base/Rollback | 3,360 | 83 | 0.102 | B | 0 1 2 3 5 10 14 25 26 27 |
| Removal/Base/KeepConduit | 1,232 | 232 | 0.071 | C V | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/Base/AutoReplace | 1,232 | 232 | 0.072 | R V N | 0 1 2 5 6 7 8 9 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/Base/IgnoreTombstone | 1,232 | 232 | 0.071 | T V N | 0 1 2 5 6 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/Base/EarlyRemove | 1,012 | 194 | 0.062 | D V N | 0 1 2 5 6 7 8 9 10 12 14 15 19 20 25 26 31 35 |
| Pool/Base/EarlyPool | 3,009 | 145 | 0.107 | P V | 0 1 18 21 22 25 28 |
| Removal/Base/OtherData | 1,232 | 232 | 0.072 | F V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/Base/Revive | 1,234 | 234 | 0.071 | L Q V | 0 1 2 5 6 7 8 9 10 11 13 14 15 17 18 19 20 25 26 27 29 30 31 32 33 35 |
| Setup/Timer/Unfenced | 47,256 | 93 | 2.013 | A A2 | 0 1 2 3 4 5 6 10 14 25 27 |
| Removal/Base/HookPending | 1,232 | 232 | 0.072 | D N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/Base/PreserveReferences | 1,232 | 232 | 0.072 | C V | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |
| Removal/ShutdownTimer/TimeConfirms | 13,324 | 233 | 0.827 | D V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 34 35 |
| Removal/Base/CloseStops | 1,235 | 235 | 0.072 | B V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 |

First single-thread BFS refutation lengths in the counted run (actions):

| Variant | Target | Actions |
| --- | --- | --- |
| Regress | L | 86 |
| EarlyPart | S | 3 |
| RepeatHook | H | 35 |
| WrongKey | K | 8 |
| FalseSuccess | O | 12 |
| Rollback | B | 8 |
| KeepConduit | C | 147 |
| AutoReplace | R | 54 |
| IgnoreTombstone | T | 64 |
| EarlyRemove | D | 92 |
| EarlyPool | P | 64 |
| OtherData | F | 147 |
| Revive | Q | 133 |
| Unfenced | A | 37 |
| HookPending | D | 92 |
| PreserveReferences | C | 147 |
| TimeConfirms | D | 93 |
| CloseStops | B | 50 |

The additional Unfenced/A2 trace has 50 actions; Correct/Removal/Base's literal V and N
traces have 92 and 147 respectively. All 18 variant traces remain refuted after erasing
node data and queues on replay. The Pool child-timer regression and the per-invocation
in-flight witness regression pass. No vector was written. No harness, existing model,
specification, engine, ruling, catalogue or other experiment was changed. The accounting
remains **47 modelled slices and 11 static exclusions**, with the partial exclusions and
supporting promises explicitly kept outside those claims above. MC8 and the owner must
carry the two contradictions and the pending fencing rule forward; this work order does
not amend any binding text.

### MC6 verifier revision: registration before execution

The admin's next verifier found four unchecked obligations in the reference model.
This revision changes only the lifecycle module and this section. The scripts, fault
windows, budgets and bounds above stay in force. The original expected and observed
tables above are retained. This registration precedes any run of the revised model.

- **INS-020 / F:** each instance stores its immutable creation-time resolved definition
  separately from its current definition. Template replacement/unregistration logs the
  actual post-operation definition of I against that saved resolution, and removal
  compares against the same saved resolution, rather than a fresh snapshot. Replacements
  save their own resolution under their new ID. The definition number represents the
  resolved definition and configuration together, as in the original bounded model.
- **TPL-009 / O and TPL-018 / H:** a retirement-dispatch event records whether a setup
  was ever dispatched and the prior retire-hook outcome. The ledger scan requires an
  eligible hook's actual dispatch before the stop when its run succeeded, and a processed
  hook part before every stop, including failed, unknown and retained hooks. A hook
  dispatch requires a successful run part and no preceding stop in that invocation.
  Every completed retirement with a dispatched setup must contain an actual stop
  dispatch and its completed part; a never-set-up instance has no retirement parts.
  H also retains the eligibility check against None/Failed, and K checks identity.
  This checks ordering and required parts as well as O's existing outcome aggregation.
  Cancellation, remote items and run-reported-unknown remain excluded above.
- **LIF-035 / D:** closing a Conduit, completing a retirement, positively reconciling a
  retirement, and the quiescent boundary log actual existence. A retired instance with
  confirmed shutdown must be absent at each such completed boundary. Intermediate
  ledger events within the atomic completion are not mistaken for a separate boundary.
  Records remain covered by F. This is a safety condition on completion, not liveness.
- **LIF-005 and LIF-023 / L:** actor requests are logged when R sends them. Every
  retirement dispatch must immediately record retired and match that actor request,
  or belong to an actor-requested Pool cascade. Every new transition to retired must
  have a preceding retirement dispatch or a permitted Conduit closure: explicit close,
  retirement of its target, or removal of its consumer. Cascade dependencies remain P's
  responsibility. Machine loss and Fleets remain outside this model. A later request
  or state transition cannot justify an earlier transition.

These implement the complete when/then quotations already registered above for those
six promises, with the same bounded preconditions and exclusions. They correct model
defects; they do not amend the promises or the two recorded contradictions.

New witnesses, appended without renumbering existing reaches: 36 a changed template
differs from the instance's saved resolution; 37 stop after a failed retire hook;
38 stop after an unknown retire hook; 39 stop after a retained retire hook; 40 a
confirmed retired non-Conduit instance is absent; 41 retirement is recorded retired
at dispatch. The template witness observes the precondition independently of F's
verdict, so it also reaches in the broken adoption control.

Expected additions to every existing row: Setup reaches 36; Removal reaches
36–41; Pool reaches 37, 38, 40, 41. Existing safety verdicts and reach 0–35 are
expected unchanged. Every unlisted new reach is absent. Five new controls run once
each, with a separate first-refutation run. Their complete expected verdicts use the
previously retained observed Correct row for their part/configuration, plus:

| Control | Part/configuration | Additional refuted safety | Reach changes |
| --- | --- | --- | --- |
| AdoptTemplate | Setup/Base | F | none |
| StopBeforeHook | Removal/Base | H O | remove 37 38 39 |
| KeepConfirmed | Removal/Base | D | remove 40 |
| DelayRetirement | Removal/Base | L | none |
| UnsolicitedRetirement | Setup/Base | L | none |

AdoptTemplate replaces I's current resolved definition when its template changes,
while retaining the creation snapshot. StopBeforeHook plans run/stop/hook instead of
run/hook/stop. KeepConfirmed omits automatic removal after both successful retirement
and positive reconciliation but still allows explicit unregistration. DelayRetirement
leaves lifecycle unchanged at dispatch and sets retired only at completion.
UnsolicitedRetirement retires I when its template is unregistered, without an actor
retirement request or a listed exception. Its transition is monotone and attributed
to template unregistration, so the old monotonicity/observation predicate would accept it.

### MC6 verifier revision: observed differences

The first revised full-table test stopped at Rollback's additional L refutation. The
next diagnostic run exhausted all 48 rows and collected every discrepancy before
failing. The registered expectations above remain unchanged. Observed changes are:

| Row | Registered revision expectation | Observed |
| --- | --- | --- |
| Setup/Base/Rollback | B refuted | L and B refuted: rollback closes c without an authorized retirement cause |
| Removal/Base/EarlyRemove | New reaches 38 and 39 present | Both absent: early removal prevents I's later unknown and retained retire hooks |
| Removal/Base/StopBeforeHook | Existing reach 18 present | Absent: J's second stop is chosen before its second hook attempt, so it succeeds; that retirement succeeds and removes J before the positive reconciliation request |

These are additional expected-table gaps in the experiment (MC6-005), not new language
counterexamples. The stronger predicates accept every Correct row with the same two
previously recorded literal contradictions. All five new controls refute their
registered safety properties. The executable observed table records exactly these
three changes beside the earlier MC6-004 differences. The full-run test now collects
all discrepancies before failing, so one mismatch cannot hide later rows.

### MC6 verifier revision results — 2026-11-11

All four verifier findings are fixed within this order's scope. Formatting and
Clippy with warnings denied passed. The ordinary release suite passed **39 tests /
0 doc tests** in **64.74 test seconds / 71.42 seconds wall**, including compilation.
The counted single-test-thread suite passed the same **39 tests / 0 doc tests** in
**64.24 test seconds / 64.47 seconds wall**. No timing-sensitive shell test failed,
and no `RUST_TEST_THREADS=2` rerun was needed. `CARGO_BUILD_JOBS` was inherited as 6;
cargo commands ran one at a time.

The 48 MC6 full searches total **20.197 seconds**, plus the separate first-refutation
searches. The largest search has **65,666 unique states**; the slowest is
**3.018 seconds**. These are measured values; the approximately
20-second target remains a target, not a result. All searches exhausted below the
190,000-state and ten-second bounds. Every configured fault had its witness. All
verdicts and reaches match the observed table, including MC6-005's three additions;
there are no further differences. The two existing literal contradictions remain.

The measurements below replace no earlier table. Columns have the same meanings as
the earlier results; reaches now range from 0 to 41. Unlisted safety columns hold and
unlisted reaches are absent.

| Part/configuration/variant | Unique states | Depth | Seconds | Refuted | Reached |
| --- | --- | --- | --- | --- | --- |
| Setup/Base/Correct | 3,121 | 89 | 0.087 | none | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Setup/LossR/Correct | 4,440 | 89 | 0.144 | none | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Setup/ReorderR/Correct | 7,478 | 89 | 0.271 | none | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Setup/LossH/Correct | 3,604 | 89 | 0.123 | none | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Setup/ReorderH/Correct | 5,426 | 89 | 0.176 | none | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Setup/Crash/Correct | 36,282 | 92 | 1.363 | none | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Setup/Timer/Correct | 62,691 | 90 | 3.018 | none | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Setup/Silence/Correct | 11,810 | 89 | 0.359 | none | 0 1 2 3 4 5 6 10 14 24 25 27 36 |
| Removal/Base/Correct | 1,232 | 232 | 0.082 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/LossR/Correct | 1,737 | 232 | 0.108 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/ReorderR/Correct | 3,878 | 232 | 0.257 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/LossH/Correct | 1,493 | 232 | 0.099 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/ReorderH/Correct | 1,992 | 232 | 0.142 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/Crash/Correct | 19,633 | 239 | 1.522 | V N | 0 1 2 5 6 7 8 9 10 11 12 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/Timer/Correct | 3,540 | 236 | 0.210 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/Silence/Correct | 2,449 | 232 | 0.153 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 24 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Pool/Base/Correct | 2,967 | 143 | 0.103 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/LossR/Correct | 3,714 | 143 | 0.144 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/ReorderR/Correct | 10,453 | 143 | 0.395 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/LossH/Correct | 3,731 | 143 | 0.148 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/ReorderH/Correct | 5,332 | 143 | 0.195 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/Crash/Correct | 23,127 | 146 | 1.019 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/Timer/Correct | 34,131 | 144 | 1.540 | V | 0 1 2 4 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/Silence/Correct | 12,911 | 143 | 0.459 | V | 0 1 2 4 18 19 21 22 23 24 25 28 37 38 40 41 |
| Removal/ShutdownTimer/Correct | 24,458 | 233 | 1.679 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 34 35 36 37 38 39 40 41 |
| Setup/Base/Regress | 3,121 | 89 | 0.089 | L | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Setup/Base/EarlyPart | 846 | 83 | 0.019 | S | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Setup/Base/RepeatHook | 3,436 | 98 | 0.098 | H A A2 | 0 1 2 3 5 6 10 25 27 36 |
| Setup/Base/WrongKey | 3,121 | 89 | 0.091 | K | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Setup/Base/FalseSuccess | 3,121 | 89 | 0.089 | O | 0 1 3 4 5 6 10 25 27 36 |
| Setup/Base/Rollback | 3,360 | 83 | 0.096 | L B | 0 1 2 3 5 10 14 25 26 27 36 |
| Removal/Base/KeepConduit | 1,232 | 232 | 0.122 | C V | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/Base/AutoReplace | 1,232 | 232 | 0.087 | R V N | 0 1 2 5 6 7 8 9 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/Base/IgnoreTombstone | 1,232 | 232 | 0.082 | T V N | 0 1 2 5 6 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/Base/EarlyRemove | 1,012 | 194 | 0.059 | D V N | 0 1 2 5 6 7 8 9 10 12 14 15 19 20 25 26 31 35 36 37 40 41 |
| Pool/Base/EarlyPool | 3,009 | 145 | 0.100 | P V | 0 1 18 21 22 25 28 37 38 40 41 |
| Removal/Base/OtherData | 1,232 | 232 | 0.082 | F V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/Base/Revive | 1,234 | 234 | 0.078 | L Q V | 0 1 2 5 6 7 8 9 10 11 13 14 15 17 18 19 20 25 26 27 29 30 31 32 33 35 36 37 38 39 40 41 |
| Setup/Timer/Unfenced | 65,666 | 93 | 2.960 | A A2 | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Removal/Base/HookPending | 1,232 | 232 | 0.082 | D N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/Base/PreserveReferences | 1,232 | 232 | 0.080 | C V | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Removal/ShutdownTimer/TimeConfirms | 24,309 | 233 | 1.684 | D V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 34 35 36 37 38 39 40 41 |
| Removal/Base/CloseStops | 1,235 | 235 | 0.082 | B V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Setup/Base/AdoptTemplate | 3,121 | 89 | 0.089 | F | 0 1 2 3 4 5 6 10 14 25 27 36 |
| Removal/Base/StopBeforeHook | 1,232 | 232 | 0.082 | H O V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 19 20 25 26 29 30 31 32 33 35 36 40 41 |
| Removal/Base/KeepConfirmed | 1,232 | 232 | 0.082 | D V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 41 |
| Removal/Base/DelayRetirement | 1,232 | 232 | 0.080 | L V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 |
| Setup/Base/UnsolicitedRetirement | 3,121 | 89 | 0.088 | L | 0 1 2 3 4 5 6 10 14 25 27 36 |

First single-thread BFS refutations, including all five new controls:

| Variant | Target | Actions |
| --- | --- | --- |
| Regress | L | 86 |
| EarlyPart | S | 3 |
| RepeatHook | H | 35 |
| WrongKey | K | 8 |
| FalseSuccess | O | 12 |
| Rollback | B | 8 |
| KeepConduit | C | 147 |
| AutoReplace | R | 54 |
| IgnoreTombstone | T | 64 |
| EarlyRemove | D | 92 |
| EarlyPool | P | 64 |
| OtherData | F | 147 |
| Revive | Q | 133 |
| Unfenced | A | 37 |
| HookPending | D | 92 |
| PreserveReferences | C | 147 |
| TimeConfirms | D | 93 |
| CloseStops | B | 50 |
| AdoptTemplate | F | 64 |
| StopBeforeHook | O | 84 |
| KeepConfirmed | D | 220 |
| DelayRetirement | L | 82 |
| UnsolicitedRetirement | L | 83 |

Every first variant refutation remains refuted after all node data and queues are
erased. The pre-existing Pool timer and in-flight invocation regressions pass. The
accounting remains **47 modelled slices / 11 static exclusions**, with the same
partial exclusions and supporting promises above. The revision adds no conformance
vector and changes no specification or other model. The existing contradictions
and pending fenced-reconciliation rule still require the owner/MC8 follow-up already
recorded above.

### MC6 final verifier checks — 2026-11-11

Work order 114 closes four faults in the model's checks. This registration precedes
its first execution. All earlier registrations, witness numbers and the heading
“MC6 findings, traces and limits” remain unchanged, as do MC6-001 and MC6-002.

- **K / ACC-018:** Begin saves the owner's creation-time resolved definition. Each
  Dispatch carries the definition from which the engine selected its implementation.
  K derives the expected instance, hook operation and definition from the owning
  Begin, and checks target, key and caller against that owner. The definition number
  identifies the resolved implementation/configuration in this bounded model.
  “Not a request through any Conduit” holds by the model's structure: hooks are sent
  directly from E to H; no hook passes through a Conduit request or access path.
- **L / LIF-010:** Observe now waits for a model choice: the implementation answers
  with its lifecycle observation, or does not answer and the observation is reported
  unknown. Neither choice consumes a fault or adds a budget. The observation event
  logs the unavailable condition, reported value (`None` means unknown), and lifecycle
  before and after. Witness 5 remains the original Observe witness. Appended witness
  **42** requires an unavailable observation reported unknown. L checks that its
  lifecycle is unchanged. Machine placement and PLC-023 remain a slice outside the
  model; MC4's property P2 covers PLC-023. The explicit unavailability choice is the
  LIF-010 precondition within this model, so the accounting is expected to remain
  **47 modelled / 11 not modelled**.
- **D / LIF-014, LIF-018, LIF-024, LIF-025:** track retirement dispatch and end by
  invocation ID. Every Retire, State, Existence and retirement Reconciled event while
  the latest retirement is active must report pending, except when no setup was ever
  dispatched or an earlier retirement succeeded or was reconciled as having occurred.
  Confirmation is retained across a further retirement. D also checks the latest
  negative reconciliation restores pending, independently of Q's existing check.
- **Negative reconciliation / LIF-026:** J has a third retirement before its positive
  reconciliation. It ends unknown without applying any effect, then is fenced and
  negatively reconciled as the latest retirement. Appended witness **43** requires
  exactly J's latest, unknown, fenced retirement with no effect in H's ledger and
  shutdown unknown before reconciliation. The earlier negative case stays, and
  witness 18 must still reach in Removal. To make that earlier negative truthful too,
  J's first stop reports unknown without applying an effect; its failed hook also
  applies none. Its second retirement applies its successful hook and stop; its third
  reuses the hook and stops an already stopped run. H logs all applied retirement effects by
  invocation, and a negative retirement reconciliation is admitted only with none.

New controls, registered before execution, run once each plus a first-refutation
search. Their expected complete verdicts start from the retained observed Correct
row for their part/configuration, with the additions below. Every existing row is
expected to retain its safety verdicts and reaches 0–41; Setup and Removal add reach
42, Removal adds reach 43, Pool adds neither. Existing table differences remain.

| Control | Part/configuration | Additional refuted safety | Reach changes |
| --- | --- | --- | --- |
| WrongHookInstance | Setup/Base | K | none |
| CurrentTemplateHook | Removal/Base | K | none |
| UnknownAtDispatch | Removal/Base | D | none |
| KeepLatestUnknown | Removal/Base | D Q | none |

WrongHookInstance sends I's setup hook to J with a key consistent with J.
CurrentTemplateHook selects a hook implementation from the current template after
ReplaceTemplate, rather than the instance's saved resolved definition.
UnknownAtDispatch reports unknown in place of pending at retirement dispatch.
KeepLatestUnknown leaves shutdown unknown after J's latest negative reconciliation.
Targeted regression tests also remove or corrupt the new ledger evidence to check
that each strengthened predicate/witness depends on its full precondition.

The first diagnostic run passed seven lifecycle tests and failed the full-table test
only on two reach differences. All 52 searches exhausted; all registered safety
verdicts matched. The unchanged registrations are retained:

| Row | Registered final-check expectation | Observed |
| --- | --- | --- |
| Removal/Base/StopBeforeHook | 17 reached, 18 absent (the earlier observed row) | 17 absent, 18 reached: the control's first stop is selected before the first hook and succeeds, so that retirement has an applied stop and cannot truthfully be reconciled negatively; J's later stop is unknown and its positive reconciliation still occurs |
| Setup/Base/WrongHookInstance | 4 reached | 4 absent: the reply updates J's hook, so I never retains an unknown hook for reuse |

These are model-table differences, not language findings. The executable observed
table adds exactly these differences. To keep exploration small, the next run logs
H's additional per-invocation effect events only for retirements, which are the
invocations whose negative records need them. The existing hook Applied ledger
continues to log every hook effect. No script, choice, fault budget or verdict is
changed by omitting redundant run-effect events for setup invocations.

The queued Observe choice also participates in the quiescent boundary: an unanswered
observation still awaiting that choice prevents quiescence. This preserves the
original boundary's meaning after making observations explicit.

Final source review restricts H's first-stop no-effect choice to an **unknown** stop.
A successful stop in StopBeforeHook still applies its effect; a regression in the
negative-reconciliation test checks H's stopped run and applied-stop ledger. This
keeps the control's original successful-stop meaning while making every negative
record truthful. Its registered verdicts and the two recorded reach differences
are unchanged. The complete release checks below use this final source.

Final validation passed **43 tests / 0 doc tests** in **78.15 test
seconds / 87.22 seconds wall** for `cargo test --release`, including compilation.
The counted `--nocapture --test-threads=1` run passed the same totals in
**78.00 test seconds / 78.25 seconds wall**. Formatting and Clippy with
warnings denied passed. `CARGO_BUILD_JOBS` was inherited as **4**; cargo commands ran
one at a time. An earlier diagnostic table run failed only on the two reach
differences recorded above; initial Clippy complaints about a nested conditional
and a redundant closure were fixed before these final checks.

All **52** complete searches exhausted below the 190,000-state and ten-second bounds,
with every configured fault witnessed. They total **26.988 seconds**,
excluding first-refutation searches. The largest has **72,666 unique states**;
the slowest takes **3.811 seconds**. All safety verdicts and reaches match
the executable observed table. Beyond the two reach differences above, there are
no differences from this section's registration. MC6-004 and MC6-005's earlier
differences remain recorded in their original tables. MC6-001 and MC6-002 remain
unchanged. Every first refutation still fails with node data and queues erased.

The new unavailable-observation precondition is reachable, so LIF-010 remains
modelled by L and witness 42; the fallback row of **46 modelled / 12 not modelled**
is not needed. The accounting remains **47 modelled / 11 not modelled** with the
same earlier partial exclusions. PLC-023's machine slice is outside MC6 and is
covered by MC4's P2. No conformance vector, specification, ruling or other model
changed. No additional out-of-scope change is needed beyond the already recorded
owner/MC8 follow-up for the two contradictions and fenced reconciliation.

These final measurements retain all earlier tables. Every row is listed because its
state count, depth, time or registered reach set changed. V and N denote the two
literal contradiction predicates; all unlisted safety predicates hold and unlisted
reaches are absent. Reaches range from 0 to 43.

| Part/configuration/variant | Unique states | Depth | Seconds | Refuted | Reached |
| --- | --- | --- | --- | --- | --- |
| Setup/Base/Correct | 3,261 | 90 | 0.098 | none | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/LossR/Correct | 4,650 | 90 | 0.166 | none | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/ReorderR/Correct | 7,842 | 90 | 0.281 | none | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/LossH/Correct | 3,744 | 90 | 0.129 | none | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/ReorderH/Correct | 5,670 | 90 | 0.173 | none | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/Crash/Correct | 37,962 | 93 | 1.510 | none | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/Timer/Correct | 69,411 | 91 | 3.639 | none | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/Silence/Correct | 12,426 | 90 | 0.393 | none | 0 1 2 3 4 5 6 10 14 24 25 27 36 42 |
| Removal/Base/Correct | 1,442 | 243 | 0.111 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/LossR/Correct | 2,037 | 243 | 0.172 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/ReorderR/Correct | 4,564 | 243 | 0.360 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/LossH/Correct | 1,739 | 243 | 0.143 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/ReorderH/Correct | 2,356 | 243 | 0.204 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/Crash/Correct | 23,623 | 250 | 2.170 | V N | 0 1 2 5 6 7 8 9 10 11 12 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/Timer/Correct | 4,170 | 247 | 0.390 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/Silence/Correct | 2,869 | 243 | 0.238 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 24 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Pool/Base/Correct | 2,967 | 143 | 0.104 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/LossR/Correct | 3,714 | 143 | 0.159 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/ReorderR/Correct | 10,453 | 143 | 0.420 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/LossH/Correct | 3,731 | 143 | 0.167 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/ReorderH/Correct | 5,332 | 143 | 0.208 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/Crash/Correct | 24,534 | 146 | 1.263 | V | 0 1 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/Timer/Correct | 34,131 | 144 | 1.830 | V | 0 1 2 4 18 19 21 22 23 25 28 37 38 40 41 |
| Pool/Silence/Correct | 12,911 | 143 | 0.546 | V | 0 1 2 4 18 19 21 22 23 24 25 28 37 38 40 41 |
| Removal/ShutdownTimer/Correct | 29,078 | 244 | 2.922 | V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 |
| Setup/Base/Regress | 3,261 | 90 | 0.096 | L | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/Base/EarlyPart | 886 | 84 | 0.023 | S | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/Base/RepeatHook | 3,576 | 99 | 0.108 | H A A2 | 0 1 2 3 5 6 10 25 27 36 42 |
| Setup/Base/WrongKey | 3,261 | 90 | 0.100 | K | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/Base/FalseSuccess | 3,261 | 90 | 0.098 | O | 0 1 3 4 5 6 10 25 27 36 42 |
| Setup/Base/Rollback | 3,524 | 84 | 0.102 | L B | 0 1 2 3 5 10 14 25 26 27 36 42 |
| Removal/Base/KeepConduit | 1,442 | 243 | 0.107 | C V | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/Base/AutoReplace | 1,442 | 243 | 0.109 | R V N | 0 1 2 5 6 7 8 9 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/Base/IgnoreTombstone | 1,442 | 243 | 0.108 | T V N | 0 1 2 5 6 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/Base/EarlyRemove | 1,162 | 201 | 0.075 | D V N | 0 1 2 5 6 7 8 9 10 12 14 15 19 20 25 26 31 35 36 37 40 41 42 43 |
| Pool/Base/EarlyPool | 3,009 | 145 | 0.104 | P V | 0 1 18 21 22 25 28 37 38 40 41 |
| Removal/Base/OtherData | 1,442 | 243 | 0.109 | F V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/Base/Revive | 1,444 | 245 | 0.107 | L Q V | 0 1 2 5 6 7 8 9 10 11 13 14 15 17 18 19 20 25 26 27 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Setup/Timer/Unfenced | 72,666 | 94 | 3.811 | A A2 | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Removal/Base/HookPending | 1,442 | 243 | 0.110 | D N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/Base/PreserveReferences | 1,442 | 243 | 0.107 | C V | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/ShutdownTimer/TimeConfirms | 28,929 | 244 | 2.841 | D V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 |
| Removal/Base/CloseStops | 1,445 | 246 | 0.127 | B V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Setup/Base/AdoptTemplate | 3,261 | 90 | 0.099 | F | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Removal/Base/StopBeforeHook | 1,487 | 246 | 0.111 | H O V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 18 19 20 25 26 29 30 31 32 33 35 36 40 41 42 43 |
| Removal/Base/KeepConfirmed | 1,442 | 243 | 0.109 | D V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 41 42 43 |
| Removal/Base/DelayRetirement | 1,442 | 243 | 0.111 | L V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Setup/Base/UnsolicitedRetirement | 3,261 | 90 | 0.100 | L | 0 1 2 3 4 5 6 10 14 25 27 36 42 |
| Setup/Base/WrongHookInstance | 3,156 | 87 | 0.093 | K | 0 1 2 3 5 6 10 14 25 27 36 42 |
| Removal/Base/CurrentTemplateHook | 1,442 | 243 | 0.108 | K V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/Base/UnknownAtDispatch | 1,442 | 243 | 0.109 | D V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |
| Removal/Base/KeepLatestUnknown | 1,442 | 243 | 0.110 | D Q V N | 0 1 2 5 6 7 8 9 10 11 12 13 14 15 17 18 19 20 25 26 29 30 31 32 33 35 36 37 38 39 40 41 42 43 |

Final first-refutation lengths (single-thread BFS):

| Variant | Target | Actions |
| --- | --- | --- |
| Regress | L | 87 |
| EarlyPart | S | 3 |
| RepeatHook | H | 35 |
| WrongKey | K | 8 |
| FalseSuccess | O | 12 |
| Rollback | B | 8 |
| KeepConduit | C | 147 |
| AutoReplace | R | 54 |
| IgnoreTombstone | T | 64 |
| EarlyRemove | D | 92 |
| EarlyPool | P | 64 |
| OtherData | F | 147 |
| Revive | Q | 133 |
| Unfenced | A | 37 |
| HookPending | D | 92 |
| PreserveReferences | C | 147 |
| TimeConfirms | D | 93 |
| CloseStops | B | 50 |
| AdoptTemplate | F | 64 |
| StopBeforeHook | O | 84 |
| KeepConfirmed | D | 230 |
| DelayRetirement | L | 82 |
| UnsolicitedRetirement | L | 83 |
| WrongHookInstance | K | 8 |
| CurrentTemplateHook | K | 84 |
| UnknownAtDispatch | D | 82 |
| KeepLatestUnknown | D | 227 |

## MC7b — records, observation and installing packages: pre-registration

Registered 2026-11-11 before the first run. Work order 113 extends only this experiment;
`records.rs` contains three separate protocols using the existing harness, never engine code.
All safety and protocol reachability predicates read only the external emission ledger.
Snapshots are logged when a record, read result, observation, fetch, decision or use is emitted;
transport delivery is distinct. No predicate infers history from a node's current storage.

### MC7b nodes, scripts and bounds

Each part has three nodes: engine E, second source/runtime/fetch service S, and actor/reader A.
E's records, invocation index, declarations, observation cache, provider registry, pending
installs and installed packages are saved state. S keeps its records at their source in saved
state too. Crash/recovery pauses E and erases no saved state. The network and ledger survive.

**Records.** E and S independently emit batches covering all fourteen event categories in
REC-001 (provider call start/end, object register/unregister/remove and enrollment open/close/
expire are separate categories). Batch emission is atomic within each source, with no order
between sources. A makes one plain keyed effect request, then sends a non-root and a root
purge together, then a second root purge, repeats the original key, and reads both sources.
The first purge uses `before=3`, exercising equality and later records; the second uses
`before=30`, exercising preservation of earlier purge records. Root purge broadcasts to the
two sources; it is not a distributed atomic deletion. Purge records themselves never expire.
An explicit storage-check step exercises alteration separately from purging. Unknown starts
with an effect whose reply is unknown; that plain repeat must not dispatch. Other parts start
with a known succeeded invocation, with no expiring output. Purging does not change either.
Each source sends two frames with the expected surviving record IDs and disjoint records.
The reader assembles and orders each source separately; it imposes no order across sources.
Unreadable nondeterministically withholds one record from S's frames, without deleting it.
Read loss can withhold a frame. At transport quiescence the reader completes the read, reporting
exact missing IDs. An intentionally purged record is absent from the expected IDs and is no gap.
This is a bounded completion step, not a claim about eventual delivery or failure detection.

**Observation.** E observes one remote instance successfully, another remote instance has never
been observed, and a local instance has a previous successful observation. The runtime changes
its actual run state; E receives drift, A inspects, then the remote machine may be reported
unreachable. Both remote instances then report unknown, retaining respectively a last-success
time and no time; the local condition stays as before. The runtime may restart its own run,
or the machine returns; a later observation may remove drift without any engine repair call.
A final inspection closes the script. Declarations carry independent definition, configuration
and revision values; effect, access and lifecycle snapshots are separate from observed conditions.
Observing may update conditions; it changes no declared value, lifecycle state or Conduit.
Crash is enabled after drift and before the unreachable report, so saved last-success time is
used after recovery. No setup, control, open or retirement is requested by this script.

**Installs.** A submits two system-owned claimants of one fixed scheme, and a non-system Resource
claiming a different scheme. Scheme lists do not change. Two requests for one address/version
then race, with different digests (or the same digest in Same). Decide and fetch completion are
separate transitions: each first request can see no installed package and start fetching. A
completion uses its previously made decision, with no invented commit-time arbitration. Both
first installs may succeed with different digests; the last completion then occupies the slot.
After quiescence A reads that slot to construct same-digest authorized, refused and invalid
requests, plus a different-digest request. A separate address exercises a fetch that may fail
or return bytes with a different digest. Fetch service actions choose good/bad/failed results;
bad and failed choices each have a one-use local budget in their own configurations. Package
bytes and digests are finite symbols, not a cryptographic implementation. Provider mode claims
`https`, testing that it overrides built-in fetching; Https and File have only an unrelated
provider, so E fetches itself, represented by S's archive-service abstraction. All fetch messages
carry the request's address, version, digest and fetch scheme. A check event must precede use.
An install is judged against the installed slot at decision time; failed-fetch noninterference
compares immediately before/after that failure, not against a request-start snapshot that another
install may legitimately change. A timer in FetchLoss can end a stranded fetch unavailable only
at transport quiescence, with no archive work remaining at S. No eventual response is asserted.

The maximum is 190,000 unique states and 9 seconds per full/refutation run, with two checker
threads for full runs and one for first refutations. Queues hold at most 16 messages, including
the initial competing requests and both record frames. There are two sources, fourteen initial
record events, one keyed effect, three purge requests, one repeat and one read per source;
observation has two remote instances and one local instance; installs have two addresses,
one version, four digest symbols (three requested and one mismatched), three provider
registrations and seven requests.
Scripts and local choice budgets are finite; no retries, wire copies or unbounded clocks occur.

### MC7b catalogue accounting (20 promises)

Each target promise occurs once in this accounting. “Modelled” denotes the stated bounded
projection and property, not every static field or all possible invocations of the promise.
Supporting promises are excluded from these counts.

| Promise | Account | Property / reason |
| --- | --- | --- |
| REC-001 | modelled | R1: every listed event category emits a corresponding retained record at creation. |
| REC-002 | not modelled | A static decision, to be shown by vectors: fields and exclusion of credentials. |
| REC-003 | modelled | R3: every existing record is unchanged except during purge. |
| REC-004 | modelled | R4: root-only strict-before purge, permanent purge evidence, saved keyed outcome/effect unaffected. |
| REC-005 | modelled | R5: per-source read order, with no inter-source order requirement. |
| REC-006 | modelled | R6: a completed incomplete read identifies the exact gap. |
| OBS-001 | modelled | O1: definition, configuration and revision unchanged by observations. |
| OBS-002 | not modelled | A static decision, to be shown by vectors: observation fields. |
| OBS-003 | modelled | O3: all remote conditions unknown with last-success time or none; other node unchanged. |
| OBS-004 | modelled | O4: drift dispatches no setup, control, open or retirement. |
| OBS-006 | modelled | O6: inspection/observation changes no effect, access or lifecycle state. |
| PKG-023 | not modelled | A static decision, to be shown by vectors: invalid request/archive forms. Invalid-before-repeat is supporting input to I27. |
| PKG-024 | modelled | I24: failed fetch returns unavailable, dispatches no install, changes no installed package. |
| PKG-025 | modelled | I25: valid authorized different-digest installed decision conflicts with no fetch/change. |
| PKG-027 | modelled | I27: same-digest decision observes refusal/invalid priority, otherwise succeeds with no fetch/change. |
| PKG-029 | not modelled | A static decision, to be shown by vectors: submitted-project address collision. |
| PRV-013 | modelled | I13: package-source uniqueness by scheme and system ownership; machine-provider half outside this model. |
| PRV-017 | modelled | I17: selected source and fetch arguments, digest checked before archive use on provider/https/file paths. |
| NAM-023 | not modelled | A static decision, to be shown by vectors: target contract version. |
| NAM-026 | not modelled | A static decision, to be shown by vectors: operation absent from target capability. |

Totals: **14 modelled; 6 not modelled, all static decisions to be shown by vectors**.
Supporting: PKG-026 (digest), PRV-010 (runtime restart), PRV-016 (fixed schemes), PLC-023
(unreachable node), ACC-032/ACC-033 (root and install authority), OPS-008/OPS-012 (saved
index and plain unknown repeat), IFC-025 (diagnostic identity), and OPS-023 (decision priority).
Pending actions are outside MC7b and remain MC9's work after batch 2.

### MC7b property text and preconditions

The following quotes retain `must not` polarity. Each applies to the stated finite script;
its named witness below reaches its precondition independently of the implementation's verdict.

**R1 (REC-001, must).** When: “Any of these happens: an access decision; a request ends without being dispatched; an invocation is dispatched; an invocation ends; a provider call is made, or ends; an instance's lifecycle state changes; an object is registered, unregistered or removed; an object's revision changes; an enrollment opens, closes or expires (ENR-001, ENR-007 to ENR-009, PRV-013).” Then: “A record of it is kept.”

**R3 (REC-003, must not).** When: “A record exists.” Then: “It is removed or altered by anything other than a purge.”

**R4 (REC-004, must).** When: “An actor asks to purge records.” Then: “The request is dispatched only if the actor is the root Interface. A purge removes the records made before a time that the request names, and a record is kept stating which records were purged and when.”

**R5 (REC-005, must).** When: “Records made at one source are read.” Then: “They are in the order in which they were made at that source. No order is promised between records from different sources.”

**R6 (REC-006, must).** When: “Records that should exist cannot be read, or were lost.” Then: “The result of reading them says that there is a gap, and where.”

**O1 (OBS-001, must not).** When: “Something is observed about an instance.” Then: “The instance's declared definition, configuration or revision changes because of it.”

**O3 (OBS-003, must).** When: “Something about an instance cannot be observed now, for example because the machine provider reports its machine unreachable, under PLC-023.” Then: “It is reported as unknown, as a condition whose `status` is `Unknown`, with the time of the last successful observation of it in `lastHeartbeatTime`.”

**O4 (OBS-004, must not).** When: “What is observed of an instance differs from what is declared for it.” Then: “Anything acts to remove the difference without an explicit request.”

**O6 (OBS-006, must not).** When: “An instance is inspected or observed.” Then: “An effect occurs, or the actor that inspects it gains access to anything.”

**I24 (PKG-024, must).** When: “An actor asks to install a package, and its archive cannot be fetched now.” Then: “The request is not dispatched, and reports unavailable.”

**I25 (PKG-025, must).** When: “An actor asks to install a package, and a package is already installed at that address and version with another digest.” Then: “The request is not dispatched, and reports conflict. It is decided from what is installed: nothing is fetched for an address and version that is already installed.”

**I27 (PKG-027, must).** When: “An actor asks to install a package that is already installed at that address and version with the same digest.” Then: “The request is decided like any other, changes nothing and has the outcome succeeded. It is decided from what is installed: nothing is fetched for an address and version that is already installed.”

**I13 (PRV-013, must).** When: “A Resource would be instantiated whose type offers `machine-provider.v1` or `package-source.v1`, and its owner is not `system`; or, for a machine provider, another machine provider exists that is not retired; or, for a package-source provider, another that is not retired claims one of the schemes that its configuration lists under `schemes`.” Then: “It is not instantiated: at any time at most one machine provider, and for each URI scheme at most one package-source provider, is not retired, and each is owned by `system`.”

**I17 (PRV-017, must).** When: “An actor asks to install a package that is not installed, and the archive is to be fetched from the request's `fetch`.” Then: “Where a package-source provider that is not retired claims the scheme of the `fetch`, the engine fetches the archive with that provider's `fetch-package.v1`, giving the package's `address`, `version`, `digest` and `fetch`, and takes the archive from its output `archive`. Where none claims it and the scheme is `https` or `file`, the engine fetches the archive itself. Either way the archive is checked against the digest under PKG-026.”


R1's witness `events` covers all fourteen categories. R3 uses `existing`; R4 uses `purges`
(root and non-root), `repeat`, and `unknown` in Unknown. R5 uses `reads`. R6 uses `missing`,
where the model or transport withholds a record that was not purged. O1/O6 use `observed` and
`inspected`; O3 uses `unreachable` (both prior-success and never-observed instances); O4 uses
`drift`. `returned` reaches removal of drift by the environment. I24 uses `failed`; I25 uses
`conflict`; I27 uses `same`, `denied-same` and `invalid-same`; I13 uses `claimants` (including
the foreign owner); I17 uses `fetch`, `used` and `bad`. `race` reaches both first decisions
before either installs; `different-firsts` reaches both succeeding with different digests.
`done` is a quiescent script endpoint, not liveness. Ledger facts describing stimuli, including
bad archives and absent records, are independent of whether the correct response was produced.

### MC7b configurations and fault windows

Every enabled harness budget is one, split across configurations; every positive budget has
its harness witness checked in that run. Reorder witnesses require two real frames/requests
on the same directed link. `fault_allowed` opts in with `gate_remaining_faults=true`, including
recovery and timers. `fault_enabled` restricts state windows; `timer_enabled` restricts timers.

| Records configuration | Enabled fault / model choice | R witnesses expected |
| --- | --- | --- |
| Base | none | events existing purges repeat reads |
| Unknown | first keyed outcome unknown | Base + unknown |
| RequestLoss | loss A→E during first purge pair | Base |
| RequestReorder | reorder A→E during first purge pair | Base |
| ReadLoss | loss S→A while reading | Base + missing |
| ReadReorder | reorder S→A while reading | Base |
| Crash | E crash/recovery after first invocation, before repeat | Base |
| Unreadable | withhold one record from S, at most once | Base + missing |

| Observation configuration | Enabled fault / model choice | O witnesses expected |
| --- | --- | --- |
| Base | machine may be unreachable; provider restart/return | observed inspected drift unreachable returned |
| Crash | E crash/recovery after drift, before unreachable report | all Base witnesses |

| Install configuration | Enabled fault / model choice | I witnesses expected |
| --- | --- | --- |
| Provider | provider claims https | race conflict same denied-same invalid-same claimants fetch used different-firsts done |
| Https | E fetches unclaimed https | Provider witnesses |
| File | E fetches unclaimed file | Provider witnesses |
| Same | same-digest race, provider path | Provider except different-firsts |
| FailProvider | provider may fail probe fetch once | Provider + failed |
| FailHttps | E https may fail probe fetch once | Provider + failed |
| FailFile | E file may fail probe fetch once | Provider + failed |
| BadProvider | provider may return mismatched probe digest once | Provider + bad |
| BadHttps | E https may return mismatched probe digest once | Provider + bad |
| BadFile | E file may return mismatched probe digest once | Provider + bad |
| RequestLoss | loss A→E on initial install pair | Provider witnesses |
| RequestReorder | reorder A→E on initial install pair | Provider witnesses |
| FetchLoss | loss E↔S on racing fetches; quiescent E timer | Provider + failed |
| FetchReorder | reorder E↔S on racing fetches | Provider witnesses |
| Crash | E crash/recovery with first fetch pending | Provider witnesses |

All other budgets are zero: duplication is outside this order; cut/heal and permanent lost
machines are outside its faults; timers are unnecessary except FetchLoss's stranded-fetch
completion; non-Crash parts isolate other faults. Record read failure and machine unreachability
are local choices rather than saved-record loss or permanent node loss. Zero local bad/fail/read
budgets isolate the corresponding choice in its own part. Recovery is unrestricted once E is
crashed; request/reading/fetch windows never gate it out.

### MC7b expected verdicts, fixed before execution

H means holds; F means refuted. Safety order for records is R1 R3 R4 R5 R6, for observation
O1 O3 O4 O6, and for installs I24 I25 I27 I13 I17. Only that part's properties are registered.
Every row's reachability is exactly its configuration's row above; witnesses absent there are
expected not reached. Enabled harness witnesses are always expected reached. This rule specifies
every safety and reach verdict, including the mutants, without assuming mutant reach is safety.

| Variant/configuration | Expected safety |
| --- | --- |
| Correct / every Records configuration | HHHHH |
| OmitRecord / Base | FHHHH |
| AlterRecord / Base | HFHHH |
| PurgeIndex / Base | HHFHH |
| RootBypass / Base | HHFHH |
| PurgeInclusive / Base | HHFHH |
| ForgetIndex / Crash | HHFHH |
| ReverseRead / Base | HHHFH |
| HideGap / Unreadable | HHHHF |
| Correct / every Observation configuration | HHHH |
| AdoptObserved / Base | FHHH |
| StaleCurrent / Base | HFHH |
| NoLastTime / Base | HFHH |
| ForgetTime / Crash | HFHH |
| AutoRepair / Base | HHFH |
| InspectEffect / Base | HHHF |
| InspectAccess / Base | HHHF |
| Correct / every Install configuration | HHHHH |
| FailedAsSuccess / FailProvider | FHHHH |
| FetchConflict / Provider | HFHHH |
| FetchSame / Provider | HHFHH |
| SkipAuthorization / Provider | HHFHH |
| DoubleClaim / Provider | HHHFH |
| ForeignOwner / Provider | HHHFH |
| WrongRoute / Provider | HHHHF |
| Unchecked / BadProvider | HHHHF |
| Unchecked / BadHttps | HHHHF |
| Unchecked / BadFile | HHHHF |

Every full run is tested against this table. One additional single-thread BFS per variant
prints its first target refutation (Unchecked is exercised on all three paths). Traces are
replayed after replacing all node data to demonstrate that their failing predicate uses only
the ledger. Any observed mismatch will be added beside this unchanged table.

### MC7b findings registered before execution

**MC7b-001 — ambiguity, REC-003/REC-006.** Nothing except a purge may remove/alter a record,
yet the gap promise includes records that were lost. Projection: record at S → unreadable
storage → read with a gap. This model retains that record and faults only reading it, which
satisfies both. Whether actual storage loss is an admitted fault or violates REC-003 is not
settled by these words. The ambiguity is reported, not resolved by adding a loss permission.

**MC7b-002 — possible gap, PKG-025/PKG-027, with PKG-024/PRV-017.** Two first installs of one
address/version can both decide before either installs. No promise orders them or states a
commit-time recheck. The registered model retains both decisions and permits different digests
to complete successfully. `different-firsts` searches for that witness; no extra aggregate
"one first digest wins" safety promise is invented. Results will state whether it was reached.

### MC7b review additions, before their first run

The initial 50 full runs matched the original table; the seven MC7b tests took 0.13 seconds.
Before the next run R4's purge evidence is made explicit: it records the cutoff and exact IDs
removed, not just a purge flag. R3 also checks continuity between successive source snapshots,
so a silent storage change across a crash cannot disappear between logged mutations. These
strengthen the written projections without changing any expected verdict or scenario window.
A separate first-witness BFS for `different-firsts` is registered to print the shortest race
witness; it adds no safety claim. Its expected verdict is reached in Correct/Provider. The
harness receives only a new `assert_reached` helper, using its existing bounds and lock.


### MC7b dated results — 2026-11-11

All **50 full runs matched the original expected table**, including every positive fault
budget's witness and every registered property precondition. There are **no differences**
from the table, no truncation, and no correct-model safety refutation. All 25 seeded runs
refuted exactly their named property; each has a separate first-refutation trace below.
There are 23 distinct broken variants, with Unchecked run on three fetch paths.

The full crate passed **47 tests (39 existing, 8 MC7b)** plus zero doc-tests. Release took
**50.89 seconds wall**, including compilation, with 41.31 seconds reported for tests.
The required single-thread test-runner count pass took **39.32 seconds wall**, with 39.06
seconds reported for tests. Model-checker full runs still use two threads; first traces use
one. These times include all earlier models, not just this order. Every MC7b full run was
well below its 190,000-state/9-second limits. Elapsed entries below are rounded to milliseconds;
0.000 is a measured value rounded down, not an unrun check. The initial seven-test MC7b-only
pass (before the added race-witness test) took 0.13 seconds of test time.

Safety/reach orders are the registered orders above; `+` is reached, `-` not reached. The
harness witnesses are all reached in every row with their positive budgets, and are omitted
from the reach strings. Static decisions are not safety columns. Counts remain **14 modelled,
6 static decisions for vectors**, with supporting promises excluded and no new vectors.

**Records.**

| Variant/configuration | Unique states | Depth | Seconds | Safety | Reach |
| --- | ---: | ---: | ---: | --- | --- |
| Correct/Base | 63 | 24 | 0.002 | HHHHH | +++++-- |
| Correct/Unknown | 63 | 24 | 0.002 | HHHHH | +++++-+ |
| Correct/RequestLoss | 161 | 24 | 0.004 | HHHHH | +++++-- |
| Correct/RequestReorder | 161 | 24 | 0.004 | HHHHH | +++++-- |
| Correct/ReadLoss | 91 | 24 | 0.003 | HHHHH | ++++++- |
| Correct/ReadReorder | 77 | 24 | 0.002 | HHHHH | +++++-- |
| Correct/Crash | 121 | 26 | 0.004 | HHHHH | +++++-- |
| Correct/Unreadable | 83 | 24 | 0.003 | HHHHH | ++++++- |
| OmitRecord/Base | 63 | 24 | 0.002 | FHHHH | +++++-- |
| AlterRecord/Base | 63 | 24 | 0.002 | HFHHH | +++++-- |
| PurgeIndex/Base | 63 | 24 | 0.002 | HHFHH | +++++-- |
| RootBypass/Base | 115 | 25 | 0.003 | HHFHH | +++++-- |
| PurgeInclusive/Base | 63 | 24 | 0.002 | HHFHH | +++++-- |
| ForgetIndex/Crash | 121 | 26 | 0.004 | HHFHH | +++++-- |
| ReverseRead/Base | 63 | 24 | 0.002 | HHHFH | +++++-- |
| HideGap/Unreadable | 83 | 24 | 0.003 | HHHHF | ++++++- |

**Observation.**

| Variant/configuration | Unique states | Depth | Seconds | Safety | Reach |
| --- | ---: | ---: | ---: | --- | --- |
| Correct/Base | 26 | 12 | 0.000 | HHHH | +++++ |
| Correct/Crash | 51 | 14 | 0.000 | HHHH | +++++ |
| AdoptObserved/Base | 26 | 12 | 0.000 | FHHH | +++++ |
| StaleCurrent/Base | 26 | 12 | 0.000 | HFHH | +++++ |
| NoLastTime/Base | 26 | 12 | 0.000 | HFHH | +++++ |
| ForgetTime/Crash | 51 | 14 | 0.000 | HFHH | +++++ |
| AutoRepair/Base | 26 | 12 | 0.000 | HHFH | +++++ |
| InspectEffect/Base | 26 | 12 | 0.000 | HHHF | +++++ |
| InspectAccess/Base | 26 | 12 | 0.000 | HHHF | +++++ |

**Installs.**

| Variant/configuration | Unique states | Depth | Seconds | Safety | Reach |
| --- | ---: | ---: | ---: | --- | --- |
| Correct/Provider | 55 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/Https | 55 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/File | 55 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/Same | 55 | 23 | 0.001 | HHHHH | ++++++++-+-- |
| Correct/FailProvider | 64 | 23 | 0.001 | HHHHH | +++++++++++- |
| Correct/FailHttps | 64 | 23 | 0.001 | HHHHH | +++++++++++- |
| Correct/FailFile | 64 | 23 | 0.001 | HHHHH | +++++++++++- |
| Correct/BadProvider | 64 | 23 | 0.001 | HHHHH | ++++++++++-+ |
| Correct/BadHttps | 64 | 23 | 0.001 | HHHHH | ++++++++++-+ |
| Correct/BadFile | 64 | 23 | 0.001 | HHHHH | ++++++++++-+ |
| Correct/RequestLoss | 85 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/RequestReorder | 105 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/FetchLoss | 142 | 24 | 0.001 | HHHHH | +++++++++++- |
| Correct/FetchReorder | 113 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/Crash | 121 | 25 | 0.001 | HHHHH | ++++++++++-- |
| FailedAsSuccess/FailProvider | 64 | 23 | 0.001 | FHHHH | +++++++++++- |
| FetchConflict/Provider | 68 | 24 | 0.001 | HFHHH | ++++++++++-- |
| FetchSame/Provider | 70 | 24 | 0.001 | HHFHH | ++++++++++-- |
| SkipAuthorization/Provider | 55 | 23 | 0.001 | HHFHH | ++++++++++-- |
| DoubleClaim/Provider | 55 | 23 | 0.001 | HHHFH | ++++++++++-- |
| ForeignOwner/Provider | 55 | 23 | 0.001 | HHHFH | ++++++++++-- |
| WrongRoute/Provider | 55 | 23 | 0.001 | HHHHF | ++++++++++-- |
| Unchecked/BadProvider | 64 | 23 | 0.001 | HHHHF | ++++++++++-+ |
| Unchecked/BadHttps | 64 | 23 | 0.001 | HHHHF | ++++++++++-+ |
| Unchecked/BadFile | 64 | 23 | 0.001 | HHHHF | ++++++++++-+ |


### MC7b findings and shortest race witness

**MC7b-001 remains an ambiguity**, not a discovered correct-model violation. The concrete
Unreadable script keeps S's purge record, withholds it from the read, and returns its ID as
a gap. The record remains saved. If “were lost” instead admits deleting that saved record
without a purge, it directly violates the other promise's prohibition. The model applies
the registered read-failure reading and makes no ruling about actual storage loss.

**MC7b-002 is reached.** This is a gap in ordering simultaneous first installs, not a refutation
of the individual decision-time properties. The first BFS witness has 12 actions: register
one system claimant and reject the other two Resources; send the two installs; decide both
while the slot is empty; fetch and verify both; complete each succeeded, with different
digests. The second completion replaces the first slot. Neither a commit-time recheck nor a
winner is supplied by these promises. All correct safety predicates still hold. The shortest
concrete trace is:

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
3. delivered actor/reader -> engine/source-E [position 0]
4. actor/reader Next
5. delivered actor/reader -> engine/source-E [position 0]
6. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
7. source-S/runtime/archive-service Fetch(0, Good)
8. delivered actor/reader -> engine/source-E [position 0]
9. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
10. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
11. source-S/runtime/archive-service Fetch(1, Good)
12. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
```

### MC7b first single-thread refutations

These are first BFS traces from separate bounded runs, never full-run discovery replacements.
`Next` follows the registered actor script; numbered delivery positions distinguish frames
and requests. Each trace was replayed with every node's data erased and its target predicate
remained false, demonstrating that the check reads only the retained ledger.

**installs FailedAsSuccess/FailProvider: MC7b I24 (18 actions).**

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
3. delivered actor/reader -> engine/source-E [position 0]
4. actor/reader Next
5. delivered actor/reader -> engine/source-E [position 0]
6. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
7. source-S/runtime/archive-service Fetch(0, Good)
8. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
9. delivered actor/reader -> engine/source-E [position 0]
10. actor/reader Next
11. delivered actor/reader -> engine/source-E [position 0]
12. delivered actor/reader -> engine/source-E [position 0]
13. delivered actor/reader -> engine/source-E [position 0]
14. delivered actor/reader -> engine/source-E [position 0]
15. delivered actor/reader -> engine/source-E [position 0]
16. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
17. source-S/runtime/archive-service Fetch(6, Failed)
18. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
```

**installs FetchConflict/Provider: MC7b I25 (9 actions).**

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
3. delivered actor/reader -> engine/source-E [position 0]
4. actor/reader Next
5. delivered actor/reader -> engine/source-E [position 0]
6. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
7. source-S/runtime/archive-service Fetch(0, Good)
8. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
9. delivered actor/reader -> engine/source-E [position 0]
```

**installs FetchSame/Provider: MC7b I27 (11 actions).**

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
3. delivered actor/reader -> engine/source-E [position 0]
4. actor/reader Next
5. delivered actor/reader -> engine/source-E [position 0]
6. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
7. source-S/runtime/archive-service Fetch(0, Good)
8. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
9. delivered actor/reader -> engine/source-E [position 0]
10. actor/reader Next
11. delivered actor/reader -> engine/source-E [position 0]
```

**installs SkipAuthorization/Provider: MC7b I27 (13 actions).**

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
3. delivered actor/reader -> engine/source-E [position 0]
4. actor/reader Next
5. delivered actor/reader -> engine/source-E [position 0]
6. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
7. source-S/runtime/archive-service Fetch(0, Good)
8. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
9. delivered actor/reader -> engine/source-E [position 0]
10. actor/reader Next
11. delivered actor/reader -> engine/source-E [position 0]
12. delivered actor/reader -> engine/source-E [position 0]
13. delivered actor/reader -> engine/source-E [position 0]
```

**installs DoubleClaim/Provider: MC7b I13 (2 actions).**

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
```

**installs ForeignOwner/Provider: MC7b I13 (3 actions).**

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
3. delivered actor/reader -> engine/source-E [position 0]
```

**installs WrongRoute/Provider: MC7b I17 (5 actions).**

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
3. delivered actor/reader -> engine/source-E [position 0]
4. actor/reader Next
5. delivered actor/reader -> engine/source-E [position 0]
```

**installs Unchecked/BadProvider: MC7b I17 (8 actions).**

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
3. delivered actor/reader -> engine/source-E [position 0]
4. actor/reader Next
5. delivered actor/reader -> engine/source-E [position 0]
6. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
7. source-S/runtime/archive-service Fetch(0, Good)
8. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
```

**installs Unchecked/BadHttps: MC7b I17 (8 actions).**

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
3. delivered actor/reader -> engine/source-E [position 0]
4. actor/reader Next
5. delivered actor/reader -> engine/source-E [position 0]
6. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
7. source-S/runtime/archive-service Fetch(0, Good)
8. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
```

**installs Unchecked/BadFile: MC7b I17 (8 actions).**

```text
1. delivered actor/reader -> engine/source-E [position 0]
2. delivered actor/reader -> engine/source-E [position 0]
3. delivered actor/reader -> engine/source-E [position 0]
4. actor/reader Next
5. delivered actor/reader -> engine/source-E [position 0]
6. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
7. source-S/runtime/archive-service Fetch(0, Good)
8. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
```

**observation AdoptObserved: MC7b O1 (3 actions).**

```text
1. engine/source-E Initial
2. source-S/runtime/archive-service Drift
3. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
```

**observation StaleCurrent: MC7b O3 (7 actions).**

```text
1. engine/source-E Initial
2. source-S/runtime/archive-service Drift
3. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
4. actor/reader Inspect
5. delivered actor/reader -> engine/source-E [position 0]
6. source-S/runtime/archive-service Unreachable(true)
7. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
```

**observation NoLastTime: MC7b O3 (7 actions).**

```text
1. engine/source-E Initial
2. source-S/runtime/archive-service Drift
3. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
4. actor/reader Inspect
5. delivered actor/reader -> engine/source-E [position 0]
6. source-S/runtime/archive-service Unreachable(true)
7. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
```

**observation ForgetTime: MC7b O3 (9 actions).**

```text
1. engine/source-E Initial
2. source-S/runtime/archive-service Drift
3. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
4. crashed engine/source-E
5. recovered engine/source-E
6. actor/reader Inspect
7. delivered actor/reader -> engine/source-E [position 0]
8. source-S/runtime/archive-service Unreachable(true)
9. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
```

**observation AutoRepair: MC7b O4 (3 actions).**

```text
1. engine/source-E Initial
2. source-S/runtime/archive-service Drift
3. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
```

**observation InspectEffect: MC7b O6 (5 actions).**

```text
1. engine/source-E Initial
2. source-S/runtime/archive-service Drift
3. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
4. actor/reader Inspect
5. delivered actor/reader -> engine/source-E [position 0]
```

**observation InspectAccess: MC7b O6 (5 actions).**

```text
1. engine/source-E Initial
2. source-S/runtime/archive-service Drift
3. delivered source-S/runtime/archive-service -> engine/source-E [position 0]
4. actor/reader Inspect
5. delivered actor/reader -> engine/source-E [position 0]
```

**records OmitRecord: MC7b R1 (1 actions).**

```text
1. source-S/runtime/archive-service Events
```

**records AlterRecord: MC7b R3 (5 actions).**

```text
1. engine/source-E Events
2. source-S/runtime/archive-service Events
3. actor/reader Next
4. delivered actor/reader -> engine/source-E [position 0]
5. actor/reader Next
```

**records PurgeIndex: MC7b R4 (8 actions).**

```text
1. engine/source-E Events
2. source-S/runtime/archive-service Events
3. actor/reader Next
4. delivered actor/reader -> engine/source-E [position 0]
5. actor/reader Next
6. actor/reader Next
7. delivered actor/reader -> engine/source-E [position 0]
8. delivered actor/reader -> engine/source-E [position 0]
```

**records RootBypass: MC7b R4 (7 actions).**

```text
1. engine/source-E Events
2. source-S/runtime/archive-service Events
3. actor/reader Next
4. delivered actor/reader -> engine/source-E [position 0]
5. actor/reader Next
6. actor/reader Next
7. delivered actor/reader -> engine/source-E [position 0]
```

**records PurgeInclusive: MC7b R4 (8 actions).**

```text
1. engine/source-E Events
2. source-S/runtime/archive-service Events
3. actor/reader Next
4. delivered actor/reader -> engine/source-E [position 0]
5. actor/reader Next
6. actor/reader Next
7. delivered actor/reader -> engine/source-E [position 0]
8. delivered actor/reader -> engine/source-E [position 0]
```

**records ForgetIndex: MC7b R4 (16 actions).**

```text
1. engine/source-E Events
2. source-S/runtime/archive-service Events
3. actor/reader Next
4. delivered actor/reader -> engine/source-E [position 0]
5. actor/reader Next
6. crashed engine/source-E
7. recovered engine/source-E
8. actor/reader Next
9. delivered actor/reader -> engine/source-E [position 0]
10. delivered actor/reader -> engine/source-E [position 0]
11. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
12. actor/reader Next
13. delivered actor/reader -> engine/source-E [position 0]
14. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
15. actor/reader Next
16. delivered actor/reader -> engine/source-E [position 0]
```

**records ReverseRead: MC7b R5 (23 actions).**

```text
1. engine/source-E Events
2. source-S/runtime/archive-service Events
3. actor/reader Next
4. delivered actor/reader -> engine/source-E [position 0]
5. actor/reader Next
6. actor/reader Next
7. delivered actor/reader -> engine/source-E [position 0]
8. delivered actor/reader -> engine/source-E [position 0]
9. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
10. actor/reader Next
11. delivered actor/reader -> engine/source-E [position 0]
12. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
13. actor/reader Next
14. delivered actor/reader -> engine/source-E [position 0]
15. actor/reader Next
16. delivered actor/reader -> engine/source-E [position 0]
17. delivered engine/source-E -> actor/reader [position 0]
18. delivered engine/source-E -> actor/reader [position 0]
19. delivered actor/reader -> engine/source-E [position 0]
20. source-S/runtime/archive-service Read(false)
21. delivered source-S/runtime/archive-service -> actor/reader [position 0]
22. delivered source-S/runtime/archive-service -> actor/reader [position 0]
23. actor/reader Finish
```

**records HideGap: MC7b R6 (23 actions).**

```text
1. engine/source-E Events
2. source-S/runtime/archive-service Events
3. actor/reader Next
4. delivered actor/reader -> engine/source-E [position 0]
5. actor/reader Next
6. actor/reader Next
7. delivered actor/reader -> engine/source-E [position 0]
8. delivered actor/reader -> engine/source-E [position 0]
9. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
10. actor/reader Next
11. delivered actor/reader -> engine/source-E [position 0]
12. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
13. actor/reader Next
14. delivered actor/reader -> engine/source-E [position 0]
15. actor/reader Next
16. delivered actor/reader -> engine/source-E [position 0]
17. delivered engine/source-E -> actor/reader [position 0]
18. delivered engine/source-E -> actor/reader [position 0]
19. delivered actor/reader -> engine/source-E [position 0]
20. source-S/runtime/archive-service Read(true)
21. delivered source-S/runtime/archive-service -> actor/reader [position 0]
22. delivered source-S/runtime/archive-service -> actor/reader [position 0]
23. actor/reader Finish
```


### What MC7b establishes and leaves open

These finite projections can keep the registered record, observation and install rules
together, and their seeded violations are detected. This is evidence about independent
reference models, not about the engine, cryptography, archive parsing, schema validation or
unbounded executions. No fairness, eventual recovery, or eventual completion is asserted.
Fault configurations do not combine different non-recovery harness faults. Record request
faults cover the first purge pair, record transport faults the two source-S frames; purge
broadcast loss, lost read commands and simultaneous source crashes are outside these bounds.
Observation crashes cover the drift-to-unreachable interval. Install request/fetch faults
cover the racing pair, with the suffix performed after quiescence. No stale-output lifetime,
full invocation-state schema, provider retirement, machine-provider uniqueness or arbitrary
multi-version installation is claimed. Field/version/schema decisions remain for vectors.
The coordinator must disposition the two findings and feed the promise accounting to MC8;
those documents, the specification and rulings are outside this order's scope.

### MC7b admin-review correction: pre-registration

Registered 2026-11-11 **before any run of this correction**. The verifier found that R1
only checked booleans emitted by `keep`, so a missing call to `keep` was invisible. The
correct model's non-root purge refusal already omitted that call. The earlier results
above describe that implementation and do not establish occurrence coverage for REC-001.

Required occurrences now enter `ledger.events` independently of record creation. Each is
assigned a source-local ID, time and category before the record-writing helper is called;
an omitted call still consumes its ID. R1 correlates every occurrence with the exact record
in a saved-storage snapshot in the emission ledger, including source and ID. It checks
every state prefix, so an earlier record of the same category cannot stand in for a later
occurrence. Historical snapshots allow intentional later purges without erasing evidence
that the record was kept. Purge evidence (category 14) remains checked by R4, separately
from REC-001's fourteen categories. R4's permanent-evidence check reads the same storage
snapshots directly. No property reads node state.

A refused non-root purge now logs its request-end occurrence (category 1) and saves its
record. It is a refusal record, separate from the evidence of a dispatched root purge.
`OmitRefusalRecord` logs that occurrence but skips `keep` entirely; the existing `OmitRecord`
likewise skips `keep` for its synthetic occurrence. A concrete regression exercises the
non-root refusal both before any synthetic batch and after both batches. It checks R1 with
all node data erased, so neither category witnesses nor current storage can hide the omission.

Nodes, scripts, fault windows and budgets, properties and reachability orders, bounds and
promise counts remain as registered above. The existing expected table stays unchanged;
the new variant has this additional registered row, with the Base reach order unchanged:

| Added variant/configuration | Expected R1 R3 R4 R5 R6 | Expected reach |
| --- | --- | --- |
| OmitRefusalRecord/Base | FHHHH | +++++-- |

All 51 full runs must match their registered safety, reachability and fault witnesses.
The added variant also gets its own first-refutation BFS and ledger-only replay. Any
disagreement will be recorded beside these unchanged expectations.


### MC7b admin-review results — 2026-11-11

The must finding is fixed: required occurrences and saved records are separate ledger
entries, and a non-root refusal is recorded. The new `OmitRefusalRecord` skips the writing
helper and refutes R1 in seven actions even though the synthetic batch already includes a
request-end category. The direct regression covers both empty storage and completed batches;
it checks the predicate again with all node data erased.

All **51 full runs match the unchanged original expectations plus the new registered row**.
There are no safety, reachability or fault-witness differences and no correct-model safety
refutation. All 26 seeded runs refute exactly their named property (24 distinct variants,
with Unchecked on three paths). Every positive fault budget is reached. The previous two
specification findings, MC7b-001 and MC7b-002, remain unchanged; this correction is a model
and verifier defect, not a new finding in the specification.

The crate passed **48 tests (39 existing, 9 MC7b)** and zero doc-tests in both required runs.
`cargo test --release` took **51.10 seconds wall**, including 9.14 seconds compilation and
41.74 seconds test time. The single-thread test-runner count pass took **41.43 seconds wall**,
with 41.18 seconds test time. No timing-sensitive test failed or needed a reduced-thread
rerun. Full model checks use two checker threads, first refutations one. All other required
checks passed; both grep scans found no matches (grep status 1).

The tables below supersede the historical measurements above. Safety and reach orders are
unchanged; every positive harness fault witness is reached. Seconds are measured and rounded
to milliseconds, including 0.000. Counts remain **14 modelled, 6 not modelled as static
decisions for vectors**, with supporting promises excluded. No vector was written.

**Records.**

| Variant/configuration | Unique states | Depth | Seconds | Safety | Reach |
| --- | ---: | ---: | ---: | --- | --- |
| Correct/Base | 63 | 24 | 0.002 | HHHHH | +++++-- |
| Correct/Unknown | 63 | 24 | 0.002 | HHHHH | +++++-+ |
| Correct/RequestLoss | 161 | 24 | 0.004 | HHHHH | +++++-- |
| Correct/RequestReorder | 161 | 24 | 0.006 | HHHHH | +++++-- |
| Correct/ReadLoss | 91 | 24 | 0.003 | HHHHH | ++++++- |
| Correct/ReadReorder | 77 | 24 | 0.003 | HHHHH | +++++-- |
| Correct/Crash | 121 | 26 | 0.004 | HHHHH | +++++-- |
| Correct/Unreadable | 83 | 24 | 0.003 | HHHHH | ++++++- |
| OmitRecord/Base | 63 | 24 | 0.002 | FHHHH | +++++-- |
| OmitRefusalRecord/Base | 63 | 24 | 0.002 | FHHHH | +++++-- |
| AlterRecord/Base | 63 | 24 | 0.002 | HFHHH | +++++-- |
| PurgeIndex/Base | 63 | 24 | 0.002 | HHFHH | +++++-- |
| RootBypass/Base | 115 | 25 | 0.004 | HHFHH | +++++-- |
| PurgeInclusive/Base | 63 | 24 | 0.002 | HHFHH | +++++-- |
| ForgetIndex/Crash | 121 | 26 | 0.004 | HHFHH | +++++-- |
| ReverseRead/Base | 63 | 24 | 0.003 | HHHFH | +++++-- |
| HideGap/Unreadable | 83 | 24 | 0.002 | HHHHF | ++++++- |

**Observation.**

| Variant/configuration | Unique states | Depth | Seconds | Safety | Reach |
| --- | ---: | ---: | ---: | --- | --- |
| Correct/Base | 26 | 12 | 0.000 | HHHH | +++++ |
| Correct/Crash | 51 | 14 | 0.000 | HHHH | +++++ |
| AdoptObserved/Base | 26 | 12 | 0.000 | FHHH | +++++ |
| StaleCurrent/Base | 26 | 12 | 0.000 | HFHH | +++++ |
| NoLastTime/Base | 26 | 12 | 0.000 | HFHH | +++++ |
| ForgetTime/Crash | 51 | 14 | 0.000 | HFHH | +++++ |
| AutoRepair/Base | 26 | 12 | 0.000 | HHFH | +++++ |
| InspectEffect/Base | 26 | 12 | 0.000 | HHHF | +++++ |
| InspectAccess/Base | 26 | 12 | 0.000 | HHHF | +++++ |

**Installs.**

| Variant/configuration | Unique states | Depth | Seconds | Safety | Reach |
| --- | ---: | ---: | ---: | --- | --- |
| Correct/Provider | 55 | 23 | 0.000 | HHHHH | ++++++++++-- |
| Correct/Https | 55 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/File | 55 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/Same | 55 | 23 | 0.001 | HHHHH | ++++++++-+-- |
| Correct/FailProvider | 64 | 23 | 0.001 | HHHHH | +++++++++++- |
| Correct/FailHttps | 64 | 23 | 0.001 | HHHHH | +++++++++++- |
| Correct/FailFile | 64 | 23 | 0.001 | HHHHH | +++++++++++- |
| Correct/BadProvider | 64 | 23 | 0.001 | HHHHH | ++++++++++-+ |
| Correct/BadHttps | 64 | 23 | 0.001 | HHHHH | ++++++++++-+ |
| Correct/BadFile | 64 | 23 | 0.001 | HHHHH | ++++++++++-+ |
| Correct/RequestLoss | 85 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/RequestReorder | 105 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/FetchLoss | 142 | 24 | 0.001 | HHHHH | +++++++++++- |
| Correct/FetchReorder | 113 | 23 | 0.001 | HHHHH | ++++++++++-- |
| Correct/Crash | 121 | 25 | 0.001 | HHHHH | ++++++++++-- |
| FailedAsSuccess/FailProvider | 64 | 23 | 0.001 | FHHHH | +++++++++++- |
| FetchConflict/Provider | 68 | 24 | 0.001 | HFHHH | ++++++++++-- |
| FetchSame/Provider | 70 | 24 | 0.001 | HHFHH | ++++++++++-- |
| SkipAuthorization/Provider | 55 | 23 | 0.001 | HHFHH | ++++++++++-- |
| DoubleClaim/Provider | 55 | 23 | 0.001 | HHHFH | ++++++++++-- |
| ForeignOwner/Provider | 55 | 23 | 0.001 | HHHFH | ++++++++++-- |
| WrongRoute/Provider | 55 | 23 | 0.001 | HHHHF | ++++++++++-- |
| Unchecked/BadProvider | 64 | 23 | 0.001 | HHHHF | ++++++++++-+ |
| Unchecked/BadHttps | 64 | 23 | 0.001 | HHHHF | ++++++++++-+ |
| Unchecked/BadFile | 64 | 23 | 0.001 | HHHHF | ++++++++++-+ |

**Added first single-thread BFS refutation: OmitRefusalRecord/Base, R1 (7 actions).**

```text
1. engine/source-E Events
2. source-S/runtime/archive-service Events
3. actor/reader Next
4. delivered actor/reader -> engine/source-E [position 0]
5. actor/reader Next
6. actor/reader Next
7. delivered actor/reader -> engine/source-E [position 0]
```

Projection: both synthetic event batches are recorded → initial keyed request is recorded →
non-root purge is refused → its required request-end occurrence is logged, but `keep` is
omitted. The earlier source-E record of category 1 has a different source-local ID and cannot
satisfy R1 for the refusal. Correct saves that refusal record. Later legitimate purges may
remove it without changing its historical saved-record evidence or the invocation index.

Only this README and `src/records.rs` changed in the correction. Existing models, the harness,
the specification and the promise accounting are unchanged. The coordinator's disposition
of MC7b-001/002 and MC8's catalogue work remain outside this order's scope.

### MC7b final-verifier fixes: pre-registration — 2026-11-11

Work order 115 registers these changes **before their first run**. The earlier claim,
"R4's permanent-evidence check reads the same storage snapshots directly", was not true
until this order: R4 checked preservation of evidence already in snapshots, but never
checked that a purge's evidence had been saved in the first place.

Each purge now names its source. R4 requires its exact evidence record in a `Change.after`
snapshot at that source, in addition to its existing content and preservation checks.
It reads only the ledger. Readability remains with R5/R6: an unreadable or lost record
can be reported as a gap under REC-006. `OmitPurgeRecord` creates the evidence but omits
the record write, independently of REC-001's event categories.

An invocation answered from its saved key now logs its own request-end occurrence
(category 1) and saves that record, with no dispatch or invocation-end occurrence.
This applies both to a succeeded repeat (OPS-009) and an unknown repeat on the plain
implementation (OPS-012). The branch uses whether the key answered, rather than the
message's repeat flag. When ForgetIndex or PurgeIndex has erased the key, dispatch and
invocation-end occurrences are still logged and saved. `OmitRepeatRecord` logs the
cached answer's required occurrence but omits its record write. Focused regressions
cover both outcomes, both message flags, lost keys and predicates with node data erased.

Both new variants belong to `BROKEN`, have explicit R4/R1 targets respectively, and
receive full table runs and first-refutation runs with ledger-only replay:

| Added variant/configuration | Expected R1 R3 R4 R5 R6 | Expected reach |
| --- | --- | --- |
| OmitPurgeRecord/Base | HHFHH | +++++-- |
| OmitRepeatRecord/Base | FHHHH | +++++-- |

The existing registered tables, fault windows, bounds and MC7b-001/002 are unchanged.
All 53 full runs must match the original tables plus the registered additions. Results
and every difference will be recorded below; no conformance vector is added.

### MC7b final-verifier results — 2026-11-11

Both must findings in the model are fixed. R4 now verifies saved purge evidence at the
purge's source, using only `Change` snapshots. Cached answers record a request ending
without dispatch; fresh dispatches still record their dispatch and invocation end.
The direct regressions cover succeeded and plain unknown answers, both repeat flags,
ForgetIndex/PurgeIndex redispatch, source attribution, and predicates with node data erased.

All **53 full runs match the unchanged registered tables plus the additions**. There
are no MC7b safety, reachability or fault-witness differences, and no correct-model
refutation. All 28 seeded runs (26 distinct variants, Unchecked on three paths) refute
exactly their named property. Every positive fault budget is reached. MC7b-001 and
MC7b-002 remain unchanged; neither correction is a specification finding. Accounting
remains 14 modelled and 6 static decisions for vectors, with supporting promises excluded.

The crate passed **50 tests (39 existing, 11 MC7b)** and zero doc-tests in both runs.
`cargo test --release` took **57.49 seconds wall**, including 17.24 seconds compilation
and 39.98 seconds test time. The count pass with `--nocapture --test-threads=1` took
**42.05 seconds wall**, with 41.77 seconds test time. Full checks use two checker threads;
first refutations use one. Formatting, Clippy, the five repository Python checks, all
41 Python checker unit tests, whitespace checking and both grep scans passed. The scans
printed nothing and returned grep status 1 (no matches). No rerun was needed for a test failure.

These measurements supersede the previous records results. All existing unique-state
counts, depths, safety and reach verdicts are unchanged. Seconds are measured and rounded
to milliseconds. Compared with the latest results above, timing changed for records
Correct/RequestLoss (0.004 to 0.005), Correct/RequestReorder (0.006 to 0.005),
Correct/ReadLoss (0.003 to 0.004), OmitRefusalRecord/Base (0.002 to 0.003),
ReverseRead/Base (0.003 to 0.002), HideGap/Unreadable (0.002 to 0.003), and installs
Correct/Provider (0.000 to 0.001). All observation measurements and other install
measurements are unchanged at the printed precision. The two new rows have no prior
measurement. There are no other differences from the registered verdict tables.

| Records variant/configuration | Unique states | Depth | Seconds | R1 R3 R4 R5 R6 | Reach |
| --- | ---: | ---: | ---: | --- | --- |
| Correct/Base | 63 | 24 | 0.002 | HHHHH | +++++-- |
| Correct/Unknown | 63 | 24 | 0.002 | HHHHH | +++++-+ |
| Correct/RequestLoss | 161 | 24 | 0.005 | HHHHH | +++++-- |
| Correct/RequestReorder | 161 | 24 | 0.005 | HHHHH | +++++-- |
| Correct/ReadLoss | 91 | 24 | 0.004 | HHHHH | ++++++- |
| Correct/ReadReorder | 77 | 24 | 0.003 | HHHHH | +++++-- |
| Correct/Crash | 121 | 26 | 0.004 | HHHHH | +++++-- |
| Correct/Unreadable | 83 | 24 | 0.003 | HHHHH | ++++++- |
| OmitRecord/Base | 63 | 24 | 0.002 | FHHHH | +++++-- |
| OmitRefusalRecord/Base | 63 | 24 | 0.003 | FHHHH | +++++-- |
| OmitPurgeRecord/Base | 63 | 24 | 0.002 | HHFHH | +++++-- |
| OmitRepeatRecord/Base | 63 | 24 | 0.002 | FHHHH | +++++-- |
| AlterRecord/Base | 63 | 24 | 0.002 | HFHHH | +++++-- |
| PurgeIndex/Base | 63 | 24 | 0.002 | HHFHH | +++++-- |
| RootBypass/Base | 115 | 25 | 0.004 | HHFHH | +++++-- |
| PurgeInclusive/Base | 63 | 24 | 0.002 | HHFHH | +++++-- |
| ForgetIndex/Crash | 121 | 26 | 0.004 | HHFHH | +++++-- |
| ReverseRead/Base | 63 | 24 | 0.002 | HHHFH | +++++-- |
| HideGap/Unreadable | 83 | 24 | 0.003 | HHHHF | ++++++- |

| Other changed measurement | Unique states | Depth | Seconds | I24 I25 I27 I13 I17 | Reach |
| --- | ---: | ---: | ---: | --- | --- |
| Installs Correct/Provider | 55 | 23 | 0.001 | HHHHH | ++++++++++-- |

**New first single-thread BFS refutations.** Both predicates still fail after all node
data is replaced with defaults, confirming that only ledger evidence determines the failure.

`OmitPurgeRecord/Base`, R4, **8 actions**:

```text
1. engine/source-E Events
2. source-S/runtime/archive-service Events
3. actor/reader Next
4. delivered actor/reader -> engine/source-E [position 0]
5. actor/reader Next
6. actor/reader Next
7. delivered actor/reader -> engine/source-E [position 0]
8. delivered actor/reader -> engine/source-E [position 0]
```

The non-root purge is refused and recorded, then root purges. Evidence is constructed
with the right cutoff and removed IDs, but no source-E snapshot contains its record.

`OmitRepeatRecord/Base`, R1, **14 actions**:

```text
1. engine/source-E Events
2. source-S/runtime/archive-service Events
3. actor/reader Next
4. delivered actor/reader -> engine/source-E [position 0]
5. actor/reader Next
6. actor/reader Next
7. delivered actor/reader -> engine/source-E [position 0]
8. delivered actor/reader -> engine/source-E [position 0]
9. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
10. actor/reader Next
11. delivered actor/reader -> engine/source-E [position 0]
12. delivered engine/source-E -> source-S/runtime/archive-service [position 0]
13. actor/reader Next
14. delivered actor/reader -> engine/source-E [position 0]
```

The original invocation succeeds, both root purges preserve its saved key, and the repeat
is answered from that key. Its distinct request-end occurrence has no saved record.
The regression separately exercises the same omission for a plain unknown invocation.

Only this README's MC7b sections and `src/records.rs` changed. No vector was written,
and no file outside this work order's scope needs a change for these two fixes.
