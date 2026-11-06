# 004 — Stateright evaluation

Status: executed 2026-11-05. Exploratory Rust code under the
[exploratory-code ruling](../../docs/REVIEW.md#plan-audit-2026-11-05). It models one promise
group to evaluate a tool. It is not a Conduit runtime, a reference model or conformance evidence.

## Question

Is [Stateright](https://github.com/stateright/stateright), a model checker embedded in Rust, a
suitable way to check Conduit's safety promises under message loss, reordering and crashes?

## What was modelled

One revision-checked control change with an external effect, taken from the
[outcome and recovery rules](../../docs/protocol/CONDUITS.md#outcomes-and-recovery) and the
[state check](../../docs/protocol/LIFECYCLE.md#template-defined-configuration-changes):

```text
caller --Change{op, expected_rev}--> authority --Apply{op}--> provider
caller <--Outcome{op, ..}---------- authority <--Applied{op}-- provider
```

- Two callers each submit one change from revision 0 and retry with the same operation ID.
- Each send is delivered at most once, in any order, and may be lost.
- Any one actor at a time may crash and recover from what it saved.
- The provider counts how many times each effect was applied.

Promises checked in every reachable state:

1. An effect is applied at most once per operation.
2. At most one change is confirmed from one revision.
3. A confirmed change has an applied effect.
4. A caller never holds an outcome the authority did not record.

Two reachability checks guard against a model that passes because nothing happens: a change can
be confirmed to its caller, and a stale caller can be told of the conflict.

Five rule sets were run. Two were expected to keep every promise and three to break the first.

## Run it

```sh
cargo run --release              # 2 callers, 1 crashed actor at a time, 1 thread
cargo run --release -- 3 1 8     # callers, crashed-at-once, threads
cargo test --release             # asserts every variant behaves as predicted
```

## Results

Measured on an Apple M2 Max (12 cores), rustc 1.97.1, Stateright 0.31.0. `cargo test --release`
passed. Peak memory for the default run on 8 threads was 192 MB.

| Rule set | Expected | Result | Unique states (2 callers) | Time, 1 thread | Unique states (3 callers) | Time, 8 threads |
| --- | --- | --- | --- | --- | --- | --- |
| Inspect only; intent saved; plain provider | Holds | Holds | 40,835 | 0.17 s | 2,248,200 | 3.60 s |
| Inspect then reissue; intent saved; provider deduplicates by operation ID | Holds | Holds | 53,505 | 0.20 s | 2,928,474 | 4.13 s |
| Inspect then reissue; intent saved; plain provider | Breaks promise 1 | Broken | 74,285 | 0.27 s | 4,166,010 | 5.93 s |
| Blind replay; intent saved; plain provider | Breaks promise 1 | Broken | 32,035 | 0.11 s | 1,778,040 | 2.35 s |
| Inspect only; intent not saved; plain provider | Breaks promise 1 | Broken | 623,630 | 2.65 s | did not finish in 7 minutes; stopped | — |

Shortest trace for the third rule set, as printed by the spike:

```text
1. LOST caller-1 -> authority: Change { op: 1, expected_rev: 0 }
2. caller-2 -> authority: Change { op: 2, expected_rev: 0 }
3. caller-2 retries
4. caller-2 -> authority: Change { op: 2, expected_rev: 0 }
5. authority -> provider: Query { op: 2 }
6. authority -> provider: Apply { op: 2 }
7. provider -> authority: Ledger { op: 2, applied: false }
8. authority -> provider: Apply { op: 2 }
```

The provider answers "not applied" at step 5, the original request arrives at step 6, and the
authority acts on the stale answer at step 8. The effect is applied twice.

## Findings

**About the language.** The accepted rule reads: "After an uncertain write, inspect
local/provider state before reissuing". The third rule set follows that sentence literally and
breaks the duplicate-effect promise in eight steps. Inspection alone does not make reissue safe,
because the original request can still arrive after the provider reports that it has not.
Reissue is safe in this model only when the provider deduplicates by operation identity. The rule
needs that condition stated; otherwise the outcome must stay unknown. This is logged as a
language defect L1 in the [defect register](../../docs/LANGUAGE_DEFECTS.md).

**About fit.**

- It expressed what was needed: lossy and reordering networks, timers, crash and recovery from
  saved storage, and invariants over the whole system state.
- It found all three seeded defects, including the one that needs a crash.
- The model and its checks are ordinary Rust, about 460 lines, with one dependency.
- Small configurations are fast enough to run on every change.

**About limits and quirks.**

- **Bounded only.** A result holds for the configuration checked (here two or three callers),
  not for all sizes.
- **State growth is steep.** A third caller multiplied the state count by about 55. The crash
  limit counts actors crashed at once, not crashes in total, so a model with unsaved state can
  loop through crash and recovery without bound; that variant did not finish with three callers.
  Models must bound retries and keep saved state small.
- **Liveness is weak.** The `Eventually` check documents false negatives on paths that end in a
  cycle, and there are no fairness assumptions. "Every change is eventually resolved" cannot be
  established this way.
- **Traces need care.** A run to completion overwrites a broken promise's trace with later
  violating states and so reports a long one. Stopping at the first failure on one thread gives
  a short trace, which can still contain an irrelevant step.
- **One actor type per model.** Different roles need an enum wrapper. Properties are plain
  function pointers and cannot capture values.
- **Its actor runtime is UDP only.** It is not a candidate for the engine's transport.

**About maintenance.** Version 0.31.0 was released on 2025-07-27 and the repository has had no
commits since. It is MIT licensed, has 11 open issues, pulls in 46 packages and built without
warnings on current Rust. The API used here is small, so pinning the version is a workable
mitigation.

## Verdict

Suitable for one job: exhaustively checking the safety promises of a small, pure reference model
in small configurations, as part of the ordinary Rust test run. It should not be relied on for
liveness promises, for unbounded configurations or as a runtime.

Not evaluated here: the Explorer web interface, symmetry reduction, the simulation mode and the
linearizability tester. Alternatives such as TLA+, Quint, Kani and network-simulation crates were
not run and are not compared.

## Evidence log

| Date | Work | Evidence |
| --- | --- | --- |
| 2026-11-05 | Built and ran the spike; measured five rule sets at two sizes | Results table above; `cargo test --release` passed in 3.4 s |
