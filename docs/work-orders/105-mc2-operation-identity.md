# Work order 105 (MC2): operation identity, unknown outcomes and revisions, model-checked

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This order writes no conformance vector, so its `vectors` list is
empty. It has network access to fetch crates; use it for nothing else. The machine is shared:
build with `CARGO_BUILD_JOBS=3`, run one cargo command at a time, and never print a whole large
file or log: use `head`, `tail`, `grep` or a count.

## What and why

An order of the proof lane under [ruling 71](../proposals/ruling-71/README.md). A40 asks that the
safety promises hold under model checking in small configurations, with loss, reordering, a
partition and a lost machine (G5). Ruling 47 makes that a condition of v0. Order 104 built the
harness, [experiment 012](../../experiments/012-model-checking/README.md), and tested it on a
model of its own. This order adds the first models of Conduit's promises: operation identity,
at-most-once dispatch, unknown outcomes and their reconciliation, and revisions.

Experiment 004 modelled the same group on older text. Its third rule set, inspect then reissue,
applied an effect twice, because the original request could still arrive after the provider said
it had not. The promises as written may allow the same path: OPS-014 lets an actor record that an
unknown effect did not occur, and OPS-024 then dispatches a repeat of the failed invocation again.
Whether the checker finds that path is this order's first question.

Read:

- experiment 012's README and `src/`, all of it: the rules every model follows are there;
- [experiment 004](../../experiments/004-stateright-evaluation/README.md), its results and findings
  above all;
- these promises under `spec/`:
  - in `spec/conventions/operations.yaml`: OPS-008 to OPS-014, OPS-018, OPS-024, OPS-030 and
    OPS-032;
  - in `spec/conventions/implicit.yaml`: IMP-007;
  - in `spec/conventions/revisions.yaml`: REV-002, REV-010, REV-011 and REV-012;
- the traces S1-key-scope, S1-unknown-write and S1-reconcile in
  [conformance/s1-safety-cases.md](../../conformance/s1-safety-cases.md);
- `docs/implementation/SAFETY_CATALOGUE.md`, if order 100 has filled it in by the time you
  read it.

## The decisions it applies

1. **Where it lives.** New model modules under `experiments/012-model-checking/src/`, and their
   sections in its README. The harness changes only as decision 6 allows. `retry.rs` and its
   tests stay unchanged and keep passing.
2. **The operation-identity model.** It models the promises, not the engine's code.
   - **Nodes.** A sending instance A, the engine and one implementation, with one caller and one
     target. A second sending instance B appears only in the scope configuration of decision 8.
   - **A's script.** A sends these as scripted local actions, in this order: `(k, c1)`; its
     repeats of `(k, c1)` on its timer; then `(k, c2)`; then two requests with no key and equal
     contents. In the scope configuration, B sends `(k, c1)`, which is a separate invocation
     under OPS-008.
   - **The engine** keeps its index of invocations in saved state (OPS-008), and decides each
     request:
     - new: it is dispatched;
     - a repeat of one that succeeded: OPS-009;
     - different contents: OPS-010;
     - no key: OPS-011;
     - a repeat of one whose outcome is unknown: OPS-012;
     - a repeat of one that failed: OPS-024;
     - a repeat of one that has not ended: OPS-032.
   - **Unknown outcomes.** An outcome becomes unknown on a lost reply, on an implementation that
     crashed or was lost, on the engine's own timer, or when the engine recovers with an
     invocation in progress (IMP-007). A reply that arrives after the outcome became unknown
     changes nothing (OPS-013).
   - **Reconciliation** is an atomic local action of the engine, standing for an authorized
     actor's `reconcile.v1` (OPS-014, OPS-030). The actor's check reads the ledger's effects at
     that moment.
   - **The implementation** is plain, applying every dispatch it receives, or deduplicating by
     the invocation's identity across attempts. Each is a configuration.
3. **The revisions model.** The engine and the implementation of one instance, and one caller.
   - **The script.** Two changes naming revision 0 are sent together, then one naming none,
     then one naming the revision the first success produced.
   - **"In flight"** means dispatched and not yet ended in the engine's record. A change whose
     outcome becomes unknown has ended (REV-012).
   - **"Current"** means the engine's stored revision, as logged at dispatch.
   - **Faults.** Loss, reordering, the engine's crash and recovery, and timers are in. Setups,
     retirements and REV-010's reconciliation of a control are left out, and the README says
     why.
4. **The ledger.** Every property reads the ledger and nothing else, so that a broken variant
   cannot hide its own failure behind the engine's state. The ledger logs:
   - each dispatch, with its identity, contents and attempt, and the contents of the first
     dispatch of each identity;
   - each effect applied, counted per keyed identity, with each request that had no key as an
     entry of its own;
   - each outcome the engine decides, or a reconciliation records, with the effect count at that
     moment;
   - each answer a caller is given: an outcome, conflict, invalid or unknown;
   - for revisions: the revision a change named, the engine's stored revision at its dispatch,
     and each change's end.
