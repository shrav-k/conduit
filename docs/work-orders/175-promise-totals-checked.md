# Work order 175: the promise totals in spec/README.md checked against the promise files

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
It needs no cargo command; if you run one, run one at a time. Never print a whole large file or
log. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential.

## What and why

`spec/README.md` states the size of the catalogue twice under
[Contents](../../spec/README.md#contents). Line numbers here are those of `main` at `7346a280`.
Each promise file's row of the table gives its number of promises (lines 202 to 251, as in
"Draft; 24 promises; vectors test slices of 13 promises"). The sentence on lines 254 and 255
gives "519 in all: 226 of the layer `language`, 245 of the layer `convention` and 48 of the layer
`interface`". Every order that adds or removes a promise changes these figures by hand, and they
were recounted by hand at three rebases (the Records of orders
[075](075-spec-own-tools-reach.md#record), [038c](038c-spec-sign-in-return.md#record) and
[081](081-spec-one-run-per-instance.md#record)). Nothing compares them with the files:
`check_promises` in `tools/check_schemas.py` (step 7, lines 2248 to 2308) reads every promise file
and prints "Checked 519 promises in 27 files", but compares that count with nothing. The full
review of 2026-11-14 asks that the totals be checked by tool
([FR-274](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-274), point 2;
[FR-493](../proposals/full-review/findings/G4-who-verifies.md#fr-493), gate 6), as order 32 of
[its orders](../proposals/full-review/orders.md).

On `main`, every one of these figures agrees with the files. But the next figure typed wrong
would pass CI, which runs `check_schemas.py` and the unit tests in `tools/tests/` on every pull
request (`.github/workflows/checks.yml`, lines 125 to 127 and 144 to 146). This order adds the
comparison as the tool's step 8, so CI checks it with no change to the workflow. FR-493 asks for
totals computed rather than typed; FR-274 allows either a comparison or a generated sentence.
Order 32 checks them, so that a total typed wrong by one fails, and the tool writes nothing under
`spec/`. Each error names the number it counted, so the writer knows what to type, as
[ruling 52](../proposals/ruling-52/README.md), point 2, asks of a diagnostic.

## The decisions it applies

Line numbers are those of `main` at `7346a280`. If a line has moved, find it by its words.

1. **Step 7 passes on what it counted.** If `check_promises` recorded no error, it returns a
   pair. The first item is a dict from each promise file's path relative to `spec/`, in POSIX
   form such as `promises/objects.yaml`, to the number of promises the file holds. The second is
   a dict from each of `language`, `convention` and `interface` to the number of promises of that
   layer, with 0 for a layer that has none. If it recorded an error, it returns `None`. Nothing it
   reads, prints or records changes. In `main`, line 2712, the call becomes
   `check_promise_totals(check_promises())`.
2. **Step 8 reads the totals under Contents.** A new function `check_promise_totals(counts)`,
   placed after `check_promises`, reads `spec/README.md` from the module's `ROOT`, as
   `check_promises` does (line 2293), and takes its Contents section. The section runs from the
   line `## Contents` to the next line that begins `## `, and is matched the way `check_promises`
   matches the register's section (line 2294). If there is no line `## Contents`, step 8 records
   decision 3's first error and compares nothing. Within the section:
   - **The sentence.** Read with each run of whitespace as one space, the section must hold this
     sentence exactly once: "N in all: N of the layer `language`, N of the layer `convention` and
     N of the layer `interface`", where each N is one or more of the digits 0 to 9. The first N
     is compared with the number of promises in all, and each other N with the number for its
     layer.
   - **The rows.** A table row in the section is a row for a promise file when its first cell,
     stripped, is exactly one Markdown link whose target begins `promises/`, `conventions/` or
     `interface/` and ends `.yaml`. That target is the file's path relative to `spec/`. The row's
     last cell is its State. Split that cell at `;`. Exactly one part, stripped, must be one or
     more of the digits 0 to 9, then one space, then `promises` or `promise`. That number is
     compared with the file's count. Each file in the counts must have exactly one row. Each row
     for a promise file that states its number once is compared, even when the file has more than
     one row. Rows in the register of removed files are not rows for a promise file, because their
     first cell is a path in backticks, not a link.

   Nothing else in the section is read. In particular, the parts "vectors test slices of N
   promises" and "no vectors yet" are not compared. Line 194 calls them the recorded S1 coverage
   as of 2026-11-10, and comparing them would need a counting rule of their own, which this order
   does not set. The two figures that order 038c's rebase found stale were figures of that kind.
3. **Each difference is one error that says what was stated and what was counted.** Step 8
   records the errors through `fail`. In the messages, S is the number stated, C the number
   counted, K a number of times or rows, L a layer and P a path relative to `spec/`:

   ```text
   spec/README.md: has no line "## Contents", so the promise totals it states were not compared
   spec/README.md: under Contents, the sentence of the promise totals ("N in all: N of the layer `language`, N of the layer `convention` and N of the layer `interface`") appears K times; it must appear once
   spec/README.md: states S promises in all, and the promise files hold C
   spec/README.md: states S promises of the layer `L`, and the promise files hold C
   spec/README.md: the Contents table gives P S promises, and the file holds C
   spec/README.md: the Contents table has K rows for P; it must have one, stating the file's number of promises
   spec/README.md: the Contents table has a row for P, which is not a promise file under spec/promises/, spec/conventions/ or spec/interface/
   spec/README.md: the Contents row for P states its number of promises K times; it must state it once, as "N promises"
   ```

   If the sentence does not appear exactly once, step 8 records the second error and compares
   none of the sentence's numbers. A row for a path that is not a promise file, and a row that
   does not state its number exactly once, each give their one error and are not compared.
4. **The step says what it compared, or that it did not run.** Right after step 7's line, it
   prints the first line below. T, F, A, B and D are the numbers counted, and E is the number of
   errors step 8 recorded. If it is given `None`, it records no error and prints the second line
   instead:

   ```text
   8. Compared the promise totals that spec/README.md states with the promise files: T promises in F files, A of the layer `language`, B of the layer `convention` and D of the layer `interface`; E errors.
   8. NOT RUN: step 7 found errors in the promise files, so the promise totals that spec/README.md states were not compared.
   ```

   The tool already exits 1 for step 7's errors. When the tool stops after step 2 (lines 2392 and
   2393), neither step 7 nor step 8 runs, as now.
5. **What stays the same.**
   - `spec/README.md` does not change: on `main`, its 27 promise-file rows and its sentence agree
     with the files. If step 8 reports a difference on your base, change nothing in `spec/`, and
     report the error lines.
   - Steps 1 to 7 and their output do not change. Neither does
     `tools/tests/test_condensation_checks.py`, which calls `check_promises` alone and counts its
     errors (from line 43).
   - No other figure is compared. That covers the counts of contracts, documents, templates, files
     and items in the table's other rows, and figures outside `spec/README.md`, such as
     `docs/guides/full-review.md`, line 117.
   - The docstring's sentence on line 80, "If only jsonschema is missing it runs steps 1, 2, 5
     and 6", already leaves out step 7, and so do its copies in AGENTS.md (line 149) and
     CONTRIBUTING.md (line 210). This order does not change them; name them in your report.
   - The workflow does not change. The code must run under Python 3.9, which is what
     `/usr/bin/python3` is on the Mac, and under CI's 3.12.
6. **The docstring lists step 8.** After item 7 of "What it checks" (lines 68 to 70), add this
   item at the same indent:

   ```text
     8. The totals that spec/README.md states under Contents are those the promise files of
        step 7 hold: the number of promises in all and of each layer, in the sentence "N in all:
        N of the layer `language`, N of the layer `convention` and N of the layer `interface`",
        and the number of promises in the row of each promise file, one row for each. The
        coverage by vectors and the other figures of that table are not compared. Step 8 runs
        only when step 7 found no error.
   ```
7. **Tests**, in a new file `tools/tests/test_promise_totals.py`. It imports `check_schemas` as
   `test_condensation_checks.py` does (lines 15 to 18). It runs the steps as that file's
   `test_promise_errors_are_all_reported_across_layers_and_registers` does (from line 43): in a
   temporary root, with `ROOT` and `errors` patched and standard output captured. No test method
   has a docstring, so that `unittest -v` prints each test on one line ending in `ok`.

   The fixture has three promise files. Each promise has the seven fields and an ID of its own:
   - `spec/promises/a.yaml`, with two promises, of the layers `language` and `convention`;
   - `spec/conventions/b.yaml`, with one promise, of the layer `convention`;
   - `spec/interface/c.yaml`, with one promise, of the layer `interface`.

   The fixture's `spec/README.md` holds, in this order:
   - `## Contents`;
   - a table with one row for each file, written the way the rows of `spec/README.md` are, whose
     last cells are `Draft; 2 promises; vectors test slices of 1 promises`,
     `Draft; 1 promise; no vectors yet` and `Draft; 1 promise`;
   - the sentence "Every area has been written as promises, 4 in all: 1 of the layer `language`,
     2 of the layer `convention` and 1 of the layer `interface`.", wrapped over two lines;
   - the heading `### Promise files that rulings removed`, with its table's header row;
   - `## The form of a promise`.

   Each test calls `check_promise_totals(check_promises())` and asserts the exact list of errors:
   - `test_totals_that_agree_pass`: no error, and this line is printed: "8. Compared the promise
     totals that spec/README.md states with the promise files: 4 promises in 3 files, 1 of the
     layer `language`, 2 of the layer `convention` and 1 of the layer `interface`; 0 errors."
   - `test_each_number_wrong_by_one_fails`: one subtest for each of the seven numbers stated
     (four in the sentence, three in the rows), each raised by one. Each subtest gives exactly one
     error: the message of decision 3, with the raised number as stated and the fixture's number
     as counted.
   - `test_rows_and_sentence_must_be_there`: one subtest for each of these changes:
     - the row of `conventions/b.yaml` removed;
     - a row added for `promises/gone.yaml`, stating `Draft; 3 promises`;
     - the row of `promises/a.yaml` written twice;
     - that row's last cell changed to `Draft; vectors test slices of 1 promises`;
     - the sentence removed;
     - the line `## Contents` removed.

     Each subtest gives exactly one error: the message of decision 3 for that case.
   - `test_not_run_after_promise_errors`: the promise in `b.yaml` is given the layer `wrong`.
     `check_promises` returns `None`, the only error is step 7's, and the NOT RUN line is printed.

## Scope

- `tools/check_schemas.py`: the return value of `check_promises`, the new `check_promise_totals`,
  its call in `main`, and item 8 of the docstring.
- `tools/tests/test_promise_totals.py`, new.

The second block of checks below changes `spec/README.md` briefly and then restores it.

## Done when

From the repository root, each of these passes:

```sh
/usr/bin/python3 -m unittest discover -s tools/tests     # OK
/usr/bin/python3 -m unittest discover -s tools/tests -p test_promise_totals.py -v 2>&1 | grep -c -E '^test_(totals_that_agree_pass|each_number_wrong_by_one_fails|rows_and_sentence_must_be_there|not_run_after_promise_errors) .* ok$'   # prints 4
python3 tools/check_schemas.py | tail -n 1               # Errors: 0
python3 tools/check_schemas.py | grep -c -E '^8\. Compared the promise totals that spec/README\.md states with the promise files: [0-9]+ promises in [0-9]+ files, .*; 0 errors\.$'   # prints 1
python3 tools/check_docs.py                              # Errors: 0
python3 tools/check_private_paths.py                     # Errors: 0
git diff --check
grep -n -E '[[:space:]]$' tools/tests/test_promise_totals.py   # prints nothing
```

`git diff --check` does not read a file that is not tracked, so the last command checks the new
file for trailing whitespace.

Then show that a total wrong by one fails with the change, and passes without it. From the
repository root, run these one at a time:

```sh
mkdir -p target
cp tools/check_schemas.py target/check_schemas.py.fixed
perl -0pi -e 's/(\d+)( in all:)/($1+1).$2/e' spec/README.md
python3 tools/check_schemas.py | tail -n 2
git show HEAD:tools/check_schemas.py > tools/check_schemas.py
python3 tools/check_schemas.py | tail -n 1
cp target/check_schemas.py.fixed tools/check_schemas.py
git show HEAD:spec/README.md > spec/README.md
python3 tools/check_schemas.py | tail -n 1
git status --short
```

Git ignores `target/`. The third command raises the number of promises in all by one. The fourth
must then print "ERROR spec/README.md: states S promises in all, and the promise files hold C",
with S one more than C, followed by "Errors: 1". The sixth runs `main`'s `check_schemas.py` and
must print "Errors: 0": without the change, the wrong total passes. After the restore, the ninth
must print "Errors: 0", and `git status --short` must list only the two files in scope. Report
each of those lines.

The admin then has a fresh read-only Opus reader read the diff before it merges (ruling 79). Do
not stage anything. Report every choice you made yourself, and the last line of every check.

## Record

Run on 2026-11-15 on the Mac from `main` `5f9fcbe8`: the worker (292 s), a fresh Codex verifier (217
s) with two musts, a second worker round (191 s) and a second verifier (280 s) with two musts. Step
7, `check_promises`, now returns the number of promises in each promise file and of each layer when
it records no error, and `None` otherwise. A new step 8, `check_promise_totals`, compares them with
what `spec/README.md` states under Contents: the sentence "519 in all: 226 of the layer `language`,
245 of the layer `convention` and 48 of the layer `interface`", and the number in the State of each
promise file's row, one row for each file. Each difference is one of decision 3's errors, naming the
number stated and the number counted, and the step prints what it compared, or that it did not run
after errors in step 7. The docstring lists step 8, and `tools/tests/test_promise_totals.py` holds
decision 7's four tests and a regression for each finding below. With the change, the total in all
raised by one gives "ERROR spec/README.md: states 520 promises in all, and the promise files hold
519" and "Errors: 1"; the checker of `5f9fcbe8` passes it with "Errors: 0" (the second round
compared with `5f9fcbe8`, not `HEAD`, which already held the first round's change). The first
verifier's musts: stripping every pipe at the ends of a row dropped an empty last cell, so the cell
before it supplied the State's count; and the link's pattern missed balanced brackets in a label and
balanced parentheses in a target, so a row for a file that does not exist passed. The second round
removes at most one pipe at each end and reads the link by the depth of its delimiters. The second
verifier's musts: a link with a title or with its destination in angle brackets was not read as a
link, and splitting a row at every pipe broke it at an escaped pipe in a link's label. Either way a
row for a file that does not exist passed, and a promise file's own row was reported missing. With
the rounds spent, the coordinator finished the order: a row is split only at a pipe with no
backslash before it, as GitHub's tables are, and a link's destination is read bare or in angle
brackets, with an optional title in quotes or parentheses. Each fix has a test that fails without
it. As decision 5 directs, the sentence on the steps run without jsonschema, in the docstring,
AGENTS.md and CONTRIBUTING.md, still leaves out steps 7 and 8.

A fresh read-only Opus reader then verified the order (ruling 79, point 2). It reran the unit tests
under `/usr/bin/python3` (87 passed), `check_schemas.py` (step 8 compared 519 promises in 27 files,
with 0 errors), `check_docs.py`, `check_private_paths.py`, `git diff --check` and the demonstration
above on a copy. It removed in turn the second round's fix for an empty last cell and the
coordinator's two fixes and saw each one's test fail, tried more rows on a copy of `spec/README.md`,
and found no must. Its one should was applied before merge. A title or an angle-bracket destination
that held an unbalanced parenthesis still kept the link from being read, so a row for a file that
does not exist passed, and so did a second row for a promise file stating a wrong number. Now only
the label is read by the depth of its brackets, and the rest of the cell is the part in parentheses.
A bare destination's unescaped parentheses must balance, and a title in parentheses may hold none
unescaped, as in CommonMark. The test of angle brackets and titles adds the titles `"a (b"` and
`'x)'` and the destination `<promises/gone(.yaml>`, and each fails without the fix. Two forms are
still not read as links, so a row written in either passes unchecked: a reference-style link, and a
label with a bracket in a code span.

A second fresh Opus reader read that last fix (ruling 79, point 2) and found no must; its one
wording point on this Record was applied.
