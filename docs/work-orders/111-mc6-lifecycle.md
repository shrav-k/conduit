# Work order 111 (MC6): setup, retirement, removal and Pools, model-checked

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

This order models an instance's life:
- its setup in parts, with its hooks;
- its retirement and shutdown, and its removal;
- its declared Conduit, closed and made anew;
- tombstones;
- a Pool and its members.

It covers 58 promises:

- all 33 convention-layer safety promises of `spec/conventions/lifecycle.yaml`;
- OPS-021, OPS-022 and OPS-028;
- POOL-003, POOL-007, POOL-013, POOL-015 and POOL-017;
- TPL-009, TPL-012, TPL-017 and TPL-018;
- INS-020, SUB-009, FST-002, FST-003 and FST-007;
- the eight that MC3 left to this model: ACC-005, ACC-018, ACC-039, CON-004, CON-008, CON-014,
  CON-023 and CON-031.

This time the worker designs the model's script, under the rules and readings below. Earlier
orders in this crate prescribed scripts, and their reviews found steps that made promises
unreachable. Those faults are listed in decision 4.

Read:

- experiment 012's README and `src/`, all the models in it, and the findings recorded against
  earlier orders. MC2's MC2-001, MC3's MC3-001 and MC3-002, and MC4's record of its model's own
  defects, in its section and in `docs/work-orders/108-mc4-placement-and-lost-machine.md`, are
  among them;
- the catalogue's rows for the 58 promises, and the promises themselves;
- `spec/core/operations.yaml`'s `retire-hook.v1`, `open.v1` and `stop.v1`, and the promises
  the properties lean on: OPS-012, OPS-014, OPS-024, OPS-029, IMP-007, ACC-012 and ACC-053.

## The decisions it applies

1. **Where it lives.** A new module, `experiments/012-model-checking/src/lifecycle.rs`, and its
   section in the README, placed after MC5a's. MC5b may land at the same time; its section and
   this one are joined at merge. Change `src/harness.rs` only by an addition that its
   existing tests survive.
2. **Every promise accounted for.** The README lists each of the 58 promises once, either as
   modelled, with the property that checks it, or as not modelled, with the reason.
   - **"A static decision, to be shown by vectors"** is enough for a rule with no race and no
     fault. Examples: LIF-011, LIF-013, LIF-022, LIF-034, LIF-037, SUB-009, POOL-015, FST-007,
     and FST-002 and FST-003, which S1's `bootstrap` trace tests.
   - **Slices outside the model.** Mark these as slices this model does not have:
     - TPL-017's and ACC-018's filters;
     - TPL-018's run reported unknown;
     - the cancellation slices of LIF-021 and TPL-012 (OPS-027).
   - **Supporting promises** are named and kept out of the counts.

   The list is MC8's input.
3. **The design is the worker's, and it comes first.** Before any run, the README states the
   model: its nodes, its script, its configurations, its properties, its variants, and the
   table of expected verdicts. Each property quotes the `when` and `then` of the promise it
   checks, with every precondition. The script must reach each property's precondition in some
   configuration, and a reachability property shows that it does. Use up to three parts, for
   example:
   1. I's setup, its hooks and their outcomes;
   2. retirement, shutdown, removal, the declared Conduit `c` closed and made anew, and
      tombstones;
   3. Pool P and its members.
4. **Faults that earlier reviews found, to be avoided.** The verifier checks each of these:
   - **A property that asks more than its promise.** Examples: a keyed repeat that MC3 required
     to dispatch, and a late reply that MC3 required to replace a recorded unknown.
   - **A precondition never reached.** For example, a request made after its target was removed,
     which is answered invalid under OPS-029, not the conflict the property wanted.
   - **A variant that breaks nothing,** because the correct model never reaches its case.
   - **A liveness claim.** The harness rejects Eventually, so check quiescent states instead.
   - **A fault with no witness.** Reordering needs two messages on one link: say which requests
     R sends without waiting for an answer.
   - **A fault the harness cannot restrict.** Timers and recovery do not pass through
     `fault_allowed`: restrict timers with `timer_enabled`. An implementation's silence is a
     model choice with its own bound, not a harness fault.
