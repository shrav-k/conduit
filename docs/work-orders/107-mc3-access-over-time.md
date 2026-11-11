# Work order 107 (MC3): access over time, model-checked

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This order writes no conformance vector, so its `vectors` list is
empty. It has network access to fetch crates; use it for nothing else. The machine is shared:
build with `CARGO_BUILD_JOBS=3`, run one cargo command at a time, and never print a whole large
file or log: use `head`, `tail`, `grep` or a count.

## What and why

An order of the proof lane under [ruling 71](../proposals/ruling-71/README.md). A40 asks that each
safety promise of the shared conventions hold under model checking, or be listed with the reason
it cannot be checked this way (G5). [The safety catalogue](../implementation/SAFETY_CATALOGUE.md)
lists them, and [experiment 012](../../experiments/012-model-checking/README.md) holds the harness
and MC2's models of operation identity. This order takes access as it changes over time: a
Conduit opened and closed while requests race it, onward requests, and requests that meet a
closed Conduit, a target that is not ready, or a state that cannot be read. These are the
safety promises whose layer is convention in `spec/conventions/access.yaml`,
`spec/promises/conduits.yaml` and `spec/promises/fleets.yaml`: 44 in the catalogue's tables for
those files.

Read:

- experiment 012's README and `src/`, MC2's `operations.rs` and `mc2.rs` above all, for the form
  a model takes there;
- the catalogue's rows for the 44 promises, and the promises themselves;
- OPS-009, OPS-013, OPS-032 and IMP-007, and IFC-012, IFC-013, IFC-019 and IFC-025, under
  `spec/`;
- the traces S1-exact-access, S1-close, S1-close-in-flight, S1-authority-unreadable,
  S1-reason-order and E08-broker in
  [conformance/s1-safety-cases.md](../../conformance/s1-safety-cases.md).

## The decisions it applies

1. **Where it lives.** A new module under `experiments/012-model-checking/src/`, and its section in
   the README. The existing models stay unchanged and passing.
2. **Every promise accounted for.** The README lists each of the 44 promises once, either as
   modelled, with the property that checks it, or as not modelled, with one of these reasons:
   - "a static decision, to be shown by vectors": a rule with no race and no fault, such as the
     table of ACC-033, the bounds of ACC-029 and ACC-030, or root's reserved operations under
     ACC-032;
   - "belongs to the lifecycle model": a rule that needs setup or retirement, such as ACC-005,
     ACC-018, ACC-039, CON-004, CON-008, CON-014, CON-023 and CON-031;
   - another reason, stated. CON-011 concerns what an inspection shows, and ACC-008's filters
     are judged by the implementation (IFC-019).

   This list is MC8's input.
3. **The model.** It models the promises, not the engine's code.
   - **Nodes:**
     - A, a consumer that sends requests;
     - R, an actor that opens and closes Conduits through `access`;
     - the engine, which keeps the Conduits in saved state and decides each request;
     - T, the target's implementation;
     - U, a second target that T calls onward while serving A.
   - **Conduits:**
     - `c1`, from A to T, lists `op`, carries one filter, and starts open;
     - `c2`, from A to T, lists `op`, carries no filter, and starts registered;
     - `cT`, from T to U, lists `op2`, and starts open;
     - `cAU`, from A to U, lists `op2`, starts open and is never closed. It exists so that the
       broken variant of decision 7 has a Conduit of A's to misuse.

     U's implementation states support for `c1`'s filter, so that ACC-015 does not make every
     onward request unavailable. A closed Conduit is removed (ACC-011) and cannot be opened again
     (LIF-041), which is why `c2` exists.
   - **R's script.** R queues it all at once: it opens `c2`, then closes `c1`, then closes `cT`.
     R's link to the engine stays first in, first out, so `c2` opens before `c1` closes.
   - **A's script.** A sends `op` with key `k1` through `c1`, then `op` with key `k2` through
     `c2`. Each request carries a key: a copy that arrives later is a repeat. It is decided for
     access first, and then under OPS-032 or OPS-009.
   - **T** starts registered and becomes ready in one setup step. While serving an invocation it
     sends one onward request to U through `cT`, then answers its own outcome, whatever the
     onward answer was. A crash of T leaves T ready (LIF-011), and leaves a call dispatched to it
     with the outcome unknown, not unavailable.
   - **Dispatch** is the engine's send, made at the same step as its decision.
   - **Unknown outcomes.** The engine's timer marks an invocation still in progress unknown
     (OPS-013), and so does the engine's recovery (IMP-007).
   - **State that cannot be read.** One pair of local actions, used once, starts and ends a
     window in which the engine cannot read its Conduit state (ACC-037).
