# Work order 300: the gate's identity row admits the human contributors that `CONTRIBUTORS` lists, beside the work-order identity (the owner's answer of 2026-11-22)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. This order needs no network and fetches nothing. Run no
`cargo build` or `cargo test` yourself, no engine suite and not `tools/gate.py`: your tests are the
tool tests, whose own probe crate is theirs. Never print a whole large file, diff, log or list:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Run no `conduit`, `codex` or `claude`, and sign in nowhere. Prefix each
`python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local
path, user, machine or account name, person's name, e-mail address other than the work-order
identity's, or private repository name. Every identity in a test is fake: under `example.invalid`,
or, in test 4, at a `.synthetic` domain, as `test_private_paths.py`'s are.

## What and why

**The owner's answer** (2026-11-22; the coordinator logs it before this order runs, and the start
check prints its number, written D below): on a second human contributor's commits -> "Gate admits
listed humans (Recommended)". A commit passes if it is by the neutral identity or by a contributor
listed in a file the owner approves; squash merges keep the contributor as author; Agents and the
coordinator stay on the neutral identity; a contributor's commits carry their own name if they
agree. This order lists no one: the list starts empty, and the owner adds each line.

**Today**, read on `main` at `d7e63ebd` (#573). `tools/gate.py`'s identity row reads
`git --no-replace-objects log -z` with `IDENTITY_FORMAT` from the merge base to `HEAD`. It fails
each commit whose author or committer is not `IDENTITY`, `Conduit work order
<work-orders@conduit.invalid>`, naming the commit and its role (`author`, `committer`, or both)
and printing no identity it found. `test_identity_names_each_commit_not_by_the_work_order_identity`
holds this, with its `-z`, `.mailmap` and `refs/replace` guards. The method as run's squash
([section 7](../implementation/METHOD_AS_RUN.md#7-merge-and-the-record)) commits as the
work-order identity, author included. On `main`, `0e937eee` (#9) is a collaborator's pull request
squash-merged on GitHub: its author is the collaborator, its committer GitHub.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7,
everything an agent does is attributable: Agents' and the coordinator's commits stay on the
work-order identity, and a human's carry their own listed name, never an unlisted one; 8, no rule
assumes exactly one owner: a second human contributes under their own name. None at risk: the row
still prints no identity and reads no list a branch can change, and decision 7 lets only an
entry's e-mail stand, in a file whose every line the owner approves.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: Chromium's presubmit `CheckAuthorizedAuthor` refuses a change whose author
its `AUTHORS` file does not list, and Go kept a `CONTRIBUTORS` file; each holds one line per
person in git's own identity form, `Name <e-mail>`, as `git log --format='%an <%ae>'` prints it.
Not taken: `.mailmap`, which rewrites what git shows and which the row ignores by design; GitHub's
commit-metadata rules, since merges wait on the local gate, not on GitHub (ruling 80); signed
commits with git's `gpg.ssh.allowedSignersFile`, which need a key from each person and from the
work-order identity; a YAML file, which adds nothing to one line per person; reading the list
from the branch, which would let a branch admit its own commits; and a list outside the tree, as
`--private-names-file` is, which no review or other machine sees, while main's commits show each
author anyway.

**When it runs.** On `main` from `d7e63ebd` (#573), once the decision log holds D. Through
Conduit: one Codex worker turn, the blind Codex verifier and Claude reader at once, one fix turn,
the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what must hold; how is
yours. The coordinator's, open to the owner's strike (DEC-029): the file's name and form, author
and committer each checked on its own, the list read at the merge base, the words, the e-mail
check's exemption, and that a failure names the commit and the role whose identity is not
admitted, never a name or e-mail, since the gate's lines go into the pull request's description.
Shared files: `tools/gate.py`, `tools/check_private_paths.py` and their tests with any tools
order; AGENTS.md, CONTRIBUTING.md, HOW-WE-WORK.md and the method as run with docs orders.
Whichever merges later rebases.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), a file outside Scope,
or a start-check line printing otherwise. Where this order's wording contradicts a promise it does
not change, the promise governs: follow it and list it under `own_choices`, beside every choice no
decision governs.

## The decisions

1. **The file.** A new `CONTRIBUTORS` at the root, UTF-8 text, holding exactly these lines and no
   entry:

   ```text
   # Human contributors whose commits pass the gate's identity row (tools/gate.py), beside
   # Conduit work order <work-orders@conduit.invalid>, which Agents and the coordinator use.
   # The owner approves each line: one per person, the name and e-mail exactly as git records
   # them, as git log --format='%an <%ae>' prints them:
   #   Name <e-mail>
   # The gate reads this file as the branch's merge base holds it, so a line admits commits
   # only once it is merged. CONTRIBUTING.md says how a contributor's commits pass.
   ```

   An empty line, or one starting with `#`, is no entry. Any other line is one entry: a name, one
   space, `<`, an e-mail, `>`, and nothing after. The name is not empty, holds no `<` or `>`, and
   neither starts nor ends with white space; the e-mail is not empty and holds no `<`, `>` or
   white space; neither holds a control character (U+0000 to U+001F, U+007F). A line that is not
   an entry is a fault, as are an entry equal to the work-order identity and one repeating an
   earlier line.
