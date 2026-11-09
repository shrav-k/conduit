# Work order 003: a study for increment 2, the core on S1

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `003-increment-2-study.schema.json` beside this
file.

## What and why

Increment 2 of [the first build](../implementation/FIRST_BUILD.md#increments) is the core on
S1: first start, the registration and resolving of S1's documents, setup and the opening of
declared Conduits (CON-004), the access decision (ACC-001), how a request ends (OPS-001), and
records, with the safety vectors S1 touches, E05 of round 6 first. Every decision the core makes
cites the promise that requires it. Before the admin session writes the orders that build it,
it needs the ground read: what S1 is as documents, which promises the core must keep for it,
which safety cases it must show, how a vector is written, and where the text does not decide.
This order is that reading. It decides nothing: where a choice is needed, it sets out the
options with a recommendation, for the admin.

## Read

Everything under `spec/`, as far as S1 needs it, starting from `spec/README.md`'s path for an
engine builder. Then [the first build](../implementation/FIRST_BUILD.md) (increments, what
increment 3 needs, the tasks and measures), the [roadmap](../../ROADMAP.md)'s S1 row and step 3,
round 6's E05, D08 and D22 under `docs/proposals/reaudit/round-6/`, the
[loader's README](../../reference-model/README.md), the
[log of increment 1](../proposals/first-build/log-increment-1.md), rulings 40, 46 and 48 under
`docs/proposals/`, and the baseline for S1 in `experiments/007-s1-baseline/` (its setup is what
S1 does with today's tools).

## What to produce

In a new directory `docs/proposals/first-build/increment-2/`:

1. **`plan.md`**, not binding, marked as the worker's proposal awaiting the admin's decisions,
   with these sections:
   1. *S1 as documents.* The project that does T1 and T2: two Agents, one directory that one
      reaches through a Conduit listing reads and the other through one listing reads and
      writes, and a message history both post to and read. The packages it needs: the
      filesystem MCP server as a `plain` item with the contracts its tools map to, the message
      history, and the Agent types, which increment 2 needs only as far as registering and
      resolving them. Follow ruling 40: reuse an existing tool for the history, as A70 did,
      rather than invent a service; set out the options and recommend one.
   2. *The promises S1 touches*, grouped by the slices of increment 2 (first start; registration
      and resolving; setup and opening; the access decision; how a request ends; records): each
      ID with one line on what the core must do for S1, and whether the loader already covers
      it.
   3. *The safety cases S1 touches*, E05 first, each as a proposed vector: its inputs, the
      decision expected, and the promise that requires it.
   4. *A proposed format for the vectors* (A39): their layout under `conformance/`, how a vector
      states its documents, its requests and the diagnostics and outcomes it expects (by
      `ruleId`, with the keyword's location for a schema's rejection, under SUB-005), and how
      the core runs them. Include the four settlements of increment 1 whose only vectors are the
      loader's tests.
   5. *A proposed shape of the core*: its crates beside `conduit-loader`, its pure interface
      (state and inputs in, decisions and records out, each decision citing its promise), and
      how time, IDs and anything else not deterministic are passed in.
   6. *Where the text does not decide*: each gap, ambiguity or contradiction met while reading
      for S1, with the IDs, the options and a recommendation.
   7. *A proposed sequence of work orders* for increment 2, each sized for one Codex run, with
      its scope and its done-when.
2. **`s1/`**, the draft documents of section 1: the project's files as a project directory, and
   each package as a directory of its own, written in the terms and forms of `spec/`.
3. **One test**, `reference-model/crates/loader/tests/s1_drafts.rs`, that reads each draft
   directory with the loader and requires it accepted, so the drafts are valid as far as the
   loader goes. Any draft that the loader cannot judge alone (a rule of the core) is noted in
   the plan.

## Scope

Create only `docs/proposals/first-build/increment-2/` and the one test file. Change nothing
under `spec/`: what the text needs goes in section 6. Report anything else that needs changing.

## Done when

The plan's seven sections are written, every promise ID it cites exists and says what the plan
claims, the drafts are accepted by the test, and these pass: `python3 tools/check_docs.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_roadmap.py`, `git diff --check`, and
in `reference-model/`: `cargo test --release`, `cargo clippy --all-targets -- -D warnings` and
`cargo fmt --check`.