4. **The ledger.** Every property reads the ledger, never a node's own state. It logs:
   - each open and close, with the version of the Conduit state it made;
   - each decision, with the version it read, whether the state was readable, and whether T
     was ready;
   - each dispatch, and each answer a caller is given: an outcome, refused, unavailable,
     invalid, conflict or unknown;
   - each invocation's end;
   - each onward request, with the caller and the filters it carries, the Conduit it went
     through, whether any Conduit of its chain had closed, and whether the invocation it serves
     had ended.
5. **Properties.** Each is named with its promise IDs, and checked in every reachable state:
   - **no dispatch without current access** (ACC-001, ACC-011, ACC-036): a request decided while
     its Conduit is closed and the state is readable is not dispatched and is answered refused.
     Every dispatch follows a decision that read the state current at that step;
   - **the next request through an open Conduit is permitted** (CON-013): a request decided while
     its Conduit is open, the state is readable and T is ready is dispatched;
   - **in flight runs to its end** (ACC-034): once T's reply to an invocation reaches the engine,
     the answer to the request that dispatched it is T's outcome, even if its Conduit closed in
     between. A copy that arrives after the close is decided anew, and its answer is not read by
     this property;
   - **onward requests are the target's own** (ACC-016, ACC-017, ACC-019, ACC-034):
     - an onward request is dispatched only through T's own open Conduit to U;
     - it carries A's identity as caller, and `c1`'s filter when it serves a request made
       through `c1`;
     - none is dispatched once any Conduit of its chain has closed;
     - none is dispatched on behalf of an invocation that has ended;
   - **unreadable state is unavailable** (ACC-037): a request decided while the state cannot be
     read is not dispatched, and is answered unavailable;
   - **a target not ready is unavailable** (CON-024): a request through an open Conduit to T
     while T is registered is not dispatched, and is answered unavailable.

   These reachability properties stand beside the harness's own:
   - a request refused after a close;
   - an invocation answered with its outcome after its Conduit closed;
   - an onward request dispatched, and one refused after its chain closed;
   - a request answered unavailable on each of the two paths.
6. **Faults.** Add one thing to the harness: `fault_allowed` is also consulted for reordering
   and loss. MC2's hook allows them everywhere, so MC2's runs do not change; the README records
   that. Then:
   - reordering on A's link to the engine and on the links between the engine, T and U;
   - loss and duplication on A's link to the engine;
   - loss on the links between the engine and T, and between T and U;
   - the engine's crash and recovery, with its Conduits in saved state;
   - T's crash;
   - a cut between the engine and T;
   - timers, as decision 3 gives them.

   Budgets of one, and two for timers. R's link takes no reordering and no loss. Say why for
   each restriction in the README.
7. **Broken variants**, each with the property it is expected to break:
   - a decision reused from an earlier request of the same operation (no dispatch without
     current access);
   - a close that answers an in-flight invocation refused (in flight runs to its end);
   - an onward request permitted under `cAU`, A's own Conduit, after `cT` closed (onward
     requests are the target's own);
   - an onward request that checks only `cT`, not the rest of its chain (the same);
   - an onward request that drops `c1`'s filter, or names T as its caller (the same);
   - an onward request dispatched after its invocation ended (the same);
   - unreadable state read as open (unreadable state is unavailable);
   - a request dispatched to T while T is registered (a target not ready is unavailable).