2. **What passes.** Each of a commit's author (`%an`, `%ae`) and committer (`%cn`, `%ce`) passes
   on its own when it equals the work-order identity or an entry, byte for byte as git prints it.
   So a listed human's own commits pass, as do the coordinator's rebase and squash of them, which
   keep the human as author and commit as the work-order identity. All else the row does today
   stays: `GIT`, `IDENTITY_FORMAT` and its `-z` reading, the roles named and their order, the
   failure on malformed output, and no `.mailmap`.
3. **Which list.** The entries come from `CONTRIBUTORS` at the merge base, read through `GIT`, so
   `refs/replace` changes nothing, and never from the branch or the working tree. Absent there, not
   UTF-8 or not a regular file (a symlink's target text lists nobody), it lists none; a faulty line
   there admits nobody. The row also reads `CONTRIBUTORS` at `HEAD`, where present, and fails on
   each fault there, or where it is not UTF-8 or not a regular file: a branch that keeps or makes a
   faulty list fails, and the branch that fixes it passes.
4. **The words.** The reason, in a run and in `--plan`: `the branch adds N commits, each to be by
   Conduit work order <work-orders@conduit.invalid> or a contributor that CONTRIBUTORS lists at the
   merge base (K listed, J faulty)`, J counting the merge base's faults, a whole file not UTF-8 or
   not regular being one. The why for failing commits: `F of N commits are by neither Conduit work
   order <work-orders@conduit.invalid> nor a contributor that CONTRIBUTORS lists at the merge base
   (K listed, J faulty): H (ROLE), ...; the identities found are not printed; a listed contributor
   commits with the name and e-mail exactly as listed, one not listed at the merge base has the
   owner's line merged or fixed and rebases onto it, and any other commit is squashed again with
   the work-order identity, as the method as run's squash does`. A fault at `HEAD` comes first in
   the why: `CONTRIBUTORS at HEAD: line L FAULT, ...; no line is printed; fix each as the file's
   header says`, FAULT being `is not Name <e-mail>`, `is the work-order identity` or `repeats line
   M`, or for the whole file `is not UTF-8` or `is not a regular file`.
5. **Nothing private.** No name or e-mail, listed or found, and no line of the file reaches the
   output, the error stream, the record or a log, as today.
6. **The documents.** Wrap at each file's width.
   - CONTRIBUTING.md: the gate bullet's sentence "It fails a commit ..." fails a commit "whose
     author or committer is neither the work-order identity nor a contributor that `CONTRIBUTORS`
     lists at the merge base", linking the new section. A new `## Commits by a human contributor`,
     after `## Checks`, in at most 16 lines, says: Agents and the coordinator commit as the
     work-order identity; a human contributor who agrees commits under their own name once the
     owner has approved their line in `CONTRIBUTORS` (linked), merged before their branch; one who
     does not is not listed, and their branch is squashed as the work-order identity; the line is
     their `user.name` and `user.email` as git records them, and `check_private_paths.py` lets its
     e-mail stand there alone; the gate passes a commit whose author and committer are each one of
     these, and names any other with its role, printing no identity; the coordinator's rebase and
     squash keep them as author, the squash being the method as run's with
     `--author="Name <e-mail>"` as listed, committing as the work-order identity; and GitHub's
     squash merge takes the pull request's author as the commit's, so the contributor opens their
     branch's pull request, as for `0e937eee` (#9), the coordinator pushes the squash to it, and
     checks main's author after the merge.
   - HOW-WE-WORK.md: at the end of section 3, two lines, with D's number, the first linking
     `CONTRIBUTORS`, each keeping `(D)` and its label on its last line:
     - "A commit passes the gate only when its author and its committer are each `Conduit work
       order <work-orders@conduit.invalid>` or a human contributor that `CONTRIBUTORS` lists at
       the merge base. (D) `checked: tools/gate.py`"
     - "Agents and the coordinator commit as the work-order identity; a listed human commits under
       their own name and opens their branch's pull request, and its rebase and squash keep them
       as author; only the owner approves a line of `CONTRIBUTORS`. (D) `by review`"
   - AGENTS.md, the gate's paragraph: "committer is not `Conduit work order
     <work-orders@conduit.invalid>`." becomes "committer is neither `Conduit work order
     <work-orders@conduit.invalid>` nor a contributor that `CONTRIBUTORS` lists at the merge
     base.", rewrapped with no line added.
   - `docs/implementation/METHOD_AS_RUN.md`, section 7: "whose author or committer is not the
     work-order identity" becomes "whose author or committer is neither the work-order identity
     nor a contributor that `CONTRIBUTORS` lists at the merge base"; after "not the handover's
     plain `git commit`." add "A listed contributor's branch is squashed with `--author`, pushed
     to the pull request they opened, and main's author checked after the merge, as
     CONTRIBUTING.md says.", linking the new section.
7. **The e-mail check.** `tools/check_private_paths.py`, on the standard library alone, lets the
   e-mail between `<` and `>` of an entry line (decision 1's form) of the root's regular file
   `CONTRIBUTORS` stand, and nothing else: every other line, address and file, and the private
   names everywhere, are checked as today. Without it, the first entry fails the gate's `private
   paths` row and CI's `docs` job.

No `spec/` change, so no CHANGES line. The decision log is the coordinator's.

## The tests

Written first and run on the unchanged tools, each failing line reported. Tests 1 to 3 are in
`tools/tests/test_gate.py`'s `GateTests`. L is a fake listed identity and M another. How `base`
comes to hold a `CONTRIBUTORS` is yours: a commit by the work-order identity that `base` then
points to is one way.

1. `test_identity_admits_listed_contributors_by_author_and_committer`: with L listed at the merge
   base, a commit by L as author and committer, one by L as author with the work-order identity as
   committer, and one by the work-order identity as author with L as committer pass, the reason
   ending `(1 listed, 0 faulty)`. Then a commit authored by L's name with M's e-mail and one by
   M's name with L's e-mail each fail as `(author)`, `2 of 5 commits`, with decision 4's words. No
   name or e-mail of L or M is in the output, the error stream, the record or the logs.
2. `test_identity_reads_the_list_at_the_merge_base`: a branch commit by the work-order identity
   that lists L, then a commit by L: it fails `(author and committer)`, the reason ending
   `(0 listed, 0 faulty)`; once `base` holds the first commit, the second passes, `(1 listed,
   0 faulty)`. A `refs/replace` of the base's `CONTRIBUTORS` blob by one listing M, and a
   working-tree `CONTRIBUTORS` listing M, admit no commit by M.
3. `test_identity_fails_a_faulty_list_naming_its_lines`: `HEAD`'s `CONTRIBUTORS` holding, after the
   header, a line not of the form, one with a tab, the work-order identity and a repeated line: the
   row fails naming each line and its fault, and the text of none. A file that is not UTF-8, and a
   symlink named `CONTRIBUTORS`, each fail as decision 4 says. With a faulty list at the merge
   base, a branch that fixes it passes, and a commit by an identity that a faulty base line names
   fails. A merge base whose `CONTRIBUTORS` is not UTF-8, or is a symlink whose target text is
   L's entry, lists none, `(0 listed, 1 faulty)`.
4. In `tools/tests/test_private_paths.py`, `test_contributors_entries_may_hold_an_address`: a
   root `CONTRIBUTORS` whose entry's e-mail is at a `.synthetic` domain is green; the same address
   on a `#` line, after the `>`, in the entry's name, in another file, in a `CONTRIBUTORS` under a
   directory or in a symlink `CONTRIBUTORS` is red, as is a private name in the entry.

`test_identity_names_each_commit_not_by_the_work_order_identity` changes only its reason
assertion, to decision 4's with `(0 listed, 0 faulty)`. No other test changes.

## What to read

`tools/gate.py`: `IDENTITY`, `GIT`, `IDENTITY_FORMAT`, `merge_base`, the identity row and
`finish`. `tools/tests/test_gate.py`: `GateTests`' helpers,
`test_identity_names_each_commit_not_by_the_work_order_identity` and
`test_record_says_what_ran_and_holds_no_private_detail`. HOW-WE-WORK.md's form and section 3;
CONTRIBUTING.md's `## Checks`; AGENTS.md's gate paragraph; the method as run's section 7;
`tools/check_private_paths.py`'s e-mail rule and `test_private_paths.py`'s helpers.

## Before you change anything

From the repository root; stop, change nothing and report if a line prints otherwise:

```sh
export PYTHONDONTWRITEBYTECODE=1
git merge-base --is-ancestor d7e63ebd HEAD; echo "main $?"
git rev-parse HEAD  # BASE
git status --short | wc -l | tr -d ' '
D=$(awk '/^## DEC-[0-9][0-9][0-9], /{h=$2} /Gate admits listed humans \(Recommended\)/{print h; exit}' docs/decisions/log.md | tr -d ,); echo "decision ${D:-none}"
test -e CONTRIBUTORS; echo "file $?"
grep -c "^IDENTITY = ('Conduit work order', 'work-orders@conduit.invalid')$" tools/gate.py
git grep -c CONTRIBUTORS -- tools AGENTS.md CONTRIBUTING.md HOW-WE-WORK.md docs/implementation | wc -l | tr -d ' '
grep -c 'def test_identity_' tools/tests/test_gate.py
grep -cF 'committer is not `Conduit work order <work-orders@conduit.invalid>`.' AGENTS.md
grep -cF 'whose author or committer is not the' docs/implementation/METHOD_AS_RUN.md
grep -cF 'as last fetched that is not by' CONTRIBUTING.md
wc -l < AGENTS.md | tr -d ' '
/usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -k test_identity_ 2>&1 | tail -1
```

They print `main 0`, BASE, `0`, `decision DEC-` with D's three digits, `file 1`, `1`, `0`, `1`,
`1`, `1`, `1`, `225` and `OK`. `decision none` means D is not logged yet. `/usr/bin/python3` is
the Mac's interpreter with PyYAML.

## Scope

Change `CONTRIBUTORS` (new); `tools/gate.py` (the identity row and the helpers it needs);
`tools/tests/test_gate.py` (three tests and one assertion); `tools/check_private_paths.py`
(decision 7) and `tools/tests/test_private_paths.py` (test 4); and CONTRIBUTING.md,
HOW-WE-WORK.md, AGENTS.md and `docs/implementation/METHOD_AS_RUN.md` as decision 6 says. Write
outside these only under `target/wo300/`. Change nothing else: not `spec/`, `engine/`, `.github/`,
`docs/decisions/`, the work-order index or another tool.

## Done when

The first group prints otherwise on BASE and passes after; the second, guards, pass on both.

```sh
export PYTHONDONTWRITEBYTECODE=1
D=$(awk '/^## DEC-[0-9][0-9][0-9], /{h=$2} /Gate admits listed humans \(Recommended\)/{print h; exit}' docs/decisions/log.md | tr -d ,)
/usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -k test_identity_ 2>&1 | grep -E '^Ran '  # Ran 4 tests; today Ran 1 test
test -f CONTRIBUTORS; echo "file $?"  # file 0; today file 1
grep -c '^#' CONTRIBUTORS  # 7; today no file
grep -cvE '^(#.*)?$' CONTRIBUTORS  # 0, no entry; today no file
grep -c CONTRIBUTORS tools/gate.py tools/check_private_paths.py  # 1 or more each; today 0 and 0
/usr/bin/python3 -m unittest discover -s tools/tests -p test_private_paths.py -k contributors 2>&1 | grep -E '^Ran '  # Ran 1 test; today Ran 0 tests
grep -c '^## Commits by a human contributor$' CONTRIBUTING.md  # 1; today 0
grep -cF "($D) \`checked: tools/gate.py\`" HOW-WE-WORK.md  # 1; today 0
grep -cF "($D) \`by review\`" HOW-WE-WORK.md  # 1; today 0
grep -c CONTRIBUTORS AGENTS.md docs/implementation/METHOD_AS_RUN.md  # 1, then 1 or more; today 0 and 0
```

```sh
export PYTHONDONTWRITEBYTECODE=1
/usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -k test_identity_ 2>&1 | tail -1  # OK
/usr/bin/python3 -m unittest discover -s tools/tests 2>&1 | tail -1  # begins OK
wc -l < AGENTS.md | tr -d ' '  # 225
n=$(git ls-files -o --exclude-standard -- CONTRIBUTORS); [ -z "$n" ] || git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(CONTRIBUTORS|tools/gate\.py|tools/check_private_paths\.py|tools/tests/test_(gate|private_paths)\.py|AGENTS\.md|CONTRIBUTING\.md|HOW-WE-WORK\.md|docs/implementation/METHOD_AS_RUN\.md)$' | wc -l | tr -d ' '  # 0
[ -z "$n" ] || git reset -q -- $n  # stage nothing
```

Your report gives, per decision, what changed and where; each new test's failing line on the
unchanged tools, then its passing line; the assertion you changed; the why lines the tests saw for
decision 4; your own choices; and every check's last line. Stage nothing.

## What follows

- **The first entry**, which the owner approves and merges before that contributor's first branch.
- **A listed line that a private name matches**, derived or from `--private-names-file`, still
  fails `check_private_paths.py`, since decision 7 lets only the entry's e-mail stand. If one does,
  an order decides.
- **Signed commits** against git's allowed-signers file, if the owner wants a stronger check.

## Record

Run on 2026-11-23 through Conduit itself, in stage 3 of experiment 028, by its pair 2, in Pacific
time, stacked on the decision log's entries DEC-082 to DEC-102.

**Worker-2's turns.** The first (09:41 to 09:42) stopped rightly at the start check: the owner's
decision was not yet logged. The coordinator logged it as DEC-089, and the second turn (09:44 to
09:52) built it: the gate's identity row admits, beside the neutral identity, commits by the
contributors a root `CONTRIBUTORS` file lists, which starts empty; `check_private_paths` lets
exactly its entry lines' e-mails stand; HOW-WE-WORK.md, CONTRIBUTING.md and METHOD_AS_RUN.md say
how a contributor is listed and how a squash keeps the author. 211 tool tests passed.

**Two blind reads** (09:53 to 09:56): the Codex verifier, `root-verifier-2.turn.v1`, had one must:
with no new commits the identity row skipped the list's check, so a faulty committed list passed.
The Claude reader (`reader-1`, while stage 3's reader is held) had no must, three shoulds and four
nits. One fix turn (09:57 to 10:01) applied all eight, each failing first: 62 gate tests and 20
private-path tests pass. The gate's run on the squash is the coordinator's, at the merge.
