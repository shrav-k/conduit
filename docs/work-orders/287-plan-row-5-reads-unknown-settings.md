# Work order 287: `conduit plan`'s row 5 reads `status.settings`, naming each setting a control left unknown with its control and invocation, after reconciliation too (order 270's follow-on)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors` is
empty: this order adds no case. It needs no network: run cargo offline, one command at a time,
and if a build asks for a crate that is not in the cache, stop and fetch nothing. Never print a
whole large file, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Do not run `conduit` outside a test,
`npm`, `node`, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere.
Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no
absolute local path, user or machine name, or private repository name.

## What and why

**Today.** [Order 270](270-inspect-unknown-settings.md) made `inspect.v1` serve
`status.settings`: under the JSON Pointer of each setting that a control whose outcome is unknown
addressed, `state` `unknown`, the control's qualified reference as `control` and the invocation's
ID as `invocation`, kept after reconciliation until a later control settles it (REV-009,
REV-010). [Order 245](245-conduit-plan.md)'s planner does not read it. Its row 5 (`unknown` in
`engine/crates/shell/src/plan/compare.rs`) takes `unknown_endings`, the `invocation-ended`
records with outcome `unknown` that `plan/read.rs` collects from each inspection, and prints, for
each setting that the file's Type says the control's `changes` writes, `KIND/NAME
setting=POINTER control=OP invocation=ID inspection=unknown-settings-not-built REV-009`. That
reading names inputs the request did not supply (REV-009 names only those), keeps a setting that a
later success settled, and loses it to a purge (REC-004). The test
`review_a_03_live_inspection_cannot_confirm_a_reconciled_control_setting`
(`engine/crates/shell/tests/plan.rs`, as order 245 merges it) asserts that line after a
reconciliation.

**What K3 says** ([the plan-and-apply reference](../implementation/plan-and-apply/README.md), as
order 245 merges it). Row 5 stops a step that would reach "an instance with a setting reported
unknown" with `stop KIND/NAME setting=POINTER state=unknown REV-009`, and prints the same cause as
a `difference` line where no step would. Step 2 reads "unreconciled invocations"; "Stops, unknown
outcomes and repeats" asks that the invocation ID be shown; the text grammar lists the CAUSE keys,
`control` among them and `invocation` not. Once the invocation is reconciled, `unreconciled` no
longer names it, so K3's line as written would name nothing that made the setting unknown. K3's
row text therefore changes (decision 1): the line gains `control=` and `invocation=`, from the
served status. No promise and nothing under `spec/` changes.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2,
every access can be inspected: plan prints what inspection serves; 6, nothing repeats an effect
implicitly: plan stops a step over a setting a control left unknown, after reconciliation too;
7, everything an agent does is attributable: the line names the control and its invocation.
Newly shown: nothing that inspection does not already show the caller, and no value. None at
risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: a Kubernetes server-side apply that meets a field another manager set
fails, naming the field and the manager (`conflict with "MANAGER": .spec.replicas`), and changes
nothing until the author says so. Not taken: reading the records, for the reasons above; K3's
line without the control and the invocation; a flag that lets apply step over the setting (row 5:
apply sends no reconciliation and no control).

**When it runs.** On `main` once order 245 merges; until then it stacks on 245's branch, whose
last commit holds the test as the merge will. It merges ahead as the S1 chain does, by the
coordinator's reading of DEC-072, which names orders 245 to 250. Through Conduit's dogfood: one
Codex worker turn, then the Codex verifier and the Claude reader at once, blind (DEC-071: it
changes a stop line and K3), one fix turn, the gate (DEC-046, DEC-033). By ruling 89, each
decision says what must hold; how is yours. The merge order, K3's words, the line's key order and
the test's name are the coordinator's, open to the owner's strike.

**Shared files.** Orders 246 to 249 change K3's README, the planner's modules and
`engine/STANDING.md` at other lines; whichever merges later rebases and runs
`promise_standing.py --write` again.

**Stop rule.** Stop, change nothing further, and report only where two promises contradict each
other, a step needs an engine change outside the scope, or a file outside Scope must change. Where
this order's wording contradicts a promise it leaves unchanged, the promise governs: follow it and
list it under `own_choices` (the coordinator's D1). A choice no decision governs is yours: list it
under `own_choices` too.

## The decisions

1. **K3's text** (`docs/implementation/plan-and-apply/README.md`). Change only these:
   - step 2: "tombstones and unreconciled invocations (OPS-036, LIF-013, LIF-040)." becomes
     "tombstones, unreconciled invocations and the settings a control left unknown (OPS-036,
     LIF-013, LIF-040, REV-009).";
   - row 5: in both its line forms, `setting=POINTER state=unknown REV-009` becomes
     `setting=POINTER state=unknown control=CONTROL invocation=ID REV-009`; the ", and print
     beside it" after the `stop` form becomes ", the setting's CONTROL and ID as the instance's
     `status.settings` gives them, before and after reconciliation (REV-010), and print beside
     it"; its last cell "OPS-012 to OPS-014, REV-009, LIF-013; ruling 78 point 5" becomes
     "OPS-012 to OPS-014, REV-009, REV-010, LIF-013; ruling 78 point 5";
   - "Stops, unknown outcomes and repeats": "the result or LIF-013's `unreconciled`," becomes "the
     result, LIF-013's `unreconciled` or, for a setting, `status.settings` (REV-009),";
   - "Text grammar and statuses": "`control`, `tombstone`," becomes "`control`, `invocation`,
     `tombstone`,".
