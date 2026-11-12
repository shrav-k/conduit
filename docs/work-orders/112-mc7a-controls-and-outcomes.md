# Work order 112 (MC7a): controls, revisions and outcomes, model-checked

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This order writes no conformance vector, so its `vectors` list is
empty. It has network access to fetch crates; use it for nothing else. Build with
`CARGO_BUILD_JOBS` as your environment sets it, run one cargo command at a time, and never print
a whole large file or log: use `head`, `tail`, `grep` or a count.

## What and why

An order of the proof lane under [ruling 71](../proposals/ruling-71/README.md). A40 asks that each
safety promise of the shared conventions hold under model checking, or be listed with the reason
it cannot be checked this way (G5). Ruling 75, E1, counts every convention-layer safety promise in
[the catalogue](../implementation/SAFETY_CATALOGUE.md), 217 in all.

This order models a control changing an instance's settings under contention, with revisions,
unknown outcomes and reconciliation (S5's area). It also models what an outcome may claim,
cancellation, the order of reasons, and model work started only by its own operation. It covers
31 promises:

- in `spec/conventions/operations.yaml`: OPS-001 to OPS-007, OPS-019, OPS-020, OPS-023, OPS-026,
  OPS-027, OPS-029, OPS-033, OPS-034 and OPS-036;
- in `spec/conventions/revisions.yaml`: REV-001, REV-003, REV-005, REV-006, REV-007, REV-009,
  REV-010, REV-013, REV-014 and REV-015;
- in `spec/conventions/implicit.yaml`: IMP-001, IMP-002 and IMP-008;
- in `spec/promises/templates.yaml`: TPL-011 and TPL-013.

MC2 already modelled REV-002, REV-011 and REV-012, and operation identity; this order does not
repeat them. MC2's README left REV-010 out; this order's entry for it supersedes that, so MC8
counts it once.

Read:

- experiment 012's README and `src/`, every model in it, and the findings recorded against
  earlier orders: MC2-001, MC3-001, MC3-002, and MC4's record of its model's own defects, in its
  section and in `docs/work-orders/108-mc4-placement-and-lost-machine.md`;
- the catalogue's rows for the 31 promises, and the promises themselves;
- S5's pre-registration, `experiments/016-s5-control-contention/README.md`, for the cases S5 will
  run;
- the promises the properties lean on: OPS-008 to OPS-014, OPS-018, OPS-024, IMP-007, REV-002,
  REV-004, REV-011, REV-012, CTR-006, ACC-019, ACC-033, PLC-017 and IFC-025;
- order 077, `docs/work-orders/077-spec-reconcile-fenced.md`, steps 1 and 2, for what fences an
  attempt.

## The decisions it applies

1. **Where it lives.** A new module, `experiments/012-model-checking/src/controls.rs`, and its
   section in the README, placed after the others. Change `src/harness.rs` only by an addition
   that its existing tests survive.
2. **Every promise accounted for.** The README lists each of the 31 promises once, either as
   modelled, with the property that checks it, or as not modelled, with the reason.
   - **"A static decision, to be shown by vectors"** is enough for a rule with no race and no
     fault. Examples: OPS-019, OPS-020, OPS-023's order of reasons, OPS-026, OPS-029, OPS-033,
     OPS-036, REV-001, REV-006, REV-015, TPL-011 and TPL-013.
   - **Supporting promises** are named and kept out of the counts.

   The list is MC8's input.
3. **The design is the worker's, and it comes first.** Before any run, the README states the
   model: its nodes, its script, its configurations, its properties, its variants, and the
   table of expected verdicts. Each property quotes the `when` and `then` of the promise it
   checks, with every precondition. The script must reach each property's precondition in some
   configuration, and a reachability property shows that it does. Use two parts:
   1. **Controls and revisions.** Two callers send controls to one instance, with revisions:
      - the implementation may fail, not answer, or apply a two-setting control only in part;
      - unknown outcomes, and reconciliation;
      - the instance's configuration schema relates its two settings, so that REV-013 can be
        reached;
      - one retirement of the instance at once while a control is in progress (PLC-017), so that
        REV-007's and REV-010's branches for a retired instance are reached.
   2. **Outcomes, cancellation and model work:**
      - what an implementation's answer may claim;
      - a request cancelled while in progress;
      - a filtered repeat of a succeeded operation that declares outputs, and of one that
        declares none;
      - model work started only through its own operation, including onward requests sent while
        serving an invocation of an operation marked as starting model work, and of one not
        so marked.
4. **Faults that earlier reviews found, to be avoided.** The verifier checks each of these:
   - **A property that asks more than its promise.** Examples: a keyed repeat that MC3 required
     to dispatch, and a late reply that MC3 required to replace a recorded unknown.
   - **A precondition never reached.**
   - **A variant that breaks nothing.**
   - **A liveness claim.** Check quiescent states instead.
   - **A fault with no witness.** Reordering needs two messages on one link; say which requests
     are sent without waiting.
   - **Restricting faults.** Timers and recovery pass through `fault_allowed` once the protocol's
     `gate_remaining_faults` is true, and `fault_enabled` gives windows by state. Restrict timers
     with `timer_enabled`. An implementation's silence is a model choice with its own bound.
   - **Every fault with a budget must be reached in its run**, so split faults across
     configurations as MC3 did. Every budget of zero needs a reason.
5. **Readings to apply.** These are the coordinator's readings, logged for the owner to strike.
   - **Fenced reconciliation** (ruling 75, A1, in force and spec pending). An actor records that
     a control's effect did not occur only once the invocation is fenced. Apply order 077's
     readings:
     - an invocation is fenced only when every attempt is;
     - an answer fences an attempt, and a late answer fences it but changes no outcome;
     - no answer fences nothing: a timeout, a dropped connection, a cancellation with no
       confirmation, or a restart;
     - the run's end fences;
     - work the implementation still holds after it answered is outside the model.

     Pre-register an unfenced control that follows OPS-014's present text, and report what it
     shows.
   - **A control's effect** (REV-007). A control that succeeds applied exactly the values it was
     sent, and, unless its instance was retired while it ran, exactly those values are stored.
     Log what the implementation applied, and what was stored. The revision advances exactly when
     the stored configuration changes (REV-003, REV-004), to a value it has never had (REV-005).
   - **An unknown control** leaves the stored configuration and revision unchanged, and reports
     the settings it addressed as unknown (REV-009).
   - **Reconciling a control** (REV-010, REV-013). Quote REV-010 in full. A reconciliation that
     records the control as having taken effect, on an instance that exists and is not retired,
     is a request to change the instance: it stores the values sent, applied to the current
     configuration, and is judged by REV-002 and REV-011 like any other. One that records it did
     not take effect stores nothing. On a retired or removed instance it stores nothing either
     way. REV-013's invalid is reached only after another change has been stored, under the
     schema that relates the two settings.
   - **Contention** (REV-014). A setup, retirement or change of an instance that arrives while
     another has been dispatched and has not ended is answered conflict at once, and is not
     dispatched. An unknown outcome ends it. The rule does not apply to a repeat answered
     without a dispatch, and refused or invalid come first under OPS-023.
   - **What an outcome claims** (OPS-001 to OPS-004). These bind what an implementation reports,
     not a reconciled outcome:
     - succeeded means the effect occurred;
     - failed means it did not occur, in whole or in part;
     - unknown is reported where the implementation cannot tell, or knows the effect occurred
       only in part.

     Keep a ledger of the effect, outside every node, to check these.
   - **Cancellation** (OPS-027). Closing a request's stream cancels it, and the engine passes the
     cancellation on to the implementation. The outcome is then unknown, unless the
     implementation confirms that nothing was done. A confirmation is a result stating failed
     that arrives before the end is recorded; one that arrives later changes nothing (OPS-013).
   - **A repeat answered conflict** (OPS-034). Quote OPS-034's `when` in full: it is a repeat of
     a succeeded invocation of an operation that declares outputs. For an operation that
     declares none, OPS-009 answers succeeded instead. Model both, and the branch with no filter
     in which the outputs are no longer kept.
   - **Model work** (IMP-001, IMP-002, IMP-008):
     - model work starts only as the effect of an operation that declares it, dispatched through
       an open Conduit listing that operation;
     - nothing in IMP-002's list starts it, an engine restart and a hook's dispatch included;
     - a request for an operation marked as starting model work, made on behalf of an invocation
       of an operation not so marked, is refused. One made on behalf of a marked invocation is
       not refused for that reason.
6. **The ledger, the properties, the variants and the table.** As in MC2, MC3, MC4 and MC5a:
   - every property reads only the ledger, logged when each event is sent;
   - each safety property has at least one broken variant;
   - the table gives every property's expected verdict and every reachability property's
     expected reach, for each configuration and variant;
   - the correct model runs in every configuration, and each variant only where it is needed;
   - a test checks every full run against the table;
   - one refutation run per variant prints its first trace.

   When a run disagrees with the table, keep the table unchanged and add the observed one beside
   it. When a property breaks on the model as written, first check decision 4's first fault. If
   the property asks more than its promise, report a finding of class ambiguity against this
   order's wording, and add a control, registered before its first run. Otherwise report a
   finding of class contradiction or gap, with the shortest trace. Do not change the model to
   avoid it.
7. **Faults.**
   - Loss and reordering on the links between the callers and the engine, and between the
     engine and the implementation.
   - The engine's crash and recovery, with its state saved.
   - The implementation's silence, as decision 4 says.
   - The engine's timer, in every configuration, so that unknown outcomes and reconciliation are
     reached under each fault.

   The other budgets are one each, split across configurations.
8. **Bounds.** Each run stays well under 200,000 unique states and under a 10-second timeout, with
   a target near half a second. Together, this order's runs should take about 15 seconds of the
   crate's test time.
9. **No change to `spec/`.** A promise that cannot hold, or that the model shows to be ambiguous,
   is a finding in your report with a counterexample.

## Scope

Change only `experiments/012-model-checking/`: the new module, `src/lib.rs`, the README, and an
addition to `src/harness.rs` that its existing tests survive, if one is needed.

Do not change any other file: not the existing models, `engine/`, `spec/`, `conformance/`,
`packages/`, `docs/`, `experiments/README.md`, any other experiment, ROADMAP.md, IN_FORCE.md or
any record or ruling. Report anything else that needs changing.

## Done when

In `experiments/012-model-checking/`, these pass:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release`, and its wall time;
- `cargo test --release -- --nocapture --test-threads=1`, for the counts.

In your report's summary, give:

- the test totals and the wall time;
- for every configuration and variant: the unique states, depth, seconds and verdicts;
- every difference from the table;
- the counts of decision 2's list: modelled, and not modelled by reason.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes; new files are untracked, and these see
them:

```bash
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" --exclude-dir=target experiments/012-model-checking
grep -rn -E "[[:space:]]+$" --exclude-dir=target --include='*.md' --include='*.rs' --include='*.toml' experiments/012-model-checking
```

`target/` stays ignored and unstaged. Do not stage anything.

## Record

Run on 2026-11-11 on the Windows computer, from a fixed base: a worker, then a fresh verifier,
which found two musts and a should:
- a property that exempted every recorded reason without checking that the implementation gave
  one;
- a cancellation check that derived its confirmation from the outcome it checked;
- no reconciliation that told a merge into the current configuration from one into the original.

A second round of 8 minutes (477 s) fixed all three. A fresh verifier in 4 minutes (215 s) found
no fault.

The order added `controls.rs`. Of the 31 promises, 18 are modelled and 13 are static decisions
left to vectors; this order's entry for REV-010 supersedes MC2's. Forty-two bounded runs finished
within their limits, the largest 19,008 states.

**No property of the current promises was found broken beyond what ruling 75 A1 already
settles.** The properties hold in all 17 correct configurations, and each of the 25 broken
variants broke its named property. The unfenced control, which follows OPS-014's present text,
reproduces MC2-001 for a control (MC7a-002): an actor records that the effect did not occur while
the attempt can still act, and the effect lands afterwards. Ruling 75 A1 settles this, and order
077 writes it into the specification. One cell differs from the table, an expectation (MC7a-001).

The lane coordinator joined the crate with the other models on main and reran it under the
pinned Rust 1.97.1, with fmt, clippy and the repository's checks, before the merge.
