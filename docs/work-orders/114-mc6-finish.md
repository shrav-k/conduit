# Work order 114 (MC6, finished): close the four checks MC6's final verifier left open

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This order writes no conformance vector, so its `vectors` list is
empty. It has network access to fetch crates; use it for nothing else. Build with
`CARGO_BUILD_JOBS` as your environment sets it, run one cargo command at a time, and never print
a whole large file or log: use `head`, `tail`, `grep` or a count.

## What and why

Work order 111 (`docs/work-orders/111-mc6-lifecycle.md`) built the lifecycle model,
`experiments/012-model-checking/src/lifecycle.rs`. Its two rounds are spent, and its final
verifier left four must-level faults in the model's own checks. None of them is in the
specification. This order starts from order 111's last commit, which is where your working tree
begins, and closes those four. Order 111's decisions all still apply; read that order first.

## The decisions it applies

1. **Fix each finding as its fix says**, with a regression test that fails without the fix:
   - **K binds a hook to its owner** (lifecycle.rs near line 2276). K never binds a hook's target
     instance to the setup or retirement that dispatched it, and its comparison of the selected
     implementation is tautological: `dispatch_log` assigns `selected: dispatch.instance`.
     Log the owner's saved resolved definition at Begin, and add to each hook's Dispatch the
     definition the engine selected the implementation from. Then derive the expected instance
     and implementation from the owning Begin, and compare the actual dispatch with them. Bind
     the hook's part to its request: the setup hook to a setup, the retire hook to a
     retirement. Register two controls (access.yaml: the hook goes to the implementation its
     own instance selected, and "The hook itself reaches no other instance"):
     - a wrong instance, with a self-consistent key;
     - an implementation selected from the current template's definition after
       `ReplaceTemplate`, in place of the instance's resolved one.

     `via` is always false. Either derive it from the dispatch, or record in the README that
     "not a request through any Conduit" holds by the model's structure.
   - **LIF-010's precondition** (near line 2679). The "observed unknown" witness proves only that
     an Observe request occurred. Represent an unavailable observation explicitly, as a model
     choice with no new fault or budget: an Observe that the instance's implementation does not
     answer. Record the reported unknown value and the lifecycle state before and after. Append
     a new witness that requires that precondition, as order 111's revision appended its
     witnesses, and leave witness 5 as it is.

     If the model cannot reach it, leave the pre-registered row and counts unchanged. Record the
     corrected row and counts (46 modelled, 12 not) in the new section, with the reason "a slice
     outside the model", citing MC4's property P2 for PLC-023.
   - **Pending while a retirement is active** (near line 2586). D checks the shutdown only after
     a retirement ends. Track a retirement's dispatch and its end. Check that, while the latest
     retirement is in progress, the shutdown is pending unless it is confirmed: either no setup
     was ever dispatched (LIF-025), or an earlier retirement succeeded or was reconciled as
     having occurred (LIF-024; order 111's decision 5). The events that carry a shutdown are
     Retire, State, Existence and Reconciled. Add a control that reports unknown in place of
     pending at dispatch.
   - **A negative reconciliation of the latest retirement** (near line 618). The branch that
     restores pending when the latest retirement is reconciled as not occurred is never
     reached. Add a further retirement of J under LIF-026, placed before the script's positive
     reconciliation of J. It ends unknown, and its attempt is fenced with no effect applied, so
     that "did not occur" is true by H's own ledger. Then add a fenced negative reconciliation
     of that latest retirement, a witness for that exact precondition, and a control that leaves
     the shutdown unknown. A negative record must match the effects H applied. Keep the
     existing case of an earlier retirement, and keep witness 18 reached in Removal.
2. **Keep everything else.** Keep the pre-registered tables unchanged, and keep the README's
   heading "MC6 findings, traces and limits" exactly as it is: the specification's order 081
   cites it. Register each new control
   and witness, with its expected verdicts, before its first run, as order 111 did for its
   revision. Report every difference. MC6-001 and MC6-002, the two contradictions, stay as they
   are.
3. **The README.** Add a dated section after "MC6 verifier revision results". It holds:
   - the four fixes;
   - the new registrations;
   - the results of every configuration and variant whose numbers changed;
   - any difference from the tables;
   - the accounting counts, if they change.

## Scope

Change only `experiments/012-model-checking/src/lifecycle.rs` and the README's MC6 sections.

Do not change any other file: not the other models, `src/harness.rs`, `engine/`, `spec/`,
`conformance/`, `packages/`, `docs/`, `experiments/README.md`, ROADMAP.md, IN_FORCE.md, or any
record or ruling. Report anything else that needs changing.

## Done when

In `experiments/012-model-checking/`, these pass:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release`, and its wall time;
- `cargo test --release -- --nocapture --test-threads=1`, for the counts.

In your report's summary, give the test totals and the wall time; for every configuration and
variant whose numbers changed, the unique states, depth, seconds and verdicts; every difference
from the tables; and the accounting counts, if they change. From the repository root, `python3 tools/check_docs.py`,
`python3 tools/check_rulings.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py`,
`python3 -m unittest discover -s tools/tests` (with the interpreter that has PyYAML) and
`git diff --check` pass. Run with `bash`; each prints nothing when it passes:

```bash
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" --exclude-dir=target experiments/012-model-checking
grep -rn -E "[[:space:]]+$" --exclude-dir=target --include='*.md' --include='*.rs' --include='*.toml' experiments/012-model-checking
```

`target/` stays ignored and unstaged. Do not stage anything.

## Record

Run on 2026-11-11 on the Windows computer, stacked on order 111's last commit (d347244): the
worker, then a fresh verifier in 3 minutes (196 s); one round, with no fault found. It closed
the four checks:
- a hook is bound to the setup or retirement that dispatched it, with two controls;
- LIF-010's unavailable observation is now reachable, so the accounting stays at 47 modelled
  slices and 11 static decisions;
- the shutdown is checked while a retirement is active;
- a negative reconciliation of the latest retirement is reached.

All 52 MC6 searches finished within their limits, the largest 72,666 states. MC6-001 and
MC6-002 are unchanged. The lane coordinator merged orders 111 and 114 together, joined the crate
with MC7a's work on main, and reran it under the pinned toolchain with the repository's checks.
