# Work order 115 (MC7b, finished): close the two checks MC7b's final verifier left open

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This order writes no conformance vector, so its `vectors` list is
empty. It has network access to fetch crates; use it for nothing else. Build with
`CARGO_BUILD_JOBS` as your environment sets it, run one cargo command at a time, and never print
a whole large file or log: use `head`, `tail`, `grep` or a count.

## What and why

Work order 113 (`docs/work-orders/113-mc7b-records-and-packages.md`) built the model of records,
observation and installing packages, `experiments/012-model-checking/src/records.rs`. Its two
rounds are spent, and its final verifier left two must-level faults in the model's own ledger
logic. Neither is in the specification. This order starts from order 113's last commit, which is
where your working tree begins, and closes the two. Order 113's decisions all still apply; read
that order first.

## The decisions it applies

1. **Fix each finding as its fix says**, with a registered variant that the corrected property
   must refute. Add both variants to `BROKEN`, so that each gets a row in the table and a
   first-refutation run, and give each an explicit target in `target()`:
   - **R4 and purge evidence** (records.rs near line 621). R4 checks only that a purge carries
     evidence, never that the evidence was saved as a record. Add `source` to `Purge`, and check
     the ledger's `Change` snapshots at that source; never read node state (order 113's rule).
     R4 checks the saved snapshot only. Whether the record can be read stays with R5 and R6,
     since REC-006 allows an unreadable or lost record, reported as a gap. Add a variant that
     makes the evidence but does not save it; its expected row on Base is HHFHH, with reach
     `+++++--`.
   - **A cached repeat's records** (near line 425). A repeat answered from the saved answer logs a
     dispatch and an end that did not happen, and omits that its request ended without a
     dispatch. Decide by whether the saved key answered the repeat (`d.key` is `Some`, near line
     407), not by the message's `repeat` flag: in ForgetIndex and PurgeIndex the key is gone,
     and the repeat really is dispatched. A fresh dispatch still logs and saves its dispatch and
     its end. A repeat answered from the key logs and saves its own occurrence, a request that
     ends without a dispatch (REC-001). Cover both cases:
     - a repeat of a succeeded invocation, where OPS-009 says nothing is dispatched;
     - a repeat of an unknown one on a plain implementation, where OPS-012 says the same.

     Add a variant that omits that record; its expected row on Base is FHHHH, with reach
     `+++++--`.
2. **Keep everything else.** Keep the pre-registered tables unchanged. Register each new variant,
   with its expected verdicts, before its first run, and report every difference. MC7b-001 and
   MC7b-002 stay as they are.
3. **The README.** Say in the new section that the earlier claim, "R4's permanent-evidence check
   reads the same storage snapshots directly", was not true until this order. Add a dated
   section after MC7b's latest results, with:
   - the two fixes and the new registrations;
   - the results of every configuration and variant whose numbers changed;
   - any difference from the tables.

## Scope

Change only `experiments/012-model-checking/src/records.rs` and the README's MC7b sections.

Do not change any other file: not the other models, `src/harness.rs`, `engine/`, `spec/`,
`conformance/`, `packages/`, `docs/`, `experiments/README.md`, ROADMAP.md, IN_FORCE.md, or any
record or ruling. Report anything else that needs changing.

## Done when

In `experiments/012-model-checking/`, these pass:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release`, and its wall time;
- `cargo test --release -- --nocapture --test-threads=1`, for the counts.

In your report's summary, give:

- the test totals and the wall time;
- for every configuration and variant whose numbers changed, the unique states, depth, seconds
  and verdicts;
- every difference from the tables.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes:

```bash
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" --exclude-dir=target experiments/012-model-checking
grep -rn -E "[[:space:]]+$" --exclude-dir=target --include='*.md' --include='*.rs' --include='*.toml' experiments/012-model-checking
```

`target/` stays ignored and unstaged. Do not stage anything.

## Record

Run on 2026-11-11 on the Windows computer, stacked on order 113's last commit (14a9665): the
worker, then a fresh verifier; one round, with no fault found. R4 now checks purge evidence
against the saved snapshots at the purge's source. A repeat answered from its key logs and saves
the end of a request with no dispatch, while a fresh dispatch logs its dispatch and its end.
Each fix has a variant that its property refutes: OmitPurgeRecord, in 8 actions, and
OmitRepeatRecord, in 14. All 53 of MC7b's full runs matched their registered verdicts. The lane
coordinator merged orders 113 and 115 together, joined the crate with the models on main, and
reran it under the pinned toolchain with the repository's checks.
