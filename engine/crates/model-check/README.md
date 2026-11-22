# Bounded checks of the real core

Work order 237 checks safety for one caller, one request and one idempotency key.
Each invocation, discovery answer, stop and delivered answer calls the real
`conduit_core::step`. The conformance runner prepares step 2 of
`requests-unknown-write` for the plain server and
`requests-repeat-failed-earlier-unanswered` for the Conduit-aware, deduplicating
server. It runs the preceding registration step and its assertions, expands the
request fixture, and applies the patches and contract updates. Each exploration
keeps the fixture time and derives fresh IDs from the successive core steps on
its path.

The implementation stand-in applies a successful plain call. It can choose one
error without an effect. The aware stand-in remembers its first answer for the
key and repeats that answer without another effect. It can first choose a
failure without an effect. The three controls respectively apply every call,
apply the first call and answer failed, and check an aware operation as if it
could never retry an unknown outcome.

The caller sends once and may repeat once. The engine may stop waiting once.
It can stop while messages are still queued. Discovery has one immediate
answer and is outside the fault model. Each directed link holds up to three
messages in order. Normal delivery takes the first position, loss removes any
position, and reordering delivers a later position. T enables neither fault, L
one loss, R one reordering, and LR one of each. The plain server has at most one
message per link. Exceeding the queue bound fails the test.

The action ledger groups all dispatches of that one request and records effects,
answers sent and delivered, and outcomes from the core's records and replies.
It checks at most once (O), no wrong outcome (W), and unknown stays unknown
(U) from [experiment 012's MC2 rerun](../../../experiments/012-model-checking/README.md#mc2-rerun-under-ruling-78-model-and-pre-registration)
and [its predicates](../../../experiments/012-model-checking/src/operations.rs).
Reconciliation is outside this slice. The search witnesses an effect, each
enabled fault, each possible outcome, and a repeat answered without dispatch.
The aware configuration also witnesses a retry after unknown and, in L, R and
LR, a later attempt answered failed while an earlier one has no answer.

The checker hashes the invocation index, records and allocated IDs along with
the harness state. The other core fields are shared from the starting snapshot.
Every step receives them in full and must return them unchanged. The model
compares the returned fields with the shared snapshot before admitting a state.
A refutation search keeps its path for printing and excludes that path from
state identity. Later steps use no trace data.

This is a bounded safety check. It uses in-memory stand-ins for the caller and
implementation and opens no socket. It runs on one machine, with no second node,
network cut or repair, or lost machine. It excludes the shell's transport, liveness,
engine restart (`Recover`), implementation restart and duplicated dispatch. It
checks only O, W and U. W's OPS-014 half and U's reconciliation exception are idle.
Restarts, duplicated dispatch, two-machine faults, reconciliation, revisions,
hooks and access over time belong to later slices of the three to five orders
in [DEC-005](../../../docs/decisions/log.md#dec-005-2026-11-20-v0s-gate-publication-and-stewardship).
The run used one caller repeat and one engine stop, reduced from the order's two
of each to meet the per-search and crate runtime targets.

## Registered verdicts

| Configuration | Parts | O | W | U |
| --- | --- | --- | --- | --- |
| plain | T, L | holds | holds | holds |
| deduplicating | T, L, R, LR | holds | holds | holds |
| o-control | T | refuted | holds | holds |
| w-control | T | holds | refuted | holds |
| u-control | T | holds | holds | refuted |

## Run it

From `engine/`:

```sh
cargo test --offline --locked --release -p conduit-model-check -- --nocapture --test-threads=1
```

Full searches use one-thread breadth-first search. Reaching 200,000 unique
states or 300 seconds yields no verdict. An unreached witness fails the test.
A separate BFS captures each control's first refutation before Stateright can
replace it within its batch. The printout lists actions and the ledger. The
simulation tests use a single thread and Stateright's uniform chooser. It
checks the 32-visit target between traces, so with at most 10 states per trace
the run visits at most 41 states. The repeatability test compares two BFS
printouts and two simulation printouts with the same seed.
Within these bounds, BFS is exhaustive, and the simulations exercise reproducible
seeded paths for the larger bounds of later slices while visiting only states BFS
already explores.

## Results on 2026-11-21

Warm release run on macOS, based on commit `62a1c2fe`. All eight tests passed in
114.37 seconds. The initial two-repeat search exceeded 30 seconds, so the run
was stopped and the caller's repeat budget was reduced to one. A complete run
with one repeat and two stops took 301.33 seconds. The final run has one engine
stop and captures the refutation path at discovery. It retains all registered
fault parts and witnesses. BFS depth includes the initial state.

| Configuration | Part | O | W | U | Unique states | Depth | Seconds | Witnesses |
| --- | --- | --- | --- | --- | --- | ---: | ---: | ---: |
| plain | T | holds | holds | holds | 48 | 6 | 2.737 | 4/4 |
| plain | L | holds | holds | holds | 71 | 6 | 3.820 | 5/5 |
| deduplicating | T | holds | holds | holds | 104 | 10 | 5.021 | 6/6 |
| deduplicating | L | holds | holds | holds | 192 | 10 | 7.987 | 8/8 |
| deduplicating | R | holds | holds | holds | 130 | 10 | 6.513 | 8/8 |
| deduplicating | LR | holds | holds | holds | 240 | 10 | 9.654 | 9/9 |
| o-control | T | refuted | holds | holds | 52 | 10 | 2.526 | 5/5 |
| w-control | T | holds | refuted | holds | 60 | 10 | 3.057 | 5/5 |
| u-control | T | holds | holds | refuted | 104 | 10 | 5.282 | 6/6 |

The first BFS traces contain 7 actions for O, 4 for W and 5 for U. W's
four-action trace also reproduced with simulation seed 237. Both the BFS and
the seeded simulation gave byte-identical printouts on their two runs.

The correct simulations had these visit counts. Simulation depth counts
actions from the initial state.

| Configuration and part | Seed | Visits | Depth |
| --- | ---: | ---: | ---: |
| plain/L | 17 | 33 | 5 |
| plain/L | 237 | 32 | 5 |
| plain/L | 20261121 | 33 | 5 |
| deduplicating/LR | 17 | 36 | 9 |
| deduplicating/LR | 237 | 36 | 9 |
| deduplicating/LR | 20261121 | 32 | 9 |
