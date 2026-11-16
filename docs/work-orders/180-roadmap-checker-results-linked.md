# Work order 180: the roadmap checker: a finished row links its results

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
It needs no cargo command; if you run one, run one at a time. Never print a whole large file or
log: `experiments/012-model-checking/README.md` has about 7,000 lines, so read it with `grep -n`
and `sed -n`. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 40 of [its orders](../proposals/full-review/orders.md))
found that the rule that evidence is kept in the tree, in an experiment's own directory with dated
results, is stated five times (four by the analyst's count, a fifth by its verifier's). Yet the
tree records at least four cases of results kept outside it before the handover, and at the
handover all of A78's results were outside too
([FR-536](../proposals/full-review/findings/G6-outside-the-tree.md#fr-536)). Its remedy is a check
at merge, not another rule. Its check 1 would have `tools/check_roadmap.py` require each register
row whose state begins "done" or "in progress" to link a file that exists. The verifier found that
this can be met by linking the order's own file, and asked for the stronger form: a done row links
a dated results file under `experiments/`. The review's order 40 carries that form, under the title
"a finished row links its results", and its done-when names done rows only. So rows in progress are
not checked: a row is in progress before it has results, as A5's stage 0 is. The review's row waits
on its order 10, which [order 161](161-a78-stage-1-evidence.md) carried out: A78's row links
experiment 028's results of 2026-11-12.

On `main` at `ec926025`, `python3 tools/check_roadmap.py` ends "Errors: 0" and checks nothing of
this. Of the register's 25 rows, 8 are done. Six of them link a dated results file in their State
cell: A70, A71, A4, A8, A79 and Experiment 023. Two do not:

- **Experiment 012** ([the register](../../ROADMAP.md#experiments-and-probes), line 76) links work
  order 104's Record and `G5.md`. The harness's results are in the experiment's README under
  "## Results" (line 108), "Measured 2026-11-11", beside the table registered before the run. Each
  later model's results are there too, under headings such as "MC2 dated results, 2026-11-11".
- **Experiment 004** (line 84, in the table of finished items) links nothing in its State. Its
  Record cell links its README, whose results of 2026-11-05 are under "## Results" (line 52).

This order adds the check, with unit tests, and fixes those two rows: it dates each README's
results heading and links that heading from the row. It moves no results out of a README. This is
the coordinator's reading of the verifier's form: dated results are a file named for them, or a
heading named for them, in a file directly inside an experiment's own directory.
[FR-537](../proposals/full-review/findings/G6-outside-the-tree.md#fr-537), which its verifier
supported, counts experiment 012's arrangement among what works: each model's dated table sits in
the README beside the table registered before its run. Moving the tables out would separate them
from their registrations. The verifier's point stands: an order's file, a Record, or any file
outside an experiment's own directory never counts.

Rows that are not done are not checked. Of those in progress, A78 and A83 already link dated
results (orders 161 and [165](165-by-rule-examples-public-routes.md)). A5, A22, A77 and
Experiment 022 do not yet. Each will need dated results under `experiments/` before its State can
say done. A77's record is in [work orders](../implementation/WORK_ORDERS.md#the-record), and
Experiment 022 links a README heading with no date. This order changes none of their rows.

## The decisions it applies

1. **Check 7: a done row of the register links its dated results.** `check()` in
   `tools/check_roadmap.py` gains a seventh check. It runs over the tables the function already
   collects as `register` (lines 225 to 226): those under "Experiments and probes" with the columns
   Item and State, the table of finished items included.
   - Which rows: a row is checked when its State cell begins with the word done (the pattern
     `done\b`), with the cell read through `plain` as check 6 reads it (lines 327 to 336).
   - When it passes: its State cell holds at least one link that counts as dated results
     (decision 2).
   - Only the State cell is read: a row says what it was done on where it says done.
   - Links that do not count: links inside inline code or an HTML comment in the cell, as for the
     checker's other links. Take them out of the cell as `prose` (lines 178 to 193) does for a
     file.
   - Rows in any other state are not checked, and neither are the rows of the order of the work or
     of the plan audit.

2. **What counts as dated results.** A link counts when each of these holds, tested in this
   order. The first test that fails gives the link's reason (decision 3).
   - It has no scheme and no host. Otherwise: `not a local link`.
   - Its path lies directly inside a directory of `experiments/` whose name `DIRECTORY` matches
     (line 66). The path is percent-decoded and resolved against the directory of the file that
     holds the table, as `links_to` resolves (lines 196 to 205).
     - A link with no path points into the file that holds the table.
     - These do not lie directly inside such a directory: `experiments/README.md`, an
       experiment's directory itself, and a file in a subdirectory such as
       `experiments/008-real-client/evidence-2026-11-09/`.
     - Otherwise: `not in a directory experiments/NNN-name/`.
   - That path is a file. Otherwise: `no such file`.
   - The file's name or the link's anchor has the dated results form. Otherwise:
     `neither its file name nor its anchor is dated results`.
     - The date shape: four digits, a hyphen, two digits, a hyphen and two digits. The calendar
       is tested next, not here.
     - The file's name: `results-`, a date of that shape, and `.md`. An optional suffix may come
       before `.md`: a hyphen, then lower-case letters, digits and hyphens, as in
       `results-2026-11-09-later-turn.md`.
     - The anchor, percent-decoded: `results-` and a date of that shape, or a text that ends in
       `-results-` and a date of that shape.
     - The file's name is tested first. When it matches, any anchor is left to `check_docs.py`.
   - The date is a date of the calendar: `datetime.date.fromisoformat` accepts it. Otherwise:
     `not a calendar date`.
   - For the anchor's form only: the anchor is one of the file's anchors, as `check_docs.py`'s
     `anchors` computes them. Otherwise: `no heading with this anchor`.

3. **The error names the row and every link it has** ([ruling 52](../proposals/ruling-52/README.md),
   point 2). Each failing row adds one line to `errors`, in table and row order. On `main`'s
   roadmap, Experiment 012's line is:

   ```text
   ROADMAP.md:76: Experiment 012 is done, and its State cell links no dated results: a file experiments/NNN-name/results-YYYY-MM-DD.md, or a heading of a file in experiments/NNN-name/ whose anchor ends in results-YYYY-MM-DD. It links docs/work-orders/104-mc1-model-checking-harness.md#record (not in a directory experiments/NNN-name/); experiments/012-model-checking/G5.md (neither its file name nor its anchor is dated results).
   ```

   The line's parts, in order:
   - The table's file, as `label` gives it, and the row's line number.
   - The item: the Item cell, with each link replaced by its text and the emphasis around it
     stripped, in its own case (`Experiment 012`, `A70`). Do not use `plain` for the item: it
     lower-cases.
   - The fixed text, up to "It links".
   - Then `nothing.` when the cell holds no link outside inline code and comments. Otherwise, each
     link in the order written, as its target with its reason in parentheses, joined by `; ` and
     ended with a full stop.

   How a target is shown:
   - A local target: its path from the root in POSIX form, with `#` and its anchor when it has
     one.
   - An external target, or one that resolves outside the root: as written.

   Experiment 004's line on `main`'s roadmap ends `It links nothing.`

4. **The check says what it read.** After the line that begins "Order of the work:", `check()`
   prints `Done rows in the register: D, L of them linking dated results.` D is the number of rows
   check 7 read, and L the number that passed. After this order, on the tree, it prints
   `Done rows in the register: 8, 8 of them linking dated results.` Nothing else that checks 1 to 6
   print changes.

5. **The code.**
   - A new function `done_without_results(register)` returns a pair: the number of rows it read,
     and decision 3's lines.
   - `check()` calls it after check 6, adds its lines to `errors` and prints decision 4's line.
   - The function reads the module's `ROOT` and `EXPERIMENTS` when it is called, so that a test
     can patch them. It uses `DIRECTORY`, `LINK` and `plain`, the last only to read the state.
   - Import `anchors` from `check_docs`, so that the two checkers read an anchor alike. Both
     scripts run from `tools/`, and `tools/tests/test_condensation_checks.py` already imports
     `check_docs`.
   - Use the standard library only, and no syntax newer than Python 3.9. CI runs the checker with
     `python3`, and `/usr/bin/python3` on the Mac is 3.9.
   - `check_docs.py` checks that each literal path in `tools/*.py` exists, except one that holds
     `NNN-name` (`tools/check_docs.py` lines 197 to 199 and 230). Write every path in a string or
     a comment of `check_roadmap.py` in that template form, as decision 3's fixed text is.

6. **Where the check is described.**
   - In the module's docstring, add item 7 after item 6 (lines 28 to 30), in the terms of
     decisions 1 and 2.
   - At the end of the docstring's paragraph on what the checker reads (lines 32 to 38), add that
     it cannot tell whether the results a done row links show what the row says.
   - In `CONTRIBUTING.md`, the bullet on `check_roadmap.py` (lines 250 to 255) changes "and that
     each state begins with an allowed word." to "that each state begins with an allowed word; and
     that each row of the roadmap's register whose state is done links its dated results in an
     experiment's directory." This keeps the bullet in the docstring's order. Rewrap that bullet,
     at its two-space indent, to at most 100 characters a line.
   - The one-line summaries at `AGENTS.md` line 126 and `CONTRIBUTING.md` line 203 do not change.

7. **The two rows, with their headings dated.**
   - `experiments/012-model-checking/README.md` line 108: `## Results` becomes
     `## MC1 dated results, 2026-11-11`, the form of the file's later headings. Its anchor is
     `mc1-dated-results-2026-11-11`. Nothing else in the file changes.
   - `experiments/004-stateright-evaluation/README.md` line 52: `## Results` becomes
     `## Results, 2026-11-05`. Its anchor is `results-2026-11-05`. Nothing else in the file
     changes, and the evidence log's "Results table above" still holds.
   - `experiments/README.md` holds the only links to those two headings. On line 24, the link to
     004's README changes its anchor from `results` to `results-2026-11-05`. On line 31, the link
     to 012's README changes its anchor from `results` to `mc1-dated-results-2026-11-11`. Nothing
     else on those lines changes. Line 31's "No Conduit promise is checked yet" is stale (FR-537)
     and is left to the review's order 36.
   - `ROADMAP.md` line 76, Experiment 012's State:
     - "done on 2026-11-11: the harness and its own model, by work order 104" becomes "done on
       2026-11-11: dated results of the harness and its own model, by work order 104".
     - "dated results" is a link to `experiments/012-model-checking/README.md` with the anchor
       `mc1-dated-results-2026-11-11`.
     - The rest of the cell does not change, its link to order 104's Record included.
   - `ROADMAP.md` line 84, Experiment 004's State:
     - "done on 2026-11-05: a tool evaluation, not evidence about a Conduit engine; it feeds A40"
       becomes "done on 2026-11-05: dated results of a tool evaluation, not evidence about a
       Conduit engine; it feeds A40".
     - "dated results" is a link to `experiments/004-stateright-evaluation/README.md` with the
       anchor `results-2026-11-05`.
     - Its Record cell does not change.

8. **Unit tests**, in a new file `tools/tests/test_roadmap.py`.
   - It imports `check_roadmap` as `tools/tests/test_model_account.py` imports its tool (lines 21
     to 22).
   - Each test writes a small tree in a temporary directory. Resolve the directory with
     `Path.resolve()`, because macOS's temporary directory lies behind a symbolic link. Patch the
     module's `ROOT` and `EXPERIMENTS` to point into the tree.
   - Each test writes `ROADMAP.md` in the tree. It holds the heading `## Experiments and probes`
     and a table with the columns Item, Question it settles, State and Record.
   - Each test reads that file with `tables`, keeps the tables whose `h2` is `REGISTER`, and
     passes them to `done_without_results`.

   The five tests:
   - `test_done_row_linking_a_dated_results_file_passes`: two done rows, both files written.
     - One links `experiments/007-sample/results-2026-11-09.md`.
     - The other links `experiments/008-sample/results-2026-11-09-later-turn.md`.
     - The function returns 2 and no line.
   - `test_done_row_linking_a_dated_results_heading_passes`: one done row.
     - It links `experiments/012-sample/README.md` with the anchor `mc1-dated-results-2026-11-11`.
     - That README holds the heading `## MC1 dated results, 2026-11-11`.
     - The function returns 1 and no line.
   - `test_done_row_without_dated_results_fails`: three done rows, each failing with one line. The
     function returns 3 and the three lines.
     - First: the Item cell links `experiments/012-sample/README.md` with the text "Experiment
       012". The State links only `docs/work-orders/104-sample.md` with the anchor `record`, a file
       written. The row's line equals decision 3's form, with the row's line number and the item
       `Experiment 012`, and ends:

       ```text
       It links docs/work-orders/104-sample.md#record (not in a directory experiments/NNN-name/).
       ```

     - Second: the State holds a link to a written dated results file, but only inside inline
       code. Its line ends `It links nothing.`
     - Third: the Record cell links a written dated results file, and the State holds no link.
       Its line ends `It links nothing.` too.
   - `test_each_link_that_does_not_count_names_its_reason`: one done row whose State holds seven
     links. Every target but the one marked not written is written. The row's one line ends with
     the seven targets, each with its reason, in this order:

     | Link in the State | Reason in the line |
     | --- | --- |
     | An external link: scheme https, host example.com, path /results-2026-11-09.md | not a local link |
     | `experiments/README.md` | not in a directory experiments/NNN-name/ |
     | `experiments/007-sample/evidence-2026-11-09/results-2026-11-09.md` | not in a directory experiments/NNN-name/ |
     | `experiments/007-sample/results-2026-11-08.md`, not written | no such file |
     | `experiments/007-sample/results-2026-13-40.md` | not a calendar date |
     | `experiments/012-sample/README.md` with the anchor `results-2026-11-05`, when that README has no such heading | no heading with this anchor |
     | `experiments/007-sample/README.md` | neither its file name nor its anchor is dated results |

   - `test_rows_not_done_are_not_checked`: six rows with no link. The function returns 0 and no
     line. Their States:
     - "in progress: stage 0"
     - "open"
     - "superseded: closed by ruling 41"
     - "dropped by ruling 45"
     - "removed: on 2026-11-05"
     - "not adopted: candidates"

9. **What stays.**
   - Checks 1 to 6, and all they print, do not change.
   - Nothing under `spec/`, `conformance/` or `engine/` changes, so rule 8 of
     [CONTRIBUTING.md](../../CONTRIBUTING.md) has nothing to apply to. Write no declaration: this
     order changes no case or fixture.
   - D13 of [the coordinator's decisions](../proposals/full-review/decisions.md) asks that a merge
     changing how orders are dispatched or checked update the method-as-run page. This order
     changes no step of dispatch, rounds, reads or merge. It adds a rule to a check that every
     order already runs, and order 173's page names the checks of AGENTS.md, not the rules of
     `check_roadmap.py`. So nothing changes there.
   - The register's opening paragraph (`ROADMAP.md` lines 53 to 55) already says where results
     go. Add no rule to the roadmap: the check, and its error, are the remedy.
   - Other orders change other rows of the same files:
     - [Order 170](170-a78-pre-registration.md) changes A78's row of `ROADMAP.md` and experiment
       028's row of `experiments/README.md`.
     - Order 179 changes A77's row of `ROADMAP.md` (line 70).

     This order changes none of those rows.

## Scope

Change only:

- `tools/check_roadmap.py` (decisions 1 to 6);
- `tools/tests/test_roadmap.py`, a new file (decision 8);
- `CONTRIBUTING.md`, the bullet of decision 6;
- `ROADMAP.md`: the State cells of Experiment 012 (line 76) and Experiment 004 (line 84);
- `experiments/README.md`, the two anchors of decision 7 (lines 24 and 31);
- `experiments/004-stateright-evaluation/README.md`, line 52;
- `experiments/012-model-checking/README.md`, line 108.

Change nothing else: no other row or section of the roadmap, nothing in `AGENTS.md`, and nothing
under `spec/`, `conformance/`, `engine/` or `docs/`.

## Done when

From the repository root, each passes:

```sh
/usr/bin/python3 -m unittest discover -s tools/tests     # OK
/usr/bin/python3 -m unittest discover -s tools/tests -p test_roadmap.py -v 2>&1 | grep -c -E '^test_(done_row_linking_a_dated_results_file_passes|done_row_linking_a_dated_results_heading_passes|done_row_without_dated_results_fails|each_link_that_does_not_count_names_its_reason|rows_not_done_are_not_checked) .* ok$'   # prints 5
python3 tools/check_roadmap.py                           # Errors: 0
/usr/bin/python3 tools/check_roadmap.py | grep -c '^Done rows in the register: 8, 8 of them linking dated results\.$'   # prints 1
grep -c '^## MC1 dated results, 2026-11-11$' experiments/012-model-checking/README.md   # prints 1
grep -c '^## Results, 2026-11-05$' experiments/004-stateright-evaluation/README.md   # prints 1
git diff --numstat -- ROADMAP.md experiments   # 2 2 for ROADMAP.md and experiments/README.md; 1 1 for each experiment's README
python3 tools/check_docs.py                              # Errors: 0
python3 tools/check_private_paths.py                     # Errors: 0
git diff --check
git status --short    # the six changed files of the scope, and tools/tests/test_roadmap.py untracked
```

Then show that the check fails on the roadmap without the two rows' links. From the repository
root, run these one at a time:

```sh
mkdir -p target/wo-180
cp ROADMAP.md target/wo-180/ROADMAP.md.fixed
git show "$(git merge-base HEAD main)":ROADMAP.md > ROADMAP.md
python3 tools/check_roadmap.py; echo "exit $?"
cp target/wo-180/ROADMAP.md.fixed ROADMAP.md
python3 tools/check_roadmap.py
```

`git merge-base HEAD main` is the commit of `main` the branch started from, whether or not a
round's changes were committed as a checkpoint. Git ignores `target/`.

With that commit's roadmap, the first run must:
- print `exit 1`;
- end `Errors: 2`;
- print `Done rows in the register: 8, 6 of them linking dated results.`;
- print two error lines, decision 3's for Experiment 012 and for Experiment 004 (lines 76 and 84
  at `ec926025`). The first is as decision 3 shows it, and the second ends `It links nothing.`
  Report both lines whole.

After the restore, the second run must end `Errors: 0`, and `git status --short` must list what it
listed above.

Do not stage anything. In your report, mark each choice you made yourself as your own. Give the
two error lines of the run that had to fail, and the last line of every check.

After the report:
- `git diff --check` and `check_private_paths.py` read tracked files only. So the admin stages the
  new test file, then runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
- On the pull request, the admin shows the `docs` job passing.
- Before it merges, a fresh Codex verifier and a fresh read-only Opus reader read the result
  ([ruling 79](../proposals/ruling-79/README.md), point 2).
