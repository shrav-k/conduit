# Work order 169: the model-checking account's tool reads the coverage index's pieces

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
Never print a whole large file or log. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential.

## What and why

[Order 153](153-model-account-checked.md) added `tools/check_model_account.py`. Among other things,
it counts the account's `static` rows that no conformance case and no outside evidence supports.
It takes the cases from `conformance/manifest.yaml`'s `cases` list (lines 121 to 123 on `main` at
`632881df`). [Order 071](071-conformance-manifest-pieces.md) then moved every case entry out of the
manifest into the `index.yaml` file of the directory that holds the case's directory: 21 files,
494 entries. The manifest now holds only `status`, `s1_coverage` and `s1_coverage_source`, and the
conformance runner refuses a manifest that still has `cases`
(`engine/crates/conformance/src/lib.rs` lines 167 to 173).

So the tool now finds no case at all. On `main` it prints "Static rows with no manifest case and
no outside evidence: 74", where order 153's Record has 24. It still ends "Errors: 0", so CI passes
while the line is wrong. Read from the pieces, the count is 24 again. A check that finds nothing
and says nothing falls short of [ruling 52](../proposals/ruling-52/README.md), point 2. The checker
of order 168 found this on 2026-11-15.

## The decisions it applies

1. **Cases come from the index pieces.** The tool reads every file named `index.yaml` under
   `conformance/`, at any depth, in sorted path order: the same files that the runner's `paths`
   collects (`engine/crates/conformance/src/lib.rs` lines 59 to 72 and 121 to 132). Each is a
   mapping whose `cases` is a list. Each entry is a mapping whose `promises` is a list of
   promise IDs. The set of promises the static count treats as covered is the union of those
   lists. Nothing else that the tool reads or prints changes, except as decisions 2 and 3 say.
2. **What cannot be read is an error that names its file.** Each of these adds one line to the
   errors the tool prints, so the run ends with "Errors: N" for N of at least 1, and exits 1:
   - an `index.yaml` that cannot be read as YAML, or is not a mapping whose `cases` is a list of
     mappings, each with a `promises` list of strings. The line begins with the file's path from
     the repository root, such as `conformance/language/fleets/index.yaml: `, and says which of
     these is wrong.
   - `conformance/manifest.yaml` still has a `cases` key: `conformance/manifest.yaml: still has
     cases; since work order 071 a case entry belongs in the index.yaml of the directory holding
     its case directory`.
   - no `index.yaml` under `conformance/` holds a case: `conformance: no index.yaml with cases
     found, so no static row can be checked against a case`.

   The manifest is still read, but only for the second check. A missing manifest is reported as
   it is today, by the tool's `load`.
3. **The count says what it counts.** The line becomes "Static rows with no conformance case and
   no outside evidence: N". The tests quote only its end, "no outside evidence: N", which stays.
   Order 153's Record is history and does not change.
4. **G5.md says where cases are counted from.** In `experiments/012-model-checking/G5.md` (lines
   54 to 55 on `main`), "evidence outside `conformance/manifest.yaml`. The manifest's own cases
   are counted from the manifest and never copied into the account." becomes: "evidence outside
   the conformance cases. The cases are counted from the `index.yaml` files under
   `conformance/`, and never copied into the account." Rewrap only that bullet, at its two-space
   indent, to at most 100 characters a line. Nothing else in G5.md changes: order 168 changes
   other lines of that file.