5. **Properties.** Each is named with its promise IDs, and checked in every reachable state:
   - **at most once** (OPS-018, OPS-008, OPS-012, OPS-024): no keyed identity's effect is applied
     twice;
   - **no wrong outcome** (OPS-012, OPS-014): no outcome is decided as succeeded while its
     identity's effect count is 0 at that moment, or as failed while it is 1 or more. An effect
     that lands after a reconciliation recorded "failed" does not break this property, since
     OPS-014 allows that record; "at most once" covers what follows;
   - **unknown stays unknown** (OPS-013, IMP-007): an outcome that became unknown changes only by
     a reconciliation or by a new attempt under OPS-012 or OPS-024;
   - **a repeat is answered and not dispatched** (OPS-009, OPS-032): a repeat of a succeeded
     invocation is not dispatched and is answered with that invocation's outcome; a repeat of an
     unfinished one is not dispatched and is answered conflict;
   - **different contents conflict** (OPS-010): such a request is never dispatched, and is
     answered conflict;
   - **revision checked** (REV-002, REV-011): a change is dispatched only when it names the
     engine's stored revision at dispatch. A stale revision is answered conflict, and no
     revision is answered invalid;
   - **one change at a time** (REV-012): never two changes of one instance in flight.

   These reachability properties stand beside the harness's own:
   - **keyless requests stay apart** (OPS-011): the two keyless requests give two effects on
     some path;
   - **reconciliation happens**: a reconciliation to succeeded, and one to failed, each on some
     path;
   - **each source of unknown occurs**.
6. **Faults, and the hook that restricts them.** Add one hook to the harness's `Protocol` trait,
   with a default body that allows every fault, so that `retry.rs` needs no change:
   `fn fault_allowed(&self, fault: Fault, src: usize, dst: usize) -> bool { true }`. The harness
   consults it for duplication, cut, crash and a lost machine; the node fault kinds ignore `dst`.
   Add a test of the hook. The operation-identity model restricts faults as follows:
   - **duplication** only on links from a sending instance to the engine. OPS-018's "when" names
     requests that the sender's side loses, delays, reorders or repeats. The engine's dispatch is
     taken to ride a transport that delivers at most once. That is a part left out, and the
     README says so;
   - **cut** only between the engine and the implementation;
   - **crash** for the engine and the implementation;
   - **a lost machine** for the implementation only.

   Every kind of fault still has a budget of at least one in some configuration.
7. **What is expected, and a candidate finding.** Before any run, the README pre-registers a
   table. It has one row per variant and configuration, and gives every property's expected
   verdict. Its first rows are these:
   - **The model as the promises state it, with a plain implementation**: "at most once" is
     expected to be refuted. On the engine's timer the outcome becomes unknown while the
     dispatch is still on its link. The actor reads no effect and reconciles to failed (OPS-014).
     The dispatch then lands, and A's repeat is dispatched again (OPS-024). This is experiment
     004's inspect-then-reissue, reached through OPS-014.
   - **The same, with a deduplicating implementation**: every property is expected to hold.
   - **A control**: a reconciliation to failed is allowed only when no copy of the dispatch
     remains on any link and the implementation can no longer apply it. This is stronger than
     OPS-014, and the README says so. With a plain implementation, "at most once" is expected to
     hold.

   If the checker refutes "at most once" on the model as the promises state it, report it as a
   finding of class contradiction against OPS-014, OPS-024 and OPS-018, with the shortest trace.
   It is a candidate unsafe finding for the admin. Do not change the model to avoid it.
8. **Broken variants.** Each safety property has at least one variant, and each variant has a
   named property that it is expected to break:
   - a repeat of an unknown invocation dispatched again to a plain implementation, experiment
     004's blind replay (at most once);
   - an index kept only in volatile state, so that the engine's crash loses it (at most once);
   - "failed" decided on the engine's timer with no check (no wrong outcome);
   - a late reply accepted after the outcome became unknown (unknown stays unknown);
   - repeats matched on the key alone, with contents not compared (different contents conflict);
   - a repeat of a succeeded invocation dispatched again (a repeat is answered and not
     dispatched);
   - a change dispatched with no revision check (revision checked);
   - a second change of one instance dispatched while one is in flight (one change at a time).

   A variant may break more than its named property. The pre-registered table says which, and
   the README amends the harness's rule "refutes only that property" for models whose variants
   carry such a table.
   - **Checks.** A test checks each full run against the table. One refutation run per variant,
     on its named property, prints the first trace.
   - **When a run disagrees with the table**, keep the pre-registered table unchanged, add the
     observed table beside it, and report each difference as a finding.
   - **The scope configuration** adds B, and checks that A's and B's `(k, c1)` are separate
     invocations: both dispatch, and neither is answered as a repeat.
