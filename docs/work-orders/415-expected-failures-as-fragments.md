# Work order 415: expected failures in one fragment per order, read as one list (DEC-190)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
is empty, since this order adds no case. It needs no network; if a crate is missing from the
cache, fetch nothing and report it. Run one cargo command at a time, and not `tools/gate.py`.
Never print a whole large file, case, diff or list: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential. Run no `conduit` outside a test,
no `codex` or `claude`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user,
machine or account name, or private repository name.

## What and why

**The owner's words.** DEC-190, in the decision log on `main`: asked how expected failures stop
conflicting, "One fragment per order (Recommended)": each order adds its own expected-failures
fragment, the checker merges them at gate time, and the one file retires, with order 400's
changelog fragments. It follows
[DEC-177](../decisions/log.md#dec-177-2026-11-24-changelog-fragments-and-no-stored-counts), whose
[order 400](400-changelog-fragments-and-computed-counts.md) left this file out.

**Today.** `conformance/expected-failures.yaml` is one JSON document, `{"cases": [...]}`, each
entry an `id` and a `reason`, sorted by `id`. Two readers open it: the conformance runner, in
`run_matching_with_threads` (`engine/crates/conformance/src/lib.rs`) before any case runs, with
`src/expected.rs`, whose diagnostics begin `conformance/expected-failures.yaml: `; and
`tools/promise_standing.py` (`EXPECTED_FAILURES`), which counts each promise's listed cases for
`engine/STANDING.md`. Their tests write or name it: `tests/expected_failures.rs` and the unit
tests of `src/expected.rs`, and `tools/tests/test_promise_standing.py`. `git grep` finds no other
reader, and the gate reads it only through these two. `tools/check_docs.py` fails on a literal
path in `tools/*.py` or the engine's `.rs` files, tests included, that does not exist, unless
`PATH_EXCEPTIONS` lists it, as it lists the retired `docs/proposals`. `conformance/README.md` and
`CONTRIBUTING.md` (rule 8) describe the one file. Two orders that list cases both edit it, so
their merges conflict there; the coordinator's private resolver unions the lists, and a script
removes the entries of cases a later order makes pass.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1). None is
at stake: no promise, case or index piece changes; every listed case keeps its entry, byte for
byte; `engine/STANDING.md` keeps every row. Each existing runner and tool test keeps what it
asserts.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice. Beside a unit `foo.service`, systemd reads every `.conf` file of its drop-in
directory `foo.service.d/`, in the order of their names
([systemd.unit](https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html)), so
that each package adds a file of its own instead of editing a shared one; order 400 did the same
for the changelog, with scriv's fragments. Here the coverage index left `manifest.yaml` for
`index.yaml` pieces, and the runner refuses a manifest that still has cases and an ID in two
pieces, as `check_decisions.py` refuses the retired `docs/proposals/`. This order takes those
forms. Not taken:

- pieces by the case's directory: orders 316 and 318 to 320 all list cases of one directory;
- one file per case: an order adds and removes its cases together;
- Git's `union` merge driver: it breaks the JSON's commas, and DEC-177 set merge drivers aside;
- a later fragment overriding an earlier one: a case listed twice is a merge fault to show;
- the one file kept beside the fragments: one list in two places;
- YAML fragments: today's JSON stays.

