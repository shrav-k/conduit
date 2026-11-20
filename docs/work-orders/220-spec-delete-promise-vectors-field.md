# Work order 220: the promise's `vectors` field deleted from `spec/`, the case tags its one record

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` each case you add, remove or change, with its
promises and `passes` (this order expects none). This order needs no network: it reaches no host
and fetches no crate. Run one command at a time. Never print a whole large file, case, fixture or
log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Do not run `conduit` against a store, `codex`, `claude` or the dogfood harness,
and sign in nowhere.

## What and why

Every promise under `spec/promises/`, `spec/conventions/` and `spec/interface/` ends with a
`vectors` field. It is empty on 518 of the 520 promises. [Ruling 78](../proposals/ruling-78/README.md),
point 17 (F124 and F127), made the `promises` tags of the cases in the coverage index under
`conformance/` the one record of which cases test a promise, and left the field to be reworded.

**The owner's decision.** On 2026-11-20 the owner chose "Delete it (Rec.)" for the field. This
amends ruling 78 point 17: the field is deleted, not reworded. This order writes that decision.
It changes no behavior, no case and no promise's meaning. The owner's choice goes into the ruling
record and the ledger through the coordinator, not through this order.

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1):
none of the nine directly. The order keeps one place per fact (ruling 92 point 7). Which cases
test a promise is recorded once, in the case tags. Those tags are what coverage of every test is
judged by, and the empty field said otherwise on 518 promises.

**Should this exist** (ruling 92 point 3): no existing tool holds the form of Conduit's own
promises. The change removes a field and builds nothing.

**When it runs.** An order of A78's stage 2, sent through Conduit under
[order 211's registration](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/211-a78-stage-2-registration.md) ("Stage 2's orders"). The round is one
Codex worker turn, then one Codex verifier turn under [the standing verifier order](verify.md),
each by `conduit` call on the Mac. After them come the local gate and one fresh read (ruling 92
point 4). The verifier turn departs from IN_FORCE.md's reading of 92 point 4 (the Codex round drops
its verifier) because stage 2's orders run as order 211's registered worker and verifier pairs,
kept for that experiment's measures; this is the coordinator's dispatch, open to the owner's
strike. The coordinator commits this order and its README row on the worker's branch before
dispatch, and records the owner's choice in ruling 78's status line in the ledger in a change that
merges before or with this order. The order changes nothing under `experiments/028-a78-dogfood/`
and not the harness.

**Stop rule.** If an item needs a choice that this order does not make, stop there, change nothing
further, and report the item and the options. Do not choose.

## Before you change anything

From the repository root. If any line prints something other than the expected output below,
stop, change nothing and report:

```sh
git merge-base --is-ancestor 55d889bc HEAD; echo "main $?"
git grep -h -c -e '^  vectors:' -- spec | awk '{s+=$1} END {print s}'
git grep -h -A3 -e '^  vectors:$' -- spec | grep -c 'conformance/'
grep -c '^| `vectors` |' spec/README.md
grep -c 'vectors: \[\]' CONTRIBUTING.md
grep -c -e '"vectors"' -e 'seven fields' tools/check_schemas.py
grep -c "a promise's \`vectors\` field is reworded" IN_FORCE.md
/usr/bin/python3 -c "import json; print(sum(p in json.load(open(c))['promises'] for c, p in [('conformance/interface/schemas/schema-conflicting-defaults/case.yaml', 'SCH-009'), ('conformance/interface/schemas/schema-equal-defaults/case.yaml', 'SCH-009'), ('conformance/language/contracts/sign-in-logins/case.yaml', 'CTR-013')]))"
git status --short | wc -l | tr -d ' '
```

The expected output is `main 0`, `520`, `3`, `1`, `1`, `4`, `1`, `3` and `0`. The `520` is one
field for each promise. The `3` lines after it are the links of the only two non-empty lists:
SCH-009's (`spec/promises/schemas.yaml`) and CTR-013's (`spec/promises/contracts.yaml`).
The second `3` shows that each of those cases already tags its promise.

## The items

1. **The field goes from every promise.** Remove the `vectors` key, and its list where it has one,
   from every promise record under `spec/promises/`, `spec/conventions/` and `spec/interface/`.
   You may use a script, but the change must keep every other byte: no reflow, no reordering of
   keys, and the blank lines between records kept.
2. **The two lists' links are the cases' own tags.** The start check shows that each link's case
   and its entry in the coverage index already tag the promise, so no case changes.
3. **The form of a promise, in `spec/README.md`, "The form of a promise".** The table's row for
   `vectors` goes. Rule 5 becomes, as set wording: "5. A promise that no case's `promises` tag
   names, in the coverage index under `conformance/`, is not yet testable and does not count as
   covered. Those tags are the one record of which cases test a promise." The Contents section's
   coverage figures and its note on the coverage manifest stay as they are.
4. **`tools/check_schemas.py`** (the Python checkers are ruling 92 point 2's named exception).
   `check_promises` requires six fields, without `vectors`, and "seven" becomes "six" in its three
   places: step 7 of the module's docstring, the "must be a record with exactly seven fields"
   message and the step's printed line. A promise that still holds `vectors` reports "fields
   differ" with `extra ['vectors']`. Change no other check.
5. **The tools' tests.** In `tools/tests/test_condensation_checks.py` and
   `tools/tests/test_promise_totals.py`, the fixtures' promise records lose `"vectors": []`.
   In `test_condensation_checks.py`, add `test_promise_with_vectors_field_is_refused`. It writes
   one promise with the six fields plus `vectors: []` in a temporary tree, in the form of
   `test_promise_errors_are_all_reported_across_layers_and_registers`, runs `check_promises`, and
   asserts exactly one error that holds `fields differ` and `'vectors'`. The S1 rows'
   `vectors` in `tools/tests/test_promise_standing.py`, `conformance/manifest.yaml`,
   `tools/promise_standing.py` and the conformance crate are another field, the S1 inventory's.
   Leave them.
6. **`CONTRIBUTING.md`, "Writing promises", rule 1.** Its first sentence becomes, as set wording:
   "A promise keeps the exact form `id`, `layer`, `level`, `when`, `then`, `observable`, with
   folded `>-` text indented four spaces and lines under 97 characters; which cases test it is
   recorded only by the cases' `promises` tags." The rest of the rule stays.
7. **[IN_FORCE.md](../../IN_FORCE.md), topic 2.** The bullet starting "*Spec pending, ruling 78*:
   17 NS2's writing rules" loses ", and a promise's `vectors` field is reworded: the manifest's
   case tags are the one record of which cases test a promise" and gains "and " before "a rule is
   added", so it reads "Rule 1 (one obligation per promise) is reworded to say what a promise may
   hold, and a rule is added on what an `observable` may say." Its other sentences stay. Under
   **In spec/**, the bullet "The form of a promise, stable IDs that are never reused, and the
   register of removed promise files: spec/README.md (17 NS2, 20, 41 P2)." changes only its
   citation, which becomes "(17 NS2, 20, 41 P2; 78 point 17 as amended on 2026-11-20)". IN_FORCE
   cites and does not restate the rule.
8. **The standing table follows.** `tools/promise_standing.py` reads the case tags and the S1
   rows, not the field, so `engine/STANDING.md` stays byte for byte. Run
   `python3 tools/promise_standing.py --check`. If it reports a difference, do not regenerate:
   stop and report it.

`tools/check_model_account.py` hashes `level`, `observable`, `then` and `when` alone, so
`experiments/012-model-checking/account.yaml` does not change. If its check fails, stop and
report.

## Scope

Change the promise files under `spec/promises/`, `spec/conventions/` and `spec/interface/`,
`spec/README.md` as item 3 says, `tools/check_schemas.py`, the two test files of item 5,
`CONTRIBUTING.md`'s rule 1 and the two IN_FORCE.md lines of item 7. Change nothing under
`engine/`, `packages/`, `experiments/`, `docs/`, `conformance/`, `AGENTS.md`,
`ROADMAP.md` or any other file under `tools/`. Some documents that do not bind name the field:
merged orders, the full review's findings and ruling 78's own words. List them in your report and
do not change them.

## Done when

Each line below marked "fails today" prints otherwise on `main` at `55d889bc`. All lines pass
after the change.

```sh
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, since its tests read spec/
/usr/bin/python3 -m unittest discover -s tools/tests 2>&1 | tail -1   # OK
/usr/bin/python3 -m unittest discover -s tools/tests -k promise_with_vectors -v 2>&1 | grep -c ' ok$'  # 1; fails today (0)
git add -N spec tools CONTRIBUTING.md IN_FORCE.md  # so the checks see new files
python3 tools/check_docs.py                     # Errors: 0
python3 tools/check_rulings.py                  # passes
python3 tools/check_schemas.py | grep -c '^7\. Checked 520 promises in 27 files: six fields'  # 1; fails today (0)
python3 tools/check_schemas.py | tail -1        # Errors: 0
python3 tools/check_model_account.py            # passes
python3 tools/promise_standing.py --check       # Errors: 0
python3 tools/check_private_paths.py            # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"
# Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.  then: exit 0
git diff --check                                # prints nothing
git grep -l -e '^  vectors:' -- spec | wc -l | tr -d ' '   # 0; fails today (27)
grep -c '^| `vectors` |' spec/README.md          # 0; fails today (1)
tr -s ' \n' '  ' < spec/README.md | grep -c 'Those tags are the one record of which cases test a promise\.'  # 1; fails today (0)
grep -c 'A promise with no vector' spec/README.md  # 0; fails today (1)
grep -c 'vectors: \[\]' CONTRIBUTING.md          # 0; fails today (1)
tr -s ' \n' '  ' < CONTRIBUTING.md | grep -c 'recorded only by the cases'  # 1; fails today (0)
grep -c -e '"vectors"' -e 'seven fields' tools/check_schemas.py  # 0; fails today (4)
git grep -c '"observable": "visible", "vectors"' -- tools/tests | wc -l | tr -d ' '  # 0; fails today (2)
grep -c "a promise's \`vectors\` field is reworded" IN_FORCE.md  # 0; fails today (1)
grep -c 'may hold, and a rule is added on what an `observable` may say\. spec/README' IN_FORCE.md  # 1; fails today (0)
grep -c '41 P2; 78 point 17 as amended on 2026-11-20)' IN_FORCE.md  # 1; fails today (0)
git diff --name-only HEAD -- engine packages experiments docs conformance AGENTS.md ROADMAP.md | wc -l | tr -d ' '  # 0
git reset -q -- spec tools CONTRIBUTING.md IN_FORCE.md  # stage nothing
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML; the tool tests import the checkers, so they run under it. Your report gives the number of fields
removed and the files they were removed from, and the start check's line for item 2. It gives each
sentence you worded yourself (under `own_choices`), the vector delta's counts, the documents that
do not bind that name the field, and every check's last line. Stage nothing. The coordinator runs
`tools/gate.py`, then has a fresh, read-only Opus reader read the diff
([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4), and merges.

## Record

Not yet run.
