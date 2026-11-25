# Work order 400: changelog fragments, and counts the checkers give instead of the READMEs (DEC-177)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
is empty, since this order adds no case. It needs no network and changes no Rust source; if a
crate is missing from the cache, fetch nothing and report it. Run one cargo command at a time,
and not `tools/gate.py`. Never print a whole large file, case, diff or list: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Run no
`conduit` outside a test, no `codex` or `claude`, and sign in nowhere. Prefix each `python3`
with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user,
machine or account name, or private repository name.

## What and why

**The owner's words.** [DEC-177](../decisions/log.md#dec-177-2026-11-24-changelog-fragments-and-no-stored-counts):
asked how merges stop conflicting on count lines and the changelog, "Changelog fragments, no
stored counts": each order adds its own fragment instead of editing one file, and the checkers
compute the counts READMEs store. It replaces this order's first draft, git merge drivers.

**Today.** Each spec order adds lines under `## [Unreleased]` in `spec/CHANGES.md` and edits the
counts it moves in the READMEs of `spec/`, `spec/core/` and `spec/schemas/`, so merges conflict
there and the coordinator's private resolver merges them by rule. Step 8 of `check_schemas.py`
checks the promise totals alone; the rest drift (the core is said to export 158 documents, 80
of them Schema contracts: 156 and 79 today). `spec/definitions.md` and `engine/CONFORMANCE.md`,
which the resolver also named, hold none.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1). None
is at stake: no promise, case, engine source or package changes, and the changelog loses no
line. The READMEs lose their counts: the checkers print the totals, and the other figures,
which nothing compared and some of which had drifted, are dropped.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: [scriv](https://scriv.readthedocs.io/) keeps a changelog as a directory of
Markdown fragments, one per change, skipping `README.*`; `scriv collect` moves them into the
changelog at release and deletes them. [towncrier](https://towncrier.readthedocs.io/)'s `check`
fails a branch that adds none, and `build --draft` prints the assembly. `python3
tools/check_docs.py --work-orders` already prints the order index instead of storing it. Not
taken: towncrier's `{issue}.{type}` names, which split an order's lines by Kind; scriv's
timestamp names; either tool as a dependency; fragments for the expected failures, and the
counts of `docs/implementation/SAFETY_CATALOGUE.md`, a record with its own recount, which
DEC-177 (READMEs and rows) does not name; `engine/STANDING.md`, which is generated.

**When it runs.** On `main`, now: DEC-180 gives the kit's replacements to pairs 3 and 4, and this
order takes one's place. Through Conduit: one Codex worker turn, the blind Codex verifier and the
Claude reader at once, one fix turn, the gate
([DEC-050](../decisions/log.md#dec-050-2026-11-22-both-reads-on-every-order),
[DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit),
[DEC-180](../decisions/log.md#dec-180-2026-11-24-parallel-tracks-on-the-accounts)). By ruling 89,
each decision says what must hold; how is yours. If one turn cannot finish it, it splits: 400a
takes decisions 1 to 4 and 8 but rule 5 and the AGENTS.md bullet's last clause, and writes no
fragment (coupling exempts what it changes under `spec/`); 400b takes the rest. 400a runs the
Done-when lines not marked `400b`, with its values; both run every Guard. The coordinator's
choices, open to the owner's strike
([DEC-029](../decisions/log.md#dec-029-2026-11-20-who-decides-and-who-does)): scriv's convention
and names; the 68 lines kept; the refusal in draft; `--draft`; the count's pattern; only the
totals printed; the two State clauses; the edits under `docs/`, `CONTRIBUTING.md` and
`ROADMAP.md`; the expected failures left out.

**Shared files.** A spec order merged after this one rebases onto it: its `Unreleased` lines
move, word for word, to its fragment, and each count it changed takes this order's text. For
the other files, the later of two orders rebases.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Wrap each insertion at its file's width; rewrap a paragraph you shorten. In quoted text, write
each "[text] (link to `target`)" as a real link to that target.

1. **The fragments, `spec/changes/`**, as scriv keeps them.
   - A fragment is a file directly in it other than `README.md`, named as the order's file in
     `docs/work-orders/`, or for a change with no order, as its branch with `-` for `/`. A name
     is a lowercase letter or digit, then any lowercase letters, digits, `.` and `-`, then `.md`.
   - It holds only list items of the form `spec/CHANGES.md` states, at least one; a wrapped item
     indents its continuation by two spaces.
   - `spec/changes/README.md`:

     ```text
     # Changelog fragments

     Each change to `spec/` writes its lines here, in a file of its own, in the form [spec/CHANGES.md] (link to `../CHANGES.md`) states; a release moves them into it (DEC-177). This file is no fragment: it keeps the directory when a release empties it.
     ```
2. **`spec/CHANGES.md`'s preamble.** The 68 items under `## [Unreleased]` stay as they are.
   - "Each change to `spec/` adds one line under `Unreleased`, of the form" becomes "Each change
     to `spec/` adds its lines to a fragment of its own, `spec/changes/` and its order file's
     name, and none here (DEC-177). A line has the form".
   - "A release moves the `Unreleased` lines under its own heading." becomes "A release moves
     the lines under `Unreleased`, then each fragment's in the byte order of their names, under
     its own heading, and removes the fragments, as `scriv collect` does; `python3
     tools/check_decisions.py --draft WORKTREE` prints them so assembled."
3. **`tools/check_decisions.py`.**
   - A snapshot's Unreleased items are those under `## [Unreleased]` of `spec/CHANGES.md`, then
     each fragment's, in byte order of their names. "CHANGES lines", "coupling" and the not-done
     ratchet read them where they read CHANGES.md's, and a finding on a fragment's item names the
     fragment and its line. Items under a release's heading are read as today.
   - A new rule, `fragments`, fails even in draft on: an item under `## [Unreleased]` of
     CHANGES.md that BEFORE lacks, compared as `added_items` does ("add it to a fragment under
     spec/changes/ (DEC-177)"); a path under `spec/changes/` other than `README.md` that is not a
     regular file directly there or whose name does not match; a fragment with no item, or with
     any other line.
   - Coupling exempts `spec/changes/` as it exempts CHANGES.md, and its finding asks for "a new
     item in a fragment under spec/changes/ (DEC-177)".
   - `--draft REV` (a commit or `WORKTREE`) runs no rule, prints `## [Unreleased]`, a blank line
     and REV's Unreleased items as written, and exits 0, as `towncrier build --draft` does; with
     BEFORE or AFTER, or a REV that does not resolve, it exits 2. `--help` says so.
   - A path it writes literally exists, or holds `NNN-name`, which `check_docs.py` skips.
4. **Its tests**, in `tools/tests/test_decisions.py`. A test that adds an item under Unreleased
   writes it to `spec/changes/001-order.md` instead and keeps its name and what it asserts, at
   the fragment's path and line; one on a release's heading stays on CHANGES.md. New:
   - `test_fragment_item_satisfies_coupling`: in force, `SAFE` in a fragment, beside a changed
     `spec/sample.yaml` or alone, gives no coupling finding.
   - `test_fragment_item_form_is_checked`: "- Older wording." in a fragment gives
     `error: spec/changes/001-order.md:1: CHANGES lines:`.
   - `test_fragment_unreleased_line_in_changes_refused`: `SAFE` under Unreleased in CHANGES.md
     gives one `fragments` error, in draft and in force.
   - `test_fragment_release_moves_lines`: a fragment removed, its item under `## [0.1.0]`: none.
   - `test_fragment_files_are_named_lists`: `Order.md`, `sub/001.md`, a fragment with a heading
     and an empty one: one `fragments` error each; a `README.md` of prose: none.
   - `test_fragment_items_feed_the_ratchet`: the not-done ratchet counts a fragment's item.
   - `test_fragment_draft_prints_unreleased`: with `002-b.md` written before `001-a.md`,
     `--draft WORKTREE` prints the heading, a blank line and the items of CHANGES.md, `001-a.md`
     and `002-b.md`, in that order, and exits 0; with BEFORE and AFTER too, 2.
5. **No count in the spec READMEs.**
   - `spec/README.md`, `## Contents`:
     - Its first sentence, from "Counts below are current;" to "as of 2026-11-10.", becomes:

       ```text
       The table states no count: the checkers count what they check, and [STANDING.md] (link to `../engine/STANDING.md`) gives each promise's cases.
       ```
     - Each State cell keeps its first clause, `Draft` or `Current`, and drops each clause that
       counts or gives coverage (`vectors test slices of N promises`, `no vectors yet`), but
       `core/`'s becomes "Draft; its one implementation is also an export", and
       `schemas/examples/valid`'s "Draft; a file whose name begins with a dot is not read".
     - The sentence beginning "Every area has been written as promises, 522 in all:" becomes
       "Every area has been written as promises; step 8 of `python3 tools/check_schemas.py` gives
       how many, in all and by layer."
   - `spec/core/README.md`, "What this is": the file table's cells drop their counts ("The 40
     Operation contracts:" becomes "The Operation contracts:", and so on to "The 20 templates:");
     "the 52 Schema contracts under" becomes "the Schema contracts under"; and the sentence
     beginning "In all the package exports 158 documents" becomes "The package has one
     implementation, which is also an export (PKG-019); step 5 of `python3
     tools/check_schemas.py` gives how many documents it exports."
   - `spec/schemas/README.md`, "Examples": "77 files, each holding one document" and "38 files,
     each holding one set" begin "Files, each holding"; "Four files have `-manifest-` in their
     names and hold a manifest" becomes "A file with `-manifest-` in its name holds a manifest";
     "Three others hold, beside their documents," becomes "Others may hold, beside their
     documents,".
6. **`tools/check_schemas.py`'s step 8** compares no stated total. Once step 7 found no error:
   - It counts the promises as today, and checks that `spec/README.md`'s `## Contents` has one
     row for each promise file, whatever its State, and none for a `.yaml` path under
     `promises/`, `conventions/` or `interface/` that is not a promise file.
   - It fails each match of either pattern below in `spec/README.md`, `spec/core/README.md` and
     `spec/schemas/README.md`, each read whole where it exists, as `PATH:LINE: states the count
     "MATCH"; the checkers give counts instead (DEC-177)`. A count in words is not found.
   - It prints, with the numbers for N:

     ```text
     8. Counted N promises in N files: N of the layer `language`, N of the layer `convention` and N of the layer `interface`; spec/README.md's Contents has a row for each, and the three READMEs state no count; N errors.
     ```
   - Its NOT RUN line, its row errors (which ask for no count) and docstring item 8 say so.

   The patterns, in Python's `re`:

   ```text
   (?<![A-Za-z0-9.-])[1-9][0-9]* (?:[A-Z][a-z]+ )?(?:promises?|contracts?|templates|documents|files|items|reference schemas)(?![a-z])
   (?<![A-Za-z0-9.-])[1-9][0-9]* (?:in all|of the layer)\b
   ```
7. **Its tests**, in `tools/tests/test_promise_totals.py`. The fixture and every row a test adds
   state no count ("Draft; 3 promises" becomes "Draft"), so each assertion stays exact.
   `test_totals_that_agree_pass`, `test_each_number_wrong_by_one_fails` and
   `test_empty_final_state_cell_has_no_promise_count` give way to the three below;
   `test_rows_and_sentence_must_be_there` becomes `test_rows_must_be_there`, without its
   subcases for a row with no count and for no sentence, whose errors are gone; the others keep
   what they assert, in decision 6's messages. New:
   - `test_stored_count_absent_prints_totals`: no error, and step 8's line for 4, 3, 1, 2, 1.
   - `test_stored_count_fails`: one error each, at its file and line, for "Draft; 2 promises" in
     a row, "4 in all:", "1 of the layer", "The 40 Operation contracts:" in a
     `spec/core/README.md` and "| 77 files, each" in a `spec/schemas/README.md`.
   - `test_stored_count_ignores_modes_and_names`: "mode-0600 files", "0700 directory with 0600
     files", "each S1 promise's", "DEC-177 promises" and "the 0.2.1 contracts" give none.
8. **Where the rule is written.**
   - `AGENTS.md`, "If you were given a work order", after the bullet beginning "Change only the
     files in the order's scope.":

     ```text
     - An order that changes `spec/` writes its changelog lines in its own fragment under `spec/changes/`, whatever its scope lists, and adds no count to a README ([HOW-WE-WORK section 8] (link to `HOW-WE-WORK.md#8-records-where-each-fact-lives`)).
     ```
   - `HOW-WE-WORK.md`, section 8, after item 3:

     ```text
     4. **Changelog fragments.** A change to `spec/` writes its lines in its own fragment under `spec/changes/`, in the form [spec/CHANGES.md] (link to `spec/CHANGES.md`) states; a release moves them into that file. (DEC-177) `checked: tools/check_decisions.py`
     5. **No stored counts.** The READMEs of `spec/`, `spec/core/` and `spec/schemas/` state no count; the checkers print the totals. (DEC-177) `checked: tools/check_schemas.py`
     ```
   - `docs/decisions/README.md`: "has one line per change to it;" becomes "and its fragments
     under `spec/changes/` have one line per change to it;".
   - `docs/clients.md`: after "naming the promise IDs and the decision.", "A change not yet
     released is a fragment under `spec/changes/`."; "follows with its `spec/CHANGES.md` line"
     becomes "follows with its fragment under `spec/changes/`".
   - "with `spec/CHANGES.md` entries" becomes "with fragments under `spec/changes/`" in
     `CONTRIBUTING.md`, `ROADMAP.md`, `docs/AUDIT.md` (twice), `docs/guides/cold-readers.md`
     and `docs/guides/full-review.md`.
9. **This order's fragment**, `spec/changes/400-changelog-fragments-and-computed-counts.md`,
   one item on one line, as CHANGES.md's are:

   ```text
   - Changed: `spec/README.md#contents`, `spec/core/README.md` and `spec/schemas/README.md` state no count of promises, documents, contracts, templates, files or items, and `tools/check_schemas.py` gives the promise totals in its step 8 (DEC-177).
   ```

## Start check

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
d() { cat CONTRIBUTING.md ROADMAP.md docs/AUDIT.md docs/guides/cold-readers.md docs/guides/full-review.md docs/clients.md | tr -s ' \n' '  ' | grep -oE -- "$1" | wc -l | tr -d ' '; }
s8() { PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | grep '^8\. ' | grep -oE '^8\. [A-Z][a-z]+|[0-9]+ promises in [0-9]+ files|[0-9]+ errors\.$' | paste -sd' ' -; }
h() { sed -n '/^## 8\. /,/^## 9\. /p' HOW-WE-WORK.md | grep -cE '^[0-9]+\. '; }
dr() { PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py --draft WORKTREE; }
c() { for f; do grep -cE "$P" "$f"; done | paste -sd' ' -; }
P='(^|[^A-Za-z0-9.-])[1-9][0-9]* ([A-Z][a-z]+ )?(promises?|contracts?|templates|documents|files|items|reference schemas)([^a-z]|$)'
git status --short | wc -l | tr -d ' '  # 0
sed -n '/^## DEC-177, /,/^## DEC-178, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
test -e spec/changes; echo "fragments $?"  # fragments 1
echo "$(grep -c '^## ' spec/CHANGES.md) $(sed -n '/^## \[Unreleased\]$/,$p' spec/CHANGES.md | grep -c '^- ')"  # 1 68
c spec/README.md spec/core/README.md spec/schemas/README.md spec/definitions.md engine/CONFORMANCE.md  # 49 9 2 0 0
k spec/README.md 'Counts below are current;' 'promises, 522 in all:'  # 1 1
k spec/core/README.md 'the 52 Schema contracts under' 'exports 158 documents, 80 of them Schema contracts'  # 1 1
k spec/schemas/README.md '| 38 files, each holding one set' 'Four files have `-manifest-`' 'Three others hold'  # 1 1 1
d '`spec/CHANGES.md` (entries|line)'  # 7
h  # 3
s8  # 8. Compared 522 promises in 27 files 0 errors.
```

Where a line prints otherwise, stop, change nothing and report it.

## Scope

Change, as the decisions say: `spec/changes/`; `spec/CHANGES.md`'s preamble;
`spec/README.md`'s Contents, `spec/core/README.md`'s "What this is" and
`spec/schemas/README.md`'s Examples; `tools/check_decisions.py`, `tools/check_schemas.py` and
their two test files; one bullet of `AGENTS.md`, two lines of `HOW-WE-WORK.md`, and phrases of
`CONTRIBUTING.md`, `ROADMAP.md`, `docs/AUDIT.md`, `docs/clients.md`, `docs/decisions/README.md`
and two guides. Nothing else: no promise, `engine/` (STANDING.md stays current),
`conformance/`, `packages/`, `experiments/`, other `tools/` files, `docs/work-orders/` or
`.github/`. Report anything else that needs changing.

## Done when

Each line prints otherwise before and passes after, with the functions and `P` as above:

```sh
test -f spec/changes/README.md; echo "readme $?"  # readme 0; readme 1 before
k spec/CHANGES.md 'and none here (DEC-177)' 'as `scriv collect` does'  # 1 1; 0 0 before
c spec/README.md spec/core/README.md spec/schemas/README.md  # 400b: 0 0 0; 49 9 2 before
grep -cE '[1-9][0-9]* (in all|of the layer)' spec/README.md  # 400b: 0; 2 before
grep -cE '^\| \[.*\| Draft \|$' spec/README.md  # 400b: 49; 2 before
k spec/README.md '| Draft; its one implementation is also an export |' 'The table states no count:' 'gives how many, in all and by layer.'  # 400b: 1 1 1; 0 0 0 before
k spec/core/README.md 'The package also exports the Schema contracts under' 'gives how many documents it exports.'  # 400b: 1 1; 0 0 before
k spec/schemas/README.md 'A file with `-manifest-` in its name holds a manifest' 'Others may hold, beside their documents,'  # 400b: 1 1; 0 0 before
s8  # 400b: 8. Counted 522 promises in 27 files 0 errors.; Compared before
dr | grep -c '^- '  # 69, 400a 68; 0 before
dr | sed -n '1p;$p' | cut -d' ' -f1-3 | paste -sd'|' -  # ## [Unreleased]|- Changed: `spec/README.md#contents`,; 400a ## [Unreleased]|- Changed: IFC-053,
k AGENTS.md 'writes its changelog lines in its own fragment'  # 1; 0 before
h  # 5, 400a 4; 3 before
k docs/decisions/README.md 'and its fragments under `spec/changes/` have one line per change to it;'  # 1; 0 before
echo "$(d '`spec/CHANGES.md` (entries|line)') $(d 'under `spec/changes/`')"  # 0 8; 7 0 before
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 -m unittest discover -s tools/tests -k fragment -k stored_count 2>&1 | grep -E '^(Ran |OK)' | sed -E 's/ in [0-9.]+s$//' | paste -sd' ' -  # Ran 10 tests OK, 400a Ran 7 tests OK; Ran 0 tests OK before
```

Guards, which pass before and must still pass:

```sh
export PYTHONDONTWRITEBYTECODE=1
git add -N spec/changes
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Not-done ratchet|Errors)'  # each head equals its base; Errors: 0
for t in check_docs check_schemas check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, three times
python3 tools/promise_standing.py --check | tail -2 | paste -sd' ' -  # engine/STANDING.md is current; promise rows: 522. Errors: 0
/usr/bin/python3 -m unittest discover -s tools/tests 2>&1 | tail -1  # OK
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # 0 changes; 0 undeclared; 0 errors; then exit 0
(cd packages/provider-local-process && cargo test --offline --locked --test package exported_type_template_passes_semantic_registration_rules 2>&1 | grep '^test result:')  # test result: ok. 1 passed; it imports check_schemas
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(CHANGES|README|core/README|schemas/README|changes/[^/]+)\.md|tools/(check_(decisions|schemas)|tests/test_(decisions|promise_totals))\.py|(AGENTS|HOW-WE-WORK|CONTRIBUTING|ROADMAP)\.md|docs/(AUDIT|clients|decisions/README|guides/(cold-readers|full-review))\.md)$' | wc -l | tr -d ' '  # 0
git reset -q -- spec/changes  # stage nothing
```

Report each changed sentence, each test's name and every check's last line.

## What follows

The admin marks DEC-177 done with this order's pull request; later spec orders write fragments
and recount nothing. Order 401, the serial queue, is rewritten without the first draft's
drivers: it regenerates STANDING and stops on any other conflict, among them those of
`conformance/expected-failures.yaml`, which the resolver merges by case id today, unless the
owner strikes that choice and asks for fragments of them too.

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main` at its start.

**Worker-2's turn**, on the primary ChatGPT account, carried out the whole order: the fragments
directory and its rule, the checker's fragment check, the stored counts removed from the entry
documents and the checkers printing the totals, keeping the 68 existing changelog items as they
were.

**Two blind reads.** The Codex verifier, `root-verifier-2.turn.v1`, asked for one must: a
fragment whose only item was a comment passed validation and then vanished from assembly. The
Claude reader, `root-reader-a.turn.v1`, on the Team seat, passed it with a should (the documents a
writer reads did not state a fragment's name rule or form) and two nits. One fix turn by worker-2
answered all four, the must with a regression that failed first. The expected-failures file stays
one file here; a later order makes it fragments (DEC-190). The gate's run on the squash is the
coordinator's, at the merge.