9. **Bounds.**
   - **Budgets** of one, timers two, and `messages_per_link` of three.
   - **Each run** stays under about 200,000 unique states and a 10-second timeout.
   - **The crate's whole `cargo test --release`** should stay near a minute on
     `ubuntu-latest`.
   - **Splitting.** If a configuration passes its bound, split it. Say which faults each part
     enables, so that across the parts every fault reaches every property it bears on.
   - **Reachability.** Each reachability property must be reached in every configuration that
     enables what it needs.
10. **The README.** One section per model, before the results, states:
    - the model, its nodes, its script, its configurations and every bound;
    - the parts of the promises left out, with reasons: OPS-008's full list of what the index
      keeps, OPS-026's defaults behind "contents", filters under OPS-024, setups and
      retirements, REV-010, and a wire copy of a dispatch;
    - the table of expected verdicts.

    Then add dated results:
    - for every configuration and variant: the unique states, the depth, the seconds, and the
      observed verdict of every property;
    - the first trace of each refutation;
    - any difference from the table.

    Close with what the models show and what they do not, and which of S1's traces each
    property stands beside.
11. **The catalogue.** If `docs/implementation/SAFETY_CATALOGUE.md` has been filled in when you
    read it, check each promise this order models against it. Report any it does not list as a
    safety promise. Otherwise report that it was a placeholder.
12. **No change to `spec/`.** A promise that cannot hold, or that a model shows to be ambiguous,
    is a finding in your report, with a counterexample. The model follows the promises as
    written; the controls of decision 7 stand beside it, labelled.

## Scope

Change only `experiments/012-model-checking/`:

- new modules under `src/`;
- `src/lib.rs`;
- the hook of decision 6 in `src/harness.rs`, with its test;
- the README.

Do not change any other file: not `src/retry.rs`, `engine/`, `spec/`, `conformance/`,
`packages/`, `experiments/README.md`, any other experiment, ROADMAP.md, docs/AUDIT.md, IN_FORCE.md
or any record or ruling. Report anything else that needs changing.

## Done when

In `experiments/012-model-checking/`, these pass:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release`, and its wall time;
- `cargo test --release -- --nocapture --test-threads=1`, for the counts.

Say what you measured and on what. In your report's summary, give:

- the test totals and the wall time;
- for every configuration and variant: the unique states, depth, seconds and verdicts;
- every difference from the pre-registered table.

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

Run on 2026-11-11 from a fixed base: the worker in 17 minutes (1,013 s), then a fresh verifier in
8 minutes (483 s). That verifier found one must: a lost reply to a deduplicated retry could leave
an invocation in progress for ever. It also found one should: the keyless witness could count one
effect for both requests. A second round of 5 minutes (319 s) fixed both, with regressions. A
fresh verifier in 5 minutes (307 s) then found no fault, and reproduced all 40 state counts and
11 traces. Neither verifier could rebuild in its read-only sandbox.

The order added `operations.rs`, `revisions.rs` and `mc2.rs`, and the `fault_allowed` hook in the
harness; `retry.rs` is unchanged. Forty bounded runs finished within their limits, the largest
13,145 states, and every enabled witness was reached.

The candidate is confirmed. With a plain implementation, "at most once" is refuted in all seven
fault configurations, timers alone included, while the other four operation properties hold.
With a deduplicating implementation, and in the fenced control, all five hold. The shortest
trace, seven actions:

1. A's request is delivered and dispatched.
2. A's timer sends its repeat.
3. The engine's timer marks the outcome unknown while the dispatch is still on its link.
4. An authorized actor reads no effect and reconciles to failed (OPS-014).
5. A's repeat arrives, and OPS-024 dispatches it as a new attempt.
6. The first dispatch lands.
7. The second dispatch lands: one keyed effect applied twice, against OPS-018.

This is experiment 004's inspect-then-reissue reached through the current promises. It needs no
wire copy of a dispatch, no fabricated evidence and no lost saved state. The worker reported it
as MC2-001, of class contradiction. It goes to the admin as a spec question for the owner.

The broken variants each broke their named property. Two broke one more than the table
predicted: BlindReplay and VolatileIndex also broke "unknown stays unknown". The pre-registered
table is kept, with the observed one beside it (MC2-002, MC2-003). The revision model holds
REV-002, REV-011 and REV-012, and each mutation is caught.

The lane coordinator reran the crate's 15 tests, fmt and clippy before the merge. It checked
that the sixteen promises the order names are safety promises in the catalogue, which merged
after the worker read it. In the same pull request it moved IFC-007 in the catalogue, by the
admin's decision. It also ran the repository's checks. No time on `ubuntu-latest` is measured,
since CI is paused.