5. **Readings to apply.** These are the coordinator's readings of the promises, logged for the
   owner to strike.
   - **Fenced reconciliation** (ruling 75, A1, in force and spec pending). An actor records that
     an attempt's effect did not occur only once the attempt is fenced: no copy of its dispatch
     remains in flight, or the implementation confirms that it can no longer act. Model every
     reconciliation so, hooks and retirements included. Pre-register an unfenced control that
     follows OPS-014's present text, and report what it shows.
   - **`c`'s target, T.** T starts registered, and `c` is opened to it while it is still
     registered (CON-023). In some configuration, T is retired while a request to it is in
     flight (CON-008, from the moment its retirement is dispatched).
   - **Closing and remaking `c`.**
     - While I is ready, R may close `c` explicitly (ACC-012). Then:
       - setting I up again meets `c`'s closed slot (CON-004);
       - a request under LIF-009 meets `c`'s tombstone and is answered conflict (LIF-041);
       - once R lifts the tombstone (ACC-053), the request succeeds, and a further setup opens
         the new Conduit.
     - A request under LIF-009 while the Conduit made for `c` is not retired is answered conflict
       (LIF-029). Make it before I is retired.
     - CON-008 closes I's Conduits when I's shutdown is confirmed or an unregistration removes
       I.
     - Also retire I while `c`'s replacement is still registered, then ask to open it; the
       answer is conflict (CON-031).
   - **Tombstones** (LIF-041). The conflict comes only after the access decision and every check
     that gives invalid. Only `c`'s replacement can match `c`'s tombstone; a Resource's
     replacement gets a new name and matches none.
   - **A setup's or a retirement's outcome** (OPS-022). The rule counts the parts that were
     dispatched, and it applies to `retire.v1` as to `set-up.v1`. A further setup whose hook is
     unknown is unknown even if it dispatched nothing. `open.v1` has an effect, so a setup that
     opened `c` and did not succeed ends unknown, not failed.
   - **A failed retire hook.** `retire-hook.v1` says a failed hook "keeps the shutdown pending".
     OPS-022 and LIF-024 make a retirement whose hook failed, and whose stop succeeded, end
     unknown. Follow OPS-022 and LIF-024. Pre-register the other reading as a control. Report
     the difference as a finding of class contradiction, with both traces.
   - **Shutdown** (LIF-014, LIF-018, LIF-024).
     - It is pending while the latest retirement is in progress, or after it failed.
     - It is unknown where the latest retirement ended unknown.
     - It is confirmed once any retirement succeeded, or was reconciled as having occurred; an
       unknown shutdown becomes confirmed only that way.
     - A reconciliation that records that a retirement did not occur makes the shutdown pending
       only if that retirement was the latest one.
   - **LIF-025's condition** is that no setup was ever dispatched. A setup that ended unknown was
     dispatched, so the retire hook and the stop are still owed.
   - **Hooks** (OPS-021, OPS-028). An unknown hook is not dispatched again until it is
     reconciled. A failed hook's new attempt comes under OPS-024, with OPS-028's key. A
     reachability property "a hook's new attempt" means a failed hook's.
   - **Retired stays retired** (LIF-015). Set I up again after retiring it. The answer is
     conflict while I exists, and invalid once it is removed.
   - **What removal and retirement may change** (LIF-016, LIF-019). Every Conduit of which the
     instance is an end may be closed, as CON-008 states, including those closed because their
     target was retired. Nothing else that refers to an instance is changed because it was
     retired or removed.

     LIF-019 forbids retiring "any object that refers to it". Yet CON-008 and LIF-017 close a
     consumer's Conduits when it is unregistered, and closing a Conduit is retiring it. Follow
     CON-008, and report the conflict as a finding of class contradiction, with the trace.
   - **Reconciliation** (LIF-028). Reconcile a setup that ended unknown before I is retired, so
     that "makes the instance ready" is reached. Reconcile a setup, and an open, of unknown
     outcome after I is retired, so that "unless the instance has been retired" is reached too.
   - **The Pool.** Entry `e2` refers to `e1`. So P's setup sets up `e1`'s Resource first
     (POOL-013), and a retirement with its Resources retires `e2`'s Resource first (LIF-021).
     Give P and its members retire hooks that H implements, so that a member's shutdown can stay
     pending or unknown. Its script includes:
     - P set up twice;
     - a new Resource asked for while the entry's Resource is not retired (POOL-003);
     - P retired plainly while a member's shutdown is not confirmed (LIF-020);
     - P retired with its Resources (LIF-021): the members first, in the order above, and P's
       own retirement only once every member's shutdown is confirmed.

       Dispatching `retire-with-resources.v1` is not P's retirement.
     - a new Resource for an entry of P while P is retired and not yet removed (POOL-017).
   - **States are tracked per instance ID.** An instance made under LIF-009 is a new instance.
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
7. **Faults.** Loss and reordering on the links between R and the engine and between the engine
   and H; the engine's crash and recovery, with its state saved; H's silence, as decision 4
   says; and the engine's timer. Budgets of one each, split across configurations within each
   part as MC3 did, since the harness requires every fault with a budget to be reached in that
   run. Every budget of zero needs a reason.
8. **Bounds.** Each run stays well under 200,000 unique states and under a 10-second timeout,
   with a target near half a second. Together, this order's runs should take about 20 seconds
   of the crate's test time.
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

Run on 2026-11-11 on the Windows computer, from a fixed base, with the worker designing the model
under this order's checklist and readings.

The first verifier found four musts in the model's own checks:
- INS-020 compared a definition with itself;
- the order of hook and stop was not enforced;
- removal after a confirmed shutdown was not required;
- retirement at dispatch was not required.

A second round of 14 minutes (825 s) fixed them. A fresh verifier in 6 minutes (347 s) found
four more, also in the model's checks, and order 114 closed those, stacked on this order's last
commit.

**What it found in the promises:**
- **MC6-001, a contradiction.** `retire-hook.v1` says a failed hook "keeps the shutdown pending".
  But OPS-022 makes a retirement whose hook failed, and whose stop succeeded, end unknown, and
  LIF-024 then reports the shutdown unknown. The first trace on Correct/Removal/Base has 92
  actions. The opposite reading, the control HookPending, breaks LIF-024 instead.
- **MC6-002, a contradiction.** LIF-019 forbids retiring an object that refers to a removed
  instance. Yet CON-008 and LIF-017 close the Conduits whose consumer it was, and closing a
  Conduit retires it. The first trace has 147 actions. The opposite reading, the control
  PreserveReferences, breaks CON-008 instead.
- **The unfenced control** reproduces MC2-001 for hooks, which ruling 75 A1 settles.

The admin's order 081 takes MC6-001 and MC6-002 into the specification, and cites this README's
section "MC6 findings, traces and limits".
