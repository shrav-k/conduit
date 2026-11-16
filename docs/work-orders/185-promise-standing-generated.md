# Work order 185: each promise's standing, generated

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
It needs no cargo command; if you run one, run one at a time. Never print a whole large file or
log: the table this order generates has more than 550 lines, so read it with `grep`, `head` or
`tail`. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 43 of [its orders](../proposals/full-review/orders.md))
found that no document in the tree says, promise by promise, where the first build stands
([FR-354](../proposals/full-review/findings/P07-engine-build.md#fr-354), its conclusion).
[The first build's conformance choices](../../engine/CONFORMANCE.md) are arranged by
implementation-defined item and by order. The S1 inventory in `conformance/manifest.yaml` gives
308 promises cases or a reason and says nothing of the other 211. The promises' own `vectors`
lists are empty in all but SCH-009 and CTR-013. FR-354's verifier corrected the item's wait: the
table needs no new field, since the coverage index, with IFC-050's public `ruleId`, would do. The
P07 verifier also found specification text merged with no engine work behind it: PRV-019,
PRV-021 and PRV-030 (order 081's one run per instance), IFC-050 and ACC-056 (refusal tiers and
explicit results) and PKG-032 are named in no engine source and have no case
([what the verifier says the analyst missed](../proposals/full-review/findings/P07-engine-build.md#what-the-verifier-says-the-analyst-missed)).
On `main` at `ab07fd47` that is still true of all six, and nothing in the tree would show it
change. A record promise by promise is what A50's "recorded conformance document" will need
(LD5 in [the tracker](../AUDIT.md#layered-claim)).

This order adds `tools/promise_standing.py`. It generates one table, `engine/STANDING.md`, with a
row for each of the 519 promises, from what the tree already records: the promise files; the
coverage index's case tags, which [ruling 78](../proposals/ruling-78/README.md), point 17 (F127),
makes the one record of which cases test a promise; the expected failures that
[order 163](163-a24-at-entry.md) added; and the S1 rows' reasons. It adds one fact that no
record holds: which of the engine's crates name the promise in their sources. CI runs the
tool's `--check` on every pull request and every push to `main`, so the table cannot go stale
unseen, as FR-354 found the engine's pages had. The cost is that a later change to a promise, a
case's tags, the expected failures, an S1 row or a promise ID in those sources also runs the
tool with `--write`; `AGENTS.md` says so (decision 7), and the check's failure line says what to
run. The table sits beside `engine/CONFORMANCE.md` because LD5 keeps the conformance document
with the reference engine
([IN_FORCE.md, topic 2](../../IN_FORCE.md#2-the-specification-form-layers-and-conformance)).

The table records evidence, never that a promise is kept. A case shows only the slice of a
promise that its steps assert, and a reason is not a pass
([the S1 inventory](../../conformance/README.md#s1-whole-and-promise-inventory-i2-09)). The P07
verifier found that the evidence can show a promise broken only where a case asserts it, and
shows nothing about a promise with no case. So the table says "case passes", not "met", and
"no case, no reason", not "unbuilt". It shows the S1 inventory's disagreements as they are
(eight promises, OBJ-001 among them, have a case while their S1 row gives a reason) and
reconciles nothing.

What it leaves to other orders:

- It sets no rule and caps nothing: its check fails only when the table differs from its
  inputs. The check that promises with no case, no expected failure and no stated reason do not
  rise (D41 and F127 of [the decisions](../proposals/full-review/decisions.md)) is a later
  order's, which can use this tool's counts. An order that moves where reasons are recorded
  changes this tool with it.
- The stale lines of `engine/CONFORMANCE.md` and of the conformance README that FR-354 lists are
  the review's order 36, work order 182, written beside this one.
- It does not count the cases that assert a promise as a diagnostic's `ruleId` (IFC-050), which
  FR-354's verifier offered as a second source: that needs every case's expectations parsed,
  which is the conformance runner's work, and the tags already name each case's promises.
- It does not read the model-checking account, whose standings say what a model shows of a
  convention promise's text.
- No promise, case or fixture changes, so rule 8 of
  [Writing promises](../../CONTRIBUTING.md#writing-promises) asks nothing of this order, and it
  needs no declaration under [order 174](174-vector-changes-declared.md).

## The decisions it applies

Line numbers are those of `main` at `ab07fd47`. If a line has moved, find it by its words. The
counts quoted are the coordinator's, made on that commit by a script that applies decisions 2
and 3. Each block below is indented to sit in its list item: write its lines without that
indent.

1. **One tool, two modes.** Create `tools/promise_standing.py`, using the standard library and
   PyYAML only, so that it runs under `/usr/bin/python3` (Python 3.9). Its `main(root, argv)`
   takes the repository root and the arguments, as `check_model_account.py`'s does. Exactly one
   of `--write` and `--check` is required, through argparse's required, mutually exclusive
   group, whose own usage error exits 2 when neither or both are given.
   - `--write` writes the text of decision 4 to `engine/STANDING.md`.
   - `--check` builds the same text and compares it with the file's bytes. It writes nothing.
   - It exits 0 with no error; 1 with an error of decision 6, including a missing or stale
     table; 2 for a usage error, or when PyYAML is missing, in which case it prints
     `PyYAML is needed. NOTHING WAS READ OR WRITTEN.`
   - The script's entry runs itself again under `/usr/bin/python3` when PyYAML is missing, as
     `check_model_account.py` does at lines 297 to 304, with a guard variable of its own,
     `CONDUIT_PROMISE_STANDING_REEXEC`.
   - The paths it reads and writes are module constants, so that the tests can build a tree, as
     they use `check_model_account.SPEC_DIRS`. `check_docs.py` checks every literal repository
     path in `tools/*.py` (`path_mentions`, lines 202 to 246), so each path the tool spells out
     must exist.
2. **What it reads**, by paths from the repository root:
   - **Promises**: each `*.yaml` file directly in `spec/promises/`, `spec/conventions/` and
     `spec/interface/`. Each is a list of mappings, each with a string `id`, a `layer` that is
     `language`, `convention` or `interface`, and a string `level`. Nothing else of a promise is
     read; in particular its `vectors` list is not.
   - **Cases**: each file named `index.yaml` under `conformance/`, at any depth, in sorted path
     order, as `check_model_account.py` has read them since order 169. Each is a mapping whose
     `cases` is a list of mappings, each with a string `id` and a `promises` list of strings.
   - **Expected failures**: `conformance/expected-failures.yaml`, a mapping whose `cases` is a
     list of mappings, each with a string `id`. The `reason` is not read.
   - **S1 rows**: `s1_coverage` in `conformance/manifest.yaml`, a list of mappings, each with a
     string `promise`, a `vectors` list and, optionally, a string `reason`.
   - **Engine sources**: each file whose name ends `.rs` under `engine/crates/loader/src/`,
     `engine/crates/core/src/` and `engine/crates/shell/src/`, at any depth, read as UTF-8.
     Nothing else under `engine/` is read: not the crates' `tests/` or `examples/`, and not
     `engine/crates/conformance/`, the runner, which names promises to check cases, not to keep
     them.

   What the conformance runner already refuses stays the runner's: a tag or an S1 row naming no
   promise, an expected failure naming no case, entries out of order, a case ID given twice, a
   promise with two S1 rows and an S1 row with both cases and a reason. The tool does not report
   these. It counts what it reads as decision 3 says: each case entry counts, a promise's first
   S1 row is its row, and a row with cases reads `cases`.
3. **One row for each promise**, in byte order of ID. Its columns:
   - **ID**, **Layer** and **Level**: the promise's own.
   - **Cases**: the number of case entries whose `promises` holds the ID.
   - **Expected failures**: how many of those entries have an `id` that an entry of the expected
     failures names.
   - **S1**: `cases` when the promise's S1 row has a nonempty `vectors`; `reason` when its
     `vectors` is empty and its `reason`, stripped of whitespace, is not; `outside` when no S1
     row names the promise. A row with neither is an error of decision 6, as the runner refuses
     it too.
   - **Engine source**: the crates among `loader`, `core` and `shell`, in that order and joined
     by a comma and a space, that hold a source file of decision 2 naming the ID; `none` when
     none does. A file names the ID where the ID occurs with no ASCII letter or digit just
     before it and no ASCII digit just after it, so neither `ACC-0010` nor `XACC-001` names
     `ACC-001`. Code, strings, comments and unit tests count alike.
   - **Standing**, from Cases, Expected failures and S1 alone, the first that applies:
     1. `case passes` when Cases is greater than Expected failures;
     2. `expected failure` when Cases is at least 1 and equals Expected failures;
     3. `reason, no case` when S1 is `reason`;
     4. `no case, no reason` otherwise.

   The tool runs no case. "Case passes" rests on the runner, which fails a run in which a case
   fails that the expected failures do not list (the loop over results in
   `run_matching_with_threads`, `engine/crates/conformance/src/lib.rs` lines 324 to 362), and on
   CI's engine tests, which run every case on each pull request and each push to `main`.
4. **The file's text.** `engine/STANDING.md` is exactly the text below, with LF line ends, one
   final newline, no trailing whitespace and no date or commit, so that its bytes depend on the
   inputs alone. The numbers under Totals and the rows are computed by decision 3; everything
   else is fixed. Totals keeps its six rows and its three layer columns in this order, writing 0
   where there is none. On `main` at `ab07fd47` the text begins:

   ```markdown
   # Each promise's standing in the first build

   Not binding. The [specification](../spec/README.md) alone binds. This file is generated: do not
   edit it by hand. `python3 tools/promise_standing.py --write` writes it from the promise files
   under `spec/`, the coverage index under `conformance/` (its `index.yaml` pieces and the S1 rows
   of its `manifest.yaml`), [the expected failures](../conformance/README.md#expected-failures) and
   the sources of the crates `conduit-loader`, `conduit-core` and `conduit-shell`. CI runs
   `python3 tools/promise_standing.py --check`, which fails when this file differs from what those
   inputs give.

   It says what evidence the tree holds for each promise, not whether the first build keeps the
   promise. A case shows only the slice of a promise that its steps assert, and a reason is not a
   pass ([the S1 inventory](../conformance/README.md#s1-whole-and-promise-inventory-i2-09)). A
   source file that names a promise may keep it only in part, and code that names no promise may
   still keep one. The first build states its choices and limits in
   [its conformance choices](CONFORMANCE.md).

   ## The columns

   - **ID**, **Layer** and **Level**: the promise's own. The rows are in byte order of ID.
   - **Cases**: how many entries of the coverage index list the promise among their `promises`.
   - **Expected failures**: how many of those cases `conformance/expected-failures.yaml` lists.
   - **S1**: `cases` where the promise's row in the S1 inventory (`s1_coverage` in
     `conformance/manifest.yaml`) names cases, `reason` where it gives a reason instead, and
     `outside` where the promise has no row there.
   - **Engine source**: which of the crates `loader`, `core` and `shell` have a `.rs` file under
     their `src/` directory that names the promise's ID, in code, a string, a comment or a unit
     test alike; `none` where no such file does. The conformance runner's crate and the crates'
     `tests/` and `examples/` directories are not read.
   - **Standing**, from Cases, Expected failures and S1 alone:
     - `case passes`: at least one case lists the promise and is not an expected failure. The
       runner fails a run in which a case fails that the expected failures do not list, and CI
       runs every case on each pull request and each push to `main`.
     - `expected failure`: cases list the promise, and the expected failures list each of them.
     - `reason, no case`: no case lists the promise, and its S1 row gives a reason.
     - `no case, no reason`: no case lists the promise, and no reason is recorded.

   ## Totals

   Each number counts promises. The last row counts those whose Engine source is not `none`.

   | Promises | language | convention | interface | all |
   | --- | ---: | ---: | ---: | ---: |
   | case passes | 129 | 141 | 22 | 292 |
   | expected failure | 0 | 0 | 0 | 0 |
   | reason, no case | 57 | 15 | 2 | 74 |
   | no case, no reason | 40 | 89 | 24 | 153 |
   | all | 226 | 245 | 48 | 519 |
   | named in engine source | 140 | 155 | 22 | 317 |

   ## The promises

   | ID | Layer | Level | Cases | Expected failures | S1 | Engine source | Standing |
   | --- | --- | --- | ---: | ---: | --- | --- | --- |
   | ACC-001 | convention | must | 5 | 0 | cases | core | case passes |
   | ACC-002 | convention | must | 1 | 0 | cases | core | case passes |
   ```

   The rows go on, one for each promise, and the last on `main` is
   `| TPL-019 | language | must | 2 | 0 | cases | core | case passes |`. A promise named in two
   crates reads, for example, `| SUB-005 | language | must | 10 | 0 | cases | loader, core | case passes |`.
5. **What it prints.** In either mode, when decision 6 finds no fault in the inputs, it prints
   these three lines, with the counts it found in place of the angle brackets:

   ```text
   Read <P> promises in <F> files under spec/, <C> cases in <I> index.yaml files under conformance/, <E> entries of conformance/expected-failures.yaml, <S> S1 rows of conformance/manifest.yaml and <R> source files of the crates loader, core and shell.
   Standings: case passes <A>; expected failure <B>; reason, no case <D>; no case, no reason <N>.
   Named in the engine's source: <K> of <P>.
   ```

   Then `--write` prints `Wrote engine/STANDING.md; promise rows: <P>.` and `--check`, when the
   file's bytes equal the text, prints `engine/STANDING.md is current; promise rows: <P>.` The
   last line is always `Errors: <N>`, here `Errors: 0`.
6. **Each fault is one line that names its file** ([ruling 52](../proposals/ruling-52/README.md),
   point 2).
   - An input that cannot be read in the form of decision 2 gives a line that begins with its
     path from the repository root and `: `, says what is wrong, and says what would make it
     readable, in the manner of `check_model_account.py`'s lines for the index pieces (order
     169, decision 2; name the kind found as its `yaml_kind` does). That covers YAML that cannot
     be parsed, a value of the wrong kind, a missing key, a promise ID held twice (the line names
     both files), an S1 row with neither cases nor a reason (the line gives the row's position
     and its promise) and a source file that is not UTF-8.
   - A missing input gives one of these lines, with the crate's name in place of the angle
     brackets:

     ```text
     spec: no promise file holds a promise, so there is nothing to tabulate
     conformance: no index.yaml with cases found, so no case was counted
     conformance/expected-failures.yaml: not found; the conformance runner requires it (work order 163)
     conformance/manifest.yaml: not found, so no S1 row was read
     engine/crates/<crate>/src: not found, so no source of the crate was read
     ```

   - With any fault of the two items above, the tool prints each line, then
     `engine/STANDING.md was not written.` under `--write` or
     `engine/STANDING.md was not compared.` under `--check`, and then `Errors: N`. It writes
     nothing, compares nothing and exits 1.
   - In `--check`, after the three lines of decision 5, a missing table gives
     `engine/STANDING.md: not found; run python3 tools/promise_standing.py --write and commit it`.
     A table that differs gives
     `engine/STANDING.md: differs from what its inputs give; run python3 tools/promise_standing.py --write and commit it`,
     followed by the difference: `difflib.unified_diff` from the file's lines to the generated
     lines, with no context (`n=0`) and `lineterm=""`, labelled `engine/STANDING.md (committed)`
     and `engine/STANDING.md (generated)`. It prints at most 40 lines of the difference, then,
     if more remain, `(K more lines of difference not shown)`. Either fault counts as one error,
     and the run ends `Errors: 1`.
7. **CI and the lists of checks.**
   - In `.github/workflows/checks.yml`, the `docs` job gains a step right after the
     `check_model_account.py --history` step (lines 134 to 136), at the indentation of its
     other steps:

     ```yaml
           # engine/STANDING.md is generated from the promises, the coverage index, the expected
           # failures and the engine's sources (work order 185); it must match what they give.
           - name: python3 tools/promise_standing.py --check
             if: ${{ !cancelled() }}
             run: python3 tools/promise_standing.py --check
     ```

     No other job or step changes.
   - In `AGENTS.md`, in the block under "Run these from the repository root before each
     commit", after the `check_model_account.py` line (line 127), add this line, and leave the
     other lines as they are:

     ```text
     python3 tools/promise_standing.py --check # engine/STANDING.md agrees with the promises, the case tags, the expected failures and the engine's sources
     ```

     Under "What each check needs", after the item for `check_model_account.py` (lines 155 to
     157), add this item:

     ```text
     - `promise_standing.py` needs Python 3 and PyYAML, and runs itself again under
       `/usr/bin/python3` when its `python3` lacks PyYAML, as `check_model_account.py` does. A
       change to a promise, to a case's `promises`, to the expected failures, to an S1 row or to a
       promise ID in the sources of the loader, core or shell changes `engine/STANDING.md`: run
       the tool with `--write` in the same change, whatever files an order's scope lists, so that
       the file is committed with the change.
     ```

   - [The method as run](../implementation/METHOD_AS_RUN.md) is updated in the same pull request
     as any change to how orders are checked (D13 of
     [the decisions](../proposals/full-review/decisions.md)). Its lines 281 to 283 become these
     four lines; the line after them, which begins with `rust (directory)` in backticks, does
     not change:

     ```text
     `secrets` scans added commits; `docs` runs nine Python checks (docs, names, schemas, roadmap,
     rulings, model account with history, promise standing, private paths and, on pull requests,
     vector changes against declarations), tools' unit tests and PR `git diff --check`.
     `rust directories` discovers crates;
     ```

8. **Where the table is named.** In `engine/CONFORMANCE.md`, after the opening paragraph (lines
   3 to 10) and before `## Implemented choices`, add this paragraph, with a blank line on each
   side. Nothing else in that file changes. Work orders 182 and 183, written beside this one,
   also change this file: 182 corrects its opening paragraph (the review's order 36), and 183
   adds a section on v0's security claim (the review's order 38). Build on the file as it is on
   `main`; the admin joins the three at merge.

   ```markdown
   For each promise, [STANDING.md](STANDING.md) gives the evidence the tree holds: how many cases
   list it, how many of those are expected failures, its S1 row, and which of the engine's crates
   name it. `tools/promise_standing.py` generates it. It says what evidence there is, not that a
   promise is kept.
   ```

9. **Tests**, in a new `tools/tests/test_promise_standing.py`. Each builds a small tree in a
   temporary directory, as `tools/tests/test_model_account.py` does, and calls `main` with
   standard output captured. The fixture's `setUp` writes one promise file,
   `spec/conventions/sample.yaml`, holding TST-001 (layer `convention`, level `must`);
   `conformance/language/index.yaml` with one case, `a`, that lists TST-001; a manifest whose
   `s1_coverage` is an empty list; an expected-failures file with an empty `cases` list; and the
   three crates' `src/` directories, empty. Fixture IDs take the form `TST-001`. No test has a
   docstring: with one, `unittest -v` prints the test's name and its result on different lines,
   and the count under Done when misses it.
   - `test_each_standing_follows_its_evidence`: the promise file holds TST-001 to TST-005, each
     of the layer `convention` with level `must`. Case `a` lists TST-001; case `b` lists TST-002
     and TST-003; case `c` lists TST-003. The expected failures list `b`. TST-001's S1 row names
     `a`, and TST-004's has an empty `vectors` and a reason. `--write` exits 0, and the table
     holds exactly these rows:

     ```text
     | TST-001 | convention | must | 1 | 0 | cases | none | case passes |
     | TST-002 | convention | must | 1 | 1 | outside | none | expected failure |
     | TST-003 | convention | must | 2 | 1 | outside | none | case passes |
     | TST-004 | convention | must | 0 | 0 | reason | none | reason, no case |
     | TST-005 | convention | must | 0 | 0 | outside | none | no case, no reason |
     ```

     and the Totals rows `| case passes | 0 | 2 | 0 | 2 |`, `| expected failure | 0 | 1 | 0 | 1 |`,
     `| reason, no case | 0 | 1 | 0 | 1 |`, `| no case, no reason | 0 | 1 | 0 | 1 |`,
     `| all | 0 | 5 | 0 | 5 |` and `| named in engine source | 0 | 0 | 0 | 0 |`.
   - `test_engine_source_is_named_by_crate`: the promise file holds TST-001 to TST-004. TST-001
     is named in a comment of `engine/crates/core/src/a.rs` and in a string of
     `engine/crates/loader/src/deep/b.rs`, and its row says `loader, core`. TST-002 is named only
     in `engine/crates/core/tests/t.rs` and `engine/crates/conformance/src/lib.rs`, and its row
     says `none`. TST-003 is named in `engine/crates/shell/src/bin/main.rs`, and its row says
     `shell`. A core source holding `TST-0040` and `XTST-004` leaves TST-004's row at `none`.
     The last Totals row counts 2.
   - `test_one_row_per_promise_in_id_order`: with the fixture's promise file removed, ZZZ-001
     (layer `language`) in `spec/promises/`, AAA-002 (`convention`) in `spec/conventions/` and
     AAA-001 (`interface`) in `spec/interface/` give rows in the order AAA-001, AAA-002, ZZZ-001,
     once each, and the row `| all | 1 | 1 | 1 | 3 |`. The file ends with exactly one newline,
     no line ends in whitespace, and a second `--write` gives the same bytes.
   - `test_check_passes_after_write_and_reports_a_stale_row`: after `--write`, `--check` exits 0
     and its last two lines are `engine/STANDING.md is current; promise rows: 1.` and
     `Errors: 0`. Once the expected failures list case `a`, `--check` exits 1; it prints the
     `differs` line, a line beginning `-| TST-001 |` and one beginning `+| TST-001 |`, and ends
     `Errors: 1`; the table's bytes have not changed. With the table removed, `--check` exits 1
     with the `not found` line for `engine/STANDING.md`.
   - `test_unreadable_input_names_its_file_and_writes_nothing`: after a first `--write`, each of
     these in turn, with the rest of the tree restored: `conformance/language/index.yaml`
     holding `cases: 3`; an expected-failures file holding a list; a promise file holding YAML
     that cannot be parsed; an S1 row with an empty `vectors` and no `reason`; and
     `engine/crates/shell/src` removed. Each time, `--write` exits 1, a line of its output
     begins with that path and `: `, a line is `engine/STANDING.md was not written.`, the last
     line is `Errors: N` with N at least 1, and the table keeps the bytes the first `--write`
     gave it.
   - `test_one_mode_is_required`: `main` with neither flag, and with both, raises `SystemExit`
     with code 2, and writes no table.
10. **The table is written by the tool alone.** Write `engine/STANDING.md` only by running the
    tool with `--write`, never by hand.

## Scope

- New: `tools/promise_standing.py`, `tools/tests/test_promise_standing.py`, and
  `engine/STANDING.md`, written by the tool.
- Changed: the `docs` job of `.github/workflows/checks.yml` (decision 7); in `AGENTS.md`, one
  line and one item of the Checks section (decision 7); lines 281 to 283 of
  `docs/implementation/METHOD_AS_RUN.md` (decision 7); one paragraph added to
  `engine/CONFORMANCE.md` (decision 8).

Change nothing else: nothing under `spec/` or `conformance/`, no engine source, not the other
tools, the roadmap or the tracker. A50 stays open. `conformance/expected-failures.yaml` changes
for the check below and is restored byte for byte.

## Done when

From the repository root, each passes:

```sh
/usr/bin/python3 -m unittest discover -s tools/tests     # OK
/usr/bin/python3 -m unittest discover -s tools/tests -p test_promise_standing.py -v 2>&1 | grep -c -E '^test_(each_standing_follows_its_evidence|engine_source_is_named_by_crate|one_row_per_promise_in_id_order|check_passes_after_write_and_reports_a_stale_row|unreadable_input_names_its_file_and_writes_nothing|one_mode_is_required) .* ok$'   # prints 6
python3 tools/promise_standing.py --write | tail -n 1    # Errors: 0
python3 tools/promise_standing.py --check                # exit 0, and the five lines below
grep -c -E '^\| [A-Z]+-[0-9]{3} \|' engine/STANDING.md   # prints 519
grep -c -x -F -e '| all | 226 | 245 | 48 | 519 |' -e '| named in engine source | 140 | 155 | 22 | 317 |' engine/STANDING.md   # prints 2
grep -c -E '^\| (PRV-019|PRV-021|PRV-030|IFC-050|ACC-056|PKG-032) \| (convention|interface) \| must( not)? \| 0 \| 0 \| outside \| none \| no case, no reason \|$' engine/STANDING.md   # prints 6
python3 tools/check_schemas.py | grep -E '^7\. '         # Checked 519 promises in 27 files
/usr/bin/python3 -c 'import yaml; s = yaml.safe_load(open(".github/workflows/checks.yml"))["jobs"]["docs"]["steps"]; print([x["run"] for x in s if "promise_standing" in x.get("run", "")])'   # prints ['python3 tools/promise_standing.py --check']
python3 tools/check_docs.py                              # Errors: 0
python3 tools/set_names.py --check                       # ends Errors: 0; it reads engine/
python3 tools/check_private_paths.py                     # Errors: 0
git diff --check
grep -n -E '[[:space:]]$' tools/promise_standing.py tools/tests/test_promise_standing.py engine/STANDING.md   # prints nothing
```

On `main` at `ab07fd47`, the `--check` run prints exactly:

```text
Read 519 promises in 27 files under spec/, 500 cases in 22 index.yaml files under conformance/, 0 entries of conformance/expected-failures.yaml, 308 S1 rows of conformance/manifest.yaml and 41 source files of the crates loader, core and shell.
Standings: case passes 292; expected failure 0; reason, no case 74; no case, no reason 153.
Named in the engine's source: 317 of 519.
engine/STANDING.md is current; promise rows: 519.
Errors: 0
```

If `main` has moved past `ab07fd47`, a count here or in the `grep` lines above may differ. Then
report each difference with its cause in the inputs, and take the number of promises from
`check_schemas.py`'s step 7. `git diff --check` does not read a file that is not tracked, so the
last command checks the three new files for trailing whitespace.

Then show that the check fails when an input changes and the table does not. From the
repository root, run these one at a time:

```sh
mkdir -p target
cp conformance/expected-failures.yaml target/expected-failures.yaml.main
printf '{\n  "cases": [\n    {\n      "id": "register-agent-non-root",\n      "reason": "probe of work order 185"\n    }\n  ]\n}\n' > conformance/expected-failures.yaml
python3 tools/promise_standing.py --check; echo "exit $?"
cp target/expected-failures.yaml.main conformance/expected-failures.yaml
python3 tools/promise_standing.py --check; echo "exit $?"
git status --short
```

Git ignores `target/`. The case `register-agent-non-root` is the only case that lists ACC-057; it
also lists OPS-023, whose other cases keep it at `case passes`. The first `--check` must print
`exit 1`, the `differs` line of decision 6, and among the lines of difference these two:

```text
-| ACC-057 | convention | must | 1 | 0 | outside | core | case passes |
+| ACC-057 | convention | must | 1 | 1 | outside | core | expected failure |
```

and end `Errors: 1` before the `exit` line. After the restore, the second must end `Errors: 0`
and print `exit 0`, and `git status --short` must list exactly these seven paths:
`.github/workflows/checks.yml`, `AGENTS.md`, `docs/implementation/METHOD_AS_RUN.md` and
`engine/CONFORMANCE.md` changed, and `engine/STANDING.md`, `tools/promise_standing.py` and
`tools/tests/test_promise_standing.py` new.

Do not stage anything. In your report, mark each choice you made yourself as your own, and give
the output of the first `--check` above in full (it is short), and the last line of every check.

After the report:

- `git diff --check` and `check_private_paths.py` read tracked files only. So the admin stages
  the new files, then runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
- On the pull request, the admin shows the `docs` job passing, with its new step.
- Before it merges, a fresh Codex verifier and a fresh read-only Opus reader read the result
  ([ruling 79](../proposals/ruling-79/README.md), point 2).
