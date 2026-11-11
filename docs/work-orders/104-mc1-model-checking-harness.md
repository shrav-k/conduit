# Work order 104 (MC1): the model-checking harness

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This order writes no conformance vector, so its `vectors` list is
empty. It has network access to fetch crates; use it for nothing else. The machine is shared:
build with `CARGO_BUILD_JOBS=3`, run one cargo command at a time, and never print a whole large
file or log: use `head`, `tail`, `grep` or a count.

## What and why

An order of the proof lane under [ruling 71](../proposals/ruling-71/README.md). A40 asks that the
safety promises be model-checked with Stateright, in small configurations, in the ordinary test
run. The configurations include two machines with message loss, reordering, a partition and a
lost machine, and each safety promise must hold or have a recorded reason it cannot be checked
this way (G5). Ruling 47 makes that a condition of v0: "the safety promises hold under loss,
reordering and partition". [Experiment 004](../../experiments/004-stateright-evaluation/README.md)
evaluated the tool and found it fit for exhaustive checks of a small, pure model. This order
builds the harness that every model of A40 will use, with one small model that tests the
harness itself. The models of Conduit's promises come in later orders: MC2 takes operation
identity and unknown outcomes.

Read experiment 004's README and `src/main.rs`; A40's row in [docs/AUDIT.md](../AUDIT.md) and G5
in [the readiness gate](../implementation/INDEPENDENT_BUILDS.md#readiness-gate); IN_FORCE.md,
topic 13; and the documentation of Stateright 0.31.0's `Model`, `Property` and `Checker`, in
the crate's source in Cargo's registry. Its `checker/bfs.rs` and `checker.rs` matter for
decision 5.

## The decisions it applies

1. **Where it lives (D10, the lane coordinator's decision, logged in this order's Record).**
   The harness is its own crate, `experiments/012-model-checking/`, and not part of the
   engine's workspace. The directory holds a placeholder README now: replace it. CI already runs `cargo test --release` in every experiment's directory,
   and that is the ordinary test run A40 asks for. The crate touches no file of `engine/`, and
   it can move into `engine/` later. It has its own `Cargo.toml`, a committed `Cargo.lock` and
   a `rust-toolchain.toml` pinning channel `1.97.1`, as the engine does. Its one dependency is
   Stateright pinned at `=0.31.0`, the version experiment 004 evaluated. Start the lock file
   from experiment 004's, whose crates are already in Cargo's cache. Unlike other experiments,
   012 is a living harness that later orders extend. CI runs its tests but neither formats nor
   lints experiments, so every order that changes it runs `cargo fmt --check` and clippy
   itself. Its README says both.
2. **What it models.** The models are reference models of the promises, not of the engine's
   code. A model states the rules a promise sets, in the fewest states that can break them.
3. **The network and the faults.** The harness implements `stateright::Model` itself, because
   Stateright's actor model has no partition. Its state holds the nodes, the messages in
   flight and a ledger of what happened outside every node (decision 6). Messages in flight
   are kept as one sequence per one-way link, so a copy and two equal messages stay distinct.
   Every fault is an explicit action. Each has a budget the model sets, counted in total for
   its kind across the whole run, so that the state space stays finite:
   - **reordering**: delivering the oldest message on a link is ordinary delivery, while
     delivering any later message uses the reorder budget. A budget of zero means first in,
     first out on each link;
   - **loss**: any message in flight may be dropped;
   - **duplication**: a message in flight may be copied, for a model that needs a resend at
     the wire;
   - **partition**: the link between two nodes may be cut, and later healed. While it is cut,
     nothing is delivered across it. A message sent across it, or already in flight on it, is
     held until the heal, unless a loss drops it;
   - **crash and recovery**: a node may crash, losing what it did not save, and recover from
     what it saved, as in experiment 004. Messages to a crashed node stay in flight;
   - **a lost machine**: a node may crash for good and never recover. This is an action with
     its own budget, at any point of a run;
   - **timers**: a node's timer may fire.

   A model turns off a fault it does not need with a budget of zero, and says why. Across this
   order's configurations, every kind of fault has a budget of at least one, healing and
   recovery included, so that the harness's code for each is exercised.
4. **What every model has.** The harness provides, and its own tests check, that each model
   has:
   - its configuration stated in one place: the nodes, every budget and every bound. Budgets
     of one, and two for timers, are the starting point;
   - each property named, with the promise IDs it checks, or "harness" for this order's own
     model, which checks none of Conduit's promises;
   - reachability properties, so that a model cannot pass because nothing happens: one for each
     fault kind it enables, and one for each outcome its safety properties are about, such as a
     confirmation reached;
   - for each safety property, at least one deliberately broken variant. The checker must
     refute that property, and only that property, in the variant;
   - a state ceiling of at most 1,000,000 unique states, and a timeout. A run cut short by
     either fails: it never counts as "holds";
   - its unique state count and depth, recorded in the README's results with the date, the
     chip and core count (never the host name), the toolchain and the thread count. Times on
     the shared machine are indicative only.
5. **How the checker runs.**
   - **Refutation**: breadth-first, one thread, stopping at the first discovery of the named
     property (`finish_when` with `HasDiscoveries::AnyOf`). With more threads Stateright's
     breadth-first search can replace a stored trace with a longer one. The test asserts that
     the counterexample is found and prints its trace.
   - **Counts and "holds"**: a separate full run, with the thread count from
     `CONDUIT_MC_THREADS`, which defaults to 2 and is clamped to at most 2.
   - **One check at a time**: the tests share a lock, so that `cargo test`'s parallel tests
     run one model check at a time.
   - **Time**: the crate's full `cargo test --release` should finish in about 60 seconds on
     `ubuntu-latest`. Say what you measured and on what.
6. **This order's model.** A sender retries a request with an ID, over the faulty network, to a
   receiver that applies an effect and answers. The ledger, outside every node and untouched by
   any fault, counts the effects applied per ID and records each confirmation the sender
   receives. The properties read the ledger, never a node's own state:
   - **at most once**: the effect is applied at most once per ID. It holds when the receiver
     keeps the IDs it applied in saved state. The broken variant keeps them in volatile state
     only, so its refutation needs a crash;
   - **no false confirmation**: the sender never holds a confirmation for an effect that was
     not applied. It holds when the receiver answers only after applying. In the broken
     variant the receiver answers and applies in two separate steps, answering first.

   The model reaches each kind of fault, including a partition cut and healed, and a receiver
   lost for good partway through a run. Every reachability property holds in each
   configuration. It is the harness's test, not evidence about Conduit, and its README says so.
7. **The harness's helpers.**
   - One call checks that a model holds every property, within its ceiling and timeout.
   - One call checks that a named property is refuted and returns the trace.
   - A printer writes a trace one action per line, naming each fault: delivered, delivered out
     of order, lost, duplicated, cut, healed, crashed, recovered, lost for good and timer.
   - A report gives the unique states, the depth and the seconds taken.
8. **The README** is the pre-registration and the record. In this order:
   - A40's question;
   - decision 1, with its reason;
   - how to run it, with the command that shows the counts:
     `cargo test --release -- --nocapture --test-threads=1`;
   - the fault model, and the rules of decision 4 for every model;
   - before any result, each variant's expected verdict (ruling 45);
   - a dated results table;
   - how it relates to experiment 004 and to the safety catalogue, from which later models take
     their promises. Name the catalogue by its path, `docs/implementation/SAFETY_CATALOGUE.md`,
     in code text and not as a link: it may not exist yet;
   - the models to come, by name only. MC2 takes operation identity and unknown outcomes
     (OPS-008 to OPS-013, REV-002, REV-011 and REV-012), and later orders take the rest.
9. **The index and the trackers** are already set: the row of `experiments/README.md`, the
   register's row "Experiment 012" and step 6's State in ROADMAP.md, and A40's status in
   docs/AUDIT.md. Change none of them.

## Scope

Change only `experiments/012-model-checking/`.

Do not change any other file: not `engine/`, `spec/`, `conformance/`, `packages/`,
`experiments/README.md`, any other experiment, ROADMAP.md, docs/AUDIT.md, IN_FORCE.md or any
record or ruling. Report anything else that needs changing.

## Done when

In `experiments/012-model-checking/`, these pass:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release`, and its wall time;
- `cargo test --release -- --nocapture --test-threads=1`, for the counts.

Put in your report's summary the test totals, the wall time, and each variant's unique states,
depth, seconds and verdict. Put the same in the README's results.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes; new files are untracked, and
these see them:

```bash
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" --exclude-dir=target experiments/012-model-checking
grep -rn -E "[[:space:]]+$" --exclude-dir=target --include='*.md' --include='*.rs' --include='*.toml' experiments/012-model-checking
```

`target/` stays ignored and unstaged. Do not stage anything.

## Record

Run on 2026-11-11 beside orders 100 to 103 and two other build orders: the worker in 12 minutes
(740 s), then a fresh verifier in 3 minutes (200 s); one round. The verifier found no fault. It
ran the release binary's 8 tests again and reproduced the recorded counts and traces; it could
not recompile in its read-only sandbox.

The harness implements Stateright's `Model` itself. It keeps one sequence of messages per
directed link, with total budgets for reordering, loss, duplication, cut and heal, crash and
recovery, a lost machine and timers. Its ledger sits outside the nodes. A run cut short by its
exact unique-state ceiling or its timeout fails. A refutation runs on one thread and keeps the
first trace.

Its own model held and broke as pre-registered:

- Saved holds (6,090 states, depth 16).
- VolatileIds breaks only "at most once", and only through a crash (8,910 states). A control
  with crash and recovery off holds (1,395 states).
- AnswerFirst breaks only "no false confirmation" (18,360 states).

The worker read decision 5 at its nearest sound reading. Stateright checks `finish_when` only
after batches of 1,500 states, so a visitor keeps the first counterexample. The worker's one
finding was that this order named a Record it did not yet have; this is that Record.

D10, the lane coordinator's decision, is logged for the owner to strike. The harness lives under
`experiments/` as its own crate, since CI runs every experiment's tests, and it may move into
`engine/` later.

Tests: 8 passed. The lane coordinator ran them again with fmt and clippy before the merge, and
ran the repository's checks. No time on `ubuntu-latest` is measured, since CI was paused on
2026-11-11.
