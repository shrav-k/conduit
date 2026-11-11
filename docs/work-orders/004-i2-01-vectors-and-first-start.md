# Work order 004 (I2-01): the vectors' format, the core's crates, and first start

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`
beside this file. This order has network access, to fetch crates; use it for nothing else.

## What and why

This is the first of increment 2's build orders, I2-01 of the
[study](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md#7-a-proposed-sequence-of-work-orders). It
lays down what every later order builds on: the format of the conformance vectors, the pure
core and the runner of the vectors, and the core's first transition, first start, with every
decision citing the promise that requires it.

Read: [the study](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md), sections 2 (its "First start"
table), 4 and 5; [the admin's decisions](../proposals/first-build/increment-2/decisions.md),
which bind this order, decisions 3, 4 and 7 above all; [the first build](../implementation/FIRST_BUILD.md);
the loader's [README](../../engine/README.md); and the promises FST-001 to FST-006,
LIF-034, NAM-020, PLC-008, PLC-026, PRV-014 and REC-001 to REC-003, with what they cite in
`spec/core/` and `spec/conventions/`.

## The decisions to apply

1. **The vectors' format** is the study's section 4, as decision 3 adopts it: YAML cases under
   `conformance/`, each with a stable ID, its layers, its promise IDs, a title, the profile it
   assumes, an initial state, its files, a sequence of inputs and what it expects. Write
   `conformance/README.md` (not binding: the format is a convention of the runner, not part of
   the language), a JSON Schema for a case, `conformance/case.schema.json`, and the first
   build's profile, `conformance/profiles/first-build.yaml`, holding decision 4's choices on
   the implementation-defined list. A case's documents stay ordinary Conduit files.
2. **Two new crates** in the workspace under `reference-model/crates/`: `conduit-core`, pure,
   with no file, clock, network or process access, and `conduit-conformance`, which reads the
   cases and runs them against the loader and the core. Keep the loader as it is, apart from
   what the runner needs of its public interface.
3. **The core's interface** is section 5's, in its first form: a state, an input, and a
   transition that gives the new state, the decisions, the records, the intents for the shell
   and a reply. Each decision names the promises that require it, and an implementation choice
   names its DL item in a field of its own, never as a promise. Time, IDs and machine reports
   enter as inputs.
4. **First start**: from an empty state and a first-start input carrying the time, the ID
   allocations and the local machine's report, the core makes exactly the objects FST-001 and
   FST-002 name, with root's two declared Conduits open and all ready, makes nothing else and
   calls no implementation (FST-003), keeps their provenance (FST-005), makes `local` as
   PLC-008, PLC-026 and PRV-014 say, and records what it did, with the event types of decision
   7. A second first start on a state that has them makes no new identity (decision 4, DL6).
5. **Vectors**, under `conformance/`: the first-start case; FST-006, a submitted instantiation
   of each protected core template rejected; LIF-034, retiring, unregistering or declaring
   `local` lost refused; the repeated first start. And the four settlements of increment 1
   whose only vectors are the loader's tests: write each as a vector that the runner runs
   through the loader (the log of increment 1 and the study's section 4 name them).
6. **A finding**: where a promise does not decide what the core must do, stop that case, do not
   invent its answer, and log it in a new `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md`, in
   the form of [the log of increment 1](../proposals/first-build/README.md#settlements-register), with
   "For the admin" as its state. Add the log to the table in
   [the first build's findings](../proposals/first-build/README.md).

## Scope

Create and change only: `conformance/`, `reference-model/Cargo.toml`, `reference-model/Cargo.lock`,
`reference-model/crates/core/`, `reference-model/crates/conformance/`, the loader's public
interface where the runner needs it, `reference-model/README.md`,
`https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md` and `docs/proposals/first-build/README.md`.
Report anything else that needs changing; do not change `spec/`.

## Done when

The first-start vector shows exactly the objects and Conduits that FST-001 and FST-002 name,
ready and open, with the supplied time and IDs and no other intent; the protected and repeated
first-start cases and the four loader settlements pass as vectors; every decision the core
makes cites a promise. From the repository root: `python3 tools/check_docs.py`,
`python3 tools/set_names.py --check` (it scans `reference-model/`; take every name from
`spec/names.yaml`), `python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and
`git diff --check` pass; in `reference-model/`, `cargo test --release`, `cargo clippy
--all-targets -- -D warnings` and `cargo fmt --check` pass. Check the files you create for
trailing white space yourself; do not stage anything.