5. **Tests.** In `tools/tests/test_model_account.py`:
   - The fixture's `setUp` writes its case as `conformance/language/index.yaml` with `{"cases":
     [{"promises": ["TST-001"]}]}`, and writes `conformance/manifest.yaml` as `{"status":
     "fixture"}`, with no `cases`. `test_static_evidence_counts_manifest_and_outside_separately`,
     which empties the manifest's cases to make the count 1, instead rewrites that index with one
     case naming `TST-999`, so that the run has no error.
   - New `test_cases_are_read_from_nested_index_files`: a static row whose promise is named only
     in `conformance/language/deeper/still/index.yaml` counts as covered (0 unsupported).
     Removing that file makes the count 1, as long as another index still holds a case.
   - New `test_manifest_cases_are_refused`: a manifest with `cases` fails, naming the manifest
     and order 071.
   - New `test_no_index_cases_is_an_error`: with every `index.yaml` removed, or with each one's
     `cases` empty, the tool exits 1 with the `conformance: no index.yaml` line.
   - New `test_malformed_index_names_its_file`: an `index.yaml` holding `cases: 3`, and another
     holding unparsable YAML, each fail with a line that begins with that file's path.

## Scope

- `tools/check_model_account.py`
- `tools/tests/test_model_account.py`
- `experiments/012-model-checking/G5.md`: the bullet of decision 4, nothing else.

## Done when

From the repository root, each passes:

```sh
/usr/bin/python3 -m unittest discover -s tools/tests     # OK
/usr/bin/python3 -m unittest discover -s tools/tests -p test_model_account.py -v 2>&1 | grep -c -E '^test_(cases_are_read_from_nested_index_files|manifest_cases_are_refused|no_index_cases_is_an_error|malformed_index_names_its_file) .* ok$'   # prints 4
python3 tools/check_model_account.py                     # exit 0
python3 tools/check_model_account.py | grep -c '^Static rows with no conformance case and no outside evidence: 24$'   # prints 1
/usr/bin/python3 tools/check_model_account.py --history  # exit 0
git grep -n -e 'no manifest case' -e 'counted from the manifest' -- tools experiments   # prints nothing
tr -s ' \n' '  ' < experiments/012-model-checking/G5.md | grep -c 'outside the conformance cases\. The cases are counted from the `index\.yaml` files under `conformance/`, and never copied into the account\.'   # prints 1
python3 tools/check_docs.py                              # Errors: 0
python3 tools/check_private_paths.py                     # Errors: 0
git diff --check
```

With decision 1 undone (the covered set taken from the manifest's `cases` again, the tests
kept), `test_cases_are_read_from_nested_index_files` fails. Say so in the report, with the
command you ran, then restore decision 1. Do not stage anything.

## Record

Run on 2026-11-15 on the Mac from `main` `71c725bc`: the worker (140 s) and a fresh Codex verifier
(268 s), which found nothing to change in one round but could not run the whole unit suite, since
its read-only sandbox could not create the tests' temporary fixtures. `tools/check_model_account.py`
now takes the promises it counts as covered from every `index.yaml` under `conformance/`, at any
depth and in sorted path order, rather than from the manifest's `cases`, which order 071 had
removed. A manifest that still has `cases`, an index that cannot be read or has the wrong shape, and
a tree with no index case are each an error that names its file, or `conformance` for the last. The
count line now reads "Static rows with no conformance case and no outside evidence" and is 24 again,
as in order 153's Record; before this order `main` printed 74 with "Errors: 0". G5.md's bullet says
that the cases are counted from the `index.yaml` files. Four new tests cover a nested index, refused
manifest cases, no index case and a malformed index. A fresh read-only Opus reader then verified the
order (ruling 79, point 2). It ran the whole Python suite (78 tests passed) and every Done-when
command. On scratch copies it undid decision 1 and each new check in turn, and a test failed each
time. It found no must. Both of its shoulds were applied before merge.
`test_no_index_cases_is_an_error` now requires the whole line that begins `conformance: `, so an
error naming another path fails it. The index-shape diagnostics now say what was found and then the
repair, such as `cases is a number, not a list; make cases a list of case entries` and `has no cases
key; add cases, a list of case entries`. They name what was found in YAML's terms rather than
Python's type names. The fix gave the same form to the two diagnostics about a single case, which
had the same fault, and one about a promise ID that is not a string now names its position. The
malformed-index test expects each new line, and two more shapes: a `cases` key with no value and a
fault in a second case.

A second fresh Opus reader read the coordinator's fixes (ruling 79, point 2) and found no must; its
four wording points on this Record were applied as it gave them.