2. **Row 5 reads the served status** (`plan/compare.rs`'s `unknown`, `plan/read.rs`,
   `plan/mod.rs`). For each object row 5 considers today, each member of the engine's
   `status.settings` whose `state` is `unknown` gives one line:

   ```text
   PREFIX KIND/NAME setting=POINTER state=unknown control=CONTROL invocation=ID REV-009
   ```

   POINTER is the member's key as served (the key `""` prints `setting=`), CONTROL its `control`,
   ID its `invocation`; PREFIX is `stop` where a step of the plan would reach the object and
   `difference` otherwise, chosen as for the `unreconciled` lines beside it. The lines need
   neither the file nor its Type's `changes`, so an object the ledger holds and the file no longer
   does gets them too. Row 5 reads no record: `ObservedObject`'s `unknown_endings` and its reading
   in `read.rs` go, and no line prints `inspection=`. The `unreconciled` lines, the other rows and
   the sorting of lines are unchanged. Change no core source: the served status is order 270's.
3. **Two tests.** Write them first, run them on the unchanged planner, and report each failing
   line.
   - In `plan/tests.rs`, a new `row_05_served_unknown_setting_holds_after_reconciliation`: a ready
     `Resource/shared` whose status is
     `{"unreconciled":[],"settings":{"/limit":{"state":"unknown","control":"example.org/control#set-limit.v1","invocation":"invocation"}}}`,
     held unchanged by the file, its `controls` empty. The plan prints `difference
     Resource/shared setting=/limit state=unknown control=example.org/control#set-limit.v1
     invocation=invocation REV-009` and no stop. With its configuration changed in the file, as
     `row_05_unknown_looks_ahead_and_preserves_other_stops` does, the same cause prints as a
     `stop` line, beside the stop the change gives. With the object gone from the file (no
     `want`), the same status prints `stop Resource/shared setting=/limit state=unknown
     control=example.org/control#set-limit.v1 invocation=invocation REV-009` beside `stop
     Resource/shared absent=file requires=--prune shared LIF-035`: `stop`, as the removal is a
     step (row 15). Today: no such line in any of the three.
   - In `tests/plan.rs`, `review_a_03_live_inspection_cannot_confirm_a_reconciled_control_setting`
     also asserts that `status.settings` is exactly
     `{"/limit":{"state":"unknown","control":"example.org/control#set-limit.v1","invocation":ID}}`,
     ID the control's invocation, before and after the reconciliation; and that the plan, with
     exit status 2 as today, prints `stop Resource/reader setting=/limit state=unknown
     control=example.org/control#set-limit.v1 invocation=ID REV-009`, and no line holding
     `inspection=` or `unreconciled=`. Keep its name and fixture, and change no other test in the
     file. Today: the plan prints the `inspection=unknown-settings-not-built` line instead.

   Run `python3 tools/promise_standing.py --write`; it may change nothing.

## What to read

K3's step 2, row 5, "Stops, unknown outcomes and repeats" and "Text grammar and statuses";
`inspect.v1`'s description and its `status.settings` schema in `spec/core/operations.yaml`;
REV-009, REV-010, OPS-013 and LIF-013; in the shell, `plan/compare.rs`'s `unknown`, `plan/read.rs`
where it builds `unknown_endings`, `plan/mod.rs`'s `ObservedObject`, `plan/tests.rs`'s `object`
and `row_05_unknown_looks_ahead_and_preserves_other_stops`, and the test in `tests/plan.rs`, by
range with `sed -n`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 238fdd9c HEAD; echo "270 $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k=docs/implementation/plan-and-apply/README.md
p=engine/crates/shell/tests/plan.rs
grep -cF 'status["settings"] = Value::Object(settings);' engine/crates/core/src/records.rs
grep -c 'inspection=unknown-settings-not-built REV-009' engine/crates/shell/src/plan/compare.rs
git grep -l 'unknown_endings' -- engine | wc -l | tr -d ' '
grep -c '^async fn review_a_03_live_inspection_cannot_confirm_a_reconciled_control_setting()' "$p"
grep -c 'get("settings").is_none()' "$p"
grep -c 'fn row_05_served_unknown_setting_holds_after_reconciliation' engine/crates/shell/src/plan/tests.rs
j "$k" 'setting=POINTER state=unknown REV-009'
j "$k" 'tombstones and unreconciled invocations (OPS-036, LIF-013, LIF-040).'
j "$k" "the result or LIF-013's \`unreconciled\`,"
j "$k" '`control`, `tombstone`,'
j "$k" 'OPS-012 to OPS-014, REV-009, LIF-013; ruling 78 point 5'
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `270 0`, `1`, `1`, `4`, `1`, `0`, `0`, `2`, `1`, `1`, `1`, `1`, `engine 0` and `0`.
The sixth line's `0` says order 245 merged as its last commit holds the test.

