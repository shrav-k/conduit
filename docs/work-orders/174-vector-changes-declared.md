# Work order 174: vector changes checked against the order's declaration

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none:
its tests make temporary Git repositories on the machine. It needs no cargo command; if you run
one, run one at a time. Never print a whole large file or log: one comparison below prints more
than 1,500 lines, so write it to a file under `target/` and read only the lines this order names.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 29 of [its orders](../proposals/full-review/orders.md))
found that whether every vector change was meant is checked by hand. `tools/vector_delta.py`
compares the cases and their fixtures between two revisions and matches each change against an
order's declaration, but no CI job runs it
([FR-493](../proposals/full-review/findings/G4-who-verifies.md#fr-493), gate 5; its verifier
found nothing under `.github/workflows/` that names it, and its corrections concern other gates).
A pull request that changes a vector with no declaration, or with a declaration never committed,
merges with every check green.

Three things in the tool stand in the way of a gate:

- It exits 1 on a declared flip (line 397), as
  [order 036k](../implementation/WORK_ORDERS.md#order-036k) decided (its decision 2) so that a
  person would look at each one. Run on the merge of order 042, `57813f06`, with that order's own
  `042-declared.yaml`, it reports 1,521 changes, 4 flips and 0 undeclared, and exits 1. A fresh
  Opus reader now reads each order before it merges
  ([ruling 79](../proposals/ruling-79/README.md), point 2), so the tool lists the flips for that
  reader instead of failing on them.
- `--declared` takes one file, while a pull request that merges more than one order's work can
  carry more than one declaration.
- Nothing says which declarations belong to a pull request. The 13 files under
  `docs/work-orders/` named `*-declared.yaml` are earlier orders', and none of them may declare a
  later change.

This order has CI read only the declarations a pull request adds or changes, and fail on any
vector change that none of them declares. Each output says what it read and what it found
([ruling 52](../proposals/ruling-52/README.md), point 2). The review's row has this order wait
for order 152 (its order 3), merged at `bd987017`. Under D11 of
[the coordinator's decisions](../proposals/full-review/decisions.md) the owner is asked later to
make `docs` a required check; this order changes no repository setting. D13 there asks that a
merge that changes how orders are checked update the method-as-run page in the same pull request
([FR-536](../proposals/full-review/findings/G6-outside-the-tree.md#fr-536)). That page is order
173's, dispatched beside this one, and is not on `main`. Until it is,
[work orders](../implementation/WORK_ORDERS.md) says how orders are checked, so this order adds
its sentence there. The admin carries that sentence into order 173's page at whichever of the two
merges second. Order 173 also changes other parts of that file. Orders 163 and 166, not yet
merged, change other sections of `conformance/README.md`, and order 157 writes
`157-declared.yaml`. Build on `main` as it is when you start.

## The decisions it applies

Line numbers are those of `main` at `7346a280`. If a line has moved, find it by its words.

1. **A declared flip passes, and every flip is listed.** `main()` in `tools/vector_delta.py`
   exits 0 when no change is undeclared and no error is reported, whatever the number of flips;
   1 when any change is undeclared or any error is reported (an error is never declared, line
   362); and 2, as now, for bad arguments, revisions or declarations. After the change lines it
   prints one line for each change of kind `flip`, in the same order:
   `Flip: CASE step N PATH: BEFORE -> AFTER; declared by NAME.`, or, where no shape matched,
   `Flip: CASE step N PATH: BEFORE -> AFTER; undeclared.` CASE, N and PATH are the change's
   `case`, `step` and `path`. BEFORE and AFTER are its `before` and `after` written as the change
   lines write values (`json.dumps` with `sort_keys=True, ensure_ascii=True`), or the word
   `absent` where the change has none. NAME is the declaration's name written the same way, so in
   double quotes. What the tool compares, the change lines and their keys do not change.
2. **`--declared` may be given more than once**, through argparse's `append` (line 380). Shapes
   are read file by file in the order given and matched in that order, so the first matching
   shape still names the declaration (lines 358-373). A declaration file that cannot be read or is
   invalid still ends the run with exit 2 and `Vector delta: comparison failed.` (lines 386-389),
   and its line on standard error now names it: `vector_delta: PATH: MESSAGE`, with PATH as given.
3. **`--declared-changed` reads the declarations the comparison adds or changes.** With it, the
   tool also reads each file directly in `docs/work-orders/` whose name ends `-declared.yaml`, is
   on AFTER's side, and is absent from BEFORE's side or differs from it in bytes. A side's files
   are those of its revision. For `WORKTREE` they are the files on disk that Git does not ignore,
   whether tracked or not, which is how the cases are listed now (lines 130-136). Each is read
   from AFTER's side. A declaration that AFTER keeps unchanged or deletes, and one in a deeper
   directory, is not read: a declaration already on `main` does not declare a later change. These
   files come after any `--declared` files, in sorted path order. One that cannot be read or is
   invalid fails as decision 2 says, with PATH from the repository root.
4. **The output names what it read.** After the flip lines, the tool prints one line for each
   declaration file it read, in the order read: `Declarations read from PATH: N shapes.`, with
   PATH as decisions 2 and 3 give it. When `--declared-changed` finds no file, the tool prints
   `No docs/work-orders/*-declared.yaml is added or changed from BEFORE to AFTER.` after any lines
   for `--declared` files. BEFORE and AFTER are the two revisions as the command line gave them,
   or their defaults (`origin/main`, `WORKTREE`). The summary stays the last line, in its form of
   line 396, so every done-when that quotes it still holds. With neither option and no flip, the
   output is what it is today.
5. **CI runs it on every pull request.** In `.github/workflows/checks.yml`, the `docs` job gains a
   last step after `git diff --check` (lines 147-150), at the indentation of its other steps:

   ```yaml
         # Every vector change must match a declaration that the pull request adds or changes
         # under docs/work-orders/ (the full review's FR-493, gate 5). It runs on every pull
         # request, since some cases read fixtures under engine/ and spec/.
         - name: python3 tools/vector_delta.py HEAD^1 HEAD --declared-changed
           if: ${{ !cancelled() && github.event_name == 'pull_request' }}
           run: python3 tools/vector_delta.py HEAD^1 HEAD --declared-changed
   ```

   The job already has the whole history (line 106), and Python 3.12 with PyYAML 6.0.1 (lines
   112-118). On a pull request the checkout's first parent is the base (line 147). The review's
   row asks for the step only on pull requests that touch `conformance/`. It runs on all of them,
   because five fixtures that cases read lie outside that directory: three under
   `engine/examples/s1/` and two under `spec/schemas/examples/`. On the Mac, a comparison that
   finds no change takes about half a second, and the comparison of order 042's merge, with its
   1,521 changes, about 8 seconds. No other job or step changes.
6. **The texts say so.** Rewrap only the paragraphs changed, at most 100 characters a line. The
   texts quoted below with `>` go in as plain text, without the `>`.
   - `HELP` in `tools/vector_delta.py` (lines 20-54) states decisions 1 to 4: the output's order
     (change lines, flip lines, declaration lines, summary), the three exit statuses without the
     sentence "A declared flip STILL exits 1 for review.", that `--declared` may be repeated, and
     what `--declared-changed` reads. `--declared-changed` gets a one-line argparse `help`. In
     this file, name the declarations only by the pattern `docs/work-orders/*-declared.yaml`.
     `tools/check_docs.py` checks each literal repository path in the Python files of `tools/`
     (`PATH_MENTION` and `path_mentions`, lines 197-244), and a glob passes there only while a
     file matches it.
   - In `conformance/README.md`, section "Reviewing vector changes" (lines 202-248):
     - The sentence of line 210 that begins "Each change is a JSON output line" gives the
       output's order.
     - The sentence of line 223 says that `--declared` may be given more than once.
     - The exit sentence of lines 246-247 states decision 1's statuses.
     - After line 248, add a paragraph that says four things:
       - `--declared-changed` reads each `docs/work-orders/*-declared.yaml` that AFTER adds or
         changes from BEFORE, after any `--declared` file.
       - On a pull request, CI's `docs` job runs
         `python3 tools/vector_delta.py HEAD^1 HEAD --declared-changed`, on every pull request,
         since fixtures may lie outside this directory.
       - An order that changes a case or a case's fixture writes
         `docs/work-orders/NNN-declared.yaml`, and its pull request commits it.
       - A declaration already on `main` is not read again, so a later change declares itself in
         a file that its own pull request adds or changes.

     Nothing else in the file changes.
   - In `docs/work-orders/README.md`, line 46, "Any `*-declared.yaml` files stay with the orders
     that use them. None is present at this step." becomes:

     > An order that changes a case or a case's fixture writes its declaration here as
     > `NNN-declared.yaml`, and its pull request commits it: CI checks every vector change against
     > the declarations that a pull request adds or changes
     > ([reviewing vector changes](../../conformance/README.md#reviewing-vector-changes)).
     > Declarations stay with the orders that use them.

     The sentence after it, on Records, does not change.
   - In `docs/implementation/WORK_ORDERS.md`, under "Checking what comes back", item 2 (lines
     83-84) gains a last sentence:

     > On the pull request, CI's `docs` job also checks every vector change against the
     > declarations that the pull request adds or changes, and lists each flip for the reader
     > ([reviewing vector changes](../../conformance/README.md#reviewing-vector-changes)).
7. **Tests**, in `tools/tests/test_vector_delta.py`, with the fixture of `setUp`. The other tests
   do not change. None of the four tests below has a docstring: under `-v`, unittest would print
   it on the line that ends in `ok`. The tool and its tests run under the Mac's `/usr/bin/python3`
   (3.9) and CI's Python 3.12.
   - `test_declared_flip_still_fails` (lines 149-158) becomes
     `test_declared_flip_passes_and_is_listed`.
     - With its declaration it exits 0, prints
       `Flip: s1-small step 1 /steps/0/expect/reply/reason: "refused" -> "conflict"; declared by "intended".`
       and ends `Vector delta: 1 changes; 1 flips; 0 undeclared; 0 errors.`
     - Without `--declared` it exits 1, and its flip line ends `; undeclared.`
     - With its declaration and `conformance/small/source.json` deleted from the working tree, it
       exits 1 and ends `Vector delta: 2 changes; 1 flips; 1 undeclared; 1 errors.`
   - New `test_declared_may_be_given_more_than_once`: the same flip and a changed `title`, and two
     declaration files, each declaring one of them. Both given: exit 0, with a line
     `Declarations read from PATH: 1 shapes.` for each. Either alone: exit 1.
   - New `test_declared_changed_reads_what_the_change_adds_or_changes`, in three parts:
     - (a) The same flip in the working tree and an untracked
       `docs/work-orders/999-declared.yaml` declaring it: `HEAD --declared-changed` exits 0 and
       prints `Declarations read from docs/work-orders/999-declared.yaml: 1 shapes.`
     - (b) Both committed: `HEAD^ HEAD --declared-changed` exits 0, and `HEAD^ HEAD` exits 1.
     - (c) A further commit changes `title`, adds `docs/work-orders/old/998-declared.yaml`
       declaring it, and leaves `999-declared.yaml` as it was. Then
       `HEAD^ HEAD --declared-changed` exits 1, prints
       `No docs/work-orders/*-declared.yaml is added or changed from HEAD^ to HEAD.`, and ends
       `Vector delta: 1 changes; 0 flips; 1 undeclared; 0 errors.`
   - New `test_declaration_error_names_its_file`, in two parts:
     - A valid declaration file, then one with an unknown key (as on line 267), given as two
       `--declared`: exit 2, standard error holds the second file's path and "unknown
       declaration keys", and nothing holds a traceback.
     - An invalid untracked `docs/work-orders/999-declared.yaml` with `--declared-changed`: exit
       2, and standard error begins `vector_delta: docs/work-orders/999-declared.yaml: `.

## Scope

Change only these six files:

- `tools/vector_delta.py`
- `tools/tests/test_vector_delta.py`
- `.github/workflows/checks.yml`: the `docs` job's new step
- `conformance/README.md`: its section "Reviewing vector changes"
- `docs/work-orders/README.md`: line 46
- `docs/implementation/WORK_ORDERS.md`: item 2 of "Checking what comes back"

Write no declaration: this order changes no case or fixture. `tools/migrate_041a.py` imports
`structured` and `compare`, and their signatures stay. Do not change any case, `spec/`, `engine/`
or any order's file.

## Done when

From the repository root, each of these passes. `/usr/bin/python3` is the Mac's interpreter with
PyYAML; where `python3` has PyYAML, it serves as well.

```sh
/usr/bin/python3 -m unittest discover -s tools/tests     # OK
/usr/bin/python3 -m unittest discover -s tools/tests -p test_vector_delta.py -v 2>&1 | grep -c -E '^test_(declared_flip_passes_and_is_listed|declared_may_be_given_more_than_once|declared_changed_reads_what_the_change_adds_or_changes|declaration_error_names_its_file) .* ok$'   # prints 4
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared-changed; echo "exit $?"
python3 tools/check_docs.py                              # Errors: 0
python3 tools/check_private_paths.py                     # Errors: 0
git diff --check
```

The third prints `No docs/work-orders/*-declared.yaml is added or changed from HEAD to WORKTREE.`,
`Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`.

The gate on order 042's merge, with its declaration and without:

```sh
mkdir -p target/o174
/usr/bin/python3 tools/vector_delta.py 57813f06^ 57813f06 --declared-changed > target/o174/042.txt; echo "exit $?"
tail -n 2 target/o174/042.txt
grep -c '^Flip: s1-whole step [0-9]* /steps/[0-9]*/expect/stateUnchanged: .*; declared by "S1 stateUnchanged' target/o174/042.txt
/usr/bin/python3 tools/vector_delta.py 57813f06^ 57813f06 > target/o174/042-none.txt; echo "exit $?"
tail -n 1 target/o174/042-none.txt
```

The first prints `exit 0`. `tail` prints
`Declarations read from docs/work-orders/042-declared.yaml: 19 shapes.` and
`Vector delta: 1521 changes; 4 flips; 0 undeclared; 0 errors.`, and `grep` prints 4. The second
prints `exit 1` and `Vector delta: 1521 changes; 4 flips; 1521 undeclared; 0 errors.`

The step, read back from the workflow:

```sh
/usr/bin/python3 -c 'import yaml; s = yaml.safe_load(open(".github/workflows/checks.yml"))["jobs"]["docs"]["steps"][-1]; print(s["if"]); print(s["run"].strip())'
```

It prints `${{ !cancelled() && github.event_name == 'pull_request' }}`, then
`python3 tools/vector_delta.py HEAD^1 HEAD --declared-changed`.

Each of these prints 0:

```sh
grep -c 'STILL exits 1' tools/vector_delta.py
grep -c 'None is present at this step' docs/work-orders/README.md
tr -s ' \n' '  ' < conformance/README.md | grep -c 'even for declared flips'
```

Each of these prints 1. They read each file with its line breaks as spaces, so a rewrap does not
hide a phrase:

```sh
tr -s ' \n' '  ' < conformance/README.md | grep -c -F 'python3 tools/vector_delta.py HEAD^1 HEAD --declared-changed'
tr -s ' \n' '  ' < docs/work-orders/README.md | grep -c -F 'writes its declaration here as `NNN-declared.yaml`, and its pull request commits it'
tr -s ' \n' '  ' < docs/implementation/WORK_ORDERS.md | grep -c -F 'also checks every vector change against the declarations that the pull request adds or changes'
```

Then show that the new tests fail with `main`'s tool. Take it from `7346a280`, not `HEAD`: in a
second round, `HEAD` is the runner's checkpoint of your first. The test file imports
`vector_delta` from the directory above its own (line 14), so a copy beside `main`'s tool runs
against it:

```sh
mkdir -p target/o174/main/tests
git show 7346a280:tools/vector_delta.py > target/o174/main/vector_delta.py
cp tools/tests/test_vector_delta.py target/o174/main/tests/
/usr/bin/python3 -m unittest discover -s target/o174/main/tests -p test_vector_delta.py 2>&1 | grep -E '^(FAIL|ERROR): test_'
/usr/bin/python3 target/o174/main/vector_delta.py 57813f06^ 57813f06 --declared docs/work-orders/042-declared.yaml > target/o174/042-main.txt; echo "exit $?"
```

The `grep` lists the four tests of decision 7, each once as `FAIL` or `ERROR`, and no other test.
The last command prints `exit 1`: `main`'s tool fails on the four declared flips. Report each
line. Git ignores `target/`. `git status --short` lists only the six files in scope.

The admin then shows, on the pull request, the `docs` job's new step passing, ending with the
lines `No docs/work-orders/*-declared.yaml is added or changed from HEAD^1 to HEAD.` and
`Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.` A fresh read-only Opus reader then
reads the diff before it merges (ruling 79). Do not stage anything. Report each choice you make
as your own, and every check's last line.