8. **The table, before any run.**
   - **What it gives.** The README pre-registers one row per configuration and variant, with
     every property's expected verdict and every reachability property's expected reach. A
     variant may break more than its named property, or leave a witness unreached; the table
     says which.
   - **Where each runs.** The correct model runs in every configuration, and each variant only
     in the configuration it needs.
   - **Checks.** A test checks every full run against the table. One refutation run per variant
     prints its first trace on its named property.
   - **When a run disagrees.** Keep the table unchanged and add the observed one beside it.
     Report each difference as a finding.
   - **When a property breaks on the model as written.** Report it as a finding of class
     contradiction or gap, with the shortest trace and the promise IDs, as MC2 did with
     MC2-001. Do not change the model to avoid it. A labelled control may stand beside it.
9. **Bounds.** Each run stays well under 200,000 unique states, and under a 10-second timeout,
   which is asserted after the run. The crate's whole `cargo test --release` should stay near a
   minute on `ubuntu-latest`, which is slower than the machine here, with MC2's models included.
   If a configuration passes its bound, split it, and say which faults each part enables.
10. **The README.** The model's section comes before its results and states:
    - the nodes, the Conduits, the scripts, the configurations and every bound;
    - the list of decision 2;
    - the parts of the promises left out, with reasons;
    - the table.

    Then add dated results:
    - for every configuration and variant: the unique states, the depth, the seconds, and the
      observed verdict of every property;
    - the first trace of each refutation;
    - any difference from the table.

    Close with what the model shows and what it does not, and which of S1's traces each property
    stands beside.
11. **No change to `spec/`.** A promise that cannot hold, or that the model shows to be
    ambiguous, is a finding in your report with a counterexample.

## Scope

Change only `experiments/012-model-checking/`:

- the new module and `src/lib.rs`;
- the harness addition of decision 6 in `src/harness.rs`, with a test;
- the README.

Do not change any other file: not `retry.rs`, `operations.rs`, `revisions.rs`, `mc2.rs`,
`engine/`, `spec/`, `conformance/`, `packages/`, `docs/`, `experiments/README.md`, any other
experiment, ROADMAP.md, IN_FORCE.md or any record or ruling. Report anything else that needs
changing.

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

Run on 2026-11-11 from a fixed base: the worker in 22 minutes (1,312 s), then a fresh verifier in
5 minutes (327 s). That verifier found one must: T could recover before its crash was observed, so
the unknown outcome was skipped and onward work went on. A second round of 4 minutes (270 s)
carried the crash across recovery, with a regression that failed before the fix. A fresh verifier
in 5 minutes (328 s) then found no fault. Neither verifier could rerun the tools' unit tests in its
read-only sandbox.

The order added `access.rs`, and to the harness the `fault_allowed` check for reordering and
loss; MC2's runs are unchanged. Twenty bounded runs finished within their limits, the largest
150,702 states. Of the 44 promises, 11 are modelled. The other 33 are listed with reasons:

- 16 are static decisions, to be shown by vectors;
- 8 belong to the lifecycle model;
- 9 have reasons of their own.

**No property of the current promises was found broken.** On the correct model, two properties
broke only as this order worded them, which is the coordinator's fault, not the spec's:

- **CON-013's "the next request is permitted"** was written so that it also demanded dispatch for
  a keyed repeat, which OPS-032 rightly answers with conflict (MC3-001).
- **ACC-034's "in flight runs to its end"** was written so that a late reply replaced an outcome
  the engine had already recorded as unknown on its timer or a crash, which OPS-013 forbids
  (MC3-002, MC3-004).

The worker kept the literal properties and their traces, as decision 8 asks. It added a control,
registered before its first run, that reads ACC-034 with that precondition, and the control holds
in every configuration. Each broken variant broke its named property. Two cells differed from the
table: TargetCrash, which is MC3-004, and the unreadable path reached through a registered target
(MC3-003). The table is kept, with the observed one beside it.

The lane coordinator reran the crate's 19 tests, fmt and clippy, and the repository's checks,
before the merge. The order's two over-broad properties are noted for MC8, which states each
promise's property as the promise has it.
