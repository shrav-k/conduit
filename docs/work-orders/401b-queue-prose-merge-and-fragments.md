# Work order 401b: `wo queue` settles spec prose and moves changelog items to fragments (DEC-177)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
is empty, since this order adds no case. It needs no network; if a crate is missing from the
cache, fetch nothing and report it. Run one cargo command at a time, and not `tools/gate.py`.
Never print a whole large file, case, diff or list: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential. Run no `conduit` outside a
test, no `codex` or `claude`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user,
machine or account name, or private repository name.

Run no `gh`. Your tests run git only in repositories they make in a temporary directory, with
git's global and system configuration off, and the fixture binary of order 401 plays `gh`, the
gate and the repository's Python tools.

## What and why

**The owner's words.**
[DEC-177](../decisions/log.md#dec-177-2026-11-24-changelog-fragments-and-no-stored-counts):
"Changelog fragments, no stored counts".
[DEC-174](../decisions/log.md#dec-174-2026-11-23-the-coordinators-rust-tooling-lives-in-experiment-028):
where the coordinator's Rust tooling lives -> "Inside experiment 028 (Recommended)".

**Today.** [Order 401](401-coordinator-merge-queue.md)'s `wo queue` regenerates
`engine/STANDING.md` at a stopped rebase and stops on every other conflict. The coordinator's
private kit settles two more kinds: spec prose goes through a word merge that stops on overlap,
and `spec/CHANGES.md` keeps both sides. After the rebase, the kit moves new `Unreleased` items to
their fragment. Where both sides added a line at one place, the kit's word merge joined two list
items, or two table rows, into one line, and no guard caught it.

Order 400's What follows asks 401 only to regenerate STANDING and stop on any other conflict.
This order adds the word merge, the changelog's keep-both merge and the fragment step, since each
keeps both sides or stops.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1). None is
at stake: no promise, case, engine source or package changes. No hunk keeps one side over a
changed other, and no merged line joins two list items or two table rows.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice. The word merge is diff3's rule, as `git merge-file` applies it to lines,
over words instead. The fragment step writes the file that order 400 names after scriv's and
towncrier's fragments. Not taken: `git rerere`, which replays a person's resolution; merge
drivers (DEC-177).

**When it runs.** On `main` once orders 400 and 401 merge: it changes 401's crate, and the
fragment step needs `spec/changes/`. Through Conduit: one Codex worker turn, the blind Codex
verifier and the Claude reader at once, one fix turn, the gate (DEC-050, DEC-033). By ruling 89,
each decision says what must hold; how is yours. The coordinator's choices, open to the owner's
strike (DEC-029): the paths decision 1 settles, the word rules, keeping both sides of a hunk with
an empty base, and the fragment's name.

**Shared files.** None.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first order 401's text, the crate's `README.md`, `spec/changes/README.md` and the preamble
of `spec/CHANGES.md`.

1. **A stopped rebase settles two more kinds** (401's decision 6). Before 401's other conflicted
   paths stop, these are settled hunk by hunk. A path without all three stages still stops.
   - `spec/CHANGES.md`: a hunk whose ours and theirs each begin with the base's lines becomes the
     base's lines, then ours' added lines, then theirs'.
   - Prose: each `.md` and `.yaml` file under `spec/` but `spec/CHANGES.md` and the fragments;
     `engine/CONFORMANCE.md`; and `docs/work-orders/*.md`. A hunk whose base is empty, both sides
     having added lines at one place, keeps ours' lines and then theirs', each whole, in a `.md`
     file, and stops in YAML. Any other hunk merges by words: each side is split at whitespace,
     and both sides' edits against the base apply. Two edits must have an unchanged base word
     between them, but both may insert at the hunk's end, ours first, except in a table row. The
     result wraps at the widest line of ours' and theirs' sides; its first line keeps ours' indent
     and list marker. A hunk merges only where each side is one run of text lines: not empty,
     with no blank line, code fence or heading, no line after the first that begins a list item
     or a table row, in YAML no line opening a key or a list item, and each line after the first
     indented to the column where the first line's text begins. A side that is one Markdown
     table row stays one line.
   - Anything else stops, naming the path and the hunk's first line, and for an overlap up to
     five base words on each side.
2. **The fragment step**, `fragment`, runs after 401's `standing` step, only where PINNED holds
   `spec/changes/README.md`, and commits "wip: changelog item moved to its fragment (DEC-177)".
   The items that the branch's `spec/CHANGES.md` holds under `## [Unreleased]` and PINNED's
   lacks move, word for word and in order, to the end of `spec/changes/FRAGMENT`; with none, the
   step does nothing. An item is a line that begins `- ` and its continuation lines, indented two
   spaces. FRAGMENT is the file name of the entry's order text (401's decision 3), or for an
   entry with none, the branch's name with `-` for `/`, then `.md`, as `spec/changes/README.md`
   names fragments. A name that is not a fragment's fails the step. `spec/CHANGES.md` loses only
   the moved lines.
3. **Tests**, in 401's `tests/queue.rs` with its fixture, and `word_merge_cases` in the library.
   - `queue_resolves_prose`: `main` and the branch change different words of one YAML block text
     under `spec/`, and each appends a different item to one list in a `.md` file under `spec/`.
     It merges with both edits, both items whole, and the fixture's STANDING for the merged tree.
   - `queue_prose_overlap_stops`: the same word, then adjacent words, fail at `rebase`, naming
     the path and line; no rebase is in progress, HEAD equals the backup, the tree is clean.
   - `queue_changelog_moves_to_fragment`: `main` holds `spec/changes/README.md` and an
     `Unreleased` item newer than the branch, which adds another and its order text. The merge
     keeps PINNED's `spec/CHANGES.md`, and the fragment named after the order text holds the
     branch's item word for word. A `wo/ID-...` branch whose order text `main` already holds
     names the fragment after that text, a branch of another name after itself, and a branch
     with no new item writes none.
   - `word_merge_cases`: disjoint edits merge; adjacent, overlapping and same-place edits stop,
     but for both inserting at the end, which stops in a table row; a table row stays one line;
     a list item keeps its continuation indent; a blank line, a code fence or a YAML key stops.
     An empty base keeps two added list items, and two added table rows, whole, and stops in
     YAML; a side that adds a list item or a table row after its first line stops.
4. **README.** The crate's `` ## `wo queue` `` section gains decision 1's rules and the fragment
   step.

## Start check

From the repository root:

```sh
c=experiments/028-a78-dogfood/coordinator
git status --short | wc -l | tr -d ' '  # 0
test -f spec/changes/README.md; echo "fragments $?"  # fragments 0
grep -c 'and none here (DEC-177)' spec/CHANGES.md  # 1
cat $c/README.md 2>/dev/null | grep -c '^## `wo queue`$'  # 1
grep -c '^Not yet run\.$' docs/work-orders/401-coordinator-merge-queue.md  # 0
grep -rhoE 'fn (queue_resolves_standing|queue_other_conflict_stops)\(' $c/tests | wc -l | tr -d ' '  # 2
grep -rhoE 'fn (queue_resolves_prose|queue_prose_overlap_stops|queue_changelog_moves_to_fragment|word_merge_cases)\(' $c/src $c/tests | wc -l | tr -d ' '  # 0
```

Where a line prints otherwise, stop, change nothing and report it. The second and third lines
show that order 400 is in the base, the fourth to sixth that order 401 is.

## Scope

Change only `src/`, `tests/` and `README.md` in `experiments/028-a78-dogfood/coordinator/`.
Change nothing else: not the crate's manifest or lockfile, `tools/`, `spec/`, `conformance/`,
`engine/`, `packages/`, `docs/`, `.github/` or another experiment. Write outside the tree only in
temporary directories your tests make. Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after. The second is guards, which pass today
and after.

```sh
c=experiments/028-a78-dogfood/coordinator
ok='^test result: ok\. 1 passed'
for t in queue_resolves_prose queue_prose_overlap_stops queue_changelog_moves_to_fragment; do (cd $c && cargo test --offline --locked --test queue "$t" -- --exact 2>&1 | grep -c "$ok"); done  # 1 three times; 0 today
(cd $c && cargo test --offline --locked --lib word_merge_cases 2>&1 | grep -c "$ok")  # 1; 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
c=experiments/028-a78-dogfood/coordinator
(cd $c && cargo fmt --check); echo "fmt $?"  # fmt 0
(cd $c && cargo clippy --offline --locked --all-targets -- -D warnings 2>&1 | grep -c '^error')  # 0
(cd $c && cargo test --offline --locked 2>&1 | grep -E '^test result:' | grep -vc '^test result: ok\.')  # 0
git add -N $c
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, twice
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^experiments/028-a78-dogfood/coordinator/(src|tests)/.+$|^experiments/028-a78-dogfood/coordinator/README\.md$' | wc -l | tr -d ' '  # 0
git reset -q -- $c  # stage nothing
```

Your report gives, per decision, what changed and where; each test's passing line; each of your
own choices; and every check's last line. Stage nothing.

## What follows

- From this order on, `wo queue` settles prose and changelog conflicts; a conflict it cannot
  settle still fails its entry, for the coordinator to settle and `wo queue retry`.
- A conflict in `conformance/expected-failures.yaml` stops until each order adds its own
  expected-failures fragment (DEC-190).

## Record

Run on 2026-11-24, in Pacific time, on order 401's tip (the merge queue, built and queued).

**Built outside the Fleet**, as order 401 was: it changes experiment 028's directory, which no order
sent through the Fleet may change (DEC-057). One Claude worker in a worktree of its own, two blind
Claude reads, one fix pass; no Agent of the dogfood took a turn on it.

**The worker** added the prose merge, the changelog's keep-both merge and the fragment step, with
their tests; every Start check and Done-when line printed what the order expects.

**Two blind reads.** One read it for merge safety and asked for one must and three shoulds: the
word merge rewrapped lines without checking that a later line did not begin a Markdown block (a
list marker or a heading); the changelog's keep-both checked only each side's start; a Markdown
keep-both did not check where theirs' first line belongs; and the literal-block guard read only
the nearest shallower line. The other read it against the order and passed it with nits. One fix
pass answered every must and should and four nits, each with a test that failed first. The gate's
run on the squash is the coordinator's, at the merge.
