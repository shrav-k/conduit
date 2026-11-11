# 012 — model-checking harness

Status: executed 2026-11-11 under work order 104 (MC1), ruling 71's proof lane.
The expected verdicts below were registered before execution; later orders extend this harness.

## Question and scope

A40 asks whether each safety promise holds under model checking in small configurations,
including two machines with message loss, reordering, partition and a lost machine, or has
an explicit reason it cannot be checked this way (readiness gate G5).

This order tests the harness itself, **not any Conduit promise**. Its properties are labelled
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

MC2: operation identity and unknown outcomes (OPS-008 to OPS-013, REV-002, REV-011, REV-012).
Later orders take the rest of the safety catalogue.