**When it runs.** On `wo/400-changelog-fragments` at
`5ee79ece69801d16f01d144e8cc386c190a79c82`, where order 400 waits to merge, or on `main` once 400
has merged: the Start check gives the same values on both. On that commit the decision log ends
at DEC-181 and the decision framework is in draft, so `check_decisions.py` notes the HOW-WE-WORK
line's DEC-190 without failing. Through Conduit: one Codex worker turn, the blind Codex verifier
and the Claude reader at once, since the order changes diagnostics and READMEs (DEC-189, on
`main`), one fix turn, the gate
([DEC-050](../decisions/log.md#dec-050-2026-11-22-both-reads-on-every-order),
[DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit),
[DEC-180](../decisions/log.md#dec-180-2026-11-24-parallel-tracks-on-the-accounts)). By ruling 89,
each decision says what must hold; how is yours. If one turn cannot finish it, it splits: 415a
takes decisions 1 to 7 and runs the Done-when lines not marked `415b`; 415b takes decision 8; both
run every Guard. The coordinator's choices, open to the owner's strike
([DEC-029](../decisions/log.md#dec-029-2026-11-20-who-decides-and-who-does)): names as order files
with `.yaml`; one fragment for the move, named for this order; the directory's `README.md`; dot
names skipped; no empty fragment; a case listed twice refused, not overridden; the retired file
refused; both readers checking the paths; the docs guard's entry; HOW-WE-WORK's label.

**Shared files.** Orders that add or remove expected failures and merge after this one (308 and
316 to 320 among them) rebase onto it. An entry they add moves, word for word, to a fragment
named for their order file, which holds only their own entries. An entry they remove leaves the
fragment that holds it, and a fragment so emptied is deleted. Where such an order merges first,
this order rebases instead, and its fragment takes the one file as it then stands, byte for byte.
For the other files, the later of two orders rebases.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first, in `engine/crates/conformance`: `paths` and `run_matching_with_threads` in
`src/lib.rs`, `src/expected.rs`, the refusals of `entries` in `src/pieces.rs`; then `main` in
`tools/promise_standing.py`, and `setUp` in its tests.

Wrap each insertion at its file's width; rewrap a paragraph you shorten. In quoted text, each
"[text] (link to `target`)" is a real link to that target: write it so, and find it so in the text
a decision replaces.

1. **The fragments, `conformance/expected-failures/`.** A fragment's name and form are those
   decision 7's two texts state. A fragment is a regular file directly in the directory.
   `README.md` is no fragment, and a name beginning with a dot is not read, as under
   `case-schema/`. Every other path in the directory is refused: a file whose name does not match,
   anything that is not a regular file, and a subdirectory, refused at its own path
   (`conformance/expected-failures/sub` for a `sub/` holding `001.yaml`) with nothing under it
   read. So is `conformance/expected-failures.yaml`, while it exists.
   `conformance/expected-failures/README.md`:

   ```text
   # Expected failures

   Each order that lists a case the build does not pass yet writes its entries here, in a fragment of its own named for its order file, in the form [conformance/README.md] (link to `../README.md#expected-failures`) states; the runner reads every fragment as one list (DEC-190). This file is no fragment: it keeps the directory when no case is listed.
   ```
2. **One list.** Both readers read the fragments in byte order of their names, and each
   fragment's entries in order, as one list. A case ID listed twice, in one fragment or in two, is
   refused, and the refusal names where it was listed first. A missing directory is refused; a
   directory that holds only `README.md` gives an empty list. Both readers apply decision 1's
   refusals of paths and of the retired file. The runner alone checks each entry's form, that a
   fragment has an entry, the order of a fragment's entries and that each ID names a case, as it
   checks the one file today.
3. **The runner**, `engine/crates/conformance`.
   - It reads the fragments where it read the one file, before any case runs, even with an ID
     prefix, and checks them as decisions 1 and 2 say. `validate` in `src/expected.rs` checks one
     fragment, given its path, and still accepts an empty list; the reader of the fragments
     refuses a fragment whose list is empty once `validate` has accepted it. Its walk for cases
     and `index.yaml` pieces does not enter `expected-failures/`.
   - Each diagnostic begins with the path it concerns and ends
     `. See conformance/README.md, "Expected failures".`, as today:
     `conformance/expected-failures/<name>: ` for a fragment or a refused path in the directory,
     `conformance/expected-failures/: ` for a missing directory, and
     `conformance/expected-failures.yaml: ` for the retired file. A fragment's diagnostics keep
     today's words, the entry named by its ID or by its position in its fragment. The new ones,
     after the path:
     - a refused path: `not a fragment; a fragment is a regular file directly in
       conformance/expected-failures/, named as its order's file with .yaml for .md (DEC-190)`;
     - the retired file: `retired; move its entries to a fragment under
       conformance/expected-failures/ named for the order (DEC-190)`;
     - a missing directory: `not found; restore the directory with its README.md`;
     - a fragment with no entry: `the root's cases is empty; delete the fragment`;
     - an ID an earlier fragment lists: `<id>: is listed again as entry <n>, after entry <m> of
       conformance/expected-failures/<name>; keep only one entry for this ID`.
   - A listed case that passes fails the run with the line `<id> [<promises>]: passes, but
     conformance/expected-failures/<name> lists it as an expected failure (<reason>); remove its
     entry`, `<name>` its fragment's. The executable's other lines, `RunReport` and
     `ExpectedFailure` stay as they are.
4. **Its tests**, in `engine/crates/conformance`.
   - In `.rs` files, build each test fragment's path from the directory and the name (`format!`
     or `join`), never as a literal `conformance/expected-failures/<name>`: `check_docs.py`
     fails on a literal path that does not exist. `PATH_EXCEPTIONS` gains only the retired file.
   - The unit tests of `src/expected.rs` keep what they assert, each message with a fragment's
     path for `conformance/expected-failures.yaml`; `an_empty_list_gives_an_empty_map` still
     passes `{"cases": []}`.
   - In `tests/expected_failures.rs`, the helper that writes the list removes the copy's
     fragments, leaving `README.md`, and writes the list as a fragment `001-a.yaml` when it has
     an entry and as no fragment when it is empty. The three tests keep what they assert;
     `a_listed_case_that_passes_fails_the_run` also asserts that the error names
     `001-a.yaml`'s path. New, each on the copied tree:
     - `fragments_are_read_as_one_list`: the broken case listed in `002-b.yaml`, and in
       `001-a.yaml` another case's ID, which the prefix does not run. The broken case is reported
       apart, as in `a_listed_failing_case_is_reported_apart`, and the executable's last line is
       `0 cases passed; 1 expected failures`.
     - `a_case_listed_twice_is_refused`: the broken case's ID in `001-a.yaml` and in `002-b.yaml`
       gives exactly decision 3's message for `002-b.yaml`, naming entry 1 of `001-a.yaml`; the ID
       twice in `001-a.yaml` gives today's message, under that fragment's path.
     - `the_directory_holds_only_fragments`: each of these alone fails the run, with an error that
       begins with its path and holds decision 3's words: the retired file beside the fragments;
       `Order.yaml`, `001.json`, `index.yaml` and `case.yaml` in the directory; a subdirectory
       `sub/` holding `001.yaml`, at the path of `sub`; a fragment `{"cases": []}`; the directory
       removed. A `README.md` of prose and a `.hidden.yaml` that is no JSON change nothing: the
       run gives what it gives without them.
5. **`tools/promise_standing.py`** and its tests.
   - `EXPECTED_FAILURES` names the directory. The tool reads the fragments as decision 2 says,
     with today's checks of each fragment's document and entries, and its search for
     `index.yaml` pieces does not enter `expected-failures/`. It gives each refusal as
     `PATH: MESSAGE`, as its other errors are: a refused path and the retired file with decision
     3's words; a missing directory `not found; the conformance runner requires it (DEC-190)`; an
     ID listed twice `case <n> (<id>) is listed again, after case <m> of
     conformance/expected-failures/<name>; keep only one entry for this ID`. With any of them it
     writes and compares nothing, as with its other errors.
   - Its `Read` line gives `N entries in N fragments under conformance/expected-failures/` in
     place of `N entries of conformance/expected-failures.yaml`. In its intro, "how many of those
     cases `conformance/expected-failures.yaml` lists." becomes "how many of those cases a fragment
     under `conformance/expected-failures/` lists." Regenerate `engine/STANDING.md` with
     `--write`; no row changes.
   - In `tools/tests/test_promise_standing.py`, `setUp` writes the directory's `README.md` and no
     fragment. Each test that writes the list writes a fragment `001-a.yaml` instead, or deletes
     it where it wrote an empty list, and keeps what it asserts. The unreadable-input test's two
     cases become a fragment holding `[]` and a missing directory, with the messages above. New:
     - `test_expected_fragments_are_one_list`: case `a` listed in `002-b.yaml` and case `b` in
       `001-a.yaml` give the table that one fragment listing both gives.
     - `test_expected_fragment_listed_twice_is_refused`: `b` in `001-a.yaml` and in `002-b.yaml`
       gives one error, `conformance/expected-failures/002-b.yaml: case 1 (b) is listed again,
       after case 1 of conformance/expected-failures/001-a.yaml; keep only one entry for this ID`,
       and writes nothing.
     - `test_expected_fragment_paths_are_checked`: the retired file, `Order.yaml`, `001.json`,
       `index.yaml` and a `sub/` holding `001.yaml` give one error each, at its path
       (`conformance/expected-failures/sub` for the subdirectory); a `README.md` of prose and a
       `.hidden.yaml` give none.
6. **The move and the docs guard.**
   - `conformance/expected-failures.yaml` moves, byte for byte, to
     `conformance/expected-failures/415-expected-failures-as-fragments.yaml`, the fragment named
     for the move. No entry changes, and none is split out by the order that listed it.
   - `tools/check_docs.py`: `PATH_EXCEPTIONS` gains
     `"conformance/expected-failures.yaml": "retired path"`, and the comment above it says that the
     runner and `promise_standing.py` refuse that file's return. No source names the retired path
     but to refuse it.
7. **`conformance/README.md`.**
   - "Running": "A case listed in [expected-failures.yaml] (link to `#expected-failures`)" becomes
     "A case listed in [the expected failures] (link to `#expected-failures`)".
   - "Expected failures", first paragraph: "[expected-failures.yaml] (link to
     `expected-failures.yaml`) lets a new promise" becomes "The expected failures, the fragments
     under [expected-failures/] (link to `expected-failures/README.md`), let a new promise", and
     "The change that makes the build pass a listed case removes its entry." becomes "The change
     that makes the build pass a listed case removes its entry from the fragment that holds it, and
     deletes the fragment if that empties it."
   - The paragraphs from "The file always exists." to "with no duplicate IDs.", the empty form
     among them, become:

     ```text
     Each order that lists a case writes the entry in a fragment of its own, named as its file under `docs/work-orders/` with `.yaml` for `.md`; a change with no order names it for its branch, with `-` for `/` (DEC-190). A name is a lowercase letter or digit, then lowercase letters, digits, `.` and `-`, then `.yaml`, and is neither `index.yaml` nor `case.yaml`. An order edits another order's fragment only to change or remove an entry whose case it changes or makes pass, and it deletes a fragment it empties. The directory always exists and holds [README.md] (link to `expected-failures/README.md`), which is no fragment; nothing whose name begins with a dot is read. Any other path in it fails the run, and so does the retired `expected-failures.yaml`.

     A fragment is JSON written with `serde_json`'s pretty printer and a final newline, as the coverage-index pieces are written. Its root object has only `cases`, a list of at least one entry. Each entry has exactly two members, in this order: `id`, the ID of a case in the tree, and `reason`, what the build lacks and which order is to make the case pass. Both are nonempty strings. A fragment's entries are sorted by `id` in byte order. The runner reads the fragments in byte order of their names as one list, in which no ID is listed twice.
     ```
   - The next paragraph, from "Before any case runs," to "remove the entry or restore the case.",
     becomes:

     ```text
     Before any case runs, even with a requested ID prefix, the runner reads and checks every fragment against every case ID. A missing directory, a path in it that is no fragment, the retired file, an unreadable or empty fragment, the wrong root or list form, a malformed entry, an ID listed twice, IDs out of byte order in a fragment or an ID naming no case fails the run. Each diagnostic begins with the path it concerns, `conformance/expected-failures/<name>: ` for a fragment, names the entry by its ID, or by its position in its fragment (`entry 2`) where it has no ID, says what is wrong, and then says what would fix it. An ID listed again names the fragment and entry that listed it first. For an ID naming no case, remove the entry or restore the case.
     ```
   - The failure line in the `text` block becomes decision 3's, with `<name>`.
8. **Where the rule is written.**
   - `AGENTS.md`, "If you were given a work order", after the bullet beginning "An order that
     changes `spec/` writes its changelog lines":

     ```text
     - An order that lists a case the build does not pass yet writes the entry in its own fragment under `conformance/expected-failures/`, named for its order file, whatever its scope lists; an order that makes a listed case pass removes the entry from the fragment that holds it ([HOW-WE-WORK section 8] (link to `HOW-WE-WORK.md#8-records-where-each-fact-lives`)).
     ```
   - `HOW-WE-WORK.md`, section 8, after item 5:

     ```text
     6. **Expected-failure fragments.** A change that lists a case the build does not pass yet writes the entry in its own fragment under `conformance/expected-failures/`, in the form [conformance/README.md] (link to `conformance/README.md#expected-failures`) states, and the change that makes the case pass removes it; the runner and `tools/promise_standing.py` read every fragment as one list and refuse a case listed twice. (DEC-190) `checked: tools/gate.py`
     ```
   - `CONTRIBUTING.md`, Writing promises, rule 8: "in [conformance/expected-failures.yaml] (link
     to `conformance/README.md#expected-failures`)." becomes "in its own fragment of [the expected
     failures] (link to `conformance/README.md#expected-failures`), under
     `conformance/expected-failures/`."

## Start check

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
h() { sed -n '/^## 8\. /,/^## 9\. /p' HOW-WE-WORK.md | grep -cE '^[0-9]+\. '; }
ef() { PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 -I -c 'import json, sys; d = json.load(open(sys.argv[1])); c = d["cases"]; i = [e["id"] for e in c]; print(list(d), i == sorted(set(i), key=str.encode), all(list(e) == ["id", "reason"] for e in c))' "$1"; }
git status --short | wc -l | tr -d ' '  # 0
test -f spec/changes/README.md; echo "400 $?"  # 400 0
test -e conformance/expected-failures; echo "fragments $?"  # fragments 1
ef conformance/expected-failures.yaml  # ['cases'] True True
grep -rlF 'expected-failures.yaml' tools engine/crates --include='*.py' --include='*.rs' | sort | paste -sd' ' -  # engine/crates/conformance/src/expected.rs engine/crates/conformance/src/lib.rs engine/crates/conformance/tests/expected_failures.rs tools/promise_standing.py
k engine/crates/conformance/src/lib.rs 'directory.join("expected-failures.yaml")' 'passes, but conformance/expected-failures.yaml lists it' 'fn paths(directory: &Path, found: &mut Vec<PathBuf>)'  # 1 1 1
k engine/crates/conformance/src/expected.rs 'fn an_empty_list_gives_an_empty_map()' 'conformance/expected-failures.yaml: {'  # 1 2
k engine/crates/conformance/tests/expected_failures.rs '.join("expected-failures.yaml"),' 'repository.list(json!([]));'  # 1 1
k tools/promise_standing.py 'EXPECTED_FAILURES = "conformance/expected-failures.yaml"' '{len(expected_data)} entries of '  # 1 1
k tools/check_docs.py '"docs/proposals/": "retired path",' 'expected-failures'  # 1 0
k conformance/README.md '[expected-failures.yaml](expected-failures.yaml) lets' 'listed in [expected-failures.yaml](#expected-failures) is reported apart' 'The file always exists.' 'Each diagnostic begins `conformance/expected-failures.yaml: `' 'passes, but conformance/expected-failures.yaml lists it'  # 1 1 1 1 1
k CONTRIBUTING.md 'in [conformance/expected-failures.yaml](conformance/README.md#expected-failures).'  # 1
k AGENTS.md 'writes its changelog lines in its own fragment' 'conformance/expected-failures/'  # 1 0
h  # 5
k HOW-WE-WORK.md 'conformance/expected-failures/'  # 0
k engine/STANDING.md 'how many of those cases `conformance/expected-failures.yaml` lists.'  # 1
grep -c '#\[test\]' engine/crates/conformance/tests/expected_failures.rs engine/crates/conformance/src/expected.rs | paste -sd' ' -  # engine/crates/conformance/tests/expected_failures.rs:3 engine/crates/conformance/src/expected.rs:5
```

Where a line prints otherwise, stop, change nothing and report it. The values are those of
`5ee79ece` and of `main` with order 400 alike; the one file holds more entries on `main`, and no
line counts them.

## Scope

Change, as the decisions say: `conformance/expected-failures.yaml`, which moves;
`conformance/expected-failures/`, its `README.md` and the move's fragment;
`conformance/README.md`'s "Running" and "Expected failures"; in `engine/crates/conformance`,
`src/expected.rs`, `src/lib.rs` and `tests/expected_failures.rs`; `tools/promise_standing.py`,
`tools/tests/test_promise_standing.py` and `PATH_EXCEPTIONS` in `tools/check_docs.py`, with its
comment; `engine/STANDING.md`, only by `promise_standing.py --write`; one bullet of `AGENTS.md`,
one line of `HOW-WE-WORK.md` and a phrase of `CONTRIBUTING.md`. Nothing else: no promise, case,
index piece or fixture, `spec/` (so no changelog fragment), another engine file, `packages/`,
`experiments/`, other `tools/` files, `.github/` or `docs/decisions/` (DEC-190 stays not
done). Report anything else that needs changing.

## Done when

Each line prints otherwise before and passes after, with `j`, `k` and `h` as above:

```sh
export PYTHONDONTWRITEBYTECODE=1
F=conformance/expected-failures/415-expected-failures-as-fragments.yaml
test -e conformance/expected-failures.yaml; echo "one file $?"  # one file 1; one file 0 before
echo "$(ls -A conformance/expected-failures 2>/dev/null | paste -sd' ' -)"  # 415-expected-failures-as-fragments.yaml README.md; an empty line before
git show HEAD:conformance/expected-failures.yaml | cmp -s - "$F"; echo "moved $?"  # moved 0; moved 2 before
k conformance/expected-failures/README.md 'This file is no fragment: it keeps the directory when no case is listed.'  # 1; 0 before
k engine/crates/conformance/src/lib.rs 'passes, but conformance/expected-failures.yaml lists it'  # 0; 1 before
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -oE '^test result: ok\. [0-9]+ passed')  # test result: ok. 6 passed; ok. 3 passed before
/usr/bin/python3 -m unittest discover -s tools/tests -k expected_fragment 2>&1 | grep -E '^(Ran |OK)' | sed -E 's/ in [0-9.]+s$//' | paste -sd' ' -  # Ran 3 tests OK; Ran 0 tests OK before
python3 tools/promise_standing.py --check | head -1 | grep -c ' fragments under conformance/expected-failures/, '  # 1; 0 before
k engine/STANDING.md 'how many of those cases a fragment under `conformance/expected-failures/` lists.'  # 1; 0 before
k tools/check_docs.py '"conformance/expected-failures.yaml": "retired path",'  # 1; 0 before
k conformance/README.md '[expected-failures.yaml](expected-failures.yaml) lets' 'listed in [expected-failures.yaml](#expected-failures) is reported apart' 'The file always exists.' 'passes, but conformance/expected-failures.yaml lists it'  # 0 0 0 0; 1 1 1 1 before
k conformance/README.md 'listed in [the expected failures](#expected-failures) is reported apart' 'The expected failures, the fragments under [expected-failures/](expected-failures/README.md), let a new promise' 'and deletes the fragment if that empties it.' 'is neither `index.yaml` nor `case.yaml`' 'An ID listed again names the fragment and entry that listed it first.' 'passes, but conformance/expected-failures/<name> lists it'  # 1 1 1 1 1 1; 0 0 0 0 0 0 before
k AGENTS.md 'writes the entry in its own fragment under `conformance/expected-failures/`'  # 415b: 1; 0 before
h  # 415b: 6; 5 before
k CONTRIBUTING.md 'in its own fragment of [the expected failures](conformance/README.md#expected-failures)' 'in [conformance/expected-failures.yaml](conformance/README.md#expected-failures).'  # 415b: 1 0; 0 1 before
```

Guards, which pass before and must still pass. `check_private_paths.py` reads each path `git
ls-files` lists, so lines 2 and 3 index the move and the last line undoes it:

```sh
export PYTHONDONTWRITEBYTECODE=1
git add -N conformance/expected-failures 2>/dev/null
test -e conformance/expected-failures.yaml || git rm -q --cached --ignore-unmatch conformance/expected-failures.yaml
for t in check_docs check_schemas check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, three times
python3 tools/set_names.py --check | grep -oE 'Errors: [0-9]+$'  # Errors: 0
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Not-done ratchet|Errors)'  # each head equals its base; Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
git diff HEAD -- engine/STANDING.md | grep -c '^[-+]|'  # 0: no row changes
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # 0 changes; 0 undeclared; 0 errors; then exit 0
grep -c 'fn an_empty_list_gives_an_empty_map()' engine/crates/conformance/src/expected.rs  # 1
(cd engine && cargo fmt --check -p conduit-conformance; echo "fmt $?")  # fmt 0
(cd engine && cargo clippy --offline -p conduit-conformance --all-targets -- -D warnings > /dev/null 2>&1; echo "clippy $?")  # clippy 0
(cd engine && cargo test --offline --release -p conduit-conformance --lib expected:: 2>&1 | grep -oE '^test result: ok\. [0-9]+ passed')  # test result: ok. 5 passed, or more after
echo "$(git show HEAD:conformance/expected-failures.yaml | grep -c '"id": ') $(cd engine && cargo run --offline --release -q -p conduit-conformance 2>/dev/null | tail -1 | grep -oE '[0-9]+ expected failures$' | cut -d' ' -f1)"  # two equal numbers: every listed case is still reported apart
/usr/bin/python3 -m unittest discover -s tools/tests 2>&1 | tail -1  # OK
git diff --check  # prints nothing
git status --short --no-renames -uall | sed 's/^...//' | grep -vE '^(conformance/(README\.md|expected-failures\.yaml|expected-failures/(README\.md|415-expected-failures-as-fragments\.yaml))|engine/(STANDING\.md|crates/conformance/(src/(expected|lib)\.rs|tests/expected_failures\.rs))|tools/(promise_standing|check_docs|tests/test_promise_standing)\.py|(AGENTS|HOW-WE-WORK|CONTRIBUTING)\.md)$' | wc -l | tr -d ' '  # 0
git reset -q -- conformance/expected-failures conformance/expected-failures.yaml  # stage nothing
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML. Report each changed sentence, each new
test's name, each existing test you changed and why, and every check's last line.

## What follows

The admin marks DEC-190 done with this order's pull request. The coordinator's resolver and its
script then leave expected failures alone: each order writes its own fragment, and the order that
makes a case pass removes its entry. Order 401's serial queue need not stop on this file's
conflicts. The order that removes the move's fragment's last entry deletes it.

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, stacked on order 400 (changelog
fragments), built and queued to merge. Drafted and checked by the coordinator's Workflow; built
and read on the team seats (DEC-179, DEC-180).

**Worker-4's turn**, on the Business seat, carried out the whole order in 20 minutes: every reader
of the one file now reads the union of fragments, and the 13 entries moved byte for byte, with
STANDING unchanged.

**Two blind reads.** The Codex verifier, `root-verifier-4.turn.v1`, passed it with no finding. The
Claude reader, `root-reader-a.turn.v1`, on the Team seat, passed it with one should (an order that
makes a listed case pass removes the entry from whichever fragment holds it, and may name that
fragment in its scope) and two nits (the directory's README states the name rule with an example;
both walks skip only `conformance/expected-failures/` itself). One fix turn by worker-4 answered
all three. The gate's run on the squash is the coordinator's, at the merge.