## Scope

Change `engine/crates/shell/src/plan/compare.rs`, `read.rs`, `mod.rs` and `tests.rs` as decisions
2 and 3 say; in `engine/crates/shell/tests/plan.rs` only the one test; K3's README as decision 1
says; and `engine/STANDING.md` only by the tool. Change nothing else: not `spec/`,
`conformance/`, the core or the loader, another shell source, K3's cases, order 245's text,
`plan_apply_support/`, a `Cargo.toml` or `Cargo.lock`. Report anything else that needs changing.

## Done when

Each line of the first group prints otherwise today and passes on a correct result; for the second
test, today is the unchanged planner under the changed test. The second group are guards, which
pass today and must still pass; run its last cargo line, the whole engine suite, once, last.

```sh
ok='^test result: ok\. 1 passed'
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k=docs/implementation/plan-and-apply/README.md
(cd engine && cargo test --offline --release -p conduit-shell --lib plan::tests::row_05_served_unknown_setting_holds_after_reconciliation 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-shell --test plan review_a_03_live_inspection_cannot_confirm_a_reconciled_control_setting 2>&1 | grep -c "$ok")  # 1; today 0
grep -cF 'setting=/limit state=unknown control=example.org/control#set-limit.v1 invocation=' engine/crates/shell/tests/plan.rs  # 1; today 0
git grep -l 'unknown-settings-not-built' -- engine | wc -l | tr -d ' '  # 0; today 2
git grep -l 'unknown_endings' -- engine | wc -l | tr -d ' '  # 0; today 4
j "$k" 'stop KIND/NAME setting=POINTER state=unknown control=CONTROL invocation=ID REV-009'  # 1; today 0
j "$k" 'difference KIND/NAME setting=POINTER state=unknown control=CONTROL invocation=ID REV-009'  # 1; today 0
j "$k" 'setting=POINTER state=unknown REV-009'  # 0; today 2
j "$k" 'the setting'"'"'s CONTROL and ID as the instance'"'"'s `status.settings` gives them, before and after reconciliation (REV-010),'  # 1; today 0
j "$k" 'OPS-012 to OPS-014, REV-009, REV-010, LIF-013; ruling 78 point 5'  # 1; today 0
j "$k" 'unreconciled invocations and the settings a control left unknown (OPS-036, LIF-013, LIF-040, REV-009).'  # 1; today 0
j "$k" 'or, for a setting, `status.settings` (REV-009),'  # 1; today 0
j "$k" '`control`, `invocation`, `tombstone`,'  # 1; today 0
```

```sh
(cd engine && cargo test --offline --release -p conduit-shell --lib plan::tests 2>&1 | grep -c '^test result: ok\. 39 passed')  # 1
(cd engine && cargo test --offline --release -p conduit-shell --test plan 2>&1 | grep -c '^test result: ok\. 13 passed')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/shell/(src/plan/(compare|read|mod|tests)\.rs|tests/plan\.rs)|engine/STANDING\.md|docs/implementation/plan-and-apply/README\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
```

Report, per decision, what changed and where; each test's failing line on the unchanged planner,
then its passing line; and every check's last line.

Stage nothing. The verifier's and the reader's turn follows; then the gate and the merge.

## What follows

Named here and not in this order:

- **Order 245's Record** says that reading `status.settings` in row 5 is a later order: this is
  that order. 245's text is not changed.
- **Found while writing:** the shell still answers a control as unfinished
  (`engine/CONFORMANCE.md`), so on a serving engine only a test fixture reaches row 5's setting
  line, and no K3 case holds one. A case waits on control dispatch, a later order's.

## Record

Run on 2026-11-23 through Conduit itself, in stage 3 of experiment 028, by its pair 1, in Pacific
time, on `main`.

**Worker-1's turns.** The first (10:21 to 10:22) stopped rightly at a file outside Scope: removing
the field decision 2 removes meant editing four test initializers that order 248 put in
`engine/crates/shell/src/plan/apply.rs`. The coordinator widened Scope to that file for those four
lines only, open to the owner's strike, and the second turn built it: row 5 now names, for each
setting the engine's status holds as unknown, its control and invocation, after reconciliation
too. Both targeted tests and both plan suites pass.

**Two blind reads** (10:37 to 10:40). The Codex verifier, `root-verifier-1.turn.v1`, passed it,
including that `apply.rs` holds only the four allowed removals. The Claude reader (`reader-1`,
while stage 3's reader is held), not told of the ruling, raised it as a must; it was settled by
that ruling. Its three shoulds and a nit (how to clear the stop, engine/CLI.md out of step,
wrapping) were applied in one fix turn (10:40 to 10:45), with engine/CLI.md in Scope for that line
only; 48 plan unit tests, 12 plan integration tests and the document checks pass. The gate's run
on the squash is the coordinator's, at the merge.
